/*
 * SPDX-FileCopyrightText: 2025 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: GPL-2.0-or-later
 */
use crate::bashline::bashline_set_event_hook;
use crate::builtins::evalstring::evalstring;
use crate::builtins::wait::wait_builtin;
use crate::general::legal_number;
use crate::jobs::close_pgrp_pipe;
use crate::jobs::get_original_tty_job_signals;
use crate::jobs::give_terminal_to;
use crate::jobs::notify_and_cleanup;
use crate::jobs::restore_pgrp_pipe;
use crate::jobs::restore_pipeline;
use crate::jobs::run_sigchld_trap;
use crate::jobs::save_pgrp_pipe;
use crate::jobs::save_pipeline;
use crate::jobs::stop_making_children;
use crate::readline::array_dispose;
use crate::sig::initialize_terminating_signals;
use crate::sig::jump_to_top_level;
use crate::sig::set_signal_handler;
use crate::sig::sigint_sighandler;
use crate::sig::termsig_sighandler;
use crate::src_common::*;
use crate::variables::restore_pipestatus_array;
use crate::variables::save_pipestatus_array;
use crate::y_tab::reset_parser;
use crate::y_tab::restore_parser_state;
use crate::y_tab::save_parser_state;

static mut sigmodes: [libc::c_int; 68] = [0; 68];
static mut catch_flag: libc::c_int = 0;

#[no_mangle]
pub fn initialize_traps() {
    unsafe {
        trap_list[EXIT_TRAP] = 0 as *mut libc::c_void as *mut libc::c_char;
        trap_list[DEBUG_TRAP] = 0 as *mut libc::c_void as *mut libc::c_char;
        trap_list[ERROR_TRAP] = 0 as *mut libc::c_void as *mut libc::c_char;
        trap_list[RETURN_TRAP] = 0 as *mut libc::c_void as *mut libc::c_char;

        sigmodes[EXIT_TRAP] = 0 as libc::c_int;
        sigmodes[DEBUG_TRAP] = 0 as libc::c_int;
        sigmodes[ERROR_TRAP] = 0 as libc::c_int;
        sigmodes[RETURN_TRAP] = 0 as libc::c_int;

        original_signals[EXIT_TRAP] = ::core::mem::transmute::<
            Option<fn() -> ()>,
            Option<SigHandler>,
        >(Some(initialize_traps));

        let mut i: usize = 1;
        while i < NSIG {
            pending_traps[i] = 0 as libc::c_int;
            trap_list[i] = ::core::mem::transmute::<usize, *mut libc::c_char>(libc::SIG_DFL);
            sigmodes[i] = SIG_INHERITED as libc::c_int;
            original_signals[i] = ::core::mem::transmute::<Option<fn() -> ()>, Option<SigHandler>>(
                Some(initialize_traps),
            );
            i += 1;
        }

        GETORIGSIG!(libc::SIGCHLD);
        sigmodes[libc::SIGCHLD as usize] |= SIG_SPECIAL | SIG_NO_TRAP;

        GETORIGSIG!(libc::SIGINT);
        sigmodes[libc::SIGINT as usize] |= SIG_SPECIAL;

        GETORIGSIG!(libc::SIGQUIT);
        sigmodes[libc::SIGQUIT as usize] |= SIG_SPECIAL;

        if interactive != 0 {
            GETORIGSIG!(libc::SIGTERM);
            sigmodes[libc::SIGTERM as usize] |= SIG_SPECIAL;
        }
        get_original_tty_job_signals();
    }
}

