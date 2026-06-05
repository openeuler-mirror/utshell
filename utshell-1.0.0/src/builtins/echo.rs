//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

use crate::builtins::common::sh_chkwrite;
use crate::readline::c_clearerr;
use crate::src_common::*;

/* System V machines already have a /bin/sh with a v9 behaviour.  We
give Bash the identical behaviour for these machines so that the
existing system shells won't barf.  Regrettably, the SUS v2 has
standardized the Sys V echo behavior.  This variable is external
so that we can have a `shopt' variable to control it at runtime. */

pub static mut xpg_echo: i32 = 0;

/* Print the words in LIST to standard output.  If the first word is
`-n', then don't print a trailing newline.  We also support the
echo syntax from Version 9 Unix systems. */

#[no_mangle]
pub fn echo_builtin(mut list: *mut WordList) -> i32 {
    let mut display_return: i32;
    let mut do_v9: i32;
    let mut i: i32;
    let mut len: i32;
    let mut temp: *mut libc::c_char = std::ptr::null_mut();
    let mut s: *mut libc::c_char;

    unsafe {
        do_v9 = xpg_echo;
        display_return = 1;
        if !list.is_null()
            && (*list).word != std::ptr::null_mut()
            && (*(*list).word).word != std::ptr::null_mut()
        {
            temp = (*(*list).word).word;
        }
    }
    while unsafe { !list.is_null() && *temp == '-' as libc::c_char } {
        /* If it appears that we are handling options, then make sure that
        all of the options specified are actually valid.  Otherwise, the
        string should just be echoed. */

        temp = (temp as usize + 1) as *mut libc::c_char;
        let mut t = temp;
        i = 0;

        while unsafe { *temp as i32 != 0 } {
            let s = unsafe { *temp as i32 };
            let su8 = s as u8;
            let s_opt = char::from(su8);

            let msg = CString::new("neE").unwrap();
            if unsafe { strchr(msg.as_ptr(), s_opt as libc::c_int).is_null() } {
                break;
            }

            temp = (temp as usize + 1) as *mut libc::c_char;
            i += 1;
        }
        //
        /* echo - and echo -<nonopt> both mean to just echo the arguments. */
        if unsafe { *t == 0 || *((t as usize + i as usize) as *mut libc::c_char) != 0 } {
            break;
        }

        /* All of the options in TEMP are valid options to ECHO.
        Handle them. */
        while !t.is_null() {
            let optu8 = unsafe { *t as u8 };
            let opt_char = char::from(optu8);

            match opt_char {
                'n' => {
                    display_return = 0;
                }
                'e' => {
                    do_v9 = 1;
                }
                'E' => {
                    do_v9 = 0;
                }
                _ => break,
            }
            t = (t as usize + 1) as *mut libc::c_char;
        }

        unsafe {
            list = (*list).next;
            if !(*(*list).word).word.is_null() {
                temp = (*(*list).word).word;
            }
        }
    }

    unsafe {
        c_clearerr(stdout);
    } /* clear error before writing and testing success */

    while list != std::ptr::null_mut() {
        i = 0;
        len = 0;
        unsafe {
            if do_v9 != 0 {
                temp = c_ansicstr(
                    (*(*list).word).word,
                    STRLEN!((*(*list).word).word),
                    1,
                    &mut i,
                    &mut len,
                );
            } else {
                temp = (*(*list).word).word;
            }
        }
        if temp != std::ptr::null_mut() {
            if do_v9 != 0 {
                s = temp;

                for _ in 0..len {
                    unsafe {
                        putchar(*s as libc::c_int);
                    }
                    s = (s as usize + 1) as *mut libc::c_char;
                }
            } else {
                unsafe {
                    fprintf(stdout, temp);
                }
            }
        }

        unsafe {
            QUIT!();
        }
        if do_v9 != 0 && temp != std::ptr::null_mut() {
            unsafe {
                free(temp as *mut c_void);
            }
        }

        unsafe {
            list = (*list).next;
        }
        if i != 0 {
            display_return = 0;
            break;
        }

        if list != std::ptr::null_mut() {
            unsafe {
                putchar(' ' as i32);
                QUIT!();
            }
        }
    } //while

    if display_return != 0 {
        unsafe {
            putchar('\n' as i32);
        }
    }

    return sh_chkwrite(EXECUTION_SUCCESS!());
}
