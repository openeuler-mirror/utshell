
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
pub type WordList = word_list;

pub const EXECUTION_SUCCESS : c_int = 0;
pub const EXECUTION_FAILURE : c_int = 1;
pub const EX_USAGE: c_int = 258;

pub const EXP_EXPANDED: c_int = 0x01;

pub type histdata_t = *mut libc::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _hist_entry {
    pub line: *mut c_char,
    pub timestamp: *mut c_char,
    pub data: histdata_t,
}
pub type HIST_ENTRY = _hist_entry;

extern "C" {
    pub fn string_list(list: *mut WordList) -> *mut c_char;

    pub fn builtin_usage();
    pub fn builtin_help();
    pub fn builtin_error(format: *const c_char, ...);

    pub fn evalexp (expr: *mut c_char, flags: c_int, validp: *mut c_int) -> c_long;

    pub static mut list_optarg : *mut libc::c_char;
    pub static mut loptend : *mut WordList;

}
