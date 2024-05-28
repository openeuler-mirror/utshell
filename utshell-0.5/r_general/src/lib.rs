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