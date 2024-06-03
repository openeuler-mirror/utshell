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