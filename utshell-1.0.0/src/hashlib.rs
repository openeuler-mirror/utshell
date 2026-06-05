use crate::src_common::*;

#[no_mangle]
pub fn hash_create(mut buckets: libc::c_int) -> *mut HASH_TABLE {
    let new_table: *mut HASH_TABLE;
    let mut i: libc::c_int;

    new_table = unsafe { malloc(::std::mem::size_of::<HASH_TABLE>() as usize) as *mut HASH_TABLE };
    if buckets == 0 {
        buckets = DEFAULT_HASH_BUCKETS as libc::c_int;
    }
    unsafe {
        (*new_table).bucket_array = malloc(
            (buckets as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut BUCKET_CONTENTS>() as libc::c_ulong)
                as usize,
        ) as *mut *mut BUCKET_CONTENTS;
        (*new_table).nbuckets = buckets;
        (*new_table).nentries = 0;
    }
    i = 0;
    while i < buckets {
        unsafe {
            *((*new_table).bucket_array).offset(i as isize) = 0 as *mut BUCKET_CONTENTS;
        }

        i += 1;
    }
    return new_table;
}

#[no_mangle]
pub fn hash_size(table: *mut HASH_TABLE) -> libc::c_int {
    unsafe {
        return HASH_ENTRIES!(table);
    }
}

fn copy_bucket_array(
    ba: *mut BUCKET_CONTENTS,
    cpdata: Option<sh_string_func_t>,
) -> *mut BUCKET_CONTENTS {
    let mut new_bucket: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    let mut n: *mut BUCKET_CONTENTS;
    let mut e: *mut BUCKET_CONTENTS;

    if ba.is_null() {
        return 0 as *mut BUCKET_CONTENTS;
    }

    n = 0 as *mut BUCKET_CONTENTS;
    e = ba;
    while !e.is_null() {
        unsafe {
            if n.is_null() {
                new_bucket = malloc(::std::mem::size_of::<BUCKET_CONTENTS>() as usize)
                    as *mut BUCKET_CONTENTS;
                n = new_bucket;
            } else {
                (*n).next = malloc(::std::mem::size_of::<BUCKET_CONTENTS>() as usize)
                    as *mut BUCKET_CONTENTS;
                n = (*n).next;
            }

            (*n).key = savestring!((*e).key);
            (*n).data = (if !((*e).data).is_null() {
                if cpdata.is_some() {
                    (cpdata.expect("non-null function pointer"))((*e).data as *mut libc::c_char)
                } else {
                    savestring!((*e).data as *const libc::c_char)
                }
            } else {
                0 as *mut libc::c_char
            }) as *mut c_void;

            (*n).khash = (*e).khash;
            (*n).times_found = (*e).times_found;
            (*n).next = 0 as *mut c_void as *mut BUCKET_CONTENTS;

            e = (*e).next;
        }
    }
    return new_bucket;
}

fn hash_rehash(table: *mut HASH_TABLE, nsize: libc::c_int) {
    let osize: libc::c_int;
    let mut i: libc::c_int;
    let mut j: libc::c_int;
    let old_bucket_array: *mut *mut BUCKET_CONTENTS;
    let mut item: *mut BUCKET_CONTENTS;
    let mut next: *mut BUCKET_CONTENTS;
    unsafe {
        if table.is_null() || nsize == (*table).nbuckets {
            return;
        }

        osize = (*table).nbuckets;
        old_bucket_array = (*table).bucket_array;

        (*table).nbuckets = nsize;
        (*table).bucket_array = malloc(
            ((*table).nbuckets as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut BUCKET_CONTENTS>() as u64)
                as usize,
        ) as *mut *mut BUCKET_CONTENTS;

        i = 0;
        while i < (*table).nbuckets {
            *((*table).bucket_array).offset(i as isize) = 0 as *mut BUCKET_CONTENTS;
            i += 1;
        }

        j = 0;
        while j < osize {
            item = *old_bucket_array.offset(j as isize);
            while !item.is_null() {
                next = (*item).next;
                i = ((*item).khash & ((*table).nbuckets - 1 as libc::c_int) as libc::c_uint)
                    as libc::c_int;
                (*item).next = *((*table).bucket_array).offset(i as isize);
                *((*table).bucket_array).offset(i as isize) = item;

                item = next;
            }
            j += 1;
        }

        free(old_bucket_array as *mut c_void);
    }
}

