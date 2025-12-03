/*
 * SPDX-FileCopyrightText: 2025 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: GPL-2.0-or-later
 */
use crate::src_common::*;
pub const control_character_threshold: u32 = 32;
pub const control_character_mask: u32 = 31;
pub const meta_character_threshold: u32 = 127;
pub const control_character_bit: u32 = 64;
pub const meta_character_bit: u32 = 128;
pub const largest_char: u32 = 255;
pub const NEWLINE: u8 = 10u8;
pub const RUBOUT: u32 = 127;
pub const TAB: u8 = 9u8;
pub const SPACE: u8 = 32u8;
pub const KEYMAP_SIZE: u32 = 257;
pub const ANYOTHERKEY: u32 = 256;
pub const ISKMAP: u32 = 1;
pub const ISMACR: u32 = 2;
pub const RL_READLINE_VERSION: u32 = 2049;
pub const RL_VERSION_MAJOR: u32 = 8;
pub const RL_VERSION_MINOR: u32 = 1;
pub const READERR: i32 = -2;
pub const RL_PROMPT_START_IGNORE: u8 = 1u8;
pub const RL_PROMPT_END_IGNORE: u8 = 2u8;
pub const NO_MATCH: u32 = 0;
pub const SINGLE_MATCH: u32 = 1;
pub const MULT_MATCH: u32 = 2;
pub const RL_STATE_NONE: u32 = 0;
pub const RL_STATE_INITIALIZING: u32 = 1;
pub const RL_STATE_INITIALIZED: u32 = 2;
pub const RL_STATE_TERMPREPPED: u32 = 4;
pub const RL_STATE_READCMD: u32 = 8;
pub const RL_STATE_METANEXT: u32 = 16;
pub const RL_STATE_DISPATCHING: u32 = 32;
pub const RL_STATE_MOREINPUT: u32 = 64;
pub const RL_STATE_ISEARCH: u32 = 128;
pub const RL_STATE_NSEARCH: u32 = 256;
pub const RL_STATE_SEARCH: u32 = 512;
pub const RL_STATE_NUMERICARG: u32 = 1024;
pub const RL_STATE_MACROINPUT: u32 = 2048;
pub const RL_STATE_MACRODEF: u32 = 4096;
pub const RL_STATE_OVERWRITE: u32 = 8192;
pub const RL_STATE_COMPLETING: u32 = 16384;
pub const RL_STATE_UNDOING: u32 = 65536;
pub const RL_STATE_INPUTPENDING: u32 = 131072;
pub const RL_STATE_TTYCSAVED: u32 = 262144;
pub const RL_STATE_CALLBACK: u32 = 524288;
pub const RL_STATE_VIMOTION: u32 = 1048576;
pub const RL_STATE_MULTIKEY: u32 = 2097152;
pub const RL_STATE_VICMDONCE: u32 = 4194304;
pub const RL_STATE_CHARSEARCH: u32 = 8388608;
pub const RL_STATE_REDISPLAYING: u32 = 16777216;
pub const RL_STATE_DONE: u32 = 33554432;
pub const MB_FIND_ANY: u32 = 0;
pub const MB_FIND_NONZERO: u32 = 1;
pub const DEFAULT_INPUTRC: &'static [u8; 11usize] = b"~/.inputrc\0";
pub const SYS_INPUTRC: &'static [u8; 13usize] = b"/etc/inputrc\0";
pub const RL_COMMENT_BEGIN_DEFAULT: &'static [u8; 2usize] = b"#\0";
pub const RL_EMACS_MODESTR_DEFAULT: &'static [u8; 2usize] = b"@\0";
pub const RL_EMACS_MODESTR_DEFLEN: u32 = 1;
pub const RL_VI_INS_MODESTR_DEFAULT: &'static [u8; 6usize] = b"(ins)\0";
pub const RL_VI_INS_MODESTR_DEFLEN: u32 = 5;
pub const RL_VI_CMD_MODESTR_DEFAULT: &'static [u8; 6usize] = b"(cmd)\0";
pub const RL_VI_CMD_MODESTR_DEFLEN: u32 = 5;
pub const RL_SEARCH_ISEARCH: u32 = 1;
pub const RL_SEARCH_NSEARCH: u32 = 2;
pub const RL_SEARCH_CSEARCH: u32 = 4;
pub const SF_REVERSE: u32 = 1;
pub const SF_FOUND: u32 = 2;
pub const SF_FAILED: u32 = 4;
pub const SF_CHGKMAP: u32 = 8;
pub const SF_PATTERN: u32 = 16;
pub const SF_NOCASE: u32 = 32;
pub const NUM_SAWMINUS: u32 = 1;
pub const NUM_SAWDIGITS: u32 = 2;
pub const NUM_READONE: u32 = 4;
pub const KSEQ_DISPATCHED: u32 = 1;
pub const KSEQ_SUBSEQ: u32 = 2;
pub const KSEQ_RECURSIVE: u32 = 4;
pub const VIM_DELETE: u32 = 1;
pub const VIM_CHANGE: u32 = 2;
pub const VIM_YANK: u32 = 4;
pub const VMSTATE_READ: u32 = 1;
pub const VMSTATE_NUMARG: u32 = 2;
pub const BRACKETED_PASTE_DEFAULT: u32 = 1;
pub const BRACK_PASTE_PREF: &'static [u8; 7usize] = b"\x1B[200~\0";
pub const BRACK_PASTE_SUFF: &'static [u8; 7usize] = b"\x1B[201~\0";
pub const BRACK_PASTE_LAST: u8 = 126u8;
pub const BRACK_PASTE_SLEN: u32 = 6;
pub const BRACK_PASTE_INIT: &'static [u8; 9usize] = b"\x1B[?2004h\0";
pub const BRACK_PASTE_FINI: &'static [u8; 11usize] = b"\x1B[?2004l\\r\0";
pub const _SYS_IOCTL_H: u32 = 1;
pub const _IOC_NRBITS: u32 = 8;
pub const _IOC_TYPEBITS: u32 = 8;
pub const _IOC_SIZEBITS: u32 = 14;
pub const _IOC_DIRBITS: u32 = 2;
pub const _IOC_NRMASK: u32 = 255;
pub const _IOC_TYPEMASK: u32 = 255;
pub const _IOC_SIZEMASK: u32 = 16383;
pub const _IOC_DIRMASK: u32 = 3;
pub const _IOC_NRSHIFT: u32 = 0;
pub const _IOC_TYPESHIFT: u32 = 8;
pub const _IOC_SIZESHIFT: u32 = 16;
pub const _IOC_DIRSHIFT: u32 = 30;
pub const _IOC_NONE: u32 = 0;
pub const _IOC_WRITE: u32 = 1;
pub const _IOC_READ: u32 = 2;
pub const IOC_IN: u32 = 1073741824;
pub const IOC_OUT: u32 = 2147483648;
pub const IOC_INOUT: u32 = 3221225472;
pub const IOCSIZE_MASK: u32 = 1073676288;
pub const IOCSIZE_SHIFT: u32 = 16;
pub const TCGETS: u32 = 21505;
pub const TCSETS: u32 = 21506;
pub const TCSETSW: u32 = 21507;
pub const TCSETSF: u32 = 21508;
pub const TCGETA: u32 = 21509;
pub const TCSETA: u32 = 21510;
pub const TCSETAW: u32 = 21511;
pub const TCSETAF: u32 = 21512;
pub const TCSBRK: u32 = 21513;
pub const TCXONC: u32 = 21514;
pub const TCFLSH: u32 = 21515;
pub const TIOCEXCL: u32 = 21516;
pub const TIOCNXCL: u32 = 21517;
pub const TIOCSCTTY: u32 = 21518;
pub const TIOCGPGRP: u32 = 21519;
pub const TIOCSPGRP: u32 = 21520;
pub const TIOCOUTQ: u32 = 21521;
pub const TIOCSTI: u32 = 21522;
pub const TIOCGWINSZ: u32 = 21523;
pub const TIOCSWINSZ: u32 = 21524;
pub const TIOCMGET: u32 = 21525;
pub const TIOCMBIS: u32 = 21526;
pub const TIOCMBIC: u32 = 21527;
pub const TIOCMSET: u32 = 21528;
pub const TIOCGSOFTCAR: u32 = 21529;
pub const TIOCSSOFTCAR: u32 = 21530;
pub const FIONREAD: u32 = 21531;
pub const TIOCINQ: u32 = 21531;
pub const TIOCLINUX: u32 = 21532;
pub const TIOCCONS: u32 = 21533;
pub const TIOCGSERIAL: u32 = 21534;
pub const TIOCSSERIAL: u32 = 21535;
pub const TIOCPKT: u32 = 21536;
pub const FIONBIO: u32 = 21537;
pub const TIOCNOTTY: u32 = 21538;
pub const TIOCSETD: u32 = 21539;
pub const TIOCGETD: u32 = 21540;
pub const TCSBRKP: u32 = 21541;
pub const TIOCSBRK: u32 = 21543;
pub const TIOCCBRK: u32 = 21544;
pub const TIOCGSID: u32 = 21545;
pub const TIOCGRS485: u32 = 21550;
pub const TIOCSRS485: u32 = 21551;
pub const TCGETX: u32 = 21554;
pub const TCSETX: u32 = 21555;
pub const TCSETXF: u32 = 21556;
pub const TCSETXW: u32 = 21557;
pub const TIOCVHANGUP: u32 = 21559;
pub const FIONCLEX: u32 = 21584;
pub const FIOCLEX: u32 = 21585;
pub const FIOASYNC: u32 = 21586;
pub const TIOCSERCONFIG: u32 = 21587;
pub const TIOCSERGWILD: u32 = 21588;
pub const TIOCSERSWILD: u32 = 21589;
pub const TIOCGLCKTRMIOS: u32 = 21590;
pub const TIOCSLCKTRMIOS: u32 = 21591;
pub const TIOCSERGSTRUCT: u32 = 21592;
pub const TIOCSERGETLSR: u32 = 21593;
pub const TIOCSERGETMULTI: u32 = 21594;
pub const TIOCSERSETMULTI: u32 = 21595;
pub const TIOCMIWAIT: u32 = 21596;
pub const TIOCGICOUNT: u32 = 21597;
pub const FIOQSIZE: u32 = 21600;
pub const TIOCPKT_DATA: u32 = 0;
pub const TIOCPKT_FLUSHREAD: u32 = 1;
pub const TIOCPKT_FLUSHWRITE: u32 = 2;
pub const TIOCPKT_STOP: u32 = 4;
pub const TIOCPKT_START: u32 = 8;
pub const TIOCPKT_NOSTOP: u32 = 16;
pub const TIOCPKT_DOSTOP: u32 = 32;
pub const TIOCPKT_IOCTL: u32 = 64;
pub const TIOCSER_TEMT: u32 = 1;
pub const SIOCADDRT: u32 = 35083;
pub const SIOCDELRT: u32 = 35084;
pub const SIOCRTMSG: u32 = 35085;
pub const SIOCGIFNAME: u32 = 35088;
pub const SIOCSIFLINK: u32 = 35089;
pub const SIOCGIFCONF: u32 = 35090;
pub const SIOCGIFFLAGS: u32 = 35091;
pub const SIOCSIFFLAGS: u32 = 35092;
pub const SIOCGIFADDR: u32 = 35093;
pub const SIOCSIFADDR: u32 = 35094;
pub const SIOCGIFDSTADDR: u32 = 35095;
pub const SIOCSIFDSTADDR: u32 = 35096;
pub const SIOCGIFBRDADDR: u32 = 35097;
pub const SIOCSIFBRDADDR: u32 = 35098;
pub const SIOCGIFNETMASK: u32 = 35099;
pub const SIOCSIFNETMASK: u32 = 35100;
pub const SIOCGIFMETRIC: u32 = 35101;
pub const SIOCSIFMETRIC: u32 = 35102;
pub const SIOCGIFMEM: u32 = 35103;
pub const SIOCSIFMEM: u32 = 35104;
pub const SIOCGIFMTU: u32 = 35105;
pub const SIOCSIFMTU: u32 = 35106;
pub const SIOCSIFNAME: u32 = 35107;
pub const SIOCSIFHWADDR: u32 = 35108;
pub const SIOCGIFENCAP: u32 = 35109;
pub const SIOCSIFENCAP: u32 = 35110;
pub const SIOCGIFHWADDR: u32 = 35111;
pub const SIOCGIFSLAVE: u32 = 35113;
pub const SIOCSIFSLAVE: u32 = 35120;
pub const SIOCADDMULTI: u32 = 35121;
pub const SIOCDELMULTI: u32 = 35122;
pub const SIOCGIFINDEX: u32 = 35123;
pub const SIOGIFINDEX: u32 = 35123;
pub const SIOCSIFPFLAGS: u32 = 35124;
pub const SIOCGIFPFLAGS: u32 = 35125;
pub const SIOCDIFADDR: u32 = 35126;
pub const SIOCSIFHWBROADCAST: u32 = 35127;
pub const SIOCGIFCOUNT: u32 = 35128;
pub const SIOCGIFBR: u32 = 35136;
pub const SIOCSIFBR: u32 = 35137;
pub const SIOCGIFTXQLEN: u32 = 35138;
pub const SIOCSIFTXQLEN: u32 = 35139;
pub const SIOCDARP: u32 = 35155;
pub const SIOCGARP: u32 = 35156;
pub const SIOCSARP: u32 = 35157;
pub const SIOCDRARP: u32 = 35168;
pub const SIOCGRARP: u32 = 35169;
pub const SIOCSRARP: u32 = 35170;
pub const SIOCGIFMAP: u32 = 35184;
pub const SIOCSIFMAP: u32 = 35185;
pub const SIOCADDDLCI: u32 = 35200;
pub const SIOCDELDLCI: u32 = 35201;
pub const SIOCDEVPRIVATE: u32 = 35312;
pub const SIOCPROTOPRIVATE: u32 = 35296;
pub const NCC: u32 = 8;
pub const TIOCM_LE: u32 = 1;
pub const TIOCM_DTR: u32 = 2;
pub const TIOCM_RTS: u32 = 4;
pub const TIOCM_ST: u32 = 8;
pub const TIOCM_SR: u32 = 16;
pub const TIOCM_CTS: u32 = 32;
pub const TIOCM_CAR: u32 = 64;
pub const TIOCM_RNG: u32 = 128;
pub const TIOCM_DSR: u32 = 256;
pub const TIOCM_CD: u32 = 64;
pub const TIOCM_RI: u32 = 128;
pub const N_TTY: u32 = 0;
pub const N_SLIP: u32 = 1;
pub const N_MOUSE: u32 = 2;
pub const N_PPP: u32 = 3;
pub const N_STRIP: u32 = 4;
pub const N_AX25: u32 = 5;
pub const N_X25: u32 = 6;
pub const N_6PACK: u32 = 7;
pub const N_MASC: u32 = 8;
pub const N_R3964: u32 = 9;
pub const N_PROFIBUS_FDL: u32 = 10;
pub const N_IRDA: u32 = 11;
pub const N_SMSBLOCK: u32 = 12;
pub const N_HDLC: u32 = 13;
pub const N_SYNC_PPP: u32 = 14;
pub const N_HCI: u32 = 15;
pub const no_mode: i32 = -1;
pub const vi_mode: u32 = 0;
pub const emacs_mode: u32 = 1;
pub const RL_IM_INSERT: u32 = 1;
pub const RL_IM_OVERWRITE: u32 = 0;
pub const RL_IM_DEFAULT: u32 = 1;
pub const NO_BELL: u32 = 0;
pub const AUDIBLE_BELL: u32 = 1;
pub const VISIBLE_BELL: u32 = 2;
pub const FTO: u32 = 1;
pub const BTO: i32 = -1;
pub const FFIND: u32 = 2;
pub const BFIND: i32 = -2;
pub const RL_QF_SINGLE_QUOTE: u32 = 1;
pub const RL_QF_DOUBLE_QUOTE: u32 = 2;
pub const RL_QF_BACKSLASH: u32 = 4;
pub const RL_QF_OTHER_QUOTE: u32 = 8;
pub const DEFAULT_BUFFER_SIZE: u32 = 256;
pub const NCURSES_TERMCAP_H_incl: u32 = 1;
pub const NCURSES_VERSION: &'static [u8; 4usize] = b"6.1\0";
pub const NCURSES_DLL_H_incl: u32 = 1;
pub const EVENT_NOT_FOUND: u32 = 0;
pub const BAD_WORD_SPEC: u32 = 1;
pub const SUBST_FAILED: u32 = 2;
pub const BAD_MODIFIER: u32 = 3;
pub const NO_PREV_SUBST: u32 = 4;
pub const NON_ANCHORED_SEARCH: u32 = 0;
pub const ANCHORED_SEARCH: u32 = 1;
pub const PATTERN_SEARCH: u32 = 2;
pub const HISTORY_APPEND: u32 = 0;
pub const HISTORY_OVERWRITE: u32 = 1;
extern "C" {
    pub fn wcscpy(__dest: *mut wchar_t, __src: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcsncpy(__dest: *mut wchar_t, __src: *const wchar_t, __n: usize) -> *mut wchar_t;
}
extern "C" {
    pub fn wcscat(__dest: *mut wchar_t, __src: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcsncat(__dest: *mut wchar_t, __src: *const wchar_t, __n: usize) -> *mut wchar_t;
}
extern "C" {
    pub fn wcscmp(__s1: *const wchar_t, __s2: *const wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcsncmp(__s1: *const wchar_t, __s2: *const wchar_t, __n: usize)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcscasecmp(__s1: *const wchar_t, __s2: *const wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcsncasecmp(
        __s1: *const wchar_t,
        __s2: *const wchar_t,
        __n: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcscasecmp_l(
        __s1: *const wchar_t,
        __s2: *const wchar_t,
        __loc: locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcsncasecmp_l(
        __s1: *const wchar_t,
        __s2: *const wchar_t,
        __n: usize,
        __loc: locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcscoll(__s1: *const wchar_t, __s2: *const wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcsxfrm(__s1: *mut wchar_t, __s2: *const wchar_t, __n: usize) -> usize;
}
extern "C" {
    pub fn wcscoll_l(
        __s1: *const wchar_t,
        __s2: *const wchar_t,
        __loc: locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcsxfrm_l(
        __s1: *mut wchar_t,
        __s2: *const wchar_t,
        __n: usize,
        __loc: locale_t,
    ) -> usize;
}
extern "C" {
    pub fn wcsdup(__s: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcschr(__wcs: *const wchar_t, __wc: wchar_t) -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcsrchr(__wcs: *const wchar_t, __wc: wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcschrnul(__s: *const wchar_t, __wc: wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcscspn(__wcs: *const wchar_t, __reject: *const wchar_t) -> usize;
}
extern "C" {
    pub fn wcsspn(__wcs: *const wchar_t, __accept: *const wchar_t) -> usize;
}
extern "C" {
    pub fn wcspbrk(__wcs: *const wchar_t, __accept: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcsstr(__haystack: *const wchar_t, __needle: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcstok(
        __s: *mut wchar_t,
        __delim: *const wchar_t,
        __ptr: *mut *mut wchar_t,
    ) -> *mut wchar_t;
}
extern "C" {
    pub fn wcslen(__s: *const wchar_t) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn wcswcs(__haystack: *const wchar_t, __needle: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcsnlen(__s: *const wchar_t, __maxlen: usize) -> usize;
}
extern "C" {
    pub fn wmemchr(__s: *const wchar_t, __c: wchar_t, __n: usize) -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn wmemcmp(__s1: *const wchar_t, __s2: *const wchar_t, __n: usize)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wmemcpy(__s1: *mut wchar_t, __s2: *const wchar_t, __n: usize) -> *mut wchar_t;
}
extern "C" {
    pub fn wmemmove(__s1: *mut wchar_t, __s2: *const wchar_t, __n: usize) -> *mut wchar_t;
}
extern "C" {
    pub fn wmemset(__s: *mut wchar_t, __c: wchar_t, __n: usize) -> *mut wchar_t;
}
extern "C" {
    pub fn wmempcpy(__s1: *mut wchar_t, __s2: *const wchar_t, __n: usize) -> *mut wchar_t;
}
extern "C" {
    pub fn btowc(__c: ::std::os::raw::c_int) -> wint_t;
}
extern "C" {
    pub fn wctob(__c: wint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbsinit(__ps: *const mbstate_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcrtomb(__s: *mut ::std::os::raw::c_char, __wc: wchar_t, __ps: *mut mbstate_t) -> usize;
}
extern "C" {
    pub fn mbsrtowcs(
        __dst: *mut wchar_t,
        __src: *mut *const ::std::os::raw::c_char,
        __len: usize,
        __ps: *mut mbstate_t,
    ) -> usize;
}
extern "C" {
    pub fn wcsrtombs(
        __dst: *mut ::std::os::raw::c_char,
        __src: *mut *const wchar_t,
        __len: usize,
        __ps: *mut mbstate_t,
    ) -> usize;
}
extern "C" {
    pub fn mbsnrtowcs(
        __dst: *mut wchar_t,
        __src: *mut *const ::std::os::raw::c_char,
        __nmc: usize,
        __len: usize,
        __ps: *mut mbstate_t,
    ) -> usize;
}
extern "C" {
    pub fn wcsnrtombs(
        __dst: *mut ::std::os::raw::c_char,
        __src: *mut *const wchar_t,
        __nwc: usize,
        __len: usize,
        __ps: *mut mbstate_t,
    ) -> usize;
}
extern "C" {
    pub fn wcwidth(__c: wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcswidth(__s: *const wchar_t, __n: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcstod(__nptr: *const wchar_t, __endptr: *mut *mut wchar_t) -> f64;
}
extern "C" {
    pub fn wcstof(__nptr: *const wchar_t, __endptr: *mut *mut wchar_t) -> f32;
}
extern "C" {
    pub fn wcstold(__nptr: *const wchar_t, __endptr: *mut *mut wchar_t) -> f64;
}
extern "C" {
    pub fn wcstof32(__nptr: *const wchar_t, __endptr: *mut *mut wchar_t) -> _Float32;
}
extern "C" {
    pub fn wcstof64(__nptr: *const wchar_t, __endptr: *mut *mut wchar_t) -> _Float64;
}
extern "C" {
    pub fn wcstof32x(__nptr: *const wchar_t, __endptr: *mut *mut wchar_t) -> _Float32x;
}
extern "C" {
    pub fn wcstof64x(__nptr: *const wchar_t, __endptr: *mut *mut wchar_t) -> _Float64x;
}
extern "C" {
    pub fn wcstoll_l(
        __nptr: *const wchar_t,
        __endptr: *mut *mut wchar_t,
        __base: ::std::os::raw::c_int,
        __loc: locale_t,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn wcstoull_l(
        __nptr: *const wchar_t,
        __endptr: *mut *mut wchar_t,
        __base: ::std::os::raw::c_int,
        __loc: locale_t,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn wcstod_l(__nptr: *const wchar_t, __endptr: *mut *mut wchar_t, __loc: locale_t) -> f64;
}
extern "C" {
    pub fn wcstof_l(__nptr: *const wchar_t, __endptr: *mut *mut wchar_t, __loc: locale_t) -> f32;
}
extern "C" {
    pub fn wcstold_l(__nptr: *const wchar_t, __endptr: *mut *mut wchar_t, __loc: locale_t) -> f64;
}
extern "C" {
    pub fn wcstof32_l(
        __nptr: *const wchar_t,
        __endptr: *mut *mut wchar_t,
        __loc: locale_t,
    ) -> _Float32;
}
extern "C" {
    pub fn wcstof64_l(
        __nptr: *const wchar_t,
        __endptr: *mut *mut wchar_t,
        __loc: locale_t,
    ) -> _Float64;
}
extern "C" {
    pub fn wcstof32x_l(
        __nptr: *const wchar_t,
        __endptr: *mut *mut wchar_t,
        __loc: locale_t,
    ) -> _Float32x;
}
extern "C" {
    pub fn wcstof64x_l(
        __nptr: *const wchar_t,
        __endptr: *mut *mut wchar_t,
        __loc: locale_t,
    ) -> _Float64x;
}
extern "C" {
    pub fn wcpcpy(__dest: *mut wchar_t, __src: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn wcpncpy(__dest: *mut wchar_t, __src: *const wchar_t, __n: usize) -> *mut wchar_t;
}
extern "C" {
    pub fn open_wmemstream(__bufloc: *mut *mut wchar_t, __sizeloc: *mut usize) -> *mut FILE;
}
extern "C" {
    pub fn fwide(__fp: *mut FILE, __mode: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fwprintf(__stream: *mut FILE, __format: *const wchar_t, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wprintf(__format: *const wchar_t, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn swprintf(
        __s: *mut wchar_t,
        __n: usize,
        __format: *const wchar_t,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vfwprintf(
        __s: *mut FILE,
        __format: *const wchar_t,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vwprintf(__format: *const wchar_t, __arg: *mut __va_list_tag) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vswprintf(
        __s: *mut wchar_t,
        __n: usize,
        __format: *const wchar_t,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fwscanf(__stream: *mut FILE, __format: *const wchar_t, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wscanf(__format: *const wchar_t, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn swscanf(__s: *const wchar_t, __format: *const wchar_t, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vfwscanf(
        __s: *mut FILE,
        __format: *const wchar_t,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vwscanf(__format: *const wchar_t, __arg: *mut __va_list_tag) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vswscanf(
        __s: *const wchar_t,
        __format: *const wchar_t,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetwc(__stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn getwc(__stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn getwchar() -> wint_t;
}
extern "C" {
    pub fn fputwc(__wc: wchar_t, __stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn putwc(__wc: wchar_t, __stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn putwchar(__wc: wchar_t) -> wint_t;
}
extern "C" {
    pub fn fgetws(
        __ws: *mut wchar_t,
        __n: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> *mut wchar_t;
}
extern "C" {
    pub fn fputws(__ws: *const wchar_t, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ungetwc(__wc: wint_t, __stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn getwc_unlocked(__stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn getwchar_unlocked() -> wint_t;
}
extern "C" {
    pub fn fgetwc_unlocked(__stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn fputwc_unlocked(__wc: wchar_t, __stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn putwc_unlocked(__wc: wchar_t, __stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn putwchar_unlocked(__wc: wchar_t) -> wint_t;
}
extern "C" {
    pub fn fgetws_unlocked(
        __ws: *mut wchar_t,
        __n: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> *mut wchar_t;
}
extern "C" {
    pub fn fputws_unlocked(__ws: *const wchar_t, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcsftime(
        __s: *mut wchar_t,
        __maxsize: usize,
        __format: *const wchar_t,
        __tp: *const tm,
    ) -> usize;
}
extern "C" {
    pub fn wcsftime_l(
        __s: *mut wchar_t,
        __maxsize: usize,
        __format: *const wchar_t,
        __tp: *const tm,
        __loc: locale_t,
    ) -> usize;
}
extern "C" {
    pub fn iswalnum(__wc: wint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswalpha(__wc: wint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswcntrl(__wc: wint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswdigit(__wc: wint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswgraph(__wc: wint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswlower(__wc: wint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswprint(__wc: wint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswpunct(__wc: wint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswspace(__wc: wint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswupper(__wc: wint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswxdigit(__wc: wint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswblank(__wc: wint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wctype(__property: *const ::std::os::raw::c_char) -> wctype_t;
}
extern "C" {
    pub fn iswctype(__wc: wint_t, __desc: wctype_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn towlower(__wc: wint_t) -> wint_t;
}
extern "C" {
    pub fn towupper(__wc: wint_t) -> wint_t;
}
pub type wctrans_t = *const __int32_t;
extern "C" {
    pub fn wctrans(__property: *const ::std::os::raw::c_char) -> wctrans_t;
}
extern "C" {
    pub fn towctrans(__wc: wint_t, __desc: wctrans_t) -> wint_t;
}
extern "C" {
    pub fn iswalnum_l(__wc: wint_t, __locale: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswalpha_l(__wc: wint_t, __locale: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswcntrl_l(__wc: wint_t, __locale: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswdigit_l(__wc: wint_t, __locale: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswgraph_l(__wc: wint_t, __locale: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswlower_l(__wc: wint_t, __locale: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswprint_l(__wc: wint_t, __locale: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswpunct_l(__wc: wint_t, __locale: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswspace_l(__wc: wint_t, __locale: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswupper_l(__wc: wint_t, __locale: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswxdigit_l(__wc: wint_t, __locale: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswblank_l(__wc: wint_t, __locale: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wctype_l(__property: *const ::std::os::raw::c_char, __locale: locale_t) -> wctype_t;
}
extern "C" {
    pub fn iswctype_l(__wc: wint_t, __desc: wctype_t, __locale: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn towlower_l(__wc: wint_t, __locale: locale_t) -> wint_t;
}
extern "C" {
    pub fn towupper_l(__wc: wint_t, __locale: locale_t) -> wint_t;
}
extern "C" {
    pub fn wctrans_l(__property: *const ::std::os::raw::c_char, __locale: locale_t) -> wctrans_t;
}
extern "C" {
    pub fn towctrans_l(__wc: wint_t, __desc: wctrans_t, __locale: locale_t) -> wint_t;
}
extern "C" {
    pub fn access(
        __name: *const ::std::os::raw::c_char,
        __type: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn euidaccess(
        __name: *const ::std::os::raw::c_char,
        __type: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn eaccess(
        __name: *const ::std::os::raw::c_char,
        __type: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faccessat(
        __fd: ::std::os::raw::c_int,
        __file: *const ::std::os::raw::c_char,
        __type: ::std::os::raw::c_int,
        __flag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lseek64(
        __fd: ::std::os::raw::c_int,
        __offset: __off64_t,
        __whence: ::std::os::raw::c_int,
    ) -> __off64_t;
}
extern "C" {
    pub fn read(
        __fd: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_void,
        __nbytes: usize,
    ) -> isize;
}
extern "C" {
    pub fn write(
        __fd: ::std::os::raw::c_int,
        __buf: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> isize;
}
extern "C" {
    pub fn pread(
        __fd: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_void,
        __nbytes: usize,
        __offset: __off_t,
    ) -> isize;
}
extern "C" {
    pub fn pwrite(
        __fd: ::std::os::raw::c_int,
        __buf: *const ::std::os::raw::c_void,
        __n: usize,
        __offset: __off_t,
    ) -> isize;
}
extern "C" {
    pub fn pread64(
        __fd: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_void,
        __nbytes: usize,
        __offset: __off64_t,
    ) -> isize;
}
extern "C" {
    pub fn pwrite64(
        __fd: ::std::os::raw::c_int,
        __buf: *const ::std::os::raw::c_void,
        __n: usize,
        __offset: __off64_t,
    ) -> isize;
}
extern "C" {
    pub fn pipe2(
        __pipedes: *mut ::std::os::raw::c_int,
        __flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn alarm(__seconds: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn sleep(__seconds: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn ualarm(__value: __useconds_t, __interval: __useconds_t) -> __useconds_t;
}
extern "C" {
    pub fn usleep(__useconds: __useconds_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pause() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn chown(
        __file: *const ::std::os::raw::c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fchown(
        __fd: ::std::os::raw::c_int,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lchown(
        __file: *const ::std::os::raw::c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fchownat(
        __fd: ::std::os::raw::c_int,
        __file: *const ::std::os::raw::c_char,
        __owner: __uid_t,
        __group: __gid_t,
        __flag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn chdir(__path: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fchdir(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getcwd(__buf: *mut ::std::os::raw::c_char, __size: usize)
        -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn get_current_dir_name() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn getwd(__buf: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn dup(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn dup3(
        __fd: ::std::os::raw::c_int,
        __fd2: ::std::os::raw::c_int,
        __flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__environ"]
    pub static mut __environ: *mut *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}environ"]
    pub static mut environ: *mut *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn execve(
        __path: *const ::std::os::raw::c_char,
        __argv: *const *mut ::std::os::raw::c_char,
        __envp: *const *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fexecve(
        __fd: ::std::os::raw::c_int,
        __argv: *const *mut ::std::os::raw::c_char,
        __envp: *const *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn execv(
        __path: *const ::std::os::raw::c_char,
        __argv: *const *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn execle(
        __path: *const ::std::os::raw::c_char,
        __arg: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn execl(
        __path: *const ::std::os::raw::c_char,
        __arg: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn execvp(
        __file: *const ::std::os::raw::c_char,
        __argv: *const *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn execlp(
        __file: *const ::std::os::raw::c_char,
        __arg: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn execvpe(
        __file: *const ::std::os::raw::c_char,
        __argv: *const *mut ::std::os::raw::c_char,
        __envp: *const *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nice(__inc: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _exit(__status: ::std::os::raw::c_int);
}
extern "C" {
    pub fn pathconf(
        __path: *const ::std::os::raw::c_char,
        __name: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn fpathconf(
        __fd: ::std::os::raw::c_int,
        __name: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn sysconf(__name: ::std::os::raw::c_int) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn confstr(
        __name: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> usize;
}
extern "C" {
    pub fn getpid() -> __pid_t;
}
extern "C" {
    pub fn getppid() -> __pid_t;
}
extern "C" {
    pub fn getpgrp() -> __pid_t;
}
extern "C" {
    pub fn __getpgid(__pid: __pid_t) -> __pid_t;
}
extern "C" {
    pub fn getpgid(__pid: __pid_t) -> __pid_t;
}
extern "C" {
    pub fn setpgid(__pid: __pid_t, __pgid: __pid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setpgrp() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setsid() -> __pid_t;
}
extern "C" {
    pub fn getsid(__pid: __pid_t) -> __pid_t;
}
extern "C" {
    pub fn getuid() -> __uid_t;
}
extern "C" {
    pub fn geteuid() -> __uid_t;
}
extern "C" {
    pub fn getgid() -> __gid_t;
}
extern "C" {
    pub fn getegid() -> __gid_t;
}
extern "C" {
    pub fn group_member(__gid: __gid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setuid(__uid: __uid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setreuid(__ruid: __uid_t, __euid: __uid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn seteuid(__uid: __uid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setgid(__gid: __gid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setregid(__rgid: __gid_t, __egid: __gid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setegid(__gid: __gid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getresuid(
        __ruid: *mut __uid_t,
        __euid: *mut __uid_t,
        __suid: *mut __uid_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getresgid(
        __rgid: *mut __gid_t,
        __egid: *mut __gid_t,
        __sgid: *mut __gid_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setresuid(__ruid: __uid_t, __euid: __uid_t, __suid: __uid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setresgid(__rgid: __gid_t, __egid: __gid_t, __sgid: __gid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fork() -> __pid_t;
}
extern "C" {
    pub fn vfork() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ttyname_r(
        __fd: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isatty(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ttyslot() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn link(
        __from: *const ::std::os::raw::c_char,
        __to: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn linkat(
        __fromfd: ::std::os::raw::c_int,
        __from: *const ::std::os::raw::c_char,
        __tofd: ::std::os::raw::c_int,
        __to: *const ::std::os::raw::c_char,
        __flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn symlink(
        __from: *const ::std::os::raw::c_char,
        __to: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn readlink(
        __path: *const ::std::os::raw::c_char,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> isize;
}
extern "C" {
    pub fn symlinkat(
        __from: *const ::std::os::raw::c_char,
        __tofd: ::std::os::raw::c_int,
        __to: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn readlinkat(
        __fd: ::std::os::raw::c_int,
        __path: *const ::std::os::raw::c_char,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> isize;
}
extern "C" {
    pub fn unlink(__name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn unlinkat(
        __fd: ::std::os::raw::c_int,
        __name: *const ::std::os::raw::c_char,
        __flag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rmdir(__path: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tcgetpgrp(__fd: ::std::os::raw::c_int) -> __pid_t;
}
extern "C" {
    pub fn tcsetpgrp(__fd: ::std::os::raw::c_int, __pgrp_id: __pid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getlogin() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn getlogin_r(
        __name: *mut ::std::os::raw::c_char,
        __name_len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setlogin(__name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}optarg"]
    pub static mut optarg: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}optind"]
    pub static mut optind: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}opterr"]
    pub static mut opterr: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}optopt"]
    pub static mut optopt: ::std::os::raw::c_int;
}
extern "C" {
    pub fn getopt(
        ___argc: ::std::os::raw::c_int,
        ___argv: *const *mut ::std::os::raw::c_char,
        __shortopts: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gethostname(__name: *mut ::std::os::raw::c_char, __len: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sethostname(
        __name: *const ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sethostid(__id: ::std::os::raw::c_long) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getdomainname(
        __name: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setdomainname(
        __name: *const ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vhangup() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn revoke(__file: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn profil(
        __sample_buffer: *mut ::std::os::raw::c_ushort,
        __size: usize,
        __offset: usize,
        __scale: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn acct(__name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getusershell() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn endusershell();
}
extern "C" {
    pub fn setusershell();
}
extern "C" {
    pub fn daemon(
        __nochdir: ::std::os::raw::c_int,
        __noclose: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn chroot(__path: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getpass(__prompt: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fsync(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn syncfs(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gethostid() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn sync();
}
extern "C" {
    pub fn getpagesize() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn truncate(
        __file: *const ::std::os::raw::c_char,
        __length: __off_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn truncate64(
        __file: *const ::std::os::raw::c_char,
        __length: __off64_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftruncate(__fd: ::std::os::raw::c_int, __length: __off_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftruncate64(__fd: ::std::os::raw::c_int, __length: __off64_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn brk(__addr: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sbrk(__delta: isize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn syscall(__sysno: ::std::os::raw::c_long, ...) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn lockf(
        __fd: ::std::os::raw::c_int,
        __cmd: ::std::os::raw::c_int,
        __len: __off_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lockf64(
        __fd: ::std::os::raw::c_int,
        __cmd: ::std::os::raw::c_int,
        __len: __off64_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn copy_file_range(
        __infd: ::std::os::raw::c_int,
        __pinoff: *mut __off64_t,
        __outfd: ::std::os::raw::c_int,
        __poutoff: *mut __off64_t,
        __length: usize,
        __flags: ::std::os::raw::c_uint,
    ) -> isize;
}
extern "C" {
    pub fn fdatasync(__fildes: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypt(
        __key: *const ::std::os::raw::c_char,
        __salt: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn swab(
        __from: *const ::std::os::raw::c_void,
        __to: *mut ::std::os::raw::c_void,
        __n: isize,
    );
}
extern "C" {
    pub fn getentropy(
        __buffer: *mut ::std::os::raw::c_void,
        __length: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __sysv_signal(__sig: ::std::os::raw::c_int, __handler: __sighandler_t)
        -> __sighandler_t;
}
extern "C" {
    pub fn sysv_signal(__sig: ::std::os::raw::c_int, __handler: __sighandler_t) -> __sighandler_t;
}
extern "C" {
    pub fn signal(__sig: ::std::os::raw::c_int, __handler: __sighandler_t) -> __sighandler_t;
}
extern "C" {
    pub fn kill(__pid: __pid_t, __sig: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn killpg(__pgrp: __pid_t, __sig: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn raise(__sig: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ssignal(__sig: ::std::os::raw::c_int, __handler: __sighandler_t) -> __sighandler_t;
}
extern "C" {
    pub fn gsignal(__sig: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn psignal(__sig: ::std::os::raw::c_int, __s: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn psiginfo(__pinfo: *const siginfo_t, __s: *const ::std::os::raw::c_char);
}
extern "C" {
    #[link_name = "\u{1}__xpg_sigpause"]
    pub fn sigpause(__sig: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sigblock(__mask: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sigsetmask(__mask: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn siggetmask() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sigemptyset(__set: *mut sigset_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sigfillset(__set: *mut sigset_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sigaddset(__set: *mut sigset_t, __signo: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sigdelset(__set: *mut sigset_t, __signo: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sigismember(
        __set: *const sigset_t,
        __signo: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sigisemptyset(__set: *const sigset_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sigandset(
        __set: *mut sigset_t,
        __left: *const sigset_t,
        __right: *const sigset_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sigorset(
        __set: *mut sigset_t,
        __left: *const sigset_t,
        __right: *const sigset_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sigprocmask(
        __how: ::std::os::raw::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sigsuspend(__set: *const sigset_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sigaction(
        __sig: ::std::os::raw::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sigpending(__set: *mut sigset_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sigwait(
        __set: *const sigset_t,
        __sig: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sigwaitinfo(__set: *const sigset_t, __info: *mut siginfo_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sigtimedwait(
        __set: *const sigset_t,
        __info: *mut siginfo_t,
        __timeout: *const timespec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sigqueue(
        __pid: __pid_t,
        __sig: ::std::os::raw::c_int,
        __val: sigval,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_sys_siglist"]
    pub static mut _sys_siglist: [*const ::std::os::raw::c_char; 65usize];
}
extern "C" {
    #[link_name = "\u{1}sys_siglist"]
    pub static mut sys_siglist: [*const ::std::os::raw::c_char; 65usize];
}
extern "C" {
    pub fn sigstack(__ss: *mut sigstack, __oss: *mut sigstack) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sighold(__sig: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sigrelse(__sig: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sigignore(__sig: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sigset(__sig: ::std::os::raw::c_int, __disp: __sighandler_t) -> __sighandler_t;
}
extern "C" {
    pub fn pthread_sigmask(
        __how: ::std::os::raw::c_int,
        __newmask: *const __sigset_t,
        __oldmask: *mut __sigset_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_kill(
        __threadid: pthread_t,
        __signo: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_sigqueue(
        __threadid: pthread_t,
        __signo: ::std::os::raw::c_int,
        __value: sigval,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __libc_current_sigrtmin() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __libc_current_sigrtmax() -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union command__bindgen_ty_1 {
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
    _bindgen_union_align: u64,
}
extern "C" {
    pub fn copy_function_def_contents(
        arg1: *mut FUNCTION_DEF,
        arg2: *mut FUNCTION_DEF,
    ) -> *mut FUNCTION_DEF;
}
extern "C" {
    pub fn copy_function_def(arg1: *mut FUNCTION_DEF) -> *mut FUNCTION_DEF;
}
extern "C" {
    pub fn copy_word(arg1: *mut WordDesc) -> *mut WordDesc;
}
extern "C" {
    pub fn copy_WordList(arg1: *mut WordList) -> *mut WordList;
}
extern "C" {
    pub fn copy_command(arg1: *mut COMMAND) -> *mut COMMAND;
}

extern "C" {
    pub fn select(
        __nfds: ::std::os::raw::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pselect(
        __nfds: ::std::os::raw::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *const timespec,
        __sigmask: *const __sigset_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn imaxabs(__n: intmax_t) -> intmax_t;
}
extern "C" {
    pub fn imaxdiv(__numer: intmax_t, __denom: intmax_t) -> imaxdiv_t;
}
extern "C" {
    pub fn strtoimax(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> intmax_t;
}
extern "C" {
    pub fn strtoumax(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> uintmax_t;
}
extern "C" {
    pub fn wcstoimax(
        __nptr: *const __gwchar_t,
        __endptr: *mut *mut __gwchar_t,
        __base: ::std::os::raw::c_int,
    ) -> intmax_t;
}
extern "C" {
    pub fn wcstoumax(
        __nptr: *const __gwchar_t,
        __endptr: *mut *mut __gwchar_t,
        __base: ::std::os::raw::c_int,
    ) -> uintmax_t;
}
extern "C" {
    pub fn __ctype_tolower_loc() -> *mut *const __int32_t;
}
extern "C" {
    pub fn __ctype_toupper_loc() -> *mut *const __int32_t;
}
extern "C" {
    pub fn isalnum(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isalpha(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iscntrl(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isdigit(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn islower(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isgraph(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isprint(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ispunct(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isspace(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isupper(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isxdigit(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tolower(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn toupper(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isblank(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isctype(
        __c: ::std::os::raw::c_int,
        __mask: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isascii(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn toascii(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _toupper(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _tolower(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isalnum_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isalpha_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iscntrl_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isdigit_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn islower_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isgraph_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isprint_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ispunct_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isspace_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isupper_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isxdigit_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isblank_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __tolower_l(__c: ::std::os::raw::c_int, __l: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tolower_l(__c: ::std::os::raw::c_int, __l: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __toupper_l(__c: ::std::os::raw::c_int, __l: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn toupper_l(__c: ::std::os::raw::c_int, __l: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn settimeofday(__tv: *const timeval, __tz: *const timezone) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn adjtime(__delta: *const timeval, __olddelta: *mut timeval) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getitimer(__which: __itimer_which_t, __value: *mut itimerval) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setitimer(
        __which: __itimer_which_t,
        __new: *const itimerval,
        __old: *mut itimerval,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn utimes(
        __file: *const ::std::os::raw::c_char,
        __tvp: *const timeval,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lutimes(
        __file: *const ::std::os::raw::c_char,
        __tvp: *const timeval,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn futimes(__fd: ::std::os::raw::c_int, __tvp: *const timeval) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn futimesat(
        __fd: ::std::os::raw::c_int,
        __file: *const ::std::os::raw::c_char,
        __tvp: *const timeval,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn prlimit(
        __pid: __pid_t,
        __resource: __rlimit_resource,
        __new_limit: *const rlimit,
        __old_limit: *mut rlimit,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn prlimit64(
        __pid: __pid_t,
        __resource: __rlimit_resource,
        __new_limit: *const rlimit64,
        __old_limit: *mut rlimit64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getrlimit(
        __resource: __rlimit_resource_t,
        __rlimits: *mut rlimit,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getrlimit64(
        __resource: __rlimit_resource_t,
        __rlimits: *mut rlimit64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setrlimit(
        __resource: __rlimit_resource_t,
        __rlimits: *const rlimit,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setrlimit64(
        __resource: __rlimit_resource_t,
        __rlimits: *const rlimit64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getrusage(__who: __rusage_who_t, __usage: *mut rusage) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getpriority(__which: __priority_which_t, __who: id_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setpriority(
        __which: __priority_which_t,
        __who: id_t,
        __prio: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn memcpy(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memccpy(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memcmp(
        __s1: *const ::std::os::raw::c_void,
        __s2: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn memchr(
        __s: *const ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn rawmemchr(
        __s: *const ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memrchr(
        __s: *const ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn strcpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strncat(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strcoll(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strxfrm(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strcoll_l(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __l: locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strxfrm_l(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: usize,
        __l: locale_t,
    ) -> usize;
}
extern "C" {
    pub fn strdup(__s: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strndup(
        __string: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strchr(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strchrnul(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strcspn(
        __s: *const ::std::os::raw::c_char,
        __reject: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strspn(
        __s: *const ::std::os::raw::c_char,
        __accept: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ulong;
}

extern "C" {
    pub fn strstr(
        __haystack: *const ::std::os::raw::c_char,
        __needle: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strtok(
        __s: *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __strtok_r(
        __s: *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
        __save_ptr: *mut *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strtok_r(
        __s: *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
        __save_ptr: *mut *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strcasestr(
        __haystack: *const ::std::os::raw::c_char,
        __needle: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn memmem(
        __haystack: *const ::std::os::raw::c_void,
        __haystacklen: usize,
        __needle: *const ::std::os::raw::c_void,
        __needlelen: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn __mempcpy(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn mempcpy(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn strnlen(__string: *const ::std::os::raw::c_char, __maxlen: usize) -> usize;
}
extern "C" {
    pub fn strerror(__errnum: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strerror_r(
        __errnum: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strerror_l(
        __errnum: ::std::os::raw::c_int,
        __l: locale_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn bcmp(
        __s1: *const ::std::os::raw::c_void,
        __s2: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bcopy(
        __src: *const ::std::os::raw::c_void,
        __dest: *mut ::std::os::raw::c_void,
        __n: usize,
    );
}
extern "C" {
    pub fn bzero(__s: *mut ::std::os::raw::c_void, __n: usize);
}
extern "C" {
    pub fn index(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn rindex(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ffs(__i: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ffsl(__l: ::std::os::raw::c_long) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ffsll(__ll: ::std::os::raw::c_longlong) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strcasecmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strcasecmp_l(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __loc: locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strncasecmp_l(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __n: usize,
        __loc: locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn explicit_bzero(__s: *mut ::std::os::raw::c_void, __n: usize);
}
extern "C" {
    pub fn strsep(
        __stringp: *mut *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strsignal(__sig: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __stpcpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn stpcpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __stpncpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn stpncpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strverscmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strfry(__string: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn memfrob(__s: *mut ::std::os::raw::c_void, __n: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn basename(__filename: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
// extern "C" {
//     pub fn __ctype_get_mb_cur_max() -> usize;
// }
extern "C" {
    pub fn atof(__nptr: *const ::std::os::raw::c_char) -> f64;
}
extern "C" {
    pub fn atoi(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn atol(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn atoll(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtod(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> f64;
}
extern "C" {
    pub fn strtof(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> f32;
}
extern "C" {
    pub fn strtof32(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> _Float32;
}
extern "C" {
    pub fn strtof64(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> _Float64;
}
extern "C" {
    pub fn strtof32x(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> _Float32x;
}
extern "C" {
    pub fn strtof64x(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> _Float64x;
}
extern "C" {
    pub fn strtoul(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strtoq(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtouq(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn strtoll(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtoull(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn strfromd(
        __dest: *mut ::std::os::raw::c_char,
        __size: usize,
        __format: *const ::std::os::raw::c_char,
        __f: f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strfromf(
        __dest: *mut ::std::os::raw::c_char,
        __size: usize,
        __format: *const ::std::os::raw::c_char,
        __f: f32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strfroml(
        __dest: *mut ::std::os::raw::c_char,
        __size: usize,
        __format: *const ::std::os::raw::c_char,
        __f: f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strfromf32(
        __dest: *mut ::std::os::raw::c_char,
        __size: usize,
        __format: *const ::std::os::raw::c_char,
        __f: _Float32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strfromf64(
        __dest: *mut ::std::os::raw::c_char,
        __size: usize,
        __format: *const ::std::os::raw::c_char,
        __f: _Float64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strfromf32x(
        __dest: *mut ::std::os::raw::c_char,
        __size: usize,
        __format: *const ::std::os::raw::c_char,
        __f: _Float32x,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strfromf64x(
        __dest: *mut ::std::os::raw::c_char,
        __size: usize,
        __format: *const ::std::os::raw::c_char,
        __f: _Float64x,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strtol_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
        __loc: locale_t,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn strtoul_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
        __loc: locale_t,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strtoll_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
        __loc: locale_t,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtoull_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
        __loc: locale_t,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn strtod_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __loc: locale_t,
    ) -> f64;
}
extern "C" {
    pub fn strtof_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __loc: locale_t,
    ) -> f32;
}
extern "C" {
    pub fn strtold_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __loc: locale_t,
    ) -> f64;
}
extern "C" {
    pub fn strtof32_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __loc: locale_t,
    ) -> _Float32;
}
extern "C" {
    pub fn strtof64_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __loc: locale_t,
    ) -> _Float64;
}
extern "C" {
    pub fn strtof32x_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __loc: locale_t,
    ) -> _Float32x;
}
extern "C" {
    pub fn strtof64x_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __loc: locale_t,
    ) -> _Float64x;
}
extern "C" {
    pub fn l64a(__n: ::std::os::raw::c_long) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn a64l(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn random() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn srandom(__seed: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn initstate(
        __seed: ::std::os::raw::c_uint,
        __statebuf: *mut ::std::os::raw::c_char,
        __statelen: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn setstate(__statebuf: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn random_r(__buf: *mut random_data, __result: *mut i32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srandom_r(
        __seed: ::std::os::raw::c_uint,
        __buf: *mut random_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn initstate_r(
        __seed: ::std::os::raw::c_uint,
        __statebuf: *mut ::std::os::raw::c_char,
        __statelen: usize,
        __buf: *mut random_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setstate_r(
        __statebuf: *mut ::std::os::raw::c_char,
        __buf: *mut random_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rand() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srand(__seed: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn rand_r(__seed: *mut ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn drand48() -> f64;
}
extern "C" {
    pub fn erand48(__xsubi: *mut ::std::os::raw::c_ushort) -> f64;
}
extern "C" {
    pub fn lrand48() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn nrand48(__xsubi: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn mrand48() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn jrand48(__xsubi: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn srand48(__seedval: ::std::os::raw::c_long);
}
extern "C" {
    pub fn seed48(__seed16v: *mut ::std::os::raw::c_ushort) -> *mut ::std::os::raw::c_ushort;
}
extern "C" {
    pub fn lcong48(__param: *mut ::std::os::raw::c_ushort);
}
extern "C" {
    pub fn drand48_r(__buffer: *mut drand48_data, __result: *mut f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn erand48_r(
        __xsubi: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lrand48_r(
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nrand48_r(
        __xsubi: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mrand48_r(
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jrand48_r(
        __xsubi: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srand48_r(
        __seedval: ::std::os::raw::c_long,
        __buffer: *mut drand48_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn seed48_r(
        __seed16v: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lcong48_r(
        __param: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn malloc(__size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn calloc(__nmemb: usize, __size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn realloc(
        __ptr: *mut ::std::os::raw::c_void,
        __size: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn reallocarray(
        __ptr: *mut ::std::os::raw::c_void,
        __nmemb: usize,
        __size: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn free(__ptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn alloca(__size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn valloc(__size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn posix_memalign(
        __memptr: *mut *mut ::std::os::raw::c_void,
        __alignment: usize,
        __size: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn aligned_alloc(__alignment: usize, __size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn abort();
}
extern "C" {
    pub fn atexit(__func: ::core::option::Option<unsafe extern "C" fn()>) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn at_quick_exit(
        __func: ::core::option::Option<unsafe extern "C" fn()>,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn on_exit(
        __func: ::core::option::Option<
            unsafe extern "C" fn(
                __status: ::std::os::raw::c_int,
                __arg: *mut ::std::os::raw::c_void,
            ),
        >,
        __arg: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn exit(__status: ::std::os::raw::c_int);
}
extern "C" {
    pub fn quick_exit(__status: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _Exit(__status: ::std::os::raw::c_int);
}
extern "C" {
    pub fn getenv(__name: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn secure_getenv(__name: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn putenv(__string: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setenv(
        __name: *const ::std::os::raw::c_char,
        __value: *const ::std::os::raw::c_char,
        __replace: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn unsetenv(__name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clearenv() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mktemp(__template: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mkstemp(__template: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkstemp64(__template: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkstemps(
        __template: *mut ::std::os::raw::c_char,
        __suffixlen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkstemps64(
        __template: *mut ::std::os::raw::c_char,
        __suffixlen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkdtemp(__template: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mkostemp(
        __template: *mut ::std::os::raw::c_char,
        __flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkostemp64(
        __template: *mut ::std::os::raw::c_char,
        __flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkostemps(
        __template: *mut ::std::os::raw::c_char,
        __suffixlen: ::std::os::raw::c_int,
        __flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkostemps64(
        __template: *mut ::std::os::raw::c_char,
        __suffixlen: ::std::os::raw::c_int,
        __flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn system(__command: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn canonicalize_file_name(
        __name: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn realpath(
        __name: *const ::std::os::raw::c_char,
        __resolved: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn bsearch(
        __key: *const ::std::os::raw::c_void,
        __base: *const ::std::os::raw::c_void,
        __nmemb: usize,
        __size: usize,
        __compar: __compar_fn_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn qsort_r(
        __base: *mut ::std::os::raw::c_void,
        __nmemb: usize,
        __size: usize,
        __compar: __compar_d_fn_t,
        __arg: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn abs(__x: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn labs(__x: ::std::os::raw::c_long) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llabs(__x: ::std::os::raw::c_longlong) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn div(__numer: ::std::os::raw::c_int, __denom: ::std::os::raw::c_int) -> div_t;
}
extern "C" {
    pub fn ldiv(__numer: ::std::os::raw::c_long, __denom: ::std::os::raw::c_long) -> ldiv_t;
}
extern "C" {
    pub fn lldiv(
        __numer: ::std::os::raw::c_longlong,
        __denom: ::std::os::raw::c_longlong,
    ) -> lldiv_t;
}
extern "C" {
    pub fn ecvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fcvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn gcvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn qecvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn qfcvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn qgcvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ecvt_r(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fcvt_r(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn qecvt_r(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn qfcvt_r(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbtowc(
        __pwc: *mut wchar_t,
        __s: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wctomb(__s: *mut ::std::os::raw::c_char, __wchar: wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbstowcs(__pwcs: *mut wchar_t, __s: *const ::std::os::raw::c_char, __n: usize) -> usize;
}
extern "C" {
    pub fn wcstombs(__s: *mut ::std::os::raw::c_char, __pwcs: *const wchar_t, __n: usize) -> usize;
}
extern "C" {
    pub fn rpmatch(__response: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getsubopt(
        __optionp: *mut *mut ::std::os::raw::c_char,
        __tokens: *const *mut ::std::os::raw::c_char,
        __valuep: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn posix_openpt(__oflag: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn grantpt(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn unlockpt(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ptsname(__fd: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ptsname_r(
        __fd: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getpt() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getloadavg(__loadavg: *mut f64, __nelem: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmalloc(arg1: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xrealloc(arg1: *mut ::std::os::raw::c_void, arg2: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xfree(arg1: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn posix_initialize(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn num_posix_options() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn get_posix_options(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn set_posix_options(arg1: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn save_posix_options();
}
extern "C" {
    pub fn string_to_rlimtype(arg1: *mut ::std::os::raw::c_char) -> rlim_t;
}
extern "C" {
    pub fn print_rlimtype(arg1: rlim_t, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn all_digits(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn legal_identifier(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn importable_function_name(
        arg1: *const ::std::os::raw::c_char,
        arg2: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn exportable_function_name(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn check_identifier(
        arg1: *mut WordDesc,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn check_selfref(
        arg1: *const ::std::os::raw::c_char,
        arg2: *mut ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn legal_alias_name(
        arg1: *const ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn line_isblank(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn assignment(
        arg1: *const ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sh_setclexec(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sh_validfd(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fd_ispipe(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn check_dev_tty();
}
extern "C" {
    pub fn move_to_high_fd(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn check_binary_file(
        arg1: *const ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sh_openpipe(arg1: *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sh_closepipe(arg1: *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn file_exists(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn file_iswdir(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn path_dot_or_dotdot(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn absolute_pathname(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn absolute_program(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn base_pathname(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn full_pathname(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn polite_directory_format(
        arg1: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn trim_pathname(
        arg1: *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn printable_filename(
        arg1: *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn extract_colon_unit(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tilde_initialize();
}
extern "C" {
    pub fn bash_tilde_find_word(
        arg1: *const ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn bash_tilde_expand(
        arg1: *const ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn get_group_list(arg1: *mut ::std::os::raw::c_int) -> *mut *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn get_group_array(arg1: *mut ::std::os::raw::c_int) -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn conf_standard_path() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn default_columns() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn array_create() -> *mut ARRAY;
}
extern "C" {
    pub fn array_flush(arg1: *mut ARRAY);
}
extern "C" {
    pub fn array_dispose(arg1: *mut ARRAY);
}
extern "C" {
    pub fn array_copy(arg1: *mut ARRAY) -> *mut ARRAY;
}
extern "C" {
    pub fn array_slice(
        arg1: *mut ARRAY,
        arg2: *mut ARRAY_ELEMENT,
        arg3: *mut ARRAY_ELEMENT,
    ) -> *mut ARRAY;
}
extern "C" {
    pub fn array_walk(arg1: *mut ARRAY, arg2: sh_ae_map_func_t, arg3: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn array_shift(
        arg1: *mut ARRAY,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> *mut ARRAY_ELEMENT;
}
extern "C" {
    pub fn array_rshift(
        arg1: *mut ARRAY,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn array_unshift_element(arg1: *mut ARRAY) -> *mut ARRAY_ELEMENT;
}
extern "C" {
    pub fn array_shift_element(
        arg1: *mut ARRAY,
        arg2: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn array_quote(arg1: *mut ARRAY) -> *mut ARRAY;
}
extern "C" {
    pub fn array_quote_escapes(arg1: *mut ARRAY) -> *mut ARRAY;
}
extern "C" {
    pub fn array_dequote(arg1: *mut ARRAY) -> *mut ARRAY;
}
extern "C" {
    pub fn array_dequote_escapes(arg1: *mut ARRAY) -> *mut ARRAY;
}
extern "C" {
    pub fn array_remove_quoted_nulls(arg1: *mut ARRAY) -> *mut ARRAY;
}
extern "C" {
    pub fn array_subrange(
        arg1: *mut ARRAY,
        arg2: arrayind_t,
        arg3: arrayind_t,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
        arg6: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn array_patsub(
        arg1: *mut ARRAY,
        arg2: *mut ::std::os::raw::c_char,
        arg3: *mut ::std::os::raw::c_char,
        arg4: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn array_modcase(
        arg1: *mut ARRAY,
        arg2: *mut ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn array_create_element(
        arg1: arrayind_t,
        arg2: *mut ::std::os::raw::c_char,
    ) -> *mut ARRAY_ELEMENT;
}
extern "C" {
    pub fn array_copy_element(arg1: *mut ARRAY_ELEMENT) -> *mut ARRAY_ELEMENT;
}
extern "C" {
    pub fn array_dispose_element(arg1: *mut ARRAY_ELEMENT);
}
extern "C" {
    pub fn array_insert(
        arg1: *mut ARRAY,
        arg2: arrayind_t,
        arg3: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn array_remove(arg1: *mut ARRAY, arg2: arrayind_t) -> *mut ARRAY_ELEMENT;
}
extern "C" {
    pub fn array_reference(arg1: *mut ARRAY, arg2: arrayind_t) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn array_to_word_list(arg1: *mut ARRAY) -> *mut WordList;
}
extern "C" {
    pub fn array_from_word_list(arg1: *mut WordList) -> *mut ARRAY;
}
extern "C" {
    pub fn array_keys_to_word_list(arg1: *mut ARRAY) -> *mut WordList;
}
extern "C" {
    pub fn array_assign_list(arg1: *mut ARRAY, arg2: *mut WordList) -> *mut ARRAY;
}
extern "C" {
    pub fn array_to_argv(
        arg1: *mut ARRAY,
        arg2: *mut ::std::os::raw::c_int,
    ) -> *mut *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn array_to_kvpair(
        arg1: *mut ARRAY,
        arg2: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn array_to_assign(
        arg1: *mut ARRAY,
        arg2: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn array_to_string(
        arg1: *mut ARRAY,
        arg2: *mut ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn array_from_string(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *mut ::std::os::raw::c_char,
    ) -> *mut ARRAY;
}
extern "C" {
    pub fn execute_array_command(
        arg1: *mut ARRAY,
        arg2: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn initialize_traps();
}
extern "C" {
    pub fn run_pending_traps();
}
extern "C" {
    pub fn queue_sigchld_trap(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn maybe_set_sigchld_trap(arg1: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn set_impossible_sigchld_trap();
}
extern "C" {
    pub fn set_sigchld_trap(arg1: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn set_debug_trap(arg1: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn set_error_trap(arg1: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn set_return_trap(arg1: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn maybe_set_debug_trap(arg1: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn maybe_set_error_trap(arg1: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn maybe_set_return_trap(arg1: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn set_sigint_trap(arg1: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn set_signal(arg1: ::std::os::raw::c_int, arg2: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn restore_default_signal(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn ignore_signal(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn run_exit_trap() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn run_trap_cleanup(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn run_debug_trap() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn run_error_trap();
}
extern "C" {
    pub fn run_return_trap();
}
extern "C" {
    pub fn free_trap_strings();
}
extern "C" {
    pub fn reset_signal_handlers();
}
extern "C" {
    pub fn restore_original_signals();
}
extern "C" {
    pub fn get_original_signal(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn get_all_original_signals();
}
extern "C" {
    pub fn signal_name(arg1: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn decode_signal(
        arg1: *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn run_interrupt_trap(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn maybe_call_trap_handler(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn signal_is_special(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn signal_is_trapped(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn signal_is_pending(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn signal_is_ignored(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn signal_is_hard_ignored(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn set_signal_hard_ignored(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn set_signal_ignored(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn signal_in_progress(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn set_trap_state(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn next_pending_trap(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn first_pending_trap() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clear_pending_traps();
}
extern "C" {
    pub fn any_signals_trapped() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn check_signals();
}
extern "C" {
    pub fn check_signals_and_traps();
}
pub type rl_dequote_func_t = ::core::option::Option<
    fn(
        arg1: *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char,
>;
pub type rl_compdisp_func_t = ::core::option::Option<
    unsafe extern "C" fn(
        arg1: *mut *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ),
>;
pub type rl_intfunc_t =
    ::core::option::Option<fn(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int>;
pub type rl_icpfunc_t =
    ::core::option::Option<fn(arg1: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int>;
pub type rl_icppfunc_t =
    ::core::option::Option<fn(arg1: *mut *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int>;
pub type rl_getc_func_t =
    ::core::option::Option<unsafe extern "C" fn(arg1: *mut FILE) -> ::std::os::raw::c_int>;
pub type rl_vintfunc_t = ::core::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>;
pub type rl_vcpfunc_t =
    ::core::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_char)>;
pub type rl_vcppfunc_t =
    ::core::option::Option<unsafe extern "C" fn(arg1: *mut *mut ::std::os::raw::c_char)>;
pub type rl_cpvfunc_t =
    ::core::option::Option<unsafe extern "C" fn() -> *mut ::std::os::raw::c_char>;
pub type rl_cpifunc_t = ::core::option::Option<
    unsafe extern "C" fn(arg1: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char,
>;
pub type rl_cpcpfunc_t = ::core::option::Option<
    unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char,
>;
pub type rl_cpcppfunc_t = ::core::option::Option<
    unsafe extern "C" fn(arg1: *mut *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char,
>;
pub type KEYMAP_ENTRY_ARRAY = [KEYMAP_ENTRY; 257usize];
extern "C" {
    #[link_name = "\u{1}emacs_standard_keymap"]
    pub static mut emacs_standard_keymap: KEYMAP_ENTRY_ARRAY;
}
extern "C" {
    #[link_name = "\u{1}emacs_meta_keymap"]
    pub static mut emacs_meta_keymap: KEYMAP_ENTRY_ARRAY;
}
extern "C" {
    #[link_name = "\u{1}emacs_ctlx_keymap"]
    pub static mut emacs_ctlx_keymap: KEYMAP_ENTRY_ARRAY;
}
extern "C" {
    #[link_name = "\u{1}vi_insertion_keymap"]
    pub static mut vi_insertion_keymap: KEYMAP_ENTRY_ARRAY;
}
extern "C" {
    #[link_name = "\u{1}vi_movement_keymap"]
    pub static mut vi_movement_keymap: KEYMAP_ENTRY_ARRAY;
}
extern "C" {
    pub fn rl_make_bare_keymap() -> Keymap;
}
extern "C" {
    pub fn rl_copy_keymap(arg1: Keymap) -> Keymap;
}
extern "C" {
    pub fn rl_make_keymap() -> Keymap;
}
extern "C" {
    pub fn rl_discard_keymap(arg1: Keymap);
}
extern "C" {
    pub fn rl_set_keymap_name(
        arg1: *const ::std::os::raw::c_char,
        arg2: Keymap,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}tilde_expansion_failure_hook"]
    pub static mut tilde_expansion_failure_hook: tilde_hook_func_t;
}
extern "C" {
    #[link_name = "\u{1}tilde_additional_prefixes"]
    pub static mut tilde_additional_prefixes: *mut *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}tilde_additional_suffixes"]
    pub static mut tilde_additional_suffixes: *mut *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tilde_expand(arg1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tilde_expand_word(arg1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tilde_find_word(
        arg1: *const ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
pub const undo_code_UNDO_DELETE: undo_code = 0;
pub const undo_code_UNDO_INSERT: undo_code = 1;
pub const undo_code_UNDO_BEGIN: undo_code = 2;
pub const undo_code_UNDO_END: undo_code = 3;
pub type undo_code = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct undo_list {
    pub next: *mut undo_list,
    pub start: ::std::os::raw::c_int,
    pub end: ::std::os::raw::c_int,
    pub text: *mut ::std::os::raw::c_char,
    pub what: undo_code,
}
pub type UNDO_LIST = undo_list;
extern "C" {
    #[link_name = "\u{1}rl_undo_list"]
    pub static mut rl_undo_list: *mut UNDO_LIST;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _funmap {
    pub name: *const ::std::os::raw::c_char,
    pub function: rl_command_func_t,
}
pub type FUNMAP = _funmap;
extern "C" {
    #[link_name = "\u{1}funmap"]
    pub static mut funmap: *mut *mut FUNMAP;
}
extern "C" {
    pub fn rl_digit_argument(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_universal_argument(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_forward_byte(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_forward_char(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_forward(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_backward_byte(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_backward_char(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_backward(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_beg_of_line(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_end_of_line(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_forward_word(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_backward_word(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_refresh_line(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_clear_screen(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_clear_display(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_skip_csi_sequence(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_arrow_keys(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_previous_screen_line(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_next_screen_line(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_insert(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_quoted_insert(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_tab_insert(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_newline(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_do_lowercase_version(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_rubout(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_delete(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_rubout_or_delete(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_delete_horizontal_space(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_delete_or_show_completions(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_insert_comment(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_upcase_word(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_downcase_word(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_capitalize_word(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_transpose_words(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_transpose_chars(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_char_search(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_backward_char_search(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_beginning_of_history(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_end_of_history(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_get_next_history(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_get_previous_history(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_operate_and_get_next(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_set_mark(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_exchange_point_and_mark(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_editing_mode(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_emacs_editing_mode(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_overwrite_mode(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_re_read_init_file(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_dump_functions(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_dump_macros(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_dump_variables(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_possible_completions(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_insert_completions(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_old_menu_complete(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_menu_complete(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_backward_menu_complete(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_kill_word(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_backward_kill_word(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_kill_line(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_backward_kill_line(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_kill_full_line(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_unix_word_rubout(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_unix_filename_rubout(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_unix_line_discard(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_copy_region_to_kill(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_kill_region(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_copy_forward_word(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_copy_backward_word(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_yank(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_yank_pop(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_yank_nth_arg(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_yank_last_arg(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_bracketed_paste_begin(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_reverse_search_history(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_forward_search_history(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_start_kbd_macro(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_end_kbd_macro(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_call_last_kbd_macro(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_print_last_kbd_macro(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_revert_line(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_undo_command(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_tilde_expand(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_restart_output(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_stop_output(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_abort(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_tty_status(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_history_search_forward(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_history_search_backward(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_history_substr_search_forward(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_history_substr_search_backward(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_noninc_forward_search(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_noninc_reverse_search(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_noninc_forward_search_again(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_noninc_reverse_search_again(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_insert_close(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_callback_handler_install(arg1: *const ::std::os::raw::c_char, arg2: rl_vcpfunc_t);
}
extern "C" {
    pub fn rl_callback_read_char();
}
extern "C" {
    pub fn rl_callback_handler_remove();
}
extern "C" {
    pub fn rl_callback_sigcleanup();
}
extern "C" {
    pub fn rl_vi_redo(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_undo(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_yank_arg(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_fetch_history(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_search_again(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_search(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_complete(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_tilde_expand(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_prev_word(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_next_word(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_end_word(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_insert_beg(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_append_mode(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_append_eol(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_eof_maybe(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_insertion_mode(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_insert_mode(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_movement_mode(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_arg_digit(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_change_case(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_put(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_column(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_delete_to(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_change_to(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_yank_to(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_yank_pop(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_rubout(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_delete(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_back_to_indent(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_unix_word_rubout(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_first_print(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_char_search(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_match(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_change_char(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_subst(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_overstrike(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_overstrike_delete(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_replace(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_set_mark(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_goto_mark(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_check() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_domove(
        arg1: ::std::os::raw::c_int,
        arg2: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_bracktype(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_start_inserting(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn rl_vi_fWord(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_bWord(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_eWord(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_fword(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_bword(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_vi_eword(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn readline(arg1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn rl_set_prompt(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_expand_prompt(arg1: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_initialize() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_discard_argument() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_add_defun(
        arg1: *const ::std::os::raw::c_char,
        arg2: Option<rl_command_func_t>,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_bind_key(
        arg1: ::std::os::raw::c_int,
        arg2: rl_command_func_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_bind_key_in_map(
        arg1: ::std::os::raw::c_int,
        arg2: Option<rl_command_func_t>,
        arg3: Keymap,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_unbind_key(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_unbind_key_in_map(arg1: ::std::os::raw::c_int, arg2: Keymap)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_bind_key_if_unbound(
        arg1: ::std::os::raw::c_int,
        arg2: rl_command_func_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_bind_key_if_unbound_in_map(
        arg1: ::std::os::raw::c_int,
        arg2: Option<rl_command_func_t>,
        arg3: Keymap,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_unbind_command_in_map(
        arg1: *const ::std::os::raw::c_char,
        arg2: Keymap,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_bind_keyseq_in_map(
        arg1: *const ::std::os::raw::c_char,
        arg2: Option<rl_command_func_t>,
        arg3: Keymap,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_bind_keyseq_if_unbound(
        arg1: *const ::std::os::raw::c_char,
        arg2: rl_command_func_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_bind_keyseq_if_unbound_in_map(
        arg1: *const ::std::os::raw::c_char,
        arg2: rl_command_func_t,
        arg3: Keymap,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_generic_bind(
        arg1: ::std::os::raw::c_int,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut ::std::os::raw::c_char,
        arg4: Keymap,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_variable_value(arg1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn rl_variable_bind(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_set_key(
        arg1: *const ::std::os::raw::c_char,
        arg2: rl_command_func_t,
        arg3: Keymap,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_macro_bind(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: Keymap,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_untranslate_keyseq(arg1: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn rl_function_of_keyseq(
        arg1: *const ::std::os::raw::c_char,
        arg2: Keymap,
        arg3: *mut ::std::os::raw::c_int,
    ) -> Option<rl_command_func_t>;
}
extern "C" {
    pub fn rl_invoking_keyseqs_in_map(
        arg1: rl_command_func_t,
        arg2: Keymap,
    ) -> *mut *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn rl_empty_keymap(arg1: Keymap) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_free_keymap(arg1: Keymap);
}
extern "C" {
    pub fn rl_get_keymap_name(arg1: Keymap) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn rl_set_keymap_from_edit_mode();
}
extern "C" {
    pub fn rl_get_keymap_name_from_edit_mode() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn rl_add_funmap_entry(
        arg1: *const ::std::os::raw::c_char,
        arg2: rl_command_func_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_funmap_names() -> *mut *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn rl_initialize_funmap();
}
extern "C" {
    pub fn rl_push_macro_input(arg1: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn rl_add_undo(
        arg1: undo_code,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn rl_free_undo_list();
}
extern "C" {
    pub fn rl_do_undo() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_begin_undo_group() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_end_undo_group() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_modifying(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_redisplay();
}
extern "C" {
    pub fn rl_on_new_line() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_on_new_line_with_prompt() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_forced_update_display() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_clear_visible_line() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_clear_message() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_reset_line_state() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_crlf() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_keep_mark_active();
}
extern "C" {
    pub fn rl_activate_mark();
}
extern "C" {
    pub fn rl_deactivate_mark();
}
extern "C" {
    pub fn rl_mark_active_p() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_message(arg1: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_show_char(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_character_len(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_redraw_prompt_last_line();
}
extern "C" {
    pub fn rl_save_prompt();
}
extern "C" {
    pub fn rl_restore_prompt();
}
extern "C" {
    pub fn rl_replace_line(arg1: *const ::std::os::raw::c_char, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn rl_delete_text(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_kill_text(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_copy_text(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn rl_prep_terminal(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn rl_deprep_terminal();
}
extern "C" {
    pub fn rl_tty_set_default_bindings(arg1: Keymap);
}
extern "C" {
    pub fn rl_tty_unset_default_bindings(arg1: Keymap);
}
extern "C" {
    pub fn rl_tty_set_echoing(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_reset_terminal(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_resize_terminal();
}
extern "C" {
    pub fn rl_set_screen_size(arg1: ::std::os::raw::c_int, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn rl_get_screen_size(arg1: *mut ::std::os::raw::c_int, arg2: *mut ::std::os::raw::c_int);
}
extern "C" {
    pub fn rl_reset_screen_size();
}
extern "C" {
    pub fn rl_get_termcap(arg1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn rl_stuff_char(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_execute_next(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_clear_pending_input() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_read_key() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_getc(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_set_keyboard_input_timeout(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_extend_line_buffer(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn rl_ding() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_alphabetic(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_free(arg1: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn rl_set_signals() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_clear_signals() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_cleanup_after_signal();
}
extern "C" {
    pub fn rl_reset_after_signal();
}
extern "C" {
    pub fn rl_free_line_state();
}
extern "C" {
    pub fn rl_pending_signal() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_check_signals();
}
extern "C" {
    pub fn rl_echo_signal_char(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn rl_set_paren_blink_timeout(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_clear_history();
}
extern "C" {
    pub fn rl_maybe_save_line() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_maybe_unsave_line() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_maybe_replace_line() -> ::std::os::raw::c_int;
}
// extern "C" {
//     pub fn rl_complete_internal(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
// }
extern "C" {
    pub fn rl_display_match_list(
        arg1: *mut *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn rl_completion_matches(
        arg1: *const ::std::os::raw::c_char,
        arg2: Option<rl_compentry_func_t>,
    ) -> *mut *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn rl_username_completion_function(
        arg1: *const ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
// extern "C" {
//     pub fn rl_filename_completion_function(
//         arg1: *const ::std::os::raw::c_char,
//         arg2: ::std::os::raw::c_int,
//     ) -> *mut ::std::os::raw::c_char;
// }
extern "C" {
    pub fn rl_completion_mode(_: Option<rl_command_func_t>) -> libc::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_library_version"]
    pub static mut rl_library_version: *const ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}rl_readline_version"]
    pub static mut rl_readline_version: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_gnu_readline_p"]
    pub static mut rl_gnu_readline_p: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_readline_state"]
    pub static mut rl_readline_state: ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}rl_editing_mode"]
    pub static mut rl_editing_mode: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_insert_mode"]
    pub static mut rl_insert_mode: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_readline_name"]
    pub static mut rl_readline_name: *const ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}rl_prompt"]
    pub static mut rl_prompt: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}rl_display_prompt"]
    pub static mut rl_display_prompt: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}rl_line_buffer"]
    pub static mut rl_line_buffer: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}rl_point"]
    pub static mut rl_point: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_end"]
    pub static mut rl_end: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_mark"]
    pub static mut rl_mark: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_pending_input"]
    pub static mut rl_pending_input: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_explicit_arg"]
    pub static mut rl_explicit_arg: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_numeric_arg"]
    pub static mut rl_numeric_arg: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_last_func"]
    pub static mut rl_last_func: Option<rl_command_func_t>;
}
extern "C" {
    #[link_name = "\u{1}rl_terminal_name"]
    pub static mut rl_terminal_name: *const ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}rl_prefer_env_winsize"]
    pub static mut rl_prefer_env_winsize: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_pre_input_hook"]
    pub static mut rl_pre_input_hook: rl_hook_func_t;
}
extern "C" {
    #[link_name = "\u{1}rl_event_hook"]
    pub static mut rl_event_hook: rl_hook_func_t;
}
extern "C" {
    #[link_name = "\u{1}rl_signal_event_hook"]
    pub static mut rl_signal_event_hook: Option<rl_hook_func_t>;
}
extern "C" {
    #[link_name = "\u{1}rl_input_available_hook"]
    pub static mut rl_input_available_hook: rl_hook_func_t;
}
extern "C" {
    #[link_name = "\u{1}rl_getc_function"]
    pub static mut rl_getc_function: rl_getc_func_t;
}
extern "C" {
    #[link_name = "\u{1}rl_redisplay_function"]
    pub static mut rl_redisplay_function: rl_voidfunc_t;
}
extern "C" {
    #[link_name = "\u{1}rl_prep_term_function"]
    pub static mut rl_prep_term_function: rl_vintfunc_t;
}
extern "C" {
    #[link_name = "\u{1}rl_deprep_term_function"]
    pub static mut rl_deprep_term_function: Option<rl_voidfunc_t>;
}
extern "C" {
    #[link_name = "\u{1}rl_executing_keymap"]
    pub static mut rl_executing_keymap: Keymap;
}
extern "C" {
    #[link_name = "\u{1}rl_binding_keymap"]
    pub static mut rl_binding_keymap: Keymap;
}
extern "C" {
    #[link_name = "\u{1}rl_executing_key"]
    pub static mut rl_executing_key: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_executing_keyseq"]
    pub static mut rl_executing_keyseq: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}rl_key_sequence_length"]
    pub static mut rl_key_sequence_length: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_erase_empty_line"]
    pub static mut rl_erase_empty_line: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_already_prompted"]
    pub static mut rl_already_prompted: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_num_chars_to_read"]
    pub static mut rl_num_chars_to_read: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_executing_macro"]
    pub static mut rl_executing_macro: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}rl_catch_signals"]
    pub static mut rl_catch_signals: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_catch_sigwinch"]
    pub static mut rl_catch_sigwinch: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_change_environment"]
    pub static mut rl_change_environment: ::std::os::raw::c_int;
}
// extern "C" {
//     #[link_name = "\u{1}rl_completion_entry_function"]
//     pub static mut rl_completion_entry_function: rl_compentry_func_t;
// }
extern "C" {
    #[link_name = "\u{1}rl_menu_completion_entry_function"]
    pub static mut rl_menu_completion_entry_function: Option<rl_compentry_func_t>;
}
// extern "C" {
//     #[link_name = "\u{1}rl_ignore_some_completions_function"]
//     pub static mut rl_ignore_some_completions_function: rl_compignore_func_t;
// }
extern "C" {
    #[link_name = "\u{1}rl_basic_word_break_characters"]
    pub static mut rl_basic_word_break_characters: *const ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}rl_completer_word_break_characters"]
    pub static mut rl_completer_word_break_characters: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}rl_completion_word_break_hook"]
    pub static mut rl_completion_word_break_hook: rl_cpvfunc_t;
}
extern "C" {
    #[link_name = "\u{1}rl_completer_quote_characters"]
    pub static mut rl_completer_quote_characters: *const ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}rl_basic_quote_characters"]
    pub static mut rl_basic_quote_characters: *const ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}rl_filename_quote_characters"]
    pub static mut rl_filename_quote_characters: *const ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}rl_special_prefixes"]
    pub static mut rl_special_prefixes: *const ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}rl_directory_completion_hook"]
    pub static mut rl_directory_completion_hook: rl_icppfunc_t;
}
extern "C" {
    #[link_name = "\u{1}rl_directory_rewrite_hook"]
    pub static mut rl_directory_rewrite_hook: rl_icppfunc_t;
}
extern "C" {
    #[link_name = "\u{1}rl_filename_stat_hook"]
    pub static mut rl_filename_stat_hook: rl_icppfunc_t;
}
extern "C" {
    #[link_name = "\u{1}rl_filename_rewrite_hook"]
    pub static mut rl_filename_rewrite_hook: rl_dequote_func_t;
}
extern "C" {
    #[link_name = "\u{1}rl_completion_display_matches_hook"]
    pub static mut rl_completion_display_matches_hook: rl_compdisp_func_t;
}
extern "C" {
    #[link_name = "\u{1}rl_filename_completion_desired"]
    pub static mut rl_filename_completion_desired: ::std::os::raw::c_int;
}
// extern "C" {
//     #[link_name = "\u{1}rl_filename_quoting_desired"]
//     pub static mut rl_filename_quoting_desired: ::std::os::raw::c_int;
// }
// extern "C" {
//     #[link_name = "\u{1}rl_filename_quoting_function"]
//     pub static mut rl_filename_quoting_function: rl_quote_func_t;
// }
extern "C" {
    #[link_name = "\u{1}rl_filename_dequoting_function"]
    pub static mut rl_filename_dequoting_function: rl_dequote_func_t;
}
extern "C" {
    #[link_name = "\u{1}rl_char_is_quoted_p"]
    pub static mut rl_char_is_quoted_p: Option<rl_linebuf_func_t>;
}
extern "C" {
    #[link_name = "\u{1}rl_attempted_completion_over"]
    pub static mut rl_attempted_completion_over: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_completion_type"]
    pub static mut rl_completion_type: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_completion_invoking_key"]
    pub static mut rl_completion_invoking_key: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_completion_query_items"]
    pub static mut rl_completion_query_items: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_completion_append_character"]
    pub static mut rl_completion_append_character: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_completion_suppress_append"]
    pub static mut rl_completion_suppress_append: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_completion_quote_character"]
    pub static mut rl_completion_quote_character: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_completion_found_quote"]
    pub static mut rl_completion_found_quote: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_completion_suppress_quote"]
    pub static mut rl_completion_suppress_quote: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_sort_completion_matches"]
    pub static mut rl_sort_completion_matches: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_completion_mark_symlink_dirs"]
    pub static mut rl_completion_mark_symlink_dirs: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_ignore_completion_duplicates"]
    pub static mut rl_ignore_completion_duplicates: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_inhibit_completion"]
    pub static mut rl_inhibit_completion: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_persistent_signal_handlers"]
    pub static mut rl_persistent_signal_handlers: ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct readline_state {
    pub point: ::std::os::raw::c_int,
    pub end: ::std::os::raw::c_int,
    pub mark: ::std::os::raw::c_int,
    pub buflen: ::std::os::raw::c_int,
    pub buffer: *mut ::std::os::raw::c_char,
    pub ul: *mut UNDO_LIST,
    pub prompt: *mut ::std::os::raw::c_char,
    pub rlstate: ::std::os::raw::c_int,
    pub done: ::std::os::raw::c_int,
    pub kmap: Keymap,
    pub lastfunc: rl_command_func_t,
    pub insmode: ::std::os::raw::c_int,
    pub edmode: ::std::os::raw::c_int,
    pub kseq: *mut ::std::os::raw::c_char,
    pub kseqlen: ::std::os::raw::c_int,
    pub pendingin: ::std::os::raw::c_int,
    pub inf: *mut FILE,
    pub outf: *mut FILE,
    pub macro_: *mut ::std::os::raw::c_char,
    pub catchsigs: ::std::os::raw::c_int,
    pub catchsigwinch: ::std::os::raw::c_int,
    pub entryfunc: rl_compentry_func_t,
    pub menuentryfunc: rl_compentry_func_t,
    pub ignorefunc: rl_compignore_func_t,
    pub attemptfunc: rl_completion_func_t,
    pub wordbreakchars: *mut ::std::os::raw::c_char,
    pub reserved: [::std::os::raw::c_char; 64usize],
}
extern "C" {
    pub fn rl_save_state(arg1: *mut readline_state) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rl_restore_state(arg1: *mut readline_state) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_find_prev_mbchar(
        arg1: *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_find_next_mbchar(
        arg1: *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_compare_chars(
        arg1: *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: *mut mbstate_t,
        arg4: *mut ::std::os::raw::c_char,
        arg5: ::std::os::raw::c_int,
        arg6: *mut mbstate_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_get_char_len(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *mut mbstate_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_adjust_point(
        arg1: *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: *mut mbstate_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_read_mbchar(
        arg1: *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_read_mbstring(
        arg1: ::std::os::raw::c_int,
        arg2: *mut ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_is_mbchar_matched(
        arg1: *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_char,
        arg5: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_char_value(
        arg1: *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
    ) -> wchar_t;
}
extern "C" {
    pub fn _rl_walphabetic(arg1: wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_byte_oriented"]
    pub static mut rl_byte_oriented: ::std::os::raw::c_int;
}
extern "C" {
    pub fn _setjmp(__env: *mut __jmp_buf_tag) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn longjmp(__env: *mut __jmp_buf_tag, __val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _longjmp(__env: *mut __jmp_buf_tag, __val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn siglongjmp(__env: *mut __jmp_buf_tag, __val: ::std::os::raw::c_int);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __rl_search_context {
    pub type_: ::std::os::raw::c_int,
    pub sflags: ::std::os::raw::c_int,
    pub search_string: *mut ::std::os::raw::c_char,
    pub search_string_index: ::std::os::raw::c_int,
    pub search_string_size: ::std::os::raw::c_int,
    pub lines: *mut *mut ::std::os::raw::c_char,
    pub allocated_line: *mut ::std::os::raw::c_char,
    pub hlen: ::std::os::raw::c_int,
    pub hindex: ::std::os::raw::c_int,
    pub save_point: ::std::os::raw::c_int,
    pub save_mark: ::std::os::raw::c_int,
    pub save_line: ::std::os::raw::c_int,
    pub last_found_line: ::std::os::raw::c_int,
    pub prev_line_found: *mut ::std::os::raw::c_char,
    pub save_undo_list: *mut UNDO_LIST,
    pub keymap: Keymap,
    pub okeymap: Keymap,
    pub history_pos: ::std::os::raw::c_int,
    pub direction: ::std::os::raw::c_int,
    pub prevc: ::std::os::raw::c_int,
    pub lastc: ::std::os::raw::c_int,
    pub mb: [::std::os::raw::c_char; 16usize],
    pub pmb: [::std::os::raw::c_char; 16usize],
    pub sline: *mut ::std::os::raw::c_char,
    pub sline_len: ::std::os::raw::c_int,
    pub sline_index: ::std::os::raw::c_int,
    pub search_terminators: *mut ::std::os::raw::c_char,
}
pub type _rl_search_cxt = __rl_search_context;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _rl_cmd {
    pub map: Keymap,
    pub count: ::std::os::raw::c_int,
    pub key: ::std::os::raw::c_int,
    pub func: rl_command_func_t,
}
extern "C" {
    #[link_name = "\u{1}_rl_pending_command"]
    pub static mut _rl_pending_command: _rl_cmd;
}
extern "C" {
    #[link_name = "\u{1}_rl_command_to_execute"]
    pub static mut _rl_command_to_execute: *mut _rl_cmd;
}
pub type _rl_arg_cxt = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __rl_keyseq_context {
    pub flags: ::std::os::raw::c_int,
    pub subseq_arg: ::std::os::raw::c_int,
    pub subseq_retval: ::std::os::raw::c_int,
    pub okey: ::std::os::raw::c_int,
    pub dmap: Keymap,
    pub oldmap: Keymap,
    pub ocxt: *mut __rl_keyseq_context,
    pub childval: ::std::os::raw::c_int,
}
pub type _rl_keyseq_cxt = __rl_keyseq_context;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __rl_vimotion_context {
    pub op: ::std::os::raw::c_int,
    pub state: ::std::os::raw::c_int,
    pub flags: ::std::os::raw::c_int,
    pub ncxt: _rl_arg_cxt,
    pub numeric_arg: ::std::os::raw::c_int,
    pub start: ::std::os::raw::c_int,
    pub end: ::std::os::raw::c_int,
    pub key: ::std::os::raw::c_int,
    pub motion: ::std::os::raw::c_int,
}
pub type _rl_vimotion_cxt = __rl_vimotion_context;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __rl_callback_generic_arg {
    pub count: ::std::os::raw::c_int,
    pub i1: ::std::os::raw::c_int,
    pub i2: ::std::os::raw::c_int,
}
pub type _rl_callback_generic_arg = __rl_callback_generic_arg;
pub type _rl_callback_func_t = ::core::option::Option<
    unsafe extern "C" fn(arg1: *mut _rl_callback_generic_arg) -> ::std::os::raw::c_int,
>;
pub type _rl_sigcleanup_func_t = ::core::option::Option<
    unsafe extern "C" fn(arg1: ::std::os::raw::c_int, arg2: *mut ::std::os::raw::c_void),
>;
extern "C" {
    #[link_name = "\u{1}rl_complete_with_tilde_expansion"]
    pub static mut rl_complete_with_tilde_expansion: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_visible_stats"]
    pub static mut rl_visible_stats: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_colored_stats"]
    pub static mut _rl_colored_stats: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_colored_completion_prefix"]
    pub static mut _rl_colored_completion_prefix: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_line_buffer_len"]
    pub static mut rl_line_buffer_len: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_arg_sign"]
    pub static mut rl_arg_sign: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_visible_prompt_length"]
    pub static mut rl_visible_prompt_length: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_display_fixed"]
    pub static mut rl_display_fixed: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}rl_blink_matching_paren"]
    pub static mut rl_blink_matching_paren: ::std::os::raw::c_int;
}
extern "C" {
    #[doc = "\t\t\t\t\t\t\t\t\t *"]
    #[doc = " Global functions and variables unused and undocumented\t\t *"]
    #[doc = "\t\t\t\t\t\t\t\t\t *"]
    pub fn rl_set_retained_kills(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_set_screen_size(arg1: ::std::os::raw::c_int, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _rl_fix_last_undo_of_type(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_savestring(arg1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn readline_internal_setup();
}
extern "C" {
    pub fn readline_internal_teardown(arg1: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn readline_internal_char() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_keyseq_cxt_alloc() -> *mut _rl_keyseq_cxt;
}
extern "C" {
    pub fn _rl_keyseq_cxt_dispose(arg1: *mut _rl_keyseq_cxt);
}
extern "C" {
    pub fn _rl_keyseq_chain_dispose();
}
extern "C" {
    pub fn _rl_dispatch_callback(arg1: *mut _rl_keyseq_cxt) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_callback_data_alloc(arg1: ::std::os::raw::c_int) -> *mut _rl_callback_generic_arg;
}
extern "C" {
    pub fn _rl_callback_data_dispose(arg1: *mut _rl_callback_generic_arg);
}
extern "C" {
    pub fn _rl_untranslate_macro_value(
        arg1: *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _rl_reset_completion_state();
}
extern "C" {
    pub fn _rl_find_completion_word(
        arg1: *mut ::std::os::raw::c_int,
        arg2: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_char;
}
extern "C" {
    pub fn _rl_free_match_list(arg1: *mut *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn _rl_strip_prompt(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _rl_reset_prompt();
}
extern "C" {
    pub fn _rl_move_vert(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _rl_save_prompt();
}
extern "C" {
    pub fn _rl_restore_prompt();
}
extern "C" {
    pub fn _rl_make_prompt_for_search(arg1: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _rl_erase_at_end_of_line(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _rl_clear_to_eol(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _rl_clear_screen(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _rl_update_final();
}
extern "C" {
    pub fn _rl_optimize_redisplay();
}
extern "C" {
    pub fn _rl_redisplay_after_sigwinch();
}
extern "C" {
    pub fn _rl_clean_up_for_exit();
}
extern "C" {
    pub fn _rl_erase_entire_line();
}
extern "C" {
    pub fn _rl_current_display_line() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_refresh_line();
}
extern "C" {
    pub fn _rl_any_typein() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_input_available() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_nchars_available() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_input_queued(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_insert_typein(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _rl_unget_char(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_pushed_input_available() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_scxt_alloc(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> *mut _rl_search_cxt;
}
extern "C" {
    pub fn _rl_scxt_dispose(arg1: *mut _rl_search_cxt, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _rl_isearch_dispatch(
        arg1: *mut _rl_search_cxt,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_isearch_callback(arg1: *mut _rl_search_cxt) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_isearch_cleanup(
        arg1: *mut _rl_search_cxt,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_search_getchar(arg1: *mut _rl_search_cxt) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_read_bracketed_paste_prefix(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_bracketed_text(arg1: *mut usize) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _rl_bracketed_read_key() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_bracketed_read_mbstring(
        arg1: *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_with_macro_input(arg1: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn _rl_peek_macro_key() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_next_macro_key() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_prev_macro_key() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_push_executing_macro();
}
extern "C" {
    pub fn _rl_pop_executing_macro();
}
extern "C" {
    pub fn _rl_add_macro_char(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _rl_kill_kbd_macro();
}
extern "C" {
    pub fn _rl_arg_overflow() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_arg_init();
}
extern "C" {
    pub fn _rl_arg_getchar() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_arg_callback(arg1: _rl_arg_cxt) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_reset_argument();
}
extern "C" {
    pub fn _rl_start_using_history();
}
extern "C" {
    pub fn _rl_free_saved_history_line() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_set_insert_mode(arg1: ::std::os::raw::c_int, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _rl_revert_previous_lines();
}
extern "C" {
    pub fn _rl_revert_all_lines();
}
extern "C" {
    pub fn _rl_init_locale() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _rl_init_eightbit() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_enable_paren_matching(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _rl_init_line_state();
}
extern "C" {
    pub fn _rl_set_the_line();
}
extern "C" {
    pub fn _rl_dispatch(arg1: ::std::os::raw::c_int, arg2: Keymap) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_dispatch_subseq(
        arg1: ::std::os::raw::c_int,
        arg2: Keymap,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_internal_char_cleanup();
}
extern "C" {
    pub fn _rl_init_executing_keyseq();
}
extern "C" {
    pub fn _rl_term_executing_keyseq();
}
extern "C" {
    pub fn _rl_end_executing_keyseq();
}
extern "C" {
    pub fn _rl_add_executing_keyseq(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _rl_disable_tty_signals() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_restore_tty_signals() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_nsearch_callback(arg1: *mut _rl_search_cxt) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_nsearch_cleanup(
        arg1: *mut _rl_search_cxt,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_signal_handler(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _rl_block_sigint();
}
extern "C" {
    pub fn _rl_release_sigint();
}
extern "C" {
    pub fn _rl_block_sigwinch();
}
extern "C" {
    pub fn _rl_release_sigwinch();
}
extern "C" {
    pub fn _rl_get_screen_size(arg1: ::std::os::raw::c_int, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _rl_sigwinch_resize_terminal();
}
extern "C" {
    pub fn _rl_init_terminal_io(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_output_character_function(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_cr();
}
extern "C" {
    pub fn _rl_output_some_chars(arg1: *const ::std::os::raw::c_char, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _rl_backspace(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_enable_meta_key();
}
extern "C" {
    pub fn _rl_disable_meta_key();
}
extern "C" {
    pub fn _rl_control_keypad(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _rl_set_cursor(arg1: ::std::os::raw::c_int, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _rl_standout_on();
}
extern "C" {
    pub fn _rl_standout_off();
}
extern "C" {
    pub fn _rl_fix_point(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _rl_fix_mark();
}
extern "C" {
    pub fn _rl_replace_text(
        arg1: *const ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_forward_char_internal(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_backward_char_internal(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_insert_char(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_overwrite_char(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_overwrite_rubout(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_rubout_char(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_char_search_internal(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_char,
        arg4: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_set_mark_at_pos(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_copy_undo_entry(arg1: *mut UNDO_LIST) -> *mut UNDO_LIST;
}
extern "C" {
    pub fn _rl_copy_undo_list(arg1: *mut UNDO_LIST) -> *mut UNDO_LIST;
}
extern "C" {
    pub fn _rl_free_undo_list(arg1: *mut UNDO_LIST);
}
extern "C" {
    pub fn _rl_ttymsg(arg1: *const ::std::os::raw::c_char, ...);
}
extern "C" {
    pub fn _rl_errmsg(arg1: *const ::std::os::raw::c_char, ...);
}
extern "C" {
    pub fn _rl_trace(arg1: *const ::std::os::raw::c_char, ...);
}
extern "C" {
    pub fn _rl_audit_tty(arg1: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn _rl_tropen() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_abort_internal() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_null_function(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_strindex(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _rl_qsort_string_compare(
        arg1: *mut *mut ::std::os::raw::c_char,
        arg2: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_uppercase_p(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_lowercase_p(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_pure_alphabetic(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_digit_p(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_to_lower(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_to_upper(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_digit_value(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_vi_initialize_line();
}
extern "C" {
    pub fn _rl_vi_reset_last();
}
extern "C" {
    pub fn _rl_vi_set_last(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn _rl_vi_textmod_command(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_vi_motion_command(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_vi_done_inserting();
}
extern "C" {
    pub fn _rl_vi_domove_callback(arg1: *mut _rl_vimotion_cxt) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_vi_domove_motion_cleanup(
        arg1: ::std::os::raw::c_int,
        arg2: *mut _rl_vimotion_cxt,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_possible_control_prefixes"]
    pub static mut _rl_possible_control_prefixes: [*const ::std::os::raw::c_char; 0usize];
}
extern "C" {
    #[link_name = "\u{1}_rl_possible_meta_prefixes"]
    pub static mut _rl_possible_meta_prefixes: [*const ::std::os::raw::c_char; 0usize];
}
extern "C" {
    #[link_name = "\u{1}_rl_callback_func"]
    pub static mut _rl_callback_func: _rl_callback_func_t;
}
extern "C" {
    #[link_name = "\u{1}_rl_callback_data"]
    pub static mut _rl_callback_data: *mut _rl_callback_generic_arg;
}
extern "C" {
    #[link_name = "\u{1}_rl_complete_show_all"]
    pub static mut _rl_complete_show_all: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_complete_show_unmodified"]
    pub static mut _rl_complete_show_unmodified: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_complete_mark_directories"]
    pub static mut _rl_complete_mark_directories: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_complete_mark_symlink_dirs"]
    pub static mut _rl_complete_mark_symlink_dirs: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_completion_prefix_display_length"]
    pub static mut _rl_completion_prefix_display_length: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_completion_columns"]
    pub static mut _rl_completion_columns: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_print_completions_horizontally"]
    pub static mut _rl_print_completions_horizontally: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_completion_case_fold"]
    pub static mut _rl_completion_case_fold: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_completion_case_map"]
    pub static mut _rl_completion_case_map: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_match_hidden_files"]
    pub static mut _rl_match_hidden_files: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_page_completions"]
    pub static mut _rl_page_completions: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_skip_completed_text"]
    pub static mut _rl_skip_completed_text: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_menu_complete_prefix_first"]
    pub static mut _rl_menu_complete_prefix_first: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_vis_botlin"]
    pub static mut _rl_vis_botlin: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_last_c_pos"]
    pub static mut _rl_last_c_pos: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_suppress_redisplay"]
    pub static mut _rl_suppress_redisplay: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_want_redisplay"]
    pub static mut _rl_want_redisplay: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_emacs_mode_str"]
    pub static mut _rl_emacs_mode_str: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}_rl_emacs_modestr_len"]
    pub static mut _rl_emacs_modestr_len: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_vi_ins_mode_str"]
    pub static mut _rl_vi_ins_mode_str: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}_rl_vi_ins_modestr_len"]
    pub static mut _rl_vi_ins_modestr_len: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_vi_cmd_mode_str"]
    pub static mut _rl_vi_cmd_mode_str: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}_rl_vi_cmd_modestr_len"]
    pub static mut _rl_vi_cmd_modestr_len: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_isearch_terminators"]
    pub static mut _rl_isearch_terminators: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}_rl_iscxt"]
    pub static mut _rl_iscxt: *mut _rl_search_cxt;
}
extern "C" {
    #[link_name = "\u{1}_rl_executing_macro"]
    pub static mut _rl_executing_macro: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}_rl_history_preserve_point"]
    pub static mut _rl_history_preserve_point: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_history_saved_point"]
    pub static mut _rl_history_saved_point: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_argcxt"]
    pub static mut _rl_argcxt: _rl_arg_cxt;
}
extern "C" {
    #[link_name = "\u{1}_rl_utf8locale"]
    pub static mut _rl_utf8locale: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_echoing_p"]
    pub static mut _rl_echoing_p: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_horizontal_scroll_mode"]
    pub static mut _rl_horizontal_scroll_mode: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_mark_modified_lines"]
    pub static mut _rl_mark_modified_lines: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_bell_preference"]
    pub static mut _rl_bell_preference: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_meta_flag"]
    pub static mut _rl_meta_flag: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_convert_meta_chars_to_ascii"]
    pub static mut _rl_convert_meta_chars_to_ascii: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_output_meta_chars"]
    pub static mut _rl_output_meta_chars: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_bind_stty_chars"]
    pub static mut _rl_bind_stty_chars: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_revert_all_at_newline"]
    pub static mut _rl_revert_all_at_newline: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_echo_control_chars"]
    pub static mut _rl_echo_control_chars: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_show_mode_in_prompt"]
    pub static mut _rl_show_mode_in_prompt: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_enable_bracketed_paste"]
    pub static mut _rl_enable_bracketed_paste: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_enable_active_region"]
    pub static mut _rl_enable_active_region: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_comment_begin"]
    pub static mut _rl_comment_begin: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}_rl_parsing_conditionalized_out"]
    pub static mut _rl_parsing_conditionalized_out: ::std::os::raw::c_uchar;
}
extern "C" {
    #[link_name = "\u{1}_rl_keymap"]
    pub static mut _rl_keymap: Keymap;
}
extern "C" {
    #[link_name = "\u{1}_rl_in_stream"]
    pub static mut _rl_in_stream: *mut FILE;
}
extern "C" {
    #[link_name = "\u{1}_rl_out_stream"]
    pub static mut _rl_out_stream: *mut FILE;
}
extern "C" {
    #[link_name = "\u{1}_rl_last_command_was_kill"]
    pub static mut _rl_last_command_was_kill: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_eof_char"]
    pub static mut _rl_eof_char: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_eof_found"]
    pub static mut _rl_eof_found: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_top_level"]
    pub static mut _rl_top_level: sigjmp_buf;
}
extern "C" {
    #[link_name = "\u{1}_rl_kscxt"]
    pub static mut _rl_kscxt: *mut _rl_keyseq_cxt;
}
extern "C" {
    #[link_name = "\u{1}_rl_keyseq_timeout"]
    pub static mut _rl_keyseq_timeout: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_executing_keyseq_size"]
    pub static mut _rl_executing_keyseq_size: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_internal_startup_hook"]
    pub static mut _rl_internal_startup_hook: rl_hook_func_t;
}
extern "C" {
    #[link_name = "\u{1}_rl_nscxt"]
    pub static mut _rl_nscxt: *mut _rl_search_cxt;
}
extern "C" {
    #[link_name = "\u{1}_rl_caught_signal"]
    pub static mut _rl_caught_signal: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_sigcleanup"]
    pub static mut _rl_sigcleanup: _rl_sigcleanup_func_t;
}
extern "C" {
    #[link_name = "\u{1}_rl_sigcleanarg"]
    pub static mut _rl_sigcleanarg: *mut ::std::os::raw::c_void;
}
extern "C" {
    #[link_name = "\u{1}_rl_echoctl"]
    pub static mut _rl_echoctl: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_intr_char"]
    pub static mut _rl_intr_char: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_quit_char"]
    pub static mut _rl_quit_char: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_susp_char"]
    pub static mut _rl_susp_char: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_enable_keypad"]
    pub static mut _rl_enable_keypad: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_enable_meta"]
    pub static mut _rl_enable_meta: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_term_clreol"]
    pub static mut _rl_term_clreol: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}_rl_term_clrpag"]
    pub static mut _rl_term_clrpag: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}_rl_term_clrscroll"]
    pub static mut _rl_term_clrscroll: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}_rl_term_im"]
    pub static mut _rl_term_im: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}_rl_term_ic"]
    pub static mut _rl_term_ic: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}_rl_term_ei"]
    pub static mut _rl_term_ei: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}_rl_term_DC"]
    pub static mut _rl_term_DC: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}_rl_term_up"]
    pub static mut _rl_term_up: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}_rl_term_dc"]
    pub static mut _rl_term_dc: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}_rl_term_cr"]
    pub static mut _rl_term_cr: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}_rl_term_IC"]
    pub static mut _rl_term_IC: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}_rl_term_forward_char"]
    pub static mut _rl_term_forward_char: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}_rl_screenheight"]
    pub static mut _rl_screenheight: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_screenwidth"]
    pub static mut _rl_screenwidth: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_screenchars"]
    pub static mut _rl_screenchars: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_terminal_can_insert"]
    pub static mut _rl_terminal_can_insert: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_term_autowrap"]
    pub static mut _rl_term_autowrap: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_optimize_typeahead"]
    pub static mut _rl_optimize_typeahead: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_keep_mark_active"]
    pub static mut _rl_keep_mark_active: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_doing_an_undo"]
    pub static mut _rl_doing_an_undo: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_undo_group_level"]
    pub static mut _rl_undo_group_level: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_vi_last_command"]
    pub static mut _rl_vi_last_command: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_vi_redoing"]
    pub static mut _rl_vi_redoing: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_rl_vimvcxt"]
    pub static mut _rl_vimvcxt: *mut _rl_vimotion_cxt;
}
extern "C" {
    pub fn opendir(__name: *const ::std::os::raw::c_char) -> *mut DIR;
}
extern "C" {
    pub fn fdopendir(__fd: ::std::os::raw::c_int) -> *mut DIR;
}
extern "C" {
    pub fn closedir(__dirp: *mut DIR) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn readdir(__dirp: *mut DIR) -> *mut dirent;
}
extern "C" {
    pub fn readdir64(__dirp: *mut DIR) -> *mut dirent64;
}
extern "C" {
    pub fn readdir_r(
        __dirp: *mut DIR,
        __entry: *mut dirent,
        __result: *mut *mut dirent,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn readdir64_r(
        __dirp: *mut DIR,
        __entry: *mut dirent64,
        __result: *mut *mut dirent64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rewinddir(__dirp: *mut DIR);
}
extern "C" {
    pub fn seekdir(__dirp: *mut DIR, __pos: ::std::os::raw::c_long);
}
extern "C" {
    pub fn telldir(__dirp: *mut DIR) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn dirfd(__dirp: *mut DIR) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn scandir(
        __dir: *const ::std::os::raw::c_char,
        __namelist: *mut *mut *mut dirent,
        __selector: ::core::option::Option<
            unsafe extern "C" fn(arg1: *const dirent) -> ::std::os::raw::c_int,
        >,
        __cmp: ::core::option::Option<
            unsafe extern "C" fn(
                arg1: *mut *const dirent,
                arg2: *mut *const dirent,
            ) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn scandir64(
        __dir: *const ::std::os::raw::c_char,
        __namelist: *mut *mut *mut dirent64,
        __selector: ::core::option::Option<
            unsafe extern "C" fn(arg1: *const dirent64) -> ::std::os::raw::c_int,
        >,
        __cmp: ::core::option::Option<
            unsafe extern "C" fn(
                arg1: *mut *const dirent64,
                arg2: *mut *const dirent64,
            ) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn scandirat(
        __dfd: ::std::os::raw::c_int,
        __dir: *const ::std::os::raw::c_char,
        __namelist: *mut *mut *mut dirent,
        __selector: ::core::option::Option<
            unsafe extern "C" fn(arg1: *const dirent) -> ::std::os::raw::c_int,
        >,
        __cmp: ::core::option::Option<
            unsafe extern "C" fn(
                arg1: *mut *const dirent,
                arg2: *mut *const dirent,
            ) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn scandirat64(
        __dfd: ::std::os::raw::c_int,
        __dir: *const ::std::os::raw::c_char,
        __namelist: *mut *mut *mut dirent64,
        __selector: ::core::option::Option<
            unsafe extern "C" fn(arg1: *const dirent64) -> ::std::os::raw::c_int,
        >,
        __cmp: ::core::option::Option<
            unsafe extern "C" fn(
                arg1: *mut *const dirent64,
                arg2: *mut *const dirent64,
            ) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn alphasort(__e1: *mut *const dirent, __e2: *mut *const dirent) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn alphasort64(
        __e1: *mut *const dirent64,
        __e2: *mut *const dirent64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getdirentries(
        __fd: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __nbytes: usize,
        __basep: *mut __off_t,
    ) -> __ssize_t;
}
extern "C" {
    pub fn getdirentries64(
        __fd: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __nbytes: usize,
        __basep: *mut __off64_t,
    ) -> __ssize_t;
}
extern "C" {
    pub fn versionsort(__e1: *mut *const dirent, __e2: *mut *const dirent)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn versionsort64(
        __e1: *mut *const dirent64,
        __e2: *mut *const dirent64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _rl_parse_colors();
}
extern "C" {
    #[link_name = "\u{1}indicator_name"]
    pub static mut indicator_name: [*const ::std::os::raw::c_char; 25usize];
}
extern "C" {
    #[link_name = "\u{1}color_buf"]
    pub static mut color_buf: *mut ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct winsize {
    pub ws_row: ::std::os::raw::c_ushort,
    pub ws_col: ::std::os::raw::c_ushort,
    pub ws_xpixel: ::std::os::raw::c_ushort,
    pub ws_ypixel: ::std::os::raw::c_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct termio {
    pub c_iflag: ::std::os::raw::c_ushort,
    pub c_oflag: ::std::os::raw::c_ushort,
    pub c_cflag: ::std::os::raw::c_ushort,
    pub c_lflag: ::std::os::raw::c_ushort,
    pub c_line: ::std::os::raw::c_uchar,
    pub c_cc: [::std::os::raw::c_uchar; 8usize],
}
extern "C" {
    pub fn ioctl(
        __fd: ::std::os::raw::c_int,
        __request: ::std::os::raw::c_ulong,
        ...
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _rl_tty_chars {
    pub t_eof: ::std::os::raw::c_uchar,
    pub t_eol: ::std::os::raw::c_uchar,
    pub t_eol2: ::std::os::raw::c_uchar,
    pub t_erase: ::std::os::raw::c_uchar,
    pub t_werase: ::std::os::raw::c_uchar,
    pub t_kill: ::std::os::raw::c_uchar,
    pub t_reprint: ::std::os::raw::c_uchar,
    pub t_intr: ::std::os::raw::c_uchar,
    pub t_quit: ::std::os::raw::c_uchar,
    pub t_susp: ::std::os::raw::c_uchar,
    pub t_dsusp: ::std::os::raw::c_uchar,
    pub t_start: ::std::os::raw::c_uchar,
    pub t_stop: ::std::os::raw::c_uchar,
    pub t_lnext: ::std::os::raw::c_uchar,
    pub t_flush: ::std::os::raw::c_uchar,
    pub t_status: ::std::os::raw::c_uchar,
}
pub type _RL_TTY_CHARS = _rl_tty_chars;
extern "C" {
    #[link_name = "\u{1}PC"]
    pub static mut PC: ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}UP"]
    pub static mut UP: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}BC"]
    pub static mut BC: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}ospeed"]
    pub static mut ospeed: ::std::os::raw::c_short;
}
extern "C" {
    pub fn tgetstr(
        arg1: *const ::std::os::raw::c_char,
        arg2: *mut *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tgoto(
        arg1: *const ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tgetent(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tgetflag(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tgetnum(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tputs(
        arg1: *const ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: ::core::option::Option<
            unsafe extern "C" fn(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}

extern "C" {
    pub fn fstat(__fd: ::std::os::raw::c_int, __buf: *mut stat) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stat64(
        __file: *const ::std::os::raw::c_char,
        __buf: *mut stat64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fstat64(__fd: ::std::os::raw::c_int, __buf: *mut stat64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fstatat(
        __fd: ::std::os::raw::c_int,
        __file: *const ::std::os::raw::c_char,
        __buf: *mut stat,
        __flag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fstatat64(
        __fd: ::std::os::raw::c_int,
        __file: *const ::std::os::raw::c_char,
        __buf: *mut stat64,
        __flag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lstat64(
        __file: *const ::std::os::raw::c_char,
        __buf: *mut stat64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn chmod(__file: *const ::std::os::raw::c_char, __mode: __mode_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lchmod(__file: *const ::std::os::raw::c_char, __mode: __mode_t)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fchmod(__fd: ::std::os::raw::c_int, __mode: __mode_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fchmodat(
        __fd: ::std::os::raw::c_int,
        __file: *const ::std::os::raw::c_char,
        __mode: __mode_t,
        __flag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn umask(__mask: __mode_t) -> __mode_t;
}
extern "C" {
    pub fn getumask() -> __mode_t;
}
extern "C" {
    pub fn mkdir(__path: *const ::std::os::raw::c_char, __mode: __mode_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkdirat(
        __fd: ::std::os::raw::c_int,
        __path: *const ::std::os::raw::c_char,
        __mode: __mode_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mknod(
        __path: *const ::std::os::raw::c_char,
        __mode: __mode_t,
        __dev: __dev_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mknodat(
        __fd: ::std::os::raw::c_int,
        __path: *const ::std::os::raw::c_char,
        __mode: __mode_t,
        __dev: __dev_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkfifo(__path: *const ::std::os::raw::c_char, __mode: __mode_t)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkfifoat(
        __fd: ::std::os::raw::c_int,
        __path: *const ::std::os::raw::c_char,
        __mode: __mode_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn utimensat(
        __fd: ::std::os::raw::c_int,
        __path: *const ::std::os::raw::c_char,
        __times: *const timespec,
        __flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn futimens(__fd: ::std::os::raw::c_int, __times: *const timespec)
        -> ::std::os::raw::c_int;
}
// extern "C" {
//     pub fn __fxstat(
//         __ver: ::std::os::raw::c_int,
//         __fildes: ::std::os::raw::c_int,
//         __stat_buf: *mut stat,
//     ) -> ::std::os::raw::c_int;
// }
// extern "C" {
//     pub fn __xstat(
//         __ver: ::std::os::raw::c_int,
//         __filename: *const ::std::os::raw::c_char,
//         __stat_buf: *mut stat,
//     ) -> ::std::os::raw::c_int;
// }
// extern "C" {
//     pub fn __lxstat(
//         __ver: ::std::os::raw::c_int,
//         __filename: *const ::std::os::raw::c_char,
//         __stat_buf: *mut crate::src_common::stat,
//     ) -> ::std::os::raw::c_int;
// }
extern "C" {
    pub fn __fxstatat(
        __ver: ::std::os::raw::c_int,
        __fildes: ::std::os::raw::c_int,
        __filename: *const ::std::os::raw::c_char,
        __stat_buf: *mut stat,
        __flag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __fxstat64(
        __ver: ::std::os::raw::c_int,
        __fildes: ::std::os::raw::c_int,
        __stat_buf: *mut stat64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __xstat64(
        __ver: ::std::os::raw::c_int,
        __filename: *const ::std::os::raw::c_char,
        __stat_buf: *mut stat64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __lxstat64(
        __ver: ::std::os::raw::c_int,
        __filename: *const ::std::os::raw::c_char,
        __stat_buf: *mut stat64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __fxstatat64(
        __ver: ::std::os::raw::c_int,
        __fildes: ::std::os::raw::c_int,
        __filename: *const ::std::os::raw::c_char,
        __stat_buf: *mut stat64,
        __flag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __xmknod(
        __ver: ::std::os::raw::c_int,
        __path: *const ::std::os::raw::c_char,
        __mode: __mode_t,
        __dev: *mut __dev_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __xmknodat(
        __ver: ::std::os::raw::c_int,
        __fd: ::std::os::raw::c_int,
        __path: *const ::std::os::raw::c_char,
        __mode: __mode_t,
        __dev: *mut __dev_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn statx(
        __dirfd: ::std::os::raw::c_int,
        __path: *const ::std::os::raw::c_char,
        __flags: ::std::os::raw::c_int,
        __mask: ::std::os::raw::c_uint,
        __buf: *mut statx,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _hs_history_patsearch(
        arg1: *const ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_adjtime(__clock_id: __clockid_t, __utx: *mut timex) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock() -> clock_t;
}
extern "C" {
    pub fn time(__timer: *mut time_t) -> time_t;
}
extern "C" {
    pub fn difftime(__time1: time_t, __time0: time_t) -> f64;
}
extern "C" {
    pub fn mktime(__tp: *mut tm) -> time_t;
}
extern "C" {
    pub fn strftime(
        __s: *mut ::std::os::raw::c_char,
        __maxsize: usize,
        __format: *const ::std::os::raw::c_char,
        __tp: *const tm,
    ) -> usize;
}
extern "C" {
    pub fn strptime(
        __s: *const ::std::os::raw::c_char,
        __fmt: *const ::std::os::raw::c_char,
        __tp: *mut tm,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strftime_l(
        __s: *mut ::std::os::raw::c_char,
        __maxsize: usize,
        __format: *const ::std::os::raw::c_char,
        __tp: *const tm,
        __loc: locale_t,
    ) -> usize;
}
extern "C" {
    pub fn strptime_l(
        __s: *const ::std::os::raw::c_char,
        __fmt: *const ::std::os::raw::c_char,
        __tp: *mut tm,
        __loc: locale_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn gmtime(__timer: *const time_t) -> *mut tm;
}
extern "C" {
    pub fn localtime(__timer: *const time_t) -> *mut tm;
}
extern "C" {
    pub fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
}
extern "C" {
    pub fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
}
extern "C" {
    pub fn asctime(__tp: *const tm) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ctime(__timer: *const time_t) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn asctime_r(
        __tp: *const tm,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ctime_r(
        __timer: *const time_t,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}__tzname"]
    pub static mut __tzname: [*mut ::std::os::raw::c_char; 2usize];
}
extern "C" {
    #[link_name = "\u{1}__daylight"]
    pub static mut __daylight: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__timezone"]
    pub static mut __timezone: ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}tzname"]
    pub static mut tzname: [*mut ::std::os::raw::c_char; 2usize];
}
extern "C" {
    pub fn tzset();
}
extern "C" {
    #[link_name = "\u{1}daylight"]
    pub static mut daylight: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}timezone"]
    pub static mut timezone: ::std::os::raw::c_long;
}
extern "C" {
    pub fn stime(__when: *const time_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timegm(__tp: *mut tm) -> time_t;
}
extern "C" {
    pub fn timelocal(__tp: *mut tm) -> time_t;
}
extern "C" {
    pub fn dysize(__year: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_getres(__clock_id: clockid_t, __res: *mut timespec) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_settime(__clock_id: clockid_t, __tp: *const timespec) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_nanosleep(
        __clock_id: clockid_t,
        __flags: ::std::os::raw::c_int,
        __req: *const timespec,
        __rem: *mut timespec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_getcpuclockid(__pid: pid_t, __clock_id: *mut clockid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_create(
        __clock_id: clockid_t,
        __evp: *mut sigevent,
        __timerid: *mut timer_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_delete(__timerid: timer_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_settime(
        __timerid: timer_t,
        __flags: ::std::os::raw::c_int,
        __value: *const itimerspec,
        __ovalue: *mut itimerspec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_gettime(__timerid: timer_t, __value: *mut itimerspec) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_getoverrun(__timerid: timer_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timespec_get(
        __ts: *mut timespec,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}getdate_err"]
    pub static mut getdate_err: ::std::os::raw::c_int;
}
extern "C" {
    pub fn getdate(__string: *const ::std::os::raw::c_char) -> *mut tm;
}
extern "C" {
    pub fn getdate_r(
        __string: *const ::std::os::raw::c_char,
        __resbufp: *mut tm,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _hist_state {
    pub entries: *mut *mut HIST_ENTRY,
    pub offset: ::std::os::raw::c_int,
    pub length: ::std::os::raw::c_int,
    pub size: ::std::os::raw::c_int,
    pub flags: ::std::os::raw::c_int,
}
pub type HISTORY_STATE = _hist_state;

extern "C" {
    pub fn history_get_history_state() -> *mut HISTORY_STATE;
}
extern "C" {
    pub fn history_set_history_state(arg1: *mut HISTORY_STATE);
}
extern "C" {
    pub fn add_history_time(arg1: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn alloc_history_entry(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *mut ::std::os::raw::c_char,
    ) -> *mut HIST_ENTRY;
}
extern "C" {
    pub fn copy_history_entry(arg1: *mut HIST_ENTRY) -> *mut HIST_ENTRY;
}
extern "C" {
    pub fn stifle_history(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn unstifle_history() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn current_history() -> *mut HIST_ENTRY;
}
extern "C" {
    pub fn history_get_time(arg1: *mut HIST_ENTRY) -> time_t;
}
extern "C" {
    pub fn history_total_bytes() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn next_history() -> *mut HIST_ENTRY;
}
extern "C" {
    pub fn history_search(
        arg1: *const ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn history_search_prefix(
        arg1: *const ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn history_search_pos(
        arg1: *const ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn read_history_range(
        arg1: *const ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn history_truncate_file(
        arg1: *const ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn history_arg_extract(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn get_history_event(
        arg1: *const ::std::os::raw::c_char,
        arg2: *mut ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn history_tokenize(
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}history_offset"]
    pub static mut history_offset: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}history_word_delimiters"]
    pub static mut history_word_delimiters: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}history_comment_char"]
    pub static mut history_comment_char: ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}history_no_expand_chars"]
    pub static mut history_no_expand_chars: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}history_write_timestamps"]
    pub static mut history_write_timestamps: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}history_multiline_entries"]
    pub static mut history_multiline_entries: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}history_file_version"]
    pub static mut history_file_version: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}max_input_history"]
    pub static mut max_input_history: ::std::os::raw::c_int;
}
// extern "C" {
//     #[link_name = "\u{1}stdin"]
//     pub static mut stdin: *mut FILE;
// }
extern "C" {
    pub fn remove(__filename: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rename(
        __old: *const ::std::os::raw::c_char,
        __new: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn renameat(
        __oldfd: ::std::os::raw::c_int,
        __old: *const ::std::os::raw::c_char,
        __newfd: ::std::os::raw::c_int,
        __new: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn renameat2(
        __oldfd: ::std::os::raw::c_int,
        __old: *const ::std::os::raw::c_char,
        __newfd: ::std::os::raw::c_int,
        __new: *const ::std::os::raw::c_char,
        __flags: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tmpfile() -> *mut FILE;
}
extern "C" {
    pub fn tmpfile64() -> *mut FILE;
}
extern "C" {
    pub fn tmpnam(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tmpnam_r(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tempnam(
        __dir: *const ::std::os::raw::c_char,
        __pfx: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fclose(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fflush_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fcloseall() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fopen(
        __filename: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn freopen(
        __filename: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fopen64(
        __filename: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn freopen64(
        __filename: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fdopen(__fd: ::std::os::raw::c_int, __modes: *const ::std::os::raw::c_char)
        -> *mut FILE;
}
extern "C" {
    pub fn fopencookie(
        __magic_cookie: *mut ::std::os::raw::c_void,
        __modes: *const ::std::os::raw::c_char,
        __io_funcs: cookie_io_functions_t,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fmemopen(
        __s: *mut ::std::os::raw::c_void,
        __len: usize,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn open_memstream(
        __bufloc: *mut *mut ::std::os::raw::c_char,
        __sizeloc: *mut usize,
    ) -> *mut FILE;
}
extern "C" {
    pub fn setbuf(__stream: *mut FILE, __buf: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut ::std::os::raw::c_char,
        __modes: ::std::os::raw::c_int,
        __n: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setbuffer(__stream: *mut FILE, __buf: *mut ::std::os::raw::c_char, __size: usize);
}
extern "C" {
    pub fn setlinebuf(__stream: *mut FILE);
}
extern "C" {
    pub fn vfprintf(
        __s: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vprintf(
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsprintf(
        __s: *mut ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn snprintf(
        __s: *mut ::std::os::raw::c_char,
        __maxlen: usize,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsnprintf(
        __s: *mut ::std::os::raw::c_char,
        __maxlen: usize,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vasprintf(
        __ptr: *mut *mut ::std::os::raw::c_char,
        __f: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __asprintf(
        __ptr: *mut *mut ::std::os::raw::c_char,
        __fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn asprintf(
        __ptr: *mut *mut ::std::os::raw::c_char,
        __fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vdprintf(
        __fd: ::std::os::raw::c_int,
        __fmt: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn dprintf(
        __fd: ::std::os::raw::c_int,
        __fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fscanf(
        __stream: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn scanf(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sscanf(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vfscanf(
        __s: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vscanf(
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsscanf(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetc(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getc(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getchar() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getc_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getchar_unlocked() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetc_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putchar(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputc_unlocked(__c: ::std::os::raw::c_int, __stream: *mut FILE)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putc_unlocked(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putchar_unlocked(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getw(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putw(__w: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgets(
        __s: *mut ::std::os::raw::c_char,
        __n: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fgets_unlocked(
        __s: *mut ::std::os::raw::c_char,
        __n: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __getdelim(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut usize,
        __delimiter: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
extern "C" {
    pub fn getdelim(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut usize,
        __delimiter: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
extern "C" {
    pub fn getline(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut usize,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
extern "C" {
    pub fn fputs(__s: *const ::std::os::raw::c_char, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn puts(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ungetc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fread(
        __ptr: *mut ::std::os::raw::c_void,
        __size: usize,
        __n: usize,
        __stream: *mut FILE,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn fwrite(
        __ptr: *const ::std::os::raw::c_void,
        __size: usize,
        __n: usize,
        __s: *mut FILE,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn fputs_unlocked(
        __s: *const ::std::os::raw::c_char,
        __stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fread_unlocked(
        __ptr: *mut ::std::os::raw::c_void,
        __size: usize,
        __n: usize,
        __stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn fwrite_unlocked(
        __ptr: *const ::std::os::raw::c_void,
        __size: usize,
        __n: usize,
        __stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn fseek(
        __stream: *mut FILE,
        __off: ::std::os::raw::c_long,
        __whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftell(__stream: *mut FILE) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn rewind(__stream: *mut FILE);
}
extern "C" {
    pub fn fseeko(
        __stream: *mut FILE,
        __off: __off_t,
        __whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftello(__stream: *mut FILE) -> __off_t;
}
extern "C" {
    pub fn fgetpos(__stream: *mut FILE, __pos: *mut fpos_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fsetpos(__stream: *mut FILE, __pos: *const fpos_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fseeko64(
        __stream: *mut FILE,
        __off: __off64_t,
        __whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftello64(__stream: *mut FILE) -> __off64_t;
}
extern "C" {
    pub fn fgetpos64(__stream: *mut FILE, __pos: *mut fpos64_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fsetpos64(__stream: *mut FILE, __pos: *const fpos64_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clearerr(__stream: *mut FILE);
}
extern "C" {
    pub fn feof(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ferror(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clearerr_unlocked(__stream: *mut FILE);
}
extern "C" {
    pub fn feof_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ferror_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn perror(__s: *const ::std::os::raw::c_char);
}
extern "C" {
    #[link_name = "\u{1}sys_nerr"]
    pub static mut sys_nerr: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}sys_errlist"]
    pub static mut sys_errlist: [*const ::std::os::raw::c_char; 0usize];
}
extern "C" {
    #[link_name = "\u{1}_sys_nerr"]
    pub static mut _sys_nerr: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_sys_errlist"]
    pub static mut _sys_errlist: [*const ::std::os::raw::c_char; 0usize];
}
extern "C" {
    pub fn fileno_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn popen(
        __command: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn pclose(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ctermid(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn cuserid(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn obstack_printf(
        __obstack: *mut obstack,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn obstack_vprintf(
        __obstack: *mut obstack,
        __format: *const ::std::os::raw::c_char,
        __args: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn flockfile(__stream: *mut FILE);
}
extern "C" {
    pub fn ftrylockfile(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn funlockfile(__stream: *mut FILE);
}
extern "C" {
    pub fn __uflow(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __overflow(arg1: *mut FILE, arg2: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bin_str {
    pub len: usize,
    pub string: *const ::std::os::raw::c_char,
}
extern "C" {
    #[link_name = "\u{1}_rl_color_indicator"]
    pub static mut _rl_color_indicator: [bin_str; 0usize];
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _color_ext_type {
    pub ext: bin_str,
    pub seq: bin_str,
    pub next: *mut _color_ext_type,
}
pub type COLOR_EXT_TYPE = _color_ext_type;
extern "C" {
    #[link_name = "\u{1}_rl_color_ext_list"]
    pub static mut _rl_color_ext_list: *mut COLOR_EXT_TYPE;
}
pub const indicator_no_C_LEFT: indicator_no = 0;
pub const indicator_no_C_RIGHT: indicator_no = 1;
pub const indicator_no_C_END: indicator_no = 2;
pub const indicator_no_C_RESET: indicator_no = 3;
pub const indicator_no_C_NORM: indicator_no = 4;
pub const indicator_no_C_FILE: indicator_no = 5;
pub const indicator_no_C_DIR: indicator_no = 6;
pub const indicator_no_C_LINK: indicator_no = 7;
pub const indicator_no_C_FIFO: indicator_no = 8;
pub const indicator_no_C_SOCK: indicator_no = 9;
pub const indicator_no_C_BLK: indicator_no = 10;
pub const indicator_no_C_CHR: indicator_no = 11;
pub const indicator_no_C_MISSING: indicator_no = 12;
pub const indicator_no_C_ORPHAN: indicator_no = 13;
pub const indicator_no_C_EXEC: indicator_no = 14;
pub const indicator_no_C_DOOR: indicator_no = 15;
pub const indicator_no_C_SETUID: indicator_no = 16;
pub const indicator_no_C_SETGID: indicator_no = 17;
pub const indicator_no_C_STICKY: indicator_no = 18;
pub const indicator_no_C_OTHER_WRITABLE: indicator_no = 19;
pub const indicator_no_C_STICKY_OTHER_WRITABLE: indicator_no = 20;
pub const indicator_no_C_CAP: indicator_no = 21;
pub const indicator_no_C_MULTIHARDLINK: indicator_no = 22;
pub const indicator_no_C_CLR_TO_EOL: indicator_no = 23;
pub type indicator_no = u32;
pub const filetype_unknown: filetype = 0;
pub const filetype_fifo: filetype = 1;
pub const filetype_chardev: filetype = 2;
pub const filetype_directory: filetype = 3;
pub const filetype_blockdev: filetype = 4;
pub const filetype_normal: filetype = 5;
pub const filetype_symbolic_link: filetype = 6;
pub const filetype_sock: filetype = 7;
pub const filetype_whiteout: filetype = 8;
pub const filetype_arg_directory: filetype = 9;
pub type filetype = u32;
extern "C" {
    pub fn _rl_put_indicator(ind: *const bin_str);
}
extern "C" {
    pub fn _rl_set_normal_color();
}
extern "C" {
    pub fn _rl_print_prefix_color() -> bool;
}
extern "C" {
    pub fn _rl_print_color_indicator(f: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn _rl_prep_non_filename_text();
}
