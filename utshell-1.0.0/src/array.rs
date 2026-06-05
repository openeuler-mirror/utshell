use crate::dispose_cmd::dispose_words;
use crate::make_cmd::{make_bare_word, make_word_list};
use crate::src_common::*;
use crate::subst::{
    dequote_escapes, dequote_string, pat_subst, quote_escapes, quote_string, remove_quoted_nulls,
    string_list_pos_params,
};

/// Helper function to check if array is null or empty
#[inline]
fn array_is_null_or_empty(a: *mut ARRAY) -> bool {
    unsafe { a.is_null() || array_empty!(a) }
}

/// Helper function to check if array head is valid
#[inline]
fn array_head_is_valid(array: *mut ARRAY) -> bool {
    unsafe { !array.is_null() && !array_head!(array).is_null() }
}

/// Helper function to get element forward
// #[inline]
// fn get_element_forw(ae: *mut ARRAY_ELEMENT) -> *mut ARRAY_ELEMENT {
//     unsafe { element_forw!(ae) }
// }

// /// Helper function to get element back
// #[inline]
// fn get_element_back(ae: *mut ARRAY_ELEMENT) -> *mut ARRAY_ELEMENT {
//     unsafe { element_back!(ae) }
// }

// /// Helper function to get element index
// #[inline]
// fn get_element_index(ae: *mut ARRAY_ELEMENT) -> arrayind_t {
//     unsafe { element_index!(ae) }
// }

// /// Helper function to get element value
// #[inline]
// fn get_element_value(ae: *mut ARRAY_ELEMENT) -> *mut libc::c_char {
//     unsafe { element_value!(ae) }
// }

#[no_mangle]
pub fn array_create() -> *mut ARRAY {
    unsafe {
        let r = libc::malloc(std::mem::size_of::<ARRAY>() as usize) as *mut ARRAY;
        (*r).type_0 = array_indexed;
        (*r).max_index = -(1 as libc::c_int) as arrayind_t;
        (*r).num_elements = 0 as libc::c_int;
        (*r).lastref = std::ptr::null_mut();
        let head = array_create_element(-(1 as libc::c_int) as arrayind_t, std::ptr::null_mut());
        (*head).next = head;
        (*head).prev = (*head).next;
        (*r).head = head;
        r
    }
}

#[no_mangle]
pub fn array_flush(a: *mut ARRAY) {
    if a.is_null() {
        return;
    }

    unsafe {
        let mut r = element_forw!((*a).head);
        while r != (*a).head {
            let r1 = element_forw!(r);
            array_dispose_element(r);
            r = r1;
        }
        (*(*a).head).prev = (*a).head;
        (*(*a).head).next = (*(*a).head).prev;
        (*a).max_index = -(1 as libc::c_int) as arrayind_t;
        (*a).num_elements = 0 as libc::c_int;
        (*a).lastref = std::ptr::null_mut();
    }
}

#[no_mangle]
pub fn array_dispose(a: *mut ARRAY) {
    if a.is_null() {
        return;
    }
    array_flush(a);
    unsafe {
        array_dispose_element((*a).head);
        libc::free(a as *mut libc::c_void);
    }
}

#[no_mangle]
pub fn array_copy(a: *mut ARRAY) -> *mut ARRAY {
    if a.is_null() {
        return std::ptr::null_mut();
    }

    let a1 = array_create();

    unsafe {
        (*a1).type_0 = (*a).type_0;
        (*a1).max_index = (*a).max_index;
        (*a1).num_elements = (*a).num_elements;
        let mut ae = element_forw!((*a).head);
        while ae != (*a).head {
            let new = array_create_element(element_index!(ae), element_value!(ae));
            ADD_BEFORE!((*a1).head, new);
            if ae == LASTREF!(a) {
                SET_LASTREF!(a1, new);
            }
            ae = element_forw!(ae);
        }
    }

    a1
}

#[no_mangle]
pub fn array_slice(array: *mut ARRAY, s: *mut ARRAY_ELEMENT, e: *mut ARRAY_ELEMENT) -> *mut ARRAY {
    unsafe {
        let a = array_create();
        (*a).type_0 = (*array).type_0;
        let mut mi: arrayind_t = 0;
        let mut p = s;
        let mut i: libc::c_int = 0;

        while p != e {
            let n = array_create_element(element_index!(p), element_value!(p));
            ADD_BEFORE!((*a).head, n);
            mi = element_index!(n);
            p = element_forw!(p);
            i += 1;
        }
        (*a).num_elements = i;
        (*a).max_index = mi;
        a
    }
}

