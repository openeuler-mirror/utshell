use super::help::builtin_help;
use crate::array::array_flush;
use crate::arrayfunc::{bind_array_element, find_or_make_array_variable};
use crate::builtins::bashgetopt::{internal_getopt, reset_internal_getopt};
use crate::builtins::common::{builtin_usage, sh_invalidid};
use crate::builtins::evalstring::evalstring;
use crate::general::{legal_identifier, legal_number, sh_validfd};
use crate::src_common::*;

pub const DEFAULT_QUANTUM: libc::c_long = 5000;
pub const MAPF_CLEARARRAY: libc::c_int = 0x01;
pub const MAPF_CHOP: libc::c_int = 0x02;

static mut delim: libc::c_int = 0;

#[no_mangle]
pub fn mapfile_builtin(mut list: *mut WordList) -> i32 {
    let mut opt: libc::c_int;
    let mut code: libc::c_int;
    let mut fd: libc::c_int = 0;
    let mut flags: libc::c_int = MAPF_CLEARARRAY;
    let intval: libc::c_long = 0;
    let mut lines: libc::c_long = 0;
    let mut origin: libc::c_long = 0;
    let mut nskip: libc::c_long = 0;
    let mut callback_quantum: libc::c_long = DEFAULT_QUANTUM;

    let array_name: *mut libc::c_char;
    let mut callback: *mut libc::c_char = PT_NULL as *mut libc::c_char;

    unsafe {
        delim = b'\n' as libc::c_int;

        reset_internal_getopt();
        let opt_str = CString::new("d:u:n:O:tC:c:s:").unwrap();
        opt = internal_getopt(list, opt_str.as_ptr() as *mut libc::c_char);
        while opt != -1 {
            let opt_char: char = char::from(opt as u8);
            match opt_char {
                'd' => delim = *list_optarg as libc::c_int,
                'u' => {
                    code = legal_number(list_optarg, std::mem::transmute(&intval));
                    if code == 0 || intval < 0 || intval != (intval as libc::c_int) as libc::c_long
                    {
                        builtin_error(
                            "%s: invalid file descriptor specification\0".as_ptr()
                                as *const libc::c_char,
                            list_optarg,
                        );
                        return EXECUTION_FAILURE;
                    } else {
                        fd = intval as libc::c_int;
                    }
                    if sh_validfd(fd) == 0 {
                        builtin_error(
                            "%d: invalid file descriptor: %s\0".as_ptr() as *const libc::c_char,
                            fd,
                            libc::strerror(*libc::__errno_location()),
                        );
                        return EXECUTION_FAILURE;
                    }
                }
                'n' => {
                    code = legal_number(list_optarg, std::mem::transmute(&intval));
                    if code == 0 || intval < 0 || intval != (intval as libc::c_uint) as libc::c_long
                    {
                        builtin_error(
                            "%s: invalid line count\0".as_ptr() as *const libc::c_char,
                            list_optarg,
                        );
                        return EXECUTION_FAILURE;
                    } else {
                        lines = intval;
                    }
                }
                'O' => {
                    code = legal_number(list_optarg, std::mem::transmute(&intval));
                    if code == 0 || intval < 0 || intval != (intval as libc::c_uint) as libc::c_long
                    {
                        builtin_error(
                            "%s: invalid array origin\0".as_ptr() as *const libc::c_char,
                            list_optarg,
                        );
                        return EXECUTION_FAILURE;
                    } else {
                        origin = intval;
                    }
                    flags &= (MAPF_CLEARARRAY as libc::c_uint ^ 0xffffffff) as libc::c_int;
                }
                't' => flags |= MAPF_CHOP,
                'C' => callback = list_optarg,
                'c' => {
                    code = legal_number(list_optarg, std::mem::transmute(&intval));
                    if code == 0 || intval < 0 || intval != (intval as libc::c_uint) as libc::c_long
                    {
                        builtin_error(
                            "%s: invalid callback quantum\0".as_ptr() as *const libc::c_char,
                            list_optarg,
                        );
                        return EXECUTION_FAILURE;
                    } else {
                        callback_quantum = intval;
                    }
                }
                's' => {
                    code = legal_number(list_optarg, std::mem::transmute(&intval));
                    if code == 0 || intval < 0 || intval != (intval as libc::c_uint) as libc::c_long
                    {
                        builtin_error(
                            "%s: invalid line count\0".as_ptr() as *const libc::c_char,
                            list_optarg,
                        );
                        return EXECUTION_FAILURE;
                    } else {
                        nskip = intval;
                    }
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
            opt = internal_getopt(list, opt_str.as_ptr() as *mut libc::c_char);
        }
        list = loptend;

        if list.is_null() {
            array_name = "MAPFILE".as_ptr() as *mut libc::c_char;
        } else if (*list).word.is_null() || (*(*list).word).word.is_null() {
            builtin_error("internal error: getting variable name\0".as_ptr() as *const libc::c_char);
            return EXECUTION_FAILURE;
        } else if *(*(*list).word).word == b'\0' as libc::c_char {
            builtin_error("empty array variable name\0".as_ptr() as *const libc::c_char);
            return EX_USAGE;
        } else {
            array_name = (*(*list).word).word;
        }
        if legal_identifier(array_name) == 0 {
            sh_invalidid(array_name);
            return EXECUTION_FAILURE;
        }

        return mapfile(
            fd,
            lines,
            origin,
            nskip,
            callback_quantum,
            callback,
            array_name,
            delim,
            flags,
        );
    }
}

fn run_callback(
    callback: *const libc::c_char,
    curindex: libc::c_uint,
    curline: *mut libc::c_char,
) -> libc::c_int {
    unsafe {
        let qline = sh_single_quote(curline);
        let execlen = libc::strlen(callback) + libc::strlen(qline) + 10 + 3;
        let execstr = libc::malloc(execlen);

        let flags = SEVAL_NOHIST;

        libc::snprintf(
            execstr as *mut libc::c_char,
            execlen,
            "%s %d %s\0".as_ptr() as *const libc::c_char,
            callback,
            curindex,
            qline,
        );
        libc::free(qline as *mut c_void);
        return evalstring(
            execstr as *mut libc::c_char,
            PT_NULL as *const libc::c_char,
            flags,
        );
    }
}

fn do_chop(line: *mut libc::c_char, d: libc::c_uchar) {
    unsafe {
        let length = libc::strlen(line);
        if length != 0 && *((line as usize + length - 1) as *mut libc::c_char) == d as libc::c_char
        {
            *((line as usize + length - 1) as *mut libc::c_char) = b'\0' as libc::c_char;
        }
    }
}

fn mapfile(
    fd: libc::c_int,
    line_count_goal: libc::c_long,
    origin: libc::c_long,
    nskip: libc::c_long,
    callback_quantum: libc::c_long,
    callback: *mut libc::c_char,
    array_name: *mut libc::c_char,
    dlm: libc::c_int,
    flags: libc::c_int,
) -> libc::c_int {
    let mut line: *mut libc::c_char = PT_NULL as *mut libc::c_char;
    let mut line_length: size_t = 0;
    let mut unbuffered_read: libc::c_int;
    unsafe {
        let entry = find_or_make_array_variable(array_name, 1);
        // let entry_test = *entry;
        // println!("entry:{:#?}",entry_test);

        if entry.is_null()
            || ((*entry).attributes & att_readonly) != 0
            || ((*entry).attributes & att_noassign) != 0
        {
            if !entry.is_null() && ((*entry).attributes & att_readonly) != 0 {
                err_readonly(array_name);
            }

            return EXECUTION_FAILURE;
        } else if ((*entry).attributes & att_array) == 0 {
            builtin_error(
                "%s: not an indexed array\0".as_ptr() as *const libc::c_char,
                array_name,
            );
            return EXECUTION_FAILURE;
        } else if ((*entry).attributes & att_array) != 0 {
            (*entry).attributes &= (att_invisible as libc::c_uint ^ 0xffffffff) as libc::c_int;
        }

        if (flags & MAPF_CLEARARRAY) != 0 {
            array_flush(std::mem::transmute((*entry).value));
        }
        unbuffered_read = ((libc::lseek(fd, 0, SEEK_CUR) < 0)
            && (*libc::__errno_location() == ESPIPE)) as libc::c_int;

        if dlm != b'\n' as libc::c_int {
            unbuffered_read = 1;
        }
        zreset();

        let mut line_count: libc::c_uint = 0;
        while (line_count as libc::c_long) < nskip {
            if zgetline(
                fd,
                std::mem::transmute(&line),
                std::mem::transmute(&line_length),
                dlm,
                unbuffered_read,
            ) < 0
            {
                break;
            }
            line_count += 1;
        }

        line = PT_NULL as *mut libc::c_char;
        line_length = 0;
        let mut array_index: libc::c_uint = origin as libc::c_uint;
        line_count = 1;
        while zgetline(
            fd,
            std::mem::transmute(&line),
            std::mem::transmute(&line_length),
            dlm,
            unbuffered_read,
        ) != -1
        {
            if (flags & MAPF_CHOP) != 0 {
                do_chop(line, dlm as libc::c_uchar);
            }

            if !callback.is_null()
                && line_count != 0
                && (line_count as libc::c_long % callback_quantum) == 0
            {
                run_callback(callback, array_index, line);

                if unbuffered_read == 0 {
                    zsyncfd(fd);
                }
            }

            bind_array_element(entry, array_index as libc::c_long, line, 0);

            line_count += 1;
            if line_count_goal != 0 && (line_count as libc::c_long) > line_count_goal {
                break;
            }

            array_index += 1;
        }

        libc::free(line as *mut c_void);

        if unbuffered_read == 0 {
            zsyncfd(fd);
        }
    }
    return EXECUTION_SUCCESS;
}
