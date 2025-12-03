//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

use crate::bashhist::{bash_delete_last_history, maybe_add_history};
use crate::builtins::evalfile::fc_execute_file;
use crate::builtins::{
    bashgetopt::{internal_getopt, reset_internal_getopt},
    common::{builtin_usage, sh_chkwrite, sh_erange, sh_wrerror},
    help::builtin_help,
};
use crate::general::legal_number;
use crate::list::list_reverse;
use crate::readline::xfree;
use crate::sig::{termsig_handler, throw_to_top_level};
use crate::src_common::*;
use crate::stringlib::strsub;
use crate::unwind_prot::{add_unwind_protect, unwind_protect_mem};

#[no_mangle]
pub fn set_verbose_flag() {
    unsafe {
        echo_input_at_read = verbose_flag;
    }
}

#[no_mangle]
pub fn fc_number(list: *mut WordList) -> i32 {
    let mut s: *mut libc::c_char;
    unsafe {
        if list.is_null() {
            return 0;
        }
        s = (*(*list).word).word;
        if *s as libc::c_int == '-' as i32 {
            s = s.offset(1);
        }
        let res = legal_number(s, std::ptr::null_mut());
        return res;
    }
}

fn REVERSE_LIST(list: *mut GENERIC_LIST) -> *mut REPL {
    unsafe {
        if list != std::ptr::null_mut() && (*list).next != std::ptr::null_mut() {
            list_reverse(list) as *mut REPL
        } else {
            list as *mut REPL
        }
    }
}

fn printToStderr(str: *mut libc::c_char) -> std::io::Result<()> {
    let stderr_1 = std::io::stderr();
    let mut handle = stderr_1.lock();
    unsafe {
        handle.write_all(std::ffi::CStr::from_ptr(str).to_bytes())?;
        Ok(())
    }
}

fn printToStdout(str: *mut libc::c_char) -> std::io::Result<()> {
    let stdouts = std::io::stdout();
    let mut handle = stdouts.lock();
    unsafe {
        handle.write_all(std::ffi::CStr::from_ptr(str).to_bytes())?;
        Ok(())
    }
}

fn printToStdoutflush() -> std::io::Result<()> {
    let stdouts = std::io::stdout();
    let mut handle = stdouts.lock();
    handle.flush()?;
    Ok(())
}

fn QUIT() {
    unsafe {
        if terminating_signal != 0 {
            termsig_handler(terminating_signal);
        }

        if interrupt_state != 0 {
            throw_to_top_level();
        }
    }
}

fn STREQN(a: *const libc::c_char, b: *const libc::c_char, n: i32) -> bool {
    if n == 0 {
        return true;
    } else {
        return unsafe { *a == *b && libc::strncmp(a, b, n as libc::size_t) == 0 };
    }
}

