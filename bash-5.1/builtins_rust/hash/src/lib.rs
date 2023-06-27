extern crate libc;
extern crate rread;


use libc::{c_char,c_int, strchr,free,c_void,strerror,EISDIR};
use std::ffi::{CStr,CString};
use std::io::{stdout, Write};
use rread::{SHELL_VAR};

//struct
//结构体
#[repr (C)]
pub struct WORD_DESC{
    pub word:*mut c_char,
    pub flags:c_int,
}

#[repr (C)]
// #[derive(Copy,Clone)]
pub struct WORD_LIST{
    pub next:*mut WORD_LIST,
    pub word:*mut WORD_DESC,
}

type PTR_T=c_void;
#[repr (C)]
pub struct bucket_contents{
    pub next:*mut bucket_contents,
    pub key:*mut c_char,
    pub data:*mut PTR_T,     //void *  or char *
    pub khash:u32,
    pub times_found:i32,
}
type BUCKET_CONTENTS=bucket_contents;

#[repr (C)]
pub struct hash_table{
    pub bucket_array:*mut *mut BUCKET_CONTENTS,
    pub nbucjets:i32,
    pub nentries:i32,
}
type HASH_TABLE = hash_table;

#[repr (C)]
pub struct _pathdata{
    pub path:*mut c_char,
    pub flags:i32,
}
type PATH_DATA = _pathdata;
//enum

//macro
#[macro_export]
macro_rules! EXECUTION_SUCCESS {
    () => { 0 }
}

#[macro_export]
macro_rules! EXECUTION_FAILURE{
    () => { 1 }
}

#[macro_export]
macro_rules! EX_USAGE {
    () => { 258 }
}

#[macro_export]
macro_rules! PARAMS {
    ($protos:expr) => {
        $protos
    };
}
pub unsafe fn hash_entries(ht: *mut HASH_TABLE) -> i32 {
    if ht != std::ptr::null_mut() {
        return (*ht).nentries;
    } else {
        return 0;
    }
}

#[macro_export]
macro_rules! HASH_ENTRIES {
    ($ht:expr) => {
        if ($ht) != std::ptr::null_mut(){
            return (*$ht).nentries;
        }
        else{
            return 0;
        }
    };
}

#[macro_export]
macro_rules! pathdata {
    ($x:expr) => {
        (*$x).data as *mut PATH_DATA
    };
}

#[macro_export]
macro_rules! FREE {
    ($s:expr) => {
        if $s != std::ptr::null_mut(){
            free($s as *mut c_void);
        }
    };
}

// type i32 hash_efunc PARAMS(*mut BUCKET_CONTENTS);
type hash_wfunc = extern  fn(*mut BUCKET_CONTENTS)->i32;
type sh_builtin_func_t = extern fn (*mut WORD_LIST)->i32;

//extern c
extern "C"{
    static loptend:*mut WORD_LIST;
    static hashing_enabled:i32;
    static list_optarg:*mut c_char;
    static this_command_name:*mut c_char;
    static posixly_correct:i32;
    static restricted:i32;
    static shell_compatibility_level:i32;
    static hashed_filenames:*mut HASH_TABLE;
    static dot_found_in_search:i32;

    fn builtin_error(format:*const c_char,...);
    fn reset_internal_getopt();
    fn internal_getopt(list:*mut WORD_LIST,opts:*mut c_char)->i32;
    fn builtin_usage();
    fn sh_needarg(s:*mut c_char);
    fn phash_flush();
    fn sh_restricted(s:*mut c_char);
    fn absolute_program(string:*const c_char)->i32;
    fn find_user_command(name:*const c_char)->*mut c_char;
    fn executable_file(file:*const c_char)->i32;
    fn sh_notfound(s:*mut c_char);
    fn is_directory(file:*const c_char)->i32;
    fn phash_insert(filename:*mut c_char,full_path:*mut c_char,check_dot:i32,found:i32);
    fn phash_remove(filename:*const c_char)->i32;
    fn find_function(name:*const c_char)->*mut SHELL_VAR;
    fn find_shell_builtin(name:*mut c_char)->*mut sh_builtin_func_t;
    fn hash_walk(table:*mut HASH_TABLE,func:*mut hash_wfunc);
    fn phash_search(filename:*const c_char)->*mut c_char;
    fn printable_filename(f:*mut c_char,flage:i32)->*mut c_char;
}

