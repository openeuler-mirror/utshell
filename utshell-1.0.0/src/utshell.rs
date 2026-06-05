#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

extern crate lazy_static;

use crate::bashhist::{
    bash_history_reinit, bash_initialize_history, load_history, maybe_save_shell_history,
};
use crate::bashline::bashline_reinitialize;
use crate::builtins::common::{initialize_shell_builtins, remember_args};
use crate::builtins::evalfile::{force_execute_file, maybe_execute_file};
use crate::builtins::read::read_tty_cleanup;
use crate::builtins::read::read_tty_modified;
use crate::builtins::set::{
    initialize_shell_options, list_minus_o_opts, set_minus_o_option, set_shellopts,
};
use crate::builtins::shopt::{
    initialize_bashopts, set_bashopts, set_login_shell, shopt_listopt, shopt_setopt,
};
use crate::dispose_cmd::dispose_words;
use crate::error::{command_error, file_error};
use crate::eval::{pretty_print_loop, reader_loop};
use crate::execute_cmd::{coproc_flush, executing_line_number};
use crate::findcmd::find_path_file;
use crate::flags::{change_flag, initialize_flags};
use crate::general::{
    absolute_program, base_pathname, c_sh_unset_nodelay_mode, check_binary_file, check_dev_tty,
    move_to_high_fd, tilde_initialize,
};
use crate::input::{close_buffered_fd, with_input_from_buffered_stream};
use crate::jobs::{
    end_job_control, get_tty_state, hangup_all_jobs, initialize_job_control, set_job_control,
};
use crate::local::set_default_locale;
use crate::local::{set_default_lang, set_default_locale_vars};
use crate::mailcheck::{init_mail_dates, reset_mail_timer};
use crate::make_cmd::{cmd_init, make_word, make_word_list};
use crate::parse_and_execute;
use crate::print_cmd::xtrace_init;
use crate::sig::initialize_signals;
use crate::src_common;
use crate::src_common::*;
use crate::subst::{expand_string_unsplit_to_string, unlink_all_fifos, unlink_fifo_list};
use crate::trap::{initialize_traps, run_exit_trap};
use crate::unwind_prot::uwp_init;
use crate::variables::get_string_value;
use crate::variables::{
    bind_variable, delete_all_contexts, delete_all_variables, initialize_shell_variables, pop_args,
    push_args, reinit_special_variables, set_pipestatus_from_exit, set_var_read_only,
    sv_strict_posix, unbind_variable,
};
use crate::version::shell_version_string;
use crate::version::show_shell_version;
use crate::y_tab::{
    bash_input, expand_aliases, initialize_bash_input, line_number, with_input_from_stdin,
    with_input_from_stream,
};
use std::convert::TryInto;

#[link(name = "termcap")]

extern "C" {
    static mut rl_deprep_term_function: Option<rl_voidfunc_t>;
    #[link_name = "\u{1}stderr"]
    pub static mut stderr: *mut FILE;
    #[link_name = "\u{1}stdin"]
    pub static mut stdin: *mut FILE;
    #[link_name = "\u{1}stdout"]
    pub static mut stdout: *mut FILE;

}

static mut act_like_sh: libc::c_int = 0;
static mut su_shell: libc::c_int = 0;
static mut sourced_env: libc::c_int = 0;
static mut running_setuid: libc::c_int = 0;
static mut debugging: libc::c_int = 0;
static mut no_rc: libc::c_int = 0;
static mut no_profile: libc::c_int = 0;
static mut do_version: libc::c_int = 0;
static mut make_login_shell: libc::c_int = 0;
static mut want_initial_help: libc::c_int = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub name: *const libc::c_char,
    pub type_0: libc::c_int,
    pub int_value: *mut libc::c_int,
    pub char_value: *mut *mut libc::c_char,
}
static mut long_args: [C2RustUnnamed_0; 18] = unsafe {
    [
        {
            let init = C2RustUnnamed_0 {
                name: b"debug\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                int_value: &debugging as *const libc::c_int as *mut libc::c_int,
                char_value: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            };
            init
        },
        {
            let init = C2RustUnnamed_0 {
                name: b"debugger\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                int_value: &debugging_mode as *const libc::c_int as *mut libc::c_int,
                char_value: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            };
            init
        },
        {
            let init = C2RustUnnamed_0 {
                name: b"dump-po-strings\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                int_value: &dump_po_strings as *const libc::c_int as *mut libc::c_int,
                char_value: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            };
            init
        },
        {
            let init = C2RustUnnamed_0 {
                name: b"dump-strings\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                int_value: &dump_translatable_strings as *const libc::c_int as *mut libc::c_int,
                char_value: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            };
            init
        },
        {
            let init = C2RustUnnamed_0 {
                name: b"help\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                int_value: &want_initial_help as *const libc::c_int as *mut libc::c_int,
                char_value: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            };
            init
        },
        {
            let init = C2RustUnnamed_0 {
                name: b"init-file\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                int_value: 0 as *const libc::c_int as *mut libc::c_int,
                char_value: &bashrc_file as *const *mut libc::c_char as *mut *mut libc::c_char,
            };
            init
        },
        {
            let init = C2RustUnnamed_0 {
                name: b"login\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                int_value: &make_login_shell as *const libc::c_int as *mut libc::c_int,
                char_value: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            };
            init
        },
        {
            let init = C2RustUnnamed_0 {
                name: b"noediting\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                int_value: &no_line_editing as *const libc::c_int as *mut libc::c_int,
                char_value: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            };
            init
        },
        {
            let init = C2RustUnnamed_0 {
                name: b"noprofile\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                int_value: &no_profile as *const libc::c_int as *mut libc::c_int,
                char_value: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            };
            init
        },
        {
            let init = C2RustUnnamed_0 {
                name: b"norc\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                int_value: &no_rc as *const libc::c_int as *mut libc::c_int,
                char_value: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            };
            init
        },
        {
            let init = C2RustUnnamed_0 {
                name: b"posix\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                int_value: &posixly_correct as *const libc::c_int as *mut libc::c_int,
                char_value: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            };
            init
        },
        {
            let init = C2RustUnnamed_0 {
                name: b"pretty-print\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                int_value: &pretty_print_mode as *const libc::c_int as *mut libc::c_int,
                char_value: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            };
            init
        },
        {
            let init = C2RustUnnamed_0 {
                name: b"rcfile\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                int_value: 0 as *const libc::c_int as *mut libc::c_int,
                char_value: &bashrc_file as *const *mut libc::c_char as *mut *mut libc::c_char,
            };
            init
        },
        {
            let init = C2RustUnnamed_0 {
                name: b"rpm-requires\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                int_value: &rpm_requires as *const libc::c_int as *mut libc::c_int,
                char_value: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            };
            init
        },
        {
            let init = C2RustUnnamed_0 {
                name: b"restricted\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                int_value: &restricted as *const libc::c_int as *mut libc::c_int,
                char_value: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            };
            init
        },
        {
            let init = C2RustUnnamed_0 {
                name: b"verbose\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                int_value: &verbose_flag as *const libc::c_int as *mut libc::c_int,
                char_value: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            };
            init
        },
        {
            let init = C2RustUnnamed_0 {
                name: b"version\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                int_value: &do_version as *const libc::c_int as *mut libc::c_int,
                char_value: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            };
            init
        },
        {
            let init = C2RustUnnamed_0 {
                name: 0 as *const libc::c_char as *mut libc::c_char,
                type_0: 1 as libc::c_int,
                int_value: 0 as *const libc::c_int as *mut libc::c_int,
                char_value: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            };
            init
        },
    ]
};

