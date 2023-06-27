#[macro_use]
#[warn(temporary_cstring_as_ptr)]
extern crate  libc;
extern crate nix;

use std::ffi::CString;
use libc::c_long;
#[repr(C)]
pub struct WORD_DESC {
    pub word: *mut libc::c_char,
    pub flags: libc::c_int
}

#[repr (C)]
#[derive(Copy,Clone)]
pub struct WORD_LIST {
    next: *mut WORD_LIST,
    word: *mut WORD_DESC
}

// 屏蔽警告。
#[allow(non_camel_case_types)]
type intmax_t = c_long;

#[macro_export]
macro_rules! EXECUTION_SUCCESS {
   () => {0}
}

#[macro_export]
macro_rules! EXECUTION_FAILURE {
   () => {1}
}

#[macro_export]
macro_rules! EX_USAGE {
   () => {258}
}

#[macro_export]
macro_rules! ISHELP {
   ($s:expr) => {
    libc::strcmp($s as *const libc::c_char,CString::new("--help").unwrap().as_ptr())
    }
}

#[macro_export]
macro_rules! CHECK_HELPOPT {
  ($l:expr) => {
    if $l  !=std::ptr::null_mut() && (*$l).word !=std::ptr::null_mut() && ISHELP!((*(*$l).word).word) == 0 {
      builtin_help ();
      return EX_USAGE!();
    }
  }
}

extern "C" {
    fn get_numeric_arg(list :*mut WORD_LIST, i: i32 , intmax :*mut intmax_t) -> i32;
    fn builtin_help ();
    fn get_loop_level() -> i32;
    fn set_continuing(cont : i32);
    fn set_breaking(breaking : i32);
    fn sh_erange (s:* mut libc::c_char, desc:* mut libc::c_char);
    //pub static  fn  check_loop_level () -> i64;
    /* Non-zero when a "break" instruction is encountered. */
    pub static  posixly_correct :i32;
}

#[no_mangle]
pub extern "C" fn r_break_builtin(mut list :*mut WORD_LIST) -> i32 {
    //println!("enter r_break_builtin");
    let  mut  newbreak : intmax_t = 1 as intmax_t;
    unsafe {
        CHECK_HELPOPT! (list);
    if check_loop_level() == 0 {
        return (EXECUTION_SUCCESS!());
    }
        get_numeric_arg(list, 1, &mut newbreak as *mut intmax_t);

    if newbreak <= 0{
        #[warn(temporary_cstring_as_ptr)]
            sh_erange ((*(*list).word).word, CString::new("loop count").unwrap().as_ptr() as * mut libc::c_char);
            set_breaking (get_loop_level());
      return (EXECUTION_FAILURE!());
    }

  if newbreak > get_loop_level()  as  libc::c_long{ 
    newbreak = get_loop_level() as i64;
  }

  set_breaking(newbreak as i32);
  }
  return (EXECUTION_SUCCESS!());
}

fn continue_builtin (list :*mut WORD_LIST) -> i32 {
    let mut newcont : intmax_t = 0 as intmax_t;
    unsafe {
        CHECK_HELPOPT! (list);
    }
    if check_loop_level() == 0 {
        return (EXECUTION_SUCCESS!());
    }
    unsafe {
        get_numeric_arg(list, 1, newcont  as *mut intmax_t);
    }
    unsafe {
    if newcont <= 0{
        #[warn(temporary_cstring_as_ptr)]
        sh_erange ((*(*list).word).word, CString::new("loop count ").unwrap().as_ptr() as * mut libc::c_char);
        set_breaking(get_loop_level());
      return (EXECUTION_FAILURE!());
    }
   if newcont > get_loop_level().into(){
      newcont = get_loop_level() as i64;
    }
    set_continuing(newcont as i32);

    }
    return (EXECUTION_SUCCESS!());
}

#[no_mangle]
pub extern "C" fn check_loop_level () -> i32 {
unsafe { 
  if get_loop_level()== 0 &&  posixly_correct == 0 {
      println! ("only meaningful in a `for`, `while`, or until `loop` ");
      return 0;
  }
  return (get_loop_level());
}
}