//rust
/* Print statistics on the current state of hashed commands.  If LIST is
   not empty, then rehash (or hash in the first place) the specified
   commands. */
#[no_mangle]
pub extern "C" fn r_hash_builtin(mut list:*mut WORD_LIST)->i32{
    let mut expunge_hash_table:i32;
    let mut list_targets:i32;
    let mut list_portably:i32;
    let mut delete:i32;
    let mut opt:i32;
    let mut w:*mut c_char;
    let mut pathname:*mut c_char;

    unsafe{
        if hashing_enabled == 0{
            let c_str = CString::new("hashing disabled").unwrap();
            let c_str_ptr = c_str.as_ptr();

            builtin_error(c_str_ptr);
            return EXECUTION_FAILURE!();
        }
        expunge_hash_table = 0;
        list_targets = 0;
        list_portably = 0;
        delete = 0;
        pathname = std::ptr::null_mut();

        reset_internal_getopt();
        let opts = CString::new("dlp:rt").unwrap();
        opt = internal_getopt(list,opts.as_ptr() as *mut c_char);
        while opt != -1{
            let optu8 = opt as u8;
            let opt_char = char::from(optu8);
            match opt_char{
                'd' => delete = 1,
                'l' => list_portably = 1,
                'p' => pathname = list_optarg,
                'r' => expunge_hash_table = 1,
                't' => list_targets = 1,
                 _  => {
                     builtin_usage();
                     return EX_USAGE!();
                 }

            }

            opt = internal_getopt(list,opts.as_ptr() as *mut c_char);
        }

        list = loptend;

        /* hash -t requires at least one argument. */
        if list == std::ptr::null_mut() && (delete != 0 || list_targets != 0) {
            let mut temp:*mut c_char;
            if delete != 0{
                temp = CString::new("-d").unwrap().as_ptr() as *mut c_char;
            }
            else{
                temp = CString::new("-t").unwrap().as_ptr() as *mut c_char;
            }
            sh_needarg(temp);
            return EXECUTION_FAILURE!();
        }
        
        /* We want hash -r to be silent, but hash -- to print hashing info, so
         we test expunge_hash_table. */
        if list==std::ptr::null_mut() && expunge_hash_table == 0{
            opt = r_print_hashed_commands(list_portably);
            if opt==0 && posixly_correct==0 && (list_portably==0 || shell_compatibility_level<=50){
                let s_cstr = CStr::from_ptr(this_command_name);
                let s_str = s_cstr.to_str().unwrap();
                let s_string = s_str.to_owned();
                println!("{}:hash table empty",s_string);
            }
            return EXECUTION_SUCCESS!();
        }

        if expunge_hash_table != 0{
            phash_flush();
        }

        /* If someone runs `hash -r -t xyz' he will be disappointed. */
        if list_targets != 0{
            return r_list_hashed_filename_targets(list,list_portably);
        }

        if restricted != 0 && pathname != std::ptr::null_mut(){
            if strchr(pathname,'/' as c_int) != std::ptr::null_mut(){
                sh_restricted(pathname);
                return EXECUTION_FAILURE!();
            }

            /* If we are changing the hash table in a restricted shell, make sure the
             target pathname can be found using a $PATH search. */
            w = find_user_command(pathname);
            if w==std::ptr::null_mut() || *w==0 || executable_file(w)==0{
                sh_notfound(pathname);
                free(w as *mut c_void);
                return EXECUTION_FAILURE!();
            }
            free(w as *mut c_void);
        }

        opt = EXECUTION_SUCCESS!();
        while list != std::ptr::null_mut(){
            /* Add, remove or rehash the specified commands. */
            w = (*(*list).word).word;
            if absolute_program(w as *const c_char) != 0{
                continue;
            }
            else if pathname != std::ptr::null_mut(){
                if is_directory(pathname) != 0{
                    let c_err = CString::new("%s:%s");
                    builtin_error(c_err.unwrap().as_ptr(),pathname,strerror(EISDIR));
                    let c_err = CString::new("%s: is a directory").unwrap().as_ptr();
                    builtin_error(c_err ,pathname,strerror(EISDIR));

                    opt = EXECUTION_FAILURE!();
                }
                else{
                    phash_insert(w,pathname,0,0);
                }
            }
            else if delete != 0{
                if phash_remove(w) != 0{
                    sh_notfound(w);
                    opt = EXECUTION_FAILURE!();
                }
            }
            else if r_add_hashed_command(w,0) != 0{
                opt = EXECUTION_FAILURE!();
            }
            list = (*list).next;
        }

        stdout().flush();
        return opt;
    }//unsafe
}

