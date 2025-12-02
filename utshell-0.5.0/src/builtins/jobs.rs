/*
 * SPDX-FileCopyrightText: 2025 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: GPL-2.0-or-later
 */
use super::help::builtin_help;
use crate::builtins::bashgetopt::{internal_getopt, reset_internal_getopt};
use crate::builtins::common::{builtin_usage, get_job_spec, sh_badjob};
use crate::copycmd::copy_word_list;
use crate::dispose_cmd::dispose_command;
use crate::execute_cmd::execute_command;
use crate::general::legal_number;
use crate::jobs::{
    delete_all_jobs, delete_job, get_job_by_pid, list_all_jobs, list_one_job, list_running_jobs,
    list_stopped_jobs, nohup_all_jobs, nohup_job,
};
use crate::make_cmd::make_bare_simple_command;
use crate::src_common::*;
use crate::unwind_prot::{add_unwind_protect, begin_unwind_frame, discard_unwind_frame};

#[no_mangle]
pub fn execute_list_with_replacements(list: *mut WordList) -> i32 {
    unsafe {
        let mut l: *mut WordList = list;
        let mut job: i32;
        let result: i32;
        let command: *mut COMMAND;

        /* First do the replacement of job specifications with pids. */
        while l != std::ptr::null_mut() {
            let lchar: char = char::from((*(*(*l).word).word) as u8);
            if lchar == '%'
            /* we have a winner */
            {
                job = get_job_spec(l);
                /* A bad job spec is not really a job spec! Pass it through. */
                if INVALID_JOB!(job) {
                    continue;
                }

                libc::free((*(*l).word).word as *mut libc::c_void);
                (*(*(*l).word).word) = (*get_job_by_jid!(job)).pgrp as libc::c_char;
            }
            l = (*l).next;
        }

        let c_str_jobs_builtin = CString::new("jobs_builtin").unwrap();
        /* Next make a new simple command and execute it. */
        begin_unwind_frame(c_str_jobs_builtin.as_ptr() as *mut libc::c_char);

        command = make_bare_simple_command();
        (*((*command).value.Simple)).words = copy_word_list(list);
        (*((*command).value.Simple)).redirects = std::ptr::null_mut();
        (*command).flags |= CMD_INHIBIT_EXPANSION!();
        (*((*command).value.Simple)).flags |= CMD_INHIBIT_EXPANSION!();

        add_unwind_protect(
            std::mem::transmute::<fn(command: *mut COMMAND), Option<Function>>(dispose_command),
            command as *mut libc::c_char,
        );
        result = execute_command(command);
        dispose_command(command);
        discard_unwind_frame(c_str_jobs_builtin.as_ptr() as *mut libc::c_char);
        return result;
    }
}

#[no_mangle]
pub fn jobs_builtin(mut list: *mut WordList) -> i32 {
    let mut form: i32;
    let mut execute: i32 = 0;
    let mut state: i32;
    let mut opt: i32;
    let mut any_failed: i32 = 0;
    let mut job: i32;

    let mut set: nix::sys::signal::SigSet = nix::sys::signal::SigSet::empty();
    let mut oset: nix::sys::signal::SigSet = nix::sys::signal::SigSet::empty();
    // let mut set: sigset_t = sigset_t { __val: [0; 16] };
    // let mut oset: sigset_t = sigset_t { __val: [0; 16] };

    form = JLIST_STANDARD!();
    state = JSTATE_ANY!();

    unsafe {
        reset_internal_getopt();

        let c_str_lpnxrs = CString::new("lpnxrs").unwrap(); // from a &str, creates a new allocation

        opt = internal_getopt(list, c_str_lpnxrs.as_ptr() as *mut libc::c_char);
        while opt != -1 {
            let opt_har: char = opt as u8 as char;
            match opt_har {
                'l' => {
                    form = JLIST_LONG!();
                }
                'p' => {
                    form = JLIST_PID_ONLY!();
                }
                'n' => {
                    form = JLIST_CHANGED_ONLY!();
                }
                'x' => {
                    if form != JLIST_STANDARD!() {
                        let c_str_err = CString::new("no other options allowed with `-x'").unwrap(); // from a &str, creates a new allocation
                        builtin_error(c_str_err.as_ptr());
                        return EXECUTION_FAILURE!();
                    }
                    execute += 1;
                }
                'r' => {
                    state = JSTATE_RUNNING!();
                }
                's' => {
                    state = JSTATE_STOPPED!();
                }
                _ => {
                    if opt == -99 {
                        builtin_help();
                        return EX_USAGE;
                    }
                    builtin_usage();
                    return EX_USAGE;
                }
            }
            opt = internal_getopt(list, c_str_lpnxrs.as_ptr() as *mut libc::c_char);
        }

        list = loptend;

        if execute != 0 {
            return execute_list_with_replacements(loptend);
        }

        if loptend == std::ptr::null_mut() {
            if state == JSTATE_ANY!() {
                list_all_jobs(form);
            } else if state == JSTATE_RUNNING!() {
                list_running_jobs(form);
            } else if state == JSTATE_STOPPED!() {
                list_stopped_jobs(form);
            }
            return EXECUTION_SUCCESS!();
        }

        while list != std::ptr::null_mut() {
            BLOCK_CHILD_1!(Some(&mut set), Some(&mut oset));
            job = get_job_spec(list);

            if (job == NO_JOB!())
                || jobs == std::ptr::null_mut()
                || get_job_by_jid!(job) == std::ptr::null_mut()
            {
                sh_badjob((*((*list).word)).word);
                any_failed += 1;
            } else if job != DUP_JOB!() {
                list_one_job(0 as *mut JOB, form, 0, job);
            }

            UNBLOCK_CHILD_1!(Some(&oset));

            list = (*list).next;
        }
        if any_failed != 0 {
            return EXECUTION_FAILURE!();
        } else {
            return EXECUTION_SUCCESS!();
        }
    }
}

