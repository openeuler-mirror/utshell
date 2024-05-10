//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later


use libc::{c_int};
use r_bash::*;
#[no_mangle]
pub static mut sh_syntabsiz:c_int = 256;

#[no_mangle]
pub static mut sh_syntaxtab:[c_int; 256] = [
    CWORD as c_int,		/* 0 */
	CSPECL as c_int,		/* CTLESC */
	CWORD as c_int,		/* 2 */
	CWORD as c_int,		/* 3 */
	CWORD as c_int,		/* 4 */
	CWORD as c_int,		/* 5 */
	CWORD as c_int,		/* 6 */
	CWORD as c_int,		/* \a */
	CWORD as c_int,		/* \b */
	CSHBRK as c_int|CBLANK as c_int,		/* \t */
	CSHBRK as c_int|CBSDQUOTE as c_int,		/* \n */
	CWORD as c_int,		/* \v */
	CWORD as c_int,		/* \f */
	CWORD as c_int,		/* \r */
	CWORD as c_int,		/* 14 */
	CWORD as c_int,		/* 15 */
	CWORD as c_int,		/* 16 */
	CWORD as c_int,		/* 17 */
	CWORD as c_int,		/* 18 */
	CWORD as c_int,		/* 19 */
	CWORD as c_int,		/* 20 */
	CWORD as c_int,		/* 21 */
	CWORD as c_int,		/* 22 */
	CWORD as c_int,		/* 23 */
	CWORD as c_int,		/* 24 */
	CWORD as c_int,		/* 25 */
	CWORD as c_int,		/* 26 */
	CWORD as c_int,		/* ESC */
	CWORD as c_int,		/* 28 */
	CWORD as c_int,		/* 29 */
	CWORD as c_int,		/* 30 */
	CWORD as c_int,		/* 31 */
	CSHBRK as c_int|CBLANK as c_int,		/* SPC */
	CXGLOB as c_int|CSPECVAR as c_int,		/* ! */
	CQUOTE as c_int|CBSDQUOTE as c_int|CXQUOTE as c_int,		/* " */
	CSPECVAR as c_int,		/* # */
	CEXP as c_int|CBSDQUOTE as c_int|CBSHDOC as c_int|CSPECVAR as c_int,		/* $ */
	CWORD as c_int,		/* % */
	CSHMETA as c_int|CSHBRK as c_int,		/* & */
	CQUOTE as c_int|CXQUOTE as c_int,		/* ' */
	CSHMETA as c_int|CSHBRK as c_int,		/* ( */
	CSHMETA as c_int|CSHBRK as c_int,		/* ) */
	CGLOB as c_int|CXGLOB as c_int|CSPECVAR as c_int,		/* * */
	CXGLOB as c_int|CSUBSTOP as c_int,		/* + */
	CWORD as c_int,		/* , */
	CSPECVAR as c_int|CSUBSTOP as c_int,		/* - */
	CWORD as c_int,		/* . */
	CWORD as c_int,		/* / */
	CWORD as c_int,		/* 0 */
	CWORD as c_int,		/* 1 */
	CWORD as c_int,		/* 2 */
	CWORD as c_int,		/* 3 */
	CWORD as c_int,		/* 4 */
	CWORD as c_int,		/* 5 */
	CWORD as c_int,		/* 6 */
	CWORD as c_int,		/* 7 */
	CWORD as c_int,		/* 8 */
	CWORD as c_int,		/* 9 */
	CWORD as c_int,		/* : */
	CSHMETA as c_int|CSHBRK as c_int,		/* ; */
	CSHMETA as c_int|CSHBRK as c_int|CEXP as c_int,		/* < */
	CSUBSTOP as c_int,		/* = */
	CSHMETA as c_int|CSHBRK as c_int|CEXP as c_int,		/* > */
	CGLOB as c_int|CXGLOB as c_int|CSPECVAR as c_int|CSUBSTOP as c_int,		/* ? */
	CXGLOB as c_int|CSPECVAR as c_int,		/* @ */
	CWORD as c_int,		/* A */
	CWORD as c_int,		/* B */
	CWORD as c_int,		/* C */
	CWORD as c_int,		/* D */
	CWORD as c_int,		/* E */
	CWORD as c_int,		/* F */
	CWORD as c_int,		/* G */
	CWORD as c_int,		/* H */
	CWORD as c_int,		/* I */
	CWORD as c_int,		/* J */
	CWORD as c_int,		/* K */
	CWORD as c_int,		/* L */
	CWORD as c_int,		/* M */
	CWORD as c_int,		/* N */
	CWORD as c_int,		/* O */
	CWORD as c_int,		/* P */
	CWORD as c_int,		/* Q */
	CWORD as c_int,		/* R */
	CWORD as c_int,		/* S */
	CWORD as c_int,		/* T */
	CWORD as c_int,		/* U */
	CWORD as c_int,		/* V */
	CWORD as c_int,		/* W */
	CWORD as c_int,		/* X */
	CWORD as c_int,		/* Y */
	CWORD as c_int,		/* Z */
	CGLOB as c_int,		/* [ */
	CBSDQUOTE as c_int|CBSHDOC as c_int|CXQUOTE as c_int,		/* \ */
	CGLOB as c_int,		/* ] */
	CGLOB as c_int,		/* ^ */
	CWORD as c_int,		/* _ */
	CBACKQ as c_int|CQUOTE as c_int|CBSDQUOTE as c_int|CBSHDOC as c_int|CXQUOTE as c_int,		/* ` */
	CWORD as c_int,		/* a */
	CWORD as c_int,		/* b */
	CWORD as c_int,		/* c */
	CWORD as c_int,		/* d */
	CWORD as c_int,		/* e */
	CWORD as c_int,		/* f */
	CWORD as c_int,		/* g */
	CWORD as c_int,		/* h */
	CWORD as c_int,		/* i */
	CWORD as c_int,		/* j */
	CWORD as c_int,		/* k */
	CWORD as c_int,		/* l */
	CWORD as c_int,		/* m */
	CWORD as c_int,		/* n */
	CWORD as c_int,		/* o */
	CWORD as c_int,		/* p */
	CWORD as c_int,		/* q */
	CWORD as c_int,		/* r */
	CWORD as c_int,		/* s */
	CWORD as c_int,		/* t */
	CWORD as c_int,		/* u */
	CWORD as c_int,		/* v */
	CWORD as c_int,		/* w */
	CWORD as c_int,		/* x */
	CWORD as c_int,		/* y */
	CWORD as c_int,		/* z */
	CWORD as c_int,		/* { */
	CSHMETA as c_int|CSHBRK as c_int,		/* | */
	CWORD as c_int,		/* } */
	CWORD as c_int,		/* ~ */
	CSPECL as c_int,		/* CTLNUL */
	CWORD as c_int,		/* 128 */
	CWORD as c_int,		/* 129 */
	CWORD as c_int,		/* 130 */
	CWORD as c_int,		/* 131 */
	CWORD as c_int,		/* 132 */
	CWORD as c_int,		/* 133 */
	CWORD as c_int,		/* 134 */
	CWORD as c_int,		/* 135 */
	CWORD as c_int,		/* 136 */
	CWORD as c_int,		/* 137 */
	CWORD as c_int,		/* 138 */
	CWORD as c_int,		/* 139 */
	CWORD as c_int,		/* 140 */
	CWORD as c_int,		/* 141 */
	CWORD as c_int,		/* 142 */
	CWORD as c_int,		/* 143 */
	CWORD as c_int,		/* 144 */
	CWORD as c_int,		/* 145 */
	CWORD as c_int,		/* 146 */
	CWORD as c_int,		/* 147 */
	CWORD as c_int,		/* 148 */
	CWORD as c_int,		/* 149 */
	CWORD as c_int,		/* 150 */
	CWORD as c_int,		/* 151 */
	CWORD as c_int,		/* 152 */
	CWORD as c_int,		/* 153 */
	CWORD as c_int,		/* 154 */
	CWORD as c_int,		/* 155 */
	CWORD as c_int,		/* 156 */
	CWORD as c_int,		/* 157 */
	CWORD as c_int,		/* 158 */
	CWORD as c_int,		/* 159 */
	CWORD as c_int,		/* 160 */
	CWORD as c_int,		/* 161 */
	CWORD as c_int,		/* 162 */
	CWORD as c_int,		/* 163 */
	CWORD as c_int,		/* 164 */
	CWORD as c_int,		/* 165 */
	CWORD as c_int,		/* 166 */
	CWORD as c_int,		/* 167 */
	CWORD as c_int,		/* 168 */
	CWORD as c_int,		/* 169 */
	CWORD as c_int,		/* 170 */
	CWORD as c_int,		/* 171 */
	CWORD as c_int,		/* 172 */
	CWORD as c_int,		/* 173 */
	CWORD as c_int,		/* 174 */
	CWORD as c_int,		/* 175 */
	CWORD as c_int,		/* 176 */
	CWORD as c_int,		/* 177 */
	CWORD as c_int,		/* 178 */
	CWORD as c_int,		/* 179 */
	CWORD as c_int,		/* 180 */
	CWORD as c_int,		/* 181 */
	CWORD as c_int,		/* 182 */
	CWORD as c_int,		/* 183 */
	CWORD as c_int,		/* 184 */
	CWORD as c_int,		/* 185 */
	CWORD as c_int,		/* 186 */
	CWORD as c_int,		/* 187 */
	CWORD as c_int,		/* 188 */
	CWORD as c_int,		/* 189 */
	CWORD as c_int,		/* 190 */
	CWORD as c_int,		/* 191 */
	CWORD as c_int,		/* 192 */
	CWORD as c_int,		/* 193 */
	CWORD as c_int,		/* 194 */
	CWORD as c_int,		/* 195 */
	CWORD as c_int,		/* 196 */
	CWORD as c_int,		/* 197 */
	CWORD as c_int,		/* 198 */
	CWORD as c_int,		/* 199 */
	CWORD as c_int,		/* 200 */
	CWORD as c_int,		/* 201 */
	CWORD as c_int,		/* 202 */
	CWORD as c_int,		/* 203 */
	CWORD as c_int,		/* 204 */
	CWORD as c_int,		/* 205 */
	CWORD as c_int,		/* 206 */
	CWORD as c_int,		/* 207 */
	CWORD as c_int,		/* 208 */
	CWORD as c_int,		/* 209 */
	CWORD as c_int,		/* 210 */
	CWORD as c_int,		/* 211 */
	CWORD as c_int,		/* 212 */
	CWORD as c_int,		/* 213 */
	CWORD as c_int,		/* 214 */
	CWORD as c_int,		/* 215 */
	CWORD as c_int,		/* 216 */
	CWORD as c_int,		/* 217 */
	CWORD as c_int,		/* 218 */
	CWORD as c_int,		/* 219 */
	CWORD as c_int,		/* 220 */
	CWORD as c_int,		/* 221 */
	CWORD as c_int,		/* 222 */
	CWORD as c_int,		/* 223 */
	CWORD as c_int,		/* 224 */
	CWORD as c_int,		/* 225 */
	CWORD as c_int,		/* 226 */
	CWORD as c_int,		/* 227 */
	CWORD as c_int,		/* 228 */
	CWORD as c_int,		/* 229 */
	CWORD as c_int,		/* 230 */
	CWORD as c_int,		/* 231 */
	CWORD as c_int,		/* 232 */
	CWORD as c_int,		/* 233 */
	CWORD as c_int,		/* 234 */
	CWORD as c_int,		/* 235 */
	CWORD as c_int,		/* 236 */
	CWORD as c_int,		/* 237 */
	CWORD as c_int,		/* 238 */
	CWORD as c_int,		/* 239 */
	CWORD as c_int,		/* 240 */
	CWORD as c_int,		/* 241 */
	CWORD as c_int,		/* 242 */
	CWORD as c_int,		/* 243 */
	CWORD as c_int,		/* 244 */
	CWORD as c_int,		/* 245 */
	CWORD as c_int,		/* 246 */
	CWORD as c_int,		/* 247 */
	CWORD as c_int,		/* 248 */
	CWORD as c_int,		/* 249 */
	CWORD as c_int,		/* 250 */
	CWORD as c_int,		/* 251 */
	CWORD as c_int,		/* 252 */
	CWORD as c_int,		/* 253 */
	CWORD as c_int,		/* 254 */
	CWORD as c_int,		/* 255 */
];





