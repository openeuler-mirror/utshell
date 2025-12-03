//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later
use crate::general::sh_unset_nodelay_mode;
use crate::src_common::*;
use crate::stringlib::xbcopy;
use crate::trap::run_pending_traps;
use crate::y_tab::{bash_input, init_yy_io, return_EOF};

/* Functions to handle reading input on systems that don't restart read(2)
if a signal is received. */

static mut localbuf: [libc::c_char; 1024] = [0; 1024];
static mut local_index: libc::c_int = 0 as libc::c_int;
static mut local_bufused: libc::c_int = 0 as libc::c_int;

/* Posix and USG systems do not guarantee to restart read () if it is
interrupted by a signal.  We do the read ourselves, and restart it
if it returns EINTR. */
#[no_mangle]
pub fn getc_with_restart(stream: *mut FILE) -> libc::c_int {
    unsafe {
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
                    ::core::mem::size_of::<[libc::c_char; 1024]>() as usize,
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
}

#[no_mangle]
pub fn ungetc_with_restart(c: libc::c_int, stream: *mut FILE) -> libc::c_int {
    unsafe {
        if local_index == 0 as libc::c_int || c == EOF {
            return EOF;
        }
        local_index -= 1;
        localbuf[local_index as usize] = c as libc::c_char;
        return c;
    }
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
fn allocate_buffers(n: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut orig_nbuffers: libc::c_int = 0;
    unsafe {
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
}

/* Construct and return a BUFFERED_STREAM corresponding to file descriptor
FD, using BUFFER. */
fn make_buffered_stream(
    fd: libc::c_int,
    buffer: *mut libc::c_char,
    bufsize: size_t,
) -> *mut BUFFERED_STREAM {
    unsafe {
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
}

/* Allocate a new BUFFERED_STREAM, copy BP to it, and return the new copy. */
fn copy_buffered_stream(bp: *mut BUFFERED_STREAM) -> *mut BUFFERED_STREAM {
    unsafe {
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
}

#[no_mangle]
pub fn set_bash_input_fd(fd: libc::c_int) -> libc::c_int {
    unsafe {
        if bash_input.type_0 as libc::c_uint == st_bstream as libc::c_int as libc::c_uint {
            bash_input.location.buffered_fd = fd;
        } else if interactive_shell == 0 as libc::c_int {
            default_buffered_input = fd;
        }
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub fn fd_is_bash_input(fd: libc::c_int) -> libc::c_int {
    unsafe {
        if bash_input.type_0 as libc::c_uint == st_bstream as libc::c_int as libc::c_uint
            && bash_input.location.buffered_fd == fd
        {
            return 1 as libc::c_int;
        } else if interactive_shell == 0 as libc::c_int && default_buffered_input == fd {
            return 1 as libc::c_int;
        }
        return 0 as libc::c_int;
    }
}

/* Save the buffered stream corresponding to file descriptor FD (which bash
is using to read input) to a buffered stream associated with NEW_FD.  If
NEW_FD is -1, a new file descriptor is allocated with fcntl.  The new
file descriptor is returned on success, -1 on error. */
#[no_mangle]
pub fn save_bash_input(fd: libc::c_int, new_fd: libc::c_int) -> libc::c_int {
    unsafe {
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
                        b"cannot allocate new file descriptor for bash input from fd %d\0"
                            as *const u8 as *const libc::c_char,
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
}

/* Check that file descriptor FD is not the one that bash is currently
using to read input from a script.  FD is about to be duplicated onto,
which means that the kernel will close it for us.  If FD is the bash
input file descriptor, we need to seek backwards in the script (if
possible and necessary -- scripts read from stdin are still unbuffered),
allocate a new file descriptor to use for bash input, and re-initialize
the buffered stream.  Make sure the file descriptor used to save bash
input is set close-on-exec. Returns 0 on success, -1 on failure.  This
works only if fd is > 0 -- if fd == 0 and bash is reading input from
fd 0, sync_buffered_stream is used instead, to cooperate with input
redirection (look at redir.c:add_undo_redirect()). */
#[no_mangle]
pub fn check_bash_input(fd: libc::c_int) -> libc::c_int {
    if fd_is_bash_input(fd) != 0 {
        if fd > 0 as libc::c_int {
            return if save_bash_input(fd, -(1 as libc::c_int)) == -(1 as libc::c_int) {
                -(1 as libc::c_int)
            } else {
                0 as libc::c_int
            };
        } else if fd == 0 as libc::c_int {
            return if sync_buffered_stream(fd) == -(1 as libc::c_int) {
                -(1 as libc::c_int)
            } else {
                0 as libc::c_int
            };
        }
    }
    return 0 as libc::c_int;
}

/* This is the buffered stream analogue of dup2(fd1, fd2).  The
BUFFERED_STREAM corresponding to fd2 is deallocated, if one exists.
BUFFERS[fd1] is copied to BUFFERS[fd2].  This is called by the
redirect code for constructs like 4<&0 and 3</etc/rc.local. */
#[no_mangle]
pub fn duplicate_buffered_stream(fd1: libc::c_int, fd2: libc::c_int) -> libc::c_int {
    let mut is_bash_input: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    unsafe {
        if fd1 == fd2 {
            return 0 as libc::c_int;
        }

        m = max!(fd1, fd2);
        ALLOCATE_BUFFERS!(m);

        /* If FD2 is the file descriptor bash is currently using for shell input,
        we need to do some extra work to make sure that the buffered stream
        actually exists (it might not if fd1 was not active, and the copy
        didn't actually do anything). */
        is_bash_input = (bash_input.type_0 as libc::c_uint
            == st_bstream as libc::c_int as libc::c_uint
            && bash_input.location.buffered_fd == fd2) as libc::c_int;

        if !(*buffers.offset(fd2 as isize)).is_null() {
            /* If the two objects share the same b_buffer, don't free it. */
            if !(*buffers.offset(fd1 as isize)).is_null()
                && !((**buffers.offset(fd1 as isize)).b_buffer).is_null()
                && (**buffers.offset(fd1 as isize)).b_buffer
                    == (**buffers.offset(fd2 as isize)).b_buffer
            {
                let ref mut fresh4 = *buffers.offset(fd2 as isize);
                *fresh4 = 0 as *mut libc::c_void as *mut BUFFERED_STREAM;
                /* If this buffer is shared with another fd, don't free the buffer */
            } else if (**buffers.offset(fd2 as isize)).b_flag & B_SHAREDBUF as libc::c_int != 0 {
                let ref mut fresh5 = (**buffers.offset(fd2 as isize)).b_buffer;
                *fresh5 = 0 as *mut libc::c_void as *mut libc::c_char;
                free_buffered_stream(*buffers.offset(fd2 as isize));
            } else {
                free_buffered_stream(*buffers.offset(fd2 as isize));
            }
        }
        let ref mut fresh6 = *buffers.offset(fd2 as isize);
        *fresh6 = copy_buffered_stream(*buffers.offset(fd1 as isize));
        if !(*buffers.offset(fd2 as isize)).is_null() {
            (**buffers.offset(fd2 as isize)).b_fd = fd2;
        }
        if is_bash_input != 0 {
            if (*buffers.offset(fd2 as isize)).is_null() {
                fd_to_buffered_stream(fd2);
            }
            (**buffers.offset(fd2 as isize)).b_flag |= B_WASBASHINPUT as libc::c_int;
        }
        if fd_is_bash_input(fd1) != 0
            || !(*buffers.offset(fd1 as isize)).is_null()
                && (**buffers.offset(fd1 as isize)).b_flag & B_SHAREDBUF as libc::c_int != 0
        {
            (**buffers.offset(fd2 as isize)).b_flag |= B_SHAREDBUF as libc::c_int;
        }

        return fd2;
    }
}

/* Take FD, a file descriptor, and create and return a buffered stream
corresponding to it.  If something is wrong and the file descriptor
is invalid, return a NULL stream. */
#[no_mangle]
pub fn fd_to_buffered_stream(fd: libc::c_int) -> *mut BUFFERED_STREAM {
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: size_t = 0;
    let mut sb: crate::src_common::stat = crate::src_common::stat_init;
    unsafe {
        if crate::src_common::fstat(fd, &mut sb) < 0 as libc::c_int {
            close(fd);
            return 0 as *mut libc::c_void as *mut BUFFERED_STREAM;
        }

        size = (if fd_is_seekable!(fd) {
            min!(sb.st_size, MAX_INPUT_BUFFER_SIZE as libc::c_long)
        } else {
            1 as libc::c_int as libc::c_long
        }) as size_t;
        if size == 0 as libc::c_int as libc::c_ulong {
            size = 1 as libc::c_int as size_t;
        }
        buffer = libc::malloc(size as usize) as *mut libc::c_char;

        return make_buffered_stream(fd, buffer, size);
    }
}

/* Return a buffered stream corresponding to FILE, a file name. */
#[no_mangle]
pub fn open_buffered_stream(file: *mut libc::c_char) -> *mut BUFFERED_STREAM {
    let mut fd: libc::c_int = 0;
    unsafe {
        fd = open(file, O_RDONLY);
        return if fd >= 0 as libc::c_int {
            fd_to_buffered_stream(fd)
        } else {
            0 as *mut libc::c_void as *mut BUFFERED_STREAM
        };
    }
}

/* Deallocate a buffered stream and free up its resources.  Make sure we
zero out the slot in BUFFERS that points to BP. */
#[no_mangle]
pub fn free_buffered_stream(bp: *mut BUFFERED_STREAM) {
    let mut n: libc::c_int = 0;

    if bp.is_null() {
        return;
    }
    unsafe {
        n = (*bp).b_fd;
        if !((*bp).b_buffer).is_null() {
            libc::free((*bp).b_buffer as *mut libc::c_void);
        }
        libc::free(bp as *mut libc::c_void);
        let ref mut fresh7 = *buffers.offset(n as isize);
        *fresh7 = 0 as *mut libc::c_void as *mut BUFFERED_STREAM;
    }
}

/* Close the file descriptor associated with BP, a buffered stream, and free
up the stream.  Return the status of closing BP's file descriptor. */
#[no_mangle]
pub fn close_buffered_stream(bp: *mut BUFFERED_STREAM) -> libc::c_int {
    let mut fd: libc::c_int = 0;

    if bp.is_null() {
        return 0 as libc::c_int;
    }
    unsafe {
        fd = (*bp).b_fd;
        if (*bp).b_flag & B_SHAREDBUF as libc::c_int != 0 {
            (*bp).b_buffer = 0 as *mut libc::c_void as *mut libc::c_char;
        }
        free_buffered_stream(bp);
        return close(fd);
    }
}

/* Deallocate the buffered stream associated with file descriptor FD, and
close FD.  Return the status of the close on FD. */
#[no_mangle]
pub fn close_buffered_fd(fd: libc::c_int) -> libc::c_int {
    unsafe {
        if fd < 0 as libc::c_int {
            *__errno_location() = EBADF;
            return -(1 as libc::c_int);
        }
        if fd >= nbuffers || buffers.is_null() || (*buffers.offset(fd as isize)).is_null() {
            return close(fd);
        }
        return close_buffered_stream(*buffers.offset(fd as isize));
    }
}

/* Make the BUFFERED_STREAM associated with buffers[FD] be BP, and return
the old BUFFERED_STREAM. */
#[no_mangle]
pub fn set_buffered_stream(fd: libc::c_int, bp: *mut BUFFERED_STREAM) -> *mut BUFFERED_STREAM {
    unsafe {
        let mut ret: *mut BUFFERED_STREAM = 0 as *mut BUFFERED_STREAM;

        ret = *buffers.offset(fd as isize);
        let ref mut fresh8 = *buffers.offset(fd as isize);
        *fresh8 = bp;
        return ret;
    }
}

/* Read a buffer full of characters from BP, a buffered stream. */
fn b_fill_buffer(bp: *mut BUFFERED_STREAM) -> libc::c_int {
    let mut nr: ssize_t = 0;
    let mut o: off_t = 0;
    unsafe {
        CHECK_TERMSIG!();
        /* In an environment where text and binary files are treated differently,
        compensate for lseek() on text files returning an offset different from
        the count of characters read() returns.  Text-mode streams have to be
        treated as unbuffered. */
        if (*bp).b_flag & (B_TEXT | B_UNBUFF) == B_TEXT {
            o = lseek((*bp).b_fd, 0 as __off_t, SEEK_CUR);
            nr = zread((*bp).b_fd, (*bp).b_buffer, (*bp).b_size);
            if nr > 0 as libc::c_long && nr < lseek((*bp).b_fd, 0 as __off_t, 1) - o {
                lseek((*bp).b_fd, o, SEEK_SET as libc::c_int);
                (*bp).b_flag |= B_UNBUFF;
                (*bp).b_size = 1 as size_t;
                nr = zread((*bp).b_fd, (*bp).b_buffer, (*bp).b_size);
            }
        } else {
            nr = zread((*bp).b_fd, (*bp).b_buffer, (*bp).b_size);
        }
        if nr <= 0 as libc::c_long {
            (*bp).b_inputp = 0 as size_t;
            (*bp).b_used = (*bp).b_inputp;
            *((*bp).b_buffer).offset(0 as isize) = 0 as libc::c_char;
            if nr == 0 as libc::c_long {
                (*bp).b_flag |= B_EOF as libc::c_int;
            } else {
                (*bp).b_flag |= B_ERROR as libc::c_int;
            }
            return EOF;
        }

        (*bp).b_used = nr as size_t;
        (*bp).b_inputp = 0 as size_t;
        let fresh9 = (*bp).b_inputp;
        (*bp).b_inputp = ((*bp).b_inputp).wrapping_add(1);
        return *((*bp).b_buffer).offset(fresh9 as isize) as libc::c_int & 0xff as libc::c_int;
    }
}

/* Push C back onto buffered stream BP. */
fn bufstream_ungetc(c: libc::c_int, bp: *mut BUFFERED_STREAM) -> libc::c_int {
    unsafe {
        if c == EOF || bp.is_null() || (*bp).b_inputp == 0 as libc::c_int as libc::c_ulong {
            return EOF;
        }

        (*bp).b_inputp = ((*bp).b_inputp).wrapping_sub(1);
        *((*bp).b_buffer).offset((*bp).b_inputp as isize) = c as libc::c_char;
        return c;
    }
}

/* Seek backwards on file BFD to synchronize what we've read so far
with the underlying file pointer. */
#[no_mangle]
pub fn sync_buffered_stream(bfd: libc::c_int) -> libc::c_int {
    let mut bp: *mut BUFFERED_STREAM = 0 as *mut BUFFERED_STREAM;
    let mut chars_left: off_t = 0;
    unsafe {
        if buffers.is_null() || {
            bp = *buffers.offset(bfd as isize);
            bp.is_null()
        } {
            return -(1 as libc::c_int);
        }

        chars_left = ((*bp).b_used).wrapping_sub((*bp).b_inputp) as off_t;
        if chars_left != 0 {
            lseek((*bp).b_fd, -chars_left, SEEK_CUR);
        }
        (*bp).b_inputp = 0 as size_t;
        (*bp).b_used = (*bp).b_inputp;
    }
    return 0;
}

#[no_mangle]
pub fn buffered_getchar() -> libc::c_int {
    unsafe {
        CHECK_TERMSIG!();

        if bash_input.location.buffered_fd < 0 as libc::c_int
            || (*buffers.offset(bash_input.location.buffered_fd as isize)).is_null()
        {
            return EOF;
        }

        return bufstream_getc!(**buffers.offset(bash_input.location.buffered_fd as isize));
    }
}

#[no_mangle]
pub fn buffered_ungetchar(c: libc::c_int) -> libc::c_int {
    unsafe {
        return bufstream_ungetc(c, *buffers.offset(bash_input.location.buffered_fd as isize));
    }
}

/* Make input come from file descriptor BFD through a buffered stream. */
#[no_mangle]
pub fn with_input_from_buffered_stream(bfd: libc::c_int, name: *mut libc::c_char) {
    let mut location: crate::y_tab::INPUT_STREAM = crate::y_tab::INPUT_STREAM {
        file: 0 as *const FILE as *mut FILE,
    };
    let mut bp: *mut BUFFERED_STREAM = 0 as *mut BUFFERED_STREAM;

    location.buffered_fd = bfd;
    /* Make sure the buffered stream exists. */
    bp = fd_to_buffered_stream(bfd);
    init_yy_io(
        if bp.is_null() {
            Some(return_EOF as fn() -> libc::c_int)
        } else {
            Some(buffered_getchar as fn() -> libc::c_int)
        },
        unsafe {
            ::core::mem::transmute::<Option<fn() -> libc::c_int>, Option<sh_cunget_func_t>>(Some(
                ::core::mem::transmute::<fn(libc::c_int) -> libc::c_int, fn() -> libc::c_int>(
                    buffered_ungetchar,
                ),
            ))
        },
        st_bstream,
        name,
        location,
    );
}
