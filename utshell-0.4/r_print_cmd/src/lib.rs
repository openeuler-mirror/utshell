//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

use libc::{c_int,c_char,FILE, c_void, fprintf,fileno, strnlen, size_t};
use std::ffi::{CString, CStr, };
use r_bash::*;
use rcommon::{WordDesc, WordList};


extern "C"{
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
}

pub type PFUNC = unsafe extern "C" fn (*const libc::c_char, ...) -> ();

pub const  FUNC_MULTILINE:i32 = 0x01;
pub const  FUNC_EXTERNAL:i32 = 0x02;

pub static mut indentation:c_int = 0;
pub static mut indentation_amount:c_int = 4;


pub const PRINTED_COMMAND_INITIAL_SIZE:c_int = 64;
pub const PRINTED_COMMAND_CROW_SIZE:c_int = 128;



static mut inside_function_def:c_int = 0;
static mut skip_this_indent:c_int = 0;
static mut was_heredoc:c_int = 0;
static mut printing_connection:c_int = 0;
static mut deferred_heredocs:*mut REDIRECT = 0 as *mut REDIRECT;

static mut group_command_nesting:c_int = 0;

static mut indirection_string:*mut c_char = 0 as *mut c_char;
static mut indirection_stringsiz:c_int = 0;



const MB_LEN_MAX:usize = 16; //在C端这是一个宏，
const MB_CUR_MAX:usize  =  1;
const AND_AND:i32 = 288;
const OR_OR:i32 = 289;

/* 宏定义 */
#[macro_export]
macro_rules! CHECK_XTRACE_FP{
    () => (
        if !xtrace_fp.is_null(){
            xtrace_fp = xtrace_fp;
        } 
        else{
            xtrace_fp = stderr;
        }
    )
}

#[macro_export]
macro_rules! EXPCHAR{
    ($c:expr) => (
        if $c == b'{' as c_char || $c == b'~' as c_char || $c == b'$' as c_char || $c == b'`' as c_char{
            1
        }
        else{
            0
        }
    )
}

#[macro_export]
macro_rules! PRINT_DEFERRED_HEREDOCS{
    ($x:expr) => (
        if !deferred_heredocs.is_null(){
            print_deferred_heredocs($x);
        }
    )
}


#[macro_export]
macro_rules! RESIZE_MALLOCED_BUFFER{
    ($str:expr, $cind:expr, $room:expr, $csize:expr, $simcr:expr) => (
        if $cind + $room >= $csize {
            while $cind + $room >= $csize {
                $csize = $csize + $simcr;
            }
            $str = libc::realloc($str as *mut c_void, $csize as usize ) as *mut c_char;
        }
    )
}

pub  fn MBLEN(s: *const c_char ,n:size_t) -> c_int
{
    if MB_CUR_MAX > 1
    {
        return unsafe { mblen(s,n) };
    }
    else {
        return  1;
    }
}



unsafe fn DIGIT(c:c_char) -> bool{
    char::from(c as u8 ) >= '0' && char::from(c as u8) <= '9'
}

//pub type Function = unsafe extern "C" fn() -> c_int;
pub type Function = ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>;

#[link(name = "print_cmd")]
extern "C" {
    static indirection_level:c_int;
    static posixly_correct:c_int;

    fn command_error(func:*const c_char, code:c_int, e:c_int, flags:c_int);
    fn sh_validfd(fd:c_int)->c_int;
    //fn internal_error(format:*mut c_char, arg1:*mut c_char, arg2:*mut c_char)->c_int;
    fn internal_error(format:*const c_char, _:...);
    fn internal_warning(_:*const c_char, _:...);
    fn get_string_value(_:*const c_char)->*mut c_char;
    //fn change_flag(flag:c_int, on_or_off:c_int)->c_int;
    fn decode_prompt_string(string:*mut c_char)->*mut c_char;
    fn sh_contains_shell_metas(string:*const c_char)->c_int;
    fn sh_single_quote(string:*mut c_char)->*mut c_char;
    fn ansic_shouldquote(string:*const c_char)->c_int;
    fn ansic_quote(str:*mut c_char, flags:c_int, rlen:*mut c_int)->*mut c_char;
    fn dispose_redirects(list:*mut REDIRECT);
    fn add_unwind_protect(cleanup:*mut Function, arg:*mut c_char);
    fn remove_unwind_protect();
    fn dispose_command(command:*mut COMMAND);
    fn find_reserved_word(tokstr:*mut c_char)->c_int;
    fn remove_quoted_escapes(string:*mut c_char)->*mut c_char;
    fn mblen(s:*const c_char, n:size_t)->c_int;
    fn cprintf(_:*const c_char,...);
}


#[no_mangle]
pub static mut the_printed_command: *mut libc::c_char = 0 as *mut libc::c_char;
#[no_mangle]
pub static mut the_printed_command_size: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut command_string_index: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut xtrace_fd: libc::c_int = -(1 as libc::c_int);
#[no_mangle]
pub static mut xtrace_fp: *mut FILE = 0 as *const FILE as *mut FILE;


#[no_mangle]
pub unsafe extern "C" fn print_command(command:*mut COMMAND)
{   let _str:*mut c_char;
    command_string_index = 0;
    print!("{}",CStr::from_ptr(make_command_string(command)).to_str().unwrap());
}

#[no_mangle]
pub unsafe extern "C" fn make_command_string(command:*mut COMMAND) -> *mut c_char
{
    command_string_index = 0;
    was_heredoc = 0;
    deferred_heredocs = 0 as *mut REDIRECT;
    make_command_string_internal(command);
    return the_printed_command;
}