static mut shell_reinitialized: libc::c_int = 0 as libc::c_int;
static mut default_input: *mut FILE = 0 as *const FILE as *mut FILE;
static mut shopt_alist: *mut STRING_INT_ALIST =
    0 as *const STRING_INT_ALIST as *mut STRING_INT_ALIST;
static mut shopt_ind: libc::c_int = 0 as libc::c_int;
static mut shopt_len: libc::c_int = 0 as libc::c_int;
pub fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut env: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut code: libc::c_int = 0;
    let mut old_errexit_flag: libc::c_int = 0;
    let mut saverst: libc::c_int = 0;
    let mut locally_skip_execution: libc::c_int = 0;
    let mut arg_index: libc::c_int = 0;
    let mut top_level_arg_index: libc::c_int = 0;
    unsafe {
        code = setjmp_nosigs!(top_level.as_mut_ptr());
        if code != 0 {
            exit(2);
        }
        xtrace_init();
        check_dev_tty();
        while debugging_login_shell != 0 {
            sleep(3);
        }
        set_default_locale();

        running_setuid = uidget();

        if !(getenv(b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char)).is_null()
            || !(getenv(b"POSIX_PEDANTIC\0" as *const u8 as *const libc::c_char)).is_null()
        {
            posixly_correct = 1;
        }
        if setjmp_sigs!(subshell_top_level.as_mut_ptr()) != 0 {
            argc = subshell_argc;
            argv = subshell_argv;
            env = subshell_envp;
            sourced_env = 0;
        }

        shell_reinitialized = 0;

        arg_index = 1;
        if arg_index > argc {
            arg_index = argc;
        }
        shell_script_filename = 0 as *mut libc::c_char;
        command_execution_string = shell_script_filename;
        read_from_stdin = 0;
        locally_skip_execution = read_from_stdin;
        want_pending_command = locally_skip_execution;
        default_input = stdin;
        default_buffered_input = -1;

        make_login_shell = 0;
        login_shell = make_login_shell;

        if shell_initialized != 0 || !shell_name.is_null() {
            if *shell_name as libc::c_int == '-' as i32 {
                shell_name = shell_name.offset(1);
            }
            shell_reinitialize();
            if setjmp_nosigs!(top_level.as_mut_ptr()) != 0 {
                exit(2);
            }
        }
        shell_environment = env;
        set_shell_name(*argv.offset(0 as isize));

        c_gettimeofday(&mut shellstart, 0 as *mut crate::src_common::timezone);
        shell_start_time = shellstart.tv_sec;

        arg_index = parse_long_options(argv, arg_index, argc);

        if want_initial_help != 0 {
            show_shell_usage(stdout, 1);
            exit(EXECUTION_SUCCESS as libc::c_int);
        }

        if do_version != 0 {
            show_shell_version(1);
            exit(EXECUTION_SUCCESS as libc::c_int);
        }

        echo_input_at_read = verbose_flag;

        this_command_name = shell_name;
        arg_index = parse_shell_options(argv, arg_index, argc);

        if make_login_shell != 0 {
            login_shell += 1;
            login_shell = -login_shell;
        }

        set_login_shell(
            b"login_shell\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (login_shell != 0) as libc::c_int,
        );

        if dump_po_strings != 0 {
            dump_translatable_strings = 1;
        }

        if dump_translatable_strings != 0 {
            read_but_dont_execute = 1;
        }

        if rpm_requires != 0 {
            read_but_dont_execute = 1;
            initialize_shell_builtins();
        }

        if running_setuid != 0 && privileged_mode == 0 {
            disable_priv_mode();
        }

        if want_pending_command != 0 {
            command_execution_string = *argv.offset(arg_index as isize);
            if command_execution_string.is_null() {
                report_error(
                    b"%s: option requires an argument\0" as *const u8 as *mut libc::c_char,
                    b"-c\0" as *const u8 as *const libc::c_char,
                );
                exit(EX_BADUSAGE as libc::c_int);
            }
            arg_index += 1;
        }

        this_command_name = 0 as *mut libc::c_char;

        if forced_interactive != 0
            || command_execution_string.is_null()
                && wordexp_only == 0
                && (arg_index == argc || read_from_stdin != 0)
                && isatty(fileno(stdin)) != 0
                && isatty(fileno(stderr)) != 0
        {
            init_interactive();
        } else {
            init_noninteractive();
        }

        if login_shell != 0 && interactive_shell != 0 {
            i = 3;
            while i < 20 {
                SET_CLOSE_ON_EXEC!(i);
                i += 1;
            }
        }

        if posixly_correct != 0 {
            bind_variable(
                b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char,
                b"y\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0,
            );
            sv_strict_posix(
                b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }

        if !shopt_alist.is_null() {
            run_shopt_alist();
        }

        shell_initialize();

        set_default_lang();
        set_default_locale_vars();

        if interactive_shell != 0 {
            let mut term: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut emacs: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut inside_emacs: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut emacs_term: libc::c_int = 0;
            let mut in_emacs: libc::c_int = 0;

            term = get_string_value(b"TERM\0" as *const u8 as *const libc::c_char);
            emacs = get_string_value(b"EMACS\0" as *const u8 as *const libc::c_char);
            inside_emacs = get_string_value(b"INSIDE_EMACS\0" as *const u8 as *const libc::c_char);

            if !inside_emacs.is_null() {
                emacs_term = (strstr(
                    inside_emacs,
                    b",term:\0" as *const u8 as *const libc::c_char,
                ) != 0 as *mut libc::c_char) as libc::c_int;
                in_emacs = 1;
            } else if !emacs.is_null() {
                let msg = CString::new("t").unwrap();
                emacs_term = (strstr(emacs, b" (term:\0" as *const u8 as *const libc::c_char)
                    != 0 as *mut libc::c_char) as libc::c_int;
                in_emacs = (emacs_term != 0 || STREQ(emacs, msg.as_ptr())) as libc::c_int;
            } else {
                emacs_term = 0;
                in_emacs = emacs_term;
            }

            let msg1 = CString::new("emacs").unwrap();
            no_line_editing |= STREQ(term, msg1.as_ptr()) as libc::c_int;
            let msg2 = CString::new("dumb").unwrap();
            no_line_editing |= (in_emacs != 0 && STREQ(term, msg2.as_ptr())) as libc::c_int;

            running_under_emacs = (in_emacs != 0
                || STREQN!(term, String::from("emacs").as_ptr() as *mut libc::c_char, 5) != 0)
                as libc::c_int;
            running_under_emacs += (emacs_term != 0
                && STREQN!(term, String::from("eterm").as_ptr() as *mut libc::c_char, 5) != 0)
                as libc::c_int;

            if running_under_emacs != 0 {
                gnu_error_format = 1;
            }
        }

        top_level_arg_index = arg_index;
        old_errexit_flag = exit_immediately_on_error;

        code = setjmp_sigs!(top_level.as_mut_ptr());
        if code != 0 {
            if code == EXITPROG as libc::c_int || code == ERREXIT as libc::c_int {
                exit_shell(last_command_exit_value);
            } else {
                set_job_control(interactive_shell);

                exit_immediately_on_error += old_errexit_flag;
                locally_skip_execution += 1;
            }
        }

        arg_index = top_level_arg_index;

        if interactive_shell == 0 {
            unbind_variable(b"PS1\0" as *const u8 as *const libc::c_char);
            unbind_variable(b"PS2\0" as *const u8 as *const libc::c_char);
            interactive = 0;
        } else {
            change_flag('i' as i32, FLAG_ON as libc::c_int);
            interactive = 1;
        }

        restricted_shell = shell_is_restricted(shell_name);

        saverst = restricted;
        restricted = 0;

        if !(wordexp_only != 0) {
            if !command_execution_string.is_null() {
                arg_index = bind_args(argv, arg_index, argc, 0);
            } else if arg_index != argc && read_from_stdin == 0 {
                shell_script_filename = *argv.offset(arg_index as isize);
                arg_index += 1;
                arg_index = bind_args(argv, arg_index, argc, 1);
            } else {
                arg_index = bind_args(argv, arg_index, argc, 1);
            }
        }

        if locally_skip_execution == 0 && running_setuid == 0 {
            old_errexit_flag = exit_immediately_on_error;
            exit_immediately_on_error = 0;
            run_startup_files();
            exit_immediately_on_error += old_errexit_flag;
        }

        if act_like_sh != 0 {
            bind_variable(
                b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char,
                b"y\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0,
            );
            sv_strict_posix(
                b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }

        restricted = (saverst != 0 || restricted != 0) as libc::c_int;
        if shell_reinitialized == 0 {
            maybe_make_restricted(shell_name);
        }

        cmd_init();
        uwp_init();

        if !command_execution_string.is_null() {
            startup_state = 2;

            if debugging_mode != 0 {
                start_debugger();
            }

            executing = 1;
            run_one_command(command_execution_string);
            exit_shell(last_command_exit_value);
        }

        if !shell_script_filename.is_null() {
            open_shell_script(shell_script_filename);
        } else if interactive == 0 {
            default_buffered_input = fileno(stdin);
            read_from_stdin = 1;
        } else if top_level_arg_index == argc {
            read_from_stdin = 1;
        }

        set_bash_input();

        if debugging_mode != 0
            && locally_skip_execution == 0
            && running_setuid == 0
            && (reading_shell_script != 0 || interactive_shell == 0)
        {
            start_debugger();
        }

        if interactive_shell != 0 {
            reset_mail_timer();
            init_mail_dates();

            bash_initialize_history();

            if shell_initialized == 0 && history_lines_this_session == 0 {
                load_history();
            }
            get_tty_state();
        }

        shell_initialized = 1;

        if pretty_print_mode != 0 && interactive_shell != 0 {
            internal_warning(
                b"pretty-printing mode ignored in interactive shells\0" as *const u8
                    as *mut libc::c_char,
            );
            pretty_print_mode = 0;
        }
        if pretty_print_mode != 0 {
            exit_shell(pretty_print_loop());
        }
        reader_loop();
        exit_shell(last_command_exit_value);
        return 0;
    }
}

fn parse_long_options(
    argv: *mut *mut libc::c_char,
    arg_start: libc::c_int,
    arg_end: libc::c_int,
) -> libc::c_int {
    let mut arg_index: libc::c_int = 0;
    let mut longarg: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut arg_string: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        arg_index = arg_start;
        while arg_index != arg_end
            && {
                arg_string = *argv.offset(arg_index as isize);
                !arg_string.is_null()
            }
            && *arg_string as libc::c_int == '-' as i32
        {
            longarg = 0;

            if *arg_string.offset(1 as isize) as libc::c_int == '-' as i32
                && *arg_string.offset(2 as isize) as libc::c_int != 0
            {
                longarg = 1;
                arg_string = arg_string.offset(1);
            }

            i = 0;
            while !(long_args[i as usize].name).is_null() {
                if STREQ(arg_string.offset(1), long_args[i as usize].name) {
                    if long_args[i as usize].type_0 == 1 {
                        *long_args[i as usize].int_value = 1;
                    } else {
                        arg_index += 1;
                        if (*argv.offset(arg_index as isize)).is_null() {
                            report_error(
                                b"%s: option requires an argument\0" as *const u8
                                    as *const libc::c_char,
                                long_args[i as usize].name,
                            );
                            exit(EX_BADUSAGE as libc::c_int);
                        } else {
                            *long_args[i as usize].char_value = *argv.offset(arg_index as isize);
                        }
                    }
                    break;
                } else {
                    i += 1;
                }
            }
            if (long_args[i as usize].name).is_null() {
                if longarg != 0 {
                    report_error(
                        b"%s: invalid option\0" as *const u8 as *const libc::c_char,
                        *argv.offset(arg_index as isize),
                    );
                    show_shell_usage(stderr, 0 as libc::c_int);
                    exit(EX_BADUSAGE as libc::c_int);
                }
                break;
            } else {
                arg_index += 1;
            }
        }
        return arg_index;
    }
}

