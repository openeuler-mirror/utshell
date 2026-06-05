use super::help::builtin_help;
use crate::builtins::bashgetopt::{internal_getopt, reset_internal_getopt};
use crate::builtins::common::{builtin_usage, no_args, sh_nojobs};
use crate::sig::set_signal_handler;
use crate::src_common::*;

#[no_mangle]
pub fn suspend_builtin(mut list: *mut WordList) -> i32 {
    let mut opt: libc::c_int;
    let mut force: libc::c_int = 0;

    reset_internal_getopt();
    let opt_str = "f\0".as_ptr() as *mut libc::c_char;
    opt = internal_getopt(list, opt_str);
    while opt != -1 {
        let opt_char: char = char::from(opt as u8);
        match opt_char {
            'f' => force += 1,
            _ => {
                if opt == -99 {
                    builtin_help();
                    return EX_USAGE;
                }
                builtin_usage();
                return EX_USAGE;
            }
        }

        opt = internal_getopt(list, opt_str);
    }
    unsafe {
        list = loptend;
        if job_control == 0 {
            sh_nojobs("cannot suspend\0".as_ptr() as *mut libc::c_char);
            return EXECUTION_FAILURE;
        }
        if force == 0 {
            no_args(list);
            if login_shell != 0 {
                builtin_error("cannot suspend a login shell\0".as_ptr() as *mut libc::c_char);
                return EXECUTION_FAILURE;
            }
        }

        old_cont = set_signal_handler(
            libc::SIGCONT,
            std::mem::transmute(suspend_continue as usize),
        );
        killpg(shell_pgrp, libc::SIGSTOP);
    }
    return EXECUTION_SUCCESS;
}

fn suspend_continue(_sig: libc::c_int) {
    unsafe {
        set_signal_handler(libc::SIGCONT, old_cont);
    }
}