extern "C" fn r_add_hashed_command(w:*mut c_char,quiet:i32)->i32{
    let mut rv:i32;
    let full_path:*mut c_char;

    rv = 0;

    unsafe{
        // if find_function(w) == std::ptr::null_mut() && find_shell_builtin(w) == std::ptr::null_mut(){
        if find_function(w).is_null() && find_shell_builtin(w).is_null(){
            phash_remove(w);
            full_path = find_user_command(w);
            if full_path != std::ptr::null_mut() && executable_file(full_path) != 0{
                phash_insert(w,full_path,dot_found_in_search,0)
            }
            else{
                if quiet == 0{
                    sh_notfound(w);
                }
                rv += 1;
            }
            FREE!(full_path);
        }

        return rv;

    }//unsafe
}

extern "C" fn r_print_hash_info(item:*mut BUCKET_CONTENTS)->i32{
    
    unsafe{
        println!("{:04}\t{}",(*item).times_found,*(*pathdata!(item)).path);
    }//unsafe
    0
}

#[no_mangle]
extern "C" fn r_print_portable_hash_info(item:*mut BUCKET_CONTENTS)->i32{
    let fp:*mut c_char;
    let f:*mut c_char;

    unsafe{
        fp = printable_filename((*pathdata!(item)).path,1);
        f = printable_filename((*item).key,1);
        println!("builtin hash -p {} {}",*fp,*f);

        if fp != (*pathdata!(item)).path{
            free(fp as *mut c_void);
        }
        if f != (*item).key{
            free(f as *mut c_void);
        }
        return 0;
    }//unsafe
}

#[no_mangle]
extern "C" fn r_print_hashed_commands(fmt:i32)->i32{
    unsafe{
        if hashed_filenames.is_null() || hash_entries(hashed_filenames) == 0 {
            return 0;
        }
        if fmt == 0{
            println!("hits\tcommand");
        }
        let fmt_t:hash_wfunc;
        if fmt != 0{
            fmt_t = r_print_portable_hash_info;
        }
        else{
            fmt_t = r_print_hash_info;
        }
        hash_walk(hashed_filenames,fmt_t as *mut hash_wfunc);
        return 1;
    }
}

#[no_mangle]
extern "C" fn r_list_hashed_filename_targets(list:*mut WORD_LIST,fmt:i32)->i32{
    let mut all_found:i32;
    let multiple:i32;
    let mut target:*mut c_char;
    let mut l:*mut WORD_LIST;

    all_found = 1;
  
    unsafe{
        if !(*list).next.is_null(){
            multiple = 1;
        }
        else{
            multiple = 0;
        }

        l = list;
        while !l.is_null(){
            target = phash_search((*(*l).word).word);
            if target.is_null(){
                all_found = 0;
                sh_notfound((*(*l).word).word);
                continue;
            }
            if fmt != 0{
                println!("builtin hash -p {} {}",*target,*(*(*l).word).word)
            }
            else{
                if multiple != 0{
                    print!("{}\t",*((*(*l).word).word));
                }
                println!("{}",*target);
            }
            free(target as *mut c_void);
            l = (*l).next;
        }
        
        if all_found != 0{
            return EXECUTION_SUCCESS!();
        }
        else{
            return EXECUTION_FAILURE!();
        }
    }

}
