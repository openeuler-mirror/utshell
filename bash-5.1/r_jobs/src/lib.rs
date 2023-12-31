use libc::{c_int,c_char, c_void, fprintf,fileno,  size_t, c_long, getcwd, c_ulong, __errno_location, getpgrp, tcgetpgrp, isatty, open, dup, tcsetpgrp,};
use std::{ffi::{CString, CStr, }, fmt::Alignment};
use rcommon::{the_current_working_directory, r_savestring, r_get_working_directory};
// include!(concat!("jobs_h.rs"));
// include!(concat!("flags_h.rs"));
use r_bash::*;
use stdext::function_name;


extern "C" {
    static mut interactive:  c_int;
    static mut interactive_shell:  c_int;
    static mut subshell_environment:  c_int;
    static mut stdout: *mut libc::FILE;
    static mut stderr: *mut libc::FILE;
    static mut posixly_correct:c_int;
    static mut default_buffered_input: c_int;
    static mut shell_level: c_int;
    static mut wait_signal_received: c_int;
    static mut this_shell_builtin:sh_builtin_func_t;
    static mut last_command_exit_value: c_int;
    static mut last_command_exit_signal: c_int;
    static mut rl_readline_state: libc::c_ulong;
    static mut loop_level: c_int;
    static mut shell_compatibility_level: c_int;
    static mut executing_list: c_int;
    static mut sourcelevel: c_int;
    static mut this_command_name: *mut c_char;
    static mut trap_list: [*mut c_char; 0];
    static mut running_trap: c_int;
    static mut executing_builtin: c_int;
    static mut breaking: c_int;
    static mut subst_assign_varlist: *mut WORD_LIST;
    static mut temporary_env: *mut HASH_TABLE;
    static mut startup_state: c_int;
    static mut line_number: c_int;
    
    fn get_string_value(_:*const c_char) -> *mut c_char;
    fn sys_error(format:*const c_char, ...);
    fn list_reverse(list:*mut GENERIC_LIST) -> *mut GENERIC_LIST;
    fn internal_warning(_: *const  c_char, _: ...);
    fn coproc_reap();
    fn programming_error(_: *const  c_char, _: ...);
    fn signal_name(_: c_int) -> *mut  c_char;
    fn sync_buffered_stream(_: c_int) -> c_int;
    fn set_exit_status(_: c_int);
    fn getpid() -> __pid_t;
    fn unset_bash_input(_: c_int);
    fn setpgid(__pid: __pid_t, __pgid: __pid_t) -> c_int;
    fn signal_is_trapped(_: c_int) -> c_int;
    fn signal_is_hard_ignored(_: c_int) -> c_int;
    fn set_original_signal(_: c_int, _: Option::<SigHandler>);
    fn get_original_signal(_: c_int);
    fn get_new_window_size(_: c_int, _: *mut c_int, _: *mut c_int);
    fn internal_error(_: *const c_char, _: ...);
    fn wait_builtin(_: *mut WORD_LIST) -> c_int;
    fn siglongjmp(_: *mut __jmp_buf_tag, _: c_int) -> !;
    
    fn builtin_warning(_: *const c_char, _: ...);
    fn waitpid(__pid: __pid_t,__stat_loc: *mut c_int,__options: c_int) -> __pid_t;
    fn initialize_traps();
    fn dispose_command(_: *mut COMMAND);
    fn coproc_pidchk(_: pid_t, _: c_int);
    fn find_procsub_child(_: pid_t) -> c_int;
    fn set_procsub_status(_: c_int, _: pid_t, _: c_int);
    fn queue_sigchld_trap(_: c_int);
    fn signal_in_progress(_: c_int) -> c_int;
    fn maybe_call_trap_handler(_: c_int) -> c_int;
    fn set_pipestatus_array(_: *mut c_int, _: c_int);
    fn unwind_protect_mem(_: *mut c_char, _: c_int);
    fn set_impossible_sigchld_trap();
    fn parse_and_execute(
        _: *mut c_char,
        _: *const c_char,
        _: c_int,
    ) -> c_int;
    fn run_unwind_frame(_: *mut c_char);
    fn get_name_for_error() -> *mut c_char;
    fn begin_unwind_frame(_: *mut c_char);
    fn getmaxchild() -> libc::c_long;
    fn add_unwind_protect(cleanup:*mut Function, arg:*mut c_char);
    
    fn maybe_set_sigchld_trap(_: *mut c_char);

    fn sh_xrealloc(
        _: *mut libc::c_void,
        _: size_t,
        _: *const c_char,
        _: c_int,
    ) -> *mut libc::c_void;

    fn sh_xmalloc(
        _: size_t,
        _: *const c_char,
        _: c_int,
    ) -> *mut libc::c_void;


}

// pub  static mut SIG_IGN:__sighandler_t = 1;
#[macro_export]
macro_rules! SIG_IGN {
    () => {
        ::std::mem::transmute::<
            libc::intptr_t,
            *mut __sighandler_t,
        >(1 as c_int as libc::intptr_t)
    };
}

#[macro_export]
macro_rules! SIG_DFL {
    () => {
        ::std::mem::transmute::<
        libc::intptr_t,
        __sighandler_t,
    >(0 as c_int as libc::intptr_t)
    };
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct bucket_contents {
    pub next: *mut bucket_contents,
    pub key: *mut c_char,
    pub data: *mut libc::c_void,
    pub khash: libc::c_uint,
    pub times_found: c_int,
}
pub type BUCKET_CONTENTS = bucket_contents;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table {
    pub bucket_array: *mut *mut BUCKET_CONTENTS,
    pub nbuckets: c_int,
    pub nentries: c_int,
}
pub type HASH_TABLE = hash_table;

pub type sigjmp_buf = [__jmp_buf_tag; 1];
pub type __jmp_buf = [libc::c_long; 8];

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: c_int,
    pub __saved_mask: __sigset_t,
}


pub const DEFAULT_CHILD_MAX:u32 = 4096;
pub const MAX_CHILD_MAX:u32 = 32768;
pub const MAX_JOBS_IN_ARRAY:u32 = 4096;
pub const r_pidstat_table_SZ:i32 = 4096;
pub const BGPIDS_TABLE_SZ:u32 = 512;
pub const DEL_WARNSTOPPED:u32 = 1;
pub const DEL_NOBGPID:u32 = 2;
pub const JOB_SLOTS:u32 = 8;
pub const NO_PIDSTAT:i32 = -1;
pub const NO_PID:pid_t = -1;

pub const EAGAIN:i32 = 11;
pub const EX_NOEXEC:i32 = 126;

pub const ECHILD:i32 = 10;


#[macro_export]
macro_rules! PRECYCLED {
    ($p:expr) => {
        0
    };
}

#[macro_export]
macro_rules! __WIFSTOPPED {
    ($status:expr) => {
        (($status) & 0xff) == 0x7f
    };
}


#[macro_export]
macro_rules! WIFSTOPPED {
    ($status:expr) => {
        (($status) & 0xff) == 0x7f
    };
}

#[macro_export]
macro_rules! PSTOPPED {
    ($p:expr) => {
        WIFSTOPPED!((*$p).status)
    }
    
}

#[macro_export]
macro_rules! PRUNNING {
    ($p:expr) => {
        (*$p).running == PS_RUNNING as c_int
    }   
}

#[macro_export]
macro_rules! PALIVE {
    ($p:expr) => {
        PRUNNING!($p) || PSTOPPED!($p)
    }   
}

#[macro_export]
macro_rules! WAITPID {
    ($pid:expr, $statusp:expr, $options:expr) => {
        waitpid($pid as pid_t, $statusp, $options);
    };
}

#[macro_export]
macro_rules! getpgid {
    ($p:expr) => {
        getpgpr();
    };
}

#[macro_export]
macro_rules! REINSTALL_SIGCHLD_HANDLER {
    () => {
        
    };
}

pub type sh_job_map_func_t = unsafe extern "C" fn(*mut JOB, c_int, c_int, c_int) -> c_int;

static mut zerojs: jobstats = {
    let mut init = jobstats {
        c_childmax: -(1 as libc::c_long),
        c_living: 0 as  c_int,
        c_reaped: 0 as  c_int,
        c_injobs: 0 as  c_int,
        c_totforked: 0 as  c_int,
        c_totreaped: 0 as  c_int,
        j_jobslots: 0 as  c_int,
        j_lastj: 0 as  c_int,
        j_firstj: 0 as  c_int,
        j_njobs: 0 as  c_int,
        j_ndead: 0 as  c_int,
        j_current: -(1 as  c_int),
        j_previous: -(1 as  c_int),
        j_lastmade: 0 as *const JOB as *mut JOB,
        j_lastasync: 0 as *const JOB as *mut JOB,
    };
    init
};


#[no_mangle]
pub static mut js: jobstats = {
    let mut init = jobstats{
        c_childmax: -(1 as libc::c_long),
        c_living: 0 as  c_int,
        c_reaped: 0 as  c_int,
        c_injobs: 0 as  c_int,
        c_totforked: 0 as  c_int,
        c_totreaped: 0 as  c_int,
        j_jobslots: 0 as  c_int,
        j_lastj: 0 as  c_int,
        j_firstj: 0 as  c_int,
        j_njobs: 0 as  c_int,
        j_ndead: 0 as  c_int,
        j_current: -(1 as  c_int),
        j_previous: -(1 as  c_int),
        j_lastmade: 0 as *const JOB as *mut JOB,
        j_lastasync: 0 as *const JOB as *mut JOB,
    };
    init
};

#[no_mangle]
pub static mut r_pidstat_table:[ps_index_t; 4096] = [0; 4096];

pub static mut bgpids:bgpids = {
    let init = bgpids{
        storage: 0 as *const pidstat as *mut pidstat,
        head: 0 as ps_index_t,
        nalloc: 0 as ps_index_t,
        npid: 0 as c_int,
    };
    init
};

pub static mut procsubs:procchain = {
    let mut init = procchain{
        head: 0 as *mut PROCESS,
        end: 0 as *mut PROCESS,
        nproc: 0 as c_int,
    };
    init
};


#[no_mangle]
pub static mut wait_intr_buf: sigjmp_buf = [
    __jmp_buf_tag{
        __jmpbuf: [0;8],
        __mask_was_saved: 0 as c_int,
        __saved_mask: __sigset_t{__val:[0;16usize]},
}  ];

#[no_mangle]
pub static mut jobs: *mut *mut JOB = 0 as *const c_void as *mut c_void  as *mut *mut JOB;

#[no_mangle]
pub static mut shell_tty:c_int = -1;
#[no_mangle]
pub static mut shell_pgrp:pid_t = -1 as c_int; 
pub static mut terminal_pgrp:pid_t = -1;
#[no_mangle]
pub static mut original_pgrp:pid_t = -1;
#[no_mangle]
pub static mut pipeline_pgrp:pid_t = 0 as pid_t;

#[no_mangle]
pub static mut pgrp_pipe:[c_int; 2] = [-1 as c_int, -1 as c_int];
#[no_mangle]
pub static mut last_made_pid:pid_t = -1;
#[no_mangle]
pub static mut last_asynchronous_pid: pid_t = -1;
#[no_mangle]
pub static mut the_pipeline:*mut PROCESS = 0 as *const c_void as *mut c_void as *mut PROCESS;
#[no_mangle]
pub static mut job_control: c_int = 1;
#[no_mangle]
pub static mut running_in_background: c_int = 0;
#[no_mangle]
pub static mut already_making_children: c_int = 0;
#[no_mangle]
pub static mut check_window_size: c_int = CHECKWINSIZE_DEFAULT as i32;
#[no_mangle]
pub static mut last_procsub_child:*mut PROCESS = 0 as *const c_void as *mut c_void as *mut PROCESS;

pub static mut pstatuses:*mut c_int = 0 as *mut c_int;
pub static mut statsize:c_int = 0;

pub static mut sigchld:c_int = 0;
pub static mut queue_sigchld:c_int = 0;


#[macro_export]
macro_rules! QUEUE_SIGCHLD {
    ($os:expr) => {
        $os = sigchld;
        queue_sigchld += 1;
    };
}

#[no_mangle]
pub unsafe extern "C" fn UNQUEUE_SIGCHLD(os: c_int) {
    queue_sigchld -= 1;
    if queue_sigchld == 0 && os != sigchld {
        queue_sigchld = 1;
        waitchld(-1, 0);
        queue_sigchld = 0;
    }
}



#[no_mangle]
pub unsafe extern "C" fn PSTOPPED(p:*mut PROCESS) -> c_int
{
    if (*p).status & 0xff == 0x7f {
        return 1;
    } else {
        return 0;
    }
}

#[macro_export]
macro_rules! DEADJOB {
    ($j:expr) => {
        (**jobs.offset($j as isize)).state as  c_int == JDEAD 
    };
}

#[macro_export]
macro_rules! IS_NOTIFIED {
    ($j:expr) => {
         (**jobs.offset($j as isize)).flags & J_NOTIFIED as  c_int != 0 as  c_int 
    };
}


#[macro_export]
macro_rules! JOBSTATE {
    ($job:expr) => {
        (**jobs.offset($job as isize)).state
    };
}

#[macro_export]
macro_rules! STOPPED {
    ($job:expr) => {
        (**jobs.offset($job as isize)).state == JSTOPPED 
    };
}

pub const JMIXED: JOB_STATE = 8;
pub const JDEAD: JOB_STATE = 4;
pub const JSTOPPED: JOB_STATE = 2;
pub const JRUNNING: JOB_STATE = 1;
pub const JNONE: JOB_STATE = -1;

pub static mut old_tstp:*mut  SigHandler = 0 as *mut SigHandler;
pub static mut old_ttou:*mut  SigHandler = 0 as *mut SigHandler;
pub static mut old_ttin:*mut  SigHandler = 0 as *mut SigHandler;
pub static mut old_cont:*mut  SigHandler = 0 as *mut SigHandler; //SIG_DFL 
pub static mut saved_pipeline:*mut pipeline_saver = 0 as *mut pipeline_saver;
pub static mut saved_already_making_children:c_int = 0;

pub static mut jobs_list_frozen:c_int = 0;
pub static mut retcode_name_buffer:[c_char; 64] = [0; 64];

#[no_mangle]
pub unsafe extern "C" fn BLOCK_CHILD(nvar:*mut sigset_t,ovar:*mut sigset_t)
{
    sigemptyset (nvar); 
    sigaddset (nvar, SIGCHLD as c_int); 
    sigemptyset (ovar); 
    sigprocmask (SIG_BLOCK as i32, nvar, ovar); 
}

#[no_mangle]
pub unsafe extern "C" fn UNBLOCK_CHILD(over:*const sigset_t)
{
    sigprocmask(SIG_SETMASK as  c_int, over, 0 as *mut  c_void as *mut sigset_t);
}

/* 函数部分重构 */
#[no_mangle]
pub unsafe extern "C"  fn  init_job_stats() 
{
    js = zerojs;
}


unsafe extern "C"  fn  current_working_directory() -> *mut c_char
{
    let mut dir:*mut c_char;
    let mut d:[c_char; PATH_MAX as usize] = [0; PATH_MAX as usize]; 
    
    dir = get_string_value(b"PWD\0" as *const u8 as *const c_char);

    if dir.is_null() && !the_current_working_directory.is_null() && no_symbolic_links != 0 {
        dir = the_current_working_directory;
    }

    if dir.is_null() {
        dir = getcwd(d.as_mut_ptr(), (::std::mem::size_of::<[ c_char; 4096]>() as libc::c_ulong).try_into().unwrap());
        if !dir.is_null() {
            dir = d.as_mut_ptr();
        } 
    }

    if dir.is_null() {
        return b"<unknown>\0" as *const u8 as *mut c_char;
    } else {
        return dir;
    }
}

unsafe extern "C"  fn  job_working_directory() -> *mut c_char
{
    let mut dir:*mut c_char;

    dir = get_string_value(b"PWD\0" as *const u8 as *const c_char);
    dir = r_get_working_directory(b"job-working-directory\0" as * const u8 as *mut c_char);
    if !dir.is_null() {
        return r_savestring(dir) ;
    }

    return r_savestring(b"<unknown>\0" as *const u8 as *const c_char);
}

#[no_mangle]
pub unsafe extern "C"  fn  making_children()
{
    if already_making_children != 0 {
        return;
    }

    already_making_children = 1;
    start_pipeline();
}

#[no_mangle]
pub unsafe extern "C"  fn  stop_making_children()
{
    already_making_children = 0;
}

#[no_mangle]
pub unsafe extern "C"  fn  cleanup_the_pipeline()
{
    let mut disposer:*mut PROCESS;
    let mut set:sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset:sigset_t = __sigset_t { __val: [0; 16] };

    BLOCK_CHILD(&mut set, &mut oset);
    disposer = the_pipeline;
    the_pipeline = 0 as *mut PROCESS;
    UNBLOCK_CHILD(&mut oset);

    if !disposer.is_null() {
        discard_pipeline(disposer);
    }
}

unsafe extern "C"  fn  alloc_pipeline_saver() -> *mut pipeline_saver
{
    let mut ret:*mut pipeline_saver;

    ret = xmalloc(::std::mem::size_of::<pipeline_saver>() as usize) as *mut pipeline_saver; 

    (*ret).pipeline = 0 as *mut process;
    (*ret).next = 0 as *mut pipeline_saver;
    
    return ret;
}

