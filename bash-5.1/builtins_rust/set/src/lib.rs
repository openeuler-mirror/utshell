extern crate libc;
extern crate nix;

use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;
use std::mem;
use std::io;
use rcommon::{WordList, WordDesc, EX_USAGE, EXECUTION_SUCCESS, EXECUTION_FAILURE, EX_NOTFOUND, EX_NOEXEC, SUBSHELL_PAREN,r_builtin_usage};

#[macro_export]
macro_rules! FLAG_UNKNOWN {
    () => {0 as *mut i32}
}

#[macro_export]
macro_rules! MINUS_O_FORMAT{
    () => {CString::new("%-15s\t%s\n")}
}

#[macro_export]
 macro_rules! GET_BINARY_O_OPTION_VALUE {
    ($a:expr,$b:expr) =>{
        if (o_options[$a as usize].get_func).is_some() {
          (Some(
            (o_options[$a as usize].get_func)
                .expect("non-null function pointer"),
        ))
            .expect("non-null function pointer")($b)
    } else {
        *o_options[$a as usize].variable
    }
  }
}

#[macro_export]
macro_rules! SET_BINARY_O_OPTION_VALUE {
  ($a:expr,$onoff:expr,$c:expr) =>{
    unsafe {
      if (o_options[ $a as usize].set_func).is_some() {
        (Some(
          (o_options[$a as usize].set_func)
              .expect("non-null function pointer"),
      ))
          .expect("non-null function pointer")($onoff, $c)
    }
    else {
         $onoff == FLAG_ON!();
        let b = $onoff;
        *o_options[$a as usize].variable = b;
        *o_options[$a as usize].variable
    }
  }
  }
}

#[macro_export]
macro_rules! N_O_OPTIONS {
  () => {
    (std::mem::size_of::<[opp;28]>() as usize
    / std::mem::size_of::<opp>() as usize)
  }
}

#[macro_export]
macro_rules! FLAG_ON{
  () =>{
    b'-' as i32
  }
}

#[macro_export]
macro_rules! FLAG_OFF{
  () =>{
    b'+' as i32
  }
}

#[macro_export]
macro_rules!  VUNSETATTR{
  ($var:expr,$attr:expr) => {
    (*$var).attributes = (*$var).attributes & !(&$attr);
    (*$var).attributes
  }
}

#[macro_export]
macro_rules! att_exported {
  () => {
    0x0000001
  }
}

#[macro_export]
macro_rules!  exported_p {
  ($var:expr) => {
    (*$var).attributes & att_exported!() 
  }
}

#[macro_export]
macro_rules! VSETATTR {
  ($var:expr,$attr:expr) => {
    (*$var).attributes = (*$var).attributes | (&$attr);
    (*$var).attributes
  }
}

#[macro_export]
macro_rules!  imported_p {
  ($var:expr) => {
    (*$var).attributes & att_imported!()
  }
}

#[macro_export]
macro_rules! att_imported {
  () => {
    0x0008000
  }
}

#[macro_export]
macro_rules!  att_assoc{
  () => {
    0x0000040
  }
}

#[macro_export]
macro_rules! assoc_p {
  ($var:expr) => {
    (*$var).attributes & att_assoc!()
  }
}

#[macro_export]
macro_rules! array_p {
  ($var:expr) => {
    (*$var).attributes & att_array!()
  }
}

#[macro_export]
macro_rules!  non_unsettable_p {
  ($var:expr) => {
    (*$var).attributes & att_nounset!()
  }
}

#[macro_export]
macro_rules! readonly_p {
  ($var:expr) => {
    (*$var).attributes & att_readonly!()
  }
}

#[macro_export]
macro_rules! nameref_p {
  ($var:expr) => {
    (*$var).attributes & att_nameref!()
  }
}

#[macro_export]
macro_rules! nameref_cell {
  ($var:expr) => {
    (*$var).value
  }
}

#[macro_export]
macro_rules! att_nameref{
  () => {
    0x0000800
  }
}

#[macro_export]
macro_rules! att_readonly{
  () => {
    0x0000002
  }
}

#[macro_export]
macro_rules! att_nounset {
  () => {
    0x0002000
  }
}

#[macro_export]
macro_rules! name_cell {
  ($var:expr) => {
    (*$var).name
  }
}

#[macro_export]
macro_rules!  att_array{
  () => {
    0x0000004
  }
}

#[macro_export]
macro_rules! savestring {
  ($x:expr) => {
    libc::strcpy(xmalloc ((1+ strlen ($x)) as u64) as *mut i8, $x)
  }
}

