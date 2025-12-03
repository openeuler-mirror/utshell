use crate::findcmd::executable_file;
use crate::general::same_file;
use crate::hashlib::{hash_create, hash_flush, hash_insert, hash_remove, hash_search};
use crate::src_common::*;

#[no_mangle]
pub fn phash_create() {
    unsafe {
        if hashed_filenames.is_null() {
            hashed_filenames = hash_create(FILENAME_HASH_BUCKETS as libc::c_int);
        }
    }
}

fn phash_freedata(mut data: *mut c_void) {
    unsafe {
        free((*(data as *mut PATH_DATA)).path as *mut c_void);
        free(data);
    }
}

#[no_mangle]
pub fn phash_flush() {
    unsafe {
        if !hashed_filenames.is_null() {
            hash_flush(hashed_filenames, Some(phash_freedata));
        }
    }
}

#[no_mangle]
pub fn phash_remove(mut filename: *const libc::c_char) -> libc::c_int {
    let mut item: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    unsafe {
        if hashing_enabled == 0 || hashed_filenames.is_null() {
            return 0 as libc::c_int;
        }

        item = hash_remove(filename, hashed_filenames, 0);
        if !item.is_null() {
            if !((*item).data).is_null() {
                phash_freedata((*item).data);
            }
            free((*item).key as *mut c_void);
            free(item as *mut c_void);
            return 0;
        }
        return 1;
    }
}

#[no_mangle]
pub fn phash_insert(
    mut filename: *mut libc::c_char,
    mut full_path: *mut libc::c_char,
    mut check_dot: libc::c_int,
    mut found: libc::c_int,
) {
    let mut item: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    unsafe {
        if hashing_enabled == 0 {
            return;
        }

        if hashed_filenames.is_null() {
            phash_create();
        }

        item = hash_insert(filename, hashed_filenames, 0);
        if !((*item).data).is_null() {
            free((*((*item).data as *mut PATH_DATA)).path as *mut c_void);
        } else {
            (*item).key = savestring!(filename);
            (*item).data = libc::malloc(::std::mem::size_of::<PATH_DATA>() as usize);
        }

        (*((*item).data as *mut PATH_DATA)).path = savestring!(full_path);
        (*((*item).data as *mut PATH_DATA)).flags = 0;
        if check_dot != 0 {
            (*((*item).data as *mut PATH_DATA)).flags |= HASH_CHKDOT as libc::c_int;
        }
        if *full_path as libc::c_int != '/' as i32 {
            (*((*item).data as *mut PATH_DATA)).flags |= HASH_RELPATH as libc::c_int;
        }
        (*item).times_found = found;
    }
}

#[no_mangle]
pub fn phash_search(mut filename: *const libc::c_char) -> *mut libc::c_char {
    let mut item: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dotted_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tail: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut same: libc::c_int = 0;
    unsafe {
        if hashing_enabled == 0 || hashed_filenames.is_null() {
            return 0 as *mut libc::c_char;
        }

        item = hash_search(filename, hashed_filenames, 0);

        if item.is_null() {
            return 0 as *mut libc::c_char;
        }
        path = (*((*item).data as *mut PATH_DATA)).path;
        if (*((*item).data as *mut PATH_DATA)).flags
            & (HASH_CHKDOT as libc::c_int | HASH_RELPATH as libc::c_int)
            != 0
        {
            tail = if (*((*item).data as *mut PATH_DATA)).flags & HASH_RELPATH as libc::c_int != 0 {
                path
            } else {
                filename as *mut libc::c_char
            };
            if *tail.offset(0 as libc::c_int as isize) as libc::c_int != '.' as i32
                || *tail.offset(1 as libc::c_int as isize) as libc::c_int != '/' as i32
            {
                dotted_filename = libc::malloc(
                    (3 as libc::c_int as libc::c_ulong).wrapping_add(strlen(tail)) as usize,
                ) as *mut libc::c_char;
                *dotted_filename.offset(0 as isize) = '.' as i32 as libc::c_char;
                *dotted_filename.offset(1 as isize) = '/' as i32 as libc::c_char;
                strcpy(dotted_filename.offset(2 as libc::c_int as isize), tail);
            } else {
                dotted_filename = savestring!(tail);
            }
            if executable_file(dotted_filename) != 0 {
                return dotted_filename;
            }
            free(dotted_filename as *mut c_void);

            if *path as libc::c_int == '.' as i32 {
                same = 0 as libc::c_int;
                tail = strrchr(path, '/' as i32);
                if !tail.is_null() {
                    *tail = '\u{0}' as i32 as libc::c_char;
                    same = same_file(
                        b".\0" as *const u8 as *const libc::c_char,
                        path,
                        0 as *mut c_void as *mut crate::src_common::stat,
                        0 as *mut c_void as *mut crate::src_common::stat,
                    );
                    *tail = '/' as i32 as libc::c_char;
                }
                return if same != 0 {
                    0 as *mut c_void as *mut libc::c_char
                } else {
                    savestring!(path)
                };
            }
        }
        return savestring!(path);
    }
}