#[no_mangle]
pub fn disown_builtin(list: *mut WordList) -> libc::c_int {
    let mut opt: i32;
    let mut job: i32 = 0;
    let mut retval: i32;
    let mut nohup_only: i32 = 0;
    let mut running_jobs: i32 = 0;
    let mut all_jobs: i32 = 0;

    let mut set: nix::sys::signal::SigSet = nix::sys::signal::SigSet::empty();
    let mut oset: nix::sys::signal::SigSet = nix::sys::signal::SigSet::empty();
    // let mut set: sigset_t = sigset_t { __val: [0; 16] };
    // let mut oset: sigset_t = sigset_t { __val: [0; 16] };
    let mut pid_value: libc::c_long = 0;
    unsafe {
        reset_internal_getopt();
        let c_str_ahr = CString::new("ahr").unwrap(); // from a &str, creates a new allocation
        opt = internal_getopt(list, c_str_ahr.as_ptr() as *mut libc::c_char);
        while opt != -1 {
            let opt_char: char = opt as u8 as char;
            match opt_char {
                'a' => {
                    all_jobs = 1;
                }
                'h' => {
                    nohup_only = 1;
                }
                'r' => {
                    running_jobs = 1;
                }
                _ => {
                    if opt == -99 {
                        builtin_help();
                        return EX_USAGE;
                    }
                    builtin_usage();
                    return EX_USAGE;
                }
            }
            opt = internal_getopt(list, c_str_ahr.as_ptr() as *mut libc::c_char);
        }

        retval = EXECUTION_SUCCESS!();

        /* `disown -a' or `disown -r' */
        if loptend == std::ptr::null_mut() && (all_jobs != 0 || running_jobs != 0) {
            if nohup_only != 0 {
                nohup_all_jobs(running_jobs);
            } else {
                delete_all_jobs(running_jobs);
            }
            return EXECUTION_SUCCESS!();
        }
        BLOCK_CHILD_1!(Some(&mut set), Some(&mut oset));
        if loptend != std::ptr::null_mut()
            && legal_number((*(*loptend).word).word, &mut pid_value) != 0
            && pid_value == pid_value
        {
            job = get_job_by_pid(pid_value as i32, 0, 0 as *mut *mut PROCESS);
        } else {
            job = get_job_spec(loptend);
        }
        if (job == NO_JOB!()) || (jobs == std::ptr::null_mut()) || (INVALID_JOB!(job)) {
            if loptend != std::ptr::null_mut() {
                sh_badjob((*(*loptend).word).word);
            } else {
                sh_badjob(CString::new("current").unwrap().as_ptr() as *mut libc::c_char);
            }
            retval = EXECUTION_FAILURE!();
        } else if nohup_only != 0 {
            nohup_job(job);
        } else {
            delete_job(job, 1);
        }

        UNBLOCK_CHILD_1!(Some(&oset));

        if loptend != std::ptr::null_mut() {
            let mut loptendt = *loptend;
            while loptendt.next != std::ptr::null_mut() {
                loptendt = *loptendt.next;
                BLOCK_CHILD_1!(Some(&mut set), Some(&mut oset));
                if legal_number((*loptendt.word).word, &mut pid_value) != 0
                    && pid_value == pid_value
                {
                    job = get_job_by_pid(pid_value as i32, 0, 0 as *mut *mut PROCESS);
                } else {
                    get_job_spec(&mut loptendt);
                }
                if job == NO_JOB!() || jobs != std::ptr::null_mut() || INVALID_JOB!(job) {
                    sh_badjob((*loptendt.word).word);
                    retval = EXECUTION_FAILURE!();
                } else if nohup_only != 0 {
                    nohup_job(job);
                } else {
                    delete_job(job, 1);
                }
                UNBLOCK_CHILD_1!(Some(&oset));
            }
        }
        return retval;
    }
}

fn cmd_name() -> *const u8 {
    return b"jobs" as *const u8;
}
