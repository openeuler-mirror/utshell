use crate::general::{extract_colon_unit, full_pathname, legal_number};
use crate::src_common::*;
use crate::subst::expand_string_to_string;
use crate::variables::{bind_variable, unbind_variable};

#[no_mangle]
pub fn time_to_check_mail() -> libc::c_int {
    let mut seconds: intmax_t = 0;
    let temp: *mut libc::c_char;
    let now: time_t;
    unsafe {
        temp = get_string_value(b"MAILCHECK\0" as *const u8 as *const libc::c_char);
        if temp.is_null()
            || legal_number(temp, &mut seconds) == 0 as libc::c_int
            || seconds < 0 as libc::c_int as intmax_t
        {
            return 0 as libc::c_int;
        }
        now = time(0 as *mut time_t);
        return (seconds == 0 as libc::c_int as intmax_t || now - last_time_mail_checked >= seconds)
            as libc::c_int;
    }
}

#[no_mangle]
pub fn reset_mail_timer() {
    unsafe {
        last_time_mail_checked = time(0 as *mut time_t);
    }
}

fn find_mail_file(file: *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int;
    i = 0 as libc::c_int;
    unsafe {
        while i < mailfiles_count {
            if *((**mailfiles.offset(i as isize)).name).offset(0 as libc::c_int as isize)
                as libc::c_int
                == *file.offset(0 as libc::c_int as isize) as libc::c_int
                && strcmp((**mailfiles.offset(i as isize)).name, file) == 0 as libc::c_int
            {
                return i;
            }
            i += 1;
        }
    }
    return -(1 as libc::c_int);
}

fn init_mail_file(i: libc::c_int) {
    unsafe {
        let ref mut fresh0 = (**mailfiles.offset(i as isize)).mod_time;
        *fresh0 = if last_time_mail_checked != 0 {
            last_time_mail_checked
        } else {
            shell_start_time
        };
        (**mailfiles.offset(i as isize)).access_time = *fresh0;
        (**mailfiles.offset(i as isize)).file_size = 0 as libc::c_int as off_t;
        (**mailfiles.offset(i as isize)).flags = 0 as libc::c_int;
    }
}