#[no_mangle]
pub fn array_walk(a: *mut ARRAY, func: Option<sh_ae_map_func_t>, udata: *mut libc::c_void) {
    if array_is_null_or_empty(a) {
        return;
    }

    unsafe {
        let mut ae = element_forw!((*a).head);
        while ae != (*a).head {
            if (Some(func.expect("non-null function pointer"))).expect("non-null function pointer")(
                ae, udata,
            ) < 0
            {
                return;
            }
            ae = element_forw!(ae);
        }
    }
}

#[no_mangle]
pub fn array_shift(a: *mut ARRAY, n: libc::c_int, flags: libc::c_int) -> *mut ARRAY_ELEMENT {
    unsafe {
        if a.is_null() || array_empty!(a) || n <= 0 {
            return std::ptr::null_mut();
        }

        INVALIDATE_LASTREF!(a);

        let mut i: libc::c_int = 0;
        let mut ae = element_forw!((*a).head);
        let ret = ae;

        while ae != (*a).head && i < n {
            ae = element_forw!(ae);
            i += 1;
        }

        if ae == (*a).head {
            if flags & AS_DISPOSE as libc::c_int != 0 {
                array_flush(a);
                return std::ptr::null_mut();
            }
            ae = ret;
            while element_forw!(ae) != (*a).head {
                ae = element_forw!(ae);
            }
            element_forw!(ae) = std::ptr::null_mut();
            (*(*a).head).prev = (*a).head;
            (*(*a).head).next = (*(*a).head).prev;
            (*a).max_index = -(1 as libc::c_int) as arrayind_t;
            (*a).num_elements = 0;
            return ret;
        }

        (*(*ae).prev).next = std::ptr::null_mut();
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
                let next = element_forw!(ae);
                array_dispose_element(ae);
                ae = next;
            }
            return std::ptr::null_mut();
        }

        ret
    }
}

#[no_mangle]
pub fn array_rshift(a: *mut ARRAY, n: libc::c_int, s: *mut libc::c_char) -> libc::c_int {
    unsafe {
        if a.is_null() || array_empty!(a) && s.is_null() {
            return 0;
        } else if n <= 0 {
            return (*a).num_elements;
        }

        let mut ae = element_forw!((*a).head);

        if !s.is_null() {
            let new = array_create_element(0 as arrayind_t, s);
            ADD_BEFORE!(ae, new);
            (*a).num_elements += 1;
            if array_num_elements!(a) == 1 {
                (*a).max_index = 0;
                return 1;
            }
        }

        while ae != (*a).head {
            element_index!(ae) += n as libc::c_long;
            ae = element_forw!(ae);
        }

        (*a).max_index = (*(*(*a).head).prev).ind;
        INVALIDATE_LASTREF!(a);
        (*a).num_elements
    }
}

#[no_mangle]
pub fn array_unshift_element(a: *mut ARRAY) -> *mut ARRAY_ELEMENT {
    array_shift(a, 1, 0)
}

#[no_mangle]
pub fn array_shift_element(a: *mut ARRAY, v: *mut libc::c_char) -> libc::c_int {
    array_rshift(a, 1, v)
}

#[no_mangle]
pub fn array_quote(array: *mut ARRAY) -> *mut ARRAY {
    unsafe {
        if !array_head_is_valid(array) || array_empty!(array) {
            return std::ptr::null_mut();
        }

        let mut a = element_forw!((*array).head);
        while a != (*array).head {
            let t = quote_string((*a).value);
            if !((*a).value).is_null() {
                libc::free((*a).value as *mut libc::c_void);
            }
            (*a).value = t;
            a = element_forw!(a);
        }
    }
    array
}

#[no_mangle]
pub fn array_quote_escapes(array: *mut ARRAY) -> *mut ARRAY {
    unsafe {
        if !array_head_is_valid(array) || array_empty!(array) {
            return std::ptr::null_mut();
        }

        let mut a = element_forw!((*array).head);
        while a != (*array).head {
            let t = quote_escapes((*a).value);
            if !((*a).value).is_null() {
                libc::free((*a).value as *mut libc::c_void);
            }
            (*a).value = t;
            a = element_forw!(a);
        }
    }
    array
}

