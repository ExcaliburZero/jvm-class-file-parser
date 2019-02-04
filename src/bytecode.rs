const ILOAD_1: u8 = 27;
const ALOAD_0: u8 = 42;
const IRETURN: u8 = 172;
const RETURN: u8 = 177;
const GETFIELD: u8 = 180;
const PUTFIELD: u8 = 181;
const INVOKESPECIAL: u8 = 183;

#[derive(Debug)]
pub enum Bytecode {
    Iload_1,
    Aload_0,
    Ireturn,
    Return,
    Getfield(u16),
    Putfield(u16),
    Invokespecial(u16),
}

impl Bytecode {
    pub fn from_bytes(bytes: &[u8]) -> Vec<(usize, Bytecode)> {
        use Bytecode::*;

        let mut bytecode = Vec::new();

        let mut i = 0;
        while i < bytes.len() {
            let instruction = bytes[i];

            match instruction {
                ILOAD_1 => {
                    bytecode.push((i, Iload_1));

                    i += 1;
                },
                ALOAD_0 => {
                    bytecode.push((i, Aload_0));

                    i += 1;
                },
                IRETURN => {
                    bytecode.push((i, Ireturn));

                    i += 1;
                },
                RETURN => {
                    bytecode.push((i, Return));

                    i += 1;
                },
                GETFIELD => {
                    let field = u16::from_be_bytes(
                        [bytes[i + 1], bytes[i + 2]]
                    );

                    bytecode.push((i, Getfield(field)));

                    i += 3;
                },
                PUTFIELD => {
                    let field = u16::from_be_bytes(
                        [bytes[i + 1], bytes[i + 2]]
                    );

                    bytecode.push((i, Putfield(field)));

                    i += 3;
                },
                INVOKESPECIAL => {
                    let method = u16::from_be_bytes(
                        [bytes[i + 1], bytes[i + 2]]
                    );

                    bytecode.push((i, Invokespecial(method)));

                    i += 3;
                },
                _ => panic!("Unknown bytecode: {}", instruction),
            }
        }

        bytecode
    }
}

impl ToString for Bytecode {
    fn to_string(&self) -> String {
        use Bytecode::*;

        match self {
            Iload_1 => "iload_1".to_string(),
            Aload_0 => "aload_0".to_string(),
            Ireturn => "ireturn".to_string(),
            Return => "return".to_string(),
            Getfield(field) => format!("{:13} #{}", "getfield", field),
            Putfield(field) => format!("{:13} #{}", "putfield", field),
            Invokespecial(method) => format!("{:13} #{}", "invokespecial", method),
            _ => panic!(),
        }
    }
}
