//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later


#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

//#![feature(extern_types)]
extern "C" {
    static mut stdout: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn free(_: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn bind_variable(
        _: *const libc::c_char,
        _: *mut libc::c_char,
        _: libc::c_int,
    ) -> *mut r_bash::SHELL_VAR;
    fn get_string_value(_: *const libc::c_char) -> *mut libc::c_char;
    fn unbind_variable(_: *const libc::c_char) -> libc::c_int;

    fn legal_number(_: *const libc::c_char, _: *mut intmax_t) -> libc::c_int;
    fn full_pathname(_: *mut libc::c_char) -> *mut libc::c_char;
    fn extract_colon_unit(
        _: *mut libc::c_char,
        _: *mut libc::c_int,
    ) -> *mut libc::c_char;
    fn expand_string_to_string(
        _: *mut libc::c_char,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    fn get_current_user_info();
    static mut current_user: user_info;
    static mut shell_start_time: time_t;
    fn mailstat(_: *const libc::c_char, _: *mut stat) -> libc::c_int;
}
pub type size_t = libc::c_ulong;

pub static MBOX_INITIALIZED:libc::c_int = 0x01;

#[macro_export] 
macro_rules! UPDATE_MAIL_FILE { 
    ($i:expr, $finfo:expr) => { 
        (**mailfiles.offset($i as isize)).access_time = $finfo.st_atim.tv_sec;
        (**mailfiles.offset($i as isize)).mod_time = $finfo.st_mtim.tv_sec;
        (**mailfiles.offset($i as isize)).file_size = $finfo.st_size;
        (**mailfiles.offset($i as isize)).flags |= MBOX_INITIALIZED;
    }; 
}

#[macro_export] 
macro_rules! RESET_MAIL_FILE { 
    ($i:expr) => {
        let ref mut fresh1 = (**mailfiles.offset($i as isize)).mod_time;
        *fresh1 = 0 as libc::c_int as time_t;
        (**mailfiles.offset($i as isize)).access_time = *fresh1;
        (**mailfiles.offset($i as isize)).file_size = 0 as libc::c_int as off_t;
        (**mailfiles.offset($i as isize)).flags = 0 as libc::c_int;
    }
}

pub const DEFAULT_MAIL_DIRECTORY: *const libc::c_char = b"/var/mail\0" as *const u8 as *const libc::c_char;

#[derive(Copy, Clone)]
#[repr(C)] 
pub struct FILE{ _private:[u8;0]}

pub type __syscall_slong_t = libc::c_long;

pub type off_t = libc::c_long;
pub type gid_t = libc::c_uint;
pub type uid_t = libc::c_uint;

pub type time_t = libc::c_long;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: libc::c_long,
    pub tv_nsec: libc::c_long,
}

pub type intmax_t = libc::c_long;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: libc::c_ulong,
    pub st_ino: libc::c_ulong,
    pub st_nlink: libc::c_ulong,
    pub st_mode: libc::c_uint,
    pub st_uid: libc::c_uint,
    pub st_gid: libc::c_uint,
    pub __pad0: libc::c_int,
    pub st_rdev: libc::c_ulong,
    pub st_size: libc::c_long,
    pub st_blksize: libc::c_long,
    pub st_blocks: libc::c_long,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}

pub type arrayind_t = intmax_t;

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
pub struct FILEINFO {
    pub name: *mut libc::c_char,
    pub msg: *mut libc::c_char,
    pub access_time: time_t,
    pub mod_time: time_t,
    pub file_size: off_t,
    pub flags: libc::c_int,
}

static mut mailfiles: *mut *mut FILEINFO = 0 as *const libc::c_void as *mut libc::c_void
    as *mut *mut FILEINFO;
