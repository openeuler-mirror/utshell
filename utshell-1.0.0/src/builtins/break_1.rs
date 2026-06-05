//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

use super::common::{get_numeric_arg, sh_erange};
use super::help::builtin_help;
use crate::src_common::*;

// fn checkhelp(l: *mut WordList) -> i32 {
//     let tmp = CString::new("--help").unwrap();
//     if l != std::ptr::null_mut()
//         && unsafe { (*l).word != std::ptr::null_mut() }
//         && unsafe { libc::strcmp((*((*l).word)).word, tmp.as_ptr()) == 0 }
//     {
//         builtin_help();
//     }
//     return EX_USAGE!();
// }

/* Set up to break x levels, where x defaults to 1, but can be specified
as the first argument. */
#[no_mangle]
pub fn break_builtin(list: *mut WordList) -> i32 {
    let mut newbreak: intmax_t = 1 as intmax_t;
    unsafe {
        CHECK_HELPOPT!(list);
        if check_loop_level() == 0 {
            return EXECUTION_SUCCESS!();
        }
        get_numeric_arg(list, 1, &mut newbreak as *mut intmax_t);

        if newbreak <= 0 {
            let tmp = CString::new("loop count ").unwrap();
            sh_erange((*(*list).word).word, tmp.as_ptr() as *mut libc::c_char);
            breaking = loop_level;
            return EXECUTION_FAILURE!();
        }

        if newbreak > loop_level as libc::c_long {
            newbreak = loop_level as i64;
        }
        breaking = newbreak as i32;
    }
    return EXECUTION_SUCCESS!();
}

/* Set up to continue x levels, where x defaults to 1, but can be specified
as the first argument. */
#[no_mangle]
pub fn continue_builtin(list: *mut WordList) -> i32 {
    let mut newcont: intmax_t = 0 as intmax_t;
    unsafe {
        CHECK_HELPOPT!(list);
    }
    if check_loop_level() == 0 {
        return EXECUTION_SUCCESS!();
    }

    get_numeric_arg(list, 1, &mut newcont as *mut intmax_t);

    unsafe {
        if newcont <= 0 {
            let tmp = CString::new("loop count ").unwrap();
            sh_erange((*(*list).word).word, tmp.as_ptr() as *mut libc::c_char);
            breaking = loop_level;
            return EXECUTION_FAILURE!();
        }
        if newcont > loop_level.into() {
            newcont = loop_level as i64;
        }
        continuing = newcont as i32;
    }
    return EXECUTION_SUCCESS!();
}

/* Return non-zero if a break or continue command would be okay.
Print an error message if break or continue is meaningless here. */
#[no_mangle]
pub fn check_loop_level() -> i32 {
    unsafe {
        if loop_level == 0 && posixly_correct == 0 {
            builtin_error(
                b"only meaningful in a `for`, `while`, or until `loop` \0" as *const u8
                    as *const libc::c_char,
            );
            return 0;
        }
        loop_level
    }
}
