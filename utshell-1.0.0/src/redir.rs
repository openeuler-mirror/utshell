use crate::arrayfunc::{array_variable_part, get_array_value, valid_array_reference};
use crate::copycmd::{copy_redirects, copy_word};
use crate::dispose_cmd::{dispose_redirects, dispose_words};
use crate::execute_cmd::{coproc_fdchk, dispose_exec_redirects};
use crate::general::{all_digits, legal_number, same_file};
use crate::input::{check_bash_input, close_buffered_fd, duplicate_buffered_stream};
use crate::make_cmd::{make_bare_word, make_redirection, make_word_list};
use crate::print_cmd::xtrace_fdchk;
use crate::src_common::*;
use crate::stringlib::find_string_in_alist;
use crate::subst::{
    expand_assignment_string_to_string, expand_string_to_string, expand_words_no_vars, string_list,
};
use crate::trap::run_pending_traps;
use crate::variables::{
    bind_var_to_int, find_variable_last_nameref, get_variable_value,
    stupidly_hack_special_variables, sv_ifs,
};

static mut rd: REDIRECTEE = REDIRECTEE { dest: 0 };
static mut heredoc_errno: libc::c_int = 0;

/* STRLEN_1 本来为STRLE，由于存在冲突，所以将本宏该为STRLEN_1，表示第一个变型 */
#[macro_export]
macro_rules! STRLEN_1 {
    ($desc: expr) => {
        if !((*($desc as *mut WORD_DESC)).word).is_null()
            && *((*($desc as *mut WORD_DESC)).word).offset(0 as libc::c_int as isize) as libc::c_int
                != 0
        {
            if *((*($desc as *mut WORD_DESC)).word).offset(1 as libc::c_int as isize) as libc::c_int
                != 0
            {
                if *((*($desc as *mut WORD_DESC)).word).offset(2 as libc::c_int as isize)
                    as libc::c_int
                    != 0
                {
                    strlen((*($desc as *mut WORD_DESC)).word) as u64
                } else {
                    2 as libc::c_int as libc::c_ulong
                }
            } else {
                1 as libc::c_int as libc::c_ulong
            }
        } else {
            0 as libc::c_int as libc::c_ulong
        }
    };
}

