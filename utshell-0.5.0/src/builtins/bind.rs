/*
 * SPDX-FileCopyrightText: 2025 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: GPL-2.0-or-later
 */
//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

use super::bashgetopt::{internal_getopt, reset_internal_getopt};
use super::common::{builtin_usage, err_translate_fn, sh_chkwrite, translate_fn};
use super::help::builtin_help;
use crate::bashline::initialize_readline;
use crate::bashline::{
    bash_execute_unix_command, bind_keyseq_to_unix_command, print_unix_command_map,
    unbind_unix_command,
};
use crate::general::printable_filename;
use crate::src_common::*;
use crate::unwind_prot::{begin_unwind_frame, run_unwind_frame, unwind_protect_mem};

#[no_mangle]
pub fn bind_builtin(mut list: *mut WordList) -> i32 {
    let mut return_code: i32;
    let mut kmap: Keymap;
    let mut saved_keymap: Keymap;
    let mut flags: i32;
    let mut opt: i32;
    let mut initfile: *mut libc::c_char;
    let mut map_name: *mut libc::c_char;
    let mut fun_name: *mut libc::c_char;
    let mut unbind_name: *mut libc::c_char;
    let mut remove_seq: *mut libc::c_char;
    let mut cmd_seq: *mut libc::c_char;
    let t: *mut libc::c_char;

    unsafe {
        if no_line_editing != 0 {
            builtin_warning(dcgettext(
                0 as *const libc::c_char,
                CString::new("line editing not enabled").unwrap().as_ptr() as *const libc::c_char,
                5 as libc::c_int,
            ));
        }

        kmap = std::ptr::null_mut();
        saved_keymap = std::ptr::null_mut();
        flags = 0;
        initfile = std::ptr::null_mut();
        map_name = std::ptr::null_mut();
        fun_name = std::ptr::null_mut();
        unbind_name = std::ptr::null_mut();
        remove_seq = std::ptr::null_mut();
        cmd_seq = std::ptr::null_mut();

        return_code = EXECUTION_SUCCESS!();

        if bash_readline_initialized == 0 {
            initialize_readline();
        }
        let bind_str = CString::new("bind_builtin").unwrap();

        begin_unwind_frame(bind_str.as_ptr() as *mut libc::c_char);

        unwind_protect_mem(
            &mut rl_outstream as *mut *mut FILE as *mut libc::c_char,
            ::std::mem::size_of::<*mut libc::FILE>() as libc::c_ulong as libc::c_int,
        );

        rl_outstream = stdout;
        reset_internal_getopt();

        let c_str = CString::new("lvpVPsSXf:q:u:m:r:x:").unwrap();
        let c_ptr = c_str.as_ptr() as *mut libc::c_char;
        opt = internal_getopt(list, c_ptr);
        while opt != -1 {
            let optu8 = opt as u8;
            let opt_char = char::from(optu8);
            match opt_char {
                'l' => flags |= LFLAG!(),
                'v' => flags |= VFLAG!(),
                'p' => flags |= PFLAG!(),
                'f' => {
                    flags |= FFLAG!();
                    initfile = list_optarg;
                }
                'm' => {
                    flags |= MFLAG!();
                    map_name = list_optarg;
                }
                'q' => {
                    flags |= QFLAG!();
                    fun_name = list_optarg;
                }
                'u' => {
                    flags |= UFLAG!();
                    unbind_name = list_optarg;
                }
                'r' => {
                    flags |= RFLAG!();
                    remove_seq = list_optarg;
                }
                'V' => flags |= VVFLAG!(),
                'P' => flags |= PPFLAG!(),
                's' => flags |= SFLAG!(),
                'S' => flags |= SSFLAG!(),
                'x' => {
                    flags |= XFLAG!();
                    cmd_seq = list_optarg;
                }
                'X' => flags |= XXFLAG!(),
                _ => {
                    if opt == -99 {
                        builtin_help();
                        return EX_USAGE!();
                    }
                    builtin_usage();

                    return_code = EX_USAGE!();
                    if !saved_keymap.is_null() {
                        rl_set_keymap(saved_keymap);
                    }
                    run_unwind_frame(bind_str.as_ptr() as *mut libc::c_char);
                    if return_code < 0 {
                        return_code = EXECUTION_FAILURE!();
                    }
                    return sh_chkwrite(return_code);
                }
            }
            opt = internal_getopt(list, c_ptr);
        }

        list = loptend;

        /* First, see if we need to install a special keymap for this
        command.  Then start on the arguments. */

        if (flags & MFLAG!()) != 0 && !map_name.is_null() {
            kmap = rl_get_keymap_by_name(map_name);
            if kmap.is_null() {
                let names = String::from("invaildmap");
                err_translate_fn(&names, map_name);
                println!();
                return_code = EXECUTION_FAILURE!();
                if !saved_keymap.is_null() {
                    rl_set_keymap(saved_keymap);
                }
                run_unwind_frame(bind_str.as_ptr() as *mut libc::c_char);
                if return_code < 0 {
                    return_code = EXECUTION_FAILURE!();
                }
                return sh_chkwrite(return_code);
            }
        }

        if !kmap.is_null() {
            saved_keymap = rl_get_keymap();
            rl_set_keymap(kmap);
        }

        /* XXX - we need to add exclusive use tests here.  It doesn't make sense
        to use some of these options together. */
        /* Now hack the option arguments */
        if flags & LFLAG!() != 0 {
            rl_list_funmap_names();
        }
        if flags & PFLAG!() != 0 {
            rl_function_dumper(1);
        }
        if flags & PPFLAG!() != 0 {
            rl_function_dumper(0);
        }
        if flags & SFLAG!() != 0 {
            rl_macro_dumper(1);
        }
        if flags & SSFLAG!() != 0 {
            rl_macro_dumper(0);
        }
        if flags & VFLAG!() != 0 {
            rl_variable_dumper(1);
        }
        if flags & VVFLAG!() != 0 {
            rl_variable_dumper(0);
        }

        if (flags & FFLAG!()) != 0 && !initfile.is_null() {
            if rl_read_init_file(initfile) != 0 {
                t = printable_filename(initfile, 0);
                let c_str = CString::new("%s: cannot read: %s").unwrap();
                let c_ptr = c_str.as_ptr();

                builtin_error(c_ptr, t, strerror(5 as libc::c_int));
                if t != initfile {
                    free(t as *mut c_void);
                }
                return_code = EXECUTION_FAILURE!();
                if !saved_keymap.is_null() {
                    rl_set_keymap(saved_keymap);
                }
                run_unwind_frame(bind_str.as_ptr() as *mut libc::c_char);
                if return_code < 0 {
                    return_code = EXECUTION_FAILURE!();
                }
                return sh_chkwrite(return_code);
            }
        }

        if (flags & QFLAG!()) != 0 && !fun_name.is_null() {
            return_code = query_bindings(fun_name);
        }

        if (flags & UFLAG!()) != 0 && !unbind_name.is_null() {
            return_code = unbind_command(unbind_name);
        }

        if (flags & RFLAG!()) != 0 && !remove_seq.is_null() {
            opt = unbind_keyseq(remove_seq);
            return_code = opt;
            if !saved_keymap.is_null() {
                rl_set_keymap(saved_keymap);
            }
            run_unwind_frame(bind_str.as_ptr() as *mut libc::c_char);
            if return_code < 0 {
                return_code = EXECUTION_FAILURE!();
            }
            return sh_chkwrite(return_code);
        }

        if flags & XFLAG!() != 0 {
            return_code = bind_keyseq_to_unix_command(cmd_seq);
        }

        if flags & XXFLAG!() != 0 {
            return_code = print_unix_command_map();
        }

        /* Process the rest of the arguments as binding specifications. */
        while !list.is_null() {
            let olen: i32;
            let nlen: i32;
            let mut d: i32;
            let mut i: i32;
            let obindings: *mut *mut libc::c_char;
            let nbindings: *mut *mut libc::c_char;

            obindings = rl_invoking_keyseqs(bash_execute_unix_command as *mut rl_command_func_t);
            if !obindings.is_null() {
                olen = strvec_len(obindings);
            } else {
                olen = 0;
            }

            rl_parse_and_bind((*(*list).word).word);

            nbindings = rl_invoking_keyseqs(bash_execute_unix_command as *mut rl_command_func_t);
            if !nbindings.is_null() {
                nlen = strvec_len(nbindings);
            } else {
                nlen = 0;
            }

            if nlen < olen {
                d = olen - nlen;
                i = 0;
                let mut t: *mut libc::c_char;
                while i < olen && d > 0 {
                    t = *((obindings as usize + (i * 8) as usize) as *mut *mut libc::c_char)
                        as *mut libc::c_char;
                    if nlen == 0 || strvec_search(nbindings, t) >= 0 {
                        unbind_unix_command(t);
                        d = d - 1;
                    }
                    i += 1;
                }
            }

            strvec_dispose(obindings);
            strvec_dispose(nbindings);

            list = (*list).next;
        }

        if !saved_keymap.is_null() {
            rl_set_keymap(saved_keymap);
        }
        run_unwind_frame(bind_str.as_ptr() as *mut libc::c_char);

        if return_code < 0 {
            return_code = EXECUTION_FAILURE!();
        }

        return sh_chkwrite(return_code);
    } //unsafe
}

