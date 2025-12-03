//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later
use super::help::builtin_help;
use crate::builtins::cd::cd_builtin;
use crate::builtins::common::{
    builtin_usage, get_working_directory, sh_chkwrite, sh_erange, sh_invalidnum, sh_invalidopt,
};
use crate::dispose_cmd::dispose_words;
use crate::general::{legal_number, polite_directory_format};
use crate::make_cmd::{make_word, make_word_list};
use crate::src_common::*;
use crate::variables::get_string_value;

fn STREQ(a: *const libc::c_char, b: *const libc::c_char) -> bool {
    unsafe {
        return *a == *b && libc::strcmp(a, b) == 0;
    }
}

fn ISHELP(s: *const libc::c_char) -> bool {
    return STREQ(s, CString::new("--help").unwrap().as_ptr());
}

fn ISOPTION(s: *const libc::c_char, c: libc::c_char) -> bool {
    unsafe {
        return *s == '-' as libc::c_char && *(s.offset(1)) == c && *(s.offset(2)) == 0;
    }
}

#[no_mangle]
pub fn pushd_builtin(listt: *mut WordList) -> i32 {
    let orig_list: *mut WordList;
    let mut temp: *mut libc::c_char;
    let current_directory: *mut libc::c_char;
    let mut top: *mut libc::c_char;
    let mut j: i32;
    let mut flags: i32;
    let skipopt: i32;
    let mut num: libc::c_long = 0;
    let mut direction: libc::c_char;

    unsafe {
        let mut list: *mut WordList = listt.clone();
        orig_list = list.clone();

        if list != std::ptr::null_mut()
            && (*list).word != std::ptr::null_mut()
            && ISHELP((*((*list).word)).word)
        {
            builtin_help();
            return EX_USAGE;
        }

        if list != std::ptr::null_mut()
            && (*list).word != std::ptr::null_mut()
            && ISOPTION((*((*list).word)).word, '-' as libc::c_char)
        {
            list = (*list).next;
            skipopt = 1;
        } else {
            skipopt = 0;
        }

        /* If there is no argument list then switch current and
        top of list. */
        if list == std::ptr::null_mut() {
            if directory_list_offset == 0 {
                builtin_error(CString::new("no other directory").unwrap().as_ptr());
                return EXECUTION_FAILURE!();
            }

            current_directory =
                get_working_directory(CString::new("pushd").unwrap().as_ptr() as *mut libc::c_char);
            if current_directory == std::ptr::null_mut() {
                return EXECUTION_FAILURE!();
            }

            j = directory_list_offset - 1;
            temp = *((pushd_directory_list as usize + (j * 8) as usize) as *mut *mut libc::c_char);
            *((pushd_directory_list as usize + (j * 8) as usize) as *mut *mut libc::c_char) =
                current_directory;
            j = change_to_temp(temp);
            libc::free(temp as *mut c_void);
            return j;
        }

        flags = 0;

        while skipopt == 0 && list != std::ptr::null_mut() {
            if ISOPTION((*((*list).word)).word, 'n' as libc::c_char) {
                flags |= NOCD!();
            } else if ISOPTION((*((*list).word)).word, '-' as libc::c_char) {
                list = (*list).next;
                break;
            } else if *((*((*list).word)).word) == '-' as libc::c_char
                && *(((*((*list).word)).word as usize + 1) as *mut libc::c_char)
                    == '\0' as libc::c_char
            {
                /* Let `pushd -' work like it used to. */
                break;
            } else {
                direction = *((*((*list).word)).word);
                if direction == '+' as libc::c_char || direction == '-' as libc::c_char {
                    if legal_number(
                        ((*((*list).word)).word as usize + 1) as *mut libc::c_char,
                        &mut num,
                    ) == 0
                    {
                        sh_invalidnum((*((*list).word)).word);
                        builtin_usage();
                        return EX_USAGE;
                    }

                    if direction == '-' as libc::c_char {
                        num = directory_list_offset as libc::c_long - num;
                    }

                    if num > directory_list_offset as libc::c_long || num < 0 {
                        pushd_error(directory_list_offset, (*((*list).word)).word);
                        return EXECUTION_FAILURE!();
                    }
                    flags |= ROTATE!();
                } else if *((*((*list).word)).word) == '-' as libc::c_char {
                    sh_invalidopt((*((*list).word)).word);
                    builtin_usage();
                    return EX_USAGE;
                } else {
                    break;
                }
            }
            list = (*list).next;
        }

        if (flags & ROTATE!()) != 0 {
            /* Rotate the stack num times.  Remember, the current
            directory acts like it is part of the stack. */
            temp =
                get_working_directory(CString::new("pushd").unwrap().as_ptr() as *mut libc::c_char);

            if num == 0 {
                if (flags & NOCD!()) == 0 {
                    j = change_to_temp(temp);
                } else {
                    j = EXECUTION_SUCCESS!();
                }

                libc::free(temp as *mut c_void);
                return j;
            }

            {
                top = *((pushd_directory_list as usize + ((directory_list_offset - 1) * 8) as usize)
                    as *mut *mut libc::c_char);
                j = directory_list_offset - 2;

                while j > -1 {
                    *((pushd_directory_list as usize + ((j + 1) * 8) as usize)
                        as *mut *mut libc::c_char) = *((pushd_directory_list as usize
                        + (j * 8) as usize)
                        as *mut *mut libc::c_char);
                    j -= 1;
                }

                *((pushd_directory_list as usize + ((j + 1) * 8) as usize)
                    as *mut *mut libc::c_char) = temp;

                temp = top;
                num -= 1;
            }

            while num != 0 {
                top = *((pushd_directory_list as usize + ((directory_list_offset - 1) * 8) as usize)
                    as *mut *mut libc::c_char);
                j = directory_list_offset - 2;

                while j > -1 {
                    *((pushd_directory_list as usize + ((j + 1) * 8) as usize)
                        as *mut *mut libc::c_char) = *((pushd_directory_list as usize
                        + (j * 8) as usize)
                        as *mut *mut libc::c_char);
                    j -= 1;
                }

                *((pushd_directory_list as usize + ((j + 1) * 8) as usize)
                    as *mut *mut libc::c_char) = temp;

                temp = top;
                num -= 1;
            }

            if (flags & NOCD!()) == 0 {
                j = change_to_temp(temp);
            } else {
                j = EXECUTION_SUCCESS!();
            }

            libc::free(temp as *mut c_void);
            return j;
        }

        if list == std::ptr::null_mut() {
            return EXECUTION_SUCCESS!();
        }

        /* Change to the directory in list->word->word.  Save the current
        directory on the top of the stack. */
        current_directory =
            get_working_directory(CString::new("pushd").unwrap().as_ptr() as *mut libc::c_char);
        if current_directory == std::ptr::null_mut() {
            return EXECUTION_FAILURE!();
        }

        if (flags & NOCD!()) == 0 {
            if skipopt != 0 {
                j = cd_builtin(orig_list);
            } else {
                j = cd_builtin(list);
            }
        } else {
            j = EXECUTION_SUCCESS!();
        }

        if j == EXECUTION_SUCCESS!() {
            if (flags & NOCD!()) != 0 {
                add_dirstack_element(savestring!((*((*list).word)).word));
            } else {
                add_dirstack_element(current_directory);
            }

            dirs_builtin(std::ptr::null_mut());
            if (flags & NOCD!()) != 0 {
                libc::free(current_directory as *mut c_void);
            }
            return EXECUTION_SUCCESS!();
        } else {
            libc::free(current_directory as *mut c_void);
            return EXECUTION_FAILURE!();
        }
    }
}

