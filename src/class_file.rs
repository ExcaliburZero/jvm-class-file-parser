use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::ops::Deref;

use attribute::*;
use class_access::*;
use constant_pool::*;
use field::*;
use method::*;
use parsing;

/// A representation of a JVM class file.
///
/// For details on the format and structure of a JVM class file, see the
/// corresponding section of the Java Virtual Machine Specification.
///
/// https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-4.html
#[derive(Debug)]
pub struct ClassFile {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: Vec<Box<ConstantPoolEntry>>,
    pub access_flags: HashSet<ClassAccess>,
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
        use ConstantPoolEntry::*;

        let class = self.get_constant(self.this_class as usize);

        if let ConstantClass { name_index } = *class.deref() {
            let class_name = self.get_constant(name_index as usize);

            if let ConstantUtf8 { ref string } = *class_name.deref() {
                string
            } else {
                panic!("The \"name_index\" pointed to by \"this_class\" did not point to a ConstantUtf8. Found: {:?}", class_name.deref())
            }
        } else {
            panic!("The \"this_class\" did not point to a ConstantClass. Found: {:?}", class.deref())
        }
    }

    /// Returns the name of the source file that the class file was compiled
    /// from.
    ///
    /// If the class file does not have a `SourceFile` attribute, then a `None`
    /// option is returned.
    ///
    /// ```
    /// # use std::fs::File;
    /// # use jvm_class_file_parser::ClassFile;
    /// #
    /// let mut file = File::open("classes/Dummy.class").unwrap();
    /// let class_file = ClassFile::from_file(&mut file).unwrap();
    ///
    /// assert_eq!(Some("Dummy.java"), class_file.get_source_file_name());
    /// ```
    pub fn get_source_file_name(&self) -> Option<&str> {
        use ConstantPoolEntry::*;

        for ref attr in self.attributes.iter() {
            let name_constant = self.get_constant(attr.attribute_name_index as usize);

            if let ConstantUtf8 { ref string } = *name_constant.deref() {
                if string == "SourceFile" {
                    if attr.info.len() != 2 {
                        panic!("Incorrectly formatted SourceFile attribute. Expected info length of 2, found: {}", attr.info.len());
                    }

                    let info = [attr.info[0], attr.info[1]];
                    let source_file_index = u16::from_be_bytes(info);
                    let source_constant = self.get_constant(source_file_index as usize);

                    if let ConstantUtf8 { ref string } =
                        *source_constant.deref() { return Some(string) }
                    else {
                        panic!("The \"info\" of the \"SourceFile\" annotation did not point to a ConstantUtf8. Found: {:?}", source_constant.deref());
                    }
                }
            }
        }

        None
    }

    /// Returns a string representation of the specified Utf8 constant.
    ///
    /// ```
    /// # use std::fs::File;
    /// # use jvm_class_file_parser::ClassFile;
    /// #
    /// let mut file = File::open("classes/Dummy.class").unwrap();
    /// let class_file = ClassFile::from_file(&mut file).unwrap();
    ///
    /// assert_eq!("<init>", class_file.get_constant_utf8(4));
    /// ```
    pub fn get_constant_utf8(&self, index: usize) -> &str {
        use ConstantPoolEntry::*;

        let constant_utf8 = self.get_constant(index);

        if let ConstantUtf8 { ref string } = *constant_utf8.deref() {
            string
        } else {
            panic!("Failed to get constant \"#{}\" as a ConstantUtf8. Found: {:?}", index, constant_utf8)
        }
    }

    /// Returns a string representation of the specified class constant.
    ///
    /// ```
    /// # use std::fs::File;
    /// # use jvm_class_file_parser::ClassFile;
    /// #
    /// let mut file = File::open("classes/Dummy.class").unwrap();
    /// let class_file = ClassFile::from_file(&mut file).unwrap();
    ///
    /// assert_eq!("java/lang/Object", class_file.get_constant_class_str(3));
    /// ```
    pub fn get_constant_class_str(&self, index: usize) -> &str {
        use ConstantPoolEntry::*;

        let constant_class = self.get_constant(index);

        if let ConstantClass { name_index } = *constant_class.deref() {
            self.get_constant_utf8(name_index as usize)
        } else {
            panic!("Failed to get constant \"#{}\" as a ConstantClass. Found: {:?}", index, constant_class)
        }
    }

    /// Returns a string representation of the specified name and type
    /// constant.
    ///
    /// ```
    /// # use std::fs::File;
    /// # use jvm_class_file_parser::ClassFile;
    /// #
    /// let mut file = File::open("classes/Dummy.class").unwrap();
    /// let class_file = ClassFile::from_file(&mut file).unwrap();
    ///
    /// assert_eq!(
    ///     "\"<init>\":()V",
    ///     class_file.get_constant_name_and_type_str(10)
    /// );
    /// ```
    pub fn get_constant_name_and_type_str(&self, index: usize) -> String {
        use ConstantPoolEntry::*;

        let constant_nat = self.get_constant(index);

        if let ConstantNameAndType { name_index, descriptor_index } =
                *constant_nat.deref() {
            format!(
                "\"{}\":{}",
                self.get_constant_utf8(name_index as usize),
                self.get_constant_utf8(descriptor_index as usize),
            )
        } else {
            panic!("Failed to get constant \"#{}\" as a ConstantNameAndType. Found: {:?}", index, constant_nat)
        }
    }

    /// Returns the specified constant from the constant pool.
    ///
    /// This method exists in order to encapsulate the fact that the constant
    /// pool indexes start at 1 rather than 0.
    ///
    /// ```
    /// # use std::fs::File;
    /// # use std::ops::Deref;
    /// # use jvm_class_file_parser::ClassFile;
    /// # use jvm_class_file_parser::ConstantPoolEntry::*;
    /// #
    /// let mut file = File::open("classes/Dummy.class").unwrap();
    /// let class_file = ClassFile::from_file(&mut file).unwrap();
    ///
    /// assert_eq!(
    ///     ConstantClass {
    ///         name_index: 11,
    ///     },
    ///     *class_file.get_constant(2).deref()
    /// );
    /// ```
    pub fn get_constant(&self, index: usize) -> &Box<ConstantPoolEntry> {
        &self.constant_pool[index - 1]
    }
}
