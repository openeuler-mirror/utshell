
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


#[derive(Copy, Clone)]
#[repr(C)]
pub struct saved_dollar_vars {
    pub first_ten: *mut *mut libc::c_char,
    pub rest: *mut WORD_LIST,
    pub count: libc::c_int,
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