#[no_mangle]
fn query_bindings(name: *mut libc::c_char) -> i32 {
    let function: *mut rl_command_func_t;
    let keyseqs: *mut *mut libc::c_char;
    let mut j: i32;
    // let mut name_str:String;

    unsafe {
        function = rl_named_function(name);
        if function.is_null() {
            let names = String::from("unknowdfunction");
            err_translate_fn(&names, name);
            println!();
            return EXECUTION_FAILURE!();
        }

        keyseqs = rl_invoking_keyseqs(function);

        if keyseqs.is_null() {
            let names = String::from("bindnokeys");
            err_translate_fn(&names, name);
            println!();
            return EXECUTION_FAILURE!();
        }
        let names = String::from("bindvia");
        translate_fn(&names, name);
        j = 0;
        let mut t: *mut libc::c_char;
        t = *keyseqs;
        while j < 5 && !t.is_null() {
            let c: String;
            if !(*((keyseqs as usize + ((j + 1) * 8) as usize) as *mut *mut libc::c_char)
                as *mut libc::c_char)
                .is_null()
            {
                c = String::from(",");
            } else {
                c = String::from(".\n");
            }
            let c_cstr = CStr::from_ptr(t);
            let c_str = c_cstr.to_str().unwrap();
            print!("\"{}\"{}", c_str, c);
            j += 1;
            t = *((keyseqs as usize + (j * 8) as usize) as *mut *mut libc::c_char)
                as *mut libc::c_char;
        }
        if !(*((keyseqs as usize + (j * 8) as usize) as *mut *mut libc::c_char)
            as *mut libc::c_char)
            .is_null()
        {
            print!("...\n");
        }
        strvec_dispose(keyseqs);
        return EXECUTION_SUCCESS!();
    }
}

