use crate::builtins::bashgetopt::{internal_getopt, reset_internal_getopt};
use crate::builtins::common::{builtin_usage, sh_chkwrite, sh_erange, sh_invalidnum};
use crate::builtins::help::builtin_help;
use crate::general::{all_digits, string_to_rlimtype};
use crate::src_common::*;
use std::ffi::{CStr, CString};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RESOURCE_LIMITS {
    pub option: i32,                      /* The ulimit option for this limit. */
    pub parameter: i32,                   /* Parameter to pass to get_limit (). */
    pub block_factor: i32,                /* Blocking factor for specific limit. */
    pub description: *const libc::c_char, /* Descriptive string to output. */
    pub units: *const libc::c_char,       /* scale */
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _cmd {
    pub cmd: i32,
    pub arg: *mut libc::c_char,
}
pub type ULCMD = _cmd;
pub type RLIMTYPE = i64;
pub type RESOURCE_LIMITS_T = RESOURCE_LIMITS;
static mut cmdlistsz: i32 = 0;
const limits: [RESOURCE_LIMITS_T; 18] = [
    {
        RESOURCE_LIMITS {
            option: 'R' as i32,
            parameter: __RLIMIT_RTTIME as i32,
            block_factor: 1 as i32,
            description: b"real-time non-blocking time\0" as *const u8 as *const libc::c_char,
            units: b"microseconds\0" as *const u8 as *const libc::c_char,
        }
    },
    {
        RESOURCE_LIMITS {
            option: 'c' as i32,
            parameter: RLIMIT_CORE as i32,
            block_factor: -(2 as i32),
            description: b"core file size\0" as *const u8 as *const libc::c_char,
            units: b"blocks\0" as *const u8 as *const libc::c_char,
        }
    },
    {
        RESOURCE_LIMITS {
            option: 'd' as i32,
            parameter: RLIMIT_DATA as i32,
            block_factor: 1024 as i32,
            description: b"data seg size\0" as *const u8 as *const libc::c_char,
            units: b"kbytes\0" as *const u8 as *const libc::c_char,
        }
    },
    {
        RESOURCE_LIMITS {
            option: 'e' as i32,
            parameter: __RLIMIT_NICE as i32,
            block_factor: 1 as i32,
            description: b"scheduling priority\0" as *const u8 as *const libc::c_char,
            units: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
        }
    },
    {
        RESOURCE_LIMITS {
            option: 'f' as i32,
            parameter: RLIMIT_FSIZE as i32,
            block_factor: -(2 as i32),
            description: b"file size\0" as *const u8 as *const libc::c_char,
            units: b"blocks\0" as *const u8 as *const libc::c_char,
        }
    },
    {
        RESOURCE_LIMITS {
            option: 'i' as i32,
            parameter: __RLIMIT_SIGPENDING as i32,
            block_factor: 1 as i32,
            description: b"pending signals\0" as *const u8 as *const libc::c_char,
            units: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
        }
    },
    {
        RESOURCE_LIMITS {
            option: 'l' as i32,
            parameter: __RLIMIT_MEMLOCK as i32,
            block_factor: 1024 as i32,
            description: b"max locked memory\0" as *const u8 as *const libc::c_char,
            units: b"kbytes\0" as *const u8 as *const libc::c_char,
        }
    },
    {
        RESOURCE_LIMITS {
            option: 'm' as i32,
            parameter: __RLIMIT_RSS as i32,
            block_factor: 1024 as i32,
            description: b"max memory size\0" as *const u8 as *const libc::c_char,
            units: b"kbytes\0" as *const u8 as *const libc::c_char,
        }
    },
    {
        RESOURCE_LIMITS {
            option: 'n' as i32,
            parameter: RLIMIT_NOFILE as i32,
            block_factor: 1 as i32,
            description: b"open files\0" as *const u8 as *const libc::c_char,
            units: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
        }
    },
    {
        RESOURCE_LIMITS {
            option: 'p' as i32,
            parameter: 257 as i32,
            block_factor: 512 as i32,
            description: b"pipe size\0" as *const u8 as *const libc::c_char,
            units: b"512 bytes\0" as *const u8 as *const libc::c_char,
        }
    },
    {
        RESOURCE_LIMITS {
            option: 'q' as i32,
            parameter: __RLIMIT_MSGQUEUE as i32,
            block_factor: 1 as i32,
            description: b"POSIX message queues\0" as *const u8 as *const libc::c_char,
            units: b"bytes\0" as *const u8 as *const libc::c_char,
        }
    },
    {
        RESOURCE_LIMITS {
            option: 'r' as i32,
            parameter: __RLIMIT_RTPRIO as i32,
            block_factor: 1 as i32,
            description: b"real-time priority\0" as *const u8 as *const libc::c_char,
            units: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
        }
    },
    {
        RESOURCE_LIMITS {
            option: 's' as i32,
            parameter: RLIMIT_STACK as i32,
            block_factor: 1024 as i32,
            description: b"stack size\0" as *const u8 as *const libc::c_char,
            units: b"kbytes\0" as *const u8 as *const libc::c_char,
        }
    },
    {
        RESOURCE_LIMITS {
            option: 't' as i32,
            parameter: RLIMIT_CPU as i32,
            block_factor: 1 as i32,
            description: b"cpu time\0" as *const u8 as *const libc::c_char,
            units: b"seconds\0" as *const u8 as *const libc::c_char,
        }
    },
    {
        RESOURCE_LIMITS {
            option: 'u' as i32,
            parameter: __RLIMIT_NPROC as i32,
            block_factor: 1 as i32,
            description: b"max user processes\0" as *const u8 as *const libc::c_char,
            units: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
        }
    },
    {
        RESOURCE_LIMITS {
            option: 'v' as i32,
            parameter: RLIMIT_AS as i32,
            block_factor: 1024 as i32,
            description: b"virtual memory\0" as *const u8 as *const libc::c_char,
            units: b"kbytes\0" as *const u8 as *const libc::c_char,
        }
    },
    {
        RESOURCE_LIMITS {
            option: 'x' as i32,
            parameter: __RLIMIT_LOCKS as i32,
            block_factor: 1 as i32,
            description: b"file locks\0" as *const u8 as *const libc::c_char,
            units: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
        }
    },
    {
        RESOURCE_LIMITS {
            option: -1,
            parameter: -1,
            block_factor: -1,
            description: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
            units: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
        }
    },
];

extern "C" {
    fn getdtablesize() -> libc::c_int;
    fn strerror(_: i32) -> *mut libc::c_char;
    fn getrlimit(__resource: __rlimit_resource_t, __rlimits: *mut rlimit) -> i32;
    fn setrlimit(__resource: __rlimit_resource_t, __rlimits: *const rlimit) -> i32;
}

static mut optstring: [libc::c_char; 4 + 2 * NCMDS!() as usize] = [0; 4 + 2 * NCMDS!() as usize];
static mut cmdlist: *mut ULCMD = 0 as *const ULCMD as *mut ULCMD;
static mut ncmd: i32 = 0;

fn _findlim(opt: i32) -> i32 {
    //  let mut register : i32;
    //let i : i32 = 0;

    for i in 0..17 {
        if limits[i].option > 0 {
            if limits[i].option == opt {
                return i as i32;
            }
        }
    }
    -1
}

#[no_mangle]
pub fn ulimit_builtin(mut list: *mut WordList) -> i32 {
    let mut s: *mut libc::c_char;
    let c: i32;
    let mut limind: i32;
    let mut mode: i32 = 0;
    let mut opt: i32 = 0;
    let mut all_limits: i32 = 0;
    unsafe {
        if optstring[0] == 0 {
            s = optstring.as_mut_ptr();
            s = s.offset(0);
            *s = 'a' as libc::c_char;
            s = s.offset(1);
            *s = 'S' as libc::c_char;
            s = s.offset(1);
            *s = 'H' as libc::c_char;
            s = s.offset(1);
            c = 0;
            for i in 0..17 {
                if limits[i].option > 0 {
                    *s = limits[i].option as libc::c_char;
                    s = s.offset(1);
                    *s = ';' as libc::c_char;
                    s = s.offset(1);
                }
            }
            *s = '\0' as libc::c_char;
        }
    }

    if unsafe { cmdlistsz } == 0 {
        unsafe { cmdlistsz = 16 };
        unsafe {
            cmdlist = libc::malloc(
                (cmdlistsz as u64 as usize)
                    * (std::mem::size_of::<ULCMD>() as libc::c_ulong) as usize,
            ) as *mut ULCMD;
        }
    }
    unsafe { ncmd = 0 };
    reset_internal_getopt();
    opt = internal_getopt(list, unsafe { optstring.as_ptr() } as *mut libc::c_char);
    while opt != -1 {
        let optu8: u8 = opt as u8;
        let optChar: char = char::from(optu8);
        match optChar {
            'a' => {
                all_limits = all_limits + 1;
            }
            'S' => {
                mode = mode | LIMIT_SOFT!();
            }
            'H' => {
                mode = mode | LIMIT_HARD!();
            }
            '?' => {
                builtin_usage();
                return EX_USAGE as libc::c_int;
            }
            _ => {
                if opt == -99 {
                    builtin_help();
                    return EX_USAGE as libc::c_int;
                }
                if unsafe { ncmd } >= unsafe { cmdlistsz } {
                    unsafe { cmdlistsz = cmdlistsz * 2 };
                    unsafe {
                        cmdlist = libc::realloc(
                            cmdlist as *mut libc::c_void,
                            ((cmdlistsz as libc::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<ULCMD>() as libc::c_ulong))
                                as usize,
                            //(cmdlistsz as u64) * std::mem::size_of::<ULCMD>() as usize,
                        ) as *mut ULCMD
                    };
                }
                unsafe {
                    (*cmdlist.offset(ncmd as isize)).cmd = opt;
                    let fresh5 = ncmd;
                    //ncmd = ncmd + 1;
                    let ref mut fresh6 = (*cmdlist.offset(fresh5 as isize)).arg;
                    *fresh6 = list_optarg;
                    // let mut cmm =&mut  (*((cmdlist as usize +
                    //                     (ncmd as usize)*std::mem::size_of::<ULCMD>())as *mut ULCMD) as ULCMD);
                    // cmm.cmd = opt;
                    // cmm.arg = list_optarg;
                    // (*((cmdlist as usize + (ncmd as usize)*std::mem::size_of::<ULCMD>())
                    // as *mut ULCMD) as ULCMD).cmd = opt ;
                    //  (*((cmdlist as usize + (ncmd as usize) * std::mem::size_of::<ULCMD>())
                    //  as *mut ULCMD) as ULCMD).arg = list_optarg;
                    ncmd = ncmd + 1;
                }
            }
        }
        opt = internal_getopt(list, unsafe { optstring.as_ptr() } as *mut libc::c_char);
    }

    //  as *mut ULCMD) as ULCMD).cmd );
    list = unsafe { loptend };

    if all_limits != 0 {
        if mode == 0 {
            print_all_limits(LIMIT_SOFT!());
        } else {
            print_all_limits(mode);
        }
        return sh_chkwrite(EXECUTION_SUCCESS!());
    }

    if unsafe { ncmd } == 0 {
        unsafe {
            (*cmdlist.offset(ncmd as isize)).cmd = 'f' as i32;
            //   let mut cmm =  *((cmdlist as usize + (ncmd as usize )*std::mem::size_of::<ULCMD>())as *mut ULCMD) as ULCMD;
            //   cmm.cmd = 'f' as i32;
        }
        /* `ulimit something' is same as `ulimit -f something' */
        if !list.is_null() {
            unsafe {
                (*cmdlist.offset(ncmd as isize)).arg = (*(*list).word).word;
                // let mut cmm =  *((cmdlist as usize + (ncmd as usize )*std::mem::size_of::<ULCMD>())as *mut ULCMD) as ULCMD;
                // cmm.arg =  (*(*list).word).word;
                ncmd = ncmd + 1;
            }
        } else {
            unsafe {
                (*cmdlist.offset(ncmd as isize)).arg = std::ptr::null_mut();
                // let mut cmm = *((cmdlist as usize + (ncmd as usize )*std::mem::size_of::<ULCMD>())as *mut ULCMD) as ULCMD;
                // cmm.arg  =  std::ptr::null_mut();
                ncmd = ncmd + 1;
            }
        }
        if !list.is_null() {
            list = (unsafe { *list }).next;
        }
    }

    for d in 0..unsafe { ncmd } {
        //as *mut ULCMD) as ULCMD).cmd);
        let cmm = unsafe {
            *((cmdlist as usize + (d as usize) * std::mem::size_of::<ULCMD>()) as *mut ULCMD)
        } as ULCMD;
        let _dmd = cmm.cmd;

        limind = unsafe { _findlim((*cmdlist.offset(d as isize)).cmd) };
        if limind == -1 {
            unsafe {
                builtin_error(
                    b"%s: bad command : %s\0" as *const u8 as *const libc::c_char,
                    (*cmdlist.offset(d as isize)).cmd,
                    strerror(errno!()) as *const libc::c_char,
                );
            }
            return EX_USAGE as libc::c_int;
        }
    }
    unsafe {
        for d in 0..ncmd {
            let dmd = (*cmdlist.offset(d as isize)).cmd;
            let drg = (*cmdlist.offset(d as isize)).arg;
            // let dmd =   (*((cmdlist as usize + (d as usize )*std::mem::size_of::<ULCMD>())
            // as *mut ULCMD) as ULCMD).cmd;
            // let drg =  (*((cmdlist as usize + (d as usize )*std::mem::size_of::<ULCMD>())
            // as *mut ULCMD) as ULCMD).arg;
            if (ulimit_internal(dmd, drg, mode, d - 1)) == EXECUTION_FAILURE!() {
                return EXECUTION_FAILURE!();
            }
        }
    }
    return EXECUTION_SUCCESS!();
}

fn ulimit_internal(cmd: i32, cmdarg: *mut libc::c_char, mut mode: i32, multiple: i32) -> i32 {
    let opt: i32;
    let limind: i32;
    let setting: i32;
    let block_factor: i32;
    let mut soft_limit: RLIMTYPE = 0;
    let mut hard_limit: RLIMTYPE = 0;
    let mut real_limit: RLIMTYPE = 0;
    let limit: RLIMTYPE;

    if cmdarg != std::ptr::null_mut() {
        setting = 1;
    } else {
        setting = 0;
    }
    limind = _findlim(cmd);
    if mode == 0 {
        if setting != 0 {
            mode = LIMIT_HARD!() | LIMIT_SOFT!();
        } else {
            mode = LIMIT_SOFT!();
        }
    }
    opt = get_limit(limind, &mut soft_limit, &mut hard_limit);

    if opt < 0 {
        unsafe {
            builtin_error(
                b"%s: cannot get limit : %s\0" as *const u8 as *const libc::c_char,
                limits[limind as usize].description,
                strerror(errno!()) as *const libc::c_char,
            );
        }

        return EXECUTION_FAILURE!();
    }

    if setting == 0 {
        if (mode & LIMIT_SOFT!()) != 0 {
            printone(limind, soft_limit, multiple);
        } else {
            printone(limind, hard_limit, multiple);
        }
        return EXECUTION_SUCCESS!();
    }

    let c_str_hard = CString::new("hard").unwrap();
    let c_str_soft = CString::new("soft").unwrap();
    let c_str_unlimited = CString::new("unlimited").unwrap();
    unsafe {
        if STREQ!(cmdarg, c_str_hard.as_ptr() as *mut libc::c_char) {
            real_limit = hard_limit;
        } else if STREQ!(cmdarg, c_str_soft.as_ptr() as *mut libc::c_char) {
            real_limit = soft_limit;
        } else if STREQ!(cmdarg, c_str_unlimited.as_ptr() as *mut libc::c_char) {
            real_limit = RLIM_INFINITY!();
        } else if all_digits(cmdarg) != 0 {
            limit = string_to_rlimtype(cmdarg) as i64;
            block_factor = BLOCKSIZE!(limits[limind as usize].block_factor);
            real_limit = limit * block_factor as i64;

            if (real_limit / block_factor as i64) != limit {
                let c_str_limit = CString::new("limit").unwrap();
                sh_erange(cmdarg, c_str_limit.as_ptr() as *mut libc::c_char);
                return EXECUTION_FAILURE!();
            }
        } else {
            sh_invalidnum(cmdarg);
            return EXECUTION_FAILURE!();
        }
    }
    if set_limit(limind, real_limit, mode) < 0 {
        unsafe {
            builtin_error(
                b"%s: cannot modify limit : %s\0" as *const u8 as *const libc::c_char,
                limits[limind as usize].description,
                strerror(errno!()) as *const libc::c_char,
            )
        };
        return EXECUTION_FAILURE!();
    }
    return EXECUTION_SUCCESS!();
}

fn get_limit(ind: i32, softlim: *mut RLIMTYPE, hardlim: *mut RLIMTYPE) -> i32 {
    let mut value: RLIMTYPE = 0;
    let mut limit: rlimit = rlimit {
        rlim_cur: 1,
        rlim_max: 1,
    };

    if limits[ind as usize].parameter >= 256 {
        match limits[ind as usize].parameter {
            RLIMIT_FILESIZE!() => {
                if filesize(((&mut value) as *mut i64) as *mut u64) < 0 {
                    return -1;
                }
            }
            RLIMIT_PIPESIZE!() => {
                if pipesize(((&mut value) as *mut i64) as *mut u64) < 0 {
                    return -1;
                }
            }
            RLIMIT_OPENFILES!() => {
                value = unsafe { getdtablesize() } as RLIMTYPE;
            }
            RLIMIT_VIRTMEM!() => {
                return getmaxvm(softlim, hardlim as *mut libc::c_char);
            }
            RLIMIT_MAXUPROC!() => {
                if getmaxuprc((value as usize) as *mut u64) < 0 {
                    return -1;
                }
            }
            _ => unsafe {
                errno!() = libc::EINVAL;
            },
        }
        unsafe {
            *softlim = value;
            *hardlim = value;
        }
        return 0;
    } else {
        unsafe {
            let ii = getrlimit(
                limits[ind as u32 as usize].parameter as __rlimit_resource_t,
                &mut limit,
            );
            if ii < 0 {
                return -1;
            }
        }
        unsafe {
            // limit.rlim_max as i64);
            *softlim = limit.rlim_cur as i64;
            *hardlim = limit.rlim_max as i64;
        }
        return 0;
    }
}

fn set_limit(ind: i32, newlim: RLIMTYPE, mode: i32) -> i32 {
    let mut limit: rlimit = rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };
    let mut val: RLIMTYPE = 0;

    if limits[ind as usize].parameter >= 256 {
        match limits[ind as usize].parameter {
            RLIMIT_FILESIZE!() => {
                unsafe {
                    errno!() = libc::EINVAL;
                }
                return -1;
            }
            RLIMIT_OPENFILES!()
            | RLIMIT_PIPESIZE!()
            | RLIMIT_VIRTMEM!()
            | RLIMIT_MAXUPROC!()
            | _ => {
                unsafe {
                    errno!() = libc::EINVAL;
                }
                return -1;
            }
        }
    } else {
        if unsafe {
            getrlimit(
                limits[ind as usize].parameter as __rlimit_resource_t,
                &mut limit,
            )
        } < 0
        {
            return -1;
        }
        let b = unsafe { current_user.euid } != 0
            && newlim == RLIM_INFINITY!()
            && (mode & LIMIT_HARD!()) == 0
            && limit.rlim_cur <= limit.rlim_max;
        if b {
            val = limit.rlim_max as i64;
        } else {
            val = newlim;
        }
        if mode & LIMIT_SOFT!() != 0 {
            limit.rlim_cur = val as u64;
        }
        if mode & LIMIT_HARD!() != 0 {
            limit.rlim_max = val as u64;
        }
        return unsafe {
            setrlimit(
                limits[ind as usize].parameter as __rlimit_resource_t,
                &mut limit,
            )
        };
    }
}

