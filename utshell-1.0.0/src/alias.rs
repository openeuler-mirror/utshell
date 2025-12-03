/*
 * SPDX-FileCopyrightText: 2025 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: GPL-2.0-or-later
 */
use crate::hashlib::{
    hash_create, hash_dispose, hash_flush, hash_insert, hash_remove, hash_search,
};
use crate::pcomplete::{it_aliases, set_itemlist_dirty};
#[allow(dead_code)]
use crate::src_common::*;
use crate::y_tab::clear_string_list_expander;
#[no_mangle]
pub fn initialize_aliases() {
    unsafe {
        if aliases.is_null() {
            aliases = hash_create(64 as libc::c_int);
        }
    }
}
#[no_mangle]
pub fn find_alias(name: *mut libc::c_char) -> *mut alias_t {
    let al: *mut BUCKET_CONTENTS;
    unsafe {
        if aliases.is_null() {
            return 0 as *mut libc::c_void as *mut alias_t;
        }
        al = hash_search(name, aliases, 0 as libc::c_int);
        return if !al.is_null() {
            (*al).data as *mut alias_t
        } else {
            0 as *mut libc::c_void as *mut alias_t
        };
    }
}
#[no_mangle]
pub fn get_alias_value(name: *mut libc::c_char) -> *mut libc::c_char {
    let alias: *mut alias_t;
    if unsafe { aliases.is_null() } {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    alias = find_alias(name);
    return if !alias.is_null() {
        unsafe { (*alias).value }
    } else {
        0 as *mut libc::c_void as *mut libc::c_char
    };
}
#[no_mangle]
pub fn add_alias(name: *mut libc::c_char, value: *mut libc::c_char) {
    let elt: *mut BUCKET_CONTENTS;
    let mut temp: *mut alias_t;
    let n: libc::c_int;
    unsafe {
        if aliases.is_null() {
            initialize_aliases();
            temp = 0 as *mut libc::c_void as *mut alias_t;
        } else {
            temp = find_alias(name);
        }
        if !temp.is_null() {
            libc::free((*temp).value as *mut libc::c_void);
            (*temp).value = savestring!(value);

            (*temp).flags =
                ((*temp).flags as libc::c_int & !(AL_EXPANDNEXT as libc::c_int)) as libc::c_char;
            if *value.offset(0 as libc::c_int as isize) != 0 {
                n = *value.offset((libc::strlen(value) + 1) as isize) as libc::c_int;
                if n == ' ' as i32 || n == '\t' as i32 {
                    (*temp).flags = ((*temp).flags as libc::c_int | AL_EXPANDNEXT as libc::c_int)
                        as libc::c_char;
                }
            }
        } else {
            temp = libc::malloc(::core::mem::size_of::<alias_t>()) as *mut alias_t;
            (*temp).name = savestring!(name);
            (*temp).value = savestring!(value);
            (*temp).flags = 0 as libc::c_int as libc::c_char;
            if *value.offset(0 as libc::c_int as isize) != 0 {
                n = *value.offset((libc::strlen(value) - 1) as isize) as libc::c_int;
                if n == ' ' as i32 || n == '\t' as i32 {
                    (*temp).flags = ((*temp).flags as libc::c_int | AL_EXPANDNEXT as libc::c_int)
                        as libc::c_char;
                }
            }
            elt = hash_insert(savestring!(name), aliases, HASH_NOSRCH as libc::c_int);
            (*elt).data = temp as *mut libc::c_void;
            set_itemlist_dirty(&mut it_aliases);
        };
    }
}
fn free_alias_data(data: *mut libc::c_void) {
    let a: *mut alias_t;
    a = data as *mut alias_t;
    unsafe {
        if (*a).flags as libc::c_int & 0x2 as libc::c_int != 0 {
            clear_string_list_expander(a);
        }
        libc::free((*a).value as *mut libc::c_void);
        libc::free((*a).name as *mut libc::c_void);
        libc::free(data);
    }
}
#[no_mangle]
pub(crate) fn remove_alias(name: *mut libc::c_char) -> libc::c_int {
    let elt: *mut BUCKET_CONTENTS;
    unsafe {
        if aliases.is_null() {
            return -(1 as libc::c_int);
        }
        elt = hash_remove(name, aliases, 0 as libc::c_int);
        if !elt.is_null() {
            free_alias_data((*elt).data);
            libc::free((*elt).key as *mut libc::c_void);
            libc::free(elt as *mut libc::c_void);
            set_itemlist_dirty(&mut it_aliases);
            return (*aliases).nentries;
        }
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub fn delete_all_aliases() {
    unsafe {
        if aliases.is_null() {
            return;
        }
        hash_flush(
            aliases,
            Some(free_alias_data), /*
                                   Some(                ::core::mem::transmute::<
                                       unsafe extern "C" fn(*mut libc::c_void) -> (),
                                       sh_free_func_t
                                       >(free_alias_data)
                                   )

                                           ::core::mem::transmute::<
                                       Option::<unsafe extern "C" fn() -> ()>,
                                       Option::<sh_free_func_t>,
                                   >(
                                       Some(
                                           ::core::mem::transmute::<
                                               unsafe extern "C" fn(*mut libc::c_void) -> (),
                                               unsafe extern "C" fn() -> (),
                                           >(free_alias_data),
                                       ),
                                   ),
                                   */
        );
        hash_dispose(aliases);
        aliases = 0 as *mut libc::c_void as *mut HASH_TABLE;
        set_itemlist_dirty(&mut it_aliases);
    }
}
fn map_over_aliases(function: Option<sh_alias_map_func_t>) -> *mut *mut alias_t {
    let mut i: libc::c_int;
    let mut tlist: *mut BUCKET_CONTENTS;
    let mut alias: *mut alias_t;
    let list: *mut *mut alias_t;
    let mut list_index: libc::c_int;
    unsafe {
        i = if !aliases.is_null() {
            (*aliases).nentries
        } else {
            0 as libc::c_int
        };
    }
    if i == 0 as libc::c_int {
        return 0 as *mut libc::c_void as *mut *mut alias_t;
    }
    unsafe {
        list = libc::malloc(
            ((i + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut alias_t>() as libc::c_ulong)
                as usize,
        ) as *mut *mut alias_t;
    }
    list_index = 0 as libc::c_int;
    i = list_index;
    unsafe {
        while i < (*aliases).nbuckets {
            tlist = if !aliases.is_null() && i < (*aliases).nbuckets {
                *((*aliases).bucket_array).offset(i as isize)
            } else {
                0 as *mut libc::c_void as *mut BUCKET_CONTENTS
            };
            while !tlist.is_null() {
                alias = (*tlist).data as *mut alias_t;
                if function.is_none()
                    || (Some(function.expect("non-null function pointer")))
                        .expect("non-null function pointer")(alias)
                        != 0
                {
                    *list.offset(list_index as isize) = alias;
                    list_index = list_index + 1;
                    *list.offset(list_index as isize) = 0 as *mut libc::c_void as *mut alias_t;
                }
                tlist = (*tlist).next;
            }
            i += 1;
        }
    }
    return list;
}
fn sort_aliases(array: *mut *mut alias_t) {
    unsafe {
        qsort(
            array as *mut libc::c_void,
            strvec_len(array as *mut *mut libc::c_char) as libc::size_t,
            ::core::mem::size_of::<*mut alias_t>() as usize,
            ::core::mem::transmute::<
                Option<fn(*mut *mut alias_t, *mut *mut alias_t) -> libc::c_int>,
                Option<QSFUNC>,
            >(Some(qsort_alias_compare)),
        );
    }
}
fn qsort_alias_compare(as1: *mut *mut alias_t, as2: *mut *mut alias_t) -> libc::c_int {
    let mut result: libc::c_int;
    unsafe {
        result = *((**as1).name).offset(0 as libc::c_int as isize) as libc::c_int
            - *((**as2).name).offset(0 as libc::c_int as isize) as libc::c_int;
        if result == 0 as libc::c_int {
            result = libc::strcmp((**as1).name, (**as2).name);
        }
    }
    return result;
}
#[no_mangle]
pub fn all_aliases() -> *mut *mut alias_t {
    unsafe {
        let list: *mut *mut alias_t;

        if aliases.is_null()
            || (if !aliases.is_null() {
                (*aliases).nentries
            } else {
                0 as libc::c_int
            }) == 0 as libc::c_int
        {
            return 0 as *mut libc::c_void as *mut *mut alias_t;
        }
        list = map_over_aliases(::core::mem::transmute::<
            *mut libc::c_void,
            Option<sh_alias_map_func_t>,
        >(0 as *mut libc::c_void));
        if !list.is_null() {
            sort_aliases(list);
        }
        return list;
    }
}
#[no_mangle]
pub fn alias_expand_word(s: *mut libc::c_char) -> *mut libc::c_char {
    let r: *mut alias_t;
    r = find_alias(s);
    return if !r.is_null() {
        unsafe { savestring!((*r).value) }
    } else {
        0 as *mut libc::c_void as *mut libc::c_char
    };
}
static mut command_word: libc::c_int = 0;
fn skipquotes(string: *mut libc::c_char, start: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int;
    unsafe {
        let delimiter: libc::c_int = *string.offset(start as isize) as libc::c_int;

        i = start + 1 as libc::c_int;
        while *string.offset(i as isize) != 0 {
            if *string.offset(i as isize) as libc::c_int == '\\' as i32 {
                i += 1;

                if *string.offset(i as isize) as libc::c_int == 0 as libc::c_int {
                    break;
                }
            } else if *string.offset(i as isize) as libc::c_int == delimiter {
                return i;
            }
            i += 1;
        }
    }
    return i;
}
fn skipws(string: *mut libc::c_char, _start: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int;
    let mut pass_next: libc::c_int;
    let mut backslash_quoted_word: libc::c_int;
    let mut peekc: libc::c_uchar;
    unsafe {
        pass_next = 0 as libc::c_int;
        backslash_quoted_word = 0;
        i = 0;
        while *string.offset(i as isize) != 0 {
            if pass_next != 0 {
                pass_next = 0 as libc::c_int;
            } else if *string.offset(i as isize) as libc::c_int == ' ' as i32
                || *string.offset(i as isize) as libc::c_int == '\t' as i32
            {
                backslash_quoted_word = 0 as libc::c_int;
            } else if *string.offset(i as isize) as libc::c_int == '\\' as i32 {
                peekc = *string.offset((i + 1 as libc::c_int) as isize) as libc::c_uchar;
                if peekc as libc::c_int == 0 as libc::c_int {
                    break;
                }
                if 1 as libc::c_int != 0
                    && *(*__ctype_b_loc()).offset(peekc as libc::c_int as isize) as libc::c_int
                        & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                        != 0
                {
                    backslash_quoted_word += 1;
                } else {
                    pass_next += 1;
                }
            } else if *string.offset(i as isize) as libc::c_int == '\'' as i32
                || *string.offset(i as isize) as libc::c_int == '"' as i32
            {
                i = skipquotes(string, i);
                if *string.offset(i as isize) as libc::c_int == '\0' as i32 {
                    break;
                }
                peekc = *string.offset((i + 1 as libc::c_int) as isize) as libc::c_uchar;
                if 1 as libc::c_int != 0
                    && *(*__ctype_b_loc()).offset(peekc as libc::c_int as isize) as libc::c_int
                        & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                        != 0
                {
                    backslash_quoted_word += 1;
                }
            } else if !(backslash_quoted_word != 0) {
                if !(if *string.offset(i as isize) as libc::c_int != 0 {
                    (mbschr(
                        b"\r\n;|&(\0" as *const u8 as *const libc::c_char,
                        *string.offset(i as isize) as libc::c_int,
                    ) != 0 as *mut libc::c_void as *mut libc::c_char)
                        as libc::c_int
                } else {
                    0 as libc::c_int
                } != 0)
                {
                    break;
                }
                command_word += 1;
            }
            i += 1;
        }
    }
    return i;
}
fn rd_token(string: *mut libc::c_char, start: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int;
    unsafe {
        i = start;
        while *string.offset(i as isize) as libc::c_int != 0
            && !(*string.offset(i as isize) as libc::c_int == ' ' as i32
                || *string.offset(i as isize) as libc::c_int == '\t' as i32
                || (if *string.offset(i as isize) as libc::c_int != 0 {
                    unsafe {
                        (mbschr(
                            b" \t\n\r;|&()\0" as *const u8 as *const libc::c_char,
                            *string.offset(i as isize) as libc::c_int,
                        ) != 0 as *mut libc::c_void as *mut libc::c_char)
                            as libc::c_int
                    }
                } else {
                    0 as libc::c_int
                }) != 0)
        {
            if *string.offset(i as isize) as libc::c_int == '\\' as i32 {
                i += 1;

                if *string.offset(i as isize) as libc::c_int == 0 as libc::c_int {
                    break;
                }
            } else if *string.offset(i as isize) as libc::c_int == '\'' as i32
                || *string.offset(i as isize) as libc::c_int == '"' as i32
            {
                i = skipquotes(string, i);
                if *string.offset(i as isize) as libc::c_int == '\0' as i32 {
                    break;
                }
            }
            i += 1;
        }
    }
    return i;
}
#[no_mangle]
pub fn alias_expand(string: *mut libc::c_char) -> *mut libc::c_char {
    let mut i: libc::c_int;
    let mut j: libc::c_int;
    let mut start: libc::c_int;
    let mut line: *mut libc::c_char;
    let token: *mut libc::c_char;
    let mut line_len: libc::c_int;
    let mut tl: libc::c_int;
    let mut real_start: libc::c_int;
    let mut expand_next: libc::c_int;
    let mut expand_this_token: libc::c_int;
    let mut alias: *mut alias_t = 0 as *mut alias_t;
    unsafe {
        line_len = (libc::strlen(string) + 1) as libc::c_int;
        line = libc::malloc(line_len as usize) as *mut libc::c_char;
        token = libc::malloc(line_len as usize) as *mut libc::c_char;

        i = 0 as libc::c_int;
        *line.offset(0 as libc::c_int as isize) = i as libc::c_char;
        expand_next = 0 as libc::c_int;
        command_word = 1 as libc::c_int;
        loop {
            *token.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
            start = i;
            i = skipws(string, start);

            if start == i && *string.offset(i as isize) as libc::c_int == '\0' as i32 {
                libc::free(token as *mut libc::c_void);
                return line;
            }
            j = libc::strlen(line) as libc::c_int;
            tl = i - start;
            RESIZE_MALLOCED_BUFFER!(line, j, (tl + 1), line_len, tl + 50);
            libc::strncpy(
                line.offset(j as isize),
                string.offset(start as isize),
                tl as usize,
            );
            *line.offset((j + tl) as isize) = '\0' as i32 as libc::c_char;
            real_start = i;
            command_word = (command_word != 0
                || (if *string.offset(i as isize) as libc::c_int != 0 {
                    (mbschr(
                        b"\r\n;|&(\0" as *const u8 as *const libc::c_char,
                        *string.offset(i as isize) as libc::c_int,
                    ) != 0 as *mut libc::c_void as *mut libc::c_char)
                        as libc::c_int
                } else {
                    0 as libc::c_int
                }) != 0) as libc::c_int;
            expand_this_token = (command_word != 0 || expand_next != 0) as libc::c_int;
            expand_next = 0 as libc::c_int;

            start = i;
            i = rd_token(string, start);
            tl = i - start;
            if tl == 0 as libc::c_int && *string.offset(i as isize) as libc::c_int != '\0' as i32 {
                tl = 1 as libc::c_int;
                i += 1;
            }

            libc::strncpy(token, string.offset(start as isize), tl as usize);
            *token.offset(tl as isize) = '\0' as i32 as libc::c_char;
            if !(mbschr(token, '\\' as i32)).is_null() {
                expand_this_token = 0 as libc::c_int;
            }
            if *token.offset(0 as libc::c_int as isize) as libc::c_int != 0
                && (expand_this_token != 0 || alias_expand_all != 0)
                && {
                    alias = find_alias(token);
                    !alias.is_null()
                }
            {
                let v: *mut libc::c_char;
                let vlen: libc::c_int;
                let llen: libc::c_int;
                v = (*alias).value;
                vlen = libc::strlen(v) as libc::c_int;
                llen = libc::strlen(line) as libc::c_int;
                RESIZE_MALLOCED_BUFFER!(line, llen, (vlen + 3), line_len, vlen + 50);
                libc::strcpy(line.offset(llen as isize), v);
                if expand_this_token != 0
                    && vlen != 0
                    && (*v.offset((vlen - 1 as libc::c_int) as isize) as libc::c_int == ' ' as i32
                        || *v.offset((vlen - 1 as libc::c_int) as isize) as libc::c_int
                            == '\t' as i32)
                    || alias_expand_all != 0
                {
                    expand_next = 1 as libc::c_int;
                }
            } else {
                let llen_0: libc::c_int;
                let tlen: libc::c_int;
                llen_0 = libc::strlen(line) as libc::c_int;
                tlen = i - real_start;
                RESIZE_MALLOCED_BUFFER!(line, llen_0, (tlen + 1), line_len, llen_0 + tlen + 50);
                libc::strncpy(
                    line.offset(llen_0 as isize),
                    string.offset(real_start as isize),
                    tlen as usize,
                );
                *line.offset((llen_0 + tlen) as isize) = '\0' as i32 as libc::c_char;
            }
            command_word = 0 as libc::c_int;
        }
    }
}
