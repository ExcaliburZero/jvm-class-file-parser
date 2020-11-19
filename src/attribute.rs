use std::{convert::TryInto, io};

use bytecode::*;

use crate::ClassFile;
use {parsing, ConstantPoolIndex};

const EXCEPTION_ENTRY_LENGTH: usize = 8;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Attribute {
    pub attribute_name_index: ConstantPoolIndex,
    pub info: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AttributeSet {
    pub attributes: Vec<Attribute>,
}

impl AttributeSet {
    /// Find an attribute with the specified name
    pub fn find_attribute<T: AsRef<str>>(
        &self,
        class_file: &ClassFile,
        attribute_name: T,
    ) -> Option<&Attribute> {
        // we can index this more efficiently
        self.attributes.iter().find(|attr| {
            class_file.get_constant_utf8(attr.attribute_name_index) == attribute_name.as_ref()
        })
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

#[derive(Debug, PartialEq)]
pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<(usize, Bytecode)>,
    pub exception_table: Vec<ExceptionTableEntry>,
    pub attributes: AttributeSet,
}

impl Code {
    pub fn from_bytes(bytes: &[u8]) -> io::Result<Code> {
        let max_stack = u16::from_be_bytes([bytes[0], bytes[1]]);
        let max_locals = u16::from_be_bytes([bytes[2], bytes[3]]);

        let code_length = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]) as usize;

        let code_start = 8;
        let code_end = code_start + code_length;
        let code_bytes = &bytes[code_start..code_end];

        let code = Bytecode::from_bytes(code_bytes);

        let exception_table_length =
            u16::from_be_bytes([bytes[code_end], bytes[code_end + 1]]) as usize;

        let mut exception_table = Vec::with_capacity(exception_table_length);
        for i in 0..exception_table_length {
            let entry_start = (code_end + 2) + i * EXCEPTION_ENTRY_LENGTH;
            let entry_end = entry_start + EXCEPTION_ENTRY_LENGTH;

            let entry = ExceptionTableEntry::from_bytes(&bytes[entry_start..entry_end]);

            exception_table.push(entry);
        }

        let attributes_start = (code_end + 2) + exception_table_length * EXCEPTION_ENTRY_LENGTH;

        let mut attribute_bytes = &bytes[attributes_start..];
        let attributes = parsing::read_attributes(&mut attribute_bytes)?;

        Ok(Code {
            max_stack,
            max_locals,
            code,
            exception_table,
            attributes,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct ExceptionTableEntry {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

impl ExceptionTableEntry {
    pub fn from_bytes(bytes: &[u8]) -> ExceptionTableEntry {
        let start_pc = u16::from_be_bytes([bytes[0], bytes[1]]);
        let end_pc = u16::from_be_bytes([bytes[2], bytes[3]]);
        let handler_pc = u16::from_be_bytes([bytes[4], bytes[5]]);
        let catch_type = u16::from_be_bytes([bytes[6], bytes[7]]);

        ExceptionTableEntry {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        }
    }
}
