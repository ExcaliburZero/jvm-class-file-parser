use std::io;
use std::io::Write;

use attribute::*;
use class_access::*;
use class_file::ClassFile;
use constant_pool::*;
use method::*;

const MAGIC: u32 = 0xCAFE_BABE;

const CONSTANT_TAG_UTF8: u8 = 1;
const CONSTANT_TAG_CLASS: u8 = 7;
const CONSTANT_TAG_STRING: u8 = 8;
const CONSTANT_TAG_FIELDREF: u8 = 9;
const CONSTANT_TAG_METHODREF: u8 = 10;
const CONSTANT_TAG_NAME_AND_TYPE: u8 = 12;

pub fn write_class_file<W: Write>(file: &mut W, class_file: &ClassFile) -> io::Result<()> {
    write_u32(file, MAGIC)?;

    write_u16(file, class_file.minor_version)?;
    write_u16(file, class_file.major_version)?;

    write_constant_pool(file, &class_file.constant_pool)?;

    write_u16(file, ClassAccess::to_access_flags(&class_file.access_flags))?;
    write_u16(file, class_file.this_class)?;
    write_u16(file, class_file.super_class)?;

    write_u16(file, 0)?; // interfaces
    write_u16(file, 0)?; // fields
    write_methods(file, &class_file.methods)?;
    write_attributes(file, &class_file.attributes)?;

    Ok(())
}

fn write_u8<W: Write>(file: &mut W, value: u8) -> io::Result<()> {
    file.write_all(&u8::to_be_bytes(value))
}

fn write_u16<W: Write>(file: &mut W, value: u16) -> io::Result<()> {
    file.write_all(&u16::to_be_bytes(value))
}

fn write_u32<W: Write>(file: &mut W, value: u32) -> io::Result<()> {
    file.write_all(&u32::to_be_bytes(value))
}

fn write_n_bytes<W: Write>(file: &mut W, bytes: &[u8]) -> io::Result<()> {
    file.write_all(bytes)
}

fn write_constant_pool<W: Write>(file: &mut W, constant_pool: &[Box<ConstantPoolEntry>]) -> io::Result<()> {
    write_u16(file, (constant_pool.len() + 1) as u16)?;

    for entry in constant_pool {
        write_constant_pool_entry(file, entry)?;
    }

    Ok(())
}

#[allow(clippy::borrowed_box)]
fn write_constant_pool_entry<W: Write>(file: &mut W, entry: &Box<ConstantPoolEntry>) -> io::Result<()> {
    use ConstantPoolEntry::*;

    match **entry {
        ConstantUtf8 { ref string } => write_constant_utf8(file, &string)?,
        ConstantClass { name_index } => write_constant_class(file, name_index)?,
        ConstantString { string_index } =>
            write_constant_string(file, string_index)?,
        ConstantFieldref { class_index, name_and_type_index } =>
            write_constant_fieldref(file, class_index, name_and_type_index)?,
        ConstantMethodref { class_index, name_and_type_index } =>
            write_constant_methodref(file, class_index, name_and_type_index)?,
        ConstantNameAndType { name_index, descriptor_index } =>
            write_constant_name_and_type(file, name_index, descriptor_index)?,
        _ => panic!(),
    }

    Ok(())
}

fn write_constant_utf8<W: Write>(file: &mut W, string: &str) -> io::Result<()> {
    let bytes = string.as_bytes();

    write_u8(file, CONSTANT_TAG_UTF8)?;
    write_u16(file, bytes.len() as u16)?;
    write_n_bytes(file, &bytes)?;

    Ok(())
}

fn write_constant_class<W: Write>(file: &mut W, name_index: u16) -> io::Result<()> {
    write_u8(file, CONSTANT_TAG_CLASS)?;
    write_u16(file, name_index)?;

    Ok(())
}

fn write_constant_string<W: Write>(file: &mut W, string_index: u16) -> io::Result<()> {
    write_u8(file, CONSTANT_TAG_STRING)?;
    write_u16(file, string_index)?;

    Ok(())
}

fn write_constant_fieldref<W: Write>(file: &mut W, class_index: u16, name_and_type_index: u16) -> io::Result<()> {
    write_u8(file, CONSTANT_TAG_FIELDREF)?;
    write_u16(file, class_index)?;
    write_u16(file, name_and_type_index)?;

    Ok(())
}

fn write_constant_methodref<W: Write>(file: &mut W, class_index: u16, name_and_type_index: u16) -> io::Result<()> {
    write_u8(file, CONSTANT_TAG_METHODREF)?;
    write_u16(file, class_index)?;
    write_u16(file, name_and_type_index)?;

    Ok(())
}

fn write_constant_name_and_type<W: Write>(file: &mut W, name_index: u16, descriptor_index: u16) -> io::Result<()> {
    write_u8(file, CONSTANT_TAG_NAME_AND_TYPE)?;
    write_u16(file, name_index)?;
    write_u16(file, descriptor_index)?;

    Ok(())
}

fn write_methods<W: Write>(file: &mut W, methods: &[Method]) -> io::Result<()> {
    write_u16(file, methods.len() as u16)?;

    for method in methods.iter() {
        write_method(file, method)?;
    }

    Ok(())
}

fn write_method<W: Write>(file: &mut W, method: &Method) -> io::Result<()> {
    write_u16(file, method.access_flags)?;
    write_u16(file, method.name_index)?;
    write_u16(file, method.descriptor_index)?;

    write_attributes(file, &method.attributes)?;

    Ok(())
}

fn write_attributes<W: Write>(file: &mut W, attributes: &[Attribute]) -> io::Result<()> {
    write_u16(file, attributes.len() as u16)?;

    for attribute in attributes.iter() {
        write_attribute(file, attribute)?;
    }

    Ok(())
}

fn write_attribute<W: Write>(file: &mut W, attributes: &Attribute) -> io::Result<()> {
    write_u16(file, attributes.attribute_name_index)?;
    write_u32(file, attributes.info.len() as u32)?;
    write_n_bytes(file, &attributes.info)?;

    Ok(())
}
