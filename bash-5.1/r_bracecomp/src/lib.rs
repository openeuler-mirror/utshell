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

pub type rl_quote_func_t = unsafe extern "C" fn(
    *mut libc::c_char,
    libc::c_int,
    *mut libc::c_char,
) -> *mut libc::c_char;

pub type rl_compignore_func_t = unsafe extern "C" fn(
    *mut *mut libc::c_char,
) -> libc::c_int;

extern "C" {
    fn rl_complete_internal(_: libc::c_int) -> libc::c_int;
    fn rl_filename_completion_function(
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    static mut rl_completion_entry_function: Option::<rl_compentry_func_t>;
    static mut rl_ignore_some_completions_function: Option::<rl_compignore_func_t>;
    static mut rl_attempted_completion_function: Option::<rl_completion_func_t>;
    static mut rl_filename_quoting_desired: libc::c_int;
    static mut rl_filename_quoting_function: Option::<rl_quote_func_t>;

}

unsafe extern "C" fn hack_braces_completion(
    mut names: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    i = strvec_len(names);
    if __ctype_get_mb_cur_max() > 1 as usize
        && i > 2 as libc::c_int
    {
        qsort(
            names.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            (i - 1 as libc::c_int) as usize,
            std::mem::size_of::<*mut libc::c_char>() as usize,
            std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> libc::c_int>,
                Option::<QSFUNC>,
            >(
                Some(
                    std::mem::transmute::<
                        unsafe extern "C" fn(
                            *mut *mut libc::c_char,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                        unsafe extern "C" fn() -> libc::c_int,
                    >(_strcompare),
                ),
            ),
        );
    }
    temp = really_munge_braces(names, 1 as libc::c_int, i, 0 as libc::c_int);
    i = 0 as libc::c_int;
    while !(*names.offset(i as isize)).is_null() {
        free(*names.offset(i as isize) as *mut libc::c_void);
        let ref mut fresh0 = *names.offset(i as isize);
        *fresh0 = 0 as *mut libc::c_char;
        i += 1;
        i;
    }
    let ref mut fresh1 = *names.offset(0 as libc::c_int as isize);
    *fresh1 = temp;
    return 0 as libc::c_int;
}

