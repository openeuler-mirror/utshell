extern crate libc;

use libc::{c_char,c_int, strchr, putchar,clearerr,free,FILE, fprintf, c_void};
use std::ffi::{CString,CStr,};
use rcommon::{WordList, WordDesc, EX_USAGE, EXECUTION_SUCCESS, EXECUTION_FAILURE};
// use std::io::{stdout, Write};
use std::ptr::read_volatile;
use rhelp::r_builtin_help;

//结构体

//枚举

//宏
#[macro_export]
macro_rules!  VALID_ECHO_OPTIONS {
    () => {
        CString::new("neE").unwrap().as_ptr()
    };
}


#[macro_export]
macro_rules!  QUIT {
    () => {
        if read_volatile(&terminating_signal as *const i32) != 0{
            termsig_handler(read_volatile(&terminating_signal as *const i32));
        }
        if interrupt_state != 0{
            throw_to_top_level();
        }
    };
}


