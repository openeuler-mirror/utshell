//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later
use crate::builtins::{
    common::{get_working_directory, the_current_working_directory},
    evalfile::sourcelevel,
    wait::wait_builtin,
};
use crate::dispose_cmd::dispose_command;
use crate::error::get_name_for_error;
use crate::execute_cmd::{coproc_pidchk, coproc_reap};
use crate::flags::change_flag;
use crate::general::{move_to_high_fd, polite_directory_format, sh_closepipe};
use crate::input::sync_buffered_stream;
use crate::list::list_reverse;
use crate::readline::rl_readline_state;
use crate::sig::{
    restore_sigmask, set_signal_handler, sigint_sighandler, termsig_handler, throw_to_top_level,
};
use crate::src_common::*;
use crate::subst::{find_procsub_child, set_procsub_status};
use crate::trap::{
    get_original_signal, initialize_traps, maybe_call_trap_handler, maybe_set_sigchld_trap,
    queue_sigchld_trap, set_impossible_sigchld_trap, set_original_signal, signal_in_progress,
    signal_is_hard_ignored, signal_is_trapped, signal_name,
};
use crate::trap::{trap_handler, trap_to_sighandler};
use crate::unwind_prot::{
    add_unwind_protect, begin_unwind_frame, run_unwind_frame, unwind_protect_mem,
};
use crate::utshell::unset_bash_input;
use crate::variables::{get_string_value, set_pipestatus_array};
use crate::version::shell_compatibility_level;
use crate::y_tab::line_number;
use std::convert::TryInto;

#[no_mangle]
pub fn UNQUEUE_SIGCHLD(os: libc::c_int) {
    unsafe {
        queue_sigchld -= 1;
        if queue_sigchld == 0 && os != sigchld {
            queue_sigchld = 1;
            waitchld(-1, 0);
            queue_sigchld = 0;
        }
    }
}

#[no_mangle]
pub fn PSTOPPED(p: *mut PROCESS) -> libc::c_int {
    unsafe {
        if (*p).status & 0xff == 0x7f {
            return 1;
        } else {
            return 0;
        }
    }
}

#[no_mangle]
pub fn BLOCK_CHILD(nvar: *mut sigset_t, ovar: *mut sigset_t) {
    unsafe {
        sigemptyset(nvar);
        sigaddset(nvar, SIGCHLD as libc::c_int);
        sigemptyset(ovar);
        sigprocmask(SIG_BLOCK as libc::c_int, nvar, ovar);
    }
}

#[no_mangle]
pub fn UNBLOCK_CHILD(over: *const sigset_t) {
    unsafe {
        sigprocmask(
            SIG_SETMASK as libc::c_int,
            over,
            0 as *mut c_void as *mut sigset_t,
        );
    }
}

/* 函数部分重构 */
#[no_mangle]
pub fn init_job_stats() {
    unsafe {
        js = zerojs;
    }
}

fn current_working_directory() -> *mut libc::c_char {
    unsafe {
        let mut dir: *mut libc::c_char;
        let mut d: [libc::c_char; PATH_MAX as usize] = [0; PATH_MAX as usize];

        dir = get_string_value(b"PWD\0" as *const u8 as *const libc::c_char);

        if dir.is_null() && !the_current_working_directory.is_null() && no_symbolic_links != 0 {
            dir = the_current_working_directory;
        }

        if dir.is_null() {
            dir = getcwd(
                d.as_mut_ptr(),
                (::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
                    .try_into()
                    .unwrap(),
            );
            if !dir.is_null() {
                dir = d.as_mut_ptr();
            }
        }

        if dir.is_null() {
            return b"<unknown>\0" as *const u8 as *mut libc::c_char;
        } else {
            return dir;
        }
    }
}

fn job_working_directory() -> *mut libc::c_char {
    unsafe {
        let mut dir: *mut libc::c_char;

        dir = get_string_value(b"PWD\0" as *const u8 as *const libc::c_char);
        dir = get_working_directory(b"job-working-directory\0" as *const u8 as *mut libc::c_char);
        if !dir.is_null() {
            return savestring!(dir);
        }

        return savestring!(b"<unknown>\0" as *const u8 as *const libc::c_char);
    }
}

#[no_mangle]
pub fn making_children() {
    unsafe {
        if already_making_children != 0 {
            return;
        }

        already_making_children = 1;
        start_pipeline();
    }
}

#[no_mangle]
pub fn stop_making_children() {
    unsafe {
        already_making_children = 0;
    }
}

#[no_mangle]
pub fn cleanup_the_pipeline() {
    unsafe {
        let disposer: *mut PROCESS;
        let mut set: sigset_t = __sigset_t { __val: [0; 16] };
        let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

        BLOCK_CHILD(&mut set, &mut oset);
        disposer = the_pipeline;
        the_pipeline = 0 as *mut PROCESS;
        UNBLOCK_CHILD(&mut oset);

        if !disposer.is_null() {
            discard_pipeline(disposer);
        }
    }
}

#[no_mangle]
pub fn discard_last_procsub_child() {
    let disposer: *mut PROCESS;
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };
    unsafe {
        BLOCK_CHILD(&mut set, &mut oset);
        disposer = last_procsub_child;
        last_procsub_child = 0 as *mut PROCESS;
        UNBLOCK_CHILD(&mut oset);

        if !disposer.is_null() {
            discard_pipeline(disposer);
        }
    }
}

fn alloc_pipeline_saver() -> *mut pipeline_saver {
    unsafe {
        let ret: *mut pipeline_saver;

        ret = malloc(::std::mem::size_of::<pipeline_saver>() as usize) as *mut pipeline_saver;

        (*ret).pipeline = 0 as *mut process;
        (*ret).next = 0 as *mut pipeline_saver;

        return ret;
    }
}

#[no_mangle]
pub fn save_pipeline(clear: libc::c_int) {
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };
    let saver: *mut pipeline_saver;
    unsafe {
        BLOCK_CHILD(&mut set, &mut oset);
        saver = alloc_pipeline_saver();
        (*saver).pipeline = the_pipeline;
        (*saver).next = saved_pipeline;
        saved_pipeline = saver;

        if clear != 0 {
            the_pipeline = 0 as *mut PROCESS;
        }
        saved_already_making_children = already_making_children;
        UNBLOCK_CHILD(&mut oset);
    }
}

#[no_mangle]
pub fn restore_pipeline(discard: libc::c_int) -> *mut PROCESS {
    unsafe {
        let old_pipeline: *mut PROCESS;
        let mut set: sigset_t = __sigset_t { __val: [0; 16] };
        let mut oset: sigset_t = __sigset_t { __val: [0; 16] };
        let saver: *mut pipeline_saver;

        BLOCK_CHILD(&mut set, &mut oset);
        old_pipeline = the_pipeline;
        the_pipeline = (*saved_pipeline).pipeline;
        saver = saved_pipeline;
        saved_pipeline = (*saved_pipeline).next;
        free(saver as *mut c_void);
        already_making_children = saved_already_making_children;
        UNBLOCK_CHILD(&mut oset);

        if discard != 0 && !old_pipeline.is_null() {
            discard_pipeline(old_pipeline);
            return 0 as *mut PROCESS;
        }
        return old_pipeline;
    }
}

#[no_mangle]
pub fn start_pipeline() {
    unsafe {
        if !the_pipeline.is_null() {
            cleanup_the_pipeline();
            if pipeline_pgrp != shell_pgrp {
                pipeline_pgrp = 0 as pid_t;
            }
            sh_closepipe(pgrp_pipe.as_mut_ptr());
        }

        if job_control != 0 {
            if libc::pipe(pgrp_pipe.as_mut_ptr()) == -1 {
                sys_error(b"start_pipeline: pgrp pipe\0" as *const u8 as *const libc::c_char);
            }
        }
    }
}

#[no_mangle]
pub fn stop_pipeline(async_0: libc::c_int, deferred: *mut COMMAND) -> libc::c_int {
    unsafe {
        let mut i: libc::c_int;
        let mut j: libc::c_int;
        let newjob: *mut JOB;
        let mut set: sigset_t = __sigset_t { __val: [0; 16] };
        let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

        BLOCK_CHILD(&mut set, &mut oset);
        sh_closepipe(pgrp_pipe.as_mut_ptr());
        cleanup_dead_jobs();

        if js.j_jobslots == 0 {
            js.j_jobslots = JOB_SLOTS as libc::c_int;
            jobs =
                malloc((js.j_jobslots * (std::mem::size_of::<*mut JOB>() as libc::c_int)) as usize)
                    as *mut *mut JOB;

            i = 0 as libc::c_int;
            while i < js.j_jobslots {
                (*jobs.offset(i as isize)) = std::ptr::null_mut();

                i += 1;
            }
            js.j_njobs = 0 as libc::c_int;
            js.j_lastj = js.j_njobs;
            js.j_firstj = js.j_lastj;
        }

        if interactive != 0 {
            i = js.j_jobslots;
            while i != 0 {
                let temp = i - 1;
                if !(*jobs.offset(temp as isize)).is_null() {
                    break;
                }
                i -= 1;
            }
        } else {
            if js.j_lastj != 0 {
                i = js.j_lastj + 1;
            } else {
                i = js.j_lastj;
            }
            while i < js.j_jobslots {
                if (*jobs.offset(i as isize)).is_null() {
                    break;
                }
                i += 1;
            }
        }

        if (interactive_shell == 0 || subshell_environment != 0)
            && i == js.j_jobslots
            && js.j_jobslots >= MAX_JOBS_IN_ARRAY as i32
        {
            i = compact_jobs_list(0 as libc::c_int);
        }

        if i == js.j_jobslots {
            js.j_jobslots += JOB_SLOTS as i32;
            jobs = realloc(
                jobs as *mut c_void,
                (js.j_jobslots * std::mem::size_of::<*mut JOB>() as libc::c_int) as usize,
            ) as *mut *mut JOB;

            j = i;
            while j < js.j_jobslots {
                (*jobs.offset(j as isize)) = 0 as *mut JOB;
                j += 1;
            }
        }

        if !the_pipeline.is_null() {
            let mut p: *mut PROCESS = 0 as *mut PROCESS;
            let mut any_running: libc::c_int = 0;
            let mut any_stopped: libc::c_int = 0;
            let mut n: libc::c_int = 0;

            newjob = malloc(std::mem::size_of::<JOB>() as libc::c_int as usize) as *mut JOB;

            n = 1 as libc::c_int;
            p = the_pipeline;
            while (*p).next != the_pipeline {
                n += 1;
                p = (*p).next;
            }

            (*p).next = 0 as *mut PROCESS;
            if !the_pipeline.is_null() && !((*the_pipeline).next).is_null() {
                (*newjob).pipe = list_reverse(the_pipeline as *mut GENERIC_LIST) as *mut PROCESS
            } else {
                (*newjob).pipe = the_pipeline;
            };

            p = (*newjob).pipe;
            while !((*p).next).is_null() {
                p = (*p).next;
            }

            (*p).next = (*newjob).pipe;

            the_pipeline = 0 as *mut PROCESS;
            (*newjob).pgrp = pipeline_pgrp;

            if pipeline_pgrp != shell_pgrp {
                pipeline_pgrp = 0;
            }

            (*newjob).flags = 0;

            if pipefail_opt != 0 {
                (*newjob).flags |= J_PIPEFAIL as i32;
            }

            if job_control != 0 {
                (*newjob).flags |= J_JOBCONTROL as i32;
            }

            p = (*newjob).pipe;
            any_running = 0;
            any_stopped = 0;

            loop {
                any_running |= PRUNNING!(p) as libc::c_int;
                any_stopped |= PSTOPPED(p);
                p = (*p).next;
                if !(p != (*newjob).pipe) {
                    break;
                }
            }

            (*newjob).state = (if any_running != 0 {
                JRUNNING as libc::c_int
            } else if any_stopped != 0 {
                JSTOPPED as libc::c_int
            } else {
                JDEAD as libc::c_int
            }) as JOB_STATE;

            (*newjob).wd = job_working_directory();
            (*newjob).deferred = deferred;

            (*newjob).j_cleanup =
                ::std::mem::transmute::<*mut libc::c_void, sh_vptrfunc_t>(0 as *mut libc::c_void);
            (*newjob).cleanarg = 0 as *mut c_void;

            *jobs.offset(i as isize) = newjob;

            if (*newjob).state == JDEAD as libc::c_int && (*newjob).flags & 0x1 as libc::c_int != 0
            {
                setjstatus(i);
            }
            if (*newjob).state == JDEAD as libc::c_int {
                js.c_reaped += n;
                js.j_ndead += 1;
            }
            js.c_injobs += n;
            js.j_lastj = i;
            js.j_njobs += 1;
        } else {
            newjob = 0 as *mut JOB;
        }

        if !newjob.is_null() {
            js.j_lastmade = newjob;
        }

        if async_0 != 0 {
            if !newjob.is_null() {
                (*newjob).flags &= !J_FOREGROUND as libc::c_int;
                (*newjob).flags |= J_ASYNC as libc::c_int;
                js.j_lastasync = newjob;
            }
            reset_current();
        } else {
            if !newjob.is_null() {
                (*newjob).flags |= J_FOREGROUND as libc::c_int;

                if job_control != 0
                    && (*newjob).pgrp != 0
                    && (subshell_environment & SUBSHELL_ASYNC as libc::c_int) == 0
                    && running_in_background == 0
                {
                    maybe_give_terminal_to(shell_pgrp, (*newjob).pgrp, 0);
                }
            }
        }

        stop_making_children();
        UNBLOCK_CHILD(&mut oset);
        return if !newjob.is_null() { i } else { js.j_current };
    }
}

#[no_mangle]
fn bgp_resize() {
    let mut nsize: ps_index_t = 0;
    let mut nsize_cur: ps_index_t = 0;
    let mut nsize_max: ps_index_t = 0;
    let mut psi: ps_index_t = 0;
    unsafe {
        if bgpids.nalloc == 0 {
            psi = 0 as libc::c_int;
            while psi < r_pidstat_table_SZ as libc::c_int {
                r_pidstat_table[psi as usize] = NO_PIDSTAT;
                psi += 1;
            }
            nsize = BGPIDS_TABLE_SZ as ps_index_t;
            bgpids.head = 0;
        } else {
            nsize = bgpids.nalloc;
        }

        nsize_max = TYPE_MAXIMUM!(ps_index_t);
        nsize_cur = js.c_childmax as ps_index_t;

        if nsize_cur < 0 {
            nsize_cur = MAX_CHILD_MAX as libc::c_int;
        }

        while nsize > 0 && nsize < nsize_cur {
            nsize <<= 1;
        }

        if nsize > nsize_max || nsize <= 0 {
            nsize = nsize_max;
        }

        if nsize > MAX_CHILD_MAX as libc::c_int {
            nsize_max = MAX_CHILD_MAX as libc::c_int;
            nsize = nsize_max;
        }

        if bgpids.nalloc < nsize_cur && bgpids.nalloc < nsize_max {
            bgpids.storage = realloc(
                bgpids.storage as *mut c_void,
                (nsize * std::mem::size_of::<pidstat>() as libc::c_int) as usize,
            ) as *mut pidstat;
            psi = bgpids.nalloc;
            while psi < nsize {
                (*(bgpids.storage).offset(psi as isize)).pid = -(1 as libc::c_int);
                psi += 1;
            }
            bgpids.nalloc = nsize;
        } else if bgpids.head >= bgpids.nalloc {
            bgpids.head = 0 as libc::c_int;
        }
    }
}

#[no_mangle]
fn bgp_getindex() -> ps_index_t {
    unsafe {
        if bgpids.nalloc < js.c_childmax as ps_index_t || bgpids.head >= bgpids.nalloc {
            bgp_resize();
        }
        pshash_delindex(bgpids.head);
        bgpids.head += 1;
        return bgpids.head;
    }
}

#[no_mangle]
fn pshash_getbucket(pid: pid_t) -> *mut ps_index_t {
    let mut hash: libc::c_ulong = 0;
    hash = (pid as libc::c_ulong).wrapping_mul(0x9e370001 as libc::c_ulong);
    unsafe {
        return &mut *r_pidstat_table
            .as_mut_ptr()
            .offset((hash % r_pidstat_table_SZ as u64) as isize) as *mut ps_index_t;
    }
}

#[no_mangle]
fn bgp_add(pid: pid_t, status: libc::c_int) -> *mut pidstat {
    unsafe {
        let mut bucket: *mut ps_index_t = 0 as *mut ps_index_t;
        let mut psi: ps_index_t = 0;
        let mut ps: *mut pidstat = 0 as *mut pidstat;
        bucket = pshash_getbucket(pid);
        psi = bgp_getindex();
        if psi == *bucket {
            (*(bgpids.storage).offset(psi as isize)).pid = -1;
            psi = bgp_getindex();
        }
        ps = &mut *(bgpids.storage).offset(psi as isize) as *mut pidstat;
        (*ps).pid = pid;
        (*ps).status = status as libc::c_short;
        (*ps).bucket_next = *bucket;
        (*ps).bucket_prev = -1;
        bgpids.npid += 1;
        if (*ps).bucket_next != -1 {
            (*(bgpids.storage).offset((*ps).bucket_next as isize)).bucket_prev = psi;
        }
        *bucket = psi;
        return ps;
    }
}

#[no_mangle]
fn pshash_delindex(psi: ps_index_t) {
    let mut ps: *mut pidstat = 0 as *mut pidstat;
    let mut bucket: *mut ps_index_t = 0 as *mut ps_index_t;
    unsafe {
        ps = &mut *(bgpids.storage).offset(psi as isize) as *mut pidstat;
        if (*ps).pid == NO_PID {
            return;
        }
        if (*ps).bucket_next != NO_PIDSTAT {
            (*(bgpids.storage).offset((*ps).bucket_next as isize)).bucket_prev = (*ps).bucket_prev;
        }
        if (*ps).bucket_prev != NO_PIDSTAT {
            (*(bgpids.storage).offset((*ps).bucket_prev as isize)).bucket_next = (*ps).bucket_next;
        } else {
            bucket = pshash_getbucket((*ps).pid);
            *bucket = (*ps).bucket_next;
        }
        (*ps).pid = NO_PID;
        (*ps).bucket_next = NO_PIDSTAT;
        (*ps).bucket_prev = NO_PIDSTAT;
    }
}

