//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later


// #![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

extern "C" {
    fn list_reverse(list: *mut rcommon::GENERIC_LIST) -> *mut rcommon::GENERIC_LIST;
    fn make_bare_word(_: *const libc::c_char) -> *mut WordDesc;
    fn make_word_list(_: *mut WordDesc, _: *mut WordList) -> *mut WordList;
}
use libc::malloc;
use rcommon::{WordList, WordDesc};
pub type size_t = libc::c_ulong;
pub type r_instruction = libc::c_uint;
pub const r_append_err_and_out: r_instruction = 19;
pub const r_move_output_word: r_instruction = 18;
pub const r_move_input_word: r_instruction = 17;
pub const r_move_output: r_instruction = 16;
pub const r_move_input: r_instruction = 15;
pub const r_duplicating_output_word: r_instruction = 14;
pub const r_duplicating_input_word: r_instruction = 13;
pub const r_output_force: r_instruction = 12;
pub const r_input_output: r_instruction = 11;
pub const r_err_and_out: r_instruction = 10;
pub const r_close_this: r_instruction = 9;
pub const r_deblank_reading_until: r_instruction = 8;
pub const r_duplicating_output: r_instruction = 7;
pub const r_duplicating_input: r_instruction = 6;
pub const r_reading_string: r_instruction = 5;
pub const r_reading_until: r_instruction = 4;
pub const r_appending_to: r_instruction = 3;
pub const r_inputa_direction: r_instruction = 2;
pub const r_input_direction: r_instruction = 1;
pub const r_output_direction: r_instruction = 0;