/* Pop the directory stack, and then change to the new top of the stack.
If LIST is non-null it should consist of a word +N or -N, which says
what element to delete from the stack.  The default is the top one. */
#[no_mangle]
pub fn popd_builtin(listt: *mut WordList) -> i32 {
    let mut i: i32;
    let mut which: libc::c_long;
    let mut flags: i32;
    let mut direction: libc::c_char;
    let mut which_word: *mut libc::c_char;

    unsafe {
        let mut list: *mut WordList = listt.clone();
        if list != std::ptr::null_mut()
            && (*list).word != std::ptr::null_mut()
            && ISHELP((*((*list).word)).word)
        {
            builtin_help();
            return EX_USAGE;
        }

        which_word = std::ptr::null_mut();
        flags = 0;
        which = 0;
        direction = '+' as libc::c_char;
        while list != std::ptr::null_mut() {
            if ISOPTION((*((*list).word)).word, 'n' as libc::c_char) {
                flags |= NOCD!();
            } else if ISOPTION((*((*list).word)).word, '-' as libc::c_char) {
                list = (*list).next;
                break;
            } else {
                direction = *((*((*list).word)).word);
                if direction == '+' as libc::c_char || direction == '-' as libc::c_char {
                    if legal_number(
                        ((*((*list).word)).word as usize + 1) as *mut libc::c_char,
                        &mut which,
                    ) == 0
                    {
                        sh_invalidnum((*((*list).word)).word);
                        builtin_usage();
                        return EX_USAGE;
                    }
                    which_word = (*((*list).word)).word;
                } else if *((*((*list).word)).word) == '-' as libc::c_char {
                    sh_invalidopt((*((*list).word)).word);
                    builtin_usage();
                    return EX_USAGE;
                } else if (*((*list).word)).word != std::ptr::null_mut() {
                    builtin_error(
                        CString::new("%s: invalid argument").unwrap().as_ptr() as *mut libc::c_char,
                        (*((*list).word)).word,
                    );
                    builtin_usage();
                    return EX_USAGE;
                } else {
                    break;
                }
            }
            list = (*list).next;
        }

        if which > directory_list_offset as libc::c_long
            || (which < -directory_list_offset as libc::c_long)
            || (directory_list_offset == 0 && which == 0)
        {
            if which_word != std::ptr::null_mut() {
                pushd_error(directory_list_offset, which_word);
            } else {
                pushd_error(
                    directory_list_offset,
                    CString::new("").unwrap().as_ptr() as *mut libc::c_char,
                );
            }
            return EXECUTION_FAILURE!();
        }

        /* Handle case of no specification, or top of stack specification. */
        if (direction == '+' as libc::c_char && which == 0)
            || (direction == '-' as libc::c_char && which == directory_list_offset as libc::c_long)
        {
            if (flags & NOCD!()) == 0 {
                i = cd_to_string(
                    *((pushd_directory_list as usize + ((directory_list_offset - 1) * 8) as usize)
                        as *mut *mut libc::c_char),
                );
            } else {
                i = EXECUTION_SUCCESS!();
            }

            if i != EXECUTION_SUCCESS!() {
                return i;
            }

            directory_list_offset -= 1;

            libc::free(
                (*((pushd_directory_list as usize + (directory_list_offset * 8) as usize)
                    as *mut *mut libc::c_char)) as *mut c_void,
            );
        } else {
            /* Since an offset other than the top directory was specified,
            remove that directory from the list and shift the remainder
            of the list into place. */
            if direction == '+' as libc::c_char {
                i = directory_list_offset - which as i32;
            } else {
                i = which as i32;
            }

            if i < 0 || i > directory_list_offset {
                if which_word != std::ptr::null_mut() {
                    pushd_error(directory_list_offset, which_word);
                } else {
                    pushd_error(
                        directory_list_offset,
                        CString::new("").unwrap().as_ptr() as *mut libc::c_char,
                    );
                }

                return EXECUTION_FAILURE!();
            }
            libc::free(
                (*((pushd_directory_list as usize + (i * 8) as usize) as *mut *mut libc::c_char))
                    as *mut c_void,
            );
            directory_list_offset -= 1;

            /* Shift the remainder of the list into place. */
            while i < directory_list_offset {
                *((pushd_directory_list as usize + (i * 8) as usize) as *mut *mut libc::c_char) =
                    *((pushd_directory_list as usize + ((i + 1) * 8) as usize)
                        as *mut *mut libc::c_char);
                i += 1;
            }
        }

        dirs_builtin(std::ptr::null_mut());
        return EXECUTION_SUCCESS!();
    }
}

