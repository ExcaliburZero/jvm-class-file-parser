#![feature(try_from)]
//! This is a (partially implemented) Rust library and program for parsing JVM class files.
//!
//! ```
//! extern crate jvm_class_file_parser;
//! 
//! use std::fs::File;
//! use jvm_class_file_parser::ClassFile;
//! 
//! let mut file = File::open("classes/Dummy.class").unwrap();
//! let class_file = ClassFile::from_file(&mut file).unwrap();
//! 
//! assert_eq!("Dummy", class_file.get_class_name());
//! ```

mod attribute;
mod bytecode;
mod class_file;
mod class_access;
mod constant_pool;
mod field;
mod method;
mod parsing;
mod util;

pub use attribute::*;
pub use bytecode::*;
pub use class_file::*;
pub use class_access::*;
pub use constant_pool::*;
pub use field::*;
pub use method::*;
