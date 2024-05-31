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

#[no_mangle]
unsafe extern "C" fn exp3() -> intmax_t {
    let mut val1: intmax_t = 0;
    let mut val2: intmax_t = 0;
    val1 = expmuldiv();
    while curtok == '+' as i32 || curtok == '-' as i32 {
        let mut op: libc::c_int = curtok;
        readtok();
        val2 = expmuldiv();
        if op == '+' as i32 {
            val1 += val2;
        } else if op == '-' as i32 {
            val1 -= val2;
        }
        lasttok = NUM as libc::c_int;
    }
    return val1;
}

#[no_mangle]
unsafe extern "C" fn expshift() -> intmax_t {
    let mut val1: intmax_t = 0;
    let mut val2: intmax_t = 0;
    val1 = exp3();
    while curtok == LSH as libc::c_int || curtok == RSH as libc::c_int {
        let mut op: libc::c_int = curtok;
        readtok();
        val2 = exp3();
        if op == LSH as libc::c_int {
            val1 = val1 << val2;
        } else {
            val1 = val1 >> val2;
        }
        lasttok = NUM as libc::c_int;
    }
    return val1;
}

#[no_mangle]
unsafe extern "C" fn exp4() -> intmax_t {
    let mut val1: intmax_t = 0;
    let mut val2: intmax_t = 0;
    val1 = expshift();
    while curtok == LEQ as libc::c_int
        || curtok == GEQ as libc::c_int
        || curtok == '<' as i32
        || curtok == '>' as i32
    {
        let mut op: libc::c_int = curtok;
        readtok();
        val2 = expshift();
        if op == LEQ as libc::c_int {
            val1 = (val1 <= val2) as libc::c_int as intmax_t;
        } else if op == GEQ as libc::c_int {
            val1 = (val1 >= val2) as libc::c_int as intmax_t;
        } else if op == '<' as i32 {
            val1 = (val1 < val2) as libc::c_int as intmax_t;
        } else {
            val1 = (val1 > val2) as libc::c_int as intmax_t;
        }
        lasttok = NUM as libc::c_int;
    }
    return val1;
}

#[no_mangle]
unsafe extern "C" fn exp5() -> intmax_t {
    let mut val1: intmax_t = 0;
    let mut val2: intmax_t = 0;
    val1 = exp4();
    while curtok == EQEQ as libc::c_int || curtok == NEQ as libc::c_int {
        let mut op: libc::c_int = curtok;
        readtok();
        val2 = exp4();
        if op == EQEQ as libc::c_int {
            val1 = (val1 == val2) as libc::c_int as intmax_t;
        } else if op == NEQ as libc::c_int {
            val1 = (val1 != val2) as libc::c_int as intmax_t;
        }
        lasttok = NUM as libc::c_int;
    }
    return val1;
}

#[no_mangle]
unsafe extern "C" fn expband() -> intmax_t {
    let mut val1: intmax_t = 0;
    let mut val2: intmax_t = 0;
    val1 = exp5();
    while curtok == '&' as i32 {
        readtok();
        val2 = exp5();
        val1 = val1 & val2;
        lasttok = NUM as libc::c_int;
    }
    return val1;
}

#[no_mangle]
unsafe extern "C" fn expbxor() -> intmax_t {
    let mut val1: intmax_t = 0;
    let mut val2: intmax_t = 0;
    val1 = expband();
    while curtok == '^' as i32 {
        readtok();
        val2 = expband();
        val1 = val1 ^ val2;
        lasttok = NUM as libc::c_int;
    }
    return val1;
}


#[no_mangle]
unsafe extern "C" fn expbor() -> intmax_t {
    let mut val1: intmax_t = 0;
    let mut val2: intmax_t = 0;
    val1 = expbxor();
    while curtok == '|' as i32 {
        readtok();
        val2 = expbxor();
        val1 = val1 | val2;
        lasttok = NUM as libc::c_int;
    }
    return val1;
}

