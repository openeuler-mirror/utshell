/*
 * SPDX-FileCopyrightText: 2025 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: GPL-2.0-or-later
 */
//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

use crate::arrayfunc::{assign_array_element, valid_array_reference};
use crate::copycmd::copy_word_list;
use crate::dispose_cmd::dispose_words;
use crate::error::get_name_for_error;
use crate::execute_cmd::executing_line_number;
use crate::general::{all_digits, legal_number};
use crate::list::list_length;
use crate::sig::{jump_to_top_level, termsig_handler, throw_to_top_level, top_level_cleanup};
use crate::src_common::*;
use crate::subst::invalidate_cached_quoted_dollar_at;
use crate::trap::{decode_signal, signal_name};
use crate::variables::{bind_variable, find_variable, unbind_variable};

use crate::builtins::{
    bashgetopt::{internal_getopt, reset_internal_getopt},
    builtins::{current_builtin, num_shell_builtins, shell_builtins},
    help::builtin_help,
    kill::kill_builtin,
    return_1::return_builtin,
    set::set_builtin,
};

fn ISOPTION(s: *const libc::c_char, c: libc::c_char) -> bool {
    unsafe {
        return *s == '-' as libc::c_char && *s.offset(1) == c && *s.offset(2) != 0;
    }
}

/* **************************************************************** */
/*                                                                  */
/*           Error reporting, usage, and option processing          */
/*                                                                  */
/* **************************************************************** */

/* This is a lot like report_error (), but it is for shell builtins
instead of shell control structures, and it won't ever exit the
shell. */

#[no_mangle]
fn builtin_error_prolog() {
    let name: *mut libc::c_char;

    unsafe {
        name = get_name_for_error();
        eprint!("{}: ", CStr::from_ptr(name).to_str().unwrap());

        if interactive_shell == 0 {
            eprint!("line {}: ", executing_line_number())
        }

        if !this_command_name.is_null() && *this_command_name != 0 {
            eprint!("{}: ", CStr::from_ptr(this_command_name).to_str().unwrap());
        }
    }
}

/* Print a usage summary for the currently-executing builtin command. */
#[no_mangle]
pub fn builtin_usage() {
    unsafe {
        if !this_command_name.is_null() && *this_command_name != 0 {
            eprint!(
                "{}: usage: ",
                CStr::from_ptr(this_command_name).to_str().unwrap()
            );
            eprintln!(
                "{}",
                CStr::from_ptr((*current_builtin).short_doc)
                    .to_str()
                    .unwrap()
            );
        }
    }
}

/* Return if LIST is NULL else barf and jump to top_level.  Used by some
builtins that do not accept arguments. */
#[no_mangle]
pub fn no_args(list: *mut WordList) {
    unsafe {
        if !list.is_null() {
            let c_str = CString::new("too many arguments").unwrap();
            let c_ptr = c_str.as_ptr();
            builtin_error(c_ptr);
            top_level_cleanup();
            jump_to_top_level(DISCARD!());
        }
    }
}

/* Check that no options were given to the currently-executing builtin,
and return 0 if there were options. */
#[no_mangle]
pub fn no_options(list: *mut WordList) -> i32 {
    let opt: i32;

    reset_internal_getopt();
    let c_str = CString::new("").unwrap();
    let c_ptr = c_str.as_ptr();
    opt = internal_getopt(list, c_ptr as *mut libc::c_char);
    if opt != -1 {
        if opt == GETOPT_HELP!() {
            builtin_help();
            return 2;
        }
        builtin_usage();
        return 1;
    }
    return 0;
}

#[no_mangle]
pub fn sh_needarg(s: *mut libc::c_char) {
    unsafe {
        let c_str = CString::new("%s: option requires an argument").unwrap();
        let c_ptr = c_str.as_ptr();
        builtin_error(c_ptr, s);
    }
}

#[no_mangle]
pub fn sh_neednumarg(s: *mut libc::c_char) {
    unsafe {
        let c_str = CString::new("%s: numeric argument requited").unwrap();
        let c_ptr = c_str.as_ptr();
        builtin_error(c_ptr, s);
    }
}

#[no_mangle]
pub fn sh_notfound(s: *mut libc::c_char) {
    unsafe {
        let c_str = CString::new("%s: not found").unwrap();
        let c_ptr = c_str.as_ptr();
        builtin_error(c_ptr, s);
    }
}

/* Function called when one of the builtin commands detects an invalid
option. */
#[no_mangle]
pub fn sh_invalidopt(s: *mut libc::c_char) {
    unsafe {
        let c_str = CString::new("%s: invalid option").unwrap();
        let c_ptr = c_str.as_ptr();
        builtin_error(c_ptr, s);
    }
}