#[no_mangle]
pub fn fc_builtin(mut list: *mut WordList) -> i32 {
    let mut i: i32;
    let mut sep: *mut libc::c_char;
    let mut numbering: i32;
    let mut reverse: i32;
    let mut listing: i32;
    let mut execute: i32;
    let mut histbeg: i32;
    let mut histend: i32;
    let mut last_hist: i32;
    let mut retval: i32;
    let mut opt: i32;
    let rh: i32;
    let mut real_last: i32;
    let stream: *mut libc::FILE;
    let mut rlist: *mut REPL;
    let mut rl: *mut REPL;
    let mut ename: *mut libc::c_char;
    let mut command: *mut libc::c_char;
    let newcom: *mut libc::c_char;
    let fcedit: std::ffi::CString;
    let hlist: *mut *mut HIST_ENTRY;
    let mut fnc: *mut libc::c_char = std::ptr::null_mut();

    numbering = 1;
    reverse = 0;
    listing = 0;
    execute = 0;
    ename = std::ptr::null_mut();
    unsafe {
        reset_internal_getopt();
        lcurrent = list;

        loptend = lcurrent;
        while fc_number(loptend) == 0 {
            opt = internal_getopt(
                list,
                CString::new(":e:lnrs").unwrap().as_ptr() as *mut libc::c_char,
            );
            if opt != -1 {
                let optu8: u8 = opt as u8;
                let optChar: char = char::from(optu8);
                match optChar {
                    'n' => numbering = 0,
                    'l' => listing = HN_LISTING!(),
                    'r' => reverse = 1,
                    's' => execute = 1,
                    'e' => ename = list_optarg,
                    _ => {
                        if opt == -99 {
                            builtin_help();
                            return EX_USAGE!();
                        }
                        builtin_usage();
                        return EX_USAGE!();
                    }
                }
            } else {
                break;
            }
            loptend = lcurrent;
        }

        list = loptend;

        if ename != std::ptr::null_mut()
            && char::from(*ename as u8) == '-'
            && char::from(*((ename as usize + 4) as *mut libc::c_char) as u8) == '\0'
        {
            execute = 1;
        }

        if execute != 0 {
            rlist = std::ptr::null_mut();

            let mut ret: bool = loptend != std::ptr::null_mut();
            sep = libc::strchr((*(*list).word).word, char::from('=') as libc::c_int);
            ret = ret && sep != std::ptr::null_mut();
            while ret {
                sep = (sep as usize + 4) as *mut libc::c_char;
                *sep = char::from('\0') as libc::c_char;
                rl = libc::malloc(std::mem::size_of::<&REPL>()) as *mut REPL;
                (*rl).next = std::ptr::null_mut();
                (*rl).pat = savestring!((*(*list).word).word);
                (*rl).rep = savestring!(sep);

                if rlist == std::ptr::null_mut() {
                    rlist = rl;
                } else {
                    (*rl).next = rlist;
                    rlist = rl;
                }
                list = (*list).next;
            }

            rlist = REVERSE_LIST(rlist as *mut GENERIC_LIST);
            hlist = history_list();
            if list != std::ptr::null_mut() {
                command = fc_gethist((*(*list).word).word, hlist, 0);
            } else {
                command = fc_gethist(std::ptr::null_mut(), hlist, 0);
            }

            if command == std::ptr::null_mut() {
                builtin_error(CString::new("no command found").unwrap().as_ptr());
                if rlist != std::ptr::null_mut() {
                    rl = rlist;
                    while rl != std::ptr::null_mut() {
                        let r: *mut REPL;
                        r = (*rl).next;
                        if (*rl).pat != std::ptr::null_mut() {
                            libc::free((*rl).pat as *mut c_void);
                        }

                        if (*rl).rep != std::ptr::null_mut() {
                            libc::free((*rl).rep as *mut c_void);
                        }

                        libc::free(rl as *mut c_void);
                        rl = r;
                    }
                }
                return EXECUTION_FAILURE!();
            }

            if rlist != std::ptr::null_mut() {
                newcom = fc_dosubs(command, rlist);
                libc::free(command as *mut c_void);
                rl = rlist;
                while rl != std::ptr::null_mut() {
                    let r: *mut REPL;
                    r = (*rl).next;
                    if (*rl).pat != std::ptr::null_mut() {
                        libc::free((*rl).pat as *mut c_void);
                    }

                    if (*rl).rep != std::ptr::null_mut() {
                        libc::free((*rl).rep as *mut c_void);
                    }

                    libc::free(rl as *mut c_void);
                    rl = r;
                }
                command = newcom;
            }
            printToStderr(command);
            fc_replhist(command);
            return parse_and_execute(
                command,
                CString::new("fc").unwrap().as_ptr(),
                SEVAL_NOHIST!(),
            );
        }

        hlist = history_list();

        if hlist == std::ptr::null_mut() {
            return EXECUTION_SUCCESS!();
        }
        i = 0;

        while !(*hlist.offset(i as isize)).is_null() {
            i += 1;
        }

        rh = (remember_on_history != 0
            || ((subshell_environment & SUBSHELL_COMSUB!()) != 0 && enable_history_list != 0))
            as i32;
        last_hist = i - rh - hist_last_line_added;

        real_last = i;
        while (*hlist.offset(real_last as isize)).is_null() && real_last > 0 {
            real_last -= 1;
        }

        if i == last_hist && (*hlist.offset(last_hist as isize)).is_null() {
            while last_hist >= 0 && (*hlist.offset(last_hist as isize)).is_null() {
                last_hist -= 1;
            }
        }

        if last_hist < 0 {
            last_hist = 0;
        }

        if !(list.is_null()) {
            histbeg = fc_gethnum((*(*list).word).word, hlist, listing | HN_FIRST!());
            list = (*list).next;

            if list != std::ptr::null_mut() {
                histend = fc_gethnum((*(*list).word).word, hlist, listing);
            } else if histbeg == real_last {
                histend = if listing != 0 { real_last } else { histbeg };
            } else {
                histend = if listing != 0 { last_hist } else { histbeg }
            }
        } else {
            if listing != 0 {
                histend = last_hist;
                histbeg = histend - 16 + 1;
                if histbeg < 0 {
                    histbeg = 0;
                }
            } else {
                histbeg = last_hist;
                histend = last_hist;
            }
        }

        if histbeg == HIST_INVALID!() || histend == HIST_INVALID!() {
            sh_erange(
                std::ptr::null_mut(),
                CString::new("history specification").unwrap().as_ptr() as *mut libc::c_char,
            );
            return EXECUTION_FAILURE!();
        } else if histbeg == HIST_ERANGE!() || histend == HIST_ERANGE!() {
            sh_erange(
                std::ptr::null_mut(),
                CString::new("history specification").unwrap().as_ptr() as *mut libc::c_char,
            );
            return EXECUTION_FAILURE!();
        } else if histbeg == HIST_NOTFOUND!() || histend == HIST_NOTFOUND!() {
            builtin_error(CString::new("no command found").unwrap().as_ptr() as *mut libc::c_char);
            return EXECUTION_FAILURE!();
        }

        if histbeg < 0 {
            histbeg = 0;
        }

        if histend < 0 {
            histend = 0;
        }

        if listing == 0 && hist_last_line_added != 0 {
            bash_delete_last_history();

            if histbeg == histend
                && histend == last_hist
                && *((hlist as usize + (8 * last_hist) as usize) as *mut *mut HIST_ENTRY)
                    == std::ptr::null_mut()
            {
                histend -= 1;
                last_hist = histend;
                histbeg = histend;
            }

            if *((hlist as usize + (8 * last_hist) as usize) as *mut *mut HIST_ENTRY)
                == std::ptr::null_mut()
            {
                last_hist -= 1;
            }

            if histend >= last_hist {
                histend = last_hist;
            } else if histbeg >= last_hist {
                histbeg = last_hist;
            }
        }

        if histbeg == HIST_INVALID!() || histend == HIST_INVALID!() {
            sh_erange(
                std::ptr::null_mut(),
                CString::new("history specification").unwrap().as_ptr() as *mut libc::c_char,
            );
            return EXECUTION_FAILURE!();
        } else if histbeg == HIST_ERANGE!() || histend == HIST_ERANGE!() {
            sh_erange(
                std::ptr::null_mut(),
                CString::new("history specification").unwrap().as_ptr() as *mut libc::c_char,
            );
            return EXECUTION_FAILURE!();
        } else if histbeg == HIST_NOTFOUND!() || histend == HIST_NOTFOUND!() {
            builtin_error(CString::new("no command found").unwrap().as_ptr());
            return EXECUTION_FAILURE!();
        }

        if histbeg < 0 {
            histbeg = 0;
        }

        if histend < 0 {
            histend = 0;
        }

        if histend < histbeg {
            i = histend;
            histend = histbeg;
            histbeg = i;
            reverse = 1;
        }

        if listing != 0 {
            stream = std::ptr::null_mut();
        } else {
            numbering = 0;
            stream = sh_mktmpfp(
                CString::new("bash-fc").unwrap().as_ptr() as *mut libc::c_char,
                MT_USERANDOM!() | MT_USETMPDIR!(),
                &mut fnc,
            );

            if stream == std::ptr::null_mut() {
                if fnc != std::ptr::null_mut() {
                    builtin_error(
                        CString::new("%s: cannot open temp file: %s")
                            .unwrap()
                            .as_ptr(),
                        fnc,
                        strerror(errno!()),
                    );
                } else {
                    builtin_error(
                        CString::new("%s: cannot open temp file: %s")
                            .unwrap()
                            .as_ptr(),
                        CString::new("").unwrap().as_ptr(),
                        strerror(errno!()),
                    );
                }

                libc::free(fnc as *mut c_void);
                return EXECUTION_FAILURE!();
            }
        }

        if reverse != 0 {
            i = histend;
        } else {
            i = histbeg;
        }

        let mut ret: bool = reverse != 0;

        if ret {
            ret = i >= histbeg;
        } else {
            ret = i <= histend;
        }

        while ret {
            QUIT();
            if numbering != 0 {
                if stream != std::ptr::null_mut() {
                    libc::fprintf(
                        stream,
                        CString::new("%d").unwrap().as_ptr(),
                        i + history_base,
                    );
                } else {
                    let diff = i + history_base;
                    printToStdout(
                        CString::new(diff.to_string()).unwrap().as_ptr() as *mut libc::c_char
                    );
                }
            }

            if listing != 0 {
                if posixly_correct != 0 {
                    if stream != std::ptr::null_mut() {
                        libc::fputs(CString::new("\t").unwrap().as_ptr(), stream);
                    } else {
                        printToStdout(CString::new("\t").unwrap().as_ptr() as *mut libc::c_char);
                    }
                } else {
                    let mut ch: char;
                    if (**((hlist as usize + (8 * i) as usize) as *mut *mut HIST_ENTRY)).data
                        != std::ptr::null_mut()
                    {
                        ch = '*';
                    } else {
                        ch = ' ';
                    }

                    if stream != std::ptr::null_mut() {
                        libc::fprintf(stream, CString::new("\t%c").unwrap().as_ptr(), &mut ch);
                    } else {
                        let mut th = vec!['\t' as libc::c_char];
                        th.push(ch as libc::c_char);
                        th.push(0);
                        printToStdout(th.as_ptr() as *mut libc::c_char);
                    }
                }
            }

            if stream != std::ptr::null_mut() {
                libc::fprintf(
                    stream,
                    CString::new("%s\n").unwrap().as_ptr(),
                    (**((hlist as usize + (i * 8) as usize) as *mut *mut HIST_ENTRY)).line,
                );
            } else {
                printToStdout(
                    (**((hlist as usize + (i * 8) as usize) as *mut *mut HIST_ENTRY)).line,
                );
                printToStdout(CString::new("\n").unwrap().as_ptr() as *mut libc::c_char);
            }

            ret = reverse != 0;
            if ret {
                i -= 1;
            } else {
                i += 1;
            }

            if ret {
                ret = i >= histbeg;
            } else {
                ret = i <= histend;
            }
        }

        if listing != 0 {
            return sh_chkwrite(EXECUTION_SUCCESS!());
        }

        if stream != std::ptr::null_mut() {
            libc::fflush(stream);
            if libc::ferror(stream) != 0 {
                sh_wrerror();
                libc::fclose(stream);
                libc::free(fnc as *mut c_void);
                return EXECUTION_FAILURE!();
            }
            libc::fclose(stream);
        } else {
            if printToStdoutflush().is_err() {
                sh_wrerror();
                libc::free(fnc as *mut c_void);
                return EXECUTION_FAILURE!();
            }
        }

        /* Now edit the file of commands. */
        if ename != std::ptr::null_mut() {
            command =
                libc::malloc(libc::strlen(ename) + libc::strlen(fnc) + 2) as *mut libc::c_char;
            libc::sprintf(command, CString::new("").unwrap().as_ptr());
            libc::strcpy(command, ename);
            libc::strcat(command, CString::new(" ").unwrap().as_ptr());
            libc::strcat(command, fnc);
        } else {
            if posixly_correct != 0 {
                fcedit = CString::new("${FCEDIT:-${EDITOR:-ed}}").unwrap();
            } else {
                fcedit = CString::new("${FCEDIT:-${EDITOR:-vi}}").unwrap();
            }

            command =
                libc::malloc(3 + libc::strlen(fcedit.as_ptr()) as libc::size_t + libc::strlen(fnc))
                    as *mut libc::c_char;
            libc::sprintf(command, CString::new("").unwrap().as_ptr());
            libc::strcpy(command, fcedit.as_ptr());
            libc::strcat(command, CString::new(" ").unwrap().as_ptr());
            libc::strcat(command, fnc);
        }

        retval = parse_and_execute(
            command,
            CString::new("fc").unwrap().as_ptr(),
            SEVAL_NOHIST!(),
        );
        if retval != EXECUTION_SUCCESS!() {
            unlink(fnc);
            libc::free(fnc as *mut c_void);
            return EXECUTION_FAILURE!();
        }

        /* Make sure parse_and_execute doesn't turn this off, even though a
        call to parse_and_execute farther up the function call stack (e.g.,
        if this is called by vi_edit_and_execute_command) may have already
        called bash_history_disable. */
        remember_on_history = 1;

        /* Turn on the `v' flag while fc_execute_file runs so the commands
        will be echoed as they are read by the parser. */
        //begin_unwind_frame (CString::new ("fc builtin").unwrap().as_ptr() as * mut libc::c_char);
        // let xf: Functions = Functions { f_xfree: xfree };
        // let uk: Functions = Functions { f_unlink: unlink };
        // let r_flag: Functions = Functions {
        //     f_set_verbose: set_verbose_flag,
        // };
        // add_unwind_protect(xf, fnc);
        // add_unwind_protect(uk, fnc);
        add_unwind_protect(
            std::mem::transmute::<
                unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void),
                Option<Function>,
            >(xfree),
            fnc,
        );
        add_unwind_protect(
            std::mem::transmute::<unsafe extern "C" fn(*const libc::c_char) -> libc::c_int, Option<Function>>(unlink),
            fnc,
        );
        add_unwind_protect(
            std::mem::transmute::<fn(), Option<Function>>(set_verbose_flag),
            std::ptr::null_mut(),
        );
        unwind_protect_mem(
            (&mut (suppress_debug_trap_verbose as libc::c_char)) as *mut libc::c_char,
            4,
        );
        echo_input_at_read = 1;
        suppress_debug_trap_verbose = 1;

        retval = fc_execute_file(fnc);
        //run_unwind_frame (CString::new ("fc builtin").unwrap().as_ptr() as * mut libc::c_char);

        return retval;
    }
}