#[no_mangle]
pub fn array_dequote(array: *mut ARRAY) -> *mut ARRAY {
    unsafe {
        if !array_head_is_valid(array) || array_empty!(array) {
            return std::ptr::null_mut();
        }

        let mut a = element_forw!((*array).head);
        while a != (*array).head {
            let t = dequote_string((*a).value);
            if !((*a).value).is_null() {
                libc::free((*a).value as *mut libc::c_void);
            }
            (*a).value = t;
            a = element_forw!(a);
        }
    }
    array
}

#[no_mangle]
pub fn array_dequote_escapes(array: *mut ARRAY) -> *mut ARRAY {
    unsafe {
        if !array_head_is_valid(array) || array_empty!(array) {
            return std::ptr::null_mut();
        }

        let mut a = element_forw!((*array).head);
        while a != (*array).head {
            let t = dequote_escapes((*a).value);
            if !((*a).value).is_null() {
                libc::free((*a).value as *mut libc::c_void);
            }
            (*a).value = t;
            a = element_forw!(a);
        }
    }
    array
}

#[no_mangle]
pub fn array_remove_quoted_nulls(array: *mut ARRAY) -> *mut ARRAY {
    unsafe {
        if !array_head_is_valid(array) || array_empty!(array) {
            return std::ptr::null_mut();
        }

        let mut a = element_forw!((*array).head);
        while a != (*array).head {
            (*a).value = remove_quoted_nulls((*a).value);
            a = element_forw!(a);
        }
    }
    array
}

#[no_mangle]
pub fn array_subrange(
    a: *mut ARRAY,
    start: arrayind_t,
    nelem: arrayind_t,
    starsub: libc::c_int,
    quoted: libc::c_int,
    pflags: libc::c_int,
) -> *mut libc::c_char {
    let (h, p) = unsafe {
        let p = if !a.is_null() {
            array_head!(a)
        } else {
            std::ptr::null_mut()
        };

        if p.is_null() || array_empty!(a) || start > array_max_index!(a) {
            return std::ptr::null_mut();
        }

        let mut p = element_forw!(p);
        while p != array_head!(a) && start > element_index!(p) {
            p = element_forw!(p);
        }

        if p == (*a).head {
            return std::ptr::null_mut();
        }

        let mut i: arrayind_t = 0;
        let h = p;
        while p != (*a).head && i < nelem {
            i += 1;
            p = element_forw!(p);
        }
        (h, p)
    };

    let a2 = array_slice(a, h, p);
    let wl = array_to_word_list(a2);
    array_dispose(a2);

    if wl.is_null() {
        return std::ptr::null_mut();
    }

    let t = string_list_pos_params(
        if starsub != 0 { '*' as i32 } else { '@' as i32 },
        wl,
        quoted,
        pflags,
    );
    dispose_words(wl);
    t
}

#[no_mangle]
pub fn array_patsub(
    a: *mut ARRAY,
    pat: *mut libc::c_char,
    rep: *mut libc::c_char,
    mflags: libc::c_int,
) -> *mut libc::c_char {
    if !array_head_is_valid(a) || unsafe { array_empty!(a) } {
        return std::ptr::null_mut();
    }

    let wl = array_to_word_list(a);
    if wl.is_null() {
        return std::ptr::null_mut();
    }

    let save = wl;

    unsafe {
        let mut current = wl;
        while !current.is_null() {
            let t = pat_subst((*(*current).word).word, pat, rep, mflags);
            if !((*(*current).word).word).is_null() {
                libc::free((*(*current).word).word as *mut libc::c_void);
            }
            (*(*current).word).word = t;
            current = (*current).next;
        }
    }

    let pchar = if mflags & MATCH_STARSUB as libc::c_int == MATCH_STARSUB as libc::c_int {
        '*' as i32
    } else {
        '@' as i32
    };

    let qflags = if mflags & MATCH_QUOTED as libc::c_int == MATCH_QUOTED as libc::c_int {
        0x1
    } else {
        0
    };

    let pflags = if mflags & MATCH_ASSIGNRHS as libc::c_int != 0 {
        PF_ASSIGNRHS as libc::c_int
    } else {
        0
    };

    let t = string_list_pos_params(pchar, save, qflags, pflags);
    dispose_words(save);
    t
}

