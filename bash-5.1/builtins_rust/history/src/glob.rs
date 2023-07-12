//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.  

//# SPDX-License-Identifier: GPL-3.0-or-later
use std::{ffi::{CString, CStr}, io::Write};

use libc::{size_t, c_int, c_char, c_long, c_void, PT_NULL};
use rcommon::{r_builtin_usage,r_sh_erange,r_sh_restricted,r_sh_chkwrite,r_get_numeric_arg,WordList};
use rhelp::r_builtin_help;
include!(concat!("intercdep.rs"));
/* automatically generated by rust-bindgen */

pub const __EXTENSIONS__: u32 = 1;
pub const _ALL_SOURCE: u32 = 1;
pub const _GNU_SOURCE: u32 = 1;
pub const _POSIX_PTHREAD_SEMANTICS: u32 = 1;
pub const _TANDEM_SOURCE: u32 = 1;
pub const JOB_CONTROL: u32 = 1;
pub const ALIAS: u32 = 1;
pub const PUSHD_AND_POPD: u32 = 1;
pub const BRACE_EXPANSION: u32 = 1;
pub const READLINE: u32 = 1;
pub const BANG_HISTORY: u32 = 1;
pub const HISTORY: u32 = 1;
pub const HELP_BUILTIN: u32 = 1;
pub const RESTRICTED_SHELL: u32 = 1;
pub const PROCESS_SUBSTITUTION: u32 = 1;
pub const PROMPT_STRING_DECODE: u32 = 1;
pub const SELECT_COMMAND: u32 = 1;
pub const COMMAND_TIMING: u32 = 1;
pub const ARRAY_VARS: u32 = 1;
pub const DPAREN_ARITHMETIC: u32 = 1;
pub const EXTENDED_GLOB: u32 = 1;
pub const EXTGLOB_DEFAULT: u32 = 0;
pub const COND_COMMAND: u32 = 1;
pub const COND_REGEXP: u32 = 1;
pub const COPROCESS_SUPPORT: u32 = 1;
pub const ARITH_FOR_COMMAND: u32 = 1;
pub const NETWORK_REDIRECTIONS: u32 = 1;
pub const PROGRAMMABLE_COMPLETION: u32 = 1;
pub const DEBUGGER: u32 = 1;
pub const MEMSCRAMBLE: u32 = 1;
pub const CASEMOD_ATTRS: u32 = 1;
pub const CASEMOD_EXPANSIONS: u32 = 1;
pub const GLOBASCII_DEFAULT: u32 = 1;
pub const FUNCTION_IMPORT: u32 = 1;
pub const ENABLE_NLS: u32 = 1;
pub const DEFAULT_PATH_VALUE: &'static [u8; 63usize] =
    b"/usr/local/bin:/usr/local/sbin:/usr/bin:/usr/sbin:/bin:/sbin:.\0";
