/*
 * SPDX-FileCopyrightText: 2025 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: GPL-2.0-or-later
 */
use crate::array::array_to_argv;
use crate::builtins::common::get_working_directory;
use crate::builtins::exit::bash_logout;
use crate::dispose_cmd::dispose_command;
use crate::error::command_error;
use crate::execute_cmd::execute_command;
use crate::print_cmd::make_command_string;
use crate::sig::restore_sigmask;
use crate::sig::{jump_to_top_level, sigint_sighandler};
use crate::src_common::*;
use crate::subst::unlink_fifo_list;
use crate::trap::{run_pending_traps, signal_is_ignored, signal_is_trapped};
use crate::variables::{dispose_used_env_vars, find_variable, get_string_value};
use crate::y_tab::{
    bash_input, current_command_line_count, decode_prompt_string, execute_variable_command,
    gather_here_documents, need_here_doc, parser_expanding_alias, parser_will_prompt, ps0_prompt,
    reset_readahead_token, set_current_prompt_level, yyparse, EOF_Reached,
};
#[no_mangle]
pub fn reader_loop() -> libc::c_int {
    let mut flag: bool = false;
    let mut our_indirection_level: libc::c_int = 0;
    let mut current_command: *mut COMMAND = 0 as *mut COMMAND;

    current_command = 0 as *mut c_void as *mut COMMAND;
    unsafe {
        indirection_level += 1;
        our_indirection_level = indirection_level;

        if just_one_command != 0 {
            reset_readahead_token();
        }
        while EOF_Reached == 0 {
            let mut code: libc::c_int = 0;

            code = setjmp_nosigs!(top_level.as_mut_ptr());

            unlink_fifo_list();

            if interactive_shell != 0
                && signal_is_ignored(SIGINT as libc::c_int) == 0
                && signal_is_trapped(SIGINT as libc::c_int) == 0
            {
                set_signal_handler(
                    SIGINT as libc::c_int,
                    sigint_sighandler as *mut SigHandler, //可能存在问题
                );
            }

            loop {
                if code != NOT_JUMPED as libc::c_int {
                    indirection_level = our_indirection_level;

                    match code as u32 {
                        FORCE_EOF!() | ERREXIT!() | EXITPROG!() => {
                            current_command = 0 as *mut c_void as *mut COMMAND;

                            if exit_immediately_on_error != 0 {
                                variable_context = 0;
                            }
                            EOF_Reached = EOF as i32;
                            flag = true;
                            break;
                        }
                        DISCARD!() => {
                            if last_command_exit_value == 0 {
                                set_exit_status(EXECUTION_FAILURE as libc::c_int);
                            }
                            if subshell_environment != 0 {
                                current_command = 0 as *mut c_void as *mut COMMAND;
                                EOF_Reached = EOF as i32;
                                flag = true;
                                break;
                            }

                            if !current_command.is_null() {
                                dispose_command(current_command);
                                current_command = 0 as *mut c_void as *mut COMMAND;
                            }
                            restore_sigmask();
                        }
                        _ => {
                            command_error(
                                b"reader_loop\0" as *const u8 as *const libc::c_char,
                                3 as libc::c_int,
                                code,
                                0 as libc::c_int,
                            );
                        }
                    }
                }

                executing = 0;
                if !temporary_env.is_null() {
                    dispose_used_env_vars();
                }

                if read_command() == 0 {
                    if interactive_shell == 0 && (read_but_dont_execute != 0 && rpm_requires == 0) {
                        set_exit_status(EXECUTION_SUCCESS as i32);
                        dispose_command(global_command);
                        global_command = 0 as *mut COMMAND;
                    } else {
                        current_command = global_command;
                        if !current_command.is_null() {
                            global_command = 0 as *mut COMMAND;

                            if interactive != 0 && !ps0_prompt.is_null() {
                                let mut ps0_string: *mut libc::c_char = 0 as *mut libc::c_char;

                                ps0_string = decode_prompt_string(ps0_prompt);
                                if !ps0_string.is_null() && *ps0_string as libc::c_int != 0 {
                                    fprintf(
                                        stderr,
                                        b"%s\0" as *const u8 as *const libc::c_char,
                                        ps0_string,
                                    );
                                    fflush(stderr);
                                }

                                free(ps0_string as *mut c_void);
                            }
                            current_command_number += 1;

                            executing = 1;
                            stdin_redir = 0;

                            execute_command(current_command);
                            flag = true;
                            break;
                        }
                    }
                } else {
                    if interactive == 0 {
                        EOF_Reached = EOF;
                    }
                }
                break; //跳出loop循环
            }

            if flag == true {
                flag = false;
                QUIT!();

                if !current_command.is_null() {
                    dispose_command(current_command);
                    current_command = 0 as *mut COMMAND;
                }
            }

            if just_one_command != 0 {
                EOF_Reached = EOF;
            }
        }

        indirection_level -= 1;
        return last_command_exit_value;
    }
}

