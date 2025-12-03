/*
 * SPDX-FileCopyrightText: 2025 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: GPL-2.0-or-later
 */
pub use libc::*;

pub use crate::array::array_dispose_element;
pub use crate::array::array_rshift;
pub use crate::array::array_shift;
pub use crate::builtins::help::builtin_help;
pub use crate::builtins::read::sigalrm_seen;
pub use crate::general::file_isdir;
pub use crate::general::legal_number;
pub use crate::general::make_absolute;
pub use crate::general::same_file;
pub use crate::hashlib::hash_create;
pub use crate::jobs::describe_pid;
pub use crate::jobs::reap_dead_jobs;
pub use crate::jobs::stop_pipeline;
pub use crate::jobs::wait_for;
pub use crate::list::list_reverse;
pub use crate::pcomplete::prog_completion_enabled;
pub use crate::readline::environ;
pub use crate::readline::mbstowcs;
pub use crate::readline::rl_readline_state;
pub use crate::sig::termsig_handler;
pub use crate::sig::throw_to_top_level;
pub use crate::stringlib::xbcopy;
pub use crate::trap::signal_is_ignored;
pub use crate::trap::signal_is_trapped;
pub use crate::variables::find_variable;
pub use fluent_bundle::FluentArgs;
pub use fluent_resmgr::resource_manager::ResourceManager;
pub use std::env::var;
pub use std::ffi::*;
pub use std::fs::File;
pub use std::io::{Stdout, Write}; //去掉了stdout
pub use std::mem::{size_of, transmute};
pub use std::ops::Add;
pub use std::os::linux::fs::MetadataExt;
pub use std::path::Path;
pub use std::ptr::{null_mut, read_volatile};
pub use unic_langid::LanguageIdentifier;

/*
#[link(name = "glob", kind = "static")]
#[link(name = "test", kind = "static")]
#[link(name = "tilde", kind = "static")]
#[link(name = "readline", kind = "static")]
#[link(name = "sh", kind = "static")]
#[link(name = "termcap", kind = "static")]
#[link(name = "test", kind = "static")]
*/
extern "C" {
    pub static mut rl_completion_entry_function: Option<rl_compentry_func_t>;
    pub static mut rl_ignore_some_completions_function: Option<rl_compignore_func_t>;
    pub static mut rl_filename_quoting_desired: libc::c_int;
    pub static mut rl_filename_quoting_function: Option<rl_quote_func_t>;
    pub static mut rl_attempted_completion_function: Option<rl_completion_func_t>;
    pub static mut stdout: *mut libc::FILE;
    pub static mut stderr: *mut libc::FILE;
    pub static mut rl_instream: *mut libc::FILE;
    pub static mut rl_outstream: *mut libc::FILE;
    pub static is_basic_table: [libc::c_uint; 0];
    pub static mut history_expansion_char: libc::c_char;
    pub static mut history_quoting_state: libc::c_int;
    pub static mut history_quotes_inhibit_expansion: libc::c_int;
    pub static mut history_search_delimiter_chars: *mut libc::c_char;
    pub static mut history_inhibit_expansion_function: Option<rl_linebuf_func_t>;
    pub static mut history_lines_read_from_file: libc::c_int;
    pub static mut history_base: libc::c_int;
    pub static mut history_length: libc::c_int;
    pub static mut history_lines_written_to_file: libc::c_int;
    pub static mut history_subst_char: libc::c_char;
    pub static mut rl_dispatching: libc::c_int;
    pub static mut rl_done: libc::c_int;
    pub static mut history_max_entries: libc::c_int;
    pub static mut rl_num_chars_to_read: libc::c_int;
    pub static mut rl_startup_hook: Option<rl_hook_func_t>;
    #[link_name = "\u{1}num_shell_builtins"]
    pub static mut num_shell_builtins: ::std::os::raw::c_int;
    pub static mut glob_ignore_case: libc::c_int;
    #[link_name = "\u{1}shell_builtins"]
    pub static mut shell_builtins: *mut builtin;
    #[link_name = "\u{1}glob_error_return"]
    pub static mut glob_error_return: *mut ::std::os::raw::c_char;
    pub static mut glob_asciirange: i32;
    #[link_name = "\u{1}sourcelevel"]
    pub static mut sourcelevel: ::std::os::raw::c_int;
    pub static mut tilde_expansion_preexpansion_hook: Option<tilde_hook_func_t>;
    pub static mut sh_syntaxtab: [libc::c_int; 0];
    pub static mut sh_syntabsiz: libc::c_int;
    pub static mut gnu_error_format: libc::c_int;
    #[link_name = "\u{1}stdin"]
    pub static mut stdin: *mut FILE;

    pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    pub fn getrusage(__who: __rusage_who_t, __usage: *mut rusage) -> ::std::os::raw::c_int;
    pub fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> libc::c_int;
    pub fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    pub fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    pub fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    pub fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    pub fn sigismember(__set: *const sigset_t, __signo: libc::c_int) -> libc::c_int;
    pub fn sigdelset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    pub fn xfree(_: *mut libc::c_void);
    fn clearerr(__stream: *mut FILE);
    pub fn strlist_resize(_: *mut STRINGLIST, _: libc::c_int) -> *mut STRINGLIST;
    pub fn strlist_dispose(_: *mut STRINGLIST);
    pub fn strlist_append(_: *mut STRINGLIST, _: *mut STRINGLIST) -> *mut STRINGLIST;
    pub fn netopen(_: *mut libc::c_char) -> libc::c_int;
    pub fn sh_mktmpfd(
        _: *mut libc::c_char,
        _: libc::c_int,
        _: *mut *mut libc::c_char,
    ) -> libc::c_int;
    pub fn mbstrlen(_: *const libc::c_char) -> size_t;
    pub fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: i32,
    ) -> *mut libc::c_char;
    pub fn builtin_error(format: *const libc::c_char, ...);
    pub fn builtin_warning(format: *const libc::c_char, ...);
    pub fn sh_single_quote(_: *const libc::c_char) -> *mut libc::c_char;
    pub fn sh_double_quote(string: *const libc::c_char) -> *mut libc::c_char;
    pub fn sh_contains_shell_metas(_: *const libc::c_char) -> libc::c_int;
    pub fn qsort(
        __base: *mut libc::c_void,
        __nmemb: libc::size_t,
        __size: libc::size_t,
        __compar: __compar_fn_t,
    );
    pub fn strvec_len(arg1: *mut *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn mbschr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    pub fn fmtulong(
        _: libc::c_ulong,
        _: libc::c_int,
        _: *mut libc::c_char,
        _: size_t,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    pub fn match_pattern_wchar(_: *mut wchar_t, _: *mut wchar_t, _: libc::c_int) -> libc::c_int;
    pub fn wmatchlen(_: *mut wchar_t, _: size_t) -> libc::c_int;
    pub fn sh_stat(_: *const libc::c_char, _: *mut stat) -> libc::c_int;
    pub fn sh_quote_reusable(_: *mut libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    pub fn strvec_to_word_list(
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut WORD_LIST;
    pub fn get_host_type() -> *mut libc::c_char;
    pub fn get_os_type() -> *mut libc::c_char;
    pub fn get_mach_type() -> *mut libc::c_char;
    pub fn strvec_flush(_: *mut *mut libc::c_char);
    pub fn sbrand(_: libc::c_ulong);
    pub fn brand() -> libc::c_int;
    pub fn get_urandom32() -> libc::c_uint;
    pub fn parser_error(_: libc::c_int, _: *const libc::c_char, _: ...);
    pub fn parse_and_execute(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int;
    pub fn readline(p: *const libc::c_char) -> *mut libc::c_char;
    pub fn exit_shell(_: libc::c_int) -> !;
    pub fn ansiexpand(
        _: *mut libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_int,
    ) -> *mut libc::c_char;
    pub fn sh_mkdoublequoted(
        _: *const libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    pub fn sh_backslash_quote_for_double_quotes(_: *mut libc::c_char) -> *mut libc::c_char;
    pub fn get_current_user_info();
    pub fn utf8_mblen(_: *const libc::c_char, _: size_t) -> libc::c_int;
    pub fn seedrand();
    pub fn seedrand32();
    pub fn wcsmatch(_: *mut wchar_t, _: *mut wchar_t, _: libc::c_int) -> libc::c_int;
    pub fn umatchlen(_: *mut libc::c_char, _: size_t) -> libc::c_int;
    pub fn match_pattern_char(
        _: *mut libc::c_char,
        _: *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int;
    pub fn mbsmbchar(_: *const libc::c_char) -> *mut libc::c_char;
    pub fn xdupmbstowcs(
        _: *mut *mut wchar_t,
        _: *mut *mut *mut libc::c_char,
        _: *const libc::c_char,
    ) -> size_t;
    pub fn rl_set_keymap(map: Keymap);
    pub fn rl_named_function(string: *const libc::c_char) -> *mut rl_command_func_t;
    pub fn rl_invoking_keyseqs(function: *mut rl_command_func_t) -> *mut *mut libc::c_char;
    pub fn rl_unbind_function_in_map(func: *mut rl_command_func_t, map: Keymap) -> i32;
    pub fn rl_get_keymap() -> Keymap;
    pub fn rl_bind_keyseq(keyseq: *const libc::c_char, function: *mut rl_command_func_t) -> i32;
    pub fn rl_function_of_keyseq_len(
        keyseq: *const libc::c_char,
        len: size_t,
        map: Keymap,
        Type: *mut i32,
    ) -> Option<rl_command_func_t>;
    pub fn rl_translate_keyseq(
        seq: *const libc::c_char,
        array: *mut libc::c_char,
        len: *mut i32,
    ) -> i32;
    pub fn rl_get_keymap_by_name(name: *const libc::c_char) -> Keymap;
    pub fn rl_list_funmap_names();
    pub fn rl_function_dumper(print_readably: i32);
    pub fn rl_macro_dumper(print_readably: i32);
    pub fn rl_variable_dumper(print_readably: i32);
    pub fn rl_read_init_file(filename: *const libc::c_char) -> i32;
    pub fn rl_parse_and_bind(string: *mut libc::c_char) -> i32;
    pub fn strvec_search(array: *mut *mut libc::c_char, name: *mut libc::c_char) -> i32;
    pub fn strvec_dispose(array: *mut *mut libc::c_char);
    pub fn sh_modcase(
        _: *const libc::c_char,
        _: *mut libc::c_char,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    pub fn itos(_: intmax_t) -> *mut libc::c_char;
    pub fn strvec_create(_: libc::c_int) -> *mut *mut libc::c_char;
    pub fn inttostr(_: intmax_t, _: *mut libc::c_char, _: size_t) -> *mut libc::c_char;
    pub fn ansic_shouldquote(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn ansic_quote(
        arg1: *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
    pub fn err_readonly(_: *const libc::c_char);
    pub fn valid_nameref_value(_: *const libc::c_char, _: libc::c_int) -> libc::c_int;
    pub fn report_error(_: *const libc::c_char, _: ...);
    pub fn internal_warning(_: *const libc::c_char, _: ...);
    pub fn set_exit_status(_: libc::c_int);
    pub fn read_history(_: *const libc::c_char) -> libc::c_int;
    pub fn using_history();
    pub fn clear_history();
    pub fn remove_history(_: libc::c_int) -> *mut HIST_ENTRY;
    pub fn free_history_entry(_: *mut HIST_ENTRY) -> histdata_t;
    pub fn remove_history_range(_: libc::c_int, _: libc::c_int) -> *mut *mut HIST_ENTRY;
    pub fn history_list() -> *mut *mut HIST_ENTRY;
    pub fn history_get(_: libc::c_int) -> *mut HIST_ENTRY;
    pub fn where_history() -> libc::c_int;
    pub fn history_set_pos(_: libc::c_int) -> libc::c_int;
    pub fn append_history(_: libc::c_int, _: *const libc::c_char) -> libc::c_int;
    pub fn __errno_location() -> *mut libc::c_int;
    pub fn write_history(_: *const libc::c_char) -> libc::c_int;
    pub fn history_expand(_: *mut libc::c_char, _: *mut *mut libc::c_char) -> libc::c_int;
    pub fn previous_history() -> *mut HIST_ENTRY;
    pub fn replace_history_entry(
        _: libc::c_int,
        _: *const libc::c_char,
        _: histdata_t,
    ) -> *mut HIST_ENTRY;
    pub fn history_is_stifled() -> libc::c_int;
    pub fn add_history(_: *const libc::c_char);
    pub fn strmatch(_: *mut libc::c_char, _: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    pub fn internal_error(arg1: *const ::std::os::raw::c_char, ...);
    pub fn sh_physpath(path: *mut libc::c_char, flags: i32) -> *mut libc::c_char;
    pub fn sh_makepath(
        path: *const libc::c_char,
        dir: *const libc::c_char,
        flags: i32,
    ) -> *mut libc::c_char;
    pub fn dirspell(dirname: *mut libc::c_char) -> *mut libc::c_char;
    pub fn sh_canonpath(path: *mut libc::c_char, flags: i32) -> *mut libc::c_char;
    pub fn fpurge(stream: *mut FILE) -> i32;
    pub fn strvec_from_word_list(
        list: *mut WordList,
        alloc: i32,
        starting_index: i32,
        ip: *mut i32,
    ) -> *mut *mut libc::c_char;
    pub fn ansicstr(
        string: *mut libc::c_char,
        len: i32,
        flags: i32,
        sawc: *mut libc::c_int,
        rlen: *mut libc::c_int,
    ) -> *mut libc::c_char;
    pub fn fstat(
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    pub fn lstat(
        __path: *const libc::c_char,
        __statbuf: *mut crate::src_common::stat,
    ) -> libc::c_int;

    pub fn zmapfd(_: libc::c_int, _: *mut *mut libc::c_char, _: *mut libc::c_char) -> libc::c_int;
    pub fn uconvert(
        s: *mut libc::c_char,
        ip: *mut libc::c_long,
        up: *mut libc::c_long,
        ep: *mut *mut libc::c_char,
    ) -> libc::c_int;
    pub fn input_avail(arg1: libc::c_int) -> libc::c_int;
    pub fn rl_insert_text(p: *const libc::c_char) -> libc::c_int;
    pub fn zreadintr(arg1: libc::c_int, arg2: *mut libc::c_char, arg3: size_t) -> libc::ssize_t;
    pub fn zreadcintr(arg1: libc::c_int, arg2: *mut libc::c_char) -> libc::ssize_t;
    pub fn zread(_: libc::c_int, _: *mut libc::c_char, _: size_t) -> ssize_t;
    pub fn zreadn(arg1: libc::c_int, arg2: *mut libc::c_char, arg3: size_t) -> libc::ssize_t;
    pub fn zreadc(arg1: libc::c_int, arg2: *mut libc::c_char) -> libc::ssize_t;
    pub fn zsyncfd(fd: libc::c_int) -> libc::c_void;
    pub fn zreset();
    pub fn zgetline(
        arg1: ::std::os::raw::c_int,
        arg2: *mut *mut ::std::os::raw::c_char,
        arg3: *mut usize,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> isize;
    pub fn falarm(arg1: libc::c_uint, arg2: libc::c_uint) -> libc::c_uint;
    pub fn ttgetattr(arg1: libc::c_int, arg2: *mut libc::termios) -> libc::c_int;
    pub fn ttsetattr(arg1: libc::c_int, arg2: *mut libc::termios) -> libc::c_int;
    pub fn ttfd_noecho(arg1: libc::c_int, arg2: *mut libc::termios) -> libc::c_int;
    pub fn ttfd_cbreak(fd: libc::c_int, ttp: *mut libc::termios) -> libc::c_int;
    pub fn ttfd_onechar(fd: libc::c_int, ttp: *mut libc::termios) -> libc::c_int;
    pub fn programmable_completions(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
        arg5: *mut ::std::os::raw::c_int,
    ) -> *mut *mut ::std::os::raw::c_char;
    pub fn pcomp_set_readline_variables(arg1: ::std::os::raw::c_int, arg2: ::std::os::raw::c_int);
    pub fn fnx_fromfs(
        arg1: *mut ::std::os::raw::c_char,
        arg2: size_t,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strvec_strcmp(
        arg1: *mut *mut ::std::os::raw::c_char,
        arg2: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn glob_pattern_p(_: *const libc::c_char) -> libc::c_int;
    pub fn strvec_resize(
        arg1: *mut *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
    ) -> *mut *mut ::std::os::raw::c_char;
    // pub fn read_builtin(_: *mut WordList) -> libc::c_int;
    pub fn sh_backslash_quote(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
    pub fn zcatfd(_: libc::c_int, _: libc::c_int, _: *mut libc::c_char) -> libc::c_int;
    pub fn sh_mktmpfp(
        nameroot: *mut libc::c_char,
        flags: i32,
        namep: &mut *mut libc::c_char,
    ) -> *mut libc::FILE;
    pub fn __strtol_internal(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
        __group: libc::c_int,
    ) -> libc::c_long;
    pub fn strvec_mresize(
        arg1: *mut *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
    ) -> *mut *mut ::std::os::raw::c_char;
    pub fn asprintf(
        __ptr: *mut *mut ::std::os::raw::c_char,
        __fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
    pub fn strvec_mcreate(arg1: ::std::os::raw::c_int) -> *mut *mut ::std::os::raw::c_char;
    pub fn programming_error(arg1: *const ::std::os::raw::c_char, ...);
    pub fn stat(
            __filename: *const libc::c_char,
            __stat_buf: *mut stat,
        ) -> libc::c_int;
    pub fn sh_eaccess(_: *const libc::c_char, _: libc::c_int) -> libc::c_int;
    pub fn getmaxchild() -> i64;
    pub fn make_command_string(arg1: *mut COMMAND) -> *mut ::std::os::raw::c_char;
    pub fn siglongjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    pub fn sys_error(arg1: *const ::std::os::raw::c_char, ...);
    pub fn sh_exit(arg1: ::std::os::raw::c_int);
    pub fn optimize_fork(arg1: *mut COMMAND);
    pub fn wcswidth(__s: *const wchar_t, __n: usize) -> ::std::os::raw::c_int;
    pub fn putc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn subshell_exit(arg1: ::std::os::raw::c_int);
    pub fn sh_regmatch(
        a: *const libc::c_char,
        b: *const libc::c_char,
        c: libc::c_int,
    ) -> libc::c_int;
  
    pub fn sh_getopt_restore_istate(arg1: *mut sh_getopt_state_t);
    pub fn optimize_shell_function(arg1: *mut COMMAND);
    pub fn sh_getopt_save_istate() -> *mut sh_getopt_state_t;
  
    pub fn unset_bash_input(arg1: ::std::os::raw::c_int);
    pub fn unbind_args();
    pub fn bind_function_def(
        arg1: *const ::std::os::raw::c_char,
        arg2: *mut FUNCTION_DEF,
        arg3: ::std::os::raw::c_int,
    );
    pub fn bind_function(arg1: *const ::std::os::raw::c_char, arg2: *mut COMMAND)
        -> *mut SHELL_VAR;
    pub fn difftimeval(_: *mut timeval, _: *mut timeval, _: *mut timeval) -> *mut timeval;
    pub fn timeval_to_secs(tvp: *mut timeval, sp: *mut time_t, sfp: *mut libc::c_int);
    pub fn addtimeval(_: *mut timeval, _: *mut timeval, _: *mut timeval) -> *mut timeval;
    pub fn timeval_to_cpu(_: *mut timeval, _: *mut timeval, _: *mut timeval) -> libc::c_int;
    pub fn fmtumax(
        arg1: uintmax_t,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_char,
        arg4: usize,
        arg5: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
    pub fn imaxdiv(__numer: intmax_t, __denom: intmax_t) -> imaxdiv_t;
    pub fn get_new_window_size(_: libc::c_int, _: *mut libc::c_int, _: *mut libc::c_int);
    pub fn getmaxgroups() -> libc::c_int;
    pub fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    pub fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    pub fn __ctype_get_mb_cur_max() -> libc::size_t;
    pub fn get_string_value(_: *const libc::c_char) -> *mut libc::c_char;
    pub fn u32reset();
    pub fn mailstat(_: *const libc::c_char, _: *mut stat) -> libc::c_int;
    pub fn isnetconn(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn sh_setlinebuf(_: *mut FILE) -> libc::c_int;
    pub fn strlist_create(arg1: ::std::os::raw::c_int) -> *mut STRINGLIST;
    pub fn strlist_prefix_suffix(
        arg1: *mut STRINGLIST,
        arg2: *mut ::std::os::raw::c_char,
        arg3: *mut ::std::os::raw::c_char,
    ) -> *mut STRINGLIST;
    pub fn sh_contains_quotes(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn rl_complete_internal(_: libc::c_int) -> libc::c_int;
    pub fn rl_filename_completion_function(
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    pub fn set_signal_handler(arg1: libc::c_int, arg2: *mut SigHandler) -> *mut SigHandler;
    pub fn rl_complete(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    pub fn __mbrlen(__s: *const libc::c_char, __n: size_t, __ps: *mut mbstate_t) -> size_t;
    pub fn mbrtowc(
        __pwc: *mut wchar_t,
        __s: *const libc::c_char,
        __n: size_t,
        __p: *mut mbstate_t,
    ) -> size_t;
    pub fn __sigsetjmp(
        __env: *mut __jmp_buf_tag,
        __savemask: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn mblen(s: *const libc::c_char, n: size_t) -> libc::c_int;
}

#[no_mangle]
pub fn sh_xfree(arg: *mut libc::c_void, unval1: *const libc::c_char, unval2: libc::c_int) {
    unsafe {
        free(arg as *mut libc::c_void);
    }
}
#[no_mangle]
pub fn sh_xmalloc(
    arg: libc::c_ulong,
    unval1: *const libc::c_char,
    unval2: libc::c_int,
) -> *mut libc::c_void {
    unsafe { malloc(arg as usize) }
}
#[no_mangle]
pub fn sh_xrealloc(
    arg: *mut libc::c_void,
    unval1: libc::c_ulong,
    unval2: *const libc::c_char,
    unval3: libc::c_int,
) -> *mut libc::c_void {
    //    realloc(arg as *mut libc::c_void,unval1:libc::c_ulong)
    unsafe { realloc(arg as *mut libc::c_void, unval1 as libc::size_t) }
}

pub static mut parse_and_execute_level: libc::c_int = 0 as libc::c_int;

pub static mut expand_word_fatal: WORD_LIST = WORD_LIST {
    next: 0 as *const word_list as *mut word_list,
    word: 0 as *const WORD_DESC as *mut WORD_DESC,
};
pub static mut expand_wdesc_error: WORD_DESC = WORD_DESC {
    word: 0 as *const libc::c_char as *mut libc::c_char,
    flags: 0,
};
pub static mut expand_wdesc_fatal: WORD_DESC = WORD_DESC {
    word: 0 as *const libc::c_char as *mut libc::c_char,
    flags: 0,
};

pub static mut expand_word_error: WORD_LIST = WORD_LIST {
    next: 0 as *const word_list as *mut word_list,
    word: 0 as *const WORD_DESC as *mut WORD_DESC,
};

#[no_mangle]
pub static mut no_exit_on_failed_exec: libc::c_int = 0;
#[no_mangle]
pub unsafe fn STREQ(a: *const libc::c_char, b: *const libc::c_char) -> bool {
    return *a == *b && libc::strcmp(a, b) == 0;
}

#[no_mangle]
pub static mut primary_prompt: *mut libc::c_char =
    b"\\s-\\v\\$ \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut secondary_prompt: *mut libc::c_char =
    b"> \0" as *const u8 as *const libc::c_char as *mut libc::c_char;

pub static mut original_signals: [Option<SigHandler>; 65] = [None; 65];
pub const CHAR_BIT: libc::c_int = 8 as libc::c_int;

/* Used by some builtins and the mainline code. */
pub static mut last_shell_builtin: Option<sh_builtin_func_t> = None;
pub static mut this_shell_builtin: Option<sh_builtin_func_t> = None;

#[macro_export]
macro_rules! GETOPT_HELP {
    () => {
        -99
    };
}

//break_1.rs
/* The depth of while's and until's. */
#[no_mangle]
pub static mut loop_level: libc::c_int = 0 as libc::c_int;
/* Non-zero when a "break" instruction is encountered. */
#[no_mangle]
pub static mut breaking: libc::c_int = 0 as libc::c_int;
/* Non-zero when we have encountered a continue instruction. */
#[no_mangle]
pub static mut continuing: libc::c_int = 0 as libc::c_int;
pub type SizeT = libc::c_ulong;
pub static AL_REUSABLE: i32 = 0x01;

//bashgetopt
#[no_mangle]
pub static mut list_optarg: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut list_optopt: libc::c_int = 0;
#[no_mangle]
pub static mut list_opttype: libc::c_int = 0;
#[no_mangle]
pub static mut lcurrent: *mut WordList =
    0 as *const libc::c_void as *mut libc::c_void as *mut WordList;
#[no_mangle]
pub static mut loptend: *mut WordList = 0 as *const WordList as *mut WordList;

//bind
#[repr(C)]
pub struct _keymap_entry {
    pub Type: libc::c_char,
    pub function: Option<rl_command_func_t>,
}
pub type KEYMAP_ENTRY = _keymap_entry;
pub type Keymap = *mut KEYMAP_ENTRY;

//emun
#[macro_export]
macro_rules! LFLAG {
    () => {
        0x0001
    };
}
#[macro_export]
macro_rules! PFLAG {
    () => {
        0x0002
    };
}
#[macro_export]
macro_rules! FFLAG {
    () => {
        0x0004
    };
}
#[macro_export]
macro_rules! VFLAG {
    () => {
        0x0008
    };
}
#[macro_export]
macro_rules! QFLAG {
    () => {
        0x0010
    };
}
#[macro_export]
macro_rules! MFLAG {
    () => {
        0x0020
    };
}
#[macro_export]
macro_rules! RFLAG {
    () => {
        0x0040
    };
}
#[macro_export]
macro_rules! PPFLAG {
    () => {
        0x0080
    };
}
#[macro_export]
macro_rules! VVFLAG {
    () => {
        0x0100
    };
}
#[macro_export]
macro_rules! SFLAG {
    () => {
        0x0200
    };
}
#[macro_export]
macro_rules! SSFLAG {
    () => {
        0x0400
    };
}
#[macro_export]
macro_rules! UFLAG {
    () => {
        0x0800
    };
}
#[macro_export]
macro_rules! XFLAG {
    () => {
        0x1000
    };
}
#[macro_export]
macro_rules! XXFLAG {
    () => {
        0x2000
    };
}

#[macro_export]
macro_rules! ISKMAP {
    () => {
        1
    };
}

#[macro_export]
macro_rules! KEYMAP_SIZE {
    () => {
        257
    };
}

#[macro_export]
macro_rules! ANYOTHERKEY {
    () => {
        KEYMAP_SIZE!() - 1
    };
}

//caller
#[macro_export]
macro_rules! att_cell {
    ($var:expr) => {
        return (*($var).value) as *mut ARRAY;
    };
}

#[macro_export]
macro_rules! CHECK_HELPOPT {
    ($l:expr) => {
        if $l != std::ptr::null_mut()
            && (*($l)).word != std::ptr::null_mut()
            && ISHELP!((*(*($l)).word).word) == true
        {
            builtin_help();
            return EX_USAGE as i32;
        }
    };
}

//cd
#[repr(C)]
pub union VALUE_COMMAND {
    For: *mut for_com,
    Case: *mut case_com,
    While: *mut while_com,
    If: *mut if_com,
    Connection: *mut connection,
    Simple: *mut simple_com,
    Function_def: *mut function_def,
    Group: *mut group_com,
    Select: *mut select_com,
    Arith: *mut arith_com,
    Cond: *mut cond_com,
    ArithFor: *mut arith_for_com,
    Subshell: *mut subshell_com,
    Coproc: *mut coproc_com,
}

#[macro_export]
macro_rules! DUP_JOB {
    () => {
        -2
    };
}

#[macro_export]
macro_rules! LCD_DOVARS {
    () => {
        0x001
    };
}
#[macro_export]
macro_rules! LCD_DOSPELL {
    () => {
        0x002
    };
}
#[macro_export]
macro_rules! LCD_PRINTPATH {
    () => {
        0x004
    };
}

#[macro_export]
macro_rules! LCD_FREEDIRNAME {
    () => {
        0x008
    };
}

#[macro_export]
macro_rules! MP_DOTILDE {
    () => {
        0x01
    };
}

pub static mut xattrfd: i32 = -1;
pub static mut xattrflag: i32 = 0;
pub static mut verbatim_pwd: i32 = 0;
pub static mut eflag: i32 = 0;

//command
pub const const_command_builtin: *mut libc::c_char =
    b"command_builtin\0" as *const u8 as *const libc::c_char as *mut libc::c_char;

pub const CMD_WANT_SUBSHELL: i32 = 0x01;
pub const CMD_FORCE_SUBSHELL: i32 = 0x02;
pub const CMD_INVERT_RETURN: i32 = 0x04;
pub const CMD_IGNORE_RETURN: i32 = 0x08;
pub const CMD_NO_FUNCTIONS: i32 = 0x10;
pub const CMD_INHIBIT_EXPANSION: i32 = 0x20;
pub const CMD_NO_FORK: i32 = 0x40;
pub const CMD_TIME_PIPELINE: i32 = 0x80;
pub const CMD_TIME_POSIX: i32 = 0x100;
pub const CMD_AMPERSAND: i32 = 0x200;
pub const CMD_STDIN_REDIR: i32 = 0x400;
pub const CMD_COMMAND_BUILTIN: i32 = 0x0800;
pub const CMD_COPROC_SUBSHELL: i32 = 0x1000;
pub const CMD_LASTPIPE: i32 = 0x2000;
pub const CMD_STDPATH: i32 = 0x4000;
pub const CMD_TRY_OPTIMIZING: i32 = 0x8000;

//common.rs
#[macro_export]
macro_rules! SUBSHELL_PAREN {
    () => {
        0x02
    };
}

#[macro_export]
macro_rules! EXECUTION_SUCCESS {
    () => {
        0
    };
}

#[macro_export]
macro_rules! DISCARD {
    () => {
        2
    };
}

#[macro_export]
macro_rules! ARGS_INVOC {
    () => {
        0x01
    };
}

#[macro_export]
macro_rules! ARGS_FUNC {
    () => {
        0x02
    };
}

#[macro_export]
macro_rules! ARGS_SETBLTIN {
    () => {
        0x04
    };
}

#[macro_export]
macro_rules! EX_BADUSAGE {
    () => {
        2
    };
}

#[macro_export]
macro_rules! DEBUG_TRAP {
    () => {
        NSIG!() + 1
    };
}

#[macro_export]
macro_rules! NSIG {
    () => {
        64
    };
}

#[macro_export]
macro_rules! NO_JOB {
    () => {
        -1
    };
}

#[macro_export]
macro_rules! JM_SUBSTRING {
    () => {
        0x02
    };
}

#[macro_export]
macro_rules! JM_EXACT {
    () => {
        0x04
    };
}

#[macro_export]
macro_rules! JM_STOPPED {
    () => {
        0x08
    };
}

#[macro_export]
macro_rules! JM_FIRSTMATCH {
    () => {
        0x10
    };
}

#[macro_export]
macro_rules! VA_NOEXPAND {
    () => {
        0x001
    };
}

#[macro_export]
macro_rules! VA_ONEWORD {
    () => {
        0x002
    };
}

#[macro_export]
macro_rules! ASS_NOEXPAND {
    () => {
        0x0080
    };
}

#[macro_export]
macro_rules! att_invisible {
    () => {
        0x0001000
    };
}

#[macro_export]
macro_rules! att_nounset {
    () => {
        0x0002000
    };
}

#[macro_export]
macro_rules! SPECIAL_BUILTIN {
    () => {
        0x08
    };
}

#[macro_export]
macro_rules! BUILTIN_ENABLED {
    () => {
        0x01
    };
}

#[macro_export]
macro_rules! BUILTIN_DELETED {
    () => {
        0x02
    };
}

#[macro_export]
macro_rules! DSIG_SIGPREFIX {
    () => {
        0x01
    };
}

#[macro_export]
macro_rules! DSIG_NOCASE {
    () => {
        0x02
    };
}

#[macro_export]
macro_rules! NO_SIG {
    () => {
        -1
    };
}

#[macro_export]
macro_rules! ISOCTAL {
    ($c:expr) => {
        ($c) >= b'0' as libc::c_char && ($c) <= b'7' as libc::c_char
    };
}

#[macro_export]
macro_rules! J_JOBSTATE {
    ($j:expr) => {
        (*$j).state
    };
}

//type
pub static EX_SUCCESS: i32 = 0;

/* Structure containing all the non-action (binary) options; filled in by
build_actions(). */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _optflags {
    pub pflag: libc::c_int,
    pub rflag: libc::c_int,
    pub Dflag: libc::c_int,
    pub Eflag: libc::c_int,
    pub Iflag: libc::c_int,
}

#[macro_export]
macro_rules! CA_ALIAS {
    () => {
        1 << 0
    };
}

#[macro_export]
macro_rules! CA_ARRAYVAR {
    () => {
        1 << 1
    };
}

#[macro_export]
macro_rules! CA_BINDING {
    () => {
        1 << 2
    };
}

#[macro_export]
macro_rules! CA_BUILTIN {
    () => {
        1 << 3
    };
}

#[macro_export]
macro_rules! CA_COMMAND {
    () => {
        1 << 4
    };
}

#[macro_export]
macro_rules! CA_DIRECTORY {
    () => {
        1 << 5
    };
}

#[macro_export]
macro_rules! CA_DISABLED {
    () => {
        1 << 6
    };
}

#[macro_export]
macro_rules! CA_ENABLED {
    () => {
        1 << 7
    };
}

#[macro_export]
macro_rules! CA_EXPORT {
    () => {
        1 << 8
    };
}

#[macro_export]
macro_rules! CA_FILE {
    () => {
        1 << 9
    };
}

#[macro_export]
macro_rules! CA_FUNCTION {
    () => {
        1 << 10
    };
}

#[macro_export]
macro_rules! CA_GROUP {
    () => {
        1 << 11
    };
}

#[macro_export]
macro_rules! CA_HELPTOPIC {
    () => {
        1 << 12
    };
}

#[macro_export]
macro_rules! CA_HOSTNAME {
    () => {
        1 << 13
    };
}

#[macro_export]
macro_rules! CA_JOB {
    () => {
        1 << 14
    };
}

#[macro_export]
macro_rules! CA_KEYWORD {
    () => {
        1 << 15
    };
}

#[macro_export]
macro_rules! CA_RUNNING {
    () => {
        1 << 16
    };
}

#[macro_export]
macro_rules! CA_SERVICE {
    () => {
        1 << 17
    };
}

#[macro_export]
macro_rules! CA_SETOPT {
    () => {
        1 << 18
    };
}

#[macro_export]
macro_rules! CA_SHOPT {
    () => {
        1 << 19
    };
}

#[macro_export]
macro_rules! CA_SIGNAL {
    () => {
        1 << 20
    };
}

#[macro_export]
macro_rules! CA_STOPPED {
    () => {
        1 << 21
    };
}

#[macro_export]
macro_rules! CA_USER {
    () => {
        1 << 22
    };
}

#[macro_export]
macro_rules! CA_VARIABLE {
    () => {
        1 << 23
    };
}

#[macro_export]
macro_rules! COPT_RESERVED {
    () => {
        1 << 0
    };
}

#[macro_export]
macro_rules! COPT_DEFAULT {
    () => {
        1 << 1
    };
}

#[macro_export]
macro_rules! COPT_FILENAMES {
    () => {
        1 << 2
    };
}

#[macro_export]
macro_rules! COPT_DIRNAMES {
    () => {
        1 << 3
    };
}

#[macro_export]
macro_rules! COPT_NOQUOTE {
    () => {
        1 << 4
    };
}

#[macro_export]
macro_rules! COPT_NOSPACE {
    () => {
        1 << 5
    };
}

#[macro_export]
macro_rules! COPT_BASHDEFAULT {
    () => {
        1 << 6
    };
}

#[macro_export]
macro_rules! COPT_PLUSDIRS {
    () => {
        1 << 7
    };
}

#[macro_export]
macro_rules! COPT_NOSORT {
    () => {
        1 << 8
    };
}

pub static mut Garg: *mut libc::c_char = std::ptr::null_mut();
pub static mut Warg: *mut libc::c_char = std::ptr::null_mut();
pub static mut Parg: *mut libc::c_char = std::ptr::null_mut();
pub static mut Sarg: *mut libc::c_char = std::ptr::null_mut();
pub static mut Xarg: *mut libc::c_char = std::ptr::null_mut();
pub static mut Farg: *mut libc::c_char = std::ptr::null_mut();
pub static mut Carg: *mut libc::c_char = std::ptr::null_mut();

//declare
#[macro_export]
macro_rules! EXITPROG {
    () => {
        3
    };
}
#[macro_export]
macro_rules! ERREXIT {
    () => {
        4
    };
}
#[macro_export]
macro_rules! FORCE_EOF {
    () => {
        1
    };
}
#[macro_export]
macro_rules! att_local {
    () => {
        0x0000020
    };
}

#[macro_export]
macro_rules! att_assoc {
    () => {
        0x0000040 /* variable is an associative array */
    };
}

#[macro_export]
macro_rules! att_function {
    () => {
        0x0000008 /* value is a function */
    };
}

#[macro_export]
macro_rules! att_integer {
    () => {
        0x0000010 /* internal representation is int */
    };
}

#[macro_export]
macro_rules! att_trace {
    () => {
        0x0000080 /* function is traced with DEBUG trap */
    };
}

#[macro_export]
macro_rules! att_exported {
    () => {
        0x0000001 /* export to environment */
    };
}

#[macro_export]
macro_rules! att_capcase {
    () => {
        0x0000400 /* word capitalized on assignment */
    };
}

#[macro_export]
macro_rules! att_uppercase {
    () => {
        0x0000100 /* word converted to uppercase on assignment */
    };
}

#[macro_export]
macro_rules! att_lowercase {
    () => {
        0x0000200 /* word converted to lowercase on assignment */
    };
}

#[macro_export]
macro_rules! FUNC_MULTILINE {
    () => {
        0x01
    };
}

#[macro_export]
macro_rules! FUNC_EXTERNAL {
    () => {
        0x02
    };
}

#[macro_export]
macro_rules! ASS_FORCE {
    () => {
        0x0020 /* force assignment even to readonly variable */
    };
}

#[macro_export]
macro_rules! W_COMPASSIGN {
    () => {
        1 << 15 /* Compound assignment */
    };
}

#[macro_export]
macro_rules! att_propagate {
    () => {
        0x0200000 /* propagate to previous scope */
    };
}

#[macro_export]
macro_rules! ASS_NAMEREF {
    () => {
        0x0010 /* assigning to nameref variable */
    };
}

#[macro_export]
macro_rules! VALID_ECHO_OPTIONS {
    () => {
        CString::new("neE").unwrap().as_ptr()
    };
}

//enable
pub const ENABLED: i32 = 1;
pub const DISABLED: i32 = 2;
pub const SPECIAL: i32 = 4;

pub const AFLAG: i32 = 0x01;
pub const DFLAG: i32 = 0x02;
pub const FFLAG: i32 = 0x04;
pub const NFLAG: i32 = 0x08;
pub const PFLAG: i32 = 0x10;
pub const SFLAG: i32 = 0x20;

//history.rs
pub const SFLAG_H: i32 = 0x10;
pub const PFLAG_H: i32 = 0x20;
pub const DFLAG_H: i32 = 0x80;

pub const RTLD_LAZY: i32 = 0x00001;
pub const RTLD_NOW: i32 = 0x00002;
pub const RTLD_BINDING_MASK: i32 = 0x3;
pub const RTLD_NOLOAD: i32 = 0x00004;
pub const RTLD_DEEPBIND: i32 = 0x00008;
pub const RTLD_GLOBAL: i32 = 0x00100;

//eval
#[macro_export]
macro_rules! SEVAL_NOHIST {
    () => {
        0x004
    };
}

//exit
#[macro_export]
macro_rules! SYS_BASH_LOGOOUT {
    () => {
        CString::new(" \"/etc/utshell.utshell_logout\" ")
            .unwrap()
            .as_ptr()
    };
}

//fc
#[repr(C)]
pub struct REPL {
    pub next: *mut REPL,
    pub pat: *mut libc::c_char,
    pub rep: *mut libc::c_char,
}

#[macro_export]
macro_rules! ISHELP {
    ($s:expr) => {
        STREQ!(
            $s as *const libc::c_char,
            CString::new("--help").unwrap().as_ptr()
        )
    };
}

#[macro_export]
macro_rules! HN_LISTING {
    () => {
        0x01
    };
}

#[macro_export]
macro_rules! SUBSHELL_COMSUB {
    () => {
        0x04
    };
}

#[macro_export]
macro_rules! HN_FIRST {
    () => {
        0x02
    };
}

#[macro_export]
macro_rules! HIST_INVALID {
    () => {
        std::i32::MIN
    };
}

#[macro_export]
macro_rules! HIST_ERANGE {
    () => {
        std::i32::MIN + 1
    };
}

#[macro_export]
macro_rules! HIST_NOTFOUND {
    () => {
        std::i32::MIN + 2
    };
}

#[macro_export]
macro_rules! MT_USETMPDIR {
    () => {
        0x0001
    };
}

#[macro_export]
macro_rules! MT_READWRITE {
    () => {
        0x0002
    };
}

#[macro_export]
macro_rules! MT_USERANDOM {
    () => {
        0x0004
    };
}

#[macro_export]
macro_rules! MT_TEMPLATE {
    () => {
        0x0008
    };
}

pub union Functions {
    pub f_unlink: fn(t: *const libc::c_char) -> i32,
    pub f_xfree: fn(str1: *mut libc::c_void),
    pub f_set_verbose: fn(),
    pub restore_funcarray_state: fn(*mut func_array_state) -> (),
    pub pop_args: fn(),
    pub set_current_prompt_level: fn(libc::c_int) -> (),
    pub pop_stream: fn() -> (),
    pub parser_restore_alias: fn() -> (),
    pub xfree: fn(*mut libc::c_void) -> (),
    pub dispose_fd_bitmap: fn(*mut fd_bitmap) -> (),
    pub dispose_command: fn(*mut COMMAND) -> (),
    pub set_history_remembering: fn() -> (),
    pub restore_lastcom: fn(*mut libc::c_char) -> (),
}

//fg_bg
#[macro_export]
macro_rules! J_JOBCONTROL {
    () => {
        0x04
    };
}

//getopts
#[macro_export]
macro_rules! EX_MISCERROR {
    () => {
        2
    };
}

#[macro_export]
macro_rules! G_EOF {
    () => {
        -1
    };
}

#[macro_export]
macro_rules! G_INVALID_OPT {
    () => {
        -2
    };
}

#[macro_export]
macro_rules! G_ARG_MISSING {
    () => {
        -3
    };
}

type PTR_T = c_void;

//enum
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

fn HASH_ENTRIES(ht: *mut HASH_TABLE) -> i32 {
    unsafe {
        if ht != std::ptr::null_mut() {
            return (*ht).nentries;
        } else {
            return 0;
        }
    }
}
#[macro_export]
macro_rules! pathdata {
    ($x:expr) => {
        (*$x).data as *mut PATH_DATA
    };
}

//help
#[repr(C)]
struct FieldStruct {
    name: *mut libc::c_char,
}

#[macro_export]
macro_rules! EX_USAGE {
    () => {
        258
    };
}

#[macro_export]
macro_rules! BASE_INDENT {
    () => {
        4
    };
}

#[macro_export]
macro_rules! BUILTIN_SIZEOF {
    () => {
        48
    };
}

#[macro_export]
macro_rules! EXIT_FAILURE {
    () => {
        1
    };
}

//history
pub const RFLAG: libc::c_int = 0x02;
pub const WFLAG: libc::c_int = 0x04;
pub const CFLAG: libc::c_int = 0x40;

//jobs
#[macro_export]
macro_rules! JLIST_STANDARD {
    () => {
        0
    };
}

#[macro_export]
macro_rules! JSTATE_ANY {
    () => {
        0x0
    };
}

#[macro_export]
macro_rules! JLIST_LONG {
    () => {
        1
    };
}

#[macro_export]
macro_rules! JLIST_PID_ONLY {
    () => {
        2
    };
}

#[macro_export]
macro_rules! JLIST_CHANGED_ONLY {
    () => {
        3
    };
}

#[macro_export]
macro_rules! JSTATE_RUNNING {
    () => {
        0x1
    };
}

#[macro_export]
macro_rules! JSTATE_STOPPED {
    () => {
        0x2
    };
}

#[macro_export]
macro_rules! CMD_INHIBIT_EXPANSION {
    /* Do not expand the command words. */
    () => {
        0x20
    };
}

//mapfile
pub const atype_array_indexed: atype = 0;

pub const att_readonly: libc::c_int = 0x0000002;
pub const att_array: libc::c_int = 0x0000004;
pub const att_invisible: libc::c_int = 0x0001000;
pub const att_noassign: libc::c_int = 0x0004000;

pub const MAPF_CLEARARRAY: libc::c_int = 0x01;
pub const MAPF_CHOP: libc::c_int = 0x02;

//printf
pub type mbstate_t = __mbstate_t;

macro_rules! IS_DIGITAL {
    ($x: expr) => {
        $x >= b'0' as libc::c_char && $x <= b'9' as libc::c_char
    };
}

//pushd
#[macro_export]
macro_rules! NOCD {
    () => {
        0x01
    };
}

#[macro_export]
macro_rules! ROTATE {
    () => {
        0x02
    };
}

#[macro_export]
macro_rules! LONGFORM {
    () => {
        0x04
    };
}

#[macro_export]
macro_rules! CLEARSTAK {
    () => {
        0x08
    };
}

pub static mut pushd_directory_list: *mut *mut libc::c_char = std::ptr::null_mut();
pub static mut directory_list_offset: i32 = 0;
pub static mut directory_list_size: i32 = 0;

//read
pub const ISFUNC: libc::c_int = 0;

#[no_mangle]
pub static mut alrmbuf: sigjmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];

#[derive(Clone, Copy)]
pub struct tty_save {
    pub fd: i32,
    pub attrs: libc::termios,
}

//set
#[macro_export]
macro_rules! FLAG_UNKNOWN {
    () => {
        0 as *mut i32
    };
}

#[macro_export]
macro_rules! MINUS_O_FORMAT {
    () => {
        CString::new("%-15s\t%s\n")
    };
}

#[macro_export]
macro_rules! GET_BINARY_O_OPTION_VALUE {
    ($a:expr,$b:expr) => {
        if (o_options[$a as usize].get_func).is_some() {
            (Some((o_options[$a as usize].get_func).expect("non-null function pointer")))
                .expect("non-null function pointer")($b)
        } else {
            *o_options[$a as usize].variable
        }
    };
}

#[macro_export]
macro_rules! SET_BINARY_O_OPTION_VALUE {
    ($a:expr,$onoff:expr,$c:expr) => {
        if (o_options[$a as usize].set_func).is_some() {
            (Some((o_options[$a as usize].set_func).expect("non-null function pointer")))
                .expect("non-null function pointer")($onoff, $c)
        } else {
            $onoff == FLAG_ON!();
            let b = $onoff;
            *o_options[$a as usize].variable = b;
            *o_options[$a as usize].variable
        }
    };
}

#[macro_export]
macro_rules! N_O_OPTIONS {
    () => {
        (std::mem::size_of::<[opp; 28]>() as usize / std::mem::size_of::<opp>() as usize)
    };
}

#[macro_export]
macro_rules! FLAG_ON {
    () => {
        b'-' as i32
    };
}

#[macro_export]
macro_rules! FLAG_OFF {
    () => {
        b'+' as i32
    };
}

#[macro_export]
macro_rules! att_imported {
    () => {
        0x0008000
    };
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct opp {
    pub name: *mut libc::c_char,
    pub letter: i32,
    pub variable: *mut i32,
    pub set_func: Option<setopt_set_func_t>,
    pub get_func: Option<setopt_get_func_t>,
}

#[macro_export]
macro_rules! FLAG_ERROR {
    () => {
        -1
    };
}

type setopt_set_func_t = fn(i: i32, name: *mut libc::c_char) -> i32;
type setopt_get_func_t = fn(name: *mut libc::c_char) -> i32;

//setattr
pub const att_exported: libc::c_int = 0x0000001;
pub const att_function: libc::c_int = 0x0000008;
pub const att_integer: libc::c_int = 0x0000010;
pub const att_local: libc::c_int = 0x0000020;
pub const att_assoc: libc::c_int = 0x0000040;
pub const att_trace: libc::c_int = 0x0000080;
pub const att_uppercase: libc::c_int = 0x0000100;
pub const att_lowercase: libc::c_int = 0x0000200;
pub const att_capcase: libc::c_int = 0x0000400;
pub const att_nameref: libc::c_int = 0x0000800;
pub const att_imported: libc::c_int = 0x0008000;
pub const att_tempvar: libc::c_int = 0x0100000;
pub const att_propagate: libc::c_int = 0x0200000;

//shift
pub static mut print_shift_error: libc::c_int = 0;

//shopt
pub type IntmaxT = libc::c_long;
pub type ArrayindT = intmax_t;

pub type ShVarAssignFuncT =
    fn(*mut variable, *mut libc::c_char, ArrayindT, *mut libc::c_char) -> *mut variable;
pub type ShVarValueFuncT = fn(*mut variable) -> *mut variable;
pub type ShellVar = variable;

//trap
pub const SET: libc::c_int = 0;
pub const REVERT: libc::c_int = 1;
pub const IGNORE: libc::c_int = 2;

//type_1
#[macro_export]
macro_rules! CDESC_ALL {
    //print all descriptions of a command
    () => {
        0x001
    };
}

#[macro_export]
macro_rules! CDESC_SHORTDESC {
    //print the description for type and command -V
    () => {
        0x002
    };
}

#[macro_export]
macro_rules! CDESC_REUSABLE {
    //print in a format that may be reused as input
    () => {
        0x004
    };
}

#[macro_export]
macro_rules! CDESC_TYPE {
    //print the type for type -t
    () => {
        0x008
    };
}

#[macro_export]
macro_rules! CDESC_PATH_ONLY {
    //print the path for type -p
    () => {
        0x010
    };
}

#[macro_export]
macro_rules! CDESC_FORCE_PATH {
    //force a path search for type -P
    () => {
        0x020
    };
}

#[macro_export]
macro_rules! CDESC_NOFUNCS {
    //skip function lookup for type -f
    () => {
        0x040
    };
}

#[macro_export]
macro_rules! CDESC_ABSPATH {
    //CDESC_ABSPATH
    () => {
        0x080
    };
}

#[macro_export]
macro_rules! CDESC_STDPATH {
    () => {
        0x100
    };
}

#[macro_export]
macro_rules! FS_EXECABLE {
    () => {
        0x2
    };
}
#[macro_export]
macro_rules! FS_EXEC_PREFERRED {
    () => {
        0x4
    };
}

#[macro_export]
macro_rules! FS_NODIRS {
    () => {
        0x20
    };
}

#[macro_export]
macro_rules! MP_DOCWD {
    () => {
        0
    };
}

#[macro_export]
macro_rules! MP_RMDOT {
    () => {
        1
    };
}

#[macro_export]
macro_rules! SIZEOFWORD {
    () => {
        std::mem::size_of::<WordDesc>()
    };
}

pub fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    // 通过函数指针调用函数
    op(a, b)
}

#[macro_export]
macro_rules! FS_EXEC_ONLY {
    () => {
        0x8
    };
}

//ulimit
#[macro_export]
macro_rules! SIZEOFLIMIT {
    () => {
        std::mem::size_of::<RESOURCE_LIMITS>() as usize
    };
}

#[macro_export]
macro_rules! SIZEOFLIMITS {
    () => {
        SIZEOFLIMIT!() * 18
    };
}

#[macro_export]
macro_rules! SIZEOFULCMD {
    () => {
        std::mem::size_of::<cmdlist>()
    };
}

#[macro_export]
macro_rules! LIMIT_HARD {
    () => {
        0x01
    };
}

#[macro_export]
macro_rules! LIMIT_SOFT {
    () => {
        0x02
    };
}

#[macro_export]
macro_rules! POSIXBLK {
    () => {
        -2
    };
}

#[macro_export]
macro_rules! BLOCKSIZE {
    ($s:expr) => {
        if $s == POSIXBLK!() {
            if unsafe { posixly_correct != 0 } {
                512
            } else {
                1024
            }
        } else {
            $s
        }
    };
}

#[macro_export]
macro_rules! RLIM_SAVED_MAX {
    () => {
        RLIM_INFINITY!()
    };
}

#[macro_export]
macro_rules! NCMDS {
    () => {
        SIZEOFLIMITS!() / SIZEOFLIMIT!()
    };
}

#[macro_export]
macro_rules! RLIMIT_FILESIZE {
    () => {
        1
    };
}

#[macro_export]
macro_rules! RLIMIT_PIPESIZE {
    () => {
        257
    };
}

#[macro_export]
macro_rules! PIPESIZE {
    () => {
        4096
    };
}

#[macro_export]
macro_rules! PIPE_BUF {
    () => {
        PIPESIZE!()
    };
}

#[macro_export]
macro_rules! RLIMIT_OPENFILES {
    () => {
        7
    };
}

#[macro_export]
macro_rules! RLIMIT_VIRTMEM {
    () => {
        9
    };
}

#[macro_export]
macro_rules! RLIMIT_MAXUPROC {
    () => {
        6
    };
}

#[macro_export]
macro_rules! RLIM_INFINITY {
    () => {
        -1
        //0x7fffffff
    };
}

#[macro_export]
macro_rules! RLIM_SAVED_CUR {
    () => {
        RLIM_INFINITY!()
    };
}

pub const __RLIM_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_RTTIME: __rlimit_resource = 15;
pub const __RLIMIT_RTPRIO: __rlimit_resource = 14;
pub const __RLIMIT_NICE: __rlimit_resource = 13;
pub const __RLIMIT_MSGQUEUE: __rlimit_resource = 12;
pub const __RLIMIT_SIGPENDING: __rlimit_resource = 11;
pub const __RLIMIT_LOCKS: __rlimit_resource = 10;
pub const __RLIMIT_MEMLOCK: __rlimit_resource = 8;
pub const __RLIMIT_NPROC: __rlimit_resource = 6;
pub const RLIMIT_AS: __rlimit_resource = 9;
pub const __RLIMIT_OFILE: __rlimit_resource = 7;
pub const RLIMIT_NOFILE: __rlimit_resource = 7;
pub const __RLIMIT_RSS: __rlimit_resource = 5;
pub const RLIMIT_CORE: __rlimit_resource = 4;
pub const RLIMIT_STACK: __rlimit_resource = 3;
pub const RLIMIT_DATA: __rlimit_resource = 2;
pub const RLIMIT_FSIZE: __rlimit_resource = 1;
pub const RLIMIT_CPU: __rlimit_resource = 0;

//umask
#[macro_export]
macro_rules! mode_t {
    () => {
        u32
    };
}

#[macro_export]
macro_rules! S_IREAD {
    () => {
        0o0400
    };
}

#[macro_export]
macro_rules! S_IWRITE {
    () => {
        0o0200
    };
}

#[macro_export]
macro_rules! S_IEXEC {
    () => {
        0o0100
    };
}

#[macro_export]
macro_rules! S_IRUSR {
    /* read, owner */
    () => {
        S_IREAD!()
    };
}

#[macro_export]
macro_rules! S_IWUSR {
    /* write, owner */
    () => {
        S_IWRITE!()
    };
}

#[macro_export]
macro_rules! S_IXUSR {
    /* execute, owner */
    () => {
        S_IEXEC!()
    };
}

#[macro_export]
macro_rules! S_IRGRP {
    /* read, group */
    () => {
        S_IREAD!() >> 3
    };
}

#[macro_export]
macro_rules! S_IWGRP {
    /* write, group */
    () => {
        S_IWRITE!() >> 3
    };
}

#[macro_export]
macro_rules! S_IXGRP {
    /* execute, group */
    () => {
        S_IEXEC!() >> 3
    };
}

#[macro_export]
macro_rules! S_IROTH {
    /* read, other */
    () => {
        S_IREAD!() >> 6
    };
}

#[macro_export]
macro_rules! S_IWOTH {
    /* write, other */
    () => {
        S_IWRITE!() >> 6
    };
}

#[macro_export]
macro_rules! S_IXOTH {
    /* execute, other */
    () => {
        S_IEXEC!() >> 6
    };
}

#[macro_export]
macro_rules! S_IRWXU {
    () => {
        S_IRUSR!() | S_IWUSR!() | S_IXUSR!()
    };
}

#[macro_export]
macro_rules! S_IRWXG {
    () => {
        S_IRGRP!() | S_IWGRP!() | S_IXGRP!()
    };
}

#[macro_export]
macro_rules! S_IRWXO {
    () => {
        S_IROTH!() | S_IWOTH!() | S_IXOTH!()
    };
}

#[macro_export]
macro_rules! S_IRUGO {
    () => {
        S_IRUSR!() | S_IRGRP!() | S_IROTH!()
    };
}

#[macro_export]
macro_rules! S_IWUGO {
    () => {
        S_IWUSR!() | S_IWGRP!() | S_IWOTH!()
    };
}

#[macro_export]
macro_rules! S_IXUGO {
    () => {
        S_IXUSR!() | S_IXGRP!() | S_IXOTH!()
    };
}

//wait
#[macro_export]
macro_rules! J_WAITING {
    () => {
        0x08
    };
}

#[macro_export]
macro_rules! JWAIT_FORCE {
    () => {
        1 << 1
    };
}

#[macro_export]
macro_rules! JWAIT_WAITING {
    () => {
        1 << 3
    };
}

#[macro_export]
macro_rules! JWAIT_PEROOR {
    () => {
        1 << 0
    };
}

pub type procenv_t = __jmp_buf_tag;

//evalstring
#[macro_export]
macro_rules! CMD_INVERT_RETURN {
    () => {
        0x4
    };
}
#[macro_export]
macro_rules! CMD_TIME_PIPELINE {
    () => {
        0x80
    };
}
#[macro_export]
macro_rules! AND_AND {
    () => {
        288
    };
}
#[macro_export]
macro_rules! OR_OR {
    () => {
        289
    };
}
#[macro_export]
macro_rules! CMD_TRY_OPTIMIZING {
    () => {
        0x8000
    };
}
#[macro_export]
macro_rules! CMD_NO_FORK {
    () => {
        0x40
    };
}
#[macro_export]
macro_rules! SEVAL_NONINT {
    () => {
        0x1
    };
}
#[macro_export]
macro_rules! SEVAL_INTERACT {
    () => {
        0x2
    };
}
#[macro_export]
macro_rules! SEVAL_RESETLINE {
    () => {
        0x10
    };
}
#[macro_export]
macro_rules! SEVAL_PARSEONLY {
    () => {
        0x20
    };
}
#[macro_export]
macro_rules! SEVAL_ONECMD {
    () => {
        0x100
    };
}
#[macro_export]
macro_rules! CMD_IGNORE_RETURN {
    () => {
        0x8
    };
}
#[macro_export]
macro_rules! SEVAL_NOHISTEXP {
    () => {
        0x200
    };
}

//evalfile
#[macro_export]
macro_rules! FEVAL_ENOENTOK {
    () => {
        0x1
    };
}
#[macro_export]
macro_rules! FEVAL_BUILTIN {
    () => {
        0x2
    };
}
#[macro_export]
macro_rules! FEVAL_UNWINDPROT {
    () => {
        0x4
    };
}
#[macro_export]
macro_rules! FEVAL_NONINT {
    () => {
        0x8
    };
}
#[macro_export]
macro_rules! FEVAL_NOPUSHARGS {
    () => {
        0x100
    };
}
#[macro_export]
macro_rules! FEVAL_LONGJMP {
    () => {
        0x10
    };
}
#[macro_export]
macro_rules! FEVAL_HISTORY {
    () => {
        0x20
    };
}
#[macro_export]
macro_rules! FEVAL_REGFILE {
    () => {
        0x80
    };
}
#[macro_export]
macro_rules! yacc_EOF {
    () => {
        304
    };
}
#[macro_export]
macro_rules! FEVAL_CHECKBINARY {
    () => {
        0x40
    };
}
#[macro_export]
macro_rules! SSIZE_MAX {
    () => {
        9223372036854775807
    };
}
#[macro_export]
macro_rules! __S_IFREG {
    () => {
        0o100000
    };
}

#[no_mangle]
pub static mut sh_optarg: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut sh_optind: libc::c_int = 0 as libc::c_int;
pub static mut sh_curopt: libc::c_int = 0;
pub static mut nextchar: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut sh_charindex: libc::c_int = 0;
#[no_mangle]
pub static mut sh_opterr: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut sh_optopt: libc::c_int = '?' as i32;
#[no_mangle]
pub static mut sh_badopt: libc::c_int = 0 as libc::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct word_desc {
    pub word: *mut libc::c_char,
    pub flags: libc::c_int,
}
pub type WordDesc = word_desc;
pub type WORD_DESC = word_desc;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct word_list {
    pub next: *mut word_list,
    pub word: *mut WordDesc,
}
pub type WordList = word_list;
pub type WORD_LIST = word_list;

//alias
pub const _ISalpha: libc::c_uint = 1024;

pub type __compar_fn_t = Option<fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>;

pub type QSFUNC = fn(*const libc::c_void, *const libc::c_void) -> libc::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _list_of_strings {
    pub list: *mut *mut libc::c_char,
    pub list_size: libc::c_int,
    pub list_len: libc::c_int,
}
pub type STRINGLIST = _list_of_strings;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct bucket_contents {
    pub next: *mut bucket_contents,
    pub key: *mut libc::c_char,
    pub data: *mut libc::c_void,
    pub khash: libc::c_uint,
    pub times_found: libc::c_int,
}
pub type BUCKET_CONTENTS = bucket_contents;
pub type BucketContents = bucket_contents;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table {
    pub bucket_array: *mut *mut BUCKET_CONTENTS,
    pub nbuckets: libc::c_int,
    pub nentries: libc::c_int,
}
pub type HASH_TABLE = hash_table;
pub type HashTable = hash_table;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alias {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub flags: libc::c_char,
}
pub type alias_t = alias;
pub type AliasT = alias;
pub type ITEMLIST = _list_of_items;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _list_of_items {
    pub flags: libc::c_int,
    pub list_getter: Option<fn(*mut _list_of_items) -> libc::c_int>,
    pub slist: *mut STRINGLIST,
    pub genlist: *mut STRINGLIST,
    pub genindex: libc::c_int,
}
pub type sh_alias_map_func_t = fn(*mut alias_t) -> libc::c_int;
#[no_mangle]
pub static mut alias_expand_all: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut aliases: *mut HASH_TABLE =
    0 as *const libc::c_void as *mut libc::c_void as *mut HASH_TABLE;

pub const AL_EXPANDNEXT: libc::c_int = 0x1;
pub const HASH_NOSRCH: libc::c_int = 0x1;
pub const AL_BEINGEXPANDED: libc::c_int = 0x2;

//array
pub const array_indexed: atype = 0;

pub type sh_ae_map_func_t = fn(arg1: *mut ARRAY_ELEMENT, arg2: *mut libc::c_void) -> libc::c_int;

#[macro_export]
macro_rules! element_forw {
    ($ae:expr) => {
        (*$ae).next
    };
}

#[macro_export]
macro_rules! element_back {
    ($ae:expr) => {
        (*$ae).prev
    };
}

#[macro_export]
macro_rules! element_index {
    ($ae:expr) => {
        (*$ae).ind
    };
}

#[macro_export]
macro_rules! element_value {
    ($ae:expr) => {
        (*$ae).value
    };
}

#[macro_export]
macro_rules! ADD_BEFORE {
    ($ae:expr,$new:expr) => {
        (*(*$ae).prev).next = $new;
        (*$new).prev = (*$ae).prev;
        (*$ae).prev = $new;
        (*$new).next = $ae;
    };
}

#[macro_export]
macro_rules! ADD_AFTER {
    ($ae:expr,$new:expr) => {
        (*(*$ae).next).prev = $new;
        (*$new).next = (*$ae).next;
        (*$new).prev = $ae;
        (*$ae).next = $new;
    };
}

#[macro_export]
macro_rules! REVERSE_LIST {
    ($list:expr,$type:ty) => {
        if !$list.is_null() && !((*$list).next).is_null() {
            list_reverse($list as *mut GENERIC_LIST) as $type
        } else {
            $list as $type
        }
    };
}

#[macro_export]
macro_rules! array_empty {
    ($a:expr) => {
        (*$a).num_elements == 0
    };
}
#[macro_export]
macro_rules! LASTREF {
    ($a:expr) => {
        if !(*$a).lastref.is_null() {
            (*$a).lastref
        } else {
            element_forw!((*$a).head)
        }
    };
}

#[macro_export]
macro_rules! SET_LASTREF {
    ($a:expr,$e:expr) => {
        (*$a).lastref = $e
    };
}

#[macro_export]
macro_rules! INVALIDATE_LASTREF {
    ($a:expr) => {
        (*$a).lastref = 0 as *mut array_element
    };
}

#[macro_export]
macro_rules! array_max_index {
    ($a:expr) => {
        (*$a).max_index
    };
}

#[macro_export]
macro_rules! array_first_index {
    ($a:expr) => {
        (*(*(*$a).head).next).ind
    };
}

#[macro_export]
macro_rules! array_num_elements {
    ($a:expr) => {
        (*$a).num_elements
    };
}

#[macro_export]
macro_rules! array_head {
    ($a:expr) => {
        (*$a).head
    };
}

#[macro_export]
macro_rules! STRLEN {
    ($s:expr) => {
        if !$s.is_null() && *$s.offset(0 as isize) as libc::c_int != 0 {
            if *$s.offset(1 as isize) as libc::c_int != 0 {
                if *$s.offset(2 as isize) as libc::c_int != 0 {
                    libc::strlen($s) as i32
                } else {
                    2
                }
            } else {
                1
            }
        } else {
            0
        }
    };
}

#[macro_export]
macro_rules! RESIZE_MALLOCED_BUFFER {
    ($srt:expr,$cind:expr, $room:expr, $csize:expr, $sincr:expr) => {
        if $cind + $room >= $csize {
            while $cind + $room >= $csize {
                $csize += $sincr;
            }
            $srt = libc::realloc($srt as *mut libc::c_void, $csize as usize) as *mut libc::c_char;
        }
    };
}

//arrayfunc
pub const W_ASSIGNMENT: i32 = 1 << 2;

pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: libc::c_int,
    pub __value: mbstate_t_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union mbstate_t_value {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
}

pub type __intmax_t = libc::c_long;

pub type intmax_t = __intmax_t;
pub type arrayind_t = intmax_t;
pub type atype = libc::c_uint;
pub const array_assoc: atype = 1;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct array {
    pub type_0: atype,
    pub max_index: arrayind_t,
    pub num_elements: libc::c_int,
    pub head: *mut array_element,
    pub lastref: *mut array_element,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array_element {
    pub ind: arrayind_t,
    pub value: *mut libc::c_char,
    pub next: *mut array_element,
    pub prev: *mut array_element,
}
pub type ARRAY = array;
pub type ARRAY_ELEMENT = array_element;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub exportstr: *mut libc::c_char,
    pub dynamic_value: Option<sh_var_value_func_t>,
    pub assign_func: Option<sh_var_assign_func_t>,
    pub attributes: libc::c_int,
    pub context: libc::c_int,
}
pub type sh_var_assign_func_t =
    fn(*mut variable, *mut libc::c_char, arrayind_t, *mut libc::c_char) -> *mut variable;
pub type sh_var_value_func_t = fn(*mut variable) -> *mut variable;
pub type SHELL_VAR = variable;

#[macro_export]
macro_rules! value_cell {
    ($var:expr) => {
        (*$var).value
    };
}
#[macro_export]
macro_rules! FREE {
    ($s:expr) => {
        if ($s) != std::ptr::null_mut() {
            libc::free($s as *mut libc::c_void);
        }
    };
}

#[macro_export]
macro_rules! var_setarray {
    ($var:expr,$arr:expr) => {
        (*$var).value = $arr as *mut libc::c_char;
        (*$var).value
    };
}
#[macro_export]
macro_rules! INVALIDATE_EXPORTSTR {
    ($var:expr) => {
        if !((*$var).exportstr.is_null()) {
            libc::free((*$var).exportstr as *mut libc::c_void);
            (*$var).exportstr = 0 as *mut libc::c_void as *mut libc::c_char;
        }
    };
}
#[macro_export]
macro_rules! exported_p {
    ($var:expr) => {
        (*$var).attributes & (att_exported!())
    };
}

#[macro_export]
macro_rules! VUNSETATTR {
    ($var:expr,$attr:expr) => {
        (*$var).attributes &= !($attr) as libc::c_int;
        (*$var).attributes
    };
}
#[macro_export]
macro_rules! assoc_create {
    ($var:expr) => {
        hash_create($var)
    };
}
#[macro_export]
macro_rules! var_setassoc {
    ($var:expr,$arr:expr) => {
        (*$var).value = $arr as *mut libc::c_char;
    };
}
#[macro_export]
macro_rules! assoc_p {
    ($var:expr) => {
        (*$var).attributes & (att_assoc as libc::c_int)
    };
}
#[macro_export]
macro_rules! assoc_cell {
    ($var:expr) => {
        (*$var).value as *mut hash_table
    };
}
#[macro_export]
macro_rules! array_cell {
    ($var:expr) => {
        (*$var).value as *mut ARRAY
    };
}
#[macro_export]
macro_rules! INVALID_NAMEREF_VALUE {
    () => {
        &mut nameref_invalid_value as *mut SHELL_VAR as *mut libc::c_void as *mut SHELL_VAR
    };
}
#[macro_export]
macro_rules! nameref_p {
    ($var:expr) => {
        (*$var).attributes & att_nameref as libc::c_int
    };
}
#[macro_export]
macro_rules! nameref_cell {
    ($var:expr) => {
        (*$var).value
    };
}
#[macro_export]
macro_rules! readonly_p {
    ($var:expr) => {
        (*$var).attributes & att_readonly as libc::c_int
    };
}
#[macro_export]
macro_rules! noassign_p {
    ($var:expr) => {
        (*$var).attributes & att_noassign as libc::c_int
    };
}

#[macro_export]
macro_rules! ALL_ELEMENT_SUB {
    ($c:expr) => {
        $c == '@' as i32 || $c == '*' as i32
    };
}

#[macro_export]
macro_rules! invisible_p {
    ($var:expr) => {
        (*$var).attributes & att_invisible as libc::c_int
    };
}
#[macro_export]
macro_rules! integer_p {
    ($var:expr) => {
        (*$var).attributes & att_integer as libc::c_int
    };
}

#[macro_export]
macro_rules! isifs {
    ($c:expr) => {
        *ifs_cmap.as_mut_ptr().offset($c as libc::c_uchar as isize) as libc::c_int
    };
}
#[macro_export]
macro_rules! LBRACK {
    () => {
        '['
    };
}
#[macro_export]
macro_rules! RBRACK {
    () => {
        ']'
    };
}

#[macro_export]
macro_rules! INDEX_ERROR {
    ($var: expr, $t: expr, $s: expr) => {
        if !$var.is_null() {
            err_badarraysub((*$var).name);
        } else {
            *$t.offset(-(1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
            err_badarraysub($s);
            *$t.offset(-(1 as libc::c_int) as isize) = '[' as i32 as libc::c_char;
        }
        return 0 as *mut libc::c_void as *mut libc::c_char;
    };
}
#[macro_export]
macro_rules! var_isset {
    ($var:expr) => {
        (*$var).value != 0 as *mut libc::c_char
    };
}

/* This variable means to not expand associative array subscripts more than
once, when performing variable expansion. */
#[no_mangle]
pub static mut assoc_expand_once: libc::c_int = 0 as libc::c_int;

/* Ditto for indexed array subscripts -- currently unused */
#[no_mangle]
pub static mut array_expand_once: libc::c_int = 0 as libc::c_int;

/* Standard error message to use when encountering an invalid array subscript */
#[no_mangle]
pub static mut bash_badsub_errmsg: *const libc::c_char =
    b"bad array subscript\0" as *const u8 as *const libc::c_char;

//assoc
#[macro_export]
macro_rules! assoc_empty {
    ($h:expr) => {
        (*$h).nentries == 0 as libc::c_int
    };
}

#[macro_export]
macro_rules! hash_items {
    ($bucket:expr,$table:expr) => {
        if !$table.is_null() && $bucket < (*$table).nbuckets {
            *((*$table).bucket_array).offset($bucket as isize)
        } else {
            0 as *mut libc::c_void as *mut BUCKET_CONTENTS
        };
    };
}

//bashhist
pub type rl_linebuf_func_t = fn(*mut libc::c_char, libc::c_int) -> libc::c_int;

#[macro_export]
macro_rules! HISTSIZE_DEFAULT {
    () => {
        b"500\0" as *const u8 as *mut libc::c_char
    };
}

#[macro_export]
macro_rules! HIGN_EXPAND {
    () => {
        0x01
    };
}

#[macro_export]
macro_rules! ENOENT {
    () => {
        2
    };
}

#[macro_export]
macro_rules! errno {
    () => {
        *__errno_location()
    };
}

#[macro_export]
macro_rules! whitespace {
    ($c:expr) => {
        ($c as libc::c_int == ' ' as i32 || $c as libc::c_int == '\t' as i32)
    };
}

#[macro_export]
macro_rules! FNM_NOMATCH {
    () => {
        1
    };
}

pub const st_stdin: stream_type = 1;

pub type histdata_t = *mut libc::c_void;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _hist_entry {
    pub line: *mut libc::c_char,
    pub timestamp: *mut libc::c_char,
    pub data: histdata_t,
}
pub type HIST_ENTRY = _hist_entry;

#[no_mangle]
pub static mut remember_on_history: libc::c_int = 0;
#[no_mangle]
pub static mut enable_history_list: libc::c_int = -1;
#[no_mangle]
pub static mut history_lines_this_session: libc::c_int = 0;
#[no_mangle]
pub static mut history_lines_in_file: libc::c_int = 0;
#[no_mangle]
pub static mut history_expansion_inhibited: libc::c_int = 0;
#[no_mangle]
pub static mut double_quotes_inhibit_history_expansion: libc::c_int = 0;
#[no_mangle]
pub static mut command_oriented_history: libc::c_int = 1;
#[no_mangle]
pub static mut current_command_first_line_saved: libc::c_int = 0;
#[no_mangle]
pub static mut current_command_line_comment: libc::c_int = 0;
#[no_mangle]
pub static mut literal_history: libc::c_int = 0;
#[no_mangle]
pub static mut force_append_history: libc::c_int = 0;
#[no_mangle]
pub static mut history_control: libc::c_int = 0;
#[no_mangle]
pub static mut hist_last_line_added: libc::c_int = 0;
#[no_mangle]
pub static mut hist_last_line_pushed: libc::c_int = 0;
#[no_mangle]
pub static mut history_reediting: libc::c_int = 0;
#[no_mangle]
pub static mut hist_verify: libc::c_int = 0;
#[no_mangle]
pub static mut dont_save_function_defs: libc::c_int = 0;

//bashline
pub type rl_hook_func_t = fn() -> libc::c_int;
pub type rl_command_func_t = fn(libc::c_int, libc::c_int) -> libc::c_int;

#[macro_export]
macro_rules! STREQN {
    ($a:expr,$b:expr,$n:expr) => {
        if $n == 0 {
            1
        } else {
            (*$a.offset(0 as isize) == *$b.offset(0 as isize)
                && libc::strncmp($a, $b, $n as usize) == 0) as i32
        }
    };
}

#[macro_export]
macro_rules! STREQ {
    ($a:expr, $b:expr) => {
        (*$a.offset(0 as libc::c_int as isize) as libc::c_int
            == *$b.offset(0 as libc::c_int as isize) as libc::c_int
            && strcmp($a, $b) == 0 as libc::c_int)
    };
}

#[no_mangle]
pub static mut bash_readline_initialized: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut perform_hostname_completion: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut no_empty_command_completion: libc::c_int = 0;
#[no_mangle]
pub static mut force_fignore: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut dircomplete_spelling: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut dircomplete_expand: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut dircomplete_expand_relpath: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut complete_fullquote: libc::c_int = 1 as libc::c_int;
pub static mut bash_completer_word_break_characters: *mut libc::c_char =
    b" \t\n\"'@><=;|&(:\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
pub static mut bash_nohostname_word_break_characters: *mut libc::c_char =
    b" \t\n\"'><=;|&(:\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
pub static mut default_filename_quote_characters: *const libc::c_char =
    b" \t\n\\\"'@<>=;|&()#$`?*[!:{~\0" as *const u8 as *const libc::c_char;
pub static mut custom_filename_quote_characters: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
pub static mut filename_bstab: [libc::c_char; 256] = [0; 256];
pub static mut old_rl_startup_hook: Option<rl_hook_func_t> = unsafe {
    ::std::mem::transmute::<*mut libc::c_void, Option<rl_hook_func_t>>(
        0 as *const libc::c_void as *mut libc::c_void,
    )
};
pub static mut dot_in_path: libc::c_int = 0 as libc::c_int;
pub static mut dabbrev_expand_active: libc::c_int = 0 as libc::c_int;
pub static mut completion_quoting_style: libc::c_int = 3 as libc::c_int;
pub static mut vi_tab_binding: Option<rl_command_func_t> = unsafe {
    Some(std::mem::transmute::<
        unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int,
        fn(libc::c_int, libc::c_int) -> libc::c_int,
    >(rl_complete))
};

#[macro_export]
macro_rules! control_character_mask {
    () => {
        0x1f
    };
}

#[macro_export]
macro_rules! CTRL {
    ($c:expr) => {
        $c & control_character_mask!()
    };
}

#[macro_export]
macro_rules! cr_whitespace {
    ($c:expr) => {
        $c as libc::c_int == '\r' as i32 || $c as libc::c_int == '\n' as i32 || whitespace!($c)
    };
}

#[macro_export]
macro_rules! DIGIT {
    ($c:expr) => {
        $c as libc::c_int >= '0' as i32 && $c as libc::c_int <= '9' as i32
    };
}

#[macro_export]
macro_rules! RL_BOOLEAN_VARIABLE_VALUE {
    ($c:expr) => {
        (*$c.offset(0 as isize) as libc::c_int == 'o' as i32
            && *$c.offset(1 as isize) as libc::c_int == 'n' as i32
            && *$c.offset(2 as isize) as libc::c_int == '\u{0}' as i32)
    };
}

#[macro_export]
macro_rules! POSIX_VI_EDIT_COMMAND {
    () => {
        b"fc -e vi\0" as *const u8 as *const libc::c_char as *mut libc::c_char
    };
}

#[macro_export]
macro_rules! VI_EDIT_COMMAND {
    () => {
        b"fc -e \"${VISUAL:-${EDITOR:-vi}}\"\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char
    };
}

#[macro_export]
macro_rules! EMACS_EDITING_MODE {
    () => {
        1
    };
}

#[macro_export]
macro_rules! EMACS_EDIT_COMMAND {
    () => {
        b"fc -e \"${VISUAL:-${EDITOR:-emacs}}\"\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char
    };
}

#[macro_export]
macro_rules! DECLARE_MBSTATE {
    ($state:expr) => {
        memset(
            &mut $state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            (::std::mem::size_of::<mbstate_t>() as usize),
        )
    };
}

//brace
#[no_mangle]
pub fn mbrlen(mut __s: *const libc::c_char, mut __n: size_t, mut __ps: *mut mbstate_t) -> size_t {
    unsafe {
        return if !__ps.is_null() {
            mbrtowc(0 as *mut libc::wchar_t as *mut i32, __s, __n, __ps)
        } else {
            __mbrlen(__s, __n, 0 as *mut mbstate_t)
        };
    }
}

#[macro_export]
macro_rules! ADVANCE_CHAR {
    ($_str:expr, $_strsize:expr, $_i:expr,$state:expr) => {
        if locale_mb_cur_max > 1 {
            let mut state_bak: mbstate_t = mbstate_t {
                __count: 0,
                __value: mbstate_t_value { __wch: 0 },
            };
            let mut mblength: size_t = 0;
            let mut _f: libc::c_int = 0;

            _f = is_basic(*$_str.offset($_i as isize));
            if _f != 0 {
                mblength = 1 as size_t;
            } else if locale_utf8locale != 0
                && *$_str.offset($_i as isize) as libc::c_int & 0x80 as libc::c_int
                    == 0 as libc::c_int
            {
                mblength =
                    (*$_str.offset($_i as isize) as libc::c_int != 0) as libc::c_int as size_t;
            } else {
                state_bak = $state;
                mblength = mbrlen(
                    $_str.offset($_i as isize),
                    $_strsize.wrapping_sub($_i as libc::c_ulong),
                    &mut $state,
                ) as u64;
            }
            if mblength == -(2 as libc::c_int) as size_t
                || mblength == -(1 as libc::c_int) as size_t
            {
                $state = state_bak;
                $_i += 1;
            } else if mblength == 0 as libc::c_ulong {
                $_i += 1;
            } else {
                $_i = ($_i as libc::c_ulong).wrapping_add(mblength) as libc::c_int;
            }
        } else {
            $_i += 1;
        }
    };
}

#[macro_export]
macro_rules! WORDDELIM {
    ($c:expr) => {
        (*sh_syntaxtab
            .as_mut_ptr()
            .offset($c as libc::c_uchar as isize)
            & 0x1 as libc::c_int
            != 0
            || *sh_syntaxtab
                .as_mut_ptr()
                .offset($c as libc::c_uchar as isize)
                & 0x2000 as libc::c_int
                != 0)
    };
}

#[macro_export]
macro_rules! COMMAND_SEPARATORS {
    () => {
        b";|&{(`\0" as *const u8 as *const libc::c_char as *mut libc::c_char
    };
}

#[macro_export]
macro_rules! member {
    ($c:expr,$s:expr) => {
        (if *$c as libc::c_int != 0 {
            (mbschr($s, $c as libc::c_int) != 0 as *mut libc::c_void as *mut libc::c_char)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) != 0
    };
}

#[macro_export]
macro_rules! INITIALWORD {
    () => {
        b"_InitialWorD_\0" as *const u8 as *const libc::c_char
    };
}

#[macro_export]
macro_rules! EMPTYCMD {
    () => {
        b"_EmptycmD_\0" as *const u8 as *const libc::c_char
    };
}

#[macro_export]
macro_rules! DEFCOMP_CMDPOS {
    () => {
        1
    };
}

#[macro_export]
macro_rules! CMD_IS_DIR {
    ($x:expr) => {
        absolute_pathname($x) == 0 as libc::c_int
            && absolute_program($x) == 0 as libc::c_int
            && *$x as libc::c_int != '~' as i32
            && test_for_directory($x) != 0
    };
}

#[macro_export]
macro_rules! TAB {
    () => {
        '\t' as i32
    };
}

#[macro_export]
macro_rules! UNDO_BEGIN {
    () => {
        2
    };
}

#[macro_export]
macro_rules! UNDO_END {
    () => {
        3
    };
}

#[macro_export]
macro_rules! Q_HERE_DOCUMENT {
    () => {
        0x002
    };
}

#[macro_export]
macro_rules! PATH_CHECKDOTDOT {
    () => {
        0x0001
    };
}

#[macro_export]
macro_rules! PATH_CHECKEXISTS {
    () => {
        0x0002
    };
}

#[macro_export]
macro_rules! GLOB_FAILED {
    ($glist:expr) => {
        $glist == &mut glob_error_return as *mut *mut libc::c_char
    };
}

#[macro_export]
macro_rules! CBSDQUOTE {
    () => {
        0x0040
    };
}

#[macro_export]
macro_rules! COMPLETE_BSQUOTE {
    () => {
        3
    };
}

#[macro_export]
macro_rules! COMPLETE_SQUOTE {
    () => {
        2
    };
}

#[macro_export]
macro_rules! COMPLETE_DQUOTE {
    () => {
        1
    };
}

#[macro_export]
macro_rules! MB_CUR_MAX {
    ($s:expr) => {
        if !$s.is_null() && *$s.offset(0 as isize) as libc::c_int != 0 {
            if *$s.offset(1 as isize) as libc::c_int != 0 {
                mbstrlen($s) as usize
            } else {
                1 as usize
            }
        } else {
            0 as usize
        }
    };
}

#[macro_export]
macro_rules! MBSLEN {
    ($s:expr) => {
        if !$s.is_null() && *$s.offset(0 as isize) as libc::c_int != 0 {
            if *$s.offset(1 as isize) as libc::c_int != 0 {
                mbstrlen($s) as i32
            } else {
                1
            }
        } else {
            0
        }
    };
}

#[macro_export]
macro_rules! MB_STRLEN {
    ($s:expr) => {
        if MB_CUR_MAX!($s) > 1 {
            MBSLEN!($s)
        } else {
            STRLEN!($s)
        }
    };
}

//bashline
#[macro_export]
macro_rules! VSETATTR_1 {
    ($var:expr,$attr:expr) => {
        (*$var).attributes = (*$var).attributes | (&$attr);
        (*$var).attributes
    };
}

//arrayfunc
#[macro_export]
macro_rules! VSETATTR {
    ($var:expr,$attr:expr) => {
        (*$var).attributes |= ($attr) as libc::c_int;
        // (*$var).attributes
    };
}

#[macro_export]
macro_rules! ESC {
    () => {
        CTRL!('[' as i32)
    };
}

//brace
pub const _ISdigit: libc::c_uint = 2048;

#[macro_export]
macro_rules! ST_BAD {
    () => {
        0 as libc::c_int
    };
}

#[macro_export]
macro_rules! ST_INT {
    () => {
        1 as libc::c_int
    };
}

#[macro_export]
macro_rules! ST_CHAR {
    () => {
        2 as libc::c_int
    };
}

#[macro_export]
macro_rules! ST_ZINT {
    () => {
        3 as libc::c_int
    };
}

#[macro_export]
macro_rules! INTMAX_MIN {
    () => {
        -(9223372036854775807 as libc::c_long) - 1 as libc::c_int as libc::c_long
    };
}

#[macro_export]
macro_rules! INTMAX_MAX {
    () => {
        9223372036854775807 as libc::c_long - 2 as libc::c_int as libc::c_long
    };
}

#[macro_export]
macro_rules! ERANGE {
    () => {
        34 as libc::c_int
    };
}
#[macro_export]
macro_rules! BRACE_SEQ_SPECIFIER {
    () => {
        b"..\0" as *const u8 as *const libc::c_char
    };
}

#[macro_export]
macro_rules! INT_MAX {
    () => {
        2147483647 as libc::c_int as libc::c_long
    };
}

#[macro_export]
macro_rules! INT_MIN {
    () => {
        (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_long
    };
}

#[macro_export]
macro_rules! sh_imaxabs {
    ($x:expr) => {
        if $x >= 0 as libc::c_int as libc::c_long {
            $x
        } else {
            -$x
        }
    };
}

#[macro_export]
macro_rules! SIZEOF_fUNC {
    ($t:ty) => {
        std::mem::size_of::<$t>()
    };
}

#[macro_export]
macro_rules! TYPE_WIDTH {
    ($t:ty) => {
        (SIZEOF_fUNC!($t) * CHAR_BIT as usize) as $t
    };
}

#[macro_export]
macro_rules! TYPE_SIGNED {
    ($t:ty) => {
        if 0 as $t < (-1) as libc::c_int as $t {
            0 as $t
        }
        else {
            1 as $t
        }
    }
}

#[macro_export]
macro_rules! INT_STRLEN_BOUND {
    ($t:ty) => {
        (std::mem::size_of::<$t>() * CHAR_BIT as usize)
            - TYPE_SIGNED!($t) as usize * 302 as usize / 1000 as usize
            + 1 as usize
            + TYPE_SIGNED!($t) as usize
    };
}

#[macro_export]
macro_rules! SUBOVERFLOW {
    ($a:expr,$b:expr,$minv:expr,$maxv:expr) => {
        ($b > 0 as libc::c_long && $a < ($minv + $b))
            || ($b < 0 as libc::c_long && $a < ($maxv + $b))
    };
}

#[macro_export]
macro_rules! ADDOVERFLOW {
    ($a:expr,$b:expr,$minv:expr,$maxv:expr) => {
        ($a > 0 as libc::c_long && $b > ($maxv - $a))
            || ($a < 0 as libc::c_long && $b < ($minv + $a))
    };
}

#[macro_export]
macro_rules! brace_whitespace {
    ($c:expr) => {
        $c == 0
            || $c as libc::c_int == ' ' as i32
            || $c as libc::c_int == '\t' as i32
            || $c as libc::c_int == '\n' as i32
    };
}

#[macro_export]
macro_rules! ISALPHA {
    ($c:expr) => {
        IN_CTYPE_DOMAIN!($c) && isalpha!($c) != 0 as libc::c_int
    };
}

#[macro_export]
macro_rules! ISDIGIT {
    ($c:expr) => {
        IN_CTYPE_DOMAIN!($c) && isdigit!($c) != 0 as libc::c_int
    };
}

#[macro_export]
macro_rules! isalpha {
    ($c:expr) => {
        __isctype_f!($c, _ISalpha)
    };
}

#[macro_export]
macro_rules! isdigit {
    ($c:expr) => {
        __isctype_f!($c, _ISdigit)
    };
}

#[macro_export]
macro_rules! __isctype_f {
    ($c:expr,$type:expr) => {
        *(*__ctype_b_loc()).offset($c as libc::c_int as isize) as libc::c_int
            & ($type as libc::c_int as libc::c_ushort as libc::c_int)
    };
}

#[macro_export]
macro_rules! IN_CTYPE_DOMAIN {
    ($c:expr) => {
        1 as libc::c_int != 0 as libc::c_int
    };
}

//bracecomp
pub type rl_compentry_func_t = fn(*const libc::c_char, libc::c_int) -> *mut libc::c_char;

pub type rl_completion_func_t =
    fn(*const libc::c_char, libc::c_int, libc::c_int) -> *mut *mut libc::c_char;

pub type rl_quote_func_t =
    fn(*mut libc::c_char, libc::c_int, *mut libc::c_char) -> *mut libc::c_char;

pub type rl_compignore_func_t = fn(*mut *mut libc::c_char) -> libc::c_int;

//copycmd
pub const r_append_err_and_out: r_instruction = 19;
pub const r_move_output_word: r_instruction = 18;
pub const r_move_input_word: r_instruction = 17;
pub const r_move_output: r_instruction = 16;
pub const r_move_input: r_instruction = 15;
pub const r_duplicating_output_word: r_instruction = 14;
pub const r_duplicating_input_word: r_instruction = 13;
pub const r_output_force: r_instruction = 12;
pub const r_input_output: r_instruction = 11;
pub const r_err_and_out: r_instruction = 10;
pub const r_close_this: r_instruction = 9;
pub const r_deblank_reading_until: r_instruction = 8;
pub const r_duplicating_output: r_instruction = 7;
pub const r_duplicating_input: r_instruction = 6;
pub const r_reading_string: r_instruction = 5;
pub const r_reading_until: r_instruction = 4;
pub const r_appending_to: r_instruction = 3;
pub const r_inputa_direction: r_instruction = 2;
pub const r_input_direction: r_instruction = 1;
pub const r_output_direction: r_instruction = 0;

pub type command_type = libc::c_uint;
pub const cm_coproc: command_type = 14;
pub const cm_subshell: command_type = 13;
pub const cm_arith_for: command_type = 12;
pub const cm_cond: command_type = 11;
pub const cm_arith: command_type = 10;
pub const cm_group: command_type = 9;
pub const cm_until: command_type = 8;
pub const cm_function_def: command_type = 7;
pub const cm_connection: command_type = 6;
pub const cm_select: command_type = 5;
pub const cm_simple: command_type = 4;
pub const cm_if: command_type = 3;
pub const cm_while: command_type = 2;
pub const cm_case: command_type = 1;
pub const cm_for: command_type = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub union REDIRECTEE {
    pub dest: libc::c_int,
    pub filename: *mut WordDesc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redirect {
    pub next: *mut redirect,
    pub redirector: REDIRECTEE,
    pub rflags: libc::c_int,
    pub flags: libc::c_int,
    pub instruction: r_instruction,
    pub redirectee: REDIRECTEE,
    pub here_doc_eof: *mut libc::c_char,
}
pub type REDIRECT = redirect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct command {
    pub type_0: command_type,
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub redirects: *mut REDIRECT,
    pub value: command_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union command_value {
    pub For: *mut for_com,
    pub Case: *mut case_com,
    pub While: *mut while_com,
    pub If: *mut if_com,
    pub Connection: *mut connection,
    pub Simple: *mut simple_com,
    pub Function_def: *mut function_def,
    pub Group: *mut group_com,
    pub Select: *mut select_com,
    pub Arith: *mut arith_com,
    pub Cond: *mut cond_com,
    pub ArithFor: *mut arith_for_com,
    pub Subshell: *mut subshell_com,
    pub Coproc: *mut coproc_com,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct coproc_com {
    pub flags: libc::c_int,
    pub name: *mut libc::c_char,
    pub command: *mut COMMAND,
}
pub type COMMAND = command;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subshell_com {
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub command: *mut COMMAND,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arith_for_com {
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub init: *mut WordList,
    pub test: *mut WordList,
    pub step: *mut WordList,
    pub action: *mut COMMAND,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cond_com {
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub type_0: libc::c_int,
    pub op: *mut WordDesc,
    pub left: *mut cond_com,
    pub right: *mut cond_com,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arith_com {
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub exp: *mut WordList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct select_com {
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub name: *mut WordDesc,
    pub map_list: *mut WordList,
    pub action: *mut COMMAND,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group_com {
    pub ignore: libc::c_int,
    pub command: *mut COMMAND,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct function_def {
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub name: *mut WordDesc,
    pub command: *mut COMMAND,
    pub source_file: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct simple_com {
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub words: *mut WordList,
    pub redirects: *mut REDIRECT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct connection {
    pub ignore: libc::c_int,
    pub first: *mut COMMAND,
    pub second: *mut COMMAND,
    pub connector: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct if_com {
    pub flags: libc::c_int,
    pub test: *mut COMMAND,
    pub true_case: *mut COMMAND,
    pub false_case: *mut COMMAND,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct while_com {
    pub flags: libc::c_int,
    pub test: *mut COMMAND,
    pub action: *mut COMMAND,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct case_com {
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub word: *mut WordDesc,
    pub clauses: *mut PATTERN_LIST,
}
pub type PATTERN_LIST = pattern_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pattern_list {
    pub next: *mut pattern_list,
    pub patterns: *mut WordList,
    pub action: *mut COMMAND,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct for_com {
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub name: *mut WordDesc,
    pub map_list: *mut WordList,
    pub action: *mut COMMAND,
}
pub type CONNECTION = connection;
pub type CASE_COM = case_com;
pub type FOR_COM = for_com;
pub type ARITH_FOR_COM = arith_for_com;
pub type SELECT_COM = select_com;
pub type IF_COM = if_com;
pub type WHILE_COM = while_com;
pub type ARITH_COM = arith_com;
pub type COND_COM = cond_com;
pub type SIMPLE_COM = simple_com;
pub type FUNCTION_DEF = function_def;
pub type GROUP_COM = group_com;
pub type SUBSHELL_COM = subshell_com;
pub type COPROC_COM = coproc_com;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct g_list {
    pub next: *mut g_list,
}

//error
#[no_mangle]
pub static mut the_current_maintainer: *const libc::c_char =
    b"bash-maintainers@gnu.org\0" as *const u8 as *const libc::c_char;

//eval
#[macro_export]
macro_rules! setjmp_nosigs {
    ($x:expr) => {
        __sigsetjmp($x, 0)
    };
}

pub type stream_type = libc::c_uint;
pub const st_bstream: stream_type = 4;
pub const st_string: stream_type = 3;
pub const st_stream: stream_type = 2;
pub const st_none: stream_type = 0;

#[macro_export]
macro_rules! QUIT {
    () => {
        if terminating_signal != 0 {
            termsig_handler(terminating_signal);
        }
        if interrupt_state != 0 {
            throw_to_top_level();
        }
    };
}

//execute_cmd
#[macro_export]
macro_rules! FD_BITMAP_DEFAULT_SIZE {
    () => {
        32
    };
}

#[macro_export]
macro_rules! DESCRIBE_PID {
    ($pid:expr) => {
        if interactive != 0 {
            describe_pid($pid);
        }
    };
}

#[macro_export]
macro_rules! NO_PID {
    () => {
        -1 as libc::c_int
    };
}

#[macro_export]
macro_rules! COPY_PROCENV {
    ($old:expr, $save:expr) => {
        xbcopy(
            $old.as_mut_ptr() as *mut libc::c_char,
            $save.as_mut_ptr() as *mut libc::c_char,
            size_of::<sigjmp_buf>() as libc::c_ulong as libc::c_int,
        );
    };
}

#[macro_export]
macro_rules! POSIX_TIMEFORMAT {
    () => {
        b"real %2R\nuser %2U\nsys %2S\0" as *const u8 as *mut libc::c_char
    };
}

#[macro_export]
macro_rules! BASH_TIMEFORMAT {
    () => {
        b"\nreal\t%3lR\nuser\t%3lU\nsys\t%3lS\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    };
}

#[macro_export]
macro_rules! CHECK_TERMSIG {
    () => {
        if terminating_signal != 0 {
            termsig_handler(terminating_signal);
        }
    };
}

pub unsafe fn DIGIT(c: libc::c_char) -> bool {
    char::from(c as u8) >= '0' && char::from(c as u8) <= '9'
}

pub type __rusage_who = libc::c_int;
pub const RUSAGE_THREAD: __rusage_who = 1;
pub const RUSAGE_CHILDREN: __rusage_who = -1;
pub const RUSAGE_SELF: __rusage_who = 0;

#[no_mangle]
pub static mut stdin_redir: libc::c_int = 0;
#[no_mangle]
pub static mut this_command_name: *mut libc::c_char = 0 as *mut libc::c_char;
#[no_mangle]
pub static mut the_printed_command_except_trap: *mut libc::c_char = 0 as *mut libc::c_char;
#[no_mangle]
pub static mut return_catch_flag: libc::c_int = 0;
#[no_mangle]
pub static mut return_catch_value: libc::c_int = 0;
#[no_mangle]
pub static mut return_catch: sigjmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
#[no_mangle]
pub static mut last_command_exit_value: libc::c_int = 0;
#[no_mangle]
pub static mut last_command_exit_signal: libc::c_int = 0;
#[no_mangle]
pub static mut builtin_ignoring_errexit: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut redirection_undo_list: *mut REDIRECT =
    0 as *const libc::c_void as *mut libc::c_void as *mut REDIRECT;
#[no_mangle]
pub static mut exec_redirection_undo_list: *mut REDIRECT =
    0 as *const libc::c_void as *mut libc::c_void as *mut REDIRECT;
#[no_mangle]
pub static mut executing_builtin: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut executing_list: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut comsub_ignore_return: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut subshell_environment: libc::c_int = 0;
#[no_mangle]
pub static mut subshell_level: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut this_shell_function: *mut SHELL_VAR = 0 as *const SHELL_VAR as *mut SHELL_VAR;
#[no_mangle]
pub static mut match_ignore_case: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut executing_command_builtin: libc::c_int = 0 as libc::c_int;

pub static mut special_builtin_failed: libc::c_int = 0;
pub static mut currently_executing_command: *mut COMMAND = 0 as *const COMMAND as *mut COMMAND;
pub static mut function_line_number: libc::c_int = 0;
pub static mut showing_function_line: libc::c_int = 0;
#[no_mangle]
pub static mut line_number_for_err_trap: libc::c_int = 0;
#[no_mangle]
pub static mut funcnest: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut funcnest_max: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut evalnest: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut evalnest_max: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut sourcenest: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut sourcenest_max: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut from_return_trap: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut lastpipe_opt: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut current_fds_to_close: *mut fd_bitmap =
    0 as *const libc::c_void as *mut libc::c_void as *mut fd_bitmap;

#[no_mangle]
pub static mut sh_coproc: Coproc = {
    let mut init = coproc {
        c_name: 0 as *const libc::c_char as *mut libc::c_char,
        c_pid: -(1 as libc::c_int),
        c_rfd: -(1 as libc::c_int),
        c_wfd: -(1 as libc::c_int),
        c_rsave: 0 as libc::c_int,
        c_wsave: 0 as libc::c_int,
        c_flags: 0 as libc::c_int,
        c_status: 0 as libc::c_int,
        c_lock: 0 as libc::c_int,
    };
    init
};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpelement {
    pub next: *mut cpelement,
    pub coproc: *mut coproc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cplist {
    pub head: *mut cpelement,
    pub tail: *mut cpelement,
    pub ncoproc: libc::c_int,
    pub lock: libc::c_int,
}
pub type cplist_t = cplist;
#[no_mangle]
pub static mut coproc_list: cplist_t = {
    let mut init = cplist {
        head: 0 as *const cpelement as *mut cpelement,
        tail: 0 as *const cpelement as *mut cpelement,
        ncoproc: 0 as libc::c_int,
        lock: 0,
    };
    init
};
#[macro_export]
macro_rules! SIG_BLOCK {
    () => {
        0
    };
}

#[macro_export]
macro_rules! SIG_SETMASK {
    () => {
        2
    };
}

//UNBLOCK_SIGNAL的变形1
#[macro_export]
macro_rules! UNBLOCK_SIGNAL_1 {
    ($ovar:expr) => {
        sigprocmask(2 as libc::c_int, $ovar, 0 as *mut libc::c_void as *mut sigset_t)
    };
}

//UNBLOCK_CHILD的变形1
#[macro_export]
macro_rules! UNBLOCK_CHILD_1 {
    ($ovar:expr) => {
        UNBLOCK_SIGNAL_1!($ovar);
    };
}

//BLOCK_SIGNAL的变形1
#[macro_export]
macro_rules! BLOCK_SIGNAL_1 {
    ($sig:expr, $nvar:expr, $ovar:expr) => {
        sigemptyset($nvar);
        sigaddset($nvar, $sig as libc::c_int);
        sigemptyset($ovar);
        sigprocmask(0 as libc::c_int, $nvar, $ovar);
    };
}

#[macro_export]
macro_rules! BLOCK_CHILD_1 {
    ($nvar:expr, $ovar:expr) => {
        BLOCK_SIGNAL_1!(17 as libc::c_int, $nvar, $ovar);
    };
}

//src目录下使用
#[macro_export]
macro_rules! BLOCK_SIGNAL {
    ($sig:expr, $nvar:expr, $ovar:expr) => {
        sigemptyset(&mut $nvar);
        sigaddset(&mut $nvar, $sig as libc::c_int);
        sigemptyset(&mut $ovar);
        sigprocmask(SIG_BLOCK as libc::c_int, &mut $nvar, &mut $ovar);
    };
}
#[macro_export]
macro_rules! UNBLOCK_SIGNAL {
    ($ovar:expr) => {
        sigprocmask(SIG_SETMASK as libc::c_int, &mut $ovar, 0 as *mut sigset_t)
    };
}

#[macro_export]
macro_rules! att_nameref {
    () => {
        0x0000800
    };
}

#[macro_export]
macro_rules! att_readonly {
    () => {
        0x0000002
    };
}

#[macro_export]
macro_rules! att_noassign {
    () => {
        0x0004000
    };
}

#[macro_export]
macro_rules! att_array {
    () => {
        0x0000004
    };
}

#[macro_export]
macro_rules! get_job_by_jid {
    ($ind:expr) => {
        (*jobs).offset($ind as isize)
    };
}
#[macro_export]
macro_rules! INVALID_JOB {
    ($j:expr) => {
        ($j < 0 || $j >= js.j_jobslots || get_job_by_jid!($j) == 0 as *mut job)
    };
}

#[macro_export]
macro_rules! REAP {
    () => {
        if job_control == 0 || interactive_shell == 0 {
            reap_dead_jobs();
        }
    };
}

#[macro_export]
macro_rules! name_cell {
    ($var:expr) => {
        ((*$var).name)
    };
}

#[macro_export]
macro_rules! ifsname {
    ($s:expr) => {
        *$s.offset(0 as libc::c_int as isize) as libc::c_int == 'I' as i32
            && *$s.offset(1 as libc::c_int as isize) as libc::c_int == 'F' as i32
            && *$s.offset(2 as libc::c_int as isize) as libc::c_int == 'S' as i32
            && *$s.offset(3 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
    };
}

#[macro_export]
macro_rules! NUMBER_LEN {
    ($s:expr) => {
        if $s < 10 {
            1
        } else if $s < 100 {
            2
        } else if $s < 1000 {
            3
        } else if $s < 10000 {
            4
        } else if $s < 100000 {
            5
        } else {
            6
        };
    };
}

#[macro_export]
macro_rules! RP_SPACE_LEN {
    () => {
        2
    };
}

#[macro_export]
macro_rules! FNMATCH_IGNCASE {
    () => {
        if match_ignore_case != 0 {
            1 << 4
        } else {
            0
        }
    };
}

#[macro_export]
macro_rules! CMD_WHILE {
    () => {
        0
    };
}

#[macro_export]
macro_rules! CMD_UNTIL {
    () => {
        1
    };
}

#[macro_export]
macro_rules! INPUT_REDIRECT {
    ($ri:expr) => {
        $ri == r_input_direction as libc::c_uint
            || $ri == r_inputa_direction as libc::c_uint
            || $ri == r_input_output as libc::c_uint
    };
}

#[macro_export]
macro_rules! TRANSLATE_REDIRECT {
    ($ri:expr) => {
        $ri == r_duplicating_input_word
            || $ri == r_duplicating_output_word
            || $ri == r_move_input_word
            || $ri == r_move_output_word
    };
}

#[macro_export]
macro_rules! ISOPTION {
    ($s:expr, $c:expr) => {
        (*$s.offset(0 as isize) as libc::c_int == '-' as i32
            && *$s.offset(1 as isize) as libc::c_int == $c as i32
            && *$s.offset(2 as isize) == 0)
    };
}

#[macro_export]
macro_rules! EX_REDIRFAIL {
    () => {
        259
    };
}

#[macro_export]
macro_rules! EX_BADASSIGN {
    () => {
        260
    };
}

#[macro_export]
macro_rules! EX_EXPFAIL {
    () => {
        261
    };
}

#[macro_export]
macro_rules! EX_DISKFALLBACK {
    () => {
        262
    };
}

#[macro_export]
macro_rules! TRAP_STRING {
    ($s:expr) => {
        if signal_is_trapped($s) != 0 && signal_is_ignored($s) == 0 {
            *trap_list.as_mut_ptr().offset(($s) as isize)
        } else {
            0 as *mut libc::c_char
        }
    };
}

#[macro_export]
macro_rules! array_pop {
    ($a:expr) => {
        array_dispose_element(array_shift(($a), 1, 0));
    };
}

#[macro_export]
macro_rules! GET_ARRAY_FROM_VAR {
    ($n:expr, $v:expr, $a:expr) => {
        $v = find_variable($n);
        $a = if !$v.is_null() && array_p!($v) != 0 {
            array_cell!($v)
        } else {
            0 as *mut ARRAY
        }
    };
}

#[macro_export]
macro_rules! trace_p {
    ($var:expr) => {
        (*$var).attributes & att_trace as i32
    };
}

#[macro_export]
macro_rules! array_push {
    ($a:expr, $v:expr) => {
        array_rshift($a, 1, $v)
    };
}

#[macro_export]
macro_rules! NOTFOUND_HOOK {
    () => {
        b"command_not_found_handle\0" as *const u8 as *const libc::c_char
    };
}

#[macro_export]
macro_rules! vc_isbltnenv {
    ($vc:expr) => {
        (*$vc).flags & VC_BLTNENV as libc::c_int != 0
    };
}

#[macro_export]
macro_rules! ENOEXEC {
    () => {
        8
    };
}

#[macro_export]
macro_rules! E2BIG {
    () => {
        7
    };
}

#[macro_export]
macro_rules! ENOMEM {
    () => {
        12
    };
}

#[macro_export]
macro_rules! EISDIR {
    () => {
        21
    };
}

#[macro_export]
macro_rules! HASH_BANG_BUFSIZ {
    () => {
        128
    };
}

#[macro_export]
macro_rules! READ_SAMPLE_BUF {
    ($file:expr, $buf:expr, $len:expr) => {
        let mut fd_0 = open($file, O_RDONLY as libc::c_int);
        if fd_0 >= 0 {
            $len = read(
                fd_0,
                $buf.as_mut_ptr() as *mut libc::c_void,
                HASH_BANG_BUFSIZ!() as libc::c_int as usize,
            ) as libc::c_int;
            close(fd_0);
        } else {
            $len = -1;
        }
    };
}

#[macro_export]
macro_rules! CTLESC {
    () => {
        '\u{1}' as i32
    };
}

//expr
pub const EXP_EXPANDED: libc::c_int = 1;
pub const ASS_NOEXPAND: libc::c_int = 128;
pub const AV_NOEXPAND: libc::c_int = 0x20;
pub const EXECUTION_FAILURE: libc::c_int = 1;
pub const DISCARD: libc::c_int = 2;
pub const FORCE_EOF: libc::c_int = 1;
pub const PREINC: libc::c_int = 14;
pub const PREDEC: libc::c_int = 15;
pub const STR: libc::c_int = 5;
pub const NUM: libc::c_int = 6;
pub const POSTINC: libc::c_int = 16;
pub const POSTDEC: libc::c_int = 17;
pub const POWER: libc::c_int = 13;
pub const LSH: libc::c_int = 9;
pub const RSH: libc::c_int = 10;
pub const LEQ: libc::c_int = 3;
pub const GEQ: libc::c_int = 4;
pub const EQEQ: libc::c_int = 1;
pub const NEQ: libc::c_int = 2;
pub const LAND: libc::c_int = 7;
pub const LOR: libc::c_int = 8;
pub const COND: libc::c_int = 12;
pub const OP_ASSIGN: libc::c_int = 11;
pub const MAX_EXPR_RECURSION_LEVEL: libc::c_int = 1024;
pub const EXPR_STACK_GROW_SIZE: libc::c_int = 10;
pub const EQ: libc::c_int = '=' as libc::c_int;
pub const NOT: libc::c_char = '!' as libc::c_char;
pub const GT: libc::c_int = '>' as libc::c_int;
pub const LT: libc::c_int = '<' as libc::c_int;
pub const BAND: libc::c_char = '&' as libc::c_char;
pub const BOR: libc::c_char = '|' as libc::c_char;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct EXPR_CONTEXT {
    pub curtok: libc::c_int,
    pub lasttok: libc::c_int,
    pub expression: *mut libc::c_char,
    pub tp: *mut libc::c_char,
    pub lasttp: *mut libc::c_char,
    pub tokval: intmax_t,
    pub tokstr: *mut libc::c_char,
    pub noeval: libc::c_int,
    pub lval: lvalue,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct lvalue {
    pub tokstr: *mut libc::c_char,
    pub tokval: intmax_t,
    pub tokvar: *mut SHELL_VAR,
    pub ind: intmax_t,
}

//find_cmd
#[no_mangle]
pub static mut check_hashed_filenames: libc::c_int = CHECKHASH_DEFAULT as libc::c_int;
#[no_mangle]
pub static mut dot_found_in_search: libc::c_int = 0;

#[macro_export]
macro_rules! FNM_EXTMATCH {
    () => {
        1 << 5
    };
}

#[macro_export]
macro_rules! FNM_CASEFOLD {
    () => {
        1 << 4
    };
}

#[macro_export]
macro_rules! __S_IFDIR {
    () => {
        0o40000
    };
}

#[macro_export]
macro_rules! __S_IFMT {
    () => {
        0o170000
    };
}

#[macro_export]
macro_rules! __S_ISTYPE {
    ($mode:expr, $mask:expr) => {
        $mode & __S_IFMT!() == $mask
    };
}

#[macro_export]
macro_rules! S_ISDIR {
    ($mode:expr) => {
        __S_ISTYPE!($mode, __S_IFDIR!())
    };
}

#[macro_export]
macro_rules! att_tempvar {
    () => {
        0x0100000
    };
}

#[macro_export]
macro_rules! tempvar_p {
    ($var:expr) => {
        (*$var).attributes & att_tempvar!()
    };
}

//flags
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flags_alist {
    pub name: libc::c_char,
    pub value: *mut libc::c_int,
}
#[no_mangle]
pub static mut mark_modified_vars: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut asynchronous_notification: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut errexit_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut exit_immediately_on_error: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut disallow_filename_globbing: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut place_keywords_in_env: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut read_but_dont_execute: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut just_one_command: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut noclobber: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut unbound_vars_is_error: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut echo_input_at_read: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut verbose_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut echo_command_at_execute: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut jobs_m_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut forced_interactive: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut no_symbolic_links: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut hashing_enabled: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut history_expansion: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut histexp_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut interactive_comments: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut restricted: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut restricted_shell: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut privileged_mode: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut brace_expansion: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut function_trace_mode: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut error_trace_mode: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut pipefail_opt: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut shell_flags: [flags_alist; 22] = unsafe {
    [
        {
            let mut init = flags_alist {
                name: 'a' as i32 as libc::c_char,
                value: &mark_modified_vars as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'b' as i32 as libc::c_char,
                value: &asynchronous_notification as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'e' as i32 as libc::c_char,
                value: &errexit_flag as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'f' as i32 as libc::c_char,
                value: &disallow_filename_globbing as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'h' as i32 as libc::c_char,
                value: &hashing_enabled as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'i' as i32 as libc::c_char,
                value: &forced_interactive as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'k' as i32 as libc::c_char,
                value: &place_keywords_in_env as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'm' as i32 as libc::c_char,
                value: &jobs_m_flag as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'n' as i32 as libc::c_char,
                value: &read_but_dont_execute as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'p' as i32 as libc::c_char,
                value: &privileged_mode as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'r' as i32 as libc::c_char,
                value: &restricted as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 't' as i32 as libc::c_char,
                value: &just_one_command as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'u' as i32 as libc::c_char,
                value: &unbound_vars_is_error as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'v' as i32 as libc::c_char,
                value: &verbose_flag as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'x' as i32 as libc::c_char,
                value: &echo_command_at_execute as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'B' as i32 as libc::c_char,
                value: &brace_expansion as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'C' as i32 as libc::c_char,
                value: &noclobber as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'E' as i32 as libc::c_char,
                value: &error_trace_mode as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'H' as i32 as libc::c_char,
                value: &histexp_flag as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'P' as i32 as libc::c_char,
                value: &no_symbolic_links as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 'T' as i32 as libc::c_char,
                value: &function_trace_mode as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = flags_alist {
                name: 0 as libc::c_int as libc::c_char,
                value: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut optflags: [libc::c_char; 26] = [
    '+' as i32 as libc::c_char,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];

pub const FLAG_UNKNOWN: libc::c_int = 0;
pub const FLAG_OFF: std::os::raw::c_char = b'+' as std::os::raw::c_char;
pub const FLAG_ERROR: libc::c_int = -1;
pub const FLAG_ON: std::os::raw::c_char = b'-' as std::os::raw::c_char;

//general
pub const _ISalnum: libc::c_int = 8;
pub const _ISblank: libc::c_int = 1;
pub const _CS_PATH: libc::c_int = 0;
pub const W_HASDOLLAR: libc::c_int = 1 << 0;
pub const W_QUOTED: libc::c_int = 1 << 1;
pub const CBLANK: libc::c_int = 0x2000;
pub const CSHBRK: libc::c_int = 0x0002;
pub const CXQUOTE: libc::c_int = 0x0400;
pub const F_GETFL: libc::c_int = 3;
pub const F_SETFL: libc::c_int = 4;
pub const O_NONBLOCK: libc::c_int = 0o4000;
pub const O_NDELAY: libc::c_int = 0o4000;
pub const F_SETFD: libc::c_int = 2;
pub const FD_CLOEXEC: libc::c_int = 1;
pub const F_GETFD: libc::c_int = 1;
pub const SEEK_CUR: libc::c_int = 1;
pub const ESPIPE: libc::c_int = 29;
pub const O_RDWR: libc::c_int = 0o2;
pub const HIGH_FD_MAX: libc::c_int = 256;
pub const W_OK: libc::c_int = 2;
pub const MP_DOCWD: libc::c_int = 0x02;
pub const MP_RMDOT: libc::c_int = 0x04;
pub const PATH_MAX: libc::c_int = 4096;

pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;

pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;

#[cfg(target_arch = "x86_64")]
pub type __nlink_t = libc::c_ulong;
#[cfg(not(target_arch = "x86_64"))]
pub type __nlink_t = libc::c_uint;

pub type __rlim_t = libc::c_ulong;
pub type __time_t = libc::c_long;

#[cfg(target_arch = "x86_64")]
pub type __blksize_t = libc::c_long;
#[cfg(not(target_arch = "x86_64"))]
pub type __blksize_t = libc::c_int;

pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}

pub type sig_atomic_t = __sig_atomic_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    #[cfg(target_arch = "x86_64")]
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    #[cfg(not(target_arch = "x86_64"))]
    pub st_nlink: __nlink_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    #[cfg(target_arch = "x86_64")]
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    #[cfg(not(target_arch = "x86_64"))]
    pub __pad1: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    #[cfg(not(target_arch = "x86_64"))]
    pub __pad2: libc::c_int,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    #[cfg(target_arch = "x86_64")]
    pub __glibc_reserved: [__syscall_slong_t; 3],
    #[cfg(not(target_arch = "x86_64"))]
    pub __glibc_reserved: [libc::c_int; 2],
}

// stat类型初始化
pub const stat_init: stat = stat {
    st_dev: 0,
    st_ino: 0,
    st_nlink: 0,
    st_mode: 0,
    st_uid: 0,
    st_gid: 0,
    #[cfg(target_arch = "x86_64")]
    __pad0: 0,
    st_rdev: 0,
    #[cfg(not(target_arch = "x86_64"))]
    __pad1: 0,
    st_size: 0,
    st_blksize: 0,
    #[cfg(not(target_arch = "x86_64"))]
    __pad2: 0,
    st_blocks: 0,
    st_atim: timespec {
        tv_sec: 0,
        tv_nsec: 0,
    },
    st_mtim: timespec {
        tv_sec: 0,
        tv_nsec: 0,
    },
    st_ctim: timespec {
        tv_sec: 0,
        tv_nsec: 0,
    },
    #[cfg(target_arch = "x86_64")]
    __glibc_reserved: [0; 3],
    #[cfg(not(target_arch = "x86_64"))]
    __glibc_reserved: [0; 2],
};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct user_info {
    pub uid: uid_t,
    pub euid: uid_t,
    pub gid: gid_t,
    pub egid: gid_t,
    pub user_name: *mut libc::c_char,
    pub shell: *mut libc::c_char,
    pub home_dir: *mut libc::c_char,
}

pub type rlim_t = __rlim_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub posix_mode_var: *mut libc::c_int,
}
pub type tilde_hook_func_t = fn(*mut libc::c_char) -> *mut libc::c_char;

#[macro_export]
macro_rules! TODIGIT {
    ($c:expr) => {
        ($c as libc::c_int - '0' as i32)
    };
}

#[macro_export]
macro_rules! shellblank {
    ($c: expr) => {
        (*sh_syntaxtab.as_mut_ptr().offset($c) & CBLANK)
    };
}
#[macro_export]
macro_rules! shellbreak {
    ($c: expr) => {
        (*sh_syntaxtab.as_mut_ptr().offset($c) & CSHBRK)
    };
}
#[macro_export]
macro_rules! shellxquote {
    ($c: expr) => {
        (*sh_syntaxtab.as_mut_ptr().offset($c) & CXQUOTE)
    };
}
#[macro_export]
macro_rules! shellexp {
    ($c: expr) => {
        ($c == '$' as i32 || $c == '<' as i32  || $c == '>' as i32)
    };
}
#[macro_export]
macro_rules! isblank {
    ($c: expr) => {
        (__isctype_f!(($c), _ISblank))
    };
}

#[macro_export]
macro_rules! PATHSEP {
    ($c: expr) => {
        (ISDIRSEP!($c) || ($c) == 0)
    };
}
#[macro_export]
macro_rules! ISDIRSEP {
    ($c: expr) => {
        (($c) == '/' as i32)
    };
}
#[macro_export]
macro_rules! ABSPATH {
    ($c: expr) => {
        (*$c.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32)
    };
}
#[macro_export]
macro_rules! TILDE_END {
    ($c: expr) => {
        (($c) == '\0' as i32 || ($c) == '/' as i32 || ($c) == ':' as i32)
    };
}

#[macro_export]
macro_rules! STANDARD_UTILS_PATH {
    () => {
        (b"/bin:/usr/bin:/usr/sbin:/sbin\0" as *const u8 as *const libc::c_char)
    };
}

//hashcmd
#[no_mangle]
pub static mut hashed_filenames: *mut HASH_TABLE =
    0 as *const libc::c_void as *mut libc::c_void as *mut HASH_TABLE;

//hashlib
#[macro_export]
macro_rules! HASH_ENTRIES {
    ($ht:expr) => {
        if !$ht.is_null() {
            (*$ht).nentries
        } else {
            0 as libc::c_int
        };
    };
}

#[macro_export]
macro_rules! HASH_REHASH_MULTIPLIER {
    () => {
        4
    };
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

#[macro_export]
macro_rules! HASH_BUCKET {
    ($s:expr, $t:expr, $h:expr) => {{
        $h = hash_string($s);
        $h & (((*$t).nbuckets - 1) as libc::c_uint)
    }};
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

//input
pub const EAGAIN: libc::c_int = 11;
pub const X_EAGAIN: libc::c_int = EAGAIN;
pub const EWOULDBLOCK: libc::c_int = EAGAIN;
pub const X_EWOULDBLOCK: libc::c_int = EAGAIN;
pub const EINTR: libc::c_int = 4;
pub const B_UNBUFF: libc::c_int = 0x04;
pub const O_TEXT: libc::c_int = 0;
pub const B_TEXT: libc::c_int = 0x10;
pub const MAX_INPUT_BUFFER_SIZE: libc::c_int = 8172;
pub const O_RDONLY: libc::c_int = 0;
pub const EBADF: libc::c_int = 9;
pub type __ssize_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type sh_cget_func_t = fn() -> libc::c_int;
pub type sh_cunget_func_t = fn(libc::c_int) -> libc::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct BSTREAM {
    pub b_fd: libc::c_int,
    pub b_buffer: *mut libc::c_char,
    pub b_size: size_t,
    pub b_used: size_t,
    pub b_flag: libc::c_int,
    pub b_inputp: size_t,
}
pub type BUFFERED_STREAM = BSTREAM;
#[derive(Copy, Clone)]
#[repr(C)]
pub union INPUT_STREAM {
    pub file: *mut FILE,
    pub string: *mut libc::c_char,
    pub buffered_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BASH_INPUT {
    pub type_0: stream_type,
    pub name: *mut libc::c_char,
    pub location: INPUT_STREAM,
    pub getter: Option<sh_cget_func_t>,
    pub ungetter: Option<sh_cunget_func_t>,
}

#[macro_export]
macro_rules! ALLOCATE_BUFFERS {
    ($n:expr) => {
        if $n >= nbuffers {
            allocate_buffers($n);
        }
    };
}

#[macro_export]
macro_rules! max {
    ($a: expr, $b: expr) => {
        if $a > $b {
            $a
        } else {
            $b
        }
    };
}
#[macro_export]
macro_rules! min {
    ($a: expr, $b: expr) => {
        if $a > $b {
            $b
        } else {
            $a
        }
    };
}
#[macro_export]
macro_rules! fd_is_seekable {
    ($fd: expr) => {
        lseek($fd, 0 as libc::c_long, SEEK_CUR) >= 0 as libc::c_long
    };
}
#[macro_export]
macro_rules! bufstream_getc {
    ($bp: expr) => {
        if ($bp).b_inputp == ($bp).b_used || ($bp).b_used == 0 {
            b_fill_buffer(*buffers.offset(bash_input.location.buffered_fd as isize))
        } else {
            let ref mut fresh10 =
                (**buffers.offset(bash_input.location.buffered_fd as isize)).b_inputp;
            let fresh11 = *fresh10;
            *fresh10 = (*fresh10).wrapping_add(1);
            *(($bp).b_buffer).offset(fresh11 as isize) as libc::c_int & 0xff as libc::c_int
        }
    };
}

//jobs
#[macro_export]
macro_rules! SIG_IGN {
    () => {
        ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1 as libc::c_int as libc::intptr_t)
    };
}

#[macro_export]
macro_rules! SIG_DFL {
    () => {
        ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(0 as libc::c_int as libc::intptr_t)
    };
}

pub type sigjmp_buf = [__jmp_buf_tag; 1];
pub type __jmp_buf = [libc::c_long; 8];

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}

pub const DEFAULT_CHILD_MAX: u32 = 4096;
pub const MAX_CHILD_MAX: u32 = 32768;
pub const MAX_JOBS_IN_ARRAY: u32 = 4096;
pub const r_pidstat_table_SZ: i32 = 4096;
pub const BGPIDS_TABLE_SZ: u32 = 512;
pub const DEL_WARNSTOPPED: u32 = 1;
pub const DEL_NOBGPID: u32 = 2;
pub const JOB_SLOTS: u32 = 8;
pub const NO_PIDSTAT: i32 = -1;
pub const NO_PID: pid_t = -1;

pub const EX_NOEXEC: i32 = 126;

pub const ECHILD: i32 = 10;

#[macro_export]
macro_rules! PRECYCLED {
    ($p:expr) => {
        0
    };
}

#[macro_export]
macro_rules! __WIFSTOPPED {
    ($status:expr) => {
        (($status) & 0xff) == 0x7f
    };
}

#[macro_export]
macro_rules! WIFSTOPPED {
    ($status:expr) => {
        (($status) & 0xff) == 0x7f
    };
}

#[macro_export]
macro_rules! PSTOPPED {
    ($p:expr) => {
        WIFSTOPPED!((*$p).status)
    };
}

#[macro_export]
macro_rules! PRUNNING {
    ($p:expr) => {
        (*$p).running == PS_RUNNING as libc::c_int
    };
}

#[macro_export]
macro_rules! PALIVE {
    ($p:expr) => {
        PRUNNING!($p) || PSTOPPED!($p)
    };
}

#[macro_export]
macro_rules! WAITPID {
    ($pid:expr, $statusp:expr, $options:expr) => {
        waitpid($pid as pid_t, $statusp, $options);
    };
}

#[macro_export]
macro_rules! getpgid {
    ($p:expr) => {
        getpgpr();
    };
}

#[macro_export]
macro_rules! REINSTALL_SIGCHLD_HANDLER {
    () => {};
}

pub type sh_job_map_func_t = fn(*mut JOB, libc::c_int, libc::c_int, libc::c_int) -> libc::c_int;

pub static mut zerojs: jobstats = {
    let mut init = jobstats {
        c_childmax: -(1 as libc::c_long),
        c_living: 0 as libc::c_int,
        c_reaped: 0 as libc::c_int,
        c_injobs: 0 as libc::c_int,
        c_totforked: 0 as libc::c_int,
        c_totreaped: 0 as libc::c_int,
        j_jobslots: 0 as libc::c_int,
        j_lastj: 0 as libc::c_int,
        j_firstj: 0 as libc::c_int,
        j_njobs: 0 as libc::c_int,
        j_ndead: 0 as libc::c_int,
        j_current: -(1 as libc::c_int),
        j_previous: -(1 as libc::c_int),
        j_lastmade: 0 as *const JOB as *mut JOB,
        j_lastasync: 0 as *const JOB as *mut JOB,
    };
    init
};

#[no_mangle]
pub static mut js: jobstats = {
    let mut init = jobstats {
        c_childmax: -(1 as libc::c_long),
        c_living: 0 as libc::c_int,
        c_reaped: 0 as libc::c_int,
        c_injobs: 0 as libc::c_int,
        c_totforked: 0 as libc::c_int,
        c_totreaped: 0 as libc::c_int,
        j_jobslots: 0 as libc::c_int,
        j_lastj: 0 as libc::c_int,
        j_firstj: 0 as libc::c_int,
        j_njobs: 0 as libc::c_int,
        j_ndead: 0 as libc::c_int,
        j_current: -(1 as libc::c_int),
        j_previous: -(1 as libc::c_int),
        j_lastmade: 0 as *const JOB as *mut JOB,
        j_lastasync: 0 as *const JOB as *mut JOB,
    };
    init
};

#[no_mangle]
pub static mut r_pidstat_table: [ps_index_t; 4096] = [0; 4096];

pub static mut bgpids: bgpids = {
    let init = bgpids {
        storage: 0 as *const pidstat as *mut pidstat,
        head: 0 as ps_index_t,
        nalloc: 0 as ps_index_t,
        npid: 0 as libc::c_int,
    };
    init
};

pub static mut procsubs: procchain = {
    let mut init = procchain {
        head: 0 as *mut PROCESS,
        end: 0 as *mut PROCESS,
        nproc: 0 as libc::c_int,
    };
    init
};

#[no_mangle]
pub static mut wait_intr_buf: sigjmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0 as libc::c_int,
    __saved_mask: __sigset_t {
        __val: [0; 16usize],
    },
}];

#[no_mangle]
pub static mut jobs: *mut *mut JOB = 0 as *const libc::c_void as *mut libc::c_void as *mut *mut JOB;

#[no_mangle]
pub static mut shell_tty: libc::c_int = -1;
#[no_mangle]
pub static mut shell_pgrp: pid_t = -1 as libc::c_int;
pub static mut terminal_pgrp: pid_t = -1;
#[no_mangle]
pub static mut original_pgrp: pid_t = -1;
#[no_mangle]
pub static mut pipeline_pgrp: pid_t = 0 as pid_t;

#[no_mangle]
pub static mut pgrp_pipe: [libc::c_int; 2] = [-1 as libc::c_int, -1 as libc::c_int];
#[no_mangle]
pub static mut last_made_pid: pid_t = -1;
#[no_mangle]
pub static mut last_asynchronous_pid: pid_t = -1;
#[no_mangle]
pub static mut the_pipeline: *mut PROCESS =
    0 as *const libc::c_void as *mut libc::c_void as *mut PROCESS;
#[no_mangle]
pub static mut job_control: libc::c_int = 1;
#[no_mangle]
pub static mut running_in_background: libc::c_int = 0;
#[no_mangle]
pub static mut already_making_children: libc::c_int = 0;
#[no_mangle]
pub static mut check_window_size: libc::c_int = CHECKWINSIZE_DEFAULT as i32;
#[no_mangle]
pub static mut last_procsub_child: *mut PROCESS =
    0 as *const libc::c_void as *mut libc::c_void as *mut PROCESS;

pub static mut pstatuses: *mut libc::c_int = 0 as *mut libc::c_int;
pub static mut statsize: libc::c_int = 0;

pub static mut sigchld: libc::c_int = 0;
pub static mut queue_sigchld: libc::c_int = 0;

#[macro_export]
macro_rules! QUEUE_SIGCHLD {
    ($os:expr) => {
        $os = sigchld;
        queue_sigchld += 1;
    };
}

#[macro_export]
macro_rules! DEADJOB {
    ($j:expr) => {
        (**jobs.offset($j as isize)).state as libc::c_int == JDEAD
    };
}

#[macro_export]
macro_rules! IS_NOTIFIED {
    ($j:expr) => {
        (**jobs.offset($j as isize)).flags & J_NOTIFIED as libc::c_int != 0 as libc::c_int
    };
}

#[macro_export]
macro_rules! JOBSTATE {
    ($job:expr) => {
        (**jobs.offset($job as isize)).state
    };
}

#[macro_export]
macro_rules! STOPPED {
    ($job:expr) => {
        (**jobs.offset($job as isize)).state == JSTOPPED
    };
}

pub const JMIXED: JOB_STATE = 8;
pub const JDEAD: JOB_STATE = 4;
pub const JSTOPPED: JOB_STATE = 2;
pub const JRUNNING: JOB_STATE = 1;
pub const JNONE: JOB_STATE = -1;

pub static mut old_tstp: Option<SigHandler> = None;
pub static mut old_ttou: Option<SigHandler> = None;
pub static mut old_ttin: Option<SigHandler> = None;
pub static mut old_cont: Option<SigHandler> = None; //bgz
pub static mut saved_pipeline: *mut pipeline_saver = 0 as *mut pipeline_saver;
pub static mut saved_already_making_children: libc::c_int = 0;

pub static mut jobs_list_frozen: libc::c_int = 0;
pub static mut retcode_name_buffer: [libc::c_char; 64] = [0; 64];

macro_rules! TYPE_WIDTH {
    ($t:ty) => {
        std::mem::size_of::<$t>() * 8
    };
}

macro_rules! TYPE_SIGNED_1 {
    ($t:ty) => {
        !(0 as $t < -1 as $t)
    };
}

macro_rules! TYPE_MAXIMUM {
    ($t:ty) => {
        if !TYPE_SIGNED_1!($t){
            -1 as $t
        } else {
            (((1 as $t << (TYPE_WIDTH!($t) -2)) -1) * 2 + 1)
        }

    }
}

#[macro_export]
macro_rules! WSTOPSIG {
    ($status:expr) => {
        ($status & 0xff00) >> 8
    };
}

#[macro_export]
macro_rules! RUNNING {
    ($j:expr) => {
        (**jobs.offset($j as isize)).state as libc::c_int == JRUNNING
    };
}

#[macro_export]
macro_rules! WIFSIGNALED {
    ($status:expr) => {
        ((($status) & 0x7f) + 1) >> 1 > 0
    };
}

#[macro_export]
macro_rules! WTERMSIG {
    ($status:expr) => {
        ($status) & 0x7f
    };
}

#[macro_export]
macro_rules! WIFEXITED {
    ($status:expr) => {
        ($status) & 0x7f == 0
    };
}

#[macro_export]
macro_rules! WEXITSTATUS {
    ($status:expr) => {
        (($status) & 0xff00) >> 8
    };
}

#[macro_export]
macro_rules! WSTATUS {
    ($t:expr) => {
        $t
    };
}

#[macro_export]
macro_rules! WIFCONTINUED {
    ($status:expr) => {
        $status == 0xffff
    };
}

#[macro_export]
macro_rules! WIFCORED {
    ($status:expr) => {
        $status == 0x80
    };
}

#[macro_export]
macro_rules! J_FOREGROUND {
    () => {
        0x01
    };
}

#[macro_export]
macro_rules! IS_FOREGROUND {
    ($j:expr) => {
        (**jobs.offset($j as isize)).flags & J_FOREGROUND!() != 0
    };
}

#[macro_export]
macro_rules! CLRINTERRUPT {
    () => {
        interrupt_state = 0
    };
}

#[macro_export]
macro_rules! input_tty {
    () => {
        if shell_tty != -1 {
            shell_tty
        } else {
            fileno(stderr)
        }
    };
}

#[macro_export]
macro_rules! CHECK_WAIT_INTR {
    () => {
        if wait_intr_flag != 0
            && wait_signal_received != 0
            && this_shell_builtin.is_some()
            && this_shell_builtin == (Some(wait_builtin))
        {
            siglongjmp(wait_intr_buf.as_mut_ptr(), 1 as libc::c_int);
        }
    };
}

#[no_mangle]
pub static mut waiting_for_child: libc::c_int = 0;

pub type SigHandler1 = Option<fn(arg1: ::std::os::raw::c_int)>;

pub const ANY_PID: pid_t = -1;
#[macro_export]
macro_rules! IS_JOBCONTROL {
    ($job:expr) => {
        (**jobs.offset($job as isize)).flags & 0x4 as libc::c_int != 0 as libc::c_int
    };
}

#[macro_export]
macro_rules! RL_ISSTATE {
    ($x:expr) => {
        rl_readline_state & ($x)
    };
}
#[macro_export]
macro_rules! RL_STATE_COMPLETING {
    () => {
        0x0004000
    };
}

#[macro_export]
macro_rules! ADDINTERRUPT {
    () => {
        interrupt_state += 1
    };
}

#[macro_export]
macro_rules! IS_WAITING {
    ($i:expr) => {
        (**jobs.offset($i as isize)).flags & 0x80 as libc::c_int != 0 as libc::c_int
    };
}

pub type cc_t = libc::c_uchar;
pub type speed_t = libc::c_uint;
pub type tcflag_t = libc::c_uint;

#[macro_export]
macro_rules! PEXITED {
    ($p:expr) => {
        (*$p).running == 0 as libc::c_int
    };
}

#[macro_export]
macro_rules! sh_longjmp {
    ($x:expr,$n:expr) => {
        siglongjmp($x, $n as libc::c_int)
    };
}

pub const EINVAL: i32 = 22;

#[macro_export]
macro_rules! IMPOSSIBLE_TRAP_HANDLER {
    () => {
        initialize_traps as *mut SigHandler
    };
}

#[macro_export]
macro_rules! IGNORE_SIG {
    () => {
        SIG_IGN!()
    };
}

pub const SEVAL_NOHIST: libc::c_int = 0x004;
pub const SEVAL_RESETLINE: libc::c_int = 0x010;

#[macro_export]
macro_rules! SET_CLOSE_ON_EXEC {
    ($fd:expr) => {
        libc::fcntl(($fd), F_SETFD, FD_CLOEXEC)
    };
}

pub const ENOTTY: i32 = 25;

//list
pub type GENERIC_LIST = g_list;
#[no_mangle]
pub static mut global_error_list: GENERIC_LIST = g_list {
    next: 0 as *const g_list as *mut g_list,
};

//local
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lconv {
    pub decimal_point: *mut libc::c_char,
    pub thousands_sep: *mut libc::c_char,
    pub grouping: *mut libc::c_char,
    pub int_curr_symbol: *mut libc::c_char,
    pub currency_symbol: *mut libc::c_char,
    pub mon_decimal_point: *mut libc::c_char,
    pub mon_thousands_sep: *mut libc::c_char,
    pub mon_grouping: *mut libc::c_char,
    pub positive_sign: *mut libc::c_char,
    pub negative_sign: *mut libc::c_char,
    pub int_frac_digits: libc::c_char,
    pub frac_digits: libc::c_char,
    pub p_cs_precedes: libc::c_char,
    pub p_sep_by_space: libc::c_char,
    pub n_cs_precedes: libc::c_char,
    pub n_sep_by_space: libc::c_char,
    pub p_sign_posn: libc::c_char,
    pub n_sign_posn: libc::c_char,
    pub int_p_cs_precedes: libc::c_char,
    pub int_p_sep_by_space: libc::c_char,
    pub int_n_cs_precedes: libc::c_char,
    pub int_n_sep_by_space: libc::c_char,
    pub int_p_sign_posn: libc::c_char,
    pub int_n_sign_posn: libc::c_char,
}

#[no_mangle]
pub static mut locale_utf8locale: libc::c_int = 0;
#[no_mangle]
pub static mut locale_mb_cur_max: libc::c_int = 0;
#[no_mangle]
pub static mut locale_shiftstates: libc::c_int = 0 as libc::c_int;
pub static mut default_locale: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut default_domain: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut default_dir: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut lc_all: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut lang: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;

pub const LC_ALL: libc::c_int = 6;
pub const STR_LC_ALL: *const libc::c_char = b"LC_ALL\0" as *const u8 as *const libc::c_char;
pub const STR_LC_CTYPE: *const libc::c_char = b"LC_CTYPE\0" as *const u8 as *const libc::c_char;
pub const STR_LC_COLLATE: *const libc::c_char = b"LC_COLLATE\0" as *const u8 as *const libc::c_char;
pub const STR_LC_MESSAGES: *const libc::c_char =
    b"LC_MESSAGES\0" as *const u8 as *const libc::c_char;
pub const STR_LC_NUMERIC: *const libc::c_char = b"LC_NUMERIC\0" as *const u8 as *const libc::c_char;
pub const STR_LC_TIME: *const libc::c_char = b"LC_TIME\0" as *const u8 as *const libc::c_char;

pub const STR_TEXTDOMAIN: *const libc::c_char = b"TEXTDOMAIN\0" as *const u8 as *const libc::c_char;
pub const STR_TEXTDOMAINDIR: *const libc::c_char =
    b"TEXTDOMAINDIR\0" as *const u8 as *const libc::c_char;

pub const STR_PACKAGE: *const libc::c_char = b"utshell\0" as *const u8 as *const libc::c_char;
pub const STR_LOCALEDIR: *const libc::c_char =
    b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char;

//mainlcheck
pub static MBOX_INITIALIZED: libc::c_int = 0x01;

#[macro_export]
macro_rules! UPDATE_MAIL_FILE {
    ($i:expr, $finfo:expr) => {
        (**mailfiles.offset($i as isize)).access_time = $finfo.st_atim.tv_sec;
        (**mailfiles.offset($i as isize)).mod_time = $finfo.st_mtim.tv_sec;
        (**mailfiles.offset($i as isize)).file_size = $finfo.st_size;
        (**mailfiles.offset($i as isize)).flags |= MBOX_INITIALIZED;
    };
}

#[macro_export]
macro_rules! RESET_MAIL_FILE {
    ($i:expr) => {
        let ref mut fresh1 = (**mailfiles.offset($i as isize)).mod_time;
        *fresh1 = 0 as libc::c_int as time_t;
        (**mailfiles.offset($i as isize)).access_time = *fresh1;
        (**mailfiles.offset($i as isize)).file_size = 0 as libc::c_int as off_t;
        (**mailfiles.offset($i as isize)).flags = 0 as libc::c_int;
    };
}

pub const DEFAULT_MAIL_DIRECTORY: *const libc::c_char =
    b"/var/mail\0" as *const u8 as *const libc::c_char;

pub type time_t = libc::c_long;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct FILEINFO {
    pub name: *mut libc::c_char,
    pub msg: *mut libc::c_char,
    pub access_time: time_t,
    pub mod_time: time_t,
    pub file_size: off_t,
    pub flags: libc::c_int,
}

pub static mut mailfiles: *mut *mut FILEINFO =
    0 as *const libc::c_void as *mut libc::c_void as *mut *mut FILEINFO;
pub static mut mailfiles_count: libc::c_int = 0;
pub static mut last_time_mail_checked: time_t = 0 as libc::c_int as time_t;
#[no_mangle]
pub static mut mail_warning: libc::c_int = 0;

//utshell.rs
pub type rl_voidfunc_t = fn() -> ();

#[no_mangle]
pub static mut shell_initialized: libc::c_int = 0;
#[no_mangle]
pub static mut bash_argv_initialized: libc::c_int = 0;
#[no_mangle]
pub static mut global_command: *mut COMMAND = 0 as *mut COMMAND;
#[no_mangle]
pub static mut current_user: user_info = {
    let init = user_info {
        uid: -(1 as libc::c_int) as uid_t,
        euid: -(1 as libc::c_int) as uid_t,
        gid: -(1 as libc::c_int) as gid_t,
        egid: -(1 as libc::c_int) as gid_t,
        user_name: 0 as *mut libc::c_char,
        shell: 0 as *mut libc::c_char,
        home_dir: 0 as *mut libc::c_char,
    };
    init
};
#[no_mangle]
pub static mut current_host_name: *mut libc::c_char = 0 as *mut libc::c_char;
/* Non-zero means that this shell is a login shell.
  Specifically:
  0 = not login shell.
  1 = login shell from getty (or equivalent fake out)
 -1 = login shell from "--login" (or -l) flag.
 -2 = both from getty, and from flag.
*/
#[no_mangle]
pub static mut login_shell: libc::c_int = 0;

/* Non-zero means that at this moment, the shell is interactive.  In
general, this means that the shell is at this moment reading input
from the keyboard. */
#[no_mangle]
pub static mut interactive: libc::c_int = 0;
#[no_mangle]
pub static mut interactive_shell: libc::c_int = 0;
#[no_mangle]
pub static mut hup_on_exit: libc::c_int = 0;
#[no_mangle]
pub static mut check_jobs_at_exit: libc::c_int = 0;
#[no_mangle]
pub static mut autocd: libc::c_int = 0;
#[no_mangle]
pub static mut startup_state: libc::c_int = 0;
#[no_mangle]
pub static mut reading_shell_script: libc::c_int = 0;
#[no_mangle]
pub static mut debugging_login_shell: libc::c_int = 0;
#[no_mangle]
pub static mut shell_environment: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
#[no_mangle]
pub static mut executing: libc::c_int = 0;
#[no_mangle]
pub static mut current_command_number: libc::c_int = 1;
#[no_mangle]
pub static mut indirection_level: libc::c_int = 0;

#[no_mangle]
pub static mut shell_name: *mut libc::c_char = 0 as *mut libc::c_char;
#[no_mangle]
pub static mut shell_start_time: time_t = 0;
#[no_mangle]
pub static mut shellstart: timeval = timeval {
    tv_sec: 0,
    tv_usec: 0,
};
#[no_mangle]
pub static mut running_under_emacs: libc::c_int = 0;

#[macro_export]
macro_rules! HAVE_DEV_FD {
    () => {
        1
    };
}

#[macro_export]
macro_rules! DEFAULT_BASHRC {
    () => {
        // b"~/.bashrc\0" as *const u8 as *const libc::c_char as *mut libc::c_char
        b"~/.utshellrc\0" as *const u8 as *const libc::c_char as *mut libc::c_char
    };
}

#[no_mangle]
pub static mut have_devfd: libc::c_int = HAVE_DEV_FD!();

pub static mut bashrc_file: *mut libc::c_char = DEFAULT_BASHRC!();

#[no_mangle]
pub static mut rpm_requires: libc::c_int = 0;
#[no_mangle]
pub static mut debugging_mode: libc::c_int = 0;
#[no_mangle]
pub static mut no_line_editing: libc::c_int = 0;
#[no_mangle]
pub static mut dump_translatable_strings: libc::c_int = 0;
#[no_mangle]
pub static mut dump_po_strings: libc::c_int = 0;
#[no_mangle]
pub static mut wordexp_only: libc::c_int = 0;
#[no_mangle]
pub static mut protected_mode: libc::c_int = 0;
#[no_mangle]
pub static mut pretty_print_mode: libc::c_int = 0;
#[no_mangle]
pub static mut posixly_correct: libc::c_int = 0;

#[no_mangle]
pub static mut subshell_top_level: sigjmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
#[no_mangle]
pub static mut subshell_argc: libc::c_int = 0;
#[no_mangle]
pub static mut subshell_argv: *mut *mut libc::c_char =
    0 as *const *mut libc::c_char as *mut *mut libc::c_char;
#[no_mangle]
pub static mut subshell_envp: *mut *mut libc::c_char =
    0 as *const *mut libc::c_char as *mut *mut libc::c_char;
#[no_mangle]
pub static mut exec_argv0: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut default_buffered_input: libc::c_int = -(1 as libc::c_int);
#[no_mangle]
pub static mut read_from_stdin: libc::c_int = 0;
#[no_mangle]
pub static mut want_pending_command: libc::c_int = 0;
#[no_mangle]
pub static mut command_execution_string: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut shell_script_filename: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut malloc_trace_at_exit: libc::c_int = 0 as libc::c_int;

#[macro_export]
macro_rules! setjmp_sigs {
    ($x:expr) => {
        __sigsetjmp($x, 1)
    };
}

#[macro_export]
macro_rules! RL_STATE_TERMPREPPED {
    () => {
        0x0000004
    };
}

#[macro_export]
macro_rules! SYS_PROFILE {
    () => {
        b"/etc/profile\0" as *const u8 as *const libc::c_char
    };
}

#[macro_export]
macro_rules! RESTRICTED_SHELL_NAME {
    () => {
        b"rbash\0" as *const u8 as *const libc::c_char
    };
}

#[macro_export]
macro_rules! savestring {
    ($x:expr) => {
        libc::strcpy(
            libc::malloc((libc::strlen($x as *const libc::c_char) + 1) as usize)
                as *mut libc::c_char,
            $x,
        ) as *mut libc::c_char
    };
}

#[macro_export]
macro_rules! DEBUGGER_START_FILE {
    () => {
        b"/usr/local/share/bashdb/bashdb-main.inc\0" as *const u8 as *const libc::c_char
    };
}

#[macro_export]
macro_rules! PROGRAM {
    () => {
        b"utshell\0" as *const u8 as *const libc::c_char as *mut libc::c_char
    };
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}

#[macro_export]
macro_rules! PPROMPT {
    () => {
        b"\\s-\\v\\$ \0" as *const u8 as *const libc::c_char as *mut libc::c_char
    };
}

#[macro_export]
macro_rules! SPROMPT {
    () => {
        b"> \0" as *const u8 as *const libc::c_char as *mut libc::c_char
    };
}


//makecmd
#[no_mangle]
pub static mut here_doc_first_line: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut wdcache: sh_obj_cache_t = {
    let mut init = objcache {
        data: 0 as *const libc::c_void as *mut libc::c_void,
        cs: 0 as libc::c_int,
        nc: 0 as libc::c_int,
    };
    init
};
#[no_mangle]
pub static mut wlcache: sh_obj_cache_t = {
    let mut init = objcache {
        data: 0 as *const libc::c_void as *mut libc::c_void,
        cs: 0 as libc::c_int,
        nc: 0 as libc::c_int,
    };
    init
};

#[macro_export]
macro_rules! WDCACHESIZE {
    () => {
        128
    };
}

#[macro_export]
macro_rules! WLCACHESIZE {
    () => {
        128
    };
}

#[macro_export]
macro_rules! ocache_create {
    ($c:expr, $otype:ty, $n:expr) => {
        $c.data = malloc($n * (std::mem::size_of::<*mut $otype>() as libc::c_ulong) as usize);
        wdcache.cs = 128 as libc::c_int;
        wdcache.nc = 0 as libc::c_int;
    };
}

#[macro_export]
macro_rules! ocache_alloc {
    ($c:expr, $otype:ty, $r:expr) => {
        if $c.nc > 0 {
            $c.nc -= 1;
            $r = ($c.data as *mut *mut $otype).offset($c.nc as isize) as *mut $otype;
        } else {
            $r = malloc(std::mem::size_of::<$otype>() as usize) as *mut $otype;
        }
    };
}

#[macro_export]
macro_rules! W_NOGLOB {
    () => {
        1 << 5
    };
}

#[macro_export]
macro_rules! W_NOSPLIT {
    () => {
        1 << 4
    };
}

#[macro_export]
macro_rules! W_QUOTED {
    () => {
        1 << 1
    };
}

#[macro_export]
macro_rules! W_DQUOTE {
    () => {
        1 << 19
    };
}

#[macro_export]
macro_rules! W_NOPROCSUB {
    () => {
        1 << 20
    };
}

#[macro_export]
macro_rules! FASTCOPY {
    ($s:expr, $d:expr, $n:expr) => {
        libc::memcpy(
            $d as *mut libc::c_void,
            $s as *const libc::c_void,
            $n as libc::c_ulong as libc::size_t,
        );
    };
}

//pathexp
#[derive(Copy, Clone)]
#[repr(C)]
pub union __mbstate_u {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
}

pub type sh_ignore_func_t = fn(*const libc::c_char) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ign {
    pub val: *mut libc::c_char,
    pub len: libc::c_int,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ignorevar {
    pub varname: *mut libc::c_char,
    pub ignores: *mut ign,
    pub num_ignores: libc::c_int,
    pub last_ignoreval: *mut libc::c_char,
    pub item_func: Option<sh_iv_item_func_t>,
}
pub type sh_iv_item_func_t = fn(*mut ign) -> libc::c_int;

#[no_mangle]
pub static mut glob_dot_filenames: libc::c_int = 0;
#[no_mangle]
pub static mut extended_glob: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut glob_star: libc::c_int = 0 as libc::c_int;

//pcomplib
pub type hash_wfunc = fn(*mut BUCKET_CONTENTS) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct compspec {
    pub refcount: libc::c_int,
    pub actions: libc::c_ulong,
    pub options: libc::c_ulong,
    pub globpat: *mut libc::c_char,
    pub words: *mut libc::c_char,
    pub prefix: *mut libc::c_char,
    pub suffix: *mut libc::c_char,
    pub funcname: *mut libc::c_char,
    pub command: *mut libc::c_char,
    pub lcommand: *mut libc::c_char,
    pub filterpat: *mut libc::c_char,
}
pub type COMPSPEC = compspec;

#[no_mangle]
pub static mut prog_completes: *mut HASH_TABLE =
    0 as *const libc::c_void as *mut libc::c_void as *mut HASH_TABLE;

#[macro_export]
macro_rules! STRDUP {
    ($x:expr) => {
        if !($x).is_null() {
            savestring!($x)
        } else {
            0 as *mut libc::c_char
        }
    };
}

pub const COMPLETE_HASH_BUCKETS: libc::c_int = 256;

//print_cmd
pub type PFUNC = unsafe extern "C" fn(*const libc::c_char, ...) -> ();

pub const FUNC_MULTILINE: i32 = 0x01;
pub const FUNC_EXTERNAL: i32 = 0x02;

pub static mut indentation: libc::c_int = 0;
pub static mut indentation_amount: libc::c_int = 4;

pub const PRINTED_COMMAND_INITIAL_SIZE: libc::c_int = 64;
pub const PRINTED_COMMAND_CROW_SIZE: libc::c_int = 128;

pub static mut inside_function_def: libc::c_int = 0;
pub static mut skip_this_indent: libc::c_int = 0;
pub static mut was_heredoc: libc::c_int = 0;
pub static mut printing_connection: libc::c_int = 0;
pub static mut deferred_heredocs: *mut REDIRECT = 0 as *mut REDIRECT;

pub static mut group_command_nesting: libc::c_int = 0;

pub static mut indirection_string: *mut libc::c_char = 0 as *mut libc::c_char;
pub static mut indirection_stringsiz: libc::c_int = 0;

pub const MB_LEN_MAX: usize = 16; //在C端这是一个宏，
pub const MB_CUR_MAX: usize = 1;
pub const AND_AND: i32 = 288;
pub const OR_OR: i32 = 289;

/* 宏定义 */
#[macro_export]
macro_rules! CHECK_XTRACE_FP {
    () => {
        if !xtrace_fp.is_null() {
            xtrace_fp = xtrace_fp;
        } else {
            xtrace_fp = stderr;
        }
    };
}

#[macro_export]
macro_rules! EXPCHAR{
    ($c:expr) => (
        if $c == b'{' as libc::c_char || $c == b'~' as libc::c_char || $c == b'$' as libc::c_char || $c == b'`' as libc::c_char{
            1
        }
        else{
            0
        }
    )
}

#[macro_export]
macro_rules! PRINT_DEFERRED_HEREDOCS {
    ($x:expr) => {
        if !deferred_heredocs.is_null() {
            print_deferred_heredocs($x);
        }
    };
}

pub fn MBLEN(s: *const libc::c_char, n: size_t) -> libc::c_int {
    if MB_CUR_MAX > 1 {
        return unsafe { mblen(s, n) };
    } else {
        return 1;
    }
}

pub type Function = fn() -> libc::c_int;

#[no_mangle]
pub static mut the_printed_command: *mut libc::c_char = 0 as *mut libc::c_char;
#[no_mangle]
pub static mut the_printed_command_size: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut command_string_index: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut xtrace_fd: libc::c_int = -(1 as libc::c_int);
#[no_mangle]
pub static mut xtrace_fp: *mut FILE = 0 as *const FILE as *mut FILE;

//redir
#[repr(C)]
pub struct _IO_wide_data {
    pub _private: [u8; 0],
}
#[repr(C)]
pub struct _IO_codecvt {
    pub _private: [u8; 0],
}
#[repr(C)]
pub struct _IO_marker {
    pub _private: [u8; 0],
}

pub const S_IFMT: libc::c_uint = 0o170000;
pub const S_IFREG: libc::c_uint = 0o100000;
pub const RX_INTERNAL: libc::c_int = 0x8;
pub const RX_SAVCLEXEC: libc::c_int = 0x20;

pub const SUBSHELL_ASYNC: libc::c_int = 0x1;
pub const RX_CLEXEC: libc::c_int = 0x4;
pub const SHELL_FD_BASE: libc::c_int = 10;
pub const RX_SAVEFD: libc::c_int = 0x40;

#[macro_export]
macro_rules! REDIRECTION_ERROR {
    ($r:expr, $errno:expr, $fd:expr) => {
        if $r < 0 as libc::c_int {
            if $fd >= 0 as libc::c_int {
                close($fd);
            }
            set_exit_status(EXECUTION_FAILURE);
            return if *__errno_location() == 0 as libc::c_int {
                EINVAL
            } else {
                *__errno_location()
            };
        }
    };
}

#[macro_export]
macro_rules! S_ISREG {
    ($m:expr) => {
        $m & S_IFMT == S_IFREG
    };
}

#[macro_export]
macro_rules! CLOBBERING_REDIRECT {
    ($ri:expr) => {
        ($ri as libc::c_uint == r_output_direction as libc::c_int as libc::c_uint
            || $ri as libc::c_uint == r_err_and_out as libc::c_int as libc::c_uint)
    };
}

#[macro_export]
macro_rules! WRITE_REDIRECT {
    ($ri:expr) => {
        $ri as libc::c_uint == r_output_direction as libc::c_int as libc::c_uint
            || $ri as libc::c_uint == r_input_output as libc::c_int as libc::c_uint
            || $ri as libc::c_uint == r_err_and_out as libc::c_int as libc::c_uint
            || $ri as libc::c_uint == r_appending_to as libc::c_int as libc::c_uint
            || $ri as libc::c_uint == r_append_err_and_out as libc::c_int as libc::c_uint
            || $ri as libc::c_uint == r_output_force as libc::c_int as libc::c_uint
    };
}

// Redirection errors.
pub const AMBIGUOUS_REDIRECT: libc::c_int = -1;
pub const NOCLOBBER_REDIRECT: libc::c_int = -2;
pub const RESTRICTED_REDIRECT: libc::c_int = -3; /* can only happen in restricted shells. */
pub const HEREDOC_REDIRECT: libc::c_int = -4; /* here-doc temp file can't be created */
pub const BADVAR_REDIRECT: libc::c_int = -5; /* something wrong with {varname}redir */

pub const REDIR_VARASSIGN: libc::c_int = 0x01;
pub const HEREDOC_PIPESIZE: size_t = 65535;
pub const F_GETPIPE_SZ: libc::c_int = 1032;

pub const EPERM: libc::c_int = 1;
pub const ENOENT: libc::c_int = 2;
pub const ESRCH: libc::c_int = 3;

pub const EIO: libc::c_int = 5;
pub const ENXIO: libc::c_int = 6;
pub const E2BIG: libc::c_int = 7;
pub const ENOEXEC: libc::c_int = 8;

pub const ENOMEM: libc::c_int = 12;
pub const EACCES: libc::c_int = 13;
pub const EFAULT: libc::c_int = 14;
pub const ENOTBLK: libc::c_int = 15;
pub const EBUSY: libc::c_int = 16;
pub const EEXIST: libc::c_int = 17;
pub const EXDEV: libc::c_int = 18;
pub const ENODEV: libc::c_int = 19;
pub const ENOTDIR: libc::c_int = 20;
pub const EISDIR: libc::c_int = 21;

pub const ENFILE: libc::c_int = 23;
pub const EMFILE: libc::c_int = 24;

pub const ETXTBSY: libc::c_int = 26;
pub const EFBIG: libc::c_int = 27;
pub const ENOSPC: libc::c_int = 28;

pub const EROFS: libc::c_int = 30;
pub const EMLINK: libc::c_int = 31;
pub const EPIPE: libc::c_int = 32;
pub const EDOM: libc::c_int = 33;
pub const ERANGE: libc::c_int = 34;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct STRING_INT_ALIST {
    pub word: *mut libc::c_char,
    pub token: libc::c_int,
}

#[no_mangle]
pub static mut expanding_redir: libc::c_int = 0;

pub const RF_END: libc::c_int = -1;
pub const RF_DEVFD: libc::c_int = 1;
pub const RF_DEVSTDERR: libc::c_int = 2;
pub const RF_DEVSTDIN: libc::c_int = 3;
pub const RF_DEVSTDOUT: libc::c_int = 4;
pub const RF_DEVTCP: libc::c_int = 5;
pub const RF_DEVUDP: libc::c_int = 6;

//sig
#[no_mangle]
pub static mut interrupt_state: sig_atomic_t = 0 as libc::c_int;
#[no_mangle]
pub static mut sigwinch_received: sig_atomic_t = 0 as libc::c_int;
#[no_mangle]
pub static mut sigterm_received: sig_atomic_t = 0 as libc::c_int;
#[no_mangle]
pub static mut terminating_signal: sig_atomic_t = 0 as libc::c_int;
#[no_mangle]
pub static mut top_level: sigjmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
#[no_mangle]
pub static mut top_level_mask: sigset_t = __sigset_t { __val: [0; 16] };
#[no_mangle]
pub static mut interrupt_immediately: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut terminate_immediately: libc::c_int = 0 as libc::c_int;

#[macro_export]
macro_rules! TERMSIGS_LENGTH {
    () => {
        (::std::mem::size_of::<[termsig; 17]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<termsig>() as libc::c_ulong)
    };
}

#[macro_export]
macro_rules! XSIG {
    ($x:expr) => {
        terminating_signals[$x as usize].signum
    };
}

#[macro_export]
macro_rules! XHANDLER {
    ($x:expr) => {
        terminating_signals[$x as usize].orig_handler
    };
}

#[macro_export]
macro_rules! XSAFLAGS {
    ($x:expr) => {
        terminating_signals[$x as usize].orig_flags
    };
}

#[macro_export]
macro_rules! XCOREDUMP {
    ($x:expr) => {
        terminating_signals[$x as usize].core_dump
    };
}

#[macro_export]
macro_rules! RL_STATE_READCMD {
    () => {
        0x0000008
    };
}

#[macro_export]
macro_rules! RL_STATE_SIGHANDLER {
    () => {
        0x0008000
    };
}

//subst
pub type wint_t = libc::c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}

pub type __pid_t = libc::c_int;

pub type pid_t = __pid_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;

pub type C2RustUnnamed_0 = libc::c_uint;

pub const _ISpunct: C2RustUnnamed_0 = 4;
pub const _IScntrl: C2RustUnnamed_0 = 2;

pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISprint: C2RustUnnamed_0 = 16384;
pub const _ISspace: C2RustUnnamed_0 = 8192;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;

pub const _ISlower: C2RustUnnamed_0 = 512;
pub const _ISupper: C2RustUnnamed_0 = 256;

pub type sh_wassign_func_t = fn(*mut WORD_DESC, libc::c_int) -> libc::c_int;
pub type sh_builtin_func_t = fn(*mut WORD_LIST) -> libc::c_int;

pub type SigHandler = fn(libc::c_int) -> ();
pub type EXPFUNC = fn(*mut libc::c_char, libc::c_int) -> *mut WORD_LIST;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_bitmap {
    pub size: libc::c_int,
    pub bitmap: *mut libc::c_char,
}
pub type PROCESS = process;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct process {
    pub next: *mut process,
    pub pid: pid_t,
    pub status: WAIT,
    pub running: libc::c_int,
    pub command: *mut libc::c_char,
}
pub type WAIT = libc::c_int;

#[no_mangle]
pub static mut last_command_subst_pid: pid_t = -(1 as libc::c_int);
#[no_mangle]
pub static mut current_command_subst_pid: pid_t = -(1 as libc::c_int);
#[no_mangle]
pub static mut ifs_var: *mut SHELL_VAR = 0 as *const SHELL_VAR as *mut SHELL_VAR;
#[no_mangle]
pub static mut ifs_value: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut ifs_cmap: [libc::c_uchar; 256] = [0; 256];
#[no_mangle]
pub static mut ifs_is_set: libc::c_int = 0;
#[no_mangle]
pub static mut ifs_is_null: libc::c_int = 0;
#[no_mangle]
pub static mut ifs_firstc: [libc::c_uchar; 16] = [0; 16];
#[no_mangle]
pub static mut ifs_firstc_len: size_t = 0;
#[no_mangle]
pub static mut inherit_errexit: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut assigning_in_environment: libc::c_int = 0;
#[no_mangle]
pub static mut subst_assign_varlist: *mut WORD_LIST =
    0 as *const libc::c_void as *mut libc::c_void as *mut WORD_LIST;
#[no_mangle]
pub static mut no_longjmp_on_fatal_error: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut allow_null_glob_expansion: libc::c_int = 0;
#[no_mangle]
pub static mut fail_glob_expansion: libc::c_int = 0;
pub static mut cached_quoted_dollar_at: *mut WORD_LIST = 0 as *const WORD_LIST as *mut WORD_LIST;

pub static mut expand_param_error: libc::c_char = 0;
pub static mut expand_param_fatal: libc::c_char = 0;
pub static mut extract_string_error: libc::c_char = 0;
pub static mut extract_string_fatal: libc::c_char = 0;
pub static mut expand_no_split_dollar_star: libc::c_int = 0 as libc::c_int;
pub static mut garglist: *mut WORD_LIST =
    0 as *const libc::c_void as *mut libc::c_void as *mut WORD_LIST;

//test
pub const R_OK: libc::c_int = 4;
pub const X_OK: libc::c_int = 1;

#[macro_export]
macro_rules! FNMATCH_EXTFLAG {
    () => {
        if extended_glob != 0 {
            (1 as libc::c_int) << 5 as libc::c_int
        } else {
            0 as libc::c_int
        }
    };
}

pub const LE: libc::c_int = 4;
pub const GE: libc::c_int = 5;
pub const NE: libc::c_int = 1;
pub const EF: libc::c_int = 2;

pub const OT: libc::c_int = 1;
pub const NT: libc::c_int = 0;

//trap
pub const RL_STATE_SIGHANDLER: libc::c_ulong = 0x0008000;

#[macro_export]
macro_rules! CHECK_ALRM {
    () => {
        if sigalrm_seen != 0 {
            siglongjmp(alrmbuf.as_mut_ptr(), 1 as libc::c_int);
        }
    };
}

pub type __sighandler_t = Option<fn(libc::c_int) -> ()>;

pub type sh_parser_state_t = _sh_parser_state_t;

pub const NSIG: usize = 64;
pub const DEBUG_TRAP: usize = NSIG;
pub const ERROR_TRAP: usize = NSIG + 1;
pub const RETURN_TRAP: usize = NSIG + 2;
pub const BASH_NSIG: usize = NSIG + 3;
pub const EXIT_TRAP: usize = 0;

/* Flags which describe the current handling state of a signal. */
pub const SIG_INHERITED: libc::c_int = 0x0; /* Value inherited from parent. */
pub const SIG_TRAPPED: libc::c_int = 0x1; /* Currently trapped. */
pub const SIG_HARD_IGNORE: libc::c_int = 0x2; /* Signal was ignored on shell entry. */
pub const SIG_SPECIAL: libc::c_int = 0x4; /* Treat this signal specially. */
pub const SIG_NO_TRAP: libc::c_int = 0x8; /* Signal cannot be trapped. */
pub const SIG_INPROGRESS: libc::c_int = 0x10; /* Signal cannot be trapped. */
pub const SIG_CHANGED: libc::c_int = 0x20; /* Trap value changed in trap handler. */
pub const SIG_IGNORED: libc::c_int = 0x40; /* The signal is currently being ignored. */

#[macro_export]
macro_rules! GETORIGSIG {
    ($sig:expr) => {
        original_signals[$sig as libc::c_int as usize] =
            set_signal_handler($sig as libc::c_int, None);
        set_signal_handler(
            $sig as libc::c_int,
            original_signals[$sig as libc::c_int as usize],
        );
        if original_signals[$sig as libc::c_int as usize]
            == ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(
                libc::SIG_IGN as libc::intptr_t,
            )
        {
            sigmodes[$sig as libc::c_int as usize] |= SIG_HARD_IGNORE as libc::c_int;
        }
    };
}

#[macro_export]
macro_rules! SETORIGSIG {
    ($sig:expr, $handler:expr) => {
        original_signals[$sig] == handler;
        if original_signals[$sig] == libc::SIG_IGN {
            sigmodes[$sig] |= SIG_HARD_IGNORE;
        }
    };
}
#[macro_export]
macro_rules! GET_ORIGINAL_SIGNAL {
    ($sig: expr) => {
        if $sig && $sig < NSIG && original_signals[$sig] == initialize_traps {
            GETORIGSIG!($sig)
        }
    };
}

#[macro_export]
macro_rules! SPECIAL_TRAP {
    ($sig:expr) => {
        $sig == 0 as libc::c_int
            || $sig == 64 as libc::c_int + 1 as libc::c_int
            || $sig == 64 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
            || $sig == 64 as libc::c_int + 1 as libc::c_int + 2 as libc::c_int
    };
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sh_parser_state_t {
    pub parser_state: libc::c_int,
    pub token_state: *mut libc::c_int,
    pub token: *mut libc::c_char,
    pub token_buffer_size: libc::c_int,
    pub input_line_terminator: libc::c_int,
    pub eof_encountered: libc::c_int,
    pub prompt_string_pointer: *mut *mut libc::c_char,
    pub current_command_line_count: libc::c_int,
    pub remember_on_history: libc::c_int,
    pub history_expansion_inhibited: libc::c_int,
    pub last_command_exit_value: libc::c_int,
    pub pipestatus: *mut ARRAY,
    pub last_shell_builtin: Option<sh_builtin_func_t>,
    pub this_shell_builtin: Option<sh_builtin_func_t>,
    pub expand_aliases: libc::c_int,
    pub echo_input_at_read: libc::c_int,
    pub need_here_doc: libc::c_int,
    pub here_doc_first_line: libc::c_int,
    pub redir_stack: [*mut REDIRECT; 16],
}

pub type sh_resetsig_func_t = fn(libc::c_int) -> ();
#[no_mangle]
pub static mut signal_names: [*mut libc::c_char; 69] = [
    b"EXIT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGHUP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGINT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGQUIT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGILL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGTRAP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGABRT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGBUS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGFPE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGKILL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGUSR1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGSEGV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGUSR2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGPIPE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGALRM\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGTERM\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGSTKFLT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGCHLD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGCONT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGSTOP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGTSTP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGTTIN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGTTOU\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGURG\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGXCPU\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGXFSZ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGVTALRM\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGPROF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGWINCH\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGIO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGPWR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGSYS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGJUNK(32)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGJUNK(33)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMIN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMIN+1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMIN+2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMIN+3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMIN+4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMIN+5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMIN+6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMIN+7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMIN+8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMIN+9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMIN+10\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMIN+11\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMIN+12\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMIN+13\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMIN+14\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMIN+15\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMAX-14\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMAX-13\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMAX-12\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMAX-11\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMAX-10\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMAX-9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMAX-8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMAX-7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMAX-6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMAX-5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMAX-4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMAX-3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMAX-2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMAX-1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SIGRTMAX\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"DEBUG\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ERR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"RETURN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];

#[no_mangle]
pub static mut SigHandler: [Option<SigHandler>; 65] = [None; 65];
#[no_mangle]
pub static mut trap_list: [*mut libc::c_char; 68] =
    [0 as *const libc::c_char as *mut libc::c_char; 68];
#[no_mangle]
pub static mut pending_traps: [libc::c_int; 65] = [0; 65];
#[no_mangle]
pub static mut running_trap: libc::c_int = 0;
#[no_mangle]
pub static mut trap_saved_exit_value: libc::c_int = 0;
#[no_mangle]
pub static mut wait_signal_received: libc::c_int = 0;
#[no_mangle]
pub static mut trapped_signal_received: libc::c_int = 0;
#[no_mangle]
pub static mut suppress_debug_trap_verbose: libc::c_int = 0 as libc::c_int;

pub const EXITPROG: libc::c_int = 3;
pub const ERREXIT: libc::c_int = 4;

//unwind_port
pub type VFunction = fn() -> ();
pub type sh_obj_cache_t = objcache;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct objcache {
    pub data: *mut libc::c_void,
    pub cs: libc::c_int,
    pub nc: libc::c_int,
}
pub type UNWIND_ELT = uwp;
#[derive(Copy, Clone)]
#[repr(C)]
pub union uwp {
    pub head: uwp_head,
    pub arg: UWP_HEAD_0,
    pub sv: UWP_HEAD_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UWP_HEAD_1 {
    pub uwp_head: uwp_head,
    pub v: SAVED_VAR,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SAVED_VAR {
    pub variable: *mut libc::c_char,
    pub size: libc::c_int,
    pub desired_setting: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uwp_head {
    pub next: *mut uwp,
    pub cleanup: Option<Function>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UWP_HEAD_0 {
    pub uwp_head: uwp_head,
    pub v: *mut libc::c_char,
}
pub static mut unwind_protect_list: *mut UNWIND_ELT =
    0 as *const libc::c_void as *mut libc::c_void as *mut UNWIND_ELT;
#[no_mangle]
pub static mut uwcache: sh_obj_cache_t = {
    let mut init = objcache {
        data: 0 as *const libc::c_void as *mut libc::c_void,
        cs: 0 as libc::c_int,
        nc: 0 as libc::c_int,
    };
    init
};

//variables
pub type sh_string_func_t = fn(*mut libc::c_char) -> *mut libc::c_char;
pub type sh_sv_func_t = fn(*mut libc::c_char) -> ();

pub type sh_var_map_func_t = fn(*mut SHELL_VAR) -> libc::c_int;

#[macro_export]
macro_rules! Root_PS1_Value {
    () => {
        b"\\u@\\h:\\w# \0" as *const u8 as *mut libc::c_char
    };
}

#[macro_export]
macro_rules! PS1_Value {
    () => {
        b"\\u@\\h:\\w$ \0" as *const u8 as *mut libc::c_char
    };
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: libc::c_int,
    pub tz_dsttime: libc::c_int,
}
pub type __timezone_ptr_t = *mut timezone;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct name_and_function {
    pub name: *mut libc::c_char,
    pub function: Option<sh_sv_func_t>,
}

#[no_mangle]
pub static mut global_variables: *mut VAR_CONTEXT =
    0 as *const libc::c_void as *mut libc::c_void as *mut VAR_CONTEXT;

#[no_mangle]
pub static mut shell_variables: *mut VAR_CONTEXT =
    0 as *const libc::c_void as *mut libc::c_void as *mut VAR_CONTEXT;

#[no_mangle]
pub static mut shell_functions: *mut HASH_TABLE =
    0 as *const libc::c_void as *mut libc::c_void as *mut HASH_TABLE;

#[no_mangle]
pub static mut invalid_env: *mut HASH_TABLE =
    0 as *const libc::c_void as *mut libc::c_void as *mut HASH_TABLE;

#[no_mangle]
pub static mut shell_function_defs: *mut HASH_TABLE =
    0 as *const libc::c_void as *mut libc::c_void as *mut HASH_TABLE;

#[no_mangle]
pub static mut temporary_env: *mut HASH_TABLE =
    0 as *const libc::c_void as *mut libc::c_void as *mut HASH_TABLE;

#[no_mangle]
pub static mut variable_context: libc::c_int = 0 as libc::c_int;

#[no_mangle]
pub static mut localvar_inherit: libc::c_int = 0 as libc::c_int;

#[no_mangle]
pub static mut localvar_unset: libc::c_int = 0 as libc::c_int;

#[no_mangle]
pub static mut tempenv_assign_error: libc::c_int = 0;

#[no_mangle]
pub static mut dollar_vars: [*mut libc::c_char; 10] =
    [0 as *const libc::c_char as *mut libc::c_char; 10];

#[no_mangle]
pub static mut rest_of_args: *mut WORD_LIST =
    0 as *const libc::c_void as *mut libc::c_void as *mut WORD_LIST;

#[no_mangle]
pub static mut posparam_count: libc::c_int = 0 as libc::c_int;

#[no_mangle]
pub static mut dollar_dollar_pid: pid_t = 0;

#[no_mangle]
pub static mut array_needs_making: libc::c_int = 1 as libc::c_int;

#[no_mangle]
pub static mut shell_level: libc::c_int = 0 as libc::c_int;

#[no_mangle]
pub static mut export_env: *mut *mut libc::c_char =
    0 as *const libc::c_void as *mut libc::c_void as *mut *mut libc::c_char;

pub static mut export_env_index: libc::c_int = 0;
pub static mut export_env_size: libc::c_int = 0;
pub static mut winsize_assignment: libc::c_int = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct saved_dollar_vars {
    pub first_ten: *mut *mut libc::c_char,
    pub rest: *mut WORD_LIST,
    pub count: libc::c_int,
}

#[macro_export]
macro_rules! ASS_MKLOCAL {
    () => {
        0x0002
    };
}

#[macro_export]
macro_rules! DEFAULT_COMPAT_LEVEL {
    () => {
        01
    };
}

#[macro_export]
macro_rules! ASS_MKGLOBAL {
    () => {
        0x0008
    };
}

#[macro_export]
macro_rules! ASS_NOEVAL {
    () => {
        0x0100
    };
}

#[macro_export]
macro_rules! ASS_APPEND {
    () => {
        0x0001
    };
}

#[macro_export]
macro_rules! ASSOC_HASH_BUCKETS {
    () => {
        1024
    };
}

#[macro_export]
macro_rules! ASS_CHKLOCAL {
    () => {
        0x0040
    };
}

#[macro_export]
macro_rules! MKLOC_ARRAYOK {
    () => {
        0x02
    };
}

#[macro_export]
macro_rules! MKLOC_ASSOCOK {
    () => {
        0x01
    };
}

#[macro_export]
macro_rules! MKLOC_INHERIT {
    () => {
        0x04
    };
}

#[macro_export]
macro_rules! MIN_COMPAT_LEVEL {
    () => {
        31
    };
}

#[macro_export]
macro_rules! SAVE_EXPORTSTR {
    ($var:expr,$value:expr) => {
        if $value.is_null() {
            (*$var).exportstr = savestring!($value);
        } else {
            (*$var).exportstr = 0 as *mut libc::c_char;
        }
    };
}

#[macro_export]
macro_rules! set_element_value {
    ($ae:expr,$val:expr) => {
        (*$ae).value = $val;
    };
}

#[macro_export]
macro_rules! local_p {
    ($var:expr) => {
        (*$var).attributes & att_local as libc::c_int
    };
}

#[macro_export]
macro_rules! propagate_p {
    ($var:expr) => {
        (*$var).attributes & att_propagate as libc::c_int
    };
}

#[macro_export]
macro_rules! VC_FUNCENV {
    () => {
        0x04
    };
}

#[macro_export]
macro_rules! capcase_p {
    ($vc:expr) => {
        (*$vc).attributes & att_capcase as libc::c_int
    };
}

#[macro_export]
macro_rules! uppercase_p {
    ($vc:expr) => {
        (*$vc).attributes & att_uppercase as libc::c_int
    };
}

#[macro_export]
macro_rules! lowercase_p {
    ($vc:expr) => {
        (*$vc).attributes & att_lowercase as libc::c_int
    };
}

#[macro_export]
macro_rules! vc_isfuncenv {
    ($vc:expr) => {
        ((*$vc).flags & VC_FUNCENV as libc::c_int) != 0
    };
}

#[macro_export]
macro_rules! vc_istempenv {
    ($vc:expr) => {
        ((*$vc).flags & VC_TEMPFLAGS as libc::c_int) == VC_TEMPFLAGS as libc::c_int
    };
}

#[macro_export]
macro_rules! vc_istempscope {
    ($vc:expr) => {
        ((*$vc).flags & (VC_TEMPENV as libc::c_int | VC_BLTNENV as libc::c_int)) != 0 as libc::c_int
    };
}

#[macro_export]
macro_rules! vc_haslocals {
    ($vc:expr) => {
        ((*$vc).flags & VC_HASLOCAL as libc::c_int) != 0
    };
}

#[macro_export]
macro_rules! regen_p {
    ($var:expr) => {
        (*$var).attributes & att_regenerate as libc::c_int
    };
}

#[macro_export]
macro_rules! add_to_export_env {
    ($envstr:expr,$do_alloc:expr) => {{
        if export_env_index >= (export_env_size - 1 as libc::c_int) {
            export_env_size += 16 as libc::c_int;
            export_env = strvec_resize(export_env, export_env_size);
            environ = export_env;
        }
        *export_env.offset(export_env_index as isize) = if $do_alloc != 0 {
            savestring!($envstr)
        } else {
            $envstr
        };
        export_env_index += 1;
        *export_env.offset(export_env_index as isize) = 0 as *mut libc::c_void as *mut libc::c_char;
    }};
}

#[macro_export]
macro_rules! FREE_EXPORTSTR {
    ($var:expr) => {
        if !((*$var).exportstr.is_null()) {
            free((*$var).exportstr as *mut libc::c_void);
        }
    };
}

#[macro_export]
macro_rules! non_unsettable_p {
    ($var:expr) => {
        (*$var).attributes & (att_nounset as libc::c_int)
    };
}

#[macro_export]
macro_rules! NOW {
    () => {
        time(0 as *mut time_t)
    };
}

#[macro_export]
macro_rules! nofree_p {
    ($var:expr) => {
        (*$var).attributes & att_nofree as libc::c_int
    };
}

#[macro_export]
macro_rules! function_p {
    ($var:expr) => {
        (*$var).attributes & att_function as libc::c_int
    };
}

#[macro_export]
macro_rules! function_cell {
    ($var:expr) => {
        (*$var).value as *mut COMMAND
    };
}

#[macro_export]
macro_rules! CLEAR_EXPORTSTR {
    ($var:expr) => {
        (*$var).exportstr = 0 as *mut libc::c_void as *mut libc::c_char;
    };
}

#[macro_export]
macro_rules! var_setvalue {
    ($var:expr,$str:expr) => {
        (*$var).value = $str
    };
}

#[macro_export]
macro_rules! var_setfunc {
    ($var:expr,$func:expr) => {
        (*$var).value = $func as *mut libc::c_char;
    };
}

#[macro_export]
macro_rules! CACHE_IMPORTSTR {
    ($var:expr,$value:expr) => {
        (*$var).exportstr = savestring!($value);
    };
}

#[macro_export]
macro_rules! imported_p {
    ($var:expr) => {
        (*$var).attributes & (att_imported as libc::c_int)
    };
}

#[macro_export]
macro_rules! array_p {
    ($var:expr) => {
        (*$var).attributes & (att_array as libc::c_int)
    };
}

#[macro_export]
macro_rules! FIND_OR_MAKE_VARIABLE {
    ($name:expr,$entry:expr) => {
        $entry = find_variable($name);
        if !($entry.is_null()) {
            $entry = bind_variable(
                $name,
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0,
            );
            if !($entry.is_null()) {
                (*$entry).attributes |= att_invisible as libc::c_int;
            }
        }
    };
}

#[macro_export]
macro_rules! INIT_DYNAMIC_VAR {
    ($v:expr, $var:expr,$val:expr,$gfunc:expr,$afunc:expr) => {
        $v = bind_variable($var, ($val), 0 as libc::c_int);
        (*$v).dynamic_value = $gfunc;
        (*$v).assign_func = $afunc;
    };
}

#[macro_export]
macro_rules! INIT_DYNAMIC_ARRAY_VAR {
    ($v:expr,$var:expr,$gfunc:expr,$afunc:expr) => {
        $v = make_new_array_variable($var);
        (*$v).dynamic_value = $gfunc;
        (*$v).assign_func = $afunc;
    };
}

#[macro_export]
macro_rules! INIT_DYNAMIC_ASSOC_VAR {
    ($v:expr,$var:expr,$gfunc:expr,$afunc:expr) => {
        $v = make_new_assoc_variable($var);
        (*$v).dynamic_value = $gfunc;
        (*$v).assign_func = $afunc;
    };
}

#[macro_export]
macro_rules! set_auto_export {
    ($var:expr) => {
        (*$var).attributes |= att_exported as libc::c_int;
        array_needs_making = 1 as libc::c_int;
    };
}

#[macro_export]
macro_rules! RELPATH {
    ($x:expr) => {
        (*$x) != b'/' as u8 as libc::c_char
    };
}

#[macro_export]
macro_rules! assoc_copy {
    ($h:expr) => {
        hash_copy(
            $h,
            std::mem::transmute::<*mut libc::c_void, Option<sh_string_func_t>>(
                0 as *mut libc::c_void,
            ),
        )
    };
}

#[macro_export]
macro_rules! VARIABLES_HASH_BUCKETS {
    () => {
        1024
    };
}

#[macro_export]
macro_rules! FUNCTIONS_HASH_BUCKETS {
    () => {
        512
    };
}

#[macro_export]
macro_rules! TEMPENV_HASH_BUCKETS {
    () => {
        4
    };
}

#[macro_export]
macro_rules! BASHFUNC_PREFIX {
    () => {
        b"BASH_FUNC_\0" as *const u8 as *const libc::c_char as *mut libc::c_char
    };
}

#[macro_export]
macro_rules! BASHFUNC_PREFLEN {
    () => {
        10 as libc::c_int
    };
}

#[macro_export]
macro_rules! BASHFUNC_SUFFIX {
    () => {
        b"%%\0" as *const u8 as *const libc::c_char as *mut libc::c_char
    };
}

#[macro_export]
macro_rules! BASHFUNC_SUFFLEN {
    () => {
        2 as libc::c_int
    };
}

#[macro_export]
macro_rules! FV_FORCETEMPENV {
    () => {
        0x01
    };
}

#[macro_export]
macro_rules! FV_SKIPINVISIBLE {
    () => {
        0x02
    };
}

#[macro_export]
macro_rules! FV_NODYNAMIC {
    () => {
        0x04
    };
}

#[macro_export]
macro_rules! EXECUTION_FAILURE {
    () => {
        1 as libc::c_int
    };
}

#[macro_export]
macro_rules! DEFAULT_PATH_VALUE {
    () => {
        b"/usr/local/bin:/usr/local/sbin:/usr/bin:/usr/sbin:/bin:/sbin:.\0" as *const u8
            as *const libc::c_char as *mut libc::c_char
    };
}

#[no_mangle]
pub static mut nameref_invalid_value: SHELL_VAR = SHELL_VAR {
    name: 0 as *const libc::c_char as *mut libc::c_char,
    value: 0 as *const libc::c_char as *mut libc::c_char,
    exportstr: 0 as *const libc::c_char as *mut libc::c_char,
    dynamic_value: None,
    assign_func: None,
    attributes: 0,
    context: 0,
};

//r_bash
/* automatically generated by rust-bindgen */
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    storage: Storage,
    align: [Align; 0],
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn new(storage: Storage) -> Self {
        Self { storage, align: [] }
    }
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::core::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub fn new() -> Self {
        __IncompleteArrayField(::core::marker::PhantomData, [])
    }
    #[inline]
    pub unsafe fn as_ptr(&self) -> *const T {
        ::core::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
        ::core::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::core::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::core::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::core::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
impl<T> ::core::clone::Clone for __IncompleteArrayField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
pub const __EXTENSIONS__: u32 = 1;
pub const _ALL_SOURCE: u32 = 1;
pub const _GNU_SOURCE: u32 = 1;
pub const _POSIX_PTHREAD_SEMANTICS: u32 = 1;
pub const _TANDEM_SOURCE: u32 = 1;
pub const JOB_CONTROL: u32 = 1;
pub const ALIAS: u32 = 1;
pub const PUSHD_AND_POPD: u32 = 1;
pub const BRACE_EXPANSION: u32 = 1;
pub const READLINE: u32 = 1;
pub const BANG_HISTORY: u32 = 1;
pub const HISTORY: u32 = 1;
pub const HELP_BUILTIN: u32 = 1;
pub const RESTRICTED_SHELL: u32 = 1;
pub const PROCESS_SUBSTITUTION: u32 = 1;
pub const PROMPT_STRING_DECODE: u32 = 1;
pub const SELECT_COMMAND: u32 = 1;
pub const COMMAND_TIMING: u32 = 1;
pub const ARRAY_VARS: u32 = 1;
pub const DPAREN_ARITHMETIC: u32 = 1;
pub const EXTENDED_GLOB: u32 = 1;
pub const EXTGLOB_DEFAULT: u32 = 0;
pub const COND_COMMAND: u32 = 1;
pub const COND_REGEXP: u32 = 1;
pub const COPROCESS_SUPPORT: u32 = 1;
pub const ARITH_FOR_COMMAND: u32 = 1;
pub const NETWORK_REDIRECTIONS: u32 = 1;
pub const PROGRAMMABLE_COMPLETION: u32 = 1;
pub const DEBUGGER: u32 = 1;
pub const MEMSCRAMBLE: u32 = 1;
pub const CASEMOD_ATTRS: u32 = 1;
pub const CASEMOD_EXPANSIONS: u32 = 1;
pub const GLOBASCII_DEFAULT: u32 = 1;
pub const FUNCTION_IMPORT: u32 = 1;
pub const ENABLE_NLS: u32 = 1;
pub const DEFAULT_PATH_VALUE: &'static [u8; 63usize] =
    b"/usr/local/bin:/usr/local/sbin:/usr/bin:/usr/sbin:/bin:/sbin:.\0";
pub const STANDARD_UTILS_PATH: &'static [u8; 30usize] = b"/bin:/usr/bin:/usr/sbin:/sbin\0";
pub const PPROMPT: &'static [u8; 11usize] = b"\\s-\\\\v\\\\$ \0";
pub const SPROMPT: &'static [u8; 3usize] = b"> \0";
// pub const DEFAULT_BASHRC: &'static [u8; 10usize] = b"~/.bashrc\0";
pub const DEFAULT_BASHRC: &'static [u8; 13usize] = b"~/.utshellrc\0";
// pub const SYS_BASH_LOGOUT: &'static [u8; 22usize] = b"/etc/bash.bash_logout\0";     //只有定义，没有使用
pub const SYS_BASH_LOGOUT: &'static [u8; 28usize] = b"/etc/utshell.utshell_logout\0";
pub const MULTIPLE_COPROCS: u32 = 0;
pub const CHECKWINSIZE_DEFAULT: u32 = 1;
pub const OPTIMIZE_SEQUENTIAL_ARRAY_ASSIGNMENT: u32 = 1;
pub const CHECKHASH_DEFAULT: u32 = 0;
pub const EVALNEST_MAX: u32 = 0;
pub const SOURCENEST_MAX: u32 = 0;
pub const OLDPWD_CHECK_DIRECTORY: u32 = 1;
pub const HISTEXPAND_DEFAULT: u32 = 1;
pub const ASSOC_KVPAIR_ASSIGNMENT: u32 = 1;
pub const HAVE_STRINGIZE: u32 = 1;
pub const HAVE_LONG_DOUBLE: u32 = 1;
pub const PROTOTYPES: u32 = 1;
pub const __PROTOTYPES: u32 = 1;
pub const HAVE_LONG_LONG: u32 = 1;
pub const HAVE_UNSIGNED_LONG_LONG: u32 = 1;
pub const SIZEOF_INT: u32 = 4;
pub const SIZEOF_LONG: u32 = 8;
pub const SIZEOF_CHAR_P: u32 = 8;
pub const SIZEOF_DOUBLE: u32 = 8;
pub const SIZEOF_INTMAX_T: u32 = 8;
pub const SIZEOF_LONG_LONG: u32 = 8;
pub const SIZEOF_WCHAR_T: u32 = 4;

pub const STDC_HEADERS: u32 = 1;
pub const HAVE_ALLOCA: u32 = 1;
pub const HAVE_ALLOCA_H: u32 = 1;
pub const MAJOR_IN_SYSMACROS: u32 = 1;
pub const HAVE_MBSTATE_T: u32 = 1;
pub const HAVE_QUAD_T: u32 = 1;
pub const HAVE_WCHAR_T: u32 = 1;
pub const HAVE_WCTYPE_T: u32 = 1;
pub const HAVE_WINT_T: u32 = 1;
pub const HAVE_DECL_SYS_SIGLIST: u32 = 1;
pub const UNDER_SYS_SIGLIST_DECLARED: u32 = 1;
pub const HAVE_SYS_SIGLIST: u32 = 1;
pub const HAVE_UNDER_SYS_SIGLIST: u32 = 1;
pub const HAVE_SYS_ERRLIST: u32 = 1;
pub const HAVE_STRUCT_DIRENT_D_INO: u32 = 1;
pub const HAVE_STRUCT_DIRENT_D_FILENO: u32 = 1;
pub const FIONREAD_IN_SYS_IOCTL: u32 = 1;
pub const GWINSZ_IN_SYS_IOCTL: u32 = 1;
pub const STRUCT_WINSIZE_IN_SYS_IOCTL: u32 = 1;
pub const TERMIOS_LDISC: u32 = 1;
pub const TERMIO_LDISC: u32 = 1;
pub const HAVE_STRUCT_STAT_ST_BLOCKS: u32 = 1;
pub const HAVE_STRUCT_TM_TM_ZONE: u32 = 1;
pub const HAVE_TM_ZONE: u32 = 1;
pub const HAVE_TIMEVAL: u32 = 1;
pub const HAVE_STRUCT_TIMEZONE: u32 = 1;
pub const WEXITSTATUS_OFFSET: u32 = 8;
pub const HAVE_STRUCT_TIMESPEC: u32 = 1;
pub const TIME_H_DEFINES_STRUCT_TIMESPEC: u32 = 1;
pub const HAVE_STRUCT_STAT_ST_ATIM_TV_NSEC: u32 = 1;
pub const TYPEOF_STRUCT_STAT_ST_ATIM_IS_STRUCT_TIMESPEC: u32 = 1;
pub const HAVE_GETPW_DECLS: u32 = 1;
pub const HAVE_DECL_AUDIT_USER_TTY: u32 = 1;
pub const HAVE_DECL_CONFSTR: u32 = 1;
pub const HAVE_DECL_PRINTF: u32 = 1;
pub const HAVE_DECL_SBRK: u32 = 1;
pub const HAVE_DECL_STRCPY: u32 = 1;
pub const HAVE_DECL_STRSIGNAL: u32 = 1;
pub const HAVE_DECL_STRTOLD: u32 = 1;
pub const HAVE_DECL_STRTOIMAX: u32 = 1;
pub const HAVE_DECL_STRTOL: u32 = 1;
pub const HAVE_DECL_STRTOLL: u32 = 1;
pub const HAVE_DECL_STRTOUL: u32 = 1;
pub const HAVE_DECL_STRTOULL: u32 = 1;
pub const HAVE_DECL_STRTOUMAX: u32 = 1;
pub const GETPGRP_VOID: u32 = 1;
pub const PGRP_PIPE: u32 = 1;
pub const ULIMIT_MAXFDS: u32 = 1;
pub const CAN_REDEFINE_GETENV: u32 = 1;
pub const HAVE_STD_PUTENV: u32 = 1;
pub const HAVE_STD_UNSETENV: u32 = 1;
pub const HAVE_PRINTF_A_FORMAT: u32 = 1;
pub const HAVE_LANGINFO_CODESET: u32 = 1;
pub const HAVE_HASH_BANG_EXEC: u32 = 1;
pub const HAVE_DEV_FD: u32 = 1;
pub const DEV_FD_PREFIX: &'static [u8; 9usize] = b"/dev/fd/\0";
pub const HAVE_DEV_STDIN: u32 = 1;
pub const VOID_SIGHANDLER: u32 = 1;
pub const HAVE_POSIX_SIGNALS: u32 = 1;
pub const HAVE_ASPRINTF: u32 = 1;
pub const HAVE_BCOPY: u32 = 1;
pub const HAVE_BZERO: u32 = 1;
pub const HAVE_CHOWN: u32 = 1;
pub const HAVE_CONFSTR: u32 = 1;
pub const HAVE_DLCLOSE: u32 = 1;
pub const HAVE_DLOPEN: u32 = 1;
pub const HAVE_DLSYM: u32 = 1;
pub const HAVE_DPRINTF: u32 = 1;
pub const HAVE_DUP2: u32 = 1;
pub const HAVE_EACCESS: u32 = 1;
pub const HAVE_FACCESSAT: u32 = 1;
pub const HAVE_FCNTL: u32 = 1;
pub const HAVE_FNMATCH: u32 = 1;
pub const FNMATCH_EQUIV_FALLBACK: u32 = 1;
pub const HAVE___FPURGE: u32 = 1;
pub const HAVE_DECL_FPURGE: u32 = 0;
pub const HAVE_GETADDRINFO: u32 = 1;
pub const HAVE_GETCWD: u32 = 1;
pub const HAVE_GETENTROPY: u32 = 1;
pub const HAVE_GETDTABLESIZE: u32 = 1;
pub const HAVE_GETGROUPS: u32 = 1;
pub const HAVE_GETHOSTBYNAME: u32 = 1;
pub const HAVE_GETHOSTNAME: u32 = 1;
pub const HAVE_GETPAGESIZE: u32 = 1;
pub const HAVE_GETPEERNAME: u32 = 1;
pub const HAVE_GETPWENT: u32 = 1;
pub const HAVE_GETPWNAM: u32 = 1;
pub const HAVE_GETPWUID: u32 = 1;
pub const HAVE_GETRANDOM: u32 = 1;
pub const HAVE_GETRLIMIT: u32 = 1;
pub const HAVE_GETRUSAGE: u32 = 1;
pub const HAVE_GETSERVBYNAME: u32 = 1;
pub const HAVE_GETSERVENT: u32 = 1;
pub const HAVE_GETTIMEOFDAY: u32 = 1;
pub const HAVE_ICONV: u32 = 1;
pub const HAVE_IMAXDIV: u32 = 1;
pub const HAVE_INET_ATON: u32 = 1;
pub const HAVE_ISASCII: u32 = 1;
pub const HAVE_ISBLANK: u32 = 1;
pub const HAVE_ISGRAPH: u32 = 1;
pub const HAVE_ISPRINT: u32 = 1;
pub const HAVE_ISSPACE: u32 = 1;
pub const HAVE_ISWCTYPE: u32 = 1;
pub const HAVE_ISWLOWER: u32 = 1;
pub const HAVE_ISWUPPER: u32 = 1;
pub const HAVE_ISXDIGIT: u32 = 1;
pub const HAVE_KILL: u32 = 1;
pub const HAVE_KILLPG: u32 = 1;
pub const HAVE_LSTAT: u32 = 1;
pub const HAVE_MBRLEN: u32 = 1;
pub const HAVE_MBRTOWC: u32 = 1;
pub const HAVE_MBSNRTOWCS: u32 = 1;
pub const HAVE_MBSRTOWCS: u32 = 1;
pub const HAVE_MEMMOVE: u32 = 1;
pub const HAVE_MEMSET: u32 = 1;
pub const HAVE_MKDTEMP: u32 = 1;
pub const HAVE_MKFIFO: u32 = 1;
pub const HAVE_MKSTEMP: u32 = 1;
pub const HAVE_PATHCONF: u32 = 1;
pub const HAVE_PSELECT: u32 = 1;
pub const HAVE_PREAD: u32 = 1;
pub const HAVE_PUTENV: u32 = 1;
pub const HAVE_RAISE: u32 = 1;
pub const HAVE_RANDOM: u32 = 1;
pub const HAVE_READLINK: u32 = 1;
pub const HAVE_REGCOMP: u32 = 1;
pub const HAVE_REGEXEC: u32 = 1;
pub const HAVE_RENAME: u32 = 1;
pub const HAVE_SBRK: u32 = 1;
pub const HAVE_SELECT: u32 = 1;
pub const HAVE_SETENV: u32 = 1;
pub const HAVE_SETITIMER: u32 = 1;
pub const HAVE_SETLINEBUF: u32 = 1;
pub const HAVE_SETLOCALE: u32 = 1;
pub const HAVE_DECL_SETREGID: u32 = 1;
pub const HAVE_SETRESGID: u32 = 1;
pub const HAVE_SETRESUID: u32 = 1;
pub const HAVE_SETVBUF: u32 = 1;
pub const HAVE_SIGINTERRUPT: u32 = 1;
pub const HAVE_POSIX_SIGSETJMP: u32 = 1;
pub const HAVE_SNPRINTF: u32 = 1;
pub const HAVE_STRCASECMP: u32 = 1;
pub const HAVE_STRCASESTR: u32 = 1;
pub const HAVE_STRCHR: u32 = 1;
pub const HAVE_STRCHRNUL: u32 = 1;
pub const HAVE_STRCOLL: u32 = 1;
pub const HAVE_STRERROR: u32 = 1;
pub const HAVE_STRFTIME: u32 = 1;
pub const HAVE_STRNLEN: u32 = 1;
pub const HAVE_STRPBRK: u32 = 1;
pub const HAVE_STRSTR: u32 = 1;
pub const HAVE_STRTOD: u32 = 1;
pub const HAVE_STRTOIMAX: u32 = 1;
pub const HAVE_STRTOL: u32 = 1;
pub const HAVE_STRTOLL: u32 = 1;
pub const HAVE_STRTOUL: u32 = 1;
pub const HAVE_STRTOULL: u32 = 1;
pub const HAVE_STRTOUMAX: u32 = 1;
pub const HAVE_STRSIGNAL: u32 = 1;
pub const HAVE_SYSCONF: u32 = 1;
pub const HAVE_SYSLOG: u32 = 1;
pub const HAVE_TCGETATTR: u32 = 1;
pub const HAVE_TCGETPGRP: u32 = 1;
pub const HAVE_TIMES: u32 = 1;
pub const HAVE_TOWLOWER: u32 = 1;
pub const HAVE_TOWUPPER: u32 = 1;
pub const HAVE_TTYNAME: u32 = 1;
pub const HAVE_TZSET: u32 = 1;
pub const HAVE_ULIMIT: u32 = 1;
pub const HAVE_UNAME: u32 = 1;
pub const HAVE_UNSETENV: u32 = 1;
pub const HAVE_VASPRINTF: u32 = 1;
pub const HAVE_VPRINTF: u32 = 1;
pub const HAVE_VSNPRINTF: u32 = 1;
pub const HAVE_WAITPID: u32 = 1;
pub const HAVE_WAIT3: u32 = 1;
pub const HAVE_WCRTOMB: u32 = 1;
pub const HAVE_WCSCOLL: u32 = 1;
pub const HAVE_WCSDUP: u32 = 1;
pub const HAVE_WCTYPE: u32 = 1;
pub const HAVE_WCSWIDTH: u32 = 1;
pub const HAVE_WCWIDTH: u32 = 1;
pub const HAVE_ARPA_INET_H: u32 = 1;
pub const HAVE_DIRENT_H: u32 = 1;
pub const HAVE_DLFCN_H: u32 = 1;
pub const HAVE_GRP_H: u32 = 1;
pub const HAVE_INTTYPES_H: u32 = 1;
pub const HAVE_LANGINFO_H: u32 = 1;
pub const HAVE_LIMITS_H: u32 = 1;
pub const HAVE_LOCALE_H: u32 = 1;
pub const HAVE_NETDB_H: u32 = 1;
pub const HAVE_NETINET_IN_H: u32 = 1;
pub const HAVE_PWD_H: u32 = 1;
pub const HAVE_REGEX_H: u32 = 1;
pub const HAVE_STDLIB_H: u32 = 1;
pub const HAVE_STDARG_H: u32 = 1;
pub const HAVE_STRING_H: u32 = 1;
pub const HAVE_STRINGS_H: u32 = 1;
pub const HAVE_MEMORY_H: u32 = 1;
pub const HAVE_STDBOOL_H: u32 = 1;
pub const HAVE_STDDEF_H: u32 = 1;
pub const HAVE_STDINT_H: u32 = 1;
pub const HAVE_SYSLOG_H: u32 = 1;
pub const HAVE_SYS_FILE_H: u32 = 1;
pub const HAVE_SYS_IOCTL_H: u32 = 1;
pub const HAVE_SYS_MMAN_H: u32 = 1;
pub const HAVE_SYS_PARAM_H: u32 = 1;
pub const HAVE_SYS_RANDOM_H: u32 = 1;
pub const HAVE_SYS_RESOURCE_H: u32 = 1;
pub const HAVE_SYS_SELECT_H: u32 = 1;
pub const HAVE_SYS_SOCKET_H: u32 = 1;
pub const HAVE_SYS_STAT_H: u32 = 1;
pub const HAVE_SYS_TIME_H: u32 = 1;
pub const TIME_WITH_SYS_TIME: u32 = 1;
pub const HAVE_SYS_TIMES_H: u32 = 1;
pub const HAVE_SYS_TYPES_H: u32 = 1;
pub const HAVE_SYS_WAIT_H: u32 = 1;
pub const HAVE_TERMCAP_H: u32 = 1;
pub const HAVE_TERMIO_H: u32 = 1;
pub const HAVE_TERMIOS_H: u32 = 1;
pub const HAVE_ULIMIT_H: u32 = 1;
pub const HAVE_UNISTD_H: u32 = 1;
pub const HAVE_WCHAR_H: u32 = 1;
pub const HAVE_WCTYPE_H: u32 = 1;
pub const HAVE_LIBDL: u32 = 1;
pub const HAVE_ARGZ_H: u32 = 1;
pub const HAVE_ERRNO_H: u32 = 1;
pub const HAVE_FCNTL_H: u32 = 1;
pub const HAVE_MALLOC_H: u32 = 1;
pub const HAVE_STDIO_EXT_H: u32 = 1;
pub const HAVE_DCGETTEXT: u32 = 1;
pub const HAVE_LOCALECONV: u32 = 1;
pub const HAVE_MEMPCPY: u32 = 1;
pub const HAVE_MMAP: u32 = 1;
pub const HAVE_MREMAP: u32 = 1;
pub const HAVE_MUNMAP: u32 = 1;
pub const HAVE_STPCPY: u32 = 1;
pub const HAVE_STRCSPN: u32 = 1;
pub const HAVE_STRDUP: u32 = 1;
pub const HAVE___ARGZ_COUNT: u32 = 1;
pub const HAVE___ARGZ_NEXT: u32 = 1;
pub const HAVE___ARGZ_STRINGIFY: u32 = 1;
pub const RESTRICTED_SHELL_NAME: &'static [u8; 6usize] = b"rbash\0";
pub const _WCHAR_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _ISOC95_SOURCE: u32 = 1;
pub const _ISOC99_SOURCE: u32 = 1;
pub const _ISOC11_SOURCE: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const _XOPEN_SOURCE: u32 = 700;
pub const _XOPEN_SOURCE_EXTENDED: u32 = 1;
pub const _LARGEFILE64_SOURCE: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const __USE_XOPEN: u32 = 1;
pub const __USE_XOPEN_EXTENDED: u32 = 1;
pub const __USE_UNIX98: u32 = 1;
pub const _LARGEFILE_SOURCE: u32 = 1;
pub const __USE_XOPEN2K8XSI: u32 = 1;
pub const __USE_XOPEN2KXSI: u32 = 1;
pub const __USE_LARGEFILE: u32 = 1;
pub const __USE_LARGEFILE64: u32 = 1;
pub const __USE_MISC: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_GNU: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 28;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const __HAVE_GENERIC_SELECTION: u32 = 1;
pub const __GLIBC_USE_LIB_EXT2: u32 = 1;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 1;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 1;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 1;
pub const __HAVE_FLOAT128: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT128: u32 = 0;
pub const __HAVE_FLOAT64X: u32 = 1;
pub const __HAVE_FLOAT64X_LONG_DOUBLE: u32 = 1;
pub const __HAVE_FLOAT16: u32 = 0;
pub const __HAVE_FLOAT32: u32 = 1;
pub const __HAVE_FLOAT64: u32 = 1;
pub const __HAVE_FLOAT32X: u32 = 1;
pub const __HAVE_FLOAT128X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT16: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT128X: u32 = 0;
pub const __HAVE_FLOATN_NOT_TYPEDEF: u32 = 0;
pub const __GNUC_VA_LIST: u32 = 1;
pub const _BITS_WCHAR_H: u32 = 1;
pub const __wint_t_defined: u32 = 1;
pub const _WINT_T: u32 = 1;
pub const __mbstate_t_defined: u32 = 1;
pub const ____mbstate_t_defined: u32 = 1;
pub const ____FILE_defined: u32 = 1;
pub const __FILE_defined: u32 = 1;
pub const _BITS_TYPES_LOCALE_T_H: u32 = 1;
pub const _BITS_TYPES___LOCALE_T_H: u32 = 1;
pub const WEOF: u32 = 4294967295;
pub const _WCTYPE_H: u32 = 1;
pub const _BITS_TYPES_H: u32 = 1;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
pub const __INO_T_MATCHES_INO64_T: u32 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
pub const __FD_SETSIZE: u32 = 1024;
pub const _BITS_WCTYPE_WCHAR_H: u32 = 1;
pub const _ENDIAN_H: u32 = 1;
pub const __LITTLE_ENDIAN: u32 = 1234;
pub const __BIG_ENDIAN: u32 = 4321;
pub const __PDP_ENDIAN: u32 = 3412;
pub const __BYTE_ORDER: u32 = 1234;
pub const __FLOAT_WORD_ORDER: u32 = 1234;
pub const LITTLE_ENDIAN: u32 = 1234;
pub const BIG_ENDIAN: u32 = 4321;
pub const PDP_ENDIAN: u32 = 3412;
pub const BYTE_ORDER: u32 = 1234;
pub const _BITS_BYTESWAP_H: u32 = 1;
pub const _BITS_UINTN_IDENTITY_H: u32 = 1;
pub const HANDLE_MULTIBYTE: u32 = 1;
pub const _LIBC_LIMITS_H_: u32 = 1;

pub const CHAR_WIDTH: u32 = 8;
pub const SCHAR_WIDTH: u32 = 8;
pub const UCHAR_WIDTH: u32 = 8;
pub const SHRT_WIDTH: u32 = 16;
pub const USHRT_WIDTH: u32 = 16;
pub const INT_WIDTH: u32 = 32;
pub const UINT_WIDTH: u32 = 32;
pub const LONG_WIDTH: u32 = 64;
pub const ULONG_WIDTH: u32 = 64;
pub const LLONG_WIDTH: u32 = 64;
pub const ULLONG_WIDTH: u32 = 64;
pub const _BITS_POSIX1_LIM_H: u32 = 1;
pub const _POSIX_AIO_LISTIO_MAX: u32 = 2;
pub const _POSIX_AIO_MAX: u32 = 1;
pub const _POSIX_ARG_MAX: u32 = 4096;
pub const _POSIX_CHILD_MAX: u32 = 25;
pub const _POSIX_DELAYTIMER_MAX: u32 = 32;
pub const _POSIX_HOST_NAME_MAX: u32 = 255;
pub const _POSIX_LINK_MAX: u32 = 8;
pub const _POSIX_LOGIN_NAME_MAX: u32 = 9;
pub const _POSIX_MAX_CANON: u32 = 255;
pub const _POSIX_MAX_INPUT: u32 = 255;
pub const _POSIX_MQ_OPEN_MAX: u32 = 8;
pub const _POSIX_MQ_PRIO_MAX: u32 = 32;
pub const _POSIX_NAME_MAX: u32 = 14;
pub const _POSIX_NGROUPS_MAX: u32 = 8;
pub const _POSIX_OPEN_MAX: u32 = 20;
pub const _POSIX_FD_SETSIZE: u32 = 20;
pub const _POSIX_PATH_MAX: u32 = 256;
pub const _POSIX_PIPE_BUF: u32 = 512;
pub const _POSIX_RE_DUP_MAX: u32 = 255;
pub const _POSIX_RTSIG_MAX: u32 = 8;
pub const _POSIX_SEM_NSEMS_MAX: u32 = 256;
pub const _POSIX_SEM_VALUE_MAX: u32 = 32767;
pub const _POSIX_SIGQUEUE_MAX: u32 = 32;
pub const _POSIX_SSIZE_MAX: u32 = 32767;
pub const _POSIX_STREAM_MAX: u32 = 8;
pub const _POSIX_SYMLINK_MAX: u32 = 255;
pub const _POSIX_SYMLOOP_MAX: u32 = 8;
pub const _POSIX_TIMER_MAX: u32 = 32;
pub const _POSIX_TTY_NAME_MAX: u32 = 9;
pub const _POSIX_TZNAME_MAX: u32 = 6;
pub const _POSIX_QLIMIT: u32 = 1;
pub const _POSIX_HIWAT: u32 = 512;
pub const _POSIX_UIO_MAXIOV: u32 = 16;
pub const _POSIX_CLOCKRES_MIN: u32 = 20000000;
pub const NR_OPEN: u32 = 1024;
pub const NGROUPS_MAX: u32 = 65536;
pub const ARG_MAX: u32 = 131072;
pub const LINK_MAX: u32 = 127;
pub const MAX_CANON: u32 = 255;
pub const MAX_INPUT: u32 = 255;
pub const NAME_MAX: u32 = 255;

pub const PIPE_BUF: u32 = 4096;
pub const XATTR_NAME_MAX: u32 = 255;
pub const XATTR_SIZE_MAX: u32 = 65536;
pub const XATTR_LIST_MAX: u32 = 65536;
pub const RTSIG_MAX: u32 = 32;
pub const _POSIX_THREAD_KEYS_MAX: u32 = 128;
pub const PTHREAD_KEYS_MAX: u32 = 1024;
pub const _POSIX_THREAD_DESTRUCTOR_ITERATIONS: u32 = 4;
pub const PTHREAD_DESTRUCTOR_ITERATIONS: u32 = 4;
pub const _POSIX_THREAD_THREADS_MAX: u32 = 64;
pub const AIO_PRIO_DELTA_MAX: u32 = 20;
pub const PTHREAD_STACK_MIN: u32 = 16384;
pub const DELAYTIMER_MAX: u32 = 2147483647;
pub const TTY_NAME_MAX: u32 = 32;
pub const LOGIN_NAME_MAX: u32 = 256;
pub const HOST_NAME_MAX: u32 = 64;
pub const MQ_PRIO_MAX: u32 = 32768;
pub const SEM_VALUE_MAX: u32 = 2147483647;
pub const _BITS_POSIX2_LIM_H: u32 = 1;
pub const _POSIX2_BC_BASE_MAX: u32 = 99;
pub const _POSIX2_BC_DIM_MAX: u32 = 2048;
pub const _POSIX2_BC_SCALE_MAX: u32 = 99;
pub const _POSIX2_BC_STRING_MAX: u32 = 1000;
pub const _POSIX2_COLL_WEIGHTS_MAX: u32 = 2;
pub const _POSIX2_EXPR_NEST_MAX: u32 = 32;
pub const _POSIX2_LINE_MAX: u32 = 2048;
pub const _POSIX2_RE_DUP_MAX: u32 = 255;
pub const _POSIX2_CHARCLASS_NAME_MAX: u32 = 14;
pub const BC_BASE_MAX: u32 = 99;
pub const BC_DIM_MAX: u32 = 2048;
pub const BC_SCALE_MAX: u32 = 99;
pub const BC_STRING_MAX: u32 = 1000;
pub const COLL_WEIGHTS_MAX: u32 = 255;
pub const EXPR_NEST_MAX: u32 = 32;
pub const LINE_MAX: u32 = 2048;
pub const CHARCLASS_NAME_MAX: u32 = 2048;
pub const RE_DUP_MAX: u32 = 32767;
pub const _XOPEN_LIM_H: u32 = 1;
pub const _XOPEN_IOV_MAX: u32 = 16;
pub const _BITS_UIO_LIM_H: u32 = 1;
pub const __IOV_MAX: u32 = 1024;
pub const IOV_MAX: u32 = 1024;
pub const NL_ARGMAX: u32 = 4096;
pub const NL_LANGMAX: u32 = 2048;
pub const NZERO: u32 = 20;
pub const WORD_BIT: u32 = 32;
pub const LONG_BIT: u32 = 64;
pub const _UNISTD_H: u32 = 1;
pub const _POSIX_VERSION: u32 = 200809;
pub const __POSIX2_THIS_VERSION: u32 = 200809;
pub const _POSIX2_VERSION: u32 = 200809;
pub const _POSIX2_C_VERSION: u32 = 200809;
pub const _POSIX2_C_BIND: u32 = 200809;
pub const _POSIX2_C_DEV: u32 = 200809;
pub const _POSIX2_SW_DEV: u32 = 200809;
pub const _POSIX2_LOCALEDEF: u32 = 200809;
pub const _XOPEN_VERSION: u32 = 700;
pub const _XOPEN_XCU_VERSION: u32 = 4;
pub const _XOPEN_XPG2: u32 = 1;
pub const _XOPEN_XPG3: u32 = 1;
pub const _XOPEN_XPG4: u32 = 1;
pub const _XOPEN_UNIX: u32 = 1;
pub const _XOPEN_ENH_I18N: u32 = 1;
pub const _XOPEN_LEGACY: u32 = 1;
pub const _BITS_POSIX_OPT_H: u32 = 1;
pub const _POSIX_JOB_CONTROL: u32 = 1;
pub const _POSIX_SAVED_IDS: u32 = 1;
pub const _POSIX_PRIORITY_SCHEDULING: u32 = 200809;
pub const _POSIX_SYNCHRONIZED_IO: u32 = 200809;
pub const _POSIX_FSYNC: u32 = 200809;
pub const _POSIX_MAPPED_FILES: u32 = 200809;
pub const _POSIX_MEMLOCK: u32 = 200809;
pub const _POSIX_MEMLOCK_RANGE: u32 = 200809;
pub const _POSIX_MEMORY_PROTECTION: u32 = 200809;
pub const _POSIX_CHOWN_RESTRICTED: u32 = 0;
pub const _POSIX_VDISABLE: u8 = 0u8;
pub const _POSIX_NO_TRUNC: u32 = 1;
pub const _XOPEN_REALTIME: u32 = 1;
pub const _XOPEN_REALTIME_THREADS: u32 = 1;
pub const _XOPEN_SHM: u32 = 1;
pub const _POSIX_THREADS: u32 = 200809;
pub const _POSIX_REENTRANT_FUNCTIONS: u32 = 1;
pub const _POSIX_THREAD_SAFE_FUNCTIONS: u32 = 200809;
pub const _POSIX_THREAD_PRIORITY_SCHEDULING: u32 = 200809;
pub const _POSIX_THREAD_ATTR_STACKSIZE: u32 = 200809;
pub const _POSIX_THREAD_ATTR_STACKADDR: u32 = 200809;
pub const _POSIX_THREAD_PRIO_INHERIT: u32 = 200809;
pub const _POSIX_THREAD_PRIO_PROTECT: u32 = 200809;
pub const _POSIX_THREAD_ROBUST_PRIO_INHERIT: u32 = 200809;
pub const _POSIX_THREAD_ROBUST_PRIO_PROTECT: i32 = -1;
pub const _POSIX_SEMAPHORES: u32 = 200809;
pub const _POSIX_REALTIME_SIGNALS: u32 = 200809;
pub const _POSIX_ASYNCHRONOUS_IO: u32 = 200809;
pub const _POSIX_ASYNC_IO: u32 = 1;
pub const _LFS_ASYNCHRONOUS_IO: u32 = 1;
pub const _POSIX_PRIORITIZED_IO: u32 = 200809;
pub const _LFS64_ASYNCHRONOUS_IO: u32 = 1;
pub const _LFS_LARGEFILE: u32 = 1;
pub const _LFS64_LARGEFILE: u32 = 1;
pub const _LFS64_STDIO: u32 = 1;
pub const _POSIX_SHARED_MEMORY_OBJECTS: u32 = 200809;
pub const _POSIX_CPUTIME: u32 = 0;
pub const _POSIX_THREAD_CPUTIME: u32 = 0;
pub const _POSIX_REGEXP: u32 = 1;
pub const _POSIX_READER_WRITER_LOCKS: u32 = 200809;
pub const _POSIX_SHELL: u32 = 1;
pub const _POSIX_TIMEOUTS: u32 = 200809;
pub const _POSIX_SPIN_LOCKS: u32 = 200809;
pub const _POSIX_SPAWN: u32 = 200809;
pub const _POSIX_TIMERS: u32 = 200809;
pub const _POSIX_BARRIERS: u32 = 200809;
pub const _POSIX_MESSAGE_PASSING: u32 = 200809;
pub const _POSIX_THREAD_PROCESS_SHARED: u32 = 200809;
pub const _POSIX_MONOTONIC_CLOCK: u32 = 0;
pub const _POSIX_CLOCK_SELECTION: u32 = 200809;
pub const _POSIX_ADVISORY_INFO: u32 = 200809;
pub const _POSIX_IPV6: u32 = 200809;
pub const _POSIX_RAW_SOCKETS: u32 = 200809;
pub const _POSIX2_CHAR_TERM: u32 = 200809;
pub const _POSIX_SPORADIC_SERVER: i32 = -1;
pub const _POSIX_THREAD_SPORADIC_SERVER: i32 = -1;
pub const _POSIX_TRACE: i32 = -1;
pub const _POSIX_TRACE_EVENT_FILTER: i32 = -1;
pub const _POSIX_TRACE_INHERIT: i32 = -1;
pub const _POSIX_TRACE_LOG: i32 = -1;
pub const _POSIX_TYPED_MEMORY_OBJECTS: i32 = -1;
pub const _POSIX_V7_LPBIG_OFFBIG: i32 = -1;
pub const _POSIX_V6_LPBIG_OFFBIG: i32 = -1;
pub const _XBS5_LPBIG_OFFBIG: i32 = -1;
pub const _POSIX_V7_LP64_OFF64: u32 = 1;
pub const _POSIX_V6_LP64_OFF64: u32 = 1;
pub const _XBS5_LP64_OFF64: u32 = 1;
pub const __ILP32_OFF32_CFLAGS: &'static [u8; 5usize] = b"-m32\0";
pub const __ILP32_OFF32_LDFLAGS: &'static [u8; 5usize] = b"-m32\0";
pub const __ILP32_OFFBIG_CFLAGS: &'static [u8; 48usize] =
    b"-m32 -D_LARGEFILE_SOURCE -D_FILE_OFFSET_BITS=64\0";
pub const __ILP32_OFFBIG_LDFLAGS: &'static [u8; 5usize] = b"-m32\0";
pub const __LP64_OFF64_CFLAGS: &'static [u8; 5usize] = b"-m64\0";
pub const __LP64_OFF64_LDFLAGS: &'static [u8; 5usize] = b"-m64\0";
pub const STDIN_FILENO: u32 = 0;
pub const STDOUT_FILENO: u32 = 1;
pub const STDERR_FILENO: u32 = 2;

pub const F_OK: u32 = 0;
pub const SEEK_SET: u32 = 0;

pub const SEEK_END: u32 = 2;
pub const SEEK_DATA: u32 = 3;
pub const SEEK_HOLE: u32 = 4;
pub const L_SET: u32 = 0;
pub const L_INCR: u32 = 1;
pub const L_XTND: u32 = 2;
pub const _GETOPT_POSIX_H: u32 = 1;
pub const _GETOPT_CORE_H: u32 = 1;
pub const F_ULOCK: u32 = 0;
pub const F_LOCK: u32 = 1;
pub const F_TLOCK: u32 = 2;
pub const F_TEST: u32 = 3;
pub const _BITS_SIGNUM_H: u32 = 1;
pub const _BITS_SIGNUM_GENERIC_H: u32 = 1;
pub const SIGINT: u32 = 2;
pub const SIGILL: u32 = 4;
pub const SIGABRT: u32 = 6;
pub const SIGFPE: u32 = 8;
pub const SIGSEGV: u32 = 11;
pub const SIGTERM: u32 = 15;
pub const SIGHUP: u32 = 1;
pub const SIGQUIT: u32 = 3;
pub const SIGTRAP: u32 = 5;
pub const SIGKILL: u32 = 9;
// pub const SIGBUS: u32 = 10;
pub const SIGBUS: u32 = 7;
// pub const SIGSYS: u32 = 12;
pub const SIGSYS: u32 = 31;
pub const SIGPIPE: u32 = 13;
pub const SIGALRM: u32 = 14;
// pub const SIGURG: u32 = 16;
pub const SIGURG: u32 = 23;
// pub const SIGSTOP: u32 = 17;
pub const SIGSTOP: u32 = 19;
// pub const SIGTSTP: u32 = 18;
pub const SIGTSTP: u32 = 20;
// pub const SIGCONT: u32 = 19;
pub const SIGCONT: u32 = 18;
// pub const SIGCHLD: u32 = 20;
pub const SIGCHLD: u32 = 17;
pub const SIGTTIN: u32 = 21;
pub const SIGTTOU: u32 = 22;
pub const SIGPOLL: u32 = 23;
pub const SIGXCPU: u32 = 24;
pub const SIGXFSZ: u32 = 25;
pub const SIGVTALRM: u32 = 26;
pub const SIGPROF: u32 = 27;
// pub const SIGUSR1: u32 = 30;
pub const SIGUSR1: u32 = 10;
// pub const SIGUSR2: u32 = 31;
pub const SIGUSR2: u32 = 12;
pub const SIGWINCH: u32 = 28;
// pub const SIGIO: u32 = 23;
pub const SIGIO: u32 = 29;
pub const SIGIOT: u32 = 6;
pub const SIGCLD: u32 = 20;
pub const __SIGRTMIN: u32 = 32;
pub const __SIGRTMAX: u32 = 32;
pub const _NSIG: u32 = 33;
pub const SIGSTKFLT: u32 = 16;
pub const SIGPWR: u32 = 30;
pub const __sig_atomic_t_defined: u32 = 1;
pub const __sigset_t_defined: u32 = 1;
pub const _STRUCT_TIMESPEC: u32 = 1;
pub const __siginfo_t_defined: u32 = 1;
pub const __SI_MAX_SIZE: u32 = 128;
pub const _BITS_SIGINFO_ARCH_H: u32 = 1;
pub const __SI_ERRNO_THEN_CODE: u32 = 1;
pub const __SI_HAVE_SIGSYS: u32 = 1;
pub const _BITS_SIGINFO_CONSTS_H: u32 = 1;
pub const __SI_ASYNCIO_AFTER_SIGIO: u32 = 1;
pub const _BITS_SIGINFO_CONSTS_ARCH_H: u32 = 1;
pub const __sigevent_t_defined: u32 = 1;
pub const __SIGEV_MAX_SIZE: u32 = 64;
pub const __have_pthread_attr_t: u32 = 1;
pub const _BITS_SIGEVENT_CONSTS_H: u32 = 1;

pub const _BITS_SIGACTION_H: u32 = 1;
pub const SA_NOCLDSTOP: u32 = 1;
pub const SA_NOCLDWAIT: u32 = 2;
pub const SA_SIGINFO: u32 = 4;
pub const SA_ONSTACK: u32 = 134217728;
pub const SA_RESTART: u32 = 268435456;
pub const SA_NODEFER: u32 = 1073741824;
pub const SA_RESETHAND: u32 = 2147483648;
pub const SA_INTERRUPT: u32 = 536870912;
pub const SA_NOMASK: u32 = 1073741824;
pub const SA_ONESHOT: u32 = 2147483648;
pub const SA_STACK: u32 = 134217728;
pub const SIG_BLOCK: u32 = 0;
pub const SIG_UNBLOCK: u32 = 1;
pub const SIG_SETMASK: u32 = 2;
pub const _BITS_SIGCONTEXT_H: u32 = 1;
pub const FP_XSTATE_MAGIC1: u32 = 1179670611;
pub const FP_XSTATE_MAGIC2: u32 = 1179670597;
pub const __stack_t_defined: u32 = 1;
pub const _SYS_UCONTEXT_H: u32 = 1;
pub const __NGREG: u32 = 23;
pub const NGREG: u32 = 23;
pub const _BITS_SIGSTACK_H: u32 = 1;
pub const MINSIGSTKSZ: u32 = 2048;
pub const SIGSTKSZ: u32 = 8192;
pub const _BITS_SS_FLAGS_H: u32 = 1;
pub const __sigstack_defined: u32 = 1;
pub const _BITS_PTHREADTYPES_COMMON_H: u32 = 1;
pub const _THREAD_SHARED_TYPES_H: u32 = 1;
pub const _BITS_PTHREADTYPES_ARCH_H: u32 = 1;
pub const __SIZEOF_PTHREAD_MUTEX_T: u32 = 40;
pub const __SIZEOF_PTHREAD_ATTR_T: u32 = 56;
pub const __SIZEOF_PTHREAD_RWLOCK_T: u32 = 56;
pub const __SIZEOF_PTHREAD_BARRIER_T: u32 = 32;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: u32 = 4;
pub const __SIZEOF_PTHREAD_COND_T: u32 = 48;
pub const __SIZEOF_PTHREAD_CONDATTR_T: u32 = 4;
pub const __SIZEOF_PTHREAD_RWLOCKATTR_T: u32 = 8;
pub const __SIZEOF_PTHREAD_BARRIERATTR_T: u32 = 4;
pub const __PTHREAD_MUTEX_LOCK_ELISION: u32 = 1;
pub const __PTHREAD_MUTEX_NUSERS_AFTER_KIND: u32 = 0;
pub const __PTHREAD_MUTEX_USE_UNION: u32 = 0;
pub const __PTHREAD_RWLOCK_INT_FLAGS_SHARED: u32 = 1;
pub const __PTHREAD_MUTEX_HAVE_PREV: u32 = 1;
pub const _BITS_SIGTHREAD_H: u32 = 1;
pub const _STDINT_H: u32 = 1;
pub const _BITS_STDINT_INTN_H: u32 = 1;
pub const _BITS_STDINT_UINTN_H: u32 = 1;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i64 = -9223372036854775808;
pub const INT_FAST32_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u64 = 9223372036854775807;
pub const INT_FAST32_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: i32 = -1;
pub const UINT_FAST32_MAX: i32 = -1;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const UINTPTR_MAX: i32 = -1;
pub const PTRDIFF_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u64 = 9223372036854775807;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: i32 = -1;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub const INT8_WIDTH: u32 = 8;
pub const UINT8_WIDTH: u32 = 8;
pub const INT16_WIDTH: u32 = 16;
pub const UINT16_WIDTH: u32 = 16;
pub const INT32_WIDTH: u32 = 32;
pub const UINT32_WIDTH: u32 = 32;
pub const INT64_WIDTH: u32 = 64;
pub const UINT64_WIDTH: u32 = 64;
pub const INT_LEAST8_WIDTH: u32 = 8;
pub const UINT_LEAST8_WIDTH: u32 = 8;
pub const INT_LEAST16_WIDTH: u32 = 16;
pub const UINT_LEAST16_WIDTH: u32 = 16;
pub const INT_LEAST32_WIDTH: u32 = 32;
pub const UINT_LEAST32_WIDTH: u32 = 32;
pub const INT_LEAST64_WIDTH: u32 = 64;
pub const UINT_LEAST64_WIDTH: u32 = 64;
pub const INT_FAST8_WIDTH: u32 = 8;
pub const UINT_FAST8_WIDTH: u32 = 8;
pub const INT_FAST16_WIDTH: u32 = 64;
pub const UINT_FAST16_WIDTH: u32 = 64;
pub const INT_FAST32_WIDTH: u32 = 64;
pub const UINT_FAST32_WIDTH: u32 = 64;
pub const INT_FAST64_WIDTH: u32 = 64;
pub const UINT_FAST64_WIDTH: u32 = 64;
pub const INTPTR_WIDTH: u32 = 64;
pub const UINTPTR_WIDTH: u32 = 64;
pub const INTMAX_WIDTH: u32 = 64;
pub const UINTMAX_WIDTH: u32 = 64;
pub const PTRDIFF_WIDTH: u32 = 64;
pub const SIG_ATOMIC_WIDTH: u32 = 32;
pub const SIZE_WIDTH: u32 = 64;
pub const WCHAR_WIDTH: u32 = 32;
pub const WINT_WIDTH: u32 = 32;

pub const W_SPLITSPACE: u32 = 8;
pub const W_NOSPLIT: u32 = 16;
pub const W_NOGLOB: u32 = 32;
pub const W_NOSPLIT2: u32 = 64;
pub const W_TILDEEXP: u32 = 128;
pub const W_DOLLARAT: u32 = 256;
pub const W_DOLLARSTAR: u32 = 512;
pub const W_NOCOMSUB: u32 = 1024;
pub const W_ASSIGNRHS: u32 = 2048;
pub const W_NOTILDE: u32 = 4096;
pub const W_ITILDE: u32 = 8192;
pub const W_EXPANDRHS: u32 = 16384;
pub const W_COMPASSIGN: u32 = 32768;
pub const W_ASSNBLTIN: u32 = 65536;
pub const W_ASSIGNARG: u32 = 131072;
pub const W_HASQUOTEDNULL: u32 = 262144;
pub const W_DQUOTE: u32 = 524288;
pub const W_NOPROCSUB: u32 = 1048576;
pub const W_SAWQUOTEDNULL: u32 = 2097152;
pub const W_ASSIGNASSOC: u32 = 4194304;
pub const W_ASSIGNARRAY: u32 = 8388608;
pub const W_ARRAYIND: u32 = 16777216;
pub const W_ASSNGLOBAL: u32 = 33554432;
pub const W_NOBRACE: u32 = 67108864;
pub const W_COMPLETE: u32 = 134217728;
pub const W_CHKLOCAL: u32 = 268435456;
pub const W_NOASSNTILDE: u32 = 536870912;
pub const W_FORCELOCAL: u32 = 1073741824;
pub const PF_NOCOMSUB: u32 = 1;
pub const PF_IGNUNBOUND: u32 = 2;
pub const PF_NOSPLIT2: u32 = 4;
pub const PF_ASSIGNRHS: u32 = 8;
pub const PF_COMPLETE: u32 = 16;
pub const PF_EXPANDRHS: u32 = 32;
pub const PF_ALLINDS: u32 = 64;

pub const SUBSHELL_PAREN: u32 = 2;
pub const SUBSHELL_COMSUB: u32 = 4;
pub const SUBSHELL_FORK: u32 = 8;
pub const SUBSHELL_PIPE: u32 = 16;
pub const SUBSHELL_PROCSUB: u32 = 32;
pub const SUBSHELL_COPROC: u32 = 64;
pub const SUBSHELL_RESETTRAP: u32 = 128;

pub const CASEPAT_FALLTHROUGH: u32 = 1;
pub const CASEPAT_TESTNEXT: u32 = 2;
pub const COND_AND: u32 = 1;
pub const COND_OR: u32 = 2;
pub const COND_UNARY: u32 = 3;
pub const COND_BINARY: u32 = 4;
pub const COND_TERM: u32 = 5;
pub const COND_EXPR: u32 = 6;
pub const COPROC_RUNNING: u32 = 1;
pub const COPROC_DEAD: u32 = 2;
pub const CMDERR_DEFAULT: u32 = 0;
pub const CMDERR_BADTYPE: u32 = 1;
pub const CMDERR_BADCONN: u32 = 2;
pub const CMDERR_BADJUMP: u32 = 3;
pub const CMDERR_LAST: u32 = 3;
pub const _SYS_TYPES_H: u32 = 1;
pub const __clock_t_defined: u32 = 1;
pub const __clockid_t_defined: u32 = 1;
pub const __time_t_defined: u32 = 1;
pub const __timer_t_defined: u32 = 1;
pub const __BIT_TYPES_DEFINED__: u32 = 1;
pub const _SYS_SELECT_H: u32 = 1;
pub const __FD_ZERO_STOS: &'static [u8; 6usize] = b"stosq\0";
pub const __timeval_defined: u32 = 1;
pub const FD_SETSIZE: u32 = 1024;
pub const _INTTYPES_H: u32 = 1;
pub const ____gwchar_t_defined: u32 = 1;
pub const __PRI64_PREFIX: &'static [u8; 2usize] = b"l\0";
pub const __PRIPTR_PREFIX: &'static [u8; 2usize] = b"l\0";
pub const PRId8: &'static [u8; 2usize] = b"d\0";
pub const PRId16: &'static [u8; 2usize] = b"d\0";
pub const PRId32: &'static [u8; 2usize] = b"d\0";
pub const PRId64: &'static [u8; 3usize] = b"ld\0";
pub const PRIdLEAST8: &'static [u8; 2usize] = b"d\0";
pub const PRIdLEAST16: &'static [u8; 2usize] = b"d\0";
pub const PRIdLEAST32: &'static [u8; 2usize] = b"d\0";
pub const PRIdLEAST64: &'static [u8; 3usize] = b"ld\0";
pub const PRIdFAST8: &'static [u8; 2usize] = b"d\0";
pub const PRIdFAST16: &'static [u8; 3usize] = b"ld\0";
pub const PRIdFAST32: &'static [u8; 3usize] = b"ld\0";
pub const PRIdFAST64: &'static [u8; 3usize] = b"ld\0";
pub const PRIi8: &'static [u8; 2usize] = b"i\0";
pub const PRIi16: &'static [u8; 2usize] = b"i\0";
pub const PRIi32: &'static [u8; 2usize] = b"i\0";
pub const PRIi64: &'static [u8; 3usize] = b"li\0";
pub const PRIiLEAST8: &'static [u8; 2usize] = b"i\0";
pub const PRIiLEAST16: &'static [u8; 2usize] = b"i\0";
pub const PRIiLEAST32: &'static [u8; 2usize] = b"i\0";
pub const PRIiLEAST64: &'static [u8; 3usize] = b"li\0";
pub const PRIiFAST8: &'static [u8; 2usize] = b"i\0";
pub const PRIiFAST16: &'static [u8; 3usize] = b"li\0";
pub const PRIiFAST32: &'static [u8; 3usize] = b"li\0";
pub const PRIiFAST64: &'static [u8; 3usize] = b"li\0";
pub const PRIo8: &'static [u8; 2usize] = b"o\0";
pub const PRIo16: &'static [u8; 2usize] = b"o\0";
pub const PRIo32: &'static [u8; 2usize] = b"o\0";
pub const PRIo64: &'static [u8; 3usize] = b"lo\0";
pub const PRIoLEAST8: &'static [u8; 2usize] = b"o\0";
pub const PRIoLEAST16: &'static [u8; 2usize] = b"o\0";
pub const PRIoLEAST32: &'static [u8; 2usize] = b"o\0";
pub const PRIoLEAST64: &'static [u8; 3usize] = b"lo\0";
pub const PRIoFAST8: &'static [u8; 2usize] = b"o\0";
pub const PRIoFAST16: &'static [u8; 3usize] = b"lo\0";
pub const PRIoFAST32: &'static [u8; 3usize] = b"lo\0";
pub const PRIoFAST64: &'static [u8; 3usize] = b"lo\0";
pub const PRIu8: &'static [u8; 2usize] = b"u\0";
pub const PRIu16: &'static [u8; 2usize] = b"u\0";
pub const PRIu32: &'static [u8; 2usize] = b"u\0";
pub const PRIu64: &'static [u8; 3usize] = b"lu\0";
pub const PRIuLEAST8: &'static [u8; 2usize] = b"u\0";
pub const PRIuLEAST16: &'static [u8; 2usize] = b"u\0";
pub const PRIuLEAST32: &'static [u8; 2usize] = b"u\0";
pub const PRIuLEAST64: &'static [u8; 3usize] = b"lu\0";
pub const PRIuFAST8: &'static [u8; 2usize] = b"u\0";
pub const PRIuFAST16: &'static [u8; 3usize] = b"lu\0";
pub const PRIuFAST32: &'static [u8; 3usize] = b"lu\0";
pub const PRIuFAST64: &'static [u8; 3usize] = b"lu\0";
pub const PRIx8: &'static [u8; 2usize] = b"x\0";
pub const PRIx16: &'static [u8; 2usize] = b"x\0";
pub const PRIx32: &'static [u8; 2usize] = b"x\0";
pub const PRIx64: &'static [u8; 3usize] = b"lx\0";
pub const PRIxLEAST8: &'static [u8; 2usize] = b"x\0";
pub const PRIxLEAST16: &'static [u8; 2usize] = b"x\0";
pub const PRIxLEAST32: &'static [u8; 2usize] = b"x\0";
pub const PRIxLEAST64: &'static [u8; 3usize] = b"lx\0";
pub const PRIxFAST8: &'static [u8; 2usize] = b"x\0";
pub const PRIxFAST16: &'static [u8; 3usize] = b"lx\0";
pub const PRIxFAST32: &'static [u8; 3usize] = b"lx\0";
pub const PRIxFAST64: &'static [u8; 3usize] = b"lx\0";
pub const PRIX8: &'static [u8; 2usize] = b"X\0";
pub const PRIX16: &'static [u8; 2usize] = b"X\0";
pub const PRIX32: &'static [u8; 2usize] = b"X\0";
pub const PRIX64: &'static [u8; 3usize] = b"lX\0";
pub const PRIXLEAST8: &'static [u8; 2usize] = b"X\0";
pub const PRIXLEAST16: &'static [u8; 2usize] = b"X\0";
pub const PRIXLEAST32: &'static [u8; 2usize] = b"X\0";
pub const PRIXLEAST64: &'static [u8; 3usize] = b"lX\0";
pub const PRIXFAST8: &'static [u8; 2usize] = b"X\0";
pub const PRIXFAST16: &'static [u8; 3usize] = b"lX\0";
pub const PRIXFAST32: &'static [u8; 3usize] = b"lX\0";
pub const PRIXFAST64: &'static [u8; 3usize] = b"lX\0";
pub const PRIdMAX: &'static [u8; 3usize] = b"ld\0";
pub const PRIiMAX: &'static [u8; 3usize] = b"li\0";
pub const PRIoMAX: &'static [u8; 3usize] = b"lo\0";
pub const PRIuMAX: &'static [u8; 3usize] = b"lu\0";
pub const PRIxMAX: &'static [u8; 3usize] = b"lx\0";
pub const PRIXMAX: &'static [u8; 3usize] = b"lX\0";
pub const PRIdPTR: &'static [u8; 3usize] = b"ld\0";
pub const PRIiPTR: &'static [u8; 3usize] = b"li\0";
pub const PRIoPTR: &'static [u8; 3usize] = b"lo\0";
pub const PRIuPTR: &'static [u8; 3usize] = b"lu\0";
pub const PRIxPTR: &'static [u8; 3usize] = b"lx\0";
pub const PRIXPTR: &'static [u8; 3usize] = b"lX\0";
pub const SCNd8: &'static [u8; 4usize] = b"hhd\0";
pub const SCNd16: &'static [u8; 3usize] = b"hd\0";
pub const SCNd32: &'static [u8; 2usize] = b"d\0";
pub const SCNd64: &'static [u8; 3usize] = b"ld\0";
pub const SCNdLEAST8: &'static [u8; 4usize] = b"hhd\0";
pub const SCNdLEAST16: &'static [u8; 3usize] = b"hd\0";
pub const SCNdLEAST32: &'static [u8; 2usize] = b"d\0";
pub const SCNdLEAST64: &'static [u8; 3usize] = b"ld\0";
pub const SCNdFAST8: &'static [u8; 4usize] = b"hhd\0";
pub const SCNdFAST16: &'static [u8; 3usize] = b"ld\0";
pub const SCNdFAST32: &'static [u8; 3usize] = b"ld\0";
pub const SCNdFAST64: &'static [u8; 3usize] = b"ld\0";
pub const SCNi8: &'static [u8; 4usize] = b"hhi\0";
pub const SCNi16: &'static [u8; 3usize] = b"hi\0";
pub const SCNi32: &'static [u8; 2usize] = b"i\0";
pub const SCNi64: &'static [u8; 3usize] = b"li\0";
pub const SCNiLEAST8: &'static [u8; 4usize] = b"hhi\0";
pub const SCNiLEAST16: &'static [u8; 3usize] = b"hi\0";
pub const SCNiLEAST32: &'static [u8; 2usize] = b"i\0";
pub const SCNiLEAST64: &'static [u8; 3usize] = b"li\0";
pub const SCNiFAST8: &'static [u8; 4usize] = b"hhi\0";
pub const SCNiFAST16: &'static [u8; 3usize] = b"li\0";
pub const SCNiFAST32: &'static [u8; 3usize] = b"li\0";
pub const SCNiFAST64: &'static [u8; 3usize] = b"li\0";
pub const SCNu8: &'static [u8; 4usize] = b"hhu\0";
pub const SCNu16: &'static [u8; 3usize] = b"hu\0";
pub const SCNu32: &'static [u8; 2usize] = b"u\0";
pub const SCNu64: &'static [u8; 3usize] = b"lu\0";
pub const SCNuLEAST8: &'static [u8; 4usize] = b"hhu\0";
pub const SCNuLEAST16: &'static [u8; 3usize] = b"hu\0";
pub const SCNuLEAST32: &'static [u8; 2usize] = b"u\0";
pub const SCNuLEAST64: &'static [u8; 3usize] = b"lu\0";
pub const SCNuFAST8: &'static [u8; 4usize] = b"hhu\0";
pub const SCNuFAST16: &'static [u8; 3usize] = b"lu\0";
pub const SCNuFAST32: &'static [u8; 3usize] = b"lu\0";
pub const SCNuFAST64: &'static [u8; 3usize] = b"lu\0";
pub const SCNo8: &'static [u8; 4usize] = b"hho\0";
pub const SCNo16: &'static [u8; 3usize] = b"ho\0";
pub const SCNo32: &'static [u8; 2usize] = b"o\0";
pub const SCNo64: &'static [u8; 3usize] = b"lo\0";
pub const SCNoLEAST8: &'static [u8; 4usize] = b"hho\0";
pub const SCNoLEAST16: &'static [u8; 3usize] = b"ho\0";
pub const SCNoLEAST32: &'static [u8; 2usize] = b"o\0";
pub const SCNoLEAST64: &'static [u8; 3usize] = b"lo\0";
pub const SCNoFAST8: &'static [u8; 4usize] = b"hho\0";
pub const SCNoFAST16: &'static [u8; 3usize] = b"lo\0";
pub const SCNoFAST32: &'static [u8; 3usize] = b"lo\0";
pub const SCNoFAST64: &'static [u8; 3usize] = b"lo\0";
pub const SCNx8: &'static [u8; 4usize] = b"hhx\0";
pub const SCNx16: &'static [u8; 3usize] = b"hx\0";
pub const SCNx32: &'static [u8; 2usize] = b"x\0";
pub const SCNx64: &'static [u8; 3usize] = b"lx\0";
pub const SCNxLEAST8: &'static [u8; 4usize] = b"hhx\0";
pub const SCNxLEAST16: &'static [u8; 3usize] = b"hx\0";
pub const SCNxLEAST32: &'static [u8; 2usize] = b"x\0";
pub const SCNxLEAST64: &'static [u8; 3usize] = b"lx\0";
pub const SCNxFAST8: &'static [u8; 4usize] = b"hhx\0";
pub const SCNxFAST16: &'static [u8; 3usize] = b"lx\0";
pub const SCNxFAST32: &'static [u8; 3usize] = b"lx\0";
pub const SCNxFAST64: &'static [u8; 3usize] = b"lx\0";
pub const SCNdMAX: &'static [u8; 3usize] = b"ld\0";
pub const SCNiMAX: &'static [u8; 3usize] = b"li\0";
pub const SCNoMAX: &'static [u8; 3usize] = b"lo\0";
pub const SCNuMAX: &'static [u8; 3usize] = b"lu\0";
pub const SCNxMAX: &'static [u8; 3usize] = b"lx\0";
pub const SCNdPTR: &'static [u8; 3usize] = b"ld\0";
pub const SCNiPTR: &'static [u8; 3usize] = b"li\0";
pub const SCNoPTR: &'static [u8; 3usize] = b"lo\0";
pub const SCNuPTR: &'static [u8; 3usize] = b"lu\0";
pub const SCNxPTR: &'static [u8; 3usize] = b"lx\0";
pub const _CTYPE_H: u32 = 1;
pub const _SYS_TIME_H: u32 = 1;
pub const _SYS_RESOURCE_H: u32 = 1;
pub const RLIM64_INFINITY: i32 = -1;
pub const __rusage_defined: u32 = 1;
pub const PRIO_MIN: i32 = -20;
pub const PRIO_MAX: u32 = 20;
pub const _STRING_H: u32 = 1;
pub const _STRINGS_H: u32 = 1;
pub const _STDLIB_H: u32 = 1;
pub const WNOHANG: u32 = 1;
pub const WUNTRACED: u32 = 2;
pub const WSTOPPED: u32 = 2;
pub const WEXITED: u32 = 4;
pub const WCONTINUED: u32 = 8;
pub const WNOWAIT: u32 = 16777216;
pub const __WNOTHREAD: u32 = 536870912;
pub const __WALL: u32 = 1073741824;
pub const __WCLONE: u32 = 2147483648;
pub const __ENUM_IDTYPE_T: u32 = 1;
pub const __W_CONTINUED: u32 = 65535;
pub const __WCOREFLAG: u32 = 128;
pub const __ldiv_t_defined: u32 = 1;
pub const __lldiv_t_defined: u32 = 1;
pub const RAND_MAX: u32 = 2147483647;
pub const EXIT_FAILURE: u32 = 1;
pub const EXIT_SUCCESS: u32 = 0;
pub const _ALLOCA_H: u32 = 1;
pub const FS_EXISTS: u32 = 1;
pub const FS_EXECABLE: u32 = 2;
pub const FS_EXEC_PREFERRED: u32 = 4;
pub const FS_EXEC_ONLY: u32 = 8;
pub const FS_DIRECTORY: u32 = 16;
pub const FS_NODIRS: u32 = 32;
pub const FS_READABLE: u32 = 64;

pub const DIRSEP: u8 = 47u8;
pub const AS_DISPOSE: u32 = 1;
pub const NO_SIG: i32 = -1;

pub const DSIG_SIGPREFIX: i32 = 1;
pub const DSIG_NOCASE: i32 = 2;
pub const _LIBGETTEXT_H: u32 = 1;
pub const _LIBINTL_H: u32 = 1;
pub const __USE_GNU_GETTEXT: u32 = 1;
pub const _LOCALE_H: u32 = 1;
pub const _BITS_LOCALE_H: u32 = 1;
pub const __LC_CTYPE: u32 = 0;
pub const __LC_NUMERIC: u32 = 1;
pub const __LC_TIME: u32 = 2;
pub const __LC_COLLATE: u32 = 3;
pub const __LC_MONETARY: u32 = 4;
pub const __LC_MESSAGES: u32 = 5;
pub const __LC_ALL: u32 = 6;
pub const __LC_PAPER: u32 = 7;
pub const __LC_NAME: u32 = 8;
pub const __LC_ADDRESS: u32 = 9;
pub const __LC_TELEPHONE: u32 = 10;
pub const __LC_MEASUREMENT: u32 = 11;
pub const __LC_IDENTIFICATION: u32 = 12;
pub const LC_CTYPE: u32 = 0;
pub const LC_NUMERIC: u32 = 1;
pub const LC_TIME: u32 = 2;
pub const LC_COLLATE: u32 = 3;
pub const LC_MONETARY: u32 = 4;
pub const LC_MESSAGES: u32 = 5;

pub const LC_PAPER: u32 = 7;
pub const LC_NAME: u32 = 8;
pub const LC_ADDRESS: u32 = 9;
pub const LC_TELEPHONE: u32 = 10;
pub const LC_MEASUREMENT: u32 = 11;
pub const LC_IDENTIFICATION: u32 = 12;
pub const LC_CTYPE_MASK: u32 = 1;
pub const LC_NUMERIC_MASK: u32 = 2;
pub const LC_TIME_MASK: u32 = 4;
pub const LC_COLLATE_MASK: u32 = 8;
pub const LC_MONETARY_MASK: u32 = 16;
pub const LC_MESSAGES_MASK: u32 = 32;
pub const LC_PAPER_MASK: u32 = 128;
pub const LC_NAME_MASK: u32 = 256;
pub const LC_ADDRESS_MASK: u32 = 512;
pub const LC_TELEPHONE_MASK: u32 = 1024;
pub const LC_MEASUREMENT_MASK: u32 = 2048;
pub const LC_IDENTIFICATION_MASK: u32 = 4096;
pub const LC_ALL_MASK: u32 = 8127;

pub const CASE_LOWER: u32 = 1;
pub const CASE_UPPER: u32 = 2;
pub const CASE_CAPITALIZE: u32 = 4;
pub const CASE_UNCAP: u32 = 8;
pub const CASE_TOGGLE: u32 = 16;
pub const CASE_TOGGLEALL: u32 = 32;
pub const CASE_UPFIRST: u32 = 64;
pub const CASE_LOWFIRST: u32 = 128;
pub const CASE_USEWORDS: u32 = 4096;
pub const FL_PREFIX: u32 = 1;
pub const FL_ADDBASE: u32 = 2;
pub const FL_HEXUPPER: u32 = 4;
pub const FL_UNSIGNED: u32 = 8;
pub const MP_DOTILDE: u32 = 1;

pub const MP_IGNDOT: u32 = 8;
pub const PATH_CHECKDOTDOT: u32 = 1;
pub const PATH_CHECKEXISTS: u32 = 2;
pub const PATH_HARDPATH: u32 = 4;
pub const PATH_NOALLOC: u32 = 8;
pub const SHMAT_SUBEXP: u32 = 1;
pub const SHMAT_PWARN: u32 = 2;
pub const MT_USETMPDIR: u32 = 1;
pub const MT_READWRITE: u32 = 2;
pub const MT_USERANDOM: u32 = 4;
pub const MT_TEMPLATE: u32 = 8;
pub const DEFAULT_HASH_BUCKETS: u32 = 128;

pub const HASH_CREATE: u32 = 2;
pub const ASSOC_HASH_BUCKETS: u32 = 1024;
pub const VC_HASLOCAL: u32 = 1;
pub const VC_HASTMPVAR: u32 = 2;
pub const VC_FUNCENV: u32 = 4;
pub const VC_BLTNENV: u32 = 8;
pub const VC_TEMPENV: u32 = 16;
pub const VC_TEMPFLAGS: u32 = 28;

pub const user_attrs: u32 = 4019;
pub const attmask_user: u32 = 4095;

pub const att_nounset: u32 = 8192;

pub const att_special: u32 = 65536;
pub const att_nofree: u32 = 131072;
pub const att_regenerate: u32 = 262144;
pub const attmask_int: u32 = 1044480;

pub const attmask_scope: u32 = 15728640;
pub const NAMEREF_MAX: u32 = 8;
pub const MKLOC_ASSOCOK: u32 = 1;
pub const MKLOC_ARRAYOK: u32 = 2;
pub const MKLOC_INHERIT: u32 = 4;
pub const AV_ALLOWALL: u32 = 1;
pub const AV_QUOTED: u32 = 2;
pub const AV_USEIND: u32 = 4;
pub const AV_USEVAL: u32 = 8;
pub const AV_ASSIGNRHS: u32 = 16;

pub const VA_NOEXPAND: i32 = 1;
pub const VA_ONEWORD: i32 = 2;
pub const RX_ACTIVE: u32 = 1;
pub const RX_UNDOABLE: u32 = 2;

pub const RX_USER: u32 = 16;

pub const HC_IGNSPACE: u32 = 1;
pub const HC_IGNDUPS: u32 = 2;
pub const HC_ERASEDUPS: u32 = 4;
pub const HC_IGNBOTH: u32 = 3;
pub const CA_ALIAS: u32 = 1;
pub const CA_ARRAYVAR: u32 = 2;
pub const CA_BINDING: u32 = 4;
pub const CA_BUILTIN: u32 = 8;
pub const CA_COMMAND: u32 = 16;
pub const CA_DIRECTORY: u32 = 32;
pub const CA_DISABLED: u32 = 64;
pub const CA_ENABLED: u32 = 128;
pub const CA_EXPORT: u32 = 256;
pub const CA_FILE: u32 = 512;
pub const CA_FUNCTION: u32 = 1024;
pub const CA_GROUP: u32 = 2048;
pub const CA_HELPTOPIC: u32 = 4096;
pub const CA_HOSTNAME: u32 = 8192;
pub const CA_JOB: u32 = 16384;
pub const CA_KEYWORD: u32 = 32768;
pub const CA_RUNNING: u32 = 65536;
pub const CA_SERVICE: u32 = 131072;
pub const CA_SETOPT: u32 = 262144;
pub const CA_SHOPT: u32 = 524288;
pub const CA_SIGNAL: u32 = 1048576;
pub const CA_STOPPED: u32 = 2097152;
pub const CA_USER: u32 = 4194304;
pub const CA_VARIABLE: u32 = 8388608;
pub const COPT_RESERVED: u32 = 1;
pub const COPT_DEFAULT: u32 = 2;
pub const COPT_FILENAMES: u32 = 4;
pub const COPT_DIRNAMES: u32 = 8;
pub const COPT_NOQUOTE: u32 = 16;
pub const COPT_NOSPACE: u32 = 32;
pub const COPT_BASHDEFAULT: u32 = 64;
pub const COPT_PLUSDIRS: u32 = 128;
pub const COPT_NOSORT: u32 = 256;
pub const COPT_LASTUSER: u32 = 256;
pub const PCOMP_RETRYFAIL: u32 = 512;
pub const PCOMP_NOTFOUND: u32 = 1024;
pub const LIST_DYNAMIC: u32 = 1;
pub const LIST_DIRTY: u32 = 2;
pub const LIST_INITIALIZED: u32 = 4;
pub const LIST_MUSTSORT: u32 = 8;
pub const LIST_DONTFREE: u32 = 16;
pub const LIST_DONTFREEMEMBERS: u32 = 32;
pub const EMPTYCMD: &'static [u8; 11usize] = b"_EmptycmD_\0";
pub const DEFAULTCMD: &'static [u8; 13usize] = b"_DefaultCmD_\0";
pub const INITIALWORD: &'static [u8; 14usize] = b"_InitialWorD_\0";
pub const _STDIO_H: u32 = 1;
pub const _____fpos_t_defined: u32 = 1;
pub const _____fpos64_t_defined: u32 = 1;
pub const __struct_FILE_defined: u32 = 1;
pub const _IO_EOF_SEEN: u32 = 16;
pub const _IO_ERR_SEEN: u32 = 32;
pub const _IO_USER_LOCK: u32 = 32768;
pub const __cookie_io_functions_t_defined: u32 = 1;
pub const _IOFBF: u32 = 0;
pub const _IOLBF: u32 = 1;
pub const _IONBF: u32 = 2;
pub const BUFSIZ: u32 = 8192;
pub const EOF: i32 = -1;
pub const P_tmpdir: &'static [u8; 5usize] = b"/tmp\0";
pub const _BITS_STDIO_LIM_H: u32 = 1;
pub const L_tmpnam: u32 = 20;
pub const TMP_MAX: u32 = 238328;
pub const FILENAME_MAX: u32 = 4096;
pub const L_ctermid: u32 = 9;
pub const L_cuserid: u32 = 9;
pub const FOPEN_MAX: u32 = 16;
pub const RENAME_NOREPLACE: u32 = 1;
pub const RENAME_EXCHANGE: u32 = 2;
pub const RENAME_WHITEOUT: u32 = 4;
pub const DEFAULT_HOSTS_FILE: &'static [u8; 11usize] = b"/etc/hosts\0";
pub const SYS_PROFILE: &'static [u8; 13usize] = b"/etc/profile\0";
pub const DEBUGGER_START_FILE: &'static [u8; 46usize] =
    b"/usr/local/share/utshelldb/utshelldb-main.inc\0";
pub const FILENAME_HASH_BUCKETS: u32 = 256;
pub const HASH_RELPATH: u32 = 1;
pub const HASH_CHKDOT: u32 = 2;

pub const QGLOB_CVTNULL: u32 = 1;
pub const QGLOB_FILENAME: u32 = 2;
pub const QGLOB_REGEXP: u32 = 4;
pub const QGLOB_CTLESC: u32 = 8;
pub const QGLOB_DEQUOTE: u32 = 16;

pub const PATCHLEVEL: u32 = 4;
pub const Q_DOUBLE_QUOTES: u32 = 1;
pub const Q_HERE_DOCUMENT: u32 = 2;
pub const Q_KEEP_BACKSLASH: u32 = 4;
pub const Q_PATQUOTE: u32 = 8;
pub const Q_QUOTED: u32 = 16;
pub const Q_ADDEDQUOTES: u32 = 32;
pub const Q_QUOTEDNULL: u32 = 64;
pub const Q_DOLBRACE: u32 = 128;
pub const Q_ARITH: u32 = 256;
pub const Q_ARRAYSUB: u32 = 512;
pub const ASS_APPEND: i32 = 1;
pub const ASS_MKLOCAL: u32 = 2;
pub const ASS_MKASSOC: u32 = 4;
pub const ASS_MKGLOBAL: u32 = 8;
pub const ASS_NAMEREF: u32 = 16;
pub const ASS_FORCE: u32 = 32;
pub const ASS_CHKLOCAL: u32 = 64;

pub const ASS_NOEVAL: u32 = 256;
pub const ASS_NOLONGJMP: u32 = 512;
pub const ASS_NOINVIS: u32 = 1024;
pub const SX_NOALLOC: u32 = 1;
pub const SX_VARNAME: u32 = 2;
pub const SX_REQMATCH: u32 = 4;
pub const SX_COMMAND: u32 = 8;
pub const SX_NOCTLESC: u32 = 16;
pub const SX_NOESCCTLNUL: u32 = 32;
pub const SX_NOLONGJMP: u32 = 64;
pub const SX_ARITHSUB: u32 = 128;
pub const SX_POSIXEXP: u32 = 256;
pub const SX_WORD: u32 = 512;
pub const SX_COMPLETE: u32 = 1024;
pub const SX_STRIPDQ: u32 = 2048;
pub const SD_NOJMP: u32 = 1;
pub const SD_INVERT: u32 = 2;
pub const SD_NOQUOTEDELIM: u32 = 4;
pub const SD_NOSKIPCMD: u32 = 8;
pub const SD_EXTGLOB: u32 = 16;
pub const SD_IGNOREQUOTE: u32 = 32;
pub const SD_GLOB: u32 = 64;
pub const SD_NOPROCSUB: u32 = 128;
pub const SD_COMPLETE: u32 = 256;
pub const SD_HISTEXP: u32 = 512;
pub const SD_ARITHEXP: u32 = 1024;
pub const _SYS_WAIT_H: u32 = 1;
pub const WCOREFLAG: u32 = 128;
pub const WAIT_ANY: i32 = -1;
pub const WAIT_MYPGRP: u32 = 0;
pub const JLIST_STANDARD: u32 = 0;
pub const JLIST_LONG: u32 = 1;
pub const JLIST_PID_ONLY: u32 = 2;
pub const JLIST_CHANGED_ONLY: u32 = 3;
pub const JLIST_NONINTERACTIVE: u32 = 4;
pub const LONGEST_SIGNAL_DESC: u32 = 24;
pub const JWAIT_PERROR: u32 = 1;
pub const JWAIT_FORCE: u32 = 2;
pub const JWAIT_NOWAIT: u32 = 4;
pub const JWAIT_WAITING: u32 = 8;
pub const JWAIT_NOTERM: u32 = 256;
pub const FORKSLEEP_MAX: u32 = 16;
pub const PS_DONE: u32 = 0;
pub const PS_RUNNING: u32 = 1;
pub const PS_STOPPED: u32 = 2;
pub const PS_RECYCLED: u32 = 4;
pub const J_FOREGROUND: u32 = 1;
pub const J_NOTIFIED: u32 = 2;
pub const J_JOBCONTROL: i32 = 4;
pub const J_NOHUP: u32 = 8;
pub const J_STATSAVED: u32 = 16;
pub const J_ASYNC: u32 = 32;
pub const J_PIPEFAIL: u32 = 64;
pub const J_WAITING: u32 = 128;
pub const NO_JOB: i32 = -1;
pub const DUP_JOB: i32 = -2;
pub const BAD_JOBSPEC: i32 = -3;
pub const FORK_SYNC: u32 = 0;
pub const FORK_ASYNC: u32 = 1;
pub const FORK_NOJOB: u32 = 2;
pub const FORK_NOTERM: u32 = 4;
pub const CMDSRCH_HASH: u32 = 1;
pub const CMDSRCH_STDPATH: u32 = 2;
pub const CMDSRCH_TEMPENV: u32 = 4;
pub const _SETJMP_H: u32 = 1;
pub const _BITS_SETJMP_H: u32 = 1;
pub const NOT_JUMPED: u32 = 0;

pub const SIGEXIT: u32 = 5;
pub const slashify_in_here_document: &'static [u8; 4usize] = b"\\`$\0";
pub const shell_meta_chars: &'static [u8; 8usize] = b"()<>;&|\0";
pub const shell_break_chars: &'static [u8; 13usize] = b"()<>;&| \\t\\n\0";
pub const shell_quote_chars: &'static [u8; 4usize] = b"\"`'\0";
pub const shell_exp_chars: &'static [u8; 4usize] = b"$<>\0";
pub const ext_glob_chars: &'static [u8; 6usize] = b"@*+?!\0";
pub const shell_glob_chars: &'static [u8; 6usize] = b"*?[]^\0";
pub const CWORD: u32 = 0;
pub const CSHMETA: u32 = 1;

pub const CBACKQ: u32 = 4;
pub const CQUOTE: u32 = 8;
pub const CSPECL: u32 = 16;
pub const CEXP: u32 = 32;
pub const CBSDQUOTE: u32 = 64;
pub const CBSHDOC: u32 = 128;
pub const CGLOB: u32 = 256;
pub const CXGLOB: u32 = 512;

pub const CSPECVAR: u32 = 2048;
pub const CSUBSTOP: u32 = 4096;

pub const CTLESC: u8 = 1u8;
pub const CTLNUL: u8 = 127u8;
pub const _OCACHE_H_: u32 = 1;
pub const NO_PIPE: i32 = -1;
pub const REDIRECT_BOTH: i32 = -2;
pub const NO_VARIABLE: i32 = -1;

pub const EXECUTION_SUCCESS: i32 = 0;
pub const EX_BADUSAGE: i32 = 2;
pub const EX_MISCERROR: u32 = 2;
pub const EX_RETRYFAIL: u32 = 124;
pub const EX_WEXPCOMSUB: u32 = 125;
pub const EX_BINARY_FILE: u32 = 126;

pub const EX_NOINPUT: u32 = 126;
pub const EX_NOTFOUND: i32 = 127;
pub const EX_SHERRBASE: u32 = 256;
pub const EX_BADSYNTAX: u32 = 257;
pub const EX_USAGE: i32 = 258;
pub const EX_REDIRFAIL: u32 = 259;
pub const EX_BADASSIGN: i32 = 260;
pub const EX_EXPFAIL: u32 = 261;
pub const EX_DISKFALLBACK: u32 = 262;
pub const MATCH_ANY: u32 = 0;
pub const MATCH_BEG: u32 = 1;
pub const MATCH_END: u32 = 2;
pub const MATCH_TYPEMASK: u32 = 3;
pub const MATCH_GLOBREP: u32 = 16;
pub const MATCH_QUOTED: u32 = 32;
pub const MATCH_ASSIGNRHS: u32 = 64;
pub const MATCH_STARSUB: u32 = 128;
pub const FD_BITMAP_SIZE: u32 = 32;
pub const HEREDOC_MAX: u32 = 16;
pub const B_EOF: u32 = 1;
pub const B_ERROR: u32 = 2;

pub const B_WASBASHINPUT: u32 = 8;

pub const B_SHAREDBUF: i32 = 32;
pub const BUILTIN_ENABLED: i32 = 1;
pub const BUILTIN_DELETED: i32 = 2;
pub const STATIC_BUILTIN: i32 = 4;
pub const SPECIAL_BUILTIN: i32 = 8;
pub const ASSIGNMENT_BUILTIN: i32 = 16;
pub const POSIX_BUILTIN: i32 = 32;
pub const LOCALVAR_BUILTIN: i32 = 64;
pub const REQUIRES_BUILTIN: i32 = 128;
pub const BASE_INDENT: i32 = 4;
pub const PST_CASEPAT: i32 = 1;
pub const PST_ALEXPNEXT: i32 = 2;
pub const PST_ALLOWOPNBRC: i32 = 4;
pub const PST_NEEDCLOSBRC: i32 = 8;
pub const PST_DBLPAREN: u32 = 16;
pub const PST_SUBSHELL: u32 = 32;
pub const PST_CMDSUBST: u32 = 64;
pub const PST_CASESTMT: u32 = 128;
pub const PST_CONDCMD: u32 = 256;
pub const PST_CONDEXPR: u32 = 512;
pub const PST_ARITHFOR: u32 = 1024;
pub const PST_ALEXPAND: u32 = 2048;
pub const PST_EXTPAT: u32 = 4096;
pub const PST_COMPASSIGN: u32 = 8192;
pub const PST_ASSIGNOK: u32 = 16384;
pub const PST_EOFTOKEN: u32 = 32768;
pub const PST_REGEXP: u32 = 65536;
pub const PST_HEREDOC: u32 = 131072;
pub const PST_REPARSE: u32 = 262144;
pub const PST_REDIRLIST: u32 = 524288;
pub const PST_COMMENT: u32 = 1048576;
pub const PST_ENDALIAS: u32 = 2097152;
pub const DOLBRACE_PARAM: u32 = 1;
pub const DOLBRACE_OP: u32 = 2;
pub const DOLBRACE_WORD: u32 = 4;
pub const DOLBRACE_QUOTE: u32 = 64;
pub const DOLBRACE_QUOTE2: u32 = 128;
pub const YYDEBUG: u32 = 0;
pub const IF: u32 = 258;
pub const THEN: u32 = 259;
pub const ELSE: u32 = 260;
pub const ELIF: u32 = 261;
pub const FI: u32 = 262;
pub const CASE: u32 = 263;
pub const ESAC: u32 = 264;
pub const FOR: u32 = 265;
pub const SELECT: u32 = 266;
pub const WHILE: u32 = 267;
pub const UNTIL: u32 = 268;
pub const DO: u32 = 269;
pub const DONE: u32 = 270;
pub const FUNCTION: u32 = 271;
pub const COPROC: u32 = 272;
pub const COND_START: u32 = 273;
pub const COND_END: u32 = 274;
pub const COND_ERROR: u32 = 275;
pub const IN: u32 = 276;
pub const BANG: u32 = 277;
pub const TIME: u32 = 278;
pub const TIMEOPT: u32 = 279;
pub const TIMEIGN: u32 = 280;
pub const WORD: u32 = 281;
pub const ASSIGNMENT_WORD: u32 = 282;
pub const REDIR_WORD: u32 = 283;
pub const NUMBER: u32 = 284;
pub const ARITH_CMD: u32 = 285;
pub const ARITH_FOR_EXPRS: u32 = 286;
pub const COND_CMD: u32 = 287;

pub const GREATER_GREATER: u32 = 290;
pub const LESS_LESS: u32 = 291;
pub const LESS_AND: u32 = 292;
pub const LESS_LESS_LESS: u32 = 293;
pub const GREATER_AND: u32 = 294;
pub const SEMI_SEMI: u32 = 295;
pub const SEMI_AND: u32 = 296;
pub const SEMI_SEMI_AND: u32 = 297;
pub const LESS_LESS_MINUS: u32 = 298;
pub const AND_GREATER: u32 = 299;
pub const AND_GREATER_GREATER: u32 = 300;
pub const LESS_GREATER: u32 = 301;
pub const GREATER_BAR: u32 = 302;
pub const BAR_AND: u32 = 303;
pub const yacc_EOF: u32 = 304;
pub const YYSTYPE_IS_TRIVIAL: u32 = 1;
pub const YYSTYPE_IS_DECLARED: u32 = 1;
pub const _FCNTL_H: u32 = 1;
pub const __O_LARGEFILE: u32 = 0;
pub const F_GETLK64: u32 = 5;
pub const F_SETLK64: u32 = 6;
pub const F_SETLKW64: u32 = 7;
pub const __iovec_defined: u32 = 1;
pub const O_ACCMODE: u32 = 3;

pub const O_WRONLY: u32 = 1;

pub const O_CREAT: u32 = 64;
pub const O_EXCL: u32 = 128;
pub const O_NOCTTY: u32 = 256;
pub const O_TRUNC: u32 = 512;
pub const O_APPEND: u32 = 1024;

pub const O_SYNC: u32 = 1052672;
pub const O_FSYNC: u32 = 1052672;
pub const O_ASYNC: u32 = 8192;
pub const __O_DIRECTORY: u32 = 65536;
pub const __O_NOFOLLOW: u32 = 131072;
pub const __O_CLOEXEC: u32 = 524288;
pub const __O_DIRECT: u32 = 16384;
pub const __O_NOATIME: u32 = 262144;
pub const __O_PATH: u32 = 2097152;
pub const __O_DSYNC: u32 = 4096;
pub const __O_TMPFILE: u32 = 4259840;
pub const F_GETLK: u32 = 5;
pub const F_SETLK: u32 = 6;
pub const F_SETLKW: u32 = 7;
pub const F_OFD_GETLK: u32 = 36;
pub const F_OFD_SETLK: u32 = 37;
pub const F_OFD_SETLKW: u32 = 38;
pub const O_LARGEFILE: u32 = 0;
pub const O_DIRECTORY: u32 = 65536;
pub const O_NOFOLLOW: u32 = 131072;
pub const O_CLOEXEC: u32 = 524288;
pub const O_DIRECT: u32 = 16384;
pub const O_NOATIME: u32 = 262144;
pub const O_PATH: u32 = 2097152;
pub const O_TMPFILE: u32 = 4259840;
pub const O_DSYNC: u32 = 4096;
pub const O_RSYNC: u32 = 1052672;
pub const F_DUPFD: u32 = 0;

pub const __F_SETOWN: u32 = 8;
pub const __F_GETOWN: u32 = 9;
pub const F_SETOWN: u32 = 8;
pub const F_GETOWN: u32 = 9;
pub const __F_SETSIG: u32 = 10;
pub const __F_GETSIG: u32 = 11;
pub const __F_SETOWN_EX: u32 = 15;
pub const __F_GETOWN_EX: u32 = 16;
pub const F_SETSIG: u32 = 10;
pub const F_GETSIG: u32 = 11;
pub const F_SETOWN_EX: u32 = 15;
pub const F_GETOWN_EX: u32 = 16;
pub const F_SETLEASE: u32 = 1024;
pub const F_GETLEASE: u32 = 1025;
pub const F_NOTIFY: u32 = 1026;
pub const F_SETPIPE_SZ: u32 = 1031;
pub const F_ADD_SEALS: u32 = 1033;
pub const F_GET_SEALS: u32 = 1034;
pub const F_GET_RW_HINT: u32 = 1035;
pub const F_SET_RW_HINT: u32 = 1036;
pub const F_GET_FILE_RW_HINT: u32 = 1037;
pub const F_SET_FILE_RW_HINT: u32 = 1038;
pub const F_DUPFD_CLOEXEC: u32 = 1030;

pub const F_RDLCK: u32 = 0;
pub const F_WRLCK: u32 = 1;
pub const F_UNLCK: u32 = 2;
pub const F_EXLCK: u32 = 4;
pub const F_SHLCK: u32 = 8;
pub const LOCK_SH: u32 = 1;
pub const LOCK_EX: u32 = 2;
pub const LOCK_NB: u32 = 4;
pub const LOCK_UN: u32 = 8;
pub const LOCK_MAND: u32 = 32;
pub const LOCK_READ: u32 = 64;
pub const LOCK_WRITE: u32 = 128;
pub const LOCK_RW: u32 = 192;
pub const DN_ACCESS: u32 = 1;
pub const DN_MODIFY: u32 = 2;
pub const DN_CREATE: u32 = 4;
pub const DN_DELETE: u32 = 8;
pub const DN_RENAME: u32 = 16;
pub const DN_ATTRIB: u32 = 32;
pub const DN_MULTISHOT: u32 = 2147483648;
pub const F_SEAL_SEAL: u32 = 1;
pub const F_SEAL_SHRINK: u32 = 2;
pub const F_SEAL_GROW: u32 = 4;
pub const F_SEAL_WRITE: u32 = 8;
pub const RWF_WRITE_LIFE_NOT_SET: u32 = 0;
pub const RWH_WRITE_LIFE_NONE: u32 = 1;
pub const RWH_WRITE_LIFE_SHORT: u32 = 2;
pub const RWH_WRITE_LIFE_MEDIUM: u32 = 3;
pub const RWH_WRITE_LIFE_LONG: u32 = 4;
pub const RWH_WRITE_LIFE_EXTREME: u32 = 5;
pub const FAPPEND: u32 = 1024;
pub const FFSYNC: u32 = 1052672;
pub const FASYNC: u32 = 8192;
pub const FNONBLOCK: u32 = 2048;
pub const FNDELAY: u32 = 2048;
pub const __POSIX_FADV_DONTNEED: u32 = 4;
pub const __POSIX_FADV_NOREUSE: u32 = 5;
pub const POSIX_FADV_NORMAL: u32 = 0;
pub const POSIX_FADV_RANDOM: u32 = 1;
pub const POSIX_FADV_SEQUENTIAL: u32 = 2;
pub const POSIX_FADV_WILLNEED: u32 = 3;
pub const POSIX_FADV_DONTNEED: u32 = 4;
pub const POSIX_FADV_NOREUSE: u32 = 5;
pub const SYNC_FILE_RANGE_WAIT_BEFORE: u32 = 1;
pub const SYNC_FILE_RANGE_WRITE: u32 = 2;
pub const SYNC_FILE_RANGE_WAIT_AFTER: u32 = 4;
pub const SPLICE_F_MOVE: u32 = 1;
pub const SPLICE_F_NONBLOCK: u32 = 2;
pub const SPLICE_F_MORE: u32 = 4;
pub const SPLICE_F_GIFT: u32 = 8;
pub const FALLOC_FL_KEEP_SIZE: u32 = 1;
pub const FALLOC_FL_PUNCH_HOLE: u32 = 2;
pub const FALLOC_FL_NO_HIDE_STALE: u32 = 4;
pub const FALLOC_FL_COLLAPSE_RANGE: u32 = 8;
pub const FALLOC_FL_ZERO_RANGE: u32 = 16;
pub const FALLOC_FL_INSERT_RANGE: u32 = 32;
pub const FALLOC_FL_UNSHARE_RANGE: u32 = 64;
pub const MAX_HANDLE_SZ: u32 = 128;
pub const AT_FDCWD: i32 = -100;
pub const AT_SYMLINK_NOFOLLOW: u32 = 256;
pub const AT_REMOVEDIR: u32 = 512;
pub const AT_SYMLINK_FOLLOW: u32 = 1024;
pub const AT_NO_AUTOMOUNT: u32 = 2048;
pub const AT_EMPTY_PATH: u32 = 4096;
pub const AT_STATX_SYNC_TYPE: u32 = 24576;
pub const AT_STATX_SYNC_AS_STAT: u32 = 0;
pub const AT_STATX_FORCE_SYNC: u32 = 8192;
pub const AT_STATX_DONT_SYNC: u32 = 16384;
pub const AT_EACCESS: u32 = 512;
pub const _BITS_STAT_H: u32 = 1;
pub const _STAT_VER_KERNEL: u32 = 0;
pub const _STAT_VER_LINUX: u32 = 1;
pub const _MKNOD_VER_LINUX: u32 = 0;
pub const _STAT_VER: u32 = 1;
pub const __S_IFMT: u32 = 61440;
pub const __S_IFDIR: u32 = 16384;
pub const __S_IFCHR: u32 = 8192;
pub const __S_IFBLK: u32 = 24576;
pub const __S_IFREG: u32 = 32768;
pub const __S_IFIFO: u32 = 4096;
pub const __S_IFLNK: u32 = 40960;
pub const __S_IFSOCK: u32 = 49152;
pub const __S_ISUID: u32 = 2048;
pub const __S_ISGID: u32 = 1024;
pub const __S_ISVTX: u32 = 512;
pub const __S_IREAD: u32 = 256;
pub const __S_IWRITE: u32 = 128;
pub const __S_IEXEC: u32 = 64;
pub const UTIME_NOW: u32 = 1073741823;
pub const UTIME_OMIT: u32 = 1073741822;

pub const S_IFDIR: u32 = 16384;
pub const S_IFCHR: u32 = 8192;
pub const S_IFBLK: u32 = 24576;

pub const S_IFIFO: u32 = 4096;
pub const S_IFLNK: u32 = 40960;
pub const S_IFSOCK: u32 = 49152;
pub const S_ISUID: u32 = 2048;
pub const S_ISGID: u32 = 1024;
pub const S_ISVTX: u32 = 512;
pub const S_IRUSR: u32 = 256;
pub const S_IWUSR: u32 = 128;
pub const S_IXUSR: u32 = 64;
pub const S_IRWXU: u32 = 448;
pub const S_IRGRP: u32 = 32;
pub const S_IWGRP: u32 = 16;
pub const S_IXGRP: u32 = 8;
pub const S_IRWXG: u32 = 56;
pub const S_IROTH: u32 = 4;
pub const S_IWOTH: u32 = 2;
pub const S_IXOTH: u32 = 1;
pub const S_IRWXO: u32 = 7;
pub const FD_NCLOEXEC: u32 = 0;
pub const O_BINARY: u32 = 0;

pub const STAT_TIME_H: u32 = 1;
pub const _SYS_STAT_H: u32 = 1;
pub const S_IREAD: u32 = 256;
pub const S_IWRITE: u32 = 128;
pub const S_IEXEC: u32 = 64;
pub const ACCESSPERMS: u32 = 511;
pub const ALLPERMS: u32 = 4095;
pub const DEFFILEMODE: u32 = 438;
pub const S_BLKSIZE: u32 = 512;
pub const _MKNOD_VER: u32 = 0;
pub const STATX_TYPE: u32 = 1;
pub const STATX_MODE: u32 = 2;
pub const STATX_NLINK: u32 = 4;
pub const STATX_UID: u32 = 8;
pub const STATX_GID: u32 = 16;
pub const STATX_ATIME: u32 = 32;
pub const STATX_MTIME: u32 = 64;
pub const STATX_CTIME: u32 = 128;
pub const STATX_INO: u32 = 256;
pub const STATX_SIZE: u32 = 512;
pub const STATX_BLOCKS: u32 = 1024;
pub const STATX_BASIC_STATS: u32 = 2047;
pub const STATX_ALL: u32 = 4095;
pub const STATX_BTIME: u32 = 2048;
pub const STATX__RESERVED: u32 = 2147483648;
pub const STATX_ATTR_COMPRESSED: u32 = 4;
pub const STATX_ATTR_IMMUTABLE: u32 = 16;
pub const STATX_ATTR_APPEND: u32 = 32;
pub const STATX_ATTR_NODUMP: u32 = 64;
pub const STATX_ATTR_ENCRYPTED: u32 = 2048;
pub const STATX_ATTR_AUTOMOUNT: u32 = 4096;
pub const _TIME_H: u32 = 1;
pub const _BITS_TIME_H: u32 = 1;
pub const CLOCK_REALTIME: u32 = 0;
pub const CLOCK_MONOTONIC: u32 = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: u32 = 2;
pub const CLOCK_THREAD_CPUTIME_ID: u32 = 3;
pub const CLOCK_MONOTONIC_RAW: u32 = 4;
pub const CLOCK_REALTIME_COARSE: u32 = 5;
pub const CLOCK_MONOTONIC_COARSE: u32 = 6;
pub const CLOCK_BOOTTIME: u32 = 7;
pub const CLOCK_REALTIME_ALARM: u32 = 8;
pub const CLOCK_BOOTTIME_ALARM: u32 = 9;
pub const CLOCK_TAI: u32 = 11;
pub const TIMER_ABSTIME: u32 = 1;
pub const _BITS_TIMEX_H: u32 = 1;
pub const ADJ_OFFSET: u32 = 1;
pub const ADJ_FREQUENCY: u32 = 2;
pub const ADJ_MAXERROR: u32 = 4;
pub const ADJ_ESTERROR: u32 = 8;
pub const ADJ_STATUS: u32 = 16;
pub const ADJ_TIMECONST: u32 = 32;
pub const ADJ_TAI: u32 = 128;
pub const ADJ_SETOFFSET: u32 = 256;
pub const ADJ_MICRO: u32 = 4096;
pub const ADJ_NANO: u32 = 8192;
pub const ADJ_TICK: u32 = 16384;
pub const ADJ_OFFSET_SINGLESHOT: u32 = 32769;
pub const ADJ_OFFSET_SS_READ: u32 = 40961;
pub const MOD_OFFSET: u32 = 1;
pub const MOD_FREQUENCY: u32 = 2;
pub const MOD_MAXERROR: u32 = 4;
pub const MOD_ESTERROR: u32 = 8;
pub const MOD_STATUS: u32 = 16;
pub const MOD_TIMECONST: u32 = 32;
pub const MOD_CLKB: u32 = 16384;
pub const MOD_CLKA: u32 = 32769;
pub const MOD_TAI: u32 = 128;
pub const MOD_MICRO: u32 = 4096;
pub const MOD_NANO: u32 = 8192;
pub const STA_PLL: u32 = 1;
pub const STA_PPSFREQ: u32 = 2;
pub const STA_PPSTIME: u32 = 4;
pub const STA_FLL: u32 = 8;
pub const STA_INS: u32 = 16;
pub const STA_DEL: u32 = 32;
pub const STA_UNSYNC: u32 = 64;
pub const STA_FREQHOLD: u32 = 128;
pub const STA_PPSSIGNAL: u32 = 256;
pub const STA_PPSJITTER: u32 = 512;
pub const STA_PPSWANDER: u32 = 1024;
pub const STA_PPSERROR: u32 = 2048;
pub const STA_CLOCKERR: u32 = 4096;
pub const STA_NANO: u32 = 8192;
pub const STA_MODE: u32 = 16384;
pub const STA_CLK: u32 = 32768;
pub const STA_RONLY: u32 = 65280;
pub const __struct_tm_defined: u32 = 1;
pub const __itimerspec_defined: u32 = 1;
pub const TIME_UTC: u32 = 1;
pub const _BASH_SYSTIMES_H: u32 = 1;
pub const _SYS_TIMES_H: u32 = 1;
pub const _DIRENT_H: u32 = 1;
pub const _DIRENT_MATCHES_DIRENT64: u32 = 1;
pub const MAXNAMLEN: u32 = 255;
pub const D_FILENO_AVAILABLE: u32 = 1;
pub const _SHMBCHAR_H: u32 = 1;
pub const IS_BASIC_ASCII: u32 = 1;
pub const _STDLIB_H_: u32 = 1;
pub const S_IRUGO: u32 = 292;
pub const S_IWUGO: u32 = 146;
pub const S_IXUGO: u32 = 73;
pub const USEC_PER_SEC: u32 = 1000000;
pub const _TERMIOS_H: u32 = 1;
pub const NCCS: u32 = 32;
pub const _HAVE_STRUCT_TERMIOS_C_ISPEED: u32 = 1;
pub const _HAVE_STRUCT_TERMIOS_C_OSPEED: u32 = 1;
pub const VINTR: u32 = 0;
pub const VQUIT: u32 = 1;
pub const VERASE: u32 = 2;
pub const VKILL: u32 = 3;
pub const VEOF: u32 = 4;
pub const VTIME: u32 = 5;
pub const VMIN: u32 = 6;
pub const VSWTC: u32 = 7;
pub const VSTART: u32 = 8;
pub const VSTOP: u32 = 9;
pub const VSUSP: u32 = 10;
pub const VEOL: u32 = 11;
pub const VREPRINT: u32 = 12;
pub const VDISCARD: u32 = 13;
pub const VWERASE: u32 = 14;
pub const VLNEXT: u32 = 15;
pub const VEOL2: u32 = 16;
pub const IGNBRK: u32 = 1;
pub const BRKINT: u32 = 2;
pub const IGNPAR: u32 = 4;
pub const PARMRK: u32 = 8;
pub const INPCK: u32 = 16;
pub const ISTRIP: u32 = 32;
pub const INLCR: u32 = 64;
pub const IGNCR: u32 = 128;
pub const ICRNL: u32 = 256;
pub const IUCLC: u32 = 512;
pub const IXON: u32 = 1024;
pub const IXANY: u32 = 2048;
pub const IXOFF: u32 = 4096;
pub const IMAXBEL: u32 = 8192;
pub const IUTF8: u32 = 16384;
pub const OPOST: u32 = 1;
pub const OLCUC: u32 = 2;
pub const ONLCR: u32 = 4;
pub const OCRNL: u32 = 8;
pub const ONOCR: u32 = 16;
pub const ONLRET: u32 = 32;
pub const OFILL: u32 = 64;
pub const OFDEL: u32 = 128;
pub const NLDLY: u32 = 256;
pub const NL0: u32 = 0;
pub const NL1: u32 = 256;
pub const CRDLY: u32 = 1536;
pub const CR0: u32 = 0;
pub const CR1: u32 = 512;
pub const CR2: u32 = 1024;
pub const CR3: u32 = 1536;
pub const TABDLY: u32 = 6144;
pub const TAB0: u32 = 0;
pub const TAB1: u32 = 2048;
pub const TAB2: u32 = 4096;
pub const TAB3: u32 = 6144;
pub const BSDLY: u32 = 8192;
pub const BS0: u32 = 0;
pub const BS1: u32 = 8192;
pub const FFDLY: u32 = 32768;
pub const FF0: u32 = 0;
pub const FF1: u32 = 32768;
pub const VTDLY: u32 = 16384;
pub const VT0: u32 = 0;
pub const VT1: u32 = 16384;
pub const XTABS: u32 = 6144;
pub const CBAUD: u32 = 4111;
pub const B0: u32 = 0;
pub const B50: u32 = 1;
pub const B75: u32 = 2;
pub const B110: u32 = 3;
pub const B134: u32 = 4;
pub const B150: u32 = 5;
pub const B200: u32 = 6;
pub const B300: u32 = 7;
pub const B600: u32 = 8;
pub const B1200: u32 = 9;
pub const B1800: u32 = 10;
pub const B2400: u32 = 11;
pub const B4800: u32 = 12;
pub const B9600: u32 = 13;
pub const B19200: u32 = 14;
pub const B38400: u32 = 15;
pub const EXTA: u32 = 14;
pub const EXTB: u32 = 15;
pub const CSIZE: u32 = 48;
pub const CS5: u32 = 0;
pub const CS6: u32 = 16;
pub const CS7: u32 = 32;
pub const CS8: u32 = 48;
pub const CSTOPB: u32 = 64;
pub const CREAD: u32 = 128;
pub const PARENB: u32 = 256;
pub const PARODD: u32 = 512;
pub const HUPCL: u32 = 1024;
pub const CLOCAL: u32 = 2048;
pub const CBAUDEX: u32 = 4096;
pub const B57600: u32 = 4097;
pub const B115200: u32 = 4098;
pub const B230400: u32 = 4099;
pub const B460800: u32 = 4100;
pub const B500000: u32 = 4101;
pub const B576000: u32 = 4102;
pub const B921600: u32 = 4103;
pub const B1000000: u32 = 4104;
pub const B1152000: u32 = 4105;
pub const B1500000: u32 = 4106;
pub const B2000000: u32 = 4107;
pub const B2500000: u32 = 4108;
pub const B3000000: u32 = 4109;
pub const B3500000: u32 = 4110;
pub const B4000000: u32 = 4111;
pub const __MAX_BAUD: u32 = 4111;
pub const CIBAUD: u32 = 269418496;
pub const CMSPAR: u32 = 1073741824;
pub const CRTSCTS: u32 = 2147483648;
pub const ISIG: u32 = 1;
pub const ICANON: u32 = 2;
pub const XCASE: u32 = 4;
pub const ECHO: u32 = 8;
pub const ECHOE: u32 = 16;
pub const ECHOK: u32 = 32;
pub const ECHONL: u32 = 64;
pub const NOFLSH: u32 = 128;
pub const TOSTOP: u32 = 256;
pub const ECHOCTL: u32 = 512;
pub const ECHOPRT: u32 = 1024;
pub const ECHOKE: u32 = 2048;
pub const FLUSHO: u32 = 4096;
pub const PENDIN: u32 = 16384;
pub const IEXTEN: u32 = 32768;
pub const EXTPROC: u32 = 65536;
pub const TCOOFF: u32 = 0;
pub const TCOON: u32 = 1;
pub const TCIOFF: u32 = 2;
pub const TCION: u32 = 3;
pub const TCIFLUSH: u32 = 0;
pub const TCOFLUSH: u32 = 1;
pub const TCIOFLUSH: u32 = 2;
pub const TCSANOW: u32 = 0;
pub const TCSADRAIN: u32 = 1;
pub const TCSAFLUSH: u32 = 2;
pub const TTYDEF_IFLAG: u32 = 11554;
pub const TTYDEF_OFLAG: u32 = 6149;
pub const TTYDEF_LFLAG: u32 = 35355;
pub const TTYDEF_CFLAG: u32 = 1440;
pub const TTYDEF_SPEED: u32 = 13;
pub const CEOL: u8 = 0u8;
pub const CERASE: u32 = 127;
pub const CSTATUS: u8 = 0u8;
pub const CMIN: u32 = 1;
pub const CQUIT: u32 = 28;
pub const CTIME: u32 = 0;
pub const CBRK: u8 = 0u8;
pub const GETOPT_EOF: i32 = -1;
pub const GETOPT_HELP: i32 = -99;
pub const _SH_GETOPT_H: u32 = 1;
pub const SEVAL_NONINT: u32 = 1;
pub const SEVAL_INTERACT: u32 = 2;

pub const SEVAL_NOFREE: u32 = 8;

pub const SEVAL_PARSEONLY: u32 = 32;
pub const SEVAL_NOLONGJMP: u32 = 64;
pub const SEVAL_FUNCDEF: u32 = 128;
pub const SEVAL_ONECMD: u32 = 256;
pub const SEVAL_NOHISTEXP: u32 = 512;
pub const CDESC_ALL: u32 = 1;
pub const CDESC_SHORTDESC: u32 = 2;
pub const CDESC_REUSABLE: u32 = 4;
pub const CDESC_TYPE: u32 = 8;
pub const CDESC_PATH_ONLY: u32 = 16;
pub const CDESC_FORCE_PATH: u32 = 32;
pub const CDESC_NOFUNCS: u32 = 64;
pub const CDESC_ABSPATH: u32 = 128;
pub const CDESC_STDPATH: u32 = 256;
pub const JM_PREFIX: u32 = 1;
pub const JM_SUBSTRING: u32 = 2;
pub const JM_EXACT: u32 = 4;
pub const JM_STOPPED: u32 = 8;
pub const JM_FIRSTMATCH: u32 = 16;
pub const ARGS_NONE: u32 = 0;
pub const ARGS_INVOC: u32 = 1;
pub const ARGS_FUNC: u32 = 2;
pub const ARGS_SETBLTIN: u32 = 4;
pub const MAX_ATTRIBUTES: u32 = 16;
pub const TEST_PATMATCH: u32 = 1;
pub const TEST_ARITHEXP: u32 = 2;
pub const TEST_LOCALE: u32 = 4;
pub type _Float32 = f32;
pub type _Float64 = f64;
pub type _Float32x = f64;
pub type _Float64x = f64;

pub type va_list = __builtin_va_list;
pub type __gnuc_va_list = __builtin_va_list;

#[repr(C)]
#[derive(Copy, Clone)]
pub union __mbstate_t__bindgen_ty_1 {
    pub __wch: ::std::os::raw::c_uint,
    pub __wchb: [::std::os::raw::c_char; 4usize],
    _bindgen_union_align: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13usize],
    pub __ctype_b: *const ::std::os::raw::c_ushort,
    pub __ctype_tolower: *const ::std::os::raw::c_int,
    pub __ctype_toupper: *const ::std::os::raw::c_int,
    pub __names: [*const ::std::os::raw::c_char; 13usize],
}
pub type __locale_t = *mut __locale_struct;
pub type locale_t = __locale_t;

pub type __u_char = ::std::os::raw::c_uchar;
pub type __u_short = ::std::os::raw::c_ushort;
pub type __u_int = ::std::os::raw::c_uint;
pub type __u_long = ::std::os::raw::c_ulong;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __int_least8_t = __int8_t;
pub type __uint_least8_t = __uint8_t;
pub type __int_least16_t = __int16_t;
pub type __uint_least16_t = __uint16_t;
pub type __int_least32_t = __int32_t;
pub type __uint_least32_t = __uint32_t;
pub type __int_least64_t = __int64_t;
pub type __uint_least64_t = __uint64_t;
pub type __quad_t = ::std::os::raw::c_long;
pub type __u_quad_t = ::std::os::raw::c_ulong;

pub type __uintmax_t = ::std::os::raw::c_ulong;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [::std::os::raw::c_int; 2usize],
}
pub type __clock_t = ::std::os::raw::c_long;

pub type __rlim64_t = ::std::os::raw::c_ulong;
pub type __id_t = ::std::os::raw::c_uint;

pub type __useconds_t = ::std::os::raw::c_uint;
pub type __suseconds_t = ::std::os::raw::c_long;
pub type __daddr_t = ::std::os::raw::c_int;
pub type __key_t = ::std::os::raw::c_int;
pub type __clockid_t = ::std::os::raw::c_int;
pub type __timer_t = *mut ::std::os::raw::c_void;

pub type __blkcnt64_t = ::std::os::raw::c_long;
pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;
pub type __fsword_t = ::std::os::raw::c_long;

pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut ::std::os::raw::c_char;
pub type __intptr_t = ::std::os::raw::c_long;
pub type __socklen_t = ::std::os::raw::c_uint;

pub type wctype_t = ::std::os::raw::c_ulong;
pub const __ISwupper: _bindgen_ty_1 = 0;
pub const __ISwlower: _bindgen_ty_1 = 1;
pub const __ISwalpha: _bindgen_ty_1 = 2;
pub const __ISwdigit: _bindgen_ty_1 = 3;
pub const __ISwxdigit: _bindgen_ty_1 = 4;
pub const __ISwspace: _bindgen_ty_1 = 5;
pub const __ISwprint: _bindgen_ty_1 = 6;
pub const __ISwgraph: _bindgen_ty_1 = 7;
pub const __ISwblank: _bindgen_ty_1 = 8;
pub const __ISwcntrl: _bindgen_ty_1 = 9;
pub const __ISwpunct: _bindgen_ty_1 = 10;
pub const __ISwalnum: _bindgen_ty_1 = 11;
pub const _ISwupper: _bindgen_ty_1 = 16777216;
pub const _ISwlower: _bindgen_ty_1 = 33554432;
pub const _ISwalpha: _bindgen_ty_1 = 67108864;
pub const _ISwdigit: _bindgen_ty_1 = 134217728;
pub const _ISwxdigit: _bindgen_ty_1 = 268435456;
pub const _ISwspace: _bindgen_ty_1 = 536870912;
pub const _ISwprint: _bindgen_ty_1 = 1073741824;
pub const _ISwgraph: _bindgen_ty_1 = -2147483648;
pub const _ISwblank: _bindgen_ty_1 = 65536;
pub const _ISwcntrl: _bindgen_ty_1 = 131072;
pub const _ISwpunct: _bindgen_ty_1 = 262144;
pub const _ISwalnum: _bindgen_ty_1 = 524288;
pub type _bindgen_ty_1 = i32;

pub type off64_t = __off64_t;
pub type useconds_t = __useconds_t;

pub type socklen_t = __socklen_t;

pub const _PC_LINK_MAX: _bindgen_ty_2 = 0;
pub const _PC_MAX_CANON: _bindgen_ty_2 = 1;
pub const _PC_MAX_INPUT: _bindgen_ty_2 = 2;
pub const _PC_NAME_MAX: _bindgen_ty_2 = 3;
pub const _PC_PATH_MAX: _bindgen_ty_2 = 4;
pub const _PC_PIPE_BUF: _bindgen_ty_2 = 5;
pub const _PC_CHOWN_RESTRICTED: _bindgen_ty_2 = 6;
pub const _PC_NO_TRUNC: _bindgen_ty_2 = 7;
pub const _PC_VDISABLE: _bindgen_ty_2 = 8;
pub const _PC_SYNC_IO: _bindgen_ty_2 = 9;
pub const _PC_ASYNC_IO: _bindgen_ty_2 = 10;
pub const _PC_PRIO_IO: _bindgen_ty_2 = 11;
pub const _PC_SOCK_MAXBUF: _bindgen_ty_2 = 12;
pub const _PC_FILESIZEBITS: _bindgen_ty_2 = 13;
pub const _PC_REC_INCR_XFER_SIZE: _bindgen_ty_2 = 14;
pub const _PC_REC_MAX_XFER_SIZE: _bindgen_ty_2 = 15;
pub const _PC_REC_MIN_XFER_SIZE: _bindgen_ty_2 = 16;
pub const _PC_REC_XFER_ALIGN: _bindgen_ty_2 = 17;
pub const _PC_ALLOC_SIZE_MIN: _bindgen_ty_2 = 18;
pub const _PC_SYMLINK_MAX: _bindgen_ty_2 = 19;
pub const _PC_2_SYMLINKS: _bindgen_ty_2 = 20;
pub type _bindgen_ty_2 = u32;
pub const _SC_ARG_MAX: _bindgen_ty_3 = 0;
pub const _SC_CHILD_MAX: _bindgen_ty_3 = 1;
pub const _SC_CLK_TCK: _bindgen_ty_3 = 2;
pub const _SC_NGROUPS_MAX: _bindgen_ty_3 = 3;
pub const _SC_OPEN_MAX: _bindgen_ty_3 = 4;
pub const _SC_STREAM_MAX: _bindgen_ty_3 = 5;
pub const _SC_TZNAME_MAX: _bindgen_ty_3 = 6;
pub const _SC_JOB_CONTROL: _bindgen_ty_3 = 7;
pub const _SC_SAVED_IDS: _bindgen_ty_3 = 8;
pub const _SC_REALTIME_SIGNALS: _bindgen_ty_3 = 9;
pub const _SC_PRIORITY_SCHEDULING: _bindgen_ty_3 = 10;
pub const _SC_TIMERS: _bindgen_ty_3 = 11;
pub const _SC_ASYNCHRONOUS_IO: _bindgen_ty_3 = 12;
pub const _SC_PRIORITIZED_IO: _bindgen_ty_3 = 13;
pub const _SC_SYNCHRONIZED_IO: _bindgen_ty_3 = 14;
pub const _SC_FSYNC: _bindgen_ty_3 = 15;
pub const _SC_MAPPED_FILES: _bindgen_ty_3 = 16;
pub const _SC_MEMLOCK: _bindgen_ty_3 = 17;
pub const _SC_MEMLOCK_RANGE: _bindgen_ty_3 = 18;
pub const _SC_MEMORY_PROTECTION: _bindgen_ty_3 = 19;
pub const _SC_MESSAGE_PASSING: _bindgen_ty_3 = 20;
pub const _SC_SEMAPHORES: _bindgen_ty_3 = 21;
pub const _SC_SHARED_MEMORY_OBJECTS: _bindgen_ty_3 = 22;
pub const _SC_AIO_LISTIO_MAX: _bindgen_ty_3 = 23;
pub const _SC_AIO_MAX: _bindgen_ty_3 = 24;
pub const _SC_AIO_PRIO_DELTA_MAX: _bindgen_ty_3 = 25;
pub const _SC_DELAYTIMER_MAX: _bindgen_ty_3 = 26;
pub const _SC_MQ_OPEN_MAX: _bindgen_ty_3 = 27;
pub const _SC_MQ_PRIO_MAX: _bindgen_ty_3 = 28;
pub const _SC_VERSION: _bindgen_ty_3 = 29;
pub const _SC_PAGESIZE: _bindgen_ty_3 = 30;
pub const _SC_RTSIG_MAX: _bindgen_ty_3 = 31;
pub const _SC_SEM_NSEMS_MAX: _bindgen_ty_3 = 32;
pub const _SC_SEM_VALUE_MAX: _bindgen_ty_3 = 33;
pub const _SC_SIGQUEUE_MAX: _bindgen_ty_3 = 34;
pub const _SC_TIMER_MAX: _bindgen_ty_3 = 35;
pub const _SC_BC_BASE_MAX: _bindgen_ty_3 = 36;
pub const _SC_BC_DIM_MAX: _bindgen_ty_3 = 37;
pub const _SC_BC_SCALE_MAX: _bindgen_ty_3 = 38;
pub const _SC_BC_STRING_MAX: _bindgen_ty_3 = 39;
pub const _SC_COLL_WEIGHTS_MAX: _bindgen_ty_3 = 40;
pub const _SC_EQUIV_CLASS_MAX: _bindgen_ty_3 = 41;
pub const _SC_EXPR_NEST_MAX: _bindgen_ty_3 = 42;
pub const _SC_LINE_MAX: _bindgen_ty_3 = 43;
pub const _SC_RE_DUP_MAX: _bindgen_ty_3 = 44;
pub const _SC_CHARCLASS_NAME_MAX: _bindgen_ty_3 = 45;
pub const _SC_2_VERSION: _bindgen_ty_3 = 46;
pub const _SC_2_C_BIND: _bindgen_ty_3 = 47;
pub const _SC_2_C_DEV: _bindgen_ty_3 = 48;
pub const _SC_2_FORT_DEV: _bindgen_ty_3 = 49;
pub const _SC_2_FORT_RUN: _bindgen_ty_3 = 50;
pub const _SC_2_SW_DEV: _bindgen_ty_3 = 51;
pub const _SC_2_LOCALEDEF: _bindgen_ty_3 = 52;
pub const _SC_PII: _bindgen_ty_3 = 53;
pub const _SC_PII_XTI: _bindgen_ty_3 = 54;
pub const _SC_PII_SOCKET: _bindgen_ty_3 = 55;
pub const _SC_PII_INTERNET: _bindgen_ty_3 = 56;
pub const _SC_PII_OSI: _bindgen_ty_3 = 57;
pub const _SC_POLL: _bindgen_ty_3 = 58;
pub const _SC_SELECT: _bindgen_ty_3 = 59;
pub const _SC_UIO_MAXIOV: _bindgen_ty_3 = 60;
pub const _SC_IOV_MAX: _bindgen_ty_3 = 60;
pub const _SC_PII_INTERNET_STREAM: _bindgen_ty_3 = 61;
pub const _SC_PII_INTERNET_DGRAM: _bindgen_ty_3 = 62;
pub const _SC_PII_OSI_COTS: _bindgen_ty_3 = 63;
pub const _SC_PII_OSI_CLTS: _bindgen_ty_3 = 64;
pub const _SC_PII_OSI_M: _bindgen_ty_3 = 65;
pub const _SC_T_IOV_MAX: _bindgen_ty_3 = 66;
pub const _SC_THREADS: _bindgen_ty_3 = 67;
pub const _SC_THREAD_SAFE_FUNCTIONS: _bindgen_ty_3 = 68;
pub const _SC_GETGR_R_SIZE_MAX: _bindgen_ty_3 = 69;
pub const _SC_GETPW_R_SIZE_MAX: _bindgen_ty_3 = 70;
pub const _SC_LOGIN_NAME_MAX: _bindgen_ty_3 = 71;
pub const _SC_TTY_NAME_MAX: _bindgen_ty_3 = 72;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: _bindgen_ty_3 = 73;
pub const _SC_THREAD_KEYS_MAX: _bindgen_ty_3 = 74;
pub const _SC_THREAD_STACK_MIN: _bindgen_ty_3 = 75;
pub const _SC_THREAD_THREADS_MAX: _bindgen_ty_3 = 76;
pub const _SC_THREAD_ATTR_STACKADDR: _bindgen_ty_3 = 77;
pub const _SC_THREAD_ATTR_STACKSIZE: _bindgen_ty_3 = 78;
pub const _SC_THREAD_PRIORITY_SCHEDULING: _bindgen_ty_3 = 79;
pub const _SC_THREAD_PRIO_INHERIT: _bindgen_ty_3 = 80;
pub const _SC_THREAD_PRIO_PROTECT: _bindgen_ty_3 = 81;
pub const _SC_THREAD_PROCESS_SHARED: _bindgen_ty_3 = 82;
pub const _SC_NPROCESSORS_CONF: _bindgen_ty_3 = 83;
pub const _SC_NPROCESSORS_ONLN: _bindgen_ty_3 = 84;
pub const _SC_PHYS_PAGES: _bindgen_ty_3 = 85;
pub const _SC_AVPHYS_PAGES: _bindgen_ty_3 = 86;
pub const _SC_ATEXIT_MAX: _bindgen_ty_3 = 87;
pub const _SC_PASS_MAX: _bindgen_ty_3 = 88;
pub const _SC_XOPEN_VERSION: _bindgen_ty_3 = 89;
pub const _SC_XOPEN_XCU_VERSION: _bindgen_ty_3 = 90;
pub const _SC_XOPEN_UNIX: _bindgen_ty_3 = 91;
pub const _SC_XOPEN_CRYPT: _bindgen_ty_3 = 92;
pub const _SC_XOPEN_ENH_I18N: _bindgen_ty_3 = 93;
pub const _SC_XOPEN_SHM: _bindgen_ty_3 = 94;
pub const _SC_2_CHAR_TERM: _bindgen_ty_3 = 95;
pub const _SC_2_C_VERSION: _bindgen_ty_3 = 96;
pub const _SC_2_UPE: _bindgen_ty_3 = 97;
pub const _SC_XOPEN_XPG2: _bindgen_ty_3 = 98;
pub const _SC_XOPEN_XPG3: _bindgen_ty_3 = 99;
pub const _SC_XOPEN_XPG4: _bindgen_ty_3 = 100;
pub const _SC_CHAR_BIT: _bindgen_ty_3 = 101;
pub const _SC_CHAR_MAX: _bindgen_ty_3 = 102;
pub const _SC_CHAR_MIN: _bindgen_ty_3 = 103;
pub const _SC_INT_MAX: _bindgen_ty_3 = 104;
pub const _SC_INT_MIN: _bindgen_ty_3 = 105;
pub const _SC_LONG_BIT: _bindgen_ty_3 = 106;
pub const _SC_WORD_BIT: _bindgen_ty_3 = 107;
pub const _SC_MB_LEN_MAX: _bindgen_ty_3 = 108;
pub const _SC_NZERO: _bindgen_ty_3 = 109;
pub const _SC_SSIZE_MAX: _bindgen_ty_3 = 110;
pub const _SC_SCHAR_MAX: _bindgen_ty_3 = 111;
pub const _SC_SCHAR_MIN: _bindgen_ty_3 = 112;
pub const _SC_SHRT_MAX: _bindgen_ty_3 = 113;
pub const _SC_SHRT_MIN: _bindgen_ty_3 = 114;
pub const _SC_UCHAR_MAX: _bindgen_ty_3 = 115;
pub const _SC_UINT_MAX: _bindgen_ty_3 = 116;
pub const _SC_ULONG_MAX: _bindgen_ty_3 = 117;
pub const _SC_USHRT_MAX: _bindgen_ty_3 = 118;
pub const _SC_NL_ARGMAX: _bindgen_ty_3 = 119;
pub const _SC_NL_LANGMAX: _bindgen_ty_3 = 120;
pub const _SC_NL_MSGMAX: _bindgen_ty_3 = 121;
pub const _SC_NL_NMAX: _bindgen_ty_3 = 122;
pub const _SC_NL_SETMAX: _bindgen_ty_3 = 123;
pub const _SC_NL_TEXTMAX: _bindgen_ty_3 = 124;
pub const _SC_XBS5_ILP32_OFF32: _bindgen_ty_3 = 125;
pub const _SC_XBS5_ILP32_OFFBIG: _bindgen_ty_3 = 126;
pub const _SC_XBS5_LP64_OFF64: _bindgen_ty_3 = 127;
pub const _SC_XBS5_LPBIG_OFFBIG: _bindgen_ty_3 = 128;
pub const _SC_XOPEN_LEGACY: _bindgen_ty_3 = 129;
pub const _SC_XOPEN_REALTIME: _bindgen_ty_3 = 130;
pub const _SC_XOPEN_REALTIME_THREADS: _bindgen_ty_3 = 131;
pub const _SC_ADVISORY_INFO: _bindgen_ty_3 = 132;
pub const _SC_BARRIERS: _bindgen_ty_3 = 133;
pub const _SC_BASE: _bindgen_ty_3 = 134;
pub const _SC_C_LANG_SUPPORT: _bindgen_ty_3 = 135;
pub const _SC_C_LANG_SUPPORT_R: _bindgen_ty_3 = 136;
pub const _SC_CLOCK_SELECTION: _bindgen_ty_3 = 137;
pub const _SC_CPUTIME: _bindgen_ty_3 = 138;
pub const _SC_THREAD_CPUTIME: _bindgen_ty_3 = 139;
pub const _SC_DEVICE_IO: _bindgen_ty_3 = 140;
pub const _SC_DEVICE_SPECIFIC: _bindgen_ty_3 = 141;
pub const _SC_DEVICE_SPECIFIC_R: _bindgen_ty_3 = 142;
pub const _SC_FD_MGMT: _bindgen_ty_3 = 143;
pub const _SC_FIFO: _bindgen_ty_3 = 144;
pub const _SC_PIPE: _bindgen_ty_3 = 145;
pub const _SC_FILE_ATTRIBUTES: _bindgen_ty_3 = 146;
pub const _SC_FILE_LOCKING: _bindgen_ty_3 = 147;
pub const _SC_FILE_SYSTEM: _bindgen_ty_3 = 148;
pub const _SC_MONOTONIC_CLOCK: _bindgen_ty_3 = 149;
pub const _SC_MULTI_PROCESS: _bindgen_ty_3 = 150;
pub const _SC_SINGLE_PROCESS: _bindgen_ty_3 = 151;
pub const _SC_NETWORKING: _bindgen_ty_3 = 152;
pub const _SC_READER_WRITER_LOCKS: _bindgen_ty_3 = 153;
pub const _SC_SPIN_LOCKS: _bindgen_ty_3 = 154;
pub const _SC_REGEXP: _bindgen_ty_3 = 155;
pub const _SC_REGEX_VERSION: _bindgen_ty_3 = 156;
pub const _SC_SHELL: _bindgen_ty_3 = 157;
pub const _SC_SIGNALS: _bindgen_ty_3 = 158;
pub const _SC_SPAWN: _bindgen_ty_3 = 159;
pub const _SC_SPORADIC_SERVER: _bindgen_ty_3 = 160;
pub const _SC_THREAD_SPORADIC_SERVER: _bindgen_ty_3 = 161;
pub const _SC_SYSTEM_DATABASE: _bindgen_ty_3 = 162;
pub const _SC_SYSTEM_DATABASE_R: _bindgen_ty_3 = 163;
pub const _SC_TIMEOUTS: _bindgen_ty_3 = 164;
pub const _SC_TYPED_MEMORY_OBJECTS: _bindgen_ty_3 = 165;
pub const _SC_USER_GROUPS: _bindgen_ty_3 = 166;
pub const _SC_USER_GROUPS_R: _bindgen_ty_3 = 167;
pub const _SC_2_PBS: _bindgen_ty_3 = 168;
pub const _SC_2_PBS_ACCOUNTING: _bindgen_ty_3 = 169;
pub const _SC_2_PBS_LOCATE: _bindgen_ty_3 = 170;
pub const _SC_2_PBS_MESSAGE: _bindgen_ty_3 = 171;
pub const _SC_2_PBS_TRACK: _bindgen_ty_3 = 172;
pub const _SC_SYMLOOP_MAX: _bindgen_ty_3 = 173;
pub const _SC_STREAMS: _bindgen_ty_3 = 174;
pub const _SC_2_PBS_CHECKPOINT: _bindgen_ty_3 = 175;
pub const _SC_V6_ILP32_OFF32: _bindgen_ty_3 = 176;
pub const _SC_V6_ILP32_OFFBIG: _bindgen_ty_3 = 177;
pub const _SC_V6_LP64_OFF64: _bindgen_ty_3 = 178;
pub const _SC_V6_LPBIG_OFFBIG: _bindgen_ty_3 = 179;
pub const _SC_HOST_NAME_MAX: _bindgen_ty_3 = 180;
pub const _SC_TRACE: _bindgen_ty_3 = 181;
pub const _SC_TRACE_EVENT_FILTER: _bindgen_ty_3 = 182;
pub const _SC_TRACE_INHERIT: _bindgen_ty_3 = 183;
pub const _SC_TRACE_LOG: _bindgen_ty_3 = 184;
pub const _SC_LEVEL1_ICACHE_SIZE: _bindgen_ty_3 = 185;
pub const _SC_LEVEL1_ICACHE_ASSOC: _bindgen_ty_3 = 186;
pub const _SC_LEVEL1_ICACHE_LINESIZE: _bindgen_ty_3 = 187;
pub const _SC_LEVEL1_DCACHE_SIZE: _bindgen_ty_3 = 188;
pub const _SC_LEVEL1_DCACHE_ASSOC: _bindgen_ty_3 = 189;
pub const _SC_LEVEL1_DCACHE_LINESIZE: _bindgen_ty_3 = 190;
pub const _SC_LEVEL2_CACHE_SIZE: _bindgen_ty_3 = 191;
pub const _SC_LEVEL2_CACHE_ASSOC: _bindgen_ty_3 = 192;
pub const _SC_LEVEL2_CACHE_LINESIZE: _bindgen_ty_3 = 193;
pub const _SC_LEVEL3_CACHE_SIZE: _bindgen_ty_3 = 194;
pub const _SC_LEVEL3_CACHE_ASSOC: _bindgen_ty_3 = 195;
pub const _SC_LEVEL3_CACHE_LINESIZE: _bindgen_ty_3 = 196;
pub const _SC_LEVEL4_CACHE_SIZE: _bindgen_ty_3 = 197;
pub const _SC_LEVEL4_CACHE_ASSOC: _bindgen_ty_3 = 198;
pub const _SC_LEVEL4_CACHE_LINESIZE: _bindgen_ty_3 = 199;
pub const _SC_IPV6: _bindgen_ty_3 = 235;
pub const _SC_RAW_SOCKETS: _bindgen_ty_3 = 236;
pub const _SC_V7_ILP32_OFF32: _bindgen_ty_3 = 237;
pub const _SC_V7_ILP32_OFFBIG: _bindgen_ty_3 = 238;
pub const _SC_V7_LP64_OFF64: _bindgen_ty_3 = 239;
pub const _SC_V7_LPBIG_OFFBIG: _bindgen_ty_3 = 240;
pub const _SC_SS_REPL_MAX: _bindgen_ty_3 = 241;
pub const _SC_TRACE_EVENT_NAME_MAX: _bindgen_ty_3 = 242;
pub const _SC_TRACE_NAME_MAX: _bindgen_ty_3 = 243;
pub const _SC_TRACE_SYS_MAX: _bindgen_ty_3 = 244;
pub const _SC_TRACE_USER_EVENT_MAX: _bindgen_ty_3 = 245;
pub const _SC_XOPEN_STREAMS: _bindgen_ty_3 = 246;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: _bindgen_ty_3 = 247;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: _bindgen_ty_3 = 248;
pub type _bindgen_ty_3 = u32;

pub const _CS_V6_WIDTH_RESTRICTED_ENVS: _bindgen_ty_4 = 1;
pub const _CS_GNU_LIBC_VERSION: _bindgen_ty_4 = 2;
pub const _CS_GNU_LIBPTHREAD_VERSION: _bindgen_ty_4 = 3;
pub const _CS_V5_WIDTH_RESTRICTED_ENVS: _bindgen_ty_4 = 4;
pub const _CS_V7_WIDTH_RESTRICTED_ENVS: _bindgen_ty_4 = 5;
pub const _CS_LFS_CFLAGS: _bindgen_ty_4 = 1000;
pub const _CS_LFS_LDFLAGS: _bindgen_ty_4 = 1001;
pub const _CS_LFS_LIBS: _bindgen_ty_4 = 1002;
pub const _CS_LFS_LINTFLAGS: _bindgen_ty_4 = 1003;
pub const _CS_LFS64_CFLAGS: _bindgen_ty_4 = 1004;
pub const _CS_LFS64_LDFLAGS: _bindgen_ty_4 = 1005;
pub const _CS_LFS64_LIBS: _bindgen_ty_4 = 1006;
pub const _CS_LFS64_LINTFLAGS: _bindgen_ty_4 = 1007;
pub const _CS_XBS5_ILP32_OFF32_CFLAGS: _bindgen_ty_4 = 1100;
pub const _CS_XBS5_ILP32_OFF32_LDFLAGS: _bindgen_ty_4 = 1101;
pub const _CS_XBS5_ILP32_OFF32_LIBS: _bindgen_ty_4 = 1102;
pub const _CS_XBS5_ILP32_OFF32_LINTFLAGS: _bindgen_ty_4 = 1103;
pub const _CS_XBS5_ILP32_OFFBIG_CFLAGS: _bindgen_ty_4 = 1104;
pub const _CS_XBS5_ILP32_OFFBIG_LDFLAGS: _bindgen_ty_4 = 1105;
pub const _CS_XBS5_ILP32_OFFBIG_LIBS: _bindgen_ty_4 = 1106;
pub const _CS_XBS5_ILP32_OFFBIG_LINTFLAGS: _bindgen_ty_4 = 1107;
pub const _CS_XBS5_LP64_OFF64_CFLAGS: _bindgen_ty_4 = 1108;
pub const _CS_XBS5_LP64_OFF64_LDFLAGS: _bindgen_ty_4 = 1109;
pub const _CS_XBS5_LP64_OFF64_LIBS: _bindgen_ty_4 = 1110;
pub const _CS_XBS5_LP64_OFF64_LINTFLAGS: _bindgen_ty_4 = 1111;
pub const _CS_XBS5_LPBIG_OFFBIG_CFLAGS: _bindgen_ty_4 = 1112;
pub const _CS_XBS5_LPBIG_OFFBIG_LDFLAGS: _bindgen_ty_4 = 1113;
pub const _CS_XBS5_LPBIG_OFFBIG_LIBS: _bindgen_ty_4 = 1114;
pub const _CS_XBS5_LPBIG_OFFBIG_LINTFLAGS: _bindgen_ty_4 = 1115;
pub const _CS_POSIX_V6_ILP32_OFF32_CFLAGS: _bindgen_ty_4 = 1116;
pub const _CS_POSIX_V6_ILP32_OFF32_LDFLAGS: _bindgen_ty_4 = 1117;
pub const _CS_POSIX_V6_ILP32_OFF32_LIBS: _bindgen_ty_4 = 1118;
pub const _CS_POSIX_V6_ILP32_OFF32_LINTFLAGS: _bindgen_ty_4 = 1119;
pub const _CS_POSIX_V6_ILP32_OFFBIG_CFLAGS: _bindgen_ty_4 = 1120;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LDFLAGS: _bindgen_ty_4 = 1121;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LIBS: _bindgen_ty_4 = 1122;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LINTFLAGS: _bindgen_ty_4 = 1123;
pub const _CS_POSIX_V6_LP64_OFF64_CFLAGS: _bindgen_ty_4 = 1124;
pub const _CS_POSIX_V6_LP64_OFF64_LDFLAGS: _bindgen_ty_4 = 1125;
pub const _CS_POSIX_V6_LP64_OFF64_LIBS: _bindgen_ty_4 = 1126;
pub const _CS_POSIX_V6_LP64_OFF64_LINTFLAGS: _bindgen_ty_4 = 1127;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_CFLAGS: _bindgen_ty_4 = 1128;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LDFLAGS: _bindgen_ty_4 = 1129;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LIBS: _bindgen_ty_4 = 1130;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LINTFLAGS: _bindgen_ty_4 = 1131;
pub const _CS_POSIX_V7_ILP32_OFF32_CFLAGS: _bindgen_ty_4 = 1132;
pub const _CS_POSIX_V7_ILP32_OFF32_LDFLAGS: _bindgen_ty_4 = 1133;
pub const _CS_POSIX_V7_ILP32_OFF32_LIBS: _bindgen_ty_4 = 1134;
pub const _CS_POSIX_V7_ILP32_OFF32_LINTFLAGS: _bindgen_ty_4 = 1135;
pub const _CS_POSIX_V7_ILP32_OFFBIG_CFLAGS: _bindgen_ty_4 = 1136;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS: _bindgen_ty_4 = 1137;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LIBS: _bindgen_ty_4 = 1138;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LINTFLAGS: _bindgen_ty_4 = 1139;
pub const _CS_POSIX_V7_LP64_OFF64_CFLAGS: _bindgen_ty_4 = 1140;
pub const _CS_POSIX_V7_LP64_OFF64_LDFLAGS: _bindgen_ty_4 = 1141;
pub const _CS_POSIX_V7_LP64_OFF64_LIBS: _bindgen_ty_4 = 1142;
pub const _CS_POSIX_V7_LP64_OFF64_LINTFLAGS: _bindgen_ty_4 = 1143;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS: _bindgen_ty_4 = 1144;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS: _bindgen_ty_4 = 1145;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LIBS: _bindgen_ty_4 = 1146;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LINTFLAGS: _bindgen_ty_4 = 1147;
pub const _CS_V6_ENV: _bindgen_ty_4 = 1148;
pub const _CS_V7_ENV: _bindgen_ty_4 = 1149;
pub type _bindgen_ty_4 = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub union sigval {
    pub sival_int: ::std::os::raw::c_int,
    pub sival_ptr: *mut ::std::os::raw::c_void,
    _bindgen_union_align: u64,
}
pub type __sigval_t = sigval;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct siginfo_t {
    pub si_signo: ::std::os::raw::c_int,
    pub si_errno: ::std::os::raw::c_int,
    pub si_code: ::std::os::raw::c_int,
    pub __pad0: ::std::os::raw::c_int,
    pub _sifields: siginfo_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union siginfo_t__bindgen_ty_1 {
    pub _pad: [::std::os::raw::c_int; 28usize],
    pub _kill: siginfo_t__bindgen_ty_1__bindgen_ty_1,
    pub _timer: siginfo_t__bindgen_ty_1__bindgen_ty_2,
    pub _rt: siginfo_t__bindgen_ty_1__bindgen_ty_3,
    pub _sigchld: siginfo_t__bindgen_ty_1__bindgen_ty_4,
    pub _sigfault: siginfo_t__bindgen_ty_1__bindgen_ty_5,
    pub _sigpoll: siginfo_t__bindgen_ty_1__bindgen_ty_6,
    pub _sigsys: siginfo_t__bindgen_ty_1__bindgen_ty_7,
    _bindgen_union_align: [u64; 14usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct siginfo_t__bindgen_ty_1__bindgen_ty_1 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct siginfo_t__bindgen_ty_1__bindgen_ty_2 {
    pub si_tid: ::std::os::raw::c_int,
    pub si_overrun: ::std::os::raw::c_int,
    pub si_sigval: __sigval_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct siginfo_t__bindgen_ty_1__bindgen_ty_3 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct siginfo_t__bindgen_ty_1__bindgen_ty_4 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: ::std::os::raw::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct siginfo_t__bindgen_ty_1__bindgen_ty_5 {
    pub si_addr: *mut ::std::os::raw::c_void,
    pub si_addr_lsb: ::std::os::raw::c_short,
    pub _bounds: siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1 {
    pub _addr_bnd: siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1,
    pub _pkey: __uint32_t,
    _bindgen_union_align: [u64; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1 {
    pub _lower: *mut ::std::os::raw::c_void,
    pub _upper: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct siginfo_t__bindgen_ty_1__bindgen_ty_6 {
    pub si_band: ::std::os::raw::c_long,
    pub si_fd: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct siginfo_t__bindgen_ty_1__bindgen_ty_7 {
    pub _call_addr: *mut ::std::os::raw::c_void,
    pub _syscall: ::std::os::raw::c_int,
    pub _arch: ::std::os::raw::c_uint,
}
pub const SI_ASYNCNL: _bindgen_ty_5 = -60;
pub const SI_TKILL: _bindgen_ty_5 = -6;
pub const SI_SIGIO: _bindgen_ty_5 = -5;
pub const SI_ASYNCIO: _bindgen_ty_5 = -4;
pub const SI_MESGQ: _bindgen_ty_5 = -3;
pub const SI_TIMER: _bindgen_ty_5 = -2;
pub const SI_QUEUE: _bindgen_ty_5 = -1;
pub const SI_USER: _bindgen_ty_5 = 0;
pub const SI_KERNEL: _bindgen_ty_5 = 128;
pub type _bindgen_ty_5 = i32;
pub const ILL_ILLOPC: _bindgen_ty_6 = 1;
pub const ILL_ILLOPN: _bindgen_ty_6 = 2;
pub const ILL_ILLADR: _bindgen_ty_6 = 3;
pub const ILL_ILLTRP: _bindgen_ty_6 = 4;
pub const ILL_PRVOPC: _bindgen_ty_6 = 5;
pub const ILL_PRVREG: _bindgen_ty_6 = 6;
pub const ILL_COPROC: _bindgen_ty_6 = 7;
pub const ILL_BADSTK: _bindgen_ty_6 = 8;
pub type _bindgen_ty_6 = u32;
pub const FPE_INTDIV: _bindgen_ty_7 = 1;
pub const FPE_INTOVF: _bindgen_ty_7 = 2;
pub const FPE_FLTDIV: _bindgen_ty_7 = 3;
pub const FPE_FLTOVF: _bindgen_ty_7 = 4;
pub const FPE_FLTUND: _bindgen_ty_7 = 5;
pub const FPE_FLTRES: _bindgen_ty_7 = 6;
pub const FPE_FLTINV: _bindgen_ty_7 = 7;
pub const FPE_FLTSUB: _bindgen_ty_7 = 8;
pub type _bindgen_ty_7 = u32;
pub const SEGV_MAPERR: _bindgen_ty_8 = 1;
pub const SEGV_ACCERR: _bindgen_ty_8 = 2;
pub const SEGV_BNDERR: _bindgen_ty_8 = 3;
pub const SEGV_PKUERR: _bindgen_ty_8 = 4;
pub type _bindgen_ty_8 = u32;
pub const BUS_ADRALN: _bindgen_ty_9 = 1;
pub const BUS_ADRERR: _bindgen_ty_9 = 2;
pub const BUS_OBJERR: _bindgen_ty_9 = 3;
pub const BUS_MCEERR_AR: _bindgen_ty_9 = 4;
pub const BUS_MCEERR_AO: _bindgen_ty_9 = 5;
pub type _bindgen_ty_9 = u32;
pub const TRAP_BRKPT: _bindgen_ty_10 = 1;
pub const TRAP_TRACE: _bindgen_ty_10 = 2;
pub type _bindgen_ty_10 = u32;
pub const CLD_EXITED: _bindgen_ty_11 = 1;
pub const CLD_KILLED: _bindgen_ty_11 = 2;
pub const CLD_DUMPED: _bindgen_ty_11 = 3;
pub const CLD_TRAPPED: _bindgen_ty_11 = 4;
pub const CLD_STOPPED: _bindgen_ty_11 = 5;
pub const CLD_CONTINUED: _bindgen_ty_11 = 6;
pub type _bindgen_ty_11 = u32;
pub const POLL_IN: _bindgen_ty_12 = 1;
pub const POLL_OUT: _bindgen_ty_12 = 2;
pub const POLL_MSG: _bindgen_ty_12 = 3;
pub const POLL_ERR: _bindgen_ty_12 = 4;
pub const POLL_PRI: _bindgen_ty_12 = 5;
pub const POLL_HUP: _bindgen_ty_12 = 6;
pub type _bindgen_ty_12 = u32;
pub type sigval_t = __sigval_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigevent {
    pub sigev_value: __sigval_t,
    pub sigev_signo: ::std::os::raw::c_int,
    pub sigev_notify: ::std::os::raw::c_int,
    pub _sigev_un: sigevent__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union sigevent__bindgen_ty_1 {
    pub _pad: [::std::os::raw::c_int; 12usize],
    pub _tid: __pid_t,
    pub _sigev_thread: sigevent__bindgen_ty_1__bindgen_ty_1,
    _bindgen_union_align: [u64; 6usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sigevent__bindgen_ty_1__bindgen_ty_1 {
    pub _function: ::core::option::Option<fn(arg1: __sigval_t)>,
    pub _attribute: *mut pthread_attr_t,
}
pub type sigevent_t = sigevent;
pub const SIGEV_SIGNAL: _bindgen_ty_13 = 0;
pub const SIGEV_NONE: _bindgen_ty_13 = 1;
pub const SIGEV_THREAD: _bindgen_ty_13 = 2;
pub const SIGEV_THREAD_ID: _bindgen_ty_13 = 4;
pub type _bindgen_ty_13 = u32;

pub type sighandler_t = __sighandler_t;
pub type sig_t = __sighandler_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigaction {
    pub __sigaction_handler: sigaction__bindgen_ty_1,
    pub sa_mask: __sigset_t,
    pub sa_flags: ::std::os::raw::c_int,
    pub sa_restorer: ::core::option::Option<fn()>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union sigaction__bindgen_ty_1 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: ::core::option::Option<
        fn(arg1: ::std::os::raw::c_int, arg2: *mut siginfo_t, arg3: *mut ::std::os::raw::c_void),
    >,
    _bindgen_union_align: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _fpx_sw_bytes {
    pub magic1: __uint32_t,
    pub extended_size: __uint32_t,
    pub xstate_bv: __uint64_t,
    pub xstate_size: __uint32_t,
    pub __glibc_reserved1: [__uint32_t; 7usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _fpreg {
    pub significand: [::std::os::raw::c_ushort; 4usize],
    pub exponent: ::std::os::raw::c_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _fpxreg {
    pub significand: [::std::os::raw::c_ushort; 4usize],
    pub exponent: ::std::os::raw::c_ushort,
    pub __glibc_reserved1: [::std::os::raw::c_ushort; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmmreg {
    pub element: [__uint32_t; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _fpstate {
    pub cwd: __uint16_t,
    pub swd: __uint16_t,
    pub ftw: __uint16_t,
    pub fop: __uint16_t,
    pub rip: __uint64_t,
    pub rdp: __uint64_t,
    pub mxcsr: __uint32_t,
    pub mxcr_mask: __uint32_t,
    pub _st: [_fpxreg; 8usize],
    pub _xmm: [_xmmreg; 16usize],
    pub __glibc_reserved1: [__uint32_t; 24usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigcontext {
    pub r8: __uint64_t,
    pub r9: __uint64_t,
    pub r10: __uint64_t,
    pub r11: __uint64_t,
    pub r12: __uint64_t,
    pub r13: __uint64_t,
    pub r14: __uint64_t,
    pub r15: __uint64_t,
    pub rdi: __uint64_t,
    pub rsi: __uint64_t,
    pub rbp: __uint64_t,
    pub rbx: __uint64_t,
    pub rdx: __uint64_t,
    pub rax: __uint64_t,
    pub rcx: __uint64_t,
    pub rsp: __uint64_t,
    pub rip: __uint64_t,
    pub eflags: __uint64_t,
    pub cs: ::std::os::raw::c_ushort,
    pub gs: ::std::os::raw::c_ushort,
    pub fs: ::std::os::raw::c_ushort,
    pub __pad0: ::std::os::raw::c_ushort,
    pub err: __uint64_t,
    pub trapno: __uint64_t,
    pub oldmask: __uint64_t,
    pub cr2: __uint64_t,
    pub __bindgen_anon_1: sigcontext__bindgen_ty_1,
    pub __reserved1: [__uint64_t; 8usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union sigcontext__bindgen_ty_1 {
    pub fpstate: *mut _fpstate,
    pub __fpstate_word: __uint64_t,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xsave_hdr {
    pub xstate_bv: __uint64_t,
    pub __glibc_reserved1: [__uint64_t; 2usize],
    pub __glibc_reserved2: [__uint64_t; 5usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ymmh_state {
    pub ymmh_space: [__uint32_t; 64usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _xstate {
    pub fpstate: _fpstate,
    pub xstate_hdr: _xsave_hdr,
    pub ymmh: _ymmh_state,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct stack_t {
    pub ss_sp: *mut ::std::os::raw::c_void,
    pub ss_flags: ::std::os::raw::c_int,
    pub ss_size: usize,
}
pub type greg_t = ::std::os::raw::c_longlong;
pub type gregset_t = [greg_t; 23usize];
pub const REG_R8: _bindgen_ty_14 = 0;
pub const REG_R9: _bindgen_ty_14 = 1;
pub const REG_R10: _bindgen_ty_14 = 2;
pub const REG_R11: _bindgen_ty_14 = 3;
pub const REG_R12: _bindgen_ty_14 = 4;
pub const REG_R13: _bindgen_ty_14 = 5;
pub const REG_R14: _bindgen_ty_14 = 6;
pub const REG_R15: _bindgen_ty_14 = 7;
pub const REG_RDI: _bindgen_ty_14 = 8;
pub const REG_RSI: _bindgen_ty_14 = 9;
pub const REG_RBP: _bindgen_ty_14 = 10;
pub const REG_RBX: _bindgen_ty_14 = 11;
pub const REG_RDX: _bindgen_ty_14 = 12;
pub const REG_RAX: _bindgen_ty_14 = 13;
pub const REG_RCX: _bindgen_ty_14 = 14;
pub const REG_RSP: _bindgen_ty_14 = 15;
pub const REG_RIP: _bindgen_ty_14 = 16;
pub const REG_EFL: _bindgen_ty_14 = 17;
pub const REG_CSGSFS: _bindgen_ty_14 = 18;
pub const REG_ERR: _bindgen_ty_14 = 19;
pub const REG_TRAPNO: _bindgen_ty_14 = 20;
pub const REG_OLDMASK: _bindgen_ty_14 = 21;
pub const REG_CR2: _bindgen_ty_14 = 22;
pub type _bindgen_ty_14 = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _libc_fpxreg {
    pub significand: [::std::os::raw::c_ushort; 4usize],
    pub exponent: ::std::os::raw::c_ushort,
    pub __glibc_reserved1: [::std::os::raw::c_ushort; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _libc_xmmreg {
    pub element: [__uint32_t; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _libc_fpstate {
    pub cwd: __uint16_t,
    pub swd: __uint16_t,
    pub ftw: __uint16_t,
    pub fop: __uint16_t,
    pub rip: __uint64_t,
    pub rdp: __uint64_t,
    pub mxcsr: __uint32_t,
    pub mxcr_mask: __uint32_t,
    pub _st: [_libc_fpxreg; 8usize],
    pub _xmm: [_libc_xmmreg; 16usize],
    pub __glibc_reserved1: [__uint32_t; 24usize],
}
pub type fpregset_t = *mut _libc_fpstate;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mcontext_t {
    pub gregs: gregset_t,
    pub fpregs: fpregset_t,
    pub __reserved1: [::std::os::raw::c_ulonglong; 8usize],
}
#[repr(C)]
// #[derive(Debug, Copy, Clone)]
#[derive(Copy, Clone)]
pub struct ucontext_t {
    pub uc_flags: ::std::os::raw::c_ulong,
    pub uc_link: *mut ucontext_t,
    pub uc_stack: stack_t,
    pub uc_mcontext: mcontext_t,
    pub uc_sigmask: sigset_t,
    pub __fpregs_mem: _libc_fpstate,
    pub __ssp: [::std::os::raw::c_ulonglong; 4usize],
}

pub const SS_ONSTACK: _bindgen_ty_15 = 1;
pub const SS_DISABLE: _bindgen_ty_15 = 2;
pub type _bindgen_ty_15 = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sigstack {
    pub ss_sp: *mut ::std::os::raw::c_void,
    pub ss_onstack: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: ::std::os::raw::c_uint,
    pub __writers: ::std::os::raw::c_uint,
    pub __wrphase_futex: ::std::os::raw::c_uint,
    pub __writers_futex: ::std::os::raw::c_uint,
    pub __pad3: ::std::os::raw::c_uint,
    pub __pad4: ::std::os::raw::c_uint,
    pub __cur_writer: ::std::os::raw::c_int,
    pub __shared: ::std::os::raw::c_int,
    pub __rwelision: ::std::os::raw::c_schar,
    pub __pad1: [::std::os::raw::c_uchar; 7usize],
    pub __pad2: ::std::os::raw::c_ulong,
    pub __flags: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_mutex_s {
    pub __lock: ::std::os::raw::c_int,
    pub __count: ::std::os::raw::c_uint,
    pub __owner: ::std::os::raw::c_int,
    pub __nusers: ::std::os::raw::c_uint,
    pub __kind: ::std::os::raw::c_int,
    pub __spins: ::std::os::raw::c_short,
    pub __elision: ::std::os::raw::c_short,
    pub __list: __pthread_list_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pthread_cond_s {
    pub __bindgen_anon_1: __pthread_cond_s__bindgen_ty_1,
    pub __bindgen_anon_2: __pthread_cond_s__bindgen_ty_2,
    pub __g_refs: [::std::os::raw::c_uint; 2usize],
    pub __g_size: [::std::os::raw::c_uint; 2usize],
    pub __g1_orig_size: ::std::os::raw::c_uint,
    pub __wrefs: ::std::os::raw::c_uint,
    pub __g_signals: [::std::os::raw::c_uint; 2usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __pthread_cond_s__bindgen_ty_1 {
    pub __wseq: ::std::os::raw::c_ulonglong,
    pub __wseq32: __pthread_cond_s__bindgen_ty_1__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 {
    pub __low: ::std::os::raw::c_uint,
    pub __high: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __pthread_cond_s__bindgen_ty_2 {
    pub __g1_start: ::std::os::raw::c_ulonglong,
    pub __g1_start32: __pthread_cond_s__bindgen_ty_2__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 {
    pub __low: ::std::os::raw::c_uint,
    pub __high: ::std::os::raw::c_uint,
}
pub type pthread_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutexattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
    _bindgen_union_align: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_condattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
    _bindgen_union_align: u32,
}
pub type pthread_key_t = ::std::os::raw::c_uint;
pub type pthread_once_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_attr_t {
    pub __size: [::std::os::raw::c_char; 56usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: [u64; 7usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::std::os::raw::c_char; 40usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: [u64; 5usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [::std::os::raw::c_char; 48usize],
    pub __align: ::std::os::raw::c_longlong,
    _bindgen_union_align: [u64; 6usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [::std::os::raw::c_char; 56usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: [u64; 7usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlockattr_t {
    pub __size: [::std::os::raw::c_char; 8usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: u64,
}
pub type pthread_spinlock_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrier_t {
    pub __size: [::std::os::raw::c_char; 32usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: [u64; 4usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrierattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
    _bindgen_union_align: u32,
}

pub type int_least8_t = __int_least8_t;
pub type int_least16_t = __int_least16_t;
pub type int_least32_t = __int_least32_t;
pub type int_least64_t = __int_least64_t;
pub type uint_least8_t = __uint_least8_t;
pub type uint_least16_t = __uint_least16_t;
pub type uint_least32_t = __uint_least32_t;
pub type uint_least64_t = __uint_least64_t;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_ulong;
pub type uint_fast32_t = ::std::os::raw::c_ulong;
pub type uint_fast64_t = ::std::os::raw::c_ulong;

pub type uintmax_t = __uintmax_t;
pub const r_instruction_r_output_direction: r_instruction = 0;
pub const r_instruction_r_input_direction: r_instruction = 1;
pub const r_instruction_r_inputa_direction: r_instruction = 2;
pub const r_instruction_r_appending_to: r_instruction = 3;
pub const r_instruction_r_reading_until: r_instruction = 4;
pub const r_instruction_r_reading_string: r_instruction = 5;
pub const r_instruction_r_duplicating_input: r_instruction = 6;
pub const r_instruction_r_duplicating_output: r_instruction = 7;
pub const r_instruction_r_deblank_reading_until: r_instruction = 8;
pub const r_instruction_r_close_this: r_instruction = 9;
pub const r_instruction_r_err_and_out: r_instruction = 10;
pub const r_instruction_r_input_output: r_instruction = 11;
pub const r_instruction_r_output_force: r_instruction = 12;
pub const r_instruction_r_duplicating_input_word: r_instruction = 13;
pub const r_instruction_r_duplicating_output_word: r_instruction = 14;
pub const r_instruction_r_move_input: r_instruction = 15;
pub const r_instruction_r_move_output: r_instruction = 16;
pub const r_instruction_r_move_input_word: r_instruction = 17;
pub const r_instruction_r_move_output_word: r_instruction = 18;
pub const r_instruction_r_append_err_and_out: r_instruction = 19;
pub type r_instruction = u32;
pub const command_type_cm_for: command_type = 0;
pub const command_type_cm_case: command_type = 1;
pub const command_type_cm_while: command_type = 2;
pub const command_type_cm_if: command_type = 3;
pub const command_type_cm_simple: command_type = 4;
pub const command_type_cm_select: command_type = 5;
pub const command_type_cm_connection: command_type = 6;
pub const command_type_cm_function_def: command_type = 7;
pub const command_type_cm_until: command_type = 8;
pub const command_type_cm_group: command_type = 9;
pub const command_type_cm_arith: command_type = 10;
pub const command_type_cm_cond: command_type = 11;
pub const command_type_cm_arith_for: command_type = 12;
pub const command_type_cm_subshell: command_type = 13;
pub const command_type_cm_coproc: command_type = 14;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct element {
    pub word: *mut WordDesc,
    pub redirect: *mut REDIRECT,
}
pub type ELEMENT = element;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct coproc {
    pub c_name: *mut ::std::os::raw::c_char,
    pub c_pid: pid_t,
    pub c_rfd: ::std::os::raw::c_int,
    pub c_wfd: ::std::os::raw::c_int,
    pub c_rsave: ::std::os::raw::c_int,
    pub c_wsave: ::std::os::raw::c_int,
    pub c_flags: ::std::os::raw::c_int,
    pub c_status: ::std::os::raw::c_int,
    pub c_lock: ::std::os::raw::c_int,
}
pub type Coproc = coproc;

pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type u_long = __u_long;
pub type quad_t = __quad_t;
pub type u_quad_t = __u_quad_t;
pub type fsid_t = __fsid_t;
pub type loff_t = __loff_t;
pub type ino_t = __ino_t;
pub type ino64_t = __ino64_t;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type nlink_t = __nlink_t;
pub type id_t = __id_t;
pub type daddr_t = __daddr_t;
pub type caddr_t = __caddr_t;
pub type key_t = __key_t;
pub type clock_t = __clock_t;
pub type clockid_t = __clockid_t;

pub type timer_t = __timer_t;
pub type suseconds_t = __suseconds_t;
pub type ulong = ::std::os::raw::c_ulong;
pub type ushort = ::std::os::raw::c_ushort;
pub type uint = ::std::os::raw::c_uint;
pub type u_int8_t = ::std::os::raw::c_uchar;
pub type u_int16_t = ::std::os::raw::c_ushort;
pub type u_int32_t = ::std::os::raw::c_uint;
pub type u_int64_t = ::std::os::raw::c_ulong;
pub type register_t = ::std::os::raw::c_long;

pub type __fd_mask = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16usize],
}
pub type fd_mask = __fd_mask;

pub type blksize_t = __blksize_t;
pub type blkcnt_t = __blkcnt_t;
pub type fsblkcnt_t = __fsblkcnt_t;
pub type fsfilcnt_t = __fsfilcnt_t;
pub type blkcnt64_t = __blkcnt64_t;
pub type fsblkcnt64_t = __fsblkcnt64_t;
pub type fsfilcnt64_t = __fsfilcnt64_t;
pub type __gwchar_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct imaxdiv_t {
    pub quot: ::std::os::raw::c_long,
    pub rem: ::std::os::raw::c_long,
}

pub type _bindgen_ty_16 = u32;

pub const __itimer_which_ITIMER_REAL: __itimer_which = 0;
pub const __itimer_which_ITIMER_VIRTUAL: __itimer_which = 1;
pub const __itimer_which_ITIMER_PROF: __itimer_which = 2;
pub type __itimer_which = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct itimerval {
    pub it_interval: timeval,
    pub it_value: timeval,
}

pub use self::__itimer_which as __itimer_which_t;

pub const __rlimit_resource_RLIMIT_CPU: __rlimit_resource = 0;
pub const __rlimit_resource_RLIMIT_FSIZE: __rlimit_resource = 1;
pub const __rlimit_resource_RLIMIT_DATA: __rlimit_resource = 2;
pub const __rlimit_resource_RLIMIT_STACK: __rlimit_resource = 3;
pub const __rlimit_resource_RLIMIT_CORE: __rlimit_resource = 4;
pub const __rlimit_resource___RLIMIT_RSS: __rlimit_resource = 5;
pub const __rlimit_resource_RLIMIT_NOFILE: __rlimit_resource = 7;
pub const __rlimit_resource___RLIMIT_OFILE: __rlimit_resource = 7;
pub const __rlimit_resource_RLIMIT_AS: __rlimit_resource = 9;
pub const __rlimit_resource___RLIMIT_NPROC: __rlimit_resource = 6;
pub const __rlimit_resource___RLIMIT_MEMLOCK: __rlimit_resource = 8;
pub const __rlimit_resource___RLIMIT_LOCKS: __rlimit_resource = 10;
pub const __rlimit_resource___RLIMIT_SIGPENDING: __rlimit_resource = 11;
pub const __rlimit_resource___RLIMIT_MSGQUEUE: __rlimit_resource = 12;
pub const __rlimit_resource___RLIMIT_NICE: __rlimit_resource = 13;
pub const __rlimit_resource___RLIMIT_RTPRIO: __rlimit_resource = 14;
pub const __rlimit_resource___RLIMIT_RTTIME: __rlimit_resource = 15;
pub const __rlimit_resource___RLIMIT_NLIMITS: __rlimit_resource = 16;
pub const __rlimit_resource___RLIM_NLIMITS: __rlimit_resource = 16;
pub type __rlimit_resource = u32;

pub type rlim64_t = __rlim64_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rlimit64 {
    pub rlim_cur: rlim64_t,
    pub rlim_max: rlim64_t,
}
pub const __rusage_who_RUSAGE_SELF: __rusage_who = 0;
pub const __rusage_who_RUSAGE_CHILDREN: __rusage_who = -1;
pub const __rusage_who_RUSAGE_THREAD: __rusage_who = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rusage {
    pub ru_utime: timeval,
    pub ru_stime: timeval,
    pub __bindgen_anon_1: rusage__bindgen_ty_1,
    pub __bindgen_anon_2: rusage__bindgen_ty_2,
    pub __bindgen_anon_3: rusage__bindgen_ty_3,
    pub __bindgen_anon_4: rusage__bindgen_ty_4,
    pub __bindgen_anon_5: rusage__bindgen_ty_5,
    pub __bindgen_anon_6: rusage__bindgen_ty_6,
    pub __bindgen_anon_7: rusage__bindgen_ty_7,
    pub __bindgen_anon_8: rusage__bindgen_ty_8,
    pub __bindgen_anon_9: rusage__bindgen_ty_9,
    pub __bindgen_anon_10: rusage__bindgen_ty_10,
    pub __bindgen_anon_11: rusage__bindgen_ty_11,
    pub __bindgen_anon_12: rusage__bindgen_ty_12,
    pub __bindgen_anon_13: rusage__bindgen_ty_13,
    pub __bindgen_anon_14: rusage__bindgen_ty_14,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union rusage__bindgen_ty_1 {
    pub ru_maxrss: ::std::os::raw::c_long,
    pub __ru_maxrss_word: __syscall_slong_t,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union rusage__bindgen_ty_2 {
    pub ru_ixrss: ::std::os::raw::c_long,
    pub __ru_ixrss_word: __syscall_slong_t,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union rusage__bindgen_ty_3 {
    pub ru_idrss: ::std::os::raw::c_long,
    pub __ru_idrss_word: __syscall_slong_t,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union rusage__bindgen_ty_4 {
    pub ru_isrss: ::std::os::raw::c_long,
    pub __ru_isrss_word: __syscall_slong_t,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union rusage__bindgen_ty_5 {
    pub ru_minflt: ::std::os::raw::c_long,
    pub __ru_minflt_word: __syscall_slong_t,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union rusage__bindgen_ty_6 {
    pub ru_majflt: ::std::os::raw::c_long,
    pub __ru_majflt_word: __syscall_slong_t,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union rusage__bindgen_ty_7 {
    pub ru_nswap: ::std::os::raw::c_long,
    pub __ru_nswap_word: __syscall_slong_t,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union rusage__bindgen_ty_8 {
    pub ru_inblock: ::std::os::raw::c_long,
    pub __ru_inblock_word: __syscall_slong_t,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union rusage__bindgen_ty_9 {
    pub ru_oublock: ::std::os::raw::c_long,
    pub __ru_oublock_word: __syscall_slong_t,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union rusage__bindgen_ty_10 {
    pub ru_msgsnd: ::std::os::raw::c_long,
    pub __ru_msgsnd_word: __syscall_slong_t,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union rusage__bindgen_ty_11 {
    pub ru_msgrcv: ::std::os::raw::c_long,
    pub __ru_msgrcv_word: __syscall_slong_t,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union rusage__bindgen_ty_12 {
    pub ru_nsignals: ::std::os::raw::c_long,
    pub __ru_nsignals_word: __syscall_slong_t,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union rusage__bindgen_ty_13 {
    pub ru_nvcsw: ::std::os::raw::c_long,
    pub __ru_nvcsw_word: __syscall_slong_t,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union rusage__bindgen_ty_14 {
    pub ru_nivcsw: ::std::os::raw::c_long,
    pub __ru_nivcsw_word: __syscall_slong_t,
    _bindgen_union_align: u64,
}
pub const __priority_which_PRIO_PROCESS: __priority_which = 0;
pub const __priority_which_PRIO_PGRP: __priority_which = 1;
pub const __priority_which_PRIO_USER: __priority_which = 2;
pub type __priority_which = u32;

pub use self::__priority_which as __priority_which_t;
pub use self::__rlimit_resource as __rlimit_resource_t;
pub use self::__rusage_who as __rusage_who_t;

pub const idtype_t_P_ALL: idtype_t = 0;
pub const idtype_t_P_PID: idtype_t = 1;
pub const idtype_t_P_PGID: idtype_t = 2;
pub type idtype_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct div_t {
    pub quot: ::std::os::raw::c_int,
    pub rem: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ldiv_t {
    pub quot: ::std::os::raw::c_long,
    pub rem: ::std::os::raw::c_long,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lldiv_t {
    pub quot: ::std::os::raw::c_longlong,
    pub rem: ::std::os::raw::c_longlong,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct random_data {
    pub fptr: *mut i32,
    pub rptr: *mut i32,
    pub state: *mut i32,
    pub rand_type: ::std::os::raw::c_int,
    pub rand_deg: ::std::os::raw::c_int,
    pub rand_sep: ::std::os::raw::c_int,
    pub end_ptr: *mut i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drand48_data {
    pub __x: [::std::os::raw::c_ushort; 3usize],
    pub __old_x: [::std::os::raw::c_ushort; 3usize],
    pub __c: ::std::os::raw::c_ushort,
    pub __init: ::std::os::raw::c_ushort,
    pub __a: ::std::os::raw::c_ulonglong,
}

pub type comparison_fn_t = __compar_fn_t;
pub type __compar_d_fn_t = ::core::option::Option<
    fn(
        arg1: *const ::std::os::raw::c_void,
        arg2: *const ::std::os::raw::c_void,
        arg3: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;

pub type CPFunction = ::core::option::Option<fn() -> *mut ::std::os::raw::c_char>;
pub type CPPFunction = ::core::option::Option<fn() -> *mut *mut ::std::os::raw::c_char>;
pub type sh_intfunc_t =
    ::core::option::Option<fn(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int>;
pub type sh_ivoidfunc_t = ::core::option::Option<fn() -> ::std::os::raw::c_int>;
pub type sh_icpfunc_t =
    ::core::option::Option<fn(arg1: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int>;
pub type sh_icppfunc_t =
    ::core::option::Option<fn(arg1: *mut *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int>;
pub type sh_iptrfunc_t =
    ::core::option::Option<fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int>;
pub type sh_voidfunc_t = ::core::option::Option<fn()>;
pub type sh_vintfunc_t = ::core::option::Option<fn(arg1: ::std::os::raw::c_int)>;
pub type sh_vcpfunc_t = ::core::option::Option<fn(arg1: *mut ::std::os::raw::c_char)>;
pub type sh_vcppfunc_t = ::core::option::Option<fn(arg1: *mut *mut ::std::os::raw::c_char)>;
pub type sh_vptrfunc_t = ::core::option::Option<fn(arg1: *mut ::std::os::raw::c_void)>;
pub type sh_wdesc_func_t = ::core::option::Option<fn(arg1: *mut WordDesc) -> ::std::os::raw::c_int>;
pub type sh_wlist_func_t = ::core::option::Option<fn(arg1: *mut WordList) -> ::std::os::raw::c_int>;
pub type sh_glist_func_t =
    ::core::option::Option<fn(arg1: *mut GENERIC_LIST) -> ::std::os::raw::c_int>;

pub type sh_msg_func_t = ::core::option::Option<
    unsafe extern "C" fn(arg1: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int,
>;
pub type sh_vmsg_func_t =
    ::core::option::Option<unsafe extern "C" fn(arg1: *const ::std::os::raw::c_char, ...)>;

pub type sh_free_func_t = ::core::option::Option<fn(arg1: *mut ::std::os::raw::c_void)>;

pub type sh_assign_func_t =
    ::core::option::Option<fn(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int>;

pub type sh_load_func_t =
    ::core::option::Option<fn(arg1: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int>;
pub type sh_unload_func_t = ::core::option::Option<fn(arg1: *mut ::std::os::raw::c_char)>;

pub const atype_array_assoc: atype = 1;

pub type sh_strlist_map_func_t =
    ::core::option::Option<fn(arg1: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct var_context {
    pub name: *mut ::std::os::raw::c_char,
    pub scope: ::std::os::raw::c_int,
    pub flags: ::std::os::raw::c_int,
    pub up: *mut var_context,
    pub down: *mut var_context,
    pub table: *mut HASH_TABLE,
}
pub type VAR_CONTEXT = var_context;

#[repr(C)]
#[derive(Copy, Clone)]
pub union _value {
    pub s: *mut ::std::os::raw::c_char,
    pub i: intmax_t,
    pub f: *mut COMMAND,
    pub a: *mut ARRAY,
    pub h: *mut HASH_TABLE,
    pub d: f64,
    pub ld: f64,
    pub v: *mut variable,
    pub opaque: *mut ::std::os::raw::c_void,
    _bindgen_union_align: [u8; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _vlist {
    pub list: *mut *mut SHELL_VAR,
    pub list_size: usize,
    pub list_len: usize,
}
pub type VARLIST = _vlist;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _G_fpos_t {
    pub __pos: __off_t,
    pub __state: __mbstate_t,
}
pub type __fpos_t = _G_fpos_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _G_fpos64_t {
    pub __pos: __off64_t,
    pub __state: __mbstate_t,
}
pub type __fpos64_t = _G_fpos64_t;

pub type cookie_read_function_t = ::core::option::Option<
    fn(
        __cookie: *mut ::std::os::raw::c_void,
        __buf: *mut ::std::os::raw::c_char,
        __nbytes: usize,
    ) -> __ssize_t,
>;
pub type cookie_write_function_t = ::core::option::Option<
    fn(
        __cookie: *mut ::std::os::raw::c_void,
        __buf: *const ::std::os::raw::c_char,
        __nbytes: usize,
    ) -> __ssize_t,
>;
pub type cookie_seek_function_t = ::core::option::Option<
    fn(
        __cookie: *mut ::std::os::raw::c_void,
        __pos: *mut __off64_t,
        __w: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type cookie_close_function_t =
    ::core::option::Option<fn(__cookie: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_cookie_io_functions_t {
    pub read: cookie_read_function_t,
    pub write: cookie_write_function_t,
    pub seek: cookie_seek_function_t,
    pub close: cookie_close_function_t,
}
pub type cookie_io_functions_t = _IO_cookie_io_functions_t;
pub type fpos_t = __fpos_t;
pub type fpos64_t = __fpos64_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct obstack {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _pathdata {
    pub path: *mut ::std::os::raw::c_char,
    pub flags: ::std::os::raw::c_int,
}
pub type PATH_DATA = _pathdata;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pipeline_saver {
    pub pipeline: *mut process,
    pub next: *mut pipeline_saver,
}
#[no_mangle]
pub static mut wait_intr_flag: ::std::os::raw::c_int = 0;
pub const JOB_STATE_JNONE: JOB_STATE = -1;
pub const JOB_STATE_JRUNNING: JOB_STATE = 1;
pub const JOB_STATE_JSTOPPED: JOB_STATE = 2;
pub const JOB_STATE_JDEAD: JOB_STATE = 4;
pub const JOB_STATE_JMIXED: JOB_STATE = 8;
pub type JOB_STATE = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct job {
    pub wd: *mut ::std::os::raw::c_char,
    pub pipe: *mut PROCESS,
    pub pgrp: pid_t,
    pub state: JOB_STATE,
    pub flags: ::std::os::raw::c_int,
    pub deferred: *mut COMMAND,
    pub j_cleanup: sh_vptrfunc_t,
    pub cleanarg: *mut ::std::os::raw::c_void,
}
pub type JOB = job;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jobstats {
    pub c_childmax: ::std::os::raw::c_long,
    pub c_living: ::std::os::raw::c_int,
    pub c_reaped: ::std::os::raw::c_int,
    pub c_injobs: ::std::os::raw::c_int,
    pub c_totforked: ::std::os::raw::c_int,
    pub c_totreaped: ::std::os::raw::c_int,
    pub j_jobslots: ::std::os::raw::c_int,
    pub j_lastj: ::std::os::raw::c_int,
    pub j_firstj: ::std::os::raw::c_int,
    pub j_njobs: ::std::os::raw::c_int,
    pub j_ndead: ::std::os::raw::c_int,
    pub j_current: ::std::os::raw::c_int,
    pub j_previous: ::std::os::raw::c_int,
    pub j_lastmade: *mut JOB,
    pub j_lastasync: *mut JOB,
}
pub type ps_index_t = pid_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pidstat {
    pub bucket_next: ps_index_t,
    pub bucket_prev: ps_index_t,
    pub pid: pid_t,
    pub status: ::std::os::raw::c_short,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgpids {
    pub storage: *mut pidstat,
    pub head: ps_index_t,
    pub nalloc: ps_index_t,
    pub npid: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct procstat {
    pub pid: pid_t,
    pub status: ::std::os::raw::c_short,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct procchain {
    pub head: *mut PROCESS,
    pub end: *mut PROCESS,
    pub nproc: ::std::os::raw::c_int,
}

pub type jmp_buf = [__jmp_buf_tag; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _sh_input_line_state_t {
    pub input_line: *mut ::std::os::raw::c_char,
    pub input_line_index: usize,
    pub input_line_size: usize,
    pub input_line_len: usize,
    pub input_property: *mut ::std::os::raw::c_char,
    pub input_propsize: usize,
}
pub type sh_input_line_state_t = _sh_input_line_state_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct func_array_state {
    pub funcname_a: *mut ARRAY,
    pub funcname_v: *mut SHELL_VAR,
    pub source_a: *mut ARRAY,
    pub source_v: *mut SHELL_VAR,
    pub lineno_a: *mut ARRAY,
    pub lineno_v: *mut SHELL_VAR,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct execstate {
    pub pid: pid_t,
    pub subshell_env: ::std::os::raw::c_int,
}

pub const stream_type_st_none: stream_type = 0;
pub const stream_type_st_stdin: stream_type = 1;
pub const stream_type_st_stream: stream_type = 2;
pub const stream_type_st_string: stream_type = 3;
pub const stream_type_st_bstream: stream_type = 4;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct builtin {
    pub name: *mut ::std::os::raw::c_char,
    pub function: Option<sh_builtin_func_t>,
    pub flags: ::std::os::raw::c_int,
    pub long_doc: *const *mut ::std::os::raw::c_char,
    pub short_doc: *const ::std::os::raw::c_char,
    pub handle: *mut ::std::os::raw::c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dstack {
    pub delimiters: *mut ::std::os::raw::c_char,
    pub delimiter_depth: ::std::os::raw::c_int,
    pub delimiter_space: ::std::os::raw::c_int,
}

pub const yytokentype_IF: yytokentype = 258;
pub const yytokentype_THEN: yytokentype = 259;
pub const yytokentype_ELSE: yytokentype = 260;
pub const yytokentype_ELIF: yytokentype = 261;
pub const yytokentype_FI: yytokentype = 262;
pub const yytokentype_CASE: yytokentype = 263;
pub const yytokentype_ESAC: yytokentype = 264;
pub const yytokentype_FOR: yytokentype = 265;
pub const yytokentype_SELECT: yytokentype = 266;
pub const yytokentype_WHILE: yytokentype = 267;
pub const yytokentype_UNTIL: yytokentype = 268;
pub const yytokentype_DO: yytokentype = 269;
pub const yytokentype_DONE: yytokentype = 270;
pub const yytokentype_FUNCTION: yytokentype = 271;
pub const yytokentype_COPROC: yytokentype = 272;
pub const yytokentype_COND_START: yytokentype = 273;
pub const yytokentype_COND_END: yytokentype = 274;
pub const yytokentype_COND_ERROR: yytokentype = 275;
pub const yytokentype_IN: yytokentype = 276;
pub const yytokentype_BANG: yytokentype = 277;
pub const yytokentype_TIME: yytokentype = 278;
pub const yytokentype_TIMEOPT: yytokentype = 279;
pub const yytokentype_TIMEIGN: yytokentype = 280;
pub const yytokentype_WORD: yytokentype = 281;
pub const yytokentype_ASSIGNMENT_WORD: yytokentype = 282;
pub const yytokentype_REDIR_WORD: yytokentype = 283;
pub const yytokentype_NUMBER: yytokentype = 284;
pub const yytokentype_ARITH_CMD: yytokentype = 285;
pub const yytokentype_ARITH_FOR_EXPRS: yytokentype = 286;
pub const yytokentype_COND_CMD: yytokentype = 287;
pub const yytokentype_AND_AND: yytokentype = 288;
pub const yytokentype_OR_OR: yytokentype = 289;
pub const yytokentype_GREATER_GREATER: yytokentype = 290;
pub const yytokentype_LESS_LESS: yytokentype = 291;
pub const yytokentype_LESS_AND: yytokentype = 292;
pub const yytokentype_LESS_LESS_LESS: yytokentype = 293;
pub const yytokentype_GREATER_AND: yytokentype = 294;
pub const yytokentype_SEMI_SEMI: yytokentype = 295;
pub const yytokentype_SEMI_AND: yytokentype = 296;
pub const yytokentype_SEMI_SEMI_AND: yytokentype = 297;
pub const yytokentype_LESS_LESS_MINUS: yytokentype = 298;
pub const yytokentype_AND_GREATER: yytokentype = 299;
pub const yytokentype_AND_GREATER_GREATER: yytokentype = 300;
pub const yytokentype_LESS_GREATER: yytokentype = 301;
pub const yytokentype_GREATER_BAR: yytokentype = 302;
pub const yytokentype_BAR_AND: yytokentype = 303;
pub const yytokentype_yacc_EOF: yytokentype = 304;
pub type yytokentype = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub union YYSTYPE {
    pub word: *mut WordDesc,
    pub number: ::std::os::raw::c_int,
    pub WordList: *mut WordList,
    pub command: *mut COMMAND,
    pub redirect: *mut REDIRECT,
    pub element: ELEMENT,
    pub pattern: *mut PATTERN_LIST,
    _bindgen_union_align: [u64; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct flock {
    pub l_type: ::std::os::raw::c_short,
    pub l_whence: ::std::os::raw::c_short,
    pub l_start: __off_t,
    pub l_len: __off_t,
    pub l_pid: __pid_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct flock64 {
    pub l_type: ::std::os::raw::c_short,
    pub l_whence: ::std::os::raw::c_short,
    pub l_start: __off64_t,
    pub l_len: __off64_t,
    pub l_pid: __pid_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iovec {
    pub iov_base: *mut ::std::os::raw::c_void,
    pub iov_len: usize,
}
pub const __pid_type_F_OWNER_TID: __pid_type = 0;
pub const __pid_type_F_OWNER_PID: __pid_type = 1;
pub const __pid_type_F_OWNER_PGRP: __pid_type = 2;
pub const __pid_type_F_OWNER_GID: __pid_type = 2;
pub type __pid_type = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct f_owner_ex {
    pub type_: __pid_type,
    pub pid: __pid_t,
}
#[repr(C)]
#[derive(Debug)]
pub struct file_handle {
    pub handle_bytes: ::std::os::raw::c_uint,
    pub handle_type: ::std::os::raw::c_int,
    pub f_handle: __IncompleteArrayField<::std::os::raw::c_uchar>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct stat64 {
    pub st_dev: __dev_t,
    pub st_ino: __ino64_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::std::os::raw::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt64_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct statx_timestamp {
    pub tv_sec: __int64_t,
    pub tv_nsec: __uint32_t,
    pub __statx_timestamp_pad1: [__int32_t; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct statx {
    pub stx_mask: __uint32_t,
    pub stx_blksize: __uint32_t,
    pub stx_attributes: __uint64_t,
    pub stx_nlink: __uint32_t,
    pub stx_uid: __uint32_t,
    pub stx_gid: __uint32_t,
    pub stx_mode: __uint16_t,
    pub __statx_pad1: [__uint16_t; 1usize],
    pub stx_ino: __uint64_t,
    pub stx_size: __uint64_t,
    pub stx_blocks: __uint64_t,
    pub stx_attributes_mask: __uint64_t,
    pub stx_atime: statx_timestamp,
    pub stx_btime: statx_timestamp,
    pub stx_ctime: statx_timestamp,
    pub stx_mtime: statx_timestamp,
    pub stx_rdev_major: __uint32_t,
    pub stx_rdev_minor: __uint32_t,
    pub stx_dev_major: __uint32_t,
    pub stx_dev_minor: __uint32_t,
    pub __statx_pad2: [__uint64_t; 14usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timex {
    pub modes: ::std::os::raw::c_uint,
    pub offset: __syscall_slong_t,
    pub freq: __syscall_slong_t,
    pub maxerror: __syscall_slong_t,
    pub esterror: __syscall_slong_t,
    pub status: ::std::os::raw::c_int,
    pub constant: __syscall_slong_t,
    pub precision: __syscall_slong_t,
    pub tolerance: __syscall_slong_t,
    pub time: timeval,
    pub tick: __syscall_slong_t,
    pub ppsfreq: __syscall_slong_t,
    pub jitter: __syscall_slong_t,
    pub shift: ::std::os::raw::c_int,
    pub stabil: __syscall_slong_t,
    pub jitcnt: __syscall_slong_t,
    pub calcnt: __syscall_slong_t,
    pub errcnt: __syscall_slong_t,
    pub stbcnt: __syscall_slong_t,
    pub tai: ::std::os::raw::c_int,
    pub _bitfield_1: [i32; 11],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tm {
    pub tm_sec: ::std::os::raw::c_int,
    pub tm_min: ::std::os::raw::c_int,
    pub tm_hour: ::std::os::raw::c_int,
    pub tm_mday: ::std::os::raw::c_int,
    pub tm_mon: ::std::os::raw::c_int,
    pub tm_year: ::std::os::raw::c_int,
    pub tm_wday: ::std::os::raw::c_int,
    pub tm_yday: ::std::os::raw::c_int,
    pub tm_isdst: ::std::os::raw::c_int,
    pub tm_gmtoff: ::std::os::raw::c_long,
    pub tm_zone: *const ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tms {
    pub tms_utime: clock_t,
    pub tms_stime: clock_t,
    pub tms_cutime: clock_t,
    pub tms_cstime: clock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: ::std::os::raw::c_ushort,
    pub d_type: ::std::os::raw::c_uchar,
    pub d_name: [::std::os::raw::c_char; 256usize],
}
pub type __ino64_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dirent64 {
    pub d_ino: __ino64_t,
    pub d_off: __off64_t,
    pub d_reclen: ::std::os::raw::c_ushort,
    pub d_type: ::std::os::raw::c_uchar,
    pub d_name: [::std::os::raw::c_char; 256usize],
}
pub const DT_UNKNOWN: _bindgen_ty_17 = 0;
pub const DT_FIFO: _bindgen_ty_17 = 1;
pub const DT_CHR: _bindgen_ty_17 = 2;
pub const DT_DIR: _bindgen_ty_17 = 4;
pub const DT_BLK: _bindgen_ty_17 = 6;
pub const DT_REG: _bindgen_ty_17 = 8;
pub const DT_LNK: _bindgen_ty_17 = 10;
pub const DT_SOCK: _bindgen_ty_17 = 12;
pub const DT_WHT: _bindgen_ty_17 = 14;
pub type _bindgen_ty_17 = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __dirstream {
    pub _unused: [u8; 0],
}
pub type DIR = __dirstream;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32usize],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union wait {
    pub w_status: ::std::os::raw::c_int,
    pub w_T: wait__bindgen_ty_1,
    pub w_S: wait__bindgen_ty_2,
    _bindgen_union_align: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wait__bindgen_ty_1 {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u16>,
    pub __bindgen_align: [u16; 0usize],
}
impl wait__bindgen_ty_1 {
    #[inline]
    pub fn w_Termsig(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 7u8) as u16) }
    }
    #[inline]
    pub fn set_w_Termsig(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 7u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Coredump(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set_w_Coredump(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::core::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Retcode(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(8usize, 8u8) as u16) }
    }
    #[inline]
    pub fn set_w_Retcode(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::core::mem::transmute(val);
            self._bitfield_1.set(8usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Fill1(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as u16) }
    }
    #[inline]
    pub fn set_w_Fill1(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::core::mem::transmute(val);
            self._bitfield_1.set(16usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        w_Termsig: ::std::os::raw::c_ushort,
        w_Coredump: ::std::os::raw::c_ushort,
        w_Retcode: ::std::os::raw::c_ushort,
        w_Fill1: ::std::os::raw::c_ushort,
    ) -> __BindgenBitfieldUnit<[u8; 4usize], u16> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize], u16> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 7u8, {
            let w_Termsig: u16 = unsafe { ::core::mem::transmute(w_Termsig) };
            w_Termsig as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let w_Coredump: u16 = unsafe { ::core::mem::transmute(w_Coredump) };
            w_Coredump as u64
        });
        __bindgen_bitfield_unit.set(8usize, 8u8, {
            let w_Retcode: u16 = unsafe { ::core::mem::transmute(w_Retcode) };
            w_Retcode as u64
        });
        __bindgen_bitfield_unit.set(16usize, 16u8, {
            let w_Fill1: u16 = unsafe { ::core::mem::transmute(w_Fill1) };
            w_Fill1 as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wait__bindgen_ty_2 {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u16>,
    pub __bindgen_align: [u16; 0usize],
}
impl wait__bindgen_ty_2 {
    #[inline]
    pub fn w_Stopval(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 8u8) as u16) }
    }
    #[inline]
    pub fn set_w_Stopval(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Stopsig(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(8usize, 8u8) as u16) }
    }
    #[inline]
    pub fn set_w_Stopsig(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::core::mem::transmute(val);
            self._bitfield_1.set(8usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Fill2(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as u16) }
    }
    #[inline]
    pub fn set_w_Fill2(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::core::mem::transmute(val);
            self._bitfield_1.set(16usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        w_Stopval: ::std::os::raw::c_ushort,
        w_Stopsig: ::std::os::raw::c_ushort,
        w_Fill2: ::std::os::raw::c_ushort,
    ) -> __BindgenBitfieldUnit<[u8; 4usize], u16> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize], u16> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 8u8, {
            let w_Stopval: u16 = unsafe { ::core::mem::transmute(w_Stopval) };
            w_Stopval as u64
        });
        __bindgen_bitfield_unit.set(8usize, 8u8, {
            let w_Stopsig: u16 = unsafe { ::core::mem::transmute(w_Stopsig) };
            w_Stopsig as u64
        });
        __bindgen_bitfield_unit.set(16usize, 16u8, {
            let w_Fill2: u16 = unsafe { ::core::mem::transmute(w_Fill2) };
            w_Fill2 as u64
        });
        __bindgen_bitfield_unit
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sh_getopt_state {
    pub gs_optarg: *mut ::std::os::raw::c_char,
    pub gs_optind: ::std::os::raw::c_int,
    pub gs_curopt: ::std::os::raw::c_int,
    pub gs_nextchar: *mut ::std::os::raw::c_char,
    pub gs_charindex: ::std::os::raw::c_int,
    pub gs_flags: ::std::os::raw::c_int,
}
pub type sh_getopt_state_t = sh_getopt_state;

pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __locale_data {
    pub _address: u8,
}
