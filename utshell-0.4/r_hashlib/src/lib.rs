//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later


use libc::{c_int, c_char, c_void};
use r_bash::*;

extern "C" {

}

#[no_mangle]
pub unsafe extern "C" fn hash_create(mut buckets: c_int) -> *mut HASH_TABLE {
    let mut new_table: *mut HASH_TABLE = 0 as *mut HASH_TABLE;
    let mut i: c_int = 0;
    
    new_table = malloc(
        ::std::mem::size_of::<HASH_TABLE>() as usize
    ) as *mut HASH_TABLE;
    if buckets == 0 {
        buckets = DEFAULT_HASH_BUCKETS as c_int ;
    }

    (*new_table).bucket_array = malloc(
        (buckets as libc::c_ulong)
            .wrapping_mul(
                ::std::mem::size_of::<*mut BUCKET_CONTENTS>() as libc::c_ulong,
            ) as usize
    ) as *mut *mut BUCKET_CONTENTS;
    (*new_table).nbuckets = buckets;
    (*new_table).nentries = 0 ;
    
    i = 0 ;
    while i < buckets {
        *((*new_table).bucket_array).offset(i as isize) = 0 as *mut BUCKET_CONTENTS;
        
        i += 1;
    }
    return new_table;
}

#[macro_export]
macro_rules! HASH_ENTRIES {
    ($ht:expr) => {
        if !$ht.is_null() { (*$ht).nentries } else { 0 as c_int };
    };
}

#[no_mangle]
pub unsafe extern "C" fn hash_size(mut table: *mut HASH_TABLE) -> c_int {
    return HASH_ENTRIES!(table);
}

#[macro_export]
macro_rules! savestring {
    ($x:expr) => {
        strcpy(malloc((strlen($x as *const c_char) + 1) as usize) as *mut c_char, $x) as *mut c_char
    };
}

unsafe extern "C" fn copy_bucket_array(
    mut ba: *mut BUCKET_CONTENTS,
    mut cpdata: sh_string_func_t,
) -> *mut BUCKET_CONTENTS {
    let mut new_bucket: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    let mut n: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    let mut e: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    
    if ba.is_null() {
        return 0 as *mut BUCKET_CONTENTS;
    }
    
    n = 0 as *mut BUCKET_CONTENTS;
    e = ba;
    while !e.is_null() {
        if n.is_null() {
            new_bucket = malloc(
                ::std::mem::size_of::<BUCKET_CONTENTS>() as usize
            ) as *mut BUCKET_CONTENTS;
            n = new_bucket;
        } else {
            (*n).next = malloc(
                ::std::mem::size_of::<BUCKET_CONTENTS>() as usize) as *mut BUCKET_CONTENTS;
            n = (*n).next;
        }
        
        (*n).key = savestring!((*e).key);
        (*n).data= (if !((*e).data).is_null() {
            if cpdata.is_some() {
                (cpdata.expect("non-null function pointer"))((*e).data as *mut c_char)
            } else {
                savestring!((*e).data as *const c_char) 
            }
        } else {
            0 as *mut c_char
        }) as *mut c_void;
        
        (*n).khash = (*e).khash;
        (*n).times_found = (*e).times_found;
        (*n).next = 0 as *mut c_void as *mut BUCKET_CONTENTS;
        
        e = (*e).next;
    }
    return new_bucket;
}

unsafe extern "C" fn hash_rehash(mut table: *mut HASH_TABLE, mut nsize: c_int) {
    let mut osize: c_int = 0;
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut old_bucket_array: *mut *mut BUCKET_CONTENTS = 0 as *mut *mut BUCKET_CONTENTS;
    let mut item: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    let mut next: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    
    if table.is_null() || nsize == (*table).nbuckets {
        return;
    }
    
    osize = (*table).nbuckets;
    old_bucket_array = (*table).bucket_array;
    
    (*table).nbuckets = nsize;
    (*table).bucket_array = malloc(
        ((*table).nbuckets as libc::c_ulong)
            .wrapping_mul(
                ::std::mem::size_of::<*mut BUCKET_CONTENTS>() as u64
            ) as usize
    ) as *mut *mut BUCKET_CONTENTS;
    
    i = 0 ;
    while i < (*table).nbuckets {
        *((*table).bucket_array).offset(i as isize) = 0 as *mut BUCKET_CONTENTS;
        i += 1;
    }
    
    j = 0 ;
    while j < osize {
        item = *old_bucket_array.offset(j as isize);
        while !item.is_null() {
            next = (*item).next;
            i = ((*item).khash & ((*table).nbuckets - 1 as c_int) as libc::c_uint)
                as c_int;
            (*item).next = *((*table).bucket_array).offset(i as isize);
            *((*table).bucket_array).offset(i as isize) = item;
            
            item = next;
        }
        j += 1;
    }

    free(
        old_bucket_array as *mut c_void
    );
}

