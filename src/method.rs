use std::{convert::TryInto, io};

use attribute::*;
use class_file::ClassFile;
use ConstantPoolIndex;

// Method flags are from Table 4.6-A of the JVM specification
//
// https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-4.html#jvms-4.6-200-A.1
pub const METHOD_PUBLIC_FLAG: u16 = 0x0001;
pub const METHOD_PRIVATE_FLAG: u16 = 0x0002;
pub const METHOD_PROTECTED_FLAG: u16 = 0x0004;
pub const METHOD_STATIC_FLAG: u16 = 0x0008;
pub const METHOD_FINAL_FLAG: u16 = 0x0010;
pub const METHOD_SYNCHRONIZED_FLAG: u16 = 0x0020;
pub const METHOD_BRIDGE_FLAG: u16 = 0x0040;
pub const METHOD_VARARGS_FLAG: u16 = 0x0080;
pub const METHOD_NATIVE_FLAG: u16 = 0x0100;
pub const METHOD_ABSTRACT_FLAG: u16 = 0x0400;
pub const METHOD_STRICT_FLAG: u16 = 0x0800;
pub const METHOD_SYNTHETIC_FLAG: u16 = 0x1000;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Method {
    pub access_flags: u16,
    pub name_index: ConstantPoolIndex,
    pub descriptor_index: ConstantPoolIndex,
    pub attributes: Vec<Attribute>,
}

impl Method {
    /// Find an attribute with the specified name
    fn find_attribute<T: AsRef<str>>(
        &self,
        class_file: &ClassFile,
        attribute_name: T,
    ) -> Option<&Attribute> {
        // we can index this more efficiently
        self.attributes.iter().find(|attr| {
            class_file.get_constant_utf8(attr.attribute_name_index) == attribute_name.as_ref()
        })
    }

    pub fn get_code(&self, class_file: &ClassFile) -> io::Result<Option<Code>> {
        for attr in self.attributes.iter() {
            if class_file.get_constant_utf8(attr.attribute_name_index) == "Code" {
                return Ok(Some(Code::from_bytes(&attr.info)?));
            }
        }

        Ok(None)
    }

    pub fn get_signature(&self, class_file: &ClassFile) -> Option<String> {
        self.find_attribute(class_file, "Signature").map(|attr| {
            // why is this such a PITA
            let boxed_slice = attr.info.clone().into_boxed_slice();
            let boxed_array: Box<[u8; 2]> = match boxed_slice.try_into() {
                Ok(ba) => ba,
                Err(o) => panic!("Expected a Vec of length {} but it was {}", 2, o.len()),
            };
            let index = u16::from_be_bytes(*boxed_array);

            class_file.get_constant_utf8(index as usize).to_string()
        })
    }
}
