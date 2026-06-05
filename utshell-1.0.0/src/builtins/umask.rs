use crate::src_common::*;

use crate::builtins::bashgetopt::{internal_getopt, reset_internal_getopt};
use crate::builtins::common::{builtin_usage, read_octal, sh_chkwrite, sh_erange};
use crate::builtins::help::builtin_help;
use std::ffi::CString;

//C库
extern "C" {
    fn umask(__mask: __mode_t) -> __mode_t;
}

fn c_umask(__mask: __mode_t) -> __mode_t {
    unsafe { umask(__mask) }
}

fn member(c: *mut libc::c_char, s: *mut libc::c_char) -> bool {
    if c != std::ptr::null_mut() {
        let c = c as libc::c_int;
        let ptr = unsafe { libc::strchr(s, c) };
        if ptr != std::ptr::null_mut() {
            true
        } else {
            false
        }
    } else {
        false
    }
}

#[no_mangle]
/* Set or display the mask used by the system when creating files.  Flag
of -S means display the umask in a symbolic mode. */

pub fn umask_builtin(mut list: *mut WordList) -> i32 {
    let mut print_symbolically: i32;
    let mut opt: i32;
    let umask_value: i32;
    let mut pflag: i32;
    let umask_arg: mode_t!();

    print_symbolically = 0;
    pflag = 0;
    reset_internal_getopt();

    let c_str_sp = CString::new("Sp").unwrap();
    opt = internal_getopt(list, c_str_sp.as_ptr() as *mut libc::c_char);
    while opt != -1 {
        let optu8 = opt as u8;
        let opt_char = char::from(optu8);
        match opt_char {
            'S' => {
                print_symbolically = print_symbolically + 1;
            }
            'p' => {
                pflag = pflag + 1;
            }
            _ => {
                if opt == -99 {
                    builtin_help();
                    return EX_USAGE as libc::c_int;
                }
                builtin_usage();
                return EX_USAGE as libc::c_int;
            }
        }

        opt = internal_getopt(list, c_str_sp.as_ptr() as *mut libc::c_char);
    }

    list = unsafe { loptend };
    if list != std::ptr::null_mut() {
        if unsafe { DIGIT!(*(*(*list).word).word) } != false {
            umask_value = read_octal((unsafe { *(*list).word }).word);
            /* Note that other shells just let you set the umask to zero
            by specifying a number out of range.  This is a problem
            with those shells.  We don't change the umask if the input
            is lousy. */
            if umask_value == -1 {
                let c_str = CString::new("octal number").unwrap();
                let c_char_str: *mut libc::c_char = c_str.into_raw();
                sh_erange((unsafe { *(*list).word }).word, c_char_str);
                return EXECUTION_FAILURE!();
            }
        } else {
            umask_value = symbolic_umask(list);
            if umask_value == -1 {
                return EXECUTION_FAILURE!();
            }
        }
        umask_arg = umask_value as u32;
        c_umask(umask_arg as __mode_t);
        if print_symbolically != 0 {
            print_symbolic_umask(umask_arg);
        }
    } else {
        /* Display the UMASK for this user. */
        umask_arg = c_umask(0o22 as libc::c_uint);
        c_umask(umask_arg as __mode_t);
        if pflag != 0 {
            if print_symbolically != 0 {
                println!("umask  -S");
            } else {
                print!("umask ")
            }
        }

        if print_symbolically != 0 {
            print_symbolic_umask(umask_arg);
        } else {
            println!("{:04o}", umask_arg);
        }
    }
    return sh_chkwrite(EXECUTION_SUCCESS!());
}

#[no_mangle]
/* Print the umask in a symbolic form.  In the output, a letter is
printed if the corresponding bit is clear in the umask. */

fn print_symbolic_umask(um: mode_t!()) {
    /* u=rwx,g=rwx,o=rwx */
    let mut ubits = String::new();
    let mut gbits = String::new();
    let mut obits = String::new();

    if um & S_IRUSR!() == 0 {
        ubits.push('r');
    }
    if um & S_IWUSR!() == 0 {
        ubits.push('w');
    }
    if um & S_IXUSR!() == 0 {
        ubits.push('x');
    }

    if um & S_IRGRP!() == 0 {
        gbits.push('r');
    }
    if um & S_IWGRP!() == 0 {
        gbits.push('w');
    }
    if um & S_IXGRP!() == 0 {
        gbits.push('x');
    }

    if um & S_IROTH!() == 0 {
        obits.push('r');
    }
    if um & S_IWOTH!() == 0 {
        obits.push('w');
    }
    if um & S_IXOTH!() == 0 {
        obits.push('x');
    }

    println! {"u={},g={},o={}",ubits,gbits,obits};
}

