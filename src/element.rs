/*
 * Information about an XML element, plus its subelements
 */

use dyn_clone::DynClone;
//use std::convert::Infallible;
use std::fmt;
//use std::ops::{FromResidual, Try};
use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::namespace::Namespace;

// FIXME: split into walk and parse sets of errors
//use crate::xml_document_error::XmlDocumentError;
use crate::misc::nl_indent;
use crate::parse_item::LineNumber;

/*
 * trait making TreeElement and IndirectElement work well together
 * name:            Function that returns the name of the element
 * get:             Search for an element by name. FIXME: This is probably for
 *                  future expansion.
 * name:            Returns the name for the element. FIXME: This really only
 *                  makes sense for TreeElements and should probably be removed
 * subelements:     Returns a reference to a vector of Elements. These are
 *                  sub-elements for TreeElements and a linear set of elements
 *                  at the same depth as the parent element for IndirectElements.
 * subelements_mut: Like subelements but returns a mutable value
 */
pub trait Element: DynClone {
    fn display(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result;
    fn debug(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result;
    fn get(&self, name: &str) -> Option<&dyn Element>;
    fn name(&self) -> &str;
    // This is actually available in XmlEvent. Use that.
    fn lineno(&self) -> LineNumber;
    fn subelements(&self) -> &Vec<Box<dyn Element>>;
    fn subelements_mut(&mut self) -> &mut Vec<Box<dyn Element>>;
}

#[derive(Clone, Debug)]
pub struct ElementInfo {
    pub owned_name: OwnedName,
    pub lineno:     LineNumber,
}

impl ElementInfo {
    pub fn new(
        owned_name:     OwnedName,
        lineno:         LineNumber,
        _attributes:    Vec<OwnedAttribute>,
        _namespace:     Namespace,
    ) -> ElementInfo {
        ElementInfo {
            owned_name,
            lineno,
        }
    }
}

pub fn element_info_display(f: &mut fmt::Formatter<'_>, depth: usize, element_info: &ElementInfo) -> fmt::Result {
    write!(f, "{}ElementInfo::new({}, vec!(),", nl_indent(depth), element_info.lineno)?;
    write!(f, "{}Namespace(BTreeMap::<String, String>::new())),", nl_indent(depth + 1))
}

dyn_clone::clone_trait_object!(Element);

/* Check all Display impls to ensure status is passed back properly */
impl fmt::Display for Box<dyn Element> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f, 0)
    }
}

impl fmt::Debug for Box<dyn Element> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
// FIXME: do better
        self.display(f, 0)
    }
}
