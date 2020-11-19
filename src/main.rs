extern crate jvm_class_file_parser;

use std::collections::HashSet;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::ops::Deref;
use std::path::PathBuf;

use jvm_class_file_parser::{Attribute, AttributeSet, Bytecode, ClassAccess, ClassFile, ConstantPoolEntry, ExceptionTableEntry, Method};

const CONSTRUCTOR_NAME: &str = "<init>";

fn main() {
    let args: Vec<String> = env::args().collect();

    let filepath = &args[1];

    let javap_result = javap(filepath, false);
    println!("{}", javap_result);
}

fn javap(filepath: &str, print_code: bool) -> String {
    let mut file = File::open(filepath).unwrap();
    let class_file = ClassFile::from_file(&mut file).unwrap();

    let absolute_filepath = to_absolute_filepath(filepath).unwrap();

    let mut output = String::new();

    output = output + format!("Classfile {}\n", absolute_filepath.to_str().unwrap()).as_ref();

    let source_file = class_file.get_source_file_name();
    if let Some(source_file) = source_file {
        output = output + format!("  Compiled from: \"{}\"\n", source_file).as_ref();
    }

    output = output + format!("class {}\n", class_file.get_class_name()).as_ref();

    output = output + format!("  minor version: {}\n", class_file.minor_version).as_ref();
    output = output + format!("  major version: {}\n", class_file.major_version).as_ref();

    output = output + print_access_flags(&class_file.access_flags).as_ref();

    output = output + print_constant_pool(&class_file).as_ref();

    output = output + print_attributes(&class_file, &class_file.attributes, "").as_ref();

    output = output + format!("{{\n").as_ref();

    for method in class_file.methods.iter() {
        output = output + print_method(&class_file, method, print_code).as_ref();
    }

    output = output + format!("}}\n").as_ref();

    if let Some(source_file) = source_file {
        output = output + format!("SourceFile: \"{}\"\n", source_file).as_ref();
    }

    //println!("{:#?}", class_file);
    output
}

fn to_absolute_filepath(filepath: &str) -> io::Result<PathBuf> {
    let path = PathBuf::from(filepath);

    fs::canonicalize(path)
}

fn print_access_flags(access_flags: &HashSet<ClassAccess>) -> String {
    let mut access_flags = access_flags.iter().cloned().collect::<Vec<ClassAccess>>();
    access_flags.sort();

    let flags_str = access_flags
        .iter()
        .map(access_flag_to_name)
        .collect::<Vec<&str>>()
        .join(", ");

    format!("  flags: {}\n", flags_str)
}

fn print_attributes(
    class_file: &ClassFile,
    attributes: &AttributeSet,
    prefix: &'static str,
) -> String {
    let mut output = format!("{}Attributes:\n", prefix);

    attributes.attributes.iter().for_each(|attr| {
        output.push_str(format!("{}  {}\n", prefix, format_attribute(class_file, attr)).as_ref());
    });

    output
}

/// Format an attribute (into a single-line value to preserve outer formatting)
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
        _ => format!("{} = <TODO>", attr_type.to_string()),
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

fn print_constant_pool(class_file: &ClassFile) -> String {
    let mut output = "Constant pool:\n".to_string();

    for (i, constant) in class_file.constant_pool.iter().enumerate() {
        // Account for 1 indexing
        let i = i + 1;

        output = output
            + format!(
                "{:>5} = {}\n",
                format!("#{}", i),
                format_constant_pool_entry(class_file, constant)
            )
            .as_ref();
    }

    output
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

fn print_method(class_file: &ClassFile, method: &Method, print_code: bool) -> String {
    let method_name = class_file.get_constant_utf8(method.name_index as usize);

    const PREFIX: &'static str = "    ";

    let mut output = String::new();

    output = output
        + format!(
            "  {}();\n",
            if method_name == CONSTRUCTOR_NAME {
                class_file.get_class_name()
            } else {
                method_name
            }
        )
        .as_ref();

    output = output
        + format!(
            "{}descriptor: {}\n",
            PREFIX,
            class_file.get_constant_utf8(method.descriptor_index as usize)
        )
        .as_ref();

    if let Some(sig) = method.attributes.get_signature(class_file) {
        output = output + format!("{}signature: {}\n", PREFIX, sig).as_ref();
    }

    output = output + format!("{}flags: TODO\n", PREFIX).as_ref();

    print_attributes(class_file, &method.attributes, PREFIX);

    if print_code {
        let code_opt = method.get_code(class_file).unwrap();

        match code_opt {
            Some(code) => {
                output = output + format!("    Code:").as_ref();
                output = output
                    + format!(
                        "      stack={}, locals={}, args_size={}",
                        code.max_stack, code.max_locals, "TODO"
                    )
                    .as_ref();

                output = output + print_bytecode(class_file, &code.code).as_ref();

                if !code.exception_table.is_empty() {
                    output =
                        output + print_exception_table(class_file, &code.exception_table).as_ref();
                }
            }
            _ => {}
        }
    }

    output
}

fn print_bytecode(_class_file: &ClassFile, code: &[(usize, Bytecode)]) -> String {
    let mut output = String::new();

    for (i, bytecode) in code {
        output =
            output + format!("        {:>3}: {:35}\n", i, bytecode.to_string(*i as u16)).as_ref();

        // TODO: show constants to the side
    }

    output
}

fn print_exception_table(
    class_file: &ClassFile,
    exception_table: &[ExceptionTableEntry],
) -> String {
    let mut output = "      Exception table:\n         from    to  target type\n".to_string();

    for entry in exception_table.iter() {
        output = output
            + format!(
                "         {:5} {:5} {:5}   Class {}",
                entry.start_pc,
                entry.end_pc,
                entry.handler_pc,
                class_file.get_constant_class_str(entry.catch_type as usize),
            )
            .as_ref();
    }

    output
}

#[cfg(test)]
mod tests {
    use javap;

    #[test]
    fn javap_dummy_runs_without_error() {
        insta::assert_debug_snapshot!(javap("classes/Dummy.class", true));
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
