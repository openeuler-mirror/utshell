use crate::general::legal_number;
use crate::sig::{termsig_handler, throw_to_top_level};
use crate::src_common::*;
use crate::stringlib::substring;
use crate::subst::extract_command_subst;

/* 宏ADVANCE_CHAR在src_common中有定义，这里是其中的一个变形 */
#[macro_export]
macro_rules! ADVANCE_CHAR_1 {
    ($str:expr, $strsize:expr, $i:expr) => {
        $i += 1;
    };
}

//errno 的一个变形
static mut errno: libc::c_int = 0 as libc::c_int;

#[inline]
fn strtoimax(
    nptr: *const libc::c_char,
    endptr: *mut *mut libc::c_char,
    base: libc::c_int,
) -> intmax_t {
    // SAFETY: 调用C库函数
    c___strtol_internal(nptr, endptr, base, 0 as libc::c_int)
}

fn STREQN(a: *const libc::c_char, b: *const libc::c_char, n: i32) -> bool {
    if n == 0 {
        return true;
    }
    // SAFETY: 指针解引用和字符串比较
    unsafe { *a == *b && libc::strncmp(a, b, n as libc::size_t) == 0 }
}

#[inline]
fn is_basic(c: libc::c_char) -> libc::c_int {
    // SAFETY: 访问静态查找表
    unsafe {
        (*is_basic_table
            .as_ptr()
            .offset((c as libc::c_uchar as libc::c_int >> 5 as libc::c_int) as isize)
            >> (c as libc::c_uchar as libc::c_int & 31 as libc::c_int)
            & 1 as libc::c_int as libc::c_uint) as libc::c_int
    }
}

static mut brace_arg_separator: libc::c_int = ',' as i32;

