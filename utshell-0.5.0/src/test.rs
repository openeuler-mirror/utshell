use crate::arrayfunc::{array_value, valid_array_reference};
use crate::assoc::assoc_reference;
use crate::builtins::common::number_of_args;
use crate::builtins::set::minus_o_option_value;
use crate::expr::evalexp;
use crate::general::{legal_number, same_file};
use crate::readline::{ array_reference};
use crate::src_common::*;
use crate::variables::{find_variable, find_variable_noref};
use crate::version::shell_compatibility_level;

pub const GT: libc::c_int = 3;
pub const EQ: libc::c_int = 0;
pub const LT: libc::c_int = 2;

#[inline]
fn get_stat_atime(mut st: *const crate::src_common::stat) -> crate::src_common::timespec {
    unsafe {
        return (*st).st_atim;
    }
}

#[inline]
fn get_stat_mtime(mut st: *const crate::src_common::stat) -> crate::src_common::timespec {
    unsafe {
        return (*st).st_mtim;
    }
}

#[inline]
fn timespec_cmp(
    mut a: crate::src_common::timespec,
    mut b: crate::src_common::timespec,
) -> libc::c_int {
    return if a.tv_sec < b.tv_sec {
        -(1 as libc::c_int)
    } else if a.tv_sec > b.tv_sec {
        1 as libc::c_int
    } else {
        (a.tv_nsec - b.tv_nsec) as libc::c_int
    };
}


static mut test_error_return: libc::c_int = 0;
static mut pos: libc::c_int = 0;
static mut test_exit_buf: sigjmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
static mut argc: libc::c_int = 0;
static mut argv: *mut *mut libc::c_char = 0 as *const *mut libc::c_char as *mut *mut libc::c_char;
static mut noeval: libc::c_int = 0;
pub type __jmp_buf = [libc::c_long; 8];

#[no_mangle]
pub fn patcomp(
    mut string: *mut libc::c_char,
    mut pat: *mut libc::c_char,
    mut op: libc::c_int,
) -> libc::c_int {
    let mut m: libc::c_int = 0;
    unsafe {
        m = strmatch(
            pat,
            string,
            FNMATCH_EXTFLAG!() | FNMATCH_IGNCASE!()
        );
        return if op == EQ as libc::c_int {
            (m == 0) as libc::c_int
        } else {
            (m != 0) as i32
        };
    }
}

#[no_mangle]
fn integer_expected_error(mut pch: *mut libc::c_char) {
    unsafe {
        printf(
            b"%s: integer expression expected\n\0" as *const u8 as *const libc::c_char,
            pch,
        );
    }
}

#[no_mangle]
fn beyond() {
    unsafe {
        printf(b"argument expected\n\0" as *const u8 as *const libc::c_char);
    }
}