#[no_mangle]
pub unsafe extern "C"  fn  save_pipeline(clear:c_int) 
{
    let mut set:sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset:sigset_t = __sigset_t { __val: [0; 16] };
    let mut saver:*mut pipeline_saver;

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

#[no_mangle]
pub unsafe extern "C"  fn  restore_pipeline(discard:c_int) -> *mut PROCESS
{
    let mut old_pipeline:*mut PROCESS;
    let mut set:sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset:sigset_t = __sigset_t { __val: [0; 16] };
    let mut saver:*mut pipeline_saver; 

    BLOCK_CHILD (&mut set, &mut oset);
    old_pipeline = the_pipeline;
    the_pipeline = (*saved_pipeline).pipeline;
    saver = saved_pipeline;
    saved_pipeline = (*saved_pipeline).next;
    free (saver as *mut c_void);
    already_making_children = saved_already_making_children;
    UNBLOCK_CHILD (&mut oset);

    if discard!= 0 && !old_pipeline.is_null() {
        discard_pipeline (old_pipeline);
        return 0 as *mut PROCESS ;
    }
    return old_pipeline;   
}

#[no_mangle]
pub unsafe extern "C"  fn  start_pipeline ()
{
    if !the_pipeline.is_null() {
        cleanup_the_pipeline ();
        if pipeline_pgrp != shell_pgrp {
            pipeline_pgrp = 0 as pid_t;
        }
        sh_closepipe (pgrp_pipe.as_mut_ptr());
    }

    if job_control != 0
    {
        if libc::pipe(pgrp_pipe.as_mut_ptr()) == -1 {
            sys_error (b"start_pipeline: pgrp pipe\0" as *const u8 as *const i8);
        }
    }
}


#[no_mangle]
pub unsafe extern "C"  fn  stop_pipeline(mut async_0:c_int, mut deferred:*mut COMMAND) -> c_int
{ 
    // println!("stop_pipeline");
    let mut i:c_int;
    let mut j:c_int;
    let mut newjob:*mut JOB;
    let mut set:sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset:sigset_t = __sigset_t { __val: [0; 16] };

    BLOCK_CHILD (&mut set, &mut oset);
    sh_closepipe (pgrp_pipe.as_mut_ptr());
    cleanup_dead_jobs ();

    if js.j_jobslots == 0 {
        js.j_jobslots = JOB_SLOTS as c_int;
        jobs = xmalloc ((js.j_jobslots * (std::mem::size_of::<*mut JOB>() as c_int)) as usize) as *mut *mut JOB;

        i = 0 as  c_int;
        while i < js.j_jobslots {
        //    (*jobs.offset(i as isize)) = 0 as *mut JOB;
           (*jobs.offset(i as isize)) = std::ptr::null_mut();
            
            i += 1;
        }
        js.j_njobs = 0 as  c_int;
        js.j_lastj = js.j_njobs;
        js.j_firstj = js.j_lastj;
    }

    if interactive != 0
    {
        // println!("258963147");
        i = js.j_jobslots;
        while i != 0 {
            let temp = i -1 ;
            if !(*jobs.offset(temp as isize)).is_null() {
                // println!("258963147 i={}",i);
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

    if (interactive_shell == 0 || subshell_environment != 0) && 
        i == js.j_jobslots && js.j_jobslots >= MAX_JOBS_IN_ARRAY as i32{
            i = compact_jobs_list (0 as c_int);
        }

    if i == js.j_jobslots {
        js.j_jobslots += JOB_SLOTS as i32;
        jobs = xrealloc (jobs as *mut c_void, (js.j_jobslots * std::mem::size_of::<*mut JOB>() as c_int) as usize) as *mut *mut JOB;

        j = i;
        while j < js.j_jobslots {
            (*jobs.offset(j as isize)) = 0 as *mut JOB;
            j += 1;
        }
    }

    if !the_pipeline.is_null() {
        let mut p: *mut PROCESS = 0 as *mut PROCESS;
        let mut any_running:  c_int = 0;
        let mut any_stopped:  c_int = 0;
        let mut n:  c_int = 0;
        newjob = xmalloc (std::mem::size_of::<JOB>() as c_int as usize) as *mut JOB;
   
        n = 1 as  c_int;
        p = the_pipeline;
        while (*p).next != the_pipeline {
            n += 1;
            p = (*p).next;
        }

        (*p).next = 0 as *mut PROCESS;
        if !the_pipeline.is_null() && !((*the_pipeline).next).is_null() {
            (*newjob).pipe =  list_reverse(the_pipeline as *mut GENERIC_LIST) as *mut PROCESS
        } else {
            (*newjob).pipe = the_pipeline;
        };

        p = (*newjob).pipe;
        while !((*p).next).is_null() {
            p = (*p).next;
        }

        (*p).next = (*newjob).pipe;

        the_pipeline = 0 as *mut PROCESS ;
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
        
        loop{
            any_running |= PRUNNING! (p) as c_int;
            any_stopped |= PSTOPPED (p);
            p = (*p).next;
            if !(p != (*newjob).pipe) {
                break;
            }
        }
        
        (*newjob).state = (if any_running != 0 {
            JRUNNING as  c_int
        } else if any_stopped != 0 {
            JSTOPPED as  c_int
        } else {
            JDEAD as  c_int
        }) as JOB_STATE;

        (*newjob).wd = job_working_directory();
        (*newjob).deferred = deferred;

        (*newjob).j_cleanup = ::std::mem::transmute::<*mut libc::c_void, sh_vptrfunc_t>(0 as *mut libc::c_void);
        (*newjob).cleanarg = 0 as *mut c_void;

        *jobs.offset(i as isize) = newjob;

        if (*newjob).state == JDEAD as  c_int
        && (*newjob).flags & 0x1 as  c_int != 0
        {
            setjstatus(i);
        }
        if (*newjob).state == JDEAD as  c_int {
            js.c_reaped += n;
            js.j_ndead += 1;
        }
        js.c_injobs += n;
        js.j_lastj = i;
        js.j_njobs += 1;

    } else {
        newjob = 0 as *mut JOB ;
    }

    if !newjob.is_null() {
        js.j_lastmade = newjob;
    } 

    if async_0 != 0
    {
        if !newjob.is_null()
        {
            (*newjob).flags &= !J_FOREGROUND as c_int;
            (*newjob).flags |= J_ASYNC as c_int;
            js.j_lastasync = newjob;
        }
        reset_current ();
    } else {
        if !newjob.is_null() {
            (*newjob).flags |= J_FOREGROUND as c_int;

            if job_control != 0 && (*newjob).pgrp != 0 && 
                (subshell_environment&SUBSHELL_ASYNC as c_int) == 0 && 
                running_in_background == 0 {
                    maybe_give_terminal_to (shell_pgrp, (*newjob).pgrp, 0);
                }   
        }
    }
    stop_making_children ();
    UNBLOCK_CHILD (&mut oset);
    return if !newjob.is_null() { i } else { js.j_current };
}



macro_rules! TYPE_WIDTH {
    ($t:ty) => {
        std::mem::size_of::<$t>() * 8
    };
}

macro_rules! TYPE_SIGNED {
    ($t:ty) => {
        !(0 as $t < -1 as $t)
    };
}

macro_rules! TYPE_MAXIMUM {
    ($t:ty) => {
        if !TYPE_SIGNED!($t){
            -1 as $t
        } else {
            (1 << ((TYPE_WIDTH!($t) -2) -1) * 2 + 1) as $t
        }
         
    }
}

#[no_mangle]
unsafe extern "C"  fn  bgp_resize ()
{
    let mut nsize: ps_index_t = 0;
    let mut nsize_cur: ps_index_t = 0;
    let mut nsize_max: ps_index_t = 0;
    let mut psi: ps_index_t = 0;

    if bgpids.nalloc == 0 {
        psi = 0 as  c_int;
        while psi < r_pidstat_table_SZ as c_int {
            r_pidstat_table[psi as usize] = NO_PIDSTAT ;
            psi += 1;
        }
        nsize = BGPIDS_TABLE_SZ as ps_index_t;
        bgpids.head = 0 ;
    } else {
        nsize = bgpids.nalloc;
    }

    nsize_max = TYPE_MAXIMUM!(ps_index_t);
    nsize_cur = js.c_childmax as ps_index_t;

    if nsize_cur < 0 {
        nsize_cur = MAX_CHILD_MAX as c_int;
    }				
    
    while nsize > 0  && nsize < nsize_cur {
        nsize <<= 1 ;
    }

    if nsize > nsize_max || nsize <= 0 {
        nsize = nsize_max;
    }

    if nsize > MAX_CHILD_MAX as c_int {
        nsize_max = MAX_CHILD_MAX as c_int;
        nsize = nsize_max;
    }

    if bgpids.nalloc < nsize_cur && bgpids.nalloc < nsize_max {
        bgpids.storage = xrealloc(bgpids.storage as *mut c_void, (nsize * std::mem::size_of::<pidstat>()as c_int) as usize) as *mut pidstat;
        psi = bgpids.nalloc;
        while psi < nsize {
            (*(bgpids.storage).offset(psi as isize)).pid = -(1 as  c_int);
            psi += 1;
        }
        bgpids.nalloc = nsize;
    } else if bgpids.head >= bgpids.nalloc {
        bgpids.head = 0 as  c_int;
    }
}




#[no_mangle]
unsafe extern "C"  fn  bgp_getindex()-> ps_index_t
{ 
    if bgpids.nalloc < js.c_childmax as ps_index_t || bgpids.head >= bgpids.nalloc {
        bgp_resize();
    }
    pshash_delindex(bgpids.head);
    bgpids.head += 1;
    return bgpids.head;
}


#[no_mangle]
unsafe extern "C"  fn  pshash_getbucket (pid:pid_t) -> *mut ps_index_t
{
    let mut hash: c_ulong = 0;
    hash = (pid as libc::c_ulong).wrapping_mul(0x9e370001 as libc::c_ulong);


    return &mut *r_pidstat_table.as_mut_ptr().offset((hash % r_pidstat_table_SZ as u64) as isize)as *mut ps_index_t;
}


#[no_mangle]
unsafe extern "C"  fn  bgp_add(mut pid: pid_t, mut status: c_int) -> *mut pidstat {
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


#[no_mangle]
unsafe extern "C"  fn  pshash_delindex(mut psi: ps_index_t) {
    let mut ps: *mut pidstat = 0 as *mut pidstat;
    let mut bucket: *mut ps_index_t = 0 as *mut ps_index_t;

    ps = &mut *(bgpids.storage).offset(psi as isize) as *mut pidstat;
    if (*ps).pid == NO_PID {
        return;
    }
    if (*ps).bucket_next != NO_PIDSTAT {
        (*(bgpids.storage).offset((*ps).bucket_next as isize))
            .bucket_prev = (*ps).bucket_prev;
    }

    if (*ps).bucket_prev != NO_PIDSTAT {
        (*(bgpids.storage).offset((*ps).bucket_prev as isize))
            .bucket_next = (*ps).bucket_next;
    } else {
        bucket = pshash_getbucket((*ps).pid);
        *bucket = (*ps).bucket_next;
    }
    (*ps).pid = NO_PID;
    (*ps).bucket_next = NO_PIDSTAT;
    (*ps).bucket_prev = NO_PIDSTAT;
}


unsafe extern "C" fn bgp_delete(mut pid: pid_t) ->  c_int {
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
            internal_warning( b"bgp_delete: LOOP: psi (%d) == storage[psi].bucket_next\0" as *const u8 as *const c_char,psi);
            return 0 as  c_int;
        }
        psi = (*(bgpids.storage).offset(psi as isize)).bucket_next;
    }

    if psi == NO_PIDSTAT {
        return 0;
    }
    pshash_delindex(psi);
    bgpids.npid -= 1;
    return 1 ;
}

unsafe extern "C" fn bgp_clear() {
    if (bgpids.storage).is_null() || bgpids.nalloc == 0 {
        return;
    }
    libc::free(bgpids.storage as *mut  c_void);

    bgpids.storage = 0 as *mut pidstat;
    bgpids.nalloc = 0 ;
    bgpids.head = 0 ;
    bgpids.npid = 0 ;
}

unsafe extern "C" fn bgp_search(mut pid: pid_t) ->  c_int {
    let mut psi: ps_index_t = 0;
    let mut orig_psi: ps_index_t = 0;
    psi = *pshash_getbucket(pid);
    orig_psi = psi;

    if (bgpids.storage).is_null() || bgpids.nalloc == 0 || bgpids.npid == 0 
    {
        return -1;
    }

    psi = *pshash_getbucket(pid);
    orig_psi = psi;
    while psi != NO_PIDSTAT {
        if (*(bgpids.storage).offset(psi as isize)).pid == pid {
            return (*(bgpids.storage).offset(psi as isize)).status as c_int;
        }
        if orig_psi == (*(bgpids.storage).offset(psi as isize)).bucket_next {
            internal_warning(b"bgp_search: LOOP: psi (%d) == storage[psi].bucket_next\0" as *const u8 as *const c_char,psi);
            return -1;
        }
        psi = (*(bgpids.storage).offset(psi as isize)).bucket_next;
    }

    return -1;
}


#[no_mangle]
pub unsafe extern "C"  fn  save_proc_status(mut pid: pid_t, mut status:  c_int) {
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };
    
    BLOCK_CHILD(&mut set, &mut oset);
    bgp_add(pid, status);
    UNBLOCK_CHILD(&mut oset);
}

unsafe extern "C" fn procsub_free(mut p: *mut PROCESS) 
{
    if !((*p).command).is_null() {
        free(((*p).command) as *mut c_void);
    }  
    free (p as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C"  fn  procsub_add(mut p: *mut PROCESS) -> *mut PROCESS {
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

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

#[no_mangle]
pub unsafe extern "C"  fn  procsub_search(mut pid: pid_t) -> *mut PROCESS {
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

#[no_mangle]
pub unsafe extern "C"  fn  procsub_delete(mut pid: pid_t) -> *mut PROCESS {
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

#[no_mangle]
pub unsafe extern "C"  fn  procsub_waitpid(mut pid: pid_t) -> c_int {
    let mut p: *mut PROCESS = 0 as *mut PROCESS;
    let mut r:  c_int = 0;

    p = procsub_search(pid);
    if p.is_null() {
        return -1;
    }
    if (*p).running == PS_DONE as i32 {
        return (*p).status;
    }
    r = wait_for((*p).pid, 0 );

    return r;
}

#[no_mangle]
pub unsafe extern "C"  fn  procsub_waitall() {
    let mut p: *mut PROCESS = 0 as *mut PROCESS;
    let mut r:  c_int = 0;

    p = procsubs.head;
    while !p.is_null() {
        if !((*p).running == PS_DONE as i32) {
            r = wait_for((*p).pid, 0 );
        }
        p = (*p).next;
    }
}


#[no_mangle]
pub unsafe extern "C"  fn  procsub_clear() {
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
    procsubs.nproc = 0 as  c_int;
    UNBLOCK_CHILD(&mut oset);

}

#[no_mangle]
pub unsafe extern "C"  fn  procsub_prune() {
    let mut ohead: *mut PROCESS = 0 as *mut PROCESS;
    let mut oend: *mut PROCESS = 0 as *mut PROCESS;
    let mut ps: *mut PROCESS = 0 as *mut PROCESS;
    let mut p: *mut PROCESS = 0 as *mut PROCESS;
    let mut onproc:  c_int = 0;

    if procsubs.nproc == 0  {
        return;
    }
    ohead = procsubs.head;
    oend = procsubs.end;
    onproc = procsubs.nproc;

    procsubs.end = 0 as *mut PROCESS;
    procsubs.head = procsubs.end;
    procsubs.nproc = 0 as c_int;

    p = ohead;
    while !p.is_null() {
        ps = (*p).next;
        (*p).next = 0 as *mut process;
        if (*p).running == 0 as  c_int {
            bgp_add((*p).pid, process_exit_status((*p).status));
            procsub_free(p);
        } else {
            procsub_add(p);
        }
        p = ps;
    }
}


unsafe extern "C" fn reset_job_indices() {
    let mut old:  c_int = 0;

    if (*jobs.offset(js.j_firstj as isize)).is_null() {
        js.j_firstj = js.j_firstj + 1;
        old = js.j_firstj;
        
        if old >= js.j_jobslots {
            old = js.j_jobslots - 1;
        }
        while js.j_firstj != old {
            if js.j_firstj >= js.j_jobslots {
                js.j_firstj = 0 ;
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
            old = 0 ;
        }
        while js.j_lastj != old {
            if js.j_lastj < 0 {
                js.j_lastj = js.j_jobslots - 1 ;
            }
            if !(*jobs.offset(js.j_lastj as isize)).is_null() || js.j_lastj == old {
                break;
            }
            js.j_lastj -= 1;
        } 

       if js.j_lastj == old {
            js.j_njobs = 0 as  c_int;
            js.j_lastj = js.j_njobs;
            js.j_firstj = js.j_lastj;
        }
    }
}


unsafe extern "C" fn cleanup_dead_jobs() {
    let mut i:  c_int = 0;
    let mut os:  c_int = 0;
    let mut discard: *mut PROCESS = 0 as *mut PROCESS;

    if js.j_jobslots == 0 || jobs_list_frozen != 0 {
        return;
    }
    QUEUE_SIGCHLD!(os);

    i = 0 ;
    while i < js.j_jobslots {
        if !(*jobs.offset(i as isize)).is_null()
            && DEADJOB!(i)
            && IS_NOTIFIED!(i) 
        {
            delete_job(i, 0 as  c_int);
        }
        i += 1;
    }
    procsub_prune();
    last_procsub_child = 0 as *mut  c_void as *mut PROCESS;
    coproc_reap();
    UNQUEUE_SIGCHLD(os)

}

unsafe extern "C" fn processes_in_job(mut job:  c_int) ->  c_int {
    let mut nproc:  c_int = 0;
    let mut p: *mut PROCESS = 0 as *mut PROCESS;
    nproc = 0 as  c_int;
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

unsafe extern "C" fn delete_old_job(mut pid: pid_t) {
    let mut p: *mut PROCESS = 0 as *mut PROCESS;
    let mut job:  c_int = 0;

    job = find_job(pid, 0 , &mut p);
    if job != NO_JOB {
        if JOBSTATE!(job) == JDEAD {
            delete_job(job, 2 as  c_int);
        } else if !p.is_null() {
            (*p).pid = 0 ;
        }
    }
}


unsafe extern "C" fn realloc_jobs_list() {
    // println!("realloc_jobs_list");
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };
    let mut nsize:  c_int = 0;
    let mut i:  c_int = 0;
    let mut j:  c_int = 0;
    let mut ncur:  c_int = 0;
    let mut nprev:  c_int = 0;
    let mut nlist: *mut *mut JOB = 0 as *mut *mut JOB;

    nprev = NO_JOB;
    ncur = nprev;
    nsize = (js.j_njobs + JOB_SLOTS as c_int - 1 ) / JOB_SLOTS as c_int ;
    nsize *= JOB_SLOTS as c_int;
    i = js.j_njobs % JOB_SLOTS as c_int;
    if i == 0  || i > (JOB_SLOTS as c_int >> 1) {
        nsize += JOB_SLOTS as c_int;
    }

    BLOCK_CHILD(&mut set, &mut oset);
    nlist = if js.j_jobslots == nsize {
        jobs
    } else {
        xmalloc((nsize * std::mem::size_of::<JOB>() as c_int) as usize) as *mut *mut JOB   
    };

    js.j_ndead = 0 ;
    js.c_reaped = js.j_ndead;
    j = 0 ;
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
            j = j + 1;  //
            if (**jobs.offset(i as isize)).state as c_int == JDEAD as c_int {
                js.j_ndead += 1;
                js.c_reaped += processes_in_job(i);
            }  
        }
        i += 1;
    }
    
    js.j_firstj = 0 ;
    js.j_lastj = if j > 0  {
        j - 1 
    } else {
        0 
    };
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
    if js.j_current == NO_JOB || js.j_previous == NO_JOB
        || js.j_current > js.j_lastj || js.j_previous > js.j_lastj
    {
        reset_current();
    }

    UNBLOCK_CHILD (&mut oset);
}

unsafe extern "C" fn compact_jobs_list(mut flags: c_int) -> c_int {
    if js.j_jobslots == 0  || jobs_list_frozen != 0 {
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

#[no_mangle]
pub unsafe extern "C"  fn  delete_job(
    mut job_index: c_int,
    mut dflags: c_int,
) {
    let mut temp: *mut JOB = 0 as *mut JOB;
    let mut proc_0: *mut PROCESS = 0 as *mut PROCESS;
    let mut ndel:  c_int = 0;

    if js.j_jobslots == 0 || jobs_list_frozen != 0 {
        return;
    }
    if dflags & DEL_WARNSTOPPED as c_int != 0 && subshell_environment == 0 
        && STOPPED!(job_index) 
    {
        internal_warning(b"deleting stopped job %d with process group %ld\0" as *const u8 as *const c_char,
            job_index + 1 ,
            (**jobs.offset(job_index as isize)).pgrp as libc::c_long,
        );
    }

    temp = *jobs.offset(job_index as isize);
    if temp.is_null() {
        return;
    }

    if dflags & DEL_NOBGPID as c_int == 0 as  c_int
        && (*temp).flags & (J_ASYNC as c_int|J_FOREGROUND as c_int) == J_ASYNC as c_int
    {
        proc_0 = find_last_proc(job_index, 0 );
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
    if (*temp).state  == JDEAD as c_int {
        js.c_reaped -= ndel;
        js.j_ndead -= 1;
        if js.c_reaped < 0  {
            js.c_reaped = 0 ;
        }
    }
    if !((*temp).deferred).is_null() {
        dispose_command((*temp).deferred);
    }

    libc::free(temp as *mut c_void);

    js.j_njobs -= 1;
    if js.j_njobs == 0  {
        js.j_lastj = 0 ;
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

#[no_mangle]
pub unsafe extern "C"  fn  nohup_job(mut job_index: c_int) {
    let mut temp: *mut JOB = 0 as *mut JOB;

    if js.j_jobslots == 0 {
        return;
    }
    temp = *jobs.offset(job_index as isize);
    if !temp.is_null() {
        (*temp).flags |= J_NOHUP as c_int;
    }
}

#[no_mangle]
pub unsafe extern "C"  fn  discard_pipeline(mut chain: *mut PROCESS) -> c_int {
    let mut this: *mut PROCESS = 0 as *mut PROCESS;
    let mut next: *mut PROCESS = 0 as *mut PROCESS;
    let mut n: c_int = 0;

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

unsafe extern "C" fn add_process(mut name: *mut  c_char, mut pid: pid_t) {
    let mut t: *mut PROCESS = 0 as *mut PROCESS;
    let mut p: *mut PROCESS = 0 as *mut PROCESS;

    t = xmalloc(::std::mem::size_of::<PROCESS>() as usize) as *mut PROCESS;
    (*t).next = the_pipeline;
    (*t).pid = pid;
    (*t).status = 0 ;
    (*t).running = PS_RUNNING as c_int;
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

#[no_mangle]
pub unsafe extern "C"  fn  append_process(
    mut name: *mut  c_char,
    mut pid: pid_t,
    mut status:  c_int,
    mut jid:  c_int,
) {
    // println!("append_process");
    let mut t: *mut PROCESS = 0 as *mut PROCESS;
    let mut p: *mut PROCESS = 0 as *mut PROCESS;

    t = xmalloc(::std::mem::size_of::<PROCESS>() as usize) as *mut PROCESS;
    (*t).next = 0 as *mut  c_void as *mut PROCESS;
    (*t).pid = pid;
    (*t).status = (status & 0xff as  c_int) << WEXITSTATUS_OFFSET as c_int;
    (*t).running = PS_DONE as c_int;
    (*t).command = name;

    js.c_reaped += 1;
    p = (**jobs.offset(jid as isize)).pipe;
    while (*p).next != (**jobs.offset(jid as isize)).pipe {
        p = (*p).next;
    }

    (*p).next = t;
    (*t).next = (**jobs.offset(jid as isize)).pipe;
}

unsafe extern "C" fn map_over_jobs(
    mut func: Option::<sh_job_map_func_t>,
    mut arg1: c_int,
    mut arg2: c_int,
) -> c_int {
    let mut i: c_int = 0;
    let mut result: c_int = 0;
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };
    if js.j_jobslots == 0  {
        return 0 ;
    }
    BLOCK_CHILD (&mut set, &mut oset);
    result = 0 ;
    i = result;
    while i < js.j_jobslots {
        if !(*jobs.offset(i as isize)).is_null() {
            result = (Some(func.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(*jobs.offset(i as isize), arg1, arg2, i);
            if result != 0 {
                break;
            }
        }
        i += 1;
    }
    UNBLOCK_CHILD (&mut oset);
    return result;
}

#[no_mangle]
pub unsafe extern "C"  fn  terminate_current_pipeline() {
    if pipeline_pgrp != 0 && pipeline_pgrp != shell_pgrp {
        killpg(pipeline_pgrp, SIGTERM as c_int);
        killpg(pipeline_pgrp, SIGCONT as c_int);
    }
}

#[no_mangle]
pub unsafe extern "C"  fn  terminate_stopped_jobs() {
    let mut i:c_int = 0;
    i = 0 as c_int;
    while i < js.j_jobslots {
        if !(*jobs.offset(i as isize)).is_null()
            && STOPPED!(i) 
        {
            killpg((**jobs.offset(i as isize)).pgrp, SIGTERM as c_int);
            killpg((**jobs.offset(i as isize)).pgrp, SIGCONT as c_int);
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C"  fn  hangup_all_jobs() {
    let mut i: c_int = 0;
    i = 0 ;
    while i < js.j_jobslots {
        if !(*jobs.offset(i as isize)).is_null() {
            if !((**jobs.offset(i as isize)).flags & J_NOHUP as c_int != 0) {
                continue;
            }
                killpg((**jobs.offset(i as isize)).pgrp, SIGHUP  as c_int);
                if STOPPED!(i)
                {
                    killpg((**jobs.offset(i as isize)).pgrp, SIGCONT as c_int);
                } 
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C"  fn  kill_current_pipeline() {
    stop_making_children();
    start_pipeline();
}

unsafe extern "C" fn find_pid_in_pipeline(
    mut pid: pid_t,
    mut pipeline: *mut PROCESS,
    mut alive_only: c_int,
) -> *mut PROCESS {
    let mut p: *mut PROCESS = 0 as *mut PROCESS;
    p = pipeline;
    loop {
        if (*p).pid == pid
            && (alive_only == 0 && PRECYCLED!(p) == 0|| PALIVE!(p))
        {
            return p;
        }
        p = (*p).next;
        if !(p != pipeline) {
            break;
        }
    }
    return 0 as *mut PROCESS;
}


unsafe extern "C" fn find_pipeline(
    mut pid: pid_t,
    mut alive_only: c_int,
    mut jobp: *mut c_int,
) -> *mut PROCESS {
    // println!("find_pipeline");
    let mut job: c_int = 0;
    let mut p: *mut PROCESS = 0 as *mut PROCESS;
    let mut save: *mut pipeline_saver = 0 as *mut pipeline_saver;
    p = 0 as *mut libc::c_void as *mut PROCESS;
    if !jobp.is_null() {
        *jobp = -(1 as c_int);
    }
    if !the_pipeline.is_null()
        && {
            p = find_pid_in_pipeline(pid, the_pipeline, alive_only);
            !p.is_null()
        }
    {
        return p;
    }
    save = saved_pipeline;
    while !save.is_null() {
        if !((*save).pipeline).is_null()
            && {
                p = find_pid_in_pipeline(pid, (*save).pipeline, alive_only);
                !p.is_null()
            }
        {
            return p;
        }
        save = (*save).next;
    }
    if procsubs.nproc > 0 as c_int
        && {
            p = procsub_search(pid);
            !p.is_null()
        }
        && (alive_only == 0 as c_int && 0 as c_int == 0 as c_int
            || ((*p).running == 1 as c_int
                || (*p).status & 0xff as c_int == 0x7f as c_int))
    {
        return p;
    }
    job = find_job(pid, alive_only, &mut p);
    if !jobp.is_null() {
        *jobp = job;
    }
    return if job == -(1 as c_int) {
        0 as *mut libc::c_void as *mut PROCESS
    } else {
        (**jobs.offset(job as isize)).pipe
    };
}


unsafe extern "C" fn find_process(
    mut pid: pid_t,
    mut alive_only:  c_int,
    mut jobp: *mut  c_int,
) -> *mut PROCESS {
    let mut p: *mut PROCESS = 0 as *mut PROCESS;
    p = find_pipeline(pid, alive_only, jobp);
    while !p.is_null() && (*p).pid != pid {
        p = (*p).next;
    }
    return p;
}

unsafe extern "C" fn find_job(
    mut pid: pid_t,
    mut alive_only: c_int,
    mut procp: *mut *mut PROCESS,
) -> c_int {
    let mut i: c_int = 0;
    let mut p: *mut PROCESS = 0 as *mut PROCESS;
    i = 0 as c_int;
    // println!("j_jobslots {}",js.j_jobslots);
    while i < js.j_jobslots {
        if !(*jobs.offset(i as isize)).is_null() {
            p = (**jobs.offset(i as isize)).pipe;
            loop {
                if (*p).pid == pid
                    && (alive_only == 0 as c_int
                        && 0 as c_int == 0 as c_int
                        || ((*p).running == 1 as c_int
                            || (*p).status & 0xff as c_int == 0x7f as c_int))
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
    return -(1 as c_int);
}

#[no_mangle]
pub unsafe extern "C"  fn  get_job_by_pid( mut pid: pid_t, mut block: c_int, mut procp: *mut *mut PROCESS,) ->  c_int {
    // println!("get_job_by_pid");
    let mut job:  c_int = 0;
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

    if block != 0 {
        BLOCK_CHILD (&mut set, &mut oset);
    }
    job = find_job(pid, 0 as  c_int, procp);
    if block != 0 {
        UNBLOCK_CHILD (&mut oset);
    }
    return job;
}

#[no_mangle]
pub unsafe extern "C"  fn  describe_pid(mut pid: pid_t) {
    let mut job: c_int = 0;
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

    BLOCK_CHILD (&mut set, &mut oset);
    job = find_job(pid, 0, 0 as *mut *mut PROCESS);

    if job != NO_JOB {
            libc::fprintf(stderr,b"[%d] %ld\n\0" as *const u8 as *const  c_char,job + 1 as  c_int,pid as libc::c_long,
        );
    } else {
        programming_error(b"describe_pid: %ld: no such pid\0" as *const u8 as *const  c_char, pid as  c_long,
        );
    }
    UNBLOCK_CHILD (&mut oset);
}


unsafe extern "C" fn j_strsignal(mut s: c_int) -> *mut c_char 
{
    let mut x: *mut c_char = 0 as *mut c_char;
    x = strsignal(s);
    if x.is_null() {
        x = retcode_name_buffer.as_mut_ptr();
        libc::snprintf(x,::std::mem::size_of::<[ c_char; 64]>() as usize,
            b"Signal %d\0" as *const u8 as *const  c_char,
            s,
        );
    }
    return x;
}

#[macro_export]
macro_rules! WSTOPSIG {
    ($status:expr) => {
        ($status & 0xff00) >> 8
    };
}

#[macro_export]
macro_rules! RUNNING {
    ($j:expr) => {
        (**jobs.offset($j as isize)).state as c_int == JRUNNING
    };
}

#[macro_export]
macro_rules! WIFSIGNALED {
    ($status:expr) => {
        ((($status) & 0x7f) + 1) >> 1 > 0
    };
}

#[macro_export]
macro_rules! WTERMSIG {
    ($status:expr) => {
        ($status) & 0x7f
    };
}

#[macro_export]
macro_rules! WIFEXITED {
    ($status:expr) => {
        ($status) & 0x7f == 0
    };
}

#[macro_export]
macro_rules! WEXITSTATUS {
    ($status:expr) => {
        (($status) & 0xff00) >> 8
    };
}

#[macro_export]
macro_rules!  WSTATUS{
    ($t:expr) => {
        $t
    };
}

unsafe extern "C" fn printable_job_status(mut j: c_int, mut p: *mut PROCESS, mut format: c_int,) -> *mut c_char 
{
    static mut temp: *mut c_char = 0 as *const c_char as *mut c_char;
    let mut es: c_int = 0;
    temp = b"Done\0" as *const u8 as *const c_char as *mut c_char;
    
    if STOPPED!(j) && format == 0
    {
        if posixly_correct == 0 as  c_int || p.is_null() || (WIFSTOPPED!((*p).status)) {
            temp = b"Stopped\0" as *const u8 as *mut c_char;
        } else {
            temp = retcode_name_buffer.as_mut_ptr();
            libc::snprintf(temp,::std::mem::size_of::<[ c_char; 64]>() as usize,
                    b"Stopped(%s)\0" as *const u8 as *const  c_char,
                signal_name(WSTOPSIG!((*p).status)),
            );
        }
    } else if RUNNING!(j)
        {
        temp = b"Running\0" as *const u8 as *mut c_char;
    } else {
        if WIFSTOPPED!((*p).status) {
            temp = j_strsignal(WSTOPSIG!((*p).status));
        } else if WIFSIGNALED!((*p).status) {
            temp = j_strsignal(WTERMSIG!((*p).status));
        } else if WIFEXITED!((*p).status){
            temp = retcode_name_buffer.as_mut_ptr();
	        es = WEXITSTATUS!((*p).status);
            if es == 0 {
                libc::strncpy (temp, b"Done\0" as *const u8 as *mut c_char, std::mem::size_of::<[ c_char; 64]>() - 1);
	            *temp.offset(
                    (::std::mem::size_of::<[ c_char; 64]>() as  c_ulong)
                        .wrapping_sub(1 as  c_int as  c_ulong) as isize,
                ) = '\u{0}' as i32 as  c_char;
            } else if posixly_correct != 0 {
                libc::snprintf(temp,::std::mem::size_of::<[ c_char; 64]>() as usize,
                    b"Done(%d)\0" as *const u8 as *const  c_char,
                    es
                );
            } else {
                libc::snprintf(temp,::std::mem::size_of::<[ c_char; 64]>() as usize,
                    b"Exit(%d)\0" as *const u8 as *const  c_char,
                    es
                );
            }
        }
        else {
            temp = b"Unknown status\0" as *const u8 as *mut c_char;
        } 
    } 
    
    return temp;
}

#[macro_export]
macro_rules!  STRLEN{
    ($s:expr) => {
        if !$s.is_null() && *$s.offset(0 as isize) as c_int != 0
            {
                if *$s.offset(1 as isize) as c_int != 0 {
                    if *$s.offset(2 as isize) as c_int != 0 {
                        libc::strlen($s)
                    } else {
                        2 
                    }
                } else {
                    1 
                }
            } else {
                0 
            }
    };
}

#[macro_export]
macro_rules!  WIFCONTINUED{
    ($status:expr) => {
        $status == 0xffff
    };
}

#[macro_export]
macro_rules!  WIFCORED{
    ($status:expr) => {
        $status == 0x80
    };
}

#[macro_export]
macro_rules!  J_FOREGROUND{
    () => {
        0x01
    };
}

#[macro_export]
macro_rules!  IS_FOREGROUND{
    ($j:expr) => {
        (**jobs.offset($j as isize)).flags & J_FOREGROUND!() != 0
    };
}

unsafe extern "C" fn print_pipeline(
    mut p: *mut PROCESS,
    mut job_index: c_int,
    mut format: c_int,
    mut stream: *mut libc::FILE,
) {
    let mut first: *mut PROCESS = 0 as *mut PROCESS;
    let mut last: *mut PROCESS = 0 as *mut PROCESS;
    let mut show: *mut PROCESS = 0 as *mut PROCESS;
    let mut es: c_int = 0;
    let mut name_padding: c_int = 0;
    let mut temp: *mut c_char = 0 as *mut c_char;

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
                    b"     \0" as *const u8 as *const c_char
                } else {
                    b" |\0" as *const u8 as *const c_char
                },
            );
        }
        if format != 0  {
            libc::fprintf(
                stream,
                b"%5ld\0" as *const u8 as *const c_char,
                (*p).pid as libc::c_long,
            );
        }
        libc::fprintf(stream, b" \0" as *const u8 as *const c_char);

        if format > -1 && job_index >= 0  {
            show = if format != 0 { p } else { last };
            temp = printable_job_status(job_index, show, format);

            if p != first {
                if format != 0 {
                    if (*show).running == (*first).running
                        && WSTATUS!((*show).status) == WSTATUS!((*first).status) 
                    {
                        temp = b"\0" as *const u8 as *mut c_char;
                            
                    }
                } else {
                    temp = 0 as *mut c_char;
                }
            }

            if !temp.is_null() {
                libc::fprintf(stream, b"%s\0" as *const u8 as *const c_char, temp);
                es = STRLEN!(temp) as c_int;

                if es == 0 {
                    es = 2 ;
                }
                name_padding = LONGEST_SIGNAL_DESC as c_int - es;
                libc::fprintf(
                    stream,
                    b"%*s\0" as *const u8 as *const c_char,
                    name_padding,
                    b"\0" as *const u8 as *const c_char,
                );
                if (WIFSTOPPED!((*show).status)) as c_int == 0
                    && (WIFCONTINUED!((*show).status)) as c_int == 0 && WIFCORED!((*show).status) as c_int != 0
                {
                    libc::fprintf(stream, b"(core dumped) \0" as *const u8 as *const c_char );
                }
            }
        }
        if p != first && format != 0 {
            libc::fprintf(stream, b"| \0" as *const u8 as *const c_char);
        }
        if !((*p).command).is_null() {
            fprintf(stream, b"%s\0" as *const u8 as *const c_char, (*p).command);
        }
        if p == last && job_index >= 0 {
            temp = current_working_directory();
            if RUNNING!(job_index) && IS_FOREGROUND!(job_index) as c_int == 0
            {
                fprintf(stream, b" &\0" as *const u8 as *const c_char);
            }
            if libc::strcmp(temp, (**jobs.offset(job_index as isize)).wd) != 0 as c_int {
                fprintf(stream,b"  (wd: %s)\0" as *const u8 as *const c_char,
                    polite_directory_format((**jobs.offset(job_index as isize)).wd),
                );
            }
        }
        if format != 0 || p == last {
            if asynchronous_notification != 0 && interactive != 0 {
                libc::fputc('\r' as i32, stream);
            }
            fprintf(stream, b"\n\0" as *const u8 as *const c_char);
        }
        if p == last {
            break;
        }
        p = (*p).next;
    }
    libc::fflush(stream);
}

unsafe extern "C" fn pretty_print_job(
    mut job_index: c_int,
    mut format: c_int,
    mut stream: *mut libc::FILE,
) {
    let mut p: *mut PROCESS = 0 as *mut PROCESS;

    if format == JLIST_PID_ONLY as c_int {
        fprintf(
            stream,
            b"%ld\n\0" as *const u8 as *const c_char,
            (*(**jobs.offset(job_index as isize)).pipe).pid as libc::c_long,
        );
        return;
    }
    if format == JLIST_CHANGED_ONLY as c_int{
        if IS_NOTIFIED! (job_index)
        {
            return;
        }
        format = JLIST_STANDARD as c_int;
    }
    if format != JLIST_NONINTERACTIVE as c_int{
        fprintf(
            stream,
            b"[%d]%c \0" as *const u8 as *const c_char,
            job_index + 1 as c_int,
            if job_index == js.j_current {
                '+' as i32
            } else if job_index == js.j_previous {
                '-' as i32
            } else {
                ' ' as i32
            },
        );
    }
    if format == JLIST_NONINTERACTIVE as c_int{
        format = JLIST_LONG as c_int;
    }
    p = (**jobs.offset(job_index as isize)).pipe;
    print_pipeline(p, job_index, format, stream);
    (**jobs.offset(job_index as isize)).flags |= J_NOTIFIED as c_int;
}


unsafe extern "C" fn print_job(mut job: *mut JOB, mut format: c_int, mut state: c_int, mut job_index: c_int) -> c_int 
{
    if state == -(1 as c_int)
        || state as JOB_STATE as c_int == (*job).state as c_int
    {
        pretty_print_job(job_index, format, stdout);
    }
    return 0 as c_int;
}

#[no_mangle]
pub unsafe extern "C"  fn  list_one_job(mut job: *mut JOB, mut format: c_int, mut ignore: c_int,mut job_index: c_int,)
{
    pretty_print_job(job_index, format, stdout);
    cleanup_dead_jobs();
}

#[no_mangle]
pub unsafe extern "C"  fn  list_stopped_jobs(mut format: c_int) {
    cleanup_dead_jobs();
    map_over_jobs(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> c_int>,
            Option::<sh_job_map_func_t>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn(
                        *mut JOB,
                        c_int,
                        c_int,
                        c_int,
                    ) -> c_int,
                    unsafe extern "C" fn() -> c_int,
                >(print_job),
            ),
        ),
        format,
        JSTOPPED as c_int,
    );
}
#[no_mangle]
pub unsafe extern "C"  fn  list_running_jobs(mut format: c_int) {
    cleanup_dead_jobs();
    map_over_jobs(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> c_int>,
            Option::<sh_job_map_func_t>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn(
                        *mut JOB,
                        c_int,
                        c_int,
                        c_int,
                    ) -> c_int,
                    unsafe extern "C" fn() -> c_int,
                >(print_job),
            ),
        ),
        format,
        JRUNNING as c_int,
    );
}

#[no_mangle]
pub unsafe extern "C"  fn  list_all_jobs(mut format: c_int) {
    cleanup_dead_jobs();
    map_over_jobs(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> c_int>,
            Option::<sh_job_map_func_t>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn(
                        *mut JOB,
                        c_int,
                        c_int,
                        c_int,
                    ) -> c_int,
                    unsafe extern "C" fn() -> c_int,
                >(print_job),
            ),
        ),
        format,
        -(1 as c_int),
    );
}

#[macro_export]
macro_rules! errno {
    () => {
        *__errno_location()
    };
}

#[macro_export]
macro_rules! CLRINTERRUPT {
    () => {
        interrupt_state = 0
    };
}

#[no_mangle]
pub unsafe extern "C" fn make_child(
    mut command: *mut c_char,
    mut flags: c_int,
) -> pid_t {
    // println!("make_child");
    let mut async_p: c_int = 0;
    let mut forksleep: c_int = 0;
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };
    let mut termset: sigset_t = __sigset_t { __val: [0; 16] };
    let mut chldset: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset_copy: sigset_t = __sigset_t { __val: [0; 16] };
    let mut pid: pid_t = 0;
    let mut oterm:*mut SigHandler = 0 as *mut SigHandler;
    sigemptyset(&mut oset_copy);
    sigprocmask(
        0 as c_int,
        0 as *mut libc::c_void as *mut sigset_t,
        &mut oset_copy,
    );
    sigaddset(&mut oset_copy, 15 as c_int);
    sigemptyset(&mut set);
    sigaddset(&mut set, 17 as c_int);
    sigaddset(&mut set, 2 as c_int);
    sigaddset(&mut set, 15 as c_int);
    sigemptyset(&mut oset);
    sigprocmask(0 as c_int, &mut set, &mut oset);
    if interactive_shell != 0 {
        oterm = set_signal_handler(15 as c_int,  &mut None);
    }
    making_children();
    async_p = flags & 1 as c_int;
    forksleep = 1 as c_int;
    if default_buffered_input != -(1 as c_int)
        && (async_p == 0 || default_buffered_input > 0 as c_int)
    {
        sync_buffered_stream(default_buffered_input);
    }
    loop {
        pid = fork();
        if !(pid < 0 as c_int && *__errno_location() == 11 as c_int
            && forksleep < 16 as c_int)
        {
            break;
        }
        sigprocmask(
            2 as c_int,
            &mut oset_copy,
            0 as *mut libc::c_void as *mut sigset_t,
        );
        waitchld(-(1 as c_int), 0 as c_int);
        *__errno_location() = 11 as c_int;
        sys_error(b"fork: retry\0" as *const u8 as *const c_char);
        if sleep(forksleep as libc::c_uint) != 0 as c_int as libc::c_uint {
            break;
        }
        forksleep <<= 1 as c_int;
        if interrupt_state != 0 {
            break;
        }
        sigprocmask(2 as c_int, &mut set, 0 as *mut libc::c_void as *mut sigset_t);
    }

    if pid != 0 as c_int {
        if interactive_shell != 0 {
            set_signal_handler(15 as c_int, oterm);
        }
    }
    if pid < 0 as c_int {
        sys_error(b"fork\0" as *const u8 as *const c_char);
        terminate_current_pipeline();
        if !the_pipeline.is_null() {
            kill_current_pipeline();
        }
        set_exit_status(126 as c_int);
        throw_to_top_level();
    }

    if pid == 0 as c_int {
        let mut mypid: pid_t = 0;
        mypid = getpid();
        unset_bash_input(0 as c_int);
        ::std::ptr::write_volatile(
            &mut interrupt_state as *mut sig_atomic_t,
            0 as c_int,
        );
        restore_sigmask();
        if job_control != 0 {
            if pipeline_pgrp == 0 as c_int {
                pipeline_pgrp = mypid;
            }
            if pipeline_pgrp == shell_pgrp {
                ignore_tty_job_signals();
            } else {
                default_tty_job_signals();
            }
            if setpgid(mypid, pipeline_pgrp) < 0 as c_int {
                sys_error(
                    dcgettext(
                        0 as *const c_char,
                        b"child setpgid (%ld to %ld)\0" as *const u8
                            as *const c_char,
                        5 as c_int,
                    ),
                    mypid as libc::c_long,
                    pipeline_pgrp as libc::c_long,
                );
            }
            if flags & 4 as c_int == 0 as c_int
                && async_p == 0 as c_int && pipeline_pgrp != shell_pgrp
                && subshell_environment & (0x1 as c_int | 0x10 as c_int)
                    == 0 as c_int && running_in_background == 0 as c_int
            {
                give_terminal_to(pipeline_pgrp, 0 as c_int);
            }
            if pipeline_pgrp == mypid {
                pipe_read(pgrp_pipe.as_mut_ptr());
            }
        } else {
            if pipeline_pgrp == 0 as c_int {
                pipeline_pgrp = shell_pgrp;
            }
            default_tty_job_signals();
        }
        sh_closepipe(pgrp_pipe.as_mut_ptr());
    }  else {
        if job_control != 0 {
            if pipeline_pgrp == 0 as c_int {
                pipeline_pgrp = pid;
            }
            setpgid(pid, pipeline_pgrp);
        } else if pipeline_pgrp == 0 as c_int {
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
            2 as c_int,
            &mut oset,
            0 as *mut libc::c_void as *mut sigset_t,
        );
    }
    // println!("make child pid {}",pid);
    return pid;
}

#[no_mangle]
pub unsafe extern "C"  fn  ignore_tty_job_signals() {
    set_signal_handler(
        SIGTSTP as c_int,
        SIG_IGN!() 
    );
    set_signal_handler(
        SIGTTIN as c_int,
        SIG_IGN!() 
    );
    set_signal_handler(
        SIGTTOU as c_int,
        SIG_IGN!() 
    );
}

#[no_mangle]
pub unsafe extern "C"  fn  default_tty_job_signals() {
    if signal_is_trapped(SIGTSTP as c_int) == 0 
        && signal_is_hard_ignored(SIGTSTP as c_int) != 0
    {
        set_signal_handler(
            SIGTSTP as c_int,
            SIG_IGN!() 
        );
    } else {
        set_signal_handler(SIGTSTP as c_int, &mut None);
    }
    if signal_is_trapped(SIGTTIN as c_int) == 0 
        && signal_is_hard_ignored(SIGTTIN as c_int) != 0
    {
        set_signal_handler(
            SIGTTIN as c_int,
            SIG_IGN!() 
        );
    } else {
        set_signal_handler(SIGTTIN as c_int, &mut None);
    }
    if signal_is_trapped(SIGTTOU as c_int) == 0 
        && signal_is_hard_ignored(SIGTTOU as c_int) != 0
    {
        set_signal_handler(
            SIGTTOU as c_int,
            SIG_IGN!() 
        );
    } else {
        set_signal_handler(SIGTTOU as c_int, &mut None);
    };
}

#[no_mangle]
pub unsafe extern "C"  fn  get_original_tty_job_signals() {
    static mut fetched: c_int = 0 as c_int;
    if fetched == 0 as c_int {
        if interactive_shell != 0 {
            set_original_signal(SIGTSTP as c_int, None);
            set_original_signal(SIGTTIN as c_int, None);
            set_original_signal(SIGTTOU as c_int, None);
        } else {
            get_original_signal(SIGTSTP as c_int);
            get_original_signal(SIGTTIN as c_int);
            get_original_signal(SIGTTOU as c_int);
        }
        fetched = 1;
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

#[macro_export]
macro_rules! input_tty {
    () => {
        if shell_tty != -1 { shell_tty } else { fileno(stderr) }
    };
}

#[no_mangle]
pub unsafe extern "C"  fn  get_tty_state() -> c_int {
    let mut tty: c_int = 0;

    tty = input_tty!();
    if tty != -1 {
        if libc::tcgetattr(tty, &mut shell_tty_info) < 0 {
            return -(1 as c_int);
        }
        if check_window_size != 0 {
            get_new_window_size(0 as c_int, 0 as *mut c_int, 0 as *mut c_int);
        }
    }
    return 0 as c_int;
}

#[no_mangle]
pub unsafe extern "C"  fn  set_tty_state() -> c_int {
    let mut tty: c_int = 0;
    tty = input_tty!();
    if tty != -1 {
        if libc::tcsetattr(tty, 1 as c_int, &mut shell_tty_info) < 0 as c_int {
            if interactive != 0 {
                sys_error(
                    b"[%ld: %d (%d)] tcsetattr\0" as *const u8 as *const c_char,
                    getpid() as libc::c_long,
                    shell_level,
                    tty,
                );
            }
            return -1;
        }
    }
    return 0 ;
}

unsafe extern "C" fn find_last_proc(mut job: c_int, mut block: c_int) -> *mut PROCESS 
{
    let mut p: *mut PROCESS = 0 as *mut PROCESS;
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

    if block != 0 {
        BLOCK_CHILD (&mut  set, &mut oset);
    }
    p = (**jobs.offset(job as isize)).pipe;
    while !p.is_null() && (*p).next != (**jobs.offset(job as isize)).pipe {
        p = (*p).next;
    }
    if block != 0 {
        UNBLOCK_CHILD (&mut oset);
    }
    return p;
}

unsafe extern "C" fn find_last_pid(mut job: c_int, mut block: c_int) -> pid_t 
{
    let mut p: *mut PROCESS = 0 as *mut PROCESS;
    p = find_last_proc(job, block);
    return (*p).pid;
}

#[macro_export]
macro_rules! CHECK_WAIT_INTR {
    () => {
        if wait_intr_flag != 0 && wait_signal_received != 0 && this_shell_builtin.is_some()
        && this_shell_builtin 
            == (Some(wait_builtin as unsafe extern "C" fn(*mut WORD_LIST) -> c_int))
    {
        siglongjmp(wait_intr_buf.as_mut_ptr(), 1 as c_int);
    }
    };
}

#[macro_export]
macro_rules! CHECK_TERMSIG {
    () => {
        if terminating_signal != 0 {
            termsig_handler(terminating_signal);
        }
    };
}

#[no_mangle]
pub unsafe extern "C"  fn  wait_for_single_pid(mut pid: pid_t, mut flags: c_int) -> c_int 
{
    let mut child: *mut PROCESS = 0 as *mut PROCESS;
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };
    let mut r: c_int = 0;
    let mut job: c_int = 0;
    let mut alive: c_int = 0;

    BLOCK_CHILD (&mut set, &mut oset);
    child = find_pipeline(pid,0 ,0  as *mut c_int);
    UNBLOCK_CHILD (&mut oset);

    if child.is_null() {
        r = bgp_search(pid);
        if r >= 0  {
            return r;
        }
    }
    if child.is_null() {
        if flags & JWAIT_PERROR as c_int != 0 {
            internal_error(b"wait: pid %ld is not a child of this shell\0" as *const u8 as *const c_char,
                pid as libc::c_long,
            );
        }
        return 127 ;
    }
    alive = 0 ;
    loop {
        r = wait_for(pid, 0 );
        if flags & JWAIT_FORCE as c_int == 0 {
            break;
        }

        BLOCK_CHILD (&mut set, &mut oset);
        alive = PALIVE!(child) as c_int;
        UNBLOCK_CHILD (& mut oset);

        if !(alive != 0) {
            break;
        }
    }

    BLOCK_CHILD (&mut set, &mut oset);
    job = find_job(pid, 0 , 0 as *mut *mut PROCESS);
    if job != NO_JOB && !(*jobs.offset(job as isize)).is_null()
        && DEADJOB!(job)
    {
        (**jobs.offset(job as isize)).flags |= J_NOTIFIED as c_int;
    }
    UNBLOCK_CHILD (&mut oset);
    
    
    if posixly_correct != 0 {
        cleanup_dead_jobs();
        bgp_delete(pid);
    }
    
    CHECK_WAIT_INTR!();

    return r;
}

#[macro_export]
macro_rules! QUIT {
    () => {
        if terminating_signal != 0 {
            termsig_handler(terminating_signal);
        }
        if interrupt_state != 0 {
            throw_to_top_level();
        }
    };
}

#[no_mangle]
pub unsafe extern "C"  fn  wait_for_background_pids(mut ps: *mut procstat) {
    let mut i: c_int = 0;
    let mut r: c_int = 0;
    let mut any_stopped: c_int = 0;
    let mut check_async: c_int = 0;
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };
    let mut pid: pid_t = 0;


    any_stopped = 0 ;
    check_async = 1;
    loop {
        BLOCK_CHILD (&mut set, &mut set);

        i = 0;
        while i < js.j_jobslots {
            if !(*jobs.offset(i as isize)).is_null() && STOPPED! (i)
            {
                builtin_warning(
                    b"job %d[%d] stopped\0" as *const u8 as *const c_char,
                    i + 1 ,
                    find_last_pid(i, 0 ),
                );
                any_stopped = 1;
            }
            if !(*jobs.offset(i as isize)).is_null() && RUNNING!(i) && IS_FOREGROUND!(i)
            {
                break;
            }
            i += 1;
        }
        if i == js.j_jobslots {
            UNBLOCK_CHILD (&mut oset);
            break;
        } 

        pid = find_last_pid(i, 0  );
        UNBLOCK_CHILD (&mut oset);
        QUIT!();
        if terminating_signal != 0 {
            termsig_handler(terminating_signal);
        }
        if interrupt_state != 0 {
            throw_to_top_level();
        }
        *__errno_location() = 0 ;
        r = wait_for_single_pid(pid, JWAIT_PERROR as c_int);
        if !ps.is_null() {
            (*ps).pid = pid;
            (*ps)
                .status = (if r < 0 as c_int { 127 as c_int } else { r })
                as libc::c_short;
        }
        if r == -1 && *__errno_location() == ECHILD as c_int {
            check_async = 0 ;
            mark_all_jobs_as_dead();
        }
        
    }
    procsub_waitall();
    mark_dead_jobs_as_notified(1);
    cleanup_dead_jobs();
    bgp_clear();
}

pub type SigHandler1 = Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>;
#[macro_export]
macro_rules! INVALID_SIGNAL_HANDLER {
    () => {
        wait_for_background_pids as *mut SigHandler
    }
}
static mut old_sigint_handler:*mut SigHandler = INVALID_SIGNAL_HANDLER!();




unsafe extern "C" fn restore_sigint_handler() {
    if old_sigint_handler != INVALID_SIGNAL_HANDLER!()
    {
        set_signal_handler(SIGINT as c_int, old_sigint_handler);     
        old_sigint_handler = INVALID_SIGNAL_HANDLER!();
        waiting_for_child = 0 ;
    }
}



unsafe extern "C" fn wait_sigint_handler(mut sig: c_int) {
    println!("wait_sigint_handler");
    let mut sigint_handler: Option::<SigHandler> = None;

    if this_shell_builtin.is_some() && this_shell_builtin == Some(wait_builtin) 
    {
        set_exit_status(128 + SIGINT as c_int);
        restore_sigint_handler();
        if this_shell_builtin.is_some()
            && this_shell_builtin == Some(wait_builtin)  && signal_is_trapped(SIGINT as c_int) != 0
            && {
                sigint_handler = Some(trap_to_sighandler(SIGINT as c_int));
                sigint_handler
                    == Some(Some(trap_handler as unsafe extern "C" fn(c_int) -> ()))
            }
        {
            trap_handler(SIGINT as c_int);
            wait_signal_received = SIGINT as c_int;
            if wait_intr_flag != 0 {
                siglongjmp(wait_intr_buf.as_mut_ptr(), 1 );
            } else {
                return
            }
        } else {
            kill(getpid(), SIGINT as c_int);
        }
    }
    if waiting_for_child != 0 {
        wait_sigint_received = 1 ;
    } else {
        set_exit_status(128  + SIGINT as c_int);
        restore_sigint_handler();
        kill(getpid(), SIGINT as c_int);
    };
}


unsafe extern "C" fn process_exit_signal(mut status: WAIT) -> c_int {

    if libc::WIFSIGNALED(status) {
        return libc::WTERMSIG(status);
    } else {
        return 0;
    }
}


unsafe extern "C" fn process_exit_status(mut status: WAIT) -> c_int {
    if WIFSIGNALED!(status) {
        return 128 + WTERMSIG! (status);
    } else if WIFSTOPPED! (status) as c_int == 0 {
        return WEXITSTATUS!(status);
    } else {
        return 0;
    }

}

unsafe extern "C" fn job_signal_status(mut job: c_int) -> WAIT {
    let mut p: *mut PROCESS = 0 as *mut PROCESS;
    let mut s: WAIT = 0;

    p = (**jobs.offset(job as isize)).pipe;
    loop {
        s = (*p).status;
        if WIFSIGNALED!(s) || WIFSTOPPED!(s)
        {
            break;
        }
        p = (*p).next;
        if !(p != (**jobs.offset(job as isize)).pipe) {
            break;
        }
    }
    return s;
}

unsafe extern "C" fn raw_job_exit_status(mut job: c_int) -> WAIT {
    let mut p: *mut PROCESS = 0 as *mut PROCESS;
    let mut fail: c_int = 0;
    let mut ret: WAIT = 0;

    if (**jobs.offset(job as isize)).flags & J_PIPEFAIL as c_int != 0 {
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


#[no_mangle]
pub unsafe extern "C"  fn  job_exit_status(mut job: c_int) -> c_int {
    return process_exit_status(raw_job_exit_status(job));
}
#[no_mangle]
pub unsafe extern "C"  fn  job_exit_signal(mut job: c_int) -> c_int {
    return process_exit_signal(raw_job_exit_status(job));
}

pub const ANY_PID:pid_t = -1;
#[macro_export]
macro_rules! IS_JOBCONTROL {
    ($job:expr) => {
        (**jobs.offset($job as isize)).flags & 0x4 as c_int
                    != 0 as c_int
    };
}

#[macro_export]
macro_rules! RL_ISSTATE {
    ($x:expr) => {
        rl_readline_state & ($x)
    };
}
#[macro_export]
macro_rules! RL_STATE_COMPLETING {
    () => {
        0x0004000
    };
}

#[macro_export]
macro_rules! ADDINTERRUPT {
    () => {
        interrupt_state += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn wait_for(
    mut pid: pid_t,
    mut flags: c_int,
) -> c_int {
    let mut current_block: u64;
    let mut job: c_int = 0;
    let mut termination_state: c_int = 0;
    let mut r: c_int = 0;
    let mut s: WAIT = 0;
    let mut child: *mut PROCESS = 0 as *mut PROCESS;
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };
    child = 0 as *mut PROCESS;
    sigemptyset(&mut set);
    sigaddset(&mut set, 17 as c_int);
    sigemptyset(&mut oset);
    sigprocmask(0 as c_int, &mut set, &mut oset);
    child_caught_sigint = 0 as c_int;
    wait_sigint_received = child_caught_sigint;
    if job_control == 0 as c_int || subshell_environment & 0x4 as c_int != 0
    {
        let mut temp_sigint_handler: *mut SigHandler;
        temp_sigint_handler = set_signal_handler(SIGINT as c_int, wait_sigint_handler as *mut Option<unsafe extern "C" fn(i32)>);
        if !(temp_sigint_handler == wait_sigint_handler as *mut SigHandler) {
                        old_sigint_handler = temp_sigint_handler;
                    }
                    waiting_for_child = 0;
                    if old_sigint_handler
                        == &mut ::std::mem::transmute::<
                            libc::intptr_t,
                            __sighandler_t,
                        >(1 as c_int as libc::intptr_t) 
                    {
                        set_signal_handler(2 as c_int, old_sigint_handler);
                    }
    }
    termination_state = last_command_exit_value;
    if interactive != 0 && job_control == 0 as c_int {
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
    if wait_intr_flag != 0 && wait_signal_received != 0 && this_shell_builtin.is_some()
        && this_shell_builtin
            == Some(wait_builtin as unsafe extern "C" fn(*mut WordList) -> c_int)
    {
        siglongjmp(wait_intr_buf.as_mut_ptr(), 1 as c_int);
    }
    job = -(1 as c_int);
    loop {
        if pid != -(1 as c_int) {
            child = find_pipeline(
                pid,
                0 as c_int,
                0 as *mut libc::c_void as *mut c_int,
            );
            if child.is_null() {
                give_terminal_to(shell_pgrp, 0 as c_int);
                sigprocmask(
                    2 as c_int,
                    &mut oset,
                    0 as *mut libc::c_void as *mut sigset_t,
                );
                internal_error(
                    dcgettext(
                        0 as *const c_char,
                        b"wait_for: No record of process %ld\0" as *const u8
                            as *const c_char,
                        5 as c_int,
                    ),
                    pid as libc::c_long,
                );
                restore_sigint_handler();
                termination_state = 127 as c_int;
                return termination_state;
            }
        }
        if job == -(1 as c_int) && pid != -(1 as c_int) {
            job = find_job(pid, 0 as c_int, 0 as *mut *mut PROCESS);
        }
        if pid == -(1 as c_int) || (*child).running == 1 as c_int
            || job != -(1 as c_int)
                && (**jobs.offset(job as isize)).state as c_int
                    == JRUNNING as c_int
        {
            let mut old_waiting: c_int = 0;
            queue_sigchld = 1 as c_int;
            old_waiting = waiting_for_child;
            waiting_for_child = 1 as c_int;
            if wait_intr_flag != 0 && wait_signal_received != 0
                && this_shell_builtin.is_some()
                && this_shell_builtin
                    == Some(
                        wait_builtin
                            as unsafe extern "C" fn(*mut WordList) -> c_int,
                    )
            {
                siglongjmp(wait_intr_buf.as_mut_ptr(), 1 as c_int);
            }
            r = waitchld(pid, 1 as c_int);
            waiting_for_child = old_waiting;
            queue_sigchld = 0 as c_int;
            if r == -(1 as c_int) && *__errno_location() == 10 as c_int
                && this_shell_builtin
                    == Some(
                        wait_builtin
                            as unsafe extern "C" fn(*mut WordList) -> c_int,
                    )
            {
                termination_state = -(1 as c_int);
                restore_sigint_handler();
                current_block = 6718615339517147058;
                break;
            } else if r == -(1 as c_int)
                    && *__errno_location() == 10 as c_int
                {
                if !child.is_null() {
                    (*child).running = 0 as c_int;
                    (*child).status = 0 as c_int;
                }
                js.c_living = 0 as c_int;
                if job != -(1 as c_int) {
                    (**jobs.offset(job as isize)).state = JDEAD;
                    js.c_reaped += 1;
                    js.j_ndead += 1;
                }
                if pid == -(1 as c_int) {
                    termination_state = -(1 as c_int);
                    current_block = 7072655752890836508;
                    break;
                }
            }
        }
        if interactive != 0 && job_control == 0 as c_int {
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
        if wait_intr_flag != 0 && wait_signal_received != 0
            && this_shell_builtin.is_some()
            && this_shell_builtin
                == Some(
                    wait_builtin as unsafe extern "C" fn(*mut WordList) -> c_int,
                )
        {
            siglongjmp(wait_intr_buf.as_mut_ptr(), 1 as c_int);
        }
        if pid == -(1 as c_int) {
            restore_sigint_handler();
            current_block = 6718615339517147058;
            break;
        } else if !((*child).running == 1 as c_int
                || job != -(1 as c_int)
                    && (**jobs.offset(job as isize)).state as c_int
                        == JRUNNING as c_int)
            {
            current_block = 7072655752890836508;
            break;
        }
    }
    match current_block {
        7072655752890836508 => {
            restore_sigint_handler();
            termination_state = if job != -(1 as c_int) {
                job_exit_status(job)
            } else if !child.is_null() {
                process_exit_status((*child).status)
            } else {
                0 as c_int
            };
            last_command_exit_signal = if job != -(1 as c_int) {
                job_exit_signal(job)
            } else if !child.is_null() {
                process_exit_signal((*child).status)
            } else {
                0 as c_int
            };
            if job != -(1 as c_int)
                && (**jobs.offset(job as isize)).state as c_int
                    == JSTOPPED as c_int
                || !child.is_null()
                    && (*child).status & 0xff as c_int == 0x7f as c_int
            {
                termination_state = 128 as c_int
                    + (((*child).status & 0xff00 as c_int) >> 8 as c_int);
            }
            if job == -(1 as c_int)
                || (**jobs.offset(job as isize)).flags & 0x4 as c_int
                    != 0 as c_int
            {
                if flags & (1 as c_int) << 8 as c_int == 0 as c_int
                    && running_in_background == 0 as c_int
                    && subshell_environment & (0x1 as c_int | 0x10 as c_int)
                        == 0 as c_int
                {
                    give_terminal_to(shell_pgrp, 0 as c_int);
                }
            }
            if job != -(1 as c_int) {
                if interactive_shell != 0 && subshell_environment == 0 as c_int {
                    s = job_signal_status(job);
                    if ((s & 0x7f as c_int) + 1 as c_int) as libc::c_schar
                        as c_int >> 1 as c_int > 0 as c_int
                        || s & 0xff as c_int == 0x7f as c_int
                    {
                        set_tty_state();
                        if check_window_size != 0
                            && (job == js.j_current
                                || (**jobs.offset(job as isize)).flags & 0x1 as c_int
                                    != 0 as c_int)
                        {
                            get_new_window_size(
                                0 as c_int,
                                0 as *mut c_int,
                                0 as *mut c_int,
                            );
                        }
                    } else if rl_readline_state & 0x4000 as c_int as libc::c_ulong
                            == 0 as c_int as libc::c_ulong
                        {
                        get_tty_state();
                    }
                    if job_control != 0
                        && (**jobs.offset(job as isize)).flags & 0x4 as c_int
                            != 0 as c_int
                        && (**jobs.offset(job as isize)).flags & 0x1 as c_int
                            != 0 as c_int
                        && ((s & 0x7f as c_int) + 1 as c_int)
                            as libc::c_schar as c_int >> 1 as c_int
                            > 0 as c_int
                        && s & 0x7f as c_int == 2 as c_int
                    {
                        if signal_is_trapped(2 as c_int) == 0 as c_int
                            && (loop_level != 0
                                || shell_compatibility_level > 32 as c_int
                                    && executing_list != 0)
                        {
                            ::std::ptr::write_volatile(
                                &mut interrupt_state as *mut sig_atomic_t,
                                ::std::ptr::read_volatile::<
                                    sig_atomic_t,
                                >(&interrupt_state as *const sig_atomic_t) + 1,
                            );
                        } else if signal_is_trapped(2 as c_int) != 0
                                && loop_level != 0
                            {
                            ::std::ptr::write_volatile(
                                &mut interrupt_state as *mut sig_atomic_t,
                                ::std::ptr::read_volatile::<
                                    sig_atomic_t,
                                >(&interrupt_state as *const sig_atomic_t) + 1,
                            );
                        } else if interactive_shell != 0
                                && signal_is_trapped(2 as c_int) == 0 as c_int
                                && sourcelevel != 0
                            {
                            ::std::ptr::write_volatile(
                                &mut interrupt_state as *mut sig_atomic_t,
                                ::std::ptr::read_volatile::<
                                    sig_atomic_t,
                                >(&interrupt_state as *const sig_atomic_t) + 1,
                            );
                        } else {
                            putchar('\n' as i32);
                            libc::fflush(stdout);
                        }
                    }
                } else if subshell_environment
                        & (0x4 as c_int | 0x10 as c_int) != 0
                        && wait_sigint_received != 0
                    {
                    if child_caught_sigint == 0 as c_int
                        && signal_is_trapped(2 as c_int) == 0 as c_int
                    {
                        sigprocmask(
                            2 as c_int,
                            &mut oset,
                            0 as *mut libc::c_void as *mut sigset_t,
                        );
                        old_sigint_handler = set_signal_handler(2 as c_int, &mut None);
                        if old_sigint_handler == SIG_IGN!()
                        {
                            restore_sigint_handler();
                        } else {
                            kill(getpid(), 2 as c_int);
                        }
                    }
                } else if interactive_shell == 0 as c_int
                        && subshell_environment == 0 as c_int
                        && (**jobs.offset(job as isize)).flags & 0x1 as c_int
                            != 0 as c_int
                    {
                    s = job_signal_status(job);
                    if job_control != 0
                        && (**jobs.offset(job as isize)).flags & 0x4 as c_int
                            != 0 as c_int
                        && ((s & 0x7f as c_int) + 1 as c_int)
                            as libc::c_schar as c_int >> 1 as c_int
                            > 0 as c_int
                        && s & 0x7f as c_int == 2 as c_int
                    {
                        ::std::ptr::write_volatile(
                            &mut interrupt_state as *mut sig_atomic_t,
                            ::std::ptr::read_volatile::<
                                sig_atomic_t,
                            >(&interrupt_state as *const sig_atomic_t) + 1,
                        );
                    }
                    if check_window_size != 0 {
                        get_new_window_size(
                            0 as c_int,
                            0 as *mut c_int,
                            0 as *mut c_int,
                        );
                    }
                }
                if (**jobs.offset(job as isize)).state as c_int
                    == JDEAD as c_int
                    && (**jobs.offset(job as isize)).flags & 0x1 as c_int
                        != 0 as c_int
                {
                    setjstatus(job);
                }
                notify_and_cleanup();
            }
        }
        _ => {}
    }
    sigprocmask(2 as c_int, &mut oset, 0 as *mut libc::c_void as *mut sigset_t);
    return termination_state;
}

#[no_mangle]
pub unsafe extern "C"  fn  wait_for_job(mut job: c_int, mut flags: c_int, mut ps: *mut procstat) -> c_int 
{
    let mut pid: pid_t = 0;
    let mut r: c_int = 0;
    let mut state: c_int = 0;
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

    BLOCK_CHILD(&mut set, &mut oset);
    state = JOBSTATE!(job);
    if state == JSTOPPED as c_int {
        internal_warning(b"wait_for_job: job %d is stopped\0" as *const u8 as *const c_char,
                job + 1 as c_int);
    }

    pid = find_last_pid(job, 0 );
    UNBLOCK_CHILD(&mut oset);

    loop {
        r = wait_for(pid, 0);
        if r == -1 && errno!() == ECHILD {
            mark_all_jobs_as_dead();
        }

        CHECK_WAIT_INTR!();

        if flags & JWAIT_FORCE as c_int == 0  {
            break;
        }
        BLOCK_CHILD (&mut set, &mut oset);
        state = if job != NO_JOB && !(*jobs.offset(job as isize)).is_null()
        {
            JOBSTATE!(job)
        } else {
            JDEAD as c_int
        };
        UNBLOCK_CHILD (&mut oset);
        if !(state != JDEAD as c_int) {
            break;
        }
    }

    BLOCK_CHILD (&mut set, &mut oset);
    if job != NO_JOB && !(*jobs.offset(job as isize)).is_null() && DEADJOB!(job)
    {
        (**jobs.offset(job as isize)).flags |= J_NOTIFIED as c_int;
    }
    UNBLOCK_CHILD (&mut oset);
    if !ps.is_null() {
        (*ps).pid = pid;
        (*ps).status = (if r < 0  { 127  } else { r }) as libc::c_short;
    }
    return r;
}

#[macro_export]
macro_rules! IS_WAITING {
    ($i:expr) => {
        (**jobs.offset($i as isize)).flags & 0x80 as c_int != 0 as c_int
    };
}

#[no_mangle]
pub unsafe extern "C"  fn  wait_for_any_job(mut flags: c_int, mut ps: *mut procstat) -> c_int 
{
    let mut current_block: u64;
    let mut pid: pid_t = 0;
    let mut i: c_int = 0;
    let mut r: c_int = 0;
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

    if jobs_list_frozen != 0 {
        return -1;
    }

    BLOCK_CHILD (&mut set, &mut oset);
    i = 0 as c_int;
    loop {
        if i < js.j_jobslots {
            if !(flags & JWAIT_WAITING as c_int != 0 && !(*jobs.offset(i as isize)).is_null()
                && IS_WAITING!(i) as c_int == 0)
            {
                if !(*jobs.offset(i as isize)).is_null() && DEADJOB!(i) && IS_NOTIFIED! (i)  as c_int == 0
                {
                    current_block = 2729223887955387488;
                    break;
                }
            }
            i += 1;
        } else {
            UNBLOCK_CHILD (&mut oset);
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
                BLOCK_CHILD (&mut set, &mut oset);
                i = 0 as c_int;
                while i < js.j_jobslots {
                    if !(*jobs.offset(i as isize)).is_null()
                        && RUNNING!(i)
                        &&  IS_FOREGROUND!(i) as c_int == 0
                    {
                        break;
                    }
                    i += 1;
                }
                if i == js.j_jobslots {
                    UNBLOCK_CHILD (&mut oset);
                    return -1;
                }
                UNBLOCK_CHILD (&mut oset);

                QUIT!();
                CHECK_TERMSIG!();
                CHECK_WAIT_INTR!();

                errno!() = 0;
                r = wait_for(ANY_PID, 0 );
                if r == -1 && errno!() == ECHILD {
                    mark_all_jobs_as_dead();
                }
                BLOCK_CHILD (&mut set, &mut oset);

                i = 0 as c_int;
                while i < js.j_jobslots {
                    if !(flags & JWAIT_WAITING as c_int != 0 && !(*jobs.offset(i as isize)).is_null() && IS_WAITING!(i) as c_int == 0)
                    {
                        if !(*jobs.offset(i as isize)).is_null()
                            && DEADJOB!(i)
                        {
                            current_block = 2729223887955387488;
                            continue 'c_22951;
                        }
                    }
                    i += 1;
                }
                UNBLOCK_CHILD (&mut oset);
                current_block = 7172762164747879670;
            }
        }
    }
    pid = find_last_pid(i, 0 );
    if !ps.is_null() {
        (*ps).pid = pid;
        (*ps).status = r as libc::c_short;
    }
    notify_of_job_status();
    delete_job(i, 0 );
    coproc_reap();
    UNBLOCK_CHILD (&mut oset);
    return r;
}

#[no_mangle]
pub unsafe extern "C"  fn  notify_and_cleanup() {
    if jobs_list_frozen != 0 {
        return;
    }
    if interactive != 0 || interactive_shell == 0  || sourcelevel != 0 {
        notify_of_job_status();
    }
    cleanup_dead_jobs();
}

#[no_mangle]
pub unsafe extern "C"  fn  reap_dead_jobs() {
    mark_dead_jobs_as_notified(0);
    cleanup_dead_jobs();
}

unsafe extern "C" fn most_recent_job_in_state(
    mut job: c_int,
    mut state: JOB_STATE,
) -> c_int {
    let mut i: c_int = 0;
    let mut result: c_int = 0;
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };
    sigemptyset(&mut set);
    sigaddset(&mut set, 17 as c_int);
    sigemptyset(&mut oset);
    sigprocmask(0 as c_int, &mut set, &mut oset);
    result = -(1 as c_int);
    i = job - 1 as c_int;
    while i >= 0 as c_int {
        if !(*jobs.offset(i as isize)).is_null()
            && (**jobs.offset(i as isize)).state as c_int == state as c_int
        {
            result = i;
            break;
        } else {
            i -= 1;
        }
    }
    sigprocmask(2 as c_int, &mut oset, 0 as *mut libc::c_void as *mut sigset_t);
    return result;
}

unsafe extern "C" fn job_last_stopped(mut job: c_int) -> c_int {
    return most_recent_job_in_state(job, JSTOPPED as c_int);
}

unsafe extern "C" fn job_last_running(mut job: c_int) -> c_int {
    return most_recent_job_in_state(job, JRUNNING as c_int);
}


unsafe extern "C" fn set_current_job(mut job: c_int) {
    let mut candidate: c_int = 0;

    if js.j_current != job {
        js.j_previous = js.j_current;
        js.j_current = job;
    }

    if js.j_previous != js.j_current && js.j_previous != NO_JOB
        && !(*jobs.offset(js.j_previous as isize)).is_null()
        && (**jobs.offset(js.j_previous as isize)).state as c_int
            == JSTOPPED as c_int
    {
        return;
    }
    candidate = NO_JOB;
    if (**jobs.offset(js.j_current as isize)).state as c_int
        == JSTOPPED as c_int
    {
        candidate = job_last_stopped(js.j_current);
        if candidate != NO_JOB {
            js.j_previous = candidate;
            return;
        }
    }
    candidate = if (**jobs.offset(js.j_current as isize)).state as c_int
        == JRUNNING as c_int
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
unsafe extern "C" fn reset_current() {
    let mut candidate: c_int = 0;

    if js.j_jobslots != 0 && js.j_current != NO_JOB
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

unsafe extern "C" fn set_job_running(mut job: c_int) {
    let mut p: *mut PROCESS = 0 as *mut PROCESS;

    p = (**jobs.offset(job as isize)).pipe;
    loop {
        if WIFSTOPPED!((*p).status) {
            (*p).running = PS_RUNNING as c_int;
        }
        p = (*p).next;
        if !(p != (**jobs.offset(job as isize)).pipe) {
            break;
        }
    }
    JOBSTATE!(job) = JRUNNING;
}

pub type cc_t = libc::c_uchar;
pub type speed_t = libc::c_uint;
pub type tcflag_t = libc::c_uint;

#[no_mangle]
pub unsafe extern "C"  fn  start_job(mut job: c_int, mut foreground: c_int) -> c_int 
{
    let mut p: *mut PROCESS = 0 as *mut PROCESS;
    let mut already_running: c_int = 0;
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };
    let mut wd: *mut c_char = 0 as *mut c_char;
    let mut s: *mut c_char = 0 as *mut c_char;
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

    BLOCK_CHILD (&mut set, &mut oset);
    if subshell_environment & SUBSHELL_COMSUB as c_int != 0 && pipeline_pgrp == shell_pgrp {
        internal_error(
            b"%s: no current jobs\0" as *const u8 as *const c_char,               
            this_command_name,
        );
        UNBLOCK_CHILD (&mut oset);
        return -1;
    }
    if DEADJOB!(job){
        internal_error(           
            b"%s: job has terminated\0" as *const u8 as *const c_char,             
            this_command_name,
        );
        UNBLOCK_CHILD(&mut oset);
        return -1;
    }
    already_running = RUNNING! (job) as c_int;

    if foreground == 0  && already_running != 0 {
        internal_error(                          
            b"%s: job %d already in background\0" as *const u8 as *const c_char,              
            this_command_name,
            job + 1 as c_int,
        );
        UNBLOCK_CHILD (&mut oset);
        return 0 ;
    }

    wd = current_working_directory();

    (**jobs.offset(job as isize)).flags &= !(J_NOTIFIED as c_int);

    if foreground != 0 {
        set_current_job(job);
        (**jobs.offset(job as isize)).flags |= J_FOREGROUND as c_int;
    }

    p = (**jobs.offset(job as isize)).pipe;

    if foreground == 0  {
        if posixly_correct == 0 as c_int {
            s = (if job == js.j_current {
                b"+ \0" as *const u8 as *const c_char
            } else if job == js.j_previous {
                b"- \0" as *const u8 as *const c_char
            } else {
                b" \0" as *const u8 as *const c_char
            }) as *mut c_char;
        } else {
            s = b" \0" as *const u8 as *const c_char as *mut c_char;
        }
        print!("[{}]{}",job + 1 as c_int, CStr::from_ptr(s).to_str().unwrap());
    }
    loop {
        print!(
            "{}{}",
            if !((*p).command).is_null() {
                let str1 = (*p).command;
                CStr::from_ptr(str1).to_str().unwrap()
            } else {
                let str1 = b"\0" as *const u8 as *const c_char;
                CStr::from_ptr(str1).to_str().unwrap()
            },

            if (*p).next != (**jobs.offset(job as isize)).pipe {
                let str1 =  b" | \0" as *const u8 as *const c_char;
                CStr::from_ptr(str1).to_str().unwrap()
            } else {
                let str1 = b"\0" as *const u8 as *const c_char;
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
    if strcmp(wd, (**jobs.offset(job as isize)).wd) != 0  {
        let str1 = polite_directory_format((**jobs.offset(job as isize)).wd);
        print!(
            " (wd:{})",
            CStr::from_ptr(str1).to_str().unwrap()
        );
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
        (**jobs.offset(job as isize)).flags &= !(J_FOREGROUND as c_int);
    }
    if already_running == 0 {
        (**jobs.offset(job as isize)).flags |= J_NOTIFIED as c_int;
        killpg((**jobs.offset(job as isize)).pgrp, 18 as c_int);
    }
    if foreground != 0 {
        let mut pid: pid_t = 0;
        let mut st: c_int = 0;

        pid = find_last_pid(job, 0 as c_int);
        UNBLOCK_CHILD (&mut oset);
        st = wait_for(pid, 0 );
        shell_tty_info = save_stty;
        set_tty_state();
        return st;
    } else {
        reset_current();
        UNBLOCK_CHILD (&mut set);
        return 0 as c_int;
    };
}

#[macro_export]
macro_rules! PEXITED {
    ($p:expr) => {
        (*$p).running  == 0 as c_int
    };
}


#[no_mangle]
pub unsafe extern "C"  fn  kill_pid(mut pid: pid_t, mut sig: c_int, mut group: c_int) -> c_int 
{
    let mut p: *mut PROCESS = 0 as *mut PROCESS;
    let mut job: c_int = 0;
    let mut result: c_int = 0;
    let mut negative: c_int = 0;
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

    if pid < -1 {
        pid = -pid;
        negative = 1;
        group = negative;
    } else {
        negative = 0 ;
    }
    result = 0 ;
    if group != 0 {
        BLOCK_CHILD ( &mut set, &mut oset);
        p = find_pipeline(pid, 0, &mut job);

        if job != NO_JOB  {
            (**jobs.offset(job as isize)).flags &= !(J_NOTIFIED as c_int);
            if negative != 0 && (**jobs.offset(job as isize)).pgrp == shell_pgrp {
                result = killpg(pid, sig);
            } else if (**jobs.offset(job as isize)).pgrp == shell_pgrp {
                p = (**jobs.offset(job as isize)).pipe;
                loop {
                    if !(PALIVE!(p) as c_int == 0)
                    {
                        kill((*p).pid, sig);
                        if PEXITED!(p)
                            && (sig == SIGTERM as c_int || sig == SIGHUP as c_int)
                        {
                            kill((*p).pid, SIGCONT as c_int);
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
                    && (sig == SIGTERM as c_int || sig == SIGHUP as c_int)
                {
                    killpg((**jobs.offset(job as isize)).pgrp, SIGCONT as c_int);
                }
                if !p.is_null()
                    && STOPPED!(job) && sig == SIGCONT as c_int
                {
                    set_job_running(job);
                    (**jobs.offset(job as isize)).flags &= !(J_FOREGROUND as c_int);
                    (**jobs.offset(job as isize)).flags |= J_NOTIFIED as c_int;
                }
            }
        } else {
            result = killpg(pid, sig);
        }
        UNBLOCK_CHILD (&mut oset);
    } else {
        result = kill(pid, sig);
    }
    return result;
}

unsafe extern "C" fn sigchld_handler(mut sig: c_int) {
    let mut n: c_int = 0;
    let mut oerrno: c_int = 0;

    oerrno = errno!();
    sigchld += 1;
    n = 0 ;
    if queue_sigchld == 0  {
        n = waitchld(-1, 0 );
    }
    errno!() = oerrno;
    return;
}


#[macro_export]
macro_rules! sh_longjmp {
    ($x:expr,$n:expr) => {
        siglongjmp($x, $n as c_int)
    };
}


pub const EINVAL:i32 = 22;
pub const EINTR:i32 = 22;

#[macro_export]
macro_rules! IMPOSSIBLE_TRAP_HANDLER {
    () => {
        initialize_traps as *mut SigHandler
    };
}

#[macro_export]
macro_rules! IGNORE_SIG {
    () => {
        SIG_IGN!() 
    };
}
unsafe extern "C" fn waitchld(mut wpid: pid_t, mut block: c_int) -> c_int {
    let mut status: WAIT = 0;
    let mut child: *mut PROCESS = 0 as *mut PROCESS;
    let mut pid: pid_t = 0;
    let mut ind: c_int = 0;
    let mut call_set_current: c_int = 0;
    let mut last_stopped_job: c_int = 0;
    let mut job: c_int = 0;
    let mut children_exited: c_int = 0;
    let mut waitpid_flags: c_int = 0;
    static mut wcontinued: c_int = 8 as c_int;
    children_exited = 0 as c_int;
    call_set_current = children_exited;
    last_stopped_job = -(1 as c_int);
    loop {
        waitpid_flags = if job_control != 0 && subshell_environment == 0 as c_int {
            2 as c_int | wcontinued
        } else {
            0 as c_int
        };
        if sigchld != 0 || block == 0 as c_int {
            waitpid_flags |= 1 as c_int;
        }
        if terminating_signal != 0 {
            termsig_handler(terminating_signal);
        }
        if wait_intr_flag != 0 && wait_signal_received != 0
            && this_shell_builtin.is_some()
            && this_shell_builtin
                == Some(
                    wait_builtin as unsafe extern "C" fn(*mut WordList) -> c_int,
                )
        {
            siglongjmp(wait_intr_buf.as_mut_ptr(), 1 as c_int);
        }
        if block == 1 as c_int && queue_sigchld == 0 as c_int
            && waitpid_flags & 1 as c_int == 0 as c_int
        {
            internal_warning(
                dcgettext(
                    0 as *const c_char,
                    b"waitchld: turning on WNOHANG to avoid indefinite block\0"
                        as *const u8 as *const c_char,
                    5 as c_int,
                ),
            );
            waitpid_flags |= 1 as c_int;
        }
        pid = waitpid(-(1 as c_int), &mut status, waitpid_flags);
        if wcontinued != 0 && pid < 0 as c_int
            && *__errno_location() == 22 as c_int
        {
            wcontinued = 0 as c_int;
        } else {
            if sigchld > 0 as c_int && waitpid_flags & 1 as c_int != 0 {
                sigchld -= 1;
            }
            if pid < 0 as c_int && *__errno_location() == 10 as c_int {
                if !(children_exited == 0 as c_int) {
                    break;
                }
                return -(1 as c_int);
            } else {
                if terminating_signal != 0 {
                    termsig_handler(terminating_signal);
                }
                if wait_intr_flag != 0 && wait_signal_received != 0
                    && this_shell_builtin.is_some()
                    && this_shell_builtin
                        == Some(
                            wait_builtin
                                as unsafe extern "C" fn(*mut WordList) -> c_int,
                        )
                {
                    siglongjmp(wait_intr_buf.as_mut_ptr(), 1 as c_int);
                }
                if pid < 0 as c_int && *__errno_location() == 4 as c_int
                    && wait_sigint_received != 0
                {
                    child_caught_sigint = 1 as c_int;
                }
                if !(pid <= 0 as c_int) {
                    if wait_sigint_received != 0
                        && ((((status & 0x7f as c_int) + 1 as c_int)
                            as libc::c_schar as c_int >> 1 as c_int
                            > 0 as c_int) as c_int == 0 as c_int
                            || status & 0x7f as c_int != 2 as c_int)
                    {
                        child_caught_sigint = 1 as c_int;
                    }
                    if ((status & 0x7f as c_int) + 1 as c_int)
                        as libc::c_schar as c_int >> 1 as c_int
                        > 0 as c_int
                        && status & 0x7f as c_int == 2 as c_int
                    {
                        child_caught_sigint = 0 as c_int;
                    }
                    if (status == 0xffff as c_int) as c_int
                        == 0 as c_int
                    {
                        children_exited += 1;
                        js.c_living -= 1;
                    }
                    child = find_process(pid, 1 as c_int, &mut job);
                    coproc_pidchk(pid, status);
                    ind = find_procsub_child(pid);
                    if ind >= 0 as c_int {
                        set_procsub_status(ind, pid, status);
                    }
                    if child.is_null() {
                        if status & 0x7f as c_int == 0 as c_int
                            || ((status & 0x7f as c_int) + 1 as c_int)
                                as libc::c_schar as c_int >> 1 as c_int
                                > 0 as c_int
                        {
                            js.c_reaped += 1;
                        }
                    } else {
                        (*child).status = status;
                        (*child)
                            .running = if status == 0xffff as c_int {
                            1 as c_int
                        } else {
                            0 as c_int
                        };
                        if (*child).running == 0 as c_int {
                            js.c_totreaped += 1;
                            if job != -(1 as c_int) {
                                js.c_reaped += 1;
                            }
                        }
                        if !(job == -(1 as c_int)) {
                            call_set_current += set_job_status_and_cleanup(job);
                            if (**jobs.offset(job as isize)).state as c_int
                                == JSTOPPED as c_int
                            {
                                last_stopped_job = job;
                            } else if (**jobs.offset(job as isize)).state as c_int
                                    == JDEAD as c_int && last_stopped_job == job
                                {
                                last_stopped_job = -(1 as c_int);
                            }
                        }
                    }
                }
            }
        }
        if !((sigchld != 0 || block == 0 as c_int) && pid > 0 as c_int) {
            break;
        }
    }
    if call_set_current != 0 {
        if last_stopped_job != -(1 as c_int) {
            set_current_job(last_stopped_job);
        } else {
            reset_current();
        }
    }
    if children_exited != 0
        && (signal_is_trapped(17 as c_int) != 0
            || *trap_list.as_mut_ptr().offset(17 as c_int as isize)
                == ::std::mem::transmute::<
                    *mut SigHandler,
                    *mut c_char,
                >(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> (),
                        *mut SigHandler,
                    >(initialize_traps as unsafe extern "C" fn() -> ()),
                ))
        && *trap_list.as_mut_ptr().offset(17 as c_int as isize)
            != ::std::mem::transmute::<
                __sighandler_t,
                *mut c_char,
            >(
                ::std::mem::transmute::<
                    libc::intptr_t,
                    __sighandler_t,
                >(1 as c_int as libc::intptr_t),
            )
    {
        if posixly_correct != 0 && this_shell_builtin.is_some()
            && this_shell_builtin
                == Some(
                    wait_builtin as unsafe extern "C" fn(*mut WordList) -> c_int,
                )
        {
            queue_sigchld_trap(children_exited);
            wait_signal_received = 17 as c_int;
            if sigchld == 0 as c_int && wait_intr_flag != 0 {
                siglongjmp(wait_intr_buf.as_mut_ptr(), 1 as c_int);
            }
        } else if sigchld != 0 {
            queue_sigchld_trap(children_exited);
        } else if signal_in_progress(17 as c_int) != 0 {
            queue_sigchld_trap(children_exited);
        } else if *trap_list.as_mut_ptr().offset(17 as c_int as isize)
                == ::std::mem::transmute::<
                    *mut SigHandler,
                    *mut c_char,
                >(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> (),
                        *mut SigHandler,
                    >(initialize_traps as unsafe extern "C" fn() -> ()),
                )
            {
            queue_sigchld_trap(children_exited);
        } else if running_trap != 0 {
            queue_sigchld_trap(children_exited);
        } else if this_shell_builtin
                == Some(
                    wait_builtin as unsafe extern "C" fn(*mut WordList) -> c_int,
                )
            {
            run_sigchld_trap(children_exited);
        } else {
            queue_sigchld_trap(children_exited);
        }
    }
    if asynchronous_notification != 0 && interactive != 0
        && executing_builtin == 0 as c_int
    {
        notify_of_job_status();
    }
    return children_exited;
}

unsafe extern "C" fn set_job_status_and_cleanup(mut job: c_int) -> c_int {
    let mut child: *mut PROCESS = 0 as *mut PROCESS;
    let mut tstatus: c_int = 0;
    let mut job_state: c_int = 0;
    let mut any_stopped: c_int = 0;
    let mut any_tstped: c_int = 0;
    let mut call_set_current: c_int = 0;
    let mut temp_handler: *mut SigHandler;

    child = (**jobs.offset(job as isize)).pipe;
    (**jobs.offset(job as isize)).flags &= !(J_NOTIFIED as c_int);

    call_set_current = 0;

    any_tstped = 0;
    any_stopped = any_tstped;
    job_state = any_stopped;
    loop {
        job_state |= PRUNNING! (child) as c_int;
        if PSTOPPED (child) != 0 {
            any_stopped = 1;
            any_tstped |= (job_control != 0 && WSTOPSIG!((*child).status) == SIGTSTP as c_int) as c_int;
        }
        child = (*child).next;
        if !(child != (**jobs.offset(job as isize)).pipe) {
            break;
        }
    }

    if job_state != 0 
        && JOBSTATE!(job) != JSTOPPED as c_int
    {
        return 0 ;
    }

    if any_stopped != 0 {
        (**jobs.offset(job as isize)).state = JSTOPPED;
        (**jobs.offset(job as isize)).flags &= !(J_FOREGROUND as c_int);
        call_set_current += 1;

        if any_tstped != 0 && loop_level != 0 {
            breaking = loop_level;
        }
    } else if job_state != 0  {
        (**jobs.offset(job as isize)).state = JRUNNING;
        call_set_current += 1;
    } else {
        (**jobs.offset(job as isize)).state = JDEAD;
        js.j_ndead += 1;

        if ((**jobs.offset(job as isize)).j_cleanup).is_some() {
            (Some(
                ((**jobs.offset(job as isize)).j_cleanup)
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )((**jobs.offset(job as isize)).cleanarg);
            
            (**jobs.offset(job as isize)).j_cleanup = ::std::mem::transmute::<*mut libc::c_void, sh_vptrfunc_t>(0 as *mut libc::c_void);
        }
    }
    if JOBSTATE!(job) == JDEAD as c_int {
        if wait_sigint_received != 0 && interactive_shell == 0  
            && child_caught_sigint != 0
            && IS_FOREGROUND!(job) && signal_is_trapped(SIGINT as c_int) != 0
        {
            let mut old_frozen: c_int = 0;
            wait_sigint_received = 0 ;
            last_command_exit_value = process_exit_status ((*child).status);

            old_frozen = jobs_list_frozen;
            jobs_list_frozen = 1 ;
            tstatus = maybe_call_trap_handler(SIGINT as c_int);
            jobs_list_frozen = old_frozen;
        } else if wait_sigint_received != 0 && child_caught_sigint == 0  
                && IS_FOREGROUND!(job)
                && IS_JOBCONTROL!(job) as c_int == 0  
            {
            let mut old_frozen_0: c_int = 0;

            wait_sigint_received = 0 ;

            if signal_is_trapped(SIGINT as c_int) != 0 {
                last_command_exit_value = process_exit_status ((*child).status);
            }
            old_frozen_0 = jobs_list_frozen;
            jobs_list_frozen = 1  ;
            tstatus = maybe_call_trap_handler(SIGINT as c_int);
            jobs_list_frozen = old_frozen_0;
            if tstatus == 0 
                && old_sigint_handler  != INVALID_SIGNAL_HANDLER!()
            {
                temp_handler = old_sigint_handler;
                if temp_handler == trap_handler as *mut Option<unsafe extern "C" fn(i32)>
                    && signal_is_trapped(SIGINT as c_int) == 0 
                {
                    temp_handler = &mut trap_to_sighandler(SIGINT as c_int);
                }
                println!("161616161616");
                restore_sigint_handler();
                if !temp_handler.is_null() {
                    termsig_handler(SIGINT as c_int);
                } else if temp_handler 
                        != SIG_IGN!() 
                    {
                        //这里是函数回调传入参数
                    (*temp_handler).unwrap()(SIGINT as c_int);
                }
            }
        }
    }
    return call_set_current;
}


unsafe extern "C" fn setjstatus(mut j: c_int) {
    let mut i: c_int = 0;
    let mut p: *mut PROCESS = 0 as *mut PROCESS;

    i = 1 ;
    p = (**jobs.offset(j as isize)).pipe;
    while (*p).next != (**jobs.offset(j as isize)).pipe {
        p = (*p).next;
        i += 1;
    }
    i += 1;
    if statsize < i {
        pstatuses = libc::realloc(pstatuses as *mut c_void, (i * 4) as usize) as *mut c_int;    //i * sizeof (int)
        statsize = i;
    }
    i = 0 as c_int;
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


#[macro_export]
macro_rules! savestring {
    ($x:expr) => {
        libc::strcpy(libc::malloc(1+ libc::strlen($x)) as *mut c_char , ($x))
    };
}


pub const SEVAL_NOHIST:c_int = 0x004;
pub const SEVAL_RESETLINE:c_int = 0x010;


#[no_mangle]
pub unsafe extern "C"  fn  run_sigchld_trap(mut nchild: c_int) {
    let mut trap_command: *mut c_char ;
    let mut i: c_int = 0;
    trap_command = savestring!(*trap_list.as_mut_ptr().offset(SIGCHLD as c_int as isize));
    
    begin_unwind_frame(
        b"SIGCHLD trap\0" as *const u8 as *mut c_char,
    );

    unwind_protect_mem(
        &mut last_command_exit_value as *mut c_int as *mut c_char,
        ::std::mem::size_of::<c_int>() as libc::c_ulong as c_int,
    );
    unwind_protect_mem(
        &mut last_command_exit_signal as *mut c_int as *mut c_char,
        ::std::mem::size_of::<c_int>() as libc::c_ulong as c_int,
    );
    unwind_protect_mem(
        &mut last_made_pid as *mut pid_t as *mut c_char,
        ::std::mem::size_of::<pid_t>() as libc::c_ulong as c_int,
    );
    unwind_protect_mem(
        &mut jobs_list_frozen as *mut c_int as *mut c_char,
        ::std::mem::size_of::<c_int>() as libc::c_ulong as c_int,
    );
    unwind_protect_mem(
        &mut the_pipeline as *mut *mut PROCESS as *mut c_char,
        ::std::mem::size_of::<*mut PROCESS>() as libc::c_ulong as c_int,
    );
    unwind_protect_mem(
        &mut subst_assign_varlist as *mut *mut WordList as *mut c_char,
        ::std::mem::size_of::<*mut WordList>() as libc::c_ulong as c_int,
    );
    unsafe{
    unwind_protect_mem(
        &mut this_shell_builtin as *const sh_builtin_func_t  as *mut c_char,
        ::std::mem::size_of::<Option::<sh_builtin_func_t>>() as libc::c_ulong
            as c_int,
    );
    }
    unwind_protect_mem(
        &mut temporary_env as *mut *mut HASH_TABLE as *mut c_char,
        ::std::mem::size_of::<*mut HASH_TABLE>() as libc::c_ulong as c_int,
    );

    extern "C" {
        pub fn xfree(arg1: *mut ::std::os::raw::c_void);
    }
    let t1=xfree;
    unsafe{
    let t = xfree as *const  libc::c_void;
    let t2=t1 as *const unsafe extern "C" fn() ->i32;
    let t3 = t2 as *mut unsafe extern "C" fn() -> ::std::os::raw::c_int;
    



    let mut t4=&Some(t2);
    let mut t5=&Some(t3);
    let mut  t6=t5;

    add_unwind_protect(
        t5.unwrap() as *const ::std::option::Option<*mut unsafe  extern "C" fn()->i32> 
        as *mut ::std::option::Option<unsafe  extern "C" fn()->i32>, 
        trap_command,
    );


    let r1=maybe_set_sigchld_trap as *const fn(_: *mut c_char);
    let r2=&Some(r1);
    add_unwind_protect(
        Some(maybe_set_sigchld_trap as *mut unsafe extern "C" fn() -> std::os::raw::c_int).unwrap() as *const ::std::option::Option<*mut unsafe  extern "C" fn()->i32> 
        as *mut ::std::option::Option<unsafe  extern "C" fn()->i32>,
        trap_command,
    );
    
    subst_assign_varlist = 0 as *mut WordList;
    the_pipeline = 0 as *mut PROCESS;
    temporary_env = 0 as *mut HASH_TABLE;

    running_trap = SIGCHLD as c_int + 1 ;

    set_impossible_sigchld_trap();
    jobs_list_frozen = 1 ;
    i = 0 ;
    while i < nchild {
        parse_and_execute(
            savestring! (trap_command),
            b"trap\0" as *const u8 as *const c_char,
            SEVAL_NOHIST | SEVAL_RESETLINE 
        );
        i += 1;
    }
    run_unwind_frame(
        b"SIGCHLD trap\0" as *const u8 as *const c_char as *mut c_char,
    );
    running_trap = 0 as c_int;
}
}

unsafe extern "C" fn notify_of_job_status() {
    let mut job: c_int = 0;
    let mut termsig: c_int = 0;
    let mut dir: *mut c_char = 0 as *mut c_char;
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };
    let mut s: WAIT = 0;

    if jobs.is_null() || js.j_jobslots == 0 {
        return;
    }
    if !old_ttou.is_null(){
        sigemptyset(&mut set);
        sigaddset(&mut set, SIGCHLD as c_int);
        sigaddset(&mut set, SIGTTOU as c_int);
        sigemptyset(&mut oset);
        sigprocmask(SIG_BLOCK as c_int, &mut set, &mut oset);
    } else {
        queue_sigchld += 1;
    }
    job = 0 ;
    dir = 0 as *mut c_char;
    while job < js.j_jobslots {
        if !(*jobs.offset(job as isize)).is_null()
            && IS_NOTIFIED!(job)as c_int == 0 
        {
            s = raw_job_exit_status(job);
            termsig = WTERMSIG! (s);

            if !(startup_state == 0 && WIFSIGNALED!(s) as c_int == 0 
                && ((DEADJOB!(job) && IS_FOREGROUND!(job) as c_int == 0 || STOPPED!(job))))
            {
                if job_control == 0 && interactive_shell != 0
                    || startup_state == 2  && subshell_environment & SUBSHELL_COMSUB as c_int != 0
                    || startup_state == 2  && posixly_correct != 0
                        && subshell_environment & SUBSHELL_COMSUB as c_int == 0 as c_int
                {
                    if (DEADJOB! (job) && (interactive_shell != 0
                            || find_last_pid(job, 0)!= last_asynchronous_pid))
                    {
                        (**jobs.offset(job as isize)).flags |= J_NOTIFIED as c_int;
                    }
                } else {
                    match (**jobs.offset(job as isize)).state as c_int {
                        JDEAD => {
                            if interactive_shell == 0 && termsig != 0
                                && WIFSIGNALED! (s)
                                && termsig != SIGINT as c_int
                                && termsig != SIGTERM as c_int
                                && signal_is_trapped(termsig) == 0  
                            {
                                fprintf(
                                    stderr,
                                    b"%s: line %d: \0" as *const u8 as *const c_char,  
                                    get_name_for_error(),
                                    if line_number == 0  {
                                        1 
                                    } else {
                                        line_number
                                    },
                                );
                                pretty_print_job(job, JLIST_NONINTERACTIVE as c_int, stderr);
                            } else if IS_FOREGROUND!(job)
                                {
                                if termsig != 0
                                    &&  WIFSIGNALED! (s)
                                    && termsig != SIGINT as c_int
                                    && termsig != SIGPIPE as c_int
                                {
                                    fprintf(
                                        stderr,
                                        b"%s\0" as *const u8 as *const c_char,
                                        j_strsignal(termsig),
                                    );
                                    if WIFCORED! (s) {
                                        fprintf(
                                            stderr,
                                            b" (core dumped)\0" as *const u8 as *const c_char);
                                    }
                                    fprintf(
                                        stderr,
                                        b"\n\0" as *const u8 as *const c_char,
                                    );
                                }
                            } else if job_control != 0 {
                                if dir.is_null() {
                                    dir = current_working_directory();
                                }
                                pretty_print_job(job, JLIST_STANDARD as c_int, stderr);
                                if !dir.is_null()
                                    && strcmp(dir, (**jobs.offset(job as isize)).wd)
                                        != 0 
                                {
                                    fprintf(
                                        stderr,
                                        b"(wd now: %s)\n\0" as *const u8 as *const c_char,   
                                        polite_directory_format(dir)
                                    );
                                }
                            }
                            (**jobs.offset(job as isize)).flags |= J_NOTIFIED as c_int;
                        }
                        JSTOPPED => {
                            fprintf(stderr, b"\n\0" as *const u8 as *const c_char);
                            if dir.is_null() {
                                dir = current_working_directory();
                            }
                            pretty_print_job(job, JLIST_STANDARD as c_int, stderr);
                            if !dir.is_null()
                                && strcmp(dir, (**jobs.offset(job as isize)).wd)
                                    != 0 
                            {
                                fprintf(
                                    stderr,
                                    b"(wd now: %s)\n\0" as *const u8 as *const c_char,
                                    polite_directory_format(dir),
                                );
                            }
                            (**jobs.offset(job as isize)).flags |= J_NOTIFIED as c_int;
                        }
                        JRUNNING | JMIXED => {}
                        _ => {
                            programming_error(
                                b"notify_of_job_status\0" as *const u8
                                    as *const c_char, 
                            );
                        }
                    }
                }
            }
        }
        job += 1;
    }
    if !old_ttou.is_null()   {
        sigprocmask(
            SIG_SETMASK as c_int,
            &mut oset,
            0 as *mut libc::c_void as *mut sigset_t,
        );
    } else {
        queue_sigchld -= 1;
    };
}

pub const O_RDWR:i32 = 0o2; 
pub const O_NONBLOCK:i32 = 0o4000;
pub const F_SETFD:i32 = 2; 
pub const FD_CLOEXEC:i32 = 1;  

#[macro_export]
macro_rules! SET_CLOSE_ON_EXEC {
    ($fd:expr) => {
        libc::fcntl (($fd), F_SETFD, FD_CLOEXEC)
    };
}


#[no_mangle]
pub unsafe extern "C" fn initialize_job_control(mut force: c_int) -> c_int {
    let mut current_block: u64;
    let mut t: pid_t = 0;
    let mut t_errno: c_int = 0;
    let mut tty_sigs: c_int = 0;

    t_errno = -(1 as c_int);
    shell_pgrp = getpgid(0);

    if shell_pgrp == -(1 as c_int) {
        sys_error(
            dcgettext(
                0 as *const c_char,
                b"initialize_job_control: getpgrp failed\0" as *const u8
                    as *const c_char,
                5 as c_int,
            ),
        );
        exit(1 as c_int);
    }

    if interactive == 0 as c_int && force == 0 as c_int {
        job_control = 0 as c_int;
        original_pgrp = -(1 as c_int);
        shell_tty = fileno(stderr);
        terminal_pgrp = tcgetpgrp(shell_tty);
    } else {
        shell_tty = -(1 as c_int);
        if forced_interactive != 0 && isatty(fileno(stderr)) == 0 as c_int {
            shell_tty = open(
                b"/dev/tty\0" as *const u8 as *const c_char,
                0o2 as c_int | 0o4000 as c_int,
            );
        }
        if shell_tty == -(1 as c_int) {
            shell_tty = dup(fileno(stderr));
        }
        if shell_tty != -(1 as c_int) {
            shell_tty = move_to_high_fd(
                shell_tty,
                1 as c_int,
                -(1 as c_int),
            );
        }
        if shell_pgrp == 0 as c_int {
            shell_pgrp = getpid();
            setpgid(0 as c_int, shell_pgrp);
            if shell_tty != -(1 as c_int) {
                tcsetpgrp(shell_tty, shell_pgrp);
            }
        }
        tty_sigs = 0 as c_int;
        loop {
            terminal_pgrp = tcgetpgrp(shell_tty);
            if !(terminal_pgrp != -(1 as c_int)) {
                current_block = 5529461102203738653;
                break;
            }
            if !(shell_pgrp != terminal_pgrp) {
                current_block = 5529461102203738653;
                break;
            }
            let mut ottin: *mut SigHandler ;
            if terminating_signal != 0 {
                termsig_handler(terminating_signal);
            }
            ottin = set_signal_handler(21 as c_int, &mut None);
            kill(0 as c_int, 21 as c_int);
            set_signal_handler(21 as c_int, ottin);
            let fresh35 = tty_sigs;
            tty_sigs = tty_sigs + 1;
            if !(fresh35 > 16 as c_int) {
                continue;
            }
            sys_error(
                dcgettext(
                    0 as *const c_char,
                    b"initialize_job_control: no job control in background\0"
                        as *const u8 as *const c_char,
                    5 as c_int,
                ),
            );
            job_control = 0 as c_int;
            original_pgrp = terminal_pgrp;
            current_block = 16073591882049499585;
            break;
        }
        match current_block {
            16073591882049499585 => {}
            _ => {
                if terminal_pgrp == -(1 as c_int) {
                    t_errno = *__errno_location();
                }
                if set_new_line_discipline(shell_tty) < 0 as c_int {
                    sys_error(
                        dcgettext(
                            0 as *const c_char,
                            b"initialize_job_control: line discipline\0" as *const u8
                                as *const c_char,
                            5 as c_int,
                        ),
                    );
                    job_control = 0 as c_int;
                } else {
                    original_pgrp = shell_pgrp;
                    shell_pgrp = getpid();
                    if original_pgrp != shell_pgrp
                        && setpgid(0 as c_int, shell_pgrp) < 0 as c_int
                    {
                        sys_error(
                            dcgettext(
                                0 as *const c_char,
                                b"initialize_job_control: setpgid\0" as *const u8
                                    as *const c_char,
                                5 as c_int,
                            ),
                        );
                        shell_pgrp = original_pgrp;
                    }
                    job_control = 1 as c_int;
                    if shell_pgrp != original_pgrp && shell_pgrp != terminal_pgrp {
                        if give_terminal_to(shell_pgrp, 0 as c_int)
                            < 0 as c_int
                        {
                            t_errno = *__errno_location();
                            setpgid(0 as c_int, original_pgrp);
                            shell_pgrp = original_pgrp;
                            *__errno_location() = t_errno;
                            sys_error(
                                dcgettext(
                                    0 as *const c_char,
                                    b"cannot set terminal process group (%d)\0" as *const u8
                                        as *const c_char,
                                    5 as c_int,
                                ),
                                shell_pgrp,
                            );
                            job_control = 0 as c_int;
                        }
                    }
                    if job_control != 0
                        && {
                            t = tcgetpgrp(shell_tty);
                            t == -(1 as c_int) || t != shell_pgrp
                        }
                    {
                        if t_errno != -(1 as c_int) {
                            *__errno_location() = t_errno;
                        }
                        sys_error(
                            dcgettext(
                                0 as *const c_char,
                                b"cannot set terminal process group (%d)\0" as *const u8
                                    as *const c_char,
                                5 as c_int,
                            ),
                            t,
                        );
                        job_control = 0 as c_int;
                    }
                }
                if job_control == 0 as c_int {
                    internal_error(
                        dcgettext(
                            0 as *const c_char,
                            b"no job control in this shell\0" as *const u8
                                as *const c_char,
                            5 as c_int,
                        ),
                    );
                }
            }
        }
    }
    running_in_background = (terminal_pgrp != shell_pgrp) as c_int;
    if shell_tty != fileno(stderr) {
        fcntl(shell_tty, 2 as c_int, 1 as c_int);
    }
    set_signal_handler(
        17 as c_int,
        sigchld_handler as *mut Option<unsafe extern "C" fn(i32)>
    );
    change_flag('m' as i32, if job_control != 0 { '-' as i32 } else { '+' as i32 });
    if interactive != 0 {
        get_tty_state();
    }
    set_maxchild(0 as c_int);
    return job_control;
}

unsafe extern "C" fn set_new_line_discipline(mut tty: c_int) -> c_int {
    return 0 as c_int;
}

#[no_mangle]
pub unsafe extern "C"  fn  initialize_job_signals() {
    if interactive != 0 {
        set_signal_handler(
                    2 as c_int,
                    std::mem::transmute::<
                    std::option::Option<unsafe extern "C" fn(i32)>,
                    *mut std::option::Option<unsafe extern "C" fn(i32)>
                    >(Some(sigint_sighandler as unsafe extern "C" fn(c_int) -> ()),),
                );
        set_signal_handler (SIGTSTP as c_int, SIG_IGN!());
        set_signal_handler (SIGTTOU as c_int, SIG_IGN!());
        set_signal_handler (SIGTTIN as c_int, SIG_IGN!());
    } else if job_control != 0 {
        old_tstp = set_signal_handler(
            SIGTSTP as c_int,
            sigstop_sighandler as *mut Option<unsafe extern "C" fn(i32)>,
        );
        old_ttin = set_signal_handler(
            SIGTTIN as c_int,
            sigstop_sighandler as *mut Option<unsafe extern "C" fn(i32)>,
        );
        old_ttou = set_signal_handler(
            SIGTTOU as c_int,
            sigstop_sighandler as *mut Option<unsafe extern "C" fn(i32)>,
        );
    }
}

unsafe extern "C" fn sigcont_sighandler(mut sig: c_int) {
    initialize_job_signals();
    set_signal_handler(SIGCONT as c_int, old_cont);
    kill(getpid(), SIGCONT as c_int);
}
unsafe extern "C" fn sigstop_sighandler(mut sig: c_int) {
    set_signal_handler(SIGTSTP as c_int, old_tstp);
    set_signal_handler(SIGTTOU as c_int, old_ttou);
    set_signal_handler(SIGTTIN as c_int, old_ttin);
    old_cont = set_signal_handler(
        SIGCONT as c_int,
        sigcont_sighandler as *mut Option<unsafe extern "C" fn(i32)>
    );
    give_terminal_to(shell_pgrp, 0 );
    kill(getpid(), sig);
}


#[no_mangle]
pub unsafe extern "C"  fn  give_terminal_to(mut pgrp: pid_t,mut force: c_int) -> c_int 
{
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };
    let mut r: c_int = 0;
    let mut e: c_int = 0;

    if job_control != 0 || force != 0 {
        sigemptyset(&mut set);
        sigaddset(&mut set, SIGTTOU as c_int);
        sigaddset(&mut set, SIGTTIN as c_int);
        sigaddset(&mut set, SIGTSTP as c_int);
        sigaddset(&mut set, SIGCHLD as c_int);
        sigemptyset(&mut oset);
        sigprocmask(SIG_BLOCK as c_int, &mut set, &mut oset);

        if tcsetpgrp(shell_tty, pgrp) < 0 {
            r = -1;
            e = errno!();
        } else {
            terminal_pgrp = pgrp;
        }
        sigprocmask(
            SIG_SETMASK as c_int,
            &mut oset,
            0 as *mut sigset_t,
        );
    }
    if r == -1 {
        errno!() = e;
    }
    return r;
}

pub const ENOTTY:i32 = 25;

unsafe extern "C" fn maybe_give_terminal_to(mut opgrp: pid_t, mut npgrp: pid_t, mut flags: c_int) -> c_int 
{
    let mut tpgrp: c_int = 0;

    tpgrp = tcgetpgrp(shell_tty);
    if tpgrp < 0 && errno!() == ENOTTY {
        return -1;
    }
    if tpgrp == npgrp {
        terminal_pgrp = npgrp;
        return 0 ;
    } else if tpgrp != opgrp {
        return -1
    } else {
        return give_terminal_to(npgrp, flags)
    };
}


#[no_mangle]
pub unsafe extern "C"  fn  delete_all_jobs(mut running_only: c_int) {
    let mut i: c_int = 0;
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

    BLOCK_CHILD (&mut set, &mut oset);
    if js.j_jobslots != 0 {
        js.j_previous = NO_JOB;
        js.j_current = js.j_previous;
        i = 0  ;
        while i < js.j_jobslots {
            if !(*jobs.offset(i as isize)).is_null()
                && (running_only == 0  
                    || running_only != 0
                        && RUNNING!(i))
            {
                delete_job(i, DEL_WARNSTOPPED as c_int | DEL_NOBGPID as c_int);
            }
            i += 1;
        }
        if running_only == 0 {
            libc::free(jobs as *mut c_void);
            
            js.j_jobslots = 0 ;
            js.j_njobs = 0  ;
            js.j_lastj = js.j_njobs;
            js.j_firstj = js.j_lastj;
        }
    }
    if running_only == 0 {
        bgp_clear();
    }
    UNBLOCK_CHILD (&mut oset);
}

#[no_mangle]
pub unsafe extern "C"  fn  nohup_all_jobs(mut running_only: c_int) {
    let mut i: c_int = 0;
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

    BLOCK_CHILD (&mut set, &mut oset);
    if js.j_jobslots != 0 {
        i = 0 ;
        while i < js.j_jobslots {
            if !(*jobs.offset(i as isize)).is_null()
                && (running_only == 0  || running_only != 0 && RUNNING!(i))
            {
                nohup_job(i);
            }
            i += 1;
        }
    }
    UNBLOCK_CHILD (&mut oset);
}

#[no_mangle]
pub unsafe extern "C"  fn  count_all_jobs() -> c_int {
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

    BLOCK_CHILD (&mut set, &mut oset);
    n = 0 ;
    i = n;
    while i < js.j_jobslots {
        if !(*jobs.offset(i as isize)).is_null()
            && DEADJOB!(i)as c_int == 0 as c_int
        {
            n += 1;
        }
        i += 1;
    }
    UNBLOCK_CHILD (&mut oset);
    return n;
}

unsafe extern "C" fn mark_all_jobs_as_dead() {
    let mut i: c_int = 0;
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

    if js.j_jobslots == 0  {
        return;
    }
    BLOCK_CHILD (&mut set, &mut oset);
    i = 0 ;
    while i < js.j_jobslots {
        if !(*jobs.offset(i as isize)).is_null() {
            (**jobs.offset(i as isize)).state = JDEAD;
            js.j_ndead += 1;
        }
        i += 1;
    }
    UNBLOCK_CHILD (&mut oset);
}


unsafe extern "C" fn mark_dead_jobs_as_notified(mut force: c_int) {
    let mut i: c_int = 0;
    let mut ndead: c_int = 0;
    let mut ndeadproc: c_int = 0;
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = __sigset_t { __val: [0; 16] };

    if js.j_jobslots == 0 {
        return;
    }
    BLOCK_CHILD (&mut set, &mut oset);
    if force != 0 {
        i = 0 ;
        while i < js.j_jobslots {
            if !(*jobs.offset(i as isize)).is_null()
                && DEADJOB!(i)
                && (interactive_shell != 0
                    || find_last_pid(i, 0 as c_int) != last_asynchronous_pid)
            {
                (**jobs.offset(i as isize)).flags |= J_NOTIFIED as c_int;
            }
            i += 1;
        }
        UNBLOCK_CHILD (&mut oset);
        return;
    }
    ndeadproc = 0 ;
    ndead = ndeadproc;
    i = ndead;
    while i < js.j_jobslots {
        if !(*jobs.offset(i as isize)).is_null()
            && DEADJOB!(i)
        {
            ndead += 1;
            ndeadproc += processes_in_job(i);
        }
        i += 1;
    }
    if js.c_childmax < 0 {
        set_maxchild(0 );
    }
    if ndeadproc as libc::c_long <= js.c_childmax {
        UNBLOCK_CHILD (&mut oset);
        return;
    }
    i = 0 ;
    while i < js.j_jobslots {
        if !(*jobs.offset(i as isize)).is_null()
            && DEADJOB!(i)
            && (interactive_shell != 0
                || find_last_pid(i, 0 as c_int) != last_asynchronous_pid)
        {
            ndeadproc -= processes_in_job(i);
            if ndeadproc as libc::c_long <= js.c_childmax {
                break;
            }
            (**jobs.offset(i as isize)).flags |= J_NOTIFIED as c_int;
        }
        i += 1;
    }
    UNBLOCK_CHILD (&mut oset);
}

#[no_mangle]
pub unsafe extern "C"  fn  freeze_jobs_list() -> c_int {
    let mut o: c_int = 0;

    o = jobs_list_frozen;
    jobs_list_frozen = 1;
    return o;
}

#[no_mangle]
pub unsafe extern "C"  fn  unfreeze_jobs_list() {
    jobs_list_frozen = 0;
}

#[no_mangle]
pub unsafe extern "C"  fn  set_jobs_list_frozen(mut s: c_int) {
    jobs_list_frozen = s;
}

#[no_mangle]
pub unsafe extern "C"  fn  set_job_control(mut arg: c_int) -> c_int {
    let mut old: c_int = 0;

    old = job_control;
    job_control = arg;

    if terminal_pgrp ==NO_PID as c_int {
        terminal_pgrp = tcgetpgrp(shell_tty);
    }
    if job_control != old && job_control != 0 {
        shell_pgrp = getpgid(0);
    }
    running_in_background = (terminal_pgrp != shell_pgrp) as c_int;

    if job_control != old && job_control != 0 {
        pipeline_pgrp = 0;
    }
    return old;
}

#[no_mangle]
pub unsafe extern "C"  fn  without_job_control() {
    stop_making_children();
    start_pipeline();
    sh_closepipe(pgrp_pipe.as_mut_ptr());
    delete_all_jobs(0 );
    set_job_control(0 );
}
#[no_mangle]
pub unsafe extern "C"  fn  end_job_control() {
    if job_control != 0 {
        terminate_stopped_jobs();
    }
    if original_pgrp >= 0 as c_int && terminal_pgrp != original_pgrp {
        give_terminal_to(original_pgrp, 1 );
    }
    if original_pgrp >= 0  
        && setpgid(0 , original_pgrp) == 0  
    {
        shell_pgrp = original_pgrp;
    }
}
#[no_mangle]
pub unsafe extern "C"  fn  restart_job_control() {
    if shell_tty != -1 {
        libc::close(shell_tty);
    }
    initialize_job_control(0 );
}

#[no_mangle]
pub unsafe extern "C"  fn  set_maxchild(mut nchild: c_int) {
    static mut lmaxchild: c_int =-1;

    if lmaxchild < 0{
        errno!() = 0 ;
        lmaxchild = getmaxchild() as c_int;
        if lmaxchild < 0 && *__errno_location() == 0  {
            lmaxchild = MAX_CHILD_MAX as c_int;
        }
    }
    if lmaxchild < 0 {
        lmaxchild = DEFAULT_CHILD_MAX as c_int;
    }
    if nchild < lmaxchild {
        nchild = lmaxchild;
    } else if nchild > MAX_CHILD_MAX as c_int {
        nchild = MAX_CHILD_MAX as c_int;
    }
    js.c_childmax = nchild as libc::c_long;
}

#[no_mangle]
pub unsafe extern "C"  fn  set_sigchld_handler() {
    set_signal_handler(
        SIGCHLD as c_int,sigchld_handler as *mut Option<unsafe extern "C" fn(i32)>);
}


unsafe extern "C" fn pipe_read(mut pp: *mut c_int) {
    let mut ch: c_char = 0;

    if *pp.offset(1 as isize) >= 0 {
        libc::close(*pp.offset(1 as c_int as isize));
        *pp.offset(1 as isize) = -(1 as c_int);
    }
    if *pp.offset(0 as c_int as isize) >= 0 {
        while libc::read(
            *pp.offset(0 as c_int as isize),
            &mut ch as *mut c_char as *mut libc::c_void,
            1 as c_int as size_t,
        ) == -1
            && errno!() == EINTR as c_int
        {}
    }
}



#[no_mangle]
pub unsafe extern "C"  fn  close_pgrp_pipe() {
    sh_closepipe(pgrp_pipe.as_mut_ptr());
}


#[no_mangle]
pub unsafe extern "C"  fn  save_pgrp_pipe(mut p: *mut c_int, mut clear: c_int,) 
{
    *p.offset(0 as c_int as isize) = pgrp_pipe[0 as c_int as usize];
    *p.offset(1 as c_int as isize) = pgrp_pipe[1 as c_int as usize];
    if clear != 0 {
        pgrp_pipe[1] = -1;
        pgrp_pipe[0] = pgrp_pipe[1];
    }
}
#[no_mangle]
pub unsafe extern "C"  fn  restore_pgrp_pipe(mut p: *mut c_int) {
    pgrp_pipe[0 ] = *p.offset(0 );
    pgrp_pipe[1 ] = *p.offset(1 as c_int as isize);
}