#[no_mangle]
pub fn fc_gethist(
    command: *mut libc::c_char,
    hlist: *mut *mut HIST_ENTRY,
    mode: i32,
) -> *mut libc::c_char {
    let i: i32;

    if hlist == std::ptr::null_mut() {
        return std::ptr::null_mut();
    }

    i = fc_gethnum(command, hlist, mode);
    unsafe {
        if i >= 0 {
            return savestring!(
                (*(*((hlist as usize + (8 * i) as usize) as *mut *mut HIST_ENTRY))).line
            );
        } else {
            return std::ptr::null_mut();
        }
    }
}

#[no_mangle]
pub fn fc_gethnum(command: *mut libc::c_char, hlist: *mut *mut HIST_ENTRY, mode: i32) -> i32 {
    let mut sign: i32;
    let mut n: i32;
    let clen: i32;
    let rh: i32;
    let mut i: i32 = 0;
    let mut j: i32;
    let mut last_hist: i32;
    let mut real_last: i32;
    let listing: i32;

    let mut s: *mut libc::c_char;

    unsafe {
        listing = mode & HN_LISTING!();
        sign = 1;
        /* Count history elements. */
        while !(*hlist.offset(i as isize)).is_null() {
            i += 1;
        }
        /* With the Bash implementation of history, the current command line
        ("fc blah..." and so on) is already part of the history list by
        the time we get to this point.  This just skips over that command
        and makes the last command that this deals with be the last command
        the user entered before the fc.  We need to check whether the
        line was actually added (HISTIGNORE may have caused it to not be),
        so we check hist_last_line_added.  This needs to agree with the
        calculation of last_hist in fc_builtin above. */
        /* Even though command substitution through parse_and_execute turns off
        remember_on_history, command substitution in a shell when set -o history
        has been enabled (interactive or not) should use it in the last_hist
        calculation as if it were on. */
        rh = (remember_on_history != 0
            || ((subshell_environment & SUBSHELL_COMSUB!()) != 0 && enable_history_list != 0))
            as i32;
        last_hist = i - rh - hist_last_line_added;

        if i == last_hist && (*hlist.offset(last_hist as isize)).is_null() {
            while last_hist >= 0 && (*hlist.offset(last_hist as isize)).is_null() {
                last_hist -= 1;
            }
        }

        if last_hist < 0 {
            return -1;
        }

        real_last = i;
        i = last_hist;

        /* No specification defaults to most recent command. */
        if command == std::ptr::null_mut() {
            return i;
        }

        /* back up from the end to the last non-null history entry */
        while (*hlist.offset(real_last as isize)).is_null() && real_last > 0 {
            real_last -= 1;
        }

        /* Otherwise, there is a specification.  It can be a number relative to
        the current position, or an absolute history number. */
        s = command;

        /* Handle possible leading minus sign. */
        if s != std::ptr::null_mut() && (char::from(*s as u8) == '-') {
            sign = -1;
            s = s.offset(1)
        }

        if s != std::ptr::null_mut() && DIGIT!(*s) {
            n = libc::atoi(s);
            n *= sign;

            /* We want to return something that is an offset to HISTORY_BASE. */
            /* If the value is negative or zero, then it is an offset from
            the current history item. */
            /* We don't use HN_FIRST here, so we don't return different values
            depending on whether we're looking for the first or last in a
            pair of range arguments, but nobody else does, either. */
            if n < 0 {
                n += i + 1;
                if n < 0 {
                    return 0;
                } else {
                    return n;
                }
            } else if n == 0 {
                if sign == -1 {
                    if listing != 0 {
                        return real_last;
                    } else {
                        return HIST_INVALID!();
                    }
                } else {
                    return i;
                }
            } else {
                /* If we're out of range (greater than I (last history entry) or
                less than HISTORY_BASE, we want to return different values
                based on whether or not we are looking for the first or last
                value in a desired range of history entries. */
                n -= history_base;
                if n < 0 {
                    if mode & HN_FIRST!() != 0 {
                        return 0;
                    } else {
                        return i;
                    }
                } else if n >= i {
                    if mode & HN_FIRST!() != 0 {
                        return 0;
                    } else {
                        return i;
                    }
                } else {
                    return n;
                }
            }
        }

        clen = libc::strlen(command as *const libc::c_char) as i32;
        j = i;
        while j >= 0 {
            if STREQN(command, (*(*hlist.offset(j as isize))).line, clen) {
                //if STREQN (command, (*(*((hlist as usize + (8*j) as usize ) as  * mut * mut HIST_ENTRY))).line, clen) {
                return j;
            }
            j -= 1;
        }
        return HIST_NOTFOUND!();
    }
}

