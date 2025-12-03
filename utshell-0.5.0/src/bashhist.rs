use crate::bashline::bash_re_edit;
use crate::general::file_exists;
use crate::pathexp::setup_ignore_patterns;
use crate::src_common::*;
use crate::stringlib::strcreplace;
use crate::subst::{skip_to_delim, skip_to_histexp};
use crate::variables::{
    get_string_value, set_if_not, sv_histchars, sv_histignore, sv_history_control, sv_histsize,
};
use crate::y_tab::{
    bash_input, current_command_line_count, dstack, history_delimiting_chars, parser_state,
};
use std::ffi::CStr;


fn member(c: i32, s: *const libc::c_char) -> bool {
    if c != 0 {
        return unsafe { mbschr(s, c) as libc::c_char != 0 as libc::c_char };
    } else {
        return false;
    }
}

static mut histignore: ignorevar = unsafe {
    {
        let mut init = ignorevar {
            varname: b"HISTIGNORE\0" as *const u8 as *mut libc::c_char,
            ignores: 0 as *mut ign,
            num_ignores: 0 as libc::c_int,
            last_ignoreval: 0 as *const libc::c_char as *mut libc::c_char,
            item_func: Some(::std::mem::transmute::<
                fn(*mut ign) -> libc::c_int,
                sh_iv_item_func_t,
            >(histignore_item_func)),
        };
        init
    }
};

fn bash_history_inhibit_expansion(
    mut string: *mut libc::c_char,
    mut i: libc::c_int,
) -> libc::c_int {
    let mut t: libc::c_int = 0;
    let mut si: libc::c_int = 0;
    let mut hx: [libc::c_char; 2] = [0; 2];
    hx[0 as usize] = unsafe { history_expansion_char };
    hx[1 as usize] = '\u{0}' as i32 as libc::c_char;
    unsafe {
        if i > 0 as libc::c_int
            && *string.offset((i - 1) as isize) as libc::c_int == '[' as i32
            && member(
                ']' as i32,
                string.offset(i as isize).offset(1 as libc::c_int as isize),
            )
        {
            return 1;
        } else if i > 1 as libc::c_int
            && *string.offset((i - 1 as libc::c_int) as isize) as libc::c_int == '{' as i32
            && *string.offset((i - 2 as libc::c_int) as isize) as libc::c_int == '$' as i32
            && member(
                '}' as i32,
                string.offset(i as isize).offset(1 as libc::c_int as isize),
            )
        {
            return 1;
        } else if i > 1 as libc::c_int
            && *string.offset((i - 1 as libc::c_int) as isize) as libc::c_int == '$' as i32
            && *string.offset(i as isize) as libc::c_int == '!' as i32
        {
            return 1;
        } else if extended_glob != 0
            && i > 1 as libc::c_int
            && *string.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '(' as i32
            && member(
                ')' as i32,
                string.offset(i as isize).offset(2 as libc::c_int as isize),
            )
        {
            return 1;
        }

        si = 0;
        if history_quoting_state == '\'' as i32 {
            si = skip_to_delim(
                string,
                0,
                b"'\0" as *const u8 as *mut libc::c_char,
                SD_NOJMP as libc::c_int | SD_HISTEXP as libc::c_int,
            );
            if *string.offset(si as isize) == 0 || si >= i {
                return 1;
            }
            si += 1;
        }
    }
    t = skip_to_histexp(
        string,
        si,
        hx.as_mut_ptr(),
        SD_NOJMP as libc::c_int | SD_HISTEXP as libc::c_int,
    );
    if t > 0 {
        while t < i {
            t = skip_to_histexp(
                string,
                t + 1 as libc::c_int,
                hx.as_mut_ptr(),
                SD_NOJMP as libc::c_int | SD_HISTEXP as libc::c_int,
            );
            if t <= 0 {
                return 0;
            }
        }
        return (t > i) as libc::c_int;
    } else {
        return 0;
    };
}

