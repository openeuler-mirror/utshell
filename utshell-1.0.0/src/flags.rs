//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later
use crate::bashhist::bash_initialize_history;
use crate::jobs::set_job_control;
use crate::src_common::*;
use crate::utshell::{disable_priv_mode, maybe_make_restricted};

#[no_mangle]
pub fn find_flag(name: libc::c_int) -> *mut libc::c_int {
    let mut i: libc::c_int = 0;
    unsafe {
        while shell_flags[i as usize].name != 0 {
            if shell_flags[i as usize].name as libc::c_int == name {
                return shell_flags[i as usize].value;
            }
            i += 1;
        }
    }
    return FLAG_UNKNOWN as *mut libc::c_int;
}
#[no_mangle]
pub fn change_flag(flag: libc::c_int, on_or_off: libc::c_int) -> libc::c_int {
    let value: *mut libc::c_int;
    let old_value: libc::c_int;
    unsafe {
        if restricted != 0 && flag == b'r' as i32 && on_or_off == FLAG_OFF as i32 {
            return FLAG_ERROR;
        }
        value = find_flag(flag);
        if value as *const libc::c_int == FLAG_UNKNOWN as *const libc::c_int
            || on_or_off != FLAG_ON as i32 && on_or_off != FLAG_OFF as i32
        {
            return FLAG_ERROR;
        }
        old_value = *value;
        *value = if on_or_off == FLAG_ON as i32 {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        match flag as u8 {
            b'H' => {
                history_expansion = histexp_flag;
                if on_or_off == '-' as i32 {
                    bash_initialize_history();
                }
            }
            b'm' => {
                set_job_control((on_or_off == '-' as i32) as libc::c_int);
            }
            b'e' => {
                if builtin_ignoring_errexit == 0 as libc::c_int {
                    exit_immediately_on_error = errexit_flag;
                }
            }
            b'n' => {
                if interactive_shell != 0 {
                    read_but_dont_execute = 0 as libc::c_int;
                }
            }
            b'p' => {
                if on_or_off == '+' as i32 {
                    disable_priv_mode();
                }
            }
            b'r' => {
                if on_or_off == '-' as i32 && shell_initialized != 0 {
                    maybe_make_restricted(shell_name);
                }
            }
            b'v' => {
                echo_input_at_read = verbose_flag;
            }
            _ => {}
        }
    }
    return old_value;
}

#[no_mangle]
pub fn which_set_flags() -> *mut libc::c_char {
    let temp: *mut libc::c_char;
    let mut i: libc::c_int;
    let mut string_index: libc::c_int;
    unsafe {
        temp = libc::malloc(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<[flags_alist; 22]>() as libc::c_ulong)
                .wrapping_add(read_from_stdin as libc::c_ulong)
                .wrapping_add(want_pending_command as libc::c_ulong) as usize,
        ) as *mut libc::c_char;
        string_index = 0 as libc::c_int;
        i = string_index;
        while shell_flags[i as usize].name != 0 {
            if *shell_flags[i as usize].value != 0 {
                let fresh0 = string_index;
                string_index = string_index + 1;
                *temp.offset(fresh0 as isize) = shell_flags[i as usize].name;
            }
            i += 1;
        }
        if want_pending_command != 0 {
            let fresh1 = string_index;
            string_index = string_index + 1;
            *temp.offset(fresh1 as isize) = 'c' as i32 as libc::c_char;
        }
        if read_from_stdin != 0 {
            let fresh2 = string_index;
            string_index = string_index + 1;
            *temp.offset(fresh2 as isize) = 's' as i32 as libc::c_char;
        }
        *temp.offset(string_index as isize) = '\0' as i32 as libc::c_char;
    }
    return temp;
}

#[no_mangle]
pub fn get_current_flags() -> *mut libc::c_char {
    let temp: *mut libc::c_char;
    let mut i: libc::c_int;
    unsafe {
        temp = libc::malloc(1 + ::core::mem::size_of::<[flags_alist; 22]>()) as *mut libc::c_char;
        i = 0 as libc::c_int;
        while shell_flags[i as usize].name != 0 {
            *temp.offset(i as isize) = *shell_flags[i as usize].value as libc::c_char;
            i += 1;
        }
        *temp.offset(i as isize) = '\0' as i32 as libc::c_char;
    }
    return temp;
}

#[no_mangle]
pub fn set_current_flags(bitmap: *const libc::c_char) {
    let mut i: libc::c_int = 0;
    if bitmap.is_null() {
        return;
    }
    unsafe {
        while shell_flags[i as usize].name != 0 {
            *shell_flags[i as usize].value = *bitmap.offset(i as isize) as libc::c_int;
            i += 1;
        }
    }
}

#[no_mangle]
pub fn reset_shell_flags() {
    unsafe {
        disallow_filename_globbing = 0 as libc::c_int;
        mark_modified_vars = 0 as libc::c_int;
        just_one_command = 0 as libc::c_int;
        read_but_dont_execute = 0 as libc::c_int;
        place_keywords_in_env = 0 as libc::c_int;
        unbound_vars_is_error = 0 as libc::c_int;
        noclobber = 0 as libc::c_int;
        forced_interactive = 0 as libc::c_int;
        jobs_m_flag = 0 as libc::c_int;
        echo_command_at_execute = 0 as libc::c_int;
        no_symbolic_links = 0 as libc::c_int;
        pipefail_opt = 0 as libc::c_int;
        privileged_mode = 0 as libc::c_int;
        function_trace_mode = 0 as libc::c_int;
        error_trace_mode = 0 as libc::c_int;
        errexit_flag = 0 as libc::c_int;
        exit_immediately_on_error = 0 as libc::c_int;
        verbose_flag = 0 as libc::c_int;
        echo_input_at_read = 0 as libc::c_int;
        interactive_comments = 1 as libc::c_int;
        hashing_enabled = 1 as libc::c_int;
        asynchronous_notification = 0 as libc::c_int;
        histexp_flag = 0 as libc::c_int;
        brace_expansion = 1 as libc::c_int;
        restricted = 0 as libc::c_int;
    }
}
#[no_mangle]
pub fn initialize_flags() {
    let mut i: libc::c_int = 0;
    unsafe {
        while shell_flags[i as usize].name != 0 {
            optflags[(i + 1 as libc::c_int) as usize] = shell_flags[i as usize].name;
            i += 1;
        }
        i += 1;
        optflags[i as usize] = 'o' as i32 as libc::c_char;
        i += 1;
        optflags[i as usize] = ';' as i32 as libc::c_char;
        optflags[(i + 1 as libc::c_int) as usize] = '\0' as i32 as libc::c_char;
    }
}