#[no_mangle]
unsafe fn make_command_string_internal(command:*mut COMMAND)
{
    let mut s:[c_char;3] = [0;3];

    if command.is_null(){
        cprintf (b"\0" as *const u8 as *const c_char);
    }
    else{
        if skip_this_indent != 0 {
            skip_this_indent = skip_this_indent - 1;
        }
        else{
            indent(indentation);
        } 
        
        if (*command).flags != 0 && CMD_TIME_PIPELINE != 0{
            cprintf(b"time \0" as *const u8 as *const c_char);
            if (*command).flags != 0 && CMD_TIME_POSIX != 0{
                cprintf(b"-p \0" as *const u8 as *const c_char);
            }
        }

        if (*command).flags != 0 && CMD_INVERT_RETURN != 0{
            cprintf(b"! \0" as *const u8 as *const c_char);
        }
        // (*command).type_ = 11;
        match (*command).type_ as libc::c_uint {
            command_type_cm_for => print_for_command((*command).value.For),
            command_type_cm_arith_for => print_arith_for_command((*command).value.ArithFor),
            command_type_cm_select => print_select_command((*command).value.Select),
            command_type_cm_case => print_case_command((*command).value.Case),
            command_type_cm_while => print_while_command((*command).value.While),
            command_type_cm_until => print_until_command((*command).value.While),
            command_type_cm_if => print_if_command((*command).value.If),
            command_type_cm_arith => print_arith_command((*(*command).value.Arith).exp),
            command_type_cm_cond => print_cond_command((*command).value.Cond),
            command_type_cm_simple => print_simple_command((*command).value.Simple),
            command_type_cm_connection => {
                skip_this_indent = skip_this_indent + 1;
                printing_connection = printing_connection + 1;
                make_command_string_internal((*(*command).value.Connection).first);
                let _str_1 = b'&' as i32;
                let _str_1 = b'|' as i32;
                let _str_3 = b';' as i32;
                match (*(*command).value.Connection).connector{
                    38 | 124 => {
                        let c:c_char = (*(*command).value.Connection).connector as c_char;
                        s[0] = ' ' as i32 as c_char;
                        s[1] = c;
                        s[2] = '\u{0}' as i32 as c_char;

                        print_deferred_heredocs(s.as_mut_ptr());

                        if c as c_int != '&' as i32 || !((*(*command).value.Connection).second.is_null()){
                            cprintf(b" \0" as *const u8 as *const c_char);
                            skip_this_indent = skip_this_indent + 1;
                        }
                    }
                    AND_AND => {
                        print_deferred_heredocs(b" && \0" as *const u8 as *const c_char);
                        if !(*(*command).value.Connection).second.is_null()
                        {
                            skip_this_indent = skip_this_indent + 1;
                        }
                    }
                    OR_OR => {
                        print_deferred_heredocs(b" || \0" as *const u8 as *const c_char);
                        if !(*(*command).value.Connection).second.is_null() {
                            skip_this_indent = skip_this_indent + 1;
                        }
                    }
                    // str_3 => {
                    59 => {    
                        if deferred_heredocs.is_null() {
                            if was_heredoc == 0 
                            {
                                cprintf(b";\0" as *const u8 as *const c_char);
                            }
                            else{
                                was_heredoc = 0;
                            }
                        }
                        else{
                            print_deferred_heredocs(
                                if inside_function_def != 0{
                                    b"\0" as *const u8 as *const c_char
                                }
                                else{
                                    b";\0" as *const u8 as *const c_char
                                },
                            )
                        }

                        if inside_function_def != 0 {
                            cprintf(b"\n\0" as *const u8 as *const c_char);
                        }
                        else{
                            cprintf(b" \0" as *const u8 as *const c_char);
                            if !(*(*command).value.Connection).second.is_null() {
                                skip_this_indent = skip_this_indent + 1;
                            }
                        }
                    }
                    _ => {
                        cprintf(b"print_command:bas connector %s \0" as *const u8 as *const c_char,(*(*command).value.Connection).connector);
                    }
                }

                make_command_string_internal((*(*command).value.Connection).second);
                PRINT_DEFERRED_HEREDOCS!(b"\0" as *const u8 as *const c_char);
                printing_connection = printing_connection - 1;
            }

            command_type_cm_function_def => print_function_def((*command).value.Function_def),
            command_type_cm_group => print_group_command((*command).value.Group),
            command_type_cm_subshell =>{
                cprintf(b"( \0" as *const u8 as *const c_char);
                skip_this_indent = skip_this_indent + 1;
                make_command_string_internal((*(*command).value.Subshell).command);
                PRINT_DEFERRED_HEREDOCS!(b"\0" as *const u8 as *const c_char);
                cprintf(b" )\0" as *const u8 as *const c_char);
            }
            command_type_cm_coproc => {
                cprintf(b"coproc %s \0" as *const u8 as *const c_char,(*(*command).value.Coproc).name);
                skip_this_indent += 1;
                make_command_string_internal((*(*command).value.Coproc).command);
            }
            _ => {
                // let c_str = CString::new("print_command").unwrap();
                // let c_str_ptr = c_str.as_ptr();
                command_error(b"print_command\0" as *const u8 as *const c_char, CMDERR_BADTYPE as i32, (*command).type_ as i32, 0 as i32);
            }
        }

        if !(*command).redirects.is_null() {
            cprintf(b" \0" as *const u8 as *const c_char); 
            print_redirection_list((*command).redirects);
        }
    }
}


#[no_mangle]
pub unsafe extern "C" fn print_word_list(list:*mut WordList, separator:*mut c_char)
{
    let mut w:*mut WordList;
    // let mut str:*mut c_char;
    w = list;

    
    while !w.is_null() {
        if !(*w).word.is_null(){          
            print!("{}{}",CStr::from_ptr((*(*w).word).word).to_str().unwrap(),CStr::from_ptr(separator).to_str().unwrap());
        }
        else {      
            print!("{}",CStr::from_ptr((*(*w).word).word).to_str().unwrap());
        }

        w = (*w).next;
    }
}

#[no_mangle]
pub unsafe extern "C" fn xtrace_set(fd:c_int, fp:*mut FILE)
{
    if fd >= 0 && sh_validfd(fd) == 0 {
        internal_error(b"xtrace_set: %d: invalid file descriptor\0" as *const u8 as *const c_char as *mut c_char, fd );
        return;
    }

    if fp.is_null() {
        internal_error(b"xtrace_set: NULL file pointer\0" as *const u8 as *const c_char);
        return;
    }

    if fd >= 0 && fileno(fp) != fd {
        internal_warning(b"xtrace fd (%d) != fileno xtrace fp (%d)\0" as *const u8 as *const c_char as *mut c_char, fd, fileno(fp));
    }

    xtrace_fd = fd;
    xtrace_fp = fp;
}

#[no_mangle]
pub unsafe extern "C" fn xtrace_init()
{
    xtrace_set(-1, stderr);
}

#[no_mangle]
pub unsafe extern "C" fn xtrace_reset()
{
    if xtrace_fd >= 0  && !xtrace_fp.is_null(){
        libc::fflush(xtrace_fp);
        libc::fclose(xtrace_fp);
    }
    else if xtrace_fd >= 0{
        libc::close(xtrace_fd);
    }

    xtrace_fd = -1;
    xtrace_fp = stderr;
}

#[no_mangle]
pub unsafe extern "C" fn xtrace_fdchk(fd:c_int)
{
    if fd == xtrace_fd{
        xtrace_reset();
    }
}

#[no_mangle]
pub unsafe extern "C" fn indirection_level_string()->*mut c_char
{
    let mut i:c_int;
    let mut j:c_int;
    let mut ps4:*mut c_char;
    let mut ps4_firstc:[c_char;MB_LEN_MAX+1] = [0;MB_LEN_MAX+1];
    let ps4_firstc_len:c_int;
    let ps4_len:c_int;
    let ineed:c_int;
    let old:c_int;

    ps4 = get_string_value(b"PS4\0" as *const u8 as *const c_char);

    if indirection_string.is_null(){
        indirection_stringsiz = 100 ;
        indirection_string = libc::malloc(100) as *mut c_char;
    }
    *indirection_string.offset(0 as isize) = '\u{0}' as i32 as c_char;

    if ps4.is_null() || *ps4 as c_int == '\u{0}' as i32{
        return indirection_string;
    }

    old = change_flag('x' as i32, FLAG_OFF as i32);
    ps4 = decode_prompt_string(ps4);
    if old != 0 {
        change_flag('x' as i32 , FLAG_ON.into());
    }

    if ps4.is_null() || *ps4 as c_int == '\u{0}' as i32 {
        return indirection_string;
    }

    ps4_len = strnlen(ps4 as *const c_char , MB_CUR_MAX) as c_int;

    ps4_firstc_len = MBLEN(ps4, ps4_len as size_t);
    if ps4_firstc_len == 1 || ps4_firstc_len == 0 || ps4_firstc_len < 0{
        ps4_firstc[0] = *ps4.offset(0 as isize);
        ps4_firstc[1] = '\u{0}' as i32 as c_char;
    }
    else{
        libc::memcpy(ps4_firstc.as_mut_ptr() as *mut c_void, ps4  as *const c_void , ps4_firstc_len as usize);
    }

    ineed = ((ps4_firstc_len * indirection_level) as libc::c_ulong).wrapping_add(libc::strlen(ps4).try_into().unwrap()) as c_int;
    if ineed > indirection_stringsiz - 1{
        indirection_stringsiz = ineed + 1;
        indirection_string = libc::realloc(indirection_string as *mut c_void, indirection_stringsiz as size_t) as *mut c_char;
    }

    i = 0;
    j = 0;
    while j < indirection_level && (i < indirection_stringsiz - 1)  {

        if ps4_firstc_len == 1{
            *indirection_string.offset(i as isize) = ps4_firstc[0];
        }
        else{
            libc::memcpy(indirection_string.offset(i as isize) as *mut c_void, ps4_firstc.as_mut_ptr() as *const c_void, ps4_firstc_len as usize);
        }

        i += ps4_firstc_len;
        j += 1;
    }

    j = ps4_firstc_len;
    while *ps4 as c_int != 0 && *ps4.offset(j as isize) != 0 && (i < indirection_stringsiz -1 ) {
        *indirection_string.offset(i as isize) = *ps4.offset(j as isize);

        i += 1;
        j += 1;
    }

    *indirection_string.offset(i as isize) = '\u{0}' as i32 as c_char;
    libc::free(ps4 as *mut libc::c_void);
    return indirection_string;    

}


