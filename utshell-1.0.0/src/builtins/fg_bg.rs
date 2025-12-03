//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

use std::{ffi::CString, ops::Add};

use crate::builtins::common::{get_job_spec, no_options, sh_badjob, sh_nojobs};
use crate::jobs::start_job;
use crate::src_common::*;

/* How to bring a job into the foreground. */
#[no_mangle]
pub fn fg_builtin(list: *mut WordList) -> i32 {
    let fg_bit: i32;
    unsafe {
        CHECK_HELPOPT!(list);

        if job_control == 0 {
            sh_nojobs(0 as *mut libc::c_char);
            return EXECUTION_FAILURE!();
        }

        if no_options(list) != 0 {
            return EX_USAGE!();
        }

        /* If the last arg on the line is '&', then start this job in the
        background.  Else, fg the job. */

        if loptend == std::ptr::null_mut() {
            return fg_bg(loptend, 1);
        } else {
            let mut t: WordList = *loptend;
            while t.next != std::ptr::null_mut() {
                t = *(t.next);
            }
            let cstr: &std::ffi::CStr = std::ffi::CStr::from_ptr((*(t.word)).word);
            let mut isfg: bool = char::from(cstr.to_bytes()[0]) == '&';
            isfg = isfg && char::from(cstr.to_bytes()[1]) == '\0';
            isfg = isfg == false;
            if isfg {
                fg_bit = 1;
            } else {
                fg_bit = 0;
            }
            return fg_bg(loptend, fg_bit);
        }
    }
}

/* How to put a job into the background. */
#[no_mangle]
pub fn bg_builtin(list: *mut WordList) -> i32 {
    let mut r: i32;
    unsafe {
        CHECK_HELPOPT!(list);

        if job_control == 0 {
            sh_nojobs(0 as *mut libc::c_char);
            return EXECUTION_FAILURE!();
        }

        if no_options(list) != 0 {
            return EX_USAGE!();
        }

        /* This relies on the fact that fg_bg() takes a WordList *, but only acts
        on the first member (if any) of that list. */
        r = EXECUTION_SUCCESS!();

        if fg_bg(loptend, 0) == EXECUTION_FAILURE!() {
            r = EXECUTION_FAILURE!();
        }

        if loptend != std::ptr::null_mut() {
            let mut t: WordList = *loptend;
            while t.next != std::ptr::null_mut() {
                if fg_bg(&mut t, 0) == EXECUTION_FAILURE!() {
                    r = EXECUTION_FAILURE!();
                }
                t = *(t.next);
            }
            return r;
        } else {
            return r;
        }
    }
}

/* How to put a job into the foreground/background. */
#[no_mangle]
pub fn fg_bg(list: *mut WordList, foreground: i32) -> i32 {
    let mut set: sigset_t = sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = sigset_t { __val: [0; 16] };
    let job: i32;
    let status: i32;
    let mut old_async_pid: i32 = 0;
    let j: *mut JOB;

    unsafe {
        BLOCK_CHILD_1!(&mut set, &mut oset);
        job = get_job_spec(list);

        if INVALID_JOB!(job) {
            if job != DUP_JOB!() {
                if list != std::ptr::null_mut() {
                    sh_badjob((*(*list).word).word);
                } else {
                    let c_str_current = CString::new("current").unwrap(); // from a &str, creates a new allocation
                    sh_badjob(c_str_current.as_ptr() as *mut libc::c_char);
                }
            }

            UNBLOCK_CHILD_1!(&mut oset);
            return EXECUTION_FAILURE!();
        }

        j = get_job_by_jid!(job);
        /* Or if j->pgrp == shell_pgrp. */
        if !IS_JOBCONTROL!(job) {
            let jobNum: i32 = job + 1;
            builtin_error(
                String::from("job ")
                    .add(&jobNum.to_string())
                    .add(&String::from("started without job control").to_string())
                    .as_ptr() as *const libc::c_char,
            );
            UNBLOCK_CHILD_1!(&mut oset);
            return EXECUTION_FAILURE!();
        }

        if foreground == 0 {
            old_async_pid = i32::from(last_asynchronous_pid);
            last_asynchronous_pid = i32::from((*j).pgrp); /* As per Posix.2 5.4.2 */
        }

        status = start_job(job, foreground);

        if status >= 0 {
            /* win: */
            UNBLOCK_CHILD_1!(&mut oset);
            if foreground != 0 {
                return status;
            } else {
                return EXECUTION_SUCCESS!();
            }
        } else {
            if foreground == 0 {
                last_asynchronous_pid = i32::from(old_async_pid);
            }

            UNBLOCK_CHILD_1!(&mut oset);
            return EXECUTION_FAILURE!();
        }
    }
}
