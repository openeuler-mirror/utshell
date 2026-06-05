use crate::alias::{alias_expand, all_aliases, get_alias_value};
use crate::bashhist::{bash_add_history, pre_process_line};
use crate::bracecomp::bash_brace_completion;
use crate::builtins::common::get_working_directory;
use crate::builtins::read::read_builtin;
use crate::builtins::read::sigalrm_seen;
use crate::dispose_cmd::{dispose_word, dispose_words};
use crate::findcmd::{executable_file, executable_or_directory, search_for_command};
use crate::general::bash_tilde_expand;
use crate::general::extract_colon_unit;
use crate::general::file_exists;
use crate::general::tilde_initialize;
use crate::general::{absolute_program, assignment, path_dot_or_dotdot};
use crate::make_cmd::alloc_word_desc;
use crate::parse_and_execute;
use crate::pathexp::{setup_ignore_patterns, shell_glob_filename};
use crate::pcomplete::pcomp_set_readline_variables;
use crate::pcomplete::prog_completion_enabled;
use crate::pcomplete::programmable_completions;
use crate::pcomplib::{progcomp_search, progcomp_size};
use crate::readline::*;
use crate::sig::throw_to_top_level;
use crate::src_common::*;
use crate::stringlib::substring;
use crate::subst::{
    char_is_quoted, expand_prompt_string, expand_word, skip_to_delim, string_list, unclosed_pair,
};
use crate::syntax::sh_syntaxtab;
use crate::trap::{check_signals_and_traps, first_pending_trap};
use crate::variables::{
    all_variables_matching_prefix, all_visible_functions, bind_int_variable, bind_variable,
    check_unbind_variable, find_variable, get_string_value,
};
use crate::version::show_shell_version;
use crate::y_tab::{
    current_command_line_count, current_prompt_string, parser_in_command_position, ps1_prompt,
    reset_readahead_token, restore_parser_state, save_parser_state, saved_command_line_count,
    word_token_alist,
};
use std::convert::TryInto;

extern "C" {
    static mut stdin: *mut FILE;
}

#[macro_export]
macro_rules! BACKUP_CHAR {
    ($_str:expr, $_strsize:expr, $_i:expr,$state:expr) => {
        if locale_mb_cur_max > 1 as libc::c_int {
            let mut state_bak: mbstate_t;
            let mut mblength: size_t;
            let mut _x: libc::c_int = 0;
            let mut _p: libc::c_int = 0;
            _p = 0 as libc::c_int;
            _x = _p;
            while _x < $_i {
                state_bak = $state;
                mblength = mbrlen(
                    $_str.offset(_x as isize),
                    $_strsize.wrapping_sub(_x as libc::c_ulong),
                    &mut $state,
                ) as u64;
                if mblength == -(2 as libc::c_int) as size_t
                    || mblength == -(1 as libc::c_int) as size_t
                {
                    $state = state_bak;
                    _x += 1;
                } else if mblength == 0 as libc::c_int as libc::c_ulong {
                    _x += 1;
                } else {
                    _p = _x;
                    _x = (_x as libc::c_ulong).wrapping_add(mblength) as libc::c_int as libc::c_int;
                }
            }
            $_i = _p;
        } else {
            $_i -= 1;
        }
    };
}
#[no_mangle]
pub fn posix_readline_initialize(on_or_off: libc::c_int) {
    static mut kseq: [libc::c_char; 2] = [CTRL!('I' as i32) as libc::c_char, 0 as libc::c_char];
    unsafe {
        if on_or_off != 0 {
            c_rl_variable_bind(
                b"comment-begin\0" as *const u8 as *const libc::c_char,
                b"#\0" as *const u8 as *const libc::c_char,
            );
        }
        if on_or_off != 0 {
            vi_tab_binding = c_rl_function_of_keyseq(
                kseq.as_mut_ptr(),
                vi_insertion_keymap.as_mut_ptr(),
                0 as *mut libc::c_void as *mut libc::c_int,
            );
            c_rl_bind_key_in_map(
                CTRL!('I' as i32) as libc::c_int,
                Some(c_rl_insert),
                vi_insertion_keymap.as_mut_ptr(),
            );
        } else if c_rl_function_of_keyseq(
            kseq.as_mut_ptr(),
            vi_insertion_keymap.as_mut_ptr(),
            0 as *mut c_void as *mut libc::c_int,
        ) == Some(c_rl_insert)
        {
            c_rl_bind_key_in_map(
                CTRL!('I' as i32) as libc::c_int,
                vi_tab_binding,
                vi_insertion_keymap.as_mut_ptr(),
            );
        }
    }
}

#[no_mangle]
pub fn reset_completer_word_break_chars() {
    unsafe {
        rl_completer_word_break_characters = if perform_hostname_completion != 0 {
            savestring!(bash_completer_word_break_characters)
        } else {
            savestring!(bash_nohostname_word_break_characters)
        };
    }
}

#[no_mangle]
pub fn enable_hostname_completion(on_or_off: libc::c_int) -> libc::c_int {
    let old_value: libc::c_int;
    let mut at: *mut libc::c_char;
    let mut nv: *mut libc::c_char;
    let nval: *mut libc::c_char;
    unsafe {
        old_value = perform_hostname_completion;

        if on_or_off != 0 {
            perform_hostname_completion = 1 as libc::c_int;
            rl_special_prefixes = b"$@\0" as *const u8 as *const libc::c_char;
        } else {
            perform_hostname_completion = 0 as libc::c_int;
            rl_special_prefixes = b"$\0" as *const u8 as *const libc::c_char;
        }
        if bash_readline_initialized == 0 as libc::c_int
            && (rl_completer_word_break_characters.is_null()
                || rl_completer_word_break_characters
                    == rl_basic_word_break_characters as *mut libc::c_char)
        {
            if on_or_off != 0 {
                rl_completer_word_break_characters =
                    savestring!(bash_completer_word_break_characters);
            } else {
                rl_completer_word_break_characters =
                    savestring!(bash_nohostname_word_break_characters);
            }
        } else {
            at = libc::strchr(rl_completer_word_break_characters, '@' as i32);
            if at.is_null() && on_or_off == 0 as libc::c_int
                || !at.is_null() && on_or_off != 0 as libc::c_int
            {
                return old_value;
            }
            nval = libc::malloc(
                ((libc::strlen(rl_completer_word_break_characters)) + 1 + on_or_off as usize)
                    as usize,
            ) as *mut libc::c_char;

            if on_or_off == 0 as libc::c_int {
                nv = nval;
                at = rl_completer_word_break_characters;
                while *at != 0 {
                    if *at as libc::c_int != '@' as i32 {
                        let fresh0 = at;
                        at = at.offset(1);
                        let fresh1 = nv;
                        nv = nv.offset(1);
                        *fresh1 = *fresh0;
                    } else {
                        at = at.offset(1);
                    }
                }
                *nv = '\u{0}' as i32 as libc::c_char;
            } else {
                *nval.offset(0 as libc::c_int as isize) = '@' as i32 as libc::c_char;
                libc::strcpy(
                    nval.offset(1 as libc::c_int as isize),
                    rl_completer_word_break_characters,
                );
            }
            libc::free(rl_completer_word_break_characters as *mut c_void);
            rl_completer_word_break_characters = nval;
        }
    }
    return old_value;
}

