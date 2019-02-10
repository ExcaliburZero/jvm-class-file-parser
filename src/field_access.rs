use std::collections::HashSet;

use util::flag_is_set;

// Access flag masks are from Table 4.5-A of the JVM specification
//
// https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-4.html#jvms-4.5-200-A.1
const PUBLIC_FLAG: u16 = 0x0001;
const PRIVATE_FLAG: u16 = 0x0002;
const PROTECTED_FLAG : u16 = 0x0004;
const STATIC_FLAG: u16 = 0x0008;
const FINAL_FLAG : u16 = 0x0010;
const VOLATILE_FLAG: u16 = 0x0040;
const TRANSIENT_FLAG : u16 = 0x0080;
const SYNTHETIC_FLAG : u16 = 0x1000;
const ENUM_FLAG: u16 = 0x4000;

/// A flag that denotes an acess level or property of a field.
///
/// See the `access_flags` section of Chapter 4.5 of the JVM specification for
/// details.
///
/// https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-4.html#jvms-4.5-200-A
#[derive(Clone)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum FieldAccess {
    Public,
    Private,
    Protected,
    Static,
    Final,
    Volatile,
    Transient,
    Synthetic,
    Enum,
}

impl FieldAccess {
    /// Extracts the list of field access flags that are embedded in the given
    /// access flag value.
    ///
    /// Returns an error message if the extracted combination of access flags
    /// are inconsistent. (This validation has not yet been implemented)
    ///
    /// See Table 4.5-A of the JVM specification for more details.
    ///
    /// https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-4.html#jvms-4.5-200-A.1
    ///
    /// ```
    /// # use std::collections::HashSet;
    /// # use jvm_class_file_parser::FieldAccess;
    /// #
    /// let access_flags = 0b0000_0000_0100_0001;
    ///
    /// let mut expected = HashSet::new();
    /// expected.insert(FieldAccess::Public);
    /// expected.insert(FieldAccess::Volatile);
    ///
    /// assert_eq!(Ok(expected), FieldAccess::from_access_flags(access_flags));
    /// ```
    pub fn from_access_flags(access_flags: u16) -> Result<HashSet<FieldAccess>, String> {
        use FieldAccess::*;

        let mut access = HashSet::new();

        let is_public = flag_is_set(PUBLIC_FLAG, access_flags);
        let is_private = flag_is_set(PRIVATE_FLAG, access_flags);
        let is_protected = flag_is_set(PROTECTED_FLAG, access_flags);
        let is_static = flag_is_set(STATIC_FLAG, access_flags);
        let is_final = flag_is_set(FINAL_FLAG, access_flags);
        let is_volatile = flag_is_set(VOLATILE_FLAG, access_flags);
        let is_transient = flag_is_set(TRANSIENT_FLAG, access_flags);
        let is_synthetic = flag_is_set(SYNTHETIC_FLAG, access_flags);
        let is_enum = flag_is_set(ENUM_FLAG, access_flags);

        // TODO: Add validation for inconsistent access flags

        if is_public { access.insert(Public); }
        if is_private { access.insert(Private); }
        if is_protected { access.insert(Protected); }
        if is_static { access.insert(Static); }
        if is_final { access.insert(Final); }
        if is_volatile { access.insert(Volatile); }
        if is_transient { access.insert(Transient); }
        if is_synthetic { access.insert(Synthetic); }
        if is_enum { access.insert(Enum); }

        Ok(access)
    }
}
