use std::io;
use std::ops::Deref;

use attribute::*;
use class_file::ClassFile;
use ConstantPoolIndex;

// Method flags are from Table 4.6-A of the JVM specification
//
// https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-4.html#jvms-4.6-200-A.1
pub const METHOD_PUBLIC_FLAG: u16 = 0x0001;
pub const METHOD_PRIVATE_FLAG: u16 = 0x0002;
pub const METHOD_PROTECTED_FLAG : u16 = 0x0004;
pub const METHOD_STATIC_FLAG: u16 = 0x0008;
pub const METHOD_FINAL_FLAG : u16 = 0x0010;
pub const METHOD_SYNCHRONIZED_FLAG : u16 = 0x0020;
pub const METHOD_BRIDGE_FLAG: u16 = 0x0040;
pub const METHOD_VARARGS_FLAG : u16 = 0x0080;
pub const METHOD_NATIVE_FLAG : u16 = 0x0100;
pub const METHOD_ABSTRACT_FLAG: u16 = 0x0400;
pub const METHOD_STRICT_FLAG: u16 = 0x0800;
pub const METHOD_SYNTHETIC_FLAG: u16 = 0x1000;

#[derive(Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
pub struct Method {
    pub access_flags: u16,
    pub name_index: ConstantPoolIndex,
    pub descriptor_index: ConstantPoolIndex,
    pub attributes: Vec<Attribute>,
}

impl Method {
    pub fn get_code(&self, class_file: &ClassFile) -> io::Result<Option<Code>> {
        use ConstantPoolEntry::*;

        for attr in self.attributes.iter() {
            let name_constant = class_file.get_constant(
                attr.attribute_name_index as usize
            );

            if let ConstantUtf8 { ref string } = *name_constant.deref() {
                if string == "Code" {
                    return Ok(Some(Code::from_bytes(&attr.info)?))
                }
            }
        }

        Ok(None)
    }
}