pub const STANDARD_UTILS_PATH: &'static [u8; 30usize] = b"/bin:/usr/bin:/usr/sbin:/sbin\0";
pub const PPROMPT: &'static [u8; 11usize] = b"\\s-\\\\v\\\\$ \0";
pub const SPROMPT: &'static [u8; 3usize] = b"> \0";
pub const DEFAULT_BASHRC: &'static [u8; 10usize] = b"~/.bashrc\0";
pub const SYS_BASH_LOGOUT: &'static [u8; 22usize] = b"/etc/bash.bash_logout\0";
pub const MULTIPLE_COPROCS: u32 = 0;
pub const CHECKWINSIZE_DEFAULT: u32 = 1;
pub const OPTIMIZE_SEQUENTIAL_ARRAY_ASSIGNMENT: u32 = 1;
pub const CHECKHASH_DEFAULT: u32 = 0;
pub const EVALNEST_MAX: u32 = 0;
pub const SOURCENEST_MAX: u32 = 0;
pub const OLDPWD_CHECK_DIRECTORY: u32 = 1;
pub const HISTEXPAND_DEFAULT: u32 = 1;
pub const ASSOC_KVPAIR_ASSIGNMENT: u32 = 1;
pub const HAVE_STRINGIZE: u32 = 1;
pub const HAVE_LONG_DOUBLE: u32 = 1;
pub const PROTOTYPES: u32 = 1;
pub const __PROTOTYPES: u32 = 1;
pub const HAVE_LONG_LONG: u32 = 1;
pub const HAVE_UNSIGNED_LONG_LONG: u32 = 1;
pub const SIZEOF_INT: u32 = 4;
pub const SIZEOF_LONG: u32 = 8;
pub const SIZEOF_CHAR_P: u32 = 8;
pub const SIZEOF_DOUBLE: u32 = 8;
pub const SIZEOF_INTMAX_T: u32 = 8;
pub const SIZEOF_LONG_LONG: u32 = 8;
pub const SIZEOF_WCHAR_T: u32 = 4;
pub const DEFAULT_MAIL_DIRECTORY: &'static [u8; 10usize] = b"/var/mail\0";
pub const STDC_HEADERS: u32 = 1;
pub const HAVE_ALLOCA: u32 = 1;
pub const HAVE_ALLOCA_H: u32 = 1;
pub const MAJOR_IN_SYSMACROS: u32 = 1;
pub const HAVE_MBSTATE_T: u32 = 1;
pub const HAVE_QUAD_T: u32 = 1;
pub const HAVE_WCHAR_T: u32 = 1;
pub const HAVE_WCTYPE_T: u32 = 1;
pub const HAVE_WINT_T: u32 = 1;
pub const HAVE_DECL_SYS_SIGLIST: u32 = 1;
pub const UNDER_SYS_SIGLIST_DECLARED: u32 = 1;
pub const HAVE_SYS_SIGLIST: u32 = 1;
pub const HAVE_UNDER_SYS_SIGLIST: u32 = 1;
pub const HAVE_SYS_ERRLIST: u32 = 1;
pub const HAVE_STRUCT_DIRENT_D_INO: u32 = 1;
pub const HAVE_STRUCT_DIRENT_D_FILENO: u32 = 1;
pub const FIONREAD_IN_SYS_IOCTL: u32 = 1;
pub const GWINSZ_IN_SYS_IOCTL: u32 = 1;
pub const STRUCT_WINSIZE_IN_SYS_IOCTL: u32 = 1;
pub const TERMIOS_LDISC: u32 = 1;
pub const TERMIO_LDISC: u32 = 1;
pub const HAVE_STRUCT_STAT_ST_BLOCKS: u32 = 1;
pub const HAVE_STRUCT_TM_TM_ZONE: u32 = 1;
pub const HAVE_TM_ZONE: u32 = 1;
pub const HAVE_TIMEVAL: u32 = 1;
pub const HAVE_STRUCT_TIMEZONE: u32 = 1;
pub const WEXITSTATUS_OFFSET: u32 = 8;
pub const HAVE_STRUCT_TIMESPEC: u32 = 1;
pub const TIME_H_DEFINES_STRUCT_TIMESPEC: u32 = 1;
pub const HAVE_STRUCT_STAT_ST_ATIM_TV_NSEC: u32 = 1;
pub const TYPEOF_STRUCT_STAT_ST_ATIM_IS_STRUCT_TIMESPEC: u32 = 1;
pub const HAVE_GETPW_DECLS: u32 = 1;
pub const HAVE_DECL_AUDIT_USER_TTY: u32 = 1;
pub const HAVE_DECL_CONFSTR: u32 = 1;
pub const HAVE_DECL_PRINTF: u32 = 1;
pub const HAVE_DECL_SBRK: u32 = 1;
pub const HAVE_DECL_STRCPY: u32 = 1;
pub const HAVE_DECL_STRSIGNAL: u32 = 1;
pub const HAVE_DECL_STRTOLD: u32 = 1;
pub const HAVE_DECL_STRTOIMAX: u32 = 1;
pub const HAVE_DECL_STRTOL: u32 = 1;
pub const HAVE_DECL_STRTOLL: u32 = 1;
pub const HAVE_DECL_STRTOUL: u32 = 1;
pub const HAVE_DECL_STRTOULL: u32 = 1;
pub const HAVE_DECL_STRTOUMAX: u32 = 1;
pub const GETPGRP_VOID: u32 = 1;
pub const PGRP_PIPE: u32 = 1;
pub const ULIMIT_MAXFDS: u32 = 1;
pub const CAN_REDEFINE_GETENV: u32 = 1;
pub const HAVE_STD_PUTENV: u32 = 1;
pub const HAVE_STD_UNSETENV: u32 = 1;
pub const HAVE_PRINTF_A_FORMAT: u32 = 1;
pub const HAVE_LANGINFO_CODESET: u32 = 1;
pub const HAVE_HASH_BANG_EXEC: u32 = 1;
pub const HAVE_DEV_FD: u32 = 1;
pub const DEV_FD_PREFIX: &'static [u8; 9usize] = b"/dev/fd/\0";
pub const HAVE_DEV_STDIN: u32 = 1;
pub const VOID_SIGHANDLER: u32 = 1;
pub const HAVE_POSIX_SIGNALS: u32 = 1;
pub const HAVE_ASPRINTF: u32 = 1;
pub const HAVE_BCOPY: u32 = 1;
pub const HAVE_BZERO: u32 = 1;
pub const HAVE_CHOWN: u32 = 1;
pub const HAVE_CONFSTR: u32 = 1;
pub const HAVE_DLCLOSE: u32 = 1;
pub const HAVE_DLOPEN: u32 = 1;
pub const HAVE_DLSYM: u32 = 1;
pub const HAVE_DPRINTF: u32 = 1;
pub const HAVE_DUP2: u32 = 1;
pub const HAVE_EACCESS: u32 = 1;
pub const HAVE_FACCESSAT: u32 = 1;
pub const AFLAG: c_int = 0x01;
pub const RFLAG: c_int = 0x02;
pub const WFLAG: c_int = 0x04;
pub const NFLAG: c_int = 0x08;
pub const SFLAG: c_int = 0x10;
pub const PFLAG: c_int = 0x20;
pub const CFLAG: c_int = 0x40;
pub const DFLAG: c_int = 0x80;

