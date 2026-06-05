use crate::src_common::*;
use crate::unwind_prot::begin_unwind_frame;
use std::convert::TryInto;

use crate::unwind_prot::add_unwind_protect;

use crate::alias::{all_aliases, find_alias};
use crate::array::array_to_argv;
use crate::arrayfunc::{assign_array_var_from_word_list, convert_var_to_array};
use crate::bashline::{
    bash_dequote_text, bash_directory_completion_matches, bash_groupname_completion_function,
    bash_servicename_completion_function, command_word_completion_function, get_hostname_list,
};
use crate::builtins::{complete::compgen_builtin, set::get_minus_o_opts, shopt::get_shopt_options};
use crate::copycmd::copy_word;
use crate::dispose_cmd::{dispose_word_desc, dispose_words};
use crate::execute_cmd::execute_shell_function;
use crate::list::list_length;
use crate::make_cmd::{make_bare_word, make_word, make_word_list};
use crate::pathexp::shell_glob_filename;
use crate::pcomplib::{compspec_copy, compspec_create, compspec_dispose, progcomp_search};
use crate::readline::c_rl_filename_completion_function;
use crate::readline::{
    c_rl_completion_matches, c_rl_ding, c_rl_funmap_names, c_rl_on_new_line,
    c_rl_username_completion_function, rl_completer_word_break_characters,
    rl_completion_found_quote, rl_completion_invoking_key, rl_completion_mark_symlink_dirs,
    rl_completion_quote_character, rl_completion_suppress_append, rl_completion_type,
    rl_filename_completion_desired, rl_filename_dequoting_function, rl_line_buffer, rl_point,
    rl_readline_state, rl_sort_completion_matches,
};
use crate::stringlib::{strcreplace, substring};
use crate::subst::{command_substitute, expand_words_shellexp, skip_to_delim, split_at_delims};
use crate::unwind_prot::discard_unwind_frame;
use crate::variables::{
    all_array_variables, all_exported_variables, all_visible_functions, all_visible_variables,
    bind_int_variable, bind_variable, find_function, find_variable_noref, make_new_array_variable,
    unbind_variable_noref,
};
use crate::y_tab::{restore_parser_state, save_parser_state, word_token_alist};

// pub union Functions {
//     restore_parser_state: unsafe extern "C" fn(*mut sh_parser_state_t) -> (),
//     dispose_words: unsafe extern "C" fn(*mut WORD_LIST) -> (),
//     unbind_compfunc_variables: unsafe extern "C" fn(libc::c_int) -> (),
// }

pub type SVFUNC = fn() -> *mut *mut SHELL_VAR;

pub type rl_dequote_func_t = fn(*mut libc::c_char, libc::c_int) -> *mut libc::c_char;

#[no_mangle]
pub static mut prog_completion_enabled: libc::c_int = 1;

#[no_mangle]
pub static mut progcomp_alias: libc::c_int = 0 as libc::c_int;

#[no_mangle]
pub static mut it_aliases: ITEMLIST = {
    {
        let init = _list_of_items {
            flags: 0 as libc::c_int,
            list_getter: Some(it_init_aliases),
            slist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genlist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genindex: 0,
        };
        init
    }
};