#[no_mangle]
pub fn signal_name(sig: libc::c_int) -> *mut libc::c_char {
    unsafe {
        let ret: *mut libc::c_char = if sig >= BASH_NSIG as libc::c_int
            || sig < 0 as libc::c_int
            || (signal_names[sig as usize]).is_null()
        {
            dcgettext(
                0 as *const libc::c_char,
                b"invalid signal number\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            )
        } else {
            signal_names[sig as usize]
        };
        return ret;
    }
}

#[no_mangle]
pub fn decode_signal(string: *mut libc::c_char, flags: libc::c_int) -> libc::c_int {
    unsafe {
        let mut sig: intmax_t = 0;
        let mut name: *mut libc::c_char;
        if legal_number(string, &mut sig) != 0 {
            return if sig >= 0 as libc::c_int as intmax_t && sig < NSIG as intmax_t {
                sig as libc::c_int
            } else {
                NO_SIG as libc::c_int
            };
        }

        if (libc::strncmp(
            string,
            b"SIGRTMIN+\0" as *const u8 as *const libc::c_char,
            9,
        ) == 0)
            || ((flags & DSIG_NOCASE as libc::c_int) != 0
                && libc::strncasecmp(
                    string,
                    b"SIGRTMIN+\0" as *const u8 as *const libc::c_char,
                    9,
                ) == 0)
        {
            if legal_number(string.offset(9 as libc::c_int as isize), &mut sig) != 0
                && sig >= 0 as libc::c_int as intmax_t
                && sig <= (__libc_current_sigrtmax() - __libc_current_sigrtmin()) as intmax_t
            {
                return (__libc_current_sigrtmin() as intmax_t + sig) as libc::c_int;
            } else {
                return NO_SIG;
            }
        } else if (libc::strncmp(string, b"RTMIN+\0" as *const u8 as *const libc::c_char, 6) == 0)
            || ((flags & DSIG_NOCASE as libc::c_int) != 0
                && libc::strncasecmp(string, b"RTMIN+\0" as *const u8 as *const libc::c_char, 6)
                    == 0)
        {
            if legal_number(string.offset(6 as libc::c_int as isize), &mut sig) != 0
                && sig >= 0 as libc::c_int as intmax_t
                && sig <= (__libc_current_sigrtmax() - __libc_current_sigrtmin()) as intmax_t
            {
                return (__libc_current_sigrtmin() as intmax_t + sig) as libc::c_int;
            } else {
                return NO_SIG;
            }
        }
        sig = 0 as libc::c_int as intmax_t;
        while sig < (BASH_NSIG) as intmax_t {
            name = signal_names[sig as usize];
            if !(name.is_null()
                || *name.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32)
            {
                if strncmp(
                    name,
                    b"SIG\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as usize,
                ) == 0 as libc::c_int
                {
                    name = name.offset(3 as libc::c_int as isize);
                    if flags & DSIG_NOCASE as libc::c_int != 0
                        && strcasecmp(string, name) == 0 as libc::c_int
                    {
                        return sig as libc::c_int;
                    } else if flags & DSIG_NOCASE as libc::c_int == 0 as libc::c_int
                        && strcmp(string, name) == 0 as libc::c_int
                    {
                        return sig as libc::c_int;
                    } else if flags & DSIG_SIGPREFIX as libc::c_int == 0 as libc::c_int {
                        sig += 1;
                        continue;
                    }
                }

                name = signal_names[sig as usize];
                if flags & DSIG_NOCASE as libc::c_int != 0
                    && strcasecmp(string, name) == 0 as libc::c_int
                {
                    return sig as libc::c_int;
                } else if flags & DSIG_NOCASE as libc::c_int == 0 as libc::c_int
                    && strcmp(string, name) == 0 as libc::c_int
                {
                    return sig as libc::c_int;
                }
            }
            sig += 1;
        }
        return NO_SIG;
    }
}
#[no_mangle]
pub fn run_pending_traps() {
    let mut sig: libc::c_int;
    let old_exit_value: libc::c_int;
    let mut x: libc::c_int;
    let old_running: libc::c_int;
    let mut save_subst_varlist: *mut WordList;
    let mut save_tempenv: *mut HASH_TABLE;
    let mut pstate: sh_parser_state_t = _sh_parser_state_t {
        parser_state: 0,
        token_state: 0 as *mut libc::c_int,
        token: 0 as *mut libc::c_char,
        token_buffer_size: 0,
        input_line_terminator: 0,
        eof_encountered: 0,
        prompt_string_pointer: 0 as *mut *mut libc::c_char,
        current_command_line_count: 0,
        remember_on_history: 0,
        history_expansion_inhibited: 0,
        last_command_exit_value: 0,
        pipestatus: 0 as *mut ARRAY,
        last_shell_builtin: None,
        this_shell_builtin: None,
        expand_aliases: 0,
        echo_input_at_read: 0,
        need_here_doc: 0,
        here_doc_first_line: 0,
        redir_stack: [0 as *mut REDIRECT; 16],
    };
    let ps: *mut ARRAY;
    unsafe {
        if catch_flag == 0 as libc::c_int {
            return;
        }
        if running_trap > 0 as libc::c_int {
            if running_trap == libc::SIGWINCH as libc::c_int + 1 as libc::c_int
                && pending_traps[libc::SIGWINCH as usize] != 0
            {
                return;
            }
            if evalnest_max > 0 as libc::c_int && evalnest > evalnest_max {
                internal_error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"trap handler: maximum trap handler level exceeded (%d)\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    evalnest_max,
                );
                evalnest = 0 as libc::c_int;
                jump_to_top_level(DISCARD);
            }
        }
        trapped_signal_received = 0 as libc::c_int;
        catch_flag = 0 as libc::c_int;

        old_exit_value = last_command_exit_value;
        trap_saved_exit_value = last_command_exit_value;
        ps = save_pipestatus_array();
        old_running = running_trap;

        //let mut current_block_56: u64;
        sig = 1 as libc::c_int;
        while sig < NSIG as i32 {
            if pending_traps[sig as usize] != 0 {
                running_trap = sig + 1 as libc::c_int;
                if sig == libc::SIGINT as libc::c_int {
                    pending_traps[sig as usize] = 0 as libc::c_int;
                    run_interrupt_trap(0 as libc::c_int);
                    ::core::ptr::write_volatile(
                        &mut interrupt_state as *mut sig_atomic_t,
                        0 as libc::c_int,
                    );
                    //current_block_56 = 1345366029464561491;
                } else if sig == libc::SIGCHLD as libc::c_int
                    && trap_list[libc::SIGCHLD as libc::c_int as usize]
                        != ::core::mem::transmute::<fn() -> (), *mut libc::c_char>(initialize_traps)
                    && sigmodes[libc::SIGCHLD as libc::c_int as usize]
                        & SIG_INPROGRESS as libc::c_int
                        == 0 as libc::c_int
                {
                    sigmodes[libc::SIGCHLD as libc::c_int as usize] |=
                        SIG_INPROGRESS as libc::c_int;
                    evalnest += 1;
                    x = pending_traps[sig as usize];
                    pending_traps[sig as usize] = 0 as libc::c_int;
                    run_sigchld_trap(x);
                    running_trap = 0 as libc::c_int;
                    evalnest -= 1;
                    sigmodes[libc::SIGCHLD as libc::c_int as usize] &= !(SIG_INPROGRESS);
                    sig += 1;
                    /* continue here rather than reset pending_traps[SIGCHLD] below in
                    case there are recursive calls to run_pending_traps and children
                    have been reaped while run_sigchld_trap was running. */
                    continue; //current_block_56 = 10599921512955367680;
                } else if sig == libc::SIGCHLD as libc::c_int
                    && trap_list[libc::SIGCHLD as libc::c_int as usize]
                        == ::core::mem::transmute::<fn() -> (), *mut libc::c_char>(initialize_traps)
                    && sigmodes[libc::SIGCHLD as usize] & SIG_INPROGRESS as libc::c_int
                        != 0 as libc::c_int
                {
                    running_trap = 0 as libc::c_int;
                    sig += 1;
                    continue; // current_block_56 = 10599921512955367680;
                } else if sig == libc::SIGCHLD as libc::c_int
                    && sigmodes[libc::SIGCHLD as usize] & SIG_INPROGRESS as libc::c_int != 0
                //else if (sig == libc::SIGCHLD && (sigmodes[libc::SIGCHLD] & SIG_INPROGRESS))
                {
                    running_trap = 0 as libc::c_int;
                    sig += 1;
                    continue; //current_block_56 = 10599921512955367680;
                } else {
                    if (trap_list[sig as usize]).is_null()
                        || trap_list[sig as usize]
                            == ::core::mem::transmute::<__sighandler_t, *mut libc::c_char>(
                                ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(
                                    1 as libc::c_int as libc::intptr_t,
                                ),
                            )
                        || trap_list[sig as usize]
                            == ::core::mem::transmute::<fn() -> (), *mut libc::c_char>(
                                initialize_traps,
                            )
                    {
                        internal_warning(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"run_pending_traps: bad value in trap_list[%d]: %p\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            sig,
                            trap_list[sig as usize],
                        );
                        if (trap_list[sig as usize]).is_null() {
                            internal_warning(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"run_pending_traps: signal handler is SIG_DFL, resending %d (%s) to myself\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            sig,
                            signal_name(sig),
                        );
                            kill(getpid(), sig);
                        }
                    } else {
                        save_parser_state(&mut pstate);
                        save_subst_varlist = subst_assign_varlist;
                        subst_assign_varlist = 0 as *mut WordList;
                        save_tempenv = temporary_env;
                        temporary_env = 0 as *mut HASH_TABLE;
                        save_pipeline(1 as libc::c_int);
                        pending_traps[sig as usize] = 0 as libc::c_int;
                        evalnest += 1;
                        evalstring(
                            savestring!(trap_list[sig as usize]),
                            b"trap\0" as *const u8 as *const libc::c_char,
                            0x1 as libc::c_int | 0x4 as libc::c_int | 0x10 as libc::c_int,
                        );
                        evalnest -= 1;
                        restore_pipeline(1 as libc::c_int);
                        subst_assign_varlist = save_subst_varlist;
                        restore_parser_state(&mut pstate);
                        temporary_env = save_tempenv;
                    }
                    //current_block_56 = 1345366029464561491;
                }

                pending_traps[sig as usize] = 0 as libc::c_int;
                running_trap = old_running;
            }
            sig += 1;
        }
        restore_pipestatus_array(ps);
        ::core::ptr::write_volatile(
            &mut last_command_exit_value as *mut libc::c_int,
            old_exit_value,
        );
    }
}

