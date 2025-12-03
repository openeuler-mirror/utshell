/*
 * SPDX-FileCopyrightText: 2025 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: GPL-2.0-or-later
 */
use libc::{close, free, malloc, open, strcmp, strcpy, strlen};

use crate::bashhist::bash_history_disable;
use crate::builtins::{eval::eval_builtin, source::source_builtin};
use crate::dispose_cmd::dispose_command;
use crate::error::{command_error, file_error};
use crate::eval::parse_command;
use crate::execute_cmd::{dispose_fd_bitmap, execute_command_internal, new_fd_bitmap};
use crate::jobs::unfreeze_jobs_list;
use crate::readline::siglongjmp;
use crate::redir::{redirection_error, redirection_expand};
use crate::sig::{jump_to_top_level, throw_to_top_level, top_level_cleanup};
use crate::src_common::*;
use crate::trap::{any_signals_trapped, run_trap_cleanup, signal_is_trapped};
use crate::unwind_prot::{
    add_unwind_protect, begin_unwind_frame, discard_unwind_frame, have_unwind_protects,
    run_unwind_frame, unwind_protect_mem,
};
use crate::variables::set_pipestatus_from_exit;
use crate::y_tab::{
    bash_input, clear_shell_input_line, current_token, get_current_prompt_level, line_number,
    parser_expanding_alias, parser_remaining_input, parser_restore_alias, parser_save_alias,
    pop_stream, push_stream, reset_parser, set_current_prompt_level, shell_eof_token,
    with_input_from_string,
};

