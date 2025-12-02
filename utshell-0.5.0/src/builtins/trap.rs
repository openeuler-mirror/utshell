/*
 * SPDX-FileCopyrightText: 2025 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: GPL-2.0-or-later
 */
use super::help::builtin_help;
use crate::builtins::bashgetopt::{internal_getopt, reset_internal_getopt};
use crate::builtins::common::{builtin_usage, display_signal_list, sh_chkwrite, sh_invalidsig};
use crate::builtins::evalfile::sourcelevel;
use crate::general::all_digits;
use crate::sig::{
    initialize_terminating_signals, set_signal_handler, sigint_sighandler, termsig_sighandler,
};
use crate::src_common::*;
use crate::trap::{
    decode_signal, free_trap_strings, get_all_original_signals, ignore_signal,
    restore_default_signal, set_signal, signal_is_hard_ignored, signal_name,
};

extern "C" {
    pub static parse_and_execute_level: libc::c_int;
}

#[no_mangle]
pub fn trap_builtin(mut list: *mut WordList) -> i32 {
    let mut list_signal_names: libc::c_int = 0;
    let mut display: libc::c_int = 0;
    let mut result: libc::c_int = EXECUTION_SUCCESS;

    reset_internal_getopt();
    let opt_str = CString::new("lp").unwrap();
    let mut opt = internal_getopt(list, opt_str.as_ptr() as *mut libc::c_char);
    while opt != -1 {
        let opt_char: char = char::from(opt as u8);
        match opt_char {
            'l' => list_signal_names += 1,
            'p' => display += 1,
            _ => {
                if opt == -99 {
                    builtin_help();
                    return EX_USAGE;
                }
                builtin_usage();
                return EX_USAGE;
            }
        }
        opt = internal_getopt(list, opt_str.as_ptr() as *mut libc::c_char);
    }
    unsafe {
        list = loptend;
    }

    opt = DSIG_NOCASE | DSIG_SIGPREFIX;
    unsafe {
        if list_signal_names != 0 {
            return sh_chkwrite(display_signal_list(PT_NULL as *mut WordList, 1));
        } else if display != 0 || list.is_null() {
            initialize_terminating_signals();
            get_all_original_signals();
            return sh_chkwrite(display_traps(
                list,
                (display != 0 && posixly_correct != 0) as libc::c_int,
            ));
        } else {
            let mut operation = SET;
            let first_arg = (*(*list).word).word;
            let first_signal = !first_arg.is_null()
                && *first_arg != 0
                && all_digits(first_arg) != 0
                && decode_signal(first_arg, opt) != NO_SIG;
            if first_signal {
                operation = REVERT;
            } else if posixly_correct == 0
                && !first_arg.is_null()
                && *first_arg != 0
                && (*first_arg != b'-' as libc::c_char
                    || *((first_arg as usize + 1) as *mut libc::c_char) != 0)
                && decode_signal(first_arg, opt) != NO_SIG
                && (*list).next.is_null()
            {
                operation = REVERT;
            } else {
                list = (*list).next;
                if list.is_null() {
                    builtin_usage();
                    return EX_USAGE;
                } else if *first_arg == b'\0' as libc::c_char {
                    operation = IGNORE;
                } else if *first_arg == b'-' as libc::c_char
                    && *((first_arg as usize + 1) as *mut libc::c_char) == 0
                {
                    operation = REVERT;
                }
            }

            if subshell_environment & SUBSHELL_RESETTRAP as i32 != 0 {
                free_trap_strings();
                subshell_environment &= !(SUBSHELL_RESETTRAP as i32);
            }

            let mut sig: libc::c_int;
            while !list.is_null() {
                sig = decode_signal((*(*list).word).word, opt);
                if sig == NO_SIG {
                    sh_invalidsig((*(*list).word).word);
                    result = EXECUTION_FAILURE;
                } else {
                    match operation {
                        SET => set_signal(sig, first_arg),
                        IGNORE => ignore_signal(sig),
                        REVERT => {
                            restore_default_signal(sig);
                            match sig {
                                libc::SIGINT => {
                                    if interactive != 0 {
                                        set_signal_handler(
                                            libc::SIGINT,
                                            //sigint_sighandler as *mut SigHandler,
                                            Some(sigint_sighandler as fn(libc::c_int) -> ()),
                                        );
                                    } else if interactive_shell != 0
                                        && (sourcelevel != 0
                                            || running_trap != 0
                                            || parse_and_execute_level != 0)
                                    {
                                        set_signal_handler(
                                            libc::SIGINT,
                                            //sigint_sighandler as *mut SigHandler,
                                            Some(sigint_sighandler as fn(libc::c_int) -> ()),
                                        );
                                    } else {
                                        set_signal_handler(
                                            libc::SIGINT,
                                            //termsig_sighandler as *mut SigHandler,
                                            Some(termsig_sighandler as fn(libc::c_int) -> ()),
                                        );
                                    }
                                }
                                libc::SIGQUIT => {
                                    set_signal_handler(libc::SIGQUIT, std::mem::transmute(1_usize));
                                }
                                libc::SIGTERM | libc::SIGTTIN | libc::SIGTTOU | libc::SIGTSTP => {
                                    if interactive != 0 {
                                        set_signal_handler(sig, std::mem::transmute(1_usize));
                                    }
                                }
                                _ => (),
                            }
                            break;
                        }
                        _ => (),
                    }
                }

                list = (*list).next;
            }
        }
    }
    return result;
}

