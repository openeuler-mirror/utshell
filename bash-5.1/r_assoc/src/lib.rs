use libc::*;
use r_bash::*;
use rcommon::WordList as WORD_LIST;
use rcommon::WordDesc as WORD_DESC;

pub type sh_free_func_t = unsafe extern "C" fn(*mut libc::c_void) -> ();

#[macro_export] 
macro_rules! assoc_empty {
    ($h:expr) =>{
        (*$h).nentries == 0 as libc::c_int
    }
}

#[macro_export]
macro_rules! savestring {
    ($name:expr) => {
        libc::strcpy(
            malloc((1 as libc::c_int + libc::strlen($name) as libc::c_int) as size_t) as *mut libc::c_char,
            $name )
    }
}

#[macro_export]
macro_rules! ALL_ELEMENT_SUB {
    ($c:expr) => {
        $c == '@' as i32 || $c == '*' as i32
    }
}

#[macro_export]
macro_rules! STRLEN {
    ($s:expr) => {
        if !$s.is_null()
        && *$s.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            if *$s.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                if *$s.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                    libc::strlen($s) as libc::c_int
                } else {
                    2 as libc::c_int 
                }
            } 
            else {
                1 as libc::c_int 
            }
        } 
        else {
        0 as libc::c_int 
        }
    }
}

#[macro_export]
macro_rules! RESIZE_MALLOCED_BUFFER {
    ($str:expr,$cind:expr,$room:expr,$csize:expr,$sincr:expr) => {
        if $cind + $room >= $csize
        {
            while $cind + $room >= $csize {
                $csize += $sincr;
            }   
            $str = xrealloc ($str as *mut c_void , $csize as usize) as *mut libc::c_char;
        } 
    }
}

#[macro_export]
macro_rules! REVERSE_LIST {
    ($list:expr,$type:ty) => {
        if !$list.is_null() && !((*$list).next).is_null() {
            list_reverse($list as *mut GENERIC_LIST) as $type
        }
        else {
            $list as $type
        }
    }

}

#[macro_export]
macro_rules! hash_items {
    ($bucket:expr,$table:expr) => {
        if !$table.is_null() && $bucket < (*$table).nbuckets {
            *((*$table).bucket_array).offset($bucket as isize)
        } else {
            0 as *mut libc::c_void as *mut BUCKET_CONTENTS
        };
    }
}

