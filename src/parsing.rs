use std::io;
use std::io::{Error, ErrorKind, Read};
use std::str;

use attribute::*;
use class_access::*;
use class_file::ClassFile;
use constant_pool::*;
use field::*;
use field_access::*;
use method::*;
use std::ops::Deref;
use util::{promote_result_to_io, Contextable, FloatBuffer};

const EXPECTED_MAGIC: u32 = 0xCAFE_BABE;

const CONSTANT_TAG_UTF8: u8 = 1;
const CONSTANT_TAG_INTEGER: u8 = 3;
const CONSTANT_TAG_FLOAT: u8 = 4;
const CONSTANT_TAG_LONG: u8 = 5;
const CONSTANT_TAG_DOUBLE: u8 = 6;
const CONSTANT_TAG_CLASS: u8 = 7;
const CONSTANT_TAG_STRING: u8 = 8;
const CONSTANT_TAG_FIELDREF: u8 = 9;
const CONSTANT_TAG_METHODREF: u8 = 10;
const CONSTANT_TAG_INTERFACE_METHODREF: u8 = 11;
const CONSTANT_TAG_NAME_AND_TYPE: u8 = 12;
const CONSTANT_METHOD_HANDLE: u8 = 15;
const CONSTANT_METHOD_TYPE: u8 = 16;
const CONSTANT_DYNAMIC: u8 = 17;
const CONSTANT_INVOKE_DYNAMIC: u8 = 18;
const CONSTANT_MODULE: u8 = 19;
const CONSTANT_PACKAGE: u8 = 20;

const READ_MINOR_VERSION: &str = "Failed to read minor version.";
const READ_MAJOR_VERSION: &str = "Failed to read major version.";
const READ_CONSTANT_POOL: &str = "Failed to read constant pool.";
const READ_ACCESS_FLAGS: &str = "Failed to read access flags.";
const READ_THIS_CLASS: &str = "Failed to read the 'this class' index.";
const READ_SUPER_CLASS: &str = "Failed to read the 'super class' index.";
const READ_INTERFACES: &str = "Failed to read interfaces.";
const READ_FIELDS: &str = "Failed to read fields.";
const READ_METHODS: &str = "Failed to read methods.";
const READ_ATTRIBUTES: &str = "Failed to read attributes.";

pub fn read_class_file<R: Read>(file: &mut R) -> io::Result<ClassFile> {
    let magic = read_u32(file)?;

    if magic != EXPECTED_MAGIC {
        let error_msg = format!("The given file does not appear to be a valid JVM class file. JVM class files must start with the magic bytes \"CAFEBABE\", but this file started with \"{:x}\"", magic);

        return Err(Error::new(ErrorKind::Other, error_msg));
    }

    let minor_version = read_u16(file).context(READ_MINOR_VERSION)?;
    let major_version = read_u16(file).context(READ_MAJOR_VERSION)?;

    let constant_pool = read_constant_pool(file).context(READ_CONSTANT_POOL)?;

    let access_flags = read_u16(file).context(READ_ACCESS_FLAGS)?;
    let this_class = read_cp_index(file).context(READ_THIS_CLASS)?;
    let super_class = read_cp_index(file).context(READ_SUPER_CLASS)?;

    let interfaces = read_interfaces(file).context(READ_INTERFACES)?;
    let fields = read_fields(file).context(READ_FIELDS)?;
    let methods = read_methods(file).context(READ_METHODS)?;
    let attributes = read_attributes(file).context(READ_ATTRIBUTES)?;

    let access_flags = promote_result_to_io(ClassAccess::from_access_flags(access_flags))?;

    Ok(ClassFile {
        minor_version,
        major_version,
        constant_pool,
        access_flags,
        this_class,
        super_class,
        interfaces,
        fields,
        methods,
        attributes,
    })
}

fn read_u8<R: Read>(file: &mut R) -> io::Result<u8> {
    let mut buffer = [0; 1];

    file.read_exact(&mut buffer)?;

    Ok(u8::from_be_bytes(buffer))
}

fn read_u16<R: Read>(file: &mut R) -> io::Result<u16> {
    let mut buffer = [0; 2];

    file.read_exact(&mut buffer)?;

    Ok(u16::from_be_bytes(buffer))
}

