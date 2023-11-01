use libc::*;
use r_bash::*;

pub type __intmax_t = libc::c_long;
pub type intmax_t = __intmax_t;
// pub type mbstate_t = __mbstate_t;

pub const _ISdigit: libc::c_uint = 2048;
pub const _ISalpha: libc::c_uint = 1024;
const CHAR_BIT : libc::c_int = 8 as libc::c_int;
static mut  errno :  libc::c_int = 0 as libc::c_int;

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


