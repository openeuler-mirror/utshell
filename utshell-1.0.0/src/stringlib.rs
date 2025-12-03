//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later
use crate::pathexp::quote_globbing_chars;
use crate::src_common::*;

extern "C" {
    fn strmatch(_: *mut libc::c_char, _: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn glob_pattern_p(_: *const libc::c_char) -> libc::c_int;
}

#[no_mangle]
pub fn find_string_in_alist(
    string: *mut libc::c_char,
    alist: *mut STRING_INT_ALIST,
    flags: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;

    r = 0;
    i = r;
    unsafe {
        while !((*alist.offset(i as isize)).word).is_null() {
            if flags != 0 {
                r = (strmatch((*alist.offset(i as isize)).word, string, FNM_EXTMATCH!())
                    != FNM_NOMATCH!()) as libc::c_int;
            } else {
                r = STREQ!(string, (*alist.offset(i as isize)).word) as libc::c_int;
            }
            if r != 0 {
                return (*alist.offset(i as isize)).token;
            }
            i += 1;
        }
    }
    return -(1 as libc::c_int);
}

#[no_mangle]
pub fn find_token_in_alist(
    token: libc::c_int,
    alist: *mut STRING_INT_ALIST,
    flags: libc::c_int,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    unsafe {
        i = 0;
        while !((*alist.offset(i as isize)).word).is_null() {
            if (*alist.offset(i as isize)).token == token {
                return savestring!((*alist.offset(i as isize)).word);
            }
            i += 1;
        }
    }
    return 0 as *mut libc::c_char;
}

#[no_mangle]
pub fn find_index_in_alist(
    string: *mut libc::c_char,
    alist: *mut STRING_INT_ALIST,
    flags: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    unsafe {
        r = 0;
        i = r;
        while !((*alist.offset(i as isize)).word).is_null() {
            if flags != 0 {
                r = (strmatch((*alist.offset(i as isize)).word, string, FNM_EXTMATCH!())
                    != FNM_NOMATCH!()) as libc::c_int;
            } else {
                r = STREQ!(string, (*alist.offset(i as isize)).word) as libc::c_int;
            }
            if r != 0 {
                return i;
            }
            i += 1;
        }
    }
    return -(1 as libc::c_int);
}

#[no_mangle]
pub fn substring(
    string: *const libc::c_char,
    start: libc::c_int,
    end: libc::c_int,
) -> *mut libc::c_char {
    unsafe {
        let mut len: libc::c_int = 0;
        let mut result: *mut libc::c_char = 0 as *mut libc::c_char;

        len = end - start;
        result = malloc((len + 1 as libc::c_int) as usize) as *mut libc::c_char;
        memcpy(
            result as *mut c_void,
            string.offset(start as isize) as *const c_void,
            len as usize,
        );
        *result.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
        return result;
    }
}

#[no_mangle]
pub fn strsub(
    string: *mut libc::c_char,
    pat: *mut libc::c_char,
    rep: *mut libc::c_char,
    global: libc::c_int,
) -> *mut libc::c_char {
    unsafe {
        let mut patlen: libc::c_int = 0;
        let mut replen: libc::c_int = 0;
        let mut templen: libc::c_int = 0;
        let mut tempsize: libc::c_int = 0;
        let mut repl: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut r: *mut libc::c_char = 0 as *mut libc::c_char;

        patlen = strlen(pat) as libc::c_int;
        replen = strlen(rep) as libc::c_int;
        temp = 0 as *mut libc::c_char;
        tempsize = 0;
        templen = tempsize;
        i = templen;
        repl = 1;
        while *string.offset(i as isize) != 0 {
            if repl != 0 && STREQN!(string.offset(i as isize), pat, patlen) != 0 {
                if replen != 0 {
                    RESIZE_MALLOCED_BUFFER!(temp, templen, replen, tempsize, replen * 2);
                }

                r = rep;
                while *r != 0 {
                    *temp.offset(templen as isize) = *r;
                    templen = templen + 1;
                    r = r.offset(1);
                }

                i += if patlen != 0 { patlen } else { 1 };
                repl = (global != 0) as libc::c_int;
            } else {
                RESIZE_MALLOCED_BUFFER!(temp, templen, 1, tempsize, 16);

                *temp.offset(templen as isize) = *string.offset(i as isize);
                templen = templen + 1;
                i = i + 1;
            }
        }

        if !temp.is_null() {
            *temp.offset(templen as isize) = 0 as libc::c_char;
        } else {
            temp = savestring!(string);
        }
        return temp;
    }
}

#[no_mangle]
pub fn strcreplace(
    string: *mut libc::c_char,
    c: libc::c_int,
    text: *const libc::c_char,
    do_glob: libc::c_int,
) -> *mut libc::c_char {
    unsafe {
        let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut len: libc::c_int = 0;
        let mut rlen: libc::c_int = 0;
        let mut ind: libc::c_int = 0;
        let mut tlen: libc::c_int = 0;

        len = STRLEN!(text);
        rlen = len + strlen(string) as libc::c_int + 2;
        ret = malloc(rlen as usize) as *mut libc::c_char;

        p = string;
        r = ret;
        while !p.is_null() && *p as libc::c_int != 0 {
            if *p as libc::c_int == c {
                if len != 0 {
                    ind = r.offset_from(ret) as libc::c_long as libc::c_int;
                    if do_glob != 0
                        && (glob_pattern_p(text) != 0 || !(strchr(text, '\\' as i32)).is_null())
                    {
                        t = quote_globbing_chars(text);
                        tlen = strlen(t) as libc::c_int;
                        RESIZE_MALLOCED_BUFFER!(ret, ind, tlen, rlen, rlen);
                        r = ret.offset(ind as isize);
                        strcpy(r, t);
                        r = r.offset(tlen as isize);
                        free(t as *mut c_void);
                    } else {
                        RESIZE_MALLOCED_BUFFER!(ret, ind, len, rlen, rlen);
                        r = ret.offset(ind as isize);
                        strcpy(r, text);
                        r = r.offset(len as isize);
                    }
                }
                p = p.offset(1);
            } else {
                if *p as libc::c_int == '\\' as i32
                    && *p.offset(1 as libc::c_int as isize) as libc::c_int == c
                {
                    p = p.offset(1);
                }
                ind = r.offset_from(ret) as libc::c_long as libc::c_int;
                RESIZE_MALLOCED_BUFFER!(ret, ind, 2, rlen, rlen);
                r = ret.offset(ind as isize);

                *r = *p;
                r = r.offset(1);
                p = p.offset(1);
            }
        }
        *r = '\u{0}' as i32 as libc::c_char;
        return ret;
    }
}

#[no_mangle]
pub fn strip_trailing(string: *mut libc::c_char, mut len: libc::c_int, newlines_only: libc::c_int) {
    unsafe {
        while len >= 0 {
            if newlines_only != 0 && *string.offset(len as isize) as libc::c_int == '\n' as i32
                || newlines_only == 0 && whitespace!(*string.offset(len as isize))
            {
                len -= 1;
            } else {
                break;
            }
        }
        *string.offset((len + 1 as libc::c_int) as isize) = '\u{0}' as i32 as libc::c_char;
    }
}

#[no_mangle]
pub fn xbcopy(s: *mut libc::c_char, d: *mut libc::c_char, n: libc::c_int) {
    unsafe {
        FASTCOPY!(s, d, n);
    }
}
