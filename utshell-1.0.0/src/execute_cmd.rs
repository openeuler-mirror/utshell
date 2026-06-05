use crate::src_common::*;

use crate::alias::delete_all_aliases;
use crate::arrayfunc::{bind_array_variable, convert_var_to_array};
use crate::builtins::command::command_builtin;
use crate::builtins::common::{
    builtin_address, builtin_address_internal, find_shell_builtin, find_special_builtin,
    get_job_by_name, remember_args, sh_invalidid,
};
use crate::builtins::eval::eval_builtin;
use crate::builtins::exec::exec_builtin;
use crate::builtins::exec_cmd::exec_cmd;
use crate::builtins::fc::fc_builtin;
use crate::builtins::jobs::jobs_builtin;
use crate::builtins::mapfile::mapfile_builtin;
use crate::builtins::read::read_builtin;
use crate::builtins::return_1::return_builtin;
use crate::builtins::set::{reset_shell_options, unset_builtin};
use crate::builtins::shopt::reset_shopt_options;
use crate::builtins::source::source_builtin;
use crate::copycmd::{copy_command, copy_word_list};
use crate::dispose_cmd::{dispose_command, dispose_redirects, dispose_words};
use crate::error::err_readonly;
use crate::error::{command_error, file_error};
use crate::expr::evalexp;
use crate::findcmd::{executable_file, search_for_command};
use crate::flags::{change_flag, reset_shell_flags};
use crate::general::valid_nameref_value;
use crate::general::{
    check_binary_file, check_identifier, default_columns, file_isdir, legal_identifier,
    legal_number, move_to_high_fd, printable_filename, sh_openpipe,
};
use crate::input::fd_is_bash_input;
use crate::jobs::{
    append_process, freeze_jobs_list, init_job_stats, job_exit_status, kill_current_pipeline,
    make_child, set_jobs_list_frozen, set_sigchld_handler, start_job, stop_pipeline,
    terminate_current_pipeline, wait_for, wait_for_single_pid, without_job_control, BLOCK_CHILD,
    UNBLOCK_CHILD,
};
use crate::list::list_length;
use crate::local::locale_decpoint;
use crate::make_cmd::{make_word, make_word_list};
use crate::optimize_fork;
use crate::optimize_shell_function;
use crate::pathexp::quote_string_for_globbing;
use crate::print_cmd::make_command_string;
use crate::print_cmd::{
    print_arith_command, print_case_command_head, print_cond_command, print_for_command_head,
    print_select_command_head, print_simple_command, xtrace_print_arith_cmd,
    xtrace_print_case_command_head, xtrace_print_cond_term, xtrace_print_for_command_head,
    xtrace_print_select_command_head, xtrace_print_word_list,
};
use crate::readline::c_clearerr;
use crate::redir::{do_redirections, stdin_redirects};
use crate::sh_getopt_restore_istate;
use crate::sh_getopt_save_istate;
use crate::sig::{
    jump_to_top_level, reset_terminating_signals, set_signal_handler, throw_to_top_level,
};
use crate::stringlib::{substring, xbcopy};
use crate::subst::{
    clear_fifo_list, close_new_fifos, cond_expand_word, copy_fifo_list, dequote_escapes,
    dequote_string, expand_arith_string, expand_string_unsplit_to_string, expand_word_leave_quoted,
    expand_words, expand_words_no_vars, fifos_pending, num_fifos, setifs, string_list,
    unlink_fifo_list,
};
use crate::test::{binary_test, unary_test};
use crate::trap::{
    clear_pending_traps, get_original_signal, maybe_set_debug_trap, maybe_set_error_trap,
    maybe_set_return_trap, reset_signal_handlers, restore_default_signal, restore_original_signals,
    run_debug_trap, run_error_trap, run_exit_trap, run_pending_traps, run_return_trap,
    run_trap_cleanup, set_error_trap, set_sigint_handler, signal_in_progress, signal_is_ignored,
    signal_is_trapped,
};
use crate::unwind_prot::{
    add_unwind_protect, begin_unwind_frame, clear_unwind_protect_list, discard_unwind_frame,
    run_unwind_frame, unwind_protect_mem, unwind_protect_tag_on_stack,
};
use crate::utshell::sh_exit;
use crate::utshell::subshell_exit;
use crate::utshell::unbind_args;
use crate::utshell::unset_bash_input;
use crate::variables::bind_function;
use crate::variables::bind_function_def;
use crate::variables::{
    adjust_shell_level, bind_variable, bind_variable_value, check_unbind_variable,
    dispose_used_env_vars, find_function, find_function_def, find_variable,
    find_variable_last_nameref, find_variable_nameref_for_create, get_string_value, init_bash_argv,
    make_funcname_visible, make_new_array_variable, maybe_make_export_env, merge_temporary_env,
    pop_args, pop_context, pop_scope, push_args, push_context, push_scope,
    put_command_name_into_env, set_pipestatus_from_exit, stupidly_hack_special_variables,
    unbind_variable_noref,
};
use crate::version::shell_compatibility_level;
use crate::y_tab::{expand_aliases, line_number, reset_parser};

/* Functions to allocate and deallocate the structures used to pass
information from the shell to its children about file descriptors
to close. */
#[no_mangle]
pub fn new_fd_bitmap(size: libc::c_int) -> *mut fd_bitmap {
    let ret: *mut fd_bitmap;
    unsafe {
        ret = malloc(size_of::<fd_bitmap>() as usize) as *mut fd_bitmap;

        (*ret).size = size;

        if size != 0 {
            (*ret).bitmap = malloc(size as usize) as *mut libc::c_char;
            memset(
                (*ret).bitmap as *mut libc::c_void,
                '\u{0}' as i32,
                size as usize,
            );
        } else {
            (*ret).bitmap = 0 as *mut libc::c_char;
        }
    }
    return ret;
}

#[no_mangle]
pub fn dispose_fd_bitmap(fdbp: *mut fd_bitmap) {
    unsafe {
        FREE!((*fdbp).bitmap);
        free(fdbp as *mut c_void);
    }
}

#[no_mangle]
pub fn close_fd_bitmap(fdbp: *mut fd_bitmap) {
    let mut i: libc::c_int;

    if !fdbp.is_null() {
        i = 0;
        unsafe {
            while i < (*fdbp).size {
                if *((*fdbp).bitmap).offset(i as isize) != 0 {
                    close(i);
                    *((*fdbp).bitmap).offset(i as isize) = 0 as libc::c_char;
                }
                i += 1;
            }
        }
    }
}

#[no_mangle]
pub fn executing_line_number() -> libc::c_int {
    unsafe {
        if executing != 0
            && showing_function_line == 0
            && (variable_context == 0 || interactive_shell == 0)
            && !currently_executing_command.is_null()
        {
            if (*currently_executing_command).type_0 as libc::c_uint
                == command_type_cm_cond as libc::c_uint
            {
                return (*(*currently_executing_command).value.Cond).line;
            }
            if (*currently_executing_command).type_0 as libc::c_uint
                == command_type_cm_arith as libc::c_uint
            {
                return (*(*currently_executing_command).value.Arith).line;
            }
            if (*currently_executing_command).type_0 as libc::c_uint
                == command_type_cm_arith_for as libc::c_uint
            {
                return (*(*currently_executing_command).value.ArithFor).line;
            }
            return line_number;
        } else {
            return line_number;
        };
    }
}

#[no_mangle]
pub fn execute_command(command: *mut COMMAND) -> libc::c_int {
    let bitmap: *mut fd_bitmap;
    let result: libc::c_int;

    unsafe {
        current_fds_to_close = 0 as *mut fd_bitmap;
    }
    bitmap = new_fd_bitmap(FD_BITMAP_DEFAULT_SIZE!());
    begin_unwind_frame(b"execute-command\0" as *const u8 as *mut libc::c_char);
    unsafe {
        add_unwind_protect(
            ::std::mem::transmute::<fn(*mut fd_bitmap) -> (), Option<Function>>(dispose_fd_bitmap),
            bitmap as *mut libc::c_char,
        );
    }
    //执行内部命令
    result = execute_command_internal(command, 0, NO_PIPE, NO_PIPE, bitmap);

    dispose_fd_bitmap(bitmap);
    discard_unwind_frame(b"execute-command\0" as *const u8 as *mut libc::c_char);

    if unsafe { variable_context == 0 && executing_list == 0 } {
        unlink_fifo_list();
    }

    unsafe {
        QUIT!();
    }

    return result;
}

fn shell_control_structure(type_0: command_type) -> libc::c_int {
    match type_0 as libc::c_uint {
        command_type_cm_arith_for
        | command_type_cm_select
        | command_type_cm_arith
        | command_type_cm_cond
        | command_type_cm_case
        | command_type_cm_while
        | command_type_cm_until
        | command_type_cm_if
        | command_type_cm_for
        | command_type_cm_group
        | command_type_cm_function_def => return 1 as libc::c_int,

        _ => return 0 as libc::c_int,
    };
}

fn cleanup_redirects(list: *mut REDIRECT) {
    do_redirections(list, RX_ACTIVE as libc::c_int);
    dispose_redirects(list);
}

#[no_mangle]
pub fn undo_partial_redirects() {
    unsafe {
        if !redirection_undo_list.is_null() {
            cleanup_redirects(redirection_undo_list);
            redirection_undo_list = 0 as *mut REDIRECT;
        }
    }
}

#[no_mangle]
pub fn dispose_exec_redirects() {
    unsafe {
        if !exec_redirection_undo_list.is_null() {
            dispose_redirects(exec_redirection_undo_list);
            exec_redirection_undo_list = 0 as *mut REDIRECT;
        }
    }
}

#[no_mangle]
pub fn dispose_partial_redirects() {
    unsafe {
        if !redirection_undo_list.is_null() {
            dispose_redirects(redirection_undo_list);
            redirection_undo_list = 0 as *mut REDIRECT;
        }
    }
}

fn restore_signal_mask(set: *mut sigset_t) -> libc::c_int {
    return c_sigprocmask(SIG_SETMASK as libc::c_int, set, 0 as *mut sigset_t);
}

#[no_mangle]
pub fn async_redirect_stdin() {
    let fd: libc::c_int;
    unsafe {
        fd = open(
            b"/dev/null\0" as *const u8 as *const libc::c_char,
            O_RDONLY as libc::c_int,
        );
        if fd > 0 {
            dup2(fd, 0);
            close(fd);
        } else if fd < 0 {
            internal_error(
                b"cannot redirect standard input from /dev/null: %s\0" as *const u8
                    as *mut libc::c_char,
                strerror(*c___errno_location()),
            );
        }
    }
}

