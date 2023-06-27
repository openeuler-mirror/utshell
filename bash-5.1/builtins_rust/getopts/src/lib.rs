extern crate  libc;
extern crate nix;

use libc::{c_char, c_long, c_void};
use std::{ffi::CString};

#[repr(C)]
pub struct WORD_DESC {
    pub word: *mut libc::c_char,
    pub flags:libc::c_int
}

#[repr(C)]
#[derive(Copy,Clone)]
pub struct WORD_LIST {
    next: *mut WORD_LIST,
    word: *mut WORD_DESC
}

#[repr(u8)]
enum command_type { cm_for, cm_case, cm_while, cm_if, cm_simple, cm_select,
    cm_connection, cm_function_def, cm_until, cm_group,
    cm_arith, cm_cond, cm_arith_for, cm_subshell, cm_coproc
}

#[repr(u8)]
#[derive(Copy,Clone)]
enum r_instruction {
    r_output_direction, r_input_direction, r_inputa_direction,
    r_appending_to, r_reading_until, r_reading_string,
    r_duplicating_input, r_duplicating_output, r_deblank_reading_until,
    r_close_this, r_err_and_out, r_input_output, r_output_force,
    r_duplicating_input_word, r_duplicating_output_word,
    r_move_input, r_move_output, r_move_input_word, r_move_output_word,
    r_append_err_and_out
}

#[repr(C)]
#[derive(Copy,Clone)]
pub union REDIRECTEE {
    dest:libc::c_int,
    filename:* mut WORD_DESC
}

#[repr(C)]
pub union REDIRECT {
  next:*mut REDIRECT,
  redirector:REDIRECTEE,
  rflags:libc::c_int,
  flags:libc::c_int,
  instruction:r_instruction,
  redirectee:REDIRECTEE,
  here_doc_eof:*mut c_char
}

/* FOR command. */
#[repr(C)]
pub struct for_com {
    flags:libc::c_int,
    line:libc::c_int,
    name:*mut WORD_DESC,
    map_list:*mut WORD_LIST,
    action:*mut COMMAND
}

#[repr(C)]
pub struct PATTERN_LIST {
    next:* mut PATTERN_LIST,
    patterns:* mut WORD_LIST,
    action:*mut COMMAND,
    flags:libc::c_int
}

#[repr(C)]
pub struct case_com {
    flags:libc::c_int,
    line:libc::c_int,
    word:*mut WORD_DESC,
    clauses:*mut PATTERN_LIST
}

#[repr(C)]
pub struct while_com {
    flags:libc::c_int,
    test:*mut COMMAND,
    action:*mut COMMAND
}

#[repr(C)]
pub struct if_com {
    flags:libc::c_int,
    test:*mut COMMAND,
    true_case:*mut COMMAND,
    false_case:*mut COMMAND
}

#[repr(C)]
pub struct connection {
    ignore:libc::c_int,
    first:*mut COMMAND,
    second:*mut COMMAND,
    connector:libc::c_int
}

#[repr(C)]
pub struct simple_com {
    flags:libc::c_int,
    line:libc::c_int,
    words:*mut WORD_LIST,
    redirects:*mut REDIRECT
}

#[repr(C)]
pub struct function_def {
    flags:libc::c_int,
    line:libc::c_int,
    name:*mut WORD_DESC,
    command:*mut COMMAND,
    source_file:*mut c_char
}

#[repr(C)]
pub struct group_com {
    ignore:libc::c_int,
    command:*mut COMMAND,
    source_file:*mut c_char
}

#[repr(C)]
pub struct select_com {
    flags:libc::c_int,
    line:libc::c_int,
    name:*mut WORD_DESC,
    map_list:*mut WORD_LIST,
    action:*mut COMMAND
}

#[repr(C)]
pub struct arith_com {
    flags:libc::c_int,
    line:libc::c_int,
    exp:*mut WORD_LIST
}

#[repr(C)]
pub struct cond_com {
    flags:libc::c_int,
    line:libc::c_int,
    type_c:libc::c_int,
    exp:*mut WORD_LIST
}