#[no_mangle]
fn unbind_command(name: *mut libc::c_char) -> i32 {
    let function: *mut rl_command_func_t;

    unsafe {
        function = rl_named_function(name);
        if function.is_null() {
            let names = String::from("unknowdfunction");
            err_translate_fn(&names, name);
            println!();
            return EXECUTION_FAILURE!();
        }

        rl_unbind_function_in_map(function, rl_get_keymap());
        return EXECUTION_SUCCESS!();
    }
}

#[no_mangle]
fn unbind_keyseq(seq: *mut libc::c_char) -> i32 {
    let kseq: *mut libc::c_char;
    let mut kslen: i32 = 0;
    let mut type1: i32 = 0;
    let mut f: Option<rl_command_func_t> = None;
    unsafe {
        kseq = malloc((2 * strlen(seq) as usize) + 1) as *mut libc::c_char;
        if rl_translate_keyseq(seq, kseq, &mut kslen) != 0 {
            free(kseq as *mut c_void);
            let names = String::from("unbindfaild");
            err_translate_fn(&names, seq);
            println!();
            return EXECUTION_FAILURE!();
        }
        //可能存在错误
        f = rl_function_of_keyseq_len(kseq, kslen as size_t, 0 as Keymap, &mut type1);
        if f.is_none() {
            free(kseq as *mut c_void);
            return EXECUTION_SUCCESS!();
        }

        if type1 == ISKMAP!() {
            //不清楚这个条件是如何触发的，所以下面语句可能存在问题
            // f = (*(f as Keymap).offset(ANYOTHERKEY!() as isize)).function ;
            f = (*(::std::mem::transmute::<Option<rl_command_func_t>, Keymap>(f))
                .offset(ANYOTHERKEY!() as isize))
            .function;
        }

        /* I wish this didn't have to translate the key sequence again, but readline
        doesn't have a binding function that takes a translated key sequence as
        an argument. */
        if rl_bind_keyseq(seq, std::ptr::null_mut() as *mut rl_command_func_t) != 0 {
            free(kseq as *mut c_void);
            let names = String::from("unbindfaild");
            err_translate_fn(&names, seq);
            println!();
            return EXECUTION_FAILURE!();
        }

        if f == Some(bash_execute_unix_command as rl_command_func_t) {
            unbind_unix_command(seq);
        }
        free(kseq as *mut c_void);
        return EXECUTION_SUCCESS!();
    }
}
