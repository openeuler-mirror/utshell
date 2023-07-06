use libc::{c_int,c_char, c_void, fprintf,fileno,  size_t, c_long, getcwd, c_ulong, __errno_location, getpgrp, tcgetpgrp, isatty, open, dup, tcsetpgrp,};
use std::{ffi::{CString, CStr, }, fmt::Alignment};
use rcommon::{the_current_working_directory, r_savestring, r_get_working_directory};
// include!(concat!("jobs_h.rs"));
// include!(concat!("flags_h.rs"));
use r_bash::*;
use stdext::function_name;


extern "C" {
    static mut interactive:  c_int;
    static mut interactive_shell:  c_int;
    static mut subshell_environment:  c_int;
    static mut stdout: *mut libc::FILE;
    static mut stderr: *mut libc::FILE;
    static mut posixly_correct:c_int;
    static mut default_buffered_input: c_int;
    static mut shell_level: c_int;
    static mut wait_signal_received: c_int;
    static mut this_shell_builtin:sh_builtin_func_t;
    static mut last_command_exit_value: c_int;
    static mut last_command_exit_signal: c_int;
    static mut rl_readline_state: libc::c_ulong;
    static mut loop_level: c_int;
    static mut shell_compatibility_level: c_int;
    static mut executing_list: c_int;
    static mut sourcelevel: c_int;
    static mut this_command_name: *mut c_char;
    static mut trap_list: [*mut c_char; 0];
    static mut running_trap: c_int;
    static mut executing_builtin: c_int;
    static mut breaking: c_int;
    static mut subst_assign_varlist: *mut WORD_LIST;
    static mut temporary_env: *mut HASH_TABLE;
    static mut startup_state: c_int;
    static mut line_number: c_int;
    
    fn get_string_value(_:*const c_char) -> *mut c_char;
    fn sys_error(format:*const c_char, ...);
    fn list_reverse(list:*mut GENERIC_LIST) -> *mut GENERIC_LIST;
    fn internal_warning(_: *const  c_char, _: ...);
    fn coproc_reap();
    fn programming_error(_: *const  c_char, _: ...);
    fn signal_name(_: c_int) -> *mut  c_char;
    fn sync_buffered_stream(_: c_int) -> c_int;
    fn set_exit_status(_: c_int);
    fn getpid() -> __pid_t;
    fn unset_bash_input(_: c_int);
    fn setpgid(__pid: __pid_t, __pgid: __pid_t) -> c_int;
    fn signal_is_trapped(_: c_int) -> c_int;
    fn signal_is_hard_ignored(_: c_int) -> c_int;}

// pub  static mut SIG_IGN:__sighandler_t = 1;
#[macro_export]
macro_rules! SIG_IGN {
    () => {
        ::std::mem::transmute::<
            libc::intptr_t,
            *mut __sighandler_t,
        >(1 as c_int as libc::intptr_t)
    };
}

#[macro_export]
macro_rules! SIG_DFL {
    () => {
        ::std::mem::transmute::<
        libc::intptr_t,
        __sighandler_t,
    >(0 as c_int as libc::intptr_t)
    };
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct bucket_contents {
    pub next: *mut bucket_contents,
    pub key: *mut c_char,
    pub data: *mut libc::c_void,
    pub khash: libc::c_uint,
    pub times_found: c_int,
}
pub type BUCKET_CONTENTS = bucket_contents;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table {
    pub bucket_array: *mut *mut BUCKET_CONTENTS,
    pub nbuckets: c_int,
    pub nentries: c_int,
}
pub type HASH_TABLE = hash_table;