#[no_mangle]
pub fn array_modcase(
    a: *mut ARRAY,
    pat: *mut libc::c_char,
    modop: libc::c_int,
    mflags: libc::c_int,
) -> *mut libc::c_char {
    if !array_head_is_valid(a) || unsafe { array_empty!(a) } {
        return std::ptr::null_mut();
    }

    let wl = array_to_word_list(a);
    if wl.is_null() {
        return std::ptr::null_mut();
    }

    let save = wl;

    unsafe {
        let mut current = wl;
        while !current.is_null() {
            let t = c_sh_modcase((*(*current).word).word, pat, modop);
            if !((*(*current).word).word).is_null() {
                libc::free((*(*current).word).word as *mut libc::c_void);
            }
            (*(*current).word).word = t;
            current = (*current).next;
        }
    }

    let pchar = if mflags & MATCH_STARSUB as libc::c_int == MATCH_STARSUB as libc::c_int {
        '*' as i32
    } else {
        '@' as i32
    };

    let qflags: libc::c_int = if mflags & MATCH_QUOTED as libc::c_int == MATCH_QUOTED as libc::c_int
    {
        Q_DOUBLE_QUOTES as libc::c_int
    } else {
        0
    };

    let pflags = if mflags & MATCH_ASSIGNRHS as libc::c_int != 0 {
        PF_ASSIGNRHS as libc::c_int
    } else {
        0
    };

    let t = string_list_pos_params(pchar, save, qflags, pflags);
    dispose_words(save);
    t
}

#[no_mangle]
pub fn array_create_element(indx: arrayind_t, value: *mut libc::c_char) -> *mut ARRAY_ELEMENT {
    unsafe {
        let r = libc::malloc(std::mem::size_of::<ARRAY_ELEMENT>() as usize) as *mut ARRAY_ELEMENT;
        (*r).ind = indx;
        (*r).value = if !value.is_null() {
            savestring!(value)
        } else {
            std::ptr::null_mut()
        };
        (*r).prev = std::ptr::null_mut();
        (*r).next = (*r).prev;
        r
    }
}

#[no_mangle]
pub fn array_dispose_element(ae: *mut ARRAY_ELEMENT) {
    if ae.is_null() {
        return;
    }

    unsafe {
        if !((*ae).value).is_null() {
            libc::free((*ae).value as *mut libc::c_void);
        }
        libc::free(ae as *mut libc::c_void);
    }
}

#[no_mangle]
pub fn array_insert(a: *mut ARRAY, i: arrayind_t, v: *mut libc::c_char) -> libc::c_int {
    if a.is_null() {
        return -1;
    }

    unsafe {
        let new = array_create_element(i, v);

        if i > array_max_index!(a) {
            ADD_BEFORE!((*a).head, new);
            (*a).max_index = i;
            (*a).num_elements += 1;
            SET_LASTREF!(a, new);
            return 0;
        } else if i < array_first_index!(a) {
            ADD_AFTER!((*a).head, new);
            (*a).num_elements += 1;
            SET_LASTREF!(a, new);
            return 0;
        }

        let start = LASTREF!(a);
        let startind = element_index!(start);
        let direction: libc::c_int;

        if i < startind / 2 {
            let start = element_forw!((*a).head);
            // startind = element_index!(start);
            element_index!(start);
            direction = 1;
        } else if i >= startind {
            direction = 1;
        } else {
            direction = -1;
        }

        let mut ae = start;
        while ae != (*a).head {
            if element_index!(ae) == i {
                libc::free(element_value!(ae) as *mut libc::c_void);
                (*ae).value = (*new).value;
                (*new).value = std::ptr::null_mut();
                array_dispose_element(new);
                SET_LASTREF!(a, ae);
                return 0;
            } else if direction == 1 && (*ae).ind > i {
                ADD_BEFORE!(ae, new);
                (*a).num_elements += 1;
                SET_LASTREF!(a, new);
                return 0;
            } else if direction == -1 && (*ae).ind < i {
                ADD_AFTER!(ae, new);
                (*a).num_elements += 1;
                (*a).lastref = new;
                SET_LASTREF!(a, new);
                return 0;
            }
            ae = if direction == 1 {
                element_forw!(ae)
            } else {
                element_back!(ae)
            };
        }

        array_dispose_element(new);
        INVALIDATE_LASTREF!(a);
    }

    -1
}

