//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.  

//# SPDX-License-Identifier: GPL-3.0-or-later
use libc::{c_int};
use rcommon::{r_sh_chkwrite};
include!(concat!("intercdep.rs"));

#[no_mangle]
pub extern "C" fn r_times_builtin(list: *mut WordList) -> i32 {

unsafe {

    if no_options(list) != 0 {
        return EX_USAGE;
    }

    let curr: libc::rusage = std::mem::zeroed();
    let kids: libc::rusage = std::mem::zeroed();
    libc::putchar(b'\n' as c_int);

    libc::getrusage(libc::RUSAGE_SELF, std::mem::transmute(&curr));
    libc::getrusage(libc::RUSAGE_CHILDREN, std::mem::transmute(&kids));

    print_timeval(stdout, std::mem::transmute(&curr.ru_utime));
    libc::putchar(b' ' as c_int);
    print_timeval(stdout, std::mem::transmute(&curr.ru_stime));
    libc::putchar(b'\n' as c_int);

    print_timeval(stdout, std::mem::transmute(&kids.ru_utime));
    libc::putchar(b' ' as c_int);
    print_timeval(stdout, std::mem::transmute(&kids.ru_stime));
    libc::putchar(b'\n' as c_int);

    return r_sh_chkwrite(EXECUTION_SUCCESS);
}
}
