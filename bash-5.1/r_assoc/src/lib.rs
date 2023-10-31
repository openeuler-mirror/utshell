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

#[macro_export]
macro_rules! STRLEN {
    ($s:expr) => {
        if !$s.is_null()
        && *$s.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            if *$s.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                if *$s.offset(2 as libc::c_int as isize) as libc::c_int != 0 {
                    libc::strlen($s) as libc::c_int
                } else {
                    2 as libc::c_int 
                }
            } 
            else {
                1 as libc::c_int 
            }
        } 
        else {
        0 as libc::c_int 
        }
    }
}

#[macro_export]
macro_rules! RESIZE_MALLOCED_BUFFER {
    ($str:expr,$cind:expr,$room:expr,$csize:expr,$sincr:expr) => {
        if $cind + $room >= $csize
        {
            while $cind + $room >= $csize {
                $csize += $sincr;
            }   
            $str = xrealloc ($str as *mut c_void , $csize as usize) as *mut libc::c_char;
        } 
    }
}

#[macro_export]
macro_rules! REVERSE_LIST {
    ($list:expr,$type:ty) => {
        if !$list.is_null() && !((*$list).next).is_null() {
            list_reverse($list as *mut GENERIC_LIST) as $type
        }
        else {
            $list as $type
        }
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

#[no_mangle]
pub unsafe extern "C" fn assoc_remove(
    mut hash: *mut HASH_TABLE,
    mut string: *mut libc::c_char,
) {
    let mut b: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    b = hash_remove(string, hash, 0 as libc::c_int);
    if !b.is_null() {
        libc::free((*b).data as *mut libc::c_char as *mut libc::c_void);
        libc::free((*b).key as *mut libc::c_void);
        libc::free(b as *mut libc::c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn assoc_reference(
    mut hash: *mut HASH_TABLE,
    mut string: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut b: *mut BUCKET_CONTENTS = 0 as *mut BUCKET_CONTENTS;
    if hash.is_null() {
        return 0 as *mut libc::c_char;
    }
    b = hash_search(string, hash, 0 as libc::c_int);
    return if !b.is_null() {
        (*b).data as *mut libc::c_char
    } else {
        0 as *mut libc::c_char
    };
}
#[no_mangle]

