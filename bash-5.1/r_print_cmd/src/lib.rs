use libc::{c_int,c_char,FILE, c_void, fprintf,fileno, strnlen, size_t};
use std::ffi::{CString, CStr, };

use r_bash::*;extern "C"{
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
}

pub const  FUNC_MULTILINE:i32 = 0x01;
pub const  FUNC_EXTERNAL:i32 = 0x02;
pub static mut indentation:c_int = 0;
pub static mut indentation_amount:c_int = 4;
pub const PRINTED_COMMAND_INITIAL_SIZE:c_int = 64;
pub const PRINTED_COMMAND_CROW_SIZE:c_int = 128;
static mut inside_function_def:c_int = 0;
static mut skip_this_indent:c_int = 0;
static mut was_heredoc:c_int = 0;
static mut printing_connection:c_int = 0;
static mut deferred_heredocs:*mut REDIRECT = 0 as *mut REDIRECT;
static mut group_command_nesting:c_int = 0;
static mut indirection_string:*mut c_char = 0 as *mut c_char;
static mut indirection_stringsiz:c_int = 0;

const MB_LEN_MAX:usize = 16; //在C端这是一个宏，
const MB_CUR_MAX:usize  =  1;
const AND_AND:i32 = 288;
const OR_OR:i32 = 289;

/* 宏定义 */
#[macro_export]
macro_rules! CHECK_XTRACE_FP{
    () => (
        if !xtrace_fp.is_null(){
            xtrace_fp = xtrace_fp;
        } 
        else{
            xtrace_fp = stderr;
        }
    )
}

#[macro_export]
macro_rules! EXPCHAR{
    ($c:expr) => (
        if $c == b'{' as c_char || $c == b'~' as c_char || $c == b'$' as c_char || $c == b'`' as c_char{
            1
        }
        else{
            0
        }
    )
}

#[macro_export]
macro_rules! PRINT_DEFERRED_HEREDOCS{
    ($x:expr) => (
        if !deferred_heredocs.is_null(){
            print_deferred_heredocs($x);
        }
    )
}

#[macro_export]
macro_rules! RESIZE_MALLOCED_BUFFER{
    ($str:expr, $cind:expr, $room:expr, $csize:expr, $simcr:expr) => (
        if $cind + $room >= $csize {
            while $cind + $room >= $csize {
                $csize = $csize + $simcr;
            }
            $str = libc::realloc($str as *mut c_void, $csize as usize ) as *mut c_char;
        }
    )

}

pub  fn MBLEN(s: *const c_char ,n:size_t) -> c_int
{
    if MB_CUR_MAX > 1
    {
        return unsafe { mblen(s,n) };
    }
    else {
        return  1;
    }
}

unsafe fn DIGIT(c:c_char) -> bool{
    char::from(c as u8 ) >= '0' && char::from(c as u8) <= '9'
}

