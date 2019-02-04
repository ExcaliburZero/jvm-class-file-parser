const ALOAD_0: u8 = 42;
const RETURN: u8 = 177;
const INVOKESPECIAL: u8 = 183;

#[derive(Debug)]
pub enum Bytecode {
    Aload_0,
    Return,
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
                ALOAD_0 => {
                    bytecode.push((i, Aload_0));

                    i += 1;
                },
                RETURN => {
                    bytecode.push((i, Return));

                    i += 1;
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
            Aload_0 => "aload_0".to_string(),
            Return => "return".to_string(),
            Invokespecial(method) => format!("invokespecial #{}", method),
            _ => panic!(),
        }
    }
}