#[no_mangle]
pub fn pretty_print_loop() -> libc::c_int {
    let mut current_command: *mut COMMAND = 0 as *mut COMMAND;
    let mut command_to_print: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut code: libc::c_int = 0;
    let mut global_posix_mode: libc::c_int = 0;
    let mut last_was_newline: libc::c_int = 0;
    unsafe {
        global_posix_mode = posixly_correct;
        last_was_newline = 0;
        while EOF_Reached == 0 {
            code = setjmp_nosigs!(top_level.as_mut_ptr());
            if code != 0 {
                return EXECUTION_FAILURE as libc::c_int;
            }
            if read_command() == 0 {
                current_command = global_command;
                global_command = 0 as *mut COMMAND;
                posixly_correct = 1;
                if !current_command.is_null() && {
                    command_to_print = make_command_string(current_command);
                    !command_to_print.is_null()
                } {
                    printf(
                        b"%s\n\0" as *const u8 as *const libc::c_char,
                        command_to_print,
                    );
                    last_was_newline = 0;
                } else if last_was_newline == 0 {
                    printf(b"\n\0" as *const u8 as *const libc::c_char);
                    last_was_newline = 1;
                }
                posixly_correct = global_posix_mode;
                dispose_command(current_command);
            } else {
                return 1;
            }
        }
    }
    return 0;
}

fn alrm_catcher(i: libc::c_int) {
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    msg = b"\x07timed out waiting for input: auto-logout\n\0" as *const u8 as *mut libc::c_char;
    unsafe {
        write(1 as libc::c_int, msg as *const c_void, strlen(msg) as usize);
        bash_logout();
        jump_to_top_level(EXITPROG as libc::c_int);
    }
}
fn send_pwd_to_eterm() {
    let mut pwd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        f = 0 as *mut libc::c_char;
        pwd = get_string_value(b"PWD\0" as *const u8 as *const libc::c_char);
        if pwd.is_null() {
            pwd = get_working_directory(
                b"eterm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            f = pwd;
        }

        fprintf(
            stderr,
            b"\x1A/%s\n\0" as *const u8 as *const libc::c_char,
            pwd,
        );
        free(f as *mut c_void);
    }
}

#[no_mangle]
pub fn execute_array_command(a: *mut ARRAY, v: *mut c_void) -> libc::c_int {
    let mut tag: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut argc: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    unsafe {
        tag = v as *mut libc::c_char;
        argc = 0;
        argv = array_to_argv(a, &mut argc);
        i = 0;
        while i < argc {
            if !(*argv.offset(i as isize)).is_null()
                && *(*argv.offset(i as isize)).offset(0) as libc::c_int != 0
            {
                execute_variable_command(*argv.offset(i as isize), tag);
            }
            i += 1;
        }
        strvec_dispose(argv);
    }
    return 0;
}

fn execute_prompt_command() {
    let mut command_to_execute: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pcv: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut pcmds: *mut ARRAY = 0 as *mut ARRAY;

    pcv = find_variable(b"PROMPT_COMMAND\0" as *const u8 as *const libc::c_char);
    unsafe {
        if pcv.is_null() || var_isset!(pcv) as libc::c_int == 0 || invisible_p!(pcv) != 0 {
            return;
        }
        if array_p!(pcv) != 0 {
            pcmds = array_cell!(pcv);
            if !pcmds.is_null() && array_num_elements!(pcmds) > 0 {
                execute_array_command(
                    pcmds,
                    b"PROMPT_COMMAND\0" as *const u8 as *const libc::c_char as *mut c_void,
                );
            }
            return;
        } else {
            if assoc_p!(pcv) != 0 {
                return;
            }
        }
        command_to_execute = value_cell!(pcv);
        if !command_to_execute.is_null() && *command_to_execute as libc::c_int != 0 {
            execute_variable_command(
                command_to_execute,
                b"PROMPT_COMMAND\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
    }
}

#[no_mangle]
pub fn parse_command() -> libc::c_int {
    let mut r: libc::c_int = 0;
    unsafe {
        need_here_doc = 0;
        run_pending_traps();

        if interactive != 0
            && bash_input.type_0 as libc::c_uint != st_string
            && parser_expanding_alias() == 0
        {
            if no_line_editing != 0
                || bash_input.type_0 as libc::c_uint == st_stdin && parser_will_prompt() != 0
            {
                execute_prompt_command();
            }
            if running_under_emacs == 2 {
                send_pwd_to_eterm();
            }
        }
        current_command_line_count = 0;
        r = yyparse();
        if need_here_doc != 0 {
            gather_here_documents();
        }
    }
    return r;
}
#[no_mangle]
pub fn read_command() -> libc::c_int {
    let mut tmout_var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut tmout_len: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    let mut old_alrm: *mut SigHandler = 0 as *mut SigHandler;

    set_current_prompt_level(1);
    unsafe {
        global_command = 0 as *mut COMMAND;
        tmout_var = 0 as *mut SHELL_VAR;
        tmout_len = 0;
        old_alrm = 0 as *mut SigHandler;

        if interactive != 0 {
            tmout_var = find_variable(b"TMOUT\0" as *const u8 as *const libc::c_char);

            if !tmout_var.is_null() && !((*tmout_var).value).is_null() {
                tmout_len = atoi((*tmout_var).value);
                if tmout_len > 0 {
                    old_alrm =
                        set_signal_handler(SIGALRM as libc::c_int, alrm_catcher as *mut SigHandler);
                    alarm(tmout_len as libc::c_uint);
                }
            }
        }

        QUIT!();

        current_command_line_count = 0;
        result = parse_command();
        if interactive != 0 && !tmout_var.is_null() && tmout_len > 0 {
            alarm(0);
            set_signal_handler(SIGALRM as libc::c_int, old_alrm);
        }
    }
    return result;
}
