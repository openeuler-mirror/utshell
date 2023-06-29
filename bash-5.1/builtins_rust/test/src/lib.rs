use libc::{c_int, c_char, c_void};
use rcommon::{r_make_builtin_argv,WordList};
include!(concat!("intercdep.rs"));

#[no_mangle]
pub extern "C" fn r_test_builtin(list: *mut WordList) -> i32 {

    let result: c_int;
    let mut argc: c_int = 0;
unsafe {
    if list.is_null() {
        if *this_command_name == b'[' as c_char &&
            *((this_command_name as usize + 1) as *mut c_char) == 0 {
            builtin_error("missing `]'\0".as_ptr() as *mut c_char);
            return EX_BADUSAGE;
        }
        return EXECUTION_FAILURE;
    }
    let argv = r_make_builtin_argv(list, std::mem::transmute(&argc));
    result = test_command(argc, argv);
    libc::free(argv as *mut c_void);
}
    return result;
}
