/*
 * SPDX-FileCopyrightText: 2025 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: GPL-2.0-or-later
 */
use crate::src_common::*;
use std::convert::TryInto;

#[no_mangle]
pub fn uwp_init() {
    unsafe {
        uwcache.data = libc::malloc(
            (128 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut UNWIND_ELT>() as libc::c_ulong)
                .try_into()
                .unwrap(),
        );
        uwcache.cs = 128 as libc::c_int;
        uwcache.nc = 0 as libc::c_int;
    }
}
fn without_interrupts(
    mut function: Option<VFunction>,
    mut arg1: *mut libc::c_char,
    mut arg2: *mut libc::c_char,
) {
    unsafe {
        ::std::mem::transmute::<_, fn(_, _)>(
            (Some(function.expect("non-null function pointer")))
                .expect("non-null function pointer"),
        )(arg1, arg2);
    } //unsafe
}
#[no_mangle]
pub fn begin_unwind_frame(mut tag: *mut libc::c_char) {
    unsafe {
        add_unwind_protect(
            ::std::mem::transmute::<*mut libc::c_void, Option<Function>>(0 as *mut libc::c_void),
            tag,
        );
    } //unsafe
}
#[no_mangle]
pub fn discard_unwind_frame(mut tag: *mut libc::c_char) {
    unsafe {
        if !unwind_protect_list.is_null() {
            without_interrupts(
                ::std::mem::transmute::<
                    Option<fn(*mut libc::c_char, *mut libc::c_char) -> ()>,
                    Option<VFunction>,
                >(Some(unwind_frame_discard_internal)),
                tag,
                0 as *mut libc::c_void as *mut libc::c_char,
            );
        }
    } //unsafe
}
#[no_mangle]
pub fn run_unwind_frame(mut tag: *mut libc::c_char) {
    unsafe {
        if !unwind_protect_list.is_null() {
            without_interrupts(
                ::std::mem::transmute::<
                    Option<fn(*mut libc::c_char, *mut libc::c_char) -> ()>,
                    Option<VFunction>,
                >(Some(unwind_frame_run_internal)),
                tag,
                0 as *mut libc::c_void as *mut libc::c_char,
            );
        }
    } //unsafe
}
#[no_mangle]
pub fn add_unwind_protect(mut cleanup: Option<Function>, mut arg: *mut libc::c_char) {
    unsafe {
        without_interrupts(
            ::std::mem::transmute::<
                Option<fn(Option<Function>, *mut libc::c_char) -> ()>,
                Option<VFunction>,
            >(Some(add_unwind_protect_internal)),
            ::std::mem::transmute::<Option<Function>, *mut libc::c_char>(cleanup),
            arg,
        );
    } //unsafe
}
#[no_mangle]
pub fn remove_unwind_protect() {
    unsafe {
        if !unwind_protect_list.is_null() {
            without_interrupts(
                ::std::mem::transmute::<
                    Option<fn(*mut libc::c_char, *mut libc::c_char) -> ()>,
                    Option<VFunction>,
                >(Some(remove_unwind_protect_internal)),
                0 as *mut libc::c_void as *mut libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_char,
            );
        }
    } //unsafe
}
#[no_mangle]
pub fn run_unwind_protects() {
    unsafe {
        if !unwind_protect_list.is_null() {
            without_interrupts(
                ::std::mem::transmute::<
                    Option<fn(*mut libc::c_char, *mut libc::c_char) -> ()>,
                    Option<VFunction>,
                >(Some(run_unwind_protects_internal)),
                0 as *mut libc::c_void as *mut libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_char,
            );
        }
    } //unsafe
}
#[no_mangle]
pub fn clear_unwind_protect_list(mut flags: libc::c_int) {
    unsafe {
        let mut flag: *mut libc::c_char = 0 as *mut libc::c_char;
        if !unwind_protect_list.is_null() {
            flag = (if flags != 0 {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                0 as *mut libc::c_void as *mut libc::c_char
            }) as *mut libc::c_char;
            without_interrupts(
                ::std::mem::transmute::<
                    Option<fn(*mut libc::c_char, *mut libc::c_char) -> ()>,
                    Option<VFunction>,
                >(Some(clear_unwind_protects_internal)),
                flag,
                0 as *mut libc::c_void as *mut libc::c_char,
            );
        }
    } //unsafe
}
#[no_mangle]
pub fn have_unwind_protects() -> libc::c_int {
    unsafe {
        return (unwind_protect_list != 0 as *mut UNWIND_ELT) as libc::c_int;
    }
}
#[no_mangle]
pub fn unwind_protect_tag_on_stack(mut tag: *const libc::c_char) -> libc::c_int {
    unsafe {
        let mut elt: *mut UNWIND_ELT = 0 as *mut UNWIND_ELT;
        elt = unwind_protect_list;
        while !elt.is_null() {
            if ((*elt).head.cleanup).is_none()
                && (*((*elt).arg.v).offset(0 as libc::c_int as isize) as libc::c_int
                    == *tag.offset(0 as libc::c_int as isize) as libc::c_int
                    && strcmp((*elt).arg.v, tag) == 0 as libc::c_int)
            {
                return 1 as libc::c_int;
            }
            elt = (*elt).head.next;
        }
        return 0 as libc::c_int;
    } //unsafe
}
fn add_unwind_protect_internal(
    //need
    mut cleanup: Option<Function>,
    mut arg: *mut libc::c_char,
) {
    unsafe {
        let mut elt: *mut UNWIND_ELT = 0 as *mut UNWIND_ELT;
        if uwcache.nc > 0 as libc::c_int {
            uwcache.nc -= 1;
            elt = *(uwcache.data as *mut *mut UNWIND_ELT).offset(uwcache.nc as isize);
        } else {
            elt = libc::malloc(
                (::std::mem::size_of::<UNWIND_ELT>() as libc::c_ulong)
                    .try_into()
                    .unwrap(),
            ) as *mut UNWIND_ELT;
        }
        let ref mut fresh0 = (*elt).head.next;
        *fresh0 = unwind_protect_list;
        let ref mut fresh1 = (*elt).head.cleanup;
        *fresh1 = cleanup;
        let ref mut fresh2 = (*elt).arg.v;
        *fresh2 = arg;
        unwind_protect_list = elt;
    } //unsafe
}
fn remove_unwind_protect_internal(
    //need
    mut ignore1: *mut libc::c_char,
    mut ignore2: *mut libc::c_char,
) {
    unsafe {
        let mut elt: *mut UNWIND_ELT = 0 as *mut UNWIND_ELT;
        elt = unwind_protect_list;
        if !elt.is_null() {
            unwind_protect_list = (*unwind_protect_list).head.next;
            if uwcache.nc < uwcache.cs {
                if ::std::mem::size_of::<UNWIND_ELT>() as libc::c_ulong
                    <= 32 as libc::c_int as libc::c_ulong
                {
                    let mut mzp: *mut libc::c_char = elt as *mut libc::c_char;
                    let mut mctmp: libc::c_ulong =
                        ::std::mem::size_of::<UNWIND_ELT>() as libc::c_ulong;
                    let mut mcn: libc::c_long = 0;
                    if mctmp < 8 as libc::c_int as libc::c_ulong {
                        mcn = 0 as libc::c_int as libc::c_long;
                    } else {
                        mcn = mctmp
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_div(8 as libc::c_int as libc::c_ulong)
                            as libc::c_long;
                        mctmp &= 7 as libc::c_int as libc::c_ulong;
                    }
                    let mut loop_one = true;
                    loop {
                        if loop_one == true {
                            match mctmp {
                                0 => {
                                    let xch = mzp;
                                    mzp = mzp.offset(8);
                                    *xch = 0xdf as libc::c_int as libc::c_char;
                                }
                                7 => {
                                    let xch = mzp;
                                    mzp = mzp.offset(7);
                                    *xch = 0xdf as libc::c_int as libc::c_char;
                                }
                                6 => {
                                    let xch = mzp;
                                    mzp = mzp.offset(6);
                                    *xch = 0xdf as libc::c_int as libc::c_char;
                                }
                                5 => {
                                    let xch = mzp;
                                    mzp = mzp.offset(5);
                                    *xch = 0xdf as libc::c_int as libc::c_char;
                                }
                                4 => {
                                    let xch = mzp;
                                    mzp = mzp.offset(4);
                                    *xch = 0xdf as libc::c_int as libc::c_char;
                                }
                                3 => {
                                    let xch = mzp;
                                    mzp = mzp.offset(3);
                                    *xch = 0xdf as libc::c_int as libc::c_char;
                                }
                                2 => {
                                    let xch = mzp;
                                    mzp = mzp.offset(2);
                                    *xch = 0xdf as libc::c_int as libc::c_char;
                                }
                                1 => {
                                    let xch = mzp;
                                    mzp = mzp.offset(1);
                                    *xch = 0xdf as libc::c_int as libc::c_char;
                                }
                                _ => {}
                            }
                        } else {
                            let xch = mzp;
                            mzp = mzp.offset(8);
                            *xch = 0xdf as libc::c_int as libc::c_char;
                        }
                        if mcn <= 0 as libc::c_int as libc::c_long {
                            break;
                        }
                        mcn -= 1;
                        loop_one = false;
                    }
                } else {
                    memset(
                        elt as *mut libc::c_void,
                        0xdf as libc::c_int,
                        (::std::mem::size_of::<UNWIND_ELT>() as libc::c_ulong)
                            .try_into()
                            .unwrap(),
                    );
                }
                let fresh11 = uwcache.nc;
                uwcache.nc = uwcache.nc + 1;
                let ref mut fresh12 =
                    *(uwcache.data as *mut *mut UNWIND_ELT).offset(fresh11 as isize);
                *fresh12 = elt;
            } else {
                libc::free(elt as *mut libc::c_void);
            }
        }
    } //unsafe
}
fn run_unwind_protects_internal(
    //need
    mut ignore1: *mut libc::c_char,
    mut ignore2: *mut libc::c_char,
) {
    unwind_frame_run_internal(
        0 as *mut libc::c_void as *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_char,
    );
}
fn clear_unwind_protects_internal(
    //need
    mut flag: *mut libc::c_char,
    mut ignore: *mut libc::c_char,
) {
    unsafe {
        if !flag.is_null() {
            while !unwind_protect_list.is_null() {
                remove_unwind_protect_internal(
                    0 as *mut libc::c_void as *mut libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
            }
        }
        unwind_protect_list = 0 as *mut libc::c_void as *mut UNWIND_ELT;
    } //unsafe
}
fn unwind_frame_discard_internal(
    //need
    mut tag: *mut libc::c_char,
    mut ignore: *mut libc::c_char,
) {
    unsafe {
        let mut elt: *mut UNWIND_ELT = 0 as *mut UNWIND_ELT;
        let mut found: libc::c_int = 0;
        found = 0 as libc::c_int;
        loop {
            elt = unwind_protect_list;
            if elt.is_null() {
                break;
            }
            unwind_protect_list = (*unwind_protect_list).head.next;
            if ((*elt).head.cleanup).is_none()
                && (*((*elt).arg.v).offset(0 as libc::c_int as isize) as libc::c_int
                    == *tag.offset(0 as libc::c_int as isize) as libc::c_int
                    && strcmp((*elt).arg.v, tag) == 0 as libc::c_int)
            {
                if uwcache.nc < uwcache.cs {
                    if ::std::mem::size_of::<UNWIND_ELT>() as libc::c_ulong
                        <= 32 as libc::c_int as libc::c_ulong
                    {
                        let mut mzp: *mut libc::c_char = elt as *mut libc::c_char;
                        let mut mctmp: libc::c_ulong =
                            ::std::mem::size_of::<UNWIND_ELT>() as libc::c_ulong;
                        let mut mcn: libc::c_long = 0;
                        if mctmp < 8 as libc::c_int as libc::c_ulong {
                            mcn = 0 as libc::c_int as libc::c_long;
                        } else {
                            mcn = mctmp
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_div(8 as libc::c_int as libc::c_ulong)
                                as libc::c_long;
                            mctmp &= 7 as libc::c_int as libc::c_ulong;
                        }
                        let mut loop_one = true;
                        loop {
                            if loop_one == true {
                                match mctmp {
                                    0 => {
                                        let xch = mzp;
                                        mzp = mzp.offset(8);
                                        *xch = 0xdf as libc::c_int as libc::c_char;
                                    }
                                    7 => {
                                        let xch = mzp;
                                        mzp = mzp.offset(7);
                                        *xch = 0xdf as libc::c_int as libc::c_char;
                                    }
                                    6 => {
                                        let xch = mzp;
                                        mzp = mzp.offset(6);
                                        *xch = 0xdf as libc::c_int as libc::c_char;
                                    }
                                    5 => {
                                        let xch = mzp;
                                        mzp = mzp.offset(5);
                                        *xch = 0xdf as libc::c_int as libc::c_char;
                                    }
                                    4 => {
                                        let xch = mzp;
                                        mzp = mzp.offset(4);
                                        *xch = 0xdf as libc::c_int as libc::c_char;
                                    }
                                    3 => {
                                        let xch = mzp;
                                        mzp = mzp.offset(3);
                                        *xch = 0xdf as libc::c_int as libc::c_char;
                                    }
                                    2 => {
                                        let xch = mzp;
                                        mzp = mzp.offset(2);
                                        *xch = 0xdf as libc::c_int as libc::c_char;
                                    }
                                    1 => {
                                        let xch = mzp;
                                        mzp = mzp.offset(1);
                                        *xch = 0xdf as libc::c_int as libc::c_char;
                                    }
                                    _ => {}
                                }
                            } else {
                                let xch = mzp;
                                mzp = mzp.offset(8);
                                *xch = 0xdf as libc::c_int as libc::c_char;
                            }
                            if mcn <= 0 as libc::c_int as libc::c_long {
                                break;
                            }
                            mcn -= 1;
                            loop_one = false;
                        }
                    } else {
                        memset(
                            elt as *mut libc::c_void,
                            0xdf as libc::c_int,
                            (::std::mem::size_of::<UNWIND_ELT>() as libc::c_ulong)
                                .try_into()
                                .unwrap(),
                        );
                    }
                    let fresh21 = uwcache.nc;
                    uwcache.nc = uwcache.nc + 1;
                    let ref mut fresh22 =
                        *(uwcache.data as *mut *mut UNWIND_ELT).offset(fresh21 as isize);
                    *fresh22 = elt;
                } else {
                    libc::free(elt as *mut libc::c_void);
                }
                found = 1 as libc::c_int;
                break;
            } else if uwcache.nc < uwcache.cs {
                if ::std::mem::size_of::<UNWIND_ELT>() as libc::c_ulong
                    <= 32 as libc::c_int as libc::c_ulong
                {
                    let mut mzp_0: *mut libc::c_char = elt as *mut libc::c_char;
                    let mut mctmp_0: libc::c_ulong =
                        ::std::mem::size_of::<UNWIND_ELT>() as libc::c_ulong;
                    let mut mcn_0: libc::c_long = 0;
                    if mctmp_0 < 8 as libc::c_int as libc::c_ulong {
                        mcn_0 = 0 as libc::c_int as libc::c_long;
                    } else {
                        mcn_0 = mctmp_0
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_div(8 as libc::c_int as libc::c_ulong)
                            as libc::c_long;
                        mctmp_0 &= 7 as libc::c_int as libc::c_ulong;
                    }
                    let mut loop_one = true;
                    loop {
                        if loop_one == true {
                            match mctmp_0 {
                                0 => {
                                    let xch = mzp_0;
                                    mzp_0 = mzp_0.offset(8);
                                    *xch = 0xdf as libc::c_int as libc::c_char;
                                }
                                7 => {
                                    let xch = mzp_0;
                                    mzp_0 = mzp_0.offset(7);
                                    *xch = 0xdf as libc::c_int as libc::c_char;
                                }
                                6 => {
                                    let xch = mzp_0;
                                    mzp_0 = mzp_0.offset(6);
                                    *xch = 0xdf as libc::c_int as libc::c_char;
                                }
                                5 => {
                                    let xch = mzp_0;
                                    mzp_0 = mzp_0.offset(5);
                                    *xch = 0xdf as libc::c_int as libc::c_char;
                                }
                                4 => {
                                    let xch = mzp_0;
                                    mzp_0 = mzp_0.offset(4);
                                    *xch = 0xdf as libc::c_int as libc::c_char;
                                }
                                3 => {
                                    let xch = mzp_0;
                                    mzp_0 = mzp_0.offset(3);
                                    *xch = 0xdf as libc::c_int as libc::c_char;
                                }
                                2 => {
                                    let xch = mzp_0;
                                    mzp_0 = mzp_0.offset(2);
                                    *xch = 0xdf as libc::c_int as libc::c_char;
                                }
                                1 => {
                                    let xch = mzp_0;
                                    mzp_0 = mzp_0.offset(1);
                                    *xch = 0xdf as libc::c_int as libc::c_char;
                                }
                                _ => {}
                            }
                        } else {
                            let xch = mzp_0;
                            mzp_0 = mzp_0.offset(8);
                            *xch = 0xdf as libc::c_int as libc::c_char;
                        }
                        if mcn_0 <= 0 as libc::c_int as libc::c_long {
                            break;
                        }
                        mcn_0 -= 1;
                        loop_one = false;
                    }
                } else {
                    memset(
                        elt as *mut libc::c_void,
                        0xdf as libc::c_int,
                        (::std::mem::size_of::<UNWIND_ELT>() as libc::c_ulong)
                            .try_into()
                            .unwrap(),
                    );
                }
                let fresh31 = uwcache.nc;
                uwcache.nc = uwcache.nc + 1;
                let ref mut fresh32 =
                    *(uwcache.data as *mut *mut UNWIND_ELT).offset(fresh31 as isize);
                *fresh32 = elt;
            } else {
                libc::free(elt as *mut libc::c_void);
            }
        }
        if found == 0 as libc::c_int {
            internal_warning(
                b"unwind_frame_discard: %s: frame not found\0" as *const u8 as *const libc::c_char,
                tag,
            );
        }
    } //unsafe
}
#[inline]
fn restore_variable(mut sv: *mut SAVED_VAR) {
    //need
    unsafe {
        libc::memcpy(
            (*sv).variable as *mut libc::c_void,
            ((*sv).desired_setting).as_mut_ptr() as *const libc::c_void,
            (*sv).size as libc::c_ulong as libc::size_t,
        );
    } //unsafe
}
fn unwind_frame_run_internal(
    //need
    mut tag: *mut libc::c_char,
    mut ignore: *mut libc::c_char,
) {
    unsafe {
        let mut elt: *mut UNWIND_ELT = 0 as *mut UNWIND_ELT;
        let mut found: libc::c_int = 0;
        found = 0 as libc::c_int;
        loop {
            elt = unwind_protect_list;
            if elt.is_null() {
                break;
            }
            unwind_protect_list = (*elt).head.next;
            if ((*elt).head.cleanup).is_none() {
                if !tag.is_null()
                    && (*((*elt).arg.v).offset(0 as libc::c_int as isize) as libc::c_int
                        == *tag.offset(0 as libc::c_int as isize) as libc::c_int
                        && strcmp((*elt).arg.v, tag) == 0 as libc::c_int)
                {
                    if uwcache.nc < uwcache.cs {
                        if ::std::mem::size_of::<UNWIND_ELT>() as libc::c_ulong
                            <= 32 as libc::c_int as libc::c_ulong
                        {
                            let mut mzp: *mut libc::c_char = elt as *mut libc::c_char;
                            let mut mctmp: libc::c_ulong =
                                ::std::mem::size_of::<UNWIND_ELT>() as libc::c_ulong;
                            let mut mcn: libc::c_long = 0;
                            if mctmp < 8 as libc::c_int as libc::c_ulong {
                                mcn = 0 as libc::c_int as libc::c_long;
                            } else {
                                mcn = mctmp
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_div(8 as libc::c_int as libc::c_ulong)
                                    as libc::c_long;
                                mctmp &= 7 as libc::c_int as libc::c_ulong;
                            }
                            let mut loop_one = true;
                            loop {
                                if loop_one == true {
                                    match mctmp {
                                        0 => {
                                            let xch = mzp;
                                            mzp = mzp.offset(8);
                                            *xch = 0xdf as libc::c_int as libc::c_char;
                                        }
                                        7 => {
                                            let xch = mzp;
                                            mzp = mzp.offset(7);
                                            *xch = 0xdf as libc::c_int as libc::c_char;
                                        }
                                        6 => {
                                            let xch = mzp;
                                            mzp = mzp.offset(6);
                                            *xch = 0xdf as libc::c_int as libc::c_char;
                                        }
                                        5 => {
                                            let xch = mzp;
                                            mzp = mzp.offset(5);
                                            *xch = 0xdf as libc::c_int as libc::c_char;
                                        }
                                        4 => {
                                            let xch = mzp;
                                            mzp = mzp.offset(4);
                                            *xch = 0xdf as libc::c_int as libc::c_char;
                                        }
                                        3 => {
                                            let xch = mzp;
                                            mzp = mzp.offset(3);
                                            *xch = 0xdf as libc::c_int as libc::c_char;
                                        }
                                        2 => {
                                            let xch = mzp;
                                            mzp = mzp.offset(2);
                                            *xch = 0xdf as libc::c_int as libc::c_char;
                                        }
                                        1 => {
                                            let xch = mzp;
                                            mzp = mzp.offset(1);
                                            *xch = 0xdf as libc::c_int as libc::c_char;
                                        }
                                        _ => {}
                                    }
                                } else {
                                    let xch = mzp;
                                    mzp = mzp.offset(8);
                                    *xch = 0xdf as libc::c_int as libc::c_char;
                                }
                                if mcn <= 0 as libc::c_int as libc::c_long {
                                    break;
                                }
                                mcn -= 1;
                                loop_one = false;
                            }
                        } else {
                            memset(
                                elt as *mut libc::c_void,
                                0xdf as libc::c_int,
                                (::std::mem::size_of::<UNWIND_ELT>() as libc::c_ulong)
                                    .try_into()
                                    .unwrap(),
                            );
                        }
                        let fresh41 = uwcache.nc;
                        uwcache.nc = uwcache.nc + 1;
                        let ref mut fresh42 =
                            *(uwcache.data as *mut *mut UNWIND_ELT).offset(fresh41 as isize);
                        *fresh42 = elt;
                    } else {
                        libc::free(elt as *mut libc::c_void);
                    }
                    found = 1 as libc::c_int;
                    break;
                }
            } else if (*elt).head.cleanup
                == ::std::mem::transmute::<Option<fn() -> ()>, Option<Function>>(Some(
                    ::std::mem::transmute::<fn(*mut SAVED_VAR) -> (), fn() -> ()>(restore_variable),
                ))
            {
                restore_variable(&mut (*elt).sv.v);
            } else {
                ::std::mem::transmute::<_, fn(_) -> libc::c_int>(
                    (Some(((*elt).head.cleanup).expect("non-null function pointer")))
                        .expect("non-null function pointer"),
                )((*elt).arg.v);
            }
            if uwcache.nc < uwcache.cs {
                if ::std::mem::size_of::<UNWIND_ELT>() as libc::c_ulong
                    <= 32 as libc::c_int as libc::c_ulong
                {
                    let mut mzp_0: *mut libc::c_char = elt as *mut libc::c_char;
                    let mut mctmp_0: libc::c_ulong =
                        ::std::mem::size_of::<UNWIND_ELT>() as libc::c_ulong;
                    let mut mcn_0: libc::c_long = 0;
                    if mctmp_0 < 8 as libc::c_int as libc::c_ulong {
                        mcn_0 = 0 as libc::c_int as libc::c_long;
                    } else {
                        mcn_0 = mctmp_0
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_div(8 as libc::c_int as libc::c_ulong)
                            as libc::c_long;
                        mctmp_0 &= 7 as libc::c_int as libc::c_ulong;
                    }
                    let mut loop_one = true;
                    loop {
                        if loop_one == true {
                            match mctmp_0 {
                                0 => {
                                    let xch = mzp_0;
                                    mzp_0 = mzp_0.offset(8);
                                    *xch = 0xdf as libc::c_int as libc::c_char;
                                }
                                7 => {
                                    let xch = mzp_0;
                                    mzp_0 = mzp_0.offset(7);
                                    *xch = 0xdf as libc::c_int as libc::c_char;
                                }
                                6 => {
                                    let xch = mzp_0;
                                    mzp_0 = mzp_0.offset(6);
                                    *xch = 0xdf as libc::c_int as libc::c_char;
                                }
                                5 => {
                                    let xch = mzp_0;
                                    mzp_0 = mzp_0.offset(5);
                                    *xch = 0xdf as libc::c_int as libc::c_char;
                                }
                                4 => {
                                    let xch = mzp_0;
                                    mzp_0 = mzp_0.offset(4);
                                    *xch = 0xdf as libc::c_int as libc::c_char;
                                }
                                3 => {
                                    let xch = mzp_0;
                                    mzp_0 = mzp_0.offset(3);
                                    *xch = 0xdf as libc::c_int as libc::c_char;
                                }
                                2 => {
                                    let xch = mzp_0;
                                    mzp_0 = mzp_0.offset(2);
                                    *xch = 0xdf as libc::c_int as libc::c_char;
                                }
                                1 => {
                                    let xch = mzp_0;
                                    mzp_0 = mzp_0.offset(1);
                                    *xch = 0xdf as libc::c_int as libc::c_char;
                                }
                                _ => {}
                            }
                        } else {
                            let xch = mzp_0;
                            mzp_0 = mzp_0.offset(8);
                            *xch = 0xdf as libc::c_int as libc::c_char;
                        }
                        if mcn_0 <= 0 as libc::c_int as libc::c_long {
                            break;
                        }
                        mcn_0 -= 1;
                        loop_one = false;
                    }
                } else {
                    memset(
                        elt as *mut libc::c_void,
                        0xdf as libc::c_int,
                        (::std::mem::size_of::<UNWIND_ELT>() as libc::c_ulong)
                            .try_into()
                            .unwrap(),
                    );
                }
                let fresh51 = uwcache.nc;
                uwcache.nc = uwcache.nc + 1;
                let ref mut fresh52 =
                    *(uwcache.data as *mut *mut UNWIND_ELT).offset(fresh51 as isize);
                *fresh52 = elt;
            } else {
                libc::free(elt as *mut libc::c_void);
            }
        }
        if !tag.is_null() && found == 0 as libc::c_int {
            internal_warning(
                b"unwind_frame_run: %s: frame not found\0" as *const u8 as *const libc::c_char,
                tag,
            );
        }
    } //unsafe
}
fn unwind_protect_mem_internal(
    //need
    mut var: *mut libc::c_char,
    mut psize: *mut libc::c_char,
) {
    unsafe {
        let mut size: libc::c_int = 0;
        let mut allocated: libc::c_int = 0;
        let mut elt: *mut UNWIND_ELT = 0 as *mut UNWIND_ELT;
        size = *(psize as *mut libc::c_int);
        allocated = (size as libc::c_ulong).wrapping_add(28 as libc::c_ulong) as libc::c_int;
        if (allocated as libc::c_ulong) < ::std::mem::size_of::<UNWIND_ELT>() as libc::c_ulong {
            allocated = ::std::mem::size_of::<UNWIND_ELT>() as libc::c_ulong as libc::c_int;
        }
        elt = libc::malloc((allocated as size_t).try_into().unwrap()) as *mut UNWIND_ELT;
        let ref mut fresh53 = (*elt).head.next;
        *fresh53 = unwind_protect_list;
        let ref mut fresh54 = (*elt).head.cleanup;
        *fresh54 = ::std::mem::transmute::<Option<fn() -> ()>, Option<Function>>(Some(
            ::std::mem::transmute::<fn(*mut SAVED_VAR) -> (), fn() -> ()>(restore_variable),
        ));
        let ref mut fresh55 = (*elt).sv.v.variable;
        *fresh55 = var;
        (*elt).sv.v.size = size;
        libc::memcpy(
            ((*elt).sv.v.desired_setting).as_mut_ptr() as *mut libc::c_void,
            var as *const libc::c_void,
            size as libc::c_ulong as libc::size_t,
        );
        unwind_protect_list = elt;
    }
}
#[no_mangle]
pub fn unwind_protect_mem(mut var: *mut libc::c_char, mut size: libc::c_int) {
    unsafe {
        without_interrupts(
            Some(::std::mem::transmute::<
                fn(*mut libc::c_char, *mut libc::c_char) -> (),
                fn() -> (),
            >(unwind_protect_mem_internal)),
            var,
            &mut size as *mut libc::c_int as *mut libc::c_char,
        );
    } //unsafe
}