#[no_mangle]
pub fn brace_expand(text: *mut libc::c_char) -> *mut *mut libc::c_char {
    let start: libc::c_int;
    let tlen: size_t;
    let preamble: *mut libc::c_char;
    let postamble: *mut libc::c_char;
    let amble: *mut libc::c_char;
    let alen: size_t;
    let mut tack: *mut *mut libc::c_char;
    let mut result: *mut *mut libc::c_char;
    let mut i: libc::c_int;
    let mut j: libc::c_int;
    let mut c: libc::c_int;
    let mut c1: libc::c_int;

    let mut state: mbstate_t = mbstate_t {
        __count: 0,
        __value: mbstate_t_value { __wch: 0 },
    };

    // SAFETY: 所有内存操作和字符串操作
    unsafe {
        libc::memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\0' as i32,
            std::mem::size_of::<mbstate_t>() as usize,
        );
        tlen = libc::strlen(text) as u64;
    }

    i = 0 as libc::c_int;

    loop {
        c = brace_gobbler(text, tlen, &mut i, '{' as i32);
        c1 = c;
        if c != 0 {
            j = i + 1 as libc::c_int;
            // start = j;
            c = brace_gobbler(text, tlen, &mut j, '}' as i32);
            if c == 0 as libc::c_int {
                i += 1;
                // c = c1;
                continue;
            } else {
                c = c1;
                break;
            }
        } else {
            break;
        }
    }

    // SAFETY: 内存分配和字符串操作
    unsafe {
        preamble = libc::malloc((i + 1 as libc::c_int) as size_t as usize) as *mut libc::c_char;

        if i > 0 as libc::c_int {
            libc::strncpy(preamble, text, i as usize);
        }
        *preamble.offset(i as isize) = '\0' as i32 as libc::c_char;

        result = libc::malloc(
            ((2 as libc::c_int as size_t) * (std::mem::size_of::<*mut libc::c_char>() as size_t))
                as usize,
        ) as *mut *mut libc::c_char;

        *result.offset(0 as libc::c_int as isize) = preamble;
        *result.offset(1 as libc::c_int as isize) = 0 as *mut libc::c_void as *mut libc::c_char;
    }

    if c != '{' as i32 {
        return result;
    }

    i += 1;
    start = i;

    c = brace_gobbler(text, tlen, &mut i, '}' as i32);
    if c == 0 as libc::c_int {
        // SAFETY: 内存释放和字符串操作
        unsafe {
            libc::free(preamble as *mut libc::c_void);
            *result.offset(0 as libc::c_int as isize) = savestring!(text);
        }
        return result;
    }
    amble = substring(text, start, i);
    alen = (i - start) as size_t;

    // SAFETY: 内存初始化
    unsafe {
        libc::memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\0' as i32,
            std::mem::size_of::<mbstate_t>() as size_t as usize,
        );
    }
    j = 0 as libc::c_int;

    // SAFETY: 字符串遍历和处理
    unsafe {
        loop {
            if *amble.offset(j as isize) == 0 {
                break;
            }
            if *amble.offset(j as isize) as libc::c_int == '\\' as i32 {
                j += 1;
                ADVANCE_CHAR_1!(amble, alen, j);
                continue;
            }
            if *amble.offset(j as isize) as libc::c_int == brace_arg_separator {
                break;
            }
            ADVANCE_CHAR_1!(amble, alen, j);
        }

        loop {
            if *amble.offset(j as isize) as libc::c_int == 0 as libc::c_int {
                tack = expand_seqterm(amble, alen);
                if !tack.is_null() {
                    break;
                } else if !text.offset((i + 1) as isize).is_null() {
                    tack = c_strvec_create(2 as libc::c_int);
                    *tack = savestring!(text.offset((start - 1) as isize) as *mut libc::c_char);
                    *(*tack.offset((i - start + 2) as isize)) = '\0' as i32 as libc::c_char;
                    *tack.offset(1 as isize) = 0 as *mut libc::c_char;
                    break;
                } else {
                    libc::free(amble as *mut libc::c_void);
                    libc::free(preamble as *mut libc::c_void);
                    *result.offset(0 as libc::c_int as isize) = libc::strcpy(
                        libc::malloc((1 as libc::c_int as usize).wrapping_add(libc::strlen(text)))
                            as *mut libc::c_char,
                        text,
                    );
                    return result;
                }
            }
            tack = expand_amble(amble, alen, 0);
            break;
        }
    }

    result = array_concat(result, tack);

    // SAFETY: 内存释放
    unsafe {
        libc::free(amble as *mut libc::c_void);
    }
    if tack != result {
        c_strvec_dispose(tack);
    }

    // SAFETY: 指针操作
    unsafe {
        postamble = text.offset(i as isize).offset(1 as libc::c_int as isize);
        if !postamble.is_null() && *postamble as libc::c_int != 0 {
            tack = brace_expand(postamble);
            result = array_concat(result, tack);
            if tack != result {
                c_strvec_dispose(tack);
            }
        }
    }

    return result;
}