fn read_cp_index<R: Read>(file: &mut R) -> io::Result<ConstantPoolIndex> {
    read_u16(file).map(ConstantPoolIndex::from)
}

fn read_u32<R: Read>(file: &mut R) -> io::Result<u32> {
    let mut buffer = [0; 4];

    file.read_exact(&mut buffer)?;

    Ok(u32::from_be_bytes(buffer))
}

fn read_n_bytes<R: Read>(file: &mut R, length: usize) -> io::Result<Vec<u8>> {
    let mut bytes = vec![0u8; length as usize];

    file.read_exact(&mut bytes)?;

    Ok(bytes)
}

#[allow(clippy::vec_box)]
fn read_constant_pool<R: Read>(file: &mut R) -> io::Result<Vec<ConstantPoolEntry>> {
    let constant_pool_count = read_u16(file)? - 1;

    let mut constant_pool = Vec::<ConstantPoolEntry>::with_capacity(constant_pool_count as usize);

    let mut idx = 0;
    while idx < constant_pool_count {
        let entry = read_constant_pool_entry(file)?;

        constant_pool.push(entry);

        // from https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-4.html#jvms-4.4.5
        // All 8-byte constants take up two entries in the constant_pool table of the class file.
        // If a CONSTANT_Long_info or CONSTANT_Double_info structure is the entry at index n in the
        // constant_pool table, then the next usable entry in the table is located at index n+2. The
        // constant_pool index n+1 must be valid but is considered unusable.
        // (we could abstract this slightly by placing the structure constraint in the enum)
        match constant_pool.get(constant_pool.len() - 1).unwrap().deref() {
            ConstantPoolEntry::ConstantLong { val: _ }
            | ConstantPoolEntry::ConstantDouble { val: _ } => {
                // we need this ensure proper indexes of all other entries
                constant_pool.push(ConstantPoolEntry::ConstantEmptySlot {});
                idx = idx + 1
            }
            _ => {}
        }

        idx = idx + 1
    }

    Ok(constant_pool)
}

fn read_constant_pool_entry<R: Read>(file: &mut R) -> io::Result<ConstantPoolEntry> {
    let tag = read_u8(file)?;

    let entry: ConstantPoolEntry = match tag {
        CONSTANT_TAG_UTF8 => read_constant_utf8(file)?,
        CONSTANT_TAG_CLASS => read_constant_class(file)?,
        CONSTANT_TAG_STRING => read_constant_string(file)?,
        CONSTANT_TAG_INTEGER => read_constant_integer(file)?,
        CONSTANT_TAG_FLOAT => read_constant_float(file)?,
        CONSTANT_TAG_LONG => read_constant_long(file)?,
        CONSTANT_TAG_DOUBLE => read_constant_double(file)?,
        CONSTANT_TAG_FIELDREF => read_constant_fieldref(file)?,
        CONSTANT_TAG_METHODREF => read_constant_methodref(file)?,
        CONSTANT_TAG_INTERFACE_METHODREF => read_constant_interface_methodref(file)?,
        CONSTANT_TAG_NAME_AND_TYPE => read_constant_name_and_type(file)?,
        CONSTANT_METHOD_HANDLE => read_method_handle(file)?,
        CONSTANT_METHOD_TYPE => read_method_type(file)?,
        CONSTANT_DYNAMIC => read_dynamic(file)?,
        CONSTANT_INVOKE_DYNAMIC => read_invoke_dynamic(file)?,
        CONSTANT_MODULE => read_module(file)?,
        CONSTANT_PACKAGE => read_package(file)?,
        _ => panic!(
            "Encountered unknown type of constant pool entry with a tag of: {}",
            tag
        ),
    };

    Ok(entry)
}

fn read_constant_utf8<R: Read>(file: &mut R) -> io::Result<ConstantPoolEntry> {
    let length = read_u16(file)?;

    let bytes = read_n_bytes(file, length as usize)?;

    // try str::from_utf8 which handles the happy path efficiently and then fall back to handling NULLs as needed
    // https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.4.7
    match str::from_utf8(&bytes) {
        Ok(parsed) => Ok(ConstantPoolEntry::ConstantUtf8 {
            string: parsed.to_string(),
        }),
        _ => {
            let mut new_bytes = Vec::with_capacity(bytes.len());

            // go through the bytes and when we find an encoded null (2 bytes), replace it with the single byte null
            let mut iter = bytes.iter().peekable();
            while let Some(&b) = iter.next() {
                if b == 0xc0 && iter.peek() == Some(&&(0x80 as u8)) {
                    new_bytes.push(0);
                    iter.next();
                } else {
                    new_bytes.push(b);
                }
            }

            // try parsing again and return the Err if it fails
            str::from_utf8(&new_bytes)
                .map(|string| ConstantPoolEntry::ConstantUtf8 {
                    string: string.to_string(),
                })
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        }
    }
}