#[no_mangle]
pub static mut it_arrayvars: ITEMLIST = {
    {
        let init = _list_of_items {
            flags: LIST_DYNAMIC as libc::c_int,
            list_getter: Some(it_init_arrayvars),
            slist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genlist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genindex: 0,
        };
        init
    }
};
#[no_mangle]
pub static mut it_bindings: ITEMLIST = {
    {
        let init = _list_of_items {
            flags: 0 as libc::c_int,
            list_getter: Some(it_init_bindings),
            slist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genlist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genindex: 0,
        };
        init
    }
};
#[no_mangle]
pub static mut it_builtins: ITEMLIST = {
    {
        let init = _list_of_items {
            flags: 0 as libc::c_int,
            list_getter: Some(it_init_builtins),
            slist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genlist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genindex: 0,
        };
        init
    }
};
#[no_mangle]
pub static mut it_disabled: ITEMLIST = {
    {
        let init = _list_of_items {
            flags: 0 as libc::c_int,
            list_getter: Some(it_init_disabled),
            slist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genlist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genindex: 0,
        };
        init
    }
};
#[no_mangle]
pub static mut it_enabled: ITEMLIST = {
    {
        let init = _list_of_items {
            flags: 0 as libc::c_int,
            list_getter: Some(it_init_enabled),
            slist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genlist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genindex: 0,
        };
        init
    }
};
#[no_mangle]
pub static mut it_exports: ITEMLIST = {
    {
        let init = _list_of_items {
            flags: LIST_DYNAMIC as libc::c_int,
            list_getter: Some(it_init_exported),
            slist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genlist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genindex: 0,
        };
        init
    }
};
#[no_mangle]
pub static mut it_functions: ITEMLIST = {
    {
        let init = _list_of_items {
            flags: 0 as libc::c_int,
            list_getter: Some(it_init_functions),
            slist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genlist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genindex: 0,
        };
        init
    }
};
#[no_mangle]
pub static mut it_helptopics: ITEMLIST = {
    {
        let init = _list_of_items {
            flags: 0 as libc::c_int,
            list_getter: Some(it_init_helptopics),
            slist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genlist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genindex: 0,
        };
        init
    }
};
#[no_mangle]
pub static mut it_hostnames: ITEMLIST = {
    {
        let init = _list_of_items {
            flags: LIST_DYNAMIC as libc::c_int,
            list_getter: Some(it_init_hostnames),
            slist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genlist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genindex: 0,
        };
        init
    }
};
#[no_mangle]
pub static mut it_jobs: ITEMLIST = {
    {
        let init = _list_of_items {
            flags: LIST_DYNAMIC as libc::c_int,
            list_getter: Some(it_init_jobs),
            slist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genlist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genindex: 0,
        };
        init
    }
};
#[no_mangle]
pub static mut it_keywords: ITEMLIST = {
    {
        let init = _list_of_items {
            flags: 0 as libc::c_int,
            list_getter: Some(it_init_keywords),
            slist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genlist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genindex: 0,
        };
        init
    }
};
#[no_mangle]
pub static mut it_running: ITEMLIST = {
    {
        let init = _list_of_items {
            flags: LIST_DYNAMIC as libc::c_int,
            list_getter: Some(it_init_running),
            slist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genlist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genindex: 0,
        };
        init
    }
};
#[no_mangle]
pub static mut it_setopts: ITEMLIST = {
    {
        let init = _list_of_items {
            flags: 0 as libc::c_int,
            list_getter: Some(it_init_setopts),
            slist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genlist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genindex: 0,
        };
        init
    }
};
#[no_mangle]
pub static mut it_shopts: ITEMLIST = {
    {
        let init = _list_of_items {
            flags: 0 as libc::c_int,
            list_getter: Some(it_init_shopts),
            slist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genlist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genindex: 0,
        };
        init
    }
};
#[no_mangle]
pub static mut it_signals: ITEMLIST = {
    {
        let init = _list_of_items {
            flags: 0 as libc::c_int,
            list_getter: Some(it_init_signals),
            slist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genlist: 0 as *const STRINGLIST as *mut STRINGLIST,
            genindex: 0,
        };
        init
    }
};
#[no_mangle]
pub static mut it_stopped: ITEMLIST = {
    let init = _list_of_items {
        flags: LIST_DYNAMIC as libc::c_int,
        list_getter: Some(it_init_stopped),
        slist: 0 as *const STRINGLIST as *mut STRINGLIST,
        genlist: 0 as *const STRINGLIST as *mut STRINGLIST,
        genindex: 0,
    };
    init
};
#[no_mangle]
pub static mut it_variables: ITEMLIST = {
    let init = _list_of_items {
        flags: LIST_DYNAMIC as libc::c_int,
        list_getter: Some(it_init_variables),
        slist: 0 as *const STRINGLIST as *mut STRINGLIST,
        genlist: 0 as *const STRINGLIST as *mut STRINGLIST,
        genindex: 0,
    };
    init
};
#[no_mangle]
pub static mut pcomp_curcs: *mut COMPSPEC = 0 as *const COMPSPEC as *mut COMPSPEC;
#[no_mangle]
pub static mut pcomp_curcmd: *const libc::c_char = 0 as *const libc::c_char;
#[no_mangle]
pub static mut pcomp_curtxt: *const libc::c_char = 0 as *const libc::c_char;
#[no_mangle]
pub static mut pcomp_line: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut pcomp_ind: libc::c_int = 0;
#[no_mangle]
pub fn set_itemlist_dirty(it: *mut ITEMLIST) {
    unsafe {
        (*it).flags |= LIST_DIRTY as libc::c_int;
    }
}
#[no_mangle]
pub fn initialize_itemlist(itp: *mut ITEMLIST) {
    unsafe {
        (Some(((*itp).list_getter).expect("non-null function pointer")))
            .expect("non-null function pointer")(itp);
        (*itp).flags |= LIST_INITIALIZED as libc::c_int;
        (*itp).flags &= !(LIST_DIRTY as libc::c_int);
    }
}
#[no_mangle]
pub fn clean_itemlist(itp: *mut ITEMLIST) {
    let sl: *mut STRINGLIST;
    unsafe {
        sl = (*itp).slist;
        if !sl.is_null() {
            if (*itp).flags & (0x20 as libc::c_int | 0x10 as libc::c_int) == 0 as libc::c_int {
                c_strvec_flush((*sl).list);
            }
            if (*itp).flags & 0x10 as libc::c_int == 0 as libc::c_int {
                free((*sl).list as *mut libc::c_void);
            }
            free(sl as *mut libc::c_void);
        }
        let ref mut fresh0 = (*itp).slist;
        *fresh0 = 0 as *mut libc::c_void as *mut STRINGLIST;
        (*itp).flags &= !(0x10 as libc::c_int
            | 0x20 as libc::c_int
            | LIST_INITIALIZED as libc::c_int
            | LIST_DIRTY as libc::c_int);
    }
}
#[no_mangle]
fn shouldexp_filterpat(s: *mut libc::c_char) -> libc::c_int {
    let mut p: *mut libc::c_char;
    p = s;
    unsafe {
        while !p.is_null() && *p as libc::c_int != 0 {
            if *p as libc::c_int == '\\' as i32 {
                p = p.offset(1);
            } else if *p as libc::c_int == '&' as i32 {
                return 1 as libc::c_int;
            }
            p = p.offset(1);
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
fn preproc_filterpat(pat: *mut libc::c_char, text: *const libc::c_char) -> *mut libc::c_char {
    let ret: *mut libc::c_char;
    ret = strcreplace(pat, '&' as i32, text, 1 as libc::c_int);
    return ret;
}
#[no_mangle]
pub fn filter_stringlist(
    sl: *mut STRINGLIST,
    filterpat: *mut libc::c_char,
    text: *const libc::c_char,
) -> *mut STRINGLIST {
    let mut i: libc::c_int;
    let mut m: libc::c_int;
    let not: libc::c_int;
    let ret: *mut STRINGLIST;
    let npat: *mut libc::c_char;
    let t: *mut libc::c_char;
    unsafe {
        if sl.is_null() || ((*sl).list).is_null() || (*sl).list_len == 0 as libc::c_int {
            return sl;
        }
        npat = if shouldexp_filterpat(filterpat) != 0 {
            preproc_filterpat(filterpat, text)
        } else {
            filterpat
        };
        not = (*npat.offset(0 as libc::c_int as isize) as libc::c_int == '!' as i32
            && (extended_glob == 0 as libc::c_int
                || *npat.offset(1 as libc::c_int as isize) as libc::c_int != '(' as i32))
            as libc::c_int;
        t = if not != 0 {
            npat.offset(1 as libc::c_int as isize)
        } else {
            npat
        };
        ret = c_strlist_create((*sl).list_size);
        i = 0 as libc::c_int;
        while i < (*sl).list_len {
            m = c_strmatch(
                t,
                *((*sl).list).offset(i as isize),
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
            if not != 0 && m == 1 as libc::c_int || not == 0 as libc::c_int && m != 1 as libc::c_int
            {
                free(*((*sl).list).offset(i as isize) as *mut libc::c_void);
            } else {
                let ref mut fresh1 = (*ret).list_len;
                let fresh2 = *fresh1;
                *fresh1 = *fresh1 + 1;
                let ref mut fresh3 = *((*ret).list).offset(fresh2 as isize);
                *fresh3 = *((*sl).list).offset(i as isize);
            }
            i += 1;
        }
        let ref mut fresh4 = *((*ret).list).offset((*ret).list_len as isize);
        *fresh4 = 0 as *mut libc::c_void as *mut libc::c_char;
        if npat != filterpat {
            free(npat as *mut libc::c_void);
        }
        return ret;
    }
}
#[no_mangle]
pub fn completions_to_stringlist(matches: *mut *mut libc::c_char) -> *mut STRINGLIST {
    let sl: *mut STRINGLIST;
    let mlen: libc::c_int;
    let mut i: libc::c_int;
    let mut n: libc::c_int;
    unsafe {
        mlen = if matches.is_null() {
            0 as libc::c_int
        } else {
            c_strvec_len(matches)
        };
        sl = c_strlist_create(mlen + 1 as libc::c_int);
        if matches.is_null() || (*matches.offset(0 as libc::c_int as isize)).is_null() {
            return sl;
        }
        if (*matches.offset(1 as libc::c_int as isize)).is_null() {
            let ref mut fresh5 = *((*sl).list).offset(0 as libc::c_int as isize);
            *fresh5 = if !(*matches.offset(0 as libc::c_int as isize)).is_null() {
                strcpy(
                    malloc(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_add(strlen(*matches.offset(0 as libc::c_int as isize)) as u64)
                            .try_into()
                            .unwrap(),
                    ) as *mut libc::c_char,
                    *matches.offset(0 as libc::c_int as isize),
                )
            } else {
                0 as *mut libc::c_void as *mut libc::c_char
            };
            let ref mut fresh6 = (*sl).list_len;
            *fresh6 = 1 as libc::c_int;
            let ref mut fresh7 = *((*sl).list).offset(*fresh6 as isize);
            *fresh7 = 0 as *mut libc::c_void as *mut libc::c_char;
            return sl;
        }
        i = 1 as libc::c_int;
        n = 0 as libc::c_int;
        while i < mlen {
            let ref mut fresh8 = *((*sl).list).offset(n as isize);
            *fresh8 = if !(*matches.offset(i as isize)).is_null() {
                strcpy(
                    malloc(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_add(strlen(*matches.offset(i as isize)) as u64)
                            .try_into()
                            .unwrap(),
                    ) as *mut libc::c_char,
                    *matches.offset(i as isize),
                )
            } else {
                0 as *mut libc::c_void as *mut libc::c_char
            };
            i += 1;
            n += 1;
        }
        (*sl).list_len = n;
        let ref mut fresh9 = *((*sl).list).offset(n as isize);
        *fresh9 = 0 as *mut libc::c_void as *mut libc::c_char;
        return sl;
    }
}
#[no_mangle]
fn it_init_aliases(itp: *mut ITEMLIST) -> libc::c_int {
    let alias_list: *mut *mut alias_t;
    let mut i: libc::c_int;
    let mut n: libc::c_int;
    let sl: *mut STRINGLIST;
    unsafe {
        alias_list = all_aliases();
        if alias_list.is_null() {
            let ref mut fresh10 = (*itp).slist;
            *fresh10 = 0 as *mut libc::c_void as *mut STRINGLIST;
            return 0 as libc::c_int;
        }
        n = 0 as libc::c_int;
        while !(*alias_list.offset(n as isize)).is_null() {
            n += 1;
        }
        sl = c_strlist_create(n + 1 as libc::c_int);
        i = 0 as libc::c_int;
        while i < n {
            let ref mut fresh11 = *((*sl).list).offset(i as isize);
            *fresh11 = if !((**alias_list.offset(i as isize)).name).is_null() {
                strcpy(
                    malloc(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_add(strlen((**alias_list.offset(i as isize)).name) as u64)
                            .try_into()
                            .unwrap(),
                    ) as *mut libc::c_char,
                    (**alias_list.offset(i as isize)).name,
                )
            } else {
                0 as *mut libc::c_void as *mut libc::c_char
            };
            i += 1;
        }
        let ref mut fresh12 = *((*sl).list).offset(n as isize);
        *fresh12 = 0 as *mut libc::c_void as *mut libc::c_char;
        let ref mut fresh13 = (*sl).list_len;
        *fresh13 = n;
        (*sl).list_size = *fresh13;
        let ref mut fresh14 = (*itp).slist;
        *fresh14 = sl;
        free(alias_list as *mut libc::c_void);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
fn init_itemlist_from_varlist(itp: *mut ITEMLIST, svfunc: Option<SVFUNC>) {
    let vlist: *mut *mut SHELL_VAR;
    let sl: *mut STRINGLIST;
    let mut i: libc::c_int;
    let mut n: libc::c_int;
    unsafe {
        vlist = ::std::mem::transmute::<_, fn() -> *mut *mut SHELL_VAR>(
            (Some(svfunc.expect("non-null function pointer"))).expect("non-null function pointer"),
        )();
        if vlist.is_null() {
            let ref mut fresh15 = (*itp).slist;
            *fresh15 = 0 as *mut libc::c_void as *mut STRINGLIST;
            return;
        }
        n = 0 as libc::c_int;
        while !(*vlist.offset(n as isize)).is_null() {
            n += 1;
        }
        sl = c_strlist_create(n + 1 as libc::c_int);
        i = 0 as libc::c_int;
        while i < n {
            let ref mut fresh16 = *((*sl).list).offset(i as isize);
            *fresh16 = strcpy(
                malloc(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(strlen((**vlist.offset(i as isize)).name) as u64)
                        .try_into()
                        .unwrap(),
                ) as *mut libc::c_char,
                (**vlist.offset(i as isize)).name,
            );
            i += 1;
        }
        let ref mut fresh17 = (*sl).list_len;
        *fresh17 = n;
        let ref mut fresh18 = *((*sl).list).offset(*fresh17 as isize);
        *fresh18 = 0 as *mut libc::c_void as *mut libc::c_char;
        let ref mut fresh19 = (*itp).slist;
        *fresh19 = sl;
    }
}
#[no_mangle]
fn it_init_arrayvars(itp: *mut ITEMLIST) -> libc::c_int {
    unsafe {
        init_itemlist_from_varlist(
            itp,
            ::std::mem::transmute::<Option<fn() -> *mut *mut SHELL_VAR>, Option<SVFUNC>>(Some(
                all_array_variables,
            )),
        );
    }
    return 1 as libc::c_int;
}
#[no_mangle]
fn it_init_bindings(itp: *mut ITEMLIST) -> libc::c_int {
    let blist: *mut *mut libc::c_char;
    let sl: *mut STRINGLIST;
    unsafe {
        blist = c_rl_funmap_names() as *mut *mut libc::c_char;
        sl = c_strlist_create(0 as libc::c_int);
        let ref mut fresh20 = (*sl).list;
        *fresh20 = blist;
        (*sl).list_size = 0 as libc::c_int;
        (*sl).list_len = c_strvec_len((*sl).list);
        (*itp).flags |= 0x20 as libc::c_int;
        let ref mut fresh21 = (*itp).slist;
        *fresh21 = sl;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
fn it_init_builtins(itp: *mut ITEMLIST) -> libc::c_int {
    let sl: *mut STRINGLIST;
    let mut i: libc::c_int;
    let mut n: libc::c_int;
    unsafe {
        sl = c_strlist_create(num_shell_builtins);
        n = 0 as libc::c_int;
        i = n;
        while i < num_shell_builtins {
            if ((*shell_builtins.offset(i as isize)).function).is_some() {
                let fresh22 = n;
                n = n + 1;
                let ref mut fresh23 = *((*sl).list).offset(fresh22 as isize);
                *fresh23 = (*shell_builtins.offset(i as isize)).name;
            }
            i += 1;
        }
        let ref mut fresh24 = (*sl).list_len;
        *fresh24 = n;
        let ref mut fresh25 = *((*sl).list).offset(*fresh24 as isize);
        *fresh25 = 0 as *mut libc::c_void as *mut libc::c_char;
        (*itp).flags |= 0x20 as libc::c_int;
        let ref mut fresh26 = (*itp).slist;
        *fresh26 = sl;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
fn it_init_enabled(itp: *mut ITEMLIST) -> libc::c_int {
    let sl: *mut STRINGLIST;
    let mut i: libc::c_int;
    let mut n: libc::c_int;
    unsafe {
        sl = c_strlist_create(num_shell_builtins);
        n = 0 as libc::c_int;
        i = n;
        while i < num_shell_builtins {
            if ((*shell_builtins.offset(i as isize)).function).is_some()
                && (*shell_builtins.offset(i as isize)).flags & LIST_DYNAMIC as libc::c_int != 0
            {
                let fresh27 = n;
                n = n + 1;
                let ref mut fresh28 = *((*sl).list).offset(fresh27 as isize);
                *fresh28 = (*shell_builtins.offset(i as isize)).name;
            }
            i += 1;
        }
        let ref mut fresh29 = (*sl).list_len;
        *fresh29 = n;
        let ref mut fresh30 = *((*sl).list).offset(*fresh29 as isize);
        *fresh30 = 0 as *mut libc::c_void as *mut libc::c_char;
        (*itp).flags |= 0x20 as libc::c_int;
        let ref mut fresh31 = (*itp).slist;
        *fresh31 = sl;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
fn it_init_disabled(itp: *mut ITEMLIST) -> libc::c_int {
    let sl: *mut STRINGLIST;
    let mut i: libc::c_int;
    let mut n: libc::c_int;
    unsafe {
        sl = c_strlist_create(num_shell_builtins);
        n = 0 as libc::c_int;
        i = n;
        while i < num_shell_builtins {
            if ((*shell_builtins.offset(i as isize)).function).is_some()
                && (*shell_builtins.offset(i as isize)).flags & LIST_DYNAMIC as libc::c_int
                    == 0 as libc::c_int
            {
                let fresh32 = n;
                n = n + 1;
                let ref mut fresh33 = *((*sl).list).offset(fresh32 as isize);
                *fresh33 = (*shell_builtins.offset(i as isize)).name;
            }
            i += 1;
        }
        let ref mut fresh34 = (*sl).list_len;
        *fresh34 = n;
        let ref mut fresh35 = *((*sl).list).offset(*fresh34 as isize);
        *fresh35 = 0 as *mut libc::c_void as *mut libc::c_char;
        (*itp).flags |= 0x20 as libc::c_int;
        let ref mut fresh36 = (*itp).slist;
        *fresh36 = sl;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
fn it_init_exported(itp: *mut ITEMLIST) -> libc::c_int {
    unsafe {
        init_itemlist_from_varlist(
            itp,
            ::std::mem::transmute::<Option<fn() -> *mut *mut SHELL_VAR>, Option<SVFUNC>>(Some(
                all_exported_variables,
            )),
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
fn it_init_functions(itp: *mut ITEMLIST) -> libc::c_int {
    unsafe {
        init_itemlist_from_varlist(
            itp,
            ::std::mem::transmute::<Option<fn() -> *mut *mut SHELL_VAR>, Option<SVFUNC>>(Some(
                all_visible_functions,
            )),
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
fn it_init_helptopics(itp: *mut ITEMLIST) -> libc::c_int {
    let sl: *mut STRINGLIST;
    let mut i: libc::c_int;
    let mut n: libc::c_int;
    unsafe {
        sl = c_strlist_create(num_shell_builtins);
        n = 0 as libc::c_int;
        i = n;
        while i < num_shell_builtins {
            let fresh37 = n;
            n = n + 1;
            let ref mut fresh38 = *((*sl).list).offset(fresh37 as isize);
            *fresh38 = (*shell_builtins.offset(i as isize)).name;
            i += 1;
        }
        let ref mut fresh39 = (*sl).list_len;
        *fresh39 = n;
        let ref mut fresh40 = *((*sl).list).offset(*fresh39 as isize);
        *fresh40 = 0 as *mut libc::c_void as *mut libc::c_char;
        (*itp).flags |= 0x20 as libc::c_int;
        let ref mut fresh41 = (*itp).slist;
        *fresh41 = sl;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
fn it_init_hostnames(itp: *mut ITEMLIST) -> libc::c_int {
    let sl: *mut STRINGLIST;
    unsafe {
        sl = c_strlist_create(0 as libc::c_int);
        let ref mut fresh42 = (*sl).list;
        *fresh42 = get_hostname_list();
        (*sl).list_len = if !((*sl).list).is_null() {
            c_strvec_len((*sl).list)
        } else {
            0 as libc::c_int
        };
        (*sl).list_size = (*sl).list_len;
        let ref mut fresh43 = (*itp).slist;
        *fresh43 = sl;
        (*itp).flags |= 0x20 as libc::c_int | 0x10 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
fn it_init_joblist(itp: *mut ITEMLIST, jstate: libc::c_int) -> libc::c_int {
    let sl: *mut STRINGLIST;
    let mut i: libc::c_int;
    let mut p: *mut PROCESS;
    let mut s: *mut libc::c_char;
    let mut t: *mut libc::c_char;
    let mut j: *mut JOB;
    let mut ws: JOB_STATE;
    ws = JNONE;
    if jstate == 0 as libc::c_int {
        ws = JRUNNING;
    } else if jstate == 1 as libc::c_int {
        ws = JSTOPPED;
    }
    unsafe {
        sl = c_strlist_create(js.j_jobslots);
        i = js.j_jobslots - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            j = *jobs.offset(i as isize);
            if !j.is_null() {
                p = (*j).pipe;
                if jstate == -(1 as libc::c_int)
                    || (**jobs.offset(i as isize)).state as libc::c_int == ws as libc::c_int
                {
                    s = strcpy(
                        malloc(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(strlen((*p).command) as u64)
                                .try_into()
                                .unwrap(),
                        ) as *mut libc::c_char,
                        (*p).command,
                    );
                    t = strpbrk(s, b" \t\n\0" as *const u8 as *const libc::c_char);
                    if !t.is_null() {
                        *t = '\u{0}' as i32 as libc::c_char;
                    }
                    let ref mut fresh44 = (*sl).list_len;
                    let fresh45 = *fresh44;
                    *fresh44 = *fresh44 + 1;
                    let ref mut fresh46 = *((*sl).list).offset(fresh45 as isize);
                    *fresh46 = s;
                }
            }
            i -= 1;
        }
        let ref mut fresh47 = (*itp).slist;
        *fresh47 = sl;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
fn it_init_jobs(itp: *mut ITEMLIST) -> libc::c_int {
    return it_init_joblist(itp, -(1 as libc::c_int));
}
#[no_mangle]
fn it_init_running(itp: *mut ITEMLIST) -> libc::c_int {
    return it_init_joblist(itp, 0 as libc::c_int);
}
#[no_mangle]
fn it_init_stopped(itp: *mut ITEMLIST) -> libc::c_int {
    return it_init_joblist(itp, 1 as libc::c_int);
}
#[no_mangle]
fn it_init_keywords(itp: *mut ITEMLIST) -> libc::c_int {
    let sl: *mut STRINGLIST;
    let mut i: libc::c_int;
    let mut n: libc::c_int;
    n = 0 as libc::c_int;
    unsafe {
        while !((*word_token_alist.as_mut_ptr().offset(n as isize)).word).is_null() {
            n += 1;
        }
        sl = c_strlist_create(n);
        i = 0 as libc::c_int;
        while i < n {
            let ref mut fresh48 = *((*sl).list).offset(i as isize);
            *fresh48 = (*word_token_alist.as_mut_ptr().offset(i as isize)).word;
            i += 1;
        }
        let ref mut fresh49 = (*sl).list_len;
        *fresh49 = i;
        let ref mut fresh50 = *((*sl).list).offset(*fresh49 as isize);
        *fresh50 = 0 as *mut libc::c_void as *mut libc::c_char;
        (*itp).flags |= 0x20 as libc::c_int;
        let ref mut fresh51 = (*itp).slist;
        *fresh51 = sl;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
fn it_init_signals(itp: *mut ITEMLIST) -> libc::c_int {
    let sl: *mut STRINGLIST;
    unsafe {
        sl = c_strlist_create(0 as libc::c_int);
        let ref mut fresh52 = (*sl).list;
        *fresh52 = signal_names.as_mut_ptr();
        (*sl).list_len = c_strvec_len((*sl).list);
        (*itp).flags |= 0x10 as libc::c_int;
        let ref mut fresh53 = (*itp).slist;
        *fresh53 = sl;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
fn it_init_variables(itp: *mut ITEMLIST) -> libc::c_int {
    unsafe {
        init_itemlist_from_varlist(
            itp,
            ::std::mem::transmute::<Option<fn() -> *mut *mut SHELL_VAR>, Option<SVFUNC>>(Some(
                all_visible_variables,
            )),
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
fn it_init_setopts(itp: *mut ITEMLIST) -> libc::c_int {
    let sl: *mut STRINGLIST;
    unsafe {
        sl = c_strlist_create(0 as libc::c_int);
        let ref mut fresh54 = (*sl).list;
        *fresh54 = get_minus_o_opts();
        (*sl).list_len = c_strvec_len((*sl).list);
        let ref mut fresh55 = (*itp).slist;
        *fresh55 = sl;
        (*itp).flags |= 0x20 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
fn it_init_shopts(itp: *mut ITEMLIST) -> libc::c_int {
    let sl: *mut STRINGLIST;
    unsafe {
        sl = c_strlist_create(0 as libc::c_int);
        let ref mut fresh56 = (*sl).list;
        *fresh56 = get_shopt_options();
        (*sl).list_len = c_strvec_len((*sl).list);
        let ref mut fresh57 = (*itp).slist;
        *fresh57 = sl;
        (*itp).flags |= 0x20 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
fn gen_matches_from_itemlist(itp: *mut ITEMLIST, text: *const libc::c_char) -> *mut STRINGLIST {
    let ret: *mut STRINGLIST;
    let sl: *mut STRINGLIST;
    let tlen: libc::c_int;
    let mut i: libc::c_int;
    let mut n: libc::c_int;
    let ntxt: *mut libc::c_char;
    unsafe {
        if (*itp).flags & (LIST_DIRTY as libc::c_int | LIST_DYNAMIC as libc::c_int) != 0
            || (*itp).flags & LIST_INITIALIZED as libc::c_int == 0 as libc::c_int
        {
            if (*itp).flags & (LIST_DIRTY as libc::c_int | LIST_DYNAMIC as libc::c_int) != 0 {
                clean_itemlist(itp);
            }
            if (*itp).flags & LIST_INITIALIZED as libc::c_int == 0 as libc::c_int {
                initialize_itemlist(itp);
            }
        }
        if ((*itp).slist).is_null() {
            return 0 as *mut libc::c_void as *mut STRINGLIST;
        }
        ret = c_strlist_create((*(*itp).slist).list_len + 1 as libc::c_int);
        sl = (*itp).slist;
        ntxt = bash_dequote_text(text);
        tlen = (if !ntxt.is_null() && *ntxt.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            if *ntxt.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                if *ntxt.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                    strlen(ntxt) as usize
                } else {
                    2 as libc::c_int as usize
                }
            } else {
                1 as libc::c_int as usize
            }
        } else {
            0 as libc::c_int as usize
        }) as libc::c_int;
        n = 0 as libc::c_int;
        i = n;
        while i < (*sl).list_len {
            if tlen == 0 as libc::c_int
                || (if tlen == 0 as libc::c_int {
                    1 as libc::c_int
                } else {
                    (*(*((*sl).list).offset(i as isize)).offset(0 as libc::c_int as isize)
                        as libc::c_int
                        == *ntxt.offset(0 as libc::c_int as isize) as libc::c_int
                        && strncmp(
                            *((*sl).list).offset(i as isize),
                            ntxt,
                            (tlen as libc::c_ulong).try_into().unwrap(),
                        ) == 0 as libc::c_int) as libc::c_int
                }) != 0
            {
                let fresh58 = n;
                n = n + 1;
                let ref mut fresh59 = *((*ret).list).offset(fresh58 as isize);
                *fresh59 = if !(*((*sl).list).offset(i as isize)).is_null() {
                    strcpy(
                        malloc(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(strlen(*((*sl).list).offset(i as isize)) as u64)
                                .try_into()
                                .unwrap(),
                        ) as *mut libc::c_char,
                        *((*sl).list).offset(i as isize),
                    )
                } else {
                    0 as *mut libc::c_void as *mut libc::c_char
                };
            }
            i += 1;
        }
        let ref mut fresh60 = (*ret).list_len;
        *fresh60 = n;
        let ref mut fresh61 = *((*ret).list).offset(*fresh60 as isize);
        *fresh61 = 0 as *mut libc::c_void as *mut libc::c_char;
        if !ntxt.is_null() {
            free(ntxt as *mut libc::c_void);
        }
    }
    return ret;
}
#[no_mangle]
fn pcomp_filename_completion_function(
    text: *const libc::c_char,
    state: libc::c_int,
) -> *mut libc::c_char {
    static mut dfn: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    let iscompgen: libc::c_int;
    let iscompleting: libc::c_int;
    unsafe {
        if state == 0 as libc::c_int {
            if !dfn.is_null() {
                free(dfn as *mut libc::c_void);
            }
            iscompgen = (this_shell_builtin == Some(compgen_builtin)) as libc::c_int;
            iscompleting =
                (rl_readline_state & 0x4000 as libc::c_int as libc::c_ulong) as libc::c_int;
            if iscompgen != 0
                && iscompleting == 0 as libc::c_int
                && rl_completion_found_quote == 0 as libc::c_int
                && rl_filename_dequoting_function.is_some()
            {
                dfn = (Some(rl_filename_dequoting_function.expect("non-null function pointer")))
                    .expect("non-null function pointer")(
                    text as *mut libc::c_char,
                    rl_completion_quote_character,
                );
            } else if iscompgen != 0
                && iscompleting != 0
                && !pcomp_curtxt.is_null()
                && *pcomp_curtxt as libc::c_int == 0 as libc::c_int
                && !text.is_null()
                && (*text as libc::c_int == '\'' as i32 || *text as libc::c_int == '"' as i32)
                && *text.offset(1 as libc::c_int as isize) as libc::c_int
                    == *text.offset(0 as libc::c_int as isize) as libc::c_int
                && *text.offset(2 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
                && rl_filename_dequoting_function.is_some()
            {
                dfn = (Some(rl_filename_dequoting_function.expect("non-null function pointer")))
                    .expect("non-null function pointer")(
                    text as *mut libc::c_char,
                    rl_completion_quote_character,
                );
            } else if iscompgen != 0
                && iscompleting != 0
                && rl_filename_dequoting_function.is_some()
                && !pcomp_curtxt.is_null()
                && !text.is_null()
                && (*pcomp_curtxt.offset(0 as libc::c_int as isize) as libc::c_int
                    == *text.offset(0 as libc::c_int as isize) as libc::c_int
                    && strcmp(pcomp_curtxt, text) == 0 as libc::c_int)
                    as libc::c_int
                    == 0 as libc::c_int
                && variable_context != 0
                && c_sh_contains_quotes(text) != 0
            {
                dfn = (Some(rl_filename_dequoting_function.expect("non-null function pointer")))
                    .expect("non-null function pointer")(
                    text as *mut libc::c_char,
                    rl_completion_quote_character,
                );
            } else {
                dfn = strcpy(
                    malloc(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_add(strlen(text) as u64)
                            .try_into()
                            .unwrap(),
                    ) as *mut libc::c_char,
                    text,
                );
            }
        }
        return c_rl_filename_completion_function(dfn, state);
    }
}
#[no_mangle]
fn gen_action_completions(cs: *mut COMPSPEC, text: *const libc::c_char) -> *mut STRINGLIST {
    let mut ret: *mut STRINGLIST;
    let mut tmatches: *mut STRINGLIST;
    let mut cmatches: *mut *mut libc::c_char;
    let flags: libc::c_ulong;
    let t: libc::c_int;
    unsafe {
        tmatches = 0 as *mut libc::c_void as *mut STRINGLIST;
        ret = tmatches;

        flags = (*cs).actions;
        if flags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong != 0 {
            tmatches = gen_matches_from_itemlist(&mut it_aliases, text);
            if !tmatches.is_null() {
                ret = c_strlist_append(ret, tmatches);
                c_strlist_dispose(tmatches);
            }
        }
        if flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong != 0 {
            tmatches = gen_matches_from_itemlist(&mut it_arrayvars, text);
            if !tmatches.is_null() {
                ret = c_strlist_append(ret, tmatches);
                c_strlist_dispose(tmatches);
            }
        }
        if flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong != 0 {
            tmatches = gen_matches_from_itemlist(&mut it_bindings, text);
            if !tmatches.is_null() {
                ret = c_strlist_append(ret, tmatches);
                c_strlist_dispose(tmatches);
            }
        }
        if flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong != 0 {
            tmatches = gen_matches_from_itemlist(&mut it_builtins, text);
            if !tmatches.is_null() {
                ret = c_strlist_append(ret, tmatches);
                c_strlist_dispose(tmatches);
            }
        }
        if flags & ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_ulong != 0 {
            tmatches = gen_matches_from_itemlist(&mut it_disabled, text);
            if !tmatches.is_null() {
                ret = c_strlist_append(ret, tmatches);
                c_strlist_dispose(tmatches);
            }
        }
        if flags & ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_ulong != 0 {
            tmatches = gen_matches_from_itemlist(&mut it_enabled, text);
            if !tmatches.is_null() {
                ret = c_strlist_append(ret, tmatches);
                c_strlist_dispose(tmatches);
            }
        }
        if flags & ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_ulong != 0 {
            tmatches = gen_matches_from_itemlist(&mut it_exports, text);
            if !tmatches.is_null() {
                ret = c_strlist_append(ret, tmatches);
                c_strlist_dispose(tmatches);
            }
        }
        if flags & ((1 as libc::c_int) << 10 as libc::c_int) as libc::c_ulong != 0 {
            tmatches = gen_matches_from_itemlist(&mut it_functions, text);
            if !tmatches.is_null() {
                ret = c_strlist_append(ret, tmatches);
                c_strlist_dispose(tmatches);
            }
        }
        if flags & ((1 as libc::c_int) << 12 as libc::c_int) as libc::c_ulong != 0 {
            tmatches = gen_matches_from_itemlist(&mut it_helptopics, text);
            if !tmatches.is_null() {
                ret = c_strlist_append(ret, tmatches);
                c_strlist_dispose(tmatches);
            }
        }
        if flags & ((1 as libc::c_int) << 13 as libc::c_int) as libc::c_ulong != 0 {
            tmatches = gen_matches_from_itemlist(&mut it_hostnames, text);
            if !tmatches.is_null() {
                ret = c_strlist_append(ret, tmatches);
                c_strlist_dispose(tmatches);
            }
        }
        if flags & ((1 as libc::c_int) << 14 as libc::c_int) as libc::c_ulong != 0 {
            tmatches = gen_matches_from_itemlist(&mut it_jobs, text);
            if !tmatches.is_null() {
                ret = c_strlist_append(ret, tmatches);
                c_strlist_dispose(tmatches);
            }
        }
        if flags & ((1 as libc::c_int) << 15 as libc::c_int) as libc::c_ulong != 0 {
            tmatches = gen_matches_from_itemlist(&mut it_keywords, text);
            if !tmatches.is_null() {
                ret = c_strlist_append(ret, tmatches);
                c_strlist_dispose(tmatches);
            }
        }
        if flags & ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_ulong != 0 {
            tmatches = gen_matches_from_itemlist(&mut it_running, text);
            if !tmatches.is_null() {
                ret = c_strlist_append(ret, tmatches);
                c_strlist_dispose(tmatches);
            }
        }
        if flags & ((1 as libc::c_int) << 18 as libc::c_int) as libc::c_ulong != 0 {
            tmatches = gen_matches_from_itemlist(&mut it_setopts, text);
            if !tmatches.is_null() {
                ret = c_strlist_append(ret, tmatches);
                c_strlist_dispose(tmatches);
            }
        }
        if flags & ((1 as libc::c_int) << 19 as libc::c_int) as libc::c_ulong != 0 {
            tmatches = gen_matches_from_itemlist(&mut it_shopts, text);
            if !tmatches.is_null() {
                ret = c_strlist_append(ret, tmatches);
                c_strlist_dispose(tmatches);
            }
        }
        if flags & ((1 as libc::c_int) << 20 as libc::c_int) as libc::c_ulong != 0 {
            tmatches = gen_matches_from_itemlist(&mut it_signals, text);
            if !tmatches.is_null() {
                ret = c_strlist_append(ret, tmatches);
                c_strlist_dispose(tmatches);
            }
        }
        if flags & ((1 as libc::c_int) << 21 as libc::c_int) as libc::c_ulong != 0 {
            tmatches = gen_matches_from_itemlist(&mut it_stopped, text);
            if !tmatches.is_null() {
                ret = c_strlist_append(ret, tmatches);
                c_strlist_dispose(tmatches);
            }
        }
        if flags & ((1 as libc::c_int) << 23 as libc::c_int) as libc::c_ulong != 0 {
            tmatches = gen_matches_from_itemlist(&mut it_variables, text);
            if !tmatches.is_null() {
                ret = c_strlist_append(ret, tmatches);
                c_strlist_dispose(tmatches);
            }
        }
        if flags & ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong != 0 {
            cmatches = c_rl_completion_matches(text, Some(command_word_completion_function));
            tmatches = completions_to_stringlist(cmatches);
            ret = c_strlist_append(ret, tmatches);
            c_strvec_dispose(cmatches);
            c_strlist_dispose(tmatches);
        }
        if flags & ((1 as libc::c_int) << 9 as libc::c_int) as libc::c_ulong != 0 {
            cmatches = c_rl_completion_matches(
                text,
                ::std::mem::transmute::<
                    Option<fn() -> *mut libc::c_char>,
                    Option<rl_compentry_func_t>,
                >(Some(::std::mem::transmute::<
                    fn(*const libc::c_char, libc::c_int) -> *mut libc::c_char,
                    fn() -> *mut libc::c_char,
                >(pcomp_filename_completion_function))),
            );
            tmatches = completions_to_stringlist(cmatches);
            ret = c_strlist_append(ret, tmatches);
            c_strvec_dispose(cmatches);
            c_strlist_dispose(tmatches);
        }
        if flags & ((1 as libc::c_int) << 22 as libc::c_int) as libc::c_ulong != 0 {
            cmatches = c_rl_completion_matches(
                text,
                Some(std::mem::transmute::<
                    fn(*const libc::c_char, libc::c_int) -> *mut libc::c_char,
                    fn(
                        arg1: *const ::std::os::raw::c_char,
                        arg2: ::std::os::raw::c_int,
                    ) -> *mut ::std::os::raw::c_char,
                >(c_rl_username_completion_function)),
            );
            tmatches = completions_to_stringlist(cmatches);
            ret = c_strlist_append(ret, tmatches);
            c_strvec_dispose(cmatches);
            c_strlist_dispose(tmatches);
        }
        if flags & ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_ulong != 0 {
            cmatches = c_rl_completion_matches(text, Some(bash_groupname_completion_function));
            tmatches = completions_to_stringlist(cmatches);
            ret = c_strlist_append(ret, tmatches);
            c_strvec_dispose(cmatches);
            c_strlist_dispose(tmatches);
        }
        if flags & ((1 as libc::c_int) << 17 as libc::c_int) as libc::c_ulong != 0 {
            cmatches = c_rl_completion_matches(text, Some(bash_servicename_completion_function));
            tmatches = completions_to_stringlist(cmatches);
            ret = c_strlist_append(ret, tmatches);
            c_strvec_dispose(cmatches);
            c_strlist_dispose(tmatches);
        }
        if flags & ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_ulong != 0 {
            t = rl_filename_completion_desired;
            rl_completion_mark_symlink_dirs = 1 as libc::c_int;
            cmatches = bash_directory_completion_matches(text);
            if t == 0 as libc::c_int
                && cmatches.is_null()
                && rl_filename_completion_desired == 1 as libc::c_int
            {
                rl_filename_completion_desired = 0 as libc::c_int;
            }
            tmatches = completions_to_stringlist(cmatches);
            ret = c_strlist_append(ret, tmatches);
            c_strvec_dispose(cmatches);
            c_strlist_dispose(tmatches);
        }
        return ret;
    }
}
#[no_mangle]
pub fn gen_globpat_matches(cs: *mut COMPSPEC, _text: *const libc::c_char) -> *mut STRINGLIST {
    unsafe {
        let sl: *mut STRINGLIST;
        sl = c_strlist_create(0 as libc::c_int);
        let ref mut fresh62 = (*sl).list;
        *fresh62 = shell_glob_filename((*cs).globpat, 0 as libc::c_int);
        if (*sl).list == &mut glob_error_return as *mut *mut libc::c_char {
            let ref mut fresh63 = (*sl).list;
            *fresh63 = 0 as *mut libc::c_void as *mut *mut libc::c_char;
        }
        if !((*sl).list).is_null() {
            let ref mut fresh64 = (*sl).list_size;
            *fresh64 = c_strvec_len((*sl).list);
            (*sl).list_len = *fresh64;
        }
        return sl;
    }
}
#[no_mangle]
fn gen_wordlist_matches(cs: *mut COMPSPEC, text: *const libc::c_char) -> *mut STRINGLIST {
    let mut l: *mut WORD_LIST;
    let l2: *mut WORD_LIST;
    let sl: *mut STRINGLIST;
    let mut nw: libc::c_int;
    let tlen: libc::c_int;
    let ntxt: *mut libc::c_char;
    unsafe {
        if ((*cs).words).is_null()
            || *((*cs).words).offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
        {
            return 0 as *mut libc::c_void as *mut STRINGLIST;
        }
        l = split_at_delims(
            (*cs).words,
            strlen((*cs).words) as libc::c_int,
            0 as *mut libc::c_void as *mut libc::c_char,
            -(1 as libc::c_int),
            0 as libc::c_int,
            0 as *mut libc::c_void as *mut libc::c_int,
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        if l.is_null() {
            return 0 as *mut libc::c_void as *mut STRINGLIST;
        }
        l2 = expand_words_shellexp(l);
        dispose_words(l);
        nw = list_length(l2 as *mut GENERIC_LIST);
        sl = c_strlist_create(nw + 1 as libc::c_int);
        ntxt = bash_dequote_text(text);
        tlen = (if !ntxt.is_null() && *ntxt.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            if *ntxt.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                if *ntxt.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                    strlen(ntxt) as usize
                } else {
                    2 as libc::c_int as usize
                }
            } else {
                1 as libc::c_int as usize
            }
        } else {
            0 as libc::c_int as usize
        }) as libc::c_int;
        nw = 0 as libc::c_int;
        l = l2;
        while !l.is_null() {
            if tlen == 0 as libc::c_int
                || (if tlen == 0 as libc::c_int {
                    1 as libc::c_int
                } else {
                    (*((*(*l).word).word).offset(0 as libc::c_int as isize) as libc::c_int
                        == *ntxt.offset(0 as libc::c_int as isize) as libc::c_int
                        && strncmp(
                            (*(*l).word).word,
                            ntxt,
                            (tlen as libc::c_ulong).try_into().unwrap(),
                        ) == 0 as libc::c_int) as libc::c_int
                }) != 0
            {
                let fresh65 = nw;
                nw = nw + 1;
                let ref mut fresh66 = *((*sl).list).offset(fresh65 as isize);
                *fresh66 = if !((*(*l).word).word).is_null() {
                    strcpy(
                        malloc(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(strlen((*(*l).word).word) as u64)
                                .try_into()
                                .unwrap(),
                        ) as *mut libc::c_char,
                        (*(*l).word).word,
                    )
                } else {
                    0 as *mut libc::c_void as *mut libc::c_char
                };
            }
            l = (*l).next;
        }
        let ref mut fresh67 = (*sl).list_len;
        *fresh67 = nw;
        let ref mut fresh68 = *((*sl).list).offset(*fresh67 as isize);
        *fresh68 = 0 as *mut libc::c_void as *mut libc::c_char;
        dispose_words(l2);
        if !ntxt.is_null() {
            free(ntxt as *mut libc::c_void);
        }
        return sl;
    }
}
#[no_mangle]
fn bind_comp_words(lwords: *mut WORD_LIST) -> *mut SHELL_VAR {
    unsafe {
        let mut v: *mut SHELL_VAR;
        v = find_variable_noref(b"COMP_WORDS\0" as *const u8 as *const libc::c_char);
        if v.is_null() {
            v = make_new_array_variable(
                b"COMP_WORDS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        if (*v).attributes & att_nameref as libc::c_int != 0 {
            (*v).attributes &= !(att_nameref as libc::c_int);
        }
        if (*v).attributes & LIST_INITIALIZED as libc::c_int == 0 as libc::c_int {
            v = convert_var_to_array(v);
        }
        v = assign_array_var_from_word_list(v, lwords, 0 as libc::c_int);
        (*v).attributes &= !(0x1000 as libc::c_int);
        return v;
    }
}
#[no_mangle]
fn bind_compfunc_variables(
    line: *mut libc::c_char,
    ind: libc::c_int,
    _lwords: *mut WORD_LIST,
    cw: libc::c_int,
    exported: libc::c_int,
) {
    let mut ibuf: [libc::c_char; 12] = [0; 12];
    let mut value: *mut libc::c_char;
    let mut v: *mut SHELL_VAR;
    let llen: size_t;
    let c: libc::c_int;
    v = bind_variable(
        b"COMP_LINE\0" as *const u8 as *const libc::c_char,
        line,
        0 as libc::c_int,
    );
    unsafe {
        if !v.is_null() && exported != 0 {
            (*v).attributes |= LIST_DYNAMIC as libc::c_int;
        }
        c = *line.offset(ind as isize) as libc::c_int;
        *line.offset(ind as isize) = '\u{0}' as i32 as libc::c_char;
        llen = if c___ctype_get_mb_cur_max()
            > (1 as libc::c_int as libc::c_ulong).try_into().unwrap()
        {
            if !line.is_null() && *line.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
                if *line.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                    c_mbstrlen(line).try_into().unwrap()
                } else {
                    1 as libc::c_int as libc::c_ulong
                }
            } else {
                0 as libc::c_int as libc::c_ulong
            }
        } else if !line.is_null() && *line.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            if *line.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                if *line.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                    strlen(line) as libc::c_ulong
                } else {
                    2 as libc::c_int as libc::c_ulong
                }
            } else {
                1 as libc::c_int as libc::c_ulong
            }
        } else {
            0 as libc::c_int as libc::c_ulong
        };
        *line.offset(ind as isize) = c as libc::c_char;
        value = c_inttostr(
            llen as intmax_t,
            ibuf.as_mut_ptr(),
            (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                .try_into()
                .unwrap(),
        );
        v = bind_int_variable(
            b"COMP_POINT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value,
            0 as libc::c_int,
        );
        if !v.is_null() && exported != 0 {
            (*v).attributes |= LIST_DYNAMIC as libc::c_int;
        }
        value = c_inttostr(
            rl_completion_type as intmax_t,
            ibuf.as_mut_ptr(),
            (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                .try_into()
                .unwrap(),
        );
        v = bind_int_variable(
            b"COMP_TYPE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value,
            0 as libc::c_int,
        );
        if !v.is_null() && exported != 0 {
            (*v).attributes |= LIST_DYNAMIC as libc::c_int;
        }
        value = c_inttostr(
            rl_completion_invoking_key as intmax_t,
            ibuf.as_mut_ptr(),
            (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                .try_into()
                .unwrap(),
        );
        v = bind_int_variable(
            b"COMP_KEY\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value,
            0 as libc::c_int,
        );
        if !v.is_null() && exported != 0 {
            (*v).attributes |= LIST_DYNAMIC as libc::c_int;
        }
        if exported == 0 as libc::c_int {
            // v = bind_comp_words(lwords);
            value = c_inttostr(
                cw as intmax_t,
                ibuf.as_mut_ptr(),
                (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                    .try_into()
                    .unwrap(),
            );
            bind_int_variable(
                b"COMP_CWORD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value,
                0 as libc::c_int,
            );
        } else {
            array_needs_making = 1 as libc::c_int;
        };
    }
}
#[no_mangle]
fn unbind_compfunc_variables(exported: libc::c_int) {
    unbind_variable_noref(b"COMP_LINE\0" as *const u8 as *const libc::c_char);
    unbind_variable_noref(b"COMP_POINT\0" as *const u8 as *const libc::c_char);
    unbind_variable_noref(b"COMP_TYPE\0" as *const u8 as *const libc::c_char);
    unbind_variable_noref(b"COMP_KEY\0" as *const u8 as *const libc::c_char);
    unbind_variable_noref(b"COMP_WORDS\0" as *const u8 as *const libc::c_char);
    unbind_variable_noref(b"COMP_CWORD\0" as *const u8 as *const libc::c_char);
    unsafe {
        if exported != 0 {
            array_needs_making = 1 as libc::c_int;
        }
    }
}
#[no_mangle]
fn build_arg_list(
    cmd: *mut libc::c_char,
    cname: *const libc::c_char,
    text: *const libc::c_char,
    lwords: *mut WORD_LIST,
    ind: libc::c_int,
) -> *mut WORD_LIST {
    let ret: *mut WORD_LIST;
    let mut cl: *mut WORD_LIST;
    let mut l: *mut WORD_LIST;
    let mut w: *mut WORD_DESC;
    let mut i: libc::c_int;
    unsafe {
        // ret = 0 as *mut libc::c_void as *mut WORD_LIST;
        w = make_word(cmd);
        ret = make_word_list(w, 0 as *mut libc::c_void as *mut WORD_LIST);
        w = make_word(cname);
        let ref mut fresh69 = (*ret).next;
        *fresh69 = make_word_list(w, 0 as *mut libc::c_void as *mut WORD_LIST);
        cl = *fresh69;
        w = make_word(text);
        let ref mut fresh70 = (*cl).next;
        *fresh70 = make_word_list(w, 0 as *mut libc::c_void as *mut WORD_LIST);
        cl = (*cl).next;
        l = lwords;
        i = 1 as libc::c_int;
        while !l.is_null() && i < ind - 1 as libc::c_int {
            l = (*l).next;
            i += 1;
        }
        w = if !l.is_null() && !((*l).word).is_null() {
            copy_word((*l).word)
        } else {
            make_word(b"\0" as *const u8 as *const libc::c_char)
        };
        let ref mut fresh71 = (*cl).next;
        *fresh71 = make_word_list(w, 0 as *mut libc::c_void as *mut WORD_LIST);
        return ret;
    }
}
#[no_mangle]
fn gen_shell_function_matches(
    cs: *mut COMPSPEC,
    cmd: *const libc::c_char,
    text: *const libc::c_char,
    line: *mut libc::c_char,
    ind: libc::c_int,
    lwords: *mut WORD_LIST,
    _nw: libc::c_int,
    cw: libc::c_int,
    foundp: *mut libc::c_int,
) -> *mut STRINGLIST {
    let funcname: *mut libc::c_char;
    let sl: *mut STRINGLIST;
    let f: *mut SHELL_VAR;
    let mut v: *mut SHELL_VAR;
    let cmdlist: *mut WORD_LIST;
    let fval: libc::c_int;
    let mut found: libc::c_int;
    let mut ps: sh_parser_state_t = sh_parser_state_t {
        parser_state: 0,
        token_state: 0 as *mut libc::c_int,
        token: 0 as *mut libc::c_char,
        token_buffer_size: 0,
        input_line_terminator: 0,
        eof_encountered: 0,
        prompt_string_pointer: 0 as *mut *mut libc::c_char,
        current_command_line_count: 0,
        remember_on_history: 0,
        history_expansion_inhibited: 0,
        last_command_exit_value: 0,
        pipestatus: 0 as *mut ARRAY,
        last_shell_builtin: None,
        this_shell_builtin: None,
        expand_aliases: 0,
        echo_input_at_read: 0,
        need_here_doc: 0,
        here_doc_first_line: 0,
        redir_stack: [0 as *mut REDIRECT; 16],
    };
    let pps: *mut sh_parser_state_t;
    let a: *mut ARRAY;
    unsafe {
        found = 0 as libc::c_int;
        if !foundp.is_null() {
            *foundp = found;
        }
        funcname = (*cs).funcname;
        f = find_function(funcname);
        if f.is_null() {
            internal_error(
                c_dcgettext(
                    0 as *const libc::c_char,
                    b"completion: function `%s' not found\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                funcname,
            );
            c_rl_ding();
            c_rl_on_new_line();
            return 0 as *mut libc::c_void as *mut STRINGLIST;
        }
        bind_compfunc_variables(line, ind, lwords, cw - 1 as libc::c_int, 0 as libc::c_int);
        cmdlist = build_arg_list(funcname, cmd, text, lwords, cw);
        pps = &mut ps;
        save_parser_state(pps);
        begin_unwind_frame(
            b"gen-shell-function-matches\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        add_unwind_protect(
            std::mem::transmute::<fn(_), Option<Function>>(restore_parser_state),
            pps as *mut libc::c_char,
        );
        add_unwind_protect(
            std::mem::transmute::<fn(_), Option<Function>>(dispose_words),
            cmdlist as *mut libc::c_char,
        );
        add_unwind_protect(
            std::mem::transmute::<fn(_), Option<Function>>(unbind_compfunc_variables),
            0 as *mut libc::c_char,
        );
        fval = execute_shell_function(f, cmdlist);
        discard_unwind_frame(
            b"gen-shell-function-matches\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        restore_parser_state(pps);
        found = (fval != EX_NOTFOUND as libc::c_int) as libc::c_int;
        if fval == EX_RETRYFAIL as libc::c_int {
            found |= ((1 as libc::c_int) << 8 as libc::c_int) << 1 as libc::c_int;
        }
        if !foundp.is_null() {
            *foundp = found;
        }
        dispose_words(cmdlist);
        unbind_compfunc_variables(0 as libc::c_int);
        v = find_variable(b"COMPREPLY\0" as *const u8 as *const libc::c_char);
        if v.is_null() {
            return 0 as *mut libc::c_void as *mut STRINGLIST;
        }
        if (*v).attributes & LIST_INITIALIZED as libc::c_int == 0 as libc::c_int
            && (*v).attributes & att_assoc as libc::c_int == 0 as libc::c_int
        {
            v = convert_var_to_array(v);
        }
        (*v).attributes &= !(0x1000 as libc::c_int);
        a = (*v).value as *mut ARRAY;
        if found == 0 as libc::c_int
            || found & ((1 as libc::c_int) << 8 as libc::c_int) << 1 as libc::c_int != 0
            || a.is_null()
            || (*v).attributes & LIST_INITIALIZED as libc::c_int == 0 as libc::c_int
            || (*a).num_elements == 0 as libc::c_int
        {
            sl = 0 as *mut libc::c_void as *mut STRINGLIST;
        } else {
            sl = c_strlist_create(0 as libc::c_int);
            let ref mut fresh72 = (*sl).list;
            *fresh72 = array_to_argv(a, 0 as *mut libc::c_int);
            let ref mut fresh73 = (*sl).list_size;
            *fresh73 = (*a).num_elements;
            (*sl).list_len = *fresh73;
        }
        unbind_variable_noref(b"COMPREPLY\0" as *const u8 as *const libc::c_char);
        return sl;
    }
}
#[no_mangle]
fn gen_command_matches(
    cs: *mut COMPSPEC,
    cmd: *const libc::c_char,
    text: *const libc::c_char,
    line: *mut libc::c_char,
    ind: libc::c_int,
    lwords: *mut WORD_LIST,
    _nw: libc::c_int,
    cw: libc::c_int,
) -> *mut STRINGLIST {
    let csbuf: *mut libc::c_char;
    let mut cscmd: *mut libc::c_char;
    let mut t: *mut libc::c_char;
    let mut cmdlen: libc::c_int;
    let mut cmdsize: libc::c_int;
    let mut n: libc::c_int;
    let mut ws: libc::c_int;
    let mut we: libc::c_int;
    let cmdlist: *mut WORD_LIST;
    let mut cl: *mut WORD_LIST;
    let tw: *mut WORD_DESC;
    let sl: *mut STRINGLIST;
    unsafe {
        bind_compfunc_variables(line, ind, lwords, cw, 1 as libc::c_int);
        cmdlist = build_arg_list((*cs).command, cmd, text, lwords, cw);
        n = strlen((*cs).command) as libc::c_int;
        cmdsize = n + 1 as libc::c_int;
        cl = (*cmdlist).next;
        while !cl.is_null() {
            cmdsize = (cmdsize as libc::c_ulong).wrapping_add(
                (if !((*(*cl).word).word).is_null()
                    && *((*(*cl).word).word).offset(0 as libc::c_int as isize) as libc::c_int != 0
                {
                    if *((*(*cl).word).word).offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                        if *((*(*cl).word).word).offset(2 as libc::c_int as isize) as libc::c_int
                            != 0
                        {
                            strlen((*(*cl).word).word) as libc::c_ulong
                        } else {
                            2 as libc::c_int as libc::c_ulong
                        }
                    } else {
                        1 as libc::c_int as libc::c_ulong
                    }
                } else {
                    0 as libc::c_int as libc::c_ulong
                })
                .wrapping_add(3 as libc::c_int as libc::c_ulong),
            ) as libc::c_int;
            cl = (*cl).next;
        }
        cmdsize += 2 as libc::c_int;
        cscmd = malloc(((cmdsize + 1 as libc::c_int) as size_t).try_into().unwrap())
            as *mut libc::c_char;
        strcpy(cscmd, (*cs).command);
        cmdlen = n;
        let fresh74 = cmdlen;
        cmdlen = cmdlen + 1;
        *cscmd.offset(fresh74 as isize) = ' ' as i32 as libc::c_char;
        cl = (*cmdlist).next;
        while !cl.is_null() {
            t = c_sh_single_quote(if !((*(*cl).word).word).is_null() {
                (*(*cl).word).word
            } else {
                b"\0" as *const u8 as *const libc::c_char
            });
            n = strlen(t) as libc::c_int;
            if cmdlen + (n + 2 as libc::c_int) >= cmdsize {
                while cmdlen + (n + 2 as libc::c_int) >= cmdsize {
                    cmdsize += 64 as libc::c_int;
                }
                cscmd = realloc(
                    cscmd as *mut libc::c_void,
                    (cmdsize as size_t).try_into().unwrap(),
                ) as *mut libc::c_char;
            }
            strcpy(cscmd.offset(cmdlen as isize), t);
            cmdlen += n;
            if !((*cl).next).is_null() {
                let fresh75 = cmdlen;
                cmdlen = cmdlen + 1;
                *cscmd.offset(fresh75 as isize) = ' ' as i32 as libc::c_char;
            }
            free(t as *mut libc::c_void);
            cl = (*cl).next;
        }
        *cscmd.offset(cmdlen as isize) = '\u{0}' as i32 as libc::c_char;
        tw = command_substitute(cscmd, 0 as libc::c_int, 0 as libc::c_int);
        csbuf = if !tw.is_null() {
            (*tw).word
        } else {
            0 as *mut libc::c_void as *mut libc::c_char
        };
        if !tw.is_null() {
            dispose_word_desc(tw);
        }
        dispose_words(cmdlist);
        free(cscmd as *mut libc::c_void);
        unbind_compfunc_variables(1 as libc::c_int);
        if csbuf.is_null() || *csbuf as libc::c_int == '\u{0}' as i32 {
            if !csbuf.is_null() {
                free(csbuf as *mut libc::c_void);
            }
            return 0 as *mut libc::c_void as *mut STRINGLIST;
        }
        sl = c_strlist_create(16 as libc::c_int);
        ws = 0 as libc::c_int;
        while *csbuf.offset(ws as isize) != 0 {
            we = ws;
            while *csbuf.offset(we as isize) as libc::c_int != 0
                && *csbuf.offset(we as isize) as libc::c_int != '\n' as i32
            {
                if *csbuf.offset(we as isize) as libc::c_int == '\\' as i32
                    && *csbuf.offset((we + 1 as libc::c_int) as isize) as libc::c_int == '\n' as i32
                {
                    we += 1;
                }
                we += 1;
            }
            t = substring(csbuf, ws, we);
            if (*sl).list_len >= (*sl).list_size - 1 as libc::c_int {
                c_strlist_resize(sl, (*sl).list_size + 16 as libc::c_int);
            }
            let ref mut fresh76 = (*sl).list_len;
            let fresh77 = *fresh76;
            *fresh76 = *fresh76 + 1;
            let ref mut fresh78 = *((*sl).list).offset(fresh77 as isize);
            *fresh78 = t;
            while *csbuf.offset(we as isize) as libc::c_int == '\n' as i32 {
                we += 1;
            }
            ws = we;
        }
        let ref mut fresh79 = *((*sl).list).offset((*sl).list_len as isize);
        *fresh79 = 0 as *mut libc::c_void as *mut libc::c_char;
        free(csbuf as *mut libc::c_void);
        return sl;
    }
}
#[no_mangle]
fn command_line_to_word_list(
    line: *mut libc::c_char,
    llen: libc::c_int,
    sentinel: libc::c_int,
    nwp: *mut libc::c_int,
    cwp: *mut libc::c_int,
) -> *mut WORD_LIST {
    let ret: *mut WORD_LIST;
    let delims: *mut libc::c_char;
    unsafe {
        delims = rl_completer_word_break_characters;
    }
    ret = split_at_delims(
        line,
        llen,
        delims,
        sentinel,
        LIST_INITIALIZED as libc::c_int | 0x100 as libc::c_int,
        nwp,
        cwp,
    );
    return ret;
}
#[no_mangle]
pub fn gen_compspec_completions(
    cs: *mut COMPSPEC,
    cmd: *const libc::c_char,
    word: *const libc::c_char,
    start: libc::c_int,
    end: libc::c_int,
    foundp: *mut libc::c_int,
) -> *mut STRINGLIST {
    let mut ret: *mut STRINGLIST;
    let mut tmatches: *mut STRINGLIST;
    let mut line: *mut libc::c_char;
    let llen: libc::c_int;
    let mut nw: libc::c_int = 0;
    let mut cw: libc::c_int = 0;
    let mut found: libc::c_int;
    let mut foundf: libc::c_int;
    let mut lwords: *mut WORD_LIST;
    let lw: *mut WORD_DESC;
    let tcs: *mut COMPSPEC;
    unsafe {
        found = 1 as libc::c_int;
        ret = gen_action_completions(cs, word);
        if !((*cs).globpat).is_null() {
            tmatches = gen_globpat_matches(cs, word);
            if !tmatches.is_null() {
                ret = c_strlist_append(ret, tmatches);
                c_strlist_dispose(tmatches);
                rl_filename_completion_desired = 1 as libc::c_int;
            }
        }
        if !((*cs).words).is_null() {
            tmatches = gen_wordlist_matches(cs, word);
            if !tmatches.is_null() {
                ret = c_strlist_append(ret, tmatches);
                c_strlist_dispose(tmatches);
            }
        }
        lwords = 0 as *mut libc::c_void as *mut WORD_LIST;
        line = 0 as *mut libc::c_void as *mut libc::c_char;
        if !((*cs).command).is_null() || !((*cs).funcname).is_null() {
            line = substring(pcomp_line, start, end);
            llen = end - start;
            lwords = command_line_to_word_list(line, llen, pcomp_ind - start, &mut nw, &mut cw);
            if !lwords.is_null()
                && !((*lwords).word).is_null()
                && *cmd.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
                && *((*(*lwords).word).word).offset(0 as libc::c_int as isize) as libc::c_int
                    != 0 as libc::c_int
            {
                lw = make_bare_word(cmd);
                lwords = make_word_list(lw, lwords);
                nw += 1;
                cw += 1;
            }
        }
        if !((*cs).funcname).is_null() {
            foundf = 0 as libc::c_int;
            tmatches = gen_shell_function_matches(
                cs,
                cmd,
                word,
                line,
                pcomp_ind - start,
                lwords,
                nw,
                cw,
                &mut foundf,
            );
            if foundf != 0 as libc::c_int {
                found = foundf;
            }
            if !tmatches.is_null() {
                ret = c_strlist_append(ret, tmatches);
                c_strlist_dispose(tmatches);
            }
        }
        if !((*cs).command).is_null() {
            tmatches = gen_command_matches(cs, cmd, word, line, pcomp_ind - start, lwords, nw, cw);
            if !tmatches.is_null() {
                ret = c_strlist_append(ret, tmatches);
                c_strlist_dispose(tmatches);
            }
        }
        if !((*cs).command).is_null() || !((*cs).funcname).is_null() {
            if !lwords.is_null() {
                dispose_words(lwords);
            }
            if !line.is_null() {
                free(line as *mut libc::c_void);
            }
        }
        if !foundp.is_null() {
            *foundp = found;
        }
        if found == 0 as libc::c_int
            || found & ((1 as libc::c_int) << 8 as libc::c_int) << 1 as libc::c_int != 0
        {
            c_strlist_dispose(ret);
            return 0 as *mut STRINGLIST;
        }
        if !((*cs).filterpat).is_null() {
            tmatches = filter_stringlist(ret, (*cs).filterpat, word);
            if !ret.is_null() && ret != tmatches {
                if !((*ret).list).is_null() {
                    free((*ret).list as *mut libc::c_void);
                }
                free(ret as *mut libc::c_void);
            }
            ret = tmatches;
        }
        if !((*cs).prefix).is_null() || !((*cs).suffix).is_null() {
            ret = c_strlist_prefix_suffix(ret, (*cs).prefix, (*cs).suffix);
        }
        if (ret.is_null() || (*ret).list_len == 0 as libc::c_int)
            && (*cs).options & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong != 0
        {
            tcs = compspec_create();
            (*tcs).actions = ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_ulong;
            if !ret.is_null() {
                free(ret as *mut libc::c_void);
            }
            ret = gen_action_completions(tcs, word);
            compspec_dispose(tcs);
        } else if (*cs).options & ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_ulong != 0 {
            tcs = compspec_create();
            (*tcs).actions = ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_ulong;
            tmatches = gen_action_completions(tcs, word);
            ret = c_strlist_append(ret, tmatches);
            c_strlist_dispose(tmatches);
            compspec_dispose(tcs);
        }
        return ret;
    }
}
#[no_mangle]
pub fn pcomp_set_readline_variables(flags: libc::c_int, nval: libc::c_int) {
    unsafe {
        if flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            rl_filename_completion_desired = nval;
        }
        if flags & (1 as libc::c_int) << 5 as libc::c_int != 0 {
            rl_completion_suppress_append = nval;
        }
        if flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
            rl_filename_quoting_desired = 1 as libc::c_int - nval;
        }
        if flags & (1 as libc::c_int) << 8 as libc::c_int != 0 {
            rl_sort_completion_matches = 1 as libc::c_int - nval;
        }
    }
}
#[no_mangle]
pub fn pcomp_set_compspec_options(
    mut cs: *mut COMPSPEC,
    flags: libc::c_int,
    set_or_unset: libc::c_int,
) {
    unsafe {
        if cs.is_null() && {
            cs = pcomp_curcs;
            cs.is_null()
        } {
            return;
        }
        if set_or_unset != 0 {
            (*cs).options |= flags as libc::c_ulong;
        } else {
            (*cs).options &= !flags as libc::c_ulong;
        };
    }
}
#[no_mangle]
fn gen_progcomp_completions(
    ocmd: *const libc::c_char,
    cmd: *const libc::c_char,
    word: *const libc::c_char,
    start: libc::c_int,
    end: libc::c_int,
    foundp: *mut libc::c_int,
    retryp: *mut libc::c_int,
    lastcs: *mut *mut COMPSPEC,
) -> *mut STRINGLIST {
    let mut cs: *mut COMPSPEC;
    let oldcs: *mut COMPSPEC;
    let oldcmd: *const libc::c_char;
    let oldtxt: *const libc::c_char;
    let ret: *mut STRINGLIST;
    unsafe {
        cs = progcomp_search(ocmd);
        if cs.is_null() || cs == *lastcs {
            return 0 as *mut STRINGLIST;
        }
        if !(*lastcs).is_null() {
            compspec_dispose(*lastcs);
        }
        let ref mut fresh80 = (*cs).refcount;
        *fresh80 += 1;
        *lastcs = cs;
        cs = compspec_copy(cs);
        oldcs = pcomp_curcs;
        oldcmd = pcomp_curcmd;
        oldtxt = pcomp_curtxt;
        pcomp_curcs = cs;
        pcomp_curcmd = cmd;
        pcomp_curtxt = word;
        ret = gen_compspec_completions(cs, cmd, word, start, end, foundp);
        pcomp_curcs = oldcs;
        pcomp_curcmd = oldcmd;
        pcomp_curtxt = oldtxt;
        if !retryp.is_null() {
            *retryp = (!foundp.is_null()
                && *foundp & ((1 as libc::c_int) << 8 as libc::c_int) << 1 as libc::c_int != 0)
                as libc::c_int;
        }
        if !foundp.is_null() {
            *foundp &= !(((1 as libc::c_int) << 8 as libc::c_int) << 1 as libc::c_int);
            *foundp = (*foundp as libc::c_ulong | (*cs).options) as libc::c_int;
        }
        compspec_dispose(cs);
        return ret;
    }
}
#[no_mangle]
pub fn programmable_completions(
    cmd: *const libc::c_char,
    word: *const libc::c_char,
    start: libc::c_int,
    end: libc::c_int,
    foundp: *mut libc::c_int,
) -> *mut *mut libc::c_char {
    let mut lastcs: *mut COMPSPEC;
    let mut ret: *mut STRINGLIST;
    let rmatches: *mut *mut libc::c_char;
    let mut t: *mut libc::c_char;
    let mut found: libc::c_int;
    let mut retry: libc::c_int;
    let mut count: libc::c_int;
    let mut ocmd: *mut libc::c_char;
    let mut oend: libc::c_int;
    let mut al: *mut alias_t = 0 as *mut alias_t;
    unsafe {
        lastcs = 0 as *mut COMPSPEC;
        count = 0 as libc::c_int;
        found = count;
        pcomp_line = rl_line_buffer;
        pcomp_ind = rl_point;
        ocmd = cmd as *mut libc::c_char;
        oend = end;
        loop {
            retry = 0 as libc::c_int;
            ret = gen_progcomp_completions(
                ocmd,
                ocmd,
                word,
                start,
                oend,
                &mut found,
                &mut retry,
                &mut lastcs,
            );
            if found == 0 as libc::c_int {
                t = strrchr(ocmd, '/' as i32);
                if !t.is_null() && {
                    t = t.offset(1);
                    *t as libc::c_int != 0
                } {
                    ret = gen_progcomp_completions(
                        t,
                        ocmd,
                        word,
                        start,
                        oend,
                        &mut found,
                        &mut retry,
                        &mut lastcs,
                    );
                }
            }
            if found == 0 as libc::c_int {
                ret = gen_progcomp_completions(
                    b"_DefaultCmD_\0" as *const u8 as *const libc::c_char,
                    ocmd,
                    word,
                    start,
                    oend,
                    &mut found,
                    &mut retry,
                    &mut lastcs,
                );
            }
            if found == 0 as libc::c_int && retry == 0 as libc::c_int && progcomp_alias != 0 && {
                al = find_alias(ocmd);
                !al.is_null()
            } {
                let ncmd: *mut libc::c_char;
                let nline: *mut libc::c_char;
                let ntxt: *mut libc::c_char;
                let ind: libc::c_int;
                let lendiff: libc::c_int;
                let nlen: size_t;
                let olen: size_t;
                let llen: size_t;
                ntxt = (*al).value;
                nlen = strlen(ntxt) as size_t;
                if nlen == 0 as libc::c_int as libc::c_ulong {
                    break;
                }
                olen = strlen(ocmd) as size_t;
                lendiff = nlen.wrapping_sub(olen) as libc::c_int;
                llen = strlen(pcomp_line) as size_t;
                nline = malloc(
                    llen.wrapping_add(lendiff as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .try_into()
                        .unwrap(),
                ) as *mut libc::c_char;
                if start > 0 as libc::c_int {
                    strncpy(
                        nline,
                        pcomp_line,
                        (start as libc::c_ulong).try_into().unwrap(),
                    );
                }
                strncpy(nline.offset(start as isize), ntxt, nlen.try_into().unwrap());
                strcpy(
                    nline.offset(start as isize).offset(nlen as isize),
                    pcomp_line.offset(start as isize).offset(olen as isize),
                );
                ind = skip_to_delim(
                    ntxt,
                    0 as libc::c_int,
                    b"()<>;&| \t\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    LIST_DYNAMIC as libc::c_int | 0x100 as libc::c_int,
                );
                if ind > 0 as libc::c_int {
                    ncmd = substring(ntxt, 0 as libc::c_int, ind);
                    pcomp_ind += lendiff;
                    oend += lendiff;
                    if ocmd != cmd as *mut libc::c_char {
                        free(ocmd as *mut libc::c_void);
                    }
                    if pcomp_line != rl_line_buffer {
                        free(pcomp_line as *mut libc::c_void);
                    }
                    ocmd = ncmd;
                    pcomp_line = nline;
                    retry = 1 as libc::c_int;
                } else {
                    free(nline as *mut libc::c_void);
                    break;
                }
            }
            count += 1;
            if count > 32 as libc::c_int {
                internal_warning(
                    c_dcgettext(
                        0 as *const libc::c_char,
                        b"programmable_completion: %s: possible retry loop\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    cmd,
                );
                break;
            } else if !(retry != 0) {
                break;
            }
        }
        if pcomp_line != rl_line_buffer {
            free(pcomp_line as *mut libc::c_void);
        }
        if ocmd != cmd as *mut libc::c_char {
            free(ocmd as *mut libc::c_void);
        }
        if !ret.is_null() {
            rmatches = (*ret).list;
            free(ret as *mut libc::c_void);
        } else {
            rmatches = 0 as *mut libc::c_void as *mut *mut libc::c_char;
        }
        if !foundp.is_null() {
            *foundp = found;
        }
        if !lastcs.is_null() {
            compspec_dispose(lastcs);
        }
        pcomp_line = rl_line_buffer;
        pcomp_ind = rl_point;
        return rmatches;
    }
}
