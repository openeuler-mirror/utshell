//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

use super::help::builtin_help;
use crate::builtins::bashgetopt::{internal_getopt, reset_internal_getopt};
use crate::builtins::common::{builtin_usage, make_builtin_argv, number_of_args, sh_invalidid};
use crate::builtins::getopt::{sh_getopt, sh_getopt_restore_state};
use crate::general::legal_identifier;
use crate::src_common::*;
use crate::variables::{bind_variable, unbind_variable_noref};

/* getopts_reset is magic code for when OPTIND is reset.  N is the
value that has just been assigned to OPTIND. */
#[no_mangle]
pub fn getopts_reset(newind: i32) {
    unsafe {
        sh_optind = newind;
        sh_badopt = 0;
    }
}

#[no_mangle]
pub fn getopts_unbind_variable(name: *mut libc::c_char) -> i32 {
    return unbind_variable_noref(name);
}

fn readonly_p(va: *mut SHELL_VAR) -> i32 {
    unsafe {
        return (*va).attributes & att_readonly!();
    }
}

fn noassign_p(va: *mut SHELL_VAR) -> i32 {
    unsafe {
        return (*va).attributes & att_noassign!();
    }
}

#[no_mangle]
pub fn getopts_bind_variable(name: *mut libc::c_char, value: *mut libc::c_char) -> i32 {
    let v: *mut SHELL_VAR;

    if legal_identifier(name) != 0 {
        v = bind_variable(name, value, 0);
        if v != std::ptr::null_mut() && (readonly_p(v) != 0 || noassign_p(v) != 0) {
            return EX_MISCERROR!();
        }

        if v != std::ptr::null_mut() {
            return EXECUTION_SUCCESS!();
        } else {
            return EXECUTION_FAILURE!();
        }
    } else {
        sh_invalidid(name);
        return EXECUTION_FAILURE!();
    }
}