fn getmaxvm(softlim: *mut RLIMTYPE, hardlim: *mut libc::c_char) -> i32 {
    let mut datalim: rlimit = rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };
    let mut stacklim: rlimit = rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };

    if unsafe { getrlimit(RLIMIT_DATA, &mut datalim) } < 0 {
        return -1;
    }
    if unsafe { getrlimit(RLIMIT_STACK, &mut stacklim) } < 0 {
        return -1;
    }
    unsafe {
        *softlim =
            (datalim.rlim_cur as i64 / 1024 as i64) + (stacklim.rlim_cur as i64 / 1024 as i64)
    };
    unsafe {
        *hardlim = ((datalim.rlim_max as i64) / 1024 as i64) as libc::c_char
            + (stacklim.rlim_max as i64 / 1024 as i64) as libc::c_char
    };
    return 0;
}

fn filesize(_valuep: *mut rlim_t) -> i32 {
    unsafe {
        errno!() = libc::EINVAL;
    }
    return -1;
}

fn pipesize(valuep: *mut rlim_t) -> i32 {
    unsafe { *((valuep as usize) as *mut rlim_t) = PIPE_BUF!() as rlim_t };
    return 0;
}

fn getmaxuprc(valuep: *mut rlim_t) -> i32 {
    let mut maxchild: i64 = 0;
    maxchild = unsafe { getmaxchild() };
    if maxchild < 0 as i32 as libc::c_long {
        unsafe {
            errno!() = libc::EINVAL;
        }
        return -1;
    } else {
        unsafe {
            *valuep = maxchild as rlim_t;
        }
        return 0;
    };
}

