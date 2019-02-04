extern crate jvm_class_file_parser;

use std::fs::File;

use jvm_class_file_parser::ClassFile;

fn main() {
    let mut file = File::open("Test.class").unwrap();

    let class_file = ClassFile::from_file(&mut file).unwrap();

    println!("{:#?}", class_file);

    println!("{}", class_file.get_class_name());
}
