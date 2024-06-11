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
pub type sig_atomic_t = __sig_atomic_t;
pub type sh_cget_func_t = unsafe extern "C" fn() -> libc::c_int;
pub type sh_cunget_func_t = unsafe extern "C" fn(libc::c_int) -> libc::c_int;
pub type stream_type = libc::c_uint;
pub const st_bstream: stream_type = 4;
pub const st_string: stream_type = 3;
pub const st_stream: stream_type = 2;
pub const st_stdin: stream_type = 1;
pub const st_none: stream_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BSTREAM {
    pub b_fd: libc::c_int,
    pub b_buffer: *mut libc::c_char,
    pub b_size: size_t,
    pub b_used: size_t,
    pub b_flag: libc::c_int,
    pub b_inputp: size_t,
}
pub type BUFFERED_STREAM = BSTREAM;
#[derive(Copy, Clone)]
#[repr(C)]
pub union INPUT_STREAM {
    pub file: *mut FILE,
    pub string: *mut libc::c_char,
    pub buffered_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BASH_INPUT {
    pub type_0: stream_type,
    pub name: *mut libc::c_char,
    pub location: INPUT_STREAM,
    pub getter: Option<sh_cget_func_t>,
    pub ungetter: Option<sh_cunget_func_t>,
}

#[macro_export]
macro_rules! CHECK_TERMSIG {
    () => {
        if terminating_signal != 0 {
            termsig_handler(terminating_signal);
        }
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
macro_rules! ALLOCATE_BUFFERS {
    ($n:expr) => {
        if $n >= nbuffers {
            allocate_buffers($n);
        }
    };
}
#[macro_export]
macro_rules! SET_CLOSE_ON_EXEC {
    ($fd:expr) => {
        fcntl($fd, 2, 1)
    };
}
#[macro_export]
macro_rules! max {
    ($a: expr, $b: expr) => {
        if $a > $b {
            $a
        } else {
            $b
        }
    };
}

#[macro_export]
macro_rules! min {
    ($a: expr, $b: expr) => {
        if $a > $b {
            $b
        } else {
            $a
        }
    };
}
#[macro_export]
macro_rules! fd_is_seekable {
    ($fd: expr) => {
        lseek($fd, 0 as libc::c_long, SEEK_CUR) >= 0 as libc::c_long
    };
}
#[macro_export]
macro_rules! bufstream_getc {
    ($bp: expr) => {
        if ($bp).b_inputp == ($bp).b_used || ($bp).b_used == 0 {
            b_fill_buffer(*buffers.offset(bash_input.location.buffered_fd as isize))
        } else {
            let ref mut fresh10 =
                (**buffers.offset(bash_input.location.buffered_fd as isize)).b_inputp;
            let fresh11 = *fresh10;
            *fresh10 = (*fresh10).wrapping_add(1);
            *(($bp).b_buffer).offset(fresh11 as isize) as libc::c_int & 0xff as libc::c_int
        }
    };
}


#[inline]
unsafe extern "C" fn fstat(mut __fd: libc::c_int, mut __statbuf: *mut stat) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}

/* Functions to handle reading input on systems that don't restart read(2)
if a signal is received. */

static mut localbuf: [libc::c_char; 1024] = [0; 1024];
static mut local_index: libc::c_int = 0 as libc::c_int;
static mut local_bufused: libc::c_int = 0 as libc::c_int;

/* Posix and USG systems do not guarantee to restart read () if it is
interrupted by a signal.  We do the read ourselves, and restart it
if it returns EINTR. */
#[no_mangle]
pub unsafe extern "C" fn getc_with_restart(mut stream: *mut FILE) -> libc::c_int {
    let mut uc: libc::c_uchar = 0;

    CHECK_TERMSIG!();

    /* Try local buffering to reduce the number of read(2) calls. */
    if local_index == local_bufused || local_bufused == 0 as libc::c_int {
        loop {
            QUIT!();
            run_pending_traps();

            local_bufused = read(
                fileno(stream),
                localbuf.as_mut_ptr() as *mut libc::c_void,
                ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            ) as libc::c_int;
            if local_bufused > 0 as libc::c_int {
                break;
            }
            if local_bufused == 0 as libc::c_int {
                local_index = 0 as libc::c_int;
                return EOF;
            } else if *__errno_location() == X_EAGAIN || *__errno_location() == X_EWOULDBLOCK {
                if sh_unset_nodelay_mode(fileno(stream)) < 0 as libc::c_int {
                    sys_error(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"cannot reset nodelay mode for fd %d\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        fileno(stream),
                    );
                    local_bufused = 0 as libc::c_int;
                    local_index = local_bufused;
                    return EOF;
                }
            } else if *__errno_location() != EINTR {
                local_bufused = 0 as libc::c_int;
                local_index = local_bufused;
                return EOF;
            } else if interrupt_state != 0 || terminating_signal != 0 {
                /* QUIT; */
                local_bufused = 0 as libc::c_int;
                local_index = local_bufused;
            }
        }
        local_index = 0 as libc::c_int;
    }
    let fresh0 = local_index;
    local_index = local_index + 1;
    uc = localbuf[fresh0 as usize] as libc::c_uchar;
    return uc as libc::c_int;
}


#[no_mangle]
pub unsafe extern "C" fn ungetc_with_restart(
    mut c: libc::c_int,
    mut stream: *mut FILE,
) -> libc::c_int {
    if local_index == 0 as libc::c_int || c == EOF {
        return EOF;
    }
    local_index -= 1;
    localbuf[local_index as usize] = c as libc::c_char;
    return c;
}

#[no_mangle]
pub static mut bash_input_fd_changed: libc::c_int = 0;

/* This provides a way to map from a file descriptor to the buffer
associated with that file descriptor, rather than just the other
way around.  This is needed so that buffers are managed properly
in constructs like 3<&4.  buffers[x]->b_fd == x -- that is how the
correspondence is maintained. */
static mut buffers: *mut *mut BUFFERED_STREAM =
    0 as *const libc::c_void as *mut libc::c_void as *mut *mut BUFFERED_STREAM;
static mut nbuffers: libc::c_int = 0;

/* Make sure `buffers' has at least N elements. */
unsafe extern "C" fn allocate_buffers(mut n: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut orig_nbuffers: libc::c_int = 0;

    orig_nbuffers = nbuffers;
    nbuffers = n + 20 as libc::c_int;
    buffers = libc::realloc(
        buffers as *mut libc::c_void,
        (nbuffers as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut BUFFERED_STREAM>() as libc::c_ulong)
            as usize,
    ) as *mut *mut BUFFERED_STREAM;

    /* Zero out the new buffers. */
    i = orig_nbuffers;
    while i < nbuffers {
        let ref mut fresh1 = *buffers.offset(i as isize);
        *fresh1 = 0 as *mut libc::c_void as *mut BUFFERED_STREAM;
        i += 1;
        i;
    }
}

/* Construct and return a BUFFERED_STREAM corresponding to file descriptor
FD, using BUFFER. */
unsafe extern "C" fn make_buffered_stream(
    mut fd: libc::c_int,
    mut buffer: *mut libc::c_char,
    mut bufsize: size_t,
) -> *mut BUFFERED_STREAM {
    let mut bp: *mut BUFFERED_STREAM = 0 as *mut BUFFERED_STREAM;
    bp = libc::malloc(::core::mem::size_of::<BUFFERED_STREAM>() as libc::c_ulong as usize)
        as *mut BUFFERED_STREAM;
    ALLOCATE_BUFFERS!(fd);
    let ref mut fresh2 = *buffers.offset(fd as isize);
    *fresh2 = bp;
    (*bp).b_fd = fd;
    (*bp).b_buffer = buffer;
    (*bp).b_size = bufsize;
    (*bp).b_flag = 0 as libc::c_int;
    (*bp).b_inputp = (*bp).b_flag as size_t;
    (*bp).b_used = (*bp).b_inputp;
    if bufsize == 1 as libc::c_int as libc::c_ulong {
        (*bp).b_flag |= B_UNBUFF;
    }
    if O_TEXT != 0 && fcntl(fd, F_GETFL) & O_TEXT != 0 as libc::c_int {
        (*bp).b_flag |= B_TEXT;
    }
    return bp;
}

/* Allocate a new BUFFERED_STREAM, copy BP to it, and return the new copy. */
unsafe extern "C" fn copy_buffered_stream(mut bp: *mut BUFFERED_STREAM) -> *mut BUFFERED_STREAM {
    let mut nbp: *mut BUFFERED_STREAM = 0 as *mut BUFFERED_STREAM;

    if bp.is_null() {
        return 0 as *mut libc::c_void as *mut BUFFERED_STREAM;
    }

    nbp = libc::malloc(::core::mem::size_of::<BUFFERED_STREAM>() as libc::c_ulong as usize)
        as *mut BUFFERED_STREAM;
    xbcopy(
        bp as *mut libc::c_char,
        nbp as *mut libc::c_char,
        ::core::mem::size_of::<BUFFERED_STREAM>() as libc::c_ulong as libc::c_int,
    );
    return nbp;
}

#[no_mangle]
pub unsafe extern "C" fn set_bash_input_fd(mut fd: libc::c_int) -> libc::c_int {
    if bash_input.type_0 as libc::c_uint == st_bstream as libc::c_int as libc::c_uint {
        bash_input.location.buffered_fd = fd;
    } else if interactive_shell == 0 as libc::c_int {
        default_buffered_input = fd;
    }
    return 0 as libc::c_int;
}


#[no_mangle]
pub unsafe extern "C" fn fd_is_bash_input(mut fd: libc::c_int) -> libc::c_int {
    if bash_input.type_0 as libc::c_uint == st_bstream as libc::c_int as libc::c_uint
        && bash_input.location.buffered_fd == fd
    {
        return 1 as libc::c_int;
    } else if interactive_shell == 0 as libc::c_int && default_buffered_input == fd {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}

/* Save the buffered stream corresponding to file descriptor FD (which bash
is using to read input) to a buffered stream associated with NEW_FD.  If
NEW_FD is -1, a new file descriptor is allocated with fcntl.  The new
file descriptor is returned on success, -1 on error. */
#[no_mangle]
pub unsafe extern "C" fn save_bash_input(
    mut fd: libc::c_int,
    mut new_fd: libc::c_int,
) -> libc::c_int {
    let mut nfd: libc::c_int = 0;

    /* Sync the stream so we can re-read from the new file descriptor.  We
    might be able to avoid this by copying the buffered stream verbatim
    to the new file descriptor. */
    if !(*buffers.offset(fd as isize)).is_null() {
        sync_buffered_stream(fd);
    }

    /* Now take care of duplicating the file descriptor that bash is
    using for input, so we can reinitialize it later. */
    nfd = if new_fd == -(1 as libc::c_int) {
        fcntl(fd, 0 as libc::c_int, 10 as libc::c_int)
    } else {
        new_fd
    };
    if nfd == -(1 as libc::c_int) {
        if fcntl(fd, 1 as libc::c_int, 0 as libc::c_int) == 0 as libc::c_int {
            sys_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot allocate new file descriptor for bash input from fd %d\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                fd,
            );
        }
        return -(1 as libc::c_int);
    }

    if nfd < nbuffers && !(*buffers.offset(nfd as isize)).is_null() {
        /* What's this?  A stray buffer without an associated open file
        descriptor?  Free up the buffer and report the error. */
        internal_error(
            dcgettext(
                0 as *const libc::c_char,
                b"save_bash_input: buffer already exists for new fd %d\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            nfd,
        );
        if (**buffers.offset(nfd as isize)).b_flag & B_SHAREDBUF as libc::c_int != 0 {
            let ref mut fresh3 = (**buffers.offset(nfd as isize)).b_buffer;
            *fresh3 = 0 as *mut libc::c_void as *mut libc::c_char;
        }
        free_buffered_stream(*buffers.offset(nfd as isize));
    }

    /* Reinitialize bash_input.location. */
    if bash_input.type_0 as libc::c_uint == st_bstream as libc::c_int as libc::c_uint {
        bash_input.location.buffered_fd = nfd;
        fd_to_buffered_stream(nfd);
        close_buffered_fd(fd); /* XXX */
    } else {
        /* If the current input type is not a buffered stream, but the shell
        is not interactive and therefore using a buffered stream to read
        input (e.g. with an `eval exec 3>output' inside a script), note
        that the input fd has been changed.  pop_stream() looks at this
        value and adjusts the input fd to the new value of
        default_buffered_input accordingly. */
        bash_input_fd_changed += 1;
        bash_input_fd_changed;
    }
    if default_buffered_input == fd {
        default_buffered_input = nfd;
    }

    SET_CLOSE_ON_EXEC!(nfd);
    return nfd;
}