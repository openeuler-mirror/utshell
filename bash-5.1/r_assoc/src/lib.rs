use libc::*;
use r_bash::*;
use rcommon::WordList as WORD_LIST;
use rcommon::WordDesc as WORD_DESC;

pub type sh_free_func_t = unsafe extern "C" fn(*mut libc::c_void) -> ();

#[macro_export] 
macro_rules! assoc_empty {
    ($h:expr) =>{
        (*$h).nentries == 0 as libc::c_int
    }
}

#[macro_export]
macro_rules! savestring {
    ($name:expr) => {
        libc::strcpy(
            malloc((1 as libc::c_int + libc::strlen($name) as libc::c_int) as size_t) as *mut libc::c_char,
            $name )
    }
}

#[macro_export]
macro_rules! ALL_ELEMENT_SUB {
    ($c:expr) => {
        $c == '@' as i32 || $c == '*' as i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn assoc_dispose(mut hash: *mut HASH_TABLE) {
    if !hash.is_null() {
        hash_flush(hash, None);
        hash_dispose(hash);
    }
}

#[no_mangle]
pub unsafe extern "C" fn assoc_flush(mut hash: *mut HASH_TABLE) {
    hash_flush(hash, None);
}

#[no_mangle]
pub unsafe extern "C" fn assoc_insert(
    mut hash: *mut HASH_TABLE,
    mut key: *mut libc::c_char,
    mut value: *mut libc::c_char,
) -> libc::c_int {
    let mut b: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    b = hash_search(key, hash, 0x2 as libc::c_int);
    if b.is_null() {
        return -(1 as libc::c_int);
    }
    if (*b).key != key {
        libc::free(key as *mut libc::c_void);
    }
    if !((*b).data).is_null() {
        libc::free((*b).data);
    }
    (*b).data = 0 as *mut libc::c_void;
    (*b)
        .data = (if !value.is_null() {
       savestring!(value)
    } else {
        0 as *mut libc::c_char 
    }) as *mut libc::c_void;
    return 0 as libc::c_int;
}
