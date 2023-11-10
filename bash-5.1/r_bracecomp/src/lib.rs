use r_bash::*;

#[macro_export]
macro_rules! savestring {
    ($x:expr) => {
        strcpy(
            xmalloc(
                (strlen($x as *const libc::c_char) + 1) as usize)
                 as *mut libc::c_char, $x
                ) as *mut libc::c_char
    };
}

unsafe extern "C" fn string_gcd(
    mut s1: *mut libc::c_char,
    mut s2: *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if s1.is_null() || s2.is_null() {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while *s1 as libc::c_int != 0 && *s2 as libc::c_int != 0 {
        if *s1 as libc::c_int != *s2 as libc::c_int {
            break;
        }
        s1 = s1.offset(1);
        s2 = s2.offset(1);
        i += 1;
    }
    return i;
}

pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type QSFUNC = unsafe extern "C" fn(
    *const libc::c_void,
    *const libc::c_void,
) -> libc::c_int;

pub type rl_compentry_func_t = unsafe extern "C" fn(
    *const libc::c_char,
    libc::c_int,
) -> *mut libc::c_char;

pub type rl_completion_func_t = unsafe extern "C" fn(
    *const libc::c_char,
    libc::c_int,
    libc::c_int,
) -> *mut *mut libc::c_char;