fn showtrap(i: libc::c_int, show_default: libc::c_int) {
    let t: *mut libc::c_char;
    let p = unsafe { trap_list[i as usize] };
    if (p == libc::SIG_DFL as *mut libc::c_char) && unsafe { signal_is_hard_ignored(i) } == 0 {
        if show_default != 0 {
            t = "-\0".as_ptr() as *mut libc::c_char;
        } else {
            return;
        }
    } else if unsafe { signal_is_hard_ignored(i) } != 0 {
        t = PT_NULL as *mut libc::c_char;
    } else {
        t = if p == libc::SIG_IGN as *mut libc::c_char {
            PT_NULL as *mut libc::c_char
        } else {
            unsafe { sh_single_quote(p) }
        }
    }
    unsafe {
        let sn = signal_name(i);
        if libc::strncmp(sn, "SIGJUNK\0".as_ptr() as *const libc::c_char, 7) == 0
            || libc::strncmp(sn, "unknown\0".as_ptr() as *const libc::c_char, 7) == 0
        {
            libc::printf(
                "trap -- %s %d\n\0".as_ptr() as *const libc::c_char,
                if t.is_null() {
                    "''\0".as_ptr() as *mut libc::c_char
                } else {
                    t
                },
                i,
            );
        } else if posixly_correct != 0 {
            if libc::strncmp(sn, "SIG\0".as_ptr() as *const libc::c_char, 3) == 0 {
                libc::printf(
                    "trap -- %s %s\n\0".as_ptr() as *const libc::c_char,
                    if t.is_null() {
                        "''\0".as_ptr() as *mut libc::c_char
                    } else {
                        t
                    },
                    (sn as usize + 3) as *mut libc::c_char,
                );
            } else {
                libc::printf(
                    "trap -- %s %s\n\0".as_ptr() as *const libc::c_char,
                    if t.is_null() {
                        "''\0".as_ptr() as *mut libc::c_char
                    } else {
                        t
                    },
                    sn,
                );
            }
        } else {
            libc::printf(
                "trap -- %s %s\n\0".as_ptr() as *const libc::c_char,
                if t.is_null() {
                    "''\0".as_ptr() as *mut libc::c_char
                } else {
                    t
                },
                sn,
            );
        }

        if show_default == 0 {
            if !t.is_null() {
                libc::free(t as *mut c_void);
            }
        }
    }
}

fn display_traps(mut list: *mut WordList, show_all: libc::c_int) -> libc::c_int {
    if list.is_null() {
        for i in 0..BASH_NSIG {
            showtrap(i as i32, show_all);
        }
        return EXECUTION_SUCCESS;
    }

    let mut result = EXECUTION_SUCCESS;
    let mut i: libc::c_int;
    while !list.is_null() {
        unsafe {
            i = decode_signal((*(*list).word).word, DSIG_NOCASE | DSIG_SIGPREFIX);
            if i == NO_SIG {
                sh_invalidsig((*(*list).word).word);
                result = EXECUTION_FAILURE;
            } else {
                showtrap(i, show_all);
            }

            list = (*list).next;
        }
    }

    return result;
}
