use crate::dispose_cmd::dispose_words;
use crate::hashlib::{hash_dispose, hash_flush, hash_remove, hash_search};
use crate::make_cmd::{make_bare_word, make_word_list};
use crate::src_common::*;
use crate::subst::{
    dequote_escapes, dequote_string, pat_subst, quote_escapes, quote_string, remove_quoted_nulls,
    string_list_internal, string_list_pos_params,
};

#[no_mangle]
pub fn assoc_dispose(hash: *mut HASH_TABLE) {
    if !hash.is_null() {
        hash_flush(hash, None);
        hash_dispose(hash);
    }
}

#[no_mangle]
pub fn assoc_flush(hash: *mut HASH_TABLE) {
    hash_flush(hash, None);
}

#[no_mangle]
pub fn assoc_insert(
    hash: *mut HASH_TABLE,
    key: *mut libc::c_char,
    value: *mut libc::c_char,
) -> libc::c_int {
    let b: *mut BUCKET_CONTENTS;
    b = hash_search(key, hash, 0x2 as libc::c_int);
    if b.is_null() {
        return -(1 as libc::c_int);
    }
    unsafe {
        if (*b).key != key {
            libc::free(key as *mut libc::c_void);
        }
        if !((*b).data).is_null() {
            libc::free((*b).data);
        }
        (*b).data = 0 as *mut libc::c_void;
        (*b).data = (if !value.is_null() {
            savestring!(value)
        } else {
            0 as *mut libc::c_char
        }) as *mut libc::c_void;
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub fn assoc_remove(hash: *mut HASH_TABLE, string: *mut libc::c_char) {
    let b: *mut BUCKET_CONTENTS;
    b = hash_remove(string, hash, 0 as libc::c_int);
    unsafe {
        if !b.is_null() {
            libc::free((*b).data as *mut libc::c_char as *mut libc::c_void);
            libc::free((*b).key as *mut libc::c_void);
            libc::free(b as *mut libc::c_void);
        }
    }
}

#[no_mangle]
pub fn assoc_reference(hash: *mut HASH_TABLE, string: *mut libc::c_char) -> *mut libc::c_char {
    let b: *mut BUCKET_CONTENTS;
    if hash.is_null() {
        return 0 as *mut libc::c_char;
    }
    b = hash_search(string, hash, 0 as libc::c_int);

    return if !b.is_null() {
        unsafe { (*b).data as *mut libc::c_char }
    } else {
        0 as *mut libc::c_char
    };
}
#[no_mangle]
pub fn assoc_quote(h: *mut HASH_TABLE) -> *mut HASH_TABLE {
    let mut i: libc::c_int;
    let mut tlist: *mut BUCKET_CONTENTS;
    let mut t: *mut libc::c_char;
    unsafe {
        if h.is_null() || assoc_empty!(h) {
            return 0 as *mut libc::c_void as *mut HASH_TABLE;
        }
        i = 0 as libc::c_int;

        while i < (*h).nbuckets {
            tlist = hash_items!(i, h);
            while !tlist.is_null() {
                t = quote_string((*tlist).data as *mut libc::c_char);
                if !((*tlist).data).is_null() {
                    libc::free((*tlist).data);
                }
                (*tlist).data = 0 as *mut libc::c_void;
                (*tlist).data = t as *mut libc::c_void;
                tlist = (*tlist).next;
            }
            i += 1;
        }
    }
    return h;
}

#[no_mangle]
pub fn assoc_quote_escapes(h: *mut HASH_TABLE) -> *mut HASH_TABLE {
    let mut i: libc::c_int;
    let mut tlist: *mut BUCKET_CONTENTS;
    let mut t: *mut libc::c_char;
    unsafe {
        if h.is_null() || assoc_empty!(h) {
            return 0 as *mut libc::c_void as *mut HASH_TABLE;
        }
        i = 0 as libc::c_int;

        while i < (*h).nbuckets {
            tlist = hash_items!(i, h);
            while !tlist.is_null() {
                t = quote_escapes((*tlist).data as *mut libc::c_char);
                if !((*tlist).data).is_null() {
                    libc::free((*tlist).data);
                }
                (*tlist).data = 0 as *mut libc::c_void;
                (*tlist).data = t as *mut libc::c_void;
                tlist = (*tlist).next;
            }
            i += 1;
        }
    }
    return h;
}

#[no_mangle]
pub fn assoc_dequote(h: *mut HASH_TABLE) -> *mut HASH_TABLE {
    let mut i: libc::c_int;
    let mut tlist: *mut BUCKET_CONTENTS;
    let mut t: *mut libc::c_char;
    unsafe {
        if h.is_null() || assoc_empty!(h) {
            return 0 as *mut libc::c_void as *mut HASH_TABLE;
        }
        i = 0 as libc::c_int;

        while i < (*h).nbuckets {
            tlist = hash_items!(i, h);
            while !tlist.is_null() {
                t = dequote_string((*tlist).data as *mut libc::c_char);
                if !((*tlist).data).is_null() {
                    libc::free((*tlist).data);
                }
                (*tlist).data = 0 as *mut libc::c_void;
                (*tlist).data = t as *mut libc::c_void;
                tlist = (*tlist).next;
            }
            i += 1;
        }
    }
    return h;
}

#[no_mangle]
pub fn assoc_dequote_escapes(h: *mut HASH_TABLE) -> *mut HASH_TABLE {
    let mut i: libc::c_int;
    let mut tlist: *mut BUCKET_CONTENTS;
    let mut t: *mut libc::c_char;
    unsafe {
        if h.is_null() || assoc_empty!(h) {
            return 0 as *mut libc::c_void as *mut HASH_TABLE;
        }
        i = 0 as libc::c_int;

        while i < (*h).nbuckets {
            tlist = hash_items!(i, h);
            while !tlist.is_null() {
                t = dequote_escapes((*tlist).data as *mut libc::c_char);
                if !((*tlist).data).is_null() {
                    libc::free((*tlist).data);
                }
                (*tlist).data = 0 as *mut libc::c_void;
                (*tlist).data = t as *mut libc::c_void;
                tlist = (*tlist).next;
            }
            i += 1;
        }
    }
    return h;
}

#[no_mangle]
pub fn assoc_remove_quoted_nulls(h: *mut HASH_TABLE) -> *mut HASH_TABLE {
    let mut i: libc::c_int;
    let mut tlist: *mut BUCKET_CONTENTS;
    let mut t: *mut libc::c_char;
    unsafe {
        if h.is_null() || assoc_empty!(h) {
            return 0 as *mut libc::c_void as *mut HASH_TABLE;
        }
        i = 0 as libc::c_int;

        while i < (*h).nbuckets {
            tlist = hash_items!(i, h);
            while !tlist.is_null() {
                t = remove_quoted_nulls((*tlist).data as *mut libc::c_char);
                (*tlist).data = t as *mut libc::c_void;
                tlist = (*tlist).next;
            }
            i += 1;
        }
    }
    return h;
}

#[no_mangle]
pub fn assoc_subrange(
    hash: *mut HASH_TABLE,
    start: arrayind_t,
    nelem: arrayind_t,
    starsub: libc::c_int,
    quoted: libc::c_int,
    pflags: libc::c_int,
) -> *mut libc::c_char {
    let mut l: *mut WORD_LIST;
    let save: *mut WORD_LIST;
    let h: *mut WORD_LIST;
    let mut t: *mut WORD_LIST;
    let mut i: libc::c_int;
    let mut j: libc::c_int;
    let ret: *mut libc::c_char;
    if unsafe { assoc_empty!(hash) } {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    l = assoc_to_word_list(hash);
    save = l;

    if save.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    i = 1 as libc::c_int;

    while !l.is_null() && (i as libc::c_long) < start {
        l = unsafe { (*l).next };
        i += 1;
    }
    if l.is_null() {
        dispose_words(save);
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    j = 0 as libc::c_int;
    t = l;
    h = t;
    while !l.is_null() && (j as libc::c_long) < nelem {
        t = l;
        l = unsafe { (*l).next };
        j += 1;
    }
    unsafe {
        (*t).next = 0 as *mut libc::c_void as *mut WORD_LIST;
    }
    ret = string_list_pos_params(
        if starsub != 0 { '*' as i32 } else { '@' as i32 },
        h,
        quoted,
        pflags,
    );
    if t != l {
        unsafe {
            (*t).next = l;
        }
    }
    dispose_words(save);
    return ret;
}

#[no_mangle]
pub fn assoc_patsub(
    h: *mut HASH_TABLE,
    pat: *mut libc::c_char,
    rep: *mut libc::c_char,
    mflags: libc::c_int,
) -> *mut libc::c_char {
    let mut t: *mut libc::c_char;
    let pchar: libc::c_int;
    let qflags: libc::c_int;
    let pflags: libc::c_int;
    let mut wl: *mut WORD_LIST;
    let save: *mut WORD_LIST;
    if h.is_null() || unsafe { assoc_empty!(h) } {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    wl = assoc_to_word_list(h);
    if wl.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    save = wl;
    unsafe {
        while !wl.is_null() {
            t = pat_subst((*(*wl).word).word, pat, rep, mflags);
            if !((*(*wl).word).word).is_null() {
                libc::free((*(*wl).word).word as *mut libc::c_void);
            }
            (*(*wl).word).word = 0 as *mut libc::c_char;
            (*(*wl).word).word = t;
            wl = (*wl).next;
        }
    }
    pchar = if mflags & MATCH_STARSUB as libc::c_int == MATCH_STARSUB as libc::c_int {
        '*' as i32
    } else {
        '@' as i32
    };
    qflags = if mflags & MATCH_QUOTED as libc::c_int == MATCH_QUOTED as libc::c_int {
        Q_DOUBLE_QUOTES as libc::c_int
    } else {
        0 as libc::c_int
    };
    pflags = if mflags & MATCH_ASSIGNRHS as libc::c_int == MATCH_ASSIGNRHS as libc::c_int {
        PF_ASSIGNRHS as libc::c_int
    } else {
        0 as libc::c_int
    };
    t = string_list_pos_params(pchar, save, qflags, pflags);
    dispose_words(save);
    return t;
}

#[no_mangle]
pub fn assoc_modcase(
    h: *mut HASH_TABLE,
    pat: *mut libc::c_char,
    modop: libc::c_int,
    mflags: libc::c_int,
) -> *mut libc::c_char {
    let mut t: *mut libc::c_char;
    let pchar: libc::c_int;
    let qflags: libc::c_int;
    let pflags: libc::c_int;
    let mut wl: *mut WORD_LIST;
    let save: *mut WORD_LIST;
    if unsafe { h.is_null() || (*h).nentries == 0 as libc::c_int } {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    wl = assoc_to_word_list(h);
    if wl.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    save = wl;
    unsafe {
        while !wl.is_null() {
            t = c_sh_modcase((*(*wl).word).word, pat, modop);
            if !((*(*wl).word).word).is_null() {
                libc::free((*(*wl).word).word as *mut libc::c_void);
            }
            (*(*wl).word).word = 0 as *mut libc::c_char;
            (*(*wl).word).word = t;
            wl = (*wl).next;
        }
    }
    pchar = if mflags & MATCH_STARSUB as libc::c_int == MATCH_STARSUB as libc::c_int {
        '*' as i32
    } else {
        '@' as i32
    };
    qflags = if mflags & MATCH_QUOTED as libc::c_int == MATCH_QUOTED as libc::c_int {
        Q_DOUBLE_QUOTES as libc::c_int
    } else {
        0 as libc::c_int
    };
    pflags = if mflags & MATCH_ASSIGNRHS as libc::c_int == MATCH_ASSIGNRHS as libc::c_int {
        PF_ASSIGNRHS as libc::c_int
    } else {
        0 as libc::c_int
    };
    t = string_list_pos_params(pchar, save, qflags, pflags);
    dispose_words(save);
    return t;
}

#[no_mangle]
pub fn assoc_to_kvpair(hash: *mut HASH_TABLE, quoted: libc::c_int) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char;
    let mut istr: *mut libc::c_char;
    let mut vstr: *mut libc::c_char;
    let mut i: libc::c_int;
    let mut rsize: libc::c_int;
    let mut rlen: libc::c_int;
    let mut elen: libc::c_int;
    let mut tlist: *mut BUCKET_CONTENTS;

    unsafe {
        if hash.is_null() || assoc_empty!(hash) {
            return 0 as *mut libc::c_char;
        }
        rsize = 128 as libc::c_int;
        ret = libc::malloc(rsize as size_t as usize) as *mut libc::c_char;
        rlen = 0 as libc::c_int;
        *ret.offset(rlen as isize) = '\0' as i32 as libc::c_char;
        i = 0 as libc::c_int;
        while i < (*hash).nbuckets {
            tlist = hash_items!(i, hash);
            while !tlist.is_null() {
                if c_ansic_shouldquote((*tlist).key) != 0 {
                    istr = c_ansic_quote((*tlist).key, 0 as libc::c_int, 0 as *mut libc::c_int);
                } else if c_sh_contains_shell_metas((*tlist).key) != 0 {
                    istr = c_sh_double_quote((*tlist).key);
                } else if ALL_ELEMENT_SUB!(*((*tlist).key) as i32)
                    && *((*tlist).key).offset(1 as libc::c_int as isize) as libc::c_int
                        == '\0' as i32
                {
                    istr = c_sh_double_quote((*tlist).key);
                } else {
                    istr = (*tlist).key;
                }
                vstr = if !((*tlist).data).is_null() {
                    if c_ansic_shouldquote((*tlist).data as *mut libc::c_char) != 0 {
                        c_ansic_quote(
                            (*tlist).data as *mut libc::c_char,
                            0 as libc::c_int,
                            0 as *mut libc::c_int,
                        )
                    } else {
                        c_sh_double_quote((*tlist).data as *mut libc::c_char)
                    }
                } else {
                    0 as *mut libc::c_char
                };
                elen = STRLEN!(istr) + 4 as libc::c_int + STRLEN!(vstr);
                RESIZE_MALLOCED_BUFFER!(ret, rlen, (elen + 1), rsize, rsize);
                libc::strcpy(ret.offset(rlen as isize), istr);
                rlen += STRLEN!(istr);
                *ret.offset(rlen as isize) = ' ' as i32 as libc::c_char;
                rlen += 1;
                if !vstr.is_null() {
                    libc::strcpy(ret.offset(rlen as isize), vstr);
                    rlen += STRLEN!(vstr);
                } else {
                    libc::strcpy(
                        ret.offset(rlen as isize),
                        b"\"\"\0" as *const u8 as *const libc::c_char,
                    );
                    rlen += 2 as libc::c_int;
                }
                *ret.offset(rlen as isize) = ' ' as i32 as libc::c_char;
                rlen += 1;

                if istr != (*tlist).key {
                    if !istr.is_null() {
                        libc::free(istr as *mut libc::c_void);
                    }
                    // istr = 0 as *mut libc::c_char;
                }
                if !vstr.is_null() {
                    libc::free(vstr as *mut libc::c_void);
                }
                // vstr = 0 as *mut libc::c_char;
                tlist = (*tlist).next;
            }
            i += 1;
        }

        RESIZE_MALLOCED_BUFFER!(ret, rlen, 1 as libc::c_int, rsize, 8 as libc::c_int);
        *ret.offset(rlen as isize) = '\0' as i32 as libc::c_char;

        if quoted != 0 {
            vstr = c_sh_single_quote(ret);
            libc::free(ret as *mut libc::c_void);
            ret = vstr;
        }
    }
    return ret;
}

#[no_mangle]
pub fn assoc_to_assign(hash: *mut HASH_TABLE, quoted: libc::c_int) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char;
    let mut istr: *mut libc::c_char;
    let mut vstr: *mut libc::c_char;
    let mut i: libc::c_int;
    let mut rsize: libc::c_int;
    let mut rlen: libc::c_int;
    let mut elen: libc::c_int;
    let mut tlist: *mut BUCKET_CONTENTS;
    unsafe {
        if hash.is_null() || assoc_empty!(hash) {
            return 0 as *mut libc::c_char;
        }
        rsize = 128 as libc::c_int;
        ret = libc::malloc(rsize as size_t as usize) as *mut libc::c_char;
        *ret.offset(0 as libc::c_int as isize) = '(' as i32 as libc::c_char;
        rlen = 1 as libc::c_int;

        i = 0 as libc::c_int;

        while i < (*hash).nbuckets {
            tlist = hash_items!(i, hash);
            while !tlist.is_null() {
                if c_ansic_shouldquote((*tlist).key) != 0 {
                    istr = c_ansic_quote((*tlist).key, 0 as libc::c_int, 0 as *mut libc::c_int);
                } else if c_sh_contains_shell_metas((*tlist).key) != 0 {
                    istr = c_sh_double_quote((*tlist).key);
                } else if ALL_ELEMENT_SUB!(*((*tlist).key) as i32)
                    && *((*tlist).key).offset(1 as libc::c_int as isize) as libc::c_int
                        == '\0' as i32
                {
                    istr = c_sh_double_quote((*tlist).key);
                } else {
                    istr = (*tlist).key;
                }
                vstr = if !((*tlist).data).is_null() {
                    if c_ansic_shouldquote((*tlist).data as *mut libc::c_char) != 0 {
                        c_ansic_quote(
                            (*tlist).data as *mut libc::c_char,
                            0 as libc::c_int,
                            0 as *mut libc::c_int,
                        )
                    } else {
                        c_sh_double_quote((*tlist).data as *mut libc::c_char)
                    }
                } else {
                    0 as *mut libc::c_char
                };
                elen = STRLEN!(istr) + 8 as libc::c_int + STRLEN!(vstr);
                RESIZE_MALLOCED_BUFFER!(ret, rlen, (elen + 1), rsize, rsize);
                *ret.offset(rlen as isize) = '[' as i32 as libc::c_char;
                rlen += 1;
                libc::strcpy(ret.offset(rlen as isize), istr);
                rlen += STRLEN!(istr);

                *ret.offset(rlen as isize) = ']' as i32 as libc::c_char;
                rlen += 1;

                *ret.offset(rlen as isize) = '=' as i32 as libc::c_char;
                rlen += 1;
                if !vstr.is_null() {
                    libc::strcpy(ret.offset(rlen as isize), vstr);
                    rlen += STRLEN!(istr);
                }

                *ret.offset(rlen as isize) = ' ' as i32 as libc::c_char;
                rlen += 1;

                if istr != (*tlist).key {
                    if !istr.is_null() {
                        libc::free(istr as *mut libc::c_void);
                    }
                    // istr = 0 as *mut libc::c_char;
                }
                if !vstr.is_null() {
                    libc::free(vstr as *mut libc::c_void);
                }
                // vstr = 0 as *mut libc::c_char;
                tlist = (*tlist).next;
            }
            i += 1;
        }
        RESIZE_MALLOCED_BUFFER!(ret, rlen, 1 as libc::c_int, rsize, 8 as libc::c_int);
        *ret.offset(rlen as isize) = ')' as i32 as libc::c_char;
        rlen = rlen + 1;
        *ret.offset(rlen as isize) = '\0' as i32 as libc::c_char;
        if quoted != 0 {
            vstr = c_sh_single_quote(ret);
            libc::free(ret as *mut libc::c_void);
            ret = vstr;
        }
    }
    return ret;
}

fn assoc_to_word_list_internal(h: *mut HASH_TABLE, t: libc::c_int) -> *mut WORD_LIST {
    let mut list: *mut WORD_LIST;
    let mut i: libc::c_int;
    let mut tlist: *mut BUCKET_CONTENTS;
    let mut w: *mut libc::c_char;
    unsafe {
        if h.is_null() || assoc_empty!(h) {
            return 0 as *mut libc::c_void as *mut WORD_LIST;
        }
        list = 0 as *mut libc::c_void as *mut WORD_LIST;
        i = 0 as libc::c_int;

        while i < (*h).nbuckets {
            tlist = hash_items!(i, h);
            while !tlist.is_null() {
                w = if t == 0 as libc::c_int {
                    (*tlist).data as *mut libc::c_char
                } else {
                    (*tlist).key
                };
                list = make_word_list(make_bare_word(w), list);
                tlist = (*tlist).next;
            }
            i += 1;
        }
        return REVERSE_LIST!(list, *mut WORD_LIST);
    }
}

#[no_mangle]
pub fn assoc_to_word_list(h: *mut HASH_TABLE) -> *mut WORD_LIST {
    return assoc_to_word_list_internal(h, 0 as libc::c_int);
}

#[no_mangle]
pub fn assoc_keys_to_word_list(h: *mut HASH_TABLE) -> *mut WORD_LIST {
    return assoc_to_word_list_internal(h, 1 as libc::c_int);
}

#[no_mangle]
pub fn assoc_to_string(
    h: *mut HASH_TABLE,
    sep: *mut libc::c_char,
    quoted: libc::c_int,
) -> *mut libc::c_char {
    let mut tlist: *mut BUCKET_CONTENTS;
    let mut i: libc::c_int;
    let result: *mut libc::c_char;
    let mut t: *mut libc::c_char;
    let mut w: *mut libc::c_char;
    let mut list: *mut WORD_LIST;
    let l: *mut WORD_LIST;

    if h.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    unsafe {
        if assoc_empty!(h) {
            return savestring!(b"\0" as *const u8 as *const libc::c_char);
        }

        // result = 0 as *mut libc::c_char;
        list = 0 as *mut WORD_LIST;
        // l = list;
        i = 0 as libc::c_int;
        while i < (*h).nbuckets {
            tlist = hash_items!(i, h);
            while !tlist.is_null() {
                w = (*tlist).data as *mut libc::c_char;
                if !w.is_null() {
                    t = if quoted != 0 {
                        quote_string(w)
                    } else {
                        savestring!(w)
                    };
                    list = make_word_list(make_bare_word(t), list);
                    if !t.is_null() {
                        libc::free(t as *mut libc::c_void);
                    }
                    // t = 0 as *mut libc::c_char;
                }
                tlist = (*tlist).next;
            }
            i += 1;
        }
        l = REVERSE_LIST!(list, *mut WORD_LIST);
        result = if !l.is_null() {
            string_list_internal(l, sep)
        } else {
            savestring!(b"\0" as *const u8 as *const libc::c_char)
        };
        dispose_words(l);
    }
    return result;
}
