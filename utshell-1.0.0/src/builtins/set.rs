//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later
use super::help::builtin_help;
use crate::arrayfunc::{array_variable_part, unbind_array_element, valid_array_reference};
use crate::bashhist::{bash_history_disable, bash_history_enable, load_history};
use crate::builtins::bashgetopt::{internal_getopt, reset_internal_getopt};
use crate::builtins::common::{
    builtin_usage, remember_args, sh_chkwrite, sh_invalidid, sh_invalidopt, sh_invalidoptname,
};
use crate::flags::{change_flag, find_flag};
use crate::general::{extract_colon_unit, get_posix_options, legal_identifier, num_posix_options};
use crate::src_common::*;
use crate::variables::{
    all_shell_functions, all_shell_variables, bind_variable, find_function, find_variable,
    find_variable_last_nameref, print_func_list, print_var_list, stupidly_hack_special_variables,
    sv_ignoreeof, sv_strict_posix, unbind_func, unbind_nameref, unbind_variable,
    unbind_variable_noref,
};
use crate::y_tab::{ignoreeof, with_input_from_stdin, with_input_from_stream};
use std::io::stdin;

pub type setopt_get_func_t = fn(*mut libc::c_char) -> libc::c_int;
pub type setopt_set_func_t = fn(libc::c_int, *mut libc::c_char) -> libc::c_int;

#[no_mangle]
pub static mut o_options: [opp; 28] = unsafe {
    [
        {
            opp {
                name: b"allexport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                letter: b'a' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void as *mut i32,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_set_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
                get_func: std::mem::transmute::<*mut libc::c_void, Option<setopt_get_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            }
        },
        {
            opp {
                name: b"braceexpand\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                letter: b'B' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void as *mut i32,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_set_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
                get_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_get_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            }
        },
        {
            opp {
                name: b"emacs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                letter: b'\0' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void as *mut i32,
                set_func: Some(set_edit_mode),
                get_func: Some(get_edit_mode),
            }
        },
        {
            opp {
                name: b"errexit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                letter: b'e' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void as *mut i32,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_set_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
                get_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_get_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            }
        },
        {
            opp {
                name: b"errtrace\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                letter: b'E' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void as *mut i32,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_set_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
                get_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_get_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            }
        },
        {
            opp {
                name: b"functrace\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                letter: b'T' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void as *mut i32,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_set_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
                get_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_get_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            }
        },
        {
            opp {
                name: b"hashall\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                letter: b'h' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void as *mut i32,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_set_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
                get_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_get_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            }
        },
        {
            opp {
                name: b"histexpand\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                letter: b'H' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void as *mut i32,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_set_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
                get_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_get_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            }
        },
        {
            opp {
                name: b"history\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                letter: b'\0' as i32,
                // variable : 0 as *const libc::c_void
                // as *mut libc::c_void
                // as *mut i32,
                variable: &enable_history_list as *const i32 as *mut i32,
                set_func: Some(bash_set_history),
                get_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_get_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            }
        },
        {
            opp {
                name: b"ignoreeof\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                letter: b'\0' as i32,
                /*variable : 0 as *const libc::c_void
                as *mut libc::c_void
                as *mut i32, */
                variable: &ignoreeof as *const i32 as *mut i32,
                set_func: Some(set_ignoreeof),
                get_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_get_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            }
        },
        {
            opp {
                name: b"interactive-comments\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                letter: b'\0' as i32,
                /*variable : 0 as *const libc::c_void
                as *mut libc::c_void
                as *mut i32, */
                variable: &interactive_comments as *const i32 as *mut i32,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_set_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
                get_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_get_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            }
        },
        {
            opp {
                name: b"keyword\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                letter: b'k' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void as *mut i32,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_set_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
                get_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_get_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            }
        },
        {
            opp {
                name: b"monitor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                letter: b'm' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void as *mut i32,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_set_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
                get_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_get_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            }
        },
        {
            opp {
                name: b"noclobber\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                letter: b'C' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void as *mut i32,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_set_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
                get_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_get_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            }
        },
        {
            opp {
                name: b"noexec\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                letter: b'n' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void as *mut i32,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_set_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
                get_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_get_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            }
        },
        {
            opp {
                name: b"noglob\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                letter: b'f' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void as *mut i32,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_set_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
                get_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_get_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            }
        },
        {
            opp {
                name: b"nolog\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                letter: b'\0' as i32,
                /*variable : 0 as *const libc::c_void
                as *mut libc::c_void
                as *mut i32, */
                variable: &dont_save_function_defs as *const i32 as *mut i32,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_set_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
                get_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_get_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            }
        },
        {
            opp {
                name: b"notify\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                letter: b'b' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void as *mut i32,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_set_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
                get_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_get_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            }
        },
        {
            opp {
                name: b"nounset\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                letter: b'u' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void as *mut i32,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_set_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
                get_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_get_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            }
        },
        {
            opp {
                name: b"onecmd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                letter: b't' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void as *mut i32,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_set_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
                get_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_get_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            }
        },
        {
            opp {
                name: b"physical\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                letter: b'P' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void as *mut i32,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_set_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
                get_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_get_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            }
        },
        {
            opp {
                name: b"pipefail\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                letter: b'\0' as i32,
                /*variable : 0 as *const libc::c_void
                as *mut libc::c_void
                as *mut i32, */
                variable: &pipefail_opt as *const i32 as *mut i32,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_set_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
                get_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_get_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            }
        },
        {
            opp {
                name: b"posix\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                letter: b'\0' as i32,
                /*variable : 0 as *const libc::c_void
                as *mut libc::c_void
                as *mut i32, */
                variable: &posixly_correct as *const libc::c_int as *mut libc::c_int,
                set_func: Some(set_posix_mode),
                get_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_get_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            }
        },
        {
            opp {
                name: b"privileged\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                letter: b'p' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void as *mut i32,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_set_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
                get_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_get_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            }
        },
        {
            opp {
                name: b"verbose\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                letter: b'v' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void as *mut i32,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_set_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
                get_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_get_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            }
        },
        {
            opp {
                name: b"vi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                letter: b'\0' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void as *mut i32,
                set_func: Some(set_edit_mode), //set_edit_mode as *mut setopt_set_func_t ,// unsafe {&mut set_edit_mode},
                get_func: Some(get_edit_mode),
            }
        },
        {
            opp {
                name: b"xtrace\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                letter: b'x' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void as *mut i32,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_set_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
                get_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_get_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            }
        },
        {
            opp {
                name: std::ptr::null_mut(),
                letter: 0 as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void as *mut i32,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_set_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
                get_func: ::std::mem::transmute::<*mut libc::c_void, Option<setopt_get_func_t>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            }
        },
    ]
};

