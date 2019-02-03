use std::fs::File;
use std::io;

use attribute::*;
use constant_pool::*;
use field::*;
use method::*;
use parsing;

#[derive(Debug)]
pub struct ClassFile {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: Vec<Box<ConstantPoolEntry>>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub attributes: Vec<Attribute>,
}

impl ClassFile {
    pub fn from_file(file: &mut File) -> io::Result<ClassFile> {
        parsing::read_class_file(file)
    }
}
