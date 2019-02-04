extern crate jvm_class_file_parser;

use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::ops::Deref;
use std::path::PathBuf;

use jvm_class_file_parser::{ClassFile, ConstantPoolEntry};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filepath = &args[1];

    let mut file = File::open(filepath).unwrap();
    let class_file = ClassFile::from_file(&mut file).unwrap();

    let absolute_filepath = to_absolute_filepath(filepath).unwrap();

    println!("Classfile {}", absolute_filepath.to_str().unwrap());

    let source_file = class_file.get_source_file_name();
    if let Some(source_file) = source_file {
        println!("  Compiled from: \"{}\"", source_file);
    }

    println!("class {}", class_file.get_class_name());

    println!("  minor version: {}", class_file.minor_version);
    println!("  major version: {}", class_file.major_version);

    print_constant_pool(&class_file);

    println!("{}\n{}", "{", "}");

    if let Some(source_file) = source_file {
        println!("SourceFile: \"{}\"", source_file);
    }
}

fn to_absolute_filepath(filepath: &str) -> io::Result<PathBuf> {
    let path = PathBuf::from(filepath);

    fs::canonicalize(path)
}

fn print_constant_pool(class_file: &ClassFile) {
    println!("Constant pool:");

    for (i, constant) in class_file.constant_pool.iter().enumerate() {
        // Account for 1 indexing
        let i = i + 1;

        println!(
            "{:>5} = {}",
            format!("#{}", i),
            format_constant_pool_entry(class_file, constant)
        );
    }
}

fn format_constant_pool_entry(
        class_file: &ClassFile, constant: &Box<ConstantPoolEntry>
    ) -> String {
    use ConstantPoolEntry::*;

    match constant.deref() {
        &ConstantUtf8 { ref string } => {
            format!(
                "{:<20}{}",
                "Utf8",
                string
            )
        },
        &ConstantClass { name_index } => {
            format!(
                "{:<20}{:<16}// {}",
                "Class",
                format!("#{}", name_index),
                class_file.get_constant_utf8(name_index as usize)
            )
        },
        &ConstantFieldref { class_index, name_and_type_index } => {
            format!(
                "{:<20}{:<16}// {}",
                "Fieldref",
                format!("#{}.#{}", class_index, name_and_type_index),
                format!(
                    "{}.{}",
                    class_file.get_constant_class_str(class_index as usize),
                    class_file.get_constant_name_and_type_str(
                        name_and_type_index as usize
                    ),
                )
            )
        },
        &ConstantMethodref { class_index, name_and_type_index } => {
            format!(
                "{:<20}{:<16}// {}",
                "Methodref",
                format!("#{}.#{}", class_index, name_and_type_index),
                format!(
                    "{}.{}",
                    class_file.get_constant_class_str(class_index as usize),
                    class_file.get_constant_name_and_type_str(
                        name_and_type_index as usize
                    ),
                )
            )
        },
        &ConstantNameAndType { name_index, descriptor_index } => {
            format!(
                "{:<20}{:<16}// {}",
                "NameAndType",
                format!("#{}:#{}", name_index, descriptor_index),
                format!(
                    "\"{}\":{}",
                    class_file.get_constant_utf8(name_index as usize),
                    class_file.get_constant_utf8(descriptor_index as usize),
                )
            )
        },
        _ => panic!(),
    }
}