#[macro_export]
macro_rules! value_cell {
  ($var:expr) => {
    (*$var).value
  }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable {
    pub name: *mut i8,
    pub value: *mut i8,
    pub exportstr: *mut i8,
    pub dynamic_value: sh_var_value_func_t,
    pub assign_func: sh_var_assign_func_t,
    pub attributes: i32,
    pub context: i32,
}

#[derive(Copy, Clone)]
#[repr(C)]
 pub struct opp{
  name : *mut i8,
  letter : i32,
  variable : *mut i32,
  set_func : Option::<setopt_set_func_t>,
  get_func : Option::<setopt_get_func_t>,
}

// #[deny(missing_fragment_specifier)]
// #[macro_export]
// macro_rules! STREQ{
//    ($a:expr,$b:expr) =>{
//        (*$a==*$b) && (libc::strcmp($a,$b)==0)
//     }
// }


#[macro_export]
macro_rules! FLAG_ERROR{
  () => {-1}
}

#[macro_export]
macro_rules! VA_NOEXPAND {
  () => {0x001}
}

#[macro_export]
macro_rules! VA_ONEWORD {
  () => {0x001}
}

#[no_mangle]
pub static mut o_options : [opp ; 28] = unsafe {[  
    {
    opp{
        name : b"allexport\0" as *const u8 as *const i8 as *mut i8, 
        letter : b'a' as  i32,
        variable : 0 as *const libc::c_void 
           as *mut libc::c_void
           as  *mut i32,
        set_func : 
          ::std::mem::transmute::< 
          *mut libc::c_void,
          Option::<setopt_set_func_t>,
          > 
          (0 as *const libc::c_void as *mut libc::c_void),
        get_func : 
        std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_get_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
      }
    },

    {
      opp{
        name : b"braceexpand\0" as *const u8 as *const i8 as *mut i8,
        letter : b'B' as i32,
        variable : 0 as  *const libc::c_void 
        as *mut libc::c_void
        as *mut i32,
        set_func:
        ::std::mem::transmute::< *mut libc::c_void,
        Option::<setopt_set_func_t>,
        > (0 as *const libc::c_void as *mut libc::c_void),
        get_func :
        ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<setopt_get_func_t>,>
        (0 as *const libc::c_void as *mut libc::c_void)
      }
    },

    {
      opp{
        name : b"emacs\0" as *const u8 as *const i8 as *mut i8,
        letter :  b'\0' as i32, 
        variable : 0 as *const libc::c_void 
        as *mut libc::c_void
        as *mut i32, 
        set_func :  Some(set_edit_mode),
        get_func :  Some(get_edit_mode)
       }
    },
 
    {
      opp{
        name : b"errexit\0" as *const u8 as *const i8 as *mut i8 ,
        letter :  b'e' as i32, 
        variable : 0 as *const libc::c_void 
        as *mut libc::c_void
        as *mut i32,
        set_func : 
          ::std::mem::transmute::< *mut libc::c_void,
          Option::<setopt_set_func_t>,
          > (0 as *const libc::c_void as *mut libc::c_void),
        get_func : 
          ::std::mem::transmute::<
          *mut libc::c_void,
          Option::<setopt_get_func_t>,
          >(0 as *const libc::c_void as *mut libc::c_void)
       }
    },
  
    {
      opp {
        name : b"errtrace\0" as *const u8 as *const i8 as *mut i8,   
        letter :  b'E' as i32,
        variable : 0 as *const libc::c_void 
        as *mut libc::c_void
        as *mut i32,
        set_func : 
          ::std::mem::transmute::< *mut libc::c_void,
          Option::<setopt_set_func_t>,
          > (0 as *const libc::c_void as *mut libc::c_void),
        get_func :
          ::std::mem::transmute::<
          *mut libc::c_void,
          Option::<setopt_get_func_t>,
          >(0 as *const libc::c_void as *mut libc::c_void)
      }
    },
    
    {
      opp {
        name : b"functrace\0" as *const u8 as *const i8 as *mut i8,  
        letter : b'T' as i32, 
        variable : 0 as *const libc::c_void 
        as *mut libc::c_void
        as *mut i32, 
        set_func : 
          ::std::mem::transmute::< *mut libc::c_void,
          Option::<setopt_set_func_t>,
          > (0 as *const libc::c_void as *mut libc::c_void), 
        get_func : 
          ::std::mem::transmute::<
          *mut libc::c_void,
          Option::<setopt_get_func_t>,
          >(0 as *const libc::c_void as *mut libc::c_void)
      }
    },
    
    {
      opp {
        name : b"hashall\0" as *const u8 as *const i8 as *mut i8,   
        letter : b'h' as i32, 
        variable : 0 as *const libc::c_void 
        as *mut libc::c_void
        as *mut i32, 
        set_func : 
          ::std::mem::transmute::< *mut libc::c_void,
          Option::<setopt_set_func_t>,
          > (0 as *const libc::c_void as *mut libc::c_void),
        get_func : 
          ::std::mem::transmute::<
          *mut libc::c_void,
          Option::<setopt_get_func_t>,
          >(0 as *const libc::c_void as *mut libc::c_void)
      }
    },
  
    {
      opp{ 
        name : b"histexpand\0" as *const u8 as *const i8 as *mut i8, 
        letter : b'H' as i32, 
        variable : 0 as *const libc::c_void 
        as *mut libc::c_void
        as *mut i32,
        set_func : 
          ::std::mem::transmute::< *mut libc::c_void,
          Option::<setopt_set_func_t>,
          > (0 as *const libc::c_void as *mut libc::c_void),
        get_func : 
          ::std::mem::transmute::<
          *mut libc::c_void,
          Option::<setopt_get_func_t>,
          >(0 as *const libc::c_void as *mut libc::c_void)
      }
    },
  
    {
      opp{ 
        name : b"history\0" as *const u8 as *const i8 as *mut i8,   
        letter : b'\0' as i32, 
        // variable : 0 as *const libc::c_void 
        // as *mut libc::c_void
        // as *mut i32, 
        variable : &enable_history_list as *const i32 as *mut i32, 
        set_func : Some (bash_set_history),
        get_func : 
          ::std::mem::transmute::<
          *mut libc::c_void,
          Option::<setopt_get_func_t>,
          >(0 as *const libc::c_void as *mut libc::c_void)
      }
    },
  
    {
      opp{
        name : b"ignoreeof\0" as *const u8 as *const i8 as *mut i8, 
        letter : b'\0' as i32,
        /*variable : 0 as *const libc::c_void 
        as *mut libc::c_void
        as *mut i32, */
        variable : &ignoreeof as *const i32 as *mut i32,
        set_func : Some (set_ignoreeof), 
        get_func : 
          ::std::mem::transmute::<
          *mut libc::c_void,
          Option::<setopt_get_func_t>,
           >(0 as *const libc::c_void as *mut libc::c_void)
       }
    },

    {
      opp{  
        name : b"interactive-comments\0" as *const u8 as *const i8 as *mut i8,
        letter : b'\0' as i32,
        /*variable : 0 as *const libc::c_void 
        as *mut libc::c_void
        as *mut i32, */
        variable :  &interactive_comments  as *const i32 as *mut i32, 
        set_func : 
          ::std::mem::transmute::< *mut libc::c_void,
          Option::<setopt_set_func_t>,
          > (0 as *const libc::c_void as *mut libc::c_void),
        get_func : 
          ::std::mem::transmute::<
          *mut libc::c_void,
          Option::<setopt_get_func_t>,
          >(0 as *const libc::c_void as *mut libc::c_void)
      }
    },

    {
      opp{ 
        name : b"keyword\0" as *const u8 as *const i8 as *mut i8,    
        letter : b'k' as i32,
        variable : 0 as *const libc::c_void 
        as *mut libc::c_void
        as *mut i32, 
        set_func : 
          ::std::mem::transmute::< *mut libc::c_void,
          Option::<setopt_set_func_t>,
          > (0 as *const libc::c_void as *mut libc::c_void),
        get_func : 
          ::std::mem::transmute::<
          *mut libc::c_void,
          Option::<setopt_get_func_t>,
          >(0 as *const libc::c_void as *mut libc::c_void)
      }
    },
  
    {
      opp{ 
        name : b"monitor\0" as *const u8 as *const i8 as *mut i8,    
        letter : b'm' as i32, 
        variable : 0 as *const libc::c_void 
        as *mut libc::c_void
        as *mut i32, 
        set_func : 
          ::std::mem::transmute::< *mut libc::c_void,
          Option::<setopt_set_func_t>,
          > (0 as *const libc::c_void as *mut libc::c_void),
        get_func :
          ::std::mem::transmute::<
          *mut libc::c_void,
          Option::<setopt_get_func_t>,
          >(0 as *const libc::c_void as *mut libc::c_void)
      }
    },

    {
      opp{ 
        name : b"noclobber\0" as *const u8 as *const i8 as *mut i8,  
        letter : b'C' as i32, 
        variable : 0 as *const libc::c_void 
        as *mut libc::c_void
        as *mut i32, 
        set_func : 
          ::std::mem::transmute::< *mut libc::c_void,
          Option::<setopt_set_func_t>,
          > (0 as *const libc::c_void as *mut libc::c_void),
        get_func : 
          ::std::mem::transmute::<
          *mut libc::c_void,
          Option::<setopt_get_func_t>,
          >(0 as *const libc::c_void as *mut libc::c_void)
      }
    },
  
    {
      opp{ 
        name : b"noexec\0" as *const u8 as *const i8 as *mut i8,     
        letter : b'n' as i32, 
        variable : 0 as *const libc::c_void 
        as *mut libc::c_void
        as *mut i32, 
        set_func : 
          ::std::mem::transmute::< *mut libc::c_void,
          Option::<setopt_set_func_t>,
          > (0 as *const libc::c_void as *mut libc::c_void),
        get_func : 
          ::std::mem::transmute::<
          *mut libc::c_void,
          Option::<setopt_get_func_t>,
          >(0 as *const libc::c_void as *mut libc::c_void)
      }
    },

    { 
      opp{ 
        name : b"noglob\0" as *const u8 as *const i8 as *mut i8,     
        letter : b'f' as i32, 
        variable : 0 as *const libc::c_void 
          as *mut libc::c_void
          as *mut i32, 
        set_func : 
          ::std::mem::transmute::< *mut libc::c_void,
          Option::<setopt_set_func_t>,
          > (0 as *const libc::c_void as *mut libc::c_void),
        get_func : 
          ::std::mem::transmute::<
          *mut libc::c_void,
          Option::<setopt_get_func_t>,
          >(0 as *const libc::c_void as *mut libc::c_void)
      }
    },

    {
      opp{ 
        name : b"nolog\0" as *const u8 as *const i8 as *mut i8,     
        letter : b'\0' as i32, 
        /*variable : 0 as *const libc::c_void 
        as *mut libc::c_void
        as *mut i32, */
        variable : &dont_save_function_defs as *const i32 as *mut i32, 
        set_func : 
          ::std::mem::transmute::< *mut libc::c_void,
          Option::<setopt_set_func_t>,
          > (0 as *const libc::c_void as *mut libc::c_void),
        get_func : 
          ::std::mem::transmute::<
          *mut libc::c_void,
          Option::<setopt_get_func_t>,
          >(0 as *const libc::c_void as *mut libc::c_void)
      }
    },

    {
      opp{ 
        name : b"notify\0" as *const u8 as *const i8 as *mut i8,     
        letter : b'b' as i32, 
        variable : 0 as *const libc::c_void 
        as *mut libc::c_void
        as *mut i32, 
        set_func :
          ::std::mem::transmute::< *mut libc::c_void,
          Option::<setopt_set_func_t>,
          > (0 as *const libc::c_void as *mut libc::c_void),
        get_func :
          ::std::mem::transmute::<
          *mut libc::c_void,
          Option::<setopt_get_func_t>,
          >(0 as *const libc::c_void as *mut libc::c_void)
      }
    },

    {
      opp{ 
        name : b"nounset\0" as *const u8 as *const i8 as *mut i8,    
        letter : b'u' as i32, 
        variable : 0 as *const libc::c_void 
        as *mut libc::c_void
        as *mut i32, 
        set_func : 
          ::std::mem::transmute::< *mut libc::c_void,
          Option::<setopt_set_func_t>,
          > (0 as *const libc::c_void as *mut libc::c_void),
        get_func :
          ::std::mem::transmute::<
          *mut libc::c_void,
          Option::<setopt_get_func_t>,
          >(0 as *const libc::c_void as *mut libc::c_void)
    }
    },

    {
      opp{ 
        name : b"onecmd\0" as *const u8 as *const i8 as *mut i8,     
        letter : b't' as i32,
        variable : 0 as *const libc::c_void 
        as *mut libc::c_void
        as *mut i32, 
        set_func : 
          ::std::mem::transmute::< *mut libc::c_void,
          Option::<setopt_set_func_t>,
          > (0 as *const libc::c_void as *mut libc::c_void),
        get_func :
          ::std::mem::transmute::<
          *mut libc::c_void,
          Option::<setopt_get_func_t>,
          >(0 as *const libc::c_void as *mut libc::c_void)
      }
    },

    {
      opp{ 
        name : b"physical\0" as *const u8 as *const i8 as *mut i8,   
        letter : b'P' as i32, 
        variable : 0 as *const libc::c_void 
        as *mut libc::c_void
        as *mut i32, 
        set_func : 
          ::std::mem::transmute::< *mut libc::c_void,
          Option::<setopt_set_func_t>,
          > (0 as *const libc::c_void as *mut libc::c_void),
        get_func : 
          ::std::mem::transmute::<
          *mut libc::c_void,
          Option::<setopt_get_func_t>,
          >(0 as *const libc::c_void as *mut libc::c_void)
      }
    },

    {
      opp{ 
        name : b"pipefail\0" as *const u8 as *const i8 as *mut i8,  
        letter : b'\0' as i32, 
        /*variable : 0 as *const libc::c_void 
        as *mut libc::c_void
        as *mut i32, */
        variable : &pipefail_opt as *const i32 as *mut i32, 
        set_func :
          ::std::mem::transmute::< *mut libc::c_void,
          Option::<setopt_set_func_t>,
          > (0 as *const libc::c_void as *mut libc::c_void),
        get_func : 
          ::std::mem::transmute::<
          *mut libc::c_void,
          Option::<setopt_get_func_t>,
    >(0 as *const libc::c_void as *mut libc::c_void)
      }
    },

    {
      opp{ 
        name : b"posix\0" as *const u8 as *const i8 as *mut i8,     
        letter : b'\0' as i32, 
        /*variable : 0 as *const libc::c_void 
        as *mut libc::c_void
        as *mut i32, */
        variable : &posixly_correct as *const libc::c_int as *mut libc::c_int,
        set_func : Some(set_posix_mode), 
        get_func :  
          ::std::mem::transmute::<
          *mut libc::c_void,
          Option::<setopt_get_func_t>,
          >(0 as *const libc::c_void as *mut libc::c_void)
      }
    },

    {
      opp{ 
        name : b"privileged\0" as *const u8 as *const i8 as *mut i8, 
        letter : b'p' as i32, 
        variable : 0 as *const libc::c_void 
        as *mut libc::c_void
        as *mut i32, 
        set_func : 
          ::std::mem::transmute::< *mut libc::c_void,
          Option::<setopt_set_func_t>,
          > (0 as *const libc::c_void as *mut libc::c_void),
        get_func : 
          ::std::mem::transmute::<
          *mut libc::c_void,
          Option::<setopt_get_func_t>,
          >(0 as *const libc::c_void as *mut libc::c_void)
      }
    },
  
    {
      opp{ 
        name : b"verbose\0" as *const u8 as *const i8 as *mut i8,    
        letter : b'v' as i32, 
        variable : 0 as *const libc::c_void 
        as *mut libc::c_void
        as *mut i32, 
        set_func : 
          ::std::mem::transmute::< *mut libc::c_void,
          Option::<setopt_set_func_t>,
          > (0 as *const libc::c_void as *mut libc::c_void),
        get_func : 
          ::std::mem::transmute::<
          *mut libc::c_void,
          Option::<setopt_get_func_t>,
          >(0 as *const libc::c_void as *mut libc::c_void)
      }
    },

    {
      opp{ 
        name : b"vi\0" as *const u8 as *const i8 as *mut i8,        
        letter : b'\0' as i32, 
        variable : 0 as *const libc::c_void 
        as *mut libc::c_void
        as *mut i32, 
        set_func : Some(set_edit_mode), //set_edit_mode as *mut setopt_set_func_t ,// unsafe {&mut set_edit_mode}, 
        get_func : Some(get_edit_mode) 
      }
    },
  
    {
      opp{ 
        name : b"xtrace\0" as *const u8 as *const i8 as *mut i8,     
        letter : b'x' as i32, 
        variable : 0 as *const libc::c_void 
        as *mut libc::c_void
        as *mut i32, 
        set_func : 
          ::std::mem::transmute::< *mut libc::c_void,
          Option::<setopt_set_func_t>,
          > (0 as *const libc::c_void as *mut libc::c_void),
        get_func :
          ::std::mem::transmute::<
          *mut libc::c_void,
          Option::<setopt_get_func_t>,
          >(0 as *const libc::c_void as *mut libc::c_void)
      }
    },
  
    {
      opp{
        name : std::ptr::null_mut() , 
        letter : 0 as i32, 
        variable : 0 as *const libc::c_void 
        as *mut libc::c_void
        as *mut i32, 
        set_func : 
          ::std::mem::transmute::< *mut libc::c_void,
          Option::<setopt_set_func_t>,
          > (0 as *const libc::c_void as *mut libc::c_void),
        get_func : 
          ::std::mem::transmute::<
          *mut libc::c_void,
          Option::<setopt_get_func_t>,
          >(0 as *const libc::c_void as *mut libc::c_void)
      }
    }

]};

extern "C" {
     fn setopt_set_func_t (i :i32 , name : *mut i8) -> i32;
     fn setopt_get_func_t (name : *mut i8)-> i32;
     fn xmalloc(_: u64) -> *mut libc::c_void;
     fn unbind_variable_noref(_: *const i8) -> i32;
     fn unbind_nameref(_: *const i8) -> i32;
     fn unbind_func(_: *const i8) -> i32;
     fn strvec_create(_: i32) -> *mut *mut i8;
     fn all_shell_variables() -> *mut *mut SHELL_VAR;
     fn print_var_list(_: *mut *mut SHELL_VAR);
     fn print_func_list(_: *mut *mut SHELL_VAR);
     fn change_flag(_: i32, _: i32) -> i32;
     fn strlen(_: *const i8) -> u64;
     fn builtin_usage();
     fn find_function (name:* const i8)->* mut SHELL_VAR;
     fn bind_variable(
      _: *const i8,
      _: *mut i8,
      _: i32,
    ) -> *mut SHELL_VAR;

    fn find_variable(_: *const i8) -> *mut SHELL_VAR;
    fn rl_variable_bind (_: *const i8, _: *const i8) -> i32;
    fn find_variable_last_nameref(
    _: *const i8,
    _: i32,
    ) -> *mut SHELL_VAR;

    fn extract_colon_unit(
      _: *mut i8,
      _: *mut i32,
  ) -> *mut i8;

  fn valid_array_reference (
    _ : *const i8 ,
    _ : i32
  )-> i32;

  fn array_variable_part (
    _: *const i8,
    _: i32,
    _:*mut *mut i8,
    _:*mut  i32
    ) -> *mut SHELL_VAR;
   fn all_shell_functions () -> *mut *mut SHELL_VAR;
   fn num_posix_options() -> i32;
   fn find_flag(_: i32) -> *mut i32;
   fn internal_getopt (list:*mut WordList , opts:*mut i8)->i32;
   fn get_posix_options(_: *mut i8) -> *mut i8;
   fn sh_chkwrite (_:i32)->i32;
   fn  reset_internal_getopt();
   fn sh_invalidopt (value:* mut i8);
   fn sv_ignoreeof (_ : *mut  i8);
   fn sv_strict_posix (_: *mut i8);  
   fn with_input_from_stdin();
   fn sh_invalidoptname (value:* mut i8);
   fn bash_history_enable();
   fn load_history();
   fn bash_history_disable();
   fn remember_args (list:* mut WordList, argc:i32);
   fn sh_invalidid (value:* mut i8);
   fn legal_identifier (_:*const i8) -> i32;
   fn unbind_array_element(_: *mut SHELL_VAR, _:*mut i8,_: i32) -> i32;
   fn unbind_variable (_: *const i8) -> i32;
   fn with_input_from_stream (_:libc::FILE , _: *const i8);
   fn stupidly_hack_special_variables (_ : *mut i8);
   fn builtin_error(_: *const i8, _: ...);
   fn  builtin_help();
     static mut posixly_correct : i32;
     static mut enable_history_list : i32;
     static mut ignoreeof : i32 ;
     static mut interactive_comments : i32;
     static mut dont_save_function_defs : i32;
     static mut pipefail_opt : i32;
     static mut mark_modified_vars: i32;
     static mut remember_on_history: i32;
     static mut optflags: [i8; 0];
     static mut list_opttype:i32;
     static mut no_line_editing :i32;
     static mut interactive : i32;
     static mut interactive_shell : i32;
     static mut history_lines_this_session : i32;
     static mut  rl_editing_mode : i32;
     static mut  list_optopt :i32;
     static mut loptend:*mut WordList;
     static assoc_expand_once:i32;
     static mut stdin : libc::FILE;
}

type setopt_set_func_t = unsafe extern "C" fn (
  i :i32 ,
  name : *mut i8
) -> i32;

type setopt_get_func_t =  unsafe extern "C" fn (
  name : *mut i8
) -> i32;

type sh_var_value_func_t =  unsafe extern "C" fn (
  _ : *mut SHELL_VAR 
) -> *mut SHELL_VAR;

type  sh_var_assign_func_t =  unsafe extern "C" fn (
  _ : *mut SHELL_VAR ,
  _ : *mut i8,
  _ : arrayind_t,
  _ : *mut i8
) -> *mut SHELL_VAR;

//type check = String::from_utf8(cc::Build::new().file("../builtins/set.def").expand()).unwrap();
static mut on: *const i8 = b"on\0" as *const u8 as *const i8;
static mut off: *const i8 = b"off\0" as *const u8 as *const i8;
static mut previous_option_value: i32 = 0;
pub type SHELL_VAR = variable;
pub type arrayind_t = i64;

unsafe fn STREQ( a:* const i8, b:* const i8)->bool {  
  //println!("hahhahahhahahah");
  //println!("a  is  {:?}, b is  {:?}",CStr::from_ptr(a),CStr::from_ptr(b));
  return (*a ==*b) && (libc::strcmp(a, b) == 0); 
}

unsafe fn find_minus_o_option (mut name : *mut i8) -> i32 {
  //println! ("enter find_minus_o_option");
  let mut  i : i32 = 0;
  for j in 0..N_O_OPTIONS!()-1 {
    i = j as i32;
    //println! ("i  is  {}, j is  {}",i,j);
    let  ooo = o_options[j];
    //println! ("i  is  {}, j is  {}",i,j);
    if STREQ(name, o_options[j as usize].name) {
      //println! ("qqqqqqqqqqqq");
      return i;
    }
  }
  -1
}

unsafe fn minus_o_option_value (name : *mut i8) -> i32{
    let mut  i : i32 = 0;
    let mut on_or_off : *mut i32 = 0 as *mut i32;

  i = find_minus_o_option (name);
  if i < 0 {
     return  -1;
  }
  let  options  =  o_options [i as  usize ];
  if unsafe {options.letter != 0}{
      if on_or_off == FLAG_UNKNOWN!() {
        return -1;
    }
      return unsafe {*on_or_off};
    }
  else{
     unsafe {GET_BINARY_O_OPTION_VALUE!(i, name)}
  }
}

unsafe fn print_minus_o_option (name : *mut i8, value : i32, pflag : i32){ 
    if pflag == 0 {
      if value > 0 {
        println!("{:?} {:?}", CStr::from_ptr(name), CStr::from_ptr(on));
      }
      else {
        println!("{:?} {:?}", CStr::from_ptr(name), CStr::from_ptr(off));
      } 
    }
    else {
      if value > 0 {
        println!("set -o  {:?}", CStr::from_ptr(name));
      }
      else {
        println!("set +o {:?}", CStr::from_ptr(name));
      }
    }
}

unsafe fn list_minus_o_opts (mode : i32 , reusable :i32){
  // println!("list_minus_o_opts");
  let mut i: i32 = 0;
  let mut on_or_off : *mut i32 = 0 as *mut i32 ;
  let mut value : i32 = 0;

  for j in 0..N_O_OPTIONS!()-1{
    i = j as i32 ;
    //println!("enter loop");
    if o_options[j as usize].letter != 0 {
      value = 0;
          on_or_off = unsafe {
            find_flag (o_options[i as usize].letter)
          };
          if on_or_off == FLAG_UNKNOWN!(){
            on_or_off = &mut value;
          }
          if mode == -1 || mode == unsafe {*on_or_off}  {
            //println!("value  is  {} , i  is  {} , pflag is {}", *on_or_off, i, reusable);
            print_minus_o_option (o_options[i as usize].name, unsafe {
              *on_or_off
            }, reusable);
          }
    }
    else {
      value = unsafe {
        GET_BINARY_O_OPTION_VALUE !(i, o_options[i as usize].name) 
      };
      if mode == -1 || mode == value {
        //println!("value ==== is {}, i   is  {}, pflag  is  {}",value, i, reusable);
        print_minus_o_option (o_options[i as usize].name ,value, reusable);
      }
    }

  }
}

unsafe fn  get_minus_o_opts () -> *mut *mut i8{
 
  let mut ret = 0 as *mut *mut i8;
  let mut i : i32 = 0;
  ret = strvec_create(N_O_OPTIONS!() as i32 + 1);
  for j in 0..N_O_OPTIONS!(){
    i = j as i32;
    if o_options[i as usize].name != std::ptr::null_mut() {

      unsafe {
        *ret.offset(i as isize) = o_options[i as usize].name ;
        //*ret.as_ptr().offset(i as isize) = o_options[i as usize].name ;
      }
    }
  }  
  *ret.offset(i as isize) = o_options[i as usize].name ; 
 // *ret.as_ptr().offset(27 as usize) = std::ptr::null_mut();
  ret
}

unsafe fn get_current_options () -> *mut i8{
  
  let mut temp : *mut i8 = 0 as *mut i8;
  let mut i : i32 =0 ;
  let mut posixopts: i32 = 0;
  posixopts = unsafe {num_posix_options ()};	/* shopts modified by posix mode */
  /* Make the buffer big enough to hold the set -o options and the shopt
     options modified by posix mode. */
     temp = unsafe {xmalloc((1 + N_O_OPTIONS!() as i32 + posixopts) as u64) as *mut i8};
     for t in 0..N_O_OPTIONS!() {
       i = t as i32;
       if o_options[t as usize].letter != 0 {
       unsafe {
          *(temp.offset(t as isize)) =
          *(find_flag (o_options[t as usize].letter)) as i8
         };
       }
       else {
       unsafe {
        *(temp.offset(t as isize)) = GET_BINARY_O_OPTION_VALUE!(t,o_options[i as usize].name) as i8; 
       }
     }
    }
  /* Add the shell options that are modified by posix mode to the end of the
     bitmap. They will be handled in set_current_options() */
  unsafe {
    get_posix_options (temp.offset(i as isize));
    *(temp.offset((i+posixopts) as isize) )= b'\0' as i8;
  }
  return (temp);
}

unsafe fn set_current_options (bitmap : *const i8)  {

  let mut i : i32 ;
  let mut v : i32 ;
  let mut cv : i32;
  let mut on_or_off :*mut i32;

  if bitmap == std::ptr::null_mut(){
    return; 
   }
  
 for t in 0..N_O_OPTIONS!() {
   i = t as i32;
   if bitmap.offset(i as isize) != std::ptr::null_mut(){
      v = FLAG_ON!();
   }
   else {
     v = FLAG_OFF!();
   }
   if o_options[t as usize].letter != 0 {
      on_or_off = unsafe {
        find_flag (o_options[i as usize].letter)
      };
      if on_or_off != std::ptr::null_mut() {
        cv = FLAG_ON!();
      }
      else {
        cv = FLAG_OFF!();
      }
      if v != cv {
          change_flag (o_options[i as usize].letter,v);
      }
      else {
          cv = GET_BINARY_O_OPTION_VALUE! (i,o_options[i as usize].name);
        if cv > 0 {
          cv = FLAG_ON!();
        }
        else {
          cv = FLAG_OFF!();
        }
        if v != cv {
            SET_BINARY_O_OPTION_VALUE!(i,v,o_options[i as usize].name);
       }
      }
    }
   }
}

unsafe extern "C" fn set_ignoreeof (on_or_off : i32 , option_name : *mut i8) -> i32 {
  on_or_off == FLAG_ON!();
  ignoreeof = on_or_off;
  unbind_variable_noref (b"ignoreeof\0" as *const u8 as *const i8);
  if  ignoreeof != 0 {
    bind_variable (b"IGNOREEOF\0" as *const u8 as *const i8, 
                  b"10\0"  as *const u8 as *mut i8, 0); 
  }
  else {
    unbind_variable_noref (b"IGNOREEOF\0"  as *const u8 as *const i8);
  }
  sv_ignoreeof (b"IGNOREEOF\0" as *const u8 as *const i8  as *mut i8);
  return 0;
}

unsafe extern "C" fn set_posix_mode (on_or_off : i32 , option_name : *mut i8) -> i32 {
  if (on_or_off == FLAG_ON!() && posixly_correct != 0 ) ||
  (on_or_off == FLAG_OFF!() && posixly_correct == 0){      
    return 0;
  }
  on_or_off == FLAG_ON!();
  posixly_correct = on_or_off ;
  
  if posixly_correct != 0 {
    unbind_variable_noref(b"POSIXLY_CORRECT\0" as *const u8 as *const i8);
  }
    
  else  {
    bind_variable (b"POSIXLY_CORRECT\0" as *const u8 as *const i8,
                   b"y\0" as *const u8 as *mut i8, 0);
  }
  sv_strict_posix (b"POSIXLY_CORRECT\0" as *const u8 as *mut i8);
  return 0;
}

unsafe extern "C" fn set_edit_mode (on_or_off : i32 , option_name : *mut i8) -> i32{
  //println!("set edit mode by huanhuan");
  let  mut isemacs : i32;

  if on_or_off == FLAG_ON!() {

      rl_variable_bind (b"editing-mode\0" as *const u8 as *const i8,
      option_name);
     if interactive > 0 {
        with_input_from_stdin ()
       ;
     }

      no_line_editing = 0;
    }
    else {
      if rl_editing_mode == 1 {
        isemacs = 1;
      }
      else {
        isemacs = 0;
      }
      if isemacs != 0 &&  *option_name == b'e' as  i8  
      || (isemacs == 0 && *option_name == b'v' as  i8)  {
        if interactive > 0 {
          with_input_from_stream (stdin,  b"stdin\0" as *const u8 as *const i8);
        }

      }
    }
    return 1- unsafe  
    {no_line_editing};
}

unsafe extern "C" fn get_edit_mode (name : *mut i8) -> i32 {

  if *name == b'e' as i8 {
    if no_line_editing== 0 && rl_editing_mode == 1 {
      return  1;
    }
    else {
      return 0
    }
  }
  else {
    if no_line_editing == 0 && rl_editing_mode == 0 {
      return 1;
    }
    else {
      return 0;
    }
  }
} 

unsafe extern "C" fn  bash_set_history (on_or_off : i32 , option_name : *mut i8) -> i32 {
  if on_or_off == FLAG_ON!() {
    
    enable_history_list = 1;
      bash_history_enable ()
    ;
    if  history_lines_this_session == 0 {
   
        load_history();
    }
  }
  else{
      enable_history_list = 0;
      bash_history_disable ();
  }
  return 1 - enable_history_list;
}

unsafe fn set_minus_o_option (on_or_off : i32, option_name : *mut i8) -> i32 {

  //println!("enter set_minus_o_option");
  let mut i : i32 ;

  i = find_minus_o_option (option_name);
  //println!("i  is  {}",i);
  if i < 0{
      sh_invalidoptname (option_name);
      return EX_USAGE;
    }

  if o_options[i as usize].letter == 0{
      previous_option_value = GET_BINARY_O_OPTION_VALUE!(i, o_options[i as usize].name);
      SET_BINARY_O_OPTION_VALUE!(i, on_or_off, option_name);
      return EXECUTION_SUCCESS!();
    }
  else{
      previous_option_value = change_flag (o_options[i as usize].letter,on_or_off) ;
      if previous_option_value == FLAG_ERROR!(){
        sh_invalidoptname (option_name);
        return EXECUTION_FAILURE!();
      }
      else{
        return EXECUTION_SUCCESS!();
      }
    }
  }

unsafe fn print_all_shell_variables (){

  let mut vars = 0 as *mut *mut SHELL_VAR;

  vars = all_shell_variables ();
  if vars != std::ptr::null_mut() {
      print_var_list (vars);
      libc::free (*vars as *mut libc::c_void );
    }
  /* POSIX.2 does not allow function names and definitions to be output when
     `set' is invoked without options (PASC Interp #202). */
  if posixly_correct == 0 {
      vars = all_shell_functions ();
      if vars != std::ptr::null_mut() {
          print_func_list (vars);
          libc::free (*vars as *mut libc::c_void );
      }
    }
}

pub unsafe fn r_set_shellopts () {
  //println!("set shellopts  by huanhuan");
  let mut value : *mut  i8;
  let mut tflag : [i8;N_O_OPTIONS!()] = [0 as i8 ;N_O_OPTIONS!()];
  let mut vsize : i32 = 0;
  let mut i:  i32 = 0;
  let mut vptr : i32 ;
  let mut ip :*mut   i32 ;
  let mut exported : i32;

  let mut v : *mut SHELL_VAR;
  for j in 0..N_O_OPTIONS!() {
    i = j  as i32;
    if o_options[i as usize].name != std::ptr::null_mut(){
      tflag[i as usize] = 0;
      if o_options[i as usize].letter != 0 {
        ip = find_flag (o_options[i as usize].letter);
        if ip != std::ptr::null_mut() && unsafe {*ip} != 0{
            vsize  = vsize + unsafe {strlen (o_options[i as usize].name) as u64 as u32 as i32 } + 1;
            tflag[i as usize] = 1;
          }
      }
      else if unsafe {GET_BINARY_O_OPTION_VALUE!(i,o_options[i as usize].name)} != 0{
        vsize = vsize + unsafe {strlen (o_options[i as usize].name) as i32} + 1;
        tflag[i as usize] = 1;
      }
    }
  }
  value = unsafe {xmalloc((vsize + 1) as u32 as u64) as *mut i8};
  vptr = 0;

  for j in 0..N_O_OPTIONS!(){
    i = j as i32;
    if o_options[i as usize].name != std::ptr::null_mut(){
      if tflag[i as usize] != 0 as i8 {
        unsafe {
          libc::strcpy (value.offset(vptr as isize), o_options[i as usize].name);
          vptr = vptr + strlen (o_options[i as usize].name) as u64 as i64 as i32;
        }
       *value.offset(vptr as isize)  = b':' as i8;
        vptr =  vptr+1;
      } 
    }
  }

  if vptr > 0 {
    vptr = vptr-1;	
  }
  *value.offset(vptr as isize)  = b'\0' as i8;

  v = find_variable (b"SHELLOPTS\0" as *const u8  as *mut i8);

  /* Turn off the read-only attribute so we can bind the new value, and
     note whether or not the variable was exported. */
  if v != std::ptr::null_mut(){
      VUNSETATTR!(v, att_readonly!());
      exported = exported_p!(v);
    }
  else {
    exported = 0;
  } 
  v = bind_variable (b"SHELLOPTS\0" as *const u8  as *mut i8, value, 0);
  /* Turn the read-only attribute back on, and turn off the export attribute
     if it was set implicitly by mark_modified_vars and SHELLOPTS was not
     exported before we bound the new value. */
    
  VSETATTR!(v, att_readonly!());
 
  if mark_modified_vars!= 0 && exported != 0 && exported_p!(v) != 0 {
  
    VUNSETATTR!(v, att_exported!());
  }   
  libc::free (value as *mut libc::c_void );

}

unsafe fn parse_shellopts (value : *mut  i8) {
  let mut vname : *mut i8;
  let mut vptr : i32 = 0; 
  loop {
      vname = extract_colon_unit(value, &mut vptr);
      if vname != std::ptr::null_mut() {
          break;
      }
      set_minus_o_option(FLAG_ON!(), vname);
      libc::free(vname as *mut libc::c_void);
  };
}

unsafe fn initialize_shell_options (no_shellopts : i32) {
  let mut temp: *mut i8;
  let mut var : *mut SHELL_VAR = 0 as *mut SHELL_VAR;
  
  if no_shellopts == 0 {
      var = find_variable (b"SHELLOPTS\0" as *const u8 as *const i8);
      /* set up any shell options we may have inherited. */
      if !var.is_null() && imported_p!(var) != 0  {
        if assoc_p! (var) != 0 || array_p !(var) != 0{
          temp = std::ptr::null_mut();
        }
        else {
          temp = savestring !(value_cell!(var));
        }

	      if temp != std::ptr::null_mut() {
	        parse_shellopts (temp);
	        libc::free (temp as *mut libc::c_void );
	      }
    	}
    }

  /* Set up the $SHELLOPTS variable. */
  r_set_shellopts ();
}

unsafe fn reset_shell_options () {
  pipefail_opt  = 0;
  ignoreeof  = 0 ;
  posixly_correct = 0 ;
  dont_save_function_defs = 0;
  enable_history_list = 1 ;
  remember_on_history = enable_history_list ;
}

#[no_mangle]
 pub extern "C" fn r_set_builtin (mut list: *mut WordList) -> i32 {
  //println!("write  by huanhuan");
  let mut on_or_off : i32 ;
  let mut flag_name : i32 = 0;
  let mut force_assignment : i32 ;
  let mut opts_changed : i32;
  let mut  rv : i32;
  let mut r : i32 ;
  let mut arg : *mut i8 = 0 as *mut i8;
  let mut s: [i8;3] = [0 as i8;3];
  let mut opt : i32;
  let mut flag : bool = false;
  if list.is_null() {
    //println!("list.is_null()");
    unsafe {
      print_all_shell_variables();
      return  sh_chkwrite (EXECUTION_SUCCESS!());
    }
  }
  //println!("Not list.is_null()");
  rv = EXECUTION_SUCCESS!();
  unsafe {
    reset_internal_getopt ();
    opt= internal_getopt (list, optflags.as_mut_ptr());
    //println!("now get  opt is  {}",opt);
  }
  while  opt != -1 { 
    let optu8 :u8= flag_name as u8;
    let optChar:char=char::from(optu8);
    //println!("now get  optChar is  {}",optChar);
    match  optChar {
      'i' => {
          s[0] = unsafe {
            list_opttype as i8
          };
          s[1] = b'i' as  u8  as i8;
          s[2] = b'\0' as u8 as i8;
          unsafe {
            sh_invalidopt (s.as_ptr() as *mut i8);
            builtin_usage();
          }
          return EX_USAGE;}
      '?' => {
        unsafe {
          builtin_usage ();
        }
          if unsafe {list_optopt} == b'?' as i8 as i32 {
            return EXECUTION_SUCCESS!();
          }
          else {
            return EX_USAGE;
          }
        }
      _ => {
        if opt == -99 {
          unsafe { 
          builtin_help();
          }
          return EX_USAGE;
        }
        unsafe {
          builtin_usage ();
        }
          return EX_USAGE;
        }
      }
   // opt = unsafe {internal_getopt(list, optflags.as_ptr() as *mut i8)};
   opt = unsafe {internal_getopt (list, optflags.as_mut_ptr())};
  }
  opts_changed = 0;
  force_assignment = opts_changed ;
  while list != std::ptr::null_mut() {
    //println!(" !list.is_null()");
    if unsafe {(*(*list).word).word != std::ptr::null_mut()} {
      arg = unsafe {(*(*list).word).word};
      //if (arg[0] == '-' && (!arg[1] || (arg[1] == '-' && !arg[2])))
      if unsafe {
        (*arg == b'-' as u8 as i8)
       && ( arg.offset(1 as isize) == std::ptr::null_mut()
       || (*(arg.offset(1 as isize)) == b'-' as u8 as i8 
       && arg.offset(2 as isize) != std::ptr::null_mut()))
      } {
          //println!("*arg == b'-' && arg[1] && arg[1]== b'-'");
          unsafe {
             list = (*list).next;
          /* `set --' unsets the positional parameters. */
          if *arg.offset(1 as isize) == b'-' as u8 as i8 {
            //println!("arg[1]== b'-'");
            force_assignment = 1;
          }
          /* Until told differently, the old shell behaviour of
           `set - [arg ...]' being equivalent to `set +xv [arg ...]'
           stands.  Posix.2 says the behaviour is marked as obsolescent. */    
          else { 
            //println!("else .........");
            change_flag ('x' as i32 , b'+' as i32);
            change_flag ('v' as i32, b'+' as i32);
            opts_changed = 1;
          }
        }
      }
       on_or_off = unsafe {
        *arg as i32
      };
      if on_or_off != 0 && (on_or_off == '-' as i32 || on_or_off == '+' as i32) {
        //println!("on_or_off != 0 && on_or_off == '-' || on_or_off == '+' ");
        unsafe {
          arg = arg.offset(1 as isize);
          //println!("*++arg");
        }
        flag_name = unsafe{*arg as i32};
       // println!("now flag_name is {}",flag_name);
        while flag_name != 0 {
          //println!("flag_name = *++arg");
          let optu8 :u8 = flag_name as u8;
          let optChar:char=char::from(optu8);
          //println!("now get opt is optchar {}",optChar);
          if optChar == '?'{
              unsafe {
                builtin_usage ();
              }
              return (EXECUTION_SUCCESS!());
          }
          else if optChar == 'o' {
           /* -+o option-name */
            //println!("optChar == 'o'");
            let mut option_name : *mut i8 = 0 as *mut i8 ;
            let mut opt : *mut WordList = 0 as *mut WordList;
            unsafe {opt = (*list).next;}
            if opt == std::ptr::null_mut(){
              //println!("opt is  null");
              if on_or_off == '+' as i32{
                unsafe {
                  list_minus_o_opts (-1, 1 );
                }
              }
              else { 
                unsafe {
                list_minus_o_opts (-1, 0 );
                }
              }
          
              rv = unsafe {sh_chkwrite (rv)};
              unsafe {
                arg = arg.offset(1 as isize);
              }
              flag_name = unsafe{*arg as i32};
              
              continue;
            }
    
            unsafe {
               if !(*opt).word.is_null(){
                option_name = (*(*opt).word).word;
              }
            }
           
            if (option_name == std::ptr::null_mut() 
            || unsafe {
              *option_name  == '\u{0}' as i8
               ||*option_name  == '-' as i8 
               || *option_name  == '+' as i8
            }){
              //on_or_off == '+' as i32;
              unsafe {
                
                if on_or_off == '+' as i32{
                  list_minus_o_opts (-1, 1 );
                }
                else {
                  list_minus_o_opts (-1, 0);
                }
              }
              unsafe {
                arg = arg.offset(1 as isize);
              }
              flag_name = unsafe{*arg as i32};
              
              continue;
            }
            unsafe {
              list = (*list).next; /* Skip over option name. */
            }
            opts_changed = 1;
            r = unsafe {
            
              set_minus_o_option (on_or_off, option_name)
            };
            if r != EXECUTION_SUCCESS!() {
              unsafe {
                r_set_shellopts ()
              };
              return (r);
            }
          }    
          else if unsafe{change_flag (flag_name, on_or_off) == FLAG_ERROR!()}{
        //println!("change_flag ....");
         s[0] = on_or_off as i8;
         s[1] = flag_name as i8 ;
         s[2] = '\0' as i32 as i8 ;
         unsafe {
           sh_invalidopt (s.as_ptr() as *mut i8);
           builtin_usage ();
            r_set_shellopts ();
         }
         return EXECUTION_FAILURE!();
         }
          opts_changed = 1;
          unsafe {
            arg = arg.offset(1 as isize);
          }
         
          flag_name = 0 ;
          //flag_name = unsafe{*arg as i32};
          
        } 
      }
      else {
        break ;
      }
      
      unsafe {
        list = (*list).next;
      }
   }
  }
  
  if list != std::ptr::null_mut() || force_assignment != 0 {
    
      unsafe {
        remember_args(list, 1 as i32);
      }
    }
   
  if opts_changed != 0 {
     unsafe {
        r_set_shellopts();
    }
  }
  return rv;
}

#[no_mangle]
pub  extern "C"  fn r_unset_builtin(mut list: *mut WordList) -> i32 {
  let mut unset_function: i32 = 0;
  let mut unset_variable: i32 = 0;
  let mut unset_array: i32 = 0;
  let mut opt: i32 = 0;
  let mut nameref: i32 = 0;
  let mut any_failed: i32 = 0;
  let mut global_unset_func: i32 = 0;
  let mut global_unset_var: i32 = 0;
  let mut vflags: i32 = 0;
  let mut valid_id: i32 = 0;
  let mut name: *mut i8 = 0 as *mut i8;
  let mut tname: *mut i8 = 0 as *mut i8;

  println!("enter  r_unset by huanhuan");
  let mut c_str_fnv   = CString::new("fnv").unwrap();
  unsafe {
    reset_internal_getopt();
    opt= internal_getopt (list, c_str_fnv.as_ptr() as * mut i8);
  
    while  opt != -1 {
      let optu8:u8= opt as u8;
      let optChar:char=char::from(optu8);
      match optChar {
        'f'=>{global_unset_func = 1;}  
        'v'=>{global_unset_var = 0;} 
        'n'=>{nameref = 1;}
        _=>{
          if opt == -99 {
            builtin_help();
            return EX_USAGE;
        }
          builtin_usage ();
          return EX_USAGE;
        }
      }
      opt =internal_getopt (list, c_str_fnv.as_ptr() as * mut i8);
    }
    //println!("unset func={},  unset val=%{}", global_unset_func, global_unset_var);

  list = loptend;

  if global_unset_func != 0 && global_unset_var != 0 {
      builtin_error (b"cannot simultaneously unset a function and a variable \0" as *const u8
      as *const i8);
      return EXECUTION_FAILURE!();
    }
  else if unset_function != 0  && nameref != 0 {
    nameref = 0;
  }

  if assoc_expand_once != 0 {
    vflags =  VA_NOEXPAND!()|VA_ONEWORD!();
  }  
  else {
    vflags = 0;
  }
  while !list.is_null() {
    let mut var : *mut SHELL_VAR;
    let mut tem : i32  = 0;

    let mut t : *mut i8 = 0 as *mut i8;

    name =  (*(*list).word).word; 
    unset_function = global_unset_func;
    unset_variable = global_unset_var;
    unset_array = 0 ;

    if !unset_function == 0 && nameref == 0 && valid_array_reference (name, vflags) != 0 {
      t = libc::strchr (name, '[' as i32);
	    *t.offset(1 as isize) = b'\0' as i32 as i8;
	    unset_array = unset_array + 1;
    }

    valid_id = legal_identifier (name);

    if global_unset_func == 0 && global_unset_var == 0 && valid_id == 0 {
      unset_array = 0;
      unset_variable = unset_array ;
      unset_function = 1;
    }

    if (unset_function == 0 && valid_id == 0)
    {
      sh_invalidid (name);
      any_failed = any_failed + 1;
      list = (*list).next;
    }
  
    if unset_function != 0 {
      var = find_function (name);
    }
    else {
      if nameref != 0 {
        var = find_variable_last_nameref (name, 0) ;
      }
      else {
        var = find_variable (name);
      }
    }

    if var !=  std::ptr::null_mut() && unset_function == 0 && non_unsettable_p!(var) != 0 {
      builtin_error (b"%s: cannot unset \0" as *const u8
      as *const i8, name);
      any_failed = any_failed + 1;
      list = (*list).next;
    }

    if var != std::ptr::null_mut() && unset_function == 0 && nameref == 0 && STREQ (name, name_cell!(var)) {
      name = name_cell!(var);
    }
   
    if var == std::ptr::null_mut() && nameref == 0 &&  unset_variable == 0 && unset_function == 0{
      var = find_function (name);
      if var != std::ptr::null_mut() {
        unset_function = 1;
      }
    }

    if var!= std::ptr::null_mut() && readonly_p! (var)!= 0 {
      if unset_function != 0 {
        builtin_error (b"%s: cannot unset: readonly %s  \0 " as *const u8 as *mut i8,
        (*var).name, b"function\0" as *const u8 as *mut i8);
      }
      else {
        builtin_error (b"%s: cannot unset: readonly %s \0" as *const u8 as *mut i8,
        (*var).name, b"variable\0" as *const u8 as *mut i8);
      }
      any_failed = any_failed + 1;
      list = (*list).next;
    }
   // #if defined (ARRAY_VARS)
    if var != std::ptr::null_mut() && unset_array != 0 {
    /* Let unbind_array_element decide what to do with non-array vars */
      tem = unbind_array_element (var, t, vflags);	/* XXX new third arg */
      if tem == -2 && array_p!(var) == 0 && assoc_p! (var) == 0 {
        builtin_error (b"%s: not an array variable\0" as *const u8
        as *const i8, (*var).name);
        any_failed = any_failed + 1;
        list = (*list).next;
      }
      else if tem < 0 {
        any_failed = any_failed + 1;
      }
    }
   
    else {
      if var ==  std::ptr::null_mut() && nameref == 0 && unset_function == 0 {
        var = find_variable_last_nameref (name, 0);
        if var !=  std::ptr::null_mut() && nameref_p!(var) != 0 { 
          if valid_array_reference (nameref_cell!(var), 0) != 0 {
            tname = savestring!(nameref_cell!(var));
            var = array_variable_part (tname, 0,  &mut t, &mut 0);
            if var != std::ptr::null_mut() {
              tem = unbind_array_element (var, t, vflags);	/* XXX new third arg */
            }
            libc::free (tname as *mut libc::c_void );
          }
        
          else {
            tem = unbind_variable(nameref_cell! (var));
          }
        }
        else {
          tem = unbind_variable (name);
        }
      }
      else {
        if unset_function != 0 {
          tem = unbind_func (name);
        }
        else if nameref != 0 {
          tem = unbind_nameref (name);
        }
        else {
          tem =  unbind_variable (name);
        }
      }
    }
    
    if tem == -1 && nameref == 0 && unset_function == 0 && unset_variable == 0 {
      tem = unbind_func (name);
    }
    name = (*(*list).word).word;

    if unset_function == 0 {
      stupidly_hack_special_variables (name);
    }
    list = (*list).next;
  }

  if any_failed != 0 {
    return EXECUTION_FAILURE!();
  }
  else {
    return EXECUTION_SUCCESS!();
  }
}
}
