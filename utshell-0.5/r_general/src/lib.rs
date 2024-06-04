//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use libc::FILE;
extern "C" {
    fn __strtol_internal(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
        __group: libc::c_int,
    ) -> libc::c_long;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn confstr(__name: libc::c_int, __buf: *mut libc::c_char, __len: size_t) -> size_t;
    fn getgroups(__size: libc::c_int, __list: *mut __gid_t) -> libc::c_int;
    fn ttyname(__fd: libc::c_int) -> *mut libc::c_char;
    fn getdtablesize() -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut sh_syntaxtab: [libc::c_int; 0];
    static mut interactive_shell: libc::c_int;
    static mut inherit_errexit: libc::c_int;
    static mut posixly_correct: libc::c_int;
    fn mbschr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn internal_error(_: *const libc::c_char, _: ...);
    fn valid_array_reference(_: *const libc::c_char, _: libc::c_int) -> libc::c_int;
    fn array_variable_name(
        _: *const libc::c_char,
        _: libc::c_int,
        _: *mut *mut libc::c_char,
        _: *mut libc::c_int,
    ) -> *mut libc::c_char;
    fn skipsubscript(_: *const libc::c_char, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn sh_eaccess(_: *const libc::c_char, _: libc::c_int) -> libc::c_int;
    fn sh_makepath(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    fn throw_to_top_level();
    static mut interrupt_state: sig_atomic_t;
    static mut terminating_signal: sig_atomic_t;
    fn termsig_handler(_: libc::c_int);
    fn get_string_value(_: *const libc::c_char) -> *mut libc::c_char;
    fn sh_single_quote(_: *const libc::c_char) -> *mut libc::c_char;
    fn sh_contains_shell_metas(_: *const libc::c_char) -> libc::c_int;
    fn ansic_quote(_: *mut libc::c_char, _: libc::c_int, _: *mut libc::c_int) -> *mut libc::c_char;
    fn ansic_shouldquote(_: *const libc::c_char) -> libc::c_int;
    fn substring(_: *const libc::c_char, _: libc::c_int, _: libc::c_int) -> *mut libc::c_char;
    fn strvec_create(_: libc::c_int) -> *mut *mut libc::c_char;
    fn itos(_: intmax_t) -> *mut libc::c_char;
    fn get_new_window_size(_: libc::c_int, _: *mut libc::c_int, _: *mut libc::c_int);
    static mut check_window_size: libc::c_int;
    fn getmaxgroups() -> libc::c_int;
    static mut current_user: user_info;
    static mut expand_aliases: libc::c_int;
    static mut interactive_comments: libc::c_int;
    fn get_dirstack_from_string(_: *mut libc::c_char) -> *mut libc::c_char;
    static mut print_shift_error: libc::c_int;
    static mut source_searches_cwd: libc::c_int;
    static mut source_uses_path: libc::c_int;
    static mut tilde_expansion_preexpansion_hook: Option<tilde_hook_func_t>;
    static mut tilde_additional_prefixes: *mut *mut libc::c_char;
    static mut tilde_additional_suffixes: *mut *mut libc::c_char;
    fn tilde_expand(_: *const libc::c_char) -> *mut libc::c_char;
}


pub const _ISalnum: libc::c_int = 8;
pub const _ISalpha: libc::c_int = 1024;
pub const _ISblank: libc::c_int = 1;
pub const _CS_PATH: libc::c_int = 0;
pub const W_HASDOLLAR: libc::c_int = 1 << 0;
pub const W_QUOTED: libc::c_int = 1 << 1;
pub const CBLANK: libc::c_int = 0x2000;
pub const CSHBRK: libc::c_int = 0x0002;
pub const CXQUOTE: libc::c_int = 0x0400;
pub const F_GETFL: libc::c_int = 3;
pub const F_SETFL: libc::c_int = 4;
pub const O_NONBLOCK: libc::c_int = 0o4000;
pub const O_NDELAY: libc::c_int = 0o4000;
pub const F_SETFD: libc::c_int = 2;
pub const FD_CLOEXEC: libc::c_int = 1;
pub const F_GETFD: libc::c_int = 1;
pub const SEEK_CUR: libc::c_int = 1;
pub const ESPIPE: libc::c_int = 29;
pub const O_RDWR: libc::c_int = 0o2;
pub const HIGH_FD_MAX: libc::c_int = 256;
pub const W_OK: libc::c_int = 2;
pub const MP_DOCWD: libc::c_int = 0x02;
pub const MP_RMDOT: libc::c_int = 0x04;
pub const PATH_MAX: libc::c_int = 4096;

pub type size_t = libc::c_ulong;
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub type __intmax_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __rlim_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type intmax_t = __intmax_t;
pub type sig_atomic_t = __sig_atomic_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct user_info {
    pub uid: uid_t,
    pub euid: uid_t,
    pub gid: gid_t,
    pub egid: gid_t,
    pub user_name: *mut libc::c_char,
    pub shell: *mut libc::c_char,
    pub home_dir: *mut libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct word_desc {
    pub word: *mut libc::c_char,
    pub flags: libc::c_int,
}
pub type WORD_DESC = word_desc;
pub type rlim_t = __rlim_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub posix_mode_var: *mut libc::c_int,
}
pub type tilde_hook_func_t = unsafe extern "C" fn(*mut libc::c_char) -> *mut libc::c_char;

#[macro_export]
macro_rules! whitespace {
    ($c:expr) => {
        ($c as libc::c_int == ' ' as i32 || $c as libc::c_int == '\t' as i32)
    };
}
#[macro_export]
macro_rules! DIGIT {
    ($c:expr) => {
        ($c as libc::c_int >= '0' as i32 && $c as libc::c_int <= '9' as i32)
    };
}
#[macro_export]
macro_rules! TODIGIT {
    ($c:expr) => {
        ($c as libc::c_int - '0' as i32)
    };
}
#[macro_export]
macro_rules! savestring {
    ($x:expr) => {
        strcpy(
            libc::malloc((strlen($x as *const libc::c_char) + 1) as usize) as *mut libc::c_char,
            $x,
        ) as *mut libc::c_char
    };
}
#[macro_export]
macro_rules! IN_CTYPE_DOMAIN {
    ($c:expr) => {
        1 as i32
    };
}
#[macro_export]
macro_rules! ISALPHA {
    ($c:expr) => {
        IN_CTYPE_DOMAIN!($c) != 0 && isalpha!($c) != 0 as libc::c_int
    };
}
#[macro_export]
macro_rules! isalpha {
    ($c:expr) => {
        __isctype_f!($c, _ISalpha)
    };
}
#[macro_export]
macro_rules! __isctype_f {
    ($c:expr,$type:expr) => {
        *(*__ctype_b_loc()).offset($c as libc::c_int as isize) as libc::c_int
            & ($type as libc::c_int as libc::c_ushort as libc::c_int)
    };
}
#[macro_export]
macro_rules! legal_variable_starter {
    ($c: expr) => {
        (ISALPHA!($c) || ($c as i32 == '_' as i32))
    };
}
#[macro_export]
macro_rules! legal_variable_char {
    ($c: expr) => {
        (ISALNUM!($c) || ($c as i32 == '_' as i32))
    };
}
#[macro_export]
macro_rules! ISALNUM {
    ($c:expr) => {
        IN_CTYPE_DOMAIN!($c) != 0 && isalnum!($c) != 0 as libc::c_int
    };
}
#[macro_export]
macro_rules! isalnum {
    ($c:expr) => {
        __isctype_f!($c, _ISalnum)
    };
}

#[macro_export]
macro_rules! STREQ {
    ($a:expr, $b:expr) => {
        (*$a.offset(0 as libc::c_int as isize) as libc::c_int
            == *$b.offset(0 as libc::c_int as isize) as libc::c_int
            && strcmp($a, $b) == 0 as libc::c_int)
    };
}

#[macro_export]
macro_rules! shellblank {
    ($c: expr) => {
        (*sh_syntaxtab.as_mut_ptr().offset($c) & CBLANK)
    };
}
#[macro_export]
macro_rules! shellbreak {
    ($c: expr) => {
        (*sh_syntaxtab.as_mut_ptr().offset($c) & CSHBRK)
    };
}
#[macro_export]
macro_rules! shellxquote {
    ($c: expr) => {
        (*sh_syntaxtab.as_mut_ptr().offset($c) & CXQUOTE)
    };
}
#[macro_export]
macro_rules! shellexp {
    ($c: expr) => {
        ($c == '$' as i32 || $c == '<' as i32  || $c == '>' as i32)
    };
}
#[macro_export]
macro_rules! isblank {
    ($c: expr) => {
        (__isctype_f!(($c), _ISblank))
    };
}
#[macro_export]
macro_rules! SET_CLOSE_ON_EXEC {
    ($fd: expr) => {
        (fcntl(($fd), F_SETFD, FD_CLOEXEC))
    };
}
#[macro_export]
macro_rules! S_ISDIR {
    ($mode:expr) => {
        $mode & 0o170000 as libc::c_uint == 0o40000 as libc::c_uint
    };
}
#[macro_export]
macro_rules! PATHSEP {
    ($c: expr) => {
        (ISDIRSEP!($c) || ($c) == 0)
    };
}
#[macro_export]
macro_rules! ISDIRSEP {
    ($c: expr) => {
        (($c) == '/' as i32)
    };
}
#[macro_export]
macro_rules! ABSPATH {
    ($c: expr) => {
        (*$c.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32)
    };
}
#[macro_export]
macro_rules! TILDE_END {
    ($c: expr) => {
        (($c) == '\0' as i32 || ($c) == '/' as i32 || ($c) == ':' as i32)
    };
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
#[macro_export]
macro_rules! STANDARD_UTILS_PATH {
    () => {
        (b"/bin:/usr/bin:/usr/sbin:/sbin\0" as *const u8 as *const libc::c_char)
    };
}

#[inline]
unsafe extern "C" fn strtoimax(
    mut nptr: *const libc::c_char,
    mut endptr: *mut *mut libc::c_char,
    mut base: libc::c_int,
) -> intmax_t {
    return __strtol_internal(nptr, endptr, base, 0 as libc::c_int);
}

#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}

#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}

/* A standard error message to use when getcwd() returns NULL. */
#[no_mangle]
pub static mut bash_getcwd_errstr: *const libc::c_char =
    b"getcwd: cannot access parent directories\0" as *const u8 as *const libc::c_char;

/* Do whatever is necessary to initialize `Posix mode'.  This currently
 modifies the following variables which are controlled via shopt:
    interactive_comments
    source_uses_path
    expand_aliases
    inherit_errexit
    print_shift_error
    posixglob

 and the following variables which cannot be user-modified:

    source_searches_cwd

If we add to the first list, we need to change the table and functions
below */
static mut posix_vars: [C2RustUnnamed_1; 6] = unsafe {
    [
        {
            let mut init = C2RustUnnamed_1 {
                posix_mode_var: &interactive_comments as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_1 {
                posix_mode_var: &source_uses_path as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_1 {
                posix_mode_var: &expand_aliases as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_1 {
                posix_mode_var: &inherit_errexit as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_1 {
                posix_mode_var: &print_shift_error as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_1 {
                posix_mode_var: 0 as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
    ]
};

static mut saved_posix_vars: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;

#[no_mangle]
pub unsafe extern "C" fn posix_initialize(mut on: libc::c_int) {
    /* Things that should be turned on when posix mode is enabled. */
    if on != 0 as libc::c_int {
        expand_aliases = 1 as libc::c_int;
        source_uses_path = expand_aliases;
        interactive_comments = source_uses_path;
        inherit_errexit = 1 as libc::c_int;
        source_searches_cwd = 0 as libc::c_int;
        print_shift_error = 1 as libc::c_int;

        /* Things that should be turned on when posix mode is disabled. */
    } else if !saved_posix_vars.is_null() {
        set_posix_options(saved_posix_vars);
        libc::free(saved_posix_vars as *mut libc::c_void);
        saved_posix_vars = 0 as *mut libc::c_char;
    } else {
        /* on == 0, restore a default set of settings */
        source_searches_cwd = 1 as libc::c_int;
        expand_aliases = interactive_shell;
        print_shift_error = 0 as libc::c_int;
    };
}

#[no_mangle]
pub unsafe extern "C" fn num_posix_options() -> libc::c_int {
    return (::core::mem::size_of::<[C2RustUnnamed_1; 6]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn get_posix_options(mut bitmap: *mut libc::c_char) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    if bitmap.is_null() {
        bitmap = libc::malloc(num_posix_options() as usize) as *mut libc::c_char;
        /* no trailing NULL */
    }
    i = 0 as libc::c_int;
    while !(posix_vars[i as usize].posix_mode_var).is_null() {
        *bitmap.offset(i as isize) = *posix_vars[i as usize].posix_mode_var as libc::c_char;
        i += 1;
        i;
    }
    return bitmap;
}

#[no_mangle]
pub unsafe extern "C" fn save_posix_options() {
    saved_posix_vars = get_posix_options(saved_posix_vars);
}

#[no_mangle]
pub unsafe extern "C" fn set_posix_options(mut bitmap: *const libc::c_char) {
    let mut i: libc::c_int = 0;

    i = 0 as libc::c_int;
    while !(posix_vars[i as usize].posix_mode_var).is_null() {
        *posix_vars[i as usize].posix_mode_var = *bitmap.offset(i as isize) as libc::c_int;
        i += 1;
        i;
    }
}

/* **************************************************************** */
/*								    */
/*  Functions to convert to and from and display non-standard types */
/*								    */
/* **************************************************************** */
#[no_mangle]
pub unsafe extern "C" fn string_to_rlimtype(mut s: *mut libc::c_char) -> rlim_t {
    let mut ret: rlim_t = 0 as rlim_t;
    let mut neg: libc::c_int = 0;

    while !s.is_null() && *s as libc::c_int != 0 && whitespace!(*s) {
        s = s.offset(1);
        s;
    }
    if !s.is_null() && (*s as libc::c_int == '-' as i32 || *s as libc::c_int == '+' as i32) {
        neg = (*s as libc::c_int == '-' as i32) as libc::c_int;
        s = s.offset(1);
        s;
    }
    while !s.is_null() && *s as libc::c_int != 0 && DIGIT!(*s) {
        ret = ret
            .wrapping_mul(10 as libc::c_int as libc::c_ulong)
            .wrapping_add((TODIGIT!(*s)) as libc::c_ulong);
        s = s.offset(1);
        s;
    }
    return if neg != 0 { ret.wrapping_neg() } else { ret };
}

#[no_mangle]
pub unsafe extern "C" fn print_rlimtype(mut n: rlim_t, mut addnl: libc::c_int) {
    let mut s: [libc::c_char; 21] = [0; 21];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;

    p = s
        .as_mut_ptr()
        .offset(::core::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong as isize);
    p = p.offset(-1);
    *p = '\0' as i32 as libc::c_char;

    if n < 0 as libc::c_int as libc::c_ulong {
        loop {
            p = p.offset(-1);
            *p = ('0' as i32 as libc::c_ulong)
                .wrapping_sub(n.wrapping_rem(10 as libc::c_int as libc::c_ulong))
                as libc::c_char;
            n = (n as libc::c_ulong).wrapping_div(10 as libc::c_int as libc::c_ulong) as rlim_t
                as rlim_t;
            if !(n != 0 as libc::c_int as libc::c_ulong) {
                break;
            }
        }
        p = p.offset(-1);
        *p = '-' as i32 as libc::c_char;
    } else {
        loop {
            p = p.offset(-1);
            *p = ('0' as i32 as libc::c_ulong)
                .wrapping_add(n.wrapping_rem(10 as libc::c_int as libc::c_ulong))
                as libc::c_char;
            n = (n as libc::c_ulong).wrapping_div(10 as libc::c_int as libc::c_ulong) as rlim_t
                as rlim_t;
            if !(n != 0 as libc::c_int as libc::c_ulong) {
                break;
            }
        }
    }

    printf(
        b"%s%s\0" as *const u8 as *const libc::c_char,
        p,
        if addnl != 0 {
            b"\n\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
}


/* **************************************************************** */
/*								    */
/*		       Input Validation Functions		    */
/*								    */
/* **************************************************************** */

/* Return non-zero if all of the characters in STRING are digits. */
#[no_mangle]
pub unsafe extern "C" fn all_digits(mut string: *const libc::c_char) -> libc::c_int {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;

    s = string;
    while *s != 0 {
        if DIGIT!(*s) as libc::c_int == 0 {
            return 0;
        }
        s = s.offset(1);
        s;
    }
    return 1;
}

/* Return non-zero if the characters pointed to by STRING constitute a
valid number.  Stuff the converted number into RESULT if RESULT is
not null. */
#[no_mangle]
pub unsafe extern "C" fn legal_number(
    mut string: *const libc::c_char,
    mut result: *mut intmax_t,
) -> libc::c_int {
    let mut value: intmax_t = 0;
    let mut ep: *mut libc::c_char = 0 as *mut libc::c_char;

    if !result.is_null() {
        *result = 0 as intmax_t;
    }

    if string.is_null() {
        return 0;
    }

    *__errno_location() = 0;
    value = strtoimax(string, &mut ep, 10);
    if *__errno_location() != 0 || ep == string as *mut libc::c_char {
        return 0; /* errno is set on overflow or underflow */
    }

    /* Skip any trailing whitespace, since strtoimax does not. */
    while *ep as libc::c_int == ' ' as i32 || *ep as libc::c_int == '\t' as i32 {
        ep = ep.offset(1);
        ep;
    }

    /* If *string is not '\0' but *ep is '\0' on return, the entire string
    is valid. */
    if *string as libc::c_int != 0 && *ep as libc::c_int == '\0' as i32 {
        if !result.is_null() {
            *result = value;
        }
        /* The SunOS4 implementation of strtol() will happily ignore
        overflow conditions, so this cannot do overflow correctly
        on those systems. */
        return 1;
    }

    return 0;
}

/* Return 1 if this token is a legal shell `identifier'; that is, it consists
solely of letters, digits, and underscores, and does not begin with a
digit. */
#[no_mangle]
pub unsafe extern "C" fn legal_identifier(mut name: *const libc::c_char) -> libc::c_int {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut c: libc::c_uchar = 0;

    if name.is_null()
        || {
            c = *name as libc::c_uchar;
            c == 0
        }
        || legal_variable_starter!(c) as libc::c_int == 0 as libc::c_int
    {
        return 0;
    }

    s = name.offset(1 as isize);
    loop {
        c = *s as libc::c_uchar;
        if !(c as libc::c_int != 0 as libc::c_int) {
            break;
        }
        if legal_variable_char!(c) as libc::c_int == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        s = s.offset(1);
        s;
    }
    return 1;
}

/* Return 1 if NAME is a valid value that can be assigned to a nameref
variable.  FLAGS can be 2, in which case the name is going to be used
to create a variable.  Other values are currently unused, but could
be used to allow values to be stored and indirectly referenced, but
not used in assignments. */
#[no_mangle]
pub unsafe extern "C" fn valid_nameref_value(
    mut name: *const libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    if name.is_null() || *name as libc::c_int == 0 as libc::c_int {
        return 0;
    }

    /* valid identifier */
    if legal_identifier(name) != 0
        || flags != 2 as libc::c_int && valid_array_reference(name, 0 as libc::c_int) != 0
    {
        return 1;
    }
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn check_selfref(
    mut name: *const libc::c_char,
    mut value: *mut libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;

    if STREQ!(name, value) {
        return 1;
    }

    if valid_array_reference(value, 0 as libc::c_int) != 0 {
        t = array_variable_name(
            value,
            0 as libc::c_int,
            0 as *mut libc::c_void as *mut *mut libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        if !t.is_null() && STREQ!(name, t) {
            libc::free(t as *mut libc::c_void);
            return 1;
        }
        libc::free(t as *mut libc::c_void);
    }

    return 0; /* not a self reference */
}

/* Make sure that WORD is a valid shell identifier, i.e.
does not contain a dollar sign, nor is quoted in any way.
If CHECK_WORD is non-zero,
the word is checked to ensure that it consists of only letters,
digits, and underscores, and does not consist of all digits. */
#[no_mangle]
pub unsafe extern "C" fn check_identifier(
    mut word: *mut WORD_DESC,
    mut check_word: libc::c_int,
) -> libc::c_int {
    /* XXX - HASDOLLAR? */
    if (*word).flags & (W_HASDOLLAR | W_QUOTED) != 0 {
        internal_error(
            dcgettext(
                0 as *const libc::c_char,
                b"`%s': not a valid identifier\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*word).word,
        );
        return 0;
    } else if check_word != 0
        && (all_digits((*word).word) != 0 || legal_identifier((*word).word) == 0 as libc::c_int)
    {
        internal_error(
            dcgettext(
                0 as *const libc::c_char,
                b"`%s': not a valid identifier\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*word).word,
        );
        return 0;
    } else {
        return 1;
    };
}

/* Return 1 if STRING is a function name that the shell will import from
the environment.  Currently we reject attempts to import shell functions
containing slashes, beginning with newlines or containing blanks.  In
Posix mode, we require that STRING be a valid shell identifier.  Not
used yet. */
#[no_mangle]
pub unsafe extern "C" fn importable_function_name(
    mut string: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    if absolute_program(string) != 0 {
        /* don't allow slash */
        return 0;
    }
    if *string as libc::c_int == '\n' as i32 {
        /* can't start with a newline */
        return 0;
    }
    if shellblank!(*string as libc::c_uchar as isize) != 0
        || shellblank!(len.wrapping_sub(1 as libc::c_ulong) as isize) != 0
    {
        return 0;
    }
    return if posixly_correct != 0 {
        legal_identifier(string)
    } else {
        1
    };
}

#[no_mangle]
pub unsafe extern "C" fn exportable_function_name(mut string: *const libc::c_char) -> libc::c_int {
    if absolute_program(string) != 0 {
        return 0;
    }
    if !(mbschr(string, '=' as i32)).is_null() {
        return 0;
    }
    return 1;
}

/* Return 1 if STRING comprises a valid alias name.  The shell accepts
essentially all characters except those which must be quoted to the
parser (which disqualifies them from alias expansion anyway) and `/'. */
#[no_mangle]
pub unsafe extern "C" fn legal_alias_name(
    mut string: *const libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    s = string;
    while *s != 0 {
        if shellbreak!(*s as libc::c_uchar as isize) != 0
            || shellxquote!(*s as libc::c_uchar as isize) != 0
            || shellexp!(*s as libc::c_int)
            || *s as libc::c_int == '/' as i32
        {
            return 0;
        }
        s = s.offset(1);
        s;
    }
    return 1;
}

/* Returns non-zero if STRING is an assignment statement.  The returned value
is the index of the `=' sign.  If FLAGS&1 we are expecting a compound assignment
and require an array subscript before the `=' to denote an assignment
statement. */
#[no_mangle]
pub unsafe extern "C" fn assignment(
    mut string: *const libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut c: libc::c_uchar = 0;
    let mut newi: libc::c_int = 0;
    let mut indx: libc::c_int = 0;

    indx = 0;
    c = *string.offset(indx as isize) as libc::c_uchar;

    /* If parser_state includes PST_COMPASSIGN, FLAGS will include 1, so we are
    parsing the contents of a compound assignment. If parser_state includes
    PST_REPARSE, we are in the middle of an assignment statement and breaking
    the words between the parens into words and assignment statements, but
    we don't need to check for that right now. Within a compound assignment,
    the subscript is required to make the word an assignment statement. If
    we don't have a subscript, even if the word is a valid assignment
    statement otherwise, we don't want to treat it as one. */
    if flags & 1 != 0 && c as libc::c_int != '[' as i32 {
        return 0;
    } else if flags & 1 == 0 && legal_variable_starter!(c) as libc::c_int == 0 as libc::c_int {
        return 0;
    }

    loop {
        c = *string.offset(indx as isize) as libc::c_uchar;
        if !(c != 0) {
            break;
        }
        /* The following is safe.  Note that '=' at the start of a word
        is not an assignment statement. */
        if c as libc::c_int == '=' as i32 {
            return indx;
        }

        if c as libc::c_int == '[' as i32 {
            newi = skipsubscript(string, indx, if flags & 2 != 0 { 1 } else { 0 });
            /* XXX - why not check for blank subscripts here, if we do in
            valid_array_reference? */
            let fresh0 = newi;
            newi = newi + 1;
            if *string.offset(fresh0 as isize) as libc::c_int != ']' as i32 {
                return 0 as libc::c_int;
            }
            if *string.offset(newi as isize) as libc::c_int == '+' as i32
                && *string.offset((newi + 1 as libc::c_int) as isize) as libc::c_int == '=' as i32
            {
                return newi + 1 as libc::c_int;
            }
            return if *string.offset(newi as isize) as libc::c_int == '=' as i32 {
                newi
            } else {
                0 as libc::c_int
            };
        }

        /* Check for `+=' */
        if c as libc::c_int == '+' as i32
            && *string.offset((indx + 1 as libc::c_int) as isize) as libc::c_int == '=' as i32
        {
            return indx + 1 as libc::c_int;
        }

        /* Variable names in assignment statements may contain only letters,
        digits, and `_'. */
        if legal_variable_char!(c) as libc::c_int == 0 as libc::c_int {
            return 0;
        }
        indx += 1;
        indx;
    }
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn line_isblank(mut line: *const libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;

    if line.is_null() {
        return 0; /* XXX */
    }

    i = 0;
    while *line.offset(i as isize) != 0 {
        if isblank!(*line.offset(i as isize) as libc::c_uchar as libc::c_int as isize)
            == 0 as libc::c_int
        {
            break;
        }
        i += 1;
        i;
    }
    return (*line.offset(i as isize) as libc::c_int == '\0' as i32) as libc::c_int;
}

/* Make sure no-delay mode is not set on file descriptor FD. */
#[no_mangle]
pub unsafe extern "C" fn sh_unset_nodelay_mode(mut fd: libc::c_int) -> libc::c_int {
    let mut flags: libc::c_int = 0;
    let mut bflags: libc::c_int = 0;

    flags = fcntl(fd, F_GETFL, 0 as libc::c_int);
    if flags < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }

    bflags = 0 as libc::c_int;

    /* This is defined to O_NDELAY in filecntl.h if O_NONBLOCK is not present
    and O_NDELAY is defined. */
    bflags |= O_NONBLOCK;
    bflags |= O_NDELAY;

    if flags & bflags != 0 {
        flags &= !bflags;
        return fcntl(fd, F_SETFL, flags);
    }

    return 0;
}

/* Just a wrapper for the define in include/filecntl.h */
#[no_mangle]
pub unsafe extern "C" fn sh_setclexec(mut fd: libc::c_int) -> libc::c_int {
    return SET_CLOSE_ON_EXEC!(fd);
}

/* Return 1 if file descriptor FD is valid; 0 otherwise. */
#[no_mangle]
pub unsafe extern "C" fn sh_validfd(mut fd: libc::c_int) -> libc::c_int {
    return (fcntl(fd, F_GETFD, 0) >= 0) as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fd_ispipe(mut fd: libc::c_int) -> libc::c_int {
    *__errno_location() = 0;
    return (lseek(fd, 0 as libc::c_long, SEEK_CUR) < 0 as libc::c_int as libc::c_long
        && *__errno_location() == ESPIPE) as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn check_dev_tty() {
    let mut tty_fd: libc::c_int = 0;
    let mut tty: *mut libc::c_char = 0 as *mut libc::c_char;

    tty_fd = open(
        b"/dev/tty\0" as *const u8 as *const libc::c_char,
        O_RDWR | O_NONBLOCK,
    );

    if tty_fd < 0 as libc::c_int {
        tty = ttyname(fileno(stdin));
        if tty.is_null() {
            return;
        }
        tty_fd = open(tty, O_RDWR | O_NONBLOCK);
    }
    if tty_fd >= 0 as libc::c_int {
        close(tty_fd);
    }
}

/* Return 1 if PATH1 and PATH2 are the same file.  This is kind of
expensive.  If non-NULL STP1 and STP2 point to stat structures
corresponding to PATH1 and PATH2, respectively. */
#[no_mangle]
pub unsafe extern "C" fn same_file(
    mut path1: *const libc::c_char,
    mut path2: *const libc::c_char,
    mut stp1: *mut stat,
    mut stp2: *mut stat,
) -> libc::c_int {
    let mut st1: stat = stat {
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
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut st2: stat = stat {
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
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };

    if stp1.is_null() {
        if stat(path1, &mut st1) != 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        stp1 = &mut st1;
    }

    if stp2.is_null() {
        if stat(path2, &mut st2) != 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        stp2 = &mut st2;
    }

    return ((*stp1).st_dev == (*stp2).st_dev && (*stp1).st_ino == (*stp2).st_ino) as libc::c_int;
}

/* Move FD to a number close to the maximum number of file descriptors
allowed in the shell process, to avoid the user stepping on it with
redirection and causing us extra work.  If CHECK_NEW is non-zero,
we check whether or not the file descriptors are in use before
duplicating FD onto them.  MAXFD says where to start checking the
file descriptors.  If it's less than 20, we get the maximum value
available from getdtablesize(2). */
#[no_mangle]
pub unsafe extern "C" fn move_to_high_fd(
    mut fd: libc::c_int,
    mut check_new: libc::c_int,
    mut maxfd: libc::c_int,
) -> libc::c_int {
    let mut script_fd: libc::c_int = 0;
    let mut nfds: libc::c_int = 0;
    let mut ignore: libc::c_int = 0;

    if maxfd < 20 as libc::c_int {
        nfds = getdtablesize();
        if nfds <= 0 as libc::c_int {
            nfds = 20 as libc::c_int;
        }
        if nfds > HIGH_FD_MAX {
            nfds = HIGH_FD_MAX; /* reasonable maximum */
        }
    } else {
        nfds = maxfd;
    }

    nfds -= 1;
    nfds;
    while check_new != 0 && nfds > 3 as libc::c_int {
        if fcntl(nfds, F_GETFD, &mut ignore as *mut libc::c_int) == -(1 as libc::c_int) {
            break;
        }
        nfds -= 1;
        nfds;
    }

    if nfds > 3 as libc::c_int && fd != nfds && {
        script_fd = dup2(fd, nfds);
        script_fd != -(1 as libc::c_int)
    } {
        /* don't close stderr */
        if check_new == 0 as libc::c_int || fd != fileno(stderr) {
            close(fd);
        }
        return script_fd;
    }

    /* OK, we didn't find one less than our artificial maximum; return the
    original file descriptor. */
    return fd;
}

/* Return non-zero if the characters from SAMPLE are not all valid
characters to be found in the first line of a shell script.  We
check up to the first newline, or SAMPLE_LEN, whichever comes first.
All of the characters must be printable or whitespace. */
#[no_mangle]
pub unsafe extern "C" fn check_binary_file(
    mut sample: *const libc::c_char,
    mut sample_len: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_uchar = 0;

    while i < sample_len {
        c = *sample.offset(i as isize) as libc::c_uchar;
        if c as libc::c_int == '\n' as i32 {
            return 0 as libc::c_int;
        }
        if c as libc::c_int == '\0' as i32 {
            return 1 as libc::c_int;
        }
        i += 1;
        i;
    }

    return 0 as libc::c_int;
}

/* **************************************************************** */
/*								    */
/*		    Functions to manipulate pipes		    */
/*								    */
/* **************************************************************** */
#[no_mangle]
pub unsafe extern "C" fn sh_openpipe(mut pv: *mut libc::c_int) -> libc::c_int {
    let mut r: libc::c_int = 0;

    r = pipe(pv);
    if r < 0 {
        return r;
    }

    *pv.offset(0 as isize) = move_to_high_fd(*pv.offset(0 as isize), 1, 64);
    *pv.offset(1 as isize) = move_to_high_fd(*pv.offset(1 as isize), 1, 64);

    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn sh_closepipe(mut pv: *mut libc::c_int) -> libc::c_int {
    if *pv.offset(0 as isize) >= 0 as libc::c_int {
        close(*pv.offset(0 as isize));
    }

    if *pv.offset(1 as isize) >= 0 as libc::c_int {
        close(*pv.offset(1 as isize));
    }

    let ref mut fresh1 = *pv.offset(1 as isize);
    *fresh1 = -(1 as libc::c_int);
    *pv.offset(0 as isize) = *fresh1;
    return 0;
}

/* **************************************************************** */
/*								    */
/*		    Functions to inspect pathnames		    */
/*								    */
/* **************************************************************** */
#[no_mangle]
pub unsafe extern "C" fn file_exists(mut fn_0: *const libc::c_char) -> libc::c_int {
    let mut sb: stat = stat {
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
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };

    return (stat(fn_0, &mut sb) == 0 as libc::c_int) as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn file_isdir(mut fn_0: *const libc::c_char) -> libc::c_int {
    let mut sb: stat = stat {
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
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };

    return (stat(fn_0, &mut sb) == 0 && S_ISDIR!(sb.st_mode)) as libc::c_int;
}