fn bgp_delete(pid: pid_t) -> libc::c_int {
    unsafe {
        let mut psi: ps_index_t = 0;
        let mut orig_psi: ps_index_t = 0;

        if (bgpids.storage).is_null() || bgpids.nalloc == 0 || bgpids.npid == 0 {
            return 0;
        }

        psi = *pshash_getbucket(pid);
        orig_psi = psi;
        while psi != NO_PIDSTAT {
            if (*(bgpids.storage).offset(psi as isize)).pid == pid {
                break;
            }
            if orig_psi == (*(bgpids.storage).offset(psi as isize)).bucket_next {
                internal_warning(
                    b"bgp_delete: LOOP: psi (%d) == storage[psi].bucket_next\0" as *const u8
                        as *const libc::c_char,
                    psi,
                );
                return 0 as libc::c_int;
            }
            psi = (*(bgpids.storage).offset(psi as isize)).bucket_next;
        }
        if psi == NO_PIDSTAT {
            return 0;
        }
        pshash_delindex(psi);
        bgpids.npid -= 1;
        return 1;
    }
}

fn bgp_clear() {
    unsafe {
        if (bgpids.storage).is_null() || bgpids.nalloc == 0 {
            return;
        }
        libc::free(bgpids.storage as *mut c_void);

        bgpids.storage = 0 as *mut pidstat;
        bgpids.nalloc = 0;
        bgpids.head = 0;
        bgpids.npid = 0;
    }
}

fn bgp_search(pid: pid_t) -> libc::c_int {
    unsafe {
        let mut psi: ps_index_t = 0;
        let mut orig_psi: ps_index_t = 0;
        if (bgpids.storage).is_null() || bgpids.nalloc == 0 || bgpids.npid == 0 {
            return -1;
        }

        psi = *pshash_getbucket(pid);
        orig_psi = psi;
        while psi != NO_PIDSTAT {
            if (*(bgpids.storage).offset(psi as isize)).pid == pid {
                return (*(bgpids.storage).offset(psi as isize)).status as libc::c_int;
            }
            if orig_psi == (*(bgpids.storage).offset(psi as isize)).bucket_next {
                internal_warning(
                    b"bgp_search: LOOP: psi (%d) == storage[psi].bucket_next\0" as *const u8
                        as *const libc::c_char,
                    psi,
                );
                return -1;
            }
            psi = (*(bgpids.storage).offset(psi as isize)).bucket_next;
        }
        return -1;
    }
}

#[no_mangle]
pub fn save_proc_status(pid: pid_t, status: libc::c_int) {
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };
    unsafe {
        BLOCK_CHILD(&mut set, &mut oset);
        bgp_add(pid, status);
        UNBLOCK_CHILD(&mut oset);
    }
}

fn procsub_free(p: *mut PROCESS) {
    unsafe {
        if !((*p).command).is_null() {
            free(((*p).command) as *mut c_void);
        }
        free(p as *mut c_void);
    }
}

#[no_mangle]
pub fn procsub_add(p: *mut PROCESS) -> *mut PROCESS {
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };
    unsafe {
        BLOCK_CHILD(&mut set, &mut oset);
        if (procsubs.head).is_null() {
            procsubs.end = p;
            procsubs.head = procsubs.end;
            procsubs.nproc = 0;
        } else {
            (*procsubs.end).next = p;
            procsubs.end = p;
        }
        procsubs.nproc += 1;
        UNBLOCK_CHILD(&mut oset);

        return p;
    }
}

#[no_mangle]
pub fn procsub_search(pid: pid_t) -> *mut PROCESS {
    unsafe {
        let mut p: *mut PROCESS = 0 as *mut PROCESS;
        let mut set: sigset_t = __sigset_t { __val: [0; 16] };
        let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

        BLOCK_CHILD(&mut set, &mut oset);
        p = procsubs.head;
        while !p.is_null() {
            if (*p).pid == pid {
                break;
            }
            p = (*p).next;
        }
        UNBLOCK_CHILD(&mut oset);
        return p;
    }
}

#[no_mangle]
pub fn procsub_delete(pid: pid_t) -> *mut PROCESS {
    unsafe {
        let mut p: *mut PROCESS = 0 as *mut PROCESS;
        let mut prev: *mut PROCESS = 0 as *mut PROCESS;
        let mut set: sigset_t = __sigset_t { __val: [0; 16] };
        let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

        BLOCK_CHILD(&mut set, &mut oset);
        prev = procsubs.head;
        p = prev;
        while !p.is_null() {
            if (*p).pid == pid {
                (*prev).next = (*p).next;
                break;
            } else {
                prev = p;
                p = (*p).next;
            }
        }
        if p.is_null() {
            UNBLOCK_CHILD(&mut oset);
            return p;
        }
        if p == procsubs.head {
            procsubs.head = (*procsubs.head).next;
        } else if p == procsubs.end {
            procsubs.end = prev;
        }
        procsubs.nproc -= 1;
        if procsubs.nproc == 0 {
            procsubs.end = 0 as *mut PROCESS;
            procsubs.head = procsubs.end;
        } else if procsubs.nproc == 1 {
            procsubs.end = procsubs.head;
        }
        bgp_add((*p).pid, process_exit_status((*p).status));
        UNBLOCK_CHILD(&mut oset);
        return p;
    }
}

#[no_mangle]
pub fn procsub_waitpid(pid: pid_t) -> libc::c_int {
    unsafe {
        let mut p: *mut PROCESS = 0 as *mut PROCESS;
        let mut r: libc::c_int = 0;

        p = procsub_search(pid);
        if p.is_null() {
            return -1;
        }
        if (*p).running == PS_DONE as i32 {
            return (*p).status;
        }
        r = wait_for((*p).pid, 0);
        return r;
    }
}

#[no_mangle]
pub fn procsub_waitall() {
    unsafe {
        let mut p: *mut PROCESS = 0 as *mut PROCESS;
        let mut r: libc::c_int = 0;

        p = procsubs.head;
        while !p.is_null() {
            if !((*p).running == PS_DONE as i32) {
                r = wait_for((*p).pid, 0);
            }
            p = (*p).next;
        }
    }
}

#[no_mangle]
pub fn procsub_clear() {
    unsafe {
        let mut p: *mut PROCESS = 0 as *mut PROCESS;
        let mut ps: *mut PROCESS = 0 as *mut PROCESS;
        let mut set: sigset_t = __sigset_t { __val: [0; 16] };
        let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

        BLOCK_CHILD(&mut set, &mut oset);
        ps = procsubs.head;
        while !ps.is_null() {
            p = ps;
            ps = (*ps).next;
            procsub_free(p);
        }
        procsubs.end = 0 as *mut PROCESS;
        procsubs.head = procsubs.end;
        procsubs.nproc = 0 as libc::c_int;
        UNBLOCK_CHILD(&mut oset);
    }
}

#[no_mangle]
pub fn procsub_prune() {
    unsafe {
        let mut ohead: *mut PROCESS = 0 as *mut PROCESS;
        let mut oend: *mut PROCESS = 0 as *mut PROCESS;
        let mut ps: *mut PROCESS = 0 as *mut PROCESS;
        let mut p: *mut PROCESS = 0 as *mut PROCESS;
        let mut onproc: libc::c_int = 0;

        if procsubs.nproc == 0 {
            return;
        }
        ohead = procsubs.head;
        oend = procsubs.end;
        onproc = procsubs.nproc;

        procsubs.end = 0 as *mut PROCESS;
        procsubs.head = procsubs.end;
        procsubs.nproc = 0 as libc::c_int;

        p = ohead;
        while !p.is_null() {
            ps = (*p).next;
            (*p).next = 0 as *mut process;
            if (*p).running == 0 as libc::c_int {
                bgp_add((*p).pid, process_exit_status((*p).status));
                procsub_free(p);
            } else {
                procsub_add(p);
            }
            p = ps;
        }
    }
}

fn reset_job_indices() {
    unsafe {
        let mut old: libc::c_int = 0;

        if (*jobs.offset(js.j_firstj as isize)).is_null() {
            js.j_firstj = js.j_firstj + 1;
            old = js.j_firstj;
            if old >= js.j_jobslots {
                old = js.j_jobslots - 1;
            }
            while js.j_firstj != old {
                if js.j_firstj >= js.j_jobslots {
                    js.j_firstj = 0;
                }
                if !(*jobs.offset(js.j_firstj as isize)).is_null() || js.j_firstj == old {
                    break;
                }
                js.j_firstj += 1;
            }
            if js.j_firstj == old {
                js.j_njobs = 0;
                js.j_lastj = js.j_njobs;
                js.j_firstj = js.j_lastj;
            }
        }
        if (*jobs.offset(js.j_lastj as isize)).is_null() {
            js.j_lastj = js.j_lastj - 1;
            old = js.j_lastj;
            if old < 0 {
                old = 0;
            }
            while js.j_lastj != old {
                if js.j_lastj < 0 {
                    js.j_lastj = js.j_jobslots - 1;
                }
                if !(*jobs.offset(js.j_lastj as isize)).is_null() || js.j_lastj == old {
                    break;
                }
                js.j_lastj -= 1;
            }
            if js.j_lastj == old {
                js.j_njobs = 0 as libc::c_int;
                js.j_lastj = js.j_njobs;
                js.j_firstj = js.j_lastj;
            }
        }
    }
}

fn cleanup_dead_jobs() {
    unsafe {
        let mut i: libc::c_int = 0;
        let mut os: libc::c_int = 0;
        let discard: *mut PROCESS = 0 as *mut PROCESS;

        if js.j_jobslots == 0 || jobs_list_frozen != 0 {
            return;
        }
        QUEUE_SIGCHLD!(os);
        i = 0;
        while i < js.j_jobslots {
            if !(*jobs.offset(i as isize)).is_null() && DEADJOB!(i) && IS_NOTIFIED!(i) {
                delete_job(i, 0 as libc::c_int);
            }
            i += 1;
        }
        procsub_prune();
        last_procsub_child = 0 as *mut c_void as *mut PROCESS;
        coproc_reap();
        UNQUEUE_SIGCHLD(os)
    }
}

fn processes_in_job(job: libc::c_int) -> libc::c_int {
    unsafe {
        let mut nproc: libc::c_int = 0;
        let mut p: *mut PROCESS = 0 as *mut PROCESS;

        nproc = 0 as libc::c_int;
        p = (**jobs.offset(job as isize)).pipe;
        loop {
            p = (*p).next;
            nproc += 1;
            if !(p != (**jobs.offset(job as isize)).pipe) {
                break;
            }
        }
        return nproc;
    }
}

fn delete_old_job(pid: pid_t) {
    unsafe {
        let mut p: *mut PROCESS = 0 as *mut PROCESS;
        let mut job: libc::c_int = 0;

        job = find_job(pid, 0, &mut p);
        if job != NO_JOB {
            if JOBSTATE!(job) == JDEAD {
                delete_job(job, 2 as libc::c_int);
            } else if !p.is_null() {
                (*p).pid = 0;
            }
        }
    }
}

fn realloc_jobs_list() {
    unsafe {
        let mut set: sigset_t = __sigset_t { __val: [0; 16] };
        let mut oset: sigset_t = __sigset_t { __val: [0; 16] };
        let mut nsize: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut ncur: libc::c_int = 0;
        let mut nprev: libc::c_int = 0;
        let mut nlist: *mut *mut JOB = 0 as *mut *mut JOB;

        nprev = NO_JOB;
        ncur = nprev;
        nsize = (js.j_njobs + JOB_SLOTS as libc::c_int - 1) / JOB_SLOTS as libc::c_int;
        nsize *= JOB_SLOTS as libc::c_int;
        i = js.j_njobs % JOB_SLOTS as libc::c_int;
        if i == 0 || i > (JOB_SLOTS as libc::c_int >> 1) {
            nsize += JOB_SLOTS as libc::c_int;
        }
        BLOCK_CHILD(&mut set, &mut oset);
        nlist = if js.j_jobslots == nsize {
            jobs
        } else {
            malloc((nsize * std::mem::size_of::<JOB>() as libc::c_int) as usize) as *mut *mut JOB
        };

        js.j_ndead = 0;
        js.c_reaped = js.j_ndead;
        j = 0;
        i = j;
        while i < js.j_jobslots {
            if !(*jobs.offset(i as isize)).is_null() {
                if i == js.j_current {
                    ncur = j;
                }
                if i == js.j_previous {
                    nprev = j;
                }

                *nlist.offset(j as isize) = *jobs.offset(i as isize);
                j = j + 1; //
                if (**jobs.offset(i as isize)).state as libc::c_int == JDEAD as libc::c_int {
                    js.j_ndead += 1;
                    js.c_reaped += processes_in_job(i);
                }
            }
            i += 1;
        }

        js.j_firstj = 0;
        js.j_lastj = if j > 0 { j - 1 } else { 0 };
        js.j_njobs = j;

        js.j_jobslots = nsize;

        while j < nsize {
            *nlist.offset(j as isize) = 0 as *mut JOB;
            j += 1;
        }

        if jobs != nlist {
            libc::free(jobs as *mut c_void);
            jobs = nlist;
        }

        if ncur != NO_JOB {
            js.j_current = ncur;
        }

        if nprev != NO_JOB {
            js.j_previous = nprev;
        }
        if js.j_current == NO_JOB
            || js.j_previous == NO_JOB
            || js.j_current > js.j_lastj
            || js.j_previous > js.j_lastj
        {
            reset_current();
        }
        UNBLOCK_CHILD(&mut oset);
    }
}

fn compact_jobs_list(flags: libc::c_int) -> libc::c_int {
    unsafe {
        if js.j_jobslots == 0 || jobs_list_frozen != 0 {
            return js.j_jobslots;
        }
        reap_dead_jobs();
        realloc_jobs_list();
        return if js.j_lastj != 0 || !(*jobs.offset(js.j_lastj as isize)).is_null() {
            js.j_lastj + 1
        } else {
            0
        };
    }
}

#[no_mangle]
pub fn delete_job(job_index: libc::c_int, dflags: libc::c_int) {
    unsafe {
        let mut temp: *mut JOB = 0 as *mut JOB;
        let mut proc_0: *mut PROCESS = 0 as *mut PROCESS;
        let mut ndel: libc::c_int = 0;

        if js.j_jobslots == 0 || jobs_list_frozen != 0 {
            return;
        }
        if dflags & DEL_WARNSTOPPED as libc::c_int != 0
            && subshell_environment == 0
            && STOPPED!(job_index)
        {
            internal_warning(
                b"deleting stopped job %d with process group %ld\0" as *const u8
                    as *const libc::c_char,
                job_index + 1,
                (**jobs.offset(job_index as isize)).pgrp as libc::c_long,
            );
        }
        temp = *jobs.offset(job_index as isize);
        if temp.is_null() {
            return;
        }

        if dflags & DEL_NOBGPID as libc::c_int == 0 as libc::c_int
            && (*temp).flags & (J_ASYNC as libc::c_int | J_FOREGROUND as libc::c_int)
                == J_ASYNC as libc::c_int
        {
            proc_0 = find_last_proc(job_index, 0);
            if !proc_0.is_null() {
                bgp_add((*proc_0).pid, process_exit_status((*proc_0).status));
            }
        }

        *jobs.offset(job_index as isize) = 0 as *mut JOB;
        if temp == js.j_lastmade {
            js.j_lastmade = 0 as *mut JOB;
        } else if temp == js.j_lastasync {
            js.j_lastasync = 0 as *mut JOB;
        }

        libc::free((*temp).wd as *mut c_void);
        ndel = discard_pipeline((*temp).pipe);

        js.c_injobs -= ndel;
        if (*temp).state == JDEAD as libc::c_int {
            js.c_reaped -= ndel;
            js.j_ndead -= 1;
            if js.c_reaped < 0 {
                js.c_reaped = 0;
            }
        }
        if !((*temp).deferred).is_null() {
            dispose_command((*temp).deferred);
        }

        libc::free(temp as *mut c_void);

        js.j_njobs -= 1;
        if js.j_njobs == 0 {
            js.j_lastj = 0;
            js.j_firstj = js.j_lastj;
        } else if (*jobs.offset(js.j_firstj as isize)).is_null()
            || (*jobs.offset(js.j_lastj as isize)).is_null()
        {
            reset_job_indices();
        }
        if job_index == js.j_current || job_index == js.j_previous {
            reset_current();
        }
    }
}

#[no_mangle]
pub fn nohup_job(job_index: libc::c_int) {
    unsafe {
        let mut temp: *mut JOB = 0 as *mut JOB;

        if js.j_jobslots == 0 {
            return;
        }
        temp = *jobs.offset(job_index as isize);
        if !temp.is_null() {
            (*temp).flags |= J_NOHUP as libc::c_int;
        }
    }
}

#[no_mangle]
pub fn discard_pipeline(chain: *mut PROCESS) -> libc::c_int {
    unsafe {
        let mut this: *mut PROCESS = 0 as *mut PROCESS;
        let mut next: *mut PROCESS = 0 as *mut PROCESS;
        let mut n: libc::c_int = 0;

        this = chain;
        n = 0;
        loop {
            next = (*this).next;
            if !((*this).command).is_null() {
                libc::free((*this).command as *mut c_void);
            }
            libc::free(this as *mut c_void);
            n += 1;
            this = next;
            if !(this != chain) {
                break;
            }
        }
        return n;
    }
}

