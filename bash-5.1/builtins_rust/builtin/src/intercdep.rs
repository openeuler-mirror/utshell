
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

/*
#[macro_export]
macro_rules! EX_USAGE {
   () => {258}
}
*/
pub const EX_USAGE: c_int = 258;
pub const EXECUTION_SUCCESS :c_int = 0;

/*
pub const EXECUTION_SUCCESS : c_int = 0;
pub const EXECUTION_FAILURE : c_int = 1;
*/
//typedef int sh_builtin_func_t PARAMS((WORD_LIST *)); /* sh_wlist_func_t */
extern "C" {
    static mut loption :*mut WORD_LIST;
    pub fn no_options(list: *mut WORD_LIST) -> c_int;
    pub fn builtin_address(command: *const c_char) -> extern "C" fn(w:*mut WORD_LIST) ->i32;
    pub static mut loptend : *mut WORD_LIST;

    /*
    pub fn print_timeval(fp: *mut libc::FILE, tvp: *mut libc::timeval);
    pub fn sh_chkwrite(s: c_int) -> c_int;

    pub static stdout: *mut libc::FILE;
    */
}
