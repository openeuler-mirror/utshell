//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::CString;

use crate::src_common::*;
use crate::subst::string_list;

use crate::builtins::{common::no_options, evalstring::evalstring};

#[no_mangle]
pub fn eval_builtin(mut list: *mut WordList) -> i32 {
    if no_options(list) != 0 {
        return EX_USAGE!();
    }

    list = unsafe { loptend };

    if !list.is_null() {
        let c_str = CString::new("eval").unwrap();
        let c_ptr = c_str.as_ptr();
        return unsafe { evalstring(string_list(list), c_ptr, SEVAL_NOHIST!()) };
    } else {
        return EXECUTION_SUCCESS!();
    }
}