pub const HAVE_FCNTL: u32 = 1;
pub const HAVE_FNMATCH: u32 = 1;
pub const FNMATCH_EQUIV_FALLBACK: u32 = 1;
pub const HAVE___FPURGE: u32 = 1;
pub const HAVE_DECL_FPURGE: u32 = 0;
pub const HAVE_GETADDRINFO: u32 = 1;
pub const HAVE_GETCWD: u32 = 1;
pub const HAVE_GETENTROPY: u32 = 1;
pub const HAVE_GETDTABLESIZE: u32 = 1;
pub const HAVE_GETGROUPS: u32 = 1;
pub const HAVE_GETHOSTBYNAME: u32 = 1;
pub const HAVE_GETHOSTNAME: u32 = 1;
pub const HAVE_GETPAGESIZE: u32 = 1;
pub const HAVE_GETPEERNAME: u32 = 1;
pub const HAVE_GETPWENT: u32 = 1;
pub const HAVE_GETPWNAM: u32 = 1;
pub const HAVE_GETPWUID: u32 = 1;
pub const HAVE_GETRANDOM: u32 = 1;
pub const HAVE_GETRLIMIT: u32 = 1;
pub const HAVE_GETRUSAGE: u32 = 1;
pub const HAVE_GETSERVBYNAME: u32 = 1;
pub const HAVE_GETSERVENT: u32 = 1;
pub const HAVE_GETTIMEOFDAY: u32 = 1;
pub const HAVE_ICONV: u32 = 1;
pub const HAVE_IMAXDIV: u32 = 1;
pub const HAVE_INET_ATON: u32 = 1;
#[no_mangle]
pub extern "C" fn r_history_glob(mut list: *mut WordList) -> i32 {

    let mut flags: c_int = 0;
    let mut opt: c_int;
    let mut result: c_int;

    let filename: *mut c_char;
    let mut delete_arg: *mut c_char = PT_NULL as *mut c_char;
    let mut range: *mut c_char;

    let mut delete_offset: c_long = 0;

unsafe {
    reset_internal_getopt();
    let opt_str = CString::new("acd:npsrw").unwrap();
    opt = internal_getopt (list, opt_str.as_ptr() as * mut c_char);
    while  opt != -1 {
        let opt_char:char=char::from(opt as u8);
        match opt_char {
            'a' => flags |= AFLAG,
            'c' => flags |= CFLAG,
            'n' => flags |= NFLAG,
            'r' => flags |= RFLAG,
            'w' => flags |= WFLAG,
            's' => flags |= SFLAG,
            'd' => {
                flags |= DFLAG;
                delete_arg = list_optarg;
            }
            'p' => flags |= PFLAG,
            _ => {
                if opt == -99 {
                    r_builtin_help();
                    return EX_USAGE;
                }
            r_builtin_usage ();
            return EX_USAGE;
            }
        }
        opt = internal_getopt (list, opt_str.as_ptr() as * mut c_char);
    }
    list = loptend;

    opt = flags & (AFLAG | RFLAG | WFLAG | NFLAG);
    if opt != 0 && opt != AFLAG && opt != RFLAG && opt != WFLAG && opt != NFLAG {
        let c_err = CString::new("cannot use more than one of -anrw").unwrap();
        builtin_error( c_err.as_ptr());
        return EXECUTION_FAILURE;
    }

    if (flags & CFLAG) != 0 {
        bash_clear_history();
        if list.is_null() {
            return EXECUTION_SUCCESS;
        }
    }

    if (flags & SFLAG) != 0 {
        if !list.is_null() {
            push_history(list);
        }
        return EXECUTION_SUCCESS;
    }
    else if (flags & PFLAG) != 0 {
        if !list.is_null() {
            return expand_and_print_history(list);
        }
        return r_sh_chkwrite(EXECUTION_SUCCESS);
    } 
    else if (flags & DFLAG) != 0 {
        let c_tmp = if *delete_arg == b'-' as c_char {delete_arg.offset(1 as isize ) as *mut c_char} else {delete_arg};
        range = libc::strchr(c_tmp, b'-' as c_int);
        if  !range.is_null() {
            let mut delete_start: c_long = 0;
            let mut delete_end: c_long = 0;

        *range = b'\0' as c_char;
        range = (range as usize + 1) as *mut c_char;
        if legal_number(delete_arg, std::mem::transmute(&delete_start)) == 0 ||
        legal_number(range, std::mem::transmute(&delete_end)) == 0 {
            *((range as usize - 1) as *mut c_char) = b'-' as c_char;
            r_sh_erange(delete_arg, "history position\0".as_ptr() as *mut c_char);
            return EXECUTION_FAILURE;
        }
        if *delete_arg == b'-' as c_char && delete_start < 0 {
            delete_start += history_length as c_long;
            if delete_start < history_base as c_long {
                r_sh_erange(delete_arg, "history position\0".as_ptr() as *mut c_char);
                return EXECUTION_FAILURE;
            }
        } else if delete_start > 0 {
            delete_start -= history_base as c_long;
        }
        if delete_start < 0 || delete_start >= history_length as c_long {
            r_sh_erange(delete_arg, "history position\0".as_ptr() as *mut c_char);
            return EXECUTION_FAILURE;
        }
        if *range == b'-' as c_char && delete_end < 0 {
            delete_end += history_length as c_long;
            if delete_end < history_base as c_long {
                r_sh_erange(range, "history position\0".as_ptr() as *mut c_char);
                return EXECUTION_FAILURE;
            }
        } else if delete_end > 0 {
            delete_end -= history_base as c_long;
        }

        if delete_end < 0 || delete_end >= history_length as c_long {
            r_sh_erange(range, "history position\0".as_ptr() as *mut c_char);
            return EXECUTION_FAILURE;
        }
        result = bash_delete_history_range(delete_start as c_int, delete_end as c_int);
        if where_history() > history_length {
            history_set_pos(history_length);
        }

        return if result != 0 {EXECUTION_SUCCESS} else {EXECUTION_FAILURE};
        }
     else if (flags & DFLAG) != 0 {
        if legal_number(delete_arg, &mut delete_offset) == 0 {
            r_sh_erange(delete_arg, "history position\0".as_ptr() as *mut c_char);
            return EXECUTION_FAILURE;
        }

        if *delete_arg == b'-' as c_char && delete_offset < 0 {
            let ind = history_length + delete_offset as c_int;
            if ind < history_base {
                r_sh_erange(delete_arg, "history position\0".as_ptr() as *mut c_char);
                return EXECUTION_FAILURE;
            }
            opt = ind + history_base;
        } else if delete_offset < history_base as c_long ||
            (delete_offset >= (history_base + history_length) as c_long) {
            r_sh_erange(delete_arg, "history position\0".as_ptr() as *mut c_char);
            return EXECUTION_FAILURE;
        } else {
            opt = delete_offset as c_int;
        }
        result = bash_delete_histent(opt - history_base);
        if where_history() > history_length {
            history_set_pos(history_length);
        }
        return if result != 0 {EXECUTION_FAILURE} else {EXECUTION_SUCCESS};
    }
}
    else if (flags & (AFLAG | RFLAG | NFLAG | WFLAG | CFLAG)) == 0 {
        result = display_history(list);
        return r_sh_chkwrite(result);
    }

    filename = if !list.is_null() {(*((*list).word)).word} else {get_string_value("HISTFILE\0".as_ptr() as *mut c_char)};
    result = EXECUTION_SUCCESS;

    if restricted != 0 && !(libc::strchr(filename, b'/' as c_int).is_null()) {
        r_sh_restricted(filename);
        return EXECUTION_FAILURE;
    }
    if (flags & AFLAG) != 0 {
        result = maybe_append_history(filename);
    } else if (flags & WFLAG) != 0 {
        result = write_history(filename);
    } else if (flags & RFLAG) != 0{
        result = read_history(filename);
        history_lines_in_file = history_lines_read_from_file;
    } else if (flags & NFLAG) != 0{
        let old_history_lines = history_lines_in_file;
        let obase = history_base;

        using_history();
        result = read_history_range(filename, history_lines_in_file, -1);
        using_history();

        history_lines_in_file = history_lines_read_from_file;
        if force_append_history == 0 {
            history_lines_this_session +=
            history_lines_in_file - old_history_lines + history_base - obase;
        }
    }
}

    return if result != 0 {EXECUTION_FAILURE} else {EXECUTION_SUCCESS};
}

