/*
 * SPDX-FileCopyrightText: 2025 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: GPL-2.0-or-later
 */
//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

use super::common::{builtin_usage, sh_notfound, sh_restricted};
use super::help::builtin_help;
use super::type_1::describe_command;
use crate::builtins::bashgetopt::{internal_getopt, reset_internal_getopt};
use crate::copycmd::copy_word_list;
use crate::execute_cmd::execute_command;
use crate::make_cmd::make_bare_simple_command;
use crate::src_common::*;
use crate::unwind_prot::{begin_unwind_frame, run_unwind_frame};

#[no_mangle]
pub fn command_builtin(mut list: *mut WordList) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut verbose: libc::c_int = 0;
    let mut use_standard_path: libc::c_int = 0;
    let mut opt: libc::c_int = 0;
    let mut command: *mut COMMAND = 0 as *mut COMMAND;
    use_standard_path = 0 as libc::c_int;
    verbose = use_standard_path;

    reset_internal_getopt();
    let adnpsf = CString::new("pvV").expect("CString::new failed");
    loop {
        opt = internal_getopt(list, adnpsf.as_ptr() as *mut libc::c_char);
        if !(opt != -1) {
            break;
        }
        let opt_char = opt as u8 as char;
        match opt_char {
            'p' => {
                use_standard_path = CDESC_STDPATH!();
            }
            'V' => {
                verbose = CDESC_SHORTDESC!() | CDESC_ABSPATH!();
            }
            'v' => {
                verbose = CDESC_REUSABLE!(); // ditto
            }
            _ => {
                if opt == -99 {
                    builtin_help();
                    return EX_USAGE!();
                }
                builtin_usage();
                return EX_USAGE!();
            }
        }
    }
    unsafe {
        list = loptend;
        if list.is_null() {
            return EXECUTION_SUCCESS!();
        }
        if use_standard_path != 0 && restricted != 0 {
            sh_restricted(b"-p\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
            return EXECUTION_FAILURE!();
        }
        if verbose != 0 {
            let mut found: libc::c_int = 0;
            let mut any_found: libc::c_int = 0;
            any_found = 0 as libc::c_int;
            while !list.is_null() {
                found = describe_command((*(*list).word).word, verbose | use_standard_path);
                if found == 0 as libc::c_int && verbose != CDESC_REUSABLE!() {
                    sh_notfound((*(*list).word).word);
                }
                any_found += found;
                list = (*list).next;
            }
            return if any_found != 0 {
                EXECUTION_SUCCESS!()
            } else {
                EXECUTION_FAILURE!()
            };
        }
        begin_unwind_frame(const_command_builtin);
        command = make_bare_simple_command();

        (*(*command).value.Simple).words = copy_word_list(list);

        (*(*command).value.Simple).redirects = 0 as *mut libc::c_void as *mut REDIRECT;

        (*command).flags |= CMD_NO_FUNCTIONS
            | CMD_INHIBIT_EXPANSION
            | CMD_COMMAND_BUILTIN
            | (if use_standard_path != 0 {
                CMD_STDPATH
            } else {
                0 as libc::c_int
            });
        (*(*command).value.Simple).flags |= CMD_NO_FUNCTIONS
            | CMD_INHIBIT_EXPANSION
            | CMD_COMMAND_BUILTIN
            | (if use_standard_path != 0 {
                CMD_STDPATH
            } else {
                0 as libc::c_int
            });
        /*add_unwind_protect(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut COMMAND) -> ()>,
                *mut libc::c_char,
            >(Some(dispose_command as unsafe extern "C" fn(*mut COMMAND) -> ())),
            command,
        );*/
        result = execute_command(command);
        run_unwind_frame(
            b"command_builtin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return result;
    }
}