#[no_mangle]
pub fn bash_initialize_history() {
    unsafe {
        history_quotes_inhibit_expansion = 1;
        history_search_delimiter_chars =
            b";&()|<>\0" as *const u8 as *const libc::c_char as *mut libc::c_char;

        history_inhibit_expansion_function = std::mem::transmute::<
            fn(*mut libc::c_char, libc::c_int) -> libc::c_int,
            Option<rl_linebuf_func_t>,
        >(bash_history_inhibit_expansion);

        sv_histchars(b"histchars\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    }
}

#[no_mangle]
pub fn bash_history_reinit(mut interact: libc::c_int) {
    unsafe {
        history_expansion = if interact == 0 {
            histexp_flag
        } else {
            HISTEXPAND_DEFAULT as libc::c_int
        };

        history_expansion_inhibited = if interact == 0 { 1 - histexp_flag } else { 0 };
        history_inhibit_expansion_function = std::mem::transmute::<
            fn(*mut libc::c_char, libc::c_int) -> libc::c_int,
            Option<rl_linebuf_func_t>,
        >(bash_history_inhibit_expansion);
        remember_on_history = enable_history_list;
    }
}

#[no_mangle]
pub fn bash_history_disable() {
    unsafe {
        remember_on_history = 0;
        history_expansion_inhibited = 1;
    }
}

#[no_mangle]
pub fn bash_history_enable() {
    unsafe {
        remember_on_history = 1;
        enable_history_list = 1;
        history_expansion_inhibited = 0;
        history_inhibit_expansion_function = std::mem::transmute::<
            fn(*mut libc::c_char, libc::c_int) -> libc::c_int,
            Option<rl_linebuf_func_t>,
        >(bash_history_inhibit_expansion);
        sv_history_control(b"HISTCONTROL\0" as *const u8 as *mut libc::c_char);
        sv_histignore(b"HISTIGNORE\0" as *const u8 as *mut libc::c_char);
    }
}

#[no_mangle]
pub fn load_history() {
    let mut hf: *mut libc::c_char;
    unsafe {
        set_if_not(
            b"HISTSIZE\0" as *const u8 as *mut libc::c_char,
            HISTSIZE_DEFAULT!(),
        );
        sv_histsize(b"HISTSIZE\0" as *const u8 as *mut libc::c_char);

        set_if_not(
            b"HISTFILESIZE\0" as *const u8 as *mut libc::c_char,
            get_string_value(b"HISTSIZE\0" as *const u8 as *mut libc::c_char),
        );
        sv_histsize(b"HISTFILESIZE\0" as *const u8 as *mut libc::c_char);

        hf = get_string_value(b"HISTFILE\0" as *const u8 as *mut libc::c_char);

        if !hf.is_null() && *hf as libc::c_int != 0 && file_exists(hf) != 0 {
            read_history(hf);
            history_lines_in_file = history_lines_read_from_file;
            using_history();
        }
    }
}

#[no_mangle]
pub fn bash_clear_history() {
    unsafe {
        clear_history();
        history_lines_this_session = 0;
    }
}

#[no_mangle]
pub fn bash_delete_histent(mut i: libc::c_int) -> libc::c_int {
    let mut discard: *mut HIST_ENTRY = 0 as *mut HIST_ENTRY;
    unsafe {
        discard = remove_history(i);
        if !discard.is_null() {
            free_history_entry(discard);
            history_lines_this_session -= 1;
        }
    }
    return (discard != 0 as *mut HIST_ENTRY) as libc::c_int;
}

#[no_mangle]
pub fn bash_delete_history_range(mut first: libc::c_int, mut last: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut discard_list: *mut *mut HIST_ENTRY = 0 as *mut *mut HIST_ENTRY;
    unsafe {
        discard_list = remove_history_range(first, last);
        i = 0 as libc::c_int;
        while !discard_list.is_null() && !(*discard_list.offset(i as isize)).is_null() {
            free_history_entry(*discard_list.offset(i as isize));
            i += 1;
        }
        history_lines_this_session -= i;
    }
    return 1 as libc::c_int;
}

#[no_mangle]
pub fn bash_delete_last_history() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut hlist: *mut *mut HIST_ENTRY = 0 as *mut *mut HIST_ENTRY;
    let mut histent: *mut HIST_ENTRY = 0 as *mut HIST_ENTRY;
    let mut r: libc::c_int = 0;
    unsafe {
        hlist = history_list();
        if hlist.is_null() {
            return 0;
        }

        i = 0;
        while !(*hlist.offset(i as isize)).is_null() {
            i += 1;
        }

        i -= 1;
        histent = history_get(history_base + i);
        if histent.is_null() {
            return 0;
        }

        r = bash_delete_histent(i);
        if where_history() > history_length {
            history_set_pos(history_length);
        }
    }
    return r;
}

#[no_mangle]
pub fn maybe_append_history(mut filename: *mut libc::c_char) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    let mut histlen: libc::c_int = 0;
    let mut buf: crate::src_common::stat = crate::src_common::stat {
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
        st_atim: crate::src_common::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: crate::src_common::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: crate::src_common::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    result = EXECUTION_SUCCESS as i32;
    unsafe {
        if history_lines_this_session > 0 {
            if stat(filename, &mut buf) == -1 && errno!() == ENOENT!() {
                fd = libc::open(
                    filename,
                    O_WRONLY as i32 | O_CREAT as i32,
                    0o600 as libc::c_int,
                );
                if fd < 0 as libc::c_int {
                    builtin_error(
                        b"%s: cannot create: %s\0" as *const u8 as *const libc::c_char,
                        filename,
                        libc::strerror(errno!()),
                    );
                    return 1 as libc::c_int;
                }
                libc::close(fd);
            }
            histlen = where_history();
            if histlen > 0 && history_lines_this_session > histlen {
                history_lines_this_session = histlen;
            }
            result = append_history(history_lines_this_session, filename);
            history_lines_in_file += history_lines_this_session;
            history_lines_this_session = 0;
        } else {
            history_lines_this_session = 0;
        }
    }
    return result;
}

#[no_mangle]
pub fn maybe_save_shell_history() -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut hf: *mut libc::c_char = 0 as *mut libc::c_char;
    result = 0;
    unsafe {
        if history_lines_this_session > 0 {
            hf = get_string_value(b"HISTFILE\0" as *const u8 as *const libc::c_char);
            if !hf.is_null() && *hf as libc::c_int != 0 {
                if file_exists(hf) == 0 {
                    let mut file: libc::c_int = 0;
                    file = libc::open(
                        hf,
                        O_CREAT as libc::c_int | O_TRUNC as libc::c_int | O_WRONLY as libc::c_int,
                        0o600 as libc::c_int,
                    );
                    if file != -1 {
                        libc::close(file);
                    }
                }
                using_history();
                if history_lines_this_session <= where_history() || force_append_history != 0 {
                    result = append_history(history_lines_this_session, hf);
                    history_lines_in_file += history_lines_this_session;
                } else {
                    result = write_history(hf);
                    history_lines_in_file = history_lines_written_to_file;
                }
                history_lines_this_session = 0;
                sv_histsize(b"HISTFILESIZE\0" as *const u8 as *mut libc::c_char);
            }
        }
    }
    return result;
}

