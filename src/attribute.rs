use std::io;
use std::io::Write;

use bytecode::*;
use parsing;

const EXCEPTION_ENTRY_LENGTH: usize = 8;

#[derive(Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
pub struct Attribute {
    pub attribute_name_index: u16,
    pub info: Vec<u8>,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<(usize, Bytecode)>,
    pub exception_table: Vec<ExceptionTableEntry>,
    pub attributes: Vec<Attribute>,
}

impl Code {
    pub fn from_bytes(bytes: &[u8]) -> io::Result<Code> {
        let max_stack = u16::from_be_bytes([bytes[0], bytes[1]]);
        let max_locals = u16::from_be_bytes([bytes[2], bytes[3]]);

        let code_length = u32::from_be_bytes(
            [bytes[4], bytes[5], bytes[6], bytes[7]]
        ) as usize;

        let code_start = 8;
        let code_end = code_start + code_length;
        let code_bytes = &bytes[code_start..code_end];

        let code = Bytecode::from_bytes(code_bytes);

        let exception_table_length = u16::from_be_bytes(
            [bytes[code_end], bytes[code_end + 1]]
        ) as usize;

        let mut exception_table = Vec::with_capacity(exception_table_length);
        for i in 0..exception_table_length {
            let entry_start = (code_end + 2) + i * EXCEPTION_ENTRY_LENGTH;
            let entry_end = entry_start + EXCEPTION_ENTRY_LENGTH;

            let entry = ExceptionTableEntry::from_bytes(
                &bytes[entry_start..entry_end]
            );

            exception_table.push(entry);
        }

        let attributes_start = (code_end + 2) +
            exception_table_length * EXCEPTION_ENTRY_LENGTH;

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

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = vec![];

        let max_stack = u16::to_be_bytes(self.max_stack);
        let max_locals = u16::to_be_bytes(self.max_locals);

        buffer.write_all(&max_stack).unwrap();
        buffer.write_all(&max_locals).unwrap();

        let mut code: Vec<u8> = self.code.iter()
            .map(|(_, bytecode)| bytecode)
            .flat_map(Bytecode::to_bytes)
            .collect();

        let code_length = u32::to_be_bytes(code.len() as u32);

        buffer.write_all(&code_length).unwrap();
        buffer.write_all(&mut code).unwrap();

        // TODO: implement writing of execption table and attributes
        assert!(self.exception_table.len() == 0);
        assert!(self.attributes.len() == 0);

        let exception_table_length = u16::to_be_bytes(0);
        let attributes_length = u16::to_be_bytes(0);

        buffer.write_all(&exception_table_length).unwrap();
        buffer.write_all(&attributes_length).unwrap();

        buffer
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
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