#[no_mangle]
pub fn set_trap_state(sig: libc::c_int) {
    unsafe {
        catch_flag = 1 as libc::c_int;
        pending_traps[sig as usize] += 1;
        trapped_signal_received = sig;
    }
}

#[no_mangle]
pub fn trap_handler(sig: libc::c_int) {
    unsafe {
        let oerrno: libc::c_int;
        if sigmodes[sig as usize] & SIG_TRAPPED == 0 as libc::c_int {
            return;
        }
        if sig >= NSIG as libc::c_int
            || (trap_list[sig as usize])
                == ::core::mem::transmute::<libc::size_t, *mut libc::c_char>(libc::SIG_DFL)
            || trap_list[sig as usize]
                == ::core::mem::transmute::<libc::size_t, *mut libc::c_char>(libc::SIG_IGN)
        {
            programming_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"trap_handler: bad signal %d\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                sig,
            );
        } else {
            oerrno = *__errno_location();
            set_trap_state(sig);
            if this_shell_builtin.is_some() && this_shell_builtin == Some(wait_builtin) {
                wait_signal_received = sig;
                if waiting_for_child != 0 && wait_intr_flag != 0 {
                    siglongjmp(wait_intr_buf.as_mut_ptr(), 1 as libc::c_int);
                }
            }
            if RL_ISSTATE!(RL_STATE_SIGHANDLER) != 0 {
                bashline_set_event_hook();
            }
            *__errno_location() = oerrno;
        };
    }
}

#[no_mangle]
pub fn next_pending_trap(start: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = start;
    unsafe {
        while i < NSIG as libc::c_int {
            if pending_traps[i as usize] != 0 {
                return i;
            }
            i += 1;
        }
    }
    return -(1 as libc::c_int);
}

#[no_mangle]
pub fn first_pending_trap() -> libc::c_int {
    return next_pending_trap(1 as libc::c_int);
}

#[no_mangle]
pub fn any_signals_trapped() -> libc::c_int {
    let mut i: libc::c_int = 1 as libc::c_int;
    unsafe {
        while i < NSIG as libc::c_int + 1 as libc::c_int {
            if sigmodes[i as usize] & SIG_TRAPPED != 0 {
                return i;
            }
            i += 1;
        }
    }
    return -1;
}

#[no_mangle]
pub fn clear_pending_traps() {
    let mut i: libc::c_int = 1 as libc::c_int;
    unsafe {
        while i < NSIG as libc::c_int + 1 as libc::c_int {
            pending_traps[i as usize] = 0 as libc::c_int;
            i += 1;
        }
    }
}

#[no_mangle]
pub fn check_signals() {
    unsafe {
        CHECK_ALRM!();
        QUIT!();
    }
}

#[no_mangle]
pub fn check_signals_and_traps() {
    check_signals();
    run_pending_traps();
}

#[no_mangle]
pub fn maybe_set_sigchld_trap(command_string: *mut libc::c_char) {
    unsafe {
        if sigmodes[libc::SIGCHLD as usize] & SIG_TRAPPED == 0 as libc::c_int
            && trap_list[libc::SIGCHLD as libc::c_int as usize]
                == ::core::mem::transmute::<fn() -> (), *mut libc::c_char>(initialize_traps)
        {
            set_signal(libc::SIGCHLD as libc::c_int, command_string);
        }
    }
}

#[no_mangle]
pub fn set_impossible_sigchld_trap() {
    restore_default_signal(libc::SIGCHLD as libc::c_int);
    unsafe {
        change_signal(
            libc::SIGCHLD as libc::c_int,
            ::core::mem::transmute::<fn() -> (), *mut libc::c_char>(initialize_traps),
        );
        sigmodes[libc::SIGCHLD as libc::c_int as usize] &= !(SIG_TRAPPED);
    }
}

#[no_mangle]
pub fn queue_sigchld_trap(nchild: libc::c_int) {
    if nchild > 0 as libc::c_int {
        unsafe {
            catch_flag = 1 as libc::c_int;
            pending_traps[libc::SIGCHLD as libc::c_int as usize] += nchild;
            trapped_signal_received = libc::SIGCHLD as libc::c_int;
        }
    }
}

#[inline]
fn trap_if_untrapped(sig: libc::c_int, command: *mut libc::c_char) {
    unsafe {
        if sigmodes[sig as usize] & SIG_TRAPPED == 0 as libc::c_int {
            set_signal(sig, command);
        }
    }
}

