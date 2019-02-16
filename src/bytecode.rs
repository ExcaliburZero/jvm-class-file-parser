const ACONST_NULL: u8 = 1;
const ICONST_0: u8 = 3;
const ICONST_1: u8 = 4;
const LDC: u8 = 18;
const ILOAD_1: u8 = 27;
const ALOAD_0: u8 = 42;
const ASTORE_1: u8 = 76;
const DUP: u8 = 89;
const IFEQ: u8 = 153;
const IFNE: u8 = 154;
const GOTO: u8 = 167;
const IRETURN: u8 = 172;
const RETURN: u8 = 177;
const GETSTATIC: u8 = 178;
const PUTSTATIC: u8 = 179;
const GETFIELD: u8 = 180;
const PUTFIELD: u8 = 181;
const INVOKEVIRTUAL: u8 = 182;
const INVOKESPECIAL: u8 = 183;
const NEW: u8 = 187;
const ATHROW: u8 = 191;
const CHECKCAST: u8 = 192;

/// A JVM bytecode instruction.
///
/// For more detailed information on the different types of bytecode
/// instructions, see the following section of the Java Virtual Machine
/// Specification.
///
/// https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-6.html
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Bytecode {
    Aconst_null,
    Iconst_0,
    Iconst_1,
    Ldc(u8),
    Iload_1,
    Aload_0,
    Astore_1,
    Dup,
    Ifeq(u16),
    Ifne(u16),
    Goto(u16),
    Ireturn,
    Return,
    Getstatic(u16),
    Putstatic(u16),
    Getfield(u16),
    Putfield(u16),
    Invokevirtual(u16),
    Invokespecial(u16),
    New(u16),
    Athrow,
    Checkcast(u16),
}

