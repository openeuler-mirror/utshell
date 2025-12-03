use crate::error::command_error;
use crate::src_common::*;

#[no_mangle]
pub fn dispose_command(command: *mut COMMAND) {
    if command.is_null() {
        return;
    }
    unsafe {
        if !((*command).redirects).is_null() {
            dispose_redirects((*command).redirects);
        }
        match (*command).type_0 {
            command_type_cm_for | command_type_cm_select => {
                let mut c: *mut FOR_COM = 0 as *mut FOR_COM;
                if (*command).type_0 == command_type_cm_select {
                    c = (*command).value.Select as *mut FOR_COM;
                } else {
                    c = (*command).value.For;
                }
                dispose_word((*c).name);
                dispose_words((*c).map_list);
                dispose_command((*c).action);
                free(c as *mut libc::c_void);
            }
            command_type_cm_arith_for => {
                let mut c_0: *mut ARITH_FOR_COM = 0 as *mut ARITH_FOR_COM;
                c_0 = (*command).value.ArithFor;
                dispose_words((*c_0).init);
                dispose_words((*c_0).test);
                dispose_words((*c_0).step);
                dispose_command((*c_0).action);
                free(c_0 as *mut libc::c_void);
            }
            command_type_cm_group => {
                dispose_command((*(*command).value.Group).command);
                free((*command).value.Group as *mut libc::c_void);
            }
            command_type_cm_subshell => {
                dispose_command((*(*command).value.Subshell).command);
                free((*command).value.Subshell as *mut libc::c_void);
            }
            command_type_cm_coproc => {
                free((*(*command).value.Coproc).name as *mut libc::c_void);
                dispose_command((*(*command).value.Coproc).command);
                free((*command).value.Coproc as *mut libc::c_void);
            }
            command_type_cm_case => {
                let mut c_1: *mut CASE_COM = 0 as *mut CASE_COM;
                let mut t: *mut PATTERN_LIST = 0 as *mut PATTERN_LIST;
                let mut p: *mut PATTERN_LIST = 0 as *mut PATTERN_LIST;
                c_1 = (*command).value.Case;
                dispose_word((*c_1).word);
                p = (*c_1).clauses;
                while !p.is_null() {
                    dispose_words((*p).patterns);
                    dispose_command((*p).action);
                    t = p;
                    p = (*p).next;
                    free(t as *mut libc::c_void);
                }
                free(c_1 as *mut libc::c_void);
            }
            command_type_cm_until | command_type_cm_while => {
                let mut c_2: *mut WHILE_COM = 0 as *mut WHILE_COM;
                c_2 = (*command).value.While;
                dispose_command((*c_2).test);
                dispose_command((*c_2).action);
                free(c_2 as *mut libc::c_void);
            }
            command_type_cm_if => {
                let mut c_3: *mut IF_COM = 0 as *mut IF_COM;
                c_3 = (*command).value.If;
                dispose_command((*c_3).test);
                dispose_command((*c_3).true_case);
                dispose_command((*c_3).false_case);
                free(c_3 as *mut libc::c_void);
            }
            command_type_cm_simple => {
                let mut c_4: *mut SIMPLE_COM = 0 as *mut SIMPLE_COM;
                c_4 = (*command).value.Simple;
                dispose_words((*c_4).words);
                dispose_redirects((*c_4).redirects);
                free(c_4 as *mut libc::c_void);
            }
            command_type_cm_connection => {
                let mut c_5: *mut CONNECTION = 0 as *mut CONNECTION;
                c_5 = (*command).value.Connection;
                dispose_command((*c_5).first);
                dispose_command((*c_5).second);
                free(c_5 as *mut libc::c_void);
            }
            command_type_cm_arith => {
                let mut c_6: *mut ARITH_COM = 0 as *mut ARITH_COM;
                c_6 = (*command).value.Arith;
                dispose_words((*c_6).exp);
                free(c_6 as *mut libc::c_void);
            }
            command_type_cm_cond => {
                let mut c_7: *mut COND_COM = 0 as *mut COND_COM;
                c_7 = (*command).value.Cond;
                dispose_cond_node(c_7);
            }
            command_type_cm_function_def => {
                let mut c_8: *mut FUNCTION_DEF = 0 as *mut FUNCTION_DEF;
                c_8 = (*command).value.Function_def;
                dispose_function_def(c_8);
            }
            _ => {
                command_error(
                    b"dispose_command\0" as *const u8 as *const libc::c_char,
                    CMDERR_BADTYPE as libc::c_int,
                    (*command).type_0 as libc::c_int,
                    0,
                );
            }
        }
        free(command as *mut libc::c_void);
    }
}

