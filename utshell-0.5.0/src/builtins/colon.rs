/*
 * SPDX-FileCopyrightText: 2025 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: GPL-2.0-or-later
 */
//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

use crate::src_common::*;

#[no_mangle]
pub fn colon_builtin(_ignore: *mut WordList) -> i32 {
    0
}

#[no_mangle]
pub fn false_builtin(_ignore: *mut WordList) -> i32 {
    1
}
