
#[repr(C)]
#[derive(Copy, Clone)]
pub struct word_desc {
    pub word: *mut c_char,
    pub flags: c_int,
}
pub type WORD_DESC = word_desc;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct word_list {
    pub next: *mut word_list,
    pub word: *mut WORD_DESC,
}
pub type WORD_LIST = word_list;

pub const EXECUTION_SUCCESS : c_int = 0;
pub const EXECUTION_FAILURE : c_int = 1;
pub const EX_USAGE: c_int = 258;

extern "C" {
    pub fn no_options(list: *mut WORD_LIST) -> c_int;
    pub fn print_timeval(fp: *mut libc::FILE, tvp: *mut libc::timeval);
    pub fn sh_chkwrite(s: c_int) -> c_int;

    pub static stdout: *mut libc::FILE;
}