fn update_mail_file(i: libc::c_int) {
    let file: *mut libc::c_char;
    let mut finfo: crate::src_common::stat = crate::src_common::stat {
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
        st_atim: crate::src_common::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: crate::src_common::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: crate::src_common::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    unsafe {
        file = (**mailfiles.offset(i as isize)).name;
        if mailstat(file, &mut finfo) == 0 as libc::c_int {
            UPDATE_MAIL_FILE!(i, finfo);
        } else {
            RESET_MAIL_FILE!(i);
        };
    }
}

fn add_mail_file(file: *mut libc::c_char, msg: *mut libc::c_char) -> libc::c_int {
    let mut finfo: crate::src_common::stat = crate::src_common::stat {
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
        st_atim: crate::src_common::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: crate::src_common::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: crate::src_common::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let filename: *mut libc::c_char;
    let mut i: libc::c_int;
    filename = full_pathname(file);
    i = find_mail_file(filename);
    unsafe {
        if i >= 0 as libc::c_int {
            if mailstat(filename, &mut finfo) == 0 as libc::c_int {
                UPDATE_MAIL_FILE!(i, finfo);
            }
            free(filename as *mut libc::c_void);
            return i;
        }
        let fresh2 = mailfiles_count;
        mailfiles_count = mailfiles_count + 1;
        i = fresh2;
        mailfiles = libc::realloc(
            mailfiles as *mut libc::c_void,
            (mailfiles_count as libc::size_t)
                .wrapping_mul(::core::mem::size_of::<*mut FILEINFO>() as libc::size_t),
        ) as *mut *mut FILEINFO;
        let ref mut fresh3 = *mailfiles.offset(i as isize);
        *fresh3 = alloc_mail_file(filename, msg);
        init_mail_file(i);
    }
    return i;
}

#[no_mangle]
pub fn reset_mail_files() {
    unsafe {
        let mut i: libc::c_int;
        i = 0 as libc::c_int;
        while i < mailfiles_count {
            RESET_MAIL_FILE!(i);
            i += 1;
        }
    }
}

fn alloc_mail_file(filename: *mut libc::c_char, msg: *mut libc::c_char) -> *mut FILEINFO {
    unsafe {
        let mf: *mut FILEINFO;
        mf = libc::malloc(::core::mem::size_of::<FILEINFO>() as libc::size_t) as *mut FILEINFO;
        (*mf).name = filename;
        (*mf).msg = if !msg.is_null() {
            strcpy(
                libc::malloc(
                    (1 as libc::c_int as libc::size_t).wrapping_add(strlen(msg) as libc::size_t),
                ) as *mut libc::c_char,
                msg,
            )
        } else {
            0 as *mut libc::c_void as *mut libc::c_char
        };
        (*mf).flags = 0 as libc::c_int;
        return mf;
    }
}

fn dispose_mail_file(mf: *mut FILEINFO) {
    unsafe {
        free((*mf).name as *mut libc::c_void);
        if !((*mf).msg).is_null() {
            free((*mf).msg as *mut libc::c_void);
        }
        (*mf).msg = 0 as *mut libc::c_char;
        free(mf as *mut libc::c_void);
    }
}

#[no_mangle]
pub fn free_mail_files() {
    let mut i: libc::c_int;
    i = 0 as libc::c_int;
    unsafe {
        while i < mailfiles_count {
            dispose_mail_file(*mailfiles.offset(i as isize));
            i += 1;
        }
        if !mailfiles.is_null() {
            free(mailfiles as *mut libc::c_void);
        }
        mailfiles_count = 0 as libc::c_int;
        mailfiles = 0 as *mut libc::c_void as *mut *mut FILEINFO;
    }
}

#[no_mangle]
pub fn init_mail_dates() {
    unsafe {
        if mailfiles.is_null() {
            remember_mail_dates();
        }
    }
}

fn file_mod_date_changed(i: libc::c_int) -> libc::c_int {
    let mtime_0: time_t;
    let mut finfo: crate::src_common::stat = crate::src_common::stat {
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
        st_atim: crate::src_common::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: crate::src_common::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: crate::src_common::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    unsafe {
        let file: *mut libc::c_char;
        file = (**mailfiles.offset(i as isize)).name;
        mtime_0 = (**mailfiles.offset(i as isize)).mod_time;
        if mailstat(file, &mut finfo) != 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        if finfo.st_size > 0 as libc::c_int as libc::c_long {
            return (mtime_0 < finfo.st_mtim.tv_sec) as libc::c_int;
        }
        if finfo.st_size == 0 as libc::c_int as libc::c_long
            && (**mailfiles.offset(i as isize)).file_size > 0 as libc::c_int as off_t
        {
            UPDATE_MAIL_FILE!(i, finfo);
        }
    }
    return 0 as libc::c_int;
}

fn file_access_date_changed(i: libc::c_int) -> libc::c_int {
    let atime_0: time_t;
    let mut finfo: crate::src_common::stat = crate::src_common::stat {
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
        st_atim: crate::src_common::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: crate::src_common::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: crate::src_common::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let file: *mut libc::c_char;
    unsafe {
        file = (**mailfiles.offset(i as isize)).name;
        atime_0 = (**mailfiles.offset(i as isize)).access_time;
        if mailstat(file, &mut finfo) != 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        if finfo.st_size > 0 as libc::c_int as libc::c_long {
            return (atime_0 < finfo.st_atim.tv_sec) as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}

fn file_has_grown(i: libc::c_int) -> libc::c_int {
    let size: off_t;
    let mut finfo: crate::src_common::stat = crate::src_common::stat {
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
        st_atim: crate::src_common::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: crate::src_common::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: crate::src_common::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    unsafe {
        let file: *mut libc::c_char;
        file = (**mailfiles.offset(i as isize)).name;
        size = (**mailfiles.offset(i as isize)).file_size;
        return (mailstat(file, &mut finfo) == 0 as libc::c_int && finfo.st_size > size)
            as libc::c_int;
    }
}

fn parse_mailpath_spec(str: *mut libc::c_char) -> *mut libc::c_char {
    let mut s: *mut libc::c_char;
    let mut pass_next: libc::c_int;
    s = str;
    unsafe {
        pass_next = 0 as libc::c_int;
        while !s.is_null() && *s as libc::c_int != 0 {
            if pass_next != 0 {
                pass_next = 0 as libc::c_int;
            } else if *s as libc::c_int == '\\' as i32 {
                pass_next += 1;
            } else if *s as libc::c_int == '?' as i32 || *s as libc::c_int == '%' as i32 {
                return s;
            }
            s = s.offset(1);
        }
    }
    return 0 as *mut libc::c_void as *mut libc::c_char;
}

#[no_mangle]
pub fn make_default_mailpath() -> *mut libc::c_char {
    unsafe {
        let mp: *mut libc::c_char;
        get_current_user_info();
        mp = libc::malloc(
            (2 as libc::c_int as usize)
                .wrapping_add(strlen(DEFAULT_MAIL_DIRECTORY) as usize)
                .wrapping_add(strlen(current_user.user_name) as usize),
        ) as *mut libc::c_char;
        strcpy(mp, DEFAULT_MAIL_DIRECTORY);
        *mp.offset((strlen(DEFAULT_MAIL_DIRECTORY)).wrapping_sub(1) as isize) =
            '/' as i32 as libc::c_char;
        strcpy(
            mp.offset(strlen(DEFAULT_MAIL_DIRECTORY) as libc::c_ulong as isize),
            current_user.user_name,
        );
        return mp;
    }
}

#[no_mangle]
pub fn remember_mail_dates() {
    let mut mailpaths: *mut libc::c_char;
    let mut mailfile: *mut libc::c_char;
    let mut mp: *mut libc::c_char;
    let mut i: libc::c_int = 0 as libc::c_int;
    unsafe {
        mailpaths = get_string_value(b"MAILPATH\0" as *const u8 as *const libc::c_char);
        if mailpaths.is_null() && {
            mailpaths = get_string_value(b"MAIL\0" as *const u8 as *const libc::c_char);
            !mailpaths.is_null()
        } {
            add_mail_file(mailpaths, 0 as *mut libc::c_void as *mut libc::c_char);
            return;
        }
        if mailpaths.is_null() {
            mailpaths = make_default_mailpath();
            if !mailpaths.is_null() {
                add_mail_file(mailpaths, 0 as *mut libc::c_void as *mut libc::c_char);
                free(mailpaths as *mut libc::c_void);
            }
            return;
        }
        loop {
            mailfile = extract_colon_unit(mailpaths, &mut i);
            if mailfile.is_null() {
                break;
            }
            mp = parse_mailpath_spec(mailfile);
            if !mp.is_null() && *mp as libc::c_int != 0 {
                let fresh5 = mp;
                mp = mp.offset(1);
                *fresh5 = '\0' as i32 as libc::c_char;
            }
            add_mail_file(mailfile, mp);
            free(mailfile as *mut libc::c_void);
        }
    }
}

#[no_mangle]
pub fn check_mail() {
    let mut current_mail_file: *mut libc::c_char;
    let mut message: *mut libc::c_char;
    let mut i: libc::c_int;
    let mut use_user_notification: libc::c_int;
    let mut dollar_underscore: *mut libc::c_char;
    let mut temp: *mut libc::c_char;
    unsafe {
        dollar_underscore = get_string_value(b"_\0" as *const u8 as *const libc::c_char);
        if !dollar_underscore.is_null() {
            dollar_underscore = strcpy(
                libc::malloc(
                    (1 as libc::c_int as usize).wrapping_add(strlen(dollar_underscore) as usize),
                ) as *mut libc::c_char,
                dollar_underscore,
            );
        }
        let mut _current_block_20: u64;
        i = 0 as libc::c_int;
        while i < mailfiles_count {
            current_mail_file = (**mailfiles.offset(i as isize)).name;
            if !(*current_mail_file as libc::c_int == '\0' as i32) {
                if file_mod_date_changed(i) != 0 {
                    let file_is_bigger: libc::c_int;
                    use_user_notification = ((**mailfiles.offset(i as isize)).msg
                        != 0 as *mut libc::c_void as *mut libc::c_char)
                        as libc::c_int;
                    message = if !((**mailfiles.offset(i as isize)).msg).is_null() {
                        (**mailfiles.offset(i as isize)).msg
                    } else {
                        dcgettext(
                            0 as *const libc::c_char,
                            b"You have mail in $_\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        )
                    };
                    bind_variable(
                        b"_\0" as *const u8 as *const libc::c_char,
                        current_mail_file,
                        0 as libc::c_int,
                    );
                    file_is_bigger = file_has_grown(i);
                    update_mail_file(i);
                    if (**mailfiles.offset(i as isize)).access_time
                        >= (**mailfiles.offset(i as isize)).mod_time
                        && file_is_bigger == 0
                    {
                        i += 1;
                        continue;
                    } else {
                        if use_user_notification == 0 as libc::c_int
                            && (**mailfiles.offset(i as isize)).access_time
                                < (**mailfiles.offset(i as isize)).mod_time
                            && file_is_bigger != 0
                        {
                            message = dcgettext(
                                0 as *const libc::c_char,
                                b"You have new mail in $_\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            );
                        }
                        temp = expand_string_to_string(message, 0x1 as libc::c_int);
                        if !temp.is_null() {
                            puts(temp);
                            free(temp as *mut libc::c_void);
                        } else {
                            putc('\n' as i32, stdout);
                        }
                    }
                    if mail_warning != 0 && file_access_date_changed(i) != 0 {
                        update_mail_file(i);
                        printf(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"The mail in %s has been read\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            current_mail_file,
                        );
                    }
                }
            }
            i += 1;
        }
        if !dollar_underscore.is_null() {
            bind_variable(
                b"_\0" as *const u8 as *const libc::c_char,
                dollar_underscore,
                0 as libc::c_int,
            );
            free(dollar_underscore as *mut libc::c_void);
        } else {
            unbind_variable(b"_\0" as *const u8 as *const libc::c_char);
        };
    }
}