#[no_mangle]
pub fn set_debug_trap(command: *mut libc::c_char) {
    set_signal(DEBUG_TRAP as libc::c_int, command);
}
#[no_mangle]
pub fn maybe_set_debug_trap(command: *mut libc::c_char) {
    unsafe {
        trap_if_untrapped(DEBUG_TRAP as libc::c_int, command);
    }
}
#[no_mangle]
pub fn set_error_trap(command: *mut libc::c_char) {
    set_signal(ERROR_TRAP as libc::c_int, command);
}
#[no_mangle]
pub fn maybe_set_error_trap(command: *mut libc::c_char) {
    trap_if_untrapped(ERROR_TRAP as libc::c_int, command);
}
#[no_mangle]
pub fn set_return_trap(command: *mut libc::c_char) {
    set_signal(RETURN_TRAP as libc::c_int, command);
}
#[no_mangle]
pub fn maybe_set_return_trap(command: *mut libc::c_char) {
    trap_if_untrapped(RETURN_TRAP as libc::c_int, command);
}
#[no_mangle]
pub fn set_sigint_handler() -> Option<SigHandler> {
    unsafe {
        if sigmodes[libc::SIGINT as usize] & SIG_HARD_IGNORE as libc::c_int != 0 {
            return Some(::core::mem::transmute::<libc::intptr_t, SigHandler>(
                libc::SIG_IGN as libc::intptr_t,
            ));
        } else if sigmodes[libc::SIGINT as usize] & SIG_IGNORED != 0 {
            return set_signal_handler(
                libc::SIGINT as libc::c_int,
                Some(::core::mem::transmute::<libc::intptr_t, SigHandler>(
                    libc::SIG_IGN as libc::intptr_t,
                )),
            );
        } else if sigmodes[libc::SIGINT as libc::c_int as usize] & libc::SIG_IGN as libc::c_int != 0
        {
            return set_signal_handler(libc::SIGINT as libc::c_int, Some(trap_handler));
        } else if interactive != 0 {
            return set_signal_handler(libc::SIGINT as libc::c_int, Some(sigint_sighandler));
        } else {
            return set_signal_handler(libc::SIGINT as libc::c_int, Some(termsig_sighandler));
        };
    }
}

#[no_mangle]
pub fn trap_to_sighandler(sig: libc::c_int) -> Option<SigHandler> {
    unsafe {
        if sigmodes[sig as usize] & (SIG_IGNORED | SIG_HARD_IGNORE as libc::c_int) != 0 {
            return Some(::core::mem::transmute::<libc::intptr_t, SigHandler>(
                libc::SIG_IGN as libc::intptr_t,
            ));
        } else if sigmodes[sig as usize] & SIG_TRAPPED != 0 {
            return Some(trap_handler);
        } else {
            return None;
        };
    }
}

