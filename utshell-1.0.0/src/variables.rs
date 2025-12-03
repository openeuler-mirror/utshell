//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later
use crate::alias::add_alias;
use crate::array::array_copy;
use crate::array::array_create;
use crate::arrayfunc::array_variable_name;
use crate::arrayfunc::array_variable_part;
use crate::arrayfunc::assign_array_element;
use crate::arrayfunc::bind_array_variable;
use crate::arrayfunc::convert_var_to_array;
use crate::arrayfunc::convert_var_to_assoc;
use crate::arrayfunc::make_array_variable_value;
use crate::arrayfunc::print_array_assignment;
use crate::arrayfunc::print_assoc_assignment;
use crate::arrayfunc::valid_array_reference;
use crate::assoc::assoc_dispose;
use crate::assoc::assoc_insert;
use crate::assoc::assoc_reference;
use crate::bashhist::history_number;
use crate::bashhist::setup_history_ignore;
use crate::bashline::clear_hostname_list;
use crate::bashline::enable_hostname_completion;
use crate::bashline::hostname_list_initialized;
use crate::bashline::posix_readline_initialize;
use crate::bashline::reset_completer_word_break_chars;
use crate::builtins::common::get_working_directory;
use crate::builtins::common::set_dollar_vars_unchanged;
use crate::builtins::common::set_working_directory;
use crate::builtins::common::sh_invalidid;
use crate::builtins::common::sh_notfound;
use crate::builtins::common::sh_readonly;
use crate::builtins::common::sh_restricted;
use crate::builtins::getopts::getopts_reset;
use crate::builtins::pushd::get_directory_stack;
use crate::builtins::pushd::set_dirstack_element;
use crate::builtins::set::set_current_options;
use crate::builtins::set::set_shellopts;
use crate::builtins::shopt::set_compatibility_opts;
use crate::copycmd::copy_function_def;
use crate::copycmd::copy_function_def_contents;
use crate::dispose_cmd::dispose_command;
use crate::dispose_cmd::dispose_function_def;
use crate::dispose_cmd::dispose_function_def_contents;
use crate::dispose_cmd::dispose_words;
use crate::execute_cmd::executing_line_number;
use crate::expr::evalexp;
use crate::findcmd::executable_file;
use crate::findcmd::file_status;
use crate::findcmd::find_user_command;
use crate::findcmd::setup_exec_ignore;
use crate::general::assignment;
use crate::general::file_isdir;
use crate::general::get_group_list;
use crate::general::same_file;
use crate::general::sh_validfd;
use crate::hashcmd::phash_flush;
use crate::hashcmd::phash_insert;
use crate::hashlib::hash_copy;
use crate::hashlib::hash_create;
use crate::hashlib::hash_dispose;
use crate::hashlib::hash_flush;
use crate::hashlib::hash_insert;
use crate::hashlib::hash_remove;
use crate::hashlib::hash_search;
use crate::jobs::set_maxchild;
use crate::local::locale_decpoint;
use crate::local::set_lang;
use crate::local::set_locale_var;
use crate::mailcheck::free_mail_files;
use crate::mailcheck::remember_mail_dates;
use crate::mailcheck::reset_mail_timer;
use crate::pathexp::setup_glob_ignore;
use crate::pcomplete::it_functions;
use crate::pcomplete::set_itemlist_dirty;
use crate::print_cmd::named_function_string;
use crate::print_cmd::xtrace_print_assignment;
use crate::print_cmd::xtrace_reset;
use crate::print_cmd::xtrace_set;
use crate::readline::absolute_program;
use crate::readline::all_digits;
use crate::readline::array_dispose;
use crate::readline::array_dispose_element;
use crate::readline::array_flush;
use crate::readline::array_from_word_list;
use crate::readline::array_insert;
use crate::readline::array_reference;
use crate::readline::array_rshift;
use crate::readline::array_shift;
use crate::readline::bash_tilde_expand;
use crate::readline::check_selfref;
use crate::readline::copy_command;
use crate::readline::environ;
use crate::readline::extract_colon_unit;
use crate::readline::full_pathname;
use crate::readline::history_comment_char;
use crate::readline::history_truncate_file;
use crate::readline::history_write_timestamps;
use crate::readline::legal_alias_name;
use crate::readline::legal_identifier;
use crate::readline::posix_initialize;
use crate::readline::rl_basic_word_break_characters;
use crate::readline::rl_completer_word_break_characters;
use crate::readline::rl_reset_terminal;
use crate::readline::save_posix_options;
use crate::readline::stifle_history;
use crate::readline::tzset;
use crate::readline::unstifle_history;
use crate::sig::jump_to_top_level;
use crate::sig::top_level_cleanup;
use crate::src_common::*;
use crate::subst::dequote_escapes;
use crate::subst::expand_assignment_string_to_string;
use crate::subst::invalidate_cached_quoted_dollar_at;
use crate::subst::list_rest_of_args;
use crate::subst::setifs;
use crate::version::build_version;
use crate::version::dist_version;
use crate::version::patch_level;
use crate::version::release_status;
use crate::version::shell_compatibility_level;
use crate::version::shell_version_string;
use crate::y_tab::eof_encountered;
use crate::y_tab::eof_encountered_limit;
use crate::y_tab::ignoreeof;
use crate::y_tab::line_number;
use crate::y_tab::line_number_base;
use std::convert::TryInto;

//add by bgz
#[link(name = "history")]
#[link(name = "readline")]
extern "C" {
    pub fn sh_get_home_dir() -> *mut libc::c_char;
}
//end of bgz

#[no_mangle]
pub fn bind_spcial_variable(
    name: *const libc::c_char,
    mut value: *mut libc::c_char,
) -> *mut libc::c_char {
    unsafe {
        if libc::strcmp(name, b"PS1\0" as *const u8 as *mut libc::c_char) == 0 {
            if value.is_null() {
                if current_user.euid == 0 {
                    value = Root_PS1_Value!();
                } else {
                    value = PS1_Value!();
                }
            }
        }
        return value;
    }
}

#[no_mangle]
pub fn bind_variable(
    name: *const libc::c_char,
    mut value: *mut libc::c_char,
    flags: libc::c_int,
) -> *mut SHELL_VAR {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut nv: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut vc: *mut VAR_CONTEXT = 0 as *mut VAR_CONTEXT;
    let mut nvc: *mut VAR_CONTEXT = 0 as *mut VAR_CONTEXT;
    unsafe {
        value = bind_spcial_variable(name, value);
        if shell_variables.is_null() {
            create_variable_tables();
        }
        if !temporary_env.is_null() && !value.is_null() {
            bind_tempenv_variable(name, value);
        }
        vc = shell_variables;
        while !vc.is_null() {
            if vc_isfuncenv!(vc) || vc_isbltnenv!(vc) {
                v = hash_lookup(name, (*vc).table);
                nvc = vc;

                if !v.is_null() && nameref_p!(v) != 0 {
                    nv = find_variable_nameref_context(v, vc, &mut nvc);
                    if nv.is_null() {
                        nv = find_variable_last_nameref_context(v, vc, &mut nvc);
                        if !nv.is_null() && nameref_p!(nv) != 0 {
                            if nameref_cell!(nv).is_null() {
                                return bind_variable_internal(
                                    (*nv).name,
                                    value,
                                    (*nvc).table,
                                    0 as libc::c_int,
                                    flags,
                                );
                            } else if valid_array_reference(nameref_cell!(nv), 0 as libc::c_int)
                                != 0
                            {
                                return assign_array_element(nameref_cell!(nv), value, flags);
                            } else {
                                return bind_variable_internal(
                                    nameref_cell!(nv),
                                    value,
                                    (*nvc).table,
                                    0 as libc::c_int,
                                    flags,
                                );
                            }
                        } else if nv == &mut nameref_maxloop_value as *mut SHELL_VAR {
                            internal_warning(
                                b"%s: circular name reference\0" as *const u8
                                    as *const libc::c_char,
                                (*v).name,
                            );
                            return bind_global_variable((*v).name, value, flags);
                        } else {
                            v = nv;
                        }
                    } else if nv == &mut nameref_maxloop_value as *mut SHELL_VAR {
                        internal_warning(
                            b"%s: circular name reference\0" as *const u8 as *const libc::c_char,
                            (*v).name,
                        );
                        return bind_global_variable((*v).name, value, flags);
                    } else {
                        v = nv;
                    }
                }

                if !v.is_null() {
                    return bind_variable_internal(
                        (*v).name,
                        value,
                        (*nvc).table,
                        0 as libc::c_int,
                        flags,
                    );
                }
            }

            vc = (*vc).down;
        }

        return bind_variable_internal(
            name,
            value,
            (*global_variables).table,
            0 as libc::c_int,
            flags,
        );
    }
}

fn STREQN(a: *const libc::c_char, b: *const libc::c_char, n: i32) -> bool {
    unsafe {
        if n == 0 {
            return true;
        } else {
            return *a == *b && libc::strncmp(a, b, n as libc::size_t) == 0;
        }
    }
}

fn STREQ(a: *const libc::c_char, b: *const libc::c_char) -> bool {
    unsafe {
        return (*a == *b) && (libc::strcmp(a, b) == 0);
    }
}

static mut nameref_maxloop_value: SHELL_VAR = SHELL_VAR {
    name: 0 as *const libc::c_char as *mut libc::c_char,
    value: 0 as *const libc::c_char as *mut libc::c_char,
    exportstr: 0 as *const libc::c_char as *mut libc::c_char,
    dynamic_value: None,
    assign_func: None,
    attributes: 0,
    context: 0,
};

static mut last_table_searched: *mut HASH_TABLE = 0 as *const HASH_TABLE as *mut HASH_TABLE;

#[inline]
fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    unsafe {
        return strtol(
            __nptr,
            0 as *mut libc::c_void as *mut *mut libc::c_char,
            10 as libc::c_int,
        ) as libc::c_int;
    }
}

fn create_variable_tables() {
    unsafe {
        if shell_variables.is_null() {
            global_variables = new_var_context(
                0 as *mut libc::c_void as *mut libc::c_char,
                0 as libc::c_int,
            );

            shell_variables = global_variables;
            (*shell_variables).scope = 0 as libc::c_int;

            (*shell_variables).table = hash_create(VARIABLES_HASH_BUCKETS!() as libc::c_int);
        }
        if shell_functions.is_null() {
            shell_functions = hash_create(FUNCTIONS_HASH_BUCKETS!() as libc::c_int);
        }
    }
}