#[no_mangle]
fn unary_operator() -> libc::c_int {
    unsafe {
        let mut op: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut r: intmax_t = 0;
        op = *argv.offset(pos as isize);
        if test_unop(op) == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        if *op.offset(1 as libc::c_int as isize) as libc::c_int == 't' as i32 {
            pos += 1;
            if 0 as libc::c_int != 0 && pos >= argc {
                beyond();
            }
            if pos < argc {
                if legal_number(*argv.offset(pos as isize), &mut r) != 0 {
                    pos += 1;
                    if 0 as libc::c_int != 0 && pos >= argc {
                        beyond();
                    }
                    return unary_test(op, *argv.offset((pos - 1 as libc::c_int) as isize));
                } else {
                    return 0 as libc::c_int;
                }
            } else {
                return unary_test(
                    op,
                    b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
        }
        pos += 1;
        if 1 as libc::c_int != 0 && pos >= argc {
            beyond();
        }
        pos += 1;
        return unary_test(op, *argv.offset((pos - 1 as libc::c_int) as isize));
    }
}

#[no_mangle]
pub fn test_unop(mut op: *mut libc::c_char) -> libc::c_int {
    unsafe {
        if *op.offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32
            || (*op.offset(1 as libc::c_int as isize) as libc::c_int != 0
                && *op.offset(2 as libc::c_int as isize) as libc::c_int != 0 as libc::c_int)
        {
            return 0 as libc::c_int;
        }
        match *op.offset(1 as libc::c_int as isize) as u8 as char {
            'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'k' | 'n' | 'o' | 'p' | 'r' | 's'
            | 't' | 'u' | 'v' | 'w' | 'x' | 'z' | 'G' | 'L' | 'O' | 'S' | 'N' | 'R' => {
                return 1;
            }
            _ => {}
        }
        return 0;
    }
}

#[no_mangle]
fn three_arguments() -> libc::c_int {
    let mut value: libc::c_int = 0;
    unsafe {
        if test_binop(*argv.offset((pos + 1 as libc::c_int) as isize)) != 0 {
            value = binary_operator();
            pos = argc;
        } else if *(*argv.offset((pos + 1 as libc::c_int) as isize))
            .offset(0 as libc::c_int as isize) as libc::c_int
            == '-' as i32
            && (*(*argv.offset((pos + 1 as libc::c_int) as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int
                == 'a' as i32
                || *(*argv.offset((pos + 1 as libc::c_int) as isize))
                    .offset(1 as libc::c_int as isize) as libc::c_int
                    == 'o' as i32)
            && *(*argv.offset((pos + 1 as libc::c_int) as isize)).offset(2 as libc::c_int as isize)
                as libc::c_int
                == 0 as libc::c_int
        {
            if *(*argv.offset((pos + 1 as libc::c_int) as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int
                == 'a' as i32
            {
                value = (*(*argv.offset(pos as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int
                    != '\u{0}' as i32
                    && *(*argv.offset((pos + 2 as libc::c_int) as isize))
                        .offset(0 as libc::c_int as isize) as libc::c_int
                        != '\u{0}' as i32) as libc::c_int;
            } else {
                value = (*(*argv.offset(pos as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int
                    != '\u{0}' as i32
                    || *(*argv.offset((pos + 2 as libc::c_int) as isize))
                        .offset(0 as libc::c_int as isize) as libc::c_int
                        != '\u{0}' as i32) as libc::c_int;
            }
            pos = argc;
        } else if *(*argv.offset(pos as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            == '!' as i32
            && *(*argv.offset(pos as isize)).offset(1 as libc::c_int as isize) as libc::c_int
                == '\u{0}' as i32
        {
            pos += 1;
            if 1 as libc::c_int != 0 && pos >= argc {
                beyond();
            }
            //value = !two_arguments();
            value = (two_arguments() == 0) as libc::c_int;
        } else if *(*argv.offset(pos as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            == '(' as i32
            && *(*argv.offset((pos + 2 as libc::c_int) as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int
                == ')' as i32
        {
            value = (*(*argv.offset((pos + 1 as libc::c_int) as isize))
                .offset(0 as libc::c_int as isize) as libc::c_int
                != '\u{0}' as i32) as libc::c_int;
            pos = argc;
        } else {
            printf(
                b"%s: binary operator expected\n\0" as *const u8 as *const libc::c_char,
                (*argv).offset((pos + 1) as isize),
            );
        }
        return value;
    }
}

#[no_mangle]
fn two_arguments() -> libc::c_int {
    unsafe {
        if *(*argv.offset(pos as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            == '!' as i32
            && *(*argv.offset(pos as isize)).offset(1 as libc::c_int as isize) as libc::c_int
                == '\u{0}' as i32
        {
            return (*(*argv.offset((pos + 1 as libc::c_int) as isize))
                .offset(0 as libc::c_int as isize) as libc::c_int
                == '\u{0}' as i32) as libc::c_int;
        } else if *(*argv.offset(pos as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            == '-' as i32
            && *(*argv.offset(pos as isize)).offset(1 as libc::c_int as isize) as libc::c_int != 0
            && *(*argv.offset(pos as isize)).offset(2 as libc::c_int as isize) as libc::c_int
                == '\u{0}' as i32
        {
            if test_unop(*argv.offset(pos as isize)) != 0 {
                return unary_operator();
            } else {
                printf(
                    b"%s: unary operator expected\n\0" as *const u8 as *const libc::c_char,
                    (*argv).offset(pos as isize),
                );
            }
        } else {
            printf(
                b"%s: unary operator expected\n\0" as *const u8 as *const libc::c_char,
                (*argv).offset(pos as isize),
            );
        };
        return 0;
    }
}

#[no_mangle]
fn expr() -> libc::c_int {
    unsafe {
        if pos >= argc {
            beyond();
        }
        return 0 as libc::c_int ^ or();
    }
}

#[no_mangle]
fn or() -> libc::c_int {
    let mut value: libc::c_int = 0;
    let mut v2: libc::c_int = 0;
    value = and();
    unsafe {
        if pos < argc
            && *(*argv.offset(pos as isize)).offset(0 as libc::c_int as isize) as libc::c_int
                == '-' as i32
            && *(*argv.offset(pos as isize)).offset(1 as libc::c_int as isize) as libc::c_int
                == 'o' as i32
            && *(*argv.offset(pos as isize)).offset(2 as libc::c_int as isize) == 0
        {
            pos += 1;
            if 0 as libc::c_int != 0 && pos >= argc {
                beyond();
            }
            v2 = or();
            return (value != 0 || v2 != 0) as libc::c_int;
        }
        return value;
    }
}

#[no_mangle]
fn and() -> libc::c_int {
    unsafe {
        let mut value: libc::c_int = 0;
        let mut v2: libc::c_int = 0;
        value = term();
        if pos < argc
            && *(*argv.offset(pos as isize)).offset(0 as libc::c_int as isize) as libc::c_int
                == '-' as i32
            && *(*argv.offset(pos as isize)).offset(1 as libc::c_int as isize) as libc::c_int
                == 'a' as i32
            && *(*argv.offset(pos as isize)).offset(2 as libc::c_int as isize) == 0
        {
            pos += 1;
            if 0 as libc::c_int != 0 && pos >= argc {
                beyond();
            }
            v2 = and();
            return (value != 0 && v2 != 0) as libc::c_int;
        }
        return value;
    }
}

#[no_mangle]
fn term() -> libc::c_int {
    unsafe {
        let mut value: libc::c_int = 0;
        if pos >= argc {
            beyond();
        }
        if *(*argv.offset(pos as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            == '!' as i32
            && *(*argv.offset(pos as isize)).offset(1 as libc::c_int as isize) as libc::c_int
                == '\u{0}' as i32
        {
            value = 0 as libc::c_int;
            while pos < argc
                && *(*argv.offset(pos as isize)).offset(0 as libc::c_int as isize) as libc::c_int
                    == '!' as i32
                && *(*argv.offset(pos as isize)).offset(1 as libc::c_int as isize) as libc::c_int
                    == '\u{0}' as i32
            {
                pos += 1;
                if 1 as libc::c_int != 0 && pos >= argc {
                    beyond();
                }
                value = 1 as libc::c_int - value;
            }
            return if value != 0 {
                (term() == 0) as libc::c_int
            } else {
                term()
            };
        }
        if *(*argv.offset(pos as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            == '(' as i32
            && *(*argv.offset(pos as isize)).offset(1 as libc::c_int as isize) as libc::c_int
                == '\u{0}' as i32
        {
            pos += 1;
            if 1 as libc::c_int != 0 && pos >= argc {
                beyond();
            }
            value = expr();
            if (*argv.offset(pos as isize)).is_null() {
                printf(b"`)' expected\n\0" as *const u8 as *const libc::c_char);
            } else if *(*argv.offset(pos as isize)).offset(0 as libc::c_int as isize) as libc::c_int
                != ')' as i32
                || *(*argv.offset(pos as isize)).offset(1 as libc::c_int as isize) as libc::c_int
                    != 0
            {
                printf(
                    b"`)' expected, found %s\n\0" as *const u8 as *const libc::c_char,
                    (*argv).offset(pos as isize),
                );
            } else {
                pos += 1;
                if 0 as libc::c_int != 0 && pos >= argc {
                    beyond();
                }
            }
            return value;
        }

        if pos + 3 as libc::c_int <= argc
            && test_binop(*argv.offset((pos + 1 as libc::c_int) as isize)) != 0
        {
            value = binary_operator();
        } else if pos + 2 as libc::c_int <= argc && test_unop(*argv.offset(pos as isize)) != 0 {
            value = unary_operator();
        } else {
            value = (*(*argv.offset(pos as isize)).offset(0 as libc::c_int as isize) as libc::c_int
                != '\u{0}' as i32) as libc::c_int;
            pos += 1;
            if 0 as libc::c_int != 0 && pos >= argc {
                beyond();
            }
        }
        return value;
    }
}

#[no_mangle]
fn posixtest() -> libc::c_int {
    unsafe {
        let mut value: libc::c_int = 0;
        match argc - 1 as libc::c_int {
            0 => {
                value = 0;
                pos = argc;
            }
            1 => {
                value = (*(*argv.offset(1 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_int
                    != '\u{0}' as i32) as libc::c_int;
                pos = argc;
            }
            2 => {
                value = two_arguments();
                pos = argc;
            }
            3 => {
                value = three_arguments();
            }
            4 => {
                if *(*argv.offset(pos as isize)).offset(0 as libc::c_int as isize) as libc::c_int
                    == '!' as i32
                    && *(*argv.offset(pos as isize)).offset(1 as libc::c_int as isize)
                        as libc::c_int
                        == '\u{0}' as i32
                {
                    pos += 1;
                    if 1 as libc::c_int != 0 && pos >= argc {
                        beyond();
                    }
                    //value = !three_arguments() as libc::c_int;
                    value = (three_arguments() == 0) as libc::c_int;
                } else if *(*argv.offset(pos as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int
                    == '(' as i32
                    && *(*argv.offset(pos as isize)).offset(1 as libc::c_int as isize)
                        as libc::c_int
                        == '\u{0}' as i32
                    && *(*argv.offset((argc - 1 as libc::c_int) as isize))
                        .offset(0 as libc::c_int as isize) as libc::c_int
                        == ')' as i32
                    && *(*argv.offset((argc - 1 as libc::c_int) as isize))
                        .offset(1 as libc::c_int as isize) as libc::c_int
                        == '\u{0}' as i32
                {
                    pos += 1;
                    if 1 as libc::c_int != 0 && pos >= argc {
                        beyond();
                    }
                    value = two_arguments();
                    pos = argc;
                }
            }
            _ => {
                value = expr();
            }
        }
        return value;
    }
}

#[no_mangle]
fn binary_operator() -> libc::c_int {
    unsafe {
        let mut value: libc::c_int = 0;
        let mut w: *mut libc::c_char = 0 as *mut libc::c_char;
        w = *argv.offset((pos + 1 as libc::c_int) as isize);
        if *w.offset(0 as libc::c_int as isize) as libc::c_int == '=' as i32
            && (*w.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
                || *w.offset(1 as libc::c_int as isize) as libc::c_int == '=' as i32
                    && *w.offset(2 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32)
            || (*w.offset(0 as libc::c_int as isize) as libc::c_int == '>' as i32
                || *w.offset(0 as libc::c_int as isize) as libc::c_int == '<' as i32)
                && *w.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
            || *w.offset(0 as libc::c_int as isize) as libc::c_int == '!' as i32
                && *w.offset(1 as libc::c_int as isize) as libc::c_int == '=' as i32
                && *w.offset(2 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
        {
            value = binary_test(
                w,
                *argv.offset(pos as isize),
                *argv.offset((pos + 2 as libc::c_int) as isize),
                0 as libc::c_int,
            );
            pos += 3 as libc::c_int;
            return value;
        }

        if *w.offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32
            || *w.offset(3 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
            || test_binop(w) == 0 as libc::c_int
        {
            printf(
                b"%s: binary operator expected\n\0" as *const u8 as *const libc::c_char,
                w,
            );
            return 0;
        }
        value = binary_test(
            w,
            *argv.offset(pos as isize),
            *argv.offset((pos + 2 as libc::c_int) as isize),
            0 as libc::c_int,
        );
        pos += 3 as libc::c_int;
        return value;
    }
}

#[no_mangle]
pub fn test_binop(mut op: *mut libc::c_char) -> libc::c_int {
    unsafe {
        if *op.offset(0 as libc::c_int as isize) as libc::c_int == '=' as i32
            && *op.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
        {
            return 1 as libc::c_int;
        } else if (*op.offset(0 as libc::c_int as isize) as libc::c_int == '<' as i32
            || *op.offset(0 as libc::c_int as isize) as libc::c_int == '>' as i32)
            && *op.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
        {
            return 1 as libc::c_int;
        } else if (*op.offset(0 as libc::c_int as isize) as libc::c_int == '=' as i32
            || *op.offset(0 as libc::c_int as isize) as libc::c_int == '!' as i32)
            && *op.offset(1 as libc::c_int as isize) as libc::c_int == '=' as i32
            && *op.offset(2 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
        {
            return 1 as libc::c_int;
        } else if *op.offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32
            || *op.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
            || *op.offset(2 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
            || *op.offset(3 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
        {
            return 0 as libc::c_int;
        } else {
            if *op.offset(2 as libc::c_int as isize) as libc::c_int == 't' as i32 {
                match *op.offset(1 as libc::c_int as isize) as u8 as char {
                    'n' | 'o' | 'l' | 'g' => {
                        return 1;
                    }
                    _ => {
                        return 0;
                    }
                }
            } else if *op.offset(1 as libc::c_int as isize) as libc::c_int == 'e' as i32 {
                match *op.offset(2 as libc::c_int as isize) as u8 as char {
                    'q' | 'f' => {
                        return 1;
                    }
                    _ => {
                        return 0;
                    }
                }
            } else if *op.offset(2 as libc::c_int as isize) as libc::c_int == 'e' as i32 {
                match *op.offset(1 as libc::c_int as isize) as u8 as char {
                    'n' | 'l' | 'g' => {
                        return 1;
                    }
                    _ => {
                        return 0;
                    }
                }
            } else {
                return 0;
            }
        }
    }
}

#[no_mangle]
pub fn test_command(mut margc: libc::c_int, mut margv: *mut *mut libc::c_char) -> libc::c_int {
    unsafe {
        let mut value: libc::c_int = 0;
        let mut code: libc::c_int = 0;
        code = __sigsetjmp(test_exit_buf.as_mut_ptr(), 0 as libc::c_int);
        if code != 0 {
            return test_error_return;
        }

        argv = margv;
        if !(*margv.offset(0 as libc::c_int as isize)).is_null()
            && *(*margv.offset(0 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int
                == '[' as i32
            && *(*margv.offset(0 as libc::c_int as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int
                == '\u{0}' as i32
        {
            margc -= 1;
            if !(*margv.offset(margc as isize)).is_null()
                && (*(*margv.offset(margc as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int
                    != ']' as i32
                    || *(*margv.offset(margc as isize)).offset(1 as libc::c_int as isize)
                        as libc::c_int
                        != 0)
            {
                printf(b"missing `]'\n\0" as *const u8 as *const libc::c_char);
            }
            if margc < 2 as libc::c_int {
                test_error_return = (0 as libc::c_int == 0) as libc::c_int;
                siglongjmp(test_exit_buf.as_mut_ptr(), 1 as libc::c_int);
            }
        }
        argc = margc;
        pos = 1 as libc::c_int;
        if pos >= argc {
            test_error_return = (0 as libc::c_int == 0) as libc::c_int;
            siglongjmp(test_exit_buf.as_mut_ptr(), 1 as libc::c_int);
        }
        noeval = 0 as libc::c_int;
        value = posixtest();

        if pos != argc {
            if pos < argc
                && *(*argv.offset(pos as isize)).offset(0 as libc::c_int as isize) as libc::c_int
                    == '-' as i32
            {
                printf(
                    b"syntax error: '%s' unexpected\n\0" as *const u8 as *const libc::c_char,
                    (*argv).offset(pos as isize),
                );
            } else {
                printf(b"too many arguments\n\0" as *const u8 as *const libc::c_char);
            }
        }

        test_error_return = (value == 0) as libc::c_int;
        siglongjmp(test_exit_buf.as_mut_ptr(), 1 as libc::c_int);
    }
}

#[no_mangle]
pub fn arithcomp(
    mut s: *mut libc::c_char,
    mut t: *mut libc::c_char,
    op: libc::c_int,
    flags: libc::c_int,
) -> libc::c_int {
    let mut l: intmax_t = 0;
    let mut r: intmax_t = 0;
    let mut expok: libc::c_int = 0;

    if flags as libc::c_int & TEST_ARITHEXP as libc::c_int != 0 {
        l = evalexp(s, EXP_EXPANDED as i32, &mut expok);
        if expok == 0 {
            return 0;
        }
        r = evalexp(t, EXP_EXPANDED as i32, &mut expok);
        if expok == 0 {
            return 0;
        }
    } else {
        if legal_number(s, &mut l) == 0 {
            integer_expected_error(s);
        }
        if legal_number(t, &mut r) == 0 {
            integer_expected_error(t);
        }
    }
    match op {
        EQ => return (l == r) as libc::c_int,
        NE => return (l != r) as libc::c_int,
        LT => return (l < r) as libc::c_int,
        GT => return (l > r) as libc::c_int,
        LE => return (l <= r) as libc::c_int,
        GE => return (l >= r) as libc::c_int,
        _ => {}
    }
    return 0;
}

#[no_mangle]
pub fn stat_mtime(
    mut fn0: *mut libc::c_char,
    mut st: *mut crate::src_common::stat,
    mut ts: *mut crate::src_common::timespec,
) -> libc::c_int {
    unsafe {
        let mut r: libc::c_int = 0;
        r = sh_stat(fn0, st);
        if r < 0 {
            return r;
        }
        *ts = get_stat_mtime(st);
        return 0;
    }
}

#[no_mangle]
pub fn filecomp(
    mut s: *mut libc::c_char,
    mut t: *mut libc::c_char,
    op: libc::c_int,
) -> libc::c_int {
    let mut st1: crate::src_common::stat = crate::src_common::stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: crate::src_common::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: crate::src_common::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: crate::src_common::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut st2: crate::src_common::stat = crate::src_common::stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: crate::src_common::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: crate::src_common::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: crate::src_common::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut ts1: crate::src_common::timespec = crate::src_common::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut ts2: crate::src_common::timespec = crate::src_common::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut r1: libc::c_int = 0;
    let mut r2: libc::c_int = 0;

    r1 = stat_mtime(s, &mut st1, &mut ts1);
    if r1 < 0 {
        if op == EF {
            return 0;
        }
    }

    r2 = stat_mtime(t, &mut st2, &mut ts2);
    if r2 < 0 {
        if op == EF {
            return 0;
        }
    }

    match op {
        OT => {
            return (r1 < r2 || r2 == 0 as libc::c_int && timespec_cmp(ts1, ts2) < 0) as libc::c_int
        }
        NT => {
            return (r1 > r2 || r1 == 0 as libc::c_int && timespec_cmp(ts1, ts2) > 0) as libc::c_int
        }
        EF => return same_file(s, t, &mut st1, &mut st2),
        _ => {}
    }
    return 0;
}

#[no_mangle]
pub fn binary_test(
    mut op: *mut libc::c_char,
    mut arg1: *mut libc::c_char,
    mut arg2: *mut libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    unsafe {
        let mut patmatch: libc::c_int = 0;
        patmatch = flags & 1 as libc::c_int;
        if *op.offset(0 as libc::c_int as isize) as libc::c_int == '=' as i32
            && (*op.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
                || *op.offset(1 as libc::c_int as isize) as libc::c_int == '=' as i32
                    && *op.offset(2 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32)
        {
            return if patmatch != 0 {
                patcomp(arg1, arg2, 0 as libc::c_int)
            } else {
                (*arg1.offset(0 as libc::c_int as isize) as libc::c_int
                    == *arg2.offset(0 as libc::c_int as isize) as libc::c_int
                    && strcmp(arg1, arg2) == 0 as libc::c_int) as libc::c_int
            };
        } else if *op.offset(0 as isize) as libc::c_int == '>' as i32
            || *op.offset(0 as isize) as libc::c_int == '<' as i32
                && *op.offset(1 as isize) as libc::c_int == '\u{0}' as i32
        {
            if shell_compatibility_level > 40 as libc::c_int
                && flags & TEST_LOCALE as libc::c_int != 0
            {
                return if *op.offset(0 as libc::c_int as isize) as libc::c_int == '>' as i32 {
                    (strcoll(arg1, arg2) > 0 as libc::c_int) as libc::c_int
                } else {
                    (strcoll(arg1, arg2) < 0 as libc::c_int) as libc::c_int
                };
            } else {
                return if *op.offset(0 as libc::c_int as isize) as libc::c_int == '>' as i32 {
                    (strcmp(arg1, arg2) > 0 as libc::c_int) as libc::c_int
                } else {
                    (strcmp(arg1, arg2) < 0 as libc::c_int) as libc::c_int
                };
            }
        } else if *op.offset(0 as libc::c_int as isize) as libc::c_int == '!' as i32
            && *op.offset(1 as libc::c_int as isize) as libc::c_int == '=' as i32
            && *op.offset(2 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
        {
            return if patmatch != 0 {
                patcomp(arg1, arg2, 1 as libc::c_int)
            } else {
                ((*arg1.offset(0 as libc::c_int as isize) as libc::c_int
                    == *arg2.offset(0 as libc::c_int as isize) as libc::c_int
                    && strcmp(arg1, arg2) == 0 as libc::c_int) as libc::c_int
                    == 0 as libc::c_int) as libc::c_int
            };
        } else if *op.offset(2 as libc::c_int as isize) as libc::c_int == 't' as i32 {
            match *op.offset(1 as libc::c_int as isize) as u8 as char {
                'n' => {
                    return filecomp(arg1, arg2, NT) as libc::c_int;
                }
                'o' => {
                    return filecomp(arg1, arg2, OT) as libc::c_int;
                }
                'l' => {
                    return arithcomp(arg1, arg2, LT, flags) as libc::c_int;
                }
                'g' => {
                    return arithcomp(arg1, arg2, GT, flags) as libc::c_int;
                }
                _ => {}
            }
        } else if *op.offset(1 as libc::c_int as isize) as libc::c_int == 'e' as i32 {
            match *op.offset(2 as libc::c_int as isize) as u8 as char {
                'f' => {
                    return filecomp(arg1, arg2, EF) as libc::c_int;
                }
                'q' => {
                    return arithcomp(arg1, arg2, EQ, flags) as libc::c_int;
                }
                _ => {}
            }
        } else if *op.offset(2 as libc::c_int as isize) as libc::c_int == 'e' as i32 {
            match *op.offset(1 as libc::c_int as isize) as u8 as char {
                'n' => {
                    return arithcomp(arg1, arg2, NE, flags) as libc::c_int;
                }
                'g' => {
                    return arithcomp(arg1, arg2, GE, flags) as libc::c_int;
                }
                'l' => {
                    return arithcomp(arg1, arg2, LE, flags) as libc::c_int;
                }
                _ => {}
            }
        }
        return 0;
    }
}

//whether diff types of variables from stat at arch arm/x86?
#[no_mangle]
pub fn unary_test(mut op: *mut libc::c_char, mut arg: *mut libc::c_char) -> libc::c_int {
    let mut r: intmax_t = 0;
    let mut stat_buf: crate::src_common::stat = crate::src_common::stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: crate::src_common::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: crate::src_common::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: crate::src_common::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut mtime: crate::src_common::timespec = crate::src_common::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut atime: crate::src_common::timespec = crate::src_common::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    unsafe {
        match *op.offset(1) as u8 as char {
            'a' | 'e' => {
                return (sh_stat(arg, &mut stat_buf) == 0) as libc::c_int;
            }
            'r' => {
                return (sh_eaccess(arg, R_OK) == 0) as libc::c_int;
            }
            'w' => {
                return (sh_eaccess(arg, W_OK) == 0) as libc::c_int;
            }
            'x' => {
                return (sh_eaccess(arg, X_OK) == 0) as libc::c_int;
            }
            'O' => {
                return (sh_stat(arg, &mut stat_buf) == 0 && current_user.euid == stat_buf.st_uid)
                    as libc::c_int;
            }
            'G' => {
                return (sh_stat(arg, &mut stat_buf) == 0 && current_user.egid == stat_buf.st_gid)
                    as libc::c_int;
            }
            'N' => {
                if sh_stat(arg, &mut stat_buf) < 0 {
                    return 0;
                }
                atime = get_stat_atime(&mut stat_buf);
                mtime = get_stat_mtime(&mut stat_buf);
                return (timespec_cmp(mtime, atime) > 0) as libc::c_int;
            }
            'f' => {
                if sh_stat(arg, &mut stat_buf) < 0 {
                    return 0;
                }
                return (stat_buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o100000 as libc::c_int as libc::c_uint
                    || stat_buf.st_mode & 0o170000 as libc::c_int as libc::c_uint == 0)
                    as libc::c_int;
            }
            'd' => {
                return (sh_stat(arg, &mut stat_buf) == 0
                    && stat_buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint)
                    as libc::c_int;
            }
            's' => {
                return (sh_stat(arg, &mut stat_buf) == 0 && stat_buf.st_size > 0 as off_t)
                    as libc::c_int;
            }
            'S' => {
                return (sh_stat(arg, &mut stat_buf) == 0 as libc::c_int
                    && stat_buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o140000 as libc::c_int as libc::c_uint)
                    as libc::c_int;
            }
            'c' => {
                return (sh_stat(arg, &mut stat_buf) == 0 as libc::c_int
                    && stat_buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o20000 as libc::c_int as libc::c_uint)
                    as libc::c_int;
            }
            'b' => {
                return (sh_stat(arg, &mut stat_buf) == 0 as libc::c_int
                    && stat_buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o60000 as libc::c_int as libc::c_uint)
                    as libc::c_int;
            }
            'p' => {
                return (sh_stat(arg, &mut stat_buf) == 0 as libc::c_int
                    && stat_buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o10000 as libc::c_int as libc::c_uint)
                    as libc::c_int;
            }
            'L' | 'h' => {
                return (*arg.offset(0 as libc::c_int as isize) as i32 != '\u{0}' as i32
                    && lstat(arg, &mut stat_buf) == 0 as i32
                    && stat_buf.st_mode & 0o170000 as i32 as u32 == 0o120000 as i32 as u32)
                    as libc::c_int;
            }
            'u' => {
                return (sh_stat(arg, &mut stat_buf) == 0 as libc::c_int
                    && stat_buf.st_mode & 0o4000 as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint) as libc::c_int;
            }
            'g' => {
                return (sh_stat(arg, &mut stat_buf) == 0 as libc::c_int
                    && stat_buf.st_mode & 0o2000 as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint) as libc::c_int;
            }
            'k' => {
                return (sh_stat(arg, &mut stat_buf) == 0 as libc::c_int
                    && stat_buf.st_mode & 0o1000 as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint) as libc::c_int;
            }
            't' => {
                if legal_number(arg, &mut r) == 0 as libc::c_int {
                    return 0 as libc::c_int;
                }
                return (r == r as libc::c_int as libc::c_long && isatty(r as libc::c_int) != 0)
                    as libc::c_int;
            }
            'n' => {
                return (*arg.offset(0 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32)
                    as libc::c_int;
            }
            'z' => {
                return (*arg.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32)
                    as libc::c_int;
            }
            'o' => {
                return (minus_o_option_value(arg) == 1 as libc::c_int) as libc::c_int;
            }
            'v' => {
                if valid_array_reference(arg, 0) != 0 {
                    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut rtype: libc::c_int = 0;
                    let mut ret: libc::c_int = 0;
                    let mut flags: libc::c_int = 0;
                    flags = if assoc_expand_once != 0 {
                        0x20 as libc::c_int
                    } else {
                        0 as libc::c_int
                    };
                    t = array_value(
                        arg,
                        0 as libc::c_int,
                        flags,
                        &mut rtype,
                        0 as *mut arrayind_t,
                    );
                    ret = if !t.is_null() {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    };
                    if rtype > 0 as libc::c_int {
                        free(t as *mut libc::c_void);
                    }
                    return ret;
                } else {
                    if legal_number(arg, &mut r) != 0 {
                        return if r >= 0 as libc::c_int as libc::c_long
                            && r <= number_of_args() as libc::c_long
                        {
                            1 as libc::c_int
                        } else {
                            0 as libc::c_int
                        };
                    }
                }

                v = find_variable(arg);
                if !v.is_null()
                    && (*v).attributes & 0x1000 as libc::c_int == 0 as libc::c_int
                    && (*v).attributes & 0x4 as libc::c_int != 0
                {
                    let mut t_0: *mut libc::c_char = 0 as *mut libc::c_char;
                    t_0 = array_reference((*v).value as *mut ARRAY, 0 as libc::c_int as arrayind_t);
                    return if !t_0.is_null() {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    };
                } else {
                    if !v.is_null()
                        && (*v).attributes & 0x1000 as libc::c_int == 0 as libc::c_int
                        && (*v).attributes & 0x40 as libc::c_int != 0
                    {
                        let mut t_1: *mut libc::c_char = 0 as *mut libc::c_char;
                        t_1 = assoc_reference(
                            (*v).value as *mut HASH_TABLE,
                            b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        );
                        return if !t_1.is_null() {
                            1 as libc::c_int
                        } else {
                            0 as libc::c_int
                        };
                    }
                }
                return if !v.is_null()
                    && (*v).attributes & 0x1000 as libc::c_int == 0 as libc::c_int
                    && !((*v).value).is_null()
                {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                };
            }
            'R' => {
                v = find_variable_noref(arg);
                return if !v.is_null()
                    && (*v).attributes & 0x1000 as libc::c_int == 0 as libc::c_int
                    && !((*v).value).is_null()
                    && (*v).attributes & 0x800 as libc::c_int != 0
                {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                };
            }
            _ => {}
        }
    }
    return 0;
}
