use crate::general::{absolute_program, bash_tilde_expand, conf_standard_path, extract_colon_unit};
use crate::hashcmd::{phash_insert, phash_remove, phash_search};
use crate::pathexp::setup_ignore_patterns;
use crate::src_common::*;
use crate::variables::{find_variable_tempenv, get_string_value};

static mut file_to_lose_on: *mut libc::c_char = std::ptr::null_mut();
static mut execignore: ignorevar = ignorevar {
    varname: b"EXECIGNORE\0" as *const u8 as *mut libc::c_char,
    ignores: std::ptr::null_mut(),
    num_ignores: 0,
    last_ignoreval: std::ptr::null_mut(),
    item_func: None,
};

#[no_mangle]
pub fn setup_exec_ignore(_varname: *mut libc::c_char) {
    // SAFETY: setup_ignore_patterns 需要访问静态变量 execignore
    unsafe {
        setup_ignore_patterns(&mut execignore);
    }
}

/// 检查执行名称是否应该被忽略
/// SAFETY: 访问静态变量 execignore
fn exec_name_should_ignore(name: *const libc::c_char) -> libc::c_int {
    unsafe {
        let mut p = execignore.ignores;
        while !p.is_null() && !(*p).val.is_null() {
            if c_strmatch(
                (*p).val,
                name as *mut libc::c_char,
                FNMATCH_EXTFLAG!() | FNM_CASEFOLD!(),
            ) != FNM_NOMATCH!()
            {
                return 1;
            }
            p = p.offset(1);
        }
    }
    0
}

#[no_mangle]
pub fn file_status(name: *const libc::c_char) -> libc::c_int {
    let mut finfo: crate::src_common::stat = crate::src_common::stat_init;

    // SAFETY: 调用C库函数 c_stat 和 eaccess

    if c_stat(name, &mut finfo) < 0 {
        return 0;
    }

    if S_ISDIR!(finfo.st_mode) {
        return FS_EXISTS as libc::c_int | FS_DIRECTORY as libc::c_int;
    }

    let mut r = FS_EXISTS as libc::c_int;
    // SAFETY: 调用 eaccess 和 exec_name_should_ignore
    unsafe {
        if exec_name_should_ignore(name) == 0 && eaccess(name, libc::X_OK) == 0 {
            r |= FS_EXECABLE as libc::c_int;
        }
        if eaccess(name, libc::R_OK) == 0 {
            r |= FS_READABLE as libc::c_int;
        }
    }
    r
}

#[no_mangle]
pub fn executable_file(file: *const libc::c_char) -> libc::c_int {
    let s = file_status(file);
    if s & FS_DIRECTORY as libc::c_int != 0 {
        // SAFETY: 设置 errno
        unsafe {
            *c___errno_location() = EISDIR!();
        }
    }
    (s & FS_EXECABLE as libc::c_int != 0 && s & FS_DIRECTORY as libc::c_int == 0) as libc::c_int
}

#[no_mangle]
pub fn is_directory(file: *const libc::c_char) -> libc::c_int {
    file_status(file) & FS_DIRECTORY as libc::c_int
}

#[no_mangle]
pub fn executable_or_directory(file: *const libc::c_char) -> libc::c_int {
    let s = file_status(file);
    (s & FS_EXECABLE as libc::c_int != 0 || s & FS_DIRECTORY as libc::c_int != 0) as libc::c_int
}

#[no_mangle]
pub fn find_user_command(name: *const libc::c_char) -> *mut libc::c_char {
    // SAFETY: 调用 find_user_command_internal

    find_user_command_internal(
        name,
        FS_EXEC_PREFERRED as libc::c_int | FS_NODIRS as libc::c_int,
    )
}

#[no_mangle]
pub fn find_path_file(name: *const libc::c_char) -> *mut libc::c_char {
    find_user_command_internal(name, FS_READABLE as libc::c_int)
}

fn _find_user_command_internal(name: *const libc::c_char, flags: libc::c_int) -> *mut libc::c_char {
    let var = find_variable_tempenv(b"PATH\0" as *const u8 as *const libc::c_char);

    // SAFETY: 访问变量值和字符串操作
    let path_list = unsafe {
        if !var.is_null() {
            value_cell!(var)
        } else {
            std::ptr::null_mut()
        }
    };

    // SAFETY: 字符串操作
    unsafe {
        if path_list.is_null() || *path_list == 0 {
            return savestring!(name);
        }
    }

    find_user_command_in_path(name, path_list, flags)
}

fn find_user_command_internal(name: *const libc::c_char, flags: libc::c_int) -> *mut libc::c_char {
    _find_user_command_internal(name, flags)
}

