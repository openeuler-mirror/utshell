
use libc::{F_UNLCK, c_char, c_long};
use std::ffi::CString;

#[repr(C)]
pub struct WORD_DESC {
    pub word: *mut libc::c_char,
    pub flags:libc::c_int
}

#[repr(C)]
#[derive(Copy,Clone)]
pub struct WORD_LIST {
    next: *mut WORD_LIST,
    word: *mut WORD_DESC
}

/*
Read a line from the standard input and split it into fields.

Reads a single line from the standard input, or from file descriptor FD
if the -u option is supplied.  The line is split into fields as with word
splitting, and the first word is assigned to the first NAME, the second
word to the second NAME, and so on, with any leftover words assigned to
the last NAME.  Only the characters found in $IFS are recognized as word
delimiters.

If no NAMEs are supplied, the line read is stored in the REPLY variable.
*/

#[no_mangle]
pub extern "C" fn r_read_builtin(list: *mut WORD_LIST) -> i32 {
    println!("r_read_builtin call");
    0
}