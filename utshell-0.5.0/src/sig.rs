use crate::bashhist::maybe_save_shell_history;
use crate::bashline::{bashline_reset, bashline_set_event_hook};
use crate::builtins::evalstring::parse_and_execute_cleanup;
use crate::builtins::read::{read_builtin, read_tty_cleanup};
use crate::jobs::{
    end_job_control, give_terminal_to, hangup_all_jobs, initialize_job_signals, procsub_clear,
};
use crate::src_common::*;
use crate::subst::{unlink_all_fifos, unlink_fifo_list};
use crate::trap::{
    get_original_signal, run_exit_trap, run_interrupt_trap, run_trap_cleanup,
    set_signal_hard_ignored, set_trap_state, signal_is_pending, signal_is_special,
};
use crate::unwind_prot::run_unwind_protects;
use crate::variables::set_pipestatus_from_exit;
use crate::y_tab::reset_parser;

static mut old_winch: Option<SigHandler> = None;
#[no_mangle]
pub fn initialize_signals(mut reinit: libc::c_int) {
    initialize_shell_signals();
    initialize_job_signals();
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct termsig {
    pub signum: libc::c_int,
    pub orig_handler: Option<SigHandler>,
    pub orig_flags: libc::c_int,
    pub core_dump: libc::c_int,
}

static mut terminating_signals: [termsig; 17] = {
    [
        {
            let mut init = termsig {
                signum: SIGHUP as libc::c_int,
                orig_handler: None,
                orig_flags: 0 as libc::c_int,
                core_dump: 0,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGINT as libc::c_int,
                orig_handler: None,
                orig_flags: 0 as libc::c_int,
                core_dump: 0,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGILL as libc::c_int,
                orig_handler: None,
                orig_flags: 0 as libc::c_int,
                core_dump: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGTRAP as libc::c_int,
                orig_handler: None,
                orig_flags: 0 as libc::c_int,
                core_dump: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGIOT as libc::c_int,
                orig_handler: None,
                orig_flags: 0 as libc::c_int,
                core_dump: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGFPE as libc::c_int,
                orig_handler: None,
                orig_flags: 0 as libc::c_int,
                core_dump: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGBUS as libc::c_int,
                orig_handler: None,
                orig_flags: 0 as libc::c_int,
                core_dump: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGSEGV as libc::c_int,
                orig_handler: None,
                orig_flags: 0 as libc::c_int,
                core_dump: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGSYS as libc::c_int,
                orig_handler: None,
                orig_flags: 0 as libc::c_int,
                core_dump: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGPIPE as libc::c_int,
                orig_handler: None,
                orig_flags: 0 as libc::c_int,
                core_dump: 0,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGALRM as libc::c_int,
                orig_handler: None,
                orig_flags: 0 as libc::c_int,
                core_dump: 0,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGTERM as libc::c_int,
                orig_handler: None,
                orig_flags: 0 as libc::c_int,
                core_dump: 0,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGXCPU as libc::c_int,
                orig_handler: None,
                orig_flags: 0 as libc::c_int,
                core_dump: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGXFSZ as libc::c_int,
                orig_handler: None,
                orig_flags: 0 as libc::c_int,
                core_dump: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGVTALRM as libc::c_int,
                orig_handler: None,
                orig_flags: 0 as libc::c_int,
                core_dump: 0,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGUSR1 as libc::c_int,
                orig_handler: None,
                orig_flags: 0 as libc::c_int,
                core_dump: 0,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGUSR2 as libc::c_int,
                orig_handler: None,
                orig_flags: 0 as libc::c_int,
                core_dump: 0,
            };
            init
        },
    ]
};

static mut termsigs_initialized: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub fn initialize_terminating_signals() {
    unsafe {
        let mut i: libc::c_int = 0;
        let mut act: crate::src_common::sigaction = crate::src_common::sigaction {
            __sigaction_handler: sigaction__bindgen_ty_1 { sa_handler: None },
            sa_mask: __sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };
        let mut oact: crate::src_common::sigaction = crate::src_common::sigaction {
            __sigaction_handler: sigaction__bindgen_ty_1 { sa_handler: None },
            sa_mask: __sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };

        if termsigs_initialized != 0 {
            return;
        }
        act.__sigaction_handler.sa_handler = Some(termsig_sighandler);
        act.sa_flags = 0 as libc::c_int;

        sigemptyset(&mut act.sa_mask);
        sigemptyset(&mut oact.sa_mask);

        i = 0;
        while (i as libc::c_ulong) < TERMSIGS_LENGTH!() {
            sigaddset(&mut act.sa_mask, XSIG!(i));
            i += 1;
        }
        i = 0;
        while (i as libc::c_ulong) < TERMSIGS_LENGTH!() {
            if !(signal_is_trapped(XSIG!(i)) != 0) {
                sigaction(XSIG!(i), &mut act, &mut oact);
                // XHANDLER!(i) = Some(oact.__sigaction_handler.sa_handler);
                terminating_signals[i as usize].orig_handler = oact.__sigaction_handler.sa_handler;
                XSAFLAGS!(i) = oact.sa_flags;

                if interactive_shell == 0
                    && terminating_signals[i as usize].orig_handler
                        == ::std::mem::transmute::<libc::intptr_t, Option<SigHandler>>(
                            1 as libc::c_int as libc::intptr_t,
                        )
                {
                    sigaction(XSIG!(i), &mut oact, &mut act);
                    set_signal_hard_ignored(XSIG!(i));
                }
                if XSIG!(i) == SIGPROF as libc::c_int
                    && terminating_signals[i as usize].orig_handler.is_some()
                    && terminating_signals[i as usize].orig_handler
                        != ::std::mem::transmute::<libc::intptr_t, Option<SigHandler>>(
                            1 as libc::c_int as libc::intptr_t,
                        )
                {
                    sigaction(
                        XSIG!(i),
                        &mut oact,
                        0 as *mut c_void as *mut crate::src_common::sigaction,
                    );
                }
            }
            i += 1;
        }
        termsigs_initialized = 1;
    }
}

fn initialize_shell_signals() {
    unsafe {
        if interactive != 0 {
            initialize_terminating_signals();
        }
        sigemptyset(&mut top_level_mask);
        sigprocmask(
            SIG_BLOCK as libc::c_int,
            0 as *mut c_void as *mut sigset_t,
            &mut top_level_mask,
        );
        if sigismember(&mut top_level_mask, SIGCHLD as libc::c_int) != 0 {
            sigdelset(&mut top_level_mask, SIGCHLD as libc::c_int);
            sigprocmask(
                SIG_SETMASK as libc::c_int,
                &mut top_level_mask,
                0 as *mut c_void as *mut sigset_t,
            );
        }
        set_signal_handler(
            SIGQUIT as libc::c_int,
            ::std::mem::transmute::<libc::intptr_t, Option<SigHandler>>(
                1 as libc::c_int as libc::intptr_t,
            ),
        );
        if interactive != 0 {
            set_signal_handler(SIGINT as libc::c_int, Some(sigint_sighandler));
            get_original_signal(SIGTERM as libc::c_int);
            set_signal_handler(
                SIGTERM as libc::c_int,
                ::std::mem::transmute::<libc::intptr_t, Option<SigHandler>>(
                    1 as libc::c_int as libc::intptr_t,
                ),
            );
            set_sigwinch_handler();
        }
    }
}

#[no_mangle]
pub fn reset_terminating_signals() {
    let mut i: libc::c_int = 0;
    let mut act: crate::src_common::sigaction = crate::src_common::sigaction {
        __sigaction_handler: sigaction__bindgen_ty_1 { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    unsafe {
        if termsigs_initialized == 0 {
            return;
        }
        act.sa_flags = 0;
        sigemptyset(&mut act.sa_mask);

        i = 0;
        while (i as libc::c_ulong) < TERMSIGS_LENGTH!() {
            if !(signal_is_trapped(XSIG!(i)) != 0 || signal_is_special(XSIG!(i)) != 0) {
                act.__sigaction_handler.sa_handler = terminating_signals[i as usize].orig_handler;

                act.sa_flags = XSAFLAGS!(i);
                sigaction(
                    XSIG!(i),
                    &mut act,
                    0 as *mut c_void as *mut crate::src_common::sigaction,
                );
            }
            i += 1;
        }
        termsigs_initialized = 0;
    }
}

#[no_mangle]
pub fn top_level_cleanup() {
    unsafe {
        while parse_and_execute_level != 0 {
            parse_and_execute_cleanup(-(1 as libc::c_int));
        }
        unlink_fifo_list();
        run_unwind_protects();
        funcnest = 0 as libc::c_int;
        breaking = funcnest;
        continuing = breaking;
        loop_level = continuing;
        wait_intr_flag = 0 as libc::c_int;
        return_catch_flag = wait_intr_flag;
        comsub_ignore_return = return_catch_flag;
        executing_list = comsub_ignore_return;
    }
}

#[no_mangle]
pub fn throw_to_top_level() {
    let mut print_newline: libc::c_int = 0 as libc::c_int;
    unsafe {
        if interrupt_state != 0 {
            if last_command_exit_value < 128 as libc::c_int {
                last_command_exit_value = 128 + SIGINT as libc::c_int;
            }
            set_pipestatus_from_exit(last_command_exit_value);
            print_newline = 1 as libc::c_int;
            interrupt_state -= 1;
        }

        if interrupt_state != 0 {
            return;
        }

        last_command_exit_signal = if last_command_exit_value > 128 as libc::c_int {
            last_command_exit_value - 128 as libc::c_int
        } else {
            0 as libc::c_int
        };
        last_command_exit_value |= 128;
        set_pipestatus_from_exit(last_command_exit_value);

        if signal_is_trapped(SIGINT as libc::c_int) != 0
            && signal_is_pending(SIGINT as libc::c_int) != 0
        {
            run_interrupt_trap(1 as libc::c_int);
        }

        while parse_and_execute_level != 0 {
            parse_and_execute_cleanup(-(1 as libc::c_int));
        }

        if running_trap > 0 as libc::c_int {
            run_trap_cleanup(running_trap - 1 as libc::c_int);
            running_trap = 0 as libc::c_int;
        }

        give_terminal_to(shell_pgrp, 0 as libc::c_int);

        restore_sigmask();
        reset_parser();

        if interactive != 0 {
            bashline_reset();
        }

        unlink_fifo_list();

        run_unwind_protects();
        funcnest = 0 as libc::c_int;
        breaking = funcnest;
        continuing = breaking;
        loop_level = continuing;

        wait_intr_flag = 0 as libc::c_int;
        return_catch_flag = wait_intr_flag;
        comsub_ignore_return = return_catch_flag;
        executing_list = comsub_ignore_return;

        if interactive != 0 && print_newline != 0 {
            fflush(stdout);
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            fflush(stderr);
        }

        if interactive != 0
            || interactive_shell != 0 && shell_initialized == 0
            || print_newline != 0 && signal_is_trapped(SIGINT as libc::c_int) != 0
        {
            jump_to_top_level(DISCARD as libc::c_int);
        } else {
            jump_to_top_level(EXITPROG as libc::c_int);
        };
    }
}

#[no_mangle]
pub fn jump_to_top_level(mut value: libc::c_int) {
    unsafe {
        siglongjmp(top_level.as_mut_ptr(), value);
    }
}

#[no_mangle]
pub fn restore_sigmask() {
    unsafe {
        sigprocmask(
            SIG_SETMASK as libc::c_int,
            &mut top_level_mask,
            0 as *mut c_void as *mut sigset_t,
        );
    }
}

#[no_mangle]
pub fn termsig_sighandler(mut sig: libc::c_int) {
    unsafe {
        if sig != SIGHUP as libc::c_int
            && sig != SIGINT as libc::c_int
            && sig != SIGPIPE as libc::c_int
            && sig != SIGALRM as libc::c_int
            && sig != SIGTERM as libc::c_int
            && sig != SIGXCPU as libc::c_int
            && sig != SIGXFSZ as libc::c_int
            && sig != SIGVTALRM as libc::c_int
            && sig != SIGUSR1 as libc::c_int
            && sig != SIGUSR2 as libc::c_int
            && sig == terminating_signal
        {
            terminate_immediately = 1 as libc::c_int;
        }

        terminating_signal = sig;

        if terminate_immediately != 0 {
            if interactive_shell == 0
                || interactive == 0
                || sig != SIGHUP as libc::c_int && sig != SIGTERM as libc::c_int
                || no_line_editing != 0
                || RL_ISSTATE!(RL_STATE_READCMD!()) == 0
            {
                history_lines_this_session = 0 as libc::c_int;
            }
            terminate_immediately = 0 as libc::c_int;
            termsig_handler(sig);
        }
        if RL_ISSTATE!(RL_STATE_SIGHANDLER!()) != 0 || RL_ISSTATE!(RL_STATE_TERMPREPPED!()) != 0 {
            bashline_set_event_hook();
        }
    }
}

#[no_mangle]
pub fn termsig_handler(mut sig: libc::c_int) {
    static mut handling_termsig: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut core: libc::c_int = 0;
    let mut mask: sigset_t = __sigset_t { __val: [0; 16] };
    unsafe {
        if handling_termsig != 0 {
            return;
        }
        handling_termsig = 1;
        terminating_signal = 0;

        if sig == SIGINT as libc::c_int && signal_is_trapped(SIGINT as libc::c_int) != 0 {
            run_interrupt_trap(0 as libc::c_int);
        }
        if interactive_shell != 0
            && interactive != 0
            && (sig == SIGHUP as libc::c_int || sig == SIGTERM as libc::c_int)
            && remember_on_history != 0
        {
            maybe_save_shell_history();
        }
        if this_shell_builtin == Some(read_builtin) {
            read_tty_cleanup();
        }
        if sig == SIGHUP as libc::c_int
            && (interactive != 0
                || subshell_environment
                    & (SUBSHELL_COMSUB as libc::c_int | SUBSHELL_PROCSUB as libc::c_int)
                    != 0)
        {
            hangup_all_jobs();
        }
        if subshell_environment & (SUBSHELL_COMSUB as libc::c_int | SUBSHELL_PROCSUB as libc::c_int)
            == 0
        {
            end_job_control();
        }
        unlink_all_fifos();
        procsub_clear();

        funcnest = 0 as libc::c_int;
        breaking = funcnest;
        continuing = breaking;
        loop_level = continuing;

        wait_intr_flag = 0 as libc::c_int;
        return_catch_flag = wait_intr_flag;
        comsub_ignore_return = return_catch_flag;
        executing_list = comsub_ignore_return;

        run_exit_trap();

        restore_sigmask();

        set_signal_handler(sig, None);

        kill(getpid(), sig);

        if dollar_dollar_pid != 1 as libc::c_int {
            exit(128 as libc::c_int + sig);
        }
        sigprocmask(
            SIG_SETMASK as libc::c_int,
            0 as *mut c_void as *mut sigset_t,
            &mut mask,
        );
        core = 0 as libc::c_int;
        i = core;
        while (i as libc::c_ulong) < TERMSIGS_LENGTH!() {
            set_signal_handler(XSIG!(i), None);
            sigdelset(&mut mask, XSIG!(i));
            if sig == XSIG!(i) {
                core = XCOREDUMP!(i);
            }
            i += 1;
        }
        sigprocmask(
            SIG_SETMASK as libc::c_int,
            &mut mask,
            0 as *mut c_void as *mut sigset_t,
        );
        if core != 0 {
            ::std::ptr::write_volatile(
                0 as *mut c_void as *mut libc::c_ulong,
                (0xdead0000 as libc::c_uint).wrapping_add(sig as libc::c_uint) as libc::c_ulong,
            );
        }
        exit(128 as libc::c_int + sig);
    }
}

#[no_mangle]
pub fn sigint_sighandler(mut sig: libc::c_int) {
    unsafe {
        if interrupt_state == 0 as libc::c_int {
            interrupt_state += 1;
        }
        if wait_intr_flag != 0 {
            last_command_exit_value = 128 + sig;
            set_pipestatus_from_exit(last_command_exit_value);
            wait_signal_received = sig;
            return;
        }

        if signal_is_trapped(sig) != 0 {
            set_trap_state(sig);
        }
        if interrupt_immediately != 0 {
            interrupt_immediately = 0 as libc::c_int;
            set_exit_status(128 as libc::c_int + sig);
            throw_to_top_level();
        } else if rl_readline_state & 0x8000 as libc::c_int as libc::c_ulong != 0 {
            bashline_set_event_hook();
        }
    }
}

#[no_mangle]
pub fn sigwinch_sighandler(mut sig: libc::c_int) {
    unsafe {
        sigwinch_received = 1;
    }
}

#[no_mangle]
pub fn set_sigwinch_handler() {
    unsafe {
        old_winch = set_signal_handler(SIGWINCH as libc::c_int, old_winch);
    }
}

#[no_mangle]
pub fn unset_sigwinch_handler() {
    unsafe {
        set_signal_handler(SIGWINCH as libc::c_int, old_winch);
    }
}

#[no_mangle]
pub fn sigterm_sighandler(mut sig: libc::c_int) {
    unsafe {
        sigterm_received = 1;
    }
}

#[no_mangle]
pub fn set_signal_handler(
    mut sig: libc::c_int,
    mut handler: Option<SigHandler>,
) -> Option<SigHandler> {
    unsafe {
        let mut act: crate::src_common::sigaction = crate::src_common::sigaction {
            __sigaction_handler: sigaction__bindgen_ty_1 { sa_handler: None },
            sa_mask: __sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };
        let mut oact: crate::src_common::sigaction = crate::src_common::sigaction {
            __sigaction_handler: sigaction__bindgen_ty_1 { sa_handler: None },
            sa_mask: __sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };

        act.__sigaction_handler.sa_handler = handler;

        act.sa_flags = 0;

        if sig == SIGCHLD as libc::c_int {
            act.sa_flags |= SA_RESTART as libc::c_int;
        }
        if sig == SIGWINCH as libc::c_int {
            act.sa_flags |= SA_RESTART as libc::c_int;
        }
        if sig == SIGTERM as libc::c_int && handler == Some(sigterm_sighandler) {
            act.sa_flags |= SA_RESTART as libc::c_int;
        }
        sigemptyset(&mut act.sa_mask);
        sigemptyset(&mut oact.sa_mask);
        if sigaction(sig, &mut act, &mut oact) == 0 as libc::c_int {
            return oact.__sigaction_handler.sa_handler;
        } else {
            return None;
        };
    }
}
