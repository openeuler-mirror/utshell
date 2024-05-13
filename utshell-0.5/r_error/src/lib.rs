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