#[no_mangle]
unsafe extern "C" fn expland() -> intmax_t {
    let mut val1: intmax_t = 0;
    let mut val2: intmax_t = 0;
    let mut set_noeval: libc::c_int = 0;
    val1 = expbor();
    while curtok == LAND as libc::c_int {
        set_noeval = 0 as libc::c_int;
        if val1 == 0 as libc::c_int as libc::c_long {
            set_noeval = 1 as libc::c_int;
            noeval += 1;
        }
        readtok();
        val2 = expbor();
        if set_noeval != 0 {
            noeval -= 1;
        }
        val1 = (val1 != 0 && val2 != 0) as libc::c_int as intmax_t;
        lasttok = LAND as libc::c_int;
    }
    return val1;
}

#[no_mangle]
unsafe extern "C" fn explor() -> intmax_t {
    let mut val1: intmax_t = 0;
    let mut val2: intmax_t = 0;
    let mut set_noeval: libc::c_int = 0;
    val1 = expland();
    while curtok == LOR as libc::c_int {
        set_noeval = 0 as libc::c_int;
        if val1 != 0 as libc::c_int as libc::c_long {
            noeval += 1;
            set_noeval = 1 as libc::c_int;
        }
        readtok();
        val2 = expland();
        if set_noeval != 0 {
            noeval -= 1;
        }
        val1 = (val1 != 0 || val2 != 0) as libc::c_int as intmax_t;
        lasttok = LOR as libc::c_int;
    }
    return val1;
}

