use std::{ffi::CString};
use libc::{size_t, c_int, c_uint, c_char, c_long, c_void, PT_NULL, c_ulong, strchr};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct word_desc {
    pub word: *mut c_char,
    pub flags: c_int,
}
pub type WordDesc = word_desc;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct word_list {
    pub next: *mut word_list,
    pub word: *mut WordDesc,
}
pub type WORD_LIST = word_list;

#[no_mangle]
pub extern "C" fn r_colon_builtin(ignore:WORD_LIST)->i32 {
    println!("in r_colon_builtin");
    0
}

#[no_mangle]
pub extern "C" fn r_false_builtin(ignore: WORD_LIST) -> i32 {
    println!("in r_false_builtin");
    1
}