fn re_edit(mut text: *mut libc::c_char) {
    unsafe {
        if bash_input.type_0 as libc::c_uint == st_stdin as libc::c_int as libc::c_uint {
            bash_re_edit(text);
        }
    }
}

fn history_expansion_p(mut line: *mut libc::c_char) -> libc::c_int {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = line;
    unsafe {
        while *s != 0 {
            if *s as libc::c_int == history_expansion_char as libc::c_int
                || *s as libc::c_int == history_subst_char as libc::c_int
            {
                return 1;
            }
            s = s.offset(1);
        }
    }
    return 0;
}

#[no_mangle]
pub fn pre_process_line(
    mut line: *mut libc::c_char,
    mut print_changes: libc::c_int,
    mut addit: libc::c_int,
) -> *mut libc::c_char {
    let mut history_value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut return_value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut expanded: libc::c_int = 0;
    return_value = line;
    expanded = 0;
    unsafe {
        if history_expansion_inhibited == 0
            && history_expansion != 0
            && history_expansion_p(line) != 0
        {
            let mut old_len: libc::c_int = 0;
            old_len = history_length;
            if history_length > 0
                && command_oriented_history != 0
                && current_command_first_line_saved != 0
                && current_command_line_count > 1
            {
                history_length -= 1;
            }
            expanded = history_expand(line, &mut history_value);
            if history_length >= 0
                && command_oriented_history != 0
                && current_command_first_line_saved != 0
                && current_command_line_count > 1
            {
                history_length = old_len;
            }
            if expanded != 0 {
                if print_changes != 0 {
                    if expanded < 0 {
                        internal_error(b"%s\0" as *const u8 as *const libc::c_char, history_value);
                    } else if hist_verify == 0 || expanded == 2 {
                        println!("{}", CStr::from_ptr(history_value).to_str().unwrap())
                    }
                }
                if expanded < 0 || expanded == 2 {
                    if expanded == 2 as libc::c_int
                        && rl_dispatching == 0
                        && *history_value as libc::c_int != 0
                    {
                        maybe_add_history(history_value);
                    }
                    libc::free(history_value as *mut libc::c_void);
                    if history_reediting != 0 && expanded < 0 && rl_done != 0 {
                        re_edit(line);
                    }
                    return 0 as *mut libc::c_char;
                }
                if hist_verify != 0 && expanded == 1 {
                    re_edit(history_value);
                    libc::free(history_value as *mut libc::c_void);
                    return 0 as *mut libc::c_char;
                }
            }
            expanded = 1;
            return_value = history_value;
        }
        if addit != 0 && remember_on_history != 0 && *return_value as libc::c_int != 0 {
            maybe_add_history(return_value);
        }
    }
    return return_value;
}

