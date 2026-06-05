use fluent_bundle::FluentArgs;
use fluent_resmgr::resource_manager::ResourceManager;

use crate::builtins::bashgetopt::{internal_getopt, reset_internal_getopt};
use crate::builtins::common::{
    builtin_usage, find_shell_builtin, find_special_builtin, get_local_str, sh_chkwrite,
    sh_notfound,
};
use crate::builtins::help::builtin_help;
use crate::findcmd::{file_status, find_in_path, find_user_command, user_command_matches};
use crate::general::{absolute_program, conf_standard_path};
use crate::hashcmd::phash_search;
use crate::print_cmd::named_function_string;
use crate::src_common::*;
use crate::variables::find_function;
use crate::y_tab::find_reserved_word;

fn function_cell(var: *mut SHELL_VAR) -> *mut COMMAND {
    return (unsafe { *var }).value as *mut COMMAND;
}

#[no_mangle]
pub fn type_builtin(mut list: *mut WordList) -> i32 {
    let mut dflags: i32;
    let mut any_failed: i32 = 0;
    let _opt: i32 = 0;
    let mut this: *mut WordList;

    dflags = CDESC_SHORTDESC!(); /* default */
    unsafe {
        this = list;
        while this != std::ptr::null_mut() && char::from((*(*(*this).word).word) as u8) == '-' {
            let flag = (((*(*this).word).word) as usize + 1) as *mut libc::c_char;
            let c_str_type = CString::new("type").unwrap();
            let c_str_type1 = CString::new("-type").unwrap();
            let c_str_path = CString::new("path").unwrap();
            let c_str_path1 = CString::new("-path").unwrap();
            let c_str_all = CString::new("all").unwrap();
            let c_str_all1 = CString::new("-all").unwrap();
            if STREQ!(flag, c_str_type.as_ptr() as *mut libc::c_char)
                || STREQ!(flag, c_str_type1.as_ptr() as *mut libc::c_char)
            {
                *((*(*this).word).word).offset(1) = 't' as libc::c_char;
                *((*(*this).word).word).offset(2) = '\0' as libc::c_char;
            } else if STREQ!(flag, c_str_path.as_ptr() as *mut libc::c_char)
                || STREQ!(flag, c_str_path1.as_ptr() as *mut libc::c_char)
            {
                *((*(*this).word).word).offset(1) = 'p' as libc::c_char;
                *((*(*this).word).word).offset(2) = '\0' as libc::c_char;
            } else if STREQ!(flag, c_str_all.as_ptr() as *mut libc::c_char)
                || STREQ!(flag, c_str_all1.as_ptr() as *mut libc::c_char)
            {
                *((*(*this).word).word).offset(1) = 'a' as libc::c_char;
                *((*(*this).word).word).offset(2) = '\0' as libc::c_char;
            }

            if (*this).next != std::ptr::null_mut() {
                this = (*this).next;
            } else {
                break;
            }
        }
    }
    reset_internal_getopt();

    let c_str_afptP = CString::new("afptP").unwrap();
    let mut opt = internal_getopt(list, c_str_afptP.as_ptr() as *mut libc::c_char);
    while opt != -1 {
        let optu8: u8 = opt as u8;
        let optChar: char = char::from(optu8);
        match optChar {
            'a' => {
                dflags = dflags | CDESC_ALL!();
            }
            'f' => {
                dflags = dflags | CDESC_NOFUNCS!();
            }
            'p' => {
                dflags = dflags | CDESC_PATH_ONLY!();
                dflags = dflags & !(CDESC_TYPE!() | CDESC_SHORTDESC!());
            }
            't' => {
                dflags = dflags | CDESC_TYPE!();
                dflags = dflags & !(CDESC_PATH_ONLY!() | CDESC_SHORTDESC!());
            }
            'P' => {
                dflags = dflags | CDESC_PATH_ONLY!() | CDESC_FORCE_PATH!();
                dflags = dflags & !(CDESC_TYPE!() | CDESC_SHORTDESC!());
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
        opt = internal_getopt(list, c_str_afptP.as_ptr() as *mut libc::c_char);
    }
    unsafe {
        list = loptend;
    }
    while list != std::ptr::null_mut() {
        let found: i32;
        unsafe {
            found = describe_command((*(*list).word).word, dflags);
        }
        if found == 0 && (dflags & (CDESC_PATH_ONLY!() | CDESC_TYPE!())) == 0 {
            unsafe {
                sh_notfound((*(*list).word).word);
            }
        }

        any_failed += (found == 0 as libc::c_int) as libc::c_int;
        unsafe {
            list = (*list).next;
        }
    }
    if any_failed == 0 {
        opt = EXECUTION_SUCCESS!();
    } else {
        opt = EXECUTION_FAILURE!();
    }
    return sh_chkwrite(opt);
}

pub fn describe_command(command: *mut libc::c_char, dflags: i32) -> i32 {
    let mut found: i32 = 0;
    let mut _i: i32;
    let mut found_file: i32 = 0;
    let mut f: i32;
    let all: i32;
    let mut full_path: *mut libc::c_char;
    let mut x: *mut libc::c_char;
    let mut pathlist: *mut libc::c_char;
    let _func: *mut SHELL_VAR = 0 as *mut SHELL_VAR;

    if (dflags & CDESC_ALL!()) != 0 {
        all = 1;
    } else {
        all = 0;
    }

    // full_path = std::ptr::null_mut();

    /* Command is a shell reserved word? */
    if ((dflags & CDESC_FORCE_PATH!()) == 0) && find_reserved_word(command) >= 0 {
        if dflags & CDESC_TYPE!() != 0 {
            unsafe {
                let c_str_keyword = CString::new("keyword").unwrap();
                libc::puts(c_str_keyword.as_ptr());
            }
        } else if dflags & CDESC_SHORTDESC!() != 0 {
            let name = String::from("iskeyword");
            translation_fn(&name, command, std::ptr::null_mut());
        } else if dflags & CDESC_REUSABLE!() != 0 {
            unsafe {
                println!("{}", CStr::from_ptr(command).to_str().unwrap());
            }
        }

        found = 1;
        if all == 0 {
            return 1;
        }
    }

    /* Command is a function? */
    if (dflags & (CDESC_FORCE_PATH!() | CDESC_NOFUNCS!()) == 0)
        && find_function(command) != std::ptr::null_mut()
    {
        if dflags & CDESC_TYPE!() != 0 {
            unsafe {
                let c_str_function = CString::new("function").unwrap();
                libc::puts(c_str_function.as_ptr());
            }
        } else if dflags & CDESC_SHORTDESC!() != 0 {
            let result: *mut libc::c_char;
            unsafe {
                let name = String::from("isfunction");
                translation_fn(&name, command, std::ptr::null_mut());
                result = named_function_string(
                    command,
                    function_cell(find_function(command)),
                    FUNC_MULTILINE!() | FUNC_EXTERNAL!(),
                );
                println!("{}", CStr::from_ptr(result).to_str().unwrap());
            }
        } else if dflags & CDESC_REUSABLE!() != 0 {
            unsafe {
                println!("{}", CStr::from_ptr(command).to_str().unwrap());
            }
        }

        found = 1;

        if all == 0 {
            return 1;
        }
    }

    /* Command is a builtin? */
    if ((dflags & CDESC_FORCE_PATH!()) == 0) && find_shell_builtin(command).is_some() {
        if dflags & CDESC_TYPE!() != 0 {
            unsafe {
                let c_str_builtin = CString::new("builtin").unwrap();
                libc::puts(c_str_builtin.as_ptr());
            }
        } else if dflags & CDESC_SHORTDESC!() != 0 {
            if unsafe { posixly_correct } != 0 && find_special_builtin(command).is_some() {
                let name = String::from("special");
                translation_fn(&name, command, std::ptr::null_mut());
            } else {
                let name = String::from("isbuiltin");
                translation_fn(&name, command, std::ptr::null_mut());
            }
        } else if dflags & CDESC_REUSABLE!() != 0 {
            unsafe {
                println!("{}", CStr::from_ptr(command).to_str().unwrap());
            }
        }

        found = 1;
        if all == 0 {
            return 1;
        }
    }

    /* Command is a disk file? */
    /* If the command name given is already an absolute command, just
    check to see if it is executable. */
    if absolute_program(command) != 0 {
        f = file_status(command);
        if f & FS_EXECABLE!() != 0 {
            if dflags & CDESC_TYPE!() != 0 {
                unsafe {
                    let c_str_file = CString::new("file").unwrap();
                    libc::puts(c_str_file.as_ptr());
                }
            }
        } else if dflags & CDESC_SHORTDESC!() != 0 {
            let name = String::from("is");
            translation_fn(&name, command, command);
        } else if dflags & (CDESC_REUSABLE!() | CDESC_PATH_ONLY!()) != 0 {
            unsafe {
                println!("{}", CStr::from_ptr(command).to_str().unwrap());
            }

            /* There's no use looking in the hash table or in $PATH,
            because they're not consulted when an absolute program
            name is supplied. */
            return 1;
        }
    }

    /* If the user isn't doing "-a", then we might care about
    whether the file is present in our hash table. */
    if all == 0 || (dflags & CDESC_FORCE_PATH!() != 0) {
        full_path = phash_search(command);
        if full_path != std::ptr::null_mut() {
            if dflags & CDESC_TYPE!() != 0 {
                unsafe {
                    let c_str_file = CString::new("file").unwrap();
                    libc::puts(c_str_file.as_ptr());
                }
            } else if dflags & CDESC_SHORTDESC!() != 0 {
                let name = String::from("hashed");
                translation_fn(&name, command, full_path);
            } else if (dflags & (CDESC_REUSABLE!() | CDESC_PATH_ONLY!())) != 0 {
                unsafe {
                    println!("{}", CStr::from_ptr(full_path).to_str().unwrap());
                }
            }
            unsafe {
                libc::free(full_path as *mut c_void);
            }
            return 1;
        }
    }

    /* Now search through $PATH. */
    #[warn(while_true)]
    loop {
        if dflags & CDESC_STDPATH!() != 0 {
            /* command -p, all cannot be non-zero */
            unsafe {
                pathlist = conf_standard_path();
                full_path = find_in_path(command, pathlist, FS_EXEC_PREFERRED!() | FS_NODIRS!());
                libc::free(pathlist as *mut c_void);
            }
        /* Will only go through this once, since all == 0 if STDPATH set */
        } else if all == 0 {
            full_path = find_user_command(command);
        } else {
            full_path = user_command_matches(command, FS_EXEC_ONLY!(), found_file);
            /* XXX - should that be FS_EXEC_PREFERRED? */
        }
        if full_path == std::ptr::null_mut() {
            break;
        }

        /* If we found the command as itself by looking through $PATH, it
        probably doesn't exist.  Check whether or not the command is an
        executable file.  If it's not, don't report a match.  This is
        the default posix mode behavior */
        if (unsafe { STREQ!(full_path, command) } || unsafe { posixly_correct } != 0) {
            f = file_status(full_path);

            if f & FS_EXECABLE!() == 0 {
                unsafe {
                    libc::free(full_path as *mut c_void);
                    full_path = std::ptr::null_mut();
                }
                if all == 0 {
                    break;
                }
            } else if unsafe { ABSPATH!(full_path) } {
            }
            /* placeholder; don't need to do anything yet */
            else if dflags & (CDESC_REUSABLE!() | CDESC_PATH_ONLY!() | CDESC_SHORTDESC!()) != 0 {
                if MP_DOCWD!() != 0 | (dflags & CDESC_ABSPATH!()) {
                    f = MP_RMDOT!();
                } else {
                    f = 0;
                }
                unsafe {
                    x = c_sh_makepath(std::ptr::null_mut(), full_path, f);
                    libc::free(full_path as *mut c_void);
                }

                full_path = x;
            }
        }
        /* If we require a full path and don't have one, make one */
        else if ((dflags & CDESC_ABSPATH!()) != 0) && unsafe { ABSPATH!(full_path) } == false {
            unsafe {
                x = c_sh_makepath(std::ptr::null_mut(), full_path, MP_DOCWD!() | MP_RMDOT!());
                libc::free(full_path as *mut c_void);
            }
            full_path = x;
        }
        found_file += 1;
        found = 1;
        if dflags & CDESC_TYPE!() != 0 {
            unsafe {
                let c_str_file = CString::new("file").unwrap();
                libc::puts(c_str_file.as_ptr());
            }
        } else if dflags & CDESC_SHORTDESC!() != 0 {
            let name = String::from("is");
            translation_fn(&name, command, full_path);
        } else if dflags & (CDESC_REUSABLE!() | CDESC_PATH_ONLY!()) != 0 {
            unsafe {
                println!("{}", CStr::from_ptr(full_path).to_str().unwrap());
            }
        }

        unsafe {
            libc::free(full_path as *mut c_void);
        }
        // full_path = std::ptr::null_mut();
        if all == 0 {
            break;
        }
    }
    found
}

fn translation_fn(command: &String, args1: *mut libc::c_char, args2: *mut libc::c_char) {
    let mgr = ResourceManager::new("/usr/share/utshell/resources/{locale}/{res_id}".into());
    let resources = vec!["message.ftl".into()];
    let mut args = FluentArgs::new();
    if args1 != std::ptr::null_mut() {
        args.set("str1", unsafe {
            format!("{}", CStr::from_ptr(args1).to_str().unwrap())
        });
    }
    if args2 != std::ptr::null_mut() {
        args.set("str2", unsafe {
            format!("{}", CStr::from_ptr(args2).to_str().unwrap())
        });
    }
    let bundle = mgr.get_bundle(get_local_str(), resources);
    let value = bundle.get_message(command).unwrap();
    let pattern = value.value().expect("partern err");
    let mut errors = vec![];
    if args1 != std::ptr::null_mut() {
        let msg1 = bundle.format_pattern(&pattern, Some(&args), &mut errors);
        println!("{}", msg1.replace("\"", ""));
    } else {
        let msg1 = bundle.format_pattern(&pattern, None, &mut errors);
        println!("{}", msg1.replace("\"", ""));
    }
}
