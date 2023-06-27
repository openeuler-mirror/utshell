extern crate libc;
extern crate rjobs;
extern crate nix;

use libc::c_char;
use std::ffi::CString;

use rjobs::{PROCESS, COMMAND, r_jobs_builtin, JLIST_STANDARD, WORD_LIST};

#[repr(C)]
pub struct JOB {
    wd: *mut c_char,
    pipe: *mut PROCESS,
    pgrp:i32,
    state:JOB_STATE,
    flags:i32,
    deferred:*mut COMMAND,
    j_cleanup:*mut fn(),
    cleanarg:* mut fn()
}

#[repr(C)]
pub struct jobstats {
    /* limits */
    c_childmax:libc::c_long,
    /* child process statistics */
    c_living:libc::c_int,       /* running or stopped child processes */
    c_reaped:libc::c_int,   /* exited child processes still in jobs list */
    c_injobs:libc::c_int,   /* total number of child processes in jobs list */
    /* child process totals */
    c_totforked:libc::c_int,    /* total number of children this shell has forked */
    c_totreaped:libc::c_int,    /* total number of children this shell has reaped */
    /* job counters and indices */
    j_jobslots:libc::c_int,/* total size of jobs array */
    j_lastj:libc::c_int,        /* last (newest) job allocated */
    j_firstj:libc::c_int,   /* first (oldest) job allocated */
    j_njobs:libc::c_int,        /* number of non-NULL jobs in jobs array */
    j_ndead:libc::c_int,        /* number of JDEAD jobs in jobs array */
    /* */
    j_current:libc::c_int,  /* current job */
    j_previous:libc::c_int, /* previous job */
    /* */
    j_lastmade:* mut JOB,   /* last job allocated by stop_pipeline */
    j_lastasync:* mut JOB   /* last async job allocated by stop_pipeline */
}


//枚举
#[repr(i8)]
#[derive(PartialEq)]
pub enum JOB_STATE {
    JNONE = -1,
    JRUNNING = 1,
    JSTOPPED = 2,
    JDEAD = 4,
    JMIXED = 8
}

//宏
#[macro_export]
macro_rules! EX_USAGE {
    () => { 258 }
}

#[macro_export]
macro_rules! EXECUTION_SUCCESS {
    () => { 0 }
}

#[macro_export]
macro_rules! EXECUTION_FAILURE{
    () => { 1 }
}

#[macro_export]
macro_rules! get_job_by_jid {
   ($ind:expr) => {
    (*((jobs as usize + ($ind*8) as usize ) as *mut*mut JOB) as *mut JOB)
    }
}

#[macro_export]
macro_rules! STOPPED {
    ($j:expr) => {
        (*get_job_by_jid!($j)).state == JOB_STATE::JSTOPPED
    }
}

#[macro_export]
macro_rules! RUNNING{
    ($j:expr) => {
        (*get_job_by_jid!($j)).state == JOB_STATE::JRUNNING
    }
}

#[macro_export]
macro_rules! EXITPROG{
    () => { 3 }
}

#[macro_export]
macro_rules! SYS_BASH_LOGOOUT {
    () => {
        CString::new(" \"/etc/bash.bash_logout\" ").unwrap().as_ptr()
    }
}

//C库
extern "C"{
    static mut interactive:i32;
    static mut login_shell:i32;
    // static mut last_shell_builtin:*mut fn(v:*mut WORD_LIST)->i32;
    static mut last_shell_builtin:extern  fn(v:*mut WORD_LIST)->i32;
    // static mut this_shell_builtin:*mut fn(v:*mut WORD_LIST)->i32;
    static mut this_shell_builtin:extern fn(v:*mut WORD_LIST)->i32;
    static  js:jobstats ;
    static mut check_jobs_at_exit:i32;
    static mut jobs:*mut*mut JOB;
    static mut running_trap:i32;
    static mut trap_saved_exit_value:i32;
    static mut last_command_exit_value:i32;
    static subshell_environment:i32;

    fn builtin_help();
    fn builtin_error(err:*const c_char,...);
    fn list_all_jobs(form:i32);
    fn get_exitstat(list:*mut WORD_LIST) -> i32;
    fn jump_to_top_level(level:i32);
    fn maybe_execute_file(fname:*const c_char,force_noninteractive:i32)->i32;
}

unsafe fn STREQ(a:*const c_char,b:*const c_char)->bool{
    return *a == *b && libc::strcmp(a,b) == 0;
}