fn get_next_path_element(
    path_list: *mut libc::c_char,
    path_index_pointer: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut path = extract_colon_unit(path_list, path_index_pointer);
    if path.is_null() {
        return path;
    }
    // SAFETY: 字符串操作和内存释放
    unsafe {
        if *path == 0 {
            free(path as *mut libc::c_void);
            path = savestring!(b".\0" as *const u8 as *const libc::c_char);
        }
    }
    path
}

#[no_mangle]
pub fn search_for_command(pathname: *const libc::c_char, flags: libc::c_int) -> *mut libc::c_char {
    let mut hashed_file: *mut libc::c_char = std::ptr::null_mut();
    let command: *mut libc::c_char;
    let path_list: *mut libc::c_char;
    let temp_path: libc::c_int;
    let mut st: libc::c_int;

    let path = find_variable_tempenv(b"PATH\0" as *const u8 as *const libc::c_char);

    // SAFETY: 访问静态变量和指针操作
    unsafe {
        temp_path = (!path.is_null() && tempvar_p!(path) != 0) as libc::c_int;

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
                hashed_file = std::ptr::null_mut();
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
                path_list = std::ptr::null_mut();
            }

            command = find_user_command_in_path(
                pathname,
                path_list,
                FS_EXEC_PREFERRED as libc::c_int | FS_NODIRS as libc::c_int,
            );

            if !command.is_null()
                && hashing_enabled != 0
                && temp_path == 0
                && flags & CMDSRCH_HASH as libc::c_int != 0
            {
                if STREQ!(command, pathname) {
                    st = file_status(command);
                    if st & FS_EXECABLE as libc::c_int != 0 {
                        phash_insert(
                            pathname as *mut libc::c_char,
                            command,
                            dot_found_in_search,
                            1,
                        );
                    }
                } else if posixly_correct != 0 {
                    st = file_status(command);
                    if st & FS_EXECABLE as libc::c_int != 0 {
                        phash_insert(
                            pathname as *mut libc::c_char,
                            command,
                            dot_found_in_search,
                            1,
                        );
                    }
                } else {
                    phash_insert(
                        pathname as *mut libc::c_char,
                        command,
                        dot_found_in_search,
                        1,
                    );
                }
            }

            if flags & CMDSRCH_STDPATH as libc::c_int != 0 {
                free(path_list as *mut libc::c_void);
            }
        }
    }

    command
}

#[no_mangle]
pub fn user_command_matches(
    name: *const libc::c_char,
    flags: libc::c_int,
    state: libc::c_int,
) -> *mut libc::c_char {
    static mut match_list: *mut *mut libc::c_char = std::ptr::null_mut();
    static mut match_list_size: libc::c_int = 0;
    static mut match_index: libc::c_int = 0;

    // SAFETY: 访问静态变量和指针操作
    unsafe {
        if state == 0 {
            if match_list.is_null() {
                match_list_size = 5;
                match_list = c_strvec_create(match_list_size);
            }

            let mut i = 0;
            while i < match_list_size {
                *match_list.offset(i as isize) = std::ptr::null_mut();
                i += 1;
            }

            match_index = 0;

            if absolute_program(name) != 0 {
                *match_list.offset(0) = find_absolute_program(name, flags);
                *match_list.offset(1) = std::ptr::null_mut();
                return *match_list.offset(match_index as isize);
            }

            let path_list = get_string_value(b"PATH\0" as *const u8 as *const libc::c_char);
            let mut path_index: libc::c_int = 0;
            let name_len = strlen(name) as libc::c_int;
            let mut dotinfo: crate::src_common::stat = crate::src_common::stat_init;

            file_to_lose_on = std::ptr::null_mut();
            dot_found_in_search = 0;

            if c_stat(b".\0" as *const u8 as *const libc::c_char, &mut dotinfo) < 0 {
                dotinfo.st_ino = 0;
                dotinfo.st_dev = 0;
            }

            while !path_list.is_null() && *path_list.offset(path_index as isize) != 0 {
                let path_element = get_next_path_element(path_list, &mut path_index);
                if path_element.is_null() {
                    break;
                }

                let match_0 =
                    find_in_path_element(name, path_element, flags, name_len, &mut dotinfo);
                free(path_element as *mut libc::c_void);

                if match_0.is_null() {
                    continue;
                }

                if match_index + 1 == match_list_size {
                    match_list_size += 10;
                    match_list = c_strvec_resize(match_list, match_list_size + 1);
                }

                *match_list.offset(match_index as isize) = match_0;
                match_index += 1;
                *match_list.offset(match_index as isize) = std::ptr::null_mut();

                FREE!(file_to_lose_on);
                file_to_lose_on = std::ptr::null_mut();
            }
            match_index = 0;
        }

        let match_0 = *match_list.offset(match_index as isize);
        if !match_0.is_null() {
            match_index += 1;
        }
        match_0
    }
}

