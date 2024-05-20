//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

use libc::size_t;
use r_bash::_ISalnum;
use r_bash::_ISalpha;
use r_bash::__ctype_b_loc;
use r_bash::__jmp_buf_tag;
use r_bash::__sigset_t;
use r_bash::__sigsetjmp;
use r_bash::array_variable_name;
use r_bash::array_variable_part;
use r_bash::arrayind_t;
use r_bash::assoc_expand_once;
use r_bash::bind_int_variable;
use r_bash::dcgettext;
use r_bash::err_unboundvar;
use r_bash::find_variable;
use r_bash::find_variable_last_nameref;
use r_bash::fmtumax;
use r_bash::get_array_value;
use r_bash::get_variable_value;
use r_bash::imaxdiv;
use r_bash::imaxdiv_t;
use r_bash::interactive_shell;
use r_bash::internal_error;
use r_bash::itos;
use r_bash::jump_to_top_level;
use r_bash::legal_identifier;
use r_bash::mbschr;
use r_bash::no_longjmp_on_fatal_error;
use r_bash::set_exit_status;
use r_bash::sh_xfree;
use r_bash::sh_xmalloc;
use r_bash::sh_xrealloc;
use r_bash::sigjmp_buf;
use r_bash::siglongjmp;
use r_bash::skipsubscript;
use r_bash::sprintf;
use r_bash::strcpy;
use r_bash::strlen;
use r_bash::stupidly_hack_special_variables;
use r_bash::this_command_name;
use r_bash::top_level_cleanup;
use r_bash::uintmax_t;
use r_bash::unbound_vars_is_error;
use r_bash::SHELL_VAR;