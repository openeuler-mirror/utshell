//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later


use libc::{c_int,c_char,c_void, servent, setservent, getservent, endservent, group, setgrent, getgrent, endgrent};
use r_bash::{get_string_value,show_shell_version, strvec_resize,strvec_create,sh_parser_state_t,saved_command_line_count,current_command_line_count,
    bash_add_history, history_lines_this_session,save_parser_state,parse_and_execute,restore_parser_state,reset_readahead_token,posixly_correct,get_alias_value,
    locale_mb_cur_max,is_basic_table,locale_utf8locale,SEVAL_NOHIST, skip_to_delim,SD_NOJMP,SD_COMPLETE, mbschr, COMPSPEC, current_prompt_string, ps1_prompt, 
    parser_in_command_position, unclosed_pair, prog_completion_enabled, progcomp_size, progcomp_search, programmable_completions, pcomp_set_readline_variables, 
    COPT_BASHDEFAULT, COPT_DEFAULT, strvec_dispose, search_for_command, executable_file, executable_or_directory, SHELL_VAR, alias_t, all_variables_matching_prefix, 
    hist_verify, pre_process_line, alias_expand, dispose_word, expand_word, string_list, dispose_words, ignorevar, ign, setup_ignore_patterns, fnx_fromfs, 
    unbound_vars_is_error, expand_prompt_string, no_symbolic_links, get_working_directory, sh_canonpath, dirspell, strvec_strcmp, shell_glob_filename, substring, 
    history_expansion, history_expansion_inhibited,mbstrlen, sh_double_quote, sh_single_quote, sh_backslash_quote, internal_error, bind_variable, inttostr,
    bind_int_variable, array_needs_making, att_exported, find_variable, check_unbind_variable, builtin_error, sigterm_received, terminating_signal, interrupt_state, 
    sigalrm_seen, this_shell_builtin, last_command_exit_value, throw_to_top_level,glob_error_return, all_visible_functions, all_aliases, num_shell_builtins, shell_builtins,
    BUILTIN_ENABLED, sh_makepath,};
use r_readline::*;
use rcommon::{WordList,WordDesc};
use r_shell::{FREE,};
use r_bracecomp::bash_brace_completion;

extern "C" {
    fn rl_complete(_: c_int, _: c_int) -> c_int;
    // fn bash_brace_completion(_: c_int, _: c_int) -> c_int;
    fn char_is_quoted(_: *mut c_char, _: c_int) -> c_int;
    static mut sh_syntaxtab: [c_int; 0];
    fn alloc_word_desc() -> *mut WordDesc;
    fn glob_pattern_p(_: *const libc::c_char) -> libc::c_int;
    fn read_builtin(_: *mut WordList) -> libc::c_int;
    static mut word_token_alist: [STRING_INT_ALIST; 0];
    static mut glob_ignore_case: libc::c_int;
}


pub type rl_hook_func_t = unsafe extern "C" fn() -> c_int;
pub type rl_command_func_t = unsafe extern "C" fn(
    c_int,
    c_int,
) -> c_int;

#[macro_export]
macro_rules! STREQN {
    ($a:expr,$b:expr,$n:expr) => {
        if $n == 0 {
            1  
        } else {
            (*$a.offset(0 as isize) == *$b.offset(0 as isize)
                && strncmp($a, $b, $n as usize) == 0 ) as i32
        }
    };
}

#[macro_export]
macro_rules! STREQ {
    ($a:expr, $b:expr) => {
        (*$a.offset(0 as c_int as isize) as c_int
                    == *$b.offset(0 as c_int as isize) as c_int
                    && strcmp($a, $b) == 0 as c_int)
    };
}

#[no_mangle]
pub static mut bash_readline_initialized: c_int = 0 as c_int;
#[no_mangle]
pub static mut perform_hostname_completion: c_int = 1 as c_int;
#[no_mangle]
pub static mut no_empty_command_completion: c_int = 0;
#[no_mangle]
pub static mut force_fignore: c_int = 1 as c_int;
#[no_mangle]
pub static mut dircomplete_spelling: c_int = 0 as c_int;
#[no_mangle]
pub static mut dircomplete_expand: c_int = 0 as c_int;
#[no_mangle]
pub static mut dircomplete_expand_relpath: c_int = 0 as c_int;
#[no_mangle]
pub static mut complete_fullquote: c_int = 1 as c_int;
static mut bash_completer_word_break_characters: *mut c_char = b" \t\n\"'@><=;|&(:\0"
    as *const u8 as *const c_char as *mut c_char;
static mut bash_nohostname_word_break_characters: *mut c_char = b" \t\n\"'><=;|&(:\0"
    as *const u8 as *const c_char as *mut c_char;
static mut default_filename_quote_characters: *const c_char = b" \t\n\\\"'@<>=;|&()#$`?*[!:{~\0"
    as *const u8 as *const c_char;
static mut custom_filename_quote_characters: *mut c_char = 0 as *const c_char
    as *mut c_char;
