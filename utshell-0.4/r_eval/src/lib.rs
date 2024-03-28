//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later


use libc::{c_int, c_char, c_void};
use r_bash::*;


#[macro_export]
macro_rules! setjmp_nosigs {
    ($x:expr) => {
        __sigsetjmp($x, 0)
    };
}



#[no_mangle]
pub unsafe extern "C" fn reader_loop() -> c_int {
    let mut flag:bool = false;
    let mut our_indirection_level: c_int = 0;
    let mut current_command: *mut COMMAND = 0 as *mut COMMAND;

    current_command = 0 as *mut c_void as *mut COMMAND;
   
    indirection_level += 1;
    our_indirection_level = indirection_level;

    if just_one_command != 0 {
        reset_readahead_token();
    }
    while EOF_Reached == 0 {
        let mut code: c_int = 0;

        code = setjmp_nosigs!(top_level.as_mut_ptr());

        unlink_fifo_list();

        if interactive_shell != 0
            && signal_is_ignored(SIGINT as c_int) == 0 
            && signal_is_trapped(SIGINT as c_int) == 0  
        {
            set_signal_handler(
                SIGINT as c_int,
                sigint_sighandler  as *mut SigHandler,      //可能存在问题
            );
        }

        loop{
            if code != NOT_JUMPED as c_int {
                indirection_level = our_indirection_level;

                match code as u32 {
                    FORCE_EOF | ERREXIT  | EXITPROG   => {
                        current_command = 0 as *mut c_void as *mut COMMAND;
                        
                        if exit_immediately_on_error != 0 {
                            variable_context = 0 ;
                        }
                        EOF_Reached = EOF as i32;
                        flag = true;
                        break;
                    }
                    DISCARD => {
                        if last_command_exit_value == 0 {
                            set_exit_status(EXECUTION_FAILURE as c_int);
                        }
                        if subshell_environment != 0 {
                            current_command = 0 as *mut c_void as *mut COMMAND;
                            EOF_Reached = EOF as i32;
                            flag = true;
                            break;
                        } 

                        if !current_command.is_null() {
                            dispose_command(current_command);
                            current_command = 0 as *mut c_void as *mut COMMAND;
                        }
                        restore_sigmask();
                    }
                    _ => {
                        command_error(
                            b"reader_loop\0" as *const u8 as *const c_char,
                            3 as c_int,
                            code,
                            0 as c_int,
                        );
                    }
                }
            }

            executing = 0;
            if !temporary_env.is_null() {
                dispose_used_env_vars();
            }

            if read_command() == 0 {
                if interactive_shell == 0 && (read_but_dont_execute != 0 && rpm_requires == 0) {
                    set_exit_status(EXECUTION_SUCCESS as i32 );
                    dispose_command(global_command);
                    global_command = 0 as *mut COMMAND;
                }
                else {
                    current_command = global_command;
                    if !current_command.is_null() {
                        global_command = 0 as *mut COMMAND ;

                        if interactive != 0 && !ps0_prompt.is_null() {
                            let mut ps0_string:*mut c_char = 0 as *mut c_char; 

                            ps0_string = decode_prompt_string(ps0_prompt);
                            if !ps0_string.is_null() && *ps0_string as c_int != 0 {
                                fprintf(
                                    stderr,
                                    b"%s\0" as *const u8 as *const c_char,
                                    ps0_string,
                                );
                                fflush(stderr);
                            }

                            free(ps0_string as *mut c_void);
                        }
                        current_command_number += 1;

                        executing = 1;
                        stdin_redir = 0;
                
                        execute_command(current_command);
                        flag = true;
                        break;
                    }
                } 
            }
            else {
                if interactive == 0 {
                    EOF_Reached = EOF;
                }
            }
            break;  //跳出loop循环
        }

        if flag == true {
            flag = false;
            QUIT!();

            if !current_command.is_null()
            {
                dispose_command(current_command);
                current_command = 0 as *mut COMMAND;
            }
        } 

        if just_one_command != 0 {
            EOF_Reached = EOF;
        }    
    }

    indirection_level -= 1;
    return last_command_exit_value;
}




