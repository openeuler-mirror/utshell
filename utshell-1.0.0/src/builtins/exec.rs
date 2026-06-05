//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

use crate::bashhist::maybe_save_shell_history;
use crate::builtins::{
    bashgetopt::{internal_getopt, reset_internal_getopt},
    common::{builtin_usage, sh_notfound, sh_restricted},
    help::builtin_help,
};
use crate::dispose_cmd::dispose_redirects;
use crate::error::file_error;
use crate::execute_cmd::shell_execve;
use crate::findcmd::{executable_file, search_for_command};
use crate::general::{absolute_program, file_isdir, full_pathname};
use crate::input::sync_buffered_stream;
use crate::jobs::{default_tty_job_signals, end_job_control, restart_job_control};
use crate::sig::initialize_signals;
use crate::src_common::*;
use crate::trap::{initialize_traps, restore_original_signals};
use crate::utshell::exit_shell;
use crate::variables::{adjust_shell_level, maybe_make_export_env};

/* If the user wants this to look like a login shell, then
prepend a `-' onto NAME and return the new name. */
#[no_mangle]
fn mkdashname(name: *mut libc::c_char) -> *mut libc::c_char {
    let ret: *mut libc::c_char;

    unsafe {
        ret = malloc(2 + strlen(name) as usize) as *mut libc::c_char;
        *ret.offset(0) = '-' as libc::c_char;
        strcpy(ret.offset(1), name);
        return ret;
    }
}

