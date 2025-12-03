//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

use super::common::{builtin_usage, sh_invalidnum};
use crate::array::array_reference;
use crate::builtins::common::no_options;
use crate::general::legal_number;
use crate::src_common::*;
use crate::GET_ARRAY_FROM_VAR;

fn array_empty(a: *mut ARRAY) -> bool {
    if unsafe { (*a).num_elements == 0 } {
        return true;
    } else {
        return false;
    }
}

#[no_mangle]
pub fn caller_builtin(mut list: *mut WordList) -> i32 {
    let funcname_v: *mut SHELL_VAR;
    let bash_source_v: *mut SHELL_VAR;
    let bash_lineno_v: *mut SHELL_VAR;
    let funcname_a: *mut ARRAY;
    let bash_source_a: *mut ARRAY;
    let bash_lineno_a: *mut ARRAY;
    let funcname_s: *mut libc::c_char;
    let mut source_s: *mut libc::c_char;
    let mut lineno_s: *mut libc::c_char;
    let mut num: intmax_t = 0;

    let mut c_str: CString;

    unsafe {
        CHECK_HELPOPT!(list);

        let c_str1 = CString::new("FUNCNAME").unwrap();
        let c_ptr1 = c_str1.as_ptr();
        GET_ARRAY_FROM_VAR!(c_ptr1, funcname_v, funcname_a);

        let c_str1 = CString::new("BASH_SOURCE").unwrap();
        let c_ptr1 = c_str1.as_ptr();
        GET_ARRAY_FROM_VAR!(c_ptr1, bash_source_v, bash_source_a);

        let c_str1 = CString::new("BASH_LINENO").unwrap();
        let c_ptr1 = c_str1.as_ptr();
        GET_ARRAY_FROM_VAR!(c_ptr1, bash_lineno_v, bash_lineno_a);

        if bash_lineno_a.is_null() || array_empty(bash_lineno_a) {
            return EXECUTION_FAILURE!();
        }

        if bash_source_a.is_null() || array_empty(bash_source_a) {
            return EXECUTION_FAILURE!();
        }

        if no_options(list) != 0 {
            return EX_USAGE!();
        }

        list = loptend; /* skip over possible `--' */
        /* If there is no argument list, then give short form: line filename. */
        if list.is_null() {
            lineno_s = array_reference(bash_lineno_a, 0);
            source_s = array_reference(bash_source_a, 1);

            if !lineno_s.is_null() {
                lineno_s = lineno_s;
            } else {
                c_str = CString::new("NULL").unwrap();
                lineno_s = c_str.as_ptr() as *mut libc::c_char;
            }

            if !source_s.is_null() {
                source_s = source_s;
            } else {
                c_str = CString::new("NULL").unwrap();
                source_s = c_str.as_ptr() as *mut libc::c_char;
            }
            let lineno_s_str = CStr::from_ptr(lineno_s).to_str().unwrap().to_owned();
            let source_s_str = CStr::from_ptr(source_s).to_str().unwrap().to_owned();
            println!("{} {}", lineno_s_str, source_s_str);

            return EXECUTION_SUCCESS!();
        }

        if funcname_a.is_null() || array_empty(funcname_a) {
            return EXECUTION_FAILURE!();
        }
        if legal_number((*(*list).word).word, &mut num) != 0 {
            lineno_s = array_reference(bash_lineno_a, num);
            source_s = array_reference(bash_source_a, num + 1);
            funcname_s = array_reference(funcname_a, num + 1);

            if lineno_s == PT_NULL as *mut libc::c_char
                || source_s == PT_NULL as *mut libc::c_char
                || funcname_s == PT_NULL as *mut libc::c_char
            {
                return EXECUTION_FAILURE!();
            }
            let lineno_s_str = CStr::from_ptr(lineno_s).to_str().unwrap().to_owned();
            let funcname_s_str = CStr::from_ptr(funcname_s).to_str().unwrap().to_owned();
            let source_s_str = CStr::from_ptr(source_s).to_str().unwrap().to_owned();
            println!("{} {} {}", lineno_s_str, funcname_s_str, source_s_str);
        } else {
            sh_invalidnum((*(*list).word).word);
            builtin_usage();
            return EX_USAGE!();
        }

        return EXECUTION_SUCCESS!();
    }
}