fn expand_amble(
    text: *mut libc::c_char,
    tlen: size_t,
    _flags: libc::c_int,
) -> *mut *mut libc::c_char {
    let mut result: *mut *mut libc::c_char;
    let mut partial: *mut *mut libc::c_char;
    let mut tresult: *mut *mut libc::c_char;
    let mut tem: *mut libc::c_char;
    let mut start: libc::c_int;
    let mut i: libc::c_int;
    let mut c: libc::c_int;

    let mut state: mbstate_t = mbstate_t {
        __count: 0,
        __value: mbstate_t_value { __wch: 0 },
    };

    // SAFETY: 内存初始化
    unsafe {
        libc::memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\0' as i32,
            std::mem::size_of::<mbstate_t>() as usize,
        );
    }

    result = 0 as *mut libc::c_void as *mut *mut libc::c_char;
    i = 0 as libc::c_int;
    start = i;
    c = 1 as libc::c_int;

    // SAFETY: 所有字符串和内存操作
    unsafe {
        while c != 0 {
            c = brace_gobbler(text, tlen, &mut i, brace_arg_separator);
            tem = substring(text, start, i);
            partial = brace_expand(tem);
            if result.is_null() {
                result = partial;
            } else {
                let lr: libc::c_int;
                let lp: libc::c_int;
                let mut j: libc::c_int;
                lr = c_strvec_len(result);
                lp = c_strvec_len(partial);
                tresult = c_strvec_mresize(result, lp + lr + 1 as libc::c_int);
                if tresult.is_null() {
                    internal_error(
                        b"brace expansion: cannot allocate memory for %s\0" as *const u8
                            as *mut libc::c_char,
                        tem,
                    );
                    libc::free(tem as *mut libc::c_void);
                    c_strvec_dispose(partial);
                    c_strvec_dispose(result);
                    result = 0 as *mut libc::c_void as *mut *mut libc::c_char;
                    return result;
                } else {
                    result = tresult;
                }
                j = 0 as libc::c_int;
                while j < lp {
                    *result.offset((lr + j) as isize) = *partial.offset(j as isize);
                    j += 1;
                }
                *result.offset((lr + j) as isize) = 0 as *mut libc::c_void as *mut libc::c_char;
                libc::free(partial as *mut libc::c_void);
            }
            libc::free(tem as *mut libc::c_void);
            if locale_mb_cur_max > 1 as libc::c_int {
                let mut state_bak: mbstate_t = mbstate_t {
                    __count: 0,
                    __value: mbstate_t_value { __wch: 0 },
                };
                let mblength: size_t;
                let mut _f: libc::c_int = 0;
                _f = is_basic(*text.offset(i as isize));
                if _f != 0 {
                    mblength = 1 as libc::c_int as size_t;
                } else if locale_utf8locale != 0
                    && *text.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                        == 0 as libc::c_int
                {
                    mblength = (*text.offset(i as isize) as libc::c_int != 0 as libc::c_int)
                        as libc::c_int as size_t;
                } else {
                    state_bak = state;
                    mblength = mbrlen(
                        text.offset(i as isize),
                        tlen.wrapping_sub(i as usize as u64),
                        &mut state,
                    );
                }
                if mblength == -(2 as libc::c_int) as size_t
                    || mblength == -(1 as libc::c_int) as size_t
                {
                    state = state_bak;
                    i += 1;
                } else if mblength == 0 as libc::c_int as usize as u64 {
                    i += 1;
                } else {
                    i = (i as usize).wrapping_add(mblength as usize) as libc::c_int as libc::c_int;
                }
            } else {
                i += 1;
            }
            start = i;
        }
    }

    return result;
}

