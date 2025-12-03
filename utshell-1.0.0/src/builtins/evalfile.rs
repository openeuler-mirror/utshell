//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later
use libc::{__errno_location, close, free, malloc, memmove, open, read, strlen};

use crate::array::{array_dispose_element, array_rshift, array_shift};
use crate::builtins::evalstring::{parse_and_execute, parse_and_execute_cleanup};
use crate::error::file_error;
use crate::execute_cmd::{executing_line_number, restore_funcarray_state};
use crate::general::{bash_tilde_expand, check_binary_file};
use crate::sig::jump_to_top_level;
use crate::src_common::*;
use crate::stringlib::xbcopy;
use crate::trap::run_return_trap;
use crate::unwind_prot::{
    add_unwind_protect, begin_unwind_frame, run_unwind_frame, unwind_protect_mem,
};
use crate::variables::{find_variable, init_bash_argv, pop_args};
use crate::version::shell_compatibility_level;
use crate::y_tab::{current_token, push_token};

#[no_mangle]
pub static mut sourcelevel: libc::c_int = 0 as libc::c_int;
fn evalfile(filename: *const libc::c_char, flags: libc::c_int) -> libc::c_int {
    unsafe {
        let mut old_interactive: libc::c_int = 0;
        let mut old_return_catch: sigjmp_buf = [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1];
        let mut return_val: libc::c_int = 0;
        let mut fd: libc::c_int = 0;
        let mut result: libc::c_int = 0;
        let mut pflags: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut nnull: libc::c_int = 0;
        let mut nr: ssize_t = 0;
        let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut finfo: crate::src_common::stat = crate::src_common::stat_init;
        let mut file_size: size_t = 0;
        let mut errfunc: sh_vmsg_func_t = None;
        let mut funcname_v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
        let mut bash_source_v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
        let mut bash_lineno_v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
        let mut funcname_a: *mut ARRAY = 0 as *mut ARRAY;
        let mut bash_source_a: *mut ARRAY = 0 as *mut ARRAY;
        let mut bash_lineno_a: *mut ARRAY = 0 as *mut ARRAY;
        let mut fa: *mut func_array_state = 0 as *mut func_array_state;
        let mut bash_argv_v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
        let mut bash_argc_v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
        let mut bash_argv_a: *mut ARRAY = 0 as *mut ARRAY;
        let mut bash_argc_a: *mut ARRAY = 0 as *mut ARRAY;
        let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut tt: [libc::c_char; 2] = [0; 2];
        funcname_v = find_variable(b"FUNCNAME\0" as *const u8 as *const libc::c_char);
        funcname_a = if !funcname_v.is_null()
            && (*funcname_v).attributes & FEVAL_UNWINDPROT!() as libc::c_int != 0
        {
            (*funcname_v).value as *mut ARRAY
        } else {
            0 as *mut ARRAY
        };
        bash_source_v = find_variable(b"BASH_SOURCE\0" as *const u8 as *const libc::c_char);
        bash_source_a = if !bash_source_v.is_null()
            && (*bash_source_v).attributes & FEVAL_UNWINDPROT!() as libc::c_int != 0
        {
            (*bash_source_v).value as *mut ARRAY
        } else {
            0 as *mut ARRAY
        };
        bash_lineno_v = find_variable(b"BASH_LINENO\0" as *const u8 as *const libc::c_char);
        bash_lineno_a = if !bash_lineno_v.is_null()
            && (*bash_lineno_v).attributes & FEVAL_UNWINDPROT!() as libc::c_int != 0
        {
            (*bash_lineno_v).value as *mut ARRAY
        } else {
            0 as *mut ARRAY
        };
        bash_argv_v = find_variable(b"BASH_ARGV\0" as *const u8 as *const libc::c_char);
        bash_argv_a = if !bash_argv_v.is_null()
            && (*bash_argv_v).attributes & FEVAL_UNWINDPROT!() as libc::c_int != 0
        {
            (*bash_argv_v).value as *mut ARRAY
        } else {
            0 as *mut ARRAY
        };
        bash_argc_v = find_variable(b"BASH_ARGC\0" as *const u8 as *const libc::c_char);
        bash_argc_a = if !bash_argc_v.is_null()
            && (*bash_argc_v).attributes & FEVAL_UNWINDPROT!() as libc::c_int != 0
        {
            (*bash_argc_v).value as *mut ARRAY
        } else {
            0 as *mut ARRAY
        };

        fd = open(filename, 0 as libc::c_int);
        if fd < 0 as libc::c_int || fstat(fd, &mut finfo) == -(1 as libc::c_int) {
            i = *__errno_location();
            if fd >= 0 as libc::c_int {
                close(fd);
            }
            *__errno_location() = i;
            if flags & FEVAL_ENOENTOK!() as libc::c_int == 0 as libc::c_int
                || *__errno_location() != 2 as libc::c_int
            {
                file_error(filename);
            }
            if flags & FEVAL_LONGJMP!() as libc::c_int != 0 {
                ::std::ptr::write_volatile(
                    &mut last_command_exit_value as *mut libc::c_int,
                    1 as libc::c_int,
                );
                jump_to_top_level(3 as libc::c_int);
            }
            return if flags & FEVAL_BUILTIN!() as libc::c_int != 0 {
                1 as libc::c_int
            } else if *__errno_location() == 2 as libc::c_int
                && flags & FEVAL_ENOENTOK!() as libc::c_int != 0 as libc::c_int
            {
                0 as libc::c_int
            } else {
                -(1 as libc::c_int)
            };
        } else {
            errfunc = if flags & FEVAL_BUILTIN!() as libc::c_int != 0 {
                Some(builtin_error)
            } else {
                Some(internal_error)
            };
            if finfo.st_mode & __S_IFMT!() as libc::c_int as libc::c_uint
                == __S_IFDIR!() as libc::c_int as libc::c_uint
            {
                (Some(errfunc.expect("non-null function pointer")))
                    .expect("non-null function pointer")(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: is a directory\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    filename,
                );
                close(fd);
                return if flags & FEVAL_BUILTIN!() as libc::c_int != 0 {
                    1 as libc::c_int
                } else {
                    -(1 as libc::c_int)
                };
            } else {
                if flags & FEVAL_REGFILE!() as libc::c_int != 0
                    && (finfo.st_mode & __S_IFMT!() as libc::c_int as libc::c_uint
                        == __S_IFREG!() as libc::c_int as libc::c_uint)
                        as libc::c_int
                        == 0 as libc::c_int
                {
                    (Some(errfunc.expect("non-null function pointer")))
                        .expect("non-null function pointer")(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: not a regular file\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        filename,
                    );
                    close(fd);
                    return if flags & FEVAL_BUILTIN!() as libc::c_int != 0 {
                        1 as libc::c_int
                    } else {
                        -(1 as libc::c_int)
                    };
                }
            }
            file_size = finfo.st_size as size_t;
            if file_size != finfo.st_size as libc::c_ulong
                || file_size.wrapping_add(1 as libc::c_int as libc::c_ulong) < file_size
            {
                (Some(errfunc.expect("non-null function pointer")))
                    .expect("non-null function pointer")(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: file is too large\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    filename,
                );
                close(fd);
                return if flags & FEVAL_BUILTIN!() as libc::c_int != 0 {
                    1 as libc::c_int
                } else {
                    -(1 as libc::c_int)
                };
            }
            if finfo.st_mode & __S_IFMT!() as libc::c_int as libc::c_uint
                == __S_IFREG!() as libc::c_int as libc::c_uint
                && file_size <= SSIZE_MAX!() as libc::c_long as libc::c_ulong
            {
                string = malloc((1 as libc::c_int as libc::c_ulong).wrapping_add(file_size) as usize)
                    as *mut libc::c_char;
                nr = read(fd, string as *mut libc::c_void, file_size as usize) as ssize_t;
                if nr >= 0 as libc::c_int as libc::c_long {
                    *string.offset(nr as isize) = '\u{0}' as i32 as libc::c_char;
                }
            } else {
                nr = zmapfd(fd, &mut string, 0 as *mut libc::c_char) as ssize_t;
            }
            return_val = *__errno_location();
            close(fd);
            *__errno_location() = return_val;
            if nr < 0 as libc::c_int as libc::c_long {
                free(string as *mut libc::c_void);
                if flags & FEVAL_ENOENTOK!() as libc::c_int == 0 as libc::c_int
                    || *__errno_location() != 2 as libc::c_int
                {
                    file_error(filename);
                }
                if flags & FEVAL_LONGJMP!() as libc::c_int != 0 {
                    ::std::ptr::write_volatile(
                        &mut last_command_exit_value as *mut libc::c_int,
                        1 as libc::c_int,
                    );
                    jump_to_top_level(3 as libc::c_int);
                }
                return if flags & FEVAL_BUILTIN!() as libc::c_int != 0 {
                    1 as libc::c_int
                } else if *__errno_location() == 2 as libc::c_int
                    && flags & FEVAL_ENOENTOK!() as libc::c_int != 0 as libc::c_int
                {
                    0 as libc::c_int
                } else {
                    -(1 as libc::c_int)
                };
            } else {
                if nr == 0 as libc::c_int as libc::c_long {
                    free(string as *mut libc::c_void);
                    return if flags & FEVAL_BUILTIN!() as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        1 as libc::c_int
                    };
                }
                if flags & FEVAL_CHECKBINARY!() as libc::c_int != 0
                    && check_binary_file(
                        string,
                        (if nr > 80 as libc::c_int as libc::c_long {
                            80 as libc::c_int as libc::c_long
                        } else {
                            nr
                        }) as libc::c_int,
                    ) != 0
                {
                    free(string as *mut libc::c_void);
                    (Some(errfunc.expect("non-null function pointer")))
                        .expect("non-null function pointer")(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: cannot execute binary file\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        filename,
                    );
                    return if flags & FEVAL_BUILTIN!() as libc::c_int != 0 {
                        126 as libc::c_int
                    } else {
                        -(1 as libc::c_int)
                    };
                }
                i = strlen(string) as libc::c_int;
                if (i as libc::c_long) < nr {
                    i = 0 as libc::c_int;
                    nnull = i;
                    while (i as libc::c_long) < nr {
                        if *string.offset(i as isize) as libc::c_int == '\u{0}' as i32 {
                            memmove(
                                string.offset(i as isize) as *mut libc::c_void,
                                string.offset(i as isize).offset(1 as libc::c_int as isize)
                                    as *const libc::c_void,
                                (nr - i as libc::c_long) as usize,
                            );
                            nr -= 1;
                            if flags & FEVAL_BUILTIN!() as libc::c_int != 0 && {
                                nnull += 1;
                                nnull > 256 as libc::c_int
                            } {
                                free(string as *mut libc::c_void);
                                (Some(errfunc.expect("non-null function pointer")))
                                    .expect("non-null function pointer")(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"%s: cannot execute binary file\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    filename,
                                );
                                return if flags & FEVAL_BUILTIN!() as libc::c_int != 0 {
                                    126 as libc::c_int
                                } else {
                                    -(1 as libc::c_int)
                                };
                            }
                        }
                        i += 1;
                    }
                }
                if flags & FEVAL_UNWINDPROT!() as libc::c_int != 0 {
                    begin_unwind_frame(
                        b"evalfile\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                    unwind_protect_mem(
                        &mut return_catch_flag as *mut libc::c_int as *mut libc::c_char,
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
                    );
                    unwind_protect_mem(
                        &mut return_catch as *mut sigjmp_buf as *mut libc::c_char,
                        ::std::mem::size_of::<sigjmp_buf>() as libc::c_ulong as libc::c_int,
                    );
                    if flags & FEVAL_NONINT!() as libc::c_int != 0 {
                        unwind_protect_mem(
                            &mut interactive as *mut libc::c_int as *mut libc::c_char,
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
                        );
                    }
                    unwind_protect_mem(
                        &mut sourcelevel as *mut libc::c_int as *mut libc::c_char,
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
                    );
                } else {
                    xbcopy(
                        return_catch.as_mut_ptr() as *mut libc::c_char,
                        old_return_catch.as_mut_ptr() as *mut libc::c_char,
                        ::std::mem::size_of::<sigjmp_buf>() as libc::c_ulong as libc::c_int,
                    );
                    if flags & FEVAL_NONINT!() as libc::c_int != 0 {
                        ::std::ptr::write_volatile(
                            &mut old_interactive as *mut libc::c_int,
                            interactive,
                        );
                    }
                }
                if flags & FEVAL_NONINT!() as libc::c_int != 0 {
                    interactive = 0 as libc::c_int;
                }
                return_catch_flag += 1;
                sourcelevel += 1;
                array_rshift(
                    bash_source_a,
                    1 as libc::c_int,
                    filename as *mut libc::c_char,
                );
                t = itos(executing_line_number() as intmax_t);
                array_rshift(bash_lineno_a, 1 as libc::c_int, t);
                free(t as *mut libc::c_void);
                array_rshift(
                    funcname_a,
                    1 as libc::c_int,
                    b"source\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                fa = malloc(::std::mem::size_of::<func_array_state>() as usize)
                    as *mut func_array_state;
                let ref mut fresh0 = (*fa).source_a;
                *fresh0 = bash_source_a;
                let ref mut fresh1 = (*fa).source_v;
                *fresh1 = bash_source_v;
                let ref mut fresh2 = (*fa).lineno_a;
                *fresh2 = bash_lineno_a;
                let ref mut fresh3 = (*fa).lineno_v;
                *fresh3 = bash_lineno_v;
                let ref mut fresh4 = (*fa).funcname_a;
                *fresh4 = funcname_a;
                let ref mut fresh5 = (*fa).funcname_v;
                *fresh5 = funcname_v;
                // let rfs: Option<Function> = Some(restore_funcarray_state);
                if flags & FEVAL_UNWINDPROT!() as libc::c_int != 0 {
                    add_unwind_protect(
                        std::mem::transmute::<fn(*mut func_array_state) -> (), Option<Function>>(
                            restore_funcarray_state,
                        ),
                        fa as *mut libc::c_char,
                    );
                }
                if flags & FEVAL_NOPUSHARGS!() as libc::c_int == 0 as libc::c_int {
                    if shell_compatibility_level <= 44 as libc::c_int {
                        init_bash_argv();
                    }
                    array_rshift(bash_argv_a, 1 as libc::c_int, filename as *mut libc::c_char);
                    tt[0 as libc::c_int as usize] = '1' as i32 as libc::c_char;
                    tt[1 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
                    array_rshift(bash_argc_a, 1 as libc::c_int, tt.as_mut_ptr());
                    // let pa: Functions = Functions { pop_args };
                    if flags & FEVAL_UNWINDPROT!() as libc::c_int != 0 {
                        add_unwind_protect(
                            std::mem::transmute::<fn(), Option<Function>>(pop_args),
                            0 as *mut libc::c_char,
                        );
                    }
                }
                pflags = FEVAL_LONGJMP!() as libc::c_int;
                pflags |= if flags & FEVAL_HISTORY!() as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    FEVAL_UNWINDPROT!() as libc::c_int
                };
                if flags & FEVAL_BUILTIN!() as libc::c_int != 0 {
                    result = 0 as libc::c_int;
                }
                return_val = __sigsetjmp(return_catch.as_mut_ptr(), 0 as libc::c_int); //问题
                if return_val != 0 {
                    parse_and_execute_cleanup(-(1 as libc::c_int));
                    result = return_catch_value;
                } else {
                    result = parse_and_execute(string, filename, pflags);
                }
                if flags & FEVAL_UNWINDPROT!() as libc::c_int != 0 {
                    run_unwind_frame(
                        b"evalfile\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                } else {
                    if flags & FEVAL_NONINT!() as libc::c_int != 0 {
                        interactive = old_interactive;
                    }
                    restore_funcarray_state(fa);
                    if flags & FEVAL_NOPUSHARGS!() as libc::c_int == 0 as libc::c_int {
                        array_dispose_element(array_shift(
                            bash_argc_a,
                            1 as libc::c_int,
                            0 as libc::c_int,
                        ));
                        array_dispose_element(array_shift(
                            bash_argv_a,
                            1 as libc::c_int,
                            0 as libc::c_int,
                        ));
                    }
                    return_catch_flag -= 1;
                    sourcelevel -= 1;
                    xbcopy(
                        old_return_catch.as_mut_ptr() as *mut libc::c_char,
                        return_catch.as_mut_ptr() as *mut libc::c_char,
                        ::std::mem::size_of::<sigjmp_buf>() as libc::c_ulong as libc::c_int,
                    );
                }
                if current_token == yacc_EOF!() as libc::c_int {
                    push_token('\n' as i32);
                }
                return if flags & FEVAL_BUILTIN!() as libc::c_int != 0 {
                    result
                } else {
                    1 as libc::c_int
                };
            }
        }
    }
}
#[no_mangle]
pub fn maybe_execute_file(
    fname: *const libc::c_char,
    force_noninteractive: libc::c_int,
) -> libc::c_int {
    unsafe {
        let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut result: libc::c_int = 0;
        let mut flags: libc::c_int = 0;
        filename = bash_tilde_expand(fname, 0 as libc::c_int);
        flags = FEVAL_ENOENTOK!() as libc::c_int;
        if force_noninteractive != 0 {
            flags |= FEVAL_NONINT!() as libc::c_int;
        }
        result = evalfile(filename, flags);
        free(filename as *mut libc::c_void);
        return result;
    }
}
#[no_mangle]
pub fn force_execute_file(
    fname: *const libc::c_char,
    force_noninteractive: libc::c_int,
) -> libc::c_int {
    unsafe {
        let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut result: libc::c_int = 0;
        let mut flags: libc::c_int = 0;
        filename = bash_tilde_expand(fname, 0 as libc::c_int);
        flags = 0 as libc::c_int;
        if force_noninteractive != 0 {
            flags |= FEVAL_NONINT!() as libc::c_int;
        }
        result = evalfile(filename, flags);
        free(filename as *mut libc::c_void);
        return result;
    }
}
#[no_mangle]
pub fn fc_execute_file(filename: *const libc::c_char) -> libc::c_int {
    let mut flags: libc::c_int = 0;
    flags = FEVAL_ENOENTOK!() as libc::c_int
        | FEVAL_HISTORY!() as libc::c_int
        | FEVAL_REGFILE!() as libc::c_int
        | FEVAL_BUILTIN!() as libc::c_int;
    return evalfile(filename, flags);
}
#[no_mangle]
pub fn source_file(filename: *const libc::c_char, sflags: libc::c_int) -> libc::c_int {
    let mut flags: libc::c_int = 0;
    let mut rval: libc::c_int = 0;
    flags = FEVAL_BUILTIN!() as libc::c_int
        | FEVAL_UNWINDPROT!() as libc::c_int
        | FEVAL_NONINT!() as libc::c_int;
    if sflags != 0 {
        flags |= FEVAL_NOPUSHARGS!() as libc::c_int;
    }
    if unsafe {
        posixly_correct != 0
            && interactive_shell == 0 as libc::c_int
            && executing_command_builtin == 0 as libc::c_int
    } {
        flags |= FEVAL_LONGJMP!() as libc::c_int;
    }
    rval = evalfile(filename, flags);
    run_return_trap();
    return rval;
}