impl Bytecode {
    /// Converts the given slice of bytes into the bytecode instructions that
    /// they represent.
    ///
    /// ```
    /// # use jvm_class_file_parser::Bytecode;
    /// # use jvm_class_file_parser::Bytecode::*;
    /// #
    /// let bytes = vec![
    ///     42,
    ///     183, 0, 1,
    ///     177,
    /// ];
    ///
    /// let bytecodes = vec![
    ///     (0, Aload_0),
    ///     (1, Invokespecial(1)),
    ///     (4, Return),
    /// ];
    ///
    /// assert_eq!(bytecodes, Bytecode::from_bytes(&bytes));
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> Vec<(usize, Bytecode)> {
        use Bytecode::*;

        let mut bytecode = Vec::new();

        let mut i = 0;
        while i < bytes.len() {
            let instruction = bytes[i];

            match instruction {
                ACONST_NULL => {
                    bytecode.push((i, Aconst_null));

                    i += 1;
                },
                ICONST_0 => {
                    bytecode.push((i, Iconst_0));

                    i += 1;
                },
                ICONST_1 => {
                    bytecode.push((i, Iconst_1));

                    i += 1;
                },
                LDC => {
                    let constant_index = bytes[i + 1];

                    bytecode.push((i, Ldc(constant_index)));

                    i += 2;
                },
                ILOAD_1 => {
                    bytecode.push((i, Iload_1));

                    i += 1;
                },
                ALOAD_0 => {
                    bytecode.push((i, Aload_0));

                    i += 1;
                },
                ASTORE_1 => {
                    bytecode.push((i, Astore_1));

                    i += 1;
                },
                DUP => {
                    bytecode.push((i, Dup));

                    i += 1;
                },
                IFEQ => {
                    let jump_offset = u16::from_be_bytes(
                        [bytes[i + 1], bytes[i + 2]]
                    );

                    bytecode.push((i, Ifeq(jump_offset)));

                    i += 3;
                },
                IFNE => {
                    let jump_offset = u16::from_be_bytes(
                        [bytes[i + 1], bytes[i + 2]]
                    );

                    bytecode.push((i, Ifne(jump_offset)));

                    i += 3;
                },
                GOTO => {
                    let jump_offset = u16::from_be_bytes(
                        [bytes[i + 1], bytes[i + 2]]
                    );

                    bytecode.push((i, Goto(jump_offset)));

                    i += 3;
                },
                IRETURN => {
                    bytecode.push((i, Ireturn));

                    i += 1;
                },
                RETURN => {
                    bytecode.push((i, Return));

                    i += 1;
                },
                GETSTATIC => {
                    let field = u16::from_be_bytes(
                        [bytes[i + 1], bytes[i + 2]]
                    );

                    bytecode.push((i, Getstatic(field)));

                    i += 3;
                },
                PUTSTATIC => {
                    let field = u16::from_be_bytes(
                        [bytes[i + 1], bytes[i + 2]]
                    );

                    bytecode.push((i, Putstatic(field)));

                    i += 3;
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
                INVOKEVIRTUAL => {
                    let method = u16::from_be_bytes(
                        [bytes[i + 1], bytes[i + 2]]
                    );

                    bytecode.push((i, Invokevirtual(method)));

                    i += 3;
                },
                INVOKESPECIAL => {
                    let method = u16::from_be_bytes(
                        [bytes[i + 1], bytes[i + 2]]
                    );

                    bytecode.push((i, Invokespecial(method)));

                    i += 3;
                },
                NEW => {
                    let class = u16::from_be_bytes(
                        [bytes[i + 1], bytes[i + 2]]
                    );

                    bytecode.push((i, New(class)));

                    i += 3;
                },
                ATHROW => {
                    bytecode.push((i, Athrow));

                    i += 1;
                },
                CHECKCAST => {
                    let class = u16::from_be_bytes(
                        [bytes[i + 1], bytes[i + 2]]
                    );

                    bytecode.push((i, Checkcast(class)));

                    i += 3;
                },
                _ => panic!("Unknown bytecode: {}", instruction),
            }
        }

        bytecode
    }

    /// Converts the bytecode into a String representation.
    ///
    /// Takes in the index of the instruction so that it can be used to display
    /// bytecode instructions that contain an instruction offset.
    ///
    /// ```
    /// # use jvm_class_file_parser::Bytecode::*;
    /// #
    /// assert_eq!("aconst_null", Aconst_null.to_string(2));
    /// assert_eq!("ifeq          15", Ifeq(5).to_string(10));
    /// ```
    pub fn to_string(&self, index: u16) -> String {
        use Bytecode::*;

        match self {
            Aconst_null => "aconst_null".to_string(),
            Iconst_0 => "iconst_0".to_string(),
            Iconst_1 => "iconst_1".to_string(),
            Ldc(constant_index) => format!("{:13} #{}", "ldc", constant_index),
            Iload_1 => "iload_1".to_string(),
            Aload_0 => "aload_0".to_string(),
            Astore_1 => "astore_1".to_string(),
            Dup => "dup".to_string(),
            Ifeq(jump_offset) => format!("{:13} {}", "ifeq", jump_offset + index),
            Ifne(jump_offset) => format!("{:13} {}", "ifne", jump_offset + index),
            Goto(jump_offset) => format!("{:13} {}", "goto", jump_offset + index),
            Ireturn => "ireturn".to_string(),
            Return => "return".to_string(),
            Getstatic(field) => format!("{:13} #{}", "getstatic", field),
            Putstatic(field) => format!("{:13} #{}", "putstatic", field),
            Getfield(field) => format!("{:13} #{}", "getfield", field),
            Putfield(field) => format!("{:13} #{}", "putfield", field),
            Invokevirtual(method) => format!("{:13} #{}", "invokevirtual", method),
            Invokespecial(method) => format!("{:13} #{}", "invokespecial", method),
            New(class) => format!("{:13} #{}", "new", class),
            Athrow => "athrow".to_string(),
            Checkcast(class) => format!("{:13} #{}", "invokespecial", class),
        }
    }
}
