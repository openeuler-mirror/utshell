//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::CStr;
use std::ffi::CString;

use crate::alias::{add_alias, all_aliases, delete_all_aliases, find_alias, remove_alias};
use crate::findcmd::find_user_command;
use crate::general::legal_alias_name;
use crate::src_common::*;
use crate::variables::find_function;

use crate::builtins::{
    bashgetopt::{internal_getopt, reset_internal_getopt},
    common::{builtin_usage, find_shell_builtin, sh_chkwrite, sh_notfound},
    help::builtin_help,
};

/* Hack the alias command in a Korn shell way. */
#[no_mangle]
pub fn alias_builtin(mut list: *mut WordList) -> libc::c_int {
    let mut any_failed;
    let mut offset;
    let mut pflag;
    let mut dflags;
    let alias_list: *mut *mut AliasT;
    let mut t: *mut AliasT;
    let mut name: *mut libc::c_char;
    let mut value: *mut libc::c_char;

    dflags = if unsafe { posixly_correct != 0 } {
        0 as libc::c_int
    } else {
        0x1 as libc::c_int
    };
    pflag = 0 as libc::c_int;
    reset_internal_getopt();
    loop {
        offset = internal_getopt(
            list,
            b"p\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !(offset != -(1 as libc::c_int)) {
            break;
        }
        match offset as u8 {
            b'p' => {
                pflag = 1;
                dflags |= AL_REUSABLE;
            }
            _ => {
                if offset == -99 {
                    builtin_help();
                    return EX_USAGE!();
                }
                builtin_usage();
                return EX_USAGE!();
            }
        }
    }
    unsafe {
        list = loptend;
        if list.is_null() || pflag != 0 {
            if aliases.is_null() {
                return EXECUTION_SUCCESS!();
            }
            alias_list = all_aliases();
            if alias_list.is_null() {
                return EXECUTION_SUCCESS!();
            }
            offset = 0;
            while !(*alias_list.offset(offset as isize)).is_null() {
                print_alias(*alias_list.offset(offset as isize), dflags);
                offset += 1;
            }
            libc::free(alias_list as *mut libc::c_void);
            if list.is_null() {
                return sh_chkwrite(EXECUTION_SUCCESS!());
            }
        }
        any_failed = 0;
        while !list.is_null() {
            name = (*(*list).word).word;
            offset = 0;
            while *name.offset(offset as isize) as libc::c_int != 0
                && *name.offset(offset as isize) as libc::c_int != '=' as i32
            {
                offset += 1;
            }
            if offset != 0 && *name.offset(offset as isize) as libc::c_int == '=' as i32 {
                *name.offset(offset as isize) = '\u{0}' as i32 as libc::c_char;
                value = name
                    .offset(offset as isize)
                    .offset(1 as libc::c_int as isize);
                if legal_alias_name(name, 0) == 0 {
                    builtin_error(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"`%s': invalid alias name\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        name,
                    );
                    any_failed += 1;
                } else {
                    let slice = CStr::from_ptr(value);
                    let r_str = slice.to_str().unwrap().to_owned();
                    let new_str = CString::new(r_str).unwrap();
                    if legal_alias_rust(name, new_str.as_ptr() as *mut libc::c_char) == 0 {
                        add_alias(name, value);
                    }
                }
            } else {
                t = find_alias(name);
                if !t.is_null() {
                    print_alias(t, dflags);
                } else {
                    sh_notfound(name);
                    any_failed += 1;
                }
            }
            list = (*list).next;
        }
    }
    return if any_failed != 0 {
        EXECUTION_FAILURE!()
    } else {
        EXECUTION_SUCCESS!()
    };
}

#[no_mangle]
pub fn unalias_builtin(mut list: *mut WordList) -> libc::c_int {
    let mut alias: *mut AliasT;
    let mut opt: libc::c_int;
    let mut aflag: libc::c_int;
    aflag = 0 as libc::c_int;
    reset_internal_getopt();
    loop {
        opt = internal_getopt(
            list,
            b"a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !(opt != -(1 as libc::c_int)) {
            break;
        }
        match opt as u8 {
            b'a' => {
                aflag = 1 as libc::c_int;
            }
            _ => {
                if opt == -99 {
                    builtin_help();
                    return EX_USAGE!();
                }
                builtin_usage();
                return EX_USAGE!();
            }
        }
    }
    unsafe {
        list = loptend;
        if aflag != 0 {
            delete_all_aliases();
            return 0;
        }
        if list.is_null() {
            builtin_usage();
            return EX_USAGE!();
        }
        aflag = 0 as libc::c_int;
        while !list.is_null() {
            alias = find_alias((*(*list).word).word);
            if !alias.is_null() {
                remove_alias((*alias).name);
            } else {
                sh_notfound((*(*list).word).word);
                aflag += 1;
            }
            list = (*list).next;
        }
    }
    return if aflag != 0 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}

