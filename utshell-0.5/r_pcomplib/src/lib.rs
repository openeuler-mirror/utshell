//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;

    fn programming_error(_: *const libc::c_char, _: ...);
    fn hash_create(_: libc::c_int) -> *mut HASH_TABLE;
    fn hash_flush(_: *mut HASH_TABLE, _: Option::<sh_free_func_t>);
    fn hash_dispose(_: *mut HASH_TABLE);
    fn hash_walk(_: *mut HASH_TABLE, _: Option::<hash_wfunc>);
    fn hash_search(
        _: *const libc::c_char,
        _: *mut HASH_TABLE,
        _: libc::c_int,
    ) -> *mut BUCKET_CONTENTS;
    fn hash_insert(
        _: *mut libc::c_char,
        _: *mut HASH_TABLE,
        _: libc::c_int,
    ) -> *mut BUCKET_CONTENTS;
    fn hash_remove(
        _: *const libc::c_char,
        _: *mut HASH_TABLE,
        _: libc::c_int,
    ) -> *mut BUCKET_CONTENTS;
}
pub type size_t = libc::c_ulong;
pub type sh_free_func_t = unsafe extern "C" fn(*mut libc::c_void) -> ();
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
pub type hash_wfunc = unsafe extern "C" fn(*mut BUCKET_CONTENTS) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct compspec {
    pub refcount: libc::c_int,
    pub actions: libc::c_ulong,
    pub options: libc::c_ulong,
    pub globpat: *mut libc::c_char,
    pub words: *mut libc::c_char,
    pub prefix: *mut libc::c_char,
    pub suffix: *mut libc::c_char,
    pub funcname: *mut libc::c_char,
    pub command: *mut libc::c_char,
    pub lcommand: *mut libc::c_char,
    pub filterpat: *mut libc::c_char,
}
pub type COMPSPEC = compspec;

#[no_mangle]
pub static mut prog_completes: *mut HASH_TABLE = 0 as *const libc::c_void
    as *mut libc::c_void as *mut HASH_TABLE;

#[no_mangle]
pub unsafe extern "C" fn compspec_create() -> *mut COMPSPEC {
    let ret: *mut COMPSPEC = libc::malloc(
        ::core::mem::size_of::<COMPSPEC>() as usize,
    ) as *mut COMPSPEC;
    (*ret).refcount = 0 as libc::c_int;
    (*ret).actions = 0 as libc::c_int as libc::c_ulong;
    (*ret).options = 0 as libc::c_int as libc::c_ulong;
    (*ret).globpat = 0 as *mut libc::c_void as *mut libc::c_char;
    (*ret).words = 0 as *mut libc::c_void as *mut libc::c_char;
    (*ret).prefix = 0 as *mut libc::c_void as *mut libc::c_char;
    (*ret).suffix = 0 as *mut libc::c_void as *mut libc::c_char;
    (*ret).funcname = 0 as *mut libc::c_void as *mut libc::c_char;
    (*ret).command = 0 as *mut libc::c_void as *mut libc::c_char;
    (*ret).lcommand = 0 as *mut libc::c_void as *mut libc::c_char;
    (*ret).filterpat = 0 as *mut libc::c_void as *mut libc::c_char;
    return ret;
}

#[no_mangle]
pub unsafe extern "C" fn compspec_dispose(cs: *mut COMPSPEC) {
    (*cs).refcount -= 1;
    if (*cs).refcount == 0 as libc::c_int {
        if !((*cs).globpat).is_null() {
            libc::free(
                (*cs).globpat as *mut libc::c_void,
            );
        }
        if !((*cs).words).is_null() {
            libc::free(
                (*cs).words as *mut libc::c_void,
            );
        }
        if !((*cs).prefix).is_null() {
            libc::free(
                (*cs).prefix as *mut libc::c_void,
            );
        }
        if !((*cs).suffix).is_null() {
            libc::free(
                (*cs).suffix as *mut libc::c_void,
            );
        }
        if !((*cs).funcname).is_null() {
            libc::free(
                (*cs).funcname as *mut libc::c_void,
            );
        }
        if !((*cs).command).is_null() {
            libc::free(
                (*cs).command as *mut libc::c_void,
            );
        }
        if !((*cs).lcommand).is_null() {
            libc::free(
                (*cs).lcommand as *mut libc::c_void,
            );
        }
        if !((*cs).filterpat).is_null() {
            libc::free(
                (*cs).filterpat as *mut libc::c_void,
            );
        }
        libc::free(
            cs as *mut libc::c_void,
        );
    }
}
#[macro_export]
macro_rules! STRDUP {
    ($x:expr) => {
        if !($x).is_null(){
r_shell::savestring!($x)
        }else{
0 as *mut libc::c_char
        }
        
    };
}