#[no_mangle]
pub fn initialize_readline() {
    let mut func: Option<rl_command_func_t>;
    let mut kseq: [libc::c_char; 2] = [0; 2];
    unsafe {
        if bash_readline_initialized != 0 {
            return;
        }
        rl_terminal_name = get_string_value(b"TERM\0" as *const u8 as *const libc::c_char);
        rl_instream = stdin;
        rl_outstream = stderr;

        rl_readline_name = b"Bash\0" as *const u8 as *const libc::c_char;
    }
    c_rl_add_defun(
        b"shell-expand-line\0" as *const u8 as *const libc::c_char,
        Some(shell_expand_line),
        -(1 as libc::c_int),
    );
    c_rl_add_defun(
        b"history-expand-line\0" as *const u8 as *const libc::c_char,
        Some(history_expand_line),
        -(1 as libc::c_int),
    );
    c_rl_add_defun(
        b"magic-space\0" as *const u8 as *const libc::c_char,
        Some(tcsh_magic_space),
        -(1 as libc::c_int),
    );
    c_rl_add_defun(
        b"shell-forward-word\0" as *const u8 as *const libc::c_char,
        Some(bash_forward_shellword),
        -(1 as libc::c_int),
    );
    c_rl_add_defun(
        b"shell-backward-word\0" as *const u8 as *const libc::c_char,
        Some(bash_backward_shellword),
        -(1 as libc::c_int),
    );
    c_rl_add_defun(
        b"shell-kill-word\0" as *const u8 as *const libc::c_char,
        Some(bash_kill_shellword),
        -(1 as libc::c_int),
    );
    c_rl_add_defun(
        b"shell-backward-kill-word\0" as *const u8 as *const libc::c_char,
        Some(bash_backward_kill_shellword),
        -(1 as libc::c_int),
    );
    c_rl_add_defun(
        b"shell-transpose-words\0" as *const u8 as *const libc::c_char,
        Some(bash_transpose_shellwords),
        -(1 as libc::c_int),
    );
    c_rl_add_defun(
        b"alias-expand-line\0" as *const u8 as *const libc::c_char,
        Some(alias_expand_line),
        -(1 as libc::c_int),
    );
    c_rl_add_defun(
        b"history-and-alias-expand-line\0" as *const u8 as *const libc::c_char,
        Some(history_and_alias_expand_line),
        -(1 as libc::c_int),
    );
    c_rl_add_defun(
        b"insert-last-argument\0" as *const u8 as *const libc::c_char,
        Some(c_rl_yank_last_arg),
        -(1 as libc::c_int),
    );
    c_rl_add_defun(
        b"display-shell-version\0" as *const u8 as *const libc::c_char,
        Some(display_shell_version),
        -(1 as libc::c_int),
    );
    c_rl_add_defun(
        b"edit-and-execute-command\0" as *const u8 as *const libc::c_char,
        Some(emacs_edit_and_execute_command),
        -(1 as libc::c_int),
    );
    c_rl_add_defun(
        b"complete-into-braces\0" as *const u8 as *const libc::c_char,
        Some(bash_brace_completion),
        -(1 as libc::c_int),
    );
    c_rl_add_defun(
        b"complete-filename\0" as *const u8 as *const libc::c_char,
        Some(bash_complete_filename),
        -(1 as libc::c_int),
    );
    c_rl_add_defun(
        b"possible-filename-completions\0" as *const u8 as *const libc::c_char,
        Some(bash_possible_filename_completions),
        -(1 as libc::c_int),
    );
    c_rl_add_defun(
        b"complete-username\0" as *const u8 as *const libc::c_char,
        Some(bash_complete_username),
        -(1 as libc::c_int),
    );
    c_rl_add_defun(
        b"possible-username-completions\0" as *const u8 as *const libc::c_char,
        Some(bash_possible_username_completions),
        -(1 as libc::c_int),
    );
    c_rl_add_defun(
        b"complete-hostname\0" as *const u8 as *const libc::c_char,
        Some(bash_complete_hostname),
        -(1 as libc::c_int),
    );
    c_rl_add_defun(
        b"possible-hostname-completions\0" as *const u8 as *const libc::c_char,
        Some(bash_possible_hostname_completions),
        -(1 as libc::c_int),
    );
    c_rl_add_defun(
        b"complete-variable\0" as *const u8 as *const libc::c_char,
        Some(bash_complete_variable),
        -(1 as libc::c_int),
    );
    c_rl_add_defun(
        b"possible-variable-completions\0" as *const u8 as *const libc::c_char,
        Some(bash_possible_variable_completions),
        -(1 as libc::c_int),
    );
    c_rl_add_defun(
        b"complete-command\0" as *const u8 as *const libc::c_char,
        Some(bash_complete_command),
        -(1 as libc::c_int),
    );
    c_rl_add_defun(
        b"possible-command-completions\0" as *const u8 as *const libc::c_char,
        Some(bash_possible_command_completions),
        -(1 as libc::c_int),
    );
    c_rl_add_defun(
        b"glob-complete-word\0" as *const u8 as *const libc::c_char,
        Some(bash_glob_complete_word),
        -(1 as libc::c_int),
    );
    c_rl_add_defun(
        b"glob-expand-word\0" as *const u8 as *const libc::c_char,
        Some(bash_glob_expand_word),
        -(1 as libc::c_int),
    );
    c_rl_add_defun(
        b"glob-list-expansions\0" as *const u8 as *const libc::c_char,
        Some(bash_glob_list_expansions),
        -(1 as libc::c_int),
    );
    c_rl_add_defun(
        b"dynamic-complete-history\0" as *const u8 as *const libc::c_char,
        Some(dynamic_complete_history),
        -(1 as libc::c_int),
    );
    c_rl_add_defun(
        b"dabbrev-expand\0" as *const u8 as *const libc::c_char,
        Some(bash_dabbrev_expand),
        -(1 as libc::c_int),
    );
    if unsafe {
        rl_readline_state & 0x2 as libc::c_int as libc::c_ulong == 0 as libc::c_int as libc::c_ulong
    } {
        c_rl_initialize();
    }
    unsafe {
        c_rl_bind_key_if_unbound_in_map(
            'E' as i32 & 0x1f as libc::c_int,
            Some(shell_expand_line),
            emacs_meta_keymap.as_mut_ptr(),
        );
        c_rl_bind_key_if_unbound_in_map(
            '^' as i32,
            Some(history_expand_line),
            emacs_meta_keymap.as_mut_ptr(),
        );
        c_rl_bind_key_if_unbound_in_map(
            'V' as i32 & 0x1f as libc::c_int,
            Some(display_shell_version),
            emacs_ctlx_keymap.as_mut_ptr(),
        );
    }
    kseq[0 as usize] = CTRL!('J' as i32) as libc::c_char;
    kseq[1 as usize] = '\u{0}' as i32 as libc::c_char;
    unsafe {
        func = c_rl_function_of_keyseq(
            kseq.as_mut_ptr(),
            emacs_meta_keymap.as_mut_ptr(),
            0 as *mut libc::c_void as *mut libc::c_int,
        );
    }
    if func == Some(c_rl_vi_editing_mode) {
        unsafe {
            c_rl_unbind_key_in_map(
                CTRL!('J' as i32) as libc::c_int,
                emacs_meta_keymap.as_mut_ptr(),
            );
        }
    }
    kseq[0 as usize] = CTRL!('M' as i32) as libc::c_char;
    unsafe {
        func = c_rl_function_of_keyseq(
            kseq.as_mut_ptr(),
            emacs_meta_keymap.as_mut_ptr(),
            0 as *mut c_void as *mut libc::c_int,
        );
    }
    if func == Some(c_rl_vi_editing_mode) {
        unsafe {
            c_rl_unbind_key_in_map(
                CTRL!('M' as i32) as libc::c_int,
                emacs_meta_keymap.as_mut_ptr(),
            );
        }
    }
    kseq[0 as usize] = CTRL!('E' as i32) as libc::c_char;
    unsafe {
        func = c_rl_function_of_keyseq(
            kseq.as_mut_ptr(),
            vi_movement_keymap.as_mut_ptr(),
            0 as *mut c_void as *mut libc::c_int,
        );
    }
    if func == Some(c_rl_emacs_editing_mode) {
        unsafe {
            c_rl_unbind_key_in_map(
                CTRL!('E' as i32) as libc::c_int,
                vi_movement_keymap.as_mut_ptr(),
            );
        }
    }
    unsafe {
        c_rl_bind_key_if_unbound_in_map(
            '{' as i32,
            Some(bash_brace_completion),
            emacs_meta_keymap.as_mut_ptr(),
        );
        c_rl_bind_key_if_unbound_in_map(
            '/' as i32,
            Some(bash_complete_filename),
            emacs_meta_keymap.as_mut_ptr(),
        );
        c_rl_bind_key_if_unbound_in_map(
            '/' as i32,
            Some(bash_possible_filename_completions),
            emacs_ctlx_keymap.as_mut_ptr(),
        );
    }
    kseq[0 as libc::c_int as usize] = '~' as i32 as libc::c_char;
    kseq[1 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    unsafe {
        func = c_rl_function_of_keyseq(
            kseq.as_mut_ptr(),
            emacs_meta_keymap.as_mut_ptr(),
            0 as *mut c_void as *mut libc::c_int,
        );
    }
    if func.is_none() || func == Some(c_rl_tilde_expand) {
        unsafe {
            c_rl_bind_keyseq_in_map(
                kseq.as_mut_ptr(),
                Some(bash_complete_username),
                emacs_meta_keymap.as_mut_ptr(),
            );
        }
    }
    unsafe {
        c_rl_bind_key_if_unbound_in_map(
            '~' as i32,
            Some(bash_possible_username_completions),
            emacs_ctlx_keymap.as_mut_ptr(),
        );
        c_rl_bind_key_if_unbound_in_map(
            '@' as i32,
            Some(bash_complete_hostname),
            emacs_meta_keymap.as_mut_ptr(),
        );
        c_rl_bind_key_if_unbound_in_map(
            '@' as i32,
            Some(bash_possible_hostname_completions),
            emacs_ctlx_keymap.as_mut_ptr(),
        );
        c_rl_bind_key_if_unbound_in_map(
            '$' as i32,
            Some(bash_complete_variable),
            emacs_meta_keymap.as_mut_ptr(),
        );
        c_rl_bind_key_if_unbound_in_map(
            '$' as i32,
            Some(bash_possible_variable_completions),
            emacs_ctlx_keymap.as_mut_ptr(),
        );
        c_rl_bind_key_if_unbound_in_map(
            '!' as i32,
            Some(bash_complete_command),
            emacs_meta_keymap.as_mut_ptr(),
        );
        c_rl_bind_key_if_unbound_in_map(
            '!' as i32,
            Some(bash_possible_command_completions),
            emacs_ctlx_keymap.as_mut_ptr(),
        );
        c_rl_bind_key_if_unbound_in_map(
            'g' as i32,
            Some(bash_glob_complete_word),
            emacs_meta_keymap.as_mut_ptr(),
        );
        c_rl_bind_key_if_unbound_in_map(
            '*' as i32,
            Some(bash_glob_expand_word),
            emacs_ctlx_keymap.as_mut_ptr(),
        );
        c_rl_bind_key_if_unbound_in_map(
            'g' as i32,
            Some(bash_glob_list_expansions),
            emacs_ctlx_keymap.as_mut_ptr(),
        );
    }
    kseq[0 as libc::c_int as usize] = '\t' as i32 as libc::c_char;
    kseq[1 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    unsafe {
        func = c_rl_function_of_keyseq(
            kseq.as_mut_ptr(),
            emacs_meta_keymap.as_mut_ptr(),
            0 as *mut c_void as *mut libc::c_int,
        );
    }
    if func.is_none() || func == Some(c_rl_tab_insert) {
        unsafe {
            c_rl_bind_key_in_map(
                '\t' as i32,
                Some(dynamic_complete_history),
                emacs_meta_keymap.as_mut_ptr(),
            );
        }
    }
    unsafe {
        rl_attempted_completion_function = Some(attempt_shell_completion);
        set_directory_hook();
        rl_filename_rewrite_hook = Some(bash_filename_rewrite_hook);
        rl_filename_stat_hook = Some(bash_filename_stat_hook);
        rl_ignore_some_completions_function = Some(filename_completion_ignore);
        c_rl_bind_key_if_unbound_in_map(
            'E' as i32 & 0x1f as libc::c_int,
            Some(emacs_edit_and_execute_command),
            emacs_ctlx_keymap.as_mut_ptr(),
        );
        c_rl_bind_key_if_unbound_in_map(
            'v' as i32,
            Some(vi_edit_and_execute_command),
            vi_movement_keymap.as_mut_ptr(),
        );
        c_rl_bind_key_if_unbound_in_map(
            '@' as i32,
            Some(posix_edit_macros),
            vi_movement_keymap.as_mut_ptr(),
        );
        c_rl_bind_key_in_map(
            '\\' as i32,
            Some(bash_vi_complete),
            vi_movement_keymap.as_mut_ptr(),
        );
        c_rl_bind_key_in_map(
            '*' as i32,
            Some(bash_vi_complete),
            vi_movement_keymap.as_mut_ptr(),
        );
        c_rl_bind_key_in_map(
            '=' as i32,
            Some(bash_vi_complete),
            vi_movement_keymap.as_mut_ptr(),
        );
        rl_completer_quote_characters = b"'\"\0" as *const u8 as *const libc::c_char;
        enable_hostname_completion(perform_hostname_completion);
        rl_filename_quote_characters = default_filename_quote_characters;
        set_filename_bstab(rl_filename_quote_characters);
        rl_filename_quoting_function = Some(bash_quote_filename);
        rl_filename_dequoting_function = Some(bash_dequote_filename);
        rl_char_is_quoted_p = Some(char_is_quoted);
        c_rl_bind_key_if_unbound_in_map(
            'B' as i32 & 0x1f as libc::c_int,
            Some(bash_backward_shellword),
            emacs_meta_keymap.as_mut_ptr(),
        );
        c_rl_bind_key_if_unbound_in_map(
            'D' as i32 & 0x1f as libc::c_int,
            Some(bash_kill_shellword),
            emacs_meta_keymap.as_mut_ptr(),
        );
        c_rl_bind_key_if_unbound_in_map(
            'F' as i32 & 0x1f as libc::c_int,
            Some(bash_forward_shellword),
            emacs_meta_keymap.as_mut_ptr(),
        );
        c_rl_bind_key_if_unbound_in_map(
            'T' as i32 & 0x1f as libc::c_int,
            Some(bash_transpose_shellwords),
            emacs_meta_keymap.as_mut_ptr(),
        );
        bash_readline_initialized = 1 as libc::c_int;
    }
}

#[no_mangle]
pub fn bashline_reinitialize() {
    unsafe {
        bash_readline_initialized = 0;
    }
}
#[no_mangle]
pub fn bashline_set_event_hook() {
    unsafe {
        rl_signal_event_hook = Some(bash_event_hook);
    }
}
#[no_mangle]
pub fn bashline_reset_event_hook() {
    unsafe {
        rl_signal_event_hook = None;
    }
}

#[no_mangle]
pub fn bashline_reset() {
    tilde_initialize();
    unsafe {
        rl_attempted_completion_function = Some(attempt_shell_completion);
        rl_completion_entry_function = None;
        rl_ignore_some_completions_function = Some(filename_completion_ignore);
        rl_filename_quote_characters = default_filename_quote_characters;
        set_filename_bstab(rl_filename_quote_characters);

        set_directory_hook();
        rl_filename_stat_hook = Some(bash_filename_stat_hook);

        bashline_reset_event_hook();

        rl_sort_completion_matches = 1 as libc::c_int;
    }
}

static mut push_to_readline: *mut libc::c_char = 0 as *mut libc::c_char;

fn bash_push_line() -> libc::c_int {
    unsafe {
        if !push_to_readline.is_null() {
            c_rl_insert_text(push_to_readline);
            libc::free(push_to_readline as *mut c_void);
            push_to_readline = 0 as *mut libc::c_char;
            rl_startup_hook = old_rl_startup_hook;
        }
    }
    return 0;
}

#[no_mangle]
pub fn bash_re_edit(line: *mut libc::c_char) -> libc::c_int {
    unsafe {
        FREE!(push_to_readline);

        push_to_readline = savestring!(line);
        old_rl_startup_hook = rl_startup_hook;
        rl_startup_hook = Some(bash_push_line);
    }
    return 0;
}

fn display_shell_version(_count: libc::c_int, _c: libc::c_int) -> libc::c_int {
    c_rl_crlf();
    show_shell_version(0);
    unsafe {
        c_putc('\r' as i32, rl_outstream);
        fflush(rl_outstream);
    }
    c_rl_on_new_line();
    c_rl_redisplay();

    return 0;
}

static mut hostname_list: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;

static mut hostname_list_size: libc::c_int = 0;

static mut hostname_list_length: libc::c_int = 0;

#[no_mangle]
pub static mut hostname_list_initialized: libc::c_int = 0;
fn initialize_hostname_list() {
    let mut temp: *mut libc::c_char;

    temp = get_string_value(b"HOSTFILE\0" as *const u8 as *const libc::c_char);
    if temp.is_null() {
        temp = get_string_value(b"hostname_completion_file\0" as *const u8 as *const libc::c_char);
    }
    if temp.is_null() {
        temp = b"/etc/hosts\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }

    snarf_hosts_from_file(temp);
    unsafe {
        if !hostname_list.is_null() {
            hostname_list_initialized += 1;
        }
    }
}

fn add_host_name(name: *mut libc::c_char) {
    unsafe {
        if hostname_list_length + 2 > hostname_list_size {
            hostname_list_size = hostname_list_size + 32 - hostname_list_size % 32;
            hostname_list = c_strvec_resize(hostname_list, hostname_list_size);
        }
        //有可能存在错误的地方
        *hostname_list.offset(hostname_list_length as isize) = savestring!(name);
        hostname_list_length = hostname_list_length + 1;
        *hostname_list.offset(hostname_list_length as isize) = 0 as *mut libc::c_char;
    }
}

fn snarf_hosts_from_file(filename: *mut libc::c_char) {
    let file: *mut FILE;
    let mut temp: *mut libc::c_char;
    let mut buffer: [libc::c_char; 256] = [0; 256];
    let mut name: [libc::c_char; 256] = [0; 256];
    let mut i: libc::c_int;
    let mut start: libc::c_int;

    file = unsafe { libc::fopen(filename, b"r\0" as *const u8 as *const libc::c_char) };
    if file.is_null() {
        return;
    }

    loop {
        temp = unsafe { libc::fgets(buffer.as_mut_ptr(), 255 as libc::c_int, file) };
        if temp.is_null() {
            break;
        }

        i = 0;
        while buffer[i as usize] as libc::c_int != 0 && cr_whitespace!(buffer[i as usize]) {
            i += 1;
        }

        if buffer[i as usize] as libc::c_int == '\u{0}' as i32
            || buffer[i as usize] as libc::c_int == '#' as i32
        {
            continue;
        }

        if unsafe {
            libc::strncmp(
                buffer.as_mut_ptr().offset(i as isize),
                b"$include \0" as *const u8 as *const libc::c_char,
                (9 as libc::c_int as libc::c_ulong).try_into().unwrap(),
            ) == 0
        } {
            let mut incfile: *mut libc::c_char;
            let mut t: *mut libc::c_char;

            incfile = unsafe { buffer.as_mut_ptr().offset(i as isize).offset(9 as isize) };
            unsafe {
                while *incfile as libc::c_int != 0 && whitespace!(*incfile) {
                    incfile = incfile.offset(1);
                }

                t = incfile;
                while *t as libc::c_int != 0 && cr_whitespace!(*t as libc::c_int) == false {
                    t = t.offset(1);
                }
                *t = '\u{0}' as i32 as libc::c_char;
            }
            snarf_hosts_from_file(incfile);
        } else {
            if DIGIT![i as usize] {
                while buffer[i as usize] as libc::c_int != 0
                    && cr_whitespace!(buffer[i as usize]) == false
                {
                    i += 1;
                }
            }

            while buffer[i as usize] != 0 {
                while cr_whitespace!(buffer[i as usize]) {
                    i += 1;
                }
                if buffer[i as usize] as libc::c_int == '\u{0}' as i32
                    || buffer[i as usize] as libc::c_int == '#' as i32
                {
                    break;
                }
                start = i;
                while buffer[i as usize] as libc::c_int != 0
                    && cr_whitespace!(buffer[i as usize]) == false
                {
                    i += 1;
                }

                if i == start {
                    continue;
                }
                unsafe {
                    libc::strncpy(
                        name.as_mut_ptr(),
                        buffer.as_mut_ptr().offset(start as isize),
                        ((i - start) as libc::c_ulong).try_into().unwrap(),
                    );
                }
                name[(i - start) as usize] = '\u{0}' as i32 as libc::c_char;
                add_host_name(name.as_mut_ptr());
            }
        }
    }
    unsafe {
        libc::fclose(file);
    }
}

#[no_mangle]
pub fn get_hostname_list() -> *mut *mut libc::c_char {
    unsafe {
        if hostname_list_initialized == 0 as libc::c_int {
            initialize_hostname_list();
        }
        return hostname_list;
    }
}

#[no_mangle]
pub fn clear_hostname_list() {
    let mut i: libc::c_int;
    unsafe {
        if hostname_list_initialized == 0 as libc::c_int {
            return;
        }
        i = 0;
        while i < hostname_list_length {
            libc::free(*hostname_list.offset(i as isize) as *mut c_void);
            i += 1;
        }
        hostname_list_initialized = 0;
        hostname_list_length = hostname_list_initialized;
    }
}

fn hostnames_matching(text: *mut libc::c_char) -> *mut *mut libc::c_char {
    let mut i: libc::c_int;
    let len: libc::c_int;
    let mut nmatch: libc::c_int;
    let mut rsize: libc::c_int;
    let mut result: *mut *mut libc::c_char;
    unsafe {
        if hostname_list_initialized == 0 {
            initialize_hostname_list();
        }

        if hostname_list_initialized == 0 {
            return 0 as *mut *mut libc::c_char;
        }

        if *text as libc::c_int == '\u{0}' as i32 {
            result = c_strvec_create(1 + hostname_list_length);
            i = 0;
            while i < hostname_list_length {
                *result.offset(i as isize) = *hostname_list.offset(i as isize);
                i += 1;
            }
            *result.offset(i as isize) = 0 as *mut libc::c_char;
            return result;
        }

        len = libc::strlen(text) as libc::c_int;
        result = 0 as *mut *mut libc::c_char;

        rsize = 0 as libc::c_int;
        nmatch = rsize;
        i = nmatch;
        while i < hostname_list_length {
            if STREQN!(text, *hostname_list.offset(i as isize), len as usize) as i32 == 0 {
                continue;
            }

            if nmatch >= rsize - 1 {
                rsize = rsize + 16 - rsize % 16;
                result = c_strvec_resize(result, rsize);
            }

            *result.offset(nmatch as isize) = *hostname_list.offset(i as isize);
            nmatch = nmatch + 1;

            i += 1;
        }

        if nmatch != 0 {
            *result.offset(nmatch as isize) = 0 as *mut libc::c_char;
        }
    }
    return result;
}

fn edit_and_execute_command(
    count: libc::c_int,
    c: libc::c_int,
    editing_mode: libc::c_int,
    edit_command: *mut libc::c_char,
) -> libc::c_int {
    let command: *mut libc::c_char;
    let metaval: *mut libc::c_char;
    let r: libc::c_int;
    let rrs: libc::c_int;
    let metaflag: libc::c_int;
    let mut ps: sh_parser_state_t = sh_parser_state_t {
        parser_state: 0,
        token_state: 0 as *mut libc::c_int,
        token: 0 as *mut libc::c_char,
        token_buffer_size: 0,
        input_line_terminator: 0,
        eof_encountered: 0,
        prompt_string_pointer: 0 as *mut *mut libc::c_char,
        current_command_line_count: 0,
        remember_on_history: 0,
        history_expansion_inhibited: 0,
        last_command_exit_value: 0,
        pipestatus: 0 as *mut ARRAY,
        last_shell_builtin: None,
        this_shell_builtin: None,
        expand_aliases: 0,
        echo_input_at_read: 0,
        need_here_doc: 0,
        here_doc_first_line: 0,
        redir_stack: [0 as *mut REDIRECT; 16],
    };
    unsafe {
        rrs = rl_readline_state as libc::c_int;
        saved_command_line_count = current_command_line_count;
    }
    c_rl_newline(1, c);
    unsafe {
        if rl_explicit_arg != 0 {
            command = libc::malloc((libc::strlen(edit_command)).wrapping_add(8 as usize) as usize)
                as *mut libc::c_char;
            sprintf(
                command,
                b"%s %d\0" as *const u8 as *const libc::c_char,
                edit_command,
                count,
            );
        } else {
            c_using_history();
            current_command_line_count += 1;
            bash_add_history(rl_line_buffer);
            current_command_line_count = 0 as libc::c_int;
            bash_add_history(b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
            history_lines_this_session += 1;
            c_using_history();
            command = savestring!(edit_command);
        }
    }
    metaval = c_rl_variable_value(b"input-meta\0" as *const u8 as *const libc::c_char);
    metaflag = unsafe { RL_BOOLEAN_VARIABLE_VALUE!(metaval) as libc::c_int };
    unsafe {
        if rl_deprep_term_function.is_some() {
            (Some(rl_deprep_term_function.expect("non-null function pointer")))
                .expect("non-null function pointer")();
        }
    }
    c_rl_clear_signals();
    save_parser_state(&mut ps);
    r = parse_and_execute(
        command,
        if editing_mode == 0 as libc::c_int {
            b"v\0" as *const u8 as *const libc::c_char
        } else {
            b"C-xC-e\0" as *const u8 as *const libc::c_char
        },
        SEVAL_NOHIST as libc::c_int,
    );
    restore_parser_state(&mut ps);

    reset_readahead_token();
    unsafe {
        if rl_prep_term_function.is_some() {
            (Some(rl_prep_term_function.expect("non-null function pointer")))
                .expect("non-null function pointer")(metaflag);
        }
    }
    c_rl_set_signals();
    unsafe {
        current_command_line_count = saved_command_line_count;

        *rl_line_buffer.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
        rl_end = 0 as libc::c_int;
        rl_point = rl_end;
        rl_done = 0 as libc::c_int;
        rl_readline_state = rrs as libc::c_ulong;
    }
    if editing_mode == 0 {
        c_rl_vi_insertion_mode(1 as libc::c_int, c);
    }
    c_rl_forced_update_display();

    return r;
}

fn vi_edit_and_execute_command(count: libc::c_int, c: libc::c_int) -> libc::c_int {
    if unsafe { posixly_correct != 0 } {
        return edit_and_execute_command(count, c, 0 as libc::c_int, POSIX_VI_EDIT_COMMAND!());
    } else {
        return edit_and_execute_command(count, c, 0 as libc::c_int, VI_EDIT_COMMAND!());
    };
}

fn emacs_edit_and_execute_command(count: libc::c_int, c: libc::c_int) -> libc::c_int {
    return edit_and_execute_command(count, c, EMACS_EDITING_MODE!(), EMACS_EDIT_COMMAND!());
}

fn posix_edit_macros(_count: libc::c_int, _key: libc::c_int) -> libc::c_int {
    let c: libc::c_int;
    let mut alias_name: [libc::c_char; 3] = [0; 3];
    let alias_value: *mut libc::c_char;
    let macro_0: *mut libc::c_char;

    c = c_rl_read_key();
    alias_name[0 as usize] = '_' as i32 as libc::c_char;
    alias_name[1 as usize] = c as libc::c_char;
    alias_name[2 as usize] = '\u{0}' as i32 as libc::c_char;

    alias_value = get_alias_value(alias_name.as_mut_ptr());
    if unsafe { !alias_value.is_null() && *alias_value as libc::c_int != 0 } {
        unsafe {
            macro_0 = savestring!(alias_value);
        }
        c_rl_push_macro_input(macro_0);
    }

    return 0;
}

fn is_basic(c: libc::c_char) -> libc::c_int {
    unsafe {
        return (*is_basic_table
            .as_ptr()
            .offset((c as libc::c_uchar as libc::c_int >> 5 as libc::c_int) as isize)
            >> (c as libc::c_uchar as libc::c_int & 31 as libc::c_int)
            & 1 as libc::c_int as libc::c_uint) as libc::c_int;
    }
}

fn bash_forward_shellword(mut count: libc::c_int, key: libc::c_int) -> libc::c_int {
    let slen: size_t;
    let mut c: libc::c_int = 0;
    let mut p: libc::c_int;
    let mut state: mbstate_t = __mbstate_t {
        __count: 0,
        __value: mbstate_t_value { __wch: 0 },
    };
    unsafe {
        DECLARE_MBSTATE!(state);

        if count < 0 {
            return bash_backward_shellword(-count, key);
        }

        p = rl_point;
        slen = rl_end as size_t;

        while count != 0 {
            if p == rl_end {
                rl_point = rl_end;
                return 0;
            }

            if char_is_quoted(rl_line_buffer, p) != 0
                && p > 0
                && *rl_line_buffer.offset((p - 1) as isize) as libc::c_int != '\\' as i32
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
                        c = *rl_line_buffer.offset(p as isize) as libc::c_int;
                        c != 0
                    }
                    && WORDDELIM!(c)
                {
                    let optu8: u8 = c as u8;
                    let optChar: char = char::from(optu8);
                    match optChar {
                        '\\' => {
                            if p < rl_end && *rl_line_buffer.offset(p as isize) as libc::c_int != 0
                            {
                                ADVANCE_CHAR!(rl_line_buffer, slen, p, state);
                            }
                        }
                        '\'' => {
                            p += 1;
                            p = skip_to_delim(
                                rl_line_buffer,
                                p,
                                b"'\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                                0x1 as libc::c_int,
                            );
                        }
                        '"' => {
                            p += 1;
                            p = skip_to_delim(
                                rl_line_buffer,
                                p,
                                b"\"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                                0x1 as libc::c_int,
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

                if *rl_line_buffer.offset(p as isize) as libc::c_int == 0 || p == rl_end {
                    rl_point = rl_end;
                    c_rl_ding();
                    return 0;
                }

                while p < rl_end
                    && {
                        c = *rl_line_buffer.offset(p as isize) as libc::c_int;
                        c != 0
                    }
                    && WORDDELIM!(c) == false
                {
                    let optu8: u8 = c as u8;
                    let optChar: char = char::from(optu8);
                    match optChar {
                        '\\' => {
                            if p < rl_end && *rl_line_buffer.offset(p as isize) as libc::c_int != 0
                            {
                                ADVANCE_CHAR!(rl_line_buffer, slen, p, state);
                            }
                        }
                        '\'' => {
                            p += 1;
                            p = skip_to_delim(
                                rl_line_buffer,
                                p,
                                b"'\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                                0x1 as libc::c_int,
                            );
                        }
                        '"' => {
                            p += 1;
                            p = skip_to_delim(
                                rl_line_buffer,
                                p,
                                b"\"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                                0x1 as libc::c_int,
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
                    || *rl_line_buffer.offset(p as isize) as libc::c_int == 0 as libc::c_int
                {
                    rl_point = rl_end;
                    return 0;
                }
                count -= 1;
            }
        }
        rl_point = p;
    }
    return 0;
}

fn bash_backward_shellword(mut count: libc::c_int, key: libc::c_int) -> libc::c_int {
    let slen: size_t;
    let mut c: libc::c_int;
    let mut p: libc::c_int;
    let mut prev_p: libc::c_int;
    let mut state: mbstate_t = __mbstate_t {
        __count: 0,
        __value: mbstate_t_value { __wch: 0 },
    };
    unsafe {
        memset(
            &mut state as *mut mbstate_t as *mut c_void,
            '\u{0}' as i32,
            (::std::mem::size_of::<mbstate_t>() as libc::c_ulong) as usize,
        );
        if count < 0 {
            return bash_forward_shellword(-count, key);
        }
        p = rl_point;
        slen = rl_end as size_t;

        while count != 0 {
            if p == 0 {
                rl_point = 0;
                return 0;
            }

            BACKUP_CHAR!(rl_line_buffer, slen, p, state);
            while p > 0 {
                c = *rl_line_buffer.offset(p as isize) as libc::c_int;
                if WORDDELIM!(c) == false || char_is_quoted(rl_line_buffer, p) != 0 {
                    break;
                }
                BACKUP_CHAR!(rl_line_buffer, slen, p, state);
            }

            if p == 0 {
                rl_point = 0;
                return 0;
            }

            prev_p = p;
            while p > 0 {
                c = *rl_line_buffer.offset(p as isize) as libc::c_int;
                if WORDDELIM!(c) && char_is_quoted(rl_line_buffer, p) == 0 as libc::c_int {
                    p = prev_p;
                    break;
                } else {
                    prev_p = p;
                    BACKUP_CHAR!(rl_line_buffer, slen, p, state);
                }
            }
            count -= 1;
        }
        rl_point = p;
    }
    return 0;
}

fn bash_kill_shellword(count: libc::c_int, key: libc::c_int) -> libc::c_int {
    let p: libc::c_int;

    if count < 0 {
        return bash_backward_kill_shellword(-count, key);
    }
    unsafe {
        p = rl_point;
        bash_forward_shellword(count, key);

        if rl_point != p {
            c_rl_kill_text(p, rl_point);
        }

        rl_point = p;
        if rl_editing_mode == 1 {
            rl_mark = rl_point;
        }
    }
    return 0;
}

fn bash_backward_kill_shellword(count: libc::c_int, key: libc::c_int) -> libc::c_int {
    let p: libc::c_int;

    if count < 0 as libc::c_int {
        return bash_kill_shellword(-count, key);
    }
    unsafe {
        p = rl_point;
        bash_backward_shellword(count, key);

        if rl_point != p {
            c_rl_kill_text(p, rl_point);
        }
        if rl_editing_mode == 1 as libc::c_int {
            rl_mark = rl_point;
        }
    }
    return 0 as libc::c_int;
}

fn bash_transpose_shellwords(count: libc::c_int, key: libc::c_int) -> libc::c_int {
    let word1: *mut libc::c_char;
    let word2: *mut libc::c_char;
    let w1_beg: libc::c_int;
    let w1_end: libc::c_int;
    let w2_beg: libc::c_int;
    let w2_end: libc::c_int;
    unsafe {
        let orig_point: libc::c_int = rl_point;

        if count == 0 as libc::c_int {
            return 0 as libc::c_int;
        }

        bash_forward_shellword(count, key);
        w2_end = rl_point;
        bash_backward_shellword(1, key);
        w2_beg = rl_point;
        bash_backward_shellword(count, key);
        w1_beg = rl_point;
        bash_forward_shellword(1, key);
        w1_end = rl_point;

        if w1_beg == w2_beg || w2_beg < w1_end {
            c_rl_ding();
            rl_point = orig_point;
            return 1;
        }

        word1 = c_rl_copy_text(w1_beg, w1_end);
        word2 = c_rl_copy_text(w2_beg, w2_end);

        c_rl_begin_undo_group();

        rl_point = w2_beg;
        c_rl_delete_text(w2_beg, w2_end);
        c_rl_insert_text(word1);

        rl_point = w1_beg;
        c_rl_delete_text(w1_beg, w1_end);
        c_rl_insert_text(word2);

        rl_point = w2_end;

        c_rl_end_undo_group();
        libc::free(word1 as *mut c_void);
        libc::free(word2 as *mut c_void);
    }
    return 0;
}

fn check_redir(ti: libc::c_int) -> libc::c_int {
    let this_char: libc::c_int;
    let prev_char: libc::c_int;

    this_char = unsafe { *rl_line_buffer.offset(ti as isize) as libc::c_int };
    prev_char = if ti > 0 as libc::c_int {
        unsafe { *rl_line_buffer.offset((ti - 1 as libc::c_int) as isize) as libc::c_int }
    } else {
        0 as libc::c_int
    };

    if this_char == '&' as i32 && (prev_char == '<' as i32 || prev_char == '>' as i32)
        || this_char == '|' as i32 && prev_char == '>' as i32
    {
        return 1 as libc::c_int;
    } else {
        if this_char == '{' as i32 && prev_char == '$' as i32 {
            return 1 as libc::c_int;
        } else {
            if unsafe { char_is_quoted(rl_line_buffer, ti) != 0 } {
                return 1 as libc::c_int;
            }
        }
    }

    return 0 as libc::c_int;
}

fn find_cmd_start(start: libc::c_int) -> libc::c_int {
    let mut s: libc::c_int;
    let mut os: libc::c_int;
    let mut ns: libc::c_int;
    os = 0 as libc::c_int;
    unsafe {
        loop {
            s = skip_to_delim(
                rl_line_buffer,
                os,
                b";|&{(`\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0x1 as libc::c_int | 0x100 as libc::c_int,
            );
            if !(s <= start && *rl_line_buffer.offset(s as isize) as libc::c_int != 0) {
                break;
            }
            if s > 0 as libc::c_int
                && *rl_line_buffer.offset(s as isize) as libc::c_int == '|' as i32
                && *rl_line_buffer.offset((s - 1 as libc::c_int) as isize) as libc::c_int
                    == '>' as i32
            {
                ns = skip_to_delim(
                    rl_line_buffer,
                    s + 1 as libc::c_int,
                    b";|&{(`\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    0x1 as libc::c_int | 0x100 as libc::c_int,
                );
                if ns > start
                    || *rl_line_buffer.offset(ns as isize) as libc::c_int == 0 as libc::c_int
                {
                    return os;
                }
                os = ns + 1 as libc::c_int;
            } else {
                if s >= os && *rl_line_buffer.offset(s as isize) as libc::c_int == '{' as i32 {
                    let mut pc: libc::c_int;
                    let nc: libc::c_int;
                    pc = if s > os { s - 1 as libc::c_int } else { os };
                    while pc > os
                        && (*rl_line_buffer.offset(pc as isize) as libc::c_int == ' ' as i32
                            || *rl_line_buffer.offset(pc as isize) as libc::c_int == '\t' as i32)
                    {
                        pc -= 1;
                    }
                    nc = *rl_line_buffer.offset((s + 1 as libc::c_int) as isize) as libc::c_int;
                    if pc > os
                        && (*rl_line_buffer.offset((s - 1 as libc::c_int) as isize) as libc::c_int
                            == '{' as i32
                            || (libc::strchr(
                                b";|&{(`\0" as *const u8 as *const libc::c_char,
                                *rl_line_buffer.offset(pc as isize) as libc::c_int,
                            ))
                            .is_null())
                        || *sh_syntaxtab
                            .as_mut_ptr()
                            .offset(nc as libc::c_uchar as isize)
                            & 0x2 as libc::c_int
                            == 0 as libc::c_int
                    {
                        ns = skip_to_delim(
                            rl_line_buffer,
                            s + 1 as libc::c_int,
                            b";|&{(`\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            0x1 as libc::c_int | 0x100 as libc::c_int,
                        );
                        if ns > start
                            || *rl_line_buffer.offset(ns as isize) as libc::c_int
                                == 0 as libc::c_int
                        {
                            return os;
                        }
                        os = ns + 1 as libc::c_int;
                        continue;
                    }
                }
                os = s + 1 as libc::c_int;
            }
        }
    }
    return os;
}

fn find_cmd_end(end: libc::c_int) -> libc::c_int {
    let e: libc::c_int;

    e = unsafe {
        skip_to_delim(
            rl_line_buffer,
            end,
            COMMAND_SEPARATORS!(),
            SD_NOJMP as libc::c_int | SD_COMPLETE as libc::c_int,
        )
    };
    return e;
}
fn find_cmd_name(
    start: libc::c_int,
    sp: *mut libc::c_int,
    ep: *mut libc::c_int,
) -> *mut libc::c_char {
    let name: *mut libc::c_char;
    let mut s: libc::c_int;
    let e: libc::c_int;
    unsafe {
        s = start;
        while whitespace!(*rl_line_buffer.offset(s as isize)) {
            s += 1;
        }
        e = skip_to_delim(
            rl_line_buffer,
            s,
            b"()<>;&| \t\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0x1 as libc::c_int | 0x100 as libc::c_int,
        );
        name = substring(rl_line_buffer, s, e);
        if !sp.is_null() {
            *sp = s;
        }
        if !ep.is_null() {
            *ep = e;
        }
    }
    return name;
}

static mut prog_complete_matches: *mut *mut libc::c_char =
    0 as *const *mut libc::c_char as *mut *mut libc::c_char;

fn prog_complete_return(_text: *const libc::c_char, matchnum: libc::c_int) -> *mut libc::c_char {
    static mut ind: libc::c_int = 0;
    unsafe {
        if matchnum == 0 as libc::c_int {
            ind = 0 as libc::c_int;
        }

        if prog_complete_matches.is_null()
            || (*prog_complete_matches.offset(ind as isize)).is_null()
        {
            return 0 as *mut c_void as *mut libc::c_char;
        }
        let fresh10 = ind;
        ind = ind + 1;
        return *prog_complete_matches.offset(fresh10 as isize);
    }
}

fn invalid_completion(_text: *const libc::c_char, ind: libc::c_int) -> libc::c_int {
    let mut pind: libc::c_int;
    unsafe {
        if ind > 0 as libc::c_int
            && *rl_line_buffer.offset(ind as isize) as libc::c_int == '(' as i32
            && member!(
                rl_line_buffer.offset((ind - 1 as libc::c_int) as isize),
                b"$<>\0" as *const u8 as *const libc::c_char
            )
        {
            return 0 as libc::c_int;
        }
        pind = ind - 1 as libc::c_int;
        while pind > 0 as libc::c_int && whitespace!(*rl_line_buffer.offset(pind as isize)) {
            pind -= 1;
        }
        if ind >= 0
            && pind <= 0
            && *rl_line_buffer.offset(ind as isize) as libc::c_int == '(' as i32
        {
            return 0;
        }
        if ind > 0
            && *rl_line_buffer.offset(ind as isize) as libc::c_int == '(' as i32
            && member!(rl_line_buffer.offset(pind as isize), COMMAND_SEPARATORS!())
        {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}

fn attempt_shell_completion(
    text: *const libc::c_char,
    start: libc::c_int,
    end: libc::c_int,
) -> *mut *mut libc::c_char {
    let mut in_command_position: libc::c_int;
    let mut ti: libc::c_int;
    let mut qc: libc::c_int;
    let mut dflags: libc::c_int;
    let mut matches: *mut *mut libc::c_char;
    let command_separator_chars: *mut libc::c_char;
    let have_progcomps: libc::c_int;
    let mut was_assignment: libc::c_int;
    let iw_compspec: *mut COMPSPEC;
    unsafe {
        // 内存安全检查：验证输入参数
        // 基本安全检查
        if text.is_null() || rl_line_buffer.is_null() {
            return 0 as *mut *mut libc::c_char;
        }

        // 验证start和end范围
        if start < 0 || end < start || end > rl_end {
            return 0 as *mut *mut libc::c_char;
        }

        command_separator_chars = COMMAND_SEPARATORS!();
        matches = 0 as *mut *mut libc::c_char;

        rl_ignore_some_completions_function = Some(filename_completion_ignore);

        rl_filename_quote_characters = default_filename_quote_characters;
        set_filename_bstab(rl_filename_quote_characters);

        set_directory_hook();
        rl_filename_stat_hook = Some(bash_filename_stat_hook);
        rl_sort_completion_matches = 1;

        ti = start - 1 as libc::c_int;
        qc = -(1 as libc::c_int);

        while ti > -(1 as libc::c_int) && whitespace!(rl_line_buffer.offset(ti as isize)) {
            ti -= 1;
        }

        if ti >= 0
            && (*rl_line_buffer.offset(ti as isize) as libc::c_int == '"' as i32
                || *rl_line_buffer.offset(ti as isize) as libc::c_int == '\'' as i32)
        {
            qc = *rl_line_buffer.offset(ti as isize) as libc::c_int;
            ti -= 1;
            while ti > -(1 as libc::c_int) && whitespace!(rl_line_buffer.offset(ti as isize)) {
                ti -= 1;
            }
        }

        in_command_position = 0;
        if ti < 0 {
            if current_prompt_string == ps1_prompt {
                in_command_position += 1;
            } else if parser_in_command_position() != 0 {
                in_command_position += 1;
            }
        } else if member!(rl_line_buffer.offset(ti as isize), command_separator_chars) {
            in_command_position += 1;
            if check_redir(ti) == 1 as libc::c_int {
                in_command_position = 0 as libc::c_int;
            }
        }

        if in_command_position != 0 && invalid_completion(text, ti) != 0 {
            rl_attempted_completion_over = 1;
            return 0 as *mut *mut libc::c_char;
        }

        if in_command_position != 0
            && ti >= 0
            && *rl_line_buffer.offset(ti as isize) as libc::c_int == '`' as i32
            && *text as libc::c_int != '`' as i32
            && unclosed_pair(
                rl_line_buffer,
                end,
                b"`\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0 as libc::c_int
        {
            in_command_position = 0;
        }
        if *text as libc::c_int == '`' as i32
            && rl_completion_quote_character != '\'' as i32
            && (in_command_position != 0
                || unclosed_pair(
                    rl_line_buffer,
                    start,
                    b"`\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) != 0
                    && unclosed_pair(
                        rl_line_buffer,
                        end,
                        b"`\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ) != 0)
        {
            matches = c_rl_completion_matches(text, Some(command_subst_completion_function));
        }
        have_progcomps =
            (prog_completion_enabled != 0 && progcomp_size() > 0 as libc::c_int) as libc::c_int;
        iw_compspec = progcomp_search(INITIALWORD!());
        if matches.is_null()
            && (in_command_position == 0
                || *text.offset(0 as isize) as libc::c_int == '\u{0}' as i32
                || in_command_position != 0 && !iw_compspec.is_null())
            && current_prompt_string == ps1_prompt
        {
            let mut s: libc::c_int;
            let e: libc::c_int;
            let mut s1: libc::c_int = 0;
            let mut e1: libc::c_int = 0;
            let os: libc::c_int;
            let mut foundcs: libc::c_int = 0;
            let mut n: *mut libc::c_char;

            if !prog_complete_matches.is_null() {
                libc::free(prog_complete_matches as *mut c_void);
            }
            prog_complete_matches = 0 as *mut *mut libc::c_char;

            os = start;
            n = 0 as *mut libc::c_char;
            was_assignment = 0 as libc::c_int;
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
                s = e1 + 1;
                was_assignment = assignment(n, 0 as libc::c_int);
                if !(was_assignment != 0) {
                    break;
                }
            }

            s = s1;
            if start == 0
                && end == 0
                && e != 0
                && *text.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
            {
                foundcs = 0;
            } else if start == end && start == s1 && e != 0 && e1 > end {
                foundcs = 0;
            } else if e == 0
                && e == s
                && *text.offset(0 as isize) as libc::c_int == '\u{0}' as i32
                && have_progcomps != 0
            {
                prog_complete_matches =
                    programmable_completions(EMPTYCMD!(), text, s, e, &mut foundcs);
            } else if start == end
                && *text.offset(0 as isize) as libc::c_int == '\u{0}' as i32
                && s1 > start
                && whitespace!(rl_line_buffer.offset(start as isize))
            {
                foundcs = 0;
            } else if e > s
                && was_assignment == 0 as libc::c_int
                && e1 == end
                && *rl_line_buffer.offset(e as isize) as libc::c_int == 0 as libc::c_int
                && whitespace!(rl_line_buffer.offset((e - 1 as libc::c_int) as isize))
            {
                foundcs = 0;
                in_command_position = (s == start && STREQ!(n, text)) as i32;
            } else if e > s && was_assignment == 0 && have_progcomps != 0 {
                prog_complete_matches = programmable_completions(n, text, s, e, &mut foundcs);

                in_command_position =
                    (s == start && (!iw_compspec.is_null() || STREQ!(n, text))) as libc::c_int;
                if !iw_compspec.is_null() && in_command_position != 0 {
                    foundcs = 0;
                }
            } else if s >= e
                && *n.offset(0 as isize) as libc::c_int == '\u{0}' as i32
                && *text.offset(0 as isize) as libc::c_int == '\u{0}' as i32
                && start > 0
                && was_assignment == 0
                && member!(
                    rl_line_buffer.offset((start - 1 as libc::c_int).try_into().unwrap()),
                    COMMAND_SEPARATORS!()
                )
            {
                foundcs = 0 as libc::c_int;
                in_command_position = 1 as libc::c_int;
            } else if s >= e
                && *n.offset(0 as isize) as libc::c_int == '\u{0}' as i32
                && *text.offset(0 as isize) as libc::c_int == '\u{0}' as i32
                && start > 0 as libc::c_int
            {
                foundcs = 0;
                in_command_position += was_assignment;
            } else if s == start && e == end && STREQ!(n, text) && start > 0 as libc::c_int {
                foundcs = 0;
                in_command_position = 1;
            } else {
                foundcs = 0;
            }
            if in_command_position != 0
                && have_progcomps != 0
                && foundcs == 0 as libc::c_int
                && !iw_compspec.is_null()
            {
                prog_complete_matches =
                    programmable_completions(INITIALWORD!(), text, s, e, &mut foundcs);
            }

            FREE!(n);

            if foundcs != 0 {
                pcomp_set_readline_variables(foundcs, 1 as libc::c_int);
                matches = c_rl_completion_matches(text, Some(prog_complete_return));
                if foundcs & COPT_DEFAULT as libc::c_int == 0 {
                    rl_attempted_completion_over = 1;
                }
                if !matches.is_null() || foundcs & COPT_BASHDEFAULT as libc::c_int == 0 {
                    return matches;
                }
            }
        }
        if matches.is_null() {
            dflags = 0;
            if in_command_position != 0 {
                dflags |= DEFCOMP_CMDPOS!();
            }
            matches = bash_default_completion(text, start, end, qc, dflags);
        }
    }
    return matches;
}

#[no_mangle]
pub fn bash_default_completion(
    text: *const libc::c_char,
    start: libc::c_int,
    end: libc::c_int,
    qc: libc::c_int,
    compflags: libc::c_int,
) -> *mut *mut libc::c_char {
    let mut matches: *mut *mut libc::c_char;
    let mut t: *mut libc::c_char;

    matches = 0 as *mut c_void as *mut *mut libc::c_char;
    unsafe {
        if *text as libc::c_int == '$' as i32 {
            if qc != '\'' as i32 && *text.offset(1 as isize) as libc::c_int == '(' as i32 {
                matches = c_rl_completion_matches(text, Some(command_subst_completion_function));
            } else {
                matches = c_rl_completion_matches(text, Some(variable_completion_function));
                if !matches.is_null()
                    && !(*matches.offset(0 as isize)).is_null()
                    && (*matches.offset(1 as isize)).is_null()
                {
                    t = savestring!(*matches.offset(0 as libc::c_int as isize));
                    bash_filename_stat_hook(&mut t);
                    if file_isdir(t) != 0 {
                        rl_completion_append_character = '/' as i32;
                    }
                    libc::free(t as *mut c_void);
                }
            }
        }
        if matches.is_null()
            && *text as libc::c_int == '~' as i32
            && (c_mbschr(text, '/' as i32)).is_null()
        {
            c_rl_completion_matches(text, Some(c_rl_username_completion_function));
        }

        if matches.is_null()
            && perform_hostname_completion != 0
            && *text as libc::c_int == '@' as i32
        {
            matches = c_rl_completion_matches(
                text,
                Some(
                    hostname_completion_function
                        as fn(*const libc::c_char, libc::c_int) -> *mut libc::c_char,
                ),
            );
        }

        if matches.is_null() && compflags & DEFCOMP_CMDPOS!() != 0 {
            if no_empty_command_completion != 0
                && end == start
                && *text.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
            {
                matches = 0 as *mut *mut libc::c_char;
                rl_ignore_some_completions_function = Some(bash_ignore_everything);
            } else {
                dot_in_path = 0;
                matches = c_rl_completion_matches(text, Some(command_word_completion_function));
                if matches.is_null() {
                    rl_ignore_some_completions_function = Some(bash_ignore_filenames);
                } else if (*matches.offset(1 as libc::c_int as isize)).is_null()
                    && CMD_IS_DIR!(*matches.offset(0 as isize))
                    && dot_in_path == 0
                {
                    rl_completion_suppress_append = 1;
                    rl_filename_completion_desired = 0;
                } else if !(*matches.offset(0 as isize)).is_null()
                    && !(*matches.offset(1 as isize)).is_null()
                    && STREQ!(*matches.offset(0 as isize), *matches.offset(1 as isize))
                    && CMD_IS_DIR!(*matches.offset(0 as isize))
                {
                    rl_completion_suppress_append = 1 as libc::c_int;
                    rl_filename_completion_desired = 0 as libc::c_int;
                }
            }
        }
        if matches.is_null() && completion_glob_pattern(text as *mut libc::c_char) != 0 {
            matches = c_rl_completion_matches(text, Some(glob_complete_word));

            if !matches.is_null()
                && !(*matches.offset(1 as isize)).is_null()
                && rl_completion_type == '\t' as i32
            {
                c_strvec_dispose(matches);
                matches = 0 as *mut *mut libc::c_char;
            } else if !matches.is_null()
                && !(*matches.offset(1 as libc::c_int as isize)).is_null()
                && rl_completion_type == '!' as i32
            {
                rl_completion_suppress_append = 1 as libc::c_int;
                rl_filename_completion_desired = 0 as libc::c_int;
            }
        }
    }
    return matches;
}

fn bash_command_name_stat_hook(name: *mut *mut libc::c_char) -> libc::c_int {
    let cname: *mut libc::c_char;
    let result: *mut libc::c_char;
    unsafe {
        if absolute_program(*name) != 0 {
            return bash_filename_stat_hook(name);
        }
        cname = *name;
        result = search_for_command(cname, 0 as libc::c_int);
        if !result.is_null() {
            *name = result;
            return 1;
        }
    }
    return 0;
}

fn executable_completion(
    filename: *const libc::c_char,
    searching_path: libc::c_int,
) -> libc::c_int {
    let mut f: *mut libc::c_char;
    let r: libc::c_int;

    f = unsafe { savestring!(filename) };

    bash_directory_completion_hook(&mut f);

    r = if searching_path != 0 {
        executable_file(f)
    } else {
        executable_or_directory(f)
    };
    unsafe {
        libc::free(f as *mut c_void);
    }

    return r;
}

#[no_mangle]
pub fn command_word_completion_function(
    hint_text: *const libc::c_char,
    state: libc::c_int,
) -> *mut libc::c_char {
    let mut match_0: libc::c_int;
    let mut freetemp: libc::c_int;
    let mut current_block: u64;
    static mut hint: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut filename_hint: *mut libc::c_char = 0 as *mut c_void as *mut libc::c_char;
    static mut fnhint: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut dequoted_hint: *mut libc::c_char = 0 as *mut c_void as *mut libc::c_char;
    static mut directory_part: *mut libc::c_char = 0 as *mut c_void as *mut libc::c_char;
    static mut glob_matches: *mut *mut libc::c_char = 0 as *mut c_void as *mut *mut libc::c_char;
    static mut path_index: libc::c_int = 0;
    static mut hint_len: libc::c_int = 0;
    static mut istate: libc::c_int = 0;
    static mut igncase: libc::c_int = 0;
    static mut mapping_over: libc::c_int = 0;
    static mut local_index: libc::c_int = 0;
    static mut searching_path: libc::c_int = 0;
    static mut hint_is_dir: libc::c_int = 0;
    static mut old_glob_ignore_case: libc::c_int = 0;
    static mut globpat: libc::c_int = 0;
    static mut varlist: *mut *mut SHELL_VAR = 0 as *mut *mut SHELL_VAR;
    static mut alias_list: *mut *mut alias_t = 0 as *mut *mut alias_t;
    let mut temp: *mut libc::c_char;
    let mut cval: *mut libc::c_char;
    unsafe {
        if state == 0 {
            rl_filename_stat_hook = Some(bash_command_name_stat_hook);

            if !dequoted_hint.is_null() && dequoted_hint != hint {
                libc::free(dequoted_hint as *mut c_void);
            }
            if !hint.is_null() {
                libc::free(hint as *mut c_void);
            }

            searching_path = 0;
            mapping_over = searching_path;
            hint_is_dir = CMD_IS_DIR!(hint_text) as i32;
            val = 0 as *mut libc::c_char;

            temp = c_rl_variable_value(
                b"completion-ignore-case\0" as *const u8 as *const libc::c_char,
            );
            igncase = RL_BOOLEAN_VARIABLE_VALUE!(temp) as libc::c_int;

            if !glob_matches.is_null() {
                libc::free(glob_matches as *mut c_void);
                glob_matches = 0 as *mut *mut libc::c_char;
            }

            globpat = completion_glob_pattern(hint_text as *mut libc::c_char);

            if globpat != 0 || absolute_program(hint_text) != 0 {
                if *hint_text as libc::c_int == '~' as i32 {
                    hint = bash_tilde_expand(hint_text, 0);
                    directory_part = savestring!(hint_text);
                    temp = libc::strchr(directory_part, '/' as i32);

                    if !temp.is_null() {
                        *temp = 0 as libc::c_char;
                    } else {
                        libc::free(directory_part as *mut c_void);
                        directory_part = 0 as *mut libc::c_char;
                    }
                } else if dircomplete_expand != 0 {
                    hint = savestring!(hint_text);
                    bash_directory_completion_hook(&mut hint);
                } else {
                    hint = savestring!(hint_text);
                }

                dequoted_hint = hint;

                if rl_completion_found_quote != 0 && rl_completion_quote_character == 0 {
                    dequoted_hint = bash_dequote_filename(hint, 0);
                    libc::free(hint as *mut c_void);
                    hint = dequoted_hint;
                }
                hint_len = libc::strlen(hint) as libc::c_int;

                if !filename_hint.is_null() {
                    libc::free(filename_hint as *mut c_void);
                }

                filename_hint = savestring!(hint);
                fnhint = filename_hint;

                istate = 0;

                if globpat != 0 {
                    mapping_over = 5;
                    current_block = 6770491532623035343; //globword
                } else {
                    if dircomplete_expand != 0 && path_dot_or_dotdot(filename_hint) != 0 {
                        dircomplete_expand = 0 as libc::c_int;
                        set_directory_hook();
                        dircomplete_expand = 1 as libc::c_int;
                    }
                    mapping_over = 4 as libc::c_int;
                    current_block = 14702977349412626834; //inner
                }
            } else {
                hint = savestring!(hint_text);
                dequoted_hint = hint;
                hint_len = libc::strlen(hint) as libc::c_int;

                if rl_completion_found_quote != 0
                    && rl_completion_quote_character == 0 as libc::c_int
                {
                    dequoted_hint = bash_dequote_filename(hint, 0 as libc::c_int);
                }

                path = get_string_value(b"PATH\0" as *const u8 as *const libc::c_char);
                dot_in_path = 0 as libc::c_int;
                path_index = dot_in_path;
                local_index = 0 as libc::c_int;

                if !varlist.is_null() {
                    libc::free(varlist as *mut c_void);
                }

                varlist = all_visible_functions();
                if !alias_list.is_null() {
                    libc::free(alias_list as *mut c_void);
                }

                alias_list = all_aliases();
                current_block = 1874315696050160458; //Normal
            }
        } else {
            current_block = 1874315696050160458; //Normal
        }

        match current_block {
            1874315696050160458 => {
                //Normal
                let mut current_block_92: u64;
                match mapping_over {
                    0 => {
                        while !alias_list.is_null()
                            && !(*alias_list.offset(local_index as isize)).is_null()
                        {
                            let alias: *mut libc::c_char;

                            alias = (**alias_list.offset(local_index as isize)).name;
                            local_index = local_index + 1;

                            if igncase == 0 && STREQN!(alias, hint, hint_len) != 0 {
                                return savestring!(alias);
                            } else {
                                if igncase != 0
                                    && strncasecmp(alias, hint, hint_len as usize)
                                        == 0 as libc::c_int
                                {
                                    return savestring!(alias);
                                }
                            }
                        }
                        local_index = 0;
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
                        while !((*word_token_alist.as_mut_ptr().offset(local_index as isize)).word)
                            .is_null()
                        {
                            let reserved_word: *mut libc::c_char;

                            reserved_word =
                                (*word_token_alist.as_mut_ptr().offset(local_index as isize)).word;
                            local_index = local_index + 1;

                            if STREQN!(reserved_word, hint, hint_len) != 0 {
                                return savestring!(reserved_word);
                            }
                        }
                        local_index = 0 as libc::c_int;
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
                            let varname: *mut libc::c_char;

                            varname = (**varlist.offset(local_index as isize)).name;
                            local_index = local_index + 1;

                            if igncase == 0 && STREQN!(varname, hint, hint_len) != 0 {
                                return savestring!(varname);
                            } else {
                                if igncase != 0
                                    && strncasecmp(varname, hint, hint_len as usize) == 0
                                {
                                    return savestring!(varname);
                                }
                            }
                        }
                        local_index = 0 as libc::c_int;
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
                                    & BUILTIN_ENABLED as libc::c_int
                                    == 0 as libc::c_int)
                            {
                                if STREQN!(
                                    (*shell_builtins.offset(local_index as isize)).name,
                                    hint,
                                    hint_len
                                ) != 0
                                {
                                    let i: libc::c_int = local_index;
                                    local_index = local_index + 1;

                                    return savestring!((*shell_builtins.offset(i as isize)).name);
                                }
                            }

                            local_index += 1;
                        }
                        local_index = 0 as libc::c_int;
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
                    if state == 0 as libc::c_int {
                        glob_ignore_case = igncase;
                        glob_matches = shell_glob_filename(hint, 0 as libc::c_int);
                        glob_ignore_case = old_glob_ignore_case;

                        if glob_matches == &mut glob_error_return as *mut *mut libc::c_char
                            || glob_matches.is_null()
                        {
                            glob_matches = 0 as *mut *mut libc::c_char;
                            return 0 as *mut libc::c_char;
                        }

                        local_index = 0;

                        if !(*glob_matches.offset(1 as libc::c_int as isize)).is_null()
                            && rl_completion_type == TAB!()
                        {
                            return 0 as *mut libc::c_char;
                        }
                    }

                    loop {
                        val = *glob_matches.offset(local_index as isize);
                        local_index = local_index + 1;
                        if val.is_null() {
                            break;
                        }
                        if executable_or_directory(val) != 0 {
                            if *hint_text as libc::c_int == '~' as i32 && !directory_part.is_null()
                            {
                                temp = maybe_restore_tilde(val, directory_part);
                                libc::free(val as *mut c_void);
                                val = temp;
                            }
                            return val;
                        }
                        libc::free(val as *mut c_void);
                    }
                    glob_ignore_case = old_glob_ignore_case;
                    return 0 as *mut libc::c_char;
                }

                if hint_is_dir != 0 {
                    hint_is_dir = 0;
                    return savestring!(hint_text);
                }
                current_block = 14531478163722833811;
            }
            _ => {}
        }

        loop {
            match current_block {
                14702977349412626834 => {
                    val = c_rl_filename_completion_function(fnhint, istate);
                    if mapping_over == 4 as libc::c_int && dircomplete_expand != 0 {
                        set_directory_hook();
                    }

                    istate = 1;

                    if val.is_null() {
                        if absolute_program(hint) != 0 {
                            return 0 as *mut libc::c_char;
                        }
                        current_block = 14531478163722833811;
                    } else {
                        // match_0 = 0;
                        // freetemp = 0;
                        if absolute_program(hint) != 0 {
                            if igncase == 0 {
                                match_0 = (libc::strncmp(val, hint, hint_len as usize) == 0)
                                    as libc::c_int;
                            } else {
                                match_0 =
                                    (strncasecmp(val, hint, hint_len as usize) == 0) as libc::c_int;
                            }
                            if *hint_text as libc::c_int == '~' as i32 {
                                temp = maybe_restore_tilde(val, directory_part);
                            } else {
                                temp = savestring!(val);
                            }
                            freetemp = 1;
                        } else {
                            temp = strrchr(val, '/' as i32);
                            if !temp.is_null() {
                                temp = temp.offset(1);
                                if igncase == 0 {
                                    match_0 = (libc::strncmp(temp, hint, hint_len as usize) == 0)
                                        as libc::c_int;
                                    freetemp = match_0;
                                } else {
                                    match_0 = (strncasecmp(temp, hint, hint_len as usize)
                                        == 0 as libc::c_int)
                                        as libc::c_int;
                                    freetemp = match_0;
                                }
                                if match_0 != 0 {
                                    temp = savestring!(temp);
                                }
                            } else {
                                match_0 = 0 as libc::c_int;
                                freetemp = match_0;
                            }
                        }
                        cval = val;
                        if match_0 != 0
                            && executable_completion(
                                if searching_path != 0 { val } else { cval },
                                searching_path,
                            ) != 0
                        {
                            break;
                        }
                        if freetemp != 0 {
                            libc::free(temp as *mut c_void);
                        }
                        if cval != val {
                            libc::free(cval as *mut c_void);
                        }
                        libc::free(val as *mut c_void);
                        current_block = 14702977349412626834;
                    }
                }
                _ => {
                    istate = (val != 0 as *mut libc::c_char) as libc::c_int;

                    if istate == 0 {
                        let mut current_path: *mut libc::c_char = 0 as *mut libc::c_char;

                        if path.is_null()
                            || *path.offset(path_index as isize) as libc::c_int == 0
                            || {
                                current_path = extract_colon_unit(path, &mut path_index);
                                current_path.is_null()
                            }
                        {
                            return 0 as *mut c_void as *mut libc::c_char;
                        }
                        searching_path = 1;
                        if *current_path as libc::c_int == 0 as libc::c_int {
                            libc::free(current_path as *mut c_void);
                            current_path = savestring!(b".\0" as *const u8 as *const libc::c_char);
                        }
                        if *current_path as libc::c_int == '~' as i32 {
                            let t: *mut libc::c_char;

                            t = bash_tilde_expand(current_path, 0 as libc::c_int);
                            libc::free(current_path as *mut c_void);
                            current_path = t;
                        }

                        if *current_path.offset(0 as isize) as libc::c_int == '.' as i32
                            && *current_path.offset(1 as isize) as libc::c_int == '\u{0}' as i32
                        {
                            dot_in_path = 1;
                        }
                        if !fnhint.is_null() && fnhint != filename_hint {
                            libc::free(fnhint as *mut c_void);
                        }
                        if !filename_hint.is_null() {
                            libc::free(filename_hint as *mut c_void);
                        }
                        filename_hint = c_sh_makepath(current_path, hint, 0);
                        if !(strpbrk(
                            filename_hint,
                            b"\"'\\\0" as *const u8 as *const libc::c_char,
                        ))
                        .is_null()
                        {
                            fnhint = c_sh_backslash_quote(
                                filename_hint,
                                filename_bstab.as_mut_ptr(),
                                0 as libc::c_int,
                            );
                        } else {
                            fnhint = filename_hint;
                        }
                        libc::free(current_path as *mut c_void);
                    }
                    current_block = 14702977349412626834;
                }
            }
        }
        if cval != val {
            libc::free(cval as *mut c_void);
        }
        libc::free(val as *mut c_void);
        val = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    return temp;
}

fn command_subst_completion_function(
    mut text: *const libc::c_char,
    state: libc::c_int,
) -> *mut libc::c_char {
    static mut matches: *mut *mut libc::c_char =
        0 as *const c_void as *mut c_void as *mut *mut libc::c_char;
    static mut orig_start: *const libc::c_char = 0 as *const libc::c_char;
    static mut filename_text: *mut libc::c_char =
        0 as *const c_void as *mut c_void as *mut libc::c_char;
    static mut cmd_index: libc::c_int = 0;
    static mut start_len: libc::c_int = 0;
    unsafe {
        let mut value: *mut libc::c_char;

        if state == 0 as libc::c_int {
            if !filename_text.is_null() {
                libc::free(filename_text as *mut c_void);
            }
            orig_start = text;
            if *text as libc::c_int == '`' as i32 {
                text = text.offset(1);
            } else if *text as libc::c_int == '$' as i32
                && *text.offset(1 as libc::c_int as isize) as libc::c_int == '(' as i32
            {
                text = text.offset(2 as libc::c_int as isize);
            }

            rl_completion_suppress_quote = 1 as libc::c_int;
            start_len = text.offset_from(orig_start) as libc::c_long as libc::c_int;
            filename_text = savestring!(text);
            if !matches.is_null() {
                libc::free(matches as *mut c_void);
            }

            value = filename_text
                .offset(libc::strlen(filename_text) as isize)
                .offset(-(1 as libc::c_int as isize));
            while value > filename_text {
                if whitespace!(*value) || member!(value, COMMAND_SEPARATORS!()) {
                    break;
                }
                value = value.offset(-1);
            }

            if value <= filename_text {
                matches =
                    c_rl_completion_matches(filename_text, Some(command_word_completion_function));
            } else {
                value = value.offset(1);
                start_len = (start_len as libc::c_long
                    + value.offset_from(filename_text) as libc::c_long)
                    as libc::c_int;
                if whitespace!(value.offset(-(1 as libc::c_int) as isize)) {
                    matches =
                        c_rl_completion_matches(value, Some(c_rl_filename_completion_function));
                } else {
                    matches =
                        c_rl_completion_matches(value, Some(command_word_completion_function));
                }
            }

            cmd_index = (!matches.is_null()
                && !(*matches.offset(0 as isize)).is_null()
                && !(*matches.offset(1 as isize)).is_null()) as libc::c_int;

            if !matches.is_null()
                && !(*matches.offset(0 as isize)).is_null()
                && (*matches.offset(1 as libc::c_int as isize)).is_null()
                && test_for_directory(*matches.offset(0 as isize)) != 0
            {
                rl_completion_append_character = '/' as i32;
            } else {
                rl_completion_suppress_append = 1 as libc::c_int;
            }
        }

        if matches.is_null() || (*matches.offset(cmd_index as isize)).is_null() {
            rl_filename_quoting_desired = 0;
            return 0 as *mut libc::c_char;
        } else {
            value = libc::malloc(
                (1 + start_len + strlen(*matches.offset(cmd_index as isize)) as i32) as usize,
            ) as *mut libc::c_char;
            if start_len == 1 {
                *value.offset(0 as isize) = *orig_start;
            } else {
                libc::strncpy(
                    value,
                    orig_start,
                    (start_len as libc::c_ulong).try_into().unwrap(),
                );
            }

            libc::strcpy(
                value.offset(start_len as isize),
                *matches.offset(cmd_index as isize),
            );

            cmd_index += 1;
            return value;
        };
    }
}

fn variable_completion_function(
    text: *const libc::c_char,
    state: libc::c_int,
) -> *mut libc::c_char {
    static mut varlist: *mut *mut libc::c_char =
        0 as *const c_void as *mut c_void as *mut *mut libc::c_char;
    static mut varlist_index: libc::c_int = 0;
    static mut varname: *mut libc::c_char = 0 as *const c_void as *mut c_void as *mut libc::c_char;
    static mut first_char: libc::c_int = 0;
    static mut first_char_loc: libc::c_int = 0;
    unsafe {
        if state == 0 {
            if !varname.is_null() {
                libc::free(varname as *mut c_void);
            }

            first_char_loc = 0 as libc::c_int;
            first_char = *text.offset(0 as isize) as libc::c_int;

            if first_char == '$' as i32 {
                first_char_loc += 1;
            }

            if *text.offset(first_char_loc as isize) as libc::c_int == '{' as i32 {
                first_char_loc += 1;
            }
            varname = savestring!(text.offset(first_char_loc as isize));

            if !varlist.is_null() {
                c_strvec_dispose(varlist);
            }

            varlist = all_variables_matching_prefix(varname);
            varlist_index = 0;
        }

        if varlist.is_null() || (*varlist.offset(varlist_index as isize)).is_null() {
            return 0 as *mut libc::c_char;
        } else {
            let value: *mut libc::c_char;
            value = libc::malloc(
                (4 + libc::strlen(*varlist.offset(varlist_index as isize)))
                    .try_into()
                    .unwrap(),
            ) as *mut libc::c_char;

            if first_char_loc != 0 {
                *value.offset(0 as isize) = first_char as libc::c_char;
                if first_char_loc == 2 {
                    *value.offset(1 as isize) = '{' as i32 as libc::c_char;
                }
            }

            libc::strcpy(
                value.offset(first_char_loc as isize),
                *varlist.offset(varlist_index as isize),
            );
            if first_char_loc == 2 as libc::c_int {
                strcat(value, b"}\0" as *const u8 as *const libc::c_char);
            }

            varlist_index += 1;
            return value;
        };
    }
}

fn hostname_completion_function(
    text: *const libc::c_char,
    state: libc::c_int,
) -> *mut libc::c_char {
    static mut list: *mut *mut libc::c_char =
        0 as *const c_void as *mut c_void as *mut *mut libc::c_char;
    static mut list_index: libc::c_int = 0 as libc::c_int;
    static mut first_char: libc::c_int = 0;
    static mut first_char_loc: libc::c_int = 0;
    unsafe {
        if state == 0 {
            FREE!(list);

            list = 0 as *mut *mut libc::c_char;

            first_char_loc = 0;
            first_char = *text as libc::c_int;

            if first_char == '@' as i32 {
                first_char_loc += 1;
            }

            list = hostnames_matching((text as *mut libc::c_char).offset(first_char_loc as isize));
            list_index = 0 as libc::c_int;
        }

        if !list.is_null() && !(*list.offset(list_index as isize)).is_null() {
            let t: *mut libc::c_char;

            t = libc::malloc(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(libc::strlen(*list.offset(list_index as isize)) as u64)
                    as usize,
            ) as *mut libc::c_char;

            *t = first_char as libc::c_char;
            libc::strcpy(
                t.offset(first_char_loc as isize),
                *list.offset(list_index as isize),
            );
            list_index += 1;
            return t;
        }
    }
    return 0 as *mut libc::c_char;
}

#[no_mangle]
pub fn bash_servicename_completion_function(
    text: *const libc::c_char,
    state: libc::c_int,
) -> *mut libc::c_char {
    static mut sname: *mut libc::c_char = 0 as *const c_void as *mut c_void as *mut libc::c_char;
    static mut srvent: *mut servent = 0 as *const servent as *mut servent;
    static mut snamelen: libc::c_int = 0;
    let value: *mut libc::c_char;
    let mut alist: *mut *mut libc::c_char;
    let mut aentry: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut afound: libc::c_int = 0;
    unsafe {
        if state == 0 as libc::c_int {
            FREE!(sname);

            sname = savestring!(text);
            snamelen = libc::strlen(sname) as libc::c_int;
            setservent(0 as libc::c_int);
        }

        loop {
            srvent = getservent();
            if srvent.is_null() {
                break;
            }

            afound = 0;
            if snamelen == 0 || STREQN!(sname, (*srvent).s_name, snamelen) != 0 {
                break;
            }

            alist = (*srvent).s_aliases;
            while !(*alist).is_null() {
                aentry = *alist;
                if STREQN!(sname, aentry, snamelen) != 0 {
                    afound = 1;
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
            return 0 as *mut libc::c_char;
        }
        value = if afound != 0 {
            savestring!(aentry)
        } else {
            savestring!((*srvent).s_name)
        };
    }
    return value;
}

#[no_mangle]
pub fn bash_groupname_completion_function(
    text: *const libc::c_char,
    state: libc::c_int,
) -> *mut libc::c_char {
    static mut gname: *mut libc::c_char = 0 as *const c_void as *mut c_void as *mut libc::c_char;
    static mut grent: *mut group = 0 as *const group as *mut group;
    static mut gnamelen: libc::c_int = 0;
    let value: *mut libc::c_char;
    unsafe {
        if state == 0 {
            FREE!(gname);

            gname = savestring!(text);
            gnamelen = libc::strlen(gname) as libc::c_int;

            setgrent();
        }

        loop {
            grent = getgrent();
            if grent.is_null() {
                break;
            }
            if gnamelen == 0 || STREQN!(gname, (*grent).gr_name, gnamelen) != 0 {
                break;
            }
        }
        if grent.is_null() {
            endgrent();
            return 0 as *mut c_void as *mut libc::c_char;
        }
        value = savestring!((*grent).gr_name);
    }
    return value;
}

fn history_expand_line_internal(line: *mut libc::c_char) -> *mut libc::c_char {
    let new_line: *mut libc::c_char;
    let old_verify: libc::c_int;
    unsafe {
        old_verify = hist_verify;
        hist_verify = 0 as libc::c_int;
        new_line = pre_process_line(line, 0 as libc::c_int, 0 as libc::c_int);
        hist_verify = old_verify;

        return if new_line == line {
            savestring!(line)
        } else {
            new_line
        };
    }
}
fn cleanup_expansion_error() {
    let to_free: *mut libc::c_char;
    let old_verify: libc::c_int;
    unsafe {
        old_verify = hist_verify;
        hist_verify = 0 as libc::c_int;

        fprintf(rl_outstream, b"\r\n\0" as *const u8 as *const libc::c_char);
        to_free = pre_process_line(rl_line_buffer, 1, 0);

        hist_verify = old_verify;

        if to_free != rl_line_buffer {
            FREE!(to_free);
        }
        c_putc('\r' as i32, rl_outstream);
        c_rl_forced_update_display();
    }
}

fn maybe_make_readline_line(new_line: *mut libc::c_char) {
    unsafe {
        if !new_line.is_null() && libc::strcmp(new_line, rl_line_buffer) != 0 as libc::c_int {
            rl_point = rl_end;
            c_rl_add_undo(UNDO_BEGIN!(), 0, 0, 0 as *mut libc::c_char);
            c_rl_delete_text(0 as libc::c_int, rl_point);
            rl_mark = 0 as libc::c_int;
            rl_end = rl_mark;
            rl_point = rl_end;
            c_rl_insert_text(new_line);
            c_rl_add_undo(UNDO_END!(), 0, 0, 0 as *mut libc::c_char);
        }
    }
}

fn set_up_new_line(new_line: *mut libc::c_char) {
    let at_end: libc::c_int;
    unsafe {
        let old_point: libc::c_int = rl_point;
        // old_point = rl_point;
        at_end = (rl_point == rl_end) as libc::c_int;

        maybe_make_readline_line(new_line);
        libc::free(new_line as *mut c_void);

        if at_end != 0 {
            rl_point = rl_end;
        } else if old_point < rl_end {
            rl_point = old_point;
            if !whitespace!(*rl_line_buffer.offset(rl_point as isize)) {
                c_rl_forward_word(1 as libc::c_int, 0 as libc::c_int);
            }
        }
    }
}

fn alias_expand_line(_count: libc::c_int, _ignore: libc::c_int) -> libc::c_int {
    // let new_line: *mut libc::c_char  ;
    let new_line = unsafe { alias_expand(rl_line_buffer) };
    if !new_line.is_null() {
        set_up_new_line(new_line);
        return 0;
    } else {
        cleanup_expansion_error();
        return 1;
    };
}

fn history_expand_line(_count: libc::c_int, _ignore: libc::c_int) -> libc::c_int {
    // let mut new_line: *mut libc::c_char ;

    let new_line = unsafe { history_expand_line_internal(rl_line_buffer) };

    if !new_line.is_null() {
        set_up_new_line(new_line);
        return 0;
    } else {
        cleanup_expansion_error();
        return 1;
    };
}

fn tcsh_magic_space(count: libc::c_int, ignore: libc::c_int) -> libc::c_int {
    let dist_from_end: libc::c_int;
    unsafe {
        let old_point: libc::c_int = rl_point;
        // old_point = rl_point;
        dist_from_end = rl_end - rl_point;

        if history_expand_line(count, ignore) == 0 {
            rl_point = if old_point == 0 {
                old_point
            } else {
                rl_end - dist_from_end
            };
            c_rl_insert(1, ' ' as i32);
            return 0;
        } else {
            return 1;
        };
    }
}

fn history_and_alias_expand_line(_count: libc::c_int, _ignore: libc::c_int) -> libc::c_int {
    let mut new_line: *mut libc::c_char;
    unsafe {
        // new_line = 0 as *mut libc::c_char;
        new_line = history_expand_line_internal(rl_line_buffer);

        if !new_line.is_null() {
            // let alias_line: *mut libc::c_char ;
            let alias_line = alias_expand(new_line);
            libc::free(new_line as *mut c_void);
            new_line = alias_line;
        }
    }
    if !new_line.is_null() {
        set_up_new_line(new_line);
        return 0;
    } else {
        cleanup_expansion_error();
        return 1;
    };
}

fn shell_expand_line(_count: libc::c_int, _ignore: libc::c_int) -> libc::c_int {
    // let mut new_line: *mut libc::c_char ;
    let expanded_string: *mut WordList;
    let w: *mut WordDesc;
    unsafe {
        // let new_line = 0 as *mut libc::c_char;
        let mut new_line = history_expand_line_internal(rl_line_buffer);

        if !new_line.is_null() {
            // let mut alias_line: *mut libc::c_char = 0 as *mut libc::c_char;
            let alias_line = alias_expand(new_line);
            libc::free(new_line as *mut c_void);
            new_line = alias_line;
        }
        if !new_line.is_null() {
            let old_point: libc::c_int = rl_point;
            let at_end: libc::c_int = (rl_point == rl_end) as libc::c_int;

            maybe_make_readline_line(new_line);
            libc::free(new_line as *mut c_void);

            w = alloc_word_desc();
            // let ref mut fresh16 = (*w).word;
            (*w).word = savestring!(rl_line_buffer);
            (*w).flags = if rl_explicit_arg != 0 {
                (W_NOPROCSUB as libc::c_int) | (W_NOCOMSUB as libc::c_int)
            } else {
                0
            };

            expanded_string = expand_word(
                w,
                if rl_explicit_arg != 0 {
                    Q_HERE_DOCUMENT!()
                } else {
                    0 as libc::c_int
                },
            );
            dispose_word(w);

            if expanded_string.is_null() {
                new_line = libc::malloc(1 as usize) as *mut libc::c_char;
                *new_line.offset(0 as isize) = '\u{0}' as i32 as libc::c_char;
            } else {
                new_line = string_list(expanded_string);
                dispose_words(expanded_string);
            }

            maybe_make_readline_line(new_line);
            libc::free(new_line as *mut c_void);

            if at_end != 0 {
                rl_point = rl_end;
            } else if old_point < rl_end {
                rl_point = old_point;
                if !whitespace!(*rl_line_buffer.offset(rl_point as isize)) {
                    c_rl_forward_word(1, 0);
                }
            }
            return 0;
        } else {
            cleanup_expansion_error();
            return 1;
        };
    }
}

static mut fignore: ignorevar = {
    let init = ignorevar {
        varname: b"FIGNORE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ignores: 0 as *const ign as *mut ign,
        num_ignores: 0,
        last_ignoreval: 0 as *mut libc::c_char,
        item_func: None,
    };
    init
};

fn _ignore_completion_names(names: *mut *mut libc::c_char, name_func: Option<sh_ignore_func_t>) {
    let newnames: *mut *mut libc::c_char;
    let mut idx: libc::c_int;
    let mut nidx: libc::c_int;
    let mut oldnames: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut oidx: libc::c_int = 0;
    unsafe {
        // 内存安全检查：验证输入参数
        if names.is_null() || name_func.is_none() {
            return;
        }

        // 安全检查第一个元素
        if (*names.offset(0)).is_null() {
            return;
        }

        if (*names.offset(1 as isize)).is_null() {
            if force_fignore != 0 {
                let first_name = *names.offset(0 as libc::c_int as isize);
                if !first_name.is_null() {
                    if (Some(name_func.expect("non-null function pointer")))
                        .expect("non-null function pointer")(first_name)
                        == 0 as libc::c_int
                    {
                        // 验证指针有效性后再释放
                        let ptr_val = first_name as usize;
                        if ptr_val > 0x1000 && ptr_val < 0x7fffffffffff {
                            libc::free(first_name as *mut c_void);
                        }
                        *names.offset(0 as isize) = 0 as *mut libc::c_char;
                    }
                }
            }
            return;
        }

        // 安全计算数组长度，防止无限循环和缓冲区溢出
        nidx = 1;
        const MAX_COMPLETION_ITEMS: i32 = 50000;
        while nidx < MAX_COMPLETION_ITEMS && !(*names.offset(nidx as isize)).is_null() {
            nidx += 1;
        }

        if nidx >= MAX_COMPLETION_ITEMS {
            return;
        }

        newnames = c_strvec_create(nidx + 1);

        if force_fignore == 0 {
            oldnames = c_strvec_create(nidx - 1);
            oidx = 0;
        }

        *newnames.offset(0 as isize) = *names.offset(0 as isize);
        nidx = 1;
        idx = nidx;
        // 安全遍历数组，添加边界检查
        while idx < MAX_COMPLETION_ITEMS && !(*names.offset(idx as isize)).is_null() {
            let current_name = *names.offset(idx as isize);
            if current_name.is_null() {
                break;
            }

            // 安全调用name_func，防止panic
            let should_keep = match std::panic::catch_unwind(|| {
                (Some(name_func.expect("non-null function pointer")))
                    .expect("non-null function pointer")(current_name)
            }) {
                Ok(result) => result != 0,
                Err(_) => {
                    // 如果函数调用失败，默认保留项目
                    true
                }
            };

            if should_keep {
                if nidx < MAX_COMPLETION_ITEMS {
                    *newnames.offset(nidx as isize) = current_name;
                    nidx = nidx + 1;
                }
            } else if force_fignore == 0 {
                if oidx < MAX_COMPLETION_ITEMS {
                    *oldnames.offset(oidx as isize) = current_name;
                    oidx = oidx + 1;
                }
            } else {
                // 安全释放内存，验证指针有效性
                let ptr_val = current_name as usize;
                if ptr_val > 0x1000 && ptr_val < 0x7fffffffffff {
                    libc::free(current_name as *mut c_void);
                }
            }
            idx += 1;
        }

        *newnames.offset(nidx as isize) = 0 as *mut libc::c_char;

        if nidx == 1 {
            if force_fignore != 0 {
                let first_name = *names.offset(0 as isize);
                if !first_name.is_null() {
                    let ptr_val = first_name as usize;
                    if ptr_val > 0x1000 && ptr_val < 0x7fffffffffff {
                        libc::free(first_name as *mut c_void);
                    }
                    *names.offset(0 as isize) = 0 as *mut libc::c_char;
                }
            } else {
                libc::free(oldnames as *mut c_void);
            }
            libc::free(newnames as *mut c_void);
            return;
        }

        if force_fignore == 0 {
            while oidx != 0 {
                oidx -= 1;
                libc::free(*oldnames.offset(oidx as isize) as *mut c_void);
            }
            libc::free(oldnames as *mut c_void);
        }

        if nidx == 2 {
            libc::free(*names.offset(0 as isize) as *mut c_void);
            *names.offset(0 as isize) = *newnames.offset(1 as isize);
            *names.offset(1 as isize) = 0 as *mut libc::c_char;
            libc::free(newnames as *mut c_void);
            return;
        }

        nidx = 1;
        while !(*newnames.offset(nidx as isize)).is_null() {
            *names.offset(nidx as isize) = *newnames.offset(nidx as isize);
            nidx += 1;
        }

        *names.offset(nidx as isize) = 0 as *mut libc::c_char;
        libc::free(newnames as *mut c_void);
    }
}

fn name_is_acceptable(name: *const libc::c_char) -> libc::c_int {
    let mut p: *mut ign;
    let nlen: libc::c_int;
    unsafe {
        nlen = libc::strlen(name) as libc::c_int;
        p = fignore.ignores;
        while !((*p).val).is_null() {
            if nlen > (*p).len
                && (*p).len > 0
                && (*((*p).val).offset(0 as isize) as libc::c_int
                    == *(&*name.offset((nlen - (*p).len) as isize) as *const libc::c_char)
                        .offset(0 as isize) as libc::c_int
                    && libc::strcmp((*p).val, &*name.offset((nlen - (*p).len) as isize))
                        == 0 as libc::c_int)
            {
                return 0;
            }
            p = p.offset(1);
        }
    }
    return 1 as libc::c_int;
}

fn filename_completion_ignore(names: *mut *mut libc::c_char) -> libc::c_int {
    unsafe {
        setup_ignore_patterns(&mut fignore);

        if fignore.num_ignores == 0 {
            return 0;
        }
    }
    _ignore_completion_names(names, Some(name_is_acceptable));
    return 0;
}

fn test_for_directory(name: *const libc::c_char) -> libc::c_int {
    let fn_0: *mut libc::c_char;
    let r: libc::c_int;
    unsafe {
        fn_0 = bash_tilde_expand(name, 0);
        r = file_isdir(fn_0);
        libc::free(fn_0 as *mut c_void);
    }
    return r;
}

fn test_for_canon_directory(name: *const libc::c_char) -> libc::c_int {
    let mut fn_0: *mut libc::c_char;
    let r: libc::c_int;
    unsafe {
        fn_0 = if *name as libc::c_int == '~' as i32 {
            bash_tilde_expand(name, 0)
        } else {
            savestring!(name)
        };
        bash_filename_stat_hook(&mut fn_0);
        r = file_isdir(fn_0);
        libc::free(fn_0 as *mut c_void);
    }
    return r;
}

fn bash_ignore_filenames(names: *mut *mut libc::c_char) -> libc::c_int {
    _ignore_completion_names(names, Some(test_for_directory));
    return 0;
}

fn bash_progcomp_ignore_filenames(names: *mut *mut libc::c_char) -> libc::c_int {
    _ignore_completion_names(names, Some(test_for_canon_directory));
    return 0;
}

fn return_zero(_name: *const libc::c_char) -> libc::c_int {
    return 0;
}

fn bash_ignore_everything(names: *mut *mut libc::c_char) -> libc::c_int {
    _ignore_completion_names(names, Some(return_zero));
    return 0;
}

fn restore_tilde(val: *mut libc::c_char, directory_part: *mut libc::c_char) -> *mut libc::c_char {
    let l: libc::c_int;
    let mut vl: libc::c_int;
    let dl2: libc::c_int;
    let xl: libc::c_int;
    let mut dh2: *mut libc::c_char;
    let expdir: *mut libc::c_char;
    let ret: *mut libc::c_char;
    let v: *mut libc::c_char;
    unsafe {
        vl = libc::strlen(val) as libc::c_int;
        dh2 = if !directory_part.is_null() {
            bash_dequote_filename(directory_part, 0)
        } else {
            0 as *mut libc::c_char
        };
        bash_directory_expansion(&mut dh2);
        dl2 = libc::strlen(dh2) as libc::c_int;

        expdir = bash_tilde_expand(directory_part, 0);
        xl = libc::strlen(expdir) as libc::c_int;
        if *directory_part as libc::c_int == '~' as i32 && STREQ!(directory_part, expdir) {
            v = c_mbschr(val, '/' as i32);
            vl = STRLEN!(v) as libc::c_int;
            ret = libc::malloc(((xl + vl + 2 as libc::c_int) as size_t).try_into().unwrap())
                as *mut libc::c_char;
            libc::strcpy(ret, directory_part);
            if !v.is_null() && *v as libc::c_int != 0 {
                libc::strcpy(ret.offset(xl as isize), v);
            }

            libc::free(dh2 as *mut c_void);
            libc::free(expdir as *mut c_void);
            return ret;
        }

        libc::free(expdir as *mut c_void);

        l = vl - xl + 1;
        if l <= 0 {
            libc::free(dh2 as *mut c_void);
            return savestring!(val);
        }

        ret = libc::malloc(((dl2 + 2 as libc::c_int + l) as size_t).try_into().unwrap())
            as *mut libc::c_char;
        libc::strcpy(ret, dh2);
        libc::strcpy(ret.offset(dl2 as isize), val.offset(xl as isize));

        libc::free(dh2 as *mut c_void);
    }
    return ret;
}

fn maybe_restore_tilde(
    val: *mut libc::c_char,
    directory_part: *mut libc::c_char,
) -> *mut libc::c_char {
    let save: rl_icppfunc_t;
    let ret: *mut libc::c_char;
    save = if unsafe { dircomplete_expand == 0 as libc::c_int } {
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

fn bash_directory_expansion(dirname: *mut *mut libc::c_char) {
    let mut d: *mut libc::c_char;
    let nd: *mut libc::c_char;
    unsafe {
        d = savestring!(*dirname);
        if rl_directory_rewrite_hook.is_some()
            && (rl_directory_rewrite_hook.expect("non-null function pointer"))(&mut d) != 0
        {
            libc::free(*dirname as *mut c_void);
            *dirname = d;
        } else if rl_directory_completion_hook.is_some()
            && (rl_directory_completion_hook.expect("non-null function pointer"))(&mut d) != 0
        {
            libc::free(*dirname as *mut c_void);
            *dirname = d;
        } else if rl_completion_found_quote != 0 {
            nd = bash_dequote_filename(d, rl_completion_quote_character);
            libc::free(*dirname as *mut c_void);
            libc::free(d as *mut c_void);
            *dirname = nd;
        }
    }
}

fn bash_filename_rewrite_hook(fname: *mut libc::c_char, fnlen: libc::c_int) -> *mut libc::c_char {
    let mut conv: *mut libc::c_char;
    unsafe {
        conv = c_fnx_fromfs(fname, fnlen as size_t);
        if conv != fname {
            conv = savestring!(conv);
        }
    }
    return conv;
}

#[no_mangle]
pub fn set_directory_hook() {
    unsafe {
        if dircomplete_expand != 0 {
            rl_directory_completion_hook =
                Some(bash_directory_completion_hook as fn(*mut *mut libc::c_char) -> libc::c_int);
            rl_directory_rewrite_hook = None;
        } else {
            rl_directory_rewrite_hook =
                Some(bash_directory_completion_hook as fn(*mut *mut libc::c_char) -> libc::c_int);
            rl_directory_completion_hook = None;
        };
    }
}

fn save_directory_hook() -> rl_icppfunc_t {
    let ret: rl_icppfunc_t;
    unsafe {
        if dircomplete_expand != 0 {
            ret = rl_directory_completion_hook;
            rl_directory_completion_hook =
                ::std::mem::transmute::<*mut c_void, rl_icppfunc_t>(0 as *mut c_void);
        } else {
            ret = rl_directory_rewrite_hook;
            rl_directory_rewrite_hook =
                ::std::mem::transmute::<*mut c_void, rl_icppfunc_t>(0 as *mut c_void);
        }
    }
    return ret;
}

fn restore_directory_hook(hookf: rl_icppfunc_t) {
    unsafe {
        if dircomplete_expand != 0 {
            rl_directory_completion_hook = hookf;
        } else {
            rl_directory_rewrite_hook = hookf;
        };
    }
}

fn directory_exists(dirname: *const libc::c_char, should_dequote: libc::c_int) -> libc::c_int {
    let new_dirname: *mut libc::c_char;
    let dirlen: libc::c_int;
    let r: libc::c_int;
    let mut sb: crate::src_common::stat = crate::src_common::stat_init;
    unsafe {
        new_dirname = if should_dequote != 0 {
            bash_dequote_filename(dirname as *mut libc::c_char, rl_completion_quote_character)
        } else {
            savestring!(dirname)
        };

        dirlen = STRLEN!(new_dirname);
        if *new_dirname.offset((dirlen - 1 as libc::c_int) as isize) as libc::c_int == '/' as i32 {
            *new_dirname.offset((dirlen - 1 as libc::c_int) as isize) =
                '\u{0}' as i32 as libc::c_char;
        }
        r = (crate::src_common::c_lstat(new_dirname, &mut sb) == 0) as libc::c_int;
        libc::free(new_dirname as *mut c_void);
    }
    return r;
}

fn bash_filename_stat_hook(dirname: *mut *mut libc::c_char) -> libc::c_int {
    let mut local_dirname: *mut libc::c_char;
    let mut new_dirname: *mut libc::c_char;
    let mut t: *mut libc::c_char;
    let mut should_expand_dirname: libc::c_int;
    let mut return_value: libc::c_int;
    let global_nounset: libc::c_int;
    let wl: *mut WordList;

    local_dirname = unsafe { *dirname };
    return_value = 0;
    should_expand_dirname = return_value;
    t = c_mbschr(local_dirname, '$' as i32);
    if !t.is_null() {
        should_expand_dirname = '$' as i32;
    } else {
        t = c_mbschr(local_dirname, '`' as i32);
        if !t.is_null() {
            should_expand_dirname = '`' as i32;
        }
    }
    if should_expand_dirname != 0 && directory_exists(local_dirname, 0 as libc::c_int) != 0 {
        should_expand_dirname = 0 as libc::c_int;
    }
    unsafe {
        if should_expand_dirname != 0 {
            new_dirname = savestring!(local_dirname);

            global_nounset = unbound_vars_is_error;
            unbound_vars_is_error = 0 as libc::c_int;
            wl = expand_prompt_string(
                new_dirname,
                0 as libc::c_int,
                W_NOCOMSUB as libc::c_int | W_NOPROCSUB as libc::c_int | W_COMPLETE as libc::c_int,
            );
            unbound_vars_is_error = global_nounset;
            if !wl.is_null() {
                libc::free(new_dirname as *mut c_void);
                new_dirname = string_list(wl);

                if !new_dirname.is_null() && *new_dirname as libc::c_int != 0 {
                    libc::free(local_dirname as *mut c_void);
                    *dirname = new_dirname;
                    local_dirname = *dirname;
                    return_value = (STREQ!(local_dirname, *dirname) as libc::c_int
                        == 0 as libc::c_int) as libc::c_int;
                } else {
                    libc::free(new_dirname as *mut c_void);
                }
                dispose_words(wl);
            } else {
                libc::free(new_dirname as *mut c_void);
            }
        }

        if no_symbolic_links == 0 as libc::c_int
            && (*local_dirname.offset(0 as isize) as libc::c_int != '.' as i32
                || *local_dirname.offset(1 as isize) as libc::c_int != 0)
        {
            let temp1: *mut libc::c_char;
            let temp2: *mut libc::c_char;
            t = get_working_directory(
                b"symlink-hook\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            temp1 = make_absolute(local_dirname, t);
            libc::free(t as *mut c_void);
            temp2 = c_sh_canonpath(temp1, 0x1 as libc::c_int | 0x2 as libc::c_int);

            if temp2.is_null() {
                libc::free(temp1 as *mut c_void);
                return return_value;
            }
            libc::free(local_dirname as *mut c_void);
            *dirname = temp2;
            libc::free(temp1 as *mut c_void);
        }
    }
    return return_value;
}

fn bash_directory_completion_hook(dirname: *mut *mut libc::c_char) -> libc::c_int {
    let mut local_dirname: *mut libc::c_char;
    let new_dirname: *mut libc::c_char;
    let mut t: *mut libc::c_char;
    let mut return_value: libc::c_int;
    let mut should_expand_dirname: libc::c_int;
    let mut nextch: libc::c_int;
    let mut closer: libc::c_int;
    let wl: *mut WordList;

    closer = 0;
    nextch = closer;
    should_expand_dirname = nextch;
    // return_value = should_expand_dirname;

    local_dirname = unsafe { *dirname };

    t = c_mbschr(local_dirname, '$' as i32);
    if !t.is_null() {
        should_expand_dirname = '$' as i32;
        nextch = unsafe { *t.offset(1 as isize) as libc::c_int };

        if nextch == '(' as i32 {
            closer = ')' as i32;
        } else if nextch == '{' as i32 {
            closer = '}' as i32;
        } else {
            nextch = 0 as libc::c_int;
        }

        if closer != 0 {
            let p: libc::c_int;
            let mut delims: [libc::c_char; 2] = [0; 2];

            delims[0 as usize] = closer as libc::c_char;
            delims[1 as usize] = 0 as libc::c_char;
            p = skip_to_delim(
                t,
                1,
                delims.as_mut_ptr(),
                SD_NOJMP as libc::c_int | SD_COMPLETE as libc::c_int,
            );
            if unsafe { *t.offset(p as isize) as libc::c_int != closer } {
                should_expand_dirname = 0 as libc::c_int;
            }
        }
    } else if unsafe { *local_dirname.offset(0 as isize) as libc::c_int == '~' as i32 } {
        should_expand_dirname = '~' as i32;
    } else {
        t = c_mbschr(local_dirname, '`' as i32);
        if !t.is_null()
            && unsafe {
                unclosed_pair(
                    local_dirname,
                    libc::strlen(local_dirname) as libc::c_int,
                    b"`\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) == 0 as libc::c_int
            }
        {
            should_expand_dirname = '`' as i32;
        }
    }

    if should_expand_dirname != 0 && directory_exists(local_dirname, 1 as libc::c_int) != 0 {
        should_expand_dirname = 0 as libc::c_int;
    }
    unsafe {
        if should_expand_dirname != 0 {
            new_dirname = savestring!(local_dirname);
            wl = expand_prompt_string(
                new_dirname,
                0 as libc::c_int,
                W_NOCOMSUB as libc::c_int | W_NOPROCSUB as libc::c_int | W_COMPLETE as libc::c_int,
            );

            if !wl.is_null() {
                *dirname = string_list(wl);
                return_value = (STREQ!(local_dirname, *dirname) as libc::c_int == 0 as libc::c_int)
                    as libc::c_int;
                libc::free(local_dirname as *mut c_void);
                libc::free(new_dirname as *mut c_void);
                dispose_words(wl);
                local_dirname = *dirname;

                if !rl_filename_quote_characters.is_null()
                    && *rl_filename_quote_characters as libc::c_int != 0
                {
                    let mut i: libc::c_int;
                    let mut j: libc::c_int;
                    let mut c: libc::c_int;
                    i = libc::strlen(default_filename_quote_characters) as libc::c_int;
                    custom_filename_quote_characters = c_xrealloc(
                        custom_filename_quote_characters as *mut c_void,
                        (i + 1 as libc::c_int) as usize,
                    ) as *mut libc::c_char;

                    j = 0;
                    i = j;
                    loop {
                        c = *default_filename_quote_characters.offset(i as isize) as libc::c_int;
                        if !(c != 0) {
                            break;
                        }
                        if !(c == should_expand_dirname || c == nextch || c == closer) {
                            let fresh29 = j;
                            j = j + 1;
                            *custom_filename_quote_characters.offset(fresh29 as isize) =
                                c as libc::c_char;
                        }
                        i += 1;
                    }
                    *custom_filename_quote_characters.offset(j as isize) =
                        '\u{0}' as i32 as libc::c_char;
                    rl_filename_quote_characters = custom_filename_quote_characters;
                    set_filename_bstab(rl_filename_quote_characters);
                }
            } else {
                libc::free(new_dirname as *mut c_void);
                libc::free(local_dirname as *mut c_void);
                *dirname = libc::malloc(1 as libc::c_int as usize) as *mut libc::c_char;
                **dirname = '\u{0}' as i32 as libc::c_char;
                return 1 as libc::c_int;
            }
        } else {
            new_dirname = bash_dequote_filename(local_dirname, rl_completion_quote_character);
            return_value = ((STREQ!(local_dirname, new_dirname)) as libc::c_int == 0 as libc::c_int)
                as libc::c_int;
            libc::free(local_dirname as *mut c_void);
            *dirname = new_dirname;
            local_dirname = *dirname;
        }

        if no_symbolic_links == 0
            && (*local_dirname.offset(0 as isize) as libc::c_int != '.' as i32
                || *local_dirname.offset(1 as isize) as libc::c_int != 0)
        {
            let mut temp1: *mut libc::c_char;
            let mut temp2: *mut libc::c_char;
            let len1: libc::c_int;
            let len2: libc::c_int;
            t = get_working_directory(
                b"symlink-hook\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            temp1 = make_absolute(local_dirname, t);
            libc::free(t as *mut c_void);
            temp2 = c_sh_canonpath(
                temp1,
                PATH_CHECKDOTDOT!() as libc::c_int | PATH_CHECKEXISTS!() as libc::c_int,
            );

            if temp2.is_null() && dircomplete_spelling != 0 && dircomplete_expand != 0 {
                temp2 = c_dirspell(temp1);
                if !temp2.is_null() {
                    libc::free(temp1 as *mut c_void);
                    temp1 = temp2;
                    temp2 = c_sh_canonpath(
                        temp1,
                        PATH_CHECKDOTDOT!() as libc::c_int | PATH_CHECKEXISTS!() as libc::c_int,
                    );
                    return_value |= (temp2 != 0 as *mut libc::c_char) as libc::c_int;
                }
            }
            if temp2.is_null() {
                libc::free(temp1 as *mut c_void);
                return return_value;
            }
            len1 = libc::strlen(temp1) as libc::c_int;
            if *temp1.offset((len1 - 1) as isize) as libc::c_int == '/' as i32 {
                len2 = libc::strlen(temp2) as libc::c_int;
                if len2 > 2 as libc::c_int {
                    temp2 = c_xrealloc(temp2 as *mut c_void, (len2 + 2 as libc::c_int) as usize)
                        as *mut libc::c_char;
                    *temp2.offset(len2 as isize) = '/' as i32 as libc::c_char;
                    *temp2.offset((len2 + 1) as isize) = '\u{0}' as i32 as libc::c_char;
                }
            }
            if dircomplete_expand_relpath != 0
                || *local_dirname.offset(0 as isize) as libc::c_int != '/' as i32
                    && *local_dirname.offset(0 as isize) as libc::c_int != '.' as i32
                    && STREQ!(temp1, temp2) as libc::c_int == 0 as libc::c_int
            {
                return_value |= (STREQ!(local_dirname, temp2) as libc::c_int == 0 as libc::c_int)
                    as libc::c_int;
            }
            libc::free(local_dirname as *mut c_void);
            *dirname = temp2;
            libc::free(temp1 as *mut c_void);
        }
    }
    return return_value;
}

static mut history_completion_array: *mut *mut libc::c_char =
    0 as *const c_void as *mut c_void as *mut *mut libc::c_char;
static mut harry_size: libc::c_int = 0;
static mut harry_len: libc::c_int = 0;

fn build_history_completion_array() {
    let mut i: libc::c_int;
    let mut j: libc::c_int;
    let hlist: *mut *mut HIST_ENTRY;
    let mut tokens: *mut *mut libc::c_char;
    unsafe {
        if harry_size != 0 {
            c_strvec_dispose(history_completion_array);
            history_completion_array = 0 as *mut c_void as *mut *mut libc::c_char;
            harry_size = 0 as libc::c_int;
            harry_len = 0 as libc::c_int;
        }
        hlist = c_history_list();
        if !hlist.is_null() {
            i = 0 as libc::c_int;
            while !(*hlist.offset(i as isize)).is_null() {
                i += 1;
            }
            i -= 1;
            while i >= 0 as libc::c_int {
                tokens = c_history_tokenize((**hlist.offset(i as isize)).line);
                j = 0 as libc::c_int;
                while !tokens.is_null() && !(*tokens.offset(j as isize)).is_null() {
                    if harry_len + 2 as libc::c_int > harry_size {
                        harry_size += 10 as libc::c_int;
                        history_completion_array =
                            c_strvec_resize(history_completion_array, harry_size);
                    }

                    *history_completion_array.offset(harry_len as isize) =
                        *tokens.offset(j as isize);
                    harry_len = harry_len + 1;
                    *history_completion_array.offset(harry_len as isize) = 0 as *mut libc::c_char;

                    j += 1;
                }
                libc::free(tokens as *mut c_void);
                i -= 1;
            }

            if dabbrev_expand_active == 0 {
                c_qsort(
                    history_completion_array as *mut c_void,
                    harry_len as usize,
                    ::std::mem::size_of::<*mut libc::c_char>() as usize,
                    ::std::mem::transmute::<
                        fn(*mut *mut libc::c_char, *mut *mut libc::c_char) -> i32,
                        Option<fn(*const c_void, *const c_void) -> i32>,
                    >(c_strvec_strcmp),
                );
            }
        }
    }
}

fn history_completion_generator(
    hint_text: *const libc::c_char,
    state: libc::c_int,
) -> *mut libc::c_char {
    static mut local_index: libc::c_int = 0;
    static mut len: libc::c_int = 0;
    static mut text: *const libc::c_char = 0 as *const libc::c_char;
    unsafe {
        if state == 0 {
            if dabbrev_expand_active != 0 {
                rl_completion_suppress_append = 1;
            }
            local_index = 0;
            build_history_completion_array();
            text = hint_text;
            len = libc::strlen(text) as libc::c_int;
        }

        while !history_completion_array.is_null()
            && !(*history_completion_array.offset(local_index as isize)).is_null()
        {
            let local_index_temp = local_index; //local_index_temp 临时变量
            local_index = local_index + 1;
            if libc::strncmp(
                text,
                *history_completion_array.offset(local_index_temp as isize),
                len as usize,
            ) == 0 as libc::c_int
            {
                return savestring!(*history_completion_array.offset((local_index - 1) as isize));
            }
        }
    }
    return 0 as *mut libc::c_char;
}

fn dynamic_complete_history(_count: libc::c_int, _key: libc::c_int) -> libc::c_int {
    let r: libc::c_int;
    let orig_func: Option<rl_compentry_func_t>;
    let orig_attempt_func: Option<rl_completion_func_t>;
    let orig_ignore_func: Option<rl_compignore_func_t>;
    unsafe {
        orig_func = rl_completion_entry_function;
        orig_attempt_func = rl_attempted_completion_function;
        orig_ignore_func = rl_ignore_some_completions_function;

        rl_completion_entry_function = Some(history_completion_generator);
        rl_attempted_completion_function = None;
        rl_ignore_some_completions_function = Some(filename_completion_ignore);

        if rl_last_func
            == ::std::mem::transmute::<Option<fn() -> libc::c_int>, Option<rl_command_func_t>>(
                Some(::std::mem::transmute::<
                    fn(libc::c_int, libc::c_int) -> libc::c_int,
                    fn() -> libc::c_int,
                >(dynamic_complete_history)),
            )
        {
            r = c_rl_complete_internal('?' as i32);
        } else {
            r = c_rl_complete_internal('\t' as i32);
        }

        rl_completion_entry_function = orig_func;
        rl_attempted_completion_function = orig_attempt_func;
        rl_ignore_some_completions_function = orig_ignore_func;
    }
    return r;
}

fn bash_dabbrev_expand(count: libc::c_int, key: libc::c_int) -> libc::c_int {
    let r: libc::c_int;
    let orig_suppress: libc::c_int;
    let orig_sort: libc::c_int;
    let orig_func: Option<rl_compentry_func_t>;
    let orig_attempt_func: Option<rl_completion_func_t>;
    let orig_ignore_func: Option<rl_compignore_func_t>;
    unsafe {
        orig_func = rl_menu_completion_entry_function;
        orig_attempt_func = rl_attempted_completion_function;
        orig_ignore_func = rl_ignore_some_completions_function;
        orig_suppress = rl_completion_suppress_append;
        orig_sort = rl_sort_completion_matches;

        rl_menu_completion_entry_function = Some(history_completion_generator);
        rl_attempted_completion_function = None;
        rl_ignore_some_completions_function = Some(filename_completion_ignore);
        rl_filename_completion_desired = 0 as libc::c_int;
        rl_completion_suppress_append = 1 as libc::c_int;
        rl_sort_completion_matches = 0 as libc::c_int;

        dabbrev_expand_active = 1 as libc::c_int;
        if rl_last_func
            == ::std::mem::transmute::<Option<fn() -> libc::c_int>, Option<rl_command_func_t>>(
                Some(::std::mem::transmute::<
                    fn(libc::c_int, libc::c_int) -> libc::c_int,
                    fn() -> libc::c_int,
                >(bash_dabbrev_expand)),
            )
        {
            rl_last_func = Some(c_rl_menu_complete);
        }

        r = c_rl_menu_complete(count, key);
        dabbrev_expand_active = 0 as libc::c_int;
        rl_last_func = ::std::mem::transmute::<
            Option<fn() -> libc::c_int>,
            Option<rl_command_func_t>,
        >(Some(::std::mem::transmute::<
            fn(libc::c_int, libc::c_int) -> libc::c_int,
            fn() -> libc::c_int,
        >(bash_dabbrev_expand)));
        rl_menu_completion_entry_function = orig_func;
        rl_attempted_completion_function = orig_attempt_func;
        rl_ignore_some_completions_function = orig_ignore_func;
        rl_completion_suppress_append = orig_suppress;
        rl_sort_completion_matches = orig_sort;
    }
    return r;
}

fn bash_complete_username(_ignore: libc::c_int, _ignore2: libc::c_int) -> libc::c_int {
    unsafe {
        return bash_complete_username_internal(c_rl_completion_mode(::core::mem::transmute::<
            Option<fn() -> libc::c_int>,
            Option<rl_command_func_t>,
        >(Some(
            ::core::mem::transmute::<
                fn(libc::c_int, libc::c_int) -> libc::c_int,
                fn() -> libc::c_int,
            >(bash_complete_username),
        ))));
    }
}

fn bash_possible_username_completions(_ignore: libc::c_int, _ignore2: libc::c_int) -> libc::c_int {
    return bash_complete_username_internal('?' as i32);
}

fn bash_complete_username_internal(what_to_do: libc::c_int) -> libc::c_int {
    return bash_specific_completion(what_to_do, Some(c_rl_username_completion_function));
}

fn bash_complete_filename(_ignore: libc::c_int, _ignore2: libc::c_int) -> libc::c_int {
    unsafe {
        return bash_complete_filename_internal(c_rl_completion_mode(::std::mem::transmute::<
            Option<fn() -> libc::c_int>,
            Option<rl_command_func_t>,
        >(Some(
            ::std::mem::transmute::<fn(libc::c_int, libc::c_int) -> libc::c_int, fn() -> libc::c_int>(
                bash_complete_filename,
            ),
        ))));
    }
}

fn bash_possible_filename_completions(_ignore: libc::c_int, _ignore2: libc::c_int) -> libc::c_int {
    return bash_complete_filename_internal('?' as i32);
}

fn bash_complete_filename_internal(what_to_do: libc::c_int) -> libc::c_int {
    let orig_func: Option<rl_compentry_func_t>;
    let orig_attempt_func: Option<rl_completion_func_t>;
    let orig_dir_func: rl_icppfunc_t;
    let orig_ignore_func: Option<rl_compignore_func_t>;
    let orig_rl_completer_word_break_characters: *mut libc::c_char;
    let r: libc::c_int;
    unsafe {
        orig_func = rl_completion_entry_function;
        orig_attempt_func = rl_attempted_completion_function;
        orig_ignore_func = rl_ignore_some_completions_function;
        orig_rl_completer_word_break_characters = rl_completer_word_break_characters;
        orig_dir_func = save_directory_hook();
        rl_completion_entry_function = Some(c_rl_filename_completion_function);

        rl_completion_entry_function = Some(c_rl_filename_completion_function);
        rl_attempted_completion_function = None;
        rl_ignore_some_completions_function = Some(filename_completion_ignore);
        rl_completer_word_break_characters =
            b" \t\n\"'\0" as *const u8 as *const libc::c_char as *mut libc::c_char;

        r = c_rl_complete_internal(what_to_do);
        rl_completion_entry_function = orig_func;
        rl_attempted_completion_function = orig_attempt_func;
        rl_ignore_some_completions_function = orig_ignore_func;
        rl_completer_word_break_characters = orig_rl_completer_word_break_characters;
        restore_directory_hook(orig_dir_func);
    }
    return r;
}

fn bash_complete_hostname(_ignore: libc::c_int, _ignore2: libc::c_int) -> libc::c_int {
    unsafe {
        return bash_complete_hostname_internal(c_rl_completion_mode(::std::mem::transmute::<
            Option<fn() -> libc::c_int>,
            Option<rl_command_func_t>,
        >(Some(
            ::std::mem::transmute::<fn(libc::c_int, libc::c_int) -> libc::c_int, fn() -> libc::c_int>(
                bash_complete_hostname,
            ),
        ))));
    }
}

fn bash_possible_hostname_completions(_ignore: libc::c_int, _ignore2: libc::c_int) -> libc::c_int {
    return bash_complete_hostname_internal('?' as i32);
}

fn bash_complete_variable(_ignore: libc::c_int, _ignore2: libc::c_int) -> libc::c_int {
    unsafe {
        return bash_complete_variable_internal(c_rl_completion_mode(::std::mem::transmute::<
            Option<fn() -> libc::c_int>,
            Option<rl_command_func_t>,
        >(Some(
            ::std::mem::transmute::<fn(libc::c_int, libc::c_int) -> libc::c_int, fn() -> libc::c_int>(
                bash_complete_variable,
            ),
        ))));
    }
}

fn bash_possible_variable_completions(_ignore: libc::c_int, _ignore2: libc::c_int) -> libc::c_int {
    return bash_complete_variable_internal('?' as i32);
}

fn bash_complete_command(_ignore: libc::c_int, _ignore2: libc::c_int) -> libc::c_int {
    unsafe {
        return bash_complete_command_internal(c_rl_completion_mode(::std::mem::transmute::<
            Option<fn() -> libc::c_int>,
            Option<rl_command_func_t>,
        >(Some(
            ::std::mem::transmute::<fn(libc::c_int, libc::c_int) -> libc::c_int, fn() -> libc::c_int>(
                bash_complete_command,
            ),
        ))));
    }
}

fn bash_possible_command_completions(_ignore: libc::c_int, _ignore2: libc::c_int) -> libc::c_int {
    return bash_complete_command_internal('?' as i32);
}

fn bash_complete_hostname_internal(what_to_do: libc::c_int) -> libc::c_int {
    return bash_specific_completion(what_to_do, Some(hostname_completion_function));
}

fn bash_complete_variable_internal(what_to_do: libc::c_int) -> libc::c_int {
    return bash_specific_completion(what_to_do, Some(variable_completion_function));
}

fn bash_complete_command_internal(what_to_do: libc::c_int) -> libc::c_int {
    return bash_specific_completion(what_to_do, Some(command_word_completion_function));
}

fn completion_glob_pattern(string: *mut libc::c_char) -> libc::c_int {
    return (c_glob_pattern_p(string) == 1 as libc::c_int) as libc::c_int;
}

static mut globtext: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut globorig: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;

fn glob_complete_word(text: *const libc::c_char, state: libc::c_int) -> *mut libc::c_char {
    static mut matches: *mut *mut libc::c_char =
        0 as *const c_void as *mut c_void as *mut *mut libc::c_char;
    static mut ind: libc::c_int = 0;
    let glen: libc::c_int;
    let ret: *mut libc::c_char;
    let ttext: *mut libc::c_char;
    unsafe {
        if state == 0 {
            rl_filename_completion_desired = 1;
            if !matches.is_null() {
                FREE!(matches);
            }
            if globorig != globtext {
                FREE!(globorig);
            }
            FREE!(globtext);

            ttext = bash_tilde_expand(text, 0);
            if rl_explicit_arg != 0 {
                globorig = savestring!(ttext);
                glen = libc::strlen(ttext) as libc::c_int;
                globtext = libc::malloc((glen + 2 as libc::c_int) as usize) as *mut libc::c_char;
                libc::strcpy(globtext, ttext);
                *globtext.offset(glen as isize) = '*' as i32 as libc::c_char;
                *globtext.offset((glen + 1 as libc::c_int) as isize) =
                    '\u{0}' as i32 as libc::c_char;
            } else {
                globorig = savestring!(ttext);
                globtext = globorig;
            }
            if ttext != text as *mut libc::c_char {
                libc::free(ttext as *mut c_void);
            }
            matches = shell_glob_filename(globtext, 0);
            if GLOB_FAILED!(matches) {
                matches = 0 as *mut *mut libc::c_char;
            }
            ind = 0;
        }

        ret = if !matches.is_null() {
            *matches.offset(ind as isize)
        } else {
            0 as *mut libc::c_char
        };
        ind += 1;
    }
    return ret;
}

fn bash_glob_completion_internal(what_to_do: libc::c_int) -> libc::c_int {
    return bash_specific_completion(what_to_do, Some(glob_complete_word));
}

fn bash_glob_quote_filename(
    s: *mut libc::c_char,
    rtype: libc::c_int,
    qcp: *mut libc::c_char,
) -> *mut libc::c_char {
    unsafe {
        if !globorig.is_null()
            && !qcp.is_null()
            && *qcp as libc::c_int == '\u{0}' as i32
            && STREQ!(s, globorig)
        {
            return savestring!(s);
        } else {
            return bash_quote_filename(s, rtype, qcp);
        };
    }
}

fn bash_glob_complete_word(_count: libc::c_int, _key: libc::c_int) -> libc::c_int {
    let r: libc::c_int;
    let orig_quoting_function: Option<rl_quote_func_t>;
    unsafe {
        if rl_editing_mode == EMACS_EDITING_MODE!() {
            rl_explicit_arg = 1;
        }
        orig_quoting_function = rl_filename_quoting_function;
        rl_filename_quoting_function = Some(bash_glob_quote_filename);

        r = bash_glob_completion_internal(c_rl_completion_mode(Some(bash_glob_complete_word)));

        rl_filename_quoting_function = orig_quoting_function;
    }
    return r;
}

fn bash_glob_expand_word(_count: libc::c_int, _key: libc::c_int) -> libc::c_int {
    return bash_glob_completion_internal('*' as i32);
}

fn bash_glob_list_expansions(_count: libc::c_int, _key: libc::c_int) -> libc::c_int {
    return bash_glob_completion_internal('?' as i32);
}

fn bash_specific_completion(
    what_to_do: libc::c_int,
    generator: Option<rl_compentry_func_t>,
) -> libc::c_int {
    let orig_func: Option<rl_compentry_func_t>;
    let orig_attempt_func: Option<rl_completion_func_t>;
    let orig_ignore_func: Option<rl_compignore_func_t>;
    let r: libc::c_int;
    unsafe {
        orig_func = rl_completion_entry_function;
        orig_attempt_func = rl_attempted_completion_function;
        orig_ignore_func = rl_ignore_some_completions_function;
        rl_completion_entry_function = generator;
        rl_attempted_completion_function = None;
        rl_ignore_some_completions_function = orig_ignore_func;
        r = c_rl_complete_internal(what_to_do);
        rl_completion_entry_function = orig_func;
        rl_attempted_completion_function = orig_attempt_func;
        rl_ignore_some_completions_function = orig_ignore_func;
    }
    return r;
}

fn bash_vi_complete(count: libc::c_int, key: libc::c_int) -> libc::c_int {
    let mut p: libc::c_int;
    let mut r: libc::c_int;
    let mut t: *mut libc::c_char;
    unsafe {
        if rl_point < rl_end && !whitespace!(*rl_line_buffer.offset(rl_point as isize)) {
            if !whitespace!(*rl_line_buffer.offset((rl_point + 1) as isize)) {
                c_rl_vi_end_word(1, 'E' as i32);
            }
            rl_point += 1;
        }

        t = 0 as *mut libc::c_char;
        if rl_point > 0 {
            p = rl_point;
            c_rl_vi_bWord(1, 'B' as i32);
            r = rl_point;
            rl_point = p;
            p = r;
            t = substring(rl_line_buffer, p, rl_point);
        }

        if !t.is_null() && completion_glob_pattern(t) == 0 {
            rl_explicit_arg = 1;
        }
        FREE!(t);
    }
    if key == '*' as i32 {
        r = bash_glob_expand_word(count, key);
    } else if key == '=' as i32 {
        r = bash_glob_list_expansions(count, key);
    } else if key == '\\' as i32 {
        r = bash_glob_complete_word(count, key);
    } else {
        r = c_rl_complete(0, key);
    }
    if key == '*' as i32 || key == '\\' as i32 {
        c_rl_vi_start_inserting(key, 1, 1);
    }

    return r;
}

fn bash_dequote_filename(text: *mut libc::c_char, quote_char: libc::c_int) -> *mut libc::c_char {
    let ret: *mut libc::c_char;
    let mut p: *mut libc::c_char;
    let mut r: *mut libc::c_char;
    let l: libc::c_int;
    let mut quoted: libc::c_int;
    unsafe {
        l = libc::strlen(text) as libc::c_int;
        ret = libc::malloc((l + 1 as libc::c_int) as usize) as *mut libc::c_char;

        quoted = quote_char;
        p = text;
        r = ret;
        while !p.is_null() && *p as libc::c_int != 0 {
            if *p as libc::c_int == '\\' as i32 {
                if quoted == '\'' as i32 {
                    *r = *p;
                    r = r.offset(1);
                } else if quoted == '"' as i32
                    && *sh_syntaxtab
                        .as_mut_ptr()
                        .offset(*p.offset(1 as libc::c_int as isize) as libc::c_uchar as isize)
                        & CBSDQUOTE!()
                        == 0
                {
                    *r = *p;
                    r = r.offset(1);
                }

                p = p.offset(1);
                *r = *p;
                r = r.offset(1);

                if *p == '\u{0}' as libc::c_char {
                    return ret;
                }
            } else if quoted != 0 && *p as libc::c_int == quoted {
                quoted = 0;
            } else if quoted == 0
                && (*p as libc::c_int == '\'' as i32 || *p as libc::c_int == '"' as i32)
            {
                quoted = *p as libc::c_int;
            } else {
                *r = *p;
                r = r.offset(1);
            }

            p = p.offset(1);
        }
        *r = '\u{0}' as i32 as libc::c_char;
    }
    return ret;
}

fn quote_word_break_chars(text: *mut libc::c_char) -> *mut libc::c_char {
    let ret: *mut libc::c_char;
    let mut r: *mut libc::c_char;
    let mut s: *mut libc::c_char;
    let l: libc::c_int;
    unsafe {
        l = libc::strlen(text) as libc::c_int;
        ret = libc::malloc(
            ((2 as libc::c_int * l + 1 as libc::c_int) as size_t)
                .try_into()
                .unwrap(),
        ) as *mut libc::c_char;

        s = text;
        r = ret;
        while *s != 0 {
            if *s as libc::c_int == '\\' as i32 {
                *r = '\\' as i32 as libc::c_char;
                r = r.offset(1);

                s = s.offset(1);
                *r = *s;
                r = r.offset(1);

                if *s as libc::c_int == '\u{0}' as i32 {
                    break;
                }
            } else {
                if !(c_mbschr(rl_completer_word_break_characters, *s as libc::c_int)).is_null() {
                    *r = '\\' as i32 as libc::c_char;
                    r = r.offset(1);
                }
                if s == text && *s as libc::c_int == '~' as i32 && file_exists(text) != 0 {
                    *r = '\\' as i32 as libc::c_char;
                    r = r.offset(1);
                }
                *r = *s;
                r = r.offset(1);
            }
            s = s.offset(1);
        }
        *r = '\u{0}' as i32 as libc::c_char;
    }
    return ret;
}

fn set_filename_bstab(string: *const libc::c_char) {
    unsafe {
        let mut s: *const libc::c_char;
        memset(
            filename_bstab.as_mut_ptr() as *mut c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[libc::c_char; 256]>() as usize,
        );
        s = string;
        while !s.is_null() && *s as libc::c_int != 0 {
            filename_bstab[*s as libc::c_uchar as usize] = 1 as libc::c_int as libc::c_char;
            s = s.offset(1);
        }
    }
}

fn bash_quote_filename(
    s: *mut libc::c_char,
    rtype: libc::c_int,
    qcp: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut rtext: *mut libc::c_char;
    let mut mtext: *mut libc::c_char;
    let ret: *mut libc::c_char;
    let rlen: libc::c_int;
    let mut cs: libc::c_int;

    rtext = 0 as *mut libc::c_char;
    unsafe {
        cs = completion_quoting_style;

        if *qcp as libc::c_int == '\u{0}' as i32
            && cs == COMPLETE_BSQUOTE!()
            && !(c_mbschr(s, '\n' as i32)).is_null()
        {
            cs = COMPLETE_SQUOTE!();
        } else if *qcp as libc::c_int == '"' as i32 {
            cs = COMPLETE_DQUOTE!();
        } else if *qcp as libc::c_int == '\'' as i32 {
            cs = COMPLETE_SQUOTE!();
        } else if *qcp as libc::c_int == '\u{0}' as i32
            && history_expansion != 0
            && cs == COMPLETE_DQUOTE!()
            && history_expansion_inhibited == 0
            && !(c_mbschr(s, '!' as i32)).is_null()
        {
            cs = COMPLETE_BSQUOTE!();
        }
        if *qcp as libc::c_int == '"' as i32
            && history_expansion != 0
            && cs == COMPLETE_DQUOTE!()
            && history_expansion_inhibited == 0
            && !(c_mbschr(s, '!' as i32)).is_null()
        {
            cs = COMPLETE_BSQUOTE!();
            *qcp = '\u{0}' as i32 as libc::c_char;
        }

        mtext = s;
        if *mtext.offset(0 as libc::c_int as isize) as libc::c_int == '~' as i32
            && rtype == SINGLE_MATCH as libc::c_int
            && cs != COMPLETE_BSQUOTE!()
        {
            mtext = bash_tilde_expand(s, 0);
        }
        match cs {
            COMPLETE_DQUOTE!() => {
                rtext = c_sh_double_quote(mtext);
            }
            COMPLETE_SQUOTE!() => {
                rtext = c_sh_single_quote(mtext);
            }
            COMPLETE_BSQUOTE!() => {
                rtext = c_sh_backslash_quote(
                    mtext,
                    if complete_fullquote != 0 {
                        0 as *mut libc::c_char
                    } else {
                        filename_bstab.as_mut_ptr()
                    },
                    0 as libc::c_int,
                );
            }
            _ => {}
        }

        if mtext != s {
            libc::free(mtext as *mut c_void);
        }

        if !rtext.is_null() && cs == COMPLETE_BSQUOTE!() {
            mtext = quote_word_break_chars(rtext);
            libc::free(rtext as *mut c_void);
            rtext = mtext;
        }

        if !rtext.is_null() {
            rlen = libc::strlen(rtext) as libc::c_int;
            ret = libc::malloc((rlen + 1 as libc::c_int) as usize) as *mut libc::c_char;
            libc::strcpy(ret, rtext);
        } else {
            rlen = 1;
            ret = libc::malloc(rlen as usize) as *mut libc::c_char;
            *ret.offset(0 as isize) = '\u{0}' as i32 as libc::c_char;
        }

        if rtype == MULT_MATCH as libc::c_int && cs != COMPLETE_BSQUOTE!() {
            *ret.offset((rlen - 1 as libc::c_int) as isize) = '\u{0}' as i32 as libc::c_char;
        }
        libc::free(rtext as *mut c_void);
    }
    return ret;
}

static mut emacs_std_cmd_xmap: Keymap = 0 as *const KEYMAP_ENTRY as *mut KEYMAP_ENTRY;
static mut vi_insert_cmd_xmap: Keymap = 0 as *const KEYMAP_ENTRY as *mut KEYMAP_ENTRY;
static mut vi_movement_cmd_xmap: Keymap = 0 as *const KEYMAP_ENTRY as *mut KEYMAP_ENTRY;

// fn putx(c: libc::c_int) -> libc::c_int {
//     let mut x: libc::c_int = 0;
//     unsafe {
//         x = c_putc(c, rl_outstream);
//     }
//     return x;
// }

fn readline_get_char_offset(ind: libc::c_int) -> libc::c_int {
    let mut r: libc::c_int;
    let old_ch: libc::c_int;

    r = ind;
    unsafe {
        if locale_mb_cur_max > 1 {
            old_ch = *rl_line_buffer.offset(ind as isize) as libc::c_int;
            *rl_line_buffer.offset(ind as isize) = '\u{0}' as i32 as libc::c_char;
            r = MB_STRLEN!(rl_line_buffer);
            *rl_line_buffer.offset(ind as isize) = old_ch as libc::c_char;
        }
    }
    return r;
}

fn readline_set_char_offset(ind: libc::c_int, varp: *mut libc::c_int) {
    let mut i: libc::c_int;
    i = ind;
    unsafe {
        if i > 0 as libc::c_int && locale_mb_cur_max > 1 as libc::c_int {
            i = c__rl_find_next_mbchar(rl_line_buffer, 0 as libc::c_int, i, 0 as libc::c_int);
        }
        if i != *varp {
            if i > rl_end {
                i = rl_end;
            } else if i < 0 as libc::c_int {
                i = 0 as libc::c_int;
            }
            *varp = i;
        }
    }
}

#[no_mangle]
pub fn bash_execute_unix_command(_count: libc::c_int, _key: libc::c_int) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut i: libc::c_int;
    let r: libc::c_int;
    let mut mi: intmax_t = 0;
    let mut ps: sh_parser_state_t = sh_parser_state_t {
        parser_state: 0,
        token_state: 0 as *mut libc::c_int,
        token: 0 as *mut libc::c_char,
        token_buffer_size: 0,
        input_line_terminator: 0,
        eof_encountered: 0,
        prompt_string_pointer: 0 as *mut *mut libc::c_char,
        current_command_line_count: 0,
        remember_on_history: 0,
        history_expansion_inhibited: 0,
        last_command_exit_value: 0,
        pipestatus: 0 as *mut ARRAY,
        last_shell_builtin: None,
        this_shell_builtin: None,
        expand_aliases: 0,
        echo_input_at_read: 0,
        need_here_doc: 0,
        here_doc_first_line: 0,
        redir_stack: [0 as *mut REDIRECT; 16],
    };
    let mut cmd: *mut libc::c_char;
    let mut value: *mut libc::c_char;
    let ce: *mut libc::c_char;
    // let old_ch: libc::c_char = 0;
    let mut v: *mut SHELL_VAR;
    let mut ibuf: [libc::c_char; 12] = [0; 12];
    let cmd_xmap: Keymap;

    cmd_xmap = get_cmd_xmap_from_keymap(c_rl_get_keymap());
    cmd = unsafe {
        ::std::mem::transmute::<Option<rl_command_func_t>, *mut libc::c_char>(
            c_rl_function_of_keyseq_len(
                rl_executing_keyseq,
                rl_key_sequence_length as size_t,
                cmd_xmap,
                &mut type_0,
            ),
        )
    };
    unsafe {
        if type_0 == ISKMAP!() as libc::c_int && {
            type_0 = (*(cmd as Keymap).offset(ANYOTHERKEY!() as isize)).Type as libc::c_int;
            type_0 == ISMACR as libc::c_int
        } {
            cmd = ::core::mem::transmute::<Option<rl_command_func_t>, *mut libc::c_char>(
                (*(cmd as Keymap).offset(ANYOTHERKEY!() as isize)).function,
            );
        }
    }
    if cmd.is_null() || type_0 != ISMACR as libc::c_int {
        c_rl_crlf();
        unsafe {
            internal_error(
                b"bash_execute_unix_command: cannot find keymap for command\0" as *const u8
                    as *const libc::c_char,
            );
        }
        c_rl_forced_update_display();
        return 1 as libc::c_int;
    }

    ce = c_rl_get_termcap(b"ce\0" as *const u8 as *const libc::c_char);
    if !ce.is_null() {
        c_rl_clear_visible_line();
        unsafe {
            fflush(rl_outstream);
        }
    } else {
        c_rl_crlf();
    }

    v = unsafe {
        bind_variable(
            b"READLINE_LINE\0" as *const u8 as *const libc::c_char,
            rl_line_buffer,
            0 as libc::c_int,
        )
    };

    if !v.is_null() {
        unsafe {
            VSETATTR_1!(v, att_exported as i32);
        }
    }

    i = unsafe { readline_get_char_offset(rl_point) };
    value = c_inttostr(
        i as intmax_t,
        ibuf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
    );
    v = bind_int_variable(
        b"READLINE_POINT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        value,
        0,
    );

    if !v.is_null() {
        unsafe {
            VSETATTR_1!(v, att_exported as i32);
        }
    }

    i = unsafe { readline_get_char_offset(rl_mark) };
    value = c_inttostr(
        i as intmax_t,
        ibuf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
    );
    v = bind_int_variable(
        b"READLINE_MARK\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        value,
        0,
    );

    if !v.is_null() {
        unsafe { VSETATTR!(v, att_exported as i32) };
    }
    unsafe {
        array_needs_making = 1;
    }

    save_parser_state(&mut ps);
    c_rl_clear_signals();
    r = unsafe {
        parse_and_execute(
            savestring!(cmd),
            b"bash_execute_unix_command\0" as *const u8 as *const libc::c_char,
            SEVAL_NOHIST as libc::c_int,
        )
    };
    c_rl_set_signals();
    restore_parser_state(&mut ps);

    v = find_variable(b"READLINE_LINE\0" as *const u8 as *const libc::c_char);
    maybe_make_readline_line(if !v.is_null() {
        unsafe { value_cell!(v) }
    } else {
        0 as *mut libc::c_char
    });

    v = find_variable(b"READLINE_POINT\0" as *const u8 as *const libc::c_char);
    unsafe {
        if !v.is_null() && legal_number(value_cell!(v), &mut mi) != 0 {
            readline_set_char_offset(mi as libc::c_int, &mut rl_point);
        }
    }
    v = find_variable(b"READLINE_MARK\0" as *const u8 as *const libc::c_char);
    unsafe {
        if !v.is_null() && legal_number(value_cell!(v), &mut mi) != 0 {
            readline_set_char_offset(mi as libc::c_int, &mut rl_mark);
        }
    }

    check_unbind_variable(b"READLINE_LINE\0" as *const u8 as *const libc::c_char);
    check_unbind_variable(b"READLINE_POINT\0" as *const u8 as *const libc::c_char);
    check_unbind_variable(b"READLINE_MARK\0" as *const u8 as *const libc::c_char);
    unsafe {
        array_needs_making = 1;
    }

    if !ce.is_null() && r != 124 {
        c_rl_redraw_prompt_last_line();
    } else {
        c_rl_forced_update_display();
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub fn print_unix_command_map() -> libc::c_int {
    let save: Keymap;
    let cmd_xmap: Keymap;

    save = c_rl_get_keymap();
    cmd_xmap = get_cmd_xmap_from_keymap(save);
    c_rl_set_keymap(cmd_xmap);
    c_rl_macro_dumper(1 as libc::c_int);
    c_rl_set_keymap(save);

    return 0 as libc::c_int;
}

fn init_unix_command_map() {
    unsafe {
        emacs_std_cmd_xmap = c_rl_make_bare_keymap();

        (*emacs_std_cmd_xmap.offset(CTRL!('X' as i32) as isize)).Type =
            ISKMAP as libc::c_int as libc::c_char;
        // let ref mut fresh43 = (*emacs_std_cmd_xmap.offset(CTRL!('X' as i32) as isize)).function;
        (*emacs_std_cmd_xmap.offset(CTRL!('X' as i32) as isize)).function =
            ::std::mem::transmute::<Keymap, Option<rl_command_func_t>>(c_rl_make_bare_keymap());

        (*emacs_std_cmd_xmap.offset(ESC!() as isize)).Type = ISKMAP as libc::c_int as libc::c_char;
        (*emacs_std_cmd_xmap.offset(ESC!() as isize)).function =
            ::std::mem::transmute::<Keymap, Option<rl_command_func_t>>(c_rl_make_bare_keymap());

        vi_insert_cmd_xmap = c_rl_make_bare_keymap();
        vi_movement_cmd_xmap = c_rl_make_bare_keymap();
    }
}

fn get_cmd_xmap_from_keymap(kmap: Keymap) -> Keymap {
    unsafe {
        if emacs_std_cmd_xmap.is_null() {
            init_unix_command_map();
        }

        if kmap == emacs_standard_keymap.as_mut_ptr() {
            return emacs_std_cmd_xmap;
        } else if kmap == emacs_meta_keymap.as_mut_ptr() {
            return ::core::mem::transmute::<Option<rl_command_func_t>, Keymap>(
                (*emacs_std_cmd_xmap.offset(('[' as i32 & 0x1f as libc::c_int) as isize)).function,
            );
        } else if kmap == emacs_ctlx_keymap.as_mut_ptr() {
            return ::core::mem::transmute::<Option<rl_command_func_t>, Keymap>(
                (*emacs_std_cmd_xmap.offset(('X' as i32 & 0x1f as libc::c_int) as isize)).function,
            );
        } else if kmap == vi_insertion_keymap.as_mut_ptr() {
            return vi_insert_cmd_xmap;
        } else if kmap == vi_movement_keymap.as_mut_ptr() {
            return vi_movement_cmd_xmap;
        } else {
            return 0 as *mut c_void as Keymap;
        };
    }
}

fn isolate_sequence(
    string: *mut libc::c_char,
    ind: libc::c_int,
    need_dquote: libc::c_int,
    startp: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int;
    let mut c: libc::c_int;
    let mut passc: libc::c_int;
    let delim: libc::c_int;

    i = ind;

    while unsafe {
        *string.offset(i as isize) as libc::c_int != 0 && whitespace!(*string.offset(i as isize))
    } {
        i += 1;
    }
    unsafe {
        if need_dquote != 0 && *string.offset(i as isize) as libc::c_int != '"' as i32 {
            builtin_error(
                b"%s: first non-whitespace character is not `\"'\0" as *const u8
                    as *const libc::c_char,
                string,
            );
            return -(1 as libc::c_int);
        }

        delim = if *string.offset(i as isize) as libc::c_int == '"' as i32
            || *string.offset(i as isize) as libc::c_int == '\'' as i32
        {
            *string.offset(i as isize) as libc::c_int
        } else {
            0 as libc::c_int
        };
    }
    if !startp.is_null() {
        unsafe {
            *startp = if delim != 0 {
                i += 1;
                i
            } else {
                i
            };
        }
    }

    passc = 0 as libc::c_int;
    loop {
        c = unsafe { *string.offset(i as isize) as libc::c_int };
        if !(c != 0) {
            break;
        }
        if passc != 0 {
            passc = 0 as libc::c_int;
        } else if c == '\\' as i32 {
            passc += 1;
        } else if c == delim {
            break;
        }
        i += 1;
    }
    unsafe {
        if delim != 0 && *string.offset(i as isize) as libc::c_int != delim {
            builtin_error(
                b"no closing `%c' in %s\0" as *const u8 as *const libc::c_char,
                delim,
                string,
            );
            return -(1 as libc::c_int);
        }
    }
    return i;
}

#[no_mangle]
pub fn bind_keyseq_to_unix_command(line: *mut libc::c_char) -> libc::c_int {
    let kmap: Keymap;
    let cmd_xmap: Keymap;
    let kseq: *mut libc::c_char;
    let value: *mut libc::c_char;
    let mut i: libc::c_int;
    let mut kstart: libc::c_int = 0;

    kmap = c_rl_get_keymap();

    i = isolate_sequence(line, 0 as libc::c_int, 1 as libc::c_int, &mut kstart);
    if i < 0 {
        return -(1 as libc::c_int);
    }

    kseq = substring(line, kstart, i);

    while unsafe {
        *line.offset(i as isize) as libc::c_int != 0
            && *line.offset(i as isize) as libc::c_int != ':' as i32
    } {
        i += 1;
    }
    unsafe {
        if *line.offset(i as isize) as libc::c_int != ':' as i32 {
            builtin_error(
                b"%s: missing colon separator\0" as *const u8 as *const libc::c_char,
                line,
            );
            FREE!(kseq);
            return -(1 as libc::c_int);
        }
    }
    i = isolate_sequence(line, i + 1 as libc::c_int, 0 as libc::c_int, &mut kstart);
    if i < 0 {
        unsafe {
            FREE!(kseq);
        }
        return -(1 as libc::c_int);
    }

    value = substring(line, kstart, i);
    cmd_xmap = get_cmd_xmap_from_keymap(kmap);
    c_rl_generic_bind(ISMACR as libc::c_int, kseq, value, cmd_xmap);

    unsafe {
        c_rl_bind_keyseq_in_map(
            kseq,
            ::std::mem::transmute::<Option<fn() -> libc::c_int>, Option<rl_command_func_t>>(Some(
                ::std::mem::transmute::<
                    fn(libc::c_int, libc::c_int) -> libc::c_int,
                    fn() -> libc::c_int,
                >(bash_execute_unix_command),
            )),
            kmap,
        );
        libc::free(kseq as *mut c_void);
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub fn unbind_unix_command(kseq: *mut libc::c_char) -> libc::c_int {
    let cmd_xmap: Keymap;
    unsafe {
        cmd_xmap = get_cmd_xmap_from_keymap(c_rl_get_keymap());
        if c_rl_bind_keyseq_in_map(
            kseq,
            ::std::mem::transmute::<*mut c_void, Option<rl_command_func_t>>(0 as *mut c_void),
            cmd_xmap,
        ) != 0 as libc::c_int
        {
            builtin_error(
                b"`%s': cannot unbind in command keymap\0" as *const u8 as *const libc::c_char,
                kseq,
            );
            return 0 as libc::c_int;
        }

        return 1 as libc::c_int;
    }
}

#[no_mangle]
pub fn bash_directory_completion_matches(text: *const libc::c_char) -> *mut *mut libc::c_char {
    let m1: *mut *mut libc::c_char;
    let dfn: *mut libc::c_char;
    let qc: libc::c_int;

    qc = unsafe {
        if rl_dispatching != 0 {
            rl_completion_quote_character
        } else {
            0 as libc::c_int
        }
    };
    if unsafe { rl_dispatching != 0 && rl_completion_found_quote == 0 } {
        dfn = bash_dequote_filename(text as *mut libc::c_char, qc);
    } else {
        dfn = text as *mut libc::c_char;
    }
    m1 = c_rl_completion_matches(dfn, Some(c_rl_filename_completion_function));

    if dfn != text as *mut libc::c_char {
        unsafe {
            libc::free(dfn as *mut c_void);
        }
    }

    if unsafe { m1.is_null() || (*m1.offset(0 as libc::c_int as isize)).is_null() } {
        return m1;
    }
    bash_progcomp_ignore_filenames(m1);
    return m1;
}

#[no_mangle]
pub fn bash_dequote_text(text: *const libc::c_char) -> *mut libc::c_char {
    let dtxt: *mut libc::c_char;
    let qc: libc::c_int;
    unsafe {
        qc = if *text.offset(0 as isize) as libc::c_int == '"' as i32
            || *text.offset(0 as isize) as libc::c_int == '\'' as i32
        {
            *text.offset(0 as isize) as libc::c_int
        } else {
            0
        };
        dtxt = bash_dequote_filename(text as *mut libc::c_char, qc);
    }
    return dtxt;
}

fn bash_event_hook() -> libc::c_int {
    let sig: libc::c_int;
    unsafe {
        if sigterm_received != 0 {
            return 0;
        }
        // sig = 0;
        if terminating_signal != 0 {
            sig = terminating_signal;
        } else if interrupt_state != 0 {
            sig = SIGINT as libc::c_int;
        } else if sigalrm_seen != 0 {
            sig = SIGALRM as libc::c_int;
        } else {
            sig = first_pending_trap();
        }
        if terminating_signal != 0 || interrupt_state != 0 || sigalrm_seen != 0 {
            c_rl_cleanup_after_signal();
        }
        bashline_reset_event_hook();
        if posixly_correct != 0 && this_shell_builtin == Some(read_builtin) && sig == 2 {
            ::std::ptr::write_volatile(
                &mut last_command_exit_value as *mut libc::c_int,
                128 as libc::c_int | SIGINT as libc::c_int,
            );
            throw_to_top_level();
        }
        check_signals_and_traps();
    }
    return 0;
}