fn print_alias(alias: *mut AliasT, flags: libc::c_int) {
    let value: *mut libc::c_char;
    unsafe {
        value = sh_single_quote((*alias).value);
        if flags & 0x1 as libc::c_int != 0 {
            print!("alias ");
            //printf(
            //    b"alias %s\0" as *const u8 as *const libc::c_char,
            if !((*alias).name).is_null()
                && *((*alias).name).offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
            {
                // b"-- \0" as *const u8 as *const libc::c_char
                print!("-- ");
            } else {
                // b"\0" as *const u8 as *const libc::c_char
                print!(" ");
            }
            //);
        }
        println!(
            "{}={}",
            CStr::from_ptr((*alias).name).to_string_lossy().into_owned(),
            CStr::from_ptr(value).to_string_lossy().into_owned()
        );
        libc::free(value as *mut libc::c_void);
    }
}

//增加alias安全策略逻辑
fn legal_alias_rust(name: *mut libc::c_char, value: *mut libc::c_char) -> libc::c_int {
    let name_w: *mut libc::c_char;
    let value_w: *mut libc::c_char;
    let new_value: *mut libc::c_char;
    let mut new_value_2: *mut libc::c_char;
    let mut _shell_bui: *mut libc::c_char;
    let mut t: *mut AliasT;
    let dflags;
    dflags = if unsafe { posixly_correct != 0 } {
        0 as libc::c_int
    } else {
        0x1 as libc::c_int
    };
    unsafe {
        if libc::strstr(
            value,
            CString::new(";").unwrap().as_ptr() as *mut libc::c_char,
        ) != std::ptr::null_mut()
        {
            println!("; is not allow in alias");
            return 1;
        }
        t = find_alias(name);
        if !t.is_null() {
            println!(
                "{} is already in alias",
                CStr::from_ptr(name).to_string_lossy().into_owned()
            );
            print_alias(t, dflags);
            return 1;
        }
        name_w = find_user_command(name);
        new_value = sh_single_quote(value);
        // 按照空格区分
        new_value_2 = libc::strtok(
            value,
            CString::new(" ").unwrap().as_ptr() as *mut libc::c_char,
        );
        t = find_alias(new_value_2);
        while t != std::ptr::null_mut() {
            new_value_2 = libc::strtok(
                (*t).value,
                CString::new(" ").unwrap().as_ptr() as *mut libc::c_char,
            );
            if libc::strcmp((*t).name, new_value_2) == 0 {
                break;
            }
            t = find_alias(new_value_2);
        }
    }
    let arr: [*mut libc::c_char; 7] = [
        CString::new("exec").unwrap().into_raw() as *mut libc::c_char,
        CString::new("eval").unwrap().into_raw() as *mut libc::c_char,
        CString::new("builtin").unwrap().into_raw() as *mut libc::c_char,
        CString::new("command").unwrap().into_raw() as *mut libc::c_char,
        CString::new("function").unwrap().into_raw() as *mut libc::c_char,
        CString::new("source").unwrap().into_raw() as *mut libc::c_char,
        CString::new(".").unwrap().into_raw() as *mut libc::c_char,
    ];

    for index in 0..7 {
        unsafe {
            if libc::strcmp(new_value_2, arr[index]) == 0 {
                println!(
                    "command {} will raise an unsafe operation",
                    CStr::from_ptr(arr[index]).to_string_lossy().into_owned()
                );
                return 1;
            }
        }
    }
    unsafe {
        value_w = find_user_command(new_value_2);
        if name_w != std::ptr::null_mut() {
            if value_w != std::ptr::null_mut() && libc::strcmp(name_w, value_w) == 0 {
                return 0;
            } else {
                println!("The name and value point to different executable files");
                return 1;
            }
        } else {
            if find_shell_builtin(name) != None {
                println!(
                    "name {} is shell builtin",
                    CStr::from_ptr(name).to_string_lossy().into_owned()
                );
                return 1;
            } else if find_function(name) != std::ptr::null_mut() {
                println!(
                    "name {} is function",
                    CStr::from_ptr(name).to_string_lossy().into_owned()
                );
                return 1;
            }
        }
    }
    return 0;
}