#[repr(C)]
pub struct arith_for_com {
    flags:libc::c_int,
    line:libc::c_int,
    init:*mut WORD_LIST,
    test:*mut WORD_LIST,
    step:*mut WORD_LIST,
    action:*mut COMMAND
}

#[repr(C)]
pub struct subshell_com {
    flags:i32,
    line:i32,
    command:*mut COMMAND
}

#[repr(C)]
pub struct coproc_com {
    flags:i32,
    name:*mut c_char,
    command:*mut COMMAND
}

#[repr(C)]
pub union VALUE_COMMAND {
    For:*mut for_com,
    Case:*mut case_com,
    While:*mut while_com,
    If:*mut if_com,
    Connection:*mut connection,
    Simple:*mut simple_com,
    Function_def:*mut function_def,
    Group:*mut group_com,
    Select:*mut select_com,
    Arith:*mut arith_com,
    Cond:*mut cond_com,
    ArithFor:*mut arith_for_com,
    Subshell:*mut subshell_com,
    Coproc:*mut coproc_com
}

#[repr(C)]
pub struct COMMAND {
    type_c:command_type,
    flags:i32,
    line:i32,
    redirects:*mut REDIRECT,
    value:VALUE_COMMAND
}

#[repr(C)]
pub struct SHELL_VAR {
  name:*mut c_char,
  value:*mut c_char,
  exportstr:*mut c_char,
  dynamic_value:*mut fn(v:* mut SHELL_VAR)->*mut SHELL_VAR,
  assign_func:* mut fn(v:* mut SHELL_VAR,str1:* mut c_char,t:c_long,str2:* mut c_char)->*mut SHELL_VAR,
  attributes:i32,
  context:i32
}

#[macro_export]
macro_rules! EXECUTION_FAILURE {
   () => {1}
}

#[macro_export]
macro_rules! EX_USAGE {
   () => {258}
}

#[macro_export]
macro_rules! EXECUTION_SUCCESS {
   () => {0}
}

#[macro_export]
macro_rules! EX_MISCERROR {
   () => {2}
}

#[macro_export]
macro_rules! att_readonly {
  () => {
    0x0000002 /* cannot change */
  }
}

#[macro_export]
macro_rules! att_noassign {
  () => {
    0x0004000	/* assignment not allowed */
  }
}

#[macro_export]
macro_rules! G_EOF {
  () => {
    -1
  }
}

#[macro_export]
macro_rules! G_INVALID_OPT {
  () => {
    -2
  }
}

#[macro_export]
macro_rules! G_ARG_MISSING {
  () => {
    -3
  }
}

#[macro_export]
macro_rules! GETOPT_HELP {
  () => {
    -99
  }
}

extern "C" {
    fn unbind_variable_noref (name: * const c_char)->i32;
    static mut sh_optind:i32;
    static mut sh_badopt:i32;
    fn legal_identifier (name: * const c_char)->i32;
    fn bind_variable (name: * const c_char, value: * mut c_char, flags:i32)->* mut SHELL_VAR;
    fn sh_invalidid(name:* mut c_char);
    fn builtin_usage();
    static mut sh_opterr:i32;
    fn sh_getopt_restore_state (argv:*mut*mut c_char);
    static dollar_vars:[* mut c_char;10];
    fn sh_getopt (argc:i32, argv: * const*mut c_char, optstring: * const c_char)->i32;
    static rest_of_args:* mut WORD_LIST;
    fn number_of_args ()->i32;
    fn strvec_create (i:i32)->*mut*mut c_char;
    static sh_optarg:* mut c_char;
    static sh_optopt:i32;
    fn reset_internal_getopt();
    fn internal_getopt (list:*mut WORD_LIST , opts:*mut c_char)->i32;
    fn builtin_help ();
    static mut loptend:*mut WORD_LIST;
    fn make_builtin_argv (list:* mut WORD_LIST, ac:* mut i32)->*mut*mut c_char;
}

/* getopts_reset is magic code for when OPTIND is reset.  N is the
   value that has just been assigned to OPTIND. */
#[no_mangle]
pub extern "C" fn r_getopts_reset (newind:i32){
  unsafe {
    sh_optind = newind;
    sh_badopt = 0;
  }
}

