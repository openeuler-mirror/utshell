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

//pub type Function = unsafe extern "C" fn() -> c_int;
pub type Function = ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>;

extern "C" {
    static indirection_level:c_int;
    static posixly_correct:c_int;
    static mut the_printed_command:*mut c_char;
    static mut the_printed_command_size:c_int;
    static mut command_string_index:c_int ;

    fn command_error(func:*const c_char, code:c_int, e:c_int, flags:c_int);
    fn sh_validfd(fd:c_int)->c_int;

    //fn internal_error(format:*mut c_char, arg1:*mut c_char, arg2:*mut c_char)->c_int;
    fn internal_error(format:*const c_char, _:...);
    fn internal_warning(_:*const c_char, _:...);
    fn get_string_value(_:*const c_char)->*mut c_char;
    //fn change_flag(flag:c_int, on_or_off:c_int)->c_int;
    fn decode_prompt_string(string:*mut c_char)->*mut c_char;
    fn sh_contains_shell_metas(string:*const c_char)->c_int;
    fn sh_single_quote(string:*mut c_char)->*mut c_char;
    fn ansic_shouldquote(string:*const c_char)->c_int;
    fn ansic_quote(str:*mut c_char, flags:c_int, rlen:*mut c_int)->*mut c_char;
    fn dispose_redirects(list:*mut REDIRECT);
    fn add_unwind_protect(cleanup:*mut Function, arg:*mut c_char);

    fn remove_unwind_protect();
    fn dispose_command(command:*mut COMMAND);
    fn find_reserved_word(tokstr:*mut c_char)->c_int;
    fn remove_quoted_escapes(string:*mut c_char)->*mut c_char;
    fn mblen(s:*const c_char, n:size_t)->c_int;
    fn cprintf(control:*const c_char,...);
}

#[no_mangle]
pub unsafe extern "C" fn r_print_command(command:*mut COMMAND)
{
    let _str:*mut c_char;
    command_string_index = 0;
    print!("{}",CStr::from_ptr(r_make_command_string(command)).to_str().unwrap());

}

#[no_mangle]
pub unsafe extern "C" fn r_make_command_string(command:*mut COMMAND) -> *mut c_char
{
    command_string_index = 0;
    was_heredoc = 0;
    deferred_heredocs = 0 as *mut REDIRECT;
    make_command_string_internal(command);
    the_printed_command
        
}

