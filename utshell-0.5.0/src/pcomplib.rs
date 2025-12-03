use crate::hashlib::{
    hash_create, hash_dispose, hash_flush, hash_insert, hash_remove, hash_search, hash_walk,
};
use crate::src_common::*;

#[no_mangle]
pub fn compspec_create() -> *mut COMPSPEC {
    unsafe {
        let ret: *mut COMPSPEC =
            libc::malloc(::core::mem::size_of::<COMPSPEC>() as usize) as *mut COMPSPEC;
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
}

#[no_mangle]
pub fn compspec_dispose(cs: *mut COMPSPEC) {
    unsafe {
        (*cs).refcount -= 1;
        if (*cs).refcount == 0 as libc::c_int {
            if !((*cs).globpat).is_null() {
                libc::free((*cs).globpat as *mut libc::c_void);
            }
            if !((*cs).words).is_null() {
                libc::free((*cs).words as *mut libc::c_void);
            }
            if !((*cs).prefix).is_null() {
                libc::free((*cs).prefix as *mut libc::c_void);
            }
            if !((*cs).suffix).is_null() {
                libc::free((*cs).suffix as *mut libc::c_void);
            }
            if !((*cs).funcname).is_null() {
                libc::free((*cs).funcname as *mut libc::c_void);
            }
            if !((*cs).command).is_null() {
                libc::free((*cs).command as *mut libc::c_void);
            }
            if !((*cs).lcommand).is_null() {
                libc::free((*cs).lcommand as *mut libc::c_void);
            }
            if !((*cs).filterpat).is_null() {
                libc::free((*cs).filterpat as *mut libc::c_void);
            }
            libc::free(cs as *mut libc::c_void);
        }
    }
}

#[no_mangle]
pub fn compspec_copy(cs: *mut COMPSPEC) -> *mut COMPSPEC {
    unsafe {
        let new: *mut COMPSPEC =
            libc::malloc(::core::mem::size_of::<COMPSPEC>() as usize) as *mut COMPSPEC;
        (*new).refcount = 1 as libc::c_int;
        (*new).actions = (*cs).actions;
        (*new).options = (*cs).options;
        (*new).globpat = STRDUP!((*cs).globpat);
        (*new).words = STRDUP!((*cs).words);
        (*new).prefix = STRDUP!((*cs).prefix);
        (*new).suffix = STRDUP!((*cs).suffix);
        (*new).funcname = STRDUP!((*cs).funcname);
        (*new).command = STRDUP!((*cs).command);
        (*new).lcommand = STRDUP!((*cs).lcommand);
        (*new).filterpat = STRDUP!((*cs).filterpat);
        return new;
    }
}
// pub const COMPLETE_HASH_BUCKETS: libc::c_int = 256;

#[no_mangle]
pub fn progcomp_create() {
    unsafe {
        if prog_completes.is_null() {
            prog_completes = hash_create(COMPLETE_HASH_BUCKETS);
        }
    }
}

#[no_mangle]
pub fn progcomp_size() -> libc::c_int {
    unsafe {
        return if !prog_completes.is_null() {
            (*prog_completes).nentries
        } else {
            0 as libc::c_int
        };
    }
}

fn free_progcomp(data: *mut libc::c_void) {
    let cs: *mut COMPSPEC = data as *mut COMPSPEC;
    compspec_dispose(cs);
}

#[no_mangle]
pub fn progcomp_flush() {
    unsafe {
        if !prog_completes.is_null() {
            hash_flush(prog_completes, Some(free_progcomp));
        }
    }
}

#[no_mangle]
pub fn progcomp_dispose() {
    unsafe {
        if !prog_completes.is_null() {
            hash_dispose(prog_completes);
        }
        prog_completes = 0 as *mut libc::c_void as *mut HASH_TABLE;
    }
}

#[no_mangle]
pub fn progcomp_remove(cmd: *mut libc::c_char) -> libc::c_int {
    unsafe {
        let item: *mut BUCKET_CONTENTS;
        if prog_completes.is_null() {
            return 1 as libc::c_int;
        }
        item = hash_remove(cmd, prog_completes, 0 as libc::c_int);
        if !item.is_null() {
            if !((*item).data).is_null() {
                free_progcomp((*item).data);
            }
            libc::free((*item).key as *mut libc::c_void);
            libc::free(item as *mut libc::c_void);
            return 1 as libc::c_int;
        }
        return 0 as libc::c_int;
    }
}

#[no_mangle]
pub fn progcomp_insert(cmd: *mut libc::c_char, cs: *mut COMPSPEC) -> libc::c_int {
    unsafe {
        let item: *mut BUCKET_CONTENTS;
        if cs.is_null() {
            programming_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"progcomp_insert: %s: NULL COMPSPEC\0" as *const u8 as *const libc::c_char,
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
            (*item).key = libc::strcpy(
                libc::malloc((1 + libc::strlen(cmd)) as usize) as *mut libc::c_char,
                cmd,
            );
        }
        (*item).data = cs as *mut libc::c_void;
        return 1 as libc::c_int;
    }
}

#[no_mangle]
pub fn progcomp_search(cmd: *const libc::c_char) -> *mut COMPSPEC {
    unsafe {
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
}

#[no_mangle]
pub fn progcomp_walk(pfunc: Option<hash_wfunc>) {
    unsafe {
        if prog_completes.is_null()
            || pfunc.is_none()
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
}