fn hash_grow(table: *mut HASH_TABLE) {
    let nsize: libc::c_int;

    nsize = unsafe { (*table).nbuckets * HASH_REHASH_MULTIPLIER!() };
    if nsize > 0 {
        hash_rehash(table, nsize);
    }
}

// fn hash_shrink(table: *mut HASH_TABLE) {
//     let mut nsize: libc::c_int = 0;

//     nsize = unsafe { (*table).nbuckets / HASH_REHASH_MULTIPLIER!() };
//     hash_rehash(table, nsize);
// }

#[no_mangle]
pub fn hash_copy(table: *mut HASH_TABLE, cpdata: Option<sh_string_func_t>) -> *mut HASH_TABLE {
    let new_table: *mut HASH_TABLE;
    let mut i: libc::c_int;

    if table.is_null() {
        return 0 as *mut HASH_TABLE;
    }
    unsafe {
        new_table = hash_create((*table).nbuckets);

        i = 0;
        while i < (*table).nbuckets {
            *((*new_table).bucket_array).offset(i as isize) =
                copy_bucket_array(*((*table).bucket_array).offset(i as isize), cpdata);
            i += 1;
        }

        (*new_table).nentries = (*table).nentries;
    }
    return new_table;
}

#[no_mangle]
pub fn hash_string(mut s: *const libc::c_char) -> libc::c_uint {
    let mut i: libc::c_uint;

    i = FNV_OFFSET!();
    while unsafe { *s != 0 } {
        i = i.wrapping_add(
            (i << 1 as libc::c_int)
                .wrapping_add(i << 4 as libc::c_int)
                .wrapping_add(i << 7 as libc::c_int)
                .wrapping_add(i << 8 as libc::c_int)
                .wrapping_add(i << 24 as libc::c_int),
        );
        unsafe {
            i ^= *s as libc::c_uint;
            s = s.offset(1);
        }
    }
    return i;
}

#[no_mangle]
pub fn hash_bucket(string: *const libc::c_char, table: *mut HASH_TABLE) -> libc::c_int {
    let h: libc::c_uint;
    unsafe {
        return HASH_BUCKET!(string, table, h) as i32;
    }
}

#[no_mangle]
pub fn hash_search(
    string: *const libc::c_char,
    table: *mut HASH_TABLE,
    flags: libc::c_int,
) -> *mut BUCKET_CONTENTS {
    let mut list: *mut BUCKET_CONTENTS;
    let mut bucket: libc::c_int;
    let mut hv: libc::c_uint;
    unsafe {
        if table.is_null()
            || flags & HASH_CREATE as libc::c_int == 0 as libc::c_int
                && HASH_ENTRIES!(table) == 0 as libc::c_int
        {
            return 0 as *mut BUCKET_CONTENTS;
        }

        bucket = HASH_BUCKET!(string, table, hv) as i32;

        list = if !((*table).bucket_array).is_null() {
            *((*table).bucket_array).offset(bucket as isize)
        } else {
            0 as *mut BUCKET_CONTENTS
        };
        while !list.is_null() {
            if hv == (*list).khash && STREQ!((*list).key, string) {
                (*list).times_found += 1;
                return list;
            }
            list = (*list).next;
        }

        if flags & HASH_CREATE as libc::c_int != 0 {
            if HASH_SHOULDGROW!(table) {
                hash_grow(table);
                hash_string(string);
                bucket = HASH_BUCKET!(string, table, hv) as i32;
            }
            list =
                malloc(::std::mem::size_of::<BUCKET_CONTENTS>() as usize) as *mut BUCKET_CONTENTS;
            (*list).next = *((*table).bucket_array).offset(bucket as isize);
            *((*table).bucket_array).offset(bucket as isize) = list;

            (*list).data = 0 as *mut c_void;
            (*list).key = string as *mut libc::c_char;
            (*list).khash = hv;
            (*list).times_found = 0 as libc::c_int;

            (*table).nentries += 1;
            return list;
        }
    }
    return 0 as *mut BUCKET_CONTENTS;
}

