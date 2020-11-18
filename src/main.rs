extern crate jvm_class_file_parser;

use std::collections::HashSet;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::ops::Deref;
use std::path::PathBuf;

use jvm_class_file_parser::{
    Attribute, Bytecode, ClassAccess, ClassFile, ConstantPoolEntry, ExceptionTableEntry, Method,
};

const CONSTRUCTOR_NAME: &str = "<init>";

fn main() {
    let args: Vec<String> = env::args().collect();

    let filepath = &args[1];

    javap(filepath, false);
}

fn javap(filepath: &str, print_code: bool) {
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

    print_access_flags(&class_file.access_flags);

    print_constant_pool(&class_file);

    print_attributes(&class_file);

    println!("{{");

    for method in class_file.methods.iter() {
        print_method(&class_file, method, print_code);
    }

    println!("}}");

    if let Some(source_file) = source_file {
        println!("SourceFile: \"{}\"", source_file);
    }

    //println!("{:#?}", class_file);
}

fn to_absolute_filepath(filepath: &str) -> io::Result<PathBuf> {
    let path = PathBuf::from(filepath);

    fs::canonicalize(path)
}

fn print_access_flags(access_flags: &HashSet<ClassAccess>) {
    let mut access_flags = access_flags.iter().cloned().collect::<Vec<ClassAccess>>();
    access_flags.sort();

    let flags_str = access_flags
        .iter()
        .map(access_flag_to_name)
        .collect::<Vec<&str>>()
        .join(", ");

    println!("  flags: {}", flags_str);
}

fn print_attributes(class_file: &ClassFile) {
    println!("Attributes:");

    class_file.attributes.iter().for_each(|attr| {
        println!("  {}", format_attribute(class_file, attr));
    });
}

fn format_attribute(class_file: &ClassFile, attr: &Attribute) -> String {
    let attr_type = class_file.get_constant_utf8(attr.attribute_name_index);
    // https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7
    match attr_type {
        // "ConstantValue" => {},
        // "Code" => {},
        // "StackMapTable" => {},
        // "Exceptions" => {},
        // "InnerClasses" => {},
        // "EnclosingMethod" => {},
        // "Synthetic" => {},
        // "Signature" => {},
        "SourceFile" | "Signature" => {
            // clean this up with u16::from() on a vec slice
            let index = ((attr.info[0] as usize) << 8) + attr.info[1] as usize;
            format!("{} = {:?}", attr_type, class_file.get_constant_utf8(index))
        }
        // "SourceDebugExtension" => {},
        // "LineNumberTable" => {},
        // "LocalVariableTable" => {},
        // "LocalVariableTypeTable" => {},
        // "Deprecated" => {},
        // "RuntimeVisibleAnnotations" => {},
        // "RuntimeInvisibleAnnotations" => {},
        // "RuntimeVisibleParameterAnnotations" => {},
        // "RuntimeInvisibleParameterAnnotations" => {},
        // "AnnotationDefault" => {},
        // "BootstrapMethods" => {},
        _ => attr_type.to_string(),
    }
}

fn access_flag_to_name(flag: &ClassAccess) -> &'static str {
    use ClassAccess::*;

    match flag {
        Public => "ACC_PUBLIC",
        Final => "ACC_FINAL",
        Super => "ACC_SUPER",
        Interface => "ACC_INTERFACE",
        Abstract => "ACC_ABSTRACT",
        Synthetic => "ACC_SYNTHETIC",
        Annotation => "ACC_ANNOTATION",
        Enum => "ACC_ENUM",
        Module => "ACC_MODULE",
    }
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

