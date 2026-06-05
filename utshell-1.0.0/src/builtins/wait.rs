use crate::arrayfunc::valid_array_reference;
use crate::builtins::bashgetopt::{internal_getopt, reset_internal_getopt};
use crate::builtins::common::{
    builtin_unbind_variable, builtin_usage, get_job_spec, sh_badjob, sh_invalidid,
};
use crate::builtins::help::builtin_help;
use crate::general::{legal_identifier, legal_number};
use crate::jobs::{
    get_job_by_pid, wait_for_any_job, wait_for_background_pids, wait_for_job, wait_for_single_pid,
    wait_sigint_cleanup,
};
use crate::src_common::*;
use crate::trap::{first_pending_trap, next_pending_trap};
use crate::variables::bind_var_to_int;

// include!("./signal.rs");

#[macro_export]
macro_rules! WAIT_RETURN {
    ($s:expr) => {{
        wait_signal_received = 0;
        wait_intr_flag = 0;
        $s
    }};
}

#[no_mangle]
pub fn wait_builtin(mut list: *mut WordList) -> i32 {
    let mut status: i32;
    let code: i32;
    let mut opt: i32;
    let mut nflag: i32;
    let mut wflags: i32;
    let mut vname: *mut libc::c_char;
    // let pidvar: *mut SHELL_VAR;
    let mut pstat: procstat = procstat { pid: 0, status: 0 };

    nflag = 0;
    wflags = 0;
    vname = std::ptr::null_mut();
    // pidvar = std::ptr::null_mut();

    reset_internal_getopt();
    let c_fnp = std::ffi::CString::new("fnp:").unwrap();

    loop {
        opt = internal_getopt(list, c_fnp.as_ptr() as *mut libc::c_char);
        if opt == -1 {
            break;
        }
        let optu8 = opt as u8;
        let opt_char = char::from(optu8);

        match opt_char {
            'n' => nflag = 1,
            'f' => wflags |= JWAIT_FORCE!(),
            'p' => vname = unsafe { list_optarg },
            _ => {
                if opt == -99 {
                    builtin_help();
                    return EX_USAGE as libc::c_int;
                }
                builtin_usage();
                return EX_USAGE as libc::c_int;
            }
        }
    }

    list = unsafe { loptend };
    /* Sanity-check variable name if -p supplied. */
    if vname != std::ptr::null_mut() {
        let arrayflags: i32;
        if unsafe { assoc_expand_once != 0 } {
            arrayflags = VA_NOEXPAND!() | VA_ONEWORD!();
        } else {
            arrayflags = 0;
        }

        if legal_identifier(vname) == 0 && valid_array_reference(vname, arrayflags) == 0 {
            sh_invalidid(vname);
            return unsafe { WAIT_RETURN!(EXECUTION_FAILURE!()) };
        }

        if builtin_unbind_variable(vname) == -2 {
            return unsafe { WAIT_RETURN!(EXECUTION_FAILURE!()) };
        }
    }
    /* POSIX.2 says:  When the shell is waiting (by means of the wait utility)
    for asynchronous commands to complete, the reception of a signal for
    which a trap has been set shall cause the wait utility to return
    immediately with an exit status greater than 128, after which the trap
    associated with the signal shall be taken.

    We handle SIGINT here; it's the only one that needs to be treated
    specially (I think), since it's handled specially in {no,}jobs.c. */

    unsafe { wait_intr_flag = 1 };

    code = unsafe { __sigsetjmp(wait_intr_buf.as_mut_ptr(), 1) };

    if code != 0 {
        unsafe {
            last_command_exit_signal = wait_signal_received;
            status = 128 + wait_signal_received;
            wait_sigint_cleanup();
            return WAIT_RETURN!(status);
        }
    }

    opt = first_pending_trap();

    /* We special case SIGCHLD when not in posix mode because we don't break
    out of the wait even when the signal is trapped; we run the trap after
    the wait completes. See how it's handled in jobs.c:waitchld(). */

    if opt == (SIGCHLD as i32) && unsafe { posixly_correct == 0 } {
        opt = next_pending_trap(opt + 1);
    }
    if opt != -1 {
        unsafe {
            last_command_exit_signal = opt;
            wait_signal_received = opt;
            status = opt + 128;
            return WAIT_RETURN!(status);
        }
    }

    /* We support jobs or pids.
    wait <pid-or-job> [pid-or-job ...] */
    if nflag != 0 {
        if list != std::ptr::null_mut() {
            opt = set_waitlist(list);
            if opt == 0 {
                return unsafe { WAIT_RETURN!(127) };
            }
            wflags |= JWAIT_WAITING!();
        }

        status = wait_for_any_job(wflags, &mut pstat);
        if vname != std::ptr::null_mut() && status >= 0 {
            bind_var_to_int(vname, pstat.pid as intmax_t);
        }

        if status < 0 {
            status = 127;
        }
        if list != std::ptr::null_mut() {
            unset_waitlist();
        }
        return unsafe { WAIT_RETURN!(status) };
    }

    /* But wait without any arguments means to wait for all of the shell's
    currently active background processes. */
    if list == std::ptr::null_mut() {
        wait_for_background_pids(&mut pstat);
        if vname != std::ptr::null_mut() {
            bind_var_to_int(vname, pstat.pid as intmax_t);
        }
        return unsafe { WAIT_RETURN!(EXECUTION_SUCCESS!()) };
    }

    status = EXECUTION_SUCCESS!();
    while list != std::ptr::null_mut() {
        let pid: pid_t;
        let w: *mut libc::c_char;
        let mut pid_value: intmax_t = 0;

        w = unsafe { (*(*list).word).word };
        if unsafe { DIGIT!(*w) } {
            if legal_number(w, &mut pid_value) != 0 && pid_value == (pid_value as pid_t) as i64 {
                pid = pid_value as pid_t;
                status = wait_for_single_pid(pid, wflags | JWAIT_PEROOR!());
                pstat.pid = pid;
                pstat.status = status as libc::c_short;
            } else {
                sh_badjob(w);
                pstat.pid = NO_PID!();
                pstat.status = 127;
                return unsafe { WAIT_RETURN!(EXECUTION_FAILURE!()) };
            }
        } else if unsafe { *w != 0 && *w == '%' as libc::c_char } {
            /* Must be a job spec.  Check it out. */
            let job: i32;
            let mut set: sigset_t = sigset_t { __val: [0; 16] };
            let mut oset: sigset_t = sigset_t { __val: [0; 16] };

            BLOCK_CHILD_1!(&mut set, &mut oset);
            job = get_job_spec(list);

            if unsafe { INVALID_JOB!(job) == true } {
                if job != DUP_JOB!() {
                    unsafe {
                        sh_badjob((*(*list).word).word);
                    }
                }
                UNBLOCK_CHILD_1!(&mut oset);
                status = 127; /* As per Posix.2, section 4.70.2 */
                pstat.pid = NO_PID!();
                pstat.status = status as libc::c_short;
                list = unsafe { (*list).next };
                continue;
            }

            /* Job spec used.  Wait for the last pid in the pipeline. */
            UNBLOCK_CHILD_1!(&mut oset);
            status = wait_for_job(job, wflags, &mut pstat)
        } else {
            sh_badjob(w);
            pstat.pid = NO_PID!();
            pstat.status = 127;
            status = EXECUTION_FAILURE!();
        }

        /* Don't waste time with a longjmp. */
        unsafe {
            if wait_signal_received != 0 {
                last_command_exit_signal = wait_signal_received;
                status = 128 + wait_signal_received;
                wait_sigint_cleanup();
                return WAIT_RETURN!(status);
            }
        }

        list = unsafe { (*list).next };
    }

    return unsafe { WAIT_RETURN!(status) };
}

