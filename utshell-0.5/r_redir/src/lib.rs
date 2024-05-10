//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// #![feature(extern_types)]

#[repr(C)] pub struct _IO_wide_data{ _private:[u8;0]}
#[repr(C)] pub struct _IO_codecvt{ _private:[u8;0]}
#[repr(C)] pub struct _IO_marker{ _private:[u8;0]}
#[repr(C)] pub struct FILE { _private:[u8;0]}


//use rcommon::{word_list, word_desc, REDIRECTEE};
use rcommon::{word_list, word_desc};

pub const S_IFMT:libc::c_uint = 0o170000;
pub const S_IFREG:libc::c_uint = 0o100000;
pub const RX_INTERNAL:libc::c_int = 0x8;
pub const RX_SAVCLEXEC: libc::c_int = 0x20;

pub const F_GETFD :libc::c_int = 1;
pub const SUBSHELL_ASYNC: libc::c_int = 0x1;
pub const RX_CLEXEC :libc::c_int = 0x4;
pub const SHELL_FD_BASE : libc::c_int = 10;
pub const RX_SAVEFD : libc::c_int = 0x40;

pub const EXECUTION_FAILURE: libc::c_int = 1;

#[macro_export] 
macro_rules! readonly_p { 
    ($var:expr) => { 
        (*$var).attributes & 0x0000002 
    }; 
}

#[macro_export] 
macro_rules! noassign_p {
    ($var:expr) => {
        (*$var).attributes & 0x0004000
    }; 
} 
#[macro_export] 
macro_rules! array_p {
    ($var:expr) => {
        (*$var).attributes & 0x0000004
    }; 
} 

#[macro_export] 
macro_rules! assoc_p {
    ($var:expr) => {
        (*$var).attributes & 0x0000040
    }; 
} 
#[macro_export] 
macro_rules! invisible_p {
    ($var:expr) => {
        (*$var).attributes & 0x0001000
    }; 
} 


#[macro_export]
macro_rules! REDIRECTION_ERROR{
    ($r:expr, $errno:expr, $fd:expr)=>{
        if $r < 0 as libc::c_int {
            if $fd >= 0 as libc::c_int {
                close($fd);
            }
            set_exit_status(EXECUTION_FAILURE);
            return if *__errno_location() == 0 as libc::c_int {
                EINVAL 
            } else {
                *__errno_location()
            };
        }
    }
}

#[macro_export]
macro_rules! S_ISREG {
    ($m:expr) => {
      $m &  S_IFMT == S_IFREG
    };
}

#[macro_export]
macro_rules! W_NOGLOB {
    () => {
       (1 as libc::c_int) << 5 as libc::c_int         
    };
}

#[macro_export]
macro_rules! TRANSLATE_REDIRECT {
    ($ri: expr) => {
        $ri as libc::c_uint == r_duplicating_input_word as libc::c_int as libc::c_uint
        || $ri as libc::c_uint == r_duplicating_output_word as libc::c_int as libc::c_uint
        || $ri as libc::c_uint == r_move_input_word as libc::c_int as libc::c_uint
        || $ri as libc::c_uint == r_move_output_word as libc::c_int as libc::c_uint
        
    };
}

#[macro_export]
macro_rules! CLOBBERING_REDIRECT {
    ($ri:expr) => {
($ri as libc::c_uint == r_output_direction as libc::c_int as libc::c_uint
    || $ri as libc::c_uint == r_err_and_out as libc::c_int as libc::c_uint)
}
}

