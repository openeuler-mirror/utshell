use crate::src_common::*;

pub const ISKMAP: u32 = 1;
pub const ISMACR: u32 = 2;

pub const SINGLE_MATCH: u32 = 1;
pub const MULT_MATCH: u32 = 2;
pub type undo_code = u32;

pub type rl_dequote_func_t = ::core::option::Option<
    fn(
        arg1: *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char,
>;

pub type rl_icppfunc_t =
    ::core::option::Option<fn(arg1: *mut *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int>;

pub type rl_vintfunc_t = ::core::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>;

pub type KEYMAP_ENTRY_ARRAY = [KEYMAP_ENTRY; 257usize];

extern "C" {
    fn wcsdup(__s: *const wchar_t) -> *mut wchar_t;
    fn wcschr(__wcs: *const wchar_t, __wc: wchar_t) -> *mut ::std::os::raw::c_int;
    fn wcslen(__s: *const wchar_t) -> ::std::os::raw::c_ulong;
    fn wcsrtombs(
        __dst: *mut ::std::os::raw::c_char,
        __src: *mut *const wchar_t,
        __len: usize,
        __ps: *mut mbstate_t,
    ) -> usize;
    fn iswupper(__wc: wint_t) -> ::std::os::raw::c_int;
    fn towlower(__wc: wint_t) -> wint_t;

    #[link_name = "\u{1}environ"]
    pub static mut environ: *mut *mut ::std::os::raw::c_char;

    fn mbstowcs(__pwcs: *mut wchar_t, __s: *const ::std::os::raw::c_char, __n: usize) -> usize;
    fn xrealloc(arg1: *mut ::std::os::raw::c_void, arg2: usize) -> *mut ::std::os::raw::c_void;
    #[link_name = "\u{1}emacs_standard_keymap"]
    pub static mut emacs_standard_keymap: KEYMAP_ENTRY_ARRAY;
    #[link_name = "\u{1}emacs_meta_keymap"]
    pub static mut emacs_meta_keymap: KEYMAP_ENTRY_ARRAY;
    #[link_name = "\u{1}emacs_ctlx_keymap"]
    pub static mut emacs_ctlx_keymap: KEYMAP_ENTRY_ARRAY;
    #[link_name = "\u{1}vi_insertion_keymap"]
    pub static mut vi_insertion_keymap: KEYMAP_ENTRY_ARRAY;
    #[link_name = "\u{1}vi_movement_keymap"]
    pub static mut vi_movement_keymap: KEYMAP_ENTRY_ARRAY;
    fn rl_make_bare_keymap() -> Keymap;
    #[link_name = "\u{1}tilde_additional_prefixes"]
    pub static mut tilde_additional_prefixes: *mut *mut ::std::os::raw::c_char;
    #[link_name = "\u{1}tilde_additional_suffixes"]
    pub static mut tilde_additional_suffixes: *mut *mut ::std::os::raw::c_char;
    fn tilde_expand(arg1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    fn rl_forward_word(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_insert(arg1: ::std::os::raw::c_int, arg2: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
    fn rl_tab_insert(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_newline(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_transpose_words(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_transpose_chars(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_char_search(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_backward_char_search(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_beginning_of_history(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_end_of_history(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_get_next_history(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_get_previous_history(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_operate_and_get_next(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_set_mark(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_exchange_point_and_mark(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_vi_editing_mode(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_emacs_editing_mode(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_overwrite_mode(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_re_read_init_file(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_dump_functions(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_dump_macros(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_dump_variables(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_possible_completions(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_insert_completions(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_old_menu_complete(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_menu_complete(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_backward_menu_complete(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_kill_word(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_backward_kill_word(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_kill_line(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_backward_kill_line(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_kill_full_line(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_unix_word_rubout(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_unix_filename_rubout(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_unix_line_discard(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_copy_region_to_kill(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_kill_region(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_copy_forward_word(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_copy_backward_word(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_yank(arg1: ::std::os::raw::c_int, arg2: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    fn rl_yank_pop(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_yank_nth_arg(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_yank_last_arg(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_bracketed_paste_begin(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_reverse_search_history(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_forward_search_history(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_start_kbd_macro(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_end_kbd_macro(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_call_last_kbd_macro(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_print_last_kbd_macro(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_revert_line(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_undo_command(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_tilde_expand(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_restart_output(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_stop_output(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_abort(arg1: ::std::os::raw::c_int, arg2: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    fn rl_tty_status(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_history_search_forward(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_history_search_backward(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_history_substr_search_forward(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_history_substr_search_backward(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_noninc_forward_search(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_noninc_reverse_search(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_noninc_forward_search_again(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_noninc_reverse_search_again(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_insert_close(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_vi_end_word(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_vi_insertion_mode(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_vi_start_inserting(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    );
    fn rl_vi_bWord(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_initialize() -> ::std::os::raw::c_int;
    fn rl_add_defun(
        arg1: *const ::std::os::raw::c_char,
        arg2: Option<rl_command_func_t>,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn rl_bind_key_in_map(
        arg1: ::std::os::raw::c_int,
        arg2: Option<rl_command_func_t>,
        arg3: Keymap,
    ) -> ::std::os::raw::c_int;
    fn rl_unbind_key_in_map(arg1: ::std::os::raw::c_int, arg2: Keymap) -> ::std::os::raw::c_int;
    fn rl_bind_key_if_unbound_in_map(
        arg1: ::std::os::raw::c_int,
        arg2: Option<rl_command_func_t>,
        arg3: Keymap,
    ) -> ::std::os::raw::c_int;
    fn rl_bind_keyseq_in_map(
        arg1: *const ::std::os::raw::c_char,
        arg2: Option<rl_command_func_t>,
        arg3: Keymap,
    ) -> ::std::os::raw::c_int;
    fn rl_generic_bind(
        arg1: ::std::os::raw::c_int,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut ::std::os::raw::c_char,
        arg4: Keymap,
    ) -> ::std::os::raw::c_int;
    fn rl_variable_value(arg1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    fn rl_variable_bind(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    fn rl_function_of_keyseq(
        arg1: *const ::std::os::raw::c_char,
        arg2: Keymap,
        arg3: *mut ::std::os::raw::c_int,
    ) -> Option<rl_command_func_t>;

    fn rl_funmap_names() -> *mut *const ::std::os::raw::c_char;

    fn rl_push_macro_input(arg1: *mut ::std::os::raw::c_char);

    fn rl_add_undo(
        arg1: undo_code,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_char,
    );

    fn rl_begin_undo_group() -> ::std::os::raw::c_int;

    fn rl_end_undo_group() -> ::std::os::raw::c_int;

    fn rl_redisplay();

    fn rl_on_new_line() -> ::std::os::raw::c_int;

    fn rl_forced_update_display() -> ::std::os::raw::c_int;

    fn rl_clear_visible_line() -> ::std::os::raw::c_int;

    fn rl_crlf() -> ::std::os::raw::c_int;

    fn rl_redraw_prompt_last_line();

    fn rl_delete_text(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    fn rl_kill_text(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    fn rl_copy_text(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;

    fn rl_reset_terminal(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;

    fn rl_get_termcap(arg1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;

    fn rl_read_key() -> ::std::os::raw::c_int;

    fn rl_ding() -> ::std::os::raw::c_int;

    fn rl_set_signals() -> ::std::os::raw::c_int;

    fn rl_clear_signals() -> ::std::os::raw::c_int;

    fn rl_cleanup_after_signal();

    fn rl_completion_matches(
        arg1: *const ::std::os::raw::c_char,
        arg2: Option<rl_compentry_func_t>,
    ) -> *mut *mut ::std::os::raw::c_char;

    fn rl_username_completion_function(
        arg1: *const ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;

    fn rl_completion_mode(_: Option<rl_command_func_t>) -> libc::c_int;

    #[link_name = "\u{1}rl_readline_state"]
    pub static mut rl_readline_state: ::std::os::raw::c_ulong;

    #[link_name = "\u{1}rl_editing_mode"]
    pub static mut rl_editing_mode: ::std::os::raw::c_int;

    #[link_name = "\u{1}rl_readline_name"]
    pub static mut rl_readline_name: *const ::std::os::raw::c_char;

    #[link_name = "\u{1}rl_line_buffer"]
    pub static mut rl_line_buffer: *mut ::std::os::raw::c_char;

    #[link_name = "\u{1}rl_point"]
    pub static mut rl_point: ::std::os::raw::c_int;

    #[link_name = "\u{1}rl_end"]
    pub static mut rl_end: ::std::os::raw::c_int;

    #[link_name = "\u{1}rl_mark"]
    pub static mut rl_mark: ::std::os::raw::c_int;

    #[link_name = "\u{1}rl_explicit_arg"]
    pub static mut rl_explicit_arg: ::std::os::raw::c_int;

    #[link_name = "\u{1}rl_last_func"]
    pub static mut rl_last_func: Option<rl_command_func_t>;

    #[link_name = "\u{1}rl_terminal_name"]
    pub static mut rl_terminal_name: *const ::std::os::raw::c_char;

    #[link_name = "\u{1}rl_signal_event_hook"]
    pub static mut rl_signal_event_hook: Option<rl_hook_func_t>;

    #[link_name = "\u{1}rl_prep_term_function"]
    pub static mut rl_prep_term_function: rl_vintfunc_t;

    #[link_name = "\u{1}rl_deprep_term_function"]
    pub static mut rl_deprep_term_function: Option<rl_voidfunc_t>;

    #[link_name = "\u{1}rl_executing_keyseq"]
    pub static mut rl_executing_keyseq: *mut ::std::os::raw::c_char;

    #[link_name = "\u{1}rl_key_sequence_length"]
    pub static mut rl_key_sequence_length: ::std::os::raw::c_int;
    #[link_name = "\u{1}rl_menu_completion_entry_function"]
    pub static mut rl_menu_completion_entry_function: Option<rl_compentry_func_t>;

    #[link_name = "\u{1}rl_basic_word_break_characters"]
    pub static mut rl_basic_word_break_characters: *const ::std::os::raw::c_char;

    #[link_name = "\u{1}rl_completer_word_break_characters"]
    pub static mut rl_completer_word_break_characters: *mut ::std::os::raw::c_char;

    #[link_name = "\u{1}rl_completer_quote_characters"]
    pub static mut rl_completer_quote_characters: *const ::std::os::raw::c_char;

    #[link_name = "\u{1}rl_filename_quote_characters"]
    pub static mut rl_filename_quote_characters: *const ::std::os::raw::c_char;

    #[link_name = "\u{1}rl_special_prefixes"]
    pub static mut rl_special_prefixes: *const ::std::os::raw::c_char;

    #[link_name = "\u{1}rl_directory_completion_hook"]
    pub static mut rl_directory_completion_hook: rl_icppfunc_t;

    #[link_name = "\u{1}rl_directory_rewrite_hook"]
    pub static mut rl_directory_rewrite_hook: rl_icppfunc_t;

    #[link_name = "\u{1}rl_filename_stat_hook"]
    pub static mut rl_filename_stat_hook: rl_icppfunc_t;

    #[link_name = "\u{1}rl_filename_rewrite_hook"]
    pub static mut rl_filename_rewrite_hook: rl_dequote_func_t;

    #[link_name = "\u{1}rl_filename_completion_desired"]
    pub static mut rl_filename_completion_desired: ::std::os::raw::c_int;

    #[link_name = "\u{1}rl_filename_dequoting_function"]
    pub static mut rl_filename_dequoting_function: rl_dequote_func_t;

    #[link_name = "\u{1}rl_char_is_quoted_p"]
    pub static mut rl_char_is_quoted_p: Option<rl_linebuf_func_t>;

    #[link_name = "\u{1}rl_attempted_completion_over"]
    pub static mut rl_attempted_completion_over: ::std::os::raw::c_int;

    #[link_name = "\u{1}rl_completion_type"]
    pub static mut rl_completion_type: ::std::os::raw::c_int;

    #[link_name = "\u{1}rl_completion_invoking_key"]
    pub static mut rl_completion_invoking_key: ::std::os::raw::c_int;

    #[link_name = "\u{1}rl_completion_append_character"]
    pub static mut rl_completion_append_character: ::std::os::raw::c_int;

    #[link_name = "\u{1}rl_completion_suppress_append"]
    pub static mut rl_completion_suppress_append: ::std::os::raw::c_int;

    #[link_name = "\u{1}rl_completion_quote_character"]
    pub static mut rl_completion_quote_character: ::std::os::raw::c_int;

    #[link_name = "\u{1}rl_completion_found_quote"]
    pub static mut rl_completion_found_quote: ::std::os::raw::c_int;

    #[link_name = "\u{1}rl_completion_suppress_quote"]
    pub static mut rl_completion_suppress_quote: ::std::os::raw::c_int;

    #[link_name = "\u{1}rl_sort_completion_matches"]
    pub static mut rl_sort_completion_matches: ::std::os::raw::c_int;

    #[link_name = "\u{1}rl_completion_mark_symlink_dirs"]
    pub static mut rl_completion_mark_symlink_dirs: ::std::os::raw::c_int;

    fn _rl_find_next_mbchar(
        arg1: *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    fn tzset();

    fn stifle_history(arg1: ::std::os::raw::c_int);

    fn unstifle_history() -> ::std::os::raw::c_int;

    fn history_get_time(arg1: *mut HIST_ENTRY) -> time_t;

    fn read_history_range(
        arg1: *const ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    fn history_truncate_file(
        arg1: *const ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    fn history_tokenize(arg1: *const ::std::os::raw::c_char) -> *mut *mut ::std::os::raw::c_char;

    #[link_name = "\u{1}history_comment_char"]
    pub static mut history_comment_char: ::std::os::raw::c_char;

    #[link_name = "\u{1}history_write_timestamps"]
    pub static mut history_write_timestamps: ::std::os::raw::c_int;

    fn clearerr(__stream: *mut FILE);
    pub fn rl_filename_completion_function(
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut libc::c_char;
}

pub fn c_rl_filename_completion_function(
    a: *const libc::c_char,
    b: libc::c_int,
) -> *mut libc::c_char {
    unsafe { rl_filename_completion_function(a, b) }
}

pub fn c_wcsdup(__s: *const wchar_t) -> *mut wchar_t {
    unsafe { wcsdup(__s) }
}

pub fn c_wcschr(__wcs: *const wchar_t, __wc: wchar_t) -> *mut ::std::os::raw::c_int {
    unsafe { wcschr(__wcs, __wc) }
}

pub fn c_wcslen(__s: *const wchar_t) -> ::std::os::raw::c_ulong {
    unsafe { wcslen(__s) }
}

pub fn c_wcsrtombs(
    __dst: *mut ::std::os::raw::c_char,
    __src: *mut *const wchar_t,
    __len: usize,
    __ps: *mut mbstate_t,
) -> usize {
    unsafe { wcsrtombs(__dst, __src, __len, __ps) }
}

pub fn c_iswupper(__wc: wint_t) -> ::std::os::raw::c_int {
    unsafe { iswupper(__wc) }
}

pub fn c_towlower(__wc: wint_t) -> wint_t {
    unsafe { towlower(__wc) }
}

pub fn c_mbstowcs(__pwcs: *mut wchar_t, __s: *const ::std::os::raw::c_char, __n: usize) -> usize {
    unsafe { mbstowcs(__pwcs, __s, __n) }
}

pub fn c_xrealloc(arg1: *mut ::std::os::raw::c_void, arg2: usize) -> *mut ::std::os::raw::c_void {
    unsafe { xrealloc(arg1, arg2) }
}

pub fn c_rl_make_bare_keymap() -> Keymap {
    unsafe { rl_make_bare_keymap() }
}

pub fn c_tilde_expand(arg1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char {
    unsafe { tilde_expand(arg1) }
}

pub fn c_rl_forward_word(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_forward_word(arg1, arg2) }
}

pub fn c_rl_insert(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_insert(arg1, arg2) }
}

pub fn c_rl_tab_insert(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_tab_insert(arg1, arg2) }
}

pub fn c_rl_newline(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_newline(arg1, arg2) }
}

pub fn c_rl_transpose_words(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_transpose_words(arg1, arg2) }
}

pub fn c_rl_transpose_chars(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_transpose_chars(arg1, arg2) }
}

pub fn c_rl_char_search(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_char_search(arg1, arg2) }
}

pub fn c_rl_backward_char_search(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_backward_char_search(arg1, arg2) }
}

pub fn c_rl_beginning_of_history(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_beginning_of_history(arg1, arg2) }
}

pub fn c_rl_end_of_history(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_end_of_history(arg1, arg2) }
}

pub fn c_rl_get_next_history(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_get_next_history(arg1, arg2) }
}

pub fn c_rl_get_previous_history(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_get_previous_history(arg1, arg2) }
}

pub fn c_rl_operate_and_get_next(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_operate_and_get_next(arg1, arg2) }
}

pub fn c_rl_set_mark(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_set_mark(arg1, arg2) }
}

pub fn c_rl_exchange_point_and_mark(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_exchange_point_and_mark(arg1, arg2) }
}

pub fn c_rl_vi_editing_mode(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_vi_editing_mode(arg1, arg2) }
}

pub fn c_rl_emacs_editing_mode(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_emacs_editing_mode(arg1, arg2) }
}

pub fn c_rl_overwrite_mode(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_overwrite_mode(arg1, arg2) }
}

pub fn c_rl_re_read_init_file(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_re_read_init_file(arg1, arg2) }
}

pub fn c_rl_dump_functions(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_dump_functions(arg1, arg2) }
}

pub fn c_rl_dump_macros(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_dump_macros(arg1, arg2) }
}

pub fn c_rl_dump_variables(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_dump_variables(arg1, arg2) }
}

pub fn c_rl_possible_completions(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_possible_completions(arg1, arg2) }
}

pub fn c_rl_insert_completions(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_insert_completions(arg1, arg2) }
}

pub fn c_rl_old_menu_complete(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_old_menu_complete(arg1, arg2) }
}

pub fn c_rl_menu_complete(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_menu_complete(arg1, arg2) }
}

pub fn c_rl_backward_menu_complete(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_backward_menu_complete(arg1, arg2) }
}

pub fn c_rl_kill_word(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_kill_word(arg1, arg2) }
}

pub fn c_rl_backward_kill_word(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_backward_kill_word(arg1, arg2) }
}

pub fn c_rl_kill_line(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_kill_line(arg1, arg2) }
}

pub fn c_rl_backward_kill_line(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_backward_kill_line(arg1, arg2) }
}

pub fn c_rl_kill_full_line(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_kill_full_line(arg1, arg2) }
}

pub fn c_rl_unix_word_rubout(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_unix_word_rubout(arg1, arg2) }
}

pub fn c_rl_unix_filename_rubout(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_unix_filename_rubout(arg1, arg2) }
}

pub fn c_rl_unix_line_discard(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_unix_line_discard(arg1, arg2) }
}

pub fn c_rl_copy_region_to_kill(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_copy_region_to_kill(arg1, arg2) }
}

pub fn c_rl_kill_region(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_kill_region(arg1, arg2) }
}

pub fn c_rl_copy_forward_word(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_copy_forward_word(arg1, arg2) }
}

pub fn c_rl_copy_backward_word(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_copy_backward_word(arg1, arg2) }
}

pub fn c_rl_yank(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_yank(arg1, arg2) }
}

pub fn c_rl_yank_pop(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_yank_pop(arg1, arg2) }
}

pub fn c_rl_yank_nth_arg(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_yank_nth_arg(arg1, arg2) }
}

pub fn c_rl_yank_last_arg(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_yank_last_arg(arg1, arg2) }
}

pub fn c_rl_bracketed_paste_begin(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_bracketed_paste_begin(arg1, arg2) }
}

pub fn c_rl_reverse_search_history(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_reverse_search_history(arg1, arg2) }
}

pub fn c_rl_forward_search_history(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_forward_search_history(arg1, arg2) }
}

pub fn c_rl_start_kbd_macro(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_start_kbd_macro(arg1, arg2) }
}

pub fn c_rl_end_kbd_macro(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_end_kbd_macro(arg1, arg2) }
}

pub fn c_rl_call_last_kbd_macro(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_call_last_kbd_macro(arg1, arg2) }
}

pub fn c_rl_print_last_kbd_macro(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_print_last_kbd_macro(arg1, arg2) }
}

pub fn c_rl_revert_line(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_revert_line(arg1, arg2) }
}

pub fn c_rl_undo_command(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_undo_command(arg1, arg2) }
}

pub fn c_rl_tilde_expand(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_tilde_expand(arg1, arg2) }
}

pub fn c_rl_restart_output(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_restart_output(arg1, arg2) }
}

pub fn c_rl_stop_output(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_stop_output(arg1, arg2) }
}

pub fn c_rl_abort(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_abort(arg1, arg2) }
}

pub fn c_rl_tty_status(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_tty_status(arg1, arg2) }
}

pub fn c_rl_history_search_forward(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_history_search_forward(arg1, arg2) }
}

pub fn c_rl_history_search_backward(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_history_search_backward(arg1, arg2) }
}

pub fn c_rl_history_substr_search_forward(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_history_substr_search_forward(arg1, arg2) }
}

pub fn c_rl_history_substr_search_backward(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_history_substr_search_backward(arg1, arg2) }
}

pub fn c_rl_noninc_forward_search(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_noninc_forward_search(arg1, arg2) }
}

pub fn c_rl_noninc_reverse_search(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_noninc_reverse_search(arg1, arg2) }
}

pub fn c_rl_noninc_forward_search_again(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_noninc_forward_search_again(arg1, arg2) }
}

pub fn c_rl_noninc_reverse_search_again(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_noninc_reverse_search_again(arg1, arg2) }
}

pub fn c_rl_insert_close(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_insert_close(arg1, arg2) }
}

pub fn c_rl_vi_end_word(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_vi_end_word(arg1, arg2) }
}

pub fn c_rl_vi_insertion_mode(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_vi_insertion_mode(arg1, arg2) }
}

pub fn c_rl_vi_start_inserting(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
    arg3: ::std::os::raw::c_int,
) -> () {
    unsafe { rl_vi_start_inserting(arg1, arg2, arg3) }
}

pub fn c_rl_vi_bWord(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_vi_bWord(arg1, arg2) }
}

pub fn c_rl_initialize() -> ::std::os::raw::c_int {
    unsafe { rl_initialize() }
}

pub fn c_rl_add_defun(
    arg1: *const ::std::os::raw::c_char,
    arg2: Option<rl_command_func_t>,
    arg3: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_add_defun(arg1, arg2, arg3) }
}

pub fn c_rl_bind_key_in_map(
    arg1: ::std::os::raw::c_int,
    arg2: Option<rl_command_func_t>,
    arg3: Keymap,
) -> ::std::os::raw::c_int {
    unsafe { rl_bind_key_in_map(arg1, arg2, arg3) }
}

pub fn c_rl_unbind_key_in_map(arg1: ::std::os::raw::c_int, arg2: Keymap) -> ::std::os::raw::c_int {
    unsafe { rl_unbind_key_in_map(arg1, arg2) }
}

pub fn c_rl_bind_key_if_unbound_in_map(
    arg1: ::std::os::raw::c_int,
    arg2: Option<rl_command_func_t>,
    arg3: Keymap,
) -> ::std::os::raw::c_int {
    unsafe { rl_bind_key_if_unbound_in_map(arg1, arg2, arg3) }
}

pub fn c_rl_bind_keyseq_in_map(
    arg1: *const ::std::os::raw::c_char,
    arg2: Option<rl_command_func_t>,
    arg3: Keymap,
) -> ::std::os::raw::c_int {
    unsafe { rl_bind_keyseq_in_map(arg1, arg2, arg3) }
}

pub fn c_rl_generic_bind(
    arg1: ::std::os::raw::c_int,
    arg2: *const ::std::os::raw::c_char,
    arg3: *mut ::std::os::raw::c_char,
    arg4: Keymap,
) -> ::std::os::raw::c_int {
    unsafe { rl_generic_bind(arg1, arg2, arg3, arg4) }
}

pub fn c_rl_variable_value(arg1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char {
    unsafe { rl_variable_value(arg1) }
}

pub fn c_rl_variable_bind(
    arg1: *const ::std::os::raw::c_char,
    arg2: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    unsafe { rl_variable_bind(arg1, arg2) }
}

pub fn c_rl_function_of_keyseq(
    arg1: *const ::std::os::raw::c_char,
    arg2: Keymap,
    arg3: *mut ::std::os::raw::c_int,
) -> Option<rl_command_func_t> {
    unsafe { rl_function_of_keyseq(arg1, arg2, arg3) }
}

pub fn c_rl_funmap_names() -> *mut *const ::std::os::raw::c_char {
    unsafe { rl_funmap_names() }
}

pub fn c_rl_push_macro_input(arg1: *mut ::std::os::raw::c_char) -> () {
    unsafe { rl_push_macro_input(arg1) }
}

pub fn c_rl_add_undo(
    arg1: undo_code,
    arg2: ::std::os::raw::c_int,
    arg3: ::std::os::raw::c_int,
    arg4: *mut ::std::os::raw::c_char,
) -> () {
    unsafe { rl_add_undo(arg1, arg2, arg3, arg4) }
}

pub fn c_rl_begin_undo_group() -> ::std::os::raw::c_int {
    unsafe { rl_begin_undo_group() }
}

pub fn c_rl_end_undo_group() -> ::std::os::raw::c_int {
    unsafe { rl_end_undo_group() }
}

pub fn c_rl_redisplay() -> () {
    unsafe { rl_redisplay() }
}

pub fn c_rl_on_new_line() -> ::std::os::raw::c_int {
    unsafe { rl_on_new_line() }
}

pub fn c_rl_forced_update_display() -> ::std::os::raw::c_int {
    unsafe { rl_forced_update_display() }
}

pub fn c_rl_clear_visible_line() -> ::std::os::raw::c_int {
    unsafe { rl_clear_visible_line() }
}

pub fn c_rl_crlf() -> ::std::os::raw::c_int {
    unsafe { rl_crlf() }
}

pub fn c_rl_redraw_prompt_last_line() -> () {
    unsafe { rl_redraw_prompt_last_line() }
}

pub fn c_rl_delete_text(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_delete_text(arg1, arg2) }
}

pub fn c_rl_kill_text(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { rl_kill_text(arg1, arg2) }
}

pub fn c_rl_copy_text(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
) -> *mut ::std::os::raw::c_char {
    unsafe { rl_copy_text(arg1, arg2) }
}

pub fn c_rl_reset_terminal(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int {
    unsafe { rl_reset_terminal(arg1) }
}

pub fn c_rl_get_termcap(arg1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char {
    unsafe { rl_get_termcap(arg1) }
}

pub fn c_rl_read_key() -> ::std::os::raw::c_int {
    unsafe { rl_read_key() }
}

pub fn c_rl_ding() -> ::std::os::raw::c_int {
    unsafe { rl_ding() }
}

pub fn c_rl_set_signals() -> ::std::os::raw::c_int {
    unsafe { rl_set_signals() }
}

pub fn c_rl_clear_signals() -> ::std::os::raw::c_int {
    unsafe { rl_clear_signals() }
}

pub fn c_rl_cleanup_after_signal() -> () {
    unsafe { rl_cleanup_after_signal() }
}

pub fn c_rl_completion_matches(
    arg1: *const ::std::os::raw::c_char,
    arg2: Option<rl_compentry_func_t>,
) -> *mut *mut ::std::os::raw::c_char {
    unsafe { rl_completion_matches(arg1, arg2) }
}

pub fn c_rl_username_completion_function(
    arg1: *const ::std::os::raw::c_char,
    arg2: ::std::os::raw::c_int,
) -> *mut ::std::os::raw::c_char {
    unsafe { rl_username_completion_function(arg1, arg2) }
}

pub fn c_rl_completion_mode(a: Option<rl_command_func_t>) -> libc::c_int {
    unsafe { rl_completion_mode(a) }
}

pub fn c__rl_find_next_mbchar(
    arg1: *mut ::std::os::raw::c_char,
    arg2: ::std::os::raw::c_int,
    arg3: ::std::os::raw::c_int,
    arg4: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { _rl_find_next_mbchar(arg1, arg2, arg3, arg4) }
}

pub fn c_tzset() -> () {
    unsafe { tzset() }
}

pub fn c_stifle_history(arg1: ::std::os::raw::c_int) -> () {
    unsafe { stifle_history(arg1) }
}

pub fn c_unstifle_history() -> ::std::os::raw::c_int {
    unsafe { unstifle_history() }
}

pub fn c_history_get_time(arg1: *mut HIST_ENTRY) -> time_t {
    unsafe { history_get_time(arg1) }
}

pub fn c_read_history_range(
    arg1: *const ::std::os::raw::c_char,
    arg2: ::std::os::raw::c_int,
    arg3: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { read_history_range(arg1, arg2, arg3) }
}

pub fn c_history_truncate_file(
    arg1: *const ::std::os::raw::c_char,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { history_truncate_file(arg1, arg2) }
}

pub fn c_history_tokenize(arg1: *const ::std::os::raw::c_char) -> *mut *mut ::std::os::raw::c_char {
    unsafe { history_tokenize(arg1) }
}

pub fn c_clearerr(__stream: *mut FILE) -> () {
    unsafe { clearerr(__stream) }
}
