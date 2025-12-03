use crate::array::array_reference;
use crate::execute_cmd::executing_line_number;
use crate::general::base_pathname;
use crate::src_common::*;
use crate::variables::find_variable;
use std::convert::TryInto;
#[no_mangle]
pub static mut gnu_error_format: libc::c_int = 0 as libc::c_int;

//该文件中有多个可变餐函数，在variables/error.c中实现
#[no_mangle]
pub fn get_name_for_error() -> *mut libc::c_char {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bash_source_v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut bash_source_a: *mut ARRAY = 0 as *mut ARRAY;
    name = 0 as *mut libc::c_void as *mut libc::c_char;
    unsafe {
        if interactive_shell == 0 as libc::c_int {
            bash_source_v = find_variable(b"BASH_SOURCE\0" as *const u8 as *const libc::c_char);
            if !bash_source_v.is_null()
                && (*bash_source_v).attributes & 0x4 as libc::c_int != 0
                && {
                    bash_source_a = (*bash_source_v).value as *mut ARRAY;
                    !bash_source_a.is_null()
                }
            {
                name = array_reference(bash_source_a, 0 as libc::c_int as arrayind_t);
            }
            if name.is_null() || *name as libc::c_int == '\u{0}' as i32 {
                name = *dollar_vars.as_mut_ptr().offset(0 as libc::c_int as isize);
            }
        }
        if name.is_null() && !shell_name.is_null() && *shell_name as libc::c_int != 0 {
            name = base_pathname(shell_name);
        }
    }
    if name.is_null() {
        name = b"bash\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    return name;
}

#[no_mangle]
pub fn file_error(mut filename: *const libc::c_char) {
    unsafe {
        report_error(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            filename,
            strerror(*__errno_location()),
        );
    }
}

static mut cmd_error_table: [*const libc::c_char; 5] = [
    b"unknown command error\0" as *const u8 as *const libc::c_char,
    b"bad command type\0" as *const u8 as *const libc::c_char,
    b"bad connector\0" as *const u8 as *const libc::c_char,
    b"bad jump\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];

#[no_mangle]
pub fn command_error(
    mut func: *const libc::c_char,
    mut code: libc::c_int,
    mut e: libc::c_int,
    mut flags: libc::c_int,
) {
    if code > CMDERR_LAST.try_into().unwrap() {
        code = CMDERR_DEFAULT as i32;
    }
    unsafe {
        programming_error(
            b"%s: %s: %d\0" as *const u8 as *const libc::c_char,
            func,
            dcgettext(
                0 as *const libc::c_char,
                cmd_error_table[code as usize],
                5 as libc::c_int,
            ),
            e,
        );
    }
}

#[no_mangle]
pub fn command_errstr(mut code: libc::c_int) -> *mut libc::c_char {
    if code > CMDERR_LAST.try_into().unwrap() {
        code = CMDERR_DEFAULT as i32;
    }
    unsafe {
        return dcgettext(
            0 as *const libc::c_char,
            cmd_error_table[code as usize],
            5 as libc::c_int,
        );
    }
}

#[no_mangle]
pub fn err_badarraysub(mut s: *const libc::c_char) {
    unsafe {
        report_error(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            s,
            dcgettext(
                0 as *const libc::c_char,
                bash_badsub_errmsg,
                5 as libc::c_int,
            ),
        );
    }
}

#[no_mangle]
pub fn err_unboundvar(mut s: *const libc::c_char) {
    unsafe {
        report_error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: unbound variable\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            s,
        );
    }
}

#[no_mangle]
pub fn err_readonly(mut s: *const libc::c_char) {
    unsafe {
        report_error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: readonly variable\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            s,
        );
    }
}
