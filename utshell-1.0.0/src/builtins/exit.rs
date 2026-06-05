//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

use crate::builtins::{
    common::{err_translate_fn, get_exitstat},
    evalfile::maybe_execute_file,
    help::builtin_help,
    jobs::jobs_builtin,
};
use crate::jobs::list_all_jobs;
use crate::sig::jump_to_top_level;
use crate::src_common::*;

unsafe fn STREQ(a: *const libc::c_char, b: *const libc::c_char) -> bool {
    return *a == *b && libc::strcmp(a, b) == 0;
}

static mut sourced_logout: i32 = 0;

#[no_mangle]
pub fn exit_builtin(list: *mut WordList) -> i32 {
    let c_str = CString::new("--help").unwrap();
    let c_ptr = c_str.as_ptr();
    if unsafe {
        list != std::ptr::null_mut()
            && (*list).word != std::ptr::null_mut()
            && STREQ((*(*list).word).word, c_ptr)
    } {
        builtin_help();
        return EX_USAGE;
    }

    if unsafe { interactive != 0 } {
        if unsafe { login_shell != 0 } {
            let names = String::from("logout");
            err_translate_fn(&names, std::ptr::null_mut());
            println!();
        } else {
            eprintln!("exit");
        }
    }
    return exit_or_logout(list);
}

#[no_mangle]
pub fn logout_builtin(list: *mut WordList) -> i32 {
    let c_str = CString::new("--help").unwrap();
    let c_ptr = c_str.as_ptr();
    if unsafe {
        list != std::ptr::null_mut()
            && (*list).word != std::ptr::null_mut()
            && STREQ((*(*list).word).word, c_ptr)
    } {
        builtin_help();
        return EX_USAGE;
    }

    if unsafe { login_shell == 0 } {
        let names = String::from("logout");
        err_translate_fn(&names, std::ptr::null_mut());
        println!();
        let c_str = CString::new("not login shell: use `exit'").unwrap();
        let c_ptr = c_str.as_ptr();
        unsafe {
            builtin_error(c_ptr);
        }

        return EXECUTION_FAILURE!();
    } else {
        return exit_or_logout(list);
    }
}

pub fn exit_or_logout(list: *mut WordList) -> i32 {
    let exit_value: i32;
    let exit_immediate_okay: i32;
    let mut stopmsg: i32;

    exit_immediate_okay = unsafe {
        (interactive == 0
            || last_shell_builtin == Some(exit_builtin)
            || last_shell_builtin == Some(logout_builtin)
            || last_shell_builtin == Some(jobs_builtin)) as i32
    };

    /* Check for stopped jobs if thw user wants to.*/
    if exit_immediate_okay == 0 {
        stopmsg = 0;
        for i in 0..unsafe { js.j_jobslots } {
            if unsafe { !(*jobs.offset(i as isize)).is_null() } {
                // Skip jobs that have been disowned with -h flag (J_NOHUP)
                if unsafe { (**jobs.offset(i as isize)).flags & J_NOHUP as libc::c_int != 0 } {
                    continue;
                }

                if unsafe { STOPPED!(i) } {
                    stopmsg = JSTOPPED as i32;
                    break;
                } else if unsafe { (check_jobs_at_exit != 0) && (stopmsg == 0) && RUNNING!(i) } {
                    stopmsg = JRUNNING as i32;
                    break;
                }
            }
        }

        if stopmsg == JSTOPPED as i32 {
            let names = String::from("stoppedjobs");
            err_translate_fn(&names, std::ptr::null_mut());
            eprintln!();
        } else if stopmsg == JRUNNING as i32 {
            let names = String::from("runjobs");
            err_translate_fn(&names, std::ptr::null_mut());
            eprintln!();
        }

        if unsafe { stopmsg == check_jobs_at_exit } {
            // Only list jobs if there are actually non-disowned jobs to show
            let mut has_non_nohup_jobs = false;
            for i in 0..unsafe { js.j_jobslots } {
                if unsafe { !(*jobs.offset(i as isize)).is_null() } {
                    if unsafe { (**jobs.offset(i as isize)).flags & J_NOHUP as libc::c_int == 0 } {
                        has_non_nohup_jobs = true;
                        break;
                    }
                }
            }
            if has_non_nohup_jobs {
                list_all_jobs(JLIST_STANDARD!())
            }
        }

        if stopmsg != 0 {
            unsafe {
                last_shell_builtin = Some(exit_builtin);
                this_shell_builtin = last_shell_builtin;
            }
            return EXECUTION_FAILURE!();
        }
    }

    if unsafe { (running_trap == 1) && (list == std::ptr::null_mut()) } {
        unsafe {
            exit_value = trap_saved_exit_value;
        }
    } else {
        exit_value = get_exitstat(list);
    }

    bash_logout();
    unsafe {
        last_command_exit_value = exit_value;
    }

    jump_to_top_level(EXITPROG!());

    0
}

pub fn bash_logout() {
    if unsafe { login_shell != 0 && sourced_logout == 0 && subshell_environment == 0 } {
        unsafe {
            sourced_logout = sourced_logout + 1;
        }
        let c_str = CString::new("~/.utshell_logout").unwrap();
        let c_ptr = c_str.as_ptr();
        maybe_execute_file(c_ptr, 1);
        let msg = CString::new(" \"/etc/utshell.utshell_logout\" ").unwrap();
        maybe_execute_file(msg.as_ptr(), 1);
    }
}
