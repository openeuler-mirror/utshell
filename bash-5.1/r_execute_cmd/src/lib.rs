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

#[no_mangle]
pub unsafe extern "C" fn undo_partial_redirects() {
    if !redirection_undo_list.is_null() {
        cleanup_redirects(redirection_undo_list);
        redirection_undo_list = 0 as *mut REDIRECT;
    }
}

#[no_mangle]
pub unsafe extern "C" fn dispose_exec_redirects() {
    if !exec_redirection_undo_list.is_null() {
        dispose_redirects(exec_redirection_undo_list);
        exec_redirection_undo_list = 0 as *mut REDIRECT;
    }
}


#[no_mangle]
pub unsafe extern "C" fn dispose_partial_redirects() {
    if !redirection_undo_list.is_null() {
        dispose_redirects(redirection_undo_list);
        redirection_undo_list = 0 as *mut REDIRECT;
    }
}

unsafe extern "C" fn restore_signal_mask(mut set: *mut sigset_t) -> libc::c_int {
    return sigprocmask(SIG_SETMASK as libc::c_int, set, 0 as *mut sigset_t);
}

#[no_mangle]
pub unsafe extern "C" fn async_redirect_stdin() {
    let mut fd: libc::c_int = 0;
    
    fd = open(b"/dev/null\0" as *const u8 as *const libc::c_char, O_RDONLY as libc::c_int);
    if fd > 0 {
        dup2(fd, 0 );
        close(fd);
    } else if fd < 0 {
        internal_error(
            b"cannot redirect standard input from /dev/null: %s\0" as *const u8 as *mut c_char,
            strerror(errno!()),
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn execute_command_internal(
    mut command: *mut COMMAND,
    mut asynchronous: libc::c_int,
    mut pipe_in: libc::c_int,
    mut pipe_out: libc::c_int,
    mut fds_to_close: *mut fd_bitmap,
) -> libc::c_int
{
    let mut exec_result: libc::c_int = 0;
    let mut user_subshell: libc::c_int = 0;
    let mut invert: libc::c_int = 0;
    let mut ignore_return: libc::c_int = 0;
    let mut was_error_trap: libc::c_int = 0;
    let mut fork_flags: libc::c_int = 0;
    let mut my_undo_list: *mut REDIRECT = 0 as *mut REDIRECT;
    let mut exec_undo_list: *mut REDIRECT = 0 as *mut REDIRECT;
    let mut tcmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut save_line_number: libc::c_int = 0;
    let mut ofifo: libc::c_int = 0;
    let mut nfifo: libc::c_int = 0;
    let mut osize: libc::c_int = 0;
    let mut saved_fifo: libc::c_int = 0;
    let mut ofifo_list: *mut libc::c_void = 0 as *mut libc::c_void;
    
    if breaking != 0 || continuing != 0 {
        return last_command_exit_value;
    }
    if command.is_null() || read_but_dont_execute != 0 && rpm_requires == 0 {
        return EXECUTION_SUCCESS as i32 ;
    }
    if rpm_requires != 0
        && (*command).type_ == command_type_cm_function_def 
    {
        last_command_exit_value =
               execute_intern_function((*(*command).value.Function_def).name,
                                       (*command).value.Function_def);
        return last_command_exit_value;
    }
    if read_but_dont_execute != 0 {
        return EXECUTION_SUCCESS as libc::c_int;
    }

    QUIT!();
    run_pending_traps();

    currently_executing_command = command;

    invert = ((*command).flags & CMD_INVERT_RETURN as libc::c_int != 0  ) as libc::c_int;
    
    if exit_immediately_on_error != 0 && invert != 0 {
        (*command).flags |= CMD_IGNORE_RETURN as libc::c_int;
    }
    
    exec_result = EXECUTION_SUCCESS as libc::c_int;
    
    if (*command).type_  == command_type_cm_subshell  
        && (*command).flags & CMD_NO_FORK as libc::c_int != 0
    {
        return execute_in_subshell(
            command,
            asynchronous,
            pipe_in,
            pipe_out,
            fds_to_close,
        );
    }
    if (*command).type_  == command_type_cm_coproc {
        last_command_exit_value = execute_coproc(command, pipe_in, pipe_out, fds_to_close);
        return last_command_exit_value;
    }

    user_subshell = ((*command).type_ == command_type_cm_subshell 
        || (*command).flags & CMD_WANT_SUBSHELL as libc::c_int != 0 ) as libc::c_int;
    
    if (*command).type_ == command_type_cm_subshell
        || (*command).flags & (CMD_WANT_SUBSHELL as libc::c_int | CMD_FORCE_SUBSHELL as libc::c_int) != 0
        || shell_control_structure((*command).type_ as libc::c_uint) != 0
            && (pipe_out != NO_PIPE || pipe_in != NO_PIPE
                || asynchronous != 0)
    {
        let mut paren_pid: pid_t = 0;
        let mut s: libc::c_int = 0;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        
        save_line_number = line_number;
        if (*command).type_== command_type_cm_subshell
        {
            line_number = (*(*command).value.Subshell).line;
            line_number_for_err_trap = line_number;
        }

        tcmd = make_command_string(command);
        fork_flags = if asynchronous != 0 { FORK_ASYNC as libc::c_int } else { 0 };
        p = savestring!(tcmd);
        paren_pid = make_child(p, fork_flags);

        if user_subshell != 0
            && signal_is_trapped(ERROR_TRAP as libc::c_int) != 0
            && signal_in_progress(DEBUG_TRAP as libc::c_int) == 0
            && running_trap == 0 
        {
            FREE!(the_printed_command_except_trap);
            the_printed_command_except_trap = savestring!(the_printed_command);
        }

        if paren_pid == 0 {
            FREE!(p);
            s = (user_subshell == 0 
                && (*command).type_ == command_type_cm_group 
                && pipe_in == NO_PIPE && pipe_out == NO_PIPE
                && asynchronous != 0) as libc::c_int;
            
            s += (user_subshell == 0
                 && (*command).type_== command_type_cm_group 
                 && (pipe_in != NO_PIPE || pipe_out != NO_PIPE)
                 && asynchronous == 0 ) as libc::c_int;

            last_command_exit_value = execute_in_subshell(command, asynchronous, pipe_in, pipe_out, fds_to_close);
            if s != 0 {
                subshell_exit(last_command_exit_value);
            } else {
                sh_exit(last_command_exit_value);
            }
        } else {
            close_pipes(pipe_in, pipe_out);

            if variable_context == 0 {
                unlink_fifo_list();
            }

            if pipe_out != NO_PIPE {
                return EXECUTION_SUCCESS as c_int;
            }

            stop_pipeline(asynchronous, 0 as *mut COMMAND);

            line_number = save_line_number;

            if asynchronous == 0 {
                was_error_trap = (signal_is_trapped(ERROR_TRAP as c_int) != 0
                    && signal_is_ignored( ERROR_TRAP as c_int) == 0 ) as libc::c_int;
                invert = ((*command).flags & CMD_INVERT_RETURN as libc::c_int != 0 ) as libc::c_int;
                ignore_return = ((*command).flags & CMD_IGNORE_RETURN as libc::c_int != 0) as libc::c_int;
               
                exec_result = wait_for(paren_pid, 0 );
                
                if invert != 0 {
                    exec_result = if exec_result == EXECUTION_SUCCESS as libc::c_int {
                        EXECUTION_FAILURE as libc::c_int
                    } else {
                        EXECUTION_SUCCESS as libc::c_int
                    };
                }

                last_command_exit_value = exec_result;
                if user_subshell != 0 && was_error_trap != 0
                    && ignore_return == 0 && invert == 0  
                    && exec_result != EXECUTION_SUCCESS as libc::c_int
                {
                    save_line_number = line_number;
                    line_number = line_number_for_err_trap;
                    run_error_trap();
                    line_number = save_line_number;
                }

                if user_subshell != 0 && ignore_return == 0 
                    && invert == 0 && exit_immediately_on_error != 0
                    && exec_result != EXECUTION_SUCCESS as libc::c_int
                {
                    run_pending_traps();
                    jump_to_top_level(ERREXIT as libc::c_int);
                }
                return last_command_exit_value;
            } else {
                DESCRIBE_PID!(paren_pid);

                run_pending_traps();
                
                last_command_exit_value = 0;
                return EXECUTION_SUCCESS as libc::c_int;
            }
        }
    }
    if (*command).flags & CMD_TIME_PIPELINE as libc::c_int != 0 
    {
        if asynchronous != 0 {
            (*command).flags |= CMD_FORCE_SUBSHELL as libc::c_int;
            exec_result = execute_command_internal(
                command,
                1,
                pipe_in,
                pipe_out,
                fds_to_close,
            );
        } else {
            exec_result = time_command(
                command,
                asynchronous,
                pipe_in,
                pipe_out,
                fds_to_close,
            );
            currently_executing_command = 0 as *mut COMMAND;
        }
        return exec_result;
    }
    if shell_control_structure((*command).type_ ) != 0 && !((*command).redirects).is_null()
    {
        stdin_redir = stdin_redirects((*command).redirects);
    }

    if variable_context != 0 || executing_list != 0 {
        ofifo = num_fifos();
        ofifo_list = copy_fifo_list(&mut osize as *mut libc::c_int);
        begin_unwind_frame( b"internal_fifos\0" as *const u8 as *mut libc::c_char);
        if !ofifo_list.is_null() {
            add_unwind_protect(
                ::std::mem::transmute::<
                unsafe extern "C" fn(*mut c_void) -> (),
                *mut Function,
            >(xfree),
                ofifo_list as *mut c_char);
        }
        saved_fifo = 1;
    } else {
        saved_fifo = 0;
    }

    was_error_trap = (signal_is_trapped(ERROR_TRAP as c_int) != 0
        && signal_is_ignored(ERROR_TRAP as libc::c_int) == 0 ) as libc::c_int;
    ignore_return = ((*command).flags & CMD_IGNORE_RETURN as libc::c_int != 0 ) as libc::c_int;
    
    if do_redirections((*command).redirects, RX_ACTIVE as libc::c_int | RX_UNDOABLE as libc::c_int) != 0
    {
        undo_partial_redirects();
        dispose_exec_redirects();
        if saved_fifo != 0 {
            free(ofifo_list as *mut c_void);
            discard_unwind_frame(b"internal_fifos\0" as *const u8 as *mut libc::c_char);
        }

        last_command_exit_value = EXECUTION_FAILURE as c_int;
        if ignore_return == 0 && invert == 0 && pipe_in == NO_PIPE && pipe_out == NO_PIPE
        {
            if was_error_trap != 0 {
                save_line_number = line_number;
                line_number = line_number_for_err_trap;
                run_error_trap();
                line_number = save_line_number;
            }
            if exit_immediately_on_error != 0 {
                run_pending_traps();
                jump_to_top_level(ERREXIT as libc::c_int);
            }
        }
        return last_command_exit_value;
    }

    my_undo_list = redirection_undo_list;
    redirection_undo_list = 0 as *mut REDIRECT;

    exec_undo_list = exec_redirection_undo_list;
    exec_redirection_undo_list = 0 as *mut REDIRECT;

    if !my_undo_list.is_null() || !exec_undo_list.is_null() {
        begin_unwind_frame( b"loop_redirections\0" as *const u8 as *mut libc::c_char);
    }
    if !my_undo_list.is_null() {
        add_unwind_protect(
            std::mem::transmute::<
            unsafe extern "C" fn(*mut REDIRECT) -> (),
                *mut Function,
            >(cleanup_redirects),
            my_undo_list as *mut c_char,
        );
    }
    if !exec_undo_list.is_null() {
        add_unwind_protect(
            transmute::<
                unsafe extern "C" fn (arg1: *mut REDIRECT) -> (),
                *mut Function,
            >(dispose_redirects),
            exec_undo_list as *mut c_char,
        );
    }
    
    QUIT!();

    match (*command).type_ {
        command_type_cm_simple => {
            save_line_number = line_number;
            was_error_trap = (signal_is_trapped(ERROR_TRAP as libc::c_int) != 0
                && signal_is_ignored(ERROR_TRAP as libc::c_int) == 0) as libc::c_int;
            
            if ignore_return != 0 && !((*command).value.Simple).is_null() {
                (*(*command).value.Simple).flags |= CMD_IGNORE_RETURN as libc::c_int;
            }
            if (*command).flags & CMD_STDIN_REDIR as libc::c_int != 0 {
                (*(*command).value.Simple).flags |= CMD_STDIN_REDIR as libc::c_int;
            }
            
            line_number = (*(*command).value.Simple).line;
            line_number_for_err_trap = line_number;
            exec_result = execute_simple_command(
                (*command).value.Simple,
                pipe_in,
                pipe_out,
                asynchronous,
                fds_to_close,
            );
            line_number = save_line_number;

            dispose_used_env_vars();

            if already_making_children != 0 && pipe_out == NO_PIPE {
                stop_pipeline(asynchronous, 0 as *mut COMMAND);
                if asynchronous != 0 {
                    DESCRIBE_PID!(last_made_pid);
                    exec_result = EXECUTION_SUCCESS as libc::c_int;
                    invert = 0;
                } else if last_made_pid != NO_PID!() {
                    exec_result = wait_for(last_made_pid, 0 as libc::c_int);
                }
            }

            if was_error_trap != 0 && ignore_return == 0 
                && invert == 0 && pipe_in == NO_PIPE
                && pipe_out == NO_PIPE
                && (*(*command).value.Simple).flags & CMD_COMMAND_BUILTIN as libc::c_int == 0 
                && exec_result != EXECUTION_SUCCESS as libc::c_int
            {
                last_command_exit_value = exec_result;
                line_number = line_number_for_err_trap;
                run_error_trap();
                line_number = save_line_number;
            }

            if ignore_return == 0 && invert == 0 
                && (posixly_correct != 0 && interactive == 0 
                    && special_builtin_failed != 0
                    || exit_immediately_on_error != 0 && pipe_in == NO_PIPE
                        && pipe_out == NO_PIPE
                        && exec_result != EXECUTION_SUCCESS as libc::c_int)
            {
                last_command_exit_value = exec_result;
                run_pending_traps();
                if exit_immediately_on_error != 0
                    && signal_is_trapped(0 ) != 0
                    && unwind_protect_tag_on_stack(
                        b"saved-redirects\0" as *const u8 as *const libc::c_char,
                    ) != 0
                {
                    run_unwind_frame(b"saved-redirects\0" as *const u8 as *mut libc::c_char);
                }
                jump_to_top_level(4 as libc::c_int);
            }
        }
        command_type_cm_for => {
            if ignore_return != 0 {
                (*(*command).value.For).flags |= CMD_IGNORE_RETURN as libc::c_int;
            }
            exec_result = execute_for_command((*command).value.For);
        }
        command_type_cm_arith_for => {
            if ignore_return != 0 {
                (*(*command).value.ArithFor).flags |= CMD_IGNORE_RETURN as libc::c_int;
            }
            exec_result = execute_arith_for_command((*command).value.ArithFor);
        }
        command_type_cm_select => {
            if ignore_return != 0 {
                (*(*command).value.Select).flags |= CMD_IGNORE_RETURN as libc::c_int;
            }
            exec_result = execute_select_command((*command).value.Select);
        }
        command_type_cm_case => {
            if ignore_return != 0 {
                (*(*command).value.Case).flags |= CMD_IGNORE_RETURN as libc::c_int;
            }
            exec_result = execute_case_command((*command).value.Case);
        }
        command_type_cm_while => {
            if ignore_return != 0 {
                (*(*command).value.While).flags |= CMD_IGNORE_RETURN as libc::c_int;
            }
            exec_result = execute_while_command((*command).value.While);
        }
        command_type_cm_until => {
            if ignore_return != 0 {
                (*(*command).value.While).flags |= CMD_IGNORE_RETURN as libc::c_int;
            }
            exec_result = execute_until_command((*command).value.While);
        }
        command_type_cm_if => {
            if ignore_return != 0 {
                (*(*command).value.If).flags |= CMD_IGNORE_RETURN as libc::c_int;
            }
            exec_result = execute_if_command((*command).value.If);
        }
        command_type_cm_group => {
            if asynchronous != 0 {
                (*command).flags |= CMD_FORCE_SUBSHELL as libc::c_int;
                exec_result = execute_command_internal(
                    command,
                    1 ,
                    pipe_in,
                    pipe_out,
                    fds_to_close,
                );
            } else {
                if ignore_return != 0 && !((*(*command).value.Group).command).is_null() {
                    (*(*(*command).value.Group).command).flags |= CMD_IGNORE_RETURN as libc::c_int;
                }
                exec_result = execute_command_internal(
                    (*(*command).value.Group).command,
                    asynchronous,
                    pipe_in,
                    pipe_out,
                    fds_to_close,
                );
            }
        }
        command_type_cm_connection => {
            exec_result = execute_connection(
                command,
                asynchronous,
                pipe_in,
                pipe_out,
                fds_to_close,
            );
            if asynchronous != 0 {
                invert = 0;
            }
        }
        command_type_cm_arith | command_type_cm_cond | command_type_cm_function_def => {
            was_error_trap = (signal_is_trapped(ERROR_TRAP as libc::c_int) != 0
                && signal_is_ignored(ERROR_TRAP as libc::c_int) == 0 ) as libc::c_int;
            if ignore_return != 0
                && (*command).type_== command_type_cm_arith 
            {
                (*(*command).value.Arith).flags |= CMD_IGNORE_RETURN as libc::c_int;
            }
            if ignore_return != 0
                && (*command).type_ == command_type_cm_cond
            {
                (*(*command).value.Cond).flags |= CMD_IGNORE_RETURN as libc::c_int;
            }
            line_number_for_err_trap = save_line_number;
            line_number_for_err_trap = line_number;

            if (*command).type_== command_type_cm_arith 
            {
                exec_result = execute_arith_command((*command).value.Arith);
            } else if (*command).type_ == command_type_cm_cond 
                {
                exec_result = execute_cond_command((*command).value.Cond);
            } else if (*command).type_ == command_type_cm_function_def 
                {
                exec_result = execute_intern_function(
                    (*(*command).value.Function_def).name,
                    (*command).value.Function_def,
                );
            }
            line_number = save_line_number;
            if was_error_trap != 0 && ignore_return == 0 
                && invert == 0 && exec_result != EXECUTION_SUCCESS as libc::c_int
            {
                last_command_exit_value = exec_result;
                save_line_number = line_number;
                line_number = line_number_for_err_trap;
                run_error_trap();
                line_number = save_line_number;
            }
            if ignore_return == 0 && invert == 0  
                && exit_immediately_on_error != 0 && exec_result != EXECUTION_SUCCESS as libc::c_int
            {
                last_command_exit_value = exec_result;
                run_pending_traps();
                jump_to_top_level(ERREXIT as libc::c_int);
            }
        }
        _ => {
            command_error(
                b"execute_command\0" as *const u8 as *const libc::c_char,
                CMDERR_BADTYPE as libc::c_int,
                (*command).type_ as libc::c_int,
                0,
            );
        }
    }
    if !my_undo_list.is_null() {
        cleanup_redirects(my_undo_list);
    }
    if !exec_undo_list.is_null() {
        dispose_redirects(exec_undo_list);
    }
    if !my_undo_list.is_null() || !exec_undo_list.is_null() {
        discard_unwind_frame(b"loop_redirections\0" as *const u8 as *mut libc::c_char);
    }

    if saved_fifo != 0 {
        nfifo = num_fifos();
        if nfifo > ofifo {
            close_new_fifos(ofifo_list as *mut libc::c_void, osize);
        }
        free(ofifo_list as *mut c_void);
        discard_unwind_frame(b"internal_fifos\0" as *const u8 as *mut libc::c_char);
    }

    if invert != 0 {
        exec_result = if exec_result == EXECUTION_SUCCESS as libc::c_int {
            EXECUTION_FAILURE as libc::c_int
        } else {
            EXECUTION_SUCCESS as libc::c_int
        };
    }
    match (*command).type_ {
        command_type_cm_arith | command_type_cm_cond => {
            set_pipestatus_from_exit(exec_result);
        }
        _ => {}
    }
    last_command_exit_value = exec_result;
    run_pending_traps();
    currently_executing_command = 0 as *mut COMMAND;
    return last_command_exit_value;
}

static mut precs: [libc::c_int; 4] = [
    0 as libc::c_int,
    100 as libc::c_int,
    10 as libc::c_int,
    1 as libc::c_int,
];

unsafe extern "C" fn mkfmt(
    mut buf: *mut libc::c_char,
    mut prec: libc::c_int,
    mut lng: libc::c_int,
    mut sec: time_t,
    mut sec_fraction: libc::c_int,
) -> libc::c_int {
    let mut min: time_t = 0;
    let mut abuf: [libc::c_char; 22] = [0; 22];
    let mut ind: libc::c_int = 0;
    let mut aind: libc::c_int = 0;

    ind = 0;
    abuf[((size_of::<[libc::c_char; 22]>() ) - 1)] = '\u{0}' as libc::c_char;
    
    if lng != 0 {
        min = sec / 60 as libc::c_long;
        sec %= 60 as libc::c_long;
        aind = (size_of::<[libc::c_char; 22]>() -2 ) as libc::c_int;
        loop {  //有可能aind的值不正确
            abuf[aind as usize] = (min % 10 + '0' as libc::c_long) as libc::c_char;
            aind = aind - 1;
            min /= 10 as libc::c_long;
            if !(min != 0) {
                break;
            }
        }
        aind += 1;
        while abuf[aind as usize] != 0 { //有可能ind，aind的值不正确
            *buf.offset(ind as isize) = abuf[aind as usize];
            aind = aind + 1;
            ind = ind + 1;
        }
        *buf.offset(ind as isize) = 'm' as libc::c_char;
        ind = ind + 1;
    }

    aind = ( size_of::<[libc::c_char; 22]>() -2 ) as libc::c_int;
    loop {        
        abuf[aind as usize] = ((sec % 10 )+ '0' as libc::c_long) as libc::c_char;
        aind = aind - 1;
        sec /= 10 ;
        if !(sec != 0) {
            break;
        }
    }
    aind += 1;
    while abuf[aind as usize] != 0 {
        *buf.offset(ind as isize) = abuf[aind as usize];
        aind = aind + 1;
        ind = ind + 1;
    }

    if prec != 0 {
        *buf.offset(ind as isize) = locale_decpoint() as libc::c_char;
        ind = ind + 1;
        aind = 1 ;
        while aind <= prec {            
            *buf.offset(ind as isize) = (sec_fraction / precs[aind as usize] + '0' as i32) as libc::c_char;
            ind = ind + 1;
            sec_fraction %= precs[aind as usize];
            aind += 1;
        }
    }

    if lng != 0 {
        *buf.offset(ind as isize) = 's' as libc::c_char;
        ind = ind + 1;
    }
    *buf.offset(ind as isize) = '\u{0}' as libc::c_char;

    return ind;
}

unsafe extern "C" fn print_formatted_time(
    mut fp: *mut FILE,
    mut format: *mut libc::c_char,
    mut rs: time_t,
    mut rsf: libc::c_int,
    mut us: time_t,
    mut usf: libc::c_int,
    mut ss: time_t,
    mut ssf: libc::c_int,
    mut cpu: libc::c_int,
) {
    let mut prec: libc::c_int = 0;
    let mut lng: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ts: [libc::c_char; 30] = [0; 30];
    let mut sum: time_t = 0;
    let mut sum_frac: libc::c_int = 0;
    let mut sindex: libc::c_int = 0;
    let mut ssize: libc::c_int = 0;

    len = strlen(format) as libc::c_int;
    ssize = (len + 64) - (len % 64);
    str = malloc(ssize as usize) as *mut c_char;
    sindex = 0 ;

    s = format;
    while *s != 0 {
        if *s as libc::c_int != '%' as i32
            || *s.offset(1 as isize) as libc::c_int == '\u{0}' as i32
        {
            RESIZE_MALLOCED_BUFFER!(str, sindex, 1, ssize, 64);
            *str.offset(sindex as isize) = *s;
            sindex = sindex + 1;
        } else if *s.offset(1 as isize) as libc::c_int == '%' as i32 {
            s = s.offset(1);
            RESIZE_MALLOCED_BUFFER!(str, sindex, 1, ssize, 64);
            *str.offset(sindex as isize) = *s;
            sindex = sindex + 1;
        } else if *s.offset(1 as isize) as libc::c_int == 'P' as i32 {
            s = s.offset(1);
            sum = (cpu / 100 ) as time_t;
            sum_frac = cpu % 100  * 10;
            len = mkfmt(
                ts.as_mut_ptr(),
                2  ,
                0 ,
                sum,
                sum_frac,
            );
            RESIZE_MALLOCED_BUFFER!(str, sindex, len, ssize, 64);
            strcpy(str.offset(sindex as isize), ts.as_mut_ptr());
            sindex += len;
        } else {
            prec = 3 ;
            lng = 0 ;
            s = s.offset(1);
            if DIGIT(*s) {
                prec = *s as libc::c_int - '0' as i32;
                s = s.offset(1);
                if prec > 3 {
                    prec = 3 ;
                }
            }
            if *s as libc::c_int == 'l' as i32 {
                lng = 1 ;
                s = s.offset(1);
            }
            if *s as libc::c_int == 'R' as i32 || *s as libc::c_int == 'E' as i32 {
                len = mkfmt(ts.as_mut_ptr(), prec, lng, rs, rsf);
            } else if *s as libc::c_int == 'U' as i32 {
                len = mkfmt(ts.as_mut_ptr(), prec, lng, us, usf);
            } else if *s as libc::c_int == 'S' as i32 {
                len = mkfmt(ts.as_mut_ptr(), prec, lng, ss, ssf);
            } else {
                internal_error(
                        b"TIMEFORMAT: `%c': invalid format character\0" as *const u8 as *mut c_char,
                     *s as libc::c_int,
                );
                free(str as *mut c_void);
                return;
            }

            RESIZE_MALLOCED_BUFFER!(str, sindex, len, ssize, 64);
            strcpy(str.offset(sindex as isize), ts.as_mut_ptr());
            sindex += len;
        }
        s = s.offset(1);
    }
    *str.offset(sindex as isize) = '\u{0}' as libc::c_char;
    fprintf(fp, b"%s\n\0" as *const u8 as *const libc::c_char, str);
    fflush(fp);
    free(str as *mut c_void);
}

unsafe extern "C" fn time_command(
    mut command: *mut COMMAND,
    mut asynchronous: libc::c_int,
    mut pipe_in: libc::c_int,
    mut pipe_out: libc::c_int,
    mut fds_to_close: *mut fd_bitmap,
) -> libc::c_int {
    let mut rv: libc::c_int = 0;
    let mut posix_time: libc::c_int = 0;
    let mut old_flags: libc::c_int = 0;
    let mut nullcmd: libc::c_int = 0;
    let mut code: libc::c_int = 0;
    let mut rs: time_t = 0;
    let mut us: time_t = 0;
    let mut ss: time_t = 0;
    let mut rsf: libc::c_int = 0;
    let mut usf: libc::c_int = 0;
    let mut ssf: libc::c_int = 0;
    let mut cpu: libc::c_int = 0;
    let mut time_format: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut save_top_level: sigjmp_buf = [__jmp_buf_tag {
        __jmpbuf: [0; 8],
        __mask_was_saved: 0,
        __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1];
    let mut real: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut user: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut sys: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut before: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut after: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut dtz: timezone = timezone {
        tz_minuteswest: 0,
        tz_dsttime: 0,
    };
    let mut selfb: rusage = rusage {
        ru_utime: timeval { tv_sec: 0, tv_usec: 0 },
        ru_stime: timeval { tv_sec: 0, tv_usec: 0 },
        __bindgen_anon_1: rusage__bindgen_ty_1 { ru_maxrss: 0 },
        __bindgen_anon_2: rusage__bindgen_ty_2 { ru_ixrss: 0 },
        __bindgen_anon_3: rusage__bindgen_ty_3 { ru_idrss: 0 },
        __bindgen_anon_4: rusage__bindgen_ty_4 { ru_isrss: 0 },
        __bindgen_anon_5: rusage__bindgen_ty_5 { ru_minflt: 0 },
        __bindgen_anon_6: rusage__bindgen_ty_6 { ru_majflt: 0 },
        __bindgen_anon_7: rusage__bindgen_ty_7 { ru_nswap: 0 },
        __bindgen_anon_8: rusage__bindgen_ty_8{ ru_inblock: 0 },
        __bindgen_anon_9: rusage__bindgen_ty_9 { ru_oublock: 0 },
        __bindgen_anon_10: rusage__bindgen_ty_10 { ru_msgsnd: 0 },
        __bindgen_anon_11: rusage__bindgen_ty_11 { ru_msgrcv: 0 },
        __bindgen_anon_12: rusage__bindgen_ty_12 { ru_nsignals: 0 },
        __bindgen_anon_13: rusage__bindgen_ty_13 { ru_nvcsw: 0 },
        __bindgen_anon_14: rusage__bindgen_ty_14 { ru_nivcsw: 0 },
    };
    let mut selfa: rusage = rusage {
        ru_utime: timeval { tv_sec: 0, tv_usec: 0 },
        ru_stime: timeval { tv_sec: 0, tv_usec: 0 },
        __bindgen_anon_1: rusage__bindgen_ty_1 { ru_maxrss: 0 },
        __bindgen_anon_2: rusage__bindgen_ty_2 { ru_ixrss: 0 },
        __bindgen_anon_3: rusage__bindgen_ty_3 { ru_idrss: 0 },
        __bindgen_anon_4: rusage__bindgen_ty_4 { ru_isrss: 0 },
        __bindgen_anon_5: rusage__bindgen_ty_5 { ru_minflt: 0 },
        __bindgen_anon_6: rusage__bindgen_ty_6 { ru_majflt: 0 },
        __bindgen_anon_7: rusage__bindgen_ty_7 { ru_nswap: 0 },
        __bindgen_anon_8: rusage__bindgen_ty_8{ ru_inblock: 0 },
        __bindgen_anon_9: rusage__bindgen_ty_9 { ru_oublock: 0 },
        __bindgen_anon_10: rusage__bindgen_ty_10 { ru_msgsnd: 0 },
        __bindgen_anon_11: rusage__bindgen_ty_11 { ru_msgrcv: 0 },
        __bindgen_anon_12: rusage__bindgen_ty_12 { ru_nsignals: 0 },
        __bindgen_anon_13: rusage__bindgen_ty_13 { ru_nvcsw: 0 },
        __bindgen_anon_14: rusage__bindgen_ty_14 { ru_nivcsw: 0 },
    };
    let mut kidsb: rusage = rusage {
        ru_utime: timeval { tv_sec: 0, tv_usec: 0 },
        ru_stime: timeval { tv_sec: 0, tv_usec: 0 },
        __bindgen_anon_1: rusage__bindgen_ty_1 { ru_maxrss: 0 },
        __bindgen_anon_2: rusage__bindgen_ty_2 { ru_ixrss: 0 },
        __bindgen_anon_3: rusage__bindgen_ty_3 { ru_idrss: 0 },
        __bindgen_anon_4: rusage__bindgen_ty_4 { ru_isrss: 0 },
        __bindgen_anon_5: rusage__bindgen_ty_5 { ru_minflt: 0 },
        __bindgen_anon_6: rusage__bindgen_ty_6 { ru_majflt: 0 },
        __bindgen_anon_7: rusage__bindgen_ty_7 { ru_nswap: 0 },
        __bindgen_anon_8: rusage__bindgen_ty_8{ ru_inblock: 0 },
        __bindgen_anon_9: rusage__bindgen_ty_9 { ru_oublock: 0 },
        __bindgen_anon_10: rusage__bindgen_ty_10 { ru_msgsnd: 0 },
        __bindgen_anon_11: rusage__bindgen_ty_11 { ru_msgrcv: 0 },
        __bindgen_anon_12: rusage__bindgen_ty_12 { ru_nsignals: 0 },
        __bindgen_anon_13: rusage__bindgen_ty_13 { ru_nvcsw: 0 },
        __bindgen_anon_14: rusage__bindgen_ty_14 { ru_nivcsw: 0 },
    };
    let mut kidsa: rusage = rusage {
        ru_utime: timeval { tv_sec: 0, tv_usec: 0 },
        ru_stime: timeval { tv_sec: 0, tv_usec: 0 },
        __bindgen_anon_1: rusage__bindgen_ty_1 { ru_maxrss: 0 },
        __bindgen_anon_2: rusage__bindgen_ty_2 { ru_ixrss: 0 },
        __bindgen_anon_3: rusage__bindgen_ty_3 { ru_idrss: 0 },
        __bindgen_anon_4: rusage__bindgen_ty_4 { ru_isrss: 0 },
        __bindgen_anon_5: rusage__bindgen_ty_5 { ru_minflt: 0 },
        __bindgen_anon_6: rusage__bindgen_ty_6 { ru_majflt: 0 },
        __bindgen_anon_7: rusage__bindgen_ty_7 { ru_nswap: 0 },
        __bindgen_anon_8: rusage__bindgen_ty_8{ ru_inblock: 0 },
        __bindgen_anon_9: rusage__bindgen_ty_9 { ru_oublock: 0 },
        __bindgen_anon_10: rusage__bindgen_ty_10 { ru_msgsnd: 0 },
        __bindgen_anon_11: rusage__bindgen_ty_11 { ru_msgrcv: 0 },
        __bindgen_anon_12: rusage__bindgen_ty_12 { ru_nsignals: 0 },
        __bindgen_anon_13: rusage__bindgen_ty_13 { ru_nvcsw: 0 },
        __bindgen_anon_14: rusage__bindgen_ty_14 { ru_nivcsw: 0 },
    };

    gettimeofday(&mut before, &mut dtz);
    getrusage(RUSAGE_SELF, &mut selfb);
    getrusage(RUSAGE_CHILDREN, &mut kidsb);

    posix_time = (!command.is_null() && (*command).flags & CMD_TIME_POSIX as libc::c_int != 0) as libc::c_int;
    nullcmd = (command.is_null()
            || (*command).type_ == command_type_cm_simple
            && ((*(*command).value.Simple).words).is_null()
            && ((*(*command).value.Simple).redirects).is_null()) as libc::c_int;
    if posixly_correct != 0 && nullcmd != 0 {
        kidsb.ru_stime.tv_sec = 0 as __time_t;
        selfb.ru_stime.tv_sec = kidsb.ru_stime.tv_sec;
        kidsb.ru_utime.tv_sec = selfb.ru_stime.tv_sec;
        selfb.ru_utime.tv_sec = kidsb.ru_utime.tv_sec;
        kidsb.ru_stime.tv_usec = 0 as __suseconds_t;
        selfb.ru_stime.tv_usec = kidsb.ru_stime.tv_usec;
        kidsb.ru_utime.tv_usec = selfb.ru_stime.tv_usec;
        selfb.ru_utime.tv_usec = kidsb.ru_utime.tv_usec;
        before = shellstart;
    }

    old_flags = (*command).flags;
    COPY_PROCENV!(top_level, save_top_level);
    (*command).flags &= !(CMD_TIME_PIPELINE as libc::c_int | CMD_TIME_POSIX as libc::c_int);
    code = setjmp_nosigs!(top_level);
    if code == NOT_JUMPED as libc::c_int {
        rv = execute_command_internal(
            command,
            asynchronous,
            pipe_in,
            pipe_out,
            fds_to_close,
        );
        (*command).flags = old_flags;
    }
    COPY_PROCENV!(save_top_level, top_level);

    ss = 0 as time_t;
    us = ss;
    rs = us;
    cpu = 0;
    ssf = cpu;
    usf = ssf;
    rsf = usf;

    gettimeofday(&mut after, &mut dtz);

    getrusage(RUSAGE_SELF, &mut selfa);
    getrusage(RUSAGE_CHILDREN, &mut kidsa);

    difftimeval(&mut real, &mut before, &mut after);
    timeval_to_secs(&mut real, &mut rs, &mut rsf);

    addtimeval(
        &mut user,
        difftimeval(&mut after, &mut selfb.ru_utime, &mut selfa.ru_utime),
        difftimeval(&mut before, &mut kidsb.ru_utime, &mut kidsa.ru_utime),
    );
    timeval_to_secs(&mut user, &mut us, &mut usf);

    addtimeval(
        &mut sys,
        difftimeval(&mut after, &mut selfb.ru_stime, &mut selfa.ru_stime),
        difftimeval(&mut before, &mut kidsb.ru_stime, &mut kidsa.ru_stime),
    );
    timeval_to_secs(&mut sys, &mut ss, &mut ssf);

    cpu = timeval_to_cpu(&mut real, &mut user, &mut sys);

    if posix_time != 0 {
        time_format = POSIX_TIMEFORMAT!();
    } else {
        time_format = get_string_value(b"TIMEFORMAT\0" as *const u8 as *const libc::c_char);
        if time_format.is_null() {
            if posixly_correct != 0 && nullcmd != 0 {
                time_format = b"user\t%2lU\nsys\t%2lS\0" as *const u8 as *mut libc::c_char;
            } else {
                time_format = BASH_TIMEFORMAT!();
            }
        }
    }
    if !time_format.is_null() && *time_format as libc::c_int != 0 {
        print_formatted_time(stderr, time_format, rs, rsf, us, usf, ss, ssf, cpu);
    }
    if code != 0 {
        siglongjmp(top_level.as_mut_ptr(), code);
    }
    return rv;
}

unsafe extern "C" fn execute_in_subshell(
    mut command: *mut COMMAND,
    mut asynchronous: libc::c_int,
    mut pipe_in: libc::c_int,
    mut pipe_out: libc::c_int,
    mut fds_to_close: *mut fd_bitmap,
) -> libc::c_int {
    let mut user_subshell: libc::c_int = 0;
    let mut user_coproc: libc::c_int = 0;
    let mut invert: libc::c_int = 0;
    let mut return_code: libc::c_int = 0;
    let mut function_value: libc::c_int = 0;
    let mut should_redir_stdin: libc::c_int = 0;
    let mut ois: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    let mut tcom: *mut COMMAND = 0 as *mut COMMAND;

    subshell_level += 1;
    should_redir_stdin = (asynchronous != 0
        && (*command).flags & CMD_STDIN_REDIR as libc::c_int != 0 
        && pipe_in == NO_PIPE
        && stdin_redirects((*command).redirects) == 0 ) as libc::c_int;

    invert = (((*command).flags & CMD_INVERT_RETURN as c_int) != 0 )as c_int; 
    user_subshell = ((*command).type_ == command_type_cm_subshell || ((*command).flags & CMD_WANT_SUBSHELL as c_int) != 0) as c_int;
    user_coproc = ((*command).type_ == command_type_cm_coproc)as c_int;
    (*command).flags &= !(CMD_FORCE_SUBSHELL as libc::c_int | CMD_WANT_SUBSHELL as libc::c_int | CMD_INVERT_RETURN as libc::c_int);
   
    if asynchronous != 0 {
        original_pgrp = -1;

        ois = interactive_shell;
        interactive_shell = 0;

        if ois != interactive_shell {
            expand_aliases = 0 ;
        }
    }

    interactive = 0 ;
    login_shell = interactive;

    if shell_compatibility_level > 44 {
        loop_level = 0 ;
    }

    if user_subshell != 0 {
        subshell_environment = SUBSHELL_PAREN as libc::c_int;
        if asynchronous != 0 {
            subshell_environment |= SUBSHELL_ASYNC as libc::c_int;
        }
    } else {
        subshell_environment = 0;
        if asynchronous != 0 {
            subshell_environment |= SUBSHELL_ASYNC as libc::c_int;
        }
        if pipe_in != NO_PIPE || pipe_out != NO_PIPE {
            subshell_environment |= SUBSHELL_PIPE as libc::c_int;
        }
        if user_coproc != 0 {
            subshell_environment |= SUBSHELL_COPROC as libc::c_int;
        }
    }
    QUIT!();
    CHECK_TERMSIG!();   

    reset_terminating_signals();
    clear_pending_traps();
    reset_signal_handlers();
    subshell_environment |= SUBSHELL_RESETTRAP as libc::c_int;

    if running_trap > 0 {
        run_trap_cleanup(running_trap - 1 as libc::c_int);
        running_trap = 0 ;
    }

    if asynchronous != 0 {
        setup_async_signals();
        asynchronous = 0;
    } else {
        set_sigint_handler();
    }

    set_sigchld_handler();

    without_job_control();

    if !fds_to_close.is_null() {
        close_fd_bitmap(fds_to_close);
    }

    do_piping(pipe_in, pipe_out);

    coproc_closeall();

    clear_fifo_list();

    if user_subshell != 0 {
        stdin_redir = (stdin_redirects((*command).redirects) != 0
            || pipe_in != NO_PIPE) as libc::c_int;
    } else if shell_control_structure((*command).type_ as libc::c_uint) != 0
            && pipe_in != NO_PIPE
        {
        stdin_redir = 1  ;
    }

    if should_redir_stdin != 0 && stdin_redir == 0 {
        async_redirect_stdin();
    }

    default_buffered_input = -1;

    if !((*command).redirects).is_null() {
        if do_redirections((*command).redirects, RX_ACTIVE as libc::c_int) != 0  
        {
            exit(if invert != 0 { EXECUTION_SUCCESS as libc::c_int } else { EXECUTION_FAILURE as libc::c_int });
        }
        dispose_redirects((*command).redirects);
        (*command).redirects = 0 as *mut REDIRECT;
    }

    if (*command).type_  == command_type_cm_subshell {
        tcom = (*(*command).value.Subshell).command as *mut COMMAND;
    } else if user_coproc != 0 {
        tcom = (*(*command).value.Coproc).command as *mut COMMAND;
    } else {
        tcom = command as *mut COMMAND;
    }

    if (*command).flags & CMD_TIME_PIPELINE as libc::c_int != 0 {
        (*tcom).flags = CMD_TIME_PIPELINE as c_int;
    }
    if (*command).flags & CMD_TIME_POSIX as libc::c_int != 0 {
        (*tcom).flags = CMD_TIME_POSIX as c_int;
    }

    if (*command).flags & CMD_IGNORE_RETURN as libc::c_int != 0 && tcom != command as *mut COMMAND {
        (*tcom).flags = CMD_IGNORE_RETURN as c_int;
    }

    if (user_subshell != 0 || user_coproc != 0)
        && ((*tcom).type_  == command_type_cm_simple || (*tcom).type_ == command_type_cm_subshell )
        && (*tcom).flags & CMD_TIME_PIPELINE as libc::c_int == 0 
        && (*tcom).flags & CMD_INVERT_RETURN as libc::c_int == 0 
    {
        (*tcom).flags = CMD_NO_FORK as c_int;
        if (*tcom).type_  == command_type_cm_simple  {
            (*(*tcom).value.Simple).flags |= CMD_NO_FORK as libc::c_int;
        }
    }

    invert = ((*tcom).flags & CMD_INVERT_RETURN as c_int != 0 ) as c_int;
    (*tcom).flags &= !CMD_INVERT_RETURN as c_int;

    result = setjmp_nosigs!(top_level);

    function_value = 0 ;
    if return_catch_flag != 0 {
        function_value = setjmp_nosigs!(return_catch);
    }

    if result == EXITPROG as libc::c_int {
        invert = 0;
        return_code = last_command_exit_value;
    } else if result != 0 {
        return_code = if last_command_exit_value == EXECUTION_SUCCESS as libc::c_int {
            EXECUTION_FAILURE as libc::c_int
        } else {
            last_command_exit_value
        };
    } else if function_value != 0 {
        return_code = return_catch_value;
    } else {
        return_code = execute_command_internal(
            tcom as *mut COMMAND,
            asynchronous,
            NO_PIPE,
            NO_PIPE,
            fds_to_close,
        );
    }
    if invert != 0 {
        return_code = if return_code == EXECUTION_SUCCESS as libc::c_int {
            EXECUTION_FAILURE as libc::c_int
        } else {
            EXECUTION_SUCCESS as libc::c_int
        };
    }
    if user_subshell != 0 && signal_is_trapped(0 ) != 0 {
        last_command_exit_value = return_code;
        return_code = run_exit_trap();
    }
    return return_code;
}

#[no_mangle]
pub static mut sh_coproc: Coproc = {
    let mut init = coproc {
        c_name: 0 as *const libc::c_char as *mut libc::c_char,
        c_pid: -(1 as libc::c_int),
        c_rfd: -(1 as libc::c_int),
        c_wfd: -(1 as libc::c_int),
        c_rsave: 0 as libc::c_int,
        c_wsave: 0 as libc::c_int,
        c_flags: 0 as libc::c_int,
        c_status: 0 as libc::c_int,
        c_lock: 0 as libc::c_int,
    };
    init
};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpelement {
    pub next: *mut cpelement,
    pub coproc: *mut coproc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cplist {
    pub head: *mut cpelement,
    pub tail: *mut cpelement,
    pub ncoproc: libc::c_int,
    pub lock: libc::c_int,
}
pub type cplist_t = cplist;
#[no_mangle]
pub static mut coproc_list: cplist_t = {
    let mut init = cplist {
        head: 0 as *const cpelement as *mut cpelement,
        tail: 0 as *const cpelement as *mut cpelement,
        ncoproc: 0 as libc::c_int,
        lock: 0,
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn getcoprocbypid(mut pid: pid_t) -> *mut coproc {
    return if pid == sh_coproc.c_pid { &mut sh_coproc } else { 0 as *mut Coproc };
}

pub unsafe extern "C" fn cpe_alloc(cp:*mut Coproc)-> *mut cpelement
{
    let mut cpe:*mut cpelement;

    cpe = malloc(size_of::<cpelement>() as usize) as *mut cpelement;
    (*cpe).coproc = cp;
    (*cpe).next = 0 as *mut cpelement;

    return cpe;
}

pub unsafe extern "C" fn cpe_dispose(cpe:*mut cpelement)
{
    free(cpe as *mut c_void);
}

pub unsafe extern "C" fn cpe_add(cp:*mut Coproc)-> *mut cpelement
{
    let mut cpe:*mut cpelement;

    cpe = cpe_alloc(cp);

    if coproc_list.head == 0 as *mut cpelement{
        coproc_list.tail = cpe; 
        coproc_list.head = cpe;
        coproc_list.ncoproc = 0
    } else {
        (*coproc_list.tail).next = cpe;
        coproc_list.tail = cpe;
    }
    coproc_list.ncoproc += 1;

    return cpe;
}

pub unsafe extern "C" fn cpl_delete(pid:pid_t)-> *mut cpelement
{
    let mut prev:*mut cpelement;
    let mut p:*mut cpelement;

    p = coproc_list.head;
    prev = coproc_list.head;
    while !p.is_null(){
        if (*(*p).coproc).c_pid == pid {
            (*prev).next = (*p).next;
            break;
        }
        prev = p;
        p = (*p).next;
    }

    if p==0 as *mut cpelement{
        return 0 as *mut cpelement;
    }

    if p == coproc_list.head {
        coproc_list.head = (*coproc_list.head).next;
    } else if p == coproc_list.tail{
        coproc_list.tail = prev;
    }

    coproc_list.ncoproc -= 1;
    if coproc_list.ncoproc == 0{
        coproc_list.head = 0 as *mut cpelement;
        coproc_list.tail = 0 as *mut cpelement;
    }else if coproc_list.ncoproc == 1{
        coproc_list.tail = coproc_list.head;
    }
    
    return p;
}

pub unsafe extern "C" fn cpl_reap(){
    let mut p:*mut cpelement;
    let mut next:*mut cpelement;
    let mut nh:*mut cpelement;
    let mut nt:*mut cpelement;

    nh = 0 as *mut cpelement;
    nt = 0 as *mut cpelement;
    next = 0 as *mut cpelement;

    p = coproc_list.head;
    while !p.is_null() {
        next = (*p).next;

        if ((*(*p).coproc).c_flags & COPROC_DEAD as c_int) != 0 {
            coproc_list.ncoproc -= 0;
            coproc_dispose((*p).coproc);
            cpe_dispose(p);
        } else if nh.is_null(){
            nh = p;
            nt = p;
        }
        p = next;
    }

    if coproc_list.ncoproc == 0 {
        coproc_list.head = 0 as *mut cpelement;
        coproc_list.tail = 0 as *mut cpelement;
    } else {
        if !nt.is_null(){
            (*nt).next = 0 as *mut cpelement;
        }
        
        coproc_list.head = nh;
        coproc_list.tail = nt;
        if coproc_list.ncoproc == 1{
            coproc_list.tail = coproc_list.head; /* just to make sure */
        }
    }
}
 


pub unsafe extern "C" fn cpl_flush()
{
    let mut cpe:*mut cpelement;
    let mut p:*mut cpelement;

    cpe = coproc_list.head;
    while !cpe.is_null(){
        p = cpe;
        cpe = (*cpe).next;

        coproc_dispose((*p).coproc);
        cpe_dispose(p);
    }

    coproc_list.head = 0 as *mut cpelement;
    coproc_list.tail = 0 as *mut cpelement;
    coproc_list.ncoproc = 0;
}

pub unsafe extern "C" fn cpl_closeall()
{
    let mut cpe:*mut cpelement;

    cpe = coproc_list.head;
    while !cpe.is_null() {
        coproc_close((*cpe).coproc);

        cpe = (*cpe).next;
    }
}

pub unsafe extern "C" fn cpl_fdchk(fd:c_int)
{
    let mut cpe:*mut cpelement;

    cpe = coproc_list.head;
    while !cpe.is_null() {
        coproc_checkfd((*cpe).coproc, fd);

        cpe = (*cpe).next;
    }
}

pub unsafe extern "C" fn cpl_search(pid:pid_t)-> *mut cpelement
{
    let mut cpe:*mut cpelement;

    cpe = coproc_list.head;
    while !cpe.is_null() {
        if (*(*cpe).coproc).c_pid == pid{
            return cpe;
        }
        cpe = (*cpe).next;
    }

    return 0 as *mut cpelement;
}

pub unsafe extern "C" fn cpl_searchbyname(name: *mut c_char)-> *mut cpelement
{
    let mut cp:*mut cpelement;

    cp = coproc_list.head;
    while !cp.is_null() {
        if STREQ!((*(*cp).coproc).c_name, name) {
            return cp;
        }
        cp = (*cp).next;
    }

    return 0 as *mut cpelement    
}


pub unsafe extern "C" fn cpl_firstactive()->pid_t
{
    let mut cpe:*mut cpelement;

    cpe = coproc_list.head;
    while !cpe.is_null() {
        if (*(*cpe).coproc).c_flags & COPROC_DEAD as c_int == 0 {
            return (*(*cpe).coproc).c_pid;
        }
        cpe = (*cpe).next;
    }

    return NO_PID!() as pid_t;
}

#[no_mangle]
pub unsafe extern "C" fn getcoprocbyname(mut name: *const libc::c_char) -> *mut coproc {
    return if !(sh_coproc.c_name).is_null()
        && STREQ!(sh_coproc.c_name, name)
    {
        &mut sh_coproc
    } else {
        0 as *mut Coproc
    };
}
#[no_mangle]
pub unsafe extern "C" fn coproc_init(mut cp: *mut coproc) {
    (*cp).c_name = 0 as *mut libc::c_char;
    (*cp).c_pid = NO_PID!();
    (*cp).c_wfd = -1;
    (*cp).c_rfd = -1;
    (*cp).c_wsave = -1;
    (*cp).c_rsave = -1;
    (*cp).c_lock = 0 ;
    (*cp).c_status = 0;
    (*cp).c_flags = 0;
}

#[no_mangle]
pub unsafe extern "C" fn coproc_alloc(
    mut name: *mut libc::c_char,
    mut pid: pid_t,
) -> *mut coproc {
    let mut cp: *mut coproc = 0 as *mut coproc;

    cp = &mut sh_coproc;

    coproc_init(cp);
    (*cp).c_lock = 2 ;

    (*cp).c_pid = pid;
    (*cp).c_name = savestring!(name);

    (*cp).c_lock = 0 ;
    return cp;
}

pub unsafe extern "C" fn coproc_free(cp:*mut coproc)
{
    free(cp as *mut c_void);
}

#[macro_export]
macro_rules! SIG_BLOCK {
    () => {
        0
    };
}

#[macro_export]
macro_rules! SIG_SETMASK {
    () => {
        2
    };
}

#[macro_export]
macro_rules! BLOCK_SIGNAL {
    ($sig:expr, $nvar:expr, $ovar:expr) => {
        sigemptyset(&mut $nvar);
        sigaddset(&mut $nvar, $sig as libc::c_int);
        sigemptyset(&mut $ovar);
        sigprocmask(SIG_BLOCK!(), &mut $nvar, &mut $ovar);
    };
}

#[macro_export]
macro_rules! UNBLOCK_SIGNAL {
    ($ovar:expr) => {
        sigprocmask(SIG_SETMASK!(), &mut $ovar, 0 as *mut sigset_t)
    };
}

#[no_mangle]
pub unsafe extern "C" fn coproc_dispose(mut cp: *mut coproc) {
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

    if cp.is_null() {
        return;
    }

    BLOCK_SIGNAL!(SIGCHLD, set, oset);
    (*cp).c_lock = 3 ;
    coproc_unsetvars(cp);
    FREE!((*cp).c_name);
    coproc_close(cp);

    coproc_init(cp);
    (*cp).c_lock = 0 ;
    UNBLOCK_SIGNAL!(oset);
}


#[no_mangle]
pub unsafe extern "C" fn coproc_flush() {
    coproc_dispose(&mut sh_coproc);
}

#[no_mangle]
pub unsafe extern "C" fn coproc_close(mut cp: *mut coproc) {
    if (*cp).c_rfd >= 0   {
        close((*cp).c_rfd);
        (*cp).c_rfd = -1;
    }
    if (*cp).c_wfd >= 0 {
        close((*cp).c_wfd);
        (*cp).c_wfd = -1;
    }
    let ref mut fresh27 = (*cp).c_wsave;
    (*cp).c_wsave = -1;
    (*cp).c_rsave = -1;
}

#[no_mangle]
pub unsafe extern "C" fn coproc_closeall() {
    coproc_close(&mut sh_coproc);
}

#[no_mangle]
pub unsafe extern "C" fn coproc_reap() {
    let mut cp: *mut coproc = 0 as *mut coproc;

    cp = &mut sh_coproc;
    if !cp.is_null() && (*cp).c_flags & COPROC_DEAD as libc::c_int != 0 {
        coproc_dispose(cp);
    }
}

#[no_mangle]
pub unsafe extern "C" fn coproc_rclose(mut cp: *mut coproc, mut fd: libc::c_int) {
    if (*cp).c_rfd >= 0 && (*cp).c_rfd == fd {
        close((*cp).c_rfd);
        (*cp).c_rfd = -1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn coproc_wclose(mut cp: *mut coproc, mut fd: libc::c_int) {
    if (*cp).c_wfd >= 0 && (*cp).c_wfd == fd {
        close((*cp).c_wfd);
        (*cp).c_wfd = -1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn coproc_checkfd(mut cp: *mut coproc, mut fd: libc::c_int) {
    let mut update: libc::c_int = 0;

    update = 0  ;
    if (*cp).c_rfd >= 0  && (*cp).c_rfd == fd {
        let ref mut fresh28 = (*cp).c_rfd;
        (*cp).c_rfd = -1;
        update = -1;
    }
    if (*cp).c_wfd >= 0 && (*cp).c_wfd == fd {
        let ref mut fresh29 = (*cp).c_wfd;
        (*cp).c_wfd = -1;
        update = -1;
    }
    if update != 0 {
        coproc_setvars(cp);
    }
}

#[no_mangle]
pub unsafe extern "C" fn coproc_fdchk(mut fd: libc::c_int) {
    coproc_checkfd(&mut sh_coproc, fd);
}

#[no_mangle]
pub unsafe extern "C" fn coproc_fdclose(mut cp: *mut coproc, mut fd: libc::c_int) {
    coproc_rclose(cp, fd);
    coproc_wclose(cp, fd);
    coproc_setvars(cp);
}

#[no_mangle]
pub unsafe extern "C" fn coproc_fdsave(mut cp: *mut coproc) {
    (*cp).c_rsave = (*cp).c_rfd;
    (*cp).c_wsave = (*cp).c_wfd;
}

#[no_mangle]
pub unsafe extern "C" fn coproc_fdrestore(mut cp: *mut coproc) {
    (*cp).c_rfd = (*cp).c_rsave;
    (*cp).c_wfd = (*cp).c_wsave;
}

unsafe extern "C" fn coproc_setstatus(mut cp: *mut coproc, mut status: libc::c_int) {
    (*cp).c_lock = 4 ;
    (*cp).c_status = status;
    (*cp).c_flags |= COPROC_DEAD as libc::c_int;
    (*cp).c_flags &= !(COPROC_RUNNING as libc::c_int);
    (*cp).c_lock = 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn coproc_pidchk(mut pid: pid_t, mut status: libc::c_int) {
    let mut cp: *mut coproc = 0 as *mut coproc;

    cp = getcoprocbypid(pid);
    if !cp.is_null() {
        coproc_setstatus(cp, status);
    }
}
#[no_mangle]
pub unsafe extern "C" fn coproc_active() -> pid_t {
    return if sh_coproc.c_flags & COPROC_DEAD as libc::c_int != 0 {
        NO_PID!()
    } else {
        sh_coproc.c_pid
    };
}

#[macro_export]
macro_rules! INVALID_NAMEREF_VALUE {
    () => {
        &mut nameref_invalid_value as *mut SHELL_VAR 
    };
}

#[macro_export]
macro_rules! att_nameref {
    () => {
        0x0000800
    };
}

#[macro_export]
macro_rules! nameref_p {
    ($var:expr) => {
        (*$var).attributes & att_nameref!()
    };
}

#[macro_export]
macro_rules! nameref_cell {
    ($var:expr) => {
        (*$var).value
    };
}
#[macro_export]
macro_rules! att_readonly {
    () => {
        0x0000002
    };
}

#[macro_export]
macro_rules! readonly_p {
    ($var:expr) => {
        (*$var).attributes & att_readonly!() 
    };
}

#[macro_export]
macro_rules! att_noassign {
    () => {
        0x0004000
    };
}

#[macro_export]
macro_rules! noassign_p {
    ($var:expr) => {
        (*$var).attributes & 0x4000 as libc::c_int
    };
}

#[macro_export]
macro_rules! att_array {
    () => {
        0x0000004
    };
}

#[macro_export]
macro_rules! array_p {
    ($var:expr) => {
        (*$var).attributes & att_array!()
    };
}
#[no_mangle]
pub unsafe extern "C" fn coproc_setvars(mut cp: *mut coproc) {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut namevar: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    let mut w: WordDesc = WordDesc {
        word: 0 as *mut libc::c_char,
        flags: 0,
    };
    let mut ind: arrayind_t = 0;

    if ((*cp).c_name).is_null() {
        return;
    }

    w.word = (*cp).c_name;
    w.flags = 0 ;
    if check_identifier(&mut w, 1 ) == 0 {
        return;
    }

    l = strlen((*cp).c_name) as libc::c_int;
    namevar = malloc((l+ 16) as usize) as *mut c_char;

    v = find_variable((*cp).c_name);

    if v.is_null() {
        v = find_variable_nameref_for_create((*cp).c_name, 1);
        if v == INVALID_NAMEREF_VALUE!()
        {
            free(namevar as *mut c_void);
            return;
        }
        if !v.is_null() && nameref_p!(v) != 0   {
            free((*cp).c_name as *mut c_void);
            let ref mut fresh30 = (*cp).c_name;
            (*cp).c_name = savestring!(nameref_cell!(v));
            v = make_new_array_variable((*cp).c_name);
        }
    }

    if !v.is_null() && (readonly_p!(v) != 0 || noassign_p!(v) != 0 )
    {
        if readonly_p!(v)  != 0 {
            err_readonly((*cp).c_name);
        }
        free(namevar as *mut c_void);
        return;
    }
    if v.is_null() {
        v = make_new_array_variable((*cp).c_name);
    }
    if array_p!(v) == 0 {
        v = convert_var_to_array(v);
    }

    t = itos((*cp).c_rfd as intmax_t);
    ind = 0 as arrayind_t;
    v = bind_array_variable((*cp).c_name, ind, t, 0 );
    free(t as *mut c_void);

    t = itos((*cp).c_wfd as intmax_t);
    ind = 1 as arrayind_t;
    v = bind_array_variable((*cp).c_name, ind, t, 0 as libc::c_int);
    free(t as *mut c_void);

    sprintf(namevar, b"%s_PID\0" as *const u8 as *const libc::c_char, (*cp).c_name);
    t = itos((*cp).c_pid as intmax_t);
    v = bind_variable(namevar, t, 0 as libc::c_int);
    free(t as *mut c_void);

    free(namevar as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn coproc_unsetvars(mut cp: *mut coproc) {
    let mut l: libc::c_int = 0;
    let mut namevar: *mut libc::c_char = 0 as *mut libc::c_char;

    if ((*cp).c_name).is_null() {
        return;
    }
    l = strlen((*cp).c_name) as libc::c_int;

    namevar = malloc((l + 16) as usize) as *mut c_char;
    
    sprintf(namevar, b"%s_PID\0" as *const u8 as *const libc::c_char, (*cp).c_name);
    unbind_variable_noref(namevar);
   
    check_unbind_variable((*cp).c_name);
   
    free(namevar as *mut c_void);
}
unsafe extern "C" fn execute_coproc(
    mut command: *mut COMMAND,
    mut pipe_in: libc::c_int,
    mut pipe_out: libc::c_int,
    mut fds_to_close: *mut fd_bitmap,
) -> libc::c_int {
    let mut rpipe: [libc::c_int; 2] = [0; 2];
    let mut wpipe: [libc::c_int; 2] = [0; 2];
    let mut estat: libc::c_int = 0;
    let mut invert: libc::c_int = 0;
    let mut coproc_pid: pid_t = 0;
    let mut cp: *mut Coproc = 0 as *mut Coproc;
    let mut tcmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

    if sh_coproc.c_pid != NO_PID!()
        && (sh_coproc.c_rfd >= 0  || sh_coproc.c_wfd >= 0 )
    {
        internal_warning(
           b"execute_coproc: coproc [%d:%s] still exists\0" as *const u8 as *mut c_char,
            sh_coproc.c_pid,
            sh_coproc.c_name,
        );
    }
    coproc_init(&mut sh_coproc);
    
    invert = ((*command).flags & CMD_INVERT_RETURN as libc::c_int != 0 ) as libc::c_int;
    
    name = expand_string_unsplit_to_string( (*(*command).value.Coproc).name,0 );

    if legal_identifier(name) == 0 {
        internal_error(
            b"`%s': not a valid identifier\0" as *const u8 as *const libc::c_char,
            name,
        );
        return if invert != 0 { 0 } else { 1 };
    } else {
        free((*(*command).value.Coproc).name as *mut c_void);
        (*(*command).value.Coproc).name = name;
    }

    command_string_index = 0 ;
    tcmd = make_command_string(command);

    sh_openpipe(&mut rpipe as *mut [libc::c_int; 2] as *mut libc::c_int);
    sh_openpipe(&mut wpipe as *mut [libc::c_int; 2] as *mut libc::c_int);
   
    BLOCK_SIGNAL!(SIGCHLD, set, oset);

    p = savestring!(tcmd);
    coproc_pid = make_child(p, FORK_ASYNC as libc::c_int);

    if coproc_pid == 0 {
        close(rpipe[0 as libc::c_int as usize]);
        close(wpipe[1 as libc::c_int as usize]);
        
        FREE!(p);
    
        UNBLOCK_SIGNAL!(oset);
        estat = execute_in_subshell(
            command,
            1 ,
            wpipe[0 ],
            rpipe[1 ],
            fds_to_close,
        );
        fflush(stdout);
        fflush(stderr);

        exit(estat);
    }

    close(rpipe[1 ]);
    close(wpipe[0 ]);

    cp = coproc_alloc((*(*command).value.Coproc).name, coproc_pid);
    (*cp).c_rfd = rpipe[0 ];
    (*cp).c_wfd = wpipe[1 ];

    (*cp).c_flags |= COPROC_RUNNING as libc::c_int;

    fcntl((*cp).c_rfd, 2 as libc::c_int, 1 as libc::c_int);
    fcntl((*cp).c_wfd, 2 as libc::c_int, 1 as libc::c_int);
    coproc_setvars(cp);

    UNBLOCK_SIGNAL!(oset);

    close_pipes(pipe_in, pipe_out);

    unlink_fifo_list();

    stop_pipeline(1 , 0 as *mut libc::c_void as *mut COMMAND);
    DESCRIBE_PID!(coproc_pid);
    run_pending_traps();

    return if invert != 0 { 1 } else { 0 };
}

#[macro_export]
macro_rules! SET_CLOSE_ON_EXEC {
    ($fd:expr) => {
        fcntl($fd, F_SETFD as libc::c_int, FD_CLOEXEC as libc::c_int);
    };
}

unsafe extern "C" fn restore_stdin(mut s: libc::c_int) {
    dup2(s, 0 );
    close(s);
}
unsafe extern "C" fn lastpipe_cleanup(mut s: libc::c_int) {
    set_jobs_list_frozen(s);
}
#[macro_export]
macro_rules! get_job_by_jid {
    ($ind:expr) => {
        (*jobs).offset($ind as isize)
    };
}
#[macro_export]
macro_rules! INVALID_JOB {
    ($j:expr) => {
        ($j < 0 || $j >= js.j_jobslots || get_job_by_jid!($j) == 0 as *mut job)
    };
}

unsafe extern "C" fn execute_pipeline(
    mut command: *mut COMMAND,
    mut asynchronous: libc::c_int,
    mut pipe_in: libc::c_int,
    mut pipe_out: libc::c_int,
    mut fds_to_close: *mut fd_bitmap,
) -> libc::c_int {
    let mut prev: libc::c_int = 0;
    let mut fildes: [libc::c_int; 2] = [0; 2];
    let mut new_bitmap_size: libc::c_int = 0;
    let mut dummyfd: libc::c_int = 0;
    let mut ignore_return: libc::c_int = 0;
    let mut exec_result: libc::c_int = 0;
    let mut lstdin: libc::c_int = 0;
    let mut lastpipe_flag: libc::c_int = 0;
    let mut lastpipe_jid: libc::c_int = 0;
    let mut old_frozen: libc::c_int = 0;
    let mut cmd: *mut COMMAND = 0 as *mut COMMAND;
    let mut fd_bitmap: *mut fd_bitmap = 0 as *mut fd_bitmap;
    let mut lastpid: pid_t = 0;
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

    BLOCK_CHILD(&mut set, &mut oset);
    ignore_return = ((*command).flags & CMD_IGNORE_RETURN as libc::c_int != 0 )as libc::c_int;
    
    prev = pipe_in;
    cmd = command;
    
    while !cmd.is_null()
        && (*cmd).type_ == command_type_cm_connection 
        && !((*cmd).value.Connection).is_null()
        && (*(*cmd).value.Connection).connector == '|' as i32
    {
        if pipe(fildes.as_mut_ptr()) < 0  {
            sys_error(b"pipe error\0" as *const u8 as *const libc::c_char);
            
            terminate_current_pipeline();
            kill_current_pipeline();
            
            UNBLOCK_CHILD(&mut oset);

            last_command_exit_value = EXECUTION_FAILURE as c_int;

            throw_to_top_level();
            return 1;
        }

        new_bitmap_size = if fildes[0 ] < (*fds_to_close).size {
            (*fds_to_close).size
        } else {
            fildes[0 ] + 8 
        };

        fd_bitmap = new_fd_bitmap(new_bitmap_size);

        xbcopy((*fds_to_close).bitmap, (*fd_bitmap).bitmap, (*fds_to_close).size);
        
        *((*fd_bitmap).bitmap).offset(fildes[0] as isize) = 1 ;
        
        begin_unwind_frame(b"pipe-file-descriptors\0" as *const u8 as *mut libc::c_char);
        
        add_unwind_protect(
            transmute::<
            unsafe extern "C" fn (fdbp:*mut fd_bitmap) -> (),
            *mut Function,
            >(dispose_fd_bitmap),
            fd_bitmap as *mut c_char,
        );

        add_unwind_protect(
            transmute::<
            unsafe extern "C" fn (fdbp:*mut fd_bitmap) -> (),
            *mut Function,
            >(close_fd_bitmap),
            fd_bitmap as *mut c_char,
        );
        if prev >= 0 {
            add_unwind_protect(
                transmute::<
                unsafe extern "C" fn (__fd:c_int) -> c_int,
                *mut Function,
                >(close),
                prev as *mut c_char,
            );
        }
        dummyfd = fildes[1 ];
        add_unwind_protect(
            transmute::<
                unsafe extern "C" fn (__fd:c_int) -> c_int,
                *mut Function,
                >(close),
                dummyfd as *mut c_char,
        );

        add_unwind_protect(
            transmute::<
                    unsafe extern "C" fn(*mut sigset_t) -> libc::c_int,
                    *mut Function,
                >(restore_signal_mask),
                transmute::<*mut sigset_t, *mut c_char>(&mut oset),        //这个位置可能会存在问题
        );

        if ignore_return != 0 && !((*(*cmd).value.Connection).first).is_null() {
            (*(*(*cmd).value.Connection).first).flags |= CMD_IGNORE_RETURN as libc::c_int;
        }
        execute_command_internal(
            (*(*cmd).value.Connection).first,
            asynchronous,
            prev,
            fildes[1],
            fd_bitmap,
        );

        if prev >= 0 {
            close(prev);
        }

        prev = fildes[0];
        close(fildes[1]);

        dispose_fd_bitmap(fd_bitmap);
        discard_unwind_frame(
            b"pipe-file-descriptors\0" as *const u8 as *mut libc::c_char,
        );

        cmd = (*(*cmd).value.Connection).second;
    }

    lastpid = last_made_pid;
    if ignore_return != 0 && !cmd.is_null() {
        (*cmd).flags |= CMD_IGNORE_RETURN as libc::c_int;
    }
    lastpipe_flag = 0;

    begin_unwind_frame(
        b"lastpipe-exec\0" as *const u8 as *mut libc::c_char,
    );
    lstdin = -1;

    if lastpipe_opt != 0 && job_control == 0 
        && asynchronous == 0   && pipe_out == NO_PIPE
        && prev > 0 
    {
        lstdin = move_to_high_fd( 0, 1, -1,);
        if lstdin > 0 {
            do_piping(prev, pipe_out);
            prev = NO_PIPE;
            add_unwind_protect(
            transmute::<
                    unsafe extern "C" fn(libc::c_int) -> (),
                    *mut Function,
                >(restore_stdin),
                lstdin as *mut libc::c_char,
            );
            lastpipe_flag = 1 ;
            old_frozen = freeze_jobs_list();
            lastpipe_jid = stop_pipeline(
                0 as libc::c_int,
                0 as *mut libc::c_void as *mut COMMAND,
            );
            add_unwind_protect(
                transmute::<
                    unsafe extern "C" fn(libc::c_int) -> (),
                    *mut Function,
                >(lastpipe_cleanup),
                old_frozen as *mut libc::c_char,
            );
            UNBLOCK_CHILD(&mut oset);
        }
        if !cmd.is_null() {
            (*cmd).flags |= CMD_LASTPIPE as libc::c_int;
        }
    }

    if prev >= 0 {
        add_unwind_protect(
            transmute::<
                unsafe extern "C" fn(libc::c_int) -> c_int,
                *mut Function,
            >(close),           
        prev as *mut libc::c_char,
        );
    }

    exec_result = execute_command_internal(
        cmd,
        asynchronous,
        prev,
        pipe_out,
        fds_to_close,
    );

    if lstdin > 0 {
        restore_stdin(lstdin);
    }

    if prev >= 0 {
        close(prev);
    }

    UNBLOCK_CHILD(&mut oset);

    QUIT!();

    if lastpipe_flag != 0 {
        if (lastpipe_jid < 0 as libc::c_int || lastpipe_jid >= js.j_jobslots
            || (*jobs.offset(lastpipe_jid as isize)).is_null()) as libc::c_int
            == 0 as libc::c_int
        {
            append_process(
                savestring!(the_printed_command_except_trap),
                dollar_dollar_pid,
                exec_result,
                lastpipe_jid,
            );
            lstdin = wait_for(lastpid, 0 );
        } else {
            lstdin = wait_for_single_pid(lastpid, 0 );
        }
        if (lastpipe_jid < 0 || lastpipe_jid >= js.j_jobslots 
            || (*jobs.offset(lastpipe_jid as isize)).is_null()) as libc::c_int
            == 0 as libc::c_int
        {
            exec_result = job_exit_status(lastpipe_jid);
        } else if pipefail_opt != 0 {
            exec_result = exec_result | lstdin;
        }
        set_jobs_list_frozen(old_frozen);
    }
    discard_unwind_frame(
        b"lastpipe-exec\0" as *const u8 as *mut libc::c_char,
    );
    return exec_result;
}

const FLAG_AND:i32 = '&' as i32;
const FLAG_SEMICOLON:i32 = ';' as i32;
const FLAG_OR:i32 = '|' as i32;
const FLAG_OR_OR:i32 = OR_OR as i32;
const FLAG_AND_AND:i32 = AND_AND as i32;

unsafe extern "C" fn execute_connection(
    mut command: *mut COMMAND,
    mut asynchronous: libc::c_int,
    mut pipe_in: libc::c_int,
    mut pipe_out: libc::c_int,
    mut fds_to_close: *mut fd_bitmap,
) -> libc::c_int {
    let mut tc: *mut COMMAND = 0 as *mut COMMAND;
    let mut second: *mut COMMAND = 0 as *mut COMMAND;
    let mut ignore_return: libc::c_int = 0;
    let mut exec_result: libc::c_int = 0;
    let mut was_error_trap: libc::c_int = 0;
    let mut invert: libc::c_int = 0;
    let mut save_line_number: libc::c_int = 0;

    ignore_return = ((*command).flags & CMD_IGNORE_RETURN as libc::c_int != 0 ) as libc::c_int;
    

    match (*(*command).value.Connection).connector{
        FLAG_AND => {
            tc = (*(*command).value.Connection).first;
            if tc.is_null() {
                return EXECUTION_SUCCESS as libc::c_int;
            }

            if ignore_return != 0 {
                (*tc).flags |= CMD_IGNORE_RETURN as libc::c_int;
            }

            (*tc).flags |= CMD_AMPERSAND as libc::c_int;

            if (subshell_environment != 0 || job_control == 0) && stdin_redir == 0 {
                (*tc).flags |= CMD_STDIN_REDIR as libc::c_int;
            }
            exec_result = execute_command_internal(
                tc,
                1 ,
                pipe_in,
                pipe_out,
                fds_to_close,
            );
            QUIT!();
            
            if (*tc).flags & CMD_STDIN_REDIR as libc::c_int != 0 {
                (*tc).flags &= !(CMD_STDIN_REDIR as libc::c_int);
            }

            second = (*(*command).value.Connection).second;
            if !second.is_null() {
                if ignore_return != 0 {
                    (*second).flags |= CMD_IGNORE_RETURN as libc::c_int;
                }
                exec_result = execute_command_internal(
                    second,
                    asynchronous,
                    pipe_in,
                    pipe_out,
                    fds_to_close,
                );
            }
        }
        FLAG_SEMICOLON => {
            if ignore_return != 0 {
                if !((*(*command).value.Connection).first).is_null() {
                    (*(*(*command).value.Connection).first).flags |= CMD_IGNORE_RETURN as libc::c_int;
                }
                if !((*(*command).value.Connection).second).is_null() {
                    (*(*(*command).value.Connection).second).flags |= CMD_IGNORE_RETURN as libc::c_int;
                }
            }
            executing_list += 1;
            QUIT!(); 

            execute_command((*(*command).value.Connection).first);

            QUIT!();
            optimize_fork(command);
            exec_result = execute_command_internal(
                (*(*command).value.Connection).second,
                asynchronous,
                pipe_in,
                pipe_out,
                fds_to_close,
            );
            executing_list -= 1;
        }
        FLAG_OR => {
            was_error_trap = (signal_is_trapped(ERROR_TRAP as c_int) != 0
                && signal_is_ignored(ERROR_TRAP as libc::c_int) == 0 ) as libc::c_int;
            invert = ((*command).flags & CMD_INVERT_RETURN as libc::c_int != 0 )
                as libc::c_int;
            ignore_return = ((*command).flags & CMD_IGNORE_RETURN as libc::c_int != 0 )
                as libc::c_int;

            line_number_for_err_trap = line_number;
            exec_result = execute_pipeline(
                command,
                asynchronous,
                pipe_in,
                pipe_out,
                fds_to_close,
            );

            if asynchronous != 0 {
                exec_result = EXECUTION_SUCCESS as libc::c_int;
                invert = 0 as libc::c_int;
            }

            if was_error_trap != 0 && ignore_return == 0 
                && invert == 0 && exec_result != EXECUTION_SUCCESS as libc::c_int
            {
                last_command_exit_value = exec_result;
                save_line_number = line_number;
                line_number = line_number_for_err_trap;
                run_error_trap();
                line_number = save_line_number;
            }

            if ignore_return == 0 && invert == 0 
                && exit_immediately_on_error != 0 && exec_result != EXECUTION_SUCCESS as libc::c_int
            {
                last_command_exit_value = exec_result;
                run_pending_traps();
                jump_to_top_level(ERREXIT as libc::c_int);
            }
        }
        FLAG_AND_AND | FLAG_OR_OR => {
            if asynchronous != 0 {
                (*command).flags |= CMD_FORCE_SUBSHELL as libc::c_int;
                exec_result = execute_command_internal(
                    command,
                    1 ,
                    pipe_in,
                    pipe_out,
                    fds_to_close,
                );
            } else {
                executing_list += 1;
                if !((*(*command).value.Connection).first).is_null() {
                    (*(*(*command).value.Connection).first).flags |= CMD_IGNORE_RETURN as libc::c_int;
                }
                exec_result = execute_command((*(*command).value.Connection).first);
                
                QUIT!();

                if (*(*command).value.Connection).connector == AND_AND as libc::c_int
                    && exec_result == EXECUTION_SUCCESS as libc::c_int
                    || (*(*command).value.Connection).connector == OR_OR as libc::c_int
                        && exec_result != EXECUTION_SUCCESS as libc::c_int
                {
                    optimize_fork(command);

                    second = (*(*command).value.Connection).second;
                    if ignore_return != 0 && !second.is_null() {
                        (*second).flags |= CMD_IGNORE_RETURN as libc::c_int;
                    }
                    exec_result = execute_command(second);
                }
                executing_list -= 1;
            }
        }
        _ => {
            command_error(
                b"execute_connection\0" as *const u8 as *const libc::c_char,
                CMDERR_BADCONN as libc::c_int,
                (*(*command).value.Connection).connector,
                0 ,
            );
            jump_to_top_level(EXECUTION_FAILURE as libc::c_int);
        }
    }
    return exec_result;
}

#[macro_export]
macro_rules! REAP {
    () => {
        if job_control == 0 || interactive_shell == 0 {
            reap_dead_jobs();   
        }
    };
}

#[macro_export]
macro_rules! name_cell {
    ($var:expr) => {
        ((*$var).name)
    };
}

#[macro_export]
macro_rules! ifsname {
    ($s:expr) =>  {
        *$s.offset(0 as libc::c_int as isize) as libc::c_int == 'I' as i32
        && *$s.offset(1 as libc::c_int as isize) as libc::c_int
            == 'F' as i32
        && *$s.offset(2 as libc::c_int as isize) as libc::c_int
            == 'S' as i32
        && *$s.offset(3 as libc::c_int as isize) as libc::c_int
            == '\u{0}' as i32
    };
}

unsafe extern "C" fn execute_for_command(mut for_command: *mut FOR_COM) -> libc::c_int {
    let mut releaser: *mut WordList = 0 as *mut WordList;
    let mut list: *mut WordList = 0 as *mut WordList;
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut identifier: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut retval: libc::c_int = 0;
    let mut save_line_number: libc::c_int = 0;

    save_line_number = line_number;
    if check_identifier((*for_command).name, 1 ) == 0 {
        if posixly_correct != 0 && interactive_shell == 0 && rpm_requires == 0 
        {
            last_command_exit_value = EX_BADUSAGE as c_int;
            jump_to_top_level(ERREXIT as libc::c_int);
        }
        return EXECUTION_FAILURE as c_int;
    }

    loop_level += 1;
    identifier = (*(*for_command).name).word;

    line_number = (*for_command).line;
    releaser = expand_words_no_vars((*for_command).map_list);
    list = releaser;

    begin_unwind_frame(
        b"for\0" as *const u8 as *mut libc::c_char,
    );
    add_unwind_protect(
        transmute::<
            unsafe extern "C" fn (arg1: *mut WordList),
            *mut Function,
        >(dispose_words),
        releaser as *mut c_char,
    );

    if (*for_command).flags & CMD_IGNORE_RETURN as libc::c_int != 0 {
        (*(*for_command).action).flags |= CMD_IGNORE_RETURN as libc::c_int;
    }
    
    retval = EXECUTION_SUCCESS as libc::c_int;
    while !list.is_null() {
        QUIT!();

        line_number = (*for_command).line;

        command_string_index = 0 ;
        print_for_command_head(for_command);

        if echo_command_at_execute != 0 {
            xtrace_print_for_command_head(for_command);
        }

        if signal_in_progress(DEBUG_TRAP as c_int) == 0 && running_trap == 0  
        {
            FREE!(the_printed_command_except_trap);
            the_printed_command_except_trap = savestring!(the_printed_command);
        }

        retval = run_debug_trap();

        if !(debugging_mode != 0 && retval != EXECUTION_SUCCESS as libc::c_int) {
            this_command_name = 0 as *mut libc::c_char;

            v = find_variable_last_nameref(identifier, 1 );
            if !v.is_null() && nameref_p!(v) != 0 {
                if valid_nameref_value((*(*list).word).word, 1 ) == 0  
                {
                    sh_invalidid((*(*list).word).word);
                    v = 0 as *mut SHELL_VAR;
                } else if readonly_p!(v) != 0{
                    err_readonly(name_cell!(v));
                } else {
                    v = bind_variable_value(
                        v,
                        (*(*list).word).word,
                        ASS_NAMEREF as libc::c_int,
                    );
                }
            } else {
                v = bind_variable(identifier, (*(*list).word).word, 0  );
            }
            if v.is_null() || readonly_p!(v) != 0 || noassign_p!(v) != 0
            {
                line_number = save_line_number;
                if !v.is_null() && readonly_p!(v) != 0
                    && interactive_shell == 0 && posixly_correct != 0
                {
                    last_command_exit_value = EXECUTION_FAILURE as c_int; 
                    jump_to_top_level(FORCE_EOF as libc::c_int);
                } else {
                    dispose_words(releaser);
                    discard_unwind_frame(b"for\0" as *const u8 as *mut libc::c_char);
                    loop_level -= 1;
                    return EXECUTION_FAILURE as libc::c_int;
                }
            }

            if ifsname!(identifier)
            {
                setifs(v);
            } else {
                stupidly_hack_special_variables(identifier);
            }
            retval = execute_command((*for_command).action);
            REAP!();
            QUIT!();
           
            if breaking != 0 {
                breaking -= 1;
                break;
            } else if continuing != 0 {
                continuing -= 1;
                if continuing != 0 {
                    break;
                }
            }
        }

        list = (*list).next;
    }

    loop_level -= 1;
    line_number = save_line_number;

    dispose_words(releaser);
    discard_unwind_frame(b"for\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    return retval;
}
unsafe extern "C" fn eval_arith_for_expr(
    mut l: *mut WordList,
    mut okp: *mut libc::c_int,
) -> intmax_t {
    let mut new: *mut WordList = 0 as *mut WordList;
    let mut expresult: intmax_t = 0;
    let mut r: libc::c_int = 0;

    new = expand_words_no_vars(l);
    if !new.is_null() {
        if echo_command_at_execute != 0 {
            xtrace_print_arith_cmd(new);
        }

        this_command_name = b"((\0" as *const u8 as *mut libc::c_char;

        command_string_index = 0 ;
        print_arith_command(new);
        if signal_in_progress(DEBUG_TRAP as libc::c_int) == 0 && running_trap == 0 
        {
            FREE!(the_printed_command_except_trap);
            the_printed_command_except_trap = savestring!(the_printed_command);
        }

        r = run_debug_trap();
        if debugging_mode == 0 || r == EXECUTION_SUCCESS as libc::c_int {
            expresult = evalexp((*(*new).word).word, EXP_EXPANDED as libc::c_int, okp);
        } else {
            expresult = 0 as intmax_t;
            if !okp.is_null() {
                *okp = 1 ;
            }
        }

        dispose_words(new);
    } else {
        expresult = 0 as intmax_t;
        if !okp.is_null() {
            *okp = 1 ;
        }
    }
    return expresult;
}

unsafe extern "C" fn execute_arith_for_command(
    mut arith_for_command: *mut ARITH_FOR_COM,
) -> libc::c_int {
    let mut expresult: intmax_t = 0;
    let mut expok: libc::c_int = 0;
    let mut body_status: libc::c_int = 0;
    let mut arith_lineno: libc::c_int = 0;
    let mut save_lineno: libc::c_int = 0;
    
    body_status = EXECUTION_SUCCESS as libc::c_int;
    loop_level += 1;
    save_lineno = line_number;

    if (*arith_for_command).flags & CMD_IGNORE_RETURN as libc::c_int != 0 {
        (*(*arith_for_command).action).flags |= CMD_IGNORE_RETURN as libc::c_int;
    }

    this_command_name = b"((\0" as *const u8 as *mut libc::c_char;

    arith_lineno = (*arith_for_command).line;
    line_number = arith_lineno;
    if variable_context != 0 && interactive_shell != 0 && sourcelevel == 0
    {
        line_number -= function_line_number - 1 ;
        if line_number <= 0 {
            line_number = 1 ;
        }
    }
    expresult = eval_arith_for_expr((*arith_for_command).init, &mut expok);
    if expok == 0 {
        line_number = save_lineno;
        return EXECUTION_FAILURE as libc::c_int;
    }

    loop {
        line_number = arith_lineno;
        expresult = eval_arith_for_expr((*arith_for_command).test, &mut expok);
        line_number = save_lineno;

        if expok == 0 {
            body_status = EXECUTION_FAILURE as libc::c_int;
            break;
        } else {
            REAP!();
            if expresult == 0 {
                break;
            }

            QUIT!();
            body_status = execute_command((*arith_for_command).action);
            QUIT!();

            if breaking != 0 {
                breaking -= 1;
                break;
            } else {
                if continuing != 0 {
                    continuing -= 1;
                    if continuing != 0 {
                        break;
                    }
                }

                line_number = arith_lineno;
                expresult = eval_arith_for_expr((*arith_for_command).step, &mut expok);
                line_number = save_lineno;

                if !(expok == 0 ) {
                    continue;
                }
                body_status = 1 ;
                break;
            }
        }
    }
    loop_level -= 1;
    line_number = save_lineno;
    return body_status;
}

static mut COLS: libc::c_int = 0;
static mut tabsize: libc::c_int = 0;

pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;

#[macro_export]
macro_rules! STRLEN {
    ($s:expr) => {
        if !$s.is_null() && *$s.offset(0 as isize) as libc::c_int != 0 {
            if *$s.offset(1 as isize) as libc::c_int != 0 {
                if *$s.offset(2 as isize) as libc::c_int != 0 {
                    strlen($s)
                } else {
                    2 as libc::c_ulong
                }
            } else {
                1 as libc::c_ulong
            }
        } else {
            0 as libc::c_ulong
        }
    };
}

unsafe extern "C" fn displen(mut s: *const libc::c_char) -> libc::c_int {
    let mut wcstr: *mut wchar_t = 0 as *mut wchar_t;
    let mut slen: size_t = 0;
    let mut wclen: libc::c_int = 0;

    wcstr = 0 as *mut wchar_t;
    slen = mbstowcs(wcstr, s, 0 as size_t);
    if slen == -(1 as libc::c_int) as libc::c_ulong {
        slen = 0 as size_t;
    }

    wcstr = malloc((size_of::<wchar_t>() * (slen + 1) as usize) as usize) as *mut wchar_t;
    mbstowcs(wcstr, s, (slen + 1) as size_t);
    wclen = wcswidth(wcstr, slen as usize); 
    free(wcstr as *mut c_void);
    return (if wclen < 0 {
        STRLEN!(s)
    } else {
        wclen as libc::c_ulong
    }) as libc::c_int;
}

unsafe extern "C" fn print_index_and_element(
    mut len: libc::c_int,
    mut ind: libc::c_int,
    mut list: *mut WordList,
) -> libc::c_int {
    let mut l: *mut WordList = 0 as *mut WordList;
    let mut i: libc::c_int = 0;

    if list.is_null() {
        return 0 ;
    }

    i = ind;
    l = list;
    while !l.is_null()
        && {
            i -= 1;
            i != 0
        }
    {
        l = (*l).next;
    }
    if l.is_null() {
        return 0 ;
    }
    fprintf(
        stderr,
        b"%*d%s%s\0" as *const u8 as *const libc::c_char,
        len,
        ind,
        b") \0" as *const u8 as *const libc::c_char,
        (*(*l).word).word,
    );
    return displen((*(*l).word).word);
}


unsafe extern "C" fn indent(mut from: libc::c_int, mut to: libc::c_int) {
    while from < to {
        if to / tabsize > from / tabsize {
            putc('\t' as i32, stderr);
            from += tabsize - from % tabsize;
        } else {
            putc(' ' as i32, stderr);
            from += 1;
        }
    }
}

#[macro_export]
macro_rules! NUMBER_LEN {
    ($s:expr) => {
        if $s < 10  {
            1 
        } else if $s < 100 {
            2  
        } else if $s < 1000 {
            3  
        } else if $s < 10000  {
            4  
        } else if $s < 100000  {
            5  
        } else {
            6  
        };
    };
}

#[macro_export]
macro_rules! RP_SPACE_LEN {
    () => {
        2
    };
}

unsafe extern "C" fn print_select_list(
    mut list: *mut WordList,
    mut list_len: libc::c_int,
    mut max_elem_len: libc::c_int,
    mut indices_len: libc::c_int,
) {
    let mut ind: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut elem_len: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut cols: libc::c_int = 0;
    let mut rows: libc::c_int = 0;
    let mut first_column_indices_len: libc::c_int = 0;
    let mut other_indices_len: libc::c_int = 0;

    if list.is_null() {
        putc('\n' as i32, stderr);
        return;
    }

    cols = if max_elem_len != 0 { COLS / max_elem_len } else { 1 };
    if cols == 0 {
        cols = 1 ;
    }

    rows = if list_len != 0 {
        list_len / cols + (list_len % cols != 0 ) as c_int 
    } else { 
        1  
    };
    cols = if list_len != 0 {
        list_len / rows + (list_len % rows != 0 ) as c_int
    } else {
        1 
    };
    if rows == 1 {
        rows = cols;
        cols = 1 ;
    }

    first_column_indices_len = NUMBER_LEN!(rows);
    other_indices_len = indices_len;

    row = 0 ;
    while row < rows {
        ind = row;
        pos = 0 ;
        loop {
            indices_len = if pos == 0 {
                first_column_indices_len
            } else {
                other_indices_len
            };
            elem_len = print_index_and_element(
                indices_len,
                ind + 1 ,
                list,
            );
            elem_len += indices_len + RP_SPACE_LEN!()  ;
            ind += rows;
            if ind >= list_len {
                break;
            }
            indent(pos + elem_len, pos + max_elem_len);
            pos += max_elem_len;
        }
        putc('\n' as i32, stderr);

        row += 1;
    }
}

unsafe extern "C" fn select_query(
    mut list: *mut WordList,
    mut list_len: libc::c_int,
    mut prompt: *mut libc::c_char,
    mut print_menu: libc::c_int,
) -> *mut libc::c_char {
    let mut max_elem_len: libc::c_int = 0;
    let mut indices_len: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut oe: libc::c_int = 0;
    let mut reply: intmax_t = 0;
    let mut l: *mut WordList = 0 as *mut WordList;
    let mut repl_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;

    COLS = default_columns();

    tabsize = 8 ;
    max_elem_len = 0 ;
    l = list;
    while !l.is_null() {
        len = displen((*(*l).word).word);
        if len > max_elem_len {
            max_elem_len = len;
        }
        l = (*l).next;
    }
    indices_len = NUMBER_LEN!(list_len);
    max_elem_len += indices_len + RP_SPACE_LEN!() + 2 ;

    loop {
        if print_menu != 0 {
            print_select_list(list, list_len, max_elem_len, indices_len);
        }
        fprintf(stderr, b"%s\0" as *const u8 as *const libc::c_char, prompt);
        fflush(stderr);
        QUIT!();

        oe = executing_builtin;
        executing_builtin = 1 ;
        r = read_builtin(0 as *mut WordList);
        executing_builtin = oe;
        if r != EXECUTION_SUCCESS as libc::c_int {
            putchar('\n' as i32);
            return 0 as *mut libc::c_char;
        }
        repl_string = get_string_value(b"REPLY\0" as *const u8 as *const libc::c_char);
        if repl_string.is_null() {
            return 0 as *mut libc::c_char;
        }
        if *repl_string as libc::c_int == 0 {
            print_menu = 1 ;
        } else {
            if legal_number(repl_string, &mut reply) == 0 {
                return b"\0" as *const u8 as *mut libc::c_char;
            }
            if reply < 1 || reply > list_len as libc::c_long
            {
                return b"\0" as *const u8 as *mut libc::c_char;
            }
            l = list;
            while !l.is_null()
                && {
                    reply -= 1;
                    reply != 0
                }
            {
                l = (*l).next;
            }
            return (*(*l).word).word;
        }
    };
}

unsafe extern "C" fn execute_select_command(
    mut select_command: *mut SELECT_COM,
) -> libc::c_int {
    let mut releaser: *mut WordList = 0 as *mut WordList;
    let mut list: *mut WordList = 0 as *mut WordList;
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut identifier: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ps3_prompt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut selection: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut retval: libc::c_int = 0;
    let mut list_len: libc::c_int = 0;
    let mut show_menu: libc::c_int = 0;
    let mut save_line_number: libc::c_int = 0;

    if check_identifier((*select_command).name, 1 ) == 0 {
        return EXECUTION_FAILURE as libc::c_int;
    }
    save_line_number = line_number;
    line_number = (*select_command).line;

    command_string_index = 0 ;
    print_select_command_head(select_command);

    if echo_command_at_execute != 0 {
        xtrace_print_select_command_head(select_command);
    }

    if signal_in_progress(DEBUG_TRAP as c_int) == 0  
        && running_trap == 0 
    {
        FREE!(the_printed_command_except_trap);
        the_printed_command_except_trap = savestring!(the_printed_command);
    }

    retval = run_debug_trap();
    if debugging_mode != 0 && retval != EXECUTION_SUCCESS as libc::c_int {
        return EXECUTION_SUCCESS as libc::c_int;
    }

    loop_level += 1;
    identifier = (*(*select_command).name).word;

    releaser = expand_words_no_vars((*select_command).map_list);
    list = releaser;
    list_len = list_length(list as *mut GENERIC_LIST);
    if list.is_null() || list_len == 0 {
        if !list.is_null() {
            dispose_words(list);
        }
        line_number = save_line_number;
        return EXECUTION_SUCCESS as libc::c_int;
    }

    begin_unwind_frame(
        b"select\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    add_unwind_protect(
        transmute::<
            unsafe extern "C" fn (arg1: *mut WordList),
            *mut Function,
        >(dispose_words),
        releaser as *mut c_char,
    );

    if (*select_command).flags & CMD_IGNORE_RETURN as libc::c_int != 0 {
        (*(*select_command).action).flags |= CMD_IGNORE_RETURN as libc::c_int;
    }

    retval = EXECUTION_SUCCESS as libc::c_int;
    show_menu = 1 as libc::c_int;

    loop {
        line_number = (*select_command).line;
        ps3_prompt = get_string_value(b"PS3\0" as *const u8 as *const libc::c_char);
        if ps3_prompt.is_null() {
            ps3_prompt = b"#? \0" as *const u8 as *mut libc::c_char;
        }

        QUIT!();
        selection = select_query(list, list_len, ps3_prompt, show_menu);
        QUIT!();
        if selection.is_null() {
            retval = EXECUTION_FAILURE as libc::c_int;
            break;
        } else {
            v = bind_variable(identifier, selection, 0 );
            if v.is_null() || readonly_p!(v) != 0 || noassign_p!(v) != 0
            {
                if !v.is_null() && readonly_p!(v) != 0
                    && interactive_shell == 0 && posixly_correct != 0
                {
                    last_command_exit_value = EXECUTION_FAILURE as c_int;
                    jump_to_top_level(FORCE_EOF as libc::c_int);
                } else {
                    dispose_words(releaser);
                    discard_unwind_frame(b"select\0" as *const u8 as *mut libc::c_char);
                    loop_level -= 1;
                    line_number = save_line_number;
                    return EXECUTION_FAILURE as libc::c_int;
                }
            }

            stupidly_hack_special_variables(identifier);

            retval = execute_command((*select_command).action);

            REAP!();
            QUIT!();
            
            if breaking != 0 {
                breaking -= 1;
                break;
            } else {
                if continuing != 0 {
                    continuing -= 1;
                    if continuing != 0 {
                        break;
                    }
                }

                show_menu = 0 ;
                selection = get_string_value(
                    b"REPLY\0" as *const u8 as *const libc::c_char,
                );
                if !selection.is_null() && *selection as libc::c_int == '\u{0}' as i32 {
                    show_menu = 1 ;
                }
            }
        }
    }

    loop_level -= 1;
    line_number = save_line_number;

    dispose_words(releaser);
    discard_unwind_frame(
        b"select\0" as *const u8 as *mut libc::c_char,
    );
    return retval;
}

#[macro_export]
macro_rules! FNMATCH_EXTFLAG {
    () => {
        if extended_glob != 0 {
            1 << 5
        } else {0} 
    };
}

#[macro_export]
macro_rules! FNMATCH_IGNCASE {
    () => {
        if match_ignore_case != 0 {
            1 << 4
        } else {0} 
    };
}


#[macro_export]
macro_rules! FNM_NOMATCH {
    () => {
        1
    };
}


unsafe extern "C" fn execute_case_command(
    mut case_command: *mut CASE_COM,
) -> libc::c_int {
    let mut list: *mut WordList = 0 as *mut WordList;
    let mut wlist: *mut WordList = 0 as *mut WordList;
    let mut es: *mut WordList = 0 as *mut WordList;
    let mut clauses: *mut PATTERN_LIST = 0 as *mut PATTERN_LIST;
    let mut word: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pattern: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut retval: libc::c_int = 0;
    let mut match_0: libc::c_int = 0;
    let mut ignore_return: libc::c_int = 0;
    let mut save_line_number: libc::c_int = 0;
    let mut qflags: libc::c_int = 0;

    save_line_number = line_number;
    line_number = (*case_command).line;

    command_string_index = 0 as libc::c_int;
    print_case_command_head(case_command);

    if echo_command_at_execute != 0 {
        xtrace_print_case_command_head(case_command);
    }

    if signal_in_progress(DEBUG_TRAP as libc::c_int) == 0 && running_trap == 0  
    {
        FREE!(the_printed_command_except_trap);
        the_printed_command_except_trap = savestring!(the_printed_command);
    }

    retval = run_debug_trap();
    if debugging_mode != 0 && retval != EXECUTION_SUCCESS as c_int {
        line_number = save_line_number;
        return EXECUTION_SUCCESS as libc::c_int;
    }
    wlist = expand_word_leave_quoted((*case_command).word, 0 );

    if !wlist.is_null() {
        let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
        t = string_list(wlist);
        word = dequote_string(t);
        free(t as *mut c_void);
    } else {
        word = savestring!(b"\0" as *const u8 as *mut c_char);
    }
    dispose_words(wlist);

    retval = EXECUTION_SUCCESS as libc::c_int;
    ignore_return = (*case_command).flags & CMD_IGNORE_RETURN as libc::c_int;

    begin_unwind_frame(
        b"case\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    add_unwind_protect(
        transmute::<
        unsafe extern "C" fn (__ptr: *mut ::std::os::raw::c_void),
        *mut Function,
        >(free),
        word,
    );

    clauses = (*case_command).clauses;
    's_150: while !clauses.is_null() {
        QUIT!();
        list = (*clauses).patterns;
        while !list.is_null() {
            es = expand_word_leave_quoted((*list).word, 0 );

            if !es.is_null() && !((*es).word).is_null()
                && !((*(*es).word).word).is_null()
                && *(*(*es).word).word as libc::c_int != 0
            {
                qflags = QGLOB_CVTNULL as libc::c_int;
                qflags |= QGLOB_CTLESC as libc::c_int;
                pattern = quote_string_for_globbing((*(*es).word).word, qflags);
            } else {
                pattern = malloc(1 as usize) as *mut c_char;
                *pattern.offset(0 as isize) = '\u{0}' as i32 as libc::c_char;
            }
            match_0 = (strmatch(
                pattern,
                word,
                FNMATCH_EXTFLAG!() | FNMATCH_IGNCASE!()
            ) != FNM_NOMATCH!() )as c_int;
            free(pattern as *mut c_void);

            dispose_words(es);
            if match_0 != 0 {
                loop {
                    if !((*clauses).action).is_null() && ignore_return != 0 {
                        (*(*clauses).action).flags |= CMD_IGNORE_RETURN as libc::c_int;
                    }
                    retval = execute_command((*clauses).action);
                    if !((*clauses).flags & CASEPAT_FALLTHROUGH as libc::c_int != 0
                        && {
                            clauses = (*clauses).next;
                            !clauses.is_null()
                        })
                    {
                        break;
                    }
                }
                if clauses.is_null()
                    || (*clauses).flags & CASEPAT_TESTNEXT as libc::c_int == 0 
                {
                    break 's_150;
                } else {
                    break;
                }
            } else {
                QUIT!();
                list = (*list).next;
            }
        }
        clauses = (*clauses).next;
    }
    free(word as *mut c_void);
    discard_unwind_frame(b"case\0" as *const u8 as *mut libc::c_char);
    line_number = save_line_number;
    return retval;
}

#[macro_export]
macro_rules! CMD_WHILE {
    () => {
        0
    };
}

unsafe extern "C" fn execute_while_command(
    mut while_command: *mut WHILE_COM,
) -> libc::c_int {
    return execute_while_or_until(while_command, CMD_WHILE!());
}

#[macro_export]
macro_rules! CMD_UNTIL {
    () => {
        1
    };
}

unsafe extern "C" fn execute_until_command(
    mut while_command: *mut WHILE_COM,
) -> libc::c_int {
    return execute_while_or_until(while_command, CMD_UNTIL!());
}

unsafe extern "C" fn execute_while_or_until(
    mut while_command: *mut WHILE_COM,
    mut type_0: libc::c_int,
) -> libc::c_int {
    let mut return_value: libc::c_int = 0;
    let mut body_status: libc::c_int = 0;

    body_status = EXECUTION_SUCCESS as libc::c_int;
    loop_level += 1;
    (*(*while_command).test).flags |= CMD_IGNORE_RETURN as libc::c_int;
    if (*while_command).flags & CMD_IGNORE_RETURN as libc::c_int != 0 {
        (*(*while_command).action).flags |= CMD_IGNORE_RETURN as libc::c_int;
    }

    loop {
        return_value = execute_command((*while_command).test);
        REAP!();

        if type_0 == CMD_WHILE!() && return_value != EXECUTION_SUCCESS as libc::c_int {
            if breaking != 0 {
                breaking -= 1;
            }
            if continuing != 0 {
                continuing -= 1;
            }
            break;
        } else if type_0 == CMD_UNTIL!() && return_value == EXECUTION_SUCCESS as libc::c_int {
            if breaking != 0 {
                breaking -= 1;
            }
            if continuing != 0 {
                continuing -= 1;
            }
            break;
        } else {
            QUIT!();
            body_status = execute_command((*while_command).action);
            QUIT!();

            if breaking != 0 {
                breaking -= 1;
                break;
            } else {
                if !(continuing != 0) {
                    continue;
                }
                continuing -= 1;
                if continuing != 0 {
                    break;
                }
            }
        }
    }
    loop_level -= 1;

    return body_status;
}

unsafe extern "C" fn execute_if_command(mut if_command: *mut IF_COM) -> libc::c_int {
    let mut return_value: libc::c_int = 0;
    let mut save_line_number: libc::c_int = 0;

    save_line_number = line_number;
    (*(*if_command).test).flags |= CMD_IGNORE_RETURN as libc::c_int;
    return_value = execute_command((*if_command).test);
    line_number = save_line_number;

    if return_value == EXECUTION_SUCCESS as libc::c_int {
        QUIT!();

        if !((*if_command).true_case).is_null()
            && (*if_command).flags & CMD_IGNORE_RETURN as libc::c_int != 0
        {
            (*(*if_command).true_case).flags |= CMD_IGNORE_RETURN as libc::c_int;
        }

        return execute_command((*if_command).true_case);
    } else {
        QUIT!();

        if !((*if_command).false_case).is_null()
            && (*if_command).flags & CMD_IGNORE_RETURN as libc::c_int != 0
        {
            (*(*if_command).false_case).flags |= CMD_IGNORE_RETURN as libc::c_int;
        }

        return execute_command((*if_command).false_case);
    };
}

unsafe extern "C" fn execute_arith_command(
    mut arith_command: *mut ARITH_COM,
) -> libc::c_int {
    let mut expok: libc::c_int = 0;
    let mut save_line_number: libc::c_int = 0;
    let mut retval: libc::c_int = 0;
    let mut expresult: intmax_t = 0;
    let mut new: *mut WordList = 0 as *mut WordList;
    let mut exp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;

    expresult = 0 as intmax_t;

    save_line_number = line_number;
    this_command_name = b"((\0" as *const u8 as *mut libc::c_char;
    line_number = (*arith_command).line;
    line_number_for_err_trap = line_number;

    if variable_context != 0 && interactive_shell != 0 && sourcelevel == 0 
    {
        line_number -= function_line_number - 1 ;
        if line_number <= 0 {
            line_number = 1 ;
        }
    }

    command_string_index = 0 ;
    print_arith_command((*arith_command).exp);
    
    if signal_in_progress(DEBUG_TRAP as libc::c_int) == 0 
        && running_trap == 0 
    {
        FREE!(the_printed_command_except_trap);
        the_printed_command_except_trap = savestring!(the_printed_command);
    }

    retval = run_debug_trap();
    if debugging_mode != 0 && retval != EXECUTION_SUCCESS as libc::c_int {
        line_number = save_line_number;
        return EXECUTION_SUCCESS as libc::c_int;
    }

    t = 0 as *mut libc::c_char;
    new = (*arith_command).exp;
    if !((*new).next).is_null() {
        t = string_list(new);
        exp = t;
    } else {
        exp = (*(*new).word).word;
    }

    exp = expand_arith_string(exp, Q_DOUBLE_QUOTES as libc::c_int | Q_ARITH as libc::c_int);
    
    if echo_command_at_execute != 0 {
        new = make_word_list(
            make_word(
                if !exp.is_null() {
                    exp
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            ),
            0 as *mut WordList,
        );
        xtrace_print_arith_cmd(new);
        dispose_words(new);
    }

    if !exp.is_null() {
        expresult = evalexp(exp, EXP_EXPANDED as libc::c_int, &mut expok);
        line_number = save_line_number;
        free(exp as *mut c_void);
    } else {
        expresult = 0 as intmax_t;
        expok = 1 ;
    }
    FREE!(t);

    if expok == 0 {
        return EXECUTION_FAILURE as c_int ;
    }
    return if expresult == 0 {
        EXECUTION_FAILURE as libc::c_int
    } else {
        EXECUTION_SUCCESS as libc::c_int
    };
}

static mut nullstr: *mut libc::c_char = b"\0" as *const u8 as *mut libc::c_char;

unsafe extern "C" fn execute_cond_node(mut cond: *mut COND_COM) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut invert: libc::c_int = 0;
    let mut patmatch: libc::c_int = 0;
    let mut rmatch: libc::c_int = 0;
    let mut mflags: libc::c_int = 0;
    let mut ignore: libc::c_int = 0;
    let mut arg1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut arg2: *mut libc::c_char = 0 as *mut libc::c_char;

    invert = (*cond).flags & CMD_INVERT_RETURN as libc::c_int;
    ignore = (*cond).flags & CMD_IGNORE_RETURN as libc::c_int;
    if ignore != 0 {
        if !((*cond).left).is_null() {
            (*(*cond).left).flags |= CMD_IGNORE_RETURN as libc::c_int;
        }
        if !((*cond).right).is_null() {
            (*(*cond).right).flags |= CMD_IGNORE_RETURN as libc::c_int;
        }
    }

    if (*cond).type_ == COND_EXPR as libc::c_int {
        result = execute_cond_node((*cond).left);
    } else if (*cond).type_  == COND_OR as libc::c_int {
        result = execute_cond_node((*cond).left);
        if result != EXECUTION_SUCCESS as libc::c_int {
            result = execute_cond_node((*cond).right);
        }
    } else if (*cond).type_  == COND_AND as libc::c_int {
        result = execute_cond_node((*cond).left);
        if result == EXECUTION_SUCCESS as libc::c_int {
            result = execute_cond_node((*cond).right);
        }
    } else if (*cond).type_ == COND_UNARY as libc::c_int {
        if ignore != 0 {
            comsub_ignore_return += 1;
        }
        arg1 = cond_expand_word((*(*cond).left).op, 0 );
        if ignore != 0 {
            comsub_ignore_return -= 1;
        }
        if arg1.is_null() {
            arg1 = nullstr;
        }
        if echo_command_at_execute != 0 {
            xtrace_print_cond_term(
                (*cond).type_ ,
                invert,
                (*cond).op,
                arg1,
                0 as *mut libc::c_char,
            );
        }
        result = if unary_test((*(*cond).op).word, arg1) != 0 {
            EXECUTION_SUCCESS as libc::c_int
        } else {
            EXECUTION_FAILURE as libc::c_int
        };
        if arg1 != nullstr {
            free(arg1 as *mut c_void);
        }
    } else if (*cond).type_  == COND_BINARY as libc::c_int {
        rmatch = 0 ;
        patmatch = (*((*(*cond).op).word).offset(1 as isize)
            as libc::c_int == '=' as i32
            && *((*(*cond).op).word).offset(2 as isize) as libc::c_int
                == '\u{0}' as i32
            && (*((*(*cond).op).word).offset(0 as isize) as libc::c_int
                == '!' as i32
                || *((*(*cond).op).word).offset(0 as isize) as libc::c_int
                    == '=' as i32)
            || *((*(*cond).op).word).offset(0 as isize) as libc::c_int
                == '=' as i32
                && *((*(*cond).op).word).offset(1 as isize) as libc::c_int
                    == '\u{0}' as i32) as libc::c_int;
        rmatch = (*((*(*cond).op).word).offset(0 as isize) as libc::c_int
            == '=' as i32
            && *((*(*cond).op).word).offset(1 as isize) as libc::c_int
                == '~' as i32
            && *((*(*cond).op).word).offset(2 as isize) as libc::c_int
                == '\u{0}' as i32) as libc::c_int;

        if ignore != 0 {
            comsub_ignore_return += 1;
        }
        arg1 = cond_expand_word((*(*cond).left).op, 0 );
        if ignore != 0 {
            comsub_ignore_return -= 1;
        }
        if arg1.is_null() {
            arg1 = nullstr;
        }
        if ignore != 0 {
            comsub_ignore_return += 1;
        }
        arg2 = cond_expand_word(
            (*(*cond).right).op,
            if rmatch != 0 && shell_compatibility_level > 31 {
                2 
            } else if patmatch != 0 {
                1 
            } else {
                0 
            },
        );
        if ignore != 0 {
            comsub_ignore_return -= 1;
        }
        if arg2.is_null() {
            arg2 = nullstr;
        }

        if echo_command_at_execute != 0 {
            xtrace_print_cond_term((*cond).type_ , invert, (*cond).op, arg1, arg2);
        }

        if rmatch != 0 {
            mflags = SHMAT_PWARN as libc::c_int;
            mflags |= SHMAT_SUBEXP as libc::c_int;
            result = sh_regmatch(arg1, arg2, mflags);
        } else {
            let mut oe: libc::c_int = 0;

            oe = extended_glob;
            extended_glob = 1 ;
            result = if binary_test(
                (*(*cond).op).word,
                arg1,
                arg2,
                TEST_PATMATCH as libc::c_int | TEST_ARITHEXP as libc::c_int | TEST_LOCALE as libc::c_int,
            ) != 0
            {
                EXECUTION_SUCCESS as libc::c_int
            } else {
                EXECUTION_FAILURE as libc::c_int
            };
            extended_glob = oe;
        }
        if arg1 != nullstr {
            free(arg1 as *mut c_void);
        }
        if arg2 != nullstr {
            free(arg2  as *mut c_void);
        }
    } else {
        command_error(
            b"execute_cond_node\0" as *const u8 as *const libc::c_char,
            CMDERR_BADTYPE as libc::c_int,
            (*cond).type_ ,
            0 ,
        );
        jump_to_top_level(DISCARD as libc::c_int);
        result = EXECUTION_FAILURE  as libc::c_int;
    }
    if invert != 0 {
        result = if result == EXECUTION_FAILURE as libc::c_int {
            EXECUTION_FAILURE as libc::c_int
        } else {
            EXECUTION_SUCCESS as libc::c_int
        };
    }
    return result;
}

unsafe extern "C" fn execute_cond_command(
    mut cond_command: *mut COND_COM,
) -> libc::c_int {
    let mut retval: libc::c_int = 0;
    let mut save_line_number: libc::c_int = 0;

    save_line_number = line_number;
    this_command_name = b"[[\0" as *const u8 as *mut libc::c_char;
    line_number = (*cond_command).line;
    line_number_for_err_trap = line_number;
    if variable_context != 0 && interactive_shell != 0 && sourcelevel == 0 
    {
        line_number -= function_line_number - 1 ;
        if line_number <= 0 {
            line_number = 1 ;
        }
    }

    command_string_index = 0 ;
    print_cond_command(cond_command);
    if signal_in_progress( DEBUG_TRAP as libc::c_int) == 0 
        && running_trap == 0 
    {
        FREE!(the_printed_command_except_trap);
        the_printed_command_except_trap = savestring!(the_printed_command)
    }

    retval = run_debug_trap();
    if debugging_mode != 0 && retval != EXECUTION_SUCCESS as libc::c_int {
        line_number = save_line_number;
        return EXECUTION_SUCCESS as libc::c_int;
    }
    retval = execute_cond_node(cond_command);
    last_command_exit_value = retval;
    line_number = save_line_number;
    return retval;
}

#[macro_export]
macro_rules! VUNSETATTR {
    ($var:expr, $attr:expr) => {
        (*$var).attributes &= !($attr as libc::c_int);
    };
}


unsafe extern "C" fn bind_lastarg(mut arg: *mut libc::c_char) {
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;

    if arg.is_null() {
        arg = b"\0" as *const u8 as *mut libc::c_char;
    }
    var = bind_variable(
        b"_\0" as *const u8 as *const libc::c_char,
        arg,
        0 ,
    );

    if !var.is_null() {
        VUNSETATTR!(var, att_exported);
    }
}

#[macro_export]
macro_rules! INPUT_REDIRECT {
    ($ri:expr) => {
        $ri == r_input_direction as libc::c_uint 
        || $ri == r_inputa_direction as libc::c_uint 
        || $ri == r_input_output as libc::c_uint 
    };
}

#[macro_export]
macro_rules! TRANSLATE_REDIRECT {
    ($ri:expr) => {
        $ri == r_duplicating_input_word 
        || $ri == r_duplicating_output_word 
        || $ri == r_move_input_word 
        || $ri == r_move_output_word
    };
}

unsafe extern "C" fn execute_null_command(
    mut redirects: *mut REDIRECT,
    mut pipe_in: libc::c_int,
    mut pipe_out: libc::c_int,
    mut async_0: libc::c_int,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut forcefork: libc::c_int = 0;
    let mut fork_flags: libc::c_int = 0;
    let mut rd: *mut REDIRECT = 0 as *mut REDIRECT;

    forcefork = 0 ;
    rd = redirects;
    while !rd.is_null() {
        forcefork += (*rd).rflags & REDIR_VARASSIGN as libc::c_int;
        forcefork += ((*rd).redirector.dest == 0 
                   || fd_is_bash_input((*rd).redirector.dest) != 0
                   && INPUT_REDIRECT!((*rd).instruction)
                   || TRANSLATE_REDIRECT!((*rd).instruction)
                   || (*rd).instruction == r_instruction_r_close_this) as c_int;
        rd = (*rd).next;
    }

    if forcefork != 0 || pipe_in != NO_PIPE
        || pipe_out != NO_PIPE || async_0 != 0
    {
        fork_flags = if async_0 != 0 { FORK_ASYNC as libc::c_int } else { 0 };
        if make_child(0 as *mut libc::c_char, fork_flags) == 0 
        {
            restore_original_signals();
            do_piping(pipe_in, pipe_out);
            coproc_closeall();

            interactive = 0 ;

            subshell_environment = 0 ;
            if async_0 != 0 {
                subshell_environment |= SUBSHELL_ASYNC as libc::c_int;
            }
            if pipe_in != NO_PIPE || pipe_out != NO_PIPE {
                subshell_environment |= SUBSHELL_PIPE as libc::c_int;
            }
            if do_redirections(redirects, RX_ACTIVE as libc::c_int) == 0 {
                exit(EXECUTION_SUCCESS as libc::c_int);
            } else {
                exit(EXECUTION_FAILURE as libc::c_int);
            }
        } else {
            close_pipes(pipe_in, pipe_out);
            if pipe_out == NO_PIPE {
                unlink_fifo_list();
            }
            return EXECUTION_SUCCESS as libc::c_int;
        }
    } else {
        r = do_redirections(redirects, RX_ACTIVE as libc::c_int | RX_UNDOABLE as libc::c_int);
        cleanup_redirects(redirection_undo_list);
        redirection_undo_list = 0 as *mut REDIRECT;

        if r != 0 {
            return EXECUTION_FAILURE as libc::c_int
        } else if last_command_subst_pid != NO_PID!() {
            return last_command_exit_value
        } else {
            return EXECUTION_SUCCESS as libc::c_int
        }
    };
    return 0;
}

unsafe extern "C" fn fix_assignment_words(mut words: *mut WordList) {
    let mut w: *mut WordList = 0 as *mut WordList;
    let mut wcmd: *mut WordList = 0 as *mut WordList;
    let mut b: *mut builtin = 0 as *mut builtin;
    let mut assoc: libc::c_int = 0;
    let mut global: libc::c_int = 0;
    let mut array: libc::c_int = 0;
    let mut integer: libc::c_int = 0;

    if words.is_null() {
        return;
    }

    b = 0 as *mut builtin;
    integer = 0 ;
    array = integer;
    global = array;
    assoc = global;
    
    wcmd = words;
    wcmd = words;
    while !wcmd.is_null() {
        if (*(*wcmd).word).flags & W_ASSIGNMENT as c_int == 0 
        {
            break;
        }
        wcmd = (*wcmd).next;
    }

    while posixly_correct != 0 && !wcmd.is_null() && !((*wcmd).word).is_null()
        && !((*(*wcmd).word).word).is_null()
        && STREQ!((*(*wcmd).word).word, b"command\0" as *const u8 as *const c_char)
    {
        wcmd = (*wcmd).next;
    }
    
    w = wcmd;
    while !w.is_null() {
        if (*(*w).word).flags & W_ASSIGNMENT as libc::c_int != 0 {
            if b.is_null() {
                b = builtin_address_internal((*(*wcmd).word).word, 0 );
                if b.is_null() || (*b).flags & ASSIGNMENT_BUILTIN as libc::c_int == 0 {
                    return
                } else {
                    if !b.is_null() && (*b).flags & ASSIGNMENT_BUILTIN as libc::c_int != 0 {
                        (*(*wcmd).word).flags |= W_ASSNBLTIN as libc::c_int;
                    }
                }
            }
            (*(*w).word).flags
                |= (W_NOSPLIT as libc::c_int)  
                    | (W_NOGLOB as libc::c_int) 
                    | (W_TILDEEXP as libc::c_int)  
                    | (W_ASSIGNARG as libc::c_int) ;
            if assoc != 0 {
                (*(*w).word).flags |= W_ASSIGNASSOC as libc::c_int;
            }
            if array != 0 {
                (*(*w).word).flags |= W_ASSIGNARRAY as libc::c_int;
            }
            if global != 0 {
                (*(*w).word).flags |= W_ASSNGLOBAL as libc::c_int;
            }

            if !b.is_null()
                && (*b).flags & (ASSIGNMENT_BUILTIN as libc::c_int | LOCALVAR_BUILTIN as libc::c_int)
                    == ASSIGNMENT_BUILTIN as libc::c_int
            {
                (*(*w).word).flags
                    |= W_ASSNGLOBAL as libc::c_int
                        | W_CHKLOCAL as libc::c_int;
            } else if !b.is_null() && (*b).flags & ASSIGNMENT_BUILTIN as libc::c_int != 0
                    && (*b).flags & LOCALVAR_BUILTIN as libc::c_int != 0 && variable_context != 0
                {
                (*(*w).word).flags |= W_FORCELOCAL as libc::c_int;
            }
        } else if *((*(*w).word).word).offset(0 as isize) as libc::c_int == '-' as i32
                && !(strpbrk(
                    ((*(*w).word).word).offset(1 as isize),
                    b"Aag\0" as *const u8 as *const libc::c_char,
                )).is_null()
            {
            if b.is_null() {
                b = builtin_address_internal((*(*wcmd).word).word, 0 );
                if b.is_null() || (*b).flags & ASSIGNMENT_BUILTIN as libc::c_int == 0 {
                    return
                } else {
                    if !b.is_null() && (*b).flags & ASSIGNMENT_BUILTIN as libc::c_int != 0 {
                        (*(*wcmd).word).flags |= W_ASSNBLTIN as libc::c_int;
                    }
                }
            }
            if (*(*wcmd).word).flags & W_ASSNBLTIN as libc::c_int != 0
                && !(strchr(
                    ((*(*w).word).word).offset(1 as isize),
                    'A' as i32,
                )).is_null()
            {
                assoc = 1 ;
            } else if (*(*wcmd).word).flags & W_ASSNBLTIN as libc::c_int
                    != 0
                    && !(strchr(
                        ((*(*w).word).word).offset(1 as libc::c_int as isize),
                        'a' as i32,
                    )).is_null()
                {
                array = 1 ;
            }
            if (*(*wcmd).word).flags & W_ASSNBLTIN as libc::c_int != 0
                && !(strchr(
                    ((*(*w).word).word).offset(1 as isize),
                    'g' as i32,
                )).is_null()
            {
                global = 1 ;
            }
        }
        w = (*w).next;
    }
}

#[macro_export]
macro_rules! ISOPTION {
    ($s:expr, $c:expr) => {
        (*$s.offset(0 as isize) as libc::c_int
            == '-' as i32
            && *$s.offset(1 as isize) as libc::c_int
                == $c as i32
            && *$s.offset(2 as isize) == 0)
    };
}



unsafe extern "C" fn check_command_builtin(
    mut words: *mut WordList,
    mut typep: *mut libc::c_int,
) -> *mut WordList {
    let mut type_0: libc::c_int = 0;
    let mut w: *mut WordList = 0 as *mut WordList;

    #[macro_export]
    macro_rules! RETURN_NOT_COMMAND {
        () => {
            if !typep.is_null() {
                *typep = 0 ;
            }
            return words;
        };
    }

    w = (*words).next;
    type_0 = 1 ;

    if !w.is_null() && ISOPTION!((*(*w).word).word, 'p')
    {
        if restricted != 0 {
            RETURN_NOT_COMMAND!();
        }
        w = (*w).next;
        type_0 = 2 ;
    }
    if !w.is_null()
        && ISOPTION!((*(*w).word).word, '-')
    {
        w = (*w).next;
    } else if !w.is_null()
            && *((*(*w).word).word).offset(0 as isize) as libc::c_int == '-' as i32
        {
            RETURN_NOT_COMMAND!();
    }
    if w.is_null() || ((*(*w).word).word).is_null() {
        RETURN_NOT_COMMAND!();
    }

    if !typep.is_null() {
        *typep = type_0;
    }
    return w;
}

unsafe extern "C" fn is_dirname(mut pathname: *mut libc::c_char) -> libc::c_int {
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 0;

    temp = search_for_command(pathname, 0 );
    ret = if !temp.is_null() { file_isdir(temp) } else { file_isdir(pathname) };
    free(temp as *mut c_void);
    return ret;
}

#[macro_export]
macro_rules! EX_REDIRFAIL {
    () => {
        259
    };
}

#[macro_export]
macro_rules! EX_BADASSIGN {
    () => {
        260
    };
}

#[macro_export]
macro_rules! EX_EXPFAIL {
    () => {
        261
    };
}

#[macro_export]
macro_rules! EX_DISKFALLBACK {
    () => {
        262
    };
}

unsafe extern "C" fn execute_simple_command(
    mut simple_command: *mut SIMPLE_COM,
    mut pipe_in: libc::c_int,
    mut pipe_out: libc::c_int,
    mut async_0: libc::c_int,
    mut fds_to_close: *mut fd_bitmap,
) -> libc::c_int {
    let mut current_block: u64;
    let mut words: *mut WordList = 0 as *mut WordList;
    let mut lastword: *mut WordList = 0 as *mut WordList;
    let mut command_line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lastarg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut first_word_quoted: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    let mut builtin_is_special: libc::c_int = 0;
    let mut already_forked: libc::c_int = 0;
    let mut dofork: libc::c_int = 0;
    let mut fork_flags: libc::c_int = 0;
    let mut cmdflags: libc::c_int = 0;
    let mut old_last_async_pid: pid_t = 0;
    let mut builtin: sh_builtin_func_t = None;
    let mut func: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut old_builtin: libc::c_int = 0;
    let mut old_command_builtin: libc::c_int = 0;

    result = EXECUTION_SUCCESS as c_int ;
    builtin_is_special = 0 ;
    special_builtin_failed = builtin_is_special;
    command_line = 0 as *mut libc::c_char;

    QUIT!();

    if variable_context != 0 && interactive_shell != 0 && sourcelevel == 0  
    {
        line_number -= function_line_number - 1 ;
        if line_number <= 0 {
            line_number = 1 ;
        }
    }

    command_string_index = 0 ;
    print_simple_command(simple_command);

    if signal_in_progress( DEBUG_TRAP as c_int) == 0 
        && running_trap == 0  
    {
        if !the_printed_command_except_trap.is_null() {
            FREE!(the_printed_command_except_trap);
        }
        the_printed_command_except_trap = if !the_printed_command.is_null() {
            savestring!(the_printed_command)
        } else {
            0 as *mut libc::c_char
        };
    }

    result = run_debug_trap();

    if debugging_mode != 0 && result != EXECUTION_SUCCESS as libc::c_int {
        return EXECUTION_SUCCESS as libc::c_int;
    }

    cmdflags = (*simple_command).flags;

    first_word_quoted = if !((*simple_command).words).is_null() {
        (*(*(*simple_command).words).word).flags & W_QUOTED as c_int
    } else {
        0 
    };

    last_command_subst_pid = NO_PID!();
    old_last_async_pid = last_asynchronous_pid;

    already_forked = 0 ;

    dofork = (pipe_in != NO_PIPE as c_int || pipe_out != NO_PIPE as libc::c_int
        || async_0 != 0) as libc::c_int;

    if dofork != 0 && pipe_in == NO_PIPE as libc::c_int && pipe_out == NO_PIPE as libc::c_int
        && !((*simple_command).words).is_null()
        && !((*(*simple_command).words).word).is_null()
        && !((*(*(*simple_command).words).word).word).is_null()
        && *((*(*(*simple_command).words).word).word).offset(0 as isize)
            as libc::c_int == '%' as i32
    {
        dofork = 0 ;
    }

    if dofork != 0 {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;

        maybe_make_export_env();

        fork_flags = if async_0 != 0 { FORK_ASYNC as libc::c_int } else { 0 };
        p = savestring!(the_printed_command_except_trap);
        if make_child(p, fork_flags) == 0 {
            already_forked = 1 ;
            cmdflags |= CMD_NO_FORK as libc::c_int;

            subshell_environment = SUBSHELL_FORK as libc::c_int;
            if pipe_in != NO_PIPE as libc::c_int || pipe_out != NO_PIPE as libc::c_int {
                subshell_environment |= SUBSHELL_PIPE as libc::c_int;
            }
            if async_0 != 0 {
                subshell_environment |= SUBSHELL_ASYNC as libc::c_int;
            }

            if !fds_to_close.is_null() {
                close_fd_bitmap(fds_to_close);
            }

            stdin_redir |= (pipe_in != NO_PIPE as libc::c_int) as libc::c_int;

            do_piping(pipe_in, pipe_out);
            pipe_out = NO_PIPE as libc::c_int;
            pipe_in = pipe_out;

            coproc_closeall();
            last_asynchronous_pid = old_last_async_pid;

            if async_0 != 0 {
                subshell_level += 1;
            }
            FREE!(p);
        } else {
            if pipe_out != NO_PIPE as libc::c_int {
                result = last_command_exit_value;
            }
            close_pipes(pipe_in, pipe_out);
            command_line = 0 as *mut libc::c_char;
            return result;
        }
    }

    QUIT!();

    if cmdflags & CMD_INHIBIT_EXPANSION as libc::c_int == 0 {
        current_fds_to_close = fds_to_close;
        fix_assignment_words((*simple_command).words);

        if cmdflags & CMD_IGNORE_RETURN as libc::c_int != 0 {
            comsub_ignore_return += 1;
        }

        words = expand_words((*simple_command).words);
        if cmdflags & CMD_IGNORE_RETURN as libc::c_int != 0 {
            comsub_ignore_return -= 1;
        }
        current_fds_to_close = 0 as *mut fd_bitmap;
    } else {
        words = copy_word_list((*simple_command).words);
    }

    if words.is_null() {
        this_command_name = 0 as *mut libc::c_char;
        result = execute_null_command(
            (*simple_command).redirects,
            pipe_in,
            pipe_out,
            if already_forked != 0 { 0 } else { async_0 },
        );

        if already_forked != 0 {
            sh_exit(result);
        } else {
            bind_lastarg(0 as *mut libc::c_char);
            set_pipestatus_from_exit(result);
            return result;
        }
    }

    lastarg = 0 as *mut libc::c_char;
    begin_unwind_frame(
        b"simple-command\0" as *const u8 as *mut libc::c_char,
    );

    if echo_command_at_execute != 0
        && cmdflags & CMD_COMMAND_BUILTIN as libc::c_int == 0 
    {
        xtrace_print_word_list(words, 1 );
    }
    
    builtin = None;
    
    func = 0 as *mut SHELL_VAR;

    if cmdflags & CMD_NO_FUNCTIONS as libc::c_int == 0 {
        if posixly_correct != 0 {
            builtin = find_special_builtin((*(*words).word).word);
            if builtin.is_some() {
                builtin_is_special = 1 ;
            }
        }
        if builtin.is_none() {
            func = find_function((*(*words).word).word);
        }
    }

    if posixly_correct != 0 && builtin_is_special != 0
        && interactive_shell == 0 && tempenv_assign_error != 0
    {
        last_command_exit_value = EXECUTION_FAILURE as c_int;
        jump_to_top_level(ERREXIT as libc::c_int);
    }
    tempenv_assign_error = 0 ;
    old_command_builtin = -1;

    if builtin.is_none() && func.is_null() {
        let mut disposer: *mut WordList = 0 as *mut WordList;
        let mut l: *mut WordList = 0 as *mut WordList;
        let mut cmdtype: libc::c_int = 0;

        builtin =  find_shell_builtin((*(*words).word).word);
    
        while builtin == Some(command_builtin)   
        {
            disposer = words;
            cmdtype = 0;
            words = check_command_builtin(words, &mut cmdtype);
            if !(cmdtype > 0 ) {
                break;
            }
            l = disposer;
            while (*l).next != words {
                l = (*l).next;
            }
            
            (*l).next = 0 as *mut WordList;
            dispose_words(disposer);
            cmdflags |= CMD_COMMAND_BUILTIN as libc::c_int | CMD_NO_FUNCTIONS as libc::c_int;
            if cmdtype == 2 {
                cmdflags |= CMD_STDPATH as libc::c_int;
            }
            builtin = find_shell_builtin((*(*words).word).word);
        }
        if cmdflags & CMD_COMMAND_BUILTIN as libc::c_int != 0 {
            old_command_builtin = executing_command_builtin;
            unwind_protect_mem(
                &mut executing_command_builtin as *mut libc::c_int as *mut libc::c_char,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
            );
            executing_command_builtin |= 1 ;
        }
        builtin = None;
    }
    add_unwind_protect(
        transmute::<
        unsafe extern "C" fn (arg1: *mut WordList),
        *mut Function,
        >(dispose_words),
        words as *mut c_char,
    );

    QUIT!();

    lastword = words;
    while !((*lastword).next).is_null() {
        lastword = (*lastword).next;
    }

    lastarg = (*(*lastword).word).word;

    if *((*(*words).word).word).offset(0 as isize) as libc::c_int
        == '%' as i32 && already_forked == 0  
    {
        this_command_name = (if async_0 != 0 {
            b"bg\0" as *const u8 as *const libc::c_char
        } else {
            b"fg\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char;

        last_shell_builtin = this_shell_builtin;
        this_shell_builtin = builtin_address(this_command_name);
        result = (Some(this_shell_builtin.expect("non-null function pointer")))
            .expect("non-null function pointer")(words);
    } else {
        if job_control != 0 && already_forked == 0
            && async_0 == 0 && first_word_quoted == 0
            && ((*words).next).is_null()
            && *((*(*words).word).word).offset(0 as isize) as libc::c_int
                != 0 && ((*simple_command).redirects).is_null()
            && pipe_in == NO_PIPE as libc::c_int && pipe_out == NO_PIPE as libc::c_int
            && {
                temp = get_string_value(
                    b"auto_resume\0" as *const u8 as *const libc::c_char,
                );
                !temp.is_null()
            }
        {
            let mut job: libc::c_int = 0;
            let mut jflags: libc::c_int = 0;
            let mut started_status: libc::c_int = 0;

            jflags = JM_STOPPED as libc::c_int | JM_FIRSTMATCH as libc::c_int;
            if STREQ!(temp, b"exact" as *const u8 as *mut c_char)
            {
                jflags |= JM_EXACT as libc::c_int;
            } else if STREQ!(temp, b"substring" as *const u8 as *mut c_char)
            {
                jflags |= JM_SUBSTRING as libc::c_int;
            } else {
                jflags |= JM_PREFIX as libc::c_int;
            }
            job = get_job_by_name((*(*words).word).word, jflags);
            if job != NO_JOB  {
                run_unwind_frame(
                    b"simple-command\0" as *const u8 as *mut libc::c_char,
                );
                this_command_name = b"fg\0" as *const u8 as *mut libc::c_char;
                last_shell_builtin = this_shell_builtin;
                this_shell_builtin = builtin_address(
                    b"fg\0" as *const u8 as *mut libc::c_char,
                );

                started_status = start_job(job, 1 );
                return if started_status < 0 {
                    EXECUTION_FAILURE as libc::c_int
                } else {
                    started_status
                };
            }
        }
        loop {
            this_command_name = (*(*words).word).word;

            QUIT!();

            if func.is_null() && builtin.is_none() {
                builtin = find_shell_builtin(this_command_name);
            }

            last_shell_builtin = this_shell_builtin;
            this_shell_builtin = builtin;

            if builtin.is_some() || !func.is_null() {
                if builtin.is_some() {
                    old_builtin = executing_builtin;
                    unwind_protect_mem(
                        &mut executing_builtin as *mut libc::c_int as *mut libc::c_char,
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                            as libc::c_int,
                    );
                    if old_command_builtin == -1 {
                        old_command_builtin = executing_command_builtin;
                        unwind_protect_mem(
                            &mut executing_command_builtin as *mut libc::c_int
                                as *mut libc::c_char,
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int,
                        );
                    }
                }
                if already_forked != 0 {
                    reset_signal_handlers();
                    subshell_environment |= SUBSHELL_RESETTRAP as libc::c_int;
                    if async_0 != 0 {
                        if cmdflags & CMD_STDIN_REDIR as libc::c_int != 0
                            && pipe_in == NO_PIPE 
                            && stdin_redirects((*simple_command).redirects)
                                == 0 
                        {
                            async_redirect_stdin();
                        }
                        setup_async_signals();
                    }
                    if async_0 == 0 {
                        subshell_level += 1;
                    }
                    execute_subshell_builtin_or_function(
                        words,
                        (*simple_command).redirects,
                         builtin,
                        func,
                        pipe_in,
                        pipe_out,
                        async_0,
                        fds_to_close,
                        cmdflags,
                    );
                    subshell_level -= 1;
                } else {
                    result = execute_builtin_or_function(
                        words,
                        builtin,
                        func,
                        (*simple_command).redirects,
                        fds_to_close,
                        cmdflags,
                    );
                    if builtin.is_some() {
                        current_block = 2525024825076287515;
                        break;
                    } else {
                        current_block = 2149547614657787525;
                        break;
                    }
                }
            }
            if !(autocd != 0 && interactive != 0 && !((*words).word).is_null()
                && is_dirname((*(*words).word).word) != 0)
            {
                current_block = 5373862753408874748;
                break;
            }
            words = make_word_list(
                make_word(b"--\0" as *const u8 as *const libc::c_char),
                words,
            );
            words = make_word_list(
                make_word(b"cd\0" as *const u8 as *const libc::c_char),
                words,
            );
            xtrace_print_word_list(words, 0 as libc::c_int);
            func = find_function(b"cd\0" as *const u8 as *const libc::c_char);
        }
        match current_block {
            2525024825076287515 => {
                if result > EX_SHERRBASE as libc::c_int {
                    match result {
                        EX_REDIRFAIL!() | EX_BADASSIGN!() | EX_EXPFAIL!() => {
                            if posixly_correct != 0 && builtin_is_special != 0
                                && interactive_shell == 0 
                            {
                                last_command_exit_value = EXECUTION_FAILURE as c_int;
                                jump_to_top_level(ERREXIT as libc::c_int);
                            }
                            current_block = 5872168878400681860;
                        }
                        EX_DISKFALLBACK!() => {
                            executing_builtin = old_builtin;
                            executing_command_builtin = old_command_builtin;
                            builtin = None;
                            current_block = 5373862753408874748;
                        }
                        _ => {
                            current_block = 5872168878400681860;
                        }
                    }
                    match current_block {
                        5373862753408874748 => {}
                        _ => {
                            result = builtin_status(result);
                            if builtin_is_special != 0 {
                                special_builtin_failed = 1 as libc::c_int;
                            }
                            current_block = 8487579351791723214;
                        }
                    }
                } else {
                    current_block = 8487579351791723214;
                }
                match current_block {
                    5373862753408874748 => {}
                    _ => {
                        if posixly_correct != 0 && builtin_is_special != 0
                            && !temporary_env.is_null()
                        {
                            merge_temporary_env();
                        }
                        current_block = 11272946706888692785;
                    }
                }
            }
            2149547614657787525 => {
                if result == EX_USAGE as libc::c_int {
                    result = EX_BADUSAGE as libc::c_int;
                } else if result > EX_SHERRBASE as libc::c_int {
                    result = builtin_status(result);
                }
                current_block = 11272946706888692785;
            }
            _ => {}
        }
        match current_block {
            11272946706888692785 => {
                set_pipestatus_from_exit(result);
            }
            _ => {
                if command_line.is_null() {
                    command_line = savestring!(if !the_printed_command_except_trap.is_null() {
                        the_printed_command_except_trap
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    });
                }
                if already_forked == 0 as libc::c_int
                    && cmdflags & 0x40 as libc::c_int != 0
                    && fifos_pending() > 0 as libc::c_int
                {
                    cmdflags &= !(0x40 as libc::c_int);
                }
                result = execute_disk_command(
                    words,
                    (*simple_command).redirects,
                    command_line,
                    pipe_in,
                    pipe_out,
                    async_0,
                    fds_to_close,
                    cmdflags,
                );
            }
        }
    }
    bind_lastarg(lastarg);
    FREE!(command_line);
    dispose_words(words);
    if builtin.is_some() {
        executing_builtin = old_builtin;
        executing_command_builtin = old_command_builtin;
    }
    discard_unwind_frame(
        b"simple-command\0" as *const u8 as *mut libc::c_char,
    );
    this_command_name = 0 as *mut libc::c_char;
    return result;
}

unsafe extern "C" fn builtin_status(mut result: libc::c_int) -> libc::c_int {
    let mut r: libc::c_int = 0;

    match result as libc::c_uint{
        EX_USAGE | EX_BADSYNTAX => {
            r = EX_BADUSAGE as libc::c_int;
        }
        EX_REDIRFAIL | EX_BADASSIGN | EX_EXPFAIL => {
            r = EXECUTION_FAILURE as libc::c_int;
        }
        _ => {
            r = if result > EX_SHERRBASE as libc::c_int {
                EXECUTION_FAILURE as libc::c_int
            } else {
                0 as libc::c_int
            };
        }
    }
    return r;
}



#[macro_export]
macro_rules! TRAP_STRING {
    ($s:expr) => {
        if signal_is_trapped($s) != 0
            && signal_is_ignored($s) == 0
        {
            *trap_list
                .as_mut_ptr()
                .offset(
                    ($s) as isize,
                )
        } else {
            0 as *mut libc::c_char
        }
    };
}

#[macro_export]
macro_rules! unwind_protect_int {
    ($var:expr) => {
        unwind_protect_mem(
            &mut $var as *mut libc::c_int as *mut libc::c_char,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        );
    };
}

unsafe extern "C" fn execute_builtin(
    // mut builtin: Option::<sh_builtin_func_t>,
    mut builtin: sh_builtin_func_t,
    mut words: *mut WordList,
    mut flags: libc::c_int,
    mut subshell: libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut eval_unwind: libc::c_int = 0;
    let mut ignexit_flag: libc::c_int = 0;
    let mut isbltinenv: libc::c_int = 0;
    let mut should_keep: libc::c_int = 0;
    let mut error_trap: *mut libc::c_char = 0 as *mut libc::c_char;

    error_trap = 0 as *mut libc::c_char;
    should_keep = 0 as libc::c_int;

    
    if subshell == 0 && flags & CMD_IGNORE_RETURN as libc::c_int != 0
    && (builtin
        == Some(eval_builtin as unsafe extern "C" fn(*mut WordList) -> libc::c_int)
        || flags & 0x800 as libc::c_int != 0
        || builtin
            == Some(
                source_builtin as unsafe extern "C" fn(*mut WordList) -> libc::c_int,
            ))
    {
        begin_unwind_frame(
            b"eval_builtin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        unwind_protect_mem(
            &mut exit_immediately_on_error as *mut libc::c_int as *mut libc::c_char,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        );
        unwind_protect_mem(
            &mut builtin_ignoring_errexit as *mut libc::c_int as *mut libc::c_char,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        );
        error_trap = TRAP_STRING!(ERROR_TRAP as c_int);
        if !error_trap.is_null() { 
            error_trap = savestring!(error_trap);
            add_unwind_protect(transmute::<
                unsafe extern "C" fn (arg1: *mut ::std::os::raw::c_void),
                *mut Function,
                >(xfree),
                error_trap,
            );
            add_unwind_protect(transmute::<
                unsafe extern "C" fn (arg1: *mut ::std::os::raw::c_char),
                *mut Function,
                >(set_error_trap),
                error_trap,
            );
            restore_default_signal(ERROR_TRAP as c_int);
        }
        exit_immediately_on_error = 0 ;
        ignexit_flag = builtin_ignoring_errexit;
        builtin_ignoring_errexit = 1 ;
        eval_unwind = 1 ;
    } else {
        eval_unwind = 0 ;
    }
    
    isbltinenv = (builtin
        == Some(source_builtin as unsafe extern "C" fn(*mut WordList) -> libc::c_int)
        || builtin
            == Some(eval_builtin as unsafe extern "C" fn(*mut WordList) -> libc::c_int)
        || builtin
            == Some(unset_builtin as unsafe extern "C" fn(*mut WordList) -> libc::c_int)
        || builtin
            == Some(
                mapfile_builtin as unsafe extern "C" fn(*mut WordList) -> libc::c_int,
            )) as libc::c_int;
    should_keep = (isbltinenv != 0
        && builtin
            != Some(
                mapfile_builtin as unsafe extern "C" fn(*mut WordList) -> libc::c_int,
            )) as libc::c_int;
    if builtin == Some(fc_builtin as unsafe extern "C" fn(*mut WordList) -> libc::c_int)
        || builtin
        == Some(read_builtin as unsafe extern "C" fn(*mut WordList) -> libc::c_int)
    {
        isbltinenv = 1 ;
        should_keep = 0 ;
    }
    
    if isbltinenv != 0 {
        if subshell == 0 {
            begin_unwind_frame(
                b"builtin_env\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        if !temporary_env.is_null() {
            push_scope(VC_BLTNENV as libc::c_int, temporary_env);
            if flags & CMD_COMMAND_BUILTIN as libc::c_int != 0 {
                should_keep = 0 ;
            }
            if subshell == 0 {
                add_unwind_protect(transmute::<
                    unsafe extern "C" fn (arg1: ::std::os::raw::c_int),
                    *mut Function,
                >(pop_scope),
                    if should_keep != 0 {
                        b"1\0" as *const u8 as *mut libc::c_char
                    } else {
                        0 as *mut libc::c_char
                    },
                );
            }
            temporary_env = 0 as *mut HASH_TABLE;
        }
    }

    if subshell == 0 
    && builtin
        == Some(eval_builtin as unsafe extern "C" fn(*mut WordList) -> libc::c_int)
    {
        if evalnest_max > 0 && evalnest >= evalnest_max {
            internal_error(b"eval: maximum eval nesting level exceeded (%d)\0" as *const u8 as *mut c_char,
                evalnest,
            );
            evalnest = 0 ;
            jump_to_top_level(DISCARD as libc::c_int);
        }
        unwind_protect_int!(evalnest);
        evalnest += 1;
    } else if subshell == 0 
    && builtin
        == Some(
            source_builtin as unsafe extern "C" fn(*mut WordList) -> libc::c_int,
        )
        {
        if sourcenest_max > 0 && sourcenest >= sourcenest_max {
            internal_error(
                b"%s: maximum source nesting level exceeded (%d)\0" as *const u8 as *mut c_char,     
                this_command_name,
                sourcenest,
            );
            sourcenest = 0 ;
            jump_to_top_level(DISCARD as libc::c_int);
        }
        unwind_protect_int!(sourcenest);
        sourcenest += 1;
    }
    
    if posixly_correct != 0 && subshell == 0  
        && builtin
            == Some(
                return_builtin as unsafe extern "C" fn(*mut WordList) -> libc::c_int,
            ) && flags & 0x800 as libc::c_int == 0 as libc::c_int
        && !temporary_env.is_null()
    {
        begin_unwind_frame(
            b"return_temp_env\0" as *const u8 as *mut libc::c_char,
        );
        add_unwind_protect(transmute::<
                unsafe extern "C" fn (),
                *mut Function,
            >(merge_temporary_env),
            0 as *mut libc::c_char,
        );
    }
    
    executing_builtin += 1;
    executing_command_builtin
        |= (builtin
            == Some(
                command_builtin as unsafe extern "C" fn(*mut WordList) -> libc::c_int,
            )) as libc::c_int;

    result = r_exec_cmd((*(*words).word).word, (*words).next);

    if posixly_correct != 0 && subshell == 0 
    && builtin
        == Some(
            return_builtin as unsafe extern "C" fn(*mut WordList) -> libc::c_int,
        ) && !temporary_env.is_null()
    {
        discard_unwind_frame(
            b"return_temp_env\0" as *const u8 as *mut libc::c_char,
        );
    }
    if subshell == 0 && isbltinenv != 0 {
        run_unwind_frame(
            b"builtin_env\0" as *const u8 as *mut libc::c_char,
        );
    }
    if eval_unwind != 0 {
        builtin_ignoring_errexit = ignexit_flag;
        exit_immediately_on_error = if builtin_ignoring_errexit != 0 {
            0  
        } else {
            errexit_flag
        };
        if !error_trap.is_null() {
            set_error_trap(error_trap);
            free(error_trap as *mut c_void);
        }
        discard_unwind_frame(
            b"eval_builtin\0" as *const u8 as *mut libc::c_char,
        );
    }
    return result;
}

unsafe extern "C" fn maybe_restore_getopt_state(mut gs: *mut sh_getopt_state_t) {
    if (*gs).gs_flags & 1 != 0 {
        sh_getopt_restore_istate(gs);
    } else {
        free(gs as *mut c_void);
    };
}

#[macro_export]
macro_rules! array_pop {
    ($a:expr) => {
        array_dispose_element(
            array_shift(($a), 1  , 0 ),
        );
    };
}

#[macro_export]
macro_rules! array_cell {
    ($var:expr) => {
        (*$var).value as *mut ARRAY
    };
}

#[macro_export]
macro_rules! GET_ARRAY_FROM_VAR {
    ($n:expr, $v:expr, $a:expr) => {
        $v = find_variable($n);
        $a = if !$v.is_null() && array_p!($v) != 0 {
            array_cell!($v)
        } else {
            0 as *mut ARRAY
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn restore_funcarray_state(mut fa: *mut func_array_state) {
    let mut nfv: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut funcname_a: *mut ARRAY = 0 as *mut ARRAY;

    array_pop!((*fa).source_a);
    array_pop!((*fa).lineno_a);

    GET_ARRAY_FROM_VAR!(b"FUNCNAME\0" as *const u8 as *const libc::c_char, nfv, funcname_a);
    if nfv == (*fa).funcname_v {
        array_pop!(funcname_a);
    }
    free(fa as *mut c_void);
}

// #[macro_export]
// macro_rules! USE_VAR {
//     ($x:expr) => {
//         (&mut $x) as c_void
//     };
// }

#[macro_export]
macro_rules! trace_p {
    ($var:expr) => {
        (*$var).attributes & att_trace as i32
    };
}

#[macro_export]
macro_rules! array_push {
    ($a:expr, $v:expr) => {
        array_rshift($a, 1, $v)
    };
}

unsafe extern "C" fn execute_function(
    mut var: *mut SHELL_VAR,
    mut words: *mut WordList,
    mut flags: libc::c_int,
    mut fds_to_close: *mut fd_bitmap,
    mut async_0: libc::c_int,
    mut subshell: libc::c_int,
) -> libc::c_int {
    let mut return_val: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    let mut tc: *mut COMMAND = 0 as *mut COMMAND;
    let mut fc: *mut COMMAND = 0 as *mut COMMAND;
    let mut save_current: *mut COMMAND = 0 as *mut COMMAND;
    let mut debug_trap: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut error_trap: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut return_trap: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut funcname_v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut bash_source_v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut bash_lineno_v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut funcname_a: *mut ARRAY = 0 as *mut ARRAY;
    let mut bash_source_a: *mut ARRAY = 0 as *mut ARRAY;
    let mut bash_lineno_a: *mut ARRAY = 0 as *mut ARRAY;
    let mut fa: *mut func_array_state = 0 as *mut func_array_state;
    let mut shell_fn: *mut FUNCTION_DEF = 0 as *mut FUNCTION_DEF;
    let mut sfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut gs: *mut sh_getopt_state_t = 0 as *mut sh_getopt_state_t;
    let mut gv: *mut SHELL_VAR = 0 as *mut SHELL_VAR;

    // USE_VAR!(fc);

    if funcnest_max > 0 && funcnest >= funcnest_max {
        internal_error(
            b"%s: maximum function nesting level exceeded (%d)\0" as *const u8 as *mut c_char,
            (*var).name,
            funcnest,
        );
        funcnest = 0 ;
        jump_to_top_level(DISCARD as libc::c_int);
    }

    GET_ARRAY_FROM_VAR!(b"FUNCNAME\0" as *const u8 as *const libc::c_char,funcname_v, funcname_a);
    GET_ARRAY_FROM_VAR!(b"BASH_SOURCE" as *const u8 as *const libc::c_char, bash_source_v, bash_source_a);
    GET_ARRAY_FROM_VAR!(b"BASH_LINENO" as *const u8 as *const libc::c_char, bash_lineno_v, bash_lineno_a);
    
    tc = copy_command((*var).value as *mut COMMAND);
    if !tc.is_null() && flags & CMD_IGNORE_RETURN as libc::c_int != 0 {
        (*tc).flags |= CMD_IGNORE_RETURN as libc::c_int;
    }

    if !tc.is_null() && flags & CMD_NO_FORK as libc::c_int != 0
        && subshell_environment & SUBSHELL_COMSUB as libc::c_int != 0
    {
        optimize_shell_function(tc);
    }

    gs = sh_getopt_save_istate();
    if subshell == 0 {
        begin_unwind_frame(
            b"function_calling\0" as *const u8 as *mut libc::c_char,
        );
        push_context((*var).name, subshell, temporary_env);

        add_unwind_protect(
            transmute::<
                unsafe extern "C" fn(*mut sh_getopt_state_t) -> (),
                *mut Function,
            >(maybe_restore_getopt_state),
            gs as *mut c_char,
        );
        add_unwind_protect(
            transmute::<
                unsafe extern "C" fn  (),
                *mut Function,
            >(pop_context),
            0 as *mut libc::c_char,
        );
        unwind_protect_int!(line_number);
        unwind_protect_int!(line_number_for_err_trap);
        unwind_protect_int!(function_line_number);
        unwind_protect_int!(return_catch_flag);

        unwind_protect_mem(
            &mut return_catch as *mut sigjmp_buf as *mut libc::c_char,
            ::std::mem::size_of::<sigjmp_buf>() as libc::c_ulong as libc::c_int,
        );

        add_unwind_protect(
            transmute::<
                unsafe extern "C" fn (arg1: *mut COMMAND),
                *mut Function,
            >(dispose_command),
            tc as *mut libc::c_char,
        );
        unwind_protect_mem(
            &mut this_shell_function as *mut *mut SHELL_VAR as *mut libc::c_char,
            ::std::mem::size_of::<*mut SHELL_VAR>() as libc::c_ulong as libc::c_int,
        );
        unwind_protect_int!(funcnest);
        unwind_protect_int!(loop_level);
    } else {
        push_context((*var).name, subshell, temporary_env);
    }

    temporary_env = 0 as *mut HASH_TABLE;

    this_shell_function = var;
    make_funcname_visible(1 );

    debug_trap = TRAP_STRING!(DEBUG_TRAP as c_int);
    error_trap = TRAP_STRING!(ERROR_TRAP as c_int);
    return_trap = TRAP_STRING!(RETURN_TRAP as c_int);

    if !debug_trap.is_null()
        && (trace_p!(var) == 0
            && function_trace_mode == 0 )
    {
        if subshell == 0 {
            debug_trap = savestring!(debug_trap);
            add_unwind_protect(
                transmute::<
                    unsafe extern "C" fn (arg1: *mut ::std::os::raw::c_void),
                    *mut Function,
                >(xfree),
                debug_trap,
            );
            add_unwind_protect(
                transmute::<
                unsafe extern "C" fn (arg1: *mut ::std::os::raw::c_char),
                    *mut Function,
                >(maybe_set_debug_trap),
                debug_trap,
            );
        }
        restore_default_signal(DEBUG_TRAP as libc::c_int);
    }

    if !error_trap.is_null() && error_trace_mode == 0 {
        if subshell == 0  {
            error_trap = savestring!(error_trap);
            add_unwind_protect(
                transmute::<
                unsafe extern "C" fn (arg1: *mut ::std::os::raw::c_void),
                    *mut Function,
                >(xfree), 
                error_trap);
            add_unwind_protect(
                transmute::<
                    unsafe extern "C" fn (arg1: *mut ::std::os::raw::c_char),
                    *mut Function,
                >(maybe_set_error_trap),
                error_trap
            );
        }
        restore_default_signal(ERROR_TRAP as libc::c_int);
    }

    if !return_trap.is_null()
        && (signal_in_progress(DEBUG_TRAP as libc::c_int) != 0
            || trace_p!(var) == 0
                && function_trace_mode == 0 )
    {
        if subshell == 0 {
            return_trap = savestring!(return_trap);
            add_unwind_protect(
                transmute::<
                unsafe extern "C" fn (arg1: *mut ::std::os::raw::c_void),
                    *mut Function,
                >(xfree), 
                return_trap);
            add_unwind_protect(
                transmute::<
                    unsafe extern "C" fn (arg1: *mut ::std::os::raw::c_char),
                    *mut Function,
                >(maybe_set_return_trap),
                return_trap
            );
        }
        restore_default_signal( RETURN_TRAP as libc::c_int);
    }

    funcnest += 1;

    shell_fn = find_function_def((*this_shell_function).name);
    sfile = (if !shell_fn.is_null() {
        (*shell_fn).source_file
    } else {
        b"\0" as *const u8 as *const libc::c_char
    }) as *mut libc::c_char;
    array_push!(funcname_a, (*this_shell_function).name);
    array_push!(bash_source_a, sfile);
    t = itos(executing_line_number() as intmax_t);
    array_push!(bash_lineno_a, t);
    free(t as *mut c_void);

    fa = xmalloc(size_of::<func_array_state>()) as *mut func_array_state;
    (*fa).source_a = bash_source_a as *mut ARRAY;
    (*fa).source_v = bash_source_v;
    (*fa).lineno_a = bash_lineno_a as *mut ARRAY;
    (*fa).lineno_v = bash_lineno_v;
    (*fa).funcname_a = funcname_a;
    (*fa).funcname_v = funcname_v;

    if subshell == 0 as libc::c_int {
        add_unwind_protect(
            transmute::<
                unsafe extern "C" fn(*mut func_array_state) -> (),
                *mut Function,
                >(restore_funcarray_state),
            fa as *mut c_char,
        );
    }

    if debugging_mode != 0 || shell_compatibility_level <= 44 {
        init_bash_argv();
    }

    remember_args((*words).next, 1 );

    if debugging_mode != 0 {
        push_args((*words).next);
        if subshell == 0 {
            add_unwind_protect(
                transmute::<
                    unsafe extern "C" fn(),
                    *mut Function,
                >(pop_args),
                0 as *mut c_char,
            );
        }
    }

    function_line_number = (*tc).line;
    line_number = function_line_number;

    if subshell != 0 {
        stop_pipeline(async_0, 0 as *mut COMMAND);
    }
    if shell_compatibility_level > 43 {
        loop_level = 0 ;
    }

    fc = tc;

    from_return_trap = 0;

    return_catch_flag += 1;
    return_val = setjmp_nosigs!(return_catch);
    
    if return_val != 0 {
        result = return_catch_value;
        save_current = currently_executing_command;
        if from_return_trap == 0 {
            run_return_trap();
        }
        currently_executing_command = save_current;
    } else {
        showing_function_line = 1 ;
        save_current = currently_executing_command;
        result = run_debug_trap();
        if debugging_mode == 0 || result == EXECUTION_SUCCESS as c_int {
            showing_function_line = 0 ;
            currently_executing_command = save_current;
            result = execute_command_internal(
                fc,
                0 ,
                NO_PIPE,
                NO_PIPE,
                fds_to_close,
            );

            save_current = currently_executing_command;
            run_return_trap();
            currently_executing_command = save_current;
        }

        showing_function_line = 0 ;
    }
    gv = find_variable(b"OPTIND\0" as *const u8 as *const libc::c_char);
    if !gv.is_null() && (*gv).context == variable_context {
        (*gs).gs_flags |= 1 ;
    }

    if subshell == 0 {
        run_unwind_frame(
            b"function_calling\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    } else {
        restore_funcarray_state(fa);
        if debugging_mode != 0 {
            pop_args();
        }
    }
    if variable_context == 0 || this_shell_function.is_null() {
        make_funcname_visible(0 );
        unlink_fifo_list();
    }
    return result;
}

#[no_mangle]
pub unsafe extern "C" fn execute_shell_function(
    mut var: *mut SHELL_VAR,
    mut words: *mut WordList,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut bitmap: *mut fd_bitmap = 0 as *mut fd_bitmap;

    bitmap = new_fd_bitmap(FD_BITMAP_DEFAULT_SIZE!());
    begin_unwind_frame(
        b"execute-shell-function\0" as *const u8 as *mut libc::c_char,
    );
    add_unwind_protect(
        transmute::<
        unsafe extern "C" fn (fdbp: *mut fd_bitmap),
            *mut Function,
        >(dispose_fd_bitmap),
        bitmap as *mut libc::c_char,
    );

    ret = execute_function(
        var,
        words,
        0 ,
        bitmap,
        0 ,
        0 ,
    );

    dispose_fd_bitmap(bitmap);
    discard_unwind_frame(
        b"execute-shell-function\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return ret;
}




