#[no_mangle]
pub fn set_signal(sig: libc::c_int, string: *mut libc::c_char) {
    let mut set: sigset_t = sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = sigset_t { __val: [0; 16] };
    unsafe {
        //#define SPECIAL_TRAP(s) ((s) == EXIT_TRAP || (s) == DEBUG_TRAP || (s) == ERROR_TRAP || (s) == RETURN_TRAP)
        if SPECIAL_TRAP!(sig) {
            change_signal(sig, savestring!(string));
            if sig == EXIT_TRAP as libc::c_int && interactive == 0 as libc::c_int {
                initialize_terminating_signals();
            }
            return;
        }
        if sigmodes[sig as usize] & SIG_HARD_IGNORE as libc::c_int != 0 {
            return;
        }
        if sigmodes[sig as usize] & SIG_TRAPPED == 0 as libc::c_int {
            if original_signals[sig as usize]
                == ::core::mem::transmute::<Option<fn() -> ()>, Option<SigHandler>>(Some(
                    initialize_traps,
                ))
            {
                original_signals[sig as usize] = set_signal_handler(
                    sig,
                    ::core::mem::transmute::<libc::size_t, __sighandler_t>(libc::SIG_DFL),
                );
                set_signal_handler(sig, original_signals[sig as usize]);
                if original_signals[sig as usize]
                    == ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(
                        libc::SIG_IGN as libc::intptr_t,
                    )
                {
                    sigmodes[sig as usize] |= SIG_HARD_IGNORE as libc::c_int;
                }
            }
            if original_signals[sig as usize]
                == ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(
                    libc::SIG_IGN as libc::intptr_t,
                )
            {
                return;
            }
        }
        if sigmodes[sig as usize] & SIG_NO_TRAP as libc::c_int == 0 as libc::c_int {
            sigemptyset(&mut set);
            sigaddset(&mut set, sig);
            sigemptyset(&mut oset);
            sigprocmask(0 as libc::c_int, &mut set, &mut oset);
            change_signal(sig, savestring!(string));
            set_signal_handler(
                sig,
                ::core::mem::transmute::<Option<fn() -> ()>, Option<SigHandler>>(Some(
                    ::core::mem::transmute::<fn(libc::c_int) -> (), fn() -> ()>(trap_handler),
                )),
            );
            sigprocmask(
                2 as libc::c_int,
                &mut oset,
                0 as *mut libc::c_void as *mut sigset_t,
            );
        } else {
            change_signal(sig, savestring!(string));
        };
    }
}
fn free_trap_command(sig: libc::c_int) {
    unsafe {
        if sigmodes[sig as usize] & SIG_TRAPPED != 0
            && !(trap_list[sig as usize]).is_null()
            && trap_list[sig as usize]
                != ::core::mem::transmute::<__sighandler_t, *mut libc::c_char>(
                    ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(
                        libc::SIG_DFL as libc::c_int as libc::intptr_t,
                    ),
                )
            && !(trap_list[sig as usize]).is_null()
            && trap_list[sig as usize]
                != ::core::mem::transmute::<Option<SigHandler>, *mut libc::c_char>(
                    ::core::mem::transmute::<Option<fn() -> ()>, Option<SigHandler>>(Some(
                        initialize_traps,
                    )),
                )
        {
            libc::free(trap_list[sig as usize] as *mut libc::c_void);
        }
    }
}
fn change_signal(sig: libc::c_int, value: *mut libc::c_char) {
    unsafe {
        if sigmodes[sig as usize] & SIG_INPROGRESS as libc::c_int == 0 as libc::c_int {
            free_trap_command(sig);
        }
        trap_list[sig as usize] = value;
        sigmodes[sig as usize] |= SIG_TRAPPED;
        if value
            == ::core::mem::transmute::<__sighandler_t, *mut libc::c_char>(
                ::core::mem::transmute::<libc::size_t, __sighandler_t>(libc::SIG_IGN),
            )
        {
            sigmodes[sig as usize] |= SIG_IGNORED;
        } else {
            sigmodes[sig as usize] &= !(SIG_IGNORED);
        }
        if sigmodes[sig as usize] & SIG_INPROGRESS as libc::c_int != 0 {
            sigmodes[sig as usize] |= SIG_CHANGED as libc::c_int;
        }
    }
}
#[no_mangle]
pub fn get_original_signal(sig: libc::c_int) {
    unsafe {
        if sig > 0 as libc::c_int
            && sig < 64 as libc::c_int + 1 as libc::c_int
            && original_signals[sig as usize]
                == ::core::mem::transmute::<Option<fn() -> ()>, Option<SigHandler>>(Some(
                    initialize_traps,
                ))
        {
            original_signals[sig as usize] = set_signal_handler(sig, None);
            set_signal_handler(sig, original_signals[sig as usize]);
            if original_signals[sig as usize]
                == ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(
                    1 as libc::c_int as libc::intptr_t,
                )
            {
                sigmodes[sig as usize] |= 0x2 as libc::c_int;
            }
        }
    }
}
#[no_mangle]
pub fn get_all_original_signals() {
    let mut i: libc::c_int;
    i = 1 as libc::c_int;
    unsafe {
        while i < 64 as libc::c_int + 1 as libc::c_int {
            if i != 0
                && i < 64 as libc::c_int + 1 as libc::c_int
                && original_signals[i as usize]
                    == ::core::mem::transmute::<Option<fn() -> ()>, Option<SigHandler>>(Some(
                        initialize_traps,
                    ))
            {
                original_signals[i as usize] = set_signal_handler(i, None);
                set_signal_handler(i, original_signals[i as usize]);
                if original_signals[i as usize]
                    == ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(
                        1 as libc::c_int as libc::intptr_t,
                    )
                {
                    sigmodes[i as usize] |= 0x2 as libc::c_int;
                }
            }
            i += 1;
        }
    }
}
#[no_mangle]
pub fn set_original_signal(sig: libc::c_int, handler: Option<SigHandler>) {
    unsafe {
        if sig > 0 as libc::c_int
            && sig < 64 as libc::c_int + 1 as libc::c_int
            && original_signals[sig as usize]
                == ::core::mem::transmute::<Option<fn() -> ()>, Option<SigHandler>>(Some(
                    initialize_traps,
                ))
        {
            original_signals[sig as usize] = handler;
            if original_signals[sig as usize]
                == ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(
                    1 as libc::c_int as libc::intptr_t,
                )
            {
                sigmodes[sig as usize] |= 0x2 as libc::c_int;
            }
        }
    }
}
#[no_mangle]
pub fn restore_default_signal(sig: libc::c_int) {
    unsafe {
        if sig == 0 as libc::c_int
            || sig == 64 as libc::c_int + 1 as libc::c_int
            || sig == 64 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
            || sig == 64 as libc::c_int + 1 as libc::c_int + 2 as libc::c_int
        {
            if sig != 64 as libc::c_int + 1 as libc::c_int
                && sig != 64 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                && sig != 64 as libc::c_int + 1 as libc::c_int + 2 as libc::c_int
                || sigmodes[sig as usize] & 0x10 as libc::c_int == 0 as libc::c_int
            {
                free_trap_command(sig);
            }
            trap_list[sig as usize] = 0 as *mut libc::c_void as *mut libc::c_char;
            sigmodes[sig as usize] &= !(0x1 as libc::c_int);
            if sigmodes[sig as usize] & 0x10 as libc::c_int != 0 {
                sigmodes[sig as usize] |= 0x20 as libc::c_int;
            }
            return;
        }
        if sig != 0
            && sig < 64 as libc::c_int + 1 as libc::c_int
            && original_signals[sig as usize]
                == ::core::mem::transmute::<Option<fn() -> ()>, Option<SigHandler>>(Some(
                    initialize_traps,
                ))
        {
            original_signals[sig as usize] = set_signal_handler(sig, None);
            set_signal_handler(sig, original_signals[sig as usize]);
            if original_signals[sig as usize]
                == ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(
                    1 as libc::c_int as libc::intptr_t,
                )
            {
                sigmodes[sig as usize] |= 0x2 as libc::c_int;
            }
        }
        if sigmodes[sig as usize] & 0x2 as libc::c_int != 0 {
            return;
        }
        if sigmodes[sig as usize] & 0x1 as libc::c_int == 0 as libc::c_int
            && (sig != libc::SIGCHLD as libc::c_int
                || sigmodes[sig as usize] & 0x10 as libc::c_int == 0 as libc::c_int
                || trap_list[sig as usize]
                    != ::core::mem::transmute::<Option<SigHandler>, *mut libc::c_char>(
                        ::core::mem::transmute::<Option<fn() -> ()>, Option<SigHandler>>(Some(
                            initialize_traps,
                        )),
                    ))
        {
            return;
        }
        if sigmodes[sig as usize] & 0x8 as libc::c_int == 0 as libc::c_int {
            set_signal_handler(sig, original_signals[sig as usize]);
        }
        change_signal(
            sig,
            ::core::mem::transmute::<__sighandler_t, *mut libc::c_char>(None),
        );
        sigmodes[sig as usize] &= !(0x1 as libc::c_int);
    }
}
#[no_mangle]
pub fn ignore_signal(sig: libc::c_int) {
    unsafe {
        if (sig == 0 as libc::c_int
            || sig == 64 as libc::c_int + 1 as libc::c_int
            || sig == 64 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
            || sig == 64 as libc::c_int + 1 as libc::c_int + 2 as libc::c_int)
            && sigmodes[sig as usize] & 0x40 as libc::c_int == 0 as libc::c_int
        {
            change_signal(
                sig,
                ::core::mem::transmute::<__sighandler_t, *mut libc::c_char>(
                    ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(
                        1 as libc::c_int as libc::intptr_t,
                    ),
                ),
            );
            return;
        }
        if sig != 0
            && sig < 64 as libc::c_int + 1 as libc::c_int
            && original_signals[sig as usize]
                == ::core::mem::transmute::<Option<fn() -> ()>, Option<SigHandler>>(Some(
                    initialize_traps,
                ))
        {
            original_signals[sig as usize] = set_signal_handler(sig, None);
            set_signal_handler(sig, original_signals[sig as usize]);
            if original_signals[sig as usize]
                == ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(
                    1 as libc::c_int as libc::intptr_t,
                )
            {
                sigmodes[sig as usize] |= 0x2 as libc::c_int;
            }
        }
        if sigmodes[sig as usize] & 0x2 as libc::c_int != 0 {
            return;
        }
        if sigmodes[sig as usize] & 0x40 as libc::c_int != 0 {
            return;
        }
        if sigmodes[sig as usize] & 0x8 as libc::c_int == 0 as libc::c_int {
            set_signal_handler(
                sig,
                ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(
                    1 as libc::c_int as libc::intptr_t,
                ),
            );
        }
        change_signal(
            sig,
            ::core::mem::transmute::<libc::intptr_t, *mut libc::c_char>(
                1 as libc::c_int as libc::intptr_t,
            ),
        );
    }
}
#[no_mangle]
pub fn run_exit_trap() -> libc::c_int {
    let trap_command: *mut libc::c_char;
    let code: libc::c_int;
    let mut function_code: libc::c_int;
    let mut retval: libc::c_int;
    let ps: *mut ARRAY;
    unsafe {
        trap_saved_exit_value = last_command_exit_value;
        ps = save_pipestatus_array();
        function_code = 0 as libc::c_int;
        if sigmodes[EXIT_TRAP] & SIG_TRAPPED != 0
            && sigmodes[EXIT_TRAP] & (SIG_IGNORED | SIG_INPROGRESS as libc::c_int)
                == 0 as libc::c_int
        {
            trap_command = savestring!(trap_list[0 as libc::c_int as usize]);

            sigmodes[EXIT_TRAP] &= !(SIG_TRAPPED);
            sigmodes[EXIT_TRAP] |= SIG_INPROGRESS;
            retval = trap_saved_exit_value;
            running_trap = 1 as libc::c_int;
            code = __sigsetjmp(top_level.as_mut_ptr(), 0 as libc::c_int);
            if return_catch_flag != 0 {
                function_code = __sigsetjmp(return_catch.as_mut_ptr(), 0 as libc::c_int);
            }
            if code == 0 as libc::c_int && function_code == 0 as libc::c_int {
                reset_parser();
                parse_and_execute(
                    trap_command,
                    b"exit trap\0" as *const u8 as *const libc::c_char,
                    0x1 as libc::c_int | 0x4 as libc::c_int | 0x10 as libc::c_int,
                );
            } else if code == ERREXIT as libc::c_int {
                retval = last_command_exit_value;
            } else if code == EXITPROG as libc::c_int {
                retval = last_command_exit_value;
            } else if function_code != 0 as libc::c_int {
                retval = return_catch_value;
            } else {
                retval = trap_saved_exit_value;
            }
            running_trap = 0 as libc::c_int;
            array_dispose(ps);
            return retval;
        }
        restore_pipestatus_array(ps);
        return trap_saved_exit_value;
    }
}
#[no_mangle]
pub fn run_trap_cleanup(sig: libc::c_int) {
    unsafe {
        sigmodes[sig as usize] &= !(SIG_INPROGRESS | SIG_CHANGED);
    }
}
fn _run_trap_internal(sig: libc::c_int, tag: *mut libc::c_char) -> libc::c_int {
    unsafe {
        let trap_command: *mut libc::c_char;
        let old_trap: *mut libc::c_char;
        let mut trap_exit_value: libc::c_int;
        let mut save_return_catch_flag: libc::c_int = 0;
        let mut function_code: libc::c_int = 0;
        let old_modes: libc::c_int;
        let old_running: libc::c_int;
        let old_int: libc::c_int;
        let mut flags: libc::c_int;
        let mut save_return_catch: sigjmp_buf = [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: sigset_t { __val: [0; 16] },
        }; 1];
        let save_subst_varlist: *mut WordList;
        let save_tempenv: *mut HASH_TABLE;
        let mut pstate: sh_parser_state_t = _sh_parser_state_t {
            parser_state: 0,
            token_state: 0 as *mut libc::c_int,
            token: 0 as *mut libc::c_char,
            token_buffer_size: 0,
            input_line_terminator: 0,
            eof_encountered: 0,
            prompt_string_pointer: 0 as *mut *mut libc::c_char,
            current_command_line_count: 0,
            remember_on_history: 0,
            history_expansion_inhibited: 0,
            last_command_exit_value: 0,
            pipestatus: 0 as *mut ARRAY,
            last_shell_builtin: None,
            this_shell_builtin: None,
            expand_aliases: 0,
            echo_input_at_read: 0,
            need_here_doc: 0,
            here_doc_first_line: 0,
            redir_stack: [0 as *mut REDIRECT; 16],
        };
        let ps: *mut ARRAY;
        //old_running = -(1 as libc::c_int);
        //old_modes = old_running;
        ::core::ptr::write_volatile(&mut function_code as *mut libc::c_int, 0 as libc::c_int);
        trap_exit_value =
            ::core::ptr::read_volatile::<libc::c_int>(&function_code as *const libc::c_int);
        trap_saved_exit_value = last_command_exit_value;
        if sigmodes[sig as usize] & SIG_TRAPPED != 0
            && sigmodes[sig as usize] & SIG_IGNORED == 0 as libc::c_int
            && trap_list[sig as usize]
                != ::core::mem::transmute::<fn() -> (), *mut libc::c_char>(initialize_traps)
            && ((sig == 0 as libc::c_int
                || sig == 64 as libc::c_int + 1 as libc::c_int
                || sig == 64 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                || sig == 64 as libc::c_int + 1 as libc::c_int + 2 as libc::c_int)
                as libc::c_int
                == 0 as libc::c_int
                || sigmodes[sig as usize] & 0x10 as libc::c_int == 0 as libc::c_int)
        {
            old_trap = trap_list[sig as usize];
            old_modes = sigmodes[sig as usize];
            old_running = running_trap;
            sigmodes[sig as usize] |= SIG_INPROGRESS;
            sigmodes[sig as usize] &= !(SIG_CHANGED as libc::c_int);
            trap_command = savestring!(old_trap);
            running_trap = sig + 1 as libc::c_int;
            old_int = interrupt_state;
            ::core::ptr::write_volatile(
                &mut interrupt_state as *mut sig_atomic_t,
                0 as libc::c_int,
            );
            ps = save_pipestatus_array();
            save_parser_state(&mut pstate);
            save_subst_varlist = subst_assign_varlist;
            subst_assign_varlist = 0 as *mut WordList;
            save_tempenv = temporary_env;
            temporary_env = 0 as *mut HASH_TABLE;
            if sig != DEBUG_TRAP as libc::c_int {
                save_pipeline(1 as libc::c_int);
            }
            ::core::ptr::write_volatile(
                &mut save_return_catch_flag as *mut libc::c_int,
                return_catch_flag,
            );
            if return_catch_flag != 0 {
                xbcopy(
                    return_catch.as_mut_ptr() as *mut libc::c_char,
                    save_return_catch.as_mut_ptr() as *mut libc::c_char,
                    ::core::mem::size_of::<sigjmp_buf>() as libc::c_ulong as libc::c_int,
                );
                ::core::ptr::write_volatile(
                    &mut function_code as *mut libc::c_int,
                    __sigsetjmp(return_catch.as_mut_ptr(), 0 as libc::c_int),
                );
            }
            flags = 0x1 as libc::c_int | 0x4 as libc::c_int;
            if sig != 64 as libc::c_int + 1 as libc::c_int
                && sig != 64 as libc::c_int + 1 as libc::c_int + 2 as libc::c_int
                && sig != 64 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
            {
                flags |= 0x10 as libc::c_int;
            }
            evalnest += 1;
            if function_code == 0 as libc::c_int {
                parse_and_execute(trap_command, tag, flags);
                trap_exit_value = last_command_exit_value;
            } else {
                trap_exit_value = return_catch_value;
            }
            evalnest -= 1;
            if sig != 64 as libc::c_int + 1 as libc::c_int {
                restore_pipeline(1 as libc::c_int);
            }
            subst_assign_varlist = save_subst_varlist;
            restore_parser_state(&mut pstate);
            restore_pipestatus_array(ps);
            temporary_env = save_tempenv;
            if old_modes & 0x10 as libc::c_int == 0 as libc::c_int {
                sigmodes[sig as usize] &= !(0x10 as libc::c_int);
            }
            running_trap = old_running;
            ::core::ptr::write_volatile(&mut interrupt_state as *mut sig_atomic_t, old_int);
            if sigmodes[sig as usize] & 0x20 as libc::c_int != 0 {
                libc::free(old_trap as *mut libc::c_void);
                sigmodes[sig as usize] &= !(0x20 as libc::c_int);
                if terminating_signal != 0 {
                    termsig_handler(terminating_signal);
                }
            }
            if save_return_catch_flag != 0 {
                return_catch_flag = save_return_catch_flag;
                return_catch_value = trap_exit_value;
                xbcopy(
                    save_return_catch.as_mut_ptr() as *mut libc::c_char,
                    return_catch.as_mut_ptr() as *mut libc::c_char,
                    ::core::mem::size_of::<sigjmp_buf>() as libc::c_ulong as libc::c_int,
                );
                if function_code != 0 {
                    siglongjmp(return_catch.as_mut_ptr(), 1 as libc::c_int);
                }
            }
        }
        return trap_exit_value;
    }
}
#[no_mangle]
pub fn run_debug_trap() -> libc::c_int {
    unsafe {
        let mut trap_exit_value: libc::c_int;
        let old_verbose: libc::c_int;
        let save_pgrp: pid_t;
        let mut save_pipe: [libc::c_int; 2] = [0; 2];
        trap_exit_value = 0 as libc::c_int;
        if sigmodes[(DEBUG_TRAP as libc::c_int + 1 as libc::c_int) as usize]
            & SIG_TRAPPED as libc::c_int
            != 0
            && sigmodes[(DEBUG_TRAP as libc::c_int + 1 as libc::c_int) as usize] & SIG_IGNORED
                == 0 as libc::c_int
            && sigmodes[(DEBUG_TRAP as libc::c_int + 1 as libc::c_int) as usize]
                & SIG_INPROGRESS as libc::c_int
                == 0 as libc::c_int
        {
            save_pgrp = pipeline_pgrp;
            pipeline_pgrp = 0 as libc::c_int;
            save_pipeline(1 as libc::c_int);
            save_pgrp_pipe(save_pipe.as_mut_ptr(), 1 as libc::c_int);
            stop_making_children();
            old_verbose = echo_input_at_read;
            echo_input_at_read = if suppress_debug_trap_verbose != 0 {
                0 as libc::c_int
            } else {
                echo_input_at_read
            };
            trap_exit_value = _run_trap_internal(
                DEBUG_TRAP as libc::c_int + 1 as libc::c_int,
                b"debug trap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            echo_input_at_read = old_verbose;
            pipeline_pgrp = save_pgrp;
            restore_pipeline(1 as libc::c_int);
            close_pgrp_pipe();
            restore_pgrp_pipe(save_pipe.as_mut_ptr());
            if pipeline_pgrp > 0 as libc::c_int
                && subshell_environment
                    & (SUBSHELL_ASYNC as libc::c_int | SUBSHELL_PIPE as libc::c_int)
                    == 0 as libc::c_int
            {
                give_terminal_to(pipeline_pgrp, 1 as libc::c_int);
            }
            notify_and_cleanup();
            if debugging_mode != 0 && trap_exit_value == 2 as libc::c_int && return_catch_flag != 0
            {
                return_catch_value = trap_exit_value;
                siglongjmp(return_catch.as_mut_ptr(), 1 as libc::c_int);
            }
        }
        return trap_exit_value;
    }
}
#[no_mangle]
pub fn run_error_trap() {
    unsafe {
        if sigmodes[(ERROR_TRAP) as usize] & SIG_TRAPPED != 0
            && sigmodes[(ERROR_TRAP) as usize] & SIG_IGNORED == 0 as libc::c_int
            && sigmodes[(ERROR_TRAP) as usize] & SIG_INPROGRESS as libc::c_int == 0 as libc::c_int
        {
            _run_trap_internal(
                ERROR_TRAP as libc::c_int + 1 as libc::c_int + 1 as libc::c_int,
                b"error trap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
    }
}
#[no_mangle]
pub fn run_return_trap() {
    let old_exit_value: libc::c_int;
    unsafe {
        if sigmodes[(RETURN_TRAP as libc::c_int) as usize] & SIG_TRAPPED != 0
            && sigmodes[(RETURN_TRAP as libc::c_int) as usize] & SIG_IGNORED == 0 as libc::c_int
            && sigmodes[(RETURN_TRAP as libc::c_int) as usize] & SIG_INPROGRESS as libc::c_int
                == 0 as libc::c_int
        {
            old_exit_value = last_command_exit_value;
            _run_trap_internal(
                RETURN_TRAP as libc::c_int + 1 as libc::c_int + 2 as libc::c_int,
                b"return trap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            ::core::ptr::write_volatile(
                &mut last_command_exit_value as *mut libc::c_int,
                old_exit_value,
            );
        }
    }
}
#[no_mangle]
pub fn run_interrupt_trap(will_throw: libc::c_int) {
    unsafe {
        if will_throw != 0 && running_trap > 0 as libc::c_int {
            run_trap_cleanup(running_trap - 1 as libc::c_int);
        }
        pending_traps[libc::SIGINT as usize] = 0 as libc::c_int;
        catch_flag = 0 as libc::c_int;
        _run_trap_internal(
            libc::SIGINT as libc::c_int,
            b"interrupt trap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
}
#[no_mangle]
pub fn free_trap_strings() {
    unsafe {
        let mut i: libc::c_int;
        i = 0 as libc::c_int;
        while i < NSIG as libc::c_int {
            if trap_list[i as usize]
                != ::core::mem::transmute::<libc::intptr_t, *mut libc::c_char>(
                    libc::SIG_IGN as libc::intptr_t,
                )
            {
                free_trap_string(i);
            }
            i += 1;
        }
        i = NSIG as libc::c_int;
        while i < BASH_NSIG as libc::c_int {
            if sigmodes[i as usize] & SIG_TRAPPED == 0 as libc::c_int {
                free_trap_string(i);
                trap_list[i as usize] = 0 as *mut libc::c_void as *mut libc::c_char;
            }
            i += 1;
        }
    }
}

fn free_trap_string(sig: libc::c_int) {
    change_signal(
        sig,
        libc::SIG_DFL as *const libc::c_char as *mut libc::c_char,
    );

    unsafe {
        sigmodes[sig as usize] &= !(SIG_TRAPPED);
    }
}

fn reset_signal(sig: libc::c_int) {
    unsafe {
        set_signal_handler(sig, original_signals[sig as usize]);
        sigmodes[sig as usize] &= !(SIG_TRAPPED);
    }
}

fn restore_signal(sig: libc::c_int) {
    unsafe {
        set_signal_handler(sig, original_signals[sig as usize]);
        change_signal(
            sig,
            libc::SIG_DFL as *const libc::c_char as *mut libc::c_char, //::core::mem::transmute::<libc::intptr_t, *mut libc::c_char>(libc::SIG_DFL as libc::intptr_t),
        );
        sigmodes[sig as usize] &= !(SIG_TRAPPED);
    }
}

fn reset_or_restore_signal_handlers(reset: Option<sh_resetsig_func_t>) {
    unsafe {
        let mut i: libc::c_int;
        if sigmodes[EXIT_TRAP as libc::c_int as usize] & SIG_TRAPPED != 0 {
            sigmodes[EXIT_TRAP as libc::c_int as usize] &= !(SIG_TRAPPED);
            if reset
                != ::core::mem::transmute::<fn(libc::c_int) -> (), Option<sh_resetsig_func_t>>(
                    reset_signal,
                )
            {
                free_trap_command(EXIT_TRAP as libc::c_int);
                trap_list[EXIT_TRAP as libc::c_int as usize] =
                    0 as *mut libc::c_void as *mut libc::c_char;
            }
        }
        i = 1 as libc::c_int;
        while i < NSIG as libc::c_int {
            if sigmodes[i as usize] & SIG_TRAPPED != 0 {
                if trap_list[i as usize]
                    == ::core::mem::transmute::<libc::intptr_t, *mut libc::c_char>(
                        libc::SIG_IGN as libc::intptr_t,
                    )
                {
                    set_signal_handler(
                        i,
                        ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(
                            libc::SIG_IGN as libc::intptr_t,
                        ),
                    );
                } else {
                    (Some(reset.expect("non-null function pointer")))
                        .expect("non-null function pointer")(i);
                }
            } else if sigmodes[i as usize] & SIG_SPECIAL as libc::c_int != 0 {
                (Some(reset.expect("non-null function pointer")))
                    .expect("non-null function pointer")(i);
            }
            pending_traps[i as usize] = 0 as libc::c_int;
            i += 1;
        }
        if function_trace_mode == 0 as libc::c_int {
            sigmodes[(DEBUG_TRAP as libc::c_int) as usize] &= !(SIG_TRAPPED);
            sigmodes[(RETURN_TRAP as libc::c_int) as usize] &= !(SIG_TRAPPED);
        }
        if error_trace_mode == 0 as libc::c_int {
            sigmodes[(ERROR_TRAP as libc::c_int) as usize] &= !(SIG_TRAPPED);
        }
    }
}

#[no_mangle]
pub fn reset_signal_handlers() {
    unsafe {
        reset_or_restore_signal_handlers(::core::mem::transmute::<
            fn(libc::c_int) -> (),
            Option<sh_resetsig_func_t>,
        >(reset_signal));
    }
}

#[no_mangle]
pub fn restore_original_signals() {
    unsafe {
        reset_or_restore_signal_handlers(::core::mem::transmute::<
            fn(libc::c_int) -> (),
            Option<sh_resetsig_func_t>,
        >(restore_signal));
    }
}

#[no_mangle]
pub fn maybe_call_trap_handler(sig: libc::c_int) -> libc::c_int {
    unsafe {
        if sigmodes[sig as usize] & SIG_TRAPPED != 0
            && sigmodes[sig as usize] & SIG_IGNORED == 0 as libc::c_int
        {
            match sig as usize {
                2 => {
                    run_interrupt_trap(0 as libc::c_int);
                }
                EXIT_TRAP => {
                    run_exit_trap();
                }
                DEBUG_TRAP => {
                    run_debug_trap();
                }
                ERROR_TRAP => {
                    run_error_trap();
                }
                _ => {
                    trap_handler(sig);
                }
            }
            return 1 as libc::c_int;
        } else {
            return 0 as libc::c_int;
        };
    }
}

#[no_mangle]
pub fn signal_is_trapped(sig: libc::c_int) -> libc::c_int {
    unsafe {
        return sigmodes[sig as usize] & SIG_TRAPPED;
    }
}
#[no_mangle]
pub fn signal_is_pending(sig: libc::c_int) -> libc::c_int {
    unsafe {
        return pending_traps[sig as usize];
    }
}

#[no_mangle]
pub fn signal_is_special(sig: libc::c_int) -> libc::c_int {
    unsafe {
        return sigmodes[sig as usize] & SIG_SPECIAL as libc::c_int;
    }
}

#[no_mangle]
pub fn signal_is_ignored(sig: libc::c_int) -> libc::c_int {
    unsafe {
        return sigmodes[sig as usize] & SIG_HARD_IGNORE as libc::c_int;
    }
}

#[no_mangle]
pub fn signal_is_hard_ignored(sig: libc::c_int) -> libc::c_int {
    unsafe {
        return sigmodes[sig as usize] & SIG_HARD_IGNORE as libc::c_int;
    }
}

#[no_mangle]
pub fn set_signal_hard_ignored(sig: libc::c_int) {
    unsafe {
        sigmodes[sig as usize] |= SIG_HARD_IGNORE as libc::c_int;
        original_signals[sig as usize] = ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(
            libc::SIG_IGN as libc::intptr_t,
        );
    }
}

#[no_mangle]
pub fn set_signal_ignored(sig: libc::c_int) {
    unsafe {
        original_signals[sig as usize] = ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(
            libc::SIG_IGN as libc::intptr_t,
        );
    }
}

#[no_mangle]
pub fn signal_in_progress(sig: libc::c_int) -> libc::c_int {
    unsafe {
        return sigmodes[sig as usize] & SIG_INPROGRESS as libc::c_int;
    }
}