fn print_all_limits(mut mode: i32) {
    let mut i: i32;
    let mut softlim: RLIMTYPE = 0;
    let mut hardlim: RLIMTYPE = 0;

    if mode == 0 {
        mode = mode | LIMIT_SOFT!();
    }
    i = 0;
    while limits[i as usize].option > 0 {
        if get_limit(i, &mut softlim, &mut hardlim) == 0 {
            if mode & LIMIT_SOFT!() != 0 {
                printone(i, softlim, 1);
            } else {
                printone(i, hardlim, 1);
            }
        } else if unsafe { errno!() != libc::EINVAL } {
            unsafe {
                builtin_error(
                    b"%s: cannot get limit : %s\0" as *const u8 as *const libc::c_char,
                    limits[i as usize].description,
                    strerror(errno!()) as *const libc::c_char,
                );
            }
        }
        i = i + 1;
    }
}

fn printone(limind: i32, curlim: RLIMTYPE, pdesc: i32) {
    let mut unitstr: [libc::c_char; 64] = [0; 64];
    let factor: i32;

    factor = BLOCKSIZE!(limits[limind as usize].block_factor);
    if pdesc > 0 {
        if !limits[limind as usize].units.is_null() {
            unsafe {
                libc::sprintf(
                    unitstr.as_mut_ptr(),
                    b"(%s, -%c) \0" as *const u8 as *const libc::c_char,
                    limits[limind as usize].units,
                    limits[limind as usize].option,
                );
            }
        } else {
            unsafe {
                libc::sprintf(
                    unitstr.as_mut_ptr(),
                    b"(-%c) \0" as *const u8 as *const libc::c_char,
                    limits[limind as usize].option,
                );
            }
        }
        print!(
            "{:<20} {:>20}",
            unsafe {
                CStr::from_ptr(limits[limind as usize].description)
                    .to_str()
                    .unwrap()
            },
            unsafe { CStr::from_ptr(unitstr.as_mut_ptr()).to_str().unwrap() }
        );
    }
    if curlim == RLIM_INFINITY!() {
        let c_str_unlimited = b"unlimited" as *const u8 as *const libc::c_char;
        println!("{}", unsafe {
            CStr::from_ptr(c_str_unlimited).to_str().unwrap()
        });
    } else if curlim == RLIM_SAVED_MAX!() {
        //println!("hard");
        let c_str_hard = b"hard" as *const u8 as *const libc::c_char;
        println!("{}", unsafe {
            CStr::from_ptr(c_str_hard).to_str().unwrap()
        });
    } else if curlim == RLIM_SAVED_CUR!() {
        //println!("soft");
        let c_str_soft = b"soft" as *const u8 as *const libc::c_char;
        println!("{}", unsafe {
            CStr::from_ptr(c_str_soft).to_str().unwrap()
        });
    } else {
        print_rlimtype((curlim / factor as i64) as u64, 1);
    }
}