fn mkseq(
    start: intmax_t,
    end: intmax_t,
    mut incr: intmax_t,
    type_0: libc::c_int,
    width: libc::c_int,
) -> *mut *mut libc::c_char {
    let mut n: intmax_t;
    let prevn: intmax_t;
    let mut i: libc::c_int;
    let nelem: libc::c_int;
    let mut result: *mut *mut libc::c_char;
    let mut t: *mut libc::c_char = std::ptr::null_mut();

    if incr == 0 as libc::c_int as libc::c_long {
        incr = 1 as libc::c_int as intmax_t;
    }

    if start > end && incr > 0 as libc::c_int as libc::c_long {
        incr = -incr;
    } else if start < end && incr < 0 as libc::c_int as libc::c_long {
        if incr == INTMAX_MIN!() {
            return 0 as *mut *mut libc::c_void as *mut *mut libc::c_char;
        }
        incr = -incr;
    }
    if SUBOVERFLOW!(end, start, INTMAX_MIN!() + 3, INTMAX_MAX!() - 2) {
        return 0 as *mut libc::c_void as *mut *mut libc::c_char;
    }
    prevn = sh_imaxabs!(end - start);

    if INT_MAX!() == INTMAX_MAX!() && ADDOVERFLOW!(prevn, 2, INT_MIN!(), INT_MAX!()) {
        return 0 as *mut libc::c_void as *mut *mut libc::c_char;
    } else if ADDOVERFLOW!((prevn / sh_imaxabs!(incr)), 1, INTMAX_MIN!(), INTMAX_MAX!()) {
        return 0 as *mut libc::c_void as *mut *mut libc::c_char;
    }

    if (prevn / sh_imaxabs!(incr)) > INT_MAX!() - 3 as libc::c_int as libc::c_long {
        return 0 as *mut libc::c_void as *mut *mut libc::c_char;
    }
    nelem = (prevn / sh_imaxabs!(incr)) as libc::c_int + 1 as libc::c_int;

    result = c_strvec_mcreate(nelem + 1);

    if result.is_null() {
        // SAFETY: 错误报告
        unsafe {
            internal_error(
                b"brace expansion: failed to allocate memory for %u elements\0" as *const u8
                    as *mut libc::c_char,
                nelem as libc::c_uint,
            );
        }
        return 0 as *mut libc::c_void as *mut *mut libc::c_char;
    }

    i = 0 as libc::c_int;
    n = start;

    // SAFETY: 所有内存分配和字符串操作
    unsafe {
        loop {
            if interrupt_state != 0 as libc::c_int {
                *result.offset(i as isize) = 0 as *mut libc::c_void as *mut libc::c_char;
                c_strvec_dispose(result);
                result = 0 as *mut libc::c_void as *mut *mut libc::c_char;
            }
            if terminating_signal != 0 {
                termsig_handler(terminating_signal);
            }
            if interrupt_state != 0 {
                throw_to_top_level();
            }
            if type_0 == ST_INT!() {
                t = c_itos(n);
                *result.offset(i as isize) = t;
                i += 1;
            } else if type_0 == ST_ZINT!() {
                // let mut len: libc::c_int = 0;
                let arg: libc::c_int;
                arg = n as libc::c_int;
                asprintf(
                    &mut t as *mut *mut libc::c_char,
                    b"%0*d\0" as *const u8 as *const libc::c_char,
                    width,
                    arg,
                );
                *result.offset(i as isize) = t;
                i += 1;
            } else {
                t = libc::malloc(2 as libc::c_int as usize) as *mut libc::c_char;
                if !t.is_null() {
                    *t.offset(0 as libc::c_int as isize) = n as libc::c_char;
                    *t.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                }
                *result.offset(i as isize) = t;
                i = i + 1;
            }
            if t.is_null() {
                let p: *mut libc::c_char;
                let mut lbuf: [libc::c_char; INT_STRLEN_BOUND!(intmax_t) + 1 as usize] =
                    [0; INT_STRLEN_BOUND!(intmax_t) + 1 as usize];
                p = c_inttostr(
                    n,
                    lbuf.as_mut_ptr(),
                    std::mem::size_of::<[libc::c_char; INT_STRLEN_BOUND!(intmax_t) + 1 as usize]>()
                        as usize as u64,
                );
                internal_error(
                    b"brace expansion: failed to allocate memory for '%s'\0" as *const u8
                        as *const libc::c_char,
                    p,
                );
                c_strvec_dispose(result);
                return 0 as *mut libc::c_void as *mut *mut libc::c_char;
            }

            if ADDOVERFLOW!(n, incr, INTMAX_MIN!(), INTMAX_MAX!()) {
                break;
            }
            n += incr;
            if incr < 0 as libc::c_int as libc::c_long && n < end
                || incr > 0 as libc::c_int as libc::c_long && n > end
            {
                break;
            }
        }
        *result.offset(i as isize) = 0 as *mut libc::c_char;
    }

    return result;
}

