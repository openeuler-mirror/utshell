//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use r_bash::*;
use rcommon::WordList as WORD_LIST;

extern "C" {
    fn mbrtowc(
        __pwc: *mut wchar_t,
        __s: *const libc::c_char,
        __n: size_t,
        __p: *mut mbstate_t,
    ) -> size_t;
    fn __mbrlen(__s: *const libc::c_char, __n: size_t, __ps: *mut mbstate_t) -> size_t;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut no_longjmp_on_fatal_error: libc::c_int;
    fn legal_identifier(_: *const libc::c_char) -> libc::c_int;
    fn valid_nameref_value(_: *const libc::c_char, _: libc::c_int) -> libc::c_int;
    fn report_error(_: *const libc::c_char, _: ...);
    fn internal_warning(_: *const libc::c_char, _: ...);
    fn err_badarraysub(_: *const libc::c_char);
    fn err_readonly(_: *const libc::c_char);
    fn array_create() -> *mut ARRAY;
    fn array_flush(_: *mut ARRAY);
    fn array_dispose_element(_: *mut ARRAY_ELEMENT);
    fn array_insert(_: *mut ARRAY, _: arrayind_t, _: *mut libc::c_char) -> libc::c_int;
    fn array_remove(_: *mut ARRAY, _: arrayind_t) -> *mut ARRAY_ELEMENT;
    fn array_reference(_: *mut ARRAY, _: arrayind_t) -> *mut libc::c_char;
    fn array_to_word_list(_: *mut ARRAY) -> *mut WORD_LIST;
    fn array_keys_to_word_list(_: *mut ARRAY) -> *mut WORD_LIST;
    fn array_to_assign(_: *mut ARRAY, _: libc::c_int) -> *mut libc::c_char;
    fn hash_create(_: libc::c_int) -> *mut HASH_TABLE;
    fn assoc_dispose(_: *mut HASH_TABLE);
    fn assoc_insert(_: *mut HASH_TABLE, _: *mut libc::c_char, _: *mut libc::c_char) -> libc::c_int;
    fn assoc_remove(_: *mut HASH_TABLE, _: *mut libc::c_char);
    fn assoc_reference(_: *mut HASH_TABLE, _: *mut libc::c_char) -> *mut libc::c_char;
    fn assoc_to_assign(_: *mut HASH_TABLE, _: libc::c_int) -> *mut libc::c_char;
    fn assoc_to_word_list(_: *mut HASH_TABLE) -> *mut WORD_LIST;
    fn assoc_keys_to_word_list(_: *mut HASH_TABLE) -> *mut WORD_LIST;
    static mut nameref_invalid_value: SHELL_VAR;
    static mut array_needs_making: libc::c_int;
    fn find_variable(_: *const libc::c_char) -> *mut SHELL_VAR;
    fn find_variable_last_nameref(_: *const libc::c_char, _: libc::c_int) -> *mut SHELL_VAR;
    fn find_variable_nameref_for_create(_: *const libc::c_char, _: libc::c_int) -> *mut SHELL_VAR;
    fn find_shell_variable(_: *const libc::c_char) -> *mut SHELL_VAR;
    fn make_variable_value(
        _: *mut SHELL_VAR,
        _: *mut libc::c_char,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    fn unbind_variable(_: *const libc::c_char) -> libc::c_int;
    fn dispose_variable(_: *mut SHELL_VAR);
    fn make_new_array_variable(_: *mut libc::c_char) -> *mut SHELL_VAR;
    fn make_new_assoc_variable(_: *mut libc::c_char) -> *mut SHELL_VAR;
    fn jump_to_top_level(_: libc::c_int) -> !;
    fn top_level_cleanup();
    fn set_exit_status(_: libc::c_int);
    fn evalexp(_: *mut libc::c_char, _: libc::c_int, _: *mut libc::c_int) -> intmax_t;
    fn expand_arith_string(_: *mut libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn expand_assignment_string_to_string(
        _: *mut libc::c_char,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    fn skipsubscript(_: *const libc::c_char, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn mbschr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn dispose_words(_: *mut WORD_LIST);
    fn expand_words_no_vars(_: *mut WORD_LIST) -> *mut WORD_LIST;
    static mut locale_utf8locale: libc::c_int;
    static mut locale_mb_cur_max: libc::c_int;
    static mut ifs_cmap: [libc::c_uchar; 0];
    fn substring(_: *const libc::c_char, _: libc::c_int, _: libc::c_int) -> *mut libc::c_char;
    fn parse_string_to_word_list(
        _: *mut libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
    ) -> *mut WORD_LIST;
    fn extract_array_assignment_list(
        _: *mut libc::c_char,
        _: *mut libc::c_int,
    ) -> *mut libc::c_char;
    fn sh_single_quote(_: *const libc::c_char) -> *mut libc::c_char;
    fn string_list_dollar_at(
        _: *mut WORD_LIST,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    fn quote_string(_: *mut libc::c_char) -> *mut libc::c_char;
    fn string_list_dollar_star(
        _: *mut WORD_LIST,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    fn make_word(_: *const libc::c_char) -> *mut WORD_DESC;
    fn make_word_list(_: *mut WORD_DESC, _: *mut WORD_LIST) -> *mut WORD_LIST;
    fn string_list_pos_params(
        _: libc::c_int,
        _: *mut WORD_LIST,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    static mut this_command_name: *mut libc::c_char;
    fn glob_char_p(_: *const libc::c_char) -> libc::c_int;
    static is_basic_table: [libc::c_uint; 0];
    fn builtin_error(_: *const libc::c_char, _: ...);
    fn sh_invalidid(_: *mut libc::c_char);
}
pub const W_ASSIGNMENT: i32 = 1 << 2;

pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
}
pub type mbstate_t = __mbstate_t;
pub type __intmax_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct word_desc {
    pub word: *mut libc::c_char,
    pub flags: libc::c_int,
}
pub type WORD_DESC = word_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct word_list {
    pub next: *mut word_list,
    pub word: *mut WORD_DESC,
}
//pub type WORD_LIST = word_list;
pub type intmax_t = __intmax_t;
pub type arrayind_t = intmax_t;
pub type atype = libc::c_uint;
pub const array_assoc: atype = 1;
pub const array_indexed: atype = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array {
    pub type_0: atype,
    pub max_index: arrayind_t,
    pub num_elements: libc::c_int,
    pub head: *mut array_element,
    pub lastref: *mut array_element,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array_element {
    pub ind: arrayind_t,
    pub value: *mut libc::c_char,
    pub next: *mut array_element,
    pub prev: *mut array_element,
}
pub type ARRAY = array;
pub type ARRAY_ELEMENT = array_element;
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
pub struct variable {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub exportstr: *mut libc::c_char,
    pub dynamic_value: Option<sh_var_value_func_t>,
    pub assign_func: Option<sh_var_assign_func_t>,
    pub attributes: libc::c_int,
    pub context: libc::c_int,
}
pub type sh_var_assign_func_t = unsafe extern "C" fn(
    *mut variable,
    *mut libc::c_char,
    arrayind_t,
    *mut libc::c_char,
) -> *mut variable;
pub type sh_var_value_func_t = unsafe extern "C" fn(*mut variable) -> *mut variable;
pub type SHELL_VAR = variable;

#[macro_export]
macro_rules! value_cell {
    ($var:expr) => {
        (*$var).value
    };
}
#[macro_export]
macro_rules! FREE {
    ($s:expr) => {
        if ($s) != std::ptr::null_mut() {
            libc::free($s as *mut libc::c_void);
        }
    };
}
#[macro_export]
macro_rules! savestring {
    ($x:expr) => {
        strcpy(
            libc::malloc((strlen($x as *const libc::c_char) + 1) as usize) as *mut libc::c_char,
            $x,
        ) as *mut libc::c_char
    };
}
#[macro_export]
macro_rules! var_setarray {
    ($var:expr,$arr:expr) => {
        (*$var).value = $arr as *mut libc::c_char;
        (*$var).value
    };
}
#[macro_export]
macro_rules! INVALIDATE_EXPORTSTR {
    ($var:expr) => {
        if !((*$var).exportstr.is_null()) {
            libc::free((*$var).exportstr as *mut libc::c_void);
            (*$var).exportstr = 0 as *mut libc::c_void as *mut libc::c_char;
        }
    };
}
#[macro_export]
macro_rules! exported_p {
    ($var:expr) => {
        (*$var).attributes & (att_exported as libc::c_int)
    };
}
#[macro_export]
macro_rules! VSETATTR {
    ($var:expr,$attr:expr) => {
        (*$var).attributes |= ($attr) as libc::c_int;
        // (*$var).attributes
    };
}
#[macro_export]
macro_rules! VUNSETATTR {
    ($var:expr,$attr:expr) => {
        (*$var).attributes &= !($attr) as libc::c_int;
        (*$var).attributes
    };
}
#[macro_export]
macro_rules! assoc_create {
    ($var:expr) => {
        hash_create($var)
    };
}
#[macro_export]
macro_rules! var_setassoc {
    ($var:expr,$arr:expr) => {
        (*$var).value = $arr as *mut libc::c_char;
    };
}
#[macro_export]
macro_rules! assoc_p {
    ($var:expr) => {
        (*$var).attributes & (att_assoc as libc::c_int)
    };
}
#[macro_export]
macro_rules! assoc_cell {
    ($var:expr) => {
        (*$var).value as *mut hash_table
    };
}
#[macro_export]
macro_rules! array_cell {
    ($var:expr) => {
        (*$var).value as *mut ARRAY
    };
}
#[macro_export]
macro_rules! INVALID_NAMEREF_VALUE {
    () => {
        &mut nameref_invalid_value as *mut SHELL_VAR as *mut libc::c_void as *mut SHELL_VAR
    };
}
#[macro_export]
macro_rules! nameref_p {
    ($var:expr) => {
        (*$var).attributes & att_nameref as libc::c_int
    };
}
#[macro_export]
macro_rules! nameref_cell {
    ($var:expr) => {
        (*$var).value
    };
}
#[macro_export]
macro_rules! readonly_p {
    ($var:expr) => {
        (*$var).attributes & att_readonly as libc::c_int
    };
}
#[macro_export]
macro_rules! noassign_p {
    ($var:expr) => {
        (*$var).attributes & att_noassign as libc::c_int
    };
}
#[macro_export]
macro_rules! array_p {
    ($var:expr) => {
        (*$var).attributes & (att_array as libc::c_int)
    };
}
#[macro_export]
macro_rules! ALL_ELEMENT_SUB {
    ($c:expr) => {
        $c == '@' as i32 || $c == '*' as i32
    };
}
#[macro_export]
macro_rules! array_max_index {
    ($a:expr) => {
        (*$a).max_index
    };
}
#[macro_export]
macro_rules! invisible_p {
    ($var:expr) => {
        (*$var).attributes & att_invisible as libc::c_int
    };
}
#[macro_export]
macro_rules! integer_p {
    ($var:expr) => {
        (*$var).attributes & att_integer as libc::c_int
    };
}
#[macro_export]
macro_rules! DECLARE_MBSTATE {
    ($state:expr) => {
        memset(
            &mut $state as *mut mbstate_t as *mut c_void,
            '\u{0}' as i32,
            (::std::mem::size_of::<mbstate_t>() as libc::c_ulong)
                .try_into()
                .unwrap(),
        )
    };
}
#[macro_export]
macro_rules! isifs {
    ($c:expr) => {
        *ifs_cmap.as_mut_ptr().offset($c as libc::c_uchar as isize) as libc::c_int
    };
}
#[macro_export]
macro_rules! LBRACK {
    () => {
        '['
    };
}
#[macro_export]
macro_rules! RBRACK {
    () => {
        ']'
    };
}
#[macro_export]
macro_rules! STRLEN {
    ($s:expr) => {
        if !$s.is_null() && *$s.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            if *$s.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                if *$s.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                    strlen($s) as libc::c_int
                } else {
                    2 as libc::c_int
                }
            } else {
                1 as libc::c_int
            }
        } else {
            0 as libc::c_int
        }
    };
}
#[macro_export]
macro_rules! INDEX_ERROR {
    ($var: expr, $t: expr, $s: expr) => {
        if !$var.is_null() {
            err_badarraysub((*$var).name);
        } else {
            *$t.offset(-(1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
            err_badarraysub($s);
            *$t.offset(-(1 as libc::c_int) as isize) = '[' as i32 as libc::c_char;
        }
        return 0 as *mut libc::c_void as *mut libc::c_char;
    };
}
#[macro_export]
macro_rules! var_isset {
    ($var:expr) => {
        (*$var).value != 0 as *mut libc::c_char
    };
}

#[inline]
unsafe extern "C" fn mbrlen(
    mut __s: *const libc::c_char,
    mut __n: size_t,
    mut __ps: *mut mbstate_t,
) -> size_t {
    return if !__ps.is_null() {
        mbrtowc(0 as *mut wchar_t, __s, __n, __ps)
    } else {
        __mbrlen(__s, __n, 0 as *mut mbstate_t)
    };
}
#[inline]
unsafe extern "C" fn is_basic(mut c: libc::c_char) -> libc::c_int {
    return (*is_basic_table
        .as_ptr()
        .offset((c as libc::c_uchar as libc::c_int >> 5 as libc::c_int) as isize)
        >> (c as libc::c_uchar as libc::c_int & 31 as libc::c_int)
        & 1 as libc::c_int as libc::c_uint) as libc::c_int;
}

/* This variable means to not expand associative array subscripts more than
once, when performing variable expansion. */
#[no_mangle]
pub static mut assoc_expand_once: libc::c_int = 0 as libc::c_int;

/* Ditto for indexed array subscripts -- currently unused */
#[no_mangle]
pub static mut array_expand_once: libc::c_int = 0 as libc::c_int;

/* Standard error message to use when encountering an invalid array subscript */
#[no_mangle]
pub static mut bash_badsub_errmsg: *const libc::c_char =
    b"bad array subscript\0" as *const u8 as *const libc::c_char;

/* **************************************************************** */
/*								    */
/*  Functions to manipulate array variables and perform assignments */
/*								    */
/* **************************************************************** */

/* Convert a shell variable to an array variable.  The original value is
saved as array[0]. */
#[no_mangle]
pub unsafe extern "C" fn convert_var_to_array(mut var: *mut SHELL_VAR) -> *mut SHELL_VAR {
    let mut oldval: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut array: *mut ARRAY = 0 as *mut ARRAY;

    oldval = value_cell!(var);
    array = array_create();
    if !oldval.is_null() {
        array_insert(array, 0 as libc::c_int as arrayind_t, oldval);
    }
    FREE!(value_cell!(var));
    var_setarray!(var, array);

    /* these aren't valid anymore */
    (*var).dynamic_value = ::core::mem::transmute::<*mut libc::c_void, Option<sh_var_value_func_t>>(
        0 as *mut libc::c_void,
    );
    (*var).assign_func = ::core::mem::transmute::<*mut libc::c_void, Option<sh_var_assign_func_t>>(
        0 as *mut libc::c_void,
    );

    INVALIDATE_EXPORTSTR!(var);

    if exported_p!(var) != 0 {
        array_needs_making += 1;
        array_needs_making;
    }

    VSETATTR!(var, att_array);
    if !oldval.is_null() {
        VUNSETATTR!(var, att_invisible);
    }

    /* Make sure it's not marked as an associative array any more */
    VUNSETATTR!(var, att_assoc);

    /* Since namerefs can't be array variables, turn off nameref attribute */
    VUNSETATTR!(var, att_nameref);

    return var;
}

/* Convert a shell variable to an array variable.  The original value is
saved as array[0]. */
#[no_mangle]
pub unsafe extern "C" fn convert_var_to_assoc(mut var: *mut SHELL_VAR) -> *mut SHELL_VAR {
    let mut oldval: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hash: *mut HASH_TABLE = 0 as *mut HASH_TABLE;

    oldval = value_cell!(var);
    hash = assoc_create!(0);
    if !oldval.is_null() {
        assoc_insert(
            hash,
            savestring!(b"0\0" as *const u8 as *const libc::c_char),
            oldval,
        );
    }

    FREE!(value_cell!(var));
    var_setassoc!(var, hash);

    /* these aren't valid anymore */
    (*var).dynamic_value = ::core::mem::transmute::<*mut libc::c_void, Option<sh_var_value_func_t>>(
        0 as *mut libc::c_void,
    );
    (*var).assign_func = ::core::mem::transmute::<*mut libc::c_void, Option<sh_var_assign_func_t>>(
        0 as *mut libc::c_void,
    );

    INVALIDATE_EXPORTSTR!(var);

    if exported_p!(var) != 0 {
        array_needs_making += 1;
        array_needs_making;
    }

    VSETATTR!(var, att_assoc);
    if !oldval.is_null() {
        VUNSETATTR!(var, att_invisible);
    }

    /* Make sure it's not marked as an indexed array any more */
    VUNSETATTR!(var, att_array);

    /* Since namerefs can't be array variables, turn off nameref attribute */
    VUNSETATTR!(var, att_nameref);
    return var;
}

#[no_mangle]
pub unsafe extern "C" fn make_array_variable_value(
    mut entry: *mut SHELL_VAR,
    mut ind: arrayind_t,
    mut key: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut flags: libc::c_int,
) -> *mut libc::c_char {
    let mut dentry: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut newval: *mut libc::c_char = 0 as *mut libc::c_char;

    /* If we're appending, we need the old value of the array reference, so
    fake out make_variable_value with a dummy SHELL_VAR */
    if flags & ASS_APPEND as libc::c_int != 0 {
        dentry = libc::malloc(::core::mem::size_of::<SHELL_VAR>() as libc::c_ulong as usize)
            as *mut SHELL_VAR;
        (*dentry).name = savestring!((*entry).name);
        if assoc_p!(entry) != 0 {
            newval = assoc_reference(assoc_cell!(entry), key);
        } else {
            newval = array_reference(array_cell!(entry), ind);
        }
        if !newval.is_null() {
            (*dentry).value = savestring!(newval);
        } else {
            (*dentry).value = libc::malloc(1 as libc::c_int as usize) as *mut libc::c_char;
            *((*dentry).value).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        }
        (*dentry).exportstr = 0 as *mut libc::c_char;
        (*dentry).attributes = (*entry).attributes
            & !(att_array as libc::c_int | att_assoc as libc::c_int | att_exported as libc::c_int);
        /* Leave the rest of the members uninitialized; the code doesn't look
        at them. */
        newval = make_variable_value(dentry, value, flags);
        dispose_variable(dentry);
    } else {
        newval = make_variable_value(entry, value, flags);
    }

    return newval;
}

/* Assign HASH[KEY]=VALUE according to FLAGS. ENTRY is an associative array
variable; HASH is the hash table to assign into. HASH may or may not be
the hash table associated with ENTRY; if it's not, the caller takes care
of it.
XXX - make sure that any dynamic associative array variables recreate the
hash table on each assignment. BASH_CMDS and BASH_ALIASES already do this */
unsafe extern "C" fn bind_assoc_var_internal(
    mut entry: *mut SHELL_VAR,
    mut hash: *mut HASH_TABLE,
    mut key: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut flags: libc::c_int,
) -> *mut SHELL_VAR {
    let mut newval: *mut libc::c_char = 0 as *mut libc::c_char;

    /* Use the existing array contents to expand the value */
    newval = make_array_variable_value(entry, 0 as libc::c_int as arrayind_t, key, value, flags);

    if ((*entry).assign_func).is_some() {
        (Some(((*entry).assign_func).expect("non-null function pointer")))
            .expect("non-null function pointer")(
            entry,
            newval,
            0 as libc::c_int as arrayind_t,
            key,
        );
    } else {
        assoc_insert(hash, key, newval);
    }

    FREE!(newval);

    VUNSETATTR!(entry, att_invisible); /* no longer invisible */

    /* check mark_modified_variables if we ever want to export array vars */
    return entry;
}

/* Perform ENTRY[IND]=VALUE or ENTRY[KEY]=VALUE. This is not called for every
assignment to an associative array; see assign_compound_array_list below. */
unsafe extern "C" fn bind_array_var_internal(
    mut entry: *mut SHELL_VAR,
    mut ind: arrayind_t,
    mut key: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut flags: libc::c_int,
) -> *mut SHELL_VAR {
    let mut newval: *mut libc::c_char = 0 as *mut libc::c_char;

    newval = make_array_variable_value(entry, ind, key, value, flags);

    if ((*entry).assign_func).is_some() {
        (Some(((*entry).assign_func).expect("non-null function pointer")))
            .expect("non-null function pointer")(entry, newval, ind, key);
    } else if assoc_p!(entry) != 0 {
        assoc_insert(assoc_cell!(entry), key, newval);
    } else {
        array_insert(array_cell!(entry), ind, newval);
    }
    FREE!(newval);

    VUNSETATTR!(entry, att_invisible); /* no longer invisible */

    /* check mark_modified_variables if we ever want to export array vars */
    return entry;
}

/* Perform an array assignment name[ind]=value.  If NAME already exists and
is not an array, and IND is 0, perform name=value instead.  If NAME exists
and is not an array, and IND is not 0, convert it into an array with the
existing value as name[0].

If NAME does not exist, just create an array variable, no matter what
IND's value may be. */
#[no_mangle]
pub unsafe extern "C" fn bind_array_variable(
    mut name: *mut libc::c_char,
    mut ind: arrayind_t,
    mut value: *mut libc::c_char,
    mut flags: libc::c_int,
) -> *mut SHELL_VAR {
    let mut entry: *mut SHELL_VAR = 0 as *mut SHELL_VAR;

    entry = find_shell_variable(name);

    if entry.is_null() {
        /* Is NAME a nameref variable that points to an unset variable? */
        entry = find_variable_nameref_for_create(name, 0 as libc::c_int);
        if entry == INVALID_NAMEREF_VALUE!() {
            return 0 as *mut SHELL_VAR;
        }
        if !entry.is_null() && nameref_p!(entry) != 0 {
            entry = make_new_array_variable(nameref_cell!(entry));
        }
    }
    if entry.is_null() {
        entry = make_new_array_variable(name);
    } else if readonly_p!(entry) != 0 && flags & ASS_FORCE as libc::c_int == 0 as libc::c_int
        || noassign_p!(entry) != 0
    {
        if readonly_p!(entry) != 0 {
            err_readonly(name);
        }
        return entry;
    } else if array_p!(entry) == 0 as libc::c_int {
        entry = convert_var_to_array(entry);
    }

    /* ENTRY is an array variable, and ARRAY points to the value. */
    return bind_array_var_internal(entry, ind, 0 as *mut libc::c_char, value, flags);
}

#[no_mangle]
pub unsafe extern "C" fn bind_array_element(
    mut entry: *mut SHELL_VAR,
    mut ind: arrayind_t,
    mut value: *mut libc::c_char,
    mut flags: libc::c_int,
) -> *mut SHELL_VAR {
    return bind_array_var_internal(entry, ind, 0 as *mut libc::c_char, value, flags);
}

#[no_mangle]
pub unsafe extern "C" fn bind_assoc_variable(
    mut entry: *mut SHELL_VAR,
    mut name: *mut libc::c_char,
    mut key: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut flags: libc::c_int,
) -> *mut SHELL_VAR {
    if readonly_p!(entry) != 0 && flags & ASS_FORCE as libc::c_int == 0 as libc::c_int
        || noassign_p!(entry) != 0
    {
        if readonly_p!(entry) != 0 {
            err_readonly(name);
        }
        return entry;
    }

    return bind_assoc_var_internal(entry, assoc_cell!(entry), key, value, flags);
}

/* Parse NAME, a lhs of an assignment statement of the form v[s], and
assign VALUE to that array element by calling bind_array_variable().
Flags are ASS_ assignment flags */
#[no_mangle]
pub unsafe extern "C" fn assign_array_element(
    mut name: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut flags: libc::c_int,
) -> *mut SHELL_VAR {
    let mut sub: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sublen: libc::c_int = 0;
    let mut isassoc: libc::c_int = 0;
    let mut entry: *mut SHELL_VAR = 0 as *mut SHELL_VAR;

    vname = array_variable_name(
        name,
        (flags & ASS_NOEXPAND as libc::c_int != 0 as libc::c_int) as libc::c_int,
        &mut sub,
        &mut sublen,
    );

    if vname.is_null() {
        return 0 as *mut libc::c_void as *mut SHELL_VAR;
    }

    entry = find_variable(vname);
    isassoc = (!entry.is_null() && assoc_p!(entry) != 0) as libc::c_int;

    if (isassoc == 0 as libc::c_int || flags & ASS_NOEXPAND as libc::c_int == 0 as libc::c_int)
        && (ALL_ELEMENT_SUB!(sub.offset(0 as libc::c_int as isize) as libc::c_int)
            && *sub.offset(1 as libc::c_int as isize) as libc::c_int == ']' as i32)
        || sublen <= 1 as libc::c_int
    {
        libc::free(vname as *mut libc::c_void);
        err_badarraysub(name);
        return 0 as *mut libc::c_void as *mut SHELL_VAR;
    }

    entry = assign_array_element_internal(entry, name, vname, sub, sublen, value, flags);

    libc::free(vname as *mut libc::c_void);
    return entry;
}

unsafe extern "C" fn assign_array_element_internal(
    mut entry: *mut SHELL_VAR,
    mut name: *mut libc::c_char, /* only used for error messages */
    mut vname: *mut libc::c_char,
    mut sub: *mut libc::c_char,
    mut sublen: libc::c_int,
    mut value: *mut libc::c_char,
    mut flags: libc::c_int,
) -> *mut SHELL_VAR {
    let mut akey: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ind: arrayind_t = 0;

    if !entry.is_null() && assoc_p!(entry) as libc::c_int != 0 {
        *sub.offset((sublen - 1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
        if flags & ASS_NOEXPAND as libc::c_int == 0 as libc::c_int {
            akey = expand_assignment_string_to_string(sub, 0 as libc::c_int);
        } else {
            akey = savestring!(sub);
        }
        *sub.offset((sublen - 1 as libc::c_int) as isize) = ']' as i32 as libc::c_char;
        if akey.is_null() || *akey as libc::c_int == 0 as libc::c_int {
            err_badarraysub(name);
            FREE!(akey);
            return 0 as *mut libc::c_void as *mut SHELL_VAR;
        }
        entry = bind_assoc_variable(entry, vname, akey, value, flags);
    } else {
        ind = array_expand_index(entry, sub, sublen, 0 as libc::c_int);
        /* negative subscripts to indexed arrays count back from end */
        if !entry.is_null() && ind < 0 as libc::c_int as libc::c_long {
            ind = (if array_p!(entry) != 0 {
                array_max_index!(array_cell!(entry))
            } else {
                0 as libc::c_int as libc::c_long
            }) + 1 as libc::c_int as libc::c_long
                + ind;
        }

        if ind < 0 as libc::c_int as libc::c_long {
            err_badarraysub(name);
            return 0 as *mut libc::c_void as *mut SHELL_VAR;
        }
        entry = bind_array_variable(vname, ind, value, flags);
    }

    return entry;
}

/* Find the array variable corresponding to NAME.  If there is no variable,
create a new array variable.  If the variable exists but is not an array,
convert it to an indexed array.  If FLAGS&1 is non-zero, an existing
variable is checked for the readonly or noassign attribute in preparation
for assignment (e.g., by the `read' builtin).  If FLAGS&2 is non-zero, we
create an associative array. */
#[no_mangle]
pub unsafe extern "C" fn find_or_make_array_variable(
    mut name: *mut libc::c_char,
    mut flags: libc::c_int,
) -> *mut SHELL_VAR {
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;

    var = find_variable(name);
    if var.is_null() {
        /* See if we have a nameref pointing to a variable that hasn't been
        created yet. */
        var = find_variable_last_nameref(name, 1 as libc::c_int);
        if !var.is_null() && nameref_p!(var) != 0 && invisible_p!(var) != 0 {
            internal_warning(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: removing nameref attribute\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                name,
            );
            VUNSETATTR!(var, att_nameref);
        }
        if !var.is_null() && nameref_p!(var) != 0 {
            if valid_nameref_value(nameref_cell!(var), 2 as libc::c_int) == 0 as libc::c_int {
                sh_invalidid(nameref_cell!(var));
                return 0 as *mut libc::c_void as *mut SHELL_VAR;
            }
            var = if flags & 2 as libc::c_int != 0 {
                make_new_assoc_variable(nameref_cell!(var))
            } else {
                make_new_array_variable(nameref_cell!(var))
            };
        }
    }

    if var.is_null() {
        var = if flags & 2 as libc::c_int != 0 {
            make_new_assoc_variable(name)
        } else {
            make_new_array_variable(name)
        };
    } else if flags & 1 as libc::c_int != 0 && (readonly_p!(var) != 0 || noassign_p!(var) != 0) {
        if readonly_p!(var) != 0 {
            err_readonly(name);
        }
        return 0 as *mut libc::c_void as *mut SHELL_VAR;
    } else if flags & 2 as libc::c_int != 0 && array_p!(var) != 0 {
        set_exit_status(EXECUTION_FAILURE as libc::c_int);
        report_error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: cannot convert indexed to associative array\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            name,
        );
        return 0 as *mut libc::c_void as *mut SHELL_VAR;
    } else if array_p!(var) == 0 as libc::c_int && assoc_p!(var) == 0 as libc::c_int {
        var = convert_var_to_array(var);
    }

    return var;
}

/* Perform a compound assignment statement for array NAME, where VALUE is
the text between the parens:  NAME=( VALUE ) */
#[no_mangle]
pub unsafe extern "C" fn assign_array_from_string(
    mut name: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut flags: libc::c_int,
) -> *mut SHELL_VAR {
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut vflags: libc::c_int = 0;

    vflags = 1 as libc::c_int;
    if flags & ASS_MKASSOC as libc::c_int != 0 {
        vflags |= 2 as libc::c_int;
    }

    var = find_or_make_array_variable(name, vflags);
    if var.is_null() {
        return 0 as *mut libc::c_void as *mut SHELL_VAR;
    }

    return assign_array_var_from_string(var, value, flags);
}

/* Sequentially assign the indices of indexed array variable VAR from the
words in LIST. */
#[no_mangle]
pub unsafe extern "C" fn assign_array_var_from_word_list(
    mut var: *mut SHELL_VAR,
    mut list: *mut WORD_LIST,
    mut flags: libc::c_int,
) -> *mut SHELL_VAR {
    let mut i: arrayind_t = 0;
    let mut l: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut a: *mut ARRAY = 0 as *mut ARRAY;

    a = array_cell!(var);
    i = if flags & ASS_APPEND as libc::c_int != 0 {
        array_max_index!(a) + 1 as libc::c_int as libc::c_long
    } else {
        0 as libc::c_int as libc::c_long
    };

    l = list;
    while !l.is_null() {
        bind_array_var_internal(
            var,
            i,
            0 as *mut libc::c_char,
            (*(*l).word).word,
            flags & !(ASS_APPEND as libc::c_int),
        );
        l = (*l).next;
        i += 1;
        i;
    }

    VUNSETATTR!(var, att_invisible); /* no longer invisible */

    return var;
}

#[no_mangle]
pub unsafe extern "C" fn expand_compound_array_assignment(
    mut var: *mut SHELL_VAR,
    mut value: *mut libc::c_char,
    mut flags: libc::c_int,
) -> *mut WORD_LIST {
    let mut list: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut nlist: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ni: libc::c_int = 0;

    /* This condition is true when invoked from the declare builtin with a
     command like
    declare -a d='([1]="" [2]="bdef" [5]="hello world" "test")' */
    if *value as libc::c_int == '(' as i32 {
        ni = 1 as libc::c_int;
        val = extract_array_assignment_list(value, &mut ni);
        if val.is_null() {
            return 0 as *mut libc::c_void as *mut WORD_LIST;
        }
    } else {
        val = value;
    }

    /* Expand the value string into a list of words, performing all the
    shell expansions including pathname generation and word splitting. */
    /* First we split the string on whitespace, using the shell parser
    (ksh93 seems to do this). */
    list = parse_string_to_word_list(
        val,
        1 as libc::c_int,
        b"array assign\0" as *const u8 as *const libc::c_char,
    );

    /* Note that we defer expansion of the assignment statements for associative
    arrays here, so we don't have to scan the subscript and find the ending
    bracket twice. See the caller below. */
    if !var.is_null() && assoc_p!(var) != 0 {
        if val != value {
            libc::free(val as *mut libc::c_void);
        }
        return list;
    }

    /* If we're using [subscript]=value, we need to quote each [ and ] to
    prevent unwanted filename expansion.  This doesn't need to be done
    for associative array expansion, since that uses a different expansion
    function (see assign_compound_array_list below). */
    if !list.is_null() {
        quote_array_assignment_chars(list);
    }

    /* Now that we've split it, perform the shell expansions on each
    word in the list. */
    nlist = if !list.is_null() {
        expand_words_no_vars(list)
    } else {
        0 as *mut libc::c_void as *mut WORD_LIST
    };

    dispose_words(list);

    if val != value {
        libc::free(val as *mut libc::c_void);
    }

    return nlist;
}

unsafe extern "C" fn assign_assoc_from_kvlist(
    mut var: *mut SHELL_VAR,
    mut nlist: *mut WORD_LIST,
    mut h: *mut HASH_TABLE,
    mut flags: libc::c_int,
) {
    let mut list: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut akey: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut aval: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut k: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut free_aval: libc::c_int = 0;

    list = nlist;
    while !list.is_null() {
        free_aval = 0 as libc::c_int;

        k = (*(*list).word).word;
        v = if !((*list).next).is_null() {
            (*(*(*list).next).word).word
        } else {
            0 as *mut libc::c_char
        };

        if !((*list).next).is_null() {
            list = (*list).next;
        }

        akey = expand_assignment_string_to_string(k, 0 as libc::c_int);
        aval = expand_assignment_string_to_string(v, 0 as libc::c_int);

        if akey.is_null() || *akey as libc::c_int == 0 as libc::c_int {
            err_badarraysub(k);
            FREE!(akey);
        } else {
            if aval.is_null() {
                aval = libc::malloc(1 as libc::c_int as usize) as *mut libc::c_char;
                *aval.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char; /* like do_assignment_internal */
                free_aval = 1 as libc::c_int;
            }

            bind_assoc_var_internal(var, h, akey, aval, flags);
            if free_aval != 0 {
                libc::free(aval as *mut libc::c_void);
            }
        }
        list = (*list).next;
    }
}

/* Return non-zero if L appears to be a key-value pair associative array
compound assignment. */
#[no_mangle]
pub unsafe extern "C" fn kvpair_assignment_p(mut l: *mut WORD_LIST) -> libc::c_int {
    return (!l.is_null()
        && (*(*l).word).flags & W_ASSIGNMENT == 0 as libc::c_int
        && *((*(*l).word).word).offset(0 as libc::c_int as isize) as libc::c_int != '[' as i32)
        as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn expand_and_quote_kvpair_word(
    mut w: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;

    t = if !w.is_null() {
        expand_assignment_string_to_string(w, 0 as libc::c_int)
    } else {
        0 as *mut libc::c_char
    };
    r = sh_single_quote(if !t.is_null() {
        t as *const libc::c_char
    } else {
        b"\0" as *const u8 as *const libc::c_char
    });
    libc::free(t as *mut libc::c_void);
    return r;
}

/* Callers ensure that VAR is not NULL. Associative array assignments have not
been expanded when this is called, or have been expanded once and single-
quoted, so we don't have to scan through an unquoted expanded subscript to
find the ending bracket; indexed array assignments have been expanded and
possibly single-quoted to prevent further expansion.

If this is an associative array, we perform the assignments into NHASH and
set NHASH to be the value of VAR after processing the assignments in NLIST */
#[no_mangle]
pub unsafe extern "C" fn assign_compound_array_list(
    mut var: *mut SHELL_VAR,
    mut nlist: *mut WORD_LIST,
    mut flags: libc::c_int,
) {
    let mut a: *mut ARRAY = 0 as *mut ARRAY;
    let mut h: *mut HASH_TABLE = 0 as *mut HASH_TABLE;
    let mut nhash: *mut HASH_TABLE = 0 as *mut HASH_TABLE;
    let mut list: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut w: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nval: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut savecmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut iflags: libc::c_int = 0;
    let mut free_val: libc::c_int = 0;
    let mut ind: arrayind_t = 0;
    let mut last_ind: arrayind_t = 0;
    let mut akey: *mut libc::c_char = 0 as *mut libc::c_char;

    a = if !var.is_null() && array_p!(var) != 0 {
        array_cell!(var)
    } else {
        0 as *mut ARRAY
    };
    h = if !var.is_null() && assoc_p!(var) != 0 {
        assoc_cell!(var)
    } else {
        0 as *mut HASH_TABLE
    };
    nhash = h;

    akey = 0 as *mut libc::c_char;
    ind = 0 as libc::c_int as arrayind_t;

    /* Now that we are ready to assign values to the array, kill the existing
    value. */
    if flags & ASS_APPEND as libc::c_int == 0 as libc::c_int {
        if !a.is_null() && array_p!(var) != 0 {
            array_flush(a);
        } else if !h.is_null() && assoc_p!(var) != 0 {
            nhash = assoc_create!((*h).nbuckets);
        }
    }

    last_ind = if !a.is_null() && flags & ASS_APPEND as libc::c_int != 0 {
        array_max_index!(a) + 1 as libc::c_int as libc::c_long
    } else {
        0 as libc::c_int as libc::c_long
    };

    if assoc_p!(var) != 0 && kvpair_assignment_p(nlist) != 0 {
        iflags = flags & !(ASS_APPEND as libc::c_int);
        assign_assoc_from_kvlist(var, nlist, nhash, iflags);
        if !nhash.is_null() && nhash != h {
            h = assoc_cell!(var);
            var_setassoc!(var, nhash);
            assoc_dispose(h);
        }
        return;
    }

    list = nlist;
    loop {
        if list.is_null() {
            break;
        }
        /* Don't allow var+=(values) to make assignments in VALUES append to
        existing values by default. */
        iflags = flags & !(ASS_APPEND as libc::c_int);
        w = (*(*list).word).word;
        /* We have a word of the form [ind]=value */
        if (*(*list).word).flags & W_ASSIGNMENT != 0
            && *w.offset(0 as libc::c_int as isize) as libc::c_int == '[' as i32
        {
            /* Don't have to handle embedded quotes specially any more, since
            associative array subscripts have not been expanded yet (see
            above). */
            len = skipsubscript(w, 0 as libc::c_int, 0 as libc::c_int);

            /* XXX - changes for `+=' */
            if *w.offset(len as isize) as libc::c_int != ']' as i32
                || *w.offset((len + 1 as libc::c_int) as isize) as libc::c_int != '=' as i32
                    && (*w.offset((len + 1 as libc::c_int) as isize) as libc::c_int != '+' as i32
                        || *w.offset((len + 2 as libc::c_int) as isize) as libc::c_int
                            != '=' as i32)
            {
                if assoc_p!(var) != 0 {
                    err_badarraysub(w);
                    list = (*list).next;
                    continue;
                }
                nval = make_variable_value(var, w, flags);
                if ((*var).assign_func).is_some() {
                    (Some(((*var).assign_func).expect("non-null function pointer")))
                        .expect("non-null function pointer")(
                        var,
                        nval,
                        last_ind,
                        0 as *mut libc::c_char,
                    );
                } else {
                    array_insert(a, last_ind, nval);
                }
                FREE!(nval);
                last_ind += 1;
                last_ind;
                list = (*list).next;
                continue;
            }

            if len == 1 as libc::c_int {
                err_badarraysub(w);
                list = (*list).next;
                continue;
            }

            if ALL_ELEMENT_SUB!(*w.offset(1 as libc::c_int as isize) as libc::c_int)
                && len == 2 as libc::c_int
            {
                set_exit_status(EXECUTION_FAILURE as libc::c_int);
                if assoc_p!(var) != 0 {
                    report_error(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: invalid associative array key\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        w,
                    );
                } else {
                    report_error(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: cannot assign to non-numeric index\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        w,
                    );
                }
                list = (*list).next;
                continue;
            }

            if array_p!(var) != 0 {
                ind = array_expand_index(
                    var,
                    w.offset(1 as libc::c_int as isize),
                    len,
                    0 as libc::c_int,
                );
                /* negative subscripts to indexed arrays count back from end */
                if ind < 0 as libc::c_int as libc::c_long {
                    ind =
                        array_max_index!(array_cell!(var)) + 1 as libc::c_int as libc::c_long + ind;
                }
                if ind < 0 as libc::c_int as libc::c_long {
                    err_badarraysub(w);
                    list = (*list).next;
                    continue;
                }

                last_ind = ind;
            } else if assoc_p!(var) != 0 {
                /* This is not performed above, see expand_compound_array_assignment */
                *w.offset(len as isize) = '\0' as i32 as libc::c_char; /*[*/
                akey = expand_assignment_string_to_string(
                    w.offset(1 as libc::c_int as isize),
                    0 as libc::c_int,
                );
                *w.offset(len as isize) = ']' as i32 as libc::c_char;
                /* And we need to expand the value also, see below */
                if akey.is_null() || *akey as libc::c_int == 0 as libc::c_int {
                    err_badarraysub(w);
                    FREE!(akey);
                    list = (*list).next;
                    continue;
                }
            }

            /* XXX - changes for `+=' -- just accept the syntax.  ksh93 doesn't do this */
            if *w.offset((len + 1 as libc::c_int) as isize) as libc::c_int == '+' as i32
                && *w.offset((len + 2 as libc::c_int) as isize) as libc::c_int == '=' as i32
            {
                iflags |= ASS_APPEND as libc::c_int;
                val = w.offset(len as isize).offset(3 as libc::c_int as isize);
            } else {
                val = w.offset(len as isize).offset(2 as libc::c_int as isize);
            }
        } else if assoc_p!(var) != 0 {
            set_exit_status(EXECUTION_FAILURE as libc::c_int);
            report_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: %s: must use subscript when assigning associative array\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*var).name,
                w,
            );
            list = (*list).next;
            continue;
        } else {
            /* No [ind]=value, just a stray `=' */
            ind = last_ind;
            val = w;
        }
        free_val = 0 as libc::c_int;
        /* See above; we need to expand the value here */
        if assoc_p!(var) != 0 {
            val = expand_assignment_string_to_string(val, 0 as libc::c_int);
            if val.is_null() {
                val = libc::malloc(1 as libc::c_int as usize) as *mut libc::c_char;
                *val.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            }
            free_val = 1 as libc::c_int;
        }
        savecmd = this_command_name;
        if integer_p!(var) != 0 {
            this_command_name = 0 as *mut libc::c_void as *mut libc::c_char;
        }
        if assoc_p!(var) != 0 {
            bind_assoc_var_internal(var, nhash, akey, val, iflags);
        } else {
            bind_array_var_internal(var, ind, akey, val, iflags);
        }
        last_ind += 1;
        last_ind;
        this_command_name = savecmd;

        if free_val != 0 {
            libc::free(val as *mut libc::c_void);
        }

        list = (*list).next;
    }

    if assoc_p!(var) != 0 && !nhash.is_null() && nhash != h {
        h = assoc_cell!(var);
        var_setassoc!(var, nhash);
        assoc_dispose(h);
    }
}

/* Perform a compound array assignment:  VAR->name=( VALUE ).  The
VALUE has already had the parentheses stripped. */
#[no_mangle]
pub unsafe extern "C" fn assign_array_var_from_string(
    mut var: *mut SHELL_VAR,
    mut value: *mut libc::c_char,
    mut flags: libc::c_int,
) -> *mut SHELL_VAR {
    let mut nlist: *mut WORD_LIST = 0 as *mut WORD_LIST;

    if value.is_null() {
        return var;
    }

    nlist = expand_compound_array_assignment(var, value, flags);
    assign_compound_array_list(var, nlist, flags);

    if !nlist.is_null() {
        dispose_words(nlist);
    }

    if !var.is_null() {
        VUNSETATTR!(var, att_invisible); /* no longer invisible */
    }

    return var;
}

/* Quote globbing chars and characters in $IFS before the `=' in an assignment
statement (usually a compound array assignment) to protect them from
unwanted filename expansion or word splitting. */
unsafe extern "C" fn quote_assign(mut string: *const libc::c_char) -> *mut libc::c_char {
    let mut slen: size_t = 0;
    let mut saw_eq: libc::c_int = 0;
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut subs: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut send: *const libc::c_char = 0 as *const libc::c_char;
    let mut ss: libc::c_int = 0;
    let mut se: libc::c_int = 0;
    let mut state: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    memset(
        &mut state as *mut mbstate_t as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );

    slen = strlen(string);
    send = string.offset(slen as isize);

    temp = libc::malloc(
        slen.wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
    ) as *mut libc::c_char;
    t = temp;
    saw_eq = 0 as libc::c_int;
    s = string;
    while *s != 0 {
        if *s as libc::c_int == '=' as i32 {
            saw_eq = 1 as libc::c_int;
        }
        /* looks like a subscript */
        if saw_eq == 0 as libc::c_int && *s as libc::c_int == '[' as i32 {
            ss = s.offset_from(string) as libc::c_long as libc::c_int;
            se = skipsubscript(string, ss, 0 as libc::c_int);
            subs = substring(s, ss, se);
            let fresh0 = t;
            t = t.offset(1);
            *fresh0 = '\\' as i32 as libc::c_char;
            strcpy(t, subs);
            t = t.offset((se - ss) as isize);
            let fresh1 = t;
            t = t.offset(1);
            *fresh1 = '\\' as i32 as libc::c_char;
            let fresh2 = t;
            t = t.offset(1);
            *fresh2 = ']' as i32 as libc::c_char;
            s = s.offset((se + 1 as libc::c_int) as isize);
            libc::free(subs as *mut libc::c_void);
        }
        if saw_eq == 0 as libc::c_int && (glob_char_p(s) != 0 || isifs!(*s) != 0 as libc::c_int) {
            let fresh3 = t;
            t = t.offset(1);
            *fresh3 = '\\' as i32 as libc::c_char;
        }

        if locale_mb_cur_max > 1 as libc::c_int {
            let mut state_bak: mbstate_t = mbstate_t {
                __count: 0,
                __value: C2RustUnnamed { __wch: 0 },
            };
            let mut mblength: size_t = 0;
            let mut _k: libc::c_int = 0;
            _k = is_basic(*s);
            if _k != 0 {
                mblength = 1 as libc::c_int as size_t;
            } else if locale_utf8locale != 0
                && *s as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
            {
                mblength = (*s as libc::c_int != 0 as libc::c_int) as libc::c_int as size_t;
            } else {
                state_bak = state;
                mblength = mbrlen(s, send.offset_from(s) as libc::c_long as size_t, &mut state);
            }
            if mblength == -(2 as libc::c_int) as size_t
                || mblength == -(1 as libc::c_int) as size_t
            {
                state = state_bak;
                mblength = 1 as libc::c_int as size_t;
            } else {
                mblength = if mblength < 1 as libc::c_int as libc::c_ulong {
                    1 as libc::c_int as libc::c_ulong
                } else {
                    mblength
                };
            }
            _k = 0 as libc::c_int;
            while (_k as libc::c_ulong) < mblength {
                let fresh4 = s;
                s = s.offset(1);
                let fresh5 = t;
                t = t.offset(1);
                *fresh5 = *fresh4;
                _k += 1;
                _k;
            }
        } else {
            let fresh6 = s;
            s = s.offset(1);
            let fresh7 = t;
            t = t.offset(1);
            *fresh7 = *fresh6;
        }
    }
    *t = '\0' as i32 as libc::c_char;
    return temp;
}

/* Take a word W of the form [IND]=VALUE and transform it to ['IND']='VALUE'
to prevent further expansion. This is called for compound assignments to
indexed arrays. W has already undergone word expansions. If W has no [IND]=,
just single-quote and return it. */
unsafe extern "C" fn quote_compound_array_word(
    mut w: *mut libc::c_char,
    mut type_0: libc::c_int,
) -> *mut libc::c_char {
    let mut nword: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sub: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ind: libc::c_int = 0;
    let mut wlen: libc::c_int = 0;
    let mut i: libc::c_int = 0;

    //LBRACK  RBRACK
    if *w.offset(0 as libc::c_int as isize) as libc::c_int != LBRACK!() as i32 {
        return sh_single_quote(w);
    }
    ind = skipsubscript(w, 0 as libc::c_int, 0 as libc::c_int);
    if *w.offset(ind as isize) as libc::c_int != RBRACK!() as i32 {
        return sh_single_quote(w);
    }

    wlen = strlen(w) as libc::c_int;
    *w.offset(ind as isize) = '\0' as i32 as libc::c_char;
    sub = sh_single_quote(w.offset(1 as libc::c_int as isize));
    *w.offset(ind as isize) = RBRACK!() as i32 as libc::c_char;

    nword =
        libc::malloc((wlen * 4 as libc::c_int + 5 as libc::c_int) as usize) as *mut libc::c_char; /* wlen*4 is max single quoted length */
    *nword.offset(0 as libc::c_int as isize) = LBRACK!() as i32 as libc::c_char;
    i = STRLEN!(sub);
    memcpy(
        nword.offset(1 as libc::c_int as isize) as *mut libc::c_void,
        sub as *const libc::c_void,
        i as libc::c_ulong,
    );
    i += 1;
    i; /* accommodate the opening LBRACK */
    let fresh8 = ind;
    ind = ind + 1;
    let fresh9 = i;
    i = i + 1;
    *nword.offset(fresh9 as isize) = *w.offset(fresh8 as isize); /* RBRACK */
    if *w.offset(ind as isize) as libc::c_int == '+' as i32 {
        let fresh10 = ind;
        ind = ind + 1;
        let fresh11 = i;
        i = i + 1;
        *nword.offset(fresh11 as isize) = *w.offset(fresh10 as isize);
    }
    let fresh12 = ind;
    ind = ind + 1;
    let fresh13 = i;
    i = i + 1;
    *nword.offset(fresh13 as isize) = *w.offset(fresh12 as isize);
    value = sh_single_quote(w.offset(ind as isize));
    strcpy(nword.offset(i as isize), value);

    return nword;
}

/* Expand the key and value in W, which is of the form [KEY]=VALUE, and
reconstruct W with the expanded and single-quoted version:
['expanded-key']='expanded-value'. If there is no [KEY]=, single-quote the
word and return it. Very similar to previous function, but does not assume
W has already been expanded, and expands the KEY and VALUE separately.
Used for compound assignments to associative arrays that are arguments to
declaration builtins (declare -A a=( list )). */
#[no_mangle]
pub unsafe extern "C" fn expand_and_quote_assoc_word(
    mut w: *mut libc::c_char,
    mut type_0: libc::c_int,
) -> *mut libc::c_char {
    let mut nword: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ind: libc::c_int = 0;
    let mut wlen: libc::c_int = 0;
    let mut i: libc::c_int = 0;

    if *w.offset(0 as libc::c_int as isize) as libc::c_int != LBRACK!() as i32 {
        return sh_single_quote(w);
    }
    ind = skipsubscript(w, 0 as libc::c_int, 0 as libc::c_int);
    if *w.offset(ind as isize) as libc::c_int != RBRACK!() as i32 {
        return sh_single_quote(w);
    }

    *w.offset(ind as isize) = '\0' as i32 as libc::c_char;
    t = expand_assignment_string_to_string(w.offset(1 as libc::c_int as isize), 0 as libc::c_int);
    *w.offset(ind as isize) = RBRACK!() as i32 as libc::c_char;

    key = sh_single_quote(if !t.is_null() {
        t as *const libc::c_char
    } else {
        b"\0" as *const u8 as *const libc::c_char
    });
    libc::free(t as *mut libc::c_void);

    wlen = STRLEN!(key);
    nword = libc::malloc((wlen + 5 as libc::c_int) as usize) as *mut libc::c_char;
    *nword.offset(0 as libc::c_int as isize) = LBRACK!() as i32 as libc::c_char;
    memcpy(
        nword.offset(1 as libc::c_int as isize) as *mut libc::c_void,
        key as *const libc::c_void,
        wlen as libc::c_ulong,
    );
    i = wlen + 1 as libc::c_int; /* accommodate the opening LBRACK */

    let fresh14 = ind; /* RBRACK */
    ind = ind + 1;
    let fresh15 = i;
    i = i + 1;
    *nword.offset(fresh15 as isize) = *w.offset(fresh14 as isize);
    if *w.offset(ind as isize) as libc::c_int == '+' as i32 {
        let fresh16 = ind;
        ind = ind + 1;
        let fresh17 = i;
        i = i + 1;
        *nword.offset(fresh17 as isize) = *w.offset(fresh16 as isize);
    }
    let fresh18 = ind;
    ind = ind + 1;
    let fresh19 = i;
    i = i + 1;
    *nword.offset(fresh19 as isize) = *w.offset(fresh18 as isize);

    t = expand_assignment_string_to_string(w.offset(ind as isize), 0 as libc::c_int);
    value = sh_single_quote(if !t.is_null() {
        t as *const libc::c_char
    } else {
        b"\0" as *const u8 as *const libc::c_char
    });
    libc::free(t as *mut libc::c_void);
    nword = libc::realloc(
        nword as *mut libc::c_void,
        ((wlen + 5 as libc::c_int) as libc::c_ulong).wrapping_add(STRLEN!(value) as libc::c_ulong)
            as usize,
    ) as *mut libc::c_char;
    strcpy(nword.offset(i as isize), value);

    libc::free(key as *mut libc::c_void);
    libc::free(value as *mut libc::c_void);

    return nword;
}

/* For each word in a compound array assignment, if the word looks like
[ind]=value, single-quote ind and value, but leave the brackets and
the = sign (and any `+') alone. If it's not an assignment, just single-
quote the word. This is used for indexed arrays. */
#[no_mangle]
pub unsafe extern "C" fn quote_compound_array_list(
    mut list: *mut WORD_LIST,
    mut type_0: libc::c_int,
) {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: *mut WORD_LIST = 0 as *mut WORD_LIST;

    l = list;
    while !l.is_null() {
        if !(((*l).word).is_null() || ((*(*l).word).word).is_null()) {
            if (*(*l).word).flags & W_ASSIGNMENT == 0 as libc::c_int {
                t = sh_single_quote((*(*l).word).word);
            } else {
                t = quote_compound_array_word((*(*l).word).word, type_0);
            }
            libc::free((*(*l).word).word as *mut libc::c_void);
            (*(*l).word).word = t;
        }
        l = (*l).next;
    }
}

/* For each word in a compound array assignment, if the word looks like
[ind]=value, quote globbing chars and characters in $IFS before the `='. */
unsafe extern "C" fn quote_array_assignment_chars(mut list: *mut WORD_LIST) {
    let mut nword: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: *mut WORD_LIST = 0 as *mut WORD_LIST;

    l = list;
    while !l.is_null() {
        if ((*l).word).is_null()
            || ((*(*l).word).word).is_null()
            || *((*(*l).word).word).offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {
            l = (*l).next;
            continue; /* should not happen, but just in case... */
        }
        /* Don't bother if it hasn't been recognized as an assignment or
        doesn't look like [ind]=value */
        if (*(*l).word).flags & W_ASSIGNMENT == 0 as libc::c_int {
            l = (*l).next;
            continue;
        }
        /* ] */
        if *((*(*l).word).word).offset(0 as libc::c_int as isize) as libc::c_int != '[' as i32
            || (mbschr((*(*l).word).word, '=' as i32)).is_null()
        {
            l = (*l).next;
            continue;
        }
        nword = quote_assign((*(*l).word).word);
        libc::free((*(*l).word).word as *mut libc::c_void);
        (*(*l).word).word = nword;
        (*(*l).word).flags |= W_NOGLOB as libc::c_int; /* XXX - W_NOSPLIT also? */

        l = (*l).next;
    }
}

/* skipsubscript moved to subst.c to use private functions. 2009/02/24. */

/* This function is called with SUB pointing to just after the beginning
`[' of an array subscript and removes the array element to which SUB
expands from array VAR.  A subscript of `*' or `@' unsets the array. */
/* If FLAGS&1 we don't expand the subscript; we just use it as-is. */
#[no_mangle]
pub unsafe extern "C" fn unbind_array_element(
    mut var: *mut SHELL_VAR,
    mut sub: *mut libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut ind: arrayind_t = 0;
    let mut akey: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ae: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;

    len = skipsubscript(
        sub,
        0 as libc::c_int,
        (flags & 1 as libc::c_int != 0 || !var.is_null() && assoc_p!(var) != 0) as libc::c_int,
    ); /* XXX */
    if *sub.offset(len as isize) as libc::c_int != ']' as i32 || len == 0 as libc::c_int {
        builtin_error(
            b"%s[%s: %s\0" as *const u8 as *const libc::c_char,
            (*var).name,
            sub,
            dcgettext(
                0 as *const libc::c_char,
                bash_badsub_errmsg,
                5 as libc::c_int,
            ),
        );
        return -(1 as libc::c_int);
    }
    *sub.offset(len as isize) = '\0' as i32 as libc::c_char;

    if ALL_ELEMENT_SUB!(*sub.offset(0 as libc::c_int as isize) as libc::c_int)
        && *sub.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
    {
        if array_p!(var) != 0 || assoc_p!(var) != 0 {
            unbind_variable((*var).name); /* XXX -- {array,assoc}_flush ? */
            return 0 as libc::c_int;
        } else {
            return -(2 as libc::c_int); /* don't allow this to unset scalar variables */
        }
    }

    if assoc_p!(var) != 0 {
        akey = if flags & 1 as libc::c_int != 0 {
            sub
        } else {
            expand_assignment_string_to_string(sub, 0 as libc::c_int)
        };
        if akey.is_null() || *akey as libc::c_int == 0 as libc::c_int {
            builtin_error(
                b"[%s]: %s\0" as *const u8 as *const libc::c_char,
                sub,
                dcgettext(
                    0 as *const libc::c_char,
                    bash_badsub_errmsg,
                    5 as libc::c_int,
                ),
            );
            FREE!(akey);
            return -(1 as libc::c_int);
        }

        assoc_remove(assoc_cell!(var), akey);
        if akey != sub {
            libc::free(akey as *mut libc::c_void);
        }
    } else if array_p!(var) != 0 {
        ind = array_expand_index(var, sub, len + 1 as libc::c_int, 0 as libc::c_int);
        /* negative subscripts to indexed arrays count back from end */
        if ind < 0 as libc::c_int as libc::c_long {
            ind = array_max_index!(array_cell!(var)) + 1 as libc::c_int as libc::c_long + ind;
        }
        if ind < 0 as libc::c_int as libc::c_long {
            builtin_error(
                b"[%s]: %s\0" as *const u8 as *const libc::c_char,
                sub,
                dcgettext(
                    0 as *const libc::c_char,
                    bash_badsub_errmsg,
                    5 as libc::c_int,
                ),
            );
            return -(1 as libc::c_int);
        }
        ae = array_remove((*var).value as *mut ARRAY, ind);
        if !ae.is_null() {
            array_dispose_element(ae);
        }
    } else {
        /* array_p (var) == 0 && assoc_p (var) == 0 */
        akey = this_command_name;
        ind = array_expand_index(var, sub, len + 1 as libc::c_int, 0 as libc::c_int);
        this_command_name = akey;
        if ind == 0 as libc::c_int as libc::c_long {
            unbind_variable((*var).name);
            return 0 as libc::c_int;
        } else {
            return -(2 as libc::c_int); /* any subscript other than 0 is invalid with scalar variables */
        }
    }

    return 0 as libc::c_int;
}

/* Format and output an array assignment in compound form VAR=(VALUES),
suitable for re-use as input. */
#[no_mangle]
pub unsafe extern "C" fn print_array_assignment(mut var: *mut SHELL_VAR, mut quoted: libc::c_int) {
    let mut vstr: *mut libc::c_char = 0 as *mut libc::c_char;

    vstr = array_to_assign(array_cell!(var), quoted);

    if vstr.is_null() {
        printf(
            b"%s=%s\n\0" as *const u8 as *const libc::c_char,
            (*var).name,
            if quoted != 0 {
                b"'()'\0" as *const u8 as *const libc::c_char
            } else {
                b"()\0" as *const u8 as *const libc::c_char
            },
        );
    } else {
        printf(
            b"%s=%s\n\0" as *const u8 as *const libc::c_char,
            (*var).name,
            vstr,
        );
        libc::free(vstr as *mut libc::c_void);
    };
}

/* Format and output an associative array assignment in compound form
VAR=(VALUES), suitable for re-use as input. */
#[no_mangle]
pub unsafe extern "C" fn print_assoc_assignment(mut var: *mut SHELL_VAR, mut quoted: libc::c_int) {
    let mut vstr: *mut libc::c_char = 0 as *mut libc::c_char;

    vstr = assoc_to_assign(assoc_cell!(var), quoted);

    if vstr.is_null() {
        printf(
            b"%s=%s\n\0" as *const u8 as *const libc::c_char,
            (*var).name,
            if quoted != 0 {
                b"'()'\0" as *const u8 as *const libc::c_char
            } else {
                b"()\0" as *const u8 as *const libc::c_char
            },
        );
    } else {
        printf(
            b"%s=%s\n\0" as *const u8 as *const libc::c_char,
            (*var).name,
            vstr,
        );
        libc::free(vstr as *mut libc::c_void);
    };
}

/***********************************************************************/
/*								       */
/* Utility functions to manage arrays and their contents for expansion */
/*								       */
/***********************************************************************/

/* Return 1 if NAME is a properly-formed array reference v[sub]. */

/* We need to reserve 1 for FLAGS, which we pass to skipsubscript. */
#[no_mangle]
pub unsafe extern "C" fn valid_array_reference(
    mut name: *const libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut isassoc: libc::c_int = 0;
    let mut entry: *mut SHELL_VAR = 0 as *mut SHELL_VAR;

    t = mbschr(name, '[' as i32); /* ] */
    isassoc = 0 as libc::c_int;
    if !t.is_null() {
        *t = '\0' as i32 as libc::c_char;
        r = legal_identifier(name);

        if flags & VA_NOEXPAND as libc::c_int != 0 {
            /* Don't waste a lookup if we don't need one */
            entry = find_variable(name);
            isassoc = (!entry.is_null() && assoc_p!(entry) != 0) as libc::c_int;
        }
        *t = '[' as i32 as libc::c_char;
        if r == 0 as libc::c_int {
            return 0 as libc::c_int;
        }

        if isassoc != 0
            && flags & (VA_NOEXPAND as libc::c_int | VA_ONEWORD as libc::c_int)
                == VA_NOEXPAND as libc::c_int | VA_ONEWORD as libc::c_int
        {
            len = (strlen(t)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        } else if isassoc != 0 {
            len = skipsubscript(t, 0 as libc::c_int, flags & VA_NOEXPAND as libc::c_int);
        /* VA_NOEXPAND must be 1 */
        } else {
            /* Check for a properly-terminated non-null subscript. */
            len = skipsubscript(t, 0 as libc::c_int, 0 as libc::c_int); /* arithmetic expression */
        }

        if *t.offset(len as isize) as libc::c_int != ']' as i32
            || len == 1 as libc::c_int
            || *t.offset((len + 1 as libc::c_int) as isize) as libc::c_int != '\0' as i32
        {
            return 0 as libc::c_int;
        }
        /* This allows blank subscripts */
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}

/* Expand the array index beginning at S and extending LEN characters. */
#[no_mangle]
pub unsafe extern "C" fn array_expand_index(
    mut var: *mut SHELL_VAR,
    mut s: *mut libc::c_char,
    mut len: libc::c_int,
    mut flags: libc::c_int,
) -> arrayind_t {
    let mut exp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut savecmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut expok: libc::c_int = 0;
    let mut val: arrayind_t = 0;

    exp = libc::malloc(len as usize) as *mut libc::c_char;
    strncpy(exp, s, (len - 1 as libc::c_int) as libc::c_ulong);
    *exp.offset((len - 1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;

    t = expand_arith_string(
        exp,
        Q_DOUBLE_QUOTES as libc::c_int | Q_ARITH as libc::c_int | Q_ARRAYSUB as libc::c_int,
    ); /* XXX - Q_ARRAYSUB for future use */
    savecmd = this_command_name;
    this_command_name = 0 as *mut libc::c_void as *mut libc::c_char;
    val = evalexp(t, EXP_EXPANDED as libc::c_int, &mut expok); /* XXX - was 0 but we expanded exp already */
    this_command_name = savecmd;
    if t != exp {
        libc::free(t as *mut libc::c_void);
    }
    libc::free(exp as *mut libc::c_void);
    if expok == 0 as libc::c_int {
        set_exit_status(EXECUTION_FAILURE as libc::c_int);
        if no_longjmp_on_fatal_error != 0 {
            return 0 as libc::c_int as arrayind_t;
        }
        top_level_cleanup();
        jump_to_top_level(DISCARD as libc::c_int);
    }
    return val;
}

/* Return the name of the variable specified by S without any subscript.
If SUBP is non-null, return a pointer to the start of the subscript
in *SUBP. If LENP is non-null, the length of the subscript is returned
in *LENP.  This returns newly-allocated memory. */
#[no_mangle]
pub unsafe extern "C" fn array_variable_name(
    mut s: *const libc::c_char,
    mut flags: libc::c_int,
    mut subp: *mut *mut libc::c_char,
    mut lenp: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ind: libc::c_int = 0;
    let mut ni: libc::c_int = 0;

    t = mbschr(s, '[' as i32);
    if t.is_null() {
        if !subp.is_null() {
            *subp = t;
        }
        if !lenp.is_null() {
            *lenp = 0 as libc::c_int;
        }
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    ind = t.offset_from(s) as libc::c_long as libc::c_int;
    ni = skipsubscript(s, ind, flags); /* XXX - was 0 not flags */
    if ni <= ind + 1 as libc::c_int || *s.offset(ni as isize) as libc::c_int != ']' as i32 {
        err_badarraysub(s);
        if !subp.is_null() {
            *subp = t;
        }
        if !lenp.is_null() {
            *lenp = 0 as libc::c_int;
        }
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }

    *t = '\0' as i32 as libc::c_char;
    ret = savestring!(s);
    let fresh20 = t;
    t = t.offset(1);
    *fresh20 = '[' as i32 as libc::c_char; /* ] */

    if !subp.is_null() {
        *subp = t;
    }
    if !lenp.is_null() {
        *lenp = ni - ind;
    }

    return ret;
}

/* Return the variable specified by S without any subscript.  If SUBP is
non-null, return a pointer to the start of the subscript in *SUBP.
If LENP is non-null, the length of the subscript is returned in *LENP. */
#[no_mangle]
pub unsafe extern "C" fn array_variable_part(
    mut s: *const libc::c_char,
    mut flags: libc::c_int,
    mut subp: *mut *mut libc::c_char,
    mut lenp: *mut libc::c_int,
) -> *mut SHELL_VAR {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    t = array_variable_name(s, flags, subp, lenp);

    if t.is_null() {
        return 0 as *mut libc::c_void as *mut SHELL_VAR;
    }
    var = find_variable(t); /* XXX - handle namerefs here? */

    libc::free(t as *mut libc::c_void);
    return var; /* now return invisible variables; caller must handle */
}