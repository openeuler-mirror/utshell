use super::help::builtin_help;
use crate::builtins::common::err_translate_fn;
use crate::expr::evalexp;
use crate::src_common::*;
use crate::subst::string_list;

#[no_mangle]
pub fn let_builtin(mut list: *mut WordList) -> i32 {
    let mut ret: libc::c_long = 0;
    let expok: libc::c_int = 0;

    if unsafe {
        !list.is_null()
            && !(*list).word.is_null()
            && libc::strcmp(
                (*((*list).word)).word,
                "--help\0".as_ptr() as *const libc::c_char,
            ) == 0
    } {
        builtin_help();
        return EX_USAGE;
    }

    if unsafe {
        !list.is_null() && !(*list).word.is_null() && is_option((*((*list).word)).word, b'-')
    } {
        list = unsafe { (*list).next };
    }

    if list.is_null() {
        let names = String::from("letwarn");
        err_translate_fn(&names, std::ptr::null_mut());
        println!();
        return EXECUTION_FAILURE;
    }

    while !list.is_null() {
        ret = unsafe {
            evalexp(
                (*((*list).word)).word,
                EXP_EXPANDED,
                std::mem::transmute(&expok),
            )
        };
        if expok == 0 {
            return EXECUTION_FAILURE;
        }
        list = unsafe { (*list).next };
    }

    return if ret == 0 {
        EXECUTION_FAILURE
    } else {
        EXECUTION_SUCCESS
    };
}

#[no_mangle]
pub fn exp_builtin(list: *mut WordList) -> i32 {
    let expok: libc::c_int = 0;

    if list.is_null() {
        let names = String::from("letwarn");
        err_translate_fn(&names, std::ptr::null_mut());
        println!();
        return EXECUTION_FAILURE;
    }

    let exp = string_list(list);
    let ret = unsafe { evalexp(exp, EXP_EXPANDED, std::mem::transmute(&expok)) };
    unsafe {
        libc::free(exp as *mut c_void);
    }
    return if ret == 0 || expok == 0 {
        EXECUTION_FAILURE
    } else {
        EXECUTION_SUCCESS
    };
}

fn is_option(s: *mut libc::c_char, c: u8) -> bool {
    unsafe {
        let str = CStr::from_ptr(s).to_bytes_with_nul();
        return str[0] == b'-' && str[1] == c && str[2] != 0;
    }
}
