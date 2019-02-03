use std::fmt::Debug;

pub trait ConstantPoolEntry: Debug {}

#[derive(Debug)]
pub struct ConstantUtf8 {
    pub string: String,
}
impl ConstantPoolEntry for ConstantUtf8 {}

#[derive(Debug)]
pub struct ConstantClass {
    pub name_index: u16,
}
impl ConstantPoolEntry for ConstantClass {}

#[derive(Debug)]
pub struct ConstantMethodref {
    pub class_index: u16,
    pub name_and_type_index: u16,
}
impl ConstantPoolEntry for ConstantMethodref {}

#[derive(Debug)]
pub struct ConstantNameAndType {
    pub name_index: u16,
    pub descriptor_index: u16,
}
impl ConstantPoolEntry for ConstantNameAndType {}
