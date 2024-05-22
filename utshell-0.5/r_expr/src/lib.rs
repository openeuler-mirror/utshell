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