use std::ffi::CStr;

use libc::{c_char, c_int, c_void};
use r_bash::*;


extern "C" {
    static mut history_expansion_char:c_char;
    static mut history_quoting_state:c_int;
    static mut history_quotes_inhibit_expansion:c_int;
    static mut history_search_delimiter_chars:*mut c_char;
    static mut history_inhibit_expansion_function:Option<rl_linebuf_func_t>;
    static mut history_lines_read_from_file:c_int;
    static mut history_base: c_int;
    static mut history_length: c_int;
    static mut history_lines_written_to_file: c_int;
    static mut bash_input: BASH_INPUT;
    static mut history_subst_char: c_char;
    static mut rl_dispatching: c_int;
    static mut rl_done: c_int;
    static mut history_max_entries: c_int;

    
    fn mbschr(_: *const c_char, _: c_int) -> *mut c_char;
    fn read_history(_:*const c_char) -> c_int;
    fn using_history();
    fn file_exits(_:*const c_char) -> c_int;
    fn clear_hisroty();
    fn remove_history(_:c_int) -> *mut HIST_ENTRY;
    fn free_history_entry(_: *mut HIST_ENTRY) -> histdata_t;
    fn remove_history_range(_:c_int, _:c_int) -> *mut *mut HIST_ENTRY;
    fn history_list() -> *mut *mut HIST_ENTRY;
    fn history_get(_: c_int) -> *mut HIST_ENTRY;
    fn where_history() -> c_int;
    fn history_set_pos(_: c_int) -> c_int;
    fn append_history(_: c_int, _: *const c_char) -> c_int;
    fn __errno_location() -> *mut c_int;
    fn write_history(_: *const c_char) -> c_int;
    fn history_expand(_: *mut c_char, _: *mut *mut c_char) -> c_int;
    fn previous_history() -> *mut HIST_ENTRY;
    fn replace_history_entry(_: c_int, _: *const c_char, _: histdata_t) -> *mut HIST_ENTRY;
    fn history_is_stifled() -> c_int;
    fn add_history(_: *const c_char);
    fn strmatch( _: *mut c_char, _: *mut c_char, _: c_int) -> c_int;

}

pub type rl_linebuf_func_t = unsafe extern "C" fn(*mut c_char, c_int) -> c_int;





#[macro_export]

macro_rules! HISTSIZE_DEFAULT {

    () => {

        b"500\0" as *const u8 as *mut c_char

    };

}



#[macro_export]

macro_rules! HIGN_EXPAND {

    () => {

        0x01

    };

}



#[macro_export]

macro_rules! ENOENT {

    () => {

        2

    };

}



#[macro_export]

macro_rules! errno {

    () => {

        *__errno_location()

    };

}



#[macro_export]

macro_rules! whitespace {

    ($c:expr) => {

        ($c as c_int == ' ' as i32 || $c as c_int == '\t' as i32)

    };

}





#[macro_export]

macro_rules! STREQ {

    ($a:expr, $b:expr) => {

        *$a.offset(0 as isize) == *$b.offset(0 as isize)

        && strcmp($a, $b) == 0

    };

}



#[macro_export]

macro_rules! savestring {

    ($x:expr) => {

        strcpy(malloc((1 + strlen($x)) as usize) as *mut c_char, $x,)

    };

}



#[macro_export]

macro_rules! FNMATCH_EXTFLAG {

    () => {

        if extended_glob != 0 {

            (1 as c_int) << 5 as c_int

        } else {

            0 as c_int

        }

    };

}





#[macro_export]

macro_rules! FNM_NOMATCH {

    () => {

        1

    };

}


pub const st_stdin: stream_type = 1;


unsafe extern "C" fn member(c:i32, s:*const c_char) -> bool
{
    if c != 0 {
       return  mbschr(s, c) as c_char != 0 as c_char ;
    }
    else {
        return false;
    }
}

