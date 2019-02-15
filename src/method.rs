use std::io;
use std::ops::Deref;

use attribute::*;
use constant_pool::ConstantPoolEntry;
use class_file::ClassFile;

#[derive(Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
pub struct Method {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
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
