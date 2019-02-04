use std::fs::File;
use std::io;
use std::ops::Deref;

use attribute::*;
use constant_pool::*;
use field::*;
use method::*;
use parsing;

#[derive(Debug)]
pub struct ClassFile {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: Vec<Box<ConstantPoolEntry>>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub attributes: Vec<Attribute>,
}

impl ClassFile {
    /// Parses the given class file. Fails if the given file is not a valid
    /// class file.
    ///
    /// ```
    /// # use std::fs::File;
    /// # use jvm_class_file_parser::ClassFile;
    /// #
    /// let mut file = File::open("classes/Dummy.class").unwrap();
    /// let class_file = ClassFile::from_file(&mut file).unwrap();
    /// ```
    pub fn from_file(file: &mut File) -> io::Result<ClassFile> {
        parsing::read_class_file(file)
    }

    /// Returns the name of the class file.
    ///
    /// ```
    /// # use std::fs::File;
    /// # use jvm_class_file_parser::ClassFile;
    /// #
    /// let mut file = File::open("classes/Dummy.class").unwrap();
    /// let class_file = ClassFile::from_file(&mut file).unwrap();
    ///
    /// assert_eq!("Dummy", class_file.get_class_name());
    /// ```
    pub fn get_class_name(&self) -> &str {
        let class = self.get_constant(self.this_class as usize);

        match class.deref() {
            &ConstantPoolEntry::ConstantClass { name_index } => {
                let class_name = self.get_constant(name_index as usize);

                match class_name.deref() {
                    &ConstantPoolEntry::ConstantUtf8 { ref string } => string,
                    _ => panic!(),
                }
            },
            _ => panic!(),
        }
    }

    fn get_constant(&self, index: usize) -> &Box<ConstantPoolEntry> {
        &self.constant_pool[index - 1]
    }
}
