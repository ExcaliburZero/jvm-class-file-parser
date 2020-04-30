extern crate jvm_class_file_parser;

use std::collections::HashSet;
use std::fs::File;

use jvm_class_file_parser::{Attribute, Bytecode, ClassAccess, ClassFile, Code, Field, FieldAccess, ConstantPoolEntry};
use std::ops::Deref;

#[test]
fn parse_class_dummy() {
    let mut file = File::open("classes/Dummy.class").unwrap();
    let class_file = ClassFile::from_file(&mut file).unwrap();

    assert_eq!("Dummy", class_file.get_class_name());

    assert_eq!(Some("Dummy.java"), class_file.get_source_file_name());

    assert_eq!(2, class_file.access_flags.len());
    assert!(class_file.access_flags.contains(&ClassAccess::Public));
    assert!(class_file.access_flags.contains(&ClassAccess::Super));

    assert_eq!(0, class_file.fields.len());

    assert_eq!(1, class_file.methods.len());

    let constructor = &class_file.methods[0];
    let code = constructor.get_code(&class_file);

    use Bytecode::*;
    assert_eq!(
        Some(Code {
            max_stack: 1,
            max_locals: 1,
            code: vec![
                (0, Aload_0),
                (1, Invokespecial(1)),
                (4, Return),
            ],
            exception_table: vec![],
            attributes: vec![
                Attribute {
                    attribute_name_index: 7,
                    info: vec![0, 1, 0, 0, 0, 1],
                }
            ],
        }),
        code.unwrap()
    );
}

#[test]
fn parse_class_intbox() {
    let mut file = File::open("classes/IntBox.class").unwrap();
    let class_file = ClassFile::from_file(&mut file).unwrap();

    assert_eq!("IntBox", class_file.get_class_name());

    assert_eq!(Some("IntBox.java"), class_file.get_source_file_name());

    assert_eq!(2, class_file.access_flags.len());
    assert!(class_file.access_flags.contains(&ClassAccess::Public));
    assert!(class_file.access_flags.contains(&ClassAccess::Super));

    assert_eq!(1, class_file.fields.len());

    let field = &class_file.fields[0];

    let mut field_access_flags = HashSet::new();
    field_access_flags.insert(FieldAccess::Private);

    assert_eq!(
        Field {
            access_flags: field_access_flags,
            name_index: 5,
            descriptor_index: 6,
            attributes: vec![],
        },
        *field
    );

    assert_eq!(2, class_file.methods.len());

    let constructor = &class_file.methods[0];
    let constructor_code = constructor.get_code(&class_file);

    use Bytecode::*;
    assert_eq!(
        Some(Code {
            max_stack: 2,
            max_locals: 2,
            code: vec![
                (0, Aload_0),
                (1, Invokespecial(1)),
                (4, Aload_0),
                (5, Iload_1),
                (6, Putfield(2)),
                (9, Return),
            ],
            exception_table: vec![],
            attributes: vec![
                Attribute {
                    attribute_name_index: 10,
                    info: vec![0, 3, 0, 0, 0, 4, 0, 4, 0, 5, 0, 9, 0, 6],
                }
            ],
        }),
        constructor_code.unwrap()
    );

    let get_value = &class_file.methods[1];
    let get_value_code = get_value.get_code(&class_file);

    assert_eq!(
        Some(Code {
            max_stack: 1,
            max_locals: 1,
            code: vec![
                (0, Aload_0),
                (1, Getfield(2)),
                (4, Ireturn),
            ],
            exception_table: vec![],
            attributes: vec![
                Attribute {
                    attribute_name_index: 10,
                    info: vec![0, 1, 0, 0, 0, 9],
                }
            ],
        }),
       get_value_code.unwrap()
    );
}

#[test]
fn parse_class_constant_values() {
    let mut file = File::open("classes/ConstantValues.class").unwrap();
    let class_file = ClassFile::from_file(&mut file).unwrap();

    match class_file.get_constant(2).deref() {
        ConstantPoolEntry::ConstantInteger { val } => {
            assert_eq!(65535, *val)
        },
        _ => {
            panic!("Expected an integer")
        }
    }

    match class_file.get_constant(4).deref() {
        ConstantPoolEntry::ConstantFloat { ref val } => {
            assert_eq!(42.0 as f32, val.into())
        },
        _ => {
            panic!("Expected a float")
        }
    }

    match class_file.get_constant(6).deref() {
        ConstantPoolEntry::ConstantLong { val } => {
            assert_eq!(42, *val)
        },
        _ => {
            panic!("Expected a long")
        }
    }

    match class_file.get_constant(9).deref() {
        ConstantPoolEntry::ConstantDouble { ref val } => {
            assert_eq!(-1 as f64, val.into())
        },
        _ => {
            panic!("Expected a double")
        }
    }

    match class_file.get_constant(37).deref() {
        ConstantPoolEntry::ConstantUtf8 { ref string } => {
            assert_eq!("fourty two".to_string(), *string)
        },
        _ => {
            panic!("Expected a utf8 string")
        }
    }
}