#[no_mangle]
pub fn initialize_shell_variables(env: *mut *mut libc::c_char, privmode: libc::c_int) {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp_string: *mut libc::c_char = 0 as *mut libc::c_char;

    let mut c: libc::c_int = 0;
    let mut char_index: libc::c_int = 0;
    let mut string_index: libc::c_int = 0;
    let mut string_length: libc::c_int = 0;
    let mut ro: libc::c_int = 0;

    let mut temp_var: *mut SHELL_VAR;

    create_variable_tables();

    string_index = 0 as libc::c_int;
    unsafe {
        while !env.is_null() && {
            string = *env.offset(string_index as isize);
            string_index += 1;
            !string.is_null()
        } {
            char_index = 0 as libc::c_int;
            name = string;

            loop {
                c = *string as libc::c_int;
                string = string.offset(1);
                // c != 0 && c != '=' as i32 进入循环
                if !(c != 0 && c != '=' as i32) {
                    break;
                }
            }

            if *string.offset(-(1 as libc::c_int) as isize) as libc::c_char == '=' as libc::c_char {
                char_index = (string.offset_from(name) as libc::c_long
                    - 1 as libc::c_int as libc::c_long) as libc::c_int;
            }

            if char_index == 0 as libc::c_int {
                continue;
            }

            *name.offset(char_index as isize) = '\0' as i32 as libc::c_char;
            temp_var = 0 as *mut libc::c_void as *mut SHELL_VAR;

            if privmode == 0 as libc::c_int
                && read_but_dont_execute == 0 as libc::c_int
                && STREQN(BASHFUNC_PREFIX!(), name, BASHFUNC_PREFLEN!() as libc::c_int)
                && STREQ(
                    BASHFUNC_SUFFIX!(),
                    name.offset((char_index - BASHFUNC_SUFFLEN!()) as libc::c_int as isize)
                        as *mut libc::c_char,
                )
                && STREQN(
                    b"() {\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    string,
                    4 as libc::c_int,
                )
            {
                let mut namelen: size_t = 0;
                let mut tname: *mut libc::c_char = 0 as *mut libc::c_char;
                namelen = (char_index - BASHFUNC_PREFLEN!() - BASHFUNC_SUFFLEN!()) as size_t;
                tname = name.offset(BASHFUNC_PREFLEN!() as libc::c_int as isize);
                *tname.offset(namelen as isize) = '\0' as i32 as libc::c_char;

                string_length = strlen(string) as libc::c_int;

                temp_string = libc::malloc(
                    (namelen + 2 as libc::c_int as size_t + string_length as u64) as usize,
                ) as *mut libc::c_char;

                libc::memcpy(
                    temp_string as *mut libc::c_void,
                    tname as *const libc::c_void,
                    namelen as usize,
                );

                *temp_string.offset(namelen as isize) = ' ' as i32 as libc::c_char;

                libc::memcpy(
                    temp_string
                        .offset(namelen as isize)
                        .offset(1 as libc::c_int as isize) as *mut libc::c_void,
                    string as *const libc::c_void,
                    (string_length + 1 as libc::c_int) as usize,
                );
                // absolute_program extern func
                if absolute_program(tname) == 0
                    && (posixly_correct == 0 || legal_identifier(tname) != 0)
                {
                    // parse_and_execute extern func
                    parse_and_execute(
                        temp_string,
                        tname,
                        (SEVAL_NONINT as i32
                            | SEVAL_NOHIST
                            | SEVAL_FUNCDEF as i32
                            | SEVAL_ONECMD as i32) as libc::c_int,
                    );
                } else {
                    free(temp_string as *mut libc::c_void);
                }

                temp_var = find_function(tname);

                if !temp_var.is_null() {
                    VSETATTR!(temp_var, (att_exported | att_imported) as libc::c_int);
                    array_needs_making = 1 as libc::c_int;
                } else {
                    temp_var = bind_invalid_envvar(name, string, 0 as libc::c_int);
                    if !temp_var.is_null() {
                        VSETATTR!(
                            temp_var,
                            (att_exported | att_imported | att_invisible) as libc::c_int
                        );
                        array_needs_making = 1 as libc::c_int;
                    }
                    last_command_exit_value = EXECUTION_FAILURE!();
                    report_error(
                        b"error importing function definition for '%s'\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        tname,
                    );
                }

                *tname.offset(namelen as isize) = *BASHFUNC_SUFFIX!();
            } else {
                ro = 0 as libc::c_int;

                if STREQ(name, b"SHELLOPTS\0" as *const u8 as *const libc::c_char) {
                    temp_var = find_variable(
                        b"SHELLOPTS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                    ro = (!temp_var.is_null() && readonly_p!(temp_var) != 0) as libc::c_int;
                    if !temp_var.is_null() {
                        VUNSETATTR!(temp_var, att_readonly as libc::c_int);
                    }
                }

                // legal_identifier .h 文件里定义
                if legal_identifier(name) != 0 as libc::c_int {
                    //bind_variable  .h 文件里定义
                    temp_var = bind_variable(name, string, 0 as libc::c_int);

                    if !temp_var.is_null() {
                        VSETATTR!(temp_var, (att_exported | att_imported) as libc::c_int);
                        if ro != 0 {
                            VSETATTR!(temp_var, att_readonly as libc::c_int);
                        }
                    }
                } else {
                    temp_var = bind_invalid_envvar(name, string, 0);

                    if !temp_var.is_null() {
                        VSETATTR!(
                            temp_var,
                            (att_exported | att_imported | att_invisible) as libc::c_int
                        );
                    }
                }

                if !temp_var.is_null() {
                    array_needs_making = 1 as libc::c_int;
                }
            }

            *name.offset(char_index as isize) = '=' as i32 as libc::c_char;
            if !temp_var.is_null() && function_p!(temp_var) == 0 {
                CACHE_IMPORTSTR!(temp_var, name);
            }
        }

        set_pwd();

        temp_var = set_if_not(
            b"_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            dollar_vars[0 as libc::c_int as usize],
        );

        dollar_dollar_pid = getpid();
        temp_var = set_if_not(
            b"PATH\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            DEFAULT_PATH_VALUE!(),
        );
        temp_var = set_if_not(
            b"TERM\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"dumb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );

        set_if_not(
            b"PS1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            primary_prompt,
        );

        //set_if_not ("PS1", current_user.euid == 0 ? "# " : primary_prompt);
        set_if_not(
            b"PS2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            secondary_prompt,
        );

        if current_user.euid == 0 {
            // h file aready define
            bind_variable(
                b"PS4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"+ \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0,
            );
        } else {
            set_if_not(
                b"PS4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"+ \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }

        temp_var = bind_variable(
            b"IFS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b" \t\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0,
        );

        setifs(temp_var);
        set_machine_vars();

        if interactive_shell != 0 {
            temp_var = set_if_not(
                b"MAILCHECK\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (if posixly_correct != 0 {
                    b"600\0" as *const u8 as *const libc::c_char
                } else {
                    b"60\0" as *const u8 as *const libc::c_char
                }) as *mut libc::c_char,
            );
            VSETATTR!(temp_var, att_integer);
        }

        initialize_shell_level();
        set_ppid();
        set_argv0();

        temp_var = bind_variable(
            b"OPTIND\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0,
        );
        VSETATTR!(temp_var, att_integer);
        getopts_reset(0 as libc::c_int);
        bind_variable(
            b"OPTERR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0,
        );
        sh_opterr = 1;

        if login_shell == 1 as libc::c_int && posixly_correct == 0 as libc::c_int {
            set_home_var();
        }

        name = get_bash_name();
        temp_var = bind_variable(
            b"BASH\0" as *const u8 as *const libc::c_char,
            name,
            0 as libc::c_int,
        );
        free(name as *mut libc::c_void);

        set_shell_var();
        bind_variable(
            b"BASH_VERSION\0" as *const u8 as *const libc::c_char,
            shell_version_string(),
            0 as libc::c_int,
        );

        make_vers_array();

        if !command_execution_string.is_null() {
            bind_variable(
                b"BASH_EXECUTION_STRING\0" as *const u8 as *const libc::c_char,
                command_execution_string,
                0 as libc::c_int,
            );
        }

        temp_var = find_variable(b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char);
        if temp_var.is_null() {
            temp_var = find_variable(b"POSIX_PEDANTIC\0" as *const u8 as *const libc::c_char);
        }
        if !temp_var.is_null() && imported_p!(temp_var) as libc::c_int != 0 {
            sv_strict_posix((*temp_var).name);
        }

        if remember_on_history != 0 {
            // extern c file
            name = bash_tilde_expand(
                if posixly_correct != 0 {
                    b"~/.sh_history\0" as *const u8 as *const libc::c_char
                } else {
                    // b"~/.bash_history\0" as *const u8 as *const libc::c_char
                    b"~/.utshell_history\0" as *const u8 as *const libc::c_char
                },
                0 as libc::c_int,
            );
            set_if_not(
                b"HISTFILE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name,
            );
            free(name as *mut libc::c_void);
        }

        // extern c file
        seedrand();
        seedrand32();

        if interactive_shell != 0 {
            temp_var = find_variable(b"IGNOREEOF\0" as *const u8 as *const libc::c_char);
            if temp_var.is_null() {
                temp_var = find_variable(b"ignoreeof\0" as *const u8 as *const libc::c_char);
            }
            if !temp_var.is_null() && imported_p!(temp_var) != 0 as libc::c_int {
                sv_ignoreeof((*temp_var).name);
            }
        }

        if interactive_shell != 0 && remember_on_history != 0 {
            sv_history_control(
                b"HISTCONTROL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            sv_histignore(b"HISTIGNORE\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
            sv_histtimefmt(
                b"HISTTIMEFORMAT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }

        uidset();

        temp_var = find_variable(b"BASH_XTRACEFD\0" as *const u8 as *const libc::c_char);
        if !temp_var.is_null() && imported_p!(temp_var) as libc::c_int != 0 {
            sv_xtracefd((*temp_var).name);
        }
    }
    sv_shcompat(b"BASH_COMPAT\0" as *const u8 as *const libc::c_char as *mut libc::c_char);

    sv_funcnest(b"FUNCNEST\0" as *const u8 as *const libc::c_char as *mut libc::c_char);

    initialize_dynamic_variables();
}

fn set_machine_vars() {
    let mut temp_var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    unsafe {
        temp_var = set_if_not(
            b"HOSTTYPE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            get_host_type() as *mut libc::c_char,
        );

        temp_var = set_if_not(
            b"OSTYPE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            get_os_type() as *mut libc::c_char,
        );

        temp_var = set_if_not(
            b"MACHTYPE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            get_mach_type() as *mut libc::c_char,
        );

        temp_var = set_if_not(
            b"HOSTNAME\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            current_host_name,
        );
    }
}

#[no_mangle]
fn set_home_var() {
    let mut temp_var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    temp_var = find_variable(b"HOME\0" as *const u8 as *const libc::c_char);
    unsafe {
        if temp_var.is_null() {
            temp_var = bind_variable(
                b"HOME\0" as *const u8 as *const libc::c_char,
                sh_get_home_dir(),
                0 as libc::c_int,
            );
        }
    }
}

fn set_shell_var() {
    let mut temp_var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;

    temp_var = find_variable(b"SHELL\0" as *const u8 as *const libc::c_char);
    unsafe {
        if temp_var.is_null() {
            if (current_user.shell).is_null() {
                // get_current_user_info  in file of shell.c
                get_current_user_info();
            }
            temp_var = bind_variable(
                b"SHELL\0" as *const u8 as *const libc::c_char,
                current_user.shell,
                0 as libc::c_int,
            );
        }
    }
}

fn get_bash_name() -> *mut libc::c_char {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        if login_shell == 1 as libc::c_int && RELPATH!(shell_name) {
            if (current_user.shell).is_null() {
                // get_current_user_info  in file of shell.c
                get_current_user_info();
            }
            name = savestring!(current_user.shell);
        } else if ABSPATH!(shell_name) {
            name = savestring!(shell_name);
        } else if *shell_name.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
            && *shell_name.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
        {
            let mut cdir: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut len: libc::c_int = 0;

            cdir = get_string_value(b"PWD\0" as *const u8 as *const libc::c_char);
            if !cdir.is_null() {
                len = strlen(cdir) as libc::c_int;
                name = libc::malloc(
                    (len as isize + strlen(shell_name) as isize + 1 as libc::c_int as isize)
                        .try_into()
                        .unwrap(),
                ) as *mut libc::c_char;
                strcpy(name, cdir);
                strcpy(
                    name.offset(len as isize),
                    shell_name.offset(1 as libc::c_int as isize),
                );
            } else {
                name = savestring!(shell_name);
            }
        } else {
            let mut tname: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut s: libc::c_int = 0;
            tname = find_user_command(shell_name);
            if tname.is_null() {
                s = file_status(shell_name);
                if s & FS_EXECABLE as libc::c_int != 0 {
                    tname = make_absolute(
                        shell_name,
                        get_string_value(b"PWD\0" as *const u8 as *const libc::c_char),
                    );
                    if *shell_name as libc::c_int == '.' as i32 {
                        //sh_canonpath in extern file
                        name = sh_canonpath(
                            tname,
                            (PATH_CHECKDOTDOT | PATH_CHECKEXISTS) as libc::c_int,
                        );
                        if name.is_null() {
                            name = tname;
                        } else {
                            free(tname as *mut libc::c_void);
                        }
                    } else {
                        name = tname;
                    }
                } else {
                    if (current_user.shell).is_null() {
                        // get_current_user_info  in file of shell.c
                        get_current_user_info();
                    }
                    name = savestring!(current_user.shell);
                }
            } else {
                name = full_pathname(tname);
                free(tname as *mut libc::c_void);
            }
        }
    }
    return name;
}

#[no_mangle]
pub fn adjust_shell_level(change: libc::c_int) {
    let mut new_level: [libc::c_char; 5] = [0; 5];
    let mut old_SHLVL: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut old_level: intmax_t = 0;
    let mut temp_var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;

    old_SHLVL = get_string_value(b"SHLVL\0" as *const u8 as *const libc::c_char);
    unsafe {
        if old_SHLVL.is_null()
            || *old_SHLVL as libc::c_int == '\0' as i32
            || legal_number(old_SHLVL, &mut old_level) == 0 as libc::c_int
        {
            old_level = 0 as libc::c_int as intmax_t;
        }
        shell_level = (old_level + change as libc::c_long) as libc::c_int;
        if shell_level < 0 as libc::c_int {
            shell_level = 0 as libc::c_int;
        } else if shell_level >= 1000 as libc::c_int {
            internal_warning(
                0 as *const libc::c_char,
                b"shell level (%d) too high, resetting to 1\0" as *const u8 as *const libc::c_char,
                shell_level,
            );
            shell_level = 1 as libc::c_int;
        }
        if shell_level < 10 as libc::c_int {
            new_level[0 as libc::c_int as usize] = (shell_level + '0' as i32) as libc::c_char;
            new_level[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        } else if shell_level < 100 as libc::c_int {
            new_level[0 as libc::c_int as usize] =
                (shell_level / 10 as libc::c_int + '0' as i32) as libc::c_char;
            new_level[1 as libc::c_int as usize] =
                (shell_level % 10 as libc::c_int + '0' as i32) as libc::c_char;
            new_level[2 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        } else if shell_level < 1000 as libc::c_int {
            new_level[0 as libc::c_int as usize] =
                (shell_level / 100 as libc::c_int + '0' as i32) as libc::c_char;
            old_level = (shell_level % 100 as libc::c_int) as intmax_t;
            new_level[1 as libc::c_int as usize] = (old_level / 10 as libc::c_int as libc::c_long
                + '0' as i32 as libc::c_long)
                as libc::c_char;
            new_level[2 as libc::c_int as usize] = (old_level % 10 as libc::c_int as libc::c_long
                + '0' as i32 as libc::c_long)
                as libc::c_char;
            new_level[3 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        }

        temp_var = bind_variable(
            b"SHLVL\0" as *const u8 as *const libc::c_char,
            new_level.as_mut_ptr(),
            0 as libc::c_int,
        );

        set_auto_export!(temp_var);
    }
}

fn initialize_shell_level() {
    adjust_shell_level(1 as libc::c_int);
}

#[no_mangle]
pub fn set_pwd() {
    let mut temp_var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut home_var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut temp_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut home_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut current_dir: *mut libc::c_char = 0 as *mut libc::c_char;
    home_var = find_variable(b"HOME\0" as *const u8 as *const libc::c_char);
    unsafe {
        home_string = if !home_var.is_null() {
            value_cell!(home_var)
        } else {
            0 as *mut libc::c_void as *mut libc::c_char
        };

        temp_var = find_variable(b"PWD\0" as *const u8 as *const libc::c_char);

        if !temp_var.is_null()
            && imported_p!(temp_var) != 0 as libc::c_int
            && {
                temp_string = value_cell!(temp_var);
                !temp_string.is_null()
            }
            && *temp_string as libc::c_int == '/' as i32
            && same_file(
                temp_string,
                b".\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_void as *mut crate::src_common::stat,
                0 as *mut libc::c_void as *mut crate::src_common::stat,
            ) != 0
        {
            // sh_canonpath are extern func
            current_dir = sh_canonpath(
                temp_string,
                PATH_CHECKDOTDOT as libc::c_int | PATH_CHECKEXISTS as libc::c_int,
            );

            if current_dir.is_null() {
                // get_working_directory are extern func
                current_dir = get_working_directory(
                    b"shell_init\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            } else {
                // set_working_directory are extern func
                set_working_directory(current_dir);
            }
            if posixly_correct != 0 && !current_dir.is_null() {
                //bind_variable are extern func
                temp_var = bind_variable(
                    b"PWD\0" as *const u8 as *const libc::c_char,
                    current_dir,
                    0 as libc::c_int,
                );
                set_auto_export!(temp_var);
            }
            free(current_dir as *mut libc::c_void);
        } else if !home_string.is_null()
            && interactive_shell != 0
            && login_shell != 0
            && same_file(
                home_string,
                b".\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_void as *mut crate::src_common::stat,
                0 as *mut libc::c_void as *mut crate::src_common::stat,
            ) != 0
        {
            // set_working_directory are extern func
            set_working_directory(home_string);

            //bind_variable are extern func
            temp_var = bind_variable(
                b"PWD\0" as *const u8 as *const libc::c_char,
                home_string,
                0 as libc::c_int,
            );
            set_auto_export!(temp_var);
        } else {
            // get_working_directory are extern func
            temp_string = get_working_directory(
                b"shell-init\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );

            if !temp_string.is_null() {
                temp_var = bind_variable(
                    b"PWD\0" as *const u8 as *const libc::c_char,
                    temp_string,
                    0 as libc::c_int,
                );
                set_auto_export!(temp_var);
                free(temp_string as *mut libc::c_void);
            }
        }

        temp_var = find_variable(b"OLDPWD\0" as *const u8 as *const libc::c_char);
        if temp_var.is_null()
            || value_cell!(temp_var).is_null()
            || file_isdir(value_cell!(temp_var)) == 0 as libc::c_int
        {
            temp_var = bind_variable(
                b"OLDPWD\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_char,
                0 as libc::c_int,
            );

            VSETATTR!(temp_var, (att_exported | att_invisible));
        }
    }
}

#[no_mangle]
pub fn set_ppid() {
    // 暂时定义的宏 有问题 还得修改
    let mut namebuf: [libc::c_char; (INT_STRLEN_BOUND!(uid_t) + 1) as usize] =
        [0; (INT_STRLEN_BOUND!(uid_t) + 1) as usize];
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp_var: *mut SHELL_VAR;
    unsafe {
        name = inttostr(
            getppid() as intmax_t,
            namebuf.as_mut_ptr(),
            std::mem::size_of::<[libc::c_char; (INT_STRLEN_BOUND!(uid_t) + 1) as usize]>()
                as libc::c_ulong as size_t,
        );
        temp_var = find_variable(b"PPID\0" as *const u8 as *const libc::c_char);
        if !temp_var.is_null() {
            VUNSETATTR!(temp_var, (att_readonly | att_exported));
        }

        temp_var = bind_variable(
            b"PPID\0" as *const u8 as *const libc::c_char,
            name,
            0 as libc::c_int,
        );

        VSETATTR!(temp_var, (att_readonly | att_integer));
    }
}

fn uidset() {
    // INT_STRLEN_BOUND(uid_t) + 1
    let mut buff: [libc::c_char; (INT_STRLEN_BOUND!(uid_t) + 1) as usize] =
        [0; (INT_STRLEN_BOUND!(uid_t) + 1) as usize];
    let mut b: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    unsafe {
        b = inttostr(
            current_user.uid as intmax_t,
            buff.as_mut_ptr(),
            std::mem::size_of::<[libc::c_char; (INT_STRLEN_BOUND!(uid_t) + 1) as usize]>()
                as libc::c_ulong as size_t,
        );
        v = find_variable(b"UID\0" as *const u8 as *const libc::c_char);
        if v.is_null() {
            v = bind_variable(
                b"UID\0" as *const u8 as *const libc::c_char,
                b,
                0 as libc::c_int,
            );
            VSETATTR!(v, (att_readonly | att_integer) as libc::c_int);
        }
        if current_user.euid != current_user.uid {
            b = inttostr(
                current_user.euid as intmax_t,
                buff.as_mut_ptr(),
                std::mem::size_of::<[libc::c_char; (INT_STRLEN_BOUND!(uid_t) + 1) as usize]>()
                    as libc::c_ulong as size_t,
            );
        }
        v = find_variable(b"EUID\0" as *const u8 as *const libc::c_char);
        if v.is_null() {
            v = bind_variable(
                b"EUID\0" as *const u8 as *const libc::c_char,
                b,
                0 as libc::c_int,
            );
            VSETATTR!(v, (att_readonly | att_integer));
        }
    }
}

fn make_vers_array() {
    let mut vv: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut av: *mut ARRAY = 0 as *mut ARRAY;

    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut d: [libc::c_char; 32] = [0; 32];

    const b: [libc::c_char; (INT_STRLEN_BOUND!(libc::c_int) + 1) as usize] =
        [0 as libc::c_char; (INT_STRLEN_BOUND!(libc::c_int) + 1) as usize];
    unbind_variable_noref(b"BASH_VERSINFO\0" as *const u8 as *const libc::c_char);

    vv = make_new_array_variable(
        b"BASH_VERSINFO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    unsafe {
        av = array_cell!(vv);
        strcpy(d.as_mut_ptr(), dist_version);
        s = libc::strchr(d.as_mut_ptr(), '.' as i32);
        if !s.is_null() {
            *s = '\0' as i32 as libc::c_char;
            s = s.offset(1 as isize);
        }
        array_insert(av, 0 as libc::c_int as arrayind_t, d.as_mut_ptr());
        array_insert(av, 1 as libc::c_int as arrayind_t, s);

        s = inttostr(
            patch_level as intmax_t,
            b.as_mut_ptr(),
            std::mem::size_of::<[libc::c_char; (INT_STRLEN_BOUND!(libc::c_int) + 1) as usize]>()
                as libc::c_ulong as u64,
        );
        array_insert(av, 2 as libc::c_int as arrayind_t, s);
        s = inttostr(
            build_version as intmax_t,
            b.as_mut_ptr(),
            std::mem::size_of::<[libc::c_char; (INT_STRLEN_BOUND!(libc::c_int) + 1) as usize]>()
                as libc::c_ulong as u64,
        );
        array_insert(av, 3 as libc::c_int as arrayind_t, s);
        array_insert(
            av,
            4 as libc::c_int as arrayind_t,
            release_status as *mut libc::c_char,
        );
        array_insert(av, 5 as libc::c_int as arrayind_t, get_mach_type());
        VSETATTR!(vv, att_readonly);
    }
}

#[no_mangle]
pub fn sh_set_lines_and_columns_out(lines: libc::c_int, cols: libc::c_int) {
    let mut val: [libc::c_char; (INT_STRLEN_BOUND!(libc::c_int) + 1) as usize] =
        [0 as libc::c_char; (INT_STRLEN_BOUND!(libc::c_int) + 1) as usize];

    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        if winsize_assignment != 0 {
            return;
        }

        v = inttostr(
            lines as intmax_t,
            val.as_mut_ptr(),
            std::mem::size_of::<[libc::c_char; (INT_STRLEN_BOUND!(libc::c_int) + 1) as usize]>()
                as libc::c_ulong as u64,
        );

        bind_variable(
            b"LINES\0" as *const u8 as *const libc::c_char,
            v,
            0 as libc::c_int,
        );
        v = inttostr(
            cols as intmax_t,
            val.as_mut_ptr(),
            std::mem::size_of::<[libc::c_char; (INT_STRLEN_BOUND!(libc::c_int) + 1) as usize]>()
                as libc::c_ulong as u64,
        );
        bind_variable(
            b"COLUMNS\0" as *const u8 as *const libc::c_char,
            v,
            0 as libc::c_int,
        );
    }
}

#[no_mangle]
pub fn print_var_list(list: *mut *mut SHELL_VAR) {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    unsafe {
        while !list.is_null() && {
            var = *list.offset(i as isize);
            !var.is_null()
        } {
            if invisible_p!(var) == 0 {
                print_assignment(var);
            }
            i += 1;
        }
    }
}

#[no_mangle]
pub fn print_func_list(list: *mut *mut SHELL_VAR) {
    let mut i: libc::c_int = 0;
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    i = 0 as libc::c_int;
    unsafe {
        while !list.is_null() && {
            var = *list.offset(i as isize);
            !var.is_null()
        } {
            libc::printf(b"%s \0" as *const u8 as *const libc::c_char, (*var).name);
            print_var_function(var);
            libc::printf(b"\n\0" as *const u8 as *const libc::c_char);
            i += 1;
        }
    }
}

#[no_mangle]
pub fn print_assignment(var: *mut SHELL_VAR) {
    unsafe {
        if !var_isset!(var) {
            return;
        }
        if function_p!(var) != 0 {
            libc::printf(b"%s\0" as *const u8 as *const libc::c_char, (*var).name);
            print_var_function(var);
            libc::printf(b"\n\0" as *const u8 as *const libc::c_char);
        } else if array_p!(var) != 0 {
            print_array_assignment(var, 0 as libc::c_int);
        } else if assoc_p!(var) != 0 {
            print_assoc_assignment(var, 0 as libc::c_int);
        } else {
            libc::printf(b"%s=\0" as *const u8 as *const libc::c_char, (*var).name);
            print_var_value(var, 1 as libc::c_int);
            libc::printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
    }
}

pub fn print_var_value(var: *mut SHELL_VAR, quote: libc::c_int) {
    unsafe {
        let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
        if !var_isset!(var) {
            return;
        }
        if quote != 0
            && posixly_correct == 0 as libc::c_int
            && ansic_shouldquote(value_cell!(var)) != 0
        {
            t = ansic_quote(value_cell!(var), 0 as libc::c_int, 0 as *mut libc::c_int);
            libc::printf(b"%s\0" as *const u8 as *const libc::c_char, t);
            if !t.is_null() {
                libc::free(t as *mut libc::c_void);
            }
        } else if quote != 0 && sh_contains_shell_metas(value_cell!(var)) != 0 {
            t = sh_single_quote(value_cell!(var));
            libc::printf(b"%s\0" as *const u8 as *const libc::c_char, t);
            if !t.is_null() {
                libc::free(t as *mut libc::c_void);
            }
        } else {
            libc::printf(
                b"%s\0" as *const u8 as *const libc::c_char,
                value_cell!(var),
            );
        };
    }
}

#[no_mangle]
pub fn print_var_function(var: *mut SHELL_VAR) {
    unsafe {
        let mut x: *mut libc::c_char = 0 as *mut libc::c_char;
        if function_p!(var) != 0 as libc::c_int && var_isset!(var) {
            x = named_function_string(
                0 as *mut libc::c_void as *mut libc::c_char,
                function_cell!(var),
                (FUNC_MULTILINE | FUNC_EXTERNAL) as libc::c_int,
            );
            libc::printf(b"%s\0" as *const u8 as *const libc::c_char, x);
        }
    }
}

fn null_assign(
    self_0: *mut SHELL_VAR,
    value: *mut libc::c_char,
    unused: arrayind_t,
    key: *mut libc::c_char,
) -> *mut SHELL_VAR {
    return self_0;
}

fn null_array_assign(
    self_0: *mut SHELL_VAR,
    value: *mut libc::c_char,
    ind: arrayind_t,
    key: *mut libc::c_char,
) -> *mut SHELL_VAR {
    return self_0;
}

fn get_self(self_0: *mut SHELL_VAR) -> *mut SHELL_VAR {
    return self_0;
}

fn init_dynamic_array_var(
    name: *mut libc::c_char,
    getfunc: Option<sh_var_value_func_t>,
    setfunc: Option<sh_var_assign_func_t>,
    attrs: libc::c_int,
) -> *mut SHELL_VAR {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    v = find_variable(name);
    if !v.is_null() {
        return v;
    }
    unsafe {
        INIT_DYNAMIC_ARRAY_VAR!(v, name, getfunc, setfunc);
        if attrs != 0 {
            VSETATTR!(v, attrs);
        }
    }
    return v;
}

fn init_dynamic_assoc_var(
    name: *mut libc::c_char,
    getfunc: Option<sh_var_value_func_t>,
    setfunc: Option<sh_var_assign_func_t>,
    attrs: libc::c_int,
) -> *mut SHELL_VAR {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    v = find_variable(name);
    if !v.is_null() {
        return v;
    }
    unsafe {
        INIT_DYNAMIC_ASSOC_VAR!(v, name, getfunc, setfunc);

        if attrs != 0 {
            VSETATTR!(v, attrs);
        }
    }
    return v;
}

static mut seconds_value_assigned: intmax_t = 0;

fn assign_seconds(
    self_0: *mut SHELL_VAR,
    value: *mut libc::c_char,
    unused: arrayind_t,
    key: *mut libc::c_char,
) -> *mut SHELL_VAR {
    let mut nval: intmax_t = 0;
    let mut expok: libc::c_int = 0;
    unsafe {
        if integer_p!(self_0) != 0 as libc::c_int {
            nval = evalexp(value, 0 as libc::c_int, &mut expok);
        } else {
            expok = legal_number(value, &mut nval);
        }
        seconds_value_assigned = if expok != 0 {
            nval
        } else {
            0 as libc::c_int as libc::c_long
        };
        gettimeofday(&mut shellstart, 0 as *mut crate::src_common::timezone);
        shell_start_time = shellstart.tv_sec;
    }
    return self_0;
}

fn get_seconds(var: *mut SHELL_VAR) -> *mut SHELL_VAR {
    let mut time_since_start: time_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tv: crate::src_common::timeval = crate::src_common::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    unsafe {
        gettimeofday(&mut tv, 0 as *mut crate::src_common::timezone);
        time_since_start = tv.tv_sec - shell_start_time;

        p = itos(seconds_value_assigned + time_since_start);
        if !(value_cell!(var).is_null()) {
            libc::free(value_cell!(var) as *mut libc::c_void);
        }
        VSETATTR!(var, att_integer);
        var_setvalue!(var, p);
    }
    return var;
}

fn init_seconds_var() -> *mut SHELL_VAR {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;

    v = find_variable(b"SECONDS\0" as *const u8 as *const libc::c_char);
    unsafe {
        if !v.is_null() {
            if legal_number(value_cell!(v), &mut seconds_value_assigned) == 0 as libc::c_int {
                seconds_value_assigned = 0 as libc::c_int as intmax_t;
            }
        }
        if !v.is_null() {
            INIT_DYNAMIC_VAR!(
                v,
                b"SECONDS\0" as *const u8 as *const libc::c_char,
                value_cell!(v),
                Some(get_seconds),
                Some(assign_seconds)
            );
        } else {
            INIT_DYNAMIC_VAR!(
                v,
                b"SECONDS\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_char,
                Some(get_seconds),
                Some(assign_seconds)
            );
        }
    }
    return v;
}

#[no_mangle]
pub static mut last_random_value: libc::c_int = 0;
static mut seeded_subshell: libc::c_int = 0 as libc::c_int;

fn assign_random(
    self_0: *mut SHELL_VAR,
    value: *mut libc::c_char,
    unused: arrayind_t,
    key: *mut libc::c_char,
) -> *mut SHELL_VAR {
    let mut seedval: intmax_t = 0;
    let mut expok: libc::c_int = 0;
    unsafe {
        if integer_p!(self_0) != 0 {
            seedval = evalexp(value, 0 as libc::c_int, &mut expok);
        } else {
            expok = legal_number(value, &mut seedval);
        }
        if expok == 0 as libc::c_int {
            return self_0;
        }
        sbrand(seedval as libc::c_ulong);
        if subshell_environment != 0 {
            seeded_subshell = getpid();
        }
    }
    return self_0;
}

#[no_mangle]
pub fn get_random_number() -> libc::c_int {
    let mut rv: libc::c_int = 0;
    let mut pid: libc::c_int = 0;
    unsafe {
        pid = getpid();
        if subshell_environment != 0 && seeded_subshell != pid {
            seedrand();
            seeded_subshell = pid;
        }
        loop {
            rv = brand();
            if rv == last_random_value {
                break;
            }
        }
        last_random_value = rv;
        return last_random_value;
    }
}

fn get_random(var: *mut SHELL_VAR) -> *mut SHELL_VAR {
    let mut rv: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    rv = get_random_number();
    unsafe {
        p = itos(rv as intmax_t);
        if !(value_cell!(var).is_null()) {
            free(value_cell!(var) as *mut libc::c_void);
        }

        VSETATTR!(var, att_integer);
        var_setvalue!(var, p);
    }
    return var;
}

#[no_mangle]
pub fn get_urandom(var: *mut SHELL_VAR) -> *mut SHELL_VAR {
    let mut rv: libc::c_uint = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        rv = get_urandom32();
        p = itos(rv as intmax_t);
        if !(value_cell!(var)).is_null() {
            libc::free(value_cell!(var) as *mut libc::c_void);
        }
        VSETATTR!(var, att_integer);
        var_setvalue!(var, p);
    }
    return var;
}

fn assign_lineno(
    var: *mut SHELL_VAR,
    value: *mut libc::c_char,
    unused: arrayind_t,
    key: *mut libc::c_char,
) -> *mut SHELL_VAR {
    let mut new_value: intmax_t = 0;
    unsafe {
        if value.is_null()
            || *value as libc::c_int == '\0' as i32
            || legal_number(value, &mut new_value) == 0 as libc::c_int
        {
            new_value = 0 as libc::c_int as intmax_t;
        }
        line_number_base = new_value as libc::c_int;
        line_number = line_number_base;
    }
    return var;
}

fn get_lineno(var: *mut SHELL_VAR) -> *mut SHELL_VAR {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ln: libc::c_int = 0;
    ln = executing_line_number();
    unsafe {
        p = itos(ln as intmax_t);
        if !(value_cell!(var).is_null()) {
            free(value_cell!(var) as *mut libc::c_void);
        }
        var_setvalue!(var, p);
    }
    return var;
}

fn assign_subshell(
    var: *mut SHELL_VAR,
    value: *mut libc::c_char,
    unused: arrayind_t,
    key: *mut libc::c_char,
) -> *mut SHELL_VAR {
    let mut new_value: intmax_t = 0;
    unsafe {
        if value.is_null()
            || *value as libc::c_int == '\0' as i32
            || legal_number(value, &mut new_value) == 0 as libc::c_int
        {
            new_value = 0 as libc::c_int as intmax_t;
        }
        subshell_level = new_value as libc::c_int;
    }
    return var;
}

fn get_subshell(var: *mut SHELL_VAR) -> *mut SHELL_VAR {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        p = itos(subshell_level as intmax_t);
        if !(value_cell!(var).is_null()) {
            libc::free(value_cell!(var) as *mut libc::c_void);
        }
        var_setvalue!(var, p);
    }
    return var;
}

fn get_epochseconds(var: *mut SHELL_VAR) -> *mut SHELL_VAR {
    let mut now: intmax_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        now = NOW!();
        p = itos(now);
        if !(value_cell!(var).is_null()) {
            libc::free(value_cell!(var) as *mut libc::c_void);
        }
        var_setvalue!(var, p);
    }
    return var;
}

fn get_epochrealtime(var: *mut SHELL_VAR) -> *mut SHELL_VAR {
    let mut buf: [libc::c_char; 32] = [0; 32];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tv: crate::src_common::timeval = crate::src_common::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    unsafe {
        gettimeofday(&mut tv, 0 as *mut crate::src_common::timezone);
        libc::snprintf(
            buf.as_mut_ptr(),
            std::mem::size_of::<[libc::c_char; 32]>() as usize,
            b"%u%c%06u\0" as *const u8 as *const libc::c_char,
            tv.tv_sec as libc::c_uint,
            locale_decpoint(),
            tv.tv_usec as libc::c_uint,
        );
        p = savestring!(&mut buf[0]);
        if !(value_cell!(var).is_null()) {
            libc::free(value_cell!(var) as *mut libc::c_void);
        }
        var_setvalue!(var, p);
    }
    return var;
}

fn get_bashpid(var: *mut SHELL_VAR) -> *mut SHELL_VAR {
    let mut pid: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        pid = getpid();
        p = itos(pid as intmax_t);
        if !(value_cell!(var).is_null()) {
            free(value_cell!(var) as *mut libc::c_void);
        }
        var_setvalue!(var, p);
    }
    return var;
}

fn get_bash_argv0(var: *mut SHELL_VAR) -> *mut SHELL_VAR {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        p = savestring!(dollar_vars[0]);
        if !(value_cell!(var).is_null()) {
            free(value_cell!(var) as *mut libc::c_void);
        }
        var_setvalue!(var, p);
    }
    return var;
}

static mut static_shell_name: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;

fn assign_bash_argv0(
    var: *mut SHELL_VAR,
    value: *mut libc::c_char,
    unused: arrayind_t,
    key: *mut libc::c_char,
) -> *mut SHELL_VAR {
    let mut vlen: size_t = 0;
    if value.is_null() {
        return var;
    }
    unsafe {
        if !(dollar_vars[0 as libc::c_int as usize]).is_null() {
            libc::free(dollar_vars[0 as libc::c_int as usize] as *mut libc::c_void);
        }
        dollar_vars[0 as libc::c_int as usize] = 0 as *mut libc::c_char;
        dollar_vars[0 as libc::c_int as usize] = savestring!(value);

        vlen = STRLEN!(value) as size_t;
        static_shell_name = libc::realloc(
            static_shell_name as *mut libc::c_void,
            (vlen + 1) as libc::c_int as usize,
        ) as *mut libc::c_char;
        strcpy(static_shell_name, value);
        shell_name = static_shell_name;
    }
    return var;
}

fn set_argv0() {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    v = find_variable(b"BASH_ARGV0\0" as *const u8 as *const libc::c_char);
    unsafe {
        if !v.is_null() && imported_p!(v) != 0 {
            assign_bash_argv0(
                v,
                value_cell!(v),
                0 as libc::c_int as arrayind_t,
                0 as *mut libc::c_char,
            );
        }
    }
}

fn get_bash_command(var: *mut SHELL_VAR) -> *mut SHELL_VAR {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        if !the_printed_command_except_trap.is_null() {
            p = savestring!(the_printed_command_except_trap);
        } else {
            p = libc::malloc(1 as libc::c_int as usize) as *mut libc::c_char;
            *p = '\0' as i32 as libc::c_char;
        }
        if !(value_cell!(var).is_null()) {
            free(value_cell!(var) as *mut libc::c_void);
        }

        var_setvalue!(var, p);
    }
    return var;
}

fn get_histcmd(var: *mut SHELL_VAR) -> *mut SHELL_VAR {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0;
    unsafe {
        n = history_number() - executing;
        p = itos(n as intmax_t);
        if !(value_cell!(var).is_null()) {
            free(value_cell!(var) as *mut libc::c_void);
        }
        var_setvalue!(var, p);
    }
    return var;
}

fn get_comp_wordbreaks(var: *mut SHELL_VAR) -> *mut SHELL_VAR {
    unsafe {
        if rl_completer_word_break_characters.is_null()
            && bash_readline_initialized == 0 as libc::c_int
        {
            enable_hostname_completion(perform_hostname_completion);
        }
        if !(value_cell!(var).is_null()) {
            free(value_cell!(var) as *mut libc::c_void);
        }
        var_setvalue!(var, savestring!(rl_completer_word_break_characters));
    }
    return var;
}

fn assign_comp_wordbreaks(
    self_0: *mut SHELL_VAR,
    value: *mut libc::c_char,
    unused: arrayind_t,
    key: *mut libc::c_char,
) -> *mut SHELL_VAR {
    unsafe {
        if !rl_completer_word_break_characters.is_null()
            && rl_completer_word_break_characters
                != rl_basic_word_break_characters as *mut libc::c_char
        {
            free(rl_completer_word_break_characters as *mut libc::c_void);
        }
        rl_completer_word_break_characters = savestring!(value);
    }
    return self_0;
}

fn assign_dirstack(
    self_0: *mut SHELL_VAR,
    value: *mut libc::c_char,
    ind: arrayind_t,
    key: *mut libc::c_char,
) -> *mut SHELL_VAR {
    set_dirstack_element(ind, 1 as libc::c_int, value);
    return self_0;
}

fn get_dirstack(self_0: *mut SHELL_VAR) -> *mut SHELL_VAR {
    let mut a: *mut ARRAY = 0 as *mut ARRAY;
    let mut l: *mut WORD_LIST = 0 as *mut WORD_LIST;
    l = get_directory_stack(0 as libc::c_int);
    unsafe {
        a = array_from_word_list(l);
        array_dispose((*self_0).value as *mut ARRAY);
        dispose_words(l);

        var_setarray!(self_0, a);
    }
    return self_0;
}

fn get_groupset(self_0: *mut SHELL_VAR) -> *mut SHELL_VAR {
    let mut i: libc::c_int = 0;
    let mut ng: libc::c_int = 0;
    let mut a: *mut ARRAY = 0 as *mut ARRAY;
    static mut group_set: *mut *mut libc::c_char =
        0 as *const libc::c_void as *mut libc::c_void as *mut *mut libc::c_char;
    unsafe {
        if group_set.is_null() {
            group_set = get_group_list(&mut ng);
            a = array_cell!(self_0);
            i = 0 as libc::c_int;
            while i < ng {
                array_insert(a, i as arrayind_t, *group_set.offset(i as isize));
                i += 1;
                i;
            }
        }
    }
    return self_0;
}

fn get_bashargcv(self_0: *mut SHELL_VAR) -> *mut SHELL_VAR {
    static mut self_semaphore: libc::c_int = 0 as libc::c_int;
    unsafe {
        if self_semaphore == 0 as libc::c_int
            && variable_context == 0 as libc::c_int
            && debugging_mode == 0 as libc::c_int
        {
            self_semaphore = 1 as libc::c_int;
            init_bash_argv();
            self_semaphore = 0 as libc::c_int;
        }
    }
    return self_0;
}

fn build_hashcmd(self_0: *mut SHELL_VAR) -> *mut SHELL_VAR {
    let mut h: *mut HASH_TABLE = 0 as *mut HASH_TABLE;
    let mut i: libc::c_int = 0;
    let mut k: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut item: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    unsafe {
        h = assoc_cell!(self_0);

        if !h.is_null() {
            assoc_dispose(h);
        }

        if hashed_filenames.is_null() || HASH_ENTRIES!(hashed_filenames) == 0 {
            var_setvalue!(self_0, 0 as *mut libc::c_char);
            return self_0;
        }

        h = assoc_create!((*hashed_filenames).nbuckets);

        i = 0 as libc::c_int;

        while i < (*hashed_filenames).nbuckets {
            item = hash_items!(i, hashed_filenames);
            while !item.is_null() {
                k = savestring!((*item).key);
                v = (*((*item).data as *mut PATH_DATA)).path;
                assoc_insert(h, k, v);
                item = (*item).next;
            }
            i += 1;
            i;
        }
        var_setvalue!(self_0, h as *mut libc::c_char);
    }
    return self_0;
}

fn get_hashcmd(self_0: *mut SHELL_VAR) -> *mut SHELL_VAR {
    build_hashcmd(self_0);
    return self_0;
}

fn assign_hashcmd(
    self_0: *mut SHELL_VAR,
    value: *mut libc::c_char,
    ind: arrayind_t,
    key: *mut libc::c_char,
) -> *mut SHELL_VAR {
    let mut full_path: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        if restricted != 0 {
            if !(libc::strchr(value, '/' as i32).is_null()) {
                sh_restricted(value);
                return 0 as *mut libc::c_void as *mut SHELL_VAR;
            }
            full_path = find_user_command(value);
            if full_path.is_null()
                || *full_path as libc::c_int == 0 as libc::c_int
                || executable_file(full_path) == 0 as libc::c_int
            {
                sh_notfound(value);
                libc::free(full_path as *mut libc::c_void);
                return 0 as *mut libc::c_void as *mut SHELL_VAR;
            }
            libc::free(full_path as *mut libc::c_void);
        }
        phash_insert(key, value, 0 as libc::c_int, 0 as libc::c_int);
    }
    return build_hashcmd(self_0);
}