pub type command_type = libc::c_uint;
pub const cm_coproc: command_type = 14;
pub const cm_subshell: command_type = 13;
pub const cm_arith_for: command_type = 12;
pub const cm_cond: command_type = 11;
pub const cm_arith: command_type = 10;
pub const cm_group: command_type = 9;
pub const cm_until: command_type = 8;
pub const cm_function_def: command_type = 7;
pub const cm_connection: command_type = 6;
pub const cm_select: command_type = 5;
pub const cm_simple: command_type = 4;
pub const cm_if: command_type = 3;
pub const cm_while: command_type = 2;
pub const cm_case: command_type = 1;
pub const cm_for: command_type = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub union REDIRECTEE {
    pub dest: libc::c_int,
    pub filename: *mut WordDesc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redirect {
    pub next: *mut redirect,
    pub redirector: REDIRECTEE,
    pub rflags: libc::c_int,
    pub flags: libc::c_int,
    pub instruction: r_instruction,
    pub redirectee: REDIRECTEE,
    pub here_doc_eof: *mut libc::c_char,
}
pub type REDIRECT = redirect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct command {
    pub type_0: command_type,
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub redirects: *mut REDIRECT,
    pub value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub For: *mut for_com,
    pub Case: *mut case_com,
    pub While: *mut while_com,
    pub If: *mut if_com,
    pub Connection: *mut connection,
    pub Simple: *mut simple_com,
    pub Function_def: *mut function_def,
    pub Group: *mut group_com,
    pub Select: *mut select_com,
    pub Arith: *mut arith_com,
    pub Cond: *mut cond_com,
    pub ArithFor: *mut arith_for_com,
    pub Subshell: *mut subshell_com,
    pub Coproc: *mut coproc_com,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct coproc_com {
    pub flags: libc::c_int,
    pub name: *mut libc::c_char,
    pub command: *mut COMMAND,
}
pub type COMMAND = command;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subshell_com {
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub command: *mut COMMAND,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arith_for_com {
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub init: *mut WordList,
    pub test: *mut WordList,
    pub step: *mut WordList,
    pub action: *mut COMMAND,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cond_com {
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub type_0: libc::c_int,
    pub op: *mut WordDesc,
    pub left: *mut cond_com,
    pub right: *mut cond_com,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arith_com {
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub exp: *mut WordList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct select_com {
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub name: *mut WordDesc,
    pub map_list: *mut WordList,
    pub action: *mut COMMAND,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group_com {
    pub ignore: libc::c_int,
    pub command: *mut COMMAND,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct function_def {
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub name: *mut WordDesc,
    pub command: *mut COMMAND,
    pub source_file: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct simple_com {
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub words: *mut WordList,
    pub redirects: *mut REDIRECT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct connection {
    pub ignore: libc::c_int,
    pub first: *mut COMMAND,
    pub second: *mut COMMAND,
    pub connector: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct if_com {
    pub flags: libc::c_int,
    pub test: *mut COMMAND,
    pub true_case: *mut COMMAND,
    pub false_case: *mut COMMAND,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct while_com {
    pub flags: libc::c_int,
    pub test: *mut COMMAND,
    pub action: *mut COMMAND,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct case_com {
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub word: *mut WordDesc,
    pub clauses: *mut PATTERN_LIST,
}
pub type PATTERN_LIST = pattern_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pattern_list {
    pub next: *mut pattern_list,
    pub patterns: *mut WordList,
    pub action: *mut COMMAND,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct for_com {
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub name: *mut WordDesc,
    pub map_list: *mut WordList,
    pub action: *mut COMMAND,
}
pub type CONNECTION = connection;
pub type CASE_COM = case_com;
pub type FOR_COM = for_com;
pub type ARITH_FOR_COM = arith_for_com;
pub type SELECT_COM = select_com;
pub type IF_COM = if_com;
pub type WHILE_COM = while_com;
pub type ARITH_COM = arith_com;
pub type COND_COM = cond_com;
pub type SIMPLE_COM = simple_com;
pub type FUNCTION_DEF = function_def;
pub type GROUP_COM = group_com;
pub type SUBSHELL_COM = subshell_com;
pub type COPROC_COM = coproc_com;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct g_list {
    pub next: *mut g_list,
}

#[no_mangle]
pub unsafe extern "C" fn copy_word(w: *mut WordDesc) -> *mut WordDesc {
    let new_word: *mut WordDesc = make_bare_word((*w).word);
    (*new_word).flags = (*w).flags;
    return new_word;
}

#[no_mangle]
pub unsafe extern "C" fn copy_word_list(mut list: *mut WordList) -> *mut WordList {
    let mut new_list: *mut WordList;
    let mut tl: *mut WordList = 0 as *mut WordList;
    new_list = tl;
    while !list.is_null() {
        if new_list.is_null() {
            tl = make_word_list(copy_word((*list).word), new_list);
            new_list = tl;
        } else {
            (*tl)
                .next = make_word_list(
                copy_word((*list).word),
                0 as *mut libc::c_void as *mut WordList,
            );
            tl = (*tl).next;
        }
        list = (*list).next;
    }
    return new_list;
}

unsafe extern "C" fn copy_case_clause(
    clause: *mut PATTERN_LIST,
) -> *mut PATTERN_LIST {
    let new_clause: *mut PATTERN_LIST = malloc(
        ::core::mem::size_of::<PATTERN_LIST>() as usize,
    ) as *mut PATTERN_LIST;
    (*new_clause).patterns = copy_word_list((*clause).patterns);
    (*new_clause).action = copy_command((*clause).action);
    (*new_clause).flags = (*clause).flags;
    return new_clause;
}

unsafe extern "C" fn copy_case_clauses(
    mut clauses: *mut PATTERN_LIST,
) -> *mut PATTERN_LIST {
    let mut new_list: *mut PATTERN_LIST = 0 as *mut PATTERN_LIST;
    let mut new_clause: *mut PATTERN_LIST;

    while !clauses.is_null() {
        new_clause = copy_case_clause(clauses);
        (*new_clause).next = new_list;
        new_list = new_clause;
        clauses = (*clauses).next;
    }
    return if !new_list.is_null() && !((*new_list).next).is_null() {
        list_reverse(new_list as *mut rcommon::GENERIC_LIST) as *mut PATTERN_LIST
    } else {
        new_list
    };
}

#[no_mangle]
pub unsafe extern "C" fn copy_redirect(redirect: *mut REDIRECT) -> *mut REDIRECT {
    let new_redirect: *mut REDIRECT = malloc(
        ::core::mem::size_of::<REDIRECT>() as usize,
    ) as *mut REDIRECT;
    *new_redirect = *redirect;
    if (*redirect).rflags & 0x1 as libc::c_int != 0 {
        (*new_redirect).redirector.filename = copy_word((*redirect).redirector.filename);
    }

    match (*redirect).instruction as libc::c_uint {
        r_reading_until | r_deblank_reading_until => {
            (*new_redirect)
                .here_doc_eof = if !((*redirect).here_doc_eof).is_null() {
                r_shell::savestring!((*redirect).here_doc_eof) as *mut libc::c_char
            } else {
                0 as *mut libc::c_char
            };
            (*new_redirect)
            .redirectee
            .filename = copy_word((*redirect).redirectee.filename);

        }
        r_reading_string | r_appending_to | r_output_direction | r_input_direction | r_inputa_direction | r_err_and_out | r_append_err_and_out | r_input_output | r_output_force | r_duplicating_input_word | r_duplicating_output_word | r_move_input_word | r_move_output_word => {
            (*new_redirect)
            .redirectee
            .filename = copy_word((*redirect).redirectee.filename);
        }
        r_duplicating_input | r_duplicating_output | r_move_input | r_move_output | r_close_this | _ => {
        }
    }
    return new_redirect;
}

#[no_mangle]
pub unsafe extern "C" fn copy_redirects(mut list: *mut REDIRECT) -> *mut REDIRECT {
    let mut new_list: *mut REDIRECT;
    let mut temp: *mut REDIRECT;
    new_list = 0 as *mut libc::c_void as *mut REDIRECT;
    while !list.is_null() {
        temp = copy_redirect(list);
        (*temp).next = new_list;
        new_list = temp;
        list = (*list).next;
    }
    return if !new_list.is_null() && !((*new_list).next).is_null() {
        list_reverse(new_list as *mut rcommon::GENERIC_LIST) as *mut REDIRECT
    } else {
        new_list
    };
}

unsafe extern "C" fn copy_for_command(com: *mut FOR_COM) -> *mut FOR_COM {
    let new_for: *mut FOR_COM = malloc(
        ::core::mem::size_of::<FOR_COM>() as usize
    ) as *mut FOR_COM;
    (*new_for).flags = (*com).flags;
    (*new_for).line = (*com).line;
    (*new_for).name = copy_word((*com).name);
    (*new_for).map_list = copy_word_list((*com).map_list);
    (*new_for).action = copy_command((*com).action);
    return new_for;
}

unsafe extern "C" fn copy_arith_for_command(
    com: *mut ARITH_FOR_COM,
) -> *mut ARITH_FOR_COM {
    let new_arith_for: *mut ARITH_FOR_COM = malloc(
        ::core::mem::size_of::<ARITH_FOR_COM>() as usize
    ) as *mut ARITH_FOR_COM;
    (*new_arith_for).flags = (*com).flags;
    (*new_arith_for).line = (*com).line;
    (*new_arith_for).init = copy_word_list((*com).init);
    (*new_arith_for).test = copy_word_list((*com).test);
    (*new_arith_for).step = copy_word_list((*com).step);
    (*new_arith_for).action = copy_command((*com).action);
    return new_arith_for;
}

unsafe extern "C" fn copy_group_command(com: *mut GROUP_COM) -> *mut GROUP_COM {
    let new_group: *mut GROUP_COM = malloc(
        ::core::mem::size_of::<GROUP_COM>() as usize
    ) as *mut GROUP_COM;
    (*new_group).command = copy_command((*com).command);
    return new_group;
}

unsafe extern "C" fn copy_subshell_command(
    com: *mut SUBSHELL_COM,
) -> *mut SUBSHELL_COM {
    let new_subshell: *mut SUBSHELL_COM = malloc(
        ::core::mem::size_of::<SUBSHELL_COM>() as usize
    ) as *mut SUBSHELL_COM;
    (*new_subshell).command = copy_command((*com).command);
    (*new_subshell).flags = (*com).flags;
    (*new_subshell).line = (*com).line;
    return new_subshell;
}
unsafe extern "C" fn copy_coproc_command(com: *mut COPROC_COM) -> *mut COPROC_COM {
    let new_coproc: *mut COPROC_COM = malloc(
        ::core::mem::size_of::<COPROC_COM>() as usize
    ) as *mut COPROC_COM;
    (*new_coproc)
        .name = r_shell::savestring!(
        (*com).name
    );
    (*new_coproc).command = copy_command((*com).command);
    (*new_coproc).flags = (*com).flags;
    return new_coproc;
}

unsafe extern "C" fn copy_case_command(com: *mut CASE_COM) -> *mut CASE_COM {
    let new_case: *mut CASE_COM = malloc(
        ::core::mem::size_of::<CASE_COM>() as usize
    ) as *mut CASE_COM;
    (*new_case).flags = (*com).flags;
    (*new_case).line = (*com).line;
    (*new_case).word = copy_word((*com).word);
    (*new_case).clauses = copy_case_clauses((*com).clauses);
    return new_case;
}

unsafe extern "C" fn copy_while_command(com: *mut WHILE_COM) -> *mut WHILE_COM {
    let new_while: *mut WHILE_COM = malloc(
        ::core::mem::size_of::<WHILE_COM>()  as usize
    ) as *mut WHILE_COM;
    (*new_while).flags = (*com).flags;
    (*new_while).test = copy_command((*com).test);
    (*new_while).action = copy_command((*com).action);
    return new_while;
}

unsafe extern "C" fn copy_if_command(com: *mut IF_COM) -> *mut IF_COM {
    let new_if: *mut IF_COM = malloc(
        ::core::mem::size_of::<IF_COM>()  as usize
    ) as *mut IF_COM;
    (*new_if).flags = (*com).flags;
    (*new_if).test = copy_command((*com).test);
    (*new_if).true_case = copy_command((*com).true_case);
    (*new_if)
        .false_case = if !((*com).false_case).is_null() {
        copy_command((*com).false_case)
    } else {
        (*com).false_case
    };
    return new_if;
}

unsafe extern "C" fn copy_arith_command(com: *mut ARITH_COM) -> *mut ARITH_COM {
    let new_arith: *mut ARITH_COM = libc::malloc(
        ::core::mem::size_of::<ARITH_COM>() as libc::c_ulong as usize

    ) as *mut ARITH_COM;
    (*new_arith).flags = (*com).flags;
    (*new_arith).exp = copy_word_list((*com).exp);
    (*new_arith).line = (*com).line;
    return new_arith;
}
unsafe extern "C" fn copy_cond_command(com: *mut COND_COM) -> *mut COND_COM {
    let new_cond: *mut COND_COM = malloc(
        ::core::mem::size_of::<COND_COM>() as usize
    ) as *mut COND_COM;
    (*new_cond).flags = (*com).flags;
    (*new_cond).line = (*com).line;
    (*new_cond).type_0 = (*com).type_0;
    (*new_cond)
        .op = if !((*com).op).is_null() { copy_word((*com).op) } else { (*com).op };
    (*new_cond)
        .left = if !((*com).left).is_null() {
        copy_cond_command((*com).left)
    } else {
        0 as *mut libc::c_void as *mut COND_COM
    };
    (*new_cond)
        .right = if !((*com).right).is_null() {
        copy_cond_command((*com).right)
    } else {
        0 as *mut libc::c_void as *mut COND_COM
    };
    return new_cond;
}

unsafe extern "C" fn copy_simple_command(com: *mut SIMPLE_COM) -> *mut SIMPLE_COM {
    let new_simple: *mut SIMPLE_COM = malloc(
        ::core::mem::size_of::<SIMPLE_COM>() as usize
    ) as *mut SIMPLE_COM;
    (*new_simple).flags = (*com).flags;
    (*new_simple).words = copy_word_list((*com).words);
    (*new_simple)
        .redirects = if !((*com).redirects).is_null() {
        copy_redirects((*com).redirects)
    } else {
        0 as *mut libc::c_void as *mut REDIRECT
    };
    (*new_simple).line = (*com).line;
    return new_simple;
}

#[no_mangle]
pub unsafe extern "C" fn copy_function_def_contents(
    old: *mut FUNCTION_DEF,
    new_def: *mut FUNCTION_DEF,
) -> *mut FUNCTION_DEF {
    (*new_def).name = copy_word((*old).name);
    (*new_def)
        .command = if !((*old).command).is_null() {
        copy_command((*old).command)
    } else {
        (*old).command
    };
    (*new_def).flags = (*old).flags;
    (*new_def).line = (*old).line;
    (*new_def)
        .source_file = if !((*old).source_file).is_null() {
        r_shell::savestring!((*old).source_file)
    } else {
        (*old).source_file
    };
    return new_def;
}

#[no_mangle]
pub unsafe extern "C" fn copy_function_def(
    com: *mut FUNCTION_DEF,
) -> *mut FUNCTION_DEF {
    let mut new_def: *mut FUNCTION_DEF = malloc(
        ::core::mem::size_of::<FUNCTION_DEF>() as usize
    ) as *mut FUNCTION_DEF;
    new_def = copy_function_def_contents(com, new_def);
    return new_def;
}

#[no_mangle]
pub unsafe extern "C" fn copy_command(command: *mut COMMAND) -> *mut COMMAND {
    let new_command: *mut COMMAND;
    if command.is_null() {
        return command;
    }
    new_command = malloc(
        ::core::mem::size_of::<COMMAND>() as usize
    ) as *mut COMMAND;
    libc::memcpy(
        new_command as *mut libc::c_char as *mut libc::c_void,
        command as *mut libc::c_char as *const libc::c_void,
        ::core::mem::size_of::<COMMAND>() as libc::c_ulong as libc::size_t,
    );
    (*new_command).flags = (*command).flags;
    (*new_command).line = (*command).line;
    if !((*command).redirects).is_null() {
        (*new_command).redirects = copy_redirects((*command).redirects);
    }
    match (*command).type_0 as libc::c_uint {
        cm_for => {
            (*new_command).value.For = copy_for_command((*command).value.For);
        }
        cm_arith_for => {
            (*new_command)
                .value
                .ArithFor = copy_arith_for_command((*command).value.ArithFor);
        }
        cm_select => {
            (*new_command)
                .value
                .Select = copy_for_command((*command).value.Select as *mut FOR_COM)
                as *mut SELECT_COM;
        }
        cm_group => {
            (*new_command).value.Group = copy_group_command((*command).value.Group);
        }
        cm_subshell => {
            (*new_command)
                .value
                .Subshell = copy_subshell_command((*command).value.Subshell);
        }
        cm_coproc => {
            (*new_command).value.Coproc = copy_coproc_command((*command).value.Coproc);
        }
        cm_case => {
            (*new_command).value.Case = copy_case_command((*command).value.Case);
        }
        cm_until | cm_while => {
            (*new_command).value.While = copy_while_command((*command).value.While);
        }
        cm_if => {
            (*new_command).value.If = copy_if_command((*command).value.If);
        }
        cm_arith => {
            (*new_command).value.Arith = copy_arith_command((*command).value.Arith);
        }
        cm_cond => {
            (*new_command).value.Cond = copy_cond_command((*command).value.Cond);
        }
        cm_simple => {
            (*new_command).value.Simple = copy_simple_command((*command).value.Simple);
        }
        cm_connection => {
            let new_connection: *mut CONNECTION = malloc(
                ::core::mem::size_of::<CONNECTION>() as usize
            ) as *mut CONNECTION;
            (*new_connection).connector = (*(*command).value.Connection).connector;
            (*new_connection).first = copy_command((*(*command).value.Connection).first);
            (*new_connection)
                .second = copy_command((*(*command).value.Connection).second);
            (*new_command).value.Connection = new_connection;
        }
        cm_function_def => {
            (*new_command)
                .value
                .Function_def = copy_function_def((*command).value.Function_def);
        }
        _ => {}
    }
    return new_command;
}