#[no_mangle]
unsafe fn make_command_string_internal(command:*mut COMMAND)
{
    let mut s:[c_char;3] = [0;3];

    if command.is_null(){
        cprintf_1 (b"\0" as *const u8 as *const i8);
    }
    else
    {
        if skip_this_indent != 0 {
            skip_this_indent = skip_this_indent - 1;
        }
        else{
            indent(indentation);
        }
        if (*command).flags != 0 && CMD_TIME_PIPELINE != 0{
            cprintf_1(b"time " as *const u8 as *const i8);
            if (*command).flags != 0 && CMD_TIME_POSIX != 0{
                cprintf_1(b"-p " as *const u8 as *const i8);
            }
        }
        if (*command).flags != 0 && CMD_INVERT_RETURN != 0{
            cprintf_1(b"! " as *const u8 as *const i8);
        }   

        // (*command).type_ = 11;
        match (*command).type_ as libc::c_uint {
            command_type_cm_for => print_for_command((*command).value.For),
            command_type_cm_arith_for => print_arith_for_command((*command).value.ArithFor),
            command_type_cm_select => print_select_command((*command).value.Select),
            command_type_cm_case => print_case_command((*command).value.Case),
            command_type_cm_while => print_while_command((*command).value.While),
            command_type_cm_until => print_until_command((*command).value.While),
            command_type_cm_if => print_if_command((*command).value.If), 
            command_type_cm_arith => r_print_arith_command((*(*command).value.Arith).exp),
            command_type_cm_cond => r_print_cond_command((*command).value.Cond),
            command_type_cm_simple => r_print_simple_command((*command).value.Simple),
            command_type_cm_connection => {
                skip_this_indent = skip_this_indent + 1;
                printing_connection = printing_connection + 1;
                make_command_string_internal((*(*command).value.Connection).first);
                let _str_1 = b'&' as i32;
                let _str_1 = b'|' as i32;
                let _str_3 = b';' as i32;
                match (*(*command).value.Connection).connector{          
                    38 | 124 => {
                        let c:c_char = (*(*command).value.Connection).connector as c_char;
                        s[0] = ' ' as i32 as c_char;
                        s[1] = c;
                        s[2] = '\u{0}' as i32 as c_char;
                        print_deferred_heredocs(s.as_mut_ptr());
                        if c as c_int != '&' as i32 || !((*(*command).value.Connection).second.is_null())
                        {
                            cprintf_1(b" \0" as *const u8 as *const i8);
                            skip_this_indent = skip_this_indent + 1;
                        }
                    }
               
                    AND_AND => {
                        print_deferred_heredocs(b" && \0" as *const u8 as *const c_char);
                        if !(*(*command).value.Connection).second.is_null()
                        {
                            skip_this_indent = skip_this_indent + 1;
                        }
                    }

                    OR_OR => {
                        print_deferred_heredocs(b" || \0" as *const u8 as *const c_char);
                        if !(*(*command).value.Connection).second.is_null() {
                            skip_this_indent = skip_this_indent + 1;
                        }
                    }                    
                   
                    59 => {
                        if deferred_heredocs.is_null() {
                            if was_heredoc == 0 
                            {
                                cprintf_1(b";\0" as *const u8 as *const c_char);
                            }
                            else{
                                was_heredoc = 0;
                            }
                        }
                        
                        else{
                            print_deferred_heredocs(
                                if inside_function_def != 0{
                                    b"\0" as *const u8 as *const c_char
                                }
                                else{
                                    b";\0" as *const u8 as *const c_char
                                },
                            )
                        }
                        
                        if inside_function_def != 0 {
                            cprintf_1(b"\n\0" as *const u8 as *const c_char);
                        }
                        
                        else{
                            cprintf_1(b" \0" as *const u8 as *const c_char);
                            if !(*(*command).value.Connection).second.is_null() {
                                skip_this_indent = skip_this_indent + 1;
                            }
                        }                    
                    }
         
                    _ => {
                        // cprintf_2( CString::new("print_command:bad connector").as_mut() as *const c_char, (*(*command).value.Connection).connector);
                        let mut str = format!("print_command:bas connector {}\0", (*(*command).value.Connection).connector);
                        cprintf_1(str.as_mut_ptr() as *mut c_char);
                    }      
                }
                make_command_string_internal((*(*command).value.Connection).second);
                PRINT_DEFERRED_HEREDOCS!(b"\0" as *const u8 as *const c_char);
                printing_connection = printing_connection - 1;    
            }
            command_type_cm_function_def => print_function_def((*command).value.Function_def),
            command_type_cm_group => print_group_command((*command).value.Group),
            command_type_cm_subshell =>{
                cprintf_1(b"( \0" as *const u8 as *const c_char);
                skip_this_indent = skip_this_indent + 1;
                make_command_string_internal((*(*command).value.Subshell).command);
                PRINT_DEFERRED_HEREDOCS!(b"\0" as *const u8 as *const c_char);
                cprintf_1(b" )\0" as *const u8 as *const c_char);
            
            }
            
            command_type_cm_coproc => {
                let mut str = format!("coproc {}\0", CStr::from_ptr((*(*command).value.Coproc).name).to_str().unwrap());
                cprintf_1(str.as_mut_ptr() as *mut c_char);
                skip_this_indent += 1;
                make_command_string_internal((*(*command).value.Coproc).command);
            }

            _ => {
                // let c_str = CString::new("print_command").unwrap();
                // let c_str_ptr = c_str.as_ptr();
                command_error(b"print_command\0" as *const u8 as *const c_char, CMDERR_BADTYPE as i32, (*command).type_ as i32, 0 as i32);
            }
        }

        if !(*command).redirects.is_null() {
            cprintf_1(b" \0" as *const u8 as *const c_char); 
            print_redirection_list((*command).redirects);
        }

}