extern "C" {
    fn rl_variable_bind(_: *const libc::c_char, _: *const libc::c_char) -> i32;
    static mut rl_editing_mode: i32;
}

static mut on: *const libc::c_char = b"on\0" as *const u8 as *const libc::c_char;
static mut off: *const libc::c_char = b"off\0" as *const u8 as *const libc::c_char;
static mut previous_option_value: i32 = 0;
// pub type SHELL_VAR = variable;
// pub type arrayind_t = i64;

fn STREQ(a: *const libc::c_char, b: *const libc::c_char) -> bool {
    return unsafe { (*a == *b) && (libc::strcmp(a, b) == 0) };
}

fn find_minus_o_option(name: *mut libc::c_char) -> i32 {
    let mut i: i32 = 0;
    unsafe {
        for j in 0..N_O_OPTIONS!() - 1 {
            i = j as i32;
            let _ooo = o_options[j];

            if STREQ(name, o_options[j as usize].name) {
                return i;
            }
        }
        -1
    }
}

#[no_mangle]
pub fn minus_o_option_value(name: *mut libc::c_char) -> i32 {
    let mut i: i32 = 0;
    let on_or_off: *mut i32 = 0 as *mut i32;

    i = find_minus_o_option(name);
    if i < 0 {
        return -1;
    }
    let options = unsafe { o_options[i as usize] };
    if options.letter != 0 {
        if on_or_off == FLAG_UNKNOWN!() {
            return -1;
        }
        return unsafe { *on_or_off };
    } else {
        unsafe { GET_BINARY_O_OPTION_VALUE!(i, name) }
    }
}

fn print_minus_o_option(name: *mut libc::c_char, value: i32, pflag: i32) {
    unsafe {
        if pflag == 0 {
            if value > 0 {
                println!("{:?} {:?}", CStr::from_ptr(name), CStr::from_ptr(on));
            } else {
                println!("{:?} {:?}", CStr::from_ptr(name), CStr::from_ptr(off));
            }
        } else {
            if value > 0 {
                println!("set -o  {:?}", CStr::from_ptr(name));
            } else {
                println!("set +o {:?}", CStr::from_ptr(name));
            }
        }
    }
}

