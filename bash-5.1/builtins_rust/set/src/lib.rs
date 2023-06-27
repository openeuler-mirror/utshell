use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashMap;
use std::ffi::CString;


extern "C" {
    //pub type _IO_wide_data;
   // pub type _IO_codecvt;
   // pub type _IO_marker;
    static mut stdin: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(__ptr: *mut libc::c_void);
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn find_function(_: *const libc::c_char) -> *mut SHELL_VAR;
    fn find_variable(_: *const libc::c_char) -> *mut SHELL_VAR;
    fn find_variable_last_nameref(
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut SHELL_VAR;
    fn bind_variable(
        _: *const libc::c_char,
        _: *mut libc::c_char,
        _: libc::c_int,
    ) -> *mut SHELL_VAR;
    fn all_shell_variables() -> *mut *mut SHELL_VAR;
    fn all_shell_functions() -> *mut *mut SHELL_VAR;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn num_posix_options() -> libc::c_int;
    fn get_posix_options(_: *mut libc::c_char) -> *mut libc::c_char;
    fn set_posix_options(_: *const libc::c_char);
    fn legal_identifier(_: *const libc::c_char) -> libc::c_int;
    fn extract_colon_unit(
        _: *mut libc::c_char,
        _: *mut libc::c_int,
    ) -> *mut libc::c_char;
    fn unbind_variable(_: *const libc::c_char) -> libc::c_int;
    fn unbind_nameref(_: *const libc::c_char) -> libc::c_int;
    fn unbind_variable_noref(_: *const libc::c_char) -> libc::c_int;
    fn unbind_func(_: *const libc::c_char) -> libc::c_int;
    fn print_var_list(_: *mut *mut SHELL_VAR);
    fn print_func_list(_: *mut *mut SHELL_VAR);
    fn stupidly_hack_special_variables(_: *mut libc::c_char);
    fn sv_ignoreeof(_: *mut libc::c_char);
    fn sv_strict_posix(_: *mut libc::c_char);
    static mut assoc_expand_once: libc::c_int;
    fn unbind_array_element(
        _: *mut SHELL_VAR,
        _: *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int;
    fn valid_array_reference(_: *const libc::c_char, _: libc::c_int) -> libc::c_int;
    fn array_variable_part(
        _: *const libc::c_char,
        _: libc::c_int,
        _: *mut *mut libc::c_char,
        _: *mut libc::c_int,
    ) -> *mut SHELL_VAR;
    static mut no_line_editing: libc::c_int;
    static mut posixly_correct: libc::c_int;
    static mut interactive: libc::c_int;
    fn strvec_create(_: libc::c_int) -> *mut *mut libc::c_char;
    fn with_input_from_stdin();
    fn with_input_from_stream(_: *mut FILE, _: *const libc::c_char);
    static mut ignoreeof: libc::c_int;
    static mut optflags: [libc::c_char; 0];
    static mut mark_modified_vars: libc::c_int;
    static mut interactive_comments: libc::c_int;
    static mut pipefail_opt: libc::c_int;
    fn find_flag(_: libc::c_int) -> *mut libc::c_int;
    fn change_flag(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn builtin_error(_: *const libc::c_char, _: ...);
    fn builtin_usage();
    fn sh_invalidopt(_: *mut libc::c_char);
    fn sh_invalidoptname(_: *mut libc::c_char);
    fn sh_invalidid(_: *mut libc::c_char);
    fn sh_chkwrite(_: libc::c_int) -> libc::c_int;
    fn remember_args(_: *mut WORD_LIST, _: libc::c_int);
    fn builtin_help();
    static mut list_optopt: libc::c_int;
    static mut list_opttype: libc::c_int;
    static mut loptend: *mut WORD_LIST;
    fn internal_getopt(_: *mut WORD_LIST, _: *mut libc::c_char) -> libc::c_int;
    fn reset_internal_getopt();
    fn rl_variable_bind(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut rl_editing_mode: libc::c_int;
    static mut remember_on_history: libc::c_int;
    static mut enable_history_list: libc::c_int;
    static mut history_lines_this_session: libc::c_int;
    static mut dont_save_function_defs: libc::c_int;
    fn bash_history_disable();
    fn bash_history_enable();
    fn load_history();
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    //pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    //pub _codecvt: *mut _IO_codecvt,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub type FILE = _IO_FILE;
pub type __intmax_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct word_desc {
    pub word: *mut libc::c_char,
    pub flags: libc::c_int,
}
pub type WORD_DESC = word_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct word_list {
    pub next: *mut word_list,
    pub word: *mut WORD_DESC,
}
pub type WORD_LIST = word_list;
pub type intmax_t = __intmax_t;
pub type arrayind_t = intmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub exportstr: *mut libc::c_char,
    pub dynamic_value: Option::<sh_var_value_func_t>,
    pub assign_func: Option::<sh_var_assign_func_t>,
    pub attributes: libc::c_int,
    pub context: libc::c_int,
}
pub type sh_var_assign_func_t = unsafe extern "C" fn(
    *mut variable,
    *mut libc::c_char,
    arrayind_t,
    *mut libc::c_char,
) -> *mut variable;
pub type sh_var_value_func_t = unsafe extern "C" fn(*mut variable) -> *mut variable;
pub type SHELL_VAR = variable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub name: *mut libc::c_char,
    pub letter: libc::c_int,
    pub variable: *mut libc::c_int,
    pub set_func: Option::<setopt_set_func_t>,
    pub get_func: Option::<setopt_get_func_t>,
}


pub type setopt_get_func_t = unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int;
pub type setopt_set_func_t = unsafe extern "C" fn(
    libc::c_int,
    *mut libc::c_char,
) -> libc::c_int;
static mut on: *const libc::c_char = b"on\0" as *const u8 as *const libc::c_char;
static mut off: *const libc::c_char = b"off\0" as *const u8 as *const libc::c_char;
static mut previous_option_value: libc::c_int = 0;

//lt
/*
#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref MAP: HashMap<CString, C2RustUnnamed> = {
        let mut map = HashMap::new();
    map.insert( CString::new("allexport").expect("faild Cstring new")
                
                
            C2RustUnnamed {
                name: b"allexport\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                letter: 'a' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void
                    as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_set_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                get_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_get_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            }
                
                ); 
    map};
}
//end lt
*/


#[no_mangle]
pub static mut o_options: [C2RustUnnamed; 28] = unsafe {
    [
        {
            let mut init = C2RustUnnamed {
                name: b"allexport\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                letter: 'a' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void
                    as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_set_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                get_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_get_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"braceexpand\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                letter: 'B' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void
                    as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_set_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                get_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_get_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"emacs\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                letter: '\u{0}' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void
                    as *mut libc::c_int,
                set_func: Some(
                    set_edit_mode
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut libc::c_char,
                        ) -> libc::c_int,
                ),
                get_func: Some(
                    get_edit_mode
                        as unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"errexit\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                letter: 'e' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void
                    as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_set_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                get_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_get_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"errtrace\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                letter: 'E' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void
                    as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_set_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                get_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_get_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"functrace\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                letter: 'T' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void
                    as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_set_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                get_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_get_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"hashall\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                letter: 'h' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void
                    as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_set_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                get_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_get_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"histexpand\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                letter: 'H' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void
                    as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_set_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                get_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_get_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"history\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                letter: '\u{0}' as i32,
                variable: &enable_history_list as *const libc::c_int as *mut libc::c_int,
                set_func: Some(
                    bash_set_history
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut libc::c_char,
                        ) -> libc::c_int,
                ),
                get_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_get_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"ignoreeof\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                letter: '\u{0}' as i32,
                variable: &ignoreeof as *const libc::c_int as *mut libc::c_int,
                set_func: Some(
                    set_ignoreeof
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut libc::c_char,
                        ) -> libc::c_int,
                ),
                get_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_get_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"interactive-comments\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                letter: '\u{0}' as i32,
                variable: &interactive_comments as *const libc::c_int
                    as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_set_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                get_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_get_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"keyword\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                letter: 'k' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void
                    as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_set_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                get_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_get_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"monitor\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                letter: 'm' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void
                    as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_set_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                get_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_get_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"noclobber\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                letter: 'C' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void
                    as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_set_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                get_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_get_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"noexec\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                letter: 'n' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void
                    as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_set_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                get_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_get_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"noglob\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                letter: 'f' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void
                    as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_set_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                get_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_get_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"nolog\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                letter: '\u{0}' as i32,
                variable: &dont_save_function_defs as *const libc::c_int
                    as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_set_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                get_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_get_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"notify\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                letter: 'b' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void
                    as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_set_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                get_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_get_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"nounset\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                letter: 'u' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void
                    as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_set_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                get_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_get_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"onecmd\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                letter: 't' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void
                    as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_set_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                get_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_get_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"physical\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                letter: 'P' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void
                    as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_set_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                get_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_get_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"pipefail\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                letter: '\u{0}' as i32,
                variable: &pipefail_opt as *const libc::c_int as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_set_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                get_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_get_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"posix\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                letter: '\u{0}' as i32,
                variable: &posixly_correct as *const libc::c_int as *mut libc::c_int,
                set_func: Some(
                    set_posix_mode
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut libc::c_char,
                        ) -> libc::c_int,
                ),
                get_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_get_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"privileged\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                letter: 'p' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void
                    as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_set_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                get_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_get_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"verbose\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                letter: 'v' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void
                    as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_set_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                get_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_get_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"vi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                letter: '\u{0}' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void
                    as *mut libc::c_int,
                set_func: Some(
                    set_edit_mode
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut libc::c_char,
                        ) -> libc::c_int,
                ),
                get_func: Some(
                    get_edit_mode
                        as unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"xtrace\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                letter: 'x' as i32,
                variable: 0 as *const libc::c_void as *mut libc::c_void
                    as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_set_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                get_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_get_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
                letter: 0 as libc::c_int,
                variable: 0 as *const libc::c_void as *mut libc::c_void
                    as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_set_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                get_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<setopt_get_func_t>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
    ]
};
unsafe extern "C" fn find_minus_o_option(mut name: *mut libc::c_char) -> libc::c_int {

	
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !(o_options[i as usize].name).is_null() {
        if *name.offset(0 as libc::c_int as isize) as libc::c_int
            == *(o_options[i as usize].name).offset(0 as libc::c_int as isize)
                as libc::c_int
            && strcmp(name, o_options[i as usize].name) == 0 as libc::c_int
        {
            return i;
        }
        i += 1;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn minus_o_option_value(
    mut name: *mut libc::c_char,
) -> libc::c_int {

    let mut i: libc::c_int = 0;
    let mut on_or_off: *mut libc::c_int = 0 as *mut libc::c_int;
    i = find_minus_o_option(name);
    if i < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if o_options[i as usize].letter != 0 {
        on_or_off = find_flag(o_options[i as usize].letter);
        return if on_or_off.is_null() { -(1 as libc::c_int) } else { *on_or_off };
    } else {
        return if (o_options[i as usize].get_func).is_some() {
            (Some(
                ((*o_options.as_ptr().offset(i as isize)).get_func)
                    .expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")(name)
        } else {
            *o_options[i as usize].variable
        }
    };
}
unsafe extern "C" fn print_minus_o_option(
    mut name: *mut libc::c_char,
    mut value: libc::c_int,
    mut pflag: libc::c_int,
) {
    if pflag == 0 as libc::c_int {
        printf(
            b"%-15s\t%s\n\0" as *const u8 as *const libc::c_char,
            name,
            if value != 0 { on } else { off },
        );
    } else {
        printf(
            b"set %co %s\n\0" as *const u8 as *const libc::c_char,
            if value != 0 { '-' as i32 } else { '+' as i32 },
            name,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn list_minus_o_opts(
    mut mode: libc::c_int,
    mut reusable: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut on_or_off: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut value: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !(o_options[i as usize].name).is_null() {
        if o_options[i as usize].letter != 0 {
            value = 0 as libc::c_int;
            on_or_off = find_flag(o_options[i as usize].letter);
            if on_or_off.is_null() {
                on_or_off = &mut value;
            }
            if mode == -(1 as libc::c_int) || mode == *on_or_off {
                print_minus_o_option(o_options[i as usize].name, *on_or_off, reusable);
            }
        } else {
            value = if (o_options[i as usize].get_func).is_some() {
                (Some(
                    ((*o_options.as_ptr().offset(i as isize)).get_func)
                        .expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(o_options[i as usize].name)
            } else {
                *o_options[i as usize].variable
            };
            if mode == -(1 as libc::c_int) || mode == value {
                print_minus_o_option(o_options[i as usize].name, value, reusable);
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_minus_o_opts() -> *mut *mut libc::c_char {
    let mut ret: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut i: libc::c_int = 0;
    ret = strvec_create(
        (::std::mem::size_of::<[C2RustUnnamed; 28]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed>() as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    );
    i = 0 as libc::c_int;
    while !(o_options[i as usize].name).is_null() {
        let ref mut fresh0 = *ret.offset(i as isize);
        *fresh0 = o_options[i as usize].name;
        i += 1;
    }
    let ref mut fresh1 = *ret.offset(i as isize);
    *fresh1 = 0 as *mut libc::c_void as *mut libc::c_char;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn get_current_options() -> *mut libc::c_char {
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut posixopts: libc::c_int = 0;
    posixopts = num_posix_options();
    temp = xmalloc(
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<[C2RustUnnamed; 28]>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<C2RustUnnamed>() as libc::c_ulong,
                    ),
            )
            .wrapping_add(posixopts as libc::c_ulong),
    ) as *mut libc::c_char;
    i = 0 as libc::c_int;
    while !(o_options[i as usize].name).is_null() {
        if o_options[i as usize].letter != 0 {
            *temp
                .offset(
                    i as isize,
                ) = *find_flag(o_options[i as usize].letter) as libc::c_char;
        } else {
            *temp
                .offset(
                    i as isize,
                ) = (if (o_options[i as usize].get_func).is_some() {
                (Some(
                    ((*o_options.as_ptr().offset(i as isize)).get_func)
                        .expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(o_options[i as usize].name)
            } else {
                *o_options[i as usize].variable
            }) as libc::c_char;
        }
        i += 1;
    }
    get_posix_options(temp.offset(i as isize));
    *temp.offset((i + posixopts) as isize) = '\u{0}' as i32 as libc::c_char;
    return temp;
}
#[no_mangle]
pub unsafe extern "C" fn set_current_options(mut bitmap: *const libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut cv: libc::c_int = 0;
    let mut on_or_off: *mut libc::c_int = 0 as *mut libc::c_int;
    if bitmap.is_null() {
        return;
    }
    i = 0 as libc::c_int;
    while !(o_options[i as usize].name).is_null() {
        v = if *bitmap.offset(i as isize) as libc::c_int != 0 {
            '-' as i32
        } else {
            '+' as i32
        };
        if o_options[i as usize].letter != 0 {
            on_or_off = find_flag(o_options[i as usize].letter);
            cv = if *on_or_off != 0 { '-' as i32 } else { '+' as i32 };
            if v != cv {
                change_flag(o_options[i as usize].letter, v);
            }
        } else {
            cv = if (o_options[i as usize].get_func).is_some() {
                (Some(
                    ((*o_options.as_ptr().offset(i as isize)).get_func)
                        .expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(o_options[i as usize].name)
            } else {
                *o_options[i as usize].variable
            };
            cv = if cv != 0 { '-' as i32 } else { '+' as i32 };
            if v != cv {
                if (o_options[i as usize].set_func).is_some() {
                    (Some(
                        ((*o_options.as_ptr().offset(i as isize)).set_func)
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(v, o_options[i as usize].name);
                } else {
                    *o_options[i as usize].variable = (v == '-' as i32) as libc::c_int;
                };
            }
        }
        i += 1;
    }
    set_posix_options(bitmap.offset(i as isize));
}
unsafe extern "C" fn set_ignoreeof(
    mut on_or_off: libc::c_int,
    mut option_name: *mut libc::c_char,
) -> libc::c_int {
    ignoreeof = (on_or_off == '-' as i32) as libc::c_int;
    unbind_variable_noref(b"ignoreeof\0" as *const u8 as *const libc::c_char);
    if ignoreeof != 0 {
        bind_variable(
            b"IGNOREEOF\0" as *const u8 as *const libc::c_char,
            b"10\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int,
        );
    } else {
        unbind_variable_noref(b"IGNOREEOF\0" as *const u8 as *const libc::c_char);
    }
    sv_ignoreeof(
        b"IGNOREEOF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn set_posix_mode(
    mut on_or_off: libc::c_int,
    mut option_name: *mut libc::c_char,
) -> libc::c_int {
    if on_or_off == '-' as i32 && posixly_correct != 0
        || on_or_off == '+' as i32 && posixly_correct == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    posixly_correct = (on_or_off == '-' as i32) as libc::c_int;
    if posixly_correct == 0 as libc::c_int {
        unbind_variable_noref(b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char);
    } else {
        bind_variable(
            b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char,
            b"y\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int,
        );
    }
    sv_strict_posix(
        b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn set_edit_mode(
    mut on_or_off: libc::c_int,
    mut option_name: *mut libc::c_char,
) -> libc::c_int {
    let mut isemacs: libc::c_int = 0;
    if on_or_off == '-' as i32 {
        rl_variable_bind(
            b"editing-mode\0" as *const u8 as *const libc::c_char,
            option_name,
        );
        if interactive != 0 {
            with_input_from_stdin();
        }
        no_line_editing = 0 as libc::c_int;
    } else {
        isemacs = (rl_editing_mode == 1 as libc::c_int) as libc::c_int;
        if isemacs != 0 && *option_name as libc::c_int == 'e' as i32
            || isemacs == 0 && *option_name as libc::c_int == 'v' as i32
        {
            if interactive != 0 {
                with_input_from_stream(
                    stdin,
                    b"stdin\0" as *const u8 as *const libc::c_char,
                );
            }
            no_line_editing = 1 as libc::c_int;
        }
    }
    return 1 as libc::c_int - no_line_editing;
}
unsafe extern "C" fn get_edit_mode(mut name: *mut libc::c_char) -> libc::c_int {
    return if *name as libc::c_int == 'e' as i32 {
        (no_line_editing == 0 as libc::c_int && rl_editing_mode == 1 as libc::c_int)
            as libc::c_int
    } else {
        (no_line_editing == 0 as libc::c_int && rl_editing_mode == 0 as libc::c_int)
            as libc::c_int
    };
}
unsafe extern "C" fn bash_set_history(
    mut on_or_off: libc::c_int,
    mut option_name: *mut libc::c_char,
) -> libc::c_int {
    if on_or_off == '-' as i32 {
        enable_history_list = 1 as libc::c_int;
        bash_history_enable();
        if history_lines_this_session == 0 as libc::c_int {
            load_history();
        }
    } else {
        enable_history_list = 0 as libc::c_int;
        bash_history_disable();
    }
    return 1 as libc::c_int - enable_history_list;
}
#[no_mangle]
pub unsafe extern "C" fn set_minus_o_option(
    mut on_or_off: libc::c_int,
    mut option_name: *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = find_minus_o_option(option_name);
    if i < 0 as libc::c_int {
        sh_invalidoptname(option_name);
        return 258 as libc::c_int;
    }
    if o_options[i as usize].letter == 0 as libc::c_int {
        previous_option_value = if (o_options[i as usize].get_func).is_some() {
            (Some(
                ((*o_options.as_ptr().offset(i as isize)).get_func)
                    .expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")(o_options[i as usize].name)
        } else {
            *o_options[i as usize].variable
        };
        if (o_options[i as usize].set_func).is_some() {
            (Some(
                ((*o_options.as_ptr().offset(i as isize)).set_func)
                    .expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")(on_or_off, option_name);
        } else {
            *o_options[i as usize].variable = (on_or_off == '-' as i32) as libc::c_int;
        };
        return 0 as libc::c_int;
    } else {
        previous_option_value = change_flag(o_options[i as usize].letter, on_or_off);
        if previous_option_value == -(1 as libc::c_int) {
            sh_invalidoptname(option_name);
            return 1 as libc::c_int;
        } else {
            return 0 as libc::c_int
        }
    };
}
unsafe extern "C" fn print_all_shell_variables() {
    let mut vars: *mut *mut SHELL_VAR = 0 as *mut *mut SHELL_VAR;
    vars = all_shell_variables();
    if !vars.is_null() {
        print_var_list(vars);
        free(vars as *mut libc::c_void);
    }
    if posixly_correct == 0 as libc::c_int {
        vars = all_shell_functions();
        if !vars.is_null() {
            print_func_list(vars);
            free(vars as *mut libc::c_void);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn set_shellopts() {
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tflag: [libc::c_char; 28] = [0; 28];
    let mut vsize: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut vptr: libc::c_int = 0;
    let mut ip: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut exported: libc::c_int = 0;
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    i = 0 as libc::c_int;
    vsize = i;
    while !(o_options[i as usize].name).is_null() {
        tflag[i as usize] = 0 as libc::c_int as libc::c_char;
        if o_options[i as usize].letter != 0 {
            ip = find_flag(o_options[i as usize].letter);
            if !ip.is_null() && *ip != 0 {
                vsize = (vsize as libc::c_ulong)
                    .wrapping_add(
                        (strlen(o_options[i as usize].name))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as libc::c_int as libc::c_int;
                tflag[i as usize] = 1 as libc::c_int as libc::c_char;
            }
        } else if if (o_options[i as usize].get_func).is_some() {
                (Some(
                    ((*o_options.as_ptr().offset(i as isize)).get_func)
                        .expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(o_options[i as usize].name)
            } else {
                *o_options[i as usize].variable
            } != 0
            {
            vsize = (vsize as libc::c_ulong)
                .wrapping_add(
                    (strlen(o_options[i as usize].name))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as libc::c_int as libc::c_int;
            tflag[i as usize] = 1 as libc::c_int as libc::c_char;
        }
        i += 1;
    }
    value = xmalloc((vsize + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    vptr = 0 as libc::c_int;
    i = vptr;
    while !(o_options[i as usize].name).is_null() {
        if tflag[i as usize] != 0 {
            strcpy(value.offset(vptr as isize), o_options[i as usize].name);
            vptr = (vptr as libc::c_ulong)
                .wrapping_add(strlen(o_options[i as usize].name)) as libc::c_int
                as libc::c_int;
            let fresh2 = vptr;
            vptr = vptr + 1;
            *value.offset(fresh2 as isize) = ':' as i32 as libc::c_char;
        }
        i += 1;
    }
    if vptr != 0 {
        vptr -= 1;
    }
    *value.offset(vptr as isize) = '\u{0}' as i32 as libc::c_char;
    v = find_variable(b"SHELLOPTS\0" as *const u8 as *const libc::c_char);
    if !v.is_null() {
        (*v).attributes &= !(0x2 as libc::c_int);
        exported = (*v).attributes & 0x1 as libc::c_int;
    } else {
        exported = 0 as libc::c_int;
    }
    v = bind_variable(
        b"SHELLOPTS\0" as *const u8 as *const libc::c_char,
        value,
        0 as libc::c_int,
    );
    (*v).attributes |= 0x2 as libc::c_int;
    if mark_modified_vars != 0 && exported == 0 as libc::c_int
        && (*v).attributes & 0x1 as libc::c_int != 0
    {
        (*v).attributes &= !(0x1 as libc::c_int);
    }
    free(value as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn parse_shellopts(mut value: *mut libc::c_char) {
    let mut vname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vptr: libc::c_int = 0;
    vptr = 0 as libc::c_int;
    loop {
        vname = extract_colon_unit(value, &mut vptr);
        if vname.is_null() {
            break;
        }
        set_minus_o_option('-' as i32, vname);
        free(vname as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn initialize_shell_options(mut no_shellopts: libc::c_int) {
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    if no_shellopts == 0 as libc::c_int {
        var = find_variable(b"SHELLOPTS\0" as *const u8 as *const libc::c_char);
        if !var.is_null() && (*var).attributes & 0x8000 as libc::c_int != 0 {
            temp = if (*var).attributes & 0x4 as libc::c_int != 0
                || (*var).attributes & 0x40 as libc::c_int != 0
            {
                0 as *mut libc::c_void as *mut libc::c_char
            } else {
                strcpy(
                    xmalloc(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_add(strlen((*var).value)),
                    ) as *mut libc::c_char,
                    (*var).value,
                )
            };
            if !temp.is_null() {
                parse_shellopts(temp);
                free(temp as *mut libc::c_void);
            }
        }
    }
    set_shellopts();
}
#[no_mangle]
pub unsafe extern "C" fn reset_shell_options() {
    pipefail_opt = 0 as libc::c_int;
    ignoreeof = 0 as libc::c_int;
    posixly_correct = 0 as libc::c_int;
    dont_save_function_defs = 0 as libc::c_int;
    enable_history_list = 1 as libc::c_int;
    remember_on_history = enable_history_list;
}
#[no_mangle]
pub unsafe extern "C" fn set_builtin(mut list: *mut WORD_LIST) -> libc::c_int {
    let mut on_or_off: libc::c_int = 0;
    let mut flag_name: libc::c_int = 0;
    let mut force_assignment: libc::c_int = 0;
    let mut opts_changed: libc::c_int = 0;
    let mut rv: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: [libc::c_char; 3] = [0; 3];
    if list.is_null() {
        print_all_shell_variables();
        return sh_chkwrite(0 as libc::c_int);
    }
    rv = 0 as libc::c_int;
    reset_internal_getopt();
    loop {
        flag_name = internal_getopt(list, optflags.as_mut_ptr());
        if !(flag_name != -(1 as libc::c_int)) {
            break;
        }
        match flag_name {
            105 => {
                s[0 as libc::c_int as usize] = list_opttype as libc::c_char;
                s[1 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
                s[2 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
                sh_invalidopt(s.as_mut_ptr());
                builtin_usage();
                return 258 as libc::c_int;
            }
            -99 => {
                builtin_help();
                return 258 as libc::c_int;
            }
            63 => {
                builtin_usage();
                return if list_optopt == '?' as i32 {
                    0 as libc::c_int
                } else {
                    258 as libc::c_int
                };
            }
            _ => {}
        }
    }
    opts_changed = 0 as libc::c_int;
    force_assignment = opts_changed;
    while !list.is_null() {
        arg = (*(*list).word).word;
        if *arg.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
            && (*arg.offset(1 as libc::c_int as isize) == 0
                || *arg.offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32
                    && *arg.offset(2 as libc::c_int as isize) == 0)
        {
            list = (*list).next;
            if *arg.offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32 {
                force_assignment = 1 as libc::c_int;
            } else {
                change_flag('x' as i32, '+' as i32);
                change_flag('v' as i32, '+' as i32);
                opts_changed = 1 as libc::c_int;
            }
            break;
        } else {
            on_or_off = *arg as libc::c_int;
            if !(on_or_off != 0 && (on_or_off == '-' as i32 || on_or_off == '+' as i32))
            {
                break;
            }
            loop {
                arg = arg.offset(1);
                flag_name = *arg as libc::c_int;
                if !(flag_name != 0) {
                    break;
                }
                if flag_name == '?' as i32 {
                    builtin_usage();
                    return 0 as libc::c_int;
                } else {
                    if flag_name == 'o' as i32 {
                        let mut option_name: *mut libc::c_char = 0 as *mut libc::c_char;
                        let mut opt: *mut WORD_LIST = 0 as *mut WORD_LIST;
                        opt = (*list).next;
                        if opt.is_null() {
                            list_minus_o_opts(
                                -(1 as libc::c_int),
                                (on_or_off == '+' as i32) as libc::c_int,
                            );
                            rv = sh_chkwrite(rv);
                            continue;
                        } else {
                            option_name = (*(*opt).word).word;
                            if option_name.is_null()
                                || *option_name as libc::c_int == '\u{0}' as i32
                                || *option_name as libc::c_int == '-' as i32
                                || *option_name as libc::c_int == '+' as i32
                            {
                                list_minus_o_opts(
                                    -(1 as libc::c_int),
                                    (on_or_off == '+' as i32) as libc::c_int,
                                );
                                continue;
                            } else {
                                list = (*list).next;
                                opts_changed = 1 as libc::c_int;
                                r = set_minus_o_option(on_or_off, option_name);
                                if r != 0 as libc::c_int {
                                    set_shellopts();
                                    return r;
                                }
                            }
                        }
                    } else if change_flag(flag_name, on_or_off) == -(1 as libc::c_int) {
                        s[0 as libc::c_int as usize] = on_or_off as libc::c_char;
                        s[1 as libc::c_int as usize] = flag_name as libc::c_char;
                        s[2 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
                        sh_invalidopt(s.as_mut_ptr());
                        builtin_usage();
                        set_shellopts();
                        return 1 as libc::c_int;
                    }
                    opts_changed = 1 as libc::c_int;
                }
            }
            list = (*list).next;
        }
    }
    if !list.is_null() || force_assignment != 0 {
        remember_args(list, 1 as libc::c_int);
    }
    if opts_changed != 0 {
        set_shellopts();
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn unset_builtin(mut list: *mut WORD_LIST) -> libc::c_int {
    let mut unset_function: libc::c_int = 0;
    let mut unset_variable: libc::c_int = 0;
    let mut unset_array: libc::c_int = 0;
    let mut opt: libc::c_int = 0;
    let mut nameref: libc::c_int = 0;
    let mut any_failed: libc::c_int = 0;
    let mut global_unset_func: libc::c_int = 0;
    let mut global_unset_var: libc::c_int = 0;
    let mut vflags: libc::c_int = 0;
    let mut valid_id: libc::c_int = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tname: *mut libc::c_char = 0 as *mut libc::c_char;
    any_failed = 0 as libc::c_int;
    nameref = any_failed;
    unset_array = nameref;
    unset_variable = unset_array;
    unset_function = unset_variable;
    global_unset_var = 0 as libc::c_int;
    global_unset_func = global_unset_var;
    reset_internal_getopt();
    loop {
        opt = internal_getopt(
            list,
            b"fnv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !(opt != -(1 as libc::c_int)) {
            break;
        }
        match opt {
            102 => {
                global_unset_func = 1 as libc::c_int;
            }
            118 => {
                global_unset_var = 1 as libc::c_int;
            }
            110 => {
                nameref = 1 as libc::c_int;
            }
            -99 => {
                builtin_help();
                return 258 as libc::c_int;
            }
            _ => {
                builtin_usage();
                return 258 as libc::c_int;
            }
        }
    }
    list = loptend;
    if global_unset_func != 0 && global_unset_var != 0 {
        builtin_error(
            dcgettext(
                0 as *const libc::c_char,
                b"cannot simultaneously unset a function and a variable\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 1 as libc::c_int;
    } else {
        if unset_function != 0 && nameref != 0 {
            nameref = 0 as libc::c_int;
        }
    }
    vflags = if assoc_expand_once != 0 {
        0x1 as libc::c_int | 0x2 as libc::c_int
    } else {
        0 as libc::c_int
    };
    while !list.is_null() {
        let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
        let mut tem: libc::c_int = 0;
        let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
        name = (*(*list).word).word;
        unset_function = global_unset_func;
        unset_variable = global_unset_var;
        unset_array = 0 as libc::c_int;
        if unset_function == 0 && nameref == 0 as libc::c_int
            && valid_array_reference(name, vflags) != 0
        {
            t = strchr(name, '[' as i32);
            let fresh3 = t;
            t = t.offset(1);
            *fresh3 = '\u{0}' as i32 as libc::c_char;
            unset_array += 1;
        }
        valid_id = legal_identifier(name);
        if global_unset_func == 0 as libc::c_int && global_unset_var == 0 as libc::c_int
            && valid_id == 0 as libc::c_int
        {
            unset_array = 0 as libc::c_int;
            unset_variable = unset_array;
            unset_function = 1 as libc::c_int;
        }
        if unset_function == 0 as libc::c_int && valid_id == 0 as libc::c_int {
            sh_invalidid(name);
            any_failed += 1;
            list = (*list).next;
        } else {
            var = if unset_function != 0 {
                find_function(name)
            } else if nameref != 0 {
                find_variable_last_nameref(name, 0 as libc::c_int)
            } else {
                find_variable(name)
            };
            if !var.is_null() && unset_function == 0 as libc::c_int
                && (*var).attributes & 0x2000 as libc::c_int != 0
            {
                builtin_error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: cannot unset\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    name,
                );
                any_failed += 1;
                list = (*list).next;
            } else {
                if !var.is_null() && unset_function == 0 as libc::c_int
                    && nameref == 0 as libc::c_int
                    && (*name.offset(0 as libc::c_int as isize) as libc::c_int
                        == *((*var).name).offset(0 as libc::c_int as isize)
                            as libc::c_int
                        && strcmp(name, (*var).name) == 0 as libc::c_int) as libc::c_int
                        == 0 as libc::c_int
                {
                    name = (*var).name;
                }
                if var.is_null() && nameref == 0 as libc::c_int
                    && unset_variable == 0 as libc::c_int
                    && unset_function == 0 as libc::c_int
                {
                    var = find_function(name);
                    if !var.is_null() {
                        unset_function = 1 as libc::c_int;
                    }
                }
                if !var.is_null() && (*var).attributes & 0x2 as libc::c_int != 0 {
                    builtin_error(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: cannot unset: readonly %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*var).name,
                        if unset_function != 0 {
                            b"function\0" as *const u8 as *const libc::c_char
                        } else {
                            b"variable\0" as *const u8 as *const libc::c_char
                        },
                    );
                    any_failed += 1;
                    list = (*list).next;
                } else {
                    if !var.is_null() && unset_array != 0 {
                        tem = unbind_array_element(var, t, vflags);
                        if tem == -(2 as libc::c_int)
                            && (*var).attributes & 0x4 as libc::c_int == 0 as libc::c_int
                            && (*var).attributes & 0x40 as libc::c_int
                                == 0 as libc::c_int
                        {
                            builtin_error(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"%s: not an array variable\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                (*var).name,
                            );
                            any_failed += 1;
                            list = (*list).next;
                            continue;
                        } else if tem < 0 as libc::c_int {
                            any_failed += 1;
                        }
                    } else if var.is_null() && nameref == 0 as libc::c_int
                            && unset_function == 0 as libc::c_int
                        {
                        var = find_variable_last_nameref(name, 0 as libc::c_int);
                        if !var.is_null()
                            && (*var).attributes & 0x800 as libc::c_int != 0
                        {
                            if valid_array_reference((*var).value, 0 as libc::c_int) != 0
                            {
                                tname = strcpy(
                                    xmalloc(
                                        (1 as libc::c_int as libc::c_ulong)
                                            .wrapping_add(strlen((*var).value)),
                                    ) as *mut libc::c_char,
                                    (*var).value,
                                );
                                var = array_variable_part(
                                    tname,
                                    0 as libc::c_int,
                                    &mut t,
                                    0 as *mut libc::c_int,
                                );
                                if !var.is_null() {
                                    tem = unbind_array_element(var, t, vflags);
                                }
                                free(tname as *mut libc::c_void);
                            } else {
                                tem = unbind_variable((*var).value);
                            }
                        } else {
                            tem = unbind_variable(name);
                        }
                    } else {
                        tem = if unset_function != 0 {
                            unbind_func(name)
                        } else if nameref != 0 {
                            unbind_nameref(name)
                        } else {
                            unbind_variable(name)
                        };
                    }
                    if tem == -(1 as libc::c_int) && nameref == 0 as libc::c_int
                        && unset_function == 0 as libc::c_int
                        && unset_variable == 0 as libc::c_int
                    {
                        tem = unbind_func(name);
                    }
                    name = (*(*list).word).word;
                    if unset_function == 0 as libc::c_int {
                        stupidly_hack_special_variables(name);
                    }
                    list = (*list).next;
                }
            }
        }
    }
    return if any_failed != 0 { 1 as libc::c_int } else { 0 as libc::c_int };
}
