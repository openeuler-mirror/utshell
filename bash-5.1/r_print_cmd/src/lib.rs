
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
