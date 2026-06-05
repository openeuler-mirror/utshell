#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unreachable_code,
    non_snake_case,
    unreachable_patterns,
    unused_variables,
    clashing_extern_declarations
)]
use crate::alias::*;
use crate::arrayfunc::*;
use crate::bashhist::*;
use crate::bashline::*;
use crate::builtins::common::{builtin_address_internal, read_octal};
use crate::builtins::evalstring::parse_string;
use crate::builtins::exit::exit_builtin;
use crate::dispose_cmd::*;
use crate::general::*;
use crate::general::{legal_identifier, legal_number};
use crate::input::*;
use crate::jobs::*;
use crate::local::*;
use crate::mailcheck::*;
use crate::make_cmd::*;
use crate::parse_and_execute;
use crate::readline::c_clearerr;
use crate::sig::*;
use crate::src_common;
use crate::src_common::alias;
use crate::src_common::builtin;
use crate::src_common::*;
use crate::stringlib::*;
use crate::subst::*;
pub use crate::syntax::sh_syntaxtab;
use crate::test::*;
use crate::trap::*;
use crate::utshell::exit_shell;
use crate::utshell::get_current_user_info;
use crate::utshell::set_exit_status;
use crate::variables::bind_variable;
use crate::variables::get_string_value;
use crate::variables::restore_pipestatus_array;
use crate::variables::save_pipestatus_array;
use crate::variables::set_pipestatus_from_exit;
use crate::variables::sv_tz;
use crate::version::dist_version;
use crate::version::*;
extern "C" {
    static mut stdin: *mut FILE;
}

pub type yy_state_t = yytype_int16;
pub type yytype_int16 = libc::c_short;

pub type yysymbol_kind_t = libc::c_int;
pub const YYSYMBOL_timespec: yysymbol_kind_t = 98;
pub const YYSYMBOL_pipeline: yysymbol_kind_t = 97;
pub const YYSYMBOL_pipeline_command: yysymbol_kind_t = 96;
pub const YYSYMBOL_simple_list1: yysymbol_kind_t = 95;
pub const YYSYMBOL_simple_list: yysymbol_kind_t = 94;
pub const YYSYMBOL_newline_list: yysymbol_kind_t = 93;
pub const YYSYMBOL_list_terminator: yysymbol_kind_t = 92;
pub const YYSYMBOL_simple_list_terminator: yysymbol_kind_t = 91;
pub const YYSYMBOL_list1: yysymbol_kind_t = 90;
pub const YYSYMBOL_list0: yysymbol_kind_t = 89;
pub const YYSYMBOL_compound_list: yysymbol_kind_t = 88;
pub const YYSYMBOL_list: yysymbol_kind_t = 87;
pub const YYSYMBOL_pattern: yysymbol_kind_t = 86;
pub const YYSYMBOL_case_clause_sequence: yysymbol_kind_t = 85;
pub const YYSYMBOL_pattern_list: yysymbol_kind_t = 84;
pub const YYSYMBOL_case_clause: yysymbol_kind_t = 83;
pub const YYSYMBOL_elif_clause: yysymbol_kind_t = 82;
pub const YYSYMBOL_cond_command: yysymbol_kind_t = 81;
pub const YYSYMBOL_arith_command: yysymbol_kind_t = 80;
pub const YYSYMBOL_group_command: yysymbol_kind_t = 79;
pub const YYSYMBOL_if_command: yysymbol_kind_t = 78;
pub const YYSYMBOL_coproc: yysymbol_kind_t = 77;
pub const YYSYMBOL_subshell: yysymbol_kind_t = 76;
pub const YYSYMBOL_function_body: yysymbol_kind_t = 75;
pub const YYSYMBOL_function_def: yysymbol_kind_t = 74;
pub const YYSYMBOL_case_command: yysymbol_kind_t = 73;
pub const YYSYMBOL_select_command: yysymbol_kind_t = 72;
pub const YYSYMBOL_arith_for_command: yysymbol_kind_t = 71;
pub const YYSYMBOL_for_command: yysymbol_kind_t = 70;
pub const YYSYMBOL_shell_command: yysymbol_kind_t = 69;
pub const YYSYMBOL_command: yysymbol_kind_t = 68;
pub const YYSYMBOL_simple_command: yysymbol_kind_t = 67;
pub const YYSYMBOL_redirection_list: yysymbol_kind_t = 66;
pub const YYSYMBOL_simple_command_element: yysymbol_kind_t = 65;
pub const YYSYMBOL_redirection: yysymbol_kind_t = 64;
pub const YYSYMBOL_word_list: yysymbol_kind_t = 63;
pub const YYSYMBOL_inputunit: yysymbol_kind_t = 62;
pub const YYSYMBOL_YYACCEPT: yysymbol_kind_t = 61;
pub const YYSYMBOL_60_: yysymbol_kind_t = 60;
pub const YYSYMBOL_59_: yysymbol_kind_t = 59;
pub const YYSYMBOL_58_: yysymbol_kind_t = 58;
pub const YYSYMBOL_57_: yysymbol_kind_t = 57;
pub const YYSYMBOL_56_: yysymbol_kind_t = 56;
pub const YYSYMBOL_55_: yysymbol_kind_t = 55;
pub const YYSYMBOL_54_: yysymbol_kind_t = 54;
pub const YYSYMBOL_53_: yysymbol_kind_t = 53;
pub const YYSYMBOL_yacc_EOF: yysymbol_kind_t = 52;
pub const YYSYMBOL_51_n_: yysymbol_kind_t = 51;
pub const YYSYMBOL_50_: yysymbol_kind_t = 50;
pub const YYSYMBOL_49_: yysymbol_kind_t = 49;
pub const YYSYMBOL_BAR_AND: yysymbol_kind_t = 48;
pub const YYSYMBOL_GREATER_BAR: yysymbol_kind_t = 47;
pub const YYSYMBOL_LESS_GREATER: yysymbol_kind_t = 46;
pub const YYSYMBOL_AND_GREATER_GREATER: yysymbol_kind_t = 45;
pub const YYSYMBOL_AND_GREATER: yysymbol_kind_t = 44;
pub const YYSYMBOL_LESS_LESS_MINUS: yysymbol_kind_t = 43;
pub const YYSYMBOL_SEMI_SEMI_AND: yysymbol_kind_t = 42;
pub const YYSYMBOL_SEMI_AND: yysymbol_kind_t = 41;
pub const YYSYMBOL_SEMI_SEMI: yysymbol_kind_t = 40;
pub const YYSYMBOL_GREATER_AND: yysymbol_kind_t = 39;
pub const YYSYMBOL_LESS_LESS_LESS: yysymbol_kind_t = 38;
pub const YYSYMBOL_LESS_AND: yysymbol_kind_t = 37;
pub const YYSYMBOL_LESS_LESS: yysymbol_kind_t = 36;
pub const YYSYMBOL_GREATER_GREATER: yysymbol_kind_t = 35;
pub const YYSYMBOL_OR_OR: yysymbol_kind_t = 34;
pub const YYSYMBOL_AND_AND: yysymbol_kind_t = 33;
pub const YYSYMBOL_COND_CMD: yysymbol_kind_t = 32;
pub const YYSYMBOL_ARITH_FOR_EXPRS: yysymbol_kind_t = 31;
pub const YYSYMBOL_ARITH_CMD: yysymbol_kind_t = 30;
pub const YYSYMBOL_NUMBER: yysymbol_kind_t = 29;
pub const YYSYMBOL_REDIR_WORD: yysymbol_kind_t = 28;
pub const YYSYMBOL_ASSIGNMENT_WORD: yysymbol_kind_t = 27;
pub const YYSYMBOL_WORD: yysymbol_kind_t = 26;
pub const YYSYMBOL_TIMEIGN: yysymbol_kind_t = 25;
pub const YYSYMBOL_TIMEOPT: yysymbol_kind_t = 24;
pub const YYSYMBOL_TIME: yysymbol_kind_t = 23;
pub const YYSYMBOL_BANG: yysymbol_kind_t = 22;
pub const YYSYMBOL_IN: yysymbol_kind_t = 21;
pub const YYSYMBOL_COND_ERROR: yysymbol_kind_t = 20;
pub const YYSYMBOL_COND_END: yysymbol_kind_t = 19;
pub const YYSYMBOL_COND_START: yysymbol_kind_t = 18;
pub const YYSYMBOL_COPROC: yysymbol_kind_t = 17;
pub const YYSYMBOL_FUNCTION: yysymbol_kind_t = 16;
pub const YYSYMBOL_DONE: yysymbol_kind_t = 15;
pub const YYSYMBOL_DO: yysymbol_kind_t = 14;
pub const YYSYMBOL_UNTIL: yysymbol_kind_t = 13;
pub const YYSYMBOL_WHILE: yysymbol_kind_t = 12;
pub const YYSYMBOL_SELECT: yysymbol_kind_t = 11;
pub const YYSYMBOL_FOR: yysymbol_kind_t = 10;
pub const YYSYMBOL_ESAC: yysymbol_kind_t = 9;
pub const YYSYMBOL_CASE: yysymbol_kind_t = 8;
pub const YYSYMBOL_FI: yysymbol_kind_t = 7;
pub const YYSYMBOL_ELIF: yysymbol_kind_t = 6;
pub const YYSYMBOL_ELSE: yysymbol_kind_t = 5;
pub const YYSYMBOL_THEN: yysymbol_kind_t = 4;
pub const YYSYMBOL_IF: yysymbol_kind_t = 3;
pub const YYSYMBOL_YYUNDEF: yysymbol_kind_t = 2;
pub const YYSYMBOL_YYerror: yysymbol_kind_t = 1;
pub const YYSYMBOL_YYEOF: yysymbol_kind_t = 0;
pub const YYSYMBOL_YYEMPTY: yysymbol_kind_t = -2;
pub type yytype_int8 = libc::c_schar;
pub type STRING_SAVER = string_saver;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct string_saver {
    pub next: *mut string_saver,
    pub expand_alias: libc::c_int,
    pub saved_line: *mut libc::c_char,
    pub expander: *mut alias_t,
    pub saved_line_size: size_t,
    pub saved_line_index: size_t,
    pub saved_line_terminator: libc::c_int,
    pub flags: libc::c_int,
}
pub type alias_t = alias;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct dstack {
    pub delimiters: *mut libc::c_char,
    pub delimiter_depth: libc::c_int,
    pub delimiter_space: libc::c_int,
}
pub type yy_state_fast_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub union INPUT_STREAM {
    pub file: *mut FILE,
    pub string: *mut libc::c_char,
    pub buffered_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BASH_INPUT {
    pub type_0: stream_type,
    pub name: *mut libc::c_char,
    pub location: INPUT_STREAM,
    pub getter: Option<sh_cget_func_t>,
    pub ungetter: Option<sh_cunget_func_t>,
}
pub type sh_cunget_func_t = fn(libc::c_int) -> libc::c_int;
pub type sh_cget_func_t = fn() -> libc::c_int;
pub type stream_type = libc::c_uint;
pub const st_bstream: stream_type = 4;
pub const st_string: stream_type = 3;
pub const st_stream: stream_type = 2;
pub const st_stdin: stream_type = 1;
pub const st_none: stream_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct user_info {
    pub uid: uid_t,
    pub euid: uid_t,
    pub gid: gid_t,
    pub egid: gid_t,
    pub user_name: *mut libc::c_char,
    pub shell: *mut libc::c_char,
    pub home_dir: *mut libc::c_char,
}
pub type yytype_uint8 = libc::c_uchar;

#[derive(Copy, Clone)]
#[repr(C)]
pub union yyalloc {
    pub yyss_alloc: yy_state_t,
    pub yyvs_alloc: YYSTYPE,
}

pub type sh_input_line_state_t = _sh_input_line_state_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sh_input_line_state_t {
    pub input_line: *mut libc::c_char,
    pub input_line_index: size_t,
    pub input_line_size: size_t,
    pub input_line_len: size_t,
    pub input_property: *mut libc::c_char,
    pub input_propsize: size_t,
}
pub type STREAM_SAVER = stream_saver;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_saver {
    pub next: *mut stream_saver,
    pub bash_input: BASH_INPUT,
    pub line: libc::c_int,
    pub bstream: *mut BUFFERED_STREAM,
}
pub type BUFFERED_STREAM = BSTREAM;

#[inline]
fn mbrlen(mut __s: *const libc::c_char, mut __n: size_t, mut __ps: *mut mbstate_t) -> size_t {
    return if !__ps.is_null() {
        c_mbrtowc(0 as *mut wchar_t, __s, __n, __ps)
    } else {
        c___mbrlen(__s, __n, 0 as *mut mbstate_t)
    };
}
static mut shell_input_line_property: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut shell_input_line_propsize: size_t = 0 as libc::c_int as size_t;

#[no_mangle]
pub static mut ps1_prompt: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut ps2_prompt: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut ps0_prompt: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut prompt_string_pointer: *mut *mut libc::c_char =
    0 as *const libc::c_void as *mut libc::c_void as *mut *mut libc::c_char;
#[no_mangle]
pub static mut current_prompt_string: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut expand_aliases: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut promptvars: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut extended_quote: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut current_command_line_count: libc::c_int = 0;
#[no_mangle]
pub static mut saved_command_line_count: libc::c_int = 0;
#[no_mangle]
pub static mut shell_eof_token: libc::c_int = 0;
#[no_mangle]
pub static mut current_token: libc::c_int = 0;
#[no_mangle]
pub static mut parser_state: libc::c_int = 0;
static mut redir_stack: [*mut REDIRECT; 16] = [0 as *const REDIRECT as *mut REDIRECT; 16];
#[no_mangle]
pub static mut need_here_doc: libc::c_int = 0;
static mut shell_input_line: *mut libc::c_char =
    0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char;
static mut shell_input_line_index: size_t = 0;
static mut shell_input_line_size: size_t = 0;
static mut shell_input_line_len: size_t = 0;
static mut shell_input_line_terminator: libc::c_int = 0;
static mut function_dstart: libc::c_int = 0;
static mut function_bstart: libc::c_int = 0;
static mut arith_for_lineno: libc::c_int = 0;
static mut current_decoded_prompt: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut last_read_token: libc::c_int = 0;
static mut token_before_that: libc::c_int = 0;
static mut two_tokens_ago: libc::c_int = 0;
static mut global_extglob: libc::c_int = 0;
static mut word_lineno: [libc::c_int; 129] = [0; 129];
static mut word_top: libc::c_int = -(1 as libc::c_int);
static mut token_to_read: libc::c_int = 0;
static mut word_desc_to_read: *mut WORD_DESC = 0 as *const WORD_DESC as *mut WORD_DESC;
static mut source: REDIRECTEE = REDIRECTEE { dest: 0 };
static mut redir: REDIRECTEE = REDIRECTEE { dest: 0 };
static mut yytranslate: [yytype_int8; 305] = [
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    59 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    55 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    57 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    58 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    48 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
];
static mut yypact: [yytype_int16; 346] = [
    313 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(6 as libc::c_int) as yytype_int16,
    8 as libc::c_int as yytype_int16,
    2 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    10 as libc::c_int as yytype_int16,
    513 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    363 as libc::c_int as yytype_int16,
    153 as libc::c_int as yytype_int16,
    -(21 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    593 as libc::c_int as yytype_int16,
    606 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    14 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    127 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    95 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    100 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    65 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    551 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    572 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    146 as libc::c_int as yytype_int16,
    140 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    67 as libc::c_int as yytype_int16,
    363 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    133 as libc::c_int as yytype_int16,
    413 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    93 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    156 as libc::c_int as yytype_int16,
    161 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    551 as libc::c_int as yytype_int16,
    572 as libc::c_int as yytype_int16,
    163 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    167 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    152 as libc::c_int as yytype_int16,
    208 as libc::c_int as yytype_int16,
    217 as libc::c_int as yytype_int16,
    129 as libc::c_int as yytype_int16,
    220 as libc::c_int as yytype_int16,
    150 as libc::c_int as yytype_int16,
    221 as libc::c_int as yytype_int16,
    223 as libc::c_int as yytype_int16,
    225 as libc::c_int as yytype_int16,
    233 as libc::c_int as yytype_int16,
    234 as libc::c_int as yytype_int16,
    238 as libc::c_int as yytype_int16,
    239 as libc::c_int as yytype_int16,
    158 as libc::c_int as yytype_int16,
    240 as libc::c_int as yytype_int16,
    162 as libc::c_int as yytype_int16,
    241 as libc::c_int as yytype_int16,
    243 as libc::c_int as yytype_int16,
    244 as libc::c_int as yytype_int16,
    252 as libc::c_int as yytype_int16,
    253 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    194 as libc::c_int as yytype_int16,
    227 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    572 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    463 as libc::c_int as yytype_int16,
    463 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(7 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    59 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    52 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    62 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    235 as libc::c_int as yytype_int16,
    572 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    572 as libc::c_int as yytype_int16,
    572 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    413 as libc::c_int as yytype_int16,
    413 as libc::c_int as yytype_int16,
    191 as libc::c_int as yytype_int16,
    191 as libc::c_int as yytype_int16,
    245 as libc::c_int as yytype_int16,
    245 as libc::c_int as yytype_int16,
    203 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    37 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    176 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    270 as libc::c_int as yytype_int16,
    228 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    176 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    278 as libc::c_int as yytype_int16,
    282 as libc::c_int as yytype_int16,
    563 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    572 as libc::c_int as yytype_int16,
    572 as libc::c_int as yytype_int16,
    563 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    67 as libc::c_int as yytype_int16,
    67 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    291 as libc::c_int as yytype_int16,
    413 as libc::c_int as yytype_int16,
    413 as libc::c_int as yytype_int16,
    413 as libc::c_int as yytype_int16,
    413 as libc::c_int as yytype_int16,
    413 as libc::c_int as yytype_int16,
    294 as libc::c_int as yytype_int16,
    175 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    28 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    292 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    187 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    250 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    295 as libc::c_int as yytype_int16,
    413 as libc::c_int as yytype_int16,
    187 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    251 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    563 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    304 as libc::c_int as yytype_int16,
    314 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    196 as libc::c_int as yytype_int16,
    196 as libc::c_int as yytype_int16,
    196 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    179 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    296 as libc::c_int as yytype_int16,
    -(28 as libc::c_int) as yytype_int16,
    302 as libc::c_int as yytype_int16,
    274 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    87 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    318 as libc::c_int as yytype_int16,
    276 as libc::c_int as yytype_int16,
    322 as libc::c_int as yytype_int16,
    280 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(7 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    111 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    39 as libc::c_int as yytype_int16,
    319 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    114 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    115 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    226 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    413 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    329 as libc::c_int as yytype_int16,
    288 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    332 as libc::c_int as yytype_int16,
    297 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    413 as libc::c_int as yytype_int16,
    338 as libc::c_int as yytype_int16,
    303 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    339 as libc::c_int as yytype_int16,
    305 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
];
static mut yydefact: [yytype_uint8; 346] = [
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    169 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    115 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    168 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    75 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    66 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    71 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    70 as libc::c_int as yytype_uint8,
    72 as libc::c_int as yytype_uint8,
    73 as libc::c_int as yytype_uint8,
    74 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    153 as libc::c_int as yytype_uint8,
    160 as libc::c_int as yytype_uint8,
    161 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    135 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    110 as libc::c_int as yytype_uint8,
    106 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    149 as libc::c_int as yytype_uint8,
    148 as libc::c_int as yytype_uint8,
    150 as libc::c_int as yytype_uint8,
    165 as libc::c_int as yytype_uint8,
    162 as libc::c_int as yytype_uint8,
    170 as libc::c_int as yytype_uint8,
    171 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    39 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    9 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    62 as libc::c_int as yytype_uint8,
    146 as libc::c_int as yytype_uint8,
    147 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    154 as libc::c_int as yytype_uint8,
    155 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    164 as libc::c_int as yytype_uint8,
    163 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    152 as libc::c_int as yytype_uint8,
    134 as libc::c_int as yytype_uint8,
    136 as libc::c_int as yytype_uint8,
    145 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    101 as libc::c_int as yytype_uint8,
    108 as libc::c_int as yytype_uint8,
    107 as libc::c_int as yytype_uint8,
    116 as libc::c_int as yytype_uint8,
    172 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    35 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    38 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    23 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
    34 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    37 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    22 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    114 as libc::c_int as yytype_uint8,
    105 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    158 as libc::c_int as yytype_uint8,
    159 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    104 as libc::c_int as yytype_uint8,
    109 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    156 as libc::c_int as yytype_uint8,
    157 as libc::c_int as yytype_uint8,
    167 as libc::c_int as yytype_uint8,
    166 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    138 as libc::c_int as yytype_uint8,
    139 as libc::c_int as yytype_uint8,
    137 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    120 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    86 as libc::c_int as yytype_uint8,
    87 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    67 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    102 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    99 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    140 as libc::c_int as yytype_uint8,
    141 as libc::c_int as yytype_uint8,
    142 as libc::c_int as yytype_uint8,
    143 as libc::c_int as yytype_uint8,
    144 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    126 as libc::c_int as yytype_uint8,
    128 as libc::c_int as yytype_uint8,
    130 as libc::c_int as yytype_uint8,
    121 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    132 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    76 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    77 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    88 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    89 as libc::c_int as yytype_uint8,
    100 as libc::c_int as yytype_uint8,
    112 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    131 as libc::c_int as yytype_uint8,
    97 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    78 as libc::c_int as yytype_uint8,
    79 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    84 as libc::c_int as yytype_uint8,
    85 as libc::c_int as yytype_uint8,
    90 as libc::c_int as yytype_uint8,
    91 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    117 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    133 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    123 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    119 as libc::c_int as yytype_uint8,
    124 as libc::c_int as yytype_uint8,
    125 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    82 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    94 as libc::c_int as yytype_uint8,
    95 as libc::c_int as yytype_uint8,
    118 as libc::c_int as yytype_uint8,
    80 as libc::c_int as yytype_uint8,
    81 as libc::c_int as yytype_uint8,
    92 as libc::c_int as yytype_uint8,
    93 as libc::c_int as yytype_uint8,
];
static mut yypgoto: [yytype_int16; 38] = [
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    117 as libc::c_int as yytype_int16,
    -(37 as libc::c_int) as yytype_int16,
    -(19 as libc::c_int) as yytype_int16,
    -(67 as libc::c_int) as yytype_int16,
    353 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(8 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(184 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    53 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    142 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    102 as libc::c_int as yytype_int16,
    -(203 as libc::c_int) as yytype_int16,
    -(2 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    283 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(47 as libc::c_int) as yytype_int16,
    -(49 as libc::c_int) as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
    -(118 as libc::c_int) as yytype_int16,
    6 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(204 as libc::c_int) as yytype_int16,
];
static mut yydefgoto: [yytype_int16; 38] = [
    0 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    241 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    122 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    152 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    227 as libc::c_int as yytype_int16,
    233 as libc::c_int as yytype_int16,
    234 as libc::c_int as yytype_int16,
    235 as libc::c_int as yytype_int16,
    277 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    136 as libc::c_int as yytype_int16,
    137 as libc::c_int as yytype_int16,
    125 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    60 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    138 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
];
static mut yytable: [yytype_int16; 662] = [
    59 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    121 as libc::c_int as yytype_int16,
    154 as libc::c_int as yytype_int16,
    65 as libc::c_int as yytype_int16,
    66 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    250 as libc::c_int as yytype_int16,
    132 as libc::c_int as yytype_int16,
    254 as libc::c_int as yytype_int16,
    191 as libc::c_int as yytype_int16,
    192 as libc::c_int as yytype_int16,
    139 as libc::c_int as yytype_int16,
    141 as libc::c_int as yytype_int16,
    2 as libc::c_int as yytype_int16,
    146 as libc::c_int as yytype_int16,
    144 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    120 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    302 as libc::c_int as yytype_int16,
    196 as libc::c_int as yytype_int16,
    197 as libc::c_int as yytype_int16,
    64 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    257 as libc::c_int as yytype_int16,
    303 as libc::c_int as yytype_int16,
    121 as libc::c_int as yytype_int16,
    62 as libc::c_int as yytype_int16,
    259 as libc::c_int as yytype_int16,
    67 as libc::c_int as yytype_int16,
    274 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    198 as libc::c_int as yytype_int16,
    199 as libc::c_int as yytype_int16,
    200 as libc::c_int as yytype_int16,
    287 as libc::c_int as yytype_int16,
    288 as libc::c_int as yytype_int16,
    300 as libc::c_int as yytype_int16,
    2 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    120 as libc::c_int as yytype_int16,
    237 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    275 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    151 as libc::c_int as yytype_int16,
    153 as libc::c_int as yytype_int16,
    133 as libc::c_int as yytype_int16,
    149 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    275 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    203 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    142 as libc::c_int as yytype_int16,
    150 as libc::c_int as yytype_int16,
    220 as libc::c_int as yytype_int16,
    221 as libc::c_int as yytype_int16,
    204 as libc::c_int as yytype_int16,
    294 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    210 as libc::c_int as yytype_int16,
    189 as libc::c_int as yytype_int16,
    190 as libc::c_int as yytype_int16,
    135 as libc::c_int as yytype_int16,
    201 as libc::c_int as yytype_int16,
    193 as libc::c_int as yytype_int16,
    194 as libc::c_int as yytype_int16,
    211 as libc::c_int as yytype_int16,
    217 as libc::c_int as yytype_int16,
    188 as libc::c_int as yytype_int16,
    218 as libc::c_int as yytype_int16,
    276 as libc::c_int as yytype_int16,
    135 as libc::c_int as yytype_int16,
    135 as libc::c_int as yytype_int16,
    246 as libc::c_int as yytype_int16,
    202 as libc::c_int as yytype_int16,
    302 as libc::c_int as yytype_int16,
    248 as libc::c_int as yytype_int16,
    238 as libc::c_int as yytype_int16,
    208 as libc::c_int as yytype_int16,
    209 as libc::c_int as yytype_int16,
    276 as libc::c_int as yytype_int16,
    109 as libc::c_int as yytype_int16,
    317 as libc::c_int as yytype_int16,
    215 as libc::c_int as yytype_int16,
    307 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    135 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    219 as libc::c_int as yytype_int16,
    205 as libc::c_int as yytype_int16,
    135 as libc::c_int as yytype_int16,
    110 as libc::c_int as yytype_int16,
    143 as libc::c_int as yytype_int16,
    135 as libc::c_int as yytype_int16,
    121 as libc::c_int as yytype_int16,
    130 as libc::c_int as yytype_int16,
    121 as libc::c_int as yytype_int16,
    188 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    212 as libc::c_int as yytype_int16,
    131 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    337 as libc::c_int as yytype_int16,
    338 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    314 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    135 as libc::c_int as yytype_int16,
    321 as libc::c_int as yytype_int16,
    325 as libc::c_int as yytype_int16,
    135 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    195 as libc::c_int as yytype_int16,
    247 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    249 as libc::c_int as yytype_int16,
    134 as libc::c_int as yytype_int16,
    135 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    206 as libc::c_int as yytype_int16,
    207 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    140 as libc::c_int as yytype_int16,
    308 as libc::c_int as yytype_int16,
    213 as libc::c_int as yytype_int16,
    214 as libc::c_int as yytype_int16,
    228 as libc::c_int as yytype_int16,
    229 as libc::c_int as yytype_int16,
    230 as libc::c_int as yytype_int16,
    231 as libc::c_int as yytype_int16,
    232 as libc::c_int as yytype_int16,
    236 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    145 as libc::c_int as yytype_int16,
    160 as libc::c_int as yytype_int16,
    107 as libc::c_int as yytype_int16,
    242 as libc::c_int as yytype_int16,
    161 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    251 as libc::c_int as yytype_int16,
    135 as libc::c_int as yytype_int16,
    251 as libc::c_int as yytype_int16,
    253 as libc::c_int as yytype_int16,
    135 as libc::c_int as yytype_int16,
    135 as libc::c_int as yytype_int16,
    258 as libc::c_int as yytype_int16,
    315 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    147 as libc::c_int as yytype_int16,
    322 as libc::c_int as yytype_int16,
    326 as libc::c_int as yytype_int16,
    126 as libc::c_int as yytype_int16,
    127 as libc::c_int as yytype_int16,
    148 as libc::c_int as yytype_int16,
    164 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    165 as libc::c_int as yytype_int16,
    188 as libc::c_int as yytype_int16,
    188 as libc::c_int as yytype_int16,
    155 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    174 as libc::c_int as yytype_int16,
    162 as libc::c_int as yytype_int16,
    273 as libc::c_int as yytype_int16,
    175 as libc::c_int as yytype_int16,
    178 as libc::c_int as yytype_int16,
    128 as libc::c_int as yytype_int16,
    129 as libc::c_int as yytype_int16,
    179 as libc::c_int as yytype_int16,
    156 as libc::c_int as yytype_int16,
    283 as libc::c_int as yytype_int16,
    282 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    123 as libc::c_int as yytype_int16,
    124 as libc::c_int as yytype_int16,
    251 as libc::c_int as yytype_int16,
    251 as libc::c_int as yytype_int16,
    239 as libc::c_int as yytype_int16,
    240 as libc::c_int as yytype_int16,
    243 as libc::c_int as yytype_int16,
    292 as libc::c_int as yytype_int16,
    291 as libc::c_int as yytype_int16,
    166 as libc::c_int as yytype_int16,
    151 as libc::c_int as yytype_int16,
    224 as libc::c_int as yytype_int16,
    225 as libc::c_int as yytype_int16,
    226 as libc::c_int as yytype_int16,
    151 as libc::c_int as yytype_int16,
    157 as libc::c_int as yytype_int16,
    281 as libc::c_int as yytype_int16,
    176 as libc::c_int as yytype_int16,
    269 as libc::c_int as yytype_int16,
    270 as libc::c_int as yytype_int16,
    271 as libc::c_int as yytype_int16,
    180 as libc::c_int as yytype_int16,
    297 as libc::c_int as yytype_int16,
    298 as libc::c_int as yytype_int16,
    299 as libc::c_int as yytype_int16,
    260 as libc::c_int as yytype_int16,
    261 as libc::c_int as yytype_int16,
    126 as libc::c_int as yytype_int16,
    127 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    196 as libc::c_int as yytype_int16,
    197 as libc::c_int as yytype_int16,
    329 as libc::c_int as yytype_int16,
    225 as libc::c_int as yytype_int16,
    306 as libc::c_int as yytype_int16,
    158 as libc::c_int as yytype_int16,
    278 as libc::c_int as yytype_int16,
    279 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    222 as libc::c_int as yytype_int16,
    223 as libc::c_int as yytype_int16,
    313 as libc::c_int as yytype_int16,
    159 as libc::c_int as yytype_int16,
    285 as libc::c_int as yytype_int16,
    286 as libc::c_int as yytype_int16,
    163 as libc::c_int as yytype_int16,
    167 as libc::c_int as yytype_int16,
    2 as libc::c_int as yytype_int16,
    168 as libc::c_int as yytype_int16,
    151 as libc::c_int as yytype_int16,
    169 as libc::c_int as yytype_int16,
    186 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    320 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    170 as libc::c_int as yytype_int16,
    171 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    172 as libc::c_int as yytype_int16,
    173 as libc::c_int as yytype_int16,
    177 as libc::c_int as yytype_int16,
    181 as libc::c_int as yytype_int16,
    332 as libc::c_int as yytype_int16,
    182 as libc::c_int as yytype_int16,
    183 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    251 as libc::c_int as yytype_int16,
    251 as libc::c_int as yytype_int16,
    184 as libc::c_int as yytype_int16,
    185 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    244 as libc::c_int as yytype_int16,
    245 as libc::c_int as yytype_int16,
    187 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    255 as libc::c_int as yytype_int16,
    316 as libc::c_int as yytype_int16,
    216 as libc::c_int as yytype_int16,
    135 as libc::c_int as yytype_int16,
    256 as libc::c_int as yytype_int16,
    262 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    319 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    268 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    323 as libc::c_int as yytype_int16,
    324 as libc::c_int as yytype_int16,
    280 as libc::c_int as yytype_int16,
    284 as libc::c_int as yytype_int16,
    293 as libc::c_int as yytype_int16,
    289 as libc::c_int as yytype_int16,
    295 as libc::c_int as yytype_int16,
    327 as libc::c_int as yytype_int16,
    328 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    331 as libc::c_int as yytype_int16,
    2 as libc::c_int as yytype_int16,
    304 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    333 as libc::c_int as yytype_int16,
    334 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    275 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    341 as libc::c_int as yytype_int16,
    252 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    305 as libc::c_int as yytype_int16,
    309 as libc::c_int as yytype_int16,
    310 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    311 as libc::c_int as yytype_int16,
    312 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    335 as libc::c_int as yytype_int16,
    318 as libc::c_int as yytype_int16,
    336 as libc::c_int as yytype_int16,
    339 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    342 as libc::c_int as yytype_int16,
    344 as libc::c_int as yytype_int16,
    340 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    343 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    345 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    2 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    330 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    272 as libc::c_int as yytype_int16,
    301 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    2 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    135 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    2 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    263 as libc::c_int as yytype_int16,
    264 as libc::c_int as yytype_int16,
    265 as libc::c_int as yytype_int16,
    266 as libc::c_int as yytype_int16,
    267 as libc::c_int as yytype_int16,
    2 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    290 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    2 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    119 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    135 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    82 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    91 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    94 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    95 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    97 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
];
static mut yycheck: [yytype_int16; 662] = [
    2 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    210 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    212 as libc::c_int as yytype_int16,
    128 as libc::c_int as yytype_int16,
    129 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    62 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    64 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    215 as libc::c_int as yytype_int16,
    60 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    219 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    248 as libc::c_int as yytype_int16,
    249 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    67 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    189 as libc::c_int as yytype_int16,
    190 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    258 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    126 as libc::c_int as yytype_int16,
    127 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    130 as libc::c_int as yytype_int16,
    131 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    151 as libc::c_int as yytype_int16,
    122 as libc::c_int as yytype_int16,
    153 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    140 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    144 as libc::c_int as yytype_int16,
    145 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    60 as libc::c_int as yytype_int16,
    149 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    157 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    151 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    153 as libc::c_int as yytype_int16,
    154 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    325 as libc::c_int as yytype_int16,
    326 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    134 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    128 as libc::c_int as yytype_int16,
    129 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    142 as libc::c_int as yytype_int16,
    143 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    147 as libc::c_int as yytype_int16,
    148 as libc::c_int as yytype_int16,
    196 as libc::c_int as yytype_int16,
    197 as libc::c_int as yytype_int16,
    198 as libc::c_int as yytype_int16,
    199 as libc::c_int as yytype_int16,
    200 as libc::c_int as yytype_int16,
    201 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    204 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    210 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    212 as libc::c_int as yytype_int16,
    211 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    216 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    217 as libc::c_int as yytype_int16,
    218 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    235 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    242 as libc::c_int as yytype_int16,
    241 as libc::c_int as yytype_int16,
    189 as libc::c_int as yytype_int16,
    190 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    248 as libc::c_int as yytype_int16,
    249 as libc::c_int as yytype_int16,
    203 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    205 as libc::c_int as yytype_int16,
    253 as libc::c_int as yytype_int16,
    252 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    215 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    219 as libc::c_int as yytype_int16,
    60 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    224 as libc::c_int as yytype_int16,
    225 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    282 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    237 as libc::c_int as yytype_int16,
    238 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    193 as libc::c_int as yytype_int16,
    194 as libc::c_int as yytype_int16,
    291 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    246 as libc::c_int as yytype_int16,
    247 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    258 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    303 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    317 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    325 as libc::c_int as yytype_int16,
    326 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    60 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    60 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    303 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    307 as libc::c_int as yytype_int16,
    308 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    314 as libc::c_int as yytype_int16,
    315 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    317 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    321 as libc::c_int as yytype_int16,
    322 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    329 as libc::c_int as yytype_int16,
    211 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    316 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    235 as libc::c_int as yytype_int16,
    276 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    50 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    57 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    51 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    3 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    57 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    228 as libc::c_int as yytype_int16,
    229 as libc::c_int as yytype_int16,
    230 as libc::c_int as yytype_int16,
    231 as libc::c_int as yytype_int16,
    232 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    57 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    18 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    251 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    3 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    57 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    30 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    54 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    51 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    59 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    54 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    43 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    54 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    54 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
];
#[no_mangle]
pub static mut EOF_Reached: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub fn return_EOF() -> libc::c_int {
    return -(1 as libc::c_int);
}
static mut yystos: [yytype_int8; 346] = [
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    55 as libc::c_int as yytype_int8,
    57 as libc::c_int as yytype_int8,
    59 as libc::c_int as yytype_int8,
    62 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    65 as libc::c_int as yytype_int8,
    67 as libc::c_int as yytype_int8,
    68 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    70 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    72 as libc::c_int as yytype_int8,
    73 as libc::c_int as yytype_int8,
    74 as libc::c_int as yytype_int8,
    76 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    78 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    80 as libc::c_int as yytype_int8,
    81 as libc::c_int as yytype_int8,
    94 as libc::c_int as yytype_int8,
    95 as libc::c_int as yytype_int8,
    96 as libc::c_int as yytype_int8,
    97 as libc::c_int as yytype_int8,
    98 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    87 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    67 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    92 as libc::c_int as yytype_int8,
    96 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    59 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    55 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    55 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    65 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    91 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    48 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    92 as libc::c_int as yytype_int8,
    96 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    89 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    96 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    57 as libc::c_int as yytype_int8,
    92 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    59 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    75 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    58 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    95 as libc::c_int as yytype_int8,
    95 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    57 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    57 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    95 as libc::c_int as yytype_int8,
    95 as libc::c_int as yytype_int8,
    97 as libc::c_int as yytype_int8,
    97 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    83 as libc::c_int as yytype_int8,
    84 as libc::c_int as yytype_int8,
    85 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    57 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    63 as libc::c_int as yytype_int8,
    92 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    58 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    57 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    57 as libc::c_int as yytype_int8,
    87 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    63 as libc::c_int as yytype_int8,
    92 as libc::c_int as yytype_int8,
    87 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    75 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    75 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    84 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    59 as libc::c_int as yytype_int8,
    86 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    92 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    58 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    87 as libc::c_int as yytype_int8,
    87 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    92 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    58 as libc::c_int as yytype_int8,
    75 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    86 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    58 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    57 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    58 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    58 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    57 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    57 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    57 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    58 as libc::c_int as yytype_int8,
    87 as libc::c_int as yytype_int8,
    87 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    58 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    58 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    58 as libc::c_int as yytype_int8,
];
#[no_mangle]
pub static mut bash_input: BASH_INPUT = BASH_INPUT {
    type_0: st_none,
    name: 0 as *const libc::c_char as *mut libc::c_char,
    location: INPUT_STREAM {
        file: 0 as *const FILE as *mut FILE,
    },
    getter: None,
    ungetter: None,
};
#[no_mangle]
pub fn initialize_bash_input() {
    unsafe {
        bash_input.type_0 = st_none;
        if !(bash_input.name).is_null() {
            free(bash_input.name as *mut libc::c_void);
        }
        bash_input.name = 0 as *mut libc::c_char;
        bash_input.name = 0 as *mut libc::c_void as *mut libc::c_char;
        bash_input.location.file = 0 as *mut libc::c_void as *mut FILE;
        bash_input.location.string = 0 as *mut libc::c_void as *mut libc::c_char;
        bash_input.getter = ::core::mem::transmute::<*mut libc::c_void, Option<sh_cget_func_t>>(
            0 as *mut libc::c_void,
        );
        bash_input.ungetter = ::core::mem::transmute::<*mut libc::c_void, Option<sh_cunget_func_t>>(
            0 as *mut libc::c_void,
        );
    }
}
#[no_mangle]
pub fn init_yy_io(
    mut get: Option<sh_cget_func_t>,
    mut unget: Option<sh_cunget_func_t>,
    mut type_0: stream_type,
    mut name: *const libc::c_char,
    mut location: INPUT_STREAM,
) {
    unsafe {
        bash_input.type_0 = type_0;
        if !(bash_input.name).is_null() {
            free(bash_input.name as *mut libc::c_void);
        }
        bash_input.name = 0 as *mut libc::c_char;
        bash_input.name = if !name.is_null() {
            strcpy(
                malloc(
                    (1 as libc::c_int as libc::c_ulong).wrapping_add(libc::strlen(name) as u64)
                        as usize,
                ) as *mut libc::c_char,
                name,
            )
        } else {
            0 as *mut libc::c_void as *mut libc::c_char
        };

        bash_input.location = location;
        bash_input.getter = get;
        bash_input.ungetter = unget;
    }
}
static mut yyr1: [yytype_int8; 173] = [
    0 as libc::c_int as yytype_int8,
    61 as libc::c_int as yytype_int8,
    62 as libc::c_int as yytype_int8,
    62 as libc::c_int as yytype_int8,
    62 as libc::c_int as yytype_int8,
    62 as libc::c_int as yytype_int8,
    62 as libc::c_int as yytype_int8,
    63 as libc::c_int as yytype_int8,
    63 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    65 as libc::c_int as yytype_int8,
    65 as libc::c_int as yytype_int8,
    65 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
    67 as libc::c_int as yytype_int8,
    67 as libc::c_int as yytype_int8,
    68 as libc::c_int as yytype_int8,
    68 as libc::c_int as yytype_int8,
    68 as libc::c_int as yytype_int8,
    68 as libc::c_int as yytype_int8,
    68 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    70 as libc::c_int as yytype_int8,
    70 as libc::c_int as yytype_int8,
    70 as libc::c_int as yytype_int8,
    70 as libc::c_int as yytype_int8,
    70 as libc::c_int as yytype_int8,
    70 as libc::c_int as yytype_int8,
    70 as libc::c_int as yytype_int8,
    70 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    72 as libc::c_int as yytype_int8,
    72 as libc::c_int as yytype_int8,
    72 as libc::c_int as yytype_int8,
    72 as libc::c_int as yytype_int8,
    72 as libc::c_int as yytype_int8,
    72 as libc::c_int as yytype_int8,
    72 as libc::c_int as yytype_int8,
    72 as libc::c_int as yytype_int8,
    73 as libc::c_int as yytype_int8,
    73 as libc::c_int as yytype_int8,
    73 as libc::c_int as yytype_int8,
    74 as libc::c_int as yytype_int8,
    74 as libc::c_int as yytype_int8,
    74 as libc::c_int as yytype_int8,
    74 as libc::c_int as yytype_int8,
    75 as libc::c_int as yytype_int8,
    75 as libc::c_int as yytype_int8,
    76 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    78 as libc::c_int as yytype_int8,
    78 as libc::c_int as yytype_int8,
    78 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    80 as libc::c_int as yytype_int8,
    81 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    83 as libc::c_int as yytype_int8,
    83 as libc::c_int as yytype_int8,
    84 as libc::c_int as yytype_int8,
    84 as libc::c_int as yytype_int8,
    84 as libc::c_int as yytype_int8,
    84 as libc::c_int as yytype_int8,
    85 as libc::c_int as yytype_int8,
    85 as libc::c_int as yytype_int8,
    85 as libc::c_int as yytype_int8,
    85 as libc::c_int as yytype_int8,
    85 as libc::c_int as yytype_int8,
    85 as libc::c_int as yytype_int8,
    86 as libc::c_int as yytype_int8,
    86 as libc::c_int as yytype_int8,
    87 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    89 as libc::c_int as yytype_int8,
    89 as libc::c_int as yytype_int8,
    89 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    91 as libc::c_int as yytype_int8,
    91 as libc::c_int as yytype_int8,
    92 as libc::c_int as yytype_int8,
    92 as libc::c_int as yytype_int8,
    92 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    94 as libc::c_int as yytype_int8,
    94 as libc::c_int as yytype_int8,
    94 as libc::c_int as yytype_int8,
    95 as libc::c_int as yytype_int8,
    95 as libc::c_int as yytype_int8,
    95 as libc::c_int as yytype_int8,
    95 as libc::c_int as yytype_int8,
    95 as libc::c_int as yytype_int8,
    96 as libc::c_int as yytype_int8,
    96 as libc::c_int as yytype_int8,
    96 as libc::c_int as yytype_int8,
    96 as libc::c_int as yytype_int8,
    96 as libc::c_int as yytype_int8,
    97 as libc::c_int as yytype_int8,
    97 as libc::c_int as yytype_int8,
    97 as libc::c_int as yytype_int8,
    98 as libc::c_int as yytype_int8,
    98 as libc::c_int as yytype_int8,
    98 as libc::c_int as yytype_int8,
    98 as libc::c_int as yytype_int8,
];
#[no_mangle]
pub fn yy_input_name() -> *mut libc::c_char {
    unsafe {
        return (if !(bash_input.name).is_null() {
            bash_input.name as *const libc::c_char
        } else {
            b"stdin\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char;
    }
}
fn yy_getc() -> libc::c_int {
    // 获取终端一个字符
    unsafe {
        return (Some((bash_input.getter).expect("non-null function pointer")))
            .expect("non-null function pointer")();
    }
}
fn yy_ungetc(mut c: libc::c_int) -> libc::c_int {
    unsafe {
        return (Some((bash_input.ungetter).expect("non-null function pointer")))
            .expect("non-null function pointer")(c);
    }
}
static mut yyr2: [yytype_int8; 173] = [
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
];
#[no_mangle]
pub static mut current_readline_prompt: *mut libc::c_char =
    0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char;
#[no_mangle]
pub static mut current_readline_line: *mut libc::c_char =
    0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char;
#[no_mangle]
pub static mut current_readline_line_index: libc::c_int = 0 as libc::c_int;
fn yy_readline_get() -> libc::c_int {
    // 读取命令行数据
    let mut old_sigint: Option<SigHandler> = None;
    let mut line_len: libc::c_int = 0;
    let mut c: libc::c_uchar = 0;
    unsafe {
        if current_readline_line.is_null() {
            if bash_readline_initialized == 0 as libc::c_int {
                initialize_readline();
            }
            if job_control != 0 {
                give_terminal_to(shell_pgrp, 0 as libc::c_int);
            }
            old_sigint = ::core::mem::transmute::<Option<fn() -> ()>, Option<SigHandler>>(Some(
                initialize_traps as fn() -> (),
            ));
            if signal_is_ignored(2 as libc::c_int) == 0 as libc::c_int {
                old_sigint = set_signal_handler(2 as libc::c_int, Some(sigint_sighandler));
            }
            c_sh_unset_nodelay_mode(fileno(rl_instream));
            current_readline_line = c_readline(if !current_readline_prompt.is_null() {
                // 读取终端字符
                current_readline_prompt as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            });
            if terminating_signal != 0 {
                termsig_handler(terminating_signal);
            }
            if signal_is_ignored(2 as libc::c_int) == 0 as libc::c_int {
                if old_sigint
                    != ::core::mem::transmute::<Option<fn() -> ()>, Option<SigHandler>>(Some(
                        initialize_traps as fn() -> (),
                    ))
                {
                    set_signal_handler(2 as libc::c_int, old_sigint);
                }
            }
            if current_readline_line.is_null() {
                return -(1 as libc::c_int);
            }
            current_readline_line_index = 0 as libc::c_int;
            line_len = libc::strlen(current_readline_line) as libc::c_int;
            current_readline_line = realloc(
                current_readline_line as *mut libc::c_void,
                (2 as libc::c_int + line_len) as usize,
            ) as *mut libc::c_char;
            let fresh0 = line_len;
            line_len = line_len + 1;
            *current_readline_line.offset(fresh0 as isize) = '\n' as i32 as libc::c_char;
            *current_readline_line.offset(line_len as isize) = '\0' as i32 as libc::c_char;
        }
        if *current_readline_line.offset(current_readline_line_index as isize) as libc::c_int
            == 0 as libc::c_int
        {
            free(current_readline_line as *mut libc::c_void);
            current_readline_line = 0 as *mut libc::c_void as *mut libc::c_char;
            return yy_readline_get();
        } else {
            let fresh1 = current_readline_line_index;
            current_readline_line_index = current_readline_line_index + 1;
            c = *current_readline_line.offset(fresh1 as isize) as libc::c_uchar;
            return c as libc::c_int;
        };
    }
}
fn yy_readline_unget(mut c: libc::c_int) -> libc::c_int {
    unsafe {
        if current_readline_line_index != 0 && !current_readline_line.is_null() {
            current_readline_line_index -= 1;
            *current_readline_line.offset(current_readline_line_index as isize) = c as libc::c_char;
        }

        return c;
    }
}
#[no_mangle]
pub fn with_input_from_stdin() {
    unsafe {
        let mut location: INPUT_STREAM = INPUT_STREAM {
            file: 0 as *const FILE as *mut FILE,
        };
        if bash_input.type_0 as libc::c_uint != st_stdin as libc::c_int as libc::c_uint
            && stream_on_stack(st_stdin) == 0 as libc::c_int
        {
            location.string = current_readline_line;
            init_yy_io(
                Some(yy_readline_get as fn() -> libc::c_int),
                ::core::mem::transmute::<Option<fn() -> libc::c_int>, Option<sh_cunget_func_t>>(
                    Some(::core::mem::transmute::<
                        fn(libc::c_int) -> libc::c_int,
                        fn() -> libc::c_int,
                    >(yy_readline_unget)),
                ),
                st_stdin as libc::c_int as libc::c_uint,
                b"readline stdin\0" as *const u8 as *const libc::c_char,
                location,
            );
        }
    }
}
#[no_mangle]
pub fn parser_will_prompt() -> libc::c_int {
    unsafe {
        return (current_readline_line.is_null()
            || *current_readline_line.offset(current_readline_line_index as isize) as libc::c_int
                == 0 as libc::c_int) as libc::c_int;
    }
}
fn yy_string_get() -> libc::c_int {
    unsafe {
        let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut c: libc::c_uchar = 0;
        string = bash_input.location.string;
        if !string.is_null() && *string as libc::c_int != 0 {
            let fresh2 = string;

            string = string.offset(1);

            c = *fresh2 as libc::c_uchar;
            bash_input.location.string = string;
            return c as libc::c_int;
        } else {
            return -(1 as libc::c_int);
        };
    }
}
fn yy_string_unget(mut c: libc::c_int) -> libc::c_int {
    unsafe {
        bash_input.location.string = (bash_input.location.string).offset(-1);
        *bash_input.location.string = c as libc::c_char;

        return c;
    }
}
#[no_mangle]
pub fn with_input_from_string(mut string: *mut libc::c_char, mut name: *const libc::c_char) {
    unsafe {
        let mut location: INPUT_STREAM = INPUT_STREAM {
            file: 0 as *const FILE as *mut FILE,
        };
        location.string = string;
        init_yy_io(
            Some(yy_string_get as fn() -> libc::c_int),
            ::core::mem::transmute::<Option<fn() -> libc::c_int>, Option<sh_cunget_func_t>>(Some(
                ::core::mem::transmute::<fn(libc::c_int) -> libc::c_int, fn() -> libc::c_int>(
                    yy_string_unget,
                ),
            )),
            st_string as libc::c_int as libc::c_uint,
            name,
            location,
        );
    }
}
fn rewind_input_string() {
    unsafe {
        let mut xchars: libc::c_int = 0;
        xchars = shell_input_line_len.wrapping_sub(shell_input_line_index) as libc::c_int;
        if *(bash_input.location.string).offset(-(1 as libc::c_int) as isize) as libc::c_int
            == '\n' as i32
        {
            xchars += 1;
        }
        bash_input.location.string = (bash_input.location.string).offset(-(xchars as isize));
    }
}
fn yydestruct(
    mut yymsg: *const libc::c_char,
    mut _yykind: yysymbol_kind_t,
    mut _yyvaluep: *mut YYSTYPE,
) {
    if yymsg.is_null() {
        yymsg = b"Deleting\0" as *const u8 as *const libc::c_char;
    }
}
fn yy_stream_get() -> libc::c_int {
    unsafe {
        let mut result: libc::c_int = 0;
        result = -(1 as libc::c_int);
        if !(bash_input.location.file).is_null() {
            result = getc_with_restart(bash_input.location.file);
        }
        return result;
    }
}
#[no_mangle]
pub static mut yychar: libc::c_int = 0;
#[no_mangle]
pub static mut yylval: YYSTYPE = YYSTYPE {
    word: 0 as *const WORD_DESC as *mut WORD_DESC,
};
#[no_mangle]
pub static mut yynerrs: libc::c_int = 0;
fn yy_stream_unget(mut c: libc::c_int) -> libc::c_int {
    unsafe {
        return ungetc_with_restart(c, bash_input.location.file);
    }
}

#[no_mangle]
pub fn yyparse() -> libc::c_int {
    unsafe {
        let mut current_block: u64;
        let mut yystate: yy_state_fast_t = 0 as libc::c_int;
        let mut yyerrstatus: libc::c_int = 0 as libc::c_int;
        let mut yystacksize: libc::c_long = 200 as libc::c_int as libc::c_long;
        let mut yyssa: [yy_state_t; 200] = [0; 200];
        let mut yyss: *mut yy_state_t = yyssa.as_mut_ptr();
        let mut yyssp: *mut yy_state_t = yyss;
        let mut yyvsa: [YYSTYPE; 200] = [YYSTYPE {
            word: 0 as *const WORD_DESC as *mut WORD_DESC,
        }; 200];
        let mut yyvs: *mut YYSTYPE = yyvsa.as_mut_ptr();
        let mut yyvsp: *mut YYSTYPE = yyvs;
        let mut yyn: libc::c_int = 0;
        let mut yyresult: libc::c_int = 0;
        let mut yytoken: yysymbol_kind_t = YYSYMBOL_YYEMPTY;
        let mut yyval: YYSTYPE = YYSTYPE {
            word: 0 as *const WORD_DESC as *mut WORD_DESC,
        };
        let mut yylen: libc::c_int = 0 as libc::c_int;
        yychar = -(2 as libc::c_int);
        's_46: loop {
            (0 as libc::c_int != 0 && (0 as libc::c_int <= yystate && yystate < 346 as libc::c_int))
                as libc::c_int;
            *yyssp = yystate as yy_state_t;
            if yyss
                .offset(yystacksize as isize)
                .offset(-(1 as libc::c_int as isize))
                <= yyssp
            {
                let mut yysize: libc::c_long =
                    yyssp.offset_from(yyss) as libc::c_long + 1 as libc::c_int as libc::c_long;
                if 10000 as libc::c_int as libc::c_long <= yystacksize {
                    current_block = 5105078013716444332;
                    break;
                }
                yystacksize *= 2 as libc::c_int as libc::c_long;
                if (10000 as libc::c_int as libc::c_long) < yystacksize {
                    yystacksize = 10000 as libc::c_int as libc::c_long;
                }
                let mut yyss1: *mut yy_state_t = yyss;
                let mut yyptr: *mut yyalloc = malloc(
                    (yystacksize
                        * (::core::mem::size_of::<yy_state_t>() as libc::c_ulong as libc::c_long
                            + ::core::mem::size_of::<YYSTYPE>() as libc::c_ulong as libc::c_long)
                        + (::core::mem::size_of::<yyalloc>() as libc::c_ulong as libc::c_long
                            - 1 as libc::c_int as libc::c_long)) as usize,
                ) as *mut yyalloc;
                if yyptr.is_null() {
                    current_block = 5105078013716444332;
                    break;
                } else {
                    let mut yynewbytes: libc::c_long = 0;
                    libc::memcpy(
                        &mut (*yyptr).yyss_alloc as *mut yy_state_t as *mut libc::c_void,
                        yyss as *const libc::c_void,
                        (yysize as libc::c_ulong)
                            .wrapping_mul(::core::mem::size_of::<yy_state_t>() as libc::c_ulong)
                            as libc::size_t,
                    );
                    yyss = &mut (*yyptr).yyss_alloc;
                    yynewbytes = yystacksize
                        * ::core::mem::size_of::<yy_state_t>() as libc::c_ulong as libc::c_long
                        + (::core::mem::size_of::<yyalloc>() as libc::c_ulong as libc::c_long
                            - 1 as libc::c_int as libc::c_long);
                    yyptr = yyptr.offset(
                        (yynewbytes
                            / ::core::mem::size_of::<yyalloc>() as libc::c_ulong as libc::c_long)
                            as isize,
                    );
                    let mut yynewbytes_0: libc::c_long = 0;
                    libc::memcpy(
                        &mut (*yyptr).yyvs_alloc as *mut YYSTYPE as *mut libc::c_void,
                        yyvs as *const libc::c_void,
                        (yysize as libc::c_ulong)
                            .wrapping_mul(::core::mem::size_of::<YYSTYPE>() as libc::c_ulong)
                            as libc::size_t,
                    );
                    yyvs = &mut (*yyptr).yyvs_alloc;
                    yynewbytes_0 = yystacksize
                        * ::core::mem::size_of::<YYSTYPE>() as libc::c_ulong as libc::c_long
                        + (::core::mem::size_of::<yyalloc>() as libc::c_ulong as libc::c_long
                            - 1 as libc::c_int as libc::c_long);
                    yyptr = yyptr.offset(
                        (yynewbytes_0
                            / ::core::mem::size_of::<yyalloc>() as libc::c_ulong as libc::c_long)
                            as isize,
                    );
                    if yyss1 != yyssa.as_mut_ptr() {
                        free(yyss1 as *mut libc::c_void);
                    }
                    yyssp = yyss
                        .offset(yysize as isize)
                        .offset(-(1 as libc::c_int as isize));
                    yyvsp = yyvs
                        .offset(yysize as isize)
                        .offset(-(1 as libc::c_int as isize));
                    if yyss
                        .offset(yystacksize as isize)
                        .offset(-(1 as libc::c_int as isize))
                        <= yyssp
                    {
                        current_block = 12092114409115356139;
                        break;
                    }
                }
            }
            if yystate == 118 as libc::c_int {
                current_block = 4635604590554438705;
                break;
            }
            yyn = yypact[yystate as usize] as libc::c_int;
            if yyn == -(204 as libc::c_int) {
                current_block = 13954386586367199081;
            } else {
                if yychar == -(2 as libc::c_int) {
                    yychar = yylex();
                }
                if yychar <= 0 as libc::c_int {
                    yychar = 0 as libc::c_int;
                    yytoken = YYSYMBOL_YYEOF;
                    current_block = 6174974146017752131;
                } else if yychar == 256 as libc::c_int {
                    yychar = 257 as libc::c_int;
                    yytoken = YYSYMBOL_YYerror;
                    current_block = 5467060717235750116;
                } else {
                    yytoken = (if 0 as libc::c_int <= yychar && yychar <= 304 as libc::c_int {
                        yytranslate[yychar as usize] as yysymbol_kind_t as libc::c_int
                    } else {
                        YYSYMBOL_YYUNDEF as libc::c_int
                    }) as yysymbol_kind_t;
                    current_block = 6174974146017752131;
                }
                match current_block {
                    5467060717235750116 => {}
                    _ => {
                        yyn += yytoken as libc::c_int;
                        if yyn < 0 as libc::c_int
                            || (661 as libc::c_int) < yyn
                            || yycheck[yyn as usize] as libc::c_int != yytoken as libc::c_int
                        {
                            current_block = 13954386586367199081;
                        } else {
                            yyn = yytable[yyn as usize] as libc::c_int;
                            if yyn <= 0 as libc::c_int {
                                yyn = -yyn;
                                current_block = 15246589712108891992;
                            } else {
                                if yyerrstatus != 0 {
                                    yyerrstatus -= 1;
                                }
                                yystate = yyn;
                                yyvsp = yyvsp.offset(1);
                                *yyvsp = yylval;
                                yychar = -(2 as libc::c_int);
                                current_block = 7425917188841100089;
                            }
                        }
                    }
                }
            }
            match current_block {
                13954386586367199081 => {
                    yyn = yydefact[yystate as usize] as libc::c_int;
                    if yyn == 0 as libc::c_int {
                        yytoken = (if yychar == -(2 as libc::c_int) {
                            YYSYMBOL_YYEMPTY as libc::c_int
                        } else if 0 as libc::c_int <= yychar && yychar <= 304 as libc::c_int {
                            yytranslate[yychar as usize] as yysymbol_kind_t as libc::c_int
                        } else {
                            YYSYMBOL_YYUNDEF as libc::c_int
                        }) as yysymbol_kind_t;
                        if yyerrstatus == 0 {
                            yynerrs += 1;
                            yyerror(b"syntax error\0" as *const u8 as *const libc::c_char);
                        }
                        if yyerrstatus == 3 as libc::c_int {
                            if yychar <= 0 as libc::c_int {
                                if yychar == 0 as libc::c_int {
                                    current_block = 12092114409115356139;
                                    break;
                                }
                            } else {
                                yydestruct(
                                    b"Error: discarding\0" as *const u8 as *const libc::c_char,
                                    yytoken,
                                    &mut yylval,
                                );
                                yychar = -(2 as libc::c_int);
                            }
                        }
                        current_block = 5467060717235750116;
                    } else {
                        current_block = 15246589712108891992;
                    }
                }
                _ => {}
            }
            match current_block {
                15246589712108891992 => {
                    yylen = yyr2[yyn as usize] as libc::c_int;
                    yyval = *yyvsp.offset((1 as libc::c_int - yylen) as isize);
                    match yyn {
                        2 => {
                            global_command = (*yyvsp.offset(-(1 as libc::c_int) as isize)).command;
                            eof_encountered = 0 as libc::c_int;
                            if parser_state & 0x40 as libc::c_int != 0 {
                                parser_state |= 0x8000 as libc::c_int;
                            }
                            current_block = 4635604590554438705;
                            break;
                        }
                        3 => {
                            global_command = 0 as *mut libc::c_void as *mut COMMAND;
                            if parser_state & 0x40 as libc::c_int != 0 {
                                parser_state |= 0x8000 as libc::c_int;
                            }
                            current_block = 4635604590554438705;
                            break;
                        }
                        4 => {
                            global_command = 0 as *mut libc::c_void as *mut COMMAND;
                            eof_encountered = 0 as libc::c_int;
                            if interactive != 0 && parse_and_execute_level == 0 as libc::c_int {
                                current_block = 4635604590554438705;
                                break;
                            } else {
                                current_block = 12092114409115356139;
                                break;
                            }
                        }
                        5 => {
                            global_command = 0 as *mut libc::c_void as *mut COMMAND;
                            if last_command_exit_value == 0 as libc::c_int {
                                ::core::ptr::write_volatile(
                                    &mut last_command_exit_value as *mut libc::c_int,
                                    2 as libc::c_int,
                                );
                            }
                            if interactive != 0 && parse_and_execute_level == 0 as libc::c_int {
                                current_block = 5793491756164225964;
                                break;
                            } else {
                                current_block = 12092114409115356139;
                                break;
                            }
                        }
                        6 => {
                            global_command = 0 as *mut libc::c_void as *mut COMMAND;
                            handle_eof_input_unit();
                            current_block = 4635604590554438705;
                            break;
                        }
                        7 => {
                            yyval.WordList = make_word_list(
                                (*yyvsp.offset(0 as libc::c_int as isize)).word,
                                0 as *mut libc::c_void as *mut WORD_LIST,
                            );
                            current_block = 12964251834421583797;
                        }
                        8 => {
                            yyval.WordList = make_word_list(
                                (*yyvsp.offset(0 as libc::c_int as isize)).word,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).WordList,
                            );
                            current_block = 12964251834421583797;
                        }
                        9 => {
                            source.dest = 1 as libc::c_int;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect = make_redirection(
                                source,
                                r_output_direction,
                                redir,
                                0 as libc::c_int,
                            );
                            current_block = 12964251834421583797;
                        }
                        10 => {
                            source.dest = 0 as libc::c_int;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect = make_redirection(
                                source,
                                r_input_direction,
                                redir,
                                0 as libc::c_int,
                            );
                            current_block = 12964251834421583797;
                        }
                        11 => {
                            source.dest = (*yyvsp.offset(-(2 as libc::c_int) as isize)).number;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect = make_redirection(
                                source,
                                r_output_direction,
                                redir,
                                0 as libc::c_int,
                            );
                            current_block = 12964251834421583797;
                        }
                        12 => {
                            source.dest = (*yyvsp.offset(-(2 as libc::c_int) as isize)).number;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect = make_redirection(
                                source,
                                r_input_direction,
                                redir,
                                0 as libc::c_int,
                            );
                            current_block = 12964251834421583797;
                        }
                        13 => {
                            source.filename = (*yyvsp.offset(-(2 as libc::c_int) as isize)).word;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect = make_redirection(
                                source,
                                r_output_direction,
                                redir,
                                0x1 as libc::c_int,
                            );
                            current_block = 12964251834421583797;
                        }
                        14 => {
                            source.filename = (*yyvsp.offset(-(2 as libc::c_int) as isize)).word;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect = make_redirection(
                                source,
                                r_input_direction,
                                redir,
                                0x1 as libc::c_int,
                            );
                            current_block = 12964251834421583797;
                        }
                        15 => {
                            source.dest = 1 as libc::c_int;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect =
                                make_redirection(source, r_appending_to, redir, 0 as libc::c_int);
                            current_block = 12964251834421583797;
                        }
                        16 => {
                            source.dest = (*yyvsp.offset(-(2 as libc::c_int) as isize)).number;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect =
                                make_redirection(source, r_appending_to, redir, 0 as libc::c_int);
                            current_block = 12964251834421583797;
                        }
                        17 => {
                            source.filename = (*yyvsp.offset(-(2 as libc::c_int) as isize)).word;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect =
                                make_redirection(source, r_appending_to, redir, 0x1 as libc::c_int);
                            current_block = 12964251834421583797;
                        }
                        18 => {
                            source.dest = 1 as libc::c_int;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect =
                                make_redirection(source, r_output_force, redir, 0 as libc::c_int);
                            current_block = 12964251834421583797;
                        }
                        19 => {
                            source.dest = (*yyvsp.offset(-(2 as libc::c_int) as isize)).number;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect =
                                make_redirection(source, r_output_force, redir, 0 as libc::c_int);
                            current_block = 12964251834421583797;
                        }
                        20 => {
                            source.filename = (*yyvsp.offset(-(2 as libc::c_int) as isize)).word;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect =
                                make_redirection(source, r_output_force, redir, 0x1 as libc::c_int);
                            current_block = 12964251834421583797;
                        }
                        21 => {
                            source.dest = 0 as libc::c_int;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect =
                                make_redirection(source, r_input_output, redir, 0 as libc::c_int);
                            current_block = 12964251834421583797;
                        }
                        22 => {
                            source.dest = (*yyvsp.offset(-(2 as libc::c_int) as isize)).number;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect =
                                make_redirection(source, r_input_output, redir, 0 as libc::c_int);
                            current_block = 12964251834421583797;
                        }
                        23 => {
                            source.filename = (*yyvsp.offset(-(2 as libc::c_int) as isize)).word;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect =
                                make_redirection(source, r_input_output, redir, 0x1 as libc::c_int);
                            current_block = 12964251834421583797;
                        }
                        24 => {
                            source.dest = 0 as libc::c_int;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect =
                                make_redirection(source, r_reading_until, redir, 0 as libc::c_int);
                            push_heredoc(yyval.redirect);
                            current_block = 12964251834421583797;
                        }
                        25 => {
                            source.dest = (*yyvsp.offset(-(2 as libc::c_int) as isize)).number;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect =
                                make_redirection(source, r_reading_until, redir, 0 as libc::c_int);
                            push_heredoc(yyval.redirect);
                            current_block = 12964251834421583797;
                        }
                        26 => {
                            source.filename = (*yyvsp.offset(-(2 as libc::c_int) as isize)).word;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect = make_redirection(
                                source,
                                r_reading_until,
                                redir,
                                0x1 as libc::c_int,
                            );
                            push_heredoc(yyval.redirect);
                            current_block = 12964251834421583797;
                        }
                        27 => {
                            source.dest = 0 as libc::c_int;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect = make_redirection(
                                source,
                                r_deblank_reading_until,
                                redir,
                                0 as libc::c_int,
                            );
                            push_heredoc(yyval.redirect);
                            current_block = 12964251834421583797;
                        }
                        28 => {
                            source.dest = (*yyvsp.offset(-(2 as libc::c_int) as isize)).number;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect = make_redirection(
                                source,
                                r_deblank_reading_until,
                                redir,
                                0 as libc::c_int,
                            );
                            push_heredoc(yyval.redirect);
                            current_block = 12964251834421583797;
                        }
                        29 => {
                            source.filename = (*yyvsp.offset(-(2 as libc::c_int) as isize)).word;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect = make_redirection(
                                source,
                                r_deblank_reading_until,
                                redir,
                                0x1 as libc::c_int,
                            );
                            push_heredoc(yyval.redirect);
                            current_block = 12964251834421583797;
                        }
                        30 => {
                            source.dest = 0 as libc::c_int;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect =
                                make_redirection(source, r_reading_string, redir, 0 as libc::c_int);
                            current_block = 12964251834421583797;
                        }
                        31 => {
                            source.dest = (*yyvsp.offset(-(2 as libc::c_int) as isize)).number;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect =
                                make_redirection(source, r_reading_string, redir, 0 as libc::c_int);
                            current_block = 12964251834421583797;
                        }
                        32 => {
                            source.filename = (*yyvsp.offset(-(2 as libc::c_int) as isize)).word;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect = make_redirection(
                                source,
                                r_reading_string,
                                redir,
                                0x1 as libc::c_int,
                            );
                            current_block = 12964251834421583797;
                        }
                        33 => {
                            source.dest = 0 as libc::c_int;
                            redir.dest = (*yyvsp.offset(0 as libc::c_int as isize)).number;
                            yyval.redirect = make_redirection(
                                source,
                                r_duplicating_input,
                                redir,
                                0 as libc::c_int,
                            );
                            current_block = 12964251834421583797;
                        }
                        34 => {
                            source.dest = (*yyvsp.offset(-(2 as libc::c_int) as isize)).number;
                            redir.dest = (*yyvsp.offset(0 as libc::c_int as isize)).number;
                            yyval.redirect = make_redirection(
                                source,
                                r_duplicating_input,
                                redir,
                                0 as libc::c_int,
                            );
                            current_block = 12964251834421583797;
                        }
                        35 => {
                            source.filename = (*yyvsp.offset(-(2 as libc::c_int) as isize)).word;
                            redir.dest = (*yyvsp.offset(0 as libc::c_int as isize)).number;
                            yyval.redirect = make_redirection(
                                source,
                                r_duplicating_input,
                                redir,
                                0x1 as libc::c_int,
                            );
                            current_block = 12964251834421583797;
                        }
                        36 => {
                            source.dest = 1 as libc::c_int;
                            redir.dest = (*yyvsp.offset(0 as libc::c_int as isize)).number;
                            yyval.redirect = make_redirection(
                                source,
                                r_duplicating_output,
                                redir,
                                0 as libc::c_int,
                            );
                            current_block = 12964251834421583797;
                        }
                        37 => {
                            source.dest = (*yyvsp.offset(-(2 as libc::c_int) as isize)).number;
                            redir.dest = (*yyvsp.offset(0 as libc::c_int as isize)).number;
                            yyval.redirect = make_redirection(
                                source,
                                r_duplicating_output,
                                redir,
                                0 as libc::c_int,
                            );
                            current_block = 12964251834421583797;
                        }
                        38 => {
                            source.filename = (*yyvsp.offset(-(2 as libc::c_int) as isize)).word;
                            redir.dest = (*yyvsp.offset(0 as libc::c_int as isize)).number;
                            yyval.redirect = make_redirection(
                                source,
                                r_duplicating_output,
                                redir,
                                0x1 as libc::c_int,
                            );
                            current_block = 12964251834421583797;
                        }
                        39 => {
                            source.dest = 0 as libc::c_int;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect = make_redirection(
                                source,
                                r_duplicating_input_word,
                                redir,
                                0 as libc::c_int,
                            );
                            current_block = 12964251834421583797;
                        }
                        40 => {
                            source.dest = (*yyvsp.offset(-(2 as libc::c_int) as isize)).number;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect = make_redirection(
                                source,
                                r_duplicating_input_word,
                                redir,
                                0 as libc::c_int,
                            );
                            current_block = 12964251834421583797;
                        }
                        41 => {
                            source.filename = (*yyvsp.offset(-(2 as libc::c_int) as isize)).word;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect = make_redirection(
                                source,
                                r_duplicating_input_word,
                                redir,
                                0x1 as libc::c_int,
                            );
                            current_block = 12964251834421583797;
                        }
                        42 => {
                            source.dest = 1 as libc::c_int;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect = make_redirection(
                                source,
                                r_duplicating_output_word,
                                redir,
                                0 as libc::c_int,
                            );
                            current_block = 12964251834421583797;
                        }
                        43 => {
                            source.dest = (*yyvsp.offset(-(2 as libc::c_int) as isize)).number;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect = make_redirection(
                                source,
                                r_duplicating_output_word,
                                redir,
                                0 as libc::c_int,
                            );
                            current_block = 12964251834421583797;
                        }
                        44 => {
                            source.filename = (*yyvsp.offset(-(2 as libc::c_int) as isize)).word;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect = make_redirection(
                                source,
                                r_duplicating_output_word,
                                redir,
                                0x1 as libc::c_int,
                            );
                            current_block = 12964251834421583797;
                        }
                        45 => {
                            source.dest = 1 as libc::c_int;
                            redir.dest = 0 as libc::c_int;
                            yyval.redirect =
                                make_redirection(source, r_close_this, redir, 0 as libc::c_int);
                            current_block = 12964251834421583797;
                        }
                        46 => {
                            source.dest = (*yyvsp.offset(-(2 as libc::c_int) as isize)).number;
                            redir.dest = 0 as libc::c_int;
                            yyval.redirect =
                                make_redirection(source, r_close_this, redir, 0 as libc::c_int);
                            current_block = 12964251834421583797;
                        }
                        47 => {
                            source.filename = (*yyvsp.offset(-(2 as libc::c_int) as isize)).word;
                            redir.dest = 0 as libc::c_int;
                            yyval.redirect =
                                make_redirection(source, r_close_this, redir, 0x1 as libc::c_int);
                            current_block = 12964251834421583797;
                        }
                        48 => {
                            source.dest = 0 as libc::c_int;
                            redir.dest = 0 as libc::c_int;
                            yyval.redirect =
                                make_redirection(source, r_close_this, redir, 0 as libc::c_int);
                            current_block = 12964251834421583797;
                        }
                        49 => {
                            source.dest = (*yyvsp.offset(-(2 as libc::c_int) as isize)).number;
                            redir.dest = 0 as libc::c_int;
                            yyval.redirect =
                                make_redirection(source, r_close_this, redir, 0 as libc::c_int);
                            current_block = 12964251834421583797;
                        }
                        50 => {
                            source.filename = (*yyvsp.offset(-(2 as libc::c_int) as isize)).word;
                            redir.dest = 0 as libc::c_int;
                            yyval.redirect =
                                make_redirection(source, r_close_this, redir, 0x1 as libc::c_int);
                            current_block = 12964251834421583797;
                        }
                        51 => {
                            source.dest = 1 as libc::c_int;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect =
                                make_redirection(source, r_err_and_out, redir, 0 as libc::c_int);
                            current_block = 12964251834421583797;
                        }
                        52 => {
                            source.dest = 1 as libc::c_int;
                            redir.filename = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.redirect = make_redirection(
                                source,
                                r_append_err_and_out,
                                redir,
                                0 as libc::c_int,
                            );
                            current_block = 12964251834421583797;
                        }
                        53 => {
                            yyval.element.word = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.element.redirect = 0 as *mut REDIRECT;
                            current_block = 12964251834421583797;
                        }
                        54 => {
                            yyval.element.word = (*yyvsp.offset(0 as libc::c_int as isize)).word;
                            yyval.element.redirect = 0 as *mut REDIRECT;
                            current_block = 12964251834421583797;
                        }
                        55 => {
                            yyval.element.redirect =
                                (*yyvsp.offset(0 as libc::c_int as isize)).redirect;
                            yyval.element.word = 0 as *mut WORD_DESC;
                            current_block = 12964251834421583797;
                        }
                        56 => {
                            yyval.redirect = (*yyvsp.offset(0 as libc::c_int as isize)).redirect;
                            current_block = 12964251834421583797;
                        }
                        57 => {
                            let mut t: *mut REDIRECT = 0 as *mut REDIRECT;
                            t = (*yyvsp.offset(-(1 as libc::c_int) as isize)).redirect;
                            while !((*t).next).is_null() {
                                t = (*t).next;
                            }
                            (*t).next = (*yyvsp.offset(0 as libc::c_int as isize)).redirect;
                            yyval.redirect = (*yyvsp.offset(-(1 as libc::c_int) as isize)).redirect;
                            current_block = 12964251834421583797;
                        }
                        58 => {
                            yyval.command = make_simple_command(
                                (*yyvsp.offset(0 as libc::c_int as isize)).element,
                                0 as *mut libc::c_void as *mut COMMAND,
                            );
                            current_block = 12964251834421583797;
                        }
                        59 => {
                            yyval.command = make_simple_command(
                                (*yyvsp.offset(0 as libc::c_int as isize)).element,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                            );
                            current_block = 12964251834421583797;
                        }
                        60 => {
                            yyval.command = clean_simple_command(
                                (*yyvsp.offset(0 as libc::c_int as isize)).command,
                            );
                            current_block = 12964251834421583797;
                        }
                        61 => {
                            yyval.command = (*yyvsp.offset(0 as libc::c_int as isize)).command;
                            current_block = 12964251834421583797;
                        }
                        62 => {
                            let mut tc: *mut COMMAND = 0 as *mut COMMAND;
                            tc = (*yyvsp.offset(-(1 as libc::c_int) as isize)).command;
                            if !tc.is_null() && !((*tc).redirects).is_null() {
                                let mut t_0: *mut REDIRECT = 0 as *mut REDIRECT;
                                t_0 = (*tc).redirects;
                                while !((*t_0).next).is_null() {
                                    t_0 = (*t_0).next;
                                }
                                (*t_0).next = (*yyvsp.offset(0 as libc::c_int as isize)).redirect;
                            } else if !tc.is_null() {
                                (*tc).redirects =
                                    (*yyvsp.offset(0 as libc::c_int as isize)).redirect;
                            }
                            yyval.command = (*yyvsp.offset(-(1 as libc::c_int) as isize)).command;
                            current_block = 12964251834421583797;
                        }
                        63 => {
                            yyval.command = (*yyvsp.offset(0 as libc::c_int as isize)).command;
                            current_block = 12964251834421583797;
                        }
                        64 => {
                            yyval.command = (*yyvsp.offset(0 as libc::c_int as isize)).command;
                            current_block = 12964251834421583797;
                        }
                        65 => {
                            yyval.command = (*yyvsp.offset(0 as libc::c_int as isize)).command;
                            current_block = 12964251834421583797;
                        }
                        66 => {
                            yyval.command = (*yyvsp.offset(0 as libc::c_int as isize)).command;
                            current_block = 12964251834421583797;
                        }
                        67 => {
                            yyval.command = make_while_command(
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).command,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                            );
                            current_block = 12964251834421583797;
                        }
                        68 => {
                            yyval.command = make_until_command(
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).command,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                            );
                            current_block = 12964251834421583797;
                        }
                        69 => {
                            yyval.command = (*yyvsp.offset(0 as libc::c_int as isize)).command;
                            current_block = 12964251834421583797;
                        }
                        70 => {
                            yyval.command = (*yyvsp.offset(0 as libc::c_int as isize)).command;
                            current_block = 12964251834421583797;
                        }
                        71 => {
                            yyval.command = (*yyvsp.offset(0 as libc::c_int as isize)).command;
                            current_block = 12964251834421583797;
                        }
                        72 => {
                            yyval.command = (*yyvsp.offset(0 as libc::c_int as isize)).command;
                            current_block = 12964251834421583797;
                        }
                        73 => {
                            yyval.command = (*yyvsp.offset(0 as libc::c_int as isize)).command;
                            current_block = 12964251834421583797;
                        }
                        74 => {
                            yyval.command = (*yyvsp.offset(0 as libc::c_int as isize)).command;
                            current_block = 12964251834421583797;
                        }
                        75 => {
                            yyval.command = (*yyvsp.offset(0 as libc::c_int as isize)).command;
                            current_block = 12964251834421583797;
                        }
                        76 => {
                            yyval.command = make_for_command(
                                (*yyvsp.offset(-(4 as libc::c_int) as isize)).word,
                                make_word_list(
                                    make_word(b"\"$@\"\0" as *const u8 as *const libc::c_char),
                                    0 as *mut libc::c_void as *mut WORD_LIST,
                                ),
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                                word_lineno[word_top as usize],
                            );
                            if word_top > 0 as libc::c_int {
                                word_top -= 1;
                            }
                            current_block = 12964251834421583797;
                        }
                        77 => {
                            yyval.command = make_for_command(
                                (*yyvsp.offset(-(4 as libc::c_int) as isize)).word,
                                make_word_list(
                                    make_word(b"\"$@\"\0" as *const u8 as *const libc::c_char),
                                    0 as *mut libc::c_void as *mut WORD_LIST,
                                ),
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                                word_lineno[word_top as usize],
                            );
                            if word_top > 0 as libc::c_int {
                                word_top -= 1;
                            }
                            current_block = 12964251834421583797;
                        }
                        78 => {
                            yyval.command = make_for_command(
                                (*yyvsp.offset(-(5 as libc::c_int) as isize)).word,
                                make_word_list(
                                    make_word(b"\"$@\"\0" as *const u8 as *const libc::c_char),
                                    0 as *mut libc::c_void as *mut WORD_LIST,
                                ),
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                                word_lineno[word_top as usize],
                            );
                            if word_top > 0 as libc::c_int {
                                word_top -= 1;
                            }
                            current_block = 12964251834421583797;
                        }
                        79 => {
                            yyval.command = make_for_command(
                                (*yyvsp.offset(-(5 as libc::c_int) as isize)).word,
                                make_word_list(
                                    make_word(b"\"$@\"\0" as *const u8 as *const libc::c_char),
                                    0 as *mut libc::c_void as *mut WORD_LIST,
                                ),
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                                word_lineno[word_top as usize],
                            );
                            if word_top > 0 as libc::c_int {
                                word_top -= 1;
                            }
                            current_block = 12964251834421583797;
                        }
                        80 => {
                            yyval.command = make_for_command(
                                (*yyvsp.offset(-(8 as libc::c_int) as isize)).word,
                                if !((*yyvsp.offset(-(5 as libc::c_int) as isize)).WordList)
                                    .is_null()
                                    && !((*(*yyvsp.offset(-(5 as libc::c_int) as isize)).WordList)
                                        .next)
                                        .is_null()
                                {
                                    list_reverse(
                                        (*yyvsp.offset(-(5 as libc::c_int) as isize)).WordList
                                            as *mut GENERIC_LIST,
                                    ) as *mut WORD_LIST
                                } else {
                                    (*yyvsp.offset(-(5 as libc::c_int) as isize)).WordList
                                },
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                                word_lineno[word_top as usize],
                            );
                            if word_top > 0 as libc::c_int {
                                word_top -= 1;
                            }
                            current_block = 12964251834421583797;
                        }
                        81 => {
                            yyval.command = make_for_command(
                                (*yyvsp.offset(-(8 as libc::c_int) as isize)).word,
                                if !((*yyvsp.offset(-(5 as libc::c_int) as isize)).WordList)
                                    .is_null()
                                    && !((*(*yyvsp.offset(-(5 as libc::c_int) as isize)).WordList)
                                        .next)
                                        .is_null()
                                {
                                    list_reverse(
                                        (*yyvsp.offset(-(5 as libc::c_int) as isize)).WordList
                                            as *mut GENERIC_LIST,
                                    ) as *mut WORD_LIST
                                } else {
                                    (*yyvsp.offset(-(5 as libc::c_int) as isize)).WordList
                                },
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                                word_lineno[word_top as usize],
                            );
                            if word_top > 0 as libc::c_int {
                                word_top -= 1;
                            }
                            current_block = 12964251834421583797;
                        }
                        82 => {
                            yyval.command = make_for_command(
                                (*yyvsp.offset(-(7 as libc::c_int) as isize)).word,
                                0 as *mut libc::c_void as *mut WORD_LIST,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                                word_lineno[word_top as usize],
                            );
                            if word_top > 0 as libc::c_int {
                                word_top -= 1;
                            }
                            current_block = 12964251834421583797;
                        }
                        83 => {
                            yyval.command = make_for_command(
                                (*yyvsp.offset(-(7 as libc::c_int) as isize)).word,
                                0 as *mut libc::c_void as *mut WORD_LIST,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                                word_lineno[word_top as usize],
                            );
                            if word_top > 0 as libc::c_int {
                                word_top -= 1;
                            }
                            current_block = 12964251834421583797;
                        }
                        84 => {
                            yyval.command = make_arith_for_command(
                                (*yyvsp.offset(-(5 as libc::c_int) as isize)).WordList,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                                arith_for_lineno,
                            );
                            if (yyval.command).is_null() {
                                current_block = 15232662153667146846;
                            } else {
                                if word_top > 0 as libc::c_int {
                                    word_top -= 1;
                                }
                                current_block = 12964251834421583797;
                            }
                        }
                        85 => {
                            yyval.command = make_arith_for_command(
                                (*yyvsp.offset(-(5 as libc::c_int) as isize)).WordList,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                                arith_for_lineno,
                            );
                            if (yyval.command).is_null() {
                                current_block = 15232662153667146846;
                            } else {
                                if word_top > 0 as libc::c_int {
                                    word_top -= 1;
                                }
                                current_block = 12964251834421583797;
                            }
                        }
                        86 => {
                            yyval.command = make_arith_for_command(
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).WordList,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                                arith_for_lineno,
                            );
                            if (yyval.command).is_null() {
                                current_block = 15232662153667146846;
                            } else {
                                if word_top > 0 as libc::c_int {
                                    word_top -= 1;
                                }
                                current_block = 12964251834421583797;
                            }
                        }
                        87 => {
                            yyval.command = make_arith_for_command(
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).WordList,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                                arith_for_lineno,
                            );
                            if (yyval.command).is_null() {
                                current_block = 15232662153667146846;
                            } else {
                                if word_top > 0 as libc::c_int {
                                    word_top -= 1;
                                }
                                current_block = 12964251834421583797;
                            }
                        }
                        88 => {
                            yyval.command = make_select_command(
                                (*yyvsp.offset(-(4 as libc::c_int) as isize)).word,
                                make_word_list(
                                    make_word(b"\"$@\"\0" as *const u8 as *const libc::c_char),
                                    0 as *mut libc::c_void as *mut WORD_LIST,
                                ),
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                                word_lineno[word_top as usize],
                            );
                            if word_top > 0 as libc::c_int {
                                word_top -= 1;
                            }
                            current_block = 12964251834421583797;
                        }
                        89 => {
                            yyval.command = make_select_command(
                                (*yyvsp.offset(-(4 as libc::c_int) as isize)).word,
                                make_word_list(
                                    make_word(b"\"$@\"\0" as *const u8 as *const libc::c_char),
                                    0 as *mut libc::c_void as *mut WORD_LIST,
                                ),
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                                word_lineno[word_top as usize],
                            );
                            if word_top > 0 as libc::c_int {
                                word_top -= 1;
                            }
                            current_block = 12964251834421583797;
                        }
                        90 => {
                            yyval.command = make_select_command(
                                (*yyvsp.offset(-(5 as libc::c_int) as isize)).word,
                                make_word_list(
                                    make_word(b"\"$@\"\0" as *const u8 as *const libc::c_char),
                                    0 as *mut libc::c_void as *mut WORD_LIST,
                                ),
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                                word_lineno[word_top as usize],
                            );
                            if word_top > 0 as libc::c_int {
                                word_top -= 1;
                            }
                            current_block = 12964251834421583797;
                        }
                        91 => {
                            yyval.command = make_select_command(
                                (*yyvsp.offset(-(5 as libc::c_int) as isize)).word,
                                make_word_list(
                                    make_word(b"\"$@\"\0" as *const u8 as *const libc::c_char),
                                    0 as *mut libc::c_void as *mut WORD_LIST,
                                ),
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                                word_lineno[word_top as usize],
                            );
                            if word_top > 0 as libc::c_int {
                                word_top -= 1;
                            }
                            current_block = 12964251834421583797;
                        }
                        92 => {
                            yyval.command = make_select_command(
                                (*yyvsp.offset(-(8 as libc::c_int) as isize)).word,
                                if !((*yyvsp.offset(-(5 as libc::c_int) as isize)).WordList)
                                    .is_null()
                                    && !((*(*yyvsp.offset(-(5 as libc::c_int) as isize)).WordList)
                                        .next)
                                        .is_null()
                                {
                                    list_reverse(
                                        (*yyvsp.offset(-(5 as libc::c_int) as isize)).WordList
                                            as *mut GENERIC_LIST,
                                    ) as *mut WORD_LIST
                                } else {
                                    (*yyvsp.offset(-(5 as libc::c_int) as isize)).WordList
                                },
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                                word_lineno[word_top as usize],
                            );
                            if word_top > 0 as libc::c_int {
                                word_top -= 1;
                            }
                            current_block = 12964251834421583797;
                        }
                        93 => {
                            yyval.command = make_select_command(
                                (*yyvsp.offset(-(8 as libc::c_int) as isize)).word,
                                if !((*yyvsp.offset(-(5 as libc::c_int) as isize)).WordList)
                                    .is_null()
                                    && !((*(*yyvsp.offset(-(5 as libc::c_int) as isize)).WordList)
                                        .next)
                                        .is_null()
                                {
                                    list_reverse(
                                        (*yyvsp.offset(-(5 as libc::c_int) as isize)).WordList
                                            as *mut GENERIC_LIST,
                                    ) as *mut WORD_LIST
                                } else {
                                    (*yyvsp.offset(-(5 as libc::c_int) as isize)).WordList
                                },
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                                word_lineno[word_top as usize],
                            );
                            if word_top > 0 as libc::c_int {
                                word_top -= 1;
                            }
                            current_block = 12964251834421583797;
                        }
                        94 => {
                            yyval.command = make_select_command(
                                (*yyvsp.offset(-(7 as libc::c_int) as isize)).word,
                                0 as *mut libc::c_void as *mut WORD_LIST,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                                word_lineno[word_top as usize],
                            );
                            if word_top > 0 as libc::c_int {
                                word_top -= 1;
                            }
                            current_block = 12964251834421583797;
                        }
                        95 => {
                            yyval.command = make_select_command(
                                (*yyvsp.offset(-(7 as libc::c_int) as isize)).word,
                                0 as *mut libc::c_void as *mut WORD_LIST,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                                word_lineno[word_top as usize],
                            );
                            if word_top > 0 as libc::c_int {
                                word_top -= 1;
                            }
                            current_block = 12964251834421583797;
                        }
                        96 => {
                            yyval.command = make_case_command(
                                (*yyvsp.offset(-(4 as libc::c_int) as isize)).word,
                                0 as *mut libc::c_void as *mut PATTERN_LIST,
                                word_lineno[word_top as usize],
                            );
                            if word_top > 0 as libc::c_int {
                                word_top -= 1;
                            }
                            current_block = 12964251834421583797;
                        }
                        97 => {
                            yyval.command = make_case_command(
                                (*yyvsp.offset(-(5 as libc::c_int) as isize)).word,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).pattern,
                                word_lineno[word_top as usize],
                            );
                            if word_top > 0 as libc::c_int {
                                word_top -= 1;
                            }
                            current_block = 12964251834421583797;
                        }
                        98 => {
                            yyval.command = make_case_command(
                                (*yyvsp.offset(-(4 as libc::c_int) as isize)).word,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).pattern,
                                word_lineno[word_top as usize],
                            );
                            if word_top > 0 as libc::c_int {
                                word_top -= 1;
                            }
                            current_block = 12964251834421583797;
                        }
                        99 => {
                            yyval.command = make_function_def(
                                (*yyvsp.offset(-(4 as libc::c_int) as isize)).word,
                                (*yyvsp.offset(0 as libc::c_int as isize)).command,
                                function_dstart,
                                function_bstart,
                            );
                            current_block = 12964251834421583797;
                        }
                        100 => {
                            yyval.command = make_function_def(
                                (*yyvsp.offset(-(4 as libc::c_int) as isize)).word,
                                (*yyvsp.offset(0 as libc::c_int as isize)).command,
                                function_dstart,
                                function_bstart,
                            );
                            current_block = 12964251834421583797;
                        }
                        101 => {
                            yyval.command = make_function_def(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).word,
                                (*yyvsp.offset(0 as libc::c_int as isize)).command,
                                function_dstart,
                                function_bstart,
                            );
                            current_block = 12964251834421583797;
                        }
                        102 => {
                            yyval.command = make_function_def(
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).word,
                                (*yyvsp.offset(0 as libc::c_int as isize)).command,
                                function_dstart,
                                function_bstart,
                            );
                            current_block = 12964251834421583797;
                        }
                        103 => {
                            yyval.command = (*yyvsp.offset(0 as libc::c_int as isize)).command;
                            current_block = 12964251834421583797;
                        }
                        104 => {
                            let mut tc_0: *mut COMMAND = 0 as *mut COMMAND;
                            tc_0 = (*yyvsp.offset(-(1 as libc::c_int) as isize)).command;
                            if !tc_0.is_null() && !((*tc_0).redirects).is_null() {
                                let mut t_1: *mut REDIRECT = 0 as *mut REDIRECT;
                                t_1 = (*tc_0).redirects;
                                while !((*t_1).next).is_null() {
                                    t_1 = (*t_1).next;
                                }
                                (*t_1).next = (*yyvsp.offset(0 as libc::c_int as isize)).redirect;
                            } else if !tc_0.is_null() {
                                (*tc_0).redirects =
                                    (*yyvsp.offset(0 as libc::c_int as isize)).redirect;
                            }
                            yyval.command = (*yyvsp.offset(-(1 as libc::c_int) as isize)).command;
                            current_block = 12964251834421583797;
                        }
                        105 => {
                            yyval.command = make_subshell_command(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                            );
                            (*yyval.command).flags |= 0x1 as libc::c_int;
                            current_block = 12964251834421583797;
                        }
                        106 => {
                            yyval.command = make_coproc_command(
                                b"COPROC\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                (*yyvsp.offset(0 as libc::c_int as isize)).command,
                            );
                            (*yyval.command).flags |= 0x1 as libc::c_int | 0x1000 as libc::c_int;
                            current_block = 12964251834421583797;
                        }
                        107 => {
                            let mut tc_1: *mut COMMAND = 0 as *mut COMMAND;
                            tc_1 = (*yyvsp.offset(-(1 as libc::c_int) as isize)).command;
                            if !tc_1.is_null() && !((*tc_1).redirects).is_null() {
                                let mut t_2: *mut REDIRECT = 0 as *mut REDIRECT;
                                t_2 = (*tc_1).redirects;
                                while !((*t_2).next).is_null() {
                                    t_2 = (*t_2).next;
                                }
                                (*t_2).next = (*yyvsp.offset(0 as libc::c_int as isize)).redirect;
                            } else if !tc_1.is_null() {
                                (*tc_1).redirects =
                                    (*yyvsp.offset(0 as libc::c_int as isize)).redirect;
                            }
                            yyval.command = make_coproc_command(
                                b"COPROC\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                            );
                            (*yyval.command).flags |= 0x1 as libc::c_int | 0x1000 as libc::c_int;
                            current_block = 12964251834421583797;
                        }
                        108 => {
                            yyval.command = make_coproc_command(
                                (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).word).word,
                                (*yyvsp.offset(0 as libc::c_int as isize)).command,
                            );
                            (*yyval.command).flags |= 0x1 as libc::c_int | 0x1000 as libc::c_int;
                            current_block = 12964251834421583797;
                        }
                        109 => {
                            let mut tc_2: *mut COMMAND = 0 as *mut COMMAND;
                            tc_2 = (*yyvsp.offset(-(1 as libc::c_int) as isize)).command;
                            if !tc_2.is_null() && !((*tc_2).redirects).is_null() {
                                let mut t_3: *mut REDIRECT = 0 as *mut REDIRECT;
                                t_3 = (*tc_2).redirects;
                                while !((*t_3).next).is_null() {
                                    t_3 = (*t_3).next;
                                }
                                (*t_3).next = (*yyvsp.offset(0 as libc::c_int as isize)).redirect;
                            } else if !tc_2.is_null() {
                                (*tc_2).redirects =
                                    (*yyvsp.offset(0 as libc::c_int as isize)).redirect;
                            }
                            yyval.command = make_coproc_command(
                                (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).word).word,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                            );
                            (*yyval.command).flags |= 0x1 as libc::c_int | 0x1000 as libc::c_int;
                            current_block = 12964251834421583797;
                        }
                        110 => {
                            yyval.command = make_coproc_command(
                                b"COPROC\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                clean_simple_command(
                                    (*yyvsp.offset(0 as libc::c_int as isize)).command,
                                ),
                            );
                            (*yyval.command).flags |= 0x1 as libc::c_int | 0x1000 as libc::c_int;
                            current_block = 12964251834421583797;
                        }
                        111 => {
                            yyval.command = make_if_command(
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).command,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                                0 as *mut libc::c_void as *mut COMMAND,
                            );
                            current_block = 12964251834421583797;
                        }
                        112 => {
                            yyval.command = make_if_command(
                                (*yyvsp.offset(-(5 as libc::c_int) as isize)).command,
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).command,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                            );
                            current_block = 12964251834421583797;
                        }
                        113 => {
                            yyval.command = make_if_command(
                                (*yyvsp.offset(-(4 as libc::c_int) as isize)).command,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).command,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                            );
                            current_block = 12964251834421583797;
                        }
                        114 => {
                            yyval.command = make_group_command(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                            );
                            current_block = 12964251834421583797;
                        }
                        115 => {
                            yyval.command = make_arith_command(
                                (*yyvsp.offset(0 as libc::c_int as isize)).WordList,
                            );
                            current_block = 12964251834421583797;
                        }
                        116 => {
                            yyval.command = (*yyvsp.offset(-(1 as libc::c_int) as isize)).command;
                            current_block = 12964251834421583797;
                        }
                        117 => {
                            yyval.command = make_if_command(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).command,
                                (*yyvsp.offset(0 as libc::c_int as isize)).command,
                                0 as *mut libc::c_void as *mut COMMAND,
                            );
                            current_block = 12964251834421583797;
                        }
                        118 => {
                            yyval.command = make_if_command(
                                (*yyvsp.offset(-(4 as libc::c_int) as isize)).command,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).command,
                                (*yyvsp.offset(0 as libc::c_int as isize)).command,
                            );
                            current_block = 12964251834421583797;
                        }
                        119 => {
                            yyval.command = make_if_command(
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).command,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                                (*yyvsp.offset(0 as libc::c_int as isize)).command,
                            );
                            current_block = 12964251834421583797;
                        }
                        121 => {
                            let ref mut fresh3 =
                                (*(*yyvsp.offset(0 as libc::c_int as isize)).pattern).next;
                            *fresh3 = (*yyvsp.offset(-(1 as libc::c_int) as isize)).pattern;
                            yyval.pattern = (*yyvsp.offset(0 as libc::c_int as isize)).pattern;
                            current_block = 12964251834421583797;
                        }
                        122 => {
                            yyval.pattern = make_pattern_list(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).WordList,
                                (*yyvsp.offset(0 as libc::c_int as isize)).command,
                            );
                            current_block = 12964251834421583797;
                        }
                        123 => {
                            yyval.pattern = make_pattern_list(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).WordList,
                                0 as *mut libc::c_void as *mut COMMAND,
                            );
                            current_block = 12964251834421583797;
                        }
                        124 => {
                            yyval.pattern = make_pattern_list(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).WordList,
                                (*yyvsp.offset(0 as libc::c_int as isize)).command,
                            );
                            current_block = 12964251834421583797;
                        }
                        125 => {
                            yyval.pattern = make_pattern_list(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).WordList,
                                0 as *mut libc::c_void as *mut COMMAND,
                            );
                            current_block = 12964251834421583797;
                        }
                        126 => {
                            yyval.pattern = (*yyvsp.offset(-(1 as libc::c_int) as isize)).pattern;
                            current_block = 12964251834421583797;
                        }
                        127 => {
                            let ref mut fresh4 =
                                (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).pattern).next;
                            *fresh4 = (*yyvsp.offset(-(2 as libc::c_int) as isize)).pattern;
                            yyval.pattern = (*yyvsp.offset(-(1 as libc::c_int) as isize)).pattern;
                            current_block = 12964251834421583797;
                        }
                        128 => {
                            (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).pattern).flags |=
                                0x1 as libc::c_int;
                            yyval.pattern = (*yyvsp.offset(-(1 as libc::c_int) as isize)).pattern;
                            current_block = 12964251834421583797;
                        }
                        129 => {
                            (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).pattern).flags |=
                                0x1 as libc::c_int;
                            let ref mut fresh5 =
                                (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).pattern).next;
                            *fresh5 = (*yyvsp.offset(-(2 as libc::c_int) as isize)).pattern;
                            yyval.pattern = (*yyvsp.offset(-(1 as libc::c_int) as isize)).pattern;
                            current_block = 12964251834421583797;
                        }
                        130 => {
                            (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).pattern).flags |=
                                0x2 as libc::c_int;
                            yyval.pattern = (*yyvsp.offset(-(1 as libc::c_int) as isize)).pattern;
                            current_block = 12964251834421583797;
                        }
                        131 => {
                            (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).pattern).flags |=
                                0x2 as libc::c_int;
                            let ref mut fresh6 =
                                (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).pattern).next;
                            *fresh6 = (*yyvsp.offset(-(2 as libc::c_int) as isize)).pattern;
                            yyval.pattern = (*yyvsp.offset(-(1 as libc::c_int) as isize)).pattern;
                            current_block = 12964251834421583797;
                        }
                        132 => {
                            yyval.WordList = make_word_list(
                                (*yyvsp.offset(0 as libc::c_int as isize)).word,
                                0 as *mut libc::c_void as *mut WORD_LIST,
                            );
                            current_block = 12964251834421583797;
                        }
                        133 => {
                            yyval.WordList = make_word_list(
                                (*yyvsp.offset(0 as libc::c_int as isize)).word,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).WordList,
                            );
                            current_block = 12964251834421583797;
                        }
                        134 => {
                            yyval.command = (*yyvsp.offset(0 as libc::c_int as isize)).command;
                            if need_here_doc != 0 {
                                gather_here_documents();
                            }
                            current_block = 12964251834421583797;
                        }
                        136 => {
                            yyval.command = (*yyvsp.offset(0 as libc::c_int as isize)).command;
                            current_block = 12964251834421583797;
                        }
                        138 => {
                            if (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).command).type_0
                                as libc::c_uint
                                == cm_connection as libc::c_int as libc::c_uint
                            {
                                yyval.command = connect_async_list(
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).command,
                                    0 as *mut libc::c_void as *mut COMMAND,
                                    '&' as i32,
                                );
                            } else {
                                yyval.command = command_connect(
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).command,
                                    0 as *mut libc::c_void as *mut COMMAND,
                                    '&' as i32,
                                );
                            }
                            current_block = 12964251834421583797;
                        }
                        140 => {
                            yyval.command = command_connect(
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).command,
                                (*yyvsp.offset(0 as libc::c_int as isize)).command,
                                288 as libc::c_int,
                            );
                            current_block = 12964251834421583797;
                        }
                        141 => {
                            yyval.command = command_connect(
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).command,
                                (*yyvsp.offset(0 as libc::c_int as isize)).command,
                                289 as libc::c_int,
                            );
                            current_block = 12964251834421583797;
                        }
                        142 => {
                            if (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).command).type_0
                                as libc::c_uint
                                == cm_connection as libc::c_int as libc::c_uint
                            {
                                yyval.command = connect_async_list(
                                    (*yyvsp.offset(-(3 as libc::c_int) as isize)).command,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).command,
                                    '&' as i32,
                                );
                            } else {
                                yyval.command = command_connect(
                                    (*yyvsp.offset(-(3 as libc::c_int) as isize)).command,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).command,
                                    '&' as i32,
                                );
                            }
                            current_block = 12964251834421583797;
                        }
                        143 => {
                            yyval.command = command_connect(
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).command,
                                (*yyvsp.offset(0 as libc::c_int as isize)).command,
                                ';' as i32,
                            );
                            current_block = 12964251834421583797;
                        }
                        144 => {
                            yyval.command = command_connect(
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).command,
                                (*yyvsp.offset(0 as libc::c_int as isize)).command,
                                ';' as i32,
                            );
                            current_block = 12964251834421583797;
                        }
                        145 => {
                            yyval.command = (*yyvsp.offset(0 as libc::c_int as isize)).command;
                            current_block = 12964251834421583797;
                        }
                        148 => {
                            yyval.number = '\n' as i32;
                            current_block = 12964251834421583797;
                        }
                        149 => {
                            yyval.number = ';' as i32;
                            current_block = 12964251834421583797;
                        }
                        150 => {
                            yyval.number = 304 as libc::c_int;
                            current_block = 12964251834421583797;
                        }
                        153 => {
                            yyval.command = (*yyvsp.offset(0 as libc::c_int as isize)).command;
                            if need_here_doc != 0 {
                                gather_here_documents();
                            }
                            if parser_state & 0x40 as libc::c_int != 0
                                && current_token == shell_eof_token
                            {
                                global_command = (*yyvsp.offset(0 as libc::c_int as isize)).command;
                                eof_encountered = 0 as libc::c_int;
                                rewind_input_string();
                                current_block = 4635604590554438705;
                                break;
                            }
                            current_block = 12964251834421583797;
                        }
                        154 => {
                            if (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).command).type_0
                                as libc::c_uint
                                == cm_connection as libc::c_int as libc::c_uint
                            {
                                yyval.command = connect_async_list(
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                                    0 as *mut libc::c_void as *mut COMMAND,
                                    '&' as i32,
                                );
                            } else {
                                yyval.command = command_connect(
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).command,
                                    0 as *mut libc::c_void as *mut COMMAND,
                                    '&' as i32,
                                );
                            }
                            if need_here_doc != 0 {
                                gather_here_documents();
                            }
                            if parser_state & 0x40 as libc::c_int != 0
                                && current_token == shell_eof_token
                            {
                                global_command =
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).command;
                                eof_encountered = 0 as libc::c_int;
                                rewind_input_string();
                                current_block = 4635604590554438705;
                                break;
                            }
                            current_block = 12964251834421583797;
                        }
                        155 => {
                            yyval.command = (*yyvsp.offset(-(1 as libc::c_int) as isize)).command;
                            if need_here_doc != 0 {
                                gather_here_documents();
                            }
                            if parser_state & 0x40 as libc::c_int != 0
                                && current_token == shell_eof_token
                            {
                                global_command =
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).command;
                                eof_encountered = 0 as libc::c_int;
                                rewind_input_string();
                                current_block = 4635604590554438705;
                                break;
                            }
                            current_block = 12964251834421583797;
                        }
                        156 => {
                            yyval.command = command_connect(
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).command,
                                (*yyvsp.offset(0 as libc::c_int as isize)).command,
                                288 as libc::c_int,
                            );
                            current_block = 12964251834421583797;
                        }
                        157 => {
                            yyval.command = command_connect(
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).command,
                                (*yyvsp.offset(0 as libc::c_int as isize)).command,
                                289 as libc::c_int,
                            );
                            current_block = 12964251834421583797;
                        }
                        158 => {
                            if (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).command).type_0
                                as libc::c_uint
                                == cm_connection as libc::c_int as libc::c_uint
                            {
                                yyval.command = connect_async_list(
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).command,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).command,
                                    '&' as i32,
                                );
                            } else {
                                yyval.command = command_connect(
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).command,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).command,
                                    '&' as i32,
                                );
                            }
                            current_block = 12964251834421583797;
                        }
                        159 => {
                            yyval.command = command_connect(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).command,
                                (*yyvsp.offset(0 as libc::c_int as isize)).command,
                                ';' as i32,
                            );
                            current_block = 12964251834421583797;
                        }
                        160 => {
                            yyval.command = (*yyvsp.offset(0 as libc::c_int as isize)).command;
                            current_block = 12964251834421583797;
                        }
                        161 => {
                            yyval.command = (*yyvsp.offset(0 as libc::c_int as isize)).command;
                            current_block = 12964251834421583797;
                        }
                        162 => {
                            if !((*yyvsp.offset(0 as libc::c_int as isize)).command).is_null() {
                                (*(*yyvsp.offset(0 as libc::c_int as isize)).command).flags ^=
                                    0x4 as libc::c_int;
                            }
                            yyval.command = (*yyvsp.offset(0 as libc::c_int as isize)).command;
                            current_block = 12964251834421583797;
                        }
                        163 => {
                            if !((*yyvsp.offset(0 as libc::c_int as isize)).command).is_null() {
                                (*(*yyvsp.offset(0 as libc::c_int as isize)).command).flags |=
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).number;
                            }
                            yyval.command = (*yyvsp.offset(0 as libc::c_int as isize)).command;
                            current_block = 12964251834421583797;
                        }
                        164 => {
                            let mut x: ELEMENT = element {
                                word: 0 as *mut WORD_DESC,
                                redirect: 0 as *mut REDIRECT,
                            };
                            x.word = 0 as *mut WORD_DESC;
                            x.redirect = 0 as *mut REDIRECT;
                            yyval.command =
                                make_simple_command(x, 0 as *mut libc::c_void as *mut COMMAND);
                            (*yyval.command).flags |=
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).number;
                            if (*yyvsp.offset(0 as libc::c_int as isize)).number == '\n' as i32 {
                                token_to_read = '\n' as i32;
                            } else if (*yyvsp.offset(0 as libc::c_int as isize)).number
                                == ';' as i32
                            {
                                token_to_read = ';' as i32;
                            }
                            parser_state &= !(0x80000 as libc::c_int);
                            current_block = 12964251834421583797;
                        }
                        165 => {
                            let mut x_0: ELEMENT = element {
                                word: 0 as *mut WORD_DESC,
                                redirect: 0 as *mut REDIRECT,
                            };
                            x_0.word = 0 as *mut WORD_DESC;
                            x_0.redirect = 0 as *mut REDIRECT;
                            yyval.command =
                                make_simple_command(x_0, 0 as *mut libc::c_void as *mut COMMAND);
                            (*yyval.command).flags |= 0x4 as libc::c_int;
                            if (*yyvsp.offset(0 as libc::c_int as isize)).number == '\n' as i32 {
                                token_to_read = '\n' as i32;
                            }
                            if (*yyvsp.offset(0 as libc::c_int as isize)).number == ';' as i32 {
                                token_to_read = ';' as i32;
                            }
                            parser_state &= !(0x80000 as libc::c_int);
                            current_block = 12964251834421583797;
                        }
                        166 => {
                            yyval.command = command_connect(
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).command,
                                (*yyvsp.offset(0 as libc::c_int as isize)).command,
                                '|' as i32,
                            );
                            current_block = 12964251834421583797;
                        }
                        167 => {
                            let mut tc_3: *mut COMMAND = 0 as *mut COMMAND;
                            let mut rd: REDIRECTEE = REDIRECTEE { dest: 0 };
                            let mut sd: REDIRECTEE = REDIRECTEE { dest: 0 };
                            let mut r: *mut REDIRECT = 0 as *mut REDIRECT;
                            tc_3 = if (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).command)
                                .type_0 as libc::c_uint
                                == cm_simple as libc::c_int as libc::c_uint
                            {
                                (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).command)
                                    .value
                                    .Simple as *mut COMMAND
                            } else {
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).command
                            };
                            sd.dest = 2 as libc::c_int;
                            rd.dest = 1 as libc::c_int;
                            r = make_redirection(sd, r_duplicating_output, rd, 0 as libc::c_int);
                            if !((*tc_3).redirects).is_null() {
                                let mut t_4: *mut REDIRECT = 0 as *mut REDIRECT;
                                t_4 = (*tc_3).redirects;
                                while !((*t_4).next).is_null() {
                                    t_4 = (*t_4).next;
                                }
                                (*t_4).next = r;
                            } else {
                                (*tc_3).redirects = r;
                            }
                            yyval.command = command_connect(
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).command,
                                (*yyvsp.offset(0 as libc::c_int as isize)).command,
                                '|' as i32,
                            );
                            current_block = 12964251834421583797;
                        }
                        168 => {
                            yyval.command = (*yyvsp.offset(0 as libc::c_int as isize)).command;
                            current_block = 12964251834421583797;
                        }
                        169 => {
                            yyval.number = 0x80 as libc::c_int;
                            current_block = 12964251834421583797;
                        }
                        170 => {
                            yyval.number = 0x80 as libc::c_int | 0x100 as libc::c_int;
                            current_block = 12964251834421583797;
                        }
                        171 => {
                            yyval.number = 0x80 as libc::c_int | 0x100 as libc::c_int;
                            current_block = 12964251834421583797;
                        }
                        172 => {
                            yyval.number = 0x80 as libc::c_int | 0x100 as libc::c_int;
                            current_block = 12964251834421583797;
                        }
                        _ => {
                            current_block = 12964251834421583797;
                        }
                    }
                    match current_block {
                        12964251834421583797 => {
                            yyvsp = yyvsp.offset(-(yylen as isize));
                            yyssp = yyssp.offset(-(yylen as isize));
                            yylen = 0 as libc::c_int;
                            yyvsp = yyvsp.offset(1);
                            *yyvsp = yyval;
                            let yylhs: libc::c_int =
                                yyr1[yyn as usize] as libc::c_int - 61 as libc::c_int;
                            let yyi: libc::c_int =
                                yypgoto[yylhs as usize] as libc::c_int + *yyssp as libc::c_int;
                            yystate = if 0 as libc::c_int <= yyi
                                && yyi <= 661 as libc::c_int
                                && yycheck[yyi as usize] as libc::c_int == *yyssp as libc::c_int
                            {
                                yytable[yyi as usize] as libc::c_int
                            } else {
                                yydefgoto[yylhs as usize] as libc::c_int
                            };
                            current_block = 7425917188841100089;
                        }
                        _ => {
                            yynerrs += 1;
                            yyvsp = yyvsp.offset(-(yylen as isize));
                            yyssp = yyssp.offset(-(yylen as isize));
                            yylen = 0 as libc::c_int;
                            yystate = *yyssp as yy_state_fast_t;
                            current_block = 5467060717235750116;
                        }
                    }
                }
                _ => {}
            }
            match current_block {
                5467060717235750116 => {
                    yyerrstatus = 3 as libc::c_int;
                    loop {
                        yyn = yypact[yystate as usize] as libc::c_int;
                        if !(yyn == -(204 as libc::c_int)) {
                            yyn += YYSYMBOL_YYerror as libc::c_int;
                            if 0 as libc::c_int <= yyn
                                && yyn <= 661 as libc::c_int
                                && yycheck[yyn as usize] as libc::c_int
                                    == YYSYMBOL_YYerror as libc::c_int
                            {
                                yyn = yytable[yyn as usize] as libc::c_int;
                                if (0 as libc::c_int) < yyn {
                                    break;
                                }
                            }
                        }
                        if yyssp == yyss {
                            current_block = 12092114409115356139;
                            break 's_46;
                        }
                        yydestruct(
                            b"Error: popping\0" as *const u8 as *const libc::c_char,
                            yystos[yystate as usize] as yysymbol_kind_t,
                            yyvsp,
                        );
                        yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
                        yyssp = yyssp.offset(-(1 as libc::c_int as isize));
                        yystate = *yyssp as yy_state_fast_t;
                    }
                    yyvsp = yyvsp.offset(1);
                    *yyvsp = yylval;
                    yystate = yyn;
                }
                _ => {}
            }
            yyssp = yyssp.offset(1);
        }
        match current_block {
            5105078013716444332 => {
                yyerror(b"memory exhausted\0" as *const u8 as *const libc::c_char);
                yyresult = 2 as libc::c_int;
                current_block = 14803667992492250202;
            }
            12092114409115356139 => {
                yyresult = 1 as libc::c_int;
                current_block = 14803667992492250202;
            }
            5793491756164225964 => {
                handle_eof_input_unit();
                current_block = 4635604590554438705;
            }
            _ => {}
        }
        match current_block {
            4635604590554438705 => {
                yyresult = 0 as libc::c_int;
            }
            _ => {}
        }
        if yychar != -(2 as libc::c_int) {
            yytoken = (if 0 as libc::c_int <= yychar && yychar <= 304 as libc::c_int {
                yytranslate[yychar as usize] as yysymbol_kind_t as libc::c_int
            } else {
                YYSYMBOL_YYUNDEF as libc::c_int
            }) as yysymbol_kind_t;
            yydestruct(
                b"Cleanup: discarding lookahead\0" as *const u8 as *const libc::c_char,
                yytoken,
                &mut yylval,
            );
        }
        yyvsp = yyvsp.offset(-(yylen as isize));
        yyssp = yyssp.offset(-(yylen as isize));
        while yyssp != yyss {
            yydestruct(
                b"Cleanup: popping\0" as *const u8 as *const libc::c_char,
                yystos[*yyssp as libc::c_int as usize] as yysymbol_kind_t,
                yyvsp,
            );
            yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
            yyssp = yyssp.offset(-(1 as libc::c_int as isize));
        }
        if yyss != yyssa.as_mut_ptr() {
            free(yyss as *mut libc::c_void);
        }
        return yyresult;
    }
}

#[no_mangle]
pub fn with_input_from_stream(mut stream: *mut FILE, mut name: *const libc::c_char) {
    unsafe {
        let mut location: INPUT_STREAM = INPUT_STREAM {
            file: 0 as *const FILE as *mut FILE,
        };
        location.file = stream;
        init_yy_io(
            Some(yy_stream_get as fn() -> libc::c_int),
            ::core::mem::transmute::<Option<fn() -> libc::c_int>, Option<sh_cunget_func_t>>(Some(
                ::core::mem::transmute::<fn(libc::c_int) -> libc::c_int, fn() -> libc::c_int>(
                    yy_stream_unget,
                ),
            )),
            st_stream as libc::c_int as libc::c_uint,
            name,
            location,
        );
    }
}
#[no_mangle]
pub static mut line_number: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut line_number_base: libc::c_int = 0 as libc::c_int;
static mut cond_lineno: libc::c_int = 0;
static mut cond_token: libc::c_int = 0;
#[no_mangle]
pub static mut stream_list: *mut STREAM_SAVER =
    0 as *const libc::c_void as *mut libc::c_void as *mut STREAM_SAVER;
#[no_mangle]
pub fn push_stream(mut reset_lineno: libc::c_int) {
    unsafe {
        let mut saver: *mut STREAM_SAVER =
            malloc(::core::mem::size_of::<STREAM_SAVER>() as usize) as *mut STREAM_SAVER;
        xbcopy(
            &mut bash_input as *mut BASH_INPUT as *mut libc::c_char,
            &mut (*saver).bash_input as *mut BASH_INPUT as *mut libc::c_char,
            ::core::mem::size_of::<BASH_INPUT>() as libc::c_ulong as libc::c_int,
        );
        (*saver).bstream = 0 as *mut libc::c_void as *mut BUFFERED_STREAM;

        if bash_input.type_0 as libc::c_uint == st_bstream as libc::c_int as libc::c_uint
            && bash_input.location.buffered_fd >= 0 as libc::c_int
        {
            (*saver).bstream = set_buffered_stream(
                bash_input.location.buffered_fd,
                0 as *mut libc::c_void as *mut BUFFERED_STREAM,
            );
        }
        (*saver).line = line_number;
        bash_input.name = 0 as *mut libc::c_void as *mut libc::c_char;
        (*saver).next = stream_list;
        stream_list = saver;
        EOF_Reached = 0 as libc::c_int;
        if reset_lineno != 0 {
            line_number = 0 as libc::c_int;
        }
    }
}
#[no_mangle]
pub fn pop_stream() {
    unsafe {
        if stream_list.is_null() {
            EOF_Reached = 1 as libc::c_int;
        } else {
            let mut saver: *mut STREAM_SAVER = stream_list;
            EOF_Reached = 0 as libc::c_int;
            stream_list = (*stream_list).next;
            init_yy_io(
                (*saver).bash_input.getter,
                (*saver).bash_input.ungetter,
                (*saver).bash_input.type_0 as libc::c_uint,
                (*saver).bash_input.name,
                (*saver).bash_input.location,
            );
            if bash_input.type_0 as libc::c_uint == st_bstream as libc::c_int as libc::c_uint
                && bash_input.location.buffered_fd >= 0 as libc::c_int
            {
                if bash_input_fd_changed != 0 {
                    bash_input_fd_changed = 0 as libc::c_int;
                    if default_buffered_input >= 0 as libc::c_int {
                        bash_input.location.buffered_fd = default_buffered_input;
                        (*(*saver).bstream).b_fd = default_buffered_input;
                        fcntl(default_buffered_input, 2 as libc::c_int, 1 as libc::c_int);
                    }
                }
                set_buffered_stream(bash_input.location.buffered_fd, (*saver).bstream);
            }
            line_number = (*saver).line;
            if !((*saver).bash_input.name).is_null() {
                free((*saver).bash_input.name as *mut libc::c_void);
            }
            (*saver).bash_input.name = 0 as *mut libc::c_char;
            free(saver as *mut libc::c_void);
        };
    }
}
#[no_mangle]
pub fn stream_on_stack(mut type_0: stream_type) -> libc::c_int {
    unsafe {
        let mut s: *mut STREAM_SAVER = 0 as *mut STREAM_SAVER;
        s = stream_list;
        while !s.is_null() {
            if (*s).bash_input.type_0 as libc::c_uint == type_0 as libc::c_uint {
                return 1 as libc::c_int;
            }
            s = (*s).next;
        }
        return 0 as libc::c_int;
    }
}
#[no_mangle]
pub fn save_token_state() -> *mut libc::c_int {
    unsafe {
        let mut ret: *mut libc::c_int = 0 as *mut libc::c_int;

        ret = malloc(
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as usize,
        ) as *mut libc::c_int;

        *ret.offset(0 as libc::c_int as isize) = last_read_token;
        *ret.offset(1 as libc::c_int as isize) = token_before_that;
        *ret.offset(2 as libc::c_int as isize) = two_tokens_ago;
        *ret.offset(3 as libc::c_int as isize) = current_token;

        return ret;
    }
}
#[no_mangle]
pub fn restore_token_state(mut ts: *mut libc::c_int) {
    unsafe {
        if ts.is_null() {
            return;
        }

        last_read_token = *ts.offset(0 as libc::c_int as isize);
        token_before_that = *ts.offset(1 as libc::c_int as isize);
        two_tokens_ago = *ts.offset(2 as libc::c_int as isize);
        current_token = *ts.offset(3 as libc::c_int as isize);
    }
}
#[no_mangle]
pub static mut pushed_string_list: *mut STRING_SAVER =
    0 as *const libc::c_void as *mut libc::c_void as *mut STRING_SAVER;
fn push_string(mut s: *mut libc::c_char, mut expand: libc::c_int, mut ap: *mut alias_t) {
    unsafe {
        let mut temp: *mut STRING_SAVER =
            malloc(::core::mem::size_of::<STRING_SAVER>() as usize) as *mut STRING_SAVER;
        (*temp).expand_alias = expand;
        (*temp).saved_line = shell_input_line;
        (*temp).saved_line_size = shell_input_line_size;
        (*temp).saved_line_index = shell_input_line_index;
        (*temp).saved_line_terminator = shell_input_line_terminator;
        (*temp).flags = 0 as libc::c_int;
        (*temp).expander = ap;
        if !ap.is_null() {
            (*temp).flags = 0x1 as libc::c_int;
        }
        (*temp).next = pushed_string_list;
        pushed_string_list = temp;
        if !ap.is_null() {
            (*ap).flags = ((*ap).flags as libc::c_int | 0x2 as libc::c_int) as libc::c_char;
        }
        shell_input_line = s;
        shell_input_line_size =
            if !s.is_null() && *s.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
                if *s.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                    if *s.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                        libc::strlen(s) as u64
                    } else {
                        2 as libc::c_int as libc::c_ulong
                    }
                } else {
                    1 as libc::c_int as libc::c_ulong
                }
            } else {
                0 as libc::c_int as libc::c_ulong
            };
        shell_input_line_index = 0 as libc::c_int as size_t;
        shell_input_line_terminator = '\0' as i32;
        set_line_mbstate();
    }
}
fn pop_string() {
    unsafe {
        let mut t: *mut STRING_SAVER = 0 as *mut STRING_SAVER;

        if !shell_input_line.is_null() {
            free(shell_input_line as *mut libc::c_void);
        }
        shell_input_line = 0 as *mut libc::c_char;
        shell_input_line = (*pushed_string_list).saved_line;
        shell_input_line_index = (*pushed_string_list).saved_line_index;
        shell_input_line_size = (*pushed_string_list).saved_line_size;
        shell_input_line_terminator = (*pushed_string_list).saved_line_terminator;
        if (*pushed_string_list).expand_alias != 0 {
            parser_state |= 0x2 as libc::c_int;
        } else {
            parser_state &= !(0x2 as libc::c_int);
        }
        t = pushed_string_list;
        pushed_string_list = (*pushed_string_list).next;
        if !((*t).expander).is_null() {
            (*(*t).expander).flags =
                ((*(*t).expander).flags as libc::c_int & !(0x2 as libc::c_int)) as libc::c_char;
        }
        free(t as *mut libc::c_char as *mut libc::c_void);

        set_line_mbstate();
    }
}
fn free_string_list() {
    unsafe {
        let mut t: *mut STRING_SAVER = 0 as *mut STRING_SAVER;
        let mut t1: *mut STRING_SAVER = 0 as *mut STRING_SAVER;

        t = pushed_string_list;
        while !t.is_null() {
            t1 = (*t).next;
            if !((*t).saved_line).is_null() {
                free((*t).saved_line as *mut libc::c_void);
            }
            (*t).saved_line = 0 as *mut libc::c_char;
            if !((*t).expander).is_null() {
                (*(*t).expander).flags =
                    ((*(*t).expander).flags as libc::c_int & !(0x2 as libc::c_int)) as libc::c_char;
            }
            free(t as *mut libc::c_char as *mut libc::c_void);
            t = t1;
        }
        pushed_string_list = 0 as *mut libc::c_void as *mut STRING_SAVER;
    }
}
#[no_mangle]
pub fn free_pushed_string_input() {
    free_string_list();
}
#[no_mangle]
pub fn parser_expanding_alias() -> libc::c_int {
    unsafe {
        return (!pushed_string_list.is_null() && !((*pushed_string_list).expander).is_null())
            as libc::c_int;
    }
}
#[no_mangle]
pub fn parser_save_alias() {
    unsafe {
        push_string(
            0 as *mut libc::c_void as *mut libc::c_char,
            0 as libc::c_int,
            0 as *mut libc::c_void as *mut alias_t,
        );
        (*pushed_string_list).flags = 0x4 as libc::c_int;
    }
}
#[no_mangle]
pub fn parser_restore_alias() {
    unsafe {
        if !pushed_string_list.is_null() {
            pop_string();
        }
    }
}
#[no_mangle]
pub fn clear_string_list_expander(mut ap: *mut alias_t) {
    unsafe {
        let mut t: *mut STRING_SAVER = 0 as *mut STRING_SAVER;

        t = pushed_string_list;
        while !t.is_null() {
            if !((*t).expander).is_null() && (*t).expander == ap {
                (*t).expander = 0 as *mut alias_t;
            }
            t = (*t).next;
        }
    }
}
#[no_mangle]
pub fn clear_shell_input_line() {
    unsafe {
        if !shell_input_line.is_null() {
            shell_input_line_index = 0 as libc::c_int as size_t;
            *shell_input_line.offset(shell_input_line_index as isize) = '\0' as i32 as libc::c_char;
        }
    }
}
fn read_a_line(mut remove_quoted_newline: libc::c_int) -> *mut libc::c_char {
    unsafe {
        static mut line_buffer: *mut libc::c_char =
            0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char;
        static mut buffer_size: libc::c_int = 0 as libc::c_int;
        let mut indx: libc::c_int = 0;
        let mut c: libc::c_int = 0;
        let mut peekc: libc::c_int = 0;
        let mut pass_next: libc::c_int = 0;

        if no_line_editing != 0
            && (interactive != 0
                && (bash_input.type_0 as libc::c_uint == st_stdin as libc::c_int as libc::c_uint
                    || bash_input.type_0 as libc::c_uint
                        == st_stream as libc::c_int as libc::c_uint))
        {
            print_prompt();
        }
        indx = 0 as libc::c_int;
        pass_next = indx;
        loop {
            if terminating_signal != 0 {
                termsig_handler(terminating_signal);
            }
            if interrupt_state != 0 {
                throw_to_top_level();
            }
            c = yy_getc();
            if c == 0 as libc::c_int {
                continue;
            }
            if c == -(1 as libc::c_int) {
                if interactive != 0
                    && bash_input.type_0 as libc::c_uint == st_stream as libc::c_int as libc::c_uint
                {
                    c_clearerr(stdin);
                }

                if indx == 0 as libc::c_int {
                    return 0 as *mut libc::c_void as *mut libc::c_char;
                }
                c = '\n' as i32;
            }

            if indx + 2 as libc::c_int >= buffer_size {
                while indx + 2 as libc::c_int >= buffer_size {
                    buffer_size += 128 as libc::c_int;
                }
                line_buffer = realloc(line_buffer as *mut libc::c_void, buffer_size as usize)
                    as *mut libc::c_char;
            }

            if pass_next != 0 {
                let fresh7 = indx;
                indx = indx + 1;
                *line_buffer.offset(fresh7 as isize) = c as libc::c_char;
                pass_next = 0 as libc::c_int;
            } else if c == '\\' as i32 && remove_quoted_newline != 0 {
                if terminating_signal != 0 {
                    termsig_handler(terminating_signal);
                }
                if interrupt_state != 0 {
                    throw_to_top_level();
                }
                peekc = yy_getc();
                if peekc == '\n' as i32 {
                    line_number += 1;

                    continue;
                } else {
                    yy_ungetc(peekc);
                    pass_next = 1 as libc::c_int;
                    let fresh8 = indx;
                    indx = indx + 1;

                    *line_buffer.offset(fresh8 as isize) = c as libc::c_char;
                }
            } else {
                if remove_quoted_newline != 0 && (c == '\u{1}' as i32 || c == '\u{7f}' as i32) {
                    let fresh9 = indx;
                    indx = indx + 1;

                    *line_buffer.offset(fresh9 as isize) = '\u{1}' as i32 as libc::c_char;
                }
                let fresh10 = indx;
                indx = indx + 1;

                *line_buffer.offset(fresh10 as isize) = c as libc::c_char;
            }
            if c == '\n' as i32 {
                *line_buffer.offset(indx as isize) = '\0' as i32 as libc::c_char;

                return line_buffer;
            }
        }
    }
}
#[no_mangle]
pub fn read_secondary_line(mut remove_quoted_newline: libc::c_int) -> *mut libc::c_char {
    unsafe {
        let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut _n: libc::c_int = 0;
        let mut _c: libc::c_int = 0;
        prompt_string_pointer = &mut ps2_prompt;
        if interactive != 0
            && (bash_input.type_0 as libc::c_uint == st_stdin as libc::c_int as libc::c_uint
                || bash_input.type_0 as libc::c_uint == st_stream as libc::c_int as libc::c_uint)
        {
            prompt_again();
        }
        ret = read_a_line(remove_quoted_newline);

        if !ret.is_null() && remember_on_history != 0 && parser_state & 0x20000 as libc::c_int != 0
        {
            current_command_line_count += 1;
            maybe_add_history(ret);
        }

        return ret;
    }
}
#[no_mangle]
pub static mut word_token_alist: [STRING_INT_ALIST; 23] = [
    {
        let mut init = STRING_INT_ALIST {
            word: b"if\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 258 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"then\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 259 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"else\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 260 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"elif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 261 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"fi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 262 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"case\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 263 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"esac\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 264 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"for\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 265 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"select\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 266 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"while\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 267 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"until\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 268 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"do\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 269 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"done\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 270 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"in\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 276 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"function\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 271 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"time\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 278 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"{\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: '{' as i32,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"}\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: '}' as i32,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"!\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 277 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"[[\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 273 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"]]\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 274 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"coproc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 272 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
            token: 0 as libc::c_int,
        };
        init
    },
];
#[no_mangle]
pub static mut other_token_alist: [STRING_INT_ALIST; 31] = [
    {
        let mut init = STRING_INT_ALIST {
            word: b"--\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 280 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"-p\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 279 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"&&\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 288 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"||\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 289 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b">>\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 290 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"<<\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 291 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"<&\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 292 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b">&\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 294 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b";;\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 295 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b";&\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 296 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b";;&\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 297 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"<<-\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 298 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"<<<\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 293 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"&>\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 299 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"&>>\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 300 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"<>\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 301 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b">|\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 302 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"|&\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 303 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"EOF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 304 as libc::c_int,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b">\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: '>' as i32,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"<\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: '<' as i32,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"-\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: '-' as i32,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"{\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: '{' as i32,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"}\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: '}' as i32,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b";\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: ';' as i32,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"(\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: '(' as i32,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b")\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: ')' as i32,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"|\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: '|' as i32,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"&\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: '&' as i32,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: b"newline\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: '\n' as i32,
        };
        init
    },
    {
        let mut init = STRING_INT_ALIST {
            word: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
            token: 0 as libc::c_int,
        };
        init
    },
];
#[no_mangle]
pub static mut dstack: dstack = {
    let mut init = dstack {
        delimiters: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
        delimiter_depth: 0 as libc::c_int,
        delimiter_space: 0 as libc::c_int,
    };
    init
};
static mut temp_dstack: dstack = {
    let mut init = dstack {
        delimiters: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
        delimiter_depth: 0 as libc::c_int,
        delimiter_space: 0 as libc::c_int,
    };
    init
};
static mut eol_ungetc_lookahead: libc::c_int = 0 as libc::c_int;
static mut unquoted_backslash: libc::c_int = 0 as libc::c_int;
fn shell_getc(mut remove_quoted_newline: libc::c_int) -> libc::c_int {
    // shell获取字符
    unsafe {
        let mut current_block: u64;
        let mut i: libc::c_int = 0;
        let mut c: libc::c_int = 0;
        let mut truncating: libc::c_int = 0;
        let mut last_was_backslash: libc::c_int = 0;
        let mut uc: libc::c_uchar = 0;
        if terminating_signal != 0 {
            termsig_handler(terminating_signal);
        }
        if interrupt_state != 0 {
            throw_to_top_level();
        }
        last_was_backslash = 0 as libc::c_int;

        if sigwinch_received != 0 {
            ::core::ptr::write_volatile(
                &mut sigwinch_received as *mut sig_atomic_t,
                0 as libc::c_int,
            );
            c_get_new_window_size(
                0 as libc::c_int,
                0 as *mut libc::c_int,
                0 as *mut libc::c_int,
            );
        }

        if eol_ungetc_lookahead != 0 {
            c = eol_ungetc_lookahead;
            eol_ungetc_lookahead = 0 as libc::c_int;
            return c;
        }
        if shell_input_line.is_null()
            || *shell_input_line.offset(shell_input_line_index as isize) == 0
                && pushed_string_list.is_null()
        {
            line_number += 1;
            if !shell_input_line.is_null()
                && shell_input_line_size >= 32768 as libc::c_int as size_t
            {
                free(shell_input_line as *mut libc::c_void);
                shell_input_line = 0 as *mut libc::c_char;
                shell_input_line_size = 0 as libc::c_int as size_t;
            }
            current_block = 11101796020613380314;
        } else {
            current_block = 9354102534551541821;
        }

        '_next_alias_char: loop {
            match current_block {
                11101796020613380314 => {
                    if terminating_signal != 0 {
                        termsig_handler(terminating_signal);
                    }
                    if interrupt_state != 0 {
                        throw_to_top_level();
                    }
                    truncating = 0 as libc::c_int;
                    i = truncating;
                    shell_input_line_terminator = 0 as libc::c_int;
                    if interactive_shell == 0 as libc::c_int
                        || interactive != 0
                            && (bash_input.type_0 as libc::c_uint
                                == st_stdin as libc::c_int as libc::c_uint
                                || bash_input.type_0 as libc::c_uint
                                    == st_stream as libc::c_int as libc::c_uint)
                    {
                        notify_and_cleanup();
                    }
                    if no_line_editing != 0
                        && (interactive != 0
                            && (bash_input.type_0 as libc::c_uint
                                == st_stdin as libc::c_int as libc::c_uint
                                || bash_input.type_0 as libc::c_uint
                                    == st_stream as libc::c_int as libc::c_uint))
                    {
                        print_prompt();
                    }
                    if bash_input.type_0 as libc::c_uint == st_stream as libc::c_int as libc::c_uint
                    {
                        c_clearerr(stdin);
                    }
                    loop {
                        c = yy_getc();
                        if terminating_signal != 0 {
                            termsig_handler(terminating_signal);
                        }
                        if interrupt_state != 0 {
                            throw_to_top_level();
                        }
                        if c == '\0' as i32 {
                            if !(bash_input.type_0 as libc::c_uint
                                == st_string as libc::c_int as libc::c_uint)
                            {
                                continue;
                            }
                            if i == 0 as libc::c_int {
                                shell_input_line_terminator = -(1 as libc::c_int);
                            }
                            *shell_input_line.offset(i as isize) = '\0' as i32 as libc::c_char;
                            c = -(1 as libc::c_int);
                            break;
                        } else {
                            if shell_input_line_size
                                > (18446744073709551615 as libc::c_ulong)
                                    .wrapping_sub(256 as libc::c_int as libc::c_ulong)
                            {
                                let mut n: size_t = 0;
                                n = (18446744073709551615 as libc::c_ulong)
                                    .wrapping_sub(i as libc::c_ulong);
                                if n <= 2 as libc::c_int as size_t {
                                    if truncating == 0 as libc::c_int {
                                        internal_warning(
                                        c_dcgettext(
                                            0 as *const libc::c_char,
                                            b"shell_getc: shell_input_line_size (%zu) exceeds SIZE_MAX (%lu): line truncated\0"
                                                as *const u8 as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                        shell_input_line_size,
                                        18446744073709551615 as libc::c_ulong,
                                    );
                                    }
                                    *shell_input_line.offset(i as isize) =
                                        '\0' as i32 as libc::c_char;
                                    truncating = 1 as libc::c_int;
                                }
                                if shell_input_line_size < 18446744073709551615 as libc::c_ulong {
                                    shell_input_line_size = 18446744073709551615 as libc::c_ulong;
                                    shell_input_line = realloc(
                                        shell_input_line as *mut libc::c_void,
                                        shell_input_line_size as usize,
                                    )
                                        as *mut libc::c_char;
                                }
                            } else if (i + 2 as libc::c_int) as size_t >= shell_input_line_size {
                                while (i + 2 as libc::c_int) as size_t >= shell_input_line_size {
                                    shell_input_line_size = shell_input_line_size
                                        .wrapping_add(256 as libc::c_int as size_t);
                                }
                                shell_input_line = realloc(
                                    shell_input_line as *mut libc::c_void,
                                    shell_input_line_size as usize,
                                )
                                    as *mut libc::c_char;
                            }
                            if c == -(1 as libc::c_int) {
                                if bash_input.type_0 as libc::c_uint
                                    == st_stream as libc::c_int as libc::c_uint
                                {
                                    c_clearerr(stdin);
                                }
                                if i == 0 as libc::c_int {
                                    shell_input_line_terminator = -(1 as libc::c_int);
                                }
                                *shell_input_line.offset(i as isize) = '\0' as i32 as libc::c_char;
                                break;
                            } else {
                                if truncating == 0 as libc::c_int || c == '\n' as i32 {
                                    let fresh11 = i;
                                    i = i + 1;
                                    *shell_input_line.offset(fresh11 as isize) = c as libc::c_char;
                                }
                                if c == '\n' as i32 {
                                    i -= 1;
                                    *shell_input_line.offset(i as isize) =
                                        '\0' as i32 as libc::c_char;
                                    current_command_line_count += 1;
                                    break;
                                } else {
                                    last_was_backslash = (last_was_backslash == 0 as libc::c_int
                                        && c == '\\' as i32)
                                        as libc::c_int;
                                }
                            }
                        }
                    }
                    shell_input_line_index = 0 as libc::c_int as size_t;
                    shell_input_line_len = i as size_t;
                    set_line_mbstate();
                    if remember_on_history != 0
                        && !shell_input_line.is_null()
                        && *shell_input_line.offset(0 as libc::c_int as isize) as libc::c_int != 0
                    {
                        let mut expansions: *mut libc::c_char = 0 as *mut libc::c_char;
                        if (if dstack.delimiter_depth != 0 {
                            *(dstack.delimiters)
                                .offset((dstack.delimiter_depth - 1 as libc::c_int) as isize)
                                as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) == '\'' as i32
                        {
                            history_quoting_state = '\'' as i32;
                        } else if (if dstack.delimiter_depth != 0 {
                            *(dstack.delimiters)
                                .offset((dstack.delimiter_depth - 1 as libc::c_int) as isize)
                                as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) == '"' as i32
                        {
                            history_quoting_state = '"' as i32;
                        } else {
                            history_quoting_state = 0 as libc::c_int;
                        }
                        expansions =
                            pre_process_line(shell_input_line, 1 as libc::c_int, 1 as libc::c_int);
                        history_quoting_state = 0 as libc::c_int;
                        if expansions != shell_input_line {
                            free(shell_input_line as *mut libc::c_void);
                            shell_input_line = expansions;
                            shell_input_line_len = if !shell_input_line.is_null() {
                                libc::strlen(shell_input_line) as u64
                            } else {
                                0 as libc::c_int as libc::c_ulong
                            };
                            if shell_input_line_len == 0 as libc::c_int as size_t {
                                current_command_line_count -= 1;
                            }
                            shell_input_line_size = shell_input_line_len;
                            set_line_mbstate();
                        }
                    } else if remember_on_history != 0
                        && !shell_input_line.is_null()
                        && *shell_input_line.offset(0 as libc::c_int as isize) as libc::c_int
                            == '\0' as i32
                        && current_command_line_count > 1 as libc::c_int
                    {
                        if if dstack.delimiter_depth != 0 {
                            *(dstack.delimiters)
                                .offset((dstack.delimiter_depth - 1 as libc::c_int) as isize)
                                as libc::c_int
                        } else {
                            0 as libc::c_int
                        } != 0
                        {
                            maybe_add_history(shell_input_line);
                        } else {
                            let mut hdcs: *mut libc::c_char = 0 as *mut libc::c_char;
                            hdcs = history_delimiting_chars(shell_input_line);
                            if !hdcs.is_null()
                                && *hdcs.offset(0 as libc::c_int as isize) as libc::c_int
                                    == ';' as i32
                            {
                                maybe_add_history(shell_input_line);
                            }
                        }
                    }
                    if !shell_input_line.is_null() {
                        if echo_input_at_read != 0
                            && (*shell_input_line.offset(0 as libc::c_int as isize) as libc::c_int
                                != 0
                                || shell_input_line_terminator != -(1 as libc::c_int))
                            && shell_eof_token == 0 as libc::c_int
                        {
                            fprintf(
                                stderr,
                                b"%s\n\0" as *const u8 as *const libc::c_char,
                                shell_input_line,
                            );
                        }
                        if shell_input_line_terminator != -(1 as libc::c_int) {
                            if shell_input_line_size
                                < (18446744073709551615 as libc::c_ulong)
                                    .wrapping_sub(3 as libc::c_int as libc::c_ulong)
                                && shell_input_line_len.wrapping_add(3 as libc::c_int as size_t)
                                    > shell_input_line_size
                            {
                                shell_input_line_size =
                                    shell_input_line_size.wrapping_add(2 as libc::c_int as size_t);
                                shell_input_line = realloc(
                                    shell_input_line as *mut libc::c_void,
                                    (1 as libc::c_int as size_t).wrapping_add(shell_input_line_size)
                                        as usize,
                                )
                                    as *mut libc::c_char;
                            }
                            if bash_input.type_0 as libc::c_uint
                                == st_string as libc::c_int as libc::c_uint
                                && (!pushed_string_list.is_null()
                                    && !((*pushed_string_list).expander).is_null())
                                    as libc::c_int
                                    == 0 as libc::c_int
                                && last_was_backslash != 0
                                && c == -(1 as libc::c_int)
                                && remove_quoted_newline != 0
                            {
                                *shell_input_line.offset(shell_input_line_len as isize) =
                                    '\\' as i32 as libc::c_char;
                            } else {
                                *shell_input_line.offset(shell_input_line_len as isize) =
                                    '\n' as i32 as libc::c_char;
                            }
                            *shell_input_line.offset(
                                shell_input_line_len.wrapping_add(1 as libc::c_int as size_t)
                                    as isize,
                            ) = '\0' as i32 as libc::c_char;
                            if shell_input_line_len.wrapping_add(2 as libc::c_int as size_t)
                                > shell_input_line_propsize
                            {
                                shell_input_line_propsize =
                                    shell_input_line_len.wrapping_add(2 as libc::c_int as size_t);
                                shell_input_line_property = realloc(
                                    shell_input_line_property as *mut libc::c_void,
                                    shell_input_line_propsize as usize,
                                )
                                    as *mut libc::c_char;
                            }
                            *shell_input_line_property.offset(shell_input_line_len as isize) =
                                1 as libc::c_int as libc::c_char;
                        }
                        current_block = 9354102534551541821;
                    } else {
                        shell_input_line_size = 0 as libc::c_int as size_t;
                        prompt_string_pointer = &mut current_prompt_string;
                        if interactive != 0
                            && (bash_input.type_0 as libc::c_uint
                                == st_stdin as libc::c_int as libc::c_uint
                                || bash_input.type_0 as libc::c_uint
                                    == st_stream as libc::c_int as libc::c_uint)
                        {
                            prompt_again();
                        }
                        current_block = 11101796020613380314;
                    }
                }
                _ => {
                    if shell_input_line_index == 0 as libc::c_int as size_t {
                        unquoted_backslash = 0 as libc::c_int;
                    }
                    uc = *shell_input_line.offset(shell_input_line_index as isize) as libc::c_uchar;
                    if uc != 0 {
                        unquoted_backslash = (unquoted_backslash == 0 as libc::c_int
                            && uc as libc::c_int == '\\' as i32)
                            as libc::c_int;
                        shell_input_line_index = shell_input_line_index.wrapping_add(1);
                    }
                    if uc as libc::c_int == 0 as libc::c_int
                        && !pushed_string_list.is_null()
                        && (*pushed_string_list).flags != 0x4 as libc::c_int
                        && (*pushed_string_list).flags != 0x2 as libc::c_int
                        && parser_state & 0x100000 as libc::c_int == 0 as libc::c_int
                        && parser_state & 0x200000 as libc::c_int == 0 as libc::c_int
                        && shell_input_line_index > 0 as libc::c_int as size_t
                        && *sh_syntaxtab.as_mut_ptr().offset(*shell_input_line.offset(
                            shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                as isize,
                        )
                            as libc::c_uchar
                            as isize)
                            & 0x2000 as libc::c_int
                            == 0 as libc::c_int
                        && *shell_input_line.offset(
                            shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                as isize,
                        ) as libc::c_int
                            != '\n' as i32
                        && unquoted_backslash == 0 as libc::c_int
                        && *sh_syntaxtab.as_mut_ptr().offset(*shell_input_line.offset(
                            shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                as isize,
                        )
                            as libc::c_uchar
                            as isize)
                            & 0x1 as libc::c_int
                            == 0 as libc::c_int
                        && ((if dstack.delimiter_depth != 0 {
                            *(dstack.delimiters)
                                .offset((dstack.delimiter_depth - 1 as libc::c_int) as isize)
                                as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) != '\'' as i32
                            && (if dstack.delimiter_depth != 0 {
                                *(dstack.delimiters)
                                    .offset((dstack.delimiter_depth - 1 as libc::c_int) as isize)
                                    as libc::c_int
                            } else {
                                0 as libc::c_int
                            }) != '"' as i32)
                    {
                        parser_state |= 0x200000 as libc::c_int;
                        return ' ' as i32;
                    }
                    loop {
                        if uc as libc::c_int == 0 as libc::c_int
                            && !pushed_string_list.is_null()
                            && (*pushed_string_list).flags != 0x4 as libc::c_int
                        {
                            parser_state &= !(0x200000 as libc::c_int);
                            pop_string();
                            uc = *shell_input_line.offset(shell_input_line_index as isize)
                                as libc::c_uchar;
                            if uc != 0 {
                                shell_input_line_index = shell_input_line_index.wrapping_add(1);
                            }
                        }
                        if uc as libc::c_int == '\\' as i32
                            && remove_quoted_newline != 0
                            && *shell_input_line.offset(shell_input_line_index as isize)
                                as libc::c_int
                                == '\n' as i32
                            && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                *shell_input_line_property.offset(
                                    shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                        as isize,
                                ) as libc::c_int
                            } else {
                                1 as libc::c_int
                            }) != 0
                        {
                            if interactive != 0
                                && (bash_input.type_0 as libc::c_uint
                                    == st_stdin as libc::c_int as libc::c_uint
                                    || bash_input.type_0 as libc::c_uint
                                        == st_stream as libc::c_int as libc::c_uint)
                            {
                                prompt_again();
                            }
                            line_number += 1;
                            if !pushed_string_list.is_null()
                                && !((*pushed_string_list).expander).is_null()
                                && *shell_input_line.offset(
                                    shell_input_line_index.wrapping_add(1 as libc::c_int as size_t)
                                        as isize,
                                ) as libc::c_int
                                    == '\0' as i32
                            {
                                uc = 0 as libc::c_int as libc::c_uchar;
                            } else {
                                if !(!pushed_string_list.is_null()
                                    && !((*pushed_string_list).expander).is_null()
                                    && *shell_input_line.offset(
                                        shell_input_line_index
                                            .wrapping_add(1 as libc::c_int as size_t)
                                            as isize,
                                    ) as libc::c_int
                                        != '\0' as i32)
                                {
                                    current_block = 11101796020613380314;
                                    break;
                                }
                                shell_input_line_index = shell_input_line_index.wrapping_add(1);
                                current_block = 9354102534551541821;
                                break;
                            }
                        } else {
                            if uc as libc::c_int == 0 as libc::c_int
                                && shell_input_line_terminator == -(1 as libc::c_int)
                            {
                                return if shell_input_line_index != 0 as libc::c_int as size_t {
                                    '\n' as i32
                                } else {
                                    -(1 as libc::c_int)
                                };
                            }
                            if !(uc as libc::c_int == 0 as libc::c_int
                                && bash_input.type_0 as libc::c_uint
                                    == st_string as libc::c_int as libc::c_uint
                                && *bash_input.location.string as libc::c_int != 0
                                && !pushed_string_list.is_null()
                                && (*pushed_string_list).flags == 0x4 as libc::c_int
                                && shell_input_line_terminator == 0 as libc::c_int)
                            {
                                break '_next_alias_char;
                            }
                            shell_input_line_index = 0 as libc::c_int as size_t;
                            current_block = 11101796020613380314;
                            break;
                        }
                    }
                }
            }
        }

        return uc as libc::c_int;
    }
}
fn shell_ungetc(mut c: libc::c_int) {
    unsafe {
        if !shell_input_line.is_null() && shell_input_line_index != 0 {
            shell_input_line_index = shell_input_line_index.wrapping_sub(1);
            *shell_input_line.offset(shell_input_line_index as isize) = c as libc::c_char;
        } else {
            eol_ungetc_lookahead = c;
        };
    }
}
#[no_mangle]
pub fn parser_remaining_input() -> *mut libc::c_char {
    unsafe {
        if shell_input_line.is_null() {
            return 0 as *mut libc::c_char;
        }
        if (shell_input_line_index as libc::c_int) < 0 as libc::c_int
            || shell_input_line_index >= shell_input_line_len
        {
            return b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        return shell_input_line.offset(shell_input_line_index as isize);
    }
}
fn discard_until(mut character: libc::c_int) {
    let mut c: libc::c_int = 0;
    loop {
        c = shell_getc(0 as libc::c_int);
        if !(c != -(1 as libc::c_int) && c != character) {
            break;
        }
    }
    if c != -(1 as libc::c_int) {
        shell_ungetc(c);
    }
}
#[no_mangle]
pub fn execute_variable_command(mut command: *mut libc::c_char, mut vname: *mut libc::c_char) {
    unsafe {
        let mut last_lastarg: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ps: sh_parser_state_t = _sh_parser_state_t {
            parser_state: 0,
            token_state: 0 as *mut libc::c_int,
            token: 0 as *mut libc::c_char,
            token_buffer_size: 0,
            input_line_terminator: 0,
            eof_encountered: 0,
            prompt_string_pointer: 0 as *mut *mut libc::c_char,
            current_command_line_count: 0,
            remember_on_history: 0,
            history_expansion_inhibited: 0,
            last_command_exit_value: 0,
            pipestatus: 0 as *mut ARRAY,
            last_shell_builtin: None,
            this_shell_builtin: None,
            expand_aliases: 0,
            echo_input_at_read: 0,
            need_here_doc: 0,
            here_doc_first_line: 0,
            redir_stack: [0 as *mut REDIRECT; 16],
        };
        save_parser_state(&mut ps);
        last_lastarg = get_string_value(b"_\0" as *const u8 as *const libc::c_char);
        if !last_lastarg.is_null() {
            last_lastarg = strcpy(
                malloc(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(libc::strlen(last_lastarg) as u64)
                        as usize,
                ) as *mut libc::c_char,
                last_lastarg,
            );
        }

        parse_and_execute(
            strcpy(
                malloc(
                    (1 as libc::c_int as libc::c_ulong).wrapping_add(libc::strlen(command) as u64)
                        as usize,
                ) as *mut libc::c_char,
                command,
            ),
            vname,
            0x1 as libc::c_int | 0x4 as libc::c_int,
        );

        restore_parser_state(&mut ps);
        bind_variable(
            b"_\0" as *const u8 as *const libc::c_char,
            last_lastarg,
            0 as libc::c_int,
        );

        if !last_lastarg.is_null() {
            free(last_lastarg as *mut libc::c_void);
        }
        last_lastarg = 0 as *mut libc::c_char;
        if token_to_read == '\n' as i32 {
            token_to_read = 0 as libc::c_int;
        }
    }
}
#[no_mangle]
pub fn push_token(mut x: libc::c_int) {
    unsafe {
        two_tokens_ago = token_before_that;
        token_before_that = last_read_token;
        last_read_token = current_token;
        current_token = x;
    }
}
static mut token: *mut libc::c_char =
    0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char;
static mut token_buffer_size: libc::c_int = 0;
fn yylex() -> libc::c_int {
    unsafe {
        if interactive != 0 && (current_token == 0 as libc::c_int || current_token == '\n' as i32) {
            if (prompt_string_pointer.is_null()
                || prompt_string_pointer == &mut ps1_prompt as *mut *mut libc::c_char)
                && parse_and_execute_level == 0 as libc::c_int
                && time_to_check_mail() != 0
            {
                check_mail();
                reset_mail_timer();
            }
            if token_to_read == 0 as libc::c_int
                && (interactive != 0
                    && (bash_input.type_0 as libc::c_uint
                        == st_stdin as libc::c_int as libc::c_uint
                        || bash_input.type_0 as libc::c_uint
                            == st_stream as libc::c_int as libc::c_uint))
            {
                prompt_again();
            }
        }
        two_tokens_ago = token_before_that;
        token_before_that = last_read_token;
        last_read_token = current_token;
        current_token = read_token(0 as libc::c_int);
        if parser_state & 0x8000 as libc::c_int != 0 && current_token == shell_eof_token {
            current_token = 304 as libc::c_int;
            if bash_input.type_0 as libc::c_uint == st_string as libc::c_int as libc::c_uint {
                rewind_input_string();
            }
        }
        parser_state &= !(0x8000 as libc::c_int);
        return current_token;
    }
}
static mut esacs_needed_count: libc::c_int = 0;
static mut expecting_in_token: libc::c_int = 0;
fn push_heredoc(mut r: *mut REDIRECT) {
    unsafe {
        if need_here_doc >= 16 as libc::c_int {
            ::core::ptr::write_volatile(
                &mut last_command_exit_value as *mut libc::c_int,
                2 as libc::c_int,
            );
            need_here_doc = 0 as libc::c_int;
            report_syntax_error(c_dcgettext(
                0 as *const libc::c_char,
                b"maximum here-document count exceeded\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ));
            reset_parser();
            exit_shell(last_command_exit_value);
        }

        let fresh12 = need_here_doc;
        need_here_doc = need_here_doc + 1;
        redir_stack[fresh12 as usize] = r;
    }
}
#[no_mangle]
pub fn gather_here_documents() {
    unsafe {
        let mut r: libc::c_int = 0;
        r = 0 as libc::c_int;

        here_doc_first_line = 1 as libc::c_int;
        while need_here_doc > 0 as libc::c_int {
            parser_state |= 0x20000 as libc::c_int;
            let fresh13 = r;
            r = r + 1;
            make_here_document(redir_stack[fresh13 as usize], line_number);
            parser_state &= !(0x20000 as libc::c_int);
            need_here_doc -= 1;
            redir_stack[(r - 1 as libc::c_int) as usize] = 0 as *mut REDIRECT;
        }
        here_doc_first_line = 0 as libc::c_int;
    }
}
static mut open_brace_count: libc::c_int = 0;
fn mk_alexpansion(mut s: *mut libc::c_char) -> *mut libc::c_char {
    unsafe {
        let mut l: libc::c_int = 0;
        let mut r: *mut libc::c_char = 0 as *mut libc::c_char;

        l = libc::strlen(s) as libc::c_int;
        r = malloc((l + 2 as libc::c_int) as usize) as *mut libc::c_char;
        strcpy(r, s);

        *r.offset(l as isize) = '\0' as i32 as libc::c_char;

        return r;
    }
}
fn alias_expand_token(mut tokstr: *mut libc::c_char) -> libc::c_int {
    unsafe {
        let mut expanded: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ap: *mut alias_t = 0 as *mut alias_t;
        if (parser_state & 0x2 as libc::c_int != 0
            || (last_read_token == 282 as libc::c_int
                || parser_state & 0x80000 as libc::c_int != 0
                    && (last_read_token == '<' as i32
                        || last_read_token == '>' as i32
                        || last_read_token == 290 as libc::c_int
                        || last_read_token == 302 as libc::c_int
                        || last_read_token == 301 as libc::c_int
                        || last_read_token == 298 as libc::c_int
                        || last_read_token == 291 as libc::c_int
                        || last_read_token == 293 as libc::c_int
                        || last_read_token == 292 as libc::c_int
                        || last_read_token == 294 as libc::c_int
                        || last_read_token == 299 as libc::c_int)
                        as libc::c_int
                        == 0 as libc::c_int
                || last_read_token != 295 as libc::c_int
                    && last_read_token != 296 as libc::c_int
                    && last_read_token != 297 as libc::c_int
                    && reserved_word_acceptable(last_read_token) != 0))
            && parser_state & 0x1 as libc::c_int == 0 as libc::c_int
        {
            ap = find_alias(tokstr);
            if !ap.is_null() && (*ap).flags as libc::c_int & 0x2 as libc::c_int != 0 {
                return -(100 as libc::c_int);
            }
            expanded = if !ap.is_null() {
                mk_alexpansion((*ap).value)
            } else {
                0 as *mut libc::c_void as *mut libc::c_char
            };
            if !expanded.is_null() {
                push_string(
                    expanded,
                    (*ap).flags as libc::c_int & 0x1 as libc::c_int,
                    ap,
                );
                return -(99 as libc::c_int);
            } else {
                return -(100 as libc::c_int);
            }
        }
        return -(100 as libc::c_int);
    }
}
fn time_command_acceptable() -> libc::c_int {
    unsafe {
        let mut i: libc::c_int = 0;

        if posixly_correct != 0 && shell_compatibility_level > 41 as libc::c_int {
            i = shell_input_line_index as libc::c_int;
            while (i as size_t) < shell_input_line_len
                && (*shell_input_line.offset(i as isize) as libc::c_int == ' ' as i32
                    || *shell_input_line.offset(i as isize) as libc::c_int == '\t' as i32)
            {
                i += 1;
            }
            if *shell_input_line.offset(i as isize) as libc::c_int == '-' as i32 {
                return 0 as libc::c_int;
            }
        }

        match last_read_token {
            0 | 59 | 10 => {
                if token_before_that == '|' as i32 {
                    return 0 as libc::c_int;
                }
            }
            288 | 289 | 38 | 267 | 269 | 268 | 258 | 259 | 261 | 260 | 123 | 40 | 41 | 277
            | 278 | 279 | 280 => {}
            _ => return 0 as libc::c_int,
        }
        return 1 as libc::c_int;
    }
}
fn special_case_tokens(mut tokstr: *mut libc::c_char) -> libc::c_int {
    unsafe {
        if last_read_token == 281 as libc::c_int
            && (token_before_that == 265 as libc::c_int
                || token_before_that == 263 as libc::c_int
                || token_before_that == 266 as libc::c_int)
            && (*tokstr.offset(0 as libc::c_int as isize) as libc::c_int == 'i' as i32
                && *tokstr.offset(1 as libc::c_int as isize) as libc::c_int == 'n' as i32
                && *tokstr.offset(2 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int)
        {
            if token_before_that == 263 as libc::c_int {
                parser_state |= 0x1 as libc::c_int;
                esacs_needed_count += 1;
            }
            if expecting_in_token != 0 {
                expecting_in_token -= 1;
            }
            return 276 as libc::c_int;
        }

        if expecting_in_token != 0
            && (last_read_token == 281 as libc::c_int || last_read_token == '\n' as i32)
            && (*tokstr.offset(0 as libc::c_int as isize) as libc::c_int == 'i' as i32
                && *tokstr.offset(1 as libc::c_int as isize) as libc::c_int == 'n' as i32
                && *tokstr.offset(2 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int)
        {
            if parser_state & 0x80 as libc::c_int != 0 {
                parser_state |= 0x1 as libc::c_int;
                esacs_needed_count += 1;
            }
            expecting_in_token -= 1;
            return 276 as libc::c_int;
        } else if expecting_in_token != 0
            && (last_read_token == '\n' as i32 || last_read_token == ';' as i32)
            && (*tokstr.offset(0 as libc::c_int as isize) as libc::c_int == 'd' as i32
                && *tokstr.offset(1 as libc::c_int as isize) as libc::c_int == 'o' as i32
                && *tokstr.offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32)
        {
            expecting_in_token -= 1;
            return 269 as libc::c_int;
        }
        if last_read_token == 281 as libc::c_int
            && (token_before_that == 265 as libc::c_int || token_before_that == 266 as libc::c_int)
            && (*tokstr.offset(0 as libc::c_int as isize) as libc::c_int == 'd' as i32
                && *tokstr.offset(1 as libc::c_int as isize) as libc::c_int == 'o' as i32
                && *tokstr.offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32)
        {
            if expecting_in_token != 0 {
                expecting_in_token -= 1;
            }
            return 269 as libc::c_int;
        }
        if esacs_needed_count != 0 {
            if last_read_token == 276 as libc::c_int
                && (*tokstr.offset(0 as libc::c_int as isize) as libc::c_int
                    == (*::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"esac\0"))
                        [0 as libc::c_int as usize] as libc::c_int
                    && strcmp(tokstr, b"esac\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int)
            {
                esacs_needed_count -= 1;
                parser_state &= !(0x1 as libc::c_int);
                return 264 as libc::c_int;
            }
        }
        if parser_state & 0x4 as libc::c_int != 0 {
            parser_state &= !(0x4 as libc::c_int);
            if *tokstr.offset(0 as libc::c_int as isize) as libc::c_int == '{' as i32
                && *tokstr.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
            {
                open_brace_count += 1;
                function_bstart = line_number;
                return '{' as i32;
            }
        }

        if last_read_token == 286 as libc::c_int
            && *tokstr.offset(0 as libc::c_int as isize) as libc::c_int == 'd' as i32
            && *tokstr.offset(1 as libc::c_int as isize) as libc::c_int == 'o' as i32
            && *tokstr.offset(2 as libc::c_int as isize) == 0
        {
            return 269 as libc::c_int;
        }
        if last_read_token == 286 as libc::c_int
            && *tokstr.offset(0 as libc::c_int as isize) as libc::c_int == '{' as i32
            && *tokstr.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {
            open_brace_count += 1;

            return '{' as i32;
        }
        if open_brace_count != 0
            && reserved_word_acceptable(last_read_token) != 0
            && *tokstr.offset(0 as libc::c_int as isize) as libc::c_int == '}' as i32
            && *tokstr.offset(1 as libc::c_int as isize) == 0
        {
            open_brace_count -= 1;

            return '}' as i32;
        }
        if last_read_token == 278 as libc::c_int
            && *tokstr.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
            && *tokstr.offset(1 as libc::c_int as isize) as libc::c_int == 'p' as i32
            && *tokstr.offset(2 as libc::c_int as isize) == 0
        {
            return 279 as libc::c_int;
        }
        if last_read_token == 278 as libc::c_int
            && *tokstr.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
            && *tokstr.offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32
            && *tokstr.offset(2 as libc::c_int as isize) == 0
        {
            return 280 as libc::c_int;
        }
        if last_read_token == 279 as libc::c_int
            && *tokstr.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
            && *tokstr.offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32
            && *tokstr.offset(2 as libc::c_int as isize) == 0
        {
            return 280 as libc::c_int;
        }
        if parser_state & 0x200 as libc::c_int != 0
            && *tokstr.offset(0 as libc::c_int as isize) as libc::c_int == ']' as i32
            && *tokstr.offset(1 as libc::c_int as isize) as libc::c_int == ']' as i32
            && *tokstr.offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {
            return 274 as libc::c_int;
        }
        return -(1 as libc::c_int);
    }
}
#[no_mangle]
pub fn reset_parser() {
    unsafe {
        dstack.delimiter_depth = 0 as libc::c_int;
        open_brace_count = 0 as libc::c_int;
        if parser_state & 0x1000 as libc::c_int != 0 {
            extended_glob = global_extglob;
        }
        parser_state = 0 as libc::c_int;
        here_doc_first_line = 0 as libc::c_int;
        if !pushed_string_list.is_null() {
            free_string_list();
        }
        if !shell_input_line.is_null() {
            free(shell_input_line as *mut libc::c_void);
            shell_input_line = 0 as *mut libc::c_void as *mut libc::c_char;
            shell_input_line_index = 0 as libc::c_int as size_t;
            shell_input_line_size = shell_input_line_index;
        }
        if !word_desc_to_read.is_null() {
            free(word_desc_to_read as *mut libc::c_void);
        }
        word_desc_to_read = 0 as *mut WORD_DESC;
        word_desc_to_read = 0 as *mut libc::c_void as *mut WORD_DESC;
        eol_ungetc_lookahead = 0 as libc::c_int;
        current_token = '\n' as i32;
        last_read_token = '\n' as i32;
        token_to_read = '\n' as i32;
    }
}
#[no_mangle]
pub fn reset_readahead_token() {
    unsafe {
        if token_to_read == '\n' as i32 {
            token_to_read = 0 as libc::c_int;
        }
    }
}
fn read_token(mut command: libc::c_int) -> libc::c_int {
    unsafe {
        let mut character: libc::c_int = 0;
        let mut peek_char: libc::c_int = 0;
        let mut result: libc::c_int = 0;
        if command == 1 as libc::c_int {
            reset_parser();
            return '\n' as i32;
        }

        if token_to_read != 0 {
            result = token_to_read;
            if token_to_read == 281 as libc::c_int || token_to_read == 282 as libc::c_int {
                yylval.word = word_desc_to_read;
                word_desc_to_read = 0 as *mut libc::c_void as *mut WORD_DESC;
            }
            token_to_read = 0 as libc::c_int;
            return result;
        }
        if parser_state & (0x100 as libc::c_int | 0x200 as libc::c_int) == 0x100 as libc::c_int {
            cond_lineno = line_number;
            parser_state |= 0x200 as libc::c_int;
            yylval.command = parse_cond_command();
            if cond_token != 274 as libc::c_int {
                cond_error();
                return -(1 as libc::c_int);
            }
            token_to_read = 274 as libc::c_int;
            parser_state &= !(0x200 as libc::c_int | 0x100 as libc::c_int);
            return 287 as libc::c_int;
        }

        loop {
            loop {
                character = shell_getc(1 as libc::c_int);
                if !(character != -(1 as libc::c_int)
                    && *sh_syntaxtab
                        .as_mut_ptr()
                        .offset(character as libc::c_uchar as isize)
                        & 0x2000 as libc::c_int
                        != 0)
                {
                    break;
                }
            }
            if character == -(1 as libc::c_int) {
                EOF_Reached = 1 as libc::c_int;
                return 304 as libc::c_int;
            }
            if character == '\0' as i32
                && bash_input.type_0 as libc::c_uint == st_string as libc::c_int as libc::c_uint
                && (!pushed_string_list.is_null() && !((*pushed_string_list).expander).is_null())
                    as libc::c_int
                    == 0 as libc::c_int
            {
                EOF_Reached = 1 as libc::c_int;
                return 304 as libc::c_int;
            }
            if character == '#' as i32
                && (interactive == 0 || interactive_comments != 0)
                && (if shell_input_line_index > 1 as libc::c_int as size_t {
                    *shell_input_line_property.offset(
                        shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t) as isize,
                    ) as libc::c_int
                } else {
                    1 as libc::c_int
                }) != 0
            {
                parser_state |= 0x100000 as libc::c_int;
                discard_until('\n' as i32);
                shell_getc(0 as libc::c_int);
                parser_state &= !(0x100000 as libc::c_int);
                character = '\n' as i32;
            }
            if character == '\n' as i32 {
                if need_here_doc != 0 {
                    gather_here_documents();
                }

                parser_state &= !(0x2 as libc::c_int);
                parser_state &= !(0x4000 as libc::c_int);

                return character;
            }
            if !(parser_state & 0x10000 as libc::c_int != 0) {
                if *sh_syntaxtab
                    .as_mut_ptr()
                    .offset(character as libc::c_uchar as isize)
                    & 0x1 as libc::c_int
                    != 0
                    && parser_state & 0x10 as libc::c_int == 0 as libc::c_int
                    && (if shell_input_line_index > 1 as libc::c_int as size_t {
                        *shell_input_line_property.offset(
                            shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                as isize,
                        ) as libc::c_int
                    } else {
                        1 as libc::c_int
                    }) != 0
                {
                    if character == '<' as i32 || character == '>' as i32 {
                        parser_state &= !(0x2 as libc::c_int);
                    }
                    parser_state &= !(0x4000 as libc::c_int);
                    if parser_state & 0x40 as libc::c_int != 0 && character == shell_eof_token {
                        peek_char = shell_getc(0 as libc::c_int);
                    } else {
                        peek_char = shell_getc(1 as libc::c_int);
                    }
                    if character == peek_char {
                        match character {
                            60 => {
                                peek_char = shell_getc(1 as libc::c_int);
                                if peek_char == '-' as i32
                                    && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                        *shell_input_line_property.offset(
                                            shell_input_line_index
                                                .wrapping_sub(1 as libc::c_int as size_t)
                                                as isize,
                                        ) as libc::c_int
                                    } else {
                                        1 as libc::c_int
                                    }) != 0
                                {
                                    return 298 as libc::c_int;
                                } else if peek_char == '<' as i32
                                    && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                        *shell_input_line_property.offset(
                                            shell_input_line_index
                                                .wrapping_sub(1 as libc::c_int as size_t)
                                                as isize,
                                        ) as libc::c_int
                                    } else {
                                        1 as libc::c_int
                                    }) != 0
                                {
                                    return 293 as libc::c_int;
                                } else {
                                    shell_ungetc(peek_char);
                                    return 291 as libc::c_int;
                                }
                            }
                            62 => return 290 as libc::c_int,
                            59 => {
                                parser_state |= 0x1 as libc::c_int;
                                parser_state &= !(0x2 as libc::c_int);

                                peek_char = shell_getc(1 as libc::c_int);
                                if peek_char == '&' as i32
                                    && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                        *shell_input_line_property.offset(
                                            shell_input_line_index
                                                .wrapping_sub(1 as libc::c_int as size_t)
                                                as isize,
                                        ) as libc::c_int
                                    } else {
                                        1 as libc::c_int
                                    }) != 0
                                {
                                    return 297 as libc::c_int;
                                } else {
                                    shell_ungetc(peek_char);
                                    return 295 as libc::c_int;
                                }
                            }
                            38 => return 288 as libc::c_int,
                            124 => return 289 as libc::c_int,
                            40 => {
                                result = parse_dparen(character);
                                if !(result == -(2 as libc::c_int)) {
                                    return result;
                                }
                            }
                            _ => {}
                        }
                    } else if character == '<' as i32
                        && peek_char == '&' as i32
                        && (if shell_input_line_index > 1 as libc::c_int as size_t {
                            *shell_input_line_property.offset(
                                shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                    as isize,
                            ) as libc::c_int
                        } else {
                            1 as libc::c_int
                        }) != 0
                    {
                        return 292 as libc::c_int;
                    } else if character == '>' as i32
                        && peek_char == '&' as i32
                        && (if shell_input_line_index > 1 as libc::c_int as size_t {
                            *shell_input_line_property.offset(
                                shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                    as isize,
                            ) as libc::c_int
                        } else {
                            1 as libc::c_int
                        }) != 0
                    {
                        return 294 as libc::c_int;
                    } else if character == '<' as i32
                        && peek_char == '>' as i32
                        && (if shell_input_line_index > 1 as libc::c_int as size_t {
                            *shell_input_line_property.offset(
                                shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                    as isize,
                            ) as libc::c_int
                        } else {
                            1 as libc::c_int
                        }) != 0
                    {
                        return 301 as libc::c_int;
                    } else if character == '>' as i32
                        && peek_char == '|' as i32
                        && (if shell_input_line_index > 1 as libc::c_int as size_t {
                            *shell_input_line_property.offset(
                                shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                    as isize,
                            ) as libc::c_int
                        } else {
                            1 as libc::c_int
                        }) != 0
                    {
                        return 302 as libc::c_int;
                    } else if character == '&' as i32
                        && peek_char == '>' as i32
                        && (if shell_input_line_index > 1 as libc::c_int as size_t {
                            *shell_input_line_property.offset(
                                shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                    as isize,
                            ) as libc::c_int
                        } else {
                            1 as libc::c_int
                        }) != 0
                    {
                        peek_char = shell_getc(1 as libc::c_int);
                        if peek_char == '>' as i32
                            && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                *shell_input_line_property.offset(
                                    shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                        as isize,
                                ) as libc::c_int
                            } else {
                                1 as libc::c_int
                            }) != 0
                        {
                            return 300 as libc::c_int;
                        } else {
                            shell_ungetc(peek_char);
                            return 299 as libc::c_int;
                        }
                    } else if character == '|' as i32
                        && peek_char == '&' as i32
                        && (if shell_input_line_index > 1 as libc::c_int as size_t {
                            *shell_input_line_property.offset(
                                shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                    as isize,
                            ) as libc::c_int
                        } else {
                            1 as libc::c_int
                        }) != 0
                    {
                        return 303 as libc::c_int;
                    } else if character == ';' as i32
                        && peek_char == '&' as i32
                        && (if shell_input_line_index > 1 as libc::c_int as size_t {
                            *shell_input_line_property.offset(
                                shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                    as isize,
                            ) as libc::c_int
                        } else {
                            1 as libc::c_int
                        }) != 0
                    {
                        parser_state |= 0x1 as libc::c_int;
                        parser_state &= !(0x2 as libc::c_int);
                        return 296 as libc::c_int;
                    }
                    shell_ungetc(peek_char);
                    if character == ')' as i32
                        && last_read_token == '(' as i32
                        && token_before_that == 281 as libc::c_int
                        && (if shell_input_line_index > 1 as libc::c_int as size_t {
                            *shell_input_line_property.offset(
                                shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                    as isize,
                            ) as libc::c_int
                        } else {
                            1 as libc::c_int
                        }) != 0
                    {
                        parser_state |= 0x4 as libc::c_int;
                        parser_state &= !(0x2 as libc::c_int);
                        function_dstart = line_number;
                    }
                    if character == '(' as i32
                        && parser_state & 0x1 as libc::c_int == 0 as libc::c_int
                        && (if shell_input_line_index > 1 as libc::c_int as size_t {
                            *shell_input_line_property.offset(
                                shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                    as isize,
                            ) as libc::c_int
                        } else {
                            1 as libc::c_int
                        }) != 0
                    {
                        parser_state |= 0x20 as libc::c_int;
                    } else if parser_state & 0x1 as libc::c_int != 0
                        && character == ')' as i32
                        && (if shell_input_line_index > 1 as libc::c_int as size_t {
                            *shell_input_line_property.offset(
                                shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                    as isize,
                            ) as libc::c_int
                        } else {
                            1 as libc::c_int
                        }) != 0
                    {
                        parser_state &= !(0x1 as libc::c_int);
                    } else if parser_state & 0x20 as libc::c_int != 0
                        && character == ')' as i32
                        && (if shell_input_line_index > 1 as libc::c_int as size_t {
                            *shell_input_line_property.offset(
                                shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                    as isize,
                            ) as libc::c_int
                        } else {
                            1 as libc::c_int
                        }) != 0
                    {
                        parser_state &= !(0x20 as libc::c_int);
                    }
                    if (character != '>' as i32 && character != '<' as i32
                        || peek_char != '(' as i32)
                        && (if shell_input_line_index > 1 as libc::c_int as size_t {
                            *shell_input_line_property.offset(
                                shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                    as isize,
                            ) as libc::c_int
                        } else {
                            1 as libc::c_int
                        }) != 0
                    {
                        return character;
                    }
                }
                if character == '-' as i32
                    && (last_read_token == 292 as libc::c_int
                        || last_read_token == 294 as libc::c_int)
                    && (if shell_input_line_index > 1 as libc::c_int as size_t {
                        *shell_input_line_property.offset(
                            shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                as isize,
                        ) as libc::c_int
                    } else {
                        1 as libc::c_int
                    }) != 0
                {
                    return character;
                }
            }
            result = read_token_word(character);
            if !(result == -(99 as libc::c_int)) {
                break;
            }
        }
        return result;
    }
}
static mut matched_pair_error: libc::c_char = 0;
fn parse_matched_pair(
    mut qc: libc::c_int,
    mut open: libc::c_int,
    mut close: libc::c_int,
    mut lenp: *mut libc::c_int,
    mut flags: libc::c_int,
) -> *mut libc::c_char {
    unsafe {
        let mut current_block: u64;
        let mut count: libc::c_int = 0;
        let mut ch: libc::c_int = 0;
        let mut prevch: libc::c_int = 0;
        let mut tflags: libc::c_int = 0;
        let mut nestlen: libc::c_int = 0;
        let mut ttranslen: libc::c_int = 0;
        let mut start_lineno: libc::c_int = 0;
        let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut nestret: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ttrans: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut retind: libc::c_int = 0;
        let mut retsize: libc::c_int = 0;
        let mut rflags: libc::c_int = 0;
        let mut dolbrace_state: libc::c_int = 0;
        dolbrace_state = if flags & 0x40 as libc::c_int != 0 {
            0x1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        count = 1 as libc::c_int;
        tflags = 0 as libc::c_int;
        if flags & 0x8 as libc::c_int != 0
            && qc != '`' as i32
            && qc != '\'' as i32
            && qc != '"' as i32
            && flags & 0x4 as libc::c_int == 0 as libc::c_int
        {
            tflags |= 0x2 as libc::c_int;
        }
        rflags = if qc == '"' as i32 {
            0x4 as libc::c_int
        } else {
            flags & 0x4 as libc::c_int
        };
        retsize = 64 as libc::c_int;
        ret = malloc(retsize as usize) as *mut libc::c_char;
        retind = 0 as libc::c_int;
        start_lineno = line_number;
        ch = -(1 as libc::c_int);
        while count != 0 {
            prevch = ch;
            ch = shell_getc(
                (qc != '\'' as i32 && tflags & 0x8 as libc::c_int == 0 as libc::c_int)
                    as libc::c_int,
            );
            if ch == -(1 as libc::c_int) {
                free(ret as *mut libc::c_void);

                parser_error(
                    start_lineno,
                    c_dcgettext(
                        0 as *const libc::c_char,
                        b"unexpected EOF while looking for matching `%c'\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    close,
                );
                EOF_Reached = 1 as libc::c_int;
                return &mut matched_pair_error;
            }
            if ch == '\n' as i32
                && (interactive != 0
                    && (bash_input.type_0 as libc::c_uint
                        == st_stdin as libc::c_int as libc::c_uint
                        || bash_input.type_0 as libc::c_uint
                            == st_stream as libc::c_int as libc::c_uint))
            {
                prompt_again();
            }
            if tflags & 0x4 as libc::c_int != 0 {
                if retind + 1 as libc::c_int >= retsize {
                    while retind + 1 as libc::c_int >= retsize {
                        retsize += 64 as libc::c_int;
                    }
                    ret = realloc(ret as *mut libc::c_void, retsize as usize) as *mut libc::c_char;
                }

                let fresh14 = retind;
                retind = retind + 1;
                *ret.offset(fresh14 as isize) = ch as libc::c_char;

                if ch == '\n' as i32 {
                    tflags &= !(0x4 as libc::c_int);
                }
            } else {
                if tflags & 0x2 as libc::c_int != 0
                    && tflags & 0x4 as libc::c_int == 0 as libc::c_int
                    && ch == '#' as i32
                    && (retind == 0 as libc::c_int
                        || *ret.offset((retind - 1 as libc::c_int) as isize) as libc::c_int
                            == '\n' as i32
                        || *sh_syntaxtab
                            .as_mut_ptr()
                            .offset(*ret.offset((retind - 1 as libc::c_int) as isize)
                                as libc::c_uchar as isize)
                            & 0x2000 as libc::c_int
                            != 0)
                    && (if shell_input_line_index > 1 as libc::c_int as size_t {
                        *shell_input_line_property.offset(
                            shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                as isize,
                        ) as libc::c_int
                    } else {
                        1 as libc::c_int
                    }) != 0
                {
                    tflags |= 0x4 as libc::c_int;
                }
                if tflags & 0x8 as libc::c_int != 0 {
                    tflags &= !(0x8 as libc::c_int);
                    if qc != '\'' as i32 && ch == '\n' as i32 {
                        if retind > 0 as libc::c_int {
                            retind -= 1;
                        }
                    } else {
                        if retind + 2 as libc::c_int >= retsize {
                            while retind + 2 as libc::c_int >= retsize {
                                retsize += 64 as libc::c_int;
                            }
                            ret = realloc(ret as *mut libc::c_void, retsize as usize)
                                as *mut libc::c_char;
                        }
                        if ch == '\u{1}' as i32
                            && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                *shell_input_line_property.offset(
                                    shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                        as isize,
                                ) as libc::c_int
                            } else {
                                1 as libc::c_int
                            }) != 0
                        {
                            let fresh15 = retind;
                            retind = retind + 1;
                            *ret.offset(fresh15 as isize) = '\u{1}' as i32 as libc::c_char;
                        }

                        let fresh16 = retind;
                        retind = retind + 1;
                        *ret.offset(fresh16 as isize) = ch as libc::c_char;
                    }
                } else if parser_state & 0x40000 as libc::c_int != 0
                    && open == '\'' as i32
                    && (ch == '\u{1}' as i32 || ch == '\u{7f}' as i32)
                    && (if shell_input_line_index > 1 as libc::c_int as size_t {
                        *shell_input_line_property.offset(
                            shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                as isize,
                        ) as libc::c_int
                    } else {
                        1 as libc::c_int
                    }) != 0
                {
                    if retind + 1 as libc::c_int >= retsize {
                        while retind + 1 as libc::c_int >= retsize {
                            retsize += 64 as libc::c_int;
                        }
                        ret = realloc(ret as *mut libc::c_void, retsize as usize)
                            as *mut libc::c_char;
                    }

                    let fresh17 = retind;
                    retind = retind + 1;
                    *ret.offset(fresh17 as isize) = ch as libc::c_char;
                } else if (ch == '\u{1}' as i32 || ch == '\u{7f}' as i32)
                    && (if shell_input_line_index > 1 as libc::c_int as size_t {
                        *shell_input_line_property.offset(
                            shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                as isize,
                        ) as libc::c_int
                    } else {
                        1 as libc::c_int
                    }) != 0
                {
                    if retind + 2 as libc::c_int >= retsize {
                        while retind + 2 as libc::c_int >= retsize {
                            retsize += 64 as libc::c_int;
                        }
                        ret = realloc(ret as *mut libc::c_void, retsize as usize)
                            as *mut libc::c_char;
                    }

                    let fresh18 = retind;
                    retind = retind + 1;
                    *ret.offset(fresh18 as isize) = '\u{1}' as i32 as libc::c_char;
                    let fresh19 = retind;
                    retind = retind + 1;
                    *ret.offset(fresh19 as isize) = ch as libc::c_char;
                } else {
                    if ch == close
                        && (if shell_input_line_index > 1 as libc::c_int as size_t {
                            *shell_input_line_property.offset(
                                shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                    as isize,
                            ) as libc::c_int
                        } else {
                            1 as libc::c_int
                        }) != 0
                    {
                        count -= 1;
                    } else if open != close
                        && tflags & 0x1 as libc::c_int != 0
                        && open == '{' as i32
                        && ch == open
                        && (if shell_input_line_index > 1 as libc::c_int as size_t {
                            *shell_input_line_property.offset(
                                shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                    as isize,
                            ) as libc::c_int
                        } else {
                            1 as libc::c_int
                        }) != 0
                    {
                        count += 1;
                    } else if flags & 0x1 as libc::c_int == 0 as libc::c_int
                        && ch == open
                        && (if shell_input_line_index > 1 as libc::c_int as size_t {
                            *shell_input_line_property.offset(
                                shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                    as isize,
                            ) as libc::c_int
                        } else {
                            1 as libc::c_int
                        }) != 0
                    {
                        count += 1;
                    }
                    if retind + 1 as libc::c_int >= retsize {
                        while retind + 1 as libc::c_int >= retsize {
                            retsize += 64 as libc::c_int;
                        }
                        ret = realloc(ret as *mut libc::c_void, retsize as usize)
                            as *mut libc::c_char;
                    }

                    let fresh20 = retind;
                    retind = retind + 1;
                    *ret.offset(fresh20 as isize) = ch as libc::c_char;

                    if count == 0 as libc::c_int {
                        break;
                    }
                    if open == '\'' as i32 {
                        if flags & 0x2 as libc::c_int != 0
                            && ch == '\\' as i32
                            && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                *shell_input_line_property.offset(
                                    shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                        as isize,
                                ) as libc::c_int
                            } else {
                                1 as libc::c_int
                            }) != 0
                        {
                            tflags |= 0x8 as libc::c_int;
                        }
                    } else {
                        if ch == '\\' as i32
                            && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                *shell_input_line_property.offset(
                                    shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                        as isize,
                                ) as libc::c_int
                            } else {
                                1 as libc::c_int
                            }) != 0
                        {
                            tflags |= 0x8 as libc::c_int;
                        }
                        if flags & 0x40 as libc::c_int != 0 {
                            if dolbrace_state == 0x1 as libc::c_int
                                && ch == '%' as i32
                                && retind > 1 as libc::c_int
                                && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                    *shell_input_line_property.offset(
                                        shell_input_line_index
                                            .wrapping_sub(1 as libc::c_int as size_t)
                                            as isize,
                                    ) as libc::c_int
                                } else {
                                    1 as libc::c_int
                                }) != 0
                            {
                                dolbrace_state = 0x40 as libc::c_int;
                            } else if dolbrace_state == 0x1 as libc::c_int
                                && ch == '#' as i32
                                && retind > 1 as libc::c_int
                                && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                    *shell_input_line_property.offset(
                                        shell_input_line_index
                                            .wrapping_sub(1 as libc::c_int as size_t)
                                            as isize,
                                    ) as libc::c_int
                                } else {
                                    1 as libc::c_int
                                }) != 0
                            {
                                dolbrace_state = 0x40 as libc::c_int;
                            } else if dolbrace_state == 0x1 as libc::c_int
                                && ch == '/' as i32
                                && retind > 1 as libc::c_int
                                && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                    *shell_input_line_property.offset(
                                        shell_input_line_index
                                            .wrapping_sub(1 as libc::c_int as size_t)
                                            as isize,
                                    ) as libc::c_int
                                } else {
                                    1 as libc::c_int
                                }) != 0
                            {
                                dolbrace_state = 0x80 as libc::c_int;
                            } else if dolbrace_state == 0x1 as libc::c_int
                                && ch == '^' as i32
                                && retind > 1 as libc::c_int
                                && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                    *shell_input_line_property.offset(
                                        shell_input_line_index
                                            .wrapping_sub(1 as libc::c_int as size_t)
                                            as isize,
                                    ) as libc::c_int
                                } else {
                                    1 as libc::c_int
                                }) != 0
                            {
                                dolbrace_state = 0x40 as libc::c_int;
                            } else if dolbrace_state == 0x1 as libc::c_int
                                && ch == ',' as i32
                                && retind > 1 as libc::c_int
                                && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                    *shell_input_line_property.offset(
                                        shell_input_line_index
                                            .wrapping_sub(1 as libc::c_int as size_t)
                                            as isize,
                                    ) as libc::c_int
                                } else {
                                    1 as libc::c_int
                                }) != 0
                            {
                                dolbrace_state = 0x40 as libc::c_int;
                            } else if dolbrace_state == 0x1 as libc::c_int
                                && !(strchr(
                                    b"#%^,~:-=?+/\0" as *const u8 as *const libc::c_char,
                                    ch,
                                ))
                                .is_null()
                                && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                    *shell_input_line_property.offset(
                                        shell_input_line_index
                                            .wrapping_sub(1 as libc::c_int as size_t)
                                            as isize,
                                    ) as libc::c_int
                                } else {
                                    1 as libc::c_int
                                }) != 0
                            {
                                dolbrace_state = 0x2 as libc::c_int;
                            } else if dolbrace_state == 0x2 as libc::c_int
                                && (strchr(
                                    b"#%^,~:-=?+/\0" as *const u8 as *const libc::c_char,
                                    ch,
                                ))
                                .is_null()
                                && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                    *shell_input_line_property.offset(
                                        shell_input_line_index
                                            .wrapping_sub(1 as libc::c_int as size_t)
                                            as isize,
                                    ) as libc::c_int
                                } else {
                                    1 as libc::c_int
                                }) != 0
                            {
                                dolbrace_state = 0x4 as libc::c_int;
                            }
                        }
                        if posixly_correct != 0
                            && shell_compatibility_level > 41 as libc::c_int
                            && dolbrace_state != 0x40 as libc::c_int
                            && dolbrace_state != 0x80 as libc::c_int
                            && flags & 0x4 as libc::c_int != 0
                            && flags & 0x40 as libc::c_int != 0
                            && ch == '\'' as i32
                            && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                *shell_input_line_property.offset(
                                    shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                        as isize,
                                ) as libc::c_int
                            } else {
                                1 as libc::c_int
                            }) != 0
                        {
                            continue;
                        }
                        if open != close {
                            if *sh_syntaxtab
                                .as_mut_ptr()
                                .offset(ch as libc::c_uchar as isize)
                                & 0x8 as libc::c_int
                                != 0
                                && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                    *shell_input_line_property.offset(
                                        shell_input_line_index
                                            .wrapping_sub(1 as libc::c_int as size_t)
                                            as isize,
                                    ) as libc::c_int
                                } else {
                                    1 as libc::c_int
                                }) != 0
                            {
                                if dstack.delimiter_depth + 2 as libc::c_int
                                    > dstack.delimiter_space
                                {
                                    dstack.delimiter_space += 10 as libc::c_int;
                                    dstack.delimiters = realloc(
                                        dstack.delimiters as *mut libc::c_void,
                                        (dstack.delimiter_space as libc::c_ulong)
                                            .wrapping_mul(::core::mem::size_of::<libc::c_char>()
                                                as libc::c_ulong)
                                            as usize,
                                    )
                                        as *mut libc::c_char;
                                }
                                *(dstack.delimiters).offset(dstack.delimiter_depth as isize) =
                                    ch as libc::c_char;
                                dstack.delimiter_depth += 1;
                                dstack.delimiter_depth;
                                if tflags & 0x1 as libc::c_int != 0
                                    && ch == '\'' as i32
                                    && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                        *shell_input_line_property.offset(
                                            shell_input_line_index
                                                .wrapping_sub(1 as libc::c_int as size_t)
                                                as isize,
                                        ) as libc::c_int
                                    } else {
                                        1 as libc::c_int
                                    }) != 0
                                {
                                    nestret = parse_matched_pair(
                                        ch,
                                        ch,
                                        ch,
                                        &mut nestlen,
                                        0x2 as libc::c_int | rflags,
                                    );
                                } else {
                                    nestret = parse_matched_pair(ch, ch, ch, &mut nestlen, rflags);
                                }
                                dstack.delimiter_depth -= 1;
                                dstack.delimiter_depth;
                                if nestret == &mut matched_pair_error as *mut libc::c_char {
                                    free(ret as *mut libc::c_void);
                                    return &mut matched_pair_error;
                                }
                                if tflags & 0x1 as libc::c_int != 0
                                    && ch == '\'' as i32
                                    && (extended_quote != 0
                                        || rflags & 0x4 as libc::c_int == 0 as libc::c_int)
                                    && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                        *shell_input_line_property.offset(
                                            shell_input_line_index
                                                .wrapping_sub(1 as libc::c_int as size_t)
                                                as isize,
                                        ) as libc::c_int
                                    } else {
                                        1 as libc::c_int
                                    }) != 0
                                {
                                    ttrans = c_ansiexpand(
                                        nestret,
                                        0 as libc::c_int,
                                        nestlen - 1 as libc::c_int,
                                        &mut ttranslen,
                                    );
                                    free(nestret as *mut libc::c_void);
                                    if shell_compatibility_level > 42 as libc::c_int
                                        && rflags & 0x4 as libc::c_int != 0
                                        && dolbrace_state == 0x80 as libc::c_int
                                        && flags & 0x40 as libc::c_int != 0
                                    {
                                        nestret = c_sh_single_quote(ttrans);
                                        free(ttrans as *mut libc::c_void);
                                        nestlen = libc::strlen(nestret) as libc::c_int;
                                    } else if rflags & 0x4 as libc::c_int == 0 as libc::c_int {
                                        nestret = c_sh_single_quote(ttrans);
                                        free(ttrans as *mut libc::c_void);
                                        nestlen = libc::strlen(nestret) as libc::c_int;
                                    } else {
                                        nestret = ttrans;
                                        nestlen = ttranslen;
                                    }
                                    retind -= 2 as libc::c_int;
                                } else if tflags & 0x1 as libc::c_int != 0
                                    && ch == '"' as i32
                                    && (extended_quote != 0
                                        || rflags & 0x4 as libc::c_int == 0 as libc::c_int)
                                    && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                        *shell_input_line_property.offset(
                                            shell_input_line_index
                                                .wrapping_sub(1 as libc::c_int as size_t)
                                                as isize,
                                        ) as libc::c_int
                                    } else {
                                        1 as libc::c_int
                                    }) != 0
                                {
                                    ttrans = localeexpand(
                                        nestret,
                                        0 as libc::c_int,
                                        nestlen - 1 as libc::c_int,
                                        start_lineno,
                                        &mut ttranslen,
                                    );
                                    free(nestret as *mut libc::c_void);
                                    nestret =
                                        c_sh_mkdoublequoted(ttrans, ttranslen, 0 as libc::c_int);
                                    free(ttrans as *mut libc::c_void);
                                    nestlen = ttranslen + 2 as libc::c_int;
                                    retind -= 2 as libc::c_int;
                                }
                                if nestlen != 0 {
                                    if retind + nestlen >= retsize {
                                        while retind + nestlen >= retsize {
                                            retsize += 64 as libc::c_int;
                                        }
                                        ret = realloc(ret as *mut libc::c_void, retsize as usize)
                                            as *mut libc::c_char;
                                    }
                                    strcpy(ret.offset(retind as isize), nestret);
                                    retind += nestlen;
                                }
                                if !nestret.is_null() {
                                    free(nestret as *mut libc::c_void);
                                }
                                nestret = 0 as *mut libc::c_char;
                                current_block = 16813369756331276724;
                            } else if flags & (0x20 as libc::c_int | 0x40 as libc::c_int) != 0
                                && tflags & 0x1 as libc::c_int != 0
                                && (ch == '(' as i32 || ch == '{' as i32 || ch == '[' as i32)
                            {
                                current_block = 11507125668437329568;
                            } else if flags & (0x20 as libc::c_int | 0x40 as libc::c_int) != 0
                                && tflags & 0x1000 as libc::c_int != 0
                                && ch == '(' as i32
                            {
                                current_block = 11507125668437329568;
                            } else {
                                current_block = 16813369756331276724;
                            }
                        } else if open == '"' as i32
                            && ch == '`' as i32
                            && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                *shell_input_line_property.offset(
                                    shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                        as isize,
                                ) as libc::c_int
                            } else {
                                1 as libc::c_int
                            }) != 0
                        {
                            nestret = parse_matched_pair(
                                0 as libc::c_int,
                                '`' as i32,
                                '`' as i32,
                                &mut nestlen,
                                rflags,
                            );
                            if nestret == &mut matched_pair_error as *mut libc::c_char {
                                free(ret as *mut libc::c_void);
                                return &mut matched_pair_error;
                            }
                            if nestlen != 0 {
                                if retind + nestlen >= retsize {
                                    while retind + nestlen >= retsize {
                                        retsize += 64 as libc::c_int;
                                    }
                                    ret = realloc(ret as *mut libc::c_void, retsize as usize)
                                        as *mut libc::c_char;
                                }
                                strcpy(ret.offset(retind as isize), nestret);
                                retind += nestlen;
                            }
                            if !nestret.is_null() {
                                free(nestret as *mut libc::c_void);
                            }
                            nestret = 0 as *mut libc::c_char;
                            current_block = 16813369756331276724;
                        } else if open != '`' as i32
                            && tflags & 0x1 as libc::c_int != 0
                            && (ch == '(' as i32 || ch == '{' as i32 || ch == '[' as i32)
                            && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                *shell_input_line_property.offset(
                                    shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                        as isize,
                                ) as libc::c_int
                            } else {
                                1 as libc::c_int
                            }) != 0
                        {
                            current_block = 11507125668437329568;
                        } else {
                            current_block = 16813369756331276724;
                        }
                        match current_block {
                            11507125668437329568 => {
                                if open == ch {
                                    count -= 1;
                                }
                                if ch == '(' as i32 {
                                    nestret = parse_comsub(
                                        0 as libc::c_int,
                                        '(' as i32,
                                        ')' as i32,
                                        &mut nestlen,
                                        (rflags | 0x8 as libc::c_int) & !(0x4 as libc::c_int),
                                    );
                                } else if ch == '{' as i32 {
                                    nestret = parse_matched_pair(
                                        0 as libc::c_int,
                                        '{' as i32,
                                        '}' as i32,
                                        &mut nestlen,
                                        0x1 as libc::c_int | 0x40 as libc::c_int | rflags,
                                    );
                                } else if ch == '[' as i32 {
                                    nestret = parse_matched_pair(
                                        0 as libc::c_int,
                                        '[' as i32,
                                        ']' as i32,
                                        &mut nestlen,
                                        rflags,
                                    );
                                }
                                if nestret == &mut matched_pair_error as *mut libc::c_char {
                                    free(ret as *mut libc::c_void);
                                    return &mut matched_pair_error;
                                }
                                if nestlen != 0 {
                                    if retind + nestlen >= retsize {
                                        while retind + nestlen >= retsize {
                                            retsize += 64 as libc::c_int;
                                        }
                                        ret = realloc(ret as *mut libc::c_void, retsize as usize)
                                            as *mut libc::c_char;
                                    }
                                    strcpy(ret.offset(retind as isize), nestret);
                                    retind += nestlen;
                                }
                                if !nestret.is_null() {
                                    free(nestret as *mut libc::c_void);
                                }
                                nestret = 0 as *mut libc::c_char;
                            }
                            _ => {}
                        }
                        if (ch == '<' as i32 || ch == '>' as i32)
                            && tflags & 0x1000 as libc::c_int == 0 as libc::c_int
                            && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                *shell_input_line_property.offset(
                                    shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                        as isize,
                                ) as libc::c_int
                            } else {
                                1 as libc::c_int
                            }) != 0
                        {
                            tflags |= 0x1000 as libc::c_int;
                        } else {
                            tflags &= !(0x1000 as libc::c_int);
                        }
                        if ch == '$' as i32
                            && tflags & 0x1 as libc::c_int == 0 as libc::c_int
                            && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                *shell_input_line_property.offset(
                                    shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                        as isize,
                                ) as libc::c_int
                            } else {
                                1 as libc::c_int
                            }) != 0
                        {
                            tflags |= 0x1 as libc::c_int;
                        } else {
                            tflags &= !(0x1 as libc::c_int);
                        }
                    }
                }
            }
        }

        *ret.offset(retind as isize) = '\0' as i32 as libc::c_char;

        if !lenp.is_null() {
            *lenp = retind;
        }
        return ret;
    }
}
fn parse_comsub(
    mut qc: libc::c_int,
    mut open: libc::c_int,
    mut close: libc::c_int,
    mut lenp: *mut libc::c_int,
    mut flags: libc::c_int,
) -> *mut libc::c_char {
    unsafe {
        let mut current_block: u64;
        let mut count: libc::c_int = 0;
        let mut ch: libc::c_int = 0;
        let mut peekc: libc::c_int = 0;
        let mut tflags: libc::c_int = 0;
        let mut lex_rwlen: libc::c_int = 0;
        let mut lex_wlen: libc::c_int = 0;
        let mut lex_firstind: libc::c_int = 0;
        let mut nestlen: libc::c_int = 0;
        let mut ttranslen: libc::c_int = 0;
        let mut start_lineno: libc::c_int = 0;
        let mut orig_histexp: libc::c_int = 0;
        let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut nestret: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ttrans: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut heredelim: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut retind: libc::c_int = 0;
        let mut retsize: libc::c_int = 0;
        let mut rflags: libc::c_int = 0;
        let mut hdlen: libc::c_int = 0;
        peekc = shell_getc(0 as libc::c_int);
        shell_ungetc(peekc);
        if peekc == '(' as i32 {
            return parse_matched_pair(qc, open, close, lenp, 0 as libc::c_int);
        }
        count = 1 as libc::c_int;
        tflags = 0x10 as libc::c_int;
        orig_histexp = history_expansion_inhibited;
        if flags & 0x8 as libc::c_int != 0
            && qc != '\'' as i32
            && qc != '"' as i32
            && flags & 0x4 as libc::c_int == 0 as libc::c_int
        {
            tflags |= 0x20 as libc::c_int;
        }
        if tflags & 0x20 as libc::c_int != 0
            && (interactive == 0 as libc::c_int || interactive_comments != 0)
        {
            tflags |= 0x2 as libc::c_int;
        }
        rflags = flags & 0x4 as libc::c_int;
        retsize = 64 as libc::c_int;
        ret = malloc(retsize as usize) as *mut libc::c_char;
        retind = 0 as libc::c_int;
        start_lineno = line_number;
        lex_wlen = 0 as libc::c_int;
        lex_rwlen = lex_wlen;
        heredelim = 0 as *mut libc::c_char;
        lex_firstind = -(1 as libc::c_int);
        while count != 0 {
            ch = shell_getc(
                (qc != '\'' as i32
                    && tflags & (0x4 as libc::c_int | 0x8 as libc::c_int | 0x400 as libc::c_int)
                        == 0 as libc::c_int) as libc::c_int,
            );
            if !(ch == -(1 as libc::c_int)) {
                if ch == '\n' as i32 {
                    if tflags & 0x100 as libc::c_int != 0 && !heredelim.is_null() {
                        tflags &= !(0x100 as libc::c_int);
                        tflags |= 0x80 as libc::c_int;
                        history_expansion_inhibited = 1 as libc::c_int;
                        lex_firstind = retind + 1 as libc::c_int;
                    } else if tflags & 0x80 as libc::c_int != 0 {
                        let mut tind: libc::c_int = 0;
                        tind = lex_firstind;
                        while tflags & 0x200 as libc::c_int != 0
                            && *ret.offset(tind as isize) as libc::c_int == '\t' as i32
                        {
                            tind += 1;
                        }
                        if retind - tind == hdlen
                            && (if hdlen == 0 as libc::c_int {
                                1 as libc::c_int
                            } else {
                                (*ret.offset(tind as isize).offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == *heredelim.offset(0 as libc::c_int as isize) as libc::c_int
                                    && strncmp(
                                        ret.offset(tind as isize),
                                        heredelim,
                                        hdlen as usize,
                                    ) == 0 as libc::c_int)
                                    as libc::c_int
                            }) != 0
                        {
                            tflags &= !(0x200 as libc::c_int
                                | 0x80 as libc::c_int
                                | 0x400 as libc::c_int);
                            free(heredelim as *mut libc::c_void);
                            heredelim = 0 as *mut libc::c_char;
                            lex_firstind = -(1 as libc::c_int);
                            history_expansion_inhibited = orig_histexp;
                        } else {
                            lex_firstind = retind + 1 as libc::c_int;
                        }
                    }
                }
                if ch == '\n' as i32
                    && (interactive != 0
                        && (bash_input.type_0 as libc::c_uint
                            == st_stdin as libc::c_int as libc::c_uint
                            || bash_input.type_0 as libc::c_uint
                                == st_stream as libc::c_int as libc::c_uint))
                {
                    prompt_again();
                }
                if tflags & 0x80 as libc::c_int != 0 && ch == close && count == 1 as libc::c_int {
                    let mut tind_0: libc::c_int = 0;
                    tind_0 = lex_firstind;
                    while tflags & 0x200 as libc::c_int != 0
                        && *ret.offset(tind_0 as isize) as libc::c_int == '\t' as i32
                    {
                        tind_0 += 1;
                    }
                    if retind - tind_0 == hdlen
                        && (if hdlen == 0 as libc::c_int {
                            1 as libc::c_int
                        } else {
                            (*ret
                                .offset(tind_0 as isize)
                                .offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == *heredelim.offset(0 as libc::c_int as isize) as libc::c_int
                                && strncmp(ret.offset(tind_0 as isize), heredelim, hdlen as usize)
                                    == 0 as libc::c_int) as libc::c_int
                        }) != 0
                    {
                        tflags &=
                            !(0x200 as libc::c_int | 0x80 as libc::c_int | 0x400 as libc::c_int);
                        free(heredelim as *mut libc::c_void);
                        heredelim = 0 as *mut libc::c_char;
                        lex_firstind = -(1 as libc::c_int);
                        history_expansion_inhibited = orig_histexp;
                    }
                }
                if tflags & (0x4 as libc::c_int | 0x80 as libc::c_int) != 0 {
                    if retind + 1 as libc::c_int >= retsize {
                        while retind + 1 as libc::c_int >= retsize {
                            retsize += 64 as libc::c_int;
                        }
                        ret = realloc(ret as *mut libc::c_void, retsize as usize)
                            as *mut libc::c_char;
                    }

                    let fresh21 = retind;
                    retind = retind + 1;
                    *ret.offset(fresh21 as isize) = ch as libc::c_char;

                    if tflags & 0x4 as libc::c_int != 0 && ch == '\n' as i32 {
                        tflags &= !(0x4 as libc::c_int);
                    }
                    continue;
                } else if tflags & 0x8 as libc::c_int != 0 {
                    tflags &= !(0x8 as libc::c_int);
                    if qc != '\'' as i32 && ch == '\n' as i32 {
                        if retind > 0 as libc::c_int {
                            retind -= 1;
                        }
                        continue;
                    } else {
                        if retind + 2 as libc::c_int >= retsize {
                            while retind + 2 as libc::c_int >= retsize {
                                retsize += 64 as libc::c_int;
                            }
                            ret = realloc(ret as *mut libc::c_void, retsize as usize)
                                as *mut libc::c_char;
                        }
                        if ch == '\u{1}' as i32
                            && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                *shell_input_line_property.offset(
                                    shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                        as isize,
                                ) as libc::c_int
                            } else {
                                1 as libc::c_int
                            }) != 0
                        {
                            let fresh22 = retind;
                            retind = retind + 1;
                            *ret.offset(fresh22 as isize) = '\u{1}' as i32 as libc::c_char;
                        }

                        let fresh23 = retind;
                        retind = retind + 1;
                        *ret.offset(fresh23 as isize) = ch as libc::c_char;

                        continue;
                    }
                } else {
                    if *sh_syntaxtab
                        .as_mut_ptr()
                        .offset(ch as libc::c_uchar as isize)
                        & 0x2 as libc::c_int
                        != 0
                        && (if shell_input_line_index > 1 as libc::c_int as size_t {
                            *shell_input_line_property.offset(
                                shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                    as isize,
                            ) as libc::c_int
                        } else {
                            1 as libc::c_int
                        }) != 0
                    {
                        tflags &= !(0x800 as libc::c_int);
                    } else if tflags & 0x800 as libc::c_int != 0 {
                        lex_wlen += 1;
                    } else {
                        tflags |= 0x800 as libc::c_int;
                        lex_wlen = 0 as libc::c_int;
                        if tflags & 0x10 as libc::c_int != 0 {
                            lex_rwlen = 0 as libc::c_int;
                        }
                    }
                    if *sh_syntaxtab
                        .as_mut_ptr()
                        .offset(ch as libc::c_uchar as isize)
                        & 0x2000 as libc::c_int
                        != 0
                        && tflags & 0x100 as libc::c_int == 0 as libc::c_int
                        && lex_rwlen == 0 as libc::c_int
                        && (if shell_input_line_index > 1 as libc::c_int as size_t {
                            *shell_input_line_property.offset(
                                shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                    as isize,
                            ) as libc::c_int
                        } else {
                            1 as libc::c_int
                        }) != 0
                    {
                        if retind + 1 as libc::c_int >= retsize {
                            while retind + 1 as libc::c_int >= retsize {
                                retsize += 64 as libc::c_int;
                            }
                            ret = realloc(ret as *mut libc::c_void, retsize as usize)
                                as *mut libc::c_char;
                        }

                        let fresh24 = retind;
                        retind = retind + 1;
                        *ret.offset(fresh24 as isize) = ch as libc::c_char;

                        continue;
                    } else {
                        if tflags & 0x100 as libc::c_int != 0 {
                            if lex_firstind == -(1 as libc::c_int)
                                && *sh_syntaxtab
                                    .as_mut_ptr()
                                    .offset(ch as libc::c_uchar as isize)
                                    & 0x2 as libc::c_int
                                    == 0 as libc::c_int
                            {
                                lex_firstind = retind;
                            } else if lex_firstind >= 0 as libc::c_int
                                && tflags & 0x8 as libc::c_int == 0 as libc::c_int
                                && *sh_syntaxtab
                                    .as_mut_ptr()
                                    .offset(ch as libc::c_uchar as isize)
                                    & 0x2 as libc::c_int
                                    != 0
                            {
                                if heredelim.is_null() {
                                    nestret = substring(ret, lex_firstind, retind);
                                    heredelim = string_quote_removal(nestret, 0 as libc::c_int);
                                    hdlen = (if !heredelim.is_null()
                                        && *heredelim.offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            != 0
                                    {
                                        if *heredelim.offset(1 as libc::c_int as isize)
                                            as libc::c_int
                                            != 0
                                        {
                                            if *heredelim.offset(2 as libc::c_int as isize)
                                                as libc::c_int
                                                != 0
                                            {
                                                libc::strlen(heredelim) as usize
                                            } else {
                                                2 as libc::c_int as usize
                                            }
                                        } else {
                                            1 as libc::c_int as usize
                                        }
                                    } else {
                                        0 as libc::c_int as usize
                                    }) as libc::c_int;
                                    if (*heredelim.offset(0 as libc::c_int as isize) as libc::c_int
                                        == *nestret.offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                        && strcmp(heredelim, nestret) == 0 as libc::c_int)
                                        as libc::c_int
                                        == 0 as libc::c_int
                                    {
                                        tflags |= 0x400 as libc::c_int;
                                    }
                                    free(nestret as *mut libc::c_void);
                                }
                                if ch == '\n' as i32 {
                                    tflags |= 0x80 as libc::c_int;
                                    tflags &= !(0x100 as libc::c_int);
                                    lex_firstind = retind + 1 as libc::c_int;
                                    history_expansion_inhibited = 1 as libc::c_int;
                                } else {
                                    lex_firstind = -(1 as libc::c_int);
                                }
                            }
                        }
                        if tflags & 0x10 as libc::c_int == 0 as libc::c_int
                            && tflags & 0x20 as libc::c_int != 0
                            && tflags & 0x4 as libc::c_int == 0 as libc::c_int
                            && (*sh_syntaxtab
                                .as_mut_ptr()
                                .offset(ch as libc::c_uchar as isize)
                                & 0x1 as libc::c_int
                                != 0
                                || ch == '\n' as i32)
                            && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                *shell_input_line_property.offset(
                                    shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                        as isize,
                                ) as libc::c_int
                            } else {
                                1 as libc::c_int
                            }) != 0
                        {
                            if retind + 1 as libc::c_int >= retsize {
                                while retind + 1 as libc::c_int >= retsize {
                                    retsize += 64 as libc::c_int;
                                }
                                ret = realloc(ret as *mut libc::c_void, retsize as usize)
                                    as *mut libc::c_char;
                            }

                            let fresh25 = retind;
                            retind = retind + 1;
                            *ret.offset(fresh25 as isize) = ch as libc::c_char;

                            peekc = shell_getc(1 as libc::c_int);
                            if ch == peekc
                                && (ch == '&' as i32 || ch == '|' as i32 || ch == ';' as i32)
                            {
                                if retind + 1 as libc::c_int >= retsize {
                                    while retind + 1 as libc::c_int >= retsize {
                                        retsize += 64 as libc::c_int;
                                    }
                                    ret = realloc(ret as *mut libc::c_void, retsize as usize)
                                        as *mut libc::c_char;
                                }

                                let fresh26 = retind;
                                retind = retind + 1;
                                *ret.offset(fresh26 as isize) = peekc as libc::c_char;

                                tflags |= 0x10 as libc::c_int;
                                lex_rwlen = 0 as libc::c_int;
                                continue;
                            } else if ch == '\n' as i32
                                || (ch == ';' as i32 || ch == '&' as i32 || ch == '|' as i32)
                            {
                                shell_ungetc(peekc);
                                tflags |= 0x10 as libc::c_int;
                                lex_rwlen = 0 as libc::c_int;
                                continue;
                            } else if ch == -(1 as libc::c_int) {
                                current_block = 12717184829685720462;
                            } else {
                                retind -= 1;
                                shell_ungetc(peekc);
                                current_block = 7923086311623215889;
                            }
                        } else {
                            current_block = 7923086311623215889;
                        }
                        match current_block {
                            12717184829685720462 => {}
                            _ => {
                                if tflags & 0x10 as libc::c_int != 0 {
                                    if *(*c___ctype_b_loc())
                                        .offset(ch as libc::c_uchar as libc::c_int as isize)
                                        as libc::c_int
                                        & _ISlower as libc::c_int as libc::c_ushort as libc::c_int
                                        != 0
                                        && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                            *shell_input_line_property.offset(
                                                shell_input_line_index
                                                    .wrapping_sub(1 as libc::c_int as size_t)
                                                    as isize,
                                            )
                                                as libc::c_int
                                        } else {
                                            1 as libc::c_int
                                        }) != 0
                                    {
                                        if retind + 1 as libc::c_int >= retsize {
                                            while retind + 1 as libc::c_int >= retsize {
                                                retsize += 64 as libc::c_int;
                                            }
                                            ret =
                                                realloc(ret as *mut libc::c_void, retsize as usize)
                                                    as *mut libc::c_char;
                                        }

                                        let fresh27 = retind;
                                        retind = retind + 1;
                                        *ret.offset(fresh27 as isize) = ch as libc::c_char;

                                        lex_rwlen += 1;
                                        continue;
                                    } else if lex_rwlen == 4 as libc::c_int
                                        && *sh_syntaxtab
                                            .as_mut_ptr()
                                            .offset(ch as libc::c_uchar as isize)
                                            & 0x2 as libc::c_int
                                            != 0
                                        && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                            *shell_input_line_property.offset(
                                                shell_input_line_index
                                                    .wrapping_sub(1 as libc::c_int as size_t)
                                                    as isize,
                                            )
                                                as libc::c_int
                                        } else {
                                            1 as libc::c_int
                                        }) != 0
                                    {
                                        if if 4 as libc::c_int == 0 as libc::c_int {
                                            1 as libc::c_int
                                        } else {
                                            (*ret
                                                .offset(retind as isize)
                                                .offset(-(4 as libc::c_int as isize))
                                                .offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                == (*::core::mem::transmute::<
                                                    &[u8; 5],
                                                    &[libc::c_char; 5],
                                                >(
                                                    b"case\0"
                                                ))
                                                    [0 as libc::c_int as usize]
                                                    as libc::c_int
                                                && strncmp(
                                                    ret.offset(retind as isize)
                                                        .offset(-(4 as libc::c_int as isize)),
                                                    b"case\0" as *const u8 as *const libc::c_char,
                                                    4 as libc::c_int as usize,
                                                ) == 0 as libc::c_int)
                                                as libc::c_int
                                        } != 0
                                        {
                                            tflags |= 0x40 as libc::c_int;
                                            tflags &= !(0x10 as libc::c_int);
                                        } else if if 4 as libc::c_int == 0 as libc::c_int {
                                            1 as libc::c_int
                                        } else {
                                            (*ret
                                                .offset(retind as isize)
                                                .offset(-(4 as libc::c_int as isize))
                                                .offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                == (*::core::mem::transmute::<
                                                    &[u8; 5],
                                                    &[libc::c_char; 5],
                                                >(
                                                    b"esac\0"
                                                ))
                                                    [0 as libc::c_int as usize]
                                                    as libc::c_int
                                                && strncmp(
                                                    ret.offset(retind as isize)
                                                        .offset(-(4 as libc::c_int as isize)),
                                                    b"esac\0" as *const u8 as *const libc::c_char,
                                                    4 as libc::c_int as usize,
                                                ) == 0 as libc::c_int)
                                                as libc::c_int
                                        } != 0
                                        {
                                            tflags &= !(0x40 as libc::c_int);
                                            tflags |= 0x10 as libc::c_int;
                                            lex_rwlen = 0 as libc::c_int;
                                        } else if (if 4 as libc::c_int == 0 as libc::c_int {
                                            1 as libc::c_int
                                        } else {
                                            (*ret
                                                .offset(retind as isize)
                                                .offset(-(4 as libc::c_int as isize))
                                                .offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                == (*::core::mem::transmute::<
                                                    &[u8; 5],
                                                    &[libc::c_char; 5],
                                                >(
                                                    b"done\0"
                                                ))
                                                    [0 as libc::c_int as usize]
                                                    as libc::c_int
                                                && strncmp(
                                                    ret.offset(retind as isize)
                                                        .offset(-(4 as libc::c_int as isize)),
                                                    b"done\0" as *const u8 as *const libc::c_char,
                                                    4 as libc::c_int as usize,
                                                ) == 0 as libc::c_int)
                                                as libc::c_int
                                        }) != 0
                                            || (if 4 as libc::c_int == 0 as libc::c_int {
                                                1 as libc::c_int
                                            } else {
                                                (*ret
                                                    .offset(retind as isize)
                                                    .offset(-(4 as libc::c_int as isize))
                                                    .offset(0 as libc::c_int as isize)
                                                    as libc::c_int
                                                    == (*::core::mem::transmute::<
                                                        &[u8; 5],
                                                        &[libc::c_char; 5],
                                                    >(
                                                        b"then\0"
                                                    ))
                                                        [0 as libc::c_int as usize]
                                                        as libc::c_int
                                                    && strncmp(
                                                        ret.offset(retind as isize)
                                                            .offset(-(4 as libc::c_int as isize)),
                                                        b"then\0" as *const u8
                                                            as *const libc::c_char,
                                                        4 as libc::c_int as usize,
                                                    ) == 0 as libc::c_int)
                                                    as libc::c_int
                                            }) != 0
                                            || (if 4 as libc::c_int == 0 as libc::c_int {
                                                1 as libc::c_int
                                            } else {
                                                (*ret
                                                    .offset(retind as isize)
                                                    .offset(-(4 as libc::c_int as isize))
                                                    .offset(0 as libc::c_int as isize)
                                                    as libc::c_int
                                                    == (*::core::mem::transmute::<
                                                        &[u8; 5],
                                                        &[libc::c_char; 5],
                                                    >(
                                                        b"else\0"
                                                    ))
                                                        [0 as libc::c_int as usize]
                                                        as libc::c_int
                                                    && strncmp(
                                                        ret.offset(retind as isize)
                                                            .offset(-(4 as libc::c_int as isize)),
                                                        b"else\0" as *const u8
                                                            as *const libc::c_char,
                                                        4 as libc::c_int as usize,
                                                    ) == 0 as libc::c_int)
                                                    as libc::c_int
                                            }) != 0
                                            || (if 4 as libc::c_int == 0 as libc::c_int {
                                                1 as libc::c_int
                                            } else {
                                                (*ret
                                                    .offset(retind as isize)
                                                    .offset(-(4 as libc::c_int as isize))
                                                    .offset(0 as libc::c_int as isize)
                                                    as libc::c_int
                                                    == (*::core::mem::transmute::<
                                                        &[u8; 5],
                                                        &[libc::c_char; 5],
                                                    >(
                                                        b"elif\0"
                                                    ))
                                                        [0 as libc::c_int as usize]
                                                        as libc::c_int
                                                    && strncmp(
                                                        ret.offset(retind as isize)
                                                            .offset(-(4 as libc::c_int as isize)),
                                                        b"elif\0" as *const u8
                                                            as *const libc::c_char,
                                                        4 as libc::c_int as usize,
                                                    ) == 0 as libc::c_int)
                                                    as libc::c_int
                                            }) != 0
                                            || (if 4 as libc::c_int == 0 as libc::c_int {
                                                1 as libc::c_int
                                            } else {
                                                (*ret
                                                    .offset(retind as isize)
                                                    .offset(-(4 as libc::c_int as isize))
                                                    .offset(0 as libc::c_int as isize)
                                                    as libc::c_int
                                                    == (*::core::mem::transmute::<
                                                        &[u8; 5],
                                                        &[libc::c_char; 5],
                                                    >(
                                                        b"time\0"
                                                    ))
                                                        [0 as libc::c_int as usize]
                                                        as libc::c_int
                                                    && strncmp(
                                                        ret.offset(retind as isize)
                                                            .offset(-(4 as libc::c_int as isize)),
                                                        b"time\0" as *const u8
                                                            as *const libc::c_char,
                                                        4 as libc::c_int as usize,
                                                    ) == 0 as libc::c_int)
                                                    as libc::c_int
                                            }) != 0
                                        {
                                            tflags |= 0x10 as libc::c_int;
                                            lex_rwlen = 0 as libc::c_int;
                                        } else if *sh_syntaxtab
                                            .as_mut_ptr()
                                            .offset(ch as libc::c_uchar as isize)
                                            & 0x1 as libc::c_int
                                            == 0 as libc::c_int
                                        {
                                            tflags &= !(0x10 as libc::c_int);
                                        } else {
                                            lex_rwlen = 0 as libc::c_int;
                                        }
                                    } else if !(tflags & 0x2 as libc::c_int != 0
                                        && ch == '#' as i32
                                        && (lex_rwlen == 0 as libc::c_int
                                            || tflags & 0x800 as libc::c_int != 0
                                                && lex_wlen == 0 as libc::c_int)
                                        && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                            *shell_input_line_property.offset(
                                                shell_input_line_index
                                                    .wrapping_sub(1 as libc::c_int as size_t)
                                                    as isize,
                                            )
                                                as libc::c_int
                                        } else {
                                            1 as libc::c_int
                                        }) != 0)
                                    {
                                        if tflags & 0x40 as libc::c_int == 0 as libc::c_int
                                            && (*(*c___ctype_b_loc())
                                                .offset(ch as libc::c_uchar as libc::c_int as isize)
                                                as libc::c_int
                                                & _ISblank as libc::c_int as libc::c_ushort
                                                    as libc::c_int
                                                != 0
                                                || ch == '\n' as i32)
                                            && lex_rwlen == 2 as libc::c_int
                                            && (if 2 as libc::c_int == 0 as libc::c_int {
                                                1 as libc::c_int
                                            } else {
                                                (*ret
                                                    .offset(retind as isize)
                                                    .offset(-(2 as libc::c_int as isize))
                                                    .offset(0 as libc::c_int as isize)
                                                    as libc::c_int
                                                    == (*::core::mem::transmute::<
                                                        &[u8; 3],
                                                        &[libc::c_char; 3],
                                                    >(
                                                        b"do\0"
                                                    ))
                                                        [0 as libc::c_int as usize]
                                                        as libc::c_int
                                                    && strncmp(
                                                        ret.offset(retind as isize)
                                                            .offset(-(2 as libc::c_int as isize)),
                                                        b"do\0" as *const u8 as *const libc::c_char,
                                                        2 as libc::c_int as usize,
                                                    ) == 0 as libc::c_int)
                                                    as libc::c_int
                                            }) != 0
                                            && (if shell_input_line_index
                                                > 1 as libc::c_int as size_t
                                            {
                                                *shell_input_line_property.offset(
                                                    shell_input_line_index
                                                        .wrapping_sub(1 as libc::c_int as size_t)
                                                        as isize,
                                                )
                                                    as libc::c_int
                                            } else {
                                                1 as libc::c_int
                                            }) != 0
                                        {
                                            lex_rwlen = 0 as libc::c_int;
                                        } else if tflags & 0x40 as libc::c_int != 0
                                            && ch != '\n' as i32
                                            && (if shell_input_line_index
                                                > 1 as libc::c_int as size_t
                                            {
                                                *shell_input_line_property.offset(
                                                    shell_input_line_index
                                                        .wrapping_sub(1 as libc::c_int as size_t)
                                                        as isize,
                                                )
                                                    as libc::c_int
                                            } else {
                                                1 as libc::c_int
                                            }) != 0
                                        {
                                            tflags &= !(0x10 as libc::c_int);
                                        } else if *sh_syntaxtab
                                            .as_mut_ptr()
                                            .offset(ch as libc::c_uchar as isize)
                                            & 0x2 as libc::c_int
                                            == 0 as libc::c_int
                                            && (if shell_input_line_index
                                                > 1 as libc::c_int as size_t
                                            {
                                                *shell_input_line_property.offset(
                                                    shell_input_line_index
                                                        .wrapping_sub(1 as libc::c_int as size_t)
                                                        as isize,
                                                )
                                                    as libc::c_int
                                            } else {
                                                1 as libc::c_int
                                            }) != 0
                                        {
                                            tflags &= !(0x10 as libc::c_int);
                                        }
                                    }
                                }
                                if tflags & 0x4 as libc::c_int == 0 as libc::c_int
                                    && tflags & 0x20 as libc::c_int != 0
                                    && ch == '<' as i32
                                    && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                        *shell_input_line_property.offset(
                                            shell_input_line_index
                                                .wrapping_sub(1 as libc::c_int as size_t)
                                                as isize,
                                        ) as libc::c_int
                                    } else {
                                        1 as libc::c_int
                                    }) != 0
                                {
                                    if retind + 1 as libc::c_int >= retsize {
                                        while retind + 1 as libc::c_int >= retsize {
                                            retsize += 64 as libc::c_int;
                                        }
                                        ret = realloc(ret as *mut libc::c_void, retsize as usize)
                                            as *mut libc::c_char;
                                    }
                                    let fresh28 = retind;
                                    retind = retind + 1;

                                    *ret.offset(fresh28 as isize) = ch as libc::c_char;

                                    peekc = shell_getc(1 as libc::c_int);
                                    if !(peekc == -(1 as libc::c_int)) {
                                        if peekc == ch {
                                            if retind + 1 as libc::c_int >= retsize {
                                                while retind + 1 as libc::c_int >= retsize {
                                                    retsize += 64 as libc::c_int;
                                                }
                                                ret = realloc(
                                                    ret as *mut libc::c_void,
                                                    retsize as usize,
                                                )
                                                    as *mut libc::c_char;
                                            }
                                            let fresh29 = retind;
                                            retind = retind + 1;

                                            *ret.offset(fresh29 as isize) = peekc as libc::c_char;

                                            peekc = shell_getc(1 as libc::c_int);
                                            if !(peekc == -(1 as libc::c_int)) {
                                                if peekc == '-' as i32 {
                                                    if retind + 1 as libc::c_int >= retsize {
                                                        while retind + 1 as libc::c_int >= retsize {
                                                            retsize += 64 as libc::c_int;
                                                        }
                                                        ret = realloc(
                                                            ret as *mut libc::c_void,
                                                            retsize as usize,
                                                        )
                                                            as *mut libc::c_char;
                                                    }

                                                    let fresh30 = retind;
                                                    retind = retind + 1;
                                                    *ret.offset(fresh30 as isize) =
                                                        peekc as libc::c_char;
                                                    tflags |= 0x200 as libc::c_int;
                                                } else {
                                                    shell_ungetc(peekc);
                                                }
                                                if peekc != '<' as i32 {
                                                    tflags |= 0x100 as libc::c_int;
                                                    lex_firstind = -(1 as libc::c_int);
                                                }
                                                continue;
                                            }
                                        } else {
                                            shell_ungetc(peekc);
                                            continue;
                                        }
                                    }
                                } else {
                                    if tflags & 0x2 as libc::c_int != 0
                                        && tflags & 0x4 as libc::c_int == 0 as libc::c_int
                                        && ch == '#' as i32
                                        && (tflags & 0x10 as libc::c_int != 0
                                            && lex_rwlen == 0 as libc::c_int
                                            || tflags & 0x800 as libc::c_int != 0
                                                && lex_wlen == 0 as libc::c_int)
                                        && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                            *shell_input_line_property.offset(
                                                shell_input_line_index
                                                    .wrapping_sub(1 as libc::c_int as size_t)
                                                    as isize,
                                            )
                                                as libc::c_int
                                        } else {
                                            1 as libc::c_int
                                        }) != 0
                                    {
                                        tflags |= 0x4 as libc::c_int;
                                    }
                                    if (ch == '\u{1}' as i32 || ch == '\u{7f}' as i32)
                                        && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                            *shell_input_line_property.offset(
                                                shell_input_line_index
                                                    .wrapping_sub(1 as libc::c_int as size_t)
                                                    as isize,
                                            )
                                                as libc::c_int
                                        } else {
                                            1 as libc::c_int
                                        }) != 0
                                    {
                                        if retind + 2 as libc::c_int >= retsize {
                                            while retind + 2 as libc::c_int >= retsize {
                                                retsize += 64 as libc::c_int;
                                            }
                                            ret =
                                                realloc(ret as *mut libc::c_void, retsize as usize)
                                                    as *mut libc::c_char;
                                        }

                                        let fresh31 = retind;
                                        retind = retind + 1;
                                        *ret.offset(fresh31 as isize) =
                                            '\u{1}' as i32 as libc::c_char;
                                        let fresh32 = retind;
                                        retind = retind + 1;
                                        *ret.offset(fresh32 as isize) = ch as libc::c_char;

                                        continue;
                                    } else {
                                        if ch == close
                                            && tflags & 0x40 as libc::c_int == 0 as libc::c_int
                                            && (if shell_input_line_index
                                                > 1 as libc::c_int as size_t
                                            {
                                                *shell_input_line_property.offset(
                                                    shell_input_line_index
                                                        .wrapping_sub(1 as libc::c_int as size_t)
                                                        as isize,
                                                )
                                                    as libc::c_int
                                            } else {
                                                1 as libc::c_int
                                            }) != 0
                                        {
                                            count -= 1;
                                        } else if flags & 0x1 as libc::c_int == 0 as libc::c_int
                                            && tflags & 0x40 as libc::c_int == 0 as libc::c_int
                                            && ch == open
                                            && (if shell_input_line_index
                                                > 1 as libc::c_int as size_t
                                            {
                                                *shell_input_line_property.offset(
                                                    shell_input_line_index
                                                        .wrapping_sub(1 as libc::c_int as size_t)
                                                        as isize,
                                                )
                                                    as libc::c_int
                                            } else {
                                                1 as libc::c_int
                                            }) != 0
                                        {
                                            count += 1;
                                        }
                                        if retind + 1 as libc::c_int >= retsize {
                                            while retind + 1 as libc::c_int >= retsize {
                                                retsize += 64 as libc::c_int;
                                            }
                                            ret =
                                                realloc(ret as *mut libc::c_void, retsize as usize)
                                                    as *mut libc::c_char;
                                        }
                                        let fresh33 = retind;
                                        retind = retind + 1;

                                        *ret.offset(fresh33 as isize) = ch as libc::c_char;

                                        if count == 0 as libc::c_int {
                                            break;
                                        }
                                        if ch == '\\' as i32
                                            && (if shell_input_line_index
                                                > 1 as libc::c_int as size_t
                                            {
                                                *shell_input_line_property.offset(
                                                    shell_input_line_index
                                                        .wrapping_sub(1 as libc::c_int as size_t)
                                                        as isize,
                                                )
                                                    as libc::c_int
                                            } else {
                                                1 as libc::c_int
                                            }) != 0
                                        {
                                            tflags |= 0x8 as libc::c_int;
                                        }
                                        if *sh_syntaxtab
                                            .as_mut_ptr()
                                            .offset(ch as libc::c_uchar as isize)
                                            & 0x8 as libc::c_int
                                            != 0
                                            && (if shell_input_line_index
                                                > 1 as libc::c_int as size_t
                                            {
                                                *shell_input_line_property.offset(
                                                    shell_input_line_index
                                                        .wrapping_sub(1 as libc::c_int as size_t)
                                                        as isize,
                                                )
                                                    as libc::c_int
                                            } else {
                                                1 as libc::c_int
                                            }) != 0
                                        {
                                            if dstack.delimiter_depth + 2 as libc::c_int
                                                > dstack.delimiter_space
                                            {
                                                dstack.delimiter_space += 10 as libc::c_int;

                                                dstack.delimiters = realloc(
                                                    dstack.delimiters as *mut libc::c_void,
                                                    (dstack.delimiter_space as libc::c_ulong)
                                                        .wrapping_mul(::core::mem::size_of::<
                                                            libc::c_char,
                                                        >(
                                                        )
                                                            as libc::c_ulong)
                                                        as usize,
                                                )
                                                    as *mut libc::c_char;
                                            }

                                            *(dstack.delimiters)
                                                .offset(dstack.delimiter_depth as isize) =
                                                ch as libc::c_char;

                                            dstack.delimiter_depth += 1;
                                            dstack.delimiter_depth;
                                            if tflags & 0x1 as libc::c_int != 0
                                                && ch == '\'' as i32
                                                && (if shell_input_line_index
                                                    > 1 as libc::c_int as size_t
                                                {
                                                    *shell_input_line_property.offset(
                                                        shell_input_line_index.wrapping_sub(
                                                            1 as libc::c_int as size_t,
                                                        )
                                                            as isize,
                                                    )
                                                        as libc::c_int
                                                } else {
                                                    1 as libc::c_int
                                                }) != 0
                                            {
                                                nestret = parse_matched_pair(
                                                    ch,
                                                    ch,
                                                    ch,
                                                    &mut nestlen,
                                                    0x2 as libc::c_int | rflags,
                                                );
                                            } else {
                                                nestret = parse_matched_pair(
                                                    ch,
                                                    ch,
                                                    ch,
                                                    &mut nestlen,
                                                    rflags,
                                                );
                                            }
                                            dstack.delimiter_depth -= 1;
                                            dstack.delimiter_depth;
                                            if nestret
                                                == &mut matched_pair_error as *mut libc::c_char
                                            {
                                                free(ret as *mut libc::c_void);
                                                return &mut matched_pair_error;
                                            }
                                            if tflags & 0x1 as libc::c_int != 0
                                                && ch == '\'' as i32
                                                && (extended_quote != 0
                                                    || rflags & 0x4 as libc::c_int
                                                        == 0 as libc::c_int)
                                                && (if shell_input_line_index
                                                    > 1 as libc::c_int as size_t
                                                {
                                                    *shell_input_line_property.offset(
                                                        shell_input_line_index.wrapping_sub(
                                                            1 as libc::c_int as size_t,
                                                        )
                                                            as isize,
                                                    )
                                                        as libc::c_int
                                                } else {
                                                    1 as libc::c_int
                                                }) != 0
                                            {
                                                ttrans = c_ansiexpand(
                                                    nestret,
                                                    0 as libc::c_int,
                                                    nestlen - 1 as libc::c_int,
                                                    &mut ttranslen,
                                                );
                                                free(nestret as *mut libc::c_void);
                                                if rflags & 0x4 as libc::c_int == 0 as libc::c_int {
                                                    nestret = c_sh_single_quote(ttrans);
                                                    free(ttrans as *mut libc::c_void);
                                                    nestlen = libc::strlen(nestret) as libc::c_int;
                                                } else {
                                                    nestret = ttrans;
                                                    nestlen = ttranslen;
                                                }
                                                retind -= 2 as libc::c_int;
                                            } else if tflags & 0x1 as libc::c_int != 0
                                                && ch == '"' as i32
                                                && (extended_quote != 0
                                                    || rflags & 0x4 as libc::c_int
                                                        == 0 as libc::c_int)
                                                && (if shell_input_line_index
                                                    > 1 as libc::c_int as size_t
                                                {
                                                    *shell_input_line_property.offset(
                                                        shell_input_line_index.wrapping_sub(
                                                            1 as libc::c_int as size_t,
                                                        )
                                                            as isize,
                                                    )
                                                        as libc::c_int
                                                } else {
                                                    1 as libc::c_int
                                                }) != 0
                                            {
                                                ttrans = localeexpand(
                                                    nestret,
                                                    0 as libc::c_int,
                                                    nestlen - 1 as libc::c_int,
                                                    start_lineno,
                                                    &mut ttranslen,
                                                );
                                                free(nestret as *mut libc::c_void);
                                                nestret = c_sh_mkdoublequoted(
                                                    ttrans,
                                                    ttranslen,
                                                    0 as libc::c_int,
                                                );
                                                free(ttrans as *mut libc::c_void);
                                                nestlen = ttranslen + 2 as libc::c_int;
                                                retind -= 2 as libc::c_int;
                                            }
                                            if nestlen != 0 {
                                                if retind + nestlen >= retsize {
                                                    while retind + nestlen >= retsize {
                                                        retsize += 64 as libc::c_int;
                                                    }
                                                    ret = realloc(
                                                        ret as *mut libc::c_void,
                                                        retsize as usize,
                                                    )
                                                        as *mut libc::c_char;
                                                }
                                                strcpy(ret.offset(retind as isize), nestret);
                                                retind += nestlen;
                                            }
                                            if !nestret.is_null() {
                                                free(nestret as *mut libc::c_void);
                                            }
                                            nestret = 0 as *mut libc::c_char;
                                        } else if tflags & 0x1 as libc::c_int != 0
                                            && (ch == '(' as i32
                                                || ch == '{' as i32
                                                || ch == '[' as i32)
                                            && (if shell_input_line_index
                                                > 1 as libc::c_int as size_t
                                            {
                                                *shell_input_line_property.offset(
                                                    shell_input_line_index
                                                        .wrapping_sub(1 as libc::c_int as size_t)
                                                        as isize,
                                                )
                                                    as libc::c_int
                                            } else {
                                                1 as libc::c_int
                                            }) != 0
                                        {
                                            if tflags & 0x40 as libc::c_int == 0 as libc::c_int
                                                && open == ch
                                            {
                                                count -= 1;
                                            }
                                            if ch == '(' as i32 {
                                                nestret = parse_comsub(
                                                    0 as libc::c_int,
                                                    '(' as i32,
                                                    ')' as i32,
                                                    &mut nestlen,
                                                    (rflags | 0x8 as libc::c_int)
                                                        & !(0x4 as libc::c_int),
                                                );
                                            } else if ch == '{' as i32 {
                                                nestret = parse_matched_pair(
                                                    0 as libc::c_int,
                                                    '{' as i32,
                                                    '}' as i32,
                                                    &mut nestlen,
                                                    0x1 as libc::c_int
                                                        | 0x40 as libc::c_int
                                                        | rflags,
                                                );
                                            } else if ch == '[' as i32 {
                                                nestret = parse_matched_pair(
                                                    0 as libc::c_int,
                                                    '[' as i32,
                                                    ']' as i32,
                                                    &mut nestlen,
                                                    rflags,
                                                );
                                            }
                                            if nestret
                                                == &mut matched_pair_error as *mut libc::c_char
                                            {
                                                free(ret as *mut libc::c_void);
                                                return &mut matched_pair_error;
                                            }
                                            if nestlen != 0 {
                                                if retind + nestlen >= retsize {
                                                    while retind + nestlen >= retsize {
                                                        retsize += 64 as libc::c_int;
                                                    }
                                                    ret = realloc(
                                                        ret as *mut libc::c_void,
                                                        retsize as usize,
                                                    )
                                                        as *mut libc::c_char;
                                                }
                                                strcpy(ret.offset(retind as isize), nestret);
                                                retind += nestlen;
                                            }
                                            if !nestret.is_null() {
                                                free(nestret as *mut libc::c_void);
                                            }
                                            nestret = 0 as *mut libc::c_char;
                                        }
                                        if ch == '$' as i32
                                            && tflags & 0x1 as libc::c_int == 0 as libc::c_int
                                            && (if shell_input_line_index
                                                > 1 as libc::c_int as size_t
                                            {
                                                *shell_input_line_property.offset(
                                                    shell_input_line_index
                                                        .wrapping_sub(1 as libc::c_int as size_t)
                                                        as isize,
                                                )
                                                    as libc::c_int
                                            } else {
                                                1 as libc::c_int
                                            }) != 0
                                        {
                                            tflags |= 0x1 as libc::c_int;
                                        } else {
                                            tflags &= !(0x1 as libc::c_int);
                                        }
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            history_expansion_inhibited = orig_histexp;
            free(ret as *mut libc::c_void);
            if !heredelim.is_null() {
                free(heredelim as *mut libc::c_void);
            }
            heredelim = 0 as *mut libc::c_char;
            parser_error(
                start_lineno,
                c_dcgettext(
                    0 as *const libc::c_char,
                    b"unexpected EOF while looking for matching `%c'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                close,
            );
            EOF_Reached = 1 as libc::c_int;
            return &mut matched_pair_error;
        }

        history_expansion_inhibited = orig_histexp;
        if !heredelim.is_null() {
            free(heredelim as *mut libc::c_void);
        }

        heredelim = 0 as *mut libc::c_char;

        *ret.offset(retind as isize) = '\0' as i32 as libc::c_char;

        if !lenp.is_null() {
            *lenp = retind;
        }
        return ret;
    }
}
#[no_mangle]
pub fn xparse_dolparen(
    mut base: *mut libc::c_char,
    mut string: *mut libc::c_char,
    mut indp: *mut libc::c_int,
    mut flags: libc::c_int,
) -> *mut libc::c_char {
    unsafe {
        let mut ps: sh_parser_state_t = _sh_parser_state_t {
            parser_state: 0,
            token_state: 0 as *mut libc::c_int,
            token: 0 as *mut libc::c_char,
            token_buffer_size: 0,
            input_line_terminator: 0,
            eof_encountered: 0,
            prompt_string_pointer: 0 as *mut *mut libc::c_char,
            current_command_line_count: 0,
            remember_on_history: 0,
            history_expansion_inhibited: 0,
            last_command_exit_value: 0,
            pipestatus: 0 as *mut ARRAY,
            last_shell_builtin: None,
            this_shell_builtin: None,
            expand_aliases: 0,
            echo_input_at_read: 0,
            need_here_doc: 0,
            here_doc_first_line: 0,
            redir_stack: [0 as *mut REDIRECT; 16],
        };
        let mut ls: sh_input_line_state_t = _sh_input_line_state_t {
            input_line: 0 as *mut libc::c_char,
            input_line_index: 0,
            input_line_size: 0,
            input_line_len: 0,
            input_property: 0 as *mut libc::c_char,
            input_propsize: 0,
        };
        let mut orig_ind: libc::c_int = 0;
        let mut nc: libc::c_int = 0;
        let mut sflags: libc::c_int = 0;
        let mut orig_eof_token: libc::c_int = 0;
        let mut start_lineno: libc::c_int = 0;
        let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ep: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ostring: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut saved_pushed_strings: *mut STRING_SAVER = 0 as *mut STRING_SAVER;
        orig_ind = *indp;
        ostring = string;
        start_lineno = line_number;
        if *string as libc::c_int == 0 as libc::c_int {
            if flags & 0x1 as libc::c_int != 0 {
                return 0 as *mut libc::c_void as *mut libc::c_char;
            }

            ret = malloc(1 as libc::c_int as usize) as *mut libc::c_char;
            *ret.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;

            return ret;
        }
        sflags = 0x1 as libc::c_int | 0x4 as libc::c_int | 0x8 as libc::c_int;
        if flags & 0x40 as libc::c_int != 0 {
            sflags |= 0x40 as libc::c_int;
        }
        save_parser_state(&mut ps);
        save_input_line_state(&mut ls);

        orig_eof_token = shell_eof_token;
        echo_input_at_read = 0 as libc::c_int;
        saved_pushed_strings = pushed_string_list;
        pushed_string_list = 0 as *mut libc::c_void as *mut STRING_SAVER;
        parser_state |= 0x40 as libc::c_int | 0x8000 as libc::c_int;
        shell_eof_token = ')' as i32;

        nc = parse_string(
            string,
            b"command substitution\0" as *const u8 as *const libc::c_char,
            sflags,
            &mut ep,
        );

        if current_token == shell_eof_token {
            yychar = -(2 as libc::c_int);
        }

        reset_parser();
        restore_input_line_state(&mut ls);
        shell_eof_token = orig_eof_token;
        restore_parser_state(&mut ps);
        pushed_string_list = saved_pushed_strings;
        token_to_read = 0 as libc::c_int;
        if nc < 0 as libc::c_int {
            clear_shell_input_line();

            if bash_input.type_0 as libc::c_uint != st_string as libc::c_int as libc::c_uint {
                parser_state &= !(0x40 as libc::c_int | 0x8000 as libc::c_int);
            }

            jump_to_top_level(-nc);
        }
        if *ep.offset(-(1 as libc::c_int) as isize) as libc::c_int != ')' as i32 {
            while ep > ostring
                && *ep.offset(-(1 as libc::c_int) as isize) as libc::c_int == '\n' as i32
            {
                ep = ep.offset(-1);
            }
        }

        nc = ep.offset_from(ostring) as libc::c_long as libc::c_int;

        *indp = (ep.offset_from(base) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as libc::c_int;
        if *base.offset(*indp as isize) as libc::c_int != ')' as i32 {
            parser_error(
                start_lineno,
                c_dcgettext(
                    0 as *const libc::c_char,
                    b"unexpected EOF while looking for matching `%c'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                ')' as i32,
            );
            jump_to_top_level(2 as libc::c_int);
        }

        if flags & 0x1 as libc::c_int != 0 {
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
        if nc == 0 as libc::c_int {
            ret = malloc(1 as libc::c_int as usize) as *mut libc::c_char;
            *ret.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        } else {
            ret = substring(ostring, 0 as libc::c_int, nc - 1 as libc::c_int);
        }
        return ret;
    }
}
fn parse_dparen(mut c: libc::c_int) -> libc::c_int {
    unsafe {
        let mut cmdtyp: libc::c_int = 0;
        let mut sline: libc::c_int = 0;
        let mut wval: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut wd: *mut WORD_DESC = 0 as *mut WORD_DESC;
        if last_read_token == 265 as libc::c_int {
            arith_for_lineno = line_number;
            cmdtyp = parse_arith_cmd(&mut wval, 0 as libc::c_int);
            if cmdtyp == 1 as libc::c_int {
                wd = alloc_word_desc();
                (*wd).word = wval;

                yylval.WordList = make_word_list(wd, 0 as *mut libc::c_void as *mut WORD_LIST);
                return 286 as libc::c_int;
            } else {
                return -(1 as libc::c_int);
            }
        }
        if reserved_word_acceptable(last_read_token) != 0 {
            sline = line_number;
            cmdtyp = parse_arith_cmd(&mut wval, 0 as libc::c_int);
            if cmdtyp == 1 as libc::c_int {
                wd = alloc_word_desc();
                (*wd).word = wval;
                (*wd).flags = (1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 4 as libc::c_int
                    | (1 as libc::c_int) << 5 as libc::c_int
                    | (1 as libc::c_int) << 19 as libc::c_int;

                yylval.WordList = make_word_list(wd, 0 as *mut libc::c_void as *mut WORD_LIST);
                return 285 as libc::c_int;
            } else if cmdtyp == 0 as libc::c_int {
                push_string(
                    wval,
                    0 as libc::c_int,
                    0 as *mut libc::c_void as *mut alias_t,
                );

                (*pushed_string_list).flags = 0x2 as libc::c_int;
                if parser_state & 0x1 as libc::c_int == 0 as libc::c_int {
                    parser_state |= 0x20 as libc::c_int;
                }

                return c;
            } else {
                return -(1 as libc::c_int);
            }
        }
        return -(2 as libc::c_int);
    }
}
fn parse_arith_cmd(mut ep: *mut *mut libc::c_char, mut adddq: libc::c_int) -> libc::c_int {
    unsafe {
        let mut exp_lineno: libc::c_int = 0;
        let mut rval: libc::c_int = 0;
        let mut c: libc::c_int = 0;
        let mut ttok: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut tokstr: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ttoklen: libc::c_int = 0;
        exp_lineno = line_number;
        ttok = parse_matched_pair(
            0 as libc::c_int,
            '(' as i32,
            ')' as i32,
            &mut ttoklen,
            0 as libc::c_int,
        );
        rval = 1 as libc::c_int;
        if ttok == &mut matched_pair_error as *mut libc::c_char {
            return -(1 as libc::c_int);
        }
        c = shell_getc(0 as libc::c_int);
        if c != ')' as i32
            && (if shell_input_line_index > 1 as libc::c_int as size_t {
                *shell_input_line_property.offset(
                    shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t) as isize,
                ) as libc::c_int
            } else {
                1 as libc::c_int
            }) != 0
        {
            rval = 0 as libc::c_int;
        }
        tokstr = malloc((ttoklen + 4 as libc::c_int) as usize) as *mut libc::c_char;
        if rval == 1 as libc::c_int && adddq != 0 {
            *tokstr.offset(0 as libc::c_int as isize) = '"' as i32 as libc::c_char;

            strncpy(
                tokstr.offset(1 as libc::c_int as isize),
                ttok,
                (ttoklen - 1 as libc::c_int) as libc::c_ulong as usize,
            );
            *tokstr.offset(ttoklen as isize) = '"' as i32 as libc::c_char;
            *tokstr.offset((ttoklen + 1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
        } else if rval == 1 as libc::c_int {
            strncpy(tokstr, ttok, (ttoklen - 1 as libc::c_int) as usize);
            *tokstr.offset((ttoklen - 1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
        } else {
            *tokstr.offset(0 as libc::c_int as isize) = '(' as i32 as libc::c_char;

            strncpy(
                tokstr.offset(1 as libc::c_int as isize),
                ttok,
                (ttoklen - 1 as libc::c_int) as usize,
            );
            *tokstr.offset(ttoklen as isize) = ')' as i32 as libc::c_char;
            *tokstr.offset((ttoklen + 1 as libc::c_int) as isize) = c as libc::c_char;
            *tokstr.offset((ttoklen + 2 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
        }
        *ep = tokstr;
        if !ttok.is_null() {
            free(ttok as *mut libc::c_void);
        }
        ttok = 0 as *mut libc::c_char;
        return rval;
    }
}
fn cond_error() {
    unsafe {
        let mut etext: *mut libc::c_char = 0 as *mut libc::c_char;
        if EOF_Reached != 0 && cond_token != 275 as libc::c_int {
            parser_error(
                cond_lineno,
                c_dcgettext(
                    0 as *const libc::c_char,
                    b"unexpected EOF while looking for `]]'\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        } else if cond_token != 275 as libc::c_int {
            etext = error_token_from_token(cond_token);
            if !etext.is_null() {
                parser_error(
                    cond_lineno,
                    c_dcgettext(
                        0 as *const libc::c_char,
                        b"syntax error in conditional expression: unexpected token `%s'\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    etext,
                );
                free(etext as *mut libc::c_void);
            } else {
                parser_error(
                    cond_lineno,
                    c_dcgettext(
                        0 as *const libc::c_char,
                        b"syntax error in conditional expression\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
        }
    }
}
fn cond_expr() -> *mut COND_COM {
    return cond_or();
}
fn cond_or() -> *mut COND_COM {
    unsafe {
        let mut l: *mut COND_COM = 0 as *mut COND_COM;
        let mut r: *mut COND_COM = 0 as *mut COND_COM;
        l = cond_and();
        if cond_token == 289 as libc::c_int {
            r = cond_or();
            l = make_cond_node(
                2 as libc::c_int,
                0 as *mut libc::c_void as *mut WORD_DESC,
                l,
                r,
            );
        }
        return l;
    }
}
fn cond_and() -> *mut COND_COM {
    unsafe {
        let mut l: *mut COND_COM = 0 as *mut COND_COM;
        let mut r: *mut COND_COM = 0 as *mut COND_COM;
        l = cond_term();
        if cond_token == 288 as libc::c_int {
            r = cond_and();
            l = make_cond_node(
                1 as libc::c_int,
                0 as *mut libc::c_void as *mut WORD_DESC,
                l,
                r,
            );
        }
        return l;
    }
}
fn cond_skip_newlines() -> libc::c_int {
    unsafe {
        loop {
            cond_token = read_token(0 as libc::c_int);
            if !(cond_token == '\n' as i32) {
                break;
            }
            if interactive != 0
                && (bash_input.type_0 as libc::c_uint == st_stdin as libc::c_int as libc::c_uint
                    || bash_input.type_0 as libc::c_uint
                        == st_stream as libc::c_int as libc::c_uint)
            {
                prompt_again();
            }
        }
        return cond_token;
    }
}
fn cond_term() -> *mut COND_COM {
    unsafe {
        let mut op: *mut WORD_DESC = 0 as *mut WORD_DESC;
        let mut term: *mut COND_COM = 0 as *mut COND_COM;
        let mut tleft: *mut COND_COM = 0 as *mut COND_COM;
        let mut tright: *mut COND_COM = 0 as *mut COND_COM;
        let mut tok: libc::c_int = 0;
        let mut lineno: libc::c_int = 0;
        let mut etext: *mut libc::c_char = 0 as *mut libc::c_char;
        tok = cond_skip_newlines();
        lineno = line_number;
        if tok == 274 as libc::c_int {
            cond_token = 275 as libc::c_int;
            return 0 as *mut libc::c_void as *mut COND_COM;
        } else if tok == '(' as i32 {
            term = cond_expr();
            if cond_token != ')' as i32 {
                if !term.is_null() {
                    dispose_cond_node(term);
                }
                etext = error_token_from_token(cond_token);
                if !etext.is_null() {
                    parser_error(
                        lineno,
                        c_dcgettext(
                            0 as *const libc::c_char,
                            b"unexpected token `%s', expected `)'\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        etext,
                    );
                    free(etext as *mut libc::c_void);
                } else {
                    parser_error(
                        lineno,
                        c_dcgettext(
                            0 as *const libc::c_char,
                            b"expected `)'\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                cond_token = 275 as libc::c_int;
                return 0 as *mut libc::c_void as *mut COND_COM;
            }
            term = make_cond_node(
                6 as libc::c_int,
                0 as *mut libc::c_void as *mut WORD_DESC,
                term,
                0 as *mut libc::c_void as *mut COND_COM,
            );
            cond_skip_newlines();
        } else if tok == 277 as libc::c_int
            || tok == 281 as libc::c_int
                && (*((*yylval.word).word).offset(0 as libc::c_int as isize) as libc::c_int
                    == '!' as i32
                    && *((*yylval.word).word).offset(1 as libc::c_int as isize) as libc::c_int
                        == '\0' as i32)
        {
            if tok == 281 as libc::c_int {
                dispose_word(yylval.word);
            }
            term = cond_term();
            if !term.is_null() {
                (*term).flags |= 0x4 as libc::c_int;
            }
        } else if tok == 281 as libc::c_int
            && *((*yylval.word).word).offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
            && *((*yylval.word).word).offset(1 as libc::c_int as isize) as libc::c_int != 0
            && *((*yylval.word).word).offset(2 as libc::c_int as isize) as libc::c_int
                == 0 as libc::c_int
            && test_unop((*yylval.word).word) != 0
        {
            op = yylval.word;
            tok = read_token(0 as libc::c_int);
            if tok == 281 as libc::c_int {
                tleft = make_cond_node(
                    5 as libc::c_int,
                    yylval.word,
                    0 as *mut libc::c_void as *mut COND_COM,
                    0 as *mut libc::c_void as *mut COND_COM,
                );
                term = make_cond_node(
                    3 as libc::c_int,
                    op,
                    tleft,
                    0 as *mut libc::c_void as *mut COND_COM,
                );
            } else {
                dispose_word(op);
                etext = error_token_from_token(tok);
                if !etext.is_null() {
                    parser_error(
                        line_number,
                        c_dcgettext(
                            0 as *const libc::c_char,
                            b"unexpected argument `%s' to conditional unary operator\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        etext,
                    );
                    free(etext as *mut libc::c_void);
                } else {
                    parser_error(
                        line_number,
                        c_dcgettext(
                            0 as *const libc::c_char,
                            b"unexpected argument to conditional unary operator\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                cond_token = 275 as libc::c_int;
                return 0 as *mut libc::c_void as *mut COND_COM;
            }
            cond_skip_newlines();
        } else if tok == 281 as libc::c_int {
            tleft = make_cond_node(
                5 as libc::c_int,
                yylval.word,
                0 as *mut libc::c_void as *mut COND_COM,
                0 as *mut libc::c_void as *mut COND_COM,
            );
            tok = read_token(0 as libc::c_int);
            if tok == 281 as libc::c_int && test_binop((*yylval.word).word) != 0 {
                op = yylval.word;
                if *((*op).word).offset(0 as libc::c_int as isize) as libc::c_int == '=' as i32
                    && (*((*op).word).offset(1 as libc::c_int as isize) as libc::c_int
                        == '\0' as i32
                        || *((*op).word).offset(1 as libc::c_int as isize) as libc::c_int
                            == '=' as i32
                            && *((*op).word).offset(2 as libc::c_int as isize) as libc::c_int
                                == '\0' as i32)
                {
                    parser_state |= 0x1000 as libc::c_int;
                } else if *((*op).word).offset(0 as libc::c_int as isize) as libc::c_int
                    == '!' as i32
                    && *((*op).word).offset(1 as libc::c_int as isize) as libc::c_int == '=' as i32
                    && *((*op).word).offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32
                {
                    parser_state |= 0x1000 as libc::c_int;
                }
            } else if tok == 281 as libc::c_int
                && (*((*yylval.word).word).offset(0 as libc::c_int as isize) as libc::c_int
                    == (*::core::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b"=~\0"))
                        [0 as libc::c_int as usize] as libc::c_int
                    && strcmp(
                        (*yylval.word).word,
                        b"=~\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int)
            {
                op = yylval.word;
                parser_state |= 0x10000 as libc::c_int;
            } else if tok == '<' as i32 || tok == '>' as i32 {
                op = make_word_from_token(tok);
            } else if tok == 274 as libc::c_int
                || tok == 288 as libc::c_int
                || tok == 289 as libc::c_int
                || tok == ')' as i32
            {
                op = make_word(b"-n\0" as *const u8 as *const libc::c_char);
                term = make_cond_node(
                    3 as libc::c_int,
                    op,
                    tleft,
                    0 as *mut libc::c_void as *mut COND_COM,
                );
                cond_token = tok;
                return term;
            } else {
                etext = error_token_from_token(tok);
                if !etext.is_null() {
                    parser_error(
                        line_number,
                        c_dcgettext(
                            0 as *const libc::c_char,
                            b"unexpected token `%s', conditional binary operator expected\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        etext,
                    );
                    free(etext as *mut libc::c_void);
                } else {
                    parser_error(
                        line_number,
                        c_dcgettext(
                            0 as *const libc::c_char,
                            b"conditional binary operator expected\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                dispose_cond_node(tleft);
                cond_token = 275 as libc::c_int;
                return 0 as *mut libc::c_void as *mut COND_COM;
            }

            if parser_state & 0x1000 as libc::c_int != 0 {
                extended_glob = 1 as libc::c_int;
            }
            tok = read_token(0 as libc::c_int);
            if parser_state & 0x1000 as libc::c_int != 0 {
                extended_glob = global_extglob;
            }
            parser_state &= !(0x10000 as libc::c_int | 0x1000 as libc::c_int);

            if tok == 281 as libc::c_int {
                tright = make_cond_node(
                    5 as libc::c_int,
                    yylval.word,
                    0 as *mut libc::c_void as *mut COND_COM,
                    0 as *mut libc::c_void as *mut COND_COM,
                );
                term = make_cond_node(4 as libc::c_int, op, tleft, tright);
            } else {
                etext = error_token_from_token(tok);
                if !etext.is_null() {
                    parser_error(
                        line_number,
                        c_dcgettext(
                            0 as *const libc::c_char,
                            b"unexpected argument `%s' to conditional binary operator\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        etext,
                    );
                    free(etext as *mut libc::c_void);
                } else {
                    parser_error(
                        line_number,
                        c_dcgettext(
                            0 as *const libc::c_char,
                            b"unexpected argument to conditional binary operator\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                dispose_cond_node(tleft);
                dispose_word(op);
                cond_token = 275 as libc::c_int;
                return 0 as *mut libc::c_void as *mut COND_COM;
            }
            cond_skip_newlines();
        } else {
            if tok < 256 as libc::c_int {
                parser_error(
                    line_number,
                    c_dcgettext(
                        0 as *const libc::c_char,
                        b"unexpected token `%c' in conditional command\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    tok,
                );
            } else {
                etext = error_token_from_token(tok);
                if !etext.is_null() {
                    parser_error(
                        line_number,
                        c_dcgettext(
                            0 as *const libc::c_char,
                            b"unexpected token `%s' in conditional command\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        etext,
                    );
                    free(etext as *mut libc::c_void);
                } else {
                    parser_error(
                        line_number,
                        c_dcgettext(
                            0 as *const libc::c_char,
                            b"unexpected token %d in conditional command\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        tok,
                    );
                }
            }
            cond_token = 275 as libc::c_int;
            return 0 as *mut libc::c_void as *mut COND_COM;
        }
        return term;
    }
}
fn parse_cond_command() -> *mut COMMAND {
    unsafe {
        let mut cexp: *mut COND_COM = 0 as *mut COND_COM;
        global_extglob = extended_glob;
        cexp = cond_expr();
        return make_cond_command(cexp);
    }
}
fn token_is_assignment(mut t: *mut libc::c_char, mut i: libc::c_int) -> libc::c_int {
    unsafe {
        let mut r: libc::c_int = 0;
        let mut atoken: *mut libc::c_char = 0 as *mut libc::c_char;
        atoken = malloc((i + 3 as libc::c_int) as usize) as *mut libc::c_char;

        memcpy(
            atoken as *mut libc::c_void,
            t as *const libc::c_void,
            i as usize,
        );

        *atoken.offset(i as isize) = '=' as i32 as libc::c_char;
        *atoken.offset((i + 1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;

        r = assignment(
            atoken,
            (parser_state & 0x2000 as libc::c_int != 0 as libc::c_int) as libc::c_int,
        );
        free(atoken as *mut libc::c_void);
        return (r > 0 as libc::c_int && r == i) as libc::c_int;
    }
}
fn token_is_ident(mut t: *mut libc::c_char, mut i: libc::c_int) -> libc::c_int {
    unsafe {
        let mut c: libc::c_uchar = 0;
        let mut r: libc::c_int = 0;

        c = *t.offset(i as isize) as libc::c_uchar;

        *t.offset(i as isize) = '\0' as i32 as libc::c_char;

        r = legal_identifier(t);

        *t.offset(i as isize) = c as libc::c_char;

        return r;
    }
}
fn read_token_word(mut character: libc::c_int) -> libc::c_int {
    unsafe {
        let mut current_block: u64;
        let mut the_word: *mut WORD_DESC = 0 as *mut WORD_DESC;
        let mut token_index: libc::c_int = 0;
        let mut all_digit_token: libc::c_int = 0;
        let mut dollar_present: libc::c_int = 0;
        let mut compound_assignment: libc::c_int = 0;
        let mut quoted: libc::c_int = 0;
        let mut pass_next_character: libc::c_int = 0;
        let mut cd: libc::c_int = 0;
        let mut result: libc::c_int = 0;
        let mut peek_char: libc::c_int = 0;
        let mut ttok: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ttrans: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ttoklen: libc::c_int = 0;
        let mut ttranslen: libc::c_int = 0;
        let mut lvalue: intmax_t = 0;

        if token_buffer_size < 496 as libc::c_int {
            token_buffer_size = 496 as libc::c_int;
            token = realloc(token as *mut libc::c_void, token_buffer_size as usize)
                as *mut libc::c_char;
        }

        token_index = 0 as libc::c_int;
        all_digit_token = (character >= '0' as i32 && character <= '9' as i32) as libc::c_int;
        compound_assignment = 0 as libc::c_int;
        pass_next_character = compound_assignment;
        quoted = pass_next_character;
        dollar_present = quoted;
        while !(character == -(1 as libc::c_int)) {
            if pass_next_character != 0 {
                pass_next_character = 0 as libc::c_int;
                current_block = 2802315260714836178;
            } else {
                cd = if dstack.delimiter_depth != 0 {
                    *(dstack.delimiters)
                        .offset((dstack.delimiter_depth - 1 as libc::c_int) as isize)
                        as libc::c_int
                } else {
                    0 as libc::c_int
                };
                if character == '\\' as i32
                    && (if shell_input_line_index > 1 as libc::c_int as size_t {
                        *shell_input_line_property.offset(
                            shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                as isize,
                        ) as libc::c_int
                    } else {
                        1 as libc::c_int
                    }) != 0
                {
                    peek_char = shell_getc(0 as libc::c_int);
                    if peek_char == '\n' as i32 {
                        character = '\n' as i32;
                        current_block = 2410910991445625210;
                    } else {
                        shell_ungetc(peek_char);
                        if cd == 0 as libc::c_int
                            || cd == '`' as i32
                            || cd == '"' as i32
                                && peek_char >= 0 as libc::c_int
                                && *sh_syntaxtab.as_mut_ptr().offset(peek_char as isize)
                                    & 0x40 as libc::c_int
                                    != 0
                        {
                            pass_next_character += 1;
                        }
                        quoted = 1 as libc::c_int;
                        current_block = 11877077252304325915;
                    }
                } else if *sh_syntaxtab
                    .as_mut_ptr()
                    .offset(character as libc::c_uchar as isize)
                    & 0x8 as libc::c_int
                    != 0
                    && (if shell_input_line_index > 1 as libc::c_int as size_t {
                        *shell_input_line_property.offset(
                            shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                as isize,
                        ) as libc::c_int
                    } else {
                        1 as libc::c_int
                    }) != 0
                {
                    if dstack.delimiter_depth + 2 as libc::c_int > dstack.delimiter_space {
                        dstack.delimiter_space += 10 as libc::c_int;
                        dstack.delimiters = realloc(
                            dstack.delimiters as *mut libc::c_void,
                            (dstack.delimiter_space as libc::c_ulong).wrapping_mul(
                                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            ) as usize,
                        ) as *mut libc::c_char;
                    }
                    *(dstack.delimiters).offset(dstack.delimiter_depth as isize) =
                        character as libc::c_char;
                    dstack.delimiter_depth += 1;
                    dstack.delimiter_depth;

                    ttok = parse_matched_pair(
                        character,
                        character,
                        character,
                        &mut ttoklen,
                        if character == '`' as i32 {
                            0x8 as libc::c_int
                        } else {
                            0 as libc::c_int
                        },
                    );

                    dstack.delimiter_depth -= 1;
                    dstack.delimiter_depth;
                    if ttok == &mut matched_pair_error as *mut libc::c_char {
                        return -(1 as libc::c_int);
                    }
                    if token_index + (ttoklen + 2 as libc::c_int) >= token_buffer_size {
                        while token_index + (ttoklen + 2 as libc::c_int) >= token_buffer_size {
                            token_buffer_size += 512 as libc::c_int;
                        }
                        token = realloc(token as *mut libc::c_void, token_buffer_size as usize)
                            as *mut libc::c_char;
                    }

                    let fresh34 = token_index;
                    token_index = token_index + 1;

                    *token.offset(fresh34 as isize) = character as libc::c_char;

                    strcpy(token.offset(token_index as isize), ttok);
                    token_index += ttoklen;
                    all_digit_token = 0 as libc::c_int;
                    if character != '`' as i32 {
                        quoted = 1 as libc::c_int;
                    }
                    dollar_present |= (character == '"' as i32
                        && !(strchr(ttok, '$' as i32)).is_null())
                        as libc::c_int;
                    if !ttok.is_null() {
                        free(ttok as *mut libc::c_void);
                    }
                    ttok = 0 as *mut libc::c_char;
                    current_block = 2410910991445625210;
                } else if parser_state & 0x10000 as libc::c_int != 0
                    && (character == '(' as i32 || character == '|' as i32)
                    && (if shell_input_line_index > 1 as libc::c_int as size_t {
                        *shell_input_line_property.offset(
                            shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                as isize,
                        ) as libc::c_int
                    } else {
                        1 as libc::c_int
                    }) != 0
                {
                    if character == '|' as i32 {
                        current_block = 11877077252304325915;
                    } else {
                        if dstack.delimiter_depth + 2 as libc::c_int > dstack.delimiter_space {
                            dstack.delimiter_space += 10 as libc::c_int;
                            dstack.delimiters = realloc(
                                dstack.delimiters as *mut libc::c_void,
                                (dstack.delimiter_space as libc::c_ulong).wrapping_mul(
                                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                ) as usize,
                            ) as *mut libc::c_char;
                        }
                        *(dstack.delimiters).offset(dstack.delimiter_depth as isize) =
                            character as libc::c_char;
                        dstack.delimiter_depth += 1;
                        dstack.delimiter_depth;

                        ttok = parse_matched_pair(
                            cd,
                            '(' as i32,
                            ')' as i32,
                            &mut ttoklen,
                            0 as libc::c_int,
                        );

                        dstack.delimiter_depth -= 1;
                        dstack.delimiter_depth;
                        if ttok == &mut matched_pair_error as *mut libc::c_char {
                            return -(1 as libc::c_int);
                        }
                        if token_index + (ttoklen + 2 as libc::c_int) >= token_buffer_size {
                            while token_index + (ttoklen + 2 as libc::c_int) >= token_buffer_size {
                                token_buffer_size += 512 as libc::c_int;
                            }
                            token = realloc(token as *mut libc::c_void, token_buffer_size as usize)
                                as *mut libc::c_char;
                        }
                        let fresh35 = token_index;
                        token_index = token_index + 1;
                        *token.offset(fresh35 as isize) = character as libc::c_char;
                        strcpy(token.offset(token_index as isize), ttok);
                        token_index += ttoklen;
                        if !ttok.is_null() {
                            free(ttok as *mut libc::c_void);
                        }

                        ttok = 0 as *mut libc::c_char;
                        all_digit_token = 0 as libc::c_int;
                        dollar_present = all_digit_token;
                        current_block = 2410910991445625210;
                    }
                } else {
                    if extended_glob != 0
                        && (character == '@' as i32
                            || character == '*' as i32
                            || character == '+' as i32
                            || character == '?' as i32
                            || character == '!' as i32)
                        && (if shell_input_line_index > 1 as libc::c_int as size_t {
                            *shell_input_line_property.offset(
                                shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                    as isize,
                            ) as libc::c_int
                        } else {
                            1 as libc::c_int
                        }) != 0
                    {
                        peek_char = shell_getc(1 as libc::c_int);
                        if peek_char == '(' as i32
                            && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                *shell_input_line_property.offset(
                                    shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t)
                                        as isize,
                                ) as libc::c_int
                            } else {
                                1 as libc::c_int
                            }) != 0
                        {
                            if dstack.delimiter_depth + 2 as libc::c_int > dstack.delimiter_space {
                                dstack.delimiter_space += 10 as libc::c_int;
                                dstack.delimiters = realloc(
                                    dstack.delimiters as *mut libc::c_void,
                                    (dstack.delimiter_space as libc::c_ulong).wrapping_mul(
                                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                    ) as usize,
                                )
                                    as *mut libc::c_char;
                            }
                            *(dstack.delimiters).offset(dstack.delimiter_depth as isize) =
                                peek_char as libc::c_char;
                            dstack.delimiter_depth += 1;
                            dstack.delimiter_depth;

                            ttok = parse_matched_pair(
                                cd,
                                '(' as i32,
                                ')' as i32,
                                &mut ttoklen,
                                0 as libc::c_int,
                            );
                            dstack.delimiter_depth -= 1;
                            dstack.delimiter_depth;
                            if ttok == &mut matched_pair_error as *mut libc::c_char {
                                return -(1 as libc::c_int);
                            }

                            if token_index + (ttoklen + 3 as libc::c_int) >= token_buffer_size {
                                while token_index + (ttoklen + 3 as libc::c_int)
                                    >= token_buffer_size
                                {
                                    token_buffer_size += 512 as libc::c_int;
                                }
                                token =
                                    realloc(token as *mut libc::c_void, token_buffer_size as usize)
                                        as *mut libc::c_char;
                            }

                            let fresh36 = token_index;
                            token_index = token_index + 1;

                            *token.offset(fresh36 as isize) = character as libc::c_char;

                            let fresh37 = token_index;
                            token_index = token_index + 1;

                            *token.offset(fresh37 as isize) = peek_char as libc::c_char;

                            strcpy(token.offset(token_index as isize), ttok);
                            token_index += ttoklen;
                            if !ttok.is_null() {
                                free(ttok as *mut libc::c_void);
                            }
                            ttok = 0 as *mut libc::c_char;
                            all_digit_token = 0 as libc::c_int;
                            dollar_present = all_digit_token;
                            current_block = 2410910991445625210;
                        } else {
                            shell_ungetc(peek_char);
                            current_block = 7072655752890836508;
                        }
                    } else {
                        current_block = 7072655752890836508;
                    }
                    match current_block {
                        2410910991445625210 => {}
                        _ => {
                            if character == '$' as i32
                                || character == '<' as i32
                                || character == '>' as i32
                            {
                                peek_char = shell_getc(1 as libc::c_int);
                                if (peek_char == '(' as i32
                                    || (peek_char == '{' as i32 || peek_char == '[' as i32)
                                        && character == '$' as i32)
                                    && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                        *shell_input_line_property.offset(
                                            shell_input_line_index
                                                .wrapping_sub(1 as libc::c_int as size_t)
                                                as isize,
                                        ) as libc::c_int
                                    } else {
                                        1 as libc::c_int
                                    }) != 0
                                {
                                    if peek_char == '{' as i32 {
                                        ttok = parse_matched_pair(
                                            cd,
                                            '{' as i32,
                                            '}' as i32,
                                            &mut ttoklen,
                                            0x1 as libc::c_int | 0x40 as libc::c_int,
                                        );
                                    } else if peek_char == '(' as i32 {
                                        if dstack.delimiter_depth + 2 as libc::c_int
                                            > dstack.delimiter_space
                                        {
                                            dstack.delimiter_space += 10 as libc::c_int;
                                            dstack.delimiters = realloc(
                                                dstack.delimiters as *mut libc::c_void,
                                                (dstack.delimiter_space as libc::c_ulong)
                                                    .wrapping_mul(
                                                        ::core::mem::size_of::<libc::c_char>()
                                                            as libc::c_ulong,
                                                    )
                                                    as usize,
                                            )
                                                as *mut libc::c_char;
                                        }

                                        *(dstack.delimiters)
                                            .offset(dstack.delimiter_depth as isize) =
                                            peek_char as libc::c_char;
                                        dstack.delimiter_depth += 1;
                                        dstack.delimiter_depth;
                                        ttok = parse_comsub(
                                            cd,
                                            '(' as i32,
                                            ')' as i32,
                                            &mut ttoklen,
                                            0x8 as libc::c_int,
                                        );
                                        dstack.delimiter_depth -= 1;
                                        dstack.delimiter_depth;
                                    } else {
                                        ttok = parse_matched_pair(
                                            cd,
                                            '[' as i32,
                                            ']' as i32,
                                            &mut ttoklen,
                                            0 as libc::c_int,
                                        );
                                    }
                                    if ttok == &mut matched_pair_error as *mut libc::c_char {
                                        return -(1 as libc::c_int);
                                    }
                                    if token_index + (ttoklen + 3 as libc::c_int)
                                        >= token_buffer_size
                                    {
                                        while token_index + (ttoklen + 3 as libc::c_int)
                                            >= token_buffer_size
                                        {
                                            token_buffer_size += 512 as libc::c_int;
                                        }

                                        token = realloc(
                                            token as *mut libc::c_void,
                                            token_buffer_size as usize,
                                        )
                                            as *mut libc::c_char;
                                    }
                                    let fresh38 = token_index;
                                    token_index = token_index + 1;

                                    *token.offset(fresh38 as isize) = character as libc::c_char;

                                    let fresh39 = token_index;
                                    token_index = token_index + 1;

                                    *token.offset(fresh39 as isize) = peek_char as libc::c_char;

                                    strcpy(token.offset(token_index as isize), ttok);
                                    token_index += ttoklen;
                                    if !ttok.is_null() {
                                        free(ttok as *mut libc::c_void);
                                    }
                                    ttok = 0 as *mut libc::c_char;
                                    dollar_present = 1 as libc::c_int;
                                    all_digit_token = 0 as libc::c_int;
                                    current_block = 2410910991445625210;
                                } else if character == '$' as i32
                                    && (peek_char == '\'' as i32 || peek_char == '"' as i32)
                                    && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                        *shell_input_line_property.offset(
                                            shell_input_line_index
                                                .wrapping_sub(1 as libc::c_int as size_t)
                                                as isize,
                                        ) as libc::c_int
                                    } else {
                                        1 as libc::c_int
                                    }) != 0
                                {
                                    let mut first_line: libc::c_int = 0;
                                    first_line = line_number;
                                    if dstack.delimiter_depth + 2 as libc::c_int
                                        > dstack.delimiter_space
                                    {
                                        dstack.delimiter_space += 10 as libc::c_int;
                                        dstack.delimiters = realloc(
                                            dstack.delimiters as *mut libc::c_void,
                                            (dstack.delimiter_space as libc::c_ulong).wrapping_mul(
                                                ::core::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong,
                                            ) as usize,
                                        )
                                            as *mut libc::c_char;
                                    }

                                    *(dstack.delimiters).offset(dstack.delimiter_depth as isize) =
                                        peek_char as libc::c_char;
                                    dstack.delimiter_depth += 1;
                                    dstack.delimiter_depth;

                                    ttok = parse_matched_pair(
                                        peek_char,
                                        peek_char,
                                        peek_char,
                                        &mut ttoklen,
                                        if peek_char == '\'' as i32 {
                                            0x2 as libc::c_int
                                        } else {
                                            0 as libc::c_int
                                        },
                                    );
                                    dstack.delimiter_depth -= 1;
                                    dstack.delimiter_depth;
                                    if ttok == &mut matched_pair_error as *mut libc::c_char {
                                        return -(1 as libc::c_int);
                                    }
                                    if peek_char == '\'' as i32 {
                                        ttrans = c_ansiexpand(
                                            ttok,
                                            0 as libc::c_int,
                                            ttoklen - 1 as libc::c_int,
                                            &mut ttranslen,
                                        );
                                        free(ttok as *mut libc::c_void);
                                        ttok = c_sh_single_quote(ttrans);
                                        free(ttrans as *mut libc::c_void);
                                        ttranslen = libc::strlen(ttok) as libc::c_int;
                                        ttrans = ttok;
                                    } else {
                                        ttrans = localeexpand(
                                            ttok,
                                            0 as libc::c_int,
                                            ttoklen - 1 as libc::c_int,
                                            first_line,
                                            &mut ttranslen,
                                        );

                                        free(ttok as *mut libc::c_void);
                                        ttok = c_sh_mkdoublequoted(
                                            ttrans,
                                            ttranslen,
                                            0 as libc::c_int,
                                        );
                                        free(ttrans as *mut libc::c_void);

                                        ttranslen += 2 as libc::c_int;
                                        ttrans = ttok;
                                    }
                                    if token_index + (ttranslen + 1 as libc::c_int)
                                        >= token_buffer_size
                                    {
                                        while token_index + (ttranslen + 1 as libc::c_int)
                                            >= token_buffer_size
                                        {
                                            token_buffer_size += 512 as libc::c_int;
                                        }
                                        token = realloc(
                                            token as *mut libc::c_void,
                                            token_buffer_size as usize,
                                        )
                                            as *mut libc::c_char;
                                    }
                                    strcpy(token.offset(token_index as isize), ttrans);
                                    token_index += ttranslen;
                                    if !ttrans.is_null() {
                                        free(ttrans as *mut libc::c_void);
                                    }
                                    ttrans = 0 as *mut libc::c_char;
                                    quoted = 1 as libc::c_int;
                                    all_digit_token = 0 as libc::c_int;
                                    current_block = 2410910991445625210;
                                } else if character == '$' as i32
                                    && peek_char == '$' as i32
                                    && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                        *shell_input_line_property.offset(
                                            shell_input_line_index
                                                .wrapping_sub(1 as libc::c_int as size_t)
                                                as isize,
                                        ) as libc::c_int
                                    } else {
                                        1 as libc::c_int
                                    }) != 0
                                {
                                    if token_index + 3 as libc::c_int >= token_buffer_size {
                                        while token_index + 3 as libc::c_int >= token_buffer_size {
                                            token_buffer_size += 512 as libc::c_int;
                                        }

                                        token = realloc(
                                            token as *mut libc::c_void,
                                            token_buffer_size as usize,
                                        )
                                            as *mut libc::c_char;
                                    }
                                    let fresh40 = token_index;
                                    token_index = token_index + 1;

                                    *token.offset(fresh40 as isize) = '$' as i32 as libc::c_char;

                                    let fresh41 = token_index;
                                    token_index = token_index + 1;

                                    *token.offset(fresh41 as isize) = peek_char as libc::c_char;

                                    dollar_present = 1 as libc::c_int;
                                    all_digit_token = 0 as libc::c_int;
                                    current_block = 2410910991445625210;
                                } else {
                                    shell_ungetc(peek_char);
                                    current_block = 11099343707781121639;
                                }
                            } else if character == '[' as i32
                                && (token_index > 0 as libc::c_int
                                    && ((last_read_token == 282 as libc::c_int
                                        || parser_state & 0x80000 as libc::c_int != 0
                                            && (last_read_token == '<' as i32
                                                || last_read_token == '>' as i32
                                                || last_read_token == 290 as libc::c_int
                                                || last_read_token == 302 as libc::c_int
                                                || last_read_token == 301 as libc::c_int
                                                || last_read_token == 298 as libc::c_int
                                                || last_read_token == 291 as libc::c_int
                                                || last_read_token == 293 as libc::c_int
                                                || last_read_token == 292 as libc::c_int
                                                || last_read_token == 294 as libc::c_int
                                                || last_read_token == 299 as libc::c_int)
                                                as libc::c_int
                                                == 0 as libc::c_int
                                        || last_read_token != 295 as libc::c_int
                                            && last_read_token != 296 as libc::c_int
                                            && last_read_token != 297 as libc::c_int
                                            && reserved_word_acceptable(last_read_token) != 0)
                                        && parser_state & 0x1 as libc::c_int == 0 as libc::c_int)
                                    && token_is_ident(token, token_index) != 0
                                    || token_index == 0 as libc::c_int
                                        && parser_state & 0x2000 as libc::c_int != 0)
                                && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                    *shell_input_line_property.offset(
                                        shell_input_line_index
                                            .wrapping_sub(1 as libc::c_int as size_t)
                                            as isize,
                                    ) as libc::c_int
                                } else {
                                    1 as libc::c_int
                                }) != 0
                            {
                                ttok = parse_matched_pair(
                                    cd,
                                    '[' as i32,
                                    ']' as i32,
                                    &mut ttoklen,
                                    0x20 as libc::c_int,
                                );
                                if ttok == &mut matched_pair_error as *mut libc::c_char {
                                    return -(1 as libc::c_int);
                                }
                                if token_index + (ttoklen + 2 as libc::c_int) >= token_buffer_size {
                                    while token_index + (ttoklen + 2 as libc::c_int)
                                        >= token_buffer_size
                                    {
                                        token_buffer_size += 512 as libc::c_int;
                                    }

                                    token = realloc(
                                        token as *mut libc::c_void,
                                        token_buffer_size as usize,
                                    )
                                        as *mut libc::c_char;
                                }
                                let fresh42 = token_index;
                                token_index = token_index + 1;

                                *token.offset(fresh42 as isize) = character as libc::c_char;

                                strcpy(token.offset(token_index as isize), ttok);
                                token_index += ttoklen;
                                if !ttok.is_null() {
                                    free(ttok as *mut libc::c_void);
                                }
                                ttok = 0 as *mut libc::c_char;
                                all_digit_token = 0 as libc::c_int;
                                current_block = 2410910991445625210;
                            } else if character == '=' as i32
                                && token_index > 0 as libc::c_int
                                && ((last_read_token == 282 as libc::c_int
                                    || parser_state & 0x80000 as libc::c_int != 0
                                        && (last_read_token == '<' as i32
                                            || last_read_token == '>' as i32
                                            || last_read_token == 290 as libc::c_int
                                            || last_read_token == 302 as libc::c_int
                                            || last_read_token == 301 as libc::c_int
                                            || last_read_token == 298 as libc::c_int
                                            || last_read_token == 291 as libc::c_int
                                            || last_read_token == 293 as libc::c_int
                                            || last_read_token == 292 as libc::c_int
                                            || last_read_token == 294 as libc::c_int
                                            || last_read_token == 299 as libc::c_int)
                                            as libc::c_int
                                            == 0 as libc::c_int
                                    || last_read_token != 295 as libc::c_int
                                        && last_read_token != 296 as libc::c_int
                                        && last_read_token != 297 as libc::c_int
                                        && reserved_word_acceptable(last_read_token) != 0)
                                    && parser_state & 0x1 as libc::c_int == 0 as libc::c_int
                                    || parser_state & 0x4000 as libc::c_int != 0)
                                && token_is_assignment(token, token_index) != 0
                                && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                    *shell_input_line_property.offset(
                                        shell_input_line_index
                                            .wrapping_sub(1 as libc::c_int as size_t)
                                            as isize,
                                    ) as libc::c_int
                                } else {
                                    1 as libc::c_int
                                }) != 0
                            {
                                peek_char = shell_getc(1 as libc::c_int);
                                if peek_char == '(' as i32
                                    && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                        *shell_input_line_property.offset(
                                            shell_input_line_index
                                                .wrapping_sub(1 as libc::c_int as size_t)
                                                as isize,
                                        ) as libc::c_int
                                    } else {
                                        1 as libc::c_int
                                    }) != 0
                                {
                                    ttok = parse_compound_assignment(&mut ttoklen);
                                    if token_index + (ttoklen + 4 as libc::c_int)
                                        >= token_buffer_size
                                    {
                                        while token_index + (ttoklen + 4 as libc::c_int)
                                            >= token_buffer_size
                                        {
                                            token_buffer_size += 512 as libc::c_int;
                                        }

                                        token = realloc(
                                            token as *mut libc::c_void,
                                            token_buffer_size as usize,
                                        )
                                            as *mut libc::c_char;
                                    }
                                    let fresh43 = token_index;
                                    token_index = token_index + 1;

                                    *token.offset(fresh43 as isize) = '=' as i32 as libc::c_char;

                                    let fresh44 = token_index;
                                    token_index = token_index + 1;

                                    *token.offset(fresh44 as isize) = '(' as i32 as libc::c_char;

                                    if !ttok.is_null() {
                                        strcpy(token.offset(token_index as isize), ttok);
                                        token_index += ttoklen;
                                    }
                                    let fresh45 = token_index;
                                    token_index = token_index + 1;

                                    *token.offset(fresh45 as isize) = ')' as i32 as libc::c_char;

                                    if !ttok.is_null() {
                                        free(ttok as *mut libc::c_void);
                                    }
                                    ttok = 0 as *mut libc::c_char;
                                    all_digit_token = 0 as libc::c_int;
                                    compound_assignment = 1 as libc::c_int;
                                    current_block = 2410910991445625210;
                                } else {
                                    shell_ungetc(peek_char);
                                    current_block = 11099343707781121639;
                                }
                            } else {
                                current_block = 11099343707781121639;
                            }
                            match current_block {
                                2410910991445625210 => {}
                                _ => {
                                    if *sh_syntaxtab
                                        .as_mut_ptr()
                                        .offset(character as libc::c_uchar as isize)
                                        & 0x2 as libc::c_int
                                        != 0
                                        && (if shell_input_line_index > 1 as libc::c_int as size_t {
                                            *shell_input_line_property.offset(
                                                shell_input_line_index
                                                    .wrapping_sub(1 as libc::c_int as size_t)
                                                    as isize,
                                            )
                                                as libc::c_int
                                        } else {
                                            1 as libc::c_int
                                        }) != 0
                                    {
                                        shell_ungetc(character);
                                        break;
                                    } else {
                                        current_block = 11877077252304325915;
                                    }
                                }
                            }
                        }
                    }
                }
                match current_block {
                    2410910991445625210 => {}
                    _ => {
                        if character == '\u{1}' as i32 || character == '\u{7f}' as i32 {
                            if token_index + 2 as libc::c_int >= token_buffer_size {
                                while token_index + 2 as libc::c_int >= token_buffer_size {
                                    token_buffer_size += 512 as libc::c_int;
                                }

                                token =
                                    realloc(token as *mut libc::c_void, token_buffer_size as usize)
                                        as *mut libc::c_char;
                            }
                            let fresh46 = token_index;
                            token_index = token_index + 1;

                            *token.offset(fresh46 as isize) = '\u{1}' as i32 as libc::c_char;

                            current_block = 8115217508953982058;
                        } else {
                            current_block = 2802315260714836178;
                        }
                    }
                }
            }
            match current_block {
                2802315260714836178 => {
                    if token_index + 1 as libc::c_int >= token_buffer_size {
                        while token_index + 1 as libc::c_int >= token_buffer_size {
                            token_buffer_size += 512 as libc::c_int;
                        }

                        token = realloc(token as *mut libc::c_void, token_buffer_size as usize)
                            as *mut libc::c_char;
                    }
                    current_block = 8115217508953982058;
                }
                _ => {}
            }
            match current_block {
                8115217508953982058 => {
                    let fresh47 = token_index;
                    token_index = token_index + 1;

                    *token.offset(fresh47 as isize) = character as libc::c_char;

                    all_digit_token &=
                        (character >= '0' as i32 && character <= '9' as i32) as libc::c_int;
                    dollar_present |= (character == '$' as i32) as libc::c_int;
                }
                _ => {}
            }
            if character == '\n' as i32
                && (interactive != 0
                    && (bash_input.type_0 as libc::c_uint
                        == st_stdin as libc::c_int as libc::c_uint
                        || bash_input.type_0 as libc::c_uint
                            == st_stream as libc::c_int as libc::c_uint))
            {
                prompt_again();
            }
            cd = if dstack.delimiter_depth != 0 {
                *(dstack.delimiters).offset((dstack.delimiter_depth - 1 as libc::c_int) as isize)
                    as libc::c_int
            } else {
                0 as libc::c_int
            };
            character = shell_getc(
                (cd != '\'' as i32 && pass_next_character == 0 as libc::c_int) as libc::c_int,
            );
        }

        *token.offset(token_index as isize) = '\0' as i32 as libc::c_char;

        if all_digit_token != 0
            && (character == '<' as i32
                || character == '>' as i32
                || last_read_token == 292 as libc::c_int
                || last_read_token == 294 as libc::c_int)
            && (if shell_input_line_index > 1 as libc::c_int as size_t {
                *shell_input_line_property.offset(
                    shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t) as isize,
                ) as libc::c_int
            } else {
                1 as libc::c_int
            }) != 0
        {
            if legal_number(token, &mut lvalue) != 0 && lvalue as libc::c_int as intmax_t == lvalue
            {
                yylval.number = lvalue as libc::c_int;
                return 284 as libc::c_int;
            }
        }
        result = if if shell_input_line_index > 1 as libc::c_int as size_t {
            *shell_input_line_property
                .offset(shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t) as isize)
                as libc::c_int
        } else {
            1 as libc::c_int
        } != 0
        {
            special_case_tokens(token)
        } else {
            -(1 as libc::c_int)
        };
        if result >= 0 as libc::c_int {
            return result;
        }
        if posixly_correct != 0
            && (if shell_input_line_index > 1 as libc::c_int as size_t {
                *shell_input_line_property.offset(
                    shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t) as isize,
                ) as libc::c_int
            } else {
                1 as libc::c_int
            }) != 0
        {
            if dollar_present == 0 && quoted == 0 && reserved_word_acceptable(last_read_token) != 0
            {
                let mut i: libc::c_int = 0;
                i = 0 as libc::c_int;

                while !(word_token_alist[i as usize].word).is_null() {
                    if *token.offset(0 as libc::c_int as isize) as libc::c_int
                        == *(word_token_alist[i as usize].word).offset(0 as libc::c_int as isize)
                            as libc::c_int
                        && strcmp(token, word_token_alist[i as usize].word) == 0 as libc::c_int
                    {
                        if parser_state & 0x1 as libc::c_int != 0
                            && word_token_alist[i as usize].token != 264 as libc::c_int
                        {
                            break;
                        }
                        if word_token_alist[i as usize].token == 278 as libc::c_int
                            && time_command_acceptable() == 0 as libc::c_int
                        {
                            break;
                        }
                        if parser_state & 0x1 as libc::c_int != 0
                            && last_read_token == '|' as i32
                            && word_token_alist[i as usize].token == 264 as libc::c_int
                        {
                            break;
                        }
                        if word_token_alist[i as usize].token == 264 as libc::c_int {
                            parser_state &= !(0x1 as libc::c_int | 0x80 as libc::c_int);
                        } else if word_token_alist[i as usize].token == 263 as libc::c_int {
                            parser_state |= 0x80 as libc::c_int;
                        } else if word_token_alist[i as usize].token == 274 as libc::c_int {
                            parser_state &= !(0x100 as libc::c_int | 0x200 as libc::c_int);
                        } else if word_token_alist[i as usize].token == 273 as libc::c_int {
                            parser_state |= 0x100 as libc::c_int;
                        } else if word_token_alist[i as usize].token == '{' as i32 {
                            open_brace_count += 1;
                        } else if word_token_alist[i as usize].token == '}' as i32
                            && open_brace_count != 0
                        {
                            open_brace_count -= 1;
                        }
                        return word_token_alist[i as usize].token;
                    } else {
                        i += 1;
                    }
                }
            }
        }
        if expand_aliases != 0 && quoted == 0 as libc::c_int {
            result = alias_expand_token(token);
            if result == -(99 as libc::c_int) {
                return -(99 as libc::c_int);
            } else if result == -(100 as libc::c_int) {
                parser_state &= !(0x2 as libc::c_int);
            }
        }
        if posixly_correct == 0 as libc::c_int
            && (if shell_input_line_index > 1 as libc::c_int as size_t {
                *shell_input_line_property.offset(
                    shell_input_line_index.wrapping_sub(1 as libc::c_int as size_t) as isize,
                ) as libc::c_int
            } else {
                1 as libc::c_int
            }) != 0
        {
            if dollar_present == 0 && quoted == 0 && reserved_word_acceptable(last_read_token) != 0
            {
                let mut i_0: libc::c_int = 0;
                i_0 = 0 as libc::c_int;
                while !(word_token_alist[i_0 as usize].word).is_null() {
                    if *token.offset(0 as libc::c_int as isize) as libc::c_int
                        == *(word_token_alist[i_0 as usize].word).offset(0 as libc::c_int as isize)
                            as libc::c_int
                        && strcmp(token, word_token_alist[i_0 as usize].word) == 0 as libc::c_int
                    {
                        if parser_state & 0x1 as libc::c_int != 0
                            && word_token_alist[i_0 as usize].token != 264 as libc::c_int
                        {
                            break;
                        }
                        if word_token_alist[i_0 as usize].token == 278 as libc::c_int
                            && time_command_acceptable() == 0 as libc::c_int
                        {
                            break;
                        }
                        if parser_state & 0x1 as libc::c_int != 0
                            && last_read_token == '|' as i32
                            && word_token_alist[i_0 as usize].token == 264 as libc::c_int
                        {
                            break;
                        }
                        if word_token_alist[i_0 as usize].token == 264 as libc::c_int {
                            parser_state &= !(0x1 as libc::c_int | 0x80 as libc::c_int);
                        } else if word_token_alist[i_0 as usize].token == 263 as libc::c_int {
                            parser_state |= 0x80 as libc::c_int;
                        } else if word_token_alist[i_0 as usize].token == 274 as libc::c_int {
                            parser_state &= !(0x100 as libc::c_int | 0x200 as libc::c_int);
                        } else if word_token_alist[i_0 as usize].token == 273 as libc::c_int {
                            parser_state |= 0x100 as libc::c_int;
                        } else if word_token_alist[i_0 as usize].token == '{' as i32 {
                            open_brace_count += 1;
                        } else if word_token_alist[i_0 as usize].token == '}' as i32
                            && open_brace_count != 0
                        {
                            open_brace_count -= 1;
                        }
                        return word_token_alist[i_0 as usize].token;
                    } else {
                        i_0 += 1;
                    }
                }
            }
        }
        the_word = alloc_word_desc();

        (*the_word).word = malloc((1 as libc::c_int + token_index) as usize) as *mut libc::c_char;
        (*the_word).flags = 0 as libc::c_int;
        strcpy((*the_word).word, token);
        if dollar_present != 0 {
            (*the_word).flags |= (1 as libc::c_int) << 0 as libc::c_int;
        }
        if quoted != 0 {
            (*the_word).flags |= (1 as libc::c_int) << 1 as libc::c_int;
        }
        if compound_assignment != 0
            && *token.offset((token_index - 1 as libc::c_int) as isize) as libc::c_int == ')' as i32
        {
            (*the_word).flags |= (1 as libc::c_int) << 15 as libc::c_int;
        }
        if assignment(
            token,
            (parser_state & 0x2000 as libc::c_int != 0 as libc::c_int) as libc::c_int,
        ) != 0
        {
            (*the_word).flags |= (1 as libc::c_int) << 2 as libc::c_int;
            if (last_read_token == 282 as libc::c_int
                || parser_state & 0x80000 as libc::c_int != 0
                    && (last_read_token == '<' as i32
                        || last_read_token == '>' as i32
                        || last_read_token == 290 as libc::c_int
                        || last_read_token == 302 as libc::c_int
                        || last_read_token == 301 as libc::c_int
                        || last_read_token == 298 as libc::c_int
                        || last_read_token == 291 as libc::c_int
                        || last_read_token == 293 as libc::c_int
                        || last_read_token == 292 as libc::c_int
                        || last_read_token == 294 as libc::c_int
                        || last_read_token == 299 as libc::c_int)
                        as libc::c_int
                        == 0 as libc::c_int
                || last_read_token != 295 as libc::c_int
                    && last_read_token != 296 as libc::c_int
                    && last_read_token != 297 as libc::c_int
                    && reserved_word_acceptable(last_read_token) != 0)
                && parser_state & 0x1 as libc::c_int == 0 as libc::c_int
                || parser_state & 0x2000 as libc::c_int != 0 as libc::c_int
            {
                (*the_word).flags |= (1 as libc::c_int) << 4 as libc::c_int;
                if parser_state & 0x2000 as libc::c_int != 0 {
                    (*the_word).flags |= (1 as libc::c_int) << 5 as libc::c_int;
                }
            }
        }
        if last_read_token == 282 as libc::c_int
            || parser_state & 0x80000 as libc::c_int != 0
                && (last_read_token == '<' as i32
                    || last_read_token == '>' as i32
                    || last_read_token == 290 as libc::c_int
                    || last_read_token == 302 as libc::c_int
                    || last_read_token == 301 as libc::c_int
                    || last_read_token == 298 as libc::c_int
                    || last_read_token == 291 as libc::c_int
                    || last_read_token == 293 as libc::c_int
                    || last_read_token == 292 as libc::c_int
                    || last_read_token == 294 as libc::c_int
                    || last_read_token == 299 as libc::c_int) as libc::c_int
                    == 0 as libc::c_int
            || last_read_token != 295 as libc::c_int
                && last_read_token != 296 as libc::c_int
                && last_read_token != 297 as libc::c_int
                && reserved_word_acceptable(last_read_token) != 0
        {
            let mut b: *mut builtin = 0 as *mut builtin;
            b = builtin_address_internal(token, 0 as libc::c_int);
            if !b.is_null() && (*b).flags & 0x10 as libc::c_int != 0 {
                parser_state |= 0x4000 as libc::c_int;
            } else if *token.offset(0 as libc::c_int as isize) as libc::c_int
                == (*::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"eval\0"))
                    [0 as libc::c_int as usize] as libc::c_int
                && strcmp(token, b"eval\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
                || *token.offset(0 as libc::c_int as isize) as libc::c_int
                    == (*::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"let\0"))
                        [0 as libc::c_int as usize] as libc::c_int
                    && strcmp(token, b"let\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
            {
                parser_state |= 0x4000 as libc::c_int;
            }
        }
        yylval.word = the_word;
        if *token.offset(0 as libc::c_int as isize) as libc::c_int == '{' as i32
            && *token.offset((token_index - 1 as libc::c_int) as isize) as libc::c_int == '}' as i32
            && (character == '<' as i32 || character == '>' as i32)
        {
            *token.offset((token_index - 1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
            if legal_identifier(token.offset(1 as libc::c_int as isize)) != 0
                || valid_array_reference(token.offset(1 as libc::c_int as isize), 0 as libc::c_int)
                    != 0
            {
                strcpy((*the_word).word, token.offset(1 as libc::c_int as isize));
                yylval.word = the_word;
                return 283 as libc::c_int;
            } else {
                yylval.word = the_word;
            }
        }

        result = if (*the_word).flags
            & ((1 as libc::c_int) << 2 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int)
            == (1 as libc::c_int) << 2 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
        {
            282 as libc::c_int
        } else {
            281 as libc::c_int
        };

        match last_read_token {
            271 => {
                parser_state |= 0x4 as libc::c_int;
                function_dstart = line_number;
            }
            263 | 266 | 265 => {
                if word_top < 128 as libc::c_int {
                    word_top += 1;
                }
                word_lineno[word_top as usize] = line_number;
                expecting_in_token += 1;
            }
            _ => {}
        }

        return result;
    }
}
fn reserved_word_acceptable(mut toksym: libc::c_int) -> libc::c_int {
    unsafe {
        match toksym {
            10 | 59 | 40 | 41 | 124 | 38 | 123 | 125 | 288 | 277 | 303 | 269 | 270 | 261 | 260
            | 264 | 262 | 258 | 289 | 295 | 296 | 297 | 259 | 278 | 279 | 280 | 272 | 268 | 267
            | 0 => return 1 as libc::c_int,
            _ => {
                if last_read_token == 281 as libc::c_int && token_before_that == 272 as libc::c_int
                {
                    return 1 as libc::c_int;
                }
                if last_read_token == 281 as libc::c_int && token_before_that == 271 as libc::c_int
                {
                    return 1 as libc::c_int;
                }
                return 0 as libc::c_int;
            }
        };
    }
}
#[no_mangle]
pub fn find_reserved_word(mut tokstr: *mut libc::c_char) -> libc::c_int {
    unsafe {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while !(word_token_alist[i as usize].word).is_null() {
            if *tokstr.offset(0 as libc::c_int as isize) as libc::c_int
                == *(word_token_alist[i as usize].word).offset(0 as libc::c_int as isize)
                    as libc::c_int
                && strcmp(tokstr, word_token_alist[i as usize].word) == 0 as libc::c_int
            {
                return i;
            }
            i += 1;
        }
        return -(1 as libc::c_int);
    }
}
#[no_mangle]
pub fn parser_in_command_position() -> libc::c_int {
    unsafe {
        return (last_read_token == 282 as libc::c_int
            || parser_state & 0x80000 as libc::c_int != 0
                && (last_read_token == '<' as i32
                    || last_read_token == '>' as i32
                    || last_read_token == 290 as libc::c_int
                    || last_read_token == 302 as libc::c_int
                    || last_read_token == 301 as libc::c_int
                    || last_read_token == 298 as libc::c_int
                    || last_read_token == 291 as libc::c_int
                    || last_read_token == 293 as libc::c_int
                    || last_read_token == 292 as libc::c_int
                    || last_read_token == 294 as libc::c_int
                    || last_read_token == 299 as libc::c_int) as libc::c_int
                    == 0 as libc::c_int
            || last_read_token != 295 as libc::c_int
                && last_read_token != 296 as libc::c_int
                && last_read_token != 297 as libc::c_int
                && reserved_word_acceptable(last_read_token) != 0) as libc::c_int;
    }
}
static mut no_semi_successors: [libc::c_int; 21] = [
    '\n' as i32,
    '{' as i32,
    '(' as i32,
    ')' as i32,
    ';' as i32,
    '&' as i32,
    '|' as i32,
    263 as libc::c_int,
    269 as libc::c_int,
    260 as libc::c_int,
    258 as libc::c_int,
    295 as libc::c_int,
    296 as libc::c_int,
    297 as libc::c_int,
    259 as libc::c_int,
    268 as libc::c_int,
    267 as libc::c_int,
    288 as libc::c_int,
    289 as libc::c_int,
    276 as libc::c_int,
    0 as libc::c_int,
];
#[no_mangle]
pub fn history_delimiting_chars(mut line: *const libc::c_char) -> *mut libc::c_char {
    unsafe {
        static mut last_was_heredoc: libc::c_int = 0 as libc::c_int;
        let mut i: libc::c_int = 0;

        if parser_state & 0x20000 as libc::c_int == 0 as libc::c_int {
            last_was_heredoc = 0 as libc::c_int;
        }
        if dstack.delimiter_depth != 0 as libc::c_int {
            return b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        if parser_state & 0x20000 as libc::c_int != 0 {
            if last_was_heredoc != 0 {
                last_was_heredoc = 0 as libc::c_int;
                return b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            return (if here_doc_first_line != 0 {
                b"\n\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char;
        }
        if parser_state & 0x2000 as libc::c_int != 0 {
            return b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        if token_before_that == ')' as i32 {
            if two_tokens_ago == '(' as i32 {
                return b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            } else if parser_state & 0x80 as libc::c_int != 0 {
                return b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            } else {
                return b"; \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
        } else if token_before_that == 281 as libc::c_int && two_tokens_ago == 271 as libc::c_int {
            return b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else if parser_state & 0x20000 as libc::c_int == 0 as libc::c_int
            && current_command_line_count > 1 as libc::c_int
            && last_read_token == '\n' as i32
            && !(strstr(line, b"<<\0" as *const u8 as *const libc::c_char)).is_null()
        {
            last_was_heredoc = 1 as libc::c_int;
            return b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else if parser_state & 0x20000 as libc::c_int == 0 as libc::c_int
            && current_command_line_count > 1 as libc::c_int
            && need_here_doc > 0 as libc::c_int
        {
            return b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else if token_before_that == 281 as libc::c_int && two_tokens_ago == 265 as libc::c_int {
            i = shell_input_line_index as libc::c_int;
            while *shell_input_line.offset(i as isize) as libc::c_int == ' ' as i32
                || *shell_input_line.offset(i as isize) as libc::c_int == '\t' as i32
            {
                i += 1;
            }
            if *shell_input_line.offset(i as isize) as libc::c_int != 0
                && *shell_input_line.offset(i as isize) as libc::c_int == 'i' as i32
                && *shell_input_line.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                    == 'n' as i32
            {
                return b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            return b";\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else if two_tokens_ago == 263 as libc::c_int
            && token_before_that == 281 as libc::c_int
            && parser_state & 0x80 as libc::c_int != 0
        {
            return b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        i = 0 as libc::c_int;

        while no_semi_successors[i as usize] != 0 {
            if token_before_that == no_semi_successors[i as usize] {
                return b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            i += 1;
        }
        if line_isblank(line) != 0 {
            return b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        return b"; \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
}
fn prompt_again() {
    unsafe {
        let mut temp_prompt: *mut libc::c_char = 0 as *mut libc::c_char;
        if interactive == 0 as libc::c_int
            || !pushed_string_list.is_null() && !((*pushed_string_list).expander).is_null()
        {
            return;
        }
        ps1_prompt = get_string_value(b"PS1\0" as *const u8 as *const libc::c_char);
        ps2_prompt = get_string_value(b"PS2\0" as *const u8 as *const libc::c_char);
        ps0_prompt = get_string_value(b"PS0\0" as *const u8 as *const libc::c_char);
        if prompt_string_pointer.is_null() {
            prompt_string_pointer = &mut ps1_prompt;
        }
        temp_prompt = if !(*prompt_string_pointer).is_null() {
            decode_prompt_string(*prompt_string_pointer)
        } else {
            0 as *mut libc::c_void as *mut libc::c_char
        };
        if temp_prompt.is_null() {
            temp_prompt = malloc(1 as libc::c_int as usize) as *mut libc::c_char;
            *temp_prompt.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        }
        current_prompt_string = *prompt_string_pointer;
        prompt_string_pointer = &mut ps2_prompt;
        if no_line_editing == 0 {
            if !current_readline_prompt.is_null() {
                free(current_readline_prompt as *mut libc::c_void);
            }
            current_readline_prompt = 0 as *mut libc::c_char;
            current_readline_prompt = temp_prompt;
        } else {
            if !current_decoded_prompt.is_null() {
                free(current_decoded_prompt as *mut libc::c_void);
            }
            current_decoded_prompt = 0 as *mut libc::c_char;
            current_decoded_prompt = temp_prompt;
        };
    }
}
#[no_mangle]
pub fn get_current_prompt_level() -> libc::c_int {
    unsafe {
        return if !current_prompt_string.is_null() && current_prompt_string == ps2_prompt {
            2 as libc::c_int
        } else {
            1 as libc::c_int
        };
    }
}
#[no_mangle]
pub fn set_current_prompt_level(mut x: libc::c_int) {
    unsafe {
        prompt_string_pointer = if x == 2 as libc::c_int {
            &mut ps2_prompt
        } else {
            &mut ps1_prompt
        };
        current_prompt_string = *prompt_string_pointer;
    }
}
fn print_prompt() {
    unsafe {
        fprintf(
            stderr,
            b"%s\0" as *const u8 as *const libc::c_char,
            current_decoded_prompt,
        );
        fflush(stderr);
    }
}
fn prompt_history_number(mut pmt: *mut libc::c_char) -> libc::c_int {
    unsafe {
        let mut ret: libc::c_int = 0;
        ret = history_number();
        if ret == 1 as libc::c_int {
            return ret;
        }
        if pmt == ps1_prompt {
            return ret;
        } else if pmt == ps2_prompt && command_oriented_history == 0 as libc::c_int {
            return ret;
        } else if pmt == ps2_prompt
            && command_oriented_history != 0
            && current_command_first_line_saved != 0
        {
            return ret - 1 as libc::c_int;
        } else {
            return ret - 1 as libc::c_int;
        };
    }
}
#[no_mangle]
pub fn decode_prompt_string(mut string: *mut libc::c_char) -> *mut libc::c_char {
    unsafe {
        let mut current_block: u64;
        let mut list: *mut WORD_LIST = 0 as *mut WORD_LIST;
        let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut orig_string: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut save_dstack: dstack = dstack {
            delimiters: 0 as *mut libc::c_char,
            delimiter_depth: 0,
            delimiter_space: 0,
        };
        let mut last_exit_value: libc::c_int = 0;
        let mut last_comsub_pid: libc::c_int = 0;
        let mut result_size: size_t = 0;
        let mut result_index: libc::c_int = 0;
        let mut c: libc::c_int = 0;
        let mut n: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut t_host: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut octal_string: [libc::c_char; 4] = [0; 4];
        let mut tm: *mut tm = 0 as *mut tm;
        let mut the_time: time_t = 0;
        let mut timebuf: [libc::c_char; 128] = [0; 128];
        let mut timefmt: *mut libc::c_char = 0 as *mut libc::c_char;
        result_size = 48 as libc::c_int as size_t;
        result = malloc(result_size as usize) as *mut libc::c_char;
        result_index = 0 as libc::c_int;

        *result.offset(result_index as isize) = 0 as libc::c_int as libc::c_char;

        temp = 0 as *mut libc::c_void as *mut libc::c_char;
        orig_string = string;
        loop {
            let fresh48 = string;

            string = string.offset(1);

            c = *fresh48 as libc::c_int;
            if !(c != 0) {
                break;
            }
            if posixly_correct != 0 && c == '!' as i32 {
                if *string as libc::c_int == '!' as i32 {
                    temp = strcpy(
                        malloc(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(libc::strlen(
                                    b"!\0" as *const u8 as *const libc::c_char,
                                ) as u64) as usize,
                        ) as *mut libc::c_char,
                        b"!\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    temp = c_itos(prompt_history_number(orig_string) as intmax_t);

                    string = string.offset(-1);
                }
            } else if c == '\\' as i32 {
                c = *string as libc::c_int;
                match c {
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
                        strncpy(octal_string.as_mut_ptr(), string, 3 as libc::c_int as usize);
                        octal_string[3 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                        n = read_octal(octal_string.as_mut_ptr());
                        temp = malloc(3 as libc::c_int as usize) as *mut libc::c_char;

                        if n == '\u{1}' as i32 || n == '\u{7f}' as i32 {
                            *temp.offset(0 as libc::c_int as isize) =
                                '\u{1}' as i32 as libc::c_char;
                            *temp.offset(1 as libc::c_int as isize) = n as libc::c_char;
                            *temp.offset(2 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                        } else if n == -(1 as libc::c_int) {
                            *temp.offset(0 as libc::c_int as isize) = '\\' as i32 as libc::c_char;
                            *temp.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                        } else {
                            *temp.offset(0 as libc::c_int as isize) = n as libc::c_char;
                            *temp.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                        }

                        c = 0 as libc::c_int;
                        while n != -(1 as libc::c_int)
                            && c < 3 as libc::c_int
                            && (*string as libc::c_int >= '0' as i32
                                && *string as libc::c_int <= '7' as i32)
                        {
                            string = string.offset(1);

                            c += 1;
                        }
                        c = 0 as libc::c_int;
                        current_block = 6294327190052088242;
                    }
                    100 | 116 | 84 | 64 | 65 => {
                        time(&mut the_time);
                        sv_tz(b"TZ\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
                        tm = localtime(&mut the_time) as *mut src_common::tm;
                        if c == 'd' as i32 {
                            n = strftime(
                                timebuf.as_mut_ptr(),
                                ::core::mem::size_of::<[libc::c_char; 128]>() as usize,
                                b"%a %b %d\0" as *const u8 as *const libc::c_char,
                                tm as *const libc::tm,
                            ) as libc::c_int;
                        } else if c == 't' as i32 {
                            n = strftime(
                                timebuf.as_mut_ptr(),
                                ::core::mem::size_of::<[libc::c_char; 128]>() as usize,
                                b"%H:%M:%S\0" as *const u8 as *const libc::c_char,
                                tm as *const libc::tm,
                            ) as libc::c_int;
                        } else if c == 'T' as i32 {
                            n = strftime(
                                timebuf.as_mut_ptr(),
                                ::core::mem::size_of::<[libc::c_char; 128]>() as usize,
                                b"%I:%M:%S\0" as *const u8 as *const libc::c_char,
                                tm as *const libc::tm,
                            ) as libc::c_int;
                        } else if c == '@' as i32 {
                            n = strftime(
                                timebuf.as_mut_ptr(),
                                ::core::mem::size_of::<[libc::c_char; 128]>() as usize,
                                b"%I:%M %p\0" as *const u8 as *const libc::c_char,
                                tm as *const libc::tm,
                            ) as libc::c_int;
                        } else if c == 'A' as i32 {
                            n = strftime(
                                timebuf.as_mut_ptr(),
                                ::core::mem::size_of::<[libc::c_char; 128]>() as usize,
                                b"%H:%M\0" as *const u8 as *const libc::c_char,
                                tm as *const libc::tm,
                            ) as libc::c_int;
                        }
                        if n == 0 as libc::c_int {
                            timebuf[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                        } else {
                            timebuf[(::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as usize] = '\0' as i32 as libc::c_char;
                        }
                        temp = strcpy(
                            malloc(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(libc::strlen(timebuf.as_mut_ptr()) as u64)
                                    as usize,
                            ) as *mut libc::c_char,
                            timebuf.as_mut_ptr(),
                        );
                        current_block = 6294327190052088242;
                    }
                    68 => {
                        if *string.offset(1 as libc::c_int as isize) as libc::c_int != '{' as i32 {
                            current_block = 3323500006713522785;
                        } else {
                            time(&mut the_time);
                            tm = localtime(&mut the_time) as *mut src_common::tm;

                            string = string.offset(2 as libc::c_int as isize);

                            timefmt = malloc((libc::strlen(string)).wrapping_add(3) as usize)
                                as *mut libc::c_char;
                            t = timefmt;
                            while *string as libc::c_int != 0
                                && *string as libc::c_int != '}' as i32
                            {
                                let fresh49 = string;

                                string = string.offset(1);

                                let fresh50 = t;

                                t = t.offset(1);

                                *fresh50 = *fresh49;
                            }
                            *t = '\0' as i32 as libc::c_char;
                            c = *string as libc::c_int;
                            if *timefmt.offset(0 as libc::c_int as isize) as libc::c_int
                                == '\0' as i32
                            {
                                *timefmt.offset(0 as libc::c_int as isize) =
                                    '%' as i32 as libc::c_char;
                                *timefmt.offset(1 as libc::c_int as isize) =
                                    'X' as i32 as libc::c_char;
                                *timefmt.offset(2 as libc::c_int as isize) =
                                    '\0' as i32 as libc::c_char;
                            }
                            n = strftime(
                                timebuf.as_mut_ptr(),
                                ::core::mem::size_of::<[libc::c_char; 128]>() as usize,
                                timefmt,
                                tm as *const libc::tm,
                            ) as libc::c_int;
                            free(timefmt as *mut libc::c_void);
                            if n == 0 as libc::c_int {
                                timebuf[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                            } else {
                                timebuf[(::core::mem::size_of::<[libc::c_char; 128]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    as usize] = '\0' as i32 as libc::c_char;
                            }
                            if promptvars != 0 || posixly_correct != 0 {
                                temp = c_sh_backslash_quote_for_double_quotes(timebuf.as_mut_ptr());
                            } else {
                                temp = strcpy(
                                    malloc(
                                        (1 as libc::c_int as libc::c_ulong)
                                            .wrapping_add(libc::strlen(timebuf.as_mut_ptr()) as u64)
                                            as usize,
                                    ) as *mut libc::c_char,
                                    timebuf.as_mut_ptr(),
                                );
                            }
                            current_block = 6294327190052088242;
                        }
                    }
                    110 => {
                        temp = malloc(3 as libc::c_int as usize) as *mut libc::c_char;
                        *temp.offset(0 as libc::c_int as isize) = (if no_line_editing != 0 {
                            '\n' as i32
                        } else {
                            '\r' as i32
                        })
                            as libc::c_char;
                        *temp.offset(1 as libc::c_int as isize) = (if no_line_editing != 0 {
                            '\0' as i32
                        } else {
                            '\n' as i32
                        })
                            as libc::c_char;
                        *temp.offset(2 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                        current_block = 6294327190052088242;
                    }
                    115 => {
                        temp = base_pathname(shell_name);
                        if promptvars != 0 || posixly_correct != 0 {
                            temp = c_sh_backslash_quote_for_double_quotes(temp);
                        } else {
                            temp = strcpy(
                                malloc(
                                    (1 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(libc::strlen(temp) as u64)
                                        as usize,
                                ) as *mut libc::c_char,
                                temp,
                            );
                        }
                        current_block = 6294327190052088242;
                    }
                    118 | 86 => {
                        temp = malloc(16 as libc::c_int as usize) as *mut libc::c_char;
                        if c == 'v' as i32 {
                            strcpy(temp, dist_version);
                        } else {
                            sprintf(
                                temp,
                                b"%s.%d\0" as *const u8 as *const libc::c_char,
                                dist_version,
                                patch_level,
                            );
                        }
                        current_block = 6294327190052088242;
                    }
                    119 | 87 => {
                        let mut t_string: [libc::c_char; 4096] = [0; 4096];
                        let mut tlen: libc::c_int = 0;
                        temp = get_string_value(b"PWD\0" as *const u8 as *const libc::c_char);
                        if temp.is_null() {
                            if (getcwd(
                                t_string.as_mut_ptr(),
                                ::core::mem::size_of::<[libc::c_char; 4096]>() as usize,
                            ))
                            .is_null()
                            {
                                t_string[0 as libc::c_int as usize] = '.' as i32 as libc::c_char;
                                tlen = 1 as libc::c_int;
                            } else {
                                tlen = libc::strlen(t_string.as_mut_ptr()) as libc::c_int;
                            }
                        } else {
                            tlen = (::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as libc::c_int;
                            strncpy(t_string.as_mut_ptr(), temp, tlen as usize);
                        }
                        t_string[tlen as usize] = '\0' as i32 as libc::c_char;
                        if c == 'W' as i32 && {
                            t = get_string_value(b"HOME\0" as *const u8 as *const libc::c_char);
                            t.is_null()
                                || (*t.offset(0 as libc::c_int as isize) as libc::c_int
                                    == t_string[0 as libc::c_int as usize] as libc::c_int
                                    && strcmp(t, t_string.as_mut_ptr()) == 0 as libc::c_int)
                                    as libc::c_int
                                    == 0 as libc::c_int
                        } {
                            if (t_string[0 as libc::c_int as usize] as libc::c_int == '/' as i32
                                && t_string[1 as libc::c_int as usize] as libc::c_int
                                    == 0 as libc::c_int)
                                as libc::c_int
                                == 0 as libc::c_int
                                && (t_string[0 as libc::c_int as usize] as libc::c_int
                                    == '/' as i32
                                    && t_string[1 as libc::c_int as usize] as libc::c_int
                                        == '/' as i32
                                    && t_string[2 as libc::c_int as usize] as libc::c_int
                                        == 0 as libc::c_int)
                                    as libc::c_int
                                    == 0 as libc::c_int
                            {
                                t = strrchr(t_string.as_mut_ptr(), '/' as i32);
                                if !t.is_null() {
                                    memmove(
                                        t_string.as_mut_ptr() as *mut libc::c_void,
                                        t.offset(1 as libc::c_int as isize) as *const libc::c_void,
                                        libc::strlen(t) as usize,
                                    );
                                }
                            }
                        } else {
                            temp = polite_directory_format(t_string.as_mut_ptr());
                            if temp != t_string.as_mut_ptr() {
                                strcpy(t_string.as_mut_ptr(), temp);
                            }
                        }
                        temp = trim_pathname(
                            t_string.as_mut_ptr(),
                            4096 as libc::c_int - 1 as libc::c_int,
                        );
                        if promptvars != 0 || posixly_correct != 0 {
                            temp = c_sh_backslash_quote_for_double_quotes(t_string.as_mut_ptr());
                        } else {
                            temp = strcpy(
                                malloc(
                                    (1 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(libc::strlen(t_string.as_mut_ptr()) as u64)
                                        as usize,
                                ) as *mut libc::c_char,
                                t_string.as_mut_ptr(),
                            );
                        }
                        current_block = 6294327190052088242;
                    }
                    117 => {
                        if (current_user.user_name).is_null() {
                            get_current_user_info();
                        }
                        temp = strcpy(
                            malloc(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(libc::strlen(current_user.user_name) as u64)
                                    as usize,
                            ) as *mut libc::c_char,
                            current_user.user_name,
                        );
                        current_block = 6294327190052088242;
                    }
                    104 | 72 => {
                        t_host = strcpy(
                            malloc(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(libc::strlen(current_host_name) as u64)
                                    as usize,
                            ) as *mut libc::c_char,
                            current_host_name,
                        );
                        if c == 'h' as i32 && {
                            t = strchr(t_host, '.' as i32);
                            !t.is_null()
                        } {
                            *t = '\0' as i32 as libc::c_char;
                        }
                        if promptvars != 0 || posixly_correct != 0 {
                            temp = c_sh_backslash_quote_for_double_quotes(t_host);
                        } else {
                            temp = strcpy(
                                malloc(
                                    (1 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(libc::strlen(t_host) as u64)
                                        as usize,
                                ) as *mut libc::c_char,
                                t_host,
                            );
                        }
                        free(t_host as *mut libc::c_void);
                        current_block = 6294327190052088242;
                    }
                    35 => {
                        n = current_command_number;
                        if orig_string != ps0_prompt
                            && orig_string != ps1_prompt
                            && orig_string != ps2_prompt
                        {
                            n -= 1;
                        }
                        temp = c_itos(n as intmax_t);
                        current_block = 6294327190052088242;
                    }
                    33 => {
                        temp = c_itos(prompt_history_number(orig_string) as intmax_t);
                        current_block = 6294327190052088242;
                    }
                    36 => {
                        temp = malloc(3 as libc::c_int as usize) as *mut libc::c_char;
                        t = temp;
                        if (promptvars != 0 || posixly_correct != 0)
                            && current_user.euid != 0 as libc::c_int as uid_t
                        {
                            let fresh51 = t;

                            t = t.offset(1);

                            *fresh51 = '\\' as i32 as libc::c_char;
                        }
                        let fresh52 = t;

                        t = t.offset(1);

                        *fresh52 = (if current_user.euid == 0 as libc::c_int as uid_t {
                            '#' as i32
                        } else {
                            '$' as i32
                        }) as libc::c_char;
                        *t = '\0' as i32 as libc::c_char;
                        current_block = 6294327190052088242;
                    }
                    106 => {
                        temp = c_itos(count_all_jobs() as intmax_t);
                        current_block = 6294327190052088242;
                    }
                    108 => {
                        temp = ttyname(fileno(stdin));
                        t = (if !temp.is_null() {
                            base_pathname(temp) as *const libc::c_char
                        } else {
                            b"tty\0" as *const u8 as *const libc::c_char
                        }) as *mut libc::c_char;
                        temp = strcpy(
                            malloc(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(libc::strlen(t) as u64)
                                    as usize,
                            ) as *mut libc::c_char,
                            t,
                        );
                        current_block = 6294327190052088242;
                    }
                    91 | 93 => {
                        if no_line_editing != 0 {
                            string = string.offset(1);

                            continue;
                        } else {
                            temp = malloc(3 as libc::c_int as usize) as *mut libc::c_char;
                            n = if c == '[' as i32 {
                                '\u{1}' as i32
                            } else {
                                '\u{2}' as i32
                            };
                            i = 0 as libc::c_int;
                            if n == '\u{1}' as i32 || n == '\u{7f}' as i32 {
                                let fresh53 = i;
                                i = i + 1;

                                *temp.offset(fresh53 as isize) = '\u{1}' as i32 as libc::c_char;
                            }

                            let fresh54 = i;
                            i = i + 1;

                            *temp.offset(fresh54 as isize) = n as libc::c_char;
                            *temp.offset(i as isize) = '\0' as i32 as libc::c_char;
                        }
                        current_block = 6294327190052088242;
                    }
                    92 | 97 | 101 | 114 => {
                        temp = malloc(2 as libc::c_int as usize) as *mut libc::c_char;
                        if c == 'a' as i32 {
                            *temp.offset(0 as libc::c_int as isize) =
                                '\u{7}' as i32 as libc::c_char;
                        } else if c == 'e' as i32 {
                            *temp.offset(0 as libc::c_int as isize) =
                                '\u{1b}' as i32 as libc::c_char;
                        } else if c == 'r' as i32 {
                            *temp.offset(0 as libc::c_int as isize) = '\r' as i32 as libc::c_char;
                        } else {
                            *temp.offset(0 as libc::c_int as isize) = c as libc::c_char;
                        }
                        *temp.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                        current_block = 6294327190052088242;
                    }
                    _ => {
                        current_block = 3323500006713522785;
                    }
                }
                match current_block {
                    6294327190052088242 => {}
                    _ => {
                        temp = malloc(3 as libc::c_int as usize) as *mut libc::c_char;
                        *temp.offset(0 as libc::c_int as isize) = '\\' as i32 as libc::c_char;
                        *temp.offset(1 as libc::c_int as isize) = c as libc::c_char;
                        *temp.offset(2 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                    }
                }
            } else {
                if (result_index + 3 as libc::c_int) as size_t >= result_size {
                    while (result_index + 3 as libc::c_int) as size_t >= result_size {
                        result_size = result_size.wrapping_add(48 as libc::c_int as size_t);
                    }
                    result = realloc(result as *mut libc::c_void, result_size as usize)
                        as *mut libc::c_char;
                }
                if c == '\u{1}' as i32 || c == '\u{7f}' as i32 {
                    let fresh55 = result_index;
                    result_index = result_index + 1;
                    *result.offset(fresh55 as isize) = '\u{1}' as i32 as libc::c_char;
                }

                let fresh56 = result_index;
                result_index = result_index + 1;
                *result.offset(fresh56 as isize) = c as libc::c_char;
                *result.offset(result_index as isize) = '\0' as i32 as libc::c_char;

                continue;
            }
            if c != 0 {
                string = string.offset(1);
            }
            result = sub_append_string(temp, result, &mut result_index, &mut result_size);
            temp = 0 as *mut libc::c_void as *mut libc::c_char;

            *result.offset(result_index as isize) = '\0' as i32 as libc::c_char;
        }

        save_dstack = dstack;
        dstack = temp_dstack;
        dstack.delimiter_depth = 0 as libc::c_int;
        if promptvars != 0 || posixly_correct != 0 {
            last_exit_value = last_command_exit_value;
            last_comsub_pid = last_command_subst_pid;
            list = expand_prompt_string(result, 0x1 as libc::c_int, 0 as libc::c_int);
            free(result as *mut libc::c_void);
            result = string_list(list);
            dispose_words(list);
            ::core::ptr::write_volatile(
                &mut last_command_exit_value as *mut libc::c_int,
                last_exit_value,
            );
            last_command_subst_pid = last_comsub_pid;
        } else {
            t = dequote_string(result);
            free(result as *mut libc::c_void);
            result = t;
        }
        dstack = save_dstack;

        return result;
    }
}
#[no_mangle]
pub fn yyerror(mut _msg: *const libc::c_char) -> libc::c_int {
    report_syntax_error(0 as *mut libc::c_void as *mut libc::c_char);
    reset_parser();
    return 0 as libc::c_int;
}
fn error_token_from_token(mut tok: libc::c_int) -> *mut libc::c_char {
    unsafe {
        let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
        t = find_token_in_alist(tok, word_token_alist.as_mut_ptr(), 0 as libc::c_int);
        if !t.is_null() {
            return t;
        }
        t = find_token_in_alist(tok, other_token_alist.as_mut_ptr(), 0 as libc::c_int);
        if !t.is_null() {
            return t;
        }
        t = 0 as *mut libc::c_void as *mut libc::c_char;
        match current_token {
            281 | 282 => {
                if !(yylval.word).is_null() {
                    t = strcpy(
                        malloc(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(libc::strlen((*yylval.word).word) as u64)
                                as usize,
                        ) as *mut libc::c_char,
                        (*yylval.word).word,
                    );
                }
            }
            284 => {
                t = c_itos(yylval.number as intmax_t);
            }
            285 => {
                if !(yylval.WordList).is_null() {
                    t = string_list(yylval.WordList);
                }
            }
            286 => {
                if !(yylval.WordList).is_null() {
                    t = string_list_internal(
                        yylval.WordList,
                        b" ; \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                }
            }
            287 => {
                t = 0 as *mut libc::c_void as *mut libc::c_char;
            }
            _ => {}
        }
        return t;
    }
}
fn error_token_from_text() -> *mut libc::c_char {
    unsafe {
        let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut token_end: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        t = shell_input_line;
        i = shell_input_line_index as libc::c_int;
        token_end = 0 as libc::c_int;
        msg = 0 as *mut libc::c_void as *mut libc::c_char;
        if i != 0 && *t.offset(i as isize) as libc::c_int == '\0' as i32 {
            i -= 1;
        }
        while i != 0
            && (*t.offset(i as isize) as libc::c_int == ' ' as i32
                || *t.offset(i as isize) as libc::c_int == '\t' as i32
                || *t.offset(i as isize) as libc::c_int == '\n' as i32)
        {
            i -= 1;
        }
        if i != 0 {
            token_end = i + 1 as libc::c_int;
        }
        while i != 0
            && (if *t.offset(i as isize) as libc::c_int != 0 {
                (c_mbschr(
                    b" \n\t;|&\0" as *const u8 as *const libc::c_char,
                    *t.offset(i as isize) as libc::c_int,
                ) != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int
            } else {
                0 as libc::c_int
            }) == 0 as libc::c_int
        {
            i -= 1;
        }
        while i != token_end
            && (*t.offset(i as isize) as libc::c_int == ' ' as i32
                || *t.offset(i as isize) as libc::c_int == '\t' as i32
                || *t.offset(i as isize) as libc::c_int == '\n' as i32)
        {
            i += 1;
        }
        if token_end != 0 || i == 0 as libc::c_int && token_end == 0 as libc::c_int {
            if token_end != 0 {
                msg = substring(t, i, token_end);
            } else {
                msg = malloc(2 as libc::c_int as usize) as *mut libc::c_char;

                *msg.offset(0 as libc::c_int as isize) = *t.offset(i as isize);
                *msg.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            }
        }
        return msg;
    }
}
fn print_offending_line() {
    unsafe {
        let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut token_end: libc::c_int = 0;
        msg = strcpy(
            malloc(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(libc::strlen(shell_input_line) as u64) as usize,
            ) as *mut libc::c_char,
            shell_input_line,
        );
        token_end = libc::strlen(msg) as libc::c_int;
        while token_end != 0
            && *msg.offset((token_end - 1 as libc::c_int) as isize) as libc::c_int == '\n' as i32
        {
            token_end -= 1;

            *msg.offset(token_end as isize) = '\0' as i32 as libc::c_char;
        }

        parser_error(
            line_number,
            b"`%s'\0" as *const u8 as *const libc::c_char,
            msg,
        );
        free(msg as *mut libc::c_void);
    }
}
fn report_syntax_error(mut message: *mut libc::c_char) {
    unsafe {
        let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        if !message.is_null() {
            parser_error(
                line_number,
                b"%s\0" as *const u8 as *const libc::c_char,
                message,
            );
            if interactive != 0 && EOF_Reached != 0 {
                EOF_Reached = 0 as libc::c_int;
            }

            ::core::ptr::write_volatile(
                &mut last_command_exit_value as *mut libc::c_int,
                if executing_builtin != 0 && parse_and_execute_level != 0 {
                    257 as libc::c_int
                } else {
                    2 as libc::c_int
                },
            );
            set_pipestatus_from_exit(last_command_exit_value);
            return;
        }
        if current_token != 0 as libc::c_int && EOF_Reached == 0 as libc::c_int && {
            msg = error_token_from_token(current_token);
            !msg.is_null()
        } {
            if c_ansic_shouldquote(msg) != 0 {
                p = c_ansic_quote(msg, 0 as libc::c_int, 0 as *mut libc::c_int);
                free(msg as *mut libc::c_void);
                msg = p;
            }

            parser_error(
                line_number,
                c_dcgettext(
                    0 as *const libc::c_char,
                    b"syntax error near unexpected token `%s'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                msg,
            );

            free(msg as *mut libc::c_void);
            if interactive == 0 as libc::c_int {
                print_offending_line();
            }
            ::core::ptr::write_volatile(
                &mut last_command_exit_value as *mut libc::c_int,
                if executing_builtin != 0 && parse_and_execute_level != 0 {
                    257 as libc::c_int
                } else {
                    2 as libc::c_int
                },
            );

            set_pipestatus_from_exit(last_command_exit_value);
            return;
        }

        if !shell_input_line.is_null() && *shell_input_line as libc::c_int != 0 {
            msg = error_token_from_text();
            if !msg.is_null() {
                parser_error(
                    line_number,
                    c_dcgettext(
                        0 as *const libc::c_char,
                        b"syntax error near `%s'\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    msg,
                );
                free(msg as *mut libc::c_void);
            }
            if interactive == 0 as libc::c_int {
                print_offending_line();
            }
        } else {
            msg = if EOF_Reached != 0 {
                c_dcgettext(
                    0 as *const libc::c_char,
                    b"syntax error: unexpected end of file\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            } else {
                c_dcgettext(
                    0 as *const libc::c_char,
                    b"syntax error\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            };
            parser_error(
                line_number,
                b"%s\0" as *const u8 as *const libc::c_char,
                msg,
            );
            if interactive != 0 && EOF_Reached != 0 {
                EOF_Reached = 0 as libc::c_int;
            }
        }
        ::core::ptr::write_volatile(
            &mut last_command_exit_value as *mut libc::c_int,
            if executing_builtin != 0 && parse_and_execute_level != 0 {
                257 as libc::c_int
            } else {
                2 as libc::c_int
            },
        );

        set_pipestatus_from_exit(last_command_exit_value);
    }
}
#[no_mangle]
pub static mut ignoreeof: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut eof_encountered: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut eof_encountered_limit: libc::c_int = 10 as libc::c_int;
fn handle_eof_input_unit() {
    unsafe {
        if interactive != 0 {
            if EOF_Reached != 0 {
                EOF_Reached = 0 as libc::c_int;
            }
            if ignoreeof != 0 {
                if eof_encountered < eof_encountered_limit {
                    fprintf(
                        stderr,
                        c_dcgettext(
                            0 as *const libc::c_char,
                            b"Use \"%s\" to leave the shell.\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        if login_shell != 0 {
                            b"logout\0" as *const u8 as *const libc::c_char
                        } else {
                            b"exit\0" as *const u8 as *const libc::c_char
                        },
                    );
                    eof_encountered += 1;
                    current_token = '\n' as i32;
                    last_read_token = current_token;
                    prompt_string_pointer = 0 as *mut libc::c_void as *mut *mut libc::c_char;
                    prompt_again();
                    return;
                }
            }
            reset_parser();
            last_shell_builtin = this_shell_builtin;
            this_shell_builtin = Some(exit_builtin as fn(*mut WORD_LIST) -> libc::c_int);
            exit_builtin(0 as *mut libc::c_void as *mut WORD_LIST);
        } else {
            EOF_Reached = 1 as libc::c_int;
        };
    }
}
static mut parse_string_error: WORD_LIST = word_list {
    next: 0 as *const word_list as *mut word_list,
    word: 0 as *const WORD_DESC as *mut WORD_DESC,
};
#[no_mangle]
pub fn parse_string_to_word_list(
    mut s: *mut libc::c_char,
    mut flags: libc::c_int,
    mut whom: *const libc::c_char,
) -> *mut WORD_LIST {
    unsafe {
        let mut wl: *mut WORD_LIST = 0 as *mut WORD_LIST;
        let mut tok: libc::c_int = 0;
        let mut orig_current_token: libc::c_int = 0;
        let mut orig_line_number: libc::c_int = 0;
        let mut orig_input_terminator: libc::c_int = 0;
        let mut orig_line_count: libc::c_int = 0;
        let mut old_echo_input: libc::c_int = 0;
        let mut old_expand_aliases: libc::c_int = 0;
        let mut _ea: libc::c_int = 0;
        let mut old_remember_on_history: libc::c_int = 0;
        let mut old_history_expansion_inhibited: libc::c_int = 0;

        old_remember_on_history = remember_on_history;
        old_history_expansion_inhibited = history_expansion_inhibited;
        bash_history_disable();
        orig_line_number = line_number;
        orig_line_count = current_command_line_count;
        orig_input_terminator = shell_input_line_terminator;
        old_echo_input = echo_input_at_read;
        old_expand_aliases = expand_aliases;
        push_stream(1 as libc::c_int);
        last_read_token = 281 as libc::c_int;
        current_command_line_count = 0 as libc::c_int;
        expand_aliases = 0 as libc::c_int;
        echo_input_at_read = expand_aliases;
        with_input_from_string(s, whom);
        wl = 0 as *mut libc::c_void as *mut WORD_LIST;
        if flags & 1 as libc::c_int != 0 {
            parser_state |= 0x2000 as libc::c_int | 0x40000 as libc::c_int;
        }

        loop {
            tok = read_token(0 as libc::c_int);
            if !(tok != 304 as libc::c_int) {
                break;
            }
            if tok == '\n' as i32 && *bash_input.location.string as libc::c_int == '\0' as i32 {
                break;
            }
            if tok == '\n' as i32 {
                continue;
            }

            if tok != 281 as libc::c_int && tok != 282 as libc::c_int {
                line_number = orig_line_number + line_number - 1 as libc::c_int;
                orig_current_token = current_token;
                current_token = tok;
                yyerror(0 as *const libc::c_char);
                current_token = orig_current_token;
                if !wl.is_null() {
                    dispose_words(wl);
                }
                wl = &mut parse_string_error;
                break;
            } else {
                wl = make_word_list(yylval.word, wl);
            }
        }

        last_read_token = '\n' as i32;
        pop_stream();
        remember_on_history = old_remember_on_history;
        history_expansion_inhibited = old_history_expansion_inhibited;
        echo_input_at_read = old_echo_input;
        expand_aliases = old_expand_aliases;
        current_command_line_count = orig_line_count;
        shell_input_line_terminator = orig_input_terminator;
        if flags & 1 as libc::c_int != 0 {
            parser_state &= !(0x2000 as libc::c_int | 0x40000 as libc::c_int);
        }
        if wl == &mut parse_string_error as *mut WORD_LIST {
            set_exit_status(1 as libc::c_int);
            if interactive_shell == 0 as libc::c_int && posixly_correct != 0 {
                jump_to_top_level(1 as libc::c_int);
            } else {
                jump_to_top_level(2 as libc::c_int);
            }
        }

        return if !wl.is_null() && !((*wl).next).is_null() {
            list_reverse(wl as *mut GENERIC_LIST) as *mut WORD_LIST
        } else {
            wl
        };
    }
}
fn parse_compound_assignment(mut retlenp: *mut libc::c_int) -> *mut libc::c_char {
    unsafe {
        let mut wl: *mut WORD_LIST = 0 as *mut WORD_LIST;
        let mut rl: *mut WORD_LIST = 0 as *mut WORD_LIST;
        let mut tok: libc::c_int = 0;
        let mut orig_line_number: libc::c_int = 0;
        let mut orig_token_size: libc::c_int = 0;
        let mut orig_last_token: libc::c_int = 0;
        let mut assignok: libc::c_int = 0;
        let mut saved_token: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;

        saved_token = token;
        orig_token_size = token_buffer_size;
        orig_line_number = line_number;
        orig_last_token = last_read_token;
        last_read_token = 281 as libc::c_int;
        token = 0 as *mut libc::c_void as *mut libc::c_char;
        token_buffer_size = 0 as libc::c_int;
        assignok = parser_state & 0x4000 as libc::c_int;
        wl = 0 as *mut libc::c_void as *mut WORD_LIST;
        parser_state |= 0x2000 as libc::c_int;

        loop {
            tok = read_token(0 as libc::c_int);
            if !(tok != ')' as i32) {
                break;
            }

            if tok == '\n' as i32 {
                if interactive != 0
                    && (bash_input.type_0 as libc::c_uint
                        == st_stdin as libc::c_int as libc::c_uint
                        || bash_input.type_0 as libc::c_uint
                            == st_stream as libc::c_int as libc::c_uint)
                {
                    prompt_again();
                }
            } else if tok != 281 as libc::c_int && tok != 282 as libc::c_int {
                current_token = tok;
                if tok == 304 as libc::c_int {
                    parser_error(
                        orig_line_number,
                        c_dcgettext(
                            0 as *const libc::c_char,
                            b"unexpected EOF while looking for matching `)'\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                } else {
                    yyerror(0 as *const libc::c_char);
                }
                if !wl.is_null() {
                    dispose_words(wl);
                }
                wl = &mut parse_string_error;
                break;
            } else {
                wl = make_word_list(yylval.word, wl);
            }
        }
        if !token.is_null() {
            free(token as *mut libc::c_void);
        }

        token = 0 as *mut libc::c_char;
        token = saved_token;
        token_buffer_size = orig_token_size;
        parser_state &= !(0x2000 as libc::c_int);
        if wl == &mut parse_string_error as *mut WORD_LIST {
            set_exit_status(1 as libc::c_int);
            last_read_token = '\n' as i32;
            if interactive_shell == 0 as libc::c_int && posixly_correct != 0 {
                jump_to_top_level(1 as libc::c_int);
            } else {
                jump_to_top_level(2 as libc::c_int);
            }
        }
        last_read_token = orig_last_token;

        if !wl.is_null() {
            rl = if !wl.is_null() && !((*wl).next).is_null() {
                list_reverse(wl as *mut GENERIC_LIST) as *mut WORD_LIST
            } else {
                wl
            };
            ret = string_list(rl);
            dispose_words(rl);
        } else {
            ret = 0 as *mut libc::c_void as *mut libc::c_char;
        }
        if !retlenp.is_null() {
            *retlenp = (if !ret.is_null() && *ret as libc::c_int != 0 {
                libc::strlen(ret) as usize
            } else {
                0 as libc::c_int as usize
            }) as libc::c_int;
        }
        if assignok != 0 {
            parser_state |= 0x4000 as libc::c_int;
        }
        return ret;
    }
}
#[no_mangle]
pub fn save_parser_state(mut ps: *mut sh_parser_state_t) -> *mut sh_parser_state_t {
    unsafe {
        if ps.is_null() {
            ps = malloc(::core::mem::size_of::<sh_parser_state_t>() as usize)
                as *mut sh_parser_state_t;
        }
        if ps.is_null() {
            return 0 as *mut libc::c_void as *mut sh_parser_state_t;
        }

        (*ps).parser_state = parser_state;
        (*ps).token_state = save_token_state();
        (*ps).input_line_terminator = shell_input_line_terminator;
        (*ps).eof_encountered = eof_encountered;
        (*ps).prompt_string_pointer = prompt_string_pointer;
        (*ps).current_command_line_count = current_command_line_count;
        (*ps).remember_on_history = remember_on_history;
        (*ps).history_expansion_inhibited = history_expansion_inhibited;
        (*ps).last_command_exit_value = last_command_exit_value;
        (*ps).pipestatus = save_pipestatus_array();
        (*ps).last_shell_builtin = last_shell_builtin;
        (*ps).this_shell_builtin = this_shell_builtin;
        (*ps).expand_aliases = expand_aliases;
        (*ps).echo_input_at_read = echo_input_at_read;
        (*ps).need_here_doc = need_here_doc;
        (*ps).here_doc_first_line = here_doc_first_line;
        if need_here_doc == 0 as libc::c_int {
            (*ps).redir_stack[0 as libc::c_int as usize] = 0 as *mut REDIRECT;
        } else {
            memcpy(
                ((*ps).redir_stack).as_mut_ptr() as *mut libc::c_void,
                redir_stack.as_mut_ptr() as *const libc::c_void,
                (::core::mem::size_of::<*mut REDIRECT>() as libc::c_ulong)
                    .wrapping_mul(16 as libc::c_int as libc::c_ulong) as usize,
            );
        }
        (*ps).token = token;
        (*ps).token_buffer_size = token_buffer_size;
        token = 0 as *mut libc::c_char;
        token_buffer_size = 0 as libc::c_int;

        return ps;
    }
}
#[no_mangle]
pub fn restore_parser_state(mut ps: *mut sh_parser_state_t) {
    unsafe {
        let mut _i: libc::c_int = 0;
        if ps.is_null() {
            return;
        }

        parser_state = (*ps).parser_state;
        if !((*ps).token_state).is_null() {
            restore_token_state((*ps).token_state);
            free((*ps).token_state as *mut libc::c_void);
        }
        shell_input_line_terminator = (*ps).input_line_terminator;
        eof_encountered = (*ps).eof_encountered;
        prompt_string_pointer = (*ps).prompt_string_pointer;
        current_command_line_count = (*ps).current_command_line_count;
        remember_on_history = (*ps).remember_on_history;
        history_expansion_inhibited = (*ps).history_expansion_inhibited;
        ::core::ptr::write_volatile(
            &mut last_command_exit_value as *mut libc::c_int,
            (*ps).last_command_exit_value,
        );
        restore_pipestatus_array((*ps).pipestatus);
        last_shell_builtin = (*ps).last_shell_builtin;
        this_shell_builtin = (*ps).this_shell_builtin;
        expand_aliases = (*ps).expand_aliases;
        echo_input_at_read = (*ps).echo_input_at_read;
        need_here_doc = (*ps).need_here_doc;
        here_doc_first_line = (*ps).here_doc_first_line;
        if need_here_doc == 0 as libc::c_int {
            redir_stack[0 as libc::c_int as usize] = 0 as *mut REDIRECT;
        } else {
            memcpy(
                redir_stack.as_mut_ptr() as *mut libc::c_void,
                ((*ps).redir_stack).as_mut_ptr() as *const libc::c_void,
                (::core::mem::size_of::<*mut REDIRECT>() as libc::c_ulong)
                    .wrapping_mul(16 as libc::c_int as libc::c_ulong) as usize,
            );
        }
        if !token.is_null() {
            free(token as *mut libc::c_void);
        }
        token = 0 as *mut libc::c_char;
        token = (*ps).token;
        token_buffer_size = (*ps).token_buffer_size;
    }
}
#[no_mangle]
pub fn save_input_line_state(mut ls: *mut sh_input_line_state_t) -> *mut sh_input_line_state_t {
    unsafe {
        if ls.is_null() {
            ls = malloc(::core::mem::size_of::<sh_input_line_state_t>() as usize)
                as *mut sh_input_line_state_t;
        }
        if ls.is_null() {
            return 0 as *mut libc::c_void as *mut sh_input_line_state_t;
        }

        (*ls).input_line = shell_input_line;
        (*ls).input_line_size = shell_input_line_size;
        (*ls).input_line_len = shell_input_line_len;
        (*ls).input_line_index = shell_input_line_index;
        (*ls).input_property = shell_input_line_property;
        (*ls).input_propsize = shell_input_line_propsize;
        shell_input_line = 0 as *mut libc::c_char;
        shell_input_line_index = 0 as libc::c_int as size_t;
        shell_input_line_len = shell_input_line_index;
        shell_input_line_size = shell_input_line_len;
        shell_input_line_property = 0 as *mut libc::c_char;
        shell_input_line_propsize = 0 as libc::c_int as size_t;

        return ls;
    }
}
#[no_mangle]
pub fn restore_input_line_state(mut ls: *mut sh_input_line_state_t) {
    unsafe {
        if !shell_input_line.is_null() {
            free(shell_input_line as *mut libc::c_void);
        }
        shell_input_line = 0 as *mut libc::c_char;
        shell_input_line = (*ls).input_line;
        shell_input_line_size = (*ls).input_line_size;
        shell_input_line_len = (*ls).input_line_len;
        shell_input_line_index = (*ls).input_line_index;
        if !shell_input_line_property.is_null() {
            free(shell_input_line_property as *mut libc::c_void);
        }
        shell_input_line_property = 0 as *mut libc::c_char;
        shell_input_line_property = (*ls).input_property;
        shell_input_line_propsize = (*ls).input_propsize;
    }
}
fn set_line_mbstate() {
    unsafe {
        let mut c: libc::c_int = 0;
        let mut i: size_t = 0;
        let mut previ: size_t = 0;
        let mut len: size_t = 0;
        let mut mbs: mbstate_t = __mbstate_t {
            __count: 0,
            __value: mbstate_t_value { __wch: 0 },
        };
        let mut prevs: mbstate_t = __mbstate_t {
            __count: 0,
            __value: mbstate_t_value { __wch: 0 },
        };
        let mut mbclen: size_t = 0;
        let mut ilen: libc::c_int = 0;
        if shell_input_line.is_null() {
            return;
        }
        len = if !shell_input_line.is_null()
            && *shell_input_line.offset(0 as libc::c_int as isize) as libc::c_int != 0
        {
            if *shell_input_line.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                if *shell_input_line.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                    libc::strlen(shell_input_line) as u64
                } else {
                    2 as libc::c_int as libc::c_ulong
                }
            } else {
                1 as libc::c_int as libc::c_ulong
            }
        } else {
            0 as libc::c_int as libc::c_ulong
        };
        if len == 0 as libc::c_int as size_t {
            return;
        }

        if shell_input_line_propsize >= 32768 as libc::c_int as size_t
            && len < (32768 as libc::c_int >> 1 as libc::c_int) as size_t
        {
            free(shell_input_line_property as *mut libc::c_void);
            shell_input_line_property = 0 as *mut libc::c_char;
            shell_input_line_propsize = 0 as libc::c_int as size_t;
        }
        if len.wrapping_add(1 as libc::c_int as size_t) > shell_input_line_propsize {
            shell_input_line_propsize = len.wrapping_add(1 as libc::c_int as size_t);
            shell_input_line_property = realloc(
                shell_input_line_property as *mut libc::c_void,
                shell_input_line_propsize as usize,
            ) as *mut libc::c_char;
        }
        if locale_mb_cur_max == 1 as libc::c_int {
            memset(
                shell_input_line_property as *mut libc::c_void,
                1 as libc::c_int,
                len as usize,
            );
            return;
        }
        if locale_utf8locale == 0 as libc::c_int {
            memset(
                &mut prevs as *mut mbstate_t as *mut libc::c_void,
                '\0' as i32,
                ::core::mem::size_of::<mbstate_t>() as libc::c_ulong as usize,
            );
        }

        previ = 0 as libc::c_int as size_t;
        i = previ;
        while i < len {
            if locale_utf8locale == 0 as libc::c_int {
                mbs = prevs;
            }
            c = *shell_input_line.offset(i as isize) as libc::c_int;
            if c == -(1 as libc::c_int) {
                let mut j: size_t = 0;
                j = i;
                while j < len {
                    *shell_input_line_property.offset(j as isize) =
                        1 as libc::c_int as libc::c_char;

                    j = j.wrapping_add(1);
                }
                break;
            } else {
                if locale_utf8locale != 0 {
                    if (*shell_input_line.offset(previ as isize) as libc::c_uchar as libc::c_int)
                        < 128 as libc::c_int
                    {
                        mbclen = 1 as libc::c_int as size_t;
                    } else {
                        ilen = c_utf8_mblen(
                            shell_input_line.offset(previ as isize),
                            i.wrapping_sub(previ)
                                .wrapping_add(1 as libc::c_int as size_t),
                        );
                        mbclen = if ilen == -(1 as libc::c_int) {
                            -(1 as libc::c_int) as size_t
                        } else if ilen == -(2 as libc::c_int) {
                            -(2 as libc::c_int) as size_t
                        } else {
                            ilen as size_t
                        };
                    }
                } else {
                    mbclen = mbrlen(
                        shell_input_line.offset(previ as isize),
                        i.wrapping_sub(previ)
                            .wrapping_add(1 as libc::c_int as size_t),
                        &mut mbs,
                    );
                }
                if mbclen == 1 as libc::c_int as size_t || mbclen == -(1 as libc::c_int) as size_t {
                    mbclen = 1 as libc::c_int as size_t;
                    previ = i.wrapping_add(1 as libc::c_int as size_t);
                } else if mbclen == -(2 as libc::c_int) as size_t {
                    mbclen = 0 as libc::c_int as size_t;
                } else if mbclen > 1 as libc::c_int as size_t {
                    mbclen = 0 as libc::c_int as size_t;
                    previ = i.wrapping_add(1 as libc::c_int as size_t);
                    if locale_utf8locale == 0 as libc::c_int {
                        prevs = mbs;
                    }
                } else {
                    let mut j_0: size_t = 0;
                    j_0 = i;
                    while j_0 < len {
                        *shell_input_line_property.offset(j_0 as isize) =
                            1 as libc::c_int as libc::c_char;

                        j_0 = j_0.wrapping_add(1);
                    }
                    break;
                }

                *shell_input_line_property.offset(i as isize) = mbclen as libc::c_char;

                i = i.wrapping_add(1);
            }
        }
    }
}