static mut mailfiles_count: libc::c_int = 0;
static mut last_time_mail_checked: time_t = 0 as libc::c_int as time_t;
#[no_mangle]
pub static mut mail_warning: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn time_to_check_mail() -> libc::c_int {
    let mut seconds: intmax_t = 0;
    let temp: *mut libc::c_char;
    let now: time_t;
    temp = get_string_value(b"MAILCHECK\0" as *const u8 as *const libc::c_char);
    if temp.is_null() || legal_number(temp, &mut seconds) == 0 as libc::c_int
        || seconds < 0 as libc::c_int as intmax_t
    {
        return 0 as libc::c_int;
    }
    now = time(0 as *mut time_t);
    return (seconds == 0 as libc::c_int as intmax_t
        || now - last_time_mail_checked >= seconds) as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn reset_mail_timer() {
    last_time_mail_checked = time(0 as *mut time_t);
}

unsafe extern "C" fn find_mail_file(file: *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int;
    i = 0 as libc::c_int;
    while i < mailfiles_count {
        if *((**mailfiles.offset(i as isize)).name).offset(0 as libc::c_int as isize)
            as libc::c_int == *file.offset(0 as libc::c_int as isize) as libc::c_int
            && strcmp((**mailfiles.offset(i as isize)).name, file) == 0 as libc::c_int
        {
            return i;
        }
        i += 1;
    }
    return -(1 as libc::c_int);
}

unsafe extern "C" fn init_mail_file(i: libc::c_int) {
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

unsafe extern "C" fn update_mail_file(i: libc::c_int) {
    let file: *mut libc::c_char;
    let mut finfo: stat = stat {
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
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    file = (**mailfiles.offset(i as isize)).name;
    if mailstat(file, &mut finfo) == 0 as libc::c_int {
        UPDATE_MAIL_FILE!(i, finfo);
    } else {
        RESET_MAIL_FILE!(i);

    };
}

unsafe extern "C" fn add_mail_file(
    file: *mut libc::c_char,
    msg: *mut libc::c_char,
) -> libc::c_int {
    let mut finfo: stat = stat {
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
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let filename: *mut libc::c_char;
    let mut i: libc::c_int;
    filename = full_pathname(file);
    i = find_mail_file(filename);
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
    return i;
}

#[no_mangle]
pub unsafe extern "C" fn reset_mail_files() {
    let mut i: libc::c_int;
    i = 0 as libc::c_int;
    while i < mailfiles_count {
        RESET_MAIL_FILE!(i);
        i += 1;
    }
}

unsafe extern "C" fn alloc_mail_file(
    filename: *mut libc::c_char,
    msg: *mut libc::c_char,
) -> *mut FILEINFO {
    let mf: *mut FILEINFO;
    mf = libc::malloc(::core::mem::size_of::<FILEINFO>() as libc::size_t) as *mut FILEINFO;
    (*mf).name = filename;
    (*mf)
        .msg = if !msg.is_null() {
        strcpy(
            libc::malloc((1 as libc::c_int as libc::size_t).wrapping_add(strlen(msg)as libc::size_t))
                as *mut libc::c_char,
            msg,
        )
    } else {
        0 as *mut libc::c_void as *mut libc::c_char
    };
    (*mf).flags = 0 as libc::c_int;
    return mf;
}

unsafe extern "C" fn dispose_mail_file(mf: *mut FILEINFO) {
    free((*mf).name as *mut libc::c_void);
    if !((*mf).msg).is_null() {
        free((*mf).msg as *mut libc::c_void);
    }
    (*mf).msg = 0 as *mut libc::c_char;
    free(mf as *mut libc::c_void);
}

#[no_mangle]
pub unsafe extern "C" fn free_mail_files() {
    let mut i: libc::c_int;
    i = 0 as libc::c_int;
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

#[no_mangle]
pub unsafe extern "C" fn init_mail_dates() {
    if mailfiles.is_null() {
        remember_mail_dates();
    }
}

unsafe extern "C" fn file_mod_date_changed(i: libc::c_int) -> libc::c_int {
    let mtime_0: time_t;
    let mut finfo: stat = stat {
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
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
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
    return 0 as libc::c_int;
}

unsafe extern "C" fn file_access_date_changed(i: libc::c_int) -> libc::c_int {
    let atime_0: time_t;
    let mut finfo: stat = stat {
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
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let file: *mut libc::c_char;
    file = (**mailfiles.offset(i as isize)).name;
    atime_0 = (**mailfiles.offset(i as isize)).access_time;
    if mailstat(file, &mut finfo) != 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if finfo.st_size > 0 as libc::c_int as libc::c_long {
        return (atime_0 < finfo.st_atim.tv_sec) as libc::c_int;
    }
    return 0 as libc::c_int;
}

unsafe extern "C" fn file_has_grown(i: libc::c_int) -> libc::c_int {
    let size: off_t;
    let mut finfo: stat = stat {
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
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let file: *mut libc::c_char;
    file = (**mailfiles.offset(i as isize)).name;
    size = (**mailfiles.offset(i as isize)).file_size;
    return (mailstat(file, &mut finfo) == 0 as libc::c_int && finfo.st_size > size)
        as libc::c_int;
}

unsafe extern "C" fn parse_mailpath_spec(
    str: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char;
    let mut pass_next: libc::c_int;
    s = str;
    pass_next = 0 as libc::c_int;
    while !s.is_null() && *s as libc::c_int != 0 {
        if pass_next != 0 {
            pass_next = 0 as libc::c_int;
        } else if *s as libc::c_int == '\\' as i32 {
            pass_next += 1;
        } else if *s as libc::c_int == '?' as i32 || *s as libc::c_int == '%' as i32 {
            return s
        }
        s = s.offset(1);
    }
    return 0 as *mut libc::c_void as *mut libc::c_char;
}

#[no_mangle]
pub unsafe extern "C" fn make_default_mailpath() -> *mut libc::c_char {
    let mp: *mut libc::c_char;
    get_current_user_info();
    mp = libc::malloc(
        (2 as libc::c_int as usize)
            .wrapping_add(strlen(DEFAULT_MAIL_DIRECTORY) as usize)
            .wrapping_add(strlen(current_user.user_name) as usize),
    ) as *mut libc::c_char;
    strcpy(mp, DEFAULT_MAIL_DIRECTORY);
    *mp
        .offset(
            (strlen(DEFAULT_MAIL_DIRECTORY) )
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = '/' as i32 as libc::c_char;
    strcpy(
        mp
            .offset(
                strlen(DEFAULT_MAIL_DIRECTORY) as libc::c_ulong as isize,
            ),
        current_user.user_name,
    );
    return mp;
}

#[no_mangle]
pub unsafe extern "C" fn remember_mail_dates() {
    let mut mailpaths: *mut libc::c_char;
    let mut mailfile: *mut libc::c_char;
    let mut mp: *mut libc::c_char;
    let mut i: libc::c_int = 0 as libc::c_int;
    mailpaths = get_string_value(b"MAILPATH\0" as *const u8 as *const libc::c_char);
    if mailpaths.is_null()
        && {
            mailpaths = get_string_value(b"MAIL\0" as *const u8 as *const libc::c_char);
            !mailpaths.is_null()
        }
    {
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
    };
}

#[no_mangle]
pub unsafe extern "C" fn check_mail() {
    let mut current_mail_file: *mut libc::c_char;
    let mut message: *mut libc::c_char;
    let mut i: libc::c_int;
    let mut use_user_notification: libc::c_int;
    let mut dollar_underscore: *mut libc::c_char;
    let mut temp: *mut libc::c_char;
    dollar_underscore = get_string_value(b"_\0" as *const u8 as *const libc::c_char);
    if !dollar_underscore.is_null() {
        dollar_underscore = strcpy(
            libc::malloc(
                (1 as libc::c_int  as usize)
                    .wrapping_add(strlen(dollar_underscore) as usize),
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
                    != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
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
                    >= (**mailfiles.offset(i as isize)).mod_time && file_is_bigger == 0
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
                            b"You have new mail in $_\0" as *const u8
                                as *const libc::c_char,
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
