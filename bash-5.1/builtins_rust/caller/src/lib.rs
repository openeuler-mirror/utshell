extern crate libc;
extern crate rread;

use libc::{c_char,c_int,PT_NULL,c_long,};
use std::ffi::{CStr,CString};

use rread::{SHELL_VAR,ARRAY,intmax_t,};

//struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct word_desc {
    pub word: *mut c_char,
    pub flags: c_int,
}
pub type WORD_DESC = word_desc;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct word_list {
    pub next: *mut word_list,
    pub word: *mut WORD_DESC,
}
pub type WORD_LIST = word_list;


//enum

//macro
#[macro_export]
macro_rules! EXECUTION_SUCCESS {
  () => {
    0
  }
}

#[macro_export]
macro_rules! EXECUTION_FAILURE {
  () => {
    1
  }
}


#[macro_export]
macro_rules! EX_USAGE {
    () => {258}
}

#[macro_export]
macro_rules! att_array {
  () => {
    0x0000004 /* value is an array */
  }
}

#[macro_export]
macro_rules! att_cell {
  ($var:expr) => {  
    return (*($var).value) as *mut ARRAY;
  }
}

// #[macro_export]
// macro_rules! array_empty {
//   ($a:expr) => { 
//       if (*($a)).num_elements == 0{
//           return true;
//       }
//       else{
//           return false ;
//       }
//   }
// }

#[macro_export]
macro_rules! array_cell {
    ($var:expr) => {
        (*($var)).value as *mut ARRAY
        
    };
}

#[macro_export]
macro_rules! GET_ARRAY_FROM_VAR {
    ($n:expr,$v:expr,$a:expr) => {
        $v = find_variable($n);
        if ($v) != std::ptr::null_mut() && array_p($v) != 0{
            $a = array_cell!($v);
        }
        else{
            $a = 0 as *mut ARRAY;
        }
    }
}



#[macro_export]
macro_rules! CHECK_HELPOPT {
    ($l:expr) => {
        if $l != std::ptr::null_mut() && (*($l)).word != std::ptr::null_mut() && ISHELP((*(*($l)).word).word) == true{
            builtin_help();
            return EX_USAGE!();
        }
    };
}

// #[macro_export]
// macro_rules! ISHELP {
//     ($s:expr) => {
//         let s_str = CString::new("--help").unwrap().as_ptr();
//         STREQ( ($s),s_str);
//     };
// }



type arrayind_t = intmax_t;
//extern c
extern "C" {
    static loptend:*mut WORD_LIST;

    fn builtin_help();
    fn find_variable(str:*const c_char)->*mut SHELL_VAR;
    fn array_reference(a:*mut ARRAY,i:arrayind_t)->*mut c_char;
    fn builtin_usage();
    fn no_options(list:*mut WORD_LIST)->i32;
    fn legal_number(string:*mut c_char,result:*mut c_long)->i32;
    fn sh_invalidnum(s:*mut c_char);
}


unsafe fn STREQ(a:*const c_char,b:*const c_char)->bool{
    return *a == *b && libc::strcmp(a,b) == 0;
}
unsafe fn ISHELP(s:*const c_char)->bool{
    // let s_str = CString::new("--help").unwrap().as_ptr();
    return STREQ( s,CString::new("--help").unwrap().as_ptr());
}
unsafe fn array_p(var:*mut SHELL_VAR) ->i32 {
    return (*var).attributes & att_array!();
}
unsafe fn array_empty(a:*mut ARRAY)->bool{
  
    if (*a).num_elements == 0{
        return true;
    }
    else{
        return false ;
    }

}

