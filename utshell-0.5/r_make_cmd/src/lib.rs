//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

use libc::{c_int, c_char, c_void};
use r_bash::*;
use rcommon::{WordList, WordDesc, STREQN};
use r_shell::{savestring,};
use r_eval::{array_p,array_cell, array_num_elements};


extern "C" {
    fn list_reverse(list:*mut GENERIC_LIST) -> *mut GENERIC_LIST;

}


#[no_mangle]
pub static mut here_doc_first_line: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut wdcache: sh_obj_cache_t = {
    let mut init = objcache {
        data: 0 as *const libc::c_void as *mut libc::c_void,
        cs: 0 as libc::c_int,
        nc: 0 as libc::c_int,
    };
    init
};
#[no_mangle]
pub static mut wlcache: sh_obj_cache_t = {
    let mut init = objcache {
        data: 0 as *const libc::c_void as *mut libc::c_void,
        cs: 0 as libc::c_int,
        nc: 0 as libc::c_int,
    };
    init
};

#[macro_export]
macro_rules! WDCACHESIZE {
    () => {
        128
    };
}

#[macro_export]
macro_rules! WLCACHESIZE {
    () => {
        128
    };
}


#[macro_export]
macro_rules! ocache_create {
    ($c:expr, $otype:ty, $n:expr) => {
        $c.data = xmalloc($n * (std::mem::size_of::<*mut $otype>() as libc::c_ulong) as usize);
        wdcache.cs = 128 as libc::c_int;
        wdcache.nc = 0 as libc::c_int;
    }
}