#[no_mangle]
pub unsafe  extern "C" fn xtrace_print_assignment(name:*mut c_char, value:*mut c_char, assign_list:c_int,xflags:c_int)
{
    let nval:*mut c_char;

    CHECK_XTRACE_FP!();
    if xflags != 0{
        fprintf(xtrace_fp, CString::new("%s").unwrap().as_ptr(), indirection_level_string());
    }

    if *value as c_int == '\u{0}' as i32 || assign_list != 0{
        nval = value;
    }
    else if sh_contains_shell_metas(value) != 0{
        nval = sh_single_quote(value);
    }
    else if ansic_shouldquote(value) != 0{
        nval = ansic_quote(value, 0, 0 as *mut c_int);
    }
    else {
        nval = value;
    }

    if assign_list != 0{
        fprintf(xtrace_fp, CString::new("%s=(%s)\n").unwrap().as_ptr(), name, nval);

    }
    else{
        fprintf(xtrace_fp, CString::new("%s=%s\n").unwrap().as_ptr(), name, nval);
    }

    if nval != value  {
        if !nval.is_null() {
            libc::free(nval as *mut c_void);
        }
    }

    libc::fflush(xtrace_fp);

}


#[no_mangle]
pub unsafe extern "C" fn xtrace_print_word_list(list:*mut WordList, xtflags: c_int) 
{
    let mut w:*mut WordList;
    let mut t:*mut c_char;
    let mut x:*mut c_char;

    CHECK_XTRACE_FP!();

    if (xtflags & 1) != 0 {
        fprintf(xtrace_fp, CString::new("%s").unwrap().as_ptr(), indirection_level_string());
    }

    w = list;
    while !w.is_null(){

        t = (*(*w).word).word;
        if t.is_null() || *t as c_int == '\u{0}' as i32{
            fprintf(xtrace_fp, CString::new("''%s").unwrap().as_ptr(), 
                    if !((*w).next).is_null(){
                        b" \0" as *const u8 as *const c_char
                    }else{
                        b"\0" as *const u8 as *const c_char
                    },
            );
        }
        else if (xtflags & 2) != 0{
            fprintf(xtrace_fp, CString::new("%s%s").unwrap().as_ptr(),t,
                if !((*w).next).is_null(){
                    b" \0" as *const u8 as *const c_char
                }else{
                    b"\0" as *const u8 as *const c_char
                },
            );
        }
        else if sh_contains_shell_metas(t) != 0{
            x = sh_single_quote(t);
            fprintf(xtrace_fp,CString::new("%s%s").unwrap().as_ptr(),x,
                if !((*w).next).is_null(){
                    b" \0" as *const u8 as *const c_char
                }else{
                    b"\0" as *const u8 as *const c_char
                },
            );
            libc::free(x as *mut c_void);
        }
        else if ansic_shouldquote(t) != 0{
            x = ansic_quote(t,0,0 as *mut c_int);
            fprintf(xtrace_fp, CString::new("%s%s").unwrap().as_ptr(), x,
                if !((*w).next).is_null(){
                    b" \0" as *const u8 as *const c_char
                }else{
                    b"\0" as *const u8 as *const c_char
                },
            );
            libc::free(x as *mut c_void);
        }
        else{
            fprintf(xtrace_fp, CString::new("%s%s").unwrap().as_ptr(), t,
                if !((*w).next).is_null(){
                    b" \0" as *const u8 as *const c_char
                }else{
                    b"\0" as *const u8 as *const c_char
                },
            );
        }
        w = (*w).next;
    }

    fprintf(xtrace_fp, CString::new("\n").unwrap().as_ptr());
    libc::fflush(xtrace_fp);
}


#[no_mangle]
pub unsafe extern "C" fn command_print_word_list(list:*mut WordList, separator:*mut c_char)
{
    let mut w:*mut WordList;
    w = list;

    while !w.is_null() {

        if !(*w).next.is_null(){
            cprintf(b"%s%s\0" as *const u8 as *const c_char,(*(*w).word).word,separator);
        }
        else {
            cprintf(b"%s\0" as *const u8 as *const c_char,(*(*w).word).word);
        }
        
        if !(*w).next.is_null() {
            
        }

        w = (*w).next;
    }
}

// 有个cprintf函数
#[no_mangle]
pub unsafe extern "C" fn print_for_command_head(for_command:*mut FOR_COM)
{
    cprintf(b"for %s in  \0" as *const u8 as *const c_char,(*(*for_command).name).word);
    command_print_word_list((*for_command).map_list, b" \0" as *const u8  as *mut c_char);
}

#[no_mangle]
pub unsafe extern "C" fn xtrace_print_for_command_head(for_command:*mut FOR_COM){
    CHECK_XTRACE_FP!();
    fprintf(xtrace_fp, CString::new("%s").unwrap().as_ptr(), indirection_level_string());
    fprintf(xtrace_fp,CString::new("for %s in ").unwrap().as_ptr(),(*(*for_command).name).word);
    xtrace_print_word_list((*for_command).map_list,2);
}

#[no_mangle]
pub unsafe extern "C" fn print_for_command(for_command:*mut FOR_COM)
{
    print_for_command_head(for_command);
    cprintf(b";\0" as *const u8 as *const c_char);
    newline(b"do\n\0" as *const u8 as *const c_char as *mut c_char);

    indentation += indentation_amount;
    make_command_string_internal((*for_command).action);
    PRINT_DEFERRED_HEREDOCS!(b"\0" as *const u8 as *mut c_char);
    semicolon();
    indentation -= indentation_amount;

    newline(b"done\0" as *const u8 as *const c_char as *mut c_char);
}

