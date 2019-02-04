use std::fs::File;
use std::io;
use std::io::{Error, ErrorKind, Read};
use std::str;

use attribute::*;
use class_file::ClassFile;
use constant_pool::*;
use field::*;
use method::*;

const EXPECTED_MAGIC: u32 = 0xCAFEBABE;

const CONSTANT_TAG_UTF8: u8 = 1;
const CONSTANT_TAG_CLASS: u8 = 7;
const CONSTANT_TAG_FIELDREF: u8 = 9;
const CONSTANT_TAG_METHODREF: u8 = 10;
const CONSTANT_TAG_NAME_AND_TYPE: u8 = 12;

pub fn read_class_file(file: &mut File) -> io::Result<ClassFile> {
    let magic = read_u32(file)?;

    if magic != EXPECTED_MAGIC {
        let error_msg = format!("The given file does not appear to be a valid JVM class file. JVM class files must start with the magic bytes \"CAFEBABE\", but this file started with \"{:x}\"", magic);

        return Err(Error::new(ErrorKind::Other, error_msg))
    }

    let minor_version = read_u16(file)?;
    let major_version = read_u16(file)?;

    let constant_pool = read_constant_pool(file)?;

    let access_flags = read_u16(file)?;
    let this_class = read_u16(file)?;
    let super_class = read_u16(file)?;

    let interfaces = read_interfaces(file)?;
    let fields = read_fields(file)?;
    let methods = read_methods(file)?;
    let attributes = read_attributes(file)?;

    Ok(ClassFile {
        minor_version,
        major_version,
        constant_pool,
        access_flags,
        this_class,
        super_class,
        interfaces,
        fields,
        methods,
        attributes,
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
    let constant_pool_count = read_u16(file)? as i32;

    let mut constant_pool = Vec::<Box<ConstantPoolEntry>>::new();

    for _ in 0..(constant_pool_count - 1) {
        let entry = read_constant_pool_entry(file)?;

        constant_pool.push(entry);
    }

    Ok(constant_pool)
}

fn read_constant_pool_entry(file: &mut File) -> io::Result<Box<ConstantPoolEntry>> {
    let tag = read_u8(file)?;

    let entry: Box<ConstantPoolEntry> = match tag {
        CONSTANT_TAG_UTF8 =>
            Box::new(read_constant_utf8(file)?),
        CONSTANT_TAG_CLASS =>
            Box::new(read_constant_class(file)?),
        CONSTANT_TAG_FIELDREF =>
            Box::new(read_constant_fieldref(file)?),
        CONSTANT_TAG_METHODREF =>
            Box::new(read_constant_methodref(file)?),
        CONSTANT_TAG_NAME_AND_TYPE =>
            Box::new(read_constant_name_and_type(file)?),
        _ => panic!("Encountered unknown type of constant pool entry with a tag of: {}", tag),
    };

    Ok(entry)
}

fn read_constant_utf8(file: &mut File) -> io::Result<ConstantPoolEntry> {
    let length = read_u16(file)?;

    let bytes = read_n_bytes(file, length as usize)?;

    let string = str::from_utf8(&bytes).unwrap()
        .to_string();

    Ok(ConstantPoolEntry::ConstantUtf8 {
        string,
    })
}

fn read_constant_class(file: &mut File) -> io::Result<ConstantPoolEntry> {
    let name_index = read_u16(file)?;

    Ok(ConstantPoolEntry::ConstantClass {
        name_index,
    })
}

fn read_constant_fieldref(file: &mut File) -> io::Result<ConstantPoolEntry> {
    let class_index = read_u16(file)?;
    let name_and_type_index = read_u16(file)?;

    Ok(ConstantPoolEntry::ConstantFieldref {
        class_index,
        name_and_type_index,
    })
}

fn read_constant_methodref(file: &mut File) -> io::Result<ConstantPoolEntry> {
    let class_index = read_u16(file)?;
    let name_and_type_index = read_u16(file)?;

    Ok(ConstantPoolEntry::ConstantMethodref {
        class_index,
        name_and_type_index,
    })
}

fn read_constant_name_and_type(file: &mut File) -> io::Result<ConstantPoolEntry> {
    let name_index = read_u16(file)?;
    let descriptor_index = read_u16(file)?;

    Ok(ConstantPoolEntry::ConstantNameAndType {
        name_index,
        descriptor_index,
    })
}

fn read_interfaces(file: &mut File) -> io::Result<Vec<u16>> {
    let interfaces_count = read_u16(file)? as i32;

    let mut interfaces = Vec::<u16>::new();

    for _ in 0..interfaces_count {
        let entry = read_u16(file)?;

        interfaces.push(entry);
    }

    Ok(interfaces)
}

fn read_fields(file: &mut File) -> io::Result<Vec<Field>> {
    let fields_count = read_u16(file)? as i32;

    let mut fields = Vec::<Field>::new();

    for _ in 0..fields_count {
        let entry = read_field(file)?;

        fields.push(entry);
    }

    Ok(fields)
}

fn read_field(file: &mut File) -> io::Result<Field> {
    let access_flags = read_u16(file)?;
    let name_index = read_u16(file)?;
    let descriptor_index = read_u16(file)?;

    let attributes = read_attributes(file)?;

    Ok(Field {
        access_flags,
        name_index,
        descriptor_index,
        attributes,
    })
}

fn read_methods(file: &mut File) -> io::Result<Vec<Method>> {
    let methods_count = read_u16(file)? as i32;

    let mut methods = Vec::<Method>::new();

    for _ in 0..methods_count {
        let entry = read_method(file)?;

        methods.push(entry);
    }

    Ok(methods)
}

fn read_method(file: &mut File) -> io::Result<Method> {
    let access_flags = read_u16(file)?;
    let name_index = read_u16(file)?;
    let descriptor_index = read_u16(file)?;

    let attributes = read_attributes(file)?;

    Ok(Method {
        access_flags,
        name_index,
        descriptor_index,
        attributes,
    })
}

fn read_attributes(file: &mut File) -> io::Result<Vec<Attribute>> {
    let attributes_count = read_u16(file)?;

    let mut attributes = Vec::<Attribute>::new();

    for _ in 0..attributes_count {
        let entry = read_attribute(file)?;

        attributes.push(entry);
    }

    Ok(attributes)
}

fn read_attribute(file: &mut File) -> io::Result<Attribute> {
    let attribute_name_index = read_u16(file)?;
    let attribute_length = read_u32(file)?;

    let info = read_n_bytes(file, attribute_length as usize)?;

    Ok(Attribute {
        attribute_name_index,
        info,
    })
}
