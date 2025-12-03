/*
 * SPDX-FileCopyrightText: 2025 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: GPL-2.0-or-later
 */
use crate::array::array_reference;
use crate::builtins::common::builtin_address_internal;
use crate::dispose_cmd::{dispose_word, dispose_words};
use crate::error::command_error;
use crate::general::{all_digits, assignment, legal_number};
use crate::list::list_reverse;
use crate::src_common::*;
use crate::stringlib::substring;
use crate::subst::{skip_to_delim, string_quote_removal};
use crate::variables::{bind_function_def, find_function};
use crate::y_tab::{line_number, parser_state, read_secondary_line};

#[no_mangle]
pub fn cmd_init() {
    unsafe {
        ocache_create!(wdcache, WordDesc, WDCACHESIZE!());
        ocache_create!(wlcache, WordList, WLCACHESIZE!());
    }
}

#[no_mangle]
pub fn alloc_word_desc() -> *mut WordDesc {
    let mut temp: *mut WordDesc = 0 as *mut WordDesc;
    unsafe {
        ocache_alloc!(wdcache, WordDesc, temp);
        (*temp).flags = 0;
        (*temp).word = 0 as *mut libc::c_char;
    }
    return temp;
}

#[no_mangle]
pub fn make_bare_word(string: *const libc::c_char) -> *mut WordDesc {
    let mut temp: *mut WordDesc = 0 as *mut WordDesc;
    temp = alloc_word_desc();
    unsafe {
        if *string != 0 {
            (*temp).word = savestring!(string);
        } else {
            (*temp).word = malloc(1) as *mut libc::c_char;
            *((*temp).word).offset(0 as isize) = '\u{0}' as i32 as libc::c_char;
        }
    }
    return temp;
}

#[inline]
fn is_basic(c: libc::c_char) -> libc::c_int {
    unsafe {
        return (*is_basic_table
            .as_ptr()
            .offset((c as libc::c_uchar as libc::c_int >> 5 as libc::c_int) as isize)
            >> (c as libc::c_uchar as libc::c_int & 31 as libc::c_int)
            & 1 as libc::c_int as libc::c_uint) as libc::c_int;
    }
}

#[no_mangle]
pub fn make_word_flags(w: *mut WordDesc, string: *const libc::c_char) -> *mut WordDesc {
    let mut i: libc::c_int = 0;
    let mut slen: size_t = 0;
    // DECLARE_MBSTATE!();
    let mut state: mbstate_t = mbstate_t {
        __count: 0,
        __value: mbstate_t_value { __wch: 0 },
    };
    unsafe {
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            '\u{0}' as i32,
            std::mem::size_of::<mbstate_t>() as usize,
        );

        i = 0;
        slen = strlen(string) as size_t;
        while (i as libc::c_ulong) < slen {
            let opt = *string.offset(i as isize);
            let opt_char = char::from(opt as u8);

            match opt_char {
                '$' => {
                    (*w).flags |= W_HASDOLLAR as libc::c_int;
                }
                '\'' | '`' | '"' => {
                    (*w).flags |= W_QUOTED as libc::c_int;
                }
                '\\' | _ => {}
            }
            ADVANCE_CHAR!(string, slen, i, state);
        }

        return w;
    }
}

#[no_mangle]
pub fn make_word(string: *const libc::c_char) -> *mut WordDesc {
    unsafe {
        let mut temp: *mut WordDesc = 0 as *mut WordDesc;
        temp = make_bare_word(string);
        return make_word_flags(temp, string);
    }
}

#[no_mangle]
pub fn make_word_from_token(token: libc::c_int) -> *mut WordDesc {
    let mut tokenizer: [libc::c_char; 2] = [0; 2];

    tokenizer[0 as usize] = token as libc::c_char;
    tokenizer[1 as usize] = '\u{0}' as i32 as libc::c_char;
    return make_word(tokenizer.as_mut_ptr());
}

