//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

use std::mem::size_of;

use libc::{c_void, c_int, c_char, c_long, WIFSIGNALED, WTERMSIG, WEXITSTATUS, WIFSTOPPED,};
use r_bash::*;

#[macro_export]
macro_rules! DEFAULT_CHILD_MAX {
    () => {
        4096
    };
}


#[macro_export]
macro_rules! killpg {
    ($pg:expr,$sig:expr) => {
        kill(-($pg), ($sig))
    };
}

#[macro_export]
macro_rules! WAITPID {
    ($pid:expr,$statusp:expr, $options:expr) => {
        waitpid($pid, $statusp, $options)
    };
}


#[macro_export]
macro_rules! input_tty {
    () => {
        if shell_tty != -1{
            shell_tty
        } else{
            fileno(stderr)
        }  
   };
}

#[macro_export]
macro_rules! NO_PID {
    () => {
        -1 as pid_t
    };
}

#[no_mangle]
pub static mut last_made_pid:pid_t = NO_PID!();
#[no_mangle]
pub static mut last_asynchronous_pid:pid_t = NO_PID!();
#[no_mangle]
pub static mut queue_sigchld:c_int = 0;
#[no_mangle]
pub static mut waiting_for_child:c_int = 0;
#[no_mangle]
pub static mut already_making_children:c_int = 0;
#[no_mangle]
pub static mut shell_tty:c_int = -1;
#[no_mangle]
pub static mut check_window_size:c_int = CHECKWINSIZE_DEFAULT as c_int ;
#[no_mangle]
pub static mut job_control:c_int = 0;
#[no_mangle]
pub static mut running_in_background:c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_status {
    pid:pid_t,
    status:c_int,
    flags:c_int,
}

#[macro_export]
macro_rules! PROC_RUNNING {
    () => {
        0x01
    };
}

#[macro_export]
macro_rules! PROC_NOTIFIED {
    () => {
        0x02
    };
}

#[macro_export]
macro_rules! PROC_ASYNC {
    () => {
        0x04
    };
}

#[macro_export]
macro_rules! PROC_SIGNALED {
    () => {
        0x10
    };
}

#[macro_export]
macro_rules! PROC_BAD {
    () => {
        -1
    };
}

#[macro_export]
macro_rules! PROC_STILL_ALIVE {
    () => {
        -2
    };
}

#[no_mangle]
pub static mut pid_list:*mut proc_status = 0 as *mut proc_status;

#[no_mangle]
pub static mut pid_list_size:c_int = 0; 
#[no_mangle]
pub static mut wait_sigint_received:c_int = 0; 
#[no_mangle]
pub static mut child_max:c_long = -1; 


unsafe extern "C" fn alloc_pid_list()
{
    let mut i:c_int;
    let mut old:c_int = pid_list_size;

    pid_list_size += 10;
    pid_list = libc::realloc(pid_list as *mut c_void, (pid_list_size * size_of::<proc_status>() as c_int)  as usize ) as *mut proc_status;

    i = old;
    while i < pid_list_size
    {
        (*pid_list.offset(i as isize)).pid = NO_PID!();
        (*pid_list.offset(i as isize)).flags = 0;
        (*pid_list.offset(i as isize)).status = 0;
        i += 1; 
    }
}

unsafe extern "C" fn find_proc_slot(pid:pid_t) -> c_int
{
    let mut i:c_int;

    i=0;
    while i < pid_list_size
    {
        if (*pid_list.offset(i as isize)).pid == NO_PID!()
         ||(*pid_list.offset(i as isize)).pid == pid{
             return i;
         }
        i += 1;
    } 

    if i == pid_list_size{
        alloc_pid_list();
    }

    return i;
}

unsafe extern "C" fn find_index_by_pid(pid:pid_t) -> c_int
{
    let mut i:c_int;

    i = find_index_by_pid(pid);
    if i == NO_PID!(){
        return PROC_BAD!();
    }

    if (*pid_list.offset(i as isize)).flags & PROC_RUNNING!() != 0{
        return PROC_STILL_ALIVE!();
    }

    return (*pid_list.offset(i as isize)).status;
}


unsafe extern "C" fn process_exit_status(status:WAIT) -> c_int
{
    if WIFSIGNALED(status){
        return (128 + WTERMSIG(status));
    } else{
        return WEXITSTATUS(status);
    }

}