fn read_constant_integer<R: Read>(file: &mut R) -> io::Result<ConstantPoolEntry> {
    let mut buffer = [0; 4];
    file.read_exact(&mut buffer)?;

    Ok(ConstantPoolEntry::ConstantInteger {
        val: i32::from_be_bytes(buffer),
    })
}

fn read_constant_float<R: Read>(file: &mut R) -> io::Result<ConstantPoolEntry> {
    let mut buffer = [0; 4];
    file.read_exact(&mut buffer)?;

    Ok(ConstantPoolEntry::ConstantFloat {
        val: FloatBuffer { buf: buffer },
    })
}

fn read_constant_long<R: Read>(file: &mut R) -> io::Result<ConstantPoolEntry> {
    let mut buffer = [0; 8];
    file.read_exact(&mut buffer)?;

    Ok(ConstantPoolEntry::ConstantLong {
        val: i64::from_be_bytes(buffer),
    })
}

fn read_constant_double<R: Read>(file: &mut R) -> io::Result<ConstantPoolEntry> {
    let mut buffer = [0; 8];
    file.read_exact(&mut buffer)?;

    Ok(ConstantPoolEntry::ConstantDouble {
        val: FloatBuffer { buf: buffer },
    })
}

fn read_constant_class<R: Read>(file: &mut R) -> io::Result<ConstantPoolEntry> {
    let name_index = read_cp_index(file)?;

    Ok(ConstantPoolEntry::ConstantClass { name_index })
}

fn read_constant_string<R: Read>(file: &mut R) -> io::Result<ConstantPoolEntry> {
    let string_index = read_cp_index(file)?;

    Ok(ConstantPoolEntry::ConstantString { string_index })
}

fn read_constant_fieldref<R: Read>(file: &mut R) -> io::Result<ConstantPoolEntry> {
    let class_index = read_cp_index(file)?;
    let name_and_type_index = read_cp_index(file)?;

    Ok(ConstantPoolEntry::ConstantFieldref {
        class_index,
        name_and_type_index,
    })
}

fn read_constant_methodref<R: Read>(file: &mut R) -> io::Result<ConstantPoolEntry> {
    let class_index = read_cp_index(file)?;
    let name_and_type_index = read_cp_index(file)?;

    Ok(ConstantPoolEntry::ConstantMethodref {
        class_index,
        name_and_type_index,
    })
}

fn read_constant_interface_methodref<R: Read>(file: &mut R) -> io::Result<ConstantPoolEntry> {
    let class_index = read_u16(file)?;
    let name_and_type_index = read_u16(file)?;

    Ok(ConstantPoolEntry::ConstantInterfaceMethodref {
        class_index,
        name_and_type_index,
    })
}

fn read_constant_name_and_type<R: Read>(file: &mut R) -> io::Result<ConstantPoolEntry> {
    let name_index = read_cp_index(file)?;
    let descriptor_index = read_cp_index(file)?;

    Ok(ConstantPoolEntry::ConstantNameAndType {
        name_index,
        descriptor_index,
    })
}

fn read_interfaces<R: Read>(file: &mut R) -> io::Result<Vec<ConstantPoolIndex>> {
    let interfaces_count = i32::from(read_u16(file)?);

    let mut interfaces = Vec::<_>::new();

    for _ in 0..interfaces_count {
        let entry = read_cp_index(file)?;

        interfaces.push(entry);
    }

    Ok(interfaces)
}

fn read_fields<R: Read>(file: &mut R) -> io::Result<Vec<Field>> {
    let fields_count = i32::from(read_u16(file)?);

    let mut fields = Vec::<Field>::new();

    for _ in 0..fields_count {
        let entry = read_field(file)?;

        fields.push(entry);
    }

    Ok(fields)
}