#[no_mangle]
pub unsafe extern "C" fn print_arith_for_command(arith_for_command:*mut ARITH_FOR_COM)
{
    cprintf(b"for ((\0" as *const u8 as *const c_char );
    command_print_word_list((*arith_for_command).init, b" \0" as *const u8 as *const c_char as *mut c_char);
    cprintf(b"; \0" as *const u8 as *const c_char);
    command_print_word_list((*arith_for_command).test, b" \0" as *const u8 as *const c_char as *mut c_char);
    cprintf(b"; \0" as *const u8 as *const c_char);
    command_print_word_list((*arith_for_command).step, b" \0" as *const u8 as *const c_char as *mut c_char);
    cprintf(b"))\0" as *const u8 as *const c_char);
    newline(b"do\n\0" as *const u8 as *const c_char as *mut c_char);

    indentation += indentation_amount;
    make_command_string_internal((*arith_for_command).action);
    PRINT_DEFERRED_HEREDOCS!(b"\0" as *const u8 as *const c_char);
    semicolon();
    indentation -= indentation_amount;
    newline(b"done\0" as *const u8 as *const c_char as *mut c_char);
}

#[no_mangle]
pub unsafe extern "C" fn print_select_command_head(select_command:*mut SELECT_COM)
{
    cprintf(b"select %s in \0" as *const u8 as *const c_char,(*(*select_command).name).word);
    command_print_word_list((*select_command).map_list, b" \0" as *const u8 as *const c_char as *mut c_char);
}


#[no_mangle]
pub unsafe extern "C" fn xtrace_print_select_command_head(select_command:*mut SELECT_COM)
{
    CHECK_XTRACE_FP!();
    fprintf(xtrace_fp, CString::new("%s").unwrap().as_ptr(),indirection_level_string());
    fprintf(xtrace_fp, CString::new("select %s in ").unwrap().as_ptr(),(*(*select_command).name).word);
    xtrace_print_word_list((*select_command).map_list, 2);
}

#[no_mangle]
pub unsafe extern "C" fn print_select_command(select_command:*mut SELECT_COM)
{
    print_select_command_head(select_command);

    cprintf(b";\0" as *const u8 as *const c_char);
    newline(b"do\n\0" as *const u8 as *const c_char as *mut c_char);
    indentation += indentation_amount;
    make_command_string_internal((*select_command).action);
    PRINT_DEFERRED_HEREDOCS!(b"\0" as *const u8 as *const c_char);
    semicolon();
    indentation -= indentation_amount;
    newline(b"done\0" as *const u8 as *const c_char as *mut c_char);
}

#[no_mangle]
pub unsafe extern "C" fn print_group_command(group_command:*mut GROUP_COM)
{
    group_command_nesting += 1;
    cprintf(b"{{ \0" as *const u8 as *const c_char);
    if inside_function_def == 0{
        skip_this_indent += 1;
    }
    else {
        cprintf(b"\n\0" as *const u8 as *const c_char);
        indentation += indentation_amount;
    }

    make_command_string_internal((*group_command).command);
    PRINT_DEFERRED_HEREDOCS!(b"\0" as *const u8 as *const c_char);

    if inside_function_def != 0{
        cprintf(b"\n\0" as *const u8 as *const c_char);
        indentation -= indentation_amount;
        indent(indentation);
    }
    else{
        semicolon();
        cprintf(b" \0" as *const u8 as *const c_char);
    }
    cprintf(b"}\0" as *const u8 as *const c_char);
    group_command_nesting -= 1;
}


#[no_mangle]
pub unsafe extern "C" fn print_case_command_head(case_command:*mut CASE_COM)
{
    cprintf(b"case %s in \0" as *const u8 as *const c_char,(*(*case_command).word).word);
}


#[no_mangle]
pub unsafe extern "C" fn xtrace_print_case_command_head(case_command:*mut CASE_COM)
{
    CHECK_XTRACE_FP!();
    fprintf(xtrace_fp, CString::new("%s").unwrap().as_ptr(), indirection_level_string());
    fprintf(xtrace_fp,CString::new("case %s in\n").unwrap().as_ptr(), (*(*case_command).word).word);
}

#[no_mangle]
pub unsafe extern "C" fn print_case_command(case_command:*mut CASE_COM)
{
    print_case_command_head(case_command);

    if !(*case_command).clauses.is_null()
    {
        print_case_clauses((*case_command).clauses);
    }
    newline(b"esac\0" as *const u8 as *const c_char as *mut c_char);
}


#[no_mangle]
pub unsafe extern "C" fn print_case_clauses(mut clauses:*mut PATTERN_LIST)
{
    indentation += indentation_amount;
    while !clauses.is_null()
    {
        newline(b"\0" as *const u8 as *const c_char as *mut c_char);
        command_print_word_list((*clauses).patterns, CString::new(" | ").unwrap().as_ptr() as *mut c_char);

        cprintf(b")\n\0" as *const u8 as *const c_char);
        indentation += indentation_amount;
        make_command_string_internal((*clauses).action);
        indentation -= indentation_amount;
        PRINT_DEFERRED_HEREDOCS!(b"\0" as *const u8 as *const c_char);
        if (*clauses).flags & CASEPAT_FALLTHROUGH as i32 != 0{
            newline(b";&\0" as *const u8 as *const c_char as *mut c_char);
        }
        else if (*clauses).flags & CASEPAT_TESTNEXT as i32 != 0{
            newline(b";;&\0" as *const u8 as *const c_char as *mut c_char);
        }
        else {
            newline(b";;\0" as *const u8 as *const c_char as *mut c_char);
        }
        clauses = (*clauses).next;
    }
    indentation -= indentation_amount;
}

#[no_mangle]
pub unsafe extern "C" fn print_while_command(while_command:*mut WHILE_COM)
{
    print_until_or_while(while_command, CString::new("while").unwrap().as_ptr() as *mut c_char);
}


#[no_mangle]
pub unsafe extern "C" fn print_until_command(while_command:*mut WHILE_COM)
{
    print_until_or_while(while_command,CString::new("until").unwrap().as_ptr() as *mut c_char);
}

#[no_mangle]
pub unsafe extern "C" fn print_until_or_while(while_command:*mut WHILE_COM, which:*mut c_char)
{
    cprintf(b"%s\0" as *const u8 as *const c_char,which);
    skip_this_indent += 1;
    make_command_string_internal((*while_command).test);
    PRINT_DEFERRED_HEREDOCS!(b"\0" as *const u8 as *const c_char);
    semicolon();
    cprintf(b"do\n\0" as *const u8 as *const c_char);
    indentation += indentation_amount;
    make_command_string_internal((*while_command).action);
    PRINT_DEFERRED_HEREDOCS!(b"\0" as *const u8 as *const c_char);
    indentation -= indentation_amount;
    semicolon();
    newline(b"done\0" as *const u8 as *const c_char as *mut c_char);
}


#[no_mangle]
pub unsafe extern "C" fn print_if_command(if_command:*mut IF_COM)
{
    cprintf(b"if \0" as *const u8 as *const c_char);
    skip_this_indent += 1;
    make_command_string_internal((*if_command).test);
    semicolon();
    cprintf(b" then\n\0" as *const u8 as *const c_char);
    indentation += indentation_amount;
    make_command_string_internal((*if_command).true_case);
    PRINT_DEFERRED_HEREDOCS!(b"\0" as *const u8 as *const c_char);
    indentation -= indentation_amount;

    if !(*if_command).false_case.is_null()
    {
        semicolon();
        newline(b"else\n\0" as *const u8 as *const c_char as *mut c_char);
        indentation += indentation_amount;
        make_command_string_internal((*if_command).false_case);
        PRINT_DEFERRED_HEREDOCS!(b"\0" as *const u8 as *const c_char);
        indentation -= indentation_amount;
    }
    
    semicolon();
    newline(b"fi\0" as *const u8 as *const c_char as *mut c_char );
}