#[no_mangle]
pub fn sh_invalidoptname(s: *mut libc::c_char) {
    unsafe {
        let c_str = CString::new("%s: invalid option name").unwrap();
        let c_ptr = c_str.as_ptr();
        builtin_error(c_ptr, s);
    }
}

#[no_mangle]
pub fn sh_invalidid(s: *mut libc::c_char) {
    unsafe {
        let c_str = CString::new("`%s': not a valid identifier").unwrap();
        let c_ptr = c_str.as_ptr();
        builtin_error(c_ptr, s);
    }
}

#[no_mangle]
pub fn sh_invalidnum(s: *mut libc::c_char) {
    unsafe {
        // let msg:*mut libc::c_char;
        let mut msg = String::new();
        let mut mag_ptr: *const libc::c_char = std::ptr::null_mut();

        if *s == b'0' as libc::c_char && isdigit(*s.offset(1) as libc::c_int) != 0 {
            msg.push_str("invalid octal number");
            mag_ptr = msg.as_ptr() as *mut libc::c_char;
        } else if *s == b'0' as libc::c_char && *s.offset(1) == b'x' as libc::c_char {
            msg.push_str("invalid hex number");
            mag_ptr = msg.as_ptr() as *mut libc::c_char;
        } else {
            msg.push_str("invalid number");
            mag_ptr = msg.as_ptr() as *mut libc::c_char;
        }

        let c_str = CString::new("%s: %s").unwrap();
        let c_ptr = c_str.as_ptr();
        builtin_error(c_ptr, s, mag_ptr);
    }
}

#[no_mangle]
pub fn sh_invalidsig(s: *mut libc::c_char) {
    unsafe {
        let c_str = CString::new("%s: invalid signal specification").unwrap();
        let c_ptr = c_str.as_ptr();
        builtin_error(c_ptr, s);
    }
}

#[no_mangle]
pub fn sh_badpid(s: *mut libc::c_char) {
    unsafe {
        let c_str = CString::new("`%s': not a pid or valid job spec").unwrap();
        let c_ptr = c_str.as_ptr();
        builtin_error(c_ptr, s);
    }
}

#[no_mangle]
pub fn sh_readonly(s: *mut libc::c_char) {
    unsafe {
        let c_str = CString::new("%s: readonly variable").unwrap();
        let c_ptr = c_str.as_ptr();
        builtin_error(c_ptr, s);
    }
}

#[no_mangle]
pub fn sh_erange(s: *mut libc::c_char, desc: *mut libc::c_char) {
    unsafe {
        if !s.is_null() {
            let c_str = CString::new("%s: %s out of range").unwrap();
            let c_ptr = c_str.as_ptr();
            if !desc.is_null() {
                builtin_error(c_ptr, s, desc);
            } else {
                let desc_str = CString::new("argument").unwrap();
                let desc_ptr = desc_str.as_ptr();
                builtin_error(c_ptr, s, desc_ptr);
            }
        } else {
            let c_str = CString::new("%s out of range").unwrap();
            let c_ptr = c_str.as_ptr();
            let desc_str = CString::new("argument").unwrap();
            let desc_ptr = desc_str.as_ptr();
            builtin_error(c_ptr, desc_ptr)
        }
    }
}

#[no_mangle]
pub fn sh_badjob(s: *mut libc::c_char) {
    unsafe {
        let c_str = CString::new("%s: no job control").unwrap();
        let c_ptr = c_str.as_ptr();
        builtin_error(c_ptr, s);
    }
}

#[no_mangle]
pub fn sh_nojobs(s: *mut libc::c_char) {
    unsafe {
        if !s.is_null() {
            let c_str = CString::new("%s: no job control").unwrap();
            let c_ptr = c_str.as_ptr();
            builtin_error(c_ptr, s);
        } else {
            let c_str = CString::new("no job control").unwrap();
            let c_ptr = c_str.as_ptr();
            builtin_error(c_ptr);
        }
    }
}

#[no_mangle]
pub fn sh_restricted(s: *mut libc::c_char) {
    unsafe {
        if !s.is_null() {
            let c_str = CString::new("%s: restricted").unwrap();
            let c_ptr = c_str.as_ptr();
            builtin_error(c_ptr, s);
        } else {
            let c_str = CString::new("restricted").unwrap();
            let c_ptr = c_str.as_ptr();
            builtin_error(c_ptr);
        }
    }
}

#[no_mangle]
pub fn sh_notbuiltin(s: *mut libc::c_char) {
    unsafe {
        let c_str = CString::new("%s: not a shell builtin").unwrap();
        let c_ptr = c_str.as_ptr();
        builtin_error(c_ptr, s);
    }
}

