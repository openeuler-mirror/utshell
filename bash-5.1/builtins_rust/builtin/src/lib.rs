use libc::{c_int, c_char};

include!(concat!("intercdep.rs"));

#[no_mangle]
pub extern "C" fn r_builtin_builtin(list: *mut WORD_LIST) -> i32 {
    println!("");
  println!("r_builtin_builtin call");

  unsafe {
    if no_options (list) !=0 {
      return EX_USAGE;
    } 

    if loptend == std::ptr::null_mut() {
            return EXECUTION_SUCCESS;
    }

    let mut list = loptend;
    let t:WORD_LIST=*loptend;
    let function = builtin_address((*(t.word)).word);

    list = (*list).next;
    return function(list);
  }
}

