use crate::builtins::common::{no_options, sh_chkwrite};
use crate::src_common::*;

extern "C" {
    fn print_timeval(fp: *mut libc::FILE, tvp: *mut libc::timeval);
}

pub fn c_print_timeval(fp: *mut libc::FILE, tvp: *mut libc::timeval) -> () {
    unsafe { print_timeval(fp, tvp) }
}

#[no_mangle]
pub fn times_builtin(list: *mut WordList) -> i32 {
    if no_options(list) != 0 {
        return EX_USAGE;
    }
    unsafe {
        let curr: libc::rusage = std::mem::zeroed();
        let kids: libc::rusage = std::mem::zeroed();
        libc::putchar(b'\n' as libc::c_int);

        libc::getrusage(libc::RUSAGE_SELF, std::mem::transmute(&curr));
        libc::getrusage(libc::RUSAGE_CHILDREN, std::mem::transmute(&kids));

        c_print_timeval(stdout, std::mem::transmute(&curr.ru_utime));
        libc::putchar(b' ' as libc::c_int);
        c_print_timeval(stdout, std::mem::transmute(&curr.ru_stime));
        libc::putchar(b'\n' as libc::c_int);

        c_print_timeval(stdout, std::mem::transmute(&kids.ru_utime));
        libc::putchar(b' ' as libc::c_int);
        c_print_timeval(stdout, std::mem::transmute(&kids.ru_stime));
        libc::putchar(b'\n' as libc::c_int);
    }
    return sh_chkwrite(EXECUTION_SUCCESS);
}
