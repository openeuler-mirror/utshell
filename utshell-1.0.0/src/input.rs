use crate::general::c_sh_unset_nodelay_mode;
use crate::src_common::*;
use crate::stringlib::xbcopy;
use crate::trap::run_pending_traps;
use crate::y_tab::{bash_input, init_yy_io, return_EOF};

/* Functions to handle reading input on systems that don't restart read(2)
if a signal is received. */

static mut localbuf: [libc::c_char; 1024] = [0; 1024];
static mut local_index: libc::c_int = 0;
static mut local_bufused: libc::c_int = 0;

/* Posix and USG systems do not guarantee to restart read () if it is
interrupted by a signal.  We do the read ourselves, and restart it
if it returns EINTR. */
#[no_mangle]
pub fn getc_with_restart(stream: *mut FILE) -> libc::c_int {
    // SAFETY: 访问静态缓冲区和调用系统调用
    unsafe {
        let uc: libc::c_uchar;

        CHECK_TERMSIG!();

        /* Try local buffering to reduce the number of read(2) calls. */
        if local_index == local_bufused || local_bufused == 0 {
            loop {
                QUIT!();
                run_pending_traps();

                local_bufused = read(
                    fileno(stream),
                    localbuf.as_mut_ptr() as *mut libc::c_void,
                    std::mem::size_of::<[libc::c_char; 1024]>(),
                ) as libc::c_int;

                if local_bufused > 0 {
                    break;
                }
                if local_bufused == 0 {
                    local_index = 0;
                    return EOF;
                } else if *c___errno_location() == X_EAGAIN
                    || *c___errno_location() == X_EWOULDBLOCK
                {
                    if c_sh_unset_nodelay_mode(fileno(stream)) < 0 {
                        sys_error(
                            c_dcgettext(
                                std::ptr::null(),
                                b"cannot reset nodelay mode for fd %d\0" as *const u8
                                    as *const libc::c_char,
                                5,
                            ),
                            fileno(stream),
                        );
                        local_bufused = 0;
                        local_index = local_bufused;
                        return EOF;
                    }
                } else if *c___errno_location() != EINTR {
                    local_bufused = 0;
                    local_index = local_bufused;
                    return EOF;
                } else if interrupt_state != 0 || terminating_signal != 0 {
                    /* QUIT; */
                    local_bufused = 0;
                    local_index = local_bufused;
                }
            }
            local_index = 0;
        }
        let fresh0 = local_index;
        local_index = local_index + 1;
        uc = localbuf[fresh0 as usize] as libc::c_uchar;
        uc as libc::c_int
    }
}

#[no_mangle]
pub fn ungetc_with_restart(c: libc::c_int, _stream: *mut FILE) -> libc::c_int {
    // SAFETY: 访问静态缓冲区
    unsafe {
        if local_index == 0 || c == EOF {
            return EOF;
        }
        local_index -= 1;
        localbuf[local_index as usize] = c as libc::c_char;
        c
    }
}

#[no_mangle]
pub static mut bash_input_fd_changed: libc::c_int = 0;

/* This provides a way to map from a file descriptor to the buffer
associated with that file descriptor, rather than just the other
way around.  This is needed so that buffers are managed properly
in constructs like 3<&4.  buffers[x]->b_fd == x -- that is how the
correspondence is maintained. */
static mut buffers: *mut *mut BUFFERED_STREAM = std::ptr::null_mut();
static mut nbuffers: libc::c_int = 0;

