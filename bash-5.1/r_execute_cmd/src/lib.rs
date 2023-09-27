use std::{ffi::CStr};
use std::mem::{size_of, transmute};
use libc::{c_char, c_int, c_void,__errno_location};
use r_bash::*;
use r_jobs::{BLOCK_CHILD, UNBLOCK_CHILD, SIG_IGN,};
use rexec_cmd::{r_exec_cmd};
use rcommon::{WordList, WordDesc};
use stdext::function_name;


#[no_mangle]
pub static mut stdin_redir: libc::c_int = 0;
#[no_mangle]
pub static mut this_command_name: *mut libc::c_char = 0 as *mut libc::c_char;
#[no_mangle]
pub static mut the_printed_command_except_trap: *mut libc::c_char = 0 as *mut libc::c_char;
#[no_mangle]
pub static mut return_catch_flag: libc::c_int = 0;
#[no_mangle]
pub static mut return_catch_value: libc::c_int = 0;
#[no_mangle]
pub static mut return_catch: sigjmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
#[no_mangle]
pub static mut last_command_exit_value: libc::c_int = 0;
#[no_mangle]
pub static mut last_command_exit_signal: libc::c_int = 0;
#[no_mangle]
pub static mut builtin_ignoring_errexit: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut redirection_undo_list: *mut REDIRECT = 0 as *const libc::c_void
    as *mut libc::c_void as *mut REDIRECT;
#[no_mangle]
pub static mut exec_redirection_undo_list: *mut REDIRECT = 0 as *const libc::c_void
    as *mut libc::c_void as *mut REDIRECT;

#[no_mangle]
pub static mut executing_builtin: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut executing_list: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut comsub_ignore_return: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut subshell_environment: libc::c_int = 0;
#[no_mangle]
pub static mut subshell_level: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut this_shell_function: *mut SHELL_VAR = 0 as *const SHELL_VAR
    as *mut SHELL_VAR;
#[no_mangle]
pub static mut match_ignore_case: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut executing_command_builtin: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut SB: stat = stat {
    st_dev: 0,
    st_ino: 0,
    st_nlink: 0,
    st_mode: 0,
    st_uid: 0,
    st_gid: 0,
    __pad0: 0,
    st_rdev: 0,
    st_size: 0,
    st_blksize: 0,
    st_blocks: 0,
    st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
    st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
    st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
    __glibc_reserved: [0; 3],
};
static mut special_builtin_failed: libc::c_int = 0;
static mut currently_executing_command: *mut COMMAND = 0 as *const COMMAND
    as *mut COMMAND;
static mut function_line_number: libc::c_int = 0;
static mut showing_function_line: libc::c_int = 0;
#[no_mangle]
pub static mut line_number_for_err_trap: libc::c_int = 0;
#[no_mangle]
pub static mut funcnest: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut funcnest_max: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut evalnest: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut evalnest_max: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut sourcenest: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut sourcenest_max: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut from_return_trap: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut lastpipe_opt: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut current_fds_to_close: *mut fd_bitmap = 0 as *const libc::c_void
    as *mut libc::c_void as *mut fd_bitmap;





















