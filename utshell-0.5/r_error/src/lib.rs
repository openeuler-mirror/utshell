//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use r_bash::*;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    static bash_badsub_errmsg: *const libc::c_char;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub static mut the_current_maintainer: *const libc::c_char =
    b"bash-maintainers@gnu.org\0" as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut r_gnu_error_format: libc::c_int = 0 as libc::c_int;
#[no_mangle]
unsafe extern "C" fn error_prolog(mut print_lineno: libc::c_int) {
    let mut ename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: libc::c_int = 0;
    ename = get_name_for_error();
    line = if print_lineno != 0 && interactive_shell == 0 as libc::c_int {
        executing_line_number()
    } else {
        -(1 as libc::c_int)
    };
    if line > 0 as libc::c_int {
        fprintf(
            stderr,
            b"%s:%s%d: \0" as *const u8 as *const libc::c_char,
            ename,
            if r_gnu_error_format != 0 {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                dcgettext(
                    0 as *const libc::c_char,
                    b" line \0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            },
            line,
        );
    } else {
        fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, ename);
    };
}

#[no_mangle]
pub unsafe extern "C" fn get_name_for_error() -> *mut libc::c_char {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bash_source_v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut bash_source_a: *mut ARRAY = 0 as *mut ARRAY;
    name = 0 as *mut libc::c_void as *mut libc::c_char;
    if interactive_shell == 0 as libc::c_int {
        bash_source_v = find_variable(b"BASH_SOURCE\0" as *const u8 as *const libc::c_char);
        if !bash_source_v.is_null() && (*bash_source_v).attributes & 0x4 as libc::c_int != 0 && {
            bash_source_a = (*bash_source_v).value as *mut ARRAY;
            !bash_source_a.is_null()
        } {
            name = array_reference(bash_source_a, 0 as libc::c_int as arrayind_t);
        }
        if name.is_null() || *name as libc::c_int == '\u{0}' as i32 {
            name = *dollar_vars.as_mut_ptr().offset(0 as libc::c_int as isize);
        }
    }
    if name.is_null() && !shell_name.is_null() && *shell_name as libc::c_int != 0 {
        name = base_pathname(shell_name);
    }
    if name.is_null() {
        name = b"bash\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    return name;
}

#[no_mangle]
pub unsafe extern "C" fn file_error(mut filename: *const libc::c_char) {
    report_error(
        b"%s: %s\0" as *const u8 as *const libc::c_char,
        filename,
        strerror(*__errno_location()),
    );
}

#[no_mangle]
pub unsafe extern "C" fn programming_error(
    mut format: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::std::ffi::VaListImpl;
    let mut h: *mut libc::c_char = 0 as *mut libc::c_char;
    give_terminal_to(shell_pgrp, 0 as libc::c_int);
    args_0 = args.clone();
    vfprintf(stderr, format, args_0.as_va_list());
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    if remember_on_history != 0 {
        h = last_history_line();
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"last command: %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            if !h.is_null() {
                h
            } else {
                b"(null)\0" as *const u8 as *const libc::c_char
            },
        );
    }
    fprintf(
        stderr,
        dcgettext(
            0 as *const libc::c_char,
            b"Aborting...\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fflush(stderr);
    abort();
}

#[no_mangle]
pub unsafe extern "C" fn report_error(mut format: *const libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    error_prolog(1 as libc::c_int);
    args_0 = args.clone();
    vfprintf(stderr, format, args_0.as_va_list());
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    if exit_immediately_on_error != 0 {
        if last_command_exit_value == 0 as libc::c_int {
            ::std::ptr::write_volatile(
                &mut last_command_exit_value as *mut libc::c_int,
                1 as libc::c_int,
            );
        }
        exit_shell(last_command_exit_value);
    }
}
#[no_mangle]
pub unsafe extern "C" fn fatal_error(mut format: *const libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    error_prolog(0 as libc::c_int);
    args_0 = args.clone();
    vfprintf(stderr, format, args_0.as_va_list());
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    sh_exit(2 as libc::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn internal_error(mut format: *const libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    error_prolog(1 as libc::c_int);
    args_0 = args.clone();
    vfprintf(stderr, format, args_0.as_va_list());
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn internal_warning(
    mut format: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::std::ffi::VaListImpl;
    error_prolog(1 as libc::c_int);
    fprintf(
        stderr,
        dcgettext(
            0 as *const libc::c_char,
            b"warning: \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    args_0 = args.clone();
    vfprintf(stderr, format, args_0.as_va_list());
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
}

#[no_mangle]
pub unsafe extern "C" fn internal_inform(
    mut format: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::std::ffi::VaListImpl;
    error_prolog(1 as libc::c_int);
    fprintf(
        stderr,
        dcgettext(
            0 as *const libc::c_char,
            b"INFORM: \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    args_0 = args.clone();
    vfprintf(stderr, format, args_0.as_va_list());
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn sys_error(mut format: *const libc::c_char, mut args: ...) {
    let mut e: libc::c_int = 0;
    let mut args_0: ::std::ffi::VaListImpl;
    e = *__errno_location();
    error_prolog(0 as libc::c_int);
    args_0 = args.clone();
    vfprintf(stderr, format, args_0.as_va_list());
    fprintf(stderr, b": %s\n\0" as *const u8 as *const libc::c_char, strerror(e));
}

#[no_mangle]
pub unsafe extern "C" fn parser_error(
    mut lineno: libc::c_int,
    mut format: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::std::ffi::VaListImpl;
    let mut ename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut iname: *mut libc::c_char = 0 as *mut libc::c_char;
    ename = get_name_for_error();
    iname = yy_input_name();
    if interactive != 0 {
        fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, ename);
    } else if interactive_shell != 0 {
        fprintf(
            stderr,
            b"%s: %s:%s%d: \0" as *const u8 as *const libc::c_char,
            ename,
            iname,
            if gnu_error_format != 0 {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                dcgettext(
                    0 as *const libc::c_char,
                    b" line \0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            },
            lineno,
        );
    } else if *ename.offset(0 as libc::c_int as isize) as libc::c_int
            == *iname.offset(0 as libc::c_int as isize) as libc::c_int
            && strcmp(ename, iname) == 0 as libc::c_int
        {
        fprintf(
            stderr,
            b"%s:%s%d: \0" as *const u8 as *const libc::c_char,
            ename,
            if gnu_error_format != 0 {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                dcgettext(
                    0 as *const libc::c_char,
                    b" line \0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            },
            lineno,
        );
    } else {
        fprintf(
            stderr,
            b"%s: %s:%s%d: \0" as *const u8 as *const libc::c_char,
            ename,
            iname,
            if gnu_error_format != 0 {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                dcgettext(
                    0 as *const libc::c_char,
                    b" line \0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            },
            lineno,
        );
    }
    args_0 = args.clone();
    vfprintf(stderr, format, args_0.as_va_list());
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    if exit_immediately_on_error != 0 {
        ::std::ptr::write_volatile(
            &mut last_command_exit_value as *mut libc::c_int,
            2 as libc::c_int,
        );
        exit_shell(
            ::std::ptr::read_volatile::<
                libc::c_int,
            >(&last_command_exit_value as *const libc::c_int),
        );
    }
}

static mut cmd_error_table: [*const libc::c_char; 5] = [
    b"unknown command error\0" as *const u8 as *const libc::c_char,
    b"bad command type\0" as *const u8 as *const libc::c_char,
    b"bad connector\0" as *const u8 as *const libc::c_char,
    b"bad jump\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn command_error(
    mut func: *const libc::c_char,
    mut code: libc::c_int,
    mut e: libc::c_int,
    mut flags: libc::c_int,
) {
    if code > CMDERR_LAST.try_into().unwrap() {
        code = CMDERR_DEFAULT as i32;
    }
    programming_error(
        b"%s: %s: %d\0" as *const u8 as *const libc::c_char,
        func,
        dcgettext(
            0 as *const libc::c_char,
            cmd_error_table[code as usize],
            5 as libc::c_int,
        ),
        e,
    );
}