fn parse_shell_options(
    argv: *mut *mut libc::c_char,
    arg_start: libc::c_int,
    arg_end: libc::c_int,
) -> libc::c_int {
    let mut arg_index: libc::c_int = 0;
    let mut arg_character: libc::c_int = 0;
    let mut on_or_off: libc::c_int = 0;
    let mut next_arg: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut o_option: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut arg_string: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        arg_index = arg_start;
        while arg_index != arg_end
            && {
                arg_string = *argv.offset(arg_index as isize);
                !arg_string.is_null()
            }
            && (*arg_string as libc::c_int == '-' as i32
                || *arg_string as libc::c_int == '+' as i32)
        {
            next_arg = arg_index + 1;

            if *arg_string.offset(0 as isize) as libc::c_int == '-' as i32
                && (*arg_string.offset(1 as isize) as libc::c_int == '\u{0}' as i32
                    || *arg_string.offset(1 as isize) as libc::c_int == '-' as i32
                        && *arg_string.offset(2 as isize) as libc::c_int == '\u{0}' as i32)
            {
                return next_arg;
            }

            i = 1;

            on_or_off = *arg_string.offset(0 as isize) as libc::c_int;
            loop {
                arg_character = *arg_string.offset(i as isize) as libc::c_int;
                i = i + 1;
                if !(arg_character != 0) {
                    break;
                }

                match arg_character as u8 as char {
                    'c' => {
                        want_pending_command = 1;
                    }
                    'l' => {
                        make_login_shell = 1;
                    }
                    's' => {
                        read_from_stdin = 1;
                    }
                    'o' => {
                        o_option = *argv.offset(next_arg as isize);
                        if o_option.is_null() {
                            set_option_defaults();
                            list_minus_o_opts(
                                -(1 as libc::c_int),
                                if on_or_off == '-' as i32 { 0 } else { 1 },
                            );
                            reset_option_defaults();
                        } else {
                            if set_minus_o_option(on_or_off, o_option)
                                != EXECUTION_SUCCESS as libc::c_int
                            {
                                exit(EX_BADUSAGE as libc::c_int);
                            }
                            next_arg += 1;
                        }
                    }
                    'O' => {
                        o_option = *argv.offset(next_arg as isize);
                        if o_option.is_null() {
                            shopt_listopt(o_option, if on_or_off == '-' as i32 { 0 } else { 1 });
                        } else {
                            add_shopt_to_alist(o_option, on_or_off);
                            next_arg += 1;
                        }
                    }
                    'D' => {
                        dump_translatable_strings = 1;
                    }
                    _ => {
                        if change_flag(arg_character, on_or_off) == -1 {
                            report_error(
                                b"%c%c: invalid option\0" as *const u8 as *const libc::c_char,
                                on_or_off,
                                arg_character,
                            );
                            show_shell_usage(stderr, 0 as libc::c_int);
                            exit(2 as libc::c_int);
                        }
                    }
                }
            }
            arg_index = next_arg;
        }
        return arg_index;
    }
}

