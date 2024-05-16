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