#[no_mangle]
pub fn array_remove(a: *mut ARRAY, i: arrayind_t) -> *mut ARRAY_ELEMENT {
    unsafe {
        if a.is_null() || array_empty!(a) {
            return std::ptr::null_mut();
        }

        if i > array_max_index!(a) || i < array_first_index!(a) {
            return std::ptr::null_mut();
        }

        let start = LASTREF!(a);
        let startind = element_index!(start);
        let direction: libc::c_int;

        if i < startind / 2 {
            let start = element_forw!((*a).head);
            // startind = element_index!(start);
            element_index!(start);
            direction = 1;
        } else if i >= startind {
            direction = 1;
        } else {
            direction = -1;
        }

        let mut ae = start;
        while ae != (*a).head {
            if element_index!(ae) == i {
                (*(*ae).next).prev = (*ae).prev;
                (*(*ae).prev).next = (*ae).next;
                (*a).num_elements -= 1;

                if i == array_max_index!(a) {
                    (*a).max_index = element_index!((*ae).prev);
                }

                if (*ae).next != (*a).head {
                    SET_LASTREF!(a, (*ae).next);
                } else if (*ae).prev != (*a).head {
                    SET_LASTREF!(a, (*ae).prev);
                } else {
                    INVALIDATE_LASTREF!(a);
                }
                return ae;
            }

            ae = if direction == 1 {
                element_forw!(ae)
            } else {
                element_back!(ae)
            };

            if direction == 1 && element_index!(ae) > i {
                break;
            }
            if direction == -1 && element_index!(ae) < i {
                break;
            }
        }
    }

    std::ptr::null_mut()
}

#[no_mangle]
pub fn array_reference(a: *mut ARRAY, i: arrayind_t) -> *mut libc::c_char {
    unsafe {
        if a.is_null() || array_empty!(a) {
            return std::ptr::null_mut();
        }

        if i > array_max_index!(a) || i < array_first_index!(a) {
            return std::ptr::null_mut();
        }

        let start = LASTREF!(a);
        let startind = element_index!(start);
        let direction: libc::c_int;

        if i < startind / 2 {
            let start = element_forw!((*a).head);
            // startind = element_index!(start);
            element_index!(start);
            direction = 1;
        } else if i >= startind {
            direction = 1;
        } else {
            direction = -1;
        }

        let mut ae = start;
        let mut found_start: *mut ARRAY_ELEMENT = std::ptr::null_mut();

        while ae != (*a).head {
            if element_index!(ae) == i {
                SET_LASTREF!(a, ae);
                return element_value!(ae);
            }

            ae = if direction == 1 {
                element_forw!(ae)
            } else {
                element_back!(ae)
            };

            if direction == 1 && (*ae).ind > i {
                found_start = ae;
                break;
            } else if direction == -1 && (*ae).ind < i {
                found_start = ae;
                break;
            }
        }

        if !found_start.is_null() {
            SET_LASTREF!(a, found_start);
        }
    }

    std::ptr::null_mut()
}

#[no_mangle]
pub fn array_to_word_list(a: *mut ARRAY) -> *mut WORD_LIST {
    unsafe {
        if a.is_null() || (*a).num_elements == 0 {
            return std::ptr::null_mut();
        }

        let mut list: *mut WORD_LIST = std::ptr::null_mut();
        let mut ae = element_forw!((*a).head);

        while ae != (*a).head {
            list = make_word_list(make_bare_word(element_value!(ae)), list);
            ae = element_forw!(ae);
        }

        REVERSE_LIST!(list, *mut WORD_LIST)
    }
}

#[no_mangle]
pub fn array_assign_list(array: *mut ARRAY, list: *mut WORD_LIST) -> *mut ARRAY {
    let mut l = list;
    let i: arrayind_t = 0;

    unsafe {
        while !l.is_null() {
            array_insert(array, i, (*(*l).word).word);
            l = (*l).next;
        }
    }

    array
}

#[no_mangle]
pub fn array_from_word_list(list: *mut WORD_LIST) -> *mut ARRAY {
    if list.is_null() {
        return std::ptr::null_mut();
    }
    let a = array_create();
    array_assign_list(a, list)
}

