//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later


#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    // fn strncpy(
    //     _: *mut libc::c_char,
    //     _: *const libc::c_char,
    //     _: libc::c_ulong,
    // ) -> *mut libc::c_char;
    // fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    // fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    // fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: libc::size_t,
        __size: libc::size_t,
        __compar: __compar_fn_t,
    );
    // fn sh_xmalloc(
    //     _: size_t,
    //     _: *const libc::c_char,
    //     _: libc::c_int,
    // ) -> *mut libc::c_void;
    // fn sh_xrealloc(
    //     _: *mut libc::c_void,
    //     _: size_t,
    //     _: *const libc::c_char,
    //     _: libc::c_int,
    // ) -> *mut libc::c_void;
    // fn sh_xfree(_: *mut libc::c_void, _: *const libc::c_char, _: libc::c_int);
    fn mbschr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strvec_len(_: *mut *mut libc::c_char) -> libc::c_int;
    fn hash_create(_: libc::c_int) -> *mut HASH_TABLE;
    fn hash_flush(_: *mut HASH_TABLE, _: Option::<sh_free_func_t>);
    fn hash_dispose(_: *mut HASH_TABLE);
    fn hash_search(
        _: *const libc::c_char,
        _: *mut HASH_TABLE,
        _: libc::c_int,
    ) -> *mut BUCKET_CONTENTS;
    fn hash_insert(
        _: *mut libc::c_char,
        _: *mut HASH_TABLE,
        _: libc::c_int,
    ) -> *mut BUCKET_CONTENTS;
    fn hash_remove(
        _: *const libc::c_char,
        _: *mut HASH_TABLE,
        _: libc::c_int,
    ) -> *mut BUCKET_CONTENTS;
    fn clear_string_list_expander(_: *mut alias_t);
    static mut it_aliases: ITEMLIST;
    fn set_itemlist_dirty(_: *mut ITEMLIST);
}
// pub type size_t = libc::c_ulong;
// pub type C2RustUnnamed = libc::c_uint;
// pub const _ISalnum: C2RustUnnamed = 8;
// pub const _ISpunct: C2RustUnnamed = 4;
// pub const _IScntrl: C2RustUnnamed = 2;
// pub const _ISblank: C2RustUnnamed = 1;
// pub const _ISgraph: C2RustUnnamed = 32768;
// pub const _ISprint: C2RustUnnamed = 16384;
// pub const _ISspace: C2RustUnnamed = 8192;
// pub const _ISxdigit: C2RustUnnamed = 4096;
// pub const _ISdigit: C2RustUnnamed = 2048;
 pub const _ISalpha: libc::c_uint = 1024;
// pub const _ISlower: C2RustUnnamed = 512;
// pub const _ISupper: C2RustUnnamed = 256;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type sh_free_func_t = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type QSFUNC = unsafe extern "C" fn(
    *const libc::c_void,
    *const libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _list_of_strings {
    pub list: *mut *mut libc::c_char,
    pub list_size: libc::c_int,
    pub list_len: libc::c_int,
}
pub type STRINGLIST = _list_of_strings;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bucket_contents {
    pub next: *mut bucket_contents,
    pub key: *mut libc::c_char,
    pub data: *mut libc::c_void,
    pub khash: libc::c_uint,
    pub times_found: libc::c_int,
}
pub type BUCKET_CONTENTS = bucket_contents;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table {
    pub bucket_array: *mut *mut BUCKET_CONTENTS,
    pub nbuckets: libc::c_int,
    pub nentries: libc::c_int,
}
pub type HASH_TABLE = hash_table;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alias {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub flags: libc::c_char,
}
pub type alias_t = alias;
pub type ITEMLIST = _list_of_items;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _list_of_items {
    pub flags: libc::c_int,
    pub list_getter: Option::<unsafe extern "C" fn(*mut _list_of_items) -> libc::c_int>,
    pub slist: *mut STRINGLIST,
    pub genlist: *mut STRINGLIST,
    pub genindex: libc::c_int,
}
pub type sh_alias_map_func_t = unsafe extern "C" fn(*mut alias_t) -> libc::c_int;
#[no_mangle]
pub static mut alias_expand_all: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut aliases: *mut HASH_TABLE = 0 as *const libc::c_void as *mut libc::c_void
    as *mut HASH_TABLE;
