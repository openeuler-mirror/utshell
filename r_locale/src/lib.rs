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

