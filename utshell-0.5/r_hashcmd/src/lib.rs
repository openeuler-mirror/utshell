//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

use libc::{c_int,c_char,c_void};
use r_bash::*;

extern "C" {
    fn same_file(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut stat,
        _: *mut stat,
    ) -> libc::c_int;
}

#[no_mangle]
pub static mut hashed_filenames: *mut HASH_TABLE = 0 as *const libc::c_void
    as *mut libc::c_void as *mut HASH_TABLE;

#[no_mangle]
pub unsafe extern "C" fn phash_create() {
    if hashed_filenames.is_null() {
        hashed_filenames = hash_create(FILENAME_HASH_BUCKETS as c_int);
    }
}

unsafe extern "C" fn phash_freedata(mut data: *mut c_void) {
    free((*(data as *mut PATH_DATA)).path as *mut c_void);
    free(data);
}

#[no_mangle]
pub unsafe extern "C" fn phash_flush() {
    if !hashed_filenames.is_null() {
        hash_flush(
            hashed_filenames,
            Some(phash_freedata)
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn phash_remove(mut filename: *const c_char) -> c_int {
    let mut item: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    
    if hashing_enabled == 0 || hashed_filenames.is_null() {
        return 0 as c_int;
    }
    
    item = hash_remove(filename, hashed_filenames, 0 );
    if !item.is_null() {
        if !((*item).data).is_null() {
            phash_freedata((*item).data);
        }
        free((*item).key as *mut c_void);
        free(item as *mut c_void);
        return 0 ;
    }
    return 1 ;
}

#[macro_export]
macro_rules! savestring {
    ($x:expr) => {
        strcpy(malloc((strlen($x as *const c_char) + 1) as usize) as *mut c_char, $x) as *mut c_char
    };
}

#[no_mangle]
pub unsafe extern "C" fn phash_insert(
    mut filename: *mut c_char,
    mut full_path: *mut c_char,
    mut check_dot: c_int,
    mut found: c_int,
) {
    let mut item: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    
    if hashing_enabled == 0 {
        return;
    }
    
    if hashed_filenames.is_null() {
        phash_create();
    }
    
    item = hash_insert(filename, hashed_filenames, 0 );
    if !((*item).data).is_null() {
        free((*((*item).data as *mut PATH_DATA)).path as *mut c_void);
    } else {
        
        (*item).key = savestring!(filename);
        (*item).data = xmalloc(
            ::std::mem::size_of::<PATH_DATA>() as usize);
    }
    
    (*((*item).data as *mut PATH_DATA)).path = savestring!(full_path);
    (*((*item).data as *mut PATH_DATA)).flags = 0 ;
    if check_dot != 0 {
        (*((*item).data as *mut PATH_DATA)).flags |= HASH_CHKDOT as c_int;
    }
    if *full_path as c_int != '/' as i32 {
        (*((*item).data as *mut PATH_DATA)).flags |= HASH_RELPATH as c_int;
    }
    (*item).times_found = found;
}


#[no_mangle]
pub unsafe extern "C" fn phash_search(
    mut filename: *const c_char,
) -> *mut c_char {
    let mut item: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    let mut path: *mut c_char = 0 as *mut c_char;
    let mut dotted_filename: *mut c_char = 0 as *mut c_char;
    let mut tail: *mut c_char = 0 as *mut c_char;
    let mut same: c_int = 0;
    
    if hashing_enabled == 0 || hashed_filenames.is_null() {
        return 0 as *mut c_char;
    }
    
    item = hash_search(filename, hashed_filenames, 0 );
    
    if item.is_null() {
        return 0 as *mut c_char;
    }
    path = (*((*item).data as *mut PATH_DATA)).path;
    if (*((*item).data as *mut PATH_DATA)).flags
        & (HASH_CHKDOT as c_int | HASH_RELPATH as c_int) != 0
    {
        tail = if (*((*item).data as *mut PATH_DATA)).flags & HASH_RELPATH as c_int != 0 {
            path
        } else {
            filename as *mut c_char
        };
        if *tail.offset(0 as c_int as isize) as c_int != '.' as i32
            || *tail.offset(1 as c_int as isize) as c_int != '/' as i32
        {
            dotted_filename = xmalloc(
                (3 as c_int as libc::c_ulong).wrapping_add(strlen(tail)) as usize) as *mut c_char;
            *dotted_filename
                .offset(0 as isize) = '.' as i32 as c_char;
            *dotted_filename
                .offset(1 as isize) = '/' as i32 as c_char;
            strcpy(dotted_filename.offset(2 as c_int as isize), tail);
        } else {
            dotted_filename = savestring!(tail);
        }
        if executable_file(dotted_filename) != 0 {
            return dotted_filename;
        }
        free(dotted_filename as *mut c_void);
        
        if *path as c_int == '.' as i32 {
            same = 0 as c_int;
            tail = strrchr(path, '/' as i32);
            if !tail.is_null() {
                *tail = '\u{0}' as i32 as c_char;
                same = same_file(
                    b".\0" as *const u8 as *const c_char,
                    path,
                    0 as *mut c_void as *mut stat,
                    0 as *mut c_void as *mut stat,
                );
                *tail = '/' as i32 as c_char;
            }
            return if same != 0 {
                0 as *mut c_void as *mut c_char
            } else {
                savestring!(path)
            };
        }
    }
    return savestring!(path);
}

