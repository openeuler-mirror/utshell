//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

use libc::size_t;
use r_bash::_ISalnum;
use r_bash::_ISalpha;
use r_bash::__ctype_b_loc;
use r_bash::__jmp_buf_tag;
use r_bash::__sigset_t;
use r_bash::__sigsetjmp;
use r_bash::array_variable_name;
use r_bash::array_variable_part;
use r_bash::arrayind_t;
use r_bash::assoc_expand_once;
use r_bash::bind_int_variable;
use r_bash::dcgettext;
use r_bash::err_unboundvar;
use r_bash::find_variable;
use r_bash::find_variable_last_nameref;
use r_bash::fmtumax;
use r_bash::get_array_value;
use r_bash::get_variable_value;
use r_bash::imaxdiv;
use r_bash::imaxdiv_t;
use r_bash::interactive_shell;
use r_bash::internal_error;
use r_bash::itos;
use r_bash::jump_to_top_level;
use r_bash::legal_identifier;
use r_bash::mbschr;
use r_bash::no_longjmp_on_fatal_error;
use r_bash::set_exit_status;
use r_bash::sh_xfree;
use r_bash::sh_xmalloc;
use r_bash::sh_xrealloc;
use r_bash::sigjmp_buf;
use r_bash::siglongjmp;
use r_bash::skipsubscript;
use r_bash::sprintf;
use r_bash::strcpy;
use r_bash::strlen;
use r_bash::stupidly_hack_special_variables;
use r_bash::this_command_name;
use r_bash::top_level_cleanup;
use r_bash::uintmax_t;
use r_bash::unbound_vars_is_error;
use r_bash::SHELL_VAR;

pub type intmax_t = libc::c_long;
pub const EXP_EXPANDED: libc::c_int = 1;
pub const ASS_NOEXPAND: libc::c_int = 128;
pub const AV_NOEXPAND: libc::c_int = 0x20;
pub const EXECUTION_FAILURE: libc::c_int = 1;
pub const DISCARD: libc::c_int = 2;
pub const FORCE_EOF: libc::c_int = 1;
pub const PREINC: libc::c_int = 14;
pub const PREDEC: libc::c_int = 15;
pub const STR: libc::c_int = 5;
pub const NUM: libc::c_int = 6;
pub const POSTINC: libc::c_int = 16;
pub const POSTDEC: libc::c_int = 17;
pub const POWER: libc::c_int = 13;
pub const LSH: libc::c_int = 9;
pub const RSH: libc::c_int = 10;
pub const LEQ: libc::c_int = 3;
pub const GEQ: libc::c_int = 4;
pub const EQEQ: libc::c_int = 1;
pub const NEQ: libc::c_int = 2;
pub const LAND: libc::c_int = 7;
pub const LOR: libc::c_int = 8;
pub const COND: libc::c_int = 12;
pub const OP_ASSIGN: libc::c_int = 11;
pub const MAX_EXPR_RECURSION_LEVEL: libc::c_int = 1024;
pub const EXPR_STACK_GROW_SIZE: libc::c_int = 10;
pub const EQ: libc::c_char = '=' as libc::c_char;
pub const NOT: libc::c_char = '!' as libc::c_char;
pub const GT: libc::c_char = '>' as libc::c_char;
pub const LT: libc::c_char = '<' as libc::c_char;
pub const BAND: libc::c_char = '&' as libc::c_char;
pub const BOR: libc::c_char = '|' as libc::c_char;

extern "C" {
    static bash_badsub_errmsg: *const libc::c_char;
}

static mut assigntok: libc::c_int = 0;
static mut expression: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut expr_depth: libc::c_int = 0;
static mut tokstr: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut evalbuf: sigjmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
static mut noeval: libc::c_int = 0;
static mut already_expanded: libc::c_int = 0;
static mut expr_stack: *mut *mut EXPR_CONTEXT =
    0 as *const *mut EXPR_CONTEXT as *mut *mut EXPR_CONTEXT;
