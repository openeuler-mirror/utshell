//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later


use libc::{c_void,c_char, c_int,__errno_location};
use r_bash::*;
use r_shell::savestring;
use r_execute_cmd::{STREQ,FREE,QUIT};

extern "C" {
    fn strmatch( _: *mut libc::c_char, _: *mut libc::c_char, _: libc::c_int,)-> libc::c_int;
    fn same_file(_: *const libc::c_char,_: *const libc::c_char,_: *mut stat,_: *mut stat) -> libc::c_int;
}

static mut  file_to_lose_on:*mut c_char = 0 as *mut c_char;
#[no_mangle]
pub static mut check_hashed_filenames:c_int = CHECKHASH_DEFAULT as c_int;
#[no_mangle]
pub static mut dot_found_in_search:c_int = 0;

static mut execignore:ignorevar = {
    let init = ignorevar {
        varname: b"EXECIGNORE\0" as *const u8 as *mut libc::c_char,
        ignores: 0 as *mut ign,
        num_ignores: 0 ,
        last_ignoreval: 0 as *mut libc::c_char,
        item_func: None,
    };
    init
}; 

#[no_mangle]
pub unsafe extern "C" fn setup_exec_ignore(_varname: *mut libc::c_char) {
    setup_ignore_patterns(&mut execignore);
}

#[macro_export]
macro_rules! FNM_EXTMATCH {
    () => {
        1 << 5
    };
}

#[macro_export]
macro_rules! FNM_CASEFOLD {
    () => {
        1 << 4
    };
}

#[macro_export]
macro_rules! FNM_NOMATCH {
    () => {
        1  
    };
}

#[macro_export]
macro_rules! FNMATCH_EXTFLAG {
    () => {
        if extended_glob != 0 {
            FNM_EXTMATCH!()
        } else {
            0 
        }
    };
}

unsafe extern "C" fn exec_name_should_ignore(
    name: *const libc::c_char,
) -> libc::c_int {
    let mut p: *mut ign = 0 as *mut ign;

    p = execignore.ignores;
    while !p.is_null() && !((*p).val).is_null() {
        if strmatch(
            (*p).val,
            name as *mut libc::c_char,
            FNMATCH_EXTFLAG!()| FNM_CASEFOLD!()
        ) != FNM_NOMATCH!()
        {
            return 1 ;
        }

        p = p.offset(1);
    }
    return 0 ;
}


#[macro_export]
macro_rules! __S_IFDIR {
    () => {
        0o40000 
    };
}

#[macro_export]
macro_rules! __S_IFMT {
    () => {
        0o170000 
    };
}

#[macro_export]
macro_rules! __S_ISTYPE {
    ($mode:expr, $mask:expr) => {
        $mode & __S_IFMT!() == $mask
    };
}

#[macro_export]
macro_rules! S_ISDIR {
    ($mode:expr) => {
        __S_ISTYPE!($mode, __S_IFDIR!())
    };
}

#[no_mangle]
pub unsafe extern "C" fn file_status(name: *const libc::c_char) -> libc::c_int {
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
    let mut r: libc::c_int = 0;

    if stat(name, &mut finfo) < 0 {
        return 0 ;
    }

    if S_ISDIR!(finfo.st_mode)
    {
        return FS_EXISTS as libc::c_int | FS_DIRECTORY as libc::c_int;
    }
    r = FS_EXISTS as libc::c_int;
    if exec_name_should_ignore(name) == 0  
        && eaccess(name, X_OK as libc::c_int) == 0 
    {
        r |= FS_EXECABLE as libc::c_int;
    }
    if eaccess(name, R_OK as libc::c_int) == 0 {
        r |= FS_READABLE as libc::c_int;
    }
    return r;
}

#[macro_export]
macro_rules! EISDIR {
    () => {
        21
    };
}

#[macro_export]
macro_rules! errno {
    () => {
        *__errno_location()
    };
}