/* Make sure `buffers' has at least N elements. */
fn allocate_buffers(n: libc::c_int) {
    // SAFETY: 内存分配和指针操作
    unsafe {
        let orig_nbuffers = nbuffers;
        nbuffers = n + 20;
        buffers = libc::realloc(
            buffers as *mut libc::c_void,
            (nbuffers as usize) * std::mem::size_of::<*mut BUFFERED_STREAM>(),
        ) as *mut *mut BUFFERED_STREAM;

        /* Zero out the new buffers. */
        for i in orig_nbuffers..nbuffers {
            *buffers.offset(i as isize) = std::ptr::null_mut();
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
    // SAFETY: 内存分配和结构体初始化
    unsafe {
        let bp = libc::malloc(std::mem::size_of::<BUFFERED_STREAM>()) as *mut BUFFERED_STREAM;
        ALLOCATE_BUFFERS!(fd);
        *buffers.offset(fd as isize) = bp;
        (*bp).b_fd = fd;
        (*bp).b_buffer = buffer;
        (*bp).b_size = bufsize;
        (*bp).b_flag = 0;
        (*bp).b_inputp = (*bp).b_flag as size_t;
        (*bp).b_used = (*bp).b_inputp;
        if bufsize == 1 {
            (*bp).b_flag |= B_UNBUFF;
        }
        if O_TEXT != 0 && fcntl(fd, F_GETFL) & O_TEXT != 0 {
            (*bp).b_flag |= B_TEXT;
        }
        bp
    }
}

/* Allocate a new BUFFERED_STREAM, copy BP to it, and return the new copy. */
fn copy_buffered_stream(bp: *mut BUFFERED_STREAM) -> *mut BUFFERED_STREAM {
    if bp.is_null() {
        return std::ptr::null_mut();
    }

    // SAFETY: 内存分配和内存拷贝
    unsafe {
        let nbp = libc::malloc(std::mem::size_of::<BUFFERED_STREAM>()) as *mut BUFFERED_STREAM;
        xbcopy(
            bp as *mut libc::c_char,
            nbp as *mut libc::c_char,
            std::mem::size_of::<BUFFERED_STREAM>() as libc::c_int,
        );
        nbp
    }
}

#[no_mangle]
pub fn set_bash_input_fd(fd: libc::c_int) -> libc::c_int {
    // SAFETY: 访问静态变量 bash_input 和 interactive_shell
    unsafe {
        if bash_input.type_0 as libc::c_uint == st_bstream as libc::c_uint {
            bash_input.location.buffered_fd = fd;
        } else if interactive_shell == 0 {
            default_buffered_input = fd;
        }
    }
    0
}

#[no_mangle]
pub fn fd_is_bash_input(fd: libc::c_int) -> libc::c_int {
    // SAFETY: 访问静态变量
    unsafe {
        if bash_input.type_0 as libc::c_uint == st_bstream as libc::c_uint
            && bash_input.location.buffered_fd == fd
        {
            return 1;
        } else if interactive_shell == 0 && default_buffered_input == fd {
            return 1;
        }
        0
    }
}

/* Save the buffered stream corresponding to file descriptor FD (which bash
is using to read input) to a buffered stream associated with NEW_FD.  If
NEW_FD is -1, a new file descriptor is allocated with fcntl.  The new
file descriptor is returned on success, -1 on error. */
#[no_mangle]
pub fn save_bash_input(fd: libc::c_int, new_fd: libc::c_int) -> libc::c_int {
    // SAFETY: 文件描述符操作和缓冲区管理
    unsafe {
        /* Sync the stream so we can re-read from the new file descriptor.  We
        might be able to avoid this by copying the buffered stream verbatim
        to the new file descriptor. */
        if !(*buffers.offset(fd as isize)).is_null() {
            sync_buffered_stream(fd);
        }

        /* Now take care of duplicating the file descriptor that bash is
        using for input, so we can reinitialize it later. */
        let nfd = if new_fd == -1 {
            fcntl(fd, 0, 10)
        } else {
            new_fd
        };

        if nfd == -1 {
            if fcntl(fd, 1, 0) == 0 {
                sys_error(
                    c_dcgettext(
                        std::ptr::null(),
                        b"cannot allocate new file descriptor for bash input from fd %d\0"
                            as *const u8 as *const libc::c_char,
                        5,
                    ),
                    fd,
                );
            }
            return -1;
        }

        if nfd < nbuffers && !(*buffers.offset(nfd as isize)).is_null() {
            /* What's this?  A stray buffer without an associated open file
            descriptor?  Free up the buffer and report the error. */
            internal_error(
                c_dcgettext(
                    std::ptr::null(),
                    b"save_bash_input: buffer already exists for new fd %d\0" as *const u8
                        as *const libc::c_char,
                    5,
                ),
                nfd,
            );
            if (**buffers.offset(nfd as isize)).b_flag & B_SHAREDBUF != 0 {
                (**buffers.offset(nfd as isize)).b_buffer = std::ptr::null_mut();
            }
            free_buffered_stream(*buffers.offset(nfd as isize));
        }

        /* Reinitialize bash_input.location. */
        if bash_input.type_0 as libc::c_uint == st_bstream as libc::c_uint {
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
        }
        if default_buffered_input == fd {
            default_buffered_input = nfd;
        }

        SET_CLOSE_ON_EXEC!(nfd);
        nfd
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
        if fd > 0 {
            return if save_bash_input(fd, -1) == -1 { -1 } else { 0 };
        } else if fd == 0 {
            return if sync_buffered_stream(fd) == -1 {
                -1
            } else {
                0
            };
        }
    }
    0
}

/* This is the buffered stream analogue of dup2(fd1, fd2).  The
BUFFERED_STREAM corresponding to fd2 is deallocated, if one exists.
BUFFERS[fd1] is copied to BUFFERS[fd2].  This is called by the
redirect code for constructs like 4<&0 and 3</etc/rc.local. */
#[no_mangle]
pub fn duplicate_buffered_stream(fd1: libc::c_int, fd2: libc::c_int) -> libc::c_int {
    // SAFETY: 缓冲区复制和指针操作
    unsafe {
        if fd1 == fd2 {
            return 0;
        }

        let m = max!(fd1, fd2);
        ALLOCATE_BUFFERS!(m);

        /* If FD2 is the file descriptor bash is currently using for shell input,
        we need to do some extra work to make sure that the buffered stream
        actually exists (it might not if fd1 was not active, and the copy
        didn't actually do anything). */
        let is_bash_input = (bash_input.type_0 as libc::c_uint == st_bstream as libc::c_uint
            && bash_input.location.buffered_fd == fd2) as libc::c_int;

        if !(*buffers.offset(fd2 as isize)).is_null() {
            /* If the two objects share the same b_buffer, don't free it. */
            if !(*buffers.offset(fd1 as isize)).is_null()
                && !(**buffers.offset(fd1 as isize)).b_buffer.is_null()
                && (**buffers.offset(fd1 as isize)).b_buffer
                    == (**buffers.offset(fd2 as isize)).b_buffer
            {
                *buffers.offset(fd2 as isize) = std::ptr::null_mut();
                /* If this buffer is shared with another fd, don't free the buffer */
            } else if (**buffers.offset(fd2 as isize)).b_flag & B_SHAREDBUF != 0 {
                (**buffers.offset(fd2 as isize)).b_buffer = std::ptr::null_mut();
                free_buffered_stream(*buffers.offset(fd2 as isize));
            } else {
                free_buffered_stream(*buffers.offset(fd2 as isize));
            }
        }

        *buffers.offset(fd2 as isize) = copy_buffered_stream(*buffers.offset(fd1 as isize));

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
                && (**buffers.offset(fd1 as isize)).b_flag & B_SHAREDBUF != 0
        {
            (**buffers.offset(fd2 as isize)).b_flag |= B_SHAREDBUF;
        }

        fd2
    }
}

/* Take FD, a file descriptor, and create and return a buffered stream
corresponding to it.  If something is wrong and the file descriptor
is invalid, return a NULL stream. */
#[no_mangle]
pub fn fd_to_buffered_stream(fd: libc::c_int) -> *mut BUFFERED_STREAM {
    let mut sb: crate::src_common::stat = crate::src_common::stat_init;

    // SAFETY: 文件状态获取和内存分配
    unsafe {
        if crate::src_common::c_fstat(fd, &mut sb) < 0 {
            close(fd);
            return std::ptr::null_mut();
        }

        let size = if fd_is_seekable!(fd) {
            min!(sb.st_size, MAX_INPUT_BUFFER_SIZE as libc::c_long)
        } else {
            1
        } as size_t;

        let size = if size == 0 { 1 } else { size };
        let buffer = libc::malloc(size as usize) as *mut libc::c_char;

        make_buffered_stream(fd, buffer, size)
    }
}

/* Return a buffered stream corresponding to FILE, a file name. */
#[no_mangle]
pub fn open_buffered_stream(file: *mut libc::c_char) -> *mut BUFFERED_STREAM {
    // SAFETY: 文件打开操作
    unsafe {
        let fd = open(file, O_RDONLY);
        if fd >= 0 {
            fd_to_buffered_stream(fd)
        } else {
            std::ptr::null_mut()
        }
    }
}

/* Deallocate a buffered stream and free up its resources.  Make sure we
zero out the slot in BUFFERS that points to BP. */
#[no_mangle]
pub fn free_buffered_stream(bp: *mut BUFFERED_STREAM) {
    if bp.is_null() {
        return;
    }
    // SAFETY: 内存释放和指针操作
    unsafe {
        let n = (*bp).b_fd;
        if !(*bp).b_buffer.is_null() {
            libc::free((*bp).b_buffer as *mut libc::c_void);
        }
        libc::free(bp as *mut libc::c_void);
        *buffers.offset(n as isize) = std::ptr::null_mut();
    }
}

/* Close the file descriptor associated with BP, a buffered stream, and free
up the stream.  Return the status of closing BP's file descriptor. */
#[no_mangle]
pub fn close_buffered_stream(bp: *mut BUFFERED_STREAM) -> libc::c_int {
    if bp.is_null() {
        return 0;
    }
    // SAFETY: 文件关闭和内存释放
    unsafe {
        let fd = (*bp).b_fd;
        if (*bp).b_flag & B_SHAREDBUF != 0 {
            (*bp).b_buffer = std::ptr::null_mut();
        }
        free_buffered_stream(bp);
        close(fd)
    }
}

/* Deallocate the buffered stream associated with file descriptor FD, and
close FD.  Return the status of the close on FD. */
#[no_mangle]
pub fn close_buffered_fd(fd: libc::c_int) -> libc::c_int {
    // SAFETY: 文件描述符操作
    unsafe {
        if fd < 0 {
            *c___errno_location() = EBADF;
            return -1;
        }
        if fd >= nbuffers || buffers.is_null() || (*buffers.offset(fd as isize)).is_null() {
            return close(fd);
        }
        close_buffered_stream(*buffers.offset(fd as isize))
    }
}

/* Make the BUFFERED_STREAM associated with buffers[FD] be BP, and return
the old BUFFERED_STREAM. */
#[no_mangle]
pub fn set_buffered_stream(fd: libc::c_int, bp: *mut BUFFERED_STREAM) -> *mut BUFFERED_STREAM {
    // SAFETY: 指针交换操作
    unsafe {
        let ret = *buffers.offset(fd as isize);
        *buffers.offset(fd as isize) = bp;
        ret
    }
}

/* Read a buffer full of characters from BP, a buffered stream. */
fn b_fill_buffer(bp: *mut BUFFERED_STREAM) -> libc::c_int {
    // SAFETY: 文件读取和缓冲区操作
    unsafe {
        CHECK_TERMSIG!();
        /* In an environment where text and binary files are treated differently,
        compensate for lseek() on text files returning an offset different from
        the count of characters read() returns.  Text-mode streams have to be
        treated as unbuffered. */
        if (*bp).b_flag & (B_TEXT | B_UNBUFF) == B_TEXT {
            let o = lseek((*bp).b_fd, 0, SEEK_CUR);
            let nr = c_zread((*bp).b_fd, (*bp).b_buffer, (*bp).b_size);
            if nr > 0 && nr < lseek((*bp).b_fd, 0, 1) - o {
                lseek((*bp).b_fd, o, SEEK_SET as libc::c_int);
                (*bp).b_flag |= B_UNBUFF;
                (*bp).b_size = 1;
                c_zread((*bp).b_fd, (*bp).b_buffer, (*bp).b_size);
            }
        } else {
            let nr = c_zread((*bp).b_fd, (*bp).b_buffer, (*bp).b_size);
            if nr <= 0 {
                (*bp).b_inputp = 0;
                (*bp).b_used = (*bp).b_inputp;
                *((*bp).b_buffer).offset(0) = 0;
                if nr == 0 {
                    (*bp).b_flag |= B_EOF as libc::c_int;
                } else {
                    (*bp).b_flag |= B_ERROR as libc::c_int;
                }
                return EOF;
            }
            (*bp).b_used = nr as size_t;
            (*bp).b_inputp = 0;
            let fresh9 = (*bp).b_inputp;
            (*bp).b_inputp = (*bp).b_inputp.wrapping_add(1);
            return *((*bp).b_buffer).offset(fresh9 as isize) as libc::c_int & 0xff;
        }

        // Handle B_TEXT | B_UNBUFF case
        let nr = c_zread((*bp).b_fd, (*bp).b_buffer, (*bp).b_size);
        if nr <= 0 {
            (*bp).b_inputp = 0;
            (*bp).b_used = (*bp).b_inputp;
            *((*bp).b_buffer).offset(0) = 0;
            if nr == 0 {
                (*bp).b_flag |= B_EOF as libc::c_int;
            } else {
                (*bp).b_flag |= B_ERROR as libc::c_int;
            }
            return EOF;
        }
        (*bp).b_used = nr as size_t;
        (*bp).b_inputp = 0;
        let fresh9 = (*bp).b_inputp;
        (*bp).b_inputp = (*bp).b_inputp.wrapping_add(1);
        *((*bp).b_buffer).offset(fresh9 as isize) as libc::c_int & 0xff
    }
}

/* Push C back onto buffered stream BP. */
fn bufstream_ungetc(c: libc::c_int, bp: *mut BUFFERED_STREAM) -> libc::c_int {
    // SAFETY: 缓冲区操作
    unsafe {
        if c == EOF || bp.is_null() || (*bp).b_inputp == 0 {
            return EOF;
        }

        (*bp).b_inputp = (*bp).b_inputp.wrapping_sub(1);
        *((*bp).b_buffer).offset((*bp).b_inputp as isize) = c as libc::c_char;
        c
    }
}

/* Seek backwards on file BFD to synchronize what we've read so far
with the underlying file pointer. */
#[no_mangle]
pub fn sync_buffered_stream(bfd: libc::c_int) -> libc::c_int {
    // SAFETY: 文件定位操作
    unsafe {
        if buffers.is_null() {
            return -1;
        }
        let bp = *buffers.offset(bfd as isize);
        if bp.is_null() {
            return -1;
        }

        let chars_left = ((*bp).b_used.wrapping_sub((*bp).b_inputp)) as off_t;
        if chars_left != 0 {
            lseek((*bp).b_fd, -chars_left, SEEK_CUR);
        }
        (*bp).b_inputp = 0;
        (*bp).b_used = (*bp).b_inputp;
    }
    0
}

#[no_mangle]
pub fn buffered_getchar() -> libc::c_int {
    // SAFETY: 访问静态变量和缓冲区
    unsafe {
        CHECK_TERMSIG!();

        if bash_input.location.buffered_fd < 0
            || (*buffers.offset(bash_input.location.buffered_fd as isize)).is_null()
        {
            return EOF;
        }

        bufstream_getc!(**buffers.offset(bash_input.location.buffered_fd as isize))
    }
}

#[no_mangle]
pub fn buffered_ungetchar(c: libc::c_int) -> libc::c_int {
    // SAFETY: 访问静态变量和缓冲区
    unsafe { bufstream_ungetc(c, *buffers.offset(bash_input.location.buffered_fd as isize)) }
}

/* Make input come from file descriptor BFD through a buffered stream. */
#[no_mangle]
pub fn with_input_from_buffered_stream(bfd: libc::c_int, name: *mut libc::c_char) {
    let mut location: crate::y_tab::INPUT_STREAM = crate::y_tab::INPUT_STREAM {
        file: std::ptr::null_mut(),
    };

    location.buffered_fd = bfd;
    /* Make sure the buffered stream exists. */
    let bp = fd_to_buffered_stream(bfd);
    // SAFETY: 函数指针转换
    unsafe {
        init_yy_io(
            if bp.is_null() {
                Some(return_EOF as fn() -> libc::c_int)
            } else {
                Some(buffered_getchar as fn() -> libc::c_int)
            },
            ::core::mem::transmute::<Option<fn() -> libc::c_int>, Option<sh_cunget_func_t>>(Some(
                ::core::mem::transmute::<fn(libc::c_int) -> libc::c_int, fn() -> libc::c_int>(
                    buffered_ungetchar,
                ),
            )),
            st_bstream,
            name,
            location,
        );
    }
}
