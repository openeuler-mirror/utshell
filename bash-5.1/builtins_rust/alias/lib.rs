use std::ffi::CStr;
//extern crate rcommon;
use rcommon::r_sh_notfound;
use rcommon::{WordList, WordDesc, EX_USAGE, EXECUTION_SUCCESS, EXECUTION_FAILURE,r_builtin_usage,SHELL_VAR};
use rhelp::r_builtin_help;
use std::ffi::CString;

extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn legal_alias_name(_: *const libc::c_char, _: libc::c_int) -> libc::c_int;
    fn sh_single_quote(_: *const libc::c_char) -> *mut libc::c_char;
    static mut posixly_correct: libc::c_int;
    static mut aliases: *mut HashTable;
    fn find_alias(_: *mut libc::c_char) -> *mut AliasT;
    fn add_alias(_: *mut libc::c_char, _: *mut libc::c_char);
    fn remove_alias(_: *mut libc::c_char) -> libc::c_int;
    fn delete_all_aliases();
    fn all_aliases() -> *mut *mut AliasT;
    fn builtin_error(_: *const libc::c_char, _: ...);
    fn builtin_usage();
    fn sh_notfound(_: *mut libc::c_char);
    fn sh_chkwrite(_: libc::c_int) -> libc::c_int;
    static mut loptend: *mut WordList;
    fn internal_getopt(_: *mut WordList, _: *mut libc::c_char) -> libc::c_int;
    fn find_user_command(name:*const libc::c_char)->*mut libc::c_char;
    fn find_shell_builtin(builtin: *mut libc::c_char) -> *mut libc::c_char;
    fn find_function (name:* const libc::c_char)-> *mut SHELL_VAR;
    fn reset_internal_getopt();
}
pub type SizeT = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bucket_contents {
    pub next: *mut bucket_contents,
    pub key: *mut libc::c_char,
    pub data: *mut libc::c_void,
    pub khash: libc::c_uint,
    pub times_found: libc::c_int,
}
pub type BucketContents = bucket_contents;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table {
    pub bucket_array: *mut *mut BucketContents,
    pub nbuckets: libc::c_int,
    pub nentries: libc::c_int,
}
pub type HashTable = hash_table;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alias {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub flags: libc::c_char,
}
pub type AliasT = alias;
pub static AL_REUSABLE:i32 = 0x01;
