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