#[no_mangle]
pub fn array_keys_to_word_list(a: *mut ARRAY) -> *mut WORD_LIST {
    unsafe {
        if a.is_null() || array_empty!(a) {
            return std::ptr::null_mut();
        }

        let mut list: *mut WORD_LIST = std::ptr::null_mut();
        let mut ae = element_forw!((*a).head);

        while ae != (*a).head {
            let t = c_itos(element_index!(ae));
            list = make_word_list(make_bare_word(t), list);
            libc::free(t as *mut libc::c_void);
            ae = element_forw!(ae);
        }

        REVERSE_LIST!(list, *mut WORD_LIST)
    }
}

#[no_mangle]
pub fn array_to_argv(a: *mut ARRAY, countp: *mut libc::c_int) -> *mut *mut libc::c_char {
    unsafe {
        if a.is_null() || array_empty!(a) {
            if !countp.is_null() {
                *countp = 0;
            }
            return std::ptr::null_mut();
        }

        let ret = c_strvec_create(array_num_elements!(a) + 1);
        let mut i: libc::c_int = 0;
        let mut ae = element_forw!((*a).head);

        while ae != (*a).head {
            let t = element_value!(ae);
            if !t.is_null() {
                *ret.offset(i as isize) = savestring!(t);
                i += 1;
            }
            ae = element_forw!(ae);
        }

        *ret.offset(i as isize) = std::ptr::null_mut();
        if !countp.is_null() {
            *countp = i;
        }

        ret
    }
}

fn array_to_string_internal(
    start: *mut ARRAY_ELEMENT,
    end: *mut ARRAY_ELEMENT,
    sep: *mut libc::c_char,
    quoted: libc::c_int,
) -> *mut libc::c_char {
    if start == end {
        return std::ptr::null_mut();
    }

    let slen = unsafe { libc::strlen(sep) as libc::c_int };
    let mut result: *mut libc::c_char = std::ptr::null_mut();
    let mut rlen: libc::c_int = 0;
    let mut rsize: libc::c_int = 0;
    let mut ae = start;

    unsafe {
        while ae != end {
            if rsize == 0 {
                rsize = 64;
                result = libc::malloc(rsize as usize) as *mut libc::c_char;
            }

            if !element_value!(ae).is_null() {
                let t = if quoted != 0 {
                    quote_string(element_value!(ae))
                } else {
                    element_value!(ae)
                };
                let reg = libc::strlen(t) as libc::c_int;

                RESIZE_MALLOCED_BUFFER!(result, rlen, (reg + slen + 2), rsize, rsize);

                libc::strcpy(result.offset(rlen as isize), t);
                rlen += reg;

                if quoted != 0 {
                    libc::free(t as *mut libc::c_void);
                }

                if element_forw!(ae) != end {
                    libc::strcpy(result.offset(rlen as isize), sep);
                    rlen += slen;
                }
            }
            ae = element_forw!(ae);
        }
    }

    if !result.is_null() {
        unsafe {
            *result.offset(rlen as isize) = '\0' as i32 as libc::c_char;
        }
    }

    result
}

#[no_mangle]
pub fn array_to_kvpair(a: *mut ARRAY, quoted: libc::c_int) -> *mut libc::c_char {
    unsafe {
        if a.is_null() || array_empty!(a) {
            return std::ptr::null_mut();
        }

        let mut rsize: libc::c_int = 128;
        let mut result = libc::malloc(rsize as usize) as *mut libc::c_char;
        let mut rlen: libc::c_int = 0;
        *result.offset(rlen as isize) = '\0' as i32 as libc::c_char;

        let mut ae = element_forw!((*a).head);
        let mut indstr: [libc::c_char; 22] = [0; 22];

        while ae != (*a).head {
            let is = c_inttostr(
                (*ae).ind,
                indstr.as_mut_ptr(),
                std::mem::size_of::<[libc::c_char; 22]>() as u64,
            );

            let valstr = if !element_value!(ae).is_null() {
                if c_ansic_shouldquote(element_value!(ae)) != 0 {
                    c_ansic_quote(element_value!(ae), 0, std::ptr::null_mut())
                } else {
                    c_sh_double_quote(element_value!(ae))
                }
            } else {
                std::ptr::null_mut()
            };

            let elen = (STRLEN!(is) + 8 + STRLEN!(valstr)) as libc::c_int;
            RESIZE_MALLOCED_BUFFER!(result, rlen, (elen + 1), rsize, rsize);

            libc::strcpy(result.offset(rlen as isize), is);
            rlen += STRLEN!(is) as libc::c_int;
            *result.offset(rlen as isize) = ' ' as i32 as libc::c_char;
            rlen += 1;

            if !valstr.is_null() {
                libc::strcpy(result.offset(rlen as isize), valstr);
                rlen += STRLEN!(valstr) as libc::c_int;
            } else {
                libc::strcpy(
                    result.offset(rlen as isize),
                    b"\"\"\0" as *const u8 as *const libc::c_char,
                );
                rlen += 2;
            }

            if element_forw!(ae) != (*a).head {
                *result.offset(rlen as isize) = ' ' as i32 as libc::c_char;
                rlen += 1;
            }

            if !valstr.is_null() {
                libc::free(valstr as *mut libc::c_void);
            }

            ae = element_forw!(ae);
        }

        RESIZE_MALLOCED_BUFFER!(result, rlen, 1, rsize, 8);
        *result.offset(rlen as isize) = '\0' as i32 as libc::c_char;

        if quoted != 0 {
            let valstr = c_sh_single_quote(result);
            libc::free(result as *mut libc::c_void);
            result = valstr;
        }

        result
    }
}

