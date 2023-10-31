use libc::*;
use r_bash::*;
use rcommon::WordList as WORD_LIST;
use rcommon::WordDesc as WORD_DESC;

pub type sh_free_func_t = unsafe extern "C" fn(*mut libc::c_void) -> ();

#[macro_export] 
macro_rules! assoc_empty {
    ($h:expr) =>{
        (*$h).nentries == 0 as libc::c_int
    }
}

#[macro_export]
macro_rules! savestring {
    ($name:expr) => {
        libc::strcpy(
            malloc((1 as libc::c_int + libc::strlen($name) as libc::c_int) as size_t) as *mut libc::c_char,
            $name )
    }
}

#[macro_export]
macro_rules! ALL_ELEMENT_SUB {
    ($c:expr) => {
        $c == '@' as i32 || $c == '*' as i32
    }
}


