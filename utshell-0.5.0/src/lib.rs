/*
 * SPDX-FileCopyrightText: 2025 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: GPL-2.0-or-later
 */
extern crate lazy_static;
#[warn(unused_unsafe)]
#[macro_use]

pub mod src_common;
pub mod alias;
pub mod array;
pub mod arrayfunc;
pub mod assoc;
pub mod bashhist;
pub mod bashline;
pub mod brace;
pub mod bracecomp;
pub mod copycmd;
pub mod dispose_cmd;
pub mod error;
pub mod eval;
pub mod execute_cmd;
pub mod expr;
pub mod findcmd;
pub mod flags;
pub mod general;
pub mod hashcmd;
pub mod hashlib;
pub mod input;
pub mod jobs;
pub mod list;
pub mod local;
pub mod mailcheck;
pub mod make_cmd;
pub mod pathexp;
pub mod pcomplete;
pub mod pcomplib;
pub mod print_cmd;
pub mod readline;
pub mod redir;
pub mod sig;
pub mod stringlib;
pub mod subst;
pub mod syntax;
pub mod test;
pub mod trap;
pub mod unwind_prot;
pub mod utshell;
pub mod variables;
pub mod version;
pub mod y_tab;

pub mod builtins {
    pub mod alias;
    pub mod bashgetopt;
    pub mod bind;
    pub mod break_1;
    pub mod builtin;
    pub mod builtins;
    pub mod caller;
    pub mod cd;
    pub mod cmd;
    pub mod colon;
    pub mod command;
    pub mod common;
    pub mod complete;
    pub mod declare;
    pub mod echo;
    pub mod enable;
    pub mod eval;
    pub mod evalfile;
    pub mod evalstring;
    pub mod exec;
    pub mod exec_cmd;
    pub mod exit;
    pub mod fc;
    pub mod fg_bg;
    pub mod getopt;
    pub mod getopts;
    pub mod hash;
    pub mod help;
    pub mod history;
    pub mod jobs;
    pub mod kill;
    pub mod let_1;
    pub mod mapfile;
    pub mod printf;
    pub mod pushd;
    pub mod read;
    pub mod return_1;
    pub mod set;
    pub mod setattr;
    pub mod shift;
    pub mod shopt;
    pub mod signal;
    pub mod source;
    pub mod suspend;
    pub mod test;
    pub mod times;
    pub mod trap;
    pub mod type_1;
    pub mod ulimit;
    pub mod umask;
    pub mod wait;
}
