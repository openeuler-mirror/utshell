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