fn histtime(hlist: *mut HIST_ENTRY, histtimefmt: *const c_char) -> *mut c_char
{
unsafe {
    static mut timestr: [c_char;128] = [0;128];

    let t = history_get_time(hlist);
    let tm = if t != 0 {libc::localtime(&t)} else {PT_NULL as *mut libc::tm};
    if t != 0 && !tm.is_null() {
        strftime(std::mem::transmute(&timestr),
        std::mem::size_of_val(&timestr),
        histtimefmt,
        tm);
    } else if !(*hlist).timestamp.is_null() && (*(*hlist).timestamp) != 0 {
        let c_str = CString::new("%s: invalid timestamp").unwrap();
        libc::snprintf(std::mem::transmute(&timestr),
        std::mem::size_of_val(&timestr), c_str.as_ptr(),
        if *((*hlist).timestamp) == b'#' as c_char {((*hlist).timestamp as usize + 1) as *mut c_char} else {(*hlist).timestamp});
    } else {
        libc::strcpy(std::mem::transmute(&timestr), b"??\0".as_ptr() as *const c_char);
    }

    return timestr.as_mut_ptr();
}
}

unsafe fn quit()
{
    if terminating_signal != 0 {
        termsig_handler(terminating_signal);
    }
    if interrupt_state != 0 {
        throw_to_top_level();
    }
}