#[no_mangle]
pub fn fc_dosubs(command: *mut libc::c_char, subs: *mut REPL) -> *mut libc::c_char {
    let mut new: *mut libc::c_char;
    let mut t: *mut libc::c_char;
    let mut r: *mut REPL;
    unsafe {
        new = savestring!(command);
        while subs != std::ptr::null_mut() {
            r = subs;
            t = strsub(new, (*r).pat, (*r).rep, 1);
            r = (*r).next;
            libc::free(new as *mut c_void);
            new = t;
        }
        return new;
    }
}

#[no_mangle]
pub fn fc_replhist(command: *mut libc::c_char) {
    let n: i32;
    unsafe {
        if command == std::ptr::null_mut() || char::from(*command as u8) == '\0' {
            return;
        }

        n = libc::strlen(command as *const libc::c_char) as i32;
        if char::from(*((command as usize + 4 * (n - 1) as usize) as *mut libc::c_char) as u8)
            == '\n'
        {
            *((command as usize + 4 * (n - 1) as usize) as *mut libc::c_char) = 0 as libc::c_char;
        }

        if command != std::ptr::null_mut() && (*command) != 0 {
            bash_delete_last_history();
            maybe_add_history(command);
        }
    }
}

#[no_mangle]
pub fn fc_addhist(line: *mut libc::c_char) {
    let n: i32;
    unsafe {
        if line == std::ptr::null_mut() || *line == 0 {
            return;
        }

        n = libc::strlen(line) as i32;

        if *((line as usize + (n - 1) as usize) as *mut libc::c_char) == '\n' as libc::c_char {
            *((line as usize + (n - 1) as usize) as *mut libc::c_char) = '\0' as libc::c_char;
        }

        if line != std::ptr::null_mut() && *line != 0 {
            maybe_add_history(line);
        }
    }
}

