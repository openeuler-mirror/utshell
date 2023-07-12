/* automatically generated by rust-bindgen */

pub const __EXTENSIONS__: u32 = 1;
pub const _ALL_SOURCE: u32 = 1;
pub const _GNU_SOURCE: u32 = 1;
pub const _POSIX_PTHREAD_SEMANTICS: u32 = 1;
pub const _TANDEM_SOURCE: u32 = 1;
pub const JOB_CONTROL: u32 = 1;
pub const ALIAS: u32 = 1;
pub const PUSHD_AND_POPD: u32 = 1;
pub const BRACE_EXPANSION: u32 = 1;
pub const READLINE: u32 = 1;
pub const BANG_HISTORY: u32 = 1;
pub const HISTORY: u32 = 1;
pub const HELP_BUILTIN: u32 = 1;
pub const RESTRICTED_SHELL: u32 = 1;
pub const PROCESS_SUBSTITUTION: u32 = 1;
pub const PROMPT_STRING_DECODE: u32 = 1;
pub const SELECT_COMMAND: u32 = 1;
pub const COMMAND_TIMING: u32 = 1;
pub const ARRAY_VARS: u32 = 1;
pub const DPAREN_ARITHMETIC: u32 = 1;
pub const EXTENDED_GLOB: u32 = 1;
pub const EXTGLOB_DEFAULT: u32 = 0;
pub const COND_COMMAND: u32 = 1;
pub const COND_REGEXP: u32 = 1;
pub const COPROCESS_SUPPORT: u32 = 1;
pub const ARITH_FOR_COMMAND: u32 = 1;
pub const NETWORK_REDIRECTIONS: u32 = 1;
pub const PROGRAMMABLE_COMPLETION: u32 = 1;
pub const DEBUGGER: u32 = 1;
pub const MEMSCRAMBLE: u32 = 1;
pub const CASEMOD_ATTRS: u32 = 1;
pub const CASEMOD_EXPANSIONS: u32 = 1;
pub const GLOBASCII_DEFAULT: u32 = 1;
pub const FUNCTION_IMPORT: u32 = 1;
pub const ENABLE_NLS: u32 = 1;
pub const DEFAULT_PATH_VALUE: &'static [u8; 63usize] =
    b"/usr/local/bin:/usr/local/sbin:/usr/bin:/usr/sbin:/bin:/sbin:.\0";
