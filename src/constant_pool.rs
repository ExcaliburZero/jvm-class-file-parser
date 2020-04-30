use util::FloatBuffer;

/// Index into the constant pool "table"
pub type ConstantPoolIndex = usize;

#[derive(Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
pub enum ConstantPoolEntry {
    ConstantUtf8 {
        string: String,
    },
    ConstantInteger {
        val: i32,
    },
    ConstantFloat {
        val: FloatBuffer<[u8; 4]>,
    },
    ConstantLong {
        val: i64,
    },
    ConstantDouble {
        val: FloatBuffer<[u8; 8]>,
    },
    ConstantClass {
        name_index: ConstantPoolIndex,
    },
    ConstantString {
        string_index: ConstantPoolIndex,
    },
    ConstantFieldref {
        class_index: ConstantPoolIndex,
        name_and_type_index: ConstantPoolIndex,
    },
    ConstantMethodref {
        class_index: ConstantPoolIndex,
        name_and_type_index: ConstantPoolIndex,
    },
    ConstantInterfaceMethodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    ConstantNameAndType {
        name_index: ConstantPoolIndex,
        descriptor_index: ConstantPoolIndex,
    },
    // represents an empty slot in the constant pool table
    ConstantEmptySlot { },
}
