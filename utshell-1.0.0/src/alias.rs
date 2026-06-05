use crate::hashlib::{
    hash_create, hash_dispose, hash_flush, hash_insert, hash_remove, hash_search,
};
use crate::pcomplete::{it_aliases, set_itemlist_dirty};

use crate::src_common::*;
use crate::y_tab::clear_string_list_expander;

// Global state - accessed through helper functions
static mut aliases: *mut HASH_TABLE = std::ptr::null_mut();
static mut command_word: libc::c_int = 0;

/// Helper function to safely check if aliases is null
#[inline]
pub fn aliases_is_null() -> bool {
    unsafe { aliases.is_null() }
}

/// Helper function to safely get aliases pointer
#[inline]
fn get_aliases() -> *mut HASH_TABLE {
    unsafe { aliases }
}

/// Helper function to safely set aliases pointer
#[inline]
fn set_aliases(value: *mut HASH_TABLE) {
    unsafe { aliases = value };
}

/// Helper function to safely get nentries from aliases
#[inline]
fn get_aliases_nentries() -> libc::c_int {
    unsafe {
        if aliases.is_null() {
            0
        } else {
            (*aliases).nentries
        }
    }
}

/// Helper function to safely get nbuckets from aliases
#[inline]
fn get_aliases_nbuckets() -> libc::c_int {
    unsafe {
        if aliases.is_null() {
            0
        } else {
            (*aliases).nbuckets
        }
    }
}

/// Helper function to safely get bucket array element
#[inline]
fn get_bucket_element(bucket_idx: libc::c_int) -> *mut BUCKET_CONTENTS {
    unsafe {
        if aliases.is_null() || bucket_idx >= (*aliases).nbuckets {
            std::ptr::null_mut()
        } else {
            *((*aliases).bucket_array).offset(bucket_idx as isize)
        }
    }
}

/// Helper to safely get command_word
#[inline]
fn get_command_word() -> libc::c_int {
    unsafe { command_word }
}

/// Helper to safely set command_word
#[inline]
fn set_command_word(value: libc::c_int) {
    unsafe { command_word = value };
}

/// Helper to safely get alias name
#[inline]
fn get_alias_name(alias: *const alias_t) -> *mut libc::c_char {
    unsafe { (*alias).name }
}

/// Helper to safely get alias value
#[inline]
fn get_alias_value_ptr(alias: *const alias_t) -> *mut libc::c_char {
    unsafe { (*alias).value }
}

/// Helper to safely get alias flags
#[inline]
fn get_alias_flags(alias: *const alias_t) -> libc::c_char {
    unsafe { (*alias).flags }
}

/// Helper to safely set alias flags
#[inline]
fn set_alias_flags(alias: *mut alias_t, flags: libc::c_char) {
    unsafe { (*alias).flags = flags };
}

/// Helper to safely set alias value
#[inline]
fn set_alias_value(alias: *mut alias_t, value: *mut libc::c_char) {
    unsafe { (*alias).value = value };
}

/// Helper to safely get string char at offset
#[inline]
fn get_char_at(s: *const libc::c_char, offset: isize) -> libc::c_char {
    unsafe { *s.offset(offset) }
}

/// Helper to safely check if char at offset matches expected value
#[inline]
fn char_matches(s: *const libc::c_char, offset: isize, expected: i32) -> bool {
    unsafe { *s.offset(offset) as i32 == expected }
}

/// Helper to safely check if char at offset is whitespace
#[inline]
fn is_whitespace_at(s: *const libc::c_char, offset: isize) -> bool {
    unsafe {
        let c = *s.offset(offset) as i32;
        c == ' ' as i32 || c == '\t' as i32
    }
}

#[no_mangle]
pub fn initialize_aliases() {
    if aliases_is_null() {
        set_aliases(hash_create(64 as libc::c_int));
    }
}

#[no_mangle]
pub fn find_alias(name: *mut libc::c_char) -> *mut alias_t {
    if aliases_is_null() {
        return std::ptr::null_mut();
    }

    let al = hash_search(name, get_aliases(), 0 as libc::c_int);

    if al.is_null() {
        std::ptr::null_mut()
    } else {
        unsafe { (*al).data as *mut alias_t }
    }
}

#[no_mangle]
pub fn get_alias_value(name: *mut libc::c_char) -> *mut libc::c_char {
    if aliases_is_null() {
        return std::ptr::null_mut();
    }

    let alias = find_alias(name);

    if alias.is_null() {
        std::ptr::null_mut()
    } else {
        get_alias_value_ptr(alias)
    }
}