/* Set all limits to NEWLIM.  NEWLIM currently must be RLIM_INFINITY, which
   causes all limits to be set as high as possible depending on mode (like
   csh `unlimit').  Returns -1 if NEWLIM is invalid, 0 if all limits
   were set successfully, and 1 if at least one limit could not be set.

   To raise all soft limits to their corresponding hard limits, use
    ulimit -S -a unlimited
   To attempt to raise all hard limits to infinity (superuser-only), use
    ulimit -H -a unlimited
   To attempt to raise all soft and hard limits to infinity, use
    ulimit -a unlimited
*/

fn print_rlimtype(num: u64, nl: i32) {
    if nl > 0 {
        println!("{num}");
    } else {
        print!("{num}");
    }
}

fn set_all_limits(mut mode: i32, newlim: RLIMTYPE) -> i32 {
    let mut i: i32;
    let mut retval: i32 = 0;

    if newlim != RLIM_INFINITY!() {
        unsafe {
            errno!() = libc::EINVAL;
        }
        return -1;
    }

    if mode == 0 {
        mode = LIMIT_SOFT!() | LIMIT_HARD!();
    }
    retval = 0;
    i = 0;

    while limits[i as usize].option > 0 {
        if set_limit(i, newlim, mode) < 0 {
            unsafe {
                builtin_error(
                    b"%s: cannot modify limit : %s\0" as *const u8 as *const libc::c_char,
                    limits[i as usize].description,
                    strerror(errno!()) as *const libc::c_char,
                );
            }
            retval = 1;
            i = i + 1;
        }
    }
    return retval;
}
