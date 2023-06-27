
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

pub const EXECUTION_SUCCESS : c_int = 0;
pub const EXECUTION_FAILURE : c_int = 1;
pub const EX_USAGE: c_int = 258;

pub type __jmp_buf = [c_long; 8usize];

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sigset_t {
    pub __val: [c_ulong; 16usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: c_int,
    pub __saved_mask: __sigset_t,
}
pub type sigjmp_buf = [__jmp_buf_tag; 1usize];

extern "C" {
    pub fn builtin_usage();
    pub fn builtin_help();
    pub fn builtin_error(format: *const c_char, ...);

    pub fn get_exitstat(list: *mut WORD_LIST) -> c_int;

    pub fn siglongjmp(__env: *mut __jmp_buf_tag, __val: c_int);

    pub static mut list_optarg : *mut libc::c_char;
    pub static mut loptend : *mut WORD_LIST;

    pub static mut return_catch_value: c_int;
    pub static return_catch_flag: c_int;
    pub static return_catch: sigjmp_buf;
}
