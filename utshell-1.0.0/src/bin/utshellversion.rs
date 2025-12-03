/*
 * SPDX-FileCopyrightText: 2025 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: GPL-2.0-or-later
 */
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![allow(clippy::all)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::zero_ptr)]
#![allow(clippy::unnecessary_cast)]

use builtins::version::{dist_version, patch_level, shell_version_string, show_shell_version};
use libc::FILE;

extern "C" {
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    static mut optind: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn exit(_: libc::c_int) -> !;
}

pub const EOF: libc::c_int = -1;
pub const RFLAG: libc::c_int = 0x0001;
pub const VFLAG: libc::c_int = 0x0002;
pub const MFLAG: libc::c_int = 0x0004;
pub const PFLAG: libc::c_int = 0x0008;
pub const SFLAG: libc::c_int = 0x0010;
pub const LFLAG: libc::c_int = 0x0020;
pub const XFLAG: libc::c_int = 0x0040;

pub type size_t = libc::c_ulong;
pub type __off64_t = libc::c_long;
pub type __off_t = libc::c_long;

#[macro_export]
macro_rules! MACHTYPE {
    () => {
        (b"x86_64-pc-linux-gnu\0" as *const u8 as *const libc::c_char)
    };
}

#[no_mangle]
static mut shell_name_rename: *mut libc::c_char =
    b"utshell\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut progname: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn usage() {
    fprintf(
        stderr,
        b"%s: usage: %s [-hrvpmlsx]\n\0" as *const u8 as *const libc::c_char,
        progname,
        progname,
    );
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut opt: libc::c_int = 0;
    let mut oflags: libc::c_int = 0;
    let mut dv: [libc::c_char; 128] = [0; 128];
    let mut rv: *mut libc::c_char = 0 as *mut libc::c_char;

    progname = strrchr(*argv.offset(0 as libc::c_int as isize), '/' as i32);
    if !progname.is_null() {
        progname = progname.offset(1);
    } else {
        progname = *argv.offset(0 as libc::c_int as isize);
    }

    oflags = 0 as libc::c_int;
    loop {
        opt = getopt(
            argc,
            argv,
            b"hrvmpslx\0" as *const u8 as *const libc::c_char,
        );
        if !(opt != EOF) {
            break;
        }
        match opt as u8 as char {
            'h' => {
                usage();
                exit(0 as libc::c_int);
            }
            'r' => {
                oflags |= RFLAG; /* release */
            }
            'v' => {
                oflags |= VFLAG; /* version */
            }
            'm' => {
                oflags |= MFLAG; /* machtype */
            }
            'p' => {
                oflags |= PFLAG; /* patchlevel */
            }
            's' => {
                oflags |= SFLAG; /* short version string */
            }
            'l' => {
                oflags |= LFLAG; /* long version string */
            }
            'x' => {
                oflags |= XFLAG; /* extended version information */
            }
            _ => {
                usage();
                exit(2 as libc::c_int);
            }
        }
    }
    argc -= optind;
    argv = argv.offset(optind as isize);

    if argc > 0 as libc::c_int {
        usage();
        exit(2 as libc::c_int);
    }

    /* default behavior */
    if oflags == 0 as libc::c_int {
        oflags = SFLAG;
    }
    if oflags & (RFLAG | VFLAG) != 0 {
        strcpy(dv.as_mut_ptr(), dist_version);
        rv = strchr(dv.as_mut_ptr(), '.' as i32);
        if !rv.is_null() {
            let fresh0 = rv;
            rv = rv.offset(1);
            *fresh0 = '\0' as i32 as libc::c_char;
        } else {
            rv = b"00\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    }
    if oflags & RFLAG != 0 {
        printf(
            b"%s\n\0" as *const u8 as *const libc::c_char,
            dv.as_mut_ptr(),
        );
    } else if oflags & VFLAG != 0 {
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, rv);
    } else if oflags & MFLAG != 0 {
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, MACHTYPE!());
    } else if oflags & PFLAG != 0 {
        printf(b"%d\n\0" as *const u8 as *const libc::c_char, patch_level);
    } else if oflags & SFLAG != 0 {
        printf(
            b"%s\n\0" as *const u8 as *const libc::c_char,
            shell_version_string(),
        );
    } else if oflags & LFLAG != 0 {
        show_shell_version(0 as libc::c_int);
    } else if oflags & XFLAG != 0 {
        show_shell_version(1 as libc::c_int);
    }
    exit(0 as libc::c_int);
}

pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