/// Helper to check if value ends with whitespace (for AL_EXPANDNEXT flag)
fn value_ends_with_whitespace(value: *const libc::c_char) -> bool {
    unsafe {
        let len = libc::strlen(value);
        if len == 0 {
            return false;
        }
        let last_char = *value.offset((len - 1) as isize) as i32;
        last_char == ' ' as i32 || last_char == '\t' as i32
    }
}

/// Helper to update alias flags based on value
fn update_alias_flags(alias: *mut alias_t, value: *const libc::c_char) {
    let current_flags = get_alias_flags(alias) as libc::c_int;
    let new_flags = current_flags & !(AL_EXPANDNEXT as libc::c_int);

    if !value.is_null() && value_ends_with_whitespace(value) {
        set_alias_flags(
            alias,
            (new_flags | AL_EXPANDNEXT as libc::c_int) as libc::c_char,
        );
    } else {
        set_alias_flags(alias, new_flags as libc::c_char);
    }
}

#[no_mangle]
pub fn add_alias(name: *mut libc::c_char, value: *mut libc::c_char) {
    let mut temp: *mut alias_t;

    if aliases_is_null() {
        initialize_aliases();
        temp = std::ptr::null_mut();
    } else {
        temp = find_alias(name);
    }

    unsafe {
        if !temp.is_null() {
            // Update existing alias
            let old_value = get_alias_value_ptr(temp);
            if !old_value.is_null() {
                libc::free(old_value as *mut libc::c_void);
            }
            set_alias_value(temp, savestring!(value));
            update_alias_flags(temp, value);
        } else {
            // Create new alias
            temp = libc::malloc(::core::mem::size_of::<alias_t>()) as *mut alias_t;
            (*temp).name = savestring!(name);
            (*temp).value = savestring!(value);
            (*temp).flags = 0 as libc::c_char;

            if !value.is_null() && value_ends_with_whitespace(value) {
                set_alias_flags(temp, AL_EXPANDNEXT as libc::c_char);
            }

            let elt = hash_insert(savestring!(name), get_aliases(), HASH_NOSRCH as libc::c_int);
            (*elt).data = temp as *mut libc::c_void;
            set_itemlist_dirty(&mut it_aliases);
        }
    }
}

fn free_alias_data(data: *mut libc::c_void) {
    if data.is_null() {
        return;
    }

    let a = data as *mut alias_t;

    unsafe {
        if get_alias_flags(a) as libc::c_int & 0x2 as libc::c_int != 0 {
            clear_string_list_expander(a);
        }
        let value = get_alias_value_ptr(a);
        let name = get_alias_name(a);
        if !value.is_null() {
            libc::free(value as *mut libc::c_void);
        }
        if !name.is_null() {
            libc::free(name as *mut libc::c_void);
        }
        libc::free(data);
    }
}

#[no_mangle]
pub(crate) fn remove_alias(name: *mut libc::c_char) -> libc::c_int {
    if aliases_is_null() {
        return -(1 as libc::c_int);
    }

    let elt = hash_remove(name, get_aliases(), 0 as libc::c_int);

    if elt.is_null() {
        return -(1 as libc::c_int);
    }

    unsafe {
        free_alias_data((*elt).data);
        libc::free((*elt).key as *mut libc::c_void);
        libc::free(elt as *mut libc::c_void);
        set_itemlist_dirty(&mut it_aliases);
        (*get_aliases()).nentries
    }
}

#[no_mangle]
pub fn delete_all_aliases() {
    if aliases_is_null() {
        return;
    }

    unsafe {
        hash_flush(get_aliases(), Some(free_alias_data));
        hash_dispose(get_aliases());
        set_aliases(std::ptr::null_mut());
        set_itemlist_dirty(&mut it_aliases);
    }
}