#[no_mangle]
pub fn redirection_error(temp: *mut REDIRECT, error: libc::c_int, fn_0: *mut libc::c_char) {
    let mut filename: *mut libc::c_char;
    let mut allocname: *mut libc::c_char;
    let oflags: libc::c_int;
    allocname = 0 as *mut libc::c_char;
    unsafe {
        if (*temp).rflags & 0x1 as libc::c_int != 0 && error < 0 as libc::c_int {
            allocname = strcpy(
                malloc(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(strlen((*(*temp).redirector.filename).word) as u64)
                        as usize,
                ) as *mut libc::c_char,
                (*(*temp).redirector.filename).word,
            );
            filename = allocname;
        } else if (*temp).rflags & REDIR_VARASSIGN as libc::c_int == 0 as libc::c_int
            && (*temp).redirector.dest < REDIR_VARASSIGN as libc::c_int
        {
            filename = c_dcgettext(
                0 as *const libc::c_char,
                b"file descriptor out of range\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        } else if error != NOCLOBBER_REDIRECT
            && (*temp).redirector.dest >= 0 as libc::c_int
            && error == EBADF as libc::c_int
        {
            match (*temp).instruction as libc::c_uint {
                r_duplicating_input | r_duplicating_output | r_move_input | r_move_output => {
                    allocname = c_itos((*temp).redirectee.dest as intmax_t);
                    filename = allocname;
                }
                r_duplicating_input_word => {
                    if (*temp).redirector.dest == 0 as libc::c_int {
                        filename = (*(*temp).redirectee.filename).word;
                    } else {
                        allocname = c_itos((*temp).redirector.dest as intmax_t);
                        filename = allocname;
                    }
                }
                r_duplicating_output_word => {
                    if (*temp).redirector.dest == 1 as libc::c_int {
                        filename = (*(*temp).redirectee.filename).word;
                    } else {
                        allocname = c_itos((*temp).redirector.dest as intmax_t);
                        filename = allocname;
                    }
                }
                _ => {
                    allocname = c_itos((*temp).redirector.dest as intmax_t);
                    filename = allocname;
                }
            }
        } else if !fn_0.is_null() {
            filename = fn_0;
        } else if expandable_redirection_filename(temp) != 0 {
            oflags = (*(*temp).redirectee.filename).flags;
            if posixly_correct != 0 && interactive_shell == 0 as libc::c_int {
                (*(*temp).redirectee.filename).flags |= (1 as libc::c_int) << 5 as libc::c_int;
            }
            (*(*temp).redirectee.filename).flags |= (1 as libc::c_int) << 10 as libc::c_int;
            allocname = redirection_expand((*temp).redirectee.filename);
            filename = allocname;
            (*(*temp).redirectee.filename).flags = oflags;
            if filename.is_null() {
                filename = (*(*temp).redirectee.filename).word;
            }
        } else if (*temp).redirectee.dest < 0 as libc::c_int {
            filename = c_dcgettext(
                0 as *const libc::c_char,
                b"file descriptor out of range\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        } else {
            allocname = c_itos((*temp).redirectee.dest as intmax_t);
            filename = allocname;
        }

        match error {
            AMBIGUOUS_REDIRECT => {
                internal_error(
                    c_dcgettext(
                        0 as *const libc::c_char,
                        b"%s: ambiguous redirect\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    filename,
                );
            }
            NOCLOBBER_REDIRECT => {
                internal_error(
                    c_dcgettext(
                        0 as *const libc::c_char,
                        b"%s: cannot overwrite existing file\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    filename,
                );
            }
            RESTRICTED_REDIRECT => {
                internal_error(
                    c_dcgettext(
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
                    c_dcgettext(
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
                    c_dcgettext(
                        0 as *const libc::c_char,
                        b"%s: cannot assign fd to variable\0" as *const u8 as *const libc::c_char,
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
        // allocname = 0 as *mut libc::c_char;
    }
}
#[no_mangle]
pub fn do_redirections(list: *mut REDIRECT, flags: libc::c_int) -> libc::c_int {
    let mut error: libc::c_int;
    let mut temp: *mut REDIRECT;
    let mut fn_0: *mut libc::c_char;
    unsafe {
        if flags & (RX_UNDOABLE as i32) != 0 {
            if !redirection_undo_list.is_null() {
                dispose_redirects(redirection_undo_list);
                redirection_undo_list = 0 as *mut libc::c_void as *mut REDIRECT;
            }
            if !exec_redirection_undo_list.is_null() {
                dispose_exec_redirects();
            }
        }
    }
    temp = list;
    while !temp.is_null() {
        fn_0 = 0 as *mut libc::c_char;
        error = do_redirection_internal(temp, flags, &mut fn_0);
        if error != 0 {
            redirection_error(temp, error, fn_0);
            if !fn_0.is_null() {
                unsafe {
                    free(fn_0 as *mut libc::c_void);
                }
            }
            // fn_0 = 0 as *mut libc::c_char;
            return error;
        }
        if !fn_0.is_null() {
            unsafe {
                free(fn_0 as *mut libc::c_void);
            }
        }
        // fn_0 = 0 as *mut libc::c_char;
        unsafe {
            temp = (*temp).next;
        }
    }

    return 0 as libc::c_int;
}
fn expandable_redirection_filename(redirect: *mut REDIRECT) -> libc::c_int {
    match unsafe { (*redirect).instruction as libc::c_uint } {
        r_output_direction
        | r_appending_to
        | r_input_direction
        | r_inputa_direction
        | r_err_and_out
        | r_append_err_and_out
        | r_input_output
        | r_output_force
        | r_duplicating_input_word
        | r_duplicating_output_word
        | r_move_input_word
        | r_move_output_word => return 1 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}

#[no_mangle]
pub fn redirection_expand(word: *mut WORD_DESC) -> *mut libc::c_char {
    let result: *mut libc::c_char;
    let tlist1: *mut WORD_LIST;
    let tlist2: *mut WORD_LIST;
    let w: *mut WORD_DESC;
    let old: libc::c_int;

    w = copy_word(word);
    unsafe {
        if posixly_correct != 0 {
            (*w).flags |= (1 as libc::c_int) << 4 as libc::c_int;
        }
    }
    tlist1 = make_word_list(w, 0 as *mut libc::c_void as *mut WORD_LIST);
    unsafe {
        expanding_redir = 1 as libc::c_int;
    }
    sv_ifs(b"IFS\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    tlist2 = expand_words_no_vars(tlist1);
    unsafe {
        expanding_redir = 0 as libc::c_int;
    }
    /* Now we need to change the variable search order back to include the temp
    environment.  We force the temp environment search by forcing
    executing_builtin to 1.  This is what makes `read' get the right values
    for the IFS-related cached variables, for example. */
    unsafe {
        old = executing_builtin;
        executing_builtin = 1 as libc::c_int;
        sv_ifs(b"IFS\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        executing_builtin = old;
    }
    dispose_words(tlist1);
    if unsafe { tlist2.is_null() || !((*tlist2).next).is_null() } {
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
fn heredoc_expand(
    redirectee: *mut WORD_DESC,
    ri: libc::c_uint,
    lenp: *mut size_t,
) -> *mut libc::c_char {
    let mut document: *mut libc::c_char;
    let mut dlen: size_t;
    let old: libc::c_int;
    unsafe {
        if ((*redirectee).word).is_null()
            || *((*redirectee).word).offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
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
                (*lenp) = STRLEN_1!(redirectee);
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
        dlen = STRLEN_1!(document);
        if ri as libc::c_uint == r_reading_string {
            document = realloc(
                document as *mut libc::c_void,
                dlen.wrapping_add(2 as libc::c_int as size_t) as usize,
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
}
fn heredoc_write(fd: libc::c_int, heredoc: *mut libc::c_char, herelen: size_t) -> libc::c_int {
    let nw: ssize_t;
    let mut e: libc::c_int;
    unsafe {
        *c___errno_location() = 0 as libc::c_int;
        nw = write(fd, heredoc as *const libc::c_void, herelen as usize) as i64;
        e = *c___errno_location();
    }
    if nw as size_t != herelen {
        if e == 0 as libc::c_int {
            e = ENOSPC;
        }
        return e;
    }
    return 0 as libc::c_int;
}

fn here_document_to_fd(redirectee: *mut WORD_DESC, ri: libc::c_uint) -> libc::c_int {
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: libc::c_int;
    let fd: libc::c_int;
    let fd2: libc::c_int;
    let mut herepipe: [libc::c_int; 2] = [0; 2];
    let document: *mut libc::c_char;
    let mut document_len: size_t = 0;
    document = heredoc_expand(redirectee, ri, &mut document_len);
    unsafe {
        if document_len == 0 as libc::c_int as size_t {
            fd = open(
                b"/dev/null\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
            r = *c___errno_location();
            if document != (*redirectee).word {
                if !document.is_null() {
                    free(document as *mut libc::c_void);
                }
                // document = 0 as *mut libc::c_char;
            }
            *c___errno_location() = r;
            return fd;
        }
        if document_len <= HEREDOC_PIPESIZE as libc::c_int as size_t {
            if pipe(herepipe.as_mut_ptr()) < 0 as libc::c_int {
                r = *c___errno_location();
                if document != (*redirectee).word {
                    free(document as *mut libc::c_void);
                }
                *c___errno_location() = r;
                return -(1 as libc::c_int);
            }
            if !((fcntl(
                herepipe[1 as libc::c_int as usize],
                F_GETPIPE_SZ,
                0 as libc::c_int,
            ) as size_t)
                < document_len)
            {
                r = heredoc_write(herepipe[1 as libc::c_int as usize], document, document_len);
                if document != (*redirectee).word {
                    free(document as *mut libc::c_void);
                }
                close(herepipe[1 as libc::c_int as usize]);
                if r != 0 {
                    close(herepipe[0 as libc::c_int as usize]);
                    *c___errno_location() = r;
                    return -(1 as libc::c_int);
                }
                return herepipe[0 as libc::c_int as usize];
            }
        }
        fd = c_sh_mktmpfd(
            b"sh-thd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (MT_USERANDOM | MT_USETMPDIR) as i32,
            &mut filename,
        );
        if fd < 0 as libc::c_int {
            r = *c___errno_location();
            if !filename.is_null() {
                free(filename as *mut libc::c_void);
            }
            // filename = 0 as *mut libc::c_char;
            if document != (*redirectee).word {
                if !document.is_null() {
                    free(document as *mut libc::c_void);
                }
                // document = 0 as *mut libc::c_char;
            }
            *c___errno_location() = r;
            return fd;
        }

        fchmod(fd, (S_IRUSR | S_IWUSR) as libc::c_uint);
        fcntl(fd, 2 as libc::c_int, 1 as libc::c_int);
        r = 0 as libc::c_int;

        *c___errno_location() = r;
        r = heredoc_write(fd, document, document_len);
        if document != (*redirectee).word {
            if !document.is_null() {
                free(document as *mut libc::c_void);
            }
            // document = 0 as *mut libc::c_char;
        }
        if r != 0 {
            close(fd);
            unlink(filename);
            free(filename as *mut libc::c_void);
            *c___errno_location() = r;
            return -(1 as libc::c_int);
        }
        fd2 = open(
            filename,
            0 as libc::c_int | 0 as libc::c_int,
            0o600 as libc::c_int,
        );
        if fd2 < 0 as libc::c_int {
            r = *c___errno_location();
            unlink(filename);
            free(filename as *mut libc::c_void);
            close(fd);
            *c___errno_location() = r;
            return -(1 as libc::c_int);
        }
        close(fd);
        if unlink(filename) < 0 as libc::c_int {
            r = *c___errno_location();
            close(fd2);
            free(filename as *mut libc::c_void);
            *c___errno_location() = r;
            return -(1 as libc::c_int);
        }
        free(filename as *mut libc::c_void);
        fchmod(fd2, 0o400 as libc::c_int as libc::c_uint);
        return fd2;
    }
}

static mut _redir_special_filenames: [STRING_INT_ALIST; 3] = [
    {
        let init = STRING_INT_ALIST {
            word: b"/dev/tcp/*/*\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: RF_DEVTCP,
        };
        init
    },
    {
        let init = STRING_INT_ALIST {
            word: b"/dev/udp/*/*\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: RF_DEVUDP,
        };
        init
    },
    {
        let init = STRING_INT_ALIST {
            word: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
            token: RF_END,
        };
        init
    },
];
fn redir_special_open(
    spec: libc::c_int,
    filename: *mut libc::c_char,
    mut _flags: libc::c_int,
    mut _mode: libc::c_int,
    mut _ri: r_instruction,
) -> libc::c_int {
    let mut fd: libc::c_int;
    fd = RF_END;
    match spec {
        RF_DEVTCP | RF_DEVUDP => {
            if unsafe { restricted != 0 } {
                return RESTRICTED_REDIRECT;
            }
            fd = c_netopen(filename);
        }
        _ => {}
    }
    return fd;
}

fn noclobber_open(
    filename: *mut libc::c_char,
    mut flags: libc::c_int,
    mode: libc::c_int,
    mut _ri: r_instruction,
) -> libc::c_int {
    let r: libc::c_int;
    let fd: libc::c_int;
    let mut finfo: crate::src_common::stat = crate::src_common::stat_init;
    let mut finfo2: crate::src_common::stat = crate::src_common::stat { ..finfo };
    r = c_stat(filename, &mut finfo);
    if r == 0 && S_ISREG!(finfo.st_mode) {
        return NOCLOBBER_REDIRECT;
    }

    flags &= !(O_TRUNC as i32);
    if r != 0 as libc::c_int {
        fd = unsafe { open(filename, flags | O_EXCL as libc::c_int, mode) };
        return if unsafe { fd < 0 as libc::c_int && *c___errno_location() == 17 as libc::c_int } {
            NOCLOBBER_REDIRECT
        } else {
            fd
        };
    }
    fd = unsafe { open(filename, flags, mode) };
    if fd < 0 as libc::c_int {
        return if unsafe { *c___errno_location() == EEXIST as libc::c_int } {
            NOCLOBBER_REDIRECT
        } else {
            fd
        };
    }
    if c_fstat(fd, &mut finfo2) == 0 as libc::c_int
        && (finfo2.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint) as libc::c_int
            == 0 as libc::c_int
        && r == 0 as libc::c_int
        && (finfo.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint) as libc::c_int
            == 0 as libc::c_int
        && same_file(filename, filename, &mut finfo, &mut finfo2) != 0
    {
        return fd;
    }
    unsafe {
        close(fd);
        *c___errno_location() = EEXIST;
    }
    return NOCLOBBER_REDIRECT;
}

fn redir_open(
    filename: *mut libc::c_char,
    flags: libc::c_int,
    mode: libc::c_int,
    ri: r_instruction,
) -> libc::c_int {
    let mut fd: libc::c_int;
    let r: libc::c_int;
    let mut e: libc::c_int;
    r = unsafe {
        find_string_in_alist(
            filename,
            _redir_special_filenames.as_mut_ptr(),
            1 as libc::c_int,
        )
    };
    if r >= 0 as libc::c_int {
        return redir_special_open(r, filename, flags, mode, ri as libc::c_uint);
    }
    if unsafe { noclobber != 0 && CLOBBERING_REDIRECT!(ri) } {
        fd = noclobber_open(filename, flags, mode, ri as libc::c_uint);
        if fd == NOCLOBBER_REDIRECT {
            return NOCLOBBER_REDIRECT;
        }
    } else {
        loop {
            unsafe {
                fd = open(filename, flags, mode);
                e = *c___errno_location();
                if fd < 0 as libc::c_int && e == EINTR {
                    if terminating_signal != 0 {
                        termsig_handler(terminating_signal);
                    }
                    if interrupt_state != 0 {
                        throw_to_top_level();
                    }
                    run_pending_traps();
                }
                *c___errno_location() = e;
                if !(fd < 0 as libc::c_int && *c___errno_location() == EINTR) {
                    break;
                }
            }
        }
    }
    return fd;
}

fn do_redirection_internal(
    redirect: *mut REDIRECT,
    mut flags: libc::c_int,
    fnp: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut redirectee: *mut WORD_DESC;
    let mut redir_fd: libc::c_int;
    let fd: libc::c_int;
    let mut redirector: libc::c_int;
    let mut r: libc::c_int;
    let mut oflags: libc::c_int = 0;
    let mut lfd: intmax_t = 0;
    let mut redirectee_word: *mut libc::c_char;
    let mut ri: r_instruction;
    let mut new_redirect: *mut REDIRECT = 0 as *mut REDIRECT;
    let sd: REDIRECTEE;
    unsafe {
        redirectee = (*redirect).redirectee.filename;
        redir_fd = (*redirect).redirectee.dest;
        redirector = (*redirect).redirector.dest;
        ri = (*redirect).instruction;
        if (*redirect).flags & RX_INTERNAL != 0 {
            flags |= RX_INTERNAL;
        }
        if TRANSLATE_REDIRECT!(ri) {
            redirectee_word = redirection_expand(redirectee);
            if redirectee_word.is_null() {
                return AMBIGUOUS_REDIRECT;
            } else if *redirectee_word.offset(0 as libc::c_int as isize) as libc::c_int
                == '-' as i32
                && *redirectee_word.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
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
                        new_redirect =
                            make_redirection(sd, r_duplicating_input, rd, 0 as libc::c_int);
                    }
                    r_duplicating_output_word => {
                        new_redirect =
                            make_redirection(sd, r_duplicating_output, rd, 0 as libc::c_int);
                    }
                    r_move_input_word => {
                        new_redirect = make_redirection(sd, r_move_input, rd, 0 as libc::c_int);
                    }
                    r_move_output_word => {
                        new_redirect = make_redirection(sd, r_move_output, rd, 0 as libc::c_int);
                    }
                    _ => {}
                }
            } else if ri as libc::c_uint == r_duplicating_output_word as libc::c_int as libc::c_uint
                && (*redirect).rflags & REDIR_VARASSIGN as libc::c_int == 0 as libc::c_int
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
                let alloca_hack: *mut libc::c_char;
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
                        .wrapping_add(strlen((*(*new_redirect).redirectee.filename).word) as u64)
                        as usize,
                );
                alloca_hack = fresh2.as_mut_ptr() as *mut libc::c_char;
                (*redirectee).word = alloca_hack;
                strcpy(
                    (*redirectee).word,
                    (*(*new_redirect).redirectee.filename).word,
                );
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
            r_output_direction | r_appending_to | r_input_direction | r_inputa_direction
            | r_err_and_out | r_append_err_and_out | r_input_output | r_output_force => {
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
                if restricted != 0 && WRITE_REDIRECT!(ri) {
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
                    return *c___errno_location();
                }
                if flags & (RX_ACTIVE as i32) != 0 {
                    if (*redirect).rflags & REDIR_VARASSIGN != 0 {
                        redirector = fcntl(fd, F_DUPFD as i32, SHELL_FD_BASE as libc::c_int);
                        let _ = *c___errno_location();
                        if redirector < 0 as libc::c_int {
                            sys_error(c_dcgettext(
                                0 as *const libc::c_char,
                                b"redirection error: cannot duplicate fd\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ));
                        }
                        REDIRECTION_ERROR!(redirector, r, fd);
                    }
                    if flags & RX_UNDOABLE as i32 != 0 && (*redirect).rflags & REDIR_VARASSIGN == 0
                    {
                        if fd != redirector
                            && fcntl(redirector, F_GETFD, 0 as libc::c_int) != -(1 as libc::c_int)
                        {
                            r = add_undo_redirect(redirector, ri, -(1 as libc::c_int));
                        } else {
                            r = add_undo_close_redirect(redirector);
                        }
                        REDIRECTION_ERROR!(r, errno, fd);
                    }
                    if redirector != 0 as libc::c_int
                        || subshell_environment & SUBSHELL_ASYNC as libc::c_int == 0 as libc::c_int
                    {
                        check_bash_input(redirector);
                    }
                    if redirector == 1 as libc::c_int && fileno(stdout) == redirector {
                        fflush(stdout);
                        c_fpurge(stdout);
                    } else if redirector == 2 as libc::c_int && fileno(stderr) == redirector {
                        fflush(stderr);
                        c_fpurge(stderr);
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
                        return *c___errno_location();
                    }

                    if ri as libc::c_uint == r_input_direction as libc::c_int as libc::c_uint
                        || ri as libc::c_uint == r_input_output as libc::c_int as libc::c_uint
                    {
                        duplicate_buffered_stream(fd, redirector);
                    }

                    if flags & RX_CLEXEC as libc::c_int != 0 && redirector > 2 as libc::c_int {
                        fcntl(redirector, 2 as libc::c_int, 1 as libc::c_int);
                    }
                }
                if fd != redirector {
                    if ri as libc::c_uint == r_input_direction as libc::c_int as libc::c_uint
                        || ri as libc::c_uint == r_inputa_direction as libc::c_int as libc::c_uint
                        || ri as libc::c_uint == r_input_output as libc::c_int as libc::c_uint
                    {
                        close_buffered_fd(fd);
                    } else {
                        close(fd);
                    }
                }
                if ri as libc::c_uint == r_err_and_out as libc::c_int as libc::c_uint
                    || ri as libc::c_uint == r_append_err_and_out as libc::c_int as libc::c_uint
                {
                    if flags & RX_ACTIVE as libc::c_int != 0 {
                        if flags & RX_UNDOABLE as libc::c_int != 0 {
                            add_undo_redirect(2 as libc::c_int, ri, -(1 as libc::c_int));
                        }
                        if dup2(1 as libc::c_int, 2 as libc::c_int) < 0 as libc::c_int {
                            return *c___errno_location();
                        }
                    }
                }
            }

            r_reading_until | r_deblank_reading_until | r_reading_string => {
                if !redirectee.is_null() {
                    fd = here_document_to_fd(redirectee, ri as libc::c_uint);
                    if fd < 0 as libc::c_int {
                        heredoc_errno = *c___errno_location();
                        return HEREDOC_REDIRECT;
                    }
                    if (*redirect).rflags & REDIR_VARASSIGN as libc::c_int != 0 {
                        redirector = fcntl(fd, F_DUPFD as i32, SHELL_FD_BASE);
                        r = *c___errno_location();
                        if redirector < 0 as libc::c_int {
                            sys_error(c_dcgettext(
                                0 as *const libc::c_char,
                                b"redirection error: cannot duplicate fd\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ));
                        }
                        REDIRECTION_ERROR!(r, errno, fd);
                    }
                    if flags & RX_ACTIVE as libc::c_int != 0 {
                        if flags & RX_UNDOABLE as libc::c_int != 0
                            && (*redirect).rflags & REDIR_VARASSIGN as libc::c_int
                                == 0 as libc::c_int
                        {
                            if fd != redirector
                                && fcntl(redirector, F_GETFD, 0 as libc::c_int)
                                    != -(1 as libc::c_int)
                            {
                                r = add_undo_redirect(redirector, ri, -(1 as libc::c_int));
                            } else {
                                r = add_undo_close_redirect(redirector);
                            }

                            REDIRECTION_ERROR!(r, errno, fd);
                        }

                        check_bash_input(redirector);

                        if (*redirect).rflags & REDIR_VARASSIGN != 0 {
                            r = redir_varassign(redirect, redirector);
                            if r < 0 as libc::c_int {
                                close(redirector);
                                close(fd);
                                return r;
                            }
                        } else if fd != redirector && dup2(fd, redirector) < 0 as libc::c_int {
                            r = *c___errno_location();
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
                if flags & RX_ACTIVE as i32 != 0 && (*redirect).rflags & REDIR_VARASSIGN != 0 {
                    redirector = fcntl(redir_fd, 0 as libc::c_int, 10 as libc::c_int);
                    let _ = *c___errno_location();
                    if redirector < 0 as libc::c_int {
                        sys_error(c_dcgettext(
                            0 as *const libc::c_char,
                            b"redirection error: cannot duplicate fd\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ));
                    }
                    REDIRECTION_ERROR!(redirector, r, -1);
                }
                if flags & RX_ACTIVE as i32 != 0 && redir_fd != redirector {
                    if flags & RX_UNDOABLE as i32 != 0
                        && (*redirect).rflags & REDIR_VARASSIGN == 0 as libc::c_int
                    {
                        if fcntl(redirector, 1 as libc::c_int, 0 as libc::c_int)
                            != -(1 as libc::c_int)
                        {
                            r = add_undo_redirect(redirector, ri, redir_fd);
                        } else {
                            r = add_undo_close_redirect(redirector);
                        }
                        REDIRECTION_ERROR!(r, errno, -1);
                    }
                    if flags & RX_UNDOABLE as i32 != 0
                        && (ri as libc::c_uint == r_move_input as libc::c_int as libc::c_uint
                            || ri as libc::c_uint == r_move_output as libc::c_int as libc::c_uint)
                    {
                        if fcntl(redirector, 1 as libc::c_int, 0 as libc::c_int)
                            != -(1 as libc::c_int)
                        {
                            r = add_undo_redirect(redir_fd, r_close_this, -(1 as libc::c_int));
                            REDIRECTION_ERROR!(r, errno, -1);
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
                        return *c___errno_location();
                    }
                    if ri as libc::c_uint == r_duplicating_input as libc::c_int as libc::c_uint
                        || ri as libc::c_uint == r_move_input as libc::c_int as libc::c_uint
                    {
                        duplicate_buffered_stream(redir_fd, redirector);
                    }
                    if (fcntl(redir_fd, F_GETFD, 0 as libc::c_int) == 1 as libc::c_int
                        || redir_fd < 2 as libc::c_int && flags & RX_INTERNAL as libc::c_int != 0
                        || flags & RX_CLEXEC as libc::c_int != 0)
                        && redirector > 2 as libc::c_int
                    {
                        fcntl(redirector, 2 as libc::c_int, 1 as libc::c_int);
                    }

                    if (*redirect).flags & RX_INTERNAL as i32 != 0
                        && (*redirect).flags & RX_SAVCLEXEC as i32 != 0
                        && redirector >= 3 as libc::c_int
                        && (redir_fd >= SHELL_FD_BASE as i32
                            || (*redirect).flags & RX_SAVEFD as i32 != 0)
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
                if flags & RX_ACTIVE as libc::c_int != 0 {
                    if (*redirect).rflags & REDIR_VARASSIGN as libc::c_int != 0 {
                        redirector = redir_varvalue(redirect);
                        if redirector < 0 as libc::c_int {
                            return AMBIGUOUS_REDIRECT;
                        }
                    }
                    // r = 0 as libc::c_int;
                    if flags & RX_UNDOABLE as i32 != 0 {
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
                    if r < 0 as libc::c_int
                        && flags & RX_INTERNAL != 0
                        && (*c___errno_location() == EIO || *c___errno_location() == ENOSPC)
                    {
                        REDIRECTION_ERROR!(r, errno, -1);
                    }
                }
            }

            r_duplicating_input_word
            | r_duplicating_output_word
            | r_move_input_word
            | r_move_output_word
            | _ => {}
        }

        return 0 as libc::c_int;
    }
}
fn add_undo_redirect(fd: libc::c_int, ri: r_instruction, fdbase: libc::c_int) -> libc::c_int {
    let mut new_fd: libc::c_int;
    let clexec_flag: libc::c_int;
    let mut savefd_flag: libc::c_int;
    let mut new_redirect: *mut REDIRECT;
    let closer: *mut REDIRECT;
    let dummy_redirect: *mut REDIRECT;
    let mut sd: REDIRECTEE = REDIRECTEE { dest: 0 };
    savefd_flag = 0 as libc::c_int;
    unsafe {
        new_fd = fcntl(
            fd,
            F_DUPFD as i32,
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
            sys_error(c_dcgettext(
                0 as *const libc::c_char,
                b"redirection error: cannot duplicate fd\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ));
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
        if clexec_flag == 0 as libc::c_int
            && fd >= 3 as libc::c_int
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
    }
    return 0 as libc::c_int;
}
fn add_undo_close_redirect(fd: libc::c_int) -> libc::c_int {
    let closer: *mut REDIRECT;
    let mut sd: REDIRECTEE = REDIRECTEE { dest: 0 };
    sd.dest = fd;
    unsafe {
        rd.dest = 0 as libc::c_int;
        closer = make_redirection(sd, r_close_this, rd, 0 as libc::c_int);
        (*closer).flags |= RX_INTERNAL;
        (*closer).next = redirection_undo_list;
        redirection_undo_list = closer;
    }
    return 0 as libc::c_int;
}
fn add_exec_redirect(dummy_redirect: *mut REDIRECT) {
    unsafe {
        (*dummy_redirect).next = exec_redirection_undo_list;
        exec_redirection_undo_list = dummy_redirect;
    }
}
fn stdin_redirection(ri: r_instruction, redirector: libc::c_int) -> libc::c_int {
    match ri as libc::c_uint {
        r_input_direction
        | r_inputa_direction
        | r_input_output
        | r_reading_until
        | r_deblank_reading_until
        | r_reading_string => return 1 as libc::c_int,
        r_duplicating_input | r_duplicating_input_word | r_close_this => {
            return (redirector == 0 as libc::c_int) as libc::c_int
        }
        r_output_direction
        | r_appending_to
        | r_duplicating_output
        | r_err_and_out
        | r_append_err_and_out
        | r_output_force
        | r_duplicating_output_word
        | r_move_input
        | r_move_output
        | r_move_input_word
        | r_move_output_word => return 0 as libc::c_int,
        _ => {}
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub fn stdin_redirects(redirs: *mut REDIRECT) -> libc::c_int {
    let mut rp: *mut REDIRECT;
    let mut n: libc::c_int;
    n = 0 as libc::c_int;
    rp = redirs;

    while !rp.is_null() {
        unsafe {
            if (*rp).rflags & REDIR_VARASSIGN as libc::c_int == 0 as libc::c_int {
                n += stdin_redirection((*rp).instruction as libc::c_uint, (*rp).redirector.dest);
            }
            rp = (*rp).next;
        }
    }

    return n;
}
fn redir_varassign(redir: *mut REDIRECT, fd: libc::c_int) -> libc::c_int {
    let w: *mut WORD_DESC;
    let v: *mut SHELL_VAR;
    unsafe {
        w = (*redir).redirector.filename;
        v = bind_var_to_int((*w).word, fd as intmax_t);
        if v.is_null() || readonly_p!(v) as libc::c_int != 0 || noassign_p!(v) as libc::c_int != 0 {
            return BADVAR_REDIRECT;
        }
        stupidly_hack_special_variables((*w).word);
    }
    return 0 as libc::c_int;
}

fn redir_varvalue(redir: *mut REDIRECT) -> libc::c_int {
    let mut v: *mut SHELL_VAR;
    let val: *mut libc::c_char;
    let mut w: *mut libc::c_char;
    let mut vmax: intmax_t = 0;
    let i: libc::c_int;
    let mut sub: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut vr: libc::c_int;
    unsafe {
        w = (*(*redir).redirector.filename).word;
    }
    vr = valid_array_reference(w, 0 as libc::c_int);
    if vr != 0 {
        v = array_variable_part(w, 0 as libc::c_int, &mut sub, &mut len);
    } else {
        v = find_variable(w);
        if v.is_null() {
            v = find_variable_last_nameref(w, 0 as libc::c_int);
            if unsafe { !v.is_null() && (*v).attributes & 0x800 as libc::c_int != 0 } {
                unsafe {
                    w = (*v).value;
                }
                vr = valid_array_reference(w, 0 as libc::c_int);
                if vr != 0 {
                    v = array_variable_part(w, 0 as libc::c_int, &mut sub, &mut len);
                } else {
                    v = find_variable(w);
                }
            }
        }
    }
    if unsafe { v.is_null() || invisible_p!(v) as libc::c_int != 0 } {
        return -(1 as libc::c_int);
    }

    if unsafe { vr != 0 && (array_p!(v) as libc::c_int != 0 || assoc_p!(v) as libc::c_int != 0) } {
        val = get_array_value(
            w,
            0 as libc::c_int,
            0 as *mut libc::c_void as *mut libc::c_int,
            0 as *mut arrayind_t,
        );
    } else {
        val = get_variable_value(v);
    }
    if unsafe { val.is_null() || *val as libc::c_int == 0 as libc::c_int } {
        return -(1 as libc::c_int);
    }
    if legal_number(val, &mut vmax) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    i = vmax as libc::c_int;
    return i;
}