extern "C" {
    fn sigprocmask(
        __how: ::std::os::raw::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> ::std::os::raw::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> ::std::os::raw::c_int;
}

#[no_mangle]
pub static mut parse_and_execute_level: libc::c_int = 0 as libc::c_int;
fn set_history_remembering() {
    unsafe {
        remember_on_history = enable_history_list;
    }
}
fn restore_lastcom(mut x: *mut libc::c_char) {
    unsafe {
        if !the_printed_command_except_trap.is_null() {
            free(the_printed_command_except_trap as *mut libc::c_void);
        }
        the_printed_command_except_trap = x;
    }
}
#[no_mangle]
pub fn should_suppress_fork(mut command: *mut COMMAND) -> libc::c_int {
    unsafe {
        return (startup_state == 2 as libc::c_int
            && parse_and_execute_level == 1 as libc::c_int
            && running_trap == 0 as libc::c_int
            && *bash_input.location.string as libc::c_int == '\u{0}' as i32
            && parser_expanding_alias() == 0 as libc::c_int
            && (*command).type_0 as libc::c_uint == cm_simple as libc::c_int as libc::c_uint
            && signal_is_trapped(0 as libc::c_int) == 0 as libc::c_int
            && signal_is_trapped(64 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int)
                == 0 as libc::c_int
            && any_signals_trapped() < 0 as libc::c_int
            && ((*command).redirects).is_null()
            && ((*(*command).value.Simple).redirects).is_null()
            && (*command).flags & CMD_TIME_PIPELINE!() as libc::c_int == 0 as libc::c_int
            && (*command).flags & CMD_INVERT_RETURN!() as libc::c_int == 0 as libc::c_int)
            as libc::c_int;
    }
}
#[no_mangle]
pub fn can_optimize_connection(mut command: *mut COMMAND) -> libc::c_int {
    unsafe {
        return (*bash_input.location.string as libc::c_int == '\u{0}' as i32
            && parser_expanding_alias() == 0 as libc::c_int
            && ((*(*command).value.Connection).connector == AND_AND!() as libc::c_int
                || (*(*command).value.Connection).connector == OR_OR!() as libc::c_int
                || (*(*command).value.Connection).connector == ';' as i32)
            && (*(*(*command).value.Connection).second).type_0 as libc::c_uint
                == cm_simple as libc::c_int as libc::c_uint) as libc::c_int;
    }
}
#[no_mangle]
pub fn optimize_fork(mut command: *mut COMMAND) {
    unsafe {
        if (*command).type_0 as libc::c_uint == cm_connection as libc::c_int as libc::c_uint
            && ((*(*command).value.Connection).connector == AND_AND!() as libc::c_int
                || (*(*command).value.Connection).connector == OR_OR!() as libc::c_int
                || (*(*command).value.Connection).connector == ';' as i32)
            && (*(*(*command).value.Connection).second).flags & CMD_TRY_OPTIMIZING!() as libc::c_int
                != 0
            && should_suppress_fork((*(*command).value.Connection).second) != 0
        {
            (*(*(*command).value.Connection).second).flags |= CMD_NO_FORK!() as libc::c_int;
            (*(*(*(*command).value.Connection).second).value.Simple).flags |=
                CMD_NO_FORK!() as libc::c_int;
        }
    }
}
#[no_mangle]
pub fn optimize_subshell_command(mut command: *mut COMMAND) {
    unsafe {
        if running_trap == 0 as libc::c_int
            && (*command).type_0 as libc::c_uint == cm_simple as libc::c_int as libc::c_uint
            && signal_is_trapped(0 as libc::c_int) == 0 as libc::c_int
            && signal_is_trapped(64 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int)
                == 0 as libc::c_int
            && any_signals_trapped() < 0 as libc::c_int
            && ((*command).redirects).is_null()
            && ((*(*command).value.Simple).redirects).is_null()
            && (*command).flags & CMD_TIME_PIPELINE!() as libc::c_int == 0 as libc::c_int
            && (*command).flags & CMD_INVERT_RETURN!() as libc::c_int == 0 as libc::c_int
        {
            (*command).flags |= CMD_NO_FORK!() as libc::c_int;
            (*(*command).value.Simple).flags |= CMD_NO_FORK!() as libc::c_int;
        } else if (*command).type_0 as libc::c_uint == cm_connection as libc::c_int as libc::c_uint
            && ((*(*command).value.Connection).connector == AND_AND!() as libc::c_int
                || (*(*command).value.Connection).connector == OR_OR!() as libc::c_int)
        {
            optimize_subshell_command((*(*command).value.Connection).second);
        }
    }
}
#[no_mangle]
pub fn optimize_shell_function(mut command: *mut COMMAND) {
    let mut fc: *mut COMMAND = 0 as *mut COMMAND;
    unsafe {
        fc = if (*command).type_0 as libc::c_uint == cm_group as libc::c_int as libc::c_uint {
            (*(*command).value.Group).command
        } else {
            command
        };
        if (*fc).type_0 as libc::c_uint == cm_simple as libc::c_int as libc::c_uint
            && should_suppress_fork(fc) != 0
        {
            (*fc).flags |= CMD_NO_FORK!() as libc::c_int;
            (*(*fc).value.Simple).flags |= CMD_NO_FORK!() as libc::c_int;
        } else if (*fc).type_0 as libc::c_uint == cm_connection as libc::c_int as libc::c_uint
            && can_optimize_connection(fc) != 0
            && should_suppress_fork((*(*fc).value.Connection).second) != 0
        {
            (*(*(*fc).value.Connection).second).flags |= CMD_NO_FORK!() as libc::c_int;
            (*(*(*(*fc).value.Connection).second).value.Simple).flags |=
                CMD_NO_FORK!() as libc::c_int;
        }
    }
}
#[no_mangle]
pub fn parse_and_execute_cleanup(mut old_running_trap: libc::c_int) {
    unsafe {
        if running_trap > 0 as libc::c_int {
            if running_trap != old_running_trap {
                run_trap_cleanup(running_trap - 1 as libc::c_int);
            }
            unfreeze_jobs_list();
        }
        if have_unwind_protects() != 0 {
            run_unwind_frame(
                b"parse_and_execute top\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else {
            parse_and_execute_level = 0 as libc::c_int;
        };
    }
}
fn parse_prologue(
    mut string: *mut libc::c_char,
    mut flags: libc::c_int,
    mut tag: *mut libc::c_char,
) {
    let mut orig_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lastcom: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x: libc::c_int = 0;
    orig_string = string;
    unsafe {
        begin_unwind_frame(tag);
        unwind_protect_mem(
            &mut parse_and_execute_level as *mut libc::c_int as *mut libc::c_char,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        );
        unwind_protect_mem(
            &mut top_level as *mut sigjmp_buf as *mut libc::c_char,
            ::std::mem::size_of::<sigjmp_buf>() as libc::c_ulong as libc::c_int,
        );
        unwind_protect_mem(
            &mut indirection_level as *mut libc::c_int as *mut libc::c_char,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        );
        unwind_protect_mem(
            &mut line_number as *mut libc::c_int as *mut libc::c_char,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        );
        unwind_protect_mem(
            &mut line_number_for_err_trap as *mut libc::c_int as *mut libc::c_char,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        );
        unwind_protect_mem(
            &mut loop_level as *mut libc::c_int as *mut libc::c_char,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        );
        unwind_protect_mem(
            &mut executing_list as *mut libc::c_int as *mut libc::c_char,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        );
        unwind_protect_mem(
            &mut comsub_ignore_return as *mut libc::c_int as *mut libc::c_char,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        );
        if flags & (SEVAL_NONINT!() as libc::c_int | SEVAL_INTERACT!() as libc::c_int) != 0 {
            unwind_protect_mem(
                &mut interactive as *mut libc::c_int as *mut libc::c_char,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
            );
        }
        if parse_and_execute_level == 0 as libc::c_int {
            // let shr: Functions = Functions {
            //     set_history_remembering,
            // };
            // add_unwind_protect(set_history_remembering, 0 as *mut libc::c_void as *mut libc::c_char);
            add_unwind_protect(
                std::mem::transmute::<fn(), Option<Function>>(set_history_remembering),
                0 as *mut libc::c_void as *mut libc::c_char,
            );
        } else {
            unwind_protect_mem(
                &mut remember_on_history as *mut libc::c_int as *mut libc::c_char,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
            );
        }
        unwind_protect_mem(
            &mut history_expansion_inhibited as *mut libc::c_int as *mut libc::c_char,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        );
        if interactive_shell != 0 {
            x = get_current_prompt_level();
            // let scpl: Functions = Functions {
            //     set_current_prompt_level,
            // };
            // add_unwind_protect(scpl, x);
            add_unwind_protect(
                std::mem::transmute::<fn(x: libc::c_int), Option<Function>>(
                    set_current_prompt_level,
                ),
                x as *mut libc::c_char,
            );
        }
        if !the_printed_command_except_trap.is_null() {
            lastcom = strcpy(
                malloc(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(strlen(the_printed_command_except_trap) as u64)
                        as usize,
                ) as *mut libc::c_char,
                the_printed_command_except_trap,
            );
            // let rl: Functions = Functions { restore_lastcom };
            // add_unwind_protect(rl, lastcom);
            add_unwind_protect(
                std::mem::transmute::<fn(x: *mut libc::c_char), Option<Function>>(restore_lastcom),
                lastcom,
            );
        }
        // let ps: Functions = Functions { pop_stream };
        // add_unwind_protect(ps, 0 as *mut libc::c_void as *mut libc::c_char);
        add_unwind_protect(
            std::mem::transmute::<fn(), Option<Function>>(pop_stream),
            0 as *mut libc::c_void as *mut libc::c_char,
        );
        if parser_expanding_alias() != 0 {
            // let pra: Functions = Functions {
            //     parser_restore_alias,
            // };
            // add_unwind_protect(pra, 0 as *mut libc::c_void as *mut libc::c_char);
            add_unwind_protect(
                std::mem::transmute::<fn(), Option<Function>>(parser_restore_alias),
                0 as *mut libc::c_void as *mut libc::c_char,
            );
        }
        if !orig_string.is_null() && flags & CMD_IGNORE_RETURN!() as libc::c_int == 0 as libc::c_int
        {
            // let xf: Functions = Functions { free };
            // add_unwind_protect(xf, orig_string);
            add_unwind_protect(
                std::mem::transmute::<unsafe extern "C" fn(_), Option<Function>>(free),
                orig_string,
            );
        }
        if flags & (SEVAL_NONINT!() as libc::c_int | SEVAL_INTERACT!() as libc::c_int) != 0 {
            interactive = if flags & SEVAL_NONINT!() as libc::c_int != 0 {
                0 as libc::c_int
            } else {
                1 as libc::c_int
            };
        }
        if flags & CMD_INVERT_RETURN!() as libc::c_int != 0 {
            bash_history_disable();
        }
        if flags & SEVAL_NOHISTEXP!() as libc::c_int != 0 {
            history_expansion_inhibited = 1 as libc::c_int;
        }
    }
}
#[no_mangle]
pub fn parse_and_execute(
    mut string: *mut libc::c_char,
    mut from_file: *const libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut code: libc::c_int = 0;
    let mut lreset: libc::c_int = 0;
    let mut should_jump_to_top_level: libc::c_int = 0;
    let mut last_result: libc::c_int = 0;
    let mut command: *mut COMMAND = 0 as *mut COMMAND;
    let mut pe_sigmask: sigset_t = __sigset_t { __val: [0; 16] };
    unsafe {
        parse_prologue(
            string,
            flags,
            b"parse_and_execute top\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        parse_and_execute_level += 1;
        lreset = flags & SEVAL_RESETLINE!() as libc::c_int;
        sigemptyset(&mut pe_sigmask as *mut sigset_t as *mut sigset_t);
        sigprocmask(
            0 as libc::c_int,
            0 as *mut libc::c_void as *mut sigset_t,
            &mut pe_sigmask as *mut sigset_t as *mut sigset_t,
        );
        push_stream(lreset);
        if parser_expanding_alias() != 0 {
            parser_save_alias();
        }
        if lreset == 0 as libc::c_int {
            line_number -= 1;
        }
        indirection_level += 1;
        ::std::ptr::write_volatile(
            &mut should_jump_to_top_level as *mut libc::c_int,
            0 as libc::c_int,
        );
        code = ::std::ptr::read_volatile::<libc::c_int>(
            &should_jump_to_top_level as *const libc::c_int,
        );
        ::std::ptr::write_volatile(&mut last_result as *mut libc::c_int, 0 as libc::c_int);
        if current_token == 304 as libc::c_int {
            current_token = '\n' as i32;
        }
        with_input_from_string(string, from_file);
        clear_shell_input_line();
        while *bash_input.location.string as libc::c_int != 0 || parser_expanding_alias() != 0 {
            ::std::ptr::write_volatile(
                &mut command as *mut *mut COMMAND,
                0 as *mut libc::c_void as *mut COMMAND,
            );
            if interrupt_state != 0 {
                ::std::ptr::write_volatile(&mut last_result as *mut libc::c_int, 1 as libc::c_int);
                break;
            } else {
                code = __sigsetjmp(top_level.as_mut_ptr(), 0 as libc::c_int);
                if code != 0 {
                    ::std::ptr::write_volatile(
                        &mut should_jump_to_top_level as *mut libc::c_int,
                        0 as libc::c_int,
                    );
                    match code {
                        4 => {
                            if exit_immediately_on_error != 0 && variable_context != 0 {
                                discard_unwind_frame(
                                    b"pe_dispose\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                );
                                variable_context = 0 as libc::c_int;
                            }
                            ::std::ptr::write_volatile(
                                &mut should_jump_to_top_level as *mut libc::c_int,
                                1 as libc::c_int,
                            );
                            run_unwind_frame(
                                b"parse_and_execute top\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                            if interrupt_state != 0 && parse_and_execute_level == 0 as libc::c_int {
                                interactive = interactive_shell;
                                throw_to_top_level();
                            }
                            if should_jump_to_top_level != 0 {
                                jump_to_top_level(code);
                            }
                            return last_result;
                        }
                        1 | 3 => {
                            if !command.is_null() {
                                run_unwind_frame(
                                    b"pe_dispose\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                );
                            }
                            ::std::ptr::write_volatile(
                                &mut should_jump_to_top_level as *mut libc::c_int,
                                1 as libc::c_int,
                            );
                            run_unwind_frame(
                                b"parse_and_execute top\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                            if interrupt_state != 0 && parse_and_execute_level == 0 as libc::c_int {
                                interactive = interactive_shell;
                                throw_to_top_level();
                            }
                            if should_jump_to_top_level != 0 {
                                jump_to_top_level(code);
                            }
                            return last_result;
                        }
                        2 => {
                            if !command.is_null() {
                                run_unwind_frame(
                                    b"pe_dispose\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                );
                            }
                            ::std::ptr::write_volatile(
                                &mut last_command_exit_value as *mut libc::c_int,
                                1 as libc::c_int,
                            );
                            ::std::ptr::write_volatile(
                                &mut last_result as *mut libc::c_int,
                                ::std::ptr::read_volatile::<libc::c_int>(
                                    &last_command_exit_value as *const libc::c_int,
                                ),
                            );
                            set_pipestatus_from_exit(last_command_exit_value);
                            if subshell_environment != 0 {
                                ::std::ptr::write_volatile(
                                    &mut should_jump_to_top_level as *mut libc::c_int,
                                    1 as libc::c_int,
                                );
                                run_unwind_frame(
                                    b"parse_and_execute top\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                );
                                if interrupt_state != 0
                                    && parse_and_execute_level == 0 as libc::c_int
                                {
                                    interactive = interactive_shell;
                                    throw_to_top_level();
                                }
                                if should_jump_to_top_level != 0 {
                                    jump_to_top_level(code);
                                }
                                return last_result;
                            } else {
                                sigprocmask(
                                    2 as libc::c_int,
                                    &mut pe_sigmask as *mut sigset_t as *mut sigset_t,
                                    0 as *mut libc::c_void as *mut sigset_t,
                                );
                                continue;
                            }
                        }
                        _ => {
                            command_error(
                                b"parse_and_execute\0" as *const u8 as *const libc::c_char,
                                3 as libc::c_int,
                                code,
                                0 as libc::c_int,
                            );
                        }
                    }
                }
                if parse_command() == 0 as libc::c_int {
                    if flags & SEVAL_PARSEONLY!() as libc::c_int != 0
                        || interactive_shell == 0 as libc::c_int && read_but_dont_execute != 0
                    {
                        ::std::ptr::write_volatile(
                            &mut last_result as *mut libc::c_int,
                            0 as libc::c_int,
                        );
                        dispose_command(global_command);
                        global_command = 0 as *mut libc::c_void as *mut COMMAND;
                    } else {
                        ::std::ptr::write_volatile(
                            &mut command as *mut *mut COMMAND,
                            global_command,
                        );
                        if (::std::ptr::read_volatile::<*mut COMMAND>(
                            &command as *const *mut COMMAND,
                        ))
                        .is_null()
                        {
                            continue;
                        }
                        let mut bitmap: *mut fd_bitmap = 0 as *mut fd_bitmap;
                        if flags & CMD_TIME_PIPELINE!() as libc::c_int != 0 {
                            let mut x: *mut libc::c_char = 0 as *mut libc::c_char;
                            if (*command).type_0 as libc::c_uint
                                != cm_function_def as libc::c_int as libc::c_uint
                                || {
                                    x = parser_remaining_input();
                                    !x.is_null() && *x as libc::c_int != 0
                                }
                                || (*from_file.offset(0 as libc::c_int as isize) as libc::c_int
                                    == *((*(*(*command).value.Function_def).name).word)
                                        .offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                    && strcmp(
                                        from_file,
                                        (*(*(*command).value.Function_def).name).word,
                                    ) == 0 as libc::c_int)
                                    as libc::c_int
                                    == 0 as libc::c_int
                            {
                                internal_warning(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"%s: ignoring function definition attempt\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    from_file,
                                );
                                ::std::ptr::write_volatile(
                                    &mut should_jump_to_top_level as *mut libc::c_int,
                                    0 as libc::c_int,
                                );
                                ::std::ptr::write_volatile(
                                    &mut last_command_exit_value as *mut libc::c_int,
                                    2 as libc::c_int,
                                );
                                ::std::ptr::write_volatile(
                                    &mut last_result as *mut libc::c_int,
                                    ::std::ptr::read_volatile::<libc::c_int>(
                                        &last_command_exit_value as *const libc::c_int,
                                    ),
                                );
                                set_pipestatus_from_exit(last_command_exit_value);
                                reset_parser();
                                break;
                            }
                        }
                        bitmap = new_fd_bitmap(32 as libc::c_int);
                        begin_unwind_frame(
                            b"pe_dispose\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        // let dfb: Functions = Functions { dispose_fd_bitmap };
                        // add_unwind_protect(dfb, bitmap);
                        // let dc: Functions = Functions { dispose_command };
                        // add_unwind_protect(dc, command);
                        add_unwind_protect(
                            std::mem::transmute::<fn(*mut fd_bitmap), Option<Function>>(
                                dispose_fd_bitmap,
                            ),
                            bitmap as *mut libc::c_char,
                        );
                        add_unwind_protect(
                            std::mem::transmute::<fn(*mut command), Option<Function>>(
                                dispose_command,
                            ),
                            command as *mut libc::c_char,
                        );
                        global_command = 0 as *mut libc::c_void as *mut COMMAND;
                        if subshell_environment & CMD_INVERT_RETURN!() as libc::c_int != 0
                            && comsub_ignore_return != 0
                        {
                            (*command).flags |= CMD_IGNORE_RETURN!() as libc::c_int;
                        }
                        if should_suppress_fork(command) != 0 {
                            (*command).flags |= CMD_NO_FORK!() as libc::c_int;
                            (*(*command).value.Simple).flags |= CMD_NO_FORK!() as libc::c_int;
                        } else if (*command).type_0 as libc::c_uint
                            == cm_connection as libc::c_int as libc::c_uint
                            && can_optimize_connection(command) != 0
                        {
                            (*(*(*command).value.Connection).second).flags |=
                                CMD_TRY_OPTIMIZING!() as libc::c_int;
                            (*(*(*(*command).value.Connection).second).value.Simple).flags |=
                                CMD_TRY_OPTIMIZING!() as libc::c_int;
                        }
                        if startup_state == 2 as libc::c_int
                            && subshell_environment & CMD_INVERT_RETURN!() as libc::c_int != 0
                            && *bash_input.location.string as libc::c_int == '\u{0}' as i32
                            && (*command).type_0 as libc::c_uint
                                == cm_simple as libc::c_int as libc::c_uint
                            && ((*command).redirects).is_null()
                            && (*command).flags & CMD_TIME_PIPELINE!() as libc::c_int
                                == 0 as libc::c_int
                            && ((*(*command).value.Simple).words).is_null()
                            && !((*(*command).value.Simple).redirects).is_null()
                            && ((*(*(*command).value.Simple).redirects).next).is_null()
                            && (*(*(*command).value.Simple).redirects).instruction as libc::c_uint
                                == r_input_direction as libc::c_int as libc::c_uint
                            && (*(*(*command).value.Simple).redirects).redirector.dest
                                == 0 as libc::c_int
                        {
                            let mut r: libc::c_int = 0;
                            r = cat_file((*(*command).value.Simple).redirects);
                            ::std::ptr::write_volatile(
                                &mut last_result as *mut libc::c_int,
                                if r < 0 as libc::c_int {
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                },
                            );
                        } else {
                            ::std::ptr::write_volatile(
                                //问题
                                &mut last_result as *mut libc::c_int,
                                execute_command_internal(
                                    command,
                                    0 as libc::c_int,
                                    -(1 as libc::c_int),
                                    -(1 as libc::c_int),
                                    bitmap,
                                ),
                            );
                        }
                        dispose_command(command);
                        dispose_fd_bitmap(bitmap);
                        discard_unwind_frame(
                            b"pe_dispose\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        if !(flags & SEVAL_ONECMD!() as libc::c_int != 0) {
                            continue;
                        }
                        reset_parser();
                        break;
                    }
                } else {
                    ::std::ptr::write_volatile(
                        &mut last_result as *mut libc::c_int,
                        2 as libc::c_int,
                    );
                    if interactive_shell == 0 as libc::c_int
                        && this_shell_builtin.is_some()
                        && (this_shell_builtin == Some(source_builtin)
                            || this_shell_builtin == Some(eval_builtin))
                        && last_command_exit_value == 257 as libc::c_int
                        && posixly_correct != 0
                        && executing_command_builtin == 0 as libc::c_int
                    {
                        ::std::ptr::write_volatile(
                            &mut should_jump_to_top_level as *mut libc::c_int,
                            1 as libc::c_int,
                        );
                        code = 4 as libc::c_int;
                        ::std::ptr::write_volatile(
                            &mut last_command_exit_value as *mut libc::c_int,
                            2 as libc::c_int,
                        );
                    }
                    break;
                }
            }
        }
        run_unwind_frame(
            b"parse_and_execute top\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if interrupt_state != 0 && parse_and_execute_level == 0 as libc::c_int {
            interactive = interactive_shell;
            throw_to_top_level();
        }
        if should_jump_to_top_level != 0 {
            jump_to_top_level(code);
        }
        return last_result;
    }
}
#[no_mangle]
pub fn parse_string(
    mut string: *mut libc::c_char,
    mut from_file: *const libc::c_char,
    mut flags: libc::c_int,
    mut endp: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut code: libc::c_int = 0;
    let mut nc: libc::c_int = 0;
    let mut should_jump_to_top_level: libc::c_int = 0;
    let mut command: *mut COMMAND = 0 as *mut COMMAND;
    let mut oglobal: *mut COMMAND = 0 as *mut COMMAND;
    let mut ostring: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ps_sigmask: sigset_t = __sigset_t { __val: [0; 16] };
    unsafe {
        parse_prologue(
            string,
            flags,
            b"parse_string top\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        sigemptyset(&mut ps_sigmask as *mut sigset_t as *mut sigset_t);
        sigprocmask(
            0 as libc::c_int,
            0 as *mut libc::c_void as *mut sigset_t,
            &mut ps_sigmask as *mut sigset_t as *mut sigset_t,
        );
        push_stream(0 as libc::c_int);
        if parser_expanding_alias() != 0 {
            parser_save_alias();
        }
        ::std::ptr::write_volatile(
            &mut should_jump_to_top_level as *mut libc::c_int,
            0 as libc::c_int,
        );
        code = ::std::ptr::read_volatile::<libc::c_int>(
            &should_jump_to_top_level as *const libc::c_int,
        );
        oglobal = global_command;
        ostring = string;
        with_input_from_string(string, from_file);
        while *bash_input.location.string != 0 {
            ::std::ptr::write_volatile(
                &mut command as *mut *mut COMMAND,
                0 as *mut libc::c_void as *mut COMMAND,
            );
            code = __sigsetjmp(top_level.as_mut_ptr(), 0 as libc::c_int);
            if code != 0 {
                ::std::ptr::write_volatile(
                    &mut should_jump_to_top_level as *mut libc::c_int,
                    0 as libc::c_int,
                );
                match code {
                    1 | 4 | 3 | 2 => {
                        if !command.is_null() {
                            dispose_command(command);
                        }
                        ::std::ptr::write_volatile(
                            &mut should_jump_to_top_level as *mut libc::c_int,
                            1 as libc::c_int,
                        );
                        global_command = oglobal;
                        nc = (bash_input.location.string).offset_from(ostring) as libc::c_long
                            as libc::c_int;
                        if !endp.is_null() {
                            *endp = bash_input.location.string;
                        }
                        run_unwind_frame(
                            b"parse_string top\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        if should_jump_to_top_level != 0 {
                            if parse_and_execute_level == 0 as libc::c_int {
                                top_level_cleanup();
                            }
                            if code == 2 as libc::c_int {
                                return -(2 as libc::c_int);
                            }
                            jump_to_top_level(code);
                        }
                        return nc;
                    }
                    _ => {
                        sigprocmask(
                            2 as libc::c_int,
                            &mut ps_sigmask as *mut sigset_t as *mut sigset_t,
                            0 as *mut libc::c_void as *mut sigset_t,
                        );
                        command_error(
                            b"parse_string\0" as *const u8 as *const libc::c_char,
                            3 as libc::c_int,
                            code,
                            0 as libc::c_int,
                        );
                    }
                }
            }
            if parse_command() == 0 as libc::c_int {
                dispose_command(global_command);
                global_command = 0 as *mut libc::c_void as *mut COMMAND;
                if current_token == 304 as libc::c_int || current_token == shell_eof_token {
                    break;
                }
            } else {
                if flags & CMD_NO_FORK!() as libc::c_int == 0 as libc::c_int {
                    ::std::ptr::write_volatile(
                        &mut should_jump_to_top_level as *mut libc::c_int,
                        1 as libc::c_int,
                    );
                    code = 2 as libc::c_int;
                } else {
                    reset_parser();
                }
                break;
            }
        }
        global_command = oglobal;
        nc = (bash_input.location.string).offset_from(ostring) as libc::c_long as libc::c_int;
        if !endp.is_null() {
            *endp = bash_input.location.string;
        }
        run_unwind_frame(
            b"parse_string top\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if should_jump_to_top_level != 0 {
            if parse_and_execute_level == 0 as libc::c_int {
                top_level_cleanup();
            }
            if code == 2 as libc::c_int {
                return -(2 as libc::c_int);
            }
            jump_to_top_level(code);
        }
        return nc;
    }
}
fn cat_file(mut r: *mut REDIRECT) -> libc::c_int {
    let mut fn_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fd: libc::c_int = 0;
    let mut rval: libc::c_int = 0;
    unsafe {
        if (*r).instruction as libc::c_uint != r_input_direction as libc::c_int as libc::c_uint {
            return -(1 as libc::c_int);
        }
        if posixly_correct != 0 && interactive_shell == 0 {
            disallow_filename_globbing += 1;
        }
        fn_0 = redirection_expand((*r).redirectee.filename);
        if posixly_correct != 0 && interactive_shell == 0 {
            disallow_filename_globbing -= 1;
        }
        if fn_0.is_null() {
            redirection_error(r, -(1 as libc::c_int), fn_0);
            return -(1 as libc::c_int);
        }
        fd = open(fn_0, 0 as libc::c_int);
        if fd < 0 as libc::c_int {
            file_error(fn_0);
            free(fn_0 as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
        rval = zcatfd(fd, 1 as libc::c_int, fn_0);
        free(fn_0 as *mut libc::c_void);
        close(fd);
        return rval;
    }
}
#[no_mangle]
pub fn evalstring(
    mut string: *mut libc::c_char,
    mut from_file: *const libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut rflag: libc::c_int = 0;
    let mut rcatch: libc::c_int = 0;
    let mut was_trap: libc::c_int = 0;
    unsafe {
        ::std::ptr::write_volatile(&mut was_trap as *mut libc::c_int, running_trap);
        ::std::ptr::write_volatile(&mut rcatch as *mut libc::c_int, 0 as libc::c_int);
        ::std::ptr::write_volatile(&mut rflag as *mut libc::c_int, return_catch_flag);
        if rflag != 0 {
            begin_unwind_frame(
                b"evalstring\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            unwind_protect_mem(
                &mut return_catch_flag as *mut libc::c_int as *mut libc::c_char,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
            );
            unwind_protect_mem(
                &mut return_catch as *mut sigjmp_buf as *mut libc::c_char,
                ::std::mem::size_of::<sigjmp_buf>() as libc::c_ulong as libc::c_int,
            );
            return_catch_flag += 1;
            ::std::ptr::write_volatile(
                &mut rcatch as *mut libc::c_int,
                __sigsetjmp(return_catch.as_mut_ptr(), 0 as libc::c_int),
            );
        }
        if rcatch != 0 {
            parse_and_execute_cleanup(was_trap);
            ::std::ptr::write_volatile(&mut r as *mut libc::c_int, return_catch_value);
        } else {
            ::std::ptr::write_volatile(
                &mut r as *mut libc::c_int,
                parse_and_execute(string, from_file, flags),
            );
        }
        if rflag != 0 {
            run_unwind_frame(
                b"evalstring\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            if rcatch != 0 && return_catch_flag != 0 {
                return_catch_value = r;
                siglongjmp(return_catch.as_mut_ptr(), 1 as libc::c_int);
            }
        }
        return r;
    }
}
