/*
 * SPDX-FileCopyrightText: 2025 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: GPL-2.0-or-later
 */
use crate::src_common::*;

#[no_mangle]
pub fn sh_getopt(
    argc: libc::c_int,
    argv: *const *mut libc::c_char,
    optstring: *const libc::c_char,
) -> libc::c_int {
    let mut c: libc::c_char = 0;
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        sh_optarg = 0 as *mut libc::c_char;
        if sh_optind >= argc || sh_optind < 0 as libc::c_int {
            sh_optind = argc;
            return -(1 as libc::c_int);
        }
        if sh_optind == 0 as libc::c_int {
            sh_optind = 1 as libc::c_int;
            nextchar = 0 as *mut libc::c_void as *mut libc::c_char;
        }
        if nextchar.is_null() || *nextchar as libc::c_int == '\u{0}' as i32 {
            if sh_optind >= argc {
                return -(1 as libc::c_int);
            }
            temp = *argv.offset(sh_optind as isize);
            if *temp.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
                && *temp.offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32
                && *temp.offset(2 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
            {
                sh_optind += 1;
                return -(1 as libc::c_int);
            }
            if *temp.offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32
                || *temp.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
            {
                return -(1 as libc::c_int);
            }
            sh_curopt = sh_optind;
            nextchar = (*argv.offset(sh_curopt as isize)).offset(1 as libc::c_int as isize);
            sh_charindex = 1 as libc::c_int;
        }
        let fresh0 = nextchar;
        nextchar = nextchar.offset(1);
        c = *fresh0;
        sh_charindex += 1;
        temp = strchr(optstring, c as libc::c_int);
        sh_optopt = c as libc::c_int;
        if nextchar.is_null() || *nextchar as libc::c_int == '\u{0}' as i32 {
            sh_optind += 1;
            nextchar = 0 as *mut libc::c_void as *mut libc::c_char;
        }
        sh_badopt = (temp.is_null() || c as libc::c_int == ':' as i32) as libc::c_int;
        if sh_badopt != 0 {
            if sh_opterr != 0 {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: illegal option -- %c\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    *argv.offset(0 as libc::c_int as isize),
                    c as libc::c_int,
                );
            }
            return '?' as i32;
        }
        if *temp.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32 {
            if !nextchar.is_null() && *nextchar as libc::c_int != 0 {
                sh_optarg = nextchar;
                sh_optind += 1;
            } else if sh_optind == argc {
                if sh_opterr != 0 {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: option requires an argument -- %c\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        *argv.offset(0 as libc::c_int as isize),
                        c as libc::c_int,
                    );
                }
                sh_optopt = c as libc::c_int;
                sh_optarg = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                c = (if *optstring.offset(0 as libc::c_int as isize) as libc::c_int == ':' as i32 {
                    ':' as i32
                } else {
                    '?' as i32
                }) as libc::c_char;
            } else {
                let fresh1 = sh_optind;
                sh_optind = sh_optind + 1;
                sh_optarg = *argv.offset(fresh1 as isize);
            }
            nextchar = 0 as *mut libc::c_void as *mut libc::c_char;
        }
    }
    return c as libc::c_int;
}
#[no_mangle]
pub fn sh_getopt_restore_state(argv: *mut *mut libc::c_char) {
    unsafe {
        if !nextchar.is_null() {
            nextchar = (*argv.offset(sh_curopt as isize)).offset(sh_charindex as isize);
        }
    }
}
#[no_mangle]
pub fn sh_getopt_alloc_istate() -> *mut sh_getopt_state_t {
    unsafe {
        let mut ret: *mut sh_getopt_state_t = 0 as *mut sh_getopt_state_t;
        ret = malloc(::std::mem::size_of::<sh_getopt_state_t>() as usize) as *mut sh_getopt_state_t;
        return ret;
    }
}
#[no_mangle]
pub fn sh_getopt_dispose_istate(gs: *mut sh_getopt_state_t) {
    unsafe {
        free(gs as *mut libc::c_void);
    }
}
#[no_mangle]
pub fn sh_getopt_save_istate() -> *mut sh_getopt_state_t {
    unsafe {
        let mut ret: *mut sh_getopt_state_t = 0 as *mut sh_getopt_state_t;
        ret = sh_getopt_alloc_istate();
        let ref mut fresh2 = (*ret).gs_optarg;
        *fresh2 = sh_optarg;
        (*ret).gs_optind = sh_optind;
        (*ret).gs_curopt = sh_curopt;
        let ref mut fresh3 = (*ret).gs_nextchar;
        *fresh3 = nextchar;
        (*ret).gs_charindex = sh_charindex;
        (*ret).gs_flags = 0 as libc::c_int;
        return ret;
    }
}
#[no_mangle]
pub fn sh_getopt_restore_istate(state: *mut sh_getopt_state_t) {
    unsafe {
        sh_optarg = (*state).gs_optarg;
        sh_optind = (*state).gs_optind;
        sh_curopt = (*state).gs_curopt;
        nextchar = (*state).gs_nextchar;
        sh_charindex = (*state).gs_charindex;
        sh_getopt_dispose_istate(state);
    }
}