#[no_mangle]
pub fn sh_wrerror() {
    unsafe {
        let c_str = CString::new("write error: %s").unwrap();
        let c_ptr = c_str.as_ptr();
        builtin_error(c_ptr, strerror(*__errno_location()));
    }
}

#[no_mangle]
pub fn sh_ttyerror(set: i32) {
    unsafe {
        if set != 0 {
            let c_str = CString::new("error setting terminal attributes: %s").unwrap();
            let c_str_ptr = c_str.as_ptr();
            builtin_error(c_str_ptr, strerror(*__errno_location()));
        } else {
            let c_str = CString::new("error getting terminal attributes: %s").unwrap();
            let c_str_ptr = c_str.as_ptr();
            builtin_error(c_str_ptr, strerror(*__errno_location()));
        }
    }
}

#[no_mangle]
pub fn sh_chkwrite(s: i32) -> i32 {
    unsafe {
        QUIT!();
        fflush(stdout);
        QUIT!();

        if ferror(stdout) != 0 {
            sh_wrerror();
            fpurge(stdout);
            libc::clearerr(stdout);
            return EXECUTION_FAILURE!();
        }
        return s;
    }
}

/* **************************************************************** */
/*                                                                  */
/*           Shell positional parameter manipulation                */
/*                                                                  */
/* **************************************************************** */

/* Convert a WordList into a C-style argv.  Return the number of elements
in the list in *IP, if IP is non-null.  A convenience function for
loadable builtins; also used by `test'. */
#[no_mangle]
pub fn make_builtin_argv(list: *mut WordList, ip: *mut i32) -> *mut *mut libc::c_char {
    let argv: *mut *mut libc::c_char;
    unsafe {
        argv = strvec_from_word_list(list, 0, 1, ip);
        *argv.offset(0) = this_command_name;
        return argv;
    }
}

/* Remember LIST in $1 ... $9, and REST_OF_ARGS.  If DESTRUCTIVE is
non-zero, then discard whatever the existing arguments are, else
only discard the ones that are to be replaced.  Set POSPARAM_COUNT
to the number of args assigned (length of LIST). */
#[no_mangle]
pub fn remember_args(mut list: *mut WordList, destructive: i32) {
    let mut i: i32;

    unsafe {
        posparam_count = 0;
        i = 1;
        while i < 10 {
            if (destructive != 0 || list != std::ptr::null_mut())
                && *dollar_vars.as_ptr().offset(i as isize) != std::ptr::null_mut()
            {
                free(*dollar_vars.as_ptr().offset(i as isize) as *mut c_void);
                *dollar_vars.as_mut_ptr().offset(i as isize) = std::ptr::null_mut();
            }

            if !list.is_null() {
                posparam_count = i;
                *dollar_vars.as_mut_ptr().offset(posparam_count as isize) =
                    savestring!((*(*list).word).word);
                list = (*list).next;
            }
            i += 1;
        }

        /* If arguments remain, assign them to REST_OF_ARGS.
        Note that copy_word_list (NULL) returns NULL, and
        that dispose_words (NULL) does nothing. */
        if destructive != 0 || !list.is_null() {
            dispose_words(rest_of_args);
            rest_of_args = copy_word_list(list);
            posparam_count += list_length(list as *mut GENERIC_LIST); //there is may be problems
        }

        if destructive != 0 {
            set_dollar_vars_changed();
        }
        invalidate_cached_quoted_dollar_at();
    }
}

#[no_mangle]
pub fn shift_args(mut times: i32) {
    let mut temp: *mut WordList;
    // let mut count:i32;

    unsafe {
        if times <= 0 {
            return;
        }
        loop {
            let fresh3 = times;
            times = times - 1;
            if !(fresh3 > 0 as libc::c_int) {
                break;
            }
            if *dollar_vars.as_ptr().offset(1) != std::ptr::null_mut() {
                free(*dollar_vars.as_ptr().offset(1) as *mut c_void);
            }
            for count in 1..9 {
                *dollar_vars.as_mut_ptr().offset(count) = *dollar_vars.as_ptr().offset(count + 1)
            }
            if !rest_of_args.is_null() {
                temp = rest_of_args;
                *dollar_vars.as_mut_ptr().offset(9) = savestring!((*(*temp).word).word);
                rest_of_args = (*rest_of_args).next;
                (*temp).next = std::ptr::null_mut();
                dispose_words(temp);
            } else {
                *dollar_vars.as_mut_ptr().offset(9) = std::ptr::null_mut();
            }
            posparam_count -= 1;
        }
    }
}