fn format_constant_pool_entry(class_file: &ClassFile, constant: &ConstantPoolEntry) -> String {
    use ConstantPoolEntry::*;

    match *constant.deref() {
        ConstantUtf8 { ref string } => format!("{:<20}{}", "Utf8", string),
        ConstantClass { name_index } => format!(
            "{:<20}{:<16}// {}",
            "Class",
            format!("#{}", name_index),
            class_file.get_constant_utf8(name_index as usize)
        ),
        ConstantString { string_index } => format!(
            "{:<20}{:<16}// {}",
            "String",
            format!("#{}", string_index),
            class_file.get_constant_utf8(string_index as usize)
        ),
        ConstantInteger { ref val } => format!("{:<20}{:<16}", "Integer", format!("={}", val)),
        ConstantFloat { ref val } => {
            let as_f32: f32 = val.into();
            format!("{:<20}{:<16}", "Float", format!("={}", as_f32))
        }
        ConstantLong { val } => format!("{:<20}{:<16}", "Long", format!("={}", val)),
        ConstantDouble { ref val } => {
            let as_f64: f64 = val.into();
            format!("{:<20}{:<16}", "Double", format!("={}", as_f64))
        }
        ConstantFieldref {
            class_index,
            name_and_type_index,
        } => format!(
            "{:<20}{:<16}// {}",
            "Fieldref",
            format!("#{}.#{}", class_index, name_and_type_index),
            format!(
                "{}.{}",
                class_file.get_constant_class_str(class_index as usize),
                class_file.get_constant_name_and_type_str(name_and_type_index as usize),
            )
        ),
        ConstantMethodref {
            class_index,
            name_and_type_index,
        } => format!(
            "{:<20}{:<16}// {}",
            "Methodref",
            format!("#{}.#{}", class_index, name_and_type_index),
            format!(
                "{}.{}",
                class_file.get_constant_class_str(class_index as usize),
                class_file.get_constant_name_and_type_str(name_and_type_index as usize),
            )
        ),
        ConstantInterfaceMethodref {
            class_index,
            name_and_type_index,
        } => format!(
            "{:<20}{:<16}// {}",
            "InterfaceMethodref",
            format!("#{}.#{}", class_index, name_and_type_index),
            format!(
                "{}.{}",
                class_file.get_constant_class_str(class_index as usize),
                class_file.get_constant_name_and_type_str(name_and_type_index as usize),
            )
        ),
        ConstantNameAndType {
            name_index,
            descriptor_index,
        } => format!(
            "{:<20}{:<16}// {}",
            "NameAndType",
            format!("#{}:#{}", name_index, descriptor_index),
            format!(
                "\"{}\":{}",
                class_file.get_constant_utf8(name_index as usize),
                class_file.get_constant_utf8(descriptor_index as usize),
            )
        ),
        ConstantMethodHandle {
            reference_kind,
            reference_index,
        } => format!(
            "{:<20}{:<16}",
            "MethodHandle",
            format!("#{}:#{}", reference_kind, reference_index)
        ),
        ConstantMethodType { descriptor_index } => format!(
            "{:<20}{:<16}// {}",
            "MethodType",
            descriptor_index,
            class_file.get_constant_utf8(descriptor_index as usize)
        ),
        ConstantDynamic {
            bootstrap_method_attr_index,
            name_and_type_index,
        } => {
            // TODO : !!!!!
            format!("")
        }
        ConstantInvokeDynamic {
            bootstrap_method_attr_index,
            name_and_type_index,
        } => {
            // TODO : !!!!!
            format!("")
        }
        ConstantModule { name_index } => {
            // TODO : !!!!!
            format!("")
        }
        ConstantPackage { name_index } => {
            // TODO : !!!!!
            format!("")
        }
        ConstantEmptySlot {} => "<empty slot>".to_string(),
    }
}

fn print_method(class_file: &ClassFile, method: &Method, print_code: bool) {
    let method_name = class_file.get_constant_utf8(method.name_index as usize);

    println!(
        "  {}();",
        if method_name == CONSTRUCTOR_NAME {
            class_file.get_class_name()
        } else {
            method_name
        }
    );

    println!(
        "    descriptor: {}",
        class_file.get_constant_utf8(method.descriptor_index as usize)
    );

    println!("    flags: TODO",);

    if print_code {
        let code_opt = method.get_code(class_file).unwrap();

        match code_opt {
            Some(code) => {
                println!("    Code:");
                println!(
                    "      stack={}, locals={}, args_size={}",
                    code.max_stack, code.max_locals, "TODO"
                );

                print_bytecode(class_file, &code.code);

                if !code.exception_table.is_empty() {
                    print_exception_table(class_file, &code.exception_table);
                }
            }
            _ => {}
        }
    }
}

fn print_bytecode(_class_file: &ClassFile, code: &[(usize, Bytecode)]) {
    for (i, bytecode) in code {
        print!("        {:>3}: {:35}", i, bytecode.to_string(*i as u16));

        // TODO: show constants to the side

        println!();
    }
}

fn print_exception_table(class_file: &ClassFile, exception_table: &[ExceptionTableEntry]) {
    println!("      Exception table:");
    println!("         from    to  target type");

    for entry in exception_table.iter() {
        println!(
            "         {:5} {:5} {:5}   Class {}",
            entry.start_pc,
            entry.end_pc,
            entry.handler_pc,
            class_file.get_constant_class_str(entry.catch_type as usize),
        );
    }
}

#[cfg(test)]
mod tests {
    use javap;

    #[test]
    fn javap_dummy_runs_without_error() {
        javap("classes/Dummy.class", true);
    }

    #[test]
    fn javap_intbox_runs_without_error() {
        javap("classes/IntBox.class", true);
    }

    #[test]
    fn javap_exceptionthrows_runs_without_error() {
        javap("classes/ExceptionThrows.class", true);
    }

    #[test]
    fn javap_helloworld_runs_without_error() {
        javap("classes/HelloWorld.class", true);
    }

    #[test]
    fn javap_interface_runs_without_error() {
        javap("classes/Interface.class", true);
    }
}