#[no_mangle]
pub fn exit_shell(mut s: libc::c_int) {
    unsafe {
        fflush(stdout);
        fflush(stderr);

        if RL_ISSTATE!(RL_STATE_TERMPREPPED!()) != 0 && rl_deprep_term_function.is_some() {
            (Some(rl_deprep_term_function.expect("non-null function pointer")))
                .expect("non-null function pointer")();
        }
    }
    if read_tty_modified() != 0 {
        read_tty_cleanup();
    }

    if signal_is_trapped(0) != 0 {
        s = run_exit_trap();
    }

    unlink_all_fifos();

    if unsafe { remember_on_history != 0 } {
        maybe_save_shell_history();
    }

    coproc_flush();

    if unsafe { interactive_shell != 0 && login_shell != 0 && hup_on_exit != 0 } {
        hangup_all_jobs();
    }

    if unsafe { subshell_environment == 0 } {
        end_job_control();
    }

    sh_exit(s);
}

#[no_mangle]
pub fn sh_exit(s: libc::c_int) {
    unsafe {
        exit(s);
    }
}

#[no_mangle]
pub fn subshell_exit(mut s: libc::c_int) {
    unsafe {
        fflush(stdout);
        fflush(stderr);

        if signal_is_trapped(0) != 0 {
            s = run_exit_trap();
        }

        sh_exit(s);
    }
}

#[no_mangle]
pub fn set_exit_status(s: libc::c_int) {
    unsafe {
        last_command_exit_value = s;
        set_pipestatus_from_exit(last_command_exit_value);
    }
}

fn execute_env_file(env_file: *mut libc::c_char) {
    let mut fn_0: *mut libc::c_char = 0 as *mut libc::c_char;

    if unsafe { !env_file.is_null() && *env_file as libc::c_int != 0 } {
        fn_0 = expand_string_unsplit_to_string(env_file, Q_DOUBLE_QUOTES as libc::c_int);
        if unsafe { !fn_0.is_null() && *fn_0 as libc::c_int != 0 } {
            maybe_execute_file(fn_0, 1);
        }
        unsafe {
            FREE!(fn_0);
        }
    }
}

