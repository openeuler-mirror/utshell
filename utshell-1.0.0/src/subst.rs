//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unreachable_code,
    non_snake_case,
    unreachable_patterns,
    unused_variables,
    clashing_extern_declarations
)]
use crate::arrayfunc::assign_array_element;
use crate::arrayfunc::convert_var_to_array;
use crate::assoc::assoc_to_word_list;
use crate::jobs::cleanup_the_pipeline;
use crate::jobs::procsub_add;
use crate::make_cmd::make_bare_word;
use crate::readline::bash_tilde_expand;
use crate::readline::wcslen;
use crate::sig::reset_terminating_signals;
use crate::trap::reset_signal_handlers;
use crate::trap::restore_original_signals;
//use std::io::stdin;
extern "C" {
    static mut stdin: *mut FILE;
}
use crate::array::array_to_string;
use crate::arrayfunc::array_expand_index;
use crate::arrayfunc::array_value;
use crate::arrayfunc::array_variable_part;
use crate::assoc::assoc_reference;
use crate::assoc::assoc_to_assign;
use crate::assoc::assoc_to_string;
use crate::builtins::set::set_shellopts;
use crate::dispose_cmd::dispose_word_desc;
use crate::error::err_badarraysub;
use crate::error::err_unboundvar;
use crate::jobs::wait_for;
use crate::readline::all_digits;
use crate::readline::array_modcase;
use crate::readline::array_patsub;
use crate::readline::array_reference;
use crate::readline::array_subrange;
use crate::sig::throw_to_top_level;
use crate::unwind_prot::add_unwind_protect;
use crate::unwind_prot::begin_unwind_frame;
use crate::unwind_prot::discard_unwind_frame;
use crate::variables::all_variables_matching_prefix;
use crate::variables::find_variable_last_nameref;
//use crate::builtins::common::number_of_args;
//use crate::flags::which_set_flags;
use crate::builtins::common::number_of_args;
use crate::expr::evalexp;
//use crate::builtins::common::number_of_args;
use crate::arrayfunc::array_keys;
use crate::arrayfunc::array_variable_name;
use crate::assoc::assoc_modcase;
use crate::assoc::assoc_patsub;
use crate::assoc::assoc_subrange;
use crate::assoc::assoc_to_kvpair;
use crate::builtins::setattr::var_attribute_string;
use crate::dispose_cmd::dispose_word;
use crate::flags::which_set_flags;
use crate::readline::array_to_assign;
use crate::readline::array_to_kvpair;
use crate::stringlib::strcreplace;
use crate::y_tab::decode_prompt_string;
//use crate::copycmd::copy_word_list;
use crate::execute_cmd::undo_partial_redirects;
use crate::pathexp::shell_glob_filename;
use crate::pathexp::unquoted_glob_pattern_p;
use crate::readline::bash_tilde_find_word;
use crate::sig::top_level_cleanup;
use crate::variables::set_pipestatus_from_exit;
//use crate::list::list_append;
use crate::arrayfunc::expand_and_quote_assoc_word;
use crate::arrayfunc::expand_and_quote_kvpair_word;
use crate::arrayfunc::kvpair_assignment_p;
use crate::arrayfunc::quote_compound_array_list;
use crate::brace::brace_expand;
use crate::builtins::common::find_shell_builtin;
use crate::builtins::common::find_special_builtin;
use crate::builtins::declare::declare_builtin;
use crate::copycmd::copy_word_list;
use crate::make_cmd::make_word_flags;
use crate::variables::assign_in_env;
use crate::variables::find_function;
use crate::y_tab::parse_string_to_word_list;
//use crate::pathexp::quote_string_for_globbing;
use crate::arrayfunc::valid_array_reference;
use crate::builtins::evalfile::sourcelevel;
use crate::execute_cmd::close_fd_bitmap;
use crate::execute_cmd::setup_async_signals;
use crate::flags::change_flag;
use crate::general::{legal_identifier, legal_number};
use crate::jobs::close_pgrp_pipe;
use crate::jobs::make_child;
use crate::jobs::procsub_clear;
use crate::jobs::restore_pipeline;
use crate::jobs::save_pipeline;
use crate::jobs::set_job_control;
use crate::jobs::set_sigchld_handler;
use crate::jobs::stop_making_children;
use crate::pathexp::quote_string_for_globbing;
use crate::readline::array_to_word_list;
use crate::readline::iswupper;
use crate::readline::move_to_high_fd;
use crate::readline::towlower;
use crate::readline::wcsdup;
use crate::readline::wcsrtombs;
use crate::sig::jump_to_top_level;
use crate::sig::termsig_handler;
use crate::src_common::*;
use crate::stringlib::strip_trailing;
use crate::stringlib::substring;
use crate::syntax::sh_syntaxtab;
use crate::trap::run_exit_trap;
use crate::trap::set_sigint_handler;
use crate::variables::flush_temporary_env;
use crate::variables::maybe_make_export_env;
use crate::y_tab::free_pushed_string_input;
//use crate::readline::__ctype_get_mb_cur_max;
use crate::arrayfunc::assign_array_from_string;
use crate::arrayfunc::assign_compound_array_list;
use crate::arrayfunc::convert_var_to_assoc;
use crate::arrayfunc::expand_compound_array_assignment;
use crate::dispose_cmd::dispose_words;
use crate::list::list_reverse;
use crate::make_cmd::alloc_word_desc;
use crate::make_cmd::make_word;
use crate::make_cmd::make_word_list;
use crate::print_cmd::xtrace_print_assignment;
use crate::readline::assignment;
use crate::readline::mbstowcs;
use crate::readline::mbtowc;
use crate::readline::wcschr;
use crate::variables::bind_variable;
use crate::variables::find_global_variable;
use crate::variables::find_variable;
use crate::variables::make_local_array_variable;
use crate::variables::make_local_assoc_variable;
use crate::variables::make_new_array_variable;
use crate::variables::make_new_assoc_variable;
use crate::variables::nameref_transform_name;
use crate::variables::stupidly_hack_special_variables;
use crate::version::shell_compatibility_level;
use crate::y_tab::xparse_dolparen;
#[feature(extern_types, linkage, register_tool)]
extern "C" {
    fn list_append(_: *mut WordList, _: *mut WordList) -> *mut GENERIC_LIST;
}
#[inline]
fn is_basic(mut c: libc::c_char) -> libc::c_int {
    unsafe {
        return (*is_basic_table
            .as_ptr()
            .offset((c as libc::c_uchar as libc::c_int >> 5 as libc::c_int) as isize)
            >> (c as libc::c_uchar as libc::c_int & 31 as libc::c_int)
            & 1 as libc::c_int as libc::c_uint) as libc::c_int;
    }
}
#[no_mangle]
//#[inline]
//#[linkage = "external"]
pub fn sub_append_string(
    mut source: *mut libc::c_char,
    mut target: *mut libc::c_char,
    mut indx: *mut libc::c_int,
    mut size: *mut size_t,
) -> *mut libc::c_char {
    unsafe {
        if !source.is_null() {
            let mut n: libc::c_int = 0;
            let mut srclen: size_t = 0;
            srclen = if !source.is_null()
                && *source.offset(0 as libc::c_int as isize) as libc::c_int != 0
            {
                if *source.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                    if *source.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                        strlen(source) as u64
                    } else {
                        2 as libc::c_int as libc::c_ulong
                    }
                } else {
                    1 as libc::c_int as libc::c_ulong
                }
            } else {
                0 as libc::c_int as libc::c_ulong
            };
            if srclen
                >= (*size).wrapping_sub(*indx as libc::c_ulong) as libc::c_int as libc::c_ulong
            {
                n = srclen.wrapping_add(*indx as libc::c_ulong) as libc::c_int;
                n = n + 128 as libc::c_int - n % 128 as libc::c_int;
                *size = n as size_t;
                target = sh_xrealloc(
                    target as *mut libc::c_void,
                    *size,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    749 as libc::c_int,
                ) as *mut libc::c_char;
            }
            libc::memcpy(
                target.offset(*indx as isize) as *mut libc::c_void,
                source as *const libc::c_void,
                srclen as libc::size_t,
            );
            *indx = (*indx as libc::c_ulong).wrapping_add(srclen) as libc::c_int as libc::c_int;
            *target.offset(*indx as isize) = '\u{0}' as i32 as libc::c_char;
            sh_xfree(
                source as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                756 as libc::c_int,
            );
        }
    }
    return target;
}
fn string_extract(
    mut string: *mut libc::c_char,
    mut sindex: *mut libc::c_int,
    mut charlist: *mut libc::c_char,
    mut flags: libc::c_int,
) -> *mut libc::c_char {
    unsafe {
        let mut c: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut found: libc::c_int = 0;
        let mut slen: size_t = 0;
        let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut state: mbstate_t = mbstate_t {
            __count: 0,
            __value: mbstate_t_value { __wch: 0 },
        };
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<mbstate_t>() as libc::c_ulong as usize,
        );
        slen = if __ctype_get_mb_cur_max() > 1 as libc::c_int as usize {
            (strlen(string.offset(*sindex as isize))).wrapping_add(*sindex as u64) as u64
        } else {
            0 as libc::c_int as libc::c_ulong
        };
        i = *sindex;
        found = 0 as libc::c_int;
        loop {
            c = *string.offset(i as isize) as libc::c_int;
            if !(c != 0) {
                break;
            }
            if c == '\\' as i32 {
                if !(*string.offset((i + 1 as libc::c_int) as isize) != 0) {
                    break;
                }
                i += 1;
            } else if flags & 0x2 as libc::c_int != 0 && c == '[' as i32 {
                let mut ni: libc::c_int = 0;
                ni = skipsubscript(string, i, 0 as libc::c_int);
                if *string.offset(ni as isize) as libc::c_int == ']' as i32 {
                    i = ni;
                }
            } else if c != 0
                && c == *charlist.offset(0 as libc::c_int as isize) as libc::c_int
                && *charlist.offset(1 as libc::c_int as isize) == 0
                || (if c != 0 {
                    (mbschr(charlist, c) != 0 as *mut libc::c_void as *mut libc::c_char)
                        as libc::c_int
                } else {
                    0 as libc::c_int
                }) != 0
            {
                found = 1 as libc::c_int;
                break;
            }
            if locale_mb_cur_max > 1 as libc::c_int {
                let mut state_bak: mbstate_t = mbstate_t {
                    __count: 0,
                    __value: mbstate_t_value { __wch: 0 },
                };
                let mut mblength: size_t = 0;
                let mut _f: libc::c_int = 0;
                _f = is_basic(*string.offset(i as isize));
                if _f != 0 {
                    mblength = 1 as libc::c_int as size_t;
                } else if locale_utf8locale != 0
                    && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                        == 0 as libc::c_int
                {
                    mblength = (*string.offset(i as isize) as libc::c_int != 0 as libc::c_int)
                        as libc::c_int as size_t;
                } else {
                    state_bak = state;
                    mblength = mbrlen(
                        string.offset(i as isize),
                        slen.wrapping_sub(i as libc::c_ulong),
                        &mut state,
                    );
                }
                if mblength == -(2 as libc::c_int) as size_t
                    || mblength == -(1 as libc::c_int) as size_t
                {
                    state = state_bak;
                    i += 1;
                } else if mblength == 0 as libc::c_int as libc::c_ulong {
                    i += 1;
                } else {
                    i = (i as libc::c_ulong).wrapping_add(mblength) as libc::c_int as libc::c_int;
                }
            } else {
                i += 1;
            }
        }
        if flags & 0x4 as libc::c_int != 0 && found == 0 as libc::c_int {
            *sindex = i;
            return &mut extract_string_error;
        }
        temp = if flags & 0x1 as libc::c_int != 0 {
            0 as *mut libc::c_void as *mut libc::c_char
        } else {
            substring(string, *sindex, i)
        };
        *sindex = i;

        return temp;
    }
}
fn string_extract_double_quoted(
    mut string: *mut libc::c_char,
    mut sindex: *mut libc::c_int,
    mut flags: libc::c_int,
) -> *mut libc::c_char {
    unsafe {
        let mut slen: size_t = 0;
        let mut send: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut j: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut t: libc::c_int = 0;
        let mut c: libc::c_uchar = 0;
        let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut pass_next: libc::c_int = 0;
        let mut backquote: libc::c_int = 0;
        let mut si: libc::c_int = 0;
        let mut dquote: libc::c_int = 0;
        let mut stripdq: libc::c_int = 0;
        let mut state: mbstate_t = mbstate_t {
            __count: 0,
            __value: mbstate_t_value { __wch: 0 },
        };
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<mbstate_t>() as usize,
        );
        slen = (strlen(string.offset(*sindex as isize))).wrapping_add(*sindex as u64) as u64;
        send = string.offset(slen as isize);
        stripdq = flags & 0x800 as libc::c_int;
        dquote = 0 as libc::c_int;
        backquote = dquote;
        pass_next = backquote;
        temp = sh_xmalloc(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_add(slen)
                .wrapping_sub(*sindex as libc::c_ulong),
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            874 as libc::c_int,
        ) as *mut libc::c_char;
        j = 0 as libc::c_int;
        i = *sindex;
        loop {
            c = *string.offset(i as isize) as libc::c_uchar;
            if !(c != 0) {
                break;
            }
            if pass_next != 0 {
                if stripdq == 0 as libc::c_int && c as libc::c_int != '"' as i32
                    || stripdq != 0
                        && (dquote != 0
                            && *sh_syntaxtab.as_mut_ptr().offset(c as isize) & 0x40 as libc::c_int
                                != 0
                            || dquote == 0 as libc::c_int)
                {
                    let fresh0 = j;
                    j = j + 1;
                    *temp.offset(fresh0 as isize) = '\\' as i32 as libc::c_char;
                }
                pass_next = 0 as libc::c_int;
            } else if c as libc::c_int == '\\' as i32 {
                pass_next += 1;
                i += 1;
                continue;
            } else if backquote != 0 {
                if c as libc::c_int == '`' as i32 {
                    backquote = 0 as libc::c_int;
                }
                let fresh5 = j;
                j = j + 1;
                *temp.offset(fresh5 as isize) = c as libc::c_char;
                i += 1;
                continue;
            } else if c as libc::c_int == '`' as i32 {
                let fresh6 = j;
                j = j + 1;
                *temp.offset(fresh6 as isize) = c as libc::c_char;
                backquote += 1;
                i += 1;
                continue;
            } else if c as libc::c_int == '$' as i32
                && (*string.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '(' as i32
                    || *string.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '{' as i32)
            {
                let mut free_ret: libc::c_int = 1 as libc::c_int;
                si = i + 2 as libc::c_int;
                if *string.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '(' as i32 {
                    ret = extract_command_subst(string, &mut si, flags & 0x400 as libc::c_int);
                } else {
                    ret = extract_dollar_brace_string(
                        string,
                        &mut si,
                        0x1 as libc::c_int,
                        0 as libc::c_int,
                    );
                }
                let fresh7 = j;
                j = j + 1;
                *temp.offset(fresh7 as isize) = '$' as i32 as libc::c_char;
                let fresh8 = j;
                j = j + 1;
                *temp.offset(fresh8 as isize) = *string.offset((i + 1 as libc::c_int) as isize);
                if ret.is_null() && no_longjmp_on_fatal_error != 0 {
                    free_ret = 0 as libc::c_int;
                    ret = string.offset(i as isize).offset(2 as libc::c_int as isize);
                }
                t = 0 as libc::c_int;
                while *ret.offset(t as isize) != 0 {
                    *temp.offset(j as isize) = *ret.offset(t as isize);
                    t += 1;
                    j += 1;
                }
                *temp.offset(j as isize) = *string.offset(si as isize);
                if si < i + 2 as libc::c_int {
                    i += 2 as libc::c_int;
                } else if *string.offset(si as isize) != 0 {
                    j += 1;
                    i = si + 1 as libc::c_int;
                } else {
                    i = si;
                }
                if free_ret != 0 {
                    sh_xfree(
                        ret as *mut libc::c_void,
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        986 as libc::c_int,
                    );
                }
                continue;
            } else if !(c as libc::c_int != '"' as i32) {
                if !(stripdq != 0) {
                    break;
                }
                dquote ^= 1 as libc::c_int;
                i += 1;
                continue;
            }
            if locale_mb_cur_max > 1 as libc::c_int {
                let mut state_bak: mbstate_t = mbstate_t {
                    __count: 0,
                    __value: mbstate_t_value { __wch: 0 },
                };
                let mut mblength: size_t = 0;
                let mut _k: libc::c_int = 0;
                _k = is_basic(*string.offset(i as isize));
                if _k != 0 {
                    mblength = 1 as libc::c_int as size_t;
                } else {
                    state_bak = state;
                    mblength = mbrlen(
                        string.offset(i as isize),
                        send.offset_from(string.offset(i as isize)) as libc::c_long as size_t,
                        &mut state,
                    );
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
                    let fresh1 = i;
                    i = i + 1;
                    let fresh2 = j;
                    j = j + 1;
                    *temp.offset(fresh2 as isize) = *string.offset(fresh1 as isize);
                    _k += 1;
                }
            } else {
                let fresh3 = i;
                i = i + 1;
                let fresh4 = j;
                j = j + 1;
                *temp.offset(fresh4 as isize) = *string.offset(fresh3 as isize);
            }
        }
        *temp.offset(j as isize) = '\u{0}' as i32 as libc::c_char;
        if c != 0 {
            i += 1;
        }
        *sindex = i;

        return temp;
    }
}
fn skip_double_quoted(
    mut string: *mut libc::c_char,
    mut slen: size_t,
    mut sind: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    unsafe {
        let mut c: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut pass_next: libc::c_int = 0;
        let mut backquote: libc::c_int = 0;
        let mut si: libc::c_int = 0;
        let mut state: mbstate_t = mbstate_t {
            __count: 0,
            __value: mbstate_t_value { __wch: 0 },
        };
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<mbstate_t>() as usize,
        );
        backquote = 0 as libc::c_int;
        pass_next = backquote;
        i = sind;
        loop {
            c = *string.offset(i as isize) as libc::c_int;
            if !(c != 0) {
                break;
            }
            if pass_next != 0 {
                pass_next = 0 as libc::c_int;
                if locale_mb_cur_max > 1 as libc::c_int {
                    let mut state_bak: mbstate_t = mbstate_t {
                        __count: 0,
                        __value: mbstate_t_value { __wch: 0 },
                    };
                    let mut mblength: size_t = 0;
                    let mut _f: libc::c_int = 0;
                    _f = is_basic(*string.offset(i as isize));
                    if _f != 0 {
                        mblength = 1 as libc::c_int as size_t;
                    } else if locale_utf8locale != 0
                        && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                            == 0 as libc::c_int
                    {
                        mblength = (*string.offset(i as isize) as libc::c_int != 0 as libc::c_int)
                            as libc::c_int as size_t;
                    } else {
                        state_bak = state;
                        mblength = mbrlen(
                            string.offset(i as isize),
                            slen.wrapping_sub(i as libc::c_ulong),
                            &mut state,
                        );
                    }
                    if mblength == -(2 as libc::c_int) as size_t
                        || mblength == -(1 as libc::c_int) as size_t
                    {
                        state = state_bak;
                        i += 1;
                    } else if mblength == 0 as libc::c_int as libc::c_ulong {
                        i += 1;
                    } else {
                        i = (i as libc::c_ulong).wrapping_add(mblength) as libc::c_int
                            as libc::c_int;
                    }
                } else {
                    i += 1;
                }
            } else if c == '\\' as i32 {
                pass_next += 1;
                i += 1;
            } else if backquote != 0 {
                if c == '`' as i32 {
                    backquote = 0 as libc::c_int;
                }
                if locale_mb_cur_max > 1 as libc::c_int {
                    let mut state_bak_0: mbstate_t = mbstate_t {
                        __count: 0,
                        __value: mbstate_t_value { __wch: 0 },
                    };
                    let mut mblength_0: size_t = 0;
                    let mut _f_0: libc::c_int = 0;
                    _f_0 = is_basic(*string.offset(i as isize));
                    if _f_0 != 0 {
                        mblength_0 = 1 as libc::c_int as size_t;
                    } else if locale_utf8locale != 0
                        && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                            == 0 as libc::c_int
                    {
                        mblength_0 = (*string.offset(i as isize) as libc::c_int != 0 as libc::c_int)
                            as libc::c_int as size_t;
                    } else {
                        state_bak_0 = state;
                        mblength_0 = mbrlen(
                            string.offset(i as isize),
                            slen.wrapping_sub(i as libc::c_ulong),
                            &mut state,
                        );
                    }
                    if mblength_0 == -(2 as libc::c_int) as size_t
                        || mblength_0 == -(1 as libc::c_int) as size_t
                    {
                        state = state_bak_0;
                        i += 1;
                    } else if mblength_0 == 0 as libc::c_int as libc::c_ulong {
                        i += 1;
                    } else {
                        i = (i as libc::c_ulong).wrapping_add(mblength_0) as libc::c_int
                            as libc::c_int;
                    }
                } else {
                    i += 1;
                }
            } else if c == '`' as i32 {
                backquote += 1;
                i += 1;
            } else if c == '$' as i32
                && (*string.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '(' as i32
                    || *string.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '{' as i32)
            {
                si = i + 2 as libc::c_int;
                if *string.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '(' as i32 {
                    ret = extract_command_subst(
                        string,
                        &mut si,
                        0x1 as libc::c_int | flags & 0x400 as libc::c_int,
                    );
                } else {
                    ret = extract_dollar_brace_string(
                        string,
                        &mut si,
                        0x1 as libc::c_int,
                        0x1 as libc::c_int,
                    );
                }
                if si as libc::c_ulong >= slen {
                    i = slen as libc::c_int;
                    c = 0 as libc::c_int;
                    break;
                } else {
                    i = si + 1 as libc::c_int;
                }
            } else {
                if !(c != '"' as i32) {
                    break;
                }
                if locale_mb_cur_max > 1 as libc::c_int {
                    let mut state_bak_1: mbstate_t = mbstate_t {
                        __count: 0,
                        __value: mbstate_t_value { __wch: 0 },
                    };
                    let mut mblength_1: size_t = 0;
                    let mut _f_1: libc::c_int = 0;
                    _f_1 = is_basic(*string.offset(i as isize));
                    if _f_1 != 0 {
                        mblength_1 = 1 as libc::c_int as size_t;
                    } else if locale_utf8locale != 0
                        && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                            == 0 as libc::c_int
                    {
                        mblength_1 = (*string.offset(i as isize) as libc::c_int != 0 as libc::c_int)
                            as libc::c_int as size_t;
                    } else {
                        state_bak_1 = state;
                        mblength_1 = mbrlen(
                            string.offset(i as isize),
                            slen.wrapping_sub(i as libc::c_ulong),
                            &mut state,
                        );
                    }
                    if mblength_1 == -(2 as libc::c_int) as size_t
                        || mblength_1 == -(1 as libc::c_int) as size_t
                    {
                        state = state_bak_1;
                        i += 1;
                    } else if mblength_1 == 0 as libc::c_int as libc::c_ulong {
                        i += 1;
                    } else {
                        i = (i as libc::c_ulong).wrapping_add(mblength_1) as libc::c_int
                            as libc::c_int;
                    }
                } else {
                    i += 1;
                }
            }
        }
        if c != 0 {
            i += 1;
        }

        return i;
    }
}
#[inline]
fn string_extract_single_quoted(
    mut string: *mut libc::c_char,
    mut sindex: *mut libc::c_int,
) -> *mut libc::c_char {
    unsafe {
        let mut i: libc::c_int = 0;
        let mut slen: size_t = 0;
        let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut state: mbstate_t = mbstate_t {
            __count: 0,
            __value: mbstate_t_value { __wch: 0 },
        };
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<mbstate_t>() as usize,
        );
        slen = if __ctype_get_mb_cur_max() > 1 as libc::c_int as usize {
            (strlen(string.offset(*sindex as isize))).wrapping_add(*sindex as libc::c_ulong as u64)
                as u64
        } else {
            0 as libc::c_int as libc::c_ulong
        };
        i = *sindex;
        while *string.offset(i as isize) as libc::c_int != 0
            && *string.offset(i as isize) as libc::c_int != '\'' as i32
        {
            if locale_mb_cur_max > 1 as libc::c_int {
                let mut state_bak: mbstate_t = mbstate_t {
                    __count: 0,
                    __value: mbstate_t_value { __wch: 0 },
                };
                let mut mblength: size_t = 0;
                let mut _f: libc::c_int = 0;
                _f = is_basic(*string.offset(i as isize));
                if _f != 0 {
                    mblength = 1 as libc::c_int as size_t;
                } else if locale_utf8locale != 0
                    && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                        == 0 as libc::c_int
                {
                    mblength = (*string.offset(i as isize) as libc::c_int != 0 as libc::c_int)
                        as libc::c_int as size_t;
                } else {
                    state_bak = state;
                    mblength = mbrlen(
                        string.offset(i as isize),
                        slen.wrapping_sub(i as libc::c_ulong),
                        &mut state,
                    );
                }
                if mblength == -(2 as libc::c_int) as size_t
                    || mblength == -(1 as libc::c_int) as size_t
                {
                    state = state_bak;
                    i += 1;
                } else if mblength == 0 as libc::c_int as libc::c_ulong {
                    i += 1;
                } else {
                    i = (i as libc::c_ulong).wrapping_add(mblength) as libc::c_int as libc::c_int;
                }
            } else {
                i += 1;
            }
        }
        t = substring(string, *sindex, i);
        if *string.offset(i as isize) != 0 {
            i += 1;
        }
        *sindex = i;

        return t;
    }
}
#[inline]
fn skip_single_quoted(
    mut string: *const libc::c_char,
    mut slen: size_t,
    mut sind: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    unsafe {
        let mut c: libc::c_int = 0;
        let mut state: mbstate_t = mbstate_t {
            __count: 0,
            __value: mbstate_t_value { __wch: 0 },
        };
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<mbstate_t>() as usize,
        );
        c = sind;
        while *string.offset(c as isize) as libc::c_int != 0
            && *string.offset(c as isize) as libc::c_int != '\'' as i32
        {
            if flags & 0x400 as libc::c_int != 0
                && *string.offset(c as isize) as libc::c_int == '\\' as i32
                && *string.offset((c + 1 as libc::c_int) as isize) as libc::c_int == '\'' as i32
                && *string.offset((c + 2 as libc::c_int) as isize) as libc::c_int != 0
            {
                if locale_mb_cur_max > 1 as libc::c_int {
                    let mut state_bak: mbstate_t = mbstate_t {
                        __count: 0,
                        __value: mbstate_t_value { __wch: 0 },
                    };
                    let mut mblength: size_t = 0;
                    let mut _f: libc::c_int = 0;
                    _f = is_basic(*string.offset(c as isize));
                    if _f != 0 {
                        mblength = 1 as libc::c_int as size_t;
                    } else if locale_utf8locale != 0
                        && *string.offset(c as isize) as libc::c_int & 0x80 as libc::c_int
                            == 0 as libc::c_int
                    {
                        mblength = (*string.offset(c as isize) as libc::c_int != 0 as libc::c_int)
                            as libc::c_int as size_t;
                    } else {
                        state_bak = state;
                        mblength = mbrlen(
                            string.offset(c as isize),
                            slen.wrapping_sub(c as libc::c_ulong),
                            &mut state,
                        );
                    }
                    if mblength == -(2 as libc::c_int) as size_t
                        || mblength == -(1 as libc::c_int) as size_t
                    {
                        state = state_bak;
                        c += 1;
                    } else if mblength == 0 as libc::c_int as libc::c_ulong {
                        c += 1;
                    } else {
                        c = (c as libc::c_ulong).wrapping_add(mblength) as libc::c_int
                            as libc::c_int;
                    }
                } else {
                    c += 1;
                }
            }
            if locale_mb_cur_max > 1 as libc::c_int {
                let mut state_bak_0: mbstate_t = mbstate_t {
                    __count: 0,
                    __value: mbstate_t_value { __wch: 0 },
                };
                let mut mblength_0: size_t = 0;
                let mut _f_0: libc::c_int = 0;
                _f_0 = is_basic(*string.offset(c as isize));
                if _f_0 != 0 {
                    mblength_0 = 1 as libc::c_int as size_t;
                } else if locale_utf8locale != 0
                    && *string.offset(c as isize) as libc::c_int & 0x80 as libc::c_int
                        == 0 as libc::c_int
                {
                    mblength_0 = (*string.offset(c as isize) as libc::c_int != 0 as libc::c_int)
                        as libc::c_int as size_t;
                } else {
                    state_bak_0 = state;
                    mblength_0 = mbrlen(
                        string.offset(c as isize),
                        slen.wrapping_sub(c as libc::c_ulong),
                        &mut state,
                    );
                }
                if mblength_0 == -(2 as libc::c_int) as size_t
                    || mblength_0 == -(1 as libc::c_int) as size_t
                {
                    state = state_bak_0;
                    c += 1;
                } else if mblength_0 == 0 as libc::c_int as libc::c_ulong {
                    c += 1;
                } else {
                    c = (c as libc::c_ulong).wrapping_add(mblength_0) as libc::c_int as libc::c_int;
                }
            } else {
                c += 1;
            }
        }
        if *string.offset(c as isize) != 0 {
            c += 1;
        }

        return c;
    }
}
fn string_extract_verbatim(
    mut string: *mut libc::c_char,
    mut slen: size_t,
    mut sindex: *mut libc::c_int,
    mut charlist: *mut libc::c_char,
    mut flags: libc::c_int,
) -> *mut libc::c_char {
    unsafe {
        let mut i: libc::c_int = 0;
        let mut wcharlist: *mut wchar_t = 0 as *mut wchar_t;
        let mut c: libc::c_int = 0;
        let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut state: mbstate_t = mbstate_t {
            __count: 0,
            __value: mbstate_t_value { __wch: 0 },
        };
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<mbstate_t>() as usize,
        );
        if flags & 0x10 as libc::c_int != 0
            && *charlist.offset(0 as libc::c_int as isize) as libc::c_int == '\'' as i32
            && *charlist.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
        {
            temp = string_extract_single_quoted(string, sindex);
            *sindex -= 1;
            return temp;
        }
        if *charlist as libc::c_int == 0 as libc::c_int {
            temp = string.offset(*sindex as isize);
            c = (if *sindex == 0 as libc::c_int {
                slen as usize
            } else if !temp.is_null() && *temp.offset(0 as libc::c_int as isize) as libc::c_int != 0
            {
                if *temp.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                    if *temp.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                        strlen(temp) as usize
                    } else {
                        2 as libc::c_int as usize
                    }
                } else {
                    1 as libc::c_int as usize
                }
            } else {
                0 as libc::c_int as usize
            }) as libc::c_int;
            temp = strcpy(
                sh_xmalloc(
                    (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(temp) as u64),
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    1173 as libc::c_int,
                ) as *mut libc::c_char,
                temp,
            );
            *sindex += c;
            return temp;
        }
        i = *sindex;
        wcharlist = 0 as *mut wchar_t;
        loop {
            c = *string.offset(i as isize) as libc::c_int;
            if !(c != 0) {
                break;
            }
            let mut mblength: size_t = 0;
            if flags & 0x10 as libc::c_int == 0 as libc::c_int && c == '\u{1}' as i32 {
                i += 2 as libc::c_int;
                if !(i as libc::c_ulong >= slen) {
                    continue;
                }
                i = slen as libc::c_int;
                c = 0 as libc::c_int;
                break;
            } else if flags & 0x20 as libc::c_int == 0 as libc::c_int
                && c == '\u{1}' as i32
                && *string.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '\u{7f}' as i32
            {
                i += 2 as libc::c_int;
                if !(i as libc::c_ulong >= slen) {
                    continue;
                }
                i = slen as libc::c_int;
                c = 0 as libc::c_int;
                break;
            } else {
                if locale_utf8locale != 0
                    && slen > i as libc::c_ulong
                    && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                        == 0 as libc::c_int
                {
                    mblength = (if *string.offset(i as isize) as libc::c_int != 0 as libc::c_int {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as size_t;
                } else {
                    mblength = (if __ctype_get_mb_cur_max() > 1 as libc::c_int as usize {
                        mblen(
                            string.offset(i as isize),
                            slen.wrapping_sub(i as libc::c_ulong),
                        )
                    } else {
                        1 as libc::c_int
                    }) as size_t;
                }
                if mblength > 1 as libc::c_int as libc::c_ulong {
                    let mut wc: wchar_t = 0;
                    mblength = mbtowc(
                        &mut wc,
                        string.offset(i as isize),
                        slen.wrapping_sub(i as libc::c_ulong) as usize,
                    ) as size_t;
                    if mblength == -(1 as libc::c_int) as size_t
                        || mblength == -(2 as libc::c_int) as size_t
                    {
                        if c != 0
                            && c == *charlist.offset(0 as libc::c_int as isize) as libc::c_int
                            && *charlist.offset(1 as libc::c_int as isize) == 0
                            || (if c != 0 {
                                (mbschr(charlist, c) != 0 as *mut libc::c_void as *mut libc::c_char)
                                    as libc::c_int
                            } else {
                                0 as libc::c_int
                            }) != 0
                        {
                            break;
                        }
                    } else {
                        if wcharlist.is_null() {
                            let mut len: size_t = 0;
                            len = mbstowcs(wcharlist, charlist, 0 as libc::c_int as usize) as u64;
                            if len == -(1 as libc::c_int) as libc::c_ulong {
                                len = 0 as libc::c_int as size_t;
                            }
                            wcharlist = sh_xmalloc(
                                (::std::mem::size_of::<wchar_t>() as libc::c_ulong).wrapping_mul(
                                    len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ),
                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                1225 as libc::c_int,
                            ) as *mut wchar_t;
                            mbstowcs(
                                wcharlist,
                                charlist,
                                len.wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
                            );
                        }
                        if !(wcschr(wcharlist, wc)).is_null() {
                            break;
                        }
                    }
                } else if c != 0
                    && c == *charlist.offset(0 as libc::c_int as isize) as libc::c_int
                    && *charlist.offset(1 as libc::c_int as isize) == 0
                    || (if c != 0 {
                        (mbschr(charlist, c) != 0 as *mut libc::c_void as *mut libc::c_char)
                            as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) != 0
                {
                    break;
                }
                if locale_mb_cur_max > 1 as libc::c_int {
                    let mut state_bak: mbstate_t = mbstate_t {
                        __count: 0,
                        __value: mbstate_t_value { __wch: 0 },
                    };
                    let mut mblength_0: size_t = 0;
                    let mut _f: libc::c_int = 0;
                    _f = is_basic(*string.offset(i as isize));
                    if _f != 0 {
                        mblength_0 = 1 as libc::c_int as size_t;
                    } else if locale_utf8locale != 0
                        && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                            == 0 as libc::c_int
                    {
                        mblength_0 = (*string.offset(i as isize) as libc::c_int != 0 as libc::c_int)
                            as libc::c_int as size_t;
                    } else {
                        state_bak = state;
                        mblength_0 = mbrlen(
                            string.offset(i as isize),
                            slen.wrapping_sub(i as libc::c_ulong),
                            &mut state,
                        );
                    }
                    if mblength_0 == -(2 as libc::c_int) as size_t
                        || mblength_0 == -(1 as libc::c_int) as size_t
                    {
                        state = state_bak;
                        i += 1;
                    } else if mblength_0 == 0 as libc::c_int as libc::c_ulong {
                        i += 1;
                    } else {
                        i = (i as libc::c_ulong).wrapping_add(mblength_0) as libc::c_int
                            as libc::c_int;
                    }
                } else {
                    i += 1;
                }
            }
        }
        if !wcharlist.is_null() {
            sh_xfree(
                wcharlist as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                1242 as libc::c_int,
            );
        }
        temp = substring(string, *sindex, i);
        *sindex = i;

        return temp;
    }
}
#[no_mangle]
pub fn extract_command_subst(
    mut string: *mut libc::c_char,
    mut sindex: *mut libc::c_int,
    mut xflags: libc::c_int,
) -> *mut libc::c_char {
    unsafe {
        let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
        if *string.offset(*sindex as isize) as libc::c_int == '(' as i32
            || xflags & 0x400 as libc::c_int != 0
        {
            return extract_delimited_string(
                string,
                sindex,
                b"$(\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"(\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b")\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                xflags | 0x8 as libc::c_int,
            );
        } else {
            xflags |= if no_longjmp_on_fatal_error != 0 {
                0x40 as libc::c_int
            } else {
                0 as libc::c_int
            };
            ret = xparse_dolparen(string, string.offset(*sindex as isize), sindex, xflags);
            return ret;
        };
    }
}
#[no_mangle]
pub fn extract_arithmetic_subst(
    mut string: *mut libc::c_char,
    mut sindex: *mut libc::c_int,
) -> *mut libc::c_char {
    return extract_delimited_string(
        string,
        sindex,
        b"$[\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"[\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"]\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub fn extract_process_subst(
    mut string: *mut libc::c_char,
    mut starter: *mut libc::c_char,
    mut sindex: *mut libc::c_int,
    mut xflags: libc::c_int,
) -> *mut libc::c_char {
    unsafe {
        xflags |= if no_longjmp_on_fatal_error != 0 {
            0x40 as libc::c_int
        } else {
            0 as libc::c_int
        };
        return xparse_dolparen(string, string.offset(*sindex as isize), sindex, xflags);
    }
}
#[no_mangle]
pub fn extract_array_assignment_list(
    mut string: *mut libc::c_char,
    mut sindex: *mut libc::c_int,
) -> *mut libc::c_char {
    unsafe {
        let mut slen: libc::c_int = 0;
        let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
        slen = strlen(string) as libc::c_int;
        if *string.offset((slen - 1 as libc::c_int) as isize) as libc::c_int == ')' as i32 {
            ret = substring(string, *sindex, slen - 1 as libc::c_int);
            *sindex = slen - 1 as libc::c_int;
            return ret;
        }
    }
    return 0 as *mut libc::c_char;
}
fn extract_delimited_string(
    mut string: *mut libc::c_char,
    mut sindex: *mut libc::c_int,
    mut opener: *mut libc::c_char,
    mut alt_opener: *mut libc::c_char,
    mut closer: *mut libc::c_char,
    mut flags: libc::c_int,
) -> *mut libc::c_char {
    unsafe {
        let mut i: libc::c_int = 0;
        let mut c: libc::c_int = 0;
        let mut si: libc::c_int = 0;
        let mut slen: size_t = 0;
        let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut pass_character: libc::c_int = 0;
        let mut nesting_level: libc::c_int = 0;
        let mut in_comment: libc::c_int = 0;
        let mut len_closer: libc::c_int = 0;
        let mut len_opener: libc::c_int = 0;
        let mut len_alt_opener: libc::c_int = 0;
        let mut state: mbstate_t = mbstate_t {
            __count: 0,
            __value: mbstate_t_value { __wch: 0 },
        };
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<mbstate_t>() as usize,
        );
        slen = (strlen(string.offset(*sindex as isize))).wrapping_add(*sindex as u64) as u64;
        len_opener = (if !opener.is_null()
            && *opener.offset(0 as libc::c_int as isize) as libc::c_int != 0
        {
            if *opener.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                if *opener.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                    strlen(opener) as usize
                } else {
                    2 as libc::c_int as usize
                }
            } else {
                1 as libc::c_int as usize
            }
        } else {
            0 as libc::c_int as usize
        }) as libc::c_int;
        len_alt_opener = (if !alt_opener.is_null()
            && *alt_opener.offset(0 as libc::c_int as isize) as libc::c_int != 0
        {
            if *alt_opener.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                if *alt_opener.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                    strlen(alt_opener) as usize
                } else {
                    2 as libc::c_int as usize
                }
            } else {
                1 as libc::c_int as usize
            }
        } else {
            0 as libc::c_int as usize
        }) as libc::c_int;
        len_closer = (if !closer.is_null()
            && *closer.offset(0 as libc::c_int as isize) as libc::c_int != 0
        {
            if *closer.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                if *closer.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                    strlen(closer) as usize
                } else {
                    2 as libc::c_int as usize
                }
            } else {
                1 as libc::c_int as usize
            }
        } else {
            0 as libc::c_int as usize
        }) as libc::c_int;
        in_comment = 0 as libc::c_int;
        pass_character = in_comment;
        nesting_level = 1 as libc::c_int;
        i = *sindex;
        while nesting_level != 0 {
            c = *string.offset(i as isize) as libc::c_int;
            if i as libc::c_ulong > slen {
                i = slen as libc::c_int;
                i = slen as libc::c_int;
                c = *string.offset(i as isize) as libc::c_int;
                break;
            } else {
                if c == 0 as libc::c_int {
                    break;
                }
                if in_comment != 0 {
                    if c == '\n' as i32 {
                        in_comment = 0 as libc::c_int;
                    }
                    if locale_mb_cur_max > 1 as libc::c_int {
                        let mut state_bak: mbstate_t = mbstate_t {
                            __count: 0,
                            __value: mbstate_t_value { __wch: 0 },
                        };
                        let mut mblength: size_t = 0;
                        let mut _f: libc::c_int = 0;
                        _f = is_basic(*string.offset(i as isize));
                        if _f != 0 {
                            mblength = 1 as libc::c_int as size_t;
                        } else if locale_utf8locale != 0
                            && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                                == 0 as libc::c_int
                        {
                            mblength = (*string.offset(i as isize) as libc::c_int
                                != 0 as libc::c_int)
                                as libc::c_int as size_t;
                        } else {
                            state_bak = state;
                            mblength = mbrlen(
                                string.offset(i as isize),
                                slen.wrapping_sub(i as libc::c_ulong),
                                &mut state,
                            );
                        }
                        if mblength == -(2 as libc::c_int) as size_t
                            || mblength == -(1 as libc::c_int) as size_t
                        {
                            state = state_bak;
                            i += 1;
                        } else if mblength == 0 as libc::c_int as libc::c_ulong {
                            i += 1;
                        } else {
                            i = (i as libc::c_ulong).wrapping_add(mblength) as libc::c_int
                                as libc::c_int;
                        }
                    } else {
                        i += 1;
                    }
                } else if pass_character != 0 {
                    pass_character = 0 as libc::c_int;
                    if locale_mb_cur_max > 1 as libc::c_int {
                        let mut state_bak_0: mbstate_t = mbstate_t {
                            __count: 0,
                            __value: mbstate_t_value { __wch: 0 },
                        };
                        let mut mblength_0: size_t = 0;
                        let mut _f_0: libc::c_int = 0;
                        _f_0 = is_basic(*string.offset(i as isize));
                        if _f_0 != 0 {
                            mblength_0 = 1 as libc::c_int as size_t;
                        } else if locale_utf8locale != 0
                            && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                                == 0 as libc::c_int
                        {
                            mblength_0 = (*string.offset(i as isize) as libc::c_int
                                != 0 as libc::c_int)
                                as libc::c_int as size_t;
                        } else {
                            state_bak_0 = state;
                            mblength_0 = mbrlen(
                                string.offset(i as isize),
                                slen.wrapping_sub(i as libc::c_ulong),
                                &mut state,
                            );
                        }
                        if mblength_0 == -(2 as libc::c_int) as size_t
                            || mblength_0 == -(1 as libc::c_int) as size_t
                        {
                            state = state_bak_0;
                            i += 1;
                        } else if mblength_0 == 0 as libc::c_int as libc::c_ulong {
                            i += 1;
                        } else {
                            i = (i as libc::c_ulong).wrapping_add(mblength_0) as libc::c_int
                                as libc::c_int;
                        }
                    } else {
                        i += 1;
                    }
                } else if flags & 0x8 as libc::c_int != 0
                    && c == '#' as i32
                    && (i == 0 as libc::c_int
                        || *string.offset((i - 1 as libc::c_int) as isize) as libc::c_int
                            == '\n' as i32
                        || *sh_syntaxtab.as_mut_ptr().offset(
                            *string.offset((i - 1 as libc::c_int) as isize) as libc::c_uchar
                                as isize,
                        ) & 0x2000 as libc::c_int
                            != 0)
                {
                    in_comment = 1 as libc::c_int;
                    if locale_mb_cur_max > 1 as libc::c_int {
                        let mut state_bak_1: mbstate_t = mbstate_t {
                            __count: 0,
                            __value: mbstate_t_value { __wch: 0 },
                        };
                        let mut mblength_1: size_t = 0;
                        let mut _f_1: libc::c_int = 0;
                        _f_1 = is_basic(*string.offset(i as isize));
                        if _f_1 != 0 {
                            mblength_1 = 1 as libc::c_int as size_t;
                        } else if locale_utf8locale != 0
                            && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                                == 0 as libc::c_int
                        {
                            mblength_1 = (*string.offset(i as isize) as libc::c_int
                                != 0 as libc::c_int)
                                as libc::c_int as size_t;
                        } else {
                            state_bak_1 = state;
                            mblength_1 = mbrlen(
                                string.offset(i as isize),
                                slen.wrapping_sub(i as libc::c_ulong),
                                &mut state,
                            );
                        }
                        if mblength_1 == -(2 as libc::c_int) as size_t
                            || mblength_1 == -(1 as libc::c_int) as size_t
                        {
                            state = state_bak_1;
                            i += 1;
                        } else if mblength_1 == 0 as libc::c_int as libc::c_ulong {
                            i += 1;
                        } else {
                            i = (i as libc::c_ulong).wrapping_add(mblength_1) as libc::c_int
                                as libc::c_int;
                        }
                    } else {
                        i += 1;
                    }
                } else if c == '\u{1}' as i32 || c == '\\' as i32 {
                    pass_character += 1;
                    i += 1;
                } else if flags & 0x8 as libc::c_int != 0
                    && *string.offset(i as isize) as libc::c_int == '$' as i32
                    && *string.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '(' as i32
                {
                    si = i + 2 as libc::c_int;
                    t = extract_command_subst(string, &mut si, flags | 0x1 as libc::c_int);
                    if si as libc::c_ulong >= slen {
                        i = slen as libc::c_int;
                        c = 0 as libc::c_int;
                        break;
                    } else {
                        i = si + 1 as libc::c_int;
                    }
                } else if if len_opener == 0 as libc::c_int {
                    1 as libc::c_int
                } else {
                    (*string.offset(i as isize).offset(0 as libc::c_int as isize) as libc::c_int
                        == *opener.offset(0 as libc::c_int as isize) as libc::c_int
                        && strncmp(string.offset(i as isize), opener, len_opener as usize)
                            == 0 as libc::c_int) as libc::c_int
                } != 0
                {
                    si = i + len_opener;
                    t = extract_delimited_string(
                        string,
                        &mut si,
                        opener,
                        alt_opener,
                        closer,
                        flags | 0x1 as libc::c_int,
                    );
                    if si as libc::c_ulong >= slen {
                        i = slen as libc::c_int;
                        c = 0 as libc::c_int;
                        break;
                    } else {
                        i = si + 1 as libc::c_int;
                    }
                } else if len_alt_opener != 0
                    && (if len_alt_opener == 0 as libc::c_int {
                        1 as libc::c_int
                    } else {
                        (*string.offset(i as isize).offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == *alt_opener.offset(0 as libc::c_int as isize) as libc::c_int
                            && strncmp(
                                string.offset(i as isize),
                                alt_opener,
                                len_alt_opener as usize,
                            ) == 0 as libc::c_int) as libc::c_int
                    }) != 0
                {
                    si = i + len_alt_opener;
                    t = extract_delimited_string(
                        string,
                        &mut si,
                        alt_opener,
                        alt_opener,
                        closer,
                        flags | 0x1 as libc::c_int,
                    );
                    if si as libc::c_ulong >= slen {
                        i = slen as libc::c_int;
                        c = 0 as libc::c_int;
                        break;
                    } else {
                        i = si + 1 as libc::c_int;
                    }
                } else {
                    if if len_closer == 0 as libc::c_int {
                        1 as libc::c_int
                    } else {
                        (*string.offset(i as isize).offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == *closer.offset(0 as libc::c_int as isize) as libc::c_int
                            && strncmp(string.offset(i as isize), closer, len_closer as usize)
                                == 0 as libc::c_int) as libc::c_int
                    } != 0
                    {
                        i += len_closer - 1 as libc::c_int;
                        nesting_level -= 1;
                        if nesting_level == 0 as libc::c_int {
                            break;
                        }
                    }
                    if c == '`' as i32 {
                        si = i + 1 as libc::c_int;
                        t = string_extract(
                            string,
                            &mut si,
                            b"`\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            flags | 0x1 as libc::c_int,
                        );
                        if si as libc::c_ulong >= slen {
                            i = slen as libc::c_int;
                            c = 0 as libc::c_int;
                            break;
                        } else {
                            i = si + 1 as libc::c_int;
                        }
                    } else if c == '\'' as i32 || c == '"' as i32 {
                        si = i + 1 as libc::c_int;
                        i = if c == '\'' as i32 {
                            skip_single_quoted(string, slen, si, 0 as libc::c_int)
                        } else {
                            skip_double_quoted(string, slen, si, 0 as libc::c_int)
                        };
                    } else if locale_mb_cur_max > 1 as libc::c_int {
                        let mut state_bak_2: mbstate_t = mbstate_t {
                            __count: 0,
                            __value: mbstate_t_value { __wch: 0 },
                        };
                        let mut mblength_2: size_t = 0;
                        let mut _f_2: libc::c_int = 0;
                        _f_2 = is_basic(*string.offset(i as isize));
                        if _f_2 != 0 {
                            mblength_2 = 1 as libc::c_int as size_t;
                        } else if locale_utf8locale != 0
                            && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                                == 0 as libc::c_int
                        {
                            mblength_2 = (*string.offset(i as isize) as libc::c_int
                                != 0 as libc::c_int)
                                as libc::c_int as size_t;
                        } else {
                            state_bak_2 = state;
                            mblength_2 = mbrlen(
                                string.offset(i as isize),
                                slen.wrapping_sub(i as libc::c_ulong),
                                &mut state,
                            );
                        }
                        if mblength_2 == -(2 as libc::c_int) as size_t
                            || mblength_2 == -(1 as libc::c_int) as size_t
                        {
                            state = state_bak_2;
                            i += 1;
                        } else if mblength_2 == 0 as libc::c_int as libc::c_ulong {
                            i += 1;
                        } else {
                            i = (i as libc::c_ulong).wrapping_add(mblength_2) as libc::c_int
                                as libc::c_int;
                        }
                    } else {
                        i += 1;
                    }
                }
            }
        }
        if c == 0 as libc::c_int && nesting_level != 0 {
            if no_longjmp_on_fatal_error == 0 as libc::c_int {
                ::std::ptr::write_volatile(
                    &mut last_command_exit_value as *mut libc::c_int,
                    1 as libc::c_int,
                );
                report_error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"bad substitution: no closing `%s' in %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    closer,
                    string,
                );
                exp_jump_to_top_level(2 as libc::c_int);
            } else {
                *sindex = i;
                return 0 as *mut libc::c_void as *mut libc::c_char;
            }
        }
        si = i - *sindex - len_closer + 1 as libc::c_int;
        if flags & 0x1 as libc::c_int != 0 {
            result = 0 as *mut libc::c_void as *mut libc::c_char;
        } else {
            result = sh_xmalloc(
                (1 as libc::c_int + si) as size_t,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                1491 as libc::c_int,
            ) as *mut libc::c_char;
            strncpy(result, string.offset(*sindex as isize), si as usize);
            *result.offset(si as isize) = '\u{0}' as i32 as libc::c_char;
        }
        *sindex = i;

        return result;
    }
}
fn extract_dollar_brace_string(
    mut string: *mut libc::c_char,
    mut sindex: *mut libc::c_int,
    mut quoted: libc::c_int,
    mut flags: libc::c_int,
) -> *mut libc::c_char {
    unsafe {
        let mut i: libc::c_int = 0;
        let mut c: libc::c_int = 0;
        let mut slen: size_t = 0;
        let mut pass_character: libc::c_int = 0;
        let mut nesting_level: libc::c_int = 0;
        let mut si: libc::c_int = 0;
        let mut dolbrace_state: libc::c_int = 0;
        let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut state: mbstate_t = mbstate_t {
            __count: 0,
            __value: mbstate_t_value { __wch: 0 },
        };
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<mbstate_t>() as usize,
        );
        pass_character = 0 as libc::c_int;
        nesting_level = 1 as libc::c_int;
        slen = (strlen(string.offset(*sindex as isize))).wrapping_add(*sindex as u64) as u64;
        dolbrace_state = if flags & 0x200 as libc::c_int != 0 {
            0x4 as libc::c_int
        } else {
            0x1 as libc::c_int
        };
        if quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0
            && flags & 0x100 as libc::c_int != 0
        {
            dolbrace_state = 0x40 as libc::c_int;
        }
        i = *sindex;
        loop {
            c = *string.offset(i as isize) as libc::c_int;
            if !(c != 0) {
                break;
            }
            if pass_character != 0 {
                pass_character = 0 as libc::c_int;
                if locale_mb_cur_max > 1 as libc::c_int {
                    let mut state_bak: mbstate_t = mbstate_t {
                        __count: 0,
                        __value: mbstate_t_value { __wch: 0 },
                    };
                    let mut mblength: size_t = 0;
                    let mut _f: libc::c_int = 0;
                    _f = is_basic(*string.offset(i as isize));
                    if _f != 0 {
                        mblength = 1 as libc::c_int as size_t;
                    } else if locale_utf8locale != 0
                        && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                            == 0 as libc::c_int
                    {
                        mblength = (*string.offset(i as isize) as libc::c_int != 0 as libc::c_int)
                            as libc::c_int as size_t;
                    } else {
                        state_bak = state;
                        mblength = mbrlen(
                            string.offset(i as isize),
                            slen.wrapping_sub(i as libc::c_ulong),
                            &mut state,
                        );
                    }
                    if mblength == -(2 as libc::c_int) as size_t
                        || mblength == -(1 as libc::c_int) as size_t
                    {
                        state = state_bak;
                        i += 1;
                    } else if mblength == 0 as libc::c_int as libc::c_ulong {
                        i += 1;
                    } else {
                        i = (i as libc::c_ulong).wrapping_add(mblength) as libc::c_int
                            as libc::c_int;
                    }
                } else {
                    i += 1;
                }
            } else if c == '\u{1}' as i32 || c == '\\' as i32 {
                pass_character += 1;
                i += 1;
            } else if *string.offset(i as isize) as libc::c_int == '$' as i32
                && *string.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '{' as i32
            {
                nesting_level += 1;
                i += 2 as libc::c_int;
            } else if c == '}' as i32 {
                nesting_level -= 1;
                if nesting_level == 0 as libc::c_int {
                    break;
                }
                i += 1;
            } else if c == '`' as i32 {
                si = i + 1 as libc::c_int;
                t = string_extract(
                    string,
                    &mut si,
                    b"`\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    flags | 0x1 as libc::c_int,
                );
                if si as libc::c_ulong >= slen {
                    i = slen as libc::c_int;
                    c = 0 as libc::c_int;
                    break;
                } else {
                    i = si + 1 as libc::c_int;
                }
            } else if *string.offset(i as isize) as libc::c_int == '$' as i32
                && *string.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '(' as i32
            {
                si = i + 2 as libc::c_int;
                t = extract_command_subst(string, &mut si, flags | 0x1 as libc::c_int);
                if si as libc::c_ulong >= slen {
                    i = slen as libc::c_int;
                    c = 0 as libc::c_int;
                    break;
                } else {
                    i = si + 1 as libc::c_int;
                }
            } else if (*string.offset(i as isize) as libc::c_int == '<' as i32
                || *string.offset(i as isize) as libc::c_int == '>' as i32)
                && *string.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '(' as i32
            {
                si = i + 2 as libc::c_int;
                t = extract_process_subst(
                    string,
                    (if *string.offset(i as isize) as libc::c_int == '<' as i32 {
                        b"<(\0" as *const u8 as *const libc::c_char
                    } else {
                        b">)\0" as *const u8 as *const libc::c_char
                    }) as *mut libc::c_char,
                    &mut si,
                    flags | 0x1 as libc::c_int,
                );
                if si as libc::c_ulong >= slen {
                    i = slen as libc::c_int;
                    c = 0 as libc::c_int;
                    break;
                } else {
                    i = si + 1 as libc::c_int;
                }
            } else if c == '"' as i32 {
                si = i + 1 as libc::c_int;
                i = skip_double_quoted(string, slen, si, 0 as libc::c_int);
            } else if c == '\'' as i32 {
                if posixly_correct != 0
                    && shell_compatibility_level > 42 as libc::c_int
                    && dolbrace_state != 0x40 as libc::c_int
                    && quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0
                {
                    if locale_mb_cur_max > 1 as libc::c_int {
                        let mut state_bak_0: mbstate_t = mbstate_t {
                            __count: 0,
                            __value: mbstate_t_value { __wch: 0 },
                        };
                        let mut mblength_0: size_t = 0;
                        let mut _f_0: libc::c_int = 0;
                        _f_0 = is_basic(*string.offset(i as isize));
                        if _f_0 != 0 {
                            mblength_0 = 1 as libc::c_int as size_t;
                        } else if locale_utf8locale != 0
                            && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                                == 0 as libc::c_int
                        {
                            mblength_0 = (*string.offset(i as isize) as libc::c_int
                                != 0 as libc::c_int)
                                as libc::c_int as size_t;
                        } else {
                            state_bak_0 = state;
                            mblength_0 = mbrlen(
                                string.offset(i as isize),
                                slen.wrapping_sub(i as libc::c_ulong),
                                &mut state,
                            );
                        }
                        if mblength_0 == -(2 as libc::c_int) as size_t
                            || mblength_0 == -(1 as libc::c_int) as size_t
                        {
                            state = state_bak_0;
                            i += 1;
                        } else if mblength_0 == 0 as libc::c_int as libc::c_ulong {
                            i += 1;
                        } else {
                            i = (i as libc::c_ulong).wrapping_add(mblength_0) as libc::c_int
                                as libc::c_int;
                        }
                    } else {
                        i += 1;
                    }
                } else {
                    si = i + 1 as libc::c_int;
                    i = skip_single_quoted(string, slen, si, 0 as libc::c_int);
                }
            } else {
                if c == '[' as i32 && dolbrace_state == 0x1 as libc::c_int {
                    si = skipsubscript(string, i, 0 as libc::c_int);
                    if si as libc::c_ulong >= slen {
                        i = slen as libc::c_int;
                        c = 0 as libc::c_int;
                        break;
                    } else if *string.offset(si as isize) as libc::c_int == ']' as i32 {
                        i = si;
                        c = *string.offset(i as isize) as libc::c_int;
                    }
                }
                if locale_mb_cur_max > 1 as libc::c_int {
                    let mut state_bak_1: mbstate_t = mbstate_t {
                        __count: 0,
                        __value: mbstate_t_value { __wch: 0 },
                    };
                    let mut mblength_1: size_t = 0;
                    let mut _f_1: libc::c_int = 0;
                    _f_1 = is_basic(*string.offset(i as isize));
                    if _f_1 != 0 {
                        mblength_1 = 1 as libc::c_int as size_t;
                    } else if locale_utf8locale != 0
                        && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                            == 0 as libc::c_int
                    {
                        mblength_1 = (*string.offset(i as isize) as libc::c_int != 0 as libc::c_int)
                            as libc::c_int as size_t;
                    } else {
                        state_bak_1 = state;
                        mblength_1 = mbrlen(
                            string.offset(i as isize),
                            slen.wrapping_sub(i as libc::c_ulong),
                            &mut state,
                        );
                    }
                    if mblength_1 == -(2 as libc::c_int) as size_t
                        || mblength_1 == -(1 as libc::c_int) as size_t
                    {
                        state = state_bak_1;
                        i += 1;
                    } else if mblength_1 == 0 as libc::c_int as libc::c_ulong {
                        i += 1;
                    } else {
                        i = (i as libc::c_ulong).wrapping_add(mblength_1) as libc::c_int
                            as libc::c_int;
                    }
                } else {
                    i += 1;
                }
                if dolbrace_state == 0x1 as libc::c_int
                    && c == '%' as i32
                    && i - *sindex > 1 as libc::c_int
                {
                    dolbrace_state = 0x40 as libc::c_int;
                } else if dolbrace_state == 0x1 as libc::c_int
                    && c == '#' as i32
                    && i - *sindex > 1 as libc::c_int
                {
                    dolbrace_state = 0x40 as libc::c_int;
                } else if dolbrace_state == 0x1 as libc::c_int
                    && c == '/' as i32
                    && i - *sindex > 1 as libc::c_int
                {
                    dolbrace_state = 0x80 as libc::c_int;
                } else if dolbrace_state == 0x1 as libc::c_int
                    && c == '^' as i32
                    && i - *sindex > 1 as libc::c_int
                {
                    dolbrace_state = 0x40 as libc::c_int;
                } else if dolbrace_state == 0x1 as libc::c_int
                    && c == ',' as i32
                    && i - *sindex > 1 as libc::c_int
                {
                    dolbrace_state = 0x40 as libc::c_int;
                } else if dolbrace_state == 0x1 as libc::c_int
                    && !(strchr(b"#%^,~:-=?+/\0" as *const u8 as *const libc::c_char, c)).is_null()
                {
                    dolbrace_state = 0x2 as libc::c_int;
                } else if dolbrace_state == 0x2 as libc::c_int
                    && (strchr(b"#%^,~:-=?+/\0" as *const u8 as *const libc::c_char, c)).is_null()
                {
                    dolbrace_state = 0x4 as libc::c_int;
                }
            }
        }
        if c == 0 as libc::c_int && nesting_level != 0 {
            if no_longjmp_on_fatal_error == 0 as libc::c_int {
                ::std::ptr::write_volatile(
                    &mut last_command_exit_value as *mut libc::c_int,
                    1 as libc::c_int,
                );
                report_error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"bad substitution: no closing `%s' in %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    b"}\0" as *const u8 as *const libc::c_char,
                    string,
                );
                exp_jump_to_top_level(2 as libc::c_int);
            } else {
                *sindex = i;
                return 0 as *mut libc::c_void as *mut libc::c_char;
            }
        }
        result = if flags & 0x1 as libc::c_int != 0 {
            0 as *mut libc::c_void as *mut libc::c_char
        } else {
            substring(string, *sindex, i)
        };
        *sindex = i;

        return result;
    }
}
#[no_mangle]
pub fn de_backslash(mut string: *mut libc::c_char) -> *mut libc::c_char {
    unsafe {
        let mut slen: size_t = 0;
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut prev_i: libc::c_int = 0;
        let mut state: mbstate_t = mbstate_t {
            __count: 0,
            __value: mbstate_t_value { __wch: 0 },
        };
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<mbstate_t>() as usize,
        );
        slen = strlen(string) as u64;
        j = 0 as libc::c_int;
        i = j;
        while (i as libc::c_ulong) < slen {
            if *string.offset(i as isize) as libc::c_int == '\\' as i32
                && (*string.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '`' as i32
                    || *string.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                        == '\\' as i32
                    || *string.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '$' as i32)
            {
                i += 1;
            }
            prev_i = i;
            if locale_mb_cur_max > 1 as libc::c_int {
                let mut state_bak: mbstate_t = mbstate_t {
                    __count: 0,
                    __value: mbstate_t_value { __wch: 0 },
                };
                let mut mblength: size_t = 0;
                let mut _f: libc::c_int = 0;
                _f = is_basic(*string.offset(i as isize));
                if _f != 0 {
                    mblength = 1 as libc::c_int as size_t;
                } else if locale_utf8locale != 0
                    && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                        == 0 as libc::c_int
                {
                    mblength = (*string.offset(i as isize) as libc::c_int != 0 as libc::c_int)
                        as libc::c_int as size_t;
                } else {
                    state_bak = state;
                    mblength = mbrlen(
                        string.offset(i as isize),
                        slen.wrapping_sub(i as libc::c_ulong),
                        &mut state,
                    );
                }
                if mblength == -(2 as libc::c_int) as size_t
                    || mblength == -(1 as libc::c_int) as size_t
                {
                    state = state_bak;
                    i += 1;
                } else if mblength == 0 as libc::c_int as libc::c_ulong {
                    i += 1;
                } else {
                    i = (i as libc::c_ulong).wrapping_add(mblength) as libc::c_int as libc::c_int;
                }
            } else {
                i += 1;
            }
            if j < prev_i {
                loop {
                    let fresh9 = prev_i;
                    prev_i = prev_i + 1;
                    let fresh10 = j;
                    j = j + 1;
                    *string.offset(fresh10 as isize) = *string.offset(fresh9 as isize);
                    if !(prev_i < i) {
                        break;
                    }
                }
            } else {
                j = i;
            }
        }
        *string.offset(j as isize) = '\u{0}' as i32 as libc::c_char;
    }
    return string;
}
fn skip_matched_pair(
    mut string: *const libc::c_char,
    mut start: libc::c_int,
    mut open: libc::c_int,
    mut close_0: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    unsafe {
        let mut i: libc::c_int = 0;
        let mut pass_next: libc::c_int = 0;
        let mut backq: libc::c_int = 0;
        let mut si: libc::c_int = 0;
        let mut c: libc::c_int = 0;
        let mut count: libc::c_int = 0;
        let mut oldjmp: libc::c_int = 0;
        let mut slen: size_t = 0;
        let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ss: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut state: mbstate_t = mbstate_t {
            __count: 0,
            __value: mbstate_t_value { __wch: 0 },
        };
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<mbstate_t>() as usize,
        );
        slen = (strlen(string.offset(start as isize))).wrapping_add(start as u64) as u64;
        oldjmp = no_longjmp_on_fatal_error;
        no_longjmp_on_fatal_error = 1 as libc::c_int;
        i = start + 1 as libc::c_int;
        count = 1 as libc::c_int;
        backq = 0 as libc::c_int;
        pass_next = backq;
        ss = string as *mut libc::c_char;
        loop {
            c = *string.offset(i as isize) as libc::c_int;
            if !(c != 0) {
                break;
            }
            if pass_next != 0 {
                pass_next = 0 as libc::c_int;
                if c == 0 as libc::c_int {
                    no_longjmp_on_fatal_error = oldjmp;
                    return i;
                }
                if locale_mb_cur_max > 1 as libc::c_int {
                    let mut state_bak: mbstate_t = mbstate_t {
                        __count: 0,
                        __value: mbstate_t_value { __wch: 0 },
                    };
                    let mut mblength: size_t = 0;
                    let mut _f: libc::c_int = 0;
                    _f = is_basic(*string.offset(i as isize));
                    if _f != 0 {
                        mblength = 1 as libc::c_int as size_t;
                    } else if locale_utf8locale != 0
                        && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                            == 0 as libc::c_int
                    {
                        mblength = (*string.offset(i as isize) as libc::c_int != 0 as libc::c_int)
                            as libc::c_int as size_t;
                    } else {
                        state_bak = state;
                        mblength = mbrlen(
                            string.offset(i as isize),
                            slen.wrapping_sub(i as libc::c_ulong),
                            &mut state,
                        );
                    }
                    if mblength == -(2 as libc::c_int) as size_t
                        || mblength == -(1 as libc::c_int) as size_t
                    {
                        state = state_bak;
                        i += 1;
                    } else if mblength == 0 as libc::c_int as libc::c_ulong {
                        i += 1;
                    } else {
                        i = (i as libc::c_ulong).wrapping_add(mblength) as libc::c_int
                            as libc::c_int;
                    }
                } else {
                    i += 1;
                }
            } else if flags & 1 as libc::c_int == 0 as libc::c_int && c == '\\' as i32 {
                pass_next = 1 as libc::c_int;
                i += 1;
            } else if backq != 0 {
                if c == '`' as i32 {
                    backq = 0 as libc::c_int;
                }
                if locale_mb_cur_max > 1 as libc::c_int {
                    let mut state_bak_0: mbstate_t = mbstate_t {
                        __count: 0,
                        __value: mbstate_t_value { __wch: 0 },
                    };
                    let mut mblength_0: size_t = 0;
                    let mut _f_0: libc::c_int = 0;
                    _f_0 = is_basic(*string.offset(i as isize));
                    if _f_0 != 0 {
                        mblength_0 = 1 as libc::c_int as size_t;
                    } else if locale_utf8locale != 0
                        && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                            == 0 as libc::c_int
                    {
                        mblength_0 = (*string.offset(i as isize) as libc::c_int != 0 as libc::c_int)
                            as libc::c_int as size_t;
                    } else {
                        state_bak_0 = state;
                        mblength_0 = mbrlen(
                            string.offset(i as isize),
                            slen.wrapping_sub(i as libc::c_ulong),
                            &mut state,
                        );
                    }
                    if mblength_0 == -(2 as libc::c_int) as size_t
                        || mblength_0 == -(1 as libc::c_int) as size_t
                    {
                        state = state_bak_0;
                        i += 1;
                    } else if mblength_0 == 0 as libc::c_int as libc::c_ulong {
                        i += 1;
                    } else {
                        i = (i as libc::c_ulong).wrapping_add(mblength_0) as libc::c_int
                            as libc::c_int;
                    }
                } else {
                    i += 1;
                }
            } else if flags & 1 as libc::c_int == 0 as libc::c_int && c == '`' as i32 {
                backq = 1 as libc::c_int;
                i += 1;
            } else if flags & 1 as libc::c_int == 0 as libc::c_int && c == open {
                count += 1;
                i += 1;
            } else if c == close_0 {
                count -= 1;
                if count == 0 as libc::c_int {
                    break;
                }
                i += 1;
            } else if flags & 1 as libc::c_int == 0 as libc::c_int
                && (c == '\'' as i32 || c == '"' as i32)
            {
                i = if c == '\'' as i32 {
                    i += 1;
                    skip_single_quoted(ss, slen, i, 0 as libc::c_int)
                } else {
                    i += 1;
                    skip_double_quoted(ss, slen, i, 0 as libc::c_int)
                };
            } else if flags & 1 as libc::c_int == 0 as libc::c_int
                && c == '$' as i32
                && (*string.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '(' as i32
                    || *string.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '{' as i32)
            {
                si = i + 2 as libc::c_int;
                if *string.offset(si as isize) as libc::c_int == '\u{0}' as i32 {
                    no_longjmp_on_fatal_error = oldjmp;
                    return si;
                }
                if *string.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '(' as i32 {
                    temp = extract_delimited_string(
                        ss,
                        &mut si,
                        b"$(\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        b"(\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        b")\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        0x1 as libc::c_int | 0x8 as libc::c_int,
                    );
                } else {
                    temp = extract_dollar_brace_string(
                        ss,
                        &mut si,
                        0 as libc::c_int,
                        0x1 as libc::c_int,
                    );
                }
                if si as libc::c_ulong >= slen {
                    i = slen as libc::c_int;
                    c = 0 as libc::c_int;
                    break;
                } else {
                    i = si;
                    if *string.offset(i as isize) as libc::c_int == '\u{0}' as i32 {
                        break;
                    }
                    i += 1;
                }
            } else if locale_mb_cur_max > 1 as libc::c_int {
                let mut state_bak_1: mbstate_t = mbstate_t {
                    __count: 0,
                    __value: mbstate_t_value { __wch: 0 },
                };
                let mut mblength_1: size_t = 0;
                let mut _f_1: libc::c_int = 0;
                _f_1 = is_basic(*string.offset(i as isize));
                if _f_1 != 0 {
                    mblength_1 = 1 as libc::c_int as size_t;
                } else if locale_utf8locale != 0
                    && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                        == 0 as libc::c_int
                {
                    mblength_1 = (*string.offset(i as isize) as libc::c_int != 0 as libc::c_int)
                        as libc::c_int as size_t;
                } else {
                    state_bak_1 = state;
                    mblength_1 = mbrlen(
                        string.offset(i as isize),
                        slen.wrapping_sub(i as libc::c_ulong),
                        &mut state,
                    );
                }
                if mblength_1 == -(2 as libc::c_int) as size_t
                    || mblength_1 == -(1 as libc::c_int) as size_t
                {
                    state = state_bak_1;
                    i += 1;
                } else if mblength_1 == 0 as libc::c_int as libc::c_ulong {
                    i += 1;
                } else {
                    i = (i as libc::c_ulong).wrapping_add(mblength_1) as libc::c_int as libc::c_int;
                }
            } else {
                i += 1;
            }
        }
        no_longjmp_on_fatal_error = oldjmp;

        return i;
    }
}
#[no_mangle]
pub fn skipsubscript(
    mut string: *const libc::c_char,
    mut start: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    return skip_matched_pair(string, start, '[' as i32, ']' as i32, flags);
}
#[no_mangle]
pub fn skip_to_delim(
    mut string: *mut libc::c_char,
    mut start: libc::c_int,
    mut delims: *mut libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    unsafe {
        let mut i: libc::c_int = 0;
        let mut pass_next: libc::c_int = 0;
        let mut backq: libc::c_int = 0;
        let mut dquote: libc::c_int = 0;
        let mut si: libc::c_int = 0;
        let mut c: libc::c_int = 0;
        let mut oldjmp: libc::c_int = 0;
        let mut invert: libc::c_int = 0;
        let mut skipquote: libc::c_int = 0;
        let mut skipcmd: libc::c_int = 0;
        let mut noprocsub: libc::c_int = 0;
        let mut completeflag: libc::c_int = 0;
        let mut arithexp: libc::c_int = 0;
        let mut skipcol: libc::c_int = 0;
        let mut slen: size_t = 0;
        let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut open: [libc::c_char; 3] = [0; 3];
        let mut state: mbstate_t = mbstate_t {
            __count: 0,
            __value: mbstate_t_value { __wch: 0 },
        };
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<mbstate_t>() as usize,
        );
        slen = (strlen(string.offset(start as isize))).wrapping_add(start as u64) as u64;
        oldjmp = no_longjmp_on_fatal_error;
        if flags & 0x1 as libc::c_int != 0 {
            no_longjmp_on_fatal_error = 1 as libc::c_int;
        }
        invert = flags & 0x2 as libc::c_int;
        skipcmd = (flags & 0x8 as libc::c_int == 0 as libc::c_int) as libc::c_int;
        noprocsub = flags & 0x80 as libc::c_int;
        completeflag = if flags & 0x100 as libc::c_int != 0 {
            0x400 as libc::c_int
        } else {
            0 as libc::c_int
        };
        arithexp = flags & 0x400 as libc::c_int;
        skipcol = 0 as libc::c_int;
        i = start;
        dquote = 0 as libc::c_int;
        backq = dquote;
        pass_next = backq;
        loop {
            c = *string.offset(i as isize) as libc::c_int;
            if !(c != 0) {
                break;
            }
            skipquote = (flags & 0x4 as libc::c_int != 0 && (c == '\'' as i32 || c == '"' as i32))
                as libc::c_int;
            if pass_next != 0 {
                pass_next = 0 as libc::c_int;
                if c == 0 as libc::c_int {
                    no_longjmp_on_fatal_error = oldjmp;
                    return i;
                }
                if locale_mb_cur_max > 1 as libc::c_int {
                    let mut state_bak: mbstate_t = mbstate_t {
                        __count: 0,
                        __value: mbstate_t_value { __wch: 0 },
                    };
                    let mut mblength: size_t = 0;
                    let mut _f: libc::c_int = 0;
                    _f = is_basic(*string.offset(i as isize));
                    if _f != 0 {
                        mblength = 1 as libc::c_int as size_t;
                    } else if locale_utf8locale != 0
                        && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                            == 0 as libc::c_int
                    {
                        mblength = (*string.offset(i as isize) as libc::c_int != 0 as libc::c_int)
                            as libc::c_int as size_t;
                    } else {
                        state_bak = state;
                        mblength = mbrlen(
                            string.offset(i as isize),
                            slen.wrapping_sub(i as libc::c_ulong),
                            &mut state,
                        );
                    }
                    if mblength == -(2 as libc::c_int) as size_t
                        || mblength == -(1 as libc::c_int) as size_t
                    {
                        state = state_bak;
                        i += 1;
                    } else if mblength == 0 as libc::c_int as libc::c_ulong {
                        i += 1;
                    } else {
                        i = (i as libc::c_ulong).wrapping_add(mblength) as libc::c_int
                            as libc::c_int;
                    }
                } else {
                    i += 1;
                }
            } else if c == '\\' as i32 {
                pass_next = 1 as libc::c_int;
                i += 1;
            } else if backq != 0 {
                if c == '`' as i32 {
                    backq = 0 as libc::c_int;
                }
                if locale_mb_cur_max > 1 as libc::c_int {
                    let mut state_bak_0: mbstate_t = mbstate_t {
                        __count: 0,
                        __value: mbstate_t_value { __wch: 0 },
                    };
                    let mut mblength_0: size_t = 0;
                    let mut _f_0: libc::c_int = 0;
                    _f_0 = is_basic(*string.offset(i as isize));
                    if _f_0 != 0 {
                        mblength_0 = 1 as libc::c_int as size_t;
                    } else if locale_utf8locale != 0
                        && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                            == 0 as libc::c_int
                    {
                        mblength_0 = (*string.offset(i as isize) as libc::c_int != 0 as libc::c_int)
                            as libc::c_int as size_t;
                    } else {
                        state_bak_0 = state;
                        mblength_0 = mbrlen(
                            string.offset(i as isize),
                            slen.wrapping_sub(i as libc::c_ulong),
                            &mut state,
                        );
                    }
                    if mblength_0 == -(2 as libc::c_int) as size_t
                        || mblength_0 == -(1 as libc::c_int) as size_t
                    {
                        state = state_bak_0;
                        i += 1;
                    } else if mblength_0 == 0 as libc::c_int as libc::c_ulong {
                        i += 1;
                    } else {
                        i = (i as libc::c_ulong).wrapping_add(mblength_0) as libc::c_int
                            as libc::c_int;
                    }
                } else {
                    i += 1;
                }
            } else if c == '`' as i32 {
                backq = 1 as libc::c_int;
                i += 1;
            } else if arithexp != 0 && skipcol != 0 && c == ':' as i32 {
                skipcol -= 1;
                i += 1;
            } else if arithexp != 0 && c == '?' as i32 {
                skipcol += 1;
                i += 1;
            } else {
                if skipquote == 0 as libc::c_int
                    && invert == 0 as libc::c_int
                    && (if c != 0 {
                        (mbschr(delims, c) != 0 as *mut libc::c_void as *mut libc::c_char)
                            as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) != 0
                {
                    break;
                }
                if completeflag != 0
                    && i > 0 as libc::c_int
                    && *string.offset((i - 1 as libc::c_int) as isize) as libc::c_int == '$' as i32
                    && c == '\'' as i32
                {
                    i += 1;
                    i = skip_single_quoted(string, slen, i, 0x400 as libc::c_int);
                } else if c == '\'' as i32 {
                    i += 1;
                    i = skip_single_quoted(string, slen, i, 0 as libc::c_int);
                } else if c == '"' as i32 {
                    i += 1;
                    i = skip_double_quoted(string, slen, i, completeflag);
                } else if c == '(' as i32 && arithexp != 0 {
                    si = i + 1 as libc::c_int;
                    if *string.offset(si as isize) as libc::c_int == '\u{0}' as i32 {
                        no_longjmp_on_fatal_error = oldjmp;
                        return si;
                    }
                    temp = extract_delimited_string(
                        string,
                        &mut si,
                        b"(\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        b"(\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        b")\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        0x1 as libc::c_int,
                    );
                    i = si;
                    if *string.offset(i as isize) as libc::c_int == '\u{0}' as i32 {
                        break;
                    } else {
                        i += 1;
                    }
                } else if c == '$' as i32
                    && (skipcmd != 0
                        && *string.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                            == '(' as i32
                        || *string.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                            == '{' as i32)
                {
                    si = i + 2 as libc::c_int;
                    if *string.offset(si as isize) as libc::c_int == '\u{0}' as i32 {
                        no_longjmp_on_fatal_error = oldjmp;
                        return si;
                    }
                    if *string.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '(' as i32
                    {
                        temp = extract_delimited_string(
                            string,
                            &mut si,
                            b"$(\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            b"(\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            b")\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            0x1 as libc::c_int | 0x8 as libc::c_int,
                        );
                    } else {
                        temp = extract_dollar_brace_string(
                            string,
                            &mut si,
                            0 as libc::c_int,
                            0x1 as libc::c_int,
                        );
                    }
                    if si as libc::c_ulong >= slen {
                        i = slen as libc::c_int;
                        c = 0 as libc::c_int;
                        break;
                    } else {
                        i = si;
                        if *string.offset(i as isize) as libc::c_int == '\u{0}' as i32 {
                            break;
                        } else {
                            i += 1;
                        }
                    }
                } else if skipcmd != 0
                    && noprocsub == 0 as libc::c_int
                    && (c == '<' as i32 || c == '>' as i32)
                    && *string.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '(' as i32
                {
                    si = i + 2 as libc::c_int;
                    if *string.offset(si as isize) as libc::c_int == '\u{0}' as i32 {
                        no_longjmp_on_fatal_error = oldjmp;
                        return si;
                    }
                    temp = extract_delimited_string(
                        string,
                        &mut si,
                        (if c == '<' as i32 {
                            b"<(\0" as *const u8 as *const libc::c_char
                        } else {
                            b">(\0" as *const u8 as *const libc::c_char
                        }) as *mut libc::c_char,
                        b"(\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        b")\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        0x8 as libc::c_int | 0x1 as libc::c_int,
                    );
                    if si as libc::c_ulong >= slen {
                        i = slen as libc::c_int;
                        c = 0 as libc::c_int;
                        break;
                    } else {
                        i = si;
                        if *string.offset(i as isize) as libc::c_int == '\u{0}' as i32 {
                            break;
                        }
                        i += 1;
                    }
                } else if flags & 0x10 as libc::c_int != 0
                    && extended_glob != 0
                    && *string.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '(' as i32
                    && (if c != 0 {
                        (mbschr(b"?*+!@\0" as *const u8 as *const libc::c_char, c)
                            != 0 as *mut libc::c_void as *mut libc::c_char)
                            as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) != 0
                {
                    si = i + 2 as libc::c_int;
                    if *string.offset(si as isize) as libc::c_int == '\u{0}' as i32 {
                        no_longjmp_on_fatal_error = oldjmp;
                        return si;
                    }
                    open[0 as libc::c_int as usize] = c as libc::c_char;
                    open[1 as libc::c_int as usize] = '(' as i32 as libc::c_char;
                    open[2 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
                    temp = extract_delimited_string(
                        string,
                        &mut si,
                        open.as_mut_ptr(),
                        b"(\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        b")\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        0x1 as libc::c_int,
                    );
                    if si as libc::c_ulong >= slen {
                        i = slen as libc::c_int;
                        c = 0 as libc::c_int;
                        break;
                    } else {
                        i = si;
                        if *string.offset(i as isize) as libc::c_int == '\u{0}' as i32 {
                            break;
                        } else {
                            i += 1;
                        }
                    }
                } else if flags & 0x40 as libc::c_int != 0 && c == '[' as i32 {
                    si = i + 1 as libc::c_int;
                    if *string.offset(si as isize) as libc::c_int == '\u{0}' as i32 {
                        no_longjmp_on_fatal_error = oldjmp;
                        return si;
                    }
                    temp = extract_delimited_string(
                        string,
                        &mut si,
                        b"[\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        b"[\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        b"]\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        0x1 as libc::c_int,
                    );
                    i = si;
                    if *string.offset(i as isize) as libc::c_int == '\u{0}' as i32 {
                        break;
                    }
                    i += 1;
                } else {
                    if (skipquote != 0 || invert != 0)
                        && (if c != 0 {
                            (mbschr(delims, c) != 0 as *mut libc::c_void as *mut libc::c_char)
                                as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) == 0 as libc::c_int
                    {
                        break;
                    }
                    if locale_mb_cur_max > 1 as libc::c_int {
                        let mut state_bak_1: mbstate_t = mbstate_t {
                            __count: 0,
                            __value: mbstate_t_value { __wch: 0 },
                        };
                        let mut mblength_1: size_t = 0;
                        let mut _f_1: libc::c_int = 0;
                        _f_1 = is_basic(*string.offset(i as isize));
                        if _f_1 != 0 {
                            mblength_1 = 1 as libc::c_int as size_t;
                        } else if locale_utf8locale != 0
                            && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                                == 0 as libc::c_int
                        {
                            mblength_1 = (*string.offset(i as isize) as libc::c_int
                                != 0 as libc::c_int)
                                as libc::c_int as size_t;
                        } else {
                            state_bak_1 = state;
                            mblength_1 = mbrlen(
                                string.offset(i as isize),
                                slen.wrapping_sub(i as libc::c_ulong),
                                &mut state,
                            );
                        }
                        if mblength_1 == -(2 as libc::c_int) as size_t
                            || mblength_1 == -(1 as libc::c_int) as size_t
                        {
                            state = state_bak_1;
                            i += 1;
                        } else if mblength_1 == 0 as libc::c_int as libc::c_ulong {
                            i += 1;
                        } else {
                            i = (i as libc::c_ulong).wrapping_add(mblength_1) as libc::c_int
                                as libc::c_int;
                        }
                    } else {
                        i += 1;
                    }
                }
            }
        }
        no_longjmp_on_fatal_error = oldjmp;

        return i;
    }
}
#[no_mangle]
pub fn skip_to_histexp(
    mut string: *mut libc::c_char,
    mut start: libc::c_int,
    mut delims: *mut libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    unsafe {
        let mut i: libc::c_int = 0;
        let mut pass_next: libc::c_int = 0;
        let mut backq: libc::c_int = 0;
        let mut dquote: libc::c_int = 0;
        let mut c: libc::c_int = 0;
        let mut oldjmp: libc::c_int = 0;
        let mut histexp_comsub: libc::c_int = 0;
        let mut histexp_backq: libc::c_int = 0;
        let mut old_dquote: libc::c_int = 0;
        let mut slen: size_t = 0;
        let mut state: mbstate_t = mbstate_t {
            __count: 0,
            __value: mbstate_t_value { __wch: 0 },
        };
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<mbstate_t>() as usize,
        );
        slen = (strlen(string.offset(start as isize))).wrapping_add(start as u64) as u64;
        oldjmp = no_longjmp_on_fatal_error;
        if flags & 0x1 as libc::c_int != 0 {
            no_longjmp_on_fatal_error = 1 as libc::c_int;
        }
        old_dquote = 0 as libc::c_int;
        histexp_backq = old_dquote;
        histexp_comsub = histexp_backq;
        i = start;
        dquote = 0 as libc::c_int;
        backq = dquote;
        pass_next = backq;
        loop {
            c = *string.offset(i as isize) as libc::c_int;
            if !(c != 0) {
                break;
            }
            if pass_next != 0 {
                pass_next = 0 as libc::c_int;
                if c == 0 as libc::c_int {
                    no_longjmp_on_fatal_error = oldjmp;
                    return i;
                }
                if locale_mb_cur_max > 1 as libc::c_int {
                    let mut state_bak: mbstate_t = mbstate_t {
                        __count: 0,
                        __value: mbstate_t_value { __wch: 0 },
                    };
                    let mut mblength: size_t = 0;
                    let mut _f: libc::c_int = 0;
                    _f = is_basic(*string.offset(i as isize));
                    if _f != 0 {
                        mblength = 1 as libc::c_int as size_t;
                    } else if locale_utf8locale != 0
                        && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                            == 0 as libc::c_int
                    {
                        mblength = (*string.offset(i as isize) as libc::c_int != 0 as libc::c_int)
                            as libc::c_int as size_t;
                    } else {
                        state_bak = state;
                        mblength = mbrlen(
                            string.offset(i as isize),
                            slen.wrapping_sub(i as libc::c_ulong),
                            &mut state,
                        );
                    }
                    if mblength == -(2 as libc::c_int) as size_t
                        || mblength == -(1 as libc::c_int) as size_t
                    {
                        state = state_bak;
                        i += 1;
                    } else if mblength == 0 as libc::c_int as libc::c_ulong {
                        i += 1;
                    } else {
                        i = (i as libc::c_ulong).wrapping_add(mblength) as libc::c_int
                            as libc::c_int;
                    }
                } else {
                    i += 1;
                }
            } else if c == '\\' as i32 {
                pass_next = 1 as libc::c_int;
                i += 1;
            } else if backq != 0 && c == '`' as i32 {
                backq = 0 as libc::c_int;
                histexp_backq -= 1;
                dquote = old_dquote;
                i += 1;
            } else if c == '`' as i32 {
                backq = 1 as libc::c_int;
                histexp_backq += 1;
                old_dquote = dquote;
                dquote = 0 as libc::c_int;
                i += 1;
            } else if dquote != 0
                && c == *delims.offset(0 as libc::c_int as isize) as libc::c_int
                && *string.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '"' as i32
            {
                i += 1;
            } else {
                if c == *delims.offset(0 as libc::c_int as isize) as libc::c_int {
                    break;
                }
                if dquote != 0 && c == '\'' as i32 {
                    i += 1;
                } else if c == '\'' as i32 {
                    i += 1;
                    i = skip_single_quoted(string, slen, i, 0 as libc::c_int);
                } else if posixly_correct == 0 as libc::c_int && c == '"' as i32 {
                    dquote = 1 as libc::c_int - dquote;
                    i += 1;
                } else if c == '"' as i32 {
                    i += 1;
                    i = skip_double_quoted(string, slen, i, 0 as libc::c_int);
                } else if (c == '$' as i32 || c == '<' as i32 || c == '>' as i32)
                    && *string.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '(' as i32
                    && *string.offset((i + 2 as libc::c_int) as isize) as libc::c_int != '(' as i32
                {
                    if *string.offset((i + 2 as libc::c_int) as isize) as libc::c_int
                        == '\u{0}' as i32
                    {
                        no_longjmp_on_fatal_error = oldjmp;
                        return i + 2 as libc::c_int;
                    }
                    i += 2 as libc::c_int;
                    histexp_comsub += 1;
                    old_dquote = dquote;
                    dquote = 0 as libc::c_int;
                } else if histexp_comsub != 0 && c == ')' as i32 {
                    histexp_comsub -= 1;
                    dquote = old_dquote;
                    i += 1;
                } else if backq != 0 {
                    if locale_mb_cur_max > 1 as libc::c_int {
                        let mut state_bak_0: mbstate_t = mbstate_t {
                            __count: 0,
                            __value: mbstate_t_value { __wch: 0 },
                        };
                        let mut mblength_0: size_t = 0;
                        let mut _f_0: libc::c_int = 0;
                        _f_0 = is_basic(*string.offset(i as isize));
                        if _f_0 != 0 {
                            mblength_0 = 1 as libc::c_int as size_t;
                        } else if locale_utf8locale != 0
                            && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                                == 0 as libc::c_int
                        {
                            mblength_0 = (*string.offset(i as isize) as libc::c_int
                                != 0 as libc::c_int)
                                as libc::c_int as size_t;
                        } else {
                            state_bak_0 = state;
                            mblength_0 = mbrlen(
                                string.offset(i as isize),
                                slen.wrapping_sub(i as libc::c_ulong),
                                &mut state,
                            );
                        }
                        if mblength_0 == -(2 as libc::c_int) as size_t
                            || mblength_0 == -(1 as libc::c_int) as size_t
                        {
                            state = state_bak_0;
                            i += 1;
                        } else if mblength_0 == 0 as libc::c_int as libc::c_ulong {
                            i += 1;
                        } else {
                            i = (i as libc::c_ulong).wrapping_add(mblength_0) as libc::c_int
                                as libc::c_int;
                        }
                    } else {
                        i += 1;
                    }
                } else if locale_mb_cur_max > 1 as libc::c_int {
                    let mut state_bak_1: mbstate_t = mbstate_t {
                        __count: 0,
                        __value: mbstate_t_value { __wch: 0 },
                    };
                    let mut mblength_1: size_t = 0;
                    let mut _f_1: libc::c_int = 0;
                    _f_1 = is_basic(*string.offset(i as isize));
                    if _f_1 != 0 {
                        mblength_1 = 1 as libc::c_int as size_t;
                    } else if locale_utf8locale != 0
                        && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                            == 0 as libc::c_int
                    {
                        mblength_1 = (*string.offset(i as isize) as libc::c_int != 0 as libc::c_int)
                            as libc::c_int as size_t;
                    } else {
                        state_bak_1 = state;
                        mblength_1 = mbrlen(
                            string.offset(i as isize),
                            slen.wrapping_sub(i as libc::c_ulong),
                            &mut state,
                        );
                    }
                    if mblength_1 == -(2 as libc::c_int) as size_t
                        || mblength_1 == -(1 as libc::c_int) as size_t
                    {
                        state = state_bak_1;
                        i += 1;
                    } else if mblength_1 == 0 as libc::c_int as libc::c_ulong {
                        i += 1;
                    } else {
                        i = (i as libc::c_ulong).wrapping_add(mblength_1) as libc::c_int
                            as libc::c_int;
                    }
                } else {
                    i += 1;
                }
            }
        }
        no_longjmp_on_fatal_error = oldjmp;

        return i;
    }
}
#[no_mangle]
pub fn char_is_quoted(mut string: *mut libc::c_char, mut eindex: libc::c_int) -> libc::c_int {
    unsafe {
        let mut i: libc::c_int = 0;
        let mut pass_next: libc::c_int = 0;
        let mut c: libc::c_int = 0;
        let mut oldjmp: libc::c_int = 0;
        let mut slen: size_t = 0;
        let mut state: mbstate_t = mbstate_t {
            __count: 0,
            __value: mbstate_t_value { __wch: 0 },
        };
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<mbstate_t>() as usize,
        );
        slen = strlen(string) as u64;
        oldjmp = no_longjmp_on_fatal_error;
        no_longjmp_on_fatal_error = 1 as libc::c_int;
        pass_next = 0 as libc::c_int;
        i = pass_next;
        while i <= eindex {
            c = *string.offset(i as isize) as libc::c_int;
            if pass_next != 0 {
                pass_next = 0 as libc::c_int;
                if i >= eindex {
                    no_longjmp_on_fatal_error = oldjmp;
                    return 1 as libc::c_int;
                }
                if locale_mb_cur_max > 1 as libc::c_int {
                    let mut state_bak: mbstate_t = mbstate_t {
                        __count: 0,
                        __value: mbstate_t_value { __wch: 0 },
                    };
                    let mut mblength: size_t = 0;
                    let mut _f: libc::c_int = 0;
                    _f = is_basic(*string.offset(i as isize));
                    if _f != 0 {
                        mblength = 1 as libc::c_int as size_t;
                    } else if locale_utf8locale != 0
                        && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                            == 0 as libc::c_int
                    {
                        mblength = (*string.offset(i as isize) as libc::c_int != 0 as libc::c_int)
                            as libc::c_int as size_t;
                    } else {
                        state_bak = state;
                        mblength = mbrlen(
                            string.offset(i as isize),
                            slen.wrapping_sub(i as libc::c_ulong),
                            &mut state,
                        );
                    }
                    if mblength == -(2 as libc::c_int) as size_t
                        || mblength == -(1 as libc::c_int) as size_t
                    {
                        state = state_bak;
                        i += 1;
                    } else if mblength == 0 as libc::c_int as libc::c_ulong {
                        i += 1;
                    } else {
                        i = (i as libc::c_ulong).wrapping_add(mblength) as libc::c_int
                            as libc::c_int;
                    }
                } else {
                    i += 1;
                }
            } else if c == '\\' as i32 {
                pass_next = 1 as libc::c_int;
                i += 1;
            } else if c == '$' as i32
                && *string.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '\'' as i32
                && *string.offset((i + 2 as libc::c_int) as isize) as libc::c_int != 0
            {
                i += 2 as libc::c_int;
                i = skip_single_quoted(string, slen, i, 0x400 as libc::c_int);
                if i > eindex {
                    no_longjmp_on_fatal_error = oldjmp;
                    return i;
                }
            } else if c == '\'' as i32 || c == '"' as i32 {
                i = if c == '\'' as i32 {
                    i += 1;
                    skip_single_quoted(string, slen, i, 0 as libc::c_int)
                } else {
                    i += 1;
                    skip_double_quoted(string, slen, i, 0x400 as libc::c_int)
                };
                if i > eindex {
                    no_longjmp_on_fatal_error = oldjmp;
                    return 1 as libc::c_int;
                }
            } else if locale_mb_cur_max > 1 as libc::c_int {
                let mut state_bak_0: mbstate_t = mbstate_t {
                    __count: 0,
                    __value: mbstate_t_value { __wch: 0 },
                };
                let mut mblength_0: size_t = 0;
                let mut _f_0: libc::c_int = 0;
                _f_0 = is_basic(*string.offset(i as isize));
                if _f_0 != 0 {
                    mblength_0 = 1 as libc::c_int as size_t;
                } else if locale_utf8locale != 0
                    && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                        == 0 as libc::c_int
                {
                    mblength_0 = (*string.offset(i as isize) as libc::c_int != 0 as libc::c_int)
                        as libc::c_int as size_t;
                } else {
                    state_bak_0 = state;
                    mblength_0 = mbrlen(
                        string.offset(i as isize),
                        slen.wrapping_sub(i as libc::c_ulong),
                        &mut state,
                    );
                }
                if mblength_0 == -(2 as libc::c_int) as size_t
                    || mblength_0 == -(1 as libc::c_int) as size_t
                {
                    state = state_bak_0;
                    i += 1;
                } else if mblength_0 == 0 as libc::c_int as libc::c_ulong {
                    i += 1;
                } else {
                    i = (i as libc::c_ulong).wrapping_add(mblength_0) as libc::c_int as libc::c_int;
                }
            } else {
                i += 1;
            }
        }
        no_longjmp_on_fatal_error = oldjmp;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub fn unclosed_pair(
    mut string: *mut libc::c_char,
    mut eindex: libc::c_int,
    mut openstr: *mut libc::c_char,
) -> libc::c_int {
    unsafe {
        let mut i: libc::c_int = 0;
        let mut pass_next: libc::c_int = 0;
        let mut openc: libc::c_int = 0;
        let mut olen: libc::c_int = 0;
        let mut slen: size_t = 0;
        let mut state: mbstate_t = mbstate_t {
            __count: 0,
            __value: mbstate_t_value { __wch: 0 },
        };
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<mbstate_t>() as usize,
        );
        slen = strlen(string) as u64;
        olen = strlen(openstr) as libc::c_int;
        openc = 0 as libc::c_int;
        pass_next = openc;
        i = pass_next;
        while i <= eindex {
            if pass_next != 0 {
                pass_next = 0 as libc::c_int;
                if i >= eindex {
                    return 0 as libc::c_int;
                }
                if locale_mb_cur_max > 1 as libc::c_int {
                    let mut state_bak: mbstate_t = mbstate_t {
                        __count: 0,
                        __value: mbstate_t_value { __wch: 0 },
                    };
                    let mut mblength: size_t = 0;
                    let mut _f: libc::c_int = 0;
                    _f = is_basic(*string.offset(i as isize));
                    if _f != 0 {
                        mblength = 1 as libc::c_int as size_t;
                    } else if locale_utf8locale != 0
                        && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                            == 0 as libc::c_int
                    {
                        mblength = (*string.offset(i as isize) as libc::c_int != 0 as libc::c_int)
                            as libc::c_int as size_t;
                    } else {
                        state_bak = state;
                        mblength = mbrlen(
                            string.offset(i as isize),
                            slen.wrapping_sub(i as libc::c_ulong),
                            &mut state,
                        );
                    }
                    if mblength == -(2 as libc::c_int) as size_t
                        || mblength == -(1 as libc::c_int) as size_t
                    {
                        state = state_bak;
                        i += 1;
                    } else if mblength == 0 as libc::c_int as libc::c_ulong {
                        i += 1;
                    } else {
                        i = (i as libc::c_ulong).wrapping_add(mblength) as libc::c_int
                            as libc::c_int;
                    }
                } else {
                    i += 1;
                }
            } else if *string.offset(i as isize) as libc::c_int == '\\' as i32 {
                pass_next = 1 as libc::c_int;
                i += 1;
            } else if if olen == 0 as libc::c_int {
                1 as libc::c_int
            } else {
                (*string.offset(i as isize).offset(0 as libc::c_int as isize) as libc::c_int
                    == *openstr.offset(0 as libc::c_int as isize) as libc::c_int
                    && strncmp(string.offset(i as isize), openstr, olen as usize)
                        == 0 as libc::c_int) as libc::c_int
            } != 0
            {
                openc = 1 as libc::c_int - openc;
                i += olen;
            } else if *string.offset(i as isize) as libc::c_int == '\'' as i32
                || *string.offset(i as isize) as libc::c_int == '"' as i32
            {
                i = if *string.offset(i as isize) as libc::c_int == '\'' as i32 {
                    skip_single_quoted(string, slen, i, 0 as libc::c_int)
                } else {
                    skip_double_quoted(string, slen, i, 0x400 as libc::c_int)
                };
                if i > eindex {
                    return 0 as libc::c_int;
                }
            } else if locale_mb_cur_max > 1 as libc::c_int {
                let mut state_bak_0: mbstate_t = mbstate_t {
                    __count: 0,
                    __value: mbstate_t_value { __wch: 0 },
                };
                let mut mblength_0: size_t = 0;
                let mut _f_0: libc::c_int = 0;
                _f_0 = is_basic(*string.offset(i as isize));
                if _f_0 != 0 {
                    mblength_0 = 1 as libc::c_int as size_t;
                } else if locale_utf8locale != 0
                    && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                        == 0 as libc::c_int
                {
                    mblength_0 = (*string.offset(i as isize) as libc::c_int != 0 as libc::c_int)
                        as libc::c_int as size_t;
                } else {
                    state_bak_0 = state;
                    mblength_0 = mbrlen(
                        string.offset(i as isize),
                        slen.wrapping_sub(i as libc::c_ulong),
                        &mut state,
                    );
                }
                if mblength_0 == -(2 as libc::c_int) as size_t
                    || mblength_0 == -(1 as libc::c_int) as size_t
                {
                    state = state_bak_0;
                    i += 1;
                } else if mblength_0 == 0 as libc::c_int as libc::c_ulong {
                    i += 1;
                } else {
                    i = (i as libc::c_ulong).wrapping_add(mblength_0) as libc::c_int as libc::c_int;
                }
            } else {
                i += 1;
            }
        }

        return openc;
    }
}
#[no_mangle]
pub fn split_at_delims(
    mut string: *mut libc::c_char,
    mut slen: libc::c_int,
    mut delims: *mut libc::c_char,
    mut sentinel: libc::c_int,
    mut flags: libc::c_int,
    mut nwp: *mut libc::c_int,
    mut cwp: *mut libc::c_int,
) -> *mut WORD_LIST {
    unsafe {
        let mut ts: libc::c_int = 0;
        let mut te: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut nw: libc::c_int = 0;
        let mut cw: libc::c_int = 0;
        let mut ifs_split: libc::c_int = 0;
        let mut dflags: libc::c_int = 0;
        let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut d2: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ret: *mut WORD_LIST = 0 as *mut WORD_LIST;
        let mut tl: *mut WORD_LIST = 0 as *mut WORD_LIST;
        if string.is_null() || *string as libc::c_int == '\u{0}' as i32 {
            if !nwp.is_null() {
                *nwp = 0 as libc::c_int;
            }
            if !cwp.is_null() {
                *cwp = 0 as libc::c_int;
            }
            return 0 as *mut libc::c_void as *mut WORD_LIST;
        }
        d = if delims.is_null() { ifs_value } else { delims };
        ifs_split = (delims == 0 as *mut libc::c_char) as libc::c_int;
        d2 = 0 as *mut libc::c_char;
        if !delims.is_null() {
            let mut slength: size_t = 0;
            let mut mblength: size_t = 1 as libc::c_int as size_t;
            let mut state: mbstate_t = mbstate_t {
                __count: 0,
                __value: mbstate_t_value { __wch: 0 },
            };
            memset(
                &mut state as *mut mbstate_t as *mut libc::c_void,
                '\u{0}' as i32,
                ::std::mem::size_of::<mbstate_t>() as usize,
            );
            slength = strlen(delims) as u64;
            d2 = sh_xmalloc(
                slength.wrapping_add(1 as libc::c_int as libc::c_ulong),
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                2309 as libc::c_int,
            ) as *mut libc::c_char;
            ts = 0 as libc::c_int;
            i = ts;
            while *delims.offset(i as isize) != 0 {
                let mut state_bak: mbstate_t = mbstate_t {
                    __count: 0,
                    __value: mbstate_t_value { __wch: 0 },
                };
                state_bak = state;
                mblength = if __ctype_get_mb_cur_max() > 1 as libc::c_int as usize {
                    mbrlen(delims.offset(i as isize), slength, &mut state)
                } else {
                    1 as libc::c_int as libc::c_ulong
                };
                if mblength == -(1 as libc::c_int) as size_t
                    || mblength == -(2 as libc::c_int) as size_t
                {
                    state = state_bak;
                } else if mblength > 1 as libc::c_int as libc::c_ulong {
                    memcpy(
                        d2.offset(ts as isize) as *mut libc::c_void,
                        delims.offset(i as isize) as *const libc::c_void,
                        mblength as usize,
                    );
                    ts = (ts as libc::c_ulong).wrapping_add(mblength) as libc::c_int as libc::c_int;
                    i = (i as libc::c_ulong).wrapping_add(mblength) as libc::c_int as libc::c_int;
                    slength = (slength as libc::c_ulong).wrapping_sub(mblength) as size_t as size_t;
                    continue;
                }
                if (*delims.offset(i as isize) as libc::c_int == ' ' as i32
                    || *delims.offset(i as isize) as libc::c_int == '\t' as i32)
                    as libc::c_int
                    == 0 as libc::c_int
                {
                    let fresh11 = ts;
                    ts = ts + 1;
                    *d2.offset(fresh11 as isize) = *delims.offset(i as isize);
                }
                i += 1;
                slength = slength.wrapping_sub(1);
            }
            *d2.offset(ts as isize) = '\u{0}' as i32 as libc::c_char;
        }
        ret = 0 as *mut libc::c_void as *mut WORD_LIST;
        i = 0 as libc::c_int;
        while (if *string.offset(i as isize) as libc::c_int != 0 {
            (mbschr(d, *string.offset(i as isize) as libc::c_int)
                != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int
        } else {
            0 as libc::c_int
        }) != 0
            && (*string.offset(i as isize) as libc::c_int == ' ' as i32
                || *string.offset(i as isize) as libc::c_int == '\t' as i32
                || *string.offset(i as isize) as libc::c_int == '\n' as i32)
        {
            i += 1;
        }
        if *string.offset(i as isize) as libc::c_int == '\u{0}' as i32 {
            if !d2.is_null() {
                sh_xfree(
                    d2 as *mut libc::c_void,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    2345 as libc::c_int,
                );
            }
            return ret;
        }
        ts = i;
        nw = 0 as libc::c_int;
        cw = -(1 as libc::c_int);
        dflags = flags | 0x1 as libc::c_int;
        loop {
            te = skip_to_delim(string, ts, d, dflags);
            if ts == te
                && !d2.is_null()
                && (if *string.offset(ts as isize) as libc::c_int != 0 {
                    (mbschr(d2, *string.offset(ts as isize) as libc::c_int)
                        != 0 as *mut libc::c_void as *mut libc::c_char)
                        as libc::c_int
                } else {
                    0 as libc::c_int
                }) != 0
            {
                te = ts + 1 as libc::c_int;
                if ifs_split != 0 {
                    while (if *string.offset(te as isize) as libc::c_int != 0 {
                        (mbschr(d, *string.offset(te as isize) as libc::c_int)
                            != 0 as *mut libc::c_void as *mut libc::c_char)
                            as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) != 0
                        && (*string.offset(te as isize) as libc::c_int == ' ' as i32
                            || *string.offset(te as isize) as libc::c_int == '\t' as i32
                            || *string.offset(te as isize) as libc::c_int == '\n' as i32)
                        && (flags & 0x4 as libc::c_int == 0 as libc::c_int
                            || *string.offset(te as isize) as libc::c_int != '\'' as i32
                                && *string.offset(te as isize) as libc::c_int != '"' as i32)
                    {
                        te += 1;
                    }
                } else {
                    while (if *string.offset(te as isize) as libc::c_int != 0 {
                        (mbschr(d2, *string.offset(te as isize) as libc::c_int)
                            != 0 as *mut libc::c_void as *mut libc::c_char)
                            as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) != 0
                        && (flags & 0x4 as libc::c_int == 0 as libc::c_int
                            || *string.offset(te as isize) as libc::c_int != '\'' as i32
                                && *string.offset(te as isize) as libc::c_int != '"' as i32)
                    {
                        te += 1;
                    }
                }
            }
            token = substring(string, ts, te);
            ret = make_word_list(make_word(token), ret);
            sh_xfree(
                token as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                2376 as libc::c_int,
            );
            nw += 1;
            if sentinel >= ts && sentinel <= te {
                cw = nw;
            }
            if !cwp.is_null() && cw == -(1 as libc::c_int) && sentinel == ts - 1 as libc::c_int {
                cw = nw;
            }
            if !cwp.is_null() && cw == -(1 as libc::c_int) && sentinel < ts {
                tl = make_word_list(
                    make_word(b"\0" as *const u8 as *const libc::c_char),
                    (*ret).next,
                );
                let ref mut fresh12 = (*ret).next;
                *fresh12 = tl;
                cw = nw;
                nw += 1;
            }
            if *string.offset(te as isize) as libc::c_int == 0 as libc::c_int {
                break;
            }
            i = te;
            while (if *string.offset(i as isize) as libc::c_int != 0 {
                (mbschr(d, *string.offset(i as isize) as libc::c_int)
                    != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int
            } else {
                0 as libc::c_int
            }) != 0
                && (ifs_split != 0
                    || (*string.offset(i as isize) as libc::c_int == ' ' as i32
                        || *string.offset(i as isize) as libc::c_int == '\t' as i32
                        || *string.offset(i as isize) as libc::c_int == '\n' as i32))
                && (flags & 0x4 as libc::c_int == 0 as libc::c_int
                    || *string.offset(te as isize) as libc::c_int != '\'' as i32
                        && *string.offset(te as isize) as libc::c_int != '"' as i32)
            {
                i += 1;
            }
            if !(*string.offset(i as isize) != 0) {
                break;
            }
            ts = i;
        }
        if !cwp.is_null() && cw == -(1 as libc::c_int) && (sentinel >= slen || sentinel >= te) {
            if *string.offset((sentinel - 1 as libc::c_int) as isize) as libc::c_int == ' ' as i32
                || *string.offset((sentinel - 1 as libc::c_int) as isize) as libc::c_int
                    == '\t' as i32
            {
                token = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                ret = make_word_list(make_word(token), ret);
                nw += 1;
            }
            cw = nw;
        }
        if !nwp.is_null() {
            *nwp = nw;
        }
        if !cwp.is_null() {
            *cwp = cw;
        }
        if !d2.is_null() {
            sh_xfree(
                d2 as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                2433 as libc::c_int,
            );
        }
        return if !ret.is_null() && !((*ret).next).is_null() {
            list_reverse(ret as *mut GENERIC_LIST) as *mut WORD_LIST
        } else {
            ret
        };
    }
}
#[no_mangle]
pub fn string_list_internal(
    mut list: *mut WORD_LIST,
    mut sep: *mut libc::c_char,
) -> *mut libc::c_char {
    unsafe {
        let mut t: *mut WORD_LIST = 0 as *mut WORD_LIST;
        let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut word_len: size_t = 0;
        let mut sep_len: size_t = 0;
        let mut result_size: size_t = 0;
        if list.is_null() {
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
        if ((*list).next).is_null() {
            return strcpy(
                sh_xmalloc(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(strlen((*(*list).word).word) as u64),
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    2479 as libc::c_int,
                ) as *mut libc::c_char,
                (*(*list).word).word,
            );
        }
        sep_len = if !sep.is_null() && *sep.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            if *sep.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                if *sep.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                    strlen(sep) as u64
                } else {
                    2 as libc::c_int as libc::c_ulong
                }
            } else {
                1 as libc::c_int as libc::c_ulong
            }
        } else {
            0 as libc::c_int as libc::c_ulong
        };
        result_size = 0 as libc::c_int as size_t;
        t = list;
        while !t.is_null() {
            if t != list {
                result_size =
                    (result_size as libc::c_ulong).wrapping_add(sep_len) as size_t as size_t;
            }
            result_size = (result_size as libc::c_ulong)
                .wrapping_add(strlen((*(*t).word).word) as u64) as size_t
                as size_t;
            t = (*t).next;
        }
        result = sh_xmalloc(
            result_size.wrapping_add(1 as libc::c_int as libc::c_ulong),
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            2492 as libc::c_int,
        ) as *mut libc::c_char;
        r = result;
        t = list;
        while !t.is_null() {
            if t != list && sep_len != 0 {
                if sep_len > 1 as libc::c_int as libc::c_ulong {
                    libc::memcpy(
                        r as *mut libc::c_void,
                        sep as *const libc::c_void,
                        sep_len as libc::size_t,
                    );
                    r = r.offset(sep_len as isize);
                } else {
                    let fresh13 = r;
                    r = r.offset(1);
                    *fresh13 = *sep.offset(0 as libc::c_int as isize);
                }
            }
            word_len = strlen((*(*t).word).word) as u64;
            libc::memcpy(
                r as *mut libc::c_void,
                (*(*t).word).word as *const libc::c_void,
                word_len as libc::size_t,
            );
            r = r.offset(word_len as isize);
            t = (*t).next;
        }
        *r = '\u{0}' as i32 as libc::c_char;

        return result;
    }
}
#[no_mangle]
pub fn string_list(mut list: *mut WORD_LIST) -> *mut libc::c_char {
    return string_list_internal(
        list,
        b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
#[no_mangle]
pub fn ifs_firstchar(mut lenp: *mut libc::c_int) -> *mut libc::c_char {
    unsafe {
        let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut len: libc::c_int = 0;
        ret = sh_xmalloc(
            (16 as libc::c_int + 1 as libc::c_int) as size_t,
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            2536 as libc::c_int,
        ) as *mut libc::c_char;
        if ifs_firstc_len == 1 as libc::c_int as libc::c_ulong {
            *ret.offset(0 as libc::c_int as isize) =
                ifs_firstc[0 as libc::c_int as usize] as libc::c_char;
            *ret.offset(1 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
            len = if *ret.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            };
        } else {
            memcpy(
                ret as *mut libc::c_void,
                ifs_firstc.as_mut_ptr() as *const libc::c_void,
                ifs_firstc_len as usize,
            );
            len = ifs_firstc_len as libc::c_int;
            *ret.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
        }
        if !lenp.is_null() {
            *lenp = len;
        }

        return ret;
    }
}
#[no_mangle]
pub fn string_list_dollar_star(
    mut list: *mut WORD_LIST,
    mut quoted: libc::c_int,
    mut flags: libc::c_int,
) -> *mut libc::c_char {
    unsafe {
        let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
        let vla = (__ctype_get_mb_cur_max()).wrapping_add(1 as libc::c_int as usize) as usize;
        let mut sep: Vec<libc::c_char> = ::std::vec::from_elem(0, vla);
        if ifs_firstc_len == 1 as libc::c_int as libc::c_ulong {
            *sep.as_mut_ptr().offset(0 as libc::c_int as isize) =
                ifs_firstc[0 as libc::c_int as usize] as libc::c_char;
            *sep.as_mut_ptr().offset(1 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
        } else {
            memcpy(
                sep.as_mut_ptr() as *mut libc::c_void,
                ifs_firstc.as_mut_ptr() as *const libc::c_void,
                ifs_firstc_len as usize,
            );
            *sep.as_mut_ptr().offset(ifs_firstc_len as isize) = '\u{0}' as i32 as libc::c_char;
        }
        ret = string_list_internal(list, sep.as_mut_ptr());

        return ret;
    }
}
#[no_mangle]
pub fn string_list_dollar_at(
    mut list: *mut WORD_LIST,
    mut quoted: libc::c_int,
    mut flags: libc::c_int,
) -> *mut libc::c_char {
    unsafe {
        let mut ifs: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
        let vla = (__ctype_get_mb_cur_max()).wrapping_add(1 as libc::c_int as usize) as usize;
        let mut sep: Vec<libc::c_char> = ::std::vec::from_elem(0, vla);
        let mut tlist: *mut WORD_LIST = 0 as *mut WORD_LIST;
        ifs = if !ifs_var.is_null() {
            (*ifs_var).value
        } else {
            0 as *mut libc::c_char
        };
        if flags & 0x8 as libc::c_int != 0 {
            *sep.as_mut_ptr().offset(0 as libc::c_int as isize) = ' ' as i32 as libc::c_char;
            *sep.as_mut_ptr().offset(1 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
        } else if !ifs.is_null() && *ifs as libc::c_int != 0 {
            if ifs_firstc_len == 1 as libc::c_int as libc::c_ulong {
                *sep.as_mut_ptr().offset(0 as libc::c_int as isize) =
                    ifs_firstc[0 as libc::c_int as usize] as libc::c_char;
                *sep.as_mut_ptr().offset(1 as libc::c_int as isize) =
                    '\u{0}' as i32 as libc::c_char;
            } else {
                memcpy(
                    sep.as_mut_ptr() as *mut libc::c_void,
                    ifs_firstc.as_mut_ptr() as *const libc::c_void,
                    ifs_firstc_len as usize,
                );
                *sep.as_mut_ptr().offset(ifs_firstc_len as isize) = '\u{0}' as i32 as libc::c_char;
            }
        } else {
            *sep.as_mut_ptr().offset(0 as libc::c_int as isize) = ' ' as i32 as libc::c_char;
            *sep.as_mut_ptr().offset(1 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
        }
        tlist = if quoted & (0x2 as libc::c_int | 0x1 as libc::c_int | 0x8 as libc::c_int) != 0 {
            quote_list(list)
        } else {
            list_quote_escapes(list)
        };
        ret = string_list_internal(tlist, sep.as_mut_ptr());

        return ret;
    }
}
#[no_mangle]
pub fn string_list_pos_params(
    mut pchar: libc::c_int,
    mut list: *mut WORD_LIST,
    mut quoted: libc::c_int,
    mut pflags: libc::c_int,
) -> *mut libc::c_char {
    unsafe {
        let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut tlist: *mut WORD_LIST = 0 as *mut WORD_LIST;
        if pchar == '*' as i32 && quoted & 0x1 as libc::c_int != 0 {
            tlist = quote_list(list);
            word_list_remove_quoted_nulls(tlist);
            ret = string_list_dollar_star(tlist, 0 as libc::c_int, 0 as libc::c_int);
        } else if pchar == '*' as i32 && quoted & 0x2 as libc::c_int != 0 {
            tlist = quote_list(list);
            word_list_remove_quoted_nulls(tlist);
            ret = string_list(tlist);
        } else if pchar == '*' as i32 && quoted == 0 as libc::c_int && ifs_is_null != 0 {
            ret = if expand_no_split_dollar_star != 0 {
                string_list_dollar_star(list, quoted, 0 as libc::c_int)
            } else {
                string_list_dollar_at(list, quoted, 0 as libc::c_int)
            };
        } else if pchar == '*' as i32
            && quoted == 0 as libc::c_int
            && pflags & 0x8 as libc::c_int != 0
        {
            ret = if expand_no_split_dollar_star != 0 {
                string_list_dollar_star(list, quoted, 0 as libc::c_int)
            } else {
                string_list_dollar_at(list, quoted, 0 as libc::c_int)
            };
        } else if pchar == '*' as i32 {
            ret = string_list_dollar_star(list, quoted, 0 as libc::c_int);
        } else if pchar == '@' as i32 && quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0 {
            ret = string_list_dollar_at(list, quoted, 0 as libc::c_int);
        } else if pchar == '@' as i32 && quoted == 0 as libc::c_int && ifs_is_null != 0 {
            ret = string_list_dollar_at(list, quoted, 0 as libc::c_int);
        } else if pchar == '@' as i32
            && quoted == 0 as libc::c_int
            && pflags & 0x8 as libc::c_int != 0
        {
            ret = string_list_dollar_at(list, quoted, pflags);
        } else if pchar == '@' as i32 {
            ret = string_list_dollar_star(list, quoted, 0 as libc::c_int);
        } else {
            ret = string_list(if quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0 {
                quote_list(list)
            } else {
                list
            });
        }

        return ret;
    }
}
#[no_mangle]
pub fn list_string(
    mut string: *mut libc::c_char,
    mut separators: *mut libc::c_char,
    mut quoted: libc::c_int,
) -> *mut WORD_LIST {
    unsafe {
        let mut result: *mut WORD_LIST = 0 as *mut WORD_LIST;
        let mut t: *mut WORD_DESC = 0 as *mut WORD_DESC;
        let mut current_word: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut sindex: libc::c_int = 0;
        let mut sh_style_split: libc::c_int = 0;
        let mut whitesep: libc::c_int = 0;
        let mut xflags: libc::c_int = 0;
        let mut free_word: libc::c_int = 0;
        let mut slen: size_t = 0;
        if string.is_null() || *string == 0 {
            return 0 as *mut libc::c_void as *mut WORD_LIST;
        }
        sh_style_split = (!separators.is_null()
            && *separators.offset(0 as libc::c_int as isize) as libc::c_int == ' ' as i32
            && *separators.offset(1 as libc::c_int as isize) as libc::c_int == '\t' as i32
            && *separators.offset(2 as libc::c_int as isize) as libc::c_int == '\n' as i32
            && *separators.offset(3 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32)
            as libc::c_int;
        xflags = 0 as libc::c_int;
        s = ifs_value;
        while !s.is_null() && *s as libc::c_int != 0 {
            if *s as libc::c_int == '\u{1}' as i32 {
                xflags |= 0x10 as libc::c_int;
            } else if *s as libc::c_int == '\u{7f}' as i32 {
                xflags |= 0x20 as libc::c_int;
            }
            s = s.offset(1);
        }
        slen = 0 as libc::c_int as size_t;
        if quoted == 0 && !separators.is_null() && *separators as libc::c_int != 0 {
            s = string;
            while *s as libc::c_int != 0
                && (if *separators.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
                    if *separators.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                        (ifs_cmap[*s as libc::c_uchar as usize] as libc::c_int != 0 as libc::c_int)
                            as libc::c_int
                    } else {
                        (*s as libc::c_int
                            == *separators.offset(0 as libc::c_int as isize) as libc::c_int)
                            as libc::c_int
                    }
                } else {
                    0 as libc::c_int
                }) != 0
                && (1 as libc::c_int != 0
                    && *(*__ctype_b_loc()).offset(*s as libc::c_uchar as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                        != 0)
            {
                s = s.offset(1);
            }
            if *s == 0 {
                return 0 as *mut libc::c_void as *mut WORD_LIST;
            }
            string = s;
        }
        slen = if !string.is_null() && *string.offset(0 as libc::c_int as isize) as libc::c_int != 0
        {
            if *string.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                if *string.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                    strlen(string) as u64
                } else {
                    2 as libc::c_int as libc::c_ulong
                }
            } else {
                1 as libc::c_int as libc::c_ulong
            }
        } else {
            0 as libc::c_int as libc::c_ulong
        };
        result = 0 as *mut libc::c_void as *mut WORD_LIST;
        sindex = 0 as libc::c_int;
        while *string.offset(sindex as isize) != 0 {
            current_word = string_extract_verbatim(string, slen, &mut sindex, separators, xflags);
            if current_word.is_null() {
                break;
            }
            free_word = 1 as libc::c_int;
            if *current_word.offset(0 as libc::c_int as isize) as libc::c_int == '\u{7f}' as i32
                && *current_word.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
            {
                t = alloc_word_desc();
                let ref mut fresh14 = (*t).word;
                *fresh14 = make_quoted_char('\u{0}' as i32);
                (*t).flags |= (1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 18 as libc::c_int;
                result = make_word_list(t, result);
            } else if *current_word.offset(0 as libc::c_int as isize) as libc::c_int
                != '\u{0}' as i32
            {
                remove_quoted_nulls(current_word);
                t = alloc_word_desc();
                let ref mut fresh15 = (*t).word;
                *fresh15 = current_word;
                result = make_word_list(t, result);
                free_word = 0 as libc::c_int;
                (*(*result).word).flags &= !((1 as libc::c_int) << 18 as libc::c_int);
                if quoted & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0 {
                    (*(*result).word).flags |= (1 as libc::c_int) << 1 as libc::c_int;
                }
                if current_word.is_null()
                    || *current_word.offset(0 as libc::c_int as isize) as libc::c_int
                        == '\u{0}' as i32
                {
                    (*(*result).word).flags |= (1 as libc::c_int) << 21 as libc::c_int;
                }
            } else if sh_style_split == 0
                && !(1 as libc::c_int != 0
                    && *(*__ctype_b_loc()).offset(
                        *string.offset(sindex as isize) as libc::c_uchar as libc::c_int as isize
                    ) as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                        != 0)
            {
                t = alloc_word_desc();
                let ref mut fresh16 = (*t).word;
                *fresh16 = make_quoted_char('\u{0}' as i32);
                (*t).flags |= (1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 18 as libc::c_int;
                result = make_word_list(t, result);
            }
            if free_word != 0 {
                sh_xfree(
                    current_word as *mut libc::c_void,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    2901 as libc::c_int,
                );
            }
            whitesep = (*string.offset(sindex as isize) as libc::c_int != 0
                && (if sh_style_split != 0 || separators.is_null() {
                    (*string.offset(sindex as isize) as libc::c_int == ' ' as i32
                        || *string.offset(sindex as isize) as libc::c_int == '\t' as i32
                        || *string.offset(sindex as isize) as libc::c_int == '\n' as i32)
                        as libc::c_int
                } else {
                    (1 as libc::c_int != 0
                        && *(*__ctype_b_loc()).offset(*string.offset(sindex as isize)
                            as libc::c_uchar
                            as libc::c_int
                            as isize) as libc::c_int
                            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                            != 0) as libc::c_int
                }) != 0) as libc::c_int;
            if *string.offset(sindex as isize) != 0 {
                let mut state: mbstate_t = mbstate_t {
                    __count: 0,
                    __value: mbstate_t_value { __wch: 0 },
                };
                memset(
                    &mut state as *mut mbstate_t as *mut libc::c_void,
                    '\u{0}' as i32,
                    ::std::mem::size_of::<mbstate_t>() as usize,
                );
                if locale_mb_cur_max > 1 as libc::c_int {
                    let mut state_bak: mbstate_t = mbstate_t {
                        __count: 0,
                        __value: mbstate_t_value { __wch: 0 },
                    };
                    let mut mblength: size_t = 0;
                    let mut _f: libc::c_int = 0;
                    _f = is_basic(*string.offset(sindex as isize));
                    if _f != 0 {
                        mblength = 1 as libc::c_int as size_t;
                    } else if locale_utf8locale != 0
                        && *string.offset(sindex as isize) as libc::c_int & 0x80 as libc::c_int
                            == 0 as libc::c_int
                    {
                        mblength = (*string.offset(sindex as isize) as libc::c_int
                            != 0 as libc::c_int) as libc::c_int
                            as size_t;
                    } else {
                        state_bak = state;
                        mblength = mbrlen(
                            string.offset(sindex as isize),
                            slen.wrapping_sub(sindex as libc::c_ulong),
                            &mut state,
                        );
                    }
                    if mblength == -(2 as libc::c_int) as size_t
                        || mblength == -(1 as libc::c_int) as size_t
                    {
                        state = state_bak;
                        sindex += 1;
                    } else if mblength == 0 as libc::c_int as libc::c_ulong {
                        sindex += 1;
                    } else {
                        sindex = (sindex as libc::c_ulong).wrapping_add(mblength) as libc::c_int
                            as libc::c_int;
                    }
                } else {
                    sindex += 1;
                }
            }
            while *string.offset(sindex as isize) as libc::c_int != 0
                && (if sh_style_split != 0 || separators.is_null() {
                    (*string.offset(sindex as isize) as libc::c_int == ' ' as i32
                        || *string.offset(sindex as isize) as libc::c_int == '\t' as i32
                        || *string.offset(sindex as isize) as libc::c_int == '\n' as i32)
                        as libc::c_int
                } else {
                    (1 as libc::c_int != 0
                        && *(*__ctype_b_loc()).offset(*string.offset(sindex as isize)
                            as libc::c_uchar
                            as libc::c_int
                            as isize) as libc::c_int
                            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                            != 0) as libc::c_int
                }) != 0
                && (if *separators.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
                    if *separators.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                        (ifs_cmap[*string.offset(sindex as isize) as libc::c_uchar as usize]
                            as libc::c_int
                            != 0 as libc::c_int) as libc::c_int
                    } else {
                        (*string.offset(sindex as isize) as libc::c_int
                            == *separators.offset(0 as libc::c_int as isize) as libc::c_int)
                            as libc::c_int
                    }
                } else {
                    0 as libc::c_int
                }) != 0
            {
                sindex += 1;
            }
            if *string.offset(sindex as isize) as libc::c_int != 0
                && whitesep != 0
                && (if *separators.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
                    if *separators.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                        (ifs_cmap[*string.offset(sindex as isize) as libc::c_uchar as usize]
                            as libc::c_int
                            != 0 as libc::c_int) as libc::c_int
                    } else {
                        (*string.offset(sindex as isize) as libc::c_int
                            == *separators.offset(0 as libc::c_int as isize) as libc::c_int)
                            as libc::c_int
                    }
                } else {
                    0 as libc::c_int
                }) != 0
                && (if sh_style_split != 0 || separators.is_null() {
                    (*string.offset(sindex as isize) as libc::c_int == ' ' as i32
                        || *string.offset(sindex as isize) as libc::c_int == '\t' as i32
                        || *string.offset(sindex as isize) as libc::c_int == '\n' as i32)
                        as libc::c_int
                } else {
                    (1 as libc::c_int != 0
                        && *(*__ctype_b_loc()).offset(*string.offset(sindex as isize)
                            as libc::c_uchar
                            as libc::c_int
                            as isize) as libc::c_int
                            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                            != 0) as libc::c_int
                }) == 0
            {
                sindex += 1;
                while *string.offset(sindex as isize) as libc::c_int != 0
                    && (if sh_style_split != 0 || separators.is_null() {
                        (*string.offset(sindex as isize) as libc::c_int == ' ' as i32
                            || *string.offset(sindex as isize) as libc::c_int == '\t' as i32
                            || *string.offset(sindex as isize) as libc::c_int == '\n' as i32)
                            as libc::c_int
                    } else {
                        (1 as libc::c_int != 0
                            && *(*__ctype_b_loc())
                                .offset(*string.offset(sindex as isize) as libc::c_uchar
                                    as libc::c_int as isize)
                                as libc::c_int
                                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                != 0) as libc::c_int
                    }) != 0
                    && ifs_cmap[*string.offset(sindex as isize) as libc::c_uchar as usize]
                        as libc::c_int
                        != 0 as libc::c_int
                {
                    sindex += 1;
                }
            }
        }
        return if !result.is_null() && !((*result).next).is_null() {
            list_reverse(result as *mut GENERIC_LIST) as *mut WORD_LIST
        } else {
            result
        };
    }
}
#[no_mangle]
pub fn get_word_from_string(
    mut stringp: *mut *mut libc::c_char,
    mut separators: *mut libc::c_char,
    mut endptr: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    unsafe {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut current_word: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut sindex: libc::c_int = 0;
        let mut sh_style_split: libc::c_int = 0;
        let mut whitesep: libc::c_int = 0;
        let mut xflags: libc::c_int = 0;
        let mut local_cmap: [libc::c_uchar; 256] = [0; 256];
        let mut slen: size_t = 0;
        if stringp.is_null() || (*stringp).is_null() || **stringp == 0 {
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
        sh_style_split = (!separators.is_null()
            && *separators.offset(0 as libc::c_int as isize) as libc::c_int == ' ' as i32
            && *separators.offset(1 as libc::c_int as isize) as libc::c_int == '\t' as i32
            && *separators.offset(2 as libc::c_int as isize) as libc::c_int == '\n' as i32
            && *separators.offset(3 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32)
            as libc::c_int;
        memset(
            local_cmap.as_mut_ptr() as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<[libc::c_uchar; 256]>() as usize,
        );
        xflags = 0 as libc::c_int;
        s = separators;
        while !s.is_null() && *s as libc::c_int != 0 {
            if *s as libc::c_int == '\u{1}' as i32 {
                xflags |= 0x10 as libc::c_int;
            }
            if *s as libc::c_int == '\u{7f}' as i32 {
                xflags |= 0x20 as libc::c_int;
            }
            local_cmap[*s as libc::c_uchar as usize] = 1 as libc::c_int as libc::c_uchar;
            s = s.offset(1);
        }
        s = *stringp;
        slen = 0 as libc::c_int as size_t;
        if sh_style_split != 0 || separators.is_null() {
            while *s as libc::c_int != 0
                && (*s as libc::c_int == ' ' as i32
                    || *s as libc::c_int == '\t' as i32
                    || *s as libc::c_int == '\n' as i32)
                && local_cmap[*s as libc::c_uchar as usize] as libc::c_int != 0 as libc::c_int
            {
                s = s.offset(1);
            }
        } else {
            while *s as libc::c_int != 0
                && (1 as libc::c_int != 0
                    && *(*__ctype_b_loc()).offset(*s as libc::c_uchar as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                        != 0)
                && local_cmap[*s as libc::c_uchar as usize] as libc::c_int != 0 as libc::c_int
            {
                s = s.offset(1);
            }
        }
        if *s == 0 {
            *stringp = s;
            if !endptr.is_null() {
                *endptr = s;
            }
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
        sindex = 0 as libc::c_int;
        slen = if !s.is_null() && *s.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            if *s.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                if *s.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                    strlen(s) as u64
                } else {
                    2 as libc::c_int as libc::c_ulong
                }
            } else {
                1 as libc::c_int as libc::c_ulong
            }
        } else {
            0 as libc::c_int as libc::c_ulong
        };
        current_word = string_extract_verbatim(s, slen, &mut sindex, separators, xflags);
        if !endptr.is_null() {
            *endptr = s.offset(sindex as isize);
        }
        whitesep = (*s.offset(sindex as isize) as libc::c_int != 0
            && (if sh_style_split != 0 || separators.is_null() {
                (*s.offset(sindex as isize) as libc::c_int == ' ' as i32
                    || *s.offset(sindex as isize) as libc::c_int == '\t' as i32
                    || *s.offset(sindex as isize) as libc::c_int == '\n' as i32)
                    as libc::c_int
            } else {
                (1 as libc::c_int != 0
                    && *(*__ctype_b_loc())
                        .offset(*s.offset(sindex as isize) as libc::c_uchar as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                        != 0) as libc::c_int
            }) != 0) as libc::c_int;
        if *s.offset(sindex as isize) != 0 {
            let mut state: mbstate_t = mbstate_t {
                __count: 0,
                __value: mbstate_t_value { __wch: 0 },
            };
            memset(
                &mut state as *mut mbstate_t as *mut libc::c_void,
                '\u{0}' as i32,
                ::std::mem::size_of::<mbstate_t>() as usize,
            );
            if locale_mb_cur_max > 1 as libc::c_int {
                let mut state_bak: mbstate_t = mbstate_t {
                    __count: 0,
                    __value: mbstate_t_value { __wch: 0 },
                };
                let mut mblength: size_t = 0;
                let mut _f: libc::c_int = 0;
                _f = is_basic(*s.offset(sindex as isize));
                if _f != 0 {
                    mblength = 1 as libc::c_int as size_t;
                } else if locale_utf8locale != 0
                    && *s.offset(sindex as isize) as libc::c_int & 0x80 as libc::c_int
                        == 0 as libc::c_int
                {
                    mblength = (*s.offset(sindex as isize) as libc::c_int != 0 as libc::c_int)
                        as libc::c_int as size_t;
                } else {
                    state_bak = state;
                    mblength = mbrlen(
                        s.offset(sindex as isize),
                        slen.wrapping_sub(sindex as libc::c_ulong),
                        &mut state,
                    );
                }
                if mblength == -(2 as libc::c_int) as size_t
                    || mblength == -(1 as libc::c_int) as size_t
                {
                    state = state_bak;
                    sindex += 1;
                } else if mblength == 0 as libc::c_int as libc::c_ulong {
                    sindex += 1;
                } else {
                    sindex = (sindex as libc::c_ulong).wrapping_add(mblength) as libc::c_int
                        as libc::c_int;
                }
            } else {
                sindex += 1;
            }
        }
        while *s.offset(sindex as isize) as libc::c_int != 0
            && (*s.offset(sindex as isize) as libc::c_int == ' ' as i32
                || *s.offset(sindex as isize) as libc::c_int == '\t' as i32
                || *s.offset(sindex as isize) as libc::c_int == '\n' as i32)
            && local_cmap[*s.offset(sindex as isize) as libc::c_uchar as usize] as libc::c_int
                != 0 as libc::c_int
        {
            sindex += 1;
        }
        if *s.offset(sindex as isize) as libc::c_int != 0
            && whitesep != 0
            && local_cmap[*s.offset(sindex as isize) as libc::c_uchar as usize] as libc::c_int
                != 0 as libc::c_int
            && (if sh_style_split != 0 || separators.is_null() {
                (*s.offset(sindex as isize) as libc::c_int == ' ' as i32
                    || *s.offset(sindex as isize) as libc::c_int == '\t' as i32
                    || *s.offset(sindex as isize) as libc::c_int == '\n' as i32)
                    as libc::c_int
            } else {
                (1 as libc::c_int != 0
                    && *(*__ctype_b_loc())
                        .offset(*s.offset(sindex as isize) as libc::c_uchar as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                        != 0) as libc::c_int
            }) == 0
        {
            sindex += 1;
            while *s.offset(sindex as isize) as libc::c_int != 0
                && (if sh_style_split != 0 || separators.is_null() {
                    (*s.offset(sindex as isize) as libc::c_int == ' ' as i32
                        || *s.offset(sindex as isize) as libc::c_int == '\t' as i32
                        || *s.offset(sindex as isize) as libc::c_int == '\n' as i32)
                        as libc::c_int
                } else {
                    (1 as libc::c_int != 0
                        && *(*__ctype_b_loc()).offset(*s.offset(sindex as isize) as libc::c_uchar
                            as libc::c_int
                            as isize) as libc::c_int
                            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                            != 0) as libc::c_int
                }) != 0
                && local_cmap[*s.offset(sindex as isize) as libc::c_uchar as usize] as libc::c_int
                    != 0 as libc::c_int
            {
                sindex += 1;
            }
        }
        *stringp = s.offset(sindex as isize);

        return current_word;
    }
}
#[no_mangle]
pub fn strip_trailing_ifs_whitespace(
    mut string: *mut libc::c_char,
    mut separators: *mut libc::c_char,
    mut saw_escape: libc::c_int,
) -> *mut libc::c_char {
    unsafe {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        s = string
            .offset(
                (if !string.is_null()
                    && *string.offset(0 as libc::c_int as isize) as libc::c_int != 0
                {
                    if *string.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                        if *string.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                            strlen(string) as usize
                        } else {
                            2 as libc::c_int as usize
                        }
                    } else {
                        1 as libc::c_int as usize
                    }
                } else {
                    0 as libc::c_int as usize
                }) as isize,
            )
            .offset(-(1 as libc::c_int as isize));
        while s > string
            && ((*s as libc::c_int == ' ' as i32
                || *s as libc::c_int == '\t' as i32
                || *s as libc::c_int == '\n' as i32)
                && ifs_cmap[*s as libc::c_uchar as usize] as libc::c_int != 0 as libc::c_int
                || saw_escape != 0
                    && *s as libc::c_int == '\u{1}' as i32
                    && (*s.offset(1 as libc::c_int as isize) as libc::c_int == ' ' as i32
                        || *s.offset(1 as libc::c_int as isize) as libc::c_int == '\t' as i32
                        || *s.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32))
        {
            s = s.offset(-1);
        }
        s = s.offset(1);
        *s = '\u{0}' as i32 as libc::c_char;
    }
    return string;
}
fn do_compound_assignment(
    mut name: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut flags: libc::c_int,
) -> *mut SHELL_VAR {
    unsafe {
        let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
        let mut mklocal: libc::c_int = 0;
        let mut mkassoc: libc::c_int = 0;
        let mut mkglobal: libc::c_int = 0;
        let mut chklocal: libc::c_int = 0;
        let mut list: *mut WORD_LIST = 0 as *mut WORD_LIST;
        let mut newname: *mut libc::c_char = 0 as *mut libc::c_char;
        mklocal = flags & 0x2 as libc::c_int;
        mkassoc = flags & 0x4 as libc::c_int;
        mkglobal = flags & 0x8 as libc::c_int;
        chklocal = flags & 0x40 as libc::c_int;
        if mklocal != 0 && variable_context != 0 {
            v = find_variable(name);
            newname = if v.is_null() {
                nameref_transform_name(name, flags)
            } else {
                (*v).name
            };
            if !v.is_null()
                && ((*v).attributes & 0x2 as libc::c_int != 0
                    && flags & 0x20 as libc::c_int == 0 as libc::c_int
                    || (*v).attributes & 0x4000 as libc::c_int != 0)
            {
                if (*v).attributes & 0x2 as libc::c_int != 0 {
                    err_readonly(name);
                }
                return v;
            }
            list = expand_compound_array_assignment(v, value, flags);
            if mkassoc != 0 {
                v = make_local_assoc_variable(newname, 0 as libc::c_int);
            } else if v.is_null()
                || (*v).attributes & 0x4 as libc::c_int == 0 as libc::c_int
                    && (*v).attributes & 0x40 as libc::c_int == 0 as libc::c_int
                || (*v).context != variable_context
            {
                v = make_local_array_variable(newname, 0 as libc::c_int);
            }
            if !v.is_null() {
                assign_compound_array_list(v, list, flags);
            }
            if !list.is_null() {
                dispose_words(list);
            }
        } else if mkglobal != 0 && variable_context != 0 {
            v = if chklocal != 0 {
                find_variable(name)
            } else {
                0 as *mut SHELL_VAR
            };
            if !v.is_null()
                && ((*v).attributes & 0x20 as libc::c_int == 0 as libc::c_int
                    || (*v).context != variable_context)
            {
                v = 0 as *mut SHELL_VAR;
            }
            if v.is_null() {
                v = find_global_variable(name);
            }
            if !v.is_null()
                && ((*v).attributes & 0x2 as libc::c_int != 0
                    && flags & 0x20 as libc::c_int == 0 as libc::c_int
                    || (*v).attributes & 0x4000 as libc::c_int != 0)
            {
                if (*v).attributes & 0x2 as libc::c_int != 0 {
                    err_readonly(name);
                }
                return v;
            }
            newname = if v.is_null() {
                nameref_transform_name(name, flags)
            } else {
                name
            };
            list = expand_compound_array_assignment(v, value, flags);
            if v.is_null() && mkassoc != 0 {
                v = make_new_assoc_variable(newname);
            } else if !v.is_null()
                && mkassoc != 0
                && (*v).attributes & 0x40 as libc::c_int == 0 as libc::c_int
            {
                v = convert_var_to_assoc(v);
            } else if v.is_null() {
                v = make_new_array_variable(newname);
            } else if !v.is_null()
                && mkassoc == 0 as libc::c_int
                && (*v).attributes & 0x4 as libc::c_int == 0 as libc::c_int
            {
                v = convert_var_to_array(v);
            }
            if !v.is_null() {
                assign_compound_array_list(v, list, flags);
            }
            if !list.is_null() {
                dispose_words(list);
            }
        } else {
            v = assign_array_from_string(name, value, flags);
            if !v.is_null()
                && ((*v).attributes & 0x2 as libc::c_int != 0
                    && flags & 0x20 as libc::c_int == 0 as libc::c_int
                    || (*v).attributes & 0x4000 as libc::c_int != 0)
            {
                if (*v).attributes & 0x2 as libc::c_int != 0 {
                    err_readonly(name);
                }
                return v;
            }
        }

        return v;
    }
}
fn do_assignment_internal(mut word: *const WORD_DESC, mut expand: libc::c_int) -> libc::c_int {
    unsafe {
        let mut offset: libc::c_int = 0;
        let mut appendop: libc::c_int = 0;
        let mut assign_list: libc::c_int = 0;
        let mut aflags: libc::c_int = 0;
        let mut retval: libc::c_int = 0;
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut entry: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
        let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ni: libc::c_int = 0;
        let mut string: *const libc::c_char = 0 as *const libc::c_char;
        if word.is_null() || ((*word).word).is_null() {
            return 0 as libc::c_int;
        }
        aflags = 0 as libc::c_int;
        assign_list = aflags;
        appendop = assign_list;
        string = (*word).word;
        offset = assignment(string, 0 as libc::c_int);
        name = strcpy(
            sh_xmalloc(
                (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(string) as u64),
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                3228 as libc::c_int,
            ) as *mut libc::c_char,
            string,
        );
        value = 0 as *mut libc::c_void as *mut libc::c_char;
        if *name.offset(offset as isize) as libc::c_int == '=' as i32 {
            if *name.offset((offset - 1 as libc::c_int) as isize) as libc::c_int == '+' as i32 {
                appendop = 1 as libc::c_int;
                *name.offset((offset - 1 as libc::c_int) as isize) = '\u{0}' as i32 as libc::c_char;
            }
            *name.offset(offset as isize) = 0 as libc::c_int as libc::c_char;
            temp = name
                .offset(offset as isize)
                .offset(1 as libc::c_int as isize);
            if expand != 0 && (*word).flags & (1 as libc::c_int) << 15 as libc::c_int != 0 {
                ni = 1 as libc::c_int;
                assign_list = ni;
                value = extract_array_assignment_list(temp, &mut ni);
            } else if expand != 0 && *temp.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
                value = expand_string_if_necessary(
                    temp,
                    0 as libc::c_int,
                    Some(
                        expand_string_assignment
                            as fn(*mut libc::c_char, libc::c_int) -> *mut WORD_LIST,
                    ),
                );
            } else {
                value = strcpy(
                    sh_xmalloc(
                        (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(temp) as u64),
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        3253 as libc::c_int,
                    ) as *mut libc::c_char,
                    temp,
                );
            }
        }
        if value.is_null() {
            value = sh_xmalloc(
                1 as libc::c_int as size_t,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                3258 as libc::c_int,
            ) as *mut libc::c_char;
            *value.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
        }
        if echo_command_at_execute != 0 {
            if appendop != 0 {
                *name.offset((offset - 1 as libc::c_int) as isize) = '+' as i32 as libc::c_char;
            }
            xtrace_print_assignment(name, value, assign_list, 1 as libc::c_int);
            if appendop != 0 {
                *name.offset((offset - 1 as libc::c_int) as isize) = '\u{0}' as i32 as libc::c_char;
            }
        }
        if appendop != 0 {
            aflags |= 0x1 as libc::c_int;
        }
        t = mbschr(name, '[' as i32);
        if !t.is_null() {
            if assign_list != 0 {
                report_error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: cannot assign list to array member\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    name,
                );
                if !value.is_null() {
                    sh_xfree(
                        value as *mut libc::c_void,
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        3282 as libc::c_int,
                    );
                }
                sh_xfree(
                    name as *mut libc::c_void,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    3282 as libc::c_int,
                );
                return 0 as libc::c_int;
            }
            entry = assign_array_element(name, value, aflags);
            if entry.is_null() {
                if !value.is_null() {
                    sh_xfree(
                        value as *mut libc::c_void,
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        3286 as libc::c_int,
                    );
                }
                sh_xfree(
                    name as *mut libc::c_void,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    3286 as libc::c_int,
                );
                return 0 as libc::c_int;
            }
        } else if assign_list != 0 {
            if (*word).flags & (1 as libc::c_int) << 17 as libc::c_int != 0
                && (*word).flags & (1 as libc::c_int) << 28 as libc::c_int != 0
            {
                aflags |= 0x40 as libc::c_int;
            }
            if (*word).flags & (1 as libc::c_int) << 17 as libc::c_int != 0
                && (*word).flags & (1 as libc::c_int) << 25 as libc::c_int == 0 as libc::c_int
            {
                aflags |= 0x2 as libc::c_int;
            }
            if (*word).flags & (1 as libc::c_int) << 17 as libc::c_int != 0
                && (*word).flags & (1 as libc::c_int) << 25 as libc::c_int != 0
            {
                aflags |= 0x8 as libc::c_int;
            }
            if (*word).flags & (1 as libc::c_int) << 22 as libc::c_int != 0 {
                aflags |= 0x4 as libc::c_int;
            }
            entry = do_compound_assignment(name, value, aflags);
        } else {
            entry = bind_variable(name, value, aflags);
        }
        if !entry.is_null() {
            stupidly_hack_special_variables((*entry).name);
        } else {
            stupidly_hack_special_variables(name);
        }
        if entry.is_null() || (*entry).attributes & 0x2 as libc::c_int != 0 {
            retval = 0 as libc::c_int;
        } else if (*entry).attributes & 0x4000 as libc::c_int != 0 {
            set_exit_status(1 as libc::c_int);
            retval = 1 as libc::c_int;
        } else {
            retval = 1 as libc::c_int;
        }
        if !entry.is_null()
            && retval != 0 as libc::c_int
            && (*entry).attributes & 0x4000 as libc::c_int == 0 as libc::c_int
        {
            (*entry).attributes &= !(0x1000 as libc::c_int);
        }
        if !value.is_null() {
            sh_xfree(
                value as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                3323 as libc::c_int,
            );
        }
        sh_xfree(
            name as *mut libc::c_void,
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            3323 as libc::c_int,
        );

        return retval;
    }
}
#[no_mangle]
pub fn do_assignment(mut string: *mut libc::c_char) -> libc::c_int {
    let mut td: WORD_DESC = WORD_DESC {
        word: 0 as *const libc::c_char as *mut libc::c_char,
        flags: 0,
    };
    td.flags = (1 as libc::c_int) << 2 as libc::c_int;
    td.word = string;
    return do_assignment_internal(&mut td, 1 as libc::c_int);
}
#[no_mangle]
pub fn do_word_assignment(mut word: *mut WORD_DESC, mut flags: libc::c_int) -> libc::c_int {
    return do_assignment_internal(word, 1 as libc::c_int);
}
#[no_mangle]
pub fn do_assignment_no_expand(mut string: *mut libc::c_char) -> libc::c_int {
    let mut td: WORD_DESC = WORD_DESC {
        word: 0 as *const libc::c_char as *mut libc::c_char,
        flags: 0,
    };
    td.flags = (1 as libc::c_int) << 2 as libc::c_int;
    td.word = string;
    return do_assignment_internal(&mut td, 0 as libc::c_int);
}
#[no_mangle]
pub fn list_rest_of_args() -> *mut WORD_LIST {
    unsafe {
        let mut list: *mut WORD_LIST = 0 as *mut WORD_LIST;
        let mut args: *mut WORD_LIST = 0 as *mut WORD_LIST;
        let mut i: libc::c_int = 0;
        i = 1 as libc::c_int;
        list = 0 as *mut libc::c_void as *mut WORD_LIST;
        while i < 10 as libc::c_int && !(*dollar_vars.as_mut_ptr().offset(i as isize)).is_null() {
            list = make_word_list(
                make_bare_word(*dollar_vars.as_mut_ptr().offset(i as isize)),
                list,
            );
            i += 1;
        }
        args = rest_of_args;
        while !args.is_null() {
            list = make_word_list(make_bare_word((*(*args).word).word), list);
            args = (*args).next;
        }
        return if !list.is_null() && !((*list).next).is_null() {
            list_reverse(list as *mut GENERIC_LIST) as *mut WORD_LIST
        } else {
            list
        };
    }
}
#[no_mangle]
pub fn get_dollar_var_value(mut ind: intmax_t) -> *mut libc::c_char {
    unsafe {
        let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p: *mut WORD_LIST = 0 as *mut WORD_LIST;
        if ind < 10 as libc::c_int as libc::c_long {
            temp = if !(*dollar_vars.as_mut_ptr().offset(ind as isize)).is_null() {
                strcpy(
                    sh_xmalloc(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_add(
                                strlen(*dollar_vars.as_mut_ptr().offset(ind as isize)) as u64
                            ),
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        3395 as libc::c_int,
                    ) as *mut libc::c_char,
                    *dollar_vars.as_mut_ptr().offset(ind as isize),
                )
            } else {
                0 as *mut libc::c_void as *mut libc::c_char
            };
        } else {
            ind -= 10 as libc::c_int as libc::c_long;
            p = rest_of_args;
            while !p.is_null() && {
                let fresh17 = ind;
                ind = ind - 1;
                fresh17 != 0
            } {
                p = (*p).next;
            }
            temp = if !p.is_null() {
                strcpy(
                    sh_xmalloc(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_add(strlen((*(*p).word).word) as u64),
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        3401 as libc::c_int,
                    ) as *mut libc::c_char,
                    (*(*p).word).word,
                )
            } else {
                0 as *mut libc::c_void as *mut libc::c_char
            };
        }

        return temp;
    }
}
#[no_mangle]
pub fn string_rest_of_args(mut dollar_star: libc::c_int) -> *mut libc::c_char {
    let mut list: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
    list = list_rest_of_args();
    string = if dollar_star != 0 {
        string_list_dollar_star(list, 0 as libc::c_int, 0 as libc::c_int)
    } else {
        string_list(list)
    };
    dispose_words(list);

    return string;
}
fn pos_params(
    mut string: *mut libc::c_char,
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut quoted: libc::c_int,
    mut pflags: libc::c_int,
) -> *mut libc::c_char {
    unsafe {
        let mut save: *mut WORD_LIST = 0 as *mut WORD_LIST;
        let mut params: *mut WORD_LIST = 0 as *mut WORD_LIST;
        let mut h: *mut WORD_LIST = 0 as *mut WORD_LIST;
        let mut t: *mut WORD_LIST = 0 as *mut WORD_LIST;
        let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut i: libc::c_int = 0;
        if start == end {
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
        params = list_rest_of_args();
        save = params;
        if save.is_null() && start > 0 as libc::c_int {
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
        if start == 0 as libc::c_int {
            t = make_word_list(
                make_word(*dollar_vars.as_mut_ptr().offset(0 as libc::c_int as isize)),
                params,
            );
            params = t;
            save = params;
        }
        i = if start != 0 {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        while !params.is_null() && i < start {
            params = (*params).next;
            i += 1;
        }
        if params.is_null() {
            dispose_words(save);
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
        t = params;
        h = t;
        while !params.is_null() && i < end {
            t = params;
            params = (*params).next;
            i += 1;
        }
        let ref mut fresh18 = (*t).next;
        *fresh18 = 0 as *mut libc::c_void as *mut WORD_LIST;
        ret = string_list_pos_params(
            *string.offset(0 as libc::c_int as isize) as libc::c_int,
            h,
            quoted,
            pflags,
        );
        if t != params {
            let ref mut fresh19 = (*t).next;
            *fresh19 = params;
        }
        dispose_words(save);

        return ret;
    }
}
fn expand_string_if_necessary(
    mut string: *mut libc::c_char,
    mut quoted: libc::c_int,
    mut func: Option<EXPFUNC>,
) -> *mut libc::c_char {
    unsafe {
        let mut list: *mut WORD_LIST = 0 as *mut WORD_LIST;
        let mut slen: size_t = 0;
        let mut i: libc::c_int = 0;
        let mut saw_quote: libc::c_int = 0;
        let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut state: mbstate_t = mbstate_t {
            __count: 0,
            __value: mbstate_t_value { __wch: 0 },
        };
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<mbstate_t>() as usize,
        );
        slen = if __ctype_get_mb_cur_max() > 1 as libc::c_int as usize {
            strlen(string) as u64
        } else {
            0 as libc::c_int as libc::c_ulong
        };
        saw_quote = 0 as libc::c_int;
        i = saw_quote;
        while *string.offset(i as isize) != 0 {
            if *string.offset(i as isize) as libc::c_int == '$' as i32
                || *string.offset(i as isize) as libc::c_int == '`' as i32
                || *string.offset(i as isize) as libc::c_int == '<' as i32
                || *string.offset(i as isize) as libc::c_int == '>' as i32
                || *string.offset(i as isize) as libc::c_int == '\u{1}' as i32
                || *string.offset(i as isize) as libc::c_int == '~' as i32
            {
                break;
            }
            if *string.offset(i as isize) as libc::c_int == '\'' as i32
                || *string.offset(i as isize) as libc::c_int == '\\' as i32
                || *string.offset(i as isize) as libc::c_int == '"' as i32
            {
                saw_quote = 1 as libc::c_int;
            }
            if locale_mb_cur_max > 1 as libc::c_int {
                let mut state_bak: mbstate_t = mbstate_t {
                    __count: 0,
                    __value: mbstate_t_value { __wch: 0 },
                };
                let mut mblength: size_t = 0;
                let mut _f: libc::c_int = 0;
                _f = is_basic(*string.offset(i as isize));
                if _f != 0 {
                    mblength = 1 as libc::c_int as size_t;
                } else if locale_utf8locale != 0
                    && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                        == 0 as libc::c_int
                {
                    mblength = (*string.offset(i as isize) as libc::c_int != 0 as libc::c_int)
                        as libc::c_int as size_t;
                } else {
                    state_bak = state;
                    mblength = mbrlen(
                        string.offset(i as isize),
                        slen.wrapping_sub(i as libc::c_ulong),
                        &mut state,
                    );
                }
                if mblength == -(2 as libc::c_int) as size_t
                    || mblength == -(1 as libc::c_int) as size_t
                {
                    state = state_bak;
                    i += 1;
                } else if mblength == 0 as libc::c_int as libc::c_ulong {
                    i += 1;
                } else {
                    i = (i as libc::c_ulong).wrapping_add(mblength) as libc::c_int as libc::c_int;
                }
            } else {
                i += 1;
            }
        }
        if *string.offset(i as isize) != 0 {
            list = (Some(func.expect("non-null function pointer")))
                .expect("non-null function pointer")(string, quoted);
            if !list.is_null() {
                ret = string_list(list);
                dispose_words(list);
            } else {
                ret = 0 as *mut libc::c_void as *mut libc::c_char;
            }
        } else if saw_quote != 0
            && quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) == 0 as libc::c_int
        {
            ret = string_quote_removal(string, quoted);
        } else {
            ret = strcpy(
                sh_xmalloc(
                    (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(string) as u64),
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    3526 as libc::c_int,
                ) as *mut libc::c_char,
                string,
            );
        }

        return ret;
    }
}
#[inline]
fn expand_string_to_string_internal(
    mut string: *mut libc::c_char,
    mut quoted: libc::c_int,
    mut func: Option<EXPFUNC>,
) -> *mut libc::c_char {
    unsafe {
        let mut list: *mut WORD_LIST = 0 as *mut WORD_LIST;
        let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
        if string.is_null() || *string as libc::c_int == '\u{0}' as i32 {
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
        list = (Some(func.expect("non-null function pointer"))).expect("non-null function pointer")(
            string, quoted,
        );
        if !list.is_null() {
            ret = string_list(list);
            dispose_words(list);
        } else {
            ret = 0 as *mut libc::c_void as *mut libc::c_char;
        }
        return ret;
    }
}
#[no_mangle]
pub fn expand_string_to_string(
    mut string: *mut libc::c_char,
    mut quoted: libc::c_int,
) -> *mut libc::c_char {
    return expand_string_to_string_internal(
        string,
        quoted,
        Some(expand_string as fn(*mut libc::c_char, libc::c_int) -> *mut WORD_LIST),
    );
}
#[no_mangle]
pub fn expand_string_unsplit_to_string(
    mut string: *mut libc::c_char,
    mut quoted: libc::c_int,
) -> *mut libc::c_char {
    return expand_string_to_string_internal(
        string,
        quoted,
        Some(expand_string_unsplit as fn(*mut libc::c_char, libc::c_int) -> *mut WORD_LIST),
    );
}
#[no_mangle]
pub fn expand_assignment_string_to_string(
    mut string: *mut libc::c_char,
    mut quoted: libc::c_int,
) -> *mut libc::c_char {
    return expand_string_to_string_internal(
        string,
        quoted,
        Some(expand_string_assignment as fn(*mut libc::c_char, libc::c_int) -> *mut WORD_LIST),
    );
}
#[no_mangle]
pub fn expand_arith_string(
    mut string: *mut libc::c_char,
    mut quoted: libc::c_int,
) -> *mut libc::c_char {
    unsafe {
        let mut td: WORD_DESC = WORD_DESC {
            word: 0 as *const libc::c_char as *mut libc::c_char,
            flags: 0,
        };
        let mut list: *mut WORD_LIST = 0 as *mut WORD_LIST;
        let mut tlist: *mut WORD_LIST = 0 as *mut WORD_LIST;
        let mut slen: size_t = 0;
        let mut i: libc::c_int = 0;
        let mut saw_quote: libc::c_int = 0;
        let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut state: mbstate_t = mbstate_t {
            __count: 0,
            __value: mbstate_t_value { __wch: 0 },
        };
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<mbstate_t>() as usize,
        );
        slen = if __ctype_get_mb_cur_max() > 1 as libc::c_int as usize {
            strlen(string) as u64
        } else {
            0 as libc::c_int as u64
        };
        saw_quote = 0 as libc::c_int;
        i = saw_quote;
        while *string.offset(i as isize) != 0 {
            if *string.offset(i as isize) as libc::c_int == '$' as i32
                || *string.offset(i as isize) as libc::c_int == '`' as i32
                || *string.offset(i as isize) as libc::c_int == '<' as i32
                || *string.offset(i as isize) as libc::c_int == '>' as i32
                || *string.offset(i as isize) as libc::c_int == '\u{1}' as i32
                || *string.offset(i as isize) as libc::c_int == '~' as i32
            {
                break;
            }
            if *string.offset(i as isize) as libc::c_int == '\'' as i32
                || *string.offset(i as isize) as libc::c_int == '\\' as i32
                || *string.offset(i as isize) as libc::c_int == '"' as i32
            {
                saw_quote = 1 as libc::c_int;
            }
            if locale_mb_cur_max > 1 as libc::c_int {
                let mut state_bak: mbstate_t = mbstate_t {
                    __count: 0,
                    __value: mbstate_t_value { __wch: 0 },
                };
                let mut mblength: size_t = 0;
                let mut _f: libc::c_int = 0;
                _f = is_basic(*string.offset(i as isize));
                if _f != 0 {
                    mblength = 1 as libc::c_int as size_t;
                } else if locale_utf8locale != 0
                    && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                        == 0 as libc::c_int
                {
                    mblength = (*string.offset(i as isize) as libc::c_int != 0 as libc::c_int)
                        as libc::c_int as size_t;
                } else {
                    state_bak = state;
                    mblength = mbrlen(
                        string.offset(i as isize),
                        slen.wrapping_sub(i as libc::c_ulong),
                        &mut state,
                    );
                }
                if mblength == -(2 as libc::c_int) as size_t
                    || mblength == -(1 as libc::c_int) as size_t
                {
                    state = state_bak;
                    i += 1;
                } else if mblength == 0 as libc::c_int as libc::c_ulong {
                    i += 1;
                } else {
                    i = (i as libc::c_ulong).wrapping_add(mblength) as libc::c_int as libc::c_int;
                }
            } else {
                i += 1;
            }
        }
        if *string.offset(i as isize) != 0 {
            td.flags =
                (1 as libc::c_int) << 20 as libc::c_int | (1 as libc::c_int) << 12 as libc::c_int;
            td.word = strcpy(
                sh_xmalloc(
                    (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(string) as u64),
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    3612 as libc::c_int,
                ) as *mut libc::c_char,
                string,
            );
            list = call_expand_word_internal(
                &mut td,
                quoted,
                0 as libc::c_int,
                0 as *mut libc::c_void as *mut libc::c_int,
                0 as *mut libc::c_void as *mut libc::c_int,
            );
            if !list.is_null() {
                tlist = word_list_split(list);
                dispose_words(list);
                list = tlist;
                if !list.is_null() {
                    dequote_list(list);
                }
            }
            if !list.is_null() {
                ret = string_list(list);
                dispose_words(list);
            } else {
                ret = 0 as *mut libc::c_void as *mut libc::c_char;
            }
            if !(td.word).is_null() {
                sh_xfree(
                    td.word as *mut libc::c_void,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    3632 as libc::c_int,
                );
            }
        } else if saw_quote != 0 && quoted & 0x100 as libc::c_int != 0 {
            ret = string_quote_removal(string, quoted);
        } else if saw_quote != 0
            && quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) == 0 as libc::c_int
        {
            ret = string_quote_removal(string, quoted);
        } else {
            ret = strcpy(
                sh_xmalloc(
                    (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(string) as u64),
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    3639 as libc::c_int,
                ) as *mut libc::c_char,
                string,
            );
        }

        return ret;
    }
}
#[no_mangle]
pub fn remove_backslashes(mut string: *mut libc::c_char) -> *mut libc::c_char {
    unsafe {
        let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        ret = sh_xmalloc(
            (strlen(string)).wrapping_add(1 as libc::c_int as u64) as u64,
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            3652 as libc::c_int,
        ) as *mut libc::c_char;
        r = ret;
        s = string;
        while !s.is_null() && *s as libc::c_int != 0 {
            if *s as libc::c_int == '\\' as i32 {
                s = s.offset(1);
            }
            if *s as libc::c_int == 0 as libc::c_int {
                break;
            }
            let fresh20 = s;
            s = s.offset(1);
            let fresh21 = r;
            r = r.offset(1);
            *fresh21 = *fresh20;
        }
        *r = '\u{0}' as i32 as libc::c_char;

        return ret;
    }
}
#[no_mangle]
pub fn cond_expand_word(mut w: *mut WORD_DESC, mut special: libc::c_int) -> *mut libc::c_char {
    unsafe {
        let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut l: *mut WORD_LIST = 0 as *mut WORD_LIST;
        let mut qflags: libc::c_int = 0;
        if ((*w).word).is_null()
            || *((*w).word).offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
        {
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
        expand_no_split_dollar_star = 1 as libc::c_int;
        (*w).flags |= (1 as libc::c_int) << 6 as libc::c_int;
        l = call_expand_word_internal(
            w,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as *mut libc::c_int,
            0 as *mut libc::c_int,
        );
        expand_no_split_dollar_star = 0 as libc::c_int;
        if !l.is_null() {
            if special == 0 as libc::c_int {
                if !((*l).word).is_null() {
                    word_list_remove_quoted_nulls(l);
                }
                dequote_list(l);
                r = string_list(l);
            } else {
                qflags = 0x1 as libc::c_int | 0x8 as libc::c_int;
                if special == 2 as libc::c_int {
                    qflags |= 0x4 as libc::c_int;
                }
                word_list_remove_quoted_nulls(l);
                p = string_list(l);
                r = quote_string_for_globbing(p, qflags);
                sh_xfree(
                    p as *mut libc::c_void,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    3711 as libc::c_int,
                );
            }
            dispose_words(l);
        } else {
            r = 0 as *mut libc::c_void as *mut libc::c_char;
        }

        return r;
    }
}
fn call_expand_word_internal(
    mut w: *mut WORD_DESC,
    mut q: libc::c_int,
    mut i: libc::c_int,
    mut c: *mut libc::c_int,
    mut e: *mut libc::c_int,
) -> *mut WORD_LIST {
    unsafe {
        let mut result: *mut WORD_LIST = 0 as *mut WORD_LIST;
        result = expand_word_internal(w, q, i, c, e);
        if result == &mut expand_word_error as *mut WORD_LIST
            || result == &mut expand_word_fatal as *mut WORD_LIST
        {
            let ref mut fresh22 = (*w).word;
            *fresh22 = 0 as *mut libc::c_void as *mut libc::c_char;
            ::std::ptr::write_volatile(
                &mut last_command_exit_value as *mut libc::c_int,
                1 as libc::c_int,
            );
            exp_jump_to_top_level(if result == &mut expand_word_error as *mut WORD_LIST {
                2 as libc::c_int
            } else {
                1 as libc::c_int
            });
            return 0 as *mut WORD_LIST;
        } else {
            return result;
        };
    }
}
fn expand_string_internal(
    mut string: *mut libc::c_char,
    mut quoted: libc::c_int,
) -> *mut WORD_LIST {
    unsafe {
        let mut td: WORD_DESC = WORD_DESC {
            word: 0 as *const libc::c_char as *mut libc::c_char,
            flags: 0,
        };
        let mut tresult: *mut WORD_LIST = 0 as *mut WORD_LIST;
        if string.is_null() || *string as libc::c_int == 0 as libc::c_int {
            return 0 as *mut libc::c_void as *mut WORD_LIST;
        }
        td.flags = 0 as libc::c_int;
        td.word = strcpy(
            sh_xmalloc(
                (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(string) as u64),
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                3765 as libc::c_int,
            ) as *mut libc::c_char,
            string,
        );
        tresult = call_expand_word_internal(
            &mut td,
            quoted,
            0 as libc::c_int,
            0 as *mut libc::c_void as *mut libc::c_int,
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        if !(td.word).is_null() {
            sh_xfree(
                td.word as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                3769 as libc::c_int,
            );
        }

        return tresult;
    }
}
#[no_mangle]
pub fn expand_string_unsplit(
    mut string: *mut libc::c_char,
    mut quoted: libc::c_int,
) -> *mut WORD_LIST {
    unsafe {
        let mut value: *mut WORD_LIST = 0 as *mut WORD_LIST;
        if string.is_null() || *string as libc::c_int == '\u{0}' as i32 {
            return 0 as *mut libc::c_void as *mut WORD_LIST;
        }
        expand_no_split_dollar_star = 1 as libc::c_int;
        value = expand_string_internal(string, quoted);
        expand_no_split_dollar_star = 0 as libc::c_int;
        if !value.is_null() {
            if !((*value).word).is_null() {
                remove_quoted_nulls((*(*value).word).word);
                (*(*value).word).flags &= !((1 as libc::c_int) << 18 as libc::c_int);
            }
            dequote_list(value);
        }

        return value;
    }
}
#[no_mangle]
pub fn expand_string_assignment(
    mut string: *mut libc::c_char,
    mut quoted: libc::c_int,
) -> *mut WORD_LIST {
    unsafe {
        let mut td: WORD_DESC = WORD_DESC {
            word: 0 as *const libc::c_char as *mut libc::c_char,
            flags: 0,
        };
        let mut value: *mut WORD_LIST = 0 as *mut WORD_LIST;
        if string.is_null() || *string as libc::c_int == '\u{0}' as i32 {
            return 0 as *mut libc::c_void as *mut WORD_LIST;
        }
        expand_no_split_dollar_star = 1 as libc::c_int;
        td.flags = (1 as libc::c_int) << 11 as libc::c_int;
        td.word = strcpy(
            sh_xmalloc(
                (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(string) as u64),
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                3828 as libc::c_int,
            ) as *mut libc::c_char,
            string,
        );
        value = call_expand_word_internal(
            &mut td,
            quoted,
            0 as libc::c_int,
            0 as *mut libc::c_void as *mut libc::c_int,
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        if !(td.word).is_null() {
            sh_xfree(
                td.word as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                3830 as libc::c_int,
            );
        }
        expand_no_split_dollar_star = 0 as libc::c_int;
        if !value.is_null() {
            if !((*value).word).is_null() {
                remove_quoted_nulls((*(*value).word).word);
                (*(*value).word).flags &= !((1 as libc::c_int) << 18 as libc::c_int);
            }
            dequote_list(value);
        }

        return value;
    }
}
#[no_mangle]
pub fn expand_prompt_string(
    mut string: *mut libc::c_char,
    mut quoted: libc::c_int,
    mut wflags: libc::c_int,
) -> *mut WORD_LIST {
    unsafe {
        let mut value: *mut WORD_LIST = 0 as *mut WORD_LIST;
        let mut td: WORD_DESC = WORD_DESC {
            word: 0 as *const libc::c_char as *mut libc::c_char,
            flags: 0,
        };
        if string.is_null() || *string as libc::c_int == 0 as libc::c_int {
            return 0 as *mut libc::c_void as *mut WORD_LIST;
        }
        td.flags = wflags;
        td.word = strcpy(
            sh_xmalloc(
                (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(string) as u64),
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                3864 as libc::c_int,
            ) as *mut libc::c_char,
            string,
        );
        no_longjmp_on_fatal_error = 1 as libc::c_int;
        value = expand_word_internal(
            &mut td,
            quoted,
            0 as libc::c_int,
            0 as *mut libc::c_void as *mut libc::c_int,
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        no_longjmp_on_fatal_error = 0 as libc::c_int;
        if value == &mut expand_word_error as *mut WORD_LIST
            || value == &mut expand_word_fatal as *mut WORD_LIST
        {
            value = make_word_list(
                make_bare_word(string),
                0 as *mut libc::c_void as *mut WORD_LIST,
            );
            return value;
        }
        if !(td.word).is_null() {
            sh_xfree(
                td.word as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                3875 as libc::c_int,
            );
        }
        if !value.is_null() {
            if !((*value).word).is_null() {
                remove_quoted_nulls((*(*value).word).word);
                (*(*value).word).flags &= !((1 as libc::c_int) << 18 as libc::c_int);
            }
            dequote_list(value);
        }

        return value;
    }
}
fn expand_string_leave_quoted(
    mut string: *mut libc::c_char,
    mut quoted: libc::c_int,
) -> *mut WORD_LIST {
    unsafe {
        let mut tlist: *mut WORD_LIST = 0 as *mut WORD_LIST;
        let mut tresult: *mut WORD_LIST = 0 as *mut WORD_LIST;
        if string.is_null() || *string as libc::c_int == '\u{0}' as i32 {
            return 0 as *mut libc::c_void as *mut WORD_LIST;
        }
        tlist = expand_string_internal(string, quoted);
        if !tlist.is_null() {
            tresult = word_list_split(tlist);
            dispose_words(tlist);
            return tresult;
        }
    }
    return 0 as *mut libc::c_void as *mut WORD_LIST;
}
fn expand_string_for_rhs(
    mut string: *mut libc::c_char,
    mut quoted: libc::c_int,
    mut op: libc::c_int,
    mut pflags: libc::c_int,
    mut dollar_at_p: *mut libc::c_int,
    mut expanded_p: *mut libc::c_int,
) -> *mut WORD_LIST {
    unsafe {
        let mut td: WORD_DESC = WORD_DESC {
            word: 0 as *const libc::c_char as *mut libc::c_char,
            flags: 0,
        };
        let mut tresult: *mut WORD_LIST = 0 as *mut WORD_LIST;
        let mut old_nosplit: libc::c_int = 0;
        if string.is_null() || *string as libc::c_int == '\u{0}' as i32 {
            return 0 as *mut libc::c_void as *mut WORD_LIST;
        }
        old_nosplit = expand_no_split_dollar_star;
        expand_no_split_dollar_star = (quoted & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0
            || op == '=' as i32
            || ifs_is_null == 0 as libc::c_int)
            as libc::c_int;
        td.flags = (1 as libc::c_int) << 14 as libc::c_int;
        td.flags |= (1 as libc::c_int) << 6 as libc::c_int;
        if pflags & 0x8 as libc::c_int != 0 {
            td.flags |= (1 as libc::c_int) << 11 as libc::c_int;
        }
        if op == '=' as i32 {
            td.flags |=
                (1 as libc::c_int) << 11 as libc::c_int | (1 as libc::c_int) << 29 as libc::c_int;
        }
        td.word = string;
        tresult =
            call_expand_word_internal(&mut td, quoted, 1 as libc::c_int, dollar_at_p, expanded_p);
        expand_no_split_dollar_star = old_nosplit;

        return tresult;
    }
}
fn expand_string_for_pat(
    mut string: *mut libc::c_char,
    mut quoted: libc::c_int,
    mut dollar_at_p: *mut libc::c_int,
    mut expanded_p: *mut libc::c_int,
) -> *mut WORD_LIST {
    unsafe {
        let mut td: WORD_DESC = WORD_DESC {
            word: 0 as *const libc::c_char as *mut libc::c_char,
            flags: 0,
        };
        let mut tresult: *mut WORD_LIST = 0 as *mut WORD_LIST;
        let mut oexp: libc::c_int = 0;
        if string.is_null() || *string as libc::c_int == '\u{0}' as i32 {
            return 0 as *mut libc::c_void as *mut WORD_LIST;
        }
        oexp = expand_no_split_dollar_star;
        expand_no_split_dollar_star = 1 as libc::c_int;
        td.flags = (1 as libc::c_int) << 6 as libc::c_int;
        td.word = string;
        tresult =
            call_expand_word_internal(&mut td, quoted, 1 as libc::c_int, dollar_at_p, expanded_p);
        expand_no_split_dollar_star = oexp;

        return tresult;
    }
}
#[no_mangle]
pub fn expand_string(mut string: *mut libc::c_char, mut quoted: libc::c_int) -> *mut WORD_LIST {
    unsafe {
        let mut result: *mut WORD_LIST = 0 as *mut WORD_LIST;
        if string.is_null() || *string as libc::c_int == '\u{0}' as i32 {
            return 0 as *mut libc::c_void as *mut WORD_LIST;
        }
        result = expand_string_leave_quoted(string, quoted);
        return if !result.is_null() {
            dequote_list(result)
        } else {
            result
        };
    }
}
#[no_mangle]
pub fn expand_word(mut word: *mut WORD_DESC, mut quoted: libc::c_int) -> *mut WORD_LIST {
    let mut result: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut tresult: *mut WORD_LIST = 0 as *mut WORD_LIST;
    tresult = call_expand_word_internal(
        word,
        quoted,
        0 as libc::c_int,
        0 as *mut libc::c_void as *mut libc::c_int,
        0 as *mut libc::c_void as *mut libc::c_int,
    );
    result = word_list_split(tresult);
    dispose_words(tresult);
    return if !result.is_null() {
        dequote_list(result)
    } else {
        result
    };
}
#[no_mangle]
pub fn expand_word_unsplit(mut word: *mut WORD_DESC, mut quoted: libc::c_int) -> *mut WORD_LIST {
    let mut result: *mut WORD_LIST = 0 as *mut WORD_LIST;
    result = expand_word_leave_quoted(word, quoted);
    return if !result.is_null() {
        dequote_list(result)
    } else {
        result
    };
}
#[no_mangle]
pub fn expand_word_leave_quoted(
    mut word: *mut WORD_DESC,
    mut quoted: libc::c_int,
) -> *mut WORD_LIST {
    unsafe {
        let mut result: *mut WORD_LIST = 0 as *mut WORD_LIST;
        expand_no_split_dollar_star = 1 as libc::c_int;
        if ifs_is_null != 0 {
            (*word).flags |= (1 as libc::c_int) << 4 as libc::c_int;
        }
        (*word).flags |= (1 as libc::c_int) << 6 as libc::c_int;
        result = call_expand_word_internal(
            word,
            quoted,
            0 as libc::c_int,
            0 as *mut libc::c_void as *mut libc::c_int,
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        expand_no_split_dollar_star = 0 as libc::c_int;

        return result;
    }
}
fn quote_escapes_internal(
    mut string: *const libc::c_char,
    mut flags: libc::c_int,
) -> *mut libc::c_char {
    unsafe {
        let mut s: *const libc::c_char = 0 as *const libc::c_char;
        let mut send: *const libc::c_char = 0 as *const libc::c_char;
        let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut slen: size_t = 0;
        let mut quote_spaces: libc::c_int = 0;
        let mut skip_ctlesc: libc::c_int = 0;
        let mut skip_ctlnul: libc::c_int = 0;
        let mut nosplit: libc::c_int = 0;
        let mut state: mbstate_t = mbstate_t {
            __count: 0,
            __value: mbstate_t_value { __wch: 0 },
        };
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<mbstate_t>() as usize,
        );
        slen = strlen(string) as u64;
        send = string.offset(slen as isize);
        quote_spaces =
            (!ifs_value.is_null() && *ifs_value as libc::c_int == 0 as libc::c_int) as libc::c_int;
        nosplit = flags & 0x4 as libc::c_int;
        skip_ctlnul = 0 as libc::c_int;
        skip_ctlesc = skip_ctlnul;
        s = ifs_value;
        while !s.is_null() && *s as libc::c_int != 0 {
            skip_ctlesc |=
                (nosplit == 0 as libc::c_int && *s as libc::c_int == '\u{1}' as i32) as libc::c_int;
            skip_ctlnul |= (nosplit == 0 as libc::c_int && *s as libc::c_int == '\u{7f}' as i32)
                as libc::c_int;
            s = s.offset(1);
        }
        result = sh_xmalloc(
            slen.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            4109 as libc::c_int,
        ) as *mut libc::c_char;
        t = result;
        s = string;
        while *s != 0 {
            if skip_ctlesc == 0 as libc::c_int && *s as libc::c_int == '\u{1}' as i32
                || skip_ctlnul == 0 as libc::c_int && *s as libc::c_int == '\u{7f}' as i32
                || quote_spaces != 0 && *s as libc::c_int == ' ' as i32
            {
                let fresh23 = t;
                t = t.offset(1);
                *fresh23 = '\u{1}' as i32 as libc::c_char;
            }
            if locale_mb_cur_max > 1 as libc::c_int {
                let mut state_bak: mbstate_t = mbstate_t {
                    __count: 0,
                    __value: mbstate_t_value { __wch: 0 },
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
                    let fresh24 = s;
                    s = s.offset(1);
                    let fresh25 = t;
                    t = t.offset(1);
                    *fresh25 = *fresh24;
                    _k += 1;
                }
            } else {
                let fresh26 = s;
                s = s.offset(1);
                let fresh27 = t;
                t = t.offset(1);
                *fresh27 = *fresh26;
            }
        }
        *t = '\u{0}' as i32 as libc::c_char;

        return result;
    }
}
#[no_mangle]
pub fn quote_escapes(mut string: *const libc::c_char) -> *mut libc::c_char {
    return quote_escapes_internal(string, 0 as libc::c_int);
}
#[no_mangle]
pub fn quote_rhs(mut string: *const libc::c_char) -> *mut libc::c_char {
    return quote_escapes_internal(string, 0x4 as libc::c_int);
}
fn list_quote_escapes(mut list: *mut WORD_LIST) -> *mut WORD_LIST {
    unsafe {
        let mut w: *mut WORD_LIST = 0 as *mut WORD_LIST;
        let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
        w = list;
        while !w.is_null() {
            t = (*(*w).word).word;
            let ref mut fresh28 = (*(*w).word).word;
            *fresh28 = quote_escapes(t);
            sh_xfree(
                t as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                4148 as libc::c_int,
            );
            w = (*w).next;
        }
    }
    return list;
}
#[no_mangle]
pub fn dequote_escapes(mut string: *const libc::c_char) -> *mut libc::c_char {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut send: *const libc::c_char = 0 as *const libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut slen: size_t = 0;
    let mut quote_spaces: libc::c_int = 0;
    let mut state: mbstate_t = mbstate_t {
        __count: 0,
        __value: mbstate_t_value { __wch: 0 },
    };
    unsafe {
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<mbstate_t>() as usize,
        );
        if string.is_null() {
            return 0 as *mut libc::c_char;
        }
        slen = strlen(string) as u64;
        send = string.offset(slen as isize);
        result = sh_xmalloc(
            slen.wrapping_add(1 as libc::c_int as libc::c_ulong),
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            4180 as libc::c_int,
        ) as *mut libc::c_char;
        t = result;
        if (strchr(string, '\u{1}' as i32)).is_null() {
            return strcpy(result, string);
        }
        quote_spaces =
            (!ifs_value.is_null() && *ifs_value as libc::c_int == 0 as libc::c_int) as libc::c_int;
        s = string;
        while *s != 0 {
            if *s as libc::c_int == '\u{1}' as i32
                && (*s.offset(1 as libc::c_int as isize) as libc::c_int == '\u{1}' as i32
                    || *s.offset(1 as libc::c_int as isize) as libc::c_int == '\u{7f}' as i32
                    || quote_spaces != 0
                        && *s.offset(1 as libc::c_int as isize) as libc::c_int == ' ' as i32)
            {
                s = s.offset(1);
                if *s as libc::c_int == '\u{0}' as i32 {
                    break;
                }
            }
            if locale_mb_cur_max > 1 as libc::c_int {
                let mut state_bak: mbstate_t = mbstate_t {
                    __count: 0,
                    __value: mbstate_t_value { __wch: 0 },
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
                    let fresh29 = s;
                    s = s.offset(1);
                    let fresh30 = t;
                    t = t.offset(1);
                    *fresh30 = *fresh29;
                    _k += 1;
                }
            } else {
                let fresh31 = s;
                s = s.offset(1);
                let fresh32 = t;
                t = t.offset(1);
                *fresh32 = *fresh31;
            }
        }
        *t = '\u{0}' as i32 as libc::c_char;
    }
    return result;
}
fn make_quoted_char(mut c: libc::c_int) -> *mut libc::c_char {
    unsafe {
        let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
        temp = sh_xmalloc(
            3 as libc::c_int as size_t,
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            4230 as libc::c_int,
        ) as *mut libc::c_char;
        if c == 0 as libc::c_int {
            *temp.offset(0 as libc::c_int as isize) = '\u{7f}' as i32 as libc::c_char;
            *temp.offset(1 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
        } else {
            *temp.offset(0 as libc::c_int as isize) = '\u{1}' as i32 as libc::c_char;
            *temp.offset(1 as libc::c_int as isize) = c as libc::c_char;
            *temp.offset(2 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
        }
        return temp;
    }
}
#[no_mangle]
pub fn quote_string(mut string: *mut libc::c_char) -> *mut libc::c_char {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut slen: size_t = 0;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut send: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        if *string as libc::c_int == 0 as libc::c_int {
            result = sh_xmalloc(
                2 as libc::c_int as size_t,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                4258 as libc::c_int,
            ) as *mut libc::c_char;
            *result.offset(0 as libc::c_int as isize) = '\u{7f}' as i32 as libc::c_char;
            *result.offset(1 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
        } else {
            let mut state: mbstate_t = mbstate_t {
                __count: 0,
                __value: mbstate_t_value { __wch: 0 },
            };
            memset(
                &mut state as *mut mbstate_t as *mut libc::c_void,
                '\u{0}' as i32,
                ::std::mem::size_of::<mbstate_t>() as usize,
            );
            slen = strlen(string) as u64;
            send = string.offset(slen as isize);
            result = sh_xmalloc(
                slen.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                4269 as libc::c_int,
            ) as *mut libc::c_char;
            t = result;
            while string < send {
                let fresh33 = t;
                t = t.offset(1);
                *fresh33 = '\u{1}' as i32 as libc::c_char;
                if locale_mb_cur_max > 1 as libc::c_int {
                    let mut state_bak: mbstate_t = mbstate_t {
                        __count: 0,
                        __value: mbstate_t_value { __wch: 0 },
                    };
                    let mut mblength: size_t = 0;
                    let mut _k: libc::c_int = 0;
                    _k = is_basic(*string);
                    if _k != 0 {
                        mblength = 1 as libc::c_int as size_t;
                    } else if locale_utf8locale != 0
                        && *string as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                    {
                        mblength =
                            (*string as libc::c_int != 0 as libc::c_int) as libc::c_int as size_t;
                    } else {
                        state_bak = state;
                        mblength = mbrlen(
                            string,
                            send.offset_from(string) as libc::c_long as size_t,
                            &mut state,
                        );
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
                        let fresh34 = string;
                        string = string.offset(1);
                        let fresh35 = t;
                        t = t.offset(1);
                        *fresh35 = *fresh34;
                        _k += 1;
                    }
                } else {
                    let fresh36 = string;
                    string = string.offset(1);
                    let fresh37 = t;
                    t = t.offset(1);
                    *fresh37 = *fresh36;
                }
            }
            *t = '\u{0}' as i32 as libc::c_char;
        }
    }
    return result;
}
#[no_mangle]
pub fn dequote_string(mut string: *mut libc::c_char) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut slen: size_t = 0;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut send: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut state: mbstate_t = mbstate_t {
        __count: 0,
        __value: mbstate_t_value { __wch: 0 },
    };
    unsafe {
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<mbstate_t>() as usize,
        );
        slen = if !string.is_null() && *string.offset(0 as libc::c_int as isize) as libc::c_int != 0
        {
            if *string.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                if *string.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                    strlen(string) as u64
                } else {
                    2 as libc::c_int as libc::c_ulong
                }
            } else {
                1 as libc::c_int as libc::c_ulong
            }
        } else {
            0 as libc::c_int as libc::c_ulong
        };
        result = sh_xmalloc(
            slen.wrapping_add(1 as libc::c_int as libc::c_ulong),
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            4298 as libc::c_int,
        ) as *mut libc::c_char;
        t = result;
        if *string.offset(0 as libc::c_int as isize) as libc::c_int == '\u{7f}' as i32
            && *string.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
        {
            *result.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
            return result;
        }
        if *string.offset(0 as libc::c_int as isize) as libc::c_int == '\u{1}' as i32
            && *string.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
        {
            *result.offset(0 as libc::c_int as isize) = '\u{1}' as i32 as libc::c_char;
            *result.offset(1 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
            return result;
        }
        if (strchr(string, '\u{1}' as i32)).is_null() {
            return strcpy(result, string);
        }
        send = string.offset(slen as isize);
        s = string;
        while *s != 0 {
            if *s as libc::c_int == '\u{1}' as i32 {
                s = s.offset(1);
                if *s as libc::c_int == '\u{0}' as i32 {
                    break;
                }
            }
            if locale_mb_cur_max > 1 as libc::c_int {
                let mut state_bak: mbstate_t = mbstate_t {
                    __count: 0,
                    __value: mbstate_t_value { __wch: 0 },
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
                    let fresh38 = s;
                    s = s.offset(1);
                    let fresh39 = t;
                    t = t.offset(1);
                    *fresh39 = *fresh38;
                    _k += 1;
                }
            } else {
                let fresh40 = s;
                s = s.offset(1);
                let fresh41 = t;
                t = t.offset(1);
                *fresh41 = *fresh40;
            }
        }
        *t = '\u{0}' as i32 as libc::c_char;
    }
    return result;
}
fn quote_list(mut list: *mut WORD_LIST) -> *mut WORD_LIST {
    let mut w: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    w = list;
    unsafe {
        while !w.is_null() {
            t = (*(*w).word).word;
            let ref mut fresh42 = (*(*w).word).word;
            *fresh42 = quote_string(t);
            if *t as libc::c_int == 0 as libc::c_int {
                (*(*w).word).flags |= (1 as libc::c_int) << 18 as libc::c_int;
            }
            (*(*w).word).flags |= (1 as libc::c_int) << 1 as libc::c_int;
            sh_xfree(
                t as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                4351 as libc::c_int,
            );
            w = (*w).next;
        }
    }
    return list;
}
#[no_mangle]
pub fn dequote_word(mut word: *mut WORD_DESC) -> *mut WORD_DESC {
    unsafe {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        s = dequote_string((*word).word);
        if *((*word).word).offset(0 as libc::c_int as isize) as libc::c_int == '\u{7f}' as i32
            && *((*word).word).offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
        {
            (*word).flags &= !((1 as libc::c_int) << 18 as libc::c_int);
        }
        sh_xfree(
            (*word).word as *mut libc::c_void,
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            4365 as libc::c_int,
        );
        let ref mut fresh43 = (*word).word;
        *fresh43 = s;
    }
    return word;
}
#[no_mangle]
pub fn dequote_list(mut list: *mut WORD_LIST) -> *mut WORD_LIST {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tlist: *mut WORD_LIST = 0 as *mut WORD_LIST;
    tlist = list;
    unsafe {
        while !tlist.is_null() {
            s = dequote_string((*(*tlist).word).word);
            if *((*(*tlist).word).word).offset(0 as libc::c_int as isize) as libc::c_int
                == '\u{7f}' as i32
                && *((*(*tlist).word).word).offset(1 as libc::c_int as isize) as libc::c_int
                    == '\u{0}' as i32
            {
                (*(*tlist).word).flags &= !((1 as libc::c_int) << 18 as libc::c_int);
            }
            sh_xfree(
                (*(*tlist).word).word as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                4384 as libc::c_int,
            );
            let ref mut fresh44 = (*(*tlist).word).word;
            *fresh44 = s;
            tlist = (*tlist).next;
        }
    }
    return list;
}
#[no_mangle]
pub fn remove_quoted_escapes(mut string: *mut libc::c_char) -> *mut libc::c_char {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        if !string.is_null() {
            t = dequote_escapes(string);
            strcpy(string, t);
            sh_xfree(
                t as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                4402 as libc::c_int,
            );
        }
    }
    return string;
}
#[no_mangle]
pub fn remove_quoted_ifs(mut string: *mut libc::c_char) -> *mut libc::c_char {
    let mut slen: size_t = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut send: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut state: mbstate_t = mbstate_t {
        __count: 0,
        __value: mbstate_t_value { __wch: 0 },
    };
    unsafe {
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<mbstate_t>() as usize,
        );
        slen = strlen(string) as u64;
        send = string.offset(slen as isize);
        j = 0 as libc::c_int;
        i = j;
        ret = sh_xmalloc(
            slen.wrapping_add(1 as libc::c_int as libc::c_ulong),
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            4425 as libc::c_int,
        ) as *mut libc::c_char;
        while (i as libc::c_ulong) < slen {
            if *string.offset(i as isize) as libc::c_int == '\u{1}' as i32 {
                i += 1;
                if *string.offset(i as isize) as libc::c_int == 0 as libc::c_int
                    || (ifs_cmap[*string.offset(i as isize) as libc::c_uchar as usize]
                        as libc::c_int
                        != 0 as libc::c_int) as libc::c_int
                        == 0 as libc::c_int
                {
                    let fresh45 = j;
                    j = j + 1;
                    *ret.offset(fresh45 as isize) = '\u{1}' as i32 as libc::c_char;
                }
                if i as libc::c_ulong == slen {
                    break;
                }
            }
            if locale_mb_cur_max > 1 as libc::c_int {
                let mut state_bak: mbstate_t = mbstate_t {
                    __count: 0,
                    __value: mbstate_t_value { __wch: 0 },
                };
                let mut mblength: size_t = 0;
                let mut _k: libc::c_int = 0;
                _k = is_basic(*string.offset(i as isize));
                if _k != 0 {
                    mblength = 1 as libc::c_int as size_t;
                } else {
                    state_bak = state;
                    mblength = mbrlen(
                        string.offset(i as isize),
                        send.offset_from(string.offset(i as isize)) as libc::c_long as size_t,
                        &mut state,
                    );
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
                    let fresh46 = i;
                    i = i + 1;
                    let fresh47 = j;
                    j = j + 1;
                    *ret.offset(fresh47 as isize) = *string.offset(fresh46 as isize);
                    _k += 1;
                }
            } else {
                let fresh48 = i;
                i = i + 1;
                let fresh49 = j;
                j = j + 1;
                *ret.offset(fresh49 as isize) = *string.offset(fresh48 as isize);
            }
        }
        *ret.offset(j as isize) = '\u{0}' as i32 as libc::c_char;
    }
    return ret;
}
#[no_mangle]
pub fn remove_quoted_nulls(mut string: *mut libc::c_char) -> *mut libc::c_char {
    let mut slen: size_t = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut prev_i: libc::c_int = 0;
    let mut state: mbstate_t = mbstate_t {
        __count: 0,
        __value: mbstate_t_value { __wch: 0 },
    };
    unsafe {
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<mbstate_t>() as usize,
        );
        if (strchr(string, '\u{7f}' as i32)).is_null() {
            return string;
        }
        slen = strlen(string) as u64;
        j = 0 as libc::c_int;
        i = j;
        while (i as libc::c_ulong) < slen {
            if *string.offset(i as isize) as libc::c_int == '\u{1}' as i32 {
                i += 1;
                let fresh50 = j;
                j = j + 1;
                *string.offset(fresh50 as isize) = '\u{1}' as i32 as libc::c_char;
                if i as libc::c_ulong == slen {
                    break;
                }
            } else if *string.offset(i as isize) as libc::c_int == '\u{7f}' as i32 {
                i += 1;
                continue;
            }
            prev_i = i;
            if locale_mb_cur_max > 1 as libc::c_int {
                let mut state_bak: mbstate_t = mbstate_t {
                    __count: 0,
                    __value: mbstate_t_value { __wch: 0 },
                };
                let mut mblength: size_t = 0;
                let mut _f: libc::c_int = 0;
                _f = is_basic(*string.offset(i as isize));
                if _f != 0 {
                    mblength = 1 as libc::c_int as size_t;
                } else if locale_utf8locale != 0
                    && *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                        == 0 as libc::c_int
                {
                    mblength = (*string.offset(i as isize) as libc::c_int != 0 as libc::c_int)
                        as libc::c_int as size_t;
                } else {
                    state_bak = state;
                    mblength = mbrlen(
                        string.offset(i as isize),
                        slen.wrapping_sub(i as libc::c_ulong),
                        &mut state,
                    );
                }
                if mblength == -(2 as libc::c_int) as size_t
                    || mblength == -(1 as libc::c_int) as size_t
                {
                    state = state_bak;
                    i += 1;
                } else if mblength == 0 as libc::c_int as libc::c_ulong {
                    i += 1;
                } else {
                    i = (i as libc::c_ulong).wrapping_add(mblength) as libc::c_int as libc::c_int;
                }
            } else {
                i += 1;
            }
            if j < prev_i {
                loop {
                    let fresh51 = prev_i;
                    prev_i = prev_i + 1;
                    let fresh52 = j;
                    j = j + 1;
                    *string.offset(fresh52 as isize) = *string.offset(fresh51 as isize);
                    if !(prev_i < i) {
                        break;
                    }
                }
            } else {
                j = i;
            }
        }
        *string.offset(j as isize) = '\u{0}' as i32 as libc::c_char;
    }
    return string;
}
#[no_mangle]
pub fn word_list_remove_quoted_nulls(mut list: *mut WORD_LIST) {
    let mut t: *mut WORD_LIST = 0 as *mut WORD_LIST;
    t = list;
    unsafe {
        while !t.is_null() {
            remove_quoted_nulls((*(*t).word).word);
            (*(*t).word).flags &= !((1 as libc::c_int) << 18 as libc::c_int);
            t = (*t).next;
        }
    }
}
fn remove_upattern(
    mut param: *mut libc::c_char,
    mut pattern: *mut libc::c_char,
    mut op: libc::c_int,
) -> *mut libc::c_char {
    let mut len: size_t = 0;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_char = 0;
    unsafe {
        len = if !param.is_null() && *param.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            if *param.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                if *param.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                    strlen(param) as u64
                } else {
                    2 as libc::c_int as libc::c_ulong
                }
            } else {
                1 as libc::c_int as libc::c_ulong
            }
        } else {
            0 as libc::c_int as libc::c_ulong
        };
        end = param.offset(len as isize);
        match op {
            1 => {
                p = end;
                while p >= param {
                    c = *p;
                    *p = '\u{0}' as i32 as libc::c_char;
                    if strmatch(
                        pattern,
                        param,
                        if extended_glob != 0 {
                            (1 as libc::c_int) << 5 as libc::c_int
                        } else {
                            0 as libc::c_int
                        },
                    ) != 1 as libc::c_int
                    {
                        *p = c;
                        return strcpy(
                            sh_xmalloc(
                                (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(p) as u64),
                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                4575 as libc::c_int,
                            ) as *mut libc::c_char,
                            p,
                        );
                    }
                    *p = c;
                    p = p.offset(-1);
                }
            }
            2 => {
                p = param;
                while p <= end {
                    c = *p;
                    *p = '\u{0}' as i32 as libc::c_char;
                    if strmatch(
                        pattern,
                        param,
                        if extended_glob != 0 {
                            (1 as libc::c_int) << 5 as libc::c_int
                        } else {
                            0 as libc::c_int
                        },
                    ) != 1 as libc::c_int
                    {
                        *p = c;
                        return strcpy(
                            sh_xmalloc(
                                (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(p) as u64),
                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                4589 as libc::c_int,
                            ) as *mut libc::c_char,
                            p,
                        );
                    }
                    *p = c;
                    p = p.offset(1);
                }
            }
            3 => {
                p = param;
                while p <= end {
                    if strmatch(
                        pattern,
                        p,
                        if extended_glob != 0 {
                            (1 as libc::c_int) << 5 as libc::c_int
                        } else {
                            0 as libc::c_int
                        },
                    ) != 1 as libc::c_int
                    {
                        c = *p;
                        *p = '\u{0}' as i32 as libc::c_char;
                        ret = strcpy(
                            sh_xmalloc(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(strlen(param) as u64),
                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                4601 as libc::c_int,
                            ) as *mut libc::c_char,
                            param,
                        );
                        *p = c;
                        return ret;
                    }
                    p = p.offset(1);
                }
            }
            4 => {
                p = end;
                while p >= param {
                    if strmatch(
                        pattern,
                        p,
                        if extended_glob != 0 {
                            (1 as libc::c_int) << 5 as libc::c_int
                        } else {
                            0 as libc::c_int
                        },
                    ) != 1 as libc::c_int
                    {
                        c = *p;
                        *p = '\u{0}' as i32 as libc::c_char;
                        ret = strcpy(
                            sh_xmalloc(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(strlen(param) as u64),
                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                4614 as libc::c_int,
                            ) as *mut libc::c_char,
                            param,
                        );
                        *p = c;
                        return ret;
                    }
                    p = p.offset(-1);
                }
            }
            _ => {}
        }
    }
    return param;
}
fn remove_wpattern(
    mut wparam: *mut wchar_t,
    mut wstrlen: size_t,
    mut wpattern: *mut wchar_t,
    mut op: libc::c_int,
) -> *mut wchar_t {
    let mut wc: wchar_t = 0;
    let mut ret: *mut wchar_t = 0 as *mut wchar_t;
    let mut n: libc::c_int = 0;
    unsafe {
        match op {
            1 => {
                n = wstrlen as libc::c_int;
                while n >= 0 as libc::c_int {
                    wc = *wparam.offset(n as isize);
                    *wparam.offset(n as isize) = '\u{0}' as i32;
                    if wcsmatch(
                        wpattern,
                        wparam,
                        if extended_glob != 0 {
                            (1 as libc::c_int) << 5 as libc::c_int
                        } else {
                            0 as libc::c_int
                        },
                    ) != 1 as libc::c_int
                    {
                        *wparam.offset(n as isize) = wc;
                        return wcsdup(wparam.offset(n as isize));
                    }
                    *wparam.offset(n as isize) = wc;
                    n -= 1;
                }
            }
            2 => {
                n = 0 as libc::c_int;
                while n as libc::c_ulong <= wstrlen {
                    wc = *wparam.offset(n as isize);
                    *wparam.offset(n as isize) = '\u{0}' as i32;
                    if wcsmatch(
                        wpattern,
                        wparam,
                        if extended_glob != 0 {
                            (1 as libc::c_int) << 5 as libc::c_int
                        } else {
                            0 as libc::c_int
                        },
                    ) != 1 as libc::c_int
                    {
                        *wparam.offset(n as isize) = wc;
                        return wcsdup(wparam.offset(n as isize));
                    }
                    *wparam.offset(n as isize) = wc;
                    n += 1;
                }
            }
            3 => {
                n = 0 as libc::c_int;
                while n as libc::c_ulong <= wstrlen {
                    if wcsmatch(
                        wpattern,
                        wparam.offset(n as isize),
                        if extended_glob != 0 {
                            (1 as libc::c_int) << 5 as libc::c_int
                        } else {
                            0 as libc::c_int
                        },
                    ) != 1 as libc::c_int
                    {
                        wc = *wparam.offset(n as isize);
                        *wparam.offset(n as isize) = '\u{0}' as i32;
                        ret = wcsdup(wparam);
                        *wparam.offset(n as isize) = wc;
                        return ret;
                    }
                    n += 1;
                }
            }
            4 => {
                n = wstrlen as libc::c_int;
                while n >= 0 as libc::c_int {
                    if wcsmatch(
                        wpattern,
                        wparam.offset(n as isize),
                        if extended_glob != 0 {
                            (1 as libc::c_int) << 5 as libc::c_int
                        } else {
                            0 as libc::c_int
                        },
                    ) != 1 as libc::c_int
                    {
                        wc = *wparam.offset(n as isize);
                        *wparam.offset(n as isize) = '\u{0}' as i32;
                        ret = wcsdup(wparam);
                        *wparam.offset(n as isize) = wc;
                        return ret;
                    }
                    n -= 1;
                }
            }
            _ => {}
        }
    }
    return wparam;
}
fn remove_pattern(
    mut param: *mut libc::c_char,
    mut pattern: *mut libc::c_char,
    mut op: libc::c_int,
) -> *mut libc::c_char {
    unsafe {
        let mut xret: *mut libc::c_char = 0 as *mut libc::c_char;
        if param.is_null() {
            return param;
        }
        if *param as libc::c_int == '\u{0}' as i32
            || pattern.is_null()
            || *pattern as libc::c_int == '\u{0}' as i32
        {
            return strcpy(
                sh_xmalloc(
                    (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(param) as u64),
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    4706 as libc::c_int,
                ) as *mut libc::c_char,
                param,
            );
        }
        if __ctype_get_mb_cur_max() > 1 as libc::c_int as usize {
            let mut ret: *mut wchar_t = 0 as *mut wchar_t;
            let mut oret: *mut wchar_t = 0 as *mut wchar_t;
            let mut n: size_t = 0;
            let mut wparam: *mut wchar_t = 0 as *mut wchar_t;
            let mut wpattern: *mut wchar_t = 0 as *mut wchar_t;
            let mut ps: mbstate_t = mbstate_t {
                __count: 0,
                __value: mbstate_t_value { __wch: 0 },
            };
            n = xdupmbstowcs(&mut wpattern, 0 as *mut *mut *mut libc::c_char, pattern);
            if n == -(1 as libc::c_int) as size_t {
                xret = remove_upattern(param, pattern, op);
                return if xret == param {
                    strcpy(
                        sh_xmalloc(
                            (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(param) as u64),
                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                            4723 as libc::c_int,
                        ) as *mut libc::c_char,
                        param,
                    )
                } else {
                    xret
                };
            }
            n = xdupmbstowcs(&mut wparam, 0 as *mut *mut *mut libc::c_char, param);
            if n == -(1 as libc::c_int) as size_t {
                sh_xfree(
                    wpattern as *mut libc::c_void,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    4729 as libc::c_int,
                );
                xret = remove_upattern(param, pattern, op);
                return if xret == param {
                    strcpy(
                        sh_xmalloc(
                            (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(param) as u64),
                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                            4731 as libc::c_int,
                        ) as *mut libc::c_char,
                        param,
                    )
                } else {
                    xret
                };
            }
            ret = remove_wpattern(wparam, n, wpattern, op);
            oret = ret;
            if ret == wparam {
                sh_xfree(
                    wparam as *mut libc::c_void,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    4738 as libc::c_int,
                );
                sh_xfree(
                    wpattern as *mut libc::c_void,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    4739 as libc::c_int,
                );
                return strcpy(
                    sh_xmalloc(
                        (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(param) as u64),
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        4740 as libc::c_int,
                    ) as *mut libc::c_char,
                    param,
                );
            }
            sh_xfree(
                wparam as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                4743 as libc::c_int,
            );
            sh_xfree(
                wpattern as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                4744 as libc::c_int,
            );
            n = strlen(param) as u64;
            xret = sh_xmalloc(
                n.wrapping_add(1 as libc::c_int as libc::c_ulong),
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                4747 as libc::c_int,
            ) as *mut libc::c_char;
            memset(
                &mut ps as *mut mbstate_t as *mut libc::c_void,
                '\u{0}' as i32,
                ::std::mem::size_of::<mbstate_t>() as usize,
            );
            n = wcsrtombs(
                xret,
                &mut ret as *mut *mut wchar_t as *mut *const wchar_t,
                n as usize,
                &mut ps,
            ) as u64;
            *xret.offset(n as isize) = '\u{0}' as i32 as libc::c_char;
            sh_xfree(
                oret as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                4751 as libc::c_int,
            );
            return xret;
        } else {
            xret = remove_upattern(param, pattern, op);
            return if xret == param {
                strcpy(
                    sh_xmalloc(
                        (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(param) as u64),
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        4758 as libc::c_int,
                    ) as *mut libc::c_char,
                    param,
                )
            } else {
                xret
            };
        };
    }
}
fn match_upattern(
    mut string: *mut libc::c_char,
    mut pat: *mut libc::c_char,
    mut mtype: libc::c_int,
    mut sp: *mut *mut libc::c_char,
    mut ep: *mut *mut libc::c_char,
) -> libc::c_int {
    unsafe {
        let mut c: libc::c_int = 0;
        let mut mlen: libc::c_int = 0;
        let mut len: size_t = 0;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p1: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut npat: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
        len = if !pat.is_null() && *pat.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            if *pat.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                if *pat.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                    strlen(pat) as u64
                } else {
                    2 as libc::c_int as libc::c_ulong
                }
            } else {
                1 as libc::c_int as libc::c_ulong
            }
        } else {
            0 as libc::c_int as libc::c_ulong
        };
        if *pat.offset(0 as libc::c_int as isize) as libc::c_int != '*' as i32
            || *pat.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32
                && *pat.offset(1 as libc::c_int as isize) as libc::c_int == '(' as i32
                && extended_glob != 0
            || *pat.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                != '*' as i32
        {
            let mut unescaped_backslash: libc::c_int = 0;
            let mut pp: *mut libc::c_char = 0 as *mut libc::c_char;
            npat = sh_xmalloc(
                len.wrapping_add(3 as libc::c_int as libc::c_ulong),
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                4793 as libc::c_int,
            ) as *mut libc::c_char;
            p = npat;
            p1 = pat;
            if mtype != 0x1 as libc::c_int
                && (*p1 as libc::c_int != '*' as i32
                    || *p1 as libc::c_int == '*' as i32
                        && *p1.offset(1 as libc::c_int as isize) as libc::c_int == '(' as i32
                        && extended_glob != 0)
            {
                let fresh53 = p;
                p = p.offset(1);
                *fresh53 = '*' as i32 as libc::c_char;
            }
            while *p1 != 0 {
                let fresh54 = p1;
                p1 = p1.offset(1);
                let fresh55 = p;
                p = p.offset(1);
                *fresh55 = *fresh54;
            }
            if mtype != 0x2 as libc::c_int
                && (*p1.offset(-(1 as libc::c_int) as isize) as libc::c_int == '*' as i32 && {
                    unescaped_backslash = (*p1.offset(-(2 as libc::c_int) as isize) as libc::c_int
                        == '\\' as i32) as libc::c_int;
                    unescaped_backslash != 0
                })
            {
                pp = p1.offset(-(3 as libc::c_int as isize));
                while pp >= pat && {
                    let fresh56 = pp;
                    pp = pp.offset(-1);
                    *fresh56 as libc::c_int == '\\' as i32
                } {
                    unescaped_backslash = 1 as libc::c_int - unescaped_backslash;
                }
                if unescaped_backslash != 0 {
                    let fresh57 = p;
                    p = p.offset(1);
                    *fresh57 = '*' as i32 as libc::c_char;
                }
            } else if mtype != 0x2 as libc::c_int
                && *p1.offset(-(1 as libc::c_int) as isize) as libc::c_int != '*' as i32
            {
                let fresh58 = p;
                p = p.offset(1);
                *fresh58 = '*' as i32 as libc::c_char;
            }
            *p = '\u{0}' as i32 as libc::c_char;
        } else {
            npat = pat;
        }
        c = strmatch(
            npat,
            string,
            (if extended_glob != 0 {
                (1 as libc::c_int) << 5 as libc::c_int
            } else {
                0 as libc::c_int
            }) | (if match_ignore_case != 0 {
                (1 as libc::c_int) << 4 as libc::c_int
            } else {
                0 as libc::c_int
            }),
        );
        if npat != pat {
            sh_xfree(
                npat as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                4826 as libc::c_int,
            );
        }
        if c == 1 as libc::c_int {
            return 0 as libc::c_int;
        }
        len = if !string.is_null() && *string.offset(0 as libc::c_int as isize) as libc::c_int != 0
        {
            if *string.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                if *string.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                    strlen(string) as u64
                } else {
                    2 as libc::c_int as libc::c_ulong
                }
            } else {
                1 as libc::c_int as libc::c_ulong
            }
        } else {
            0 as libc::c_int as libc::c_ulong
        };
        end = string.offset(len as isize);
        mlen = umatchlen(pat, len);
        if mlen > len as libc::c_int {
            return 0 as libc::c_int;
        }
        match mtype {
            0 => {
                p = string;
                while p <= end {
                    if match_pattern_char(
                        pat,
                        p,
                        if match_ignore_case != 0 {
                            (1 as libc::c_int) << 4 as libc::c_int
                        } else {
                            0 as libc::c_int
                        },
                    ) != 0
                    {
                        p1 = if mlen == -(1 as libc::c_int) {
                            end
                        } else {
                            p.offset(mlen as isize)
                        };
                        if p1 > end {
                            break;
                        }
                        while p1 >= p {
                            c = *p1 as libc::c_int;
                            *p1 = '\u{0}' as i32 as libc::c_char;
                            if strmatch(
                                pat,
                                p,
                                (if extended_glob != 0 {
                                    (1 as libc::c_int) << 5 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) | (if match_ignore_case != 0 {
                                    (1 as libc::c_int) << 4 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }),
                            ) == 0 as libc::c_int
                            {
                                *p1 = c as libc::c_char;
                                *sp = p;
                                *ep = p1;
                                return 1 as libc::c_int;
                            }
                            *p1 = c as libc::c_char;
                            if mlen != -(1 as libc::c_int) {
                                break;
                            }
                            p1 = p1.offset(-1);
                        }
                    }
                    p = p.offset(1);
                }
                return 0 as libc::c_int;
            }
            1 => {
                if match_pattern_char(
                    pat,
                    string,
                    if match_ignore_case != 0 {
                        (1 as libc::c_int) << 4 as libc::c_int
                    } else {
                        0 as libc::c_int
                    },
                ) == 0 as libc::c_int
                {
                    return 0 as libc::c_int;
                }
                p = if mlen == -(1 as libc::c_int) {
                    end
                } else {
                    string.offset(mlen as isize)
                };
                while p >= string {
                    c = *p as libc::c_int;
                    *p = '\u{0}' as i32 as libc::c_char;
                    if strmatch(
                        pat,
                        string,
                        (if extended_glob != 0 {
                            (1 as libc::c_int) << 5 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) | (if match_ignore_case != 0 {
                            (1 as libc::c_int) << 4 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }),
                    ) == 0 as libc::c_int
                    {
                        *p = c as libc::c_char;
                        *sp = string;
                        *ep = p;
                        return 1 as libc::c_int;
                    }
                    *p = c as libc::c_char;
                    if mlen != -(1 as libc::c_int) {
                        break;
                    }
                    p = p.offset(-1);
                }
                return 0 as libc::c_int;
            }
            2 => {
                p = end.offset(
                    -((if mlen == -(1 as libc::c_int) {
                        len
                    } else {
                        mlen as libc::c_ulong
                    }) as isize),
                );
                while p <= end {
                    if strmatch(
                        pat,
                        p,
                        (if extended_glob != 0 {
                            (1 as libc::c_int) << 5 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) | (if match_ignore_case != 0 {
                            (1 as libc::c_int) << 4 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }),
                    ) == 0 as libc::c_int
                    {
                        *sp = p;
                        *ep = end;
                        return 1 as libc::c_int;
                    }
                    if mlen != -(1 as libc::c_int) {
                        break;
                    }
                    p = p.offset(1);
                }
                return 0 as libc::c_int;
            }
            _ => {}
        }
    }
    return 0 as libc::c_int;
}
fn match_wpattern(
    mut wstring: *mut wchar_t,
    mut indices: *mut *mut libc::c_char,
    mut wstrlen: size_t,
    mut wpat: *mut wchar_t,
    mut mtype: libc::c_int,
    mut sp: *mut *mut libc::c_char,
    mut ep: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut wc: wchar_t = 0;
    let mut wp: *mut wchar_t = 0 as *mut wchar_t;
    let mut nwpat: *mut wchar_t = 0 as *mut wchar_t;
    let mut wp1: *mut wchar_t = 0 as *mut wchar_t;
    let mut len: size_t = 0;
    let mut mlen: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut n1: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    let mut simple: libc::c_int = 0;
    unsafe {
        simple = (*wpat.offset(0 as libc::c_int as isize) != '\\' as i32
            && *wpat.offset(0 as libc::c_int as isize) != '*' as i32
            && *wpat.offset(0 as libc::c_int as isize) != '?' as i32
            && *wpat.offset(0 as libc::c_int as isize) != '[' as i32)
            as libc::c_int;
        if extended_glob != 0 {
            simple &= (*wpat.offset(1 as libc::c_int as isize) != '(' as i32
                || *wpat.offset(0 as libc::c_int as isize) != '*' as i32
                    && *wpat.offset(0 as libc::c_int as isize) != '?' as i32
                    && *wpat.offset(0 as libc::c_int as isize) != '+' as i32
                    && *wpat.offset(0 as libc::c_int as isize) != '!' as i32
                    && *wpat.offset(0 as libc::c_int as isize) != '@' as i32)
                as libc::c_int;
        }
        len = wcslen(wpat as *const wchar_t) as u64;
        if *wpat.offset(0 as libc::c_int as isize) != '*' as i32
            || *wpat.offset(0 as libc::c_int as isize) == '*' as i32
                && *wpat.offset(1 as libc::c_int as isize) == '(' as i32
                && extended_glob != 0
            || *wpat.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                != '*' as i32
        {
            let mut unescaped_backslash: libc::c_int = 0;
            let mut wpp: *mut wchar_t = 0 as *mut wchar_t;
            nwpat = sh_xmalloc(
                len.wrapping_add(3 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<wchar_t>() as libc::c_ulong),
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                4958 as libc::c_int,
            ) as *mut wchar_t;
            wp = nwpat;
            wp1 = wpat;
            if *wp1 != '*' as i32
                || *wp1 == '*' as i32
                    && *wp1.offset(1 as libc::c_int as isize) == '(' as i32
                    && extended_glob != 0
            {
                let fresh59 = wp;
                wp = wp.offset(1);
                *fresh59 = '*' as i32;
            }
            while *wp1 != '\u{0}' as i32 {
                let fresh60 = wp1;
                wp1 = wp1.offset(1);
                let fresh61 = wp;
                wp = wp.offset(1);
                *fresh61 = *fresh60;
            }
            if *wp1.offset(-(1 as libc::c_int) as isize) == '*' as i32 && {
                unescaped_backslash =
                    (*wp1.offset(-(2 as libc::c_int) as isize) == '\\' as i32) as libc::c_int;
                unescaped_backslash != 0
            } {
                wpp = wp1.offset(-(3 as libc::c_int as isize));
                while wpp >= wpat && {
                    let fresh62 = wpp;
                    wpp = wpp.offset(-1);
                    *fresh62 == '\\' as i32
                } {
                    unescaped_backslash = 1 as libc::c_int - unescaped_backslash;
                }
                if unescaped_backslash != 0 {
                    let fresh63 = wp;
                    wp = wp.offset(1);
                    *fresh63 = '*' as i32;
                }
            } else if *wp1.offset(-(1 as libc::c_int) as isize) != '*' as i32 {
                let fresh64 = wp;
                wp = wp.offset(1);
                *fresh64 = '*' as i32;
            }
            *wp = '\u{0}' as i32;
        } else {
            nwpat = wpat;
        }
        len = wcsmatch(
            nwpat,
            wstring,
            (if extended_glob != 0 {
                (1 as libc::c_int) << 5 as libc::c_int
            } else {
                0 as libc::c_int
            }) | (if match_ignore_case != 0 {
                (1 as libc::c_int) << 4 as libc::c_int
            } else {
                0 as libc::c_int
            }),
        ) as size_t;
        if nwpat != wpat {
            sh_xfree(
                nwpat as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                4986 as libc::c_int,
            );
        }
        if len == 1 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int;
        }
        mlen = wmatchlen(wpat, wstrlen);
        if mlen > wstrlen as libc::c_int {
            return 0 as libc::c_int;
        }
        match mtype {
            0 => {
                n = 0 as libc::c_int;
                while n as libc::c_ulong <= wstrlen {
                    n2 = if simple != 0 {
                        ((if match_ignore_case != 0 && iswupper(*wpat as wint_t) != 0 {
                            towlower(*wpat as wint_t)
                        } else {
                            *wpat as libc::c_uint
                        }) == (if match_ignore_case != 0
                            && iswupper(*wstring.offset(n as isize) as wint_t) != 0
                        {
                            towlower(*wstring.offset(n as isize) as wint_t)
                        } else {
                            *wstring.offset(n as isize) as libc::c_uint
                        })) as libc::c_int
                    } else {
                        match_pattern_wchar(
                            wpat,
                            wstring.offset(n as isize),
                            if match_ignore_case != 0 {
                                (1 as libc::c_int) << 4 as libc::c_int
                            } else {
                                0 as libc::c_int
                            },
                        )
                    };
                    if n2 != 0 {
                        n1 = (if mlen == -(1 as libc::c_int) {
                            wstrlen
                        } else {
                            (n + mlen) as libc::c_ulong
                        }) as libc::c_int;
                        if n1 as libc::c_ulong > wstrlen {
                            break;
                        }
                        while n1 >= n {
                            wc = *wstring.offset(n1 as isize);
                            *wstring.offset(n1 as isize) = '\u{0}' as i32;
                            if wcsmatch(
                                wpat,
                                wstring.offset(n as isize),
                                (if extended_glob != 0 {
                                    (1 as libc::c_int) << 5 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) | (if match_ignore_case != 0 {
                                    (1 as libc::c_int) << 4 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }),
                            ) == 0 as libc::c_int
                            {
                                *wstring.offset(n1 as isize) = wc;
                                *sp = *indices.offset(n as isize);
                                *ep = *indices.offset(n1 as isize);
                                return 1 as libc::c_int;
                            }
                            *wstring.offset(n1 as isize) = wc;
                            if mlen != -(1 as libc::c_int) {
                                break;
                            }
                            n1 -= 1;
                        }
                    }
                    n += 1;
                }
                return 0 as libc::c_int;
            }
            1 => {
                if match_pattern_wchar(
                    wpat,
                    wstring,
                    if match_ignore_case != 0 {
                        (1 as libc::c_int) << 4 as libc::c_int
                    } else {
                        0 as libc::c_int
                    },
                ) == 0 as libc::c_int
                {
                    return 0 as libc::c_int;
                }
                n = (if mlen == -(1 as libc::c_int) {
                    wstrlen
                } else {
                    mlen as libc::c_ulong
                }) as libc::c_int;
                while n >= 0 as libc::c_int {
                    wc = *wstring.offset(n as isize);
                    *wstring.offset(n as isize) = '\u{0}' as i32;
                    if wcsmatch(
                        wpat,
                        wstring,
                        (if extended_glob != 0 {
                            (1 as libc::c_int) << 5 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) | (if match_ignore_case != 0 {
                            (1 as libc::c_int) << 4 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }),
                    ) == 0 as libc::c_int
                    {
                        *wstring.offset(n as isize) = wc;
                        *sp = *indices.offset(0 as libc::c_int as isize);
                        *ep = *indices.offset(n as isize);
                        return 1 as libc::c_int;
                    }
                    *wstring.offset(n as isize) = wc;
                    if mlen != -(1 as libc::c_int) {
                        break;
                    }
                    n -= 1;
                }
                return 0 as libc::c_int;
            }
            2 => {
                n = wstrlen.wrapping_sub(if mlen == -(1 as libc::c_int) {
                    wstrlen
                } else {
                    mlen as libc::c_ulong
                }) as libc::c_int;
                while n as libc::c_ulong <= wstrlen {
                    if wcsmatch(
                        wpat,
                        wstring.offset(n as isize),
                        (if extended_glob != 0 {
                            (1 as libc::c_int) << 5 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) | (if match_ignore_case != 0 {
                            (1 as libc::c_int) << 4 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }),
                    ) == 0 as libc::c_int
                    {
                        *sp = *indices.offset(n as isize);
                        *ep = *indices.offset(wstrlen as isize);
                        return 1 as libc::c_int;
                    }
                    if mlen != -(1 as libc::c_int) {
                        break;
                    }
                    n += 1;
                }
                return 0 as libc::c_int;
            }
            _ => {}
        }
    }
    return 0 as libc::c_int;
}
fn match_pattern(
    mut string: *mut libc::c_char,
    mut pat: *mut libc::c_char,
    mut mtype: libc::c_int,
    mut sp: *mut *mut libc::c_char,
    mut ep: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut n: size_t = 0;
    let mut wstring: *mut wchar_t = 0 as *mut wchar_t;
    let mut wpat: *mut wchar_t = 0 as *mut wchar_t;
    let mut indices: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    unsafe {
        if string.is_null() || pat.is_null() || *pat as libc::c_int == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        if __ctype_get_mb_cur_max() > 1 as libc::c_int as usize {
            if (mbsmbchar(string)).is_null() && (mbsmbchar(pat)).is_null() {
                return match_upattern(string, pat, mtype, sp, ep);
            }
            n = xdupmbstowcs(&mut wpat, 0 as *mut *mut *mut libc::c_char, pat);
            if n == -(1 as libc::c_int) as size_t {
                return match_upattern(string, pat, mtype, sp, ep);
            }
            n = xdupmbstowcs(&mut wstring, &mut indices, string);
            if n == -(1 as libc::c_int) as size_t {
                sh_xfree(
                    wpat as *mut libc::c_void,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    5099 as libc::c_int,
                );
                return match_upattern(string, pat, mtype, sp, ep);
            }
            ret = match_wpattern(wstring, indices, n, wpat, mtype, sp, ep);
            sh_xfree(
                wpat as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                5104 as libc::c_int,
            );
            sh_xfree(
                wstring as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                5105 as libc::c_int,
            );
            sh_xfree(
                indices as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                5106 as libc::c_int,
            );
            return ret;
        } else {
            return match_upattern(string, pat, mtype, sp, ep);
        };
    }
}
fn getpatspec(mut c: libc::c_int, mut value: *mut libc::c_char) -> libc::c_int {
    unsafe {
        if c == '#' as i32 {
            return if *value as libc::c_int == '#' as i32 {
                1 as libc::c_int
            } else {
                2 as libc::c_int
            };
        } else {
            return if *value as libc::c_int == '%' as i32 {
                3 as libc::c_int
            } else {
                4 as libc::c_int
            };
        };
    }
}
fn getpattern(
    mut value: *mut libc::c_char,
    mut quoted: libc::c_int,
    mut expandpat: libc::c_int,
) -> *mut libc::c_char {
    let mut pat: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tword: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: *mut WORD_LIST = 0 as *mut WORD_LIST;
    unsafe {
        l = if *value as libc::c_int != 0 {
            expand_string_for_pat(
                value,
                if quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0 {
                    0x8 as libc::c_int
                } else {
                    quoted
                },
                0 as *mut libc::c_void as *mut libc::c_int,
                0 as *mut libc::c_void as *mut libc::c_int,
            )
        } else {
            0 as *mut WORD_LIST
        };
        if !l.is_null() {
            word_list_remove_quoted_nulls(l);
        }
        pat = string_list(l);
        dispose_words(l);
        if !pat.is_null() {
            tword = quote_string_for_globbing(pat, 0x1 as libc::c_int);
            sh_xfree(
                pat as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                5172 as libc::c_int,
            );
            pat = tword;
        }
    }
    return pat;
}
fn list_remove_pattern(
    mut list: *mut WORD_LIST,
    mut pattern: *mut libc::c_char,
    mut patspec: libc::c_int,
    mut itype: libc::c_int,
    mut quoted: libc::c_int,
) -> *mut libc::c_char {
    let mut new: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut l: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut w: *mut WORD_DESC = 0 as *mut WORD_DESC;
    let mut tword: *mut libc::c_char = 0 as *mut libc::c_char;
    new = 0 as *mut libc::c_void as *mut WORD_LIST;
    l = list;
    unsafe {
        while !l.is_null() {
            tword = remove_pattern((*(*l).word).word, pattern, patspec);
            w = alloc_word_desc();
            let ref mut fresh65 = (*w).word;
            *fresh65 = if !tword.is_null() {
                tword
            } else {
                strcpy(
                    sh_xmalloc(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_add(strlen(b"\0" as *const u8 as *const libc::c_char) as u64),
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        5208 as libc::c_int,
                    ) as *mut libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                )
            };
            new = make_word_list(w, new);
            l = (*l).next;
        }
        l = if !new.is_null() && !((*new).next).is_null() {
            list_reverse(new as *mut GENERIC_LIST) as *mut WORD_LIST
        } else {
            new
        };
        tword = string_list_pos_params(itype, l, quoted, 0 as libc::c_int);
        dispose_words(l);
    }
    return tword;
}
fn parameter_list_remove_pattern(
    mut itype: libc::c_int,
    mut pattern: *mut libc::c_char,
    mut patspec: libc::c_int,
    mut quoted: libc::c_int,
) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut list: *mut WORD_LIST = 0 as *mut WORD_LIST;
    list = list_rest_of_args();
    if list.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    ret = list_remove_pattern(list, pattern, patspec, itype, quoted);
    dispose_words(list);
    return ret;
}
fn array_remove_pattern(
    mut var: *mut SHELL_VAR,
    mut pattern: *mut libc::c_char,
    mut patspec: libc::c_int,
    mut starsub: libc::c_int,
    mut quoted: libc::c_int,
) -> *mut libc::c_char {
    let mut a: *mut ARRAY = 0 as *mut ARRAY;
    let mut h: *mut HASH_TABLE = 0 as *mut HASH_TABLE;
    let mut itype: libc::c_int = 0;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut list: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    v = var;
    unsafe {
        itype = if starsub != 0 { '*' as i32 } else { '@' as i32 };
        a = if !v.is_null() && (*v).attributes & 0x4 as libc::c_int != 0 {
            (*v).value as *mut ARRAY
        } else {
            0 as *mut ARRAY
        };
        h = if !v.is_null() && (*v).attributes & 0x40 as libc::c_int != 0 {
            (*v).value as *mut HASH_TABLE
        } else {
            0 as *mut HASH_TABLE
        };
        list = if !a.is_null() {
            array_to_word_list(a)
        } else if !h.is_null() {
            assoc_to_word_list(h)
        } else {
            0 as *mut WORD_LIST
        };
        if list.is_null() {
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
        ret = list_remove_pattern(list, pattern, patspec, itype, quoted);
        dispose_words(list);
    }
    return ret;
}
fn parameter_brace_remove_pattern(
    mut varname: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut ind: libc::c_int,
    mut patstr: *mut libc::c_char,
    mut rtype: libc::c_int,
    mut quoted: libc::c_int,
    mut flags: libc::c_int,
) -> *mut libc::c_char {
    let mut vtype: libc::c_int = 0;
    let mut patspec: libc::c_int = 0;
    let mut starsub: libc::c_int = 0;
    let mut temp1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pattern: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut oname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    if value.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    unsafe {
        oname = this_command_name;
        this_command_name = varname;
        vtype = get_var_and_type(
            varname,
            value,
            ind as arrayind_t,
            quoted,
            flags,
            &mut v,
            &mut val,
        );
        if vtype == -(1 as libc::c_int) {
            this_command_name = oname;
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
        starsub = vtype & 128 as libc::c_int;
        vtype &= !(128 as libc::c_int);
        patspec = getpatspec(rtype, patstr);
        if patspec == 1 as libc::c_int || patspec == 3 as libc::c_int {
            patstr = patstr.offset(1);
        }
        temp1 = strcpy(
            sh_xmalloc(
                (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(patstr) as u64),
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                5302 as libc::c_int,
            ) as *mut libc::c_char,
            patstr,
        );
        pattern = getpattern(temp1, quoted, 1 as libc::c_int);
        sh_xfree(
            temp1 as *mut libc::c_void,
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            5304 as libc::c_int,
        );
        temp1 = 0 as *mut libc::c_void as *mut libc::c_char;
        match vtype {
            0 | 3 => {
                temp1 = remove_pattern(val, pattern, patspec);
                if vtype == 0 as libc::c_int {
                    if !val.is_null() {
                        sh_xfree(
                            val as *mut libc::c_void,
                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                            5313 as libc::c_int,
                        );
                    }
                }
                if !temp1.is_null() {
                    val = if quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0 {
                        quote_string(temp1)
                    } else {
                        quote_escapes(temp1)
                    };
                    sh_xfree(
                        temp1 as *mut libc::c_void,
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        5319 as libc::c_int,
                    );
                    temp1 = val;
                }
            }
            2 => {
                temp1 = array_remove_pattern(v, pattern, patspec, starsub, quoted);
                if !temp1.is_null()
                    && quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) == 0 as libc::c_int
                {
                    val = quote_escapes(temp1);
                    sh_xfree(
                        temp1 as *mut libc::c_void,
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        5329 as libc::c_int,
                    );
                    temp1 = val;
                }
            }
            1 => {
                temp1 = parameter_list_remove_pattern(
                    *varname.offset(0 as libc::c_int as isize) as libc::c_int,
                    pattern,
                    patspec,
                    quoted,
                );
                if !(!temp1.is_null() && quoted == 0 as libc::c_int && ifs_is_null != 0) {
                    if !temp1.is_null()
                        && quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) == 0 as libc::c_int
                    {
                        val = quote_escapes(temp1);
                        sh_xfree(
                            temp1 as *mut libc::c_void,
                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                            5343 as libc::c_int,
                        );
                        temp1 = val;
                    }
                }
            }
            _ => {}
        }
        this_command_name = oname;
        if !pattern.is_null() {
            sh_xfree(
                pattern as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                5351 as libc::c_int,
            );
        }
    }
    return temp1;
}
static mut dev_fd_list: *mut pid_t = 0 as *const libc::c_void as *mut libc::c_void as *mut pid_t;
static mut nfds: libc::c_int = 0;
static mut totfds: libc::c_int = 0;
#[no_mangle]
pub fn clear_fifo(mut i: libc::c_int) {
    unsafe {
        if *dev_fd_list.offset(i as isize) != 0 {
            *dev_fd_list.offset(i as isize) = 0 as libc::c_int;
            nfds -= 1;
        }
    }
}
#[no_mangle]
pub fn clear_fifo_list() {
    unsafe {
        let mut i: libc::c_int = 0;
        if nfds == 0 as libc::c_int {
            return;
        }
        i = 0 as libc::c_int;
        while nfds != 0 && i < totfds {
            clear_fifo(i);
            i += 1;
        }
        nfds = 0 as libc::c_int;
    }
}
#[no_mangle]
pub fn copy_fifo_list(mut sizep: *mut libc::c_int) -> *mut libc::c_void {
    unsafe {
        let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
        if nfds == 0 as libc::c_int || totfds == 0 as libc::c_int {
            if !sizep.is_null() {
                *sizep = 0 as libc::c_int;
            }
            return 0 as *mut libc::c_void;
        }
        if !sizep.is_null() {
            *sizep = totfds;
        }
        ret = sh_xmalloc(
            (totfds as libc::c_ulong).wrapping_mul(::std::mem::size_of::<pid_t>() as libc::c_ulong),
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            5676 as libc::c_int,
        );
        return memcpy(
            ret,
            dev_fd_list as *const libc::c_void,
            (totfds as libc::c_ulong).wrapping_mul(::std::mem::size_of::<pid_t>() as libc::c_ulong)
                as usize,
        );
    }
}
fn add_fifo_list(mut fd: libc::c_int) {
    unsafe {
        if dev_fd_list.is_null() || fd >= totfds {
            let mut ofds: libc::c_int = 0;
            ofds = totfds;
            totfds = getdtablesize();
            if totfds < 0 as libc::c_int || totfds > 256 as libc::c_int {
                totfds = 256 as libc::c_int;
            }
            if fd >= totfds {
                totfds = fd + 2 as libc::c_int;
            }
            dev_fd_list = sh_xrealloc(
                dev_fd_list as *mut libc::c_void,
                (totfds as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<pid_t>() as libc::c_ulong),
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                5695 as libc::c_int,
            ) as *mut pid_t;
            memset(
                dev_fd_list.offset(ofds as isize) as *mut libc::c_void,
                '\u{0}' as i32,
                ((totfds - ofds) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<pid_t>() as libc::c_ulong)
                    as usize,
            );
        }
        *dev_fd_list.offset(fd as isize) = 1 as libc::c_int;
        nfds += 1;
    }
}
#[no_mangle]
pub fn fifos_pending() -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub fn num_fifos() -> libc::c_int {
    unsafe {
        return nfds;
    }
}
#[no_mangle]
pub fn unlink_fifo(mut fd: libc::c_int) {
    unsafe {
        if *dev_fd_list.offset(fd as isize) != 0 {
            close(fd);
            *dev_fd_list.offset(fd as isize) = 0 as libc::c_int;
            nfds -= 1;
        }
    }
}
#[no_mangle]
pub fn unlink_fifo_list() {
    unsafe {
        let mut i: libc::c_int = 0;
        if nfds == 0 as libc::c_int {
            return;
        }
        i = totfds - 1 as libc::c_int;
        while nfds != 0 && i >= 0 as libc::c_int {
            unlink_fifo(i);
            i -= 1;
        }
        nfds = 0 as libc::c_int;
    }
}
#[no_mangle]
pub fn unlink_all_fifos() {
    unlink_fifo_list();
}
#[no_mangle]
pub fn close_new_fifos(mut list: *mut libc::c_void, mut lsize: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut plist: *mut pid_t = 0 as *mut pid_t;
    if list.is_null() {
        unlink_fifo_list();
        return;
    }
    plist = list as *mut pid_t;
    i = 0 as libc::c_int;
    unsafe {
        while i < lsize {
            if *plist.offset(i as isize) == 0 as libc::c_int
                && i < totfds
                && *dev_fd_list.offset(i as isize) != 0
            {
                unlink_fifo(i);
            }
            i += 1;
        }
        i = lsize;
        while i < totfds {
            unlink_fifo(i);
            i += 1;
        }
    }
}
#[no_mangle]
pub fn find_procsub_child(mut pid: pid_t) -> libc::c_int {
    unsafe {
        let mut i: libc::c_int = 0;
        if nfds == 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        i = 0 as libc::c_int;
        while i < totfds {
            if *dev_fd_list.offset(i as isize) == pid {
                return i;
            }
            i += 1;
        }
        return -(1 as libc::c_int);
    }
}
#[no_mangle]
pub fn set_procsub_status(mut ind: libc::c_int, mut pid: pid_t, mut status: libc::c_int) {
    unsafe {
        if ind >= 0 as libc::c_int && ind < totfds {
            *dev_fd_list.offset(ind as isize) = -(1 as libc::c_int);
        }
    }
}
fn reap_some_procsubs(mut max: libc::c_int) {
    unsafe {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while nfds > 0 as libc::c_int && i < max {
            if *dev_fd_list.offset(i as isize) == -(1 as libc::c_int) {
                unlink_fifo(i);
            }
            i += 1;
        }
    }
}
#[no_mangle]
pub fn reap_procsubs() {
    unsafe {
        reap_some_procsubs(totfds);
    }
}
fn make_dev_fd_filename(mut fd: libc::c_int) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut intbuf: [libc::c_char; 12] = [0; 12];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    ret = sh_xmalloc(
        (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong),
        b"../subst.c\0" as *const u8 as *const libc::c_char,
        5862 as libc::c_int,
    ) as *mut libc::c_char;
    unsafe {
        strcpy(ret, b"/dev/fd/\0" as *const u8 as *const libc::c_char);
        p = inttostr(
            fd as intmax_t,
            intbuf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
        );
        strcpy(
            ret.offset(::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as isize)
                .offset(-(1 as libc::c_int as isize)),
            p,
        );
        add_fifo_list(fd);
    }
    return ret;
}
fn process_substitute(
    mut string: *mut libc::c_char,
    mut open_for_read_in_child: libc::c_int,
) -> *mut libc::c_char {
    let mut pathname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fd: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut function_value: libc::c_int = 0;
    let mut old_pid: pid_t = 0;
    let mut pid: pid_t = 0;
    let mut parent_pipe_fd: libc::c_int = 0;
    let mut child_pipe_fd: libc::c_int = 0;
    let mut fildes: [libc::c_int; 2] = [0; 2];
    let mut old_pipeline_pgrp: pid_t = 0;
    unsafe {
        if string.is_null() || *string == 0 || wordexp_only != 0 {
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
        if pipe(fildes.as_mut_ptr()) < 0 as libc::c_int {
            sys_error(
                b"%s\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot make pipe for process substitution\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
        parent_pipe_fd = fildes[open_for_read_in_child as usize];
        child_pipe_fd = fildes[(1 as libc::c_int - open_for_read_in_child) as usize];
        parent_pipe_fd = move_to_high_fd(parent_pipe_fd, 1 as libc::c_int, 64 as libc::c_int);
        pathname = make_dev_fd_filename(parent_pipe_fd);
        if pathname.is_null() {
            sys_error(
                b"%s\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot make pipe for process substitution\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
        old_pid = last_made_pid;
        old_pipeline_pgrp = pipeline_pgrp;
        if pipeline_pgrp == 0 as libc::c_int
            || subshell_environment
                & (0x10 as libc::c_int | 0x8 as libc::c_int | 0x1 as libc::c_int)
                == 0 as libc::c_int
        {
            pipeline_pgrp = shell_pgrp;
        }
        save_pipeline(1 as libc::c_int);
        pid = make_child(
            0 as *mut libc::c_void as *mut libc::c_char,
            1 as libc::c_int,
        );
        if pid == 0 as libc::c_int {
            interactive = 0 as libc::c_int;
            reset_terminating_signals();
            free_pushed_string_input();
            restore_original_signals();
            if terminating_signal != 0 {
                termsig_handler(terminating_signal);
            }
            if interrupt_state != 0 {
                throw_to_top_level();
            }
            setup_async_signals();
            subshell_environment |= 0x4 as libc::c_int | 0x20 as libc::c_int | 0x1 as libc::c_int;
            change_flag('v' as i32, '+' as i32);
            if expanding_redir != 0 {
                flush_temporary_env();
            }
        }
        set_sigchld_handler();
        stop_making_children();
        pipeline_pgrp = old_pipeline_pgrp;
        if pid < 0 as libc::c_int {
            sys_error(
                b"%s\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot make child for process substitution\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            sh_xfree(
                pathname as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                5986 as libc::c_int,
            );
            close(parent_pipe_fd);
            close(child_pipe_fd);
            restore_pipeline(1 as libc::c_int);
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
        if pid > 0 as libc::c_int {
            last_procsub_child = restore_pipeline(0 as libc::c_int);
            let ref mut fresh66 = (*last_procsub_child).next;
            *fresh66 = 0 as *mut process;
            procsub_add(last_procsub_child);
            *dev_fd_list.offset(parent_pipe_fd as isize) = pid;
            ::std::ptr::write_volatile(&mut last_made_pid as *mut pid_t, old_pid);
            close_pgrp_pipe();
            close(child_pipe_fd);
            return pathname;
        }
        set_sigint_handler();
        set_job_control(0 as libc::c_int);
        procsub_clear();
        if pipeline_pgrp != shell_pgrp {
            pipeline_pgrp = getpid();
        }
        fd = child_pipe_fd;
        if open_for_read_in_child == 0 as libc::c_int {
            fpurge(stdout);
        }
        if dup2(
            fd,
            if open_for_read_in_child != 0 {
                0 as libc::c_int
            } else {
                1 as libc::c_int
            },
        ) < 0 as libc::c_int
        {
            sys_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot duplicate named pipe %s as fd %d\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                pathname,
                if open_for_read_in_child != 0 {
                    0 as libc::c_int
                } else {
                    1 as libc::c_int
                },
            );
            exit(127 as libc::c_int);
        }
        if fd
            != (if open_for_read_in_child != 0 {
                0 as libc::c_int
            } else {
                1 as libc::c_int
            })
        {
            close(fd);
        }
        if !current_fds_to_close.is_null() {
            close_fd_bitmap(current_fds_to_close);
            current_fds_to_close = 0 as *mut libc::c_void as *mut fd_bitmap;
        }
        close(parent_pipe_fd);
        *dev_fd_list.offset(parent_pipe_fd as isize) = 0 as libc::c_int;
        expanding_redir = 0 as libc::c_int;
        remove_quoted_escapes(string);
        result = __sigsetjmp(top_level.as_mut_ptr(), 0 as libc::c_int);
        if result == 0 as libc::c_int && return_catch_flag != 0 {
            function_value = __sigsetjmp(return_catch.as_mut_ptr(), 0 as libc::c_int);
        } else {
            function_value = 0 as libc::c_int;
        }
        if result == 4 as libc::c_int {
            rc = last_command_exit_value;
        } else if result == 3 as libc::c_int {
            rc = last_command_exit_value;
        } else if result != 0 {
            rc = 1 as libc::c_int;
        } else if function_value != 0 {
            rc = return_catch_value;
        } else {
            subshell_level += 1;
            rc = parse_and_execute(
                string,
                b"process substitution\0" as *const u8 as *const libc::c_char,
                0x1 as libc::c_int | 0x4 as libc::c_int,
            );
        }
        ::std::ptr::write_volatile(&mut last_command_exit_value as *mut libc::c_int, rc);
        rc = run_exit_trap();
        exit(rc);
    }
}
fn read_comsub(
    mut fd: libc::c_int,
    mut quoted: libc::c_int,
    mut flags: libc::c_int,
    mut rflag: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut istring: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 512] = [0; 512];
    let mut bufp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut istring_index: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut tflag: libc::c_int = 0;
    let mut skip_ctlesc: libc::c_int = 0;
    let mut skip_ctlnul: libc::c_int = 0;
    let mut mb_cur_max: libc::c_int = 0;
    let mut istring_size: size_t = 0;
    let mut bufn: ssize_t = 0;
    let mut nullbyte: libc::c_int = 0;
    let mut ps: mbstate_t = mbstate_t {
        __count: 0,
        __value: mbstate_t_value { __wch: 0 },
    };
    let mut wc: wchar_t = 0;
    let mut mblen_0: size_t = 0;
    let mut i: libc::c_int = 0;
    istring = 0 as *mut libc::c_void as *mut libc::c_char;
    tflag = 0 as libc::c_int;
    bufn = tflag as ssize_t;
    istring_size = bufn as size_t;
    unsafe {
        istring_index = istring_size as libc::c_int;
        skip_ctlesc = ifs_cmap['\u{1}' as i32 as usize] as libc::c_int;
        skip_ctlnul = ifs_cmap['\u{7f}' as i32 as usize] as libc::c_int;
        mb_cur_max = __ctype_get_mb_cur_max() as libc::c_int;
        nullbyte = 0 as libc::c_int;
        while !(fd < 0 as libc::c_int) {
            bufn -= 1;
            if bufn <= 0 as libc::c_int as libc::c_long {
                bufn = zread(
                    fd,
                    buf.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
                ) as i64;
                if bufn <= 0 as libc::c_int as libc::c_long {
                    break;
                }
                bufp = buf.as_mut_ptr();
            }
            let fresh67 = bufp;
            bufp = bufp.offset(1);
            c = *fresh67 as libc::c_int;
            if c == 0 as libc::c_int {
                if nullbyte == 0 as libc::c_int {
                    internal_warning(
                        b"%s\0" as *const u8 as *const libc::c_char,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"command substitution: ignored null byte in input\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    nullbyte = 1 as libc::c_int;
                }
            } else {
                if (istring_index + (mb_cur_max + 1 as libc::c_int)) as libc::c_ulong
                    >= istring_size
                {
                    while (istring_index + (mb_cur_max + 1 as libc::c_int)) as libc::c_ulong
                        >= istring_size
                    {
                        istring_size = (istring_size as libc::c_ulong)
                            .wrapping_add(512 as libc::c_int as libc::c_ulong)
                            as size_t as size_t;
                    }
                    istring = sh_xrealloc(
                        istring as *mut libc::c_void,
                        istring_size,
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        6221 as libc::c_int,
                    ) as *mut libc::c_char;
                }
                if quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0 {
                    let fresh68 = istring_index;
                    istring_index = istring_index + 1;
                    *istring.offset(fresh68 as isize) = '\u{1}' as i32 as libc::c_char;
                } else if flags & 0x8 as libc::c_int != 0 && skip_ctlesc != 0 && c == '\u{1}' as i32
                {
                    let fresh69 = istring_index;
                    istring_index = istring_index + 1;
                    *istring.offset(fresh69 as isize) = '\u{1}' as i32 as libc::c_char;
                } else if skip_ctlesc == 0 as libc::c_int && c == '\u{1}' as i32 {
                    let fresh70 = istring_index;
                    istring_index = istring_index + 1;
                    *istring.offset(fresh70 as isize) = '\u{1}' as i32 as libc::c_char;
                } else if skip_ctlnul == 0 as libc::c_int && c == '\u{7f}' as i32
                    || c == ' ' as i32
                        && (!ifs_value.is_null() && *ifs_value as libc::c_int == 0 as libc::c_int)
                {
                    let fresh71 = istring_index;
                    istring_index = istring_index + 1;
                    *istring.offset(fresh71 as isize) = '\u{1}' as i32 as libc::c_char;
                }
                if locale_utf8locale != 0 && c & 0x80 as libc::c_int != 0
                    || locale_utf8locale == 0 as libc::c_int
                        && mb_cur_max > 1 as libc::c_int
                        && c as libc::c_uchar as libc::c_int > 127 as libc::c_int
                {
                    memset(
                        &mut ps as *mut mbstate_t as *mut libc::c_void,
                        '\u{0}' as i32,
                        ::std::mem::size_of::<mbstate_t>() as usize,
                    );
                    mblen_0 = mbrtowc(
                        &mut wc,
                        bufp.offset(-(1 as libc::c_int as isize)),
                        (bufn + 1 as libc::c_int as libc::c_long) as size_t,
                        &mut ps,
                    );
                    if mblen_0 == -(1 as libc::c_int) as size_t
                        || mblen_0 == -(2 as libc::c_int) as size_t
                        || mblen_0 == 0 as libc::c_int as libc::c_ulong
                        || mblen_0 == 1 as libc::c_int as libc::c_ulong
                    {
                        let fresh72 = istring_index;
                        istring_index = istring_index + 1;
                        *istring.offset(fresh72 as isize) = c as libc::c_char;
                    } else {
                        let fresh73 = istring_index;
                        istring_index = istring_index + 1;
                        *istring.offset(fresh73 as isize) = c as libc::c_char;
                        i = 0 as libc::c_int;
                        while (i as libc::c_ulong)
                            < mblen_0.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        {
                            let fresh74 = bufp;
                            bufp = bufp.offset(1);
                            let fresh75 = istring_index;
                            istring_index = istring_index + 1;
                            *istring.offset(fresh75 as isize) = *fresh74;
                            i += 1;
                        }
                        bufn = (bufn as libc::c_ulong)
                            .wrapping_sub(mblen_0.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                            as ssize_t as ssize_t;
                    }
                } else {
                    let fresh76 = istring_index;
                    istring_index = istring_index + 1;
                    *istring.offset(fresh76 as isize) = c as libc::c_char;
                }
            }
        }
        if !istring.is_null() {
            *istring.offset(istring_index as isize) = '\u{0}' as i32 as libc::c_char;
        }
        if istring_index == 0 as libc::c_int {
            if !istring.is_null() {
                sh_xfree(
                    istring as *mut libc::c_void,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    6267 as libc::c_int,
                );
            }
            if !rflag.is_null() {
                *rflag = tflag;
            }
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
        if quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0 {
            while istring_index > 0 as libc::c_int {
                if !(*istring.offset((istring_index - 1 as libc::c_int) as isize) as libc::c_int
                    == '\n' as i32)
                {
                    break;
                }
                istring_index -= 1;
                if *istring.offset((istring_index - 1 as libc::c_int) as isize) as libc::c_int
                    == '\u{1}' as i32
                {
                    istring_index -= 1;
                }
            }
            *istring.offset(istring_index as isize) = '\u{0}' as i32 as libc::c_char;
        } else {
            strip_trailing(istring, istring_index - 1 as libc::c_int, 1 as libc::c_int);
        }
        if !rflag.is_null() {
            *rflag = tflag;
        }
    }
    return istring;
}
#[no_mangle]
pub fn command_substitute(
    mut string: *mut libc::c_char,
    mut quoted: libc::c_int,
    mut flags: libc::c_int,
) -> *mut WORD_DESC {
    let mut pid: pid_t = 0;
    let mut old_pid: pid_t = 0;
    let mut old_pipeline_pgrp: pid_t = 0;
    let mut old_async_pid: pid_t = 0;
    let mut istring: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result: libc::c_int = 0;
    let mut fildes: [libc::c_int; 2] = [0; 2];
    let mut function_value: libc::c_int = 0;
    let mut pflags: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut tflag: libc::c_int = 0;
    let mut fork_flags: libc::c_int = 0;
    let mut ret: *mut WORD_DESC = 0 as *mut WORD_DESC;
    let mut set: sigset_t = sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = sigset_t { __val: [0; 16] };
    istring = 0 as *mut libc::c_void as *mut libc::c_char;
    s = string;
    unsafe {
        while !s.is_null()
            && *s as libc::c_int != 0
            && (*sh_syntaxtab
                .as_mut_ptr()
                .offset(*s as libc::c_uchar as isize)
                & 0x2000 as libc::c_int
                != 0
                || *s as libc::c_int == '\n' as i32)
        {
            s = s.offset(1);
        }
        if s.is_null() || *s as libc::c_int == 0 as libc::c_int {
            return 0 as *mut libc::c_void as *mut WORD_DESC;
        }
        if wordexp_only != 0 && read_but_dont_execute != 0 {
            ::std::ptr::write_volatile(
                &mut last_command_exit_value as *mut libc::c_int,
                125 as libc::c_int,
            );
            jump_to_top_level(3 as libc::c_int);
        }
        if subst_assign_varlist.is_null() || garglist.is_null() {
            maybe_make_export_env();
        }
        pflags = if interactive != 0 && sourcelevel == 0 as libc::c_int {
            0x10 as libc::c_int
        } else {
            0 as libc::c_int
        };
        old_pid = last_made_pid;
        if pipe(fildes.as_mut_ptr()) < 0 as libc::c_int {
            sys_error(
                b"%s\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot make pipe for command substitution\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        } else {
            old_pipeline_pgrp = pipeline_pgrp;
            if subshell_environment & (0x8 as libc::c_int | 0x10 as libc::c_int) == 0 as libc::c_int
            {
                pipeline_pgrp = shell_pgrp;
            }
            cleanup_the_pipeline();
            old_async_pid = last_asynchronous_pid;
            fork_flags = if subshell_environment & 0x1 as libc::c_int != 0 {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            };
            pid = make_child(
                0 as *mut libc::c_void as *mut libc::c_char,
                fork_flags | 4 as libc::c_int,
            );
            ::std::ptr::write_volatile(&mut last_asynchronous_pid as *mut pid_t, old_async_pid);
            if pid == 0 as libc::c_int {
                reset_signal_handlers();
                if interrupt_state != 0 as libc::c_int {
                    kill(getpid(), 2 as libc::c_int);
                    ::std::ptr::write_volatile(
                        &mut interrupt_state as *mut sig_atomic_t,
                        0 as libc::c_int,
                    );
                }
                if terminating_signal != 0 {
                    termsig_handler(terminating_signal);
                }
                if interrupt_state != 0 {
                    throw_to_top_level();
                }
                subshell_environment |= 0x80 as libc::c_int;
            }
            set_sigchld_handler();
            stop_making_children();
            if pid != 0 as libc::c_int {
                pipeline_pgrp = old_pipeline_pgrp;
            }
            if pid < 0 as libc::c_int {
                sys_error(dcgettext(
                    0 as *const libc::c_char,
                    b"cannot make child for command substitution\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ));
            } else if pid == 0 as libc::c_int {
                interactive = 0 as libc::c_int;
                set_sigint_handler();
                free_pushed_string_input();
                fpurge(stdout);
                if dup2(fildes[1 as libc::c_int as usize], 1 as libc::c_int) < 0 as libc::c_int {
                    sys_error(
                        b"%s\0" as *const u8 as *const libc::c_char,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"command_substitute: cannot duplicate pipe as fd 1\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    exit(1 as libc::c_int);
                }
                if fildes[1 as libc::c_int as usize] != fileno(stdin)
                    && fildes[1 as libc::c_int as usize] != fileno(stdout)
                    && fildes[1 as libc::c_int as usize] != fileno(stderr)
                {
                    close(fildes[1 as libc::c_int as usize]);
                }
                if fildes[0 as libc::c_int as usize] != fileno(stdin)
                    && fildes[0 as libc::c_int as usize] != fileno(stdout)
                    && fildes[0 as libc::c_int as usize] != fileno(stderr)
                {
                    close(fildes[0 as libc::c_int as usize]);
                }
                subshell_environment |= 0x4 as libc::c_int;
                change_flag('v' as i32, '+' as i32);
                if inherit_errexit == 0 as libc::c_int {
                    builtin_ignoring_errexit = 0 as libc::c_int;
                    change_flag('e' as i32, '+' as i32);
                }
                set_shellopts();
                if expanding_redir != 0 {
                    flush_temporary_env();
                    expanding_redir = 0 as libc::c_int;
                }
                remove_quoted_escapes(string);
                startup_state = 2 as libc::c_int;
                parse_and_execute_level = 0 as libc::c_int;
                result = __sigsetjmp(top_level.as_mut_ptr(), 0 as libc::c_int);
                if result == 0 as libc::c_int && return_catch_flag != 0 {
                    function_value = __sigsetjmp(return_catch.as_mut_ptr(), 0 as libc::c_int);
                } else {
                    function_value = 0 as libc::c_int;
                }
                if result == 4 as libc::c_int {
                    rc = last_command_exit_value;
                } else if result == 3 as libc::c_int {
                    rc = last_command_exit_value;
                } else if result != 0 {
                    rc = 1 as libc::c_int;
                } else if function_value != 0 {
                    rc = return_catch_value;
                } else {
                    subshell_level += 1;
                    rc = parse_and_execute(
                        string,
                        b"command substitution\0" as *const u8 as *const libc::c_char,
                        pflags | 0x4 as libc::c_int,
                    );
                }
                ::std::ptr::write_volatile(&mut last_command_exit_value as *mut libc::c_int, rc);
                rc = run_exit_trap();
                unlink_fifo_list();
                exit(rc);
            } else {
                let mut dummyfd: libc::c_int = 0;
                close_pgrp_pipe();
                close(fildes[1 as libc::c_int as usize]);
                begin_unwind_frame(
                    b"read-comsub\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                dummyfd = fildes[0 as libc::c_int as usize];
                add_unwind_protect(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(libc::c_int) -> libc::c_int,
                        Option<Function>,
                    >(close),
                    dummyfd as *mut libc::c_char,
                );
                // add_unwind_protect(
                //                 Some(close as fn() -> libc::c_int),
                //                 dummyfd as &mut i8,
                //             );
                sigemptyset(&mut set);
                sigaddset(&mut set, 2 as libc::c_int);
                sigemptyset(&mut oset);
                sigprocmask(0 as libc::c_int, &mut set, &mut oset);
                tflag = 0 as libc::c_int;
                istring = read_comsub(fildes[0 as libc::c_int as usize], quoted, flags, &mut tflag);
                close(fildes[0 as libc::c_int as usize]);
                discard_unwind_frame(
                    b"read-comsub\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                sigprocmask(
                    2 as libc::c_int,
                    &mut oset,
                    0 as *mut libc::c_void as *mut crate::src_common::__sigset_t,
                );
                current_command_subst_pid = pid;
                ::std::ptr::write_volatile(
                    &mut last_command_exit_value as *mut libc::c_int,
                    wait_for(pid, (1 as libc::c_int) << 8 as libc::c_int),
                );
                last_command_subst_pid = pid;
                ::std::ptr::write_volatile(&mut last_made_pid as *mut pid_t, old_pid);
                if last_command_exit_value == 128 as libc::c_int + 2 as libc::c_int
                    && last_command_exit_signal == 2 as libc::c_int
                {
                    kill(getpid(), 2 as libc::c_int);
                }
                ret = alloc_word_desc();
                let ref mut fresh77 = (*ret).word;
                *fresh77 = istring;
                (*ret).flags = tflag;
                return ret;
            }
        }
        ::std::ptr::write_volatile(&mut last_made_pid as *mut pid_t, old_pid);
        if !istring.is_null() {
            sh_xfree(
                istring as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                6404 as libc::c_int,
            );
        }
        close(fildes[0 as libc::c_int as usize]);
        close(fildes[1 as libc::c_int as usize]);
    }
    return 0 as *mut libc::c_void as *mut WORD_DESC;
}
fn array_length_reference(mut s: *mut libc::c_char) -> arrayind_t {
    let mut len: libc::c_int = 0;
    let mut ind: arrayind_t = 0;
    let mut akey: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_char = 0;
    let mut array: *mut ARRAY = 0 as *mut ARRAY;
    let mut h: *mut HASH_TABLE = 0 as *mut HASH_TABLE;
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    var = array_variable_part(s, 0 as libc::c_int, &mut t, &mut len);
    unsafe {
        if (var.is_null()
            || (*var).attributes & 0x1000 as libc::c_int != 0
            || (*var).attributes & 0x40 as libc::c_int == 0 as libc::c_int
                && (*var).attributes & 0x4 as libc::c_int == 0 as libc::c_int)
            && unbound_vars_is_error != 0
        {
            t = t.offset(-1);
            c = *t;
            *t = '\u{0}' as i32 as libc::c_char;
            set_exit_status(1 as libc::c_int);
            err_unboundvar(s);
            *t = c;
            return -(1 as libc::c_int) as arrayind_t;
        } else {
            if var.is_null() || (*var).attributes & 0x1000 as libc::c_int != 0 {
                return 0 as libc::c_int as arrayind_t;
            }
        }
        array = if (*var).attributes & 0x4 as libc::c_int != 0 {
            (*var).value as *mut ARRAY
        } else {
            0 as *mut libc::c_void as *mut ARRAY
        };
        h = if (*var).attributes & 0x40 as libc::c_int != 0 {
            (*var).value as *mut HASH_TABLE
        } else {
            0 as *mut libc::c_void as *mut HASH_TABLE
        };
        if (*t.offset(0 as libc::c_int as isize) as libc::c_int == '@' as i32
            || *t.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32)
            && *t.offset(1 as libc::c_int as isize) as libc::c_int == ']' as i32
        {
            if (*var).attributes & 0x40 as libc::c_int != 0 {
                return (if !h.is_null() {
                    (*h).nentries
                } else {
                    0 as libc::c_int
                }) as arrayind_t;
            } else if (*var).attributes & 0x4 as libc::c_int != 0 {
                return (if !array.is_null() {
                    (*array).num_elements
                } else {
                    0 as libc::c_int
                }) as arrayind_t;
            } else {
                return (if !((*var).value).is_null() {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as arrayind_t;
            }
        }
        if (*var).attributes & 0x40 as libc::c_int != 0 {
            *t.offset((len - 1 as libc::c_int) as isize) = '\u{0}' as i32 as libc::c_char;
            akey = expand_assignment_string_to_string(t, 0 as libc::c_int);
            *t.offset((len - 1 as libc::c_int) as isize) = ']' as i32 as libc::c_char;
            if akey.is_null() || *akey as libc::c_int == 0 as libc::c_int {
                err_badarraysub(t);
                if !akey.is_null() {
                    sh_xfree(
                        akey as *mut libc::c_void,
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        6625 as libc::c_int,
                    );
                }
                return -(1 as libc::c_int) as arrayind_t;
            }
            t = assoc_reference((*var).value as *mut HASH_TABLE, akey);
            sh_xfree(
                akey as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                6629 as libc::c_int,
            );
        } else {
            ind = array_expand_index(var, t, len, 0 as libc::c_int);
            if !var.is_null()
                && (*var).attributes & 0x4 as libc::c_int != 0
                && ind < 0 as libc::c_int as libc::c_long
            {
                ind = (*((*var).value as *mut ARRAY)).max_index
                    + 1 as libc::c_int as libc::c_long
                    + ind;
            }
            if ind < 0 as libc::c_int as libc::c_long {
                err_badarraysub(t);
                return -(1 as libc::c_int) as arrayind_t;
            }
            if (*var).attributes & 0x4 as libc::c_int != 0 {
                t = array_reference(array, ind);
            } else {
                t = if ind == 0 as libc::c_int as libc::c_long {
                    (*var).value
                } else {
                    0 as *mut libc::c_void as *mut libc::c_char
                };
            }
        }
        len = (if __ctype_get_mb_cur_max() > 1 as libc::c_int as usize {
            if !t.is_null() && *t.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
                if *t.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                    mbstrlen(t) as usize
                } else {
                    1 as libc::c_int as usize
                }
            } else {
                0 as libc::c_int as usize
            }
        } else if !t.is_null() && *t.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            if *t.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                if *t.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                    strlen(t) as usize
                } else {
                    2 as libc::c_int as usize
                }
            } else {
                1 as libc::c_int as usize
            }
        } else {
            0 as libc::c_int as usize
        }) as libc::c_int;
    }
    return len as arrayind_t;
}
fn valid_brace_expansion_word(
    mut name: *mut libc::c_char,
    mut var_is_special: libc::c_int,
) -> libc::c_int {
    unsafe {
        if *name as libc::c_int >= '0' as i32
            && *name as libc::c_int <= '9' as i32
            && all_digits(name) != 0
        {
            return 1 as libc::c_int;
        } else if var_is_special != 0 {
            return 1 as libc::c_int;
        } else if valid_array_reference(name, 0 as libc::c_int) != 0 {
            return 1 as libc::c_int;
        } else if legal_identifier(name) != 0 {
            return 1 as libc::c_int;
        } else {
            return 0 as libc::c_int;
        };
    }
}
fn chk_atstar(
    mut name: *mut libc::c_char,
    mut quoted: libc::c_int,
    mut pflags: libc::c_int,
    mut quoted_dollar_atp: *mut libc::c_int,
    mut contains_dollar_at: *mut libc::c_int,
) -> libc::c_int {
    let mut temp1: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        if name.is_null() {
            if !quoted_dollar_atp.is_null() {
                *quoted_dollar_atp = 0 as libc::c_int;
            }
            if !contains_dollar_at.is_null() {
                *contains_dollar_at = 0 as libc::c_int;
            }
            return 0 as libc::c_int;
        }
        if *name.offset(0 as libc::c_int as isize) as libc::c_int == '@' as i32
            && *name.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
        {
            if quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0
                && !quoted_dollar_atp.is_null()
            {
                *quoted_dollar_atp = 1 as libc::c_int;
            }
            if !contains_dollar_at.is_null() {
                *contains_dollar_at = 1 as libc::c_int;
            }
            return 1 as libc::c_int;
        } else {
            if *name.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32
                && *name.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
                && quoted == 0 as libc::c_int
            {
                if !contains_dollar_at.is_null() && expand_no_split_dollar_star == 0 as libc::c_int
                {
                    *contains_dollar_at = 1 as libc::c_int;
                }
                return 1 as libc::c_int;
            } else {
                if valid_array_reference(name, 0 as libc::c_int) != 0 {
                    temp1 = mbschr(name, '[' as i32);
                    if !temp1.is_null()
                        && *temp1.offset(1 as libc::c_int as isize) as libc::c_int == '@' as i32
                        && *temp1.offset(2 as libc::c_int as isize) as libc::c_int == ']' as i32
                    {
                        if quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0
                            && !quoted_dollar_atp.is_null()
                        {
                            *quoted_dollar_atp = 1 as libc::c_int;
                        }
                        if !contains_dollar_at.is_null() {
                            *contains_dollar_at = 1 as libc::c_int;
                        }
                        return 1 as libc::c_int;
                    }
                    if !temp1.is_null()
                        && *temp1.offset(1 as libc::c_int as isize) as libc::c_int == '*' as i32
                        && *temp1.offset(2 as libc::c_int as isize) as libc::c_int == ']' as i32
                        && quoted == 0 as libc::c_int
                    {
                        if !contains_dollar_at.is_null() {
                            *contains_dollar_at = 1 as libc::c_int;
                        }
                        return 1 as libc::c_int;
                    }
                }
            }
        }
    }
    return 0 as libc::c_int;
}
fn parameter_brace_expand_word(
    mut name: *mut libc::c_char,
    mut var_is_special: libc::c_int,
    mut quoted: libc::c_int,
    mut pflags: libc::c_int,
    mut indp: *mut arrayind_t,
) -> *mut WORD_DESC {
    let mut ret: *mut WORD_DESC = 0 as *mut WORD_DESC;
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut arg_index: intmax_t = 0;
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut atype: libc::c_int = 0;
    let mut rflags: libc::c_int = 0;
    let mut ind: arrayind_t = 0;
    ret = 0 as *mut WORD_DESC;
    temp = 0 as *mut libc::c_char;
    rflags = 0 as libc::c_int;
    unsafe {
        if !indp.is_null() {
            *indp = -(9223372036854775807 as libc::c_long) - 1 as libc::c_int as libc::c_long;
        }
        if legal_number(name, &mut arg_index) != 0 {
            tt = get_dollar_var_value(arg_index);
            if !tt.is_null() {
                temp = if *tt as libc::c_int != 0
                    && quoted & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0
                {
                    quote_string(tt)
                } else {
                    quote_escapes(tt)
                };
            } else {
                temp = 0 as *mut libc::c_void as *mut libc::c_char;
            }
            if !tt.is_null() {
                sh_xfree(
                    tt as *mut libc::c_void,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    6768 as libc::c_int,
                );
            }
        } else if var_is_special != 0 {
            let mut sindex: libc::c_int = 0;
            tt = sh_xmalloc(
                (2 as libc::c_int as libc::c_ulong).wrapping_add(strlen(name) as u64),
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                6773 as libc::c_int,
            ) as *mut libc::c_char;
            sindex = 0 as libc::c_int;
            *tt.offset(sindex as isize) = '$' as i32 as libc::c_char;
            strcpy(tt.offset(1 as libc::c_int as isize), name);
            ret = param_expand(
                tt,
                &mut sindex,
                quoted,
                0 as *mut libc::c_void as *mut libc::c_int,
                0 as *mut libc::c_void as *mut libc::c_int,
                0 as *mut libc::c_void as *mut libc::c_int,
                0 as *mut libc::c_void as *mut libc::c_int,
                pflags,
            );
            sh_xfree(
                tt as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                6779 as libc::c_int,
            );
        } else {
            let mut current_block_79: u64;
            if valid_array_reference(name, 0 as libc::c_int) != 0 {
                current_block_79 = 14887093560253451645;
            } else {
                var = find_variable(name);
                if !var.is_null() {
                    if !((*var).value).is_null()
                        && (*var).attributes & 0x1000 as libc::c_int == 0 as libc::c_int
                    {
                        tt = 0 as *mut libc::c_void as *mut libc::c_char;
                        if pflags & 0x40 as libc::c_int != 0
                            && (*var).attributes & 0x40 as libc::c_int != 0
                        {
                            temp = if (*((*var).value as *mut HASH_TABLE)).nentries
                                == 0 as libc::c_int
                            {
                                0 as *mut libc::c_void as *mut libc::c_char
                            } else {
                                assoc_to_string(
                                    (*var).value as *mut HASH_TABLE,
                                    b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                                    quoted,
                                )
                            };
                            tt = temp;
                        } else if pflags & 0x40 as libc::c_int != 0
                            && (*var).attributes & 0x4 as libc::c_int != 0
                        {
                            temp = if (*((*var).value as *mut ARRAY)).num_elements
                                == 0 as libc::c_int
                            {
                                0 as *mut libc::c_void as *mut libc::c_char
                            } else {
                                array_to_string(
                                    (*var).value as *mut ARRAY,
                                    b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                                    quoted,
                                )
                            };
                            tt = temp;
                        } else if (*var).attributes & 0x40 as libc::c_int != 0 {
                            temp = assoc_reference(
                                (*var).value as *mut HASH_TABLE,
                                b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            );
                        } else if (*var).attributes & 0x4 as libc::c_int != 0 {
                            temp = array_reference(
                                (*var).value as *mut ARRAY,
                                0 as libc::c_int as arrayind_t,
                            );
                        } else {
                            temp = (*var).value;
                        }
                        if !temp.is_null() {
                            temp = if *temp as libc::c_int != 0
                                && quoted & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0
                            {
                                quote_string(temp)
                            } else if pflags & 0x8 as libc::c_int != 0 {
                                quote_rhs(temp)
                            } else {
                                quote_escapes(temp)
                            };
                        }
                        if !tt.is_null() {
                            sh_xfree(
                                tt as *mut libc::c_void,
                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                6864 as libc::c_int,
                            );
                        }
                    } else {
                        temp = 0 as *mut libc::c_void as *mut libc::c_char;
                    }
                    current_block_79 = 6002151390280567665;
                } else {
                    var = find_variable_last_nameref(name, 0 as libc::c_int);
                    if !var.is_null() {
                        temp = (*var).value;
                        if !temp.is_null()
                            && *temp as libc::c_int != 0
                            && valid_array_reference(temp, 0 as libc::c_int) != 0
                        {
                            name = temp;
                            current_block_79 = 14887093560253451645;
                        } else {
                            if !temp.is_null()
                                && *temp as libc::c_int != 0
                                && legal_identifier(temp) == 0 as libc::c_int
                            {
                                set_exit_status(1 as libc::c_int);
                                report_error(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"%s: invalid variable name for name reference\0"
                                            as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    temp,
                                );
                                temp = &mut expand_param_error;
                            } else {
                                temp = 0 as *mut libc::c_void as *mut libc::c_char;
                            }
                            current_block_79 = 6002151390280567665;
                        }
                    } else {
                        temp = 0 as *mut libc::c_void as *mut libc::c_char;
                        current_block_79 = 6002151390280567665;
                    }
                }
            }
            match current_block_79 {
                14887093560253451645 => {
                    var =
                        array_variable_part(name, 0 as libc::c_int, &mut tt, 0 as *mut libc::c_int);
                    if pflags & 0x8 as libc::c_int != 0 {
                        if (*tt.offset(0 as libc::c_int as isize) as libc::c_int == '@' as i32
                            || *tt.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32)
                            && *tt.offset(1 as libc::c_int as isize) as libc::c_int == ']' as i32
                        {
                            if !var.is_null()
                                && ((*var).attributes & 0x4 as libc::c_int != 0
                                    || (*var).attributes & 0x40 as libc::c_int != 0)
                            {
                                temp = array_value(
                                    name,
                                    quoted | 0x1 as libc::c_int,
                                    0x10 as libc::c_int,
                                    &mut atype,
                                    &mut ind,
                                );
                            } else {
                                temp = array_value(
                                    name,
                                    quoted,
                                    0 as libc::c_int,
                                    &mut atype,
                                    &mut ind,
                                );
                            }
                        } else {
                            temp =
                                array_value(name, quoted, 0 as libc::c_int, &mut atype, &mut ind);
                        }
                    } else if pflags & 0x4 as libc::c_int != 0 {
                        if *tt.offset(0 as libc::c_int as isize) as libc::c_int == '@' as i32
                            && *tt.offset(1 as libc::c_int as isize) as libc::c_int == ']' as i32
                            && !var.is_null()
                            && quoted == 0 as libc::c_int
                            && ifs_is_set != 0
                            && ifs_is_null == 0 as libc::c_int
                            && ifs_firstc[0 as libc::c_int as usize] as libc::c_int != ' ' as i32
                        {
                            temp = array_value(
                                name,
                                0x1 as libc::c_int,
                                0x10 as libc::c_int,
                                &mut atype,
                                &mut ind,
                            );
                        } else if *tt.offset(0 as libc::c_int as isize) as libc::c_int == '@' as i32
                            && *tt.offset(1 as libc::c_int as isize) as libc::c_int == ']' as i32
                        {
                            temp =
                                array_value(name, quoted, 0 as libc::c_int, &mut atype, &mut ind);
                        } else if *tt.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32
                            && *tt.offset(1 as libc::c_int as isize) as libc::c_int == ']' as i32
                            && expand_no_split_dollar_star != 0
                            && ifs_is_null != 0
                        {
                            temp = array_value(
                                name,
                                0x1 as libc::c_int | 0x2 as libc::c_int,
                                0 as libc::c_int,
                                &mut atype,
                                &mut ind,
                            );
                        } else if *tt.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32
                            && *tt.offset(1 as libc::c_int as isize) as libc::c_int == ']' as i32
                        {
                            temp =
                                array_value(name, quoted, 0 as libc::c_int, &mut atype, &mut ind);
                        } else {
                            temp =
                                array_value(name, quoted, 0 as libc::c_int, &mut atype, &mut ind);
                        }
                    } else if *tt.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32
                        && *tt.offset(1 as libc::c_int as isize) as libc::c_int == ']' as i32
                        && expand_no_split_dollar_star != 0
                        && ifs_is_null != 0
                    {
                        temp = array_value(
                            name,
                            0x1 as libc::c_int | 0x2 as libc::c_int,
                            0 as libc::c_int,
                            &mut atype,
                            &mut ind,
                        );
                    } else {
                        temp = array_value(name, quoted, 0 as libc::c_int, &mut atype, &mut ind);
                    }
                    if atype == 0 as libc::c_int && !temp.is_null() {
                        temp = if *temp as libc::c_int != 0
                            && quoted & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0
                        {
                            quote_string(temp)
                        } else {
                            quote_escapes(temp)
                        };
                        rflags |= (1 as libc::c_int) << 24 as libc::c_int;
                        if !indp.is_null() {
                            *indp = ind;
                        }
                    } else if atype == 1 as libc::c_int
                        && !temp.is_null()
                        && (*temp.offset(0 as libc::c_int as isize) as libc::c_int
                            == '\u{7f}' as i32
                            && *temp.offset(1 as libc::c_int as isize) as libc::c_int
                                == '\u{0}' as i32)
                        && quoted & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0
                    {
                        rflags |= (1 as libc::c_int) << 18 as libc::c_int;
                    }
                }
                _ => {}
            }
        }
        if ret.is_null() {
            ret = alloc_word_desc();
            let ref mut fresh78 = (*ret).word;
            *fresh78 = temp;
            (*ret).flags |= rflags;
        }
    }
    return ret;
}
fn parameter_brace_find_indir(
    mut name: *mut libc::c_char,
    mut var_is_special: libc::c_int,
    mut quoted: libc::c_int,
    mut find_nameref: libc::c_int,
) -> *mut libc::c_char {
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut w: *mut WORD_DESC = 0 as *mut WORD_DESC;
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut pflags: libc::c_int = 0;
    let mut oldex: libc::c_int = 0;
    unsafe {
        if find_nameref != 0
            && var_is_special == 0 as libc::c_int
            && {
                v = find_variable_last_nameref(name, 0 as libc::c_int);
                !v.is_null()
            }
            && (*v).attributes & 0x800 as libc::c_int != 0
            && {
                t = (*v).value;
                !t.is_null()
            }
            && *t as libc::c_int != 0
        {
            return strcpy(
                sh_xmalloc(
                    (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(t) as u64),
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    6915 as libc::c_int,
                ) as *mut libc::c_char,
                t,
            );
        }
        pflags = 0x2 as libc::c_int;
        if var_is_special != 0 {
            pflags |= 0x8 as libc::c_int;
            oldex = expand_no_split_dollar_star;
            expand_no_split_dollar_star = 1 as libc::c_int;
        }
        w = parameter_brace_expand_word(name, var_is_special, quoted, pflags, 0 as *mut arrayind_t);
        if var_is_special != 0 {
            expand_no_split_dollar_star = oldex;
        }
        t = (*w).word;
        if !t.is_null() {
            temp = if quoted & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0 || var_is_special != 0
            {
                dequote_string(t)
            } else {
                dequote_escapes(t)
            };
            sh_xfree(
                t as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                6939 as libc::c_int,
            );
            t = temp;
        }
        dispose_word_desc(w);
    }
    return t;
}
fn parameter_brace_expand_indir(
    mut name: *mut libc::c_char,
    mut var_is_special: libc::c_int,
    mut quoted: libc::c_int,
    mut pflags: libc::c_int,
    mut quoted_dollar_atp: *mut libc::c_int,
    mut contains_dollar_at: *mut libc::c_int,
) -> *mut WORD_DESC {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut w: *mut WORD_DESC = 0 as *mut WORD_DESC;
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    unsafe {
        if var_is_special == 0 as libc::c_int && {
            v = find_variable_last_nameref(name, 0 as libc::c_int);
            !v.is_null()
        } {
            if (*v).attributes & 0x800 as libc::c_int != 0
                && {
                    t = (*v).value;
                    !t.is_null()
                }
                && *t as libc::c_int != 0
            {
                w = alloc_word_desc();
                let ref mut fresh79 = (*w).word;
                *fresh79 = strcpy(
                    sh_xmalloc(
                        (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(t) as u64),
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        6969 as libc::c_int,
                    ) as *mut libc::c_char,
                    t,
                );
                (*w).flags = 0 as libc::c_int;
                return w;
            }
        }
        if legal_identifier(name) != 0 && v.is_null() {
            report_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: invalid indirect expansion\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                name,
            );
            w = alloc_word_desc();
            let ref mut fresh80 = (*w).word;
            *fresh80 = &mut expand_param_error;
            (*w).flags = 0 as libc::c_int;
            return w;
        }
        t = parameter_brace_find_indir(name, var_is_special, quoted, 0 as libc::c_int);
        chk_atstar(t, quoted, pflags, quoted_dollar_atp, contains_dollar_at);
        if t.is_null() && valid_array_reference(name, 0 as libc::c_int) != 0 {
            v = array_variable_part(
                name,
                0 as libc::c_int,
                0 as *mut *mut libc::c_char,
                0 as *mut libc::c_int,
            );
            if v.is_null() {
                report_error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: invalid indirect expansion\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    name,
                );
                w = alloc_word_desc();
                let ref mut fresh81 = (*w).word;
                *fresh81 = &mut expand_param_error;
                (*w).flags = 0 as libc::c_int;
                return w;
            } else {
                return 0 as *mut libc::c_void as *mut WORD_DESC;
            }
        }
        if t.is_null() {
            return 0 as *mut libc::c_void as *mut WORD_DESC;
        }
        if valid_brace_expansion_word(
            t,
            (*t as libc::c_int != 0
                && (*t as libc::c_int >= '0' as i32
                    && *t as libc::c_int <= '9' as i32
                    && all_digits(t) != 0
                    || *t.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
                        && *sh_syntaxtab
                            .as_mut_ptr()
                            .offset(*t as libc::c_uchar as isize)
                            & 0x800 as libc::c_int
                            != 0
                    || 0 as libc::c_int != 0
                        && *t.offset(2 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
                        && (posixly_correct == 0 as libc::c_int
                            && *t.offset(1 as libc::c_int as isize) as libc::c_int == '#' as i32
                            || posixly_correct == 0 as libc::c_int
                                && *t.offset(1 as libc::c_int as isize) as libc::c_int
                                    == '?' as i32
                            || *t.offset(1 as libc::c_int as isize) as libc::c_int == '@' as i32
                            || *t.offset(1 as libc::c_int as isize) as libc::c_int == '*' as i32)))
                as libc::c_int,
        ) == 0 as libc::c_int
        {
            report_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: invalid variable name\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                t,
            );
            sh_xfree(
                t as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                7016 as libc::c_int,
            );
            w = alloc_word_desc();
            let ref mut fresh82 = (*w).word;
            *fresh82 = &mut expand_param_error;
            (*w).flags = 0 as libc::c_int;
            return w;
        }
        w = parameter_brace_expand_word(
            t,
            (*t as libc::c_int != 0
                && (*t as libc::c_int >= '0' as i32
                    && *t as libc::c_int <= '9' as i32
                    && all_digits(t) != 0
                    || *t.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
                        && *sh_syntaxtab
                            .as_mut_ptr()
                            .offset(*t as libc::c_uchar as isize)
                            & 0x800 as libc::c_int
                            != 0
                    || 0 as libc::c_int != 0
                        && *t.offset(2 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
                        && (posixly_correct == 0 as libc::c_int
                            && *t.offset(1 as libc::c_int as isize) as libc::c_int == '#' as i32
                            || posixly_correct == 0 as libc::c_int
                                && *t.offset(1 as libc::c_int as isize) as libc::c_int
                                    == '?' as i32
                            || *t.offset(1 as libc::c_int as isize) as libc::c_int == '@' as i32
                            || *t.offset(1 as libc::c_int as isize) as libc::c_int == '*' as i32)))
                as libc::c_int,
            quoted,
            pflags,
            0 as *mut arrayind_t,
        );
        sh_xfree(
            t as *mut libc::c_void,
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            7024 as libc::c_int,
        );
    }
    return w;
}
fn parameter_brace_expand_rhs(
    mut name: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut op: libc::c_int,
    mut quoted: libc::c_int,
    mut pflags: libc::c_int,
    mut qdollaratp: *mut libc::c_int,
    mut hasdollarat: *mut libc::c_int,
) -> *mut WORD_DESC {
    let mut w: *mut WORD_DESC = 0 as *mut WORD_DESC;
    let mut l: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut tl: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l_hasdollat: libc::c_int = 0;
    let mut sindex: libc::c_int = 0;
    unsafe {
        let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
        if quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0 && *value as libc::c_int != 0 {
            sindex = 0 as libc::c_int;
            temp = string_extract_double_quoted(value, &mut sindex, 0x800 as libc::c_int);
        } else {
            temp = value;
        }
        w = alloc_word_desc();
        l_hasdollat = 0 as libc::c_int;
        l = if *temp as libc::c_int != 0 {
            expand_string_for_rhs(
                temp,
                quoted,
                op,
                pflags,
                &mut l_hasdollat,
                0 as *mut libc::c_void as *mut libc::c_int,
            )
        } else {
            0 as *mut WORD_LIST
        };
        if !hasdollarat.is_null() {
            *hasdollarat =
                (l_hasdollat != 0 || !l.is_null() && !((*l).next).is_null()) as libc::c_int;
        }
        if temp != value {
            sh_xfree(
                temp as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                7063 as libc::c_int,
            );
        }
        tl = l;
        while !tl.is_null() {
            if !((*tl).word).is_null()
                && (((*(*tl).word).word).is_null()
                    || *((*(*tl).word).word).offset(0 as libc::c_int as isize) as libc::c_int
                        == 0 as libc::c_int)
                && (*(*tl).word).flags | (1 as libc::c_int) << 21 as libc::c_int != 0
            {
                t = make_quoted_char('\u{0}' as i32);
                if !((*(*tl).word).word).is_null() {
                    sh_xfree(
                        (*(*tl).word).word as *mut libc::c_void,
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        7074 as libc::c_int,
                    );
                }
                let ref mut fresh83 = (*(*tl).word).word;
                *fresh83 = t;
                (*(*tl).word).flags |= (1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 18 as libc::c_int;
                (*(*tl).word).flags &= !((1 as libc::c_int) << 21 as libc::c_int);
            }
            tl = (*tl).next;
        }
        if !l.is_null() {
            if !qdollaratp.is_null() && (l_hasdollat != 0 && quoted != 0 || !((*l).next).is_null())
            {
                *qdollaratp = 1 as libc::c_int;
            }
            if !((*l).next).is_null() && ifs_is_null != 0 {
                temp = string_list_internal(
                    l,
                    b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                (*w).flags |= (1 as libc::c_int) << 3 as libc::c_int;
            } else if l_hasdollat != 0 || !((*l).next).is_null() {
                temp = string_list_dollar_star(l, quoted, 0 as libc::c_int);
            } else {
                temp = string_list(l);
                if !temp.is_null()
                    && (*temp.offset(0 as libc::c_int as isize) as libc::c_int == '\u{7f}' as i32
                        && *temp.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32)
                        as libc::c_int
                        == 0 as libc::c_int
                    && (*(*l).word).flags & (1 as libc::c_int) << 21 as libc::c_int != 0
                {
                    (*w).flags |= (1 as libc::c_int) << 21 as libc::c_int;
                }
            }
            if ((*l).next).is_null()
                && quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0
                && (*temp.offset(0 as libc::c_int as isize) as libc::c_int == '\u{7f}' as i32
                    && *temp.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32)
                && (*((*(*l).word).word).offset(0 as libc::c_int as isize) as libc::c_int
                    == '\u{7f}' as i32
                    && *((*(*l).word).word).offset(1 as libc::c_int as isize) as libc::c_int
                        == '\u{0}' as i32)
                && (*(*l).word).flags & (1 as libc::c_int) << 18 as libc::c_int != 0
            {
                (*w).flags |= (1 as libc::c_int) << 18 as libc::c_int;
                if !qdollaratp.is_null() && l_hasdollat != 0 {
                    *qdollaratp = 0 as libc::c_int;
                }
            }
            dispose_words(l);
        } else if quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0 && l_hasdollat != 0 {
            temp = make_quoted_char('\u{0}' as i32);
            (*w).flags |= (1 as libc::c_int) << 18 as libc::c_int;
        } else {
            temp = 0 as *mut libc::c_void as *mut libc::c_char;
        }
        if op == '-' as i32 || op == '+' as i32 {
            let ref mut fresh84 = (*w).word;
            *fresh84 = temp;
            return w;
        }
        t1 = if !temp.is_null() {
            dequote_string(temp)
        } else {
            strcpy(
                sh_xmalloc(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(strlen(b"\0" as *const u8 as *const libc::c_char) as u64),
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    7160 as libc::c_int,
                ) as *mut libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
            )
        };
        sh_xfree(
            temp as *mut libc::c_void,
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            7161 as libc::c_int,
        );
        vname = name;
        if *name as libc::c_int == '!' as i32
            && (1 as libc::c_int != 0
                && *(*__ctype_b_loc()).offset(*name.offset(1 as libc::c_int as isize)
                    as libc::c_uchar as libc::c_int
                    as isize) as libc::c_int
                    & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                    != 0
                || *name.offset(1 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                    == '_' as i32
                || *name.offset(1 as libc::c_int as isize) as libc::c_int >= '0' as i32
                    && *name.offset(1 as libc::c_int as isize) as libc::c_int <= '9' as i32
                || (posixly_correct == 0 as libc::c_int
                    && *name.offset(1 as libc::c_int as isize) as libc::c_int == '#' as i32
                    || posixly_correct == 0 as libc::c_int
                        && *name.offset(1 as libc::c_int as isize) as libc::c_int == '?' as i32
                    || *name.offset(1 as libc::c_int as isize) as libc::c_int == '@' as i32
                    || *name.offset(1 as libc::c_int as isize) as libc::c_int == '*' as i32))
        {
            vname = parameter_brace_find_indir(
                name.offset(1 as libc::c_int as isize),
                (*name as libc::c_int != 0
                    && (*name as libc::c_int >= '0' as i32
                        && *name as libc::c_int <= '9' as i32
                        && all_digits(name) != 0
                        || *name.offset(1 as libc::c_int as isize) as libc::c_int
                            == '\u{0}' as i32
                            && *sh_syntaxtab
                                .as_mut_ptr()
                                .offset(*name as libc::c_uchar as isize)
                                & 0x800 as libc::c_int
                                != 0
                        || 1 as libc::c_int != 0
                            && *name.offset(2 as libc::c_int as isize) as libc::c_int
                                == '\u{0}' as i32
                            && (posixly_correct == 0 as libc::c_int
                                && *name.offset(1 as libc::c_int as isize) as libc::c_int
                                    == '#' as i32
                                || posixly_correct == 0 as libc::c_int
                                    && *name.offset(1 as libc::c_int as isize) as libc::c_int
                                        == '?' as i32
                                || *name.offset(1 as libc::c_int as isize) as libc::c_int
                                    == '@' as i32
                                || *name.offset(1 as libc::c_int as isize) as libc::c_int
                                    == '*' as i32))) as libc::c_int,
                quoted,
                1 as libc::c_int,
            );
            if vname.is_null() || *vname as libc::c_int == 0 as libc::c_int {
                report_error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: invalid indirect expansion\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    name,
                );
                sh_xfree(
                    vname as *mut libc::c_void,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    7172 as libc::c_int,
                );
                sh_xfree(
                    t1 as *mut libc::c_void,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    7173 as libc::c_int,
                );
                dispose_word(w);
                return &mut expand_wdesc_error;
            }
            if legal_identifier(vname) == 0 as libc::c_int {
                report_error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: invalid variable name\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    vname,
                );
                sh_xfree(
                    vname as *mut libc::c_void,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    7180 as libc::c_int,
                );
                sh_xfree(
                    t1 as *mut libc::c_void,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    7181 as libc::c_int,
                );
                dispose_word(w);
                return &mut expand_wdesc_error;
            }
        }
        if valid_array_reference(vname, 0 as libc::c_int) != 0 {
            v = assign_array_element(vname, t1, 0 as libc::c_int);
        } else {
            v = bind_variable(vname, t1, 0 as libc::c_int);
        }
        if v.is_null()
            || (*v).attributes & 0x2 as libc::c_int != 0
            || (*v).attributes & 0x4000 as libc::c_int != 0
        {
            if (v.is_null() || (*v).attributes & 0x2 as libc::c_int != 0)
                && interactive_shell == 0 as libc::c_int
                && posixly_correct != 0
            {
                ::std::ptr::write_volatile(
                    &mut last_command_exit_value as *mut libc::c_int,
                    1 as libc::c_int,
                );
                exp_jump_to_top_level(1 as libc::c_int);
            } else {
                if vname != name {
                    sh_xfree(
                        vname as *mut libc::c_void,
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        7204 as libc::c_int,
                    );
                }
                ::std::ptr::write_volatile(
                    &mut last_command_exit_value as *mut libc::c_int,
                    2 as libc::c_int,
                );
                exp_jump_to_top_level(2 as libc::c_int);
            }
        }
        stupidly_hack_special_variables(vname);
        if vname != name {
            sh_xfree(
                vname as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                7213 as libc::c_int,
            );
        }
        let ref mut fresh85 = (*w).word;
        *fresh85 = if quoted & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0 {
            quote_string(t1)
        } else {
            quote_escapes(t1)
        };
        if !((*w).word).is_null()
            && *((*w).word).offset(0 as libc::c_int as isize) as libc::c_int != 0
            && (*((*w).word).offset(0 as libc::c_int as isize) as libc::c_int == '\u{7f}' as i32
                && *((*w).word).offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32)
                as libc::c_int
                == 0 as libc::c_int
        {
            (*w).flags &= !((1 as libc::c_int) << 21 as libc::c_int);
        }
        sh_xfree(
            t1 as *mut libc::c_void,
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            7226 as libc::c_int,
        );
        if quoted & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0
            && (*((*w).word).offset(0 as libc::c_int as isize) as libc::c_int == '\u{7f}' as i32
                && *((*w).word).offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32)
        {
            (*w).flags |= (1 as libc::c_int) << 18 as libc::c_int;
        }
    }
    return w;
}
fn parameter_brace_expand_error(
    mut name: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut check_null: libc::c_int,
) {
    let mut l: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        set_exit_status(1 as libc::c_int);
        if !value.is_null() && *value as libc::c_int != 0 {
            l = expand_string(value, 0 as libc::c_int);
            temp = string_list(l);
            report_error(
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                name,
                if !temp.is_null() {
                    temp
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
            if !temp.is_null() {
                sh_xfree(
                    temp as *mut libc::c_void,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    7254 as libc::c_int,
                );
            }
            dispose_words(l);
        } else if check_null == 0 as libc::c_int {
            report_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: parameter not set\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                name,
            );
        } else {
            report_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: parameter null or not set\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                name,
            );
        }
        sh_xfree(
            name as *mut libc::c_void,
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            7264 as libc::c_int,
        );
        if !value.is_null() {
            sh_xfree(
                value as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                7265 as libc::c_int,
            );
        }
    }
}
fn valid_length_expression(mut name: *mut libc::c_char) -> libc::c_int {
    unsafe {
        return (*name.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
            || *sh_syntaxtab
                .as_mut_ptr()
                .offset(*name.offset(1 as libc::c_int as isize) as libc::c_uchar as isize)
                & 0x800 as libc::c_int
                != 0
                && *name.offset(2 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
            || *name.offset(1 as libc::c_int as isize) as libc::c_int >= '0' as i32
                && *name.offset(1 as libc::c_int as isize) as libc::c_int <= '9' as i32
                && all_digits(name.offset(1 as libc::c_int as isize)) != 0
            || valid_array_reference(name.offset(1 as libc::c_int as isize), 0 as libc::c_int) != 0
            || legal_identifier(name.offset(1 as libc::c_int as isize)) != 0)
            as libc::c_int;
    }
}
fn parameter_brace_expand_length(mut name: *mut libc::c_char) -> intmax_t {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut number: intmax_t = 0;
    let mut arg_index: intmax_t = 0;
    let mut list: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    var = 0 as *mut libc::c_void as *mut SHELL_VAR;
    unsafe {
        if *name.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32 {
            number = number_of_args() as intmax_t;
        } else if (*name.offset(1 as libc::c_int as isize) as libc::c_int == '@' as i32
            || *name.offset(1 as libc::c_int as isize) as libc::c_int == '*' as i32)
            && *name.offset(2 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
        {
            number = number_of_args() as intmax_t;
        } else if *sh_syntaxtab
            .as_mut_ptr()
            .offset(*name.offset(1 as libc::c_int as isize) as libc::c_uchar as isize)
            & 0x800 as libc::c_int
            != 0
            && *name.offset(2 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
        {
            match *name.offset(1 as libc::c_int as isize) as libc::c_int {
                45 => {
                    t = which_set_flags();
                }
                63 => {
                    t = itos(last_command_exit_value as intmax_t);
                }
                36 => {
                    t = itos(dollar_dollar_pid as intmax_t);
                }
                33 => {
                    if last_asynchronous_pid == -(1 as libc::c_int) {
                        t = 0 as *mut libc::c_void as *mut libc::c_char;
                    } else {
                        t = itos(last_asynchronous_pid as intmax_t);
                    }
                }
                35 => {
                    t = itos(number_of_args() as intmax_t);
                }
                _ => {}
            }
            number = (if !t.is_null() && *t.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
                if *t.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                    if *t.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                        strlen(t) as usize
                    } else {
                        2 as libc::c_int as usize
                    }
                } else {
                    1 as libc::c_int as usize
                }
            } else {
                0 as libc::c_int as usize
            }) as intmax_t;
            if !t.is_null() {
                sh_xfree(
                    t as *mut libc::c_void,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    7325 as libc::c_int,
                );
            }
        } else if valid_array_reference(name.offset(1 as libc::c_int as isize), 0 as libc::c_int)
            != 0
        {
            number = array_length_reference(name.offset(1 as libc::c_int as isize));
        } else {
            number = 0 as libc::c_int as intmax_t;
            if legal_number(name.offset(1 as libc::c_int as isize), &mut arg_index) != 0 {
                t = get_dollar_var_value(arg_index);
                if t.is_null() && unbound_vars_is_error != 0 {
                    return -(9223372036854775807 as libc::c_long)
                        - 1 as libc::c_int as libc::c_long;
                }
                number = (if __ctype_get_mb_cur_max() > 1 as libc::c_int as usize {
                    if !t.is_null() && *t.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
                        if *t.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                            mbstrlen(t) as usize
                        } else {
                            1 as libc::c_int as usize
                        }
                    } else {
                        0 as libc::c_int as usize
                    }
                } else if !t.is_null() && *t.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
                    if *t.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                        if *t.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                            strlen(t) as usize
                        } else {
                            2 as libc::c_int as usize
                        }
                    } else {
                        1 as libc::c_int as usize
                    }
                } else {
                    0 as libc::c_int as usize
                }) as intmax_t;
                if !t.is_null() {
                    sh_xfree(
                        t as *mut libc::c_void,
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        7341 as libc::c_int,
                    );
                }
            } else {
                var = find_variable(name.offset(1 as libc::c_int as isize));
                if !var.is_null()
                    && (*var).attributes & 0x1000 as libc::c_int == 0 as libc::c_int
                    && ((*var).attributes & 0x4 as libc::c_int != 0
                        || (*var).attributes & 0x40 as libc::c_int != 0)
                {
                    if (*var).attributes & 0x40 as libc::c_int != 0 {
                        t = assoc_reference(
                            (*var).value as *mut HASH_TABLE,
                            b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        );
                    } else {
                        t = array_reference(
                            (*var).value as *mut ARRAY,
                            0 as libc::c_int as arrayind_t,
                        );
                    }
                    if t.is_null() && unbound_vars_is_error != 0 {
                        return -(9223372036854775807 as libc::c_long)
                            - 1 as libc::c_int as libc::c_long;
                    }
                    number = (if __ctype_get_mb_cur_max() > 1 as libc::c_int as usize {
                        if !t.is_null() && *t.offset(0 as libc::c_int as isize) as libc::c_int != 0
                        {
                            if *t.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                                mbstrlen(t) as usize
                            } else {
                                1 as libc::c_int as usize
                            }
                        } else {
                            0 as libc::c_int as usize
                        }
                    } else if !t.is_null()
                        && *t.offset(0 as libc::c_int as isize) as libc::c_int != 0
                    {
                        if *t.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                            if *t.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                                strlen(t) as usize
                            } else {
                                2 as libc::c_int as usize
                            }
                        } else {
                            1 as libc::c_int as usize
                        }
                    } else {
                        0 as libc::c_int as usize
                    }) as intmax_t;
                } else if (!var.is_null() || {
                    var = find_variable(name.offset(1 as libc::c_int as isize));
                    !var.is_null()
                }) && (*var).attributes & 0x1000 as libc::c_int == 0 as libc::c_int
                    && (*var).attributes & 0x4 as libc::c_int == 0 as libc::c_int
                    && (*var).attributes & 0x40 as libc::c_int == 0 as libc::c_int
                    && ((*var).dynamic_value).is_none()
                {
                    number = (if !((*var).value).is_null() {
                        if __ctype_get_mb_cur_max() > 1 as libc::c_int as usize {
                            if !((*var).value).is_null()
                                && *((*var).value).offset(0 as libc::c_int as isize) as libc::c_int
                                    != 0
                            {
                                if *((*var).value).offset(1 as libc::c_int as isize) as libc::c_int
                                    != 0
                                {
                                    mbstrlen((*var).value) as usize
                                } else {
                                    1 as libc::c_int as usize
                                }
                            } else {
                                0 as libc::c_int as usize
                            }
                        } else if !((*var).value).is_null()
                            && *((*var).value).offset(0 as libc::c_int as isize) as libc::c_int != 0
                        {
                            if *((*var).value).offset(1 as libc::c_int as isize) as libc::c_int != 0
                            {
                                if *((*var).value).offset(2 as libc::c_int as isize) as libc::c_int
                                    != 0
                                {
                                    strlen((*var).value) as usize
                                } else {
                                    2 as libc::c_int as usize
                                }
                            } else {
                                1 as libc::c_int as usize
                            }
                        } else {
                            0 as libc::c_int as usize
                        }
                    } else {
                        0 as libc::c_int as usize
                    }) as i64;
                } else if var.is_null() && unbound_vars_is_error == 0 as libc::c_int {
                    number = 0 as libc::c_int as i64;
                } else {
                    newname = strcpy(
                        sh_xmalloc(
                            (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(name) as u64),
                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                            7366 as libc::c_int,
                        ) as *mut libc::c_char,
                        name,
                    );
                    *newname.offset(0 as libc::c_int as isize) = '$' as i32 as libc::c_char;
                    list = expand_string(newname, 0x1 as libc::c_int);
                    t = if !list.is_null() {
                        string_list(list)
                    } else {
                        0 as *mut libc::c_void as *mut libc::c_char
                    };
                    sh_xfree(
                        newname as *mut libc::c_void,
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        7370 as libc::c_int,
                    );
                    if !list.is_null() {
                        dispose_words(list);
                    }
                    number = (if !t.is_null() {
                        if __ctype_get_mb_cur_max() > 1 as libc::c_int as usize {
                            if !t.is_null()
                                && *t.offset(0 as libc::c_int as isize) as libc::c_int != 0
                            {
                                if *t.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                                    mbstrlen(t) as usize
                                } else {
                                    1 as libc::c_int as usize
                                }
                            } else {
                                0 as libc::c_int as usize
                            }
                        } else if !t.is_null()
                            && *t.offset(0 as libc::c_int as isize) as libc::c_int != 0
                        {
                            if *t.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                                if *t.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                                    strlen(t) as usize
                                } else {
                                    2 as libc::c_int as usize
                                }
                            } else {
                                1 as libc::c_int as usize
                            }
                        } else {
                            0 as libc::c_int as usize
                        }
                    } else {
                        0 as libc::c_int as usize
                    }) as intmax_t;
                    if !t.is_null() {
                        sh_xfree(
                            t as *mut libc::c_void,
                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                            7375 as libc::c_int,
                        );
                    }
                }
            }
        }
    }
    return number;
}
fn skiparith(mut substr: *mut libc::c_char, mut delim: libc::c_int) -> *mut libc::c_char {
    unsafe {
        let mut i: libc::c_int = 0;
        let mut delims: [libc::c_char; 2] = [0; 2];
        delims[0 as libc::c_int as usize] = delim as libc::c_char;
        delims[1 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        i = skip_to_delim(
            substr,
            0 as libc::c_int,
            delims.as_mut_ptr(),
            0x400 as libc::c_int,
        );
        return substr.offset(i as isize);
    }
}
fn verify_substring_values(
    mut v: *mut SHELL_VAR,
    mut value: *mut libc::c_char,
    mut substr: *mut libc::c_char,
    mut vtype: libc::c_int,
    mut e1p: *mut intmax_t,
    mut e2p: *mut intmax_t,
) -> libc::c_int {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: arrayind_t = 0;
    let mut expok: libc::c_int = 0;
    let mut a: *mut ARRAY = 0 as *mut ARRAY;
    let mut h: *mut HASH_TABLE = 0 as *mut HASH_TABLE;
    t = skiparith(substr, ':' as i32);
    unsafe {
        if *t as libc::c_int != 0 && *t as libc::c_int == ':' as i32 {
            *t = '\u{0}' as i32 as libc::c_char;
        } else {
            t = 0 as *mut libc::c_char;
        }
        temp1 = expand_arith_string(substr, 0x1 as libc::c_int);
        *e1p = evalexp(temp1, 0 as libc::c_int, &mut expok);
        sh_xfree(
            temp1 as *mut libc::c_void,
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            7435 as libc::c_int,
        );
        if expok == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        len = -(1 as libc::c_int) as arrayind_t;
        match vtype {
            0 | 3 => {
                len = (if __ctype_get_mb_cur_max() > 1 as libc::c_int as usize {
                    if !value.is_null()
                        && *value.offset(0 as libc::c_int as isize) as libc::c_int != 0
                    {
                        if *value.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                            mbstrlen(value) as usize
                        } else {
                            1 as libc::c_int as usize
                        }
                    } else {
                        0 as libc::c_int as usize
                    }
                } else if !value.is_null()
                    && *value.offset(0 as libc::c_int as isize) as libc::c_int != 0
                {
                    if *value.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                        if *value.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                            strlen(value) as usize
                        } else {
                            2 as libc::c_int as usize
                        }
                    } else {
                        1 as libc::c_int as usize
                    }
                } else {
                    0 as libc::c_int as usize
                }) as arrayind_t;
            }
            1 => {
                len = (number_of_args() + 1 as libc::c_int) as arrayind_t;
                if *e1p == 0 as libc::c_int as libc::c_long {
                    len += 1;
                }
            }
            2 => {
                if (*v).attributes & 0x40 as libc::c_int != 0 {
                    h = (*v).value as *mut HASH_TABLE;
                    len = ((*h).nentries + (*e1p < 0 as libc::c_int as libc::c_long) as libc::c_int)
                        as arrayind_t;
                } else {
                    a = value as *mut ARRAY;
                    len = (*a).max_index
                        + (*e1p < 0 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long;
                }
            }
            _ => {}
        }
        if len == -(1 as libc::c_int) as libc::c_long {
            return -(1 as libc::c_int);
        }
        if *e1p < 0 as libc::c_int as libc::c_long {
            *e1p += len;
        }
        if *e1p > len || *e1p < 0 as libc::c_int as libc::c_long {
            return -(1 as libc::c_int);
        }
        if vtype == 2 as libc::c_int {
            len = (if (*v).attributes & 0x40 as libc::c_int != 0 {
                (*h).nentries
            } else {
                (*a).num_elements
            }) as arrayind_t;
        }
        if !t.is_null() {
            t = t.offset(1);
            temp2 = strcpy(
                sh_xmalloc(
                    (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(t) as u64),
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    7488 as libc::c_int,
                ) as *mut libc::c_char,
                t,
            );
            temp1 = expand_arith_string(temp2, 0x1 as libc::c_int);
            sh_xfree(
                temp2 as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                7490 as libc::c_int,
            );
            *t.offset(-(1 as libc::c_int) as isize) = ':' as i32 as libc::c_char;
            *e2p = evalexp(temp1, 0 as libc::c_int, &mut expok);
            sh_xfree(
                temp1 as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                7493 as libc::c_int,
            );
            if expok == 0 as libc::c_int {
                return 0 as libc::c_int;
            }
            if (vtype == 2 as libc::c_int || vtype == 1 as libc::c_int)
                && *e2p < 0 as libc::c_int as libc::c_long
            {
                internal_error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: substring expression < 0\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    t,
                );
                return 0 as libc::c_int;
            }
            if vtype != 2 as libc::c_int {
                if *e2p < 0 as libc::c_int as libc::c_long {
                    *e2p += len;
                    if *e2p < 0 as libc::c_int as libc::c_long || *e2p < *e1p {
                        internal_error(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s: substring expression < 0\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            t,
                        );
                        return 0 as libc::c_int;
                    }
                } else {
                    *e2p += *e1p;
                }
                if *e2p > len {
                    *e2p = len;
                }
            }
        } else {
            *e2p = len;
        }
    }
    return 1 as libc::c_int;
}
fn get_var_and_type(
    mut varname: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut ind: arrayind_t,
    mut quoted: libc::c_int,
    mut flags: libc::c_int,
    mut varp: *mut *mut SHELL_VAR,
    mut valp: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut vtype: libc::c_int = 0;
    let mut want_indir: libc::c_int = 0;
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut lind: arrayind_t = 0;
    unsafe {
        want_indir = (*varname as libc::c_int == '!' as i32
            && (1 as libc::c_int != 0
                && *(*__ctype_b_loc())
                    .offset(*varname.offset(1 as libc::c_int as isize) as libc::c_uchar
                        as libc::c_int as isize) as libc::c_int
                    & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                    != 0
                || *varname.offset(1 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                    == '_' as i32
                || *varname.offset(1 as libc::c_int as isize) as libc::c_int >= '0' as i32
                    && *varname.offset(1 as libc::c_int as isize) as libc::c_int <= '9' as i32
                || (posixly_correct == 0 as libc::c_int
                    && *varname.offset(1 as libc::c_int as isize) as libc::c_int == '#' as i32
                    || posixly_correct == 0 as libc::c_int
                        && *varname.offset(1 as libc::c_int as isize) as libc::c_int
                            == '?' as i32
                    || *varname.offset(1 as libc::c_int as isize) as libc::c_int == '@' as i32
                    || *varname.offset(1 as libc::c_int as isize) as libc::c_int == '*' as i32)))
            as libc::c_int;
        if want_indir != 0 {
            vname = parameter_brace_find_indir(
                varname.offset(1 as libc::c_int as isize),
                (*varname as libc::c_int != 0
                    && (*varname as libc::c_int >= '0' as i32
                        && *varname as libc::c_int <= '9' as i32
                        && all_digits(varname) != 0
                        || *varname.offset(1 as libc::c_int as isize) as libc::c_int
                            == '\u{0}' as i32
                            && *sh_syntaxtab
                                .as_mut_ptr()
                                .offset(*varname as libc::c_uchar as isize)
                                & 0x800 as libc::c_int
                                != 0
                        || 1 as libc::c_int != 0
                            && *varname.offset(2 as libc::c_int as isize) as libc::c_int
                                == '\u{0}' as i32
                            && (posixly_correct == 0 as libc::c_int
                                && *varname.offset(1 as libc::c_int as isize) as libc::c_int
                                    == '#' as i32
                                || posixly_correct == 0 as libc::c_int
                                    && *varname.offset(1 as libc::c_int as isize) as libc::c_int
                                        == '?' as i32
                                || *varname.offset(1 as libc::c_int as isize) as libc::c_int
                                    == '@' as i32
                                || *varname.offset(1 as libc::c_int as isize) as libc::c_int
                                    == '*' as i32))) as libc::c_int,
                quoted,
                1 as libc::c_int,
            );
        } else {
            vname = varname;
        }
        if vname.is_null() {
            vtype = 0 as libc::c_int;
            *varp = 0 as *mut libc::c_void as *mut SHELL_VAR;
            *valp = 0 as *mut libc::c_void as *mut libc::c_char;
            return vtype;
        }
        vtype = ((*vname.offset(0 as libc::c_int as isize) as libc::c_int == '@' as i32
            || *vname.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32)
            && *vname.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32)
            as libc::c_int;
        if vtype == 1 as libc::c_int
            && *vname.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32
        {
            vtype |= 128 as libc::c_int;
        }
        *varp = 0 as *mut libc::c_void as *mut SHELL_VAR;
        if valid_array_reference(vname, 0 as libc::c_int) != 0 {
            v = array_variable_part(vname, 0 as libc::c_int, &mut temp, 0 as *mut libc::c_int);
            lind = if ind
                != -(9223372036854775807 as libc::c_long) - 1 as libc::c_int as libc::c_long
                && flags & 0x4 as libc::c_int != 0
            {
                ind
            } else {
                0 as libc::c_int as libc::c_long
            };
            if !v.is_null() && (*v).attributes & 0x1000 as libc::c_int != 0 {
                vtype = 3 as libc::c_int;
                *varp = 0 as *mut libc::c_void as *mut SHELL_VAR;
                *valp = 0 as *mut libc::c_void as *mut libc::c_char;
            }
            if !v.is_null()
                && ((*v).attributes & 0x4 as libc::c_int != 0
                    || (*v).attributes & 0x40 as libc::c_int != 0)
            {
                if (*temp.offset(0 as libc::c_int as isize) as libc::c_int == '@' as i32
                    || *temp.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32)
                    && *temp.offset(1 as libc::c_int as isize) as libc::c_int == ']' as i32
                {
                    vtype = 2 as libc::c_int;
                    if *temp.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32 {
                        vtype |= 128 as libc::c_int;
                    }
                    *valp = if (*v).attributes & 0x4 as libc::c_int != 0 {
                        (*v).value as *mut ARRAY as *mut libc::c_char
                    } else {
                        (*v).value as *mut HASH_TABLE as *mut libc::c_char
                    };
                } else {
                    vtype = 3 as libc::c_int;
                    *valp = array_value(
                        vname,
                        0x1 as libc::c_int,
                        flags,
                        0 as *mut libc::c_void as *mut libc::c_int,
                        &mut lind,
                    );
                }
                *varp = v;
            } else if !v.is_null()
                && ((*temp.offset(0 as libc::c_int as isize) as libc::c_int == '@' as i32
                    || *temp.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32)
                    && *temp.offset(1 as libc::c_int as isize) as libc::c_int == ']' as i32)
            {
                vtype = 0 as libc::c_int;
                *varp = v;
                if quoted & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0 {
                    *valp = if !value.is_null() {
                        dequote_string(value)
                    } else {
                        0 as *mut libc::c_void as *mut libc::c_char
                    };
                } else {
                    *valp = if !value.is_null() {
                        dequote_escapes(value)
                    } else {
                        0 as *mut libc::c_void as *mut libc::c_char
                    };
                }
            } else {
                vtype = 3 as libc::c_int;
                *varp = v;
                *valp = array_value(
                    vname,
                    0x1 as libc::c_int,
                    flags,
                    0 as *mut libc::c_void as *mut libc::c_int,
                    &mut lind,
                );
            }
        } else {
            v = find_variable(vname);
            if !v.is_null()
                && (*v).attributes & 0x1000 as libc::c_int == 0 as libc::c_int
                && ((*v).attributes & 0x40 as libc::c_int != 0
                    || (*v).attributes & 0x4 as libc::c_int != 0)
            {
                vtype = 3 as libc::c_int;
                *varp = v;
                *valp = if (*v).attributes & 0x40 as libc::c_int != 0 {
                    assoc_reference(
                        (*v).value as *mut HASH_TABLE,
                        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    )
                } else {
                    array_reference((*v).value as *mut ARRAY, 0 as libc::c_int as arrayind_t)
                };
            } else if !value.is_null() && vtype == 0 as libc::c_int {
                *varp = find_variable(vname);
                if quoted & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0 {
                    *valp = dequote_string(value);
                } else {
                    *valp = dequote_escapes(value);
                }
            } else {
                *valp = value;
            }
        }
        if want_indir != 0 {
            sh_xfree(
                vname as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                7650 as libc::c_int,
            );
        }
    }
    return vtype;
}
fn string_var_assignment(mut v: *mut SHELL_VAR, mut s: *mut libc::c_char) -> *mut libc::c_char {
    let mut flags: [libc::c_char; 16] = [0; 16];
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    unsafe {
        val = if !v.is_null()
            && ((*v).attributes & 0x1000 as libc::c_int != 0
                || ((*v).value != 0 as *mut libc::c_char) as libc::c_int == 0 as libc::c_int)
        {
            0 as *mut libc::c_void as *mut libc::c_char
        } else {
            sh_quote_reusable(s, 0 as libc::c_int)
        };
        i = var_attribute_string(v, 0 as libc::c_int, flags.as_mut_ptr());
        if i == 0 as libc::c_int && val.is_null() {
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
        ret = sh_xmalloc(
            (i as libc::c_ulong)
                .wrapping_add(
                    if !val.is_null() && *val.offset(0 as libc::c_int as isize) as libc::c_int != 0
                    {
                        if *val.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                            if *val.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                                strlen(val) as u64
                            } else {
                                2 as libc::c_int as libc::c_ulong
                            }
                        } else {
                            1 as libc::c_int as libc::c_ulong
                        }
                    } else {
                        0 as libc::c_int as libc::c_ulong
                    },
                )
                .wrapping_add(strlen((*v).name) as u64)
                .wrapping_add(16 as libc::c_int as libc::c_ulong)
                .wrapping_add(16 as libc::c_int as libc::c_ulong),
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            7674 as libc::c_int,
        ) as *mut libc::c_char;
        if i > 0 as libc::c_int && val.is_null() {
            sprintf(
                ret,
                b"declare -%s %s\0" as *const u8 as *const libc::c_char,
                flags.as_mut_ptr(),
                (*v).name,
            );
        } else if i > 0 as libc::c_int {
            sprintf(
                ret,
                b"declare -%s %s=%s\0" as *const u8 as *const libc::c_char,
                flags.as_mut_ptr(),
                (*v).name,
                val,
            );
        } else {
            sprintf(
                ret,
                b"%s=%s\0" as *const u8 as *const libc::c_char,
                (*v).name,
                val,
            );
        }
        sh_xfree(
            val as *mut libc::c_void,
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            7681 as libc::c_int,
        );
    }
    return ret;
}
fn array_var_assignment(
    mut v: *mut SHELL_VAR,
    mut itype: libc::c_int,
    mut quoted: libc::c_int,
    mut atype: libc::c_int,
) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut flags: [libc::c_char; 16] = [0; 16];
    let mut i: libc::c_int = 0;
    if v.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    unsafe {
        if atype == 2 as libc::c_int {
            val = if (*v).attributes & 0x4 as libc::c_int != 0 {
                array_to_kvpair((*v).value as *mut ARRAY, 0 as libc::c_int)
            } else {
                assoc_to_kvpair((*v).value as *mut HASH_TABLE, 0 as libc::c_int)
            };
        } else {
            val = if (*v).attributes & 0x4 as libc::c_int != 0 {
                array_to_assign((*v).value as *mut ARRAY, 0 as libc::c_int)
            } else {
                assoc_to_assign((*v).value as *mut HASH_TABLE, 0 as libc::c_int)
            };
        }
        if !(val.is_null()
            && ((*v).attributes & 0x1000 as libc::c_int != 0
                || ((*v).value != 0 as *mut libc::c_char) as libc::c_int == 0 as libc::c_int))
        {
            if val.is_null() {
                val = sh_xmalloc(
                    3 as libc::c_int as size_t,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    7707 as libc::c_int,
                ) as *mut libc::c_char;
                *val.offset(0 as libc::c_int as isize) = '(' as i32 as libc::c_char;
                *val.offset(1 as libc::c_int as isize) = ')' as i32 as libc::c_char;
                *val.offset(2 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
            } else {
                ret = if quoted & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0 {
                    quote_string(val)
                } else {
                    quote_escapes(val)
                };
                sh_xfree(
                    val as *mut libc::c_void,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    7715 as libc::c_int,
                );
                val = ret;
            }
        }
        if atype == 2 as libc::c_int {
            return val;
        }
        i = var_attribute_string(v, 0 as libc::c_int, flags.as_mut_ptr());
        ret = sh_xmalloc(
            (i as libc::c_ulong)
                .wrapping_add(
                    if !val.is_null() && *val.offset(0 as libc::c_int as isize) as libc::c_int != 0
                    {
                        if *val.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                            if *val.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                                strlen(val) as u64
                            } else {
                                2 as libc::c_int as libc::c_ulong
                            }
                        } else {
                            1 as libc::c_int as libc::c_ulong
                        }
                    } else {
                        0 as libc::c_int as libc::c_ulong
                    },
                )
                .wrapping_add(strlen((*v).name) as u64)
                .wrapping_add(16 as libc::c_int as libc::c_ulong),
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            7723 as libc::c_int,
        ) as *mut libc::c_char;
        if !val.is_null() {
            sprintf(
                ret,
                b"declare -%s %s=%s\0" as *const u8 as *const libc::c_char,
                flags.as_mut_ptr(),
                (*v).name,
                val,
            );
        } else {
            sprintf(
                ret,
                b"declare -%s %s\0" as *const u8 as *const libc::c_char,
                flags.as_mut_ptr(),
                (*v).name,
            );
        }
        sh_xfree(
            val as *mut libc::c_void,
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            7728 as libc::c_int,
        );
    }
    return ret;
}
fn pos_params_assignment(
    mut list: *mut WORD_LIST,
    mut itype: libc::c_int,
    mut quoted: libc::c_int,
) -> *mut libc::c_char {
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    temp = list_transform('Q' as i32, 0 as *mut SHELL_VAR, list, itype, quoted);
    unsafe {
        ret = sh_xmalloc(
            (strlen(temp)).wrapping_add(8 as libc::c_int as u64) as u64,
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            7743 as libc::c_int,
        ) as *mut libc::c_char;
        strcpy(ret, b"set -- \0" as *const u8 as *const libc::c_char);
        strcpy(ret.offset(7 as libc::c_int as isize), temp);
        sh_xfree(
            temp as *mut libc::c_void,
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            7746 as libc::c_int,
        );
    }
    return ret;
}
fn string_transform(
    mut xc: libc::c_int,
    mut v: *mut SHELL_VAR,
    mut s: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut flags: [libc::c_char; 16] = [0; 16];
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    if (xc == 'A' as i32 || xc == 'a' as i32) && v.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    } else {
        if xc != 'a' as i32 && xc != 'A' as i32 && s.is_null() {
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
    }
    unsafe {
        match xc {
            97 => {
                i = var_attribute_string(v, 0 as libc::c_int, flags.as_mut_ptr());
                ret = if i > 0 as libc::c_int {
                    strcpy(
                        sh_xmalloc(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(strlen(flags.as_mut_ptr()) as u64),
                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                            7769 as libc::c_int,
                        ) as *mut libc::c_char,
                        flags.as_mut_ptr(),
                    )
                } else {
                    0 as *mut libc::c_void as *mut libc::c_char
                };
            }
            65 => {
                ret = string_var_assignment(v, s);
            }
            75 => {
                ret = sh_quote_reusable(s, 0 as libc::c_int);
            }
            69 => {
                t = ansiexpand(
                    s,
                    0 as libc::c_int,
                    strlen(s) as libc::c_int,
                    0 as *mut libc::c_int,
                );
                ret = dequote_escapes(t);
                sh_xfree(
                    t as *mut libc::c_void,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    7781 as libc::c_int,
                );
            }
            80 => {
                ret = decode_prompt_string(s);
            }
            81 => {
                ret = sh_quote_reusable(s, 0 as libc::c_int);
            }
            85 => {
                ret = sh_modcase(s, 0 as *mut libc::c_char, 0x2 as libc::c_int);
            }
            117 => {
                ret = sh_modcase(s, 0 as *mut libc::c_char, 0x40 as libc::c_int);
            }
            76 => {
                ret = sh_modcase(s, 0 as *mut libc::c_char, 0x1 as libc::c_int);
            }
            _ => {
                ret = 0 as *mut libc::c_void as *mut libc::c_char;
            }
        }
    }
    return ret;
}
fn list_transform(
    mut xc: libc::c_int,
    mut v: *mut SHELL_VAR,
    mut list: *mut WORD_LIST,
    mut itype: libc::c_int,
    mut quoted: libc::c_int,
) -> *mut libc::c_char {
    let mut new: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut l: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut w: *mut WORD_DESC = 0 as *mut WORD_DESC;
    let mut tword: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut qflags: libc::c_int = 0;
    new = 0 as *mut libc::c_void as *mut WORD_LIST;
    l = list;
    unsafe {
        while !l.is_null() {
            tword = string_transform(xc, v, (*(*l).word).word);
            w = alloc_word_desc();
            let ref mut fresh86 = (*w).word;
            *fresh86 = if !tword.is_null() {
                tword
            } else {
                strcpy(
                    sh_xmalloc(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_add(strlen(b"\0" as *const u8 as *const libc::c_char) as u64),
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        7821 as libc::c_int,
                    ) as *mut libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                )
            };
            new = make_word_list(w, new);
            l = (*l).next;
        }
        l = if !new.is_null() && !((*new).next).is_null() {
            list_reverse(new as *mut GENERIC_LIST) as *mut WORD_LIST
        } else {
            new
        };
        qflags = quoted;
        if itype == '*' as i32 && expand_no_split_dollar_star != 0 && ifs_is_null != 0 {
            qflags |= 0x1 as libc::c_int;
        }
        tword = string_list_pos_params(itype, l, qflags, 0 as libc::c_int);
        dispose_words(l);
    }
    return tword;
}
fn parameter_list_transform(
    mut xc: libc::c_int,
    mut itype: libc::c_int,
    mut quoted: libc::c_int,
) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut list: *mut WORD_LIST = 0 as *mut WORD_LIST;
    list = list_rest_of_args();
    if list.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    if xc == 'A' as i32 {
        ret = pos_params_assignment(list, itype, quoted);
    } else {
        ret = list_transform(xc, 0 as *mut SHELL_VAR, list, itype, quoted);
    }
    dispose_words(list);
    return ret;
}
fn array_transform(
    mut xc: libc::c_int,
    mut var: *mut SHELL_VAR,
    mut starsub: libc::c_int,
    mut quoted: libc::c_int,
) -> *mut libc::c_char {
    let mut a: *mut ARRAY = 0 as *mut ARRAY;
    let mut h: *mut HASH_TABLE = 0 as *mut HASH_TABLE;
    let mut itype: libc::c_int = 0;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut list: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    v = var;
    itype = if starsub != 0 { '*' as i32 } else { '@' as i32 };
    if xc == 'A' as i32 {
        return array_var_assignment(v, itype, quoted, 1 as libc::c_int);
    } else {
        if xc == 'K' as i32 {
            return array_var_assignment(v, itype, quoted, 2 as libc::c_int);
        }
    }
    unsafe {
        if xc == 'a' as i32
            && ((*v).attributes & 0x1000 as libc::c_int != 0
                || ((*v).value != 0 as *mut libc::c_char) as libc::c_int == 0 as libc::c_int)
        {
            let mut flags: [libc::c_char; 16] = [0; 16];
            let mut i: libc::c_int = 0;
            i = var_attribute_string(v, 0 as libc::c_int, flags.as_mut_ptr());
            return if i > 0 as libc::c_int {
                strcpy(
                    sh_xmalloc(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_add(strlen(flags.as_mut_ptr()) as u64),
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        7889 as libc::c_int,
                    ) as *mut libc::c_char,
                    flags.as_mut_ptr(),
                )
            } else {
                0 as *mut libc::c_void as *mut libc::c_char
            };
        }
        a = if !v.is_null() && (*v).attributes & 0x4 as libc::c_int != 0 {
            (*v).value as *mut ARRAY
        } else {
            0 as *mut ARRAY
        };
        h = if !v.is_null() && (*v).attributes & 0x40 as libc::c_int != 0 {
            (*v).value as *mut HASH_TABLE
        } else {
            0 as *mut HASH_TABLE
        };
        list = if !a.is_null() {
            array_to_word_list(a)
        } else if !h.is_null() {
            assoc_to_word_list(h)
        } else {
            0 as *mut WORD_LIST
        };
        if list.is_null() {
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
        ret = list_transform(xc, v, list, itype, quoted);
        dispose_words(list);
    }
    return ret;
}
fn valid_parameter_transform(mut xform: *mut libc::c_char) -> libc::c_int {
    if unsafe { *xform.offset(1 as libc::c_int as isize) != 0 } {
        return 0 as libc::c_int;
    }
    let mut current_block_3: u64;
    match unsafe { *xform.offset(0 as libc::c_int as isize) as libc::c_int } {
        65 => {
            current_block_3 = 2998016214394651642;
        }
        75 => {
            current_block_3 = 2998016214394651642;
        }
        69 => {
            current_block_3 = 9692932599894361468;
        }
        80 => {
            current_block_3 = 1699338852606067860;
        }
        81 => {
            current_block_3 = 14850354526820511888;
        }
        85 => {
            current_block_3 = 12335206852979633349;
        }
        117 => {
            current_block_3 = 6485355310389479173;
        }
        97 | 76 => {
            current_block_3 = 10930428687648770479;
        }
        _ => return 0 as libc::c_int,
    }
    match current_block_3 {
        2998016214394651642 => {
            current_block_3 = 9692932599894361468;
        }
        _ => {}
    }
    match current_block_3 {
        9692932599894361468 => {
            current_block_3 = 1699338852606067860;
        }
        _ => {}
    }
    match current_block_3 {
        1699338852606067860 => {
            current_block_3 = 14850354526820511888;
        }
        _ => {}
    }
    match current_block_3 {
        14850354526820511888 => {
            current_block_3 = 12335206852979633349;
        }
        _ => {}
    }
    match current_block_3 {
        12335206852979633349 => {
            current_block_3 = 6485355310389479173;
        }
        _ => {}
    }
    match current_block_3 {
        6485355310389479173 => {}
        _ => {}
    }
    return 1 as libc::c_int;
}
fn parameter_brace_transform(
    mut varname: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut ind: libc::c_int,
    mut xform: *mut libc::c_char,
    mut rtype: libc::c_int,
    mut quoted: libc::c_int,
    mut pflags: libc::c_int,
    mut flags: libc::c_int,
) -> *mut libc::c_char {
    let mut vtype: libc::c_int = 0;
    let mut xc: libc::c_int = 0;
    let mut starsub: libc::c_int = 0;
    let mut temp1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut oname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    unsafe {
        xc = *xform.offset(0 as libc::c_int as isize) as libc::c_int;
    }
    if value.is_null() && xc != 'A' as i32 && xc != 'a' as i32 {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    unsafe {
        oname = this_command_name;
        this_command_name = varname;
        vtype = get_var_and_type(
            varname,
            value,
            ind as arrayind_t,
            quoted,
            flags,
            &mut v,
            &mut val,
        );
        if vtype == -(1 as libc::c_int) {
            this_command_name = oname;
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
        if valid_parameter_transform(xform) == 0 as libc::c_int {
            this_command_name = oname;
            return &mut expand_param_error;
        }
        starsub = vtype & 128 as libc::c_int;
        vtype &= !(128 as libc::c_int);
        if (xc == 'a' as i32 || xc == 'A' as i32)
            && vtype == 0 as libc::c_int
            && !varname.is_null()
            && v.is_null()
        {
            v = find_variable(varname);
        }
        temp1 = 0 as *mut libc::c_void as *mut libc::c_char;
        match vtype {
            0 | 3 => {
                temp1 = string_transform(xc, v, val);
                if vtype == 0 as libc::c_int {
                    if !val.is_null() {
                        sh_xfree(
                            val as *mut libc::c_void,
                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                            7980 as libc::c_int,
                        );
                    }
                }
                if !temp1.is_null() {
                    val = if quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0 {
                        quote_string(temp1)
                    } else {
                        quote_escapes(temp1)
                    };
                    sh_xfree(
                        temp1 as *mut libc::c_void,
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        7986 as libc::c_int,
                    );
                    temp1 = val;
                }
            }
            2 => {
                temp1 = array_transform(xc, v, starsub, quoted);
                if !(!temp1.is_null() && quoted == 0 as libc::c_int && ifs_is_null != 0) {
                    if !temp1.is_null()
                        && quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) == 0 as libc::c_int
                    {
                        val = quote_escapes(temp1);
                        sh_xfree(
                            temp1 as *mut libc::c_void,
                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                            8000 as libc::c_int,
                        );
                        temp1 = val;
                    }
                }
            }
            1 => {
                temp1 = parameter_list_transform(
                    xc,
                    *varname.offset(0 as libc::c_int as isize) as libc::c_int,
                    quoted,
                );
                if !(!temp1.is_null() && quoted == 0 as libc::c_int && ifs_is_null != 0) {
                    if !temp1.is_null()
                        && quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) == 0 as libc::c_int
                    {
                        val = quote_escapes(temp1);
                        sh_xfree(
                            temp1 as *mut libc::c_void,
                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                            8014 as libc::c_int,
                        );
                        temp1 = val;
                    }
                }
            }
            _ => {}
        }
        this_command_name = oname;
    }
    return temp1;
}
fn mb_substring(
    mut string: *mut libc::c_char,
    mut s: libc::c_int,
    mut e: libc::c_int,
) -> *mut libc::c_char {
    let mut tt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: libc::c_int = 0;
    let mut stop: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut slen: size_t = 0;
    let mut state: mbstate_t = mbstate_t {
        __count: 0,
        __value: mbstate_t_value { __wch: 0 },
    };
    unsafe {
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<mbstate_t>() as usize,
        );
        start = 0 as libc::c_int;
        slen = if __ctype_get_mb_cur_max() > 1 as libc::c_int as usize {
            if !string.is_null() && *string.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
                if *string.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                    if *string.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                        strlen(string) as u64
                    } else {
                        2 as libc::c_int as libc::c_ulong
                    }
                } else {
                    1 as libc::c_int as libc::c_ulong
                }
            } else {
                0 as libc::c_int as libc::c_ulong
            }
        } else {
            0 as libc::c_int as libc::c_ulong
        };
        i = s;
        while *string.offset(start as isize) as libc::c_int != 0 && {
            let fresh87 = i;
            i = i - 1;
            fresh87 != 0
        } {
            if locale_mb_cur_max > 1 as libc::c_int {
                let mut state_bak: mbstate_t = mbstate_t {
                    __count: 0,
                    __value: mbstate_t_value { __wch: 0 },
                };
                let mut mblength: size_t = 0;
                let mut _f: libc::c_int = 0;
                _f = is_basic(*string.offset(start as isize));
                if _f != 0 {
                    mblength = 1 as libc::c_int as size_t;
                } else if locale_utf8locale != 0
                    && *string.offset(start as isize) as libc::c_int & 0x80 as libc::c_int
                        == 0 as libc::c_int
                {
                    mblength = (*string.offset(start as isize) as libc::c_int != 0 as libc::c_int)
                        as libc::c_int as size_t;
                } else {
                    state_bak = state;
                    mblength = mbrlen(
                        string.offset(start as isize),
                        slen.wrapping_sub(start as libc::c_ulong),
                        &mut state,
                    );
                }
                if mblength == -(2 as libc::c_int) as size_t
                    || mblength == -(1 as libc::c_int) as size_t
                {
                    state = state_bak;
                    start += 1;
                } else if mblength == 0 as libc::c_int as libc::c_ulong {
                    start += 1;
                } else {
                    start = (start as libc::c_ulong).wrapping_add(mblength) as libc::c_int
                        as libc::c_int;
                }
            } else {
                start += 1;
            }
        }
        stop = start;
        i = e - s;
        while *string.offset(stop as isize) as libc::c_int != 0 && {
            let fresh88 = i;
            i = i - 1;
            fresh88 != 0
        } {
            if locale_mb_cur_max > 1 as libc::c_int {
                let mut state_bak_0: mbstate_t = mbstate_t {
                    __count: 0,
                    __value: mbstate_t_value { __wch: 0 },
                };
                let mut mblength_0: size_t = 0;
                let mut _f_0: libc::c_int = 0;
                _f_0 = is_basic(*string.offset(stop as isize));
                if _f_0 != 0 {
                    mblength_0 = 1 as libc::c_int as size_t;
                } else if locale_utf8locale != 0
                    && *string.offset(stop as isize) as libc::c_int & 0x80 as libc::c_int
                        == 0 as libc::c_int
                {
                    mblength_0 = (*string.offset(stop as isize) as libc::c_int != 0 as libc::c_int)
                        as libc::c_int as size_t;
                } else {
                    state_bak_0 = state;
                    mblength_0 = mbrlen(
                        string.offset(stop as isize),
                        slen.wrapping_sub(stop as libc::c_ulong),
                        &mut state,
                    );
                }
                if mblength_0 == -(2 as libc::c_int) as size_t
                    || mblength_0 == -(1 as libc::c_int) as size_t
                {
                    state = state_bak_0;
                    stop += 1;
                } else if mblength_0 == 0 as libc::c_int as libc::c_ulong {
                    stop += 1;
                } else {
                    stop = (stop as libc::c_ulong).wrapping_add(mblength_0) as libc::c_int
                        as libc::c_int;
                }
            } else {
                stop += 1;
            }
        }
        tt = substring(string, start, stop);
    }
    return tt;
}
fn parameter_brace_substring(
    mut varname: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut ind: libc::c_int,
    mut substr: *mut libc::c_char,
    mut quoted: libc::c_int,
    mut pflags: libc::c_int,
    mut flags: libc::c_int,
) -> *mut libc::c_char {
    let mut e1: intmax_t = 0;
    let mut e2: intmax_t = 0;
    let mut vtype: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut starsub: libc::c_int = 0;
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut oname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    if unsafe {
        value.is_null()
            && (*varname.offset(0 as libc::c_int as isize) as libc::c_int != '@' as i32
                && *varname.offset(0 as libc::c_int as isize) as libc::c_int != '*' as i32
                || *varname.offset(1 as libc::c_int as isize) as libc::c_int != 0)
    } {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    unsafe {
        oname = this_command_name;
        this_command_name = varname;
        vtype = get_var_and_type(
            varname,
            value,
            ind as arrayind_t,
            quoted,
            flags,
            &mut v,
            &mut val,
        );
        if vtype == -(1 as libc::c_int) {
            this_command_name = oname;
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
        starsub = vtype & 128 as libc::c_int;
        vtype &= !(128 as libc::c_int);
        r = verify_substring_values(v, val, substr, vtype, &mut e1, &mut e2);
        this_command_name = oname;
        if r <= 0 as libc::c_int {
            if vtype == 0 as libc::c_int {
                if !val.is_null() {
                    sh_xfree(
                        val as *mut libc::c_void,
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        8098 as libc::c_int,
                    );
                }
            }
            return if r == 0 as libc::c_int {
                &mut expand_param_error
            } else {
                0 as *mut libc::c_void as *mut libc::c_char
            };
        }
        match vtype {
            0 | 3 => {
                if __ctype_get_mb_cur_max() > 1 as libc::c_int as usize {
                    tt = mb_substring(val, e1 as libc::c_int, e2 as libc::c_int);
                } else {
                    tt = substring(val, e1 as libc::c_int, e2 as libc::c_int);
                }
                if vtype == 0 as libc::c_int {
                    if !val.is_null() {
                        sh_xfree(
                            val as *mut libc::c_void,
                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                            8114 as libc::c_int,
                        );
                    }
                }
                if quoted & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0 {
                    temp = quote_string(tt);
                } else {
                    temp = if !tt.is_null() {
                        quote_escapes(tt)
                    } else {
                        0 as *mut libc::c_void as *mut libc::c_char
                    };
                }
                if !tt.is_null() {
                    sh_xfree(
                        tt as *mut libc::c_void,
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        8119 as libc::c_int,
                    );
                }
            }
            1 | 2 => {
                if vtype == 1 as libc::c_int {
                    tt = pos_params(
                        varname,
                        e1 as libc::c_int,
                        e2 as libc::c_int,
                        quoted,
                        pflags,
                    );
                } else if (*v).attributes & 0x40 as libc::c_int != 0 {
                    tt = assoc_subrange(
                        (*v).value as *mut HASH_TABLE,
                        e1,
                        e2,
                        starsub,
                        quoted,
                        pflags,
                    );
                } else {
                    tt = array_subrange((*v).value as *mut ARRAY, e1, e2, starsub, quoted, pflags);
                }
                if !tt.is_null() && quoted == 0 as libc::c_int && ifs_is_null != 0 {
                    temp = tt;
                } else if !tt.is_null()
                    && quoted == 0 as libc::c_int
                    && pflags & 0x8 as libc::c_int != 0
                {
                    temp = tt;
                } else if quoted & (0x1 as libc::c_int | 0x2 as libc::c_int) == 0 as libc::c_int {
                    temp = if !tt.is_null() {
                        quote_escapes(tt)
                    } else {
                        0 as *mut libc::c_void as *mut libc::c_char
                    };
                    if !tt.is_null() {
                        sh_xfree(
                            tt as *mut libc::c_void,
                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                            8152 as libc::c_int,
                        );
                    }
                } else {
                    temp = tt;
                }
            }
            _ => {
                temp = 0 as *mut libc::c_void as *mut libc::c_char;
            }
        }
    }
    return temp;
}
#[no_mangle]
pub fn pat_subst(
    mut string: *mut libc::c_char,
    mut pat: *mut libc::c_char,
    mut rep: *mut libc::c_char,
    mut mflags: libc::c_int,
) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut send: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rptr: libc::c_int = 0;
    let mut mtype: libc::c_int = 0;
    let mut rxpand: libc::c_int = 0;
    let mut mlen: libc::c_int = 0;
    let mut rsize: size_t = 0;
    let mut l: size_t = 0;
    let mut replen: size_t = 0;
    let mut rslen: size_t = 0;
    let mut state: mbstate_t = mbstate_t {
        __count: 0,
        __value: mbstate_t_value { __wch: 0 },
    };
    unsafe {
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<mbstate_t>() as usize,
        );
        if string.is_null() {
            return strcpy(
                sh_xmalloc(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(strlen(b"\0" as *const u8 as *const libc::c_char) as u64),
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    8200 as libc::c_int,
                ) as *mut libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
            );
        }
        mtype = mflags & 0x3 as libc::c_int;
        rxpand = 0 as libc::c_int;
        if (pat.is_null() || *pat as libc::c_int == 0 as libc::c_int)
            && (mtype == 0x1 as libc::c_int || mtype == 0x2 as libc::c_int)
        {
            replen = if !rep.is_null() && *rep.offset(0 as libc::c_int as isize) as libc::c_int != 0
            {
                if *rep.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                    if *rep.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                        strlen(rep) as u64
                    } else {
                        2 as libc::c_int as libc::c_ulong
                    }
                } else {
                    1 as libc::c_int as libc::c_ulong
                }
            } else {
                0 as libc::c_int as libc::c_ulong
            };
            l = if !string.is_null()
                && *string.offset(0 as libc::c_int as isize) as libc::c_int != 0
            {
                if *string.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                    if *string.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                        strlen(string) as u64
                    } else {
                        2 as libc::c_int as libc::c_ulong
                    }
                } else {
                    1 as libc::c_int as libc::c_ulong
                }
            } else {
                0 as libc::c_int as libc::c_ulong
            };
            ret = sh_xmalloc(
                replen
                    .wrapping_add(l)
                    .wrapping_add(2 as libc::c_int as libc::c_ulong),
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                8223 as libc::c_int,
            ) as *mut libc::c_char;
            if replen == 0 as libc::c_int as libc::c_ulong {
                strcpy(ret, string);
            } else if mtype == 0x1 as libc::c_int {
                strcpy(ret, rep);
                strcpy(ret.offset(replen as isize), string);
            } else {
                strcpy(ret, string);
                strcpy(ret.offset(l as isize), rep);
            }
            return ret;
        } else {
            if *string as libc::c_int == 0 as libc::c_int
                && match_pattern(string, pat, mtype, &mut s, &mut e) != 0 as libc::c_int
            {
                replen = if !rep.is_null()
                    && *rep.offset(0 as libc::c_int as isize) as libc::c_int != 0
                {
                    if *rep.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                        if *rep.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                            strlen(rep) as u64
                        } else {
                            2 as libc::c_int as libc::c_ulong
                        }
                    } else {
                        1 as libc::c_int as libc::c_ulong
                    }
                } else {
                    0 as libc::c_int as libc::c_ulong
                };
                ret = sh_xmalloc(
                    replen.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    8241 as libc::c_int,
                ) as *mut libc::c_char;
                if replen == 0 as libc::c_int as libc::c_ulong {
                    *ret.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
                } else {
                    strcpy(ret, rep);
                }
                return ret;
            }
        }
        rsize = 64 as libc::c_int as size_t;
        ret = sh_xmalloc(
            rsize,
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            8249 as libc::c_int,
        ) as *mut libc::c_char;
        *ret.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
        send = string.offset(strlen(string) as isize);
        replen = if !rep.is_null() && *rep.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            if *rep.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                if *rep.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                    strlen(rep) as u64
                } else {
                    2 as libc::c_int as libc::c_ulong
                }
            } else {
                1 as libc::c_int as libc::c_ulong
            }
        } else {
            0 as libc::c_int as libc::c_ulong
        };
        rptr = 0 as libc::c_int;
        str = string;
        while *str != 0 {
            if match_pattern(str, pat, mtype, &mut s, &mut e) == 0 as libc::c_int {
                break;
            }
            l = s.offset_from(str) as libc::c_long as size_t;
            if !rep.is_null() && rxpand != 0 {
                let mut x: libc::c_int = 0;
                mlen = e.offset_from(s) as libc::c_long as libc::c_int;
                mstr = sh_xmalloc(
                    (mlen + 1 as libc::c_int) as size_t,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    8263 as libc::c_int,
                ) as *mut libc::c_char;
                x = 0 as libc::c_int;
                while x < mlen {
                    *mstr.offset(x as isize) = *s.offset(x as isize);
                    x += 1;
                }
                *mstr.offset(mlen as isize) = '\u{0}' as i32 as libc::c_char;
                rstr = strcreplace(rep, '&' as i32, mstr, 0 as libc::c_int);
                sh_xfree(
                    mstr as *mut libc::c_void,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    8268 as libc::c_int,
                );
                rslen = strlen(rstr) as u64;
            } else {
                rstr = rep;
                rslen = replen;
            }
            if (rptr as libc::c_ulong).wrapping_add(l.wrapping_add(rslen)) >= rsize {
                while (rptr as libc::c_ulong).wrapping_add(l.wrapping_add(rslen)) >= rsize {
                    rsize = (rsize as libc::c_ulong)
                        .wrapping_add(64 as libc::c_int as libc::c_ulong)
                        as size_t as size_t;
                }
                ret = sh_xrealloc(
                    ret as *mut libc::c_void,
                    rsize,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    8277 as libc::c_int,
                ) as *mut libc::c_char;
            }
            if l != 0 {
                strncpy(ret.offset(rptr as isize), str, l as usize);
                rptr = (rptr as libc::c_ulong).wrapping_add(l) as libc::c_int as libc::c_int;
            }
            if replen != 0 {
                strncpy(ret.offset(rptr as isize), rstr, rslen as usize);
                rptr = (rptr as libc::c_ulong).wrapping_add(rslen) as libc::c_int as libc::c_int;
            }
            str = e;
            if rstr != rep {
                sh_xfree(
                    rstr as *mut libc::c_void,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    8296 as libc::c_int,
                );
            }
            if mflags & 0x10 as libc::c_int == 0 as libc::c_int || mtype != 0 as libc::c_int {
                break;
            }
            if s == e {
                let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut origp: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut origs: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut clen: size_t = 0;
                if (rptr + locale_mb_cur_max) as libc::c_ulong >= rsize {
                    while (rptr + locale_mb_cur_max) as libc::c_ulong >= rsize {
                        rsize = (rsize as libc::c_ulong)
                            .wrapping_add(64 as libc::c_int as libc::c_ulong)
                            as size_t as size_t;
                    }
                    ret = sh_xrealloc(
                        ret as *mut libc::c_void,
                        rsize,
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        8308 as libc::c_int,
                    ) as *mut libc::c_char;
                }
                origp = ret.offset(rptr as isize);
                p = origp;
                origs = str;
                if locale_mb_cur_max > 1 as libc::c_int {
                    let mut state_bak: mbstate_t = mbstate_t {
                        __count: 0,
                        __value: mbstate_t_value { __wch: 0 },
                    };
                    let mut mblength: size_t = 0;
                    let mut _k: libc::c_int = 0;
                    _k = is_basic(*str);
                    if _k != 0 {
                        mblength = 1 as libc::c_int as size_t;
                    } else if locale_utf8locale != 0
                        && *str as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                    {
                        mblength =
                            (*str as libc::c_int != 0 as libc::c_int) as libc::c_int as size_t;
                    } else {
                        state_bak = state;
                        mblength = mbrlen(
                            str,
                            send.offset_from(str) as libc::c_long as size_t,
                            &mut state,
                        );
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
                        let fresh89 = str;
                        str = str.offset(1);
                        let fresh90 = p;
                        p = p.offset(1);
                        *fresh90 = *fresh89;
                        _k += 1;
                    }
                } else {
                    let fresh91 = str;
                    str = str.offset(1);
                    let fresh92 = p;
                    p = p.offset(1);
                    *fresh92 = *fresh91;
                }
                rptr = (rptr as libc::c_long + p.offset_from(origp) as libc::c_long) as libc::c_int;
                e = e.offset(str.offset_from(origs) as libc::c_long as isize);
            }
        }
        if !str.is_null() && *str as libc::c_int != 0 {
            if (rptr as libc::c_ulong).wrapping_add(
                (if !str.is_null() && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
                    if *str.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                        if *str.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                            strlen(str) as usize
                        } else {
                            2 as libc::c_int as usize
                        }
                    } else {
                        1 as libc::c_int as usize
                    }
                } else {
                    0 as libc::c_int as usize
                })
                .wrapping_add(1 as libc::c_int as usize) as u64,
            ) >= rsize
            {
                while (rptr as libc::c_ulong).wrapping_add(
                    (if !str.is_null() && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0
                    {
                        if *str.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                            if *str.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                                strlen(str) as usize
                            } else {
                                2 as libc::c_int as usize
                            }
                        } else {
                            1 as libc::c_int as usize
                        }
                    } else {
                        0 as libc::c_int as usize
                    })
                    .wrapping_add(1 as libc::c_int as usize) as u64,
                ) >= rsize
                {
                    rsize = (rsize as libc::c_ulong)
                        .wrapping_add(64 as libc::c_int as libc::c_ulong)
                        as size_t as size_t;
                }
                ret = sh_xrealloc(
                    ret as *mut libc::c_void,
                    rsize,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    8325 as libc::c_int,
                ) as *mut libc::c_char;
            }
            strcpy(ret.offset(rptr as isize), str);
        } else {
            *ret.offset(rptr as isize) = '\u{0}' as i32 as libc::c_char;
        }
    }
    return ret;
}
fn pos_params_pat_subst(
    mut string: *mut libc::c_char,
    mut pat: *mut libc::c_char,
    mut rep: *mut libc::c_char,
    mut mflags: libc::c_int,
) -> *mut libc::c_char {
    let mut save: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut params: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut w: *mut WORD_DESC = 0 as *mut WORD_DESC;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pchar: libc::c_int = 0;
    let mut qflags: libc::c_int = 0;
    let mut pflags: libc::c_int = 0;
    params = list_rest_of_args();
    save = params;
    unsafe {
        if save.is_null() {
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
        while !params.is_null() {
            ret = pat_subst((*(*params).word).word, pat, rep, mflags);
            w = alloc_word_desc();
            let ref mut fresh93 = (*w).word;
            *fresh93 = if !ret.is_null() {
                ret
            } else {
                strcpy(
                    sh_xmalloc(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_add(strlen(b"\0" as *const u8 as *const libc::c_char) as u64),
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        8353 as libc::c_int,
                    ) as *mut libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                )
            };
            dispose_word((*params).word);
            let ref mut fresh94 = (*params).word;
            *fresh94 = w;
            params = (*params).next;
        }
        pchar = if mflags & 0x80 as libc::c_int == 0x80 as libc::c_int {
            '*' as i32
        } else {
            '@' as i32
        };
        qflags = if mflags & 0x20 as libc::c_int == 0x20 as libc::c_int {
            0x1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        pflags = if mflags & 0x40 as libc::c_int == 0x40 as libc::c_int {
            0x8 as libc::c_int
        } else {
            0 as libc::c_int
        };
        if pchar == '*' as i32
            && mflags & 0x40 as libc::c_int != 0
            && expand_no_split_dollar_star != 0
            && ifs_is_null != 0
        {
            qflags |= 0x1 as libc::c_int;
        }
        ret = string_list_pos_params(pchar, save, qflags, pflags);
        dispose_words(save);
    }
    return ret;
}
fn parameter_brace_patsub(
    mut varname: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut ind: libc::c_int,
    mut patsub: *mut libc::c_char,
    mut quoted: libc::c_int,
    mut pflags: libc::c_int,
    mut flags: libc::c_int,
) -> *mut libc::c_char {
    let mut vtype: libc::c_int = 0;
    let mut mflags: libc::c_int = 0;
    let mut starsub: libc::c_int = 0;
    let mut delim: libc::c_int = 0;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pat: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lpatsub: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut oname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    if value.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    unsafe {
        oname = this_command_name;
        this_command_name = varname;
        vtype = get_var_and_type(
            varname,
            value,
            ind as arrayind_t,
            quoted,
            flags,
            &mut v,
            &mut val,
        );
        if vtype == -(1 as libc::c_int) {
            this_command_name = oname;
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
        starsub = vtype & 128 as libc::c_int;
        vtype &= !(128 as libc::c_int);
        mflags = 0 as libc::c_int;
        if *patsub as libc::c_int == '/' as i32 {
            mflags |= 0x10 as libc::c_int;
            patsub = patsub.offset(1);
        }
        lpatsub = strcpy(
            sh_xmalloc(
                (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(patsub) as u64),
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                8414 as libc::c_int,
            ) as *mut libc::c_char,
            patsub,
        );
        if quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0 {
            mflags |= 0x20 as libc::c_int;
        }
        if starsub != 0 {
            mflags |= 0x80 as libc::c_int;
        }
        if pflags & 0x8 as libc::c_int != 0 {
            mflags |= 0x40 as libc::c_int;
        }
        delim = skip_to_delim(
            lpatsub,
            if *patsub as libc::c_int == '/' as i32 {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            },
            b"/\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int,
        );
        if *lpatsub.offset(delim as isize) as libc::c_int == '/' as i32 {
            *lpatsub.offset(delim as isize) = 0 as libc::c_int as libc::c_char;
            rep = lpatsub
                .offset(delim as isize)
                .offset(1 as libc::c_int as isize);
        } else {
            rep = 0 as *mut libc::c_void as *mut libc::c_char;
        }
        if !rep.is_null() && *rep as libc::c_int == '\u{0}' as i32 {
            rep = 0 as *mut libc::c_void as *mut libc::c_char;
        }
        pat = getpattern(lpatsub, quoted, 1 as libc::c_int);
        if !rep.is_null() {
            if shell_compatibility_level > 42 as libc::c_int {
                rep = expand_string_if_necessary(
                    rep,
                    quoted & !(0x1 as libc::c_int | 0x2 as libc::c_int),
                    ::std::mem::transmute::<Option<fn() -> *mut WORD_LIST>, Option<EXPFUNC>>(Some(
                        ::std::mem::transmute::<
                            fn(*mut libc::c_char, libc::c_int) -> *mut WORD_LIST,
                            fn() -> *mut WORD_LIST,
                        >(expand_string_unsplit),
                    )),
                );
            } else if mflags & 0x20 as libc::c_int == 0 as libc::c_int {
                rep = expand_string_if_necessary(
                    rep,
                    quoted,
                    ::std::mem::transmute::<Option<fn() -> *mut WORD_LIST>, Option<EXPFUNC>>(Some(
                        ::std::mem::transmute::<
                            fn(*mut libc::c_char, libc::c_int) -> *mut WORD_LIST,
                            fn() -> *mut WORD_LIST,
                        >(expand_string_unsplit),
                    )),
                );
            } else {
                rep = expand_string_to_string_internal(
                    rep,
                    quoted,
                    ::std::mem::transmute::<Option<fn() -> *mut WORD_LIST>, Option<EXPFUNC>>(Some(
                        ::std::mem::transmute::<
                            fn(*mut libc::c_char, libc::c_int) -> *mut WORD_LIST,
                            fn() -> *mut WORD_LIST,
                        >(expand_string_unsplit),
                    )),
                );
            }
        }
        p = pat;
        if mflags & 0x10 as libc::c_int != 0 {
            mflags |= 0 as libc::c_int;
        } else if !pat.is_null()
            && *pat.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
        {
            mflags |= 0x1 as libc::c_int;
            p = p.offset(1);
        } else if !pat.is_null()
            && *pat.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32
        {
            mflags |= 0x2 as libc::c_int;
            p = p.offset(1);
        } else {
            mflags |= 0 as libc::c_int;
        }
        match vtype {
            0 | 3 => {
                temp = pat_subst(val, p, rep, mflags);
                if vtype == 0 as libc::c_int {
                    if !val.is_null() {
                        sh_xfree(
                            val as *mut libc::c_void,
                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                            8495 as libc::c_int,
                        );
                    }
                }
                if !temp.is_null() {
                    tt = if mflags & 0x20 as libc::c_int != 0 {
                        quote_string(temp)
                    } else {
                        quote_escapes(temp)
                    };
                    sh_xfree(
                        temp as *mut libc::c_void,
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        8499 as libc::c_int,
                    );
                    temp = tt;
                }
            }
            1 => {
                if pflags & 0x4 as libc::c_int != 0 && mflags & 0x80 as libc::c_int != 0 {
                    mflags |= 0x40 as libc::c_int;
                }
                temp = pos_params_pat_subst(val, p, rep, mflags);
                if !(!temp.is_null() && quoted == 0 as libc::c_int && ifs_is_null != 0) {
                    if !(!temp.is_null()
                        && quoted == 0 as libc::c_int
                        && pflags & 0x8 as libc::c_int != 0)
                    {
                        if !temp.is_null() && mflags & 0x20 as libc::c_int == 0 as libc::c_int {
                            tt = quote_escapes(temp);
                            sh_xfree(
                                temp as *mut libc::c_void,
                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                8522 as libc::c_int,
                            );
                            temp = tt;
                        }
                    }
                }
            }
            2 => {
                if mflags & 0x80 as libc::c_int != 0
                    && mflags & 0x40 as libc::c_int != 0
                    && ifs_is_null != 0
                {
                    mflags |= 0x20 as libc::c_int;
                }
                if (*v).attributes & 0x40 as libc::c_int != 0 {
                    temp = assoc_patsub((*v).value as *mut HASH_TABLE, p, rep, mflags);
                } else {
                    temp = array_patsub((*v).value as *mut ARRAY, p, rep, mflags);
                }
                if !(!temp.is_null() && quoted == 0 as libc::c_int && ifs_is_null != 0) {
                    if !temp.is_null() && mflags & 0x20 as libc::c_int == 0 as libc::c_int {
                        tt = quote_escapes(temp);
                        sh_xfree(
                            temp as *mut libc::c_void,
                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                            8547 as libc::c_int,
                        );
                        temp = tt;
                    }
                }
            }
            _ => {}
        }
        if !pat.is_null() {
            sh_xfree(
                pat as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                8554 as libc::c_int,
            );
        }
        if !rep.is_null() {
            sh_xfree(
                rep as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                8555 as libc::c_int,
            );
        }
        sh_xfree(
            lpatsub as *mut libc::c_void,
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            8556 as libc::c_int,
        );
        this_command_name = oname;
    }
    return temp;
}
fn pos_params_modcase(
    mut string: *mut libc::c_char,
    mut pat: *mut libc::c_char,
    mut modop: libc::c_int,
    mut mflags: libc::c_int,
) -> *mut libc::c_char {
    let mut save: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut params: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut w: *mut WORD_DESC = 0 as *mut WORD_DESC;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pchar: libc::c_int = 0;
    let mut qflags: libc::c_int = 0;
    let mut pflags: libc::c_int = 0;
    params = list_rest_of_args();
    save = params;
    if save.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    unsafe {
        while !params.is_null() {
            ret = sh_modcase((*(*params).word).word, pat, modop);
            w = alloc_word_desc();
            let ref mut fresh95 = (*w).word;
            *fresh95 = if !ret.is_null() {
                ret
            } else {
                strcpy(
                    sh_xmalloc(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_add(strlen(b"\0" as *const u8 as *const libc::c_char) as u64),
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        8590 as libc::c_int,
                    ) as *mut libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                )
            };
            dispose_word((*params).word);
            let ref mut fresh96 = (*params).word;
            *fresh96 = w;
            params = (*params).next;
        }
        pchar = if mflags & 0x80 as libc::c_int == 0x80 as libc::c_int {
            '*' as i32
        } else {
            '@' as i32
        };
        qflags = if mflags & 0x20 as libc::c_int == 0x20 as libc::c_int {
            0x1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        pflags = if mflags & 0x40 as libc::c_int == 0x40 as libc::c_int {
            0x8 as libc::c_int
        } else {
            0 as libc::c_int
        };
        if pchar == '*' as i32 && mflags & 0x40 as libc::c_int != 0 && ifs_is_null != 0 {
            qflags |= 0x1 as libc::c_int;
        }
        ret = string_list_pos_params(pchar, save, qflags, pflags);
        dispose_words(save);
    }
    return ret;
}
fn parameter_brace_casemod(
    mut varname: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut ind: libc::c_int,
    mut modspec: libc::c_int,
    mut patspec: *mut libc::c_char,
    mut quoted: libc::c_int,
    mut pflags: libc::c_int,
    mut flags: libc::c_int,
) -> *mut libc::c_char {
    let mut vtype: libc::c_int = 0;
    let mut starsub: libc::c_int = 0;
    let mut modop: libc::c_int = 0;
    let mut mflags: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pat: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lpat: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut oname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    if value.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    unsafe {
        oname = this_command_name;
        this_command_name = varname;
        vtype = get_var_and_type(
            varname,
            value,
            ind as arrayind_t,
            quoted,
            flags,
            &mut v,
            &mut val,
        );
        if vtype == -(1 as libc::c_int) {
            this_command_name = oname;
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
        starsub = vtype & 128 as libc::c_int;
        vtype &= !(128 as libc::c_int);
        modop = 0 as libc::c_int;
        mflags = 0 as libc::c_int;
        if quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0 {
            mflags |= 0x20 as libc::c_int;
        }
        if starsub != 0 {
            mflags |= 0x80 as libc::c_int;
        }
        if pflags & 0x8 as libc::c_int != 0 {
            mflags |= 0x40 as libc::c_int;
        }
        p = patspec;
        if modspec == '^' as i32 {
            x = (!p.is_null() && *p.offset(0 as libc::c_int as isize) as libc::c_int == modspec)
                as libc::c_int;
            modop = if x != 0 {
                0x2 as libc::c_int
            } else {
                0x40 as libc::c_int
            };
            p = p.offset(x as isize);
        } else if modspec == ',' as i32 {
            x = (!p.is_null() && *p.offset(0 as libc::c_int as isize) as libc::c_int == modspec)
                as libc::c_int;
            modop = if x != 0 {
                0x1 as libc::c_int
            } else {
                0x80 as libc::c_int
            };
            p = p.offset(x as isize);
        } else if modspec == '~' as i32 {
            x = (!p.is_null() && *p.offset(0 as libc::c_int as isize) as libc::c_int == modspec)
                as libc::c_int;
            modop = if x != 0 {
                0x20 as libc::c_int
            } else {
                0x10 as libc::c_int
            };
            p = p.offset(x as isize);
        }
        lpat = if !p.is_null() {
            strcpy(
                sh_xmalloc(
                    (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(p) as u64),
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    8670 as libc::c_int,
                ) as *mut libc::c_char,
                p,
            )
        } else {
            0 as *mut libc::c_char
        };
        pat = if !lpat.is_null() {
            getpattern(lpat, quoted, 1 as libc::c_int)
        } else {
            0 as *mut libc::c_char
        };
        match vtype {
            0 | 3 => {
                temp = sh_modcase(val, pat, modop);
                if vtype == 0 as libc::c_int {
                    if !val.is_null() {
                        sh_xfree(
                            val as *mut libc::c_void,
                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                            8682 as libc::c_int,
                        );
                    }
                }
                if !temp.is_null() {
                    tt = if mflags & 0x20 as libc::c_int != 0 {
                        quote_string(temp)
                    } else {
                        quote_escapes(temp)
                    };
                    sh_xfree(
                        temp as *mut libc::c_void,
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        8686 as libc::c_int,
                    );
                    temp = tt;
                }
            }
            1 => {
                temp = pos_params_modcase(val, pat, modop, mflags);
                if !(!temp.is_null() && quoted == 0 as libc::c_int && ifs_is_null != 0) {
                    if !temp.is_null() && mflags & 0x20 as libc::c_int == 0 as libc::c_int {
                        tt = quote_escapes(temp);
                        sh_xfree(
                            temp as *mut libc::c_void,
                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                            8700 as libc::c_int,
                        );
                        temp = tt;
                    }
                }
            }
            2 => {
                if mflags & 0x80 as libc::c_int != 0
                    && mflags & 0x40 as libc::c_int != 0
                    && ifs_is_null != 0
                {
                    mflags |= 0x20 as libc::c_int;
                }
                temp = if (*v).attributes & 0x40 as libc::c_int != 0 {
                    assoc_modcase((*v).value as *mut HASH_TABLE, pat, modop, mflags)
                } else {
                    array_modcase((*v).value as *mut ARRAY, pat, modop, mflags)
                };
                if !(!temp.is_null() && quoted == 0 as libc::c_int && ifs_is_null != 0) {
                    if !temp.is_null() && mflags & 0x20 as libc::c_int == 0 as libc::c_int {
                        tt = quote_escapes(temp);
                        sh_xfree(
                            temp as *mut libc::c_void,
                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                            8723 as libc::c_int,
                        );
                        temp = tt;
                    }
                }
            }
            _ => {}
        }
        if !pat.is_null() {
            sh_xfree(
                pat as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                8731 as libc::c_int,
            );
        }
        sh_xfree(
            lpat as *mut libc::c_void,
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            8732 as libc::c_int,
        );
        this_command_name = oname;
    }
    return temp;
}
fn chk_arithsub(mut s: *const libc::c_char, mut len: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut state: mbstate_t = mbstate_t {
        __count: 0,
        __value: mbstate_t_value { __wch: 0 },
    };
    unsafe {
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<mbstate_t>() as usize,
        );
        count = 0 as libc::c_int;
        i = count;
        while i < len {
            if *s.offset(i as isize) as libc::c_int == '(' as i32 {
                count += 1;
            } else if *s.offset(i as isize) as libc::c_int == ')' as i32 {
                count -= 1;
                if count < 0 as libc::c_int {
                    return 0 as libc::c_int;
                }
            }
            match *s.offset(i as isize) as libc::c_int {
                92 => {
                    i += 1;
                    if *s.offset(i as isize) != 0 {
                        if locale_mb_cur_max > 1 as libc::c_int {
                            let mut state_bak_0: mbstate_t = mbstate_t {
                                __count: 0,
                                __value: mbstate_t_value { __wch: 0 },
                            };
                            let mut mblength_0: size_t = 0;
                            let mut _f_0: libc::c_int = 0;
                            _f_0 = is_basic(*s.offset(i as isize));
                            if _f_0 != 0 {
                                mblength_0 = 1 as libc::c_int as size_t;
                            } else if locale_utf8locale != 0
                                && *s.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                                    == 0 as libc::c_int
                            {
                                mblength_0 = (*s.offset(i as isize) as libc::c_int
                                    != 0 as libc::c_int)
                                    as libc::c_int
                                    as size_t;
                            } else {
                                state_bak_0 = state;
                                mblength_0 =
                                    mbrlen(s.offset(i as isize), (len - i) as size_t, &mut state);
                            }
                            if mblength_0 == -(2 as libc::c_int) as size_t
                                || mblength_0 == -(1 as libc::c_int) as size_t
                            {
                                state = state_bak_0;
                                i += 1;
                            } else if mblength_0 == 0 as libc::c_int as libc::c_ulong {
                                i += 1;
                            } else {
                                i = (i as libc::c_ulong).wrapping_add(mblength_0) as libc::c_int
                                    as libc::c_int;
                            }
                        } else {
                            i += 1;
                        }
                    }
                }
                39 => {
                    i += 1;
                    i = skip_single_quoted(s, len as size_t, i, 0 as libc::c_int);
                }
                34 => {
                    i += 1;
                    i = skip_double_quoted(
                        s as *mut libc::c_char,
                        len as size_t,
                        i,
                        0 as libc::c_int,
                    );
                }
                _ => {
                    if locale_mb_cur_max > 1 as libc::c_int {
                        let mut state_bak: mbstate_t = mbstate_t {
                            __count: 0,
                            __value: mbstate_t_value { __wch: 0 },
                        };
                        let mut mblength: size_t = 0;
                        let mut _f: libc::c_int = 0;
                        _f = is_basic(*s.offset(i as isize));
                        if _f != 0 {
                            mblength = 1 as libc::c_int as size_t;
                        } else if locale_utf8locale != 0
                            && *s.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                                == 0 as libc::c_int
                        {
                            mblength = (*s.offset(i as isize) as libc::c_int != 0 as libc::c_int)
                                as libc::c_int as size_t;
                        } else {
                            state_bak = state;
                            mblength =
                                mbrlen(s.offset(i as isize), (len - i) as size_t, &mut state);
                        }
                        if mblength == -(2 as libc::c_int) as size_t
                            || mblength == -(1 as libc::c_int) as size_t
                        {
                            state = state_bak;
                            i += 1;
                        } else if mblength == 0 as libc::c_int as libc::c_ulong {
                            i += 1;
                        } else {
                            i = (i as libc::c_ulong).wrapping_add(mblength) as libc::c_int
                                as libc::c_int;
                        }
                    } else {
                        i += 1;
                    }
                }
            }
        }
    }
    return (count == 0 as libc::c_int) as libc::c_int;
}
fn parameter_brace_expand(
    mut string: *mut libc::c_char,
    mut indexp: *mut libc::c_int,
    mut quoted: libc::c_int,
    mut pflags: libc::c_int,
    mut quoted_dollar_atp: *mut libc::c_int,
    mut contains_dollar_at: *mut libc::c_int,
) -> *mut WORD_DESC {
    let mut current_block: u64;
    let mut check_nullness: libc::c_int = 0;
    let mut var_is_set: libc::c_int = 0;
    let mut var_is_null: libc::c_int = 0;
    let mut var_is_special: libc::c_int = 0;
    let mut want_substring: libc::c_int = 0;
    let mut want_indir: libc::c_int = 0;
    let mut want_patsub: libc::c_int = 0;
    let mut want_casemod: libc::c_int = 0;
    let mut want_attributes: libc::c_int = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tdesc: *mut WORD_DESC = 0 as *mut WORD_DESC;
    let mut ret: *mut WORD_DESC = 0 as *mut WORD_DESC;
    let mut t_index: libc::c_int = 0;
    let mut sindex: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut tflag: libc::c_int = 0;
    let mut modspec: libc::c_int = 0;
    let mut local_pflags: libc::c_int = 0;
    let mut all_element_arrayref: libc::c_int = 0;
    let mut number: intmax_t = 0;
    let mut ind: arrayind_t = 0;
    value = 0 as *mut libc::c_void as *mut libc::c_char;
    temp1 = value;
    temp = temp1;
    check_nullness = 0 as libc::c_int;
    var_is_special = check_nullness;
    var_is_null = var_is_special;
    var_is_set = var_is_null;
    want_attributes = 0 as libc::c_int;
    want_casemod = want_attributes;
    want_patsub = want_casemod;
    want_indir = want_patsub;
    want_substring = want_indir;
    local_pflags = 0 as libc::c_int;
    all_element_arrayref = 0 as libc::c_int;
    unsafe {
        sindex = *indexp;
        sindex += 1;
        t_index = sindex;
        if *string.offset(t_index as isize) as libc::c_int == '#' as i32
            && (1 as libc::c_int != 0
                && *(*__ctype_b_loc()).offset(*string.offset((t_index + 1 as libc::c_int) as isize)
                    as libc::c_uchar as libc::c_int
                    as isize) as libc::c_int
                    & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                    != 0
                || *string.offset((t_index + 1 as libc::c_int) as isize) as libc::c_int
                    == '_' as i32)
        {
            name = string_extract(
                string,
                &mut t_index,
                b"}\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0x2 as libc::c_int,
            );
        } else {
            name = string_extract(
                string,
                &mut t_index,
                b"#%^,~:-=?+/@}\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0x2 as libc::c_int,
            );
        }
        if *name as libc::c_int == 0 as libc::c_int
            && sindex == t_index
            && *string.offset(sindex as isize) as libc::c_int == '@' as i32
        {
            name = sh_xrealloc(
                name as *mut libc::c_void,
                2 as libc::c_int as size_t,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                8839 as libc::c_int,
            ) as *mut libc::c_char;
            *name.offset(0 as libc::c_int as isize) = '@' as i32 as libc::c_char;
            *name.offset(1 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
            t_index += 1;
        } else if *name as libc::c_int == '!' as i32
            && t_index > sindex
            && *string.offset(t_index as isize) as libc::c_int == '@' as i32
            && *string.offset((t_index + 1 as libc::c_int) as isize) as libc::c_int == '}' as i32
        {
            name = sh_xrealloc(
                name as *mut libc::c_void,
                (t_index - sindex + 2 as libc::c_int) as size_t,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                8846 as libc::c_int,
            ) as *mut libc::c_char;
            *name.offset((t_index - sindex) as isize) = '@' as i32 as libc::c_char;
            *name.offset((t_index - sindex + 1 as libc::c_int) as isize) =
                '\u{0}' as i32 as libc::c_char;
            t_index += 1;
        }
        ret = 0 as *mut WORD_DESC;
        tflag = 0 as libc::c_int;
        ind = -(9223372036854775807 as libc::c_long) - 1 as libc::c_int as libc::c_long;
        if sindex == t_index
            && (*string.offset(t_index as isize) as libc::c_int == '-' as i32
                || *string.offset(t_index as isize) as libc::c_int == '?' as i32
                || *string.offset(t_index as isize) as libc::c_int == '#' as i32
                || *string.offset(t_index as isize) as libc::c_int == '@' as i32)
            || sindex == t_index
                && *string.offset(sindex as isize) as libc::c_int == '#' as i32
                && (*string.offset((sindex + 1 as libc::c_int) as isize) as libc::c_int
                    == '-' as i32
                    || *string.offset((sindex + 1 as libc::c_int) as isize) as libc::c_int
                        == '?' as i32
                    || *string.offset((sindex + 1 as libc::c_int) as isize) as libc::c_int
                        == '#' as i32
                    || *string.offset((sindex + 1 as libc::c_int) as isize) as libc::c_int
                        == '@' as i32)
            || sindex == t_index - 1 as libc::c_int
                && *string.offset(sindex as isize) as libc::c_int == '!' as i32
                && (posixly_correct == 0 as libc::c_int
                    && *string.offset(t_index as isize) as libc::c_int == '#' as i32
                    || posixly_correct == 0 as libc::c_int
                        && *string.offset(t_index as isize) as libc::c_int == '?' as i32
                    || *string.offset(t_index as isize) as libc::c_int == '@' as i32
                    || *string.offset(t_index as isize) as libc::c_int == '*' as i32)
        {
            t_index += 1;
            temp1 = string_extract(
                string,
                &mut t_index,
                b"#%:-=?+/@}\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
            );
            name = sh_xrealloc(
                name as *mut libc::c_void,
                (3 as libc::c_int as libc::c_ulong).wrapping_add(strlen(temp1) as u64),
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                8868 as libc::c_int,
            ) as *mut libc::c_char;
            *name = *string.offset(sindex as isize);
            if *string.offset(sindex as isize) as libc::c_int == '!' as i32 {
                *name.offset(1 as libc::c_int as isize) =
                    *string.offset((sindex + 1 as libc::c_int) as isize);
                strcpy(name.offset(2 as libc::c_int as isize), temp1);
            } else {
                strcpy(name.offset(1 as libc::c_int as isize), temp1);
            }
            sh_xfree(
                temp1 as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                8878 as libc::c_int,
            );
        }
        sindex = t_index;
        c = *string.offset(sindex as isize) as libc::c_int;
        if c != 0 {
            sindex += 1;
        }
        if c == ':' as i32
            && *sh_syntaxtab
                .as_mut_ptr()
                .offset(*string.offset(sindex as isize) as libc::c_uchar as isize)
                & 0x1000 as libc::c_int
                != 0
        {
            check_nullness += 1;
            c = *string.offset(sindex as isize) as libc::c_int;
            if c != 0 {
                sindex += 1;
            }
        } else if c == ':' as i32 && *string.offset(sindex as isize) as libc::c_int != '}' as i32 {
            want_substring = 1 as libc::c_int;
        } else if c == '/' as i32 {
            want_patsub = 1 as libc::c_int;
        } else if c == '^' as i32 || c == ',' as i32 || c == '~' as i32 {
            modspec = c;
            want_casemod = 1 as libc::c_int;
        } else if c == '@' as i32
            && (*string.offset(sindex as isize) as libc::c_int == 'a' as i32
                || *string.offset(sindex as isize) as libc::c_int == 'A' as i32)
            && *string.offset((sindex + 1 as libc::c_int) as isize) as libc::c_int == '}' as i32
        {
            want_attributes = 1 as libc::c_int;
            local_pflags |= 0x40 as libc::c_int;
        }
        if *name.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
            && *name.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
            && check_nullness == 0 as libc::c_int
            && (c == '-' as i32 || c == '?' as i32 || c == '#' as i32 || c == '@' as i32)
            && *string.offset(sindex as isize) as libc::c_int == '}' as i32
        {
            name = sh_xrealloc(
                name as *mut libc::c_void,
                3 as libc::c_int as size_t,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                8922 as libc::c_int,
            ) as *mut libc::c_char;
            *name.offset(1 as libc::c_int as isize) = c as libc::c_char;
            *name.offset(2 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
            let fresh97 = sindex;
            sindex = sindex + 1;
            c = *string.offset(fresh97 as isize) as libc::c_int;
        }
        if *name.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
            && *name.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
            && check_nullness == 0 as libc::c_int
            && (if c != 0 {
                (mbschr(b"%:=+/\0" as *const u8 as *const libc::c_char, c)
                    != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int
            } else {
                0 as libc::c_int
            }) != 0
            && *string.offset(sindex as isize) as libc::c_int == '}' as i32
        {
            temp = 0 as *mut libc::c_void as *mut libc::c_char;
        } else {
            want_indir = (*name as libc::c_int == '!' as i32
                && (1 as libc::c_int != 0
                    && *(*__ctype_b_loc())
                        .offset(*name.offset(1 as libc::c_int as isize) as libc::c_uchar
                            as libc::c_int as isize) as libc::c_int
                        & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                        != 0
                    || *name.offset(1 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                        == '_' as i32
                    || *name.offset(1 as libc::c_int as isize) as libc::c_int >= '0' as i32
                        && *name.offset(1 as libc::c_int as isize) as libc::c_int <= '9' as i32
                    || (posixly_correct == 0 as libc::c_int
                        && *name.offset(1 as libc::c_int as isize) as libc::c_int == '#' as i32
                        || posixly_correct == 0 as libc::c_int
                            && *name.offset(1 as libc::c_int as isize) as libc::c_int
                                == '?' as i32
                        || *name.offset(1 as libc::c_int as isize) as libc::c_int == '@' as i32
                        || *name.offset(1 as libc::c_int as isize) as libc::c_int == '*' as i32)))
                as libc::c_int;
            if *name as libc::c_int != 0
                && (*name as libc::c_int >= '0' as i32
                    && *name as libc::c_int <= '9' as i32
                    && all_digits(name) != 0
                    || *name.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
                        && *sh_syntaxtab
                            .as_mut_ptr()
                            .offset(*name as libc::c_uchar as isize)
                            & 0x800 as libc::c_int
                            != 0
                    || want_indir != 0
                        && *name.offset(2 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
                        && (posixly_correct == 0 as libc::c_int
                            && *name.offset(1 as libc::c_int as isize) as libc::c_int
                                == '#' as i32
                            || posixly_correct == 0 as libc::c_int
                                && *name.offset(1 as libc::c_int as isize) as libc::c_int
                                    == '?' as i32
                            || *name.offset(1 as libc::c_int as isize) as libc::c_int
                                == '@' as i32
                            || *name.offset(1 as libc::c_int as isize) as libc::c_int
                                == '*' as i32))
            {
                var_is_special += 1;
            }
            if *name as libc::c_int == '#' as i32
                && *name.offset(1 as libc::c_int as isize) as libc::c_int != 0
            {
                if *string.offset((sindex - 1 as libc::c_int) as isize) as libc::c_int != '}' as i32
                    || valid_length_expression(name) == 0 as libc::c_int
                {
                    temp = 0 as *mut libc::c_void as *mut libc::c_char;
                } else {
                    number = parameter_brace_expand_length(name);
                    if number
                        == -(9223372036854775807 as libc::c_long) - 1 as libc::c_int as libc::c_long
                        && unbound_vars_is_error != 0
                    {
                        set_exit_status(1 as libc::c_int);
                        err_unboundvar(name.offset(1 as libc::c_int as isize));
                        sh_xfree(
                            name as *mut libc::c_void,
                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                            8968 as libc::c_int,
                        );
                        return if interactive_shell != 0 {
                            &mut expand_wdesc_error
                        } else {
                            &mut expand_wdesc_fatal
                        };
                    }
                    sh_xfree(
                        name as *mut libc::c_void,
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        8971 as libc::c_int,
                    );
                    *indexp = sindex;
                    if number < 0 as libc::c_int as libc::c_long {
                        return &mut expand_wdesc_error;
                    } else {
                        ret = alloc_word_desc();
                        let ref mut fresh98 = (*ret).word;
                        *fresh98 = itos(number);
                        return ret;
                    }
                }
            } else {
                if *name.offset(0 as libc::c_int as isize) as libc::c_int == '@' as i32
                    && *name.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
                {
                    if quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0
                        && !quoted_dollar_atp.is_null()
                    {
                        *quoted_dollar_atp = 1 as libc::c_int;
                    }
                    if !contains_dollar_at.is_null() {
                        *contains_dollar_at = 1 as libc::c_int;
                    }
                    tflag |= (1 as libc::c_int) << 8 as libc::c_int;
                }
                if want_indir != 0
                    && *string.offset((sindex - 1 as libc::c_int) as isize) as libc::c_int
                        == '}' as i32
                    && (*string.offset((sindex - 2 as libc::c_int) as isize) as libc::c_int
                        == '*' as i32
                        || *string.offset((sindex - 2 as libc::c_int) as isize) as libc::c_int
                            == '@' as i32)
                    && (1 as libc::c_int != 0
                        && *(*__ctype_b_loc())
                            .offset(*name.offset(1 as libc::c_int as isize) as libc::c_uchar
                                as libc::c_int as isize) as libc::c_int
                            & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                        || *name.offset(1 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                            == '_' as i32)
                {
                    let mut x: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
                    let mut xlist: *mut WORD_LIST = 0 as *mut WORD_LIST;
                    temp1 =
                        strcpy(
                            sh_xmalloc(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(
                                        strlen(name.offset(1 as libc::c_int as isize)) as u64
                                    ),
                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                9004 as libc::c_int,
                            ) as *mut libc::c_char,
                            name.offset(1 as libc::c_int as isize),
                        );
                    number = strlen(temp1) as intmax_t;
                    *temp1.offset((number - 1 as libc::c_int as libc::c_long) as isize) =
                        '\u{0}' as i32 as libc::c_char;
                    x = all_variables_matching_prefix(temp1);
                    xlist = strvec_to_word_list(x, 0 as libc::c_int, 0 as libc::c_int);
                    if *string.offset((sindex - 2 as libc::c_int) as isize) as libc::c_int
                        == '*' as i32
                    {
                        temp = string_list_dollar_star(xlist, quoted, 0 as libc::c_int);
                    } else {
                        temp = string_list_dollar_at(xlist, quoted, 0 as libc::c_int);
                        if quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0
                            && !quoted_dollar_atp.is_null()
                        {
                            *quoted_dollar_atp = 1 as libc::c_int;
                        }
                        if !contains_dollar_at.is_null() {
                            *contains_dollar_at = 1 as libc::c_int;
                        }
                        tflag |= (1 as libc::c_int) << 8 as libc::c_int;
                    }
                    sh_xfree(
                        x as *mut libc::c_void,
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        9021 as libc::c_int,
                    );
                    dispose_words(xlist);
                    sh_xfree(
                        temp1 as *mut libc::c_void,
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        9023 as libc::c_int,
                    );
                    *indexp = sindex;
                    sh_xfree(
                        name as *mut libc::c_void,
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        9026 as libc::c_int,
                    );
                    ret = alloc_word_desc();
                    let ref mut fresh99 = (*ret).word;
                    *fresh99 = temp;
                    (*ret).flags = tflag;
                    return ret;
                }
                if want_indir != 0
                    && *string.offset((sindex - 1 as libc::c_int) as isize) as libc::c_int
                        == '}' as i32
                    && *string.offset((sindex - 2 as libc::c_int) as isize) as libc::c_int
                        == ']' as i32
                    && valid_array_reference(
                        name.offset(1 as libc::c_int as isize),
                        0 as libc::c_int,
                    ) != 0
                {
                    let mut x_0: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut x1: *mut libc::c_char = 0 as *mut libc::c_char;
                    temp1 =
                        strcpy(
                            sh_xmalloc(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(
                                        strlen(name.offset(1 as libc::c_int as isize)) as u64
                                    ),
                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                9041 as libc::c_int,
                            ) as *mut libc::c_char,
                            name.offset(1 as libc::c_int as isize),
                        );
                    x_0 = array_variable_name(
                        temp1,
                        0 as libc::c_int,
                        &mut x1,
                        0 as *mut libc::c_int,
                    );
                    if !x_0.is_null() {
                        sh_xfree(
                            x_0 as *mut libc::c_void,
                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                            9043 as libc::c_int,
                        );
                    }
                    if (*x1.offset(0 as libc::c_int as isize) as libc::c_int == '@' as i32
                        || *x1.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32)
                        && *x1.offset(1 as libc::c_int as isize) as libc::c_int == ']' as i32
                    {
                        temp = array_keys(temp1, quoted, pflags);
                        if *x1.offset(0 as libc::c_int as isize) as libc::c_int == '@' as i32 {
                            if quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0
                                && !quoted_dollar_atp.is_null()
                            {
                                *quoted_dollar_atp = 1 as libc::c_int;
                            }
                            if !contains_dollar_at.is_null() {
                                *contains_dollar_at = 1 as libc::c_int;
                            }
                            tflag |= (1 as libc::c_int) << 8 as libc::c_int;
                        }
                        sh_xfree(
                            name as *mut libc::c_void,
                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                            9057 as libc::c_int,
                        );
                        sh_xfree(
                            temp1 as *mut libc::c_void,
                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                            9058 as libc::c_int,
                        );
                        *indexp = sindex;
                        ret = alloc_word_desc();
                        let ref mut fresh100 = (*ret).word;
                        *fresh100 = temp;
                        (*ret).flags = tflag;
                        return ret;
                    }
                    sh_xfree(
                        temp1 as *mut libc::c_void,
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        9067 as libc::c_int,
                    );
                }
                if valid_brace_expansion_word(
                    if want_indir != 0 {
                        name.offset(1 as libc::c_int as isize)
                    } else {
                        name
                    },
                    var_is_special,
                ) == 0 as libc::c_int
                {
                    temp = 0 as *mut libc::c_void as *mut libc::c_char;
                } else {
                    if want_indir != 0 {
                        tdesc = parameter_brace_expand_indir(
                            name.offset(1 as libc::c_int as isize),
                            var_is_special,
                            quoted,
                            pflags | local_pflags,
                            quoted_dollar_atp,
                            contains_dollar_at,
                        );
                        if tdesc == &mut expand_wdesc_error as *mut WORD_DESC
                            || tdesc == &mut expand_wdesc_fatal as *mut WORD_DESC
                        {
                            temp = 0 as *mut libc::c_void as *mut libc::c_char;
                            current_block = 1002422616457006153;
                        } else {
                            if !tdesc.is_null() && (*tdesc).flags != 0 {
                                (*tdesc).flags &= !((1 as libc::c_int) << 24 as libc::c_int);
                            }
                            current_block = 10570719081292997246;
                        }
                    } else {
                        local_pflags |=
                            0x2 as libc::c_int | pflags & (0x4 as libc::c_int | 0x8 as libc::c_int);
                        tdesc = parameter_brace_expand_word(
                            name,
                            var_is_special,
                            quoted,
                            local_pflags,
                            &mut ind,
                        );
                        current_block = 10570719081292997246;
                    }
                    match current_block {
                        1002422616457006153 => {}
                        _ => {
                            if tdesc == &mut expand_wdesc_error as *mut WORD_DESC
                                || tdesc == &mut expand_wdesc_fatal as *mut WORD_DESC
                            {
                                tflag = 0 as libc::c_int;
                                tdesc = 0 as *mut WORD_DESC;
                            }
                            if !tdesc.is_null() {
                                temp = (*tdesc).word;
                                tflag = (*tdesc).flags;
                                dispose_word_desc(tdesc);
                            } else {
                                temp = 0 as *mut libc::c_char;
                            }
                            if temp == &mut expand_param_error as *mut libc::c_char
                                || temp == &mut expand_param_fatal as *mut libc::c_char
                            {
                                if !name.is_null() {
                                    sh_xfree(
                                        name as *mut libc::c_void,
                                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                                        9116 as libc::c_int,
                                    );
                                }
                                if !value.is_null() {
                                    sh_xfree(
                                        value as *mut libc::c_void,
                                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                                        9117 as libc::c_int,
                                    );
                                }
                                return if temp == &mut expand_param_error as *mut libc::c_char {
                                    &mut expand_wdesc_error
                                } else {
                                    &mut expand_wdesc_fatal
                                };
                            }
                            if valid_array_reference(name, 0 as libc::c_int) != 0 {
                                let mut qflags: libc::c_int = 0;
                                let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
                                qflags = quoted;
                                if pflags & 0x8 as libc::c_int != 0 {
                                    qflags |= 0x1 as libc::c_int;
                                }
                                t = mbschr(name, '[' as i32);
                                if !t.is_null()
                                    && (*t.offset(1 as libc::c_int as isize) as libc::c_int
                                        == '@' as i32
                                        || *t.offset(1 as libc::c_int as isize) as libc::c_int
                                            == '*' as i32)
                                    && *t.offset(2 as libc::c_int as isize) as libc::c_int
                                        == ']' as i32
                                {
                                    all_element_arrayref = 1 as libc::c_int;
                                    if expand_no_split_dollar_star != 0
                                        && *t.offset(1 as libc::c_int as isize) as libc::c_int
                                            == '*' as i32
                                    {
                                        qflags |= 0x1 as libc::c_int;
                                    }
                                }
                                chk_atstar(
                                    name,
                                    qflags,
                                    pflags,
                                    quoted_dollar_atp,
                                    contains_dollar_at,
                                );
                            }
                            var_is_set = (temp != 0 as *mut libc::c_char) as libc::c_int;
                            var_is_null = (check_nullness != 0
                                && (var_is_set == 0 as libc::c_int
                                    || *temp as libc::c_int == 0 as libc::c_int))
                                as libc::c_int;
                            if check_nullness != 0 {
                                var_is_null |= (var_is_set != 0
                                    && var_is_special != 0
                                    && quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0
                                    && (*temp.offset(0 as libc::c_int as isize) as libc::c_int
                                        == '\u{7f}' as i32
                                        && *temp.offset(1 as libc::c_int as isize) as libc::c_int
                                            == '\u{0}' as i32))
                                    as libc::c_int;
                            }
                            if check_nullness != 0 {
                                var_is_null |= (var_is_set != 0
                                    && quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0
                                    && (*temp.offset(0 as libc::c_int as isize) as libc::c_int
                                        == '\u{7f}' as i32
                                        && *temp.offset(1 as libc::c_int as isize) as libc::c_int
                                            == '\u{0}' as i32)
                                    && valid_array_reference(name, 0 as libc::c_int) != 0
                                    && chk_atstar(
                                        name,
                                        0 as libc::c_int,
                                        0 as libc::c_int,
                                        0 as *mut libc::c_int,
                                        0 as *mut libc::c_int,
                                    ) != 0)
                                    as libc::c_int;
                            }
                            if c != 0 && c != '}' as i32 {
                                value = extract_dollar_brace_string(
                                    string,
                                    &mut sindex,
                                    quoted,
                                    if c == '%' as i32
                                        || c == '#' as i32
                                        || c == '/' as i32
                                        || c == '^' as i32
                                        || c == ',' as i32
                                        || c == ':' as i32
                                    {
                                        0x100 as libc::c_int | 0x200 as libc::c_int
                                    } else {
                                        0x200 as libc::c_int
                                    },
                                );
                                if *string.offset(sindex as isize) as libc::c_int == '}' as i32 {
                                    sindex += 1;
                                    current_block = 11322929247169729670;
                                } else {
                                    current_block = 1002422616457006153;
                                }
                            } else {
                                value = 0 as *mut libc::c_void as *mut libc::c_char;
                                current_block = 11322929247169729670;
                            }
                            match current_block {
                                1002422616457006153 => {}
                                _ => {
                                    *indexp = sindex;
                                    if want_substring != 0
                                        || want_patsub != 0
                                        || want_casemod != 0
                                        || c == '@' as i32
                                        || c == '#' as i32
                                        || c == '%' as i32
                                        || c == '}' as i32
                                    {
                                        if var_is_set == 0 as libc::c_int
                                            && unbound_vars_is_error != 0
                                            && (*name.offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                != '@' as i32
                                                && *name.offset(0 as libc::c_int as isize)
                                                    as libc::c_int
                                                    != '*' as i32
                                                || *name.offset(1 as libc::c_int as isize)
                                                    as libc::c_int
                                                    != 0)
                                            && all_element_arrayref == 0 as libc::c_int
                                        {
                                            set_exit_status(1 as libc::c_int);
                                            err_unboundvar(name);
                                            if !value.is_null() {
                                                sh_xfree(
                                                    value as *mut libc::c_void,
                                                    b"../subst.c\0" as *const u8
                                                        as *const libc::c_char,
                                                    9183 as libc::c_int,
                                                );
                                            }
                                            if !temp.is_null() {
                                                sh_xfree(
                                                    temp as *mut libc::c_void,
                                                    b"../subst.c\0" as *const u8
                                                        as *const libc::c_char,
                                                    9184 as libc::c_int,
                                                );
                                            }
                                            sh_xfree(
                                                name as *mut libc::c_void,
                                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                                9185 as libc::c_int,
                                            );
                                            return if interactive_shell != 0 {
                                                &mut expand_wdesc_error
                                            } else {
                                                &mut expand_wdesc_fatal
                                            };
                                        }
                                    }
                                    if want_substring != 0 {
                                        temp1 = parameter_brace_substring(
                                            name,
                                            temp,
                                            ind as libc::c_int,
                                            value,
                                            quoted,
                                            pflags,
                                            if tflag & (1 as libc::c_int) << 24 as libc::c_int != 0
                                            {
                                                0x4 as libc::c_int
                                            } else {
                                                0 as libc::c_int
                                            },
                                        );
                                        if !value.is_null() {
                                            sh_xfree(
                                                value as *mut libc::c_void,
                                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                                9194 as libc::c_int,
                                            );
                                        }
                                        if !temp.is_null() {
                                            sh_xfree(
                                                temp as *mut libc::c_void,
                                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                                9195 as libc::c_int,
                                            );
                                        }
                                        if temp1 == &mut expand_param_error as *mut libc::c_char
                                            || temp1 == &mut expand_param_fatal as *mut libc::c_char
                                        {
                                            if !name.is_null() {
                                                sh_xfree(
                                                    name as *mut libc::c_void,
                                                    b"../subst.c\0" as *const u8
                                                        as *const libc::c_char,
                                                    9199 as libc::c_int,
                                                );
                                            }
                                            return if temp1
                                                == &mut expand_param_error as *mut libc::c_char
                                            {
                                                &mut expand_wdesc_error
                                            } else {
                                                &mut expand_wdesc_fatal
                                            };
                                        }
                                        ret = alloc_word_desc();
                                        let ref mut fresh101 = (*ret).word;
                                        *fresh101 = temp1;
                                        if !temp1.is_null()
                                            && (quoted_dollar_atp.is_null()
                                                || *quoted_dollar_atp == 0 as libc::c_int)
                                            && (*temp1.offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                == '\u{7f}' as i32
                                                && *temp1.offset(1 as libc::c_int as isize)
                                                    as libc::c_int
                                                    == '\u{0}' as i32)
                                            && quoted & (0x2 as libc::c_int | 0x1 as libc::c_int)
                                                != 0
                                        {
                                            (*ret).flags |= (1 as libc::c_int) << 1 as libc::c_int
                                                | (1 as libc::c_int) << 18 as libc::c_int;
                                        } else if !temp1.is_null()
                                            && (*name.offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                == '*' as i32
                                                && *name.offset(1 as libc::c_int as isize)
                                                    as libc::c_int
                                                    == 0 as libc::c_int)
                                            && quoted == 0 as libc::c_int
                                            && pflags & 0x8 as libc::c_int != 0
                                        {
                                            (*ret).flags |= (1 as libc::c_int) << 3 as libc::c_int;
                                        } else if !temp1.is_null()
                                            && (*name.offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                == '*' as i32
                                                && *name.offset(1 as libc::c_int as isize)
                                                    as libc::c_int
                                                    == 0 as libc::c_int)
                                            && quoted == 0 as libc::c_int
                                            && ifs_is_null != 0
                                        {
                                            (*ret).flags |= (1 as libc::c_int) << 3 as libc::c_int;
                                        }
                                        if !name.is_null() {
                                            sh_xfree(
                                                name as *mut libc::c_void,
                                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                                9220 as libc::c_int,
                                            );
                                        }
                                        return ret;
                                    } else {
                                        if want_patsub != 0 {
                                            temp1 = parameter_brace_patsub(
                                                name,
                                                temp,
                                                ind as libc::c_int,
                                                value,
                                                quoted,
                                                pflags,
                                                if tflag & (1 as libc::c_int) << 24 as libc::c_int
                                                    != 0
                                                {
                                                    0x4 as libc::c_int
                                                } else {
                                                    0 as libc::c_int
                                                },
                                            );
                                            if !value.is_null() {
                                                sh_xfree(
                                                    value as *mut libc::c_void,
                                                    b"../subst.c\0" as *const u8
                                                        as *const libc::c_char,
                                                    9226 as libc::c_int,
                                                );
                                            }
                                            if !temp.is_null() {
                                                sh_xfree(
                                                    temp as *mut libc::c_void,
                                                    b"../subst.c\0" as *const u8
                                                        as *const libc::c_char,
                                                    9227 as libc::c_int,
                                                );
                                            }
                                            if temp1 == &mut expand_param_error as *mut libc::c_char
                                                || temp1
                                                    == &mut expand_param_fatal as *mut libc::c_char
                                            {
                                                if !name.is_null() {
                                                    sh_xfree(
                                                        name as *mut libc::c_void,
                                                        b"../subst.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        9231 as libc::c_int,
                                                    );
                                                }
                                                return if temp1
                                                    == &mut expand_param_error as *mut libc::c_char
                                                {
                                                    &mut expand_wdesc_error
                                                } else {
                                                    &mut expand_wdesc_fatal
                                                };
                                            }
                                            ret = alloc_word_desc();
                                            let ref mut fresh102 = (*ret).word;
                                            *fresh102 = temp1;
                                            if !temp1.is_null()
                                                && (quoted_dollar_atp.is_null()
                                                    || *quoted_dollar_atp == 0 as libc::c_int)
                                                && (*temp1.offset(0 as libc::c_int as isize)
                                                    as libc::c_int
                                                    == '\u{7f}' as i32
                                                    && *temp1.offset(1 as libc::c_int as isize)
                                                        as libc::c_int
                                                        == '\u{0}' as i32)
                                                && quoted
                                                    & (0x2 as libc::c_int | 0x1 as libc::c_int)
                                                    != 0
                                            {
                                                (*ret).flags |= (1 as libc::c_int)
                                                    << 1 as libc::c_int
                                                    | (1 as libc::c_int) << 18 as libc::c_int;
                                            } else if !temp1.is_null()
                                                && (*name.offset(0 as libc::c_int as isize)
                                                    as libc::c_int
                                                    == '*' as i32
                                                    && *name.offset(1 as libc::c_int as isize)
                                                        as libc::c_int
                                                        == 0 as libc::c_int)
                                                && quoted == 0 as libc::c_int
                                                && ifs_is_null != 0
                                            {
                                                (*ret).flags |=
                                                    (1 as libc::c_int) << 3 as libc::c_int;
                                            }
                                            if !name.is_null() {
                                                sh_xfree(
                                                    name as *mut libc::c_void,
                                                    b"../subst.c\0" as *const u8
                                                        as *const libc::c_char,
                                                    9245 as libc::c_int,
                                                );
                                            }
                                            return ret;
                                        } else {
                                            if want_casemod != 0 {
                                                temp1 = parameter_brace_casemod(
                                                    name,
                                                    temp,
                                                    ind as libc::c_int,
                                                    modspec,
                                                    value,
                                                    quoted,
                                                    pflags,
                                                    if tflag
                                                        & (1 as libc::c_int) << 24 as libc::c_int
                                                        != 0
                                                    {
                                                        0x4 as libc::c_int
                                                    } else {
                                                        0 as libc::c_int
                                                    },
                                                );
                                                if !value.is_null() {
                                                    sh_xfree(
                                                        value as *mut libc::c_void,
                                                        b"../subst.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        9252 as libc::c_int,
                                                    );
                                                }
                                                if !temp.is_null() {
                                                    sh_xfree(
                                                        temp as *mut libc::c_void,
                                                        b"../subst.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        9253 as libc::c_int,
                                                    );
                                                }
                                                if temp1
                                                    == &mut expand_param_error as *mut libc::c_char
                                                    || temp1
                                                        == &mut expand_param_fatal
                                                            as *mut libc::c_char
                                                {
                                                    if !name.is_null() {
                                                        sh_xfree(
                                                            name as *mut libc::c_void,
                                                            b"../subst.c\0" as *const u8
                                                                as *const libc::c_char,
                                                            9257 as libc::c_int,
                                                        );
                                                    }
                                                    return if temp1
                                                        == &mut expand_param_error
                                                            as *mut libc::c_char
                                                    {
                                                        &mut expand_wdesc_error
                                                    } else {
                                                        &mut expand_wdesc_fatal
                                                    };
                                                }
                                                ret = alloc_word_desc();
                                                let ref mut fresh103 = (*ret).word;
                                                *fresh103 = temp1;
                                                if !temp1.is_null()
                                                    && (quoted_dollar_atp.is_null()
                                                        || *quoted_dollar_atp == 0 as libc::c_int)
                                                    && (*temp1.offset(0 as libc::c_int as isize)
                                                        as libc::c_int
                                                        == '\u{7f}' as i32
                                                        && *temp1.offset(1 as libc::c_int as isize)
                                                            as libc::c_int
                                                            == '\u{0}' as i32)
                                                    && quoted
                                                        & (0x2 as libc::c_int | 0x1 as libc::c_int)
                                                        != 0
                                                {
                                                    (*ret).flags |= (1 as libc::c_int)
                                                        << 1 as libc::c_int
                                                        | (1 as libc::c_int) << 18 as libc::c_int;
                                                } else if !temp1.is_null()
                                                    && (*name.offset(0 as libc::c_int as isize)
                                                        as libc::c_int
                                                        == '*' as i32
                                                        && *name.offset(1 as libc::c_int as isize)
                                                            as libc::c_int
                                                            == 0 as libc::c_int)
                                                    && quoted == 0 as libc::c_int
                                                    && ifs_is_null != 0
                                                {
                                                    (*ret).flags |=
                                                        (1 as libc::c_int) << 3 as libc::c_int;
                                                }
                                                if !name.is_null() {
                                                    sh_xfree(
                                                        name as *mut libc::c_void,
                                                        b"../subst.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        9271 as libc::c_int,
                                                    );
                                                }
                                                return ret;
                                            }
                                        }
                                    }
                                    match c {
                                        125 => {
                                            current_block = 9668344364916463090;
                                            match current_block {
                                                6396965539391641386 => {
                                                    temp1 = parameter_brace_transform(
                                                        name,
                                                        temp,
                                                        ind as libc::c_int,
                                                        value,
                                                        c,
                                                        quoted,
                                                        pflags,
                                                        if tflag
                                                            & (1 as libc::c_int)
                                                                << 24 as libc::c_int
                                                            != 0
                                                        {
                                                            0x4 as libc::c_int
                                                        } else {
                                                            0 as libc::c_int
                                                        },
                                                    );
                                                    sh_xfree(
                                                        temp as *mut libc::c_void,
                                                        b"../subst.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        9297 as libc::c_int,
                                                    );
                                                    sh_xfree(
                                                        value as *mut libc::c_void,
                                                        b"../subst.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        9298 as libc::c_int,
                                                    );
                                                    if temp1
                                                        == &mut expand_param_error
                                                            as *mut libc::c_char
                                                        || temp1
                                                            == &mut expand_param_fatal
                                                                as *mut libc::c_char
                                                    {
                                                        sh_xfree(
                                                            name as *mut libc::c_void,
                                                            b"../subst.c\0" as *const u8
                                                                as *const libc::c_char,
                                                            9302 as libc::c_int,
                                                        );
                                                        set_exit_status(1 as libc::c_int);
                                                        report_error(
                                                            dcgettext(
                                                                0 as *const libc::c_char,
                                                                b"%s: bad substitution\0"
                                                                    as *const u8
                                                                    as *const libc::c_char,
                                                                5 as libc::c_int,
                                                            ),
                                                            if !string.is_null() {
                                                                string
                                                            } else {
                                                                b"??\0" as *const u8
                                                                    as *const libc::c_char
                                                            },
                                                        );
                                                        return if temp1
                                                            == &mut expand_param_error
                                                                as *mut libc::c_char
                                                        {
                                                            &mut expand_wdesc_error
                                                        } else {
                                                            &mut expand_wdesc_fatal
                                                        };
                                                    }
                                                    ret = alloc_word_desc();
                                                    let ref mut fresh104 = (*ret).word;
                                                    *fresh104 = temp1;
                                                    if !temp1.is_null()
                                                        && (*temp1.offset(0 as libc::c_int as isize)
                                                            as libc::c_int
                                                            == '\u{7f}' as i32
                                                            && *temp1
                                                                .offset(1 as libc::c_int as isize)
                                                                as libc::c_int
                                                                == '\u{0}' as i32)
                                                        && quoted
                                                            & (0x2 as libc::c_int
                                                                | 0x1 as libc::c_int)
                                                            != 0
                                                    {
                                                        (*ret).flags |= (1 as libc::c_int)
                                                            << 1 as libc::c_int
                                                            | (1 as libc::c_int)
                                                                << 18 as libc::c_int;
                                                    } else if !temp1.is_null()
                                                        && (*name.offset(0 as libc::c_int as isize)
                                                            as libc::c_int
                                                            == '*' as i32
                                                            && *name
                                                                .offset(1 as libc::c_int as isize)
                                                                as libc::c_int
                                                                == 0 as libc::c_int)
                                                        && quoted == 0 as libc::c_int
                                                        && ifs_is_null != 0
                                                    {
                                                        (*ret).flags |=
                                                            (1 as libc::c_int) << 3 as libc::c_int;
                                                    }
                                                    sh_xfree(
                                                        name as *mut libc::c_void,
                                                        b"../subst.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        9316 as libc::c_int,
                                                    );
                                                    return ret;
                                                }
                                                6979992077050147181 => {
                                                    current_block = 17370964531530178945;
                                                }
                                                1951049175126603298 => {
                                                    if var_is_set != 0
                                                        && var_is_null == 0 as libc::c_int
                                                    {
                                                        if c == '+' as i32 {
                                                            if quoted
                                                                & (0x2 as libc::c_int
                                                                    | 0x1 as libc::c_int)
                                                                != 0
                                                                && !quoted_dollar_atp.is_null()
                                                            {
                                                                *quoted_dollar_atp =
                                                                    0 as libc::c_int;
                                                            }
                                                            if !contains_dollar_at.is_null() {
                                                                *contains_dollar_at =
                                                                    0 as libc::c_int;
                                                            }
                                                            if !temp.is_null() {
                                                                sh_xfree(
                                                                    temp as *mut libc::c_void,
                                                                    b"../subst.c\0" as *const u8
                                                                        as *const libc::c_char,
                                                                    9359 as libc::c_int,
                                                                );
                                                            }
                                                            if !value.is_null() {
                                                                if quoted
                                                                    & (0x2 as libc::c_int
                                                                        | 0x1 as libc::c_int)
                                                                    != 0
                                                                {
                                                                    quoted |= 0x80 as libc::c_int;
                                                                }
                                                                ret = parameter_brace_expand_rhs(
                                                                    name,
                                                                    value,
                                                                    c,
                                                                    quoted,
                                                                    pflags,
                                                                    quoted_dollar_atp,
                                                                    contains_dollar_at,
                                                                );
                                                                sh_xfree(
                                                                    value as *mut libc::c_void,
                                                                    b"../subst.c\0" as *const u8
                                                                        as *const libc::c_char,
                                                                    9374 as libc::c_int,
                                                                );
                                                            } else {
                                                                temp = 0 as *mut libc::c_void
                                                                    as *mut libc::c_char;
                                                            }
                                                        } else if !value.is_null() {
                                                            sh_xfree(
                                                                value as *mut libc::c_void,
                                                                b"../subst.c\0" as *const u8
                                                                    as *const libc::c_char,
                                                                9381 as libc::c_int,
                                                            );
                                                        }
                                                    } else {
                                                        if !temp.is_null() {
                                                            sh_xfree(
                                                                temp as *mut libc::c_void,
                                                                b"../subst.c\0" as *const u8
                                                                    as *const libc::c_char,
                                                                9387 as libc::c_int,
                                                            );
                                                        }
                                                        temp = 0 as *mut libc::c_void
                                                            as *mut libc::c_char;
                                                        if c == '=' as i32 && var_is_special != 0 {
                                                            set_exit_status(1 as libc::c_int);
                                                            report_error(
                                                            dcgettext(
                                                                0 as *const libc::c_char,
                                                                b"$%s: cannot assign in this way\0"
                                                                    as *const u8
                                                                    as *const libc::c_char,
                                                                5 as libc::c_int,
                                                            ),
                                                            name,
                                                        );
                                                            sh_xfree(
                                                                name as *mut libc::c_void,
                                                                b"../subst.c\0" as *const u8
                                                                    as *const libc::c_char,
                                                                9393 as libc::c_int,
                                                            );
                                                            sh_xfree(
                                                                value as *mut libc::c_void,
                                                                b"../subst.c\0" as *const u8
                                                                    as *const libc::c_char,
                                                                9394 as libc::c_int,
                                                            );
                                                            return &mut expand_wdesc_error;
                                                        } else {
                                                            if c == '?' as i32 {
                                                                parameter_brace_expand_error(
                                                                    name,
                                                                    value,
                                                                    check_nullness,
                                                                );
                                                                return if interactive_shell != 0 {
                                                                    &mut expand_wdesc_error
                                                                } else {
                                                                    &mut expand_wdesc_fatal
                                                                };
                                                            } else {
                                                                if c != '+' as i32 {
                                                                    if quoted
                                                                        & (0x2 as libc::c_int
                                                                            | 0x1 as libc::c_int)
                                                                        != 0
                                                                        && !quoted_dollar_atp
                                                                            .is_null()
                                                                    {
                                                                        *quoted_dollar_atp =
                                                                            0 as libc::c_int;
                                                                    }
                                                                    if !contains_dollar_at.is_null()
                                                                    {
                                                                        *contains_dollar_at =
                                                                            0 as libc::c_int;
                                                                    }
                                                                    if quoted
                                                                        & (0x2 as libc::c_int
                                                                            | 0x1 as libc::c_int)
                                                                        != 0
                                                                    {
                                                                        quoted |=
                                                                            0x80 as libc::c_int;
                                                                    }
                                                                    ret =
                                                                        parameter_brace_expand_rhs(
                                                                            name,
                                                                            value,
                                                                            c,
                                                                            quoted,
                                                                            pflags,
                                                                            quoted_dollar_atp,
                                                                            contains_dollar_at,
                                                                        );
                                                                }
                                                            }
                                                        }
                                                        sh_xfree(
                                                            value as *mut libc::c_void,
                                                            b"../subst.c\0" as *const u8
                                                                as *const libc::c_char,
                                                            9423 as libc::c_int,
                                                        );
                                                    }
                                                    current_block = 9668344364916463090;
                                                }
                                                _ => {}
                                            }
                                            match current_block {
                                                17370964531530178945 => {
                                                    if value.is_null()
                                                        || *value as libc::c_int == '\u{0}' as i32
                                                        || temp.is_null()
                                                        || *temp as libc::c_int == '\u{0}' as i32
                                                    {
                                                        if !value.is_null() {
                                                            sh_xfree(
                                                                value as *mut libc::c_void,
                                                                b"../subst.c\0" as *const u8
                                                                    as *const libc::c_char,
                                                                9323 as libc::c_int,
                                                            );
                                                        }
                                                    } else {
                                                        temp1 = parameter_brace_remove_pattern(
                                                            name,
                                                            temp,
                                                            ind as libc::c_int,
                                                            value,
                                                            c,
                                                            quoted,
                                                            if tflag
                                                                & (1 as libc::c_int)
                                                                    << 24 as libc::c_int
                                                                != 0
                                                            {
                                                                0x4 as libc::c_int
                                                            } else {
                                                                0 as libc::c_int
                                                            },
                                                        );
                                                        sh_xfree(
                                                            temp as *mut libc::c_void,
                                                            b"../subst.c\0" as *const u8
                                                                as *const libc::c_char,
                                                            9327 as libc::c_int,
                                                        );
                                                        sh_xfree(
                                                            value as *mut libc::c_void,
                                                            b"../subst.c\0" as *const u8
                                                                as *const libc::c_char,
                                                            9328 as libc::c_int,
                                                        );
                                                        ret = alloc_word_desc();
                                                        let ref mut fresh105 = (*ret).word;
                                                        *fresh105 = temp1;
                                                        if !temp1.is_null()
                                                            && (*temp1
                                                                .offset(0 as libc::c_int as isize)
                                                                as libc::c_int
                                                                == '\u{7f}' as i32
                                                                && *temp1.offset(
                                                                    1 as libc::c_int as isize,
                                                                )
                                                                    as libc::c_int
                                                                    == '\u{0}' as i32)
                                                            && quoted
                                                                & (0x2 as libc::c_int
                                                                    | 0x1 as libc::c_int)
                                                                != 0
                                                        {
                                                            (*ret).flags |= (1 as libc::c_int)
                                                                << 1 as libc::c_int
                                                                | (1 as libc::c_int)
                                                                    << 18 as libc::c_int;
                                                        } else if !temp1.is_null()
                                                            && (*name
                                                                .offset(0 as libc::c_int as isize)
                                                                as libc::c_int
                                                                == '*' as i32
                                                                && *name.offset(
                                                                    1 as libc::c_int as isize,
                                                                )
                                                                    as libc::c_int
                                                                    == 0 as libc::c_int)
                                                            && quoted == 0 as libc::c_int
                                                            && ifs_is_null != 0
                                                        {
                                                            (*ret).flags |= (1 as libc::c_int)
                                                                << 3 as libc::c_int;
                                                        }
                                                        sh_xfree(
                                                            name as *mut libc::c_void,
                                                            b"../subst.c\0" as *const u8
                                                                as *const libc::c_char,
                                                            9338 as libc::c_int,
                                                        );
                                                        return ret;
                                                    }
                                                }
                                                _ => {}
                                            }
                                            sh_xfree(
                                                name as *mut libc::c_void,
                                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                                9428 as libc::c_int,
                                            );
                                            if ret.is_null() {
                                                ret = alloc_word_desc();
                                                (*ret).flags = tflag;
                                                let ref mut fresh106 = (*ret).word;
                                                *fresh106 = temp;
                                            }
                                            return ret;
                                        }
                                        64 => {
                                            current_block = 6396965539391641386;
                                            match current_block {
                                                6396965539391641386 => {
                                                    temp1 = parameter_brace_transform(
                                                        name,
                                                        temp,
                                                        ind as libc::c_int,
                                                        value,
                                                        c,
                                                        quoted,
                                                        pflags,
                                                        if tflag
                                                            & (1 as libc::c_int)
                                                                << 24 as libc::c_int
                                                            != 0
                                                        {
                                                            0x4 as libc::c_int
                                                        } else {
                                                            0 as libc::c_int
                                                        },
                                                    );
                                                    sh_xfree(
                                                        temp as *mut libc::c_void,
                                                        b"../subst.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        9297 as libc::c_int,
                                                    );
                                                    sh_xfree(
                                                        value as *mut libc::c_void,
                                                        b"../subst.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        9298 as libc::c_int,
                                                    );
                                                    if temp1
                                                        == &mut expand_param_error
                                                            as *mut libc::c_char
                                                        || temp1
                                                            == &mut expand_param_fatal
                                                                as *mut libc::c_char
                                                    {
                                                        sh_xfree(
                                                            name as *mut libc::c_void,
                                                            b"../subst.c\0" as *const u8
                                                                as *const libc::c_char,
                                                            9302 as libc::c_int,
                                                        );
                                                        set_exit_status(1 as libc::c_int);
                                                        report_error(
                                                            dcgettext(
                                                                0 as *const libc::c_char,
                                                                b"%s: bad substitution\0"
                                                                    as *const u8
                                                                    as *const libc::c_char,
                                                                5 as libc::c_int,
                                                            ),
                                                            if !string.is_null() {
                                                                string
                                                            } else {
                                                                b"??\0" as *const u8
                                                                    as *const libc::c_char
                                                            },
                                                        );
                                                        return if temp1
                                                            == &mut expand_param_error
                                                                as *mut libc::c_char
                                                        {
                                                            &mut expand_wdesc_error
                                                        } else {
                                                            &mut expand_wdesc_fatal
                                                        };
                                                    }
                                                    ret = alloc_word_desc();
                                                    let ref mut fresh104 = (*ret).word;
                                                    *fresh104 = temp1;
                                                    if !temp1.is_null()
                                                        && (*temp1.offset(0 as libc::c_int as isize)
                                                            as libc::c_int
                                                            == '\u{7f}' as i32
                                                            && *temp1
                                                                .offset(1 as libc::c_int as isize)
                                                                as libc::c_int
                                                                == '\u{0}' as i32)
                                                        && quoted
                                                            & (0x2 as libc::c_int
                                                                | 0x1 as libc::c_int)
                                                            != 0
                                                    {
                                                        (*ret).flags |= (1 as libc::c_int)
                                                            << 1 as libc::c_int
                                                            | (1 as libc::c_int)
                                                                << 18 as libc::c_int;
                                                    } else if !temp1.is_null()
                                                        && (*name.offset(0 as libc::c_int as isize)
                                                            as libc::c_int
                                                            == '*' as i32
                                                            && *name
                                                                .offset(1 as libc::c_int as isize)
                                                                as libc::c_int
                                                                == 0 as libc::c_int)
                                                        && quoted == 0 as libc::c_int
                                                        && ifs_is_null != 0
                                                    {
                                                        (*ret).flags |=
                                                            (1 as libc::c_int) << 3 as libc::c_int;
                                                    }
                                                    sh_xfree(
                                                        name as *mut libc::c_void,
                                                        b"../subst.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        9316 as libc::c_int,
                                                    );
                                                    return ret;
                                                }
                                                6979992077050147181 => {
                                                    current_block = 17370964531530178945;
                                                }
                                                1951049175126603298 => {
                                                    if var_is_set != 0
                                                        && var_is_null == 0 as libc::c_int
                                                    {
                                                        if c == '+' as i32 {
                                                            if quoted
                                                                & (0x2 as libc::c_int
                                                                    | 0x1 as libc::c_int)
                                                                != 0
                                                                && !quoted_dollar_atp.is_null()
                                                            {
                                                                *quoted_dollar_atp =
                                                                    0 as libc::c_int;
                                                            }
                                                            if !contains_dollar_at.is_null() {
                                                                *contains_dollar_at =
                                                                    0 as libc::c_int;
                                                            }
                                                            if !temp.is_null() {
                                                                sh_xfree(
                                                                    temp as *mut libc::c_void,
                                                                    b"../subst.c\0" as *const u8
                                                                        as *const libc::c_char,
                                                                    9359 as libc::c_int,
                                                                );
                                                            }
                                                            if !value.is_null() {
                                                                if quoted
                                                                    & (0x2 as libc::c_int
                                                                        | 0x1 as libc::c_int)
                                                                    != 0
                                                                {
                                                                    quoted |= 0x80 as libc::c_int;
                                                                }
                                                                ret = parameter_brace_expand_rhs(
                                                                    name,
                                                                    value,
                                                                    c,
                                                                    quoted,
                                                                    pflags,
                                                                    quoted_dollar_atp,
                                                                    contains_dollar_at,
                                                                );
                                                                sh_xfree(
                                                                    value as *mut libc::c_void,
                                                                    b"../subst.c\0" as *const u8
                                                                        as *const libc::c_char,
                                                                    9374 as libc::c_int,
                                                                );
                                                            } else {
                                                                temp = 0 as *mut libc::c_void
                                                                    as *mut libc::c_char;
                                                            }
                                                        } else if !value.is_null() {
                                                            sh_xfree(
                                                                value as *mut libc::c_void,
                                                                b"../subst.c\0" as *const u8
                                                                    as *const libc::c_char,
                                                                9381 as libc::c_int,
                                                            );
                                                        }
                                                    } else {
                                                        if !temp.is_null() {
                                                            sh_xfree(
                                                                temp as *mut libc::c_void,
                                                                b"../subst.c\0" as *const u8
                                                                    as *const libc::c_char,
                                                                9387 as libc::c_int,
                                                            );
                                                        }
                                                        temp = 0 as *mut libc::c_void
                                                            as *mut libc::c_char;
                                                        if c == '=' as i32 && var_is_special != 0 {
                                                            set_exit_status(1 as libc::c_int);
                                                            report_error(
                                                            dcgettext(
                                                                0 as *const libc::c_char,
                                                                b"$%s: cannot assign in this way\0"
                                                                    as *const u8
                                                                    as *const libc::c_char,
                                                                5 as libc::c_int,
                                                            ),
                                                            name,
                                                        );
                                                            sh_xfree(
                                                                name as *mut libc::c_void,
                                                                b"../subst.c\0" as *const u8
                                                                    as *const libc::c_char,
                                                                9393 as libc::c_int,
                                                            );
                                                            sh_xfree(
                                                                value as *mut libc::c_void,
                                                                b"../subst.c\0" as *const u8
                                                                    as *const libc::c_char,
                                                                9394 as libc::c_int,
                                                            );
                                                            return &mut expand_wdesc_error;
                                                        } else {
                                                            if c == '?' as i32 {
                                                                parameter_brace_expand_error(
                                                                    name,
                                                                    value,
                                                                    check_nullness,
                                                                );
                                                                return if interactive_shell != 0 {
                                                                    &mut expand_wdesc_error
                                                                } else {
                                                                    &mut expand_wdesc_fatal
                                                                };
                                                            } else {
                                                                if c != '+' as i32 {
                                                                    if quoted
                                                                        & (0x2 as libc::c_int
                                                                            | 0x1 as libc::c_int)
                                                                        != 0
                                                                        && !quoted_dollar_atp
                                                                            .is_null()
                                                                    {
                                                                        *quoted_dollar_atp =
                                                                            0 as libc::c_int;
                                                                    }
                                                                    if !contains_dollar_at.is_null()
                                                                    {
                                                                        *contains_dollar_at =
                                                                            0 as libc::c_int;
                                                                    }
                                                                    if quoted
                                                                        & (0x2 as libc::c_int
                                                                            | 0x1 as libc::c_int)
                                                                        != 0
                                                                    {
                                                                        quoted |=
                                                                            0x80 as libc::c_int;
                                                                    }
                                                                    ret =
                                                                        parameter_brace_expand_rhs(
                                                                            name,
                                                                            value,
                                                                            c,
                                                                            quoted,
                                                                            pflags,
                                                                            quoted_dollar_atp,
                                                                            contains_dollar_at,
                                                                        );
                                                                }
                                                            }
                                                        }
                                                        sh_xfree(
                                                            value as *mut libc::c_void,
                                                            b"../subst.c\0" as *const u8
                                                                as *const libc::c_char,
                                                            9423 as libc::c_int,
                                                        );
                                                    }
                                                    current_block = 9668344364916463090;
                                                }
                                                _ => {}
                                            }
                                            match current_block {
                                                17370964531530178945 => {
                                                    if value.is_null()
                                                        || *value as libc::c_int == '\u{0}' as i32
                                                        || temp.is_null()
                                                        || *temp as libc::c_int == '\u{0}' as i32
                                                    {
                                                        if !value.is_null() {
                                                            sh_xfree(
                                                                value as *mut libc::c_void,
                                                                b"../subst.c\0" as *const u8
                                                                    as *const libc::c_char,
                                                                9323 as libc::c_int,
                                                            );
                                                        }
                                                    } else {
                                                        temp1 = parameter_brace_remove_pattern(
                                                            name,
                                                            temp,
                                                            ind as libc::c_int,
                                                            value,
                                                            c,
                                                            quoted,
                                                            if tflag
                                                                & (1 as libc::c_int)
                                                                    << 24 as libc::c_int
                                                                != 0
                                                            {
                                                                0x4 as libc::c_int
                                                            } else {
                                                                0 as libc::c_int
                                                            },
                                                        );
                                                        sh_xfree(
                                                            temp as *mut libc::c_void,
                                                            b"../subst.c\0" as *const u8
                                                                as *const libc::c_char,
                                                            9327 as libc::c_int,
                                                        );
                                                        sh_xfree(
                                                            value as *mut libc::c_void,
                                                            b"../subst.c\0" as *const u8
                                                                as *const libc::c_char,
                                                            9328 as libc::c_int,
                                                        );
                                                        ret = alloc_word_desc();
                                                        let ref mut fresh105 = (*ret).word;
                                                        *fresh105 = temp1;
                                                        if !temp1.is_null()
                                                            && (*temp1
                                                                .offset(0 as libc::c_int as isize)
                                                                as libc::c_int
                                                                == '\u{7f}' as i32
                                                                && *temp1.offset(
                                                                    1 as libc::c_int as isize,
                                                                )
                                                                    as libc::c_int
                                                                    == '\u{0}' as i32)
                                                            && quoted
                                                                & (0x2 as libc::c_int
                                                                    | 0x1 as libc::c_int)
                                                                != 0
                                                        {
                                                            (*ret).flags |= (1 as libc::c_int)
                                                                << 1 as libc::c_int
                                                                | (1 as libc::c_int)
                                                                    << 18 as libc::c_int;
                                                        } else if !temp1.is_null()
                                                            && (*name
                                                                .offset(0 as libc::c_int as isize)
                                                                as libc::c_int
                                                                == '*' as i32
                                                                && *name.offset(
                                                                    1 as libc::c_int as isize,
                                                                )
                                                                    as libc::c_int
                                                                    == 0 as libc::c_int)
                                                            && quoted == 0 as libc::c_int
                                                            && ifs_is_null != 0
                                                        {
                                                            (*ret).flags |= (1 as libc::c_int)
                                                                << 3 as libc::c_int;
                                                        }
                                                        sh_xfree(
                                                            name as *mut libc::c_void,
                                                            b"../subst.c\0" as *const u8
                                                                as *const libc::c_char,
                                                            9338 as libc::c_int,
                                                        );
                                                        return ret;
                                                    }
                                                }
                                                _ => {}
                                            }
                                            sh_xfree(
                                                name as *mut libc::c_void,
                                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                                9428 as libc::c_int,
                                            );
                                            if ret.is_null() {
                                                ret = alloc_word_desc();
                                                (*ret).flags = tflag;
                                                let ref mut fresh106 = (*ret).word;
                                                *fresh106 = temp;
                                            }
                                            return ret;
                                        }
                                        35 => {
                                            current_block = 6979992077050147181;
                                            match current_block {
                                                6396965539391641386 => {
                                                    temp1 = parameter_brace_transform(
                                                        name,
                                                        temp,
                                                        ind as libc::c_int,
                                                        value,
                                                        c,
                                                        quoted,
                                                        pflags,
                                                        if tflag
                                                            & (1 as libc::c_int)
                                                                << 24 as libc::c_int
                                                            != 0
                                                        {
                                                            0x4 as libc::c_int
                                                        } else {
                                                            0 as libc::c_int
                                                        },
                                                    );
                                                    sh_xfree(
                                                        temp as *mut libc::c_void,
                                                        b"../subst.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        9297 as libc::c_int,
                                                    );
                                                    sh_xfree(
                                                        value as *mut libc::c_void,
                                                        b"../subst.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        9298 as libc::c_int,
                                                    );
                                                    if temp1
                                                        == &mut expand_param_error
                                                            as *mut libc::c_char
                                                        || temp1
                                                            == &mut expand_param_fatal
                                                                as *mut libc::c_char
                                                    {
                                                        sh_xfree(
                                                            name as *mut libc::c_void,
                                                            b"../subst.c\0" as *const u8
                                                                as *const libc::c_char,
                                                            9302 as libc::c_int,
                                                        );
                                                        set_exit_status(1 as libc::c_int);
                                                        report_error(
                                                            dcgettext(
                                                                0 as *const libc::c_char,
                                                                b"%s: bad substitution\0"
                                                                    as *const u8
                                                                    as *const libc::c_char,
                                                                5 as libc::c_int,
                                                            ),
                                                            if !string.is_null() {
                                                                string
                                                            } else {
                                                                b"??\0" as *const u8
                                                                    as *const libc::c_char
                                                            },
                                                        );
                                                        return if temp1
                                                            == &mut expand_param_error
                                                                as *mut libc::c_char
                                                        {
                                                            &mut expand_wdesc_error
                                                        } else {
                                                            &mut expand_wdesc_fatal
                                                        };
                                                    }
                                                    ret = alloc_word_desc();
                                                    let ref mut fresh104 = (*ret).word;
                                                    *fresh104 = temp1;
                                                    if !temp1.is_null()
                                                        && (*temp1.offset(0 as libc::c_int as isize)
                                                            as libc::c_int
                                                            == '\u{7f}' as i32
                                                            && *temp1
                                                                .offset(1 as libc::c_int as isize)
                                                                as libc::c_int
                                                                == '\u{0}' as i32)
                                                        && quoted
                                                            & (0x2 as libc::c_int
                                                                | 0x1 as libc::c_int)
                                                            != 0
                                                    {
                                                        (*ret).flags |= (1 as libc::c_int)
                                                            << 1 as libc::c_int
                                                            | (1 as libc::c_int)
                                                                << 18 as libc::c_int;
                                                    } else if !temp1.is_null()
                                                        && (*name.offset(0 as libc::c_int as isize)
                                                            as libc::c_int
                                                            == '*' as i32
                                                            && *name
                                                                .offset(1 as libc::c_int as isize)
                                                                as libc::c_int
                                                                == 0 as libc::c_int)
                                                        && quoted == 0 as libc::c_int
                                                        && ifs_is_null != 0
                                                    {
                                                        (*ret).flags |=
                                                            (1 as libc::c_int) << 3 as libc::c_int;
                                                    }
                                                    sh_xfree(
                                                        name as *mut libc::c_void,
                                                        b"../subst.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        9316 as libc::c_int,
                                                    );
                                                    return ret;
                                                }
                                                6979992077050147181 => {
                                                    current_block = 17370964531530178945;
                                                }
                                                1951049175126603298 => {
                                                    if var_is_set != 0
                                                        && var_is_null == 0 as libc::c_int
                                                    {
                                                        if c == '+' as i32 {
                                                            if quoted
                                                                & (0x2 as libc::c_int
                                                                    | 0x1 as libc::c_int)
                                                                != 0
                                                                && !quoted_dollar_atp.is_null()
                                                            {
                                                                *quoted_dollar_atp =
                                                                    0 as libc::c_int;
                                                            }
                                                            if !contains_dollar_at.is_null() {
                                                                *contains_dollar_at =
                                                                    0 as libc::c_int;
                                                            }
                                                            if !temp.is_null() {
                                                                sh_xfree(
                                                                    temp as *mut libc::c_void,
                                                                    b"../subst.c\0" as *const u8
                                                                        as *const libc::c_char,
                                                                    9359 as libc::c_int,
                                                                );
                                                            }
                                                            if !value.is_null() {
                                                                if quoted
                                                                    & (0x2 as libc::c_int
                                                                        | 0x1 as libc::c_int)
                                                                    != 0
                                                                {
                                                                    quoted |= 0x80 as libc::c_int;
                                                                }
                                                                ret = parameter_brace_expand_rhs(
                                                                    name,
                                                                    value,
                                                                    c,
                                                                    quoted,
                                                                    pflags,
                                                                    quoted_dollar_atp,
                                                                    contains_dollar_at,
                                                                );
                                                                sh_xfree(
                                                                    value as *mut libc::c_void,
                                                                    b"../subst.c\0" as *const u8
                                                                        as *const libc::c_char,
                                                                    9374 as libc::c_int,
                                                                );
                                                            } else {
                                                                temp = 0 as *mut libc::c_void
                                                                    as *mut libc::c_char;
                                                            }
                                                        } else if !value.is_null() {
                                                            sh_xfree(
                                                                value as *mut libc::c_void,
                                                                b"../subst.c\0" as *const u8
                                                                    as *const libc::c_char,
                                                                9381 as libc::c_int,
                                                            );
                                                        }
                                                    } else {
                                                        if !temp.is_null() {
                                                            sh_xfree(
                                                                temp as *mut libc::c_void,
                                                                b"../subst.c\0" as *const u8
                                                                    as *const libc::c_char,
                                                                9387 as libc::c_int,
                                                            );
                                                        }
                                                        temp = 0 as *mut libc::c_void
                                                            as *mut libc::c_char;
                                                        if c == '=' as i32 && var_is_special != 0 {
                                                            set_exit_status(1 as libc::c_int);
                                                            report_error(
                                                            dcgettext(
                                                                0 as *const libc::c_char,
                                                                b"$%s: cannot assign in this way\0"
                                                                    as *const u8
                                                                    as *const libc::c_char,
                                                                5 as libc::c_int,
                                                            ),
                                                            name,
                                                        );
                                                            sh_xfree(
                                                                name as *mut libc::c_void,
                                                                b"../subst.c\0" as *const u8
                                                                    as *const libc::c_char,
                                                                9393 as libc::c_int,
                                                            );
                                                            sh_xfree(
                                                                value as *mut libc::c_void,
                                                                b"../subst.c\0" as *const u8
                                                                    as *const libc::c_char,
                                                                9394 as libc::c_int,
                                                            );
                                                            return &mut expand_wdesc_error;
                                                        } else {
                                                            if c == '?' as i32 {
                                                                parameter_brace_expand_error(
                                                                    name,
                                                                    value,
                                                                    check_nullness,
                                                                );
                                                                return if interactive_shell != 0 {
                                                                    &mut expand_wdesc_error
                                                                } else {
                                                                    &mut expand_wdesc_fatal
                                                                };
                                                            } else {
                                                                if c != '+' as i32 {
                                                                    if quoted
                                                                        & (0x2 as libc::c_int
                                                                            | 0x1 as libc::c_int)
                                                                        != 0
                                                                        && !quoted_dollar_atp
                                                                            .is_null()
                                                                    {
                                                                        *quoted_dollar_atp =
                                                                            0 as libc::c_int;
                                                                    }
                                                                    if !contains_dollar_at.is_null()
                                                                    {
                                                                        *contains_dollar_at =
                                                                            0 as libc::c_int;
                                                                    }
                                                                    if quoted
                                                                        & (0x2 as libc::c_int
                                                                            | 0x1 as libc::c_int)
                                                                        != 0
                                                                    {
                                                                        quoted |=
                                                                            0x80 as libc::c_int;
                                                                    }
                                                                    ret =
                                                                        parameter_brace_expand_rhs(
                                                                            name,
                                                                            value,
                                                                            c,
                                                                            quoted,
                                                                            pflags,
                                                                            quoted_dollar_atp,
                                                                            contains_dollar_at,
                                                                        );
                                                                }
                                                            }
                                                        }
                                                        sh_xfree(
                                                            value as *mut libc::c_void,
                                                            b"../subst.c\0" as *const u8
                                                                as *const libc::c_char,
                                                            9423 as libc::c_int,
                                                        );
                                                    }
                                                    current_block = 9668344364916463090;
                                                }
                                                _ => {}
                                            }
                                            match current_block {
                                                17370964531530178945 => {
                                                    if value.is_null()
                                                        || *value as libc::c_int == '\u{0}' as i32
                                                        || temp.is_null()
                                                        || *temp as libc::c_int == '\u{0}' as i32
                                                    {
                                                        if !value.is_null() {
                                                            sh_xfree(
                                                                value as *mut libc::c_void,
                                                                b"../subst.c\0" as *const u8
                                                                    as *const libc::c_char,
                                                                9323 as libc::c_int,
                                                            );
                                                        }
                                                    } else {
                                                        temp1 = parameter_brace_remove_pattern(
                                                            name,
                                                            temp,
                                                            ind as libc::c_int,
                                                            value,
                                                            c,
                                                            quoted,
                                                            if tflag
                                                                & (1 as libc::c_int)
                                                                    << 24 as libc::c_int
                                                                != 0
                                                            {
                                                                0x4 as libc::c_int
                                                            } else {
                                                                0 as libc::c_int
                                                            },
                                                        );
                                                        sh_xfree(
                                                            temp as *mut libc::c_void,
                                                            b"../subst.c\0" as *const u8
                                                                as *const libc::c_char,
                                                            9327 as libc::c_int,
                                                        );
                                                        sh_xfree(
                                                            value as *mut libc::c_void,
                                                            b"../subst.c\0" as *const u8
                                                                as *const libc::c_char,
                                                            9328 as libc::c_int,
                                                        );
                                                        ret = alloc_word_desc();
                                                        let ref mut fresh105 = (*ret).word;
                                                        *fresh105 = temp1;
                                                        if !temp1.is_null()
                                                            && (*temp1
                                                                .offset(0 as libc::c_int as isize)
                                                                as libc::c_int
                                                                == '\u{7f}' as i32
                                                                && *temp1.offset(
                                                                    1 as libc::c_int as isize,
                                                                )
                                                                    as libc::c_int
                                                                    == '\u{0}' as i32)
                                                            && quoted
                                                                & (0x2 as libc::c_int
                                                                    | 0x1 as libc::c_int)
                                                                != 0
                                                        {
                                                            (*ret).flags |= (1 as libc::c_int)
                                                                << 1 as libc::c_int
                                                                | (1 as libc::c_int)
                                                                    << 18 as libc::c_int;
                                                        } else if !temp1.is_null()
                                                            && (*name
                                                                .offset(0 as libc::c_int as isize)
                                                                as libc::c_int
                                                                == '*' as i32
                                                                && *name.offset(
                                                                    1 as libc::c_int as isize,
                                                                )
                                                                    as libc::c_int
                                                                    == 0 as libc::c_int)
                                                            && quoted == 0 as libc::c_int
                                                            && ifs_is_null != 0
                                                        {
                                                            (*ret).flags |= (1 as libc::c_int)
                                                                << 3 as libc::c_int;
                                                        }
                                                        sh_xfree(
                                                            name as *mut libc::c_void,
                                                            b"../subst.c\0" as *const u8
                                                                as *const libc::c_char,
                                                            9338 as libc::c_int,
                                                        );
                                                        return ret;
                                                    }
                                                }
                                                _ => {}
                                            }
                                            sh_xfree(
                                                name as *mut libc::c_void,
                                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                                9428 as libc::c_int,
                                            );
                                            if ret.is_null() {
                                                ret = alloc_word_desc();
                                                (*ret).flags = tflag;
                                                let ref mut fresh106 = (*ret).word;
                                                *fresh106 = temp;
                                            }
                                            return ret;
                                        }
                                        37 => {
                                            current_block = 17370964531530178945;
                                            match current_block {
                                                6396965539391641386 => {
                                                    temp1 = parameter_brace_transform(
                                                        name,
                                                        temp,
                                                        ind as libc::c_int,
                                                        value,
                                                        c,
                                                        quoted,
                                                        pflags,
                                                        if tflag
                                                            & (1 as libc::c_int)
                                                                << 24 as libc::c_int
                                                            != 0
                                                        {
                                                            0x4 as libc::c_int
                                                        } else {
                                                            0 as libc::c_int
                                                        },
                                                    );
                                                    sh_xfree(
                                                        temp as *mut libc::c_void,
                                                        b"../subst.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        9297 as libc::c_int,
                                                    );
                                                    sh_xfree(
                                                        value as *mut libc::c_void,
                                                        b"../subst.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        9298 as libc::c_int,
                                                    );
                                                    if temp1
                                                        == &mut expand_param_error
                                                            as *mut libc::c_char
                                                        || temp1
                                                            == &mut expand_param_fatal
                                                                as *mut libc::c_char
                                                    {
                                                        sh_xfree(
                                                            name as *mut libc::c_void,
                                                            b"../subst.c\0" as *const u8
                                                                as *const libc::c_char,
                                                            9302 as libc::c_int,
                                                        );
                                                        set_exit_status(1 as libc::c_int);
                                                        report_error(
                                                            dcgettext(
                                                                0 as *const libc::c_char,
                                                                b"%s: bad substitution\0"
                                                                    as *const u8
                                                                    as *const libc::c_char,
                                                                5 as libc::c_int,
                                                            ),
                                                            if !string.is_null() {
                                                                string
                                                            } else {
                                                                b"??\0" as *const u8
                                                                    as *const libc::c_char
                                                            },
                                                        );
                                                        return if temp1
                                                            == &mut expand_param_error
                                                                as *mut libc::c_char
                                                        {
                                                            &mut expand_wdesc_error
                                                        } else {
                                                            &mut expand_wdesc_fatal
                                                        };
                                                    }
                                                    ret = alloc_word_desc();
                                                    let ref mut fresh104 = (*ret).word;
                                                    *fresh104 = temp1;
                                                    if !temp1.is_null()
                                                        && (*temp1.offset(0 as libc::c_int as isize)
                                                            as libc::c_int
                                                            == '\u{7f}' as i32
                                                            && *temp1
                                                                .offset(1 as libc::c_int as isize)
                                                                as libc::c_int
                                                                == '\u{0}' as i32)
                                                        && quoted
                                                            & (0x2 as libc::c_int
                                                                | 0x1 as libc::c_int)
                                                            != 0
                                                    {
                                                        (*ret).flags |= (1 as libc::c_int)
                                                            << 1 as libc::c_int
                                                            | (1 as libc::c_int)
                                                                << 18 as libc::c_int;
                                                    } else if !temp1.is_null()
                                                        && (*name.offset(0 as libc::c_int as isize)
                                                            as libc::c_int
                                                            == '*' as i32
                                                            && *name
                                                                .offset(1 as libc::c_int as isize)
                                                                as libc::c_int
                                                                == 0 as libc::c_int)
                                                        && quoted == 0 as libc::c_int
                                                        && ifs_is_null != 0
                                                    {
                                                        (*ret).flags |=
                                                            (1 as libc::c_int) << 3 as libc::c_int;
                                                    }
                                                    sh_xfree(
                                                        name as *mut libc::c_void,
                                                        b"../subst.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        9316 as libc::c_int,
                                                    );
                                                    return ret;
                                                }
                                                6979992077050147181 => {
                                                    current_block = 17370964531530178945;
                                                }
                                                1951049175126603298 => {
                                                    if var_is_set != 0
                                                        && var_is_null == 0 as libc::c_int
                                                    {
                                                        if c == '+' as i32 {
                                                            if quoted
                                                                & (0x2 as libc::c_int
                                                                    | 0x1 as libc::c_int)
                                                                != 0
                                                                && !quoted_dollar_atp.is_null()
                                                            {
                                                                *quoted_dollar_atp =
                                                                    0 as libc::c_int;
                                                            }
                                                            if !contains_dollar_at.is_null() {
                                                                *contains_dollar_at =
                                                                    0 as libc::c_int;
                                                            }
                                                            if !temp.is_null() {
                                                                sh_xfree(
                                                                    temp as *mut libc::c_void,
                                                                    b"../subst.c\0" as *const u8
                                                                        as *const libc::c_char,
                                                                    9359 as libc::c_int,
                                                                );
                                                            }
                                                            if !value.is_null() {
                                                                if quoted
                                                                    & (0x2 as libc::c_int
                                                                        | 0x1 as libc::c_int)
                                                                    != 0
                                                                {
                                                                    quoted |= 0x80 as libc::c_int;
                                                                }
                                                                ret = parameter_brace_expand_rhs(
                                                                    name,
                                                                    value,
                                                                    c,
                                                                    quoted,
                                                                    pflags,
                                                                    quoted_dollar_atp,
                                                                    contains_dollar_at,
                                                                );
                                                                sh_xfree(
                                                                    value as *mut libc::c_void,
                                                                    b"../subst.c\0" as *const u8
                                                                        as *const libc::c_char,
                                                                    9374 as libc::c_int,
                                                                );
                                                            } else {
                                                                temp = 0 as *mut libc::c_void
                                                                    as *mut libc::c_char;
                                                            }
                                                        } else if !value.is_null() {
                                                            sh_xfree(
                                                                value as *mut libc::c_void,
                                                                b"../subst.c\0" as *const u8
                                                                    as *const libc::c_char,
                                                                9381 as libc::c_int,
                                                            );
                                                        }
                                                    } else {
                                                        if !temp.is_null() {
                                                            sh_xfree(
                                                                temp as *mut libc::c_void,
                                                                b"../subst.c\0" as *const u8
                                                                    as *const libc::c_char,
                                                                9387 as libc::c_int,
                                                            );
                                                        }
                                                        temp = 0 as *mut libc::c_void
                                                            as *mut libc::c_char;
                                                        if c == '=' as i32 && var_is_special != 0 {
                                                            set_exit_status(1 as libc::c_int);
                                                            report_error(
                                                            dcgettext(
                                                                0 as *const libc::c_char,
                                                                b"$%s: cannot assign in this way\0"
                                                                    as *const u8
                                                                    as *const libc::c_char,
                                                                5 as libc::c_int,
                                                            ),
                                                            name,
                                                        );
                                                            sh_xfree(
                                                                name as *mut libc::c_void,
                                                                b"../subst.c\0" as *const u8
                                                                    as *const libc::c_char,
                                                                9393 as libc::c_int,
                                                            );
                                                            sh_xfree(
                                                                value as *mut libc::c_void,
                                                                b"../subst.c\0" as *const u8
                                                                    as *const libc::c_char,
                                                                9394 as libc::c_int,
                                                            );
                                                            return &mut expand_wdesc_error;
                                                        } else {
                                                            if c == '?' as i32 {
                                                                parameter_brace_expand_error(
                                                                    name,
                                                                    value,
                                                                    check_nullness,
                                                                );
                                                                return if interactive_shell != 0 {
                                                                    &mut expand_wdesc_error
                                                                } else {
                                                                    &mut expand_wdesc_fatal
                                                                };
                                                            } else {
                                                                if c != '+' as i32 {
                                                                    if quoted
                                                                        & (0x2 as libc::c_int
                                                                            | 0x1 as libc::c_int)
                                                                        != 0
                                                                        && !quoted_dollar_atp
                                                                            .is_null()
                                                                    {
                                                                        *quoted_dollar_atp =
                                                                            0 as libc::c_int;
                                                                    }
                                                                    if !contains_dollar_at.is_null()
                                                                    {
                                                                        *contains_dollar_at =
                                                                            0 as libc::c_int;
                                                                    }
                                                                    if quoted
                                                                        & (0x2 as libc::c_int
                                                                            | 0x1 as libc::c_int)
                                                                        != 0
                                                                    {
                                                                        quoted |=
                                                                            0x80 as libc::c_int;
                                                                    }
                                                                    ret =
                                                                        parameter_brace_expand_rhs(
                                                                            name,
                                                                            value,
                                                                            c,
                                                                            quoted,
                                                                            pflags,
                                                                            quoted_dollar_atp,
                                                                            contains_dollar_at,
                                                                        );
                                                                }
                                                            }
                                                        }
                                                        sh_xfree(
                                                            value as *mut libc::c_void,
                                                            b"../subst.c\0" as *const u8
                                                                as *const libc::c_char,
                                                            9423 as libc::c_int,
                                                        );
                                                    }
                                                    current_block = 9668344364916463090;
                                                }
                                                _ => {}
                                            }
                                            match current_block {
                                                17370964531530178945 => {
                                                    if value.is_null()
                                                        || *value as libc::c_int == '\u{0}' as i32
                                                        || temp.is_null()
                                                        || *temp as libc::c_int == '\u{0}' as i32
                                                    {
                                                        if !value.is_null() {
                                                            sh_xfree(
                                                                value as *mut libc::c_void,
                                                                b"../subst.c\0" as *const u8
                                                                    as *const libc::c_char,
                                                                9323 as libc::c_int,
                                                            );
                                                        }
                                                    } else {
                                                        temp1 = parameter_brace_remove_pattern(
                                                            name,
                                                            temp,
                                                            ind as libc::c_int,
                                                            value,
                                                            c,
                                                            quoted,
                                                            if tflag
                                                                & (1 as libc::c_int)
                                                                    << 24 as libc::c_int
                                                                != 0
                                                            {
                                                                0x4 as libc::c_int
                                                            } else {
                                                                0 as libc::c_int
                                                            },
                                                        );
                                                        sh_xfree(
                                                            temp as *mut libc::c_void,
                                                            b"../subst.c\0" as *const u8
                                                                as *const libc::c_char,
                                                            9327 as libc::c_int,
                                                        );
                                                        sh_xfree(
                                                            value as *mut libc::c_void,
                                                            b"../subst.c\0" as *const u8
                                                                as *const libc::c_char,
                                                            9328 as libc::c_int,
                                                        );
                                                        ret = alloc_word_desc();
                                                        let ref mut fresh105 = (*ret).word;
                                                        *fresh105 = temp1;
                                                        if !temp1.is_null()
                                                            && (*temp1
                                                                .offset(0 as libc::c_int as isize)
                                                                as libc::c_int
                                                                == '\u{7f}' as i32
                                                                && *temp1.offset(
                                                                    1 as libc::c_int as isize,
                                                                )
                                                                    as libc::c_int
                                                                    == '\u{0}' as i32)
                                                            && quoted
                                                                & (0x2 as libc::c_int
                                                                    | 0x1 as libc::c_int)
                                                                != 0
                                                        {
                                                            (*ret).flags |= (1 as libc::c_int)
                                                                << 1 as libc::c_int
                                                                | (1 as libc::c_int)
                                                                    << 18 as libc::c_int;
                                                        } else if !temp1.is_null()
                                                            && (*name
                                                                .offset(0 as libc::c_int as isize)
                                                                as libc::c_int
                                                                == '*' as i32
                                                                && *name.offset(
                                                                    1 as libc::c_int as isize,
                                                                )
                                                                    as libc::c_int
                                                                    == 0 as libc::c_int)
                                                            && quoted == 0 as libc::c_int
                                                            && ifs_is_null != 0
                                                        {
                                                            (*ret).flags |= (1 as libc::c_int)
                                                                << 3 as libc::c_int;
                                                        }
                                                        sh_xfree(
                                                            name as *mut libc::c_void,
                                                            b"../subst.c\0" as *const u8
                                                                as *const libc::c_char,
                                                            9338 as libc::c_int,
                                                        );
                                                        return ret;
                                                    }
                                                }
                                                _ => {}
                                            }
                                            sh_xfree(
                                                name as *mut libc::c_void,
                                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                                9428 as libc::c_int,
                                            );
                                            if ret.is_null() {
                                                ret = alloc_word_desc();
                                                (*ret).flags = tflag;
                                                let ref mut fresh106 = (*ret).word;
                                                *fresh106 = temp;
                                            }
                                            return ret;
                                        }
                                        45 | 61 | 63 | 43 => {
                                            current_block = 1951049175126603298;
                                            match current_block {
                                                6396965539391641386 => {
                                                    temp1 = parameter_brace_transform(
                                                        name,
                                                        temp,
                                                        ind as libc::c_int,
                                                        value,
                                                        c,
                                                        quoted,
                                                        pflags,
                                                        if tflag
                                                            & (1 as libc::c_int)
                                                                << 24 as libc::c_int
                                                            != 0
                                                        {
                                                            0x4 as libc::c_int
                                                        } else {
                                                            0 as libc::c_int
                                                        },
                                                    );
                                                    sh_xfree(
                                                        temp as *mut libc::c_void,
                                                        b"../subst.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        9297 as libc::c_int,
                                                    );
                                                    sh_xfree(
                                                        value as *mut libc::c_void,
                                                        b"../subst.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        9298 as libc::c_int,
                                                    );
                                                    if temp1
                                                        == &mut expand_param_error
                                                            as *mut libc::c_char
                                                        || temp1
                                                            == &mut expand_param_fatal
                                                                as *mut libc::c_char
                                                    {
                                                        sh_xfree(
                                                            name as *mut libc::c_void,
                                                            b"../subst.c\0" as *const u8
                                                                as *const libc::c_char,
                                                            9302 as libc::c_int,
                                                        );
                                                        set_exit_status(1 as libc::c_int);
                                                        report_error(
                                                            dcgettext(
                                                                0 as *const libc::c_char,
                                                                b"%s: bad substitution\0"
                                                                    as *const u8
                                                                    as *const libc::c_char,
                                                                5 as libc::c_int,
                                                            ),
                                                            if !string.is_null() {
                                                                string
                                                            } else {
                                                                b"??\0" as *const u8
                                                                    as *const libc::c_char
                                                            },
                                                        );
                                                        return if temp1
                                                            == &mut expand_param_error
                                                                as *mut libc::c_char
                                                        {
                                                            &mut expand_wdesc_error
                                                        } else {
                                                            &mut expand_wdesc_fatal
                                                        };
                                                    }
                                                    ret = alloc_word_desc();
                                                    let ref mut fresh104 = (*ret).word;
                                                    *fresh104 = temp1;
                                                    if !temp1.is_null()
                                                        && (*temp1.offset(0 as libc::c_int as isize)
                                                            as libc::c_int
                                                            == '\u{7f}' as i32
                                                            && *temp1
                                                                .offset(1 as libc::c_int as isize)
                                                                as libc::c_int
                                                                == '\u{0}' as i32)
                                                        && quoted
                                                            & (0x2 as libc::c_int
                                                                | 0x1 as libc::c_int)
                                                            != 0
                                                    {
                                                        (*ret).flags |= (1 as libc::c_int)
                                                            << 1 as libc::c_int
                                                            | (1 as libc::c_int)
                                                                << 18 as libc::c_int;
                                                    } else if !temp1.is_null()
                                                        && (*name.offset(0 as libc::c_int as isize)
                                                            as libc::c_int
                                                            == '*' as i32
                                                            && *name
                                                                .offset(1 as libc::c_int as isize)
                                                                as libc::c_int
                                                                == 0 as libc::c_int)
                                                        && quoted == 0 as libc::c_int
                                                        && ifs_is_null != 0
                                                    {
                                                        (*ret).flags |=
                                                            (1 as libc::c_int) << 3 as libc::c_int;
                                                    }
                                                    sh_xfree(
                                                        name as *mut libc::c_void,
                                                        b"../subst.c\0" as *const u8
                                                            as *const libc::c_char,
                                                        9316 as libc::c_int,
                                                    );
                                                    return ret;
                                                }
                                                6979992077050147181 => {
                                                    current_block = 17370964531530178945;
                                                }
                                                1951049175126603298 => {
                                                    if var_is_set != 0
                                                        && var_is_null == 0 as libc::c_int
                                                    {
                                                        if c == '+' as i32 {
                                                            if quoted
                                                                & (0x2 as libc::c_int
                                                                    | 0x1 as libc::c_int)
                                                                != 0
                                                                && !quoted_dollar_atp.is_null()
                                                            {
                                                                *quoted_dollar_atp =
                                                                    0 as libc::c_int;
                                                            }
                                                            if !contains_dollar_at.is_null() {
                                                                *contains_dollar_at =
                                                                    0 as libc::c_int;
                                                            }
                                                            if !temp.is_null() {
                                                                sh_xfree(
                                                                    temp as *mut libc::c_void,
                                                                    b"../subst.c\0" as *const u8
                                                                        as *const libc::c_char,
                                                                    9359 as libc::c_int,
                                                                );
                                                            }
                                                            if !value.is_null() {
                                                                if quoted
                                                                    & (0x2 as libc::c_int
                                                                        | 0x1 as libc::c_int)
                                                                    != 0
                                                                {
                                                                    quoted |= 0x80 as libc::c_int;
                                                                }
                                                                ret = parameter_brace_expand_rhs(
                                                                    name,
                                                                    value,
                                                                    c,
                                                                    quoted,
                                                                    pflags,
                                                                    quoted_dollar_atp,
                                                                    contains_dollar_at,
                                                                );
                                                                sh_xfree(
                                                                    value as *mut libc::c_void,
                                                                    b"../subst.c\0" as *const u8
                                                                        as *const libc::c_char,
                                                                    9374 as libc::c_int,
                                                                );
                                                            } else {
                                                                temp = 0 as *mut libc::c_void
                                                                    as *mut libc::c_char;
                                                            }
                                                        } else if !value.is_null() {
                                                            sh_xfree(
                                                                value as *mut libc::c_void,
                                                                b"../subst.c\0" as *const u8
                                                                    as *const libc::c_char,
                                                                9381 as libc::c_int,
                                                            );
                                                        }
                                                    } else {
                                                        if !temp.is_null() {
                                                            sh_xfree(
                                                                temp as *mut libc::c_void,
                                                                b"../subst.c\0" as *const u8
                                                                    as *const libc::c_char,
                                                                9387 as libc::c_int,
                                                            );
                                                        }
                                                        temp = 0 as *mut libc::c_void
                                                            as *mut libc::c_char;
                                                        if c == '=' as i32 && var_is_special != 0 {
                                                            set_exit_status(1 as libc::c_int);
                                                            report_error(
                                                            dcgettext(
                                                                0 as *const libc::c_char,
                                                                b"$%s: cannot assign in this way\0"
                                                                    as *const u8
                                                                    as *const libc::c_char,
                                                                5 as libc::c_int,
                                                            ),
                                                            name,
                                                        );
                                                            sh_xfree(
                                                                name as *mut libc::c_void,
                                                                b"../subst.c\0" as *const u8
                                                                    as *const libc::c_char,
                                                                9393 as libc::c_int,
                                                            );
                                                            sh_xfree(
                                                                value as *mut libc::c_void,
                                                                b"../subst.c\0" as *const u8
                                                                    as *const libc::c_char,
                                                                9394 as libc::c_int,
                                                            );
                                                            return &mut expand_wdesc_error;
                                                        } else {
                                                            if c == '?' as i32 {
                                                                parameter_brace_expand_error(
                                                                    name,
                                                                    value,
                                                                    check_nullness,
                                                                );
                                                                return if interactive_shell != 0 {
                                                                    &mut expand_wdesc_error
                                                                } else {
                                                                    &mut expand_wdesc_fatal
                                                                };
                                                            } else {
                                                                if c != '+' as i32 {
                                                                    if quoted
                                                                        & (0x2 as libc::c_int
                                                                            | 0x1 as libc::c_int)
                                                                        != 0
                                                                        && !quoted_dollar_atp
                                                                            .is_null()
                                                                    {
                                                                        *quoted_dollar_atp =
                                                                            0 as libc::c_int;
                                                                    }
                                                                    if !contains_dollar_at.is_null()
                                                                    {
                                                                        *contains_dollar_at =
                                                                            0 as libc::c_int;
                                                                    }
                                                                    if quoted
                                                                        & (0x2 as libc::c_int
                                                                            | 0x1 as libc::c_int)
                                                                        != 0
                                                                    {
                                                                        quoted |=
                                                                            0x80 as libc::c_int;
                                                                    }
                                                                    ret =
                                                                        parameter_brace_expand_rhs(
                                                                            name,
                                                                            value,
                                                                            c,
                                                                            quoted,
                                                                            pflags,
                                                                            quoted_dollar_atp,
                                                                            contains_dollar_at,
                                                                        );
                                                                }
                                                            }
                                                        }
                                                        sh_xfree(
                                                            value as *mut libc::c_void,
                                                            b"../subst.c\0" as *const u8
                                                                as *const libc::c_char,
                                                            9423 as libc::c_int,
                                                        );
                                                    }
                                                    current_block = 9668344364916463090;
                                                }
                                                _ => {}
                                            }
                                            match current_block {
                                                17370964531530178945 => {
                                                    if value.is_null()
                                                        || *value as libc::c_int == '\u{0}' as i32
                                                        || temp.is_null()
                                                        || *temp as libc::c_int == '\u{0}' as i32
                                                    {
                                                        if !value.is_null() {
                                                            sh_xfree(
                                                                value as *mut libc::c_void,
                                                                b"../subst.c\0" as *const u8
                                                                    as *const libc::c_char,
                                                                9323 as libc::c_int,
                                                            );
                                                        }
                                                    } else {
                                                        temp1 = parameter_brace_remove_pattern(
                                                            name,
                                                            temp,
                                                            ind as libc::c_int,
                                                            value,
                                                            c,
                                                            quoted,
                                                            if tflag
                                                                & (1 as libc::c_int)
                                                                    << 24 as libc::c_int
                                                                != 0
                                                            {
                                                                0x4 as libc::c_int
                                                            } else {
                                                                0 as libc::c_int
                                                            },
                                                        );
                                                        sh_xfree(
                                                            temp as *mut libc::c_void,
                                                            b"../subst.c\0" as *const u8
                                                                as *const libc::c_char,
                                                            9327 as libc::c_int,
                                                        );
                                                        sh_xfree(
                                                            value as *mut libc::c_void,
                                                            b"../subst.c\0" as *const u8
                                                                as *const libc::c_char,
                                                            9328 as libc::c_int,
                                                        );
                                                        ret = alloc_word_desc();
                                                        let ref mut fresh105 = (*ret).word;
                                                        *fresh105 = temp1;
                                                        if !temp1.is_null()
                                                            && (*temp1
                                                                .offset(0 as libc::c_int as isize)
                                                                as libc::c_int
                                                                == '\u{7f}' as i32
                                                                && *temp1.offset(
                                                                    1 as libc::c_int as isize,
                                                                )
                                                                    as libc::c_int
                                                                    == '\u{0}' as i32)
                                                            && quoted
                                                                & (0x2 as libc::c_int
                                                                    | 0x1 as libc::c_int)
                                                                != 0
                                                        {
                                                            (*ret).flags |= (1 as libc::c_int)
                                                                << 1 as libc::c_int
                                                                | (1 as libc::c_int)
                                                                    << 18 as libc::c_int;
                                                        } else if !temp1.is_null()
                                                            && (*name
                                                                .offset(0 as libc::c_int as isize)
                                                                as libc::c_int
                                                                == '*' as i32
                                                                && *name.offset(
                                                                    1 as libc::c_int as isize,
                                                                )
                                                                    as libc::c_int
                                                                    == 0 as libc::c_int)
                                                            && quoted == 0 as libc::c_int
                                                            && ifs_is_null != 0
                                                        {
                                                            (*ret).flags |= (1 as libc::c_int)
                                                                << 3 as libc::c_int;
                                                        }
                                                        sh_xfree(
                                                            name as *mut libc::c_void,
                                                            b"../subst.c\0" as *const u8
                                                                as *const libc::c_char,
                                                            9338 as libc::c_int,
                                                        );
                                                        return ret;
                                                    }
                                                }
                                                _ => {}
                                            }
                                            sh_xfree(
                                                name as *mut libc::c_void,
                                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                                9428 as libc::c_int,
                                            );
                                            if ret.is_null() {
                                                ret = alloc_word_desc();
                                                (*ret).flags = tflag;
                                                let ref mut fresh106 = (*ret).word;
                                                *fresh106 = temp;
                                            }
                                            return ret;
                                        }
                                        0 | _ => {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        set_exit_status(1 as libc::c_int);
        report_error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: bad substitution\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            if !string.is_null() {
                string
            } else {
                b"??\0" as *const u8 as *const libc::c_char
            },
        );
        if !value.is_null() {
            sh_xfree(
                value as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                9284 as libc::c_int,
            );
        }
        if !temp.is_null() {
            sh_xfree(
                temp as *mut libc::c_void,
                b"../subst.c\0" as *const u8 as *const libc::c_char,
                9285 as libc::c_int,
            );
        }
        sh_xfree(
            name as *mut libc::c_void,
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            9286 as libc::c_int,
        );
        if shell_compatibility_level <= 43 as libc::c_int {
            return &mut expand_wdesc_error;
        } else {
            return if posixly_correct != 0 && interactive_shell == 0 as libc::c_int {
                &mut expand_wdesc_fatal
            } else {
                &mut expand_wdesc_error
            };
        };
    }
}
fn param_expand(
    mut string: *mut libc::c_char,
    mut sindex: *mut libc::c_int,
    mut quoted: libc::c_int,
    mut expanded_something: *mut libc::c_int,
    mut contains_dollar_at: *mut libc::c_int,
    mut quoted_dollar_at_p: *mut libc::c_int,
    mut had_quoted_null_p: *mut libc::c_int,
    mut pflags: libc::c_int,
) -> *mut WORD_DESC {
    let mut temp2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut current_block: u64;
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut uerror: [libc::c_char; 3] = [0; 3];
    let mut savecmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zindex: libc::c_int = 0;
    let mut t_index: libc::c_int = 0;
    let mut expok: libc::c_int = 0;
    let mut c: libc::c_uchar = 0;
    let mut number: intmax_t = 0;
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut list: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut l: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut tdesc: *mut WORD_DESC = 0 as *mut WORD_DESC;
    let mut ret: *mut WORD_DESC = 0 as *mut WORD_DESC;
    let mut tflag: libc::c_int = 0;
    let mut nullarg: libc::c_int = 0;
    let mut old_echo_input: libc::c_int = 0;
    unsafe {
        zindex = *sindex;
        zindex += 1;
        c = *string.offset(zindex as isize) as libc::c_uchar;
        temp = 0 as *mut libc::c_void as *mut libc::c_char;
        tdesc = 0 as *mut libc::c_void as *mut WORD_DESC;
        ret = tdesc;
        tflag = 0 as libc::c_int;
        match c as libc::c_int {
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                temp1 = *dollar_vars
                    .as_mut_ptr()
                    .offset((c as libc::c_int - '0' as i32) as isize);
                if unbound_vars_is_error != 0 && temp1.is_null() {
                    uerror[0 as libc::c_int as usize] = '$' as i32 as libc::c_char;
                    uerror[1 as libc::c_int as usize] = c as libc::c_char;
                    uerror[2 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
                    set_exit_status(1 as libc::c_int);
                    err_unboundvar(uerror.as_mut_ptr());
                    return if interactive_shell != 0 {
                        &mut expand_wdesc_error
                    } else {
                        &mut expand_wdesc_fatal
                    };
                }
                if !temp1.is_null() {
                    temp = if *temp1 as libc::c_int != 0
                        && quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0
                    {
                        quote_string(temp1)
                    } else {
                        quote_escapes(temp1)
                    };
                } else {
                    temp = 0 as *mut libc::c_void as *mut libc::c_char;
                }
                current_block = 14877673131977174631;
            }
            36 => {
                temp = itos(dollar_dollar_pid as intmax_t);
                current_block = 14877673131977174631;
            }
            35 => {
                temp = itos(number_of_args() as intmax_t);
                current_block = 14877673131977174631;
            }
            63 => {
                temp = itos(last_command_exit_value as intmax_t);
                current_block = 14877673131977174631;
            }
            45 => {
                temp = which_set_flags();
                current_block = 14877673131977174631;
            }
            33 => {
                if last_asynchronous_pid == -(1 as libc::c_int) {
                    if !expanded_something.is_null() {
                        *expanded_something = 0 as libc::c_int;
                    }
                    temp = 0 as *mut libc::c_void as *mut libc::c_char;
                    if unbound_vars_is_error != 0 && pflags & 0x2 as libc::c_int == 0 as libc::c_int
                    {
                        uerror[0 as libc::c_int as usize] = '$' as i32 as libc::c_char;
                        uerror[1 as libc::c_int as usize] = c as libc::c_char;
                        uerror[2 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
                        set_exit_status(1 as libc::c_int);
                        err_unboundvar(uerror.as_mut_ptr());
                        return if interactive_shell != 0 {
                            &mut expand_wdesc_error
                        } else {
                            &mut expand_wdesc_fatal
                        };
                    }
                } else {
                    temp = itos(last_asynchronous_pid as intmax_t);
                }
                current_block = 14877673131977174631;
            }
            42 => {
                list = list_rest_of_args();
                if quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0 && list.is_null() {
                    temp = 0 as *mut libc::c_void as *mut libc::c_char;
                } else if quoted & (0x2 as libc::c_int | 0x1 as libc::c_int | 0x8 as libc::c_int)
                    != 0
                {
                    temp = if quoted & (0x1 as libc::c_int | 0x8 as libc::c_int) != 0 {
                        string_list_dollar_star(list, quoted, 0 as libc::c_int)
                    } else {
                        string_list(list)
                    };
                    if !temp.is_null() {
                        temp1 = if quoted & 0x1 as libc::c_int != 0 {
                            quote_string(temp)
                        } else {
                            temp
                        };
                        if *temp as libc::c_int == 0 as libc::c_int {
                            tflag |= (1 as libc::c_int) << 18 as libc::c_int;
                        }
                        if temp != temp1 {
                            sh_xfree(
                                temp as *mut libc::c_void,
                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                9589 as libc::c_int,
                            );
                        }
                        temp = temp1;
                    }
                } else {
                    if expand_no_split_dollar_star != 0
                        && quoted == 0 as libc::c_int
                        && ifs_is_set == 0 as libc::c_int
                        && pflags & 0x8 as libc::c_int != 0
                    {
                        temp1 = string_list_dollar_star(list, quoted, pflags);
                        temp = if !temp1.is_null() {
                            quote_string(temp1)
                        } else {
                            temp1
                        };
                        if !temp1.is_null()
                            && *temp1 as libc::c_int == 0 as libc::c_int
                            && (*temp.offset(0 as libc::c_int as isize) as libc::c_int
                                == '\u{7f}' as i32
                                && *temp.offset(1 as libc::c_int as isize) as libc::c_int
                                    == '\u{0}' as i32)
                        {
                            tflag |= (1 as libc::c_int) << 21 as libc::c_int;
                        }
                        if !temp1.is_null() {
                            sh_xfree(
                                temp1 as *mut libc::c_void,
                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                9609 as libc::c_int,
                            );
                        }
                    } else if expand_no_split_dollar_star != 0
                        && quoted == 0 as libc::c_int
                        && ifs_is_null != 0
                        && pflags & 0x8 as libc::c_int != 0
                    {
                        temp1 = string_list_dollar_star(list, quoted, pflags);
                        temp = if !temp1.is_null() {
                            quote_escapes(temp1)
                        } else {
                            temp1
                        };
                        if !temp1.is_null() {
                            sh_xfree(
                                temp1 as *mut libc::c_void,
                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                9616 as libc::c_int,
                            );
                        }
                    } else if expand_no_split_dollar_star != 0
                        && quoted == 0 as libc::c_int
                        && ifs_is_set != 0
                        && ifs_is_null == 0 as libc::c_int
                        && pflags & 0x8 as libc::c_int != 0
                    {
                        temp1 = string_list_dollar_star(list, quoted, pflags);
                        temp = if !temp1.is_null() {
                            quote_string(temp1)
                        } else {
                            temp1
                        };
                        if !temp1.is_null()
                            && *temp1 as libc::c_int == 0 as libc::c_int
                            && (*temp.offset(0 as libc::c_int as isize) as libc::c_int
                                == '\u{7f}' as i32
                                && *temp.offset(1 as libc::c_int as isize) as libc::c_int
                                    == '\u{0}' as i32)
                        {
                            tflag |= (1 as libc::c_int) << 21 as libc::c_int;
                        }
                        if !temp1.is_null() {
                            sh_xfree(
                                temp1 as *mut libc::c_void,
                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                9627 as libc::c_int,
                            );
                        }
                    } else if expand_no_split_dollar_star != 0
                        && ifs_firstc[0 as libc::c_int as usize] as libc::c_int == 0 as libc::c_int
                    {
                        temp = string_list_dollar_star(list, quoted, 0 as libc::c_int);
                    } else {
                        temp = string_list_dollar_at(list, quoted, 0 as libc::c_int);
                        if quoted == 0 as libc::c_int && ifs_is_null != 0 {
                            tflag |= (1 as libc::c_int) << 3 as libc::c_int;
                        } else if !temp.is_null()
                            && quoted == 0 as libc::c_int
                            && ifs_is_set != 0
                            && pflags & 0x8 as libc::c_int != 0
                        {
                            temp1 = quote_string(temp);
                            sh_xfree(
                                temp as *mut libc::c_void,
                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                9654 as libc::c_int,
                            );
                            temp = temp1;
                        }
                    }
                    if expand_no_split_dollar_star == 0 as libc::c_int
                        && !contains_dollar_at.is_null()
                    {
                        *contains_dollar_at = 1 as libc::c_int;
                    }
                }
                dispose_words(list);
                current_block = 14877673131977174631;
            }
            64 => {
                list = list_rest_of_args();
                nullarg = 0 as libc::c_int;
                l = list;
                while !l.is_null() {
                    if !((*l).word).is_null()
                        && (((*(*l).word).word).is_null()
                            || *((*(*l).word).word).offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == 0 as libc::c_int)
                    {
                        nullarg = 1 as libc::c_int;
                    }
                    l = (*l).next;
                }
                if !quoted_dollar_at_p.is_null()
                    && quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0
                {
                    *quoted_dollar_at_p = 1 as libc::c_int;
                }
                if !contains_dollar_at.is_null() {
                    *contains_dollar_at = 1 as libc::c_int;
                }
                if pflags & 0x8 as libc::c_int != 0 {
                    temp = string_list_dollar_at(list, quoted | 0x1 as libc::c_int, pflags);
                    if nullarg != 0 {
                        tflag |= (1 as libc::c_int) << 18 as libc::c_int;
                    }
                } else if pflags & 0x4 as libc::c_int != 0 {
                    if quoted == 0 as libc::c_int
                        && ifs_is_set != 0
                        && ifs_is_null == 0 as libc::c_int
                        && ifs_firstc[0 as libc::c_int as usize] as libc::c_int != ' ' as i32
                    {
                        temp = string_list_dollar_at(list, 0x1 as libc::c_int, pflags);
                    } else {
                        temp = string_list_dollar_at(list, quoted, pflags);
                    }
                } else {
                    temp = string_list_dollar_at(list, quoted, pflags);
                }
                tflag |= (1 as libc::c_int) << 8 as libc::c_int;
                dispose_words(list);
                current_block = 14877673131977174631;
            }
            123 => {
                tdesc = parameter_brace_expand(
                    string,
                    &mut zindex,
                    quoted,
                    pflags,
                    quoted_dollar_at_p,
                    contains_dollar_at,
                );
                if tdesc == &mut expand_wdesc_error as *mut WORD_DESC
                    || tdesc == &mut expand_wdesc_fatal as *mut WORD_DESC
                {
                    return tdesc;
                }
                temp = if !tdesc.is_null() {
                    (*tdesc).word
                } else {
                    0 as *mut libc::c_char
                };
                if !tdesc.is_null()
                    && !((*tdesc).word).is_null()
                    && (*tdesc).flags & (1 as libc::c_int) << 18 as libc::c_int != 0
                    && (*temp.offset(0 as libc::c_int as isize) as libc::c_int == '\u{7f}' as i32
                        && *temp.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32)
                {
                    if !had_quoted_null_p.is_null() {
                        *had_quoted_null_p = 1 as libc::c_int;
                    }
                    if *quoted_dollar_at_p == 0 as libc::c_int {
                        sh_xfree(
                            temp as *mut libc::c_void,
                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                            9777 as libc::c_int,
                        );
                        temp = 0 as *mut libc::c_void as *mut libc::c_char;
                        let ref mut fresh107 = (*tdesc).word;
                        *fresh107 = temp;
                    }
                }
                ret = tdesc;
                current_block = 13100549023021289889;
            }
            40 => {
                t_index = zindex + 1 as libc::c_int;
                temp = extract_command_subst(
                    string,
                    &mut t_index,
                    if pflags & 0x10 as libc::c_int != 0 {
                        0x400 as libc::c_int
                    } else {
                        0 as libc::c_int
                    },
                );
                zindex = t_index;
                if !temp.is_null() && *temp as libc::c_int == '(' as i32 {
                    temp2 = 0 as *mut libc::c_char;
                    temp1 = temp.offset(1 as libc::c_int as isize);
                    temp2 = strcpy(
                        sh_xmalloc(
                            (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(temp1) as u64),
                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                            9801 as libc::c_int,
                        ) as *mut libc::c_char,
                        temp1,
                    );
                    t_index = (strlen(temp2)).wrapping_sub(1 as libc::c_int as u64) as libc::c_int;
                    if *temp2.offset(t_index as isize) as libc::c_int != ')' as i32 {
                        sh_xfree(
                            temp2 as *mut libc::c_void,
                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                            9806 as libc::c_int,
                        );
                        current_block = 8854118802046536135;
                    } else {
                        *temp2.offset(t_index as isize) = '\u{0}' as i32 as libc::c_char;
                        if chk_arithsub(temp2, t_index) == 0 as libc::c_int {
                            sh_xfree(
                                temp2 as *mut libc::c_void,
                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                9815 as libc::c_int,
                            );
                            current_block = 8854118802046536135;
                        } else {
                            temp1 = expand_arith_string(
                                temp2,
                                0x1 as libc::c_int | 0x100 as libc::c_int,
                            );
                            sh_xfree(
                                temp2 as *mut libc::c_void,
                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                9824 as libc::c_int,
                            );
                            current_block = 8017591428298988586;
                        }
                    }
                } else {
                    current_block = 8854118802046536135;
                }
                match current_block {
                    8017591428298988586 => {}
                    _ => {
                        old_echo_input = echo_input_at_read;
                        echo_input_at_read = 0 as libc::c_int;
                        if pflags & 0x1 as libc::c_int != 0 {
                            temp1 = substring(string, *sindex, zindex + 1 as libc::c_int);
                        } else {
                            tdesc = command_substitute(temp, quoted, pflags & 0x8 as libc::c_int);
                            temp1 = if !tdesc.is_null() {
                                (*tdesc).word
                            } else {
                                0 as *mut libc::c_void as *mut libc::c_char
                            };
                            if !tdesc.is_null() {
                                dispose_word_desc(tdesc);
                            }
                        }
                        if !temp.is_null() {
                            sh_xfree(
                                temp as *mut libc::c_void,
                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                9862 as libc::c_int,
                            );
                        }
                        temp = temp1;
                        echo_input_at_read = old_echo_input;
                        current_block = 14877673131977174631;
                    }
                }
            }
            91 => {
                t_index = zindex + 1 as libc::c_int;
                temp = extract_arithmetic_subst(string, &mut t_index);
                zindex = t_index;
                if temp.is_null() {
                    temp = strcpy(
                        sh_xmalloc(
                            (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(string) as u64),
                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                            9876 as libc::c_int,
                        ) as *mut libc::c_char,
                        string,
                    );
                    if !expanded_something.is_null() {
                        *expanded_something = 0 as libc::c_int;
                    }
                    current_block = 13100549023021289889;
                } else {
                    temp1 = expand_arith_string(temp, 0x1 as libc::c_int | 0x100 as libc::c_int);
                    current_block = 8017591428298988586;
                }
            }
            _ => {
                temp = 0 as *mut libc::c_void as *mut libc::c_char;
                t_index = zindex;
                loop {
                    c = *string.offset(zindex as isize) as libc::c_uchar;
                    if !(c as libc::c_int != 0
                        && (1 as libc::c_int != 0
                            && *(*__ctype_b_loc()).offset(c as libc::c_int as isize)
                                as libc::c_int
                                & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                            || c as libc::c_int == '_' as i32))
                    {
                        break;
                    }
                    zindex += 1;
                }
                temp1 = if zindex > t_index {
                    substring(string, t_index, zindex)
                } else {
                    0 as *mut libc::c_void as *mut libc::c_char
                };
                if temp1.is_null() || *temp1 as libc::c_int == '\u{0}' as i32 {
                    if !temp1.is_null() {
                        sh_xfree(
                            temp1 as *mut libc::c_void,
                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                            9898 as libc::c_int,
                        );
                    }
                    temp = sh_xmalloc(
                        2 as libc::c_int as size_t,
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        9899 as libc::c_int,
                    ) as *mut libc::c_char;
                    *temp.offset(0 as libc::c_int as isize) = '$' as i32 as libc::c_char;
                    *temp.offset(1 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
                    if !expanded_something.is_null() {
                        *expanded_something = 0 as libc::c_int;
                    }
                } else {
                    var = find_variable(temp1);
                    if !var.is_null()
                        && (*var).attributes & 0x1000 as libc::c_int == 0 as libc::c_int
                        && !((*var).value).is_null()
                    {
                        if (*var).attributes & 0x40 as libc::c_int != 0
                            || (*var).attributes & 0x4 as libc::c_int != 0
                        {
                            temp = if (*var).attributes & 0x4 as libc::c_int != 0 {
                                array_reference(
                                    (*var).value as *mut ARRAY,
                                    0 as libc::c_int as arrayind_t,
                                )
                            } else {
                                assoc_reference(
                                    (*var).value as *mut HASH_TABLE,
                                    b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                                )
                            };
                            if !temp.is_null() {
                                temp = if *temp as libc::c_int != 0
                                    && quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0
                                {
                                    quote_string(temp)
                                } else {
                                    quote_escapes(temp)
                                };
                                current_block = 8487579351791723214;
                            } else if unbound_vars_is_error != 0 {
                                current_block = 17199185463623757271;
                            } else {
                                current_block = 8487579351791723214;
                            }
                        } else {
                            temp = (*var).value;
                            temp = if *temp as libc::c_int != 0
                                && quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0
                            {
                                quote_string(temp)
                            } else if pflags & 0x8 as libc::c_int != 0 {
                                quote_rhs(temp)
                            } else {
                                quote_escapes(temp)
                            };
                            current_block = 8487579351791723214;
                        }
                        match current_block {
                            17199185463623757271 => {}
                            _ => {
                                sh_xfree(
                                    temp1 as *mut libc::c_void,
                                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                                    9935 as libc::c_int,
                                );
                                current_block = 13100549023021289889;
                            }
                        }
                    } else {
                        if !var.is_null()
                            && ((*var).attributes & 0x1000 as libc::c_int != 0
                                || ((*var).value != 0 as *mut libc::c_char) as libc::c_int
                                    == 0 as libc::c_int)
                        {
                            temp = 0 as *mut libc::c_void as *mut libc::c_char;
                            current_block = 5123298911185657293;
                        } else {
                            var = find_variable_last_nameref(temp1, 0 as libc::c_int);
                            if !var.is_null()
                                && !((*var).value).is_null()
                                && (*var).attributes & 0x1000 as libc::c_int == 0 as libc::c_int
                            {
                                temp = (*var).value;
                                if !temp.is_null()
                                    && *temp as libc::c_int != 0
                                    && valid_array_reference(temp, 0 as libc::c_int) != 0
                                {
                                    tdesc = parameter_brace_expand_word(
                                        temp,
                                        (*temp as libc::c_int != 0
                                            && (*temp as libc::c_int >= '0' as i32
                                                && *temp as libc::c_int <= '9' as i32
                                                && all_digits(temp) != 0
                                                || *temp.offset(1 as libc::c_int as isize)
                                                    as libc::c_int
                                                    == '\u{0}' as i32
                                                    && *sh_syntaxtab
                                                        .as_mut_ptr()
                                                        .offset(*temp as libc::c_uchar as isize)
                                                        & 0x800 as libc::c_int
                                                        != 0
                                                || 0 as libc::c_int != 0
                                                    && *temp.offset(2 as libc::c_int as isize)
                                                        as libc::c_int
                                                        == '\u{0}' as i32
                                                    && (posixly_correct == 0 as libc::c_int
                                                        && *temp.offset(1 as libc::c_int as isize)
                                                            as libc::c_int
                                                            == '#' as i32
                                                        || posixly_correct == 0 as libc::c_int
                                                            && *temp
                                                                .offset(1 as libc::c_int as isize)
                                                                as libc::c_int
                                                                == '?' as i32
                                                        || *temp.offset(1 as libc::c_int as isize)
                                                            as libc::c_int
                                                            == '@' as i32
                                                        || *temp.offset(1 as libc::c_int as isize)
                                                            as libc::c_int
                                                            == '*' as i32)))
                                            as libc::c_int,
                                        quoted,
                                        pflags,
                                        0 as *mut libc::c_void as *mut arrayind_t,
                                    );
                                    if tdesc == &mut expand_wdesc_error as *mut WORD_DESC
                                        || tdesc == &mut expand_wdesc_fatal as *mut WORD_DESC
                                    {
                                        return tdesc;
                                    }
                                    ret = tdesc;
                                    current_block = 13100549023021289889;
                                } else {
                                    if !temp.is_null()
                                        && *temp as libc::c_int != 0
                                        && legal_identifier(temp) == 0 as libc::c_int
                                    {
                                        set_exit_status(1 as libc::c_int);
                                        report_error(
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"%s: invalid variable name for name reference\0"
                                                    as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            temp,
                                        );
                                        return &mut expand_wdesc_error;
                                    } else {
                                        temp = 0 as *mut libc::c_void as *mut libc::c_char;
                                    }
                                    current_block = 5123298911185657293;
                                }
                            } else {
                                current_block = 5123298911185657293;
                            }
                        }
                        match current_block {
                            13100549023021289889 => {}
                            _ => {
                                temp = 0 as *mut libc::c_void as *mut libc::c_char;
                                current_block = 17199185463623757271;
                            }
                        }
                    }
                    match current_block {
                        13100549023021289889 => {}
                        _ => {
                            if unbound_vars_is_error != 0 {
                                set_exit_status(1 as libc::c_int);
                                err_unboundvar(temp1);
                                sh_xfree(
                                    temp1 as *mut libc::c_void,
                                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                                    9980 as libc::c_int,
                                );
                                set_exit_status(1 as libc::c_int);
                                return if unbound_vars_is_error != 0
                                    && interactive_shell == 0 as libc::c_int
                                {
                                    &mut expand_wdesc_fatal
                                } else {
                                    &mut expand_wdesc_error
                                };
                            } else {
                                sh_xfree(
                                    temp1 as *mut libc::c_void,
                                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                                    9976 as libc::c_int,
                                );
                            }
                        }
                    }
                }
                current_block = 13100549023021289889;
            }
        }
        match current_block {
            8017591428298988586 => {
                savecmd = this_command_name;
                this_command_name = 0 as *mut libc::c_void as *mut libc::c_char;
                number = evalexp(temp1, 0x1 as libc::c_int, &mut expok);
                this_command_name = savecmd;
                sh_xfree(
                    temp as *mut libc::c_void,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    9832 as libc::c_int,
                );
                sh_xfree(
                    temp1 as *mut libc::c_void,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    9833 as libc::c_int,
                );
                if expok == 0 as libc::c_int {
                    if interactive_shell == 0 as libc::c_int && posixly_correct != 0 {
                        set_exit_status(1 as libc::c_int);
                        return &mut expand_wdesc_fatal;
                    } else {
                        return &mut expand_wdesc_error;
                    }
                }
                temp = itos(number);
                current_block = 14877673131977174631;
            }
            _ => {}
        }
        match current_block {
            14877673131977174631 => {
                if *string.offset(zindex as isize) != 0 {
                    zindex += 1;
                }
            }
            _ => {}
        }
        *sindex = zindex;
        if ret.is_null() {
            ret = alloc_word_desc();
            (*ret).flags = tflag;
            let ref mut fresh108 = (*ret).word;
            *fresh108 = temp;
        }
    }
    return ret;
}
#[no_mangle]
pub fn invalidate_cached_quoted_dollar_at() {
    unsafe {
        dispose_words(cached_quoted_dollar_at);
        cached_quoted_dollar_at = 0 as *mut WORD_LIST;
    }
}
fn expand_word_internal(
    mut word: *mut WORD_DESC,
    mut quoted: libc::c_int,
    mut isexp: libc::c_int,
    mut contains_dollar_at: *mut libc::c_int,
    mut expanded_something: *mut libc::c_int,
) -> *mut WORD_LIST {
    unsafe {
        let mut ifs_chars: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut current_block: u64;
        let mut list: *mut WORD_LIST = 0 as *mut WORD_LIST;
        let mut tword: *mut WORD_DESC = 0 as *mut WORD_DESC;
        let mut istring: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut istring_size: size_t = 0;
        let mut istring_index: libc::c_int = 0;
        let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut temp1: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut string_size: size_t = 0;
        let mut sindex: libc::c_int = 0;
        let mut quoted_dollar_at: libc::c_int = 0;
        let mut quoted_state: libc::c_int = 0;
        let mut had_quoted_null: libc::c_int = 0;
        let mut has_quoted_ifs: libc::c_int = 0;
        let mut has_dollar_at: libc::c_int = 0;
        let mut temp_has_dollar_at: libc::c_int = 0;
        let mut split_on_spaces: libc::c_int = 0;
        let mut local_expanded: libc::c_int = 0;
        let mut tflag: libc::c_int = 0;
        let mut pflags: libc::c_int = 0;
        let mut mb_cur_max: libc::c_int = 0;
        let mut assignoff: libc::c_int = 0;
        let mut c: libc::c_uchar = 0;
        let mut t_index: libc::c_int = 0;
        let mut twochars: [libc::c_char; 2] = [0; 2];
        let mut state: mbstate_t = mbstate_t {
            __count: 0,
            __value: mbstate_t_value { __wch: 0 },
        };
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<mbstate_t>() as usize,
        );
        if *((*word).word).offset(0 as libc::c_int as isize) as libc::c_int
            == (*::std::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"\"$@\"\0"))
                [0 as libc::c_int as usize] as libc::c_int
            && strcmp(
                (*word).word,
                b"\"$@\"\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            && (*word).flags
                == (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
            && !(*dollar_vars.as_mut_ptr().offset(1 as libc::c_int as isize)).is_null()
        {
            if !contains_dollar_at.is_null() {
                *contains_dollar_at = 1 as libc::c_int;
            }
            if !expanded_something.is_null() {
                *expanded_something = 1 as libc::c_int;
            }
            if !cached_quoted_dollar_at.is_null() {
                return copy_word_list(cached_quoted_dollar_at);
            }
            list = list_rest_of_args();
            list = quote_list(list);
            cached_quoted_dollar_at = copy_word_list(list);
            return list;
        }
        istring_size = 112 as libc::c_int as size_t;
        istring = sh_xmalloc(
            istring_size,
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            10110 as libc::c_int,
        ) as *mut libc::c_char;
        istring_index = 0 as libc::c_int;
        *istring.offset(istring_index as isize) = '\u{0}' as i32 as libc::c_char;
        has_dollar_at = 0 as libc::c_int;
        had_quoted_null = has_dollar_at;
        quoted_dollar_at = had_quoted_null;
        has_quoted_ifs = 0 as libc::c_int;
        split_on_spaces = 0 as libc::c_int;
        quoted_state = 0 as libc::c_int;
        string = (*word).word;
        if !string.is_null() {
            mb_cur_max = __ctype_get_mb_cur_max() as libc::c_int;
            string_size = if mb_cur_max > 1 as libc::c_int {
                strlen(string) as u64
            } else {
                1 as libc::c_int as libc::c_ulong
            };
            if !contains_dollar_at.is_null() {
                *contains_dollar_at = 0 as libc::c_int;
            }
            assignoff = -(1 as libc::c_int);
            sindex = 0 as libc::c_int;
            loop {
                c = *string.offset(sindex as isize) as libc::c_uchar;
                match c as libc::c_int {
                    0 => {
                        break;
                    }
                    1 => {
                        sindex += 1;
                        if mb_cur_max > 1 as libc::c_int
                            && *string.offset(sindex as isize) as libc::c_int != 0
                        {
                            let mut i: libc::c_int = 0;
                            let mut state_bak: mbstate_t = mbstate_t {
                                __count: 0,
                                __value: mbstate_t_value { __wch: 0 },
                            };
                            let mut mblength: size_t = 0;
                            i = is_basic(*string.offset(sindex as isize));
                            if i != 0 {
                                mblength = 1 as libc::c_int as size_t;
                            } else if locale_utf8locale != 0
                                && *string.offset(sindex as isize) as libc::c_int
                                    & 0x80 as libc::c_int
                                    == 0 as libc::c_int
                            {
                                mblength = (*string.offset(sindex as isize) as libc::c_int
                                    != 0 as libc::c_int)
                                    as libc::c_int
                                    as size_t;
                            } else {
                                state_bak = state;
                                mblength = mbrlen(
                                    string.offset(sindex as isize),
                                    string_size.wrapping_sub(sindex as libc::c_ulong),
                                    &mut state,
                                );
                            }
                            if mblength == -(1 as libc::c_int) as size_t
                                || mblength == -(2 as libc::c_int) as size_t
                            {
                                state = state_bak;
                                mblength = 1 as libc::c_int as size_t;
                            }
                            if mblength < 1 as libc::c_int as libc::c_ulong {
                                mblength = 1 as libc::c_int as size_t;
                            }
                            temp = sh_xmalloc(
                                mblength.wrapping_add(2 as libc::c_int as libc::c_ulong),
                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                10148 as libc::c_int,
                            ) as *mut libc::c_char;
                            *temp.offset(0 as libc::c_int as isize) =
                                '\u{1}' as i32 as libc::c_char;
                            i = 0 as libc::c_int;
                            while (i as libc::c_ulong) < mblength {
                                let fresh109 = sindex;
                                sindex = sindex + 1;
                                *temp.offset((i + 1 as libc::c_int) as isize) =
                                    *string.offset(fresh109 as isize);
                                i += 1;
                            }
                            *temp
                                .offset(mblength.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    as isize) = '\u{0}' as i32 as libc::c_char;
                            current_block = 16465155782176963379;
                        } else {
                            temp = sh_xmalloc(
                                3 as libc::c_int as size_t,
                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                10153 as libc::c_int,
                            ) as *mut libc::c_char;
                            *temp.offset(0 as libc::c_int as isize) =
                                '\u{1}' as i32 as libc::c_char;
                            c = *string.offset(sindex as isize) as libc::c_uchar;
                            *temp.offset(1 as libc::c_int as isize) = c as libc::c_char;
                            *temp.offset(2 as libc::c_int as isize) =
                                '\u{0}' as i32 as libc::c_char;
                            current_block = 8423922909464824957;
                        }
                    }
                    60 | 62 => {
                        sindex += 1;
                        if *string.offset(sindex as isize) as libc::c_int != '(' as i32
                            || quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0
                            || (*word).flags
                                & ((1 as libc::c_int) << 19 as libc::c_int
                                    | (1 as libc::c_int) << 20 as libc::c_int)
                                != 0
                        {
                            sindex -= 1;
                            current_block = 1398496724659307859;
                        } else {
                            t_index = sindex + 1 as libc::c_int;
                            temp1 = extract_process_subst(
                                string,
                                (if c as libc::c_int == '<' as i32 {
                                    b"<(\0" as *const u8 as *const libc::c_char
                                } else {
                                    b">(\0" as *const u8 as *const libc::c_char
                                }) as *mut libc::c_char,
                                &mut t_index,
                                0 as libc::c_int,
                            );
                            sindex = t_index;
                            temp = if !temp1.is_null() {
                                process_substitute(
                                    temp1,
                                    (c as libc::c_int == '>' as i32) as libc::c_int,
                                )
                            } else {
                                0 as *mut libc::c_char
                            };
                            if !temp1.is_null() {
                                sh_xfree(
                                    temp1 as *mut libc::c_void,
                                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                                    10196 as libc::c_int,
                                );
                            }
                            current_block = 8423922909464824957;
                        }
                    }
                    61 => {
                        if (*word).flags
                            & ((1 as libc::c_int) << 11 as libc::c_int
                                | (1 as libc::c_int) << 12 as libc::c_int)
                            != 0
                        {
                            if isexp == 0 as libc::c_int
                                && (*word).flags
                                    & ((1 as libc::c_int) << 4 as libc::c_int
                                        | (1 as libc::c_int) << 6 as libc::c_int)
                                    == 0 as libc::c_int
                                && ifs_cmap[c as usize] as libc::c_int != 0 as libc::c_int
                            {
                                current_block = 8014490948552223808;
                            } else {
                                current_block = 1398496724659307859;
                            }
                        } else {
                            if (*word).flags & (1 as libc::c_int) << 2 as libc::c_int != 0
                                && (posixly_correct == 0 as libc::c_int
                                    || (*word).flags & (1 as libc::c_int) << 7 as libc::c_int != 0)
                                && assignoff == -(1 as libc::c_int)
                                && sindex > 0 as libc::c_int
                            {
                                assignoff = sindex;
                            }
                            if sindex == assignoff
                                && *string.offset((sindex + 1 as libc::c_int) as isize)
                                    as libc::c_int
                                    == '~' as i32
                            {
                                (*word).flags |= (1 as libc::c_int) << 13 as libc::c_int;
                            }
                            if (*word).flags & (1 as libc::c_int) << 17 as libc::c_int != 0 {
                                (*word).flags |= (1 as libc::c_int) << 11 as libc::c_int;
                            }
                            if isexp == 0 as libc::c_int
                                && (*word).flags
                                    & ((1 as libc::c_int) << 4 as libc::c_int
                                        | (1 as libc::c_int) << 6 as libc::c_int)
                                    == 0 as libc::c_int
                                && ifs_cmap[c as usize] as libc::c_int != 0 as libc::c_int
                            {
                                has_quoted_ifs += 1;
                                current_block = 8014490948552223808;
                            } else {
                                current_block = 1398496724659307859;
                            }
                        }
                    }
                    58 => {
                        if (*word).flags
                            & ((1 as libc::c_int) << 12 as libc::c_int
                                | (1 as libc::c_int) << 29 as libc::c_int)
                            != 0
                        {
                            if isexp == 0 as libc::c_int
                                && (*word).flags
                                    & ((1 as libc::c_int) << 4 as libc::c_int
                                        | (1 as libc::c_int) << 6 as libc::c_int)
                                    == 0 as libc::c_int
                                && ifs_cmap[c as usize] as libc::c_int != 0 as libc::c_int
                            {
                                current_block = 8014490948552223808;
                            } else {
                                current_block = 1398496724659307859;
                            }
                        } else {
                            if (*word).flags
                                & ((1 as libc::c_int) << 2 as libc::c_int
                                    | (1 as libc::c_int) << 11 as libc::c_int)
                                != 0
                                && (posixly_correct == 0 as libc::c_int
                                    || (*word).flags & (1 as libc::c_int) << 7 as libc::c_int != 0)
                                && *string.offset((sindex + 1 as libc::c_int) as isize)
                                    as libc::c_int
                                    == '~' as i32
                            {
                                (*word).flags |= (1 as libc::c_int) << 13 as libc::c_int;
                            }
                            if isexp == 0 as libc::c_int
                                && (*word).flags
                                    & ((1 as libc::c_int) << 4 as libc::c_int
                                        | (1 as libc::c_int) << 6 as libc::c_int)
                                    == 0 as libc::c_int
                                && ifs_cmap[c as usize] as libc::c_int != 0 as libc::c_int
                            {
                                current_block = 8014490948552223808;
                            } else {
                                current_block = 1398496724659307859;
                            }
                        }
                    }
                    126 => {
                        if (*word).flags
                            & ((1 as libc::c_int) << 12 as libc::c_int
                                | (1 as libc::c_int) << 19 as libc::c_int)
                            != 0
                            || sindex > 0 as libc::c_int
                                && (*word).flags & (1 as libc::c_int) << 13 as libc::c_int
                                    == 0 as libc::c_int
                            || quoted & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0
                        {
                            (*word).flags &= !((1 as libc::c_int) << 13 as libc::c_int);
                            if isexp == 0 as libc::c_int
                                && (*word).flags
                                    & ((1 as libc::c_int) << 4 as libc::c_int
                                        | (1 as libc::c_int) << 6 as libc::c_int)
                                    == 0 as libc::c_int
                                && ifs_cmap[c as usize] as libc::c_int != 0 as libc::c_int
                                && quoted & (0x1 as libc::c_int | 0x2 as libc::c_int)
                                    == 0 as libc::c_int
                            {
                                current_block = 8014490948552223808;
                            } else {
                                current_block = 1398496724659307859;
                            }
                        } else {
                            if (*word).flags & (1 as libc::c_int) << 11 as libc::c_int != 0 {
                                tflag = 2 as libc::c_int;
                            } else if (*word).flags
                                & ((1 as libc::c_int) << 2 as libc::c_int
                                    | (1 as libc::c_int) << 7 as libc::c_int)
                                != 0
                            {
                                tflag = 1 as libc::c_int;
                            } else {
                                tflag = 0 as libc::c_int;
                            }
                            temp = bash_tilde_find_word(
                                string.offset(sindex as isize),
                                tflag,
                                &mut t_index,
                            );
                            (*word).flags &= !((1 as libc::c_int) << 13 as libc::c_int);
                            if !temp.is_null()
                                && *temp as libc::c_int != 0
                                && t_index > 0 as libc::c_int
                            {
                                temp1 = bash_tilde_expand(temp, tflag);
                                if !temp1.is_null()
                                    && *temp1 as libc::c_int == '~' as i32
                                    && (*temp.offset(0 as libc::c_int as isize) as libc::c_int
                                        == *temp1.offset(0 as libc::c_int as isize) as libc::c_int
                                        && strcmp(temp, temp1) == 0 as libc::c_int)
                                {
                                    if !temp.is_null() {
                                        sh_xfree(
                                            temp as *mut libc::c_void,
                                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                                            10295 as libc::c_int,
                                        );
                                    }
                                    if !temp1.is_null() {
                                        sh_xfree(
                                            temp1 as *mut libc::c_void,
                                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                                            10296 as libc::c_int,
                                        );
                                    }
                                    current_block = 1398496724659307859;
                                } else {
                                    sh_xfree(
                                        temp as *mut libc::c_void,
                                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                                        10299 as libc::c_int,
                                    );
                                    temp = temp1;
                                    sindex += t_index;
                                    current_block = 10574327403355912886;
                                }
                            } else {
                                if !temp.is_null() {
                                    sh_xfree(
                                        temp as *mut libc::c_void,
                                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                                        10306 as libc::c_int,
                                    );
                                }
                                current_block = 1398496724659307859;
                            }
                        }
                    }
                    36 => {
                        if !expanded_something.is_null() {
                            *expanded_something = 1 as libc::c_int;
                        }
                        local_expanded = 1 as libc::c_int;
                        temp_has_dollar_at = 0 as libc::c_int;
                        pflags = if (*word).flags & (1 as libc::c_int) << 10 as libc::c_int != 0 {
                            0x1 as libc::c_int
                        } else {
                            0 as libc::c_int
                        };
                        if (*word).flags & (1 as libc::c_int) << 6 as libc::c_int != 0 {
                            pflags |= 0x4 as libc::c_int;
                        }
                        if (*word).flags & (1 as libc::c_int) << 11 as libc::c_int != 0 {
                            pflags |= 0x8 as libc::c_int;
                        }
                        if (*word).flags & (1 as libc::c_int) << 27 as libc::c_int != 0 {
                            pflags |= 0x10 as libc::c_int;
                        }
                        tword = param_expand(
                            string,
                            &mut sindex,
                            quoted,
                            expanded_something,
                            &mut temp_has_dollar_at,
                            &mut quoted_dollar_at,
                            &mut had_quoted_null,
                            pflags,
                        );
                        has_dollar_at += temp_has_dollar_at;
                        split_on_spaces += (*tword).flags & (1 as libc::c_int) << 3 as libc::c_int;
                        if tword == &mut expand_wdesc_error as *mut WORD_DESC
                            || tword == &mut expand_wdesc_fatal as *mut WORD_DESC
                        {
                            sh_xfree(
                                string as *mut libc::c_void,
                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                10332 as libc::c_int,
                            );
                            sh_xfree(
                                istring as *mut libc::c_void,
                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                10333 as libc::c_int,
                            );
                            return if tword == &mut expand_wdesc_error as *mut WORD_DESC {
                                &mut expand_word_error
                            } else {
                                &mut expand_word_fatal
                            };
                        }
                        if !contains_dollar_at.is_null() && has_dollar_at != 0 {
                            *contains_dollar_at = 1 as libc::c_int;
                        }
                        if !tword.is_null()
                            && (*tword).flags & (1 as libc::c_int) << 18 as libc::c_int != 0
                        {
                            had_quoted_null = 1 as libc::c_int;
                        }
                        if !tword.is_null()
                            && (*tword).flags & (1 as libc::c_int) << 21 as libc::c_int != 0
                        {
                            had_quoted_null = 1 as libc::c_int;
                        }
                        temp = if !tword.is_null() {
                            (*tword).word
                        } else {
                            0 as *mut libc::c_void as *mut libc::c_char
                        };
                        dispose_word_desc(tword);
                        if had_quoted_null != 0
                            && !temp.is_null()
                            && (*temp.offset(0 as libc::c_int as isize) as libc::c_int
                                == '\u{7f}' as i32
                                && *temp.offset(1 as libc::c_int as isize) as libc::c_int
                                    == '\u{0}' as i32)
                        {
                            if !temp.is_null() {
                                sh_xfree(
                                    temp as *mut libc::c_void,
                                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                                    10352 as libc::c_int,
                                );
                            }
                            temp = 0 as *mut libc::c_void as *mut libc::c_char;
                        }
                        current_block = 16465155782176963379;
                    }
                    96 => {
                        let fresh110 = sindex;
                        sindex = sindex + 1;
                        t_index = fresh110;
                        temp = string_extract(
                            string,
                            &mut sindex,
                            b"`\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            0x4 as libc::c_int,
                        );
                        if temp == &mut extract_string_error as *mut libc::c_char
                            || temp == &mut extract_string_fatal as *mut libc::c_char
                        {
                            if sindex - 1 as libc::c_int == t_index {
                                sindex = t_index;
                            } else {
                                set_exit_status(1 as libc::c_int);
                                report_error(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"bad substitution: no closing \"`\" in %s\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    string.offset(t_index as isize),
                                );
                                sh_xfree(
                                    string as *mut libc::c_void,
                                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                                    10375 as libc::c_int,
                                );
                                sh_xfree(
                                    istring as *mut libc::c_void,
                                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                                    10376 as libc::c_int,
                                );
                                return if temp == &mut extract_string_error as *mut libc::c_char {
                                    &mut expand_word_error
                                } else {
                                    &mut expand_word_fatal
                                };
                            }
                            current_block = 1398496724659307859;
                        } else {
                            if !expanded_something.is_null() {
                                *expanded_something = 1 as libc::c_int;
                            }
                            local_expanded = 1 as libc::c_int;
                            if (*word).flags & (1 as libc::c_int) << 10 as libc::c_int != 0 {
                                temp1 = substring(string, t_index, sindex + 1 as libc::c_int);
                            } else {
                                de_backslash(temp);
                                tword = command_substitute(temp, quoted, 0 as libc::c_int);
                                temp1 = if !tword.is_null() {
                                    (*tword).word
                                } else {
                                    0 as *mut libc::c_void as *mut libc::c_char
                                };
                                if !tword.is_null() {
                                    dispose_word_desc(tword);
                                }
                            }
                            if !temp.is_null() {
                                sh_xfree(
                                    temp as *mut libc::c_void,
                                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                                    10396 as libc::c_int,
                                );
                            }
                            temp = temp1;
                            current_block = 8423922909464824957;
                        }
                    }
                    92 => {
                        if *string.offset((sindex + 1 as libc::c_int) as isize) as libc::c_int
                            == '\n' as i32
                        {
                            sindex += 2 as libc::c_int;
                            continue;
                        } else {
                            sindex += 1;
                            c = *string.offset(sindex as isize) as libc::c_uchar;
                            if quoted & 0x2 as libc::c_int != 0
                                && quoted & 0x80 as libc::c_int != 0
                                && c as libc::c_int == '"' as i32
                            {
                                tflag = 0x40 as libc::c_int;
                            } else if quoted & 0x2 as libc::c_int != 0 {
                                tflag = 0x80 as libc::c_int;
                            } else if quoted & 0x1 as libc::c_int != 0 {
                                tflag = 0x40 as libc::c_int;
                            } else {
                                tflag = 0 as libc::c_int;
                            }
                            if quoted & 0x80 as libc::c_int != 0 && c as libc::c_int == '}' as i32 {
                                if locale_mb_cur_max > 1 as libc::c_int {
                                    let mut state_bak_0: mbstate_t = mbstate_t {
                                        __count: 0,
                                        __value: mbstate_t_value { __wch: 0 },
                                    };
                                    let mut mblength_0: size_t = 0;
                                    let mut _i: libc::c_int = 0;
                                    _i = is_basic(*string.offset(sindex as isize));
                                    if _i != 0 {
                                        mblength_0 = 1 as libc::c_int as size_t;
                                    } else {
                                        state_bak_0 = state;
                                        mblength_0 = mbrlen(
                                            string.offset(sindex as isize),
                                            string_size.wrapping_sub(sindex as libc::c_ulong),
                                            &mut state,
                                        );
                                    }
                                    if mblength_0 == -(2 as libc::c_int) as size_t
                                        || mblength_0 == -(1 as libc::c_int) as size_t
                                    {
                                        state = state_bak_0;
                                        mblength_0 = 1 as libc::c_int as size_t;
                                    } else {
                                        mblength_0 =
                                            if mblength_0 < 1 as libc::c_int as libc::c_ulong {
                                                1 as libc::c_int as libc::c_ulong
                                            } else {
                                                mblength_0
                                            };
                                    }
                                    temp = sh_xmalloc(
                                        mblength_0.wrapping_add(2 as libc::c_int as libc::c_ulong),
                                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                                        10426 as libc::c_int,
                                    )
                                        as *mut libc::c_char;
                                    *temp.offset(0 as libc::c_int as isize) =
                                        '\u{1}' as i32 as libc::c_char;
                                    _i = 0 as libc::c_int;
                                    while (_i as libc::c_ulong) < mblength_0 {
                                        let fresh111 = sindex;
                                        sindex = sindex + 1;
                                        *temp.offset((_i + 1 as libc::c_int) as isize) =
                                            *string.offset(fresh111 as isize);
                                        _i += 1;
                                    }
                                    *temp.offset(
                                        mblength_0.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = '\u{0}' as i32 as libc::c_char;
                                    current_block = 16465155782176963379;
                                } else {
                                    twochars[0 as libc::c_int as usize] =
                                        '\u{1}' as i32 as libc::c_char;
                                    twochars[1 as libc::c_int as usize] = c as libc::c_char;
                                    current_block = 3893156993890508670;
                                }
                            } else if quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0
                                && *sh_syntaxtab.as_mut_ptr().offset(c as isize) & tflag
                                    == 0 as libc::c_int
                                && isexp == 0 as libc::c_int
                                && ifs_cmap[c as usize] as libc::c_int != 0 as libc::c_int
                            {
                                if (istring_index + 2 as libc::c_int) as libc::c_ulong
                                    >= istring_size
                                {
                                    while (istring_index + 2 as libc::c_int) as libc::c_ulong
                                        >= istring_size
                                    {
                                        istring_size = (istring_size as libc::c_ulong)
                                            .wrapping_add(128 as libc::c_int as libc::c_ulong)
                                            as size_t
                                            as size_t;
                                    }
                                    istring = sh_xrealloc(
                                        istring as *mut libc::c_void,
                                        istring_size,
                                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                                        10432 as libc::c_int,
                                    )
                                        as *mut libc::c_char;
                                }
                                let fresh112 = istring_index;
                                istring_index = istring_index + 1;
                                *istring.offset(fresh112 as isize) = '\u{1}' as i32 as libc::c_char;
                                let fresh113 = istring_index;
                                istring_index = istring_index + 1;
                                *istring.offset(fresh113 as isize) = '\\' as i32 as libc::c_char;
                                *istring.offset(istring_index as isize) =
                                    '\u{0}' as i32 as libc::c_char;
                                if locale_mb_cur_max > 1 as libc::c_int {
                                    let mut state_bak_1: mbstate_t = mbstate_t {
                                        __count: 0,
                                        __value: mbstate_t_value { __wch: 0 },
                                    };
                                    let mut mblength_1: size_t = 0;
                                    let mut _i_0: libc::c_int = 0;
                                    _i_0 = is_basic(*string.offset(sindex as isize));
                                    if _i_0 != 0 {
                                        mblength_1 = 1 as libc::c_int as size_t;
                                    } else {
                                        state_bak_1 = state;
                                        mblength_1 = mbrlen(
                                            string.offset(sindex as isize),
                                            string_size.wrapping_sub(sindex as libc::c_ulong),
                                            &mut state,
                                        );
                                    }
                                    if mblength_1 == -(2 as libc::c_int) as size_t
                                        || mblength_1 == -(1 as libc::c_int) as size_t
                                    {
                                        state = state_bak_1;
                                        mblength_1 = 1 as libc::c_int as size_t;
                                    } else {
                                        mblength_1 =
                                            if mblength_1 < 1 as libc::c_int as libc::c_ulong {
                                                1 as libc::c_int as libc::c_ulong
                                            } else {
                                                mblength_1
                                            };
                                    }
                                    temp = sh_xmalloc(
                                        mblength_1.wrapping_add(2 as libc::c_int as libc::c_ulong),
                                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                                        10437 as libc::c_int,
                                    )
                                        as *mut libc::c_char;
                                    *temp.offset(0 as libc::c_int as isize) =
                                        '\u{1}' as i32 as libc::c_char;
                                    _i_0 = 0 as libc::c_int;
                                    while (_i_0 as libc::c_ulong) < mblength_1 {
                                        let fresh114 = sindex;
                                        sindex = sindex + 1;
                                        *temp.offset((_i_0 + 1 as libc::c_int) as isize) =
                                            *string.offset(fresh114 as isize);
                                        _i_0 += 1;
                                    }
                                    *temp.offset(
                                        mblength_1.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = '\u{0}' as i32 as libc::c_char;
                                    current_block = 16465155782176963379;
                                } else {
                                    twochars[0 as libc::c_int as usize] =
                                        '\u{1}' as i32 as libc::c_char;
                                    twochars[1 as libc::c_int as usize] = c as libc::c_char;
                                    current_block = 3893156993890508670;
                                }
                            } else if quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0
                                && c as libc::c_int == 0 as libc::c_int
                            {
                                if (istring_index + 2 as libc::c_int) as libc::c_ulong
                                    >= istring_size
                                {
                                    while (istring_index + 2 as libc::c_int) as libc::c_ulong
                                        >= istring_size
                                    {
                                        istring_size = (istring_size as libc::c_ulong)
                                            .wrapping_add(128 as libc::c_int as libc::c_ulong)
                                            as size_t
                                            as size_t;
                                    }
                                    istring = sh_xrealloc(
                                        istring as *mut libc::c_void,
                                        istring_size,
                                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                                        10442 as libc::c_int,
                                    )
                                        as *mut libc::c_char;
                                }
                                let fresh115 = istring_index;
                                istring_index = istring_index + 1;
                                *istring.offset(fresh115 as isize) = '\u{1}' as i32 as libc::c_char;
                                let fresh116 = istring_index;
                                istring_index = istring_index + 1;
                                *istring.offset(fresh116 as isize) = '\\' as i32 as libc::c_char;
                                *istring.offset(istring_index as isize) =
                                    '\u{0}' as i32 as libc::c_char;
                                continue;
                            } else if quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0
                                && *sh_syntaxtab.as_mut_ptr().offset(c as isize) & tflag
                                    == 0 as libc::c_int
                            {
                                if locale_mb_cur_max > 1 as libc::c_int {
                                    let mut state_bak_2: mbstate_t = mbstate_t {
                                        __count: 0,
                                        __value: mbstate_t_value { __wch: 0 },
                                    };
                                    let mut mblength_2: size_t = 0;
                                    let mut _i_1: libc::c_int = 0;
                                    _i_1 = is_basic(*string.offset(sindex as isize));
                                    if _i_1 != 0 {
                                        mblength_2 = 1 as libc::c_int as size_t;
                                    } else {
                                        state_bak_2 = state;
                                        mblength_2 = mbrlen(
                                            string.offset(sindex as isize),
                                            string_size.wrapping_sub(sindex as libc::c_ulong),
                                            &mut state,
                                        );
                                    }
                                    if mblength_2 == -(2 as libc::c_int) as size_t
                                        || mblength_2 == -(1 as libc::c_int) as size_t
                                    {
                                        state = state_bak_2;
                                        mblength_2 = 1 as libc::c_int as size_t;
                                    } else {
                                        mblength_2 =
                                            if mblength_2 < 1 as libc::c_int as libc::c_ulong {
                                                1 as libc::c_int as libc::c_ulong
                                            } else {
                                                mblength_2
                                            };
                                    }
                                    temp = sh_xmalloc(
                                        mblength_2.wrapping_add(2 as libc::c_int as libc::c_ulong),
                                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                                        10450 as libc::c_int,
                                    )
                                        as *mut libc::c_char;
                                    *temp.offset(0 as libc::c_int as isize) =
                                        '\\' as i32 as libc::c_char;
                                    _i_1 = 0 as libc::c_int;
                                    while (_i_1 as libc::c_ulong) < mblength_2 {
                                        let fresh117 = sindex;
                                        sindex = sindex + 1;
                                        *temp.offset((_i_1 + 1 as libc::c_int) as isize) =
                                            *string.offset(fresh117 as isize);
                                        _i_1 += 1;
                                    }
                                    *temp.offset(
                                        mblength_2.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = '\u{0}' as i32 as libc::c_char;
                                    current_block = 16465155782176963379;
                                } else {
                                    twochars[0 as libc::c_int as usize] =
                                        '\\' as i32 as libc::c_char;
                                    twochars[1 as libc::c_int as usize] = c as libc::c_char;
                                    current_block = 3893156993890508670;
                                }
                            } else if c as libc::c_int == 0 as libc::c_int {
                                c = '\u{7f}' as i32 as libc::c_uchar;
                                sindex -= 1;
                                current_block = 1398496724659307859;
                            } else if locale_mb_cur_max > 1 as libc::c_int {
                                let mut state_bak_3: mbstate_t = mbstate_t {
                                    __count: 0,
                                    __value: mbstate_t_value { __wch: 0 },
                                };
                                let mut mblength_3: size_t = 0;
                                let mut _i_2: libc::c_int = 0;
                                _i_2 = is_basic(*string.offset(sindex as isize));
                                if _i_2 != 0 {
                                    mblength_3 = 1 as libc::c_int as size_t;
                                } else {
                                    state_bak_3 = state;
                                    mblength_3 = mbrlen(
                                        string.offset(sindex as isize),
                                        string_size.wrapping_sub(sindex as libc::c_ulong),
                                        &mut state,
                                    );
                                }
                                if mblength_3 == -(2 as libc::c_int) as size_t
                                    || mblength_3 == -(1 as libc::c_int) as size_t
                                {
                                    state = state_bak_3;
                                    mblength_3 = 1 as libc::c_int as size_t;
                                } else {
                                    mblength_3 = if mblength_3 < 1 as libc::c_int as libc::c_ulong {
                                        1 as libc::c_int as libc::c_ulong
                                    } else {
                                        mblength_3
                                    };
                                }
                                temp = sh_xmalloc(
                                    mblength_3.wrapping_add(2 as libc::c_int as libc::c_ulong),
                                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                                    10460 as libc::c_int,
                                ) as *mut libc::c_char;
                                *temp.offset(0 as libc::c_int as isize) =
                                    '\u{1}' as i32 as libc::c_char;
                                _i_2 = 0 as libc::c_int;
                                while (_i_2 as libc::c_ulong) < mblength_3 {
                                    let fresh118 = sindex;
                                    sindex = sindex + 1;
                                    *temp.offset((_i_2 + 1 as libc::c_int) as isize) =
                                        *string.offset(fresh118 as isize);
                                    _i_2 += 1;
                                }
                                *temp.offset(
                                    mblength_3.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        as isize,
                                ) = '\u{0}' as i32 as libc::c_char;
                                current_block = 16465155782176963379;
                            } else {
                                twochars[0 as libc::c_int as usize] =
                                    '\u{1}' as i32 as libc::c_char;
                                twochars[1 as libc::c_int as usize] = c as libc::c_char;
                                current_block = 3893156993890508670;
                            }
                            match current_block {
                                16465155782176963379 => {}
                                1398496724659307859 => {}
                                _ => {
                                    sindex += 1;
                                    current_block = 18404008029668793007;
                                }
                            }
                        }
                    }
                    34 => {
                        if quoted & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0
                            && quoted & 0x100 as libc::c_int == 0 as libc::c_int
                        {
                            current_block = 1398496724659307859;
                        } else {
                            sindex += 1;
                            t_index = sindex;
                            temp = string_extract_double_quoted(
                                string,
                                &mut sindex,
                                if (*word).flags & (1 as libc::c_int) << 27 as libc::c_int != 0 {
                                    0x400 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                },
                            );
                            quoted_state = if t_index == 1 as libc::c_int
                                && *string.offset(sindex as isize) as libc::c_int == '\u{0}' as i32
                            {
                                2 as libc::c_int
                            } else {
                                1 as libc::c_int
                            };
                            if !temp.is_null() && *temp as libc::c_int != 0 {
                                tword = alloc_word_desc();
                                let ref mut fresh121 = (*tword).word;
                                *fresh121 = temp;
                                if (*word).flags & (1 as libc::c_int) << 17 as libc::c_int != 0 {
                                    (*tword).flags |= (*word).flags
                                        & ((1 as libc::c_int) << 17 as libc::c_int
                                            | (1 as libc::c_int) << 11 as libc::c_int);
                                }
                                if (*word).flags & (1 as libc::c_int) << 27 as libc::c_int != 0 {
                                    (*tword).flags |= (1 as libc::c_int) << 27 as libc::c_int;
                                }
                                if (*word).flags & (1 as libc::c_int) << 10 as libc::c_int != 0 {
                                    (*tword).flags |= (1 as libc::c_int) << 10 as libc::c_int;
                                }
                                if (*word).flags & (1 as libc::c_int) << 20 as libc::c_int != 0 {
                                    (*tword).flags |= (1 as libc::c_int) << 20 as libc::c_int;
                                }
                                if (*word).flags & (1 as libc::c_int) << 11 as libc::c_int != 0 {
                                    (*tword).flags |= (1 as libc::c_int) << 11 as libc::c_int;
                                }
                                temp = 0 as *mut libc::c_void as *mut libc::c_char;
                                temp_has_dollar_at = 0 as libc::c_int;
                                list = expand_word_internal(
                                    tword,
                                    0x1 as libc::c_int,
                                    0 as libc::c_int,
                                    &mut temp_has_dollar_at,
                                    0 as *mut libc::c_void as *mut libc::c_int,
                                );
                                has_dollar_at += temp_has_dollar_at;
                                if list == &mut expand_word_error as *mut WORD_LIST
                                    || list == &mut expand_word_fatal as *mut WORD_LIST
                                {
                                    sh_xfree(
                                        istring as *mut libc::c_void,
                                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                                        10513 as libc::c_int,
                                    );
                                    sh_xfree(
                                        string as *mut libc::c_void,
                                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                                        10514 as libc::c_int,
                                    );
                                    let ref mut fresh122 = (*tword).word;
                                    *fresh122 = 0 as *mut libc::c_void as *mut libc::c_char;
                                    dispose_word(tword);
                                    return list;
                                }
                                dispose_word(tword);
                                if list.is_null() && temp_has_dollar_at != 0 {
                                    quoted_dollar_at += 1;
                                    continue;
                                } else {
                                    if !list.is_null()
                                        && !((*list).word).is_null()
                                        && ((*list).next).is_null()
                                        && (*(*list).word).flags
                                            & (1 as libc::c_int) << 18 as libc::c_int
                                            != 0
                                    {
                                        if had_quoted_null != 0 && temp_has_dollar_at != 0 {
                                            quoted_dollar_at += 1;
                                        }
                                        had_quoted_null = 1 as libc::c_int;
                                    }
                                    if !list.is_null() {
                                        dequote_list(list);
                                    }
                                    if temp_has_dollar_at != 0 {
                                        quoted_dollar_at += 1;
                                        if !contains_dollar_at.is_null() {
                                            *contains_dollar_at = 1 as libc::c_int;
                                        }
                                        if !expanded_something.is_null() {
                                            *expanded_something = 1 as libc::c_int;
                                        }
                                        local_expanded = 1 as libc::c_int;
                                    }
                                }
                            } else {
                                if !temp.is_null() {
                                    sh_xfree(
                                        temp as *mut libc::c_void,
                                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                                        10569 as libc::c_int,
                                    );
                                }
                                list = 0 as *mut libc::c_void as *mut WORD_LIST;
                                had_quoted_null = 1 as libc::c_int;
                            }
                            if !list.is_null() {
                                if !((*list).next).is_null() {
                                    temp = if quoted_dollar_at != 0 {
                                        string_list_dollar_at(
                                            list,
                                            0x1 as libc::c_int,
                                            0 as libc::c_int,
                                        )
                                    } else {
                                        string_list(quote_list(list))
                                    };
                                    dispose_words(list);
                                    current_block = 16465155782176963379;
                                } else {
                                    temp = strcpy(
                                        sh_xmalloc(
                                            (1 as libc::c_int as libc::c_ulong)
                                                .wrapping_add(strlen((*(*list).word).word) as u64),
                                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                                            10596 as libc::c_int,
                                        )
                                            as *mut libc::c_char,
                                        (*(*list).word).word,
                                    );
                                    tflag = (*(*list).word).flags;
                                    dispose_words(list);
                                    if tflag & (1 as libc::c_int) << 18 as libc::c_int != 0
                                        && (*temp.offset(0 as libc::c_int as isize) as libc::c_int
                                            == '\u{7f}' as i32
                                            && *temp.offset(1 as libc::c_int as isize)
                                                as libc::c_int
                                                == '\u{0}' as i32)
                                            as libc::c_int
                                            == 0 as libc::c_int
                                    {
                                        remove_quoted_nulls(temp);
                                    }
                                    current_block = 10637788488912693274;
                                }
                            } else {
                                temp = 0 as *mut libc::c_void as *mut libc::c_char;
                                current_block = 10637788488912693274;
                            }
                            match current_block {
                                16465155782176963379 => {}
                                _ => {
                                    if temp.is_null() && quoted_state == 1 as libc::c_int {
                                        had_quoted_null = 1 as libc::c_int;
                                    }
                                    if temp.is_null()
                                        && quoted_state == 1 as libc::c_int
                                        && quoted == 0 as libc::c_int
                                        && (*word).flags
                                            & ((1 as libc::c_int) << 4 as libc::c_int
                                                | (1 as libc::c_int) << 14 as libc::c_int
                                                | (1 as libc::c_int) << 11 as libc::c_int)
                                            == (1 as libc::c_int) << 14 as libc::c_int
                                    {
                                        c = '\u{7f}' as i32 as libc::c_uchar;
                                        sindex -= 1;
                                        had_quoted_null = 1 as libc::c_int;
                                        current_block = 1398496724659307859;
                                    } else {
                                        if temp.is_null()
                                            && quoted_state == 1 as libc::c_int
                                            && (*word).flags
                                                & ((1 as libc::c_int) << 4 as libc::c_int
                                                    | (1 as libc::c_int) << 6 as libc::c_int)
                                                != 0
                                        {
                                            continue;
                                        }
                                        current_block = 10574327403355912886;
                                    }
                                }
                            }
                        }
                    }
                    39 => {
                        if quoted & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0 {
                            current_block = 1398496724659307859;
                        } else {
                            sindex += 1;
                            t_index = sindex;
                            temp = string_extract_single_quoted(string, &mut sindex);
                            quoted_state = if t_index == 1 as libc::c_int
                                && *string.offset(sindex as isize) as libc::c_int == '\u{0}' as i32
                            {
                                2 as libc::c_int
                            } else {
                                1 as libc::c_int
                            };
                            if *temp as libc::c_int == '\u{0}' as i32 {
                                sh_xfree(
                                    temp as *mut libc::c_void,
                                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                                    10674 as libc::c_int,
                                );
                                temp = 0 as *mut libc::c_void as *mut libc::c_char;
                            } else {
                                remove_quoted_escapes(temp);
                            }
                            if temp.is_null() && quoted_state == 1 as libc::c_int {
                                had_quoted_null = 1 as libc::c_int;
                            }
                            if temp.is_null()
                                && quoted_state == 1 as libc::c_int
                                && quoted == 0 as libc::c_int
                                && (*word).flags
                                    & ((1 as libc::c_int) << 4 as libc::c_int
                                        | (1 as libc::c_int) << 14 as libc::c_int
                                        | (1 as libc::c_int) << 11 as libc::c_int)
                                    == (1 as libc::c_int) << 14 as libc::c_int
                            {
                                c = '\u{7f}' as i32 as libc::c_uchar;
                                sindex -= 1;
                                current_block = 1398496724659307859;
                            } else {
                                if temp.is_null()
                                    && quoted_state == 1 as libc::c_int
                                    && (*word).flags
                                        & ((1 as libc::c_int) << 4 as libc::c_int
                                            | (1 as libc::c_int) << 6 as libc::c_int)
                                        != 0
                                {
                                    continue;
                                }
                                if temp.is_null() {
                                    c = '\u{7f}' as i32 as libc::c_uchar;
                                    sindex -= 1;
                                    current_block = 1398496724659307859;
                                } else {
                                    current_block = 10574327403355912886;
                                }
                            }
                        }
                    }
                    32 => {
                        if ifs_is_null != 0
                            || split_on_spaces != 0
                            || (*word).flags
                                & ((1 as libc::c_int) << 4 as libc::c_int
                                    | (1 as libc::c_int) << 6 as libc::c_int
                                    | (1 as libc::c_int) << 11 as libc::c_int)
                                != 0
                                && (*word).flags & (1 as libc::c_int) << 14 as libc::c_int
                                    == 0 as libc::c_int
                        {
                            if *string.offset(sindex as isize) != 0 {
                                sindex += 1;
                            }
                            twochars[0 as libc::c_int as usize] = '\u{1}' as i32 as libc::c_char;
                            twochars[1 as libc::c_int as usize] = c as libc::c_char;
                            current_block = 18404008029668793007;
                        } else {
                            current_block = 8014490948552223808;
                        }
                    }
                    _ => {
                        current_block = 8014490948552223808;
                    }
                }
                match current_block {
                    8014490948552223808 => {
                        if quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0
                            || isexp == 0 as libc::c_int
                                && ifs_cmap[c as usize] as libc::c_int != 0 as libc::c_int
                                && (*word).flags
                                    & ((1 as libc::c_int) << 4 as libc::c_int
                                        | (1 as libc::c_int) << 6 as libc::c_int)
                                    == 0 as libc::c_int
                        {
                            if quoted & (0x2 as libc::c_int | 0x1 as libc::c_int)
                                == 0 as libc::c_int
                            {
                                has_quoted_ifs += 1;
                            }
                            if *string.offset(sindex as isize) != 0 {
                                sindex += 1;
                            }
                            if c as libc::c_int == 0 as libc::c_int {
                                c = '\u{7f}' as i32 as libc::c_uchar;
                                current_block = 1398496724659307859;
                            } else {
                                if mb_cur_max > 1 as libc::c_int {
                                    sindex -= 1;
                                }
                                if mb_cur_max > 1 as libc::c_int {
                                    let mut i_0: libc::c_int = 0;
                                    let mut state_bak_4: mbstate_t = mbstate_t {
                                        __count: 0,
                                        __value: mbstate_t_value { __wch: 0 },
                                    };
                                    let mut mblength_4: size_t = 0;
                                    i_0 = is_basic(*string.offset(sindex as isize));
                                    if i_0 != 0 {
                                        mblength_4 = 1 as libc::c_int as size_t;
                                    } else if locale_utf8locale != 0
                                        && *string.offset(sindex as isize) as libc::c_int
                                            & 0x80 as libc::c_int
                                            == 0 as libc::c_int
                                    {
                                        mblength_4 = (*string.offset(sindex as isize)
                                            as libc::c_int
                                            != 0 as libc::c_int)
                                            as libc::c_int
                                            as size_t;
                                    } else {
                                        state_bak_4 = state;
                                        mblength_4 = mbrlen(
                                            string.offset(sindex as isize),
                                            string_size.wrapping_sub(sindex as libc::c_ulong),
                                            &mut state,
                                        );
                                    }
                                    if mblength_4 == -(1 as libc::c_int) as size_t
                                        || mblength_4 == -(2 as libc::c_int) as size_t
                                    {
                                        state = state_bak_4;
                                        mblength_4 = 1 as libc::c_int as size_t;
                                    }
                                    if mblength_4 < 1 as libc::c_int as libc::c_ulong {
                                        mblength_4 = 1 as libc::c_int as size_t;
                                    }
                                    temp = sh_xmalloc(
                                        mblength_4.wrapping_add(2 as libc::c_int as libc::c_ulong),
                                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                                        10750 as libc::c_int,
                                    )
                                        as *mut libc::c_char;
                                    *temp.offset(0 as libc::c_int as isize) =
                                        '\u{1}' as i32 as libc::c_char;
                                    i_0 = 0 as libc::c_int;
                                    while (i_0 as libc::c_ulong) < mblength_4 {
                                        let fresh123 = sindex;
                                        sindex = sindex + 1;
                                        *temp.offset((i_0 + 1 as libc::c_int) as isize) =
                                            *string.offset(fresh123 as isize);
                                        i_0 += 1;
                                    }
                                    *temp.offset(
                                        mblength_4.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = '\u{0}' as i32 as libc::c_char;
                                    current_block = 16465155782176963379;
                                } else {
                                    twochars[0 as libc::c_int as usize] =
                                        '\u{1}' as i32 as libc::c_char;
                                    twochars[1 as libc::c_int as usize] = c as libc::c_char;
                                    current_block = 18404008029668793007;
                                }
                            }
                        } else if locale_mb_cur_max > 1 as libc::c_int {
                            let mut i_1: libc::c_int = 0;
                            let mut state_bak_5: mbstate_t = mbstate_t {
                                __count: 0,
                                __value: mbstate_t_value { __wch: 0 },
                            };
                            let mut mblength_5: size_t = 0;
                            i_1 = is_basic(*string.offset(sindex as isize));
                            if i_1 != 0 {
                                mblength_5 = 1 as libc::c_int as size_t;
                            } else if locale_utf8locale != 0
                                && *string.offset(sindex as isize) as libc::c_int
                                    & 0x80 as libc::c_int
                                    == 0 as libc::c_int
                            {
                                mblength_5 = (*string.offset(sindex as isize) as libc::c_int
                                    != 0 as libc::c_int)
                                    as libc::c_int
                                    as size_t;
                            } else {
                                state_bak_5 = state;
                                mblength_5 = mbrlen(
                                    string.offset(sindex as isize),
                                    string_size.wrapping_sub(sindex as libc::c_ulong),
                                    &mut state,
                                );
                            }
                            if mblength_5 == -(1 as libc::c_int) as size_t
                                || mblength_5 == -(2 as libc::c_int) as size_t
                            {
                                state = state_bak_5;
                                mblength_5 = 1 as libc::c_int as size_t;
                            }
                            if mblength_5 < 1 as libc::c_int as libc::c_ulong {
                                mblength_5 = 1 as libc::c_int as size_t;
                            }
                            temp = sh_xmalloc(
                                mblength_5.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                10762 as libc::c_int,
                            ) as *mut libc::c_char;
                            i_1 = 0 as libc::c_int;
                            while (i_1 as libc::c_ulong) < mblength_5 {
                                let fresh124 = sindex;
                                sindex = sindex + 1;
                                *temp.offset(i_1 as isize) = *string.offset(fresh124 as isize);
                                i_1 += 1;
                            }
                            *temp.offset(mblength_5 as isize) = '\u{0}' as i32 as libc::c_char;
                            current_block = 16465155782176963379;
                        } else {
                            current_block = 1398496724659307859;
                        }
                    }
                    10574327403355912886 => {
                        if !temp.is_null() {
                            temp1 = temp;
                            temp = quote_string(temp);
                            sh_xfree(
                                temp1 as *mut libc::c_void,
                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                10644 as libc::c_int,
                            );
                            current_block = 16465155782176963379;
                        } else {
                            c = '\u{7f}' as i32 as libc::c_uchar;
                            sindex -= 1;
                            had_quoted_null = 1 as libc::c_int;
                            current_block = 1398496724659307859;
                        }
                    }
                    8423922909464824957 => {
                        if *string.offset(sindex as isize) != 0 {
                            sindex += 1;
                        }
                        current_block = 16465155782176963379;
                    }
                    _ => {}
                }
                match current_block {
                    16465155782176963379 => {
                        if !temp.is_null() {
                            istring = sub_append_string(
                                temp,
                                istring,
                                &mut istring_index,
                                &mut istring_size,
                            );
                            temp = 0 as *mut libc::c_char;
                        }
                    }
                    1398496724659307859 => {
                        if (istring_index + 1 as libc::c_int) as libc::c_ulong >= istring_size {
                            while (istring_index + 1 as libc::c_int) as libc::c_ulong
                                >= istring_size
                            {
                                istring_size = (istring_size as libc::c_ulong)
                                    .wrapping_add(128 as libc::c_int as libc::c_ulong)
                                    as size_t
                                    as size_t;
                            }
                            istring = sh_xrealloc(
                                istring as *mut libc::c_void,
                                istring_size,
                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                10766 as libc::c_int,
                            ) as *mut libc::c_char;
                        }
                        let fresh125 = istring_index;
                        istring_index = istring_index + 1;
                        *istring.offset(fresh125 as isize) = c as libc::c_char;
                        *istring.offset(istring_index as isize) = '\u{0}' as i32 as libc::c_char;
                        sindex += 1;
                    }
                    _ => {
                        if (istring_index + 2 as libc::c_int) as libc::c_ulong >= istring_size {
                            while (istring_index + 2 as libc::c_int) as libc::c_ulong
                                >= istring_size
                            {
                                istring_size = (istring_size as libc::c_ulong)
                                    .wrapping_add(128 as libc::c_int as libc::c_ulong)
                                    as size_t
                                    as size_t;
                            }
                            istring = sh_xrealloc(
                                istring as *mut libc::c_void,
                                istring_size,
                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                10467 as libc::c_int,
                            ) as *mut libc::c_char;
                        }
                        let fresh119 = istring_index;
                        istring_index = istring_index + 1;
                        *istring.offset(fresh119 as isize) = twochars[0 as libc::c_int as usize];
                        let fresh120 = istring_index;
                        istring_index = istring_index + 1;
                        *istring.offset(fresh120 as isize) = twochars[1 as libc::c_int as usize];
                        *istring.offset(istring_index as isize) = '\u{0}' as i32 as libc::c_char;
                    }
                }
            }
        }
        if *istring as libc::c_int == '\u{0}' as i32 {
            if quoted_dollar_at == 0 as libc::c_int
                && (had_quoted_null != 0 || quoted_state == 1 as libc::c_int)
            {
                *istring.offset(0 as libc::c_int as isize) = '\u{7f}' as i32 as libc::c_char;
                *istring.offset(1 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
                tword = alloc_word_desc();
                let ref mut fresh126 = (*tword).word;
                *fresh126 = istring;
                istring = 0 as *mut libc::c_char;
                (*tword).flags |= (1 as libc::c_int) << 18 as libc::c_int;
                list = make_word_list(tword, 0 as *mut libc::c_void as *mut WORD_LIST);
                if quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0 {
                    (*tword).flags |= (1 as libc::c_int) << 1 as libc::c_int;
                }
            } else if quoted_state == 0 as libc::c_int || quoted_dollar_at != 0 {
                list = 0 as *mut libc::c_void as *mut WORD_LIST;
            } else {
                list = 0 as *mut libc::c_void as *mut WORD_LIST;
            }
        } else if (*word).flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
            tword = alloc_word_desc();
            let ref mut fresh127 = (*tword).word;
            *fresh127 = istring;
            if had_quoted_null != 0
                && (*istring.offset(0 as libc::c_int as isize) as libc::c_int == '\u{7f}' as i32
                    && *istring.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32)
            {
                (*tword).flags |= (1 as libc::c_int) << 18 as libc::c_int;
            }
            istring = 0 as *mut libc::c_char;
            if (*word).flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                (*tword).flags |= (1 as libc::c_int) << 2 as libc::c_int;
            }
            if (*word).flags & (1 as libc::c_int) << 15 as libc::c_int != 0 {
                (*tword).flags |= (1 as libc::c_int) << 15 as libc::c_int;
            }
            if (*word).flags & (1 as libc::c_int) << 5 as libc::c_int != 0 {
                (*tword).flags |= (1 as libc::c_int) << 5 as libc::c_int;
            }
            if (*word).flags & (1 as libc::c_int) << 26 as libc::c_int != 0 {
                (*tword).flags |= (1 as libc::c_int) << 26 as libc::c_int;
            }
            if quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0 {
                (*tword).flags |= (1 as libc::c_int) << 1 as libc::c_int;
            }
            list = make_word_list(tword, 0 as *mut libc::c_void as *mut WORD_LIST);
        } else {
            if (*word).flags & (1 as libc::c_int) << 11 as libc::c_int != 0 {
                list = list_string(
                    istring,
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    quoted,
                );
                tword = (*list).word;
                if had_quoted_null != 0
                    && (*istring.offset(0 as libc::c_int as isize) as libc::c_int
                        == '\u{7f}' as i32
                        && *istring.offset(1 as libc::c_int as isize) as libc::c_int
                            == '\u{0}' as i32)
                {
                    (*tword).flags |= (1 as libc::c_int) << 18 as libc::c_int;
                }
                sh_xfree(
                    list as *mut libc::c_void,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    10851 as libc::c_int,
                );
                sh_xfree(
                    istring as *mut libc::c_void,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    10852 as libc::c_int,
                );
                istring = 0 as *mut libc::c_char;
                current_block = 18054563751907035352;
            } else {
                ifs_chars = 0 as *mut libc::c_char;
                ifs_chars = if quoted_dollar_at != 0 || has_dollar_at != 0 {
                    ifs_value
                } else {
                    0 as *mut libc::c_void as *mut libc::c_char
                };
                if split_on_spaces != 0 {
                    if ifs_is_set == 0 as libc::c_int {
                        list = list_string(
                            istring,
                            b" \t\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            1 as libc::c_int,
                        );
                    } else {
                        list = list_string(
                            istring,
                            b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            1 as libc::c_int,
                        );
                    }
                    current_block = 16332075848674751501;
                } else if has_dollar_at != 0
                    && quoted_dollar_at == 0 as libc::c_int
                    && !ifs_chars.is_null()
                    && quoted == 0 as libc::c_int
                    && (*word).flags & (1 as libc::c_int) << 6 as libc::c_int != 0
                {
                    tword = alloc_word_desc();
                    if *ifs_chars as libc::c_int != 0 && *ifs_chars as libc::c_int != ' ' as i32 {
                        list = list_string(
                            istring,
                            (if *ifs_chars as libc::c_int != 0 {
                                ifs_chars
                            } else {
                                b" \0" as *const u8 as *const libc::c_char
                            }) as *mut libc::c_char,
                            1 as libc::c_int,
                        );
                        let ref mut fresh128 = (*tword).word;
                        *fresh128 = string_list(list);
                    } else {
                        let ref mut fresh129 = (*tword).word;
                        *fresh129 = istring;
                    }
                    if had_quoted_null != 0
                        && (*istring.offset(0 as libc::c_int as isize) as libc::c_int
                            == '\u{7f}' as i32
                            && *istring.offset(1 as libc::c_int as isize) as libc::c_int
                                == '\u{0}' as i32)
                    {
                        (*tword).flags |= (1 as libc::c_int) << 18 as libc::c_int;
                    }
                    if (*tword).word != istring {
                        sh_xfree(
                            istring as *mut libc::c_void,
                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                            10918 as libc::c_int,
                        );
                    }
                    istring = 0 as *mut libc::c_char;
                    current_block = 18054563751907035352;
                } else if has_dollar_at != 0 && !ifs_chars.is_null() {
                    list = list_string(
                        istring,
                        (if *ifs_chars as libc::c_int != 0 {
                            ifs_chars
                        } else {
                            b" \0" as *const u8 as *const libc::c_char
                        }) as *mut libc::c_char,
                        1 as libc::c_int,
                    );
                    current_block = 16332075848674751501;
                } else {
                    tword = alloc_word_desc();
                    if !expanded_something.is_null()
                        && *expanded_something == 0 as libc::c_int
                        && has_quoted_ifs != 0
                    {
                        let ref mut fresh130 = (*tword).word;
                        *fresh130 = remove_quoted_ifs(istring);
                    } else {
                        let ref mut fresh131 = (*tword).word;
                        *fresh131 = istring;
                    }
                    if had_quoted_null != 0
                        && (*istring.offset(0 as libc::c_int as isize) as libc::c_int
                            == '\u{7f}' as i32
                            && *istring.offset(1 as libc::c_int as isize) as libc::c_int
                                == '\u{0}' as i32)
                    {
                        (*tword).flags |= (1 as libc::c_int) << 18 as libc::c_int;
                    } else if had_quoted_null != 0 {
                        (*tword).flags |= (1 as libc::c_int) << 21 as libc::c_int;
                    }
                    if (*tword).word != istring {
                        sh_xfree(
                            istring as *mut libc::c_void,
                            b"../subst.c\0" as *const u8 as *const libc::c_char,
                            10936 as libc::c_int,
                        );
                    }
                    istring = 0 as *mut libc::c_char;
                    current_block = 18054563751907035352;
                }
            }
            match current_block {
                16332075848674751501 => {}
                _ => {
                    if quoted & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0
                        || quoted_state == 2 as libc::c_int
                    {
                        (*tword).flags |= (1 as libc::c_int) << 1 as libc::c_int;
                    }
                    if (*word).flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                        (*tword).flags |= (1 as libc::c_int) << 2 as libc::c_int;
                    }
                    if (*word).flags & (1 as libc::c_int) << 15 as libc::c_int != 0 {
                        (*tword).flags |= (1 as libc::c_int) << 15 as libc::c_int;
                    }
                    if (*word).flags & (1 as libc::c_int) << 5 as libc::c_int != 0 {
                        (*tword).flags |= (1 as libc::c_int) << 5 as libc::c_int;
                    }
                    if (*word).flags & (1 as libc::c_int) << 26 as libc::c_int != 0 {
                        (*tword).flags |= (1 as libc::c_int) << 26 as libc::c_int;
                    }
                    list = make_word_list(tword, 0 as *mut libc::c_void as *mut WORD_LIST);
                }
            }
        }
        sh_xfree(
            istring as *mut libc::c_void,
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            10953 as libc::c_int,
        );
        return list;
    }
}
#[no_mangle]
pub fn string_quote_removal(
    mut string: *mut libc::c_char,
    mut quoted: libc::c_int,
) -> *mut libc::c_char {
    let mut slen: size_t = 0;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut send: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sindex: libc::c_int = 0;
    let mut tindex: libc::c_int = 0;
    let mut dquote: libc::c_int = 0;
    let mut c: libc::c_uchar = 0;
    let mut state: mbstate_t = mbstate_t {
        __count: 0,
        __value: mbstate_t_value { __wch: 0 },
    };
    unsafe {
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<mbstate_t>() as usize,
        );
        slen = strlen(string) as u64;
        send = string.offset(slen as isize);
        result_string = sh_xmalloc(
            slen.wrapping_add(1 as libc::c_int as libc::c_ulong),
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            10980 as libc::c_int,
        ) as *mut libc::c_char;
        r = result_string;
        sindex = 0 as libc::c_int;
        dquote = sindex;
        loop {
            c = *string.offset(sindex as isize) as libc::c_uchar;
            if !(c != 0) {
                break;
            }
            let mut current_block_41: u64;
            match c as libc::c_int {
                92 => {
                    sindex += 1;
                    c = *string.offset(sindex as isize) as libc::c_uchar;
                    if c as libc::c_int == 0 as libc::c_int {
                        let fresh132 = r;
                        r = r.offset(1);
                        *fresh132 = '\\' as i32 as libc::c_char;
                        current_block_41 = 13619784596304402172;
                    } else {
                        if (quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0 || dquote != 0)
                            && *sh_syntaxtab.as_mut_ptr().offset(c as isize) & 0x40 as libc::c_int
                                == 0 as libc::c_int
                        {
                            let fresh133 = r;
                            r = r.offset(1);
                            *fresh133 = '\\' as i32 as libc::c_char;
                        }
                        current_block_41 = 10710088620172452289;
                    }
                }
                39 => {
                    if quoted & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0 || dquote != 0 {
                        let fresh135 = r;
                        r = r.offset(1);
                        *fresh135 = c as libc::c_char;
                        sindex += 1;
                    } else {
                        tindex = sindex + 1 as libc::c_int;
                        temp = string_extract_single_quoted(string, &mut tindex);
                        if !temp.is_null() {
                            strcpy(r, temp);
                            r = r.offset(strlen(r) as isize);
                            sh_xfree(
                                temp as *mut libc::c_void,
                                b"../subst.c\0" as *const u8 as *const libc::c_char,
                                11014 as libc::c_int,
                            );
                        }
                        sindex = tindex;
                    }
                    current_block_41 = 13619784596304402172;
                }
                34 => {
                    dquote = 1 as libc::c_int - dquote;
                    sindex += 1;
                    current_block_41 = 13619784596304402172;
                }
                _ => {
                    current_block_41 = 10710088620172452289;
                }
            }
            match current_block_41 {
                10710088620172452289 => {
                    if locale_mb_cur_max > 1 as libc::c_int {
                        let mut state_bak: mbstate_t = mbstate_t {
                            __count: 0,
                            __value: mbstate_t_value { __wch: 0 },
                        };
                        let mut mblength: size_t = 0;
                        let mut _i: libc::c_int = 0;
                        _i = is_basic(*string.offset(sindex as isize));
                        if _i != 0 {
                            mblength = 1 as libc::c_int as size_t;
                        } else {
                            state_bak = state;
                            mblength = mbrlen(
                                string.offset(sindex as isize),
                                send.offset_from(string.offset(sindex as isize)) as libc::c_long
                                    as size_t,
                                &mut state,
                            );
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
                        libc::memcpy(
                            r as *mut libc::c_void,
                            string.offset(sindex as isize) as *const libc::c_void,
                            mblength as libc::size_t,
                        );
                        r = r.offset(mblength as isize);
                        sindex = (sindex as libc::c_ulong).wrapping_add(mblength) as libc::c_int
                            as libc::c_int;
                    } else {
                        let fresh134 = r;
                        r = r.offset(1);
                        *fresh134 = *string.offset(sindex as isize);
                        sindex += 1;
                    }
                }
                _ => {}
            }
        }
        *r = '\u{0}' as i32 as libc::c_char;
    }
    return result_string;
}
#[no_mangle]
pub fn setifs(mut v: *mut SHELL_VAR) {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut uc: libc::c_uchar = 0;
    unsafe {
        ifs_var = v;
        ifs_value = (if !v.is_null() && !((*v).value).is_null() {
            (*v).value
        } else {
            b" \t\n\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char;
        ifs_is_set = (ifs_var != 0 as *mut SHELL_VAR) as libc::c_int;
        ifs_is_null =
            (ifs_is_set != 0 && *ifs_value as libc::c_int == 0 as libc::c_int) as libc::c_int;
        memset(
            ifs_cmap.as_mut_ptr() as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<[libc::c_uchar; 256]>() as usize,
        );
        t = ifs_value;
        while !t.is_null() && *t as libc::c_int != 0 {
            uc = *t as libc::c_uchar;
            ifs_cmap[uc as usize] = 1 as libc::c_int as libc::c_uchar;
            t = t.offset(1);
        }
        if ifs_value.is_null() {
            ifs_firstc[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_uchar;
            ifs_firstc_len = 1 as libc::c_int as size_t;
        } else {
            if locale_utf8locale != 0
                && *ifs_value as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
            {
                ifs_firstc_len = (if *ifs_value as libc::c_int != 0 as libc::c_int {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as size_t;
            } else {
                let mut ifs_len: size_t = 0;
                ifs_len = strnlen(ifs_value, __ctype_get_mb_cur_max()) as u64;
                ifs_firstc_len = (if __ctype_get_mb_cur_max() > 1 as libc::c_int as usize {
                    mblen(ifs_value, ifs_len)
                } else {
                    1 as libc::c_int
                }) as size_t;
            }
            if ifs_firstc_len == 1 as libc::c_int as libc::c_ulong
                || ifs_firstc_len == 0 as libc::c_int as libc::c_ulong
                || (ifs_firstc_len == -(1 as libc::c_int) as size_t
                    || ifs_firstc_len == -(2 as libc::c_int) as size_t)
            {
                ifs_firstc[0 as libc::c_int as usize] =
                    *ifs_value.offset(0 as libc::c_int as isize) as libc::c_uchar;
                ifs_firstc[1 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_uchar;
                ifs_firstc_len = 1 as libc::c_int as size_t;
            } else {
                memcpy(
                    ifs_firstc.as_mut_ptr() as *mut libc::c_void,
                    ifs_value as *const libc::c_void,
                    ifs_firstc_len as usize,
                );
            }
        };
    }
}
#[no_mangle]
pub fn getifs() -> *mut libc::c_char {
    unsafe {
        return ifs_value;
    }
}
#[no_mangle]
pub fn word_split(mut w: *mut WORD_DESC, mut ifs_chars: *mut libc::c_char) -> *mut WORD_LIST {
    let mut result: *mut WORD_LIST = 0 as *mut WORD_LIST;
    unsafe {
        if !w.is_null() {
            let mut xifs: *mut libc::c_char = 0 as *mut libc::c_char;
            xifs = (if (*w).flags & (1 as libc::c_int) << 1 as libc::c_int != 0
                || ifs_chars.is_null()
            {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                ifs_chars
            }) as *mut libc::c_char;
            result = list_string(
                (*w).word,
                xifs,
                (*w).flags & (1 as libc::c_int) << 1 as libc::c_int,
            );
        } else {
            result = 0 as *mut libc::c_void as *mut WORD_LIST;
        }
    }
    return result;
}
fn word_list_split(mut list: *mut WORD_LIST) -> *mut WORD_LIST {
    let mut result: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut t: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut tresult: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut e: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut w: *mut WORD_DESC = 0 as *mut WORD_DESC;
    t = list;
    result = 0 as *mut libc::c_void as *mut WORD_LIST;
    unsafe {
        while !t.is_null() {
            tresult = word_split((*t).word, ifs_value);
            if tresult.is_null()
                && !((*t).word).is_null()
                && (*(*t).word).flags & (1 as libc::c_int) << 21 as libc::c_int != 0
            {
                w = alloc_word_desc();
                let ref mut fresh136 = (*w).word;
                *fresh136 = sh_xmalloc(
                    1 as libc::c_int as size_t,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    11185 as libc::c_int,
                ) as *mut libc::c_char;
                *((*w).word).offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
                tresult = make_word_list(w, 0 as *mut libc::c_void as *mut WORD_LIST);
            }
            if result.is_null() {
                e = tresult;
                result = e;
            } else {
                let ref mut fresh137 = (*e).next;
                *fresh137 = tresult;
                while !((*e).next).is_null() {
                    e = (*e).next;
                }
            }
            t = (*t).next;
        }
    }
    return result;
}
fn exp_jump_to_top_level(mut v: libc::c_int) {
    unsafe {
        set_pipestatus_from_exit(last_command_exit_value);
        expand_no_split_dollar_star = 0 as libc::c_int;
        if expanding_redir != 0 {
            undo_partial_redirects();
        }
        expanding_redir = 0 as libc::c_int;
        assigning_in_environment = 0 as libc::c_int;
        if parse_and_execute_level == 0 as libc::c_int {
            top_level_cleanup();
        }
        jump_to_top_level(v);
    }
}
fn separate_out_assignments(mut tlist: *mut WORD_LIST) -> *mut WORD_LIST {
    let mut vp: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut lp: *mut WORD_LIST = 0 as *mut WORD_LIST;
    if tlist.is_null() {
        return 0 as *mut libc::c_void as *mut WORD_LIST;
    }
    unsafe {
        if !subst_assign_varlist.is_null() {
            dispose_words(subst_assign_varlist);
        }
        subst_assign_varlist = 0 as *mut libc::c_void as *mut WORD_LIST;
        lp = tlist;
        vp = lp;
        while !lp.is_null() && (*(*lp).word).flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            vp = lp;
            lp = (*lp).next;
        }
        if lp != tlist {
            subst_assign_varlist = tlist;
            let ref mut fresh138 = (*vp).next;
            *fresh138 = 0 as *mut libc::c_void as *mut WORD_LIST;
            tlist = lp;
        }
        if tlist.is_null() {
            return 0 as *mut libc::c_void as *mut WORD_LIST;
        }
        if place_keywords_in_env != 0 {
            let mut tp: *mut WORD_LIST = 0 as *mut WORD_LIST;
            tp = tlist;
            lp = (*tlist).next;
            while !lp.is_null() {
                if (*(*lp).word).flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                    if subst_assign_varlist.is_null() {
                        vp = lp;
                        subst_assign_varlist = vp;
                    } else {
                        let ref mut fresh139 = (*vp).next;
                        *fresh139 = lp;
                        vp = lp;
                    }
                    let ref mut fresh140 = (*tp).next;
                    *fresh140 = (*lp).next;
                    let ref mut fresh141 = (*lp).next;
                    *fresh141 = 0 as *mut libc::c_void as *mut WORD_LIST;
                    lp = (*tp).next;
                } else {
                    tp = lp;
                    lp = (*lp).next;
                }
            }
        }
    }
    return tlist;
}
#[no_mangle]
pub fn expand_words(mut list: *mut WORD_LIST) -> *mut WORD_LIST {
    return expand_word_list_internal(
        list,
        0x1 as libc::c_int
            | 0x2 as libc::c_int
            | 0x4 as libc::c_int
            | 0x8 as libc::c_int
            | 0x10 as libc::c_int,
    );
}
#[no_mangle]
pub fn expand_words_no_vars(mut list: *mut WORD_LIST) -> *mut WORD_LIST {
    return expand_word_list_internal(
        list,
        0x2 as libc::c_int | 0x4 as libc::c_int | 0x8 as libc::c_int | 0x10 as libc::c_int,
    );
}
#[no_mangle]
pub fn expand_words_shellexp(mut list: *mut WORD_LIST) -> *mut WORD_LIST {
    return expand_word_list_internal(
        list,
        0x2 as libc::c_int | 0x4 as libc::c_int | 0x8 as libc::c_int,
    );
}
fn glob_expand_word_list(mut tlist: *mut WORD_LIST, mut eflags: libc::c_int) -> *mut WORD_LIST {
    let mut glob_array: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut temp_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut glob_index: libc::c_int = 0;
    let mut glob_list: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut output_list: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut disposables: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut next: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut tword: *mut WORD_DESC = 0 as *mut WORD_DESC;
    let mut x: libc::c_int = 0;
    disposables = 0 as *mut libc::c_void as *mut WORD_LIST;
    output_list = disposables;
    glob_array = 0 as *mut libc::c_void as *mut *mut libc::c_char;
    unsafe {
        while !tlist.is_null() {
            next = (*tlist).next;
            if (*(*tlist).word).flags & (1 as libc::c_int) << 5 as libc::c_int == 0 as libc::c_int
                && unquoted_glob_pattern_p((*(*tlist).word).word) != 0
            {
                glob_array = shell_glob_filename((*(*tlist).word).word, 0x8 as libc::c_int);
                if glob_array.is_null()
                    || glob_array == &mut glob_error_return as *mut *mut libc::c_char
                {
                    glob_array = sh_xmalloc(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        11413 as libc::c_int,
                    ) as *mut *mut libc::c_char;
                    let ref mut fresh142 = *glob_array.offset(0 as libc::c_int as isize);
                    *fresh142 = 0 as *mut libc::c_void as *mut libc::c_char;
                }
                if (*glob_array.offset(0 as libc::c_int as isize)).is_null() {
                    temp_string = dequote_string((*(*tlist).word).word);
                    sh_xfree(
                        (*(*tlist).word).word as *mut libc::c_void,
                        b"../subst.c\0" as *const u8 as *const libc::c_char,
                        11421 as libc::c_int,
                    );
                    let ref mut fresh143 = (*(*tlist).word).word;
                    *fresh143 = temp_string;
                }
                glob_list = 0 as *mut libc::c_void as *mut WORD_LIST;
                glob_index = 0 as libc::c_int;
                while !(*glob_array.offset(glob_index as isize)).is_null() {
                    tword = make_bare_word(*glob_array.offset(glob_index as isize));
                    glob_list = make_word_list(tword, glob_list);
                    glob_index += 1;
                }
                if !glob_list.is_null() {
                    output_list = list_append(glob_list, output_list) as *mut WORD_LIST;
                    let ref mut fresh144 = (*tlist).next;
                    *fresh144 = disposables;
                    disposables = tlist;
                } else if fail_glob_expansion != 0 as libc::c_int {
                    ::std::ptr::write_volatile(
                        &mut last_command_exit_value as *mut libc::c_int,
                        1 as libc::c_int,
                    );
                    report_error(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"no match: %s\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*(*tlist).word).word,
                    );
                    exp_jump_to_top_level(2 as libc::c_int);
                } else if allow_null_glob_expansion == 0 as libc::c_int {
                    let ref mut fresh145 = (*tlist).next;
                    *fresh145 = output_list;
                    output_list = tlist;
                } else {
                    let ref mut fresh146 = (*tlist).next;
                    *fresh146 = disposables;
                    disposables = tlist;
                }
            } else {
                temp_string = dequote_string((*(*tlist).word).word);
                sh_xfree(
                    (*(*tlist).word).word as *mut libc::c_void,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    11459 as libc::c_int,
                );
                let ref mut fresh147 = (*(*tlist).word).word;
                *fresh147 = temp_string;
                let ref mut fresh148 = (*tlist).next;
                *fresh148 = output_list;
                output_list = tlist;
            }
            strvec_dispose(glob_array);
            glob_array = 0 as *mut libc::c_void as *mut *mut libc::c_char;
            tlist = next;
        }
        if !disposables.is_null() {
            dispose_words(disposables);
        }
        if !output_list.is_null() {
            output_list = if !output_list.is_null() && !((*output_list).next).is_null() {
                list_reverse(output_list as *mut GENERIC_LIST) as *mut WORD_LIST
            } else {
                output_list
            };
        }
    }
    return output_list;
}
fn brace_expand_word_list(mut tlist: *mut WORD_LIST, mut eflags: libc::c_int) -> *mut WORD_LIST {
    let mut expansions: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut temp_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut disposables: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut output_list: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut next: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut w: *mut WORD_DESC = 0 as *mut WORD_DESC;
    let mut eindex: libc::c_int = 0;
    output_list = 0 as *mut libc::c_void as *mut WORD_LIST;
    disposables = output_list;
    unsafe {
        while !tlist.is_null() {
            next = (*tlist).next;
            if (*(*tlist).word).flags & (1 as libc::c_int) << 26 as libc::c_int != 0 {
                let ref mut fresh149 = (*tlist).next;
                *fresh149 = output_list;
                output_list = tlist;
            } else if (*(*tlist).word).flags
                & ((1 as libc::c_int) << 15 as libc::c_int
                    | (1 as libc::c_int) << 17 as libc::c_int)
                == (1 as libc::c_int) << 15 as libc::c_int | (1 as libc::c_int) << 17 as libc::c_int
            {
                let ref mut fresh150 = (*tlist).next;
                *fresh150 = output_list;
                output_list = tlist;
            } else if !(mbschr((*(*tlist).word).word, '{' as i32)).is_null() {
                expansions = brace_expand((*(*tlist).word).word);
                eindex = 0 as libc::c_int;
                loop {
                    temp_string = *expansions.offset(eindex as isize);
                    if temp_string.is_null() {
                        break;
                    }
                    w = alloc_word_desc();
                    let ref mut fresh151 = (*w).word;
                    *fresh151 = temp_string;
                    if *temp_string.offset(0 as libc::c_int as isize) as libc::c_int
                        == *((*(*tlist).word).word).offset(0 as libc::c_int as isize) as libc::c_int
                        && strcmp(temp_string, (*(*tlist).word).word) == 0 as libc::c_int
                    {
                        (*w).flags = (*(*tlist).word).flags;
                    } else {
                        w = make_word_flags(w, temp_string);
                    }
                    output_list = make_word_list(w, output_list);
                    eindex += 1;
                }
                sh_xfree(
                    expansions as *mut libc::c_void,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    11535 as libc::c_int,
                );
                let ref mut fresh152 = (*tlist).next;
                *fresh152 = disposables;
                disposables = tlist;
            } else {
                let ref mut fresh153 = (*tlist).next;
                *fresh153 = output_list;
                output_list = tlist;
            }
            tlist = next;
        }
        if !disposables.is_null() {
            dispose_words(disposables);
        }
        if !output_list.is_null() {
            output_list = if !output_list.is_null() && !((*output_list).next).is_null() {
                list_reverse(output_list as *mut GENERIC_LIST) as *mut WORD_LIST
            } else {
                output_list
            };
        }
    }
    return output_list;
}
fn make_internal_declare(
    mut word: *mut libc::c_char,
    mut option: *mut libc::c_char,
    mut cmd: *mut libc::c_char,
) -> libc::c_int {
    let mut t: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut wl: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut w: *mut WORD_DESC = 0 as *mut WORD_DESC;
    w = make_word(word);
    unsafe {
        t = assignment((*w).word, 0 as libc::c_int);
        if *((*w).word).offset(t as isize) as libc::c_int == '=' as i32 {
            *((*w).word).offset(t as isize) = '\u{0}' as i32 as libc::c_char;
            if *((*w).word).offset((t - 1 as libc::c_int) as isize) as libc::c_int == '+' as i32 {
                *((*w).word).offset((t - 1 as libc::c_int) as isize) =
                    '\u{0}' as i32 as libc::c_char;
            }
        }

        wl = make_word_list(w, 0 as *mut libc::c_void as *mut WORD_LIST);
        wl = make_word_list(make_word(option), wl);
        r = declare_builtin(wl);
        dispose_words(wl);
    }
    return r;
}
fn expand_oneword(mut value: *mut libc::c_char, mut flags: libc::c_int) -> *mut WORD_LIST {
    let mut l: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut nl: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut kvpair: libc::c_int = 0;
    unsafe {
        if flags == 0 as libc::c_int {
            l = expand_compound_array_assignment(
                0 as *mut libc::c_void as *mut SHELL_VAR,
                value,
                flags,
            );
            quote_compound_array_list(l, flags);
            return l;
        } else {
            l = parse_string_to_word_list(
                value,
                1 as libc::c_int,
                b"array assign\0" as *const u8 as *const libc::c_char,
            );
            kvpair = kvpair_assignment_p(l);
            nl = l;
            while !nl.is_null() {
                if kvpair != 0 {
                    t = expand_and_quote_kvpair_word((*(*nl).word).word);
                } else if (*(*nl).word).flags & (1 as libc::c_int) << 2 as libc::c_int
                    == 0 as libc::c_int
                {
                    t = sh_single_quote(if !((*(*nl).word).word).is_null() {
                        (*(*nl).word).word
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    });
                } else {
                    t = expand_and_quote_assoc_word((*(*nl).word).word, flags);
                }
                sh_xfree(
                    (*(*nl).word).word as *mut libc::c_void,
                    b"../subst.c\0" as *const u8 as *const libc::c_char,
                    11646 as libc::c_int,
                );
                let ref mut fresh154 = (*(*nl).word).word;
                *fresh154 = t;
                nl = (*nl).next;
            }
            return l;
        };
    }
}
fn expand_compound_assignment_word(mut tlist: *mut WORD_LIST, mut flags: libc::c_int) {
    let mut l: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut wlen: libc::c_int = 0;
    let mut oind: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
        t = assignment((*(*tlist).word).word, 0 as libc::c_int);
        oind = 1 as libc::c_int;
        value = extract_array_assignment_list(
            ((*(*tlist).word).word)
                .offset(t as isize)
                .offset(1 as libc::c_int as isize),
            &mut oind,
        );
        l = expand_oneword(value, flags);
        sh_xfree(
            value as *mut libc::c_void,
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            11678 as libc::c_int,
        );
        value = string_list(l);
        wlen = (if !value.is_null() && *value.offset(0 as libc::c_int as isize) as libc::c_int != 0
        {
            if *value.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                if *value.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                    strlen(value) as usize
                } else {
                    2 as libc::c_int as usize
                }
            } else {
                1 as libc::c_int as usize
            }
        } else {
            0 as libc::c_int as usize
        }) as libc::c_int;
        temp = sh_xmalloc(
            (t + 3 as libc::c_int + wlen + 1 as libc::c_int) as size_t,
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            11684 as libc::c_int,
        ) as *mut libc::c_char;
        t += 1;
        memcpy(
            temp as *mut libc::c_void,
            (*(*tlist).word).word as *const libc::c_void,
            t as usize,
        );
        let fresh155 = t;
        t = t + 1;
        *temp.offset(fresh155 as isize) = '(' as i32 as libc::c_char;
        if !value.is_null() {
            memcpy(
                temp.offset(t as isize) as *mut libc::c_void,
                value as *const libc::c_void,
                wlen as usize,
            );
        }
        t += wlen;
        let fresh156 = t;
        t = t + 1;
        *temp.offset(fresh156 as isize) = ')' as i32 as libc::c_char;
        *temp.offset(t as isize) = '\u{0}' as i32 as libc::c_char;
        sh_xfree(
            (*(*tlist).word).word as *mut libc::c_void,
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            11694 as libc::c_int,
        );
        let ref mut fresh157 = (*(*tlist).word).word;
        *fresh157 = temp;
        sh_xfree(
            value as *mut libc::c_void,
            b"../subst.c\0" as *const u8 as *const libc::c_char,
            11697 as libc::c_int,
        );
    }
}
fn expand_declaration_argument(
    mut tlist: *mut WORD_LIST,
    mut wcmd: *mut WORD_LIST,
) -> *mut WORD_LIST {
    let mut opts: [libc::c_char; 16] = [0; 16];
    let mut omap: [libc::c_char; 128] = [0; 128];
    let mut t: libc::c_int = 0;
    let mut opti: libc::c_int = 0;
    let mut oind: libc::c_int = 0;
    let mut skip: libc::c_int = 0;
    let mut inheriting: libc::c_int = 0;
    let mut l: *mut WORD_LIST = 0 as *mut WORD_LIST;
    unsafe {
        inheriting = localvar_inherit;
        opti = 0 as libc::c_int;
        if (*(*tlist).word).flags
            & ((1 as libc::c_int) << 22 as libc::c_int
                | (1 as libc::c_int) << 25 as libc::c_int
                | (1 as libc::c_int) << 28 as libc::c_int
                | (1 as libc::c_int) << 23 as libc::c_int)
            != 0
        {
            let fresh158 = opti;
            opti = opti + 1;
            opts[fresh158 as usize] = '-' as i32 as libc::c_char;
        }
        if (*(*tlist).word).flags
            & ((1 as libc::c_int) << 22 as libc::c_int | (1 as libc::c_int) << 25 as libc::c_int)
            == (1 as libc::c_int) << 22 as libc::c_int | (1 as libc::c_int) << 25 as libc::c_int
        {
            let fresh159 = opti;
            opti = opti + 1;
            opts[fresh159 as usize] = 'g' as i32 as libc::c_char;
            let fresh160 = opti;
            opti = opti + 1;
            opts[fresh160 as usize] = 'A' as i32 as libc::c_char;
        } else if (*(*tlist).word).flags & (1 as libc::c_int) << 22 as libc::c_int != 0 {
            let fresh161 = opti;
            opti = opti + 1;
            opts[fresh161 as usize] = 'A' as i32 as libc::c_char;
        } else if (*(*tlist).word).flags
            & ((1 as libc::c_int) << 23 as libc::c_int | (1 as libc::c_int) << 25 as libc::c_int)
            == (1 as libc::c_int) << 23 as libc::c_int | (1 as libc::c_int) << 25 as libc::c_int
        {
            let fresh162 = opti;
            opti = opti + 1;
            opts[fresh162 as usize] = 'g' as i32 as libc::c_char;
            let fresh163 = opti;
            opti = opti + 1;
            opts[fresh163 as usize] = 'a' as i32 as libc::c_char;
        } else if (*(*tlist).word).flags & (1 as libc::c_int) << 23 as libc::c_int != 0 {
            let fresh164 = opti;
            opti = opti + 1;
            opts[fresh164 as usize] = 'a' as i32 as libc::c_char;
        } else if (*(*tlist).word).flags & (1 as libc::c_int) << 25 as libc::c_int != 0 {
            let fresh165 = opti;
            opti = opti + 1;
            opts[fresh165 as usize] = 'g' as i32 as libc::c_char;
        }
        if (*(*tlist).word).flags & (1 as libc::c_int) << 28 as libc::c_int != 0 {
            let fresh166 = opti;
            opti = opti + 1;
            opts[fresh166 as usize] = 'G' as i32 as libc::c_char;
        }
        memset(
            omap.as_mut_ptr() as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<[libc::c_char; 128]>() as usize,
        );
        l = (*wcmd).next;
        while l != tlist {
            if *((*(*l).word).word).offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32 {
                break;
            }
            if *((*(*l).word).word).offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
                && *((*(*l).word).word).offset(1 as libc::c_int as isize) as libc::c_int
                    == '-' as i32
                && *((*(*l).word).word).offset(2 as libc::c_int as isize) as libc::c_int
                    == 0 as libc::c_int
            {
                break;
            }
            oind = 1 as libc::c_int;
            while *((*(*l).word).word).offset(oind as isize) != 0 {
                let mut current_block_28: u64;
                match *((*(*l).word).word).offset(oind as isize) as libc::c_int {
                    73 => {
                        inheriting = 1 as libc::c_int;
                        current_block_28 = 9785635831371406912;
                    }
                    105 | 108 | 117 | 99 => {
                        current_block_28 = 9785635831371406912;
                    }
                    _ => {
                        current_block_28 = 17184638872671510253;
                    }
                }
                match current_block_28 {
                    9785635831371406912 => {
                        omap[*((*(*l).word).word).offset(oind as isize) as usize] =
                            1 as libc::c_int as libc::c_char;
                        if opti == 0 as libc::c_int {
                            let fresh167 = opti;
                            opti = opti + 1;
                            opts[fresh167 as usize] = '-' as i32 as libc::c_char;
                        }
                    }
                    _ => {}
                }
                oind += 1;
            }
            l = (*l).next;
        }
        oind = 0 as libc::c_int;
        while (oind as libc::c_ulong)
            < ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong
        {
            if omap[oind as usize] != 0 {
                let fresh168 = opti;
                opti = opti + 1;
                opts[fresh168 as usize] = oind as libc::c_char;
            }
            oind += 1;
        }
        if (*(*tlist).word).flags
            & ((1 as libc::c_int) << 22 as libc::c_int | (1 as libc::c_int) << 23 as libc::c_int)
            == 0 as libc::c_int
        {
            if opti == 0 as libc::c_int {
                let fresh169 = opti;
                opti = opti + 1;
                opts[fresh169 as usize] = '-' as i32 as libc::c_char;
                let fresh170 = opti;
                opti = opti + 1;
                opts[fresh170 as usize] = '-' as i32 as libc::c_char;
            }
        }
        opts[opti as usize] = '\u{0}' as i32 as libc::c_char;
        expand_compound_assignment_word(
            tlist,
            if (*(*tlist).word).flags & (1 as libc::c_int) << 22 as libc::c_int != 0 {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            },
        );
        skip = 0 as libc::c_int;
        if opti > 0 as libc::c_int {
            t = make_internal_declare(
                (*(*tlist).word).word,
                opts.as_mut_ptr(),
                if !wcmd.is_null() {
                    (*(*wcmd).word).word
                } else {
                    0 as *mut libc::c_char
                },
            );
            if t != 0 as libc::c_int {
                ::std::ptr::write_volatile(&mut last_command_exit_value as *mut libc::c_int, t);
                if (*(*tlist).word).flags & (1 as libc::c_int) << 30 as libc::c_int != 0 {
                    skip = 1 as libc::c_int;
                } else {
                    exp_jump_to_top_level(2 as libc::c_int);
                }
            }
        }
        if skip == 0 as libc::c_int {
            t = do_word_assignment((*tlist).word, 0 as libc::c_int);
            if t == 0 as libc::c_int {
                ::std::ptr::write_volatile(
                    &mut last_command_exit_value as *mut libc::c_int,
                    1 as libc::c_int,
                );
                exp_jump_to_top_level(2 as libc::c_int);
            }
        }
        t = assignment((*(*tlist).word).word, 0 as libc::c_int);
        *((*(*tlist).word).word).offset(t as isize) = '\u{0}' as i32 as libc::c_char;
        if *((*(*tlist).word).word).offset((t - 1 as libc::c_int) as isize) as libc::c_int
            == '+' as i32
        {
            *((*(*tlist).word).word).offset((t - 1 as libc::c_int) as isize) =
                '\u{0}' as i32 as libc::c_char;
        }
        (*(*tlist).word).flags &= !((1 as libc::c_int) << 2 as libc::c_int
            | (1 as libc::c_int) << 4 as libc::c_int
            | (1 as libc::c_int) << 15 as libc::c_int
            | (1 as libc::c_int) << 17 as libc::c_int
            | (1 as libc::c_int) << 22 as libc::c_int
            | (1 as libc::c_int) << 23 as libc::c_int);
    }
    return tlist;
}
fn shell_expand_word_list(mut tlist: *mut WORD_LIST, mut eflags: libc::c_int) -> *mut WORD_LIST {
    let mut expanded: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut orig_list: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut new_list: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut next: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut temp_list: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut wcmd: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut expanded_something: libc::c_int = 0;
    let mut has_dollar_at: libc::c_int = 0;
    new_list = 0 as *mut libc::c_void as *mut WORD_LIST;
    wcmd = new_list;
    orig_list = tlist;
    unsafe {
        while !tlist.is_null() {
            if wcmd.is_null()
                && (*(*tlist).word).flags & (1 as libc::c_int) << 16 as libc::c_int != 0
            {
                wcmd = tlist;
            }
            next = (*tlist).next;
            if (*(*tlist).word).flags
                & ((1 as libc::c_int) << 15 as libc::c_int
                    | (1 as libc::c_int) << 17 as libc::c_int)
                == (1 as libc::c_int) << 15 as libc::c_int | (1 as libc::c_int) << 17 as libc::c_int
            {
                expand_declaration_argument(tlist, wcmd);
            }
            expanded_something = 0 as libc::c_int;
            expanded = expand_word_internal(
                (*tlist).word,
                0 as libc::c_int,
                0 as libc::c_int,
                &mut has_dollar_at,
                &mut expanded_something,
            );
            if expanded == &mut expand_word_error as *mut WORD_LIST
                || expanded == &mut expand_word_fatal as *mut WORD_LIST
            {
                let ref mut fresh171 = (*(*tlist).word).word;
                *fresh171 = 0 as *mut libc::c_void as *mut libc::c_char;
                dispose_words(orig_list);
                dispose_words(new_list);
                ::std::ptr::write_volatile(
                    &mut last_command_exit_value as *mut libc::c_int,
                    1 as libc::c_int,
                );
                if expanded == &mut expand_word_error as *mut WORD_LIST {
                    exp_jump_to_top_level(2 as libc::c_int);
                } else {
                    exp_jump_to_top_level(1 as libc::c_int);
                }
            }
            if expanded_something != 0
                && (*(*tlist).word).flags & (1 as libc::c_int) << 4 as libc::c_int
                    == 0 as libc::c_int
            {
                temp_list = word_list_split(expanded);
                dispose_words(expanded);
            } else {
                word_list_remove_quoted_nulls(expanded);
                temp_list = expanded;
            }
            expanded = if !temp_list.is_null() && !((*temp_list).next).is_null() {
                list_reverse(temp_list as *mut GENERIC_LIST) as *mut WORD_LIST
            } else {
                temp_list
            };
            new_list = list_append(expanded, new_list) as *mut WORD_LIST;
            tlist = next;
        }
        if !orig_list.is_null() {
            dispose_words(orig_list);
        }
        if !new_list.is_null() {
            new_list = if !new_list.is_null() && !((*new_list).next).is_null() {
                list_reverse(new_list as *mut GENERIC_LIST) as *mut WORD_LIST
            } else {
                new_list
            };
        }
    }
    return new_list;
}
fn expand_word_list_internal(mut list: *mut WORD_LIST, mut eflags: libc::c_int) -> *mut WORD_LIST {
    let mut new_list: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut temp_list: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut tint: libc::c_int = 0;
    let mut savecmd: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        tempenv_assign_error = 0 as libc::c_int;
        if list.is_null() {
            return 0 as *mut libc::c_void as *mut WORD_LIST;
        }
        new_list = copy_word_list(list);
        garglist = new_list;
        if eflags & 0x1 as libc::c_int != 0 {
            new_list = separate_out_assignments(new_list);
            garglist = new_list;
            if new_list.is_null() {
                if !subst_assign_varlist.is_null() {
                    temp_list = subst_assign_varlist;
                    while !temp_list.is_null() {
                        savecmd = this_command_name;
                        this_command_name = 0 as *mut libc::c_void as *mut libc::c_char;
                        tint = do_word_assignment((*temp_list).word, 0 as libc::c_int);
                        this_command_name = savecmd;
                        if tint == 0 as libc::c_int {
                            ::std::ptr::write_volatile(
                                &mut last_command_exit_value as *mut libc::c_int,
                                1 as libc::c_int,
                            );
                            if interactive_shell == 0 as libc::c_int
                                && posixly_correct != 0
                                && executing_command_builtin == 0 as libc::c_int
                            {
                                exp_jump_to_top_level(1 as libc::c_int);
                            } else {
                                exp_jump_to_top_level(2 as libc::c_int);
                            }
                        }
                        temp_list = (*temp_list).next;
                    }
                    dispose_words(subst_assign_varlist);
                    subst_assign_varlist = 0 as *mut libc::c_void as *mut WORD_LIST;
                }
                return 0 as *mut libc::c_void as *mut WORD_LIST;
            }
        }
        if eflags & 0x2 as libc::c_int != 0 && brace_expansion != 0 && !new_list.is_null() {
            new_list = brace_expand_word_list(new_list, eflags);
        }
        new_list = shell_expand_word_list(new_list, eflags);
        if !new_list.is_null() {
            if eflags & 0x10 as libc::c_int != 0 && disallow_filename_globbing == 0 as libc::c_int {
                new_list = glob_expand_word_list(new_list, eflags);
            } else {
                new_list = dequote_list(new_list);
            }
        }
        if eflags & 0x1 as libc::c_int != 0 && !subst_assign_varlist.is_null() {
            let mut assign_func: Option<sh_wassign_func_t> = None;
            let mut is_special_builtin: libc::c_int = 0;
            let mut is_builtin_or_func: libc::c_int = 0;
            assign_func = if !new_list.is_null() {
                Some(assign_in_env)
            } else {
                ::std::mem::transmute::<
                    Option<fn() -> libc::c_int>,
                    Option<fn(*mut WORD_DESC, libc::c_int) -> libc::c_int>,
                >(Some(::std::mem::transmute::<
                    fn(*mut WORD_DESC, libc::c_int) -> libc::c_int,
                    fn() -> libc::c_int,
                >(do_word_assignment)))
            };
            tempenv_assign_error = 0 as libc::c_int;
            is_builtin_or_func = (!new_list.is_null()
                && !((*new_list).word).is_null()
                && ((find_shell_builtin((*(*new_list).word).word)).is_some()
                    || !(find_function((*(*new_list).word).word)).is_null()))
                as libc::c_int;
            is_special_builtin = (posixly_correct != 0
                && !new_list.is_null()
                && !((*new_list).word).is_null()
                && (find_special_builtin((*(*new_list).word).word)).is_some())
                as libc::c_int;
            temp_list = subst_assign_varlist;
            while !temp_list.is_null() {
                savecmd = this_command_name;
                this_command_name = 0 as *mut libc::c_void as *mut libc::c_char;
                assigning_in_environment = (assign_func == Some(assign_in_env)) as libc::c_int;
                tint = (Some(assign_func.expect("non-null function pointer")))
                    .expect("non-null function pointer")(
                    (*temp_list).word, is_builtin_or_func
                );
                assigning_in_environment = 0 as libc::c_int;
                this_command_name = savecmd;
                if tint == 0 as libc::c_int {
                    if assign_func
                        == ::std::mem::transmute::<
                            Option<fn() -> libc::c_int>,
                            Option<sh_wassign_func_t>,
                        >(Some(::std::mem::transmute::<
                            fn(*mut WORD_DESC, libc::c_int) -> libc::c_int,
                            fn() -> libc::c_int,
                        >(
                            do_word_assignment
                        )))
                    {
                        ::std::ptr::write_volatile(
                            &mut last_command_exit_value as *mut libc::c_int,
                            1 as libc::c_int,
                        );
                        if interactive_shell == 0 as libc::c_int && posixly_correct != 0 {
                            exp_jump_to_top_level(1 as libc::c_int);
                        } else {
                            exp_jump_to_top_level(2 as libc::c_int);
                        }
                    } else if interactive_shell == 0 as libc::c_int && is_special_builtin != 0 {
                        ::std::ptr::write_volatile(
                            &mut last_command_exit_value as *mut libc::c_int,
                            1 as libc::c_int,
                        );
                        exp_jump_to_top_level(1 as libc::c_int);
                    } else {
                        tempenv_assign_error += 1;
                    }
                }
                temp_list = (*temp_list).next;
            }
            dispose_words(subst_assign_varlist);
            subst_assign_varlist = 0 as *mut libc::c_void as *mut WORD_LIST;
        }
    }
    return new_list;
}
