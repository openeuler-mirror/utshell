use crate::src_common::*;

#[no_mangle]
pub fn list_reverse(mut list: *mut GENERIC_LIST) -> *mut GENERIC_LIST {
    unsafe {
        let mut next: *mut GENERIC_LIST;
        let mut prev: *mut GENERIC_LIST;
        prev = 0 as *mut libc::c_void as *mut GENERIC_LIST;

        while !list.is_null() {
            next = (*list).next;
            (*list).next = prev;
            prev = list;
            list = next;
        }
        return prev;
    }
}
#[no_mangle]
pub fn list_length(mut list: *mut GENERIC_LIST) -> libc::c_int {
    let mut i: libc::c_int = 0;

    while !list.is_null() {
        list = unsafe { (*list).next };
        i += 1;
    }
    return i;
}
#[no_mangle]
pub fn list_append(head: *mut GENERIC_LIST, tail: *mut GENERIC_LIST) -> *mut GENERIC_LIST {
    let mut t_head: *mut GENERIC_LIST;
    if head.is_null() {
        return tail;
    }
    t_head = head;
    unsafe {
        while !((*t_head).next).is_null() {
            t_head = (*t_head).next;
        }
        (*t_head).next = tail;
    }
    return head;
}
