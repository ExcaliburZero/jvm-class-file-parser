use std::collections::HashSet;

use util::flag_is_set;

// Access flag masks are from Table 4.1-B of the JVM specification
//
// https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-4.html#jvms-4.1-200-E.1
const PUBLIC_FLAG: u16 = 0x0001;
const FINAL_FLAG: u16 = 0x0010;
const SUPER_FLAG: u16 = 0x0020;
const INTERFACE_FLAG: u16 = 0x0200;
const ABSTRACT_FLAG: u16 = 0x0400;
const SYNTHETIC_FLAG: u16 = 0x1000;
const ANNOTATION_FLAG: u16 = 0x2000;
const ENUM_FLAG: u16 = 0x4000;
const MODULE_FLAG: u16 = 0x8000;

/// A flag that denotes an access level or property of a class.
///
/// See the `access_flags` section of Chapter 4.1 of the JVM specification for
/// details.
///
/// https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-4.html#jvms-4.1-200-E
#[derive(Clone)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum ClassAccess {
    Public,
    Final,
    Super,
    Interface,
    Abstract,
    Synthetic,
    Annotation,
    Enum,
    Module,
}

impl ClassAccess {
    /// Extracts the list of class access flags that are embedded in the given
    /// access flag value.
    ///
    /// Returns an error message if the extracted combination of access flags
    /// are inconsistent. (This validation has not yet been implemented)
    ///
    /// See Table 4.1-B of the JVM specification for more details.
    ///
    /// https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-4.html#jvms-4.1-200-E.1
    ///
    /// ```
    /// # use std::collections::HashSet;
    /// # use jvm_class_file_parser::ClassAccess;
    /// #
    /// let access_flags = 0b0000_0010_0000_0001;
    ///
    /// let mut expected = HashSet::new();
    /// expected.insert(ClassAccess::Public);
    /// expected.insert(ClassAccess::Interface);
    ///
    /// assert_eq!(Ok(expected), ClassAccess::from_access_flags(access_flags));
    /// ```
    pub fn from_access_flags(access_flags: u16) -> Result<HashSet<ClassAccess>, String> {
        use ClassAccess::*;

        let mut access = HashSet::new();

        let is_public = flag_is_set(PUBLIC_FLAG, access_flags);
        let is_final = flag_is_set(FINAL_FLAG, access_flags);
        let is_super = flag_is_set(SUPER_FLAG, access_flags);
        let is_interface = flag_is_set(INTERFACE_FLAG, access_flags);
        let is_abstract = flag_is_set(ABSTRACT_FLAG, access_flags);
        let is_synthetic = flag_is_set(SYNTHETIC_FLAG, access_flags);
        let is_annotation = flag_is_set(ANNOTATION_FLAG, access_flags);
        let is_enum = flag_is_set(ENUM_FLAG, access_flags);
        let is_module = flag_is_set(MODULE_FLAG, access_flags);

        // TODO: Add validation for inconsistent access flags

        if is_public { access.insert(Public); }
        if is_final { access.insert(Final); }
        if is_super { access.insert(Super); }
        if is_interface { access.insert(Interface); }
        if is_abstract { access.insert(Abstract); }
        if is_synthetic { access.insert(Synthetic); }
        if is_annotation { access.insert(Annotation); }
        if is_enum { access.insert(Enum); }
        if is_module { access.insert(Module); }

        Ok(access)
    }
}