#[no_mangle]
pub fn number_of_args() -> i32 {
    unsafe {
        return posparam_count;
    }
}

static mut changed_dollar_vars: i32 = 0;

/* Have the dollar variables been reset to new values since we last
checked? */

#[no_mangle]
pub fn dollar_vars_changed() -> i32 {
    unsafe {
        return changed_dollar_vars;
    }
}

#[no_mangle]
pub fn set_dollar_vars_unchanged() {
    unsafe {
        changed_dollar_vars = 0;
    }
}

#[no_mangle]
pub fn set_dollar_vars_changed() {
    unsafe {
        if variable_context != 0 {
            changed_dollar_vars |= ARGS_FUNC!();
        } else if this_shell_builtin == Some(set_builtin) {
            //there may be problems
            changed_dollar_vars |= ARGS_SETBLTIN!();
        } else {
            changed_dollar_vars |= ARGS_INVOC!();
        }
    }
}

/* **************************************************************** */
/*                                                                  */
/*              Validating numeric input and arguments              */
/*                                                                  */
/* **************************************************************** */

/* Read a numeric arg for this_command_name, the name of the shell builtin
that wants it.  LIST is the word list that the arg is to come from.
Accept only the numeric argument; report an error if other arguments
follow.  If FATAL is 1, call throw_to_top_level, which exits the
shell; if it's 2, call jump_to_top_level (DISCARD), which aborts the
current command; if FATAL is 0, return an indication of an invalid
number by setting *NUMOK == 0 and return -1. */
#[no_mangle]
pub fn get_numeric_arg(mut list: *mut WordList, fatal: i32, count: *mut intmax_t) -> i32 {
    let arg: *mut libc::c_char;
    unsafe {
        if !count.is_null() {
            *count = 1;
        }

        if !list.is_null()
            && !(*list).word.is_null()
            && ISOPTION((*(*list).word).word, b'-' as libc::c_char)
        {
            list = (*list).next;
        }

        if !list.is_null() {
            arg = (*(*list).word).word;
            if arg.is_null() || legal_number(arg, count) == 0 {
                if !(*(*list).word).word.is_null() {
                    sh_neednumarg((*(*list).word).word);
                } else {
                    sh_neednumarg(String::from("`'").as_ptr() as *mut libc::c_char);
                }

                if fatal == 0 {
                    return 0;
                } else if fatal == 1 {
                    /* fatal == 1; abort */
                    throw_to_top_level();
                } else {
                    /* fatal == 2; discard current command */
                    top_level_cleanup();
                    jump_to_top_level(DISCARD!());
                }
            }
            no_args((*list).next);
        }
        return 1;
    }
}

/* Get an eight-bit status value from LIST */
#[no_mangle]
pub fn get_exitstat(mut list: *mut WordList) -> i32 {
    let status: i32;
    let mut sval: intmax_t = 0;
    let arg: *mut libc::c_char;

    unsafe {
        if !list.is_null()
            && !(*list).word.is_null()
            && ISOPTION((*(*list).word).word, b'-' as libc::c_char)
        {
            list = (*list).next;
        }

        if list.is_null() {
            /* If we're not running the DEBUG trap, the return builtin, when not
            given any arguments, uses the value of $? before the trap ran.  If
            given an argument, return uses it.  This means that the trap can't
            change $?.  The DEBUG trap gets to change $?, though, since that is
            part of its reason for existing, and because the extended debug mode
            does things with the return value. */
            if this_shell_builtin == Some(return_builtin)
                && running_trap > 0
                && running_trap != DEBUG_TRAP!() + 1
            {
                return trap_saved_exit_value;
            }
            return last_command_exit_value;
        }

        arg = (*(*list).word).word;
        if arg.is_null() || legal_number(arg, &mut sval) == 0 {
            if !(*(*list).word).word.is_null() {
                sh_neednumarg((*(*list).word).word);
            } else {
                sh_neednumarg(String::from("`'").as_ptr() as *mut libc::c_char);
            }

            return EX_BADUSAGE!();
        }

        no_args((*list).next);

        status = (sval & 255) as i32;
        return status;
    }
}

/* Return the octal number parsed from STRING, or -1 to indicate
that the string contained a bad number. */
#[no_mangle]
pub fn read_octal(mut string: *mut libc::c_char) -> i32 {
    let mut result: i32 = 0;
    let mut digits: i32 = 0;

    unsafe {
        while *string != 0 && ISOCTAL!(*string) {
            digits += 1;
            result = (result * 8) + (*string - b'0' as libc::c_char) as i32;
            string = (string as usize + 1) as *mut libc::c_char;
            if result > 0o7777 {
                return -1;
            }
        }

        if digits == 0 || *string != 0 {
            result = -1;
        }

        return result;
    }
}