fn build_aliasvar(self_0: *mut SHELL_VAR) -> *mut SHELL_VAR {
    let mut h: *mut HASH_TABLE = 0 as *mut HASH_TABLE;
    let mut i: libc::c_int = 0;
    let mut k: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut item: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    unsafe {
        h = assoc_cell!(self_0);

        if !h.is_null() {
            assoc_dispose(h);
        }
        if aliases.is_null() || HASH_ENTRIES!(aliases) == 0 {
            var_setvalue!(self_0, 0 as *mut libc::c_void as *mut libc::c_char);
            return self_0;
        }
        h = assoc_create!((*aliases).nbuckets);

        i = 0 as libc::c_int;
        while i < (*aliases).nbuckets {
            item = if !aliases.is_null() && i < (*aliases).nbuckets {
                *((*aliases).bucket_array).offset(i as isize)
            } else {
                0 as *mut libc::c_void as *mut BUCKET_CONTENTS
            };
            while !item.is_null() {
                k = savestring!((*item).key);
                v = (*((*item).data as *mut alias_t)).value;
                assoc_insert(h, k, v);
                item = (*item).next;
            }
            i += 1;
            i;
        }
        var_setvalue!(self_0, h as *mut libc::c_char);
    }
    return self_0;
}

fn get_aliasvar(self_0: *mut SHELL_VAR) -> *mut SHELL_VAR {
    build_aliasvar(self_0);
    return self_0;
}

fn assign_aliasvar(
    self_0: *mut SHELL_VAR,
    value: *mut libc::c_char,
    ind: arrayind_t,
    key: *mut libc::c_char,
) -> *mut SHELL_VAR {
    unsafe {
        if legal_alias_name(key, 0 as libc::c_int) == 0 as libc::c_int {
            report_error(
                b"'%s': invalid alias name\0" as *const u8 as *const libc::c_char,
                key,
            );
            return self_0;
        }

        add_alias(key, value);
        return build_aliasvar(self_0);
    }
}

fn get_funcname(self_0: *mut SHELL_VAR) -> *mut SHELL_VAR {
    return self_0;
}

#[no_mangle]
pub fn make_funcname_visible(on_or_off: libc::c_int) {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    v = find_variable(b"FUNCNAME\0" as *const u8 as *const libc::c_char);
    unsafe {
        if v.is_null() || !((*v).dynamic_value).is_some() {
            return;
        }
        if on_or_off != 0 {
            VUNSETATTR!(v, att_invisible);
        } else {
            VSETATTR!(v, att_invisible);
        };
    }
}

fn init_funcname_var() -> *mut SHELL_VAR {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    v = find_variable(b"FUNCNAME\0" as *const u8 as *const libc::c_char);
    if !v.is_null() {
        return v;
    }
    unsafe {
        INIT_DYNAMIC_ARRAY_VAR!(
            v,
            b"FUNCNAME\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            Some(get_funcname),
            Some(null_array_assign)
        );

        VSETATTR!(v, att_invisible | att_noassign);
    }
    return v;
}

fn initialize_dynamic_variables() {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;

    v = init_seconds_var();
    unsafe {
        INIT_DYNAMIC_VAR!(
            v,
            b"BASH_ARGV0\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_char,
            Some(get_bash_argv0),
            Some(assign_bash_argv0)
        );

        INIT_DYNAMIC_VAR!(
            v,
            b"BASH_COMMAND\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_char,
            Some(get_bash_command),
            std::mem::transmute::<*mut libc::c_void, Option<sh_var_assign_func_t>>(
                0 as *mut libc::c_void
            )
        );

        INIT_DYNAMIC_VAR!(
            v,
            b"BASH_SUBSHELL\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_char,
            Some(get_subshell),
            Some(assign_subshell)
        );

        INIT_DYNAMIC_VAR!(
            v,
            b"RANDOM\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_char,
            Some(get_random),
            Some(assign_random)
        );

        VSETATTR!(v, att_integer);

        INIT_DYNAMIC_VAR!(
            v,
            b"SRANDOM\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_char,
            Some(get_urandom),
            std::mem::transmute::<*mut libc::c_void, Option<sh_var_assign_func_t>>(
                0 as *mut libc::c_void
            )
        );

        VSETATTR!(v, att_integer);

        INIT_DYNAMIC_VAR!(
            v,
            b"LINENO\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_char,
            Some(get_lineno),
            Some(assign_lineno)
        );

        VSETATTR!(v, att_regenerate);

        INIT_DYNAMIC_VAR!(
            v,
            b"BASHPID\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_char,
            Some(get_bashpid),
            Some(null_assign)
        );

        VSETATTR!(v, att_integer);

        INIT_DYNAMIC_VAR!(
            v,
            b"EPOCHSECONDS\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_char,
            Some(get_epochseconds),
            Some(null_assign)
        );

        VSETATTR!(v, att_regenerate);

        INIT_DYNAMIC_VAR!(
            v,
            b"EPOCHREALTIME\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_char,
            Some(get_epochrealtime),
            Some(null_assign)
        );

        VSETATTR!(v, att_regenerate);

        INIT_DYNAMIC_VAR!(
            v,
            b"HISTCMD\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_char,
            Some(get_histcmd),
            std::mem::transmute::<*mut libc::c_void, Option<sh_var_assign_func_t>>(
                0 as *mut libc::c_void
            )
        );

        VSETATTR!(v, att_integer);

        INIT_DYNAMIC_VAR!(
            v,
            b"COMP_WORDBREAKS\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_char,
            Some(get_comp_wordbreaks),
            Some(assign_comp_wordbreaks)
        );

        v = init_dynamic_array_var(
            b"DIRSTACK\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            Some(get_dirstack),
            Some(assign_dirstack),
            0 as libc::c_int,
        );

        v = init_dynamic_array_var(
            b"GROUPS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            Some(get_groupset),
            Some(null_array_assign),
            att_noassign as libc::c_int,
        );

        v = init_dynamic_array_var(
            b"BASH_ARGC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            Some(get_bashargcv),
            Some(null_array_assign),
            (att_noassign | att_nounset as i32) as libc::c_int,
        );

        v = init_dynamic_array_var(
            b"BASH_ARGV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            Some(get_bashargcv),
            Some(null_array_assign),
            (att_noassign | att_nounset as i32) as libc::c_int,
        );

        v = init_dynamic_array_var(
            b"BASH_SOURCE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            Some(get_self),
            Some(null_array_assign),
            (att_noassign | att_nounset as i32) as libc::c_int,
        );

        v = init_dynamic_array_var(
            b"BASH_LINENO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            Some(get_self),
            Some(null_array_assign),
            (att_noassign | att_nounset as i32) as libc::c_int,
        );

        v = init_dynamic_assoc_var(
            b"BASH_CMDS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            Some(get_hashcmd),
            Some(assign_hashcmd),
            att_nofree as libc::c_int,
        );

        v = init_dynamic_assoc_var(
            b"BASH_ALIASES\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            Some(get_aliasvar),
            Some(assign_aliasvar),
            att_nofree as libc::c_int,
        );
        v = init_funcname_var();
    }
}

fn hash_lookup(name: *const libc::c_char, hashed_vars: *mut HASH_TABLE) -> *mut SHELL_VAR {
    let mut bucket: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    bucket = hash_search(name, hashed_vars, 0 as libc::c_int);
    unsafe {
        if !bucket.is_null() {
            last_table_searched = hashed_vars;
        }
        return if !bucket.is_null() {
            (*bucket).data as *mut SHELL_VAR
        } else {
            0 as *mut libc::c_void as *mut SHELL_VAR
        };
    }
}

#[no_mangle]
pub fn var_lookup(name: *const libc::c_char, vcontext: *mut VAR_CONTEXT) -> *mut SHELL_VAR {
    let mut vc: *mut VAR_CONTEXT = 0 as *mut VAR_CONTEXT;
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    v = 0 as *mut libc::c_void as *mut SHELL_VAR;
    vc = vcontext;
    unsafe {
        while !vc.is_null() {
            v = hash_lookup(name, (*vc).table);
            if !v.is_null() {
                break;
            }
            vc = (*vc).down;
        }
    }
    return v;
}

fn find_variable_internal(name: *const libc::c_char, flags: libc::c_int) -> *mut SHELL_VAR {
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut search_tempenv: libc::c_int = 0;
    let mut force_tempenv: libc::c_int = 0;
    let mut vc: *mut VAR_CONTEXT = 0 as *mut VAR_CONTEXT;

    var = 0 as *mut libc::c_void as *mut SHELL_VAR;
    unsafe {
        force_tempenv = flags & FV_FORCETEMPENV!();
        search_tempenv = (force_tempenv != 0
            || (expanding_redir == 0 as libc::c_int && subshell_environment != 0))
            as libc::c_int;
        if search_tempenv != 0 && !temporary_env.is_null() {
            var = hash_lookup(name, temporary_env);
        }
        if var.is_null() {
            if (flags & FV_SKIPINVISIBLE!()) == 0 as libc::c_int {
                var = var_lookup(name, shell_variables);
            } else {
                vc = shell_variables;
                while !vc.is_null() {
                    var = hash_lookup(name, (*vc).table);
                    if !var.is_null() && invisible_p!(var) != 0 {
                        var = 0 as *mut SHELL_VAR;
                    }
                    if !var.is_null() {
                        break;
                    }
                    vc = (*vc).down;
                }
            }
        }
        if var.is_null() {
            return 0 as *mut libc::c_void as *mut SHELL_VAR;
        }

        if ((*var).dynamic_value).is_some() {
            return Some(((*var).dynamic_value).expect("non-null function pointer"))
                .expect("non-null function pointer")(var);
        } else {
            return var;
        };
    }
}

#[no_mangle]
pub fn find_variable_nameref(mut v: *mut SHELL_VAR) -> *mut SHELL_VAR {
    let mut level: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut newname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut orig: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut oldv: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    level = 0 as libc::c_int;

    orig = v;
    unsafe {
        while !v.is_null() && nameref_p!(v) != 0 {
            level += 1;
            level;
            if level > NAMEREF_MAX as libc::c_int {
                return 0 as *mut SHELL_VAR;
            }
            newname = nameref_cell!(v);
            if newname.is_null() || *newname as libc::c_int == '\0' as i32 {
                return 0 as *mut SHELL_VAR;
            }
            oldv = v;
            flags = 0 as libc::c_int;
            if expanding_redir == 0 as libc::c_int
                && (assigning_in_environment != 0 || executing_builtin != 0)
            {
                flags |= FV_FORCETEMPENV!();
            }
            v = find_variable_internal(newname, flags);
            if v == orig || v == oldv {
                internal_warning(
                    b"%s: circular name reference\0" as *const u8 as *const libc::c_char,
                    (*orig).name,
                );
                if variable_context != 0 && (*v).context != 0 {
                    return find_global_variable_noref((*v).name);
                } else {
                    return 0 as *mut SHELL_VAR;
                }
            }
        }
    }
    return v;
}

#[no_mangle]
pub fn find_variable_last_nameref(
    name: *const libc::c_char,
    vflags: libc::c_int,
) -> *mut SHELL_VAR {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut nv: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut newname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut level: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    v = find_variable_noref(name);
    nv = v;
    level = 0 as libc::c_int;
    unsafe {
        while !v.is_null() && nameref_p!(v) != 0 as libc::c_int {
            level += 1;
            level;
            if level > NAMEREF_MAX as libc::c_int {
                return 0 as *mut SHELL_VAR;
            }
            newname = nameref_cell!(v);
            if newname.is_null() || *newname as libc::c_int == '\0' as i32 {
                return if vflags != 0 && invisible_p!(v) != 0 {
                    v
                } else {
                    0 as *mut SHELL_VAR
                };
            }
            nv = v;
            flags = 0 as libc::c_int;
            if expanding_redir == 0 as libc::c_int
                && (assigning_in_environment != 0 || executing_builtin != 0)
            {
                flags |= FV_FORCETEMPENV!() as libc::c_int;
            }
            v = find_variable_internal(newname, flags);
        }
    }
    return nv;
}

#[no_mangle]
pub fn find_global_variable_last_nameref(
    name: *const libc::c_char,
    vflags: libc::c_int,
) -> *mut SHELL_VAR {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut nv: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut newname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut level: libc::c_int = 0;
    v = find_global_variable_noref(name);
    nv = v;
    level = 0 as libc::c_int;
    unsafe {
        while !v.is_null() && nameref_p!(v) != 0 {
            level += 1;
            level;
            if level > NAMEREF_MAX as libc::c_int {
                return 0 as *mut SHELL_VAR;
            }
            newname = nameref_cell!(v);
            if newname.is_null() || *newname as libc::c_int == '\0' as i32 {
                return if vflags != 0 && invisible_p!(v) != 0 {
                    v
                } else {
                    0 as *mut SHELL_VAR
                };
            }
            nv = v;
            v = find_global_variable_noref(newname);
        }
    }
    return nv;
}

fn find_nameref_at_context(v: *mut SHELL_VAR, vc: *mut VAR_CONTEXT) -> *mut SHELL_VAR {
    let mut nv: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut nv2: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut newname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut level: libc::c_int = 0;
    nv = v;
    level = 1 as libc::c_int;
    unsafe {
        while !nv.is_null() && nameref_p!(nv) != 0 {
            level += 1;
            level;
            if level > NAMEREF_MAX as libc::c_int {
                return &mut nameref_maxloop_value;
            }
            newname = nameref_cell!(nv);
            if newname.is_null() || *newname as libc::c_int == '\0' as i32 {
                return 0 as *mut libc::c_void as *mut SHELL_VAR;
            }
            nv2 = hash_lookup(newname, (*vc).table);
            if nv2.is_null() {
                break;
            }
            nv = nv2;
        }
    }
    return nv;
}

fn find_variable_nameref_context(
    v: *mut SHELL_VAR,
    vc: *mut VAR_CONTEXT,
    nvcp: *mut *mut VAR_CONTEXT,
) -> *mut SHELL_VAR {
    let mut nv: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut nv2: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut nvc: *mut VAR_CONTEXT = 0 as *mut VAR_CONTEXT;

    nv = v;
    nvc = vc;
    unsafe {
        while !nvc.is_null() {
            nv2 = find_nameref_at_context(nv, nvc);
            if nv2 == &mut nameref_maxloop_value as *mut SHELL_VAR {
                return nv2;
            }
            if !nv2.is_null() {
                nv = nv2;
                if !(*nvcp).is_null() {
                    *nvcp = nvc;
                }
                if nameref_p!(nv) == 0 as libc::c_int {
                    break;
                }
            }
            nvc = (*nvc).down;
        }
        return if nameref_p!(nv) as libc::c_int != 0 {
            0 as *mut libc::c_void as *mut SHELL_VAR
        } else {
            nv
        };
    }
}

fn find_variable_last_nameref_context(
    v: *mut SHELL_VAR,
    vc: *mut VAR_CONTEXT,
    nvcp: *mut *mut VAR_CONTEXT,
) -> *mut SHELL_VAR {
    let mut nv: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut nv2: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut nvc: *mut VAR_CONTEXT = 0 as *mut VAR_CONTEXT;
    nv = v;
    nvc = vc;
    unsafe {
        while !nvc.is_null() {
            nv2 = find_nameref_at_context(nv, nvc);
            if nv2 == &mut nameref_maxloop_value as *mut SHELL_VAR {
                return nv2;
            }
            if !nv2.is_null() {
                nv = nv2;
                if !(*nvcp).is_null() {
                    *nvcp = nvc;
                }
            }
            nvc = (*nvc).down;
        }
        return if nameref_p!(nv) != 0 {
            nv
        } else {
            0 as *mut libc::c_void as *mut SHELL_VAR
        };
    }
}

