//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later


use r_bash::*;
use rcommon::WordList as WORD_LIST;

pub const array_indexed: atype = 0;
extern "C" {
    fn list_reverse(list:*mut GENERIC_LIST) -> *mut GENERIC_LIST;
}

pub type sh_ae_map_func_t =
    unsafe extern "C" fn(
        arg1: *mut ARRAY_ELEMENT,
        arg2: *mut libc::c_void,
    ) -> libc::c_int;

#[macro_export]
macro_rules! element_forw {
    ($ae:expr) => {
        (*$ae).next
    };
}

#[macro_export]
macro_rules! element_back {
    ($ae:expr) => {
        (*$ae).prev
    };
}

#[macro_export]
macro_rules! element_index {
    ($ae:expr) => {
        (*$ae).ind
    };
}

#[macro_export]
macro_rules! element_value {
    ($ae:expr) => {
        (*$ae).value
    };
}

#[macro_export]
macro_rules! ADD_BEFORE {
    ($ae:expr,$new:expr) => {
        (*(*$ae).prev).next = $new;
        (*$new).prev = (*$ae).prev;
        (*$ae).prev = $new ;
        (*$new).next = $ae;
    };
}

#[macro_export]
macro_rules! ADD_AFTER {
    ($ae:expr,$new:expr) => {
        (*(*$ae).next).prev = $new;
        (*$new).next = (*$ae).next;
        (*$new).prev = $ae;
        (*$ae).next = $new;
    };
}

#[macro_export]
macro_rules! REVERSE_LIST {
    ($list:expr,$type:ty) => {
        if !$list.is_null() && !((*$list).next).is_null() {
            list_reverse($list as *mut GENERIC_LIST) as $type
        } else {
            $list as $type
        }
    }
}

#[macro_export]
macro_rules! array_empty {
    ($a:expr) => {
        (*$a).num_elements == 0
    }
} 
#[macro_export]
macro_rules! LASTREF{
    ($a:expr) => {
        if !(*$a).lastref.is_null() {
            (*$a).lastref
        }
        else {
            element_forw!((*$a).head)
        }
    }
}

#[macro_export]
macro_rules! SET_LASTREF{
    ($a:expr,$e:expr) => {
        (*$a).lastref = $e
    }
}

#[macro_export]
macro_rules! INVALIDATE_LASTREF{
    ($a:expr) => {
        (*$a).lastref = 0 as *mut array_element
    }
}

#[macro_export]
macro_rules! array_max_index{
    ($a:expr) => {
        (*$a).max_index 
    }
}

#[macro_export]
macro_rules! array_first_index{
    ($a:expr) => {
        (*(*(*$a).head).next).ind
    }
}

#[macro_export]
macro_rules! array_num_elements{
    ($a:expr) => {
        (*$a).num_elements
    }
} 

#[macro_export]
macro_rules! array_head{
    ($a:expr) => {
        (*$a).head
    }
}

#[macro_export]
macro_rules! savestring {
    ($x:expr) => {
        strcpy(
            malloc(
                (strlen($x as *const libc::c_char) + 1) as usize)
                 as *mut libc::c_char, $x
                ) as *mut libc::c_char
    };
}

#[macro_export]
macro_rules!  STRLEN{
    ($s:expr) => {
        if !$s.is_null() && *$s.offset(0 as isize) as libc::c_int != 0
            {
                if *$s.offset(1 as isize) as libc::c_int != 0 {
                    if *$s.offset(2 as isize) as libc::c_int != 0 {
                        libc::strlen($s)
                    } else {
                        2 
                    }
                } else {
                    1 
                }
            } else {
                0 
            }
    };
}