#[no_mangle]
pub fn list_minus_o_opts(mode: i32, reusable: i32) {
    let mut i: i32 = 0;
    let mut on_or_off: *mut i32 = 0 as *mut i32;
    let mut value: i32 = 0;
    unsafe {
        for j in 0..N_O_OPTIONS!() - 1 {
            i = j as i32;
            if o_options[j as usize].letter != 0 {
                value = 0;
                on_or_off = find_flag(o_options[i as usize].letter);
                if on_or_off == FLAG_UNKNOWN!() {
                    on_or_off = &mut value;
                }
                if mode == -1 || mode == *on_or_off {
                    print_minus_o_option(o_options[i as usize].name, *on_or_off, reusable);
                }
            } else {
                value = GET_BINARY_O_OPTION_VALUE!(i, o_options[i as usize].name);
                if mode == -1 || mode == value {
                    print_minus_o_option(o_options[i as usize].name, value, reusable);
                }
            }
        }
    }
}

pub fn get_minus_o_opts() -> *mut *mut libc::c_char {
    let mut ret = 0 as *mut *mut libc::c_char;
    let mut i: i32 = 0;
    ret = unsafe { strvec_create(N_O_OPTIONS!() as i32 + 1) };
    for j in 0..N_O_OPTIONS!() {
        i = j as i32;
        unsafe {
            if o_options[i as usize].name != std::ptr::null_mut() {
                *ret.offset(i as isize) = o_options[i as usize].name;
                //*ret.as_ptr().offset(i as isize) = o_options[i as usize].name ;
            }
        }
    }
    unsafe { *ret.offset(i as isize) = o_options[i as usize].name };
    // *ret.as_ptr().offset(27 as usize) = std::ptr::null_mut();
    ret
}

pub fn get_current_options() -> *mut libc::c_char {
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: i32 = 0;
    let mut posixopts: i32 = 0;
    posixopts = unsafe { num_posix_options() }; /* shopts modified by posix mode */
    /* Make the buffer big enough to hold the set -o options and the shopt
    options modified by posix mode. */
    temp = unsafe { malloc((1 + N_O_OPTIONS!() as i32 + posixopts) as usize) as *mut libc::c_char };
    for t in 0..N_O_OPTIONS!() {
        i = t as i32;
        if unsafe { o_options[t as usize].letter != 0 } {
            unsafe {
                *(temp.offset(t as isize)) =
                    *(find_flag(o_options[t as usize].letter)) as libc::c_char
            };
        } else {
            unsafe {
                *(temp.offset(t as isize)) =
                    GET_BINARY_O_OPTION_VALUE!(t, o_options[i as usize].name) as libc::c_char;
            }
        }
    }
    /* Add the shell options that are modified by posix mode to the end of the
    bitmap. They will be handled in set_current_options() */
    unsafe {
        get_posix_options(temp.offset(i as isize));
        *(temp.offset((i + posixopts) as isize)) = b'\0' as libc::c_char;
    }
    return temp;
}

pub fn set_current_options(bitmap: *const libc::c_char) {
    let mut i: i32;
    let mut v: i32;
    let mut cv: i32;
    let mut on_or_off: *mut i32;

    if bitmap == std::ptr::null_mut() {
        return;
    }

    for t in 0..N_O_OPTIONS!() {
        i = t as i32;
        unsafe {
            if bitmap.offset(i as isize) != std::ptr::null_mut() {
                v = FLAG_ON!();
            } else {
                v = FLAG_OFF!();
            }
        }
        if unsafe { o_options[t as usize].letter != 0 } {
            on_or_off = unsafe { find_flag(o_options[i as usize].letter) };
            if on_or_off != std::ptr::null_mut() {
                cv = FLAG_ON!();
            } else {
                cv = FLAG_OFF!();
            }
            if v != cv {
                unsafe {
                    change_flag(o_options[i as usize].letter, v);
                }
            } else {
                unsafe { cv = GET_BINARY_O_OPTION_VALUE!(i, o_options[i as usize].name) };
                if cv > 0 {
                    cv = FLAG_ON!();
                } else {
                    cv = FLAG_OFF!();
                }
                if v != cv {
                    unsafe {
                        SET_BINARY_O_OPTION_VALUE!(i, v, o_options[i as usize].name);
                    }
                }
            }
        }
    }
}