#[no_mangle]
pub unsafe extern "C" fn print_arith_command(arith_cmd_list:*mut WordList)
{
    cprintf(b"((\0" as *const u8 as *const c_char);
    command_print_word_list(arith_cmd_list, CString::new(" ").unwrap().as_ptr() as *mut c_char);
    cprintf(b"))\0" as *const u8 as *const c_char);
}

#[no_mangle]
pub unsafe extern "C" fn print_cond_node(cond:*mut COND_COM)
{
    if (*cond).flags & CMD_INVERT_RETURN as i32 != 0
    {
        cprintf(b"! \0" as *const u8 as *const c_char);
    }

    if (*cond).type_ == COND_EXPR as i32{
        cprintf(b"( \0" as *const u8 as *const c_char);
        print_cond_node((*cond).left);
        cprintf(b" )\0" as *const u8 as *const c_char);
    }
    else if (*cond).type_ == COND_AND as i32
    {
        print_cond_node((*cond).left);
        cprintf(b" && \0" as *const u8 as *const c_char);
        print_cond_node((*cond).right);
    }
    else if (*cond).type_ == COND_OR as i32
    {
        print_cond_node((*cond).left);
        cprintf(b" || \0" as *const u8 as *const c_char);
        print_cond_node((*cond).right);
    }
    else if (*cond).type_ == COND_UNARY as i32
    {
        cprintf((*(*cond).op).word);
        cprintf(b" \0" as *const u8 as *const c_char);
        print_cond_node((*cond).left);
    }
    else if (*cond).type_ == COND_BINARY as i32
    {
        print_cond_node((*cond).left);
        cprintf(b" \0" as *const u8 as *const c_char);
        cprintf((*(*cond).op).word);
        cprintf(b" \0" as *const u8 as *const c_char);
        print_cond_node((*cond).right);
    }
    else if(*cond).type_ == COND_TERM as i32
    {
        cprintf((*(*cond).op).word);
    }
}


#[no_mangle]
pub unsafe extern "C" fn print_cond_command(cond:*mut COND_COM)
{
    cprintf(b"[[ \0" as *const u8 as *const c_char);
    print_cond_node(cond);
    cprintf(b" ]]\0" as *const u8 as *const c_char);
}

//ifdef debug中没有内容
#[no_mangle]
pub unsafe extern "C" fn xtrace_print_cond_term(type_0:c_int, invert:c_int, op:*mut WordDesc, arg1:*mut c_char, arg2:*mut c_char)
{
    CHECK_XTRACE_FP!();
    command_string_index = 0;
    fprintf(xtrace_fp, CString::new("%s").unwrap().as_ptr(), indirection_level_string());
    fprintf(xtrace_fp, CString::new("[[ ").unwrap().as_ptr());

    if invert != 0
    {
        fprintf(xtrace_fp, CString::new("! ").unwrap().as_ptr());
    }

    if type_0 == COND_UNARY as i32
    { 
        let str:*mut c_char;
        if !arg1.is_null() && *arg1 as c_int != 0
        {
            str = arg1;
        }
        else{
            str = CString::new("''").unwrap().as_ptr() as *mut c_char;
        }
        fprintf(xtrace_fp, CString::new("%s ").unwrap().as_ptr(), (*op).word);
        fprintf(xtrace_fp, CString::new("%s ").unwrap().as_ptr(), str);
    }

    else if type_0 == COND_BINARY as i32
    {
        let str1:*mut c_char;
        let str2:*mut c_char;
        if !arg1.is_null() && *arg1 as c_int != 0
        {
            str1 = arg1;
        }
        else{
            str1 = CString::new("''").unwrap().as_ptr() as *mut c_char;
        }

        if !arg2.is_null() && *arg2 as c_int != 0
        {
            str2 = arg2;
        }
        else{
            str2 = CString::new("''").unwrap().as_ptr() as *mut c_char;
        }

        fprintf(xtrace_fp, CString::new("%s ").unwrap().as_ptr(), str1);
        fprintf(xtrace_fp, CString::new("%s ").unwrap().as_ptr(), (*op).word);
        fprintf(xtrace_fp, CString::new("%s ").unwrap().as_ptr(), str2);
    }

    fprintf(xtrace_fp, CString::new(" ]]\n").unwrap().as_ptr());

    libc::fflush(xtrace_fp);
}


#[no_mangle]
pub unsafe extern "C" fn xtrace_print_arith_cmd(list:*mut WordList)
{
    let mut w:*mut WordList;
    
    CHECK_XTRACE_FP!();
    fprintf(xtrace_fp, CString::new("%s").unwrap().as_ptr(), indirection_level_string());
    fprintf(xtrace_fp, CString::new("(( ").unwrap().as_ptr());

    w = list;
    while !w.is_null()
    {
        let str:*const c_char;
        if !((*w).next).is_null()
        {
            str = b" \0" as *const u8 as *const c_char;
        }
        else{
            str = b"\0" as *const u8 as *const c_char;
        }

        fprintf(xtrace_fp, CString::new("%s%s").unwrap().as_ptr(), (*(*w).word).word, str);

        w = (*w).next;
    }

    fprintf(xtrace_fp, CString::new(" ))\n").unwrap().as_ptr());

    libc::fflush(xtrace_fp);
}


#[no_mangle]
pub unsafe extern "C" fn print_simple_command(simple_command:*mut SIMPLE_COM)
{
    command_print_word_list((*simple_command).words, b" \0" as *const u8 as *const c_char as *mut c_char);
    if !(*simple_command).redirects.is_null()
    {
        cprintf(b" \0" as *const u8 as *const c_char);
        print_redirection_list((*simple_command).redirects);
    }
}

#[no_mangle]
pub unsafe extern "C" fn print_heredocs(heredocs:*mut REDIRECT)
{
    let mut hdtail:*mut REDIRECT;

    cprintf(b" \0" as *const u8 as *const c_char);

    hdtail = heredocs;
    while !hdtail.is_null()
    {
        print_redirection(hdtail);
        cprintf(b"\n\0" as *const u8 as *const c_char);

        hdtail = (*hdtail).next;
    }

    was_heredoc = 1;
}


#[no_mangle]
pub unsafe extern "C" fn print_heredoc_bodies(heredocs:*mut REDIRECT)
{
    let mut hdtail:*mut REDIRECT;

    cprintf(b"\n\0" as *const u8 as *const c_char);
    hdtail = heredocs;
    while !hdtail.is_null()
    {

        print_heredoc_body(hdtail);
        cprintf(b"\n\0" as *const u8 as *const c_char);

        hdtail = (*hdtail).next;
    }

    was_heredoc = 1;
}


#[no_mangle]
pub unsafe extern "C" fn print_deferred_heredocs(cstring:*const c_char)
{
    if !cstring.is_null() && *cstring.offset(0) != 0 && (*cstring.offset(0) != ';' as c_char|| *cstring.offset(1) != 0)
    {
        cprintf( cstring as *const c_char);
    }

    if !deferred_heredocs.is_null()
    {
        print_heredoc_bodies(deferred_heredocs);
        if !cstring.is_null() && *cstring.offset(0) != 0 && (*cstring.offset(0) != ';' as c_char|| *cstring.offset(1) != 0)
        {
            cprintf(b" \0" as *const u8 as *const c_char);
        }
        dispose_redirects(deferred_heredocs);
        was_heredoc = 1;
    }
    deferred_heredocs = std::ptr::null_mut() as *mut REDIRECT; 
}