//rust
#[no_mangle]
pub extern "C" fn r_caller_builtin(mut list:*mut WORD_LIST)->i32{
    let funcname_v:*mut SHELL_VAR ;
    let bash_source_v:*mut SHELL_VAR;
    let bash_lineno_v:*mut SHELL_VAR;
    let funcname_a:*mut ARRAY;
    let bash_source_a:*mut ARRAY;
    let bash_lineno_a:*mut ARRAY;
    let funcname_s:*mut c_char;
    let mut source_s:*mut c_char;
    let mut lineno_s:*mut c_char;
    let mut num:intmax_t = 0;

    let mut c_str :CString;

    println!("r_caller_builtin");

    unsafe{
        CHECK_HELPOPT!(list);

        let c_str1 = CString::new("FUNCNAME").unwrap();
        let c_ptr1 = c_str1.as_ptr();
        GET_ARRAY_FROM_VAR!(c_ptr1,funcname_v,funcname_a);
        // GET_ARRAY_FROM_VAR!(CString::new("FUNCNAME").unwrap().as_ptr(),funcname_v,funcname_a);

        let c_str1 = CString::new("BASH_SOURCE").unwrap();
        let c_ptr1 = c_str1.as_ptr();
        GET_ARRAY_FROM_VAR!(c_ptr1,bash_source_v,bash_source_a);
        // GET_ARRAY_FROM_VAR!(CString::new("BASH_SOURCE").unwrap().as_ptr(),bash_source_v,bash_source_a);

        let c_str1 = CString::new("BASH_LINENO").unwrap();
        let c_ptr1 = c_str1.as_ptr();
        GET_ARRAY_FROM_VAR!(c_ptr1,bash_lineno_v,bash_lineno_a);
        // GET_ARRAY_FROM_VAR!(CString::new("BASH_LINENO").unwrap().as_ptr(),bash_lineno_v,bash_lineno_a);

        if bash_lineno_a.is_null() || array_empty(bash_lineno_a){
            return EXECUTION_FAILURE!();
        }
    
        if bash_source_a.is_null() || array_empty(bash_source_a){
            return EXECUTION_FAILURE!();
        }
      
        if no_options(list) != 0{
            return EX_USAGE!();
        }

        list = loptend;     /* skip over possible `--' */
        /* If there is no argument list, then give short form: line filename. */
        if list.is_null(){
            lineno_s = array_reference(bash_lineno_a,0);
            source_s = array_reference(bash_source_a,1);
            
            if !lineno_s.is_null(){
                lineno_s = lineno_s;
            }
            else{
                c_str = CString::new("NULL").unwrap();
                lineno_s = c_str.as_ptr() as *mut c_char;
                // lineno_s = CString::new("NULL").unwrap().as_ptr() as *mut c_char;
            }

            if !source_s.is_null(){
                source_s = source_s;
            }
            else{
                c_str = CString::new("NULL").unwrap();
                source_s = c_str.as_ptr() as *mut c_char;
                // source_s = CString::new("NULL").unwrap().as_ptr() as *mut c_char;
            }
            let lineno_s_str = CStr::from_ptr(lineno_s).to_str().unwrap().to_owned();
            let source_s_str = CStr::from_ptr(source_s).to_str().unwrap().to_owned();
            println!("{} {}",lineno_s_str,source_s_str);

            return EXECUTION_SUCCESS!();   
        }

        if funcname_a.is_null() || array_empty(funcname_a) {
            return EXECUTION_FAILURE!();
        }
        if legal_number((*(*list).word).word,&mut num) != 0{
            lineno_s = array_reference(bash_lineno_a,num);
            source_s = array_reference(bash_source_a,num+1);
            funcname_s = array_reference(funcname_a,num+1);
            
            if lineno_s == PT_NULL as *mut c_char || source_s == PT_NULL as *mut c_char || funcname_s == PT_NULL as *mut c_char{
                return EXECUTION_FAILURE!();
            }
            let lineno_s_str = CStr::from_ptr(lineno_s).to_str().unwrap().to_owned();
            let funcname_s_str = CStr::from_ptr(funcname_s).to_str().unwrap().to_owned();
            let source_s_str = CStr::from_ptr(source_s).to_str().unwrap().to_owned();
            println!("{} {} {}",lineno_s_str,funcname_s_str,source_s_str);
        }
        else{
            sh_invalidnum((*(*list).word).word);
            builtin_usage();
            return EX_USAGE!();
        }

        return EXECUTION_SUCCESS!();
    }
}



/*
#ifdef LOADABLE_BUILTIN
static char *caller_doc[] = {
N_("Returns the context of the current subroutine call.\n\
    \n\
    Without EXPR, returns \"$line $filename\".  With EXPR, returns\n\
    \"$line $subroutine $filename\"; this extra information can be used to\n\
    provide a stack trace.\n\
    \n\
    The value of EXPR indicates how many call frames to go back before the\n\
    current one; the top frame is frame 0."),
  (char *)NULL
};

struct builtin caller_struct = {
        "caller",
        caller_builtin,
        BUILTIN_ENABLED,
        caller_doc,
        "caller [EXPR]",
        0
};

*/







#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
