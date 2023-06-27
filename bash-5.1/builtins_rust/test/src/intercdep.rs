
// #[repr(C)]
// #[derive(Copy, Clone)]
// pub struct word_desc {
//     pub word: *mut c_char,
//     pub flags: c_int,
// }
// pub type WordDesc = word_desc;

// #[repr(C)]
// #[derive(Copy, Clone)]
// pub struct word_list {
//     pub next: *mut word_list,
//     pub word: *mut WordDesc,
// }
// pub type WORD_LIST = word_list;

pub const EXECUTION_SUCCESS : c_int = 0;
pub const EXECUTION_FAILURE : c_int = 1;
pub const EX_BADUSAGE : c_int = 2;
pub const EX_USAGE: c_int = 258;

extern "C" {
    pub fn builtin_error(format: *const c_char, ...);

    pub fn make_builtin_argv(list: *mut WORD_LIST, ip: *mut c_int) -> *mut *mut c_char;
    pub fn test_command (margc: c_int, margv: *mut *mut c_char) -> c_int;

    pub static mut list_optarg : *mut libc::c_char;
    pub static mut loptend : *mut WORD_LIST;
    pub static mut this_command_name: *mut c_char;
}