fn shell_comment(mut line: *mut libc::c_char) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0;
    if line.is_null() {
        return 0;
    }
    unsafe {
        p = line;
        while !p.is_null() && *p as libc::c_int != 0 && whitespace!(*p) {
            p = p.offset(1);
        }
        if !p.is_null() && *p as libc::c_int == '#' as i32 {
            return 1;
        }
        n = skip_to_delim(
            line,
            p.offset_from(line) as libc::c_int,
            b"#\0" as *const u8 as *mut libc::c_char,
            SD_NOJMP as libc::c_int
                | SD_GLOB as libc::c_int
                | SD_EXTGLOB as libc::c_int
                | SD_COMPLETE as libc::c_int,
        );
        return if *line.offset(n as isize) as libc::c_int == '#' as i32 {
            2
        } else {
            0
        };
    }
}

fn check_history_control(mut line: *mut libc::c_char) -> libc::c_int {
    let mut temp: *mut HIST_ENTRY = 0 as *mut HIST_ENTRY;
    let mut r: libc::c_int = 0;
    unsafe {
        if history_control == 0 {
            return 1;
        }
        if history_control & HC_IGNSPACE as libc::c_int != 0 && *line as libc::c_int == ' ' as i32 {
            return 0;
        }
        if history_control & HC_IGNDUPS as libc::c_int != 0 {
            using_history();
            temp = previous_history();
            r = (temp.is_null() || STREQ!((*temp).line, line) == false) as libc::c_int;
            using_history();
            if r == 0 {
                return r;
            }
        }
    }
    return 1;
}

fn hc_erasedups(mut line: *mut libc::c_char) {
    let mut temp: *mut HIST_ENTRY = 0 as *mut HIST_ENTRY;
    let mut r: libc::c_int = 0;
    unsafe {
        using_history();
        loop {
            temp = previous_history();
            if temp.is_null() {
                break;
            }
            if STREQ!((*temp).line, line) {
                r = where_history();
                temp = remove_history(r);
                if !temp.is_null() {
                    free_history_entry(temp);
                }
            }
        }
        using_history();
    }
}