// unsafe fn printToStderr(str:*mut c_char) -> std::io::Result<()>{
//     let stderr = std::io::stderr();
//     let mut handle = stderr.lock();
//     handle.write_all(std::ffi::CStr::from_ptr(str).to_bytes())?;
//     Ok(())
// }

//
static mut sourced_logout:i32 = 0;

#[no_mangle]
pub extern "C" fn r_exit_builtin(list:*mut WORD_LIST) -> i32{
    println!("r_exit_builtin");
    unsafe{
        if list != std::ptr::null_mut() && (*list).word != std::ptr::null_mut() && 
           STREQ((*(*list).word).word, CString::new("--help").unwrap().as_ptr()){
               builtin_help();
               return EX_USAGE!();
        }

        if interactive != 0 {
            if login_shell != 0 {
                // let str:*mut c_char = CString::new("logout\n").unwrap().into_raw();
                // printToStderr(str);
                eprintln!("logout");
            }else{
                // let str:*mut c_char = CString::new("exit\n").unwrap().into_raw();
                // printToStderr(str);
                eprintln!("exit");
                // libc::fprintf(stderr,CString::new("exit\n").unwrap().as_ptr());
            }
            
        }
        // libc::fflush(stderr);
        return r_exit_or_logout(list);
    }
}

#[no_mangle]
pub extern "C" fn  r_logout_builtin(list:*mut WORD_LIST)->i32{
    unsafe{
        if list != std::ptr::null_mut() && (*list).word != std::ptr::null_mut() && 
           STREQ((*(*list).word).word, CString::new("--help").unwrap().as_ptr()){
               builtin_help();
               return EX_USAGE!();
        }

        if login_shell == 0{
            builtin_error(CString::new("not login shell: use `exit'").unwrap().as_ptr());
            return EXECUTION_FAILURE!();
        }else{
            return r_exit_or_logout(list)
        }
    }
}

#[no_mangle]
pub extern "C" fn r_exit_or_logout(list:*mut WORD_LIST)->i32{
    // let stream:*mut libc::FILE;
    let  exit_value:i32;
    let  exit_immediate_okay:i32;
    let mut stopmsg:i32;

    unsafe{
        exit_immediate_okay =   (interactive == 0 || 
                                last_shell_builtin == r_exit_builtin ||
                                last_shell_builtin == r_logout_builtin || 
                                last_shell_builtin == r_jobs_builtin ) as i32;

        /* Check for stopped jobs if thw user wants to.*/
        if exit_immediate_okay == 0 {
            stopmsg = 0;
            for i in 0..js.j_jobslots {
                if get_job_by_jid!(i) != std::ptr::null_mut()  && STOPPED!(i){
                    stopmsg = JOB_STATE::JSTOPPED as i32;
                }
                else if (check_jobs_at_exit != 0)  && (stopmsg ==0) && get_job_by_jid!(i) != std::ptr::null_mut() && RUNNING!(i) {
                    stopmsg = JOB_STATE::JRUNNING as i32;
                }
            }

            if stopmsg == JOB_STATE::JSTOPPED as i32 {
                // libc::fprintf(stream,CString::new("There are stopped jobs. \n").unwrap().as_ptr());
                eprintln!("There are stopped jobs. ");
            }
            else if stopmsg == JOB_STATE::JRUNNING as i32{
                // libc::fprintf(stream,CString::new("There are runing jobs.\n").unwrap().as_ptr());
                eprintln!("There are runing jobs.");
            }

            if stopmsg == check_jobs_at_exit{
                list_all_jobs(JLIST_STANDARD!())
            }

            if stopmsg != 0 {
                last_shell_builtin = r_exit_builtin ;
                this_shell_builtin = last_shell_builtin ;
                return EXECUTION_FAILURE!();
            }
        }

        if (running_trap ==1) && (list == std::ptr::null_mut()) 
        {
            exit_value = trap_saved_exit_value;
        }else{
            exit_value = get_exitstat(list);
        }

        r_bash_logout();
        last_command_exit_value = exit_value; 

        jump_to_top_level(EXITPROG!());

       0
    }
}

#[no_mangle]
pub extern "C" fn r_bash_logout(){
    unsafe{    
        if login_shell != 0 && sourced_logout == 0 && subshell_environment == 0 {
            sourced_logout = sourced_logout + 1;
            maybe_execute_file(CString::new("~/.bash_logout").unwrap().as_ptr(),1);
            maybe_execute_file(SYS_BASH_LOGOOUT!(),1);
        }
    }

}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}