fn expand_seqterm(text: *mut libc::c_char, mut tlen: size_t) -> *mut *mut libc::c_char {
    let t: *mut libc::c_char;
    let lhs: *mut libc::c_char;
    let rhs: *mut libc::c_char;
    let mut lhs_t: libc::c_int;
    let mut rhs_t: libc::c_int;
    let lhs_l: libc::c_int;
    let rhs_l: libc::c_int;
    let mut width: libc::c_int;
    let lhs_v: intmax_t;
    let rhs_v: intmax_t;
    let mut incr: intmax_t;
    let mut tl: intmax_t = 0;
    let mut tr: intmax_t = 0;
    let result: *mut *mut libc::c_char;
    let mut ep: *mut libc::c_char;
    let oep: *mut libc::c_char;

    // SAFETY: 所有字符串和内存操作
    unsafe {
        t = libc::strstr(text, b"..\0" as *const u8 as *const libc::c_char);

        if t.is_null() {
            return 0 as *mut libc::c_void as *mut *mut libc::c_char;
        }
        lhs_l = t.offset_from(text) as libc::c_long as libc::c_int;
        lhs = substring(text, 0 as libc::c_int, lhs_l);
        rhs = substring(
            text,
            (lhs_l as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            tlen as libc::c_int,
        );
        if *lhs.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
            || *rhs.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
        {
            libc::free(lhs as *mut libc::c_void);
            libc::free(rhs as *mut libc::c_void);
            return 0 as *mut libc::c_void as *mut *mut libc::c_char;
        }
    }

    lhs_t = if legal_number(lhs, &mut tl) != 0 {
        ST_INT!()
    } else {
        // SAFETY: 字符检查
        unsafe {
            if ISALPHA!(*lhs)
                && *lhs.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
            {
                ST_CHAR!() as libc::c_int
            } else {
                ST_BAD!() as libc::c_int
            }
        }
    };

    ep = 0 as *mut libc::c_char;

    // SAFETY: 字符串解析
    unsafe {
        if ISDIGIT!(*rhs)
            || ((*rhs as libc::c_int == '+' as libc::c_int
                || *rhs as libc::c_int == '-' as libc::c_int)
                && ISDIGIT!(*rhs.offset(1 as isize)))
        {
            rhs_t = ST_INT!() as libc::c_int;
            errno = 0 as libc::c_int;
            tr = strtoimax(rhs, &mut ep, 10 as libc::c_int);
            if errno == ERANGE!()
                || !ep.is_null()
                    && *ep as libc::c_int != 0 as libc::c_int
                    && *ep as libc::c_int != '.' as i32
            {
                rhs_t = ST_BAD!() as libc::c_int;
            }
        } else if ISALPHA!(*rhs) && *rhs.offset(1 as isize) == 0
            || *rhs.offset(1 as isize) as libc::c_int == '.' as libc::c_int
        {
            rhs_t = ST_CHAR!();
            ep = rhs.offset(1 as libc::c_int as isize);
        } else {
            rhs_t = ST_BAD!();
            ep = 0 as *mut libc::c_char;
        }
        incr = 1 as libc::c_int as intmax_t;
        if rhs_t != ST_BAD!() {
            oep = ep;
            errno = 0 as libc::c_int;

            if !ep.is_null()
                && *ep as libc::c_int == '.' as i32
                && *ep.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
                && *ep.offset(2 as libc::c_int as isize) as libc::c_int != 0
            {
                incr = strtoimax(
                    ep.offset(2 as libc::c_int as isize),
                    &mut ep,
                    10 as libc::c_int,
                );
            }
            if *ep as libc::c_int != 0 as libc::c_int || errno == ERANGE!() {
                rhs_t = ST_BAD!();
            }
            tlen = (tlen as usize).wrapping_sub(ep.offset_from(oep) as libc::c_long as usize)
                as size_t as size_t;
        }

        if lhs_t != rhs_t || lhs_t == ST_BAD!() as libc::c_int || rhs_t == ST_BAD!() as libc::c_int
        {
            libc::free(lhs as *mut libc::c_void);
            libc::free(rhs as *mut libc::c_void);
            return 0 as *mut libc::c_void as *mut *mut libc::c_char;
        }

        if lhs_t == ST_CHAR!() as libc::c_int {
            lhs_v = *lhs.offset(0 as libc::c_int as isize) as libc::c_uchar as intmax_t;
            rhs_v = *rhs.offset(0 as libc::c_int as isize) as libc::c_uchar as intmax_t;
            width = 1 as libc::c_int;
        } else {
            lhs_v = tl;
            rhs_v = tr;
            rhs_l = tlen as libc::c_int
                - lhs_l as libc::c_int
                - std::mem::size_of::<[libc::c_char; 3]>() as libc::c_int
                + 1 as libc::c_int;

            width = 0;
            if lhs_l > 1 as libc::c_int
                && *lhs.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
            {
                width = lhs_l;
                lhs_t = ST_ZINT!();
            }
            if lhs_l > 2 as libc::c_int
                && *lhs.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
                && *lhs.offset(1 as libc::c_int as isize) as libc::c_int == '0' as i32
            {
                width = lhs_l;
                lhs_t = ST_ZINT!();
            }
            if rhs_l > 1 as libc::c_int
                && *rhs.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
                && width < rhs_l
            {
                width = rhs_l;
                lhs_t = ST_ZINT!();
            }
            if rhs_l > 2 as libc::c_int
                && *rhs.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
                && *rhs.offset(1 as libc::c_int as isize) as libc::c_int == '0' as i32
                && width < rhs_l
            {
                width = rhs_l;
                lhs_t = ST_ZINT!();
            }
            if width < lhs_l && lhs_t == ST_ZINT!() {
                width = lhs_l;
            }
            if width < rhs_l && lhs_t == ST_ZINT!() {
                width = rhs_l;
            }
        }
        result = mkseq(lhs_v, rhs_v, incr, lhs_t, width);
        libc::free(lhs as *mut libc::c_void);
        libc::free(rhs as *mut libc::c_void);
    }

    return result;
}

fn brace_gobbler(
    text: *mut libc::c_char,
    _tlen: size_t,
    indx: *mut libc::c_int,
    satisfy: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int;
    let mut c: libc::c_int;
    let mut quoted: libc::c_int;
    let mut level: libc::c_int;
    let mut commas: libc::c_int;
    let mut pass_next: libc::c_int;
    let mut si: libc::c_int;
    let mut t: *mut libc::c_char;
    let mut Flag: bool;

    let mut state: mbstate_t = mbstate_t {
        __count: 0,
        __value: mbstate_t_value { __wch: 0 },
    };

    // SAFETY: 内存初始化
    unsafe {
        libc::memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\0' as i32,
            std::mem::size_of::<mbstate_t>() as usize,
        );
    }

    pass_next = 0 as libc::c_int;
    quoted = pass_next;
    level = quoted;
    commas = if satisfy == '}' as i32 {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };

    // SAFETY: 指针读取
    unsafe {
        i = *indx;
    }

    'outer: loop {
        Flag = false;
        // SAFETY: 字符串遍历和处理
        unsafe {
            c = *text.offset(i as isize) as libc::c_int;
        }
        if c == 0 {
            break 'outer;
        }
        if pass_next != 0 {
            pass_next = 0 as libc::c_int;
            ADVANCE_CHAR_1!(text, tlen, i);
            continue 'outer;
        }
        if c == '\\' as i32 && (quoted == 0 || quoted == '"' as i32 || quoted == '`' as i32) {
            pass_next = 1;
            i += 1;
            continue 'outer;
        }
        // SAFETY: 字符检查
        unsafe {
            if c == '$' as i32
                && *text.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '{' as i32
                && quoted != '\'' as i32
            {
                pass_next = 1 as libc::c_int;
                i += 1;
                if quoted == 0 as libc::c_int {
                    level += 1;
                }
                continue 'outer;
            }
        }
        'inner: loop {
            if quoted != 0 {
                if c == quoted {
                    quoted = 0 as libc::c_int;
                }
                // SAFETY: 字符检查
                unsafe {
                    if quoted == '"' as i32
                        && c == '$' as i32
                        && *text.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                            == '(' as i32
                    {
                        Flag = true;
                        break 'inner;
                    }
                }
                ADVANCE_CHAR_1!(text, tlen, i);
                continue 'outer;
            }
            if c == '"' as libc::c_int || c == '\'' as libc::c_int || c == '`' as libc::c_int {
                quoted = c;
                i += 1;
                continue 'outer;
            // SAFETY: 字符检查和命令替换提取
            } else {
                unsafe {
                    if (c == '$' as libc::c_int
                        || c == '<' as libc::c_int
                        || c == '>' as libc::c_int)
                        && *text.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                            == '(' as libc::c_int
                    {
                        si = i + 2 as libc::c_int;
                        t = extract_command_subst(text, &mut si, 0 as libc::c_int);
                        i = si;
                        libc::free(t as *mut libc::c_void);
                        i += 1;
                        continue 'outer;
                    }
                }
            }
            break 'inner;
        }

        if Flag {
            // SAFETY: 命令替换提取
            unsafe {
                si = i + 2 as libc::c_int;
                t = extract_command_subst(text, &mut si, 0 as libc::c_int);
                i = si;
                libc::free(t as *mut libc::c_void);
            }
            i += 1;
            continue 'outer;
        }

        // SAFETY: 大括号匹配逻辑
        unsafe {
            if c == satisfy
                && level == 0 as libc::c_int
                && quoted == 0 as libc::c_int
                && commas > 0 as libc::c_int
            {
                /* We ignore an open brace surrounded by whitespace, and also
                an open brace followed immediately by a close brace preceded
                by whitespace.  */
                if c == '{' as libc::c_int
                    && ((i == 0
                        || brace_whitespace!(*text.offset((i - 1 as libc::c_int) as isize)))
                        && (brace_whitespace!(*text.offset((i + 1 as libc::c_int) as isize))
                            || *text.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                                == '}' as i32))
                {
                    i += 1;
                    continue 'outer;
                }
                break 'outer;
            }

            if c == '{' as i32 {
                level += 1;
            } else if c == '}' as i32 && level != 0 {
                level -= 1;
            } else if satisfy == '}' as i32 && c == brace_arg_separator && level == 0 as libc::c_int
            {
                commas += 1;
            } else if satisfy == '}' as i32
                && STREQN(
                    text.offset(i as libc::c_int as isize),
                    BRACE_SEQ_SPECIFIER!(),
                    2,
                )
                && *text.offset((i + 2 as libc::c_int) as isize) as libc::c_int != satisfy
                && level == 0 as libc::c_int
            {
                commas += 1;
            }
        }
        ADVANCE_CHAR_1!(text, tlen, i);
    }

    // SAFETY: 写回索引
    unsafe {
        *indx = i;
    }

    return c;
}

