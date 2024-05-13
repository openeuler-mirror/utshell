//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

use libc::{c_int, c_void, c_char};
use r_bash::*;
use r_readline::{rl_readline_state};
use rcommon::{WordList};

extern "C" {
    fn read_builtin(_: *mut WordList) -> libc::c_int;
}


#[no_mangle]
pub static mut interrupt_state: sig_atomic_t = 0 as c_int;
#[no_mangle]
pub static mut sigwinch_received: sig_atomic_t = 0 as c_int;
#[no_mangle]
pub static mut sigterm_received: sig_atomic_t = 0 as c_int;
#[no_mangle]
pub static mut terminating_signal: sig_atomic_t = 0 as c_int;
#[no_mangle]
pub static mut top_level: sigjmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
#[no_mangle]
pub static mut top_level_mask: sigset_t = __sigset_t { __val: [0; 16] };
#[no_mangle]
pub static mut interrupt_immediately: c_int = 0 as c_int;
#[no_mangle]
pub static mut terminate_immediately: c_int = 0 as c_int;

static mut old_winch: SigHandler = unsafe {
    None
};
#[no_mangle]
pub unsafe extern "C" fn initialize_signals(mut reinit: c_int) {
    initialize_shell_signals();
    initialize_job_signals();
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct termsig {
    pub signum: libc::c_int,
    pub orig_handler: SigHandler,
    pub orig_flags: libc::c_int,
    pub core_dump: libc::c_int,
}


static mut terminating_signals: [termsig; 17] = unsafe {
    [
        {
            let mut init = termsig {
                signum: SIGHUP as c_int,
                orig_handler: None,
                orig_flags: 0 as c_int,
                core_dump: 0,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGINT as c_int,
                orig_handler: None,
                orig_flags: 0 as c_int,
                core_dump: 0,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGILL as c_int,
                orig_handler: None,
                orig_flags: 0 as c_int,
                core_dump: 1 as c_int,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGTRAP as c_int,
                orig_handler: None,
                orig_flags: 0 as c_int,
                core_dump: 1 as c_int,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGIOT as c_int,
                orig_handler: None,
                orig_flags: 0 as c_int,
                core_dump: 1 as c_int,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGFPE as c_int,
                orig_handler: None,
                orig_flags: 0 as c_int,
                core_dump: 1 as c_int,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGBUS as c_int,
                orig_handler: None,
                orig_flags: 0 as c_int,
                core_dump: 1 as c_int,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGSEGV as c_int,
                orig_handler: None,
                orig_flags: 0 as c_int,
                core_dump: 1 as c_int,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGSYS as c_int,
                orig_handler: None,
                orig_flags: 0 as c_int,
                core_dump: 1 as c_int,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGPIPE as c_int,
                orig_handler: None,
                orig_flags: 0 as c_int,
                core_dump: 0,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGALRM as c_int,
                orig_handler: None,
                orig_flags: 0 as c_int,
                core_dump: 0,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGTERM as c_int,
                orig_handler: None,
                orig_flags: 0 as c_int,
                core_dump: 0,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGXCPU as c_int,
                orig_handler: None,
                orig_flags: 0 as c_int,
                core_dump: 1 as c_int,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGXFSZ as c_int,
                orig_handler: None,
                orig_flags: 0 as c_int,
                core_dump: 1 as c_int,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGVTALRM as c_int,
                orig_handler: None,
                orig_flags: 0 as c_int,
                core_dump: 0,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGUSR1 as c_int,
                orig_handler: None,
                orig_flags: 0 as c_int,
                core_dump: 0,
            };
            init
        },
        {
            let mut init = termsig {
                signum: SIGUSR2 as c_int,
                orig_handler: None,
                orig_flags: 0 as c_int,
                core_dump: 0,
            };
            init
        },
    ]
};


#[macro_export]
macro_rules! TERMSIGS_LENGTH {
    () => {
        (::std::mem::size_of::<[termsig; 17]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<termsig>() as libc::c_ulong) 
    };
}

#[macro_export]
macro_rules! XSIG {
    ($x:expr) => {
        terminating_signals[$x as usize].signum
    };
}

#[macro_export]
macro_rules! XHANDLER {
    ($x:expr) => {
      terminating_signals[$x as usize].orig_handler
    };
}


#[macro_export]
macro_rules! XSAFLAGS {
    ($x:expr) => {
        terminating_signals[$x as usize].orig_flags
    };
}

#[macro_export]
macro_rules! XCOREDUMP {
    ($x:expr) => {
        terminating_signals[$x as usize].core_dump
    };
}

static mut termsigs_initialized: c_int = 0 as c_int;
#[no_mangle]
pub unsafe extern "C" fn initialize_terminating_signals() {
    let mut i: c_int = 0;
    let mut act: sigaction = sigaction {
        __sigaction_handler: sigaction__bindgen_ty_1 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut oact: sigaction = sigaction {
        __sigaction_handler: sigaction__bindgen_ty_1 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };

    if termsigs_initialized != 0 {
        return;
    }
    act.__sigaction_handler.sa_handler = 
        Some(termsig_sighandler as unsafe extern "C" fn(c_int) -> ());
    act.sa_flags = 0 as c_int;

    sigemptyset(&mut act.sa_mask);
    sigemptyset(&mut oact.sa_mask);

    i = 0 ;
    while (i as libc::c_ulong) < TERMSIGS_LENGTH!()
    {
        sigaddset(&mut act.sa_mask, XSIG!(i));
        i += 1;
    }
    i = 0  ;
    while (i as libc::c_ulong) < TERMSIGS_LENGTH!()
    {
        if !(signal_is_trapped(XSIG!(i)) != 0) {
            sigaction(XSIG!(i), &mut act, &mut oact);
            // XHANDLER!(i) = Some(oact.__sigaction_handler.sa_handler);
            terminating_signals[i as usize].orig_handler= oact.__sigaction_handler.sa_handler;
            XSAFLAGS!(i) = oact.sa_flags;
            
            if interactive_shell == 0 
                && terminating_signals[i as usize].orig_handler
                    == ::std::mem::transmute::<
                        libc::intptr_t,
                        SigHandler,
                    >(1 as c_int as libc::intptr_t)
            {
                sigaction(XSIG!(i), &mut oact, &mut act);
                set_signal_hard_ignored(XSIG!(i));
            }
            if XSIG!(i) == SIGPROF as c_int
                && terminating_signals[i as usize].orig_handler.is_some()
                && terminating_signals[i as usize].orig_handler
                    != ::std::mem::transmute::<
                        libc::intptr_t,
                        SigHandler,
                    >(1 as c_int as libc::intptr_t)
            {
                sigaction(
                    XSIG!(i),
                    &mut oact,
                    0 as *mut c_void as *mut sigaction,
                );
            }
        }
        i += 1;
    }
    termsigs_initialized = 1 ;
}


unsafe extern "C" fn initialize_shell_signals() {
    if interactive != 0 {
        initialize_terminating_signals();
    }
    sigemptyset(&mut top_level_mask);
    sigprocmask(
        SIG_BLOCK as c_int,
        0 as *mut c_void as *mut sigset_t,
        &mut top_level_mask,
    );
    if sigismember(&mut top_level_mask, SIGCHLD as c_int) != 0 {
        sigdelset(&mut top_level_mask, SIGCHLD as c_int);
        sigprocmask(
            SIG_SETMASK as c_int,
            &mut top_level_mask,
            0 as *mut c_void as *mut sigset_t,
        );
    }
    set_signal_handler(
        SIGQUIT as c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            SigHandler,
        >(1 as c_int as libc::intptr_t));
    if interactive != 0 {
        set_signal_handler(
            SIGINT as c_int,
            Some(sigint_sighandler as unsafe extern "C" fn(c_int) -> ()),
        );
        get_original_signal(SIGTERM as c_int);
        set_signal_handler(
            SIGTERM as c_int,
            ::std::mem::transmute::<
                libc::intptr_t,
                SigHandler,
            >(1 as c_int as libc::intptr_t));
        set_sigwinch_handler();
    }
}

#[no_mangle]
pub unsafe extern "C" fn reset_terminating_signals() {
    let mut i: c_int = 0;
    let mut act: sigaction = sigaction {
        __sigaction_handler: sigaction__bindgen_ty_1 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };

    if termsigs_initialized == 0 {
        return;
    }
    act.sa_flags = 0 ;
    sigemptyset(&mut act.sa_mask);

    i = 0 ;
    while (i as libc::c_ulong) < TERMSIGS_LENGTH!()
    {
        if !(signal_is_trapped(XSIG!(i)) != 0
            || signal_is_special(XSIG!(i)) != 0)
        {
            act.__sigaction_handler.sa_handler = terminating_signals[i as usize].orig_handler;

            act.sa_flags = XSAFLAGS!(i);
            sigaction(
                XSIG!(i),
                &mut act,
                0 as *mut c_void as *mut sigaction,
            );
        }
        i += 1;
    }
    termsigs_initialized = 0 ;
}


#[no_mangle]
pub unsafe extern "C" fn top_level_cleanup() {
    while parse_and_execute_level != 0 {
        parse_and_execute_cleanup(-(1 as c_int));
    }
    unlink_fifo_list();
    run_unwind_protects();
    funcnest = 0 as c_int;
    breaking = funcnest;
    continuing = breaking;
    loop_level = continuing;
    wait_intr_flag = 0 as c_int;
    return_catch_flag = wait_intr_flag;
    comsub_ignore_return = return_catch_flag;
    executing_list = comsub_ignore_return;
}


#[no_mangle]
pub unsafe extern "C" fn throw_to_top_level() {
    let mut print_newline: c_int = 0 as c_int;

    if interrupt_state != 0 {
        if last_command_exit_value < 128 as c_int {
            last_command_exit_value = 128 + SIGINT as c_int;
        }
        set_pipestatus_from_exit(last_command_exit_value);
        print_newline = 1 as c_int;
        interrupt_state -= 1;
    }

    if interrupt_state != 0 {
        return;
    }

    last_command_exit_signal = if last_command_exit_value > 128 as c_int {
        last_command_exit_value - 128 as c_int
    } else {
        0 as c_int
    };
    last_command_exit_value |= 128;
    set_pipestatus_from_exit(last_command_exit_value);

    if signal_is_trapped(SIGINT as c_int) != 0
        && signal_is_pending(SIGINT as c_int) != 0
    {
        run_interrupt_trap(1 as c_int);
    }

    while parse_and_execute_level != 0 {
        parse_and_execute_cleanup(-(1 as c_int));
    }

    if running_trap > 0 as c_int {
        run_trap_cleanup(running_trap - 1 as c_int);
        running_trap = 0 as c_int;
    }

    give_terminal_to(shell_pgrp, 0 as c_int);

    restore_sigmask();
    reset_parser();

    if interactive != 0 {
        bashline_reset();
    }

    unlink_fifo_list();

    run_unwind_protects();
    funcnest = 0 as c_int;
    breaking = funcnest;
    continuing = breaking;
    loop_level = continuing;

    wait_intr_flag = 0 as c_int;
    return_catch_flag = wait_intr_flag;
    comsub_ignore_return = return_catch_flag;
    executing_list = comsub_ignore_return;

    if interactive != 0 && print_newline != 0 {
        fflush(stdout);
        fprintf(stderr, b"\n\0" as *const u8 as *const c_char);
        fflush(stderr);
    }

    if interactive != 0 || interactive_shell != 0 && shell_initialized == 0
        || print_newline != 0 && signal_is_trapped(SIGINT as c_int) != 0
    {
        jump_to_top_level(DISCARD as c_int);
    } else {
        jump_to_top_level(EXITPROG as c_int);
    };
}

#[no_mangle]
pub unsafe extern "C" fn jump_to_top_level(mut value: c_int) {
    siglongjmp(top_level.as_mut_ptr(), value);
}

#[no_mangle]
pub unsafe extern "C" fn restore_sigmask() {
    sigprocmask(
        SIG_SETMASK as c_int,
        &mut top_level_mask,
        0 as *mut c_void as *mut sigset_t,
    );
}

#[macro_export]
macro_rules! RL_ISSTATE {
    ($x:expr) => {
        rl_readline_state & ($x)
    };
}

#[macro_export]
macro_rules! RL_STATE_READCMD {
    () => {
        0x0000008
    };
}

#[macro_export]
macro_rules! RL_STATE_SIGHANDLER {
    () => {
        0x0008000
    };
}

#[macro_export]
macro_rules! RL_STATE_TERMPREPPED {
    () => {
        0x0000004
    };
}

#[no_mangle]
pub unsafe extern "C" fn termsig_sighandler(mut sig: c_int) {
    if sig != SIGHUP as c_int && sig != SIGINT as c_int && sig != SIGPIPE as c_int
        && sig != SIGALRM as c_int && sig != SIGTERM as c_int
        && sig != SIGXCPU as c_int && sig != SIGXFSZ as c_int
        && sig != SIGVTALRM as c_int && sig != SIGUSR1 as c_int
        && sig != SIGUSR2 as c_int && sig == terminating_signal
    {
        terminate_immediately = 1 as c_int;
    }

    terminating_signal = sig;

    if terminate_immediately != 0 {
        if interactive_shell == 0 || interactive == 0
            || sig != SIGHUP as c_int && sig != SIGTERM as c_int
            || no_line_editing != 0
            || RL_ISSTATE!(RL_STATE_READCMD!()) == 0 
        {
            history_lines_this_session = 0 as c_int;
        }
        terminate_immediately = 0 as c_int;
        termsig_handler(sig);
    }
    if RL_ISSTATE!(RL_STATE_SIGHANDLER!()) != 0
        || RL_ISSTATE!(RL_STATE_TERMPREPPED!()) != 0
    {
        bashline_set_event_hook();
    }
}

#[no_mangle]
pub unsafe extern "C" fn termsig_handler(mut sig: c_int) {
    static mut handling_termsig: c_int = 0 as c_int;
    let mut i: c_int = 0;
    let mut core: c_int = 0;
    let mut mask: sigset_t = __sigset_t { __val: [0; 16] };

    if handling_termsig != 0 {
        return;
    }
    handling_termsig = 1 ;
    terminating_signal = 0;

    if sig == SIGINT as c_int && signal_is_trapped(SIGINT as c_int) != 0 {
        run_interrupt_trap(0 as c_int);
    }
    if interactive_shell != 0 && interactive != 0
        && (sig == SIGHUP as c_int || sig == SIGTERM as c_int)
        && remember_on_history != 0
    {
        maybe_save_shell_history();
    }
    if this_shell_builtin
        == Some(read_builtin as unsafe extern "C" fn(*mut WordList) -> c_int)
    {
        read_tty_cleanup();
    }
    if sig == SIGHUP as c_int
        && (interactive != 0
            || subshell_environment & (SUBSHELL_COMSUB as c_int | SUBSHELL_PROCSUB as c_int) != 0)
    {
        hangup_all_jobs();
    }
    if subshell_environment & (SUBSHELL_COMSUB as c_int | SUBSHELL_PROCSUB as c_int)
        == 0 
    {
        end_job_control();
    }
    unlink_all_fifos();
    procsub_clear();

    funcnest = 0 as c_int;
    breaking = funcnest;
    continuing = breaking;
    loop_level = continuing;

    wait_intr_flag = 0 as c_int;
    return_catch_flag = wait_intr_flag;
    comsub_ignore_return = return_catch_flag;
    executing_list = comsub_ignore_return;

    run_exit_trap();

    restore_sigmask();

    set_signal_handler(sig, None);

    kill(getpid(), sig);

    if dollar_dollar_pid != 1 as c_int {
        exit(128 as c_int + sig);
    }
    sigprocmask(SIG_SETMASK as c_int, 0 as *mut c_void as *mut sigset_t, &mut mask);
    core = 0 as c_int;
    i = core;
    while (i as libc::c_ulong) < TERMSIGS_LENGTH!()
    {
        set_signal_handler(XSIG!(i), None);
        sigdelset(&mut mask, XSIG!(i));
        if sig == XSIG!(i) {
            core = XCOREDUMP!(i);
        }
        i += 1;
    }
    sigprocmask(SIG_SETMASK as c_int, &mut mask, 0 as *mut c_void as *mut sigset_t);
    if core != 0 {
        ::std::ptr::write_volatile(
            (0 as *mut c_void as *mut libc::c_ulong),
            (0xdead0000 as libc::c_uint).wrapping_add(sig as libc::c_uint)
                as libc::c_ulong,
        );
    }
    exit(128 as c_int + sig);
}

#[no_mangle]
pub unsafe extern "C" fn sigint_sighandler(mut sig: c_int) {
    if interrupt_state == 0 as c_int {
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
        interrupt_immediately = 0 as c_int;
        set_exit_status(128 as c_int + sig);
        throw_to_top_level();
    } else if rl_readline_state & 0x8000 as c_int as libc::c_ulong != 0 {
        bashline_set_event_hook();
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigwinch_sighandler(mut sig: c_int) {
    sigwinch_received = 1;
}

#[no_mangle]
pub unsafe extern "C" fn set_sigwinch_handler() {
    old_winch = set_signal_handler(
        SIGWINCH as c_int, 
        old_winch);
}

#[no_mangle]
pub unsafe extern "C" fn unset_sigwinch_handler() {
    set_signal_handler(SIGWINCH as c_int, old_winch);
}

#[no_mangle]
pub unsafe extern "C" fn sigterm_sighandler(mut sig: c_int) {
    sigterm_received = 1;
}

#[no_mangle]
pub unsafe extern "C" fn set_signal_handler(
    mut sig: c_int,
    mut handler: SigHandler,
) -> SigHandler{
    let mut act: sigaction = sigaction {
        __sigaction_handler: sigaction__bindgen_ty_1 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut oact: sigaction = sigaction {
        __sigaction_handler: sigaction__bindgen_ty_1 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };

    act.__sigaction_handler.sa_handler = handler;
    
    act.sa_flags = 0 ;
    
    if sig == SIGCHLD as c_int {
        act.sa_flags |= SA_RESTART as c_int;
    }
    if sig == SIGWINCH as c_int {
        act.sa_flags |= SA_RESTART as c_int;
    }
    if sig == SIGTERM as c_int
        && handler
            == Some(sigterm_sighandler)
    {
        act.sa_flags |= SA_RESTART as c_int;
    }
    sigemptyset(&mut act.sa_mask);
    sigemptyset(&mut oact.sa_mask);
    if sigaction(sig, &mut act, &mut oact) == 0 as c_int {
        return oact.__sigaction_handler.sa_handler
    } else {
        return None
    };
}