#[no_mangle]
pub unsafe extern "C" fn pretty_print_loop() -> c_int {
    let mut current_command: *mut COMMAND = 0 as *mut COMMAND;
    let mut command_to_print: *mut c_char = 0 as *mut c_char;
    let mut code: c_int = 0;
    let mut global_posix_mode: c_int = 0;
    let mut last_was_newline: c_int = 0;

    global_posix_mode = posixly_correct;
    last_was_newline = 0 ;
    while EOF_Reached == 0 {
        code = setjmp_nosigs!(top_level.as_mut_ptr());
        if code != 0 {
            return EXECUTION_FAILURE as c_int;
        }
        if read_command() == 0 {
            current_command = global_command;
            global_command = 0 as *mut COMMAND;
            posixly_correct = 1 ;
            if !current_command.is_null()
                && {
                    command_to_print = make_command_string(current_command);
                    !command_to_print.is_null()
                }
            {
                printf(b"%s\n\0" as *const u8 as *const c_char, command_to_print);
                last_was_newline = 0 ;
            } else if last_was_newline == 0 {
                printf(b"\n\0" as *const u8 as *const c_char);
                last_was_newline = 1 ;
            }
            posixly_correct = global_posix_mode;
            dispose_command(current_command);
        } else {
            return 1 
        }
    }
    return 0 ;
}


