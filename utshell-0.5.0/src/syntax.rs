use crate::src_common::*;
// use libc::c_int;
// use r_bash::*;
#[no_mangle]
pub static mut sh_syntabsiz: libc::c_int = 256;

#[no_mangle]
pub static mut sh_syntaxtab: [libc::c_int; 256] = [
    CWORD as libc::c_int,                             /* 0 */
    CSPECL as libc::c_int,                            /* CTLESC */
    CWORD as libc::c_int,                             /* 2 */
    CWORD as libc::c_int,                             /* 3 */
    CWORD as libc::c_int,                             /* 4 */
    CWORD as libc::c_int,                             /* 5 */
    CWORD as libc::c_int,                             /* 6 */
    CWORD as libc::c_int,                             /* \a */
    CWORD as libc::c_int,                             /* \b */
    CSHBRK as libc::c_int | CBLANK as libc::c_int,    /* \t */
    CSHBRK as libc::c_int | CBSDQUOTE as libc::c_int, /* \n */
    CWORD as libc::c_int,                             /* \v */
    CWORD as libc::c_int,                             /* \f */
    CWORD as libc::c_int,                             /* \r */
    CWORD as libc::c_int,                             /* 14 */
    CWORD as libc::c_int,                             /* 15 */
    CWORD as libc::c_int,                             /* 16 */
    CWORD as libc::c_int,                             /* 17 */
    CWORD as libc::c_int,                             /* 18 */
    CWORD as libc::c_int,                             /* 19 */
    CWORD as libc::c_int,                             /* 20 */
    CWORD as libc::c_int,                             /* 21 */
    CWORD as libc::c_int,                             /* 22 */
    CWORD as libc::c_int,                             /* 23 */
    CWORD as libc::c_int,                             /* 24 */
    CWORD as libc::c_int,                             /* 25 */
    CWORD as libc::c_int,                             /* 26 */
    CWORD as libc::c_int,                             /* ESC */
    CWORD as libc::c_int,                             /* 28 */
    CWORD as libc::c_int,                             /* 29 */
    CWORD as libc::c_int,                             /* 30 */
    CWORD as libc::c_int,                             /* 31 */
    CSHBRK as libc::c_int | CBLANK as libc::c_int,    /* SPC */
    CXGLOB as libc::c_int | CSPECVAR as libc::c_int,  /* ! */
    CQUOTE as libc::c_int | CBSDQUOTE as libc::c_int | CXQUOTE as libc::c_int, /* " */
    CSPECVAR as libc::c_int,                          /* # */
    CEXP as libc::c_int
        | CBSDQUOTE as libc::c_int
        | CBSHDOC as libc::c_int
        | CSPECVAR as libc::c_int, /* $ */
    CWORD as libc::c_int,                             /* % */
    CSHMETA as libc::c_int | CSHBRK as libc::c_int,   /* & */
    CQUOTE as libc::c_int | CXQUOTE as libc::c_int,   /* ' */
    CSHMETA as libc::c_int | CSHBRK as libc::c_int,   /* ( */
    CSHMETA as libc::c_int | CSHBRK as libc::c_int,   /* ) */
    CGLOB as libc::c_int | CXGLOB as libc::c_int | CSPECVAR as libc::c_int, /* * */
    CXGLOB as libc::c_int | CSUBSTOP as libc::c_int,  /* + */
    CWORD as libc::c_int,                             /* , */
    CSPECVAR as libc::c_int | CSUBSTOP as libc::c_int, /* - */
    CWORD as libc::c_int,                             /* . */
    CWORD as libc::c_int,                             /* / */
    CWORD as libc::c_int,                             /* 0 */
    CWORD as libc::c_int,                             /* 1 */
    CWORD as libc::c_int,                             /* 2 */
    CWORD as libc::c_int,                             /* 3 */
    CWORD as libc::c_int,                             /* 4 */
    CWORD as libc::c_int,                             /* 5 */
    CWORD as libc::c_int,                             /* 6 */
    CWORD as libc::c_int,                             /* 7 */
    CWORD as libc::c_int,                             /* 8 */
    CWORD as libc::c_int,                             /* 9 */
    CWORD as libc::c_int,                             /* : */
    CSHMETA as libc::c_int | CSHBRK as libc::c_int,   /* ; */
    CSHMETA as libc::c_int | CSHBRK as libc::c_int | CEXP as libc::c_int, /* < */
    CSUBSTOP as libc::c_int,                          /* = */
    CSHMETA as libc::c_int | CSHBRK as libc::c_int | CEXP as libc::c_int, /* > */
    CGLOB as libc::c_int
        | CXGLOB as libc::c_int
        | CSPECVAR as libc::c_int
        | CSUBSTOP as libc::c_int, /* ? */
    CXGLOB as libc::c_int | CSPECVAR as libc::c_int,  /* @ */
    CWORD as libc::c_int,                             /* A */
    CWORD as libc::c_int,                             /* B */
    CWORD as libc::c_int,                             /* C */
    CWORD as libc::c_int,                             /* D */
    CWORD as libc::c_int,                             /* E */
    CWORD as libc::c_int,                             /* F */
    CWORD as libc::c_int,                             /* G */
    CWORD as libc::c_int,                             /* H */
    CWORD as libc::c_int,                             /* I */
    CWORD as libc::c_int,                             /* J */
    CWORD as libc::c_int,                             /* K */
    CWORD as libc::c_int,                             /* L */
    CWORD as libc::c_int,                             /* M */
    CWORD as libc::c_int,                             /* N */
    CWORD as libc::c_int,                             /* O */
    CWORD as libc::c_int,                             /* P */
    CWORD as libc::c_int,                             /* Q */
    CWORD as libc::c_int,                             /* R */
    CWORD as libc::c_int,                             /* S */
    CWORD as libc::c_int,                             /* T */
    CWORD as libc::c_int,                             /* U */
    CWORD as libc::c_int,                             /* V */
    CWORD as libc::c_int,                             /* W */
    CWORD as libc::c_int,                             /* X */
    CWORD as libc::c_int,                             /* Y */
    CWORD as libc::c_int,                             /* Z */
    CGLOB as libc::c_int,                             /* [ */
    CBSDQUOTE as libc::c_int | CBSHDOC as libc::c_int | CXQUOTE as libc::c_int, /* \ */
    CGLOB as libc::c_int,                             /* ] */
    CGLOB as libc::c_int,                             /* ^ */
    CWORD as libc::c_int,                             /* _ */
    CBACKQ as libc::c_int
        | CQUOTE as libc::c_int
        | CBSDQUOTE as libc::c_int
        | CBSHDOC as libc::c_int
        | CXQUOTE as libc::c_int, /* ` */
    CWORD as libc::c_int,                             /* a */
    CWORD as libc::c_int,                             /* b */
    CWORD as libc::c_int,                             /* c */
    CWORD as libc::c_int,                             /* d */
    CWORD as libc::c_int,                             /* e */
    CWORD as libc::c_int,                             /* f */
    CWORD as libc::c_int,                             /* g */
    CWORD as libc::c_int,                             /* h */
    CWORD as libc::c_int,                             /* i */
    CWORD as libc::c_int,                             /* j */
    CWORD as libc::c_int,                             /* k */
    CWORD as libc::c_int,                             /* l */
    CWORD as libc::c_int,                             /* m */
    CWORD as libc::c_int,                             /* n */
    CWORD as libc::c_int,                             /* o */
    CWORD as libc::c_int,                             /* p */
    CWORD as libc::c_int,                             /* q */
    CWORD as libc::c_int,                             /* r */
    CWORD as libc::c_int,                             /* s */
    CWORD as libc::c_int,                             /* t */
    CWORD as libc::c_int,                             /* u */
    CWORD as libc::c_int,                             /* v */
    CWORD as libc::c_int,                             /* w */
    CWORD as libc::c_int,                             /* x */
    CWORD as libc::c_int,                             /* y */
    CWORD as libc::c_int,                             /* z */
    CWORD as libc::c_int,                             /* { */
    CSHMETA as libc::c_int | CSHBRK as libc::c_int,   /* | */
    CWORD as libc::c_int,                             /* } */
    CWORD as libc::c_int,                             /* ~ */
    CSPECL as libc::c_int,                            /* CTLNUL */
    CWORD as libc::c_int,                             /* 128 */
    CWORD as libc::c_int,                             /* 129 */
    CWORD as libc::c_int,                             /* 130 */
    CWORD as libc::c_int,                             /* 131 */
    CWORD as libc::c_int,                             /* 132 */
    CWORD as libc::c_int,                             /* 133 */
    CWORD as libc::c_int,                             /* 134 */
    CWORD as libc::c_int,                             /* 135 */
    CWORD as libc::c_int,                             /* 136 */
    CWORD as libc::c_int,                             /* 137 */
    CWORD as libc::c_int,                             /* 138 */
    CWORD as libc::c_int,                             /* 139 */
    CWORD as libc::c_int,                             /* 140 */
    CWORD as libc::c_int,                             /* 141 */
    CWORD as libc::c_int,                             /* 142 */
    CWORD as libc::c_int,                             /* 143 */
    CWORD as libc::c_int,                             /* 144 */
    CWORD as libc::c_int,                             /* 145 */
    CWORD as libc::c_int,                             /* 146 */
    CWORD as libc::c_int,                             /* 147 */
    CWORD as libc::c_int,                             /* 148 */
    CWORD as libc::c_int,                             /* 149 */
    CWORD as libc::c_int,                             /* 150 */
    CWORD as libc::c_int,                             /* 151 */
    CWORD as libc::c_int,                             /* 152 */
    CWORD as libc::c_int,                             /* 153 */
    CWORD as libc::c_int,                             /* 154 */
    CWORD as libc::c_int,                             /* 155 */
    CWORD as libc::c_int,                             /* 156 */
    CWORD as libc::c_int,                             /* 157 */
    CWORD as libc::c_int,                             /* 158 */
    CWORD as libc::c_int,                             /* 159 */
    CWORD as libc::c_int,                             /* 160 */
    CWORD as libc::c_int,                             /* 161 */
    CWORD as libc::c_int,                             /* 162 */
    CWORD as libc::c_int,                             /* 163 */
    CWORD as libc::c_int,                             /* 164 */
    CWORD as libc::c_int,                             /* 165 */
    CWORD as libc::c_int,                             /* 166 */
    CWORD as libc::c_int,                             /* 167 */
    CWORD as libc::c_int,                             /* 168 */
    CWORD as libc::c_int,                             /* 169 */
    CWORD as libc::c_int,                             /* 170 */
    CWORD as libc::c_int,                             /* 171 */
    CWORD as libc::c_int,                             /* 172 */
    CWORD as libc::c_int,                             /* 173 */
    CWORD as libc::c_int,                             /* 174 */
    CWORD as libc::c_int,                             /* 175 */
    CWORD as libc::c_int,                             /* 176 */
    CWORD as libc::c_int,                             /* 177 */
    CWORD as libc::c_int,                             /* 178 */
    CWORD as libc::c_int,                             /* 179 */
    CWORD as libc::c_int,                             /* 180 */
    CWORD as libc::c_int,                             /* 181 */
    CWORD as libc::c_int,                             /* 182 */
    CWORD as libc::c_int,                             /* 183 */
    CWORD as libc::c_int,                             /* 184 */
    CWORD as libc::c_int,                             /* 185 */
    CWORD as libc::c_int,                             /* 186 */
    CWORD as libc::c_int,                             /* 187 */
    CWORD as libc::c_int,                             /* 188 */
    CWORD as libc::c_int,                             /* 189 */
    CWORD as libc::c_int,                             /* 190 */
    CWORD as libc::c_int,                             /* 191 */
    CWORD as libc::c_int,                             /* 192 */
    CWORD as libc::c_int,                             /* 193 */
    CWORD as libc::c_int,                             /* 194 */
    CWORD as libc::c_int,                             /* 195 */
    CWORD as libc::c_int,                             /* 196 */
    CWORD as libc::c_int,                             /* 197 */
    CWORD as libc::c_int,                             /* 198 */
    CWORD as libc::c_int,                             /* 199 */
    CWORD as libc::c_int,                             /* 200 */
    CWORD as libc::c_int,                             /* 201 */
    CWORD as libc::c_int,                             /* 202 */
    CWORD as libc::c_int,                             /* 203 */
    CWORD as libc::c_int,                             /* 204 */
    CWORD as libc::c_int,                             /* 205 */
    CWORD as libc::c_int,                             /* 206 */
    CWORD as libc::c_int,                             /* 207 */
    CWORD as libc::c_int,                             /* 208 */
    CWORD as libc::c_int,                             /* 209 */
    CWORD as libc::c_int,                             /* 210 */
    CWORD as libc::c_int,                             /* 211 */
    CWORD as libc::c_int,                             /* 212 */
    CWORD as libc::c_int,                             /* 213 */
    CWORD as libc::c_int,                             /* 214 */
    CWORD as libc::c_int,                             /* 215 */
    CWORD as libc::c_int,                             /* 216 */
    CWORD as libc::c_int,                             /* 217 */
    CWORD as libc::c_int,                             /* 218 */
    CWORD as libc::c_int,                             /* 219 */
    CWORD as libc::c_int,                             /* 220 */
    CWORD as libc::c_int,                             /* 221 */
    CWORD as libc::c_int,                             /* 222 */
    CWORD as libc::c_int,                             /* 223 */
    CWORD as libc::c_int,                             /* 224 */
    CWORD as libc::c_int,                             /* 225 */
    CWORD as libc::c_int,                             /* 226 */
    CWORD as libc::c_int,                             /* 227 */
    CWORD as libc::c_int,                             /* 228 */
    CWORD as libc::c_int,                             /* 229 */
    CWORD as libc::c_int,                             /* 230 */
    CWORD as libc::c_int,                             /* 231 */
    CWORD as libc::c_int,                             /* 232 */
    CWORD as libc::c_int,                             /* 233 */
    CWORD as libc::c_int,                             /* 234 */
    CWORD as libc::c_int,                             /* 235 */
    CWORD as libc::c_int,                             /* 236 */
    CWORD as libc::c_int,                             /* 237 */
    CWORD as libc::c_int,                             /* 238 */
    CWORD as libc::c_int,                             /* 239 */
    CWORD as libc::c_int,                             /* 240 */
    CWORD as libc::c_int,                             /* 241 */
    CWORD as libc::c_int,                             /* 242 */
    CWORD as libc::c_int,                             /* 243 */
    CWORD as libc::c_int,                             /* 244 */
    CWORD as libc::c_int,                             /* 245 */
    CWORD as libc::c_int,                             /* 246 */
    CWORD as libc::c_int,                             /* 247 */
    CWORD as libc::c_int,                             /* 248 */
    CWORD as libc::c_int,                             /* 249 */
    CWORD as libc::c_int,                             /* 250 */
    CWORD as libc::c_int,                             /* 251 */
    CWORD as libc::c_int,                             /* 252 */
    CWORD as libc::c_int,                             /* 253 */
    CWORD as libc::c_int,                             /* 254 */
    CWORD as libc::c_int,                             /* 255 */
];
