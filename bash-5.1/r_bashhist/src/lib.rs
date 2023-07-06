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
