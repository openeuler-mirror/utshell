#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use r_shell::savestring;
use libc::*;
extern "C" {
    fn nl_langinfo(__item: libc::c_int) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn localeconv() -> *mut lconv;
    fn __ctype_get_mb_cur_max() -> libc::size_t;
    fn free(_: *mut libc::c_void);
    fn mblen(__s: *const libc::c_char, __n: libc::size_t) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    static mut sh_syntaxtab: [libc::c_int; 0];
    static mut sh_syntabsiz: libc::c_int;
    fn internal_warning(_: *const libc::c_char, _: ...);
    fn get_string_value(_: *const libc::c_char) -> *mut libc::c_char;
    fn maybe_make_export_env();
    fn u32reset();
    fn mbschr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn yy_input_name() -> *mut libc::c_char;
    static mut dump_translatable_strings: libc::c_int;
    static mut dump_po_strings: libc::c_int;
}
use libc::{size_t,malloc};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct lconv {
    pub decimal_point: *mut libc::c_char,
    pub thousands_sep: *mut libc::c_char,
    pub grouping: *mut libc::c_char,
    pub int_curr_symbol: *mut libc::c_char,
    pub currency_symbol: *mut libc::c_char,
    pub mon_decimal_point: *mut libc::c_char,
    pub mon_thousands_sep: *mut libc::c_char,
    pub mon_grouping: *mut libc::c_char,
    pub positive_sign: *mut libc::c_char,
    pub negative_sign: *mut libc::c_char,
    pub int_frac_digits: libc::c_char,
    pub frac_digits: libc::c_char,
    pub p_cs_precedes: libc::c_char,
    pub p_sep_by_space: libc::c_char,
    pub n_cs_precedes: libc::c_char,
    pub n_sep_by_space: libc::c_char,
    pub p_sign_posn: libc::c_char,
    pub n_sign_posn: libc::c_char,
    pub int_p_cs_precedes: libc::c_char,
    pub int_p_sep_by_space: libc::c_char,
    pub int_n_cs_precedes: libc::c_char,
    pub int_n_sep_by_space: libc::c_char,
    pub int_p_sign_posn: libc::c_char,
    pub int_n_sign_posn: libc::c_char,
}
pub const _ISblank: libc::c_uint = 1;

#[no_mangle]
pub static mut locale_utf8locale: libc::c_int = 0;
#[no_mangle]
pub static mut locale_mb_cur_max: libc::c_int = 0;
#[no_mangle]
pub static mut locale_shiftstates: libc::c_int = 0 as libc::c_int;
static mut default_locale: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut default_domain: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut default_dir: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut lc_all: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut lang: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;

pub const LC_ALL          : libc::c_int = 6;
pub const STR_LC_ALL      : *const libc::c_char = b"LC_ALL\0" as *const u8 as *const libc::c_char;
pub const STR_LC_CTYPE    : *const libc::c_char = b"LC_CTYPE\0" as *const u8 as *const libc::c_char;
pub const STR_LC_COLLATE  : *const libc::c_char = b"LC_COLLATE\0" as *const u8 as *const libc::c_char;
pub const STR_LC_MESSAGES : *const libc::c_char = b"LC_MESSAGES\0" as *const u8 as *const libc::c_char;
pub const STR_LC_NUMERIC  : *const libc::c_char = b"LC_NUMERIC\0" as *const u8 as *const libc::c_char;
pub const STR_LC_TIME     : *const libc::c_char = b"LC_TIME\0" as *const u8 as *const libc::c_char;

pub const STR_TEXTDOMAIN     : *const libc::c_char = b"TEXTDOMAIN\0" as *const u8 as *const libc::c_char;
pub const STR_TEXTDOMAINDIR     : *const libc::c_char = b"TEXTDOMAINDIR\0" as *const u8 as *const libc::c_char;

pub const STR_PACKAGE : *const libc::c_char = b"utshell\0" as *const u8 as *const libc::c_char;
pub const STR_LOCALEDIR : *const libc::c_char = b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char;