#[no_mangle]
pub unsafe extern "C" fn print_redirection_list(mut redirects:*mut REDIRECT)
{
    let mut heredocs:*mut REDIRECT;
    let mut hdtail:*mut REDIRECT;
    let mut newredir:*mut REDIRECT;

    let mut rw:*mut c_char;

    heredocs = std::ptr::null_mut();
    hdtail = heredocs;

    was_heredoc = 0;

    while !redirects.is_null()
    {
        if (*redirects).instruction == r_instruction_r_reading_until || (*redirects).instruction == r_instruction_r_deblank_reading_until
        {
            newredir = copy_redirect(redirects);
            (*newredir).next = std::ptr::null_mut();

            print_heredoc_header(newredir);

            if !heredocs.is_null()
            {
                (*hdtail).next = newredir;
                hdtail = newredir;
            }
            else{
                hdtail = newredir;
                heredocs = newredir;

            }
        }
        else if (*redirects).instruction == r_instruction_r_duplicating_output_word && ((*redirects).flags & REDIR_VARASSIGN as i32)  == 0 && (*redirects).redirector.dest == 1
        {
            rw = (*(*redirects).redirectee.filename).word;
            if !rw.is_null() && *rw as c_int != '-' as i32 && DIGIT(*rw) == false && EXPCHAR!(*rw) == 0
            {
                (*redirects).instruction = r_instruction_r_err_and_out;
            }
            print_redirection(redirects);
            (*redirects).instruction = r_instruction_r_duplicating_output_word;
        }
        else{
            print_redirection(redirects);
        }
        redirects = (*redirects).next;
        if !redirects.is_null()
        {
            cprintf(b" \0" as *const u8 as *const c_char);
        }
    }

    if !heredocs.is_null() && printing_connection != 0
    {
        deferred_heredocs = heredocs;
    }
    else if !heredocs.is_null() {
        print_heredoc_bodies(heredocs);
        dispose_redirects(heredocs);
    }
}


#[no_mangle]
pub unsafe extern "C" fn print_heredoc_header(redirect:*mut REDIRECT)
{
    let kill_leading:c_int;
    let x:*mut c_char;

    kill_leading = (r_instruction_r_deblank_reading_until == (*redirect).instruction) as c_int;
    
    if (*redirect).rflags & REDIR_VARASSIGN as i32 != 0
    {
        cprintf( b"{%s}\0" as *const u8 as *const c_char,(*(*redirect).redirector.filename).word);
    }
    else if (*redirect).redirector.dest != 0
    {
        cprintf(b"%d\0" as *const u8 as *const c_char,(*redirect).redirector.dest)
    }
    if (*(*redirect).redirectee.filename).flags & W_QUOTED as i32 !=0
    {
        x = sh_single_quote((*redirect).here_doc_eof);
        cprintf (b"<<%s%s\0" as *const u8 as *const c_char, if kill_leading !=0 {
            b"-\0"  as *const u8 as *const c_char
        } 
        else  {
            b"\0"  as *const u8 as *const c_char
        }, x);  
        libc::free(x as *mut c_void);
    }
    else{
        cprintf (b"<<%s%s\0" as *const u8 as *const c_char, 
        if kill_leading !=0 {
            b"-\0"  as *const u8 as *const c_char
        } 
        else  {
            b"\0"  as *const u8 as *const c_char
        }, (*redirect).here_doc_eof);
    }
    
}


#[no_mangle]
pub unsafe extern "C" fn print_heredoc_body(redirect:*mut REDIRECT)
{
    cprintf (b"%s%s\0" as *const u8 as *const c_char, (*(*redirect).redirectee.filename).word,(*redirect).here_doc_eof);
}

#[no_mangle]
pub unsafe extern "C" fn print_redirection(redirect:*mut REDIRECT)
{
    let redirector:c_int;
    let redir_fd:c_int;
    let redirectee:*mut WordDesc;
    let redir_word:*mut WordDesc;

    redirectee = (*redirect).redirectee.filename;
    redir_fd = (*redirect).redirectee.dest;

    redir_word = (*redirect).redirector.filename;
    redirector = (*redirect).redirector.dest;

    match (*redirect).instruction {
        r_instruction_r_input_direction  => {
            if (*redirect).flags & REDIR_VARASSIGN as i32 != 0{
                cprintf(b"{%s}\0" as *const u8 as *const c_char, (*redir_word).word);
            }
            else if !redirect.is_null(){
                cprintf(b"%d\0" as *const u8 as *const c_char,redirector as *mut c_char);
            }
            cprintf(b"< %s\0" as *const u8 as *const c_char,(*redirectee).word);
            
        }

        r_instruction_r_output_direction => {
            if (*redirect).rflags & REDIR_VARASSIGN as i32 != 0{
                cprintf(b"{%s}\0" as *const u8 as *const c_char,(*redir_word).word);
            }
            else if redirector != 1{
                cprintf(b"%d\0" as *const u8 as *const c_char,redirector);
            }
            cprintf (b"> %s\0" as *const u8 as *const c_char, (*redirectee).word);
        }

        r_instruction_r_inputa_direction => {
            cprintf (b"&\0" as *const u8 as *const c_char);
        }

        r_instruction_r_output_force => {
            if (*redirect).rflags & REDIR_VARASSIGN as i32 != 0{
                cprintf (b"{%s}\0" as *const u8 as *const c_char, (*redir_word).word);
            }
            else if redirector != 1{
                cprintf(b"%d\0" as *const u8 as *const c_char,redirector);
            }
            cprintf (b">| %s\0" as *const u8 as *const c_char, (*redirectee).word);
        }

        r_instruction_r_appending_to => {
            if (*redirect).rflags & REDIR_VARASSIGN as i32 != 0{
                cprintf(b"{%s}\0" as *const u8 as *const c_char,(*redir_word).word);
            }
            else if redirector != 1{
                cprintf(b"%d\0" as *const u8 as *const c_char,redirector);
            }
            cprintf (b">> %s\0" as *const u8 as *const c_char, (*redirectee).word);
        }

        r_instruction_r_input_output =>{
            if (*redirect).rflags & REDIR_VARASSIGN as i32 != 0{
                cprintf(b"{%s}\0" as *const u8 as *const c_char,(*redir_word).word);
            }
            else if redirector != 1{
                cprintf( b"%d\0" as *const u8 as *const c_char,redirector as *mut c_char);
            }
            cprintf (b"<> %s\0" as *const u8 as *const c_char, (*redirectee).word);
        }

        r_instruction_r_deblank_reading_until | r_instruction_r_reading_until =>{
            print_heredoc_header(redirect);
            cprintf(b"\n\0" as *const u8 as *const c_char);
            print_heredoc_body(redirect);
        }

        r_instruction_r_reading_string =>{
            if (*redirect).rflags & REDIR_VARASSIGN as i32 !=0 {
                cprintf(b"{%s}\0" as *const u8 as *const c_char,(*redir_word).word);
            }
            else if redirector != 0{

                cprintf(b"%d\0" as *const u8 as *const c_char,redirector as *const c_char);
            }
            cprintf(b"<<< %s\0" as *const u8 as *const c_char, (*(*redirect).redirectee.filename).word);
        }

        r_instruction_r_duplicating_input =>{
            if (*redirect).rflags & REDIR_VARASSIGN as i32 != 0{
                cprintf (b"{%s}<&%d\0" as *const u8 as *const c_char,(*redir_word).word, redir_fd);
            }
            else{
                cprintf (b"%d<&%d\0" as *const u8 as *const c_char, redirector, redir_fd);
            }
        }

        r_instruction_r_duplicating_output => {
            if (*redirect).rflags & REDIR_VARASSIGN as i32 != 0{
                cprintf (b"{%s}>&%d\0" as *const u8 as *const c_char,(*redir_word).word, redir_fd);
            }
            else{
                cprintf (b"%d>&%d\0" as *const u8 as *const c_char, redirector, redir_fd);
            }
        }

        r_instruction_r_duplicating_input_word =>{
            if (*redirect).rflags & REDIR_VARASSIGN as i32 != 0 {
                cprintf (b"{%s}<&%s\0" as *const u8 as *const c_char, (*redir_word).word, (*redirectee).word);
            }
            else{
                cprintf (b"%d<&%s\0" as *const u8 as *const c_char, redirector, (*redirectee).word);
            }
        }

        r_instruction_r_duplicating_output_word =>{
            if (*redirect).rflags & REDIR_VARASSIGN as i32 != 0{
                cprintf(b"{%s}>&%s\0" as *const u8 as *const c_char, (*redir_word).word, (*redirectee).word);
            }
            else{
                cprintf(b"%d>&%s\0" as *const u8 as *const c_char,redirector, (*redirectee).word);
            }
        }

        r_instruction_r_move_input => {
            if (*redirect).rflags & REDIR_VARASSIGN as i32 != 0{
                cprintf(b"{%s}<&%d-\0" as *const u8 as *const c_char, (*redir_word).word, redir_fd);
            }
            else{
                cprintf(b"%d<&%d-\0" as *const u8 as *const c_char,redirector, redir_fd);
            }
        }

        r_instruction_r_move_output => {
            if (*redirect).rflags & REDIR_VARASSIGN as i32 != 0{
               cprintf(b"{%s}>&%d-\0" as *const u8 as *const c_char, (*redir_word).word, redir_fd);
            }
            else{
                cprintf(b"%d>&%d-\0" as *const u8 as *const c_char,redirector, redir_fd);
            }
        }

        r_instruction_r_move_input_word => {
            if (*redirect).rflags & REDIR_VARASSIGN as i32 != 0{
                cprintf(b"{%s}<&%s-\0" as *const u8 as *const c_char, (*redir_word).word, (*redirectee).word);
            }
            else{
                cprintf(b"%d<&%s-\0" as *const u8 as *const c_char,redirector, (*redirectee).word);
            }
        }

        r_instruction_r_move_output_word =>{
            if (*redirect).rflags & REDIR_VARASSIGN as i32 != 0{
                cprintf(b"{%s}>&%s-\0" as *const u8 as *const c_char, (*redir_word).word, (*redirectee).word);
            }
            else{
                cprintf(b"%d>&%s-\0" as *const u8 as *const c_char,redirector, (*redirectee).word);
            }
        }

        r_instruction_r_close_this =>{
            if (*redirect).rflags & REDIR_VARASSIGN as i32 != 0{
                cprintf(b"{%s}>&-\0" as *const u8 as *const c_char, (*redir_word).word);
            }
            else{
                 cprintf(b"%d>&-\0" as *const u8 as *const c_char,redirector);
            }
        }

        r_instruction_r_err_and_out =>{
            cprintf(b"&> %s\0" as *const u8 as *const c_char, (*redirectee).word);
        }

        r_instruction_r_append_err_and_out  =>{
            cprintf(b"&>> %s\0" as *const u8 as *const c_char, (*redirectee).word);
        }
        _ =>{}
    }
}


