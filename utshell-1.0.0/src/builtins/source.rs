use crate::builtins::common::{
    builtin_usage, dollar_vars_changed, no_options, remember_args, set_dollar_vars_unchanged,
    sh_restricted,
};
use crate::builtins::evalfile::source_file;
use crate::findcmd::find_path_file;
use crate::general::{absolute_pathname, printable_filename};
use crate::sig::jump_to_top_level;
use crate::src_common::*;
use crate::subst::invalidate_cached_quoted_dollar_at;
use crate::trap::{
    maybe_set_debug_trap, restore_default_signal, signal_is_ignored, signal_is_trapped,
};
use crate::unwind_prot::{add_unwind_protect, begin_unwind_frame, run_unwind_frame};
use crate::variables::{
    dispose_saved_dollar_vars, init_bash_argv, pop_args, pop_dollar_vars, push_args,
    push_dollar_vars,
};
use crate::version::shell_compatibility_level;

// pub union Functions {
//     f_xfree: fn(str1: *mut c_void),
//     f_maybe_pop_dollar_vars: fn(),
//     f_maybe_set_debug_trap: fn(*mut libc::c_char),
// }

#[no_mangle]
pub static mut source_uses_path: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut source_searches_cwd: libc::c_int = 1 as libc::c_int;

#[no_mangle]
pub fn maybe_pop_dollar_vars() {
    if unsafe { variable_context == 0 && (dollar_vars_changed() & ARGS_SETBLTIN!()) != 0 } {
        dispose_saved_dollar_vars();
    } else {
        pop_dollar_vars();
    }
    if unsafe { debugging_mode != 0 } {
        pop_args(); /* restore BASH_ARGC and BASH_ARGV */
    }

    set_dollar_vars_unchanged();
    invalidate_cached_quoted_dollar_at(); /* just invalidate to be safe */
}

fn TRAP_STRING(s: i32) -> *mut libc::c_char {
    if signal_is_trapped(s) != 0 && signal_is_ignored(s) == 0 {
        return unsafe { trap_list[s as usize] };
    } else {
        return std::ptr::null_mut();
    }
}

unsafe fn DEBUG_TRAP() -> i32 {
    return libc::SIGRTMAX() + 1;
}

#[no_mangle]
pub fn source_builtin(list: *mut WordList) -> i32 {
    let result: i32;
    let mut filename: *mut libc::c_char;
    let mut debug_trap: *mut libc::c_char;
    let x: *mut libc::c_char;

    if no_options(list) != 0 {
        return EX_USAGE;
    }
    unsafe {
        let llist: *mut WordList = loptend.clone();

        if list == std::ptr::null_mut() {
            builtin_error(
                b"filename argument required\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            builtin_usage();
            return EX_USAGE;
        }

        if restricted != 0
            && libc::strchr((*(*llist).word).word, '/' as libc::c_int) != std::ptr::null_mut()
        {
            sh_restricted((*(*llist).word).word);
            return EXECUTION_FAILURE!();
        }

        filename = std::ptr::null_mut();
        /* XXX -- should this be absolute_pathname? */
        if posixly_correct != 0
            && libc::strchr((*(*llist).word).word, '/' as libc::c_int) != std::ptr::null_mut()
        {
            filename = savestring!((*(*llist).word).word);
        } else if absolute_pathname((*(*llist).word).word) != 0 {
            filename = savestring!((*(*llist).word).word);
        } else if source_uses_path != 0 {
            filename = find_path_file((*(*llist).word).word);
        }

        if filename == std::ptr::null_mut() {
            if source_searches_cwd == 0 {
                x = printable_filename((*(*llist).word).word, 0);
                let msg = CString::new("%s: file not found").unwrap();
                builtin_error(msg.as_ptr(), x);
                if x != (*(*llist).word).word {
                    libc::free(x as *mut c_void);
                }

                if posixly_correct != 0 && interactive_shell == 0 && executing_command_builtin == 0
                {
                    last_command_exit_value = EXECUTION_FAILURE!();
                    jump_to_top_level(EXITPROG!());
                }
                return EXECUTION_FAILURE!();
            } else {
                filename = savestring!((*(*llist).word).word);
            }
        }

        begin_unwind_frame(b"source\0" as *const u8 as *const libc::c_char as *mut libc::c_char);

        add_unwind_protect(
            std::mem::transmute::<unsafe extern "C" fn(_), Option<Function>>(free),
            filename,
        );

        if (*list).next != std::ptr::null_mut() {
            push_dollar_vars();

            add_unwind_protect(
                std::mem::transmute::<fn(), Option<Function>>(maybe_pop_dollar_vars),
                std::ptr::null_mut(),
            );
            if debugging_mode != 0 || shell_compatibility_level <= 44 {
                init_bash_argv(); /* Initialize BASH_ARGV and BASH_ARGC */
            }

            remember_args((*list).next, 1);
            if debugging_mode != 0 {
                push_args((*list).next); /* Update BASH_ARGV and BASH_ARGC */
            }
        }
        set_dollar_vars_unchanged();

        /* Don't inherit the DEBUG trap unless function_trace_mode (overloaded)
        is set.  XXX - should sourced files inherit the RETURN trap?  Functions
        don't. */
        debug_trap = TRAP_STRING(DEBUG_TRAP());
        if debug_trap != std::ptr::null_mut() && function_trace_mode == 0 {
            debug_trap = savestring!(debug_trap);

            add_unwind_protect(
                std::mem::transmute::<unsafe extern "C" fn(_), Option<Function>>(free),
                debug_trap,
            );

            add_unwind_protect(
                std::mem::transmute::<fn(command: *mut libc::c_char), Option<Function>>(
                    maybe_set_debug_trap,
                ),
                debug_trap,
            );

            restore_default_signal(DEBUG_TRAP());
        }

        result = source_file(
            filename,
            (list != std::ptr::null_mut() && (*list).next != std::ptr::null_mut()) as i32,
        );

        run_unwind_frame(b"source\0" as *const u8 as *const libc::c_char as *mut libc::c_char);

        return result;
    }
}