#[macro_export]
macro_rules! ocache_alloc {
    ($c:expr, $otype:ty, $r:expr) => {
        if $c.nc > 0 {
            $c.nc -= 1;
            $r = ($c.data as *mut *mut $otype).offset($c.nc as isize) as *mut $otype;
        } else {
            $r = xmalloc(std::mem::size_of::<$otype>() as usize) as *mut $otype;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cmd_init() {
    ocache_create! (wdcache, WordDesc, WDCACHESIZE!());
    ocache_create! (wlcache, WordList, WLCACHESIZE!());
}


#[no_mangle]
pub unsafe extern "C" fn alloc_word_desc() -> *mut WordDesc {
    let mut temp: *mut WordDesc = 0 as *mut WordDesc;

    ocache_alloc! (wdcache, WordDesc, temp);
    (*temp).flags = 0;
    (*temp).word = 0 as *mut libc::c_char;
    return temp;
}

#[no_mangle]
pub unsafe extern "C" fn make_bare_word(
    mut string: *const libc::c_char,
) -> *mut WordDesc {
    let mut temp: *mut WordDesc = 0 as *mut WordDesc;
    temp = alloc_word_desc();

    if *string != 0 {
        (*temp).word = savestring!(string);
    } else {
        (*temp).word = xmalloc(1) as *mut libc::c_char;
        *((*temp).word).offset(0 as isize) = '\u{0}' as i32 as libc::c_char;
    }
    return temp;
}

pub type size_t = libc::c_ulong;


#[macro_export]
macro_rules! DECLARE_MBSTATE {
    () => {
        let mut state: mbstate_t = mbstate_t {
            __count: 0,
            __value: __mbstate_t__bindgen_ty_1 { __wch: 0 },
        };
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            std::mem::size_of::<mbstate_t>() as usize,
        );
    };
}

#[inline]
unsafe extern "C" fn is_basic(mut c: libc::c_char) -> libc::c_int {
    return (*is_basic_table
        .as_ptr()
        .offset((c as libc::c_uchar as libc::c_int >> 5 as libc::c_int) as isize)
        >> (c as libc::c_uchar as libc::c_int & 31 as libc::c_int)
        & 1 as libc::c_int as libc::c_uint) as libc::c_int;
}

#[macro_export]
macro_rules! ADVANCE_CHAR {
    ($_str:expr, $_strsize:expr, $_i:expr, $state:expr) => {
        if locale_mb_cur_max > 1 {
            let mut state_bak: mbstate_t = mbstate_t {
                __count: 0,
                __value: __mbstate_t__bindgen_ty_1 { __wch: 0 },
            };
            let mut mblength: size_t = 0;
            let mut _f: libc::c_int = 0;

            _f = is_basic(*$_str.offset($_i as isize));
            if _f != 0 {
                mblength = 1 as libc::c_int as size_t;
            } else if locale_utf8locale != 0
                    && *$_str.offset($_i as isize) as libc::c_int & 0x80 as libc::c_int == 0  
            {
                mblength = (*$_str.offset($_i as isize) as libc::c_int
                    != 0 as libc::c_int) as libc::c_int as size_t;
            } else {
                state_bak = $state;
                mblength = mbrlen(
                    $_str.offset($_i as isize),
                    $_strsize.wrapping_sub($_i as libc::c_ulong) as usize,
                    &mut $state,
                ) as u64;
            }
            if mblength == -(2 as libc::c_int) as size_t
                || mblength == -(1 as libc::c_int) as size_t
            {
                $state = state_bak;
                $_i += 1;
            } else if mblength == 0 {
                $_i += 1;
            } else {
                $_i = ($_i as libc::c_ulong).wrapping_add(mblength) as libc::c_int
                    as libc::c_int;
            }
        } else {
            $_i += 1;
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn make_word_flags(
    mut w: *mut WordDesc,
    mut string: *const libc::c_char,
) -> *mut WordDesc {
    let mut i: libc::c_int = 0;
    let mut slen: size_t = 0;
    // DECLARE_MBSTATE!();
    let mut state: mbstate_t = mbstate_t {
        __count: 0,
        __value: __mbstate_t__bindgen_ty_1 { __wch: 0 },
    };
    memset(
        &mut state as *mut mbstate_t as *mut libc::c_void,
        '\u{0}' as i32,
        std::mem::size_of::<mbstate_t>() as usize,
    );
    
    i = 0;
    slen = strlen(string);
    while (i as libc::c_ulong) < slen {
        let opt = *string.offset(i as isize);
        let opt_char = char::from(opt as u8);
        
        match opt_char{
            '$' => {
                (*w).flags |= W_HASDOLLAR as libc::c_int;
            }
            '\'' | '`' | '"' => {
                (*w).flags |= W_QUOTED as libc::c_int;
            }
            '\\' | _ => {}
        }
        ADVANCE_CHAR!(string, slen, i, state);
    }
    return w;
}


#[no_mangle]
pub unsafe extern "C" fn make_word(mut string: *const libc::c_char) -> *mut WordDesc {
    let mut temp: *mut WordDesc = 0 as *mut WordDesc;

    temp = make_bare_word(string);
    return make_word_flags(temp, string);
}

#[no_mangle]
pub unsafe extern "C" fn make_word_from_token(mut token: libc::c_int) -> *mut WordDesc {
    let mut tokenizer: [libc::c_char; 2] = [0; 2];

    tokenizer[0 as usize] = token as libc::c_char;
    tokenizer[1 as usize] = '\u{0}' as i32 as libc::c_char;
    return make_word(tokenizer.as_mut_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn make_word_list(
    mut word: *mut WordDesc,
    mut wlink: *mut WordList,
) -> *mut WordList {
    let mut temp: *mut WordList = 0 as *mut WordList;

    ocache_alloc!(wlcache, WordList, temp);

    (*temp).word = word;
    (*temp).next = wlink;
    return temp;
}


#[no_mangle]
pub unsafe extern "C" fn make_command(
    mut type_0: command_type,
    mut pointer: *mut SIMPLE_COM,
) -> *mut COMMAND {
    let mut temp: *mut COMMAND = 0 as *mut COMMAND;
    
    temp = xmalloc(std::mem::size_of::<COMMAND>() as usize) as *mut COMMAND;
    (*temp).type_ = type_0;
    (*temp).value.Simple = pointer;
    (*temp).flags = 0 ;
    (*(*temp).value.Simple).flags = 0;
    (*temp).redirects = 0 as *mut REDIRECT;
    return temp;
}

#[no_mangle]
pub unsafe extern "C" fn command_connect(
    mut com1: *mut COMMAND,
    mut com2: *mut COMMAND,
    mut connector: libc::c_int,
) -> *mut COMMAND {
    let mut temp: *mut CONNECTION = 0 as *mut CONNECTION;

    temp = xmalloc(std::mem::size_of::<CONNECTION>() as usize) as *mut CONNECTION;
    (*temp).connector = connector;
    (*temp).first = com1;
    (*temp).second = com2;
    return make_command(
        command_type_cm_connection,
        temp as *mut SIMPLE_COM,
    );
}



unsafe extern "C" fn make_for_or_select(
    mut type_0: command_type,
    mut name: *mut WordDesc,
    mut map_list: *mut WordList,
    mut action: *mut COMMAND,
    mut lineno: libc::c_int,
) -> *mut COMMAND {
    let mut temp: *mut FOR_COM = 0 as *mut FOR_COM;
    
    temp = xmalloc(std::mem::size_of::<FOR_COM>() as usize) as *mut FOR_COM;
    (*temp).flags = 0 ;
    (*temp).name = name;
    (*temp).line = lineno;
    (*temp).map_list = map_list;
    (*temp).action = action;
    return make_command(type_0 as libc::c_uint, temp as *mut SIMPLE_COM);
}


#[no_mangle]
pub unsafe extern "C" fn make_for_command(
    mut name: *mut WordDesc,
    mut map_list: *mut WordList,
    mut action: *mut COMMAND,
    mut lineno: libc::c_int,
) -> *mut COMMAND {
    return make_for_or_select(
        command_type_cm_for ,
        name,
        map_list,
        action,
        lineno,
    );
}
#[no_mangle]
pub unsafe extern "C" fn make_select_command(
    mut name: *mut WordDesc,
    mut map_list: *mut WordList,
    mut action: *mut COMMAND,
    mut lineno: libc::c_int,
) -> *mut COMMAND {
    return make_for_or_select(
        command_type_cm_select ,
        name,
        map_list,
        action,
        lineno,
    );
}

#[macro_export]
macro_rules! W_NOGLOB {
    () => {
        1 << 5
    };
}

#[macro_export]
macro_rules! W_NOSPLIT {
    () => {
        1 << 4
    };
}

#[macro_export]
macro_rules! W_QUOTED {
    () => {
        1 << 1
    };
}

#[macro_export]
macro_rules! W_DQUOTE {
    () => {
        1 << 19
    };
}

#[macro_export]
macro_rules! W_NOPROCSUB {
    () => {
        1 << 20
    };
}

unsafe extern "C" fn make_arith_for_expr(mut s: *mut libc::c_char) -> *mut WordList {
    let mut result: *mut WordList = 0 as *mut WordList;
    let mut wd: *mut WordDesc = 0 as *mut WordDesc;

    if s.is_null() || *s as libc::c_int == '\u{0}' as i32 {
        return 0 as *mut WordList;
    }
    wd = make_word(s);
    (*wd).flags |= W_NOGLOB!() | W_NOSPLIT!() | W_QUOTED!() | W_DQUOTE!();
    (*wd).flags |= W_NOPROCSUB!();
    result = make_word_list(wd, 0 as *mut WordList);
    return result;
}

#[macro_export]
macro_rules! whitespace {
    ($c:expr) => {
        $c as libc::c_int == ' ' as i32 || $c as libc::c_int == '\t' as i32
    };
}

#[macro_export]
macro_rules! FREE {
    ($s:expr) => {
        if ($s) != std::ptr::null_mut() {
            free($s as *mut c_void);
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn make_arith_for_command(
    mut exprs: *mut WordList,
    mut action: *mut COMMAND,
    mut lineno: libc::c_int,
) -> *mut COMMAND {
    let mut temp: *mut ARITH_FOR_COM = 0 as *mut ARITH_FOR_COM;
    let mut init: *mut WordList = 0 as *mut WordList;
    let mut test: *mut WordList = 0 as *mut WordList;
    let mut step: *mut WordList = 0 as *mut WordList;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nsemi: libc::c_int = 0;
    let mut i: libc::c_int = 0;

    step = 0 as *mut WordList;
    test = step;
    init = test;

    s = (*(*exprs).word).word;
    t = s;
    start = t;
    nsemi = 0;
    loop {
        while whitespace!(*s) {
            s = s.offset(1);
        }
        start = s;
        i = skip_to_delim(
            start,
            0 ,
            b";\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            SD_NOJMP as libc::c_int | SD_NOPROCSUB as libc::c_int,
        );
        s = start.offset(i as isize);

        t = if i > 0 as libc::c_int {
            substring(start, 0, i)
        } else {
            0 as *mut libc::c_char
        };

        nsemi += 1;
        match nsemi {
            1 => {
                init = make_arith_for_expr(t);
            }
            2 => {
                test = make_arith_for_expr(t);
            }
            3 => {
                step = make_arith_for_expr(t);
            }
            _ => {}
        }
        FREE!(t);
        if *s as libc::c_int == '\u{0}' as i32 {
            break;
        }
        s = s.offset(1);
    }
    if nsemi != 3 {
        if nsemi < 3 {
            parser_error(
                lineno,
                b"syntax error: arithmetic expression required\0" as *const u8 as *mut c_char     
            );
        } else {
            parser_error(
                lineno,
                b"syntax error: `;' unexpected\0" as *const u8 as *const libc::c_char
            );
        }
        parser_error(
            lineno,
            b"syntax error: `((%s))'\0" as *const u8 as *const libc::c_char,
            (*(*exprs).word).word,
        );
        free(init as *mut c_void);
        free(test as *mut c_void);
        free(step as *mut c_void);
        set_exit_status(2 );
        return 0 as *mut COMMAND;
    }

    temp = xmalloc(std::mem::size_of::<ARITH_FOR_COM>() as usize) as *mut ARITH_FOR_COM;
    (*temp).flags = 0 ;
    (*temp).line = lineno;
    (*temp).init = if !init.is_null() {
        init
    } else {
        make_arith_for_expr(
            b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        )
    };
    (*temp).test = if !test.is_null() {
        test
    } else {
        make_arith_for_expr(
            b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        )
    };
    (*temp).step = if !step.is_null() {
        step
    } else {
        make_arith_for_expr(
            b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        )
    };
    (*temp).action = action;
    dispose_words(exprs);
    return make_command(
        command_type_cm_arith_for,
        temp as *mut SIMPLE_COM,
    );
}

#[no_mangle]
pub unsafe extern "C" fn make_group_command(mut command: *mut COMMAND) -> *mut COMMAND {
    let mut temp: *mut GROUP_COM = 0 as *mut GROUP_COM;
    temp = xmalloc(std::mem::size_of::<GROUP_COM>() as usize) as *mut GROUP_COM;
    
    (*temp).command = command;
    return make_command(
        command_type_cm_group,
        temp as *mut SIMPLE_COM,
    );
}

#[macro_export]
macro_rules! REVERSE_LIST {
    ($list:expr, $type:ty) => {
        if !($list).is_null()
            && !((*$list).next).is_null()
        {
            list_reverse($list as *mut GENERIC_LIST)
                as $type
        } else {
            $list as $type
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn make_case_command(
    mut word: *mut WordDesc,
    mut clauses: *mut PATTERN_LIST,
    mut lineno: libc::c_int,
) -> *mut COMMAND {
    let mut temp: *mut CASE_COM = 0 as *mut CASE_COM;
    
    temp = xmalloc(std::mem::size_of::<CASE_COM>() as usize) as *mut CASE_COM;
    (*temp).flags = 0 ;
    (*temp).line = lineno;
    (*temp).word = word;
    (*temp).clauses = REVERSE_LIST!(clauses, *mut PATTERN_LIST);
    return make_command(command_type_cm_case, temp as *mut SIMPLE_COM);
}

#[no_mangle]
pub unsafe extern "C" fn make_pattern_list(
    mut patterns: *mut WordList,
    mut action: *mut COMMAND,
) -> *mut PATTERN_LIST {
    let mut temp: *mut PATTERN_LIST = 0 as *mut PATTERN_LIST;

    temp = xmalloc(std::mem::size_of::<PATTERN_LIST>() as usize) as *mut PATTERN_LIST;
    (*temp).patterns = REVERSE_LIST!(patterns, *mut WordList);
    (*temp).action = action;
    (*temp).next = 0 as *mut pattern_list;
    (*temp).flags = 0;
    return temp;
}


#[no_mangle]
pub unsafe extern "C" fn make_if_command(
    mut test: *mut COMMAND,
    mut true_case: *mut COMMAND,
    mut false_case: *mut COMMAND,
) -> *mut COMMAND {
    let mut temp: *mut IF_COM = 0 as *mut IF_COM;

    temp = xmalloc(std::mem::size_of::<IF_COM>() as usize) as *mut IF_COM;
    (*temp).flags = 0;
    (*temp).test = test;
    (*temp).true_case = true_case;
    (*temp).false_case = false_case;
    return make_command(command_type_cm_if , temp as *mut SIMPLE_COM);
}


unsafe extern "C" fn make_until_or_while(
    mut which: command_type,
    mut test: *mut COMMAND,
    mut action: *mut COMMAND,
) -> *mut COMMAND {
    let mut temp: *mut WHILE_COM = 0 as *mut WHILE_COM;

    temp = xmalloc(std::mem::size_of::<WHILE_COM>() as usize) as *mut WHILE_COM;
    (*temp).flags = 0 as libc::c_int;
    (*temp).test = test;
    (*temp).action = action;
    return make_command(which as libc::c_uint, temp as *mut SIMPLE_COM);
}


#[no_mangle]
pub unsafe extern "C" fn make_while_command(
    mut test: *mut COMMAND,
    mut action: *mut COMMAND,
) -> *mut COMMAND {
    return make_until_or_while(command_type_cm_while, test, action);
}

#[no_mangle]
pub unsafe extern "C" fn make_until_command(
    mut test: *mut COMMAND,
    mut action: *mut COMMAND,
) -> *mut COMMAND {
    return make_until_or_while(command_type_cm_until, test, action);
}

#[no_mangle]
pub unsafe extern "C" fn make_arith_command(mut exp: *mut WordList) -> *mut COMMAND {
    let mut command: *mut COMMAND = 0 as *mut COMMAND;
    let mut temp: *mut ARITH_COM = 0 as *mut ARITH_COM;

    command = xmalloc(std::mem::size_of::<COMMAND>() as usize) as *mut COMMAND;
    temp = xmalloc(std::mem::size_of::<ARITH_COM>() as usize) as *mut ARITH_COM;
    (*command).value.Arith = temp;
    (*temp).flags = 0;
    (*temp).line = line_number;
    (*temp).exp = exp;
    (*command).type_ = command_type_cm_arith;
    (*command).redirects = 0 as *mut libc::c_void as *mut REDIRECT;
    (*command).flags = 0;

    return command;
}

#[no_mangle]
pub unsafe extern "C" fn make_cond_node(
    mut type_0: libc::c_int,
    mut op: *mut WordDesc,
    mut left: *mut cond_com,
    mut right: *mut cond_com,
) -> *mut COND_COM {
    let mut temp: *mut COND_COM = 0 as *mut COND_COM;

    temp = xmalloc(std::mem::size_of::<COND_COM>() as usize) as *mut COND_COM;
    (*temp).flags = 0;
    (*temp).line = line_number;
    (*temp).type_ = type_0;
    (*temp).op = op;
    (*temp).left = left;
    (*temp).right = right;

    return temp;
}

#[no_mangle]
pub unsafe extern "C" fn make_cond_command(
    mut cond_node: *mut COND_COM,
) -> *mut COMMAND {
    let mut command: *mut COMMAND = 0 as *mut COMMAND;

    command = xmalloc(std::mem::size_of::<COMMAND>() as usize) as *mut COMMAND;
    (*command).value.Cond = cond_node;
    (*command).type_ = command_type_cm_cond;
    (*command).redirects = 0 as *mut REDIRECT;
    (*command).flags = 0;
    (*command).line = if !cond_node.is_null() { (*cond_node).line } else { 0 };

    return command;
}

#[no_mangle]
pub unsafe extern "C" fn make_bare_simple_command() -> *mut COMMAND {
    let mut command: *mut COMMAND = 0 as *mut COMMAND;
    let mut temp: *mut SIMPLE_COM = 0 as *mut SIMPLE_COM;

    command = xmalloc(std::mem::size_of::<COMMAND>() as usize) as *mut COMMAND;
    temp = xmalloc(std::mem::size_of::<SIMPLE_COM>() as usize) as *mut SIMPLE_COM;
    (*command).value.Simple = temp;

    (*temp).flags = 0;
    (*temp).line = line_number;
    (*temp).words = 0 as *mut WordList;
    (*temp).redirects = 0 as *mut REDIRECT;

    (*command).type_ = command_type_cm_simple;
    (*command).redirects = 0 as *mut REDIRECT;
    (*command).flags = 0;

    return command;
}

#[no_mangle]
pub unsafe extern "C" fn make_simple_command(
    mut element: ELEMENT,
    mut command: *mut COMMAND,
) -> *mut COMMAND {
    if command.is_null() {
        command = make_bare_simple_command();
        parser_state |= PST_REDIRLIST as libc::c_int;
    }
    if !(element.word).is_null() {
        (*(*command).value.Simple).words = make_word_list(element.word, (*(*command).value.Simple).words);
        parser_state &= !(PST_REDIRLIST as libc::c_int);
    } else if !(element.redirect).is_null() {
        let mut r: *mut REDIRECT = element.redirect;
        while !((*r).next).is_null() {
            r = (*r).next;
        }
        (*r).next = (*(*command).value.Simple).redirects;
        (*(*command).value.Simple).redirects = element.redirect;
    }
    return command;
}

#[macro_export]
macro_rules! FASTCOPY {
    ($s:expr, $d:expr, $n:expr) => {
        libc::memcpy(
            $d as *mut libc::c_void,
            $s as *const libc::c_void,
            $n as libc::c_ulong as libc::size_t,
        );
    };
}


#[no_mangle]
pub unsafe extern "C" fn make_here_document(
    mut temp: *mut REDIRECT,
    mut lineno: libc::c_int,
) {
    let mut current_block: u64;
    let mut kill_leading: libc::c_int = 0;
    let mut redir_len: libc::c_int = 0;
    let mut redir_word: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut document: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut full_line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut document_index: libc::c_int = 0;
    let mut document_size: libc::c_int = 0;
    let mut delim_unquoted: libc::c_int = 0;

    if (*temp).instruction != r_instruction_r_deblank_reading_until 
        && (*temp).instruction != r_instruction_r_reading_until 
    {
        internal_error(
            b"make_here_document: bad instruction type %d\0" as *const u8 as *const libc::c_char,              
            (*temp).instruction as libc::c_uint,
        );
        return;
    }
    kill_leading = ((*temp).instruction == r_instruction_r_deblank_reading_until ) as libc::c_int;
    
    document = 0 as *mut libc::c_char;
    document_size = 0 ;
    document_index = document_size;
    
    redir_word = string_quote_removal(
        (*(*temp).redirectee.filename).word,
        0 ,
    );

    'lable1:loop {
        if !redir_word.is_null() {
            redir_len = strlen(redir_word) as libc::c_int;
        } else {
            (*temp).here_doc_eof = xmalloc(1 as usize) as *mut c_char;
            *(*temp).here_doc_eof.offset(0) = '\u{0}' as i32 as libc::c_char;
            break 'lable1; 
        }

        free((*(*temp).redirector.filename).word as *mut c_void);
        (*temp).here_doc_eof = redir_word;

        delim_unquoted = (((*(*temp).redirector.filename).flags & W_QUOTED as c_int) == 0) as c_int;
        
        full_line = read_secondary_line(delim_unquoted);
        while !full_line.is_null() {
            let mut line:*mut c_char;
            let mut len:i32;

            here_doc_first_line = 0;
            line = full_line;
            line_number += 1;

            if echo_input_at_read != 0 {
                fprintf(stderr, b"%s\0" as *const u8 as *const libc::c_char, line);
            }
            if kill_leading != 0 && *line as libc::c_int != 0 {
                if STREQN!(line, redir_word, redir_len as usize) != 0 && *line.offset(redir_len as isize) as libc::c_int == '\n' as i32
                {
                    break 'lable1;
                }
                while *line as libc::c_int == '\t' as i32 {
                    line = line.offset(1);
                }
            }

            if *line as libc::c_int == 0 {
                continue;
            }

            if STREQN!(line, redir_word, redir_len as usize) != 0 && *line.offset(redir_len as isize) as libc::c_int == '\n' as i32
            {
                break 'lable1;
            }

            len = strlen(line) as libc::c_int;
            if len + document_index >= document_size {
                document_size = if document_size != 0 {
                    2 * (document_size + len)
                } else {
                    len + 2 
                };
                document = xrealloc(
                    document as *mut libc::c_void,
                    document_size as usize
                ) as *mut libc::c_char;
            }

            FASTCOPY!(line, document.offset(document_index as isize), len);
            document_index += len;

            full_line = read_secondary_line(delim_unquoted);
        }

        if !full_line.is_null() {
            internal_warning(
                b"here-document at line %d delimited by end-of-file (wanted `%s')\0" as *const u8 as *const libc::c_char,    
                lineno,
                redir_word,
            );
        }

        break 'lable1;
    }

    if !document.is_null() {
        *document.offset(document_index as isize) = '\u{0}' as c_char;
    } else {
        document = xmalloc(1) as *mut libc::c_char;
        *document.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    }

    (*(*temp).redirectee.filename).word = document;
    here_doc_first_line = 0 as libc::c_int;
}


#[no_mangle]
pub unsafe extern "C" fn make_redirection(
    mut source: REDIRECTEE,
    mut instruction: r_instruction,
    mut dest_and_filename: REDIRECTEE,
    mut flags: libc::c_int,
) -> *mut REDIRECT {
    let mut temp: *mut REDIRECT = 0 as *mut REDIRECT;
    let mut w: *mut WordDesc = 0 as *mut WordDesc;
    let mut wlen: libc::c_int = 0;
    let mut lfd: intmax_t = 0;

    temp = xmalloc(std::mem::size_of::<REDIRECT>() as usize) as *mut REDIRECT;
    
    (*temp).redirector = source;
    (*temp).redirectee = dest_and_filename;
    (*temp).here_doc_eof = 0 as *mut libc::c_char;
    (*temp).instruction = instruction;
    (*temp).flags = 0 ;
    (*temp).rflags = flags;
    let ref mut fresh47 = (*temp).next;
    (*temp).next = 0 as *mut REDIRECT;

    match instruction as u32 {
        r_instruction_r_output_direction | 
        r_instruction_r_output_force | 
        r_instruction_r_err_and_out => {
            (*temp).flags = O_TRUNC as c_int | O_WRONLY as c_int| O_CREAT as c_int;
        }

        r_instruction_r_appending_to |
        r_instruction_r_append_err_and_out => {
            (*temp).flags = O_APPEND as c_int | O_WRONLY as c_int| O_CREAT as c_int;
        }

        r_instruction_r_input_direction |
        r_instruction_r_inputa_direction => {
            (*temp).flags = O_RDONLY as c_int;
        }

        r_input_output => {
            (*temp).flags = O_RDWR as c_int | O_CREAT as c_int;
        }

        r_instruction_r_deblank_reading_until |
        r_instruction_r_reading_until |
        r_instruction_r_reading_string |
        r_instruction_r_close_this |
        r_instruction_r_duplicating_input |
        r_instruction_r_duplicating_output => {

        }

        r_instruction_r_move_input |
        r_instruction_r_move_output |
        r_instruction_r_move_input_word |
        r_instruction_r_move_output_word => {

        }

        r_instruction_r_duplicating_input_word |
        r_instruction_r_duplicating_output_word => {
            w = dest_and_filename.filename;
            wlen = (strlen((*w).word) - 1) as c_int;
            if *((*w).word).offset(wlen as isize) as libc::c_int == '-' as i32 {
                *((*w).word).offset(wlen as isize) = '\u{0}' as i32 as libc::c_char;
                if all_digits((*w).word) != 0 && legal_number((*w).word, &mut lfd) != 0
                    && lfd == lfd as libc::c_long
                {
                    dispose_word(w);
                    (*temp).instruction = (if instruction == r_instruction_r_duplicating_input_word 
                    {
                        r_instruction_r_move_input as libc::c_int
                    } else {
                        r_instruction_r_move_output as libc::c_int
                    }) as r_instruction;
                    (*temp).redirectee.dest = lfd as libc::c_int;
                } else {
                    (*temp).instruction = (if instruction == r_instruction_r_duplicating_input_word
                    {
                        r_instruction_r_move_input_word as libc::c_int
                    } else {
                        r_instruction_r_move_output_word as libc::c_int
                    }) as r_instruction;
                }
            }
        }

        _ => {
            programming_error(
                b"make_redirection: redirection instruction `%d' out of range\0"
                    as *const u8 as *const libc::c_char,
                instruction as libc::c_uint,
            );
            abort();
        }
    }

    return temp;
}

#[macro_export]
macro_rules! GET_ARRAY_FROM_VAR {
    ($n:expr, $v:expr, $a:expr) => {
        $v = find_variable($n);
        $a = if !$v.is_null() && array_p!($v) != 0 {
            array_cell!($v)
        } else {
            0 as *mut ARRAY
        }
    };
}


#[no_mangle]
pub unsafe extern "C" fn make_function_def(
    mut name: *mut WordDesc,
    mut command: *mut COMMAND,
    mut lineno: libc::c_int,
    mut lstart: libc::c_int,
) -> *mut COMMAND {
    let mut temp: *mut FUNCTION_DEF = 0 as *mut FUNCTION_DEF;
    let mut bash_source_v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut bash_source_a: *mut ARRAY = 0 as *mut ARRAY;

    temp = xmalloc(std::mem::size_of::<FUNCTION_DEF>() as usize) as *mut FUNCTION_DEF;
    (*temp).command = command;
    let ref mut fresh49 = (*temp).name;
    (*temp).name = name;
    (*temp).line = lineno;
    (*temp).flags = 0;
    (*command).line = lstart;

    (*temp).source_file = 0 as *mut libc::c_char;
    GET_ARRAY_FROM_VAR!(b"BASH_SOURCE\0" as *const u8 as *const libc::c_char, bash_source_v, bash_source_a);
    if !bash_source_a.is_null() && array_num_elements!(bash_source_a) > 0 {
        (*temp).source_file = array_reference(bash_source_a, 0 as arrayind_t);
    }
    if ((*temp).source_file).is_null() {
        (*temp).source_file = (if shell_initialized != 0 {
            b"main\0" as *const u8 as *const libc::c_char
        } else {
            b"environment\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char;
    }

    bind_function_def((*name).word, temp, 0);
    let ref mut fresh53 = (*temp).source_file;
    (*temp).source_file = if !((*temp).source_file).is_null() {
        savestring!((*temp).source_file)
    } else {
        0 as *mut libc::c_char
    };
    return make_command(
        command_type_cm_function_def ,
        temp as *mut SIMPLE_COM,
    );
}


#[no_mangle]
pub unsafe extern "C" fn make_subshell_command(
    mut command: *mut COMMAND,
) -> *mut COMMAND {
    let mut temp: *mut SUBSHELL_COM = 0 as *mut SUBSHELL_COM;
    temp = xmalloc(std::mem::size_of::<SUBSHELL_COM>() as usize) as *mut SUBSHELL_COM;

    (*temp).command = command;
    (*temp).flags = 0x1 ;
    (*temp).line = line_number;
    return make_command(
        command_type_cm_subshell ,
        temp as *mut SIMPLE_COM,
    );
}


#[no_mangle]
pub unsafe extern "C" fn make_coproc_command(
    mut name: *mut libc::c_char,
    mut command: *mut COMMAND,
) -> *mut COMMAND {
    let mut temp: *mut COPROC_COM = 0 as *mut COPROC_COM;

    temp = xmalloc(std::mem::size_of::<COPROC_COM>() as usize) as *mut COPROC_COM;
    let ref mut fresh55 = (*temp).name;
    (*temp).name = savestring!(name);
    let ref mut fresh56 = (*temp).command;
    (*temp).command = command;
    (*temp).flags = CMD_WANT_SUBSHELL as libc::c_int | CMD_COPROC_SUBSHELL as libc::c_int;
    return make_command(
        command_type_cm_coproc,
        temp as *mut SIMPLE_COM,
    );
}

unsafe extern "C" fn output_requirement(
    mut deptype: *const libc::c_char,
    mut filename: *mut libc::c_char,
) {
    static mut alphabet_set: *mut libc::c_char = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    if !(strchr(filename, '$' as i32)).is_null()
        || *filename.offset(0 as libc::c_int as isize) as libc::c_int != '/' as i32
            && !(strchr(filename, '/' as i32)).is_null()
    {
        return;
    }
    if !(strpbrk(filename, alphabet_set)).is_null() {
        printf(b"%s(%s)\n\0" as *const u8 as *const libc::c_char, deptype, filename);
    }
}


#[no_mangle]
pub unsafe extern "C" fn clean_simple_command(
    mut command: *mut COMMAND,
) -> *mut COMMAND {
    if (*command).type_  != command_type_cm_simple {
        command_error(
            b"clean_simple_command\0" as *const u8 as *const libc::c_char,
            CMDERR_BADTYPE as libc::c_int,
            (*command).type_  as libc::c_int,
            0 ,
        );
    } else {
        (*(*command).value.Simple).words = REVERSE_LIST!((*(*command).value.Simple).words, *mut WordList);
        (*(*command).value.Simple).redirects = REVERSE_LIST!((*(*command).value.Simple).redirects, *mut REDIRECT);
    }

    if rpm_requires != 0 && !((*(*command).value.Simple).words).is_null() {
        let mut cmd0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut cmd1: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut b: *mut builtin = 0 as *mut builtin;

        cmd0 = (*(*(*(*command).value.Simple).words).word).word;
        b = builtin_address_internal(cmd0, 0 );
        cmd1 = 0 as *mut libc::c_char;
        if !((*(*(*command).value.Simple).words).next).is_null() {
            cmd1 = (*(*(*(*(*command).value.Simple).words).next).word).word;
        }

        if !b.is_null() {
            if (*b).flags & REQUIRES_BUILTIN as libc::c_int != 0 && !cmd1.is_null() {
                output_requirement(
                    b"executable\0" as *const u8 as *const libc::c_char,
                    cmd1,
                );
            }
        } else if assignment(cmd0, 0 as libc::c_int) == 0 {
            output_requirement(
                if !(find_function(cmd0)).is_null() {
                    b"function\0" as *const u8 as *const libc::c_char
                } else {
                    b"executable\0" as *const u8 as *const libc::c_char
                },
                cmd0,
            );
        }
    }
    parser_state &= !(PST_REDIRLIST as libc::c_int);
    return command;
}


#[no_mangle]
pub unsafe extern "C" fn connect_async_list(
    mut command: *mut COMMAND,
    mut command2: *mut COMMAND,
    mut connector: libc::c_int,
) -> *mut COMMAND {
    let mut t: *mut COMMAND = 0 as *mut COMMAND;
    let mut t1: *mut COMMAND = 0 as *mut COMMAND;
    let mut t2: *mut COMMAND = 0 as *mut COMMAND;

    t1 = command;
    t = (*(*command).value.Connection).second;

    if t.is_null() || (*command).flags & CMD_WANT_SUBSHELL as libc::c_int != 0
        || (*(*command).value.Connection).connector != ';' as i32
    {
        t = command_connect(command, command2, connector);
        return t;
    }

    while (*t).flags & CMD_WANT_SUBSHELL as libc::c_int == 0 as libc::c_int
        && (*t).type_  == command_type_cm_connection 
        && (*(*t).value.Connection).connector == ';' as i32
    {
        t1 = t;
        t = (*(*t).value.Connection).second;
    }
    t2 = command_connect(t, command2, connector);
    (*(*t1).value.Connection).second = t2;
    return command;
}