#[macro_export]
macro_rules! RESIZE_MALLOCED_BUFFER {
    ($srt:expr,$cind:expr, $room:expr, $csize:expr, $sincr:expr) => {
        if $cind + $room >= $csize {
            while $cind + $room >= $csize {
                $csize += $sincr;
            }
            $srt = realloc($srt as *mut libc::c_void, $csize as usize) as *mut libc::c_char;
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn  array_create() -> *mut ARRAY {

    let mut r: *mut ARRAY = 0 as *mut ARRAY;
    let mut head: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    r = malloc(std::mem::size_of::<ARRAY>() as usize) as *mut ARRAY;
    (*r).type_ = array_indexed;
    (*r).max_index = -(1 as libc::c_int) as arrayind_t;
    (*r).num_elements = 0 as libc::c_int;
    (*r).lastref = 0 as *mut ARRAY_ELEMENT;
    head = array_create_element(
        -(1 as libc::c_int) as arrayind_t,
        0 as *mut libc::c_void as *mut libc::c_char,
    );
    (*head).next = head;
    (*head).prev = (*head).next;
    (*r).head = head;
    return r;
}

#[no_mangle]
pub unsafe extern "C" fn array_flush(mut a: *mut ARRAY) {
    let mut r: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    let mut r1: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    if a.is_null() {
        return;
    }
    r = element_forw!((*a).head) ;
    while r != (*a).head {
        r1 = element_forw!(r);
        array_dispose_element(r);
        r = r1;
    }
    (*(*a).head).prev = (*a).head;
    (*(*a).head).next = (*(*a).head).prev;
    (*a).max_index = -(1 as libc::c_int) as arrayind_t;
    (*a).num_elements = 0 as libc::c_int;
    (*a).lastref = 0 as *mut array_element;
}

#[no_mangle]
pub unsafe extern "C" fn array_dispose(mut a: *mut ARRAY) {
    if a.is_null() {
        return;
    }
    array_flush(a);
    array_dispose_element((*a).head);
    free(a as *mut libc::c_void);
}

#[no_mangle]
pub unsafe extern "C" fn array_copy(mut a: *mut ARRAY) -> *mut ARRAY {
    let mut a1: *mut ARRAY = 0 as *mut ARRAY;
    let mut ae: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    let mut new: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    if a.is_null() {
        return 0 as *mut libc::c_void as *mut ARRAY;
    }
    a1 = array_create();
    (*a1).type_ = (*a).type_;
    (*a1).max_index = (*a).max_index;
    (*a1).num_elements = (*a).num_elements;
    ae = element_forw!((*a).head);
    while ae != (*a).head {
        new = array_create_element(element_index!(ae), element_value!(ae));
        ADD_BEFORE!((*a1).head,new);
        if ae == LASTREF!(a) 
        {
            SET_LASTREF!(a1, new);
        }
        ae = element_forw!(ae);
    }
    return a1;
}

#[no_mangle]
pub unsafe extern "C" fn array_slice(
    mut array: *mut ARRAY,
    mut s: *mut ARRAY_ELEMENT,
    mut e: *mut ARRAY_ELEMENT,
) -> *mut ARRAY {
    let mut a: *mut ARRAY = 0 as *mut ARRAY;
    let mut p: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    let mut n: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    let mut i: libc::c_int = 0;
    let mut mi: arrayind_t = 0;
    a = array_create();
    (*a).type_ = (*array).type_;
    mi = 0 as libc::c_int as arrayind_t;
    p = s;
    i = 0 as libc::c_int;
    while p != e {
        n = array_create_element(element_index!(p), element_value!(p));
        ADD_BEFORE!((*a).head, n);
        mi = element_index!(n);
        p = element_forw!(p);
        i += 1;

    }
    (*a).num_elements = i;
    (*a).max_index = mi;
    return a;
}

#[no_mangle]
pub unsafe extern "C" fn array_walk(
    mut a: *mut ARRAY,
    mut func: Option::<sh_ae_map_func_t>,
    mut udata: *mut libc::c_void,
) {
    let mut ae: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    if a.is_null() || array_empty!(a) {
        return;
    }
    ae = element_forw!((*a).head); 
    while ae != (*a).head {
        if (Some(func.expect("non-null function pointer")))
            .expect("non-null function pointer")(ae, udata) < 0 as libc::c_int
        {
            return;
        }
        ae = element_forw!(ae);
    }
}

#[no_mangle]
pub unsafe extern "C" fn array_shift(
    mut a: *mut ARRAY,
    mut n: libc::c_int,
    mut flags: libc::c_int,
) -> *mut ARRAY_ELEMENT {
    let mut ae: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    let mut ret: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    let mut i: libc::c_int = 0;
    if a.is_null() || array_empty!(a)  || n <= 0 as libc::c_int {
        return 0 as *mut libc::c_void as *mut ARRAY_ELEMENT;
    }
    INVALIDATE_LASTREF!(a);

    i = 0 as libc::c_int;
    ae =  element_forw!((*a).head);
    ret = ae;
    while ae != (*a).head && i < n {
        ae = element_forw!(ae);
        i += 1;
    }
    if ae == (*a).head {
        if flags & AS_DISPOSE as libc::c_int != 0 {
            array_flush(a);
            return 0 as *mut libc::c_void as *mut ARRAY_ELEMENT;
        }
        ae = ret;
        while element_forw!(ae) != (*a).head {
            ae = element_forw!(ae);
        }
        element_forw!(ae) = 0 as *mut libc::c_void as *mut ARRAY_ELEMENT;
        (*(*a).head).prev = (*a).head;
        (*(*a).head).next = (*(*a).head).prev;
        (*a).max_index = -(1 as libc::c_int) as arrayind_t;
        (*a).num_elements = 0 as libc::c_int;
        return ret;
    }
    (*(*ae).prev).next = 0 as *mut libc::c_void as *mut ARRAY_ELEMENT;
    (*(*a).head).next = ae;
    (*ae).prev = (*a).head;
    while ae != (*a).head {
        element_index!(ae) -= n as libc::c_long;
        ae = element_forw!(ae);
    }
    (*a).num_elements -= n;
    (*a).max_index = element_index!((*(*a).head).prev);
    if flags & AS_DISPOSE as libc::c_int != 0 {
        ae = ret;
        while !ae.is_null() {
            ret = element_forw!(ae);
            array_dispose_element(ae);
            ae = ret;
        }
        return 0 as *mut libc::c_void as *mut ARRAY_ELEMENT;
    }
    return ret;
}

#[no_mangle]
pub unsafe extern "C" fn array_rshift(
    mut a: *mut ARRAY,
    mut n: libc::c_int,
    mut s: *mut libc::c_char,
) -> libc::c_int {
    let mut ae: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    let mut new: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    if a.is_null() || array_empty!(a) && s.is_null() {
        return 0 as libc::c_int
    } else if n <= 0 as libc::c_int {
        return (*a).num_elements
    }
    ae = element_forw!((*a).head);
    if !s.is_null() {
        new = array_create_element(0 as libc::c_int as arrayind_t, s);
        ADD_BEFORE!(ae, new);
        (*a).num_elements += 1;
        if array_num_elements!(a) == 1 as libc::c_int {
            (*a).max_index = 0 as libc::c_int as arrayind_t;
            return 1 as libc::c_int;
        }
    }
    while ae != (*a).head {
        element_index!(ae) += n as libc::c_long;
        ae = element_forw!(ae);
    }
    (*a).max_index = (*(*(*a).head).prev).ind;
    INVALIDATE_LASTREF!(a);
    return (*a).num_elements;
}

#[no_mangle]
pub unsafe extern "C" fn array_unshift_element(mut a: *mut ARRAY) -> *mut ARRAY_ELEMENT {
    return array_shift(a, 1 as libc::c_int, 0 as libc::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn array_shift_element(
    mut a: *mut ARRAY,
    mut v: *mut libc::c_char,
) -> libc::c_int {
    return array_rshift(a, 1 as libc::c_int, v);
}

#[no_mangle]
pub unsafe extern "C" fn array_quote(mut array: *mut ARRAY) -> *mut ARRAY {
    let mut a: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    if array.is_null() || array_head!(array).is_null()
        || array_empty!(array)
    {
        return 0 as *mut libc::c_void as *mut ARRAY;
    }
    a = element_forw!((*array).head);
    while a != (*array).head {
        t = quote_string((*a).value);
        if !((*a).value).is_null() {
            free((*a).value as *mut libc::c_void);
        }
        (*a).value = 0 as *mut libc::c_char;
        (*a).value = t;
        a = element_forw!(a);
    }
    return array;
}

#[no_mangle]
pub unsafe extern "C" fn array_quote_escapes(mut array: *mut ARRAY) -> *mut ARRAY {
    let mut a: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    if array.is_null() || array_head!(array).is_null()
        || array_empty!(array)
    {
        return 0 as *mut libc::c_void as *mut ARRAY;
    }
    a = element_forw!((*array).head);
    while a != (*array).head {
        t = quote_escapes((*a).value);
        if !((*a).value).is_null() {
            free((*a).value as *mut libc::c_void);
        }
        (*a).value = 0 as *mut libc::c_char;
        (*a).value = t;
        a = element_forw!(a);
    }
    return array;
}

#[no_mangle]
pub unsafe extern "C" fn array_dequote(mut array: *mut ARRAY) -> *mut ARRAY {
    let mut a: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    if array.is_null() || array_head!(array).is_null()
        || array_empty!(array)
    {
        return 0 as *mut libc::c_void as *mut ARRAY;
    }
    a = element_forw!((*array).head);
    while a != (*array).head {
        t = dequote_string((*a).value);
        if !((*a).value).is_null() {
            free((*a).value as *mut libc::c_void);
        }
        (*a).value = 0 as *mut libc::c_char;
        (*a).value = t;
        a = element_forw!(a);
    }
    return array;
}

#[no_mangle]
pub unsafe extern "C" fn array_dequote_escapes(mut array: *mut ARRAY) -> *mut ARRAY {
    let mut a: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    if array.is_null() || array_head!(array).is_null()
        || array_empty!(array)
    {
        return 0 as *mut libc::c_void as *mut ARRAY;
    }
    a = element_forw!((*array).head);
    while a != (*array).head {
        t = dequote_escapes((*a).value);
        if !((*a).value).is_null() {
            free((*a).value as *mut libc::c_void);
        }
        (*a).value = 0 as *mut libc::c_char;
        (*a).value = t;
        a = element_forw!(a);
    }
    return array;
}

#[no_mangle]
pub unsafe extern "C" fn array_remove_quoted_nulls(mut array: *mut ARRAY) -> *mut ARRAY {
    let mut a: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    if array.is_null() || array_head!(array).is_null()
        || array_empty!(array)
    {
        return 0 as *mut libc::c_void as *mut ARRAY;
    }
    a = element_forw!((*array).head); 
    while a != (*array).head {
        (*a).value = remove_quoted_nulls((*a).value);
        a = element_forw!(a);
    }
    return array;
}

#[no_mangle]
pub unsafe extern "C" fn array_subrange(
    mut a: *mut ARRAY,
    mut start: arrayind_t,
    mut nelem: arrayind_t,
    mut starsub: libc::c_int,
    mut quoted: libc::c_int,
    mut pflags: libc::c_int,
) -> *mut libc::c_char {
    let mut a2: *mut ARRAY = 0 as *mut ARRAY;
    let mut h: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    let mut p: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    let mut i: arrayind_t = 0;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut wl: *mut WORD_LIST = 0 as *mut WORD_LIST;
    
    p = if !a.is_null() { array_head!(a) } else { 0 as *mut array_element };
    if p.is_null() || array_empty!(a)  || start >  array_max_index!(a) {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    p = element_forw!(p);;
    while p != array_head!(a)  && start > element_index!(p){
        p = element_forw!(p);
    }
    if p == (*a).head {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    i = 0 as libc::c_int as arrayind_t;
    h = p;
    while p != (*a).head && i < nelem {
        i += 1;
        p = element_forw!(p);
    }
    a2 = array_slice(a, h, p);
    wl = array_to_word_list(a2);
    array_dispose(a2);
    if wl.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    t = string_list_pos_params(
        if starsub != 0 { '*' as i32 } else { '@' as i32 },
        wl,
        quoted,
        pflags,
    );
    dispose_words(wl);
    return t;
}

#[no_mangle]
pub unsafe extern "C" fn array_patsub(
    mut a: *mut ARRAY,
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
    if a.is_null() || array_head!(a).is_null() || array_empty!(a) {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    wl = array_to_word_list(a);
    if wl.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    save = wl;
    while !wl.is_null() {
        t = pat_subst((*(*wl).word).word, pat, rep, mflags);
        if !((*(*wl).word).word).is_null() {
            free((*(*wl).word).word as *mut libc::c_void);
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
        0x1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    pflags = if mflags & MATCH_ASSIGNRHS as libc::c_int != 0 {
        PF_ASSIGNRHS as libc::c_int
    } else {
        0 as libc::c_int
    };
    t = string_list_pos_params(pchar, save, qflags, pflags);
    dispose_words(save);
    return t;
}

#[no_mangle]
pub unsafe extern "C" fn array_modcase(
    mut a: *mut ARRAY,
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
    if a.is_null() || array_head!(a).is_null() || array_empty!(a) {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    wl = array_to_word_list(a);
    if wl.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    save = wl;
    while !wl.is_null() {
        t = sh_modcase((*(*wl).word).word, pat, modop);
        if !((*(*wl).word).word).is_null() {
            free((*(*wl).word).word as *mut libc::c_void);
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
    pflags = if mflags & MATCH_ASSIGNRHS as libc::c_int != 0 {
        PF_ASSIGNRHS as libc::c_int
    } else {
        0 as libc::c_int
    };
    t = string_list_pos_params(pchar, save, qflags, pflags);
    dispose_words(save);
    return t;
}

#[no_mangle]
pub unsafe extern "C" fn array_create_element(
    mut indx: arrayind_t,
    mut value: *mut libc::c_char,
) -> *mut ARRAY_ELEMENT {
    let mut r: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    r = malloc(std::mem::size_of::<ARRAY_ELEMENT>() as usize)
        as *mut ARRAY_ELEMENT;
    (*r).ind = indx;
    (*r)
        .value = if !value.is_null() {
            savestring!(value) 
    } else {
        0 as *mut libc::c_void as *mut libc::c_char
    };
    (*r).prev = 0 as *mut libc::c_void as *mut ARRAY_ELEMENT;
    (*r).next = (*r).prev;
    return r;
}

#[no_mangle]
pub unsafe extern "C" fn array_dispose_element(mut ae: *mut ARRAY_ELEMENT) {
    if !ae.is_null() {
        if !((*ae).value).is_null() {
            free((*ae).value as *mut libc::c_void);
        }
        (*ae).value = 0 as *mut libc::c_char;
        free(ae as *mut libc::c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn array_insert(
    mut a: *mut ARRAY,
    mut i: arrayind_t,
    mut v: *mut libc::c_char,
) -> libc::c_int {
    let mut new: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    let mut ae: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    let mut start: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    let mut startind: arrayind_t = 0;
    let mut direction: libc::c_int = 0;
    if a.is_null() {
        return -(1 as libc::c_int);
    }
    new = array_create_element(i, v);
    if i > array_max_index!(a) {
        ADD_BEFORE!((*a).head, new);
        (*a).max_index = i;
        (*a).num_elements += 1;
        SET_LASTREF!(a, new);
        return 0 as libc::c_int;
    } else if i < array_first_index!(a)  {
        ADD_AFTER!((*a).head, new);
        (*a).num_elements += 1;
        SET_LASTREF!(a, new);
        return 0 as libc::c_int;
    }
    start = LASTREF!(a); 
    startind = element_index!(start);
    if i < startind / 2 as libc::c_int as libc::c_long {
        start = element_forw!((*a).head);
        startind = element_index!(start);
        direction = 1 as libc::c_int;
    } else if i >= startind {
        direction = 1 as libc::c_int;
    } else {
        direction = -(1 as libc::c_int);
    }
    ae = start;
    while ae != (*a).head {
        if element_index!(ae) == i {
            free(element_value!(ae) as *mut libc::c_void);
            (*ae).value = (*new).value;
            (*new).value = 0 as *mut libc::c_char;
            array_dispose_element(new);
            SET_LASTREF!(a, ae);
            return 0 as libc::c_int;
        } else if direction == 1 as libc::c_int && (*ae).ind > i {
            ADD_BEFORE!(ae, new);
            (*a).num_elements += 1;
            SET_LASTREF!(a, new);
            return 0 as libc::c_int;
        } else if direction == -(1 as libc::c_int) && (*ae).ind < i {
            ADD_AFTER!(ae, new);
            (*a).num_elements += 1;
            (*a).lastref = new;
            SET_LASTREF!(a, new);
            return 0 as libc::c_int;
        }
        ae = if direction == 1 as libc::c_int 
        { element_forw!(ae) }
         else {
            element_back!(ae) 
        };
    }
    array_dispose_element(new);
    INVALIDATE_LASTREF!(a);
    return -(1 as libc::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn array_remove(
    mut a: *mut ARRAY,
    mut i: arrayind_t,
) -> *mut ARRAY_ELEMENT {
    let mut ae: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    let mut start: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    let mut startind: arrayind_t = 0;
    let mut direction: libc::c_int = 0;
    if a.is_null() || array_empty!(a){
        return 0 as *mut libc::c_void as *mut ARRAY_ELEMENT;
    }
    if i > array_max_index!(a) || i <  array_first_index!(a) {
        return 0 as *mut libc::c_void as *mut ARRAY_ELEMENT;
    }
    start = LASTREF!(a);
    startind = element_index!(start);
    if i < startind / 2 as libc::c_int as libc::c_long {
        start = element_forw!((*a).head);
        startind = element_index!(start);
        direction = 1 as libc::c_int;
    } else if i >= startind {
        direction = 1 as libc::c_int;
    } else {
        direction = -(1 as libc::c_int);
    }
    ae = start;
    while ae != (*a).head {
        if element_index!(ae) == i {
            (*(*ae).next).prev = (*ae).prev;
            (*(*ae).prev).next = (*ae).next;
            (*a).num_elements -= 1;
            (*a).num_elements;
            if i == array_max_index!(a){
                (*a).max_index = element_index!((*ae).prev);
            }
            if (*ae).next != (*a).head {
                SET_LASTREF!(a,(*ae).next);
            } else if (*ae).prev != (*a).head {
                SET_LASTREF!(a,(*ae).prev);
            } else {
                INVALIDATE_LASTREF!(a);
            }
            return ae;
        }
        ae = if direction == 1 as libc::c_int { element_forw!(ae) } else { element_back!(ae) };
        if direction == 1 as libc::c_int && element_index!(ae) > i {
            break;
        }
        if direction == -(1 as libc::c_int) && element_index!(ae) < i {
            break;
        }
    }
    return 0 as *mut libc::c_void as *mut ARRAY_ELEMENT;
}

#[no_mangle]
pub unsafe extern "C" fn array_reference(
    mut a: *mut ARRAY,
    mut i: arrayind_t,
) -> *mut libc::c_char {
    let mut ae: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    let mut start: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    let mut startind: arrayind_t = 0;
    let mut direction: libc::c_int = 0;
    if a.is_null() || array_empty!(a) {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    if i > array_max_index!(a) || i < array_first_index!(a) {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    start = LASTREF!(a);
    startind = element_index!(start);
    if i < startind / 2 as libc::c_int as libc::c_long {
        start = element_forw!((*a).head);;
        startind = element_index!(start);
        direction = 1 as libc::c_int;
    } else if i >= startind {
        direction = 1 as libc::c_int;
    } else {
        direction = -(1 as libc::c_int);
    }
    ae = start;
    while ae != (*a).head {
        if element_index!(ae) == i {
            SET_LASTREF!(a, ae);
            return element_value!(ae);
        }
        ae = if direction == 1 as libc::c_int { element_forw!(ae) } else { element_back!(ae) };
        if direction == 1 as libc::c_int && (*ae).ind > i {
            start = ae;
            break;
        } else {
            if !(direction == -(1 as libc::c_int) && (*ae).ind < i) {
                continue;
            }
            start = ae;
            break;
        }
    }
    SET_LASTREF!(a, start);
    return 0 as *mut libc::c_void as *mut libc::c_char;
}

#[no_mangle]
pub unsafe extern "C" fn array_to_word_list(mut a: *mut ARRAY) -> *mut WORD_LIST {
    let mut list: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut ae: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    if a.is_null() || (*a).num_elements == 0 as libc::c_int {
        return 0 as *mut libc::c_void as *mut WORD_LIST;
    }
    list = 0 as *mut libc::c_void as *mut WORD_LIST;
    ae = element_forw!((*a).head);
    while ae != (*a).head {
        list = make_word_list(make_bare_word(element_value!(ae)), list);
        ae = element_forw!(ae);
    }
    return REVERSE_LIST!(list,*mut WORD_LIST);
}


#[no_mangle]
pub unsafe extern "C" fn array_assign_list(
    mut array: *mut ARRAY,
    mut list: *mut WORD_LIST,
) -> *mut ARRAY {
    let mut l: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut i: arrayind_t = 0;
    l = list;
    i = 0 as libc::c_int as arrayind_t;
    while !l.is_null() {
        array_insert(array, i, (*(*l).word).word);
        l = (*l).next;
    }
    return array;
}


#[no_mangle]
pub unsafe extern "C" fn array_from_word_list(mut list: *mut WORD_LIST) -> *mut ARRAY {
    let mut a: *mut ARRAY = 0 as *mut ARRAY;
    if list.is_null() {
        return 0 as *mut libc::c_void as *mut ARRAY;
    }
    a = array_create();
    return array_assign_list(a, list);
}

#[no_mangle]
pub unsafe extern "C" fn array_keys_to_word_list(mut a: *mut ARRAY) -> *mut WORD_LIST {
    let mut list: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut ae: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    if a.is_null() || array_empty!(a){
        return 0 as *mut libc::c_void as *mut WORD_LIST;
    }
    list = 0 as *mut libc::c_void as *mut WORD_LIST;
    ae =  element_forw!((*a).head);
    while ae != (*a).head {
        t = itos(element_index!(ae));
        list = make_word_list(make_bare_word(t), list);
        free(t as *mut libc::c_void);
        element_forw!(ae);
    }
    return  REVERSE_LIST!(list, *mut WORD_LIST);
}

#[no_mangle]
pub unsafe extern "C" fn array_to_argv(
    mut a: *mut ARRAY,
    mut countp: *mut libc::c_int,
) -> *mut *mut libc::c_char {
    let mut ret: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut ae: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    if a.is_null() || array_empty!(a) {
        if !countp.is_null() {
            *countp = 0 as libc::c_int;
        }
        return 0 as *mut libc::c_void as *mut *mut libc::c_char;
    }
    ret = strvec_create(array_num_elements!(a) + 1 as libc::c_int);
    i = 0 as libc::c_int;
    ae = element_forw!((*a).head);
    while ae != (*a).head {
            t = element_value!(ae);
            if !t.is_null() {
                *ret.offset(i as isize) = savestring!(t);
                i = i + 1;
            }
            ae = element_forw!(ae);
        }
         *ret.offset(i as isize) 
            = 0 as *mut libc::c_void as *mut libc::c_char;
        if !countp.is_null() {
            *countp = i;
        }
        return ret;
}

unsafe extern "C" fn array_to_string_internal(
    mut start: *mut ARRAY_ELEMENT,
    mut end: *mut ARRAY_ELEMENT,
    mut sep: *mut libc::c_char,
    mut quoted: libc::c_int,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ae: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    let mut slen: libc::c_int = 0;
    let mut rsize: libc::c_int = 0;
    let mut rlen: libc::c_int = 0;
    let mut reg: libc::c_int = 0;
    if start == end {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    slen = strlen(sep) as libc::c_int;
    result = 0 as *mut libc::c_char;
    rlen = 0 as libc::c_int;
    rsize = rlen;
    ae = start;
    while ae != end {
        if rsize == 0 as libc::c_int {
            rsize = 64 as libc::c_int;
            result = malloc(rsize as usize) as *mut libc::c_char;
        }
        if !element_value!(ae).is_null() {
            t = if quoted != 0 { quote_string(element_value!(ae)) } else { element_value!(ae) };
            reg = strlen(t) as libc::c_int;

            RESIZE_MALLOCED_BUFFER!(result, rlen, (reg + slen + 2 as libc::c_int),
            rsize , rsize );

            strcpy(result.offset(rlen as isize), t);
            rlen += reg;
            if quoted != 0 {
                free(t as *mut libc::c_void);
            }
            if element_forw!(ae) != end {
                strcpy(result.offset(rlen as isize), sep);
                rlen += slen;
            }
        }
        ae = element_forw!(ae);
    }
    if !result.is_null() {
        *result.offset(rlen as isize) = '\0' as i32 as libc::c_char;
    }
    return result;
}

#[no_mangle]
pub unsafe extern "C" fn array_to_kvpair(
    mut a: *mut ARRAY,
    mut quoted: libc::c_int,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut valstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut is: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut indstr: [libc::c_char; 22] = [0; 22];
    let mut ae: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    let mut rsize: libc::c_int = 0;
    let mut rlen: libc::c_int = 0;
    let mut elen: libc::c_int = 0;

    if a.is_null() || array_empty!(a){
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    rsize = 128 as libc::c_int;
    result = malloc(rsize as usize) as *mut libc::c_char;
    rlen = 0 as libc::c_int;
    *result.offset(rlen as isize) = '\0' as i32 as libc::c_char;
    ae = element_forw!((*a).head);
    while ae != (*a).head {
        is = inttostr(
            (*ae).ind,
            indstr.as_mut_ptr(),
            std::mem::size_of::<[libc::c_char; 22]>() as usize,
        );
        valstr = if !element_value!(ae).is_null() {
            if ansic_shouldquote(element_value!(ae)) != 0 {
                ansic_quote(element_value!(ae), 0 as libc::c_int, 0 as *mut libc::c_int)
            } else {
                sh_double_quote(element_value!(ae))
            }
        } else {
            0 as *mut libc::c_void as *mut libc::c_char
        };
        elen = (STRLEN!(is) + 8  + STRLEN!(valstr)) as libc::c_int;
        RESIZE_MALLOCED_BUFFER!(result, rlen, (elen + 1), rsize, rsize);
            
        strcpy(result.offset(rlen as isize), is);
        rlen += STRLEN!(is) as libc::c_int;
        *result.offset(rlen as isize) = ' ' as i32 as libc::c_char;
        rlen = rlen + 1;
        if !valstr.is_null() {
            strcpy(result.offset(rlen as isize), valstr);
            rlen +=  STRLEN!(valstr) as libc::c_int;
        } 
        else {
            strcpy(
                result.offset(rlen as isize),
                b"\"\"\0" as *const u8 as *const libc::c_char,
            );
            rlen += 2 as libc::c_int;
        }
        if element_forw!(ae) != (*a).head {
            *result.offset(rlen as isize) = ' ' as i32 as libc::c_char;
            rlen = rlen + 1;
        }
        if !valstr.is_null() {
            free(valstr as *mut libc::c_void);
        }
        valstr = 0 as *mut libc::c_char;
        ae = element_forw!(ae);
    }

    RESIZE_MALLOCED_BUFFER!(result, rlen, 1, rsize, 8);
    *result.offset(rlen as isize) = '\0' as i32 as libc::c_char;   
        
    if quoted != 0 {
        valstr = sh_single_quote(result);
        free(result as *mut libc::c_void);
        result = valstr;
    }
    return result;
}
    
#[no_mangle]
pub unsafe extern "C" fn array_to_assign(
    mut a: *mut ARRAY,
    mut quoted: libc::c_int,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut valstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut is: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut indstr: [libc::c_char; 22] = [0; 22];
    let mut ae: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    let mut rsize: libc::c_int = 0;
    let mut rlen: libc::c_int = 0;
    let mut elen: libc::c_int = 0;
    if a.is_null() || array_empty!(a) {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }

    rsize = 128 as libc::c_int;
    result = malloc(rsize as usize) as *mut libc::c_char;
    *result.offset(0 as libc::c_int as isize) = '(' as i32 as libc::c_char;
    rlen = 1 as libc::c_int;
    ae =  element_forw!((*a).head);

    while ae != (*a).head {
        is = inttostr(element_index!(ae),
            indstr.as_mut_ptr(),
            std::mem::size_of::<[libc::c_char; 22]>() as usize
        );
        valstr = if !element_value!(ae).is_null() {
            if ansic_shouldquote(element_value!(ae)) != 0 {
                ansic_quote(element_value!(ae), 0 as libc::c_int, 0 as *mut libc::c_int)
            } else {
                sh_double_quote(element_value!(ae))
            }
        } else {
            0 as *mut libc::c_void as *mut libc::c_char
        };
        elen = (STRLEN!(is) + 8 + STRLEN!(valstr)) as libc::c_int;
        RESIZE_MALLOCED_BUFFER!(result, rlen, (elen + 1), rsize, rsize);
        *result.offset(rlen as isize) = '[' as i32 as libc::c_char;
        rlen = rlen + 1;
        strcpy(result.offset(rlen as isize), is);
        rlen += STRLEN!(is) as libc::c_int ;

        *result.offset(rlen as isize) = ']' as i32 as libc::c_char;
        rlen = rlen + 1;
        *result.offset(rlen as isize) = '=' as i32 as libc::c_char;
        rlen = rlen + 1;
        if !valstr.is_null() {
            strcpy(result.offset(rlen as isize), valstr);
            rlen += STRLEN!(valstr) as libc::c_int;
        }
        if element_forw!(ae)  != (*a).head {
            *result.offset(rlen as isize) = ' ' as i32 as libc::c_char;
            rlen = rlen + 1;
        }
        if !valstr.is_null() {
            free(valstr as *mut libc::c_void);
        }
        valstr = 0 as *mut libc::c_char;
        ae = (*ae).next;
    }
    RESIZE_MALLOCED_BUFFER!(result, rlen, 1, rsize, 8);

    *result.offset(rlen as isize) = ')' as i32 as libc::c_char;
    rlen = rlen + 1;
    *result.offset(rlen as isize) = '\0' as i32 as libc::c_char;
    if quoted != 0 {
        valstr = sh_single_quote(result);
        free(result as *mut libc::c_void);
        result = valstr;
    }
    return result;
}

#[no_mangle]
pub unsafe extern "C" fn array_to_string(
    mut a: *mut ARRAY,
    mut sep: *mut libc::c_char,
    mut quoted: libc::c_int,
) -> *mut libc::c_char {
    if a.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    if array_empty!(a) {
        return savestring!(b"\0" as *const u8 as *const libc::c_char);
    }
    return array_to_string_internal(element_forw!((*a).head), (*a).head, sep, quoted);
}