fn map_over_aliases(function: Option<sh_alias_map_func_t>) -> *mut *mut alias_t {
    let i = get_aliases_nentries();

    if i == 0 {
        return std::ptr::null_mut();
    }

    let list = unsafe {
        libc::malloc(
            ((i + 1) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut alias_t>() as libc::c_ulong)
                as usize,
        ) as *mut *mut alias_t
    };

    let mut list_index: libc::c_int = 0;
    let nbuckets = get_aliases_nbuckets();

    unsafe {
        let mut bucket_idx: libc::c_int = 0;
        while bucket_idx < nbuckets {
            let mut tlist = get_bucket_element(bucket_idx);

            while !tlist.is_null() {
                let alias = (*tlist).data as *mut alias_t;
                if function.is_none() || function.expect("non-null function pointer")(alias) != 0 {
                    *list.offset(list_index as isize) = alias;
                    list_index += 1;
                    *list.offset(list_index as isize) = std::ptr::null_mut();
                }
                tlist = (*tlist).next;
            }
            bucket_idx += 1;
        }
    }

    list
}

fn sort_aliases(array: *mut *mut alias_t) {
    unsafe {
        c_qsort(
            array as *mut libc::c_void,
            c_strvec_len(array as *mut *mut libc::c_char) as libc::size_t,
            ::core::mem::size_of::<*mut alias_t>() as usize,
            ::core::mem::transmute::<
                Option<fn(*mut *mut alias_t, *mut *mut alias_t) -> libc::c_int>,
                Option<QSFUNC>,
            >(Some(qsort_alias_compare)),
        );
    }
}

fn qsort_alias_compare(as1: *mut *mut alias_t, as2: *mut *mut alias_t) -> libc::c_int {
    unsafe {
        let name1 = get_alias_name(*as1);
        let name2 = get_alias_name(*as2);

        let mut result =
            get_char_at(name1, 0) as libc::c_int - get_char_at(name2, 0) as libc::c_int;

        if result == 0 {
            result = libc::strcmp(name1, name2);
        }

        result
    }
}

#[no_mangle]
pub fn all_aliases() -> *mut *mut alias_t {
    if aliases_is_null() || get_aliases_nentries() == 0 {
        return std::ptr::null_mut();
    }

    let list = map_over_aliases(None);

    if !list.is_null() {
        sort_aliases(list);
    }

    list
}

#[no_mangle]
pub fn alias_expand_word(s: *mut libc::c_char) -> *mut libc::c_char {
    let r = find_alias(s);

    if r.is_null() {
        std::ptr::null_mut()
    } else {
        unsafe { savestring!(get_alias_value_ptr(r)) }
    }
}

fn skipquotes(string: *mut libc::c_char, start: libc::c_int) -> libc::c_int {
    let delimiter = get_char_at(string, start as isize) as libc::c_int;
    let mut i = start + 1;

    while !char_matches(string, i as isize, 0) {
        if char_matches(string, i as isize, '\\' as i32) {
            i += 1;
            if char_matches(string, i as isize, 0) {
                break;
            }
        } else if char_matches(string, i as isize, delimiter) {
            return i;
        }
        i += 1;
    }

    i
}

fn skipws(string: *mut libc::c_char, _start: libc::c_int) -> libc::c_int {
    let mut pass_next: libc::c_int = 0;
    let mut backslash_quoted_word: libc::c_int = 0;
    let mut i: libc::c_int = 0;

    unsafe {
        while !char_matches(string, i as isize, 0) {
            if pass_next != 0 {
                pass_next = 0;
            } else if is_whitespace_at(string, i as isize) {
                backslash_quoted_word = 0;
            } else if char_matches(string, i as isize, '\\' as i32) {
                let peekc = get_char_at(string, (i + 1) as isize) as libc::c_uchar;
                if peekc as libc::c_int == 0 {
                    break;
                }
                if *(*c___ctype_b_loc()).offset(peekc as libc::c_int as isize) as libc::c_int
                    & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                    != 0
                {
                    backslash_quoted_word += 1;
                } else {
                    pass_next += 1;
                }
            } else if char_matches(string, i as isize, '\'' as i32)
                || char_matches(string, i as isize, '"' as i32)
            {
                i = skipquotes(string, i);
                if char_matches(string, i as isize, '\0' as i32) {
                    break;
                }
                let peekc = get_char_at(string, (i + 1) as isize) as libc::c_uchar;
                if *(*c___ctype_b_loc()).offset(peekc as libc::c_int as isize) as libc::c_int
                    & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                    != 0
                {
                    backslash_quoted_word += 1;
                }
            } else if backslash_quoted_word == 0 {
                let char_val = get_char_at(string, i as isize) as libc::c_int;
                if char_val != 0 {
                    if c_mbschr(b"\r\n;|&(\0" as *const u8 as *const libc::c_char, char_val)
                        .is_null()
                    {
                        break;
                    }
                } else {
                    break;
                }
                set_command_word(get_command_word() + 1);
            }
            i += 1;
        }
    }

    i
}

