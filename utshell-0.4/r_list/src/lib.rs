//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use stdext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct g_list {
    pub next: *mut g_list,
}
pub type GENERIC_LIST = g_list;
#[no_mangle]
pub static mut global_error_list: GENERIC_LIST = g_list {
    next: 0 as *const g_list as *mut g_list,
};
#[no_mangle]
pub unsafe extern "C" fn list_reverse(mut list: *mut GENERIC_LIST) -> *mut GENERIC_LIST {
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
#[no_mangle]
pub unsafe extern "C" fn list_length(mut list: *mut GENERIC_LIST) -> libc::c_int {
    let mut i: libc::c_int = 0;
    while !list.is_null() {
        list = (*list).next;
        i += 1;
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn list_append(
    head: *mut GENERIC_LIST,
    tail: *mut GENERIC_LIST,
) -> *mut GENERIC_LIST {
//    println!("{},{}", std::line!(), stdext::function_name!());
    let mut t_head: *mut GENERIC_LIST;
    if head.is_null() {
        return tail;
    }
    t_head = head;
    while !((*t_head).next).is_null() {
        t_head = (*t_head).next;
    }
    (*t_head).next = tail;
    return head;
}
