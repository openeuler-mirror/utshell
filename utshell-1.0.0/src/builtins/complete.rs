//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

use super::help::builtin_help;
use crate::bashline::bash_default_completion;
use crate::builtins::{
    bashgetopt::{internal_getopt, reset_internal_getopt},
    common::{builtin_usage, sh_chkwrite, sh_invalidid, sh_invalidopt, sh_invalidoptname},
};
use crate::dispose_cmd::dispose_words;
use crate::general::check_identifier;
use crate::make_cmd::{make_bare_word, make_word_list};
use crate::pcomplete::{
    completions_to_stringlist, gen_compspec_completions, pcomp_curcmd, pcomp_curcs, pcomp_ind,
    pcomp_line, pcomp_set_compspec_options, pcomp_set_readline_variables,
};
use crate::pcomplib::{
    compspec_create, compspec_dispose, progcomp_flush, progcomp_insert, progcomp_remove,
    progcomp_search, progcomp_walk,
};
use crate::src_common::*;

extern "C" {
    static mut rl_readline_state: libc::c_ulong;
    fn rl_filename_completion_function(text: *const libc::c_char, state: i32) -> *mut libc::c_char;
    fn rl_completion_matches(
        text: *const libc::c_char,
        entry_function: unsafe extern "C" fn(
            text: *const libc::c_char,
            state: i32,
        ) -> *mut libc::c_char,
    ) -> *mut *mut libc::c_char;
    fn strlist_print(strlist: *mut STRINGLIST, text: *mut libc::c_char);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _compacts {
    actname: *const libc::c_char,
    actflag: libc::c_ulong,
    actopt: libc::c_int,
}

pub struct CompactsArray {
    compactsArr: [_compacts; 25usize],
}

impl CompactsArray {
    pub fn new() -> CompactsArray {
        CompactsArray {
            compactsArr: [
                _compacts {
                    actname: b"alias\0".as_ptr() as *const libc::c_char,
                    actflag: CA_ALIAS!(),
                    actopt: 'a' as libc::c_int,
                },
                _compacts {
                    actname: b"arrayvar\0".as_ptr() as *const libc::c_char,
                    actflag: CA_ARRAYVAR!(),
                    actopt: 0 as libc::c_int,
                },
                _compacts {
                    actname: b"binding\0".as_ptr() as *const libc::c_char,
                    actflag: CA_BINDING!(),
                    actopt: 0 as libc::c_int,
                },
                _compacts {
                    actname: b"builtin\0".as_ptr() as *const libc::c_char,
                    actflag: CA_BUILTIN!(),
                    actopt: 'b' as libc::c_int,
                },
                _compacts {
                    actname: b"command\0".as_ptr() as *const libc::c_char,
                    actflag: CA_COMMAND!(),
                    actopt: 'c' as libc::c_int,
                },
                _compacts {
                    actname: b"directory\0".as_ptr() as *const libc::c_char,
                    actflag: CA_DIRECTORY!(),
                    actopt: 'd' as libc::c_int,
                },
                _compacts {
                    actname: b"disabled\0".as_ptr() as *const libc::c_char,
                    actflag: CA_DISABLED!(),
                    actopt: 0 as libc::c_int,
                },
                _compacts {
                    actname: b"enabled\0".as_ptr() as *const libc::c_char,
                    actflag: CA_ENABLED!(),
                    actopt: 0 as libc::c_int,
                },
                _compacts {
                    actname: b"export\0".as_ptr() as *const libc::c_char,
                    actflag: CA_EXPORT!(),
                    actopt: 'e' as libc::c_int,
                },
                _compacts {
                    actname: b"file\0".as_ptr() as *const libc::c_char,
                    actflag: CA_FILE!(),
                    actopt: 'f' as libc::c_int,
                },
                _compacts {
                    actname: b"function\0".as_ptr() as *const libc::c_char,
                    actflag: CA_FUNCTION!(),
                    actopt: 0 as libc::c_int,
                },
                _compacts {
                    actname: b"helptopic\0".as_ptr() as *const libc::c_char,
                    actflag: CA_HELPTOPIC!(),
                    actopt: 0 as libc::c_int,
                },
                _compacts {
                    actname: b"hostname\0".as_ptr() as *const libc::c_char,
                    actflag: CA_HOSTNAME!(),
                    actopt: 0 as libc::c_int,
                },
                _compacts {
                    actname: b"group\0".as_ptr() as *const libc::c_char,
                    actflag: CA_GROUP!(),
                    actopt: 'g' as libc::c_int,
                },
                _compacts {
                    actname: b"job\0".as_ptr() as *const libc::c_char,
                    actflag: CA_JOB!(),
                    actopt: 'j' as libc::c_int,
                },
                _compacts {
                    actname: b"keyword\0".as_ptr() as *const libc::c_char,
                    actflag: CA_KEYWORD!(),
                    actopt: 'k' as libc::c_int,
                },
                _compacts {
                    actname: b"running\0".as_ptr() as *const libc::c_char,
                    actflag: CA_RUNNING!(),
                    actopt: 0 as libc::c_int,
                },
                _compacts {
                    actname: b"service\0".as_ptr() as *const libc::c_char,
                    actflag: CA_SERVICE!(),
                    actopt: 's' as libc::c_int,
                },
                _compacts {
                    actname: b"setopt\0".as_ptr() as *const libc::c_char,
                    actflag: CA_SETOPT!(),
                    actopt: 0 as libc::c_int,
                },
                _compacts {
                    actname: b"shopt\0".as_ptr() as *const libc::c_char,
                    actflag: CA_SHOPT!(),
                    actopt: 0 as libc::c_int,
                },
                _compacts {
                    actname: b"signal\0".as_ptr() as *const libc::c_char,
                    actflag: CA_SIGNAL!(),
                    actopt: 0 as libc::c_int,
                },
                _compacts {
                    actname: b"stopped\0".as_ptr() as *const libc::c_char,
                    actflag: CA_STOPPED!(),
                    actopt: 0 as libc::c_int,
                },
                _compacts {
                    actname: b"user\0".as_ptr() as *const libc::c_char,
                    actflag: CA_USER!(),
                    actopt: 'u' as libc::c_int,
                },
                _compacts {
                    actname: b"variable\0".as_ptr() as *const libc::c_char,
                    actflag: CA_VARIABLE!(),
                    actopt: 'v' as libc::c_int,
                },
                _compacts {
                    actname: std::ptr::null_mut(),
                    actflag: 0,
                    actopt: 0 as libc::c_int,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _compopt {
    optname: *const libc::c_char,
    optflag: libc::c_ulong,
}

pub struct CompoptArray {
    compoptArr: [_compopt; 9usize],
}

impl CompoptArray {
    pub fn new() -> CompoptArray {
        CompoptArray {
            compoptArr: [
                _compopt {
                    optname: "bashdefault\0".as_ptr() as *const libc::c_char,
                    optflag: COPT_BASHDEFAULT!(),
                },
                _compopt {
                    optname: "default\0".as_ptr() as *const libc::c_char,
                    optflag: COPT_DEFAULT!(),
                },
                _compopt {
                    optname: "dirnames\0".as_ptr() as *const libc::c_char,
                    optflag: COPT_DIRNAMES!(),
                },
                _compopt {
                    optname: "filenames\0".as_ptr() as *const libc::c_char,
                    optflag: COPT_FILENAMES!(),
                },
                _compopt {
                    optname: "noquote\0".as_ptr() as *const libc::c_char,
                    optflag: COPT_NOQUOTE!(),
                },
                _compopt {
                    optname: "nosort\0".as_ptr() as *const libc::c_char,
                    optflag: COPT_NOSORT!(),
                },
                _compopt {
                    optname: "nospace\0".as_ptr() as *const libc::c_char,
                    optflag: COPT_NOSPACE!(),
                },
                _compopt {
                    optname: "plusdirs\0".as_ptr() as *const libc::c_char,
                    optflag: COPT_PLUSDIRS!(),
                },
                _compopt {
                    optname: std::ptr::null_mut(),
                    optflag: 0,
                },
            ],
        }
    }
}

pub static mut Garg: *mut libc::c_char = std::ptr::null_mut();
pub static mut Warg: *mut libc::c_char = std::ptr::null_mut();
pub static mut Parg: *mut libc::c_char = std::ptr::null_mut();
pub static mut Sarg: *mut libc::c_char = std::ptr::null_mut();
pub static mut Xarg: *mut libc::c_char = std::ptr::null_mut();
pub static mut Farg: *mut libc::c_char = std::ptr::null_mut();
pub static mut Carg: *mut libc::c_char = std::ptr::null_mut();

fn STRDUP(x: *const libc::c_char) -> *mut libc::c_char {
    if x != std::ptr::null_mut() {
        unsafe {
            return savestring!(x);
        }
    } else {
        return std::ptr::null_mut();
    }
}

fn STREQ(a: *const libc::c_char, b: *const libc::c_char) -> bool {
    unsafe {
        return *a == *b && libc::strcmp(a, b) == 0;
    }
}

fn shell_break_chars() -> *const libc::c_char {
    return b"()<>;&| \t\n\0".as_ptr() as *const libc::c_char;
}

fn EMPTYCMD() -> *const libc::c_char {
    return b"_EmptycmD_\0".as_ptr() as *const libc::c_char;
}

fn DEFAULTCMD() -> *const libc::c_char {
    return b"_DefaultCmD_\0".as_ptr() as *const libc::c_char;
}

fn INITIALWORD() -> *const libc::c_char {
    return b"_InitialWorD_\0".as_ptr() as *const libc::c_char;
}

fn RL_ISSTATE(x: libc::c_ulong) -> libc::c_ulong {
    unsafe {
        return rl_readline_state & x;
    }
}

#[no_mangle]
pub fn find_compact(name: *mut libc::c_char) -> i32 {
    let mut i: i32 = 0;
    unsafe {
        let compacts: CompactsArray = CompactsArray::new();
        while compacts.compactsArr[i as usize].actname != std::ptr::null_mut() {
            let tmp = CStr::from_ptr(compacts.compactsArr[i as usize].actname);
            if STREQ(name, compacts.compactsArr[i as usize].actname) {
                return i;
            }
            i += 1;
        }
        return -1;
    }
}

#[no_mangle]
pub fn find_compopt(name: *mut libc::c_char) -> i32 {
    let mut i: i32 = 0;
    let compopts: CompoptArray = CompoptArray::new();

    while compopts.compoptArr[i as usize].optname != std::ptr::null_mut() {
        if STREQ(name, compopts.compoptArr[i as usize].optname) {
            return i;
        }
        i += 1;
    }
    return -1;
}

#[no_mangle]
pub fn build_actions(
    mut list: *mut WordList,
    flagp: *mut _optflags,
    actp: *mut libc::c_ulong,
    optp: *mut libc::c_ulong,
) -> i32 {
    let mut opt: i32;
    let mut ind: i32;
    let mut opt_given: i32 = 0;
    let mut acts: libc::c_ulong = 0;
    let mut copts: libc::c_ulong = 0;
    let mut w: WordDesc = WordDesc {
        word: std::ptr::null_mut(),
        flags: 0,
    };

    unsafe {
        reset_internal_getopt();
        opt = internal_getopt(
            list,
            CString::new("abcdefgjko:prsuvA:G:W:P:S:X:F:C:DEI")
                .unwrap()
                .as_ptr() as *mut libc::c_char,
        );
        while opt != -1 {
            opt_given = 1;
            let optu8: u8 = opt as u8;
            let optChar: char = char::from(optu8);
            match optChar {
                'r' => {
                    if flagp != std::ptr::null_mut() {
                        (*flagp).rflag = 1;
                    } else {
                        sh_invalidopt(CString::new("-r").unwrap().as_ptr() as *mut libc::c_char);
                        builtin_usage();
                        return EX_USAGE!();
                    }
                }
                'p' => {
                    if flagp != std::ptr::null_mut() {
                        (*flagp).pflag = 1;
                    } else {
                        sh_invalidopt(CString::new("-p").unwrap().as_ptr() as *mut libc::c_char);
                        builtin_usage();
                        return EX_USAGE!();
                    }
                }
                'a' => {
                    acts |= CA_ALIAS!();
                }
                'b' => {
                    acts |= CA_BUILTIN!();
                }
                'c' => {
                    acts |= CA_COMMAND!();
                }
                'd' => {
                    acts |= CA_DIRECTORY!();
                }
                'e' => {
                    acts |= CA_EXPORT!();
                }
                'f' => {
                    acts |= CA_FILE!();
                }
                'g' => {
                    acts |= CA_GROUP!();
                }
                'j' => {
                    acts |= CA_GROUP!();
                }
                'k' => {
                    acts |= CA_KEYWORD!();
                }
                's' => {
                    acts |= CA_SERVICE!();
                }
                'u' => {
                    acts |= CA_USER!();
                }
                'v' => {
                    acts |= CA_VARIABLE!();
                }
                'o' => {
                    ind = find_compopt(list_optarg);
                    if ind < 0 {
                        sh_invalidoptname(list_optarg);
                        return EX_USAGE!();
                    }
                    let compopts: CompoptArray = CompoptArray::new();
                    copts |= compopts.compoptArr[ind as usize].optflag;
                }
                'A' => {
                    ind = find_compact(list_optarg);
                    if ind < 0 {
                        builtin_error(
                            CString::new("%s: invalid action name").unwrap().as_ptr(),
                            list_optarg,
                        );
                        return EX_USAGE!();
                    }
                    let compacts: CompactsArray = CompactsArray::new();
                    acts |= compacts.compactsArr[ind as usize].actflag;
                }
                'C' => {
                    Carg = list_optarg;
                }
                'D' => {
                    if flagp != std::ptr::null_mut() {
                        (*flagp).Dflag = 1;
                    } else {
                        sh_invalidopt(CString::new("-D").unwrap().as_ptr() as *mut libc::c_char);
                        builtin_usage();
                        return EX_USAGE!();
                    }
                }
                'E' => {
                    if flagp != std::ptr::null_mut() {
                        (*flagp).Eflag = 1;
                    } else {
                        sh_invalidopt(CString::new("-E").unwrap().as_ptr() as *mut libc::c_char);
                        builtin_usage();
                        return EX_USAGE!();
                    }
                }
                'I' => {
                    if flagp != std::ptr::null_mut() {
                        (*flagp).Iflag = 1;
                    } else {
                        sh_invalidopt(CString::new("-I").unwrap().as_ptr() as *mut libc::c_char);
                        builtin_usage();
                        return EX_USAGE!();
                    }
                }
                'F' => {
                    w.word = list_optarg;
                    Farg = list_optarg;
                    w.flags = 0;
                    if check_identifier(&mut w, posixly_correct) == 0
                        || libc::strpbrk(Farg, shell_break_chars()) != std::ptr::null_mut()
                    {
                        sh_invalidid(Farg);
                        return EX_USAGE!();
                    }
                }
                'G' => {
                    Garg = list_optarg;
                }
                'P' => {
                    Parg = list_optarg;
                }
                'S' => {
                    Sarg = list_optarg;
                }
                'W' => {
                    Warg = list_optarg;
                }
                'X' => {
                    Xarg = list_optarg;
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
            opt = internal_getopt(
                list,
                CString::new("abcdefgjko:prsuvA:G:W:P:S:X:F:C:DEI")
                    .unwrap()
                    .as_ptr() as *mut libc::c_char,
            );
        }
        *actp = acts;
        *optp = copts;
        list = loptend.clone();
        if opt_given != 0 {
            return EXECUTION_SUCCESS!();
        } else {
            return EXECUTION_FAILURE!();
        }
    }
}

/* Add, remove, and display completion specifiers. */
#[no_mangle]
pub fn complete_builtin(listt: *mut WordList) -> i32 {
    let mut opt_given: i32 = 0;
    let mut rval: i32;
    let mut acts: libc::c_ulong = 0;
    let mut copts: libc::c_ulong = 0;
    let mut cs: *mut COMPSPEC;
    let mut oflags: _optflags = _optflags {
        pflag: 0,
        rflag: 0,
        Dflag: 0,
        Eflag: 0,
        Iflag: 0,
    };
    let mut l: *mut WordList;
    let wl: *mut WordList;

    unsafe {
        let mut list: *mut WordList = listt.clone();
        if list == std::ptr::null_mut() {
            print_all_completions();
            return EXECUTION_SUCCESS!();
        }

        oflags.pflag = 0;
        oflags.rflag = 0;
        oflags.Dflag = 0;
        oflags.Eflag = 0;
        oflags.Iflag = 0;

        Garg = std::ptr::null_mut();
        Warg = std::ptr::null_mut();
        Parg = std::ptr::null_mut();
        Sarg = std::ptr::null_mut();
        Xarg = std::ptr::null_mut();
        Farg = std::ptr::null_mut();
        Carg = std::ptr::null_mut();

        cs = std::ptr::null_mut();

        /* Build the actions from the arguments.  Also sets the [A-Z]arg variables
        as a side effect if they are supplied as options. */
        rval = build_actions(list, &mut oflags, &mut acts, &mut copts);
        if rval == EX_USAGE!() {
            return rval;
        }

        opt_given = (rval != EXECUTION_FAILURE!()) as i32;

        list = loptend.clone();

        if oflags.Dflag != 0 {
            wl = make_word_list(make_bare_word(DEFAULTCMD()), std::ptr::null_mut());
        } else if oflags.Eflag != 0 {
            wl = make_word_list(make_bare_word(EMPTYCMD()), std::ptr::null_mut());
        } else if oflags.Iflag != 0 {
            wl = make_word_list(make_bare_word(INITIALWORD()), std::ptr::null_mut());
        } else {
            wl = std::ptr::null_mut();
        }

        /* -p overrides everything else */
        if oflags.pflag != 0 || (list == std::ptr::null_mut() && opt_given == 0) {
            if wl != std::ptr::null_mut() {
                rval = print_cmd_completions(wl);
                dispose_words(wl);
                return rval;
            } else if list == std::ptr::null_mut() {
                //给了P,但没给参数，直接打印全部并退出
                print_all_completions();
                return EXECUTION_SUCCESS!();
            }
            return print_cmd_completions(list);
        }

        /* next, -r overrides everything else. */
        if oflags.rflag != 0 {
            if wl != std::ptr::null_mut() {
                rval = remove_cmd_completions(wl);
                dispose_words(wl);
                return rval;
            } else if list == std::ptr::null_mut() {
                progcomp_flush();
                return EXECUTION_SUCCESS!();
            }
            return remove_cmd_completions(list);
        }

        if wl == std::ptr::null_mut() && list == std::ptr::null_mut() && opt_given != 0 {
            builtin_usage();
            return EX_USAGE!();
        }

        /* If we get here, we need to build a compspec and add it for each
        remaining argument. */
        cs = compspec_create();
        (*cs).actions = acts;
        (*cs).options = copts;

        (*cs).globpat = STRDUP(Garg);
        (*cs).words = STRDUP(Warg);
        (*cs).prefix = STRDUP(Parg);
        (*cs).suffix = STRDUP(Sarg);
        (*cs).funcname = STRDUP(Farg);
        (*cs).command = STRDUP(Carg);
        (*cs).filterpat = STRDUP(Xarg);

        rval = EXECUTION_SUCCESS!();

        if wl != std::ptr::null_mut() {
            l = wl.clone();
        } else {
            l = list.clone();
        }

        while l != std::ptr::null_mut() {
            /* Add CS as the compspec for the specified commands. */
            if progcomp_insert((*(*l).word).word, cs) == 0 {
                rval = EXECUTION_FAILURE!();
            }
            l = (*l).next;
        }

        dispose_words(wl);
        return rval;
    }
}

#[no_mangle]
pub fn remove_cmd_completions(list: *mut WordList) -> i32 {
    let mut l: *mut WordList;
    let mut ret: i32;
    unsafe {
        ret = EXECUTION_SUCCESS!();
        l = list.clone();
        while l != std::ptr::null_mut() {
            if progcomp_remove((*(*l).word).word) == 0 {
                builtin_error(
                    CString::new("%s: no completion specification")
                        .unwrap()
                        .as_ptr(),
                    (*(*l).word).word,
                );
                ret = EXECUTION_FAILURE!();
            }
            l = (*l).next;
        }
        return ret;
    }
}

#[no_mangle]
pub fn print_compoptions(copts: libc::c_ulong, full: i32) {
    unsafe {
        let compopts: CompoptArray = CompoptArray::new();
        for i in 0..compopts.compoptArr.len() {
            if (copts & compopts.compoptArr[i].optflag) != 0 {
                libc::printf(
                    CString::new("-o %s ").unwrap().as_ptr(),
                    compopts.compoptArr[i].optname,
                );
            } else if full != 0 {
                libc::printf(
                    CString::new("+o %s ").unwrap().as_ptr(),
                    compopts.compoptArr[i].optname,
                );
            }
        }
    }
}

#[no_mangle]
pub fn print_compactions(acts: libc::c_ulong) {
    unsafe {
        let compacts: CompactsArray = CompactsArray::new();
        for i in 0..compacts.compactsArr.len() {
            if compacts.compactsArr[i].actopt != 0 && (acts & compacts.compactsArr[i].actflag) != 0
            {
                libc::printf(
                    CString::new("-%c ").unwrap().as_ptr(),
                    compacts.compactsArr[i].actopt,
                );
            }
        }

        for i in 0..compacts.compactsArr.len() {
            if compacts.compactsArr[i].actopt == 0 && (acts & compacts.compactsArr[i].actflag) != 0
            {
                libc::printf(
                    CString::new("-A %s ").unwrap().as_ptr(),
                    compacts.compactsArr[i].actname,
                );
            }
        }
    }
}

#[no_mangle]
pub fn print_arg(arg: *const libc::c_char, flag: *const libc::c_char, quote: i32) {
    let x: *mut libc::c_char;
    unsafe {
        if arg != std::ptr::null_mut() {
            if quote != 0 {
                // 复制arg 增加单引号返给x
                x = sh_single_quote(arg as *mut libc::c_char);
            } else {
                x = arg as *mut libc::c_char;
            }
            libc::printf(CString::new("%s %s ").unwrap().as_ptr(), flag, x);
            if x != arg as *mut libc::c_char {
                libc::free(x as *mut c_void);
            }
        }
    }
}

#[no_mangle]
pub fn print_cmd_name(cmd: *const libc::c_char) {
    unsafe {
        if STREQ(cmd, DEFAULTCMD()) {
            libc::printf(CString::new("-D").unwrap().as_ptr());
        } else if STREQ(cmd, EMPTYCMD()) {
            libc::printf(CString::new("-E").unwrap().as_ptr());
        } else if STREQ(cmd, INITIALWORD()) {
            libc::printf(CString::new("-I").unwrap().as_ptr());
        } else if *cmd == 0 {
            /* XXX - can this happen? */
            libc::printf(CString::new("''").unwrap().as_ptr());
        } else {
            libc::printf(CString::new("%s").unwrap().as_ptr(), cmd);
        }
    }
}

#[no_mangle]
pub fn print_one_completion(cmd: *mut libc::c_char, cs: *mut COMPSPEC) -> i32 {
    unsafe {
        libc::printf(CString::new("complete ").unwrap().as_ptr());

        print_compoptions((*cs).options, 0);
        print_compactions((*cs).actions);

        /* now the rest of the arguments */

        /* arguments that require quoting */
        print_arg((*cs).globpat, CString::new("-G").unwrap().as_ptr(), 1);
        print_arg((*cs).words, CString::new("-W").unwrap().as_ptr(), 1);
        print_arg((*cs).prefix, CString::new("-P").unwrap().as_ptr(), 1);
        print_arg((*cs).suffix, CString::new("-S").unwrap().as_ptr(), 1);
        print_arg((*cs).filterpat, CString::new("-X").unwrap().as_ptr(), 1);

        print_arg((*cs).command, CString::new("-C").unwrap().as_ptr(), 1);

        /* simple arguments that don't require quoting */
        print_arg((*cs).funcname, CString::new("-F").unwrap().as_ptr(), 0);

        print_cmd_name(cmd);
        libc::printf(CString::new("\n").unwrap().as_ptr());

        return 0;
    }
}

#[no_mangle]
pub fn print_compopts(cmd: *mut libc::c_char, cs: *mut COMPSPEC, full: i32) {
    unsafe {
        libc::printf(CString::new("compopt ").unwrap().as_ptr());

        print_compoptions((*cs).options, full);
        print_cmd_name(cmd);

        libc::printf(CString::new("\n").unwrap().as_ptr());
    }
}

#[no_mangle]
pub fn print_compitem(item: *mut BUCKET_CONTENTS) -> i32 {
    let cs: *mut COMPSPEC;
    let cmd: *mut libc::c_char;
    unsafe {
        cmd = (*item).key;
        cs = (*item).data as *mut COMPSPEC;
    }

    return print_one_completion(cmd, cs);
}

#[no_mangle]
pub fn print_all_completions() {
    progcomp_walk(Some(print_compitem));
}

#[no_mangle]
pub fn print_cmd_completions(list: *mut WordList) -> i32 {
    let mut l: *mut WordList;
    let mut cs: *mut COMPSPEC;
    let mut ret: i32;

    unsafe {
        ret = EXECUTION_SUCCESS!();
        l = list.clone();
        while l != std::ptr::null_mut() {
            cs = progcomp_search((*(*l).word).word);
            if cs != std::ptr::null_mut() {
                print_one_completion((*(*l).word).word, cs);
            } else {
                builtin_error(
                    CString::new("%s: no completion specification")
                        .unwrap()
                        .as_ptr(),
                    (*(*l).word).word,
                );
                ret = EXECUTION_FAILURE!();
            }
            l = (*l).next;
        }
        return sh_chkwrite(ret);
    }
}

#[no_mangle]
pub fn compgen_builtin(listt: *mut WordList) -> i32 {
    let mut rval: i32;
    let mut acts: libc::c_ulong = 0;
    let mut copts: libc::c_ulong = 0;
    let mut cs: *mut COMPSPEC;
    let mut sl: *mut STRINGLIST;
    let word: *mut libc::c_char;
    let mut matches: *mut *mut libc::c_char;
    let old_line: *mut libc::c_char;
    let old_ind: i32;
    unsafe {
        let mut list: *mut WordList = listt.clone();
        if list == std::ptr::null_mut() {
            return EXECUTION_SUCCESS!();
        }

        Garg = std::ptr::null_mut();
        Warg = std::ptr::null_mut();
        Parg = std::ptr::null_mut();
        Sarg = std::ptr::null_mut();
        Xarg = std::ptr::null_mut();
        Farg = std::ptr::null_mut();
        Carg = std::ptr::null_mut();

        cs = std::ptr::null_mut();

        /* Build the actions from the arguments.  Also sets the [A-Z]arg variables
        as a side effect if they are supplied as options. */
        rval = build_actions(list, std::ptr::null_mut(), &mut acts, &mut copts);
        if rval == EX_USAGE!() {
            return rval;
        }

        if rval == EXECUTION_FAILURE!() {
            return EXECUTION_SUCCESS!();
        }

        list = loptend.clone();

        let wordtmp = CString::new("").unwrap();
        if list != std::ptr::null_mut() && (*list).word != std::ptr::null_mut() {
            word = (*((*list).word)).word;
        } else {
            word = wordtmp.as_ptr() as *mut libc::c_char;
        }

        if Farg != std::ptr::null_mut() {
            builtin_error(
                CString::new("warning: -F option may not work as you expect")
                    .unwrap()
                    .as_ptr(),
            );
        }

        if Carg != std::ptr::null_mut() {
            builtin_error(
                CString::new("warning: -C option may not work as you expect")
                    .unwrap()
                    .as_ptr(),
            );
        }

        /* If we get here, we need to build a compspec and evaluate it. */
        cs = compspec_create();
        (*cs).actions = acts;
        (*cs).options = copts;
        (*cs).refcount = 1;

        (*cs).globpat = STRDUP(Garg);
        (*cs).words = STRDUP(Warg);
        (*cs).prefix = STRDUP(Parg);
        (*cs).suffix = STRDUP(Sarg);
        (*cs).funcname = STRDUP(Farg);
        (*cs).command = STRDUP(Carg);
        (*cs).filterpat = STRDUP(Xarg);

        rval = EXECUTION_FAILURE!();

        /* probably don't have to save these, just being safe */
        old_line = pcomp_line;
        old_ind = pcomp_ind;
        pcomp_line = std::ptr::null_mut();
        pcomp_ind = 0;
        let compgenStr = CString::new("compgen").unwrap();
        sl = gen_compspec_completions(cs, compgenStr.as_ptr(), word, 0, 0, std::ptr::null_mut());
        pcomp_line = old_line;
        pcomp_ind = old_ind;

        /* If the compspec wants the bash default completions, temporarily
        turn off programmable completion and call the bash completion code. */
        if (sl == std::ptr::null_mut() || (*sl).list_len == 0) && (copts & COPT_BASHDEFAULT!()) != 0
        {
            matches = bash_default_completion(word, 0, 0, 0, 0);
            sl = completions_to_stringlist(matches);
            strvec_dispose(matches);
        }

        /* This isn't perfect, but it's the best we can do, given what readline
        exports from its set of completion utility functions. */
        if (sl == std::ptr::null_mut() || (*sl).list_len == 0) && (copts & COPT_DEFAULT!()) != 0 {
            matches = rl_completion_matches(word, rl_filename_completion_function);
            strlist_dispose(sl);
            sl = completions_to_stringlist(matches);
            strvec_dispose(matches);
        }

        if sl != std::ptr::null_mut() {
            if (*sl).list != std::ptr::null_mut() && (*sl).list_len != 0 {
                rval = EXECUTION_SUCCESS!();
                strlist_print(sl, std::ptr::null_mut());
            }
            strlist_dispose(sl);
        }

        compspec_dispose(cs);
        return rval;
    }
}

#[no_mangle]
pub fn compopt_builtin(listt: *mut WordList) -> i32 {
    let mut opts_on: i32 = 0;
    let mut opts_off: i32 = 0;
    let mut opts: *mut i32;
    let mut opt: i32;
    let mut oind: i32;
    let mut ret: i32;
    let mut Dflag: i32 = 0;
    let mut Eflag: i32 = 0;
    let mut Iflag: i32 = 0;
    let mut l: *mut WordList;
    let wl: *mut WordList;
    let mut cs: *mut COMPSPEC;

    ret = EXECUTION_SUCCESS!();
    unsafe {
        let mut list: *mut WordList = listt.clone();
        reset_internal_getopt();

        opt = internal_getopt(
            list,
            CString::new("+o:DEI").unwrap().as_ptr() as *mut libc::c_char,
        );

        while opt != -1 {
            if list_opttype == '-' as i32 {
                opts = &mut opts_on;
            } else {
                opts = &mut opts_off;
            }

            let optu8: u8 = opt as u8;
            let optChar: char = char::from(optu8);

            match optChar {
                'o' => {
                    oind = find_compopt(list_optarg);
                    if oind < 0 {
                        sh_invalidoptname(list_optarg);
                        return EX_USAGE!();
                    }
                    let compopts: CompoptArray = CompoptArray::new();
                    *opts |= compopts.compoptArr[oind as usize].optflag as i32;
                }
                'D' => {
                    Dflag = 1;
                }
                'E' => {
                    Eflag = 1;
                }
                'I' => {
                    Iflag = 1;
                }
                _ => {
                    builtin_usage();
                    return EX_USAGE!();
                }
            }
            opt = internal_getopt(
                list,
                CString::new("+o:DEI").unwrap().as_ptr() as *mut libc::c_char,
            );
        }

        list = loptend.clone();

        if Dflag != 0 {
            wl = make_word_list(make_bare_word(DEFAULTCMD()), std::ptr::null_mut());
        } else if Eflag != 0 {
            wl = make_word_list(make_bare_word(EMPTYCMD()), std::ptr::null_mut());
        } else if Iflag != 0 {
            wl = make_word_list(make_bare_word(INITIALWORD()), std::ptr::null_mut());
        } else {
            wl = std::ptr::null_mut();
        }

        if list == std::ptr::null_mut() && wl == std::ptr::null_mut() {
            if RL_ISSTATE(RL_STATE_COMPLETING!()) == 0 || pcomp_curcs == std::ptr::null_mut() {
                builtin_error(
                    CString::new("not currently executing completion function")
                        .unwrap()
                        .as_ptr(),
                );
                return EXECUTION_FAILURE!();
            }
            cs = pcomp_curcs.clone();

            if opts_on == 0 && opts_off == 0 {
                print_compopts(pcomp_curcmd as *mut libc::c_char, cs, 1);
                return sh_chkwrite(ret);
            }

            /* Set the compspec options */
            pcomp_set_compspec_options(cs, opts_on, 1);
            pcomp_set_compspec_options(cs, opts_off, 0);

            /* And change the readline variables the options control */
            pcomp_set_readline_variables(opts_on, 1);
            pcomp_set_readline_variables(opts_off, 0);

            return ret;
        }

        if wl != std::ptr::null_mut() {
            l = wl.clone();
        } else {
            l = list.clone();
        }

        while l != std::ptr::null_mut() {
            cs = progcomp_search((*((*list).word)).word);
            if cs == std::ptr::null_mut() {
                builtin_error(
                    CString::new("%s: no completion specification")
                        .unwrap()
                        .as_ptr(),
                    (*((*list).word)).word,
                );
                ret = EXECUTION_FAILURE!();
                l = (*l).next;
                continue;
            }
            if opts_on == 0 && opts_off == 0 {
                print_compopts((*((*list).word)).word, cs, 1);
                l = (*l).next;
                continue; /* XXX -- fill in later */
            }

            /* Set the compspec options */
            pcomp_set_compspec_options(cs, opts_on, 1);
            pcomp_set_compspec_options(cs, opts_off, 0);
            l = (*l).next;
        }

        if wl != std::ptr::null_mut() {
            dispose_words(wl);
        }

        return ret;
    }
}