fn rd_token(string: *mut libc::c_char, start: libc::c_int) -> libc::c_int {
    let mut i = start;

    while !char_matches(string, i as isize, 0) {
        let char_val = get_char_at(string, i as isize) as libc::c_int;

        if is_whitespace_at(string, i as isize)
            || (char_val != 0
                && !c_mbschr(
                    b" \t\n\r;|&()\0" as *const u8 as *const libc::c_char,
                    char_val,
                )
                .is_null())
        {
            break;
        }

        if char_matches(string, i as isize, '\\' as i32) {
            i += 1;
            if char_matches(string, i as isize, 0) {
                break;
            }
        } else if char_matches(string, i as isize, '\'' as i32)
            || char_matches(string, i as isize, '"' as i32)
        {
            i = skipquotes(string, i);
            if char_matches(string, i as isize, '\0' as i32) {
                break;
            }
        }
        i += 1;
    }

    i
}

#[no_mangle]
pub fn alias_expand(string: *mut libc::c_char) -> *mut libc::c_char {
    let line_len = unsafe { (libc::strlen(string) + 1) as libc::c_int };
    let mut line = unsafe { libc::malloc(line_len as usize) as *mut libc::c_char };
    let token = unsafe { libc::malloc(line_len as usize) as *mut libc::c_char };

    let mut i: libc::c_int = 0;
    let mut line_len = line_len;
    let mut expand_next: libc::c_int = 0;

    set_command_word(1);

    unsafe {
        *line.offset(0) = 0 as libc::c_char;

        loop {
            *token.offset(0) = 0;
            let start = i;
            i = skipws(string, start);

            if start == i && char_matches(string, i as isize, '\0' as i32) {
                libc::free(token as *mut libc::c_void);
                return line;
            }

            let j = libc::strlen(line) as libc::c_int;
            let tl = i - start;
            RESIZE_MALLOCED_BUFFER!(line, j, (tl + 1), line_len, tl + 50);
            libc::strncpy(
                line.offset(j as isize),
                string.offset(start as isize),
                tl as usize,
            );
            *line.offset((j + tl) as isize) = '\0' as libc::c_char;

            let real_start = i;
            let char_val = get_char_at(string, i as isize) as libc::c_int;
            let is_command_sep = char_val != 0
                && !c_mbschr(b"\r\n;|&(\0" as *const u8 as *const libc::c_char, char_val).is_null();

            set_command_word((get_command_word() != 0 || is_command_sep) as libc::c_int);

            let expand_this_token = (get_command_word() != 0 || expand_next != 0) as libc::c_int;
            expand_next = 0;

            let start = i;
            i = rd_token(string, start);
            let mut tl = i - start;

            if tl == 0 && !char_matches(string, i as isize, '\0' as i32) {
                tl = 1;
                i += 1;
            }

            libc::strncpy(token, string.offset(start as isize), tl as usize);
            *token.offset(tl as isize) = '\0' as libc::c_char;

            if !c_mbschr(token, '\\' as i32).is_null() {
                // Don't expand if token contains backslash
            } else if !char_matches(token, 0, 0)
                && (expand_this_token != 0 || alias_expand_all != 0)
            {
                let alias = find_alias(token);
                if !alias.is_null() {
                    let v = get_alias_value_ptr(alias);
                    let vlen = libc::strlen(v) as libc::c_int;
                    let llen = libc::strlen(line) as libc::c_int;
                    RESIZE_MALLOCED_BUFFER!(line, llen, (vlen + 3), line_len, vlen + 50);
                    libc::strcpy(line.offset(llen as isize), v);

                    if expand_this_token != 0
                        && vlen != 0
                        && is_whitespace_at(v, (vlen - 1) as isize)
                        || alias_expand_all != 0
                    {
                        expand_next = 1;
                    }

                    set_command_word(0);
                    continue;
                }
            }

            // Copy the token as-is
            let llen = libc::strlen(line) as libc::c_int;
            let tlen = i - real_start;
            RESIZE_MALLOCED_BUFFER!(line, llen, (tlen + 1), line_len, llen + tlen + 50);
            libc::strncpy(
                line.offset(llen as isize),
                string.offset(real_start as isize),
                tlen as usize,
            );
            *line.offset((llen + tlen) as isize) = '\0' as libc::c_char;

            set_command_word(0);
        }
    }
}