//执行启动文件  bashrc等
fn run_startup_files() {
    let mut old_job_control: libc::c_int = 0;
    let mut sourced_login: libc::c_int = 0;
    let mut run_by_ssh: libc::c_int = 0;

    if unsafe {
        interactive_shell == 0
            && no_rc == 0
            && login_shell == 0
            && act_like_sh == 0
            && !command_execution_string.is_null()
    } {
        run_by_ssh = (!(find_variable(b"SSH_CLIENT\0" as *const u8 as *const libc::c_char))
            .is_null()
            || !(find_variable(b"SSH2_CLIENT\0" as *const u8 as *const libc::c_char)).is_null())
            as libc::c_int;
        unsafe {
            if (run_by_ssh != 0 || c_isnetconn(fileno(stdin)) != 0) && shell_level < 2 {
                maybe_execute_file(bashrc_file, 1);
                return;
            }
        }
    }

    old_job_control = if unsafe { interactive_shell != 0 } {
        set_job_control(0)
    } else {
        0
    };

    sourced_login = 0;

    if unsafe { login_shell != 0 && posixly_correct == 0 } {
        unsafe {
            no_rc += 1;
        }

        if unsafe { no_profile == 0 } {
            maybe_execute_file(SYS_PROFILE!(), 1);

            if unsafe { act_like_sh != 0 } {
                maybe_execute_file(b"~/.profile\0" as *const u8 as *const libc::c_char, 1);
            } else if maybe_execute_file(
                b"~/.utshell_profile\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
            ) == 0
                && maybe_execute_file(b"~/.utshell_login\0" as *const u8 as *const libc::c_char, 1)
                    == 0
            {
                maybe_execute_file(b"~/.profile\0" as *const u8 as *const libc::c_char, 1);
            }
        }

        sourced_login = 1;
    }

    if unsafe { interactive_shell == 0 && !(su_shell != 0 && login_shell != 0) } {
        if unsafe {
            posixly_correct == 0 && act_like_sh == 0 && privileged_mode == 0 && {
                let fresh2 = sourced_env;
                sourced_env = sourced_env + 1;
                fresh2 == 0
            }
        } {
            execute_env_file(get_string_value(
                b"BASH_ENV\0" as *const u8 as *const libc::c_char,
            ));
        }
        return;
    }

    if unsafe { posixly_correct == 0 } {
        if unsafe {
            login_shell != 0 && {
                let fresh3 = sourced_login;
                sourced_login = sourced_login + 1;
                fresh3 == 0
            }
        } {
            unsafe {
                no_rc += 1;
            }

            if unsafe { no_profile == 0 } {
                maybe_execute_file(SYS_PROFILE!(), 1);

                if unsafe { act_like_sh != 0 } {
                    maybe_execute_file(b"~/.profile\0" as *const u8 as *const libc::c_char, 1);
                } else if maybe_execute_file(
                    b"~/.utshell_profile\0" as *const u8 as *const libc::c_char,
                    1,
                ) == 0
                    && maybe_execute_file(
                        b"~/.utshell_login\0" as *const u8 as *const libc::c_char,
                        1,
                    ) == 0
                {
                    maybe_execute_file(b"~/.profile\0" as *const u8 as *const libc::c_char, 1);
                }
            }
        }

        if unsafe { act_like_sh == 0 && no_rc == 0 } {
            unsafe {
                maybe_execute_file(bashrc_file, 1);
            }
        } else if unsafe {
            act_like_sh != 0 && privileged_mode == 0 && {
                let fresh4 = sourced_env;
                sourced_env = sourced_env + 1;
                fresh4 == 0
            }
        } {
            execute_env_file(get_string_value(
                b"ENV\0" as *const u8 as *const libc::c_char,
            ));
        }
    } else if unsafe {
        interactive_shell != 0 && privileged_mode == 0 && {
            let fresh5 = sourced_env;
            sourced_env = sourced_env + 1;
            fresh5 == 0
        }
    } {
        execute_env_file(get_string_value(
            b"ENV\0" as *const u8 as *const libc::c_char,
        ));
    }
    set_job_control(old_job_control);
}

#[no_mangle]
pub fn shell_is_restricted(name: *mut libc::c_char) -> libc::c_int {
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        if restricted != 0 {
            return 1;
        }

        temp = base_pathname(name);
        if *temp as libc::c_int == '-' as i32 {
            temp = temp.offset(1);
        }

        return (STREQ(temp, RESTRICTED_SHELL_NAME!())) as libc::c_int;
    }
}