fn add_process(name: *mut libc::c_char, pid: pid_t) {
    unsafe {
        let mut t: *mut PROCESS = 0 as *mut PROCESS;
        let mut p: *mut PROCESS = 0 as *mut PROCESS;

        t = malloc(::std::mem::size_of::<PROCESS>() as usize) as *mut PROCESS;
        (*t).next = the_pipeline;
        (*t).pid = pid;
        (*t).status = 0;
        (*t).running = PS_RUNNING as libc::c_int;
        (*t).command = name;
        the_pipeline = t;

        if ((*t).next).is_null() {
            (*t).next = t;
        } else {
            p = (*t).next;
            while (*p).next != (*t).next {
                p = (*p).next;
            }
            (*p).next = t;
        };
    }
}

#[no_mangle]
pub fn append_process(name: *mut libc::c_char, pid: pid_t, status: libc::c_int, jid: libc::c_int) {
    unsafe {
        let mut t: *mut PROCESS = 0 as *mut PROCESS;
        let mut p: *mut PROCESS = 0 as *mut PROCESS;

        t = malloc(::std::mem::size_of::<PROCESS>() as usize) as *mut PROCESS;
        (*t).next = 0 as *mut c_void as *mut PROCESS;
        (*t).pid = pid;
        (*t).status = (status & 0xff as libc::c_int) << WEXITSTATUS_OFFSET as libc::c_int;
        (*t).running = PS_DONE as libc::c_int;
        (*t).command = name;

        js.c_reaped += 1;
        p = (**jobs.offset(jid as isize)).pipe;
        while (*p).next != (**jobs.offset(jid as isize)).pipe {
            p = (*p).next;
        }

        (*p).next = t;
        (*t).next = (**jobs.offset(jid as isize)).pipe;
    }
}

fn map_over_jobs(
    func: Option<sh_job_map_func_t>,
    arg1: libc::c_int,
    arg2: libc::c_int,
) -> libc::c_int {
    unsafe {
        let mut i: libc::c_int = 0;
        let mut result: libc::c_int = 0;
        let mut set: sigset_t = __sigset_t { __val: [0; 16] };
        let mut oset: sigset_t = __sigset_t { __val: [0; 16] };
        if js.j_jobslots == 0 {
            return 0;
        }
        BLOCK_CHILD(&mut set, &mut oset);
        result = 0;
        i = result;
        while i < js.j_jobslots {
            if !(*jobs.offset(i as isize)).is_null() {
                result = (Some(func.expect("non-null function pointer")))
                    .expect("non-null function pointer")(
                    *jobs.offset(i as isize), arg1, arg2, i
                );
                if result != 0 {
                    break;
                }
            }
            i += 1;
        }
        UNBLOCK_CHILD(&mut oset);
        return result;
    }
}

#[no_mangle]
pub fn terminate_current_pipeline() {
    unsafe {
        if pipeline_pgrp != 0 && pipeline_pgrp != shell_pgrp {
            killpg(pipeline_pgrp, SIGTERM as libc::c_int);
            killpg(pipeline_pgrp, SIGCONT as libc::c_int);
        }
    }
}

#[no_mangle]
pub fn terminate_stopped_jobs() {
    unsafe {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < js.j_jobslots {
            if !(*jobs.offset(i as isize)).is_null() && STOPPED!(i) {
                killpg((**jobs.offset(i as isize)).pgrp, SIGTERM as libc::c_int);
                killpg((**jobs.offset(i as isize)).pgrp, SIGCONT as libc::c_int);
            }
            i += 1;
        }
    }
}

#[no_mangle]
pub fn hangup_all_jobs() {
    unsafe {
        let mut i: libc::c_int = 0;
        i = 0;
        while i < js.j_jobslots {
            if !(*jobs.offset(i as isize)).is_null() {
                if !((**jobs.offset(i as isize)).flags & J_NOHUP as libc::c_int != 0) {
                    continue;
                }
                killpg((**jobs.offset(i as isize)).pgrp, SIGHUP as libc::c_int);
                if STOPPED!(i) {
                    killpg((**jobs.offset(i as isize)).pgrp, SIGCONT as libc::c_int);
                }
            }
            i += 1;
        }
    }
}

#[no_mangle]
pub fn kill_current_pipeline() {
    stop_making_children();
    start_pipeline();
}

fn find_pid_in_pipeline(
    pid: pid_t,
    pipeline: *mut PROCESS,
    alive_only: libc::c_int,
) -> *mut PROCESS {
    unsafe {
        let mut p: *mut PROCESS = 0 as *mut PROCESS;
        p = pipeline;
        loop {
            if (*p).pid == pid && (alive_only == 0 && PRECYCLED!(p) == 0 || PALIVE!(p)) {
                return p;
            }
            p = (*p).next;
            if !(p != pipeline) {
                break;
            }
        }
        return 0 as *mut PROCESS;
    }
}

fn find_pipeline(pid: pid_t, alive_only: libc::c_int, jobp: *mut libc::c_int) -> *mut PROCESS {
    unsafe {
        let mut job: libc::c_int = 0;
        let mut p: *mut PROCESS = 0 as *mut PROCESS;
        let mut save: *mut pipeline_saver = 0 as *mut pipeline_saver;
        p = 0 as *mut libc::c_void as *mut PROCESS;
        if !jobp.is_null() {
            *jobp = -(1 as libc::c_int);
        }
        if !the_pipeline.is_null() && {
            p = find_pid_in_pipeline(pid, the_pipeline, alive_only);
            !p.is_null()
        } {
            return p;
        }
        save = saved_pipeline;
        while !save.is_null() {
            if !((*save).pipeline).is_null() && {
                p = find_pid_in_pipeline(pid, (*save).pipeline, alive_only);
                !p.is_null()
            } {
                return p;
            }
            save = (*save).next;
        }
        if procsubs.nproc > 0 as libc::c_int
            && {
                p = procsub_search(pid);
                !p.is_null()
            }
            && (alive_only == 0 as libc::c_int && 0 as libc::c_int == 0 as libc::c_int
                || ((*p).running == 1 as libc::c_int
                    || (*p).status & 0xff as libc::c_int == 0x7f as libc::c_int))
        {
            return p;
        }
        job = find_job(pid, alive_only, &mut p);
        if !jobp.is_null() {
            *jobp = job;
        }
        return if job == -(1 as libc::c_int) {
            0 as *mut libc::c_void as *mut PROCESS
        } else {
            (**jobs.offset(job as isize)).pipe
        };
    }
}

fn find_process(pid: pid_t, alive_only: libc::c_int, jobp: *mut libc::c_int) -> *mut PROCESS {
    unsafe {
        let mut p: *mut PROCESS = 0 as *mut PROCESS;
        p = find_pipeline(pid, alive_only, jobp);
        while !p.is_null() && (*p).pid != pid {
            p = (*p).next;
        }
        return p;
    }
}

fn find_job(pid: pid_t, alive_only: libc::c_int, procp: *mut *mut PROCESS) -> libc::c_int {
    unsafe {
        let mut i: libc::c_int = 0;
        let mut p: *mut PROCESS = 0 as *mut PROCESS;
        i = 0 as libc::c_int;
        while i < js.j_jobslots {
            if !(*jobs.offset(i as isize)).is_null() {
                p = (**jobs.offset(i as isize)).pipe;
                loop {
                    if (*p).pid == pid
                        && (alive_only == 0 as libc::c_int && 0 as libc::c_int == 0 as libc::c_int
                            || ((*p).running == 1 as libc::c_int
                                || (*p).status & 0xff as libc::c_int == 0x7f as libc::c_int))
                    {
                        if !procp.is_null() {
                            *procp = p;
                        }
                        return i;
                    }
                    p = (*p).next;
                    if !(p != (**jobs.offset(i as isize)).pipe) {
                        break;
                    }
                }
            }
            i += 1;
        }
        return -(1 as libc::c_int);
    }
}

#[no_mangle]
pub fn get_job_by_pid(pid: pid_t, block: libc::c_int, procp: *mut *mut PROCESS) -> libc::c_int {
    unsafe {
        let mut job: libc::c_int = 0;
        let mut set: sigset_t = __sigset_t { __val: [0; 16] };
        let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

        if block != 0 {
            BLOCK_CHILD(&mut set, &mut oset);
        }
        job = find_job(pid, 0 as libc::c_int, procp);
        if block != 0 {
            UNBLOCK_CHILD(&mut oset);
        }
        return job;
    }
}

#[no_mangle]
pub fn describe_pid(pid: pid_t) {
    unsafe {
        let mut job: libc::c_int = 0;
        let mut set: sigset_t = __sigset_t { __val: [0; 16] };
        let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

        BLOCK_CHILD(&mut set, &mut oset);
        job = find_job(pid, 0, 0 as *mut *mut PROCESS);

        if job != NO_JOB {
            libc::fprintf(
                stderr,
                b"[%d] %ld\n\0" as *const u8 as *const libc::c_char,
                job + 1 as libc::c_int,
                pid as libc::c_long,
            );
        } else {
            programming_error(
                b"describe_pid: %ld: no such pid\0" as *const u8 as *const libc::c_char,
                pid as libc::c_long,
            );
        }
        UNBLOCK_CHILD(&mut oset);
    }
}

fn j_strsignal(s: libc::c_int) -> *mut libc::c_char {
    unsafe {
        let mut x: *mut libc::c_char = 0 as *mut libc::c_char;
        x = strsignal(s);
        if x.is_null() {
            x = retcode_name_buffer.as_mut_ptr();
            libc::snprintf(
                x,
                ::std::mem::size_of::<[libc::c_char; 64]>() as usize,
                b"Signal %d\0" as *const u8 as *const libc::c_char,
                s,
            );
        }
        return x;
    }
}