#[no_mangle]
pub unsafe extern "C" fn set_default_locale() {
    default_locale = setlocale(
        LC_ALL,
        b"\0" as *const u8 as *const libc::c_char,
    );
    bindtextdomain(
        STR_PACKAGE,
        STR_LOCALEDIR
    );
    textdomain(STR_PACKAGE);
    locale_mb_cur_max = __ctype_get_mb_cur_max() as libc::c_int;
    locale_utf8locale = locale_isutf8(default_locale);
    locale_shiftstates = mblen(
        0 as *mut libc::c_void as *mut libc::c_char,
        0 as libc::c_int as libc::size_t,
    );
}

#[no_mangle]
pub unsafe extern "C" fn set_default_locale_vars() {
    let mut val: *mut libc::c_char;
    val = get_string_value(STR_LC_CTYPE);
    if val.is_null() && !lc_all.is_null() && *lc_all as libc::c_int != 0 {
        setlocale(0 as libc::c_int, lc_all);
        locale_setblanks();
        locale_mb_cur_max = __ctype_get_mb_cur_max() as libc::c_int;
        locale_utf8locale = locale_isutf8(lc_all);
        locale_shiftstates = mblen(
            0 as *mut libc::c_void as *mut libc::c_char,
            0 as libc::c_int as libc::size_t,
        );
        u32reset();
    }
    val = get_string_value(STR_LC_COLLATE);
    if val.is_null() && !lc_all.is_null() && *lc_all as libc::c_int != 0 {
        setlocale(3 as libc::c_int, lc_all);
    }
    val = get_string_value(STR_LC_MESSAGES);
    if val.is_null() && !lc_all.is_null() && *lc_all as libc::c_int != 0 {
        setlocale(5 as libc::c_int, lc_all);
    }
    val = get_string_value(STR_LC_NUMERIC);
    if val.is_null() && !lc_all.is_null() && *lc_all as libc::c_int != 0 {
        setlocale(1 as libc::c_int, lc_all);
    }
    val = get_string_value(STR_LC_TIME);
    if val.is_null() && !lc_all.is_null() && *lc_all as libc::c_int != 0 {
        setlocale(2 as libc::c_int, lc_all);
    }
    val = get_string_value(STR_TEXTDOMAIN);
    if !val.is_null() && *val as libc::c_int != 0 {
        if !default_domain.is_null() {
            free(default_domain as *mut libc::c_void);
        }
        default_domain = 0 as *mut libc::c_char;
        default_domain = r_shell::savestring!(val);
        if !default_dir.is_null() && *default_dir as libc::c_int != 0 {
            bindtextdomain(default_domain, default_dir);
        }
    }
    val = get_string_value(STR_TEXTDOMAINDIR);
    if !val.is_null() && *val as libc::c_int != 0 {
        if !default_dir.is_null() {
            free(default_dir as *mut libc::c_void);
        }
        default_dir = 0 as *mut libc::c_char;
        default_dir = savestring!(val);
        if !default_domain.is_null() && *default_domain as libc::c_int != 0 {
            bindtextdomain(default_domain, default_dir);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn set_locale_var(
    var: *mut libc::c_char,
    value: *mut libc::c_char,
) -> libc::c_int {
    let r: libc::c_int;
    let mut x: *mut libc::c_char;
    x = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    *__errno_location() = 0 as libc::c_int;
    if *var.offset(0 as libc::c_int as isize) as libc::c_int == 'T' as i32
        && *var.offset(10 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
    {
        if !default_domain.is_null() {
            free(default_domain as *mut libc::c_void);
        }
        default_domain = 0 as *mut libc::c_char;
        default_domain = if !value.is_null() {
            savestring!(value)
        } else {
            0 as *mut libc::c_void as *mut libc::c_char
        };
        if !default_dir.is_null() && *default_dir as libc::c_int != 0 {
            bindtextdomain(default_domain, default_dir);
        }
        return 1 as libc::c_int;
    } else if *var.offset(0 as libc::c_int as isize) as libc::c_int == 'T' as i32 {
        if !default_dir.is_null() {
            free(default_dir as *mut libc::c_void);
        }
        default_dir = 0 as *mut libc::c_char;
        default_dir = if !value.is_null() {
            savestring!(value)
        } else {
            0 as *mut libc::c_void as *mut libc::c_char
        };
        if !default_domain.is_null() && *default_domain as libc::c_int != 0 {
            bindtextdomain(default_domain, default_dir);
        }
        return 1 as libc::c_int;
    } else if *var.offset(3 as libc::c_int as isize) as libc::c_int == 'A' as i32 {
        if !lc_all.is_null() {
            free(lc_all as *mut libc::c_void);
        }
        lc_all = 0 as *mut libc::c_char;
        if !value.is_null() {
            lc_all = savestring!(value);
        } else {
            lc_all = malloc(1 as libc::c_int as libc::size_t) as *mut libc::c_char;
            *lc_all.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        }
        r = if *lc_all as libc::c_int != 0 {
            x = setlocale(6 as libc::c_int, lc_all);
            (x != 0 as *mut libc::c_char) as libc::c_int
        } else {
            reset_locale_vars()
        };
        if x.is_null() {
            if *__errno_location() == 0 as libc::c_int {
                internal_warning(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"setlocale: LC_ALL: cannot change locale (%s)\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    lc_all,
                );
            } else {
                internal_warning(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"setlocale: LC_ALL: cannot change locale (%s): %s\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    lc_all,
                    strerror(*__errno_location()),
                );
            }
        }
        locale_setblanks();
        locale_mb_cur_max = __ctype_get_mb_cur_max() as libc::c_int;
        if *lc_all as libc::c_int != 0 && !x.is_null() {
            locale_utf8locale = locale_isutf8(lc_all);
        }
        locale_shiftstates = mblen(
            0 as *mut libc::c_void as *mut libc::c_char,
            0 as libc::c_int as libc::size_t,
        );
        u32reset();
        return r;
    } else if *var.offset(3 as libc::c_int as isize) as libc::c_int == 'C' as i32
        && *var.offset(4 as libc::c_int as isize) as libc::c_int == 'T' as i32
    { /* LC_CTYPE */
        if lc_all.is_null() || *lc_all as libc::c_int == '\0' as i32 {
            x = setlocale(
                0 as libc::c_int,
                get_locale_var(
                    STR_LC_CTYPE as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
            );
            locale_setblanks();
            locale_mb_cur_max = __ctype_get_mb_cur_max() as libc::c_int;
            if !x.is_null() {
                locale_utf8locale = locale_isutf8(x);
            }
            locale_shiftstates = mblen(
                0 as *mut libc::c_void as *mut libc::c_char,
                0 as libc::c_int as size_t,
            );
            u32reset();
        }
    } else if *var.offset(3 as libc::c_int as isize) as libc::c_int == 'C' as i32
        && *var.offset(4 as libc::c_int as isize) as libc::c_int == 'O' as i32
    { /* LC_COLLATE */
        if lc_all.is_null() || *lc_all as libc::c_int == '\0' as i32 {
            x = setlocale(
                3 as libc::c_int,
                get_locale_var(
                    b"LC_COLLATE\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
            );
        }
    } else if *var.offset(3 as libc::c_int as isize) as libc::c_int == 'M' as i32
        && *var.offset(4 as libc::c_int as isize) as libc::c_int == 'E' as i32
    { /* LC_MESSAGES */
        if lc_all.is_null() || *lc_all as libc::c_int == '\0' as i32 {
            x = setlocale(
                5 as libc::c_int,
                get_locale_var(
                    b"LC_MESSAGES\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
            );
        }
    } else if *var.offset(3 as libc::c_int as isize) as libc::c_int == 'N' as i32
        && *var.offset(4 as libc::c_int as isize) as libc::c_int == 'U' as i32
    { /* LC_NUMERIC */
        if lc_all.is_null() || *lc_all as libc::c_int == '\0' as i32 {
            x = setlocale(
                1 as libc::c_int,
                get_locale_var(
                    b"LC_NUMERIC\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
            );
        }
    } else if *var.offset(3 as libc::c_int as isize) as libc::c_int == 'T' as i32
        && *var.offset(4 as libc::c_int as isize) as libc::c_int == 'I' as i32
    {/* LC_TIME */

        if lc_all.is_null() || *lc_all as libc::c_int == '\0' as i32 {
            x = setlocale(
                2 as libc::c_int,
                get_locale_var(
                    b"LC_TIME\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ),
            );
        }
    }
    if x.is_null() {
        if *__errno_location() == 0 as libc::c_int {
            internal_warning(
                dcgettext(
                    0 as *const libc::c_char,
                    b"setlocale: %s: cannot change locale (%s)\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                var,
                get_locale_var(var),
            );
        } else {
            internal_warning(
                dcgettext(
                    0 as *const libc::c_char,
                    b"setlocale: %s: cannot change locale (%s): %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                var,
                get_locale_var(var),
                strerror(*__errno_location()),
            );
        }
    }
    return (x != 0 as *mut libc::c_char) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn set_lang(
    mut _var: *mut libc::c_char,
    value: *mut libc::c_char,
) -> libc::c_int {
    if !lang.is_null() {
        free(lang as *mut libc::c_void);
    }
    lang = 0 as *mut libc::c_char;
    if !value.is_null() {
        lang = savestring!(value);
    } else {
        lang = malloc(1 as libc::c_int as libc::size_t) as *mut libc::c_char;
        *lang.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    }
    return if lc_all.is_null() || *lc_all as libc::c_int == 0 as libc::c_int {
        reset_locale_vars()
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn set_default_lang() {
    let mut v: *mut libc::c_char;
    v = get_string_value(b"LC_ALL\0" as *const u8 as *const libc::c_char);
    set_locale_var(
        b"LC_ALL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        v,
    );
    v = get_string_value(b"LANG\0" as *const u8 as *const libc::c_char);
    set_lang(b"LANG\0" as *const u8 as *const libc::c_char as *mut libc::c_char, v);
}
#[no_mangle]
pub unsafe extern "C" fn get_locale_var(
    var: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut locale: *mut libc::c_char;
    locale = lc_all;
    if locale.is_null() || *locale as libc::c_int == 0 as libc::c_int {
        locale = get_string_value(var);
    }
    if locale.is_null() || *locale as libc::c_int == 0 as libc::c_int {
        locale = lang;
    }
    if locale.is_null() || *locale as libc::c_int == 0 as libc::c_int {
        locale = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    return locale;
}
unsafe extern "C" fn reset_locale_vars() -> libc::c_int {
    let x: *mut libc::c_char;
    if lang.is_null() || *lang as libc::c_int == '\0' as i32 {
        maybe_make_export_env();
    }
    if (setlocale(
        6 as libc::c_int,
        if !lang.is_null() {
            lang as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    ))
        .is_null()
    {
        return 0 as libc::c_int;
    }
    // x = 0 as *mut libc::c_char;
    x = setlocale(
        0 as libc::c_int,
        get_locale_var(
            b"LC_CTYPE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    );
    setlocale(
        3 as libc::c_int,
        get_locale_var(
            b"LC_COLLATE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    );
    setlocale(
        5 as libc::c_int,
        get_locale_var(
            b"LC_MESSAGES\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    );
    setlocale(
        1 as libc::c_int,
        get_locale_var(
            b"LC_NUMERIC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    );
    setlocale(
        2 as libc::c_int,
        get_locale_var(
            b"LC_TIME\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    );
    locale_setblanks();
    locale_mb_cur_max = __ctype_get_mb_cur_max() as libc::c_int;
    if !x.is_null() {
        locale_utf8locale = locale_isutf8(x);
    }
    locale_shiftstates = mblen(
        0 as *mut libc::c_void as *mut libc::c_char,
        0 as libc::c_int as libc::size_t,
    );
    u32reset();
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn localetrans(
    string: *mut libc::c_char,
    len: libc::c_int,
    lenp: *mut libc::c_int,
) -> *mut libc::c_char {
    let locale: *mut libc::c_char;
    let t: *mut libc::c_char;
    let translated: *mut libc::c_char;
    let tlen: libc::c_int;
    if string.is_null() || *string as libc::c_int == 0 as libc::c_int {
        if !lenp.is_null() {
            *lenp = 0 as libc::c_int;
        }
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    locale = get_locale_var(
        b"LC_MESSAGES\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if locale.is_null()
        || *locale.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
        || *locale.offset(0 as libc::c_int as isize) as libc::c_int == 'C' as i32
            && *locale.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
        || *locale.offset(0 as libc::c_int as isize) as libc::c_int
            == (*::core::mem::transmute::<
                &[u8; 6],
                &[libc::c_char; 6],
            >(b"POSIX\0"))[0 as libc::c_int as usize] as libc::c_int
            && strcmp(locale, b"POSIX\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
    {
        t = malloc((len + 1 as libc::c_int) as libc::size_t) as *mut libc::c_char;
        strcpy(t, string);
        if !lenp.is_null() {
            *lenp = len;
        }
        return t;
    }
    if !default_domain.is_null() && *default_domain as libc::c_int != 0 {
        translated = dcgettext(default_domain, string, 5 as libc::c_int);
    } else {
        translated = string;
    }
    if translated == string {
        t = malloc((len + 1 as libc::c_int) as libc::size_t) as *mut libc::c_char;
        strcpy(t, string);
        if !lenp.is_null() {
            *lenp = len;
        }
    } else {
        tlen = strlen(translated) as libc::c_int;
        t = malloc((tlen + 1 as libc::c_int) as libc::size_t) as *mut libc::c_char;
        strcpy(t, translated);
        if !lenp.is_null() {
            *lenp = tlen;
        }
    }
    return t;
}

#[no_mangle]
pub unsafe extern "C" fn mk_msgstr(
    string: *mut libc::c_char,
    foundnlp: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut c: libc::c_int = 0;
    let mut len: libc::c_int;
    let result: *mut libc::c_char;
    let mut r: *mut libc::c_char;
    let mut s: *mut libc::c_char;
    len = 0 as libc::c_int;
    s = string;
    while !s.is_null() && *s as libc::c_int != 0 {
        len += 1;
        if *s as libc::c_int == '"' as i32 || *s as libc::c_int == '\\' as i32 {
            len += 1;
        } else if *s as libc::c_int == '\n' as i32 {
            len += 5 as libc::c_int;
        }
        s = s.offset(1);
    }
    result = malloc((len + 3 as libc::c_int) as libc::size_t) as *mut libc::c_char;
    r = result;
    let fresh0 = r;
    r = r.offset(1);
    *fresh0 = '"' as i32 as libc::c_char;
    s = string;
    while !s.is_null()
        && {
            c = *s as libc::c_int;
            c != 0
        }
    {
        if c == '\n' as i32 {
            let fresh1 = r;
            r = r.offset(1);
            *fresh1 = '\\' as i32 as libc::c_char;
            let fresh2 = r;
            r = r.offset(1);
            *fresh2 = 'n' as i32 as libc::c_char;
            let fresh3 = r;
            r = r.offset(1);
            *fresh3 = '"' as i32 as libc::c_char;
            let fresh4 = r;
            r = r.offset(1);
            *fresh4 = '\n' as i32 as libc::c_char;
            let fresh5 = r;
            r = r.offset(1);
            *fresh5 = '"' as i32 as libc::c_char;
            if !foundnlp.is_null() {
                *foundnlp = 1 as libc::c_int;
            }
        } else {
            if c == '"' as i32 || c == '\\' as i32 {
                let fresh6 = r;
                r = r.offset(1);
                *fresh6 = '\\' as i32 as libc::c_char;
            }
            let fresh7 = r;
            r = r.offset(1);
            *fresh7 = c as libc::c_char;
        }
        s = s.offset(1);
    }
    let fresh8 = r;
    r = r.offset(1);
    *fresh8 = '"' as i32 as libc::c_char;
    let fresh9 = r;
    r = r.offset(1);
    *fresh9 = '\0' as i32 as libc::c_char;
    return result;
}

#[no_mangle]
pub unsafe extern "C" fn localeexpand(
    string: *mut libc::c_char,
    start: libc::c_int,
    end: libc::c_int,
    lineno: libc::c_int,
    lenp: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut len: libc::c_int;
    let mut tlen: libc::c_int;
    let mut foundnl: libc::c_int;
    let temp: *mut libc::c_char;
    let t: *mut libc::c_char;
    let t2: *mut libc::c_char;
    temp = malloc((end - start + 1 as libc::c_int) as libc::size_t) as *mut libc::c_char;
    tlen = 0 as libc::c_int;
    len = start;
    while len < end {
        let fresh10 = len;
        len = len + 1;
        let fresh11 = tlen;
        tlen = tlen + 1;
        *temp.offset(fresh11 as isize) = *string.offset(fresh10 as isize);
    }
    *temp.offset(tlen as isize) = '\0' as i32 as libc::c_char;
    if dump_translatable_strings != 0 {
        if dump_po_strings != 0 {
            foundnl = 0 as libc::c_int;
            t = mk_msgstr(temp, &mut foundnl);
            t2 = (if foundnl != 0 {
                b"\"\"\n\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char;
            printf(
                b"#: %s:%d\nmsgid %s%s\nmsgstr \"\"\n\0" as *const u8
                    as *const libc::c_char,
                yy_input_name(),
                lineno,
                t2,
                t,
            );
            free(t as *mut libc::c_void);
        } else {
            printf(b"\"%s\"\n\0" as *const u8 as *const libc::c_char, temp);
        }
        if !lenp.is_null() {
            *lenp = tlen;
        }
        return temp;
    } else if *temp != 0 {
        t = localetrans(temp, tlen, &mut len);
        free(temp as *mut libc::c_void);
        if !lenp.is_null() {
            *lenp = len;
        }
        return t;
    } else {
        if !lenp.is_null() {
            *lenp = 0 as libc::c_int;
        }
        return temp;
    };
}

unsafe extern "C" fn locale_setblanks() {
    let mut x: libc::c_int;
    x = 0 as libc::c_int;
    while x < sh_syntabsiz {
        if *(*__ctype_b_loc()).offset(x as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISblank as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            *sh_syntaxtab.as_mut_ptr().offset(x as isize)
                |= 0x2 as libc::c_int | 0x2000 as libc::c_int;
        } else if if x != 0 {
            (mbschr(b"()<>;&| \t\n\0" as *const u8 as *const libc::c_char, x)
                != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            *sh_syntaxtab.as_mut_ptr().offset(x as isize) |= 0x2 as libc::c_int;
            *sh_syntaxtab.as_mut_ptr().offset(x as isize) &= !(0x2000 as libc::c_int);
        } else {
            *sh_syntaxtab.as_mut_ptr().offset(x as isize)
                &= !(0x2 as libc::c_int | 0x2000 as libc::c_int);
        }
        x += 1;
    }
}

unsafe extern "C" fn locale_isutf8(mut _lspec: *mut libc::c_char) -> libc::c_int {
    let cp: *mut libc::c_char;
    let mut _encoding: *mut libc::c_char = 0 as *mut libc::c_char;
    cp = nl_langinfo(libc::CODESET as libc::c_int);
    return (*cp.offset(0 as libc::c_int as isize) as libc::c_int
        == (*::core::mem::transmute::<
            &[u8; 6],
            &[libc::c_char; 6],
        >(b"UTF-8\0"))[0 as libc::c_int as usize] as libc::c_int
        && strcmp(cp, b"UTF-8\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || *cp.offset(0 as libc::c_int as isize) as libc::c_int
            == (*::core::mem::transmute::<
                &[u8; 5],
                &[libc::c_char; 5],
            >(b"utf8\0"))[0 as libc::c_int as usize] as libc::c_int
            && strcmp(cp, b"utf8\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int) as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn locale_decpoint() -> libc::c_int {
    // let lv: *mut lconv;
    let lv: *const lconv;
    lv = localeconv();
    return if !lv.is_null() && !((*lv).decimal_point).is_null()
        && *((*lv).decimal_point).offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        *((*lv).decimal_point).offset(0 as libc::c_int as isize) as libc::c_int
    } else {
        '.' as i32
    };
}
