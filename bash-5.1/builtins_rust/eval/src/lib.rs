extern crate libc;
extern crate rcommon;

use libc::{c_char,c_int};
use std::ffi::{CString};
use rcommon::{r_no_options,WORD_LIST};

// #[repr(C)]
// #[derive(Copy, Clone)]
// pub struct word_desc {
//     pub word: *mut c_char,
//     pub flags: c_int,
// }
// pub type WORD_DESC = word_desc;

// #[repr(C)]
// #[derive(Copy, Clone)]
// pub struct word_list {
//     pub next: *mut word_list,
//     pub word: *mut WORD_DESC,
// }
// pub type WORD_LIST = word_list;

#[macro_export]
macro_rules! EX_USAGE {
    () => {258}
}

#[macro_export]
macro_rules! SEVAL_NOHIST {
    () => {0x004}
}

#[macro_export]
macro_rules! EXECUTION_SUCCESS {
  () => {
    0
  }
}


extern "C" {
    static loptend:*mut WORD_LIST;

    // fn no_options(list:*mut WORD_LIST)->i32;
    fn evalstring(string:*mut c_char,from_file:*const c_char,flag:i32)->i32;
    fn string_list(list:*mut WORD_LIST)->*mut c_char;
}

#[no_mangle]
pub extern "C" fn r_eval_builtin(mut list:*mut WORD_LIST)->i32{
    println!("r_eval_builtin");
    
    unsafe{
        if r_no_options(list) != 0{
            return EX_USAGE!();
        }

        list = loptend;

        if !list.is_null() {
            let c_str = CString::new("eval").unwrap();
            let c_ptr = c_str.as_ptr();
            return evalstring(string_list(list),c_ptr,SEVAL_NOHIST!());
        }
        else{
            return EXECUTION_SUCCESS!();
        }
    }
  
}










#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
