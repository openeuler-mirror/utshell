
use libc::*;
use r_bash::*;
use std::ffi::CStr;
use rcommon::WordList as WORD_LIST;
use rcommon::WordDesc as WORD_DESC;

extern "C" {
    fn hash_create(_: libc::c_int) -> *mut HASH_TABLE;
    static mut shell_start_time: time_t;
    static mut shellstart: timeval;
    fn absolute_program(_: *const libc::c_char) -> libc::c_int;
    fn parse_and_execute(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int;
}

#[no_mangle] 
pub unsafe extern "C" fn bind_spcial_variable(
    mut name: *const libc::c_char,
    mut value: *mut libc::c_char
) -> *mut libc::c_char
{
    if libc::strcmp(name,b"PS1\0" as * const u8 as *mut libc::c_char) == 0 {
        if  value.is_null() {
            if current_user.euid == 0 {
                value = Root_PS1_Value!();
            }
            else {
                value = PS1_Value!();
            }
        }
    }
    return value;
}

#[no_mangle] 
pub unsafe extern "C" fn bind_variable(
    mut name: *const libc::c_char,
    mut value: *mut libc::c_char,
    mut flags: libc::c_int,
) -> *mut SHELL_VAR {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut nv: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut vc: *mut VAR_CONTEXT = 0 as *mut VAR_CONTEXT;
    let mut nvc: *mut VAR_CONTEXT = 0 as *mut VAR_CONTEXT;

    value = bind_spcial_variable(name,value);
    if shell_variables.is_null() {
        create_variable_tables();
    }
    if !temporary_env.is_null() && !value.is_null() {
        bind_tempenv_variable(name, value);
    }
    vc = shell_variables;
    while !vc.is_null() {
        if vc_isfuncenv!(vc) || vc_isbltnenv!(vc)
        {
            v = hash_lookup(name, (*vc).table);
            nvc = vc;

            if !v.is_null() && nameref_p!(v)!= 0 {
                nv = find_variable_nameref_context(v, vc, &mut nvc);
                if nv.is_null() {
                    nv = find_variable_last_nameref_context(v, vc, &mut nvc);
                    if !nv.is_null() && nameref_p!(nv) != 0 {
                        if nameref_cell!(nv).is_null() {
                            return bind_variable_internal(
                                (*nv).name,
                                value,
                                (*nvc).table,
                                0 as libc::c_int,
                                flags,
                            )
                        } else if valid_array_reference(nameref_cell!(nv), 0 as libc::c_int)
                            != 0
                        {
                            return assign_array_element(nameref_cell!(nv), value, flags)
                        } else {
                            return bind_variable_internal(
                                nameref_cell!(nv),
                                value,
                                (*nvc).table,
                                0 as libc::c_int,
                                flags,
                            )
                        }
                    } else if nv == &mut nameref_maxloop_value as *mut SHELL_VAR {
                        internal_warning(
                        b"%s: circular name reference\0" as *const u8
                            as *const libc::c_char,
                            (*v).name,
                        );
                        return bind_global_variable((*v).name, value, flags);
                    } else {
                        v = nv;
                    }
                } else if nv == &mut nameref_maxloop_value as *mut SHELL_VAR {
                    internal_warning(
           
                    b"%s: circular name reference\0" as *const u8
                        as *const libc::c_char,
                        (*v).name,
                    );
                    return bind_global_variable((*v).name, value, flags);
                } else {
                    v = nv;
                }
            }
        
            if !v.is_null() {
                return bind_variable_internal(
                    (*v).name,
                    value,
                    (*nvc).table,
                    0 as libc::c_int,
                    flags,
                );
            }
        }
        
        vc = (*vc).down;
    }

    return bind_variable_internal(
        name,
        value,
        (*global_variables).table,
        0 as libc::c_int,
        flags,
    );
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: libc::c_int,
    pub tz_dsttime: libc::c_int,
}
pub type __timezone_ptr_t = *mut timezone;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct name_and_function {
    pub name: *mut libc::c_char,
    pub function: Option::<sh_sv_func_t>,
}

#[no_mangle]
pub static mut global_variables: *mut VAR_CONTEXT = 0 as *const libc::c_void
    as *mut libc::c_void as *mut VAR_CONTEXT;

#[no_mangle]
pub static mut shell_variables: *mut VAR_CONTEXT = 0 as *const libc::c_void
    as *mut libc::c_void as *mut VAR_CONTEXT;
    
#[no_mangle]
pub static mut shell_functions: *mut HASH_TABLE = 0 as *const libc::c_void
    as *mut libc::c_void as *mut HASH_TABLE;

#[no_mangle]
pub static mut invalid_env: *mut HASH_TABLE = 0 as *const libc::c_void
    as *mut libc::c_void as *mut HASH_TABLE;

#[no_mangle]
pub static mut shell_function_defs: *mut HASH_TABLE = 0 as *const libc::c_void
    as *mut libc::c_void as *mut HASH_TABLE;

#[no_mangle]
pub static mut temporary_env: *mut HASH_TABLE = 0 as *const libc::c_void
    as *mut libc::c_void as *mut HASH_TABLE ; 
#[no_mangle]
pub static mut export_env: *mut *mut libc::c_char = 0 as *const libc::c_void
    as *mut libc::c_void as *mut *mut libc::c_char;

static mut export_env_index: libc::c_int = 0;
static mut export_env_size: libc::c_int = 0;
static mut winsize_assignment: libc::c_int = 0;
const CHAR_BIT : libc::c_int = 8 as libc::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct saved_dollar_vars {
    pub first_ten: *mut *mut libc::c_char,
    pub rest: *mut WORD_LIST,
    pub count: libc::c_int,
}

#[macro_export] 
macro_rules! propagate_p {
    ($var:expr) => {
        (*$var).attributes & att_propagate  as libc::c_int
    }
}

#[macro_export] 
macro_rules! VC_FUNCENV {
    () => {
        0x04
    }
}

#[macro_export] 
macro_rules! capcase_p {
    ($vc:expr) => {
        (*$vc).attributes & att_capcase  as libc::c_int
    }
}

#[macro_export] 
macro_rules! uppercase_p {
    ($vc:expr) => {
        (*$vc).attributes & att_uppercase  as libc::c_int
    }
}

#[macro_export] 
macro_rules! IN_CTYPE_DOMAIN {
    ($c:expr) => {
        1 as libc::c_int
    }
}

#[macro_export] 
macro_rules! ISDIGIT {
    ($c:expr) => {
        (IN_CTYPE_DOMAIN!($c) !=0 ) && (libc::isdigit($c  as libc::c_int ) != 0)
    }
}

#[macro_export] 
macro_rules! lowercase_p {
    ($vc:expr) => {
        (*$vc).attributes & att_lowercase  as libc::c_int
    }
}

#[macro_export] 
macro_rules! vc_isfuncenv {
    ($vc:expr) => {
        ((*$vc).flags & VC_FUNCENV  as libc::c_int) != 0
    }
}

#[macro_export] 
macro_rules! vc_istempenv {
    ($vc:expr) => {
        ((*$vc).flags & VC_TEMPFLAGS  as libc::c_int) == VC_TEMPFLAGS as libc::c_int
    }
}

#[macro_export]
macro_rules! TEMPENV_HASH_BUCKETS {
    () => {
        4
    };
}

#[macro_export]
macro_rules! BASHFUNC_PREFIX {
    () => {
        b"BASH_FUNC_\0" as *const u8 as *const libc::c_char as *mut libc::c_char
    };
}

#[macro_export]
macro_rules! BASHFUNC_PREFLEN {
    () => {
        10 as libc::c_int
    };
}

#[macro_export]
macro_rules! BASHFUNC_SUFFIX {
    () => {
        b"%%\0" as *const u8 as *const libc::c_char as *mut libc::c_char
    };
}


#[macro_export]
macro_rules! BASHFUNC_SUFFLEN {
    () => {
        2 as libc::c_int
    };
}
#[macro_export]
macro_rules! FV_FORCETEMPENV {
    () => {
        0x01
    };
}

#[macro_export]
macro_rules! FV_SKIPINVISIBLE {
    () => {
        0x02
    };
}

#[macro_export]
macro_rules! FV_NODYNAMIC {
    () => {
        0x04
    };
}

#[macro_export]
macro_rules! EXECUTION_FAILURE {
    () => {
        1 as libc::c_int
    };
}


#[macro_export]
macro_rules! DEFAULT_PATH_VALUE {
    () => {
        b"/usr/local/bin:/usr/local/sbin:/usr/bin:/usr/sbin:/bin:/sbin:.\0" 
        as *const u8 as *const libc::c_char as *mut libc::c_char
    }
}

#[no_mangle]
pub static mut nameref_invalid_value: SHELL_VAR = SHELL_VAR {
  
    name: 0 as *const libc::c_char as *mut libc::c_char,
    value: 0 as *const libc::c_char as *mut libc::c_char,
    exportstr: 0 as *const libc::c_char as *mut libc::c_char,
    dynamic_value: None,
    assign_func: None,
    attributes: 0,
    context: 0,

};

static mut nameref_maxloop_value: SHELL_VAR = SHELL_VAR {
    name: 0 as *const libc::c_char as *mut libc::c_char,
    value: 0 as *const libc::c_char as *mut libc::c_char,
    exportstr: 0 as *const libc::c_char as *mut libc::c_char,
    dynamic_value: None,
    assign_func: None,
    attributes: 0,
    context: 0,
};

static mut last_table_searched: *mut HASH_TABLE = 0 as *const HASH_TABLE
    as *mut HASH_TABLE;  

#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}

unsafe extern "C" fn create_variable_tables() {
    if shell_variables.is_null() {
        // new_var_context 下面会实现

        global_variables = new_var_context(
            0 as *mut libc::c_void as *mut libc::c_char,
            0 as libc::c_int,
        );

        shell_variables = global_variables;
        (*shell_variables).scope = 0 as libc::c_int;
        //hash_create  为外部函数

        (*shell_variables).table = hash_create(VARIABLES_HASH_BUCKETS!() as libc::c_int);
    }
    if shell_functions.is_null() {

        shell_functions = hash_create(FUNCTIONS_HASH_BUCKETS!() as libc::c_int);
    }

}

unsafe extern "C" fn set_machine_vars() {
    
    let mut temp_var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    //set_if_not 下文会实现
    temp_var = set_if_not(
        b"HOSTTYPE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        get_host_type() as *mut libc::c_char
    ); 

    temp_var = set_if_not(
        b"OSTYPE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        get_os_type() as *mut libc::c_char
    );

    temp_var = set_if_not(
        b"MACHTYPE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        get_mach_type() as *mut libc::c_char
    );

    temp_var = set_if_not(
        b"HOSTNAME\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        current_host_name
    );


}

#[no_mangle]
pub unsafe extern "C" fn sh_get_home_dir() -> *mut libc::c_char {
    
    if (current_user.home_dir).is_null() {
        // get_current_user_info  in file of shell.c
        get_current_user_info();
    }
    return current_user.home_dir;
}

unsafe extern "C" fn set_home_var() {
    let mut temp_var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    // find_variable  下文会实现
    temp_var = find_variable(b"HOME\0" as *const u8 as *const libc::c_char);
    if temp_var.is_null() {
        temp_var =bind_variable(
            b"HOME\0" as *const u8 as *const libc::c_char,
            sh_get_home_dir(),
            0 as libc::c_int);
    }
}

unsafe extern "C" fn set_shell_var() {
    let mut temp_var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    // find_variable  下文会实现
    temp_var = find_variable(b"SHELL\0" as *const u8 as *const libc::c_char);
    if temp_var.is_null() {
        if (current_user.shell).is_null() {
            // get_current_user_info  in file of shell.c
            get_current_user_info();
        }
        temp_var =bind_variable(
            b"SHELL\0" as *const u8 as *const libc::c_char,
            current_user.shell,
            0 as libc::c_int);
    }
}

unsafe extern "C" fn get_bash_name() -> *mut libc::c_char 
{
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    
    if login_shell == 1 as libc::c_int && RELPATH!(shell_name)
    {
        if (current_user.shell).is_null() 
        {
            // get_current_user_info  in file of shell.c
            get_current_user_info();
        }
        name = savestring!(current_user.shell);
    } 
    else if ABSPATH!(shell_name)
    {
        name = savestring!(shell_name);
    }
    else if *shell_name.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
    && *shell_name.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32 
    { 
        let mut cdir: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut len: libc::c_int = 0;

        //get_string_value  下文会实现
        cdir = get_string_value(b"PWD\0" as *const u8 as *const libc::c_char);
        if !cdir.is_null() 
        {
            len = strlen(cdir) as libc::c_int;
            name = libc::malloc(
                (len as isize + strlen(shell_name) as isize + 1 as libc::c_int as isize).try_into().unwrap()
            ) as *mut libc::c_char ;
            strcpy(name, cdir);
            strcpy(
                name.offset(len as isize) ,
                shell_name.offset(1 as libc::c_int as isize) 
            );
        } 
        else {
            name = savestring!(shell_name);
        }
    }
    else 
    {
        let mut tname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut s: libc::c_int = 0;
        tname = find_user_command(shell_name);
        if tname.is_null() {
            s = file_status(shell_name);
            if s & FS_EXECABLE as libc::c_int != 0 {
                tname = make_absolute(
                    shell_name,
                    get_string_value(b"PWD\0" as *const u8 as *const libc::c_char),
                );
                if *shell_name as libc::c_int == '.' as i32 {

                    //sh_canonpath in extern file
                    name = sh_canonpath(tname, (PATH_CHECKDOTDOT|PATH_CHECKEXISTS) as libc::c_int);
                    if name.is_null() {
                        name = tname;
                    } else {
                        free(tname as *mut libc::c_void);
                    }
                } else {
                    name = tname;
                }
            }else {
                if (current_user.shell).is_null() {
                    // get_current_user_info  in file of shell.c
                    get_current_user_info();
                }
                name = savestring!(current_user.shell);
            }
        } else {
            name = full_pathname(tname);
            free(tname as *mut libc::c_void);
        }
    }
    return name;

}

#[no_mangle]
pub unsafe extern "C" fn adjust_shell_level(mut change: libc::c_int) {

    let mut new_level: [libc::c_char; 5] = [0; 5];
    let mut old_SHLVL: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut old_level: intmax_t = 0;
    let mut temp_var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;

    old_SHLVL = get_string_value(b"SHLVL\0" as *const u8 as *const libc::c_char);
    if old_SHLVL.is_null() || *old_SHLVL as libc::c_int == '\0' as i32
        || legal_number(old_SHLVL, &mut old_level) == 0 as libc::c_int
    {
        old_level = 0 as libc::c_int as intmax_t ;
    }
    shell_level = (old_level + change as libc::c_long) as libc::c_int;
    if shell_level < 0 as libc::c_int {
        shell_level = 0 as libc::c_int;
    } else if shell_level >= 1000 as libc::c_int {
        internal_warning(
                0 as *const libc::c_char,
                b"shell level (%d) too high, resetting to 1\0" as *const u8
                    as *const libc::c_char,
            shell_level
        );
        shell_level = 1 as libc::c_int;
    }
    if shell_level < 10 as libc::c_int {
        new_level[0 as libc::c_int
            as usize] = (shell_level + '0' as i32) as libc::c_char;
        new_level[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    } 
    else if shell_level < 100 as libc::c_int {
        new_level[0 as libc::c_int
            as usize] = (shell_level / 10 as libc::c_int + '0' as i32) as libc::c_char;
        new_level[1 as libc::c_int
            as usize] = (shell_level % 10 as libc::c_int + '0' as i32) as libc::c_char;
        new_level[2 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    }
    else if shell_level < 1000 as libc::c_int {
        new_level[0 as libc::c_int
            as usize] = (shell_level / 100 as libc::c_int + '0' as i32) as libc::c_char;
        old_level = (shell_level % 100 as libc::c_int) as intmax_t;
        new_level[1 as libc::c_int
            as usize] = (old_level / 10 as libc::c_int as libc::c_long
            + '0' as i32 as libc::c_long) as libc::c_char;
        new_level[2 as libc::c_int
            as usize] = (old_level % 10 as libc::c_int as libc::c_long
            + '0' as i32 as libc::c_long) as libc::c_char;
        new_level[3 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    }

    temp_var = bind_variable(
        b"SHLVL\0" as *const u8 as *const libc::c_char,
        new_level.as_mut_ptr(),
        0 as libc::c_int,
    );

    set_auto_export!(temp_var);

}

unsafe extern "C" fn initialize_shell_level() {
    adjust_shell_level(1 as libc::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn set_ppid() {
    // 暂时定义的宏 有问题 还得修改
    let mut namebuf: [libc::c_char; (INT_STRLEN_BOUND!(uid_t) +1) as usize] = [0; (INT_STRLEN_BOUND!(uid_t) +1) as usize];
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp_var : *mut SHELL_VAR;

    name = inttostr(
        getppid() as intmax_t,
        namebuf.as_mut_ptr(),
        std::mem::size_of::<[libc::c_char; (INT_STRLEN_BOUND!(uid_t) +1) as usize]>() as libc::c_ulong as size_t
    );
    temp_var = find_variable (b"PPID\0" as *const u8 as *const libc::c_char);
    if !temp_var.is_null() {
        VUNSETATTR!(temp_var,(att_readonly|att_exported));
    }

    temp_var = bind_variable (
        b"PPID\0" as *const u8 as *const libc::c_char,
        name,
        0 as libc::c_int);

    VSETATTR!(temp_var,(att_readonly | att_integer)) ;

}

unsafe extern "C" fn uidset() {
    // INT_STRLEN_BOUND(uid_t) + 1
    let mut buff: [libc::c_char; (INT_STRLEN_BOUND!(uid_t) +1) as usize] = [0;(INT_STRLEN_BOUND!(uid_t) +1) as usize ];
    let mut b: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;

    b = inttostr(
        current_user.uid as intmax_t,
        buff.as_mut_ptr(),
        std::mem::size_of::<[libc::c_char; (INT_STRLEN_BOUND!(uid_t) +1) as usize]>() as libc::c_ulong as size_t
    );
    v = find_variable(b"UID\0" as *const u8 as *const libc::c_char);
    if v.is_null() {
        v = bind_variable(
            b"UID\0" as *const u8 as *const libc::c_char,
            b,
            0 as libc::c_int,
        );
        VSETATTR!(v,(att_readonly | att_integer) as libc::c_int);
    }
    if current_user.euid != current_user.uid {
        b = inttostr(
            current_user.euid as intmax_t,
            buff.as_mut_ptr(),
            std::mem::size_of::< [libc::c_char; (INT_STRLEN_BOUND!(uid_t) +1) as usize]>() as libc::c_ulong  as size_t
        );
    }
    v = find_variable(b"EUID\0" as *const u8 as *const libc::c_char);
    if v.is_null() {
        v = bind_variable(
            b"EUID\0" as *const u8 as *const libc::c_char,
            b,
            0 as libc::c_int,
        );
        VSETATTR!(v,(att_readonly | att_integer));
    }
}

unsafe extern "C" fn make_vers_array() {

    let mut vv: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut av: *mut ARRAY = 0 as *mut ARRAY;

    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut d: [libc::c_char; 32] = [0; 32];

    const b: [libc::c_char; (INT_STRLEN_BOUND!(libc::c_int) + 1) as usize]
     = [ 0 as libc::c_char; (INT_STRLEN_BOUND!(libc::c_int) + 1) as usize];
    unbind_variable_noref(b"BASH_VERSINFO\0" as *const u8 as *const libc::c_char);
    
    vv = make_new_array_variable (
        b"BASH_VERSINFO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    av = array_cell!(vv) ;
    strcpy(d.as_mut_ptr(), dist_version);
    s = libc::strchr(d.as_mut_ptr(), '.' as i32);
    if !s.is_null() {
        *s = '\0' as i32 as libc::c_char;
        s = s.offset(1 as isize);
    }
    array_insert(av, 0 as libc::c_int as arrayind_t, d.as_mut_ptr());
    array_insert(av, 1 as libc::c_int as arrayind_t, s);
    
    s = inttostr(
        patch_level as intmax_t,
        b.as_mut_ptr(),
        std::mem::size_of::<[libc::c_char; (INT_STRLEN_BOUND!(libc::c_int) + 1) as usize]>() as libc::c_ulong as usize
    );
    array_insert(av, 2 as libc::c_int as arrayind_t, s);
    s = inttostr(
        build_version as intmax_t,
        b.as_mut_ptr(),
        std::mem::size_of::<[libc::c_char; (INT_STRLEN_BOUND!(libc::c_int) + 1) as usize]>() as libc::c_ulong as usize
    );
    array_insert(av, 3 as libc::c_int as arrayind_t, s);
    array_insert(av, 4 as libc::c_int as arrayind_t, release_status);
    array_insert(
        av,
        5 as libc::c_int as arrayind_t,
        get_mach_type() 
    );
    VSETATTR!(vv, att_readonly);
}

#[no_mangle]
pub unsafe extern "C" fn sh_set_lines_and_columns(
    mut lines: libc::c_int,
    mut cols: libc::c_int
) {
    let mut val: [libc::c_char; (INT_STRLEN_BOUND!(libc::c_int)+1) as usize] 
    = [0 as libc::c_char; (INT_STRLEN_BOUND!(libc::c_int)+1) as usize];

    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    if winsize_assignment != 0 {
        return;
    }

    v = inttostr(
        lines as intmax_t,
        val.as_mut_ptr(),
        std::mem::size_of::<[libc::c_char;(INT_STRLEN_BOUND!(libc::c_int)+1) as usize]>() as libc::c_ulong as usize
    );

    bind_variable( b"LINES\0" as *const u8 as *const libc::c_char,
                    v, 0 as libc::c_int);
    v = inttostr(
        cols as intmax_t,
        val.as_mut_ptr(),
        std::mem::size_of::<[libc::c_char; (INT_STRLEN_BOUND!(libc::c_int)+1) as usize]>() as libc::c_ulong as usize
    );
    bind_variable(b"COLUMNS\0" as *const u8 as *const libc::c_char,
                    v, 0 as libc::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn print_var_list(mut list: *mut *mut SHELL_VAR) {

    let mut i: libc::c_int = 0 as libc::c_int;
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;

    while !list.is_null()
        && {
            var = *list.offset(i as isize);
            !var.is_null()
        }
    {
        if invisible_p!(var) == 0 {
            print_assignment(var);
        } 
        i += 1 ;
    }
}

#[no_mangle]
pub unsafe extern "C" fn print_func_list(mut list: *mut *mut SHELL_VAR) {
    let mut i: libc::c_int = 0;
    let mut var: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    i = 0 as libc::c_int;

    while !list.is_null()
        && {
            var = *list.offset(i as isize);
            !var.is_null()
        }
    {
        libc::printf(b"%s \0" as *const u8 as *const libc::c_char, (*var).name);
        print_var_function(var);
        libc::printf(b"\n\0" as *const u8 as *const libc::c_char);
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn print_assignment(var: *mut SHELL_VAR) {

    if !var_isset!(var) {
        return;
    }
    if function_p!(var) != 0 {
        libc::printf(b"%s\0" as *const u8 as *const libc::c_char, (*var).name);
        print_var_function(var);
        libc::printf(b"\n\0" as *const u8 as *const libc::c_char);
    } 
    else if array_p!(var) != 0
     {        
        print_array_assignment(var, 0 as libc::c_int);
    } else if assoc_p!(var)!= 0 {
        print_assoc_assignment(var, 0 as libc::c_int);
    } 
    else {
        libc::printf(b"%s=\0" as *const u8 as *const libc::c_char, (*var).name);
        print_var_value(var, 1 as libc::c_int);
        libc::printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
}

pub unsafe extern "C" fn print_var_value(
    mut var: *mut SHELL_VAR,
    mut quote: libc::c_int)
{
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    if !var_isset!(var)
    {
        return;
    }
    if quote != 0 && posixly_correct == 0 as libc::c_int
        && ansic_shouldquote(value_cell!(var)) != 0
    {
        t = ansic_quote(value_cell!(var), 0 as libc::c_int, 0 as *mut libc::c_int);
        libc::printf(b"%s\0" as *const u8 as *const libc::c_char, t);
        if !t.is_null() {
            libc::free(t as *mut libc::c_void);
        }
    } else if quote != 0 && sh_contains_shell_metas(value_cell!(var)) != 0 {
        t = sh_single_quote(value_cell!(var));
        libc::printf(b"%s\0" as *const u8 as *const libc::c_char, t);
        if !t.is_null() {
            libc::free(t as *mut libc::c_void);
        }
    } else {
        libc::printf(b"%s\0" as *const u8 as *const libc::c_char, value_cell!(var));
    };
}

#[no_mangle]
pub unsafe extern "C" fn print_var_function(mut var: *mut SHELL_VAR) {
    let mut x: *mut libc::c_char = 0 as *mut libc::c_char;
    if function_p!(var) != 0 as libc::c_int && var_isset!(var) {
        x = named_function_string(
            0 as *mut libc::c_void as *mut libc::c_char,
            function_cell!(var),
            (FUNC_MULTILINE|FUNC_EXTERNAL) as libc::c_int
        );
        libc::printf(b"%s\0" as *const u8 as *const libc::c_char, x);
    }
}

unsafe extern "C" fn null_assign(
    mut self_0: *mut SHELL_VAR,
    mut value: *mut libc::c_char,
    mut unused: arrayind_t,
    mut key: *mut libc::c_char,
) -> *mut SHELL_VAR {
    return self_0;
}

unsafe extern "C" fn null_array_assign(
    mut self_0: *mut SHELL_VAR,
    mut value: *mut libc::c_char,
    mut ind: arrayind_t,
    mut key: *mut libc::c_char,
) -> *mut SHELL_VAR {
    return self_0;
}

unsafe extern "C" fn get_self(mut self_0: *mut SHELL_VAR) -> *mut SHELL_VAR {
    return self_0;
}

unsafe extern "C" fn init_dynamic_array_var(
    mut name: *mut libc::c_char,
    mut getfunc: sh_var_value_func_t,
    mut setfunc: sh_var_assign_func_t,
    mut attrs: libc::c_int
) -> *mut SHELL_VAR {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    v = find_variable(name);
    if !v.is_null() {
        return v;
    }
    INIT_DYNAMIC_ARRAY_VAR!(v,name,getfunc,setfunc);
    if attrs != 0 {
        VSETATTR!(v,attrs);
    }
    return v;
}

unsafe extern "C" fn init_dynamic_assoc_var(
    mut name: *mut libc::c_char,
    mut getfunc:sh_var_value_func_t,
    mut setfunc:sh_var_assign_func_t,
    mut attrs: libc::c_int,
) -> *mut SHELL_VAR {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    v = find_variable(name);
    if !v.is_null() {
        return v;
    }

    INIT_DYNAMIC_ASSOC_VAR!(v,name,getfunc,setfunc);
    
    if attrs != 0 {
        VSETATTR!(v,attrs);
    }
    return v;
}

static mut seconds_value_assigned: intmax_t = 0;

unsafe extern "C" fn assign_seconds(
    mut self_0: *mut SHELL_VAR,
    mut value: *mut libc::c_char,
    mut unused: arrayind_t,
    mut key: *mut libc::c_char,
) -> *mut SHELL_VAR {

    let mut nval: intmax_t = 0;
    let mut expok: libc::c_int = 0;
    if  integer_p!(self_0) != 0 as libc::c_int {
        nval = evalexp(value, 0 as libc::c_int, &mut expok);
    } else {
        expok = legal_number(value, &mut nval);
    }
    seconds_value_assigned = if expok != 0 {
        nval
    } else {
        0 as libc::c_int as libc::c_long
    };
    gettimeofday(&mut shellstart, 0 as *mut timezone);
    shell_start_time = shellstart.tv_sec;
    return self_0;
}