fn set_ignoreeof(on_or_off: i32, _option_name: *mut libc::c_char) -> i32 {
    on_or_off == FLAG_ON!();
    unsafe {
        ignoreeof = on_or_off;
        unbind_variable_noref(b"ignoreeof\0" as *const u8 as *const libc::c_char);
        if ignoreeof != 0 {
            bind_variable(
                b"IGNOREEOF\0" as *const u8 as *const libc::c_char,
                b"10\0" as *const u8 as *mut libc::c_char,
                0,
            );
        } else {
            unbind_variable_noref(b"IGNOREEOF\0" as *const u8 as *const libc::c_char);
        }
        sv_ignoreeof(b"IGNOREEOF\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    }
    return 0;
}

fn set_posix_mode(on_or_off: i32, _option_name: *mut libc::c_char) -> i32 {
    unsafe {
        if (on_or_off == FLAG_ON!() && posixly_correct != 0)
            || (on_or_off == FLAG_OFF!() && posixly_correct == 0)
        {
            return 0;
        }
        on_or_off == FLAG_ON!();
        posixly_correct = on_or_off;

        if posixly_correct != 0 {
            unbind_variable_noref(b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char);
        } else {
            bind_variable(
                b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char,
                b"y\0" as *const u8 as *mut libc::c_char,
                0,
            );
        }
        sv_strict_posix(b"POSIXLY_CORRECT\0" as *const u8 as *mut libc::c_char);
    }
    return 0;
}

fn set_edit_mode(on_or_off: i32, option_name: *mut libc::c_char) -> i32 {
    unsafe {
        let isemacs: i32;

        if on_or_off == FLAG_ON!() {
            rl_variable_bind(
                b"editing-mode\0" as *const u8 as *const libc::c_char,
                option_name,
            );
            if interactive > 0 {
                with_input_from_stdin();
            }

            no_line_editing = 0;
        } else {
            if rl_editing_mode == 1 {
                isemacs = 1;
            } else {
                isemacs = 0;
            }
            if isemacs != 0 && *option_name == b'e' as libc::c_char
                || (isemacs == 0 && *option_name == b'v' as libc::c_char)
            {
                if interactive > 0 {
                    with_input_from_stream(
                        stdin as *mut libc::FILE,
                        b"stdin\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }

        return 1 - no_line_editing;
    }
}

fn get_edit_mode(name: *mut libc::c_char) -> i32 {
    unsafe {
        if *name == b'e' as libc::c_char {
            if no_line_editing == 0 && rl_editing_mode == 1 {
                return 1;
            } else {
                return 0;
            }
        } else {
            if no_line_editing == 0 && rl_editing_mode == 0 {
                return 1;
            } else {
                return 0;
            }
        }
    }
}

fn bash_set_history(on_or_off: i32, _option_name: *mut libc::c_char) -> i32 {
    unsafe {
        if on_or_off == FLAG_ON!() {
            enable_history_list = 1;
            bash_history_enable();
            if history_lines_this_session == 0 {
                load_history();
            }
        } else {
            enable_history_list = 0;
            bash_history_disable();
        }
        return 1 - enable_history_list;
    }
}

#[no_mangle]
pub fn set_minus_o_option(on_or_off: i32, option_name: *mut libc::c_char) -> i32 {
    let i: i32;

    i = find_minus_o_option(option_name);

    if i < 0 {
        sh_invalidoptname(option_name);
        return EX_USAGE;
    }
    unsafe {
        if o_options[i as usize].letter == 0 {
            previous_option_value = GET_BINARY_O_OPTION_VALUE!(i, o_options[i as usize].name);
            SET_BINARY_O_OPTION_VALUE!(i, on_or_off, option_name);
            return EXECUTION_SUCCESS!();
        } else {
            previous_option_value = change_flag(o_options[i as usize].letter, on_or_off);
            if previous_option_value == FLAG_ERROR!() {
                sh_invalidoptname(option_name);
                return EXECUTION_FAILURE!();
            } else {
                return EXECUTION_SUCCESS!();
            }
        }
    }
}

fn print_all_shell_variables() {
    let mut vars = 0 as *mut *mut SHELL_VAR;
    unsafe {
        vars = all_shell_variables();
        if vars != std::ptr::null_mut() {
            print_var_list(vars);
            libc::free(vars as *mut libc::c_void);
        }
        /* POSIX.2 does not allow function names and definitions to be output when
        `set' is invoked without options (PASC Interp #202). */
        if posixly_correct == 0 {
            vars = all_shell_functions();
            if vars != std::ptr::null_mut() {
                print_func_list(vars);
                libc::free(vars as *mut libc::c_void);
            }
        }
    }
}

#[no_mangle]
pub fn set_shellopts() {
    let value: *mut libc::c_char;
    let mut tflag: [libc::c_char; N_O_OPTIONS!()] = [0 as libc::c_char; N_O_OPTIONS!()];
    let mut vsize: i32 = 0;
    let mut i: i32 = 0;
    let mut vptr: i32;
    let mut ip: *mut i32;
    let exported: i32;

    let mut v: *mut SHELL_VAR;
    unsafe {
        for j in 0..N_O_OPTIONS!() {
            i = j as i32;
            if o_options[i as usize].name != std::ptr::null_mut() {
                tflag[i as usize] = 0;
                if o_options[i as usize].letter != 0 {
                    ip = find_flag(o_options[i as usize].letter);
                    if ip != std::ptr::null_mut() && *ip != 0 {
                        vsize = vsize + strlen(o_options[i as usize].name) as u64 as u32 as i32 + 1;
                        tflag[i as usize] = 1;
                    }
                } else if GET_BINARY_O_OPTION_VALUE!(i, o_options[i as usize].name) != 0 {
                    vsize = vsize + strlen(o_options[i as usize].name) as i32 + 1;
                    tflag[i as usize] = 1;
                }
            }
        }
        value = malloc((vsize + 1) as u32 as usize) as *mut libc::c_char;
        vptr = 0;

        for j in 0..N_O_OPTIONS!() {
            i = j as i32;
            if o_options[i as usize].name != std::ptr::null_mut() {
                if tflag[i as usize] != 0 as libc::c_char {
                    libc::strcpy(value.offset(vptr as isize), o_options[i as usize].name);
                    vptr = vptr + strlen(o_options[i as usize].name) as u64 as i64 as i32;

                    *value.offset(vptr as isize) = b':' as libc::c_char;
                    vptr = vptr + 1;
                }
            }
        }

        if vptr > 0 {
            vptr = vptr - 1;
        }
        *value.offset(vptr as isize) = b'\0' as libc::c_char;

        v = find_variable(b"SHELLOPTS\0" as *const u8 as *mut libc::c_char);

        /* Turn off the read-only attribute so we can bind the new value, and
        note whether or not the variable was exported. */
        if v != std::ptr::null_mut() {
            VUNSETATTR!(v, att_readonly!());
            exported = exported_p!(v);
        } else {
            exported = 0;
        }
        v = bind_variable(b"SHELLOPTS\0" as *const u8 as *mut libc::c_char, value, 0);
        /* Turn the read-only attribute back on, and turn off the export attribute
        if it was set implicitly by mark_modified_vars and SHELLOPTS was not
        exported before we bound the new value. */

        VSETATTR_1!(v, att_readonly!());

        if mark_modified_vars != 0 && exported != 0 && exported_p!(v) != 0 {
            VUNSETATTR!(v, att_exported!());
        }
        libc::free(value as *mut libc::c_void);
    }
}

fn parse_shellopts(value: *mut libc::c_char) {
    let mut vname: *mut libc::c_char;
    let mut vptr: i32 = 0;
    loop {
        vname = unsafe { extract_colon_unit(value, &mut vptr) };
        if vname != std::ptr::null_mut() {
            break;
        }
        set_minus_o_option(FLAG_ON!(), vname);
        unsafe {
            libc::free(vname as *mut libc::c_void);
        }
    }
}

pub fn initialize_shell_options(no_shellopts: i32) {
    let temp: *mut libc::c_char;
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;

    if no_shellopts == 0 {
        var = find_variable(b"SHELLOPTS\0" as *const u8 as *const libc::c_char);
        /* set up any shell options we may have inherited. */
        unsafe {
            if !var.is_null() && imported_p!(var) != 0 {
                if assoc_p!(var) != 0 || array_p!(var) != 0 {
                    temp = std::ptr::null_mut();
                } else {
                    temp = savestring!(value_cell!(var));
                }

                if temp != std::ptr::null_mut() {
                    parse_shellopts(temp);
                    libc::free(temp as *mut libc::c_void);
                }
            }
        }
    }

    /* Set up the $SHELLOPTS variable. */
    set_shellopts();
}

#[no_mangle]
pub fn reset_shell_options() {
    unsafe {
        pipefail_opt = 0;
        ignoreeof = 0;
        posixly_correct = 0;
        dont_save_function_defs = 0;
        enable_history_list = 1;
        remember_on_history = enable_history_list;
    }
}

#[no_mangle]
pub fn set_builtin(mut list: *mut WordList) -> i32 {
    let mut on_or_off: i32;
    let mut flag_name: i32 = 0;
    let mut force_assignment: i32;
    let mut opts_changed: i32;
    let mut rv: i32;
    let mut r: i32;
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: [libc::c_char; 3] = [0 as libc::c_char; 3];
    let mut opt: i32;
    let _flag: bool = false;
    if list.is_null() {
        print_all_shell_variables();
        return sh_chkwrite(EXECUTION_SUCCESS!());
    }
    rv = EXECUTION_SUCCESS!();
    unsafe {
        reset_internal_getopt();
        opt = internal_getopt(list, optflags.as_mut_ptr());
    }
    while opt != -1 {
        let optu8: u8 = flag_name as u8;
        let optChar: char = char::from(optu8);

        match optChar {
            'i' => {
                s[0] = unsafe { list_opttype as libc::c_char };
                s[1] = b'i' as u8 as libc::c_char;
                s[2] = b'\0' as u8 as libc::c_char;

                sh_invalidopt(s.as_ptr() as *mut libc::c_char);
                builtin_usage();

                return EX_USAGE;
            }
            '?' => {
                builtin_usage();

                if unsafe { list_optopt } == b'?' as libc::c_int {
                    return EXECUTION_SUCCESS!();
                } else {
                    return EX_USAGE;
                }
            }
            _ => {
                if opt == -99 {
                    builtin_help();
                    return EX_USAGE;
                }
            }
        }
        // opt = unsafe {internal_getopt(list, optflags.as_ptr() as *mut libc::c_char)};
        opt = unsafe { internal_getopt(list, optflags.as_mut_ptr()) };
    }
    opts_changed = 0;
    force_assignment = opts_changed;
    while list != std::ptr::null_mut() {
        if unsafe { (*(*list).word).word != std::ptr::null_mut() } {
            arg = unsafe { (*(*list).word).word };
            //if (arg[0] == '-' && (!arg[1] || (arg[1] == '-' && !arg[2])))
            if unsafe {
                (*arg == b'-' as u8 as libc::c_char)
                    && (arg.offset(1 as isize) == std::ptr::null_mut()
                        || (*(arg.offset(1 as isize)) == b'-' as u8 as libc::c_char
                            && arg.offset(2 as isize) != std::ptr::null_mut()))
            } {
                //println!("*arg == b'-' && arg[1] && arg[1]== b'-'");
                unsafe {
                    list = (*list).next;
                    /* `set --' unsets the positional parameters. */
                    if *arg.offset(1 as isize) == b'-' as u8 as libc::c_char {
                        //println!("arg[1]== b'-'");
                        force_assignment = 1;
                    }
                    /* Until told differently, the old shell behaviour of
                    `set - [arg ...]' being equivalent to `set +xv [arg ...]'
                    stands.  Posix.2 says the behaviour is marked as obsolescent. */
                    else {
                        //println!("else .........");
                        change_flag('x' as i32, b'+' as i32);
                        change_flag('v' as i32, b'+' as i32);
                        opts_changed = 1;
                    }
                }
            }
            on_or_off = unsafe { *arg as i32 };
            if on_or_off != 0 && (on_or_off == '-' as i32 || on_or_off == '+' as i32) {
                unsafe {
                    arg = arg.offset(1 as isize);
                }
                flag_name = unsafe { *arg as i32 };

                while flag_name != 0 {
                    let optu8: u8 = flag_name as u8;
                    let optChar: char = char::from(optu8);

                    if optChar == '?' {
                        builtin_usage();

                        return EXECUTION_SUCCESS!();
                    } else if optChar == 'o' {
                        /* -+o option-name */

                        let mut option_name: *mut libc::c_char = 0 as *mut libc::c_char;
                        let mut opt: *mut WordList = 0 as *mut WordList;
                        unsafe {
                            opt = (*list).next;
                        }
                        if opt == std::ptr::null_mut() {
                            if on_or_off == '+' as i32 {
                                list_minus_o_opts(-1, 1);
                            } else {
                                list_minus_o_opts(-1, 0);
                            }

                            rv = sh_chkwrite(rv);
                            unsafe {
                                arg = arg.offset(1 as isize);
                            }
                            flag_name = unsafe { *arg as i32 };

                            continue;
                        }

                        unsafe {
                            if !(*opt).word.is_null() {
                                option_name = (*(*opt).word).word;
                            }
                        }

                        if option_name == std::ptr::null_mut()
                            || unsafe {
                                *option_name == '\u{0}' as libc::c_char
                                    || *option_name == '-' as libc::c_char
                                    || *option_name == '+' as libc::c_char
                            }
                        {
                            //on_or_off == '+' as i32;

                            if on_or_off == '+' as i32 {
                                list_minus_o_opts(-1, 1);
                            } else {
                                list_minus_o_opts(-1, 0);
                            }

                            unsafe {
                                arg = arg.offset(1 as isize);
                            }
                            flag_name = unsafe { *arg as i32 };

                            continue;
                        }
                        unsafe {
                            list = (*list).next; /* Skip over option name. */
                        }
                        opts_changed = 1;
                        r = set_minus_o_option(on_or_off, option_name);
                        if r != EXECUTION_SUCCESS!() {
                            set_shellopts();
                            return r;
                        }
                    } else if unsafe { change_flag(flag_name, on_or_off) == FLAG_ERROR!() } {
                        s[0] = on_or_off as libc::c_char;
                        s[1] = flag_name as libc::c_char;
                        s[2] = '\0' as i32 as libc::c_char;

                        sh_invalidopt(s.as_ptr() as *mut libc::c_char);
                        builtin_usage();
                        set_shellopts();

                        return EXECUTION_FAILURE!();
                    }
                    opts_changed = 1;
                    unsafe {
                        arg = arg.offset(1 as isize);
                    }

                    flag_name = 0;
                    //flag_name = unsafe{*arg as i32};
                }
            } else {
                break;
            }

            unsafe {
                list = (*list).next;
            }
        }
    }

    if list != std::ptr::null_mut() || force_assignment != 0 {
        remember_args(list, 1 as i32);
    }

    if opts_changed != 0 {
        set_shellopts();
    }
    return rv;
}

#[no_mangle]
pub fn unset_builtin(mut list: *mut WordList) -> i32 {
    let mut unset_function: i32 = 0;
    let mut unset_variable: i32 = 0;
    let mut unset_array: i32 = 0;
    let mut opt: i32 = 0;
    let mut nameref: i32 = 0;
    let mut any_failed: i32 = 0;
    let mut global_unset_func: i32 = 0;
    let mut global_unset_var: i32 = 0;
    let mut vflags: i32 = 0;
    let mut valid_id: i32 = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tname: *mut libc::c_char = 0 as *mut libc::c_char;

    let c_str_fnv = CString::new("fnv").unwrap();
    unsafe {
        reset_internal_getopt();
        opt = internal_getopt(list, c_str_fnv.as_ptr() as *mut libc::c_char);

        while opt != -1 {
            let optu8: u8 = opt as u8;
            let optChar: char = char::from(optu8);
            match optChar {
                'f' => {
                    global_unset_func = 1;
                }
                'v' => {
                    global_unset_var = 0;
                }
                'n' => {
                    nameref = 1;
                }
                _ => {
                    if opt == -99 {
                        builtin_help();
                        return EX_USAGE;
                    }
                    builtin_usage();
                    return EX_USAGE;
                }
            }
            opt = internal_getopt(list, c_str_fnv.as_ptr() as *mut libc::c_char);
        }
        //println!("unset func={},  unset val=%{}", global_unset_func, global_unset_var);

        list = loptend;

        if global_unset_func != 0 && global_unset_var != 0 {
            builtin_error(
                b"cannot simultaneously unset a function and a variable \0" as *const u8
                    as *const libc::c_char,
            );
            return EXECUTION_FAILURE!();
        } else if unset_function != 0 && nameref != 0 {
            nameref = 0;
        }

        if assoc_expand_once != 0 {
            vflags = VA_NOEXPAND!() | VA_ONEWORD!();
        } else {
            vflags = 0;
        }
        while !list.is_null() {
            let mut var: *mut SHELL_VAR;
            let mut tem: i32 = 0;

            let mut t: *mut libc::c_char = 0 as *mut libc::c_char;

            name = (*(*list).word).word;
            unset_function = global_unset_func;
            unset_variable = global_unset_var;
            unset_array = 0;

            if !unset_function == 0 && nameref == 0 && valid_array_reference(name, vflags) != 0 {
                t = libc::strchr(name, '[' as i32);
                *t.offset(1 as isize) = b'\0' as i32 as libc::c_char;
                unset_array = unset_array + 1;
            }

            valid_id = legal_identifier(name);

            if global_unset_func == 0 && global_unset_var == 0 && valid_id == 0 {
                unset_array = 0;
                unset_variable = unset_array;
                unset_function = 1;
            }

            if unset_function == 0 && valid_id == 0 {
                sh_invalidid(name);
                any_failed = any_failed + 1;
                list = (*list).next;
            }

            if unset_function != 0 {
                var = find_function(name);
            } else {
                if nameref != 0 {
                    var = find_variable_last_nameref(name, 0);
                } else {
                    var = find_variable(name);
                }
            }

            if var != std::ptr::null_mut() && unset_function == 0 && non_unsettable_p!(var) != 0 {
                builtin_error(
                    b"%s: cannot unset \0" as *const u8 as *const libc::c_char,
                    name,
                );
                any_failed = any_failed + 1;
                list = (*list).next;
            }

            if var != std::ptr::null_mut()
                && unset_function == 0
                && nameref == 0
                && STREQ(name, name_cell!(var))
            {
                name = name_cell!(var);
            }

            if var == std::ptr::null_mut()
                && nameref == 0
                && unset_variable == 0
                && unset_function == 0
            {
                var = find_function(name);
                if var != std::ptr::null_mut() {
                    unset_function = 1;
                }
            }

            if var != std::ptr::null_mut() && readonly_p!(var) != 0 {
                if unset_function != 0 {
                    builtin_error(
                        b"%s: cannot unset: readonly %s  \0 " as *const u8 as *mut libc::c_char,
                        (*var).name,
                        b"function\0" as *const u8 as *mut libc::c_char,
                    );
                } else {
                    builtin_error(
                        b"%s: cannot unset: readonly %s \0" as *const u8 as *mut libc::c_char,
                        (*var).name,
                        b"variable\0" as *const u8 as *mut libc::c_char,
                    );
                }
                any_failed = any_failed + 1;
                list = (*list).next;
            }
            // #if defined (ARRAY_VARS)
            if var != std::ptr::null_mut() && unset_array != 0 {
                /* Let unbind_array_element decide what to do with non-array vars */
                tem = unbind_array_element(var, t, vflags); /* XXX new third arg */
                if tem == -2 && array_p!(var) == 0 && assoc_p!(var) == 0 {
                    builtin_error(
                        b"%s: not an array variable\0" as *const u8 as *const libc::c_char,
                        (*var).name,
                    );
                    any_failed = any_failed + 1;
                    list = (*list).next;
                } else if tem < 0 {
                    any_failed = any_failed + 1;
                }
            } else {
                if var == std::ptr::null_mut() && nameref == 0 && unset_function == 0 {
                    var = find_variable_last_nameref(name, 0);
                    if var != std::ptr::null_mut() && nameref_p!(var) != 0 {
                        if valid_array_reference(nameref_cell!(var), 0) != 0 {
                            tname = savestring!(nameref_cell!(var));
                            var = array_variable_part(tname, 0, &mut t, &mut 0);
                            if var != std::ptr::null_mut() {
                                tem = unbind_array_element(var, t, vflags); /* XXX new third arg */
                            }
                            libc::free(tname as *mut libc::c_void);
                        } else {
                            tem = unbind_variable(nameref_cell!(var));
                        }
                    } else {
                        tem = unbind_variable(name);
                    }
                } else {
                    if unset_function != 0 {
                        tem = unbind_func(name);
                    } else if nameref != 0 {
                        tem = unbind_nameref(name);
                    } else {
                        tem = unbind_variable(name);
                    }
                }
            }

            if tem == -1 && nameref == 0 && unset_function == 0 && unset_variable == 0 {
                tem = unbind_func(name);
            }
            name = (*(*list).word).word;

            if unset_function == 0 {
                stupidly_hack_special_variables(name);
            }
            list = (*list).next;
        }

        if any_failed != 0 {
            return EXECUTION_FAILURE!();
        } else {
            return EXECUTION_SUCCESS!();
        }
    }
}