fn array_concat(
    arr1: *mut *mut libc::c_char,
    arr2: *mut *mut libc::c_char,
) -> *mut *mut libc::c_char {
    let mut i: libc::c_int;
    let mut j: libc::c_int;
    let mut len: libc::c_int;
    let len1: libc::c_int;
    let len2: libc::c_int;
    let result: *mut *mut libc::c_char;

    if arr1.is_null() {
        return arr2;
    }

    if arr2.is_null() {
        return arr1;
    }

    // SAFETY: 所有数组和字符串操作
    unsafe {
        if !(*arr1.offset(0 as libc::c_int as isize)).is_null()
            && *(*arr1.offset(0 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int
                == 0 as libc::c_int
            && (*arr1.offset(1 as libc::c_int as isize)).is_null()
        {
            c_strvec_dispose(arr1);
            return arr2;
        }
        if !(*arr2.offset(0 as libc::c_int as isize)).is_null()
            && *(*arr2.offset(0 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int
                == 0 as libc::c_int
            && (*arr2.offset(1 as libc::c_int as isize)).is_null()
        {
            return arr1;
        }

        len1 = c_strvec_len(arr1);
        len2 = c_strvec_len(arr2);
        result = libc::malloc(
            ((1 as libc::c_int + len1 * len2) as usize)
                .wrapping_mul(std::mem::size_of::<*mut libc::c_char>() as usize),
        ) as *mut *mut libc::c_char;
        if result.is_null() {
            return result;
        }

        len = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < len1 {
            let strlen_1: libc::c_int = libc::strlen(*arr1.offset(i as isize)) as libc::c_int;
            j = 0 as libc::c_int;
            while j < len2 {
                *result.offset(len as isize) = libc::malloc(
                    (1 as libc::c_int
                        + strlen_1
                        + libc::strlen(*arr2.offset(j as isize)) as libc::c_int)
                        as size_t as usize,
                ) as *mut libc::c_char;

                libc::strcpy(*result.offset(len as isize), *arr1.offset(i as isize));
                libc::strcpy(
                    (*result.offset(len as isize)).offset(strlen_1 as isize),
                    *arr2.offset(j as isize),
                );
                len += 1;
                j += 1;
            }
            libc::free(*arr1.offset(i as isize) as *mut libc::c_void);
            i += 1
        }
        libc::free(arr1 as *mut libc::c_void);
        *result.offset(len as isize) = 0 as *mut libc::c_void as *mut libc::c_char;
    }
    return result;
}