#[no_mangle]
pub unsafe extern "C" fn reset_locals()
{
    inside_function_def = 0;
    indentation = 0;
    printing_connection = 0;
    deferred_heredocs = 0 as *mut REDIRECT;
}

#[no_mangle]
pub unsafe extern "C" fn print_function_def(func: *mut FUNCTION_DEF)
{
    let mut cmdcopy: *mut COMMAND;
    let mut func_redirects:*mut REDIRECT;

    func_redirects = std::ptr::null_mut();

    if posixly_correct == 0
    {
        cprintf (b"function %s () \n\0" as *const u8 as *const c_char , (*(*func).name).word);
    }
    else{
        cprintf (b"%s () \n\0" as *const u8 as *const c_char,(*(*func).name).word);
    }

    //add_unwind_protect(reset_locals, 0 );
    add_unwind_protect(reset_locals as *mut Function, 0 as *mut c_char);

    indent(indentation);
    cprintf(b"{ \n\0" as *const u8 as *const c_char);

    inside_function_def += 1;
    indentation += indentation_amount;

    cmdcopy = copy_command((*func).command);
    if (*cmdcopy).type_ == command_type_cm_group
    {
        func_redirects = (*cmdcopy).redirects;
        (*cmdcopy).redirects = 0 as *mut REDIRECT;
    }
    make_command_string_internal(if (*cmdcopy).type_ == command_type_cm_group{
                                (*(*cmdcopy).value.Group).command
                                } else {
                                    cmdcopy
                                }
                            );

    PRINT_DEFERRED_HEREDOCS!(b"\0" as *const u8 as *const c_char );

    remove_unwind_protect();
    indentation -= indentation_amount;
    inside_function_def -= 1;

    if !func_redirects.is_null()
    {
        newline(b"} \0" as *const u8 as *const c_char as *mut c_char);
        print_redirection_list(func_redirects);
        (*cmdcopy).redirects = func_redirects;
    }
    else
    {
        newline(b"}\0" as *const u8 as *const c_char as *mut c_char);
    }

    dispose_command(cmdcopy);
}



#[no_mangle]
pub unsafe extern "C" fn named_function_string(name:*mut c_char, command:*mut COMMAND, flags:c_int)-> *mut c_char
{

    let mut result:*mut c_char;
    let old_indent:c_int;
    let old_amount:c_int;
    let mut cmdcopy:*mut COMMAND;
    let mut func_redirects:*mut REDIRECT;
   
    old_indent = indentation;
    old_amount = indentation_amount;
    command_string_index = was_heredoc;
    command_string_index = 0;
    was_heredoc = 0;
    deferred_heredocs = std::ptr::null_mut();

    if !name.is_null() &&  *name as c_int != 0
    {
        if find_reserved_word(name) >= 0
        {
            cprintf(b"function \0" as *const u8 as *const c_char);
        }
        cprintf(name);
    }
    cprintf(b"() \0" as *const u8 as *const c_char);

    if (flags & FUNC_MULTILINE) == 0
    {
        indentation = 1;
        indentation_amount = 0;
    }
    else 
    {
        cprintf(b"\n\0" as *const u8 as *const c_char);
        indentation += indentation_amount;    
    }

    inside_function_def += 1;

    cprintf(if (flags & FUNC_MULTILINE) == 0 {
        b"{ \n\0" as *const u8 as *const c_char
        } else {
            b"{ \0" as *const u8 as *const c_char
        },
    );

    cmdcopy = copy_command(command);

    func_redirects = std::ptr::null_mut();
    if (*cmdcopy).type_ == command_type_cm_group
    {
        func_redirects = (*cmdcopy).redirects;
        (*cmdcopy).redirects = 0 as *mut redirect ;
    } 
    make_command_string_internal(if (*cmdcopy).type_ == command_type_cm_group {
                                    (*(*cmdcopy).value.Group).command
                                } else {
                                    cmdcopy
                                },
                                );
    PRINT_DEFERRED_HEREDOCS!(b"\0" as *const u8 as *const c_char);

    indentation = old_indent;
    indentation = old_amount;
    indentation_amount = old_amount;
    inside_function_def -= 1;

    if !func_redirects.is_null()
    {
        newline(b"} \0" as *const u8 as *const c_char as *mut c_char);
        print_redirection_list(func_redirects);
        (*cmdcopy).redirects = func_redirects;
    }
    else{
        newline(b"}\0" as *const u8 as *const c_char as *mut c_char);
    }

    result = the_printed_command;

    if (flags & FUNC_MULTILINE) == 0
    {
        if *result.offset(2) == '\n' as c_char
        {
            libc::memmove(result.offset(2) as *mut c_void, result.offset(3) as *const c_void, libc::strlen(result).wrapping_sub(2));
        }   
    }

    dispose_command(cmdcopy);

    if (flags & FUNC_EXTERNAL) != 0
    {
        result = remove_quoted_escapes(result);
    }

    return result;
}


