// 抑制与 C FFI 兼容性相关的各种编译警告
#![allow(non_snake_case)] // 允许非蛇形命名(与C兼容)
#![allow(non_camel_case_types)] // 允许非驼峰类型名(与C兼容)
#![allow(non_upper_case_globals)] // 允许非大写全局变量(与C兼容)
#![allow(improper_ctypes)]
// 允许不当的C类型(FFI需要)
// #![allow(clashing_extern_declarations)] // 允许冲突的extern声明
// #![allow(dead_code)] // 允许未使用的代码
// #![allow(unused_variables)] // 允许未使用的变量
// #![allow(unused_assignments)] // 允许未使用的赋值
// #![allow(unused_must_use)] // 允许忽略must_use结果
// #![allow(unused_unsafe)] // 允许未使用的unsafe
// #![allow(unused_attributes)]
// 允许未使用的属性
// #![allow(path_statements)] // 允许路径语句
// #![allow(unreachable_code)] // 允许不可达代码
// #![allow(temporary_cstring_as_ptr)] // 允许临时CString指针
#![allow(static_mut_refs)] // 允许静态可变引用
#![allow(const_item_mutation)] // 允许常量项突变
#![allow(ambiguous_glob_reexports)] // 允许模糊的全局重导出
#![allow(hidden_glob_reexports)] // 允许隐藏的全局重导出
#![allow(semicolon_in_expressions_from_macros)] // 允许宏表达式中的分号
#![allow(unknown_lints)] // 允许未知的lint

// 抑制 Clippy 警告 (C FFI 兼容性需要)
#![allow(clippy::all)] // 抑制所有clippy警告
#![allow(clippy::redundant_static_lifetimes)] // 常量默认有static生命周期
#![allow(clippy::missing_safety_doc)] // FFI函数缺少安全文档
#![allow(clippy::too_many_arguments)] // C函数通常有很多参数
#![allow(clippy::ptr_arg)] // 指针参数在FFI中是必需的
#![allow(clippy::unnecessary_cast)] // FFI中的类型转换通常是必需的
#![allow(clippy::needless_return)] // 显式return有时在C风格代码中更清晰
#![allow(clippy::single_match)] // 单一匹配模式在某些情况下是合理的
#![allow(clippy::comparison_chain)] // 比较链在某些逻辑中是必需的
#![allow(clippy::collapsible_if)] // 可折叠的if在某些情况下更清晰
#![allow(clippy::needless_bool)] // 在某些情况下显式布尔值更清晰
#![allow(clippy::identity_op)] // 恒等操作在某些算法中是必需的
#![allow(clippy::assign_op_pattern)] // 赋值操作模式有时是必需的
#![allow(clippy::precedence)] // 操作符优先级在复杂表达式中可能需要明确
#![allow(clippy::bool_comparison)] // 布尔比较在某些情况下更清晰
#![allow(clippy::needless_borrow)] // 不必要的借用在FFI中有时是必需的
#![allow(clippy::clone_on_copy)] // 在某些情况下clone更明确
#![allow(clippy::let_and_return)] // let和return模式有时更清晰
#![allow(clippy::double_parens)] // 双括号在复杂表达式中有时是必需的
#![allow(clippy::useless_conversion)] // 看似无用的转换在FFI中通常是必需的
#![allow(clippy::ptr_offset_with_cast)] // 指针偏移和转换在低级代码中是常见的
#![allow(clippy::transmutes_expressible_as_ptr_casts)] // transmute在FFI中有时是必需的
#![allow(clippy::needless_range_loop)] // 范围循环在某些算法中更清晰
#![allow(clippy::manual_range_contains)] // 手动范围检查在某些情况下更明确
#![allow(clippy::cmp_null)] // 与null比较在FFI中是常见的
#![allow(clippy::zero_ptr)] // 零指针在FFI中是常见的

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

pub mod builtins;
pub use builtins::*;