#[no_mangle]
fn set_waitlist(list: *mut WordList) -> i32 {
    let mut set: sigset_t = sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = sigset_t { __val: [0; 16] };
    let mut job: i32;
    let mut _r: i32;
    let mut njob: i32;
    let mut pid: intmax_t = 0;
    let mut l: *mut WordList;

    BLOCK_CHILD_1!(&mut set, &mut oset);
    njob = 0;

    l = list;
    while l != std::ptr::null_mut() {
        job = NO_JOB!();

        if l != std::ptr::null_mut()
            && unsafe { legal_number((*(*l).word).word, &mut pid) != 0 }
            && pid == (pid as pid_t) as i64
        {
            job = get_job_by_pid(pid as pid_t, 0, std::ptr::null_mut());
        } else {
            get_job_spec(l);
        }

        if unsafe { job == NO_JOB!() || jobs == std::ptr::null_mut() || INVALID_JOB!(job) } {
            unsafe {
                sh_badjob((*(*l).word).word);
            }
            continue;
        }

        /* We don't check yet to see if one of the desired jobs has already
        terminated, but we could. We wait until wait_for_any_job(). This
        has the advantage of validating all the arguments. */
        unsafe {
            if (*get_job_by_jid!(job)).flags & J_WAITING!() == 0 {
                njob = njob + 1;
                (*get_job_by_jid!(job)).flags |= J_WAITING!();
            }

            l = (*l).next;
        }
    }
    UNBLOCK_CHILD_1!(&mut oset);

    return njob;
}

/* Clean up after a call to wait -n jobs */
#[no_mangle]
fn unset_waitlist() {
    let mut set: sigset_t = sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = sigset_t { __val: [0; 16] };

    BLOCK_CHILD_1!(&mut set, &mut oset);
    unsafe {
        for i in 0..js.j_jobslots {
            if get_job_by_jid!(i) != std::ptr::null_mut()
                && (*get_job_by_jid!(i)).flags & J_WAITING!() != 0
            {
                (*get_job_by_jid!(i)).flags &= !J_WAITING!();
            }
        }
    }
    UNBLOCK_CHILD_1!(&mut oset);
}