static mut filename_bstab: [c_char; 256] = [0; 256];
static mut old_rl_startup_hook: Option::<rl_hook_func_t> = unsafe {
    ::std::mem::transmute::<
        *mut c_void,
        Option::<rl_hook_func_t>,
    >(0 as *const c_void as *mut c_void)
};
static mut dot_in_path: c_int = 0 as c_int;
static mut dabbrev_expand_active: c_int = 0 as c_int;
static mut completion_quoting_style: c_int = 3 as c_int;
static mut vi_tab_binding: Option::<rl_command_func_t> = unsafe {
    Some(rl_complete as unsafe extern "C" fn(c_int, c_int) -> c_int)
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

#[no_mangle]
pub unsafe extern "C" fn posix_readline_initialize(mut on_or_off: c_int) {
    static mut kseq: [c_char; 2] = [
        CTRL!('I' as i32) as c_char,
        0  as c_char,
    ];
    if on_or_off != 0 {
        rl_variable_bind(
            b"comment-begin\0" as *const u8 as *const c_char,
            b"#\0" as *const u8 as *const c_char,
        );
    }
    if on_or_off != 0 {
        vi_tab_binding = rl_function_of_keyseq(
            kseq.as_mut_ptr(),
            vi_insertion_keymap.as_mut_ptr(),
            0 as *mut c_int,
        );
        rl_bind_key_in_map(
            CTRL!('I' as i32) as c_int,
            Some(
                rl_insert
                    as unsafe extern "C" fn(c_int, c_int) -> c_int,
            ),
            vi_insertion_keymap.as_mut_ptr(),
        );
    } else if rl_function_of_keyseq(
            kseq.as_mut_ptr(),
            vi_insertion_keymap.as_mut_ptr(),
            0 as *mut c_void as *mut c_int,
        )
            == Some(
                rl_insert
                    as unsafe extern "C" fn(c_int, c_int) -> c_int,
            )
        {
        rl_bind_key_in_map(
            CTRL!('I' as i32) as c_int,
            vi_tab_binding,
            vi_insertion_keymap.as_mut_ptr(),
        );
    }
}

#[macro_export]
macro_rules! savestring {
    ($x:expr) => {
        strcpy(malloc((strlen($x as *const c_char) + 1) as usize) as *mut c_char, $x) as *mut c_char
    };
}

#[no_mangle]
pub unsafe extern "C" fn reset_completer_word_break_chars() {
    rl_completer_word_break_characters = if perform_hostname_completion != 0 {
        savestring!(bash_completer_word_break_characters)
    } else {
        savestring!(bash_nohostname_word_break_characters)
    };
}

#[no_mangle]
pub unsafe extern "C" fn enable_hostname_completion(
    mut on_or_off: c_int,
) -> c_int {
    let mut old_value: c_int = 0;
    let mut at: *mut c_char = 0 as *mut c_char;
    let mut nv: *mut c_char = 0 as *mut c_char;
    let mut nval: *mut c_char = 0 as *mut c_char;

    old_value = perform_hostname_completion;

    if on_or_off != 0 {
        perform_hostname_completion = 1 as c_int;
        rl_special_prefixes = b"$@\0" as *const u8 as *const c_char;
    } else {
        perform_hostname_completion = 0 as c_int;
        rl_special_prefixes = b"$\0" as *const u8 as *const c_char;
    }
    if bash_readline_initialized == 0 as c_int
        && (rl_completer_word_break_characters.is_null()
            || rl_completer_word_break_characters
                == rl_basic_word_break_characters as *mut c_char)
    {
        if on_or_off != 0 {
            rl_completer_word_break_characters = savestring!(bash_completer_word_break_characters);
        } else {
            rl_completer_word_break_characters = savestring!(bash_nohostname_word_break_characters);
        }
    } else {
        at = strchr(rl_completer_word_break_characters, '@' as i32);
        if at.is_null() && on_or_off == 0 as c_int
            || !at.is_null() && on_or_off != 0 as c_int
        {
            return old_value;
        }
        nval = malloc(
            ((strlen(rl_completer_word_break_characters))+ 1 + on_or_off as u64) as usize
        ) as *mut c_char;
        
        if on_or_off == 0 as c_int {
            nv = nval;
            at = rl_completer_word_break_characters;
            while *at != 0 {
                if *at as c_int != '@' as i32 {
                    let fresh0 = at;
                    at = at.offset(1);
                    let fresh1 = nv;
                    nv = nv.offset(1);
                    *fresh1 = *fresh0;
                } else {
                    at = at.offset(1);
                }
            }
            *nv = '\u{0}' as i32 as c_char;
        } else {
            *nval.offset(0 as c_int as isize) = '@' as i32 as c_char;
            strcpy(
                nval.offset(1 as c_int as isize),
                rl_completer_word_break_characters,
            );
        }
        free(
            rl_completer_word_break_characters as *mut c_void,
        );
        rl_completer_word_break_characters = nval;
    }
    return old_value;
}

#[no_mangle]
pub unsafe extern "C" fn initialize_readline() {
    let mut func: Option::<rl_command_func_t> = None;
    let mut kseq: [c_char; 2] = [0; 2];
    if bash_readline_initialized != 0 {
        return;
    }
    rl_terminal_name = get_string_value(b"TERM\0" as *const u8 as *const c_char);
    rl_instream = stdin;
    rl_outstream = stderr;

    rl_readline_name = b"Bash\0" as *const u8 as *const c_char;
   
    rl_add_defun(
        b"shell-expand-line\0" as *const u8 as *const c_char,
        Some(
            shell_expand_line
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        -(1 as c_int),
    );
    rl_add_defun(
        b"history-expand-line\0" as *const u8 as *const c_char,
        Some(
            history_expand_line
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        -(1 as c_int),
    );
    rl_add_defun(
        b"magic-space\0" as *const u8 as *const c_char,
        Some(
            tcsh_magic_space
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        -(1 as c_int),
    );
    rl_add_defun(
        b"shell-forward-word\0" as *const u8 as *const c_char,
        Some(
            bash_forward_shellword
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        -(1 as c_int),
    );
    rl_add_defun(
        b"shell-backward-word\0" as *const u8 as *const c_char,
        Some(
            bash_backward_shellword
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        -(1 as c_int),
    );
    rl_add_defun(
        b"shell-kill-word\0" as *const u8 as *const c_char,
        Some(
            bash_kill_shellword
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        -(1 as c_int),
    );
    rl_add_defun(
        b"shell-backward-kill-word\0" as *const u8 as *const c_char,
        Some(
            bash_backward_kill_shellword
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        -(1 as c_int),
    );
    rl_add_defun(
        b"shell-transpose-words\0" as *const u8 as *const c_char,
        Some(
            bash_transpose_shellwords
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        -(1 as c_int),
    );
    rl_add_defun(
        b"alias-expand-line\0" as *const u8 as *const c_char,
        Some(
            alias_expand_line
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        -(1 as c_int),
    );
    rl_add_defun(
        b"history-and-alias-expand-line\0" as *const u8 as *const c_char,
        Some(
            history_and_alias_expand_line
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        -(1 as c_int),
    );
    rl_add_defun(
        b"insert-last-argument\0" as *const u8 as *const c_char,
        Some(
            rl_yank_last_arg
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        -(1 as c_int),
    );
    rl_add_defun(
        b"display-shell-version\0" as *const u8 as *const c_char,
        Some(
            display_shell_version
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        -(1 as c_int),
    );
    rl_add_defun(
        b"edit-and-execute-command\0" as *const u8 as *const c_char,
        Some(
            emacs_edit_and_execute_command
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        -(1 as c_int),
    );
    rl_add_defun(
        b"complete-into-braces\0" as *const u8 as *const c_char,
        Some(
            bash_brace_completion
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        -(1 as c_int),
    );
    rl_add_defun(
        b"complete-filename\0" as *const u8 as *const c_char,
        Some(
            bash_complete_filename
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        -(1 as c_int),
    );
    rl_add_defun(
        b"possible-filename-completions\0" as *const u8 as *const c_char,
        Some(
            bash_possible_filename_completions
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        -(1 as c_int),
    );
    rl_add_defun(
        b"complete-username\0" as *const u8 as *const c_char,
        Some(
            bash_complete_username
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        -(1 as c_int),
    );
    rl_add_defun(
        b"possible-username-completions\0" as *const u8 as *const c_char,
        Some(
            bash_possible_username_completions
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        -(1 as c_int),
    );
    rl_add_defun(
        b"complete-hostname\0" as *const u8 as *const c_char,
        Some(
            bash_complete_hostname
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        -(1 as c_int),
    );
    rl_add_defun(
        b"possible-hostname-completions\0" as *const u8 as *const c_char,
        Some(
            bash_possible_hostname_completions
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        -(1 as c_int),
    );
    rl_add_defun(
        b"complete-variable\0" as *const u8 as *const c_char,
        Some(
            bash_complete_variable
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        -(1 as c_int),
    );
    rl_add_defun(
        b"possible-variable-completions\0" as *const u8 as *const c_char,
        Some(
            bash_possible_variable_completions
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        -(1 as c_int),
    );
    rl_add_defun(
        b"complete-command\0" as *const u8 as *const c_char,
        Some(
            bash_complete_command
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        -(1 as c_int),
    );
    rl_add_defun(
        b"possible-command-completions\0" as *const u8 as *const c_char,
        Some(
            bash_possible_command_completions
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        -(1 as c_int),
    );
    rl_add_defun(
        b"glob-complete-word\0" as *const u8 as *const c_char,
        Some(
            bash_glob_complete_word
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        -(1 as c_int),
    );
    rl_add_defun(
        b"glob-expand-word\0" as *const u8 as *const c_char,
        Some(
            bash_glob_expand_word
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        -(1 as c_int),
    );
    rl_add_defun(
        b"glob-list-expansions\0" as *const u8 as *const c_char,
        Some(
            bash_glob_list_expansions
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        -(1 as c_int),
    );
    rl_add_defun(
        b"dynamic-complete-history\0" as *const u8 as *const c_char,
        Some(
            dynamic_complete_history
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        -(1 as c_int),
    );
    rl_add_defun(
        b"dabbrev-expand\0" as *const u8 as *const c_char,
        Some(
            bash_dabbrev_expand
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        -(1 as c_int),
    );
    if rl_readline_state & 0x2 as c_int as libc::c_ulong
        == 0 as c_int as libc::c_ulong
    {
        rl_initialize();
    }
    rl_bind_key_if_unbound_in_map(
        'E' as i32 & 0x1f as c_int,
        Some(
            shell_expand_line
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        emacs_meta_keymap.as_mut_ptr(),
    );
    rl_bind_key_if_unbound_in_map(
        '^' as i32,
        Some(
            history_expand_line
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        emacs_meta_keymap.as_mut_ptr(),
    );
    rl_bind_key_if_unbound_in_map(
        'V' as i32 & 0x1f as c_int,
        Some(
            display_shell_version
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        emacs_ctlx_keymap.as_mut_ptr(),
    );
    kseq[0 as usize] =  CTRL!('J' as i32 ) as c_char;
    kseq[1 as usize] = '\u{0}' as i32 as c_char;
    func = rl_function_of_keyseq(
        kseq.as_mut_ptr(),
        emacs_meta_keymap.as_mut_ptr(),
        0 as *mut c_void as *mut c_int,
    );
    if func
        == Some(
            rl_vi_editing_mode
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        )
    {
        rl_unbind_key_in_map(
            CTRL!('J' as i32) as c_int,
            emacs_meta_keymap.as_mut_ptr(),
        );
    }
    kseq[0 as usize] = CTRL!('M' as i32 ) as c_char;
    func = rl_function_of_keyseq(
        kseq.as_mut_ptr(),
        emacs_meta_keymap.as_mut_ptr(),
        0 as *mut c_void as *mut c_int,
    );
    if func
        == Some(
            rl_vi_editing_mode
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        )
    {
        rl_unbind_key_in_map(
            CTRL!('M' as i32) as c_int,
            emacs_meta_keymap.as_mut_ptr(),
        );
    }
    kseq[0 as usize] = CTRL!('E' as i32 ) as c_char;
    func = rl_function_of_keyseq(
        kseq.as_mut_ptr(),
        vi_movement_keymap.as_mut_ptr(),
        0 as *mut c_void as *mut c_int,
    );
    if func
        == Some(
            rl_emacs_editing_mode
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        )
    {
        rl_unbind_key_in_map(
            CTRL!('E' as i32) as c_int,
            vi_movement_keymap.as_mut_ptr(),
        );
    }
    rl_bind_key_if_unbound_in_map(
        '{' as i32,
        Some(
            bash_brace_completion
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        emacs_meta_keymap.as_mut_ptr(),
    );
    rl_bind_key_if_unbound_in_map(
        '/' as i32,
        Some(
            bash_complete_filename
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        emacs_meta_keymap.as_mut_ptr(),
    );
    rl_bind_key_if_unbound_in_map(
        '/' as i32,
        Some(
            bash_possible_filename_completions
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        emacs_ctlx_keymap.as_mut_ptr(),
    );
    kseq[0 as c_int as usize] = '~' as i32 as c_char;
    kseq[1 as c_int as usize] = '\u{0}' as i32 as c_char;
    func = rl_function_of_keyseq(
        kseq.as_mut_ptr(),
        emacs_meta_keymap.as_mut_ptr(),
        0 as *mut c_void as *mut c_int,
    );
    if func.is_none()
        || func
            == Some(
                rl_tilde_expand
                    as unsafe extern "C" fn(c_int, c_int) -> c_int,
            )
    {
        rl_bind_keyseq_in_map(
            kseq.as_mut_ptr(),
            Some(
                bash_complete_username
                    as unsafe extern "C" fn(c_int, c_int) -> c_int,
            ),
            emacs_meta_keymap.as_mut_ptr(),
        );
    }
    rl_bind_key_if_unbound_in_map(
        '~' as i32,
        Some(
            bash_possible_username_completions
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        emacs_ctlx_keymap.as_mut_ptr(),
    );
    rl_bind_key_if_unbound_in_map(
        '@' as i32,
        Some(
            bash_complete_hostname
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        emacs_meta_keymap.as_mut_ptr(),
    );
    rl_bind_key_if_unbound_in_map(
        '@' as i32,
        Some(
            bash_possible_hostname_completions
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        emacs_ctlx_keymap.as_mut_ptr(),
    );
    rl_bind_key_if_unbound_in_map(
        '$' as i32,
        Some(
            bash_complete_variable
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        emacs_meta_keymap.as_mut_ptr(),
    );
    rl_bind_key_if_unbound_in_map(
        '$' as i32,
        Some(
            bash_possible_variable_completions
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        emacs_ctlx_keymap.as_mut_ptr(),
    );
    rl_bind_key_if_unbound_in_map(
        '!' as i32,
        Some(
            bash_complete_command
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        emacs_meta_keymap.as_mut_ptr(),
    );
    rl_bind_key_if_unbound_in_map(
        '!' as i32,
        Some(
            bash_possible_command_completions
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        emacs_ctlx_keymap.as_mut_ptr(),
    );
    rl_bind_key_if_unbound_in_map(
        'g' as i32,
        Some(
            bash_glob_complete_word
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        emacs_meta_keymap.as_mut_ptr(),
    );
    rl_bind_key_if_unbound_in_map(
        '*' as i32,
        Some(
            bash_glob_expand_word
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        emacs_ctlx_keymap.as_mut_ptr(),
    );
    rl_bind_key_if_unbound_in_map(
        'g' as i32,
        Some(
            bash_glob_list_expansions
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        emacs_ctlx_keymap.as_mut_ptr(),
    );
    kseq[0 as c_int as usize] = '\t' as i32 as c_char;
    kseq[1 as c_int as usize] = '\u{0}' as i32 as c_char;
    func = rl_function_of_keyseq(
        kseq.as_mut_ptr(),
        emacs_meta_keymap.as_mut_ptr(),
        0 as *mut c_void as *mut c_int,
    );
    if func.is_none()
        || func
            == Some(
                rl_tab_insert
                    as unsafe extern "C" fn(c_int, c_int) -> c_int,
            )
    {
        rl_bind_key_in_map(
            '\t' as i32,
            Some(
                dynamic_complete_history
                    as unsafe extern "C" fn(c_int, c_int) -> c_int,
            ),
            emacs_meta_keymap.as_mut_ptr(),
        );
    }
    rl_attempted_completion_function = Some(
        attempt_shell_completion
            as unsafe extern "C" fn(
                *const c_char,
                c_int,
                c_int,
            ) -> *mut *mut c_char,
    );
    set_directory_hook();
    rl_filename_rewrite_hook = Some(
        bash_filename_rewrite_hook
            as unsafe extern "C" fn(*mut c_char, c_int) -> *mut c_char,
    );
    rl_filename_stat_hook = Some(
        bash_filename_stat_hook
            as unsafe extern "C" fn(*mut *mut c_char) -> c_int,
    );
    rl_ignore_some_completions_function = Some(
        filename_completion_ignore
            as unsafe extern "C" fn(*mut *mut c_char) -> c_int,
    );
    rl_bind_key_if_unbound_in_map(
        'E' as i32 & 0x1f as c_int,
        Some(
            emacs_edit_and_execute_command
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        emacs_ctlx_keymap.as_mut_ptr(),
    );
    rl_bind_key_if_unbound_in_map(
        'v' as i32,
        Some(
            vi_edit_and_execute_command
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        vi_movement_keymap.as_mut_ptr(),
    );
    rl_bind_key_if_unbound_in_map(
        '@' as i32,
        Some(
            posix_edit_macros
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        vi_movement_keymap.as_mut_ptr(),
    );
    rl_bind_key_in_map(
        '\\' as i32,
        Some(
            bash_vi_complete
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        vi_movement_keymap.as_mut_ptr(),
    );
    rl_bind_key_in_map(
        '*' as i32,
        Some(
            bash_vi_complete
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        vi_movement_keymap.as_mut_ptr(),
    );
    rl_bind_key_in_map(
        '=' as i32,
        Some(
            bash_vi_complete
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        vi_movement_keymap.as_mut_ptr(),
    );
    rl_completer_quote_characters = b"'\"\0" as *const u8 as *const c_char;
    enable_hostname_completion(perform_hostname_completion);
    rl_filename_quote_characters = default_filename_quote_characters;
    set_filename_bstab(rl_filename_quote_characters);
    rl_filename_quoting_function = Some(
        bash_quote_filename
            as unsafe extern "C" fn(
                *mut c_char,
                c_int,
                *mut c_char,
            ) -> *mut c_char,
    );
    rl_filename_dequoting_function = Some(
        bash_dequote_filename
            as unsafe extern "C" fn(*mut c_char, c_int) -> *mut c_char,
    );
    rl_char_is_quoted_p = Some(
        char_is_quoted
            as unsafe extern "C" fn(*mut c_char, c_int) -> c_int,
    );
    rl_bind_key_if_unbound_in_map(
        'B' as i32 & 0x1f as c_int,
        Some(
            bash_backward_shellword
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        emacs_meta_keymap.as_mut_ptr(),
    );
    rl_bind_key_if_unbound_in_map(
        'D' as i32 & 0x1f as c_int,
        Some(
            bash_kill_shellword
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        emacs_meta_keymap.as_mut_ptr(),
    );
    rl_bind_key_if_unbound_in_map(
        'F' as i32 & 0x1f as c_int,
        Some(
            bash_forward_shellword
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        emacs_meta_keymap.as_mut_ptr(),
    );
    rl_bind_key_if_unbound_in_map(
        'T' as i32 & 0x1f as c_int,
        Some(
            bash_transpose_shellwords
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        ),
        emacs_meta_keymap.as_mut_ptr(),
    );
    bash_readline_initialized = 1 as c_int;
}


#[no_mangle]
pub unsafe extern "C" fn bashline_reinitialize() {
    bash_readline_initialized = 0 ;
}
#[no_mangle]
pub unsafe extern "C" fn bashline_set_event_hook() {
    rl_signal_event_hook = Some(
        bash_event_hook as unsafe extern "C" fn() -> c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn bashline_reset_event_hook() {
    rl_signal_event_hook = None;
}

#[no_mangle]
pub unsafe extern "C" fn bashline_reset() {
    tilde_initialize();
    rl_attempted_completion_function = Some(
        attempt_shell_completion
            as unsafe extern "C" fn(
                *const c_char,
                c_int,
                c_int,
            ) -> *mut *mut c_char,
    );
    rl_completion_entry_function = None;
    rl_ignore_some_completions_function = Some(
        filename_completion_ignore
            as unsafe extern "C" fn(*mut *mut c_char) -> c_int,
    );
    rl_filename_quote_characters = default_filename_quote_characters;
    set_filename_bstab(rl_filename_quote_characters);

    set_directory_hook();
    rl_filename_stat_hook = Some(
        bash_filename_stat_hook
            as unsafe extern "C" fn(*mut *mut c_char) -> c_int,
    );

    bashline_reset_event_hook();

    rl_sort_completion_matches = 1 as c_int;
}


static mut push_to_readline: *mut c_char = 0 as *mut c_char;

unsafe extern "C" fn bash_push_line() -> c_int {
    if !push_to_readline.is_null() {
        rl_insert_text(push_to_readline);
        free(
            push_to_readline as *mut c_void,
        );
        push_to_readline = 0 as *mut c_char;
        rl_startup_hook = old_rl_startup_hook;
    }
    return 0 ;
}

#[no_mangle]
pub unsafe extern "C" fn bash_re_edit(mut line: *mut c_char) -> c_int {
    FREE!(push_to_readline);

    push_to_readline = savestring!(line);
    old_rl_startup_hook = rl_startup_hook;
    rl_startup_hook = Some(bash_push_line );
    return 0 ;
}

unsafe extern "C" fn display_shell_version(
    mut count: c_int,
    mut c: c_int,
) -> c_int {
    rl_crlf();
    show_shell_version(0 );
    putc('\r' as i32, rl_outstream);
    fflush(rl_outstream);
    rl_on_new_line();
    rl_redisplay();
    return 0 ;
}

static mut hostname_list: *mut *mut c_char = 0 as *mut *mut c_char;

static mut hostname_list_size: c_int = 0;

static mut hostname_list_length: c_int = 0;

#[no_mangle]
pub static mut hostname_list_initialized: c_int = 0 ;
unsafe extern "C" fn initialize_hostname_list() {
    let mut temp: *mut c_char = 0 as *mut c_char;
    
    temp = get_string_value(b"HOSTFILE\0" as *const u8 as *const c_char);
    if temp.is_null() {
        temp = get_string_value(
            b"hostname_completion_file\0" as *const u8 as *const c_char,
        );
    }
    if temp.is_null() {
        temp = b"/etc/hosts\0" as *const u8 as *const c_char as *mut c_char;
    }
    
    snarf_hosts_from_file(temp);
    
    if !hostname_list.is_null() {
        hostname_list_initialized += 1;
    }
}

unsafe extern "C" fn add_host_name(mut name: *mut c_char) {
    if hostname_list_length + 2 > hostname_list_size {
        hostname_list_size = hostname_list_size + 32 
            - hostname_list_size % 32 ;
        hostname_list = strvec_resize(hostname_list, hostname_list_size);
    }
    //有可能存在错误的地方
    *hostname_list.offset(hostname_list_length as isize) = savestring!(name);
    hostname_list_length = hostname_list_length + 1;
    *hostname_list.offset(hostname_list_length as isize) = 0 as *mut c_char;
}

#[macro_export]
macro_rules! cr_whitespace {
    ($c:expr) => {
        $c as c_int == '\r' as i32
        || $c as c_int == '\n' as i32
        || whitespace!($c)
    };
}

#[macro_export]
macro_rules! whitespace {
    ($c:expr) => {
        $c as c_int == ' ' as i32
        || $c as c_int == '\t' as i32
    };
}

#[macro_export]
macro_rules! DIGIT {
    ($c:expr) => {
        $c as c_int >= '0' as i32
        && $c as c_int <= '9' as i32
    };
}

unsafe extern "C" fn snarf_hosts_from_file(mut filename: *mut c_char) {
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut temp: *mut c_char = 0 as *mut c_char;
    let mut buffer: [c_char; 256] = [0; 256];
    let mut name: [c_char; 256] = [0; 256];
    let mut i: c_int = 0;
    let mut start: c_int = 0;
    
    file = fopen(filename, b"r\0" as *const u8 as *const c_char);
    if file.is_null() {
        return;
    }
    
    loop {
        temp = fgets(buffer.as_mut_ptr(), 255 as c_int, file);
        if temp.is_null() {
            break;
        }

        i = 0 ;
        while buffer[i as usize] as c_int != 0
            && cr_whitespace!(buffer[i as usize])
        {
            i += 1;
        }
        
        if buffer[i as usize] as c_int == '\u{0}' as i32
            || buffer[i as usize] as c_int == '#' as i32
        {
            continue;
        }
        
        if strncmp(
            buffer.as_mut_ptr().offset(i as isize),
            b"$include \0" as *const u8 as *const c_char,
            (9 as c_int as libc::c_ulong).try_into().unwrap(),
        ) == 0 
        {
            let mut incfile: *mut c_char = 0 as *mut c_char;
            let mut t: *mut c_char = 0 as *mut c_char;
            
            incfile = buffer
                .as_mut_ptr()
                .offset(i as isize)
                .offset(9 as isize);
            while *incfile as c_int != 0
                && whitespace!(*incfile)
            {
                incfile = incfile.offset(1);
            }
            
            t = incfile;
            while *t as c_int != 0
                && cr_whitespace!(*t as c_int) == false
            {
                t = t.offset(1);
            }
            *t = '\u{0}' as i32 as c_char;
            snarf_hosts_from_file(incfile);
        } else {
            if DIGIT![i as usize] 
            {
                while buffer[i as usize] as c_int != 0
                    && cr_whitespace!(buffer[i as usize] )== false
                {
                    i += 1;
                }
            }

            while buffer[i as usize] != 0 {
                while cr_whitespace!(buffer[i as usize])
                {
                    i += 1;
                }
                if buffer[i as usize] as c_int == '\u{0}' as i32
                    || buffer[i as usize] as c_int == '#' as i32
                {
                    break;
                }
                start = i;
                while buffer[i as usize] as c_int != 0
                    && cr_whitespace!(buffer[i as usize]) == false
                {
                    i += 1;
                }

                if i == start {
                    continue;
                }

                strncpy(
                    name.as_mut_ptr(),
                    buffer.as_mut_ptr().offset(start as isize),
                    ((i - start) as libc::c_ulong).try_into().unwrap(),
                );
                name[(i - start) as usize] = '\u{0}' as i32 as c_char;
                add_host_name(name.as_mut_ptr());
            }
        }
    }
    fclose(file);
}


#[no_mangle]
pub unsafe extern "C" fn get_hostname_list() -> *mut *mut c_char {
    if hostname_list_initialized == 0 as c_int {
        initialize_hostname_list();
    }
    return hostname_list;
}

#[no_mangle]
pub unsafe extern "C" fn clear_hostname_list() {
    let mut i: c_int = 0;
    if hostname_list_initialized == 0 as c_int {
        return;
    }
    i = 0 ;
    while i < hostname_list_length {
        free(
            *hostname_list.offset(i as isize) as *mut c_void,
        );
        i += 1;
    }
    hostname_list_initialized = 0 ;
    hostname_list_length = hostname_list_initialized;
}

unsafe extern "C" fn hostnames_matching(
    mut text: *mut c_char,
) -> *mut *mut c_char {
    let mut i: c_int = 0;
    let mut len: c_int = 0;
    let mut nmatch: c_int = 0;
    let mut rsize: c_int = 0;
    let mut result: *mut *mut c_char = 0 as *mut *mut c_char;
    
    if hostname_list_initialized == 0 {
        initialize_hostname_list();
    }

    if hostname_list_initialized == 0 {
        return 0 as *mut *mut c_char;
    }

    if *text as c_int == '\u{0}' as i32 {
        result = strvec_create(1 + hostname_list_length);
        i = 0 ;
        while i < hostname_list_length {
            *result.offset(i as isize) = *hostname_list.offset(i as isize);
            i += 1;
        }
        *result.offset(i as isize) = 0 as *mut c_char;
        return result;
    }

    len = strlen(text) as c_int;
    result = 0 as *mut *mut c_char;

    rsize = 0 as c_int;
    nmatch = rsize;
    i = nmatch;
    while i < hostname_list_length {
        if STREQN!(text, *hostname_list.offset(i as isize),len as usize) as i32 == 0 {
            continue;
        }

        if nmatch >= rsize - 1 {
            rsize = rsize + 16 - rsize % 16 ;
            result = strvec_resize(result, rsize);
        }

        *result.offset(nmatch as isize) = *hostname_list.offset(i as isize);
        nmatch = nmatch + 1;

        i += 1;
    }

    if nmatch != 0 {
        *result.offset(nmatch as isize) = 0 as *mut c_char;
    }
    return result;
}

#[macro_export]
macro_rules! RL_BOOLEAN_VARIABLE_VALUE {
    ($c:expr) => {
        (*$c.offset(0 as isize) as c_int == 'o' as i32
        && *$c.offset(1 as isize) as c_int == 'n' as i32
        && *$c.offset(2 as isize) as c_int == '\u{0}' as i32)
    };
}

unsafe extern "C" fn edit_and_execute_command(
    mut count: c_int,
    mut c: c_int,
    mut editing_mode: c_int,
    mut edit_command: *mut c_char,
) -> c_int {
    let mut command: *mut c_char = 0 as *mut c_char;
    let mut metaval: *mut c_char = 0 as *mut c_char;
    let mut r: c_int = 0;
    let mut rrs: c_int = 0;
    let mut metaflag: c_int = 0;
    let mut ps: sh_parser_state_t = sh_parser_state_t {
        parser_state: 0,
        token_state: 0 as *mut c_int,
        token: 0 as *mut c_char,
        token_buffer_size: 0,
        input_line_terminator: 0,
        eof_encountered: 0,
        prompt_string_pointer: 0 as *mut *mut c_char,
        current_command_line_count: 0,
        remember_on_history: 0,
        history_expansion_inhibited: 0,
        last_command_exit_value: 0,
        pipestatus: 0 as *mut r_bash::ARRAY,
        last_shell_builtin: None,
        this_shell_builtin: None,
        expand_aliases: 0,
        echo_input_at_read: 0,
        need_here_doc: 0,
        here_doc_first_line: 0,
        redir_stack: [0 as *mut r_bash::REDIRECT; 16],
    };

    rrs = rl_readline_state as c_int;
    saved_command_line_count = current_command_line_count;
    
    rl_newline(1 , c);
    
    if rl_explicit_arg != 0 {
        command = malloc(
            (strlen(edit_command)).wrapping_add(8 as c_int as libc::c_ulong) as usize,
        ) as *mut c_char;
        sprintf(
            command,
            b"%s %d\0" as *const u8 as *const c_char,
            edit_command,
            count,
        );
    } else {
        using_history();
        current_command_line_count += 1;
        bash_add_history(rl_line_buffer);
        current_command_line_count = 0 as c_int;
        bash_add_history(b"\0" as *const u8 as *const c_char as *mut c_char);
        history_lines_this_session += 1;
        using_history();
        command = savestring!(edit_command);
    }

    metaval = rl_variable_value(b"input-meta\0" as *const u8 as *const c_char);
    metaflag = RL_BOOLEAN_VARIABLE_VALUE!(metaval) as c_int;
    
    if rl_deprep_term_function.is_some() {
        (Some(rl_deprep_term_function.expect("non-null function pointer")))
            .expect("non-null function pointer")();
    }
    rl_clear_signals();
    save_parser_state(&mut ps);
    r = parse_and_execute(
        command,
        if editing_mode == 0 as c_int {
            b"v\0" as *const u8 as *const c_char
        } else {
            b"C-xC-e\0" as *const u8 as *const c_char
        },
        SEVAL_NOHIST as c_int,
    );
    restore_parser_state(&mut ps);
    
    reset_readahead_token();
    
    if rl_prep_term_function.is_some() {
        (Some(rl_prep_term_function.expect("non-null function pointer")))
            .expect("non-null function pointer")(metaflag);
    }
    rl_set_signals();
    
    current_command_line_count = saved_command_line_count;
    
    *rl_line_buffer.offset(0 as c_int as isize) = '\u{0}' as i32 as c_char;
    rl_end = 0 as c_int;
    rl_point = rl_end;
    rl_done = 0 as c_int;
    rl_readline_state = rrs as libc::c_ulong;
    
    if editing_mode == 0 {
        rl_vi_insertion_mode(1 as c_int, c);
    }
    rl_forced_update_display();
    return r;
}

#[macro_export]
macro_rules! POSIX_VI_EDIT_COMMAND {
    () => {
        b"fc -e vi\0" as *const u8 as *const c_char as *mut c_char
    };
}

#[macro_export]
macro_rules! VI_EDIT_COMMAND {
    () => {
        b"fc -e \"${VISUAL:-${EDITOR:-vi}}\"\0" as *const u8 as *const c_char
                as *mut c_char
    };
}

unsafe extern "C" fn vi_edit_and_execute_command(
    mut count: c_int,
    mut c: c_int,
) -> c_int {
    if posixly_correct != 0 {
        return edit_and_execute_command(
            count,
            c,
            0 as c_int,
            POSIX_VI_EDIT_COMMAND!(),
        )
    } else {
        return edit_and_execute_command(
            count,
            c,
            0 as c_int,
            VI_EDIT_COMMAND!(),
        )
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
        b"fc -e \"${VISUAL:-${EDITOR:-emacs}}\"\0" as *const u8 as *const c_char
            as *mut c_char
    };
}

unsafe extern "C" fn emacs_edit_and_execute_command(
    mut count: c_int,
    mut c: c_int,
) -> c_int {
    return edit_and_execute_command(
        count,
        c,
        EMACS_EDITING_MODE!(),
        EMACS_EDIT_COMMAND!(),
    );
}

unsafe extern "C" fn posix_edit_macros(
    mut count: c_int,
    mut key: c_int,
) -> c_int {
    let mut c: c_int = 0;
    let mut alias_name: [c_char; 3] = [0; 3];
    let mut alias_value: *mut c_char = 0 as *mut c_char;
    let mut macro_0: *mut c_char = 0 as *mut c_char;
    
    c = rl_read_key();
    alias_name[0 as usize] = '_' as i32 as c_char;
    alias_name[1 as usize] = c as c_char;
    alias_name[2 as usize] = '\u{0}' as i32 as c_char;
    
    alias_value = get_alias_value(alias_name.as_mut_ptr());
    if !alias_value.is_null() && *alias_value as c_int != 0 {
        macro_0 = savestring!(alias_value);
        rl_push_macro_input(macro_0);
    }
    return 0 ;
}

unsafe extern "C" fn is_basic(mut c: c_char) -> c_int {
    return (*is_basic_table
        .as_ptr()
        .offset((c as libc::c_uchar as c_int >> 5 as c_int) as isize)
        >> (c as libc::c_uchar as c_int & 31 as c_int)
        & 1 as c_int as libc::c_uint) as c_int;
}

#[macro_export]
macro_rules! DECLARE_MBSTATE {
    ($state:expr) => {
        memset(
            &mut $state as *mut mbstate_t as *mut c_void,
            '\u{0}' as i32,
            (::std::mem::size_of::<mbstate_t>() as libc::c_ulong).try_into().unwrap(),
        )
    };
}

#[macro_export]
macro_rules! ADVANCE_CHAR {
    ($_str:expr, $_strsize:expr, $_i:expr,$state:expr) => {
        if locale_mb_cur_max > 1 {
            let mut state_bak: mbstate_t = mbstate_t {
                __count: 0,
                __value: __mbstate_t__bindgen_ty_1 { __wch: 0 },
            };
            let mut mblength: size_t = 0;
            let mut _f: c_int = 0;
            
            _f = is_basic(*$_str.offset($_i as isize));
            if _f != 0 {
                mblength = 1 as size_t;

            } else if locale_utf8locale != 0
                    && *$_str.offset($_i as isize) as c_int
                        & 0x80 as c_int == 0 as c_int
                {
                mblength = (*$_str.offset($_i as isize) as c_int
                    != 0 ) as c_int as size_t;
            } else {
                state_bak = $state;
                mblength = mbrlen(
                    $_str.offset($_i as isize),
                    $_strsize.wrapping_sub($_i as libc::c_ulong).try_into().unwrap(),
                    &mut $state,
                ) as u64;
            }
            if mblength == -(2 as c_int) as size_t
                || mblength == -(1 as c_int) as size_t
            {
                $state = state_bak;
                $_i += 1;
            } else if mblength == 0 as libc::c_ulong {
                $_i += 1;
            } else {
                $_i = ($_i as libc::c_ulong).wrapping_add(mblength) as c_int;
            }
        } else {
            $_i += 1;
        }
    };
}

#[macro_export]
macro_rules! WORDDELIM {
    ($c:expr) => {
        (*sh_syntaxtab.as_mut_ptr().offset($c as libc::c_uchar as isize)
            & 0x1 as c_int != 0
            || *sh_syntaxtab.as_mut_ptr().offset($c as libc::c_uchar as isize)
            & 0x2000 as c_int != 0)
    };
}


pub type size_t = libc::c_ulong;
unsafe extern "C" fn bash_forward_shellword(
    mut count: c_int,
    mut key: c_int,
) -> c_int {
    let mut slen: size_t = 0;
    let mut c: c_int = 0;
    let mut p: c_int = 0;
    let mut state: mbstate_t = mbstate_t {
        __count: 0,
        __value: __mbstate_t__bindgen_ty_1 { __wch: 0 },
    };
    DECLARE_MBSTATE!(state);
    
    if count < 0 {
        return bash_backward_shellword(-count, key);
    }
    
    p = rl_point;
    slen = rl_end as size_t;
    
    while count != 0 {
        if p == rl_end {
            rl_point = rl_end;
            return 0 ;
        }
        
        if char_is_quoted(rl_line_buffer, p) != 0 && p > 0 
            && *rl_line_buffer.offset((p - 1 ) as isize) as c_int
                != '\\' as i32
        {
            loop {
                ADVANCE_CHAR!(rl_line_buffer, slen, p, state);
                if !(p < rl_end && char_is_quoted(rl_line_buffer, p) != 0) {
                    break;
                }
            }
            count -= 1;
        } else {
            while p < rl_end
                && {
                    c = *rl_line_buffer.offset(p as isize) as c_int;
                    c != 0
                }
                && WORDDELIM!(c)
            {
                let optu8: u8 = c as u8;
                let optChar: char = char::from(optu8);
                match optChar {
                    '\\' => {
                        if p < rl_end
                            && *rl_line_buffer.offset(p as isize) as c_int != 0
                        {
                            ADVANCE_CHAR!(rl_line_buffer, slen, p,state);
                        }
                    }
                    '\'' => {
                        p += 1;
                        p = skip_to_delim(
                            rl_line_buffer,
                            p,
                            b"'\0" as *const u8 as *const c_char
                                as *mut c_char,
                            0x1 as c_int,
                        );
                    }
                    '"' => {
                        p += 1;
                        p = skip_to_delim(
                            rl_line_buffer,
                            p,
                            b"\"\0" as *const u8 as *const c_char
                                as *mut c_char,
                            0x1 as c_int,
                        );
                    }
                    _ => {
                        ADVANCE_CHAR!(rl_line_buffer, slen, p,state);
                        
                        continue;
                    }
                }
                if p < rl_end {
                    p += 1;
                }
            }

            if *rl_line_buffer.offset(p as isize) as c_int == 0 
                || p == rl_end
            {
                rl_point = rl_end;
                rl_ding();
                return 0 ;
            }

            while p < rl_end
                && {
                    c = *rl_line_buffer.offset(p as isize) as c_int;
                    c != 0
                }
                && WORDDELIM!(c) == false
            {
                let optu8: u8 = c as u8;
                let optChar: char = char::from(optu8);
                match optChar {
                    '\\' => {
                        if p < rl_end
                            && *rl_line_buffer.offset(p as isize) as c_int != 0
                        {
                            ADVANCE_CHAR!(rl_line_buffer, slen, p, state);
                        }
                    }
                    '\'' => {
                        p += 1;
                        p = skip_to_delim(
                            rl_line_buffer,
                            p,
                            b"'\0" as *const u8 as *const c_char
                                as *mut c_char,
                            0x1 as c_int,
                        );
                    }
                    '"' => {
                        p += 1;
                        p = skip_to_delim(
                            rl_line_buffer,
                            p,
                            b"\"\0" as *const u8 as *const c_char
                                as *mut c_char,
                            0x1 as c_int,
                        );
                    }
                    _ => {
                        ADVANCE_CHAR!(rl_line_buffer, slen, p, state);
                        continue;
                    }
                }
                if p < rl_end {
                    p += 1;
                }
            }
            if p == rl_end
                || *rl_line_buffer.offset(p as isize) as c_int == 0 as c_int
            {
                rl_point = rl_end;
                return 0 ;
            }
            count -= 1;
        }
    }
    rl_point = p;
    return 0 ;
}

#[macro_export]
macro_rules! BACKUP_CHAR {
    ($_str:expr, $_strsize:expr, $_i:expr,$state:expr) => {
        if locale_mb_cur_max > 1 as c_int {
            let mut state_bak: mbstate_t = mbstate_t {
                __count: 0,
                __value: __mbstate_t__bindgen_ty_1 { __wch: 0 },
            };
            let mut mblength: size_t = 0;
            let mut _x: c_int = 0;
            let mut _p: c_int = 0;
            _p = 0 as c_int;
            _x = _p;
            while _x < $_i {
                state_bak = $state;
                mblength = mbrlen(
                    $_str.offset(_x as isize),
                    $_strsize.wrapping_sub(_x as libc::c_ulong).try_into().unwrap(),
                    &mut $state,
                ) as u64;
                if mblength == -(2 as c_int) as size_t
                    || mblength == -(1 as c_int) as size_t
                {
                    $state = state_bak;
                    _x += 1;
                } else if mblength == 0 as c_int as libc::c_ulong {
                    _x += 1;
                } else {
                    _p = _x;
                    _x = (_x as libc::c_ulong).wrapping_add(mblength) as c_int
                        as c_int;
                }
            }
            $_i = _p;
        } else {
            $_i -= 1;
        }
    };
}

unsafe extern "C" fn bash_backward_shellword(
    mut count: c_int,
    mut key: c_int,
) -> c_int {
    let mut slen: size_t = 0;
    let mut c: c_int = 0;
    let mut p: c_int = 0;
    let mut prev_p: c_int = 0;
    let mut state: mbstate_t = mbstate_t {
        __count: 0,
        __value: __mbstate_t__bindgen_ty_1 { __wch: 0 },
    };
    memset(
        &mut state as *mut mbstate_t as *mut c_void,
        '\u{0}' as i32,
        (::std::mem::size_of::<mbstate_t>() as libc::c_ulong).try_into().unwrap(),
    );
    if count < 0 {
        return bash_forward_shellword(-count, key);
    }
    p = rl_point;
    slen = rl_end as size_t;

    while count != 0 {
        if p == 0 {
            rl_point = 0 ;
            return 0 ;
        }
        
        BACKUP_CHAR!(rl_line_buffer, slen, p,state);
        while p > 0 {
            c = *rl_line_buffer.offset(p as isize) as c_int;
            if WORDDELIM!(c) == false
                || char_is_quoted(rl_line_buffer, p) != 0
            {
                break;
            }
            BACKUP_CHAR!(rl_line_buffer, slen, p,state);
        }

        if p == 0 {
            rl_point = 0 ;
            return 0 ;
        }

        prev_p = p;
        while p > 0 {
            c = *rl_line_buffer.offset(p as isize) as c_int;
            if WORDDELIM!(c)
                && char_is_quoted(rl_line_buffer, p) == 0 as c_int
            {
                p = prev_p;
                break;
            } else {
                prev_p = p;
                BACKUP_CHAR!(rl_line_buffer, slen, p,state);
            }
        }
        count -= 1;
    }
    rl_point = p;
    return 0 ;
}

unsafe extern "C" fn bash_kill_shellword(
    mut count: c_int,
    mut key: c_int,
) -> c_int {
    let mut p: c_int = 0;
    
    if count < 0 {
        return bash_backward_kill_shellword(-count, key);
    }
    
    p = rl_point;
    bash_forward_shellword(count, key);
    
    if rl_point != p {
        rl_kill_text(p, rl_point);
    }
    
    rl_point = p;
    if rl_editing_mode == 1 {
        rl_mark = rl_point;
    }
    
    return 0 ;
}

unsafe extern "C" fn bash_backward_kill_shellword(
    mut count: c_int,
    mut key: c_int,
) -> c_int {
    let mut p: c_int = 0;
    
    if count < 0 as c_int {
        return bash_kill_shellword(-count, key);
    }
    
    p = rl_point;
    bash_backward_shellword(count, key);
    
    if rl_point != p {
        rl_kill_text(p, rl_point);
    }
    if rl_editing_mode == 1 as c_int {
        rl_mark = rl_point;
    }
    return 0 as c_int;
}

unsafe extern "C" fn bash_transpose_shellwords(
    mut count: c_int,
    mut key: c_int,
) -> c_int {
    let mut word1: *mut c_char = 0 as *mut c_char;
    let mut word2: *mut c_char = 0 as *mut c_char;
    let mut w1_beg: c_int = 0;
    let mut w1_end: c_int = 0;
    let mut w2_beg: c_int = 0;
    let mut w2_end: c_int = 0;
    let mut orig_point: c_int = rl_point;
    
    if count == 0 as c_int {
        return 0 as c_int;
    }

    bash_forward_shellword(count, key);
    w2_end = rl_point;
    bash_backward_shellword(1 , key);
    w2_beg = rl_point;
    bash_backward_shellword(count, key);
    w1_beg = rl_point;
    bash_forward_shellword(1 , key);
    w1_end = rl_point;

    if w1_beg == w2_beg || w2_beg < w1_end {
        rl_ding();
        rl_point = orig_point;
        return 1 ;
    }

    word1 = rl_copy_text(w1_beg, w1_end);
    word2 = rl_copy_text(w2_beg, w2_end);
    
    rl_begin_undo_group();
    
    rl_point = w2_beg;
    rl_delete_text(w2_beg, w2_end);
    rl_insert_text(word1);
    
    rl_point = w1_beg;
    rl_delete_text(w1_beg, w1_end);
    rl_insert_text(word2);
    
    rl_point = w2_end;
    
    rl_end_undo_group();
    free(
        word1 as *mut c_void
    );
    free(
        word2 as *mut c_void
    );

    return 0;
}

unsafe extern "C" fn check_redir(mut ti: c_int) -> c_int {
    let mut this_char: c_int = 0;
    let mut prev_char: c_int = 0;
    
    this_char = *rl_line_buffer.offset(ti as isize) as c_int;
    prev_char = if ti > 0 as c_int {
        *rl_line_buffer.offset((ti - 1 as c_int) as isize) as c_int
    } else {
        0 as c_int
    };
    
    if this_char == '&' as i32 && (prev_char == '<' as i32 || prev_char == '>' as i32)
        || this_char == '|' as i32 && prev_char == '>' as i32
    {
        return 1 as c_int
    } else {
        if this_char == '{' as i32 && prev_char == '$' as i32 {
            return 1 as c_int
        } else {
            if char_is_quoted(rl_line_buffer, ti) != 0 {
                return 1 as c_int;
            }
        }
    }
    
    return 0 as c_int;
}

unsafe extern "C" fn find_cmd_start(mut start: c_int) -> c_int {
    let mut s: c_int = 0;
    let mut os: c_int = 0;
    let mut ns: c_int = 0;
    os = 0 as c_int;
    
    loop {
        s = skip_to_delim(
            rl_line_buffer,
            os,
            b";|&{(`\0" as *const u8 as *const c_char as *mut c_char,
            0x1 as c_int | 0x100 as c_int,
        );
        if !(s <= start && *rl_line_buffer.offset(s as isize) as c_int != 0) {
            break;
        }
        if s > 0 as c_int
            && *rl_line_buffer.offset(s as isize) as c_int == '|' as i32
            && *rl_line_buffer.offset((s - 1 as c_int) as isize) as c_int
                == '>' as i32
        {
            ns = skip_to_delim(
                rl_line_buffer,
                s + 1 as c_int,
                b";|&{(`\0" as *const u8 as *const c_char as *mut c_char,
                0x1 as c_int | 0x100 as c_int,
            );
            if ns > start
                || *rl_line_buffer.offset(ns as isize) as c_int == 0 as c_int
            {
                return os;
            }
            os = ns + 1 as c_int;
        } else {
            if s >= os && *rl_line_buffer.offset(s as isize) as c_int == '{' as i32
            {
                let mut pc: c_int = 0;
                let mut nc: c_int = 0;
                pc = if s > os { s - 1 as c_int } else { os };
                while pc > os
                    && (*rl_line_buffer.offset(pc as isize) as c_int == ' ' as i32
                        || *rl_line_buffer.offset(pc as isize) as c_int
                            == '\t' as i32)
                {
                    pc -= 1;
                }
                nc = *rl_line_buffer.offset((s + 1 as c_int) as isize)
                    as c_int;
                if pc > os
                    && (*rl_line_buffer.offset((s - 1 as c_int) as isize)
                        as c_int == '{' as i32
                        || (strchr(
                            b";|&{(`\0" as *const u8 as *const c_char,
                            *rl_line_buffer.offset(pc as isize) as c_int,
                        ))
                            .is_null())
                    || *sh_syntaxtab.as_mut_ptr().offset(nc as libc::c_uchar as isize)
                        & 0x2 as c_int == 0 as c_int
                {
                    ns = skip_to_delim(
                        rl_line_buffer,
                        s + 1 as c_int,
                        b";|&{(`\0" as *const u8 as *const c_char
                            as *mut c_char,
                        0x1 as c_int | 0x100 as c_int,
                    );
                    if ns > start
                        || *rl_line_buffer.offset(ns as isize) as c_int
                            == 0 as c_int
                    {
                        return os;
                    }
                    os = ns + 1 as c_int;
                    continue;
                }
            }
            os = s + 1 as c_int;
        }
    }
    return os;
}


#[macro_export]
macro_rules! COMMAND_SEPARATORS {
    () => {
        b";|&{(`\0" as *const u8 as *const c_char as *mut c_char
    };
}

unsafe extern "C" fn find_cmd_end(mut end: c_int) -> c_int {
    let mut e: c_int = 0;
    
    e = skip_to_delim(
        rl_line_buffer,
        end,
        COMMAND_SEPARATORS!(),
        SD_NOJMP as c_int | SD_COMPLETE as c_int,
    );
    return e;
}
unsafe extern "C" fn find_cmd_name(
    mut start: c_int,
    mut sp: *mut c_int,
    mut ep: *mut c_int,
) -> *mut c_char {
    let mut name: *mut c_char = 0 as *mut c_char;
    let mut s: c_int = 0;
    let mut e: c_int = 0;
    
    s = start;
    while whitespace!(*rl_line_buffer.offset(s as isize))
    {
        s += 1;
    }
    e = skip_to_delim(
        rl_line_buffer,
        s,
        b"()<>;&| \t\n\0" as *const u8 as *const c_char as *mut c_char,
        0x1 as c_int | 0x100 as c_int,
    );
    name = r_bash::substring(rl_line_buffer, s, e);
    if !sp.is_null() {
        *sp = s;
    }
    if !ep.is_null() {
        *ep = e;
    }
    return name;
}

static mut prog_complete_matches: *mut *mut c_char = 0 as *const *mut c_char
    as *mut *mut c_char;

unsafe extern "C" fn prog_complete_return(
    mut text: *const c_char,
    mut matchnum: c_int,
) -> *mut c_char {
    static mut ind: c_int = 0;
    if matchnum == 0 as c_int {
        ind = 0 as c_int;
    }
    
    if prog_complete_matches.is_null()
        || (*prog_complete_matches.offset(ind as isize)).is_null()
    {
        return 0 as *mut c_void as *mut c_char;
    }
    let fresh10 = ind;
    ind = ind + 1;
    return *prog_complete_matches.offset(fresh10 as isize);
}

#[macro_export]
macro_rules! member {
    ($c:expr,$s:expr) => {
        (if *$c as c_int
            != 0
        {
            (mbschr(
                $s,
                $c as c_int,
            ) != 0 as *mut c_void as *mut c_char) as c_int
        } else {
            0 as c_int
        }) != 0
    };
}

unsafe extern "C" fn invalid_completion(
    mut text: *const c_char,
    mut ind: c_int,
) -> c_int {
    let mut pind: c_int = 0;
    
    if ind > 0 as c_int
        && *rl_line_buffer.offset(ind as isize) as c_int == '(' as i32
        && member!(rl_line_buffer.offset((ind - 1 as c_int) as isize),
           b"$<>\0" as *const u8 as *const c_char)
    {
        return 0 as c_int;
    }
    pind = ind - 1 as c_int;
    while pind > 0 as c_int
        && whitespace!(*rl_line_buffer.offset(pind as isize))
    {
        pind -= 1;
    }
    if ind >= 0 && pind <= 0 
        && *rl_line_buffer.offset(ind as isize) as c_int == '(' as i32
    {
        return 0 ;
    }
    if ind > 0 
        && *rl_line_buffer.offset(ind as isize) as c_int == '(' as i32
        && member!(rl_line_buffer.offset(pind as isize),COMMAND_SEPARATORS!())
    {
        return 1 as c_int;
    }
    return 0 as c_int;
}

#[macro_export]
macro_rules! INITIALWORD {
    () => {
        b"_InitialWorD_\0" as *const u8 as *const c_char
    };
}

#[macro_export]
macro_rules! EMPTYCMD {
    () => {
        b"_EmptycmD_\0" as *const u8 as *const c_char
    };
}


unsafe extern "C" fn attempt_shell_completion(
    mut text: *const c_char,
    mut start: c_int,
    mut end: c_int,
) -> *mut *mut c_char {
    let mut in_command_position: c_int = 0;
    let mut ti: c_int = 0;
    let mut qc: c_int = 0;
    let mut dflags: c_int = 0;
    let mut matches: *mut *mut c_char = 0 as *mut *mut c_char;
    let mut command_separator_chars: *mut c_char = 0 as *mut c_char;
    let mut have_progcomps: c_int = 0;
    let mut was_assignment: c_int = 0;
    let mut iw_compspec: *mut COMPSPEC = 0 as *mut COMPSPEC;
    
    command_separator_chars = COMMAND_SEPARATORS!();
    matches = 0 as *mut *mut c_char;
    rl_ignore_some_completions_function = Some(
        filename_completion_ignore
            as unsafe extern "C" fn(*mut *mut c_char) -> c_int,
    );
    
    rl_filename_quote_characters = default_filename_quote_characters;
    set_filename_bstab(rl_filename_quote_characters);
    
    set_directory_hook();
    rl_filename_stat_hook = Some(
        bash_filename_stat_hook
            as unsafe extern "C" fn(*mut *mut c_char) -> c_int,
    );
    rl_sort_completion_matches = 1 ;
    
    ti = start - 1 as c_int;
    qc = -(1 as c_int);
    
    while ti > -(1 as c_int)
        && whitespace!(rl_line_buffer.offset(ti as isize))
    {
        ti -= 1;
    }
    
    if ti >= 0 
        && (*rl_line_buffer.offset(ti as isize) as c_int == '"' as i32
            || *rl_line_buffer.offset(ti as isize) as c_int == '\'' as i32)
    {
        qc = *rl_line_buffer.offset(ti as isize) as c_int;
        ti -= 1;
        while ti > -(1 as c_int)
            && whitespace!(rl_line_buffer.offset(ti as isize))
        {
            ti -= 1;
        }
    }
    
    in_command_position = 0 ;
    if ti < 0 {
        if current_prompt_string == ps1_prompt {
            in_command_position += 1;
        } else if parser_in_command_position() != 0 {
            in_command_position += 1;
        }
    } else if member!(rl_line_buffer.offset(ti as isize),command_separator_chars) {
        in_command_position += 1;
        if check_redir(ti) == 1 as c_int {
            in_command_position = 0 as c_int;
        }
    }

    if in_command_position != 0 && invalid_completion(text, ti) != 0 {
        rl_attempted_completion_over = 1 ;
        return 0 as *mut *mut c_char;
    }

    if in_command_position != 0 && ti >= 0
        && *rl_line_buffer.offset(ti as isize) as c_int == '`' as i32
        && *text as c_int != '`' as i32
        && unclosed_pair(
            rl_line_buffer,
            end,
            b"`\0" as *const u8 as *const c_char as *mut c_char,
        ) == 0 as c_int
    {
        in_command_position = 0 ;
    }
    if *text as c_int == '`' as i32 && rl_completion_quote_character != '\'' as i32
        && (in_command_position != 0
            || unclosed_pair(
                rl_line_buffer,
                start,
                b"`\0" as *const u8 as *const c_char as *mut c_char,
            ) != 0
                && unclosed_pair(
                    rl_line_buffer,
                    end,
                    b"`\0" as *const u8 as *const c_char as *mut c_char,
                ) != 0)
    {
        matches = rl_completion_matches(
            text,
            Some(
                command_subst_completion_function
                    as unsafe extern "C" fn(
                        *const c_char,
                        c_int,
                    ) -> *mut c_char,
            ),
        );
    }
    have_progcomps = (prog_completion_enabled != 0 && progcomp_size() > 0 as c_int)
        as c_int;
    iw_compspec = progcomp_search(
        INITIALWORD!()
    );
    if matches.is_null()
        && (in_command_position == 0 
            || *text.offset(0 as isize) as c_int == '\u{0}' as i32
            || in_command_position != 0 && !iw_compspec.is_null())
        && current_prompt_string == ps1_prompt
    {
        let mut s: c_int = 0;
        let mut e: c_int = 0;
        let mut s1: c_int = 0;
        let mut e1: c_int = 0;
        let mut os: c_int = 0;
        let mut foundcs: c_int = 0;
        let mut n: *mut c_char = 0 as *mut c_char;
        
        if !prog_complete_matches.is_null() {
            free(
                prog_complete_matches as *mut c_void
            );
        }
        prog_complete_matches = 0 as *mut *mut c_char;
        
        os = start;
        n = 0 as *mut c_char;
        was_assignment = 0 as c_int;
        s = find_cmd_start(os);
        e = find_cmd_end(end);
        
        loop {
            if s > rl_end {
                s = e1;
                s1 = s;
                break;
            } else if was_assignment != 0 && s > rl_point {
                s = e1;
                s1 = s;
                break;
            } 

            FREE!(n);
            n = find_cmd_name(s, &mut s1, &mut e1);
            s = e1 + 1 ;
            was_assignment = assignment(n, 0 as c_int);
            if !(was_assignment != 0) {
                break;
            }
            
        }
        
        s = s1;
        if start == 0 && end == 0 && e != 0 
            && *text.offset(0 as c_int as isize) as c_int == '\u{0}' as i32
        {
            foundcs = 0 ;
        } else if start == end && start == s1 && e != 0 && e1 > end {
            foundcs = 0 ;
        } else if e == 0 && e == s
                && *text.offset(0 as isize) as c_int
                    == '\u{0}' as i32 && have_progcomps != 0
            {
            prog_complete_matches = programmable_completions(
                EMPTYCMD!(),
                text,
                s,
                e,
                &mut foundcs,
            );
        } else if start == end
                && *text.offset(0 as isize) as c_int
                    == '\u{0}' as i32 && s1 > start
                && whitespace!(rl_line_buffer.offset(start as isize) )
        {
            foundcs = 0 ;
        } else if e > s && was_assignment == 0 as c_int && e1 == end
                && *rl_line_buffer.offset(e as isize) as c_int == 0 as c_int
                && whitespace!(rl_line_buffer.offset((e - 1 as c_int) as isize)) {
            foundcs = 0 ;
            in_command_position = (s == start
                && STREQ!(n, text)) as i32;
        } 
        else if e > s && was_assignment == 0 && have_progcomps != 0 {
            prog_complete_matches = programmable_completions(
                n,
                text,
                s,
                e,
                &mut foundcs,
            );

            in_command_position = (s == start
                && (!iw_compspec.is_null()
                    || STREQ!(n, text))) as c_int;
            if !iw_compspec.is_null() && in_command_position != 0 {
                foundcs = 0 ;
            }
        } 
        else if s >= e
                && *n.offset(0 as isize) as c_int == '\u{0}' as i32
                && *text.offset(0 as isize) as c_int
                    == '\u{0}' as i32 && start > 0 
                && was_assignment == 0 
                && member!(rl_line_buffer.offset((start - 1 as c_int).try_into().unwrap()), COMMAND_SEPARATORS!())
        {
            foundcs = 0 as c_int;
            in_command_position = 1 as c_int;
        } else if s >= e
                && *n.offset(0 as isize) as c_int == '\u{0}' as i32
                && *text.offset(0 as isize) as c_int
                    == '\u{0}' as i32 && start > 0 as c_int {
            foundcs = 0 ;
            in_command_position += was_assignment;
        } else if s == start && e == end
                && STREQ!(n, text)  
                && start > 0 as c_int
        {
            foundcs = 0 ;
            in_command_position = 1 ;
        } else {
            foundcs = 0 ;
        }
        if in_command_position != 0 && have_progcomps != 0 && foundcs == 0 as c_int
            && !iw_compspec.is_null()
        {
            prog_complete_matches = programmable_completions(
                INITIALWORD!(),
                text,
                s,
                e,
                &mut foundcs,
            );
        }

        FREE!(n);
        
        if foundcs != 0 {
            pcomp_set_readline_variables(foundcs, 1 as c_int);
            matches = rl_completion_matches(
                text,
                Some(prog_complete_return)
            );
            if foundcs & COPT_DEFAULT as c_int == 0 {
                rl_attempted_completion_over = 1 ;
            }
            if !matches.is_null()
                || foundcs & COPT_BASHDEFAULT as c_int == 0 
            {
                return matches;
            }
        }
    }
    if matches.is_null() {
        dflags = 0 ;
        if in_command_position != 0 {
            dflags |= DEFCOMP_CMDPOS!();
        }
        matches = bash_default_completion(text, start, end, qc, dflags);
    }
    return matches;
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
        absolute_pathname($x)
        == 0 as c_int
        && absolute_program($x)
            == 0 as c_int
        && *$x as c_int
            != '~' as i32
        && test_for_directory($x)
            != 0
    };
}

#[no_mangle]
pub unsafe extern "C" fn bash_default_completion(
    mut text: *const c_char,
    mut start: c_int,
    mut end: c_int,
    mut qc: c_int,
    mut compflags: c_int,
) -> *mut *mut c_char {
    let mut matches: *mut *mut c_char = 0 as *mut *mut c_char;
    let mut t: *mut c_char = 0 as *mut c_char;
    
    matches = 0 as *mut c_void as *mut *mut c_char;
    
    if *text as c_int == '$' as i32 {
        if qc != '\'' as i32
            && *text.offset(1 as isize) as c_int == '(' as i32
        {
            matches = rl_completion_matches(
                text,
                Some(
                    command_subst_completion_function
                        as unsafe extern "C" fn(
                            *const c_char,
                            c_int,
                        ) -> *mut c_char,
                ),
            );
        } else {
            matches = rl_completion_matches(
                text,
                Some(
                    variable_completion_function
                        as unsafe extern "C" fn(
                            *const c_char,
                            c_int,
                        ) -> *mut c_char,
                ),
            );
            if !matches.is_null()
                && !(*matches.offset(0 as isize)).is_null()
                && (*matches.offset(1 as isize)).is_null()
            {
                t = savestring!(*matches.offset(0 as c_int as isize));
                bash_filename_stat_hook(&mut t);
                if file_isdir(t) != 0 {
                    rl_completion_append_character = '/' as i32;
                }
                free(
                    t as *mut c_void
                );
            }
        }
    }
    if matches.is_null() && *text as c_int == '~' as i32
        && (mbschr(text, '/' as i32)).is_null()
    {
        matches = rl_completion_matches(
            text,
            Some(
                rl_username_completion_function
                    as unsafe extern "C" fn(
                        *const c_char,
                        c_int,
                    ) -> *mut c_char,
            ),
        );
    }
    
    if matches.is_null() && perform_hostname_completion != 0
        && *text as c_int == '@' as i32
    {
        matches = rl_completion_matches(
            text,
            Some(
                hostname_completion_function
                    as unsafe extern "C" fn(
                        *const c_char,
                        c_int,
                    ) -> *mut c_char,
            ),
        );
    }
    
    if matches.is_null() && compflags & DEFCOMP_CMDPOS!()  != 0 {
        if no_empty_command_completion != 0 && end == start
            && *text.offset(0 as c_int as isize) as c_int == '\u{0}' as i32
        {
            matches = 0 as *mut *mut c_char;
            rl_ignore_some_completions_function = Some(
                bash_ignore_everything
                    as unsafe extern "C" fn(*mut *mut c_char) -> c_int,
            );
        } else {
            dot_in_path = 0 ;
            matches = rl_completion_matches(
                text,
                Some(
                    command_word_completion_function
                        as unsafe extern "C" fn(
                            *const c_char,
                            c_int,
                        ) -> *mut c_char,
                ),
            );
            if matches.is_null() {
                rl_ignore_some_completions_function = Some(
                    bash_ignore_filenames
                        as unsafe extern "C" fn(*mut *mut c_char) -> c_int,
                );
            } else if (*matches.offset(1 as c_int as isize)).is_null()
                    && CMD_IS_DIR!(*matches.offset(0 as isize))  && dot_in_path == 0  
            {
                rl_completion_suppress_append = 1 ;
                rl_filename_completion_desired = 0 ;
            } else if !(*matches.offset(0 as isize)).is_null()
                    && !(*matches.offset(1 as isize)).is_null()
                    && STREQ!(*matches.offset(0 as isize) ,*matches.offset(1 as isize))
                    && CMD_IS_DIR!(*matches.offset(0 as isize))  
            {
                rl_completion_suppress_append = 1 as c_int;
                rl_filename_completion_desired = 0 as c_int;
            }
        }
    }
    if matches.is_null() && completion_glob_pattern(text as *mut c_char) != 0 {
        matches = rl_completion_matches(
            text,
            Some(
                glob_complete_word
                    as unsafe extern "C" fn(
                        *const c_char,
                        c_int,
                    ) -> *mut c_char,
            ),
        );
        
        if !matches.is_null() && !(*matches.offset(1 as isize)).is_null()
            && rl_completion_type == '\t' as i32
        {
            r_bash::strvec_dispose(matches);
            matches = 0 as *mut *mut c_char;
        } else if !matches.is_null()
                && !(*matches.offset(1 as c_int as isize)).is_null()
                && rl_completion_type == '!' as i32
            {
            rl_completion_suppress_append = 1 as c_int;
            rl_filename_completion_desired = 0 as c_int;
        }
    }
    return matches;
}

unsafe extern "C" fn bash_command_name_stat_hook(
    mut name: *mut *mut c_char,
) -> c_int {
    let mut cname: *mut c_char = 0 as *mut c_char;
    let mut result: *mut c_char = 0 as *mut c_char;
    if absolute_program(*name) != 0 {
        return bash_filename_stat_hook(name);
    }
    cname = *name;
    result = search_for_command(cname, 0 as c_int);
    if !result.is_null() {
        *name = result;
        return 1 ;
    }
    return 0 ;
}

unsafe extern "C" fn executable_completion(
    mut filename: *const c_char,
    mut searching_path: c_int,
) -> c_int {
    let mut f: *mut c_char = 0 as *mut c_char;
    let mut r: c_int = 0;
    
    f = savestring!(filename);
    
    bash_directory_completion_hook(&mut f);
    
    r = if searching_path != 0 {
        executable_file(f)
    } else {
        executable_or_directory(f)
    };
    free(
        f as *mut c_void,
    );
    return r;
}

#[macro_export]
macro_rules! TAB {
    () => {
        '\t' as i32
    };
}


#[no_mangle]
pub unsafe extern "C" fn command_word_completion_function(
    mut hint_text: *const c_char,
    mut state: c_int,
) -> *mut c_char {
    
    let mut match_0: c_int = 0;
    let mut freetemp: c_int = 0;
    let mut current_block: u64;
    static mut hint: *mut c_char = 0 as *mut c_char;
    static mut path: *mut c_char = 0 as *mut c_char;
    static mut val: *mut c_char = 0 as *mut c_char;
    static mut filename_hint: *mut c_char = 0 as *mut c_void as *mut c_char;
    static mut fnhint: *mut c_char = 0 as *mut c_char;
    static mut dequoted_hint: *mut c_char = 0 as *mut c_void as *mut c_char;
    static mut directory_part: *mut c_char = 0 as *mut c_void as *mut c_char;
    static mut glob_matches: *mut *mut c_char = 0 as *mut c_void as *mut *mut c_char;
    static mut path_index: c_int = 0;
    static mut hint_len: c_int = 0;
    static mut istate: c_int = 0;
    static mut igncase: c_int = 0;
    static mut mapping_over: c_int = 0;
    static mut local_index: c_int = 0;
    static mut searching_path: c_int = 0;
    static mut hint_is_dir: c_int = 0;
    static mut old_glob_ignore_case: c_int = 0;
    static mut globpat: c_int = 0;
    static mut varlist: *mut *mut SHELL_VAR = 0 as *mut *mut SHELL_VAR;
    static mut alias_list: *mut *mut alias_t = 0 as *mut *mut alias_t;
    let mut temp: *mut c_char = 0 as *mut c_char;
    let mut cval: *mut c_char = 0 as *mut c_char;
    
    if state == 0 {
        rl_filename_stat_hook = Some(bash_command_name_stat_hook);
        
        if !dequoted_hint.is_null() && dequoted_hint != hint {
            free(
                dequoted_hint as *mut c_void
            );
        }
        if !hint.is_null() {
            free(
                hint as *mut c_void
            );
        }

        searching_path = 0 ;
        mapping_over = searching_path;
        hint_is_dir = CMD_IS_DIR!(hint_text ) as i32;
        val = 0 as *mut c_char;
        
        temp = rl_variable_value(
            b"completion-ignore-case\0" as *const u8 as *const c_char,
        );
        igncase = RL_BOOLEAN_VARIABLE_VALUE!(temp) as c_int;
        
        if !glob_matches.is_null() {
            free(
                glob_matches as *mut c_void
            );
            glob_matches = 0 as *mut *mut c_char;
        }
        
        globpat = completion_glob_pattern(hint_text as *mut c_char);
        
        if globpat != 0 || absolute_program(hint_text) != 0 {
            if *hint_text as c_int == '~' as i32 {
                hint = bash_tilde_expand(hint_text, 0 );
                directory_part = savestring!(hint_text);
                temp = strchr(directory_part, '/' as i32);
                
                if !temp.is_null() {
                    *temp = 0 as c_char;
                } else {
                    free(directory_part as *mut c_void);
                    directory_part = 0 as *mut c_char;
                }
            } else if dircomplete_expand != 0 {
                hint = savestring!(hint_text);
                bash_directory_completion_hook(&mut hint);
            } else {
                hint = savestring!(hint_text);
            }

            dequoted_hint = hint;
            
            if rl_completion_found_quote != 0
                && rl_completion_quote_character == 0 
            {
                dequoted_hint = bash_dequote_filename(hint, 0 );
                free(hint as *mut c_void);
                hint = dequoted_hint;
            }
            hint_len = strlen(hint) as c_int;
            
            if !filename_hint.is_null() {
                free(filename_hint as *mut c_void);
            }
            
            filename_hint = savestring!(hint);
            fnhint = filename_hint;
            
            istate = 0 ;
            
            if globpat != 0 {
                mapping_over = 5 ;
                current_block = 6770491532623035343;  //globword
            } else {
                if dircomplete_expand != 0 && path_dot_or_dotdot(filename_hint) != 0 {
                    dircomplete_expand = 0 as c_int;
                    set_directory_hook();
                    dircomplete_expand = 1 as c_int;
                }
                mapping_over = 4 as c_int;
                current_block = 14702977349412626834;   //inner
            }
        } else {
            hint = savestring!(hint_text);
            dequoted_hint = hint;
            hint_len = strlen(hint) as c_int;
            
            if rl_completion_found_quote != 0
                && rl_completion_quote_character == 0 as c_int
            {
                dequoted_hint = bash_dequote_filename(hint, 0 as c_int);
            }
            
            path = get_string_value(b"PATH\0" as *const u8 as *const c_char);
            dot_in_path = 0 as c_int;
            path_index = dot_in_path;
            local_index = 0 as c_int;
            
            if !varlist.is_null() {
                free(varlist as *mut c_void);
            }
            
            varlist = all_visible_functions();
            if !alias_list.is_null() {
                free(alias_list as *mut c_void);
            }
            
            alias_list = all_aliases();
            current_block = 1874315696050160458;        //Normal
        }
    } else {
        current_block = 1874315696050160458;    //Normal
    }
    
    match current_block {
        1874315696050160458 => {        //Normal
            let mut current_block_92: u64;
            match mapping_over {
                0 => {
                    while !alias_list.is_null()
                        && !(*alias_list.offset(local_index as isize)).is_null()
                    {
                        let mut alias: *mut c_char = 0 as *mut c_char;

                        alias = (**alias_list.offset(local_index as isize)).name;
                        local_index = local_index + 1;
                        
                        if igncase == 0 
                            && STREQN!(alias, hint, hint_len) != 0
                        {
                            return savestring!(alias);
                        } else {
                            if igncase != 0
                                && strncasecmp(alias, hint, hint_len as usize)
                                    == 0 as c_int
                            {
                                return savestring!(alias);
                            }
                        }
                    }
                    local_index = 0 ;
                    mapping_over += 1;
                    
                    current_block_92 = 12999253957288907134;
                }
                1 => {
                    current_block_92 = 12999253957288907134;
                }
                2 => {
                    current_block_92 = 12548522231677580853;
                }
                3 => {
                    current_block_92 = 5440753863019323004;
                }
                _ => {
                    current_block_92 = 5706507068631705000;
                }
            }
            match current_block_92 {
                12999253957288907134 => {
                    while !((*word_token_alist.as_mut_ptr().offset(local_index as isize))
                        .word)
                        .is_null()
                    {
                        let mut reserved_word: *mut c_char = 0 as *mut c_char;

                        reserved_word = (*word_token_alist
                            .as_mut_ptr()
                            .offset(local_index as isize))
                            .word;
                        local_index = local_index + 1;

                        if STREQN!(reserved_word, hint, hint_len) != 0
                        {
                            return savestring!(reserved_word);
                        }
                    }
                    local_index = 0 as c_int;
                    mapping_over += 1;
                    current_block_92 = 12548522231677580853;
                }
                _ => {}
            }

            match current_block_92 {
                12548522231677580853 => {
                    while !varlist.is_null()
                        && !(*varlist.offset(local_index as isize)).is_null()
                    {
                        let mut varname: *mut c_char = 0 as *mut c_char;
            
                        varname = (**varlist.offset(local_index as isize)).name;
                        local_index = local_index + 1;
                        
                        if igncase == 0 
                            && STREQN!(varname, hint, hint_len) != 0
                        {
                            return savestring!(varname);
                        } else {
                            if igncase != 0
                                && strncasecmp(varname, hint, hint_len as usize)
                                    == 0 
                            {
                                return savestring!(varname);
                            }
                        }
                    }
                    local_index = 0 as c_int;
                    mapping_over += 1;
                    current_block_92 = 5440753863019323004;
                }
                _ => {}
            }
            match current_block_92 {
                5440753863019323004 => {
                    while local_index < num_shell_builtins {
                        if !(((*shell_builtins.offset(local_index as isize)).function)
                            .is_none()
                            || (*shell_builtins.offset(local_index as isize)).flags
                                & BUILTIN_ENABLED as c_int == 0 as c_int)
                        {
                            if STREQN!((*shell_builtins.offset(local_index as isize)).name, hint, hint_len) != 0
                            {
                                let mut i: c_int = local_index;
                                local_index = local_index + 1;
                                
                                return savestring!((*shell_builtins.offset(i as isize)).name);
                            }
                        }

                        local_index += 1;
                    }
                    local_index = 0 as c_int;
                    mapping_over += 1;
                }
                _ => {}
            }
            current_block = 6770491532623035343;
        }
        _ => {}
    }
    match current_block {
        6770491532623035343 => {
            if globpat != 0 {
                if state == 0 as c_int {
                    glob_ignore_case = igncase;
                    glob_matches = shell_glob_filename(hint, 0 as c_int);
                    glob_ignore_case = old_glob_ignore_case;
                    
                    if glob_matches == &mut glob_error_return as *mut *mut c_char
                        || glob_matches.is_null()
                    {
                        glob_matches = 0 as *mut *mut c_char;
                        return 0 as *mut c_char;
                    }
                    
                    local_index = 0 ;
                   
                    if !(*glob_matches.offset(1 as c_int as isize)).is_null()
                        && rl_completion_type == TAB!()
                    {
                        return 0 as *mut c_char;
                    }
                }
               
                loop {
                    val = *glob_matches.offset(local_index as isize);
                    local_index = local_index + 1;
                    if val.is_null() {
                        break;
                    }
                    if executable_or_directory(val) != 0 {
                        if *hint_text as c_int == '~' as i32
                            && !directory_part.is_null()
                        {
                            temp = maybe_restore_tilde(val, directory_part);
                            free(val as *mut c_void);
                            val = temp;
                        }
                        return val;
                    }
                    free(val as *mut c_void );
                }
                glob_ignore_case = old_glob_ignore_case;
                return 0 as *mut c_char;
            }

            if hint_is_dir != 0 {
                hint_is_dir = 0 ;
                return savestring!(hint_text);
            }
            current_block = 14531478163722833811;
        }
        _ => {}
    }
    
    loop {
        match current_block {
            14702977349412626834 => {
                val = rl_filename_completion_function(fnhint, istate);
                if mapping_over == 4 as c_int && dircomplete_expand != 0 {
                    set_directory_hook();
                }
                
                istate = 1 ;
                
                if val.is_null() {
                    if absolute_program(hint) != 0 {
                        return 0 as *mut c_char;
                    }
                    current_block = 14531478163722833811;
                } else {
                    match_0 = 0;
                    freetemp = 0;
                    if absolute_program(hint) != 0 {
                        if igncase == 0 {
                            match_0 = (strncmp(val, hint, hint_len as usize)
                                == 0 ) as c_int;
                        } else {
                            match_0 = (strncasecmp(val, hint, hint_len as usize)
                                == 0 ) as c_int;
                        }
                        if *hint_text as c_int == '~' as i32 {
                            temp = maybe_restore_tilde(val, directory_part);
                        } else {
                            temp = savestring!(val);
                        }
                        freetemp = 1 ;
                    } else {
                        temp = strrchr(val, '/' as i32);
                        if !temp.is_null() {
                            temp = temp.offset(1);
                            if igncase == 0 {
                                match_0 = (strncmp(temp, hint, hint_len as usize)
                                    == 0 ) as c_int;
                                freetemp = match_0;
                            } else {
                                match_0 = (strncasecmp(
                                    temp,
                                    hint,
                                    hint_len as usize,
                                ) == 0 as c_int) as c_int;
                                freetemp = match_0;
                            }
                            if match_0 != 0 {
                                temp = savestring!(temp);
                            }
                        } else {
                            match_0 = 0 as c_int;
                            freetemp = match_0;
                        }
                    }
                    cval = val;
                    if match_0 != 0
                        && executable_completion(
                            (if searching_path != 0 { val } else { cval }),
                            searching_path,
                        ) != 0
                    {
                        break;
                    }
                    if freetemp != 0 {
                        free(temp as *mut c_void);
                    }
                    if cval != val {
                        free(cval as *mut c_void);
                    }
                    free(val as *mut c_void);
                    current_block = 14702977349412626834;
                }
            }
            _ => {
                istate = (val != 0 as *mut c_char) as c_int;
                
                if istate == 0 {
                    let mut current_path: *mut c_char = 0 as *mut c_char;
                    
                    if path.is_null()
                        || *path.offset(path_index as isize) as c_int
                            == 0  
                        || {
                            current_path = extract_colon_unit(path, &mut path_index);
                            current_path.is_null()
                        }
                    {
                        return 0 as *mut c_void as *mut c_char;
                    }
                    searching_path = 1 ;
                    if *current_path as c_int == 0 as c_int {
                        free(current_path as *mut c_void);
                        current_path = savestring!(b".\0" as *const u8 as *const c_char);
                    }
                    if *current_path as c_int == '~' as i32 {
                        let mut t: *mut c_char = 0 as *mut c_char;
                        
                        t = bash_tilde_expand(current_path, 0 as c_int);
                        free(current_path as *mut c_void);
                        current_path = t;
                    }
                    
                    if *current_path.offset(0 as isize) as c_int
                        == '.' as i32
                        && *current_path.offset(1 as isize) as c_int
                            == '\u{0}' as i32
                    {
                        dot_in_path = 1 ;
                    }
                    if !fnhint.is_null() && fnhint != filename_hint {
                        free(fnhint as *mut c_void);
                    }
                    if !filename_hint.is_null() {
                        free(filename_hint as *mut c_void);
                    }
                    filename_hint = sh_makepath(current_path, hint, 0 );
                    if !(strpbrk(
                        filename_hint,
                        b"\"'\\\0" as *const u8 as *const c_char,
                    ))
                        .is_null()
                    {
                        fnhint = sh_backslash_quote(
                            filename_hint,
                            filename_bstab.as_mut_ptr(),
                            0 as c_int,
                        );
                    } else {
                        fnhint = filename_hint;
                    }
                    free(current_path as *mut c_void);
                }
                current_block = 14702977349412626834;
            }
        }
    }
    if cval != val {
        free(cval as *mut c_void);
    }
    free(val as *mut c_void);
    val = b"\0" as *const u8 as *const c_char as *mut c_char;
    return temp;
}


unsafe extern "C" fn command_subst_completion_function(
    mut text: *const c_char,
    mut state: c_int,
) -> *mut c_char {
    static mut matches: *mut *mut c_char = 0 as *const c_void
        as *mut c_void as *mut *mut c_char;
    static mut orig_start: *const c_char = 0 as *const c_char;
    static mut filename_text: *mut c_char = 0 as *const c_void
        as *mut c_void as *mut c_char;
    static mut cmd_index: c_int = 0;
    static mut start_len: c_int = 0;
    let mut value: *mut c_char = 0 as *mut c_char;
    
    if state == 0 as c_int {
        if !filename_text.is_null() {
            free(
                filename_text as *mut c_void
            );
        }
        orig_start = text;
        if *text as c_int == '`' as i32 {
            text = text.offset(1);
        } else if *text as c_int == '$' as i32
                && *text.offset(1 as c_int as isize) as c_int == '(' as i32
            {
            text = text.offset(2 as c_int as isize);
        }
        
        rl_completion_suppress_quote = 1 as c_int;
        start_len = text.offset_from(orig_start) as libc::c_long as c_int;
        filename_text = savestring!(text);       
        if !matches.is_null() {
            free(
                matches as *mut c_void,
            );
        }
        
        value = filename_text
            .offset(strlen(filename_text) as isize)
            .offset(-(1 as c_int as isize));
        while value > filename_text {
            if whitespace!(*value) || member!(value, COMMAND_SEPARATORS!())
            {
                break;
            }
            value = value.offset(-1);
        }
        
        if value <= filename_text {
            matches = rl_completion_matches(
                filename_text,
                Some(command_word_completion_function)
            );
        } else {
            value = value.offset(1);
            start_len = (start_len as libc::c_long
                + value.offset_from(filename_text) as libc::c_long) as c_int;
            if whitespace!(value.offset(-(1 as c_int) as isize))
            {
                matches = rl_completion_matches(
                    value,
                    Some(
                        rl_filename_completion_function
                            as unsafe extern "C" fn(
                                *const c_char,
                                c_int,
                            ) -> *mut c_char,
                    ),
                );
            } else {
                matches = rl_completion_matches(
                    value,
                    Some(command_word_completion_function)
                );
            }
        }
        
        cmd_index = (!matches.is_null()
            && !(*matches.offset(0 as isize)).is_null()
            && !(*matches.offset(1 as isize)).is_null()) as c_int;
        
        if !matches.is_null() && !(*matches.offset(0 as isize)).is_null()
            && (*matches.offset(1 as c_int as isize)).is_null()
            && test_for_directory(*matches.offset(0 as isize)) != 0
        {
            rl_completion_append_character = '/' as i32;
        } else {
            rl_completion_suppress_append = 1 as c_int;
        }
    }
    
    if matches.is_null() || (*matches.offset(cmd_index as isize)).is_null() {
        rl_filename_quoting_desired = 0 ;
        return 0 as *mut c_char;
    } else {
        value = malloc( (1 + start_len + strlen(*matches.offset(cmd_index as isize)) as i32).try_into().unwrap()
        ) as *mut c_char;
        if start_len == 1 {
            *value.offset(0 as isize) = *orig_start;
        } else {
            strncpy(value, orig_start, (start_len as libc::c_ulong).try_into().unwrap());
        }
        
        strcpy(value.offset(start_len as isize), *matches.offset(cmd_index as isize));
        
        cmd_index += 1;
        return value;
    };
}

unsafe extern "C" fn variable_completion_function(
    mut text: *const c_char,
    mut state: c_int,
) -> *mut c_char {
    static mut varlist: *mut *mut c_char = 0 as *const c_void
        as *mut c_void as *mut *mut c_char;
    static mut varlist_index: c_int = 0;
    static mut varname: *mut c_char = 0 as *const c_void as *mut c_void
        as *mut c_char;
    static mut first_char: c_int = 0;
    static mut first_char_loc: c_int = 0;
    
    if state == 0 {
        if !varname.is_null() {
            free(
                varname as *mut c_void,
            );
        }
        
        first_char_loc = 0 as c_int;
        first_char = *text.offset(0 as isize) as c_int;
        
        if first_char == '$' as i32 {
            first_char_loc += 1;
        }
        
        if *text.offset(first_char_loc as isize) as c_int == '{' as i32 {
            first_char_loc += 1;
        }
        varname = savestring!(text.offset(first_char_loc as isize));
        
        if !varlist.is_null() {
            strvec_dispose(varlist);
        }
        
        varlist = all_variables_matching_prefix(varname);
        varlist_index = 0 ;
    }
    
    if varlist.is_null() || (*varlist.offset(varlist_index as isize)).is_null() {
        return 0 as *mut c_char
    } else {
        let mut value: *mut c_char = 0 as *mut c_char;
        value = malloc(((4 + strlen(*varlist.offset(varlist_index as isize)))).try_into().unwrap()) as *mut c_char;
        
        if first_char_loc != 0 {
            *value.offset(0 as isize) = first_char as c_char;
            if first_char_loc == 2 {
                *value.offset(1 as isize) = '{' as i32 as c_char;
            }
        }
        
        strcpy(
            value.offset(first_char_loc as isize),
            *varlist.offset(varlist_index as isize),
        );
        if first_char_loc == 2 as c_int {
            strcat(value, b"}\0" as *const u8 as *const c_char);
        }
        
        varlist_index += 1;
        return value;
    };
}

unsafe extern "C" fn hostname_completion_function(
    mut text: *const c_char,
    mut state: c_int,
) -> *mut c_char {
    static mut list: *mut *mut c_char = 0 as *const c_void
        as *mut c_void as *mut *mut c_char;
    static mut list_index: c_int = 0 as c_int;
    static mut first_char: c_int = 0;
    static mut first_char_loc: c_int = 0;
    
    if state == 0 {
        FREE!(list);
        
        list = 0 as *mut *mut c_char;
        
        first_char_loc = 0 ;
        first_char = *text as c_int;
        
        if first_char == '@' as i32 {
            first_char_loc += 1;
        }
        
        list = hostnames_matching(
            (text as *mut c_char).offset(first_char_loc as isize),
        );
        list_index = 0 as c_int;
    }
    
    if !list.is_null() && !(*list.offset(list_index as isize)).is_null() {
        let mut t: *mut c_char = 0 as *mut c_char;
        
        t = malloc(
            (2 as c_int as libc::c_ulong)
                .wrapping_add(strlen(*list.offset(list_index as isize))).try_into().unwrap()
        ) as *mut c_char;
        
        *t = first_char as c_char;
        strcpy(t.offset(first_char_loc as isize), *list.offset(list_index as isize));
        list_index += 1;
        return t;
    }
    return 0 as *mut c_char;
}

#[no_mangle]
pub unsafe extern "C" fn bash_servicename_completion_function(
    mut text: *const c_char,
    mut state: c_int,
) -> *mut c_char {
    static mut sname: *mut c_char = 0 as *const c_void as *mut c_void
        as *mut c_char;
    static mut srvent: *mut servent = 0 as *const servent as *mut servent;
    static mut snamelen: c_int = 0;
    let mut value: *mut c_char = 0 as *mut c_char;
    let mut alist: *mut *mut c_char = 0 as *mut *mut c_char;
    let mut aentry: *mut c_char = 0 as *mut c_char;
    let mut afound: c_int = 0;
    
    if state == 0 as c_int {
        FREE!(sname);
        
        sname = savestring!(text);
        snamelen = strlen(sname) as c_int;
        setservent(0 as c_int);
    }
    
    loop {
        srvent = getservent();
        if srvent.is_null() {
            break;
        }
       
        afound = 0 ;
        if snamelen == 0 
            || STREQN!(sname,(*srvent).s_name,snamelen) != 0
        {
            break;
        }
        
        
        alist = (*srvent).s_aliases;
        while !(*alist).is_null() {
            aentry = *alist;
            if STREQN!(sname, aentry, snamelen ) != 0
            {
                afound = 1 ;
                break;
            } else {
                alist = alist.offset(1);
            }
        }
        if afound != 0 {
            break;
        }
    }
    
    if srvent.is_null() {
        endservent();
        return 0 as *mut c_char;
    }
    value = if afound != 0 {
        savestring!(aentry)
    } else {
        savestring!((*srvent).s_name)
    };
    return value;
}


#[no_mangle]
pub unsafe extern "C" fn bash_groupname_completion_function(
    mut text: *const c_char,
    mut state: c_int,
) -> *mut c_char {
    static mut gname: *mut c_char = 0 as *const c_void as *mut c_void
        as *mut c_char;
    static mut grent: *mut group = 0 as *const group as *mut group;
    static mut gnamelen: c_int = 0;
    let mut value: *mut c_char = 0 as *mut c_char;
    
    if state == 0 {
        FREE!(gname);

        gname = savestring!(text);
        gnamelen = strlen(gname) as c_int;
        
        setgrent();
    }
    
    loop {
        grent = getgrent();
        if grent.is_null() {
            break;
        }
        if gnamelen == 0 
            || STREQN!(gname, (*grent).gr_name, gnamelen) != 0
        {
            break;
        }
    }
    if grent.is_null() {
        endgrent();
        return 0 as *mut c_void as *mut c_char;
    }
    value = savestring!((*grent).gr_name);
    
    return value;
}


unsafe extern "C" fn history_expand_line_internal(
    mut line: *mut c_char,
) -> *mut c_char {
    let mut new_line: *mut c_char = 0 as *mut c_char;
    let mut old_verify: c_int = 0;
    
    old_verify = hist_verify;
    hist_verify = 0 as c_int;
    new_line = pre_process_line(line, 0 as c_int, 0 as c_int);
    hist_verify = old_verify;
   
    return if new_line == line {
        savestring!(line)
    } else {
        new_line
    };
}
unsafe extern "C" fn cleanup_expansion_error() {
    let mut to_free: *mut c_char = 0 as *mut c_char;
    let mut old_verify: c_int = 0;
    
    old_verify = hist_verify;
    hist_verify = 0 as c_int;
    
    fprintf(rl_outstream, b"\r\n\0" as *const u8 as *const c_char);
    to_free = pre_process_line(rl_line_buffer, 1 , 0 );
    
    hist_verify = old_verify;
    
    if to_free != rl_line_buffer {
        FREE!(to_free);
    }
    putc('\r' as i32, rl_outstream);
    rl_forced_update_display();
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

unsafe extern "C" fn maybe_make_readline_line(mut new_line: *mut c_char) {
    if !new_line.is_null() && strcmp(new_line, rl_line_buffer) != 0 as c_int {
        rl_point = rl_end;
        rl_add_undo(
            UNDO_BEGIN!(),
            0 ,
            0 ,
            0 as *mut c_char,
        );
        rl_delete_text(0 as c_int, rl_point);
        rl_mark = 0 as c_int;
        rl_end = rl_mark;
        rl_point = rl_end;
        rl_insert_text(new_line);
        rl_add_undo(
            UNDO_END!(),
            0 ,
            0 ,
            0 as *mut c_char,
        );
    }
}

unsafe extern "C" fn set_up_new_line(mut new_line: *mut c_char) {
    let mut old_point: c_int = 0;
    let mut at_end: c_int = 0;
    
    old_point = rl_point;
    at_end = (rl_point == rl_end) as c_int;
    
    maybe_make_readline_line(new_line);
    free(
        new_line as *mut c_void
    );
    
    if at_end != 0 {
        rl_point = rl_end;
    } else if old_point < rl_end {
        rl_point = old_point;
        if !whitespace!(*rl_line_buffer.offset(rl_point as isize))
        {
            rl_forward_word(1 as c_int, 0 as c_int);
        }
    }
}

unsafe extern "C" fn alias_expand_line(
    mut count: c_int,
    mut ignore: c_int,
) -> c_int {
    let mut new_line: *mut c_char = 0 as *mut c_char;
    new_line = alias_expand(rl_line_buffer);
    if !new_line.is_null() {
        set_up_new_line(new_line);
        return 0 ;
    } else {
        cleanup_expansion_error();
        return 1 ;
    };
}

unsafe extern "C" fn history_expand_line(
    mut count: c_int,
    mut ignore: c_int,
) -> c_int {
    let mut new_line: *mut c_char = 0 as *mut c_char;
    
    new_line = history_expand_line_internal(rl_line_buffer);
    
    if !new_line.is_null() {
        set_up_new_line(new_line);
        return 0 ;
    } else {
        cleanup_expansion_error();
        return 1 ;
    };
}

unsafe extern "C" fn tcsh_magic_space(
    mut count: c_int,
    mut ignore: c_int,
) -> c_int {
    let mut dist_from_end: c_int = 0;
    let mut old_point: c_int = 0;
    
    old_point = rl_point;
    dist_from_end = rl_end - rl_point;
    
    if history_expand_line(count, ignore) == 0 {
        rl_point = if old_point == 0 {
            old_point
        } else {
            rl_end - dist_from_end
        };
        rl_insert(1 , ' ' as i32);
        return 0 ;
    } else {
        return 1 
    };
}

unsafe extern "C" fn history_and_alias_expand_line(
    mut count: c_int,
    mut ignore: c_int,
) -> c_int {
    let mut new_line: *mut c_char = 0 as *mut c_char;
   
    new_line = 0 as *mut c_char;
    new_line = history_expand_line_internal(rl_line_buffer);
   
    if !new_line.is_null() {
        let mut alias_line: *mut c_char = 0 as *mut c_char;
        alias_line = alias_expand(new_line);
        free(
            new_line as *mut c_void
        );
        new_line = alias_line;
    }
    
    if !new_line.is_null() {
        set_up_new_line(new_line);
        return 0 ;
    } else {
        cleanup_expansion_error();
        return 1 ;
    };
}

#[macro_export]
macro_rules! Q_HERE_DOCUMENT {
    () => {
        0x002
    };
}

unsafe extern "C" fn shell_expand_line(
    mut count: c_int,
    mut ignore: c_int,
) -> c_int {
    let mut new_line: *mut c_char = 0 as *mut c_char;
    let mut expanded_string: *mut WordList = 0 as *mut WordList;
    let mut w: *mut WordDesc = 0 as *mut WordDesc;
    
    new_line = 0 as *mut c_char;
    new_line = history_expand_line_internal(rl_line_buffer);
    
    if !new_line.is_null() {
        let mut alias_line: *mut c_char = 0 as *mut c_char;
        alias_line = alias_expand(new_line);
        free(
            new_line as *mut c_void
        );
        new_line = alias_line;
    }
    if !new_line.is_null() {
        let mut old_point: c_int = rl_point;
        let mut at_end: c_int = (rl_point == rl_end) as c_int;
        
        maybe_make_readline_line(new_line);
        free(
            new_line as *mut c_void
        );
        
        w = alloc_word_desc();
        let ref mut fresh16 = (*w).word;
        (*w).word = savestring!(rl_line_buffer);
        (*w).flags = if rl_explicit_arg != 0 {
            (W_NOPROCSUB as c_int) | (W_NOCOMSUB as c_int)
        } else {
            0 
        };
        
        expanded_string = expand_word(
            w,
            if rl_explicit_arg != 0 { Q_HERE_DOCUMENT!()} else { 0 as c_int },
        );
        dispose_word(w);
        
        if expanded_string.is_null() {
            new_line = malloc(
                1 as usize
            ) as *mut c_char;
            *new_line.offset(0 as isize) = '\u{0}' as i32 as c_char;
        } else {
            new_line = string_list(expanded_string);
            dispose_words(expanded_string);
        }
        
        maybe_make_readline_line(new_line);
        free(
            new_line as *mut c_void
        );
        
        if at_end != 0 {
            rl_point = rl_end;
        } else if old_point < rl_end {
            rl_point = old_point;
            if !whitespace!(*rl_line_buffer.offset(rl_point as isize)) 
            {
                rl_forward_word(1 , 0 );
            }
        }
        return 0;
    } else {
        cleanup_expansion_error();
        return 1 ;
    };
}

static mut fignore: ignorevar = {
    let mut init = ignorevar {
        varname: b"FIGNORE\0" as *const u8 as *const c_char as *mut c_char,
        ignores: 0 as *const ign as *mut ign,
        num_ignores: 0 ,
        last_ignoreval: 0 as *mut c_char,
        item_func: None,
    };
    init
};

unsafe extern "C" fn _ignore_completion_names(
    mut names: *mut *mut c_char,
    mut name_func: Option::<sh_ignore_func_t>,
) {
    let mut newnames: *mut *mut c_char = 0 as *mut *mut c_char;
    let mut idx: c_int = 0;
    let mut nidx: c_int = 0;
    let mut oldnames: *mut *mut c_char = 0 as *mut *mut c_char;
    let mut oidx: c_int = 0;
    
    if (*names.offset(1 as isize)).is_null() {
        if force_fignore != 0 {
            if (Some(name_func.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                ) == None       //可能会出现问题
            {
                free(
                    *names.offset(0 as c_int as isize) as *mut c_void
                );
                let ref mut fresh17 = *names.offset(0 as isize);
                *names.offset(0 as isize) = 0 as *mut c_char;
            }
        }
        return;
    }

    nidx = 1 ;
    while !(*names.offset(nidx as isize)).is_null() {
        nidx += 1;
    }

    newnames = strvec_create(nidx + 1 );
    
    if force_fignore == 0 {
        oldnames = strvec_create(nidx - 1 );
        oidx = 0 ;
    }
    
    
    *newnames.offset(0 as isize) = *names.offset(0 as isize);
    nidx = 1 ;
    idx = nidx;
    while !(*names.offset(idx as isize)).is_null() {
        if (Some(name_func.expect("non-null function pointer")))
            .expect("non-null function pointer") != None
        {
            *(*newnames).offset(nidx as isize) = *(*names).offset(idx as isize);
            nidx = nidx + 1;
        } else if force_fignore == 0 {
            *(*oldnames).offset(oidx as isize) = *(*names).offset(idx as isize);
            oidx = oidx + 1;
        } else {
            free(
                *names.offset(idx as isize) as *mut c_void
            );
        }
        idx += 1;
    }

    *newnames.offset(nidx as isize) = 0 as *mut c_char;
    
    if nidx == 1 {
        if force_fignore != 0 {
            free(
                *names.offset(0 as isize) as *mut c_void
            );
            *names.offset(0 as isize) = 0 as *mut c_char;
        } else {
            free( oldnames as *mut c_void );
        }
        free(newnames as *mut c_void);
        return;
    }

    if force_fignore == 0 {
        while oidx != 0 {
            oidx -= 1;
            free(*oldnames.offset(oidx as isize) as *mut c_void);
        }
        free(oldnames as *mut c_void);
    }
    
    if nidx == 2 {
        free(*names.offset(0 as isize) as *mut c_void);
        *names.offset(0 as isize) = *newnames.offset(1 as isize);
        *names.offset(1 as isize) = 0 as *mut c_char;
        free(newnames as *mut c_void);
        return;
    }
    
    nidx = 1 ;
    while !(*newnames.offset(nidx as isize)).is_null() {
        *names.offset(nidx as isize) = *newnames.offset(nidx as isize);
        nidx += 1;
    }
    
    *names.offset(nidx as isize) = 0 as *mut c_char;
    free(newnames as *mut c_void);
}

unsafe extern "C" fn name_is_acceptable(mut name: *const c_char) -> c_int {
    let mut p: *mut ign = 0 as *mut ign;
    let mut nlen: c_int = 0;
    
    nlen = strlen(name) as c_int;
    p = fignore.ignores;
    while !((*p).val).is_null() {
        if nlen > (*p).len && (*p).len > 0 
            && (*((*p).val).offset(0 as isize) as c_int
                == *(&*name.offset((nlen - (*p).len) as isize) as *const c_char)
                    .offset(0 as isize) as c_int
                && strcmp((*p).val, &*name.offset((nlen - (*p).len) as isize))
                    == 0 as c_int)
        {
            return 0 ;
        }
        p = p.offset(1);
    }
    return 1 as c_int;
}

unsafe extern "C" fn filename_completion_ignore(
    mut names: *mut *mut c_char,
) -> c_int {
    setup_ignore_patterns(&mut fignore);
    
    if fignore.num_ignores == 0 {
        return 0 ;
    }
    
    _ignore_completion_names(
        names,
        Some(Some(name_is_acceptable))
    );
    return 0 ;
}

unsafe extern "C" fn test_for_directory(mut name: *const c_char) -> c_int {
    let mut fn_0: *mut c_char = 0 as *mut c_char;
    let mut r: c_int = 0;
    
    fn_0 = bash_tilde_expand(name, 0 );
    r = file_isdir(fn_0);
    free(fn_0 as *mut c_void);
    return r;
}

unsafe extern "C" fn test_for_canon_directory(
    mut name: *const c_char,
) -> c_int {
    let mut fn_0: *mut c_char = 0 as *mut c_char;
    let mut r: c_int = 0;
    
    fn_0 = if *name as c_int == '~' as i32 {
        bash_tilde_expand(name, 0 )
    } else {
        savestring!(name)
    };
    bash_filename_stat_hook(&mut fn_0);
    r = file_isdir(fn_0);
    free(
        fn_0 as *mut c_void
    );
    return r;
}

unsafe extern "C" fn bash_ignore_filenames(
    mut names: *mut *mut c_char,
) -> c_int {
    _ignore_completion_names(
        names,
        Some(Some(test_for_directory))
    );
    return 0 ;
}

unsafe extern "C" fn bash_progcomp_ignore_filenames(
    mut names: *mut *mut c_char,
) -> c_int {
    _ignore_completion_names(
        names,
        Some(Some(test_for_canon_directory))
    );
    return 0;
}

unsafe extern "C" fn return_zero(mut name: *const c_char) -> c_int {
    return 0 ;
}

unsafe extern "C" fn bash_ignore_everything(
    mut names: *mut *mut c_char,
) -> c_int {
    _ignore_completion_names(
        names,
        Some(Some(return_zero))
    );
    return 0 ;
}

#[macro_export]
macro_rules! STRLEN {
    ($s:expr) => {
        (if !$s.is_null() && *$s.offset(0 as c_int as isize) as c_int != 0
        {
            if *$s.offset(1 as c_int as isize) as c_int != 0 {
                if *$s.offset(2 as c_int as isize) as c_int != 0 {
                    strlen($s)
                } else {
                    2 as c_int as libc::c_ulong
                }
            } else {
                1 as c_int as libc::c_ulong
            }
        } else {
            0 as c_int as libc::c_ulong
        }) as c_int;
    };
}

unsafe extern "C" fn restore_tilde(
    mut val: *mut c_char,
    mut directory_part: *mut c_char,
) -> *mut c_char {
    let mut l: c_int = 0;
    let mut vl: c_int = 0;
    let mut dl2: c_int = 0;
    let mut xl: c_int = 0;
    let mut dh2: *mut c_char = 0 as *mut c_char;
    let mut expdir: *mut c_char = 0 as *mut c_char;
    let mut ret: *mut c_char = 0 as *mut c_char;
    let mut v: *mut c_char = 0 as *mut c_char;
    
    vl = strlen(val) as c_int;
    dh2 = if !directory_part.is_null() {
        bash_dequote_filename(directory_part, 0 )
    } else {
        0 as *mut c_char
    };
    bash_directory_expansion(&mut dh2);
    dl2 = strlen(dh2) as c_int;
    
    expdir = bash_tilde_expand(directory_part, 0 );
    xl = strlen(expdir) as c_int;
    if *directory_part as c_int == '~' as i32
        && STREQ!(directory_part , expdir )
    {
        v = mbschr(val, '/' as i32);
        vl = STRLEN!(v) as c_int;
        ret = malloc(((xl + vl + 2 as c_int) as size_t).try_into().unwrap()) as *mut c_char;
        strcpy(ret, directory_part);
        if !v.is_null() && *v as c_int != 0 {
            strcpy(ret.offset(xl as isize), v);
        }
        
        free(dh2 as *mut c_void);
        free(expdir as *mut c_void);
        return ret;
    }
    
    free(expdir as *mut c_void);
    
    l = vl - xl + 1 ;
    if l <= 0 {
        free( dh2 as *mut c_void);
        return savestring!(val);
    }
    
    ret = malloc(((dl2 + 2 as c_int + l) as size_t).try_into().unwrap()) as *mut c_char;
    strcpy(ret, dh2);
    strcpy(ret.offset(dl2 as isize), val.offset(xl as isize));
    
    free(dh2 as *mut c_void);
    return ret;
}

unsafe extern "C" fn maybe_restore_tilde(
    mut val: *mut c_char,
    mut directory_part: *mut c_char,
) -> *mut c_char {
    let mut save: rl_icppfunc_t = None;
    let mut ret: *mut c_char = 0 as *mut c_char;
    save = if dircomplete_expand == 0 as c_int {
        save_directory_hook()
    } else {
        None
    };
    ret = restore_tilde(val, directory_part);
    if save.is_some() {
        restore_directory_hook(save);
    }
    return ret;
}

unsafe extern "C" fn bash_directory_expansion(mut dirname: *mut *mut c_char) {
    let mut d: *mut c_char = 0 as *mut c_char;
    let mut nd: *mut c_char = 0 as *mut c_char;
    
    d = savestring!(*dirname);
    if rl_directory_rewrite_hook.is_some()
        && (Some(rl_directory_rewrite_hook.expect("non-null function pointer")))
            .expect("non-null function pointer")(&mut d) != 0
    {
        free(*dirname as *mut c_void);
        *dirname = d;
    } else if rl_directory_completion_hook.is_some()
            && (Some(rl_directory_completion_hook.expect("non-null function pointer")))
                .expect("non-null function pointer")(&mut d) != 0
        {
        free(*dirname as *mut c_void);
        *dirname = d;
    } else if rl_completion_found_quote != 0 {
        nd = bash_dequote_filename(d, rl_completion_quote_character);
        free(*dirname as *mut c_void);
        free(d as *mut c_void);
        *dirname = nd;
    }
}

unsafe extern "C" fn bash_filename_rewrite_hook(
    mut fname: *mut c_char,
    mut fnlen: c_int,
) -> *mut c_char {
    let mut conv: *mut c_char = 0 as *mut c_char;
    
    conv = fnx_fromfs(fname, (fnlen as size_t).try_into().unwrap());
    if conv != fname {
        conv = savestring!(conv);
    }
    return conv;
}

#[no_mangle]
pub unsafe extern "C" fn set_directory_hook() {
    if dircomplete_expand != 0 {
        rl_directory_completion_hook = Some(
            bash_directory_completion_hook
                as unsafe extern "C" fn(*mut *mut c_char) -> c_int,
        );
        rl_directory_rewrite_hook = None;
    } else {
        rl_directory_rewrite_hook = Some(
            bash_directory_completion_hook
                as unsafe extern "C" fn(*mut *mut c_char) -> c_int,
        );
        rl_directory_completion_hook = None;
    };
}

unsafe extern "C" fn save_directory_hook() -> rl_icppfunc_t {
    let mut ret: rl_icppfunc_t = None;
    if dircomplete_expand != 0 {
        ret = rl_directory_completion_hook;
        rl_directory_completion_hook = ::std::mem::transmute::<
            *mut c_void,
            rl_icppfunc_t,
        >(0 as *mut c_void);
    } else {
        ret = rl_directory_rewrite_hook;
        rl_directory_rewrite_hook = ::std::mem::transmute::<
            *mut c_void,
            rl_icppfunc_t,
        >(0 as *mut c_void);
    }
    return ret;
}

unsafe extern "C" fn restore_directory_hook(mut hookf: rl_icppfunc_t) {
    if dircomplete_expand != 0 {
        rl_directory_completion_hook = hookf;
    } else {
        rl_directory_rewrite_hook = hookf;
    }; 
}

unsafe extern "C" fn directory_exists(
    mut dirname: *const c_char,
    mut should_dequote: c_int,
) -> c_int {
    let mut new_dirname: *mut c_char = 0 as *mut c_char;
    let mut dirlen: c_int = 0;
    let mut r: c_int = 0;
    let mut sb: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    
    new_dirname = if should_dequote != 0 {
        bash_dequote_filename(
            dirname as *mut c_char,
            rl_completion_quote_character,
        )
    } else {
        savestring!(dirname)
    };
    
    dirlen = STRLEN!(new_dirname);
    if *new_dirname.offset((dirlen - 1 as c_int) as isize) as c_int
        == '/' as i32
    {
        *new_dirname
            .offset(
                (dirlen - 1 as c_int) as isize,
            ) = '\u{0}' as i32 as c_char;
    }
    r = (lstat(new_dirname, &mut sb) == 0 ) as c_int;
    free(new_dirname as *mut c_void);
    return r;
}

unsafe extern "C" fn bash_filename_stat_hook(
    mut dirname: *mut *mut c_char,
) -> c_int {
    let mut local_dirname: *mut c_char = 0 as *mut c_char;
    let mut new_dirname: *mut c_char = 0 as *mut c_char;
    let mut t: *mut c_char = 0 as *mut c_char;
    let mut should_expand_dirname: c_int = 0;
    let mut return_value: c_int = 0;
    let mut global_nounset: c_int = 0;
    let mut wl: *mut WordList = 0 as *mut WordList;
    
    local_dirname = *dirname;
    return_value = 0 ;
    should_expand_dirname = return_value;
    t = mbschr(local_dirname, '$' as i32);
    if !t.is_null() {
        should_expand_dirname = '$' as i32;
    } else {
        t = mbschr(local_dirname, '`' as i32);
        if !t.is_null() {
            should_expand_dirname = '`' as i32;
        }
    }
    if should_expand_dirname != 0
        && directory_exists(local_dirname, 0 as c_int) != 0
    {
        should_expand_dirname = 0 as c_int;
    }
    
    if should_expand_dirname != 0 {
        new_dirname = savestring!(local_dirname);
        
        global_nounset = unbound_vars_is_error;
        unbound_vars_is_error = 0 as c_int;
        wl = expand_prompt_string(
            new_dirname,
            0 as c_int,
            W_NOCOMSUB as c_int
                | W_NOPROCSUB as c_int
                | W_COMPLETE as c_int,
        );
        unbound_vars_is_error = global_nounset;
        if !wl.is_null() {
            free(new_dirname as *mut c_void);
            new_dirname = string_list(wl);
            
            if !new_dirname.is_null() && *new_dirname as c_int != 0 {
                free(local_dirname as *mut c_void);
                *dirname = new_dirname;
                local_dirname = *dirname;
                return_value = (STREQ!(local_dirname , *dirname) as c_int == 0 as c_int) as c_int;
            } else {
                free(
                    new_dirname as *mut c_void
                );
            }
            dispose_words(wl);
        } else {
            free(new_dirname as *mut c_void);
        }
    }
    
    if no_symbolic_links == 0 as c_int
        && (*local_dirname.offset(0 as isize) as c_int != '.' as i32
            || *local_dirname.offset(1 as isize) as c_int != 0)
    {
        let mut temp1: *mut c_char = 0 as *mut c_char;
        let mut temp2: *mut c_char = 0 as *mut c_char;
        t = get_working_directory(
            b"symlink-hook\0" as *const u8 as *const c_char as *mut c_char,
        );
        temp1 = make_absolute(local_dirname, t);
        free(t as *mut c_void);
        temp2 = sh_canonpath(temp1, 0x1 as c_int | 0x2 as c_int);
        
        if temp2.is_null() {
            free(temp1 as *mut c_void);
            return return_value;
        }
        free(local_dirname as *mut c_void);
        *dirname = temp2;
        free(temp1 as *mut c_void);
    }
    return return_value;
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

unsafe extern "C" fn bash_directory_completion_hook(
    mut dirname: *mut *mut c_char,
) -> c_int {
    let mut local_dirname: *mut c_char = 0 as *mut c_char;
    let mut new_dirname: *mut c_char = 0 as *mut c_char;
    let mut t: *mut c_char = 0 as *mut c_char;
    let mut return_value: c_int = 0;
    let mut should_expand_dirname: c_int = 0;
    let mut nextch: c_int = 0;
    let mut closer: c_int = 0;
    let mut wl: *mut WordList = 0 as *mut WordList;
    
    closer = 0;
    nextch = closer;
    should_expand_dirname = nextch;
    return_value = should_expand_dirname;
    local_dirname = *dirname;
    
    t = mbschr(local_dirname, '$' as i32);
    if !t.is_null() {
        should_expand_dirname = '$' as i32;
        nextch = *t.offset(1 as isize) as c_int;
        
        if nextch == '(' as i32 {
            closer = ')' as i32;
        } else if nextch == '{' as i32 {
            closer = '}' as i32;
        } else {
            nextch = 0 as c_int;
        }
        
        if closer != 0 {
            let mut p: c_int = 0;
            let mut delims: [c_char; 2] = [0; 2];
            
            delims[0 as usize] = closer as c_char;
            delims[1 as usize] = 0 as c_char;
            p = skip_to_delim(
                t,
                1 ,
                delims.as_mut_ptr(),
                SD_NOJMP as c_int | SD_COMPLETE as c_int,
            );
            if *t.offset(p as isize) as c_int != closer {
                should_expand_dirname = 0 as c_int;
            }
        }
    } else if *local_dirname.offset(0 as isize) as c_int
            == '~' as i32
        {
        should_expand_dirname = '~' as i32;
    } else {
        t = mbschr(local_dirname, '`' as i32);
        if !t.is_null()
            && unclosed_pair(
                local_dirname,
                strlen(local_dirname) as c_int,
                b"`\0" as *const u8 as *const c_char as *mut c_char,
            ) == 0 as c_int
        {
            should_expand_dirname = '`' as i32;
        }
    }
    
    if should_expand_dirname != 0
        && directory_exists(local_dirname, 1 as c_int) != 0
    {
        should_expand_dirname = 0 as c_int;
    }
    if should_expand_dirname != 0 {
        new_dirname = savestring!(local_dirname);
        wl = expand_prompt_string(
            new_dirname,
            0 as c_int,
            W_NOCOMSUB as c_int
                | W_NOPROCSUB as c_int
                | W_COMPLETE as c_int,
        );
        
        if !wl.is_null() {
            *dirname = string_list(wl);
            return_value = (STREQ!(local_dirname, *dirname) as c_int
                == 0 as c_int) as c_int;
            free(
                local_dirname as *mut c_void
            );
            free(
                new_dirname as *mut c_void
            );
            dispose_words(wl);
            local_dirname = *dirname;
            
            if !rl_filename_quote_characters.is_null()
                && *rl_filename_quote_characters as c_int != 0
            {
                let mut i: c_int = 0;
                let mut j: c_int = 0;
                let mut c: c_int = 0;
                i = strlen(default_filename_quote_characters) as c_int;
                custom_filename_quote_characters = xrealloc(
                    custom_filename_quote_characters as *mut c_void,
                    (i + 1 as c_int) as usize
                ) as *mut c_char;
                
                j = 0 ;
                i = j;
                loop {
                    c = *default_filename_quote_characters.offset(i as isize)
                        as c_int;
                    if !(c != 0) {
                        break;
                    }
                    if !(c == should_expand_dirname || c == nextch || c == closer) {
                        let fresh29 = j;
                        j = j + 1;
                        *custom_filename_quote_characters
                            .offset(fresh29 as isize) = c as c_char;
                    }
                    i += 1;
                }
                *custom_filename_quote_characters
                    .offset(j as isize) = '\u{0}' as i32 as c_char;
                rl_filename_quote_characters = custom_filename_quote_characters;
                set_filename_bstab(rl_filename_quote_characters);
            }
        } else {
            free(
                new_dirname as *mut c_void
            );
            free(
                local_dirname as *mut c_void
            );
            *dirname = malloc(
                1 as c_int as usize
            ) as *mut c_char;
            **dirname = '\u{0}' as i32 as c_char;
            return 1 as c_int;
        }
    } else {
        new_dirname = bash_dequote_filename(
            local_dirname,
            rl_completion_quote_character,
        );
        return_value = ((STREQ!(local_dirname, new_dirname)) as c_int
            == 0 as c_int) as c_int;
        free(
            local_dirname as *mut c_void
        );
        *dirname = new_dirname;
        local_dirname = *dirname;
    }
    
    if no_symbolic_links == 0 
        && (*local_dirname.offset(0 as isize) as c_int != '.' as i32
            || *local_dirname.offset(1 as isize) as c_int != 0)
    {
        let mut temp1: *mut c_char = 0 as *mut c_char;
        let mut temp2: *mut c_char = 0 as *mut c_char;
        let mut len1: c_int = 0;
        let mut len2: c_int = 0;
        t = get_working_directory(
            b"symlink-hook\0" as *const u8 as *const c_char as *mut c_char,
        );
        temp1 = make_absolute(local_dirname, t);
        free(t as *mut c_void);
        temp2 = sh_canonpath(temp1, PATH_CHECKDOTDOT!() as c_int | PATH_CHECKEXISTS!() as c_int);
        
        if temp2.is_null() && dircomplete_spelling != 0 && dircomplete_expand != 0 {
            temp2 = dirspell(temp1);
            if !temp2.is_null() {
                free(temp1 as *mut c_void);
                temp1 = temp2;
                temp2 = sh_canonpath(temp1, PATH_CHECKDOTDOT!() as c_int | PATH_CHECKEXISTS!() as c_int);
                return_value |= (temp2 != 0 as *mut c_char) as c_int;
            }
        }
        if temp2.is_null() {
            free(temp1 as *mut c_void);
            return return_value;
        }
        len1 = strlen(temp1) as c_int;
        if *temp1.offset((len1 - 1 ) as isize) as c_int == '/' as i32
        {
            len2 = strlen(temp2) as c_int;
            if len2 > 2 as c_int {
                temp2 = xrealloc(
                    temp2 as *mut c_void,
                    (len2 + 2 as c_int) as usize,
                ) as *mut c_char;
                *temp2.offset(len2 as isize) = '/' as i32 as c_char;
                *temp2
                    .offset(
                        (len2 + 1 ) as isize,
                    ) = '\u{0}' as i32 as c_char;
            }
        }
        if dircomplete_expand_relpath != 0
            || *local_dirname.offset(0 as isize) as c_int
                != '/' as i32
                && *local_dirname.offset(0 as isize) as c_int
                    != '.' as i32
                && STREQ!(temp1, temp2) as c_int
                    == 0 as c_int
        {
            return_value
                |= (STREQ!(local_dirname, temp2) as c_int
                    == 0 as c_int) as c_int;
        }
        free(
            local_dirname as *mut c_void
        );
        *dirname = temp2;
        free(
            temp1 as *mut c_void
        );
    }
    return return_value;
}


static mut history_completion_array: *mut *mut c_char = 0 as *const c_void
    as *mut c_void as *mut *mut c_char;
static mut harry_size: c_int = 0;
static mut harry_len: c_int = 0;


unsafe extern "C" fn build_history_completion_array() {
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut hlist: *mut *mut HIST_ENTRY = 0 as *mut *mut HIST_ENTRY;
    let mut tokens: *mut *mut c_char = 0 as *mut *mut c_char;
    if harry_size != 0 {
        strvec_dispose(history_completion_array);
        history_completion_array = 0 as *mut c_void as *mut *mut c_char;
        harry_size = 0 as c_int;
        harry_len = 0 as c_int;
    }
    hlist = history_list();
    if !hlist.is_null() {
        i = 0 as c_int;
        while !(*hlist.offset(i as isize)).is_null() {
            i += 1;
        }
        i -= 1;
        while i >= 0 as c_int {
            tokens = history_tokenize((**hlist.offset(i as isize)).line);
            j = 0 as c_int;
            while !tokens.is_null() && !(*tokens.offset(j as isize)).is_null() {
                if harry_len + 2 as c_int > harry_size {
                    harry_size += 10 as c_int;
                    history_completion_array = strvec_resize(
                        history_completion_array,
                        harry_size,
                    );
                }
                
                *history_completion_array.offset(harry_len as isize) = *tokens.offset(j as isize); 
                harry_len = harry_len + 1;
                *history_completion_array.offset(harry_len as isize) =  0 as *mut c_char;
                
                j += 1;
            }
            free(tokens as *mut c_void);
            i -= 1;
        }
        
        if dabbrev_expand_active == 0 {
            qsort(
                history_completion_array as *mut c_void,
                harry_len as usize,
                ::std::mem::size_of::<*mut c_char>() as usize,
                ::std::mem::transmute::<
                    unsafe extern "C" fn(*mut *mut c_char, *mut *mut c_char) -> i32,
                    Option<unsafe extern "C" fn(*const c_void, *const c_void) -> i32>
                >(strvec_strcmp)
            );
        }
    }
}

unsafe extern "C" fn history_completion_generator(
    mut hint_text: *const c_char,
    mut state: c_int,
) -> *mut c_char {
    static mut local_index: c_int = 0;
    static mut len: c_int = 0;
    static mut text: *const c_char = 0 as *const c_char;
    
    if state == 0 {
        if dabbrev_expand_active != 0 {
            rl_completion_suppress_append = 1 ;
        }
        local_index = 0 ;
        build_history_completion_array();
        text = hint_text;
        len = strlen(text) as c_int;
    }
    
    while !history_completion_array.is_null()
        && !(*history_completion_array.offset(local_index as isize)).is_null()
    {
        let local_index_temp = local_index;   //local_index_temp 临时变量
        local_index = local_index + 1;
        if strncmp(
            text,
            *history_completion_array.offset(local_index_temp as isize),
            len as usize,
        ) == 0 as c_int
        {
            return savestring!(*history_completion_array.offset((local_index - 1 ) as isize));
        }
    }
    return 0 as *mut c_char;
}

unsafe extern "C" fn dynamic_complete_history(
    mut count: c_int,
    mut key: c_int,
) -> c_int {
    let mut r: c_int = 0;
    let mut orig_func: rl_compentry_func_t = None;
    let mut orig_attempt_func: rl_completion_func_t = None;
    let mut orig_ignore_func: rl_compignore_func_t = None;
    orig_func = rl_completion_entry_function;
    orig_attempt_func = rl_attempted_completion_function;
    orig_ignore_func = rl_ignore_some_completions_function;
    
    rl_completion_entry_function = Some(history_completion_generator);
    rl_attempted_completion_function = None;
    rl_ignore_some_completions_function = Some(filename_completion_ignore);

    
    if rl_last_func
        == ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> c_int>,
            Option::<rl_command_func_t>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn(c_int, c_int) -> c_int,
                    unsafe extern "C" fn() -> c_int,
                >(dynamic_complete_history),
            ),
        )
    {
        r = rl_complete_internal('?' as i32);
    } else {
        r = rl_complete_internal('\t' as i32);
    }
    
    rl_completion_entry_function = orig_func;
    rl_attempted_completion_function = orig_attempt_func;
    rl_ignore_some_completions_function = orig_ignore_func;
    return r;
}

unsafe extern "C" fn bash_dabbrev_expand(
    mut count: c_int,
    mut key: c_int,
) -> c_int {
    let mut r: c_int = 0;
    let mut orig_suppress: c_int = 0;
    let mut orig_sort: c_int = 0;
    let mut orig_func: rl_compentry_func_t = None;
    let mut orig_attempt_func: rl_completion_func_t = None;
    let mut orig_ignore_func: rl_compignore_func_t = None;
    
    orig_func = rl_menu_completion_entry_function;
    orig_attempt_func = rl_attempted_completion_function;
    orig_ignore_func = rl_ignore_some_completions_function;
    orig_suppress = rl_completion_suppress_append;
    orig_sort = rl_sort_completion_matches;
    
    rl_menu_completion_entry_function = Some(history_completion_generator);
    rl_attempted_completion_function = None;
    rl_ignore_some_completions_function = Some(filename_completion_ignore);
    rl_filename_completion_desired = 0 as c_int;
    rl_completion_suppress_append = 1 as c_int;
    rl_sort_completion_matches = 0 as c_int;
    
    dabbrev_expand_active = 1 as c_int;
    if rl_last_func
        == ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> c_int>,
            Option::<rl_command_func_t>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn(c_int, c_int) -> c_int,
                    unsafe extern "C" fn() -> c_int,
                >(bash_dabbrev_expand),
            ),
        )
    {
        rl_last_func = Some(
            rl_menu_complete
                as unsafe extern "C" fn(c_int, c_int) -> c_int,
        );
    }
    
    r = rl_menu_complete(count, key);
    dabbrev_expand_active = 0 as c_int;
    rl_last_func = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> c_int>,
        Option::<rl_command_func_t>,
    >(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn(c_int, c_int) -> c_int,
                unsafe extern "C" fn() -> c_int,
            >(bash_dabbrev_expand),
        ),
    );
    rl_menu_completion_entry_function = orig_func;
    rl_attempted_completion_function = orig_attempt_func;
    rl_ignore_some_completions_function = orig_ignore_func;
    rl_completion_suppress_append = orig_suppress;
    rl_sort_completion_matches = orig_sort;
    return r;
}

unsafe extern "C" fn bash_complete_username(
    mut ignore: c_int,
    mut ignore2: c_int,
) -> c_int {
    return bash_complete_username_internal(
        rl_completion_mode(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> c_int>,
                Option::<rl_command_func_t>,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(c_int, c_int) -> c_int,
                        unsafe extern "C" fn() -> c_int,
                    >(bash_complete_username),
                ),
            ),
        ),
    );
}

unsafe extern "C" fn bash_possible_username_completions(
    mut ignore: c_int,
    mut ignore2: c_int,
) -> c_int {
    return bash_complete_username_internal('?' as i32);
}

unsafe extern "C" fn bash_complete_username_internal(
    mut what_to_do: c_int,
) -> c_int {
    return bash_specific_completion(
        what_to_do,
        Some(rl_username_completion_function)
        );  
}

unsafe extern "C" fn bash_complete_filename(
    mut ignore: c_int,
    mut ignore2: c_int,
) -> c_int {
    return bash_complete_filename_internal(
        rl_completion_mode(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> c_int>,
                Option::<rl_command_func_t>,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(c_int, c_int) -> c_int,
                        unsafe extern "C" fn() -> c_int,
                    >(bash_complete_filename),
                ),
            ),
        ),
    );
}

unsafe extern "C" fn bash_possible_filename_completions(
    mut ignore: c_int,
    mut ignore2: c_int,
) -> c_int {
    return bash_complete_filename_internal('?' as i32);
}

unsafe extern "C" fn bash_complete_filename_internal(
    mut what_to_do: c_int,
) -> c_int {
    let mut orig_func: rl_compentry_func_t = None;
    let mut orig_attempt_func: rl_completion_func_t = None;
    let mut orig_dir_func: rl_icppfunc_t = None;
    let mut orig_ignore_func: rl_compignore_func_t = None;
    let mut orig_rl_completer_word_break_characters: *mut c_char = 0
        as *mut c_char;
    let mut r: c_int = 0;
    
    orig_func = rl_completion_entry_function;
    orig_attempt_func = rl_attempted_completion_function;
    orig_ignore_func = rl_ignore_some_completions_function;
    orig_rl_completer_word_break_characters = rl_completer_word_break_characters;
    orig_dir_func = save_directory_hook();
    rl_completion_entry_function = Some(
        rl_filename_completion_function
            as unsafe extern "C" fn(
                *const c_char,
                c_int,
            ) -> *mut c_char,
    );

    rl_completion_entry_function = Some(rl_filename_completion_function);
    rl_attempted_completion_function = None;
    rl_ignore_some_completions_function = Some(filename_completion_ignore);
    rl_completer_word_break_characters = b" \t\n\"'\0" as *const u8
        as *const c_char as *mut c_char;
    
    r = rl_complete_internal(what_to_do);
    rl_completion_entry_function = orig_func;
    rl_attempted_completion_function = orig_attempt_func;
    rl_ignore_some_completions_function = orig_ignore_func;
    rl_completer_word_break_characters = orig_rl_completer_word_break_characters;
    restore_directory_hook(orig_dir_func);
    return r;
}

unsafe extern "C" fn bash_complete_hostname(
    mut ignore: c_int,
    mut ignore2: c_int,
) -> c_int {
    return bash_complete_hostname_internal(
        rl_completion_mode(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> c_int>,
                Option::<rl_command_func_t>,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(c_int, c_int) -> c_int,
                        unsafe extern "C" fn() -> c_int,
                    >(bash_complete_hostname),
                ),
            ),
        ),
    );
}

unsafe extern "C" fn bash_possible_hostname_completions(
    mut ignore: c_int,
    mut ignore2: c_int,
) -> c_int {
    return bash_complete_hostname_internal('?' as i32);
}

unsafe extern "C" fn bash_complete_variable(
    mut ignore: c_int,
    mut ignore2: c_int,
) -> c_int {
    return bash_complete_variable_internal(
        rl_completion_mode(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> c_int>,
                Option::<rl_command_func_t>,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(c_int, c_int) -> c_int,
                        unsafe extern "C" fn() -> c_int,
                    >(bash_complete_variable),
                ),
            ),
        ),
    );
}

unsafe extern "C" fn bash_possible_variable_completions(
    mut ignore: c_int,
    mut ignore2: c_int,
) -> c_int {
    return bash_complete_variable_internal('?' as i32);
}

unsafe extern "C" fn bash_complete_command(
    mut ignore: c_int,
    mut ignore2: c_int,
) -> c_int {
    return bash_complete_command_internal(
        rl_completion_mode(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> c_int>,
                Option::<rl_command_func_t>,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(c_int, c_int) -> c_int,
                        unsafe extern "C" fn() -> c_int,
                    >(bash_complete_command),
                ),
            ),
        ),
    );
}

unsafe extern "C" fn bash_possible_command_completions(
    mut ignore: c_int,
    mut ignore2: c_int,
) -> c_int {
    return bash_complete_command_internal('?' as i32);
}

unsafe extern "C" fn bash_complete_hostname_internal(
    mut what_to_do: c_int,
) -> c_int {
    return bash_specific_completion(
        what_to_do,
        Some(hostname_completion_function)
    );
}

unsafe extern "C" fn bash_complete_variable_internal(
    mut what_to_do: c_int,
) -> c_int {
    return bash_specific_completion(
        what_to_do,
        Some(variable_completion_function)
    );
}

unsafe extern "C" fn bash_complete_command_internal(
    mut what_to_do: c_int,
) -> c_int {
    return bash_specific_completion(
        what_to_do,
        Some(command_word_completion_function)
    );
}

unsafe extern "C" fn completion_glob_pattern(
    mut string: *mut c_char,
) -> c_int {
    return (glob_pattern_p(string) == 1 as c_int) as c_int;
}

static mut globtext: *mut c_char = 0 as *const c_char as *mut c_char;
static mut globorig: *mut c_char = 0 as *const c_char as *mut c_char;

#[macro_export]
macro_rules! GLOB_FAILED {
    ($glist:expr) => {
        $glist == &mut glob_error_return as *mut *mut c_char
    };
}

unsafe extern "C" fn glob_complete_word(
    mut text: *const c_char,
    mut state: c_int,
) -> *mut c_char {
    static mut matches: *mut *mut c_char = 0 as *const c_void
        as *mut c_void as *mut *mut c_char;
    static mut ind: c_int = 0;
    let mut glen: c_int = 0;
    let mut ret: *mut c_char = 0 as *mut c_char;
    let mut ttext: *mut c_char = 0 as *mut c_char;
    
    if state == 0 {
        rl_filename_completion_desired = 1 ;
        if !matches.is_null() {
            FREE!(matches);
        }
        if globorig != globtext {
            FREE!(globorig);
        }
        FREE!(globtext);

        ttext = bash_tilde_expand(text, 0 );
        if rl_explicit_arg != 0 {
            globorig = savestring!(ttext);
            glen = strlen(ttext) as c_int;
            globtext = malloc(
                (glen + 2 as c_int) as usize
            ) as *mut c_char;
            strcpy(globtext, ttext);
            *globtext.offset(glen as isize) = '*' as i32 as c_char;
            *globtext
                .offset(
                    (glen + 1 as c_int) as isize,
                ) = '\u{0}' as i32 as c_char;
        } else {
            globorig = savestring!(ttext);
            globtext = globorig;
        }
        if ttext != text as *mut c_char {
            free(ttext as *mut c_void);
        }
        matches = shell_glob_filename(globtext, 0 );
        if GLOB_FAILED!(matches) {
            matches = 0 as *mut *mut c_char;
        }
        ind = 0 ;
    }

    ret = if !matches.is_null() {
        *matches.offset(ind as isize)
    } else {
        0 as *mut c_char
    };
    ind += 1;
    return ret;
}


unsafe extern "C" fn bash_glob_completion_internal(
    mut what_to_do: c_int,
) -> c_int {
    return bash_specific_completion(
        what_to_do,
        Some(glob_complete_word))
}

unsafe extern "C" fn bash_glob_quote_filename(
    mut s: *mut c_char,
    mut rtype: c_int,
    mut qcp: *mut c_char,
) -> *mut c_char {
    if !globorig.is_null() && !qcp.is_null() && *qcp as c_int == '\u{0}' as i32
        && STREQ!(s, globorig)
    {
        return savestring!(s)
    } else {
        return bash_quote_filename(s, rtype, qcp)
    };
}

unsafe extern "C" fn bash_glob_complete_word(
    mut count: c_int,
    mut key: c_int,
) -> c_int {
    let mut r: c_int = 0;
    let mut orig_quoting_function: rl_quote_func_t = None;
    
    if rl_editing_mode == EMACS_EDITING_MODE!(){
        rl_explicit_arg = 1 ;
    }
    orig_quoting_function = rl_filename_quoting_function;
    rl_filename_quoting_function = Some(bash_glob_quote_filename);
    
    r = bash_glob_completion_internal(
        rl_completion_mode(
            Some(bash_glob_complete_word)
        ),
    );

    rl_filename_quoting_function = orig_quoting_function;
    return r;
}

unsafe extern "C" fn bash_glob_expand_word(
    mut count: c_int,
    mut key: c_int,
) -> c_int {
    return bash_glob_completion_internal('*' as i32);
}

unsafe extern "C" fn bash_glob_list_expansions(
    mut count: c_int,
    mut key: c_int,
) -> c_int {
    return bash_glob_completion_internal('?' as i32);
}

unsafe extern "C" fn bash_specific_completion(
    mut what_to_do: c_int,
    mut generator: rl_compentry_func_t,
) -> c_int {
    let mut orig_func: rl_compentry_func_t = None;
    let mut orig_attempt_func: rl_completion_func_t = None;
    let mut orig_ignore_func: rl_compignore_func_t = None;
    let mut r: c_int = 0;
    orig_func = rl_completion_entry_function;
    orig_attempt_func = rl_attempted_completion_function;
    orig_ignore_func = rl_ignore_some_completions_function;
    rl_completion_entry_function = generator;
    rl_attempted_completion_function = None;
    rl_ignore_some_completions_function = orig_ignore_func;
    r = rl_complete_internal(what_to_do);
    rl_completion_entry_function = orig_func;
    rl_attempted_completion_function = orig_attempt_func;
    rl_ignore_some_completions_function = orig_ignore_func;
    return r;
}

unsafe extern "C" fn bash_vi_complete(
    mut count: c_int,
    mut key: c_int,
) -> c_int {
    let mut p: c_int = 0;
    let mut r: c_int = 0;
    let mut t: *mut c_char = 0 as *mut c_char;
    
    if rl_point < rl_end
        && !whitespace!(*rl_line_buffer.offset(rl_point as isize))
    {
        if !whitespace!(*rl_line_buffer.offset((rl_point + 1 ) as isize))
        {
            rl_vi_end_word(1 , 'E' as i32);
        }
        rl_point += 1;
    }
    
    t = 0 as *mut c_char;
    if rl_point > 0 {
        p = rl_point;
        rl_vi_bWord(1 , 'B' as i32);
        r = rl_point;
        rl_point = p;
        p = r;
        t = substring(rl_line_buffer, p, rl_point);
    }
    
    if !t.is_null() && completion_glob_pattern(t) == 0 {
        rl_explicit_arg = 1 ;
    }
    FREE!(t);

    if key == '*' as i32 {
        r = bash_glob_expand_word(count, key);
    } else if key == '=' as i32 {
        r = bash_glob_list_expansions(count, key);
    } else if key == '\\' as i32 {
        r = bash_glob_complete_word(count, key);
    } else {
        r = rl_complete(0 , key);
    }
    if key == '*' as i32 || key == '\\' as i32 {
        rl_vi_start_inserting(key, 1 , 1 );
    }
    return r;
}

#[macro_export]
macro_rules! CBSDQUOTE {
    () => {
        0x0040
    };
}

unsafe extern "C" fn bash_dequote_filename(
    mut text: *mut c_char,
    mut quote_char: c_int,
) -> *mut c_char {
    let mut ret: *mut c_char = 0 as *mut c_char;
    let mut p: *mut c_char = 0 as *mut c_char;
    let mut r: *mut c_char = 0 as *mut c_char;
    let mut l: c_int = 0;
    let mut quoted: c_int = 0;
    
    l = strlen(text) as c_int;
    ret = malloc(
        (l + 1 as c_int) as usize
    ) as *mut c_char;
    
    quoted = quote_char;
    p = text;
    r = ret;
    while !p.is_null() && *p as c_int != 0 {
        if *p as c_int == '\\' as i32 {
            if quoted == '\'' as i32 {
                *r = *p;
                r = r.offset(1);
            } else if quoted == '"' as i32
                    && *sh_syntaxtab
                        .as_mut_ptr()
                        .offset(
                            *p.offset(1 as c_int as isize) as libc::c_uchar
                                as isize,
                        ) & CBSDQUOTE!() == 0 
            {
                *r = *p;
                r = r.offset(1);
            }
            
            p = p.offset(1);
            *r = *p;
            r = r.offset(1);
           
            if *p == '\u{0}' as c_char {
                return ret;
            }
        } else if quoted != 0 && *p as c_int == quoted {
            quoted = 0 ;
        } else if quoted == 0 
                && (*p as c_int == '\'' as i32 || *p as c_int == '"' as i32)
            {
            quoted = *p as c_int;
        } else {
            *r = *p;
            r = r.offset(1);
        }

        p = p.offset(1);
    }
    *r = '\u{0}' as i32 as c_char;
    return ret;
}

unsafe extern "C" fn quote_word_break_chars(
    mut text: *mut c_char,
) -> *mut c_char {
    let mut ret: *mut c_char = 0 as *mut c_char;
    let mut r: *mut c_char = 0 as *mut c_char;
    let mut s: *mut c_char = 0 as *mut c_char;
    let mut l: c_int = 0;
    
    l = strlen(text) as c_int;
    ret = malloc(
        ((2 as c_int * l + 1 as c_int) as size_t).try_into().unwrap()
    ) as *mut c_char;
    
    s = text;
    r = ret;
    while *s != 0 {
        if *s as c_int == '\\' as i32 {
            *r = '\\' as i32 as c_char;
            r = r.offset(1);
            
            s = s.offset(1);
            *r = *s;
            r = r.offset(1);
            
            if *s as c_int == '\u{0}' as i32 {
                break;
            }
        } else {
            if !(mbschr(rl_completer_word_break_characters, *s as c_int)).is_null()
            {
                *r = '\\' as i32 as c_char;
                r = r.offset(1);
                
            }
            if s == text && *s as c_int == '~' as i32 && file_exists(text) != 0 {
                *r = '\\' as i32 as c_char;
                r = r.offset(1);  
            }
            *r = *s;
            r = r.offset(1);  
        }
        s = s.offset(1);
    }
    *r = '\u{0}' as i32 as c_char;
    return ret;
}

unsafe extern "C" fn set_filename_bstab(mut string: *const c_char) {
    let mut s: *const c_char = 0 as *const c_char;
    memset(
        filename_bstab.as_mut_ptr() as *mut c_void,
        0 as c_int,
        ::std::mem::size_of::<[c_char; 256]>() as usize,
    );
    s = string;
    while !s.is_null() && *s as c_int != 0 {
        filename_bstab[*s as libc::c_uchar as usize] = 1 as c_int as c_char;
        s = s.offset(1);
    }
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

unsafe extern "C" fn bash_quote_filename(
    mut s: *mut c_char,
    mut rtype: c_int,
    mut qcp: *mut c_char,
) -> *mut c_char {
    let mut rtext: *mut c_char = 0 as *mut c_char;
    let mut mtext: *mut c_char = 0 as *mut c_char;
    let mut ret: *mut c_char = 0 as *mut c_char;
    let mut rlen: c_int = 0;
    let mut cs: c_int = 0;
    
    rtext = 0 as *mut c_char;
   
    cs = completion_quoting_style;
    
    if *qcp as c_int == '\u{0}' as i32 && cs == COMPLETE_BSQUOTE!()
        && !(mbschr(s, '\n' as i32)).is_null()
    {
        cs = COMPLETE_SQUOTE!();
    } else if *qcp as c_int == '"' as i32 {
        cs = COMPLETE_DQUOTE!();
    } else if *qcp as c_int == '\'' as i32 {
        cs = COMPLETE_SQUOTE!();
    } else if *qcp as c_int == '\u{0}' as i32 && history_expansion != 0
            && cs == COMPLETE_DQUOTE!() && history_expansion_inhibited == 0  
            && !(mbschr(s, '!' as i32)).is_null()
        {
        cs = COMPLETE_BSQUOTE!();
    }
    if *qcp as c_int == '"' as i32 && history_expansion != 0
        && cs == COMPLETE_DQUOTE!() && history_expansion_inhibited == 0  
        && !(mbschr(s, '!' as i32)).is_null()
    {
        cs = COMPLETE_BSQUOTE!();
        *qcp = '\u{0}' as i32 as c_char;
    }
    
    mtext = s;
    if *mtext.offset(0 as c_int as isize) as c_int == '~' as i32
        && rtype == SINGLE_MATCH as c_int && cs != COMPLETE_BSQUOTE!()
    {
        mtext = bash_tilde_expand(s, 0 );
    }
    match cs {
        COMPLETE_DQUOTE!() => {
            rtext = sh_double_quote(mtext);
        }
        COMPLETE_SQUOTE!() => {
            rtext = sh_single_quote(mtext);
        }
        COMPLETE_BSQUOTE!() => {
            rtext = sh_backslash_quote(
                mtext,
                if complete_fullquote != 0 {
                    0 as *mut c_char
                } else {
                    filename_bstab.as_mut_ptr()
                },
                0 as c_int,
            );
        }
        _ => {}
    }
    
    if mtext != s {
        free(
            mtext as *mut c_void
        );
    }
    
    if !rtext.is_null() && cs == COMPLETE_BSQUOTE!(){
        mtext = quote_word_break_chars(rtext);
        free(
            rtext as *mut c_void
        );
        rtext = mtext;
    }
    
    if !rtext.is_null() {
        rlen = strlen(rtext) as c_int;
        ret = malloc(
            (rlen + 1 as c_int) as usize
        ) as *mut c_char;
        strcpy(ret, rtext);
    } else {
        rlen = 1 ;
        ret = malloc(
            rlen as usize
        ) as *mut c_char;
        *ret.offset(0 as isize) = '\u{0}' as i32 as c_char;
    }
    
    if rtype == MULT_MATCH as c_int && cs != COMPLETE_BSQUOTE!() {
        *ret.offset((rlen - 1 as c_int) as isize) = '\u{0}' as i32 as c_char;
    }
    free(
        rtext as *mut c_void
    );
    return ret;
}

static mut emacs_std_cmd_xmap: Keymap = 0 as *const KEYMAP_ENTRY as *mut KEYMAP_ENTRY;
static mut vi_insert_cmd_xmap: Keymap = 0 as *const KEYMAP_ENTRY as *mut KEYMAP_ENTRY;
static mut vi_movement_cmd_xmap: Keymap = 0 as *const KEYMAP_ENTRY as *mut KEYMAP_ENTRY;

unsafe extern "C" fn putx(c:c_int) -> c_int
{
    let mut  x:c_int = 0;
    x = putc(c, rl_outstream);
    return x;
}

#[macro_export]
macro_rules! MB_CUR_MAX {
    ($s:expr) => {
        if !$s.is_null()
                && *$s.offset(0 as isize) as c_int != 0
            {
                if *$s.offset(1 as isize) as c_int != 0
                {
                    mbstrlen($s)
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
        if !$s.is_null()
                && *$s.offset(0 as isize) as libc::c_int != 0
            {
                if *$s.offset(1 as isize) as libc::c_int != 0
                {
                    mbstrlen($s).try_into().unwrap()
                } else {
                    (1 as libc::c_ulong).try_into().unwrap()
                }
            } else {
                (0 as libc::c_ulong).try_into().unwrap()
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


unsafe extern "C" fn readline_get_char_offset(mut ind: c_int) -> c_int {
    let mut r: c_int = 0;
    let mut old_ch: c_int = 0;
    
    r = ind;
    if locale_mb_cur_max > 1 {
        old_ch = *rl_line_buffer.offset(ind as isize) as c_int;
        *rl_line_buffer.offset(ind as isize) = '\u{0}' as i32 as c_char;
        r = MB_STRLEN!(rl_line_buffer); 
        *rl_line_buffer.offset(ind as isize) = old_ch as c_char;
    }
    return r;
}

unsafe extern "C" fn readline_set_char_offset(
    mut ind: c_int,
    mut varp: *mut c_int,
) {
    let mut i: c_int = 0;
    i = ind;
    if i > 0 as c_int && locale_mb_cur_max > 1 as c_int {
        i = _rl_find_next_mbchar(rl_line_buffer, 0 as c_int, i, 0 as c_int);
    }
    if i != *varp {
        if i > rl_end {
            i = rl_end;
        } else if i < 0 as c_int {
            i = 0 as c_int;
        }
        *varp = i;
    }
}

#[macro_export]
macro_rules! VSETATTR {
    ($var:expr,$attr:expr) => {
        (*$var).attributes = (*$var).attributes | (&$attr);
        (*$var).attributes
    };
}

#[no_mangle]
pub unsafe extern "C" fn bash_execute_unix_command(
    mut count: c_int,
    mut key: c_int,
) -> c_int {
    let mut type_0: c_int = 0;
    let mut i: c_int = 0;
    let mut r: c_int = 0;
    let mut mi: intmax_t = 0;
    let mut ps: sh_parser_state_t = sh_parser_state_t {
        parser_state: 0,
        token_state: 0 as *mut c_int,
        token: 0 as *mut c_char,
        token_buffer_size: 0,
        input_line_terminator: 0,
        eof_encountered: 0,
        prompt_string_pointer: 0 as *mut *mut c_char,
        current_command_line_count: 0,
        remember_on_history: 0,
        history_expansion_inhibited: 0,
        last_command_exit_value: 0,
        pipestatus: 0 as *mut r_bash::ARRAY,
        last_shell_builtin: None,
        this_shell_builtin: None,
        expand_aliases: 0,
        echo_input_at_read: 0,
        need_here_doc: 0,
        here_doc_first_line: 0,
        redir_stack: [0 as *mut r_bash::REDIRECT; 16],
    };
    let mut cmd: *mut c_char = 0 as *mut c_char;
    let mut value: *mut c_char = 0 as *mut c_char;
    let mut ce: *mut c_char = 0 as *mut c_char;
    let mut old_ch: c_char = 0;
    let mut v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut ibuf: [c_char; 12] = [0; 12];
    let mut cmd_xmap: Keymap = 0 as *mut KEYMAP_ENTRY;
   
    cmd_xmap = get_cmd_xmap_from_keymap(rl_get_keymap());
    cmd = ::std::mem::transmute::<
        Option::<rl_command_func_t>,
        *mut c_char,
    >(
        rl_function_of_keyseq_len(
            rl_executing_keyseq,
            rl_key_sequence_length as usize,
            cmd_xmap,
            &mut type_0,
        ),
    );
   
    if type_0 == ISKMAP as c_int
        && {
            type_0 = (*(cmd as Keymap)
                .offset(ANYOTHERKEY as isize))
                .type_ as c_int;
            type_0 == ISMACR as c_int
        }
    {
        cmd = ::std::mem::transmute::<
            Option::<rl_command_func_t>,
            *mut c_char,
        >(
            (*(cmd as Keymap).offset((ANYOTHERKEY) as isize))
                .function,
        );
    }
    
    if cmd.is_null() || type_0 != ISMACR as c_int {
        rl_crlf();
        internal_error(
                b"bash_execute_unix_command: cannot find keymap for command\0"
                    as *const u8 as *const c_char
        );
        rl_forced_update_display();
        return 1 as c_int;
    }
    
    ce = rl_get_termcap(b"ce\0" as *const u8 as *const c_char);
    if !ce.is_null() {
        rl_clear_visible_line();
        fflush(rl_outstream);
    } else {
        rl_crlf();
    }
    
    v = bind_variable(
        b"READLINE_LINE\0" as *const u8 as *const c_char,
        rl_line_buffer,
        0 as c_int,
    );
   
    if !v.is_null() {
        VSETATTR!(v, att_exported as i32);
    }
    
    i = readline_get_char_offset(rl_point);
    value = inttostr(
        i as intmax_t,
        ibuf.as_mut_ptr(),
        ::std::mem::size_of::<[c_char; 12]>() as usize,
    );
    v = bind_int_variable(
        b"READLINE_POINT\0" as *const u8 as *const c_char as *mut c_char,
        value,
        0 ,
    );
    
    if !v.is_null() {
        VSETATTR!(v, att_exported as i32);
    }
    
    i = readline_get_char_offset(rl_mark);
    value = inttostr(
        i as intmax_t,
        ibuf.as_mut_ptr(),
        ::std::mem::size_of::<[c_char; 12]>() as usize,
    );
    v = bind_int_variable(
        b"READLINE_MARK\0" as *const u8 as *const c_char as *mut c_char,
        value,
        0 ,
    );
    
    if !v.is_null() {
        VSETATTR!(v, att_exported  as i32);
    }
    array_needs_making = 1 ;
    
    save_parser_state(&mut ps);
    rl_clear_signals();
    r = parse_and_execute(
        savestring!(cmd),
        b"bash_execute_unix_command\0" as *const u8 as *const c_char,
        SEVAL_NOHIST as c_int,
    );
    rl_set_signals();
    restore_parser_state(&mut ps);
   
    v = find_variable(b"READLINE_LINE\0" as *const u8 as *const c_char);
    maybe_make_readline_line(
        if !v.is_null() { value_cell!(v) } else { 0 as *mut c_char },
    );
    
    v = find_variable(b"READLINE_POINT\0" as *const u8 as *const c_char);
    if !v.is_null() && legal_number(value_cell!(v), &mut mi) != 0 {
        readline_set_char_offset(mi as c_int, &mut rl_point);
    }
    v = find_variable(b"READLINE_MARK\0" as *const u8 as *const c_char);
    if !v.is_null() && legal_number(value_cell!(v), &mut mi) != 0 {
        readline_set_char_offset(mi as c_int, &mut rl_mark);
    }
    
    check_unbind_variable(b"READLINE_LINE\0" as *const u8 as *const c_char);
    check_unbind_variable(b"READLINE_POINT\0" as *const u8 as *const c_char);
    check_unbind_variable(b"READLINE_MARK\0" as *const u8 as *const c_char);
    array_needs_making = 1 ;
    
    if !ce.is_null() && r != 124 {
        rl_redraw_prompt_last_line();
    } else {
        rl_forced_update_display();
    }
    return 0 as c_int;
}

#[macro_export]
macro_rules! value_cell {
    ($var:expr) => {
        (*$var).value 
    };
}

#[no_mangle]
pub unsafe extern "C" fn print_unix_command_map() -> c_int {
    let mut save: Keymap = 0 as *mut KEYMAP_ENTRY;
    let mut cmd_xmap: Keymap = 0 as *mut KEYMAP_ENTRY;
   
    save = rl_get_keymap();
    cmd_xmap = get_cmd_xmap_from_keymap(save);
    rl_set_keymap(cmd_xmap);
    rl_macro_dumper(1 as c_int);
    rl_set_keymap(save);
    return 0 as c_int;
}

#[macro_export]
macro_rules! ESC {
    () => {
        CTRL!('['  as i32 )
    };
}

unsafe extern "C" fn init_unix_command_map() {
    emacs_std_cmd_xmap = rl_make_bare_keymap();
    
    (*emacs_std_cmd_xmap.offset(CTRL!('X' as i32 ) as isize))
        .type_ = ISKMAP as c_int as c_char;
    let ref mut fresh43 = (*emacs_std_cmd_xmap
        .offset(CTRL!('X' as i32 ) as isize))
        .function;
    (*emacs_std_cmd_xmap.offset(CTRL!('X' as i32 ) as isize))
        .function = ::std::mem::transmute::<
        Keymap,
        Option::<rl_command_func_t>,
    >(rl_make_bare_keymap());
    
    (*emacs_std_cmd_xmap.offset(ESC!() as isize))
        .type_ = ISKMAP as c_int as c_char;
    (*emacs_std_cmd_xmap.offset(ESC!() as isize))
        .function = ::std::mem::transmute::<
        Keymap,
        Option::<rl_command_func_t>,
    >(rl_make_bare_keymap());
    
    vi_insert_cmd_xmap = rl_make_bare_keymap();
    vi_movement_cmd_xmap = rl_make_bare_keymap();
}


unsafe extern "C" fn get_cmd_xmap_from_keymap(mut kmap: Keymap) -> Keymap {
    if emacs_std_cmd_xmap.is_null() {
        init_unix_command_map();
    }
    
    if kmap == emacs_standard_keymap.as_mut_ptr() {
        return emacs_std_cmd_xmap
    } else if kmap == emacs_meta_keymap.as_mut_ptr() {
        return ::std::mem::transmute::<
            Option::<rl_command_func_t>,
            Keymap,
        >(
            (*emacs_std_cmd_xmap.offset(ESC!() as isize))
                .function,
        )
    } else if kmap == emacs_ctlx_keymap.as_mut_ptr() {
        return ::std::mem::transmute::<
            Option::<rl_command_func_t>,
            Keymap,
        >(
            (*emacs_std_cmd_xmap.offset(CTRL!('X' as i32) as isize))
                .function,
        )
    } else if kmap == vi_insertion_keymap.as_mut_ptr() {
        return vi_insert_cmd_xmap
    } else if kmap == vi_movement_keymap.as_mut_ptr() {
        return vi_movement_cmd_xmap
    } else {
        return 0 as *mut c_void as Keymap
    };
}

unsafe extern "C" fn isolate_sequence(
    mut string: *mut c_char,
    mut ind: c_int,
    mut need_dquote: c_int,
    mut startp: *mut c_int,
) -> c_int {
    let mut i: c_int = 0;
    let mut c: c_int = 0;
    let mut passc: c_int = 0;
    let mut delim: c_int = 0;
    
    i = ind;
    while *string.offset(i as isize) as c_int != 0
        && whitespace!(*string.offset(i as isize))
    {
        i += 1;
    }
    
    if need_dquote != 0 && *string.offset(i as isize) as c_int != '"' as i32 {
        builtin_error(
            b"%s: first non-whitespace character is not `\"'\0" as *const u8
                    as *const c_char,
            string,
        );
        return -(1 as c_int);
    }
    
    delim = if *string.offset(i as isize) as c_int == '"' as i32
        || *string.offset(i as isize) as c_int == '\'' as i32
    {
        *string.offset(i as isize) as c_int
    } else {
        0 as c_int
    };
    
    if !startp.is_null() {
        *startp = if delim != 0 {
            i += 1;
            i
        } else {
            i
        };
    }
    
    passc = 0 as c_int;
    loop {
        c = *string.offset(i as isize) as c_int;
        if !(c != 0) {
            break;
        }
        if passc != 0 {
            passc = 0 as c_int;
        } else if c == '\\' as i32 {
            passc += 1;
        } else if c == delim {
            break;
        }
        i += 1;
    }
    if delim != 0 && *string.offset(i as isize) as c_int != delim {
        builtin_error(
             b"no closing `%c' in %s\0" as *const u8 as *const c_char,
            delim,
            string,
        );
        return -(1 as c_int);
    }
    return i;
}

#[no_mangle]
pub unsafe extern "C" fn bind_keyseq_to_unix_command(
    mut line: *mut c_char,
) -> c_int {
    let mut kmap: Keymap = 0 as *mut KEYMAP_ENTRY;
    let mut cmd_xmap: Keymap = 0 as *mut KEYMAP_ENTRY;
    let mut kseq: *mut c_char = 0 as *mut c_char;
    let mut value: *mut c_char = 0 as *mut c_char;
    let mut i: c_int = 0;
    let mut kstart: c_int = 0;
    
    kmap = rl_get_keymap();
    
    i = isolate_sequence(line, 0 as c_int, 1 as c_int, &mut kstart);
    if i < 0 {
        return -(1 as c_int);
    }
    
    kseq = substring(line, kstart, i);
    
    while *line.offset(i as isize) as c_int != 0
        && *line.offset(i as isize) as c_int != ':' as i32
    {
        i += 1;
    }
    if *line.offset(i as isize) as c_int != ':' as i32 {
        builtin_error(
            b"%s: missing colon separator\0" as *const u8 as *const c_char,
            line,
        );
        FREE!(kseq);
        return -(1 as c_int);
    }
   
    i = isolate_sequence(line, i + 1 as c_int, 0 as c_int, &mut kstart);
    if i < 0 {
        FREE!(kseq);
        return -(1 as c_int);
    }
    
    value = substring(line, kstart, i);
    cmd_xmap = get_cmd_xmap_from_keymap(kmap);
    rl_generic_bind(ISMACR as c_int, kseq, value, cmd_xmap);
    
    rl_bind_keyseq_in_map(
        kseq,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> c_int>,
            Option::<rl_command_func_t>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn(c_int, c_int) -> c_int,
                    unsafe extern "C" fn() -> c_int,
                >(bash_execute_unix_command),
            ),
        ),
        kmap,
    );
    free(kseq as *mut c_void);
    return 0 as c_int;
}

#[no_mangle]
pub unsafe extern "C" fn unbind_unix_command(
    mut kseq: *mut c_char,
) -> c_int {
    let mut cmd_xmap: Keymap = 0 as *mut KEYMAP_ENTRY;
    
    cmd_xmap = get_cmd_xmap_from_keymap(rl_get_keymap());
    if rl_bind_keyseq_in_map(
        kseq,
        ::std::mem::transmute::<
            *mut c_void,
            Option::<rl_command_func_t>,
        >(0 as *mut c_void),
        cmd_xmap,
    ) != 0 as c_int
    {
        builtin_error(b"`%s': cannot unbind in command keymap\0" as *const u8 as *const c_char,
            kseq,
        );
        return 0 as c_int;
    }
    return 1 as c_int;
}

#[no_mangle]
pub unsafe extern "C" fn bash_directory_completion_matches(
    mut text: *const c_char,
) -> *mut *mut c_char {
    let mut m1: *mut *mut c_char = 0 as *mut *mut c_char;
    let mut dfn: *mut c_char = 0 as *mut c_char;
    let mut qc: c_int = 0;
    
    qc = if rl_dispatching != 0 {
        rl_completion_quote_character
    } else {
        0 as c_int
    };
    if rl_dispatching != 0 && rl_completion_found_quote == 0 {
        dfn = bash_dequote_filename(text as *mut c_char, qc);
    } else {
        dfn = text as *mut c_char;
    }
    m1 = rl_completion_matches(
        dfn,
        Some(
            rl_filename_completion_function
                as unsafe extern "C" fn(
                    *const c_char,
                    c_int,
                ) -> *mut c_char,
        ),
    );
    
    if dfn != text as *mut c_char {
        free(dfn as *mut c_void );
    }
    
    if m1.is_null() || (*m1.offset(0 as c_int as isize)).is_null() {
        return m1;
    }
    bash_progcomp_ignore_filenames(m1);
    return m1;
}

#[no_mangle]
pub unsafe extern "C" fn bash_dequote_text(
    mut text: *const c_char,
) -> *mut c_char {
    let mut dtxt: *mut c_char = 0 as *mut c_char;
    let mut qc: c_int = 0;
    
    qc = if *text.offset(0 as isize) as c_int == '"' as i32
        || *text.offset(0 as isize) as c_int == '\'' as i32
    {
        *text.offset(0 as isize) as c_int
    } else {
        0 
    };
    dtxt = bash_dequote_filename(text as *mut c_char, qc);
    return dtxt;
}

unsafe extern "C" fn bash_event_hook() -> c_int {
    let mut sig: c_int = 0;
    
    if sigterm_received != 0 {
        return 0 ;
    }
    sig = 0 ;
    if terminating_signal != 0 {
        sig = terminating_signal;
    } else if interrupt_state != 0 {
        sig = SIGINT as c_int;
    } else if sigalrm_seen != 0 {
        sig = SIGALRM as c_int;
    } else {
        sig = first_pending_trap();
    }
    if terminating_signal != 0 || interrupt_state != 0 || sigalrm_seen != 0 {
        rl_cleanup_after_signal();
    }
    bashline_reset_event_hook();
    if posixly_correct != 0
        && this_shell_builtin
            == Some(read_builtin as unsafe extern "C" fn(*mut WordList) -> c_int)
        && sig == 2  
    {
        ::std::ptr::write_volatile(
            &mut last_command_exit_value as *mut c_int,
            128 as c_int | SIGINT as c_int,
        );
        throw_to_top_level();
    }
    check_signals_and_traps();
    return 0 ;
}



