/*
 * SPDX-FileCopyrightText: 2025 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: GPL-2.0-or-later
 */
use crate::builtins::common::{
    builtin_usage, get_numeric_arg, number_of_args, sh_erange, shift_args,
};
use crate::builtins::help::builtin_help;
use crate::src_common::*;
use crate::subst::invalidate_cached_quoted_dollar_at;
use crate::variables::clear_dollar_vars;

pub static print_shift_error: libc::c_int = 0;

#[no_mangle]
pub fn shift_builtin(list: *mut WordList) -> i32 {
    unsafe {
        if !list.is_null()
            && !(*list).word.is_null()
            && libc::strcmp(
                (*((*list).word)).word,
                "--help\0".as_ptr() as *const libc::c_char,
            ) == 0
        {
            builtin_help();
            return EX_USAGE;
        }

        let times: intmax_t = 0;
        if get_numeric_arg(list, 0, std::mem::transmute(&times)) == 0 {
            return EXECUTION_FAILURE;
        }

        if times == 0 {
            return EXECUTION_SUCCESS;
        } else if times < 0 {
            let s = if list.is_null() {
                PT_NULL as *mut libc::c_char
            } else {
                (*(*list).word).word
            };
            sh_erange(s, "shift count\0".as_ptr() as *mut libc::c_char);
            return EXECUTION_FAILURE;
        }

        let nargs = number_of_args();
        if times > nargs.into() {
            if print_shift_error != 0 {
                let s = if list.is_null() {
                    PT_NULL as *mut libc::c_char
                } else {
                    (*(*list).word).word
                };
                sh_erange(s, "shift count\0".as_ptr() as *mut libc::c_char);
            }
            return EXECUTION_FAILURE;
        } else if times == nargs.into() {
            clear_dollar_vars();
        } else {
            shift_args(times as i32);
        }

        invalidate_cached_quoted_dollar_at();
    }
    return EXECUTION_SUCCESS;
}