/* Print the current list of directories on the directory stack. */
#[no_mangle]
pub fn dirs_builtin(listt: *mut WordList) -> i32 {
    let mut flags: i32 = 0;
    let mut desired_index: i32 = -1;
    let mut index_flag: i32 = 0;
    let mut vflag: i32 = 0;
    let mut i: libc::c_long = 0;
    let mut temp: *mut libc::c_char;
    let mut w: *mut libc::c_char = CString::new("").unwrap().as_ptr() as *mut libc::c_char;

    unsafe {
        let mut list: *mut WordList = listt.clone();
        if list != std::ptr::null_mut()
            && (*list).word != std::ptr::null_mut()
            && ISHELP((*((*list).word)).word)
        {
            builtin_help();
            return EX_USAGE;
        }

        while list != std::ptr::null_mut() {
            if ISOPTION((*((*list).word)).word, 'l' as libc::c_char) {
                flags |= LONGFORM!();
            } else if ISOPTION((*((*list).word)).word, 'c' as libc::c_char) {
                flags |= CLEARSTAK!();
            } else if ISOPTION((*((*list).word)).word, 'v' as libc::c_char) {
                vflag |= 2;
            } else if ISOPTION((*((*list).word)).word, 'p' as libc::c_char) {
                vflag |= 1;
            } else if ISOPTION((*((*list).word)).word, '-' as libc::c_char) {
                list = (*list).next;
                break;
            } else if *((*((*list).word)).word) == '+' as libc::c_char
                || *((*((*list).word)).word) == '-' as libc::c_char
            {
                let sign: i32;
                w = (*(*list).word).word.offset(1);
                if legal_number(w, &mut i) == 0 {
                    sh_invalidnum((*((*list).word)).word);
                    builtin_usage();
                    return EX_USAGE;
                }

                if *((*(*list).word).word) == '+' as libc::c_char {
                    sign = 1;
                } else {
                    sign = -1;
                }

                desired_index = get_dirstack_index(i, sign, &mut index_flag);
            } else {
                sh_invalidopt((*((*list).word)).word);
                builtin_usage();
                return EX_USAGE;
            }
            list = (*list).next
        }

        if (flags & CLEARSTAK!()) != 0 {
            clear_directory_stack();
            return EXECUTION_SUCCESS!();
        }

        if index_flag != 0 && (desired_index < 0 || desired_index > directory_list_offset) {
            pushd_error(directory_list_offset, w);
            return EXECUTION_FAILURE!();
        }

        /* The first directory printed is always the current working directory. */
        if index_flag == 0 || (index_flag == 1 && desired_index == 0) {
            temp =
                get_working_directory(CString::new("dirs").unwrap().as_ptr() as *mut libc::c_char);
            if temp == std::ptr::null_mut() {
                temp =
                    savestring!(CString::new("<no current directory>").unwrap().as_ptr()
                        as *mut libc::c_char);
            }

            if (vflag & 2) != 0 {
                if (flags & LONGFORM!()) != 0 {
                    libc::printf(CString::new("%2d  %s").unwrap().as_ptr(), 0, temp);
                } else {
                    libc::printf(
                        CString::new("%2d  %s").unwrap().as_ptr(),
                        0,
                        polite_directory_format(temp),
                    );
                }
            } else {
                if (flags & LONGFORM!()) != 0 {
                    libc::printf(CString::new("%s").unwrap().as_ptr(), temp);
                } else {
                    libc::printf(
                        CString::new("%s").unwrap().as_ptr(),
                        polite_directory_format(temp),
                    );
                }
            }

            libc::free(temp as *mut c_void);
            if index_flag != 0 {
                libc::putchar('\n' as libc::c_int);
                return sh_chkwrite(EXECUTION_SUCCESS!());
            }
        }

        /* Now print the requested directory stack entries. */
        if index_flag != 0 {
            if (vflag & 2) != 0 {
                if (flags & LONGFORM!()) != 0 {
                    libc::printf(
                        CString::new("%2d  %s").unwrap().as_ptr(),
                        directory_list_offset - desired_index,
                        *((pushd_directory_list as usize + (desired_index * 8) as usize)
                            as *mut *mut libc::c_char),
                    );
                } else {
                    libc::printf(
                        CString::new("%2d  %s").unwrap().as_ptr(),
                        directory_list_offset - desired_index,
                        polite_directory_format(
                            *((pushd_directory_list as usize + (desired_index * 8) as usize)
                                as *mut *mut libc::c_char),
                        ),
                    );
                }
            } else {
                if (flags & LONGFORM!()) != 0 {
                    libc::printf(
                        CString::new("%s").unwrap().as_ptr(),
                        *((pushd_directory_list as usize + (desired_index * 8) as usize)
                            as *mut *mut libc::c_char),
                    );
                } else {
                    libc::printf(
                        CString::new("%s").unwrap().as_ptr(),
                        polite_directory_format(
                            *((pushd_directory_list as usize + (desired_index * 8) as usize)
                                as *mut *mut libc::c_char),
                        ),
                    );
                }
            }
        } else {
            i = (directory_list_offset - 1) as libc::c_long;
            while i >= 0 {
                if vflag >= 2 {
                    if (flags & LONGFORM!()) != 0 {
                        libc::printf(
                            CString::new("\n%2d  %s").unwrap().as_ptr(),
                            directory_list_offset - i as i32,
                            *((pushd_directory_list as usize + (i * 8) as usize)
                                as *mut *mut libc::c_char),
                        );
                    } else {
                        libc::printf(
                            CString::new("\n%2d  %s").unwrap().as_ptr(),
                            directory_list_offset - i as i32,
                            polite_directory_format(
                                *((pushd_directory_list as usize + (i * 8) as usize)
                                    as *mut *mut libc::c_char),
                            ),
                        );
                    }
                } else {
                    if (flags & LONGFORM!()) != 0 {
                        if (vflag & 1) != 0 {
                            libc::printf(
                                CString::new("%s%s").unwrap().as_ptr(),
                                CString::new("\n").unwrap().as_ptr() as *mut libc::c_char,
                                *((pushd_directory_list as usize + (i * 8) as usize)
                                    as *mut *mut libc::c_char),
                            );
                        } else {
                            libc::printf(
                                CString::new("%s%s").unwrap().as_ptr(),
                                CString::new(" ").unwrap().as_ptr() as *mut libc::c_char,
                                *((pushd_directory_list as usize + (i * 8) as usize)
                                    as *mut *mut libc::c_char),
                            );
                        }
                    } else {
                        if (vflag & 1) != 0 {
                            libc::printf(
                                CString::new("%s%s").unwrap().as_ptr(),
                                CString::new("\n").unwrap().as_ptr() as *mut libc::c_char,
                                polite_directory_format(
                                    *((pushd_directory_list as usize + (i * 8) as usize)
                                        as *mut *mut libc::c_char),
                                ),
                            );
                        } else {
                            libc::printf(
                                CString::new("%s%s").unwrap().as_ptr(),
                                CString::new(" ").unwrap().as_ptr() as *mut libc::c_char,
                                polite_directory_format(
                                    *((pushd_directory_list as usize + (i * 8) as usize)
                                        as *mut *mut libc::c_char),
                                ),
                            );
                        }
                    }
                }
                i -= 1;
            }
        }

        libc::putchar('\n' as libc::c_int);
        return sh_chkwrite(EXECUTION_SUCCESS!());
    }
}

