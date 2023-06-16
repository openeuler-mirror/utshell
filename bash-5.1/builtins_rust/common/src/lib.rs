
extern crate libc;
use libc::{c_char,c_int, c_void, FILE, size_t, intmax_t,c_long, strcmp};
use libc::{isdigit,strerror, __errno_location, fflush, ferror,clearerr, free,strcpy,strlen,strncmp,atoi,qsort};
use std::ffi::{CStr, CString};
use std::mem::size_of;
use std::ptr::read_volatile;
use nix::errno::errno;
use std::env::var;
use unic_langid::LanguageIdentifier;
include!(concat!("lib_readline_keymaps.rs"));
include!(concat!("command.rs"));
use fluent_bundle::{FluentBundle, FluentResource, FluentValue, FluentArgs};
use fluent_resmgr::resource_manager::ResourceManager;

//struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct word_desc {
    pub word: *mut c_char,
    pub flags: c_int,
}
pub type WordDesc = word_desc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct word_list {
    pub next: *mut word_list,
    pub word: *mut WordDesc,
}
pub type WordList = word_list;
#[repr (C)]
pub struct builtin{
    pub name:*mut c_char,
    pub function:*mut sh_builtin_func_t,
    pub flags:i32,
    pub long_doc: *const *mut c_char ,
    pub short_doc:*const c_char,
    pub handle:*mut c_char,
}
#[repr (C)]
pub struct g_list{
    pub next:*mut g_list,
}
type GENERIC_LIST = g_list;
#[repr (C)]
pub struct process{
    pub next:*mut process,
    pub pid:pid_t,
    pub status:WAIT,
    pub running:i32,
    pub command:*mut c_char,
}
type WAIT = i32;
type pid_t = c_int;
type PROCESS = process;
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
pub struct COMMAND {
    type_c:command_type,
    flags:i32,
    line:i32,
    redirects:*mut REDIRECT,
    value:VALUE_COMMAND,
}
#[repr(C)]
#[derive(Copy,Clone)]
pub union REDIRECTEE {
    dest:libc::c_int,           /* Place to redirect REDIRECTOR to, or ... */
    filename:* mut WordDesc        /* filename to redirect to. */
}
#[repr(u8)]
#[derive(Copy,Clone)]
enum r_instruction {
    r_output_direction, r_input_direction, r_inputa_direction,
    r_appending_to, r_reading_until, r_reading_string,
    r_duplicating_input, r_duplicating_output, r_deblank_reading_until,
    r_close_this, r_err_and_out, r_input_output, r_output_force,
    r_duplicating_input_word, r_duplicating_output_word,
    r_move_input, r_move_output, r_move_input_word, r_move_output_word,
    r_append_err_and_out
}
#[repr(C)]
pub union REDIRECT {
  next:*mut REDIRECT,   /* Next element, or NULL. */
  redirector:REDIRECTEE,    /* Descriptor or varname to be redirected. */
  rflags:libc::c_int,           /* Private flags for this redirection */
  flags:libc::c_int,            /* Flag value for `open'. */
  instruction:r_instruction, /* What to do with the information. */
  redirectee:REDIRECTEE,    /* File descriptor or filename */
  here_doc_eof:*mut c_char      /* The word that appeared in <<foo. */
}
/* FOR command. */
#[repr(C)]
pub struct for_com {
    flags:libc::c_int,
    line:libc::c_int,
    name:*mut WordDesc,
    map_list:*mut WordList,
    action:*mut COMMAND
}
#[repr(C)]
pub struct case_com {
    flags:libc::c_int,
    line:libc::c_int,
    word:*mut WordDesc,
    clauses:*mut PATTERN_LIST
}
#[repr(C)]
pub struct PATTERN_LIST {
    next:* mut PATTERN_LIST,
    patterns:* mut WordList,
    action:*mut COMMAND,
    flags:libc::c_int
}
#[repr(C)]
pub struct while_com {
    flags:libc::c_int,
    test:*mut COMMAND,
    action:*mut COMMAND
}
#[repr(C)]
pub struct if_com {
    flags:libc::c_int,
    test:*mut COMMAND,
    true_case:*mut COMMAND,
    false_case:*mut COMMAND
}
#[repr(C)]
pub struct connection {
    ignore:libc::c_int,
    first:*mut COMMAND,
    second:*mut COMMAND,
    connector:libc::c_int
}
#[repr(C)]
pub struct simple_com {
    flags:libc::c_int,
    line:libc::c_int,
    words:*mut WordList,
    redirects:*mut REDIRECT
}
#[repr(C)]
pub struct function_def {
    flags:libc::c_int,
    line:libc::c_int,
    name:*mut WordDesc,
    command:*mut COMMAND,
    source_file:*mut c_char
}
#[repr(C)]
pub struct group_com {
    ignore:libc::c_int,
    command:*mut COMMAND,
    source_file:*mut c_char
}
#[repr(C)]
pub struct select_com {
    flags:libc::c_int,
    line:libc::c_int,
    name:*mut WordDesc,
    map_list:*mut WordList,
    action:*mut COMMAND
}
#[repr(C)]
pub struct arith_com {
    flags:libc::c_int,
    line:libc::c_int,
    exp:*mut WordList
}
#[repr(C)]
pub struct cond_com {
    flags:libc::c_int,
    line:libc::c_int,
    type_c:libc::c_int,
    exp:*mut WordList
}
#[repr(C)]
pub struct arith_for_com {
    flags:libc::c_int,
    line:libc::c_int,
    init:*mut WordList,
    test:*mut WordList,
    step:*mut WordList,
    action:*mut COMMAND
}
#[repr(C)]
pub struct subshell_com {
    flags:i32,
    line:i32,
    command:*mut COMMAND
}
#[repr(C)]
pub struct coproc_com {
    flags:i32,
    name:*mut c_char,
    command:*mut COMMAND
}
#[repr(C)]
pub union VALUE_COMMAND {
    For:*mut for_com,
    Case:*mut case_com,
    While:*mut while_com,
    If:*mut if_com,
    Connection:*mut connection,
    Simple:*mut simple_com,
    Function_def:*mut function_def,
    Group:*mut group_com,
    Select:*mut select_com,
    Arith:*mut arith_com,
    Cond:*mut cond_com,
    ArithFor:*mut arith_for_com,
    Subshell:*mut subshell_com,
    Coproc:*mut coproc_com
}
#[repr(u8)]
enum command_type { cm_for, cm_case, cm_while, cm_if, cm_simple, cm_select,
    cm_connection, cm_function_def, cm_until, cm_group,
    cm_arith, cm_cond, cm_arith_for, cm_subshell, cm_coproc
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
#[repr(C)]
pub struct SHELL_VAR {
  pub name:*mut c_char,         /* Symbol that the user types. */
  pub value:*mut c_char,            /* Value that is returned. */
  pub exportstr:*mut c_char,    /* String for the environment. */
  pub dynamic_value:*mut fn(v:* mut SHELL_VAR)->*mut SHELL_VAR, /* Function called to return a `dynamic'
                   value for a variable, like $SECONDS
                   or $RANDOM. */
  pub assign_func:* mut fn(v:* mut SHELL_VAR,str1:* mut c_char,t:c_long,str2:* mut c_char)->*mut SHELL_VAR, /* Function called when this `special
                   variable' is assigned a value in
                   bind_variable. */
  pub attributes:i32,       /* export, readonly, array, invisible... */
  pub context:i32           /* Which context this variable belongs to. */
}
