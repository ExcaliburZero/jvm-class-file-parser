use util::FloatBuffer;

/// Index into the constant pool "table"
pub type ConstantPoolIndex = usize;

/// Constant pool structures,
/// as defined in https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-4.html#jvms-4.4
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
    ConstantMethodHandle {
        reference_kind: u8,
        reference_index: u16,
    },
    ConstantMethodType {
        descriptor_index: u16,
    },
    ConstantDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    ConstantInvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    ConstantModule {
        name_index: u16,
    },
    ConstantPackage {
        name_index: u16,
    },
    // represents an empty slot in the constant pool table
    ConstantEmptySlot { },
}
