//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    static mut shell_name: *mut libc::c_char;
    static mut shell_initialized: libc::c_int;
    static mut interactive_shell: libc::c_int;
    fn disable_priv_mode();
    fn maybe_make_restricted(_: *mut libc::c_char) -> libc::c_int;
    static mut builtin_ignoring_errexit: libc::c_int;
    static mut want_pending_command: libc::c_int;
    static mut read_from_stdin: libc::c_int;
    fn bash_initialize_history();
    fn set_job_control(_: libc::c_int) -> libc::c_int;
}
pub type size_t = libc::c_ulong;


#[derive(Copy, Clone)]
#[repr(C)]
pub struct flags_alist {
    pub name: libc::c_char,
    pub value: *mut libc::c_int,
}
#[no_mangle]
pub static mut mark_modified_vars: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut asynchronous_notification: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut errexit_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut exit_immediately_on_error: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut disallow_filename_globbing: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut place_keywords_in_env: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut read_but_dont_execute: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut just_one_command: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut noclobber: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut unbound_vars_is_error: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut echo_input_at_read: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut verbose_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut echo_command_at_execute: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut jobs_m_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut forced_interactive: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut no_symbolic_links: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut hashing_enabled: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut history_expansion: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut histexp_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut interactive_comments: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut restricted: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut restricted_shell: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut privileged_mode: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut brace_expansion: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut function_trace_mode: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut error_trace_mode: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut pipefail_opt: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut shell_flags: [flags_alist; 22] = unsafe {
    [
        {
            let mut init = flags_alist {
                name: 'a' as i32 as libc::c_char,
                value: &mark_modified_vars as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'b' as i32 as libc::c_char,
                value: &asynchronous_notification as *const libc::c_int
                    as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'e' as i32 as libc::c_char,
                value: &errexit_flag as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'f' as i32 as libc::c_char,
                value: &disallow_filename_globbing as *const libc::c_int
                    as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'h' as i32 as libc::c_char,
                value: &hashing_enabled as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'i' as i32 as libc::c_char,
                value: &forced_interactive as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'k' as i32 as libc::c_char,
                value: &place_keywords_in_env as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'm' as i32 as libc::c_char,
                value: &jobs_m_flag as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'n' as i32 as libc::c_char,
                value: &read_but_dont_execute as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'p' as i32 as libc::c_char,
                value: &privileged_mode as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'r' as i32 as libc::c_char,
                value: &restricted as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 't' as i32 as libc::c_char,
                value: &just_one_command as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'u' as i32 as libc::c_char,
                value: &unbound_vars_is_error as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'v' as i32 as libc::c_char,
                value: &verbose_flag as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'x' as i32 as libc::c_char,
                value: &echo_command_at_execute as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'B' as i32 as libc::c_char,
                value: &brace_expansion as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'C' as i32 as libc::c_char,
                value: &noclobber as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'E' as i32 as libc::c_char,
                value: &error_trace_mode as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'H' as i32 as libc::c_char,
                value: &histexp_flag as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'P' as i32 as libc::c_char,
                value: &no_symbolic_links as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'T' as i32 as libc::c_char,
                value: &function_trace_mode as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 0 as libc::c_int as libc::c_char,
                value: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut optflags: [libc::c_char; 26] = [
    '+' as i32 as libc::c_char,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];

pub const FLAG_UNKNOWN:libc::c_int = 0;
pub const FLAG_OFF: std::os::raw::c_char   = b'+' as std::os::raw::c_char;
pub const FLAG_ERROR :libc::c_int  = -1;
pub const FLAG_ON:std::os::raw::c_char     = b'-' as std::os::raw::c_char;

#[no_mangle]
pub unsafe extern "C" fn find_flag( name: libc::c_int) -> *mut libc::c_int {
    let mut i: libc::c_int = 0;
    while shell_flags[i as usize].name != 0 {
        if shell_flags[i as usize].name as libc::c_int == name {
            return shell_flags[i as usize].value;
        }
        i += 1;
    }
    return FLAG_UNKNOWN as *mut libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn change_flag(
    flag: libc::c_int,
    on_or_off: libc::c_int,
) -> libc::c_int {

    let value: *mut libc::c_int;
    let old_value: libc::c_int;
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
    *value = if on_or_off == FLAG_ON as i32 { 1 as libc::c_int } else { 0 as libc::c_int };
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
    return old_value;
}

#[no_mangle]
pub unsafe extern "C" fn which_set_flags() -> *mut libc::c_char {
    let temp: *mut libc::c_char;
    let mut i: libc::c_int;
    let mut string_index: libc::c_int;
    temp = libc::malloc(
        (1 as libc::c_int as libc::c_ulong)
    .wrapping_add(::core::mem::size_of::<[flags_alist; 22]>() as libc::c_ulong)
    .wrapping_add(read_from_stdin as libc::c_ulong) 
    .wrapping_add(want_pending_command as libc::c_ulong) as usize
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
    return temp;
}

#[no_mangle]
pub unsafe extern "C" fn get_current_flags() -> *mut libc::c_char {
    let temp: *mut libc::c_char;
    let mut i: libc::c_int;
    temp = libc::malloc(1+::core::mem::size_of::<[flags_alist; 22]>()) as *mut libc::c_char;
    i = 0 as libc::c_int;
    while shell_flags[i as usize].name != 0 {
        *temp.offset(i as isize) = *shell_flags[i as usize].value as libc::c_char;
        i += 1;
    }
    *temp.offset(i as isize) = '\0' as i32 as libc::c_char;
    return temp;
}

#[no_mangle]
pub unsafe extern "C" fn set_current_flags(bitmap: *const libc::c_char) {
    let mut i: libc::c_int = 0;
    if bitmap.is_null() {
        return;
    }
    while shell_flags[i as usize].name != 0 {
        *shell_flags[i as usize].value = *bitmap.offset(i as isize) as libc::c_int;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn reset_shell_flags() {
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
#[no_mangle]
pub unsafe extern "C" fn initialize_flags() {
    let mut i: libc::c_int = 0;
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