#[no_mangle]
pub fn hash_remove(
    string: *const libc::c_char,
    table: *mut HASH_TABLE,
    _flags: libc::c_int,
) -> *mut BUCKET_CONTENTS {
    let bucket: libc::c_int;
    let mut prev: *mut BUCKET_CONTENTS;
    let mut temp: *mut BUCKET_CONTENTS;
    let hv: libc::c_uint;
    unsafe {
        if table.is_null() || HASH_ENTRIES!(table) == 0 as libc::c_int {
            return 0 as *mut BUCKET_CONTENTS;
        }

        bucket = HASH_BUCKET!(string, table, hv) as i32;
        prev = 0 as *mut c_void as *mut BUCKET_CONTENTS;

        temp = *((*table).bucket_array).offset(bucket as isize);
        while !temp.is_null() {
            if hv == (*temp).khash && STREQ!((*temp).key, string) {
                if !prev.is_null() {
                    (*prev).next = (*temp).next;
                } else {
                    *((*table).bucket_array).offset(bucket as isize) = (*temp).next;
                }
                (*table).nentries -= 1;
                return temp;
            }
            prev = temp;
            temp = (*temp).next;
        }
    }
    return 0 as *mut c_void as *mut BUCKET_CONTENTS;
}

#[no_mangle]
pub fn hash_insert(
    string: *mut libc::c_char,
    mut table: *mut HASH_TABLE,
    flags: libc::c_int,
) -> *mut BUCKET_CONTENTS {
    let mut item: *mut BUCKET_CONTENTS;
    let bucket: libc::c_int;
    let hv: libc::c_uint;

    if table.is_null() {
        table = hash_create(0 as libc::c_int);
    }
    item = if flags & 0x1 as libc::c_int != 0 {
        0 as *mut c_void as *mut BUCKET_CONTENTS
    } else {
        hash_search(string, table, 0 as libc::c_int)
    };
    if item.is_null() {
        if unsafe { HASH_SHOULDGROW!(table) } {
            hash_grow(table);
        }
        unsafe {
            bucket = HASH_BUCKET!(string, table, hv) as i32;
            item =
                malloc(::std::mem::size_of::<BUCKET_CONTENTS>() as usize) as *mut BUCKET_CONTENTS;

            (*item).next = *((*table).bucket_array).offset(bucket as isize);
            *((*table).bucket_array).offset(bucket as isize) = item;

            (*item).data = 0 as *mut c_void;
            (*item).key = string;
            (*item).khash = hv;
            (*item).times_found = 0 as libc::c_int;

            (*table).nentries += 1;
        }
    }

    return item;
}

#[no_mangle]
pub fn hash_flush(table: *mut HASH_TABLE, free_data: sh_free_func_t) {
    let mut i: libc::c_int;
    let mut bucket: *mut BUCKET_CONTENTS;
    let mut item: *mut BUCKET_CONTENTS;
    unsafe {
        if table.is_null() || HASH_ENTRIES!(table) == 0 as libc::c_int {
            return;
        }

        i = 0;
        while i < (*table).nbuckets {
            bucket = *((*table).bucket_array).offset(i as isize);
            while !bucket.is_null() {
                item = bucket;
                bucket = (*bucket).next;

                if free_data.is_some() {
                    (Some(free_data.expect("non-null function pointer")))
                        .expect("non-null function pointer")((*item).data);
                } else {
                    free((*item).data);
                }
                free((*item).key as *mut c_void);
                free(item as *mut c_void);
            }

            *((*table).bucket_array).offset(i as isize) = 0 as *mut BUCKET_CONTENTS;
            i += 1;
        }
        (*table).nentries = 0;
    }
}

#[no_mangle]
pub fn hash_dispose(table: *mut HASH_TABLE) {
    unsafe {
        free((*table).bucket_array as *mut c_void);
        free(table as *mut c_void);
    }
}

#[no_mangle]
pub fn hash_walk(table: *mut HASH_TABLE, func: Option<hash_wfunc>) {
    let mut i: libc::c_int;
    let mut item: *mut BUCKET_CONTENTS;
    unsafe {
        if table.is_null() || HASH_ENTRIES!(table) == 0 as libc::c_int {
            return;
        }
        i = 0 as libc::c_int;

        while i < (*table).nbuckets {
            item = hash_items!(i, table);

            while !item.is_null() {
                if (Some(func.expect("non-null function pointer")))
                    .expect("non-null function pointer")(item)
                    < 0 as libc::c_int
                {
                    return;
                }
                item = (*item).next;
            }
            i += 1;
        }
    }
}
