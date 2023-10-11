use libc::*;
use r_bash::*;
use std::ffi::CStr;

extern "C" {
    fn xmalloc(_: size_t) -> *mut libc::c_void;
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
pub unsafe extern "C" fn bind_variable(
    mut name: *const libc::c_char,
    mut value: *mut libc::c_char,
    mut flags: libc::c_int,
) -> *mut SHELL_VAR {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut nv: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut vc: *mut VAR_CONTEXT = 0 as *mut VAR_CONTEXT;
    let mut nvc: *mut VAR_CONTEXT = 0 as *mut VAR_CONTEXT;

   
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
