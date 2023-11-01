use libc::*;
use r_bash::*;

pub type __intmax_t = libc::c_long;
pub type intmax_t = __intmax_t;
// pub type mbstate_t = __mbstate_t;

pub const _ISdigit: libc::c_uint = 2048;
pub const _ISalpha: libc::c_uint = 1024;
const CHAR_BIT : libc::c_int = 8 as libc::c_int;
static mut  errno :  libc::c_int = 0 as libc::c_int;

extern "C" {
    fn __strtol_internal(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
        __group: libc::c_int,
    ) -> libc::c_long;
    // pub fn isalpha(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    // pub fn isdigit(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}

#[macro_export] 
macro_rules! ST_BAD {
    () =>{
        0 as libc::c_int
    }
}

#[macro_export] 
macro_rules! ST_INT {
    () =>{
        1 as libc::c_int
    }
}

#[inline]
unsafe extern "C" fn strtoimax(
    mut nptr: *const libc::c_char,
    mut endptr: *mut *mut libc::c_char,
    mut base: libc::c_int,
) -> intmax_t {
    return __strtol_internal(nptr, endptr, base, 0 as libc::c_int);
}

unsafe fn STREQN(a: *const c_char, b: *const c_char, n: i32) -> bool {
    if n == 0 {
        return true;
    } else {
        return *a == *b && libc::strncmp(a, b, n as libc::size_t) == 0;
    }
}

#[inline]
unsafe extern "C" fn is_basic(mut c: libc::c_char) -> libc::c_int {

    return (*is_basic_table
        .as_ptr()
        .offset((c as libc::c_uchar as libc::c_int >> 5 as libc::c_int) as isize)
        >> (c as libc::c_uchar as libc::c_int & 31 as libc::c_int)
        & 1 as libc::c_int as libc::c_uint) as libc::c_int;
}
static mut brace_arg_separator: libc::c_int = ',' as i32;