#[no_mangle]
pub fn pushd_error(offset: i32, arg: *mut libc::c_char) {
    unsafe {
        if offset == 0 {
            builtin_error(CString::new("directory stack empty").unwrap().as_ptr());
        } else {
            sh_erange(
                arg,
                CString::new("directory stack index").unwrap().as_ptr() as *mut libc::c_char,
            );
        }
    }
}

#[no_mangle]
pub fn clear_directory_stack() {
    let mut i: i32 = 0;
    unsafe {
        while i < directory_list_offset {
            libc::free(
                *((pushd_directory_list as usize + (i * 8) as usize) as *mut *mut libc::c_char)
                    as *mut c_void,
            );
            i += 1;
        }

        directory_list_offset = 0;
    }
}

/* Switch to the directory in NAME.  This uses the cd_builtin to do the work,
so if the result is EXECUTION_FAILURE then an error message has already
been printed. */
#[no_mangle]
pub fn cd_to_string(name: *mut libc::c_char) -> i32 {
    unsafe {
        let tlist: *mut WordList;
        let dir: *mut WordList;
        let result: i32;

        dir = make_word_list(make_word(name), std::ptr::null_mut());
        tlist = make_word_list(make_word(CString::new("--").unwrap().as_ptr()), dir);
        result = cd_builtin(tlist);
        dispose_words(tlist);
        return result;
    }
}