#[no_mangle]
pub unsafe extern "C" fn executable_file(file: *const libc::c_char) -> libc::c_int {
    let mut s: libc::c_int = 0;

    s = file_status(file);
    if s & FS_DIRECTORY as libc::c_int != 0 {
        errno!() = EISDIR!();
    }
    return (s & FS_EXECABLE as libc::c_int != 0 && s & FS_DIRECTORY as libc::c_int == 0 as libc::c_int)
        as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn is_directory(file: *const libc::c_char) -> libc::c_int {
    return file_status(file) & FS_DIRECTORY as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn executable_or_directory(
    file: *const libc::c_char,
) -> libc::c_int {
    let mut s: libc::c_int = 0;

    s = file_status(file);
    return (s & FS_EXECABLE as libc::c_int != 0 || s & FS_DIRECTORY as libc::c_int != 0) as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn find_user_command(
    name: *const libc::c_char,
) -> *mut libc::c_char {
    return find_user_command_internal(name, FS_EXEC_PREFERRED as libc::c_int | FS_NODIRS as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn find_path_file(
    name: *const libc::c_char,
) -> *mut libc::c_char {
    return find_user_command_internal(name, FS_READABLE as libc::c_int);
}

#[macro_export]
macro_rules! value_cell {
    ($var:expr) => {
        (*$var).value
    };
}

unsafe extern "C" fn _find_user_command_internal(
    name: *const libc::c_char,
    flags: libc::c_int,
) -> *mut libc::c_char {
    let mut path_list: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    
    var = find_variable_tempenv(b"PATH\0" as *const u8 as *const libc::c_char);
    if !var.is_null() {
        path_list = value_cell!(var);
    } else {
        path_list = 0 as *mut libc::c_char;
    }

    if path_list.is_null() || *path_list as libc::c_int == '\u{0}' as i32 {
        return savestring!(name);
    }
    cmd = find_user_command_in_path(name, path_list, flags);
    return cmd;
}
unsafe extern "C" fn find_user_command_internal(
    name: *const libc::c_char,
    flags: libc::c_int,
) -> *mut libc::c_char {
    return _find_user_command_internal(name, flags);
}



unsafe extern "C" fn get_next_path_element(
    path_list: *mut libc::c_char,
    path_index_pointer: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    
    path = extract_colon_unit(path_list, path_index_pointer);
    if path.is_null() {
        return path;
    }
    if *path as libc::c_int == '\u{0}' as i32 {
        free(path as *mut libc::c_void);
        path = savestring!(b".\0" as *const u8 as *const libc::c_char);
    }
    return path;
}

#[macro_export]
macro_rules! att_tempvar {
    () => {
        0x0100000
    };
}

#[macro_export]
macro_rules! tempvar_p {
    ($var:expr) => {
        (*$var).attributes & att_tempvar!()
    };
}

#[no_mangle]
pub unsafe extern "C" fn search_for_command(
    pathname: *const libc::c_char,
    flags: libc::c_int,
) -> *mut libc::c_char {
    let mut hashed_file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut command: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path_list: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp_path: libc::c_int = 0;
    let mut st: libc::c_int = 0;
    let mut path: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    
    command = 0 as *mut libc::c_char;
    hashed_file = command;
    
    path = find_variable_tempenv(b"PATH\0" as *const u8 as *const libc::c_char);
    temp_path = (!path.is_null() && tempvar_p!(path) != 0)as libc::c_int;
   
    if temp_path == 0 && absolute_program(pathname) == 0 {
        hashed_file = phash_search(pathname);
    }
    if !hashed_file.is_null() && (posixly_correct != 0 || check_hashed_filenames != 0) {
        st = file_status(hashed_file);
        if st & (FS_EXISTS as libc::c_int | FS_EXECABLE as libc::c_int)
            != FS_EXISTS as libc::c_int | FS_EXECABLE as libc::c_int
        {
            phash_remove(pathname);
            free(hashed_file as *mut libc::c_void);
            hashed_file = 0 as *mut libc::c_char;
        }
    }

    if !hashed_file.is_null() {
        command = hashed_file;
    } else if absolute_program(pathname) != 0 {
        command = savestring!(pathname);
    } else {
        if flags & CMDSRCH_STDPATH as libc::c_int != 0 {
            path_list = conf_standard_path();
        } else if temp_path != 0 || !path.is_null() {
            path_list = (*path).value;
        } else {
            path_list = 0 as *mut libc::c_char;
        }
        command = find_user_command_in_path(
            pathname,
            path_list,
            FS_EXEC_PREFERRED as libc::c_int | FS_NODIRS as libc::c_int,
        );

        if !command.is_null() && hashing_enabled != 0 && temp_path == 0 
            && flags & CMDSRCH_HASH as libc::c_int != 0
        {
            if STREQ!(command, pathname)
            {
                st = file_status(command);
                if st & FS_EXECABLE as libc::c_int != 0 {
                    phash_insert(
                        pathname as *mut libc::c_char,
                        command,
                        dot_found_in_search,
                        1 ,
                    );
                }
            } else if posixly_correct != 0 {
                st = file_status(command);
                if st & FS_EXECABLE as libc::c_int != 0 {
                    phash_insert(
                        pathname as *mut libc::c_char,
                        command,
                        dot_found_in_search,
                        1 ,
                    );
                }
            } else {
                phash_insert(
                    pathname as *mut libc::c_char,
                    command,
                    dot_found_in_search,
                    1 ,
                );
            }
        }
        if flags & CMDSRCH_STDPATH as libc::c_int != 0 {
            free(path_list as *mut libc::c_void);
        }
    }
    return command;
}


#[no_mangle]
pub unsafe extern "C" fn user_command_matches(
    name: *const libc::c_char,
    flags: libc::c_int,
    state: libc::c_int,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut path_index: libc::c_int = 0;
    let mut name_len: libc::c_int = 0;
    let mut path_list: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path_element: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut match_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dotinfo: stat = stat {
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
    static mut match_list: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
        as *mut *mut libc::c_char;
    static mut match_list_size: libc::c_int = 0 as libc::c_int;
    static mut match_index: libc::c_int = 0 as libc::c_int;
    
    
    if state == 0 {
        if match_list.is_null() {
            match_list_size = 5 ;
            match_list = strvec_create(match_list_size);
        }
        i = 0 ;
        while i < match_list_size {
            *match_list.offset(i as isize) = 0 as *mut libc::c_char;
            i += 1;
        }

        match_index = 0 ;
        if absolute_program(name) != 0 {
            *match_list.offset(0 as isize) = find_absolute_program(name, flags);
            *match_list.offset(1 as isize) = 0 as *mut libc::c_void as *mut libc::c_char;
            path_list = 0 as *mut libc::c_char;
        } else {
            name_len = strlen(name) as libc::c_int;
            file_to_lose_on = 0 as *mut libc::c_char;
            dot_found_in_search = 0 ;
            if stat(b".\0" as *const u8 as *const libc::c_char, &mut dotinfo)
                < 0 
            {
                dotinfo.st_ino = 0 as __ino_t;
                dotinfo.st_dev = dotinfo.st_ino;
            }
            path_list = get_string_value(b"PATH\0" as *const u8 as *const libc::c_char);
            path_index = 0 ;
        }

        while !path_list.is_null()
            && *path_list.offset(path_index as isize) as libc::c_int != 0
        {
            path_element = get_next_path_element(path_list, &mut path_index);
            if path_element.is_null() {
                break;
            }
            match_0 = find_in_path_element(
                name,
                path_element,
                flags,
                name_len,
                &mut dotinfo,
            );

            free(path_element as *mut libc::c_void);

            if match_0.is_null() {
                continue;
            }

            if match_index + 1 == match_list_size {
                match_list_size += 10 ;
                match_list = strvec_resize(
                    match_list,
                    match_list_size + 1 ,
                );
            }

            match_index = match_index + 1;
            *match_list.offset(match_index as isize) = match_0;
            *match_list.offset(match_index as isize) = 0 as *mut libc::c_char;
            FREE!(file_to_lose_on);
            file_to_lose_on = 0 as *mut libc::c_void as *mut libc::c_char;
        }
        match_index = 0 ;
    }

    match_0 = *match_list.offset(match_index as isize);

    if !match_0.is_null() {
        match_index += 1;
    }

    return match_0;
}


unsafe extern "C" fn find_absolute_program(
    name: *const libc::c_char,
    flags: libc::c_int,
) -> *mut libc::c_char {
    let mut st: libc::c_int = 0;
    
    st = file_status(name);

    if st & FS_EXISTS as libc::c_int == 0  {
        return 0 as *mut libc::c_char;
    }

    if flags & FS_EXISTS as libc::c_int != 0
        || flags & FS_EXEC_ONLY as libc::c_int != 0 
        && st & FS_EXECABLE as libc::c_int != 0
    {
        return savestring!(name);
    }
    return 0 as *mut libc::c_char;
}

unsafe extern "C" fn find_in_path_element(
    name: *const libc::c_char,
    path: *mut libc::c_char,
    flags: libc::c_int,
    _name_len: libc::c_int,
    dotinfop: *mut stat,
) -> *mut libc::c_char {
    let mut status: libc::c_int = 0;
    let mut full_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut xpath: *mut libc::c_char = 0 as *mut libc::c_char;
    
    xpath = if posixly_correct == 0 && *path as libc::c_int == '~' as i32
    {
        bash_tilde_expand(path, 0 )
    } else {
        path
    };

    if dot_found_in_search == 0 && *xpath as libc::c_int == '.' as i32 {
        dot_found_in_search = same_file(
            b".\0" as *const u8 as *const libc::c_char,
            xpath,
            dotinfop,
            0 as *mut libc::c_void as *mut stat,
        );
    }

    full_path = sh_makepath(xpath, name, 0 );
    
    status = file_status(full_path);
    
    if xpath != path {
        free( xpath as *mut libc::c_void);
    }

    if status & FS_EXISTS as libc::c_int == 0 {
        free(full_path as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }

    if flags & FS_EXISTS as libc::c_int != 0 {
        return full_path;
    }

    if flags & FS_READABLE as libc::c_int != 0 && status & FS_READABLE as libc::c_int != 0 {
        return full_path;
    }

    if status & FS_EXECABLE as libc::c_int != 0
        && flags & (FS_EXEC_ONLY as libc::c_int | FS_EXEC_PREFERRED as libc::c_int) != 0
        && (flags & FS_NODIRS as libc::c_int == 0 as libc::c_int
            || status & FS_DIRECTORY as libc::c_int == 0 as libc::c_int)
    {
        FREE!(file_to_lose_on);
        file_to_lose_on = 0 as *mut libc::c_char;
        return full_path;
    }

    if flags & FS_EXEC_PREFERRED as libc::c_int != 0 && file_to_lose_on.is_null()
        && exec_name_should_ignore(full_path) == 0 
    {
        file_to_lose_on = savestring!(full_path);
    }

    if flags & (FS_EXEC_ONLY as libc::c_int | FS_EXEC_PREFERRED as libc::c_int) != 0
        || flags & FS_NODIRS as libc::c_int != 0 && status & FS_DIRECTORY as libc::c_int != 0
        || flags & FS_READABLE as libc::c_int != 0
            && status & FS_READABLE as libc::c_int == 0  
    {
        free( full_path as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    } else {
        return full_path
    };
}

unsafe extern "C" fn find_user_command_in_path(
    name: *const libc::c_char,
    path_list: *mut libc::c_char,
    flags: libc::c_int,
) -> *mut libc::c_char {
    let mut full_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path_index: libc::c_int = 0;
    let mut name_len: libc::c_int = 0;
    let mut dotinfo: stat = stat {
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
    
    dot_found_in_search = 0 ;

    if absolute_program(name) != 0 {
        full_path = find_absolute_program(name, flags);
        return full_path;
    }
    
    if path_list.is_null() || *path_list as libc::c_int == '\u{0}' as i32 {
        return savestring!(name);
    }

    file_to_lose_on = 0 as *mut libc::c_char;
    name_len = strlen(name) as libc::c_int;
    if stat(b".\0" as *const u8 as *const libc::c_char, &mut dotinfo) < 0 as libc::c_int
    {
        dotinfo.st_ino = 0 as libc::c_int as __ino_t;
        dotinfo.st_dev = dotinfo.st_ino;
    }
    path_index = 0 ;

    while *path_list.offset(path_index as isize) != 0 {
        QUIT!();

        path = get_next_path_element(path_list, &mut path_index);
        if path.is_null() {
            break;
        }

        full_path = find_in_path_element(name, path, flags, name_len, &mut dotinfo);
        free(path as *mut libc::c_void);
        
        if !full_path.is_null() && is_directory(full_path) != 0 {
            free(full_path as *mut libc::c_void);
            continue;
        } 
        if !full_path.is_null() {
            FREE!(file_to_lose_on);
            return full_path;
        }
    }

    if !file_to_lose_on.is_null() && flags & FS_NODIRS as libc::c_int != 0
        && is_directory(file_to_lose_on) != 0
    {
        free( file_to_lose_on as *mut libc::c_void);
        file_to_lose_on = 0 as *mut libc::c_char;
    }
    return file_to_lose_on;
}


#[no_mangle]
pub unsafe extern "C" fn find_in_path(
    name: *const libc::c_char,
    path_list: *mut libc::c_char,
    flags: libc::c_int,
) -> *mut libc::c_char {
    return find_user_command_in_path(name, path_list, flags);
}