unsafe extern "C" fn alrm_catcher(mut i: c_int) {
    let mut msg: *mut c_char = 0 as *mut c_char;
    msg = b"\x07timed out waiting for input: auto-logout\n\0" as *const u8 as *mut c_char;
          
    write(1 as c_int, msg as *const c_void, strlen(msg) as usize);
    bash_logout();
    jump_to_top_level(EXITPROG as c_int);
}
unsafe extern "C" fn send_pwd_to_eterm() {
    let mut pwd: *mut c_char = 0 as *mut c_char;
    let mut f: *mut c_char = 0 as *mut c_char;
    
    f = 0 as *mut c_char;
    pwd = get_string_value(b"PWD\0" as *const u8 as *const c_char);
    if pwd.is_null() {
        pwd = get_working_directory(
            b"eterm\0" as *const u8 as *const c_char as *mut c_char,
        );
        f = pwd;
    }
    fprintf(stderr, b"\x1A/%s\n\0" as *const u8 as *const c_char, pwd);
    free(f as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn execute_array_command(
    mut a: *mut ARRAY,
    mut v: *mut c_void,
) -> c_int {
    let mut tag: *mut c_char = 0 as *mut c_char;
    let mut argv: *mut *mut c_char = 0 as *mut *mut c_char;
    let mut argc: c_int = 0;
    let mut i: c_int = 0;

    tag = v as *mut c_char;
    argc = 0 ;
    argv = array_to_argv(a, &mut argc);
    i = 0 ;
    while i < argc {
        if !(*argv.offset(i as isize)).is_null()
            && *(*argv.offset(i as isize)).offset(0 ) as c_int != 0
        {
            execute_variable_command(*argv.offset(i as isize), tag);
        }
        i += 1;
    }
    strvec_dispose(argv);
    return 0  ;
}

#[macro_export]
macro_rules! var_isset {
    ($var:expr) => {
        ((*$var).value != 0 as *mut c_char) as c_int 
    };
}

#[macro_export]
macro_rules! invisible_p {
    ($var:expr) => {
        (*$var).attributes & att_invisible as c_int
    };
}

#[macro_export]
macro_rules! array_p {
    ($var:expr) => {
        (*$var).attributes & att_array as c_int
    };
}

#[macro_export]
macro_rules! array_cell {
    ($var:expr) => {
        (*$var).value as *mut ARRAY
    };
}

#[macro_export]
macro_rules! value_cell {
    ($var:expr) => {
        (*$var).value 
    };
}

#[macro_export]
macro_rules! array_num_elements {
    ($a:expr) => {
        (*$a).num_elements
    };
}

#[macro_export]
macro_rules! assoc_p {
    ($var:expr) => {
        (*$var).attributes & att_assoc as c_int
    };
}

unsafe extern "C" fn execute_prompt_command() {
    let mut command_to_execute: *mut c_char = 0 as *mut c_char;
    let mut pcv: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut pcmds: *mut ARRAY = 0 as *mut ARRAY;

    pcv = find_variable(b"PROMPT_COMMAND\0" as *const u8 as *const c_char);
    if pcv.is_null()
        || var_isset!(pcv) == 0  
        || invisible_p!(pcv) != 0
    {
        return;
    }
    if array_p!(pcv) != 0 {
        pcmds = array_cell!(pcv);
        if !pcmds.is_null() && array_num_elements!(pcmds) > 0 {
            execute_array_command(
                pcmds,
                b"PROMPT_COMMAND\0" as *const u8 as *const c_char
                    as *mut c_void,
            );
        }
        return;
    } else {
        if assoc_p!(pcv) != 0 {
            return;
        }
    }
    command_to_execute = value_cell!(pcv);
    if !command_to_execute.is_null() && *command_to_execute as c_int != 0 {
        execute_variable_command(
            command_to_execute,
            b"PROMPT_COMMAND\0" as *const u8 as *const c_char as *mut c_char,
        );
    }
}

pub type stream_type = libc::c_uint;
pub const st_bstream: stream_type = 4;
pub const st_string: stream_type = 3;
pub const st_stream: stream_type = 2;
pub const st_none: stream_type = 0;
pub const st_stdin: stream_type = 1;

#[no_mangle]
pub unsafe extern "C" fn parse_command() -> c_int {
    let mut r: c_int = 0;

    need_here_doc = 0 ;
    run_pending_traps();

    if interactive != 0
        && bash_input.type_ as libc::c_uint != st_string  
        && parser_expanding_alias() == 0 
    {
        if no_line_editing != 0
            || bash_input.type_ as libc::c_uint
                == st_stdin  && parser_will_prompt() != 0
        {
            execute_prompt_command();
        }
        if running_under_emacs == 2 {
            send_pwd_to_eterm();
        }
    }
    current_command_line_count = 0  ;
    r = yyparse();
    if need_here_doc != 0 {
        gather_here_documents();
    }
    return r;
}

#[macro_export]
macro_rules! QUIT {
    () => {
        if terminating_signal != 0 {
            termsig_handler(terminating_signal);
        }
        if interrupt_state != 0 {
            throw_to_top_level();
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn read_command() -> c_int {
    let mut tmout_var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut tmout_len: c_int = 0;
    let mut result: c_int = 0;
    let mut old_alrm: *mut SigHandler = 0 as *mut SigHandler;

    set_current_prompt_level(1 );
    global_command = 0 as *mut COMMAND;
    tmout_var = 0 as *mut SHELL_VAR;
    tmout_len = 0 ;
    old_alrm = 0 as *mut SigHandler;

    if interactive != 0 {
        tmout_var = find_variable(b"TMOUT\0" as *const u8 as *const c_char);
       
        if !tmout_var.is_null() && !((*tmout_var).value).is_null() {
            tmout_len = atoi((*tmout_var).value);
            if tmout_len > 0 {
                old_alrm = set_signal_handler(
                    SIGALRM as c_int,
                    alrm_catcher as *mut SigHandler
                );
                alarm(tmout_len as libc::c_uint);
            }
        }
    }

    QUIT!();

    current_command_line_count = 0 ;
    result = parse_command();
    if interactive != 0 && !tmout_var.is_null() && tmout_len > 0 {
        alarm(0 );
        set_signal_handler(SIGALRM as c_int, old_alrm );
    }
    return result;
}