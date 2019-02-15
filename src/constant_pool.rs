#[derive(Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
pub enum ConstantPoolEntry {
    ConstantUtf8 {
        string: String,
    },
    ConstantClass {
        name_index: u16,
    },
    ConstantString {
        string_index: u16,
    },
    ConstantFieldref {
        class_index: u16,
        name_and_type_index: u16,
    },
    ConstantMethodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    ConstantNameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
}