#[no_mangle]
pub unsafe extern "C" fn compspec_copy(cs: *mut COMPSPEC) -> *mut COMPSPEC {
    let new: *mut COMPSPEC = libc::malloc(
        ::core::mem::size_of::<COMPSPEC>() as usize
    ) as *mut COMPSPEC;
    (*new).refcount = 1 as libc::c_int;
    (*new).actions = (*cs).actions;
    (*new).options = (*cs).options;
    (*new)
        .globpat = STRDUP!((*cs).globpat);
    (*new)
        .words = STRDUP!((*cs).words);
    (*new)
        .prefix =  STRDUP!((*cs).prefix);
    (*new)
        .suffix = STRDUP!((*cs).suffix);
    (*new)
        .funcname = STRDUP!((*cs).funcname);
    (*new)
        .command = STRDUP!((*cs).command);
    (*new)
        .lcommand = STRDUP!((*cs).lcommand);
    (*new)
        .filterpat = STRDUP!((*cs).filterpat);
    return new;
}
pub const COMPLETE_HASH_BUCKETS:libc::c_int = 256;

#[no_mangle]
pub unsafe extern "C" fn progcomp_create() {
    if prog_completes.is_null() {
        prog_completes = hash_create(COMPLETE_HASH_BUCKETS);
    }
}

#[no_mangle]
pub unsafe extern "C" fn progcomp_size() -> libc::c_int {
    return if !prog_completes.is_null() {
        (*prog_completes).nentries
    } else {
        0 as libc::c_int
    };
}

unsafe extern "C" fn free_progcomp(data: *mut libc::c_void) {
    let cs: *mut COMPSPEC = data as *mut COMPSPEC;
    compspec_dispose(cs);
}

#[no_mangle]
pub unsafe extern "C" fn progcomp_flush() {
    if !prog_completes.is_null() {
        hash_flush(
            prog_completes,
                Some(free_progcomp),
                );
    }
}

#[no_mangle]
pub unsafe extern "C" fn progcomp_dispose() {
    if !prog_completes.is_null() {
        hash_dispose(prog_completes);
    }
    prog_completes = 0 as *mut libc::c_void as *mut HASH_TABLE;
}

#[no_mangle]
pub unsafe extern "C" fn progcomp_remove(cmd: *mut libc::c_char) -> libc::c_int {
    let item: *mut BUCKET_CONTENTS;
    if prog_completes.is_null() {
        return 1 as libc::c_int;
    }
    item = hash_remove(cmd, prog_completes, 0 as libc::c_int);
    if !item.is_null() {
        if !((*item).data).is_null() {
            free_progcomp((*item).data);
        }
        libc::free(
            (*item).key as *mut libc::c_void,
        );
        libc::free(
            item as *mut libc::c_void,

        );
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn progcomp_insert(
    cmd: *mut libc::c_char,
    cs: *mut COMPSPEC,
) -> libc::c_int {
    let item: *mut BUCKET_CONTENTS;
    if cs.is_null() {
        programming_error(
            dcgettext(
                0 as *const libc::c_char,
                b"progcomp_insert: %s: NULL COMPSPEC\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            cmd,
        );
    }
    if prog_completes.is_null() {
        progcomp_create();
    }
    (*cs).refcount += 1;
    item = hash_insert(cmd, prog_completes, 0 as libc::c_int);
    if !((*item).data).is_null() {
        free_progcomp((*item).data);
    } else {
        (*item)
            .key = libc::strcpy(
            libc::malloc(
                (1 + libc::strlen(cmd)) as usize
            ) as *mut libc::c_char,
            cmd,
        );
    }
    (*item).data = cs as *mut libc::c_void;
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn progcomp_search(cmd: *const libc::c_char) -> *mut COMPSPEC {
    let item: *mut BUCKET_CONTENTS;
    let cs: *mut COMPSPEC;
    if prog_completes.is_null() {
        return 0 as *mut libc::c_void as *mut COMPSPEC;
    }
    item = hash_search(cmd, prog_completes, 0 as libc::c_int);
    if item.is_null() {
        return 0 as *mut libc::c_void as *mut COMPSPEC;
    }
    cs = (*item).data as *mut COMPSPEC;
    return cs;
}

#[no_mangle]
pub unsafe extern "C" fn progcomp_walk(pfunc: Option::<hash_wfunc>) {
    if prog_completes.is_null() || pfunc.is_none()
        || (if !prog_completes.is_null() {
            (*prog_completes).nentries
        } else {
            0 as libc::c_int
        }) == 0 as libc::c_int
    {
        return;
    }
    hash_walk(prog_completes, pfunc);
}