#[macro_export]
macro_rules! STRLEN {
    ($desc: expr) => {
        if !((*($desc as *mut WORD_DESC)).word).is_null()
        && *((*($desc as *mut WORD_DESC)).word).offset(0 as libc::c_int as isize) as libc::c_int
            != 0
    {
        if *((*($desc as *mut WORD_DESC)).word).offset(1 as libc::c_int as isize) as libc::c_int
            != 0
        {
            if *((*($desc  as *mut WORD_DESC)).word).offset(2 as libc::c_int as isize)
                as libc::c_int != 0
            {
                strlen((*($desc as *mut WORD_DESC)).word)
            } else {
                2 as libc::c_int as libc::c_ulong
            }
        } else {
            1 as libc::c_int as libc::c_ulong
        }
    } else {
        0 as libc::c_int as libc::c_ulong
    }
    }
}
#[macro_export]
macro_rules! WRITE_REDIRECT {
    ($ri:expr) => {
        $ri as libc::c_uint
                        == r_output_direction as libc::c_int as libc::c_uint
                        || $ri as libc::c_uint
                            == r_input_output as libc::c_int as libc::c_uint
                        || $ri as libc::c_uint == r_err_and_out as libc::c_int as libc::c_uint
                        || $ri as libc::c_uint
                            == r_appending_to as libc::c_int as libc::c_uint
                        || $ri as libc::c_uint
                            == r_append_err_and_out as libc::c_int as libc::c_uint
                        || $ri as libc::c_uint
                            == r_output_force as libc::c_int as libc::c_uint
    };
}
extern "C" {
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn fchmod(__fd: libc::c_int, __mode: libc::c_uint) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn free(_: *mut libc::c_void);
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn copy_word(_: *mut WORD_DESC) -> *mut WORD_DESC;
    fn copy_redirects(_: *mut REDIRECT) -> *mut REDIRECT;
    fn malloc(_: size_t) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn all_digits(_: *const libc::c_char) -> libc::c_int;
    fn legal_number(_: *const libc::c_char, _: *mut intmax_t) -> libc::c_int;
    fn same_file(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut stat,
        _: *mut stat,
    ) -> libc::c_int;
    fn sys_error(_: *const libc::c_char, _: ...);
    fn internal_error(_: *const libc::c_char, _: ...);
    fn find_variable(_: *const libc::c_char) -> *mut SHELL_VAR;
    fn find_variable_last_nameref(
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut SHELL_VAR;
    fn get_variable_value(_: *mut SHELL_VAR) -> *mut libc::c_char;
    fn bind_var_to_int(_: *mut libc::c_char, _: intmax_t) -> *mut SHELL_VAR;
    fn stupidly_hack_special_variables(_: *mut libc::c_char);
    fn sv_ifs(_: *mut libc::c_char);
    fn valid_array_reference(_: *const libc::c_char, _: libc::c_int) -> libc::c_int;
    fn get_array_value(
        _: *const libc::c_char,
        _: libc::c_int,
        _: *mut libc::c_int,
        _: *mut arrayind_t,
    ) -> *mut libc::c_char;
    fn array_variable_part(
        _: *const libc::c_char,
        _: libc::c_int,
        _: *mut *mut libc::c_char,
        _: *mut libc::c_int,
    ) -> *mut SHELL_VAR;
    fn termsig_handler(_: libc::c_int);
    fn throw_to_top_level();
    static mut interrupt_state: sig_atomic_t;
    static mut terminating_signal: sig_atomic_t;
    fn dispose_words(_: *mut WORD_LIST);
    fn dispose_redirects(_: *mut REDIRECT);
    fn make_bare_word(_: *const libc::c_char) -> *mut WORD_DESC;
    fn make_word_list(_: *mut WORD_DESC, _: *mut WORD_LIST) -> *mut WORD_LIST;
    fn make_redirection(
        _: rcommon::REDIRECTEE,
        _: r_instruction,
        _: rcommon::REDIRECTEE,
        _: libc::c_int,
    ) -> *mut REDIRECT;
    fn xtrace_fdchk(_: libc::c_int);
    fn set_exit_status(_: libc::c_int);
    fn find_string_in_alist(
        _: *mut libc::c_char,
        _: *mut STRING_INT_ALIST,
        _: libc::c_int,
    ) -> libc::c_int;
    fn xbcopy(_: *mut libc::c_char, _: *mut libc::c_char, _: libc::c_int);
    fn fpurge(stream: *mut FILE) -> libc::c_int;
    fn itos(_: intmax_t) -> *mut libc::c_char;
    fn netopen(_: *mut libc::c_char) -> libc::c_int;
    fn sh_mktmpfd(
        _: *mut libc::c_char,
        _: libc::c_int,
        _: *mut *mut libc::c_char,
    ) -> libc::c_int;
    static mut interactive_shell: libc::c_int;
    static mut subshell_environment: libc::c_int;
    static mut posixly_correct: libc::c_int;
    fn string_list(_: *mut WORD_LIST) -> *mut libc::c_char;
    fn expand_string_to_string(
        _: *mut libc::c_char,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    fn expand_assignment_string_to_string(
        _: *mut libc::c_char,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    fn expand_words_no_vars(_: *mut WORD_LIST) -> *mut WORD_LIST;
    static mut noclobber: libc::c_int;
    static mut restricted: libc::c_int;
    static mut executing_builtin: libc::c_int;
    fn dispose_exec_redirects();
    fn coproc_fdchk(_: libc::c_int);
    fn run_pending_traps();
    fn check_bash_input(_: libc::c_int) -> libc::c_int;
    fn duplicate_buffered_stream(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn close_buffered_fd(_: libc::c_int) -> libc::c_int;
    static mut redirection_undo_list: *mut REDIRECT;
    static mut exec_redirection_undo_list: *mut REDIRECT;
}
pub type size_t = libc::c_ulong;

pub type ssize_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: libc::c_long,
    pub tv_nsec: libc::c_long,
}
pub type intmax_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: libc::c_ulong,
    pub st_ino: libc::c_ulong,
    pub st_nlink: libc::c_ulong,
    pub st_mode: libc::c_uint,
    pub st_uid: libc::c_uint,
    pub st_gid: libc::c_uint,
    pub __pad0: libc::c_int,
    pub st_rdev: libc::c_ulong,
    pub st_size: libc::c_long,
    pub st_blksize: libc::c_long,
    pub st_blocks: libc::c_long,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [libc::c_long; 3],
}

pub type r_instruction = libc::c_uint;

// Redirection errors.
pub const AMBIGUOUS_REDIRECT  :libc::c_int = -1;
pub const NOCLOBBER_REDIRECT  :libc::c_int = -2;
pub const RESTRICTED_REDIRECT :libc::c_int = -3; /* can only happen in restricted shells. */
pub const  HEREDOC_REDIRECT   :libc::c_int = -4; /* here-doc temp file can't be created */
pub const  BADVAR_REDIRECT    :libc::c_int = -5;    /* something wrong with {varname}redir */

pub const REDIR_VARASSIGN  :libc::c_int  = 0x01;
pub const HEREDOC_PIPESIZE :size_t       = 65535;
pub const F_GETPIPE_SZ     : libc::c_int = 1032;

pub const EPERM: libc::c_int = 1;
pub const ENOENT: libc::c_int = 2;
pub const ESRCH:  libc::c_int = 3;
pub const EINTR:  libc::c_int = 4;
pub const EIO:    libc::c_int = 5;
pub const ENXIO: libc::c_int = 6;
pub const E2BIG: libc::c_int = 7;
pub const ENOEXEC: libc::c_int = 8;
pub const EBADF: libc::c_int = 9;
pub const ECHILD: libc::c_int = 10;
pub const EAGAIN: libc::c_int = 11;
pub const ENOMEM: libc::c_int = 12;
pub const EACCES: libc::c_int = 13;
pub const EFAULT: libc::c_int = 14;
pub const ENOTBLK: libc::c_int = 15;
pub const EBUSY: libc::c_int = 16;
pub const EEXIST: libc::c_int = 17;
pub const EXDEV: libc::c_int = 18;
pub const ENODEV: libc::c_int = 19;
pub const ENOTDIR: libc::c_int = 20; 
pub const EISDIR: libc::c_int = 21;
pub const EINVAL: libc::c_int = 22;
pub const ENFILE: libc::c_int = 23;
pub const EMFILE: libc::c_int = 24;
pub const ENOTTY: libc::c_int = 25;
pub const ETXTBSY: libc::c_int = 26; 
pub const EFBIG: libc::c_int = 27;
pub const ENOSPC: libc::c_int = 28;
pub const ESPIPE: libc::c_int = 29;
pub const EROFS: libc::c_int = 30;
pub const EMLINK: libc::c_int = 31;
pub const EPIPE: libc::c_int = 32;
pub const EDOM: libc::c_int = 33;
pub const ERANGE: libc::c_int = 34;
/*
pub const r_output_direction: libc::c_uint = 0;
pub const r_input_direction: libc::c_uint = 1;
pub const r_inputa_direction: libc::c_uint = 2;
pub const r_appending_to: libc::c_uint = 3;
pub const r_reading_until: libc::c_uint = 4;
pub const r_reading_string: libc::c_uint = 5;
pub const r_duplicating_input: libc::c_uint = 6;
pub const r_duplicating_output: libc::c_uint = 7;
pub const r_deblank_reading_until: libc::c_uint = 8;
pub const r_close_this: libc::c_uint = 9;
pub const r_err_and_out: libc::c_uint = 10;
pub const r_input_output: libc::c_uint = 11;
pub const r_output_force: libc::c_uint = 12;
pub const r_duplicating_input_word: libc::c_uint = 13;
pub const r_duplicating_output_word: libc::c_uint = 14;
pub const r_move_input: libc::c_uint = 15;
pub const r_move_output: libc::c_uint = 16;
pub const r_move_input_word: libc::c_uint = 17;
pub const r_move_output_word: libc::c_uint = 18;
pub const r_append_err_and_out: libc::c_uint = 19;
 */

 use rcommon::{r_output_direction, r_input_direction, r_inputa_direction, r_appending_to, r_reading_until, r_reading_string, 
    r_duplicating_input, r_duplicating_output, r_deblank_reading_until, r_close_this, r_err_and_out, r_input_output, r_output_force,
    r_duplicating_input_word, r_duplicating_output_word, r_move_input, r_move_output, r_move_input_word, r_move_output_word, r_append_err_and_out};
pub type WORD_DESC = word_desc;
pub type WORD_LIST = word_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redirect {
    pub next: *mut redirect,
    pub redirector: rcommon::REDIRECTEE,
    pub rflags: libc::c_int,
    pub flags: libc::c_int,
    pub instruction: libc::c_uint,
    pub redirectee: rcommon::REDIRECTEE,
    pub here_doc_eof: *mut libc::c_char,
}
pub type REDIRECT = redirect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct STRING_INT_ALIST {
    pub word: *mut libc::c_char,
    pub token: libc::c_int,
}
pub type arrayind_t = intmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub exportstr: *mut libc::c_char,
    pub dynamic_value: Option::<sh_var_value_func_t>,
    pub assign_func: Option::<sh_var_assign_func_t>,
    pub attributes: libc::c_int,
    pub context: libc::c_int,
}
pub type sh_var_assign_func_t = unsafe extern "C" fn(
    *mut variable,
    *mut libc::c_char,
    arrayind_t,
    *mut libc::c_char,
) -> *mut variable;
pub type sh_var_value_func_t = unsafe extern "C" fn(*mut variable) -> *mut variable;
pub type SHELL_VAR = variable;
pub type sig_atomic_t = libc::c_int;
#[no_mangle]
pub static mut expanding_redir: libc::c_int = 0;
static mut rd: rcommon::REDIRECTEE = rcommon::REDIRECTEE { dest: 0 };
static mut heredoc_errno: libc::c_int = 0;


#[no_mangle]
pub unsafe extern "C" fn redirection_error(
    mut temp: *mut REDIRECT,
    mut error: libc::c_int,
    mut fn_0: *mut libc::c_char,
) {
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut allocname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut oflags: libc::c_int = 0;
    allocname = 0 as *mut libc::c_char;
    if (*temp).rflags & 0x1 as libc::c_int != 0 && error < 0 as libc::c_int {
        allocname = strcpy(
            malloc(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(strlen((*(*temp).redirector.filename).word)),
            ) as *mut libc::c_char,
            (*(*temp).redirector.filename).word,
        );
        filename = allocname;
    } else if (*temp).rflags & REDIR_VARASSIGN as libc::c_int == 0 as libc::c_int
        && (*temp).redirector.dest < REDIR_VARASSIGN as libc::c_int
    {
        filename = dcgettext(
            0 as *const libc::c_char,
            b"file descriptor out of range\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
    } else if error != NOCLOBBER_REDIRECT && (*temp).redirector.dest >= 0 as libc::c_int
        && error == EBADF as libc::c_int
    {
        match (*temp).instruction as libc::c_uint {
            r_duplicating_input | r_duplicating_output | r_move_input | r_move_output =>{
                allocname = itos((*temp).redirectee.dest as intmax_t);
                filename = allocname;
            }
            r_duplicating_input_word => {
                if (*temp).redirector.dest == 0 as libc::c_int {
                    filename = (*(*temp).redirectee.filename).word;
                } else {
                    allocname = itos((*temp).redirector.dest as intmax_t);
                    filename = allocname;
                }
            }
            r_duplicating_output_word => {
                if (*temp).redirector.dest == 1 as libc::c_int {
                    filename = (*(*temp).redirectee.filename).word;
                } else {
                    allocname = itos((*temp).redirector.dest as intmax_t);
                    filename = allocname;
                }
            }
            _ => {
                allocname = itos((*temp).redirector.dest as intmax_t);
                filename = allocname;
            }
        }
    } else if !fn_0.is_null() {
        filename = fn_0;
    } else if expandable_redirection_filename(temp) != 0 {
        oflags = (*(*temp).redirectee.filename).flags;
        if posixly_correct != 0 && interactive_shell == 0 as libc::c_int {
            (*(*temp).redirectee.filename).flags
                |= (1 as libc::c_int) << 5 as libc::c_int;
        }
        (*(*temp).redirectee.filename).flags |= (1 as libc::c_int) << 10 as libc::c_int;
        allocname = redirection_expand((*temp).redirectee.filename);
        filename = allocname;
        (*(*temp).redirectee.filename).flags = oflags;
        if filename.is_null() {
            filename = (*(*temp).redirectee.filename).word;
        }
    } else if (*temp).redirectee.dest < 0 as libc::c_int {
        filename = dcgettext(
            0 as *const libc::c_char,
            b"file descriptor out of range\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
    } else {
        allocname = itos((*temp).redirectee.dest as intmax_t);
        filename = allocname;
    }

    match error {
        AMBIGUOUS_REDIRECT => {
            internal_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: ambiguous redirect\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                filename,
            );
        }
        NOCLOBBER_REDIRECT => {
            internal_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: cannot overwrite existing file\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                filename,
            );
        }
        RESTRICTED_REDIRECT => {
            internal_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: restricted: cannot redirect output\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                filename,
            );
        }
        HEREDOC_REDIRECT => {
            internal_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot create temp file for here-document: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                strerror(heredoc_errno),
            );
        }
        BADVAR_REDIRECT => {
            internal_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: cannot assign fd to variable\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                filename,
            );
        }
        _ => {
            internal_error(
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                filename,
                strerror(error),
            );
        }
    }
    if !allocname.is_null() {
        free(allocname as *mut libc::c_void);
    }
    allocname = 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn do_redirections(
    mut list: *mut REDIRECT,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut error: libc::c_int = 0;
    let mut temp: *mut REDIRECT = 0 as *mut REDIRECT;
    let mut fn_0: *mut libc::c_char = 0 as *mut libc::c_char;
    if flags & (r_bash::RX_UNDOABLE as i32) != 0 {
        if !redirection_undo_list.is_null() {
            dispose_redirects(redirection_undo_list);
            redirection_undo_list = 0 as *mut libc::c_void as *mut REDIRECT;
        }
        if !exec_redirection_undo_list.is_null() {
            dispose_exec_redirects();
        }
    }
    //println!("in {}:{}", stdext::function_name!(),std::line!());
    temp = list;
    while !temp.is_null() {
        fn_0 = 0 as *mut libc::c_char;
        error = do_redirection_internal(temp, flags, &mut fn_0);
        if error != 0 {
            redirection_error(temp, error, fn_0);
            if !fn_0.is_null() {
                free(fn_0 as *mut libc::c_void);
            }
            fn_0 = 0 as *mut libc::c_char;
            return error;
        }
        if !fn_0.is_null() {
            free(fn_0 as *mut libc::c_void);
        }
        fn_0 = 0 as *mut libc::c_char;
        temp = (*temp).next;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn expandable_redirection_filename(
    mut redirect: *mut REDIRECT,
) -> libc::c_int {
    match (*redirect).instruction as libc::c_uint {
        r_output_direction | r_appending_to | r_input_direction | r_inputa_direction | r_err_and_out | r_append_err_and_out | r_input_output | r_output_force | r_duplicating_input_word | r_duplicating_output_word | r_move_input_word | r_move_output_word => return 1 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}

#[no_mangle]
pub unsafe extern "C" fn redirection_expand(
    mut word: *mut WORD_DESC,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tlist1: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut tlist2: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut w: *mut WORD_DESC = 0 as *mut WORD_DESC;
    let mut old: libc::c_int = 0;
    w = copy_word(word);
    if posixly_correct != 0 {
        (*w).flags |= (1 as libc::c_int) << 4 as libc::c_int;
    }
    tlist1 = make_word_list(w, 0 as *mut libc::c_void as *mut WORD_LIST);
    expanding_redir = 1 as libc::c_int;
    sv_ifs(b"IFS\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    tlist2 = expand_words_no_vars(tlist1);
    expanding_redir = 0 as libc::c_int;
    /* Now we need to change the variable search order back to include the temp
     environment.  We force the temp environment search by forcing
     executing_builtin to 1.  This is what makes `read' get the right values
     for the IFS-related cached variables, for example. */
    old = executing_builtin;
    executing_builtin = 1 as libc::c_int;
    sv_ifs(b"IFS\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    executing_builtin = old;
    dispose_words(tlist1);
    if tlist2.is_null() || !((*tlist2).next).is_null() {
        /* We expanded to no words, or to more than a single word.
         Dispose of the word list and return NULL. */
        if !tlist2.is_null() {
            dispose_words(tlist2);
        }
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    result = string_list(tlist2); /* XXX savestring (tlist2->word->word)? */
    dispose_words(tlist2);
    return result;
}

/* Expand a here-document or here-string (determined by RI) contained in
   REDIRECTEE and return the expanded document. If LENP is non-zero, put
   the length of the returned string into *LENP.

   This captures everything about expanding here-documents and here-strings:
   the returned document should be written directly to whatever file
   descriptor is specified. In particular, it adds a newline to the end of
   a here-string to preserve previous semantics. */
unsafe extern "C" fn heredoc_expand(
    mut redirectee: *mut WORD_DESC,
    mut ri: libc::c_uint,
    mut lenp: *mut size_t,
) -> *mut libc::c_char {
    let mut document: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dlen: size_t = 0;
    let mut old: libc::c_int = 0;
    if ((*redirectee).word).is_null()
        || *((*redirectee).word).offset(0 as libc::c_int as isize) as libc::c_int
            == '\0' as i32
    {
        if !lenp.is_null() {
            *lenp = 0 as libc::c_int as size_t;
        }
        return (*redirectee).word;
    }
    if ri as libc::c_uint != r_reading_string 
        && (*redirectee).flags & (1 as libc::c_int) << 1 as libc::c_int != 0
    {
        if !lenp.is_null() {
            (*lenp) = STRLEN!(redirectee);
        }
        return (*redirectee).word;
    }
    expanding_redir = 1 as libc::c_int;
    sv_ifs(b"IFS\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if ri as libc::c_uint == r_reading_string {
        document = expand_assignment_string_to_string((*redirectee).word, 0 as libc::c_int)
    } else {
        document = expand_string_to_string((*redirectee).word, 0x2 as libc::c_int)
    };
    expanding_redir = 0 as libc::c_int;
    old = executing_builtin;
    executing_builtin = 1 as libc::c_int;
    sv_ifs(b"IFS\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    executing_builtin = old;
    dlen = STRLEN!(document);
    if ri as libc::c_uint == r_reading_string {
        document = realloc(
            document as *mut libc::c_void,
            dlen.wrapping_add(2 as libc::c_int as size_t),
        ) as *mut libc::c_char;
        let fresh0 = dlen;
        dlen = dlen.wrapping_add(1);
        *document.offset(fresh0 as isize) = '\n' as i32 as libc::c_char;
        *document.offset(dlen as isize) = '\0' as i32 as libc::c_char;
    }
    if !lenp.is_null() {
        *lenp = dlen;
    }
    return document;
}
unsafe extern "C" fn heredoc_write(
    mut fd: libc::c_int,
    mut heredoc: *mut libc::c_char,
    mut herelen: size_t,
) -> libc::c_int {
    let mut nw: ssize_t = 0;
    let mut e: libc::c_int = 0;
    *__errno_location() = 0 as libc::c_int;
    nw = write(fd, heredoc as *const libc::c_void, herelen);
    e = *__errno_location();
    if nw as size_t != herelen {
        if e == 0 as libc::c_int {
            e = ENOSPC;
        }
        return e;
    }
    return 0 as libc::c_int;
}


unsafe extern "C" fn here_document_to_fd(
    mut redirectee: *mut WORD_DESC,
    mut ri: libc::c_uint, //枚举
) -> libc::c_int {
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    let mut fd2: libc::c_int = 0;
    let mut herepipe: [libc::c_int; 2] = [0; 2];
    let mut document: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut document_len: size_t = 0;
    document = heredoc_expand(redirectee, ri, &mut document_len);
    if document_len == 0 as libc::c_int as size_t {
        fd = open(b"/dev/null\0" as *const u8 as *const libc::c_char, 0 as libc::c_int);
        r = *__errno_location();
        if document != (*redirectee).word {
            if !document.is_null() {
                free(document as *mut libc::c_void);
            }
            document = 0 as *mut libc::c_char;
        }
        *__errno_location() = r;
        return fd;
    }
    if document_len <= HEREDOC_PIPESIZE as libc::c_int as size_t {
        if pipe(herepipe.as_mut_ptr()) < 0 as libc::c_int {
            r = *__errno_location();
            if document != (*redirectee).word {
                free(document as *mut libc::c_void);
            }
            *__errno_location() = r;
            return -(1 as libc::c_int);
        }
        if !((fcntl(
            herepipe[1 as libc::c_int as usize],
            F_GETPIPE_SZ,
            0 as libc::c_int,
        ) as size_t) < document_len)
        {
            r = heredoc_write(
                herepipe[1 as libc::c_int as usize],
                document,
                document_len,
            );
            if document != (*redirectee).word {
                free(document as *mut libc::c_void);
            }
            close(herepipe[1 as libc::c_int as usize]);
            if r != 0 {
                close(herepipe[0 as libc::c_int as usize]);
                *__errno_location() = r;
                return -(1 as libc::c_int);
            }
            return herepipe[0 as libc::c_int as usize];
        }
    }
    fd = sh_mktmpfd(
        b"sh-thd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (r_bash::MT_USERANDOM | r_bash::MT_USETMPDIR) as i32,
        &mut filename,
    );
    if fd < 0 as libc::c_int {
        r = *__errno_location();
        if !filename.is_null() {
            free(filename as *mut libc::c_void);
        }
        filename = 0 as *mut libc::c_char;
        if document != (*redirectee).word {
            if !document.is_null() {
                free(document as *mut libc::c_void);
            }
            document = 0 as *mut libc::c_char;
        }
        *__errno_location() = r;
        return fd;
    }

    // fchmod (fd, S_IRUSR | S_IWUSR);

    fchmod(fd, (r_bash::S_IRUSR| r_bash::S_IWUSR)as libc::c_uint);
    fcntl(fd, 2 as libc::c_int, 1 as libc::c_int);
    r = 0 as libc::c_int;

    *__errno_location() = r;
    r = heredoc_write(fd, document, document_len);
    if document != (*redirectee).word {
        if !document.is_null() {
            free(document as *mut libc::c_void);
        }
        document = 0 as *mut libc::c_char;
    }
    if r != 0 {
        close(fd);
        unlink(filename);
        free(filename as *mut libc::c_void);
        *__errno_location() = r;
        return -(1 as libc::c_int);
    }
    fd2 = open(filename, 0 as libc::c_int | 0 as libc::c_int, 0o600 as libc::c_int);
    if fd2 < 0 as libc::c_int {
        r = *__errno_location();
        unlink(filename);
        free(filename as *mut libc::c_void);
        close(fd);
        *__errno_location() = r;
        return -(1 as libc::c_int);
    }
    close(fd);
    if unlink(filename) < 0 as libc::c_int {
        r = *__errno_location();
        close(fd2);
        free(filename as *mut libc::c_void);
        *__errno_location() = r;
        return -(1 as libc::c_int);
    }
    free(filename as *mut libc::c_void);
    fchmod(fd2, 0o400 as libc::c_int as libc::c_uint);
    return fd2;
}

pub const RF_END : libc::c_int = -1;
pub const RF_DEVFD : libc::c_int = 1;
pub const RF_DEVSTDERR : libc::c_int = 2;
pub const RF_DEVSTDIN : libc::c_int = 3;
pub const RF_DEVSTDOUT : libc::c_int = 4;
pub const RF_DEVTCP : libc::c_int = 5;
pub const RF_DEVUDP : libc::c_int = 6;

static mut _redir_special_filenames: [STRING_INT_ALIST; 3] = [
    {
        let mut init = STRING_INT_ALIST {
            word: b"/dev/tcp/*/*\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            token: RF_DEVTCP,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"/dev/udp/*/*\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            token: RF_DEVUDP,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
            token: RF_END,
        };
        init
    },
];
unsafe extern "C" fn redir_special_open(
    mut spec: libc::c_int,
    mut filename: *mut libc::c_char,
    mut _flags: libc::c_int,
    mut _mode: libc::c_int,
    mut _ri: r_instruction,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    fd = RF_END;
    match spec {
        RF_DEVTCP | RF_DEVUDP => {
            if restricted != 0 {
                return RESTRICTED_REDIRECT;
            }
            fd = netopen(filename);
        }
        _ => {}
    }
    return fd;
}


unsafe extern "C" fn noclobber_open(
    mut filename: *mut libc::c_char,
    mut flags: libc::c_int,
    mut mode: libc::c_int,
    mut _ri: r_instruction,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    let mut finfo: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut finfo2: stat = stat {
        ..finfo
    };
    r = stat(filename, &mut finfo);
    if r == 0 && S_ISREG!(finfo.st_mode)
    {
        return NOCLOBBER_REDIRECT;
    }

    flags &= !(r_bash::O_TRUNC as i32);
    if r != 0 as libc::c_int {
        fd = open(filename, flags | r_bash::O_EXCL as libc::c_int, mode);
        return if fd < 0 as libc::c_int && *__errno_location() == 17 as libc::c_int {
            NOCLOBBER_REDIRECT
        } else {
            fd
        };
    }
    fd = open(filename, flags, mode);
    if fd < 0 as libc::c_int {
        return if *__errno_location() == EEXIST as libc::c_int {
            NOCLOBBER_REDIRECT
        } else {
            fd
        };
    }
    if fstat(fd, &mut finfo2) == 0 as libc::c_int
        && (finfo2.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint) as libc::c_int == 0 as libc::c_int
        && r == 0 as libc::c_int
        && (finfo.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint) as libc::c_int == 0 as libc::c_int
        && same_file(filename, filename, &mut finfo, &mut finfo2) != 0
    {
        return fd;
    }
    close(fd);
    *__errno_location() = EEXIST;
    return NOCLOBBER_REDIRECT;
}


unsafe extern "C" fn redir_open(
    mut filename: *mut libc::c_char,
    mut flags: libc::c_int,
    mut mode: libc::c_int,
    mut ri: r_instruction,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    r = find_string_in_alist(
        filename,
        _redir_special_filenames.as_mut_ptr(),
        1 as libc::c_int,
    );
    if r >= 0 as libc::c_int {
        return redir_special_open(r, filename, flags, mode, ri as libc::c_uint);
    }
    if noclobber != 0
        && CLOBBERING_REDIRECT!(ri)
    {
        fd = noclobber_open(filename, flags, mode, ri as libc::c_uint);
        if fd == NOCLOBBER_REDIRECT {
            return NOCLOBBER_REDIRECT;
        }
    } else {
        loop {
            fd = open(filename, flags, mode);
            e = *__errno_location();
            if fd < 0 as libc::c_int && e == EINTR {
                if terminating_signal != 0 {
                    termsig_handler(terminating_signal);
                }
                if interrupt_state != 0 {
                    throw_to_top_level();
                }
                run_pending_traps();
            }
            *__errno_location() = e;
            if !(fd < 0 as libc::c_int && *__errno_location() == EINTR) {
                break;
            }
        }
    }
    return fd;
}


unsafe extern "C" fn do_redirection_internal(
    mut redirect: *mut REDIRECT,
    mut flags: libc::c_int,
    mut fnp: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut redirectee: *mut WORD_DESC = 0 as *mut WORD_DESC;
    let mut redir_fd: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    let mut redirector: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut oflags: libc::c_int = 0;
    let mut lfd: intmax_t = 0;
    let mut redirectee_word: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ri: r_instruction = r_output_direction;
    let mut new_redirect: *mut REDIRECT = 0 as *mut REDIRECT;
    let mut sd: rcommon::REDIRECTEE = rcommon::REDIRECTEE { dest: 0 };
    redirectee = (*redirect).redirectee.filename;
    redir_fd = (*redirect).redirectee.dest;
    redirector = (*redirect).redirector.dest;
    ri = (*redirect).instruction;
    if (*redirect).flags & RX_INTERNAL != 0 {
        flags |= RX_INTERNAL;
    }
    if TRANSLATE_REDIRECT!(ri)
    {
        redirectee_word = redirection_expand(redirectee);
        if redirectee_word.is_null() {
            return AMBIGUOUS_REDIRECT
        } else if *redirectee_word.offset(0 as libc::c_int as isize) as libc::c_int
            == '-' as i32
            && *redirectee_word.offset(1 as libc::c_int as isize) as libc::c_int
                == '\0' as i32
        {
            sd = (*redirect).redirector;
            rd.dest = 0 as libc::c_int;
            new_redirect = make_redirection(sd, r_close_this, rd, 0 as libc::c_int);
        } else if all_digits(redirectee_word) != 0 {
            sd = (*redirect).redirector;
            if legal_number(redirectee_word, &mut lfd) != 0
                && lfd as libc::c_int as intmax_t == lfd
            {
                rd.dest = lfd as libc::c_int;
            } else {
                rd.dest = -(1 as libc::c_int);
            }
            match ri as libc::c_uint {
                r_duplicating_input_word => {
                    new_redirect = make_redirection(
                        sd,
                        r_duplicating_input,
                        rd,
                        0 as libc::c_int,
                    );
                }
                r_duplicating_output_word => {
                    new_redirect = make_redirection(
                        sd,
                        r_duplicating_output,
                        rd,
                        0 as libc::c_int,
                    );
                }
                r_move_input_word => {
                    new_redirect = make_redirection(
                        sd,
                        r_move_input,
                        rd,
                        0 as libc::c_int,
                    );
                }
                r_move_output_word => {
                    new_redirect = make_redirection(
                        sd,
                        r_move_output,
                        rd,
                        0 as libc::c_int,
                    );
                }
                _ => {}
            }
        } else if ri as libc::c_uint
            == r_duplicating_output_word as libc::c_int as libc::c_uint
            && (*redirect).rflags & REDIR_VARASSIGN  as libc::c_int == 0 as libc::c_int
            && redirector == 1 as libc::c_int
        {
            sd = (*redirect).redirector;
            rd.filename = make_bare_word(redirectee_word);
            new_redirect = make_redirection(sd, r_err_and_out, rd, 0 as libc::c_int);
        } else {
            free(redirectee_word as *mut libc::c_void);
            return REDIR_VARASSIGN;
        }
        free(redirectee_word as *mut libc::c_void);
        if (*new_redirect).instruction as libc::c_uint
            == r_err_and_out as libc::c_int as libc::c_uint
        {
            let mut alloca_hack: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut fresh1 = ::std::vec::from_elem(
                0,
                ::core::mem::size_of::<WORD_DESC>() as libc::c_ulong as usize,
            );
            redirectee = fresh1.as_mut_ptr() as *mut WORD_DESC;
            xbcopy(
                (*new_redirect).redirectee.filename as *mut libc::c_char,
                redirectee as *mut libc::c_char,
                ::core::mem::size_of::<WORD_DESC>() as libc::c_ulong as libc::c_int,
            );
            let mut fresh2 = ::std::vec::from_elem(
                0,
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(strlen((*(*new_redirect).redirectee.filename).word))
                    as usize,
            );
            alloca_hack = fresh2.as_mut_ptr() as *mut libc::c_char;
            (*redirectee).word = alloca_hack;
            strcpy((*redirectee).word, (*(*new_redirect).redirectee.filename).word);
        } else {
            redirectee = (*new_redirect).redirectee.filename;
        }
        redir_fd = (*new_redirect).redirectee.dest;
        redirector = (*new_redirect).redirector.dest;
        ri = (*new_redirect).instruction;
        (*redirect).flags = (*new_redirect).flags;
        dispose_redirects(new_redirect);
    }

    let mut _current_block_flag: u64;
    match ri as libc::c_uint {
        r_output_direction | r_appending_to | r_input_direction | r_inputa_direction| r_err_and_out | r_append_err_and_out | r_input_output | r_output_force => {
            {
                if posixly_correct != 0 && interactive_shell == 0 as libc::c_int {
                    oflags = (*redirectee).flags;
                    (*redirectee).flags |= W_NOGLOB!();
                }
                redirectee_word = redirection_expand(redirectee);
                if posixly_correct != 0 && interactive_shell == 0 as libc::c_int {
                    (*redirectee).flags = oflags;
                }
                if redirectee_word.is_null() {
                    return AMBIGUOUS_REDIRECT;
                }
                if restricted != 0
                    && WRITE_REDIRECT!(ri )
                {
                    free(redirectee_word as *mut libc::c_void);
                    return RESTRICTED_REDIRECT;
                }
                fd = redir_open(
                    redirectee_word,
                    (*redirect).flags,
                    0o666 as libc::c_int,
                    ri as libc::c_uint,
                );
                if !fnp.is_null() {
                    *fnp = redirectee_word;
                } else {
                    free(redirectee_word as *mut libc::c_void);
                }
                if fd == NOCLOBBER_REDIRECT || fd == RESTRICTED_REDIRECT {
                    return fd;
                }
                if fd < 0 as libc::c_int {
                    return *__errno_location();
                }
                if flags & (r_bash::RX_ACTIVE as i32) != 0 {
                    if (*redirect).rflags & REDIR_VARASSIGN != 0 {
                        redirector = fcntl(fd, r_bash::F_DUPFD as i32, SHELL_FD_BASE as libc::c_int);
                        r = *__errno_location();
                        if redirector < 0 as libc::c_int {
                            sys_error(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"redirection error: cannot duplicate fd\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        }
                        REDIRECTION_ERROR! (redirector, r, fd);
 
                    }
                    if flags & r_bash::RX_UNDOABLE as i32 != 0
                        && (*redirect).rflags & REDIR_VARASSIGN == 0 
                    {
                        if fd != redirector
                            && fcntl(redirector, F_GETFD, 0 as libc::c_int)
                                != -(1 as libc::c_int)
                        {
                            r = add_undo_redirect(redirector, ri, -(1 as libc::c_int));
                        } else {
                            r = add_undo_close_redirect(redirector);
                        }
                        REDIRECTION_ERROR! (r, errno, fd);
                    }
                    if redirector != 0 as libc::c_int
                        || subshell_environment & SUBSHELL_ASYNC as libc::c_int == 0 as libc::c_int
                    {
                        check_bash_input(redirector);
                    }
                    if redirector == 1 as libc::c_int && fileno(stdout) == redirector {
                        fflush(stdout);
                        fpurge(stdout);
                    } else if redirector == 2 as libc::c_int && fileno(stderr) == redirector
                    {
                        fflush(stderr);
                        fpurge(stderr);
                    }

                    if (*redirect).rflags & REDIR_VARASSIGN as libc::c_int != 0 {
                        r = redir_varassign(redirect, redirector);
                        if r < 0 as libc::c_int {
                            close(redirector);
                            close(fd);
                            return r;
                        }
                    } else if fd != redirector && dup2(fd, redirector) < 0 as libc::c_int {
                        close(fd);
                        return *__errno_location();
                    }
                    
                    if ri as libc::c_uint == r_input_direction as libc::c_int as libc::c_uint
                        || ri as libc::c_uint
                            == r_input_output as libc::c_int as libc::c_uint
                    {
                        duplicate_buffered_stream(fd, redirector);
                    }

                    if flags & RX_CLEXEC  as libc::c_int != 0 && redirector > 2 as libc::c_int {
                        fcntl(redirector, 2 as libc::c_int, 1 as libc::c_int);
                    }
                }
                if fd != redirector {
                    if ri as libc::c_uint == r_input_direction as libc::c_int as libc::c_uint
                        || ri as libc::c_uint
                            == r_inputa_direction as libc::c_int as libc::c_uint
                        || ri as libc::c_uint
                            == r_input_output as libc::c_int as libc::c_uint
                    {
                        close_buffered_fd(fd);
                    } else {
                        close(fd);
                    }
                }
                if ri as libc::c_uint == r_err_and_out as libc::c_int as libc::c_uint
                    || ri as libc::c_uint
                        == r_append_err_and_out as libc::c_int as libc::c_uint
                {
                    if flags & r_bash::RX_ACTIVE  as libc::c_int != 0 {
                        if flags &  r_bash::RX_UNDOABLE  as libc::c_int != 0 {
                            add_undo_redirect(2 as libc::c_int, ri, -(1 as libc::c_int));
                        }
                        if dup2(1 as libc::c_int, 2 as libc::c_int) < 0 as libc::c_int {
                            return *__errno_location();
                        }
                    }
                }
            }
        }

        r_reading_until | r_deblank_reading_until | r_reading_string => {
            if !redirectee.is_null() {
                fd = here_document_to_fd(redirectee, ri as libc::c_uint);
                if fd < 0 as libc::c_int {
                    heredoc_errno = *__errno_location();
                    return HEREDOC_REDIRECT;
                }
                if (*redirect).rflags & REDIR_VARASSIGN as libc::c_int != 0 {
                    redirector = fcntl(fd, r_bash::F_DUPFD as i32, SHELL_FD_BASE);
                    r = *__errno_location();
                    if redirector < 0 as libc::c_int {
                        sys_error(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"redirection error: cannot duplicate fd\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                    REDIRECTION_ERROR! (r, errno, fd);
                }
                if flags & r_bash::RX_ACTIVE as libc::c_int != 0 {
                    if flags & r_bash::RX_UNDOABLE as libc::c_int != 0
                        && (*redirect).rflags & REDIR_VARASSIGN as libc::c_int == 0 as libc::c_int
                    {
                        if fd != redirector
                            && fcntl(redirector, F_GETFD, 0 as libc::c_int)
                                != -(1 as libc::c_int)
                        {
                            r = add_undo_redirect(redirector, ri, -(1 as libc::c_int));
                        } else {
                            r = add_undo_close_redirect(redirector);
                        }

                        REDIRECTION_ERROR! (r, errno, fd);
                    }

                    check_bash_input(redirector);

                    if (*redirect).rflags & REDIR_VARASSIGN != 0 {
                        r = redir_varassign(redirect, redirector);
                        if r < 0 as libc::c_int {
                            close(redirector);
                            close(fd);
                            return r;
                        }
                    } else if fd != redirector && dup2(fd, redirector) < 0 as libc::c_int
                    {
                        r = *__errno_location();
                        close(fd);
                        return r;
                    }
                    duplicate_buffered_stream(fd, redirector);

                    if flags & RX_CLEXEC != 0 && redirector > 2 as libc::c_int {
                        fcntl(redirector, 2 as libc::c_int, 1 as libc::c_int);
                    }
                }
                if fd != redirector {
                    close_buffered_fd(fd);
                }
            }
        }

        r_duplicating_input | r_duplicating_output | r_move_input | r_move_output => {
            if flags & r_bash::RX_ACTIVE as i32!= 0
                && (*redirect).rflags & REDIR_VARASSIGN != 0
            {
                redirector = fcntl(redir_fd, 0 as libc::c_int, 10 as libc::c_int);
                r = *__errno_location();
                if redirector < 0 as libc::c_int {
                    sys_error(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"redirection error: cannot duplicate fd\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                REDIRECTION_ERROR! (redirector, r, -1);
            }
            if flags & r_bash::RX_ACTIVE as i32 != 0 && redir_fd != redirector {
                if flags & r_bash::RX_UNDOABLE as i32 != 0
                    && (*redirect).rflags & REDIR_VARASSIGN == 0 as libc::c_int
                {
                    if fcntl(redirector, 1 as libc::c_int, 0 as libc::c_int)
                        != -(1 as libc::c_int)
                    {
                        r = add_undo_redirect(redirector, ri, redir_fd);
                    } else {
                        r = add_undo_close_redirect(redirector);
                    }
                    REDIRECTION_ERROR! (r, errno, -1);
                }
                if flags & r_bash::RX_UNDOABLE as i32 != 0
                    && (ri as libc::c_uint == r_move_input as libc::c_int as libc::c_uint
                        || ri as libc::c_uint
                            == r_move_output as libc::c_int as libc::c_uint)
                {
                    if fcntl(redirector, 1 as libc::c_int, 0 as libc::c_int)
                        != -(1 as libc::c_int)
                    {
                        r = add_undo_redirect(
                            redir_fd,
                            r_close_this,
                            -(1 as libc::c_int),
                        );
                        REDIRECTION_ERROR! (r, errno, -1);
                    }
                }

                if redirector != 0 as libc::c_int
                    || subshell_environment & SUBSHELL_ASYNC == 0 as libc::c_int
                {
                    check_bash_input(redirector);
                }
                if (*redirect).rflags & REDIR_VARASSIGN != 0 {
                    r = redir_varassign(redirect, redirector);
                    if r < 0 as libc::c_int {
                        close(redirector);
                        return r;
                    }
                } else if dup2(redir_fd, redirector) < 0 as libc::c_int {
                    return *__errno_location()
                }
                if ri as libc::c_uint
                    == r_duplicating_input as libc::c_int as libc::c_uint
                    || ri as libc::c_uint == r_move_input as libc::c_int as libc::c_uint
                {
                    duplicate_buffered_stream(redir_fd, redirector);
                }
                if (fcntl(redir_fd, F_GETFD, 0 as libc::c_int)
                    == 1 as libc::c_int
                    || redir_fd < 2 as libc::c_int && flags & r_bash::RX_INTERNAL as libc::c_int != 0
                    || flags & RX_CLEXEC as libc::c_int != 0) && redirector > 2 as libc::c_int
                {
                    fcntl(redirector, 2 as libc::c_int, 1 as libc::c_int);
                }

                if (*redirect).flags & r_bash::RX_INTERNAL as i32 != 0
                    && (*redirect).flags & r_bash::RX_SAVCLEXEC as i32 != 0
                    && redirector >= 3 as libc::c_int
                    && (redir_fd >= SHELL_FD_BASE as i32 || (*redirect).flags & r_bash::RX_SAVEFD as i32!= 0)
                {
                    fcntl(redirector, 2 as libc::c_int, 0 as libc::c_int);
                }
                if ri as libc::c_uint == r_move_input as libc::c_int as libc::c_uint
                    || ri as libc::c_uint == r_move_output as libc::c_int as libc::c_uint
                {
                    xtrace_fdchk(redir_fd);
                    close(redir_fd);
                    coproc_fdchk(redir_fd);
                }
            }
        }

        r_close_this => {
            if flags & r_bash::RX_ACTIVE as libc::c_int != 0 {
                if (*redirect).rflags & REDIR_VARASSIGN as libc::c_int != 0 {
                    redirector = redir_varvalue(redirect);
                    if redirector < 0 as libc::c_int {
                        return AMBIGUOUS_REDIRECT;
                    }
                }
                r = 0 as libc::c_int;
                if flags & r_bash::RX_UNDOABLE as i32 != 0 {
                    if fcntl(redirector, 1 as libc::c_int, 0 as libc::c_int)
                        != -(1 as libc::c_int)
                    {
                        r = add_undo_redirect(redirector, ri, -(1 as libc::c_int));
                    } else {
                        r = add_undo_close_redirect(redirector);
                    }
                    REDIRECTION_ERROR!(r, errno, redirector);
                }
 
                coproc_fdchk(redirector);
                xtrace_fdchk(redirector);
                if redirector != 0 as libc::c_int
                    || subshell_environment & SUBSHELL_ASYNC == 0 as libc::c_int
                {
                    check_bash_input(redirector);
                }
                r = close_buffered_fd(redirector);
                if r < 0 as libc::c_int && flags & RX_INTERNAL != 0
                    && (*__errno_location() == EIO 
                        || *__errno_location() == ENOSPC)
                {
                    REDIRECTION_ERROR! (r, errno, -1);
                }
            }

        }

        r_duplicating_input_word | r_duplicating_output_word | r_move_input_word | r_move_output_word | _ => {

        }
    }
    
    return 0 as libc::c_int;
}
unsafe extern "C" fn add_undo_redirect(
    mut fd: libc::c_int,
    mut ri: r_instruction,
    mut fdbase: libc::c_int,
) -> libc::c_int {
    let mut new_fd: libc::c_int = 0;
    let mut clexec_flag: libc::c_int = 0;
    let mut savefd_flag: libc::c_int = 0;
    let mut new_redirect: *mut REDIRECT = 0 as *mut REDIRECT;
    let mut closer: *mut REDIRECT = 0 as *mut REDIRECT;
    let mut dummy_redirect: *mut REDIRECT = 0 as *mut REDIRECT;
    let mut sd: rcommon::REDIRECTEE = rcommon::REDIRECTEE { dest: 0 };
    savefd_flag = 0 as libc::c_int;
    new_fd = fcntl(
        fd,
        r_bash::F_DUPFD as i32,
        if fdbase < SHELL_FD_BASE {
            SHELL_FD_BASE
        } else {
            fdbase + 1 as libc::c_int
        },
    );
    if new_fd < 0 as libc::c_int {
        new_fd = fcntl(fd, 0 as libc::c_int, 10 as libc::c_int);
    }
    if new_fd < 0 as libc::c_int {
        new_fd = fcntl(fd, 0 as libc::c_int, 0 as libc::c_int);
        savefd_flag = 1 as libc::c_int;
    }
    if new_fd < 0 as libc::c_int {
        sys_error(
            dcgettext(
                0 as *const libc::c_char,
                b"redirection error: cannot duplicate fd\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return -(1 as libc::c_int);
    }
    clexec_flag = fcntl(fd, 1 as libc::c_int, 0 as libc::c_int);
    sd.dest = new_fd;
    rd.dest = 0 as libc::c_int;
    closer = make_redirection(sd, r_close_this, rd, 0 as libc::c_int);
    (*closer).flags |= RX_INTERNAL as libc::c_int;
    dummy_redirect = copy_redirects(closer);

    sd.dest = fd;
    rd.dest = new_fd;
    if fd == 0 as libc::c_int {
        new_redirect = make_redirection(sd, r_duplicating_input, rd, 0 as libc::c_int);
    } else {
        new_redirect = make_redirection(sd, r_duplicating_output, rd, 0 as libc::c_int);
    }
    (*new_redirect).flags |= RX_INTERNAL;
    if savefd_flag != 0 {
        (*new_redirect).flags |= 0x40 as libc::c_int;
    }
    if clexec_flag == 0 as libc::c_int && fd >= 3 as libc::c_int
        && (new_fd >= SHELL_FD_BASE as libc::c_int || savefd_flag != 0)
    {
        (*new_redirect).flags |= RX_SAVCLEXEC;
    }
    (*new_redirect).next = closer;
    (*closer).next = redirection_undo_list;
    redirection_undo_list = new_redirect;
    add_exec_redirect(dummy_redirect);
    if fd >= SHELL_FD_BASE as libc::c_int
        && ri as libc::c_uint != r_close_this as libc::c_int as libc::c_uint
        && clexec_flag != 0
    {
        sd.dest = fd;
        rd.dest = new_fd;
        new_redirect = make_redirection(sd, r_duplicating_output, rd, 0 as libc::c_int);
        (*new_redirect).flags |= 0x8 as libc::c_int;
        add_exec_redirect(new_redirect);
    }
    if clexec_flag != 0 || fd < 3 as libc::c_int {
        fcntl(new_fd, 2 as libc::c_int, 1 as libc::c_int);
    } else if (*redirection_undo_list).flags & RX_SAVCLEXEC as libc::c_int != 0 {
        fcntl(new_fd, 2 as libc::c_int, 1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn add_undo_close_redirect(mut fd: libc::c_int) -> libc::c_int {
    let mut closer: *mut REDIRECT = 0 as *mut REDIRECT;
    let mut sd: rcommon::REDIRECTEE = rcommon::REDIRECTEE{ dest: 0 };
    sd.dest = fd;
    rd.dest = 0 as libc::c_int;
    closer = make_redirection(sd, r_close_this, rd, 0 as libc::c_int);
    (*closer).flags |= RX_INTERNAL;
    (*closer).next = redirection_undo_list;
    redirection_undo_list = closer;
    return 0 as libc::c_int;
}
unsafe extern "C" fn add_exec_redirect(mut dummy_redirect: *mut REDIRECT) {
    (*dummy_redirect).next = exec_redirection_undo_list;
    exec_redirection_undo_list = dummy_redirect;
}
unsafe extern "C" fn stdin_redirection(
    mut ri: r_instruction,
    mut redirector: libc::c_int,
) -> libc::c_int {
    match ri as libc::c_uint {
        r_input_direction | r_inputa_direction | r_input_output | r_reading_until | r_deblank_reading_until | r_reading_string => return 1 as libc::c_int,
        r_duplicating_input | r_duplicating_input_word | r_close_this => return (redirector == 0 as libc::c_int) as libc::c_int,
        r_output_direction | r_appending_to | r_duplicating_output | r_err_and_out | r_append_err_and_out | r_output_force | r_duplicating_output_word | r_move_input | r_move_output | r_move_input_word | r_move_output_word => return 0 as libc::c_int,
        _ => {}
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stdin_redirects(mut redirs: *mut REDIRECT) -> libc::c_int {
    let mut rp: *mut REDIRECT = 0 as *mut REDIRECT;
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    rp = redirs;
    while !rp.is_null() {
        if (*rp).rflags & REDIR_VARASSIGN as libc::c_int == 0 as libc::c_int {
            n
                += stdin_redirection(
                    (*rp).instruction as libc::c_uint,
                    (*rp).redirector.dest,
                );
        }
        rp = (*rp).next;
    }
    return n;
}
unsafe extern "C" fn redir_varassign(
    mut redir: *mut REDIRECT,
    mut fd: libc::c_int,
) -> libc::c_int {
    let mut w: *mut WORD_DESC = 0 as *mut WORD_DESC;
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    w = (*redir).redirector.filename;
    v = bind_var_to_int((*w).word, fd as intmax_t);
    if v.is_null() || readonly_p!(v) as libc::c_int != 0
        || noassign_p!(v) as libc::c_int != 0
    {
        return BADVAR_REDIRECT;
    }
    stupidly_hack_special_variables((*w).word);
    return 0 as libc::c_int;
}

unsafe extern "C" fn redir_varvalue(mut redir: *mut REDIRECT) -> libc::c_int {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut w: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vmax: intmax_t = 0;
    let mut i: libc::c_int = 0;
    let mut sub: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut vr: libc::c_int = 0;
    w = (*(*redir).redirector.filename).word;
    vr = valid_array_reference(w, 0 as libc::c_int);
    if vr != 0 {
        v = array_variable_part(w, 0 as libc::c_int, &mut sub, &mut len);
    } else {
        v = find_variable(w);
        if v.is_null() {
            v = find_variable_last_nameref(w, 0 as libc::c_int);
            if !v.is_null() && (*v).attributes & 0x800 as libc::c_int != 0 {
                w = (*v).value;
                vr = valid_array_reference(w, 0 as libc::c_int);
                if vr != 0 {
                    v = array_variable_part(w, 0 as libc::c_int, &mut sub, &mut len);
                } else {
                    v = find_variable(w);
                }
            }
        }
    }
    if v.is_null() || invisible_p!(v) as libc::c_int != 0 {
        return -(1 as libc::c_int);
    }

    if vr != 0
        && (array_p! (v) as libc::c_int != 0
            || assoc_p!(v) as libc::c_int != 0)
    {
        val = get_array_value(
            w,
            0 as libc::c_int,
            0 as *mut libc::c_void as *mut libc::c_int,
            0 as *mut arrayind_t,
        );
    } else {
        val = get_variable_value(v);
    }
    if val.is_null() || *val as libc::c_int == 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if legal_number(val, &mut vmax) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    i = vmax as libc::c_int;
    return i;
}