/* Execute the command passed in COMMAND, perhaps doing it asynchronously.
COMMAND is exactly what read_command () places into GLOBAL_COMMAND.
ASYNCHRONOUS, if non-zero, says to do this command in the background.
PIPE_IN and PIPE_OUT are file descriptors saying where input comes
from and where it goes.  They can have the value of NO_PIPE, which means
I/O is stdin/stdout.
FDS_TO_CLOSE is a list of file descriptors to close once the child has
been forked.  This list often contains the unusable sides of pipes, etc.

EXECUTION_SUCCESS or EXECUTION_FAILURE are the only possible
return values.  Executing a command with nothing in it returns
EXECUTION_SUCCESS. */
#[no_mangle]
pub fn execute_command_internal(
    command: *mut COMMAND,
    asynchronous: libc::c_int,
    pipe_in: libc::c_int,
    pipe_out: libc::c_int,
    fds_to_close: *mut fd_bitmap,
) -> libc::c_int {
    let mut exec_result: libc::c_int;
    let user_subshell: libc::c_int;
    let mut invert: libc::c_int;
    let ignore_return: libc::c_int;
    let mut was_error_trap: libc::c_int;
    let fork_flags: libc::c_int;
    let my_undo_list: *mut REDIRECT;
    let exec_undo_list: *mut REDIRECT;
    let tcmd: *mut libc::c_char;
    let mut save_line_number: libc::c_int = 0;
    let mut ofifo: libc::c_int = 0;
    let nfifo: libc::c_int;
    let mut osize: libc::c_int = 0;
    let saved_fifo: libc::c_int;
    let mut ofifo_list: *mut libc::c_void = 0 as *mut libc::c_void;
    unsafe {
        if breaking != 0 || continuing != 0 {
            return last_command_exit_value;
        }
        if command.is_null() || read_but_dont_execute != 0 && rpm_requires == 0 {
            return EXECUTION_SUCCESS as i32;
        }
        if rpm_requires != 0 && (*command).type_0 == command_type_cm_function_def {
            last_command_exit_value = execute_intern_function(
                (*(*command).value.Function_def).name,
                (*command).value.Function_def,
            );
            return last_command_exit_value;
        }
        if read_but_dont_execute != 0 {
            return EXECUTION_SUCCESS as libc::c_int;
        }

        QUIT!();
        run_pending_traps();

        currently_executing_command = command;

        invert = ((*command).flags & CMD_INVERT_RETURN as libc::c_int != 0) as libc::c_int;

        if exit_immediately_on_error != 0 && invert != 0 {
            (*command).flags |= CMD_IGNORE_RETURN as libc::c_int;
        }

        exec_result = EXECUTION_SUCCESS as libc::c_int;

        if (*command).type_0 == command_type_cm_subshell
            && (*command).flags & CMD_NO_FORK as libc::c_int != 0
        {
            return execute_in_subshell(command, asynchronous, pipe_in, pipe_out, fds_to_close);
        }
        if (*command).type_0 == command_type_cm_coproc {
            last_command_exit_value = execute_coproc(command, pipe_in, pipe_out, fds_to_close);
            return last_command_exit_value;
        }

        user_subshell = ((*command).type_0 == command_type_cm_subshell
            || (*command).flags & CMD_WANT_SUBSHELL as libc::c_int != 0)
            as libc::c_int;

        if (*command).type_0 == command_type_cm_subshell
            || (*command).flags
                & (CMD_WANT_SUBSHELL as libc::c_int | CMD_FORCE_SUBSHELL as libc::c_int)
                != 0
            || shell_control_structure((*command).type_0 as libc::c_uint) != 0
                && (pipe_out != NO_PIPE || pipe_in != NO_PIPE || asynchronous != 0)
        {
            let paren_pid: pid_t;
            let mut s: libc::c_int;
            let p: *mut libc::c_char;

            save_line_number = line_number;
            if (*command).type_0 == command_type_cm_subshell {
                line_number = (*(*command).value.Subshell).line;
                line_number_for_err_trap = line_number;
            }

            tcmd = make_command_string(command);
            fork_flags = if asynchronous != 0 {
                FORK_ASYNC as libc::c_int
            } else {
                0
            };
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
                    && (*command).type_0 == command_type_cm_group
                    && pipe_in == NO_PIPE
                    && pipe_out == NO_PIPE
                    && asynchronous != 0) as libc::c_int;

                s += (user_subshell == 0
                    && (*command).type_0 == command_type_cm_group
                    && (pipe_in != NO_PIPE || pipe_out != NO_PIPE)
                    && asynchronous == 0) as libc::c_int;

                last_command_exit_value =
                    execute_in_subshell(command, asynchronous, pipe_in, pipe_out, fds_to_close);
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
                    return EXECUTION_SUCCESS as libc::c_int;
                }

                stop_pipeline(asynchronous, 0 as *mut COMMAND);

                line_number = save_line_number;

                if asynchronous == 0 {
                    was_error_trap = (signal_is_trapped(ERROR_TRAP as libc::c_int) != 0
                        && signal_is_ignored(ERROR_TRAP as libc::c_int) == 0)
                        as libc::c_int;
                    invert =
                        ((*command).flags & CMD_INVERT_RETURN as libc::c_int != 0) as libc::c_int;
                    ignore_return =
                        ((*command).flags & CMD_IGNORE_RETURN as libc::c_int != 0) as libc::c_int;

                    exec_result = wait_for(paren_pid, 0);

                    if invert != 0 {
                        exec_result = if exec_result == EXECUTION_SUCCESS as libc::c_int {
                            EXECUTION_FAILURE as libc::c_int
                        } else {
                            EXECUTION_SUCCESS as libc::c_int
                        };
                    }

                    last_command_exit_value = exec_result;
                    if user_subshell != 0
                        && was_error_trap != 0
                        && ignore_return == 0
                        && invert == 0
                        && exec_result != EXECUTION_SUCCESS as libc::c_int
                    {
                        save_line_number = line_number;
                        line_number = line_number_for_err_trap;
                        run_error_trap();
                        line_number = save_line_number;
                    }

                    if user_subshell != 0
                        && ignore_return == 0
                        && invert == 0
                        && exit_immediately_on_error != 0
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
        if (*command).flags & CMD_TIME_PIPELINE as libc::c_int != 0 {
            if asynchronous != 0 {
                (*command).flags |= CMD_FORCE_SUBSHELL as libc::c_int;
                exec_result = execute_command_internal(command, 1, pipe_in, pipe_out, fds_to_close);
            } else {
                exec_result = time_command(command, asynchronous, pipe_in, pipe_out, fds_to_close);
                currently_executing_command = 0 as *mut COMMAND;
            }
            return exec_result;
        }
        if shell_control_structure((*command).type_0) != 0 && !((*command).redirects).is_null() {
            stdin_redir = stdin_redirects((*command).redirects);
        }

        if variable_context != 0 || executing_list != 0 {
            ofifo = num_fifos();
            ofifo_list = copy_fifo_list(&mut osize as *mut libc::c_int);
            begin_unwind_frame(b"internal_fifos\0" as *const u8 as *mut libc::c_char);
            if !ofifo_list.is_null() {
                add_unwind_protect(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(*mut c_void) -> (),
                        Option<Function>,
                    >(libc::free),
                    ofifo_list as *mut libc::c_char,
                );
            }
            saved_fifo = 1;
        } else {
            saved_fifo = 0;
        }

        was_error_trap = (signal_is_trapped(ERROR_TRAP as libc::c_int) != 0
            && signal_is_ignored(ERROR_TRAP as libc::c_int) == 0)
            as libc::c_int;
        ignore_return = ((*command).flags & CMD_IGNORE_RETURN as libc::c_int != 0) as libc::c_int;

        if do_redirections(
            (*command).redirects,
            RX_ACTIVE as libc::c_int | RX_UNDOABLE as libc::c_int,
        ) != 0
        {
            undo_partial_redirects();
            dispose_exec_redirects();
            if saved_fifo != 0 {
                free(ofifo_list as *mut c_void);
                discard_unwind_frame(b"internal_fifos\0" as *const u8 as *mut libc::c_char);
            }

            last_command_exit_value = EXECUTION_FAILURE as libc::c_int;
            if ignore_return == 0 && invert == 0 && pipe_in == NO_PIPE && pipe_out == NO_PIPE {
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
            begin_unwind_frame(b"loop_redirections\0" as *const u8 as *mut libc::c_char);
        }
        if !my_undo_list.is_null() {
            add_unwind_protect(
                std::mem::transmute::<fn(*mut REDIRECT) -> (), Option<Function>>(cleanup_redirects),
                my_undo_list as *mut libc::c_char,
            );
        }
        if !exec_undo_list.is_null() {
            add_unwind_protect(
                transmute::<fn(arg1: *mut REDIRECT) -> (), Option<Function>>(dispose_redirects),
                exec_undo_list as *mut libc::c_char,
            );
        }

        QUIT!();

        match (*command).type_0 {
            command_type_cm_simple => {
                save_line_number = line_number;
                was_error_trap = (signal_is_trapped(ERROR_TRAP as libc::c_int) != 0
                    && signal_is_ignored(ERROR_TRAP as libc::c_int) == 0)
                    as libc::c_int;

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

                if was_error_trap != 0
                    && ignore_return == 0
                    && invert == 0
                    && pipe_in == NO_PIPE
                    && pipe_out == NO_PIPE
                    && (*(*command).value.Simple).flags & CMD_COMMAND_BUILTIN as libc::c_int == 0
                    && exec_result != EXECUTION_SUCCESS as libc::c_int
                {
                    last_command_exit_value = exec_result;
                    line_number = line_number_for_err_trap;
                    run_error_trap();
                    line_number = save_line_number;
                }

                if ignore_return == 0
                    && invert == 0
                    && (posixly_correct != 0 && interactive == 0 && special_builtin_failed != 0
                        || exit_immediately_on_error != 0
                            && pipe_in == NO_PIPE
                            && pipe_out == NO_PIPE
                            && exec_result != EXECUTION_SUCCESS as libc::c_int)
                {
                    last_command_exit_value = exec_result;
                    run_pending_traps();
                    if exit_immediately_on_error != 0
                        && signal_is_trapped(0) != 0
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
                    exec_result =
                        execute_command_internal(command, 1, pipe_in, pipe_out, fds_to_close);
                } else {
                    if ignore_return != 0 && !((*(*command).value.Group).command).is_null() {
                        (*(*(*command).value.Group).command).flags |=
                            CMD_IGNORE_RETURN as libc::c_int;
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
                exec_result =
                    execute_connection(command, asynchronous, pipe_in, pipe_out, fds_to_close);
                if asynchronous != 0 {
                    invert = 0;
                }
            }
            command_type_cm_arith | command_type_cm_cond | command_type_cm_function_def => {
                was_error_trap = (signal_is_trapped(ERROR_TRAP as libc::c_int) != 0
                    && signal_is_ignored(ERROR_TRAP as libc::c_int) == 0)
                    as libc::c_int;
                if ignore_return != 0 && (*command).type_0 == command_type_cm_arith {
                    (*(*command).value.Arith).flags |= CMD_IGNORE_RETURN as libc::c_int;
                }
                if ignore_return != 0 && (*command).type_0 == command_type_cm_cond {
                    (*(*command).value.Cond).flags |= CMD_IGNORE_RETURN as libc::c_int;
                }
                ::core::ptr::write_volatile(&mut save_line_number as *mut libc::c_int, line_number);
                line_number_for_err_trap = ::core::ptr::read_volatile::<libc::c_int>(
                    &save_line_number as *const libc::c_int,
                );

                if (*command).type_0 == command_type_cm_arith {
                    exec_result = execute_arith_command((*command).value.Arith);
                } else if (*command).type_0 == command_type_cm_cond {
                    exec_result = execute_cond_command((*command).value.Cond);
                } else if (*command).type_0 == command_type_cm_function_def {
                    exec_result = execute_intern_function(
                        (*(*command).value.Function_def).name,
                        (*command).value.Function_def,
                    );
                }
                line_number = save_line_number;
                if was_error_trap != 0
                    && ignore_return == 0
                    && invert == 0
                    && exec_result != EXECUTION_SUCCESS as libc::c_int
                {
                    last_command_exit_value = exec_result;
                    save_line_number = line_number;
                    line_number = line_number_for_err_trap;
                    run_error_trap();
                    line_number = save_line_number;
                }
                if ignore_return == 0
                    && invert == 0
                    && exit_immediately_on_error != 0
                    && exec_result != EXECUTION_SUCCESS as libc::c_int
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
                    (*command).type_0 as libc::c_int,
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
        match (*command).type_0 {
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
}

static mut precs: [libc::c_int; 4] = [
    0 as libc::c_int,
    100 as libc::c_int,
    10 as libc::c_int,
    1 as libc::c_int,
];

fn mkfmt(
    buf: *mut libc::c_char,
    prec: libc::c_int,
    lng: libc::c_int,
    mut sec: time_t,
    mut sec_fraction: libc::c_int,
) -> libc::c_int {
    let mut min: time_t;
    let mut abuf: [libc::c_char; 22] = [0; 22];
    let mut ind: libc::c_int;
    let mut aind: libc::c_int;

    ind = 0;
    abuf[(size_of::<[libc::c_char; 22]>()) - 1] = '\u{0}' as libc::c_char;

    if lng != 0 {
        min = sec / 60 as libc::c_long;
        sec %= 60 as libc::c_long;
        aind = (size_of::<[libc::c_char; 22]>() - 2) as libc::c_int;
        loop {
            //有可能aind的值不正确
            abuf[aind as usize] = (min % 10 + '0' as libc::c_long) as libc::c_char;
            aind = aind - 1;
            min /= 10 as libc::c_long;
            if !(min != 0) {
                break;
            }
        }
        aind += 1;
        while abuf[aind as usize] != 0 {
            //有可能ind，aind的值不正确
            unsafe {
                *buf.offset(ind as isize) = abuf[aind as usize];
            }
            aind = aind + 1;
            ind = ind + 1;
        }
        unsafe {
            *buf.offset(ind as isize) = 'm' as libc::c_char;
        }
        ind = ind + 1;
    }

    aind = (size_of::<[libc::c_char; 22]>() - 2) as libc::c_int;
    loop {
        abuf[aind as usize] = ((sec % 10) + '0' as libc::c_long) as libc::c_char;
        aind = aind - 1;
        sec /= 10;
        if !(sec != 0) {
            break;
        }
    }
    aind += 1;
    while abuf[aind as usize] != 0 {
        unsafe {
            *buf.offset(ind as isize) = abuf[aind as usize];
        }
        aind = aind + 1;
        ind = ind + 1;
    }

    if prec != 0 {
        unsafe {
            *buf.offset(ind as isize) = locale_decpoint() as libc::c_char;
        }
        ind = ind + 1;
        aind = 1;
        while aind <= prec {
            unsafe {
                *buf.offset(ind as isize) =
                    (sec_fraction / precs[aind as usize] + '0' as i32) as libc::c_char;
                ind = ind + 1;
                sec_fraction %= precs[aind as usize];
            }
            aind += 1;
        }
    }

    if lng != 0 {
        unsafe {
            *buf.offset(ind as isize) = 's' as libc::c_char;
        }
        ind = ind + 1;
    }
    unsafe {
        *buf.offset(ind as isize) = '\u{0}' as libc::c_char;
    }

    return ind;
}

fn print_formatted_time(
    fp: *mut FILE,
    format: *mut libc::c_char,
    rs: time_t,
    rsf: libc::c_int,
    us: time_t,
    usf: libc::c_int,
    ss: time_t,
    ssf: libc::c_int,
    cpu: libc::c_int,
) {
    let mut prec: libc::c_int;
    let mut lng: libc::c_int;
    let mut len: libc::c_int;
    let mut str: *mut libc::c_char;
    let mut s: *mut libc::c_char;
    let mut ts: [libc::c_char; 30] = [0; 30];
    let mut sum: time_t;
    let mut sum_frac: libc::c_int;
    let mut sindex: libc::c_int;
    let mut ssize: libc::c_int;
    unsafe {
        len = strlen(format) as libc::c_int;
        ssize = (len + 64) - (len % 64);
        str = malloc(ssize as usize) as *mut libc::c_char;
        sindex = 0;

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
                sum = (cpu / 100) as time_t;
                sum_frac = cpu % 100 * 10;
                len = mkfmt(ts.as_mut_ptr(), 2, 0, sum, sum_frac);
                RESIZE_MALLOCED_BUFFER!(str, sindex, len, ssize, 64);
                strcpy(str.offset(sindex as isize), ts.as_mut_ptr());
                sindex += len;
            } else {
                prec = 3;
                lng = 0;
                s = s.offset(1);
                if DIGIT!(*s) {
                    prec = *s as libc::c_int - '0' as i32;
                    s = s.offset(1);
                    if prec > 3 {
                        prec = 3;
                    }
                }
                if *s as libc::c_int == 'l' as i32 {
                    lng = 1;
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
                        b"TIMEFORMAT: `%c': invalid format character\0" as *const u8
                            as *mut libc::c_char,
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
}

fn time_command(
    command: *mut COMMAND,
    asynchronous: libc::c_int,
    pipe_in: libc::c_int,
    pipe_out: libc::c_int,
    fds_to_close: *mut fd_bitmap,
) -> libc::c_int {
    let mut rv: libc::c_int = 0;
    let posix_time: libc::c_int;
    let old_flags: libc::c_int;
    let nullcmd: libc::c_int;
    let code: libc::c_int;
    let mut rs: time_t;
    let mut us: time_t;
    let mut ss: time_t;
    let mut rsf: libc::c_int;
    let mut usf: libc::c_int;
    let mut ssf: libc::c_int;
    let mut cpu: libc::c_int;
    let mut time_format: *mut libc::c_char;
    let mut save_top_level: sigjmp_buf = [__jmp_buf_tag {
        __jmpbuf: [0; 8],
        __mask_was_saved: 0,
        __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1];
    let mut real: crate::src_common::timeval = crate::src_common::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut user: crate::src_common::timeval = crate::src_common::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut sys: crate::src_common::timeval = crate::src_common::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut before: crate::src_common::timeval = crate::src_common::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut after: crate::src_common::timeval = crate::src_common::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut dtz: crate::src_common::timezone = crate::src_common::timezone {
        tz_minuteswest: 0,
        tz_dsttime: 0,
    };
    let mut selfb: crate::src_common::rusage = crate::src_common::rusage {
        ru_utime: crate::src_common::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        ru_stime: crate::src_common::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        __bindgen_anon_1: rusage__bindgen_ty_1 { ru_maxrss: 0 },
        __bindgen_anon_2: rusage__bindgen_ty_2 { ru_ixrss: 0 },
        __bindgen_anon_3: rusage__bindgen_ty_3 { ru_idrss: 0 },
        __bindgen_anon_4: rusage__bindgen_ty_4 { ru_isrss: 0 },
        __bindgen_anon_5: rusage__bindgen_ty_5 { ru_minflt: 0 },
        __bindgen_anon_6: rusage__bindgen_ty_6 { ru_majflt: 0 },
        __bindgen_anon_7: rusage__bindgen_ty_7 { ru_nswap: 0 },
        __bindgen_anon_8: rusage__bindgen_ty_8 { ru_inblock: 0 },
        __bindgen_anon_9: rusage__bindgen_ty_9 { ru_oublock: 0 },
        __bindgen_anon_10: rusage__bindgen_ty_10 { ru_msgsnd: 0 },
        __bindgen_anon_11: rusage__bindgen_ty_11 { ru_msgrcv: 0 },
        __bindgen_anon_12: rusage__bindgen_ty_12 { ru_nsignals: 0 },
        __bindgen_anon_13: rusage__bindgen_ty_13 { ru_nvcsw: 0 },
        __bindgen_anon_14: rusage__bindgen_ty_14 { ru_nivcsw: 0 },
    };
    let mut selfa: crate::src_common::rusage = crate::src_common::rusage {
        ru_utime: crate::src_common::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        ru_stime: crate::src_common::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        __bindgen_anon_1: rusage__bindgen_ty_1 { ru_maxrss: 0 },
        __bindgen_anon_2: rusage__bindgen_ty_2 { ru_ixrss: 0 },
        __bindgen_anon_3: rusage__bindgen_ty_3 { ru_idrss: 0 },
        __bindgen_anon_4: rusage__bindgen_ty_4 { ru_isrss: 0 },
        __bindgen_anon_5: rusage__bindgen_ty_5 { ru_minflt: 0 },
        __bindgen_anon_6: rusage__bindgen_ty_6 { ru_majflt: 0 },
        __bindgen_anon_7: rusage__bindgen_ty_7 { ru_nswap: 0 },
        __bindgen_anon_8: rusage__bindgen_ty_8 { ru_inblock: 0 },
        __bindgen_anon_9: rusage__bindgen_ty_9 { ru_oublock: 0 },
        __bindgen_anon_10: rusage__bindgen_ty_10 { ru_msgsnd: 0 },
        __bindgen_anon_11: rusage__bindgen_ty_11 { ru_msgrcv: 0 },
        __bindgen_anon_12: rusage__bindgen_ty_12 { ru_nsignals: 0 },
        __bindgen_anon_13: rusage__bindgen_ty_13 { ru_nvcsw: 0 },
        __bindgen_anon_14: rusage__bindgen_ty_14 { ru_nivcsw: 0 },
    };
    let mut kidsb: crate::src_common::rusage = crate::src_common::rusage {
        ru_utime: crate::src_common::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        ru_stime: crate::src_common::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        __bindgen_anon_1: rusage__bindgen_ty_1 { ru_maxrss: 0 },
        __bindgen_anon_2: rusage__bindgen_ty_2 { ru_ixrss: 0 },
        __bindgen_anon_3: rusage__bindgen_ty_3 { ru_idrss: 0 },
        __bindgen_anon_4: rusage__bindgen_ty_4 { ru_isrss: 0 },
        __bindgen_anon_5: rusage__bindgen_ty_5 { ru_minflt: 0 },
        __bindgen_anon_6: rusage__bindgen_ty_6 { ru_majflt: 0 },
        __bindgen_anon_7: rusage__bindgen_ty_7 { ru_nswap: 0 },
        __bindgen_anon_8: rusage__bindgen_ty_8 { ru_inblock: 0 },
        __bindgen_anon_9: rusage__bindgen_ty_9 { ru_oublock: 0 },
        __bindgen_anon_10: rusage__bindgen_ty_10 { ru_msgsnd: 0 },
        __bindgen_anon_11: rusage__bindgen_ty_11 { ru_msgrcv: 0 },
        __bindgen_anon_12: rusage__bindgen_ty_12 { ru_nsignals: 0 },
        __bindgen_anon_13: rusage__bindgen_ty_13 { ru_nvcsw: 0 },
        __bindgen_anon_14: rusage__bindgen_ty_14 { ru_nivcsw: 0 },
    };
    let mut kidsa: crate::src_common::rusage = crate::src_common::rusage {
        ru_utime: crate::src_common::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        ru_stime: crate::src_common::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        __bindgen_anon_1: rusage__bindgen_ty_1 { ru_maxrss: 0 },
        __bindgen_anon_2: rusage__bindgen_ty_2 { ru_ixrss: 0 },
        __bindgen_anon_3: rusage__bindgen_ty_3 { ru_idrss: 0 },
        __bindgen_anon_4: rusage__bindgen_ty_4 { ru_isrss: 0 },
        __bindgen_anon_5: rusage__bindgen_ty_5 { ru_minflt: 0 },
        __bindgen_anon_6: rusage__bindgen_ty_6 { ru_majflt: 0 },
        __bindgen_anon_7: rusage__bindgen_ty_7 { ru_nswap: 0 },
        __bindgen_anon_8: rusage__bindgen_ty_8 { ru_inblock: 0 },
        __bindgen_anon_9: rusage__bindgen_ty_9 { ru_oublock: 0 },
        __bindgen_anon_10: rusage__bindgen_ty_10 { ru_msgsnd: 0 },
        __bindgen_anon_11: rusage__bindgen_ty_11 { ru_msgrcv: 0 },
        __bindgen_anon_12: rusage__bindgen_ty_12 { ru_nsignals: 0 },
        __bindgen_anon_13: rusage__bindgen_ty_13 { ru_nvcsw: 0 },
        __bindgen_anon_14: rusage__bindgen_ty_14 { ru_nivcsw: 0 },
    };

    c_gettimeofday(&mut before, &mut dtz);
    c_getrusage(RUSAGE_SELF, &mut selfb);
    c_getrusage(RUSAGE_CHILDREN, &mut kidsb);

    posix_time = unsafe {
        (!command.is_null() && (*command).flags & CMD_TIME_POSIX as libc::c_int != 0) as libc::c_int
    };
    nullcmd = unsafe {
        (command.is_null()
            || (*command).type_0 == command_type_cm_simple
                && ((*(*command).value.Simple).words).is_null()
                && ((*(*command).value.Simple).redirects).is_null()) as libc::c_int
    };
    if unsafe { posixly_correct != 0 && nullcmd != 0 } {
        kidsb.ru_stime.tv_sec = 0 as __time_t;
        selfb.ru_stime.tv_sec = kidsb.ru_stime.tv_sec;
        kidsb.ru_utime.tv_sec = selfb.ru_stime.tv_sec;
        selfb.ru_utime.tv_sec = kidsb.ru_utime.tv_sec;
        kidsb.ru_stime.tv_usec = 0 as __suseconds_t;
        selfb.ru_stime.tv_usec = kidsb.ru_stime.tv_usec;
        kidsb.ru_utime.tv_usec = selfb.ru_stime.tv_usec;
        selfb.ru_utime.tv_usec = kidsb.ru_utime.tv_usec;
        before = unsafe { shellstart };
    }
    unsafe {
        old_flags = (*command).flags;
        COPY_PROCENV!(top_level, save_top_level);
        (*command).flags &= !(CMD_TIME_PIPELINE as libc::c_int | CMD_TIME_POSIX as libc::c_int);
        code = setjmp_nosigs!(top_level.as_mut_ptr());
    }
    if code == NOT_JUMPED as libc::c_int {
        rv = execute_command_internal(command, asynchronous, pipe_in, pipe_out, fds_to_close);
        unsafe {
            (*command).flags = old_flags;
        }
    }
    unsafe {
        COPY_PROCENV!(save_top_level, top_level);
    }

    ss = 0 as time_t;
    us = ss;
    rs = us;
    cpu = 0;
    ssf = cpu;
    usf = ssf;
    rsf = usf;

    c_gettimeofday(&mut after, &mut dtz);

    c_getrusage(RUSAGE_SELF, &mut selfa);
    c_getrusage(RUSAGE_CHILDREN, &mut kidsa);

    c_difftimeval(&mut real, &mut before, &mut after);
    c_timeval_to_secs(&mut real, &mut rs, &mut rsf);

    c_addtimeval(
        &mut user,
        c_difftimeval(&mut after, &mut selfb.ru_utime, &mut selfa.ru_utime),
        c_difftimeval(&mut before, &mut kidsb.ru_utime, &mut kidsa.ru_utime),
    );
    c_timeval_to_secs(&mut user, &mut us, &mut usf);

    c_addtimeval(
        &mut sys,
        c_difftimeval(&mut after, &mut selfb.ru_stime, &mut selfa.ru_stime),
        c_difftimeval(&mut before, &mut kidsb.ru_stime, &mut kidsa.ru_stime),
    );
    c_timeval_to_secs(&mut sys, &mut ss, &mut ssf);

    cpu = c_timeval_to_cpu(&mut real, &mut user, &mut sys);

    if posix_time != 0 {
        time_format = POSIX_TIMEFORMAT!();
    } else {
        time_format = get_string_value(b"TIMEFORMAT\0" as *const u8 as *const libc::c_char);
        if time_format.is_null() {
            if unsafe { posixly_correct != 0 && nullcmd != 0 } {
                time_format = b"user\t%2lU\nsys\t%2lS\0" as *const u8 as *mut libc::c_char;
            } else {
                time_format = BASH_TIMEFORMAT!();
            }
        }
    }
    unsafe {
        if !time_format.is_null() && *time_format as libc::c_int != 0 {
            print_formatted_time(stderr, time_format, rs, rsf, us, usf, ss, ssf, cpu);
        }
        if code != 0 {
            c_siglongjmp(top_level.as_mut_ptr(), code);
        }
    }
    return rv;
}

fn execute_in_subshell(
    command: *mut COMMAND,
    mut asynchronous: libc::c_int,
    pipe_in: libc::c_int,
    pipe_out: libc::c_int,
    fds_to_close: *mut fd_bitmap,
) -> libc::c_int {
    let user_subshell: libc::c_int;
    let user_coproc: libc::c_int;
    let mut invert: libc::c_int;
    let mut return_code: libc::c_int;
    let mut function_value: libc::c_int;
    let should_redir_stdin: libc::c_int;
    let ois: libc::c_int;
    let result: libc::c_int;
    let tcom: *mut COMMAND;
    unsafe {
        subshell_level += 1;
        should_redir_stdin = (asynchronous != 0
            && (*command).flags & CMD_STDIN_REDIR as libc::c_int != 0
            && pipe_in == NO_PIPE
            && stdin_redirects((*command).redirects) == 0)
            as libc::c_int;

        invert = (((*command).flags & CMD_INVERT_RETURN as libc::c_int) != 0) as libc::c_int;
        user_subshell = ((*command).type_0 == command_type_cm_subshell
            || ((*command).flags & CMD_WANT_SUBSHELL as libc::c_int) != 0)
            as libc::c_int;
        user_coproc = ((*command).type_0 == command_type_cm_coproc) as libc::c_int;
        (*command).flags &= !(CMD_FORCE_SUBSHELL as libc::c_int
            | CMD_WANT_SUBSHELL as libc::c_int
            | CMD_INVERT_RETURN as libc::c_int);

        if asynchronous != 0 {
            original_pgrp = -1;

            ois = interactive_shell;
            interactive_shell = 0;

            if ois != interactive_shell {
                expand_aliases = 0;
            }
        }

        interactive = 0;
        login_shell = interactive;

        if shell_compatibility_level > 44 {
            loop_level = 0;
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
            running_trap = 0;
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
            stdin_redir =
                (stdin_redirects((*command).redirects) != 0 || pipe_in != NO_PIPE) as libc::c_int;
        } else if shell_control_structure((*command).type_0 as libc::c_uint) != 0
            && pipe_in != NO_PIPE
        {
            stdin_redir = 1;
        }

        if should_redir_stdin != 0 && stdin_redir == 0 {
            async_redirect_stdin();
        }

        default_buffered_input = -1;

        if !((*command).redirects).is_null() {
            if do_redirections((*command).redirects, RX_ACTIVE as libc::c_int) != 0 {
                exit(if invert != 0 {
                    EXECUTION_SUCCESS as libc::c_int
                } else {
                    EXECUTION_FAILURE as libc::c_int
                });
            }
            dispose_redirects((*command).redirects);
            (*command).redirects = 0 as *mut REDIRECT;
        }

        if (*command).type_0 == command_type_cm_subshell {
            tcom = (*(*command).value.Subshell).command as *mut COMMAND;
        } else if user_coproc != 0 {
            tcom = (*(*command).value.Coproc).command as *mut COMMAND;
        } else {
            tcom = command as *mut COMMAND;
        }

        if (*command).flags & CMD_TIME_PIPELINE as libc::c_int != 0 {
            (*tcom).flags = CMD_TIME_PIPELINE as libc::c_int;
        }
        if (*command).flags & CMD_TIME_POSIX as libc::c_int != 0 {
            (*tcom).flags = CMD_TIME_POSIX as libc::c_int;
        }

        if (*command).flags & CMD_IGNORE_RETURN as libc::c_int != 0
            && tcom != command as *mut COMMAND
        {
            (*tcom).flags = CMD_IGNORE_RETURN as libc::c_int;
        }

        if (user_subshell != 0 || user_coproc != 0)
            && ((*tcom).type_0 == command_type_cm_simple
                || (*tcom).type_0 == command_type_cm_subshell)
            && (*tcom).flags & CMD_TIME_PIPELINE as libc::c_int == 0
            && (*tcom).flags & CMD_INVERT_RETURN as libc::c_int == 0
        {
            (*tcom).flags = CMD_NO_FORK as libc::c_int;
            if (*tcom).type_0 == command_type_cm_simple {
                (*(*tcom).value.Simple).flags |= CMD_NO_FORK as libc::c_int;
            }
        }

        invert = ((*tcom).flags & CMD_INVERT_RETURN as libc::c_int != 0) as libc::c_int;
        (*tcom).flags &= !CMD_INVERT_RETURN as libc::c_int;

        result = setjmp_nosigs!(top_level.as_mut_ptr());

        function_value = 0;
        if return_catch_flag != 0 {
            function_value = setjmp_nosigs!(return_catch.as_mut_ptr());
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
        if user_subshell != 0 && signal_is_trapped(0) != 0 {
            last_command_exit_value = return_code;
            return_code = run_exit_trap();
        }
    }
    return return_code;
}

#[no_mangle]
pub fn getcoprocbypid(pid: pid_t) -> *mut coproc {
    unsafe {
        return if pid == sh_coproc.c_pid {
            &mut sh_coproc
        } else {
            0 as *mut Coproc
        };
    }
}

pub fn cpe_alloc(cp: *mut Coproc) -> *mut cpelement {
    let cpe: *mut cpelement;
    unsafe {
        cpe = malloc(size_of::<cpelement>() as usize) as *mut cpelement;
        (*cpe).coproc = cp;
        (*cpe).next = 0 as *mut cpelement;
    }
    return cpe;
}

pub fn cpe_dispose(cpe: *mut cpelement) {
    unsafe {
        free(cpe as *mut c_void);
    }
}

pub fn cpe_add(cp: *mut Coproc) -> *mut cpelement {
    let cpe: *mut cpelement;

    cpe = cpe_alloc(cp);
    unsafe {
        if coproc_list.head == 0 as *mut cpelement {
            coproc_list.tail = cpe;
            coproc_list.head = cpe;
            coproc_list.ncoproc = 0
        } else {
            (*coproc_list.tail).next = cpe;
            coproc_list.tail = cpe;
        }
        coproc_list.ncoproc += 1;
    }
    return cpe;
}

pub fn cpl_delete(pid: pid_t) -> *mut cpelement {
    let mut prev: *mut cpelement;
    let mut p: *mut cpelement;
    unsafe {
        p = coproc_list.head;
        prev = coproc_list.head;
        while !p.is_null() {
            if (*(*p).coproc).c_pid == pid {
                (*prev).next = (*p).next;
                break;
            }
            prev = p;
            p = (*p).next;
        }

        if p == 0 as *mut cpelement {
            return 0 as *mut cpelement;
        }

        if p == coproc_list.head {
            coproc_list.head = (*coproc_list.head).next;
        } else if p == coproc_list.tail {
            coproc_list.tail = prev;
        }

        coproc_list.ncoproc -= 1;
        if coproc_list.ncoproc == 0 {
            coproc_list.head = 0 as *mut cpelement;
            coproc_list.tail = 0 as *mut cpelement;
        } else if coproc_list.ncoproc == 1 {
            coproc_list.tail = coproc_list.head;
        }
    }
    return p;
}

pub fn cpl_reap() {
    let mut p: *mut cpelement;
    // let mut next: *mut cpelement;
    let mut nh: *mut cpelement;
    let mut nt: *mut cpelement;

    nh = 0 as *mut cpelement;
    nt = 0 as *mut cpelement;
    // let mut next = 0 as *mut cpelement;
    unsafe {
        p = coproc_list.head;
        while !p.is_null() {
            let next = (*p).next;

            if ((*(*p).coproc).c_flags & COPROC_DEAD as libc::c_int) != 0 {
                coproc_list.ncoproc -= 0;
                coproc_dispose((*p).coproc);
                cpe_dispose(p);
            } else if nh.is_null() {
                nh = p;
                nt = p;
            }
            p = next;
        }

        if coproc_list.ncoproc == 0 {
            coproc_list.head = 0 as *mut cpelement;
            coproc_list.tail = 0 as *mut cpelement;
        } else {
            if !nt.is_null() {
                (*nt).next = 0 as *mut cpelement;
            }

            coproc_list.head = nh;
            coproc_list.tail = nt;
            if coproc_list.ncoproc == 1 {
                coproc_list.tail = coproc_list.head; /* just to make sure */
            }
        }
    }
}

pub fn cpl_flush() {
    let mut cpe: *mut cpelement;
    let mut p: *mut cpelement;
    unsafe {
        cpe = coproc_list.head;
        while !cpe.is_null() {
            p = cpe;
            cpe = (*cpe).next;

            coproc_dispose((*p).coproc);
            cpe_dispose(p);
        }

        coproc_list.head = 0 as *mut cpelement;
        coproc_list.tail = 0 as *mut cpelement;
        coproc_list.ncoproc = 0;
    }
}

pub fn cpl_closeall() {
    let mut cpe: *mut cpelement;
    unsafe {
        cpe = coproc_list.head;
        while !cpe.is_null() {
            coproc_close((*cpe).coproc);

            cpe = (*cpe).next;
        }
    }
}

pub fn cpl_fdchk(fd: libc::c_int) {
    let mut cpe: *mut cpelement;
    unsafe {
        cpe = coproc_list.head;
        while !cpe.is_null() {
            coproc_checkfd((*cpe).coproc, fd);

            cpe = (*cpe).next;
        }
    }
}

pub fn cpl_search(pid: pid_t) -> *mut cpelement {
    let mut cpe: *mut cpelement;
    unsafe {
        cpe = coproc_list.head;
        while !cpe.is_null() {
            if (*(*cpe).coproc).c_pid == pid {
                return cpe;
            }
            cpe = (*cpe).next;
        }
    }
    return 0 as *mut cpelement;
}

pub fn cpl_searchbyname(name: *mut libc::c_char) -> *mut cpelement {
    let mut cp: *mut cpelement;
    unsafe {
        cp = coproc_list.head;
        while !cp.is_null() {
            if STREQ!((*(*cp).coproc).c_name, name) {
                return cp;
            }
            cp = (*cp).next;
        }
    }
    return 0 as *mut cpelement;
}

pub fn cpl_firstactive() -> pid_t {
    let mut cpe: *mut cpelement;
    unsafe {
        cpe = coproc_list.head;
        while !cpe.is_null() {
            if (*(*cpe).coproc).c_flags & COPROC_DEAD as libc::c_int == 0 {
                return (*(*cpe).coproc).c_pid;
            }
            cpe = (*cpe).next;
        }
    }
    return NO_PID!() as pid_t;
}

#[no_mangle]
pub fn getcoprocbyname(name: *const libc::c_char) -> *mut coproc {
    unsafe {
        return if !(sh_coproc.c_name).is_null() && STREQ!(sh_coproc.c_name, name) {
            &mut sh_coproc
        } else {
            0 as *mut Coproc
        };
    }
}

#[no_mangle]
pub fn coproc_init(cp: *mut coproc) {
    unsafe {
        (*cp).c_name = 0 as *mut libc::c_char;
        (*cp).c_pid = NO_PID!();
        (*cp).c_wfd = -1;
        (*cp).c_rfd = -1;
        (*cp).c_wsave = -1;
        (*cp).c_rsave = -1;
        (*cp).c_lock = 0;
        (*cp).c_status = 0;
        (*cp).c_flags = 0;
    }
}

#[no_mangle]
pub fn coproc_alloc(name: *mut libc::c_char, pid: pid_t) -> *mut coproc {
    let cp: *mut coproc;
    unsafe {
        cp = &mut sh_coproc;

        coproc_init(cp);
        (*cp).c_lock = 2;

        (*cp).c_pid = pid;
        (*cp).c_name = savestring!(name);

        (*cp).c_lock = 0;
    }
    return cp;
}

pub fn coproc_free(cp: *mut coproc) {
    unsafe {
        free(cp as *mut c_void);
    }
}

#[no_mangle]
pub fn coproc_dispose(cp: *mut coproc) {
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

    if cp.is_null() {
        return;
    }

    BLOCK_SIGNAL!(SIGCHLD, set, oset);
    unsafe {
        (*cp).c_lock = 3;
    }
    coproc_unsetvars(cp);
    unsafe {
        FREE!((*cp).c_name);
    }
    coproc_close(cp);

    coproc_init(cp);
    unsafe {
        (*cp).c_lock = 0;
    }
    UNBLOCK_SIGNAL!(oset);
}

#[no_mangle]
pub fn coproc_flush() {
    unsafe {
        coproc_dispose(&mut sh_coproc);
    }
}

#[no_mangle]
pub fn coproc_close(cp: *mut coproc) {
    unsafe {
        if (*cp).c_rfd >= 0 {
            close((*cp).c_rfd);
            (*cp).c_rfd = -1;
        }
        if (*cp).c_wfd >= 0 {
            close((*cp).c_wfd);
            (*cp).c_wfd = -1;
        }
        // let ref mut fresh27 = (*cp).c_wsave;
        (*cp).c_wsave = -1;
        (*cp).c_rsave = -1;
    }
}

#[no_mangle]
pub fn coproc_closeall() {
    unsafe {
        coproc_close(&mut sh_coproc);
    }
}

#[no_mangle]
pub fn coproc_reap() {
    let cp: *mut coproc;
    unsafe {
        cp = &mut sh_coproc;
        if !cp.is_null() && (*cp).c_flags & COPROC_DEAD as libc::c_int != 0 {
            coproc_dispose(cp);
        }
    }
}

#[no_mangle]
pub fn coproc_rclose(cp: *mut coproc, fd: libc::c_int) {
    unsafe {
        if (*cp).c_rfd >= 0 && (*cp).c_rfd == fd {
            close((*cp).c_rfd);
            (*cp).c_rfd = -1;
        }
    }
}

#[no_mangle]
pub fn coproc_wclose(cp: *mut coproc, fd: libc::c_int) {
    unsafe {
        if (*cp).c_wfd >= 0 && (*cp).c_wfd == fd {
            close((*cp).c_wfd);
            (*cp).c_wfd = -1;
        }
    }
}

#[no_mangle]
pub fn coproc_checkfd(cp: *mut coproc, fd: libc::c_int) {
    let mut update: libc::c_int;
    unsafe {
        update = 0;
        if (*cp).c_rfd >= 0 && (*cp).c_rfd == fd {
            // let ref mut fresh28 = (*cp).c_rfd;
            (*cp).c_rfd = -1;
            update = -1;
        }
        if (*cp).c_wfd >= 0 && (*cp).c_wfd == fd {
            // let ref mut fresh29 = (*cp).c_wfd;
            (*cp).c_wfd = -1;
            update = -1;
        }
        if update != 0 {
            coproc_setvars(cp);
        }
    }
}

#[no_mangle]
pub fn coproc_fdchk(fd: libc::c_int) {
    unsafe {
        coproc_checkfd(&mut sh_coproc, fd);
    }
}

#[no_mangle]
pub fn coproc_fdclose(cp: *mut coproc, fd: libc::c_int) {
    coproc_rclose(cp, fd);
    coproc_wclose(cp, fd);
    coproc_setvars(cp);
}

#[no_mangle]
pub fn coproc_fdsave(cp: *mut coproc) {
    unsafe {
        (*cp).c_rsave = (*cp).c_rfd;
        (*cp).c_wsave = (*cp).c_wfd;
    }
}

#[no_mangle]
pub fn coproc_fdrestore(cp: *mut coproc) {
    unsafe {
        (*cp).c_rfd = (*cp).c_rsave;
        (*cp).c_wfd = (*cp).c_wsave;
    }
}

fn coproc_setstatus(cp: *mut coproc, status: libc::c_int) {
    unsafe {
        (*cp).c_lock = 4;
        (*cp).c_status = status;
        (*cp).c_flags |= COPROC_DEAD as libc::c_int;
        (*cp).c_flags &= !(COPROC_RUNNING as libc::c_int);
        (*cp).c_lock = 0 as libc::c_int;
    }
}

#[no_mangle]
pub fn coproc_pidchk(pid: pid_t, status: libc::c_int) {
    let cp: *mut coproc;

    cp = getcoprocbypid(pid);
    if !cp.is_null() {
        coproc_setstatus(cp, status);
    }
}

#[no_mangle]
pub fn coproc_active() -> pid_t {
    unsafe {
        return if sh_coproc.c_flags & COPROC_DEAD as libc::c_int != 0 {
            NO_PID!()
        } else {
            sh_coproc.c_pid
        };
    }
}

#[no_mangle]
pub fn coproc_setvars(cp: *mut coproc) {
    let mut v: *mut SHELL_VAR;
    let namevar: *mut libc::c_char;
    let mut t: *mut libc::c_char;
    let l: libc::c_int;
    let mut w: WordDesc = WordDesc {
        word: 0 as *mut libc::c_char,
        flags: 0,
    };
    let mut ind: arrayind_t;
    unsafe {
        if ((*cp).c_name).is_null() {
            return;
        }

        w.word = (*cp).c_name;
        w.flags = 0;
        if check_identifier(&mut w, 1) == 0 {
            return;
        }

        l = strlen((*cp).c_name) as libc::c_int;
        namevar = malloc((l + 16) as usize) as *mut libc::c_char;

        v = find_variable((*cp).c_name);

        if v.is_null() {
            v = find_variable_nameref_for_create((*cp).c_name, 1);
            if v == INVALID_NAMEREF_VALUE!() {
                free(namevar as *mut c_void);
                return;
            }
            if !v.is_null() && nameref_p!(v) != 0 {
                free((*cp).c_name as *mut c_void);
                // let ref mut fresh30 = (*cp).c_name;
                (*cp).c_name = savestring!(nameref_cell!(v));
                v = make_new_array_variable((*cp).c_name);
            }
        }

        if !v.is_null() && (readonly_p!(v) != 0 || noassign_p!(v) != 0) {
            if readonly_p!(v) != 0 {
                err_readonly((*cp).c_name);
            }
            free(namevar as *mut c_void);
            return;
        }
        if v.is_null() {
            v = make_new_array_variable((*cp).c_name);
        }
        if array_p!(v) == 0 {
            convert_var_to_array(v);
        }

        t = c_itos((*cp).c_rfd as intmax_t);
        ind = 0 as arrayind_t;
        bind_array_variable((*cp).c_name, ind, t, 0);
        free(t as *mut c_void);

        t = c_itos((*cp).c_wfd as intmax_t);
        ind = 1 as arrayind_t;
        bind_array_variable((*cp).c_name, ind, t, 0 as libc::c_int);
        free(t as *mut c_void);

        sprintf(
            namevar,
            b"%s_PID\0" as *const u8 as *const libc::c_char,
            (*cp).c_name,
        );
        t = c_itos((*cp).c_pid as intmax_t);
        bind_variable(namevar, t, 0 as libc::c_int);
        free(t as *mut c_void);

        free(namevar as *mut c_void);
    }
}

#[no_mangle]
pub fn coproc_unsetvars(cp: *mut coproc) {
    let l: libc::c_int;
    let namevar: *mut libc::c_char;
    unsafe {
        if ((*cp).c_name).is_null() {
            return;
        }
        l = strlen((*cp).c_name) as libc::c_int;

        namevar = malloc((l + 16) as usize) as *mut libc::c_char;

        sprintf(
            namevar,
            b"%s_PID\0" as *const u8 as *const libc::c_char,
            (*cp).c_name,
        );
        unbind_variable_noref(namevar);

        check_unbind_variable((*cp).c_name);

        free(namevar as *mut c_void);
    }
}

fn execute_coproc(
    command: *mut COMMAND,
    pipe_in: libc::c_int,
    pipe_out: libc::c_int,
    fds_to_close: *mut fd_bitmap,
) -> libc::c_int {
    let mut rpipe: [libc::c_int; 2] = [0; 2];
    let mut wpipe: [libc::c_int; 2] = [0; 2];
    let estat: libc::c_int;
    let invert: libc::c_int;
    let coproc_pid: pid_t;
    let cp: *mut Coproc;
    let tcmd: *mut libc::c_char;
    let p: *mut libc::c_char;
    let name: *mut libc::c_char;
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };
    unsafe {
        if sh_coproc.c_pid != NO_PID!() && (sh_coproc.c_rfd >= 0 || sh_coproc.c_wfd >= 0) {
            internal_warning(
                b"execute_coproc: coproc [%d:%s] still exists\0" as *const u8 as *mut libc::c_char,
                sh_coproc.c_pid,
                sh_coproc.c_name,
            );
        }
        coproc_init(&mut sh_coproc);

        invert = ((*command).flags & CMD_INVERT_RETURN as libc::c_int != 0) as libc::c_int;

        name = expand_string_unsplit_to_string((*(*command).value.Coproc).name, 0);

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

        command_string_index = 0;
        tcmd = make_command_string(command);
    }
    sh_openpipe(&mut rpipe as *mut [libc::c_int; 2] as *mut libc::c_int);
    sh_openpipe(&mut wpipe as *mut [libc::c_int; 2] as *mut libc::c_int);

    BLOCK_SIGNAL!(SIGCHLD, set, oset);

    unsafe {
        p = savestring!(tcmd);
    }
    coproc_pid = make_child(p, FORK_ASYNC as libc::c_int);
    unsafe {
        if coproc_pid == 0 {
            close(rpipe[0 as libc::c_int as usize]);
            close(wpipe[1 as libc::c_int as usize]);

            FREE!(p);

            UNBLOCK_SIGNAL!(oset);
            estat = execute_in_subshell(command, 1, wpipe[0], rpipe[1], fds_to_close);
            fflush(stdout);
            fflush(stderr);

            exit(estat);
        }

        close(rpipe[1]);
        close(wpipe[0]);

        cp = coproc_alloc((*(*command).value.Coproc).name, coproc_pid);
        (*cp).c_rfd = rpipe[0];
        (*cp).c_wfd = wpipe[1];

        (*cp).c_flags |= COPROC_RUNNING as libc::c_int;

        fcntl((*cp).c_rfd, 2 as libc::c_int, 1 as libc::c_int);
        fcntl((*cp).c_wfd, 2 as libc::c_int, 1 as libc::c_int);
    }
    coproc_setvars(cp);

    UNBLOCK_SIGNAL!(oset);

    close_pipes(pipe_in, pipe_out);

    unlink_fifo_list();

    stop_pipeline(1, 0 as *mut libc::c_void as *mut COMMAND);
    unsafe {
        DESCRIBE_PID!(coproc_pid);
    }
    run_pending_traps();

    return if invert != 0 { 1 } else { 0 };
}

fn restore_stdin(s: libc::c_int) {
    unsafe {
        dup2(s, 0);
        close(s);
    }
}
fn lastpipe_cleanup(s: libc::c_int) {
    set_jobs_list_frozen(s);
}

fn execute_pipeline(
    command: *mut COMMAND,
    asynchronous: libc::c_int,
    pipe_in: libc::c_int,
    pipe_out: libc::c_int,
    fds_to_close: *mut fd_bitmap,
) -> libc::c_int {
    let mut prev: libc::c_int;
    let mut fildes: [libc::c_int; 2] = [0; 2];
    let mut new_bitmap_size: libc::c_int;
    let mut dummyfd: libc::c_int;
    let ignore_return: libc::c_int;
    let mut exec_result: libc::c_int;
    let mut lstdin: libc::c_int;
    let mut lastpipe_flag: libc::c_int;
    let mut lastpipe_jid: libc::c_int = 0;
    let mut old_frozen: libc::c_int = 0;
    let mut cmd: *mut COMMAND;
    let mut fd_bitmap: *mut fd_bitmap;
    let lastpid: pid_t;
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };
    unsafe {
        BLOCK_CHILD(&mut set, &mut oset);
        ignore_return = ((*command).flags & CMD_IGNORE_RETURN as libc::c_int != 0) as libc::c_int;

        prev = pipe_in;
        cmd = command;

        while !cmd.is_null()
            && (*cmd).type_0 == command_type_cm_connection
            && !((*cmd).value.Connection).is_null()
            && (*(*cmd).value.Connection).connector == '|' as i32
        {
            if pipe(fildes.as_mut_ptr()) < 0 {
                sys_error(b"pipe error\0" as *const u8 as *const libc::c_char);

                terminate_current_pipeline();
                kill_current_pipeline();

                UNBLOCK_CHILD(&mut oset);

                last_command_exit_value = EXECUTION_FAILURE as libc::c_int;

                throw_to_top_level();
                return 1;
            }

            new_bitmap_size = if fildes[0] < (*fds_to_close).size {
                (*fds_to_close).size
            } else {
                fildes[0] + 8
            };

            fd_bitmap = new_fd_bitmap(new_bitmap_size);

            xbcopy(
                (*fds_to_close).bitmap,
                (*fd_bitmap).bitmap,
                (*fds_to_close).size,
            );

            *((*fd_bitmap).bitmap).offset(fildes[0] as isize) = 1;

            begin_unwind_frame(b"pipe-file-descriptors\0" as *const u8 as *mut libc::c_char);

            add_unwind_protect(
                transmute::<fn(fdbp: *mut fd_bitmap) -> (), Option<Function>>(dispose_fd_bitmap),
                fd_bitmap as *mut libc::c_char,
            );

            add_unwind_protect(
                transmute::<fn(fdbp: *mut fd_bitmap) -> (), Option<Function>>(close_fd_bitmap),
                fd_bitmap as *mut libc::c_char,
            );
            if prev >= 0 {
                add_unwind_protect(
                    transmute::<
                        unsafe extern "C" fn(__fd: libc::c_int) -> libc::c_int,
                        Option<Function>,
                    >(close),
                    prev as *mut libc::c_char,
                );
            }
            dummyfd = fildes[1];
            add_unwind_protect(
                transmute::<unsafe extern "C" fn(__fd: libc::c_int) -> libc::c_int, Option<Function>>(
                    close,
                ),
                dummyfd as *mut libc::c_char,
            );

            add_unwind_protect(
                transmute::<fn(*mut sigset_t) -> libc::c_int, Option<Function>>(
                    restore_signal_mask,
                ),
                transmute::<*mut sigset_t, *mut libc::c_char>(&mut oset),
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
            discard_unwind_frame(b"pipe-file-descriptors\0" as *const u8 as *mut libc::c_char);

            cmd = (*(*cmd).value.Connection).second;
        }

        lastpid = last_made_pid;
        if ignore_return != 0 && !cmd.is_null() {
            (*cmd).flags |= CMD_IGNORE_RETURN as libc::c_int;
        }
        lastpipe_flag = 0;

        begin_unwind_frame(b"lastpipe-exec\0" as *const u8 as *mut libc::c_char);
        lstdin = -1;

        if lastpipe_opt != 0
            && job_control == 0
            && asynchronous == 0
            && pipe_out == NO_PIPE
            && prev > 0
        {
            lstdin = move_to_high_fd(0, 1, -1);
            if lstdin > 0 {
                do_piping(prev, pipe_out);
                prev = NO_PIPE;
                add_unwind_protect(
                    transmute::<fn(libc::c_int) -> (), Option<Function>>(restore_stdin),
                    lstdin as *mut libc::c_char,
                );
                lastpipe_flag = 1;
                old_frozen = freeze_jobs_list();
                lastpipe_jid =
                    stop_pipeline(0 as libc::c_int, 0 as *mut libc::c_void as *mut COMMAND);
                add_unwind_protect(
                    transmute::<fn(libc::c_int) -> (), Option<Function>>(lastpipe_cleanup),
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
                transmute::<unsafe extern "C" fn(libc::c_int) -> libc::c_int, Option<Function>>(
                    close,
                ),
                prev as *mut libc::c_char,
            );
        }

        exec_result = execute_command_internal(cmd, asynchronous, prev, pipe_out, fds_to_close);

        if lstdin > 0 {
            restore_stdin(lstdin);
        }

        if prev >= 0 {
            close(prev);
        }

        UNBLOCK_CHILD(&mut oset);

        QUIT!();

        if lastpipe_flag != 0 {
            if (lastpipe_jid < 0 as libc::c_int
                || lastpipe_jid >= js.j_jobslots
                || (*jobs.offset(lastpipe_jid as isize)).is_null()) as libc::c_int
                == 0 as libc::c_int
            {
                append_process(
                    savestring!(the_printed_command_except_trap),
                    dollar_dollar_pid,
                    exec_result,
                    lastpipe_jid,
                );
                lstdin = wait_for(lastpid, 0);
            } else {
                lstdin = wait_for_single_pid(lastpid, 0);
            }
            if (lastpipe_jid < 0
                || lastpipe_jid >= js.j_jobslots
                || (*jobs.offset(lastpipe_jid as isize)).is_null()) as libc::c_int
                == 0 as libc::c_int
            {
                exec_result = job_exit_status(lastpipe_jid);
            } else if pipefail_opt != 0 {
                exec_result = exec_result | lstdin;
            }
            set_jobs_list_frozen(old_frozen);
        }
        discard_unwind_frame(b"lastpipe-exec\0" as *const u8 as *mut libc::c_char);
    }
    return exec_result;
}

const FLAG_AND: i32 = '&' as i32;
const FLAG_SEMICOLON: i32 = ';' as i32;
const FLAG_OR: i32 = '|' as i32;
const FLAG_OR_OR: i32 = OR_OR as i32;
const FLAG_AND_AND: i32 = AND_AND as i32;

fn execute_connection(
    command: *mut COMMAND,
    asynchronous: libc::c_int,
    pipe_in: libc::c_int,
    pipe_out: libc::c_int,
    fds_to_close: *mut fd_bitmap,
) -> libc::c_int {
    let tc: *mut COMMAND;
    let second: *mut COMMAND;
    let mut ignore_return: libc::c_int;
    let mut exec_result: libc::c_int = 0;
    let was_error_trap: libc::c_int;
    let mut invert: libc::c_int;
    let save_line_number: libc::c_int;
    unsafe {
        ignore_return = ((*command).flags & CMD_IGNORE_RETURN as libc::c_int != 0) as libc::c_int;

        match (*(*command).value.Connection).connector {
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
                exec_result = execute_command_internal(tc, 1, pipe_in, pipe_out, fds_to_close);
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
                        (*(*(*command).value.Connection).first).flags |=
                            CMD_IGNORE_RETURN as libc::c_int;
                    }
                    if !((*(*command).value.Connection).second).is_null() {
                        (*(*(*command).value.Connection).second).flags |=
                            CMD_IGNORE_RETURN as libc::c_int;
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
                was_error_trap = (signal_is_trapped(ERROR_TRAP as libc::c_int) != 0
                    && signal_is_ignored(ERROR_TRAP as libc::c_int) == 0)
                    as libc::c_int;
                invert = ((*command).flags & CMD_INVERT_RETURN as libc::c_int != 0) as libc::c_int;
                ignore_return =
                    ((*command).flags & CMD_IGNORE_RETURN as libc::c_int != 0) as libc::c_int;

                line_number_for_err_trap = line_number;
                exec_result =
                    execute_pipeline(command, asynchronous, pipe_in, pipe_out, fds_to_close);

                if asynchronous != 0 {
                    exec_result = EXECUTION_SUCCESS as libc::c_int;
                    invert = 0 as libc::c_int;
                }

                if was_error_trap != 0
                    && ignore_return == 0
                    && invert == 0
                    && exec_result != EXECUTION_SUCCESS as libc::c_int
                {
                    last_command_exit_value = exec_result;
                    save_line_number = line_number;
                    line_number = line_number_for_err_trap;
                    run_error_trap();
                    line_number = save_line_number;
                }

                if ignore_return == 0
                    && invert == 0
                    && exit_immediately_on_error != 0
                    && exec_result != EXECUTION_SUCCESS as libc::c_int
                {
                    last_command_exit_value = exec_result;
                    run_pending_traps();
                    jump_to_top_level(ERREXIT as libc::c_int);
                }
            }
            FLAG_AND_AND | FLAG_OR_OR => {
                if asynchronous != 0 {
                    (*command).flags |= CMD_FORCE_SUBSHELL as libc::c_int;
                    exec_result =
                        execute_command_internal(command, 1, pipe_in, pipe_out, fds_to_close);
                } else {
                    executing_list += 1;
                    if !((*(*command).value.Connection).first).is_null() {
                        (*(*(*command).value.Connection).first).flags |=
                            CMD_IGNORE_RETURN as libc::c_int;
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
                    0,
                );
                jump_to_top_level(EXECUTION_FAILURE as libc::c_int);
            }
        }
    }
    return exec_result;
}

fn execute_for_command(for_command: *mut FOR_COM) -> libc::c_int {
    let releaser: *mut WordList;
    let mut list: *mut WordList;
    let mut v: *mut SHELL_VAR;
    let identifier: *mut libc::c_char;
    let mut retval: libc::c_int;
    let save_line_number: libc::c_int;
    unsafe {
        save_line_number = line_number;
        if check_identifier((*for_command).name, 1) == 0 {
            if posixly_correct != 0 && interactive_shell == 0 && rpm_requires == 0 {
                last_command_exit_value = EX_BADUSAGE as libc::c_int;
                jump_to_top_level(ERREXIT as libc::c_int);
            }
            return EXECUTION_FAILURE as libc::c_int;
        }

        loop_level += 1;
        identifier = (*(*for_command).name).word;

        line_number = (*for_command).line;
        releaser = expand_words_no_vars((*for_command).map_list);
        list = releaser;

        begin_unwind_frame(b"for\0" as *const u8 as *mut libc::c_char);
        add_unwind_protect(
            transmute::<fn(arg1: *mut WordList), Option<Function>>(dispose_words),
            releaser as *mut libc::c_char,
        );

        if (*for_command).flags & CMD_IGNORE_RETURN as libc::c_int != 0 {
            (*(*for_command).action).flags |= CMD_IGNORE_RETURN as libc::c_int;
        }

        retval = EXECUTION_SUCCESS as libc::c_int;
        while !list.is_null() {
            QUIT!();

            line_number = (*for_command).line;

            command_string_index = 0;
            print_for_command_head(for_command);

            if echo_command_at_execute != 0 {
                xtrace_print_for_command_head(for_command);
            }

            if signal_in_progress(DEBUG_TRAP as libc::c_int) == 0 && running_trap == 0 {
                FREE!(the_printed_command_except_trap);
                the_printed_command_except_trap = savestring!(the_printed_command);
            }

            retval = run_debug_trap();

            if !(debugging_mode != 0 && retval != EXECUTION_SUCCESS as libc::c_int) {
                this_command_name = 0 as *mut libc::c_char;

                v = find_variable_last_nameref(identifier, 1);
                if !v.is_null() && nameref_p!(v) != 0 {
                    if valid_nameref_value((*(*list).word).word, 1) == 0 {
                        sh_invalidid((*(*list).word).word);
                        v = 0 as *mut SHELL_VAR;
                    } else if readonly_p!(v) != 0 {
                        err_readonly(name_cell!(v));
                    } else {
                        v = bind_variable_value(
                            v,
                            (*(*list).word).word,
                            ASS_NAMEREF as libc::c_int,
                        );
                    }
                } else {
                    v = bind_variable(identifier, (*(*list).word).word, 0);
                }
                if v.is_null() || readonly_p!(v) != 0 || noassign_p!(v) != 0 {
                    line_number = save_line_number;
                    if !v.is_null()
                        && readonly_p!(v) != 0
                        && interactive_shell == 0
                        && posixly_correct != 0
                    {
                        last_command_exit_value = EXECUTION_FAILURE as libc::c_int;
                        jump_to_top_level(FORCE_EOF as libc::c_int);
                    } else {
                        dispose_words(releaser);
                        discard_unwind_frame(b"for\0" as *const u8 as *mut libc::c_char);
                        loop_level -= 1;
                        return EXECUTION_FAILURE as libc::c_int;
                    }
                }

                if ifsname!(identifier) {
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
    }
    return retval;
}

fn eval_arith_for_expr(l: *mut WordList, okp: *mut libc::c_int) -> intmax_t {
    let new: *mut WordList;
    let expresult: intmax_t;
    let r: libc::c_int;

    new = expand_words_no_vars(l);

    if !new.is_null() {
        if unsafe { echo_command_at_execute != 0 } {
            xtrace_print_arith_cmd(new);
        }
        unsafe {
            this_command_name = b"((\0" as *const u8 as *mut libc::c_char;

            command_string_index = 0;
        }
        print_arith_command(new);
        unsafe {
            if signal_in_progress(DEBUG_TRAP as libc::c_int) == 0 && running_trap == 0 {
                FREE!(the_printed_command_except_trap);
                the_printed_command_except_trap = savestring!(the_printed_command);
            }
        }
        r = run_debug_trap();
        if unsafe { debugging_mode == 0 || r == EXECUTION_SUCCESS as libc::c_int } {
            unsafe {
                expresult = evalexp((*(*new).word).word, EXP_EXPANDED as libc::c_int, okp);
            }
        } else {
            expresult = 0 as intmax_t;
            if !okp.is_null() {
                unsafe {
                    *okp = 1;
                }
            }
        }

        dispose_words(new);
    } else {
        expresult = 0 as intmax_t;
        if !okp.is_null() {
            unsafe {
                *okp = 1;
            }
        }
    }

    return expresult;
}

fn execute_arith_for_command(arith_for_command: *mut ARITH_FOR_COM) -> libc::c_int {
    let mut expresult: intmax_t;
    let mut expok: libc::c_int = 0;
    let mut body_status: libc::c_int;
    let arith_lineno: libc::c_int;
    let save_lineno: libc::c_int;

    body_status = EXECUTION_SUCCESS as libc::c_int;
    unsafe {
        loop_level += 1;
        save_lineno = line_number;

        if (*arith_for_command).flags & CMD_IGNORE_RETURN as libc::c_int != 0 {
            (*(*arith_for_command).action).flags |= CMD_IGNORE_RETURN as libc::c_int;
        }

        this_command_name = b"((\0" as *const u8 as *mut libc::c_char;

        arith_lineno = (*arith_for_command).line;
        line_number = arith_lineno;
        if variable_context != 0 && interactive_shell != 0 && sourcelevel == 0 {
            line_number -= function_line_number - 1;
            if line_number <= 0 {
                line_number = 1;
            }
        }
        eval_arith_for_expr((*arith_for_command).init, &mut expok);
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
                    eval_arith_for_expr((*arith_for_command).step, &mut expok);
                    line_number = save_lineno;

                    if !(expok == 0) {
                        continue;
                    }
                    body_status = 1;
                    break;
                }
            }
        }
        loop_level -= 1;
        line_number = save_lineno;
    }
    return body_status;
}

static mut COLS: libc::c_int = 0;
static mut tabsize: libc::c_int = 0;

fn displen(s: *const libc::c_char) -> libc::c_int {
    let mut wcstr: *mut wchar_t;
    let mut slen: size_t;
    let wclen: libc::c_int;

    wcstr = 0 as *mut wchar_t;
    slen = c_mbstowcs(wcstr, s, 0 as usize) as size_t;
    if slen == -(1 as libc::c_int) as libc::c_ulong {
        slen = 0 as size_t;
    }

    wcstr =
        unsafe { malloc((size_of::<wchar_t>() * (slen + 1) as usize) as usize) as *mut wchar_t };
    c_mbstowcs(wcstr, s, (slen + 1) as usize);
    wclen = c_wcswidth(wcstr, slen as usize);
    unsafe {
        free(wcstr as *mut c_void);
    }

    return unsafe { (if wclen < 0 { STRLEN!(s) } else { wclen }) as libc::c_int };
}

fn print_index_and_element(len: libc::c_int, ind: libc::c_int, list: *mut WordList) -> libc::c_int {
    let mut l: *mut WordList;
    let mut i: libc::c_int;

    if list.is_null() {
        return 0;
    }

    i = ind;
    l = list;

    while !l.is_null() && {
        i -= 1;
        i != 0
    } {
        unsafe {
            l = (*l).next;
        }
    }
    if l.is_null() {
        return 0;
    }
    unsafe {
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
}

fn indent(mut from: libc::c_int, to: libc::c_int) {
    unsafe {
        while from < to {
            if to / tabsize > from / tabsize {
                c_putc('\t' as i32, stderr);
                from += tabsize - from % tabsize;
            } else {
                c_putc(' ' as i32, stderr);
                from += 1;
            }
        }
    }
}

fn print_select_list(
    list: *mut WordList,
    list_len: libc::c_int,
    max_elem_len: libc::c_int,
    mut indices_len: libc::c_int,
) {
    let mut ind: libc::c_int;
    let mut row: libc::c_int;
    let mut elem_len: libc::c_int;
    let mut pos: libc::c_int;
    let mut cols: libc::c_int;
    let mut rows: libc::c_int;
    let first_column_indices_len: libc::c_int;
    let other_indices_len: libc::c_int;

    if list.is_null() {
        unsafe {
            c_putc('\n' as i32, stderr);
        }
        return;
    }

    cols = if max_elem_len != 0 {
        unsafe { COLS / max_elem_len }
    } else {
        1
    };
    if cols == 0 {
        cols = 1;
    }

    rows = if list_len != 0 {
        list_len / cols + (list_len % cols != 0) as libc::c_int
    } else {
        1
    };
    cols = if list_len != 0 {
        list_len / rows + (list_len % rows != 0) as libc::c_int
    } else {
        1
    };
    if rows == 1 {
        rows = cols;
        // cols = 1;
    }

    first_column_indices_len = NUMBER_LEN!(rows);
    other_indices_len = indices_len;

    row = 0;
    while row < rows {
        ind = row;
        pos = 0;
        loop {
            indices_len = if pos == 0 {
                first_column_indices_len
            } else {
                other_indices_len
            };
            elem_len = print_index_and_element(indices_len, ind + 1, list);
            elem_len += indices_len + RP_SPACE_LEN!();
            ind += rows;
            if ind >= list_len {
                break;
            }
            indent(pos + elem_len, pos + max_elem_len);
            pos += max_elem_len;
        }
        unsafe {
            c_putc('\n' as i32, stderr);
        }

        row += 1;
    }
}

fn select_query(
    list: *mut WordList,
    list_len: libc::c_int,
    prompt: *mut libc::c_char,
    mut print_menu: libc::c_int,
) -> *mut libc::c_char {
    let mut max_elem_len: libc::c_int;
    let indices_len: libc::c_int;
    let mut len: libc::c_int;
    let mut r: libc::c_int;
    let mut oe: libc::c_int;
    let mut reply: intmax_t = 0;
    let mut l: *mut WordList;
    let mut repl_string: *mut libc::c_char;
    // let t: *mut libc::c_char = 0 as *mut libc::c_char;

    unsafe {
        COLS = default_columns();

        tabsize = 8;
    }
    max_elem_len = 0;
    l = list;
    while !l.is_null() {
        len = unsafe { displen((*(*l).word).word) };
        if len > max_elem_len {
            max_elem_len = len;
        }
        unsafe {
            l = (*l).next;
        }
    }
    indices_len = NUMBER_LEN!(list_len);
    max_elem_len += indices_len + RP_SPACE_LEN!() + 2;

    loop {
        if print_menu != 0 {
            print_select_list(list, list_len, max_elem_len, indices_len);
        }
        unsafe {
            fprintf(stderr, b"%s\0" as *const u8 as *const libc::c_char, prompt);
            fflush(stderr);
            QUIT!();

            oe = executing_builtin;
            executing_builtin = 1;
        }
        r = read_builtin(0 as *mut WordList);
        unsafe {
            executing_builtin = oe;
        }
        if r != EXECUTION_SUCCESS as libc::c_int {
            unsafe {
                putchar('\n' as i32);
            }
            return 0 as *mut libc::c_char;
        }
        repl_string = get_string_value(b"REPLY\0" as *const u8 as *const libc::c_char);
        if repl_string.is_null() {
            return 0 as *mut libc::c_char;
        }
        if unsafe { *repl_string as libc::c_int == 0 } {
            print_menu = 1;
        } else {
            if legal_number(repl_string, &mut reply) == 0 {
                return b"\0" as *const u8 as *mut libc::c_char;
            }
            if reply < 1 || reply > list_len as libc::c_long {
                return b"\0" as *const u8 as *mut libc::c_char;
            }
            l = list;
            while !l.is_null() && {
                reply -= 1;
                reply != 0
            } {
                unsafe {
                    l = (*l).next;
                }
            }
            return unsafe { (*(*l).word).word };
        }
    }
}

fn execute_select_command(select_command: *mut SELECT_COM) -> libc::c_int {
    let releaser: *mut WordList;
    let list: *mut WordList;
    let mut v: *mut SHELL_VAR;
    let identifier: *mut libc::c_char;
    let mut ps3_prompt: *mut libc::c_char;
    let mut selection: *mut libc::c_char;
    let mut retval: libc::c_int;
    let list_len: libc::c_int;
    let mut show_menu: libc::c_int;
    let save_line_number: libc::c_int;
    unsafe {
        if check_identifier((*select_command).name, 1) == 0 {
            return EXECUTION_FAILURE as libc::c_int;
        }
        save_line_number = line_number;
        line_number = (*select_command).line;

        command_string_index = 0;
        print_select_command_head(select_command);

        if echo_command_at_execute != 0 {
            xtrace_print_select_command_head(select_command);
        }

        if signal_in_progress(DEBUG_TRAP as libc::c_int) == 0 && running_trap == 0 {
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

        begin_unwind_frame(b"select\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        add_unwind_protect(
            transmute::<fn(arg1: *mut WordList), Option<Function>>(dispose_words),
            releaser as *mut libc::c_char,
        );

        if (*select_command).flags & CMD_IGNORE_RETURN as libc::c_int != 0 {
            (*(*select_command).action).flags |= CMD_IGNORE_RETURN as libc::c_int;
        }

        // retval = EXECUTION_SUCCESS as libc::c_int;
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
                v = bind_variable(identifier, selection, 0);
                if v.is_null() || readonly_p!(v) != 0 || noassign_p!(v) != 0 {
                    if !v.is_null()
                        && readonly_p!(v) != 0
                        && interactive_shell == 0
                        && posixly_correct != 0
                    {
                        last_command_exit_value = EXECUTION_FAILURE as libc::c_int;
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

                    show_menu = 0;
                    selection = get_string_value(b"REPLY\0" as *const u8 as *const libc::c_char);
                    if !selection.is_null() && *selection as libc::c_int == '\u{0}' as i32 {
                        show_menu = 1;
                    }
                }
            }
        }

        loop_level -= 1;
        line_number = save_line_number;

        dispose_words(releaser);
        discard_unwind_frame(b"select\0" as *const u8 as *mut libc::c_char);
    }
    return retval;
}

fn execute_case_command(case_command: *mut CASE_COM) -> libc::c_int {
    let mut list: *mut WordList;
    let wlist: *mut WordList;
    let mut es: *mut WordList;
    let mut clauses: *mut PATTERN_LIST;
    let word: *mut libc::c_char;
    let mut pattern: *mut libc::c_char;
    let mut retval: libc::c_int;
    let mut match_0: libc::c_int;
    let ignore_return: libc::c_int;
    let save_line_number: libc::c_int;
    let mut qflags: libc::c_int;
    unsafe {
        save_line_number = line_number;
        line_number = (*case_command).line;

        command_string_index = 0 as libc::c_int;
        print_case_command_head(case_command);

        if echo_command_at_execute != 0 {
            xtrace_print_case_command_head(case_command);
        }

        if signal_in_progress(DEBUG_TRAP as libc::c_int) == 0 && running_trap == 0 {
            FREE!(the_printed_command_except_trap);
            the_printed_command_except_trap = savestring!(the_printed_command);
        }

        retval = run_debug_trap();
        if debugging_mode != 0 && retval != EXECUTION_SUCCESS as libc::c_int {
            line_number = save_line_number;
            return EXECUTION_SUCCESS as libc::c_int;
        }
        wlist = expand_word_leave_quoted((*case_command).word, 0);

        if !wlist.is_null() {
            let t: *mut libc::c_char;
            t = string_list(wlist);
            word = dequote_string(t);
            free(t as *mut c_void);
        } else {
            word = savestring!(b"\0" as *const u8 as *mut libc::c_char);
        }
        dispose_words(wlist);

        retval = EXECUTION_SUCCESS as libc::c_int;
        ignore_return = (*case_command).flags & CMD_IGNORE_RETURN as libc::c_int;

        begin_unwind_frame(b"case\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        add_unwind_protect(
            transmute::<unsafe extern "C" fn(__ptr: *mut ::std::os::raw::c_void), Option<Function>>(
                free,
            ),
            word,
        );

        clauses = (*case_command).clauses;
        's_150: while !clauses.is_null() {
            QUIT!();
            list = (*clauses).patterns;
            while !list.is_null() {
                es = expand_word_leave_quoted((*list).word, 0);

                if !es.is_null()
                    && !((*es).word).is_null()
                    && !((*(*es).word).word).is_null()
                    && *(*(*es).word).word as libc::c_int != 0
                {
                    qflags = QGLOB_CVTNULL as libc::c_int;
                    qflags |= QGLOB_CTLESC as libc::c_int;
                    pattern = quote_string_for_globbing((*(*es).word).word, qflags);
                } else {
                    pattern = malloc(1 as usize) as *mut libc::c_char;
                    *pattern.offset(0 as isize) = '\u{0}' as i32 as libc::c_char;
                }
                match_0 = (c_strmatch(pattern, word, FNMATCH_EXTFLAG!() | FNMATCH_IGNCASE!())
                    != FNM_NOMATCH!()) as libc::c_int;
                free(pattern as *mut c_void);

                dispose_words(es);
                if match_0 != 0 {
                    loop {
                        if !((*clauses).action).is_null() && ignore_return != 0 {
                            (*(*clauses).action).flags |= CMD_IGNORE_RETURN as libc::c_int;
                        }
                        retval = execute_command((*clauses).action);
                        if !((*clauses).flags & CASEPAT_FALLTHROUGH as libc::c_int != 0 && {
                            clauses = (*clauses).next;
                            !clauses.is_null()
                        }) {
                            break;
                        }
                    }
                    if clauses.is_null() || (*clauses).flags & CASEPAT_TESTNEXT as libc::c_int == 0
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
    }
    return retval;
}

fn execute_while_command(while_command: *mut WHILE_COM) -> libc::c_int {
    return execute_while_or_until(while_command, CMD_WHILE!());
}

fn execute_until_command(while_command: *mut WHILE_COM) -> libc::c_int {
    return execute_while_or_until(while_command, CMD_UNTIL!());
}

fn execute_while_or_until(while_command: *mut WHILE_COM, type_0: libc::c_int) -> libc::c_int {
    let mut return_value: libc::c_int;
    let mut body_status: libc::c_int;
    unsafe {
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
    }
    return body_status;
}

fn execute_if_command(if_command: *mut IF_COM) -> libc::c_int {
    let return_value: libc::c_int;
    let save_line_number: libc::c_int;
    unsafe {
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
}

fn execute_arith_command(arith_command: *mut ARITH_COM) -> libc::c_int {
    let mut expok: libc::c_int = 0;
    let save_line_number: libc::c_int;
    let retval: libc::c_int;
    let expresult: intmax_t;
    let mut new: *mut WordList;
    let mut exp: *mut libc::c_char;
    let mut t: *mut libc::c_char;

    // expresult = 0 as intmax_t;
    unsafe {
        save_line_number = line_number;
        this_command_name = b"((\0" as *const u8 as *mut libc::c_char;
        line_number = (*arith_command).line;
        line_number_for_err_trap = line_number;

        if variable_context != 0 && interactive_shell != 0 && sourcelevel == 0 {
            line_number -= function_line_number - 1;
            if line_number <= 0 {
                line_number = 1;
            }
        }

        command_string_index = 0;
        print_arith_command((*arith_command).exp);

        if signal_in_progress(DEBUG_TRAP as libc::c_int) == 0 && running_trap == 0 {
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
    }
    exp = expand_arith_string(exp, Q_DOUBLE_QUOTES as libc::c_int | Q_ARITH as libc::c_int);

    if unsafe { echo_command_at_execute != 0 } {
        new = make_word_list(
            make_word(if !exp.is_null() {
                exp
            } else {
                b"\0" as *const u8 as *const libc::c_char
            }),
            0 as *mut WordList,
        );
        xtrace_print_arith_cmd(new);
        dispose_words(new);
    }

    if !exp.is_null() {
        expresult = evalexp(exp, EXP_EXPANDED as libc::c_int, &mut expok);
        unsafe {
            line_number = save_line_number;
            free(exp as *mut c_void);
        }
    } else {
        expresult = 0 as intmax_t;
        expok = 1;
    }
    unsafe {
        FREE!(t);
    }

    if expok == 0 {
        return EXECUTION_FAILURE as libc::c_int;
    }
    return if expresult == 0 {
        EXECUTION_FAILURE as libc::c_int
    } else {
        EXECUTION_SUCCESS as libc::c_int
    };
}

static mut nullstr: *mut libc::c_char = b"\0" as *const u8 as *mut libc::c_char;

fn execute_cond_node(cond: *mut COND_COM) -> libc::c_int {
    let mut result: libc::c_int;
    let invert: libc::c_int;
    let patmatch: libc::c_int;
    let rmatch: libc::c_int;
    let mut mflags: libc::c_int;
    let ignore: libc::c_int;
    let mut arg1: *mut libc::c_char;
    let mut arg2: *mut libc::c_char;
    unsafe {
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

        if (*cond).type_0 == COND_EXPR as libc::c_int {
            result = execute_cond_node((*cond).left);
        } else if (*cond).type_0 == COND_OR as libc::c_int {
            result = execute_cond_node((*cond).left);
            if result != EXECUTION_SUCCESS as libc::c_int {
                result = execute_cond_node((*cond).right);
            }
        } else if (*cond).type_0 == COND_AND as libc::c_int {
            result = execute_cond_node((*cond).left);
            if result == EXECUTION_SUCCESS as libc::c_int {
                result = execute_cond_node((*cond).right);
            }
        } else if (*cond).type_0 == COND_UNARY as libc::c_int {
            if ignore != 0 {
                comsub_ignore_return += 1;
            }
            arg1 = cond_expand_word((*(*cond).left).op, 0);
            if ignore != 0 {
                comsub_ignore_return -= 1;
            }
            if arg1.is_null() {
                arg1 = nullstr;
            }
            if echo_command_at_execute != 0 {
                xtrace_print_cond_term(
                    (*cond).type_0,
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
        } else if (*cond).type_0 == COND_BINARY as libc::c_int {
            // rmatch = 0;
            patmatch = (*((*(*cond).op).word).offset(1 as isize) as libc::c_int == '=' as i32
                && *((*(*cond).op).word).offset(2 as isize) as libc::c_int == '\u{0}' as i32
                && (*((*(*cond).op).word).offset(0 as isize) as libc::c_int == '!' as i32
                    || *((*(*cond).op).word).offset(0 as isize) as libc::c_int == '=' as i32)
                || *((*(*cond).op).word).offset(0 as isize) as libc::c_int == '=' as i32
                    && *((*(*cond).op).word).offset(1 as isize) as libc::c_int == '\u{0}' as i32)
                as libc::c_int;
            rmatch = (*((*(*cond).op).word).offset(0 as isize) as libc::c_int == '=' as i32
                && *((*(*cond).op).word).offset(1 as isize) as libc::c_int == '~' as i32
                && *((*(*cond).op).word).offset(2 as isize) as libc::c_int == '\u{0}' as i32)
                as libc::c_int;

            if ignore != 0 {
                comsub_ignore_return += 1;
            }
            arg1 = cond_expand_word((*(*cond).left).op, 0);
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
                xtrace_print_cond_term((*cond).type_0, invert, (*cond).op, arg1, arg2);
            }

            if rmatch != 0 {
                mflags = SHMAT_PWARN as libc::c_int;
                mflags |= SHMAT_SUBEXP as libc::c_int;
                result = c_sh_regmatch(arg1, arg2, mflags);
            } else {
                let oe: libc::c_int;

                oe = extended_glob;
                extended_glob = 1;
                result = if binary_test(
                    (*(*cond).op).word,
                    arg1,
                    arg2,
                    TEST_PATMATCH as libc::c_int
                        | TEST_ARITHEXP as libc::c_int
                        | TEST_LOCALE as libc::c_int,
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
                free(arg2 as *mut c_void);
            }
        } else {
            command_error(
                b"execute_cond_node\0" as *const u8 as *const libc::c_char,
                CMDERR_BADTYPE as libc::c_int,
                (*cond).type_0,
                0,
            );
            jump_to_top_level(DISCARD as libc::c_int);
            result = EXECUTION_FAILURE as libc::c_int;
        }
        if invert != 0 {
            result = if result == EXECUTION_FAILURE as libc::c_int {
                EXECUTION_FAILURE as libc::c_int
            } else {
                EXECUTION_SUCCESS as libc::c_int
            };
        }
    }
    return result;
}

fn execute_cond_command(cond_command: *mut COND_COM) -> libc::c_int {
    let mut retval: libc::c_int;
    let save_line_number: libc::c_int;
    unsafe {
        save_line_number = line_number;
        this_command_name = b"[[\0" as *const u8 as *mut libc::c_char;
        line_number = (*cond_command).line;
        line_number_for_err_trap = line_number;
        if variable_context != 0 && interactive_shell != 0 && sourcelevel == 0 {
            line_number -= function_line_number - 1;
            if line_number <= 0 {
                line_number = 1;
            }
        }

        command_string_index = 0;
        print_cond_command(cond_command);
        if signal_in_progress(DEBUG_TRAP as libc::c_int) == 0 && running_trap == 0 {
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
    }
    return retval;
}

fn bind_lastarg(mut arg: *mut libc::c_char) {
    let var: *mut SHELL_VAR;

    if arg.is_null() {
        arg = b"\0" as *const u8 as *mut libc::c_char;
    }
    var = bind_variable(b"_\0" as *const u8 as *const libc::c_char, arg, 0);

    if !var.is_null() {
        unsafe {
            VUNSETATTR!(var, att_exported);
        }
    }
}

fn execute_null_command(
    redirects: *mut REDIRECT,
    pipe_in: libc::c_int,
    pipe_out: libc::c_int,
    async_0: libc::c_int,
) -> libc::c_int {
    let r: libc::c_int;
    let mut forcefork: libc::c_int;
    let fork_flags: libc::c_int;
    let mut rd: *mut REDIRECT;

    forcefork = 0;
    rd = redirects;

    while !rd.is_null() {
        unsafe {
            forcefork += (*rd).rflags & REDIR_VARASSIGN as libc::c_int;
            forcefork += ((*rd).redirector.dest == 0
                || fd_is_bash_input((*rd).redirector.dest) != 0
                    && INPUT_REDIRECT!((*rd).instruction)
                || TRANSLATE_REDIRECT!((*rd).instruction)
                || (*rd).instruction == r_instruction_r_close_this)
                as libc::c_int;
            rd = (*rd).next;
        }
    }

    if forcefork != 0 || pipe_in != NO_PIPE || pipe_out != NO_PIPE || async_0 != 0 {
        fork_flags = if async_0 != 0 {
            FORK_ASYNC as libc::c_int
        } else {
            0
        };
        if make_child(0 as *mut libc::c_char, fork_flags) == 0 {
            restore_original_signals();
            do_piping(pipe_in, pipe_out);
            coproc_closeall();
            unsafe {
                interactive = 0;

                subshell_environment = 0;
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
            }
        } else {
            close_pipes(pipe_in, pipe_out);
            if pipe_out == NO_PIPE {
                unlink_fifo_list();
            }
            return EXECUTION_SUCCESS as libc::c_int;
        }
    } else {
        r = do_redirections(
            redirects,
            RX_ACTIVE as libc::c_int | RX_UNDOABLE as libc::c_int,
        );
        unsafe {
            cleanup_redirects(redirection_undo_list);
            redirection_undo_list = 0 as *mut REDIRECT;

            if r != 0 {
                return EXECUTION_FAILURE as libc::c_int;
            } else if last_command_subst_pid != NO_PID!() {
                return last_command_exit_value;
            } else {
                return EXECUTION_SUCCESS as libc::c_int;
            }
        }
    };
}

fn fix_assignment_words(words: *mut WordList) {
    let mut w: *mut WordList;
    let mut wcmd: *mut WordList;
    let mut b: *mut builtin;
    let mut assoc: libc::c_int;
    let mut global: libc::c_int;
    let mut array: libc::c_int;
    let integer: libc::c_int;

    if words.is_null() {
        return;
    }

    b = 0 as *mut builtin;
    integer = 0;
    array = integer;
    global = array;
    assoc = global;

    // wcmd = words;
    wcmd = words;
    unsafe {
        while !wcmd.is_null() {
            if (*(*wcmd).word).flags & W_ASSIGNMENT as libc::c_int == 0 {
                break;
            }
            wcmd = (*wcmd).next;
        }

        while posixly_correct != 0
            && !wcmd.is_null()
            && !((*wcmd).word).is_null()
            && !((*(*wcmd).word).word).is_null()
            && STREQ!(
                (*(*wcmd).word).word,
                b"command\0" as *const u8 as *const libc::c_char
            )
        {
            wcmd = (*wcmd).next;
        }

        w = wcmd;
        while !w.is_null() {
            if (*(*w).word).flags & W_ASSIGNMENT as libc::c_int != 0 {
                if b.is_null() {
                    b = builtin_address_internal((*(*wcmd).word).word, 0);
                    if b.is_null() || (*b).flags & ASSIGNMENT_BUILTIN as libc::c_int == 0 {
                        return;
                    } else {
                        if !b.is_null() && (*b).flags & ASSIGNMENT_BUILTIN as libc::c_int != 0 {
                            (*(*wcmd).word).flags |= W_ASSNBLTIN as libc::c_int;
                        }
                    }
                }
                (*(*w).word).flags |= (W_NOSPLIT as libc::c_int)
                    | (W_NOGLOB as libc::c_int)
                    | (W_TILDEEXP as libc::c_int)
                    | (W_ASSIGNARG as libc::c_int);
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
                    && (*b).flags
                        & (ASSIGNMENT_BUILTIN as libc::c_int | LOCALVAR_BUILTIN as libc::c_int)
                        == ASSIGNMENT_BUILTIN as libc::c_int
                {
                    (*(*w).word).flags |= W_ASSNGLOBAL as libc::c_int | W_CHKLOCAL as libc::c_int;
                } else if !b.is_null()
                    && (*b).flags & ASSIGNMENT_BUILTIN as libc::c_int != 0
                    && (*b).flags & LOCALVAR_BUILTIN as libc::c_int != 0
                    && variable_context != 0
                {
                    (*(*w).word).flags |= W_FORCELOCAL as libc::c_int;
                }
            } else if *((*(*w).word).word).offset(0 as isize) as libc::c_int == '-' as i32
                && !(strpbrk(
                    ((*(*w).word).word).offset(1 as isize),
                    b"Aag\0" as *const u8 as *const libc::c_char,
                ))
                .is_null()
            {
                if b.is_null() {
                    b = builtin_address_internal((*(*wcmd).word).word, 0);
                    if b.is_null() || (*b).flags & ASSIGNMENT_BUILTIN as libc::c_int == 0 {
                        return;
                    } else {
                        if !b.is_null() && (*b).flags & ASSIGNMENT_BUILTIN as libc::c_int != 0 {
                            (*(*wcmd).word).flags |= W_ASSNBLTIN as libc::c_int;
                        }
                    }
                }
                if (*(*wcmd).word).flags & W_ASSNBLTIN as libc::c_int != 0
                    && !(strchr(((*(*w).word).word).offset(1 as isize), 'A' as i32)).is_null()
                {
                    assoc = 1;
                } else if (*(*wcmd).word).flags & W_ASSNBLTIN as libc::c_int != 0
                    && !(strchr(
                        ((*(*w).word).word).offset(1 as libc::c_int as isize),
                        'a' as i32,
                    ))
                    .is_null()
                {
                    array = 1;
                }
                if (*(*wcmd).word).flags & W_ASSNBLTIN as libc::c_int != 0
                    && !(strchr(((*(*w).word).word).offset(1 as isize), 'g' as i32)).is_null()
                {
                    global = 1;
                }
            }
            w = (*w).next;
        }
    }
}

fn check_command_builtin(words: *mut WordList, typep: *mut libc::c_int) -> *mut WordList {
    let mut type_0: libc::c_int;
    let mut w: *mut WordList;

    #[macro_export]
    macro_rules! RETURN_NOT_COMMAND {
        () => {
            if !typep.is_null() {
                *typep = 0;
            }
            return words;
        };
    }
    unsafe {
        w = (*words).next;
        type_0 = 1;

        if !w.is_null() && ISOPTION!((*(*w).word).word, 'p') {
            if restricted != 0 {
                RETURN_NOT_COMMAND!();
            }
            w = (*w).next;
            type_0 = 2;
        }
        if !w.is_null() && ISOPTION!((*(*w).word).word, '-') {
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
    }
    return w;
}

fn is_dirname(pathname: *mut libc::c_char) -> libc::c_int {
    let temp: *mut libc::c_char;
    let ret: libc::c_int;

    temp = search_for_command(pathname, 0);
    ret = if !temp.is_null() {
        file_isdir(temp)
    } else {
        file_isdir(pathname)
    };
    unsafe {
        free(temp as *mut c_void);
    }

    return ret;
}

fn execute_simple_command(
    simple_command: *mut SIMPLE_COM,
    mut pipe_in: libc::c_int,
    mut pipe_out: libc::c_int,
    async_0: libc::c_int,
    fds_to_close: *mut fd_bitmap,
) -> libc::c_int {
    let mut current_block: u64;
    let mut words: *mut WordList;
    let mut lastword: *mut WordList;
    let mut command_line: *mut libc::c_char;
    let lastarg: *mut libc::c_char;
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let first_word_quoted: libc::c_int;
    let mut result: libc::c_int;
    let mut builtin_is_special: libc::c_int;
    let mut already_forked: libc::c_int;
    let mut dofork: libc::c_int;
    let fork_flags: libc::c_int;
    let mut cmdflags: libc::c_int;
    let old_last_async_pid: pid_t;
    let mut builtin: Option<sh_builtin_func_t>;
    let mut func: *mut SHELL_VAR;
    let mut old_builtin: libc::c_int = 0;
    let mut old_command_builtin: libc::c_int;
    unsafe {
        // result = EXECUTION_SUCCESS as libc::c_int;
        builtin_is_special = 0;
        special_builtin_failed = builtin_is_special;
        command_line = 0 as *mut libc::c_char;

        QUIT!();

        if variable_context != 0 && interactive_shell != 0 && sourcelevel == 0 {
            line_number -= function_line_number - 1;
            if line_number <= 0 {
                line_number = 1;
            }
        }

        command_string_index = 0;
        print_simple_command(simple_command);

        if signal_in_progress(DEBUG_TRAP as libc::c_int) == 0 && running_trap == 0 {
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
            (*(*(*simple_command).words).word).flags & W_QUOTED as libc::c_int
        } else {
            0
        };

        last_command_subst_pid = NO_PID!();
        old_last_async_pid = last_asynchronous_pid;

        already_forked = 0;

        dofork = (pipe_in != NO_PIPE as libc::c_int
            || pipe_out != NO_PIPE as libc::c_int
            || async_0 != 0) as libc::c_int;

        if dofork != 0
            && pipe_in == NO_PIPE as libc::c_int
            && pipe_out == NO_PIPE as libc::c_int
            && !((*simple_command).words).is_null()
            && !((*(*simple_command).words).word).is_null()
            && !((*(*(*simple_command).words).word).word).is_null()
            && *((*(*(*simple_command).words).word).word).offset(0 as isize) as libc::c_int
                == '%' as i32
        {
            dofork = 0;
        }

        if dofork != 0 {
            let p: *mut libc::c_char;

            maybe_make_export_env();

            fork_flags = if async_0 != 0 {
                FORK_ASYNC as libc::c_int
            } else {
                0
            };
            p = savestring!(the_printed_command_except_trap);
            if make_child(p, fork_flags) == 0 {
                already_forked = 1;
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
                // command_line = 0 as *mut libc::c_char;
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

        // lastarg = 0 as *mut libc::c_char;
        begin_unwind_frame(b"simple-command\0" as *const u8 as *mut libc::c_char);

        if echo_command_at_execute != 0 && cmdflags & CMD_COMMAND_BUILTIN as libc::c_int == 0 {
            xtrace_print_word_list(words, 1);
        }

        builtin = None;

        func = 0 as *mut SHELL_VAR;

        if cmdflags & CMD_NO_FUNCTIONS as libc::c_int == 0 {
            if posixly_correct != 0 {
                builtin = find_special_builtin((*(*words).word).word);
                if builtin.is_some() {
                    builtin_is_special = 1;
                }
            }
            if builtin.is_none() {
                func = find_function((*(*words).word).word);
            }
        }

        if posixly_correct != 0
            && builtin_is_special != 0
            && interactive_shell == 0
            && tempenv_assign_error != 0
        {
            last_command_exit_value = EXECUTION_FAILURE as libc::c_int;
            jump_to_top_level(ERREXIT as libc::c_int);
        }
        tempenv_assign_error = 0;
        old_command_builtin = -1;

        if builtin.is_none() && func.is_null() {
            let mut disposer: *mut WordList;
            let mut l: *mut WordList;
            let mut cmdtype: libc::c_int;

            builtin = find_shell_builtin((*(*words).word).word);

            while builtin == Some(command_builtin) {
                disposer = words;
                cmdtype = 0;
                words = check_command_builtin(words, &mut cmdtype);
                if !(cmdtype > 0) {
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
                executing_command_builtin |= 1;
            }
            builtin = None;
        }
        add_unwind_protect(
            transmute::<fn(arg1: *mut WordList), Option<Function>>(dispose_words),
            words as *mut libc::c_char,
        );

        QUIT!();

        lastword = words;
        while !((*lastword).next).is_null() {
            lastword = (*lastword).next;
        }

        lastarg = (*(*lastword).word).word;

        if *((*(*words).word).word).offset(0 as isize) as libc::c_int == '%' as i32
            && already_forked == 0
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
            if job_control != 0
                && already_forked == 0
                && async_0 == 0
                && first_word_quoted == 0
                && ((*words).next).is_null()
                && *((*(*words).word).word).offset(0 as isize) as libc::c_int != 0
                && ((*simple_command).redirects).is_null()
                && pipe_in == NO_PIPE as libc::c_int
                && pipe_out == NO_PIPE as libc::c_int
                && {
                    temp = get_string_value(b"auto_resume\0" as *const u8 as *const libc::c_char);
                    !temp.is_null()
                }
            {
                let job: libc::c_int;
                let mut jflags: libc::c_int;
                let started_status: libc::c_int;

                jflags = JM_STOPPED as libc::c_int | JM_FIRSTMATCH as libc::c_int;
                if STREQ!(temp, b"exact" as *const u8 as *mut libc::c_char) {
                    jflags |= JM_EXACT as libc::c_int;
                } else if STREQ!(temp, b"substring" as *const u8 as *mut libc::c_char) {
                    jflags |= JM_SUBSTRING as libc::c_int;
                } else {
                    jflags |= JM_PREFIX as libc::c_int;
                }
                job = get_job_by_name((*(*words).word).word, jflags);
                if job != NO_JOB {
                    run_unwind_frame(b"simple-command\0" as *const u8 as *mut libc::c_char);
                    this_command_name = b"fg\0" as *const u8 as *mut libc::c_char;
                    last_shell_builtin = this_shell_builtin;
                    this_shell_builtin = builtin_address(b"fg\0" as *const u8 as *mut libc::c_char);

                    started_status = start_job(job, 1);
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
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
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
                                && stdin_redirects((*simple_command).redirects) == 0
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
                if !(autocd != 0
                    && interactive != 0
                    && !((*words).word).is_null()
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
                                if posixly_correct != 0
                                    && builtin_is_special != 0
                                    && interactive_shell == 0
                                {
                                    last_command_exit_value = EXECUTION_FAILURE as libc::c_int;
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
                            if posixly_correct != 0
                                && builtin_is_special != 0
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
        discard_unwind_frame(b"simple-command\0" as *const u8 as *mut libc::c_char);
        this_command_name = 0 as *mut libc::c_char;

        return result;
    }
}

fn builtin_status(result: libc::c_int) -> libc::c_int {
    let r: libc::c_int;

    match result as libc::c_uint {
        EX_USAGE!() | EX_BADSYNTAX => {
            r = EX_BADUSAGE as libc::c_int;
        }
        EX_REDIRFAIL | EX_BADASSIGN!() | EX_EXPFAIL => {
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
macro_rules! unwind_protect_int {
    ($var:expr) => {
        unwind_protect_mem(
            &mut $var as *mut libc::c_int as *mut libc::c_char,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        );
    };
}

fn execute_builtin(
    builtin: Option<sh_builtin_func_t>,
    words: *mut WordList,
    flags: libc::c_int,
    subshell: libc::c_int,
) -> libc::c_int {
    let result: libc::c_int;
    let eval_unwind: libc::c_int;
    let mut ignexit_flag: libc::c_int = 0;
    let mut isbltinenv: libc::c_int;
    let mut should_keep: libc::c_int;
    let mut error_trap: *mut libc::c_char;

    error_trap = 0 as *mut libc::c_char;
    // should_keep = 0 as libc::c_int;

    if subshell == 0
        && flags & CMD_IGNORE_RETURN as libc::c_int != 0
        && (builtin == Some(eval_builtin)
            || flags & 0x800 as libc::c_int != 0
            || builtin == Some(source_builtin))
    {
        unsafe {
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
            error_trap = TRAP_STRING!(ERROR_TRAP as libc::c_int);
            if !error_trap.is_null() {
                error_trap = savestring!(error_trap);
                add_unwind_protect(
                    transmute::<
                        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void),
                        Option<Function>,
                    >(libc::free),
                    error_trap,
                );
                add_unwind_protect(
                    transmute::<fn(arg1: *mut ::std::os::raw::c_char), Option<Function>>(
                        set_error_trap,
                    ),
                    error_trap,
                );
                restore_default_signal(ERROR_TRAP as libc::c_int);
            }
            exit_immediately_on_error = 0;
            ignexit_flag = builtin_ignoring_errexit;
            builtin_ignoring_errexit = 1;
            eval_unwind = 1;
        }
    } else {
        eval_unwind = 0;
    }

    isbltinenv = (builtin == Some(source_builtin)
        || builtin == Some(eval_builtin)
        || builtin == Some(unset_builtin)
        || builtin == Some(mapfile_builtin)) as libc::c_int;
    should_keep = (isbltinenv != 0 && builtin != Some(mapfile_builtin)) as libc::c_int;
    if builtin == Some(fc_builtin) || builtin == Some(read_builtin) {
        isbltinenv = 1;
        should_keep = 0;
    }

    if isbltinenv != 0 {
        if subshell == 0 {
            begin_unwind_frame(
                b"builtin_env\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        if unsafe { !temporary_env.is_null() } {
            unsafe {
                push_scope(VC_BLTNENV as libc::c_int, temporary_env);
            }
            if flags & CMD_COMMAND_BUILTIN as libc::c_int != 0 {
                should_keep = 0;
            }
            if subshell == 0 {
                unsafe {
                    add_unwind_protect(
                        transmute::<fn(arg1: ::std::os::raw::c_int), Option<Function>>(pop_scope),
                        if should_keep != 0 {
                            b"1\0" as *const u8 as *mut libc::c_char
                        } else {
                            0 as *mut libc::c_char
                        },
                    );
                }
                unsafe {
                    temporary_env = 0 as *mut HASH_TABLE;
                }
            }
        }
    }

    if subshell == 0 && builtin == Some(eval_builtin) {
        unsafe {
            if evalnest_max > 0 && evalnest >= evalnest_max {
                internal_error(
                    b"eval: maximum eval nesting level exceeded (%d)\0" as *const u8
                        as *mut libc::c_char,
                    evalnest,
                );
                evalnest = 0;
                jump_to_top_level(DISCARD as libc::c_int);
            }
            unwind_protect_int!(evalnest);
            evalnest += 1;
        }
    } else if subshell == 0 && builtin == Some(source_builtin) {
        unsafe {
            if sourcenest_max > 0 && sourcenest >= sourcenest_max {
                internal_error(
                    b"%s: maximum source nesting level exceeded (%d)\0" as *const u8
                        as *mut libc::c_char,
                    this_command_name,
                    sourcenest,
                );
                sourcenest = 0;
                jump_to_top_level(DISCARD as libc::c_int);
            }
            unwind_protect_int!(sourcenest);
            sourcenest += 1;
        }
    }
    unsafe {
        if posixly_correct != 0
            && subshell == 0
            && builtin == Some(return_builtin)
            && flags & 0x800 as libc::c_int == 0 as libc::c_int
            && !temporary_env.is_null()
        {
            begin_unwind_frame(b"return_temp_env\0" as *const u8 as *mut libc::c_char);
            add_unwind_protect(
                transmute::<fn(), Option<Function>>(merge_temporary_env),
                0 as *mut libc::c_char,
            );
        }

        executing_builtin += 1;
        executing_command_builtin |= (builtin == Some(command_builtin)) as libc::c_int;

        result = exec_cmd((*(*words).word).word, (*words).next);

        if posixly_correct != 0
            && subshell == 0
            && builtin == Some(return_builtin)
            && !temporary_env.is_null()
        {
            discard_unwind_frame(b"return_temp_env\0" as *const u8 as *mut libc::c_char);
        }
        if subshell == 0 && isbltinenv != 0 {
            run_unwind_frame(b"builtin_env\0" as *const u8 as *mut libc::c_char);
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
            discard_unwind_frame(b"eval_builtin\0" as *const u8 as *mut libc::c_char);
        }
    }
    return result;
}

fn maybe_restore_getopt_state(gs: *mut sh_getopt_state_t) {
    unsafe {
        if (*gs).gs_flags & 1 != 0 {
            sh_getopt_restore_istate(gs);
        } else {
            free(gs as *mut c_void);
        };
    }
}

#[no_mangle]
pub fn restore_funcarray_state(fa: *mut func_array_state) {
    let nfv: *mut SHELL_VAR;
    let funcname_a: *mut ARRAY;
    unsafe {
        array_pop!((*fa).source_a);
        array_pop!((*fa).lineno_a);

        GET_ARRAY_FROM_VAR!(
            b"FUNCNAME\0" as *const u8 as *const libc::c_char,
            nfv,
            funcname_a
        );
        if nfv == (*fa).funcname_v {
            array_pop!(funcname_a);
        }
        free(fa as *mut c_void);
    }
}

fn execute_function(
    var: *mut SHELL_VAR,
    words: *mut WordList,
    flags: libc::c_int,
    fds_to_close: *mut fd_bitmap,
    async_0: libc::c_int,
    subshell: libc::c_int,
) -> libc::c_int {
    let return_val: libc::c_int;
    let mut result: libc::c_int;
    let tc: *mut COMMAND;
    let fc: *mut COMMAND;
    let mut save_current: *mut COMMAND;
    let mut debug_trap: *mut libc::c_char;
    let mut error_trap: *mut libc::c_char;
    let mut return_trap: *mut libc::c_char;
    let funcname_v: *mut SHELL_VAR;
    let bash_source_v: *mut SHELL_VAR;
    let bash_lineno_v: *mut SHELL_VAR;
    let funcname_a: *mut ARRAY;
    let bash_source_a: *mut ARRAY;
    let bash_lineno_a: *mut ARRAY;
    let fa: *mut func_array_state;
    let shell_fn: *mut FUNCTION_DEF;
    let sfile: *mut libc::c_char;
    let t: *mut libc::c_char;
    let gs: *mut sh_getopt_state_t;
    let gv: *mut SHELL_VAR;
    unsafe {
        if funcnest_max > 0 && funcnest >= funcnest_max {
            internal_error(
                b"%s: maximum function nesting level exceeded (%d)\0" as *const u8
                    as *mut libc::c_char,
                (*var).name,
                funcnest,
            );
            funcnest = 0;
            jump_to_top_level(DISCARD as libc::c_int);
        }

        GET_ARRAY_FROM_VAR!(
            b"FUNCNAME\0" as *const u8 as *const libc::c_char,
            funcname_v,
            funcname_a
        );
        GET_ARRAY_FROM_VAR!(
            b"BASH_SOURCE\0" as *const u8 as *const libc::c_char,
            bash_source_v,
            bash_source_a
        );
        GET_ARRAY_FROM_VAR!(
            b"BASH_LINENO\0" as *const u8 as *const libc::c_char,
            bash_lineno_v,
            bash_lineno_a
        );

        tc = copy_command((*var).value as *mut COMMAND);
        if !tc.is_null() && flags & CMD_IGNORE_RETURN as libc::c_int != 0 {
            (*tc).flags |= CMD_IGNORE_RETURN as libc::c_int;
        }

        if !tc.is_null()
            && flags & CMD_NO_FORK as libc::c_int != 0
            && subshell_environment & SUBSHELL_COMSUB as libc::c_int != 0
        {
            optimize_shell_function(tc);
        }

        gs = sh_getopt_save_istate();
        if subshell == 0 {
            begin_unwind_frame(b"function_calling\0" as *const u8 as *mut libc::c_char);
            push_context((*var).name, subshell, temporary_env);

            add_unwind_protect(
                transmute::<fn(*mut sh_getopt_state_t) -> (), Option<Function>>(
                    maybe_restore_getopt_state,
                ),
                gs as *mut libc::c_char,
            );
            add_unwind_protect(
                transmute::<fn(), Option<Function>>(pop_context),
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
                transmute::<fn(arg1: *mut COMMAND), Option<Function>>(dispose_command),
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
        make_funcname_visible(1);

        debug_trap = TRAP_STRING!((64 + 1) as libc::c_int);
        error_trap = TRAP_STRING!(((64 + 1) + 1) as libc::c_int);
        return_trap = TRAP_STRING!(((64 + 1) + 2) as libc::c_int);

        if !debug_trap.is_null() && (trace_p!(var) == 0 && function_trace_mode == 0) {
            if subshell == 0 {
                debug_trap = savestring!(debug_trap);
                add_unwind_protect(
                    transmute::<
                        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void),
                        Option<Function>,
                    >(libc::free),
                    debug_trap,
                );
                add_unwind_protect(
                    transmute::<fn(arg1: *mut ::std::os::raw::c_char), Option<Function>>(
                        maybe_set_debug_trap,
                    ),
                    debug_trap,
                );
            }
            restore_default_signal((64 + 1) as libc::c_int);
        }

        if !error_trap.is_null() && error_trace_mode == 0 {
            if subshell == 0 {
                error_trap = savestring!(error_trap);
                add_unwind_protect(
                    transmute::<
                        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void),
                        Option<Function>,
                    >(libc::free),
                    error_trap,
                );
                add_unwind_protect(
                    transmute::<fn(arg1: *mut ::std::os::raw::c_char), Option<Function>>(
                        maybe_set_error_trap,
                    ),
                    error_trap,
                );
            }
            restore_default_signal(((64 + 1) + 1) as libc::c_int);
        }

        if !return_trap.is_null()
            && (signal_in_progress(DEBUG_TRAP as libc::c_int) != 0
                || trace_p!(var) == 0 && function_trace_mode == 0)
        {
            if subshell == 0 {
                return_trap = savestring!(return_trap);
                add_unwind_protect(
                    transmute::<
                        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void),
                        Option<Function>,
                    >(libc::free),
                    return_trap,
                );
                add_unwind_protect(
                    transmute::<fn(arg1: *mut ::std::os::raw::c_char), Option<Function>>(
                        maybe_set_return_trap,
                    ),
                    return_trap,
                );
            }
            restore_default_signal(((64 + 1) + 2) as libc::c_int);
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
        t = c_itos(executing_line_number() as intmax_t);
        array_push!(bash_lineno_a, t);
        free(t as *mut c_void);

        fa = libc::malloc(size_of::<func_array_state>()) as *mut func_array_state;
        (*fa).source_a = bash_source_a as *mut ARRAY;
        (*fa).source_v = bash_source_v;
        (*fa).lineno_a = bash_lineno_a as *mut ARRAY;
        (*fa).lineno_v = bash_lineno_v;
        (*fa).funcname_a = funcname_a;
        (*fa).funcname_v = funcname_v;

        if subshell == 0 as libc::c_int {
            add_unwind_protect(
                transmute::<fn(*mut func_array_state) -> (), Option<Function>>(
                    restore_funcarray_state,
                ),
                fa as *mut libc::c_char,
            );
        }

        if debugging_mode != 0 || shell_compatibility_level <= 44 {
            init_bash_argv();
        }

        remember_args((*words).next, 1);

        if debugging_mode != 0 {
            push_args((*words).next);
            if subshell == 0 {
                add_unwind_protect(
                    transmute::<fn(), Option<Function>>(pop_args),
                    0 as *mut libc::c_char,
                );
            }
        }

        function_line_number = (*tc).line;
        line_number = function_line_number;

        if subshell != 0 {
            stop_pipeline(async_0, 0 as *mut COMMAND);
        }
        if shell_compatibility_level > 43 {
            loop_level = 0;
        }

        fc = tc;

        from_return_trap = 0;

        return_catch_flag += 1;
        return_val = setjmp_nosigs!(return_catch.as_mut_ptr());

        if return_val != 0 {
            result = return_catch_value;
            save_current = currently_executing_command;
            if from_return_trap == 0 {
                run_return_trap();
            }
            currently_executing_command = save_current;
        } else {
            showing_function_line = 1;
            save_current = currently_executing_command;
            result = run_debug_trap();
            if debugging_mode == 0 || result == EXECUTION_SUCCESS as libc::c_int {
                showing_function_line = 0;
                currently_executing_command = save_current;
                result = execute_command_internal(fc, 0, NO_PIPE, NO_PIPE, fds_to_close);

                save_current = currently_executing_command;
                run_return_trap();
                currently_executing_command = save_current;
            }

            showing_function_line = 0;
        }
        gv = find_variable(b"OPTIND\0" as *const u8 as *const libc::c_char);
        if !gv.is_null() && (*gv).context == variable_context {
            (*gs).gs_flags |= 1;
        }

        if subshell == 0 {
            run_unwind_frame(
                b"function_calling\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else {
            restore_funcarray_state(fa);
            if debugging_mode != 0 {
                pop_args();
            }
        }
        if variable_context == 0 || this_shell_function.is_null() {
            make_funcname_visible(0);
            unlink_fifo_list();
        }
    }
    return result;
}

#[no_mangle]
pub fn execute_shell_function(var: *mut SHELL_VAR, words: *mut WordList) -> libc::c_int {
    let ret: libc::c_int;
    let bitmap: *mut fd_bitmap;

    bitmap = new_fd_bitmap(FD_BITMAP_DEFAULT_SIZE!());
    begin_unwind_frame(b"execute-shell-function\0" as *const u8 as *mut libc::c_char);
    unsafe {
        add_unwind_protect(
            transmute::<fn(fdbp: *mut fd_bitmap), Option<Function>>(dispose_fd_bitmap),
            bitmap as *mut libc::c_char,
        );
    }
    ret = execute_function(var, words, 0, bitmap, 0, 0);

    dispose_fd_bitmap(bitmap);
    discard_unwind_frame(
        b"execute-shell-function\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );

    return ret;
}

fn execute_subshell_builtin_or_function(
    words: *mut WordList,
    redirects: *mut REDIRECT,
    builtin: Option<sh_builtin_func_t>,
    var: *mut SHELL_VAR,
    pipe_in: libc::c_int,
    pipe_out: libc::c_int,
    async_0: libc::c_int,
    fds_to_close: *mut fd_bitmap,
    flags: libc::c_int,
) {
    let result: libc::c_int;
    let mut r: libc::c_int;
    let mut funcvalue: libc::c_int;
    let jobs_hack: libc::c_int;
    unsafe {
        jobs_hack = (builtin == Some(jobs_builtin)
            && (subshell_environment & 0x1 as libc::c_int == 0 as libc::c_int
                || pipe_out != -(1 as libc::c_int))) as libc::c_int;
        interactive = 0 as libc::c_int;
        login_shell = interactive;

        if builtin == Some(eval_builtin) {
            evalnest = 0 as libc::c_int;
        } else if builtin == Some(source_builtin) {
            sourcenest = 0;
        }
        if async_0 != 0 {
            subshell_environment |= SUBSHELL_ASYNC as libc::c_int;
        }
        if pipe_in != NO_PIPE || pipe_out != NO_PIPE {
            subshell_environment |= SUBSHELL_PIPE as libc::c_int;
        }
    }
    maybe_make_export_env();

    if jobs_hack != 0 {
        kill_current_pipeline();
    } else {
        without_job_control();
    }

    set_sigchld_handler();

    set_sigint_handler();

    if !fds_to_close.is_null() {
        close_fd_bitmap(fds_to_close);
    }

    do_piping(pipe_in, pipe_out);

    if do_redirections(redirects, RX_ACTIVE as libc::c_int) != 0 {
        unsafe {
            exit(EXECUTION_FAILURE as libc::c_int);
        }
    }
    if builtin.is_some() {
        result = unsafe { setjmp_nosigs!(top_level.as_mut_ptr()) };
        funcvalue = 0;
        if unsafe { return_catch_flag != 0 && builtin == Some(return_builtin) } {
            funcvalue = unsafe { setjmp_nosigs!(return_catch.as_mut_ptr()) };
        }

        if result == EXITPROG as libc::c_int {
            unsafe {
                subshell_exit(last_command_exit_value);
            }
        } else if result != 0 {
            subshell_exit(EXECUTION_FAILURE as libc::c_int);
        } else if funcvalue != 0 {
            unsafe {
                subshell_exit(return_catch_value);
            }
        } else {
            r = execute_builtin(builtin, words, flags, 1);
            unsafe {
                fflush(stdout);
            }
            if r == EX_USAGE as libc::c_int {
                r = EX_BADUSAGE as libc::c_int;
            } else if r == EX_DISKFALLBACK as libc::c_int {
                let command_line: *mut libc::c_char = 0 as *mut libc::c_char;
                unsafe {
                    savestring!(if !the_printed_command_except_trap.is_null() {
                        the_printed_command_except_trap
                    } else {
                        b"\0" as *const u8 as *mut libc::c_char
                    });
                }
                r = execute_disk_command(
                    words,
                    0 as *mut REDIRECT,
                    command_line,
                    -1,
                    -1,
                    async_0,
                    0 as *mut fd_bitmap,
                    flags | CMD_NO_FORK as libc::c_int,
                );
            }
            subshell_exit(r);
        }
    } else {
        r = execute_function(var, words, flags, fds_to_close, async_0, 1);
        unsafe {
            fflush(stdout);
        }
        subshell_exit(r);
    };
}

fn execute_builtin_or_function(
    words: *mut WordList,
    builtin: Option<sh_builtin_func_t>,
    var: *mut SHELL_VAR,
    redirects: *mut REDIRECT,
    fds_to_close: *mut fd_bitmap,
    flags: libc::c_int,
) -> libc::c_int {
    let result: libc::c_int;
    let mut saved_undo_list: *mut REDIRECT;
    let ofifo: libc::c_int;
    let nfifo: libc::c_int;
    let mut osize: libc::c_int = 0;
    let ofifo_list: *mut libc::c_void;

    begin_unwind_frame(b"saved_fifos\0" as *const u8 as *mut libc::c_char);
    ofifo = num_fifos();
    ofifo_list = copy_fifo_list(&mut osize);
    if !ofifo_list.is_null() {
        unsafe {
            add_unwind_protect(
                transmute::<
                    unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void),
                    Option<Function>,
                >(libc::free),
                ofifo_list as *mut libc::c_char,
            );
        }
    }

    if do_redirections(
        redirects,
        RX_ACTIVE as libc::c_int | RX_UNDOABLE as libc::c_int,
    ) != 0
    {
        undo_partial_redirects();
        dispose_exec_redirects();
        unsafe {
            free(ofifo_list as *mut c_void);
        }
        return EX_REDIRFAIL as libc::c_int;
    }
    saved_undo_list = unsafe { redirection_undo_list };
    if builtin == Some(exec_builtin) {
        dispose_redirects(saved_undo_list);
        unsafe {
            saved_undo_list = exec_redirection_undo_list;
            exec_redirection_undo_list = 0 as *mut REDIRECT;
        }
    } else {
        dispose_exec_redirects();
    }

    if !saved_undo_list.is_null() {
        begin_unwind_frame(b"saved-redirects\0" as *const u8 as *mut libc::c_char);
        unsafe {
            add_unwind_protect(
                transmute::<fn(*mut REDIRECT) -> (), Option<Function>>(cleanup_redirects),
                saved_undo_list as *mut libc::c_char,
            );
        }
    }

    unsafe {
        redirection_undo_list = 0 as *mut REDIRECT;
    }

    if builtin.is_some() {
        result = execute_builtin(builtin, words, flags, 0);
    } else {
        result = execute_function(var, words, flags, fds_to_close, 0, 0);
    }

    unsafe {
        fflush(stdout);
        c_fpurge(stdout);
        if ferror(stdout) != 0 {
            c_clearerr(stdout);
        }
    }
    if unsafe { builtin == Some(command_builtin) && this_shell_builtin == Some(exec_builtin) } {
        let mut discard: libc::c_int;

        discard = 0;
        if !saved_undo_list.is_null() {
            dispose_redirects(saved_undo_list);
            discard = 1;
        }
        unsafe {
            redirection_undo_list = exec_redirection_undo_list;
            exec_redirection_undo_list = 0 as *mut REDIRECT;
            saved_undo_list = exec_redirection_undo_list;
        }
        if discard != 0 {
            discard_unwind_frame(b"saved-redirects\0" as *const u8 as *mut libc::c_char);
        }
    }
    if !saved_undo_list.is_null() {
        unsafe {
            redirection_undo_list = saved_undo_list;
        }
        discard_unwind_frame(b"saved-redirects\0" as *const u8 as *mut libc::c_char);
    }

    undo_partial_redirects();

    nfifo = num_fifos();
    if nfifo > ofifo {
        close_new_fifos(ofifo_list, osize);
    }
    if !ofifo_list.is_null() {
        unsafe {
            free(ofifo_list as *mut c_void);
        }
    }
    discard_unwind_frame(b"saved_fifos\0" as *const u8 as *const libc::c_char as *mut libc::c_char);

    return result;
}

#[no_mangle]
pub fn setup_async_signals() {
    if unsafe { job_control == 0 } {
        get_original_signal(SIGINT as libc::c_int);
        unsafe {
            set_signal_handler(SIGINT as libc::c_int, SIG_IGN!());
        }
        get_original_signal(SIGQUIT as libc::c_int);
        unsafe {
            set_signal_handler(SIGQUIT as libc::c_int, SIG_IGN!());
        }
    }
}

fn execute_disk_command(
    words: *mut WordList,
    redirects: *mut REDIRECT,
    command_line: *mut libc::c_char,
    pipe_in: libc::c_int,
    pipe_out: libc::c_int,
    async_0: libc::c_int,
    fds_to_close: *mut fd_bitmap,
    cmdflags: libc::c_int,
) -> libc::c_int {
    let mut pathname: *mut libc::c_char;
    let mut command: *mut libc::c_char;
    let args: *mut *mut libc::c_char;
    let mut p: *mut libc::c_char;
    let nofork: libc::c_int;
    let stdpath: libc::c_int = 0;
    let mut result: libc::c_int;
    let fork_flags: libc::c_int;
    let pid: pid_t;
    let hookf: *mut SHELL_VAR;
    let wl: *mut WordList;

    nofork = cmdflags & CMD_NO_FORK as libc::c_int;
    pathname = unsafe { (*(*words).word).word };

    p = 0 as *mut libc::c_char;
    result = EXECUTION_SUCCESS as libc::c_int;
    command = 0 as *mut libc::c_char;
    if unsafe { restricted != 0 && !(c_mbschr(pathname, '/' as i32)).is_null() } {
        unsafe {
            internal_error(
                b"%s: restricted: cannot specify `/' in command names\0" as *const u8
                    as *mut libc::c_char,
                pathname,
            );
            last_command_exit_value = EXECUTION_FAILURE as libc::c_int;
            result = last_command_exit_value;

            if nofork != 0 && pipe_in == NO_PIPE && pipe_out == NO_PIPE {
                exit(last_command_exit_value);
            }
        }
    } else {
        command = search_for_command(
            pathname,
            CMDSRCH_HASH as libc::c_int
                | (if stdpath != 0 {
                    CMDSRCH_STDPATH as libc::c_int
                } else {
                    0
                }),
        );
        unsafe {
            QUIT!();
        }

        if !command.is_null() {
            if nofork != 0 && pipe_in == NO_PIPE && pipe_out == NO_PIPE {
                adjust_shell_level(-1);
            }
            maybe_make_export_env();
            put_command_name_into_env(command);
        }

        if nofork != 0 && pipe_in == NO_PIPE && pipe_out == NO_PIPE {
            pid = 0;
        } else {
            fork_flags = if async_0 != 0 {
                FORK_ASYNC as libc::c_int
            } else {
                0
            };
            p = unsafe { savestring!(command_line) };
            pid = make_child(p, fork_flags);
        }

        if pid == 0 {
            let old_interactive: libc::c_int;

            reset_terminating_signals();
            restore_original_signals();

            unsafe {
                FREE!(p);
            }

            if async_0 != 0 {
                if cmdflags & CMD_STDIN_REDIR as libc::c_int != 0
                    && pipe_in == NO_PIPE
                    && stdin_redirects(redirects) == 0
                {
                    async_redirect_stdin();
                }
                setup_async_signals();
            }

            if !fds_to_close.is_null() {
                close_fd_bitmap(fds_to_close);
            }

            do_piping(pipe_in, pipe_out);

            old_interactive = unsafe { interactive };

            if async_0 != 0 {
                unsafe {
                    interactive = 0;
                }
            }

            unsafe {
                subshell_environment |= SUBSHELL_FORK as libc::c_int;
            }

            if !redirects.is_null() && do_redirections(redirects, RX_ACTIVE as libc::c_int) != 0 {
                unlink_fifo_list();
                unsafe {
                    exit(EXECUTION_FAILURE as libc::c_int);
                }
            }

            if async_0 != 0 {
                unsafe {
                    interactive = old_interactive;
                }
            }

            if command.is_null() {
                hookf = find_function(NOTFOUND_HOOK!());
                if hookf.is_null() {
                    pathname = printable_filename(pathname, 0);
                    unsafe {
                        internal_error(
                            b"%s: command not found\0" as *const u8 as *mut libc::c_char,
                            pathname as *mut libc::c_char,
                        );
                        exit(127 as libc::c_int);
                    }
                }

                without_job_control();

                set_sigchld_handler();

                wl = make_word_list(make_word(NOTFOUND_HOOK!()), words);
                unsafe {
                    exit(execute_shell_function(hookf, wl));
                }
            }
            args = c_strvec_from_word_list(words, 0, 0, 0 as *mut libc::c_int);
            unsafe {
                exit(shell_execve(command, args, export_env));
            }
        }
    }
    unsafe {
        QUIT!();
    }

    close_pipes(pipe_in, pipe_out);
    unsafe {
        FREE!(command);
    }

    return result;
}

fn getinterp(
    sample: *mut libc::c_char,
    sample_len: libc::c_int,
    endp: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut i: libc::c_int;
    let execname: *mut libc::c_char;
    let start: libc::c_int;

    #[macro_export]
    macro_rules! STRINGCHAR {
        ($ind:expr) => {
            $ind < sample_len
                && !whitespace!(sample.offset($ind as isize))
                && $ind as libc::c_int != '\n' as i32
        };
    }

    i = 2;
    while i < sample_len && unsafe { whitespace!(*sample.offset(i as isize)) } {
        i += 1;
    }
    start = i;
    while unsafe { STRINGCHAR!(i) } {
        i += 1;
    }
    execname = substring(sample, start, i);
    if !endp.is_null() {
        unsafe {
            *endp = i;
        }
    }

    return execname;
}

fn initialize_subshell() {
    delete_all_aliases();
    unsafe {
        history_lines_this_session = 0;
    }

    without_job_control();

    set_sigchld_handler();
    init_job_stats();

    reset_shell_flags();
    reset_shell_options();
    reset_shopt_options();
    unsafe {
        if vc_isbltnenv!(shell_variables) {
            shell_variables = (*shell_variables).down;
        }

        clear_unwind_protect_list(0);
        parse_and_execute_level = 0;
        sourcenest = 0;
        evalnest = sourcenest;
        funcnest = evalnest;
        return_catch_flag = funcnest;
        variable_context = return_catch_flag;

        executing_list = 0;
        if interactive_shell == 0 {
            unset_bash_input(0);
        }
    }
}

#[no_mangle]
pub fn shell_execve(
    command: *mut libc::c_char,
    mut args: *mut *mut libc::c_char,
    env: *mut *mut libc::c_char,
) -> libc::c_int {
    let larray: libc::c_int;
    let mut i: libc::c_int;
    // let fd: libc::c_int = 0;
    let mut sample: [libc::c_char; 128] = [0; 128];
    let sample_len: libc::c_int;
    unsafe {
        execve(
            command,
            args as *const *const libc::c_char,
            env as *const *const libc::c_char,
        );
        i = *c___errno_location();
        CHECK_TERMSIG!();

        if i != ENOEXEC!() {
            last_command_exit_value = if i == ENOENT!() as libc::c_int {
                EX_NOTFOUND as libc::c_int
            } else {
                EX_NOEXEC as libc::c_int
            };
            if file_isdir(command) != 0 {
                internal_error(
                    b"%s: %s\0" as *const u8 as *mut libc::c_char,
                    command as *mut libc::c_char,
                    strerror(EISDIR!()),
                );
            } else if executable_file(command) == 0 {
                *c___errno_location() = i;
                file_error(command);
            } else if i == E2BIG!() || i == ENOMEM!() {
                *c___errno_location() = i;
                file_error(command);
            } else {
                let fd_0: libc::c_int = open(command, O_RDONLY as libc::c_int);

                if fd_0 >= 0 {
                    read(
                        fd_0,
                        sample.as_mut_ptr() as *mut libc::c_void,
                        ::std::mem::size_of::<[libc::c_char; 128]>() as usize,
                    ) as libc::c_int;
                } else {
                    // sample_len = -1;
                }

                READ_SAMPLE_BUF!(command, sample, sample_len);
                if sample_len > 0 {
                    sample[(sample_len - 1) as usize] = '\u{0}' as i32 as libc::c_char;
                }
                if sample_len > 2
                    && sample[0] as libc::c_int == '#' as i32
                    && sample[1] as libc::c_int == '!' as i32
                {
                    let mut interp: *mut libc::c_char;
                    let ilen: libc::c_int;

                    close(fd_0);
                    interp = getinterp(sample.as_mut_ptr(), sample_len, 0 as *mut libc::c_int);
                    ilen = strlen(interp) as libc::c_int;
                    *c___errno_location() = i;
                    if *interp.offset((ilen - 1) as isize) as libc::c_int == '\r' as i32 {
                        interp = realloc(interp as *mut c_void, (ilen + 2) as usize)
                            as *mut libc::c_char;
                        *interp.offset((ilen - 1 as libc::c_int) as isize) =
                            '^' as i32 as libc::c_char;
                        *interp.offset(ilen as isize) = 'M' as i32 as libc::c_char;
                        *interp.offset((ilen + 1 as libc::c_int) as isize) =
                            '\u{0}' as i32 as libc::c_char;
                    }
                    sys_error(
                        b"%s: %s: bad interpreter\0" as *const u8 as *mut libc::c_char,
                        command,
                        if !interp.is_null() {
                            interp
                        } else {
                            b"\0" as *const u8 as *const libc::c_char
                        },
                    );
                    FREE!(interp);
                    return EX_NOEXEC as libc::c_int;
                }

                if fd_0 >= 0 {
                    close(fd_0);
                }
                *c___errno_location() = i;
                file_error(command);
            }
            return last_command_exit_value;
        }

        READ_SAMPLE_BUF!(command, sample, sample_len);

        if sample_len == 0 {
            return EXECUTION_SUCCESS as libc::c_int;
        }

        if sample_len > 0 {
            if check_binary_file(sample.as_mut_ptr(), sample_len) != 0 {
                internal_error(
                    b"%s: cannot execute binary file: %s\0" as *const u8 as *mut libc::c_char,
                    command,
                    strerror(i),
                );
                *c___errno_location() = i;
                return EX_BINARY_FILE as libc::c_int;
            }
        }

        reset_parser();
        initialize_subshell();

        set_sigint_handler();

        larray = c_strvec_len(args) + 1;
        args = c_strvec_resize(args, larray + 1);

        i = larray - 1;
        while i != 0 {
            *args.offset(i as isize) = *args.offset((i - 1) as isize);
            i -= 1;
        }

        *args.offset(0 as isize) = shell_name;
        *args.offset(1 as isize) = command;
        *args.offset(larray as isize) = 0 as *mut libc::c_char;

        if *(*args.offset(0 as isize)).offset(0 as isize) as libc::c_int == '-' as i32 {
            *args.offset(0 as isize) = *args.offset(1);
        }
        if restricted != 0 {
            change_flag('r' as i32, '+' as i32);
        }

        if !subshell_argv.is_null() {
            i = 1;
            while i < subshell_argc {
                free(subshell_argv.offset(i as isize) as *mut c_void);
                i += 1;
            }
            free(subshell_argv as *mut c_void);
        }
        dispose_command(currently_executing_command);
        currently_executing_command = 0 as *mut libc::c_void as *mut COMMAND;

        subshell_argc = larray;
        subshell_argv = args;
        subshell_envp = env;

        unbind_args();

        clear_fifo_list();

        c_siglongjmp(subshell_top_level.as_mut_ptr(), 1);
    }
}

fn execute_intern_function(name: *mut WordDesc, funcdef: *mut FUNCTION_DEF) -> libc::c_int {
    let var: *mut SHELL_VAR;
    let t: *mut libc::c_char;
    unsafe {
        if check_identifier(name, posixly_correct) == 0 {
            if posixly_correct != 0 && interactive_shell == 0 {
                last_command_exit_value = EX_BADUSAGE as libc::c_int;
                jump_to_top_level(ERREXIT as libc::c_int);
            }
            return EXECUTION_FAILURE as libc::c_int;
        }

        if !(strchr((*name).word, CTLESC!())).is_null() {
            t = dequote_escapes((*name).word);
            free((*name).word as *mut c_void);
            (*name).word = t;
        }

        if posixly_correct != 0 && (find_special_builtin((*name).word)).is_some() {
            internal_error(
                b"`%s': is a special builtin\0" as *const u8 as *mut libc::c_char,
                (*name).word,
            );
            last_command_exit_value = EX_BADUSAGE as libc::c_int;
            jump_to_top_level(if interactive_shell != 0 {
                DISCARD as libc::c_int
            } else {
                ERREXIT as libc::c_int
            });
        }

        var = find_function((*name).word);
        if !var.is_null() && (readonly_p!(var) != 0 || noassign_p!(var) != 0) {
            if readonly_p!(var) != 0 {
                internal_error(
                    b"%s: readonly function\0" as *const u8 as *mut libc::c_char,
                    (*var).name,
                );
            }
            return EXECUTION_FAILURE as libc::c_int;
        }
        bind_function_def((*name).word, funcdef, 1);
        bind_function((*name).word, (*funcdef).command);
    }
    return EXECUTION_SUCCESS as libc::c_int;
}

fn close_pipes(in_0: libc::c_int, out: libc::c_int) {
    unsafe {
        if in_0 >= 0 {
            close(in_0);
        }
        if out >= 0 {
            close(out);
        }
    }
}

fn dup_error(oldd: libc::c_int, newd: libc::c_int) {
    unsafe {
        sys_error(
            b"cannot duplicate fd %d to fd %d\0" as *const u8 as *mut libc::c_char,
            oldd,
            newd,
        );
    }
}

fn do_piping(pipe_in: libc::c_int, pipe_out: libc::c_int) {
    if pipe_in != NO_PIPE {
        if unsafe { dup2(pipe_in, 0) < 0 } {
            dup_error(pipe_in, 0);
        }
        if pipe_in > 0 {
            unsafe {
                close(pipe_in);
            }
        }
    }
    if pipe_out != NO_PIPE {
        if pipe_out != REDIRECT_BOTH as libc::c_int {
            if unsafe { dup2(pipe_out, 1) < 0 } {
                dup_error(pipe_out, 1);
            }
            if pipe_out == 0 || pipe_out > 1 {
                unsafe {
                    close(pipe_out);
                }
            }
        } else if unsafe { dup2(1, 2) < 0 } {
            dup_error(1, 2);
        }
    }
}