#[no_mangle]
pub fn array_to_assign(a: *mut ARRAY, quoted: libc::c_int) -> *mut libc::c_char {
    unsafe {
        if a.is_null() || array_empty!(a) {
            return std::ptr::null_mut();
        }

        let mut rsize: libc::c_int = 128;
        let mut result = libc::malloc(rsize as usize) as *mut libc::c_char;
        *result.offset(0) = '(' as i32 as libc::c_char;
        let mut rlen: libc::c_int = 1;

        let mut ae = element_forw!((*a).head);
        let mut indstr: [libc::c_char; 22] = [0; 22];

        while ae != (*a).head {
            let is = c_inttostr(
                element_index!(ae),
                indstr.as_mut_ptr(),
                std::mem::size_of::<[libc::c_char; 22]>() as u64,
            );

            let valstr = if !element_value!(ae).is_null() {
                if c_ansic_shouldquote(element_value!(ae)) != 0 {
                    c_ansic_quote(element_value!(ae), 0, std::ptr::null_mut())
                } else {
                    c_sh_double_quote(element_value!(ae))
                }
            } else {
                std::ptr::null_mut()
            };

            let elen = (STRLEN!(is) + 8 + STRLEN!(valstr)) as libc::c_int;
            RESIZE_MALLOCED_BUFFER!(result, rlen, (elen + 1), rsize, rsize);

            *result.offset(rlen as isize) = '[' as i32 as libc::c_char;
            rlen += 1;
            libc::strcpy(result.offset(rlen as isize), is);
            rlen += STRLEN!(is) as libc::c_int;

            *result.offset(rlen as isize) = ']' as i32 as libc::c_char;
            rlen += 1;
            *result.offset(rlen as isize) = '=' as i32 as libc::c_char;
            rlen += 1;

            if !valstr.is_null() {
                libc::strcpy(result.offset(rlen as isize), valstr);
                rlen += STRLEN!(valstr) as libc::c_int;
            }

            if element_forw!(ae) != (*a).head {
                *result.offset(rlen as isize) = ' ' as i32 as libc::c_char;
                rlen += 1;
            }

            if !valstr.is_null() {
                libc::free(valstr as *mut libc::c_void);
            }

            ae = (*ae).next;
        }

        RESIZE_MALLOCED_BUFFER!(result, rlen, 1, rsize, 8);

        *result.offset(rlen as isize) = ')' as i32 as libc::c_char;
        rlen += 1;
        *result.offset(rlen as isize) = '\0' as i32 as libc::c_char;

        if quoted != 0 {
            let valstr = c_sh_single_quote(result);
            libc::free(result as *mut libc::c_void);
            result = valstr;
        }

        result
    }
}

#[no_mangle]
pub fn array_to_string(
    a: *mut ARRAY,
    sep: *mut libc::c_char,
    quoted: libc::c_int,
) -> *mut libc::c_char {
    if a.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        if array_empty!(a) {
            return savestring!(b"\0" as *const u8 as *const libc::c_char);
        }
        array_to_string_internal(element_forw!((*a).head), (*a).head, sep, quoted)
    }
}
