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
macro_rules! ADVANCE_CHAR {
    ($str:expr, $strsize:expr, $i:expr) => {
        $i += 1;
    }
}

#[macro_export]
macro_rules! brace_whitespace{
    ($c:expr) => {
        $c == 0
        || $c as libc::c_int == ' ' as i32
        || $c  as libc::c_int == '\t' as i32
        || $c as libc::c_int == '\n' as i32
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

unsafe extern "C" fn expand_seqterm(
    mut text: *mut libc::c_char,
    mut tlen: size_t,
) -> *mut *mut libc::c_char {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lhs: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rhs: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lhs_t: libc::c_int = 0;
    let mut rhs_t: libc::c_int = 0;
    let mut lhs_l: libc::c_int = 0;
    let mut rhs_l: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut lhs_v: intmax_t = 0;
    let mut rhs_v: intmax_t = 0;
    let mut incr: intmax_t = 0;
    let mut tl: intmax_t = 0;
    let mut tr: intmax_t = 0;
    let mut result: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut ep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut oep: *mut libc::c_char = 0 as *mut libc::c_char;

    t = libc::strstr(text, b"..\0" as *const u8 as *const libc::c_char);
    
    if t.is_null() {
        return 0 as *mut libc::c_void as *mut *mut libc::c_char;
    }
    lhs_l = t.offset_from(text) as libc::c_long as libc::c_int;
    
    lhs = substring(text, 0 as libc::c_int, lhs_l);
    rhs = substring(
        text,
        (lhs_l + std::mem::size_of::<[libc::c_char; 3]> as libc::c_int - 1 as libc::c_int ),
        tlen as libc::c_int,
    );    
    if *lhs.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
        || *rhs.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
    {
        libc::free(lhs as *mut libc::c_void);
        libc::free(rhs as *mut libc::c_void);
        return 0 as *mut libc::c_void as *mut *mut libc::c_char;
    }

    lhs_t = if legal_number(lhs, &mut tl) != 0 {
       ST_INT!()
    } else if ISALPHA!(*lhs)
        && *lhs.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
    {
        ST_CHAR!() as libc::c_int
    } else {
        ST_BAD!() as libc::c_int
    };

    ep = 0 as *mut libc::c_char;

    if ISDIGIT!(*rhs) || ((*rhs as libc::c_int == '+' as libc::c_int 
    || *rhs as libc::c_int  == '-' as libc::c_int ) 
    && ISDIGIT! (*rhs.offset(1 as isize)))
    {
        rhs_t = ST_INT!() as libc::c_int;
        errno = 0 as libc::c_int;
        tr = strtoimax(rhs, &mut ep, 10 as libc::c_int);
        if errno == ERANGE!()
            || !ep.is_null() && *ep as libc::c_int != 0 as libc::c_int
                && *ep as libc::c_int != '.' as i32
        {
            rhs_t = ST_BAD!() as libc::c_int;
        }

    }
    else if ISALPHA!(*rhs) && *rhs.offset(1 as isize) == 0 
    || *rhs.offset(1 as isize) as libc::c_int == '.' as libc::c_int
    {
      rhs_t = ST_CHAR!();
      ep = rhs.offset(1 as libc::c_int as isize);
    }
    else {
      rhs_t = ST_BAD!();
      ep = 0 as *mut libc::c_char;
    }
    incr = 1 as libc::c_int as intmax_t;
    if rhs_t != ST_BAD!() {
        oep = ep;
        errno = 0 as libc::c_int;

        if !ep.is_null() && *ep as libc::c_int == '.' as i32
            && *ep.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
            && *ep.offset(2 as libc::c_int as isize) as libc::c_int != 0
        {
            incr = strtoimax(
                ep.offset(2 as libc::c_int as isize),
                &mut ep,
                10 as libc::c_int,
            );
        }
        if *ep as libc::c_int != 0 as libc::c_int
            || errno == ERANGE!() 
        {
            rhs_t = ST_BAD!();
        }
        tlen = (tlen as usize)
            .wrapping_sub(ep.offset_from(oep) as libc::c_long as usize) as size_t
            as size_t;
    }

    if lhs_t != rhs_t || lhs_t == ST_BAD!() as libc::c_int || rhs_t == ST_BAD!() as libc::c_int {
        libc::free(lhs as *mut libc::c_void);
        libc::free(rhs as *mut libc::c_void);
        return 0 as *mut libc::c_void as *mut *mut libc::c_char;
    }

    if lhs_t == ST_CHAR!() as libc::c_int {
        lhs_v = *lhs.offset(0 as libc::c_int as isize) as libc::c_uchar as intmax_t;
        rhs_v = *rhs.offset(0 as libc::c_int as isize) as libc::c_uchar as intmax_t;
        width = 1 as libc::c_int;
    }

    else {
        lhs_v = tl;
        rhs_v = tr;
        rhs_l = tlen as libc::c_int - lhs_l as libc::c_int- 
        std::mem::size_of::<[libc::c_char; 3]>() as libc::c_int 
        + 1 as libc::c_int;

        width = 0;
        if lhs_l > 1 as libc::c_int
            && *lhs.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
        {
            width = lhs_l;
            lhs_t = ST_ZINT!() ;
        }
        if lhs_l > 2 as libc::c_int
            && *lhs.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
            && *lhs.offset(1 as libc::c_int as isize) as libc::c_int == '0' as i32
        {
            width = lhs_l;
            lhs_t = ST_ZINT!() ;
        }
        if rhs_l > 1 as libc::c_int
            && *rhs.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
            && width < rhs_l
        {
            width = rhs_l;
            lhs_t = ST_ZINT!() ;
        }
        if rhs_l > 2 as libc::c_int
            && *rhs.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
            && *rhs.offset(1 as libc::c_int as isize) as libc::c_int == '0' as i32
            && width < rhs_l
        {
            width = rhs_l;
            lhs_t =  ST_ZINT!() ;
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
    return result;
}

unsafe extern "C" fn brace_gobbler(
    mut text: *mut libc::c_char,
    mut tlen: size_t,
    mut indx: *mut libc::c_int,
    mut satisfy: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut quoted: libc::c_int = 0;
    let mut level: libc::c_int = 0;
    let mut commas: libc::c_int = 0;
    let mut pass_next: libc::c_int = 0;
    let mut si: libc::c_int = 0;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut Flag  = false;

    let mut state: mbstate_t = mbstate_t {
        __count: 0,
        __value: __mbstate_t__bindgen_ty_1 { __wch: 0 },
    };
    libc::memset(
        &mut state as *mut mbstate_t as *mut libc::c_void,
        '\0' as i32,
        std::mem::size_of::<mbstate_t>() as usize,
    );

    pass_next = 0 as libc::c_int;
    quoted = pass_next;
    level = quoted;
    commas = if satisfy == '}' as i32 
    { 
        0 as libc::c_int 
    } 
    else 
    { 
        1 as libc::c_int 
    };
    i = *indx;

    'outer: loop {
        Flag = false;
        c = *text.offset(i as isize) as libc::c_int;
        if (c == 0) {

            break 'outer;
        }
        if pass_next != 0 {
            pass_next = 0 as libc::c_int;
            ADVANCE_CHAR!(text, tlen, i);
            continue 'outer;
	    }
        if c == '\\'  as i32
        && (quoted == 0 || quoted == '"' as i32
         || quoted == '`'  as i32
        ){

            pass_next = 1;
            i += 1;
            continue 'outer;
	    } 
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
        'inner: loop { 
            if quoted != 0 {
                if c == quoted {
                    quoted = 0 as libc::c_int;
                }
                if quoted == '"' as i32 && c == '$' as i32
                && *text.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                    == '(' as i32
                {
                    Flag = true;
                    break 'inner;
                }
                ADVANCE_CHAR!(text, tlen, i);
                continue 'outer;
            }
            if c == '"' as libc::c_int 
            || c == '\'' as libc::c_int 
            || c == '`' as libc::c_int
            {
              quoted = c;
              i+= 1;
              continue 'outer;
            }
            else if (c == '$' as libc::c_int 
              || c == '<' as libc::c_int 
              || c == '>' as libc::c_int ) &&
              *text.offset((i + 1 as libc::c_int) as isize) as libc::c_int
              == '(' as libc::c_int 		/* ) */
            {
                si = i + 2 as libc::c_int;
                t = extract_command_subst(text, &mut si, 0 as libc::c_int);
                i = si;
                libc::free(t as *mut libc::c_void);
                i += 1;
                continue 'outer;
            }
            break 'inner;
        }
       
        if  Flag {
            si = i + 2 as libc::c_int;
            t = extract_command_subst(text, &mut si, 0 as libc::c_int);
            i = si;
            libc::free(t as *mut libc::c_void);
            i += 1;
            continue 'outer;
        }

        if (c == satisfy
            && level == 0 as libc::c_int 
            && quoted == 0 as libc::c_int 
            && commas > 0 as libc::c_int)
        {
            /* We ignore an open brace surrounded by whitespace, and also
                an open brace followed immediately by a close brace preceded
                by whitespace.  */
            if c == '{' as libc::c_int &&
                ((i == 0
                || brace_whitespace!(*text.offset((i - 1 as libc::c_int) as isize)))
                &&
                (brace_whitespace!(*text.offset((i + 1 as libc::c_int) as isize))
                || *text.offset((i + 1 as libc::c_int) as isize)
                as libc::c_int == '}' as i32 ))
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
        } else if satisfy == '}' as i32 && c == brace_arg_separator
            && level == 0 as libc::c_int
        {
            commas += 1;
        } else if satisfy == '}' as i32 
        && STREQN(text.offset(i as libc::c_int as isize) , BRACE_SEQ_SPECIFIER!(), 2)  
        && *text.offset((i + 2 as libc::c_int) as isize) as libc::c_int != satisfy
        && level == 0 as libc::c_int {
            commas += 1;
        }
        ADVANCE_CHAR!(text, tlen, i);
    }

    *indx = i;
    return c;
}