#[macro_export]
macro_rules! HASH_REHASH_MULTIPLIER {
    () => {
        4
    };
}

unsafe extern "C" fn hash_grow(mut table: *mut HASH_TABLE) {
    let mut nsize: c_int = 0;
    
    nsize = (*table).nbuckets * HASH_REHASH_MULTIPLIER!();
    if nsize > 0 {
        hash_rehash(table, nsize);
    }
}

unsafe extern "C" fn hash_shrink(mut table: *mut HASH_TABLE){
    let mut nsize: c_int = 0;

    nsize = (*table).nbuckets / HASH_REHASH_MULTIPLIER!();
    hash_rehash(table, nsize);
}


#[no_mangle]
pub unsafe extern "C" fn hash_copy(
    mut table: *mut HASH_TABLE,
    mut cpdata: sh_string_func_t,
) -> *mut HASH_TABLE {
    let mut new_table: *mut HASH_TABLE = 0 as *mut HASH_TABLE;
    let mut i: c_int = 0;
    
    if table.is_null() {
        return 0 as *mut HASH_TABLE;
    }
    new_table = hash_create((*table).nbuckets);
    
    i = 0 ;
    while i < (*table).nbuckets {
        *((*new_table).bucket_array).offset(i as isize) = copy_bucket_array(
            *((*table).bucket_array).offset(i as isize),
            cpdata,
        );
        i += 1;
    }
    
    (*new_table).nentries = (*table).nentries;
    return new_table;
}

#[macro_export]
macro_rules! FNV_OFFSET {
    () => {
        2166136261
    };
}

#[macro_export]
macro_rules! FNV_PRIME {
    () => {
        16777619
    };
}

#[no_mangle]
pub unsafe extern "C" fn hash_string(mut s: *const c_char) -> libc::c_uint {
    let mut i: libc::c_uint = 0;
    
    i = FNV_OFFSET!();
    while *s != 0 {
        i = i
            .wrapping_add(
                (i << 1 as c_int)
                    .wrapping_add(i << 4 as c_int)
                    .wrapping_add(i << 7 as c_int)
                    .wrapping_add(i << 8 as c_int)
                    .wrapping_add(i << 24 as c_int),
            );
        i ^= *s as libc::c_uint;
        s = s.offset(1);
    }
    return i;
}

#[macro_export]
macro_rules! HASH_BUCKET {
    ($s:expr, $t:expr, $h:expr) => {
        {$h = hash_string($s) ;
        $h & (((*$t).nbuckets - 1 ) as libc::c_uint)}
    };
}

#[no_mangle]
pub unsafe extern "C" fn hash_bucket (
    mut string: *const c_char,
    mut table: *mut HASH_TABLE,
) -> c_int {
    let mut h: libc::c_uint = 0;

    return HASH_BUCKET!(string, table, h) as i32;
}

#[macro_export]
macro_rules! STREQ {
    ($a:expr, $b:expr) => {
        (*$a.offset(0 as c_int as isize) as c_int
                    == *$b.offset(0 as c_int as isize) as c_int
                    && strcmp($a, $b) == 0 as c_int)
    };
}

#[macro_export]
macro_rules! HASH_REHASH_FACTOR {
    () => {
        2
    };
}

#[macro_export]
macro_rules! HASH_SHOULDGROW {
    ($table:expr) => {
        (*$table).nentries >= (*$table).nbuckets * HASH_REHASH_FACTOR!()
    };
}


#[no_mangle]
pub unsafe extern "C" fn hash_search(
    mut string: *const c_char,
    mut table: *mut HASH_TABLE,
    mut flags: c_int,
) -> *mut BUCKET_CONTENTS {
    let mut list: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    let mut bucket: c_int = 0;
    let mut hv: libc::c_uint = 0;
    
    if table.is_null()
        || flags & HASH_CREATE as c_int == 0 as c_int
            && HASH_ENTRIES!(table)
                == 0 as c_int
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
        if hv == (*list).khash
            && STREQ!((*list).key, string)
        {
            (*list).times_found += 1;
            return list;
        }
        list = (*list).next;
    }
    
    if flags & HASH_CREATE as c_int != 0 {
        if HASH_SHOULDGROW!(table) {
            hash_grow(table);
            hv = hash_string(string);
            bucket = HASH_BUCKET!(string, table, hv) as i32; 
        }
        list = malloc(
            ::std::mem::size_of::<BUCKET_CONTENTS>() as usize
        ) as *mut BUCKET_CONTENTS;
        (*list).next = *((*table).bucket_array).offset(bucket as isize);
        *((*table).bucket_array).offset(bucket as isize) = list;
        
        (*list).data = 0 as *mut c_void;
        (*list).key = string as *mut c_char;
        (*list).khash = hv;
        (*list).times_found = 0 as c_int;
        
        (*table).nentries += 1;
        return list;
    }
    return 0 as *mut BUCKET_CONTENTS;
}