#[no_mangle]
pub unsafe extern "C" fn newline(string: *mut c_char)
{
    cprintf(b"\n\0" as *const u8 as *const c_char);
    indent(indentation);
    if !string.is_null() && *string as c_int != 0
    {
        cprintf(string);
    }
}

static mut indentation_string:*mut c_char = std::ptr::null_mut();
static mut indentation_size:c_int = 0;

#[no_mangle]
unsafe extern "C" fn indent(mut amount:c_int)
{
    let mut i:c_int;
    RESIZE_MALLOCED_BUFFER!(indentation_string, 0 as c_int, amount, indentation_size , 16);

    i = 0;
    while amount > 0
    {
        *indentation_string.offset(i as isize)  = ' ' as c_char;
        i += 1;
        amount -= 1;
    }
    *indentation_string.offset(i as isize) = '\0' as c_char;
    cprintf(indentation_string);
}


#[no_mangle]
unsafe extern "C" fn semicolon()
{
    if command_string_index > 0 &&
        (*the_printed_command.offset((command_string_index -1) as isize) == '&' as c_char ||
         *the_printed_command.offset((command_string_index -1) as isize) == '\n' as c_char)
    {
        return;
    }

    cprintf(b";\0" as *const u8 as *const c_char);
}


#[no_mangle]
unsafe extern "C" fn the_printed_command_resize(length:c_int)
{
    if the_printed_command.is_null()
    {
        the_printed_command_size = (length + PRINTED_COMMAND_INITIAL_SIZE - 1) & !PRINTED_COMMAND_INITIAL_SIZE - 1;
        the_printed_command = libc::malloc(the_printed_command_size as usize) as *mut c_char;
        command_string_index = 0;
    }
    else if (command_string_index + length) >= the_printed_command_size {
        let mut new:c_int;
        new = command_string_index + length + 1;
        new = (new + PRINTED_COMMAND_CROW_SIZE -1 ) & !(PRINTED_COMMAND_CROW_SIZE -1);
        the_printed_command_size = new;

        the_printed_command = libc::realloc(the_printed_command as *mut c_void, the_printed_command_size as usize) as *mut c_char;
    }
}
/*

//xprintf 没有重构，用到xprint的地方直接换成println！


//cprintf测试
unsafe fn cprintf_1(str:*const c_char)
{
    let mut _c:c_int;
    let arg_len:i32;

    arg_len = libc::strlen(str) as i32;
    the_printed_command_resize(arg_len + 1);
    libc::strncpy(the_printed_command.offset(command_string_index as isize), str, arg_len as usize);
    command_string_index += arg_len;
    *the_printed_command.offset(command_string_index as isize) = '\u{0}' as i32 as libc::c_char;
}
unsafe fn cprintf_2(str1:*const c_char,str2:*mut c_char)
{
    let mut _c:c_int;
    let arg_len_1:i32;
    let arg_len_2:i32;
    arg_len_1 = libc::strlen(str1) as i32;
    arg_len_2 = libc::strlen(str2 as *const c_char) as i32;
    the_printed_command_resize(arg_len_1 + arg_len_2 + 1);
    libc::strncpy(the_printed_command.offset(command_string_index as isize), str1, arg_len_1 as usize);
    command_string_index += arg_len_1;
    libc::strncpy(the_printed_command.offset(command_string_index as isize), str2, arg_len_1 as usize);
    command_string_index += arg_len_2;

    *the_printed_command.offset(command_string_index as isize) = '\u{0}' as i32 as libc::c_char;
}
*/

/* 
unsafe extern "C" fn cprintf(mut control: *const libc::c_char, mut args: ...) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut char_arg: [libc::c_char; 2] = [0; 2];
    let mut argp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut intbuf: [libc::c_char; 11] = [0; 11];
    let mut digit_arg: libc::c_int = 0;
    let mut arg_len: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    arg_len = strlen(control) as libc::c_int;
    the_printed_command_resize(arg_len + 1 as libc::c_int);
    char_arg[1 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    s = control;
    while !s.is_null() && *s as libc::c_int != 0 {
        let fresh7 = s;
        s = s.offset(1);
        c = *fresh7 as libc::c_int;
        argp = 0 as *mut libc::c_void as *mut libc::c_char;
        if c != '%' as i32 || *s == 0 {
            char_arg[0 as libc::c_int as usize] = c as libc::c_char;
            argp = char_arg.as_mut_ptr();
            arg_len = 1 as libc::c_int;
        } else {
            let fresh8 = s;
            s = s.offset(1);
            c = *fresh8 as libc::c_int;
            match c {
                37 => {
                    char_arg[0 as libc::c_int as usize] = c as libc::c_char;
                    argp = char_arg.as_mut_ptr();
                    arg_len = 1 as libc::c_int;
                }
                115 => {
                    argp = args_0.arg::<*mut libc::c_char>();
                    arg_len = strlen(argp) as libc::c_int;
                }
                100 => {
                    digit_arg = args_0.arg::<libc::c_int>();
                    if digit_arg < 0 as libc::c_int {
                        sprintf(
                            intbuf.as_mut_ptr(),
                            b"%u\0" as *const u8 as *const libc::c_char,
                            -(1 as libc::c_int) as libc::c_uint,
                        );
                        argp = intbuf.as_mut_ptr();
                    } else {
                        argp = inttostr(
                            digit_arg as intmax_t,
                            intbuf.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong,
                        );
                    }
                    arg_len = strlen(argp) as libc::c_int;
                }
                99 => {
                    char_arg[0 as libc::c_int
                        as usize] = args_0.arg::<libc::c_int>() as libc::c_char;
                    argp = char_arg.as_mut_ptr();
                    arg_len = 1 as libc::c_int;
                }
                _ => {
                    programming_error(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"cprintf: `%c': invalid format character\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        c,
                    );
                }
            }
        }
        if !argp.is_null() && arg_len != 0 {
            the_printed_command_resize(arg_len + 1 as libc::c_int);
            libc::memcpy(
                the_printed_command.offset(command_string_index as isize)
                    as *mut libc::c_void,
                argp as *const libc::c_void,
                arg_len as libc::c_ulong as libc::size_t,
            );
            command_string_index += arg_len;
        }
    }
    *the_printed_command
        .offset(command_string_index as isize) = '\u{0}' as i32 as libc::c_char;
}
*/