unsafe extern "C" fn find_termsig_by_pid(pid:pid_t) -> c_int
{
    let mut i:c_int;

    i = find_index_by_pid(pid);
    
    if i == NO_PID!(){
        return 0;
    }

    if (*pid_list.offset(i as isize)).flags & PROC_RUNNING!() != 0{
        return 0;
    }

    return get_termsig((*pid_list.offset(i as isize)).status as WAIT)
}

unsafe extern "C" fn get_termsig(status:WAIT) -> c_int
{
    if WIFSTOPPED(status) as c_int == 0 && WIFSIGNALED(status) {
        return WTERMSIG(status);
    } else {
        return 0;
    }

}

#[macro_export]
macro_rules! WSTATUS  {
    ($t:expr) => {
       $t 
    };
}

unsafe extern "C" fn set_pid_status (pid:pid_t, status:WAIT)
{
    let mut slot:c_int;

    coproc_pidchk(pid,status);

    slot = find_procsub_child(pid);
    if slot >= 0{
        set_procsub_status(slot, pid, WSTATUS!(status))
    }

    slot = find_index_by_pid(pid);
    if slot == NO_PID!(){
        return 
    }

    (*pid_list.offset(slot as isize)).status = process_exit_status(status);
    (*pid_list.offset(slot as isize)).flags &= !PROC_SIGNALED!();

    if WIFSIGNALED(status){
        (*pid_list.offset(slot as isize)).flags |= PROC_SIGNALED!();
    } 

    if (*pid_list.offset(slot as isize)).flags & PROC_ASYNC!() == 0{
        (*pid_list.offset(slot as isize)).flags |= PROC_NOTIFIED!();
    }
}


unsafe extern "C" fn set_pid_flags(pid:pid_t, flags:c_int)
{
    let mut slot:c_int;

    slot = find_index_by_pid(pid);
    if slot == NO_PID!(){
        return
    }

    (*pid_list.offset(slot as isize)).flags |= flags;
}

unsafe extern "C" fn unset_pid_flags (pid:pid_t, flags:c_int)
{
    let mut slot:c_int;

    slot = find_index_by_pid(pid);
    if slot == NO_PID!(){
        return
    }

    (*pid_list.offset(slot as isize)).flags &= !flags;
}

unsafe extern "C" fn get_pid_flags (pid:pid_t) -> c_int
{
    let mut slot:c_int;
    slot = find_index_by_pid (pid);
    if slot == NO_PID!(){
        return 0
    }
    return (*pid_list.offset(slot as isize)).flags;
}

unsafe extern "C" fn add_pid (pid:pid_t, async_:c_int)
{
    let mut slot:c_int;

    slot = find_proc_slot (pid);

    (*pid_list.offset(slot as isize)).pid = pid;
    (*pid_list.offset(slot as isize)).status = -1;
    (*pid_list.offset(slot as isize)).flags = PROC_RUNNING!();

    if async_ != 0{
        (*pid_list.offset(slot as isize)).flags |= PROC_ASYNC!();
    }
}

unsafe extern "C" fn mark_dead_jobs_as_notified (force:c_int)
{
    let mut i:c_int;
    let mut ndead:c_int;

    i = 0;
    ndead = 0;
    while force == 0 && i < pid_list_size{
        if (*pid_list.offset(i as isize)).pid == NO_PID!() {
            continue;
        }

        if (*pid_list.offset(i as isize)).flags & PROC_RUNNING!() == 0
          && (*pid_list.offset(i as isize)).flags & PROC_ASYNC!() != 0{
            ndead += 1;
          }

        i += 1;
    }

    if child_max < 0{
        child_max = getmaxchild();
    }
    if child_max < 0{
        child_max = DEFAULT_CHILD_MAX!();
    }

    if force == 0 && ndead <= child_max as c_int{
        return
    }

    i = 0;
    while i < pid_list_size{
        if (*pid_list.offset(i as isize)).pid == NO_PID!(){
            continue;
        }
        if (*pid_list.offset(i as isize)).flags & PROC_RUNNING!() == 0
          && (*pid_list.offset(i as isize)).pid != last_asynchronous_pid { 
            (*pid_list.offset(i as isize)).flags |= PROC_NOTIFIED!();

            ndead -= 1;
            if force == 0 && (*pid_list.offset(i as isize)).flags & PROC_ASYNC!() != 0
               &&  ndead <= child_max as c_int{
                   break;
               }
         }

        i += 1;
    }

}


  








