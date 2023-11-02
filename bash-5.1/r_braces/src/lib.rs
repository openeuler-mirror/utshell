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
#[macro_export] 
macro_rules! ST_CHAR {
    () =>{
        2 as libc::c_int
    }
}

#[macro_export] 
macro_rules! ST_ZINT {
    () =>{
        3 as libc::c_int
    }
}

#[macro_export] 
macro_rules! INTMAX_MIN{
    () =>{
        -(9223372036854775807 as libc::c_long) - 1 as libc::c_int as libc::c_long
    }
}

#[macro_export] 
macro_rules! INTMAX_MAX{
    () =>{
        9223372036854775807 as libc::c_long - 2 as libc::c_int as libc::c_long
    }
}

#[macro_export] 
macro_rules! ERANGE{
    () =>{
        34 as libc::c_int
    }
}
#[macro_export] 
macro_rules! BRACE_SEQ_SPECIFIER{
    () =>{
       b"..\0"  as *const u8 as *const libc::c_char
    }
}

#[macro_export] 
macro_rules! INT_MAX{
    () =>{
        2147483647 as libc::c_int as libc::c_long
    }
}

#[macro_export] 
macro_rules! INT_MIN{
    () =>{
        (-(2147483647 as libc::c_int) - 1 as libc::c_int ) as libc::c_long
    }
}

#[macro_export] 
macro_rules! sh_imaxabs{
    ($x:expr) =>{
        if $x >= 0 as libc::c_int as libc::c_long {
            $x
        } else {
           -$x
        }       
    }
}

#[macro_export]
macro_rules! savestring {
    ($name:expr) => {
        libc::strcpy(
            xmalloc((1 as libc::c_int + libc::strlen($name) as libc::c_int) as size_t) as *mut libc::c_char,
            $name )
    }
}

#[macro_export]
macro_rules! SIZEOF_fUNC {
    ($t:ty) => {
        std::mem::size_of::<$t>() 
    };
}

#[macro_export]
macro_rules! TYPE_WIDTH {
    ($t:ty) => {
        (SIZEOF_fUNC!($t) * CHAR_BIT as usize)  as $t
    }
}

#[macro_export]
macro_rules! TYPE_SIGNED {
    ($t:ty) => {
        if 0 as $t < (-1) as libc::c_int as $t {
            0 as $t
        }
        else {
            1 as $t
        } 
    }
}

#[macro_export]
macro_rules! ISALPHA{
    ($c:expr) => {
        IN_CTYPE_DOMAIN!($c) && isalpha!($c) != 0 as libc::c_int
    }
}

#[macro_export]
macro_rules! ISDIGIT{
    ($c:expr) => {
        IN_CTYPE_DOMAIN!($c) && isdigit!($c) != 0 as libc::c_int
    }
}

#[macro_export]
macro_rules! isalpha{
    ($c:expr) => {
        __isctype_f!($c,_ISalpha)
    }
}

#[macro_export]
macro_rules! isdigit{
    ($c:expr) => {
        __isctype_f!($c,_ISdigit)
    }
}

#[macro_export]
macro_rules! __isctype_f{
    ($c:expr,$type:expr) => {
        *(*__ctype_b_loc()).offset($c as libc::c_int as isize) as libc::c_int
        & ($type as libc::c_int as libc::c_ushort as libc::c_int)
    }
}



#[macro_export]
macro_rules! IN_CTYPE_DOMAIN{
    ($c:expr) => {
        1 as libc::c_int!= 0 as libc::c_int
    }
}

