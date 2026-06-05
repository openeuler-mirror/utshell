use crate::general::legal_number;
use crate::src_common::*;

use crate::builtins::common::{sh_invalidopt, sh_needarg, sh_neednumarg};

static mut sp: libc::c_int = 0;
static mut lhead: *mut WordList = 0 as *const libc::c_void as *mut libc::c_void as *mut WordList;

#[no_mangle]
pub fn internal_getopt(list: *mut WordList, mut opts: *mut libc::c_char) -> libc::c_int {
    // let c: libc::c_int ;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    // let  plus: libc::c_int ;
    static mut errstr: [libc::c_char; 3] = [
        '-' as i32 as libc::c_char,
        '\u{0}' as i32 as libc::c_char,
        '\u{0}' as i32 as libc::c_char,
    ];
    unsafe {
        let plus = (*opts as libc::c_int == '+' as i32) as libc::c_int;
        if plus != 0 {
            opts = opts.offset(1);
        }
        if list.is_null() {
            list_optarg = 0 as *mut libc::c_void as *mut libc::c_char;
            loptend = 0 as *mut libc::c_void as *mut WordList;
            return -(1 as libc::c_int);
        }
        if list != lhead || lhead.is_null() {
            sp = 1 as libc::c_int;
            lhead = list;
            lcurrent = lhead;
            loptend = 0 as *mut libc::c_void as *mut WordList;
        }
        if sp == 1 as libc::c_int {
            if lcurrent.is_null()
                || (*(*(*lcurrent).word).word as libc::c_int != '-' as i32
                    && (plus == 0 || *(*(*lcurrent).word).word as libc::c_int != '+' as i32)
                    || *((*(*lcurrent).word).word).offset(1 as libc::c_int as isize) as libc::c_int
                        == '\u{0}' as i32)
            {
                lhead = 0 as *mut libc::c_void as *mut WordList;
                loptend = lcurrent;
                return -(1 as libc::c_int);
            } else {
                if *((*(*lcurrent).word).word).offset(0 as libc::c_int as isize) as libc::c_int
                    == (*::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"--help\0"))
                        [0 as libc::c_int as usize] as libc::c_int
                    && strcmp(
                        (*(*lcurrent).word).word,
                        b"--help\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                {
                    lhead = 0 as *mut libc::c_void as *mut WordList;
                    loptend = lcurrent;
                    return -(99 as libc::c_int);
                } else {
                    if *((*(*lcurrent).word).word).offset(0 as libc::c_int as isize) as libc::c_int
                        == '-' as i32
                        && *((*(*lcurrent).word).word).offset(1 as libc::c_int as isize)
                            as libc::c_int
                            == '-' as i32
                        && *((*(*lcurrent).word).word).offset(2 as libc::c_int as isize)
                            as libc::c_int
                            == 0 as libc::c_int
                    {
                        lhead = 0 as *mut libc::c_void as *mut WordList;
                        loptend = (*lcurrent).next;
                        return -(1 as libc::c_int);
                    }
                }
            }
            list_opttype =
                *((*(*lcurrent).word).word).offset(0 as libc::c_int as isize) as libc::c_int;
            errstr[0 as libc::c_int as usize] = list_opttype as libc::c_char;
        }
        let c = *((*(*lcurrent).word).word).offset(sp as isize) as libc::c_int;
        list_optopt = c;
        if c == ':' as i32 || {
            cp = strchr(opts, c);
            cp.is_null()
        } {
            errstr[1 as libc::c_int as usize] = c as libc::c_char;
            sh_invalidopt(errstr.as_mut_ptr());
            sp += 1;
            if *((*(*lcurrent).word).word).offset(sp as isize) as libc::c_int == '\u{0}' as i32 {
                lcurrent = (*lcurrent).next;
                sp = 1 as libc::c_int;
            }
            list_optarg = 0 as *mut libc::c_char;
            if !lcurrent.is_null() {
                loptend = (*lcurrent).next;
            }
            return '?' as i32;
        }
        cp = cp.offset(1);
        if *cp as libc::c_int == ':' as i32 || *cp as libc::c_int == ';' as i32 {
            if *((*(*lcurrent).word).word).offset((sp + 1 as libc::c_int) as isize) != 0 {
                list_optarg = ((*(*lcurrent).word).word)
                    .offset(sp as isize)
                    .offset(1 as libc::c_int as isize);
                lcurrent = (*lcurrent).next;
            } else if !((*lcurrent).next).is_null()
                && (*cp as libc::c_int == ':' as i32
                    || (*(*(*(*lcurrent).next).word).word as libc::c_int != '-' as i32
                        && (plus == 0
                            || *(*(*(*lcurrent).next).word).word as libc::c_int != '+' as i32)
                        || *((*(*(*lcurrent).next).word).word).offset(1 as libc::c_int as isize)
                            as libc::c_int
                            == '\u{0}' as i32))
            {
                lcurrent = (*lcurrent).next;
                list_optarg = (*(*lcurrent).word).word;
                lcurrent = (*lcurrent).next;
            } else if *cp as libc::c_int == ';' as i32 {
                list_optarg = 0 as *mut libc::c_void as *mut libc::c_char;
                lcurrent = (*lcurrent).next;
            } else {
                errstr[1 as libc::c_int as usize] = c as libc::c_char;
                sh_needarg(errstr.as_mut_ptr());
                sp = 1 as libc::c_int;
                list_optarg = 0 as *mut libc::c_void as *mut libc::c_char;
                return '?' as i32;
            }
            sp = 1 as libc::c_int;
        } else if *cp as libc::c_int == '#' as i32 {
            if *((*(*lcurrent).word).word).offset((sp + 1 as libc::c_int) as isize) != 0 {
                if *((*(*lcurrent).word).word).offset((sp + 1 as libc::c_int) as isize)
                    as libc::c_int
                    >= '0' as i32
                    && *((*(*lcurrent).word).word).offset((sp + 1 as libc::c_int) as isize)
                        as libc::c_int
                        <= '9' as i32
                {
                    list_optarg = ((*(*lcurrent).word).word)
                        .offset(sp as isize)
                        .offset(1 as libc::c_int as isize);
                    lcurrent = (*lcurrent).next;
                } else {
                    list_optarg = 0 as *mut libc::c_void as *mut libc::c_char;
                }
            } else if !((*lcurrent).next).is_null()
                && legal_number((*(*(*lcurrent).next).word).word, 0 as *mut intmax_t) != 0
            {
                lcurrent = (*lcurrent).next;
                list_optarg = (*(*lcurrent).word).word;
                lcurrent = (*lcurrent).next;
            } else {
                errstr[1 as libc::c_int as usize] = c as libc::c_char;
                sh_neednumarg(errstr.as_mut_ptr());
                sp = 1 as libc::c_int;
                list_optarg = 0 as *mut libc::c_void as *mut libc::c_char;
                return '?' as i32;
            }
        } else {
            sp += 1;
            if *((*(*lcurrent).word).word).offset(sp as isize) as libc::c_int == '\u{0}' as i32 {
                sp = 1 as libc::c_int;
                lcurrent = (*lcurrent).next;
            }
            list_optarg = 0 as *mut libc::c_void as *mut libc::c_char;
        }
        return c;
    }
}
#[no_mangle]
pub fn reset_internal_getopt() {
    unsafe {
        loptend = 0 as *mut libc::c_void as *mut WordList;
        lcurrent = loptend;
        lhead = lcurrent;
        sp = 1 as libc::c_int;
    }
}