unsafe fn display_history(list: *mut WordList) -> c_int
{
    let mut limit:c_long = 0;
    let histtimefmt: *mut c_char;
    let mut timestr: *mut c_char;

    if !list.is_null() {
        if  r_get_numeric_arg(list, 0,&mut limit)== 0 {
            return EXECUTION_FAILURE;
        }
        if limit < 0 {
            limit = -limit;
        }
    } else {
        limit = -1;
    }
    let hlist = history_list();

    if !hlist.is_null() {
        let mut i: c_long = 0;
        while !(*hlist.offset(i as isize)).is_null() {
            i += 1;
        }

        i = if 0 <= limit && limit < i {i - limit} else {0};

        histtimefmt = get_string_value(b"HISTTIMEFORMAT\0" as *const u8 as *const c_char);

        while !(*hlist.offset(i as isize)).is_null() {
            if terminating_signal != 0 {
                termsig_handler(terminating_signal);
            }
            if interrupt_state != 0 {
                throw_to_top_level();
            }
            timestr = if !histtimefmt.is_null() && *histtimefmt as libc::c_int != 0 {
                histtime(*hlist.offset(i as isize), histtimefmt)
            } else {
                0 as *mut libc::c_void as *mut libc::c_char
            };
            printf(
                b"%5d%c %s%s\n\0" as *const u8 as *const libc::c_char,
                i + history_base as c_long,
                if !((**hlist.offset(i as isize)).data).is_null() {
                    '*' as i32
                } else {
                    ' ' as i32
                },
                if !timestr.is_null() && *timestr as libc::c_int != 0 {
                    timestr
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                (**hlist.offset(i as isize)).line,
            );
            i += 1;
        }
    }
    return EXECUTION_SUCCESS;
}

fn push_history(list: *mut WordList) {
unsafe {
    if remember_on_history != 0 && hist_last_line_pushed == 0 &&
        (hist_last_line_added != 0 || (current_command_line_count > 0 && current_command_first_line_saved != 0 && command_oriented_history != 0)) &&
        bash_delete_last_history() == 0 {
        return;
    }

    let s = string_list(list);
    check_add_history(s, 1);

    hist_last_line_pushed = 1;
    libc::free(s as *mut c_void);
}
}
pub type __locale_t = *mut __locale_struct;
pub type locale_t = __locale_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tm {
    _unused: [u8; 0],
}
extern "C" {
    pub fn wcscpy(__dest: *mut wchar_t, __src: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcsncpy(__dest: *mut wchar_t, __src: *const wchar_t, __n: usize) -> *mut wchar_t;
}
extern "C" {
    pub fn wcscat(__dest: *mut wchar_t, __src: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcsncat(__dest: *mut wchar_t, __src: *const wchar_t, __n: usize) -> *mut wchar_t;
}
extern "C" {
    pub fn wcscmp(__s1: *const wchar_t, __s2: *const wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcsncmp(__s1: *const wchar_t, __s2: *const wchar_t, __n: usize)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcscasecmp(__s1: *const wchar_t, __s2: *const wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcsncasecmp(
        __s1: *const wchar_t,
        __s2: *const wchar_t,
        __n: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcscasecmp_l(
        __s1: *const wchar_t,
        __s2: *const wchar_t,
        __loc: locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcsncasecmp_l(
        __s1: *const wchar_t,
        __s2: *const wchar_t,
        __n: usize,
        __loc: locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcscoll(__s1: *const wchar_t, __s2: *const wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcsxfrm(__s1: *mut wchar_t, __s2: *const wchar_t, __n: usize) -> usize;
}
extern "C" {
    pub fn wcscoll_l(
        __s1: *const wchar_t,
        __s2: *const wchar_t,
        __loc: locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcsxfrm_l(
        __s1: *mut wchar_t,
        __s2: *const wchar_t,
        __n: usize,
        __loc: locale_t,
    ) -> usize;
}
extern "C" {
    pub fn wcsdup(__s: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcschr(__wcs: *const wchar_t, __wc: wchar_t) -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcsrchr(__wcs: *const wchar_t, __wc: wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcschrnul(__s: *const wchar_t, __wc: wchar_t) -> *mut wchar_t;
}


fn expand_and_print_history(mut list: *mut WordList) -> c_int
{
unsafe {

    let s: *mut c_char = PT_NULL as *mut c_char;
    let mut result: c_int;

    if hist_last_line_pushed == 0 && hist_last_line_added != 0 && bash_delete_last_history() == 0 {
        return EXECUTION_FAILURE;
    }
    result = EXECUTION_SUCCESS;
    while !list.is_null() {
        let r = history_expand((*((*list).word)).word, std::mem::transmute(&s));
        if r < 0 {
            let c_err = CString::new("%s: history expansion failed").unwrap();
            builtin_error( c_err.as_ptr(), (*((*list).word)).word);
            result = EXECUTION_FAILURE;
        } else {
            println!("{}",CStr::from_ptr(s).to_str().unwrap());
            //println!("{}",String::from(CStr::from_ptr(s).to_str().unwrap()));
            //std::io::stdout().lock().write_all(CStr::from_ptr(s).to_bytes()).unwrap();
            //libc::putchar(b'\n' as c_int);
        }
        if !s.is_null() {
            libc::free(s as *mut c_void);
        }
        list = (*list).next;
    }
    std::io::stdout().lock().flush().unwrap();
    return result;
}
}
extern "C" {
    pub fn wcsspn(__wcs: *const wchar_t, __accept: *const wchar_t) -> usize;
}
extern "C" {
    pub fn wcspbrk(__wcs: *const wchar_t, __accept: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcsstr(__haystack: *const wchar_t, __needle: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcstok(
        __s: *mut wchar_t,
        __delim: *const wchar_t,
        __ptr: *mut *mut wchar_t,
    ) -> *mut wchar_t;
}
extern "C" {
    pub fn wcslen(__s: *const wchar_t) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn wcswcs(__haystack: *const wchar_t, __needle: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcsnlen(__s: *const wchar_t, __maxlen: usize) -> usize;
}
extern "C" {
    pub fn wmemchr(__s: *const wchar_t, __c: wchar_t, __n: usize) -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn wmemcmp(__s1: *const wchar_t, __s2: *const wchar_t, __n: usize)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wmemcpy(__s1: *mut wchar_t, __s2: *const wchar_t, __n: usize) -> *mut wchar_t;
}
extern "C" {
    pub fn wmemmove(__s1: *mut wchar_t, __s2: *const wchar_t, __n: usize) -> *mut wchar_t;
}
extern "C" {
    pub fn wmemset(__s: *mut wchar_t, __c: wchar_t, __n: usize) -> *mut wchar_t;
}
extern "C" {
    pub fn wmempcpy(__s1: *mut wchar_t, __s2: *const wchar_t, __n: usize) -> *mut wchar_t;
}
extern "C" {
    pub fn btowc(__c: ::std::os::raw::c_int) -> wint_t;
}
extern "C" {
    pub fn wctob(__c: wint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbsinit(__ps: *const mbstate_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbrtowc(
        __pwc: *mut wchar_t,
        __s: *const ::std::os::raw::c_char,
        __n: usize,
        __p: *mut mbstate_t,
    ) -> usize;
}
extern "C" {
    pub fn wcwidth(__c: wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcswidth(__s: *const wchar_t, __n: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcstod(__nptr: *const wchar_t, __endptr: *mut *mut wchar_t) -> f64;
}
extern "C" {
    pub fn wcstof(__nptr: *const wchar_t, __endptr: *mut *mut wchar_t) -> f32;
}
extern "C" {
    pub fn wcstold(__nptr: *const wchar_t, __endptr: *mut *mut wchar_t) -> f64;
}
extern "C" {
    pub fn wcstof32(__nptr: *const wchar_t, __endptr: *mut *mut wchar_t) -> _Float32;
}
extern "C" {
    pub fn wcstof64(__nptr: *const wchar_t, __endptr: *mut *mut wchar_t) -> _Float64;
}
extern "C" {
    pub fn wcstof32x(__nptr: *const wchar_t, __endptr: *mut *mut wchar_t) -> _Float32x;
}
extern "C" {
    pub fn wcstof64x(__nptr: *const wchar_t, __endptr: *mut *mut wchar_t) -> _Float64x;
}
extern "C" {
    pub fn wcstol_l(
        __nptr: *const wchar_t,
        __endptr: *mut *mut wchar_t,
        __base: ::std::os::raw::c_int,
        __loc: locale_t,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn wcstoul_l(
        __nptr: *const wchar_t,
        __endptr: *mut *mut wchar_t,
        __base: ::std::os::raw::c_int,
        __loc: locale_t,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn wcstoll_l(
        __nptr: *const wchar_t,
        __endptr: *mut *mut wchar_t,
        __base: ::std::os::raw::c_int,
        __loc: locale_t,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn wcstoull_l(
        __nptr: *const wchar_t,
        __endptr: *mut *mut wchar_t,
        __base: ::std::os::raw::c_int,
        __loc: locale_t,
    ) -> ::std::os::raw::c_ulonglong;
}
}
