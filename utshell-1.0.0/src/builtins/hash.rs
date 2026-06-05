use crate::alias::all_aliases;
use crate::builtins::bashgetopt::{internal_getopt, reset_internal_getopt};
use crate::builtins::common::{
    builtin_usage, find_shell_builtin, sh_needarg, sh_notfound, sh_restricted,
};
use crate::findcmd::{executable_file, find_user_command, is_directory};
use crate::general::{absolute_program, printable_filename};
use crate::hashcmd::{phash_flush, phash_insert, phash_remove, phash_search};
use crate::hashlib::hash_walk;
use crate::src_common::*;
use crate::variables::find_function;
use crate::version::shell_compatibility_level;

static mut common_inode: libc::c_int = 0;

/* Print statistics on the current state of hashed commands.  If LIST is
not empty, then rehash (or hash in the first place) the specified
commands. */
#[no_mangle]
pub fn hash_builtin(mut list: *mut WordList) -> i32 {
    let mut expunge_hash_table: i32;
    let mut list_targets: i32;
    let mut list_portably: i32;
    let mut delete: i32;
    let mut opt: i32;
    let mut w: *mut libc::c_char;
    let mut pathname: *mut libc::c_char;

    if unsafe { hashing_enabled == 0 } {
        let c_str = CString::new("hashing disabled").unwrap();
        let c_str_ptr = c_str.as_ptr();
        unsafe { builtin_error(c_str_ptr) };
        return EXECUTION_FAILURE!();
    }
    expunge_hash_table = 0;
    list_targets = 0;
    list_portably = 0;
    delete = 0;
    pathname = std::ptr::null_mut();
    reset_internal_getopt();
    let opts = CString::new("dlp:rt").unwrap();
    opt = internal_getopt(list, opts.as_ptr() as *mut libc::c_char);
    while opt != -1 {
        let optu8 = opt as u8;
        let opt_char = char::from(optu8);
        match opt_char {
            'd' => delete = 1,
            'l' => list_portably = 1,
            'p' => pathname = unsafe { list_optarg },
            'r' => expunge_hash_table = 1,
            't' => list_targets = 1,
            _ => {
                if opt == -99 {
                    // 替换 builtin_help(); 为直接输出，确保所有架构一致
                    println!("hash: hash [-lr] [-p pathname] [-dt] [name ...]");
                    println!("    Remember or display program locations.");
                    println!();
                    println!(
                        "    Determine and remember the full pathname of each command NAME.  If"
                    );
                    println!("    no arguments are given, information about remembered commands is displayed.");
                    println!();
                    println!("    Options:");
                    println!("      -d        forget the remembered location of each NAME");
                    println!("      -l        display in a format that may be reused as input");
                    println!("      -p pathname       use PATHNAME as the full pathname of NAME");
                    println!("      -r        forget all remembered locations");
                    println!(
                        "      -t        print the remembered location of each NAME, preceding"
                    );
                    println!(
                        "                each location with the corresponding NAME if multiple"
                    );
                    println!("                NAMEs are given");
                    println!("    Arguments:");
                    println!(
                        "      NAME      Each NAME is searched for in $PATH and added to the list"
                    );
                    println!("                of remembered commands.");
                    println!();
                    println!("    Exit Status:");
                    println!("    Returns success unless NAME is not found or an invalid option is given.");
                    return EX_USAGE;
                }
                builtin_usage();
                return EX_USAGE;
            }
        }
        opt = internal_getopt(list, opts.as_ptr() as *mut libc::c_char);
    }
    list = unsafe { loptend };
    /* hash -t requires at least one argument. */
    if list == std::ptr::null_mut() && (delete != 0 || list_targets != 0) {
        let temp: CString;
        let temp_ptr: *mut libc::c_char;
        if delete != 0 {
            temp = CString::new("-d").unwrap();
            temp_ptr = temp.as_ptr() as *mut libc::c_char;
            sh_needarg(temp_ptr);
        } else {
            temp = CString::new("-t").unwrap();
            temp_ptr = temp.as_ptr() as *mut libc::c_char;
            sh_needarg(temp_ptr);
        }
        return EXECUTION_FAILURE!();
    }

    /* We want hash -r to be silent, but hash -- to print hashing info, so
    we test expunge_hash_table. */
    if list == std::ptr::null_mut() && expunge_hash_table == 0 {
        opt = print_hashed_commands(list_portably);
        if unsafe {
            opt == 0
                && posixly_correct == 0
                && (list_portably == 0 || shell_compatibility_level <= 50)
        } {
            let s_cstr = unsafe { CStr::from_ptr(this_command_name) };
            let s_str = s_cstr.to_str().unwrap();
            let s_string = s_str.to_owned();
            println!("{}:hash table empty", s_string);
        }
        return EXECUTION_SUCCESS!();
    }
    if expunge_hash_table != 0 {
        phash_flush();
    }
    /* If someone runs `hash -r -t xyz' he will be disappointed. */
    if list_targets != 0 {
        return list_hashed_filename_targets(list, list_portably);
    }
    if unsafe { restricted != 0 && pathname != std::ptr::null_mut() } {
        if unsafe { strchr(pathname, '/' as libc::c_int) != std::ptr::null_mut() } {
            sh_restricted(pathname);
            return EXECUTION_FAILURE!();
        }
        /* If we are changing the hash table in a restricted shell, make sure the
        target pathname can be found using a $PATH search. */
        w = find_user_command(pathname);
        if unsafe { w == std::ptr::null_mut() || *w == 0 || executable_file(w) == 0 } {
            sh_notfound(pathname);
            unsafe {
                free(w as *mut c_void);
            }
            return EXECUTION_FAILURE!();
        }
        unsafe {
            free(w as *mut c_void);
        }
    }
    opt = EXECUTION_SUCCESS!();
    while list != std::ptr::null_mut() {
        /* Add, remove or rehash the specified commands. */
        unsafe {
            w = (*(*list).word).word;
        }
        if absolute_program(w as *const libc::c_char) != 0 {
            continue;
        } else if pathname != std::ptr::null_mut() {
            if is_directory(pathname) != 0 {
                let c_err = CString::new("%s:%s").unwrap();
                let c_err_ptr = c_err.as_ptr();
                unsafe {
                    builtin_error(c_err_ptr, pathname, strerror(EISDIR));
                }
                opt = EXECUTION_SUCCESS!();
            } else {
                if legal_hash_rust(w, pathname) == 0 {
                    phash_insert(w, pathname, 0, 0);
                }
            }
        } else if delete != 0 {
            if phash_remove(w) != 0 {
                sh_notfound(w);
                opt = EXECUTION_FAILURE!();
            }
        } else if add_hashed_command(w, 0) != 0 {
            opt = EXECUTION_FAILURE!();
        }
        list = unsafe { (*list).next };
    }
    let _ = std::io::stdout().flush();
    return opt;
}
fn add_hashed_command(w: *mut libc::c_char, quiet: i32) -> i32 {
    let mut rv: i32;
    let full_path: *mut libc::c_char;
    rv = 0;

    if find_function(w).is_null() && find_shell_builtin(w).is_none() {
        phash_remove(w);
        full_path = find_user_command(w);
        if full_path != std::ptr::null_mut() && executable_file(full_path) != 0 {
            unsafe {
                phash_insert(w, full_path, dot_found_in_search, 0);
            }
        } else {
            if quiet == 0 {
                sh_notfound(w);
            }
            rv += 1;
        }
        unsafe { FREE!(full_path) };
    }
    return rv;
}
fn print_hash_info(item: *mut BUCKET_CONTENTS) -> i32 {
    unsafe {
        let path_string = CStr::from_ptr((*pathdata!(item)).path).to_str().unwrap();
        println!("{:04}\t{}", (*item).times_found, path_string);
    }
    0
}
#[no_mangle]
fn print_portable_hash_info(item: *mut BUCKET_CONTENTS) -> i32 {
    let fp: *mut libc::c_char;
    let f: *mut libc::c_char;
    unsafe {
        fp = printable_filename((*pathdata!(item)).path, 1);
        f = printable_filename((*item).key, 1);
        let fp_string = CStr::from_ptr(fp).to_str().unwrap();
        let f_string = CStr::from_ptr(f).to_str().unwrap();
        println!("builtin hash -p {} {}", fp_string, f_string);
        if fp != (*pathdata!(item)).path {
            free(fp as *mut c_void);
        }
        if f != (*item).key {
            free(f as *mut c_void);
        }
        return 0;
    }
}
#[no_mangle]
fn print_hashed_commands(fmt: i32) -> i32 {
    if unsafe { hashed_filenames.is_null() || hash_entries(hashed_filenames) == 0 } {
        return 0;
    }
    if fmt == 0 {
        println!("hits\tcommand");
    }
    let fmt_t: hash_wfunc;
    if fmt != 0 {
        fmt_t = print_portable_hash_info;
    } else {
        fmt_t = print_hash_info;
    }
    unsafe { hash_walk(hashed_filenames, Some(fmt_t)) };
    return 1;
}
#[no_mangle]
fn list_hashed_filename_targets(list: *mut WordList, fmt: i32) -> i32 {
    let mut all_found: i32;
    let multiple: i32;
    let mut target: *mut libc::c_char;
    let mut l: *mut WordList;
    all_found = 1;

    if unsafe { !(*list).next.is_null() } {
        multiple = 1;
    } else {
        multiple = 0;
    }
    l = list;
    while !l.is_null() {
        target = unsafe { phash_search((*(*l).word).word) };
        if target.is_null() {
            all_found = 0;
            unsafe {
                sh_notfound((*(*l).word).word);
            }
            continue;
        }
        if fmt != 0 {
            unsafe {
                let target_string = CStr::from_ptr(target).to_str().unwrap();
                let c_str = CStr::from_ptr((*(*l).word).word).to_str().unwrap();
                println!("builtin hash -p {} {}", target_string, c_str)
            }
        } else {
            if multiple != 0 {
                unsafe {
                    let c_str = CStr::from_ptr((*(*l).word).word).to_str().unwrap();
                    print!("{}\t", c_str);
                }
            }
            unsafe {
                let target_str = CStr::from_ptr(target).to_str().unwrap();
                println!("{}", target_str);
            }
        }
        unsafe {
            free(target as *mut c_void);
            l = (*l).next;
        }
    }

    if all_found != 0 {
        return EXECUTION_SUCCESS!();
    } else {
        return EXECUTION_FAILURE!();
    }
}
fn legal_hash_rust(name: *mut libc::c_char, value: *mut libc::c_char) -> libc::c_int {
    let alias_list: *mut *mut AliasT = all_aliases();
    let mut t: *mut AliasT;
    let mut offset;
    let name_w: *mut libc::c_char;
    let target: *mut libc::c_char;
    offset = 0;
    if !alias_list.is_null() {
        unsafe {
            t = *alias_list.offset(offset as isize);
            while !t.is_null() {
                if !(*t).name.is_null() {
                    if libc::strcmp(name, (*t).name) == 0 {
                        println!("Prohibit setting existing variables that is already in alias");
                        println!(
                            "{} = {}",
                            CStr::from_ptr((*t).name).to_string_lossy().into_owned(),
                            CStr::from_ptr((*t).value).to_string_lossy().into_owned()
                        );
                        return 1;
                    }
                }
                offset += 1;
                t = *alias_list.offset(offset as isize);
            }
        }
    }

    unsafe {
        if find_shell_builtin(name).is_some() {
            //不是空
            println!(
                "Prohibit setting existing variables {} is a shell builtin",
                CStr::from_ptr(name).to_string_lossy().into_owned()
            );
            return 1;
        } else if find_function(name) != std::ptr::null_mut() {
            println!(
                "Prohibit setting existing variables {} is a function",
                CStr::from_ptr(name).to_string_lossy().into_owned()
            );
            return 1;
        }
    }
    name_w = find_user_command(name);
    if name_w != std::ptr::null_mut() {
        unsafe {
            let _ = file_inode(
                CStr::from_ptr(name_w).to_str().unwrap(),
                CStr::from_ptr(value).to_str().unwrap(),
            );
            if common_inode == 1 {
                return 1;
            }
        }
    }
    target = phash_search(name);
    unsafe {
        if target != std::ptr::null_mut() {
            println!(
                "{} is already in hash",
                CStr::from_ptr(name).to_string_lossy().into_owned()
            );
            return 1;
        }
    }
    return 0;
}

fn file_inode(pathname: &str, pathname2: &str) -> std::io::Result<()> {
    let meta = std::fs::metadata(pathname)?;
    let meta2 = std::fs::metadata(pathname2)?;
    unsafe {
        common_inode = 0;
        if meta.st_ino() != meta2.st_ino() {
            println!("The name and value point to different executable files");
            common_inode = 1;
        }
        Ok(())
    }
}
