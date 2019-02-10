use std::collections::HashSet;

use attribute::*;
use field_access::*;

#[derive(Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
pub struct Field {
    pub access_flags: HashSet<FieldAccess>,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<Attribute>,
}
