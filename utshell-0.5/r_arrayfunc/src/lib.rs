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