#[no_mangle]
pub fn dispose_cond_node(cond: *mut COND_COM) {
    unsafe {
        if !cond.is_null() {
            if !((*cond).left).is_null() {
                dispose_cond_node((*cond).left);
            }
            if !((*cond).right).is_null() {
                dispose_cond_node((*cond).right);
            }
            if !((*cond).op).is_null() {
                dispose_word((*cond).op);
            }
            free(cond as *mut libc::c_void);
        }
    }
}

#[no_mangle]
pub fn dispose_function_def_contents(c: *mut FUNCTION_DEF) {
    unsafe {
        dispose_word((*c).name);
        dispose_command((*c).command);
        FREE!((*c).source_file);
    }
}

#[no_mangle]
pub fn dispose_function_def(c: *mut FUNCTION_DEF) {
    dispose_function_def_contents(c);
    unsafe {
        free(c as *mut libc::c_void);
    }
}

#[no_mangle]
pub fn dispose_word(w: *mut WordDesc) {
    unsafe {
        FREE!((*w).word);
        // ocache_free!(wdcache, WordDesc, w);
        (*w).word = 0 as *mut libc::c_char;
        free(w as *mut c_void);
    }
}

#[no_mangle]
pub fn dispose_word_desc(w: *mut WordDesc) {
    unsafe {
        (*w).word = 0 as *mut libc::c_char;
        // ocache_free!(wdcache, WordDesc, w);
        free(w as *mut c_void);
    }
}

#[no_mangle]
pub fn dispose_words(mut list: *mut WordList) {
    let mut t: *mut WordList = 0 as *mut WordList;
    while !list.is_null() {
        unsafe {
            t = list;
            list = (*list).next;
            dispose_word((*t).word);

            // ocache_free!(wlcache, WordList, t);
            free(t as *mut c_void);
        }
    }
}

#[no_mangle]
pub fn dispose_redirects(mut list: *mut REDIRECT) {
    let mut t: *mut REDIRECT = 0 as *mut REDIRECT;
    unsafe {
        while !list.is_null() {
            t = list;
            list = (*list).next;

            if (*t).rflags & REDIR_VARASSIGN as libc::c_int != 0 {
                dispose_word((*t).redirector.filename);
            }

            match (*t).instruction {
                r_instruction_r_reading_until | r_instruction_r_deblank_reading_until => {
                    free((*t).here_doc_eof as *mut libc::c_void);
                    dispose_word((*t).redirectee.filename);
                }
                r_instruction_r_reading_string
                | r_instruction_r_output_direction
                | r_instruction_r_input_direction
                | r_instruction_r_inputa_direction
                | r_instruction_r_appending_to
                | r_instruction_r_err_and_out
                | r_instruction_r_append_err_and_out
                | r_instruction_r_input_output
                | r_instruction_r_output_force
                | r_instruction_r_duplicating_input_word
                | r_instruction_r_duplicating_output_word
                | r_instruction_r_move_input_word
                | r_instruction_r_move_output_word => {
                    dispose_word((*t).redirectee.filename);
                }
                _ => {}
            }

            free(t as *mut libc::c_void);
        }
    }
}