#[no_mangle]
pub fn exec_builtin(mut list: *mut WordList) -> i32 {
    let mut exit_value;
    let mut cleanenv: i32 = 0;
    let mut login: i32 = 0;
    let mut opt: i32;
    let mut orig_job_control: i32 = 0;
    let mut argv0: *mut libc::c_char = std::ptr::null_mut() as *mut libc::c_char;
    let mut command: *mut libc::c_char;
    let mut args: *mut *mut libc::c_char;
    let mut env: *mut *mut libc::c_char;
    let newname: *mut libc::c_char;
    let com2: *mut libc::c_char;

    unsafe {
        exec_argv0 = std::ptr::null_mut() as *mut libc::c_char;
    }

    reset_internal_getopt();

    loop {
        let c_str = CString::new("cla:").unwrap();
        opt = internal_getopt(list, c_str.as_ptr() as *mut libc::c_char);
        while opt != -1 {
            let optu8 = opt as u8;
            let opt_char = char::from(optu8);
            match opt_char {
                'c' => cleanenv = 1,
                'l' => login = 1,
                'a' => argv0 = unsafe { list_optarg },
                _ => {
                    if opt == -99 {
                        builtin_help();
                        return EX_USAGE;
                    }
                    builtin_usage();
                    return EX_USAGE;
                }
            }

            opt = internal_getopt(list, c_str.as_ptr() as *mut libc::c_char);
        }

        list = unsafe { loptend };

        /* First, let the redirections remain. */
        unsafe {
            dispose_redirects(redirection_undo_list);
            redirection_undo_list = std::ptr::null_mut() as *mut REDIRECT;
        }

        if list.is_null() {
            return EXECUTION_SUCCESS!();
        }

        if unsafe { restricted != 0 } {
            //限制性shell
            sh_restricted(std::ptr::null_mut() as *mut libc::c_char);
            return EXECUTION_FAILURE!();
        }

        args = c_strvec_from_word_list(list, 1, 0, 0 as *mut libc::c_int);
        env = 0 as *mut *mut libc::c_char;

        /* A command with a slash anywhere in its name is not looked up in $PATH. */
        if unsafe { absolute_program(*args.offset(0)) != 0 } {
            //命令给的绝对路径，或者执行脚本
            command = unsafe { (*args).offset(0) };
        } else {
            //exec后直接给命令
            command = unsafe { search_for_command(*args.offset(0), 1) };
        }

        if command.is_null() {
            if unsafe { file_isdir(*args.offset(0)) != 0 } {
                let c_str = CString::new("%s: cannot execute: %s").unwrap();
                let c_ptr = c_str.as_ptr();
                unsafe {
                    builtin_error(c_ptr, *args.offset(0), strerror(*c___errno_location()));
                }
                exit_value = EX_NOEXEC;
            } else {
                unsafe {
                    sh_notfound(*args.offset(0));
                }
                exit_value = EX_NOTFOUND;
            }
            //goto failed_exec;
            break;
        }

        com2 = full_pathname(command);
        if !com2.is_null() {
            if unsafe { command != *args.offset(0) } {
                unsafe {
                    free(command as *mut c_void);
                }
            }
            command = com2;
        }

        if !argv0.is_null() {
            unsafe {
                //exec有-a参数
                free(*args.offset(0) as *mut c_void);
                if login != 0 {
                    *args.offset(0) = mkdashname(argv0);
                } else {
                    *args.offset(0) = savestring!(argv0);
                }
                exec_argv0 = savestring!(*args.offset(0));
            }
        } else if login != 0 {
            unsafe {
                newname = mkdashname(*args.offset(0));
                free(*args.offset(0) as *mut c_void);
                *args.offset(0) = newname;
            }
        }

        /* Decrement SHLVL by 1 so a new shell started here has the same value,
        preserving the appearance.  After we do that, we need to change the
        exported environment to include the new value.  If we've already forked
        and are in a subshell, we don't want to decrement the shell level,
        since we are `increasing' the level */

        if unsafe { cleanenv == 0 && (subshell_environment & SUBSHELL_PAREN!() == 0) } {
            adjust_shell_level(-1);
        }

        if cleanenv != 0 {
            env = c_strvec_create(1);
            unsafe {
                *env.offset(0) = 0 as *mut libc::c_char;
            }
        } else {
            maybe_make_export_env();
            unsafe {
                env = export_env;
            }
        }

        if unsafe { interactive_shell != 0 && subshell_environment == 0 } {
            maybe_save_shell_history();
        }

        restore_original_signals();

        orig_job_control = unsafe { job_control };
        if unsafe { subshell_environment == 0 } {
            end_job_control();
        }
        if unsafe { interactive != 0 || job_control != 0 } {
            default_tty_job_signals();
        }

        if unsafe { default_buffered_input >= 0 } {
            unsafe {
                sync_buffered_stream(default_buffered_input);
            }
        }

        exit_value = shell_execve(command, args, env);

        /* We have to set this to NULL because shell_execve has called realloc()
        to stuff more items at the front of the array, which may have caused
        the memory to be freed by realloc().  We don't want to free it twice. */

        args = std::ptr::null_mut() as *mut *mut libc::c_char;

        if cleanenv == 0 {
            adjust_shell_level(1);
        }

        if exit_value == EX_NOTFOUND {
            //goto failed_exec;
            break;
        } else if executable_file(command) == 0 {
            let c_str = CString::new("%s: cannot execute: %s").unwrap();
            let c_ptr = c_str.as_ptr();
            unsafe {
                builtin_error(c_ptr, command, strerror(*c___errno_location()));
            }
            exit_value = EX_NOEXEC;
        } else {
            file_error(command);
        }

        //跳出loop循环，只执行一次loop
        break;
    }

    //fialed_exec
    unsafe {
        FREE!(command as *mut c_void);
    }

    if unsafe { subshell_environment != 0 || interactive == 0 && no_exit_on_failed_exec == 0 } {
        exit_shell(exit_value);
    }

    if !args.is_null() {
        c_strvec_dispose(args);
    }

    if unsafe { !env.is_null() && env != export_env } {
        c_strvec_dispose(env);
    }

    initialize_traps();
    initialize_signals(1);

    if orig_job_control != 0 {
        restart_job_control();
    }

    return exit_value;
}