/* **************************************************************** */
/*                                                                  */
/*           Manipulating the current working directory             */
/*                                                                  */
/* **************************************************************** */

/* Return a consed string which is the current working directory.
FOR_WHOM is the name of the caller for error printing.  */

pub static mut the_current_working_directory: *mut libc::c_char = std::ptr::null_mut();

#[no_mangle]
pub fn get_working_directory(for_whom: *mut libc::c_char) -> *mut libc::c_char {
    unsafe {
        if no_symbolic_links != 0 {
            FREE!(the_current_working_directory as *mut c_void);
            the_current_working_directory = std::ptr::null_mut();
        }

        if the_current_working_directory.is_null() {
            the_current_working_directory = getcwd(0 as *mut libc::c_char, 0);

            if the_current_working_directory.is_null() {
                let strerror_str = CStr::from_ptr(strerror(errno!()));
                let strerror_string = strerror_str.to_str().unwrap().to_owned();
                let bash_getcwd_errstr = String::from("getcwd: cannot access parent directories");
                if !for_whom.is_null() && *for_whom != 0 {
                    let for_whom_str = CStr::from_ptr(for_whom);
                    let for_whom_string = for_whom_str.to_str().unwrap().to_owned();
                    eprintln!(
                        "{}: error retrieving current directory: {}: {}",
                        for_whom_string, bash_getcwd_errstr, strerror_string
                    );
                } else {
                    let for_whom_str = CStr::from_ptr(get_name_for_error());
                    let for_whom_string = for_whom_str.to_str().unwrap().to_owned();
                    eprintln!(
                        "{}: error retrieving current directory: {}: {}",
                        for_whom_string, bash_getcwd_errstr, strerror_string
                    );
                }
                return std::ptr::null_mut();
            }
        }
        return savestring!(the_current_working_directory);
    }
}

#[no_mangle]
pub fn set_working_directory(name: *mut libc::c_char) {
    unsafe {
        FREE!(the_current_working_directory as *mut c_void);
        the_current_working_directory = savestring!(name);
    }
}

/* **************************************************************** */
/*                                                                  */
/*              Job control support functions                       */
/*                                                                  */
/* **************************************************************** */
#[no_mangle]
pub fn get_job_by_name(name: *const libc::c_char, flags: i32) -> i32 {
    let mut i: i32;
    let wl: i32;
    let mut cl: i32;
    let mut match_0: i32;
    let mut job: i32;
    let mut p: *mut PROCESS;
    let mut j: *mut JOB;

    unsafe {
        job = NO_JOB!();
        wl = strlen(name) as i32;

        i = js.j_jobslots - 1;
        while i >= 0 {
            j = get_job_by_jid!(i);
            if j.is_null() || (flags & JM_STOPPED!() != 0 && J_JOBSTATE!(j) != JSTOPPED) {
                continue;
            }

            p = (*j).pipe;

            loop {
                if (flags & JM_EXACT!()) != 0 {
                    cl = strlen((*p).command) as i32;
                    match_0 = STREQN!((*p).command, name, cl as usize);
                } else if (flags & JM_SUBSTRING!()) != 0 {
                    match_0 = (strcasestr((*p).command, name) != 0 as *mut libc::c_char) as i32;
                } else {
                    match_0 = STREQN!((*p).command, name, wl as usize);
                }

                if match_0 == 0 {
                    p = (*p).next;
                    continue;
                } else if flags & JM_FIRSTMATCH!() != 0 {
                    return i;
                } else if job != NO_JOB!() {
                    if this_shell_builtin.is_some() {
                        let c_str = CString::new("%s: ambiguous job spece").unwrap();
                        let c_str_ptr = c_str.as_ptr();
                        builtin_error(c_str_ptr, name);
                    } else {
                        let c_str = CString::new("%s: ambiguous job spece").unwrap();
                        let c_str_ptr = c_str.as_ptr();
                        internal_error(c_str_ptr, name)
                    }
                    return DUP_JOB!();
                } else {
                    job = i;
                }

                if p == (*j).pipe {
                    break;
                }
            }

            i -= 1;
        }
        return job;
    }
}

