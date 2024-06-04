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
use r_bash::*;
extern "C" {
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn __fxstat(__ver: libc::c_int, __fildes: libc::c_int, __stat_buf: *mut stat) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn sh_unset_nodelay_mode(_: libc::c_int) -> libc::c_int;
    fn sys_error(_: *const libc::c_char, _: ...);
    fn internal_error(_: *const libc::c_char, _: ...);
    fn termsig_handler(_: libc::c_int);
    fn throw_to_top_level();
    static mut interrupt_state: sig_atomic_t;
    static mut terminating_signal: sig_atomic_t;
    fn return_EOF() -> libc::c_int;
    fn xbcopy(_: *mut libc::c_char, _: *mut libc::c_char, _: libc::c_int);
    fn zread(_: libc::c_int, _: *mut libc::c_char, _: size_t) -> ssize_t;
    static mut interactive_shell: libc::c_int;
    static mut default_buffered_input: libc::c_int;
    static mut bash_input: BASH_INPUT;
    fn init_yy_io(
        _: Option<sh_cget_func_t>,
        _: Option<sh_cunget_func_t>,
        _: stream_type,
        _: *const libc::c_char,
        _: INPUT_STREAM,
    );
    fn run_pending_traps();
}

pub const EAGAIN: libc::c_int = 11;
pub const X_EAGAIN: libc::c_int = EAGAIN;
pub const EWOULDBLOCK: libc::c_int = EAGAIN;
pub const X_EWOULDBLOCK: libc::c_int = EAGAIN;
pub const EINTR: libc::c_int = 4;
pub const B_UNBUFF: libc::c_int = 0x04;
pub const O_TEXT: libc::c_int = 0;
pub const F_GETFL: libc::c_int = 3;
pub const B_TEXT: libc::c_int = 0x10;
pub const SEEK_CUR: libc::c_int = 1;
pub const MAX_INPUT_BUFFER_SIZE: libc::c_int = 8172;
pub const O_RDONLY: libc::c_int = 0;
pub const EBADF: libc::c_int = 9;

pub type size_t = libc::c_ulong;
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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