#[no_mangle]
pub fn dogetopts(argc: i32, argv: *mut *mut libc::c_char) -> i32 {
    let mut ret: i32;
    let special_error: i32;
    let mut old_opterr: i32 = 0;
    let mut i: i32;
    let n: i32;

    let mut strval: [libc::c_char; 2] = [0; 2];
    let mut numval: [libc::c_char; 16] = [0; 16];
    let mut optstr: *mut libc::c_char; /* list of options */
    let name: *mut libc::c_char; /* variable to get flag val */
    let t: *mut libc::c_char;

    let mut argcc: i32 = argc;
    let mut argvv: *mut *mut libc::c_char = argv;
    if argcc < 3 {
        builtin_usage();
        return EX_USAGE;
    }
    unsafe {
        /* argv[0] is "getopts". */
        optstr = *(argvv.offset(1));
        name = *(argvv.offset(2));
        argcc -= 2;
        argvv = argvv.offset(2);
    }
    if unsafe { *optstr == ':' as libc::c_char } {
        special_error = 1;
    } else {
        special_error = 0;
    }

    if special_error != 0 {
        unsafe {
            old_opterr = sh_opterr;
            optstr = optstr.offset(1);
            sh_opterr = 0; /* suppress diagnostic messages */
        }
    }

    if argcc > 1 {
        unsafe {
            sh_getopt_restore_state(&mut (*argvv));
            t = *argvv;
            *argvv = dollar_vars[0];
            ret = sh_getopt(argcc, argvv, optstr);
            *argvv = t;
        }
    } else if unsafe { rest_of_args == std::ptr::null_mut() } {
        i = 0;
        while unsafe { i < 10 && dollar_vars[i as usize] != std::ptr::null_mut() } {
            i += 1;
        }

        unsafe {
            sh_getopt_restore_state(&mut (dollar_vars[0] as *mut libc::c_char));
            ret = sh_getopt(i, &dollar_vars[0], optstr);
        }
    } else {
        let mut words: *mut WordList;
        let v: *mut *mut libc::c_char;

        i = number_of_args() + 1; /* +1 for $0 */
        v = c_strvec_create(i + 1);
        i = 0;
        while unsafe { i < 10 && dollar_vars[i as usize] != std::ptr::null_mut() } {
            unsafe {
                *(v.offset(i as isize)) = dollar_vars[i as usize];
                i += 1;
            }
        }

        words = unsafe { rest_of_args };
        while words != std::ptr::null_mut() {
            unsafe {
                *(v.offset(i as isize)) = (*(*words).word).word;
                words = (*words).next;
            }
            i += 1;
        }
        unsafe {
            *(v.offset(i as isize)) = std::ptr::null_mut();
            sh_getopt_restore_state(&mut (*v));
            ret = sh_getopt(i, v, optstr);
            libc::free(v as *mut c_void);
        }
    }

    if special_error != 0 {
        unsafe {
            sh_opterr = old_opterr.clone();
        }
    }

    /* Set the OPTIND variable in any case, to handle "--" skipping.  It's
    highly unlikely that 14 digits will be too few. */
    if unsafe { sh_optind < 10 } {
        numval[14] = unsafe { sh_optind as libc::c_char + '0' as libc::c_char };
        numval[15] = '\0' as libc::c_char;
        i = 14;
    } else {
        i = 15;
        numval[15] = '\0' as libc::c_char;
        n = unsafe { sh_optind };

        i -= 1;
        numval[i as usize] = (n % 10) as libc::c_char + '0' as libc::c_char;
        while n / 10 != 0 {
            i -= 1;
            numval[i as usize] = (n % 10) as libc::c_char + '0' as libc::c_char;
        }
    }
    let msg = CString::new("OPTIND").unwrap();
    bind_variable(msg.as_ptr(), &mut numval[i as usize], 0);
    /* If an error occurred, decide which one it is and set the return
    code appropriately.  In all cases, the option character in error
    is in OPTOPT.  If an invalid option was encountered, OPTARG is
    NULL.  If a required option argument was missing, OPTARG points
    to a NULL string (that is, sh_optarg[0] == 0). */
    if ret == '?' as i32 {
        if unsafe { sh_optarg == std::ptr::null_mut() } {
            ret = G_INVALID_OPT!();
        } else if unsafe { *sh_optarg == '\0' as libc::c_char } {
            ret = G_ARG_MISSING!();
        }
    }

    if ret == G_EOF!() {
        let msg = CString::new("OPTARG").unwrap();
        getopts_unbind_variable(msg.as_ptr() as *mut libc::c_char);
        let msg1 = CString::new("?").unwrap();
        getopts_bind_variable(name, msg1.as_ptr() as *mut libc::c_char);
        return EXECUTION_FAILURE!();
    }

    if ret == G_INVALID_OPT!() {
        /* Invalid option encountered. */
        let msg = CString::new("?").unwrap();
        ret = getopts_bind_variable(name, msg.as_ptr() as *mut libc::c_char);

        if special_error != 0 {
            strval[0] = unsafe { sh_optopt as libc::c_char };
            strval[1] = '\0' as libc::c_char;
            let msg1 = CString::new("OPTARG").unwrap();
            bind_variable(msg1.as_ptr() as *mut libc::c_char, &mut strval[0], 0);
        } else {
            let msg = CString::new("OPTARG").unwrap();
            getopts_unbind_variable(msg.as_ptr() as *mut libc::c_char);
        }
        return ret;
    }

    if ret == G_ARG_MISSING!() {
        /* Required argument missing. */
        if special_error != 0 {
            let msg1 = CString::new(":").unwrap();
            ret = getopts_bind_variable(name, msg1.as_ptr() as *mut libc::c_char);

            strval[0] = unsafe { sh_optopt as libc::c_char };
            strval[1] = '\0' as libc::c_char;
            let msg2 = CString::new("OPTARG").unwrap();
            bind_variable(msg2.as_ptr() as *mut libc::c_char, &mut strval[0], 0);
        } else {
            let msg = CString::new("?").unwrap();
            ret = getopts_bind_variable(name, msg.as_ptr() as *mut libc::c_char);
            let msg1 = CString::new("OPTARG").unwrap();
            getopts_unbind_variable(msg1.as_ptr() as *mut libc::c_char);
        }
        return ret;
    }

    unsafe {
        let msg = CString::new("OPTARG").unwrap();
        bind_variable(msg.as_ptr() as *mut libc::c_char, sh_optarg, 0);
    }

    strval[0] = ret as libc::c_char;
    strval[1] = '\0' as libc::c_char;
    return getopts_bind_variable(name, &mut strval[0]);
}

#[no_mangle]
pub fn getopts_builtin(list: *mut WordList) -> i32 {
    let av: *mut *mut libc::c_char;
    let mut ac: i32 = 0;
    let mut ret: i32;

    if list == std::ptr::null_mut() {
        builtin_usage();
        return EX_USAGE;
    }

    reset_internal_getopt();
    let msg = CString::new("").unwrap();
    ret = internal_getopt(list, msg.as_ptr() as *mut libc::c_char);
    if ret != -1 {
        if ret == GETOPT_HELP!() {
            builtin_help();
        } else {
            builtin_usage();
        }

        return EX_USAGE;
    }
    let llist: *mut WordList = unsafe { loptend.clone() };
    av = make_builtin_argv(llist, &mut ac);
    ret = dogetopts(ac, av);
    unsafe {
        libc::free(av as *mut c_void);
    }

    return ret;
}
