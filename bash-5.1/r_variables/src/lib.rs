use libc::*;
use r_bash::*;
use std::ffi::CStr;

extern "C" {
   fn xmalloc(_: size_t) -> *mut libc::c_void;
   fn parse_and_execute(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int;
}

#[no_mangle] 
pub unsafe extern "C" fn bind_variable(
    mut name: *const libc::c_char,
    mut value: *mut libc::c_char,
    mut flags: libc::c_int,
) -> *mut SHELL_VAR {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut nv: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut vc: *mut VAR_CONTEXT = 0 as *mut VAR_CONTEXT;
    let mut nvc: *mut VAR_CONTEXT = 0 as *mut VAR_CONTEXT;

   
    if shell_variables.is_null() {
        create_variable_tables();
    }
    if !temporary_env.is_null() && !value.is_null() {
        bind_tempenv_variable(name, value);
    }
    vc = shell_variables;
    while !vc.is_null() {
        if vc_isfuncenv!(vc) || vc_isbltnenv!(vc)
        {
        }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct saved_dollar_vars {
    pub first_ten: *mut *mut libc::c_char,
    pub rest: *mut WORD_LIST,
    pub count: libc::c_int,
}

