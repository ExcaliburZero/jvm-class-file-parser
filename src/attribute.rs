const EXCEPTION_ENTRY_LENGTH: usize = 8;

#[derive(Debug)]
pub struct Attribute {
    pub attribute_name_index: u16,
    pub info: Vec<u8>,
}

#[derive(Debug)]
pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<ExceptionTableEntry>,
}
// TODO: read attributes of code

impl Code {
    pub fn from_bytes(bytes: &[u8]) -> Code {
        let max_stack = u16::from_be_bytes([bytes[0], bytes[1]]);
        let max_locals = u16::from_be_bytes([bytes[2], bytes[3]]);

        let code_length = u32::from_be_bytes(
            [bytes[4], bytes[5], bytes[6], bytes[7]]
        ) as usize;

        let code_start = 8;
        let code_end = code_start + code_length;
        let code = bytes[code_start..code_end].to_vec();

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

        Code {
            max_stack,
            max_locals,
            code,
            exception_table,
        }
    }
}

#[derive(Debug)]
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