#[no_mangle]
fn parse_symbolic_mode(mode: *mut libc::c_char, initial_bits: i32) -> i32 {
    let mut who: i32;
    let mut op: i32;
    let mut perm: i32;
    let mut bits: i32;
    let mut c: i32;
    let mut s: *mut libc::c_char;

    s = mode;
    bits = initial_bits;

    loop {
        who = 0;
        // op = 0;
        perm = 0;

        /* Parse the `who' portion of the symbolic mode clause. */
        let c_str = CString::new("agou").unwrap();
        while member(s, c_str.as_ptr() as *mut libc::c_char) {
            c = unsafe { *s } as libc::c_int;
            s = (s as usize + 1) as *mut libc::c_char;
            let optu8 = c as u8;
            let opt_char = char::from(optu8);
            match opt_char {
                'u' => {
                    who |= S_IRWXU!();
                    continue;
                }
                'g' => {
                    who |= S_IRWXG!();
                    continue;
                }
                'o' => {
                    who |= S_IRWXO!();
                    continue;
                }
                'a' => {
                    who |= S_IRWXU!() | S_IRWXG!() | S_IRWXO!();
                    continue;
                }
                _ => {}
            }
        }

        /* The operation is now sitting in *s. */
        op = unsafe { *s } as libc::c_int;
        s = (s as usize + 1) as *mut libc::c_char;
        let opu8 = op as u8;
        let op_str = char::from(opu8);
        match op_str {
            '+' | '-' | '=' => {}
            _ => {
                println!("{}:invalid symbolic mode operator", op_str);
                return -1;
            }
        }

        /* Parse out the `perm' section of the symbolic mode clause. */
        let c_rwx_str = CString::new("rwx").unwrap();
        while member(s, c_rwx_str.as_ptr() as *mut libc::c_char) {
            c = s as libc::c_int;
            s = (s as usize + 1) as *mut libc::c_char;
            let optu8 = c as u8;
            let op_str = char::from(optu8);

            match op_str {
                'r' => perm |= S_IRUGO!(),
                'w' => perm |= S_IWUGO!(),
                'x' => perm |= S_IXUGO!(),
                _ => {}
            }
        }

        /* Now perform the operation or return an error for a
        bad permission string. */
        if unsafe { *s } != 0 || unsafe { *s } == ',' as libc::c_char {
            if who != 0 {
                perm &= who;
            }

            match op_str {
                '+' => bits |= perm,
                '-' => bits &= !perm,
                '=' => {
                    if who == 0 {
                        who = S_IRWXU!() | S_IRWXG!() | S_IRWXO!();
                        bits &= !who;
                        bits |= perm;
                    }
                }
                /* No other values are possible. */
                _ => {}
            }
            if unsafe { *s } == '\0' as libc::c_char {
                break;
            } else {
                s = (s as usize + 1) as *mut libc::c_char;
            }
        } else {
            println!("{}:invalid symbolic mode character", unsafe { *s }
                as libc::c_char);
            return -1;
        }
    } //loop
    return bits;
}

#[no_mangle]
/* Set the umask from a symbolic mode string similar to that accepted
by chmod.  If the -S argument is given, then print the umask in a
symbolic form. */

fn symbolic_umask(list: *mut WordList) -> i32 {
    let mut um: i32;
    let bits: i32;

    /* Get the initial umask.  Don't change it yet. */
    um = c_umask(0o22 as libc::c_uint) as i32;
    c_umask(um as __mode_t);

    /* All work is done with the complement of the umask -- it's
    more intuitive and easier to deal with.  It is complemented
    again before being returned. */
    bits = parse_symbolic_mode((unsafe { *(*list).word }).word, !um & 0777);
    if bits == -1 {
        return -1;
    }

    um = !bits & 0o777;
    return um;
}
