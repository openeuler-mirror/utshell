//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

use super::common::{find_shell_builtin, no_options, sh_notbuiltin};
use crate::src_common::*;

#[no_mangle]
pub fn builtin_builtin(mut list: *mut WordList) -> i32 {
    unsafe {
        let mut function: Option<sh_builtin_func_t> = None;
        let mut command: *mut libc::c_char = 0 as *mut libc::c_char;
        if no_options(list) != 0 {
            return 258 as libc::c_int;
        }
        list = loptend;
        if list.is_null() {
            return 0 as libc::c_int;
        }
        command = (*(*list).word).word;
        function = find_shell_builtin(command);
        if function.is_none() {
            sh_notbuiltin(command);
            return 1 as libc::c_int;
        } else {
            this_command_name = command;
            this_shell_builtin = function;
            list = (*list).next;
            return (Some(function.expect("non-null function pointer")))
                .expect("non-null function pointer")(list);
        };
    }
}
