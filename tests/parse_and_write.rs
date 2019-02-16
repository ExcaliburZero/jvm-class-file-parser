extern crate jvm_class_file_parser;

use std::fs::File;
use std::io::{BufReader, BufWriter};

use jvm_class_file_parser::{
    ClassFile
};

fn parse_and_write(filepath: &str) {
    let mut file = File::open(filepath).unwrap();
    let class_file = ClassFile::from_file(&mut file).unwrap();

    let mut write_buffer = BufWriter::new(vec![]);

    class_file.to_file(&mut write_buffer).unwrap();

    let tmp_file = write_buffer.into_inner().unwrap();
    let mut read_buffer = BufReader::new(&tmp_file[..]);

    let class_file_2 = ClassFile::from_file(&mut read_buffer).unwrap();

    assert_eq!(class_file, class_file_2);
}

#[test]
fn parse_and_write_class_dummy() {
    parse_and_write("classes/Dummy.class");
}

#[test]
fn parse_and_write_class_helloworld() {
    parse_and_write("classes/HelloWorld.class");
}