#[no_mangle]
pub fn change_to_temp(temp: *mut libc::c_char) -> i32 {
    let tt: i32;

    if temp != std::ptr::null_mut() {
        tt = cd_to_string(temp);
    } else {
        tt = EXECUTION_FAILURE!();
    }

    if tt == EXECUTION_SUCCESS!() {
        dirs_builtin(std::ptr::null_mut());
    }

    return tt;
}

#[no_mangle]
pub fn add_dirstack_element(dir: *mut libc::c_char) {
    unsafe {
        if directory_list_offset == directory_list_size {
            directory_list_size += 10;
            pushd_directory_list = strvec_resize(pushd_directory_list, directory_list_size);
        }

        *((pushd_directory_list as usize + (directory_list_offset * 8) as usize)
            as *mut *mut libc::c_char) = dir;
        directory_list_offset += 1;
    }
}

#[no_mangle]
pub fn get_dirstack_index(ind: libc::c_long, sign: i32, indexp: *mut i32) -> i32 {
    unsafe {
        if indexp != std::ptr::null_mut() {
            if sign > 0 {
                *indexp = 1;
            } else {
                *indexp = 2;
            }
        }
        /* dirs +0 prints the current working directory. */
        /* dirs -0 prints last element in directory stack */
        if ind == 0 && sign > 0 {
            return 0;
        } else if ind == directory_list_offset as libc::c_long {
            if indexp != std::ptr::null_mut() {
                if sign > 0 {
                    *indexp = 2;
                } else {
                    *indexp = 1;
                }
            }
            return 0;
        } else if ind >= 0 && ind <= directory_list_offset as libc::c_long {
            if sign > 0 {
                return directory_list_offset - ind as i32;
            } else {
                return ind as i32;
            }
        } else {
            return -1;
        }
    }
}