#[inline]
unsafe extern "C" fn mbrlen(
    mut __s: *const libc::c_char,
    mut __n: size_t,
    mut __ps: *mut mbstate_t,
) -> size_t {
    return if !__ps.is_null() {
        mbrtowc(0 as *mut libc::wchar_t, __s, __n, __ps)
    } else {
        __mbrlen(__s, __n, 0 as *mut mbstate_t)
    };
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

#[no_mangle]
pub unsafe extern "C" fn brace_expand(
    mut text: *mut libc::c_char,
) -> *mut *mut libc::c_char {
    let mut start: libc::c_int = 0;
    let mut tlen: size_t = 0;
    let mut preamble: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut postamble: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut amble: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut alen: size_t = 0;
    let mut tack: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut result: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut c1: libc::c_int = 0;
    /////////////////////////////////////////////////////////////

    let mut state: mbstate_t = mbstate_t {
        __count: 0,
        __value: __mbstate_t__bindgen_ty_1 { __wch: 0 },
    };
    libc::memset(
        &mut state as *mut mbstate_t as *mut libc::c_void,
        '\0' as i32,
        std::mem::size_of::<mbstate_t>() as usize,
    );

    tlen = libc::strlen(text);
    i = 0 as libc::c_int;

    loop {
  
        c = brace_gobbler(text, tlen, &mut i, '{' as i32);
        c1 = c ;
        if c != 0 {  
            j = i + 1 as libc::c_int;
            start = j;
            c = brace_gobbler(text, tlen, &mut j, '}' as i32);
            if c == 0 as libc::c_int {
                i += 1;
                c = c1;
                continue;
            }
            else 
            {
                c = c1;
                break;
            }
        }
        else {
            break;
        }
    }
    preamble = xmalloc((i + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    
    if i > 0 as libc::c_int {
        libc::strncpy(preamble, text, i as usize);
    }
    
    *preamble.offset(i as isize) = '\0' as i32 as libc::c_char;
    
    result = xmalloc(
        (2 as libc::c_int as size_t)
            *(std::mem::size_of::<*mut libc::c_char>() as size_t)
    ) as *mut *mut libc::c_char;

    *result.offset(0 as libc::c_int as isize) = preamble;
    *result.offset(1 as libc::c_int as isize) = 0 as *mut libc::c_void as *mut libc::c_char;
    
    if c != '{' as i32 {
        return result;
    }

    i += 1;
    start = i;

    c = brace_gobbler(text, tlen, &mut i, '}' as i32);
    if c == 0 as libc::c_int {
        libc::free(preamble as *mut libc::c_void);
        *result.offset(0 as libc::c_int as isize) =  savestring!(text);
        return result;
    }
    amble = substring(text, start, i);
    alen = (i - start) as size_t;

    libc::memset(
        &mut state as *mut mbstate_t as *mut libc::c_void,
        '\0' as i32,
        std::mem::size_of::<mbstate_t>() as size_t,
    );
    j = 0 as libc::c_int;

    loop  {
        if *amble.offset(j as isize) == 0 {
            break ;
        }
        if *amble.offset(j as isize) as libc::c_int == '\\' as i32 {
            j += 1;
            ADVANCE_CHAR!(amble, alen, j);
            continue ;       
        }
        if *amble.offset(j as isize) as libc::c_int == brace_arg_separator {
            break ;
        }
        ADVANCE_CHAR!(amble, alen, j);
    }
    loop  {
        if *amble.offset(j as isize) as libc::c_int == 0 as libc::c_int  {
            tack = expand_seqterm (amble, alen);
            if tack.is_null() {
                break;
            }
            else if !text.offset((i+1) as isize).is_null() {
                tack = strvec_create(2 as libc::c_int);
                *tack = savestring!(text
                    .offset((start-1) as isize) as *mut libc::c_char);
                *(*tack.offset((i-start+2) as isize)) = '\0' as i32 as libc::c_char;
                *tack.offset(1 as isize) = 0 as *mut libc::c_char;
                break;
            }
            else {
                libc::free(amble as *mut libc::c_void);
                libc::free(preamble as *mut libc::c_void);
                 *result.offset(0 as libc::c_int as isize) 
                 = libc::strcpy(
                    xmalloc(
                        (1 as libc::c_int as usize).wrapping_add(libc::strlen(text)),
                    ) as *mut libc::c_char,
                    text,
                );
                return result;
            }
        }
	    tack = expand_amble (amble, alen, 0);   
        break ;
    }
    result = array_concat(result, tack);
    libc::free(amble as *mut libc::c_void);
    if tack != result {
        strvec_dispose(tack);
    }
    postamble = text.offset(i as isize).offset(1 as libc::c_int as isize);
    if !postamble.is_null() && *postamble as libc::c_int != 0 {
        tack = brace_expand(postamble);
        result = array_concat(result, tack);
        if tack != result {
            strvec_dispose(tack);
        }
    }
    return result;
    
}

unsafe extern "C" fn expand_amble(
    mut text: *mut libc::c_char,
    mut tlen: size_t,
    mut flags: libc::c_int,
) -> *mut *mut libc::c_char {
    let mut result: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut partial: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut tresult: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut tem: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;

    let mut state: mbstate_t = mbstate_t {
        __count: 0,
        __value: __mbstate_t__bindgen_ty_1 { __wch: 0 },
    };
    libc::memset(
        &mut state as *mut mbstate_t as *mut libc::c_void,
        '\0' as i32,
        std::mem::size_of::<mbstate_t>() as usize,
    );

    result = 0 as *mut libc::c_void as *mut *mut libc::c_char;
    i = 0 as libc::c_int;
    start = i;
    c = 1 as libc::c_int;
    while c != 0 {
        c = brace_gobbler(text, tlen, &mut i, brace_arg_separator);
        tem = substring(text, start, i);
        partial = brace_expand(tem);
        if result.is_null() {
            result = partial;
        } else {
            let mut lr: libc::c_int = 0;
            let mut lp: libc::c_int = 0;
            let mut j: libc::c_int = 0;
            lr = strvec_len(result);
            lp = strvec_len(partial);
            tresult = strvec_mresize(result, lp + lr + 1 as libc::c_int);
            if tresult.is_null() {
                internal_error(
                b"brace expansion: cannot allocate memory for %s\0" as *const u8 as *mut libc::c_char,
                    tem
                );
                libc::free(tem as *mut libc::c_void);
                strvec_dispose(partial);
                strvec_dispose(result);
                result = 0 as *mut libc::c_void as *mut *mut libc::c_char;
                return result;
            } else {
                result = tresult;
            }
            j = 0 as libc::c_int;
            while j < lp {
               *result.offset((lr + j) as isize) = *partial.offset(j as isize);
                j += 1;
                j;
            }
            *result.offset((lr + j) as isize) = 0 as *mut libc::c_void as *mut libc::c_char;
            libc::free(partial as *mut libc::c_void);
        }
        libc::free(tem as *mut libc::c_void);
        if locale_mb_cur_max > 1 as libc::c_int {
            let mut state_bak: mbstate_t = mbstate_t {
                __count: 0,
                __value: __mbstate_t__bindgen_ty_1 { __wch: 0 },
            };
            let mut mblength: size_t = 0;
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
                    tlen.wrapping_sub(i as usize),
                    &mut state,
                );
            }
            if mblength == -(2 as libc::c_int) as size_t
                || mblength == -(1 as libc::c_int) as size_t
            {
                state = state_bak;
                i += 1;
                i;
            } else if mblength == 0 as libc::c_int as usize {
                i += 1;
                i;
            } else {
                i = (i as usize).wrapping_add(mblength) as libc::c_int
                    as libc::c_int;
            }
        } else {
            i += 1;
            i;
        }
        start = i;
    }
    return result;
}

unsafe extern "C" fn mkseq(
    mut start: intmax_t,
    mut end: intmax_t,
    mut incr: intmax_t,
    mut type_0: libc::c_int,
    mut width: libc::c_int,
) -> *mut *mut libc::c_char {
    let mut n: intmax_t = 0;
    let mut prevn: intmax_t = 0;
    let mut i: libc::c_int = 0;
    let mut nelem: libc::c_int = 0;
    let mut result: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    
    if incr == 0 as libc::c_int as libc::c_long {
        incr = 1 as libc::c_int as intmax_t;
    }
    
    if start > end && incr > 0 as libc::c_int as libc::c_long {
        incr = -incr;
    }
    else if start < end && incr < 0 as libc::c_int as libc::c_long {
        if incr == INTMAX_MIN!(){
            return 0 as *mut *mut libc::c_void as *mut *mut libc::c_char;
        }
        incr = -incr;
    } 
    if  SUBOVERFLOW!(end, start, INTMAX_MIN!() + 3, INTMAX_MAX!()-2)
    {
        return 0 as *mut libc::c_void as *mut *mut libc::c_char;
    }
    prevn = sh_imaxabs!(end - start);

    if INT_MAX!() == INTMAX_MAX!() && 
            ADDOVERFLOW!(prevn, 2, INT_MIN!(), INT_MAX!())
    {
        return 0 as *mut libc::c_void as *mut *mut libc::c_char;
    }
    else if ADDOVERFLOW!((prevn/sh_imaxabs!(incr)), 1, INTMAX_MIN!(), INTMAX_MAX!())
    {
        return 0 as *mut libc::c_void as *mut *mut libc::c_char;
    }

    if (prevn / sh_imaxabs!(incr)) > INT_MAX!() - 3 as libc::c_int as libc::c_long {
        return 0 as *mut libc::c_void as *mut *mut libc::c_char;
    } 
    nelem = (prevn / sh_imaxabs!(incr)) as libc::c_int + 1  as libc::c_int;
    result = strvec_mcreate (nelem + 1);

    if result.is_null() {
        internal_error(
        b"brace expansion: failed to allocate memory for %u elements\0"
            as *const u8 as *const libc::c_char,
            nelem as libc::c_uint,
        );
        return 0 as *mut libc::c_void as *mut *mut libc::c_char;
    }
    i = 0 as libc::c_int;
    n = start;
    loop {
        if interrupt_state != 0 as libc::c_int {
             *result.offset(i as isize) = 0 as *mut libc::c_void as *mut libc::c_char;
            strvec_dispose(result);
            result = 0 as *mut libc::c_void as *mut *mut libc::c_char;
        }
        if terminating_signal != 0 {
            termsig_handler(terminating_signal);
        }
        if interrupt_state != 0 {
            throw_to_top_level();
        }
        if type_0 == ST_INT!() {
            t = itos(n);
            *result.offset(i as isize) = t;
            i +=  1;
    
        }else if type_0 == ST_ZINT!() {
            let mut len: libc::c_int = 0;
            let mut arg: libc::c_int = 0;
            arg = n as libc::c_int;
            len = asprintf(
                &mut t as *mut *mut libc::c_char,
                b"%0*d\0" as *const u8 as *const libc::c_char,
                width,
                arg,
            );
            *result.offset(i as isize) = t;
            i +=  1;
        } else {
            t = xmalloc(2 as libc::c_int as usize) as *mut libc::c_char;
            if !t.is_null() {
                *t.offset(0 as libc::c_int as isize) = n as libc::c_char;
                *t.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            }
            *result.offset(i as isize) = t;
            i = i + 1;
        }
        if t.is_null() {
            let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut lbuf: [libc::c_char; INT_STRLEN_BOUND!(intmax_t) + 1 as usize] 
                    = [0;INT_STRLEN_BOUND!(intmax_t) + 1 as usize ];
            p = inttostr(
                n,
                lbuf.as_mut_ptr(),
                std::mem::size_of::<
                [libc::c_char;INT_STRLEN_BOUND!(intmax_t) + 1 as usize]>() as usize,
            );
            internal_error(
                b"brace expansion: failed to allocate memory for '%s'\0" as *const u8
                        as *const libc::c_char,
                p,
            );
            strvec_dispose(result);
            return 0 as *mut libc::c_void as *mut *mut libc::c_char;
        }
       
        if ADDOVERFLOW!(n, incr, INTMAX_MIN!(), INTMAX_MAX!()){
            break;
        }
        n += incr;
        if incr < 0 as libc::c_int as libc::c_long && n < end
        || incr > 0 as libc::c_int as libc::c_long && n > end
        {
            break;
        }
    }
    *result.offset(i as isize) =  0 as *mut libc::c_char;
    return (result);
}



