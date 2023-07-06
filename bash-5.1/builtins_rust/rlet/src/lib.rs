//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later
use libc::{c_char, c_int, c_long, c_void};
use std::ffi::CStr;

include!(concat!("intercdep.rs"));

#[no_mangle]
pub extern "C" fn r_let_builtin(mut list: *mut WordList) -> i32 {
unsafe {
    let mut ret: c_long = 0;
	let expok: c_int = 0;

        if !list.is_null()
            && !(*list).word.is_null()
        {
            r_builtin_help();
            return EX_USAGE;
        }

	if !list.is_null() && !(*list).word.is_null()
           && is_option((*((*list).word)).word, b'-') {
		list = (*list).next;
    }

	if list.is_null() {
		let names = String::from("letwarn");
        err_translate_fn(&names,std::ptr::null_mut());
	    println!();
		return EXECUTION_FAILURE;
	}

	while !list.is_null() {
		ret = evalexp((*((*list).word)).word, EXP_EXPANDED, std::mem::transmute(&expok));
		if expok == 0 {
			return EXECUTION_FAILURE;
        }
        list = (*list).next;
	}

        return if ret == 0 {
            EXECUTION_FAILURE
        } else {
            EXECUTION_SUCCESS
        };
    }
}

#[no_mangle]
pub extern "C" fn r_exp_builtin(list: *mut WordList) -> i32 {

unsafe {
	let expok: c_int = 0;

	if list.is_null() {
		let names = String::from("letwarn");
        err_translate_fn(&names,std::ptr::null_mut());
		println!();
		return EXECUTION_FAILURE;
	}

	let exp = string_list(list);
	let ret = evalexp(exp, EXP_EXPANDED, std::mem::transmute(&expok));
	libc::free(exp as *mut c_void);
	return if ret == 0 || expok == 0 {EXECUTION_FAILURE} else {EXECUTION_SUCCESS};
}
}

unsafe fn is_option(s: *mut c_char, c: u8) -> bool {
    let str = CStr::from_ptr(s).to_bytes_with_nul();
    return str[0] == b'-' && str[1] == c && str[2] != 0;
}
