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











