
extern crate libc;
use libc::{c_char,c_int, c_void, FILE, size_t, intmax_t,c_long, strcmp};
use libc::{isdigit,strerror, __errno_location, fflush, ferror,clearerr, free,strcpy,strlen,strncmp,atoi,qsort};
use std::ffi::{CStr, CString};
use std::mem::size_of;
use std::ptr::read_volatile;
use nix::errno::errno;
use std::env::var;
use unic_langid::LanguageIdentifier;
include!(concat!("lib_readline_keymaps.rs"));
include!(concat!("command.rs"));
use fluent_bundle::{FluentBundle, FluentResource, FluentValue, FluentArgs};
use fluent_resmgr::resource_manager::ResourceManager;

