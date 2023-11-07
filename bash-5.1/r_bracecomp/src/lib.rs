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
unsafe extern "C" fn really_munge_braces(
    mut array: *mut *mut libc::c_char,
    mut real_start: libc::c_int,
    mut real_end: libc::c_int,
    mut gcd_zero: libc::c_int,
) -> *mut libc::c_char {
    let mut start: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut gcd: libc::c_int = 0;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut subterm: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result_size: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    let mut tlen: libc::c_int = 0;
    flag = 0 as libc::c_int;
    if real_start == real_end {
        x = if !(*array.offset(real_start as isize)).is_null() {
            sh_backslash_quote(
                (*array.offset(real_start as isize)).offset(gcd_zero as isize),
                0 as *const libc::c_char,
                0 as libc::c_int,
            )
        } else {
            sh_backslash_quote(
                *array,
                0 as *const libc::c_char,
                0 as libc::c_int,
            )
        };
        return x;
    }
    result_size = 16 as libc::c_int;
    result = xmalloc(result_size as usize) as *mut libc::c_char;
    *result = '\0' as i32 as libc::c_char;
    start = real_start;
    while start < real_end {
        gcd = strlen(*array.offset(start as isize)) as libc::c_int;
        end = start + 1 as libc::c_int;
        while end < real_end {
            let mut temp: libc::c_int = 0;
            temp = string_gcd(
                *array.offset(start as isize),
                *array.offset(end as isize),
            );
            if temp <= gcd_zero {
                break;
            }
            gcd = temp;
            end += 1;
        }
        end -= 1;
        if gcd_zero == 0 as libc::c_int && start == real_start
            && end != real_end - 1 as libc::c_int
        {
            result_size += 1 as libc::c_int;
            result = xrealloc(result as *mut libc::c_void, result_size as usize)
                as *mut libc::c_char;
            *result.offset(0 as libc::c_int as isize) = '{' as i32 as libc::c_char;
            *result.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            flag += 1;
            flag;
        }
        if start == end {
            x = savestring!((*array.offset(start as isize)).offset(gcd_zero as isize));
            subterm = sh_backslash_quote(x, 0 as *const libc::c_char, 0 as libc::c_int);
            free(x as *mut libc::c_void);
        } else {
            tlen = gcd - gcd_zero;
            x = xmalloc((tlen + 1 as libc::c_int) as usize) as *mut libc::c_char;
            strncpy(
                x,
                (*array.offset(start as isize)).offset(gcd_zero as isize),
                tlen as usize,
            );
            *x.offset(tlen as isize) = '\0' as i32 as libc::c_char;
            subterm = sh_backslash_quote(x, 0 as *const libc::c_char, 0 as libc::c_int);
            free(x as *mut libc::c_void);
            result_size += strlen(subterm) as libc::c_int + 1 as libc::c_int;
            result = xrealloc(result as *mut libc::c_void, result_size as usize)
                as *mut libc::c_char;
            strcat(result, subterm);
            free(subterm as *mut libc::c_void);
            strcat(result, b"{\0" as *const u8 as *const libc::c_char);
            subterm = really_munge_braces(array, start, end + 1 as libc::c_int, gcd);
            *subterm
                .offset(
                    (strlen(subterm) + 1 as libc::c_ulong) as isize,
                ) = '}' as i32 as libc::c_char;
        }
        result_size += strlen(subterm) as libc::c_int +1 as libc::c_int;
        result = xrealloc(result as *mut libc::c_void, result_size as usize)
            as *mut libc::c_char;
        strcat(result, subterm);
        strcat(result, b",\0" as *const u8 as *const libc::c_char);
        free(subterm as *mut libc::c_void);
        start = end + 1 as libc::c_int;
    }
    if gcd_zero == 0 as libc::c_int {
        *result
            .offset(
                (strlen(result) - 1 as libc::c_ulong) as isize,
            ) = (if flag != 0 { '}' as i32 } else { '\0' as i32 }) as libc::c_char;
    }
    return result;
}

unsafe extern "C" fn _strcompare(
    mut s1: *mut *mut libc::c_char,
    mut s2: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    result = **s1 as libc::c_int - **s2 as libc::c_int;
    if result == 0 as libc::c_int {
        result = strcmp(*s1, *s2);
    }
    return result;
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

#[no_mangle]
pub unsafe extern "C" fn bash_brace_completion(
    mut count: libc::c_int,
    mut ignore: libc::c_int,
) -> libc::c_int {
    let mut orig_ignore_func: Option::<rl_compignore_func_t> = None;
    let mut orig_entry_func: Option::<rl_compentry_func_t> = None;
    let mut orig_quoting_func: Option::<rl_quote_func_t> = None;
    let mut orig_attempt_func: Option::<rl_completion_func_t> = None;
    let mut orig_quoting_desired: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    orig_ignore_func = rl_ignore_some_completions_function;
    orig_attempt_func = rl_attempted_completion_function;
    orig_entry_func = rl_completion_entry_function;
    orig_quoting_func = rl_filename_quoting_function;
    orig_quoting_desired = rl_filename_quoting_desired;
    rl_completion_entry_function = Some(
        rl_filename_completion_function
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> *mut libc::c_char,
    );
    rl_attempted_completion_function = ::core::mem::transmute::<
        *mut libc::c_void,
        Option::<rl_completion_func_t>,
    >(0 as *mut libc::c_void);
    rl_ignore_some_completions_function = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn() -> libc::c_int>,
        Option::<rl_compignore_func_t>,
    >(
        Some(
            ::core::mem::transmute::<
                unsafe extern "C" fn(*mut *mut libc::c_char) -> libc::c_int,
                unsafe extern "C" fn() -> libc::c_int,
            >(hack_braces_completion),
        ),
    );
    rl_filename_quoting_function = ::core::mem::transmute::<
        *mut libc::c_void,
        Option::<rl_quote_func_t>,
    >(0 as *mut libc::c_void);
    rl_filename_quoting_desired = 0 as libc::c_int;
    r = rl_complete_internal('\t' as i32);
    rl_ignore_some_completions_function = orig_ignore_func;
    rl_attempted_completion_function = orig_attempt_func;
    rl_completion_entry_function = orig_entry_func;
    rl_filename_quoting_function = orig_quoting_func;
    rl_filename_quoting_desired = orig_quoting_desired;
    return r;
}

