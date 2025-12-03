//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later
use crate::builtins::common::make_builtin_argv;
use crate::src_common::*;
use crate::test::test_command;

#[no_mangle]
pub fn test_builtin(list: *mut WordList) -> i32 {
    let result: libc::c_int;
    let argc: libc::c_int = 0;
    unsafe {
        if list.is_null() {
            if *this_command_name == b'[' as libc::c_char
                && *((this_command_name as usize + 1) as *mut libc::c_char) == 0
            {
                builtin_error("missing `]'\0".as_ptr() as *mut libc::c_char);
                return EX_BADUSAGE;
            }
            return EXECUTION_FAILURE;
        }
        let argv = make_builtin_argv(list, std::mem::transmute(&argc));

        result = test_command(argc, argv);
        libc::free(argv as *mut c_void);
    }
    return result;
}