fn read_field<R: Read>(file: &mut R) -> io::Result<Field> {
    let access_flags = read_u16(file)?;
    let name_index = read_cp_index(file)?;
    let descriptor_index = read_cp_index(file)?;

    let attributes = read_attributes(file)?;

    let access_flags = promote_result_to_io(FieldAccess::from_access_flags(access_flags))?;

    Ok(Field {
        access_flags,
        name_index,
        descriptor_index,
        attributes,
    })
}

fn read_methods<R: Read>(file: &mut R) -> io::Result<Vec<Method>> {
    let methods_count = i32::from(read_u16(file)?);

    let mut methods = Vec::<Method>::new();

    for _ in 0..methods_count {
        let entry = read_method(file)?;

        methods.push(entry);
    }

    Ok(methods)
}

fn read_method<R: Read>(file: &mut R) -> io::Result<Method> {
    let access_flags = read_u16(file)?;
    let name_index = read_cp_index(file)?;
    let descriptor_index = read_cp_index(file)?;

    let attributes = read_attributes(file)?;

    Ok(Method {
        access_flags,
        name_index,
        descriptor_index,
        attributes,
    })
}

pub fn read_attributes<R: Read>(file: &mut R) -> io::Result<AttributeSet> {
    let attributes_count = read_u16(file)?;

    let mut attributes = Vec::<Attribute>::new();

    for _ in 0..attributes_count {
        let entry = read_attribute(file)?;

        attributes.push(entry);
    }

    Ok(AttributeSet { attributes })
}

fn read_attribute<R: Read>(file: &mut R) -> io::Result<Attribute> {
    let attribute_name_index = read_cp_index(file)?;
    let attribute_length = read_u32(file)?;

    let info = read_n_bytes(file, attribute_length as usize)?;

    Ok(Attribute {
        attribute_name_index,
        info,
    })
}

fn read_method_handle<R: Read>(file: &mut R) -> io::Result<ConstantPoolEntry> {
    let reference_kind = read_u8(file)?;
    let reference_index = read_u16(file)?;

    Ok(ConstantPoolEntry::ConstantMethodHandle {
        reference_kind,
        reference_index,
    })
}

fn read_method_type<R: Read>(file: &mut R) -> io::Result<ConstantPoolEntry> {
    let descriptor_index = read_u16(file)?;

    Ok(ConstantPoolEntry::ConstantMethodType { descriptor_index })
}

fn read_dynamic<R: Read>(file: &mut R) -> io::Result<ConstantPoolEntry> {
    let bootstrap_method_attr_index = read_u16(file)?;
    let name_and_type_index = read_u16(file)?;

    Ok(ConstantPoolEntry::ConstantDynamic {
        bootstrap_method_attr_index,
        name_and_type_index,
    })
}

fn read_invoke_dynamic<R: Read>(file: &mut R) -> io::Result<ConstantPoolEntry> {
    let bootstrap_method_attr_index = read_u16(file)?;
    let name_and_type_index = read_u16(file)?;

    Ok(ConstantPoolEntry::ConstantInvokeDynamic {
        bootstrap_method_attr_index,
        name_and_type_index,
    })
}

fn read_module<R: Read>(file: &mut R) -> io::Result<ConstantPoolEntry> {
    let name_index = read_u16(file)?;

    Ok(ConstantPoolEntry::ConstantModule { name_index })
}

fn read_package<R: Read>(file: &mut R) -> io::Result<ConstantPoolEntry> {
    let name_index = read_u16(file)?;

    Ok(ConstantPoolEntry::ConstantPackage { name_index })
}

#[cfg(test)]
mod tests {
    use std::io::{self, Cursor};

    use super::read_constant_utf8;

    #[test]
    fn read_utf8_with_embedded_null() -> io::Result<()> {
        let bytes = vec![
            0, 9, // length
            0xd0, 0xaa, // cyrillic letter Ъ
            0xc0, 0x80, // 2-byte encoded null
            0xd0, 0xab, // cyrillic letter Ы
            0xc0, 0x80, // 2-byte encoded null
            0x42,
        ];
        let mut cursor = Cursor::new(bytes);
        let parsed = read_constant_utf8(&mut cursor)?;
        assert_eq!(
            parsed,
            crate::ConstantPoolEntry::ConstantUtf8 {
                string: "Ъ\0Ы\0B".to_string()
            }
        );
        Ok(())
    }
}
