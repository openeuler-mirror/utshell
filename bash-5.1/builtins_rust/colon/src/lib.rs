//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

use rcommon::WordList;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[no_mangle]
pub extern "C" fn r_colon_builtin(_ignore: *mut WordList) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn r_false_builtin(_ignore: *mut WordList) -> i32 {
    1
}
