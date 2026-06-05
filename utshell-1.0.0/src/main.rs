#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![allow(clippy::all)]

use builtins::utshell::main_0;

pub fn main() {
    //获取命令行参数
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());

    //获取环境变量
    let mut vars: Vec<*mut libc::c_char> = Vec::new();
    for (var_name, var_value) in ::std::env::vars() {
        let var: String = format!("{}={}", var_name, var_value);
        vars.push(
            (::std::ffi::CString::new(var))
                .expect("Failed to convert environment variable into CString.")
                .into_raw(),
        );
    }
    vars.push(::std::ptr::null_mut());

    ::std::process::exit(main_0(
        (args.len() - 1) as libc::c_int,
        args.as_mut_ptr() as *mut *mut libc::c_char,
        vars.as_mut_ptr() as *mut *mut libc::c_char,
    ) as i32)
}