#[no_mangle]
pub fn find_variable_nameref_for_create(
    name: *const libc::c_char,
    flags: libc::c_int,
) -> *mut SHELL_VAR {
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    var = find_variable_last_nameref(name, 1 as libc::c_int);
    unsafe {
        if flags & 1 as libc::c_int != 0
            && !var.is_null()
            && (*var).attributes & nameref_p!(var) != 0
            && invisible_p!(var) != 0
        {
            internal_warning(
                b"%s: removing nameref attribute\0" as *const u8 as *const libc::c_char,
                name,
            );
            VUNSETATTR!(var, att_nameref);
        }
        if !var.is_null() && nameref_p!(var) as libc::c_int != 0 {
            if legal_identifier(nameref_cell!(var)) == 0 as libc::c_int {
                if !nameref_cell!(var).is_null() {
                    sh_invalidid(nameref_cell!(var));
                } else {
                    sh_invalidid(b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
                }
                return INVALID_NAMEREF_VALUE!();
            }
        }
    }
    return var;
}

pub fn find_variable_nameref_for_assignment(
    name: *const libc::c_char,
    flags: libc::c_int,
) -> *mut SHELL_VAR {
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    var = find_variable_last_nameref(name, 1 as libc::c_int);
    unsafe {
        if !var.is_null() && nameref_p!(var) != 0 && invisible_p!(var) != 0 {
            internal_warning(
                b"%s: removing nameref attribute\0" as *const u8 as *const libc::c_char,
                name,
            );

            VUNSETATTR!(var, att_nameref);
        }
        if !var.is_null() && nameref_p!(var) != 0 {
            if valid_nameref_value(nameref_cell!(var), 1 as libc::c_int) == 0 as libc::c_int {
                if !nameref_cell!(var).is_null() {
                    sh_invalidid(nameref_cell!(var));
                } else {
                    sh_invalidid(b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
                }
                return INVALID_NAMEREF_VALUE!();
            }
        }
    }
    return var;
}

#[no_mangle]
pub fn nameref_transform_name(name: *mut libc::c_char, flags: libc::c_int) -> *mut libc::c_char {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let newname: *mut libc::c_char = 0 as *mut libc::c_char;
    v = 0 as *mut SHELL_VAR;
    unsafe {
        if flags & ASS_MKLOCAL!() != 0 {
            v = find_variable_last_nameref(name, 1 as libc::c_int);
            if !v.is_null() && (*v).context != variable_context {
                v = 0 as *mut SHELL_VAR;
            }
        } else if flags & ASS_MKGLOBAL!() != 0 {
            v = if flags & ASS_CHKLOCAL!() != 0 {
                find_variable_last_nameref(name, 1 as libc::c_int)
            } else {
                find_global_variable_last_nameref(name, 1 as libc::c_int)
            };
        }
        if !v.is_null()
            && nameref_p!(v) != 0
            && valid_nameref_value(nameref_cell!(v), 1 as libc::c_int) != 0
        {
            return nameref_cell!(v);
        }
    }
    return name;
}

#[no_mangle]
pub fn find_variable_tempenv(name: *const libc::c_char) -> *mut SHELL_VAR {
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    var = find_variable_internal(name, FV_FORCETEMPENV!() as libc::c_int);
    if unsafe { !var.is_null() && nameref_p!(var) != 0 } {
        var = find_variable_nameref(var);
    }
    return var;
}

#[no_mangle]
pub fn find_variable_notempenv(name: *const libc::c_char) -> *mut SHELL_VAR {
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    var = find_variable_internal(name, 0 as libc::c_int);
    if unsafe { !var.is_null() && nameref_p!(var) != 0 } {
        var = find_variable_nameref(var);
    }
    return var;
}

#[no_mangle]
pub fn find_global_variable(name: *const libc::c_char) -> *mut SHELL_VAR {
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    unsafe {
        var = var_lookup(name, global_variables);
        if !var.is_null() && nameref_p!(var) != 0 {
            var = find_variable_nameref(var);
        }
        if var.is_null() {
            return 0 as *mut libc::c_void as *mut SHELL_VAR;
        }
        return if ((*var).dynamic_value).is_some() {
            (Some(((*var).dynamic_value).expect("non-null function pointer")))
                .expect("non-null function pointer")(var)
        } else {
            var
        };
    }
}
#[no_mangle]
pub fn find_global_variable_noref(name: *const libc::c_char) -> *mut SHELL_VAR {
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    unsafe {
        var = var_lookup(name, global_variables);
        if var.is_null() {
            return 0 as *mut libc::c_void as *mut SHELL_VAR;
        }
        return if ((*var).dynamic_value).is_some() {
            (Some(((*var).dynamic_value).expect("non-null function pointer")))
                .expect("non-null function pointer")(var)
        } else {
            var
        };
    }
}

#[no_mangle]
pub fn find_shell_variable(name: *const libc::c_char) -> *mut SHELL_VAR {
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    unsafe {
        var = var_lookup(name, shell_variables);
        if !var.is_null() && nameref_p!(var) != 0 {
            var = find_variable_nameref(var);
        }
        if var.is_null() {
            return 0 as *mut libc::c_void as *mut SHELL_VAR;
        }
        return if ((*var).dynamic_value).is_some() {
            (Some(((*var).dynamic_value).expect("non-null function pointer")))
                .expect("non-null function pointer")(var)
        } else {
            var
        };
    }
}
#[no_mangle]
pub fn find_variable(name: *const libc::c_char) -> *mut SHELL_VAR {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut flags: libc::c_int = 0;
    unsafe {
        last_table_searched = 0 as *mut HASH_TABLE;

        flags = 0 as libc::c_int;
        if expanding_redir == 0 as libc::c_int
            && (assigning_in_environment != 0 || executing_builtin != 0)
        {
            flags |= FV_FORCETEMPENV!();
        }

        v = find_variable_internal(name, flags);

        if !v.is_null() && nameref_p!(v) != 0 {
            // PS1打印值
            v = find_variable_nameref(v);
        }
    }
    return v;
}

#[no_mangle]
pub fn find_variable_no_invisible(name: *const libc::c_char) -> *mut SHELL_VAR {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut flags: libc::c_int = 0;
    unsafe {
        last_table_searched = 0 as *mut HASH_TABLE;
        flags = FV_SKIPINVISIBLE!();
        if expanding_redir == 0 as libc::c_int
            && (assigning_in_environment != 0 || executing_builtin != 0)
        {
            flags |= FV_FORCETEMPENV!();
        }
        v = find_variable_internal(name, flags);
        if !v.is_null() && nameref_p!(v) != 0 {
            v = find_variable_nameref(v);
        }
    }
    return v;
}

#[no_mangle]
pub fn find_variable_for_assignment(name: *const libc::c_char) -> *mut SHELL_VAR {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut flags: libc::c_int = 0;
    unsafe {
        last_table_searched = 0 as *mut HASH_TABLE;
        flags = 0 as libc::c_int;
        if expanding_redir == 0 as libc::c_int
            && (assigning_in_environment != 0 || executing_builtin != 0)
        {
            flags |= FV_FORCETEMPENV!();
        }
        v = find_variable_internal(name, flags);
        if !v.is_null() && nameref_p!(v) != 0 {
            v = find_variable_nameref(v);
        }
    }
    return v;
}

#[no_mangle]
pub fn find_variable_noref(name: *const libc::c_char) -> *mut SHELL_VAR {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut flags: libc::c_int = 0;
    unsafe {
        flags = 0 as libc::c_int;
        if expanding_redir == 0 as libc::c_int
            && (assigning_in_environment != 0 || executing_builtin != 0)
        {
            flags |= FV_FORCETEMPENV!();
        }
    }
    v = find_variable_internal(name, flags);
    return v;
}

#[no_mangle]
pub fn find_function(name: *const libc::c_char) -> *mut SHELL_VAR {
    unsafe {
        return hash_lookup(name, shell_functions);
    }
}

#[no_mangle]
pub fn find_function_def(name: *const libc::c_char) -> *mut FUNCTION_DEF {
    unsafe {
        return hash_lookup(name, shell_function_defs) as *mut FUNCTION_DEF;
    }
}

#[no_mangle]
pub fn get_variable_value(var: *mut SHELL_VAR) -> *mut libc::c_char {
    unsafe {
        if var.is_null() {
            return 0 as *mut libc::c_void as *mut libc::c_char;
        } else if array_p!(var) != 0 {
            return array_reference(array_cell!(var), 0 as libc::c_int as arrayind_t);
        } else if assoc_p!(var) != 0 {
            return assoc_reference(
                assoc_cell!(var),
                b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else {
            return value_cell!(var);
        };
    }
}

#[no_mangle]
pub fn get_string_value(var_name: *const libc::c_char) -> *mut libc::c_char {
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    var = find_variable(var_name);
    return if !var.is_null() {
        get_variable_value(var)
    } else {
        0 as *mut libc::c_void as *mut libc::c_char
    };
}

#[no_mangle]
pub fn sh_get_env_value_rename(v: *const libc::c_char) -> *mut libc::c_char {
    return get_string_value(v);
}

#[no_mangle]
pub fn validate_inherited_value(var: *mut SHELL_VAR, type_0: libc::c_int) -> libc::c_int {
    unsafe {
        if type_0 == att_array as libc::c_int && assoc_p!(var) != 0 {
            return 0 as libc::c_int;
        } else if type_0 == att_assoc as libc::c_int && array_p!(var) != 0 {
            return 0 as libc::c_int;
        } else {
            return 1 as libc::c_int;
        };
    }
}
#[no_mangle]
pub fn var_sametype(v1: *mut SHELL_VAR, v2: *mut SHELL_VAR) -> libc::c_int {
    unsafe {
        if v1.is_null() || v2.is_null() {
            return 0 as libc::c_int;
        } else if assoc_p!(v1) != 0 && assoc_p!(v2) != 0 {
            return 1 as libc::c_int;
        } else if array_p!(v1) != 0 && array_p!(v2) != 0 {
            return 1 as libc::c_int;
        } else if array_p!(v1) != 0 || array_p!(v2) != 0 {
            return 0 as libc::c_int;
        } else if assoc_p!(v1) != 0 || assoc_p!(v2) != 0 {
            return 0 as libc::c_int;
        } else {
            return 1 as libc::c_int;
        }
    }
}

#[no_mangle]
pub fn set_if_not(name: *mut libc::c_char, value: *mut libc::c_char) -> *mut SHELL_VAR {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    unsafe {
        if shell_variables.is_null() {
            create_variable_tables();
        }

        v = find_variable(name);

        if v.is_null() {
            v = bind_variable_internal(
                name,
                value,
                (*global_variables).table,
                HASH_NOSRCH as libc::c_int,
                0 as libc::c_int,
            );
        }
    }
    return v;
}

#[no_mangle]
pub fn make_local_variable(name: *const libc::c_char, flags: libc::c_int) -> *mut SHELL_VAR {
    let mut new_var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut old_var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut old_ref: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut vc: *mut VAR_CONTEXT = 0 as *mut VAR_CONTEXT;
    let mut was_tmpvar: bool = false;
    let mut old_value: *mut libc::c_char = 0 as *mut libc::c_char;

    old_ref = find_variable_noref(name);
    unsafe {
        if !old_ref.is_null() && nameref_p!(old_ref) == 0 {
            old_ref = 0 as *mut SHELL_VAR;
        }
        old_var = find_variable(name);
        if old_ref.is_null()
            && !old_var.is_null()
            && local_p!(old_var) != 0
            && (*old_var).context == variable_context
        {
            if !((*old_var).value).is_null() {
                libc::free((*old_var).value as *mut libc::c_void);
                (*old_var).value = 0 as *mut libc::c_char;
            }
            return old_var;
        }
        if !old_ref.is_null() && local_p!(old_ref) != 0 && (*old_ref).context == variable_context {
            return old_ref;
        }
        if !old_ref.is_null() {
            old_var = old_ref;
        }

        was_tmpvar = !old_var.is_null() && tempvar_p!(old_var) != 0;

        loop {
            if was_tmpvar
                && (*old_var).context == variable_context
                && last_table_searched != temporary_env
            {
                VUNSETATTR!(old_var, att_invisible);
                new_var = old_var;
                vc = shell_variables;
                while !vc.is_null() {
                    if vc_isfuncenv!(vc) && (*vc).scope == variable_context {
                        break;
                    }
                    vc = (*vc).down;
                }
                break;
                return old_var;
            }

            if was_tmpvar {
                old_value = value_cell!(old_var);
            } else {
                old_value = 0 as *mut libc::c_char;
            }

            vc = shell_variables;
            while !vc.is_null() {
                if vc_isfuncenv!(vc) && (*vc).scope == variable_context {
                    break;
                }
                vc = (*vc).down;
            }

            if vc.is_null() {
                internal_error(
                    b"make_local_variable: no function context at current scope\0" as *const u8
                        as *const libc::c_char,
                );
                return 0 as *mut libc::c_void as *mut SHELL_VAR;
            } else if ((*vc).table).is_null() {
                (*vc).table = hash_create(TEMPENV_HASH_BUCKETS!() as libc::c_int);
            }
            if !old_var.is_null()
                && (noassign_p!(old_var) != 0
                    || (readonly_p!(old_var) != 0 && (*old_var).context == 0))
            {
                if readonly_p!(old_var) != 0 {
                    sh_readonly(name as *mut libc::c_char);
                } else if noassign_p!(old_var) != 0 {
                    builtin_error(
                        b"%s: variable may not be assigned value\0" as *const u8
                            as *mut libc::c_char,
                        name,
                    );
                }
                return 0 as *mut libc::c_void as *mut SHELL_VAR;
            }
            if old_var.is_null() {
                new_var = make_new_variable(name, (*vc).table);
            } else {
                new_var = make_new_variable(name, (*vc).table);
                if was_tmpvar {
                    var_setvalue!(new_var, savestring!(old_value));
                } else if localvar_inherit != 0 || (flags & MKLOC_INHERIT!()) != 0 {
                    if assoc_p!(old_var) != 0 {
                        var_setassoc!(new_var, assoc_copy!(assoc_cell!(old_var)));
                    } else if array_p!(old_var) != 0 {
                        var_setarray!(new_var, array_copy(array_cell!(old_var)));
                    } else if !value_cell!(old_var).is_null() {
                        var_setvalue!(new_var, savestring!(value_cell!(old_var)));
                    } else {
                        var_setvalue!(new_var, 0 as *mut libc::c_void as *mut libc::c_char);
                    }
                }
                if localvar_inherit != 0 || (flags & MKLOC_INHERIT!()) != 0 {
                    (*new_var).attributes = (*old_var).attributes & (!att_nameref) as libc::c_int;

                    (*new_var).dynamic_value = (*old_var).dynamic_value;
                    (*new_var).assign_func = (*old_var).assign_func;
                } else {
                    if exported_p!(old_var) != 0 {
                        (*new_var).attributes = att_exported as libc::c_int;
                    } else {
                        (*new_var).attributes = 0 as libc::c_int;
                    }
                }
            }
            break;
        }

        (*vc).flags |= VC_HASLOCAL as libc::c_int;
        (*new_var).context = variable_context;

        VSETATTR!(new_var, att_local);
        if ifsname!(name) {
            setifs(new_var);
        }
        if !was_tmpvar && value_cell!(new_var) == 0 as *mut libc::c_char {
            VSETATTR!(new_var, att_invisible);
        }
    }
    return new_var;
}

fn new_shell_variable(name: *const libc::c_char) -> *mut SHELL_VAR {
    let mut entry: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    unsafe {
        entry = libc::malloc(std::mem::size_of::<SHELL_VAR>() as usize) as *mut SHELL_VAR;
        (*entry).name = savestring!(name);
        var_setvalue!(entry, 0 as *mut libc::c_void as *mut libc::c_char);
        CLEAR_EXPORTSTR!(entry);

        (*entry).dynamic_value = std::mem::transmute::<
            *mut libc::c_void,
            Option<sh_var_value_func_t>,
        >(0 as *mut libc::c_void);
        (*entry).assign_func = std::mem::transmute::<*mut libc::c_void, Option<sh_var_assign_func_t>>(
            0 as *mut libc::c_void,
        );
        (*entry).attributes = 0 as libc::c_int;
        (*entry).context = 0 as libc::c_int;
    }
    return entry;
}

fn make_new_variable(name: *const libc::c_char, table: *mut HASH_TABLE) -> *mut SHELL_VAR {
    let mut entry: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut elt: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    unsafe {
        entry = new_shell_variable(name);
        if shell_variables.is_null() {
            create_variable_tables();
        }
        elt = hash_insert(savestring!(name), table, HASH_NOSRCH as libc::c_int);
        (*elt).data = entry as *mut libc::c_void;
    }
    return entry;
}

#[no_mangle]
pub fn make_new_array_variable(name: *mut libc::c_char) -> *mut SHELL_VAR {
    let mut entry: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut array: *mut ARRAY = 0 as *mut ARRAY;
    unsafe {
        entry = make_new_variable(name, (*global_variables).table);
        array = array_create();

        var_setarray!(entry, array);
        VSETATTR!(entry, att_array);
    }
    return entry;
}

#[no_mangle]
pub fn make_local_array_variable(name: *mut libc::c_char, flags: libc::c_int) -> *mut SHELL_VAR {
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut array: *mut ARRAY = 0 as *mut ARRAY;
    let mut assoc_ok: libc::c_int = 0;

    assoc_ok = flags & MKLOC_ASSOCOK!() as libc::c_int;

    var = make_local_variable(name, flags & MKLOC_INHERIT!() as libc::c_int);
    unsafe {
        if var.is_null() || array_p!(var) != 0 || (assoc_ok != 0 && assoc_p!(var) != 0) {
            return var;
        }
        if localvar_inherit != 0 && assoc_p!(var) != 0 {
            internal_warning(
                b"%s: cannot inherit value from incompatible type\0" as *const u8
                    as *const libc::c_char,
                name,
            );
            VUNSETATTR!(var, att_array);
            dispose_variable_value(var);
            array = array_create();
            var_setarray!(var, array);
        } else if localvar_inherit != 0 {
            var = convert_var_to_array(var);
        } else {
            dispose_variable_value(var);
            array = array_create();
            var_setarray!(var, array);
        }
        VSETATTR!(var, att_array);
    }
    return var;
}

#[no_mangle]
pub fn make_new_assoc_variable(name: *mut libc::c_char) -> *mut SHELL_VAR {
    let mut entry: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut hash: *mut HASH_TABLE = 0 as *mut HASH_TABLE;
    unsafe {
        entry = make_new_variable(name, (*global_variables).table);
        hash = assoc_create!(ASSOC_HASH_BUCKETS!());

        var_setassoc!(entry, hash);
        VSETATTR!(entry, att_assoc);
    }
    return entry;
}

#[no_mangle]
pub fn make_local_assoc_variable(name: *mut libc::c_char, flags: libc::c_int) -> *mut SHELL_VAR {
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut hash: *mut HASH_TABLE = 0 as *mut HASH_TABLE;
    let mut array_ok: libc::c_int = 0;
    array_ok = flags & MKLOC_ARRAYOK!() as libc::c_int;

    var = make_local_variable(name, flags & MKLOC_INHERIT!() as libc::c_int);
    unsafe {
        if var.is_null() || assoc_p!(var) != 0 || (array_ok != 0 && array_p!(var) != 0) {
            return var;
        }
        if localvar_inherit != 0 && array_p!(var) != 0 {
            internal_warning(
                b"%s: cannot inherit value from incompatible type\0" as *const u8
                    as *const libc::c_char,
                name,
            );
            VUNSETATTR!(var, att_array);

            dispose_variable_value(var);
            hash = assoc_create!(ASSOC_HASH_BUCKETS as libc::c_int);
            var_setassoc!(var, hash);
        } else if localvar_inherit != 0 {
            var = convert_var_to_assoc(var);
        } else {
            dispose_variable_value(var);
            hash = assoc_create!(ASSOC_HASH_BUCKETS as libc::c_int);
            var_setassoc!(var, hash);
        }
        VSETATTR!(var, att_assoc);
    }
    return var;
}

#[no_mangle]
pub fn make_variable_value(
    var: *mut SHELL_VAR,
    value: *mut libc::c_char,
    flags: libc::c_int,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut retval: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut oval: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lval: intmax_t = 0;
    let mut rval: intmax_t = 0;
    let mut expok: libc::c_int = 0;
    let mut olen: libc::c_int = 0;
    let mut op: libc::c_int = 0;

    let mut T_flags: bool = false;
    unsafe {
        loop {
            if flags & ASS_NOEVAL!() as libc::c_int == 0 as libc::c_int && integer_p!(var) != 0 {
                if flags & ASS_APPEND!() as libc::c_int != 0 {
                    oval = value_cell!(var);
                    lval = evalexp(oval, 0 as libc::c_int, &mut expok);
                    if expok == 0 as libc::c_int {
                        if flags & (ASS_NOLONGJMP as libc::c_int) != 0 {
                            T_flags = true;
                            break;
                        } else {
                            top_level_cleanup();
                            jump_to_top_level(DISCARD as libc::c_int);
                        }
                    }
                }

                rval = evalexp(value, 0 as libc::c_int, &mut expok);
                if expok == 0 as libc::c_int {
                    if flags & (ASS_NOLONGJMP as libc::c_int) != 0 {
                        T_flags = true;
                        break;
                    } else {
                        top_level_cleanup();
                        jump_to_top_level(DISCARD as libc::c_int);
                    }
                }

                if flags & (ASS_APPEND as libc::c_int) != 0 {
                    rval += lval;
                }
                retval = itos(rval);
            } else if flags & ASS_NOEVAL!() == 0 as libc::c_int
                && (capcase_p!(var) != 0 || uppercase_p!(var) != 0 || lowercase_p!(var) != 0)
            {
                if flags & ASS_APPEND as libc::c_int != 0 {
                    oval = get_variable_value(var);
                    if oval.is_null() {
                        oval = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                    }
                    olen = STRLEN!(oval);
                    if !value.is_null() {
                        retval = libc::malloc(olen as usize + STRLEN!(value) as usize + 1 as usize)
                            as *mut libc::c_char;
                    } else {
                        retval = libc::malloc(olen as usize + 1 as usize) as *mut libc::c_char;
                    }
                    strcpy(retval, oval);

                    if !value.is_null() {
                        strcpy(retval.offset(olen as isize), value);
                    }
                } else if *value != 0 {
                    retval = savestring!(value);
                } else {
                    retval = libc::malloc(1 as libc::c_int as usize) as *mut libc::c_char;
                    *retval.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                }

                op = if capcase_p!(var) != 0 {
                    CASE_CAPITALIZE as libc::c_int
                } else if uppercase_p!(var) != 0 {
                    CASE_UPPER as libc::c_int
                } else {
                    CASE_LOWER as libc::c_int
                };

                oval = sh_modcase(retval, 0 as *mut libc::c_char, op);
                free(retval as *mut libc::c_void);
                retval = oval;
            } else if !value.is_null() {
                if flags & ASS_APPEND as libc::c_int != 0 as libc::c_int {
                    oval = get_variable_value(var);
                    if oval.is_null() {
                        oval = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                    }
                    olen = STRLEN!(oval);
                    if !value.is_null() {
                        retval = libc::malloc(
                            (olen as size_t + STRLEN!(value) as size_t + 1 as u64) as usize,
                        ) as *mut libc::c_char;
                    } else {
                        retval =
                            libc::malloc((olen as size_t + 1 as u64) as usize) as *mut libc::c_char;
                    }
                    strcpy(retval, oval);
                    if !value.is_null() {
                        strcpy(retval.offset(olen as isize), value);
                    }
                } else if *value != 0 {
                    retval = savestring!(value);
                } else {
                    retval = libc::malloc(1 as libc::c_int as usize) as *mut libc::c_char;
                    *retval = '\0' as i32 as libc::c_char;
                }
            } else {
                retval = 0 as *mut libc::c_void as *mut libc::c_char;
            }
            break;
        }

        if T_flags {
            if flags & ASS_APPEND as libc::c_int != 0 as libc::c_int {
                oval = get_variable_value(var);
                if oval.is_null() {
                    oval = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                }
                olen = STRLEN!(oval);
                if !value.is_null() {
                    retval = libc::malloc(
                        (olen as size_t + STRLEN!(value) as size_t + 1 as u64) as usize,
                    ) as *mut libc::c_char;
                } else {
                    retval = libc::malloc(olen as usize + 1 as usize) as *mut libc::c_char;
                }
                strcpy(retval, oval);
                if !value.is_null() {
                    strcpy(retval.offset(olen as isize), value);
                }
            } else if *value != 0 {
                retval = savestring!(value);
            } else {
                retval = libc::malloc(1 as libc::c_int as usize) as *mut libc::c_char;
                *retval = '\0' as i32 as libc::c_char;
            }
        }
    }
    return retval;
}

fn can_optimize_assignment(
    entry: *mut SHELL_VAR,
    value: *mut libc::c_char,
    aflags: libc::c_int,
) -> libc::c_int {
    if aflags & ASS_APPEND as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    unsafe {
        if array_p!(entry) != 0 || assoc_p!(entry) != 0 {
            return 0 as libc::c_int;
        }
        if integer_p!(entry) != 0
            || uppercase_p!(entry) != 0
            || lowercase_p!(entry) != 0
            || capcase_p!(entry) != 0
        {
            return 0 as libc::c_int;
        }
        if readonly_p!(entry) != 0 || noassign_p!(entry) != 0 {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}

fn optimized_assignment(
    entry: *mut SHELL_VAR,
    value: *mut libc::c_char,
    aflags: libc::c_int,
) -> *mut SHELL_VAR {
    let mut len: size_t = 0;
    let mut vlen: size_t = 0;
    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        v = value_cell!(entry);

        len = STRLEN!(v) as size_t;
        vlen = STRLEN!(value) as size_t;

        new = libc::realloc(
            v as *mut libc::c_void,
            vlen as usize + len as usize + 8 as usize,
        ) as *mut libc::c_char;
        if vlen == 1 as size_t {
            *new.offset(len as isize) = *value;
            *new.offset((len + 1) as isize) = '\0' as i32 as libc::c_char;
        } else {
            strcpy(new.offset(len as isize), value);
        }

        var_setvalue!(entry, new);
    }
    return entry;
}

fn bind_variable_internal(
    name: *const libc::c_char,
    value: *mut libc::c_char,
    table: *mut HASH_TABLE,
    hflags: libc::c_int,
    aflags: libc::c_int,
) -> *mut SHELL_VAR {
    let mut newval: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut entry: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut tentry: *mut SHELL_VAR = 0 as *mut SHELL_VAR;

    entry = if hflags & HASH_NOSRCH as libc::c_int != 0 {
        0 as *mut libc::c_void as *mut SHELL_VAR
    } else {
        hash_lookup(name, table)
    };

    let mut T_flags: bool = false;
    unsafe {
        if !entry.is_null()
            && nameref_p!(entry) != 0
            && invisible_p!(entry) == 0 as libc::c_int
            && table == (*global_variables).table
        {
            entry = find_global_variable((*entry).name);
            if entry.is_null() {
                entry = find_variable_last_nameref(name, 0 as libc::c_int);
            }
            if entry.is_null() {
                return entry;
            }
        }
        loop {
            T_flags = false;
            if !entry.is_null() && invisible_p!(entry) != 0 && nameref_p!(entry) != 0 {
                if aflags & ASS_FORCE as libc::c_int == 0 as libc::c_int
                    && !value.is_null()
                    && valid_nameref_value(value, 0 as libc::c_int) == 0 as libc::c_int
                {
                    sh_invalidid(value);
                    return 0 as *mut libc::c_void as *mut SHELL_VAR;
                }
                T_flags = true;
                break;
            } else if !entry.is_null() && nameref_p!(entry) as libc::c_int != 0 {
                newval = nameref_cell!(entry);
                if valid_nameref_value(newval, 0 as libc::c_int) == 0 as libc::c_int {
                    sh_invalidid(newval);
                    return 0 as *mut libc::c_void as *mut SHELL_VAR;
                }
                if valid_array_reference(newval, 0 as libc::c_int) != 0 {
                    tname = array_variable_name(
                        newval,
                        0 as libc::c_int,
                        0 as *mut *mut libc::c_char,
                        0 as *mut libc::c_int,
                    );
                    if !tname.is_null()
                        && ({
                            tentry = find_variable_noref(tname);
                            !tentry.is_null()
                        } && nameref_p!(tentry) as libc::c_int != 0)
                    {
                        internal_warning(
                            b"%s: removing nameref attribute\0" as *const u8 as *const libc::c_char,
                            name_cell!(tentry),
                        );
                        if !(value_cell!(tentry).is_null()) {
                            free(value_cell!(tentry) as *mut libc::c_void);
                        }
                        var_setvalue!(tentry, 0 as *mut libc::c_void as *mut libc::c_char);
                        VUNSETATTR!(tentry, att_nameref);
                    }
                    free(tname as *mut libc::c_void);
                    entry = assign_array_element(
                        newval,
                        make_variable_value(entry, value, aflags),
                        aflags | ASS_NAMEREF as libc::c_int,
                    );
                    if entry.is_null() {
                        return entry;
                    }
                } else {
                    entry = make_new_variable(newval, table);
                    var_setvalue!(entry, make_variable_value(entry, value, aflags));
                }
            } else if entry.is_null() {
                entry = make_new_variable(name, table);
                var_setvalue!(entry, make_variable_value(entry, value, aflags));
            } else if ((*entry).assign_func).is_some() {
                if readonly_p!(entry) != 0 && aflags & ASS_FORCE as libc::c_int == 0 as libc::c_int
                    || noassign_p!(entry) != 0
                {
                    if readonly_p!(entry) != 0 {
                        err_readonly(name_cell!(entry));
                    }
                    return entry;
                }

                INVALIDATE_EXPORTSTR!(entry);
                newval = if aflags & ASS_APPEND as libc::c_int != 0 {
                    make_variable_value(entry, value, aflags)
                } else {
                    value
                };
                if assoc_p!(entry) != 0 {
                    entry = (Some(((*entry).assign_func).expect("non-null function pointer")))
                        .expect("non-null function pointer")(
                        entry,
                        newval,
                        -(1 as libc::c_int) as arrayind_t,
                        savestring!(b"0\0" as *const u8 as *const libc::c_char),
                    );
                } else if array_p!(entry) != 0 {
                    entry = (Some(((*entry).assign_func).expect("non-null function pointer")))
                        .expect("non-null function pointer")(
                        entry,
                        newval,
                        0 as libc::c_int as arrayind_t,
                        0 as *mut libc::c_char,
                    );
                } else {
                    entry = (Some(((*entry).assign_func).expect("non-null function pointer")))
                        .expect("non-null function pointer")(
                        entry,
                        newval,
                        -(1 as libc::c_int) as arrayind_t,
                        0 as *mut libc::c_char,
                    );
                }
                if newval != value {
                    free(newval as *mut libc::c_void);
                }
                return entry;
            } else {
                if readonly_p!(entry) != 0 && aflags & ASS_FORCE as libc::c_int == 0 as libc::c_int
                    || noassign_p!(entry) != 0
                {
                    if readonly_p!(entry) != 0 {
                        err_readonly(name_cell!(entry));
                    }
                    return entry;
                }
                VUNSETATTR!(entry, att_invisible);
                if can_optimize_assignment(entry, value, aflags) != 0 {
                    INVALIDATE_EXPORTSTR!(entry);
                    optimized_assignment(entry, value, aflags);

                    if mark_modified_vars != 0 {
                        VSETATTR!(entry, att_exported);
                    }
                    if exported_p!(entry) != 0 {
                        array_needs_making = 1 as libc::c_int;
                    }
                    return entry;
                }

                if assoc_p!(entry) != 0 || array_p!(entry) != 0 {
                    newval = make_array_variable_value(
                        entry,
                        0 as libc::c_int as arrayind_t,
                        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        value,
                        aflags,
                    );
                } else {
                    newval = make_variable_value(entry, value, aflags);
                }
                INVALIDATE_EXPORTSTR!(entry);

                if assoc_p!(entry) != 0 {
                    assoc_insert(
                        assoc_cell!(entry),
                        savestring!(b"0\0" as *const u8 as *const libc::c_char),
                        newval,
                    );
                    free(newval as *mut libc::c_void);
                } else if array_p!(entry) != 0 {
                    array_insert(array_cell!(entry), 0 as libc::c_int as arrayind_t, newval);
                    free(newval as *mut libc::c_void);
                } else {
                    if !value_cell!(entry).is_null() {
                        free(value_cell!(entry) as *mut libc::c_void);
                    }
                    var_setvalue!(entry, newval);
                }
            }
            break;
        }

        if T_flags {
            if readonly_p!(entry) != 0 && aflags & ASS_FORCE as libc::c_int == 0 as libc::c_int
                || noassign_p!(entry) != 0
            {
                if readonly_p!(entry) != 0 {
                    err_readonly(name_cell!(entry));
                }
                return entry;
            }
            VUNSETATTR!(entry, att_invisible);
            if can_optimize_assignment(entry, value, aflags) != 0 {
                INVALIDATE_EXPORTSTR!(entry);
                optimized_assignment(entry, value, aflags);

                if mark_modified_vars != 0 {
                    VSETATTR!(entry, att_exported);
                }
                if exported_p!(entry) != 0 {
                    array_needs_making = 1 as libc::c_int;
                }
                return entry;
            }

            if assoc_p!(entry) != 0 || array_p!(entry) != 0 {
                newval = make_array_variable_value(
                    entry,
                    0 as libc::c_int as arrayind_t,
                    b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    value,
                    aflags,
                );
            } else {
                newval = make_variable_value(entry, value, aflags);
            }
            INVALIDATE_EXPORTSTR!(entry);

            if assoc_p!(entry) != 0 {
                assoc_insert(
                    (*entry).value as *mut HASH_TABLE,
                    savestring!(b"0\0" as *const u8 as *const libc::c_char),
                    newval,
                );
                free(newval as *mut libc::c_void);
            } else if array_p!(entry) != 0 {
                array_insert(array_cell!(entry), 0 as libc::c_int as arrayind_t, newval);
                free(newval as *mut libc::c_void);
            } else {
                if !value_cell!(entry).is_null() {
                    free(value_cell!(entry) as *mut libc::c_void);
                }
                var_setvalue!(entry, newval);
            }
        }

        if mark_modified_vars != 0 {
            VSETATTR!(entry, att_exported);
        }
        if exported_p!(entry) != 0 {
            array_needs_making = 1 as libc::c_int;
        }
    }
    return entry;
}

#[no_mangle]
pub fn bind_global_variable(
    name: *const libc::c_char,
    value: *mut libc::c_char,
    flags: libc::c_int,
) -> *mut SHELL_VAR {
    unsafe {
        if shell_variables.is_null() {
            create_variable_tables();
        }

        return bind_variable_internal(
            name,
            value,
            (*global_variables).table,
            0 as libc::c_int,
            flags,
        );
    }
}

fn bind_invalid_envvar(
    name: *const libc::c_char,
    value: *mut libc::c_char,
    flags: libc::c_int,
) -> *mut SHELL_VAR {
    unsafe {
        if invalid_env.is_null() {
            invalid_env = hash_create(64 as libc::c_int);
        }
        return bind_variable_internal(name, value, invalid_env, HASH_NOSRCH as libc::c_int, flags);
    }
}

#[no_mangle]
pub fn bind_variable_value(
    var: *mut SHELL_VAR,
    value: *mut libc::c_char,
    aflags: libc::c_int,
) -> *mut SHELL_VAR {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut invis: libc::c_int = 0;
    unsafe {
        invis = invisible_p!(var);
        VUNSETATTR!(var, att_invisible);

        if ((*var).assign_func).is_some() {
            t = if aflags & ASS_APPEND as libc::c_int != 0 {
                make_variable_value(var, value, aflags)
            } else {
                value
            };
            (Some(((*var).assign_func).expect("non-null function pointer")))
                .expect("non-null function pointer")(
                var,
                t,
                -(1 as libc::c_int) as arrayind_t,
                0 as *mut libc::c_char,
            );
            if t != value && !t.is_null() {
                free(t as *mut libc::c_void);
            }
        } else {
            t = make_variable_value(var, value, aflags);
            if aflags & (ASS_NAMEREF as libc::c_int | ASS_FORCE as libc::c_int)
                == ASS_NAMEREF as libc::c_int
                && check_selfref(name_cell!(var), t, 0 as libc::c_int) != 0
            {
                if variable_context != 0 {
                    internal_warning(
                        b"%s: circular name reference\0" as *const u8 as *const libc::c_char,
                        name_cell!(var),
                    );
                } else {
                    internal_error(
                        b"%s: nameref variable self references not allowed\0" as *const u8
                            as *const libc::c_char,
                        name_cell!(var),
                    );
                    free(t as *mut libc::c_void);
                    if invis != 0 {
                        VSETATTR!(var, att_invisible);
                    }
                    return 0 as *mut libc::c_void as *mut SHELL_VAR;
                }
            }
            if aflags & ASS_NAMEREF as libc::c_int != 0
                && valid_nameref_value(t, 0 as libc::c_int) == 0 as libc::c_int
            {
                free(t as *mut libc::c_void);
                if invis != 0 {
                    VSETATTR!(var, att_invisible);
                }
                return 0 as *mut libc::c_void as *mut SHELL_VAR;
            }
            if !(value_cell!(var).is_null()) {
                free(value_cell!(var) as *mut libc::c_void);
            }
            var_setvalue!(var, t);
        }
        INVALIDATE_EXPORTSTR!(var);

        if mark_modified_vars != 0 {
            VSETATTR!(var, att_exported);
        }
        if exported_p!(var) != 0 {
            array_needs_making = 1 as libc::c_int;
        }
    }
    return var;
}

#[no_mangle]
pub fn bind_int_variable(
    lhs: *mut libc::c_char,
    rhs: *mut libc::c_char,
    flags: libc::c_int,
) -> *mut SHELL_VAR {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut isint: libc::c_int = 0;
    let mut isarr: libc::c_int = 0;
    let mut implicitarray: libc::c_int = 0;
    implicitarray = 0 as libc::c_int;
    isarr = implicitarray;
    isint = isarr;
    unsafe {
        if valid_array_reference(
            lhs,
            (flags & ASS_NOEXPAND as libc::c_int != 0 as libc::c_int) as libc::c_int,
        ) != 0
        {
            isarr = 1 as libc::c_int;
            v = array_variable_part(
                lhs,
                (flags & ASS_NOEXPAND as libc::c_int != 0 as libc::c_int) as libc::c_int,
                0 as *mut *mut libc::c_char,
                0 as *mut libc::c_int,
            );
        } else if legal_identifier(lhs) == 0 as libc::c_int {
            sh_invalidid(lhs);
            return 0 as *mut libc::c_void as *mut SHELL_VAR;
        } else {
            v = find_variable(lhs);
        }
        if !v.is_null() {
            isint = integer_p!(v);
            VUNSETATTR!(v, att_integer);

            if array_p!(v) != 0 && isarr == 0 as libc::c_int {
                implicitarray = 1 as libc::c_int;
            }
        }
        if isarr != 0 {
            v = assign_array_element(lhs, rhs, flags);
        } else if implicitarray != 0 {
            v = bind_array_variable(lhs, 0 as libc::c_int as arrayind_t, rhs, 0 as libc::c_int);
        } else {
            v = bind_variable(lhs, rhs, 0 as libc::c_int);
        }
        if !v.is_null() {
            if isint != 0 {
                VSETATTR!(v, att_integer);
            }
            VUNSETATTR!(v, att_invisible);
        }
        if !v.is_null() && nameref_p!(v) != 0 {
            internal_warning(
                b"%s: assigning integer to name reference\0" as *const u8 as *const libc::c_char,
                lhs,
            );
        }
    }
    return v;
}

#[no_mangle]
pub fn bind_var_to_int(var: *mut libc::c_char, val: intmax_t) -> *mut SHELL_VAR {
    let mut ibuf: [libc::c_char; (INT_STRLEN_BOUND!(intmax_t) + 1) as usize] =
        [0; (INT_STRLEN_BOUND!(intmax_t) + 1) as usize];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        p = fmtulong(
            val as libc::c_ulong,
            10 as libc::c_int,
            ibuf.as_mut_ptr(),
            std::mem::size_of::<[libc::c_char; (INT_STRLEN_BOUND!(intmax_t) + 1) as usize]>()
                as size_t,
            0 as libc::c_int,
        );
    }
    return bind_int_variable(var, p, 0 as libc::c_int);
}

#[no_mangle]
pub fn bind_function(name: *const libc::c_char, value: *mut COMMAND) -> *mut SHELL_VAR {
    let mut entry: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    entry = find_function(name);
    unsafe {
        if entry.is_null() {
            let mut elt: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
            elt = hash_insert(savestring!(name), shell_functions, 0x1 as libc::c_int);
            entry = new_shell_variable(name);
            (*elt).data = entry as *mut libc::c_void;
        } else {
            INVALIDATE_EXPORTSTR!(entry);
        }
        if !var_isset!(entry) {
            dispose_command(function_cell!(entry) as *mut COMMAND);
        }
        if !value.is_null() {
            var_setfunc!(entry, copy_command(value));
        } else {
            var_setfunc!(entry, 0);
        }

        VSETATTR!(entry, att_function);

        if mark_modified_vars != 0 {
            VSETATTR!(entry, att_function);
        }

        VUNSETATTR!(entry, att_invisible);

        if exported_p!(entry) != 0 {
            array_needs_making = 1 as libc::c_int;
        }
        set_itemlist_dirty(&mut it_functions);
    }
    return entry;
}

#[no_mangle]
pub fn bind_function_def(name: *const libc::c_char, value: *mut FUNCTION_DEF, flags: libc::c_int) {
    let mut entry: *mut FUNCTION_DEF = 0 as *mut FUNCTION_DEF;
    let mut elt: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    let mut cmd: *mut COMMAND = 0 as *mut COMMAND;

    entry = find_function_def(name);
    unsafe {
        if !entry.is_null() && flags & 1 as libc::c_int != 0 {
            dispose_function_def_contents(entry);
            entry = copy_function_def_contents(value, entry);
        } else if !entry.is_null() {
            return;
        } else {
            cmd = (*value).command;
            (*value).command = 0 as *mut COMMAND;
            entry = copy_function_def(value);
            (*value).command = cmd;
            elt = hash_insert(
                savestring!(name),
                shell_function_defs,
                HASH_NOSRCH as libc::c_int,
            );
            (*elt).data = entry as *mut *mut libc::c_void as *mut libc::c_void;
        };
    }
}

#[no_mangle]
pub fn assign_in_env(word: *mut WORD_DESC, flags: libc::c_int) -> libc::c_int {
    let mut offset: libc::c_int = 0;
    let mut aflags: libc::c_int = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut string: *const libc::c_char = 0 as *const libc::c_char;

    unsafe {
        string = (*word).word;
        aflags = 0 as libc::c_int;
        offset = assignment(string, 0 as libc::c_int);
        name = savestring!(string);
        newname = name;
        value = 0 as *mut libc::c_void as *mut libc::c_char;

        if *name.offset(offset as isize) as libc::c_int == '=' as i32 {
            *name.offset(offset as isize) = 0 as libc::c_int as libc::c_char;
            if *name.offset((offset - 1 as libc::c_int) as isize) as libc::c_int == '+' as i32 {
                *name.offset((offset - 1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
                aflags |= ASS_APPEND as libc::c_int;
            }

            if legal_identifier(name) == 0 as libc::c_int {
                sh_invalidid(name);
                return 0 as libc::c_int;
            }

            var = find_variable(name);
            if var.is_null() {
                var = find_variable_last_nameref(name, 1 as libc::c_int);
                if !var.is_null()
                    && nameref_p!(var) != 0
                    && valid_nameref_value((*var).value, 2 as libc::c_int) != 0
                {
                    newname = nameref_cell!(var);
                    var = 0 as *mut SHELL_VAR;
                }
            } else {
                newname = nameref_cell!(var);
            }
            if !var.is_null() && (readonly_p!(var) != 0 || noassign_p!(var) != 0) {
                if readonly_p!(var) != 0 {
                    err_readonly(name);
                }
                free(name as *mut libc::c_void);
                return 0 as libc::c_int;
            }
            temp = name
                .offset(offset as isize)
                .offset(1 as libc::c_int as isize);

            value = expand_assignment_string_to_string(temp, 0 as libc::c_int);

            if !var.is_null() && aflags & ASS_APPEND as libc::c_int != 0 {
                if value.is_null() {
                    value = libc::malloc(1 as libc::c_int as usize) as *mut libc::c_char;
                    *value.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                }
                temp = make_variable_value(var, value, aflags);
                if !value.is_null() {
                    free(value as *mut libc::c_void);
                }
                value = 0 as *mut libc::c_char;
                value = temp;
            }
        }
        if temporary_env.is_null() {
            temporary_env = hash_create(TEMPENV_HASH_BUCKETS!() as libc::c_int);
        }
        var = hash_lookup(newname, temporary_env);
        if var.is_null() {
            var = make_new_variable(newname, temporary_env);
        } else {
            if !(value_cell!(var).is_null()) {
                free(value_cell!(var) as *mut libc::c_void);
            }
            var_setvalue!(var, value);
        }

        if value.is_null() {
            value = libc::malloc(1 as libc::c_int as usize) as *mut libc::c_char;
            *value.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        }

        (*var).value = value;
        (*var).attributes |= att_exported as libc::c_int | att_tempvar as libc::c_int;
        (*var).context = variable_context;

        INVALIDATE_EXPORTSTR!(var);

        (*var).exportstr = mk_env_string(newname, value, 0 as libc::c_int);
        array_needs_making = 1 as libc::c_int;

        if flags != 0 {
            if STREQ(
                newname,
                b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char,
            ) || STREQ(
                newname,
                b"POSIX_PEDANDTIC\0" as *const u8 as *const libc::c_char,
            ) {
                save_posix_options();
            }
            stupidly_hack_special_variables(newname);
        }
        if echo_command_at_execute != 0 {
            xtrace_print_assignment(name, value, 0 as libc::c_int, 1 as libc::c_int);
        }
        free(name as *mut libc::c_void);
    }
    return 1 as libc::c_int;
}

fn dispose_variable_value(var: *mut SHELL_VAR) {
    unsafe {
        if function_p!(var) != 0 {
            dispose_command((*var).value as *mut COMMAND);
        } else if array_p!(var) != 0 {
            array_dispose(array_cell!(var) as *mut ARRAY);
        } else if assoc_p!(var) as libc::c_int != 0 {
            assoc_dispose(assoc_cell!(var) as *mut HASH_TABLE);
        } else if nameref_p!(var) != 0 {
            if !nameref_cell!(var).is_null() {
                free(nameref_cell!(var) as *mut libc::c_void);
            }
        } else if !value_cell!(var).is_null() {
            free(value_cell!(var) as *mut libc::c_void);
        }
    }
}

#[no_mangle]
pub fn dispose_variable(var: *mut SHELL_VAR) {
    if var.is_null() {
        return;
    }
    unsafe {
        if nofree_p!(var) == 0 as libc::c_int {
            dispose_variable_value(var);
        }

        FREE_EXPORTSTR!(var);
        free((*var).name as *mut libc::c_void);

        if exported_p!(var) != 0 {
            array_needs_making = 1 as libc::c_int;
        }
        free(var as *mut libc::c_void);
    }
}

#[no_mangle]
pub fn unbind_variable(name: *const libc::c_char) -> libc::c_int {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut nv: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut r: libc::c_int = 0;
    unsafe {
        v = var_lookup(name, shell_variables);
        nv = if !v.is_null() && nameref_p!(v) != 0 {
            find_variable_nameref(v)
        } else {
            0 as *mut libc::c_void as *mut SHELL_VAR
        };
        r = if !nv.is_null() {
            makunbound((*nv).name, shell_variables)
        } else {
            makunbound(name, shell_variables)
        };
    }
    return r;
}

#[no_mangle]
pub fn unbind_nameref(name: *const libc::c_char) -> libc::c_int {
    unsafe {
        let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
        v = var_lookup(name, shell_variables);
        if !v.is_null() && nameref_p!(v) != 0 {
            return makunbound(name, shell_variables);
        }
        return 0 as libc::c_int;
    }
}

#[no_mangle]
pub fn unbind_variable_noref(name: *const libc::c_char) -> libc::c_int {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    unsafe {
        v = var_lookup(name, shell_variables);
        if !v.is_null() {
            return makunbound(name, shell_variables);
        }
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub fn check_unbind_variable(name: *const libc::c_char) -> libc::c_int {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    v = find_variable(name);
    unsafe {
        if !v.is_null() && readonly_p!(v) != 0 {
            internal_error(
                b"%s: cannot unset: readonly %s\0" as *const u8 as *const libc::c_char,
                name,
                b"variable\0" as *const u8 as *const libc::c_char,
            );
            return -(2 as libc::c_int);
        } else if !v.is_null() && non_unsettable_p!(v) != 0 {
            internal_error(
                b"%s: cannot unset\0" as *const u8 as *const libc::c_char,
                name,
            );
            return -(2 as libc::c_int);
        }
    }
    return unbind_variable(name);
}

#[no_mangle]
pub fn unbind_func(name: *const libc::c_char) -> libc::c_int {
    let mut elt: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    let mut func: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    unsafe {
        elt = hash_remove(name, shell_functions, 0 as libc::c_int);
        if elt.is_null() {
            return -(1 as libc::c_int);
        }
        set_itemlist_dirty(&mut it_functions);
        func = (*elt).data as *mut SHELL_VAR;

        if !func.is_null() {
            if exported_p!(func) != 0 {
                array_needs_making += 1;
                array_needs_making;
            }
            dispose_variable(func);
        }
        free((*elt).key as *mut libc::c_void);
        free(elt as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub fn unbind_function_def(name: *const libc::c_char) -> libc::c_int {
    let mut elt: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    let mut funcdef: *mut FUNCTION_DEF = 0 as *mut FUNCTION_DEF;
    unsafe {
        elt = hash_remove(name, shell_function_defs, 0 as libc::c_int);
        if elt.is_null() {
            return -(1 as libc::c_int);
        }
        funcdef = (*elt).data as *mut FUNCTION_DEF;

        if !funcdef.is_null() {
            dispose_function_def(funcdef);
        }
        free((*elt).key as *mut libc::c_void);
        free(elt as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub fn delete_var(name: *const libc::c_char, vc: *mut VAR_CONTEXT) -> libc::c_int {
    let mut elt: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    let mut old_var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut v: *mut VAR_CONTEXT = 0 as *mut VAR_CONTEXT;
    elt = 0 as *mut libc::c_void as *mut BUCKET_CONTENTS;
    v = vc;
    unsafe {
        while !v.is_null() {
            elt = hash_remove(name, (*v).table, 0 as libc::c_int);
            if !elt.is_null() {
                break;
            }
            v = (*v).down;
        }
        if elt.is_null() {
            return -(1 as libc::c_int);
        }
        old_var = (*elt).data as *mut SHELL_VAR;
        free((*elt).key as *mut libc::c_void);
        free(elt as *mut libc::c_void);
        dispose_variable(old_var);
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub fn makunbound(name: *const libc::c_char, vc: *mut VAR_CONTEXT) -> libc::c_int {
    let mut elt: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    let mut new_elt: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    let mut old_var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut v: *mut VAR_CONTEXT = 0 as *mut VAR_CONTEXT;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    elt = 0 as *mut libc::c_void as *mut BUCKET_CONTENTS;
    v = vc;
    unsafe {
        while !v.is_null() {
            elt = hash_remove(name, (*v).table, 0 as libc::c_int);
            if !elt.is_null() {
                break;
            }
            v = (*v).down;
        }
        if elt.is_null() {
            return -(1 as libc::c_int);
        }
        old_var = (*elt).data as *mut SHELL_VAR;

        if !old_var.is_null() && exported_p!(old_var) != 0 {
            array_needs_making += 1;
            array_needs_making;
        }
        if !old_var.is_null()
            && local_p!(old_var) != 0
            && ((*old_var).context == variable_context
                || localvar_unset != 0 && (*old_var).context < variable_context)
        {
            if nofree_p!(old_var) != 0 {
                var_setvalue!(old_var, 0 as *mut libc::c_void as *mut libc::c_char);
            } else if array_p!(old_var) != 0 {
                array_dispose((*old_var).value as *mut ARRAY);
            } else if assoc_p!(old_var) != 0 {
                assoc_dispose((*old_var).value as *mut HASH_TABLE);
            } else if nameref_p!(old_var) != 0 {
                if !(nameref_cell!(old_var).is_null()) {
                    free(nameref_cell!(old_var) as *mut libc::c_void);
                }
            } else {
                if !value_cell!(old_var).is_null() {
                    free(value_cell!(old_var) as *mut libc::c_void);
                }
            }
            (*old_var).attributes = if exported_p!(old_var) != 0 && tempvar_p!(old_var) != 0 {
                att_exported as libc::c_int
            } else {
                0 as libc::c_int
            };
            VSETATTR!(old_var, att_local);
            VSETATTR!(old_var, att_invisible);
            var_setvalue!(old_var, 0 as *mut libc::c_void as *mut libc::c_char);
            INVALIDATE_EXPORTSTR!(old_var);

            if !((*old_var).exportstr).is_null() {
                free((*old_var).exportstr as *mut libc::c_void);
                (*old_var).exportstr = 0 as *mut libc::c_void as *mut libc::c_char;
            }
            new_elt = hash_insert(savestring!((*old_var).name), (*v).table, 0 as libc::c_int);
            (*new_elt).data = old_var as *mut libc::c_void;
            stupidly_hack_special_variables((*old_var).name);
            free((*elt).key as *mut libc::c_void);
            free(elt as *mut libc::c_void);
            return 0 as libc::c_int;
        }
        t = savestring!(name);
        free((*elt).key as *mut libc::c_void);
        free(elt as *mut libc::c_void);
        dispose_variable(old_var);
        stupidly_hack_special_variables(t);
        free(t as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub fn kill_all_local_variables() {
    let mut vc: *mut VAR_CONTEXT = 0 as *mut VAR_CONTEXT;
    unsafe {
        vc = shell_variables;
        while !vc.is_null() {
            if vc_isfuncenv!(vc) && (*vc).scope == variable_context {
                break;
            }
            vc = (*vc).down;
        }
        if vc.is_null() {
            return;
        }
        if !((*vc).table).is_null() && vc_haslocals!(vc) {
            delete_all_variables((*vc).table);
            hash_dispose((*vc).table);
        }
        (*vc).table = 0 as *mut libc::c_void as *mut HASH_TABLE;
    }
}

fn free_variable_hash_data(data: *mut libc::c_void) {
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    var = data as *mut SHELL_VAR;
    dispose_variable(var);
}

#[no_mangle]
pub fn delete_all_variables(hashed_vars: *mut HASH_TABLE) {
    unsafe {
        hash_flush(
            hashed_vars,
            ::core::mem::transmute::<Option<fn() -> ()>, sh_free_func_t>(Some(
                ::core::mem::transmute::<fn(*mut libc::c_void) -> (), fn() -> ()>(
                    free_variable_hash_data,
                ),
            )),
        );
    }
}

#[no_mangle]
pub fn set_var_read_only(name: *mut libc::c_char) {
    let mut entry: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    entry = find_variable(name);
    unsafe {
        FIND_OR_MAKE_VARIABLE!(name, entry);
        VSETATTR!(entry, att_readonly);
    }
}

fn vlist_alloc(nentries: libc::c_int) -> *mut VARLIST {
    let mut vlist: *mut VARLIST = 0 as *mut VARLIST;
    unsafe {
        vlist = libc::malloc(std::mem::size_of::<VARLIST>() as usize) as *mut VARLIST;
        (*vlist).list = libc::malloc(
            ((nentries + 1 as libc::c_int) as usize)
                * (std::mem::size_of::<*mut SHELL_VAR>() as usize),
        ) as *mut *mut SHELL_VAR;

        (*vlist).list_size = nentries as usize;
        (*vlist).list_len = 0 as libc::c_int as usize;

        *((*vlist).list).offset(0 as libc::c_int as isize) =
            0 as *mut libc::c_void as *mut SHELL_VAR;
    }
    return vlist;
}

fn vlist_realloc(mut vlist: *mut VARLIST, n: libc::c_int) -> *mut VARLIST {
    if vlist.is_null() {
        vlist = vlist_alloc(n);
        return vlist;
    }
    unsafe {
        if n as size_t > (*vlist).list_size as u64 {
            (*vlist).list_size = n as usize;
            (*vlist).list = libc::realloc(
                (*vlist).list as *mut libc::c_void,
                ((*vlist).list_size + 1 as usize)
                    * (std::mem::size_of::<*mut SHELL_VAR>() as usize),
            ) as *mut *mut SHELL_VAR;
        }
    }
    return vlist;
}

fn vlist_add(mut vlist: *mut VARLIST, var: *mut SHELL_VAR, flags: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    unsafe {
        while (i as size_t) < (*vlist).list_len as u64 {
            if STREQ((*var).name, (**((*vlist).list).offset(i as isize)).name) {
                break;
            }
            i += 1;
            i;
        }
        if (i as size_t) < (*vlist).list_len as u64 {
            return;
        }
        if i as size_t >= (*vlist).list_size as u64 {
            vlist = vlist_realloc(
                vlist,
                ((*vlist).list_size + 16 as libc::c_int as usize) as libc::c_int,
            );
        }

        *((*vlist).list).offset((*vlist).list_len as isize) = var;
        (*vlist).list_len = (*vlist).list_len + 1 as usize;
        *((*vlist).list).offset((*vlist).list_len as isize) =
            0 as *mut libc::c_void as *mut SHELL_VAR;
    }
}

#[no_mangle]
pub fn map_over(function: Option<sh_var_map_func_t>, vc: *mut VAR_CONTEXT) -> *mut *mut SHELL_VAR {
    let mut v: *mut VAR_CONTEXT = 0 as *mut VAR_CONTEXT;
    let mut vlist: *mut VARLIST = 0 as *mut VARLIST;
    let mut ret: *mut *mut SHELL_VAR = 0 as *mut *mut SHELL_VAR;
    let mut nentries: libc::c_int = 0;
    nentries = 0 as libc::c_int;
    v = vc;
    unsafe {
        while !v.is_null() {
            nentries += HASH_ENTRIES!((*v).table);
            v = (*v).down;
        }
        if nentries == 0 as libc::c_int {
            return 0 as *mut libc::c_void as *mut *mut SHELL_VAR;
        }
        vlist = vlist_alloc(nentries);
        v = vc;
        while !v.is_null() {
            flatten((*v).table, function, vlist, 0 as libc::c_int);
            v = (*v).down;
        }
        ret = (*vlist).list;
        free(vlist as *mut libc::c_void);
    }
    return ret;
}

#[no_mangle]
pub fn map_over_funcs(function: Option<sh_var_map_func_t>) -> *mut *mut SHELL_VAR {
    let mut vlist: *mut VARLIST = 0 as *mut VARLIST;
    let mut ret: *mut *mut SHELL_VAR = 0 as *mut *mut SHELL_VAR;
    unsafe {
        if shell_functions.is_null() || HASH_ENTRIES!(shell_functions) == 0 as libc::c_int {
            return 0 as *mut libc::c_void as *mut *mut SHELL_VAR;
        }
        vlist = vlist_alloc(HASH_ENTRIES!(shell_functions));
        flatten(shell_functions, function, vlist, 0 as libc::c_int);
        ret = (*vlist).list;
        free(vlist as *mut libc::c_void);
    }
    return ret;
}

fn flatten(
    var_hash_table: *mut HASH_TABLE,
    func: Option<sh_var_map_func_t>,
    vlist: *mut VARLIST,
    flags: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut tlist: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    let mut r: libc::c_int = 0;
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    unsafe {
        if var_hash_table.is_null()
            || HASH_ENTRIES!(var_hash_table) == 0 as libc::c_int
            || vlist.is_null() && func.is_none()
        {
            return;
        }
        i = 0 as libc::c_int;

        while i < (*var_hash_table).nbuckets {
            tlist = hash_items!(i, var_hash_table);
            while !tlist.is_null() {
                var = (*tlist).data as *mut SHELL_VAR;
                r = if func.is_some() {
                    Some(func.expect("non-null function pointer"))
                        .expect("non-null function pointer")(var)
                } else {
                    1 as libc::c_int
                };
                if r != 0 && !vlist.is_null() {
                    vlist_add(vlist, var, flags);
                }
                tlist = (*tlist).next;
            }
            i += 1;
            i;
        }
    }
}

#[no_mangle]
pub fn sort_variables(array: *mut *mut SHELL_VAR) {
    unsafe {
        qsort(
            array as *mut libc::c_void,
            strvec_len(array as *mut *mut libc::c_char) as usize,
            std::mem::size_of::<*mut SHELL_VAR>() as usize,
            std::mem::transmute::<
                Option<fn(*mut *mut SHELL_VAR, *mut *mut SHELL_VAR) -> libc::c_int>,
                Option<QSFUNC>,
            >(Some(
                qsort_var_comp as fn(*mut *mut SHELL_VAR, *mut *mut SHELL_VAR) -> libc::c_int,
            )),
        )
    }
}

fn qsort_var_comp(var1: *mut *mut SHELL_VAR, var2: *mut *mut SHELL_VAR) -> libc::c_int {
    let mut result: libc::c_int = 0;
    unsafe {
        result = *((**var1).name).offset(0 as libc::c_int as isize) as libc::c_int
            - *((**var2).name).offset(0 as libc::c_int as isize) as libc::c_int;
        if result == 0 as libc::c_int {
            result = libc::strcmp((**var1).name, (**var2).name);
        }
    }
    return result;
}

fn vapply(func: Option<sh_var_map_func_t>) -> *mut *mut SHELL_VAR {
    let mut list: *mut *mut SHELL_VAR = 0 as *mut *mut SHELL_VAR;
    unsafe {
        list = map_over(func, shell_variables);
    }
    if !list.is_null() {
        sort_variables(list);
    }
    return list;
}

fn fapply(func: Option<sh_var_map_func_t>) -> *mut *mut SHELL_VAR {
    let mut list: *mut *mut SHELL_VAR = 0 as *mut *mut SHELL_VAR;
    list = map_over_funcs(func);
    if !list.is_null() {
        sort_variables(list);
    }
    return list;
}

#[no_mangle]
pub fn all_shell_variables() -> *mut *mut SHELL_VAR {
    unsafe {
        return vapply(std::mem::transmute::<
            *mut libc::c_void,
            Option<sh_var_map_func_t>,
        >(0 as *mut libc::c_void));
    }
}
#[no_mangle]
pub fn all_shell_functions() -> *mut *mut SHELL_VAR {
    unsafe {
        return fapply(::core::mem::transmute::<
            *mut libc::c_void,
            Option<sh_var_map_func_t>,
        >(0 as *mut libc::c_void));
    }
}
fn visible_var(var: *mut SHELL_VAR) -> libc::c_int {
    unsafe {
        if invisible_p!(var) == 0 {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        }
    }
}

#[no_mangle]
pub fn all_visible_functions() -> *mut *mut SHELL_VAR {
    unsafe {
        return fapply(::core::mem::transmute::<
            Option<fn() -> libc::c_int>,
            Option<sh_var_map_func_t>,
        >(Some(::core::mem::transmute::<
            fn(*mut SHELL_VAR) -> libc::c_int,
            fn() -> libc::c_int,
        >(visible_var))));
    }
}

#[no_mangle]
pub fn all_visible_variables() -> *mut *mut SHELL_VAR {
    unsafe {
        return vapply(::core::mem::transmute::<
            Option<fn() -> libc::c_int>,
            Option<sh_var_map_func_t>,
        >(Some(::core::mem::transmute::<
            fn(*mut SHELL_VAR) -> libc::c_int,
            fn() -> libc::c_int,
        >(visible_var))));
    }
}
fn visible_and_exported(var: *mut SHELL_VAR) -> libc::c_int {
    unsafe {
        if invisible_p!(var) == 0 && exported_p!(var) != 0 {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        }
    }
}

fn export_environment_candidate(var: *mut SHELL_VAR) -> libc::c_int {
    unsafe {
        if exported_p!(var) != 0 && invisible_p!(var) == 0 as libc::c_int || imported_p!(var) != 0 {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        }
    }
}

fn local_and_exported(var: *mut SHELL_VAR) -> libc::c_int {
    unsafe {
        if invisible_p!(var) == 0 as libc::c_int
            && local_p!(var) != 0
            && (*var).context == variable_context
            && exported_p!(var) != 0
        {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        }
    }
}

#[no_mangle]
pub fn all_exported_variables() -> *mut *mut SHELL_VAR {
    unsafe {
        return vapply(std::mem::transmute::<
            Option<fn() -> libc::c_int>,
            Option<sh_var_map_func_t>,
        >(Some(std::mem::transmute::<
            fn(*mut SHELL_VAR) -> libc::c_int,
            fn() -> libc::c_int,
        >(visible_and_exported))));
    }
}

#[no_mangle]
pub fn local_exported_variables() -> *mut *mut SHELL_VAR {
    unsafe {
        return vapply(::core::mem::transmute::<
            Option<fn() -> libc::c_int>,
            Option<sh_var_map_func_t>,
        >(Some(::core::mem::transmute::<
            fn(*mut SHELL_VAR) -> libc::c_int,
            fn() -> libc::c_int,
        >(local_and_exported))));
    }
}
fn variable_in_context(var: *mut SHELL_VAR) -> libc::c_int {
    unsafe {
        if local_p!(var) != 0 && (*var).context == variable_context {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        }
    }
}
fn visible_variable_in_context(var: *mut SHELL_VAR) -> libc::c_int {
    unsafe {
        if invisible_p!(var) == 0 as libc::c_int
            && local_p!(var) != 0
            && (*var).context == variable_context
        {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        }
    }
}

#[no_mangle]
pub fn all_local_variables(visible_only: libc::c_int) -> *mut *mut SHELL_VAR {
    let mut vlist: *mut VARLIST = 0 as *mut VARLIST;
    let mut ret: *mut *mut SHELL_VAR = 0 as *mut *mut SHELL_VAR;
    let mut vc: *mut VAR_CONTEXT = 0 as *mut VAR_CONTEXT;
    unsafe {
        vc = shell_variables;
        vc = shell_variables;
        while !vc.is_null() {
            if vc_isfuncenv!(vc) && (*vc).scope == variable_context {
                break;
            }
            vc = (*vc).down;
        }
        if vc.is_null() {
            internal_error(
                b"all_local_variables: no function context at current scope\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as *mut libc::c_void as *mut *mut SHELL_VAR;
        }
        if ((*vc).table).is_null()
            || HASH_ENTRIES!((*vc).table) == 0 as libc::c_int
            || vc_haslocals!(vc)
        {
            return 0 as *mut libc::c_void as *mut *mut SHELL_VAR;
        }
        vlist = vlist_alloc(HASH_ENTRIES!((*vc).table));
        if visible_only != 0 {
            flatten(
                (*vc).table,
                ::core::mem::transmute::<Option<fn() -> libc::c_int>, Option<sh_var_map_func_t>>(
                    Some(::core::mem::transmute::<
                        fn(*mut SHELL_VAR) -> libc::c_int,
                        fn() -> libc::c_int,
                    >(visible_variable_in_context)),
                ),
                vlist,
                0 as libc::c_int,
            );
        } else {
            flatten(
                (*vc).table,
                ::core::mem::transmute::<Option<fn() -> libc::c_int>, Option<sh_var_map_func_t>>(
                    Some(::core::mem::transmute::<
                        fn(*mut SHELL_VAR) -> libc::c_int,
                        fn() -> libc::c_int,
                    >(variable_in_context)),
                ),
                vlist,
                0 as libc::c_int,
            );
        }
        ret = (*vlist).list;
        free(vlist as *mut libc::c_void);
        if !ret.is_null() {
            sort_variables(ret);
        }
    }
    return ret;
}

fn visible_array_vars(var: *mut SHELL_VAR) -> libc::c_int {
    unsafe {
        if invisible_p!(var) == 0 as libc::c_int && (array_p!(var) != 0 || assoc_p!(var) != 0) {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        }
    }
}

#[no_mangle]
pub fn all_array_variables() -> *mut *mut SHELL_VAR {
    unsafe {
        return vapply(std::mem::transmute::<
            Option<fn() -> libc::c_int>,
            Option<sh_var_map_func_t>,
        >(Some(std::mem::transmute::<
            fn(*mut SHELL_VAR) -> libc::c_int,
            fn() -> libc::c_int,
        >(visible_array_vars))));
    }
}

#[no_mangle]
pub fn all_variables_matching_prefix(prefix: *const libc::c_char) -> *mut *mut libc::c_char {
    let mut varlist: *mut *mut SHELL_VAR = 0 as *mut *mut SHELL_VAR;
    let mut rlist: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut vind: libc::c_int = 0;
    let mut rind: libc::c_int = 0;
    let mut plen: libc::c_int = 0;
    unsafe {
        plen = STRLEN!(prefix);

        varlist = all_visible_variables();
        vind = 0 as libc::c_int;

        while !varlist.is_null() && !(*varlist.offset(vind as isize)).is_null() {
            vind += 1;
            vind;
        }
        if varlist.is_null() || vind == 0 as libc::c_int {
            return 0 as *mut libc::c_void as *mut *mut libc::c_char;
        }
        rlist = strvec_create(vind + 1 as libc::c_int);
        rind = 0 as libc::c_int;
        vind = rind;
        while !(*varlist.offset(vind as isize)).is_null() {
            if plen == 0 as libc::c_int
                || STREQN(prefix, (**varlist.offset(vind as isize)).name, plen)
            {
                *rlist.offset(rind as isize) = savestring!((**varlist.offset(vind as isize)).name);
                rind += 1;
            }
            vind += 1;
        }
        *rlist.offset(rind as isize) = 0 as *mut libc::c_char;
        free(varlist as *mut libc::c_void);
    }
    return rlist;
}

fn bind_tempenv_variable(name: *const libc::c_char, value: *mut libc::c_char) -> *mut SHELL_VAR {
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    unsafe {
        var = if !temporary_env.is_null() {
            hash_lookup(name, temporary_env)
        } else {
            0 as *mut libc::c_void as *mut SHELL_VAR
        };

        if !var.is_null() {
            if !((*var).value).is_null() {
                free((*var).value as *mut libc::c_void);
            }
            var_setvalue!(var, savestring!(value));
            INVALIDATE_EXPORTSTR!(var);
        }
    }
    return var;
}

#[no_mangle]
pub fn find_tempenv_variable(name: *const libc::c_char) -> *mut SHELL_VAR {
    unsafe {
        return if !temporary_env.is_null() {
            hash_lookup(name, temporary_env)
        } else {
            0 as *mut libc::c_void as *mut SHELL_VAR
        };
    }
}
#[no_mangle]
pub static mut tempvar_list: *mut *mut libc::c_char =
    0 as *const *mut libc::c_char as *mut *mut libc::c_char;

#[no_mangle]
pub static mut tvlist_ind: libc::c_int = 0;
fn push_posix_temp_var(data: *mut libc::c_void) {
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut binding_table: *mut HASH_TABLE = 0 as *mut HASH_TABLE;

    var = data as *mut SHELL_VAR;
    unsafe {
        v = bind_variable(
            (*var).name,
            value_cell!(var),
            ASS_FORCE as libc::c_int | ASS_NOLONGJMP as libc::c_int,
        );

        binding_table = if (*v).context != 0 {
            (*shell_variables).table
        } else {
            (*global_variables).table
        };
        if (*v).context == 0 as libc::c_int {
            (*var).attributes &= !(att_tempvar as libc::c_int | att_propagate as libc::c_int);
        }
        if !v.is_null() {
            (*v).attributes |= (*var).attributes;
            if (*v).context > 0 as libc::c_int && local_p!(v) == 0 as libc::c_int {
                (*v).attributes |= att_propagate as libc::c_int;
            } else {
                (*v).attributes &= !(att_propagate as libc::c_int);
            }
        }
        if find_special_var((*var).name) >= 0 as libc::c_int {
            *tempvar_list.offset(tvlist_ind as isize) = savestring!((*var).name);
            tvlist_ind += 1;
        }
    }
    dispose_variable(var);
}

fn push_temp_var(data: *mut libc::c_void) {
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut binding_table: *mut HASH_TABLE = 0 as *mut HASH_TABLE;
    var = data as *mut SHELL_VAR;
    unsafe {
        binding_table = (*shell_variables).table;
        if binding_table.is_null() {
            if shell_variables == global_variables {
                (*global_variables).table = hash_create(VARIABLES_HASH_BUCKETS!() as libc::c_int);
                (*shell_variables).table = (*global_variables).table;
                binding_table = (*shell_variables).table;
            } else {
                (*shell_variables).table = hash_create(TEMPENV_HASH_BUCKETS!() as libc::c_int);
                binding_table = (*shell_variables).table;
            }
        }

        v = bind_variable_internal(
            (*var).name,
            value_cell!(var),
            binding_table,
            0 as libc::c_int,
            ASS_FORCE as libc::c_int | ASS_NOLONGJMP as libc::c_int,
        );
        if !v.is_null() {
            (*v).context = (*shell_variables).scope;
        }
        if binding_table == (*global_variables).table {
            (*var).attributes &= !(att_tempvar as libc::c_int | att_propagate as libc::c_int);
        } else {
            (*var).attributes |= att_propagate as libc::c_int;
            if binding_table == (*shell_variables).table {
                (*shell_variables).flags |= VC_HASTMPVAR as libc::c_int;
            }
        }
        if !v.is_null() {
            (*v).attributes |= (*var).attributes;
        }
        if find_special_var((*var).name) >= 0 as libc::c_int {
            *tempvar_list.offset(tvlist_ind as isize) = savestring!((*var).name);
            tvlist_ind += 1;
        }
    }
    dispose_variable(var);
}

fn propagate_temp_var(data: *mut libc::c_void) {
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    var = data as *mut SHELL_VAR;
    unsafe {
        if tempvar_p!(var) != 0 && (*var).attributes & att_propagate as libc::c_int != 0 {
            push_temp_var(data);
        } else {
            if find_special_var((*var).name) >= 0 as libc::c_int {
                *tempvar_list.offset(tvlist_ind as isize) = savestring!((*var).name);
                tvlist_ind += 1;
            }
            dispose_variable(var);
        };
    }
}

fn dispose_temporary_env(pushf: sh_free_func_t) {
    let mut i: libc::c_int = 0;
    let mut disposer: *mut HASH_TABLE = 0 as *mut HASH_TABLE;
    unsafe {
        tempvar_list = strvec_create(HASH_ENTRIES!(temporary_env) + 1 as libc::c_int);
        tvlist_ind = 0 as libc::c_int;
        *tempvar_list.offset(tvlist_ind as isize) = 0 as *mut libc::c_void as *mut libc::c_char;

        disposer = temporary_env;
        temporary_env = 0 as *mut libc::c_void as *mut HASH_TABLE;

        hash_flush(disposer, pushf);
        hash_dispose(disposer);

        *tempvar_list.offset(tvlist_ind as isize) = 0 as *mut libc::c_char;

        array_needs_making = 1 as libc::c_int;
        i = 0 as libc::c_int;
        while i < tvlist_ind {
            stupidly_hack_special_variables(*tempvar_list.offset(i as isize));
            i += 1;
            i;
        }
        strvec_dispose(tempvar_list);
        tempvar_list = 0 as *mut *mut libc::c_char;
        tvlist_ind = 0 as libc::c_int;
    }
}

#[no_mangle]
pub fn dispose_used_env_vars() {
    unsafe {
        if !temporary_env.is_null() {
            dispose_temporary_env(
                ::core::mem::transmute::<Option<fn() -> ()>, sh_free_func_t>(Some(
                    ::core::mem::transmute::<fn(*mut libc::c_void) -> (), fn() -> ()>(
                        propagate_temp_var,
                    ),
                )),
            );
            maybe_make_export_env();
        }
    }
}

#[no_mangle]
pub fn merge_temporary_env() {
    unsafe {
        if !temporary_env.is_null() {
            dispose_temporary_env(
                ::core::mem::transmute::<Option<fn() -> ()>, sh_free_func_t>(
                    if posixly_correct != 0 {
                        Some(::core::mem::transmute::<
                            fn(*mut libc::c_void) -> (),
                            fn() -> (),
                        >(push_posix_temp_var))
                    } else {
                        Some(::core::mem::transmute::<
                            fn(*mut libc::c_void) -> (),
                            fn() -> (),
                        >(push_temp_var))
                    },
                ),
            );
        }
    }
}
#[no_mangle]
pub fn merge_function_temporary_env() {
    unsafe {
        if !temporary_env.is_null() {
            dispose_temporary_env(
                ::core::mem::transmute::<Option<fn() -> ()>, sh_free_func_t>(Some(
                    ::core::mem::transmute::<fn(*mut libc::c_void) -> (), fn() -> ()>(
                        push_temp_var,
                    ),
                )),
            );
        }
    }
}

#[no_mangle]
pub fn flush_temporary_env() {
    unsafe {
        if !temporary_env.is_null() {
            hash_flush(
                temporary_env,
                ::core::mem::transmute::<Option<fn() -> ()>, sh_free_func_t>(Some(
                    ::core::mem::transmute::<fn(*mut libc::c_void) -> (), fn() -> ()>(
                        free_variable_hash_data,
                    ),
                )),
            );
            hash_dispose(temporary_env);
            temporary_env = 0 as *mut libc::c_void as *mut HASH_TABLE;
        }
    }
}

#[inline]
fn mk_env_string(
    name: *const libc::c_char,
    value: *const libc::c_char,
    isfunc: libc::c_int,
) -> *mut libc::c_char {
    let mut name_len: size_t = 0;
    let mut value_len: size_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        name_len = strlen(name) as size_t;
        value_len = STRLEN!(value) as size_t;

        if isfunc != 0 && !value.is_null() {
            p = libc::malloc(
                BASHFUNC_PREFLEN!() as usize
                    + name_len as usize
                    + BASHFUNC_SUFFLEN!() as usize
                    + value_len as usize
                    + 2 as usize,
            ) as *mut libc::c_char;
            q = p;
            memcpy(
                q as *mut libc::c_void,
                BASHFUNC_PREFIX!() as *const c_void,
                BASHFUNC_PREFLEN!() as usize,
            );
            q = q.offset(10 as libc::c_int as isize);
            memcpy(
                q as *mut libc::c_void,
                name as *const libc::c_void,
                name_len as usize,
            );
            q = q.offset(name_len as isize);
            memcpy(
                q as *mut libc::c_void,
                b"%%\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                BASHFUNC_SUFFLEN!() as libc::c_int as usize,
            );
            q = q.offset(BASHFUNC_SUFFLEN!() as libc::c_int as isize);
        } else {
            p = libc::malloc(2 as usize + name_len as usize + value_len as usize)
                as *mut libc::c_char;
            memcpy(
                p as *mut libc::c_void,
                name as *const libc::c_void,
                name_len as usize,
            );
            q = p.offset(name_len as isize);
        }

        *q.offset(0 as libc::c_int as isize) = '=' as i32 as libc::c_char;

        if !value.is_null() && *value as libc::c_int != 0 {
            if isfunc != 0 {
                t = dequote_escapes(value);
                value_len = STRLEN!(t) as u64;
                memcpy(
                    q.offset(1 as libc::c_int as isize) as *mut libc::c_void,
                    t as *const libc::c_void,
                    (value_len + 1) as libc::c_int as usize,
                );
                free(t as *mut libc::c_void);
            } else {
                memcpy(
                    q.offset(1 as libc::c_int as isize) as *mut libc::c_void,
                    value as *const libc::c_void,
                    (value_len + 1) as libc::c_int as usize,
                );
            }
        } else {
            *q.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        }
    }
    return p;
}

fn make_env_array_from_var_list(vars: *mut *mut SHELL_VAR) -> *mut *mut libc::c_char {
    unsafe {
        let mut i: libc::c_int = 0;
        let mut list_index: libc::c_int = 0;
        let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
        let mut list: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut value: *mut libc::c_char = 0 as *mut libc::c_char;

        list = strvec_create(1 as libc::c_int + strvec_len(vars as *mut *mut libc::c_char));
        let mut current_block_19: u64;
        i = 0 as libc::c_int;
        list_index = 0 as libc::c_int;

        loop {
            var = *vars.offset(i as isize);
            if var.is_null() {
                break;
            }
            if regen_p!(var) != 0 && ((*var).dynamic_value).is_some() {
                var = (Some(((*var).dynamic_value).expect("non-null function pointer")))
                    .expect("non-null function pointer")(var);
                INVALIDATE_EXPORTSTR!(var);
            }
            if !((*var).exportstr).is_null() {
                value = (*var).exportstr;
            } else if function_p!(var) != 0 {
                value = named_function_string(
                    0 as *mut libc::c_void as *mut libc::c_char,
                    function_cell!(var),
                    0 as libc::c_int,
                );
            } else if array_p!(var) != 0 {
                i += 1;
                continue;
            } else if assoc_p!(var) != 0 {
                i += 1;
                continue;
            } else {
                value = (*var).value;
            }
            if !value.is_null() {
                *list.offset(list_index as isize) = if value == (*var).exportstr {
                    savestring!(value)
                } else {
                    mk_env_string((*var).name, value, function_p!(var) as libc::c_int)
                };

                if value != (*var).exportstr {
                    SAVE_EXPORTSTR!(var, (*list).offset(list_index as isize))
                }
                list_index += 1;
                list_index;
            }
            i += 1;
            i;
        }

        *(list.offset((list_index as size_t).try_into().unwrap())) =
            0 as *mut libc::c_void as *mut libc::c_char;

        return list;
    }
}

fn make_var_export_array(vcxt: *mut VAR_CONTEXT) -> *mut *mut libc::c_char {
    let mut list: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut vars: *mut *mut SHELL_VAR = 0 as *mut *mut SHELL_VAR;
    unsafe {
        vars = map_over(
            std::mem::transmute::<Option<fn() -> libc::c_int>, Option<sh_var_map_func_t>>(Some(
                std::mem::transmute::<fn(*mut SHELL_VAR) -> libc::c_int, fn() -> libc::c_int>(
                    export_environment_candidate,
                ),
            )),
            vcxt,
        );
        if vars.is_null() {
            return 0 as *mut libc::c_void as *mut *mut libc::c_char;
        }
        list = make_env_array_from_var_list(vars);
        free(vars as *mut libc::c_void);
    }
    return list;
}

fn make_func_export_array() -> *mut *mut libc::c_char {
    let mut list: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut vars: *mut *mut SHELL_VAR = 0 as *mut *mut SHELL_VAR;
    unsafe {
        vars = map_over_funcs(std::mem::transmute::<
            Option<fn() -> libc::c_int>,
            Option<sh_var_map_func_t>,
        >(Some(std::mem::transmute::<
            fn(*mut SHELL_VAR) -> libc::c_int,
            fn() -> libc::c_int,
        >(visible_and_exported))));
        if vars.is_null() {
            return 0 as *mut libc::c_void as *mut *mut libc::c_char;
        }
        list = make_env_array_from_var_list(vars);
        free(vars as *mut libc::c_void);
    }
    return list;
}

#[no_mangle]
pub fn add_or_supercede_exported_var(
    assign: *mut libc::c_char,
    do_alloc: libc::c_int,
) -> *mut *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut equal_offset: libc::c_int = 0;
    unsafe {
        equal_offset = assignment(assign, 0 as libc::c_int);
        if equal_offset == 0 as libc::c_int {
            return export_env;
        }
        if *assign.offset((equal_offset + 1 as libc::c_int) as isize) as libc::c_int == '(' as i32
            && libc::strncmp(
                assign
                    .offset(equal_offset as isize)
                    .offset(2 as libc::c_int as isize),
                b") {\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int as usize,
            ) == 0 as libc::c_int
        {
            equal_offset += 4 as libc::c_int;
        }
        i = 0 as libc::c_int;
        while i < export_env_index {
            if STREQN(
                assign,
                *export_env.offset(i as isize) as *mut libc::c_char,
                equal_offset + 1,
            ) {
                free(*export_env.offset(i as isize) as *mut libc::c_void);
                *export_env.offset(i as isize) = if do_alloc != 0 {
                    savestring!(assign)
                } else {
                    assign
                };
                return export_env;
            }
            i += 1;
            i;
        }

        add_to_export_env!(assign, do_alloc);
        return export_env;
    }
}

fn add_temp_array_to_env(
    temp_array: *mut *mut libc::c_char,
    do_alloc: libc::c_int,
    do_supercede: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    if temp_array.is_null() {
        return;
    }
    i = 0 as libc::c_int;
    unsafe {
        while !(*temp_array.offset(i as isize)).is_null() {
            if do_supercede != 0 {
                export_env =
                    add_or_supercede_exported_var(*temp_array.offset(i as isize), do_alloc);
            } else {
                add_to_export_env!(*temp_array.offset(i as isize), do_alloc);
            }
            i += 1;
        }
        free(temp_array as *mut libc::c_void);
    }
}

fn n_shell_variables() -> libc::c_int {
    let mut vc: *mut VAR_CONTEXT = 0 as *mut VAR_CONTEXT;
    let mut n: libc::c_int = 0;
    unsafe {
        n = 0 as libc::c_int;
        vc = shell_variables;

        while !vc.is_null() {
            n += HASH_ENTRIES!((*vc).table);
            vc = (*vc).down;
        }
    }
    return n;
}

#[no_mangle]
pub fn chkexport(name: *mut libc::c_char) -> libc::c_int {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    v = find_variable(name);
    unsafe {
        if !v.is_null() && exported_p!(v) != 0 {
            array_needs_making = 1 as libc::c_int;
            maybe_make_export_env();
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub fn maybe_make_export_env() {
    let mut temp_array: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut new_size: libc::c_int = 0;
    let mut tcxt: *mut VAR_CONTEXT = 0 as *mut VAR_CONTEXT;
    let mut icxt: *mut VAR_CONTEXT = 0 as *mut VAR_CONTEXT;
    unsafe {
        if array_needs_making != 0 {
            if !export_env.is_null() {
                strvec_flush(export_env);
            }
            new_size = n_shell_variables()
                + HASH_ENTRIES!(shell_functions)
                + 1 as libc::c_int
                + HASH_ENTRIES!(temporary_env)
                + HASH_ENTRIES!(invalid_env);
            if new_size > export_env_size {
                export_env_size = new_size;
                export_env = strvec_resize(export_env, export_env_size);
                environ = export_env;
            }
            export_env_index = 0 as libc::c_int;
            *export_env.offset(export_env_index as isize) =
                0 as *mut libc::c_void as *mut libc::c_char;

            if !temporary_env.is_null() {
                tcxt = new_var_context(
                    0 as *mut libc::c_void as *mut libc::c_char,
                    0 as libc::c_int,
                );
                (*tcxt).table = temporary_env;
                (*tcxt).down = shell_variables;
            } else {
                tcxt = shell_variables;
            }
            if !invalid_env.is_null() {
                icxt = new_var_context(
                    0 as *mut libc::c_void as *mut libc::c_char,
                    0 as libc::c_int,
                );
                (*icxt).table = invalid_env;
                (*icxt).down = tcxt;
            } else {
                icxt = tcxt;
            }
            temp_array = make_var_export_array(icxt);
            if !temp_array.is_null() {
                add_temp_array_to_env(temp_array, 0 as libc::c_int, 0 as libc::c_int);
            }
            if icxt != tcxt {
                free(icxt as *mut libc::c_void);
            }
            if tcxt != shell_variables {
                free(tcxt as *mut libc::c_void);
            }
            temp_array = if restricted != 0 {
                0 as *mut *mut libc::c_char
            } else {
                make_func_export_array()
            };
            if !temp_array.is_null() {
                add_temp_array_to_env(temp_array, 0 as libc::c_int, 0 as libc::c_int);
            }
            array_needs_making = 0 as libc::c_int;
        }
    }
}

#[no_mangle]
pub fn update_export_env_inplace(
    env_prefix: *mut libc::c_char,
    preflen: libc::c_int,
    value: *mut libc::c_char,
) {
    let mut evar: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        evar = libc::malloc((STRLEN!(value) + preflen + 1) as usize) as *mut libc::c_char;
        libc::strcpy(evar, env_prefix);
        if !value.is_null() {
            libc::strcpy(evar.offset(preflen as isize), value);
        }
        export_env = add_or_supercede_exported_var(evar, 0 as libc::c_int);
    }
}

#[no_mangle]
pub fn put_command_name_into_env(command_name: *mut libc::c_char) {
    update_export_env_inplace(
        b"_=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int,
        command_name,
    );
}

#[no_mangle]
pub fn new_var_context(name: *mut libc::c_char, flags: libc::c_int) -> *mut VAR_CONTEXT {
    let mut vc: *mut VAR_CONTEXT = 0 as *mut VAR_CONTEXT;
    unsafe {
        vc = libc::malloc(std::mem::size_of::<VAR_CONTEXT>() as usize) as *mut VAR_CONTEXT;

        (*vc).name = if !name.is_null() {
            savestring!(name)
        } else {
            0 as *mut libc::c_void as *mut libc::c_char
        };

        (*vc).scope = variable_context;
        (*vc).flags = flags;
        (*vc).down = 0 as *mut libc::c_void as *mut VAR_CONTEXT;
        (*vc).up = (*vc).down;
        (*vc).table = 0 as *mut libc::c_void as *mut HASH_TABLE;
    }
    return vc;
}

#[no_mangle]
pub fn dispose_var_context(vc: *mut VAR_CONTEXT) {
    unsafe {
        if !((*vc).name).is_null() {
            free((*vc).name as *mut libc::c_void);
        }
        (*vc).name = 0 as *mut libc::c_char;
        if !((*vc).table).is_null() {
            delete_all_variables((*vc).table);
            hash_dispose((*vc).table);
        }
        free(vc as *mut libc::c_void);
    }
}

fn set_context(var: *mut SHELL_VAR) -> libc::c_int {
    unsafe {
        (*var).context = variable_context;
        return (*var).context;
    }
}

#[no_mangle]
pub fn push_var_context(
    name: *mut libc::c_char,
    flags: libc::c_int,
    tempvars: *mut HASH_TABLE,
) -> *mut VAR_CONTEXT {
    let mut vc: *mut VAR_CONTEXT = 0 as *mut VAR_CONTEXT;
    let mut posix_func_behavior: libc::c_int = 0;
    posix_func_behavior = 0 as libc::c_int;

    vc = new_var_context(name, flags);
    unsafe {
        if posix_func_behavior != 0
            && (flags & VC_FUNCENV!()) as libc::c_int != 0
            && tempvars == temporary_env
        {
            merge_temporary_env();
        } else if !tempvars.is_null() {
            (*vc).table = tempvars;
            flatten(
                tempvars,
                std::mem::transmute::<Option<fn() -> libc::c_int>, Option<sh_var_map_func_t>>(
                    Some(std::mem::transmute::<
                        fn(*mut SHELL_VAR) -> libc::c_int,
                        fn() -> libc::c_int,
                    >(set_context)),
                ),
                0 as *mut libc::c_void as *mut VARLIST,
                0 as libc::c_int,
            );
            (*vc).flags |= VC_HASTMPVAR as libc::c_int;
        }
        (*vc).down = shell_variables;
        (*shell_variables).up = vc;
        shell_variables = vc;
        return shell_variables;
    }
}

#[inline]
fn push_posix_tempvar_internal(var: *mut SHELL_VAR, isbltin: libc::c_int) {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut posix_var_behavior: libc::c_int = 0;
    unsafe {
        posix_var_behavior = (posixly_correct != 0 && isbltin != 0) as libc::c_int;
        v = 0 as *mut SHELL_VAR;

        if local_p!(var) != 0 && STREQ((*var).name, b"-\0" as *const u8 as *const libc::c_char) {
            set_current_options(value_cell!(var));
            set_shellopts();
        } else if tempvar_p!(var) != 0 && posix_var_behavior != 0 {
            v = bind_variable(
                (*var).name,
                value_cell!(var),
                ASS_FORCE as libc::c_int | ASS_NOLONGJMP as libc::c_int,
            );
            if !v.is_null() {
                (*v).attributes |= (*var).attributes;
                if (*v).context == 0 as libc::c_int {
                    (*v).attributes &= !(att_tempvar as libc::c_int | att_propagate as libc::c_int);
                }
            }
        } else if tempvar_p!(var) != 0 && propagate_p!(var) != 0 {
            if (vc_isfuncenv!(shell_variables) || vc_istempenv!(shell_variables))
                && ((*shell_variables).table).is_null()
            {
                (*shell_variables).table = hash_create(VARIABLES_HASH_BUCKETS!() as libc::c_int);
            }

            v = bind_variable_internal(
                (*var).name,
                value_cell!(var),
                (*shell_variables).table,
                0 as libc::c_int,
                0 as libc::c_int,
            );
            if !v.is_null() {
                (*v).context = (*shell_variables).scope;
            }
            if shell_variables == global_variables {
                (*var).attributes &= !(att_tempvar as libc::c_int | att_propagate as libc::c_int);
            } else {
                (*shell_variables).flags |= VC_HASTMPVAR as libc::c_int;
            }
            if !v.is_null() {
                (*v).attributes |= (*var).attributes;
            }
        } else {
            stupidly_hack_special_variables((*var).name);
        }
        if !v.is_null() && (array_p!(var) != 0 || assoc_p!(var) != 0) {
            if !((*v).value).is_null() {
                free((*v).value as *mut libc::c_void);
            }
            (*v).value = 0 as *mut libc::c_char;
            if array_p!(var) != 0 {
                var_setarray!(v, array_copy(array_cell!(var)) as *mut libc::c_char);
            } else {
                var_setassoc!(v, assoc_copy!(assoc_cell!(var)));
            }
        }
    }
    dispose_variable(var);
}

fn push_func_var(data: *mut libc::c_void) {
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    var = data as *mut SHELL_VAR;
    push_posix_tempvar_internal(var, 0 as libc::c_int);
}

fn push_builtin_var(data: *mut libc::c_void) {
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    var = data as *mut SHELL_VAR;
    push_posix_tempvar_internal(var, 1 as libc::c_int);
}

#[no_mangle]
pub fn pop_var_context() {
    let mut ret: *mut VAR_CONTEXT = 0 as *mut VAR_CONTEXT;
    let mut vcxt: *mut VAR_CONTEXT = 0 as *mut VAR_CONTEXT;
    unsafe {
        vcxt = shell_variables;

        if !vc_isfuncenv!(vcxt) {
            internal_error(
                b"pop_var_context: head of shell_variables not a function context\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
        ret = (*vcxt).down;

        if !ret.is_null() {
            (*ret).up = 0 as *mut libc::c_void as *mut VAR_CONTEXT;
            shell_variables = ret;
            if !((*vcxt).table).is_null() {
                hash_flush(
                    (*vcxt).table,
                    std::mem::transmute::<Option<fn() -> ()>, sh_free_func_t>(Some(
                        std::mem::transmute::<fn(*mut libc::c_void) -> (), fn() -> ()>(
                            push_func_var,
                        ),
                    )),
                );
            }
            dispose_var_context(vcxt);
        } else {
            internal_error(
                b"pop_var_context: no global_variables context\0" as *const u8
                    as *const libc::c_char,
            );
        };
    }
}

#[no_mangle]
pub fn delete_all_contexts(vcxt: *mut VAR_CONTEXT) {
    let mut v: *mut VAR_CONTEXT = 0 as *mut VAR_CONTEXT;
    let mut t: *mut VAR_CONTEXT = 0 as *mut VAR_CONTEXT;
    v = vcxt;
    unsafe {
        while v != global_variables {
            t = (*v).down;
            dispose_var_context(v);
            v = t;
        }
        delete_all_variables((*global_variables).table);
        shell_variables = global_variables;
    }
}

#[no_mangle]
pub fn push_scope(flags: libc::c_int, tmpvars: *mut HASH_TABLE) -> *mut VAR_CONTEXT {
    return push_var_context(0 as *mut libc::c_void as *mut libc::c_char, flags, tmpvars);
}

fn push_exported_var(data: *mut libc::c_void) {
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    var = data as *mut SHELL_VAR;
    unsafe {
        if tempvar_p!(var) != 0
            && exported_p!(var) != 0
            && (*var).attributes & att_propagate as libc::c_int != 0
        {
            (*var).attributes &= !(att_tempvar as libc::c_int);
            v = bind_variable_internal(
                (*var).name,
                value_cell!(var),
                (*shell_variables).table,
                0 as libc::c_int,
                0 as libc::c_int,
            );
            if shell_variables == global_variables {
                (*var).attributes &= !(att_propagate as libc::c_int);
            }
            if !v.is_null() {
                (*v).attributes |= (*var).attributes;
                (*v).context = (*shell_variables).scope;
            }
        } else {
            stupidly_hack_special_variables((*var).name);
        }
    }
    dispose_variable(var);
}

#[no_mangle]
pub fn pop_scope(is_special: libc::c_int) {
    let mut vcxt: *mut VAR_CONTEXT = 0 as *mut VAR_CONTEXT;
    let mut ret: *mut VAR_CONTEXT = 0 as *mut VAR_CONTEXT;
    let mut is_bltinenv: libc::c_int = 0;
    unsafe {
        vcxt = shell_variables;
        if !vc_istempscope!(vcxt) {
            internal_error(
                b"pop_scope: head of shell_variables not a temporary environment scope\0"
                    as *const u8 as *const libc::c_char,
            );
            return;
        }

        is_bltinenv = if vc_isbltnenv!(vcxt) {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };

        ret = (*vcxt).down;
        if !ret.is_null() {
            (*ret).up = 0 as *mut libc::c_void as *mut VAR_CONTEXT;
        }
        shell_variables = ret;
        if !((*vcxt).name).is_null() {
            free((*vcxt).name as *mut libc::c_void);
        }
        (*vcxt).name = 0 as *mut libc::c_char;

        if !((*vcxt).table.is_null()) {
            if is_special != 0 {
                hash_flush(
                    (*vcxt).table,
                    std::mem::transmute::<Option<fn() -> ()>, sh_free_func_t>(Some(
                        ::core::mem::transmute::<fn(*mut libc::c_void) -> (), fn() -> ()>(
                            push_builtin_var,
                        ),
                    )),
                );
            } else {
                hash_flush(
                    (*vcxt).table,
                    std::mem::transmute::<Option<fn() -> ()>, sh_free_func_t>(Some(
                        ::core::mem::transmute::<fn(*mut libc::c_void) -> (), fn() -> ()>(
                            push_exported_var,
                        ),
                    )),
                );
            }
            hash_dispose((*vcxt).table);
        }
        free(vcxt as *mut libc::c_void);
        sv_ifs(b"IFS\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    }
}

static mut dollar_arg_stack: *mut saved_dollar_vars =
    0 as *const libc::c_void as *mut libc::c_void as *mut saved_dollar_vars;
static mut dollar_arg_stack_slots: libc::c_int = 0;
static mut dollar_arg_stack_index: libc::c_int = 0;

fn save_dollar_vars() -> *mut *mut libc::c_char {
    let mut ret: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut i: libc::c_int = 0;
    unsafe {
        ret = strvec_create(10 as libc::c_int);
        i = 1 as libc::c_int;
        while i < 10 as libc::c_int {
            *ret.offset(i as isize) = dollar_vars[i as usize];
            dollar_vars[i as usize] = 0 as *mut libc::c_void as *mut libc::c_char;
            i += 1;
            i;
        }
    }
    return ret;
}

fn restore_dollar_vars(args: *mut *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    unsafe {
        while i < 10 as libc::c_int {
            dollar_vars[i as usize] = *args.offset(i as isize);
            i += 1;
            i;
        }
    }
}

fn free_dollar_vars() {
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    unsafe {
        while i < 10 as libc::c_int {
            if !(dollar_vars[i as usize]).is_null() {
                free(dollar_vars[i as usize] as *mut libc::c_void);
            }

            dollar_vars[i as usize] = 0 as *mut libc::c_void as *mut libc::c_char;
            i += 1;
            i;
        }
    }
}
fn free_saved_dollar_vars(args: *mut *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    unsafe {
        while i < 10 as libc::c_int {
            if !(*args.offset(i as isize)).is_null() {
                free(*args.offset(i as isize) as *mut libc::c_void);
            }
            // *args.offset(i as isize) = 0 as *mut libc::c_char;
            i += 1;
            i;
        }
    }
}

#[no_mangle]
pub fn clear_dollar_vars() {
    free_dollar_vars();
    unsafe {
        dispose_words(rest_of_args);
        rest_of_args = 0 as *mut libc::c_void as *mut WORD_LIST;
        posparam_count = 0 as libc::c_int;
    }
}

#[no_mangle]
pub fn push_context(name: *mut libc::c_char, is_subshell: libc::c_int, tempvars: *mut HASH_TABLE) {
    if is_subshell == 0 as libc::c_int {
        push_dollar_vars();
    }
    unsafe {
        variable_context += 1;
        variable_context;
    }
    push_var_context(name, 0x4 as libc::c_int, tempvars);
}

#[no_mangle]
pub fn pop_context() {
    pop_dollar_vars();
    unsafe {
        variable_context -= 1;
        variable_context;
    }
    pop_var_context();
    sv_ifs(b"IFS\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
}

#[no_mangle]
pub fn push_dollar_vars() {
    unsafe {
        if dollar_arg_stack_index + 2 as libc::c_int > dollar_arg_stack_slots {
            dollar_arg_stack_slots += 10 as libc::c_int;
            dollar_arg_stack = libc::realloc(
                dollar_arg_stack as *mut libc::c_void,
                (dollar_arg_stack_slots as usize)
                    * std::mem::size_of::<saved_dollar_vars>() as usize,
            ) as *mut saved_dollar_vars;
        }
        (*dollar_arg_stack.offset(dollar_arg_stack_index as isize)).count = posparam_count;
        (*dollar_arg_stack.offset(dollar_arg_stack_index as isize)).first_ten = save_dollar_vars();
        (*dollar_arg_stack.offset(dollar_arg_stack_index as isize)).rest = rest_of_args;
        dollar_arg_stack_index = dollar_arg_stack_index + 1;

        rest_of_args = 0 as *mut libc::c_void as *mut WORD_LIST;
        posparam_count = 0 as libc::c_int;

        (*dollar_arg_stack.offset(dollar_arg_stack_index as isize)).first_ten =
            0 as *mut libc::c_void as *mut *mut libc::c_char;

        (*dollar_arg_stack.offset(dollar_arg_stack_index as isize)).rest =
            0 as *mut libc::c_void as *mut WORD_LIST;
    }
}

#[no_mangle]
pub fn pop_dollar_vars() {
    unsafe {
        if dollar_arg_stack.is_null() || dollar_arg_stack_index == 0 as libc::c_int {
            return;
        }
        clear_dollar_vars();
        dollar_arg_stack_index -= 1;
        rest_of_args = (*dollar_arg_stack.offset(dollar_arg_stack_index as isize)).rest;
        restore_dollar_vars((*dollar_arg_stack.offset(dollar_arg_stack_index as isize)).first_ten);
        free(
            (*dollar_arg_stack.offset(dollar_arg_stack_index as isize)).first_ten
                as *mut libc::c_void,
        );

        posparam_count = (*dollar_arg_stack.offset(dollar_arg_stack_index as isize)).count;
        (*dollar_arg_stack.offset(dollar_arg_stack_index as isize)).first_ten =
            0 as *mut libc::c_void as *mut *mut libc::c_char;

        (*dollar_arg_stack.offset(dollar_arg_stack_index as isize)).rest =
            0 as *mut libc::c_void as *mut WORD_LIST;

        (*dollar_arg_stack.offset(dollar_arg_stack_index as isize)).count = 0 as libc::c_int;
    }
    set_dollar_vars_unchanged();
    invalidate_cached_quoted_dollar_at();
}

#[no_mangle]
pub fn dispose_saved_dollar_vars() {
    unsafe {
        if dollar_arg_stack.is_null() || dollar_arg_stack_index == 0 as libc::c_int {
            return;
        }
        dollar_arg_stack_index -= 1;
        dispose_words((*dollar_arg_stack.offset(dollar_arg_stack_index as isize)).rest);
        free_saved_dollar_vars(
            (*dollar_arg_stack.offset(dollar_arg_stack_index as isize)).first_ten,
        );
        free(
            (*dollar_arg_stack.offset(dollar_arg_stack_index as isize)).first_ten
                as *mut libc::c_void,
        );

        (*dollar_arg_stack.offset(dollar_arg_stack_index as isize)).first_ten =
            0 as *mut libc::c_void as *mut *mut libc::c_char;
        (*dollar_arg_stack.offset(dollar_arg_stack_index as isize)).rest =
            0 as *mut libc::c_void as *mut WORD_LIST;
        (*dollar_arg_stack.offset(dollar_arg_stack_index as isize)).count = 0 as libc::c_int;
    }
}

#[no_mangle]
pub fn init_bash_argv() {
    unsafe {
        if bash_argv_initialized == 0 as libc::c_int {
            save_bash_argv();
            bash_argv_initialized = 1 as libc::c_int;
        }
    }
}
#[no_mangle]
pub fn save_bash_argv() {
    let mut list: *mut WORD_LIST = 0 as *mut WORD_LIST;
    list = list_rest_of_args();
    push_args(list);
    dispose_words(list);
}

#[no_mangle]
pub fn push_args(list: *mut WORD_LIST) {
    let mut bash_argv_v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let bash_argc_v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut bash_argv_a: *mut ARRAY = 0 as *mut ARRAY;
    let bash_argc_a: *mut ARRAY = 0 as *mut ARRAY;

    let mut l: *mut WORD_LIST = 0 as *mut WORD_LIST;
    let mut i: arrayind_t = 0;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        GET_ARRAY_FROM_VAR!(
            b"BASH_ARGV\0" as *const u8 as *const libc::c_char,
            bash_argv_v,
            bash_argv_a
        );

        GET_ARRAY_FROM_VAR!(
            b"BASH_ARGC\0" as *const u8 as *const libc::c_char,
            bash_argv_v,
            bash_argv_a
        );

        l = list;
        i = 0 as libc::c_int as arrayind_t;
        while !l.is_null() {
            array_push!(bash_argv_a, (*(*l).word).word);
            l = (*l).next;
            i += 1;
            i;
        }
        t = itos(i);
        array_push!(bash_argv_a, t);
        free(t as *mut libc::c_void);
    }
}

#[no_mangle]
pub fn pop_args() {
    let mut bash_argv_v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let bash_argc_v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut bash_argv_a: *mut ARRAY = 0 as *mut ARRAY;
    let bash_argc_a: *mut ARRAY = 0 as *mut ARRAY;
    let mut ce: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;

    let mut i: intmax_t = 0;
    unsafe {
        GET_ARRAY_FROM_VAR!(
            b"BASH_ARGV\0" as *const u8 as *const libc::c_char,
            bash_argv_v,
            bash_argv_a
        );

        GET_ARRAY_FROM_VAR!(
            b"BASH_ARGC\0" as *const u8 as *const libc::c_char,
            bash_argv_v,
            bash_argv_a
        );

        ce = array_shift(bash_argc_a, 1 as libc::c_int, 0 as libc::c_int);

        if ce.is_null() || legal_number((*ce).value, &mut i) == 0 as libc::c_int {
            i = 0 as libc::c_int as intmax_t;
        }

        while i > 0 as libc::c_int as libc::c_long {
            array_pop!(bash_argc_a);
            i -= 1;
            i;
        }
        array_dispose_element(ce);
    }
}

static mut special_vars: [name_and_function; 38] = {
    [
        {
            let init = name_and_function {
                name: b"BASH_COMPAT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_shcompat),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"BASH_XTRACEFD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_xtracefd),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"CHILD_MAX\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_childmax),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"COMP_WORDBREAKS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_comp_wordbreaks),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"EXECIGNORE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_execignore),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"FUNCNEST\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_funcnest),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"GLOBIGNORE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_globignore),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"HISTCONTROL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_history_control),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"HISTFILESIZE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_histignore),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"HISTIGNORE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_histsize),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"HISTSIZE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_histsize),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"HISTTIMEFORMAT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_histtimefmt),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"HOSTFILE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_hostfile),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"IFS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_ifs),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"IGNOREEOF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_ignoreeof),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"LANG\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_locale),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"LC_ALL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_locale),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"LC_COLLATE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_locale),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"LC_CTYPE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_locale),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"LC_MESSAGES\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_locale),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"LC_NUMERIC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_locale),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"LC_TIME\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_locale),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"MAIL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_mail),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"MAILCHECK\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_mail),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"MAILPATH\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_mail),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"OPTERR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_opterr),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"OPTIND\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_optind),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"PATH\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_path),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_strict_posix),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"TERM\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_terminal),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"TERMCAP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_terminal),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"TERMINFO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_terminal),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"TEXTDOMAIN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_locale),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"TEXTDOMAINDIR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_locale),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"TZ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_tz),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"histchars\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_histchars),
            };
            init
        },
        {
            let init = name_and_function {
                name: b"ignoreeof\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                function: Some(sv_ignoreeof),
            };
            init
        },
        {
            let init = name_and_function {
                name: 0 as *const libc::c_char as *mut libc::c_char,
                function: None,
            };
            init
        },
    ]
};

fn sv_compare(sv1: *mut name_and_function, sv2: *mut name_and_function) -> libc::c_int {
    let mut r: libc::c_int = 0;
    unsafe {
        r = *((*sv1).name).offset(0 as libc::c_int as isize) as libc::c_int
            - *((*sv2).name).offset(0 as libc::c_int as isize) as libc::c_int;
        if r == 0 as libc::c_int {
            r = libc::strcmp((*sv1).name, (*sv2).name);
        }
    }
    return r;
}

#[inline]
fn find_special_var(name: *const libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    i = 0 as libc::c_int;
    unsafe {
        while !(special_vars[i as usize].name).is_null() {
            r = *(special_vars[i as usize].name).offset(0 as libc::c_int as isize) as libc::c_int
                - *name.offset(0 as libc::c_int as isize) as libc::c_int;
            if r == 0 as libc::c_int {
                r = libc::strcmp(special_vars[i as usize].name, name);
            }
            if r == 0 as libc::c_int {
                return i;
            } else {
                if r > 0 as libc::c_int {
                    break;
                }
                i += 1;
                i;
            }
        }
        return -(1 as libc::c_int);
    }
}

#[no_mangle]
pub fn stupidly_hack_special_variables(name: *mut libc::c_char) {
    static mut sv_sorted: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    unsafe {
        if sv_sorted == 0 as libc::c_int {
            qsort(
                special_vars.as_mut_ptr() as *mut libc::c_void,
                (std::mem::size_of::<[name_and_function; 38]>() as usize)
                    .wrapping_div(std::mem::size_of::<name_and_function>() as usize)
                    .wrapping_sub(1 as libc::c_int as usize),
                std::mem::size_of::<name_and_function>() as usize,
                std::mem::transmute::<Option<fn() -> libc::c_int>, Option<QSFUNC>>(Some(
                    std::mem::transmute::<
                        fn(*mut name_and_function, *mut name_and_function) -> libc::c_int,
                        fn() -> libc::c_int,
                    >(sv_compare),
                )),
            );
            sv_sorted = 1 as libc::c_int;
        }
        i = find_special_var(name);
        if i != -(1 as libc::c_int) {
            (Some(
                ((*special_vars.as_mut_ptr().offset(i as isize)).function)
                    .expect("non-null function pointer"),
            ))
            .expect("non-null function pointer")(name);
        }
    }
}

#[no_mangle]
pub fn reinit_special_variables() {
    sv_comp_wordbreaks(
        b"COMP_WORDBREAKS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    sv_globignore(b"GLOBIGNORE\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    sv_opterr(b"OPTERR\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
}

#[no_mangle]
pub fn sv_ifs(name: *mut libc::c_char) {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    v = find_variable(b"IFS\0" as *const u8 as *const libc::c_char);
    setifs(v);
}

#[no_mangle]
pub fn sv_path(name: *mut libc::c_char) {
    unsafe {
        phash_flush();
    }
}

#[no_mangle]
pub fn sv_mail(name: *mut libc::c_char) {
    unsafe {
        if *name.offset(4 as libc::c_int as isize) as libc::c_int == 'C' as i32 {
            reset_mail_timer();
        } else {
            free_mail_files();
            remember_mail_dates();
        };
    }
}

#[no_mangle]
pub fn sv_funcnest(name: *mut libc::c_char) {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut num: intmax_t = 0;
    v = find_variable(name);
    unsafe {
        if v.is_null() {
            funcnest_max = 0 as libc::c_int;
        } else if legal_number((*v).value, &mut num) == 0 as libc::c_int {
            funcnest_max = 0 as libc::c_int;
        } else {
            funcnest_max = num as libc::c_int;
        };
    }
}
#[no_mangle]
pub fn sv_execignore(name: *mut libc::c_char) {
    unsafe {
        setup_exec_ignore(name);
    }
}

#[no_mangle]
pub fn sv_globignore(name: *mut libc::c_char) {
    unsafe {
        if privileged_mode == 0 as libc::c_int {
            setup_glob_ignore(name);
        }
    }
}

#[no_mangle]
pub fn sv_comp_wordbreaks(name: *mut libc::c_char) {
    let mut sv: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    sv = find_variable(name);
    if sv.is_null() {
        reset_completer_word_break_chars();
    }
}
#[no_mangle]
pub fn sv_terminal(name: *mut libc::c_char) {
    unsafe {
        if interactive_shell != 0 && no_line_editing == 0 as libc::c_int {
            rl_reset_terminal(get_string_value(
                b"TERM\0" as *const u8 as *const libc::c_char,
            ));
        }
    }
}
#[no_mangle]
pub fn sv_hostfile(name: *mut libc::c_char) {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    v = find_variable(name);
    if v.is_null() {
        clear_hostname_list();
    } else {
        unsafe {
            hostname_list_initialized = 0 as libc::c_int;
        }
    }
}

#[no_mangle]
pub fn sv_histsize(name: *mut libc::c_char) {
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut num: intmax_t = 0;
    let mut hmax: libc::c_int = 0;
    temp = get_string_value(name);
    unsafe {
        if !temp.is_null() && *temp as libc::c_int != 0 {
            if legal_number(temp, &mut num) != 0 {
                hmax = num as libc::c_int;
                if hmax < 0 as libc::c_int
                    && *name.offset(4 as libc::c_int as isize) as libc::c_int == 'S' as i32
                {
                    unstifle_history();
                } else if *name.offset(4 as libc::c_int as isize) as libc::c_int == 'S' as i32 {
                    stifle_history(hmax);
                    hmax = where_history();

                    if history_lines_this_session > hmax {
                        history_lines_this_session = hmax;
                    }
                } else if hmax >= 0 as libc::c_int {
                    history_truncate_file(
                        get_string_value(b"HISTFILE\0" as *const u8 as *const libc::c_char),
                        hmax,
                    );
                    if hmax < history_lines_in_file {
                        history_lines_in_file = hmax;
                    }
                }
            }
        } else if *name.offset(4 as libc::c_int as isize) as libc::c_int == 'S' as i32 {
            unstifle_history();
        }
    }
}

#[no_mangle]
pub fn sv_histignore(name: *mut libc::c_char) {
    setup_history_ignore(name);
}

#[no_mangle]
pub fn sv_history_control(name: *mut libc::c_char) {
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tptr: libc::c_int = 0;
    unsafe {
        history_control = 0 as libc::c_int;
        temp = get_string_value(name);
        if temp.is_null() || *temp as libc::c_int == 0 as libc::c_int {
            return;
        }
        tptr = 0 as libc::c_int;

        loop {
            val = extract_colon_unit(temp, &mut tptr);
            if val.is_null() {
                break;
            }
            if STREQ(val, b"ignorespace\0" as *const u8 as *const libc::c_char) {
                history_control |= HC_IGNSPACE as libc::c_int;
            } else if STREQ(val, b"ignoredups\0" as *const u8 as *const libc::c_char) {
                history_control |= HC_IGNDUPS as libc::c_int;
            } else if STREQ(val, b"ignoreboth\0" as *const u8 as *const libc::c_char) {
                history_control |= HC_IGNBOTH as libc::c_int | 0x2 as libc::c_int;
            } else if STREQ(val, b"erasedups\0" as *const u8 as *const libc::c_char) {
                history_control |= HC_ERASEDUPS as libc::c_int;
            }
            free(val as *mut libc::c_void);
        }
    }
}

#[no_mangle]
pub fn sv_histchars(name: *mut libc::c_char) {
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    temp = get_string_value(name);
    unsafe {
        if !temp.is_null() {
            history_expansion_char = *temp;
            if *temp.offset(0 as libc::c_int as isize) as libc::c_int != 0
                && *temp.offset(1 as libc::c_int as isize) as libc::c_int != 0
            {
                history_subst_char = *temp.offset(1 as libc::c_int as isize);
                if *temp.offset(2 as libc::c_int as isize) != 0 {
                    history_comment_char = *temp.offset(2 as libc::c_int as isize);
                }
            }
        } else {
            history_expansion_char = '!' as i32 as libc::c_char;
            history_subst_char = '^' as i32 as libc::c_char;
            history_comment_char = '#' as i32 as libc::c_char;
        };
    }
}

#[no_mangle]
pub fn sv_histtimefmt(name: *mut libc::c_char) {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    v = find_variable(name);
    unsafe {
        if !v.is_null() {
            if history_comment_char as libc::c_int == 0 as libc::c_int {
                history_comment_char = '#' as i32 as libc::c_char;
            }
        }
        history_write_timestamps = (v != 0 as *mut SHELL_VAR) as libc::c_int;
    }
}

#[no_mangle]
pub fn sv_tz(name: *mut libc::c_char) {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;

    v = find_variable(name);
    unsafe {
        if !v.is_null() && exported_p!(v) != 0 {
            array_needs_making = 1 as libc::c_int;
        } else if v.is_null() {
            array_needs_making = 1 as libc::c_int;
        }
        if array_needs_making != 0 {
            maybe_make_export_env();
            tzset();
        }
    }
}

#[no_mangle]
pub fn sv_ignoreeof(name: *mut libc::c_char) {
    let mut tmp_var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        eof_encountered = 0 as libc::c_int;

        tmp_var = find_variable(name);
        ignoreeof = if (!tmp_var.is_null()) && var_isset!(tmp_var) {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };

        temp = if !tmp_var.is_null() {
            value_cell!(tmp_var)
        } else {
            0 as *mut libc::c_void as *mut libc::c_char
        };
        if !temp.is_null() {
            eof_encountered_limit = if *temp as libc::c_int != 0 && all_digits(temp) != 0 {
                atoi(temp)
            } else {
                10 as libc::c_int
            };
        }
    }
    set_shellopts();
}

#[no_mangle]
pub fn sv_optind(name: *mut libc::c_char) {
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut tt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: libc::c_int = 0;

    var = find_variable(b"OPTIND\0" as *const u8 as *const libc::c_char);
    unsafe {
        tt = if !var.is_null() {
            get_variable_value(var)
        } else {
            0 as *mut libc::c_void as *mut libc::c_char
        };
        if !tt.is_null() && *tt as libc::c_int != 0 {
            s = atoi(tt);
            if s < 0 as libc::c_int || s == 1 as libc::c_int {
                s = 0 as libc::c_int;
            }
        } else {
            s = 0 as libc::c_int;
        }
    }
    getopts_reset(s);
}

#[no_mangle]
pub fn sv_opterr(name: *mut libc::c_char) {
    let mut tt: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        tt = get_string_value(b"OPTERR\0" as *const u8 as *const libc::c_char);
        sh_opterr = if !tt.is_null() && *tt as libc::c_int != 0 {
            atoi(tt)
        } else {
            1 as libc::c_int
        };
    }
}

#[no_mangle]
pub fn sv_strict_posix(name: *mut libc::c_char) {
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    var = find_variable(name);
    unsafe {
        posixly_correct = if (!var.is_null()) && var_isset!(var) {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };

        posix_initialize(posixly_correct);
        if interactive_shell != 0 {
            posix_readline_initialize(posixly_correct);
        }
    }
    set_shellopts();
}

#[no_mangle]
pub fn sv_locale(name: *mut libc::c_char) {
    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: libc::c_int = 0;
    v = get_string_value(name);
    unsafe {
        if *name.offset(0 as libc::c_int as isize) as libc::c_int == 'L' as i32
            && *name.offset(1 as libc::c_int as isize) as libc::c_int == 'A' as i32
        {
            r = set_lang(name, v);
        } else {
            r = set_locale_var(name, v);
        }
        if r == 0 as libc::c_int && posixly_correct != 0 {
            set_exit_status(1 as libc::c_int);
        }
    }
}

#[no_mangle]
pub fn set_pipestatus_array(ps: *mut libc::c_int, nproc: libc::c_int) {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut a: *mut ARRAY = 0 as *mut ARRAY;
    let mut ae: *mut ARRAY_ELEMENT = 0 as *mut ARRAY_ELEMENT;
    let mut i: libc::c_int = 0;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tbuf: [libc::c_char; (INT_STRLEN_BOUND!(libc::c_int) + 1) as usize] =
        [0; (INT_STRLEN_BOUND!(libc::c_int) + 1) as usize];

    v = find_variable(b"PIPESTATUS\0" as *const u8 as *const libc::c_char);
    if v.is_null() {
        v = make_new_array_variable(
            b"PIPESTATUS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    unsafe {
        if array_p!(v) == 0 as libc::c_int {
            return;
        }

        a = array_cell!(v);

        if a.is_null() || array_num_elements!(a) == 0 as libc::c_int {
            i = 0 as libc::c_int;
            while i < nproc {
                t = inttostr(
                    *ps.offset(i as isize) as intmax_t,
                    tbuf.as_mut_ptr(),
                    std::mem::size_of::<[libc::c_char; (INT_STRLEN_BOUND!(libc::c_int) + 1) as usize]>(
                    ) as size_t,
                );
                array_insert(a, i as arrayind_t, t);
                i += 1;
                i;
            }
            return;
        }

        if array_num_elements!(a) == nproc && nproc == 1 as libc::c_int {
            ae = element_forw!((*a).head);
            free(element_value!(ae) as *mut libc::c_void);
            set_element_value!(ae, itos(*ps.offset(0 as libc::c_int as isize) as intmax_t));
        } else if array_num_elements!(a) <= nproc {
            ae = (*a).head;

            i = 0 as libc::c_int;

            while i < (*a).num_elements {
                ae = (*ae).next;
                free(element_value!(ae) as *mut libc::c_void);
                set_element_value!(ae, itos(*ps.offset(i as libc::c_int as isize) as intmax_t));
                i += 1;
                i;
            }
            while i < nproc {
                t = inttostr(
                    *ps.offset(i as isize) as intmax_t,
                    tbuf.as_mut_ptr(),
                    std::mem::size_of::<[libc::c_char; (INT_STRLEN_BOUND!(libc::c_int) + 1) as usize]>(
                    ) as u64,
                );
                array_insert(a, i as arrayind_t, t);
                i += 1;
                i;
            }
        } else {
            array_flush(a);
            i = 0 as libc::c_int;
            while *ps.offset(i as isize) != -(1 as libc::c_int) {
                t = inttostr(
                    *ps.offset(i as isize) as intmax_t,
                    tbuf.as_mut_ptr(),
                    std::mem::size_of::<[libc::c_char; (INT_STRLEN_BOUND!(libc::c_int) + 1) as usize]>(
                    ) as u64,
                );
                array_insert(a, i as arrayind_t, t);
                i += 1;
                i;
            }
        };
    }
}

#[no_mangle]
pub fn save_pipestatus_array() -> *mut ARRAY {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut a: *mut ARRAY = 0 as *mut ARRAY;
    v = find_variable(b"PIPESTATUS\0" as *const u8 as *const libc::c_char);
    unsafe {
        if v.is_null() || array_p!(v) == 0 as libc::c_int || (array_cell!(v)).is_null() {
            return 0 as *mut libc::c_void as *mut ARRAY;
        }
        a = array_copy((*v).value as *mut ARRAY);
    }
    return a;
}

#[no_mangle]
pub fn restore_pipestatus_array(a: *mut ARRAY) {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut a2: *mut ARRAY = 0 as *mut ARRAY;
    v = find_variable(b"PIPESTATUS\0" as *const u8 as *const libc::c_char);
    unsafe {
        if v.is_null() || array_p!(v) == 0 as libc::c_int || (array_cell!(v)).is_null() {
            return;
        }
        a2 = array_cell!(v);
        var_setarray!(v, a);

        array_dispose(a2);
    }
}

#[no_mangle]
pub fn set_pipestatus_from_exit(s: libc::c_int) {
    unsafe {
        static mut v: [libc::c_int; 2] = [0 as libc::c_int, -(1 as libc::c_int)];
        v[0 as libc::c_int as usize] = s;
        set_pipestatus_array(v.as_mut_ptr(), 1 as libc::c_int);
    }
}

#[no_mangle]
pub fn sv_xtracefd(name: *mut libc::c_char) {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fd: libc::c_int = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    v = find_variable(name);
    unsafe {
        if v.is_null() {
            xtrace_reset();
            return;
        }
        t = (*v).value;
        if t.is_null() || *t as libc::c_int == 0 as libc::c_int {
            xtrace_reset();
        } else {
            fd = strtol(t, &mut e, 10 as libc::c_int) as libc::c_int;
            if e != t && *e as libc::c_int == '\0' as i32 && sh_validfd(fd) != 0 {
                fp = fdopen(fd, b"w\0" as *const u8 as *const libc::c_char);
                if fp.is_null() {
                    internal_error(
                        b"%s: %s: cannot open as FILE\0" as *const u8 as *const libc::c_char,
                        name,
                        value_cell!(v),
                    );
                } else {
                    xtrace_set(fd, fp);
                }
            } else {
                internal_error(
                    b"%s: %s: invalid value for trace file descriptor\0" as *const u8
                        as *const libc::c_char,
                    name,
                    value_cell!(v),
                );
            }
        };
    }
}

#[no_mangle]
pub fn sv_shcompat(name: *mut libc::c_char) {
    let mut current_block: u64;
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tens: libc::c_int = 0;
    let mut ones: libc::c_int = 0;
    let mut compatval: libc::c_int = 0;

    v = find_variable(name);
    unsafe {
        if v.is_null() {
            shell_compatibility_level = DEFAULT_COMPAT_LEVEL!() as libc::c_int;
            set_compatibility_opts();
            return;
        }
        val = value_cell!(v);

        if val.is_null() || *val as libc::c_int == '\0' as i32 {
            shell_compatibility_level = DEFAULT_COMPAT_LEVEL!() as libc::c_int;
            set_compatibility_opts();
            return;
        }

        if ISDIGIT!(*val.offset(0 as libc::c_int as isize) as libc::c_int)
            && *val.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
            && {
                if *val.offset(2 as libc::c_int as isize) == 0
                    && *val.offset(3 as libc::c_int as isize) == 0
                {
                    ISDIGIT!(1 as libc::c_int)
                } else {
                    ISDIGIT!(0 as libc::c_int)
                }
            }
        {
            tens = *val.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32;
            ones = *val.offset(2 as libc::c_int as isize) as libc::c_int - '0' as i32;
            compatval = tens * 10 as libc::c_int + ones;
        } else if ISDIGIT!(*val.offset(0 as libc::c_int as isize) as libc::c_int) && {
            if *val.offset(1 as libc::c_int as isize) == 0
                && *val.offset(2 as libc::c_int as isize) == 0
            {
                ISDIGIT!(1 as libc::c_int)
            } else {
                ISDIGIT!(0 as libc::c_int)
            }
        } {
            tens = *val.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32;
            ones = *val.offset(1 as libc::c_int as isize) as libc::c_int - '0' as i32;
            compatval = tens * 10 as libc::c_int + ones;
        } else {
            internal_error(
                b"%s: %s: compatibility value out of range\0" as *const u8 as *const libc::c_char,
                name,
                val,
            );
            shell_compatibility_level = DEFAULT_COMPAT_LEVEL!() as libc::c_int;
            set_compatibility_opts();
            return;
        }

        if !(compatval < MIN_COMPAT_LEVEL!() as libc::c_int
            || compatval > DEFAULT_COMPAT_LEVEL!() as libc::c_int)
        {
            internal_error(
                b"%s: %s: compatibility value out of range\0" as *const u8 as *const libc::c_char,
                name,
                val,
            );
            shell_compatibility_level = DEFAULT_COMPAT_LEVEL!() as libc::c_int;
            set_compatibility_opts();
            return;
        }

        shell_compatibility_level = compatval;
    }
    set_compatibility_opts();
}

#[no_mangle]
pub fn sv_childmax(name: *mut libc::c_char) {
    let mut tt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: libc::c_int = 0;

    tt = get_string_value(name);
    unsafe {
        s = if !tt.is_null() && *tt as libc::c_int != 0 {
            atoi(tt)
        } else {
            0 as libc::c_int
        };
        set_maxchild(s);
    }
}
