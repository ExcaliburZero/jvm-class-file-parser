#![feature(try_from)]

use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::{Error, ErrorKind, Read};
use std::str;

const EXPECTED_MAGIC: u32 = 0xCAFEBABE;

const CONSTANT_TAG_UTF8: u8 = 1;
const CONSTANT_TAG_CLASS: u8 = 7;
const CONSTANT_TAG_METHODREF: u8 = 10;
const CONSTANT_TAG_NAME_AND_TYPE: u8 = 12;

fn main() {
    let mut file = File::open("Test.class").unwrap();

    let class_file = ClassFile::from_file(&mut file);

    println!("{:#?}", class_file);
}

#[derive(Debug)]
struct ClassFile {
    minor_version: u16,
    major_version: u16,
    constant_pool: Vec<Box<ConstantPoolEntry>>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces: Vec<u16>,
}

impl ClassFile {
    pub fn from_file(file: &mut File) -> io::Result<ClassFile> {
        let magic = ClassFile::read_u32(file)?;

        if magic != EXPECTED_MAGIC {
            let error_msg = format!("The given file does not appear to be a valid JVM class file. JVM class files must start with the magic bytes \"CAFEBABE\", but this file started with \"{:x}\"", magic);

            return Err(Error::new(ErrorKind::Other, error_msg))
        }

        let minor_version = ClassFile::read_u16(file)?;
        let major_version = ClassFile::read_u16(file)?;

        let constant_pool = ClassFile::read_constant_pool(file)?;

        let access_flags = ClassFile::read_u16(file)?;
        let this_class = ClassFile::read_u16(file)?;
        let super_class = ClassFile::read_u16(file)?;

        let interfaces = ClassFile::read_interfaces(file)?;

        Ok(ClassFile {
            minor_version,
            major_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
        })
    }

    fn read_u8(file: &mut File) -> io::Result<u8> {
        let mut buffer = [0; 1];

        file.read(&mut buffer)?;

        Ok(u8::from_be_bytes(buffer))
    }

    fn read_u16(file: &mut File) -> io::Result<u16> {
        let mut buffer = [0; 2];

        file.read(&mut buffer)?;

        Ok(u16::from_be_bytes(buffer))
    }

    fn read_u32(file: &mut File) -> io::Result<u32> {
        let mut buffer = [0; 4];

        file.read(&mut buffer)?;

        Ok(u32::from_be_bytes(buffer))
    }

    fn read_n_bytes(file: &mut File, length: usize) -> io::Result<Vec<u8>> {
        let mut bytes = vec![0u8; length as usize];

        file.read_exact(&mut bytes)?;

        Ok(bytes)
    }

    fn read_constant_pool(file: &mut File) -> io::Result<Vec<Box<ConstantPoolEntry>>> {
        let constant_pool_count = ClassFile::read_u16(file)? as i32;

        let mut constant_pool = Vec::<Box<ConstantPoolEntry>>::new();

        for _ in 0..(constant_pool_count - 1) {
            let entry = ClassFile::read_constant_pool_entry(file)?;

            constant_pool.push(entry);
        }

        Ok(constant_pool)
    }

    fn read_constant_pool_entry(file: &mut File) -> io::Result<Box<ConstantPoolEntry>> {
        let tag = ClassFile::read_u8(file)?;

        let entry: Box<ConstantPoolEntry> = match tag {
            CONSTANT_TAG_UTF8 =>
                Box::new(ClassFile::read_constant_utf8(file)?),
            CONSTANT_TAG_CLASS =>
                Box::new(ClassFile::read_constant_class(file)?),
            CONSTANT_TAG_METHODREF =>
                Box::new(ClassFile::read_constant_methodref(file)?),
            CONSTANT_TAG_NAME_AND_TYPE =>
                Box::new(ClassFile::read_constant_name_and_type(file)?),
            _ => panic!(),
        };

        Ok(entry)
    }

    fn read_constant_utf8(file: &mut File) -> io::Result<ConstantUtf8> {
        let length = ClassFile::read_u16(file)?;

        let bytes = ClassFile::read_n_bytes(file, length as usize)?;

        let string = str::from_utf8(&bytes).unwrap()
            .to_string();

        Ok(ConstantUtf8 {
            string,
        })
    }

    fn read_constant_class(file: &mut File) -> io::Result<ConstantClass> {
        let name_index = ClassFile::read_u16(file)?;

        Ok(ConstantClass {
            name_index,
        })
    }

    fn read_constant_methodref(file: &mut File) -> io::Result<ConstantMethodref> {
        let class_index = ClassFile::read_u16(file)?;
        let name_and_type_index = ClassFile::read_u16(file)?;

        Ok(ConstantMethodref {
            class_index,
            name_and_type_index,
        })
    }

    fn read_constant_name_and_type(file: &mut File) -> io::Result<ConstantNameAndType> {
        let name_index = ClassFile::read_u16(file)?;
        let descriptor_index = ClassFile::read_u16(file)?;

        Ok(ConstantNameAndType {
            name_index,
            descriptor_index,
        })
    }

    fn read_interfaces(file: &mut File) -> io::Result<Vec<u16>> {
        let interfaces_count = ClassFile::read_u16(file)? as i32;

        let mut interfaces = Vec::<u16>::new();

        for _ in 0..(interfaces_count - 1) {
            let entry = ClassFile::read_u16(file)?;

            interfaces.push(entry);
        }

        Ok(interfaces)
    }
}

trait ConstantPoolEntry: Debug {}

#[derive(Debug)]
struct ConstantUtf8 {
    string: String,
}
impl ConstantPoolEntry for ConstantUtf8 {}

#[derive(Debug)]
struct ConstantClass {
    name_index: u16,
}
impl ConstantPoolEntry for ConstantClass {}

#[derive(Debug)]
struct ConstantMethodref {
    class_index: u16,
    name_and_type_index: u16,
}
impl ConstantPoolEntry for ConstantMethodref {}

#[derive(Debug)]
struct ConstantNameAndType {
    name_index: u16,
    descriptor_index: u16,
}
impl ConstantPoolEntry for ConstantNameAndType {}
