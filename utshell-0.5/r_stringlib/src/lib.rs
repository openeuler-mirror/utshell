//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

use libc::{c_int,c_char,c_void};
use r_bash::*;

extern "C" {
    fn strmatch(
        _: *mut libc::c_char,
        _: *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int;
    fn glob_pattern_p(_: *const libc::c_char) -> libc::c_int;
}

#[macro_export]
macro_rules! FNM_NOMATCH {
    () => {
        1
    };
}

#[macro_export]
macro_rules! FNM_EXTMATCH {
    () => {
        1 << 5
    };
}

#[macro_export]
macro_rules! STREQ {
    ($a:expr, $b:expr) => {
        *$a.offset(0 as isize) == *$b.offset(0 as isize) 
        && strcmp($a, $b) == 0 
    };
}

#[no_mangle]
pub unsafe extern "C" fn find_string_in_alist(
    mut string: *mut c_char,
    mut alist: *mut STRING_INT_ALIST,
    mut flags: c_int,
) -> c_int {
    let mut i: c_int = 0;
    let mut r: c_int = 0;
    
    r = 0 ;
    i = r;
    while !((*alist.offset(i as isize)).word).is_null() {
        if flags != 0 {
            r = (strmatch(
                (*alist.offset(i as isize)).word,
                string,
                FNM_EXTMATCH!(),
            ) != FNM_NOMATCH!()) as c_int;
        } else {
            r = STREQ!(string, (*alist.offset(i as isize)).word) as c_int;
        }
        if r != 0 {
            return (*alist.offset(i as isize)).token;
        }
        i += 1;
    }
    return -(1 as c_int);
}

#[macro_export]
macro_rules! savestring {
    ($x:expr) => {
        strcpy(malloc((1 + strlen($x)) as usize) as *mut c_char, $x,)
    };
}

#[no_mangle]
pub unsafe extern "C" fn find_token_in_alist(
    mut token: c_int,
    mut alist: *mut STRING_INT_ALIST,
    mut flags: c_int,
) -> *mut c_char {
    let mut i: c_int = 0;
    
    i = 0 ;
    while !((*alist.offset(i as isize)).word).is_null() {
        if (*alist.offset(i as isize)).token == token {
            return savestring!((*alist.offset(i as isize)).word);
        }
        i += 1;
    }
    return 0 as *mut c_char;
}

#[no_mangle]
pub unsafe extern "C" fn find_index_in_alist(
    mut string: *mut c_char,
    mut alist: *mut STRING_INT_ALIST,
    mut flags: c_int,
) -> c_int {
    let mut i: c_int = 0;
    let mut r: c_int = 0;
    
    r = 0 ;
    i = r;
    while !((*alist.offset(i as isize)).word).is_null() {
        if flags != 0 {
            r = (strmatch(
                (*alist.offset(i as isize)).word,
                string,
                FNM_EXTMATCH!(),
            ) != FNM_NOMATCH!()) as c_int;
        } else {
            r = STREQ!(string, (*alist.offset(i as isize)).word)  as c_int;
        }
        if r != 0 {
            return i;
        }
        i += 1;
    }
    return -(1 as c_int);
}

#[no_mangle]
pub unsafe extern "C" fn substring(
    mut string: *const c_char,
    mut start: c_int,
    mut end: c_int,
) -> *mut c_char {
    let mut len: c_int = 0;
    let mut result: *mut c_char = 0 as *mut c_char;
    
    len = end - start;
    result = malloc(
        (len + 1 as c_int) as usize
    ) as *mut c_char;
    memcpy(
        result as *mut c_void,
        string.offset(start as isize) as *const c_void,
        len as usize,
    );
    *result.offset(len as isize) = '\u{0}' as i32 as c_char;
    return result;
}

#[macro_export]
macro_rules! STREQN {
    ($a:expr,$b:expr,$n:expr) => {
        if $n == 0 {
            1  
        } else {
            (*$a.offset(0 as isize) == *$b.offset(0 as isize)
                && strncmp($a, $b, $n as usize) == 0 ) as i32
        }
    };
}

#[macro_export]
macro_rules! RESIZE_MALLOCED_BUFFER {
    ($str:expr, $cind:expr, $room:expr, $csize:expr, $sincr:expr) => {
        if $cind + $room >= $csize {
            while $cind + $room >= $csize {
                $csize += $sincr as c_int;
            }
            $str = realloc(
                $str as *mut c_void,
                $csize as usize
            ) as *mut c_char;
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn strsub(
    mut string: *mut c_char,
    mut pat: *mut c_char,
    mut rep: *mut c_char,
    mut global: c_int,
) -> *mut c_char {
    let mut patlen: c_int = 0;
    let mut replen: c_int = 0;
    let mut templen: c_int = 0;
    let mut tempsize: c_int = 0;
    let mut repl: c_int = 0;
    let mut i: c_int = 0;
    let mut temp: *mut c_char = 0 as *mut c_char;
    let mut r: *mut c_char = 0 as *mut c_char;
    
    patlen = strlen(pat) as c_int;
    replen = strlen(rep) as c_int;
    temp = 0 as *mut c_char;
    tempsize = 0 ;
    templen = tempsize;
    i = templen;
    repl = 1 ;
    while *string.offset(i as isize) != 0 {
        if repl != 0
            && STREQN!(string.offset(i as isize),pat, patlen) != 0
        {
            if replen != 0 {
                RESIZE_MALLOCED_BUFFER!(temp, templen, replen, tempsize, (replen * 2));
            }
           
            r = rep;
            while *r != 0 {
                *temp.offset(templen as isize) = *r;
                templen = templen + 1;
                r = r.offset(1);
            }
            
            i += if patlen != 0 { patlen } else { 1 };
            repl = (global != 0 ) as c_int;
        } else {
            RESIZE_MALLOCED_BUFFER!(temp, templen, 1, tempsize, 16);
            
            *temp.offset(templen as isize) = *string.offset(i as isize);
            templen = templen + 1;
            i = i + 1;
        }
    }

    if !temp.is_null() {
        *temp.offset(templen as isize) = 0 as c_char;
    } else {
        temp = savestring!(string);
    }
    return temp;
}

#[macro_export]
macro_rules! STRLEN {
    ($s:expr) => {
        (if !$s.is_null() && *$s.offset(0 as c_int as isize) as c_int != 0
        {
            if *$s.offset(1 as c_int as isize) as c_int != 0 {
                if *$s.offset(2 as c_int as isize) as c_int != 0 {
                    strlen($s)
                } else {
                    2 as c_int as libc::c_ulong
                }
            } else {
                1 as c_int as libc::c_ulong
            }
        } else {
            0 as c_int as libc::c_ulong
        }) as c_int;
    };
}

#[no_mangle]
pub unsafe extern "C" fn strcreplace(
    mut string: *mut c_char,
    mut c: c_int,
    mut text: *const c_char,
    mut do_glob: c_int,
) -> *mut c_char {
    let mut ret: *mut c_char = 0 as *mut c_char;
    let mut p: *mut c_char = 0 as *mut c_char;
    let mut r: *mut c_char = 0 as *mut c_char;
    let mut t: *mut c_char = 0 as *mut c_char;
    let mut len: c_int = 0;
    let mut rlen: c_int = 0;
    let mut ind: c_int = 0;
    let mut tlen: c_int = 0;
    
    len = STRLEN!(text);
    rlen = len + strlen(string) as c_int + 2;
    ret = malloc(
        rlen as usize) as *mut c_char;
    
    p = string;
    r = ret;
    while !p.is_null() && *p as c_int != 0 {
        if *p as c_int == c {
            if len != 0 {
                ind = r.offset_from(ret) as libc::c_long as c_int;
                if do_glob != 0
                    && (glob_pattern_p(text) != 0
                        || !(strchr(text, '\\' as i32)).is_null())
                {
                    t = quote_globbing_chars(text);
                    tlen = strlen(t) as c_int;
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
            if *p as c_int == '\\' as i32
                && *p.offset(1 as c_int as isize) as c_int == c
            {
                p = p.offset(1);
            }
            ind = r.offset_from(ret) as libc::c_long as c_int;
            RESIZE_MALLOCED_BUFFER!(ret, ind, 2, rlen, rlen);
            r = ret.offset(ind as isize);

            *r = *p;
            r = r.offset(1);
            p = p.offset(1);
        }
    }
    *r = '\u{0}' as i32 as c_char;
    return ret;
}

#[macro_export]
macro_rules! whitespace {
    ($c:expr) => {
        ($c as c_int == ' ' as i32 || $c as c_int == '\t' as i32)
    };
}

#[no_mangle]
pub unsafe extern "C" fn strip_trailing(
    mut string: *mut c_char,
    mut len: c_int,
    mut newlines_only: c_int,
) {
    while len >= 0 {
        if newlines_only != 0
            && *string.offset(len as isize) as c_int == '\n' as i32
            || newlines_only == 0
                && whitespace!(*string.offset(len as isize))
        {
            len -= 1;
        } else {
            break;
        }
        
    }
    *string.offset((len + 1 as c_int) as isize) = '\u{0}' as i32 as c_char;
}

#[macro_export]
macro_rules! FASTCOPY {
    ($s:expr, $d:expr, $n:expr) => {
        libc::memcpy(
            $d as *mut c_void,
            $s as *const c_void,
            $n as libc::c_ulong as libc::size_t,
        );
    };
}


#[no_mangle]
pub unsafe extern "C" fn xbcopy(
    mut s: *mut c_char,
    mut d: *mut c_char,
    mut n: c_int,
) {
    FASTCOPY!(s, d, n);
}