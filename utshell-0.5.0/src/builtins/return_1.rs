/*
 * SPDX-FileCopyrightText: 2025 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: GPL-2.0-or-later
 */
use super::help::builtin_help;
use crate::builtins::common::get_exitstat;
use crate::src_common::*;

extern "C" {
    pub fn siglongjmp(__env: *mut __jmp_buf_tag, __val: libc::c_int);
}

#[no_mangle]
pub fn return_builtin(list: *mut WordList) -> i32 {
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

        return_catch_value = get_exitstat(list);
        if return_catch_flag != 0 {
            siglongjmp(std::mem::transmute(&return_catch), 1);
        } else {
            builtin_error(
                "can only `return' from a function or sourced script\0".as_ptr()
                    as *const libc::c_char,
            );
            return EX_USAGE;
        }
    }
    return EXECUTION_SUCCESS;
}