fn find_absolute_program(name: *const libc::c_char, flags: libc::c_int) -> *mut libc::c_char {
    let st = file_status(name);

    if st & FS_EXISTS as libc::c_int == 0 {
        return std::ptr::null_mut();
    }

    if flags & FS_EXISTS as libc::c_int != 0
        || flags & FS_EXEC_ONLY as libc::c_int != 0 && st & FS_EXECABLE as libc::c_int != 0
    {
        // SAFETY: 字符串复制
        unsafe { savestring!(name) }
    } else {
        std::ptr::null_mut()
    }
}

fn find_in_path_element(
    name: *const libc::c_char,
    path: *mut libc::c_char,
    flags: libc::c_int,
    _name_len: libc::c_int,
    dotinfop: *mut crate::src_common::stat,
) -> *mut libc::c_char {
    // SAFETY: 所有指针操作和字符串操作
    unsafe {
        let xpath = if posixly_correct == 0 && *path == b'~' as libc::c_char {
            bash_tilde_expand(path, 0)
        } else {
            path
        };

        if dot_found_in_search == 0 && *xpath == b'.' as libc::c_char {
            dot_found_in_search = same_file(
                b".\0" as *const u8 as *const libc::c_char,
                xpath,
                dotinfop,
                std::ptr::null_mut(),
            );
        }

        let full_path = c_sh_makepath(xpath, name, 0);
        let status = file_status(full_path);

        if xpath != path {
            free(xpath as *mut libc::c_void);
        }

        if status & FS_EXISTS as libc::c_int == 0 {
            free(full_path as *mut libc::c_void);
            return std::ptr::null_mut();
        }

        if flags & FS_EXISTS as libc::c_int != 0 {
            return full_path;
        }

        if flags & FS_READABLE as libc::c_int != 0 && status & FS_READABLE as libc::c_int != 0 {
            return full_path;
        }

        if status & FS_EXECABLE as libc::c_int != 0
            && flags & (FS_EXEC_ONLY as libc::c_int | FS_EXEC_PREFERRED as libc::c_int) != 0
            && (flags & FS_NODIRS as libc::c_int == 0 || status & FS_DIRECTORY as libc::c_int == 0)
        {
            FREE!(file_to_lose_on);
            file_to_lose_on = std::ptr::null_mut();
            return full_path;
        }

        if flags & FS_EXEC_PREFERRED as libc::c_int != 0
            && file_to_lose_on.is_null()
            && exec_name_should_ignore(full_path) == 0
        {
            file_to_lose_on = savestring!(full_path);
        }

        if flags & (FS_EXEC_ONLY as libc::c_int | FS_EXEC_PREFERRED as libc::c_int) != 0
            || flags & FS_NODIRS as libc::c_int != 0 && status & FS_DIRECTORY as libc::c_int != 0
            || flags & FS_READABLE as libc::c_int != 0 && status & FS_READABLE as libc::c_int == 0
        {
            free(full_path as *mut libc::c_void);
            std::ptr::null_mut()
        } else {
            full_path
        }
    }
}

fn find_user_command_in_path(
    name: *const libc::c_char,
    path_list: *mut libc::c_char,
    flags: libc::c_int,
) -> *mut libc::c_char {
    // SAFETY: 所有指针操作和字符串操作
    unsafe {
        dot_found_in_search = 0;

        if absolute_program(name) != 0 {
            return find_absolute_program(name, flags);
        }

        if path_list.is_null() || *path_list == 0 {
            return savestring!(name);
        }

        file_to_lose_on = std::ptr::null_mut();
        let name_len = strlen(name) as libc::c_int;
        let mut dotinfo: crate::src_common::stat = crate::src_common::stat_init;

        if c_stat(b".\0" as *const u8 as *const libc::c_char, &mut dotinfo) < 0 {
            dotinfo.st_ino = 0;
            dotinfo.st_dev = 0;
        }

        let mut path_index: libc::c_int = 0;

        while *path_list.offset(path_index as isize) != 0 {
            QUIT!();

            let path = get_next_path_element(path_list, &mut path_index);
            if path.is_null() {
                break;
            }

            let full_path = find_in_path_element(name, path, flags, name_len, &mut dotinfo);
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

        if !file_to_lose_on.is_null()
            && flags & FS_NODIRS as libc::c_int != 0
            && is_directory(file_to_lose_on) != 0
        {
            free(file_to_lose_on as *mut libc::c_void);
            file_to_lose_on = std::ptr::null_mut();
        }

        file_to_lose_on
    }
}

#[no_mangle]
pub fn find_in_path(
    name: *const libc::c_char,
    path_list: *mut libc::c_char,
    flags: libc::c_int,
) -> *mut libc::c_char {
    find_user_command_in_path(name, path_list, flags)
}
