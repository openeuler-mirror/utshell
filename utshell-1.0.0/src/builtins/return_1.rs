use super::help::builtin_help;
use crate::builtins::common::get_exitstat;
use crate::src_common::*;

#[no_mangle]
pub fn return_builtin(list: *mut WordList) -> i32 {
    unsafe {
        if !list.is_null()
            && !(*list).word.is_null()
            && libc::strcmp(
                (*((*list).word)).word,
                "--help\0".as_ptr() as *const libc::c_char,
            ) == 0
        {
            builtin_help();
            return EX_USAGE;
        }

        return_catch_value = get_exitstat(list);
        if return_catch_flag != 0 {
            c_siglongjmp(std::mem::transmute(&return_catch), 1);
        } else {
            builtin_error(
                "can only `return' from a function or sourced script\0".as_ptr()
                    as *const libc::c_char,
            );
            return EX_USAGE;
        }
    }
}