fn printable_job_status(j: libc::c_int, p: *mut PROCESS, format: libc::c_int) -> *mut libc::c_char {
    unsafe {
        static mut temp: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
        let mut es: libc::c_int = 0;
        temp = b"Done\0" as *const u8 as *const libc::c_char as *mut libc::c_char;

        if STOPPED!(j) && format == 0 {
            if posixly_correct == 0 as libc::c_int || p.is_null() || (WIFSTOPPED!((*p).status)) {
                temp = b"Stopped\0" as *const u8 as *mut libc::c_char;
            } else {
                temp = retcode_name_buffer.as_mut_ptr();
                libc::snprintf(
                    temp,
                    ::std::mem::size_of::<[libc::c_char; 64]>() as usize,
                    b"Stopped(%s)\0" as *const u8 as *const libc::c_char,
                    signal_name(WSTOPSIG!((*p).status)),
                );
            }
        } else if RUNNING!(j) {
            temp = b"Running\0" as *const u8 as *mut libc::c_char;
        } else {
            if WIFSTOPPED!((*p).status) {
                temp = j_strsignal(WSTOPSIG!((*p).status));
            } else if WIFSIGNALED!((*p).status) {
                temp = j_strsignal(WTERMSIG!((*p).status));
            } else if WIFEXITED!((*p).status) {
                temp = retcode_name_buffer.as_mut_ptr();
                es = WEXITSTATUS!((*p).status);
                if es == 0 {
                    libc::strncpy(
                        temp,
                        b"Done\0" as *const u8 as *mut libc::c_char,
                        std::mem::size_of::<[libc::c_char; 64]>() - 1,
                    );
                    *temp.offset(
                        (::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) = '\u{0}' as i32 as libc::c_char;
                } else if posixly_correct != 0 {
                    libc::snprintf(
                        temp,
                        ::std::mem::size_of::<[libc::c_char; 64]>() as usize,
                        b"Done(%d)\0" as *const u8 as *const libc::c_char,
                        es,
                    );
                } else {
                    libc::snprintf(
                        temp,
                        ::std::mem::size_of::<[libc::c_char; 64]>() as usize,
                        b"Exit(%d)\0" as *const u8 as *const libc::c_char,
                        es,
                    );
                }
            } else {
                temp = b"Unknown status\0" as *const u8 as *mut libc::c_char;
            }
        }

        return temp;
    }
}

fn print_pipeline(
    mut p: *mut PROCESS,
    job_index: libc::c_int,
    format: libc::c_int,
    stream: *mut libc::FILE,
) {
    unsafe {
        let mut first: *mut PROCESS = 0 as *mut PROCESS;
        let mut last: *mut PROCESS = 0 as *mut PROCESS;
        let mut show: *mut PROCESS = 0 as *mut PROCESS;
        let mut es: libc::c_int = 0;
        let mut name_padding: libc::c_int = 0;
        let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;

        if p.is_null() {
            return;
        }

        last = p;
        first = last;
        while (*last).next != first {
            last = (*last).next;
        }

        loop {
            if p != first {
                libc::fprintf(
                    stream,
                    if format != 0 {
                        b"     \0" as *const u8 as *const libc::c_char
                    } else {
                        b" |\0" as *const u8 as *const libc::c_char
                    },
                );
            }
            if format != 0 {
                libc::fprintf(
                    stream,
                    b"%5ld\0" as *const u8 as *const libc::c_char,
                    (*p).pid as libc::c_long,
                );
            }
            libc::fprintf(stream, b" \0" as *const u8 as *const libc::c_char);

            if format > -1 && job_index >= 0 {
                show = if format != 0 { p } else { last };
                temp = printable_job_status(job_index, show, format);

                if p != first {
                    if format != 0 {
                        if (*show).running == (*first).running
                            && WSTATUS!((*show).status) == WSTATUS!((*first).status)
                        {
                            temp = b"\0" as *const u8 as *mut libc::c_char;
                        }
                    } else {
                        temp = 0 as *mut libc::c_char;
                    }
                }

                if !temp.is_null() {
                    libc::fprintf(stream, b"%s\0" as *const u8 as *const libc::c_char, temp);
                    es = STRLEN!(temp) as libc::c_int;

                    if es == 0 {
                        es = 2;
                    }
                    name_padding = LONGEST_SIGNAL_DESC as libc::c_int - es;
                    libc::fprintf(
                        stream,
                        b"%*s\0" as *const u8 as *const libc::c_char,
                        name_padding,
                        b"\0" as *const u8 as *const libc::c_char,
                    );
                    if (WIFSTOPPED!((*show).status)) as libc::c_int == 0
                        && (WIFCONTINUED!((*show).status)) as libc::c_int == 0
                        && WIFCORED!((*show).status) as libc::c_int != 0
                    {
                        libc::fprintf(
                            stream,
                            b"(core dumped) \0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
            }
            if p != first && format != 0 {
                libc::fprintf(stream, b"| \0" as *const u8 as *const libc::c_char);
            }
            if !((*p).command).is_null() {
                fprintf(
                    stream,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    (*p).command,
                );
            }
            if p == last && job_index >= 0 {
                temp = current_working_directory();
                if RUNNING!(job_index) && IS_FOREGROUND!(job_index) as libc::c_int == 0 {
                    fprintf(stream, b" &\0" as *const u8 as *const libc::c_char);
                }
                if libc::strcmp(temp, (**jobs.offset(job_index as isize)).wd) != 0 as libc::c_int {
                    fprintf(
                        stream,
                        b"  (wd: %s)\0" as *const u8 as *const libc::c_char,
                        polite_directory_format((**jobs.offset(job_index as isize)).wd),
                    );
                }
            }
            if format != 0 || p == last {
                if asynchronous_notification != 0 && interactive != 0 {
                    libc::fputc('\r' as i32, stream);
                }
                fprintf(stream, b"\n\0" as *const u8 as *const libc::c_char);
            }
            if p == last {
                break;
            }
            p = (*p).next;
        }
        libc::fflush(stream);
    }
}

fn pretty_print_job(job_index: libc::c_int, mut format: libc::c_int, stream: *mut libc::FILE) {
    unsafe {
        let mut p: *mut PROCESS = 0 as *mut PROCESS;

        if format == JLIST_PID_ONLY as libc::c_int {
            fprintf(
                stream,
                b"%ld\n\0" as *const u8 as *const libc::c_char,
                (*(**jobs.offset(job_index as isize)).pipe).pid as libc::c_long,
            );
            return;
        }
        if format == JLIST_CHANGED_ONLY as libc::c_int {
            if IS_NOTIFIED!(job_index) {
                return;
            }
            format = JLIST_STANDARD as libc::c_int;
        }
        if format != JLIST_NONINTERACTIVE as libc::c_int {
            fprintf(
                stream,
                b"[%d]%c \0" as *const u8 as *const libc::c_char,
                job_index + 1 as libc::c_int,
                if job_index == js.j_current {
                    '+' as i32
                } else if job_index == js.j_previous {
                    '-' as i32
                } else {
                    ' ' as i32
                },
            );
        }
        if format == JLIST_NONINTERACTIVE as libc::c_int {
            format = JLIST_LONG as libc::c_int;
        }
        p = (**jobs.offset(job_index as isize)).pipe;
        print_pipeline(p, job_index, format, stream);
        (**jobs.offset(job_index as isize)).flags |= J_NOTIFIED as libc::c_int;
    }
}

fn print_job(
    job: *mut JOB,
    format: libc::c_int,
    state: libc::c_int,
    job_index: libc::c_int,
) -> libc::c_int {
    unsafe {
        if state == -(1 as libc::c_int)
            || state as JOB_STATE as libc::c_int == (*job).state as libc::c_int
        {
            pretty_print_job(job_index, format, stdout);
        }
        return 0 as libc::c_int;
    }
}

#[no_mangle]
pub fn list_one_job(
    job: *mut JOB,
    format: libc::c_int,
    ignore: libc::c_int,
    job_index: libc::c_int,
) {
    unsafe {
        pretty_print_job(job_index, format, stdout);
        cleanup_dead_jobs();
    }
}

#[no_mangle]
pub fn list_stopped_jobs(format: libc::c_int) {
    unsafe {
        cleanup_dead_jobs();
        map_over_jobs(
            ::std::mem::transmute::<Option<fn() -> libc::c_int>, Option<sh_job_map_func_t>>(Some(
                ::std::mem::transmute::<
                    fn(*mut JOB, libc::c_int, libc::c_int, libc::c_int) -> libc::c_int,
                    fn() -> libc::c_int,
                >(print_job),
            )),
            format,
            JSTOPPED as libc::c_int,
        );
    }
}

#[no_mangle]
pub fn list_running_jobs(format: libc::c_int) {
    cleanup_dead_jobs();
    unsafe {
        map_over_jobs(
            ::std::mem::transmute::<Option<fn() -> libc::c_int>, Option<sh_job_map_func_t>>(Some(
                ::std::mem::transmute::<
                    fn(*mut JOB, libc::c_int, libc::c_int, libc::c_int) -> libc::c_int,
                    fn() -> libc::c_int,
                >(print_job),
            )),
            format,
            JRUNNING as libc::c_int,
        );
    }
}

#[no_mangle]
pub fn list_all_jobs(format: libc::c_int) {
    cleanup_dead_jobs();
    unsafe {
        map_over_jobs(
            ::std::mem::transmute::<Option<fn() -> libc::c_int>, Option<sh_job_map_func_t>>(Some(
                ::std::mem::transmute::<
                    fn(*mut JOB, libc::c_int, libc::c_int, libc::c_int) -> libc::c_int,
                    fn() -> libc::c_int,
                >(print_job),
            )),
            format,
            -(1 as libc::c_int),
        );
    }
}

#[no_mangle]
pub fn make_child(command: *mut libc::c_char, flags: libc::c_int) -> pid_t {
    unsafe {
        let mut async_p: libc::c_int = 0;
        let mut forksleep: libc::c_int = 0;
        let mut set: sigset_t = __sigset_t { __val: [0; 16] };
        let mut oset: sigset_t = __sigset_t { __val: [0; 16] };
        let termset: sigset_t = __sigset_t { __val: [0; 16] };
        let chldset: sigset_t = __sigset_t { __val: [0; 16] };
        let mut oset_copy: sigset_t = __sigset_t { __val: [0; 16] };
        let mut pid: pid_t = 0;
        //let mut oterm: *mut SigHandler = 0 as *mut SigHandler;
        let mut oterm: Option<SigHandler> = None;
        sigemptyset(&mut oset_copy);
        sigprocmask(
            0 as libc::c_int,
            0 as *mut libc::c_void as *mut sigset_t,
            &mut oset_copy,
        );
        sigaddset(&mut oset_copy, 15 as libc::c_int);
        sigemptyset(&mut set);
        sigaddset(&mut set, 17 as libc::c_int);
        sigaddset(&mut set, 2 as libc::c_int);
        sigaddset(&mut set, 15 as libc::c_int);
        sigemptyset(&mut oset);
        sigprocmask(0 as libc::c_int, &mut set, &mut oset);
        if interactive_shell != 0 {
            oterm = set_signal_handler(15 as libc::c_int, None);
        }
        making_children();
        async_p = flags & 1 as libc::c_int;
        forksleep = 1 as libc::c_int;
        if default_buffered_input != -(1 as libc::c_int)
            && (async_p == 0 || default_buffered_input > 0 as libc::c_int)
        {
            sync_buffered_stream(default_buffered_input);
        }
        loop {
            pid = fork();
            if !(pid < 0 as libc::c_int
                && *__errno_location() == 11 as libc::c_int
                && forksleep < 16 as libc::c_int)
            {
                break;
            }
            sigprocmask(
                2 as libc::c_int,
                &mut oset_copy,
                0 as *mut libc::c_void as *mut sigset_t,
            );
            waitchld(-(1 as libc::c_int), 0 as libc::c_int);
            *__errno_location() = 11 as libc::c_int;
            sys_error(b"fork: retry\0" as *const u8 as *const libc::c_char);
            if sleep(forksleep as libc::c_uint) != 0 as libc::c_int as libc::c_uint {
                break;
            }
            forksleep <<= 1 as libc::c_int;
            if interrupt_state != 0 {
                break;
            }
            sigprocmask(
                2 as libc::c_int,
                &mut set,
                0 as *mut libc::c_void as *mut sigset_t,
            );
        }
        if pid != 0 as libc::c_int {
            if interactive_shell != 0 {
                set_signal_handler(15 as libc::c_int, oterm);
            }
        }
        if pid < 0 as libc::c_int {
            sys_error(b"fork\0" as *const u8 as *const libc::c_char);
            terminate_current_pipeline();
            if !the_pipeline.is_null() {
                kill_current_pipeline();
            }
            set_exit_status(126 as libc::c_int);
            throw_to_top_level();
        }
        if pid == 0 as libc::c_int {
            let mut mypid: pid_t = 0;
            mypid = getpid();
            unset_bash_input(0 as libc::c_int);
            ::std::ptr::write_volatile(&mut interrupt_state as *mut sig_atomic_t, 0 as libc::c_int);
            restore_sigmask();
            if job_control != 0 {
                if pipeline_pgrp == 0 as libc::c_int {
                    pipeline_pgrp = mypid;
                }
                if pipeline_pgrp == shell_pgrp {
                    ignore_tty_job_signals();
                } else {
                    default_tty_job_signals();
                }
                if setpgid(mypid, pipeline_pgrp) < 0 as libc::c_int {
                    sys_error(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"child setpgid (%ld to %ld)\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        mypid as libc::c_long,
                        pipeline_pgrp as libc::c_long,
                    );
                }
                if flags & 4 as libc::c_int == 0 as libc::c_int
                    && async_p == 0 as libc::c_int
                    && pipeline_pgrp != shell_pgrp
                    && subshell_environment & (0x1 as libc::c_int | 0x10 as libc::c_int)
                        == 0 as libc::c_int
                    && running_in_background == 0 as libc::c_int
                {
                    give_terminal_to(pipeline_pgrp, 0 as libc::c_int);
                }
                if pipeline_pgrp == mypid {
                    pipe_read(pgrp_pipe.as_mut_ptr());
                }
            } else {
                if pipeline_pgrp == 0 as libc::c_int {
                    pipeline_pgrp = shell_pgrp;
                }
                default_tty_job_signals();
            }
            sh_closepipe(pgrp_pipe.as_mut_ptr());
        } else {
            if job_control != 0 {
                if pipeline_pgrp == 0 as libc::c_int {
                    pipeline_pgrp = pid;
                }
                setpgid(pid, pipeline_pgrp);
            } else if pipeline_pgrp == 0 as libc::c_int {
                pipeline_pgrp = shell_pgrp;
            }
            add_process(command, pid);
            if async_p != 0 {
                ::std::ptr::write_volatile(&mut last_asynchronous_pid as *mut pid_t, pid);
            }
            delete_old_job(pid);
            bgp_delete(pid);
            ::std::ptr::write_volatile(&mut last_made_pid as *mut pid_t, pid);
            js.c_totforked += 1;
            js.c_living += 1;
            sigprocmask(
                2 as libc::c_int,
                &mut oset,
                0 as *mut libc::c_void as *mut sigset_t,
            );
        }
        return pid;
    }
}

#[no_mangle]
pub fn ignore_tty_job_signals() {
    unsafe {
        set_signal_handler(SIGTSTP as libc::c_int, SIG_IGN!());
        set_signal_handler(SIGTTIN as libc::c_int, SIG_IGN!());
        set_signal_handler(SIGTTOU as libc::c_int, SIG_IGN!());
    }
}

#[no_mangle]
pub fn default_tty_job_signals() {
    unsafe {
        if signal_is_trapped(SIGTSTP as libc::c_int) == 0
            && signal_is_hard_ignored(SIGTSTP as libc::c_int) != 0
        {
            set_signal_handler(SIGTSTP as libc::c_int, SIG_IGN!());
        } else {
            set_signal_handler(SIGTSTP as libc::c_int, None);
        }
        if signal_is_trapped(SIGTTIN as libc::c_int) == 0
            && signal_is_hard_ignored(SIGTTIN as libc::c_int) != 0
        {
            set_signal_handler(SIGTTIN as libc::c_int, SIG_IGN!());
        } else {
            set_signal_handler(SIGTTIN as libc::c_int, None);
        }
        if signal_is_trapped(SIGTTOU as libc::c_int) == 0
            && signal_is_hard_ignored(SIGTTOU as libc::c_int) != 0
        {
            set_signal_handler(SIGTTOU as libc::c_int, SIG_IGN!());
        } else {
            set_signal_handler(SIGTTOU as libc::c_int, None);
        };
    }
}

#[no_mangle]
pub fn get_original_tty_job_signals() {
    static mut fetched: libc::c_int = 0 as libc::c_int;
    unsafe {
        if fetched == 0 as libc::c_int {
            if interactive_shell != 0 {
                set_original_signal(SIGTSTP as libc::c_int, None);
                set_original_signal(SIGTTIN as libc::c_int, None);
                set_original_signal(SIGTTOU as libc::c_int, None);
            } else {
                get_original_signal(SIGTSTP as libc::c_int);
                get_original_signal(SIGTTIN as libc::c_int);
                get_original_signal(SIGTTOU as libc::c_int);
            }
            fetched = 1;
        }
    }
}

static mut shell_tty_info: libc::termios = libc::termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
};

#[no_mangle]
pub fn get_tty_state() -> libc::c_int {
    let mut tty: libc::c_int = 0;
    unsafe {
        tty = input_tty!();
        if tty != -1 {
            if libc::tcgetattr(tty, &mut shell_tty_info) < 0 {
                return -(1 as libc::c_int);
            }
            if check_window_size != 0 {
                get_new_window_size(
                    0 as libc::c_int,
                    0 as *mut libc::c_int,
                    0 as *mut libc::c_int,
                );
            }
        }
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub fn set_tty_state() -> libc::c_int {
    let mut tty: libc::c_int = 0;
    unsafe {
        tty = input_tty!();
        if tty != -1 {
            if libc::tcsetattr(tty, 1 as libc::c_int, &mut shell_tty_info) < 0 as libc::c_int {
                if interactive != 0 {
                    sys_error(
                        b"[%ld: %d (%d)] tcsetattr\0" as *const u8 as *const libc::c_char,
                        getpid() as libc::c_long,
                        shell_level,
                        tty,
                    );
                }
                return -1;
            }
        }
        return 0;
    }
}

fn find_last_proc(job: libc::c_int, block: libc::c_int) -> *mut PROCESS {
    unsafe {
        let mut p: *mut PROCESS = 0 as *mut PROCESS;
        let mut set: sigset_t = __sigset_t { __val: [0; 16] };
        let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

        if block != 0 {
            BLOCK_CHILD(&mut set, &mut oset);
        }
        p = (**jobs.offset(job as isize)).pipe;
        while !p.is_null() && (*p).next != (**jobs.offset(job as isize)).pipe {
            p = (*p).next;
        }
        if block != 0 {
            UNBLOCK_CHILD(&mut oset);
        }
        return p;
    }
}

fn find_last_pid(job: libc::c_int, block: libc::c_int) -> pid_t {
    unsafe {
        let mut p: *mut PROCESS = 0 as *mut PROCESS;
        p = find_last_proc(job, block);
        return (*p).pid;
    }
}

#[no_mangle]
pub fn wait_for_single_pid(pid: pid_t, flags: libc::c_int) -> libc::c_int {
    unsafe {
        let mut child: *mut PROCESS = 0 as *mut PROCESS;
        let mut set: sigset_t = __sigset_t { __val: [0; 16] };
        let mut oset: sigset_t = __sigset_t { __val: [0; 16] };
        let mut r: libc::c_int = 0;
        let mut job: libc::c_int = 0;
        let mut alive: libc::c_int = 0;

        BLOCK_CHILD(&mut set, &mut oset);
        child = find_pipeline(pid, 0, 0 as *mut libc::c_int);
        UNBLOCK_CHILD(&mut oset);

        if child.is_null() {
            r = bgp_search(pid);
            if r >= 0 {
                return r;
            }
        }
        if child.is_null() {
            if flags & JWAIT_PERROR as libc::c_int != 0 {
                internal_error(
                    b"wait: pid %ld is not a child of this shell\0" as *const u8
                        as *const libc::c_char,
                    pid as libc::c_long,
                );
            }
            return 127;
        }
        alive = 0;
        loop {
            r = wait_for(pid, 0);
            if flags & JWAIT_FORCE as libc::c_int == 0 {
                break;
            }

            BLOCK_CHILD(&mut set, &mut oset);
            alive = PALIVE!(child) as libc::c_int;
            UNBLOCK_CHILD(&mut oset);

            if !(alive != 0) {
                break;
            }
        }

        BLOCK_CHILD(&mut set, &mut oset);
        job = find_job(pid, 0, 0 as *mut *mut PROCESS);
        if job != NO_JOB && !(*jobs.offset(job as isize)).is_null() && DEADJOB!(job) {
            (**jobs.offset(job as isize)).flags |= J_NOTIFIED as libc::c_int;
        }
        UNBLOCK_CHILD(&mut oset);

        if posixly_correct != 0 {
            cleanup_dead_jobs();
            bgp_delete(pid);
        }

        CHECK_WAIT_INTR!();

        return r;
    }
}

#[no_mangle]
pub fn wait_for_background_pids(ps: *mut procstat) {
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut any_stopped: libc::c_int = 0;
    let mut check_async: libc::c_int = 0;
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };
    let mut pid: pid_t = 0;

    any_stopped = 0;
    check_async = 1;
    unsafe {
        loop {
            BLOCK_CHILD(&mut set, &mut set);

            i = 0;
            while i < js.j_jobslots {
                if !(*jobs.offset(i as isize)).is_null() && STOPPED!(i) {
                    builtin_warning(
                        b"job %d[%d] stopped\0" as *const u8 as *const libc::c_char,
                        i + 1,
                        find_last_pid(i, 0),
                    );
                    any_stopped = 1;
                }
                if !(*jobs.offset(i as isize)).is_null() && RUNNING!(i) && IS_FOREGROUND!(i) {
                    break;
                }
                i += 1;
            }
            if i == js.j_jobslots {
                UNBLOCK_CHILD(&mut oset);
                break;
            }

            pid = find_last_pid(i, 0);
            UNBLOCK_CHILD(&mut oset);
            QUIT!();
            if terminating_signal != 0 {
                termsig_handler(terminating_signal);
            }
            if interrupt_state != 0 {
                throw_to_top_level();
            }
            *__errno_location() = 0;
            r = wait_for_single_pid(pid, JWAIT_PERROR as libc::c_int);
            if !ps.is_null() {
                (*ps).pid = pid;
                (*ps).status = (if r < 0 as libc::c_int {
                    127 as libc::c_int
                } else {
                    r
                }) as libc::c_short;
            }
            if r == -1 && *__errno_location() == ECHILD as libc::c_int {
                check_async = 0;
                mark_all_jobs_as_dead();
            }
        }
        procsub_waitall();
        mark_dead_jobs_as_notified(1);
        cleanup_dead_jobs();
        bgp_clear();
    }
}

static mut wait_sigint_received: libc::c_int = 0;
static mut child_caught_sigint: libc::c_int = 0;

#[no_mangle]
pub fn wait_sigint_cleanup() {
    unsafe {
        queue_sigchld = 0;
        waiting_for_child = 0;
    }
}

#[macro_export]
macro_rules! INVALID_SIGNAL_HANDLER {
    () => {
        ::std::mem::transmute::<Option<fn() -> ()>, Option<SigHandler>>(Some(
            ::std::mem::transmute::<fn(*mut procstat) -> (), fn() -> ()>(wait_for_background_pids),
        ))
    };
}

static mut old_sigint_handler: Option<SigHandler> = unsafe { INVALID_SIGNAL_HANDLER!() };

fn restore_sigint_handler() {
    unsafe {
        if old_sigint_handler != INVALID_SIGNAL_HANDLER!() {
            set_signal_handler(SIGINT as libc::c_int, old_sigint_handler);
            old_sigint_handler = INVALID_SIGNAL_HANDLER!();
            waiting_for_child = 0;
        }
    }
}

fn wait_sigint_handler(sig: libc::c_int) {
    unsafe {
        let mut sigint_handler: Option<SigHandler> = None;

        if this_shell_builtin.is_some() && this_shell_builtin == Some(wait_builtin) {
            set_exit_status(128 + SIGINT as libc::c_int);
            restore_sigint_handler();
            if this_shell_builtin.is_some()
                && this_shell_builtin == Some(wait_builtin)
                && signal_is_trapped(SIGINT as libc::c_int) != 0
                && {
                    // sigint_handler = Some(trap_to_sighandler(SIGINT as libc::c_int));
                    // sigint_handler == Some(Some(trap_handler as unsafe extern "C" fn(c_int) -> ()))
                    sigint_handler = trap_to_sighandler(SIGINT as libc::c_int);
                    sigint_handler == Some(trap_handler)
                }
            {
                trap_handler(SIGINT as libc::c_int);
                wait_signal_received = SIGINT as libc::c_int;
                if wait_intr_flag != 0 {
                    siglongjmp(wait_intr_buf.as_mut_ptr(), 1);
                } else {
                    return;
                }
            } else {
                kill(getpid(), SIGINT as libc::c_int);
            }
        }
        if waiting_for_child != 0 {
            wait_sigint_received = 1;
        } else {
            set_exit_status(128 + SIGINT as libc::c_int);
            restore_sigint_handler();
            kill(getpid(), SIGINT as libc::c_int);
        };
    }
}

fn process_exit_signal(status: WAIT) -> libc::c_int {
    if libc::WIFSIGNALED(status) {
        return libc::WTERMSIG(status);
    } else {
        return 0;
    }
}
fn process_exit_status(status: WAIT) -> libc::c_int {
    unsafe {
        if WIFSIGNALED!(status) {
            return 128 + WTERMSIG!(status);
        } else if WIFSTOPPED!(status) as libc::c_int == 0 {
            return WEXITSTATUS!(status);
        } else {
            return 0;
        }
    }
}

fn job_signal_status(job: libc::c_int) -> WAIT {
    unsafe {
        let mut p: *mut PROCESS = 0 as *mut PROCESS;
        let mut s: WAIT = 0;

        p = (**jobs.offset(job as isize)).pipe;
        loop {
            s = (*p).status;
            if WIFSIGNALED!(s) || WIFSTOPPED!(s) {
                break;
            }
            p = (*p).next;
            if !(p != (**jobs.offset(job as isize)).pipe) {
                break;
            }
        }
        return s;
    }
}

fn raw_job_exit_status(job: libc::c_int) -> WAIT {
    unsafe {
        let mut p: *mut PROCESS = 0 as *mut PROCESS;
        let mut fail: libc::c_int = 0;
        let mut ret: WAIT = 0;

        if (**jobs.offset(job as isize)).flags & J_PIPEFAIL as libc::c_int != 0 {
            fail = 0;
            p = (**jobs.offset(job as isize)).pipe;
            loop {
                if WSTATUS!((*p).status) != 0 {
                    fail = WSTATUS!((*p).status);
                }
                p = (*p).next;
                if !(p != (**jobs.offset(job as isize)).pipe) {
                    break;
                }
            }
            WSTATUS!(ret) = fail;
            return ret;
        }
        p = (**jobs.offset(job as isize)).pipe;
        while (*p).next != (**jobs.offset(job as isize)).pipe {
            p = (*p).next;
        }
        return (*p).status;
    }
}

#[no_mangle]
pub fn job_exit_status(job: libc::c_int) -> libc::c_int {
    return process_exit_status(raw_job_exit_status(job));
}
#[no_mangle]
pub fn job_exit_signal(job: libc::c_int) -> libc::c_int {
    return process_exit_signal(raw_job_exit_status(job));
}

#[no_mangle]
pub fn wait_for(pid: pid_t, flags: libc::c_int) -> libc::c_int {
    unsafe {
        let current_block: u64;
        let mut job: libc::c_int = 0;
        let mut termination_state: libc::c_int = 0;
        let mut r: libc::c_int = 0;
        let mut s: WAIT = 0;
        let mut child: *mut PROCESS = 0 as *mut PROCESS;
        let mut set: sigset_t = __sigset_t { __val: [0; 16] };
        let mut oset: sigset_t = __sigset_t { __val: [0; 16] };
        child = 0 as *mut PROCESS;
        sigemptyset(&mut set);
        sigaddset(&mut set, 17 as libc::c_int);
        sigemptyset(&mut oset);
        sigprocmask(0 as libc::c_int, &mut set, &mut oset);
        child_caught_sigint = 0 as libc::c_int;
        wait_sigint_received = child_caught_sigint;
        if job_control == 0 as libc::c_int || subshell_environment & 0x4 as libc::c_int != 0 {
            //let mut temp_sigint_handler: *mut SigHandler;
            let mut temp_sigint_handler: Option<SigHandler> = None;
            temp_sigint_handler = set_signal_handler(
                SIGINT as libc::c_int,
                //wait_sigint_handler as *mut Option<unsafe extern "C" fn(i32)>,
                ::core::mem::transmute::<Option<fn() -> ()>, Option<SigHandler>>(Some(
                    ::core::mem::transmute::<fn(libc::c_int) -> (), fn() -> ()>(
                        wait_sigint_handler,
                    ),
                )),
            );
            if !(temp_sigint_handler
                == ::core::mem::transmute::<Option<fn() -> ()>, Option<SigHandler>>(Some(
                    ::core::mem::transmute::<fn(libc::c_int) -> (), fn() -> ()>(
                        wait_sigint_handler,
                    ),
                )))
            {
                old_sigint_handler = temp_sigint_handler;
            }
            waiting_for_child = 0;
            if old_sigint_handler
                == ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(
                    1 as libc::c_int as libc::intptr_t,
                )
            {
                set_signal_handler(2 as libc::c_int, old_sigint_handler);
            }
        }
        termination_state = last_command_exit_value;
        if interactive != 0 && job_control == 0 as libc::c_int {
            if terminating_signal != 0 {
                termsig_handler(terminating_signal);
            }
            if interrupt_state != 0 {
                throw_to_top_level();
            }
        }
        if terminating_signal != 0 {
            termsig_handler(terminating_signal);
        }
        if wait_intr_flag != 0
            && wait_signal_received != 0
            && this_shell_builtin.is_some()
            && this_shell_builtin == Some(wait_builtin)
        {
            siglongjmp(wait_intr_buf.as_mut_ptr(), 1 as libc::c_int);
        }
        job = -(1 as libc::c_int);
        loop {
            if pid != -(1 as libc::c_int) {
                child = find_pipeline(
                    pid,
                    0 as libc::c_int,
                    0 as *mut libc::c_void as *mut libc::c_int,
                );
                if child.is_null() {
                    give_terminal_to(shell_pgrp, 0 as libc::c_int);
                    sigprocmask(
                        2 as libc::c_int,
                        &mut oset,
                        0 as *mut libc::c_void as *mut sigset_t,
                    );
                    internal_error(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"wait_for: No record of process %ld\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        pid as libc::c_long,
                    );
                    restore_sigint_handler();
                    termination_state = 127 as libc::c_int;
                    return termination_state;
                }
            }
            if job == -(1 as libc::c_int) && pid != -(1 as libc::c_int) {
                job = find_job(pid, 0 as libc::c_int, 0 as *mut *mut PROCESS);
            }
            if pid == -(1 as libc::c_int)
                || (*child).running == 1 as libc::c_int
                || job != -(1 as libc::c_int)
                    && (**jobs.offset(job as isize)).state as libc::c_int == JRUNNING as libc::c_int
            {
                let mut old_waiting: libc::c_int = 0;
                queue_sigchld = 1 as libc::c_int;
                old_waiting = waiting_for_child;
                waiting_for_child = 1 as libc::c_int;
                if wait_intr_flag != 0
                    && wait_signal_received != 0
                    && this_shell_builtin.is_some()
                    && this_shell_builtin == Some(wait_builtin)
                {
                    siglongjmp(wait_intr_buf.as_mut_ptr(), 1 as libc::c_int);
                }
                r = waitchld(pid, 1 as libc::c_int);
                waiting_for_child = old_waiting;
                queue_sigchld = 0 as libc::c_int;
                if r == -(1 as libc::c_int)
                    && *__errno_location() == 10 as libc::c_int
                    && this_shell_builtin == Some(wait_builtin)
                {
                    termination_state = -(1 as libc::c_int);
                    restore_sigint_handler();
                    current_block = 6718615339517147058;
                    break;
                } else if r == -(1 as libc::c_int) && *__errno_location() == 10 as libc::c_int {
                    if !child.is_null() {
                        (*child).running = 0 as libc::c_int;
                        (*child).status = 0 as libc::c_int;
                    }
                    js.c_living = 0 as libc::c_int;
                    if job != -(1 as libc::c_int) {
                        (**jobs.offset(job as isize)).state = JDEAD;
                        js.c_reaped += 1;
                        js.j_ndead += 1;
                    }
                    if pid == -(1 as libc::c_int) {
                        termination_state = -(1 as libc::c_int);
                        current_block = 7072655752890836508;
                        break;
                    }
                }
            }
            if interactive != 0 && job_control == 0 as libc::c_int {
                if terminating_signal != 0 {
                    termsig_handler(terminating_signal);
                }
                if interrupt_state != 0 {
                    throw_to_top_level();
                }
            }
            if terminating_signal != 0 {
                termsig_handler(terminating_signal);
            }
            if wait_intr_flag != 0
                && wait_signal_received != 0
                && this_shell_builtin.is_some()
                && this_shell_builtin == Some(wait_builtin)
            {
                siglongjmp(wait_intr_buf.as_mut_ptr(), 1 as libc::c_int);
            }
            if pid == -(1 as libc::c_int) {
                restore_sigint_handler();
                current_block = 6718615339517147058;
                break;
            } else if !((*child).running == 1 as libc::c_int
                || job != -(1 as libc::c_int)
                    && (**jobs.offset(job as isize)).state as libc::c_int
                        == JRUNNING as libc::c_int)
            {
                current_block = 7072655752890836508;
                break;
            }
        }
        match current_block {
            7072655752890836508 => {
                restore_sigint_handler();
                termination_state = if job != -(1 as libc::c_int) {
                    job_exit_status(job)
                } else if !child.is_null() {
                    process_exit_status((*child).status)
                } else {
                    0 as libc::c_int
                };
                last_command_exit_signal = if job != -(1 as libc::c_int) {
                    job_exit_signal(job)
                } else if !child.is_null() {
                    process_exit_signal((*child).status)
                } else {
                    0 as libc::c_int
                };
                if job != -(1 as libc::c_int)
                    && (**jobs.offset(job as isize)).state as libc::c_int == JSTOPPED as libc::c_int
                    || !child.is_null()
                        && (*child).status & 0xff as libc::c_int == 0x7f as libc::c_int
                {
                    termination_state = 128 as libc::c_int
                        + (((*child).status & 0xff00 as libc::c_int) >> 8 as libc::c_int);
                }
                if job == -(1 as libc::c_int)
                    || (**jobs.offset(job as isize)).flags & 0x4 as libc::c_int != 0 as libc::c_int
                {
                    if flags & (1 as libc::c_int) << 8 as libc::c_int == 0 as libc::c_int
                        && running_in_background == 0 as libc::c_int
                        && subshell_environment & (0x1 as libc::c_int | 0x10 as libc::c_int)
                            == 0 as libc::c_int
                    {
                        give_terminal_to(shell_pgrp, 0 as libc::c_int);
                    }
                }
                if job != -(1 as libc::c_int) {
                    if interactive_shell != 0 && subshell_environment == 0 as libc::c_int {
                        s = job_signal_status(job);
                        if ((s & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar
                            as libc::c_int
                            >> 1 as libc::c_int
                            > 0 as libc::c_int
                            || s & 0xff as libc::c_int == 0x7f as libc::c_int
                        {
                            set_tty_state();
                            if check_window_size != 0
                                && (job == js.j_current
                                    || (**jobs.offset(job as isize)).flags & 0x1 as libc::c_int
                                        != 0 as libc::c_int)
                            {
                                get_new_window_size(
                                    0 as libc::c_int,
                                    0 as *mut libc::c_int,
                                    0 as *mut libc::c_int,
                                );
                            }
                        } else if rl_readline_state & 0x4000 as libc::c_int as libc::c_ulong
                            == 0 as libc::c_int as libc::c_ulong
                        {
                            get_tty_state();
                        }
                        if job_control != 0
                            && (**jobs.offset(job as isize)).flags & 0x4 as libc::c_int
                                != 0 as libc::c_int
                            && (**jobs.offset(job as isize)).flags & 0x1 as libc::c_int
                                != 0 as libc::c_int
                            && ((s & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar
                                as libc::c_int
                                >> 1 as libc::c_int
                                > 0 as libc::c_int
                            && s & 0x7f as libc::c_int == 2 as libc::c_int
                        {
                            if signal_is_trapped(2 as libc::c_int) == 0 as libc::c_int
                                && (loop_level != 0
                                    || shell_compatibility_level > 32 as libc::c_int
                                        && executing_list != 0)
                            {
                                ::std::ptr::write_volatile(
                                    &mut interrupt_state as *mut sig_atomic_t,
                                    ::std::ptr::read_volatile::<sig_atomic_t>(
                                        &interrupt_state as *const sig_atomic_t,
                                    ) + 1,
                                );
                            } else if signal_is_trapped(2 as libc::c_int) != 0 && loop_level != 0 {
                                ::std::ptr::write_volatile(
                                    &mut interrupt_state as *mut sig_atomic_t,
                                    ::std::ptr::read_volatile::<sig_atomic_t>(
                                        &interrupt_state as *const sig_atomic_t,
                                    ) + 1,
                                );
                            } else if interactive_shell != 0
                                && signal_is_trapped(2 as libc::c_int) == 0 as libc::c_int
                                && sourcelevel != 0
                            {
                                ::std::ptr::write_volatile(
                                    &mut interrupt_state as *mut sig_atomic_t,
                                    ::std::ptr::read_volatile::<sig_atomic_t>(
                                        &interrupt_state as *const sig_atomic_t,
                                    ) + 1,
                                );
                            } else {
                                putchar('\n' as i32);
                                libc::fflush(stdout);
                            }
                        }
                    } else if subshell_environment & (0x4 as libc::c_int | 0x10 as libc::c_int) != 0
                        && wait_sigint_received != 0
                    {
                        if child_caught_sigint == 0 as libc::c_int
                            && signal_is_trapped(2 as libc::c_int) == 0 as libc::c_int
                        {
                            sigprocmask(
                                2 as libc::c_int,
                                &mut oset,
                                0 as *mut libc::c_void as *mut sigset_t,
                            );
                            old_sigint_handler = set_signal_handler(2 as libc::c_int, None);
                            if old_sigint_handler == SIG_IGN!() {
                                restore_sigint_handler();
                            } else {
                                kill(getpid(), 2 as libc::c_int);
                            }
                        }
                    } else if interactive_shell == 0 as libc::c_int
                        && subshell_environment == 0 as libc::c_int
                        && (**jobs.offset(job as isize)).flags & 0x1 as libc::c_int
                            != 0 as libc::c_int
                    {
                        s = job_signal_status(job);
                        if job_control != 0
                            && (**jobs.offset(job as isize)).flags & 0x4 as libc::c_int
                                != 0 as libc::c_int
                            && ((s & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar
                                as libc::c_int
                                >> 1 as libc::c_int
                                > 0 as libc::c_int
                            && s & 0x7f as libc::c_int == 2 as libc::c_int
                        {
                            ::std::ptr::write_volatile(
                                &mut interrupt_state as *mut sig_atomic_t,
                                ::std::ptr::read_volatile::<sig_atomic_t>(
                                    &interrupt_state as *const sig_atomic_t,
                                ) + 1,
                            );
                        }
                        if check_window_size != 0 {
                            get_new_window_size(
                                0 as libc::c_int,
                                0 as *mut libc::c_int,
                                0 as *mut libc::c_int,
                            );
                        }
                    }
                    if (**jobs.offset(job as isize)).state as libc::c_int == JDEAD as libc::c_int
                        && (**jobs.offset(job as isize)).flags & 0x1 as libc::c_int
                            != 0 as libc::c_int
                    {
                        setjstatus(job);
                    }
                    notify_and_cleanup();
                }
            }
            _ => {}
        }
        sigprocmask(
            2 as libc::c_int,
            &mut oset,
            0 as *mut libc::c_void as *mut sigset_t,
        );
        return termination_state;
    }
}

#[no_mangle]
pub fn wait_for_job(job: libc::c_int, flags: libc::c_int, ps: *mut procstat) -> libc::c_int {
    unsafe {
        let mut pid: pid_t = 0;
        let mut r: libc::c_int = 0;
        let mut state: libc::c_int = 0;
        let mut set: sigset_t = __sigset_t { __val: [0; 16] };
        let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

        BLOCK_CHILD(&mut set, &mut oset);
        state = JOBSTATE!(job);
        if state == JSTOPPED as libc::c_int {
            internal_warning(
                b"wait_for_job: job %d is stopped\0" as *const u8 as *const libc::c_char,
                job + 1 as libc::c_int,
            );
        }

        pid = find_last_pid(job, 0);
        UNBLOCK_CHILD(&mut oset);

        loop {
            r = wait_for(pid, 0);
            if r == -1 && errno!() == ECHILD {
                mark_all_jobs_as_dead();
            }

            CHECK_WAIT_INTR!();

            if flags & JWAIT_FORCE as libc::c_int == 0 {
                break;
            }
            BLOCK_CHILD(&mut set, &mut oset);
            state = if job != NO_JOB && !(*jobs.offset(job as isize)).is_null() {
                JOBSTATE!(job)
            } else {
                JDEAD as libc::c_int
            };
            UNBLOCK_CHILD(&mut oset);
            if !(state != JDEAD as libc::c_int) {
                break;
            }
        }

        BLOCK_CHILD(&mut set, &mut oset);
        if job != NO_JOB && !(*jobs.offset(job as isize)).is_null() && DEADJOB!(job) {
            (**jobs.offset(job as isize)).flags |= J_NOTIFIED as libc::c_int;
        }
        UNBLOCK_CHILD(&mut oset);
        if !ps.is_null() {
            (*ps).pid = pid;
            (*ps).status = (if r < 0 { 127 } else { r }) as libc::c_short;
        }
        return r;
    }
}

#[no_mangle]
pub fn wait_for_any_job(flags: libc::c_int, ps: *mut procstat) -> libc::c_int {
    unsafe {
        let mut current_block: u64;
        let mut pid: pid_t = 0;
        let mut i: libc::c_int = 0;
        let mut r: libc::c_int = 0;
        let mut set: sigset_t = __sigset_t { __val: [0; 16] };
        let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

        if jobs_list_frozen != 0 {
            return -1;
        }

        BLOCK_CHILD(&mut set, &mut oset);
        i = 0 as libc::c_int;
        loop {
            if i < js.j_jobslots {
                if !(flags & JWAIT_WAITING as libc::c_int != 0
                    && !(*jobs.offset(i as isize)).is_null()
                    && IS_WAITING!(i) as libc::c_int == 0)
                {
                    if !(*jobs.offset(i as isize)).is_null()
                        && DEADJOB!(i)
                        && IS_NOTIFIED!(i) as libc::c_int == 0
                    {
                        current_block = 2729223887955387488;
                        break;
                    }
                }
                i += 1;
            } else {
                UNBLOCK_CHILD(&mut oset);
                current_block = 7172762164747879670;
                break;
            }
        }
        'c_22951: loop {
            match current_block {
                2729223887955387488 => {
                    r = job_exit_status(i);
                    break;
                }
                _ => {
                    BLOCK_CHILD(&mut set, &mut oset);
                    i = 0 as libc::c_int;
                    while i < js.j_jobslots {
                        if !(*jobs.offset(i as isize)).is_null()
                            && RUNNING!(i)
                            && IS_FOREGROUND!(i) as libc::c_int == 0
                        {
                            break;
                        }
                        i += 1;
                    }
                    if i == js.j_jobslots {
                        UNBLOCK_CHILD(&mut oset);
                        return -1;
                    }
                    UNBLOCK_CHILD(&mut oset);

                    QUIT!();
                    CHECK_TERMSIG!();
                    CHECK_WAIT_INTR!();

                    errno!() = 0;
                    r = wait_for(ANY_PID, 0);
                    if r == -1 && errno!() == ECHILD {
                        mark_all_jobs_as_dead();
                    }
                    BLOCK_CHILD(&mut set, &mut oset);

                    i = 0 as libc::c_int;
                    while i < js.j_jobslots {
                        if !(flags & JWAIT_WAITING as libc::c_int != 0
                            && !(*jobs.offset(i as isize)).is_null()
                            && IS_WAITING!(i) as libc::c_int == 0)
                        {
                            if !(*jobs.offset(i as isize)).is_null() && DEADJOB!(i) {
                                current_block = 2729223887955387488;
                                continue 'c_22951;
                            }
                        }
                        i += 1;
                    }
                    UNBLOCK_CHILD(&mut oset);
                    current_block = 7172762164747879670;
                }
            }
        }
        pid = find_last_pid(i, 0);
        if !ps.is_null() {
            (*ps).pid = pid;
            (*ps).status = r as libc::c_short;
        }
        notify_of_job_status();
        delete_job(i, 0);
        coproc_reap();
        UNBLOCK_CHILD(&mut oset);
        return r;
    }
}

#[no_mangle]
pub fn notify_and_cleanup() {
    unsafe {
        if jobs_list_frozen != 0 {
            return;
        }
        if interactive != 0 || interactive_shell == 0 || sourcelevel != 0 {
            notify_of_job_status();
        }
        cleanup_dead_jobs();
    }
}

#[no_mangle]
pub fn reap_dead_jobs() {
    mark_dead_jobs_as_notified(0);
    cleanup_dead_jobs();
}

fn most_recent_job_in_state(job: libc::c_int, state: JOB_STATE) -> libc::c_int {
    unsafe {
        let mut i: libc::c_int = 0;
        let mut result: libc::c_int = 0;
        let mut set: sigset_t = __sigset_t { __val: [0; 16] };
        let mut oset: sigset_t = __sigset_t { __val: [0; 16] };
        sigemptyset(&mut set);
        sigaddset(&mut set, 17 as libc::c_int);
        sigemptyset(&mut oset);
        sigprocmask(0 as libc::c_int, &mut set, &mut oset);
        result = -(1 as libc::c_int);
        i = job - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            if !(*jobs.offset(i as isize)).is_null()
                && (**jobs.offset(i as isize)).state as libc::c_int == state as libc::c_int
            {
                result = i;
                break;
            } else {
                i -= 1;
            }
        }
        sigprocmask(
            2 as libc::c_int,
            &mut oset,
            0 as *mut libc::c_void as *mut sigset_t,
        );
        return result;
    }
}

fn job_last_stopped(job: libc::c_int) -> libc::c_int {
    return most_recent_job_in_state(job, JSTOPPED as libc::c_int);
}

fn job_last_running(job: libc::c_int) -> libc::c_int {
    return most_recent_job_in_state(job, JRUNNING as libc::c_int);
}

fn set_current_job(job: libc::c_int) {
    let mut candidate: libc::c_int = 0;
    unsafe {
        if js.j_current != job {
            js.j_previous = js.j_current;
            js.j_current = job;
        }

        if js.j_previous != js.j_current
            && js.j_previous != NO_JOB
            && !(*jobs.offset(js.j_previous as isize)).is_null()
            && (**jobs.offset(js.j_previous as isize)).state as libc::c_int
                == JSTOPPED as libc::c_int
        {
            return;
        }
        candidate = NO_JOB;
        if (**jobs.offset(js.j_current as isize)).state as libc::c_int == JSTOPPED as libc::c_int {
            candidate = job_last_stopped(js.j_current);
            if candidate != NO_JOB {
                js.j_previous = candidate;
                return;
            }
        }
        candidate = if (**jobs.offset(js.j_current as isize)).state as libc::c_int
            == JRUNNING as libc::c_int
        {
            job_last_running(js.j_current)
        } else {
            job_last_running(js.j_jobslots)
        };
        if candidate != NO_JOB {
            js.j_previous = candidate;
            return;
        }
        js.j_previous = js.j_current;
    }
}

fn reset_current() {
    let mut candidate: libc::c_int = 0;
    unsafe {
        if js.j_jobslots != 0
            && js.j_current != NO_JOB
            && !(*jobs.offset(js.j_current as isize)).is_null()
            && STOPPED!(js.j_current)
        {
            candidate = js.j_current;
        } else {
            candidate = NO_JOB;

            if js.j_previous != NO_JOB
                && !(*jobs.offset(js.j_previous as isize)).is_null()
                && STOPPED!(js.j_previous)
            {
                candidate = js.j_previous;
            }
            if candidate == NO_JOB {
                candidate = job_last_stopped(js.j_jobslots);
            }
            if candidate == NO_JOB {
                candidate = job_last_running(js.j_jobslots);
            }
        }
        if candidate != NO_JOB {
            set_current_job(candidate);
        } else {
            js.j_previous = NO_JOB;
            js.j_current = js.j_previous;
        };
    }
}

fn set_job_running(job: libc::c_int) {
    let mut p: *mut PROCESS = 0 as *mut PROCESS;
    unsafe {
        p = (**jobs.offset(job as isize)).pipe;
        loop {
            if WIFSTOPPED!((*p).status) {
                (*p).running = PS_RUNNING as libc::c_int;
            }
            p = (*p).next;
            if !(p != (**jobs.offset(job as isize)).pipe) {
                break;
            }
        }
        JOBSTATE!(job) = JRUNNING;
    }
}

#[no_mangle]
pub fn start_job(job: libc::c_int, foreground: libc::c_int) -> libc::c_int {
    let mut p: *mut PROCESS = 0 as *mut PROCESS;
    let mut already_running: libc::c_int = 0;
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };
    let mut wd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut save_stty: libc::termios = libc::termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    unsafe {
        BLOCK_CHILD(&mut set, &mut oset);
        if subshell_environment & SUBSHELL_COMSUB as libc::c_int != 0 && pipeline_pgrp == shell_pgrp
        {
            internal_error(
                b"%s: no current jobs\0" as *const u8 as *const libc::c_char,
                this_command_name,
            );
            UNBLOCK_CHILD(&mut oset);
            return -1;
        }
        if DEADJOB!(job) {
            internal_error(
                b"%s: job has terminated\0" as *const u8 as *const libc::c_char,
                this_command_name,
            );
            UNBLOCK_CHILD(&mut oset);
            return -1;
        }
        already_running = RUNNING!(job) as libc::c_int;

        if foreground == 0 && already_running != 0 {
            internal_error(
                b"%s: job %d already in background\0" as *const u8 as *const libc::c_char,
                this_command_name,
                job + 1 as libc::c_int,
            );
            UNBLOCK_CHILD(&mut oset);
            return 0;
        }

        wd = current_working_directory();

        (**jobs.offset(job as isize)).flags &= !(J_NOTIFIED as libc::c_int);

        if foreground != 0 {
            set_current_job(job);
            (**jobs.offset(job as isize)).flags |= J_FOREGROUND as libc::c_int;
        }

        p = (**jobs.offset(job as isize)).pipe;

        if foreground == 0 {
            if posixly_correct == 0 as libc::c_int {
                s = (if job == js.j_current {
                    b"+ \0" as *const u8 as *const libc::c_char
                } else if job == js.j_previous {
                    b"- \0" as *const u8 as *const libc::c_char
                } else {
                    b" \0" as *const u8 as *const libc::c_char
                }) as *mut libc::c_char;
            } else {
                s = b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            print!(
                "[{}]{}",
                job + 1 as libc::c_int,
                CStr::from_ptr(s).to_str().unwrap()
            );
        }
        loop {
            print!(
                "{}{}",
                if !((*p).command).is_null() {
                    let str1 = (*p).command;
                    CStr::from_ptr(str1).to_str().unwrap()
                } else {
                    let str1 = b"\0" as *const u8 as *const libc::c_char;
                    CStr::from_ptr(str1).to_str().unwrap()
                },
                if (*p).next != (**jobs.offset(job as isize)).pipe {
                    let str1 = b" | \0" as *const u8 as *const libc::c_char;
                    CStr::from_ptr(str1).to_str().unwrap()
                } else {
                    let str1 = b"\0" as *const u8 as *const libc::c_char;
                    CStr::from_ptr(str1).to_str().unwrap()
                },
            );
            p = (*p).next;
            if !(p != (**jobs.offset(job as isize)).pipe) {
                break;
            }
        }
        if foreground == 0 {
            print!(" &");
        }
        if strcmp(wd, (**jobs.offset(job as isize)).wd) != 0 {
            let str1 = polite_directory_format((**jobs.offset(job as isize)).wd);
            print!(" (wd:{})", CStr::from_ptr(str1).to_str().unwrap());
        }
        print!("\n");

        if already_running == 0 {
            set_job_running(job);
        }
        if foreground != 0 {
            get_tty_state();
            save_stty = shell_tty_info;
            if IS_JOBCONTROL!(job) {
                give_terminal_to((**jobs.offset(job as isize)).pgrp, 0);
            }
        } else {
            (**jobs.offset(job as isize)).flags &= !(J_FOREGROUND as libc::c_int);
        }
        if already_running == 0 {
            (**jobs.offset(job as isize)).flags |= J_NOTIFIED as libc::c_int;
            killpg((**jobs.offset(job as isize)).pgrp, 18 as libc::c_int);
        }
        if foreground != 0 {
            let mut pid: pid_t = 0;
            let mut st: libc::c_int = 0;

            pid = find_last_pid(job, 0 as libc::c_int);
            UNBLOCK_CHILD(&mut oset);
            st = wait_for(pid, 0);
            shell_tty_info = save_stty;
            set_tty_state();
            return st;
        } else {
            reset_current();
            UNBLOCK_CHILD(&mut set);
            return 0 as libc::c_int;
        };
    }
}

#[no_mangle]
pub fn kill_pid(mut pid: pid_t, sig: libc::c_int, mut group: libc::c_int) -> libc::c_int {
    unsafe {
        let mut p: *mut PROCESS = 0 as *mut PROCESS;
        let mut job: libc::c_int = 0;
        let mut result: libc::c_int = 0;
        let mut negative: libc::c_int = 0;
        let mut set: sigset_t = __sigset_t { __val: [0; 16] };
        let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

        if pid < -1 {
            pid = -pid;
            negative = 1;
            group = negative;
        } else {
            negative = 0;
        }
        result = 0;
        if group != 0 {
            BLOCK_CHILD(&mut set, &mut oset);
            p = find_pipeline(pid, 0, &mut job);

            if job != NO_JOB {
                (**jobs.offset(job as isize)).flags &= !(J_NOTIFIED as libc::c_int);
                if negative != 0 && (**jobs.offset(job as isize)).pgrp == shell_pgrp {
                    result = killpg(pid, sig);
                } else if (**jobs.offset(job as isize)).pgrp == shell_pgrp {
                    p = (**jobs.offset(job as isize)).pipe;
                    loop {
                        if !(PALIVE!(p) as libc::c_int == 0) {
                            kill((*p).pid, sig);
                            if PEXITED!(p)
                                && (sig == SIGTERM as libc::c_int || sig == SIGHUP as libc::c_int)
                            {
                                kill((*p).pid, SIGCONT as libc::c_int);
                            }
                            p = (*p).next;
                        }
                        if !(p != (**jobs.offset(job as isize)).pipe) {
                            break;
                        }
                    }
                } else {
                    result = killpg((**jobs.offset(job as isize)).pgrp, sig);
                    if !p.is_null()
                        && STOPPED!(job)
                        && (sig == SIGTERM as libc::c_int || sig == SIGHUP as libc::c_int)
                    {
                        killpg((**jobs.offset(job as isize)).pgrp, SIGCONT as libc::c_int);
                    }
                    if !p.is_null() && STOPPED!(job) && sig == SIGCONT as libc::c_int {
                        set_job_running(job);
                        (**jobs.offset(job as isize)).flags &= !(J_FOREGROUND as libc::c_int);
                        (**jobs.offset(job as isize)).flags |= J_NOTIFIED as libc::c_int;
                    }
                }
            } else {
                result = killpg(pid, sig);
            }
            UNBLOCK_CHILD(&mut oset);
        } else {
            result = kill(pid, sig);
        }
        return result;
    }
}

fn sigchld_handler(sig: libc::c_int) {
    let mut n: libc::c_int = 0;
    let mut oerrno: libc::c_int = 0;
    unsafe {
        oerrno = errno!();
        sigchld += 1;
        n = 0;
        if queue_sigchld == 0 {
            n = waitchld(-1, 0);
        }
        errno!() = oerrno;
    }
    return;
}
fn waitchld(wpid: pid_t, block: libc::c_int) -> libc::c_int {
    unsafe {
        let mut status: WAIT = 0;
        let mut child: *mut PROCESS = 0 as *mut PROCESS;
        let mut pid: pid_t = 0;
        let mut ind: libc::c_int = 0;
        let mut call_set_current: libc::c_int = 0;
        let mut last_stopped_job: libc::c_int = 0;
        let mut job: libc::c_int = 0;
        let mut children_exited: libc::c_int = 0;
        let mut waitpid_flags: libc::c_int = 0;
        static mut wcontinued: libc::c_int = 8 as libc::c_int;
        children_exited = 0 as libc::c_int;
        call_set_current = children_exited;
        last_stopped_job = -(1 as libc::c_int);
        loop {
            waitpid_flags = if job_control != 0 && subshell_environment == 0 as libc::c_int {
                2 as libc::c_int | wcontinued
            } else {
                0 as libc::c_int
            };
            if sigchld != 0 || block == 0 as libc::c_int {
                waitpid_flags |= 1 as libc::c_int;
            }
            if terminating_signal != 0 {
                termsig_handler(terminating_signal);
            }
            if wait_intr_flag != 0
                && wait_signal_received != 0
                && this_shell_builtin.is_some()
                && this_shell_builtin == Some(wait_builtin)
            {
                siglongjmp(wait_intr_buf.as_mut_ptr(), 1 as libc::c_int);
            }
            if block == 1 as libc::c_int
                && queue_sigchld == 0 as libc::c_int
                && waitpid_flags & 1 as libc::c_int == 0 as libc::c_int
            {
                internal_warning(dcgettext(
                    0 as *const libc::c_char,
                    b"waitchld: turning on WNOHANG to avoid indefinite block\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ));
                waitpid_flags |= 1 as libc::c_int;
            }
            pid = waitpid(-(1 as libc::c_int), &mut status, waitpid_flags);
            if wcontinued != 0 && pid < 0 as libc::c_int && *__errno_location() == 22 as libc::c_int
            {
                wcontinued = 0 as libc::c_int;
            } else {
                if sigchld > 0 as libc::c_int && waitpid_flags & 1 as libc::c_int != 0 {
                    sigchld -= 1;
                }
                if pid < 0 as libc::c_int && *__errno_location() == 10 as libc::c_int {
                    if !(children_exited == 0 as libc::c_int) {
                        break;
                    }
                    return -(1 as libc::c_int);
                } else {
                    if terminating_signal != 0 {
                        termsig_handler(terminating_signal);
                    }
                    if wait_intr_flag != 0
                        && wait_signal_received != 0
                        && this_shell_builtin.is_some()
                        && this_shell_builtin == Some(wait_builtin)
                    {
                        siglongjmp(wait_intr_buf.as_mut_ptr(), 1 as libc::c_int);
                    }
                    if pid < 0 as libc::c_int
                        && *__errno_location() == 4 as libc::c_int
                        && wait_sigint_received != 0
                    {
                        child_caught_sigint = 1 as libc::c_int;
                    }
                    if !(pid <= 0 as libc::c_int) {
                        if wait_sigint_received != 0
                            && ((((status & 0x7f as libc::c_int) + 1 as libc::c_int)
                                as libc::c_schar as libc::c_int
                                >> 1 as libc::c_int
                                > 0 as libc::c_int) as libc::c_int
                                == 0 as libc::c_int
                                || status & 0x7f as libc::c_int != 2 as libc::c_int)
                        {
                            child_caught_sigint = 1 as libc::c_int;
                        }
                        if ((status & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar
                            as libc::c_int
                            >> 1 as libc::c_int
                            > 0 as libc::c_int
                            && status & 0x7f as libc::c_int == 2 as libc::c_int
                        {
                            child_caught_sigint = 0 as libc::c_int;
                        }
                        if (status == 0xffff as libc::c_int) as libc::c_int == 0 as libc::c_int {
                            children_exited += 1;
                            js.c_living -= 1;
                        }
                        child = find_process(pid, 1 as libc::c_int, &mut job);
                        coproc_pidchk(pid, status);
                        ind = find_procsub_child(pid);
                        if ind >= 0 as libc::c_int {
                            set_procsub_status(ind, pid, status);
                        }
                        if child.is_null() {
                            if status & 0x7f as libc::c_int == 0 as libc::c_int
                                || ((status & 0x7f as libc::c_int) + 1 as libc::c_int)
                                    as libc::c_schar
                                    as libc::c_int
                                    >> 1 as libc::c_int
                                    > 0 as libc::c_int
                            {
                                js.c_reaped += 1;
                            }
                        } else {
                            (*child).status = status;
                            (*child).running = if status == 0xffff as libc::c_int {
                                1 as libc::c_int
                            } else {
                                0 as libc::c_int
                            };
                            if (*child).running == 0 as libc::c_int {
                                js.c_totreaped += 1;
                                if job != -(1 as libc::c_int) {
                                    js.c_reaped += 1;
                                }
                            }
                            if !(job == -(1 as libc::c_int)) {
                                call_set_current += set_job_status_and_cleanup(job);
                                if (**jobs.offset(job as isize)).state as libc::c_int
                                    == JSTOPPED as libc::c_int
                                {
                                    last_stopped_job = job;
                                } else if (**jobs.offset(job as isize)).state as libc::c_int
                                    == JDEAD as libc::c_int
                                    && last_stopped_job == job
                                {
                                    last_stopped_job = -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
            }
            if !((sigchld != 0 || block == 0 as libc::c_int) && pid > 0 as libc::c_int) {
                break;
            }
        }
        if call_set_current != 0 {
            if last_stopped_job != -(1 as libc::c_int) {
                set_current_job(last_stopped_job);
            } else {
                reset_current();
            }
        }
        if children_exited != 0
            && (signal_is_trapped(17 as libc::c_int) != 0
                || *trap_list.as_mut_ptr().offset(17 as libc::c_int as isize)
                    == ::std::mem::transmute::<*mut SigHandler, *mut libc::c_char>(
                        ::std::mem::transmute::<fn() -> (), *mut SigHandler>(initialize_traps),
                    ))
            && *trap_list.as_mut_ptr().offset(17 as libc::c_int as isize)
                != ::std::mem::transmute::<__sighandler_t, *mut libc::c_char>(
                    ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(
                        1 as libc::c_int as libc::intptr_t,
                    ),
                )
        {
            if posixly_correct != 0
                && this_shell_builtin.is_some()
                && this_shell_builtin == Some(wait_builtin)
            {
                queue_sigchld_trap(children_exited);
                wait_signal_received = 17 as libc::c_int;
                if sigchld == 0 as libc::c_int && wait_intr_flag != 0 {
                    siglongjmp(wait_intr_buf.as_mut_ptr(), 1 as libc::c_int);
                }
            } else if sigchld != 0 {
                queue_sigchld_trap(children_exited);
            } else if signal_in_progress(17 as libc::c_int) != 0 {
                queue_sigchld_trap(children_exited);
            } else if *trap_list.as_mut_ptr().offset(17 as libc::c_int as isize)
                == ::std::mem::transmute::<*mut SigHandler, *mut libc::c_char>(
                    ::std::mem::transmute::<fn() -> (), *mut SigHandler>(initialize_traps),
                )
            {
                queue_sigchld_trap(children_exited);
            } else if running_trap != 0 {
                queue_sigchld_trap(children_exited);
            } else if this_shell_builtin == Some(wait_builtin) {
                run_sigchld_trap(children_exited);
            } else {
                queue_sigchld_trap(children_exited);
            }
        }
        if asynchronous_notification != 0
            && interactive != 0
            && executing_builtin == 0 as libc::c_int
        {
            notify_of_job_status();
        }
        return children_exited;
    }
}

fn set_job_status_and_cleanup(job: libc::c_int) -> libc::c_int {
    unsafe {
        let mut child: *mut PROCESS = 0 as *mut PROCESS;
        let mut tstatus: libc::c_int = 0;
        let mut job_state: libc::c_int = 0;
        let mut any_stopped: libc::c_int = 0;
        let mut any_tstped: libc::c_int = 0;
        let mut call_set_current: libc::c_int = 0;
        let mut temp_handler: Option<SigHandler>;

        child = (**jobs.offset(job as isize)).pipe;
        (**jobs.offset(job as isize)).flags &= !(J_NOTIFIED as libc::c_int);

        call_set_current = 0;

        any_tstped = 0;
        any_stopped = any_tstped;
        job_state = any_stopped;
        loop {
            job_state |= PRUNNING!(child) as libc::c_int;
            if PSTOPPED(child) != 0 {
                any_stopped = 1;
                any_tstped |= (job_control != 0
                    && WSTOPSIG!((*child).status) == SIGTSTP as libc::c_int)
                    as libc::c_int;
            }
            child = (*child).next;
            if !(child != (**jobs.offset(job as isize)).pipe) {
                break;
            }
        }

        if job_state != 0 && JOBSTATE!(job) != JSTOPPED as libc::c_int {
            return 0;
        }

        if any_stopped != 0 {
            (**jobs.offset(job as isize)).state = JSTOPPED;
            (**jobs.offset(job as isize)).flags &= !(J_FOREGROUND as libc::c_int);
            call_set_current += 1;

            if any_tstped != 0 && loop_level != 0 {
                breaking = loop_level;
            }
        } else if job_state != 0 {
            (**jobs.offset(job as isize)).state = JRUNNING;
            call_set_current += 1;
        } else {
            (**jobs.offset(job as isize)).state = JDEAD;
            js.j_ndead += 1;

            if ((**jobs.offset(job as isize)).j_cleanup).is_some() {
                (Some(
                    ((**jobs.offset(job as isize)).j_cleanup).expect("non-null function pointer"),
                ))
                .expect("non-null function pointer")(
                    (**jobs.offset(job as isize)).cleanarg
                );

                (**jobs.offset(job as isize)).j_cleanup = ::std::mem::transmute::<
                    *mut libc::c_void,
                    sh_vptrfunc_t,
                >(0 as *mut libc::c_void);
            }
        }
        if JOBSTATE!(job) == JDEAD as libc::c_int {
            if wait_sigint_received != 0
                && interactive_shell == 0
                && child_caught_sigint != 0
                && IS_FOREGROUND!(job)
                && signal_is_trapped(SIGINT as libc::c_int) != 0
            {
                let mut old_frozen: libc::c_int = 0;
                wait_sigint_received = 0;
                last_command_exit_value = process_exit_status((*child).status);

                old_frozen = jobs_list_frozen;
                jobs_list_frozen = 1;
                tstatus = maybe_call_trap_handler(SIGINT as libc::c_int);
                jobs_list_frozen = old_frozen;
            } else if wait_sigint_received != 0
                && child_caught_sigint == 0
                && IS_FOREGROUND!(job)
                && IS_JOBCONTROL!(job) as libc::c_int == 0
            {
                let mut old_frozen_0: libc::c_int = 0;

                wait_sigint_received = 0;

                if signal_is_trapped(SIGINT as libc::c_int) != 0 {
                    last_command_exit_value = process_exit_status((*child).status);
                }
                old_frozen_0 = jobs_list_frozen;
                jobs_list_frozen = 1;
                tstatus = maybe_call_trap_handler(SIGINT as libc::c_int);
                jobs_list_frozen = old_frozen_0;
                if tstatus == 0 && old_sigint_handler != INVALID_SIGNAL_HANDLER!() {
                    temp_handler = old_sigint_handler;
                    if temp_handler == Some(trap_handler)
                        && signal_is_trapped(SIGINT as libc::c_int) == 0
                    {
                        //temp_handler = &mut trap_to_sighandler(SIGINT as libc::c_int);
                        temp_handler = trap_to_sighandler(2 as libc::c_int);
                    }
                    println!("161616161616");
                    restore_sigint_handler();
                    if !temp_handler.is_none() {
                        termsig_handler(SIGINT as libc::c_int);
                    } else if temp_handler != SIG_IGN!() {
                        //这里是函数回调传入参数
                        //(*temp_handler).unwrap()(SIGINT as libc::c_int);
                        (Some(temp_handler.expect("non-null function pointer")))
                            .expect("non-null function pointer")(
                            2 as libc::c_int
                        );
                    }
                }
            }
        }
        return call_set_current;
    }
}

fn setjstatus(j: libc::c_int) {
    unsafe {
        let mut i: libc::c_int = 0;
        let mut p: *mut PROCESS = 0 as *mut PROCESS;

        i = 1;
        p = (**jobs.offset(j as isize)).pipe;
        while (*p).next != (**jobs.offset(j as isize)).pipe {
            p = (*p).next;
            i += 1;
        }
        i += 1;
        if statsize < i {
            pstatuses =
                libc::realloc(pstatuses as *mut c_void, (i * 4) as usize) as *mut libc::c_int; //i * sizeof (int)
            statsize = i;
        }
        i = 0 as libc::c_int;
        p = (**jobs.offset(j as isize)).pipe;
        loop {
            *pstatuses.offset(i as isize) = process_exit_status((*p).status);
            i = i + 1;
            p = (*p).next;
            if !(p != (**jobs.offset(j as isize)).pipe) {
                break;
            }
        }

        *pstatuses.offset(i as isize) = -1;
        set_pipestatus_array(pstatuses, i);
    }
}

#[no_mangle]
pub fn run_sigchld_trap(nchild: libc::c_int) {
    unsafe {
        let trap_command: *mut libc::c_char;
        let mut i: libc::c_int = 0;
        trap_command = savestring!(*trap_list
            .as_mut_ptr()
            .offset(SIGCHLD as libc::c_int as isize));

        begin_unwind_frame(b"SIGCHLD trap\0" as *const u8 as *mut libc::c_char);

        unwind_protect_mem(
            &mut last_command_exit_value as *mut libc::c_int as *mut libc::c_char,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        );
        unwind_protect_mem(
            &mut last_command_exit_signal as *mut libc::c_int as *mut libc::c_char,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        );
        unwind_protect_mem(
            &mut last_made_pid as *mut pid_t as *mut libc::c_char,
            ::std::mem::size_of::<pid_t>() as libc::c_ulong as libc::c_int,
        );
        unwind_protect_mem(
            &mut jobs_list_frozen as *mut libc::c_int as *mut libc::c_char,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        );
        unwind_protect_mem(
            &mut the_pipeline as *mut *mut PROCESS as *mut libc::c_char,
            ::std::mem::size_of::<*mut PROCESS>() as libc::c_ulong as libc::c_int,
        );
        unwind_protect_mem(
            &mut subst_assign_varlist as *mut *mut WordList as *mut libc::c_char,
            ::std::mem::size_of::<*mut WordList>() as libc::c_ulong as libc::c_int,
        );
        // unsafe {
        //     unwind_protect_mem(
        //         &mut this_shell_builtin as *const sh_builtin_func_t as *mut libc::c_char,
        //         ::std::mem::size_of::<Option<sh_builtin_func_t>>() as libc::c_ulong as libc::c_int,
        //     );
        // }
        unwind_protect_mem(
            &mut this_shell_builtin as *mut Option<sh_builtin_func_t> as *mut libc::c_char,
            ::core::mem::size_of::<Option<sh_builtin_func_t>>() as libc::c_ulong as libc::c_int,
        );
        unwind_protect_mem(
            &mut temporary_env as *mut *mut HASH_TABLE as *mut libc::c_char,
            ::std::mem::size_of::<*mut HASH_TABLE>() as libc::c_ulong as libc::c_int,
        );

        extern "C" {
            pub fn xfree(arg1: *mut ::std::os::raw::c_void);
        }

        add_unwind_protect(
            std::mem::transmute::<unsafe extern "C" fn(_), Option<Function>>(free),
            trap_command,
        );
        add_unwind_protect(
            std::mem::transmute::<fn(_), Option<Function>>(maybe_set_sigchld_trap),
            trap_command,
        );

        subst_assign_varlist = 0 as *mut WordList;
        the_pipeline = 0 as *mut PROCESS;
        temporary_env = 0 as *mut HASH_TABLE;

        running_trap = SIGCHLD as libc::c_int + 1;

        set_impossible_sigchld_trap();
        jobs_list_frozen = 1;
        i = 0;
        while i < nchild {
            parse_and_execute(
                savestring!(trap_command),
                b"trap\0" as *const u8 as *const libc::c_char,
                SEVAL_NOHIST | SEVAL_RESETLINE,
            );
            i += 1;
        }
        run_unwind_frame(
            b"SIGCHLD trap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        running_trap = 0 as libc::c_int;
    }
}

fn notify_of_job_status() {
    unsafe {
        let mut job: libc::c_int = 0;
        let mut termsig: libc::c_int = 0;
        let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut set: sigset_t = __sigset_t { __val: [0; 16] };
        let mut oset: sigset_t = __sigset_t { __val: [0; 16] };
        let mut s: WAIT = 0;

        if jobs.is_null() || js.j_jobslots == 0 {
            return;
        }
        if old_ttou.is_some() {
            sigemptyset(&mut set);
            sigaddset(&mut set, SIGCHLD as libc::c_int);
            sigaddset(&mut set, SIGTTOU as libc::c_int);
            sigemptyset(&mut oset);
            sigprocmask(SIG_BLOCK as libc::c_int, &mut set, &mut oset);
        } else {
            queue_sigchld += 1;
        }
        job = 0;
        dir = 0 as *mut libc::c_char;
        while job < js.j_jobslots {
            if !(*jobs.offset(job as isize)).is_null() && IS_NOTIFIED!(job) as libc::c_int == 0 {
                s = raw_job_exit_status(job);
                termsig = WTERMSIG!(s);

                if !(startup_state == 0
                    && WIFSIGNALED!(s) as libc::c_int == 0
                    && (DEADJOB!(job) && IS_FOREGROUND!(job) as libc::c_int == 0 || STOPPED!(job)))
                {
                    if job_control == 0 && interactive_shell != 0
                        || startup_state == 2
                            && subshell_environment & SUBSHELL_COMSUB as libc::c_int != 0
                        || startup_state == 2
                            && posixly_correct != 0
                            && subshell_environment & SUBSHELL_COMSUB as libc::c_int
                                == 0 as libc::c_int
                    {
                        if DEADJOB!(job)
                            && (interactive_shell != 0
                                || find_last_pid(job, 0) != last_asynchronous_pid)
                        {
                            (**jobs.offset(job as isize)).flags |= J_NOTIFIED as libc::c_int;
                        }
                    } else {
                        match (**jobs.offset(job as isize)).state as libc::c_int {
                            JDEAD => {
                                if interactive_shell == 0
                                    && termsig != 0
                                    && WIFSIGNALED!(s)
                                    && termsig != SIGINT as libc::c_int
                                    && termsig != SIGTERM as libc::c_int
                                    && signal_is_trapped(termsig) == 0
                                {
                                    fprintf(
                                        stderr,
                                        b"%s: line %d: \0" as *const u8 as *const libc::c_char,
                                        get_name_for_error(),
                                        if line_number == 0 { 1 } else { line_number },
                                    );
                                    pretty_print_job(
                                        job,
                                        JLIST_NONINTERACTIVE as libc::c_int,
                                        stderr,
                                    );
                                } else if IS_FOREGROUND!(job) {
                                    if termsig != 0
                                        && WIFSIGNALED!(s)
                                        && termsig != SIGINT as libc::c_int
                                        && termsig != SIGPIPE as libc::c_int
                                    {
                                        fprintf(
                                            stderr,
                                            b"%s\0" as *const u8 as *const libc::c_char,
                                            j_strsignal(termsig),
                                        );
                                        if WIFCORED!(s) {
                                            fprintf(
                                                stderr,
                                                b" (core dumped)\0" as *const u8
                                                    as *const libc::c_char,
                                            );
                                        }
                                        fprintf(
                                            stderr,
                                            b"\n\0" as *const u8 as *const libc::c_char,
                                        );
                                    }
                                } else if job_control != 0 {
                                    if dir.is_null() {
                                        dir = current_working_directory();
                                    }
                                    pretty_print_job(job, JLIST_STANDARD as libc::c_int, stderr);
                                    if !dir.is_null()
                                        && strcmp(dir, (**jobs.offset(job as isize)).wd) != 0
                                    {
                                        fprintf(
                                            stderr,
                                            b"(wd now: %s)\n\0" as *const u8 as *const libc::c_char,
                                            polite_directory_format(dir),
                                        );
                                    }
                                }
                                (**jobs.offset(job as isize)).flags |= J_NOTIFIED as libc::c_int;
                            }
                            JSTOPPED => {
                                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                                if dir.is_null() {
                                    dir = current_working_directory();
                                }
                                pretty_print_job(job, JLIST_STANDARD as libc::c_int, stderr);
                                if !dir.is_null()
                                    && strcmp(dir, (**jobs.offset(job as isize)).wd) != 0
                                {
                                    fprintf(
                                        stderr,
                                        b"(wd now: %s)\n\0" as *const u8 as *const libc::c_char,
                                        polite_directory_format(dir),
                                    );
                                }
                                (**jobs.offset(job as isize)).flags |= J_NOTIFIED as libc::c_int;
                            }
                            JRUNNING | JMIXED => {}
                            _ => {
                                programming_error(
                                    b"notify_of_job_status\0" as *const u8 as *const libc::c_char,
                                );
                            }
                        }
                    }
                }
            }
            job += 1;
        }
        if old_ttou.is_some() {
            sigprocmask(
                SIG_SETMASK as libc::c_int,
                &mut oset,
                0 as *mut libc::c_void as *mut sigset_t,
            );
        } else {
            queue_sigchld -= 1;
        };
    }
}

#[no_mangle]
pub fn initialize_job_control(force: libc::c_int) -> libc::c_int {
    unsafe {
        let current_block: u64;
        let mut t: pid_t = 0;
        let mut t_errno: libc::c_int = 0;
        let mut tty_sigs: libc::c_int = 0;

        t_errno = -(1 as libc::c_int);
        shell_pgrp = getpgid(0);

        if shell_pgrp == -(1 as libc::c_int) {
            sys_error(dcgettext(
                0 as *const libc::c_char,
                b"initialize_job_control: getpgrp failed\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ));
            exit(1 as libc::c_int);
        }

        if interactive == 0 as libc::c_int && force == 0 as libc::c_int {
            job_control = 0 as libc::c_int;
            original_pgrp = -(1 as libc::c_int);
            shell_tty = fileno(stderr);
            terminal_pgrp = tcgetpgrp(shell_tty);
        } else {
            shell_tty = -(1 as libc::c_int);
            if forced_interactive != 0 && isatty(fileno(stderr)) == 0 as libc::c_int {
                shell_tty = open(
                    b"/dev/tty\0" as *const u8 as *const libc::c_char,
                    0o2 as libc::c_int | 0o4000 as libc::c_int,
                );
            }
            if shell_tty == -(1 as libc::c_int) {
                shell_tty = dup(fileno(stderr));
            }
            if shell_tty != -(1 as libc::c_int) {
                shell_tty = move_to_high_fd(shell_tty, 1 as libc::c_int, -(1 as libc::c_int));
            }
            if shell_pgrp == 0 as libc::c_int {
                shell_pgrp = getpid();
                setpgid(0 as libc::c_int, shell_pgrp);
                if shell_tty != -(1 as libc::c_int) {
                    tcsetpgrp(shell_tty, shell_pgrp);
                }
            }
            tty_sigs = 0 as libc::c_int;
            loop {
                terminal_pgrp = tcgetpgrp(shell_tty);
                if !(terminal_pgrp != -(1 as libc::c_int)) {
                    current_block = 5529461102203738653;
                    break;
                }
                if !(shell_pgrp != terminal_pgrp) {
                    current_block = 5529461102203738653;
                    break;
                }
                //let mut ottin: *mut SigHandler;
                let mut ottin: Option<SigHandler> = None;
                if terminating_signal != 0 {
                    termsig_handler(terminating_signal);
                }
                ottin = set_signal_handler(21 as libc::c_int, None);
                kill(0 as libc::c_int, 21 as libc::c_int);
                set_signal_handler(21 as libc::c_int, ottin);
                let fresh35 = tty_sigs;
                tty_sigs = tty_sigs + 1;
                if !(fresh35 > 16 as libc::c_int) {
                    continue;
                }
                sys_error(dcgettext(
                    0 as *const libc::c_char,
                    b"initialize_job_control: no job control in background\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ));
                job_control = 0 as libc::c_int;
                original_pgrp = terminal_pgrp;
                current_block = 16073591882049499585;
                break;
            }
            match current_block {
                16073591882049499585 => {}
                _ => {
                    if terminal_pgrp == -(1 as libc::c_int) {
                        t_errno = *__errno_location();
                    }
                    if set_new_line_discipline(shell_tty) < 0 as libc::c_int {
                        sys_error(dcgettext(
                            0 as *const libc::c_char,
                            b"initialize_job_control: line discipline\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ));
                        job_control = 0 as libc::c_int;
                    } else {
                        original_pgrp = shell_pgrp;
                        shell_pgrp = getpid();
                        if original_pgrp != shell_pgrp
                            && setpgid(0 as libc::c_int, shell_pgrp) < 0 as libc::c_int
                        {
                            sys_error(dcgettext(
                                0 as *const libc::c_char,
                                b"initialize_job_control: setpgid\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ));
                            shell_pgrp = original_pgrp;
                        }
                        job_control = 1 as libc::c_int;
                        if shell_pgrp != original_pgrp && shell_pgrp != terminal_pgrp {
                            if give_terminal_to(shell_pgrp, 0 as libc::c_int) < 0 as libc::c_int {
                                t_errno = *__errno_location();
                                setpgid(0 as libc::c_int, original_pgrp);
                                shell_pgrp = original_pgrp;
                                *__errno_location() = t_errno;
                                sys_error(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"cannot set terminal process group (%d)\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    shell_pgrp,
                                );
                                job_control = 0 as libc::c_int;
                            }
                        }
                        if job_control != 0 && {
                            t = tcgetpgrp(shell_tty);
                            t == -(1 as libc::c_int) || t != shell_pgrp
                        } {
                            if t_errno != -(1 as libc::c_int) {
                                *__errno_location() = t_errno;
                            }
                            sys_error(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"cannot set terminal process group (%d)\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                t,
                            );
                            job_control = 0 as libc::c_int;
                        }
                    }
                    if job_control == 0 as libc::c_int {
                        internal_error(dcgettext(
                            0 as *const libc::c_char,
                            b"no job control in this shell\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ));
                    }
                }
            }
        }
        running_in_background = (terminal_pgrp != shell_pgrp) as libc::c_int;
        if shell_tty != fileno(stderr) {
            fcntl(shell_tty, 2 as libc::c_int, 1 as libc::c_int);
        }
        set_signal_handler(
            17 as libc::c_int,
            //sigchld_handler as *mut Option<unsafe extern "C" fn(i32)>,
            std::mem::transmute(sigchld_handler as usize),
        );
        change_flag(
            'm' as i32,
            if job_control != 0 {
                '-' as i32
            } else {
                '+' as i32
            },
        );
        if interactive != 0 {
            get_tty_state();
        }
        set_maxchild(0 as libc::c_int);
        return job_control;
    }
}

fn set_new_line_discipline(tty: libc::c_int) -> libc::c_int {
    return 0 as libc::c_int;
}

#[no_mangle]
pub fn initialize_job_signals() {
    unsafe {
        if interactive != 0 {
            set_signal_handler(
                2 as libc::c_int,
                std::mem::transmute(sigint_sighandler as usize),
            );
            set_signal_handler(SIGTSTP as libc::c_int, SIG_IGN!());
            set_signal_handler(SIGTTOU as libc::c_int, SIG_IGN!());
            set_signal_handler(SIGTTIN as libc::c_int, SIG_IGN!());
        } else if job_control != 0 {
            old_tstp = set_signal_handler(SIGTSTP as libc::c_int, Some(sigstop_sighandler));
            old_ttin = set_signal_handler(SIGTTIN as libc::c_int, Some(sigstop_sighandler));
            old_ttou = set_signal_handler(SIGTTOU as libc::c_int, Some(sigstop_sighandler));
        }
    }
}

fn sigcont_sighandler(sig: libc::c_int) {
    unsafe {
        initialize_job_signals();
        set_signal_handler(SIGCONT as libc::c_int, old_cont);
        kill(getpid(), SIGCONT as libc::c_int);
    }
}
fn sigstop_sighandler(sig: libc::c_int) {
    unsafe {
        set_signal_handler(SIGTSTP as libc::c_int, old_tstp);
        set_signal_handler(SIGTTOU as libc::c_int, old_ttou);
        set_signal_handler(SIGTTIN as libc::c_int, old_ttin);
        old_cont = set_signal_handler(
            SIGCONT as libc::c_int,
            ::core::mem::transmute::<Option<fn() -> ()>, Option<SigHandler>>(Some(
                ::core::mem::transmute::<fn(libc::c_int) -> (), fn() -> ()>(sigcont_sighandler),
            )),
        );
        give_terminal_to(shell_pgrp, 0);
        kill(getpid(), sig);
    }
}

#[no_mangle]
pub fn give_terminal_to(pgrp: pid_t, force: libc::c_int) -> libc::c_int {
    unsafe {
        let mut set: sigset_t = __sigset_t { __val: [0; 16] };
        let mut oset: sigset_t = __sigset_t { __val: [0; 16] };
        let mut r: libc::c_int = 0;
        let mut e: libc::c_int = 0;

        if job_control != 0 || force != 0 {
            sigemptyset(&mut set);
            sigaddset(&mut set, SIGTTOU as libc::c_int);
            sigaddset(&mut set, SIGTTIN as libc::c_int);
            sigaddset(&mut set, SIGTSTP as libc::c_int);
            sigaddset(&mut set, SIGCHLD as libc::c_int);
            sigemptyset(&mut oset);
            sigprocmask(SIG_BLOCK as libc::c_int, &mut set, &mut oset);

            if tcsetpgrp(shell_tty, pgrp) < 0 {
                r = -1;
                e = errno!();
            } else {
                terminal_pgrp = pgrp;
            }
            sigprocmask(SIG_SETMASK as libc::c_int, &mut oset, 0 as *mut sigset_t);
        }
        if r == -1 {
            errno!() = e;
        }
        return r;
    }
}

fn maybe_give_terminal_to(opgrp: pid_t, npgrp: pid_t, flags: libc::c_int) -> libc::c_int {
    unsafe {
        let mut tpgrp: libc::c_int = 0;

        tpgrp = tcgetpgrp(shell_tty);
        if tpgrp < 0 && errno!() == ENOTTY {
            return -1;
        }
        if tpgrp == npgrp {
            terminal_pgrp = npgrp;
            return 0;
        } else if tpgrp != opgrp {
            return -1;
        } else {
            return give_terminal_to(npgrp, flags);
        };
    }
}

#[no_mangle]
pub fn delete_all_jobs(running_only: libc::c_int) {
    unsafe {
        let mut i: libc::c_int = 0;
        let mut set: sigset_t = __sigset_t { __val: [0; 16] };
        let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

        BLOCK_CHILD(&mut set, &mut oset);
        if js.j_jobslots != 0 {
            js.j_previous = NO_JOB;
            js.j_current = js.j_previous;
            i = 0;
            while i < js.j_jobslots {
                if !(*jobs.offset(i as isize)).is_null()
                    && (running_only == 0 || running_only != 0 && RUNNING!(i))
                {
                    delete_job(
                        i,
                        DEL_WARNSTOPPED as libc::c_int | DEL_NOBGPID as libc::c_int,
                    );
                }
                i += 1;
            }
            if running_only == 0 {
                libc::free(jobs as *mut c_void);

                js.j_jobslots = 0;
                js.j_njobs = 0;
                js.j_lastj = js.j_njobs;
                js.j_firstj = js.j_lastj;
            }
        }
        if running_only == 0 {
            bgp_clear();
        }
        UNBLOCK_CHILD(&mut oset);
    }
}

#[no_mangle]
pub fn nohup_all_jobs(running_only: libc::c_int) {
    unsafe {
        let mut i: libc::c_int = 0;
        let mut set: sigset_t = __sigset_t { __val: [0; 16] };
        let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

        BLOCK_CHILD(&mut set, &mut oset);
        if js.j_jobslots != 0 {
            i = 0;
            while i < js.j_jobslots {
                if !(*jobs.offset(i as isize)).is_null()
                    && (running_only == 0 || running_only != 0 && RUNNING!(i))
                {
                    nohup_job(i);
                }
                i += 1;
            }
        }
        UNBLOCK_CHILD(&mut oset);
    }
}

#[no_mangle]
pub fn count_all_jobs() -> libc::c_int {
    unsafe {
        let mut i: libc::c_int = 0;
        let mut n: libc::c_int = 0;
        let mut set: sigset_t = __sigset_t { __val: [0; 16] };
        let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

        BLOCK_CHILD(&mut set, &mut oset);
        n = 0;
        i = n;
        while i < js.j_jobslots {
            if !(*jobs.offset(i as isize)).is_null()
                && DEADJOB!(i) as libc::c_int == 0 as libc::c_int
            {
                n += 1;
            }
            i += 1;
        }
        UNBLOCK_CHILD(&mut oset);
        return n;
    }
}

fn mark_all_jobs_as_dead() {
    unsafe {
        let mut i: libc::c_int = 0;
        let mut set: sigset_t = __sigset_t { __val: [0; 16] };
        let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

        if js.j_jobslots == 0 {
            return;
        }
        BLOCK_CHILD(&mut set, &mut oset);
        i = 0;
        while i < js.j_jobslots {
            if !(*jobs.offset(i as isize)).is_null() {
                (**jobs.offset(i as isize)).state = JDEAD;
                js.j_ndead += 1;
            }
            i += 1;
        }
        UNBLOCK_CHILD(&mut oset);
    }
}

fn mark_dead_jobs_as_notified(force: libc::c_int) {
    unsafe {
        let mut i: libc::c_int = 0;
        let mut ndead: libc::c_int = 0;
        let mut ndeadproc: libc::c_int = 0;
        let mut set: sigset_t = __sigset_t { __val: [0; 16] };
        let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

        if js.j_jobslots == 0 {
            return;
        }
        BLOCK_CHILD(&mut set, &mut oset);
        if force != 0 {
            i = 0;
            while i < js.j_jobslots {
                if !(*jobs.offset(i as isize)).is_null()
                    && DEADJOB!(i)
                    && (interactive_shell != 0
                        || find_last_pid(i, 0 as libc::c_int) != last_asynchronous_pid)
                {
                    (**jobs.offset(i as isize)).flags |= J_NOTIFIED as libc::c_int;
                }
                i += 1;
            }
            UNBLOCK_CHILD(&mut oset);
            return;
        }
        ndeadproc = 0;
        ndead = ndeadproc;
        i = ndead;
        while i < js.j_jobslots {
            if !(*jobs.offset(i as isize)).is_null() && DEADJOB!(i) {
                ndead += 1;
                ndeadproc += processes_in_job(i);
            }
            i += 1;
        }
        if js.c_childmax < 0 {
            set_maxchild(0);
        }
        if ndeadproc as libc::c_long <= js.c_childmax {
            UNBLOCK_CHILD(&mut oset);
            return;
        }
        i = 0;
        while i < js.j_jobslots {
            if !(*jobs.offset(i as isize)).is_null()
                && DEADJOB!(i)
                && (interactive_shell != 0
                    || find_last_pid(i, 0 as libc::c_int) != last_asynchronous_pid)
            {
                ndeadproc -= processes_in_job(i);
                if ndeadproc as libc::c_long <= js.c_childmax {
                    break;
                }
                (**jobs.offset(i as isize)).flags |= J_NOTIFIED as libc::c_int;
            }
            i += 1;
        }
        UNBLOCK_CHILD(&mut oset);
    }
}

#[no_mangle]
pub fn freeze_jobs_list() -> libc::c_int {
    unsafe {
        let mut o: libc::c_int = 0;

        o = jobs_list_frozen;
        jobs_list_frozen = 1;
        return o;
    }
}

#[no_mangle]
pub fn unfreeze_jobs_list() {
    unsafe {
        jobs_list_frozen = 0;
    }
}

#[no_mangle]
pub fn set_jobs_list_frozen(s: libc::c_int) {
    unsafe {
        jobs_list_frozen = s;
    }
}

#[no_mangle]
pub fn set_job_control(arg: libc::c_int) -> libc::c_int {
    unsafe {
        let mut old: libc::c_int = 0;

        old = job_control;
        job_control = arg;

        if terminal_pgrp == NO_PID as libc::c_int {
            terminal_pgrp = tcgetpgrp(shell_tty);
        }
        if job_control != old && job_control != 0 {
            shell_pgrp = getpgid(0);
        }
        running_in_background = (terminal_pgrp != shell_pgrp) as libc::c_int;

        if job_control != old && job_control != 0 {
            pipeline_pgrp = 0;
        }
        return old;
    }
}

#[no_mangle]
pub fn without_job_control() {
    unsafe {
        stop_making_children();
        start_pipeline();
        sh_closepipe(pgrp_pipe.as_mut_ptr());
        delete_all_jobs(0);
        set_job_control(0);
    }
}
#[no_mangle]
pub fn end_job_control() {
    unsafe {
        if job_control != 0 {
            terminate_stopped_jobs();
        }
        if original_pgrp >= 0 as libc::c_int && terminal_pgrp != original_pgrp {
            give_terminal_to(original_pgrp, 1);
        }
        if original_pgrp >= 0 && setpgid(0, original_pgrp) == 0 {
            shell_pgrp = original_pgrp;
        }
    }
}
#[no_mangle]
pub fn restart_job_control() {
    unsafe {
        if shell_tty != -1 {
            libc::close(shell_tty);
        }
    }
    initialize_job_control(0);
}

#[no_mangle]
pub fn set_maxchild(mut nchild: libc::c_int) {
    static mut lmaxchild: libc::c_int = -1;
    unsafe {
        if lmaxchild < 0 {
            errno!() = 0;
            lmaxchild = getmaxchild() as libc::c_int;
            if lmaxchild < 0 && *__errno_location() == 0 {
                lmaxchild = MAX_CHILD_MAX as libc::c_int;
            }
        }
        if lmaxchild < 0 {
            lmaxchild = DEFAULT_CHILD_MAX as libc::c_int;
        }
        if nchild < lmaxchild {
            nchild = lmaxchild;
        } else if nchild > MAX_CHILD_MAX as libc::c_int {
            nchild = MAX_CHILD_MAX as libc::c_int;
        }
        js.c_childmax = nchild as libc::c_long;
    }
}

#[no_mangle]
pub fn set_sigchld_handler() {
    unsafe {
        set_signal_handler(
            SIGCHLD as libc::c_int,
            ::core::mem::transmute::<Option<fn() -> ()>, Option<SigHandler>>(Some(
                ::core::mem::transmute::<fn(libc::c_int) -> (), fn() -> ()>(sigchld_handler),
            )),
        );
    }
}

fn pipe_read(pp: *mut libc::c_int) {
    let mut ch: libc::c_char = 0;
    unsafe {
        if *pp.offset(1 as isize) >= 0 {
            libc::close(*pp.offset(1 as libc::c_int as isize));
            *pp.offset(1 as isize) = -(1 as libc::c_int);
        }
        if *pp.offset(0 as libc::c_int as isize) >= 0 {
            while libc::read(
                *pp.offset(0 as libc::c_int as isize),
                &mut ch as *mut libc::c_char as *mut libc::c_void,
                1,
            ) == -1
                && errno!() == EINTR as libc::c_int
            {}
        }
    }
}

#[no_mangle]
pub fn close_pgrp_pipe() {
    unsafe {
        sh_closepipe(pgrp_pipe.as_mut_ptr());
    }
}

#[no_mangle]
pub fn save_pgrp_pipe(p: *mut libc::c_int, clear: libc::c_int) {
    unsafe {
        *p.offset(0 as libc::c_int as isize) = pgrp_pipe[0 as libc::c_int as usize];
        *p.offset(1 as libc::c_int as isize) = pgrp_pipe[1 as libc::c_int as usize];
        if clear != 0 {
            pgrp_pipe[1] = -1;
            pgrp_pipe[0] = pgrp_pipe[1];
        }
    }
}
#[no_mangle]
pub fn restore_pgrp_pipe(p: *mut libc::c_int) {
    unsafe {
        pgrp_pipe[0] = *p.offset(0);
        pgrp_pipe[1] = *p.offset(1 as libc::c_int as isize);
    }
}
