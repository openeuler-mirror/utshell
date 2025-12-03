/*
 * SPDX-FileCopyrightText: 2025 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: GPL-2.0-or-later
 */
use super::help::builtin_help;
use crate::bashline::{enable_hostname_completion, set_directory_hook};
use crate::builtins::bashgetopt::{internal_getopt, reset_internal_getopt};
use crate::builtins::cd::{cdable_vars, cdspelling};
use crate::builtins::common::{builtin_usage, sh_chkwrite, sh_invalidoptname};
use crate::builtins::echo::xpg_echo;
use crate::builtins::set::{
    list_minus_o_opts, minus_o_option_value, set_minus_o_option, set_shellopts,
};
use crate::builtins::source::source_uses_path;
use crate::dispose_cmd::dispose_words;
use crate::error::gnu_error_format;
use crate::general::extract_colon_unit;
use crate::make_cmd::{make_word, make_word_list};
use crate::pcomplete::progcomp_alias;
use crate::src_common::*;
use crate::variables::find_variable;
use crate::variables::{bind_variable, init_bash_argv};
use crate::version::shell_compatibility_level;
use crate::y_tab::{expand_aliases, extended_quote, promptvars};

extern "C" {
    fn shell_is_restricted(_: *mut libc::c_char) -> i32;
    static mut no_exit_on_failed_exec: i32;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct RShoptVars {
    pub name: *mut libc::c_char,
    pub value: *mut i32,
    pub set_func: Option<ShoptSetFuncT>,
}
pub type ShoptSetFuncT = fn(*mut libc::c_char, i32) -> i32;

static mut SHOPT_LOGIN_SHELL: i32 = 0;
static mut SHOPT_COMPAT31: i32 = 0;
static mut SHOPT_COMPAT32: i32 = 0;
static mut SHOPT_COMPAT40: i32 = 0;
static mut SHOPT_COMPAT41: i32 = 0;
static mut SHOPT_COMPAT42: i32 = 0;
static mut SHOPT_COMPAT43: i32 = 0;
static mut SHOPT_COMPAT44: i32 = 0;
static mut SHOPT_VARS: [RShoptVars; 54] = unsafe {
    [
        {
            let init = RShoptVars {
                name: b"autocd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &autocd as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"assoc_expand_once\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &assoc_expand_once as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"cdable_vars\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &cdable_vars as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"cdspell\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &cdspelling as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"checkhash\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &check_hashed_filenames as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"checkjobs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &check_jobs_at_exit as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"checkwinsize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &check_window_size as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"cmdhist\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &command_oriented_history as *const i32 as *mut i32,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"compat31\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &SHOPT_COMPAT31 as *const i32 as *mut libc::c_int,
                set_func: Some(set_compatibility_level as fn(*mut libc::c_char, i32) -> i32),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"compat32\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &SHOPT_COMPAT32 as *const i32 as *mut libc::c_int,
                set_func: Some(set_compatibility_level as fn(*mut libc::c_char, i32) -> i32),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"compat40\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &SHOPT_COMPAT40 as *const i32 as *mut libc::c_int,
                set_func: Some(set_compatibility_level as fn(*mut libc::c_char, i32) -> i32),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"compat41\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &SHOPT_COMPAT41 as *const i32 as *mut libc::c_int,
                set_func: Some(set_compatibility_level as fn(*mut libc::c_char, i32) -> i32),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"compat42\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &SHOPT_COMPAT42 as *const i32 as *mut libc::c_int,
                set_func: Some(set_compatibility_level as fn(*mut libc::c_char, i32) -> i32),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"compat43\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &SHOPT_COMPAT43 as *const i32 as *mut libc::c_int,
                set_func: Some(set_compatibility_level as fn(*mut libc::c_char, i32) -> i32),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"compat44\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &SHOPT_COMPAT44 as *const i32 as *mut libc::c_int,
                set_func: Some(set_compatibility_level as fn(*mut libc::c_char, i32) -> i32),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"complete_fullquote\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &complete_fullquote as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"direxpand\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &dircomplete_expand as *const i32 as *mut libc::c_int,
                set_func: Some(shopt_set_complete_direxpand as fn(*mut libc::c_char, i32) -> i32),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"dirspell\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &dircomplete_spelling as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"dotglob\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &glob_dot_filenames as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"execfail\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &no_exit_on_failed_exec as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"expand_aliases\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &expand_aliases as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"extdebug\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &debugging_mode as *const i32 as *mut libc::c_int,
                set_func: Some(shopt_set_debug_mode as fn(*mut libc::c_char, i32) -> i32),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"extglob\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &extended_glob as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"extquote\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &extended_quote as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"failglob\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &fail_glob_expansion as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"force_fignore\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &force_fignore as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"globasciiranges\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &glob_asciirange as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"globstar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &glob_star as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"gnu_errfmt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &gnu_error_format as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"histappend\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &force_append_history as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"histreedit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &history_reediting as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"histverify\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &hist_verify as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"hostcomplete\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &perform_hostname_completion as *const i32 as *mut i32,
                set_func: Some(
                    shopt_enable_hostname_completion as fn(*mut libc::c_char, i32) -> i32,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"huponexit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &hup_on_exit as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"inherit_errexit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &inherit_errexit as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"interactive_comments\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &interactive_comments as *const i32 as *mut libc::c_int,
                set_func: Some(set_shellopts_after_change as fn(*mut libc::c_char, i32) -> i32),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"lastpipe\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &lastpipe_opt as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"lithist\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &literal_history as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"localvar_inherit\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &localvar_inherit as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"localvar_unset\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &localvar_unset as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"login_shell\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &SHOPT_LOGIN_SHELL as *const i32 as *mut i32,
                set_func: Some(set_login_shell as fn(*mut libc::c_char, i32) -> i32),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"mailwarn\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &mail_warning as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"no_empty_cmd_completion\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &no_empty_command_completion as *const i32 as *mut i32,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"nocaseglob\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &glob_ignore_case as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"nocasematch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &match_ignore_case as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"nullglob\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &allow_null_glob_expansion as *const i32 as *mut i32,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"progcomp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &prog_completion_enabled as *const i32 as *mut i32,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"progcomp_alias\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &progcomp_alias as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"promptvars\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &promptvars as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"restricted_shell\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &restricted_shell as *const i32 as *mut libc::c_int,
                set_func: Some(set_restricted_shell as fn(*mut libc::c_char, i32) -> i32),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"shift_verbose\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &print_shift_error as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"sourcepath\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &source_uses_path as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"xpg_echo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value: &xpg_echo as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: 0 as *const libc::c_char as *mut libc::c_char,
                value: 0 as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<*mut libc::c_void, Option<ShoptSetFuncT>>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
            };
            init
        },
    ]
};

static SFLAG: i32 = 0x01;
static UFLAG: i32 = 0x02;
static QFLAG: i32 = 0x04;
static OFLAG: i32 = 0x08;
static PFLAG: i32 = 0x10;
static SETOPT: i32 = 1;
static UNSETOPT: i32 = 0;

static mut ON: *const libc::c_char = b"on\0" as *const u8 as *const libc::c_char;
static mut OFF: *const libc::c_char = b"off\0" as *const u8 as *const libc::c_char;
#[no_mangle]
pub fn shopt_builtin(mut list: *mut WordList) -> i32 {
    let mut opt: i32;
    let mut flags: i32 = 0;
    let mut rval: i32 = 0;

    reset_internal_getopt();
    let psuoq = CString::new("psuoq").expect("CString::new failed");
    loop {
        opt = internal_getopt(list, psuoq.as_ptr() as *mut libc::c_char);
        if !(opt != -(1 as i32)) {
            break;
        }
        let opt_char = opt as u8 as char;
        match opt_char {
            's' => {
                flags |= SFLAG;
            }
            'u' => flags |= UFLAG,
            'q' => {
                flags |= QFLAG;
            }
            'o' => {
                flags |= OFLAG;
            }
            'p' => {
                flags |= PFLAG;
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
    }
    list = unsafe { loptend };
    if flags & (SFLAG | UFLAG) == SFLAG | UFLAG {
        unsafe {
            builtin_error(dcgettext(
                0 as *const libc::c_char,
                b"cannot set and unset shell options simultaneously\0" as *const u8
                    as *const libc::c_char,
                5 as i32,
            ))
        };
        return EXECUTION_FAILURE!();
    }

    rval = EXECUTION_SUCCESS;
    if (flags & OFLAG != 0) && ((flags & (SFLAG | UFLAG)) == 0)
    // shopt -o
    {
        //设置了o-flag，并没设s或u-flag
        rval = list_shopt_o_options(list, flags);
    } else if !list.is_null() && flags & OFLAG != 0 {
        //设置-o了   //shopt -so args , shopt -u args
        rval = set_shopt_o_options(
            if flags & SFLAG != 0 {
                '-' as i32 /*on*/
            } else {
                '+' as i32 /*off*/
            },
            list,
            flags & QFLAG, //是否沉默?
        );
    } else if flags & OFLAG != 0 {
        // shopt -so
        rval = list_some_o_options(if flags & SFLAG != 0 { 1 } else { 0 }, flags);
    } else if !list.is_null() && flags & (SFLAG | UFLAG) != 0 {
        // shopt -s/u args
        rval = toggle_shopts(if flags & SFLAG != 0 { 1 } else { 0 }, list, flags & QFLAG);
    } else if flags & (SFLAG | UFLAG) == 0 {
        // shopt [args]
        //println!("shopt   ===list all ");
        rval = list_shopts(list, flags);
    } else {
        // shopt -su
        rval = list_some_shopts(if flags & SFLAG != 0 { SETOPT } else { UNSETOPT }, flags);
    }
    return rval;
}

//把环境变量置0
#[no_mangle]
pub fn reset_shopt_options() {
    unsafe {
        cdspelling = 0;
        cdable_vars = 0;
        autocd = 0;
        check_hashed_filenames = 0;
        check_window_size = 1;
        glob_dot_filenames = 0;
        allow_null_glob_expansion = glob_dot_filenames;
        no_exit_on_failed_exec = 0;
        expand_aliases = 0;
        extended_quote = 1;
        fail_glob_expansion = 0;
        glob_asciirange = 1;
        glob_star = 0;
        gnu_error_format = 0;
        hup_on_exit = 0;
        inherit_errexit = 0;
        interactive_comments = 1;
        lastpipe_opt = 0;
        localvar_unset = 0;
        localvar_inherit = localvar_unset;
        mail_warning = 0;
        match_ignore_case = 0;
        glob_ignore_case = match_ignore_case;
        print_shift_error = 0;
        promptvars = 1;
        source_uses_path = promptvars;
        check_jobs_at_exit = 0;
        extended_glob = 0;
        assoc_expand_once = 0;
        literal_history = 0;
        force_append_history = 0;
        command_oriented_history = 1;
        complete_fullquote = 1;
        force_fignore = 1;
        history_reediting = 0;
        hist_verify = history_reediting;
        perform_hostname_completion = 1;
        dircomplete_expand = 0;
        dircomplete_spelling = 0;
        no_empty_command_completion = 0;
        prog_completion_enabled = 1;
        progcomp_alias = 0;
        xpg_echo = 0;
        SHOPT_LOGIN_SHELL = login_shell;
    }
}

fn find_shopt(name: *mut libc::c_char) -> i32 {
    let mut i = 0;
    for item in unsafe { SHOPT_VARS } {
        i += 1;
        if item.name.is_null() {
            return -1;
        }
        if unsafe { strcmp(name, SHOPT_VARS[i as usize].name) } == 0 {
            return i;
        }
    }
    return -1;
}

fn shopt_error(s: *mut libc::c_char) {
    unsafe {
        builtin_error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: invalid shell option name\0" as *const u8 as *const libc::c_char,
                5 as i32,
            ),
            s,
        )
    };
}

fn toggle_shopts(mode: i32, list: *mut WordList, _quiet: i32) -> i32 {
    let mut l: *mut WordList;
    let mut ind: i32;
    let mut rval: i32;
    let v: *mut ShellVar;
    l = list;
    rval = EXECUTION_SUCCESS!();
    while !l.is_null() {
        unsafe {
            ind = find_shopt((*(*l).word).word);
            if ind < 0 {
                shopt_error((*(*l).word).word);
                rval = EXECUTION_FAILURE!();
            } else {
                *SHOPT_VARS[ind as usize].value = mode;
                if (SHOPT_VARS[ind as usize].set_func).is_some() {
                    SHOPT_VARS[ind as usize].set_func.expect("")(
                        SHOPT_VARS[ind as usize].name,
                        mode,
                    );
                    /*
                    (Some(
                        ((*SHOPT_VARS.as_mut_ptr().offset(ind as isize)).set_func)
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(SHOPT_VARS[ind as usize].name, mode);
                    */
                }
            }
            l = (*l).next;
        }
    }

    v = find_variable(b"BASHOPTS\0" as *const u8 as *const libc::c_char);

    if !v.is_null() {
        set_bashopts();
    }
    return rval;
}
fn print_shopt(name: *mut libc::c_char, val: i32, flags: i32) {
    let msg: CString = CString::new("shopt %s %s\n").expect("CString new faild");
    let s: CString = CString::new("-s").expect("CString new faild");
    let u: CString = CString::new("-u").expect("CString new faild");
    let optfmt: CString = CString::new("%-15s\t%s\n").expect("CString new faild");
    if flags & PFLAG != 0 {
        unsafe {
            printf(
                msg.as_ptr(),
                if val != 0 { s.as_ptr() } else { u.as_ptr() },
                name,
            )
        };
    } else {
        unsafe { printf(optfmt.as_ptr(), name, if val != 0 { ON } else { OFF }) };
    };
}

fn list_shopts(list: *mut WORD_LIST, flags: libc::c_int) -> libc::c_int {
    let mut l: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut i: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut rval: libc::c_int = 0;

    if list.is_null() {
        i = 0 as libc::c_int;
        unsafe {
            while !(SHOPT_VARS[i as usize].name).is_null() {
                val = *SHOPT_VARS[i as usize].value;
                if flags & QFLAG as libc::c_int == 0 as libc::c_int {
                    print_shopt(SHOPT_VARS[i as usize].name, val, flags);
                }
                i += 1;
            }
        }
        return sh_chkwrite(EXECUTION_SUCCESS!());
    }

    l = list;
    rval = EXECUTION_SUCCESS!();
    while !l.is_null() {
        unsafe {
            i = find_shopt((*(*l).word).word);
            if i < 0 as libc::c_int {
                shopt_error((*(*l).word).word);
                rval = EXECUTION_FAILURE!();
            } else {
                val = *SHOPT_VARS[i as usize].value;
                if val == 0 as libc::c_int {
                    rval = EXECUTION_FAILURE!();
                }
                if flags & QFLAG as libc::c_int == 0 as libc::c_int {
                    print_shopt((*(*l).word).word, val, flags);
                }
            }
            l = (*l).next;
        }
    }
    return sh_chkwrite(rval);
}

fn list_some_shopts(mode: i32, flags: i32) -> i32 {
    unsafe {
        for item in SHOPT_VARS {
            if ((flags & QFLAG) == 0) && item.value != std::ptr::null_mut() && mode == *item.value {
                print_shopt(item.name, *item.value, flags);
            }
        }
        return sh_chkwrite(EXECUTION_SUCCESS!());
    }
}

fn list_shopt_o_options(list: *mut WordList, flags: i32) -> i32 {
    let mut l: *mut WordList = 0 as *mut WordList;
    let mut val: i32 = 0;
    let mut rval: i32 = EXECUTION_SUCCESS!();
    if list.is_null() {
        if flags & QFLAG == 0 {
            list_minus_o_opts(-1, flags & PFLAG);
        }
        return sh_chkwrite(EXECUTION_SUCCESS!());
    }
    l = list;
    unsafe {
        while !l.is_null() {
            val = minus_o_option_value((*(*l).word).word);
            if val == -1 {
                sh_invalidoptname((*(*l).word).word);
                rval = EXECUTION_FAILURE!();
            } else {
                if val == 0 {
                    rval = EXECUTION_FAILURE!();
                }
                if flags & QFLAG == 0 {
                    if flags & PFLAG != 0 {
                        printf(
                            b"set %co %s\n\0" as *const u8 as *const libc::c_char,
                            if val != 0 { '-' as i32 } else { '+' as i32 },
                            (*(*l).word).word,
                        );
                        println!(
                            "set {:?}o %{:?}",
                            if val != 0 { b'-' } else { b'+' },
                            (*(*l).word).word
                        )
                    } else {
                        printf(
                            b"%-15s\t%s\n\0" as *const u8 as *const libc::c_char,
                            (*(*l).word).word,
                            if val != 0 { ON } else { OFF },
                        );
                    }
                }
            }
            l = (*l).next;
        }
    }
    return sh_chkwrite(rval);
}

fn list_some_o_options(mode: i32, flags: i32) -> i32 {
    if flags & QFLAG == 0 {
        list_minus_o_opts(mode, flags & PFLAG);
    }
    return sh_chkwrite(EXECUTION_SUCCESS!());
}
fn set_shopt_o_options(mode: i32, list: *mut WordList, _quiet: i32) -> i32 {
    //let mut l: *mut WordList =0 as *mut WordList;
    let mut l: *mut WordList;
    let mut rval: i32;
    l = list;
    rval = EXECUTION_SUCCESS!();
    while !l.is_null() {
        unsafe {
            if set_minus_o_option(mode, (*(*l).word).word) == 1 as i32 {
                rval = 1 as i32;
            }
            l = (*l).next;
        }
    }
    set_shellopts();
    return rval;
}
fn set_shellopts_after_change(_option_name: *mut libc::c_char, _mode: i32) -> i32 {
    set_shellopts();
    return 0;
}
fn shopt_set_debug_mode(_option_name: *mut libc::c_char, _mode: i32) -> i32 {
    unsafe {
        function_trace_mode = debugging_mode;
        error_trace_mode = function_trace_mode;
        set_shellopts();
        if debugging_mode != 0 {
            init_bash_argv();
        }
        return 0;
    }
}
fn shopt_enable_hostname_completion(_option_name: *mut libc::c_char, mode: i32) -> i32 {
    return enable_hostname_completion(mode);
}
fn set_compatibility_level(option_name: *mut libc::c_char, mode: i32) -> i32 {
    let mut ind: i32 = 0;
    let mut rhs: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        if mode != 0 {
            SHOPT_COMPAT32 = 0 as i32;
            SHOPT_COMPAT31 = SHOPT_COMPAT32;
            SHOPT_COMPAT43 = 0 as i32;
            SHOPT_COMPAT42 = SHOPT_COMPAT43;
            SHOPT_COMPAT41 = SHOPT_COMPAT42;
            SHOPT_COMPAT40 = SHOPT_COMPAT41;
            SHOPT_COMPAT44 = 0 as i32;
            ind = find_shopt(option_name);
            *SHOPT_VARS[ind as usize].value = mode;
        }
        if SHOPT_COMPAT31 != 0 {
            shell_compatibility_level = 31 as i32;
        } else if SHOPT_COMPAT32 != 0 {
            shell_compatibility_level = 32 as i32;
        } else if SHOPT_COMPAT40 != 0 {
            shell_compatibility_level = 40 as i32;
        } else if SHOPT_COMPAT41 != 0 {
            shell_compatibility_level = 41 as i32;
        } else if SHOPT_COMPAT42 != 0 {
            shell_compatibility_level = 42 as i32;
        } else if SHOPT_COMPAT43 != 0 {
            shell_compatibility_level = 43 as i32;
        } else if SHOPT_COMPAT44 != 0 {
            shell_compatibility_level = 44 as i32;
        } else {
            shell_compatibility_level = 51 as i32;
        }
        rhs = itos(shell_compatibility_level as intmax_t);
        bind_variable(
            b"BASH_COMPAT\0" as *const u8 as *const libc::c_char,
            rhs,
            0 as i32,
        );
        free(rhs as *mut libc::c_void);
    }
    return 0;
}
#[no_mangle]
pub fn set_compatibility_opts() {
    unsafe {
        SHOPT_COMPAT32 = 0;
        SHOPT_COMPAT31 = 0;
        SHOPT_COMPAT43 = 0;
        SHOPT_COMPAT42 = 0;
        SHOPT_COMPAT41 = 0;
        SHOPT_COMPAT40 = 0;
        SHOPT_COMPAT44 = 0;
        match shell_compatibility_level {
            44 => {
                SHOPT_COMPAT44 = 1;
            }
            43 => {
                SHOPT_COMPAT43 = 1;
            }
            42 => {
                SHOPT_COMPAT42 = 1;
            }
            41 => {
                SHOPT_COMPAT41 = 1;
            }
            40 => {
                SHOPT_COMPAT40 = 1;
            }
            32 => {
                SHOPT_COMPAT32 = 1;
            }
            31 => {
                SHOPT_COMPAT31 = 1;
            }
            _ => {}
        };
    }
}
fn shopt_set_complete_direxpand(_option_name: *mut libc::c_char, _mode: i32) -> i32 {
    set_directory_hook();
    return 0;
}
fn set_restricted_shell(_option_name: *mut libc::c_char, _mode: i32) -> i32 {
    static mut SAVE_RESTRICTED: i32 = -1;
    unsafe {
        if SAVE_RESTRICTED == -1 {
            SAVE_RESTRICTED = shell_is_restricted(shell_name);
        }
        restricted_shell = SAVE_RESTRICTED;
    }
    return 0;
}
#[no_mangle]
pub fn set_login_shell(_option_name: *mut libc::c_char, _mode: i32) -> i32 {
    unsafe {
        SHOPT_LOGIN_SHELL = if login_shell != 0 { 1 } else { 0 };
        return 0;
    }
}
#[no_mangle]
pub fn get_shopt_options() -> *mut *mut libc::c_char {
    unsafe {
        let mut ret: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut n: i32 = 0;
        let mut i: i32 = 0;
        n = (::std::mem::size_of::<[RShoptVars; 54]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<RShoptVars>() as libc::c_ulong) as i32;

        ret = strvec_create(n + 1 as i32);
        i = 0 as i32;
        while !(SHOPT_VARS[i as usize].name).is_null() {
            let ref mut fresh0 = *ret.offset(i as isize);
            *fresh0 = strcpy(
                malloc(
                    (1 as i32 as libc::c_ulong)
                        .wrapping_add(strlen(SHOPT_VARS[i as usize].name) as u64)
                        as usize,
                ) as *mut libc::c_char,
                SHOPT_VARS[i as usize].name,
            );
            i += 1;
        }
        let ref mut fresh1 = *ret.offset(i as isize);
        *fresh1 = 0 as *mut libc::c_void as *mut libc::c_char;
        return ret;
    }
}

#[no_mangle]
pub fn shopt_setopt(name: *mut libc::c_char, mode: i32) -> i32 {
    unsafe {
        let wl: *mut WordList;
        let r: i32;

        wl = make_word_list(make_word(name), std::ptr::null_mut());
        //wl = make_word_list(make_word(name), 0 as *mut libc::c_void as *mut WordList);
        r = toggle_shopts(mode, wl, 0);
        dispose_words(wl);
        return r;
    }
}
#[no_mangle]
pub fn shopt_listopt(name: *mut libc::c_char, reusable: i32) -> i32 {
    let mut i: i32 = 0;
    if name.is_null() {
        return list_shopts(
            // 0 as *mut libc::c_void as *mut WordList,
            std::ptr::null_mut(),
            if reusable != 0 { PFLAG } else { 0 },
        );
    }
    i = find_shopt(name);
    if i < 0 {
        shopt_error(name);
        return 1;
    }
    print_shopt(
        name,
        unsafe { *SHOPT_VARS[i as usize].value },
        if reusable != 0 { PFLAG } else { 0 },
    );
    return sh_chkwrite(EXECUTION_SUCCESS!());
}
#[no_mangle]
pub fn set_bashopts() {
    let value: *mut libc::c_char;
    let mut tflag: [libc::c_char; 54] = [0; 54];
    let mut vsize: i32;
    let mut i: i32;
    let mut vptr: i32;
    /*
    let mut ip: *mut i32;
    */
    let exported: i32;
    let mut v: *mut ShellVar;
    unsafe {
        i = 0;
        vsize = 0;
        while !(SHOPT_VARS[i as usize].name).is_null() {
            tflag[i as usize] = 0 as libc::c_char;
            if *SHOPT_VARS[i as usize].value != 0 {
                vsize += strlen(SHOPT_VARS[i as usize].name) as i32;
                vsize += 1;
                /*
                vsize = (vsize as libc::c_ulong)
                    .wrapping_add(
                        (strlen(SHOPT_VARS[i as usize].name))
                            .wrapping_add(1 as i32 as libc::c_ulong),
                    ) as i32 as libc::c_int;
                */
                tflag[i as usize] = 1 as libc::c_char;
            }
            i += 1;
        }
        value = libc::malloc((vsize + 1) as usize) as *mut libc::c_char;
        vptr = 0;
        i = 0;
        while !(SHOPT_VARS[i as usize].name).is_null() {
            if tflag[i as usize] != 0 {
                strcpy(value.offset(vptr as isize), SHOPT_VARS[i as usize].name);
                vptr = (vptr as libc::c_ulong)
                    .wrapping_add(strlen(SHOPT_VARS[i as usize].name) as u64)
                    as i32 as i32;
                let fresh2 = vptr;
                vptr = vptr + 1;
                *value.offset(fresh2 as isize) = ':' as i32 as libc::c_char;
            }
            i += 1;
        }
        //printf(b"the values=%s" as *const u8 as *const libc::c_char, value);
        if vptr != 0 {
            vptr -= 1;
        }
        *value.offset(vptr as isize) = '\u{0}' as i32 as libc::c_char;
        v = find_variable(b"BASHOPTS\0" as *const u8 as *const libc::c_char);
        if !v.is_null() {
            (*v).attributes &= !(0x2 as i32);
            exported = (*v).attributes & 0x1 as i32;
        } else {
            exported = 0;
        }
        v = bind_variable(b"BASHOPTS\0" as *const u8 as *const libc::c_char, value, 0);
        (*v).attributes |= 0x2;
        if mark_modified_vars != 0 && exported == 0 as i32 && (*v).attributes & 0x1 != 0 {
            (*v).attributes &= !(0x1);
        }
        libc::free(value as *mut libc::c_void);
    }
}
#[no_mangle]
pub fn parse_bashopts(value: *mut libc::c_char) {
    let mut vname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vptr: i32 = 0;
    let mut ind: i32 = 0;
    vptr = 0 as i32;
    unsafe {
        loop {
            vname = extract_colon_unit(value, &mut vptr);
            if vname.is_null() {
                break;
            }
            ind = find_shopt(vname);
            if ind >= 0 as i32 {
                *SHOPT_VARS[ind as usize].value = 1 as i32;
                if (SHOPT_VARS[ind as usize].set_func).is_some() {
                    (Some(
                        ((*SHOPT_VARS.as_mut_ptr().offset(ind as isize)).set_func)
                            .expect("non-null function pointer"),
                    ))
                    .expect("non-null function pointer")(
                        SHOPT_VARS[ind as usize].name, 1 as i32
                    );
                }
            }
            free(vname as *mut libc::c_void);
        }
    }
}
#[no_mangle]
pub fn initialize_bashopts(no_bashopts: i32) {
    unsafe {
        let temp: *mut libc::c_char;
        let var: *mut ShellVar;
        if no_bashopts == 0 {
            var = find_variable(b"BASHOPTS\0" as *const u8 as *const libc::c_char);
            if !var.is_null() && (*var).attributes & att_imported as i32 != 0 {
                temp = if (*var).attributes & att_array != 0
                    || (*var).attributes & att_assoc as i32 != 0
                {
                    std::ptr::null_mut()
                } else {
                    strcpy(
                        malloc(
                            (1 as libc::c_ulong).wrapping_add(strlen((*var).value) as u64) as usize,
                        ) as *mut libc::c_char,
                        (*var).value,
                    )
                };
                if !temp.is_null() {
                    parse_bashopts(temp);
                    free(temp as *mut libc::c_void);
                }
            }
        }
        set_bashopts();
    }
}
