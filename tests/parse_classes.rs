extern crate jvm_class_file_parser;

use std::fs::File;
use jvm_class_file_parser::ClassFile;

#[test]
fn parse_class_dummy() {
    let mut file = File::open("classes/Dummy.class").unwrap();
    let class_file = ClassFile::from_file(&mut file).unwrap();
}

#[test]
fn parse_class_intbox() {
    let mut file = File::open("classes/IntBox.class").unwrap();
    let class_file = ClassFile::from_file(&mut file).unwrap();
}
