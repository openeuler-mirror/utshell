use std::{ffi::CStr};
use std::mem::{size_of, transmute};
use libc::{c_char, c_int, c_void,__errno_location};
use r_bash::*;
use r_jobs::{BLOCK_CHILD, UNBLOCK_CHILD, SIG_IGN,};
use rexec_cmd::{r_exec_cmd};
use rcommon::{WordList, WordDesc};
use stdext::function_name;

extern "C"{
    static mut the_printed_command: *mut libc::c_char;
    static mut shellstart: timeval;
    static mut command_string_index: libc::c_int;

    fn add_unwind_protect(cleanup:*mut Function, arg:*mut c_char);
    fn make_child(_: *mut libc::c_char, _: libc::c_int) -> pid_t;
    fn difftimeval(_: *mut timeval, _: *mut timeval, _: *mut timeval) -> *mut timeval;
    fn addtimeval(_: *mut timeval, _: *mut timeval, _: *mut timeval) -> *mut timeval;
    fn timeval_to_cpu(_: *mut timeval, _: *mut timeval, _: *mut timeval) -> libc::c_int;
    fn timeval_to_secs(tvp:*mut timeval, sp:*mut time_t, sfp:*mut c_int);
    fn mbstowcs(__pwcs: *mut wchar_t, __s: *const libc::c_char, __n: size_t) -> size_t;
    fn read_builtin(_: *mut WordList) -> libc::c_int;
    fn list_length(_:*mut GENERIC_LIST) -> libc::c_int;
    fn strmatch(
        _: *mut libc::c_char,
        _: *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int;
    fn command_builtin(_: *mut WordList) -> libc::c_int;
    fn eval_builtin(_: *mut WordList) -> libc::c_int;
    fn source_builtin(_: *mut WordList) -> libc::c_int;
    fn unset_builtin(_: *mut WordList) -> libc::c_int;
    fn mapfile_builtin(_: *mut WordList) -> libc::c_int;
    fn fc_builtin(_: *mut WordList) -> libc::c_int;
    fn return_builtin(_: *mut WordList) -> libc::c_int;
    fn jobs_builtin(_: *mut WordList) -> libc::c_int;
    fn exec_builtin(_: *mut WordList) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fpurge(stream: *mut FILE) -> libc::c_int;
    fn sh_regmatch(a: *const libc::c_char, b:*const libc::c_char, c:libc::c_int) -> libc::c_int;
}
pub const r_input_direction: r_instruction = 1;
pub const r_input_output: r_instruction = 11;
pub const r_inputa_direction: r_instruction = 2;
pub const r_duplicating_input_word: r_instruction = 13;
pub const r_duplicating_output_word: r_instruction = 14;
pub const r_move_input_word: r_instruction = 17;
pub const r_move_output_word: r_instruction = 18;

#[macro_export]
macro_rules! FREE {
    ($s:expr) => {
        if ($s) != std::ptr::null_mut() {
            free($s as *mut c_void);
        }
    };
}

#[macro_export]
macro_rules! FD_BITMAP_DEFAULT_SIZE {
    () => {
        32
    };
}

#[macro_export]
macro_rules! QUIT {
    () => {
        if terminating_signal != 0 {
            termsig_handler(terminating_signal);
        }
        if interrupt_state != 0 {
            throw_to_top_level();
        } 
    };
}
#[macro_export]
macro_rules! errno {
    () => {
        *__errno_location()
    };
}

#[macro_export]
macro_rules! savestring {
    ($x:expr) => {
        strcpy(malloc((strlen($x as *const c_char) + 1) as usize) as *mut c_char, $x) as *mut c_char
    };
}

#[macro_export]
macro_rules! DESCRIBE_PID {
    ($pid:expr) => {
        if interactive != 0 {
            describe_pid($pid);
        }
    };
}

#[macro_export]
macro_rules! NO_PID {
    () => {
        -1 as pid_t
    };
}

#[macro_export]
macro_rules! RESIZE_MALLOCED_BUFFER {
    ($srt:expr,$cind:expr, $room:expr, $csize:expr, $sincr:expr) => {
        if $cind + $room   >= $csize {
            while $cind + $room >= $csize {
                $csize += $sincr;
            }
            $srt = realloc($srt as *mut c_void, $csize as usize) as *mut c_char;
        }
    };
}

#[macro_export]
macro_rules! COPY_PROCENV {
    ($old:expr, $save:expr) => {
        xbcopy(
            $old.as_mut_ptr() as *mut libc::c_char,
            $save.as_mut_ptr() as *mut libc::c_char,
            size_of::<sigjmp_buf>() as libc::c_ulong as libc::c_int,
        );
    };
}


#[macro_export]
macro_rules! setjmp_nosigs {
    ($x:expr) => {
        __sigsetjmp($x.as_mut_ptr(), 0 as libc::c_int)
    };
}


#[macro_export]
macro_rules! POSIX_TIMEFORMAT {
    () => {
        b"real %2R\nuser %2U\nsys %2S\0" as *const u8 as *mut libc::c_char
    };
}

#[macro_export]
macro_rules! BASH_TIMEFORMAT {
    () => {
        b"\nreal\t%3lR\nuser\t%3lU\nsys\t%3lS\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char;
    };
}

#[macro_export]
macro_rules! CHECK_TERMSIG {
    () => {
        if terminating_signal != 0 {
            termsig_handler(terminating_signal);
        }
    };
}
#[macro_export]
macro_rules! STREQ {
    ($a:expr, $b:expr) => {
        (*($a).offset(0 as isize) as libc::c_int
         == *$b.offset(0 as isize) as libc::c_int
            && strcmp($a, $b) == 0 )
    };
}




unsafe fn DIGIT(c: c_char) -> bool {
    char::from(c as u8) >= '0' && char::from(c as u8) <= '9'
}

pub type __rusage_who = libc::c_int;
pub const RUSAGE_THREAD: __rusage_who = 1;
pub const RUSAGE_CHILDREN: __rusage_who = -1;
pub const RUSAGE_SELF: __rusage_who = 0;

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

//函数重构部分
#[no_mangle]
pub unsafe extern "C" fn new_fd_bitmap(mut size: libc::c_int) -> *mut fd_bitmap 
{
    let mut ret: *mut fd_bitmap = 0 as *mut fd_bitmap;

    ret = malloc(size_of::<fd_bitmap>() as usize) as *mut fd_bitmap;
    
    (*ret).size = size;

    if size != 0 {
        (*ret).bitmap = malloc(size as usize) as *mut c_char;
        memset(
            (*ret).bitmap as *mut libc::c_void,
            '\u{0}' as i32,
            size as usize,
        );
    } else {
        (*ret).bitmap = 0 as *mut libc::c_char;
    }
    return ret;
}

#[no_mangle]
pub unsafe extern "C" fn dispose_fd_bitmap(mut fdbp: *mut fd_bitmap) 
{
    FREE!((*fdbp).bitmap);
    free(fdbp as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn close_fd_bitmap(mut fdbp: *mut fd_bitmap) 
{
    let mut i: libc::c_int = 0;

    if !fdbp.is_null() {
        i = 0;
        while i < (*fdbp).size {
            if *((*fdbp).bitmap).offset(i as isize) != 0 {
                close(i);
                *((*fdbp).bitmap).offset(i as isize) = 0 as libc::c_char;
            }
            i += 1;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn executing_line_number() -> libc::c_int {
    if executing != 0 && showing_function_line == 0 
        && (variable_context == 0 || interactive_shell == 0 )
        && !currently_executing_command.is_null()
    {
        if (*currently_executing_command).type_ as libc::c_uint
            == command_type_cm_cond as libc::c_uint
        {
            return (*(*currently_executing_command).value.Cond).line;
        }
        if (*currently_executing_command).type_ as libc::c_uint
            == command_type_cm_arith  as libc::c_uint
        {
            return (*(*currently_executing_command).value.Arith).line;
        }
        if (*currently_executing_command).type_ as libc::c_uint
            == command_type_cm_arith_for as libc::c_uint
        {
            return (*(*currently_executing_command).value.ArithFor).line;
        }
        return line_number;
    } else {
        return line_number
    };
}

#[no_mangle]
pub unsafe extern "C" fn execute_command(mut command: *mut COMMAND) -> libc::c_int {
    let mut bitmap: *mut fd_bitmap = 0 as *mut fd_bitmap;
    let mut result: libc::c_int = 0;

    current_fds_to_close = 0 as *mut fd_bitmap;
    bitmap = new_fd_bitmap(FD_BITMAP_DEFAULT_SIZE!());
    begin_unwind_frame(b"execute-command\0" as *const u8 as *mut libc::c_char,);
    add_unwind_protect(
            ::std::mem::transmute::<
                unsafe extern "C" fn(*mut fd_bitmap) -> (),
                *mut Function,
            >(dispose_fd_bitmap),
        bitmap as *mut libc::c_char,
    );

    result = execute_command_internal(
        command, 
        0,
        NO_PIPE,
        NO_PIPE,
        bitmap,
    );

    dispose_fd_bitmap(bitmap);
    discard_unwind_frame(b"execute-command\0" as *const u8 as *mut libc::c_char);
    
    if variable_context == 0 && executing_list == 0 {
        unlink_fifo_list();
    }

    QUIT!();
    return result;
}

unsafe extern "C" fn shell_control_structure(mut type_0: command_type) -> libc::c_int {
    match type_0 as libc::c_uint {
        command_type_cm_arith_for   | 
        command_type_cm_select      | 
        command_type_cm_arith | 
        command_type_cm_cond | 
        command_type_cm_case | 
        command_type_cm_while | 
        command_type_cm_until | 
        command_type_cm_if | 
        command_type_cm_for |
        command_type_cm_group | 
        command_type_cm_function_def => return 1 as libc::c_int,
        
        _ => return 0 as libc::c_int,
    };
}


unsafe extern "C" fn cleanup_redirects(mut list: *mut REDIRECT) {
    do_redirections(list, RX_ACTIVE as libc::c_int);
    dispose_redirects(list);
}







