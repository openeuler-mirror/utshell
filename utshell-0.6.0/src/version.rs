/*
 * SPDX-FileCopyrightText: 2025 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: GPL-2.0-or-later
 */
use crate::src_common::*;
use std::convert::TryInto;

/* The distribution version number of this shell. */
#[no_mangle]
pub static mut dist_version: *const libc::c_char = b"0.5\0" as *const u8 as *const libc::c_char;

/* It's important that there be no other strings in this file that match the
regexp `^#define[ 	]*PATCHLEVEL', since that's what support/mkversion.sh
looks for to find the patch level (for the sccs version string). */
#[no_mangle]
pub static mut patch_level: libc::c_int = 0 as libc::c_int;

/* The last built version of this shell. */
#[no_mangle]
pub static mut build_version: libc::c_int = 1 as libc::c_int;

/* The release status of this shell. */
#[no_mangle]
pub static mut release_status: *const libc::c_char =
    b"release\0" as *const u8 as *const libc::c_char;

/* A version string for use by sccs and the what command. */
#[no_mangle]
pub static mut sccs_version: *const libc::c_char =
    b"@(#)utshell version 0.5 release GNU\0" as *const u8 as *const libc::c_char;

#[no_mangle]
pub static mut bash_copyright: *const libc::c_char =
    b"Copyright (C) 2020 Free Software Foundation, Inc.\0" as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut bash_license: *const libc::c_char =
    b"License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>\n\0"
        as *const u8 as *const libc::c_char;

#[no_mangle]
pub static mut shell_compatibility_level: libc::c_int = 51 as libc::c_int;
#[no_mangle]
pub fn shell_version_string() -> *mut libc::c_char {
    unsafe {
        static mut tt: [libc::c_char; 32] = ['\u{0}' as i32 as libc::c_char; 32];
        if tt[0 as libc::c_int as usize] as libc::c_int == '\u{0}' as i32 {
            if !release_status.is_null() {
                snprintf(
                    tt.as_mut_ptr() as *mut libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
                        .try_into()
                        .unwrap(),
                    b"%s.%d(%d)-%s\0" as *const u8 as *const libc::c_char,
                    dist_version,
                    patch_level,
                    build_version,
                    release_status,
                );
            } else {
                snprintf(
                    tt.as_mut_ptr() as *mut libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
                        .try_into()
                        .unwrap(),
                    b"%s.%d(%d)\0" as *const u8 as *const libc::c_char,
                    dist_version,
                    patch_level,
                    build_version,
                );
            }
        }
        return tt.as_mut_ptr() as *mut libc::c_char;
    }
}
#[no_mangle]
pub fn show_shell_version(mut extended: libc::c_int) {
    unsafe {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"GNU utshell, version %s (%s)\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            shell_version_string(),
            get_mach_type(),
        );
        if extended != 0 {
            printf(
                b"%s\n\0" as *const u8 as *const libc::c_char,
                dcgettext(0 as *const libc::c_char, bash_copyright, 5 as libc::c_int),
            );
            printf(
                b"%s\n\0" as *const u8 as *const libc::c_char,
                dcgettext(0 as *const libc::c_char, bash_license, 5 as libc::c_int),
            );
            printf(
                b"%s\n\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"This is free software; you are free to change and redistribute it.\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            printf(
                b"%s\n\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"There is NO WARRANTY, to the extent permitted by law.\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    }
}