#[no_mangle]
pub fn maybe_add_history(mut line: *mut libc::c_char) {
    let mut is_comment: libc::c_int = 0;
    unsafe {
        hist_last_line_added = 0;
        is_comment = if parser_state & PST_HEREDOC as libc::c_int != 0 {
            0
        } else {
            shell_comment(line)
        };
        if current_command_line_count > 1 {
            if current_command_first_line_saved != 0
                && (parser_state & PST_HEREDOC as libc::c_int != 0
                    || literal_history != 0
                    || dstack.delimiter_depth != 0
                    || is_comment != 1)
            {
                bash_add_history(line);
            }
            current_command_line_comment = if is_comment != 0 {
                current_command_line_count
            } else {
                -(2 as libc::c_int)
            };
            return;
        }
        current_command_line_comment = if is_comment != 0 {
            current_command_line_count
        } else {
            -(2 as libc::c_int)
        };
        current_command_first_line_saved = check_add_history(line, 0);
    }
}

#[no_mangle]
pub fn check_add_history(mut line: *mut libc::c_char, mut force: libc::c_int) -> libc::c_int {
    unsafe {
        if check_history_control(line) != 0 && history_should_ignore(line) == 0 {
            if history_control & HC_ERASEDUPS as libc::c_int != 0 {
                hc_erasedups(line);
            }
            if force != 0 {
                really_add_history(line);
                using_history();
            } else {
                bash_add_history(line);
            }
            return 1;
        }
    }
    return 0;
}

#[no_mangle]
pub fn bash_add_history(mut line: *mut libc::c_char) {
    let mut add_it: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut curlen: libc::c_int = 0;
    let mut is_comment: libc::c_int = 0;
    let mut current: *mut HIST_ENTRY = 0 as *mut HIST_ENTRY;
    let mut old: *mut HIST_ENTRY = 0 as *mut HIST_ENTRY;
    let mut chars_to_add: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_line: *mut libc::c_char = 0 as *mut libc::c_char;

    add_it = 1;
    unsafe {
        if command_oriented_history != 0 && current_command_line_count > 1 {
            is_comment = if parser_state & PST_HEREDOC as libc::c_int != 0 {
                0
            } else {
                shell_comment(line)
            };
            if parser_state & PST_HEREDOC as libc::c_int != 0
                && current_command_line_count > 2
                && *line.offset(((libc::strlen(line)) - 1) as isize) as libc::c_int == '\n' as i32
            {
                chars_to_add = b"\0" as *const u8 as *mut libc::c_char;
            } else if current_command_line_count == current_command_line_comment + 1 {
                chars_to_add = b"\n\0" as *const u8 as *mut libc::c_char;
            } else if literal_history != 0 {
                chars_to_add = b"\n\0" as *const u8 as *mut libc::c_char;
            } else {
                chars_to_add = history_delimiting_chars(line);
            }
            using_history();
            current = previous_history();
            current_command_line_comment = if is_comment != 0 {
                current_command_line_count
            } else {
                -(2 as libc::c_int)
            };
            if !current.is_null() {
                curlen = libc::strlen((*current).line) as libc::c_int;
                if dstack.delimiter_depth == 0
                    && *((*current).line).offset((curlen - 1) as isize) as libc::c_int
                        == '\\' as i32
                    && *((*current).line).offset((curlen - 2) as isize) as libc::c_int
                        != '\\' as i32
                {
                    *((*current).line).offset((curlen - 1) as isize) =
                        '\u{0}' as i32 as libc::c_char;
                    curlen -= 1;
                    chars_to_add = b"\0" as *const u8 as *mut libc::c_char;
                }
                if dstack.delimiter_depth == 0
                    && *((*current).line).offset((curlen - 1) as isize) as libc::c_int
                        == '\n' as i32
                    && *chars_to_add as libc::c_int == ';' as i32
                {
                    chars_to_add = chars_to_add.offset(1);
                }

                new_line = libc::malloc(
                    (1 + curlen + libc::strlen(line) as i32 + libc::strlen(chars_to_add) as i32)
                        as usize,
                ) as *mut libc::c_char;
                libc::sprintf(
                    new_line,
                    b"%s%s%s\0" as *const u8 as *const libc::c_char,
                    (*current).line,
                    chars_to_add,
                    line,
                );
                offset = where_history();
                old = replace_history_entry(offset, new_line, (*current).data);

                libc::free(new_line as *mut libc::c_void);
                if !old.is_null() {
                    free_history_entry(old);
                }
                add_it = 0 as libc::c_int;
            }
        }
        if add_it != 0
            && history_is_stifled() != 0
            && history_length == 0 as libc::c_int
            && history_length == history_max_entries
        {
            add_it = 0 as libc::c_int;
        }
        if add_it != 0 {
            really_add_history(line);
        }
        using_history();
    }
}