#[no_mangle]
pub extern "C" fn r_getopts_unbind_variable (name:* mut c_char)->i32 {
  unsafe {
    return unbind_variable_noref (name);
  }
}

fn readonly_p(va:* mut SHELL_VAR)->i32
{
  unsafe {
    return (*va).attributes & att_readonly!();
  }
}

fn noassign_p(va:* mut SHELL_VAR)->i32
{
  unsafe {
    return (*va).attributes & att_noassign!();
  }
}

#[no_mangle]
pub extern "C" fn r_getopts_bind_variable(name:* mut c_char, value:* mut c_char)->i32
{
  let v:* mut SHELL_VAR;
  unsafe {
    if legal_identifier(name) !=0 {
      v = bind_variable(name, value, 0);
      if v != std::ptr::null_mut() && (readonly_p(v) != 0 || noassign_p(v) != 0) {
        return EX_MISCERROR!();
      }

      if v != std::ptr::null_mut() {
        return EXECUTION_SUCCESS!();
      } else {
        return EXECUTION_FAILURE!();
      }

    } else {
      sh_invalidid(name);
      return EXECUTION_FAILURE!();
    }
  }
}

#[no_mangle]
pub extern "C" fn r_dogetopts(argc:i32, argv:*mut*mut c_char)->i32
{
  let mut ret:i32;
  let special_error:i32;
  let mut old_opterr:i32=0;
  let mut i:i32;
  let n:i32;

  let mut strval:[c_char;2]=[0;2];
  let mut numval:[c_char;16]=[0;16];
  let mut optstr: * mut c_char; /* list of options */
  let name: * mut c_char;   /* variable to get flag val */
  let t:* mut c_char;
  unsafe {
    let mut argcc:i32=argc;
    let mut argvv:*mut*mut c_char=argv;
    if argcc < 3 {
      builtin_usage();
      return EX_USAGE!();
    }

    /* argv[0] is "getopts". */
    optstr = *((argvv as usize +8) as *mut*mut c_char);
    name = *((argvv as usize +16) as *mut*mut c_char);
    argcc -= 2;
    argvv = (argvv as usize +16) as *mut*mut c_char;    
    
    if *optstr == ':' as c_char {
      special_error = 1;
    } else {
      special_error = 0;
    }

    if special_error != 0 {
      old_opterr = sh_opterr;
      optstr=(optstr as usize + 4) as * mut c_char;
      sh_opterr = 0; /* suppress diagnostic messages */
    }

    if argcc > 1 {
      sh_getopt_restore_state(&mut(*argvv));
      t = *argvv;
      *argvv = dollar_vars[0];
      ret = sh_getopt(argcc, argvv, optstr);
      *argvv = t;
    } else if rest_of_args == std::ptr::null_mut() {
      i=0;
      while  i < 10 && dollar_vars[i as usize] != std::ptr::null_mut() {
        i+=1;
      }

      sh_getopt_restore_state(&mut (dollar_vars[0] as * mut c_char));
      ret = sh_getopt(i, &dollar_vars[0], optstr);
    } else {
      let mut words: * mut WORD_LIST;
      let v:*mut*mut c_char;

      i = number_of_args() + 1; /* +1 for $0 */
      v = strvec_create(i + 1);
      i=0;
      while i < 10 && dollar_vars[i as usize] !=std::ptr::null_mut() {
        *((v as usize +8*i as usize) as *mut*mut c_char)  = dollar_vars[i as usize];
        i+=1;
      }

      words = rest_of_args;
      while  words != std::ptr::null_mut() {
        *((v as usize +8*i as usize) as *mut*mut c_char)  = (*(*words).word).word;
        words = (*words).next;
        i+=1;
      }

      *((v as usize +8*i as usize) as *mut*mut c_char)  = std::ptr::null_mut();
      sh_getopt_restore_state(&mut(*v));
      ret = sh_getopt(i, v, optstr);
      libc::free(v as * mut c_void);
    }

    if special_error !=0 {
      sh_opterr = old_opterr.clone();
    }

    /* Set the OPTIND variable in any case, to handle "--" skipping.  It's
      highly unlikely that 14 digits will be too few. */
    if sh_optind < 10 {
      numval[14] = sh_optind as c_char+ '0' as c_char;
      numval[15] = '\0' as c_char;
      i = 14;
    } else {
      i=15;
      numval[15] = '\0' as c_char;
      n = sh_optind;

      i-=1;
      numval[i as usize] = (n % 10) as c_char + '0' as c_char;
      while n / 10 != 0 {
        i-=1;
        numval[i as usize] = (n % 10) as c_char + '0' as c_char;
      }
    }

    bind_variable(CString::new ("OPTIND").unwrap().as_ptr(), & mut numval[i as usize], 0);

    /* If an error occurred, decide which one it is and set the return
      code appropriately.  In all cases, the option character in error
      is in OPTOPT.  If an invalid option was encountered, OPTARG is
      NULL.  If a required option argument was missing, OPTARG points
      to a NULL string (that is, sh_optarg[0] == 0). */
    if ret == '?' as i32 {
      if sh_optarg == std::ptr::null_mut() {
        ret = G_INVALID_OPT!();
      } else if *sh_optarg == '\0' as c_char{
        ret = G_ARG_MISSING!();
      }
    }

    if ret == G_EOF!() {
      r_getopts_unbind_variable(CString::new ("OPTARG").unwrap().as_ptr()as * mut c_char);
      r_getopts_bind_variable(name, CString::new ("?").unwrap().as_ptr() as * mut c_char);
      return EXECUTION_FAILURE!();
    }

    if ret == G_INVALID_OPT!() {
      /* Invalid option encountered. */
      ret = r_getopts_bind_variable(name, CString::new ("?").unwrap().as_ptr() as * mut c_char);

      if special_error !=0 {
        strval[0] = sh_optopt as c_char;
        strval[1] = '\0' as c_char;
        bind_variable(CString::new ("OPTARG").unwrap().as_ptr() as * mut c_char, &mut strval[0], 0);
      } else {
        r_getopts_unbind_variable(CString::new ("OPTARG").unwrap().as_ptr() as * mut c_char);
      }

      return ret;
    }

    if ret == G_ARG_MISSING!() {
      /* Required argument missing. */
      if special_error !=0 {
        ret = r_getopts_bind_variable(name, CString::new (":").unwrap().as_ptr() as * mut c_char);

        strval[0] = sh_optopt as c_char;
        strval[1] = '\0' as c_char;
        bind_variable(CString::new ("OPTARG").unwrap().as_ptr() as * mut c_char, &mut strval[1], 0);
      } else {
        ret = r_getopts_bind_variable(name, CString::new ("?").unwrap().as_ptr() as * mut c_char);
        r_getopts_unbind_variable(CString::new ("OPTARG").unwrap().as_ptr() as * mut c_char);
      }
      return ret;
    }

    bind_variable(CString::new ("OPTARG").unwrap().as_ptr() as * mut c_char, sh_optarg, 0);

    strval[0] = ret as c_char;
    strval[1] = '\0' as c_char;
    return r_getopts_bind_variable(name, &mut strval[0]);
  }
}

#[no_mangle]
pub extern "C" fn r_getopts_builtin(list: * mut WORD_LIST)->i32
{
  unsafe {
    let av:*mut*mut c_char;
    let mut ac:i32=0;
    let mut ret:i32;

    if list == std::ptr::null_mut() {
      builtin_usage();
      return EX_USAGE!();
    }

    reset_internal_getopt();
    ret = internal_getopt(list, CString::new ("").unwrap().as_ptr() as * mut c_char);
    if ret != -1 {
      if ret == GETOPT_HELP!() {
        builtin_help();
      } else {
        builtin_usage();
      }

      return EX_USAGE!();
    }
    let llist: * mut WORD_LIST=loptend.clone();
    av = make_builtin_argv(llist, &mut ac);
    ret = r_dogetopts(ac, av);
    libc::free(av as * mut c_void);

    return ret;
  }
}

/*
#[no_mangle]
pub extern "C" fn cmd_name() ->*const u8 {
   return b"getopts" as *const u8;
}
#[no_mangle]
pub extern "C" fn run(list : *mut WORD_LIST)->i32 {
  return r_getopts_builtin(list);
}*/