#[no_mangle]
pub fn maybe_make_restricted(name: *mut libc::c_char) -> libc::c_int {
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;

    temp = base_pathname(name);
    unsafe {
        if *temp as libc::c_int == '-' as i32 {
            temp = temp.offset(1);
        }
    }
    if unsafe { restricted != 0 || STREQ(temp, RESTRICTED_SHELL_NAME!()) } {
        set_var_read_only(b"PATH\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        set_var_read_only(b"SHELL\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        set_var_read_only(b"ENV\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        set_var_read_only(b"BASH_ENV\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        set_var_read_only(b"HISTFILE\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        unsafe {
            restricted = 1;
        }
    }
    return unsafe { restricted };
}

fn uidget() -> libc::c_int {
    let mut u: uid_t = 0;
    unsafe {
        u = getuid();
        if current_user.uid != u {
            FREE!(current_user.user_name);
            FREE!(current_user.shell);
            FREE!(current_user.home_dir);
            current_user.home_dir = 0 as *mut libc::c_char;
            current_user.shell = current_user.home_dir;
            current_user.user_name = current_user.shell;
        }
        current_user.uid = u;
        current_user.gid = getgid();
        current_user.euid = geteuid();
        current_user.egid = getegid();
        return (current_user.uid != current_user.euid || current_user.gid != current_user.egid)
            as libc::c_int;
    }
}

#[no_mangle]
pub fn disable_priv_mode() {
    // let mut e: libc::c_int = 0;
    unsafe {
        if setresuid(current_user.uid, current_user.uid, current_user.uid) < 0 {
            // *c___errno_location();
            sys_error(
                b"cannot set uid to %d: effective uid %d\0" as *const u8 as *const libc::c_char,
                current_user.uid,
                current_user.euid,
            );
        }
        if setresgid(current_user.gid, current_user.gid, current_user.gid) < 0 as libc::c_int {
            sys_error(
                b"cannot set gid to %d: effective gid %d\0" as *const u8 as *const libc::c_char,
                current_user.gid,
                current_user.egid,
            );
        }
        current_user.euid = current_user.uid;
        current_user.egid = current_user.gid;
    }
}

fn run_one_command(command: *mut libc::c_char) -> libc::c_int {
    let mut code: libc::c_int = 0;
    unsafe {
        code = setjmp_nosigs!(top_level.as_mut_ptr());

        if code != NOT_JUMPED as libc::c_int {
            unlink_fifo_list();
            match code as i32 {
                FORCE_EOF => {
                    last_command_exit_value = 127;
                    return last_command_exit_value;
                }
                ERREXIT | EXITPROG => return last_command_exit_value,
                DISCARD => {
                    last_command_exit_value = 1;
                    return last_command_exit_value;
                }
                _ => {
                    command_error(
                        b"run_one_command\0" as *const u8 as *const libc::c_char,
                        CMDERR_BADJUMP as libc::c_int,
                        code,
                        0,
                    );
                }
            }
        }
        return parse_and_execute(
            savestring!(command),
            b"-c\0" as *const u8 as *const libc::c_char,
            SEVAL_NOHIST as libc::c_int | SEVAL_RESETLINE as libc::c_int,
        );
    }
}

fn bind_args(
    argv: *mut *mut libc::c_char,
    arg_start: libc::c_int,
    arg_end: libc::c_int,
    start_index: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut args: *mut WordList = 0 as *mut WordList;
    let mut tl: *mut WordList = 0 as *mut WordList;

    i = arg_start;
    tl = 0 as *mut WordList;
    args = tl;
    unsafe {
        while i < arg_end {
            if args.is_null() {
                tl = make_word_list(make_word(*argv.offset(i as isize)), args);
                args = tl;
            } else {
                (*tl).next =
                    make_word_list(make_word(*argv.offset(i as isize)), 0 as *mut WordList);
                tl = (*tl).next;
            }
            i += 1;
        }

        if !args.is_null() {
            if start_index == 0 {
                shell_name = savestring!((*(*args).word).word);
                FREE!(*dollar_vars.as_mut_ptr().offset(0 as isize));

                let ref mut _fresh7 = *dollar_vars.as_mut_ptr().offset(0 as isize);
                *dollar_vars.as_mut_ptr().offset(0 as isize) = savestring!((*(*args).word).word);
                remember_args((*args).next, 1);
                if debugging_mode != 0 {
                    push_args((*args).next);
                    bash_argv_initialized = 1;
                }
            } else {
                remember_args(args, 1);
                if debugging_mode != 0 {
                    push_args(args);
                    bash_argv_initialized = 1;
                }
            }
            dispose_words(args);
        }
        return i;
    }
}

#[no_mangle]
pub fn unbind_args() {
    remember_args(0 as *mut libc::c_void as *mut WordList, 1);
    pop_args();
}

fn start_debugger() {
    let mut old_errexit: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    unsafe {
        old_errexit = exit_immediately_on_error;
        exit_immediately_on_error = 0;

        r = force_execute_file(DEBUGGER_START_FILE!(), 1);
        if r < 0 {
            internal_warning(
                b"cannot start debugger; debugging mode disabled\0" as *const u8
                    as *mut libc::c_char,
            );
            debugging_mode = 0;
        }
        function_trace_mode = debugging_mode;
        error_trace_mode = function_trace_mode;

        set_shellopts();
        set_bashopts();

        exit_immediately_on_error += old_errexit;
    }
}

fn open_shell_script(script_name: *mut libc::c_char) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut fd_is_tty: libc::c_int = 0;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sample: [libc::c_char; 80] = [0; 80];
    let mut sample_len: libc::c_int = 0;
    let mut sb: crate::src_common::stat = crate::src_common::stat_init;
    let mut funcname_v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut bash_source_v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut bash_lineno_v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut funcname_a: *mut ARRAY = 0 as *mut ARRAY;
    let mut bash_source_a: *mut ARRAY = 0 as *mut ARRAY;
    let mut bash_lineno_a: *mut ARRAY = 0 as *mut ARRAY;
    unsafe {
        filename = savestring!(script_name);

        fd = open(filename, O_RDONLY as libc::c_int);
        if fd < 0 && *c___errno_location() == ENOENT!() && absolute_program(filename) == 0 {
            e = *c___errno_location();
            path_filename = find_path_file(script_name);
            if !path_filename.is_null() {
                free(filename as *mut c_void);
                filename = path_filename;
                fd = open(filename, O_RDONLY as libc::c_int);
            } else {
                *c___errno_location() = e;
            }
        }
        if fd < 0 {
            e = *c___errno_location();
            file_error(filename);
            end_job_control();
            sh_exit(if e == ENOENT!() {
                EX_NOTFOUND as libc::c_int
            } else {
                EX_NOINPUT as libc::c_int
            });
        }

        free(*dollar_vars.as_mut_ptr().offset(0 as isize) as *mut libc::c_void);
        *dollar_vars.as_mut_ptr().offset(0 as isize) = if !exec_argv0.is_null() {
            savestring!(exec_argv0)
        } else {
            savestring!(script_name)
        };

        if !exec_argv0.is_null() {
            free(exec_argv0 as *mut c_void);
            exec_argv0 = 0 as *mut libc::c_char;
        }

        if file_isdir(filename) != 0 {
            *c___errno_location() = EISDIR!();
            file_error(filename);
            end_job_control();
            sh_exit(EX_NOINPUT as libc::c_int);
        }

        GET_ARRAY_FROM_VAR!(
            b"FUNCNAME\0" as *const u8 as *const libc::c_char,
            funcname_v,
            funcname_a
        );
        GET_ARRAY_FROM_VAR!(
            b"BASH_SOURCE\0" as *const u8 as *const libc::c_char,
            bash_source_v,
            bash_source_a
        );
        GET_ARRAY_FROM_VAR!(
            b"BASH_LINENO\0" as *const u8 as *const libc::c_char,
            bash_lineno_v,
            bash_lineno_a
        );

        array_push!(bash_source_a, filename);
        if !bash_lineno_a.is_null() {
            t = c_itos(executing_line_number() as intmax_t);
            array_push!(bash_lineno_a, t);
            free(t as *mut c_void);
        }

        array_push!(
            funcname_a,
            b"main\0" as *const u8 as *const libc::c_char as *mut libc::c_char
        );

        fd_is_tty = isatty(fd);

        if fd_is_tty == 0 && lseek(fd, 0, 1) != -1 as libc::c_long {
            sample_len = read(
                fd,
                sample.as_mut_ptr() as *mut libc::c_void,
                (::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong)
                    .try_into()
                    .unwrap(),
            ) as libc::c_int;

            if sample_len < 0 {
                e = *c___errno_location();
                if c_fstat(fd, &mut sb) == 0 && S_ISDIR!(sb.st_mode) {
                    *c___errno_location() = EISDIR!();
                    file_error(filename);
                } else {
                    *c___errno_location() = e;
                    file_error(filename);
                }

                end_job_control();
                exit(EX_NOEXEC as libc::c_int);
            } else {
                if sample_len > 0 && check_binary_file(sample.as_mut_ptr(), sample_len) != 0 {
                    internal_error(
                        b"%s: cannot execute binary file\0" as *const u8 as *const libc::c_char,
                        filename,
                    );
                    end_job_control();
                    exit(EX_BINARY_FILE as libc::c_int);
                }
            }
            lseek(fd, 0 as libc::c_long, 0);
        }

        fd = move_to_high_fd(fd, 1, -(1 as libc::c_int));

        default_buffered_input = fd;
        SET_CLOSE_ON_EXEC!(default_buffered_input);

        if interactive_shell != 0 && fd_is_tty != 0 {
            dup2(fd, 0 as libc::c_int);
            close(fd);
            fd = 0;
            default_buffered_input = 0;
        } else if forced_interactive != 0 && fd_is_tty == 0 {
            init_interactive_script();
        }
        free(filename as *mut c_void);
        reading_shell_script = 1;
        return fd;
    }
}

fn set_bash_input() {
    unsafe {
        if interactive == 0 {
            c_sh_unset_nodelay_mode(default_buffered_input);
        } else {
            c_sh_unset_nodelay_mode(fileno(stdin));
        }
        if interactive != 0 && no_line_editing == 0 {
            with_input_from_stdin();
        } else if interactive == 0 {
            with_input_from_buffered_stream(
                default_buffered_input,
                *dollar_vars.as_mut_ptr().offset(0 as isize),
            );
        } else {
            with_input_from_stream(default_input, *dollar_vars.as_mut_ptr().offset(0 as isize));
        };
    }
}

#[no_mangle]
pub fn unset_bash_input(check_zero: libc::c_int) {
    unsafe {
        if check_zero != 0 && default_buffered_input >= 0
            || check_zero == 0 && default_buffered_input > 0
        {
            close_buffered_fd(default_buffered_input);
            bash_input.location.buffered_fd = -(1 as libc::c_int);
            default_buffered_input = bash_input.location.buffered_fd;
            bash_input.type_0 = st_none;
        }
    }
}

fn set_shell_name(argv0: *mut libc::c_char) {
    unsafe {
        shell_name = (if !argv0.is_null() {
            base_pathname(argv0)
        } else {
            PROGRAM!()
        }) as *mut libc::c_char;

        if !argv0.is_null() && *argv0 as libc::c_int == '-' as i32 {
            if *shell_name as libc::c_int == '-' as i32 {
                shell_name = shell_name.offset(1);
            }
            login_shell = 1;
        }

        if *shell_name.offset(0 as isize) as libc::c_int == 's' as i32
            && *shell_name.offset(1 as isize) as libc::c_int == 'h' as i32
            && *shell_name.offset(2 as isize) as libc::c_int == '\u{0}' as i32
        {
            act_like_sh += 1;
        }
        if *shell_name.offset(0 as isize) as libc::c_int == 's' as i32
            && *shell_name.offset(1 as isize) as libc::c_int == 'u' as i32
            && *shell_name.offset(2 as isize) as libc::c_int == '\u{0}' as i32
        {
            su_shell += 1;
        }

        shell_name = (if !argv0.is_null() { argv0 } else { PROGRAM!() }) as *mut libc::c_char;
        FREE!(*dollar_vars.as_mut_ptr().offset(0 as libc::c_int as isize));
        *dollar_vars.as_mut_ptr().offset(0 as isize) = savestring!(shell_name);

        if shell_name.is_null()
            || *shell_name == 0
            || *shell_name.offset(0 as isize) as libc::c_int == '-' as i32
                && *shell_name.offset(1 as isize) == 0
        {
            shell_name = PROGRAM!();
        }
    }
}

fn set_option_defaults() {
    unsafe {
        enable_history_list = 0;
    }
}

fn reset_option_defaults() {
    unsafe {
        enable_history_list = -(1 as libc::c_int);
    }
}

fn init_interactive() {
    unsafe {
        startup_state = 1;
        interactive_shell = startup_state;
        expand_aliases = interactive_shell;
        interactive = 1;

        if enable_history_list == -(1 as libc::c_int) {
            enable_history_list = 1;
        }
        remember_on_history = enable_history_list;
        histexp_flag = history_expansion;
    }
}

fn init_noninteractive() {
    unsafe {
        if enable_history_list == -(1 as libc::c_int) {
            enable_history_list = 0;
        }

        bash_history_reinit(0);

        interactive = 0;
        startup_state = interactive;
        interactive_shell = startup_state;
        expand_aliases = posixly_correct;
        no_line_editing = 1;
        set_job_control((forced_interactive != 0 || jobs_m_flag != 0) as libc::c_int);
    }
}

fn init_interactive_script() {
    unsafe {
        if enable_history_list == -(1 as libc::c_int) {
            enable_history_list = 1;
        }
        init_noninteractive();
        startup_state = 1;
        interactive_shell = startup_state;
        expand_aliases = interactive_shell;

        remember_on_history = enable_history_list;
    }
}

#[no_mangle]
pub fn get_current_user_info() {
    let mut entry: *mut passwd = 0 as *mut passwd;
    unsafe {
        if (current_user.user_name).is_null() {
            entry = getpwuid(current_user.uid) as *mut src_common::passwd;
            if !entry.is_null() {
                current_user.user_name = savestring!((*entry).pw_name);
                current_user.shell = if !((*entry).pw_shell).is_null()
                    && *((*entry).pw_shell).offset(0 as isize) as libc::c_int != 0
                {
                    savestring!((*entry).pw_shell)
                } else {
                    savestring!(b"/bin/sh\0" as *const u8 as *mut libc::c_char)
                };
                current_user.home_dir = savestring!((*entry).pw_dir);
            } else {
                current_user.user_name = b"I have no name!\0" as *const u8 as *mut libc::c_char;
                current_user.user_name = savestring!(current_user.user_name);
                current_user.shell = savestring!(b"/bin/sh\0" as *const u8 as *const libc::c_char);
                current_user.home_dir = savestring!(b"/\0" as *const u8 as *const libc::c_char);
            }
            endpwent();
        }
    }
}

fn shell_initialize() {
    let mut hostname: [libc::c_char; 256] = [0; 256];
    let mut should_be_restricted: libc::c_int = 0;
    unsafe {
        if shell_initialized == 0 {
            c_sh_setlinebuf(stderr);
            c_sh_setlinebuf(stdout);
        }
    }
    initialize_shell_builtins();

    initialize_traps();
    initialize_signals(0);
    unsafe {
        if current_host_name.is_null() {
            if gethostname(hostname.as_mut_ptr(), 255 as usize) < 0 {
                current_host_name = b"??host??\0" as *const u8 as *mut libc::c_char;
            } else {
                current_host_name = savestring!(hostname.as_mut_ptr());
            }
        }
        if interactive_shell != 0 {
            get_current_user_info();
        }
    }
    tilde_initialize();
    unsafe {
        should_be_restricted = shell_is_restricted(shell_name);

        initialize_shell_variables(
            shell_environment,
            (privileged_mode != 0
                || restricted != 0
                || should_be_restricted != 0
                || running_setuid != 0) as libc::c_int,
        );

        initialize_job_control(jobs_m_flag);
    }
    initialize_bash_input();

    initialize_flags();
    unsafe {
        initialize_shell_options(
            (privileged_mode != 0
                || restricted != 0
                || should_be_restricted != 0
                || running_setuid != 0) as libc::c_int,
        );
        initialize_bashopts(
            (privileged_mode != 0
                || restricted != 0
                || should_be_restricted != 0
                || running_setuid != 0) as libc::c_int,
        );
    }
}

fn shell_reinitialize() {
    unsafe {
        primary_prompt = PPROMPT!();
        secondary_prompt = SPROMPT!();

        current_command_number = 1;

        no_profile = 1;
        no_rc = no_profile;

        executing = 0;
        interactive = executing;
        make_login_shell = interactive;
        login_shell = make_login_shell;

        last_command_exit_value = 0;
        line_number = last_command_exit_value;
        do_version = line_number;
        debugging = do_version;

        interactive_shell = 0;
        forced_interactive = interactive_shell;

        running_in_background = 0;
        subshell_environment = running_in_background;

        expand_aliases = 0;
        bash_argv_initialized = 0;

        enable_history_list = 0;
        bash_history_reinit(enable_history_list);

        restricted = 0;
        bashrc_file = DEFAULT_BASHRC!();

        delete_all_contexts(shell_variables);
        delete_all_variables(shell_functions);

        reinit_special_variables();

        bashline_reinitialize();

        shell_reinitialized = 1;
    }
}

fn show_shell_usage(fp: *mut FILE, extra: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut set_opts: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        if extra != 0 {
            fprintf(
                fp,
                b"utshell, version %s-(%s)\n\0" as *const u8 as *const libc::c_char,
                shell_version_string(),
                c_get_mach_type(),
            );
        }
        fprintf(
        fp,
        b"Usage:\t%s [GNU long option] [option] ...\n\t%s [GNU long option] [option] script-file ...\n\0" as *const u8 as *const libc::c_char, 
        shell_name,
        shell_name,
    );
        fputs(
            b"GNU long options:\n\0" as *const u8 as *const libc::c_char,
            fp,
        );

        i = 0;
        while !(long_args[i as usize].name).is_null() {
            fprintf(
                fp,
                b"\t--%s\n\0" as *const u8 as *const libc::c_char,
                long_args[i as usize].name,
            );
            i += 1;
        }
        fputs(
            b"Shell options:\n\0" as *const u8 as *const libc::c_char,
            fp,
        );
        fputs(
            b"\t-ilrsD or -c command or -O shopt_option\t\t(invocation only)\n\0" as *const u8
                as *const libc::c_char,
            fp,
        );

        i = 0;
        set_opts = 0 as *mut libc::c_char;
        while !((*shell_builtins.offset(i as isize)).name).is_null() {
            if STREQ(
                (*shell_builtins.offset(i as isize)).name,
                b"set\0" as *const u8 as *const libc::c_char,
            ) {
                set_opts = savestring!((*shell_builtins.offset(i as isize)).short_doc);
                break;
            } else {
                i += 1;
            }
        }

        if !set_opts.is_null() {
            s = strchr(set_opts, '[' as i32);
            if s.is_null() {
                s = set_opts;
            }
            loop {
                s = s.offset(1);
                if !(*s as libc::c_int == '-' as i32) {
                    break;
                }
            }
            t = strchr(s, ']' as i32);
            if !t.is_null() {
                *t = '\u{0}' as i32 as libc::c_char;
            }
            fprintf(
                fp,
                b"\t-%s or -o option\n\0" as *const u8 as *const libc::c_char,
                s,
            );

            free(set_opts as *mut c_void);
        }

        if extra != 0 {
            fprintf(
                fp,
                b"Type `%s -c \"help set\"' for more information about shell options.\n\0"
                    as *const u8 as *const libc::c_char,
                shell_name,
            );
            fprintf(
                fp,
                b"Type `%s -c help' for more information about shell builtin commands.\n\0"
                    as *const u8 as *const libc::c_char,
                shell_name,
            );
            fprintf(
                fp,
                b"Use the `bashbug' command to report bugs.\n\0" as *const u8
                    as *const libc::c_char,
            );
            fprintf(fp, b"\n\0" as *const u8 as *const libc::c_char);
            fprintf(
                fp,
                b"utshell home page: <http://www.chinauos.com/>\n\0" as *const u8
                    as *const libc::c_char,
            );
            fprintf(
                fp,
                b"General help using GNU software: <http://www.chinauos.com/>\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
}

fn add_shopt_to_alist(opt: *mut libc::c_char, on_or_off: libc::c_int) {
    unsafe {
        if shopt_ind >= shopt_len {
            shopt_len += 8;
            shopt_alist = libc::realloc(
                shopt_alist as *mut libc::c_void,
                (shopt_len as usize)
                    .wrapping_mul(::std::mem::size_of::<STRING_INT_ALIST>() as usize),
            ) as *mut STRING_INT_ALIST;
        }

        (*shopt_alist.offset(shopt_ind as isize)).word = opt;
        (*shopt_alist.offset(shopt_ind as isize)).token = on_or_off;
        shopt_ind += 1;
    }
}

fn run_shopt_alist() {
    unsafe {
        let mut i: libc::c_int = 0;

        i = 0;
        while i < shopt_ind {
            if shopt_setopt(
                (*shopt_alist.offset(i as isize)).word,
                ((*shopt_alist.offset(i as isize)).token == '-' as i32) as libc::c_int,
            ) != EXECUTION_SUCCESS as libc::c_int
            {
                exit(EX_BADUSAGE as libc::c_int);
            }
            i += 1;
        }
        free(shopt_alist as *mut c_void);
        shopt_alist = 0 as *mut STRING_INT_ALIST;
        shopt_len = 0;
        shopt_ind = shopt_len;
    }
}