/* Used by the tilde expansion code. */
#[no_mangle]
pub fn get_dirstack_from_string(strt: *mut libc::c_char) -> *mut libc::c_char {
    let ind: i32;
    let mut sign: i32;
    let mut index_flag: i32;
    let mut i: libc::c_long = 0;

    sign = 1;
    let mut str1 = strt.clone();
    unsafe {
        if *str1 == '-' as libc::c_char || *str1 == '+' as libc::c_char {
            if *str1 == '-' as libc::c_char {
                sign = -1;
            } else {
                sign = 1;
            }
            str1 = (str1 as usize + 1) as *mut libc::c_char;
        }

        if legal_number(str1, &mut i) == 0 {
            return std::ptr::null_mut();
        }

        index_flag = 0;
        ind = get_dirstack_index(i, sign, &mut index_flag);
        if index_flag != 0 && (ind < 0 || ind > directory_list_offset) {
            return std::ptr::null_mut();
        }

        if index_flag == 0 || (index_flag == 1 && ind == 0) {
            return get_string_value(CString::new("PWD").unwrap().as_ptr());
        } else {
            return *((pushd_directory_list as usize + (ind * 8) as usize)
                as *mut *mut libc::c_char);
        }
    }
}

#[no_mangle]
pub fn get_dirstack_element(ind: libc::c_long, sign: i32) -> *mut libc::c_char {
    let i: i32;
    unsafe {
        i = get_dirstack_index(ind, sign, std::ptr::null_mut());
        if i < 0 || i > directory_list_offset {
            return std::ptr::null_mut();
        } else {
            return *((pushd_directory_list as usize + (i * 8) as usize) as *mut *mut libc::c_char);
        }
    }
}

#[no_mangle]
pub fn set_dirstack_element(ind: libc::c_long, sign: i32, value: *mut libc::c_char) {
    let i: i32;
    unsafe {
        i = get_dirstack_index(ind, sign, std::ptr::null_mut());
        if ind == 0 || i < 0 || i > directory_list_offset {
            return;
        }
        libc::free(
            (*((pushd_directory_list as usize + (i * 8) as usize) as *mut *mut libc::c_char))
                as *mut c_void,
        );
        *((pushd_directory_list as usize + (i * 8) as usize) as *mut *mut libc::c_char) =
            savestring!(value);
    }
}

#[no_mangle]
pub fn get_directory_stack(flags: i32) -> *mut WordList {
    let mut i: i32;
    let mut ret: *mut WordList;
    let mut d: *mut libc::c_char;
    let t: *mut libc::c_char;
    unsafe {
        ret = std::ptr::null_mut();
        i = 0;
        while i < directory_list_offset {
            if (flags & 1) != 0 {
                d = polite_directory_format(
                    *((pushd_directory_list as usize + (i * 8) as usize) as *mut *mut libc::c_char),
                );
            } else {
                d = *((pushd_directory_list as usize + (i * 8) as usize) as *mut *mut libc::c_char)
            }
            ret = make_word_list(make_word(d), ret);
            i += 1;
        }
        /* Now the current directory. */
        d = get_working_directory(CString::new("dirstack").unwrap().as_ptr() as *mut libc::c_char);
        i = 0; /* sentinel to decide whether or not to free d */
        if d == std::ptr::null_mut() {
            d = CString::new(".").unwrap().as_ptr() as *mut libc::c_char;
        } else {
            if (flags & 1) != 0 {
                t = polite_directory_format(d);
            } else {
                t = d;
            }
            /* polite_directory_format sometimes returns its argument unchanged.
            If it does not, we can free d right away.  If it does, we need to
            mark d to be deleted later. */
            if t != d {
                libc::free(d as *mut c_void);
                d = t;
            } else {
                /* t == d, so d is what we want */
                i = 1;
            }
        }
        ret = make_word_list(make_word(d), ret);
        if i != 0 {
            libc::free(d as *mut c_void);
        }
        return ret; /* was (REVERSE_LIST (ret, (WordList *)); */
    }
}
