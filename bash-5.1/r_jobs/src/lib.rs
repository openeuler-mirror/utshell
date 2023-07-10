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




    stop_making_children ();
    UNBLOCK_CHILD (&mut oset);
    return if !newjob.is_null() { i } else { js.j_current };
}