#[no_mangle]
unsafe extern "C" fn expcond() -> intmax_t {
    let mut cval: intmax_t = 0;
    let mut val1: intmax_t = 0;
    let mut val2: intmax_t = 0;
    let mut rval: intmax_t = 0;
    let mut set_noeval: libc::c_int = 0;
    set_noeval = 0 as libc::c_int;
    cval = explor();
    rval = cval;
    if curtok == '?' as i32 {
        if cval == 0 as libc::c_int as libc::c_long {
            set_noeval = 1 as libc::c_int;
            noeval += 1;
        }
        readtok();
        if curtok == 0 as libc::c_int || curtok == ':' as i32 {
            evalerror(dcgettext(
                0 as *const libc::c_char,
                b"expression expected\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ));
        }
        val1 = expcomma();
        if set_noeval != 0 {
            noeval -= 1;
        }
        if curtok != ':' as i32 {
            evalerror(dcgettext(
                0 as *const libc::c_char,
                b"`:' expected for conditional expression\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ));
        }
        set_noeval = 0 as libc::c_int;
        if cval != 0 {
            set_noeval = 1 as libc::c_int;
            noeval += 1;
        }
        readtok();
        if curtok == 0 as libc::c_int {
            evalerror(dcgettext(
                0 as *const libc::c_char,
                b"expression expected\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ));
        }
        val2 = expcond();
        if set_noeval != 0 {
            noeval -= 1;
        }
        rval = if cval != 0 { val1 } else { val2 };
        lasttok = COND as libc::c_int;
    }
    return rval;
}

#[no_mangle]
unsafe extern "C" fn expassign() -> intmax_t {
    let mut value: intmax_t = 0;
    let mut lhs: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rhs: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lind: arrayind_t = 0;
    let mut idiv: imaxdiv_t = imaxdiv_t { quot: 0, rem: 0 };
    value = expcond();
    if curtok == '=' as i32 || curtok == 11 as libc::c_int {
        let mut special: libc::c_int = 0;
        let mut op: libc::c_int = 0;
        let mut lvalue: intmax_t = 0;
        special = (curtok == 11 as libc::c_int) as libc::c_int;
        if lasttok != 5 as libc::c_int {
            evalerror(dcgettext(
                0 as *const libc::c_char,
                b"attempted assignment to non-variable\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ));
        }
        if special != 0 {
            op = assigntok;
            lvalue = value;
        }
        if tokstr.is_null() {
            evalerror(dcgettext(
                0 as *const libc::c_char,
                b"syntax error in variable assignment\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ));
        }
        lhs = strcpy(
            sh_xmalloc(
                (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(tokstr)),
                b"../expr.c\0" as *const u8 as *const libc::c_char,
                533 as libc::c_int,
            ) as *mut libc::c_char,
            tokstr,
        );
        lind = curlval.ind;
        readtok();
        value = expassign();
        if special != 0 {
            if (op == '/' as i32 || op == '%' as i32) && value == 0 as libc::c_int as libc::c_long {
                if noeval == 0 as libc::c_int {
                    evalerror(dcgettext(
                        0 as *const libc::c_char,
                        b"division by 0\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ));
                } else {
                    value = 1 as libc::c_int as intmax_t;
                }
            }
            match op {
                42 => {
                    // '*'
                    lvalue *= value;
                }
                47 | 37 => {
                    // '/' | '%'
                    if lvalue
                        == -(9223372036854775807 as libc::c_long) - 1 as libc::c_int as libc::c_long
                        && value == -(1 as libc::c_int) as libc::c_long
                    {
                        lvalue = if op == '/' as i32 {
                            -(9223372036854775807 as libc::c_long)
                                - 1 as libc::c_int as libc::c_long
                        } else {
                            0 as libc::c_int as libc::c_long
                        };
                    } else {
                        idiv = imaxdiv(lvalue, value);
                        lvalue = if op == '/' as i32 {
                            idiv.quot
                        } else {
                            idiv.rem
                        };
                    }
                }
                43 => {
                    // '+'
                    lvalue += value;
                }
                45 => {
                    // '-'
                    lvalue -= value;
                }
                LSH => {
                    lvalue <<= value;
                }
                RSH => {
                    lvalue >>= value;
                }
                38 => {
                    // '&'
                    lvalue &= value;
                }
                124 => {
                    // '|'
                    lvalue |= value;
                }
                94 => {
                    // '^'
                    lvalue ^= value;
                }
                _ => {
                    sh_xfree(
                        lhs as *mut libc::c_void,
                        b"../expr.c\0" as *const u8 as *const libc::c_char,
                        591 as libc::c_int,
                    );
                    evalerror(dcgettext(
                        0 as *const libc::c_char,
                        b"bug: bad expassign token\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ));
                }
            }
            value = lvalue;
        }
        rhs = itos(value);
        if noeval == 0 as libc::c_int {
            if lind != -(1 as libc::c_int) as libc::c_long {
                expr_bind_array_element(lhs, lind, rhs);
            } else {
                expr_bind_variable(lhs, rhs);
            }
        }
        if !(curlval.tokstr).is_null() && curlval.tokstr == tokstr {
            init_lvalue(&mut curlval);
        }
        sh_xfree(
            rhs as *mut libc::c_void,
            b"../expr.c\0" as *const u8 as *const libc::c_char,
            611 as libc::c_int,
        );
        sh_xfree(
            lhs as *mut libc::c_void,
            b"../expr.c\0" as *const u8 as *const libc::c_char,
            612 as libc::c_int,
        );
        if !tokstr.is_null() {
            sh_xfree(
                tokstr as *mut libc::c_void,
                b"../expr.c\0" as *const u8 as *const libc::c_char,
                613 as libc::c_int,
            );
        }
        tokstr = 0 as *mut libc::c_void as *mut libc::c_char;
    }
    return value;
}

#[no_mangle]
unsafe extern "C" fn expcomma() -> intmax_t {
    let mut value: intmax_t = 0;
    value = expassign();
    while curtok == ',' as i32 {
        readtok();
        value = expassign();
    }
    return value;
}

#[no_mangle]
unsafe extern "C" fn strlong(mut num: *mut libc::c_char) -> intmax_t {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_uchar = 0;
    let mut base: libc::c_int = 0;
    let mut foundbase: libc::c_int = 0;
    let mut val: intmax_t = 0;
    s = num;
    base = 10 as libc::c_int;
    foundbase = 0 as libc::c_int;
    if *s as libc::c_int == '0' as i32 {
        s = s.offset(1);
        if *s as libc::c_int == '\u{0}' as i32 {
            return 0 as libc::c_int as intmax_t;
        }
        if *s as libc::c_int == 'x' as i32 || *s as libc::c_int == 'X' as i32 {
            base = 16 as libc::c_int;
            s = s.offset(1);
        } else {
            base = 8 as libc::c_int;
        }
        foundbase += 1;
    }
    val = 0 as libc::c_int as intmax_t;
    let fresh14 = s;
    s = s.offset(1);
    c = *fresh14 as libc::c_uchar;
    while c != 0 {
        if c as libc::c_int == '#' as i32 {
            if foundbase != 0 {
                evalerror(dcgettext(
                    0 as *const libc::c_char,
                    b"invalid number\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ));
            }
            if val < 2 as libc::c_int as libc::c_long || val > 64 as libc::c_int as libc::c_long {
                evalerror(dcgettext(
                    0 as *const libc::c_char,
                    b"invalid arithmetic base\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ));
            }
            base = val as libc::c_int;
            val = 0 as libc::c_int as intmax_t;
            foundbase += 1;
            if (1 as libc::c_int != 0
                && *(*__ctype_b_loc()).offset(*s as libc::c_uchar as libc::c_int as isize)
                    as libc::c_int
                    & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                    != 0
                || *s as libc::c_int == '_' as i32
                || *s as libc::c_int == '@' as i32) as libc::c_int
                == 0 as libc::c_int
            {
                evalerror(dcgettext(
                    0 as *const libc::c_char,
                    b"invalid integer constant\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ));
            }
        } else {
            if !(1 as libc::c_int != 0
                && *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                    & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                    != 0
                || c as libc::c_int == '_' as i32
                || c as libc::c_int == '@' as i32)
            {
                break;
            }
            if c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32 {
                c = (c as libc::c_int - '0' as i32) as libc::c_uchar;
            } else if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'z' as i32 {
                c = (c as libc::c_int - ('a' as i32 - 10 as libc::c_int)) as libc::c_uchar;
            } else if c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'Z' as i32 {
                c = (c as libc::c_int
                    - ('A' as i32
                        - (if base <= 36 as libc::c_int {
                            10 as libc::c_int
                        } else {
                            36 as libc::c_int
                        }))) as libc::c_uchar;
            } else if c as libc::c_int == '@' as i32 {
                c = 62 as libc::c_int as libc::c_uchar;
            } else if c as libc::c_int == '_' as i32 {
                c = 63 as libc::c_int as libc::c_uchar;
            }
            if c as libc::c_int >= base {
                evalerror(dcgettext(
                    0 as *const libc::c_char,
                    b"value too great for base\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ));
            }
            val = val * base as libc::c_long + c as libc::c_long;
        }
        let fresh15 = s;
        s = s.offset(1);
        c = *fresh15 as libc::c_uchar;
    }
    return val;
}

#[no_mangle]
unsafe extern "C" fn expr_skipsubscript(
    mut vp: *mut libc::c_char,
    mut cp: *mut libc::c_char,
) -> libc::c_int {
    let mut flags: libc::c_int = 0;
    let mut isassoc: libc::c_int = 0;
    let mut entry: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    isassoc = 0 as libc::c_int;
    entry = 0 as *mut SHELL_VAR;
    if assoc_expand_once & already_expanded != 0 {
        *cp = '\u{0}' as i32 as libc::c_char;
        isassoc = (legal_identifier(vp) != 0
            && {
                entry = find_variable(vp);
                !entry.is_null()
            }
            && (*entry).attributes & 0x40 as libc::c_int != 0) as libc::c_int;
        *cp = '[' as i32 as libc::c_char;
    }
    flags = if isassoc != 0 && assoc_expand_once != 0 && already_expanded != 0 {
        0x1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    return skipsubscript(cp, 0 as libc::c_int, flags);
}


#[no_mangle]
unsafe extern "C" fn evalerror(mut msg: *const libc::c_char) {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    name = this_command_name;
    t = expression;
    while !t.is_null() && (*t as libc::c_int == ' ' as i32 || *t as libc::c_int == '\t' as i32) {
        t = t.offset(1);
    }
    internal_error(
        dcgettext(
            0 as *const libc::c_char,
            b"%s%s%s: %s (error token is \"%s\")\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        if !name.is_null() {
            name
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if !name.is_null() {
            b": \0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if !t.is_null() {
            t
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        msg,
        if !lasttp.is_null() && *lasttp as libc::c_int != 0 {
            lasttp
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    siglongjmp(evalbuf.as_mut_ptr(), 1 as libc::c_int);
}


#[no_mangle]
unsafe extern "C" fn pushexp() {
    let mut context: *mut EXPR_CONTEXT = 0 as *mut EXPR_CONTEXT;
    if expr_depth >= MAX_EXPR_RECURSION_LEVEL as libc::c_int {
        evalerror(dcgettext(
            0 as *const libc::c_char,
            b"expression recursion level exceeded\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ));
    }
    if expr_depth >= expr_stack_size {
        expr_stack_size += EXPR_STACK_GROW_SIZE as libc::c_int;
        expr_stack = sh_xrealloc(
            expr_stack as *mut libc::c_void,
            (expr_stack_size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut EXPR_CONTEXT>() as libc::c_ulong),
            b"../expr.c\0" as *const u8 as *const libc::c_char,
            268 as libc::c_int,
        ) as *mut *mut EXPR_CONTEXT;
    }
    context = sh_xmalloc(
        ::std::mem::size_of::<EXPR_CONTEXT>() as libc::c_ulong,
        b"../expr.c\0" as *const u8 as *const libc::c_char,
        271 as libc::c_int,
    ) as *mut EXPR_CONTEXT;
    let ref mut fresh0 = (*context).expression;
    *fresh0 = expression;
    (*context).curtok = curtok;
    (*context).lasttok = lasttok;
    let ref mut fresh1 = (*context).tp;
    *fresh1 = tp;
    let ref mut fresh2 = (*context).lasttp;
    *fresh2 = lasttp;
    (*context).tokval = tokval;
    let ref mut fresh3 = (*context).tokstr;
    *fresh3 = tokstr;
    (*context).noeval = noeval;
    (*context).lval = curlval;
    let fresh4 = expr_depth;
    expr_depth = expr_depth + 1;
    let ref mut fresh5 = *expr_stack.offset(fresh4 as isize);
    *fresh5 = context;
}

#[no_mangle]
unsafe extern "C" fn popexp() {
    let mut context: *mut EXPR_CONTEXT = 0 as *mut EXPR_CONTEXT;
    if expr_depth <= 0 as libc::c_int {
        lasttp = 0 as *mut libc::c_char;
        expression = lasttp;
        evalerror(dcgettext(
            0 as *const libc::c_char,
            b"recursion stack underflow\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ));
    }
    expr_depth -= 1;
    context = *expr_stack.offset(expr_depth as isize);
    expression = (*context).expression;
    curtok = (*context).curtok;
    lasttok = (*context).lasttok;
    tp = (*context).tp;
    lasttp = (*context).lasttp;
    tokval = (*context).tokval;
    tokstr = (*context).tokstr;
    noeval = (*context).noeval;
    curlval = (*context).lval;
    sh_xfree(
        context as *mut libc::c_void,
        b"../expr.c\0" as *const u8 as *const libc::c_char,
        299 as libc::c_int,
    );
}

#[no_mangle]
unsafe extern "C" fn _is_arithop(mut c: libc::c_int) -> libc::c_int {
    match c as u8 as char {
        '=' | '>' | '<' | '+' | '-' | '*' | '/' | '%' | '!' | '(' | ')' | '&' | '|' | '^' | '~' => {
            return 1 as libc::c_int;
        }
        '?' | ':' | ',' => return 1 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}

#[no_mangle]
unsafe extern "C" fn _is_multiop(mut c: libc::c_int) -> libc::c_int {
    match c {
        EQEQ | NEQ | LEQ | GEQ | LAND | LOR | LSH | RSH | OP_ASSIGN | COND | POWER | PREINC
        | PREDEC | POSTINC | POSTDEC => {
            return 1 as libc::c_int;
        }
        _ => return 0 as libc::c_int,
    };
}

#[no_mangle]
unsafe extern "C" fn readtok() {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut xp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_uchar = 0;
    let mut c1: libc::c_uchar = 0;
    let mut e: libc::c_int = 0;
    let mut lval: lvalue = lvalue {
        tokstr: 0 as *mut libc::c_char,
        tokval: 0,
        tokvar: 0 as *mut SHELL_VAR,
        ind: 0,
    };
    cp = tp;
    e = 0 as libc::c_int;
    c = e as libc::c_uchar;
    while !cp.is_null()
        && {
            c = *cp as libc::c_uchar;
            c as libc::c_int != 0
        }
        && (c as libc::c_int == ' ' as i32
            || c as libc::c_int == '\t' as i32
            || c as libc::c_int == '\n' as i32)
    {
        cp = cp.offset(1);
    }
    if c != 0 {
        cp = cp.offset(1);
    }
    if c as libc::c_int == '\u{0}' as i32 {
        lasttok = curtok;
        curtok = 0 as libc::c_int;
        tp = cp;
        return;
    }
    tp = cp.offset(-(1 as libc::c_int as isize));
    lasttp = tp;
    if 1 as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
            & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        || c as libc::c_int == '_' as i32
    {
        let mut savecp: *mut libc::c_char = 0 as *mut libc::c_char;
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
        let mut peektok: libc::c_int = 0;
        while 1 as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            || c as libc::c_int == '_' as i32
        {
            let fresh11 = cp;
            cp = cp.offset(1);
            c = *fresh11 as libc::c_uchar;
        }
        cp = cp.offset(-1);
        c = *cp as libc::c_uchar;
        if c as libc::c_int == '[' as i32 {
            e = expr_skipsubscript(tp, cp);
            if *cp.offset(e as isize) as libc::c_int == ']' as i32 {
                cp = cp.offset((e + 1 as libc::c_int) as isize);
                c = *cp as libc::c_uchar;
                e = ']' as i32;
            } else {
                evalerror(bash_badsub_errmsg);
            }
        }
        *cp = '\u{0}' as i32 as libc::c_char;
        if !(curlval.tokstr).is_null() && curlval.tokstr == tokstr {
            init_lvalue(&mut curlval);
        }
        if !tokstr.is_null() {
            sh_xfree(
                tokstr as *mut libc::c_void,
                b"../expr.c\0" as *const u8 as *const libc::c_char,
                1360 as libc::c_int,
            );
        }
        tokstr = strcpy(
            sh_xmalloc(
                (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(tp)),
                b"../expr.c\0" as *const u8 as *const libc::c_char,
                1361 as libc::c_int,
            ) as *mut libc::c_char,
            tp,
        );
        *cp = c as libc::c_char;
        ec.curtok = curtok;
        ec.lasttok = lasttok;
        ec.tp = tp;
        ec.lasttp = lasttp;
        ec.tokval = tokval;
        ec.tokstr = tokstr;
        ec.noeval = noeval;
        ec.lval = curlval;
        tokstr = 0 as *mut libc::c_void as *mut libc::c_char;
        savecp = cp;
        tp = savecp;
        noeval = 1 as libc::c_int;
        curtok = STR as libc::c_int;
        readtok();
        peektok = curtok;
        if peektok == STR as libc::c_int {
            if !tokstr.is_null() {
                sh_xfree(
                    tokstr as *mut libc::c_void,
                    b"../expr.c\0" as *const u8 as *const libc::c_char,
                    1373 as libc::c_int,
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
        cp = savecp;
        if lasttok == PREINC as libc::c_int
            || lasttok == PREDEC as libc::c_int
            || peektok != '=' as i32
        {
            lastlval = curlval;
            tokval = expr_streval(tokstr, e, &mut curlval);
        } else {
            tokval = 0 as libc::c_int as intmax_t;
        }
        lasttok = curtok;
        curtok = STR as libc::c_int;
    } else if c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32 {
        while 1 as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            || c as libc::c_int == '#' as i32
            || c as libc::c_int == '@' as i32
            || c as libc::c_int == '_' as i32
        {
            let fresh12 = cp;
            cp = cp.offset(1);
            c = *fresh12 as libc::c_uchar;
        }
        cp = cp.offset(-1);
        c = *cp as libc::c_uchar;
        *cp = '\u{0}' as i32 as libc::c_char;
        tokval = strlong(tp);
        *cp = c as libc::c_char;
        lasttok = curtok;
        curtok = NUM as libc::c_int;
    } else {
        let fresh13 = cp;
        cp = cp.offset(1);
        c1 = *fresh13 as libc::c_uchar;
        if c as libc::c_int == EQ as i32 && c1 as libc::c_int == EQ as i32 {
            c = EQEQ as libc::c_int as libc::c_uchar;
        } else if c as libc::c_int == NOT as i32 && c1 as libc::c_int == EQ as i32 {
            c = NEQ as libc::c_int as libc::c_uchar;
        } else if c as libc::c_int == GT as i32 && c1 as libc::c_int == EQ as i32 {
            c = GEQ as libc::c_int as libc::c_uchar;
        } else if c as libc::c_int == LT as i32 && c1 as libc::c_int == EQ as i32 {
            c = LEQ as libc::c_int as libc::c_uchar;
        } else if c as libc::c_int == LT as i32 && c1 as libc::c_int == LT as i32 {
            if *cp as libc::c_int == '=' as i32 {
                assigntok = LSH as libc::c_int;
                c = OP_ASSIGN as libc::c_int as libc::c_uchar;
                cp = cp.offset(1);
            } else {
                c = LSH as libc::c_int as libc::c_uchar;
            }
        } else if c as libc::c_int == '>' as i32 && c1 as libc::c_int == '>' as i32 {
            if *cp as libc::c_int == '=' as i32 {
                assigntok = 10 as libc::c_int;
                c = 11 as libc::c_int as libc::c_uchar;
                cp = cp.offset(1);
            } else {
                c = 10 as libc::c_int as libc::c_uchar;
            }
        } else if c as libc::c_int == BAND as i32 && c1 as libc::c_int == BAND as i32 {
            c = LAND as libc::c_int as libc::c_uchar;
        } else if c as libc::c_int == BOR as i32 && c1 as libc::c_int == BOR as i32 {
            c = LOR as libc::c_int as libc::c_uchar;
        } else if c as libc::c_int == '*' as i32 && c1 as libc::c_int == '*' as i32 {
            c = POWER as libc::c_int as libc::c_uchar;
        } else if (c as libc::c_int == '-' as i32 || c as libc::c_int == '+' as i32)
            && c1 as libc::c_int == c as libc::c_int
            && curtok == 5 as libc::c_int
        {
            c = (if c as libc::c_int == '-' as i32 {
                17 as libc::c_int
            } else {
                16 as libc::c_int
            }) as libc::c_uchar;
        } else if (c as libc::c_int == '-' as i32 || c as libc::c_int == '+' as i32)
            && c1 as libc::c_int == c as libc::c_int
            && curtok == NUM as libc::c_int
            && (lasttok == PREINC as libc::c_int || lasttok == PREDEC as libc::c_int)
        {
            if c as libc::c_int == '-' as i32 {
                evalerror(b"--: assignment requires lvalue\0" as *const u8 as *const libc::c_char);
            } else {
                evalerror(b"++: assignment requires lvalue\0" as *const u8 as *const libc::c_char);
            }
        } else if (c as libc::c_int == '-' as i32 || c as libc::c_int == '+' as i32)
            && c1 as libc::c_int == c as libc::c_int
        {
            xp = cp;
            while !xp.is_null()
                && *xp as libc::c_int != 0
                && (*xp as libc::c_int == ' ' as i32
                    || *xp as libc::c_int == '\t' as i32
                    || *xp as libc::c_int == '\n' as i32)
            {
                xp = xp.offset(1);
            }
            if 1 as libc::c_int != 0
                && *(*__ctype_b_loc()).offset(*xp as libc::c_uchar as libc::c_int as isize)
                    as libc::c_int
                    & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                    != 0
                || *xp as libc::c_uchar as libc::c_int == '_' as i32
            {
                c = (if c as libc::c_int == '-' as i32 {
                    PREDEC as libc::c_int
                } else {
                    PREINC as libc::c_int
                }) as libc::c_uchar;
            } else {
                cp = cp.offset(-1);
            }
        } else if c1 as libc::c_int == EQ as i32
            && (if c as libc::c_int != 0 {
                (mbschr(
                    b"*/%+-&^|\0" as *const u8 as *const libc::c_char,
                    c as libc::c_int,
                ) != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int
            } else {
                0 as libc::c_int
            }) != 0
        {
            assigntok = c as libc::c_int;
            c = OP_ASSIGN as libc::c_int as libc::c_uchar;
        } else if _is_arithop(c as libc::c_int) == 0 as libc::c_int {
            cp = cp.offset(-1);
            if curtok == 0 as libc::c_int || _is_arithop(curtok) != 0 || _is_multiop(curtok) != 0 {
                evalerror(dcgettext(
                    0 as *const libc::c_char,
                    b"syntax error: operand expected\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ));
            } else {
                evalerror(dcgettext(
                    0 as *const libc::c_char,
                    b"syntax error: invalid arithmetic operator\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ));
            }
        } else {
            cp = cp.offset(-1);
        }
        lasttok = curtok;
        curtok = c as libc::c_int;
    }
    tp = cp;
}

#[no_mangle]
unsafe extern "C" fn subexpr(mut expr: *mut libc::c_char) -> intmax_t {
    let mut val: intmax_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = expr;
    while !p.is_null()
        && *p as libc::c_int != 0
        && (*p as libc::c_int == ' ' as i32
            || *p as libc::c_int == '\t' as i32
            || *p as libc::c_int == '\n' as i32)
    {
        p = p.offset(1);
    }
    if p.is_null() || *p as libc::c_int == '\u{0}' as i32 {
        return 0 as libc::c_int as intmax_t;
    }
    pushexp();
    expression = strcpy(
        sh_xmalloc(
            (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(expr)),
            b"../expr.c\0" as *const u8 as *const libc::c_char,
            463 as libc::c_int,
        ) as *mut libc::c_char,
        expr,
    );
    tp = expression;
    lasttok = 0 as libc::c_int;
    curtok = lasttok;
    tokstr = 0 as *mut libc::c_void as *mut libc::c_char;
    tokval = 0 as libc::c_int as intmax_t;
    init_lvalue(&mut curlval);
    lastlval = curlval;
    readtok();
    val = expcomma();
    if curtok != 0 as libc::c_int {
        evalerror(dcgettext(
            0 as *const libc::c_char,
            b"syntax error in expression\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ));
    }
    if !tokstr.is_null() {
        sh_xfree(
            tokstr as *mut libc::c_void,
            b"../expr.c\0" as *const u8 as *const libc::c_char,
            479 as libc::c_int,
        );
    }
    if !expression.is_null() {
        sh_xfree(
            expression as *mut libc::c_void,
            b"../expr.c\0" as *const u8 as *const libc::c_char,
            480 as libc::c_int,
        );
    }
    popexp();
    return val;
}


#[no_mangle]
unsafe extern "C" fn expr_unwind() {
    loop {
        expr_depth -= 1;
        if !(expr_depth > 0 as libc::c_int) {
            break;
        }
        if !((**expr_stack.offset(expr_depth as isize)).tokstr).is_null() {
            sh_xfree(
                (**expr_stack.offset(expr_depth as isize)).tokstr as *mut libc::c_void,
                b"../expr.c\0" as *const u8 as *const libc::c_char,
                308 as libc::c_int,
            );
        }
        if !((**expr_stack.offset(expr_depth as isize)).expression).is_null() {
            sh_xfree(
                (**expr_stack.offset(expr_depth as isize)).expression as *mut libc::c_void,
                b"../expr.c\0" as *const u8 as *const libc::c_char,
                311 as libc::c_int,
            );
        }
        sh_xfree(
            *expr_stack.offset(expr_depth as isize) as *mut libc::c_void,
            b"../expr.c\0" as *const u8 as *const libc::c_char,
            313 as libc::c_int,
        );
    }
    if expr_depth == 0 as libc::c_int {
        sh_xfree(
            *expr_stack.offset(expr_depth as isize) as *mut libc::c_void,
            b"../expr.c\0" as *const u8 as *const libc::c_char,
            316 as libc::c_int,
        );
    }
    noeval = 0 as libc::c_int;
}