#[no_mangle]
pub unsafe extern "C" fn initialize_aliases() {
    if aliases.is_null() {
        aliases = hash_create(64 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn find_alias(name: *mut libc::c_char) -> *mut alias_t {
    let al: *mut BUCKET_CONTENTS;
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
#[no_mangle]
pub unsafe extern "C" fn get_alias_value(
    name: *mut libc::c_char,
) -> *mut libc::c_char {
    let alias: *mut alias_t;
    if aliases.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    alias = find_alias(name);
    return if !alias.is_null() {
        (*alias).value
    } else {
        0 as *mut libc::c_void as *mut libc::c_char
    };
}
pub const AL_EXPANDNEXT:libc::c_int = 0x1;
pub const HASH_NOSRCH:libc::c_int = 0x1;
pub const AL_BEINGEXPANDED:libc::c_int = 0x2;


#[no_mangle]
pub unsafe extern "C" fn add_alias(
    name: *mut libc::c_char,
    value: *mut libc::c_char,
) {
    let elt: *mut BUCKET_CONTENTS ;
    let mut temp: *mut alias_t;
    let n: libc::c_int;
    if aliases.is_null() {
        initialize_aliases();
        temp = 0 as *mut libc::c_void as *mut alias_t;
    } else {
        temp = find_alias(name);
    }
    if !temp.is_null() {
        libc::free(
            (*temp).value as *mut libc::c_void
        );
        (*temp).value = r_shell::savestring!(value);

        (*temp)
            .flags = ((*temp).flags as libc::c_int & !(AL_EXPANDNEXT as libc::c_int))
            as libc::c_char;
        if *value.offset(0 as libc::c_int as isize) != 0 {
            n = *value
                .offset(
                    (libc::strlen(value)+1)
                        as isize,
                ) as libc::c_int;
            if n == ' ' as i32 || n == '\t' as i32 {
                (*temp)
                    .flags = ((*temp).flags as libc::c_int | AL_EXPANDNEXT as libc::c_int)
                    as libc::c_char;
            }
        }
    } else {
        temp = libc::malloc(
            ::core::mem::size_of::<alias_t>(),
        ) as *mut alias_t;
        (*temp)
            .name = r_shell::savestring!(
            name
        );
        (*temp)
            .value = r_shell::savestring!(
            value
        );
        (*temp).flags = 0 as libc::c_int as libc::c_char;
        if *value.offset(0 as libc::c_int as isize) != 0 {
            n = *value
                .offset(
                    (libc::strlen(value)-1)
                        as isize,
                ) as libc::c_int;
            if n == ' ' as i32 || n == '\t' as i32 {
                (*temp)
                    .flags = ((*temp).flags as libc::c_int | AL_EXPANDNEXT as libc::c_int)
                    as libc::c_char;
            }
        }
        elt = hash_insert(
            r_shell::savestring!(
                name
            ),
            aliases,
            HASH_NOSRCH as libc::c_int,
        );
        (*elt).data = temp as *mut libc::c_void;
        set_itemlist_dirty(&mut it_aliases);
    };
}
unsafe extern "C" fn free_alias_data(data: *mut libc::c_void) {
    let a: *mut alias_t;
    a = data as *mut alias_t;
    if (*a).flags as libc::c_int & 0x2 as libc::c_int != 0 {
        clear_string_list_expander(a);
    }
    libc::free(
        (*a).value as *mut libc::c_void
    );
    libc::free(
        (*a).name as *mut libc::c_void
    );
    libc::free(data);
}
#[no_mangle]
pub unsafe extern "C" fn remove_alias(name: *mut libc::c_char) -> libc::c_int {
    let elt: *mut BUCKET_CONTENTS;
    if aliases.is_null() {
        return -(1 as libc::c_int);
    }
    elt = hash_remove(name, aliases, 0 as libc::c_int);
    if !elt.is_null() {
        free_alias_data((*elt).data);
        libc::free(
            (*elt).key as *mut libc::c_void
        );
        libc::free(
            elt as *mut libc::c_void
        );
        set_itemlist_dirty(&mut it_aliases);
        return (*aliases).nentries;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn delete_all_aliases() {
    if aliases.is_null() {
        return;
    }
    hash_flush(
        aliases,
        Some(free_alias_data)
        /*
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
unsafe extern "C" fn map_over_aliases(
    function: Option::<sh_alias_map_func_t>,
) -> *mut *mut alias_t {
    let mut i: libc::c_int;
    let mut tlist: *mut BUCKET_CONTENTS;
    let mut alias: *mut alias_t;
    let list: *mut *mut alias_t;
    let mut list_index: libc::c_int;
    i = if !aliases.is_null() { (*aliases).nentries } else { 0 as libc::c_int };
    if i == 0 as libc::c_int {
        return 0 as *mut libc::c_void as *mut *mut alias_t;
    }
    list = libc::malloc(
        ((i + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut alias_t>() as libc::c_ulong) as usize
    ) as *mut *mut alias_t;
    list_index = 0 as libc::c_int;
    i = list_index;
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
                    .expect("non-null function pointer")(alias) != 0
            {
                *list.offset(list_index as isize) = alias;
                list_index = list_index + 1;
                *list.offset(list_index as isize) = 0 as *mut libc::c_void as *mut alias_t;
            }
            tlist = (*tlist).next;
        }
        i += 1;
    }
    return list;
}
unsafe extern "C" fn sort_aliases(array: *mut *mut alias_t) {
    qsort(
        array as *mut libc::c_void,
        strvec_len(array as *mut *mut libc::c_char) as libc::size_t,
        ::core::mem::size_of::<*mut alias_t>() as usize,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut *mut alias_t, *mut *mut alias_t) -> libc::c_int,
            >,
            Option::<QSFUNC>,
        >(
            Some(
                qsort_alias_compare
                    as unsafe extern "C" fn(
                        *mut *mut alias_t,
                        *mut *mut alias_t,
                    ) -> libc::c_int,
            ),
        ),
    );
}
unsafe extern "C" fn qsort_alias_compare(
    as1: *mut *mut alias_t,
    as2: *mut *mut alias_t,
) -> libc::c_int {
    let mut result: libc::c_int;
    result = *((**as1).name).offset(0 as libc::c_int as isize) as libc::c_int
        - *((**as2).name).offset(0 as libc::c_int as isize) as libc::c_int;
    if result == 0 as libc::c_int {
        result = libc::strcmp((**as1).name, (**as2).name);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn all_aliases() -> *mut *mut alias_t {
    let list: *mut *mut alias_t;
    if aliases.is_null()
        || (if !aliases.is_null() { (*aliases).nentries } else { 0 as libc::c_int })
            == 0 as libc::c_int
    {
        return 0 as *mut libc::c_void as *mut *mut alias_t;
    }
    list = map_over_aliases(
        ::core::mem::transmute::<
            *mut libc::c_void,
            Option::<sh_alias_map_func_t>,
        >(0 as *mut libc::c_void),
    );
    if !list.is_null() {
        sort_aliases(list);
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn alias_expand_word(
    s: *mut libc::c_char,
) -> *mut libc::c_char {
    let r: *mut alias_t;
    r = find_alias(s);
    return if !r.is_null() {
        r_shell::savestring!(
            (*r).value
        )
    } else {
        0 as *mut libc::c_void as *mut libc::c_char
    };
}
static mut command_word: libc::c_int = 0;
unsafe extern "C" fn skipquotes(
    string: *mut libc::c_char,
    start: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int;
    let delimiter: libc::c_int = *string.offset(start as isize) as libc::c_int;
    i = start + 1 as libc::c_int;
    while *string.offset(i as isize) != 0 {
        if *string.offset(i as isize) as libc::c_int == '\\' as i32 {
            i += 1;

            if *string.offset(i as isize) as libc::c_int == 0 as libc::c_int {
                break;
            }
        } else if *string.offset(i as isize) as libc::c_int == delimiter {
            return i
        }
        i += 1;
    }
    return i;
}
unsafe extern "C" fn skipws(
    string: *mut libc::c_char,
    _start: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int;
    let mut pass_next: libc::c_int;
    let mut backslash_quoted_word: libc::c_int;
    let mut peekc: libc::c_uchar;
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
                && *(*__ctype_b_loc()).offset(peekc as libc::c_int as isize)
                    as libc::c_int
                    & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
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
                && *(*__ctype_b_loc()).offset(peekc as libc::c_int as isize)
                    as libc::c_int
                    & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                backslash_quoted_word += 1;

            }
        } else if !(backslash_quoted_word != 0) {
            if !(if *string.offset(i as isize) as libc::c_int != 0 {
                (mbschr(
                    b"\r\n;|&(\0" as *const u8 as *const libc::c_char,
                    *string.offset(i as isize) as libc::c_int,
                ) != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int
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
    return i;
}


#[macro_export] 
macro_rules! RESIZE_MALLOCED_BUFFER {
    ($str1:expr, $cind:expr, $room:expr, $csize:expr, $sincr:expr) => {

        if $cind + ($room) >= $csize {
            while $cind + ($room as libc::c_int) >= $csize {
                $csize += $sincr as libc::c_int;
            }
            $str1 = libc::realloc(
                $str1 as *mut libc::c_void,
                $csize as usize,
            ) as *mut libc::c_char;
        }
    };
}
unsafe extern "C" fn rd_token(
    string: *mut libc::c_char,
    start: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int;
    i = start;
    while *string.offset(i as isize) as libc::c_int != 0
        && !(*string.offset(i as isize) as libc::c_int == ' ' as i32
            || *string.offset(i as isize) as libc::c_int == '\t' as i32
            || (if *string.offset(i as isize) as libc::c_int != 0 {
                (mbschr(
                    b" \t\n\r;|&()\0" as *const u8 as *const libc::c_char,
                    *string.offset(i as isize) as libc::c_int,
                ) != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int
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
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn alias_expand(
    string: *mut libc::c_char,
) -> *mut libc::c_char {
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
    line_len = (libc::strlen(string)+1)
        as libc::c_int;
    line = libc::malloc(
        line_len as usize
    ) as *mut libc::c_char;
    token = libc::malloc(
        line_len as usize
    ) as *mut libc::c_char;
    i = 0 as libc::c_int;
    *line.offset(0 as libc::c_int as isize) = i as libc::c_char;
    expand_next = 0 as libc::c_int;
    command_word = 1 as libc::c_int;
    loop {
        *token.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
        start = i;
        i = skipws(string, start);
        if start == i && *string.offset(i as isize) as libc::c_int == '\0' as i32 {
            libc::free(
                token as *mut libc::c_void
            );
            return line;
        }
        j = libc::strlen(line) as libc::c_int;
        tl = i - start;
        RESIZE_MALLOCED_BUFFER! (line, j, (tl + 1), line_len, (tl + 50));

        /*
        if j + (tl + 1 as libc::c_int) >= line_len {
            while j + (tl + 1 as libc::c_int) >= line_len {
                line_len += tl + 50 as libc::c_int;
            }
            line = sh_xrealloc(
                line as *mut libc::c_void,
                line_len as size_t,
                b"alias.c\0" as *const u8 as *const libc::c_char,
                519 as libc::c_int,
            ) as *mut libc::c_char;
        } */
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
                ) != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int
            } else {
                0 as libc::c_int
            }) != 0) as libc::c_int;
        expand_this_token = (command_word != 0 || expand_next != 0) as libc::c_int;
        expand_next = 0 as libc::c_int;
        start = i;
        i = rd_token(string, start);
        tl = i - start;
        if tl == 0 as libc::c_int
            && *string.offset(i as isize) as libc::c_int != '\0' as i32
        {
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
            /* v = unsafe{
            if (!alias.is_null()  && libc::strlen((*alias).value)!=0){(*alias).value} else {0 as *mut libc::c_char}
            };
            */
            v= (*alias).value;
            vlen = libc::strlen(v) as libc::c_int;
            llen = libc::strlen(line) as libc::c_int;
            RESIZE_MALLOCED_BUFFER! (line, llen, (vlen + 3), line_len, (vlen + 50));

            // if llen + (vlen + 3 as libc::c_int) >= line_len {
            //     while llen + (vlen + 3 as libc::c_int) >= line_len {
            //         line_len += vlen + 50 as libc::c_int;
            //     }
            //     line = sh_xrealloc(
            //         line as *mut libc::c_void,
            //         line_len as size_t,
            //         b"alias.c\0" as *const u8 as *const libc::c_char,
            //         570 as libc::c_int,
            //     ) as *mut libc::c_char;
            // }
            libc::strcpy(line.offset(llen as isize), v);
            if expand_this_token != 0 && vlen != 0
                && (*v.offset((vlen - 1 as libc::c_int) as isize) as libc::c_int
                    == ' ' as i32
                    || *v.offset((vlen - 1 as libc::c_int) as isize) as libc::c_int
                        == '\t' as i32) || alias_expand_all != 0
            {
                expand_next = 1 as libc::c_int;
            }
        } else {
            let llen_0: libc::c_int;
            let tlen: libc::c_int;
            llen_0 = libc::strlen(line) as libc::c_int;
            tlen = i - real_start;
            RESIZE_MALLOCED_BUFFER! (line, llen_0, (tlen + 1), line_len, (llen_0 + tlen + 50));

            // if llen_0 + (tlen + 1 as libc::c_int) >= line_len {
            //     while llen_0 + (tlen + 1 as libc::c_int) >= line_len {
            //         line_len += llen_0 + tlen + 50 as libc::c_int;
            //     }
            //     line = sh_xrealloc(
            //         line as *mut libc::c_void,
            //         line_len as size_t,
            //         b"alias.c\0" as *const u8 as *const libc::c_char,
            //         585 as libc::c_int,
            //     ) as *mut libc::c_char;
            // }
            libc::strncpy(
                line.offset(llen_0 as isize),
                string.offset(real_start as isize),
                tlen as usize,
            );
            *line.offset((llen_0 + tlen) as isize) = '\0' as i32 as libc::c_char;
        }
        command_word = 0 as libc::c_int;
    };
}