static mut lasttok: libc::c_int = 0;
static mut curtok: libc::c_int = 0;
static mut tp: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut tokval: intmax_t = 0;
static mut expr_stack_size: libc::c_int = 0;
static mut lasttp: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut lastlval: lvalue = {
    let mut init = lvalue {
        tokstr: 0 as *const libc::c_char as *mut libc::c_char,
        tokval: 0 as libc::c_int as intmax_t,
        tokvar: 0 as *const SHELL_VAR as *mut SHELL_VAR,
        ind: -(1 as libc::c_int) as intmax_t,
    };
    init
};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct EXPR_CONTEXT {
    pub curtok: libc::c_int,
    pub lasttok: libc::c_int,
    pub expression: *mut libc::c_char,
    pub tp: *mut libc::c_char,
    pub lasttp: *mut libc::c_char,
    pub tokval: intmax_t,
    pub tokstr: *mut libc::c_char,
    pub noeval: libc::c_int,
    pub lval: lvalue,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct lvalue {
    pub tokstr: *mut libc::c_char,
    pub tokval: intmax_t,
    pub tokvar: *mut SHELL_VAR,
    pub ind: intmax_t,
}

static mut curlval: lvalue = {
    let mut init = lvalue {
        tokstr: 0 as *const libc::c_char as *mut libc::c_char,
        tokval: 0 as libc::c_int as intmax_t,
        tokvar: 0 as *const SHELL_VAR as *mut SHELL_VAR,
        ind: -(1 as libc::c_int) as intmax_t,
    };
    init
};

#[no_mangle]
unsafe extern "C" fn expr_bind_variable(mut lhs: *mut libc::c_char, mut rhs: *mut libc::c_char) {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut aflags: libc::c_int = 0;
    if lhs.is_null() || *lhs as libc::c_int == 0 as libc::c_int {
        return;
    }
    aflags = if assoc_expand_once != 0 && already_expanded != 0 {
        ASS_NOEXPAND as libc::c_int
    } else {
        0 as libc::c_int
    };
    v = bind_int_variable(lhs, rhs, aflags);
    if !v.is_null()
        && ((*v).attributes & 0x2 as libc::c_int != 0
            || (*v).attributes & 0x4000 as libc::c_int != 0)
    {
        siglongjmp(evalbuf.as_mut_ptr(), 1 as libc::c_int);
    }
    stupidly_hack_special_variables(lhs);
}

#[no_mangle]
unsafe extern "C" fn expr_streval(
    mut tok: *mut libc::c_char,
    mut e: libc::c_int,
    mut lvalue: *mut lvalue,
) -> intmax_t {
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tval: intmax_t = 0;
    let mut initial_depth: libc::c_int = 0;
    let mut ind: arrayind_t = 0;
    let mut tflag: libc::c_int = 0;
    let mut aflag: libc::c_int = 0;
    if noeval != 0 {
        return 0 as libc::c_int as intmax_t;
    }
    initial_depth = expr_depth;
    tflag = (assoc_expand_once != 0 && already_expanded != 0) as libc::c_int;
    aflag = if tflag != 0 {
        AV_NOEXPAND as libc::c_int
    } else {
        0 as libc::c_int
    };
    v = if e == ']' as i32 {
        array_variable_part(
            tok,
            tflag,
            0 as *mut *mut libc::c_char,
            0 as *mut libc::c_int,
        )
    } else {
        find_variable(tok)
    };
    if v.is_null() && e != ']' as i32 {
        v = find_variable_last_nameref(tok, 0 as libc::c_int);
    }
    if (v.is_null() || (*v).attributes & 0x1000 as libc::c_int != 0) && unbound_vars_is_error != 0 {
        value = if e == ']' as i32 {
            array_variable_name(
                tok,
                tflag,
                0 as *mut *mut libc::c_char,
                0 as *mut libc::c_int,
            )
        } else {
            tok
        };
        set_exit_status(EXECUTION_FAILURE as libc::c_int);
        err_unboundvar(value);
        if e == ']' as i32 {
            if !value.is_null() {
                sh_xfree(
                    value as *mut libc::c_void,
                    b"../expr.c\0" as *const u8 as *const libc::c_char,
                    1188 as libc::c_int,
                );
            }
        }
        if no_longjmp_on_fatal_error != 0 && interactive_shell != 0 {
            siglongjmp(evalbuf.as_mut_ptr(), 1 as libc::c_int);
        }
        if interactive_shell != 0 {
            expr_unwind();
            top_level_cleanup();
            jump_to_top_level(DISCARD as libc::c_int);
        } else {
            jump_to_top_level(FORCE_EOF as libc::c_int);
        }
    }
    ind = -(1 as libc::c_int) as arrayind_t;
    value = if e == ']' as i32 {
        get_array_value(
            tok,
            aflag,
            0 as *mut libc::c_void as *mut libc::c_int,
            &mut ind,
        )
    } else {
        get_variable_value(v)
    };
    if expr_depth < initial_depth {
        if no_longjmp_on_fatal_error != 0 && interactive_shell != 0 {
            siglongjmp(evalbuf.as_mut_ptr(), 1 as libc::c_int);
        }
        return 0 as libc::c_int as intmax_t;
    }
    tval = if !value.is_null() && *value as libc::c_int != 0 {
        subexpr(value)
    } else {
        0 as libc::c_int as libc::c_long
    };
    if !lvalue.is_null() {
        let ref mut fresh9 = (*lvalue).tokstr;
        *fresh9 = tok;
        (*lvalue).tokval = tval;
        let ref mut fresh10 = (*lvalue).tokvar;
        *fresh10 = v;
        (*lvalue).ind = ind;
    }
    return tval;
}

#[no_mangle]
unsafe extern "C" fn init_lvalue(mut lv: *mut lvalue) {
    let ref mut fresh6 = (*lv).tokstr;
    *fresh6 = 0 as *mut libc::c_char;
    let ref mut fresh7 = (*lv).tokvar;
    *fresh7 = 0 as *mut SHELL_VAR;
    let ref mut fresh8 = (*lv).ind;
    *fresh8 = -(1 as libc::c_int) as intmax_t;
    (*lv).tokval = *fresh8;
}

#[no_mangle]
unsafe extern "C" fn expr_bind_array_element(
    mut tok: *mut libc::c_char,
    mut ind: arrayind_t,
    mut rhs: *mut libc::c_char,
) {
    let mut lhs: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut llen: size_t = 0;
    let mut ibuf: [libc::c_char; 22] = [0; 22];
    let mut istr: *mut libc::c_char = 0 as *mut libc::c_char;
    istr = fmtumax(
        ind as uintmax_t,
        10 as libc::c_int,
        ibuf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 22]>() as usize,
        0 as libc::c_int,
    );
    vname = array_variable_name(
        tok,
        0 as libc::c_int,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_int,
    );
    llen = (strlen(vname))
        .wrapping_add(::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
        .wrapping_add(3 as libc::c_int as libc::c_ulong) as usize;
    lhs = sh_xmalloc(
        llen as libc::c_ulong,
        b"../expr.c\0" as *const u8 as *const libc::c_char,
        380 as libc::c_int,
    ) as *mut libc::c_char;
    sprintf(
        lhs,
        b"%s[%s]\0" as *const u8 as *const libc::c_char,
        vname,
        istr,
    );
    expr_bind_variable(lhs, rhs);
    sh_xfree(
        vname as *mut libc::c_void,
        b"../expr.c\0" as *const u8 as *const libc::c_char,
        386 as libc::c_int,
    );
    sh_xfree(
        lhs as *mut libc::c_void,
        b"../expr.c\0" as *const u8 as *const libc::c_char,
        387 as libc::c_int,
    );
}

#[no_mangle]
unsafe extern "C" fn ipow(mut base: intmax_t, mut exp: intmax_t) -> intmax_t {
    let mut result: intmax_t = 0;
    result = 1 as libc::c_int as intmax_t;
    while exp != 0 {
        if exp & 1 as libc::c_int as libc::c_long != 0 {
            result *= base;
        }
        exp >>= 1 as libc::c_int;
        base *= base;
    }
    return result;
}

#[no_mangle]
unsafe extern "C" fn exp1() -> intmax_t {
    let mut val: intmax_t = 0;
    if curtok == '!' as i32 {
        readtok();
        val = (exp1() == 0) as libc::c_int as intmax_t;
        lasttok = 6 as libc::c_int;
    } else if curtok == '~' as i32 {
        readtok();
        val = !exp1();
        lasttok = 6 as libc::c_int;
    } else if curtok == '-' as i32 {
        readtok();
        val = -exp1();
        lasttok = 6 as libc::c_int;
    } else if curtok == '+' as i32 {
        readtok();
        val = exp1();
        lasttok = 6 as libc::c_int;
    } else {
        val = exp0();
    }
    return val;
}

#[no_mangle]
unsafe extern "C" fn exp0() -> intmax_t {
    let mut val: intmax_t = 0 as libc::c_int as intmax_t;
    let mut v2: intmax_t = 0;
    let mut vincdec: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stok: libc::c_int = 0;
    let mut ec: EXPR_CONTEXT = EXPR_CONTEXT {
        curtok: 0,
        lasttok: 0,
        expression: 0 as *mut libc::c_char,
        tp: 0 as *mut libc::c_char,
        lasttp: 0 as *mut libc::c_char,
        tokval: 0,
        tokstr: 0 as *mut libc::c_char,
        noeval: 0,
        lval: lvalue {
            tokstr: 0 as *mut libc::c_char,
            tokval: 0,
            tokvar: 0 as *mut SHELL_VAR,
            ind: 0,
        },
    };
    if curtok == PREINC as libc::c_int || curtok == PREDEC as libc::c_int {
        lasttok = curtok;
        stok = lasttok;
        readtok();
        if curtok != STR as libc::c_int {
            evalerror(dcgettext(
                0 as *const libc::c_char,
                b"identifier expected after pre-increment or pre-decrement\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ));
        }
        v2 = tokval
            + (if stok == PREINC as libc::c_int {
                1 as libc::c_int
            } else {
                -(1 as libc::c_int)
            }) as libc::c_long;
        vincdec = itos(v2);
        if noeval == 0 as libc::c_int {
            if curlval.ind != -(1 as libc::c_int) as libc::c_long {
                expr_bind_array_element(curlval.tokstr, curlval.ind, vincdec);
            } else if !tokstr.is_null() {
                expr_bind_variable(tokstr, vincdec);
            }
        }
        sh_xfree(
            vincdec as *mut libc::c_void,
            b"../expr.c\0" as *const u8 as *const libc::c_char,
            1043 as libc::c_int,
        );
        val = v2;
        curtok = NUM as libc::c_int;
        readtok();
    } else if curtok == '(' as i32 {
        readtok();
        val = expcomma();
        if curtok != ')' as i32 {
            evalerror(dcgettext(
                0 as *const libc::c_char,
                b"missing `)'\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ));
        }
        readtok();
    } else if curtok == NUM as libc::c_int || curtok == STR as libc::c_int {
        val = tokval;
        if curtok == STR as libc::c_int {
            ec.curtok = curtok;
            ec.lasttok = lasttok;
            ec.tp = tp;
            ec.lasttp = lasttp;
            ec.tokval = tokval;
            ec.tokstr = tokstr;
            ec.noeval = noeval;
            ec.lval = curlval;
            tokstr = 0 as *mut libc::c_void as *mut libc::c_char;
            noeval = 1 as libc::c_int;
            readtok();
            stok = curtok;
            if stok == POSTINC as libc::c_int || stok == POSTDEC as libc::c_int {
                tokstr = ec.tokstr;
                noeval = ec.noeval;
                curlval = ec.lval;
                lasttok = 5 as libc::c_int;
                v2 = val
                    + (if stok == POSTINC as libc::c_int {
                        1 as libc::c_int
                    } else {
                        -(1 as libc::c_int)
                    }) as libc::c_long;
                vincdec = itos(v2);
                if noeval == 0 as libc::c_int {
                    if curlval.ind != -(1 as libc::c_int) as libc::c_long {
                        expr_bind_array_element(curlval.tokstr, curlval.ind, vincdec);
                    } else {
                        expr_bind_variable(tokstr, vincdec);
                    }
                }
                sh_xfree(
                    vincdec as *mut libc::c_void,
                    b"../expr.c\0" as *const u8 as *const libc::c_char,
                    1092 as libc::c_int,
                );
                curtok = 6 as libc::c_int;
            } else {
                if stok == 5 as libc::c_int {
                    if !tokstr.is_null() {
                        sh_xfree(
                            tokstr as *mut libc::c_void,
                            b"../expr.c\0" as *const u8 as *const libc::c_char,
                            1099 as libc::c_int,
                        );
                    }
                }
                curtok = ec.curtok;
                lasttok = ec.lasttok;
                tp = ec.tp;
                lasttp = ec.lasttp;
                tokval = ec.tokval;
                tokstr = ec.tokstr;
                noeval = ec.noeval;
                curlval = ec.lval;
            }
        }
        readtok();
    } else {
        evalerror(dcgettext(
            0 as *const libc::c_char,
            b"syntax error: operand expected\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ));
    }
    return val;
}

#[no_mangle]
unsafe extern "C" fn exppower() -> intmax_t {
    let mut val1: intmax_t = 0;
    let mut val2: intmax_t = 0;
    let mut c: intmax_t = 0;
    val1 = exp1();
    while curtok == POWER as libc::c_int {
        readtok();
        val2 = exppower();
        lasttok = NUM as libc::c_int;
        if val2 == 0 as libc::c_int as libc::c_long {
            return 1 as libc::c_int as intmax_t;
        }
        if val2 < 0 as libc::c_int as libc::c_long {
            evalerror(dcgettext(
                0 as *const libc::c_char,
                b"exponent less than 0\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ));
        }
        val1 = ipow(val1, val2);
    }
    return val1;
}

#[no_mangle]
unsafe extern "C" fn expmuldiv() -> intmax_t {
    let mut val1: intmax_t = 0;
    let mut val2: intmax_t = 0;
    let mut idiv: imaxdiv_t = imaxdiv_t { quot: 0, rem: 0 };
    val1 = exppower();
    while curtok == '*' as i32 || curtok == '/' as i32 || curtok == '%' as i32 {
        let mut op: libc::c_int = curtok;
        let mut stp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut sltp: *mut libc::c_char = 0 as *mut libc::c_char;
        stp = tp;
        readtok();
        val2 = exppower();
        if (op == '/' as i32 || op == '%' as i32) && val2 == 0 as libc::c_int as libc::c_long {
            if noeval == 0 as libc::c_int {
                sltp = lasttp;
                lasttp = stp;
                while !lasttp.is_null()
                    && *lasttp as libc::c_int != 0
                    && (*lasttp as libc::c_int == ' ' as i32
                        || *lasttp as libc::c_int == '\t' as i32)
                {
                    lasttp = lasttp.offset(1);
                }
                evalerror(dcgettext(
                    0 as *const libc::c_char,
                    b"division by 0\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ));
                lasttp = sltp;
            } else {
                val2 = 1 as libc::c_int as intmax_t;
            }
        } else if op == '%' as i32
            && val1 == -(9223372036854775807 as libc::c_long) - 1 as libc::c_int as libc::c_long
            && val2 == -(1 as libc::c_int) as libc::c_long
        {
            val1 = 0 as libc::c_int as intmax_t;
            continue;
        } else if op == '/' as i32
            && val1 == -(9223372036854775807 as libc::c_long) - 1 as libc::c_int as libc::c_long
            && val2 == -(1 as libc::c_int) as libc::c_long
        {
            val2 = 1 as libc::c_int as intmax_t;
        }
        if op == '*' as i32 {
            val1 *= val2;
        } else if op == '/' as i32 || op == '%' as i32 {
            idiv = imaxdiv(val1, val2);
            val1 = if op == '/' as i32 {
                idiv.quot
            } else {
                idiv.rem
            };
        }
        lasttok = 6 as libc::c_int;
    }
    return val1;
}

