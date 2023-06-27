use std::ffi::CStr;
use rcommon::r_sh_notfound;

extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn legal_alias_name(_: *const libc::c_char, _: libc::c_int) -> libc::c_int;
    fn sh_single_quote(_: *const libc::c_char) -> *mut libc::c_char;
    static mut posixly_correct: libc::c_int;
    static mut aliases: *mut HASH_TABLE;
    fn find_alias(_: *mut libc::c_char) -> *mut alias_t;
    fn add_alias(_: *mut libc::c_char, _: *mut libc::c_char);
    fn remove_alias(_: *mut libc::c_char) -> libc::c_int;
    fn delete_all_aliases();
    fn all_aliases() -> *mut *mut alias_t;
    fn builtin_error(_: *const libc::c_char, _: ...);
    fn builtin_usage();
    fn sh_notfound(_: *mut libc::c_char);
    fn sh_chkwrite(_: libc::c_int) -> libc::c_int;
    fn builtin_help();
    static mut loptend: *mut WORD_LIST;
    fn internal_getopt(_: *mut WORD_LIST, _: *mut libc::c_char) -> libc::c_int;
    fn reset_internal_getopt();
}
pub type size_t = libc::c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct word_desc {
    pub word: *mut libc::c_char,
    pub flags: libc::c_int,
}
pub type WordDesc = word_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct word_list {
    pub next: *mut word_list,
    pub word: *mut WordDesc,
}
pub type WORD_LIST = word_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bucket_contents {
    pub next: *mut bucket_contents,
    pub key: *mut libc::c_char,
    pub data: *mut libc::c_void,
    pub khash: libc::c_uint,
    pub times_found: libc::c_int,
}
pub type BUCKET_CONTENTS = bucket_contents;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table {
    pub bucket_array: *mut *mut BUCKET_CONTENTS,
    pub nbuckets: libc::c_int,
    pub nentries: libc::c_int,
}
pub type HASH_TABLE = hash_table;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alias {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub flags: libc::c_char,
}
pub type alias_t = alias;

pub static AL_REUSABLE:i32 = 0x01;
pub static EX_USAGE:i32 = 258;
//extern crate rcommon;
use rcommon::EXECUTION_SUCCESS;
use rcommon::r_builtin_usage;


#[no_mangle]
pub unsafe extern "C" fn r_alias_builtin(mut list: *mut WORD_LIST) -> libc::c_int {
    println!("alias_builtin run!");
    let mut any_failed = 0;
    let mut offset = 0;
    let mut pflag = 0;
    let mut dflags = 0;
    let mut alias_list: *mut *mut alias_t;
    let mut t: *mut alias_t;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    dflags = if posixly_correct != 0 { 0 as libc::c_int } else { 0x1 as libc::c_int };
    pflag = 0 as libc::c_int;
    reset_internal_getopt();
    loop {
        offset = internal_getopt(
            list,
            b"p\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !(offset != -(1 as libc::c_int)) {
            break;
        }
        match offset as u8 {
            b'p' => {
                pflag = 1 ;
                dflags |= AL_REUSABLE;
            }
            _ => {
                if offset == -99 {
                    builtin_help();
                    return EX_USAGE;
                }
                r_builtin_usage();
                return EX_USAGE;
            }
        }
    }
    list = loptend;
    if list.is_null() || pflag != 0 {
        if aliases.is_null() {
            return EXECUTION_SUCCESS;
        }
        alias_list = all_aliases();
        if alias_list.is_null() {
            return EXECUTION_SUCCESS;
        }
        offset = 0;
        while !(*alias_list.offset(offset as isize)).is_null() {
            print_alias(*alias_list.offset(offset as isize), dflags);
            offset += 1;
        }
        free(alias_list as *mut libc::c_void);
        if list.is_null() {
            return sh_chkwrite(EXECUTION_SUCCESS);
        }
    }
    any_failed = 0;
    while !list.is_null() {
        name = (*(*list).word).word;
        offset = 0;
        while *name.offset(offset as isize) as libc::c_int != 0
            && *name.offset(offset as isize) as libc::c_int != '=' as i32
        {
            offset += 1;
        }
        if offset != 0 && *name.offset(offset as isize) as libc::c_int == '=' as i32 {
            *name.offset(offset as isize) = '\u{0}' as i32 as libc::c_char;
            value = name.offset(offset as isize).offset(1 as libc::c_int as isize);
            if legal_alias_name(name, 0) == 0 {
                builtin_error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"`%s': invalid alias name\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    name,
                );
                any_failed += 1;
            } else {
                add_alias(name, value);
            }
        } else {
            t = find_alias(name);
            if !t.is_null() {
                print_alias(t, dflags);
            } else {
                sh_notfound(name);
                any_failed += 1;
            }
        }
        list = (*list).next;
    }
    return if any_failed != 0 { rcommon::EXECUTION_FAILURE!()} else { EXECUTION_SUCCESS};
}
#[no_mangle]
pub unsafe extern "C" fn r_unalias_builtin(mut list: *mut WORD_LIST) -> libc::c_int {
    let mut alias: *mut alias_t = 0 as *mut alias_t;
    let mut opt: libc::c_int = 0;
    let mut aflag: libc::c_int = 0;
    aflag = 0 as libc::c_int;
    reset_internal_getopt();
    loop {
        opt = internal_getopt(
            list,
            b"a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !(opt != -(1 as libc::c_int)) {
            break;
        }
        match opt as u8{
            b'a' => {
                aflag = 1 as libc::c_int;
            }
            _ => {
                if opt == -99 {
                    builtin_help();
                    return EX_USAGE;
                }
                builtin_usage();
                return EX_USAGE;
            }
        }
    }
    list = loptend;
    if aflag != 0 {
        delete_all_aliases();
        return 0;
    }
    if list.is_null() {
        builtin_usage();
        return EX_USAGE;
    }
    aflag = 0 as libc::c_int;
    while !list.is_null() {
        alias = find_alias((*(*list).word).word);
        if !alias.is_null() {
            remove_alias((*alias).name);
        } else {
            sh_notfound((*(*list).word).word);
            aflag += 1;
        }
        list = (*list).next;
    }
    return if aflag != 0 { 1 as libc::c_int } else { 0 as libc::c_int };
}
unsafe extern "C" fn print_alias(mut alias: *mut alias_t, mut flags: libc::c_int) {
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    value = sh_single_quote((*alias).value);
    if flags & 0x1 as libc::c_int != 0 {
        printf(
            b"alias %s\0" as *const u8 as *const libc::c_char,
            if !((*alias).name).is_null()
                && *((*alias).name).offset(0 as libc::c_int as isize) as libc::c_int
                    == '-' as i32
            {
                b"-- \0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
    }

    println!("{}={}", CStr::from_ptr((*alias).name).to_string_lossy().into_owned(), CStr::from_ptr(value).to_string_lossy().into_owned());
    free(value as *mut libc::c_void);
}