fn really_add_history(mut line: *mut libc::c_char) {
    unsafe {
        hist_last_line_added = 1;
        hist_last_line_pushed = 0;
        add_history(line);
        history_lines_this_session += 1;
    }
}

#[no_mangle]
pub fn history_number() -> libc::c_int {
    unsafe {
        using_history();
        return if remember_on_history != 0 || enable_history_list != 0 {
            history_base + where_history()
        } else {
            1
        };
    }
}

fn should_expand(mut s: *mut libc::c_char) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = s;
    unsafe {
        while !p.is_null() && *p as libc::c_int != 0 {
            if *p as libc::c_int == '\\' as i32 {
                p = p.offset(1);
            } else if *p as libc::c_int == '&' as i32 {
                return 1 as libc::c_int;
            }
            p = p.offset(1);
        }
    }
    return 0;
}

fn histignore_item_func(mut ign: *mut ign) -> libc::c_int {
    unsafe {
        if should_expand((*ign).val) != 0 {
            (*ign).flags |= HIGN_EXPAND!();
        }
    }
    return 0;
}

#[no_mangle]
pub fn setup_history_ignore(mut varname: *mut libc::c_char) {
    unsafe {
        setup_ignore_patterns(&mut histignore);
    }
}

fn last_history_entry() -> *mut HIST_ENTRY {
    let mut he: *mut HIST_ENTRY = 0 as *mut HIST_ENTRY;
    unsafe {
        using_history();
        he = previous_history();
        using_history();
    }
    return he;
}

#[no_mangle]
pub fn last_history_line() -> *mut libc::c_char {
    let mut he: *mut HIST_ENTRY = 0 as *mut HIST_ENTRY;

    he = last_history_entry();
    if he.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    return unsafe { (*he).line };
}

fn expand_histignore_pattern(mut pat: *mut libc::c_char) -> *mut libc::c_char {
    let mut phe: *mut HIST_ENTRY = 0 as *mut HIST_ENTRY;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;

    phe = last_history_entry();
    unsafe {
        if phe.is_null() {
            return savestring!(pat);
        }
        ret = strcreplace(pat, '&' as i32, (*phe).line, 1);
    }
    return ret;
}

fn history_should_ignore(mut line: *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut match_0: libc::c_int = 0;
    let mut npat: *mut libc::c_char = 0 as *mut libc::c_char;
    unsafe {
        if histignore.num_ignores == 0 {
            return 0;
        }
        match_0 = 0;
        i = match_0;
        while i < histignore.num_ignores {
            if (*(histignore.ignores).offset(i as isize)).flags & HIGN_EXPAND!() != 0 {
                npat = expand_histignore_pattern((*(histignore.ignores).offset(i as isize)).val);
            } else {
                npat = (*(histignore.ignores).offset(i as isize)).val;
            }
            match_0 = (strmatch(npat, line, FNMATCH_EXTFLAG!()) != FNM_NOMATCH!() as libc::c_int)
                as libc::c_int;

            if (*(histignore.ignores).offset(i as isize)).flags & HIGN_EXPAND!() != 0 {
                libc::free(npat as *mut libc::c_void);
            }
            if match_0 != 0 {
                break;
            }
            i += 1;
        }
    }
    return match_0;
}