pub const STANDARD_UTILS_PATH: &'static [u8; 30usize] = b"/bin:/usr/bin:/usr/sbin:/sbin\0";
pub const PPROMPT: &'static [u8; 11usize] = b"\\s-\\\\v\\\\$ \0";
#[no_mangle]
pub extern "C" fn r_history_glob(mut list: *mut WordList) -> i32 {

    let mut flags: c_int = 0;
    let mut opt: c_int;
    let mut result: c_int;

    let filename: *mut c_char;
    let mut delete_arg: *mut c_char = PT_NULL as *mut c_char;
    let mut range: *mut c_char;

    let mut delete_offset: c_long = 0;

unsafe {
    reset_internal_getopt();
    let opt_str = CString::new("acd:npsrw").unwrap();
    opt = internal_getopt (list, opt_str.as_ptr() as * mut c_char);
    while  opt != -1 {
        let opt_char:char=char::from(opt as u8);
        match opt_char {
            'a' => flags |= AFLAG,
            'c' => flags |= CFLAG,
            'n' => flags |= NFLAG,
            'r' => flags |= RFLAG,
            'w' => flags |= WFLAG,
            's' => flags |= SFLAG,
            'd' => {
                flags |= DFLAG;
                delete_arg = list_optarg;
            }
            'p' => flags |= PFLAG,
            _ => {
                if opt == -99 {
                    r_builtin_help();
                    return EX_USAGE;
                }
            r_builtin_usage ();
            return EX_USAGE;
            }
        }
        opt = internal_getopt (list, opt_str.as_ptr() as * mut c_char);
    }
    list = loptend;

    opt = flags & (AFLAG | RFLAG | WFLAG | NFLAG);
    if opt != 0 && opt != AFLAG && opt != RFLAG && opt != WFLAG && opt != NFLAG {
        let c_err = CString::new("cannot use more than one of -anrw").unwrap();
        builtin_error( c_err.as_ptr());
        return EXECUTION_FAILURE;
    }

    if (flags & CFLAG) != 0 {
        bash_clear_history();
        if list.is_null() {
            return EXECUTION_SUCCESS;
        }
    }

    if (flags & SFLAG) != 0 {
        if !list.is_null() {
            push_history(list);
        }
        return EXECUTION_SUCCESS;
    }
    else if (flags & PFLAG) != 0 {
        if !list.is_null() {
            return expand_and_print_history(list);
        }
        return r_sh_chkwrite(EXECUTION_SUCCESS);
    } 

    if (flags & DFLAG) != 0 {
        let c_tmp = if *delete_arg == b'-' as c_char {delete_arg.offset(1 as isize ) as *mut c_char} else {delete_arg};
        range = libc::strchr(c_tmp, b'-' as c_int);
        if  !range.is_null() {
            let mut delete_start: c_long = 0;
            let mut delete_end: c_long = 0;

        *range = b'\0' as c_char;
        range = (range as usize + 1) as *mut c_char;
        if legal_number(delete_arg, std::mem::transmute(&delete_start)) == 0 ||
        legal_number(range, std::mem::transmute(&delete_end)) == 0 {
            *((range as usize - 1) as *mut c_char) = b'-' as c_char;
            r_sh_erange(delete_arg, "history position\0".as_ptr() as *mut c_char);
            return EXECUTION_FAILURE;
        }
        if *delete_arg == b'-' as c_char && delete_start < 0 {
            delete_start += history_length as c_long;
            if delete_start < history_base as c_long {
                r_sh_erange(delete_arg, "history position\0".as_ptr() as *mut c_char);
                return EXECUTION_FAILURE;
            }
        } else if delete_start > 0 {
            delete_start -= history_base as c_long;
        }
        if delete_start < 0 || delete_start >= history_length as c_long {
            r_sh_erange(delete_arg, "history position\0".as_ptr() as *mut c_char);
            return EXECUTION_FAILURE;
        }
        if *range == b'-' as c_char && delete_end < 0 {
            delete_end += history_length as c_long;
            if delete_end < history_base as c_long {
                r_sh_erange(range, "history position\0".as_ptr() as *mut c_char);
                return EXECUTION_FAILURE;
            }
        } else if delete_end > 0 {
            delete_end -= history_base as c_long;
        }

        if delete_end < 0 || delete_end >= history_length as c_long {
            r_sh_erange(range, "history position\0".as_ptr() as *mut c_char);
            return EXECUTION_FAILURE;
        }
        result = bash_delete_history_range(delete_start as c_int, delete_end as c_int);
        if where_history() > history_length {
            history_set_pos(history_length);
        }

        return if result != 0 {EXECUTION_SUCCESS} else {EXECUTION_FAILURE};
    }

    return if result != 0 {EXECUTION_FAILURE} else {EXECUTION_SUCCESS};
}

fn histtime(hlist: *mut HIST_ENTRY, histtimefmt: *const c_char) -> *mut c_char
{
unsafe {
    static mut timestr: [c_char;128] = [0;128];

    let t = history_get_time(hlist);
    let tm = if t != 0 {libc::localtime(&t)} else {PT_NULL as *mut libc::tm};
    if t != 0 && !tm.is_null() {
        strftime(std::mem::transmute(&timestr),
        std::mem::size_of_val(&timestr),
        histtimefmt,
        tm);
    } else if !(*hlist).timestamp.is_null() && (*(*hlist).timestamp) != 0 {
        let c_str = CString::new("%s: invalid timestamp").unwrap();
        libc::snprintf(std::mem::transmute(&timestr),
        std::mem::size_of_val(&timestr), c_str.as_ptr(),
        if *((*hlist).timestamp) == b'#' as c_char {((*hlist).timestamp as usize + 1) as *mut c_char} else {(*hlist).timestamp});
    } else {
        libc::strcpy(std::mem::transmute(&timestr), b"??\0".as_ptr() as *const c_char);
    }

        if delete_end < 0 || delete_end >= history_length as c_long {
            r_sh_erange(range, "history position\0".as_ptr() as *mut c_char);
            return EXECUTION_FAILURE;
        }
        result = bash_delete_history_range(delete_start as c_int, delete_end as c_int);
        if where_history() > history_length {
            history_set_pos(history_length);
        }
    return timestr.as_mut_ptr();
}
}