#[no_mangle]
pub fn fc_readline(stream: *mut libc::FILE) -> *mut libc::c_char {
    let mut c: i32;
    let mut line_len: i32 = 0;
    let mut lindex: i32 = 0;
    let mut line: *mut libc::c_char = std::ptr::null_mut();
    unsafe {
        c = libc::fgetc(stream);
        while c != libc::EOF {
            if (lindex + 2) >= line_len {
                line_len += 128;
                line = libc::malloc(line_len as libc::size_t) as *mut libc::c_char;
            }

            if c == '\n' as i32 {
                *((line as usize + (4 * lindex) as usize) as *mut libc::c_char) =
                    '\n' as libc::c_char;
                lindex += 1;
                *((line as usize + (4 * lindex) as usize) as *mut libc::c_char) =
                    '\0' as libc::c_char;
                lindex += 1;
                return line;
            } else {
                *((line as usize + (4 * lindex) as usize) as *mut libc::c_char) = c as libc::c_char;
                lindex += 1;
            }

            c = libc::fgetc(stream);
        }

        if lindex == 0 {
            if line != std::ptr::null_mut() {
                libc::free(line as *mut c_void);
            }
            return std::ptr::null_mut();
        }

        if lindex + 2 >= line_len {
            line = libc::malloc((lindex + 3) as libc::size_t) as *mut libc::c_char;
        }

        *((line as usize + (4 * lindex) as usize) as *mut libc::c_char) = '\n' as libc::c_char;
        lindex += 1;
        *((line as usize + (4 * lindex) as usize) as *mut libc::c_char) = '\0' as libc::c_char;
        lindex += 1;

        return line;
    }
}