#[no_mangle]
pub fn make_word_list(word: *mut WordDesc, wlink: *mut WordList) -> *mut WordList {
    let mut temp: *mut WordList = 0 as *mut WordList;
    unsafe {
        ocache_alloc!(wlcache, WordList, temp);

        (*temp).word = word;
        (*temp).next = wlink;
    }
    return temp;
}

#[no_mangle]
pub fn make_command(type_0: command_type, pointer: *mut SIMPLE_COM) -> *mut COMMAND {
    let mut temp: *mut COMMAND = 0 as *mut COMMAND;
    unsafe {
        temp = malloc(std::mem::size_of::<COMMAND>() as usize) as *mut COMMAND;
        (*temp).type_0 = type_0;
        (*temp).value.Simple = pointer;
        (*temp).flags = 0;
        (*(*temp).value.Simple).flags = 0;
        (*temp).redirects = 0 as *mut REDIRECT;
    }
    return temp;
}

#[no_mangle]
pub fn command_connect(
    com1: *mut COMMAND,
    com2: *mut COMMAND,
    connector: libc::c_int,
) -> *mut COMMAND {
    let mut temp: *mut CONNECTION = 0 as *mut CONNECTION;
    unsafe {
        temp = malloc(std::mem::size_of::<CONNECTION>() as usize) as *mut CONNECTION;
        (*temp).connector = connector;
        (*temp).first = com1;
        (*temp).second = com2;
    }
    return make_command(command_type_cm_connection, temp as *mut SIMPLE_COM);
}

fn make_for_or_select(
    type_0: command_type,
    name: *mut WordDesc,
    map_list: *mut WordList,
    action: *mut COMMAND,
    lineno: libc::c_int,
) -> *mut COMMAND {
    let mut temp: *mut FOR_COM = 0 as *mut FOR_COM;
    unsafe {
        temp = malloc(std::mem::size_of::<FOR_COM>() as usize) as *mut FOR_COM;
        (*temp).flags = 0;
        (*temp).name = name;
        (*temp).line = lineno;
        (*temp).map_list = map_list;
        (*temp).action = action;
    }
    return make_command(type_0 as libc::c_uint, temp as *mut SIMPLE_COM);
}

#[no_mangle]
pub fn make_for_command(
    name: *mut WordDesc,
    map_list: *mut WordList,
    action: *mut COMMAND,
    lineno: libc::c_int,
) -> *mut COMMAND {
    return make_for_or_select(command_type_cm_for, name, map_list, action, lineno);
}
#[no_mangle]
pub fn make_select_command(
    name: *mut WordDesc,
    map_list: *mut WordList,
    action: *mut COMMAND,
    lineno: libc::c_int,
) -> *mut COMMAND {
    return make_for_or_select(command_type_cm_select, name, map_list, action, lineno);
}

fn make_arith_for_expr(s: *mut libc::c_char) -> *mut WordList {
    let mut result: *mut WordList = 0 as *mut WordList;
    let mut wd: *mut WordDesc = 0 as *mut WordDesc;
    unsafe {
        if s.is_null() || *s as libc::c_int == '\u{0}' as i32 {
            return 0 as *mut WordList;
        }
        wd = make_word(s);
        (*wd).flags |= W_NOGLOB!() | W_NOSPLIT!() | W_QUOTED!() | W_DQUOTE!();
        (*wd).flags |= W_NOPROCSUB!();
        result = make_word_list(wd, 0 as *mut WordList);
    }
    return result;
}