static mut histignore: ignorevar = unsafe {
    {
        let mut init = ignorevar {
            varname: b"HISTIGNORE\0" as *const u8 as *mut c_char,
            ignores: 0 as *mut ign,
            num_ignores: 0 as c_int,
            last_ignoreval: 0 as *const c_char as *mut c_char,
            item_func: ::std::mem::transmute::<
                unsafe extern "C" fn(*mut ign) -> c_int,
                sh_iv_item_func_t,
            >( histignore_item_func as unsafe extern "C" fn(*mut ign) -> c_int),
        };
        init
    }
};


pub type histdata_t = *mut c_void;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _hist_entry{
    pub line: *mut c_char,
    pub timestamp: *mut c_char,
    pub data: histdata_t,
}
pub type HIST_ENTRY = _hist_entry;


#[no_mangle]
pub static mut remember_on_history:c_int = 0;
#[no_mangle]
pub static mut  enable_history_list:c_int = -1;
#[no_mangle]
pub static mut history_lines_this_session:c_int = 0;
#[no_mangle]
pub static mut history_expansion_inhibited:c_int = 0;
#[no_mangle]
pub static mut double_quotes_inhibit_history_expansion:c_int = 0;
#[no_mangle]
pub static mut command_oriented_history:c_int = 1;
#[no_mangle]
pub static mut current_command_first_line_saved:c_int = 0;
#[no_mangle]
pub static mut current_command_line_comment:c_int = 0;
#[no_mangle]
pub static mut literal_history:c_int = 0;
#[no_mangle]
pub static mut force_append_history:c_int = 0;
#[no_mangle]
pub static mut history_control:c_int = 0;
#[no_mangle]
pub static mut hist_last_line_added:c_int = 0;
#[no_mangle]
pub static mut hist_last_line_pushed:c_int = 0;
#[no_mangle]
pub static mut history_reediting:c_int = 0;
#[no_mangle]
pub static mut hist_verify:c_int = 0;
#[no_mangle]
pub static mut dont_save_function_defs:c_int = 0;
unsafe extern "C" fn bash_history_inhibit_expansion(mut string: *mut c_char, mut i: c_int) -> c_int 
{
    let mut t: c_int = 0;
    let mut si: c_int = 0;
    let mut hx: [c_char; 2] = [0; 2];
    hx[0 as usize] = history_expansion_char;
    hx[1 as usize] = '\u{0}' as i32 as c_char;

    if i > 0 as c_int
        && *string.offset((i - 1) as isize) as c_int == '[' as i32
        && member(']' as i32, string.offset(i as isize).offset(1 as c_int as isize))
    {
        return 1 
    } 
    else if i > 1 as c_int
            && *string.offset((i - 1 as c_int) as isize) as c_int
                == '{' as i32
            && *string.offset((i - 2 as c_int) as isize) as c_int
                == '$' as i32
            && member('}' as i32, string.offset(i as isize).offset(1 as c_int as isize))
    {
        return 1 
    } 
    else if i > 1 as c_int
                && *string.offset((i - 1 as c_int) as isize) as c_int
                    == '$' as i32
                && *string.offset(i as isize) as c_int == '!' as i32
    {
        return 1 
    } 
    else if extended_glob != 0 && i > 1 as c_int
            && *string.offset((i + 1 as c_int) as isize) as c_int == '(' as i32
            && member(')' as i32, string.offset(i as isize).offset(2 as c_int as isize))
    {
        return 1 ;
    }
            
        
    
    si = 0;
    if history_quoting_state == '\'' as i32 {
        si = skip_to_delim(string,0 ,b"'\0" as *const u8 as *mut c_char,SD_NOJMP as c_int | SD_HISTEXP as c_int);
        if *string.offset(si as isize) == 0  || si >= i {
            return 1 ;
        }
        si += 1;
    }

    t = skip_to_histexp(string, si, hx.as_mut_ptr(), SD_NOJMP as c_int| SD_HISTEXP as c_int);
    if t > 0 {
        while t < i {
            t = skip_to_histexp(string, t + 1 as c_int, hx.as_mut_ptr(), SD_NOJMP as c_int| SD_HISTEXP as c_int);
            if t <= 0 {
                return 0 ;
            }
        }
        return (t > i) as c_int;
    } else {
        return 0 
    };
}