/* Return the job spec found in LIST. */
#[no_mangle]
pub fn get_job_spec(list: *mut WordList) -> i32 {
    let mut word: *mut libc::c_char;
    let job: i32;
    let mut jflags: i32;

    unsafe {
        if list.is_null() {
            return js.j_current;
        }

        word = (*(*list).word).word;

        if *word.offset(0) == '\0' as libc::c_char {
            return NO_JOB!();
        }

        if *word.offset(0) == '%' as libc::c_char {
            word = word.offset(1);
        }

        if DIGIT!(*word) && all_digits(word) != 0 {
            job = atoi(word);
            if job < 0 || job > js.j_jobslots {
                return NO_JOB!();
            } else {
                return job - 1;
            }
        }

        jflags = 0;
        let opt = word.offset(0) as u8;
        let opt_char = char::from(opt);
        match opt_char {
            '\0' | '%' | '+' => return js.j_current,
            '-' => return js.j_previous,
            '?' => {
                jflags |= JM_SUBSTRING!();
                word = word.offset(1);
            }
            _ => {}
        }
        return get_job_by_name(word, jflags);
    }
}

/*
 * NOTE:  `kill' calls this function with forcecols == 0
 */
pub fn display_signal_list(mut list: *mut WordList, forcecols: i32) -> i32 {
    let mut _i: i32;
    let mut column: i32;
    let mut name: *mut libc::c_char;
    let mut result: i32;
    let mut signum: i32;
    let mut dflags: i32;
    let mut lsignum: intmax_t = 0;

    unsafe {
        result = EXECUTION_SUCCESS!();
        if list.is_null() {
            column = 0;
            for i in 1..65 {
                name = signal_name(i);
                if STREQN!(
                    name,
                    String::from("SIGJUNK").as_ptr() as *mut libc::c_char,
                    7
                ) != 0
                    || STREQN!(
                        name,
                        String::from("Unknown").as_ptr() as *mut libc::c_char,
                        7
                    ) != 0
                {
                    continue;
                }

                if posixly_correct != 0 && forcecols == 0 {
                    /* This is for the kill builtin.  POSIX.2 says the signal names
                    are displayed without the `SIG' prefix. */
                    if STREQN!(name, String::from("SIG").as_ptr() as *mut libc::c_char, 3) != 0 {
                        name = name.offset(3);
                    }
                    if i == NSIG!() - 1 {
                        print!("{}", CStr::from_ptr(name).to_str().unwrap().to_owned());
                    } else {
                        print!("{} ", CStr::from_ptr(name).to_str().unwrap().to_owned());
                    }
                } else {
                    print!(
                        "{:>2}{} {}",
                        i,
                        ")",
                        CStr::from_ptr(name).to_str().unwrap().to_owned()
                    );

                    column += 1;
                    if column < 5 {
                        print! {"\t"};
                    } else {
                        print!("\n");
                        column = 0;
                    }
                }
            }

            if posixly_correct != 0 && forcecols != 0 || column != 0 {
                print!("\n");
            }
            return result;
        } //if list.is_null()

        /* List individual signal names or numbers. */
        while !list.is_null() {
            if legal_number((*(*list).word).word, &mut lsignum) != 0 {
                /* This is specified by Posix.2 so that exit statuses can be
                mapped into signal numbers. */
                if lsignum > 128 {
                    lsignum -= 128;
                }
                if lsignum < 0 || lsignum >= NSIG!() {
                    sh_invalidsig((*(*list).word).word);
                    result = EXECUTION_FAILURE!();
                    list = (*list).next;
                    continue;
                }

                signum = lsignum as i32;
                name = signal_name(signum);
                if STREQN!(
                    name,
                    String::from("SIGJUNK").as_ptr() as *mut libc::c_char,
                    7
                ) != 0
                    || STREQN!(
                        name,
                        String::from("Unknow").as_ptr() as *mut libc::c_char,
                        7
                    ) != 0
                {
                    list = (*list).next;
                    continue;
                }
                /* POSIX.2 says that `kill -l signum' prints the signal name without
                the `SIG' prefix. */
                if this_shell_builtin == Some(kill_builtin) && signum > 0 {
                    // name = name.offset(3);
                    println!(
                        "{}",
                        CStr::from_ptr(name.offset(3)).to_str().unwrap().to_owned()
                    );
                } else {
                    println!("{}", CStr::from_ptr(name).to_str().unwrap().to_owned());
                }
            } else {
                dflags = DSIG_NOCASE!();
                if posixly_correct == 0 || this_shell_builtin != Some(kill_builtin) {
                    dflags |= DSIG_SIGPREFIX!();
                }
                signum = decode_signal((*(*list).word).word, dflags);
                if signum == NO_SIG!() {
                    sh_invalidsig((*(*list).word).word);
                    result = EXECUTION_FAILURE!();
                    list = (*list).next;
                    continue;
                }
                println!("{}", signum);
            }
            list = (*list).next;
        } //while
        return result;
    }
}