#[no_mangle]
pub fn make_arith_for_command(
    exprs: *mut WordList,
    action: *mut COMMAND,
    lineno: libc::c_int,
) -> *mut COMMAND {
    let mut temp: *mut ARITH_FOR_COM = 0 as *mut ARITH_FOR_COM;
    let mut init: *mut WordList = 0 as *mut WordList;
    let mut test: *mut WordList = 0 as *mut WordList;
    let mut step: *mut WordList = 0 as *mut WordList;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nsemi: libc::c_int = 0;
    let mut i: libc::c_int = 0;

    step = 0 as *mut WordList;
    test = step;
    init = test;
    unsafe {
        s = (*(*exprs).word).word;
        t = s;
        start = t;
        nsemi = 0;
        loop {
            while whitespace!(*s) {
                s = s.offset(1);
            }
            start = s;
            i = skip_to_delim(
                start,
                0,
                b";\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                SD_NOJMP as libc::c_int | SD_NOPROCSUB as libc::c_int,
            );
            s = start.offset(i as isize);

            t = if i > 0 as libc::c_int {
                substring(start, 0, i)
            } else {
                0 as *mut libc::c_char
            };

            nsemi += 1;
            match nsemi {
                1 => {
                    init = make_arith_for_expr(t);
                }
                2 => {
                    test = make_arith_for_expr(t);
                }
                3 => {
                    step = make_arith_for_expr(t);
                }
                _ => {}
            }
            FREE!(t);
            if *s as libc::c_int == '\u{0}' as i32 {
                break;
            }
            s = s.offset(1);
        }
        if nsemi != 3 {
            if nsemi < 3 {
                parser_error(
                    lineno,
                    b"syntax error: arithmetic expression required\0" as *const u8
                        as *mut libc::c_char,
                );
            } else {
                parser_error(
                    lineno,
                    b"syntax error: `;' unexpected\0" as *const u8 as *const libc::c_char,
                );
            }
            parser_error(
                lineno,
                b"syntax error: `((%s))'\0" as *const u8 as *const libc::c_char,
                (*(*exprs).word).word,
            );
            free(init as *mut c_void);
            free(test as *mut c_void);
            free(step as *mut c_void);
            set_exit_status(2);
            return 0 as *mut COMMAND;
        }

        temp = malloc(std::mem::size_of::<ARITH_FOR_COM>() as usize) as *mut ARITH_FOR_COM;
        (*temp).flags = 0;
        (*temp).line = lineno;
        (*temp).init = if !init.is_null() {
            init
        } else {
            make_arith_for_expr(b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
        };
        (*temp).test = if !test.is_null() {
            test
        } else {
            make_arith_for_expr(b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
        };
        (*temp).step = if !step.is_null() {
            step
        } else {
            make_arith_for_expr(b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
        };
        (*temp).action = action;
        dispose_words(exprs);
    }
    return make_command(command_type_cm_arith_for, temp as *mut SIMPLE_COM);
}

#[no_mangle]
pub fn make_group_command(command: *mut COMMAND) -> *mut COMMAND {
    unsafe {
        let mut temp: *mut GROUP_COM = 0 as *mut GROUP_COM;
        temp = malloc(std::mem::size_of::<GROUP_COM>() as usize) as *mut GROUP_COM;

        (*temp).command = command;

        return make_command(command_type_cm_group, temp as *mut SIMPLE_COM);
    }
}

#[no_mangle]
pub fn make_case_command(
    word: *mut WordDesc,
    clauses: *mut PATTERN_LIST,
    lineno: libc::c_int,
) -> *mut COMMAND {
    let mut temp: *mut CASE_COM = 0 as *mut CASE_COM;
    unsafe {
        temp = malloc(std::mem::size_of::<CASE_COM>() as usize) as *mut CASE_COM;
        (*temp).flags = 0;
        (*temp).line = lineno;
        (*temp).word = word;
        (*temp).clauses = REVERSE_LIST!(clauses, *mut PATTERN_LIST);
    }
    return make_command(command_type_cm_case, temp as *mut SIMPLE_COM);
}

#[no_mangle]
pub fn make_pattern_list(patterns: *mut WordList, action: *mut COMMAND) -> *mut PATTERN_LIST {
    let mut temp: *mut PATTERN_LIST = 0 as *mut PATTERN_LIST;
    unsafe {
        temp = malloc(std::mem::size_of::<PATTERN_LIST>() as usize) as *mut PATTERN_LIST;
        (*temp).patterns = REVERSE_LIST!(patterns, *mut WordList);
        (*temp).action = action;
        (*temp).next = 0 as *mut pattern_list;
        (*temp).flags = 0;
    }
    return temp;
}

#[no_mangle]
pub fn make_if_command(
    test: *mut COMMAND,
    true_case: *mut COMMAND,
    false_case: *mut COMMAND,
) -> *mut COMMAND {
    let mut temp: *mut IF_COM = 0 as *mut IF_COM;
    unsafe {
        temp = malloc(std::mem::size_of::<IF_COM>() as usize) as *mut IF_COM;
        (*temp).flags = 0;
        (*temp).test = test;
        (*temp).true_case = true_case;
        (*temp).false_case = false_case;
    }
    return make_command(command_type_cm_if, temp as *mut SIMPLE_COM);
}

fn make_until_or_while(
    which: command_type,
    test: *mut COMMAND,
    action: *mut COMMAND,
) -> *mut COMMAND {
    let mut temp: *mut WHILE_COM = 0 as *mut WHILE_COM;
    unsafe {
        temp = malloc(std::mem::size_of::<WHILE_COM>() as usize) as *mut WHILE_COM;
        (*temp).flags = 0 as libc::c_int;
        (*temp).test = test;
        (*temp).action = action;
    }
    return make_command(which as libc::c_uint, temp as *mut SIMPLE_COM);
}

#[no_mangle]
pub fn make_while_command(test: *mut COMMAND, action: *mut COMMAND) -> *mut COMMAND {
    return make_until_or_while(command_type_cm_while, test, action);
}

#[no_mangle]
pub fn make_until_command(test: *mut COMMAND, action: *mut COMMAND) -> *mut COMMAND {
    return make_until_or_while(command_type_cm_until, test, action);
}

#[no_mangle]
pub fn make_arith_command(exp: *mut WordList) -> *mut COMMAND {
    let mut command: *mut COMMAND = 0 as *mut COMMAND;
    let mut temp: *mut ARITH_COM = 0 as *mut ARITH_COM;
    unsafe {
        command = malloc(std::mem::size_of::<COMMAND>() as usize) as *mut COMMAND;
        temp = malloc(std::mem::size_of::<ARITH_COM>() as usize) as *mut ARITH_COM;
        (*command).value.Arith = temp;
        (*temp).flags = 0;
        (*temp).line = line_number;
        (*temp).exp = exp;
        (*command).type_0 = command_type_cm_arith;
        (*command).redirects = 0 as *mut libc::c_void as *mut REDIRECT;
        (*command).flags = 0;
    }
    return command;
}

#[no_mangle]
pub fn make_cond_node(
    type_0: libc::c_int,
    op: *mut WordDesc,
    left: *mut cond_com,
    right: *mut cond_com,
) -> *mut COND_COM {
    let mut temp: *mut COND_COM = 0 as *mut COND_COM;
    unsafe {
        temp = malloc(std::mem::size_of::<COND_COM>() as usize) as *mut COND_COM;
        (*temp).flags = 0;
        (*temp).line = line_number;
        (*temp).type_0 = type_0;
        (*temp).op = op;
        (*temp).left = left;
        (*temp).right = right;
    }
    return temp;
}

#[no_mangle]
pub fn make_cond_command(cond_node: *mut COND_COM) -> *mut COMMAND {
    let mut command: *mut COMMAND = 0 as *mut COMMAND;
    unsafe {
        command = malloc(std::mem::size_of::<COMMAND>() as usize) as *mut COMMAND;
        (*command).value.Cond = cond_node;
        (*command).type_0 = command_type_cm_cond;
        (*command).redirects = 0 as *mut REDIRECT;
        (*command).flags = 0;
        (*command).line = if !cond_node.is_null() {
            (*cond_node).line
        } else {
            0
        };
    }
    return command;
}

#[no_mangle]
pub fn make_bare_simple_command() -> *mut COMMAND {
    let mut command: *mut COMMAND = 0 as *mut COMMAND;
    let mut temp: *mut SIMPLE_COM = 0 as *mut SIMPLE_COM;
    unsafe {
        command = malloc(std::mem::size_of::<COMMAND>() as usize) as *mut COMMAND;
        temp = malloc(std::mem::size_of::<SIMPLE_COM>() as usize) as *mut SIMPLE_COM;
        (*command).value.Simple = temp;

        (*temp).flags = 0;
        (*temp).line = line_number;
        (*temp).words = 0 as *mut WordList;
        (*temp).redirects = 0 as *mut REDIRECT;

        (*command).type_0 = command_type_cm_simple;
        (*command).redirects = 0 as *mut REDIRECT;
        (*command).flags = 0;
    }
    return command;
}

#[no_mangle]
pub fn make_simple_command(element: ELEMENT, mut command: *mut COMMAND) -> *mut COMMAND {
    unsafe {
        if command.is_null() {
            command = make_bare_simple_command();
            parser_state |= PST_REDIRLIST as libc::c_int;
        }
        if !(element.word).is_null() {
            (*(*command).value.Simple).words =
                make_word_list(element.word, (*(*command).value.Simple).words);
            parser_state &= !(PST_REDIRLIST as libc::c_int);
        } else if !(element.redirect).is_null() {
            let mut r: *mut REDIRECT = element.redirect;
            while !((*r).next).is_null() {
                r = (*r).next;
            }
            (*r).next = (*(*command).value.Simple).redirects;
            (*(*command).value.Simple).redirects = element.redirect;
        }
        return command;
    }
}

#[no_mangle]
pub fn make_here_document(temp: *mut REDIRECT, lineno: libc::c_int) {
    let mut current_block: u64;
    let mut kill_leading: libc::c_int = 0;
    let mut redir_len: libc::c_int = 0;
    let mut redir_word: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut document: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut full_line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut document_index: libc::c_int = 0;
    let mut document_size: libc::c_int = 0;
    let mut delim_unquoted: libc::c_int = 0;
    unsafe {
        if (*temp).instruction != r_instruction_r_deblank_reading_until
            && (*temp).instruction != r_instruction_r_reading_until
        {
            internal_error(
                b"make_here_document: bad instruction type %d\0" as *const u8
                    as *const libc::c_char,
                (*temp).instruction as libc::c_uint,
            );
            return;
        }
        kill_leading =
            ((*temp).instruction == r_instruction_r_deblank_reading_until) as libc::c_int;

        document = 0 as *mut libc::c_char;
        document_size = 0;
        document_index = document_size;

        redir_word = string_quote_removal((*(*temp).redirectee.filename).word, 0);

        'lable1: loop {
            if !redir_word.is_null() {
                redir_len = strlen(redir_word) as libc::c_int;
            } else {
                (*temp).here_doc_eof = malloc(1 as usize) as *mut libc::c_char;
                *(*temp).here_doc_eof.offset(0) = '\u{0}' as i32 as libc::c_char;
                break 'lable1;
            }

            free((*(*temp).redirector.filename).word as *mut c_void);
            (*temp).here_doc_eof = redir_word;

            delim_unquoted = (((*(*temp).redirector.filename).flags & W_QUOTED as libc::c_int) == 0)
                as libc::c_int;

            full_line = read_secondary_line(delim_unquoted);
            while !full_line.is_null() {
                let mut line: *mut libc::c_char;
                let len: i32;

                here_doc_first_line = 0;
                line = full_line;
                line_number += 1;

                if echo_input_at_read != 0 {
                    fprintf(stderr, b"%s\0" as *const u8 as *const libc::c_char, line);
                }
                if kill_leading != 0 && *line as libc::c_int != 0 {
                    if STREQN!(line, redir_word, redir_len as usize) != 0
                        && *line.offset(redir_len as isize) as libc::c_int == '\n' as i32
                    {
                        break 'lable1;
                    }
                    while *line as libc::c_int == '\t' as i32 {
                        line = line.offset(1);
                    }
                }

                if *line as libc::c_int == 0 {
                    continue;
                }

                if STREQN!(line, redir_word, redir_len as usize) != 0
                    && *line.offset(redir_len as isize) as libc::c_int == '\n' as i32
                {
                    break 'lable1;
                }

                len = strlen(line) as libc::c_int;
                if len + document_index >= document_size {
                    document_size = if document_size != 0 {
                        2 * (document_size + len)
                    } else {
                        len + 2
                    };
                    document = realloc(document as *mut libc::c_void, document_size as usize)
                        as *mut libc::c_char;
                }

                FASTCOPY!(line, document.offset(document_index as isize), len);
                document_index += len;

                full_line = read_secondary_line(delim_unquoted);
            }

            if !full_line.is_null() {
                internal_warning(
                    b"here-document at line %d delimited by end-of-file (wanted `%s')\0"
                        as *const u8 as *const libc::c_char,
                    lineno,
                    redir_word,
                );
            }

            break 'lable1;
        }

        if !document.is_null() {
            *document.offset(document_index as isize) = '\u{0}' as libc::c_char;
        } else {
            document = malloc(1) as *mut libc::c_char;
            *document.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
        }

        (*(*temp).redirectee.filename).word = document;
        here_doc_first_line = 0 as libc::c_int;
    }
}

#[no_mangle]
pub fn make_redirection(
    source: REDIRECTEE,
    instruction: r_instruction,
    dest_and_filename: REDIRECTEE,
    flags: libc::c_int,
) -> *mut REDIRECT {
    unsafe {
        let mut temp: *mut REDIRECT = 0 as *mut REDIRECT;
        let mut w: *mut WordDesc = 0 as *mut WordDesc;
        let mut wlen: libc::c_int = 0;
        let mut lfd: intmax_t = 0;

        temp = malloc(std::mem::size_of::<REDIRECT>() as usize) as *mut REDIRECT;

        (*temp).redirector = source;
        (*temp).redirectee = dest_and_filename;
        (*temp).here_doc_eof = 0 as *mut libc::c_char;
        (*temp).instruction = instruction;
        (*temp).flags = 0;
        (*temp).rflags = flags;
        let ref mut fresh47 = (*temp).next;
        (*temp).next = 0 as *mut REDIRECT;

        match instruction as u32 {
            r_instruction_r_output_direction
            | r_instruction_r_output_force
            | r_instruction_r_err_and_out => {
                (*temp).flags =
                    O_TRUNC as libc::c_int | O_WRONLY as libc::c_int | O_CREAT as libc::c_int;
            }

            r_instruction_r_appending_to | r_instruction_r_append_err_and_out => {
                (*temp).flags =
                    O_APPEND as libc::c_int | O_WRONLY as libc::c_int | O_CREAT as libc::c_int;
            }

            r_instruction_r_input_direction | r_instruction_r_inputa_direction => {
                (*temp).flags = O_RDONLY as libc::c_int;
            }

            r_input_output => {
                (*temp).flags = O_RDWR as libc::c_int | O_CREAT as libc::c_int;
            }

            r_instruction_r_deblank_reading_until
            | r_instruction_r_reading_until
            | r_instruction_r_reading_string
            | r_instruction_r_close_this
            | r_instruction_r_duplicating_input
            | r_instruction_r_duplicating_output => {}

            r_instruction_r_move_input
            | r_instruction_r_move_output
            | r_instruction_r_move_input_word
            | r_instruction_r_move_output_word => {}

            r_instruction_r_duplicating_input_word | r_instruction_r_duplicating_output_word => {
                w = dest_and_filename.filename;
                wlen = (strlen((*w).word) - 1) as libc::c_int;
                if *((*w).word).offset(wlen as isize) as libc::c_int == '-' as i32 {
                    *((*w).word).offset(wlen as isize) = '\u{0}' as i32 as libc::c_char;
                    if all_digits((*w).word) != 0
                        && legal_number((*w).word, &mut lfd) != 0
                        && lfd == lfd as libc::c_long
                    {
                        dispose_word(w);
                        (*temp).instruction =
                            (if instruction == r_instruction_r_duplicating_input_word {
                                r_instruction_r_move_input as libc::c_int
                            } else {
                                r_instruction_r_move_output as libc::c_int
                            }) as r_instruction;
                        (*temp).redirectee.dest = lfd as libc::c_int;
                    } else {
                        (*temp).instruction =
                            (if instruction == r_instruction_r_duplicating_input_word {
                                r_instruction_r_move_input_word as libc::c_int
                            } else {
                                r_instruction_r_move_output_word as libc::c_int
                            }) as r_instruction;
                    }
                }
            }

            _ => {
                programming_error(
                    b"make_redirection: redirection instruction `%d' out of range\0" as *const u8
                        as *const libc::c_char,
                    instruction as libc::c_uint,
                );
                abort();
            }
        }

        return temp;
    }
}

#[no_mangle]
pub fn make_function_def(
    name: *mut WordDesc,
    command: *mut COMMAND,
    lineno: libc::c_int,
    lstart: libc::c_int,
) -> *mut COMMAND {
    let mut temp: *mut FUNCTION_DEF = 0 as *mut FUNCTION_DEF;
    let mut bash_source_v: *mut SHELL_VAR = 0 as *mut SHELL_VAR;
    let mut bash_source_a: *mut ARRAY = 0 as *mut ARRAY;
    unsafe {
        temp = malloc(std::mem::size_of::<FUNCTION_DEF>() as usize) as *mut FUNCTION_DEF;
        (*temp).command = command;
        let ref mut fresh49 = (*temp).name;
        (*temp).name = name;
        (*temp).line = lineno;
        (*temp).flags = 0;
        (*command).line = lstart;

        (*temp).source_file = 0 as *mut libc::c_char;
        GET_ARRAY_FROM_VAR!(
            b"BASH_SOURCE\0" as *const u8 as *const libc::c_char,
            bash_source_v,
            bash_source_a
        );
        if !bash_source_a.is_null() && array_num_elements!(bash_source_a) > 0 {
            (*temp).source_file = array_reference(bash_source_a, 0 as arrayind_t);
        }
        if ((*temp).source_file).is_null() {
            (*temp).source_file = (if shell_initialized != 0 {
                b"main\0" as *const u8 as *const libc::c_char
            } else {
                b"environment\0" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char;
        }

        bind_function_def((*name).word, temp, 0);
        let ref mut fresh53 = (*temp).source_file;
        (*temp).source_file = if !((*temp).source_file).is_null() {
            savestring!((*temp).source_file)
        } else {
            0 as *mut libc::c_char
        };
    }
    return make_command(command_type_cm_function_def, temp as *mut SIMPLE_COM);
}

#[no_mangle]
pub fn make_subshell_command(command: *mut COMMAND) -> *mut COMMAND {
    unsafe {
        let mut temp: *mut SUBSHELL_COM = 0 as *mut SUBSHELL_COM;
        temp = malloc(std::mem::size_of::<SUBSHELL_COM>() as usize) as *mut SUBSHELL_COM;

        (*temp).command = command;
        (*temp).flags = 0x1;
        (*temp).line = line_number;
        return make_command(command_type_cm_subshell, temp as *mut SIMPLE_COM);
    }
}

#[no_mangle]
pub fn make_coproc_command(name: *mut libc::c_char, command: *mut COMMAND) -> *mut COMMAND {
    let mut temp: *mut COPROC_COM = 0 as *mut COPROC_COM;
    unsafe {
        temp = malloc(std::mem::size_of::<COPROC_COM>() as usize) as *mut COPROC_COM;
        let ref mut fresh55 = (*temp).name;
        (*temp).name = savestring!(name);
        let ref mut fresh56 = (*temp).command;
        (*temp).command = command;
        (*temp).flags = CMD_WANT_SUBSHELL as libc::c_int | CMD_COPROC_SUBSHELL as libc::c_int;
    }
    return make_command(command_type_cm_coproc, temp as *mut SIMPLE_COM);
}

fn output_requirement(deptype: *const libc::c_char, filename: *mut libc::c_char) {
    unsafe {
        static mut alphabet_set: *mut libc::c_char =
            b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ\0" as *const u8
                as *const libc::c_char as *mut libc::c_char;
        if !(strchr(filename, '$' as i32)).is_null()
            || *filename.offset(0 as libc::c_int as isize) as libc::c_int != '/' as i32
                && !(strchr(filename, '/' as i32)).is_null()
        {
            return;
        }
        if !(strpbrk(filename, alphabet_set)).is_null() {
            printf(
                b"%s(%s)\n\0" as *const u8 as *const libc::c_char,
                deptype,
                filename,
            );
        }
    }
}

#[no_mangle]
pub fn clean_simple_command(command: *mut COMMAND) -> *mut COMMAND {
    unsafe {
        if (*command).type_0 != command_type_cm_simple {
            command_error(
                b"clean_simple_command\0" as *const u8 as *const libc::c_char,
                CMDERR_BADTYPE as libc::c_int,
                (*command).type_0 as libc::c_int,
                0,
            );
        } else {
            (*(*command).value.Simple).words =
                REVERSE_LIST!((*(*command).value.Simple).words, *mut WordList);
            (*(*command).value.Simple).redirects =
                REVERSE_LIST!((*(*command).value.Simple).redirects, *mut REDIRECT);
        }

        if rpm_requires != 0 && !((*(*command).value.Simple).words).is_null() {
            let mut cmd0: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut cmd1: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut b: *mut builtin = 0 as *mut builtin;

            cmd0 = (*(*(*(*command).value.Simple).words).word).word;
            b = builtin_address_internal(cmd0, 0);
            cmd1 = 0 as *mut libc::c_char;
            if !((*(*(*command).value.Simple).words).next).is_null() {
                cmd1 = (*(*(*(*(*command).value.Simple).words).next).word).word;
            }

            if !b.is_null() {
                if (*b).flags & REQUIRES_BUILTIN as libc::c_int != 0 && !cmd1.is_null() {
                    output_requirement(b"executable\0" as *const u8 as *const libc::c_char, cmd1);
                }
            } else if assignment(cmd0, 0 as libc::c_int) == 0 {
                output_requirement(
                    if !(find_function(cmd0)).is_null() {
                        b"function\0" as *const u8 as *const libc::c_char
                    } else {
                        b"executable\0" as *const u8 as *const libc::c_char
                    },
                    cmd0,
                );
            }
        }
        parser_state &= !(PST_REDIRLIST as libc::c_int);
        return command;
    }
}

#[no_mangle]
pub fn connect_async_list(
    command: *mut COMMAND,
    command2: *mut COMMAND,
    connector: libc::c_int,
) -> *mut COMMAND {
    let mut t: *mut COMMAND = 0 as *mut COMMAND;
    let mut t1: *mut COMMAND = 0 as *mut COMMAND;
    let mut t2: *mut COMMAND = 0 as *mut COMMAND;
    unsafe {
        t1 = command;
        t = (*(*command).value.Connection).second;

        if t.is_null()
            || (*command).flags & CMD_WANT_SUBSHELL as libc::c_int != 0
            || (*(*command).value.Connection).connector != ';' as i32
        {
            t = command_connect(command, command2, connector);
            return t;
        }

        while (*t).flags & CMD_WANT_SUBSHELL as libc::c_int == 0 as libc::c_int
            && (*t).type_0 == command_type_cm_connection
            && (*(*t).value.Connection).connector == ';' as i32
        {
            t1 = t;
            t = (*(*t).value.Connection).second;
        }
        t2 = command_connect(t, command2, connector);
        (*(*t1).value.Connection).second = t2;
        return command;
    }
}