#[no_mangle]
pub unsafe extern "C" fn hash_remove(
    mut string: *const c_char,
    mut table: *mut HASH_TABLE,
    mut flags: c_int,
) -> *mut BUCKET_CONTENTS {
    let mut bucket: c_int = 0;
    let mut prev: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    let mut temp: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    let mut hv: libc::c_uint = 0;
    
    if table.is_null()
        || HASH_ENTRIES!(table) == 0 as c_int
    {
        return 0 as *mut BUCKET_CONTENTS;
    }
    
    bucket = HASH_BUCKET!(string, table, hv) as i32;
    prev = 0 as *mut c_void as *mut BUCKET_CONTENTS;
    
    temp = *((*table).bucket_array).offset(bucket as isize);
    while !temp.is_null() {
        if hv == (*temp).khash
            && STREQ!((*temp).key, string) 
        {
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
    return 0 as *mut c_void as *mut BUCKET_CONTENTS;
}

#[no_mangle]
pub unsafe extern "C" fn hash_insert(
    mut string: *mut c_char,
    mut table: *mut HASH_TABLE,
    mut flags: c_int,
) -> *mut BUCKET_CONTENTS {
    let mut item: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    let mut bucket: c_int = 0;
    let mut hv: libc::c_uint = 0;

    if table.is_null() {
        table = hash_create(0 as c_int);
    }
    item = if flags & 0x1 as c_int != 0 {
        0 as *mut c_void as *mut BUCKET_CONTENTS
    } else {
        hash_search(string, table, 0 as c_int)
    };
    if item.is_null() {
        if HASH_SHOULDGROW!(table) {
            hash_grow(table);
        }

        bucket = HASH_BUCKET!(string, table, hv) as i32;
        item = malloc(
            ::std::mem::size_of::<BUCKET_CONTENTS>() as usize
        ) as *mut BUCKET_CONTENTS;

        (*item).next = *((*table).bucket_array).offset(bucket as isize);
        *((*table).bucket_array).offset(bucket as isize) = item;
        
        (*item).data = 0 as *mut c_void;
        (*item).key = string;
        (*item).khash = hv;
        (*item).times_found = 0 as c_int;
        
        (*table).nentries += 1;
    }
    return item;
}

#[no_mangle]
pub unsafe extern "C" fn hash_flush(
    mut table: *mut HASH_TABLE,
    mut free_data: sh_free_func_t,
) {
    let mut i: c_int = 0;
    let mut bucket: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    let mut item: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    
    if table.is_null()
        || HASH_ENTRIES!(table) == 0 as c_int
    {
        return;
    }
    
    i = 0 ;
    while i < (*table).nbuckets {
        bucket = *((*table).bucket_array).offset(i as isize);
        while !bucket.is_null() {
            item = bucket;
            bucket = (*bucket).next;
            
            if free_data.is_some() {
                (Some(free_data.expect("non-null function pointer")))
                    .expect("non-null function pointer")((*item).data);
            } else {
                free(
                    (*item).data,
                );
            }
            free(
                (*item).key as *mut c_void
            );
            free(
                item as *mut c_void
            );
        }
        
        *((*table).bucket_array).offset(i as isize) = 0 as *mut BUCKET_CONTENTS;
        i += 1;
    }
    (*table).nentries = 0 ;
}

#[no_mangle]
pub unsafe extern "C" fn hash_dispose(mut table: *mut HASH_TABLE) {
    free(
        (*table).bucket_array as *mut c_void
    );
    free(
        table as *mut c_void
    );
}

#[macro_export]
macro_rules! hash_items {
    ($bucket:expr, $table:expr) => {
        if !$table.is_null() && $bucket < (*$table).nbuckets {
            *((*$table).bucket_array).offset($bucket as isize)
        } else {
            0 as *mut BUCKET_CONTENTS
        };
    };
}

#[no_mangle]
pub unsafe extern "C" fn hash_walk(
    mut table: *mut HASH_TABLE,
    mut func: hash_wfunc,
) {
    let mut i: c_int = 0;
    let mut item: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    if table.is_null()
        || HASH_ENTRIES!(table) == 0 as c_int
    {
        return;
    }
    i = 0 as c_int;
    while i < (*table).nbuckets {
        item = hash_items!(i, table);

        while !item.is_null() {
            if (Some(func.expect("non-null function pointer")))
                .expect("non-null function pointer")(item) < 0 as c_int
            {
                return;
            }
            item = (*item).next;
        }
        i += 1;
    }
}
