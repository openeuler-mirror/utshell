/*
 * SPDX-FileCopyrightText: 2025 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: GPL-2.0-or-later
 */
use crate::src_common::*;
use crate::stringlib::substring;
use crate::subst::skip_to_delim;
use crate::variables::get_string_value;
use std::convert::TryInto;

extern "C" {
    fn strvec_sort(_: *mut *mut libc::c_char, _: libc::c_int);
    static mut noglob_dot_filenames: libc::c_int;
    fn glob_filename(_: *mut libc::c_char, _: libc::c_int) -> *mut *mut libc::c_char;
}

#[inline]
fn is_basic(c: libc::c_char) -> libc::c_int {
    unsafe {
        return (*is_basic_table
            .as_ptr()
            .offset((c as libc::c_uchar as libc::c_int >> 5 as libc::c_int) as isize)
            >> (c as libc::c_uchar as libc::c_int & 31 as libc::c_int)
            & 1 as libc::c_int as libc::c_uint) as libc::c_int;
    }
}

#[no_mangle]
pub fn unquoted_glob_pattern_p(mut string: *mut libc::c_char) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut send: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut open: libc::c_int = 0;
    let mut bsquote: libc::c_int = 0;
    let mut state: mbstate_t = mbstate_t {
        __count: 0,
        __value: mbstate_t_value { __wch: 0 },
    };
    unsafe {
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            (::std::mem::size_of::<mbstate_t>() as libc::c_ulong)
                .try_into()
                .unwrap(),
        );
        bsquote = 0 as libc::c_int;
        open = bsquote;
        send = string.offset(libc::strlen(string) as isize);
        loop {
            let fresh0 = string;
            string = string.offset(1);
            c = *fresh0 as libc::c_int;
            if !(c != 0) {
                break;
            }
            match c {
                // '?' | '*'
                63 | 42 => return 1 as libc::c_int,
                // '['
                91 => {
                    open += 1;
                    continue;
                }
                // ']'
                93 => {
                    if open != 0 {
                        return 1 as libc::c_int;
                    }
                    continue;
                }
                // '/'
                47 => {
                    if open != 0 {
                        open = 0 as libc::c_int;
                    }
                    if *string as libc::c_int == '(' as i32 {
                        return 1 as libc::c_int;
                    }
                    continue;
                }
                // '+' | '@' | '!'
                43 | 64 | 33 => {
                    if *string as libc::c_int == '(' as i32 {
                        return 1 as libc::c_int;
                    }
                    continue;
                }
                // '\\'
                92 => {
                    if *string as libc::c_int != '\u{0}' as i32
                        && *string as libc::c_int != '/' as i32
                    {
                        bsquote = 1 as libc::c_int;
                        string = string.offset(1);
                        continue;
                    } else if open != 0 && *string as libc::c_int == '/' as i32 {
                        string = string.offset(1);
                        continue;
                    } else if *string as libc::c_int == 0 as libc::c_int {
                        return 0 as libc::c_int;
                    }
                    let fresh1 = string;
                    string = string.offset(1);
                    if *fresh1 as libc::c_int == '\u{0}' as i32 {
                        return 0 as libc::c_int;
                    }
                }
                1 => {
                    let fresh1 = string;
                    string = string.offset(1);
                    if *fresh1 as libc::c_int == '\u{0}' as i32 {
                        return 0 as libc::c_int;
                    }
                }
                _ => {}
            }

            string = string.offset(-1);
            if locale_mb_cur_max > 1 as libc::c_int {
                let mut state_bak: mbstate_t = mbstate_t {
                    __count: 0,
                    __value: mbstate_t_value { __wch: 0 },
                };
                let mut mblength: size_t = 0;
                let mut _f: libc::c_int = 0;
                _f = is_basic(*string);
                if _f != 0 {
                    mblength = 1 as libc::c_int as size_t;
                } else if locale_utf8locale != 0
                    && *string as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                {
                    mblength =
                        (*string as libc::c_int != 0 as libc::c_int) as libc::c_int as size_t;
                } else {
                    state_bak = state;
                    mblength = mbrlen(
                        string,
                        send.offset_from(string) as libc::c_long as size_t,
                        &mut state,
                    );
                }
                if mblength == -(2 as libc::c_int) as size_t
                    || mblength == -(1 as libc::c_int) as size_t
                {
                    state = state_bak;
                    mblength = 1 as libc::c_int as size_t;
                } else {
                    string = string.offset(
                        (if mblength < 1 as libc::c_int as libc::c_ulong {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            mblength.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        }) as isize,
                    );
                }
            }
            string = string.offset(1);
        }
    }
    return 0 as libc::c_int;
}
#[inline]
fn ere_char(c: libc::c_int) -> libc::c_int {
    match c as u8 as char {
        '.' | '[' | '\\' | '(' | ')' | '*' | '+' | '?' | '{' | '|' | '^' | '$' => {
            return 1 as libc::c_int;
        }
        _ => return 0 as libc::c_int,
    };
}
#[no_mangle]
pub fn glob_char_p(s: *const libc::c_char) -> libc::c_int {
    unsafe {
        match *s as libc::c_int as u8 as char {
            '*' | '[' | ']' | '?' | '\\' => return 1 as libc::c_int,
            '+' | '@' | '!' => {
                if *s.offset(1 as libc::c_int as isize) as libc::c_int == '(' as i32 {
                    return 1 as libc::c_int;
                }
            }
            _ => {}
        }
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub fn quote_string_for_globbing(
    pathname: *const libc::c_char,
    qflags: libc::c_int,
) -> *mut libc::c_char {
    unsafe {
        let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut cclass: libc::c_int = 0;
        let mut collsym: libc::c_int = 0;
        let mut equiv: libc::c_int = 0;
        let mut c: libc::c_int = 0;
        let mut last_was_backslash: libc::c_int = 0;
        let mut savei: libc::c_int = 0;
        let mut savej: libc::c_int = 0;

        temp = libc::malloc(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(libc::strlen(pathname) as u64)
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
        ) as *mut libc::c_char;

        if qflags & QGLOB_CVTNULL as libc::c_int != 0
            && (*pathname.offset(0 as libc::c_int as isize) as libc::c_int == '\u{7f}' as i32
                && *pathname.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32)
        {
            *temp.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
            return temp;
        }

        last_was_backslash = 0 as libc::c_int;
        equiv = last_was_backslash;
        collsym = equiv;
        cclass = collsym;
        j = 0 as libc::c_int;
        i = j;

        while *pathname.offset(i as isize) != 0 {
            if *pathname.offset(i as isize) as libc::c_int == CTLESC as i32
                && *pathname.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                    == '\u{0}' as i32
            {
                let fresh2 = i;
                i = i + 1;
                let fresh3 = j;
                j = j + 1;
                *temp.offset(fresh3 as isize) = *pathname.offset(fresh2 as isize);
                break;
            } else if qflags & (QGLOB_REGEXP as libc::c_int | QGLOB_CTLESC as libc::c_int) != 0
                && *pathname.offset(i as isize) as libc::c_int == CTLESC as i32
                && (*pathname.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                    == CTLESC as i32
                    || *pathname.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                        == CTLNUL as i32)
            {
                i += 1;
                let fresh4 = j;
                j = j + 1;
                *temp.offset(fresh4 as isize) = *pathname.offset(i as isize);
                i += 1;
                continue;
            } else if *pathname.offset(i as isize) as libc::c_int == CTLESC as i32 {
                //convert_to_backslash
                if qflags & QGLOB_FILENAME as libc::c_int != 0
                    && *pathname.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                        == '/' as i32
                {
                    i += 1;
                    continue;
                }
                if *pathname.offset((i + 1 as libc::c_int) as isize) as libc::c_int != CTLESC as i32
                    && qflags & QGLOB_REGEXP as libc::c_int != 0
                    && ere_char(*pathname.offset((i + 1 as libc::c_int) as isize) as libc::c_int)
                        == 0 as libc::c_int
                {
                    i += 1;
                    continue;
                }
                let fresh41 = j;
                j = j + 1;
                *temp.offset(fresh41 as isize) = '\\' as i32 as libc::c_char;
                i += 1;
                if *pathname.offset(i as isize) as libc::c_int == '\u{0}' as i32 {
                    break;
                }
            } else if qflags & QGLOB_REGEXP as libc::c_int != 0
                && (i == 0 as libc::c_int
                    || *pathname.offset((i - 1 as libc::c_int) as isize) as libc::c_int
                        != '\u{1}' as i32)
                && *pathname.offset(i as isize) as libc::c_int == '[' as i32
            {
                let fresh6 = i;
                i = i + 1;
                let fresh7 = j;
                j = j + 1;
                *temp.offset(fresh7 as isize) = *pathname.offset(fresh6 as isize);
                savej = j;
                savei = i;
                let fresh8 = i;
                i = i + 1;
                c = *pathname.offset(fresh8 as isize) as libc::c_int;
                if c == '^' as i32 {
                    let fresh9 = j;
                    j = j + 1;
                    *temp.offset(fresh9 as isize) = c as libc::c_char;
                    let fresh10 = i;
                    i = i + 1;
                    c = *pathname.offset(fresh10 as isize) as libc::c_int;
                }
                if c == ']' as i32 {
                    let fresh11 = j;
                    j = j + 1;
                    *temp.offset(fresh11 as isize) = c as libc::c_char;
                    let fresh12 = i;
                    i = i + 1;
                    c = *pathname.offset(fresh12 as isize) as libc::c_int;
                }

                loop {
                    if c == 0 as libc::c_int {
                        *temp.offset(j as isize) = '\0' as i32 as libc::c_char;
                        return temp;
                    } else if c == '\u{1}' as i32 {
                        if *pathname.offset(i as isize) as libc::c_int == 0 as libc::c_int {
                            *temp.offset(j as isize) = '\0' as i32 as libc::c_char;
                            return temp;
                        }
                        let fresh13 = i;
                        i = i + 1;
                        let fresh14 = j;
                        j = j + 1;
                        *temp.offset(fresh14 as isize) = *pathname.offset(fresh13 as isize);
                    } else if c == '[' as i32
                        && *pathname.offset(i as isize) as libc::c_int == ':' as i32
                    {
                        let fresh15 = j;
                        j = j + 1;
                        *temp.offset(fresh15 as isize) = c as libc::c_char;
                        let fresh16 = i;
                        i = i + 1;
                        let fresh17 = j;
                        j = j + 1;
                        *temp.offset(fresh17 as isize) = *pathname.offset(fresh16 as isize);
                        cclass = 1 as libc::c_int;
                    } else if cclass != 0
                        && c == ':' as i32
                        && *pathname.offset(i as isize) as libc::c_int == ']' as i32
                    {
                        let fresh18 = j;
                        j = j + 1;
                        *temp.offset(fresh18 as isize) = c as libc::c_char;
                        let fresh19 = i;
                        i = i + 1;
                        let fresh20 = j;
                        j = j + 1;
                        *temp.offset(fresh20 as isize) = *pathname.offset(fresh19 as isize);
                        cclass = 0 as libc::c_int;
                    } else if c == '[' as i32
                        && *pathname.offset(i as isize) as libc::c_int == '=' as i32
                    {
                        let fresh21 = j;
                        j = j + 1;
                        *temp.offset(fresh21 as isize) = c as libc::c_char;
                        let fresh22 = i;
                        i = i + 1;
                        let fresh23 = j;
                        j = j + 1;
                        *temp.offset(fresh23 as isize) = *pathname.offset(fresh22 as isize);
                        if *pathname.offset(i as isize) as libc::c_int == ']' as i32 {
                            let fresh24 = i;
                            i = i + 1;
                            let fresh25 = j;
                            j = j + 1;
                            *temp.offset(fresh25 as isize) = *pathname.offset(fresh24 as isize);
                        }
                        equiv = 1 as libc::c_int;
                    } else if equiv != 0
                        && c == '=' as i32
                        && *pathname.offset(i as isize) as libc::c_int == ']' as i32
                    {
                        let fresh26 = j;
                        j = j + 1;
                        *temp.offset(fresh26 as isize) = c as libc::c_char;
                        let fresh27 = i;
                        i = i + 1;
                        let fresh28 = j;
                        j = j + 1;
                        *temp.offset(fresh28 as isize) = *pathname.offset(fresh27 as isize);
                        equiv = 0 as libc::c_int;
                    } else if c == '[' as i32
                        && *pathname.offset(i as isize) as libc::c_int == '.' as i32
                    {
                        let fresh29 = j;
                        j = j + 1;
                        *temp.offset(fresh29 as isize) = c as libc::c_char;
                        let fresh30 = i;
                        i = i + 1;
                        let fresh31 = j;
                        j = j + 1;
                        *temp.offset(fresh31 as isize) = *pathname.offset(fresh30 as isize);
                        if *pathname.offset(i as isize) as libc::c_int == ']' as i32 {
                            let fresh32 = i;
                            i = i + 1;
                            let fresh33 = j;
                            j = j + 1;
                            *temp.offset(fresh33 as isize) = *pathname.offset(fresh32 as isize);
                        }
                        collsym = 1 as libc::c_int;
                    } else if collsym != 0
                        && c == '.' as i32
                        && *pathname.offset(i as isize) as libc::c_int == ']' as i32
                    {
                        let fresh34 = j;
                        j = j + 1;
                        *temp.offset(fresh34 as isize) = c as libc::c_char;
                        let fresh35 = i;
                        i = i + 1;
                        let fresh36 = j;
                        j = j + 1;
                        *temp.offset(fresh36 as isize) = *pathname.offset(fresh35 as isize);
                        collsym = 0 as libc::c_int;
                    } else {
                        let fresh37 = j;
                        j = j + 1;
                        *temp.offset(fresh37 as isize) = c as libc::c_char;
                    }
                    let fresh38 = i;
                    i = i + 1;
                    c = *pathname.offset(fresh38 as isize) as libc::c_int;
                    if !(c != ']' as i32 && c != 0 as libc::c_int) {
                        break;
                    }
                }

                if c == 0 as libc::c_int {
                    i = savei - 1 as libc::c_int;
                    j = savej;
                    i += 1;
                    continue;
                }
                let fresh39 = j;
                j = j + 1;
                *temp.offset(fresh39 as isize) = c as libc::c_char;
                i -= 1;
                i += 1;
                continue;
            } else if *pathname.offset(i as isize) as libc::c_int == '\\' as i32
                && qflags & QGLOB_REGEXP as libc::c_int == 0 as libc::c_int
            {
                let fresh40 = j;
                j = j + 1;
                *temp.offset(fresh40 as isize) = '\\' as i32 as libc::c_char;
                i += 1;
                if *pathname.offset(i as isize) as libc::c_int == '\0' as i32 {
                    break;
                }
                if qflags & QGLOB_CTLESC as libc::c_int != 0
                    && *pathname.offset(i as isize) as libc::c_int == CTLESC as i32
                    && (*pathname.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                        == CTLESC as i32
                        || *pathname.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                            == CTLNUL as i32)
                {
                    i += 1;
                } else if qflags & QGLOB_CTLESC as libc::c_int != 0
                    && *pathname.offset(i as isize) as libc::c_int == CTLESC as i32
                {
                    //goto convert_to_backslash
                    if qflags & QGLOB_FILENAME as libc::c_int != 0
                        && *pathname.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                            == '/' as i32
                    {
                        i += 1;
                        continue;
                    }
                    if *pathname.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                        != CTLESC as i32
                        && qflags & QGLOB_REGEXP as libc::c_int != 0
                        && ere_char(*pathname.offset((i + 1 as libc::c_int) as isize) as libc::c_int)
                            == 0 as libc::c_int
                    {
                        i += 1;
                        continue;
                    }
                    let fresh41 = j;
                    j = j + 1;
                    *temp.offset(fresh41 as isize) = '\\' as i32 as libc::c_char;
                    i += 1;
                    if *pathname.offset(i as isize) as libc::c_int == '\u{0}' as i32 {
                        break;
                    }
                } else if *pathname.offset(i as isize) as libc::c_int == '\\' as i32
                    && qflags & QGLOB_REGEXP as libc::c_int != 0
                {
                    last_was_backslash = 1 as libc::c_int;
                }
            }
            let fresh41 = j;
            j = j + 1;
            *temp.offset(fresh41 as isize) = *pathname.offset(i as isize);
            i += 1;
        }

        *temp.offset(j as isize) = '\u{0}' as i32 as libc::c_char;
        return temp;
    }
}

#[no_mangle]
pub fn quote_globbing_chars(string: *const libc::c_char) -> *mut libc::c_char {
    unsafe {
        let mut slen: size_t = 0;
        let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut s: *const libc::c_char = 0 as *const libc::c_char;
        let mut send: *const libc::c_char = 0 as *const libc::c_char;
        let mut state: mbstate_t = mbstate_t {
            __count: 0,
            __value: mbstate_t_value { __wch: 0 },
        };
        libc::memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<mbstate_t>() as libc::c_ulong as usize,
        );
        slen = libc::strlen(string) as u64;
        send = string.offset(slen as isize);
        temp = libc::malloc(
            slen.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
        ) as *mut libc::c_char;
        t = temp;
        s = string;
        while *s != 0 {
            if glob_char_p(s) != 0 {
                let fresh42 = t;
                t = t.offset(1);
                *fresh42 = '\\' as i32 as libc::c_char;
            }
            if locale_mb_cur_max > 1 as libc::c_int {
                let mut state_bak: mbstate_t = mbstate_t {
                    __count: 0,
                    __value: mbstate_t_value { __wch: 0 },
                };
                let mut mblength: size_t = 0;
                let mut _k: libc::c_int = 0;
                _k = is_basic(*s);
                if _k != 0 {
                    mblength = 1 as libc::c_int as size_t;
                } else if locale_utf8locale != 0
                    && *s as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                {
                    mblength = (*s as libc::c_int != 0 as libc::c_int) as libc::c_int as size_t;
                } else {
                    state_bak = state;
                    mblength = mbrlen(s, send.offset_from(s) as libc::c_long as size_t, &mut state);
                }
                if mblength == -(2 as libc::c_int) as size_t
                    || mblength == -(1 as libc::c_int) as size_t
                {
                    state = state_bak;
                    mblength = 1 as libc::c_int as size_t;
                } else {
                    mblength = if mblength < 1 as libc::c_int as libc::c_ulong {
                        1 as libc::c_int as libc::c_ulong
                    } else {
                        mblength
                    };
                }
                _k = 0 as libc::c_int;
                while (_k as libc::c_ulong) < mblength {
                    let fresh43 = s;
                    s = s.offset(1);
                    let fresh44 = t;
                    t = t.offset(1);
                    *fresh44 = *fresh43;
                    _k += 1;
                }
            } else {
                let fresh45 = s;
                s = s.offset(1);
                let fresh46 = t;
                t = t.offset(1);
                *fresh46 = *fresh45;
            }
        }
        *t = '\u{0}' as i32 as libc::c_char;
        return temp;
    }
}
#[no_mangle]
pub fn shell_glob_filename(
    pathname: *const libc::c_char,
    qflags: libc::c_int,
) -> *mut *mut libc::c_char {
    unsafe {
        let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut results: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut gflags: libc::c_int = 0;
        let quoted_pattern: libc::c_int = 0;
        noglob_dot_filenames = (glob_dot_filenames == 0 as libc::c_int) as libc::c_int;
        temp = quote_string_for_globbing(pathname, 0x2 as libc::c_int | qflags);
        gflags = if glob_star != 0 {
            0x400 as libc::c_int
        } else {
            0 as libc::c_int
        };
        results = glob_filename(temp, gflags);
        libc::free(temp as *mut libc::c_void);
        if !results.is_null()
            && (results == &mut glob_error_return as *mut *mut libc::c_char) as libc::c_int
                == 0 as libc::c_int
        {
            if should_ignore_glob_matches() != 0 {
                ignore_glob_matches(results);
            }
            if !results.is_null() && !(*results.offset(0 as libc::c_int as isize)).is_null() {
                strvec_sort(results, 1 as libc::c_int);
            } else {
                if !results.is_null() {
                    libc::free(results as *mut libc::c_void);
                }
                results = &mut glob_error_return as *mut *mut libc::c_char;
            }
        }
        return results;
    }
}
static mut globignore: ignorevar = {
    let init = ignorevar {
        varname: b"GLOBIGNORE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ignores: 0 as *const ign as *mut ign,
        num_ignores: 0 as libc::c_int,
        last_ignoreval: 0 as *const libc::c_char as *mut libc::c_char,
        item_func: None,
    };
    init
};
#[no_mangle]
pub fn setup_glob_ignore(name: *mut libc::c_char) {
    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        v = get_string_value(name);
        setup_ignore_patterns(&mut globignore);
        if globignore.num_ignores != 0 {
            glob_dot_filenames = 1 as libc::c_int;
        } else if v.is_null() {
            glob_dot_filenames = 0 as libc::c_int;
        }
    }
}
#[no_mangle]
pub fn should_ignore_glob_matches() -> libc::c_int {
    unsafe {
        return globignore.num_ignores;
    }
}
fn glob_name_is_acceptable(name: *const libc::c_char) -> libc::c_int {
    let mut p: *mut ign = 0 as *mut ign;
    let mut n: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut flags: libc::c_int = 0;
    unsafe {
        n = libc::strrchr(name, '/' as i32);
        if n.is_null() || *n.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
            n = name as *mut libc::c_char;
        } else {
            n = n.offset(1);
        }
        if *n.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
            && (*n.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
                || *n.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
                    && *n.offset(2 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32)
        {
            return 0 as libc::c_int;
        }
        flags = (1 as libc::c_int) << 0 as libc::c_int
            | (if extended_glob != 0 {
                (1 as libc::c_int) << 5 as libc::c_int
            } else {
                0 as libc::c_int
            })
            | (if glob_ignore_case != 0 {
                (1 as libc::c_int) << 4 as libc::c_int
            } else {
                0 as libc::c_int
            });
        p = globignore.ignores;
        while !((*p).val).is_null() {
            if strmatch((*p).val, name as *mut libc::c_char, flags) != 1 as libc::c_int {
                return 0 as libc::c_int;
            }
            p = p.offset(1);
        }
    }
    return 1 as libc::c_int;
}
fn ignore_globbed_names(names: *mut *mut libc::c_char, name_func: Option<sh_ignore_func_t>) {
    unsafe {
        let mut newnames: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut n: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while !(*names.offset(i as isize)).is_null() {
            i += 1;
        }

        newnames = strvec_create(i + 1 as libc::c_int);
        i = 0 as libc::c_int;
        n = i;
        while !(*names.offset(i as isize)).is_null() {
            if (Some(name_func.expect("non-null function pointer")))
                .expect("non-null function pointer")(*names.offset(i as isize))
                != 0
            {
                let fresh47 = n;
                n = n + 1;
                let ref mut fresh48 = *newnames.offset(fresh47 as isize);
                *fresh48 = *names.offset(i as isize);
            } else {
                libc::free(*names.offset(i as isize) as *mut libc::c_void);
            }
            i += 1;
        }
        let ref mut fresh49 = *newnames.offset(n as isize);
        *fresh49 = 0 as *mut libc::c_void as *mut libc::c_char;
        if n == 0 as libc::c_int {
            let ref mut fresh50 = *names.offset(0 as libc::c_int as isize);
            *fresh50 = 0 as *mut libc::c_void as *mut libc::c_char;
            libc::free(newnames as *mut libc::c_void);
            return;
        }
        n = 0 as libc::c_int;
        while !(*newnames.offset(n as isize)).is_null() {
            let ref mut fresh51 = *names.offset(n as isize);
            *fresh51 = *newnames.offset(n as isize);
            n += 1;
        }
        let ref mut fresh52 = *names.offset(n as isize);
        *fresh52 = 0 as *mut libc::c_void as *mut libc::c_char;
        libc::free(newnames as *mut libc::c_void);
    }
}
#[no_mangle]
pub fn ignore_glob_matches(names: *mut *mut libc::c_char) {
    unsafe {
        if globignore.num_ignores == 0 as libc::c_int {
            return;
        }
        ignore_globbed_names(
            names,
            ::std::mem::transmute::<Option<fn() -> libc::c_int>, Option<sh_ignore_func_t>>(Some(
                ::std::mem::transmute::<fn(*const libc::c_char) -> libc::c_int, fn() -> libc::c_int>(
                    glob_name_is_acceptable,
                ),
            )),
        );
    }
}
fn split_ignorespec(s: *mut libc::c_char, ip: *mut libc::c_int) -> *mut libc::c_char {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if s.is_null() {
        return 0 as *mut libc::c_char;
    }
    unsafe {
        i = *ip;
        if *s.offset(i as isize) as libc::c_int == 0 as libc::c_int {
            return 0 as *mut libc::c_char;
        }
        n = skip_to_delim(
            s,
            i,
            b":\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            SD_NOJMP as libc::c_int | SD_EXTGLOB as libc::c_int | SD_GLOB as libc::c_int,
        );
        t = substring(s, i, n);
        if *s.offset(n as isize) as libc::c_int == ':' as i32 {
            n += 1;
        }
        *ip = n;
    }
    return t;
}
#[no_mangle]
pub fn setup_ignore_patterns(ivp: *mut ignorevar) {
    let mut numitems: libc::c_int = 0;
    let mut maxitems: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut colon_bit: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut this_ignoreval: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut ign = 0 as *mut ign;
    unsafe {
        this_ignoreval = get_string_value((*ivp).varname);
        if !this_ignoreval.is_null()
            && !((*ivp).last_ignoreval).is_null()
            && (*this_ignoreval.offset(0 as libc::c_int as isize) as libc::c_int
                == *((*ivp).last_ignoreval).offset(0 as libc::c_int as isize) as libc::c_int
                && libc::strcmp(this_ignoreval, (*ivp).last_ignoreval) == 0 as libc::c_int)
            || this_ignoreval.is_null() && ((*ivp).last_ignoreval).is_null()
        {
            return;
        }
        (*ivp).num_ignores = 0 as libc::c_int;
        if !((*ivp).ignores).is_null() {
            p = (*ivp).ignores;
            while !((*p).val).is_null() {
                libc::free((*p).val as *mut libc::c_void);
                p = p.offset(1);
            }
            libc::free((*ivp).ignores as *mut libc::c_void);
            let ref mut fresh53 = (*ivp).ignores;
            *fresh53 = 0 as *mut libc::c_void as *mut ign;
        }
        if !((*ivp).last_ignoreval).is_null() {
            libc::free((*ivp).last_ignoreval as *mut libc::c_void);
            let ref mut fresh54 = (*ivp).last_ignoreval;
            *fresh54 = 0 as *mut libc::c_void as *mut libc::c_char;
        }
        if this_ignoreval.is_null() || *this_ignoreval as libc::c_int == '\u{0}' as i32 {
            return;
        }
        let ref mut fresh55 = (*ivp).last_ignoreval;
        *fresh55 = libc::strcpy(
            libc::malloc(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(libc::strlen(this_ignoreval) as u64) as usize,
            ) as *mut libc::c_char,
            this_ignoreval,
        );
        ptr = 0 as libc::c_int;
        maxitems = ptr;
        numitems = maxitems;
        loop {
            colon_bit = split_ignorespec(this_ignoreval, &mut ptr);
            if colon_bit.is_null() {
                break;
            }
            if numitems + 1 as libc::c_int >= maxitems {
                maxitems += 10 as libc::c_int;
                let ref mut fresh56 = (*ivp).ignores;
                *fresh56 = libc::realloc(
                    (*ivp).ignores as *mut libc::c_void,
                    (maxitems as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<ign>() as libc::c_ulong)
                        as usize,
                ) as *mut ign;
            }
            let ref mut fresh57 = (*((*ivp).ignores).offset(numitems as isize)).val;
            *fresh57 = colon_bit;
            (*((*ivp).ignores).offset(numitems as isize)).len =
                libc::strlen(colon_bit) as libc::c_int;
            (*((*ivp).ignores).offset(numitems as isize)).flags = 0 as libc::c_int;
            if ((*ivp).item_func).is_some() {
                (Some(((*ivp).item_func).expect("non-null function pointer")))
                    .expect("non-null function pointer")(
                    &mut *((*ivp).ignores).offset(numitems as isize),
                );
            }
            numitems += 1;
        }
        let ref mut fresh58 = (*((*ivp).ignores).offset(numitems as isize)).val;
        *fresh58 = 0 as *mut libc::c_void as *mut libc::c_char;
        (*ivp).num_ignores = numitems;
    }
}