/* **************************************************************** */
/*                                                                  */
/*          Finding builtin commands and their functions            */
/*                                                                  */
/* **************************************************************** */

/* Perform a binary search and return the address of the builtin function
whose name is NAME.  If the function couldn't be found, or the builtin
is disabled or has no function associated with it, return NULL.
Return the address of the builtin.
DISABLED_OKAY means find it even if the builtin is disabled. */

fn print_builtin_name() {
    let hi: i32;
    let lo: i32;
    let _mid: i32 = 0;
    let mut _j: i32;

    unsafe {
        hi = num_shell_builtins - 1;
        lo = 0;

        while lo <= hi {
            //printf(b" builtin command name is :%s\n", (*shell_builtins.offset(mid as isize)).name);
        }
    }
}

#[no_mangle]
pub fn builtin_address_internal(name: *mut libc::c_char, disabled_okay: i32) -> *mut builtin {
    let mut hi: i32;
    let mut lo: i32;
    let mut mid: i32 = 0;
    let mut j: i32;

    unsafe {
        hi = num_shell_builtins - 1;
        lo = 0;

        while lo <= hi {
            mid = (lo + hi) / 2;

            j = *((*shell_builtins.offset(mid as isize)).name).offset(0 as libc::c_int as isize)
                as libc::c_int
                - *name.offset(0 as libc::c_int as isize) as libc::c_int;

            if j == 0 {
                j = strcmp((*shell_builtins.offset(mid as isize)).name, name);
            }

            if j == 0 {
                /* It must have a function pointer.  It must be enabled, or we
                must have explicitly allowed disabled functions to be found,
                and it must not have been deleted. */
                if ((*shell_builtins.offset(mid as isize)).function).is_some()
                    && (*shell_builtins.offset(mid as isize)).flags & BUILTIN_DELETED!() == 0
                    && (*shell_builtins.offset(mid as isize)).flags & BUILTIN_ENABLED!() != 0 //应该是 !=0
                    || disabled_okay != 0
                {
                    return &mut *shell_builtins.offset(mid as isize);
                } else {
                    return 0 as *mut builtin;
                }
            }

            if j > 0 {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }

        return 0 as *mut builtin;
    }
}

/* Return the pointer to the function implementing builtin command NAME. */
pub fn find_shell_builtin(name: *mut libc::c_char) -> Option<sh_builtin_func_t> {
    unsafe {
        current_builtin = builtin_address_internal(name, 0);
        if !current_builtin.is_null() {
            return (*current_builtin).function;
        } else {
            return None;
        }
    }
}

/* Return the address of builtin with NAME, whether it is enabled or not. */
pub fn builtin_address(name: *mut libc::c_char) -> Option<sh_builtin_func_t> {
    unsafe {
        current_builtin = builtin_address_internal(name, 1);
        if !current_builtin.is_null() {
            return (*current_builtin).function;
        } else {
            return None;
        }
    }
}

/* Return the function implementing the builtin NAME, but only if it is a
POSIX.2 special builtin. */
#[no_mangle]
pub fn find_special_builtin(name: *mut libc::c_char) -> Option<sh_builtin_func_t> {
    unsafe {
        current_builtin = builtin_address_internal(name, 0);

        if !current_builtin.is_null() && (*current_builtin).flags & SPECIAL_BUILTIN!() != 0 {
            return (*current_builtin).function;
        } else {
            return None;
        }
    }
}

#[no_mangle]
fn shell_builtin_compare(sbp1: *mut builtin, sbp2: *mut builtin) -> i32 {
    unsafe {
        let mut result: libc::c_int = 0;
        result = *((*sbp1).name).offset(0 as libc::c_int as isize) as libc::c_int
            - *((*sbp2).name).offset(0 as libc::c_int as isize) as libc::c_int;
        if result == 0 as libc::c_int {
            result = strcmp((*sbp1).name, (*sbp2).name);
        }
        return result;
    }
}

/* Sort the table of shell builtins so that the binary search will work
in find_shell_builtin. */
#[no_mangle]
pub fn initialize_shell_builtins() {
    unsafe {
        qsort(
            shell_builtins as *mut c_void,
            num_shell_builtins as usize,
            size_of::<builtin>(),
            std::mem::transmute::<Option<fn() -> libc::c_int>, Option<QSFUNC>>(Some(
                std::mem::transmute::<
                    fn(*mut builtin, *mut builtin) -> libc::c_int,
                    fn() -> libc::c_int,
                >(shell_builtin_compare),
            )),
        );
    }
}

/* **************************************************************** */
/*                                                                  */
/*          Variable assignments during builtin commands            */
/*                                                                  */
/* **************************************************************** */
#[no_mangle]
pub fn builtin_bind_variable(
    name: *mut libc::c_char,
    value: *mut libc::c_char,
    flags: i32,
) -> *mut SHELL_VAR {
    let v: *mut SHELL_VAR;

    unsafe {
        let opt: i32;
        if assoc_expand_once != 0 {
            opt = VA_NOEXPAND!() | VA_ONEWORD!();
        } else {
            opt = 0;
        }

        if valid_array_reference(name, opt) == 0 {
            v = bind_variable(name, value, flags);
        } else {
            v = assign_array_element(
                name,
                value,
                flags
                    | (if assoc_expand_once != 0 {
                        ASS_NOEXPAND!()
                    } else {
                        0
                    }),
            );
        }

        if !v.is_null() && readonly_p!(v) == 0 && noassign_p!(v) == 0 {
            VUNSETATTR!(v, att_invisible!());
        }
        return v;
    }
}

/* Like check_unbind_variable, but for use by builtins (only matters for
error messages). */
pub fn builtin_unbind_variable(vname: *const libc::c_char) -> i32 {
    let v: *mut SHELL_VAR;

    unsafe {
        v = find_variable(vname);
        if !v.is_null() && readonly_p!(v) != 0 {
            let c_str = CString::new("%s: cannot unset: readonly %s").unwrap();
            let c_str_ptr = c_str.as_ptr();
            builtin_error(c_str_ptr, vname, "variable");
            return -2;
        } else if !v.is_null() && non_unsettable_p!(v) != 0 {
            let c_str = CString::new("%s: cannot unset").unwrap();
            let c_str_ptr = c_str.as_ptr();
            builtin_error(c_str_ptr, vname);
            return -2;
        }

        return unbind_variable(vname);
    }
}

pub fn get_local_str() -> Vec<LanguageIdentifier> {
    let language: String;
    match var("LANG") {
        Ok(v) => language = v,
        Err(e) => {
            language = String::from("en-US");
            println!("err is {e:?}")
        }
    }
    // println!("now language is {:?}",language);
    //parse() 用于类型转换
    let v: Vec<_> = language.split('.').collect();
    let langid: LanguageIdentifier = v[0].parse().expect("wrong language");
    let locales = vec![langid.into()];
    return locales;
}

pub fn savestring(x: *const libc::c_char) -> *mut libc::c_char {
    unsafe {
        let len = 1 + libc::strlen(x);
        let str1: *mut libc::c_char = libc::malloc(len) as *mut libc::c_char;
        libc::memset(str1 as *mut libc::c_void, 0, len);
        return libc::strcpy(str1 as *mut libc::c_char, x);
    }
}

pub fn err_translate_fn(command: &String, args1: *mut libc::c_char) {
    let mgr = ResourceManager::new("/usr/share/utshell/resources/{locale}/{res_id}".into());
    let resources = vec!["message.ftl".into()];
    let mut args = FluentArgs::new();
    unsafe {
        if args1 != std::ptr::null_mut() {
            args.set(
                "str1",
                format!("{:?}", CStr::from_ptr(args1).to_str().unwrap()),
            );
        }
    }
    let bundle = mgr.get_bundle(get_local_str(), resources);
    let value = bundle.get_message(command).unwrap();
    let pattern = value.value().expect("partern err");
    let mut errors = vec![];
    if args1 != std::ptr::null_mut() {
        let msg1 = bundle.format_pattern(&pattern, Some(&args), &mut errors);
        eprint!("{msg1}");
    } else {
        let msg1 = bundle.format_pattern(&pattern, None, &mut errors);
        eprint!("{msg1}");
    }
}

pub fn translate_fn(command: &String, args1: *mut libc::c_char) {
    let mgr = ResourceManager::new("/usr/share/utshell/resources/{locale}/{res_id}".into());
    let resources = vec!["message.ftl".into()];
    let mut args = FluentArgs::new();
    unsafe {
        if args1 != std::ptr::null_mut() {
            args.set(
                "str1",
                format!("{:?}", CStr::from_ptr(args1).to_str().unwrap()),
            );
        }
    }
    let bundle = mgr.get_bundle(get_local_str(), resources);
    let value = bundle.get_message(command).unwrap();
    let pattern = value.value().expect("partern err");
    let mut errors = vec![];
    if args1 != std::ptr::null_mut() {
        let msg1 = bundle.format_pattern(&pattern, Some(&args), &mut errors);
        print!("{msg1}");
    } else {
        let msg1 = bundle.format_pattern(&pattern, None, &mut errors);
        print!("{msg1}");
    }
}