extern "C" {
    fn hash_flush(_: *mut HASH_TABLE, _: Option::<sh_free_func_t>);
    fn hash_dispose(_: *mut HASH_TABLE);
    fn malloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn hash_search(
        _: *const libc::c_char,
        _: *mut HASH_TABLE,
        _: libc::c_int,
    ) -> *mut BUCKET_CONTENTS;
    fn hash_remove(
        _: *const libc::c_char,
        _: *mut HASH_TABLE,
        _: libc::c_int,
    ) -> *mut BUCKET_CONTENTS;
    fn dispose_words(_: *mut WORD_LIST);
    fn string_list_pos_params(
        _: libc::c_int,
        _: *mut WORD_LIST,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    fn list_reverse(list:*mut GENERIC_LIST) -> *mut GENERIC_LIST;
    fn make_bare_word(_: *const libc::c_char) -> *mut WORD_DESC;
    fn make_word_list(_: *mut WORD_DESC, _: *mut WORD_LIST) -> *mut WORD_LIST;

}
#[no_mangle]
pub unsafe extern "C" fn assoc_dispose(mut hash: *mut HASH_TABLE) {
    if !hash.is_null() {
        hash_flush(hash, None);
        hash_dispose(hash);
    }
}

#[no_mangle]
pub unsafe extern "C" fn assoc_flush(mut hash: *mut HASH_TABLE) {
    hash_flush(hash, None);
}

#[no_mangle]
pub unsafe extern "C" fn assoc_insert(
    mut hash: *mut HASH_TABLE,
    mut key: *mut libc::c_char,
    mut value: *mut libc::c_char,
) -> libc::c_int {
    let mut b: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    b = hash_search(key, hash, 0x2 as libc::c_int);
    if b.is_null() {
        return -(1 as libc::c_int);
    }
    if (*b).key != key {
        libc::free(key as *mut libc::c_void);
    }
    if !((*b).data).is_null() {
        libc::free((*b).data);
    }
    (*b).data = 0 as *mut libc::c_void;
    (*b)
        .data = (if !value.is_null() {
       savestring!(value)
    } else {
        0 as *mut libc::c_char 
    }) as *mut libc::c_void;
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn assoc_remove(
    mut hash: *mut HASH_TABLE,
    mut string: *mut libc::c_char,
) {
    let mut b: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    b = hash_remove(string, hash, 0 as libc::c_int);
    if !b.is_null() {
        libc::free((*b).data as *mut libc::c_char as *mut libc::c_void);
        libc::free((*b).key as *mut libc::c_void);
        libc::free(b as *mut libc::c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn assoc_reference(
    mut hash: *mut HASH_TABLE,
    mut string: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut b: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    if hash.is_null() {
        return 0 as *mut libc::c_char;
    }
    b = hash_search(string, hash, 0 as libc::c_int);
    return if !b.is_null() {
        (*b).data as *mut libc::c_char
    } else {
        0 as *mut libc::c_char
    };
}

#[no_mangle]
pub unsafe extern "C" fn assoc_quote(mut h: *mut HASH_TABLE) -> *mut HASH_TABLE {
    let mut i: libc::c_int = 0;
    let mut tlist: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    if h.is_null() ||  assoc_empty!(h)  {
        return 0 as *mut libc::c_void as *mut HASH_TABLE;
    }
    i = 0 as libc::c_int;
    while i < (*h).nbuckets {
        tlist = hash_items!(i,h);
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
        i;
    }
    return h;
}

#[no_mangle]
pub unsafe extern "C" fn assoc_quote_escapes(mut h: *mut HASH_TABLE) -> *mut HASH_TABLE {
    let mut i: libc::c_int = 0;
    let mut tlist: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    if h.is_null() ||  assoc_empty!(h)  {
        return 0 as *mut libc::c_void as *mut HASH_TABLE;
    }
    i = 0 as libc::c_int;
    while i < (*h).nbuckets {
        tlist = hash_items!(i,h);
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
        i;
    }
    return h;
}

#[no_mangle]
pub unsafe extern "C" fn assoc_dequote(mut h: *mut HASH_TABLE) -> *mut HASH_TABLE {
    let mut i: libc::c_int = 0;
    let mut tlist: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    if h.is_null() || assoc_empty!(h) {
        return 0 as *mut libc::c_void as *mut HASH_TABLE;
    }
    i = 0 as libc::c_int;
    while i < (*h).nbuckets {
        tlist = hash_items!(i,h);
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
        i;
    }
    return h;
}

#[no_mangle]
pub unsafe extern "C" fn assoc_dequote_escapes(
    mut h: *mut HASH_TABLE,
) -> *mut HASH_TABLE {
    let mut i: libc::c_int = 0;
    let mut tlist: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    if h.is_null() || assoc_empty!(h) {
        return 0 as *mut libc::c_void as *mut HASH_TABLE;
    }
    i = 0 as libc::c_int;
    while i < (*h).nbuckets {
        tlist = hash_items!(i,h);
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
        i;
    }
    return h;
}

#[no_mangle]
pub unsafe extern "C" fn assoc_remove_quoted_nulls(
    mut h: *mut HASH_TABLE,
) -> *mut HASH_TABLE {
    let mut i: libc::c_int = 0;
    let mut tlist: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    if h.is_null() || assoc_empty!(h) {
        return 0 as *mut libc::c_void as *mut HASH_TABLE;
    }
    i = 0 as libc::c_int;
    while i < (*h).nbuckets {
        tlist = hash_items!(i,h);
        while !tlist.is_null() {
            t = remove_quoted_nulls((*tlist).data as *mut libc::c_char);
            (*tlist).data = t as *mut libc::c_void;
            tlist = (*tlist).next;
        }
        i += 1;
        i;
    }
    return h;
}

#[no_mangle]
pub unsafe extern "C" fn assoc_subrange(
    mut hash: *mut HASH_TABLE,
    mut start: arrayind_t,
    mut nelem: arrayind_t,
    mut starsub: libc::c_int,
    mut quoted: libc::c_int,
    mut pflags: libc::c_int,
) -> *mut libc::c_char {
    let mut l: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut save: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut h: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut t: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    if assoc_empty!(hash) {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    l = assoc_to_word_list(hash);
    save = l;

    if save.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    i = 1 as libc::c_int;

    while !l.is_null() && (i as libc::c_long) < start {
        l = (*l).next;
        i += 1;
        i;
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
        l = (*l).next;
        j += 1;
        j;
    }
    (*t).next = 0 as *mut libc::c_void as *mut WORD_LIST;
    ret = string_list_pos_params(
        if starsub != 0 { '*' as i32 } else { '@' as i32 },
        h,
        quoted,
        pflags,
    );
    if t != l {
        (*t).next = l;
    }
    dispose_words(save);
    return ret;
}

#[no_mangle]
pub unsafe extern "C" fn assoc_patsub(
    mut h: *mut HASH_TABLE,
    mut pat: *mut libc::c_char,
    mut rep: *mut libc::c_char,
    mut mflags: libc::c_int,
) -> *mut libc::c_char {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pchar: libc::c_int = 0;
    let mut qflags: libc::c_int = 0;
    let mut pflags: libc::c_int = 0;
    let mut wl: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut save: *mut WORD_LIST = 0 as *mut WORD_LIST;
    if h.is_null() || assoc_empty!(h) {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    wl = assoc_to_word_list(h);
    if wl.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    save = wl;
    while !wl.is_null() {
        t = pat_subst((*(*wl).word).word, pat, rep, mflags);
        if !((*(*wl).word).word).is_null() {
            libc::free((*(*wl).word).word as *mut libc::c_void);
        }
        (*(*wl).word).word = 0 as *mut libc::c_char;
        (*(*wl).word).word = t;
        wl = (*wl).next;
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
        PF_ASSIGNRHS
         as libc::c_int
    } else {
        0 as libc::c_int
    };
    t = string_list_pos_params(pchar, save, qflags, pflags);
    dispose_words(save);
    return t;
}

#[no_mangle]
pub unsafe extern "C" fn assoc_modcase(
    mut h: *mut HASH_TABLE,
    mut pat: *mut libc::c_char,
    mut modop: libc::c_int,
    mut mflags: libc::c_int,
) -> *mut libc::c_char {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pchar: libc::c_int = 0;
    let mut qflags: libc::c_int = 0;
    let mut pflags: libc::c_int = 0;
    let mut wl: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut save: *mut WORD_LIST = 0 as *mut WORD_LIST;
    if h.is_null() || (*h).nentries == 0 as libc::c_int {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    wl = assoc_to_word_list(h);
    if wl.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    save = wl;
    while !wl.is_null() {
        t = sh_modcase((*(*wl).word).word, pat, modop);
        if !((*(*wl).word).word).is_null() {
            libc::free((*(*wl).word).word as *mut libc::c_void);
        }
        (*(*wl).word).word = 0 as *mut libc::c_char;
        (*(*wl).word).word = t;
        wl = (*wl).next;
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
