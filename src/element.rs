/*
 * Information about an XML element, plus its subelements
 */

use dyn_clone::DynClone;
use std::fmt;
use std::io::Write;
use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::namespace::Namespace;

// FIXME: split into walk and parse sets of errors
//use crate::xml_document_error::XmlDocumentError;
use crate::misc::{nl_indent, write_vec, rust_owned_attribute};
//use crate::misc::{rust_str, rust_to_string};
use crate::{ParseLoc, rust_owned_name};

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
    fn get(&self, name: &str) -> Option<&(dyn Element + Send + Sync)>;
    fn name(&self) -> &str;
    // This is actually available in XmlEvent. Use that.
    fn parse_loc(&self) -> ParseLoc;
    fn subelements(&self) -> &Vec<Box<dyn Element + Send + Sync>>;
    fn subelements_mut(&mut self) -> &mut Vec<Box<dyn Element + Send + Sync>>;
}

#[derive(Clone, Debug)]
pub struct ElementInfo {
    pub owned_name: OwnedName,
    pub parse_loc:  ParseLoc,
    pub attributes: Vec<OwnedAttribute>,
}

impl ElementInfo {
    pub fn new(
        owned_name:     OwnedName,
        parse_loc:      ParseLoc,
        attributes:     Vec<OwnedAttribute>,
        _namespace:     Namespace,
    ) -> ElementInfo {
        ElementInfo {
            owned_name,
            parse_loc,
            attributes,
        }
    }

    pub fn write(&self, output: &mut dyn Write, depth: usize) -> fmt::Result {
        // FIXME: return error
        let depth1 = depth + 1;

        let _ = write!(output, "{}ElementInfo::new(", nl_indent(depth));
        let _ = write!(output, " ");
        let _ = write!(output, "{}", rust_owned_name(&self.owned_name, depth1));
        let _ = write!(output, ",");
        let _ = write!(output, "{}", nl_indent(depth1));
        let _ = write!(output, "{}", self.parse_loc.rust());
        let _ = write!(output, ",");
        let _ = write_vec::<OwnedAttribute, _>(output, depth, &self.attributes,
            rust_owned_attribute);

        let _ = write!(output, ",");
        let _ = write!(output, "{}Namespace(BTreeMap::<String, String>::new()),",
            nl_indent(depth1));
        let _ = write!(output, "{}),", nl_indent(depth));
        Ok(())
    }
}

dyn_clone::clone_trait_object!(Element);
