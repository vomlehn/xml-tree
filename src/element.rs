/*
 * Information about an XML element, plus its subelements
 */

use dyn_clone::DynClone;
//use std::convert::Infallible;
use std::fmt;
use std::io::Write;
//use std::ops::{FromResidual, Try};
use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::namespace::Namespace;
use xml::reader::XmlEvent;

// FIXME: split into walk and parse sets of errors
//use crate::xml_document_error::XmlDocumentError;
use crate::misc::{nl_indent, vec_display};
use crate::ParseLoc;

// FIXME: need to move to BaseElement or something
use crate::parse_schema::SchemaElement;

const TREE_DEPTH: usize = 2;

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
//    fn display(&self, depth: usize) -> fmt::Result;
//    fn debug(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result;
    fn get(&self, name: &str) -> Option<&dyn Element>;
    fn name(&self) -> &str;
    // This is actually available in XmlEvent. Use that.
    fn parse_loc(&self) -> ParseLoc;
    fn subelements(&self) -> &Vec<Box<dyn Element>>;
    fn subelements_mut(&mut self) -> &mut Vec<Box<dyn Element>>;
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
}

pub fn display_element_info(output: &mut dyn Write, depth: usize, element_info: &ElementInfo) -> fmt::Result {
    // FIXME: return error
    let _ = write!(output, "{}ElementInfo::new({}, vec!(),", nl_indent(depth),
        element_info.parse_loc);
    let _ = vec_display::<OwnedAttribute>(output, depth, &element_info.attributes);
    let _ = write!(output, "{}Namespace(BTreeMap::<String, String>::new())),",
        nl_indent(depth + 1));
    Ok(())
}

dyn_clone::clone_trait_object!(Element);

// FIXME: move at least some of the following printing things to element
/*
 * Print the first part of the SchemaElement
 * self:    self
 * output:  Where to write the text
 * depth:   Number of nested SchemaElement
 */
pub fn display_element_start(element: &SchemaElement, output: &mut dyn Write,
    depth: usize, name: String) -> fmt::Result {
    let depth0 = TREE_DEPTH + 3 * depth;
    let depth1 = depth0 + 1;
    let depth2 = depth0 + 2;

    // FIXME: return error code
    let _ = write!(output, "{}vec!(Box::new({}::new(",
        nl_indent(depth0), name);

    let owned_name = OwnedName {
        local_name: element.name().to_string(),
        namespace:  None,
        prefix:     None,
    };
    display_owned_name(output, depth1, &owned_name)?;

    let attr_owned_name = OwnedName {
        local_name: "attr1".to_string(),
        namespace:  None,
        prefix:     None,
    };
    let owned_attribute = OwnedAttribute{
        name:   attr_owned_name,
        value:  "value".to_string(),
    };
    let element_info = ElementInfo {
        parse_loc:  ParseLoc::new("TBD".to_string(), 0),
        owned_name: owned_name,
        attributes: vec!(owned_attribute),
    };
    // FIXME: check for errors
    let _ = display_element_info(output, depth1, &element_info);
    let _ = write!(output, "{}", nl_indent(depth1));

    let _ = vec_display::<XmlEvent>(output, depth1, &element.before_element);
    let _ = write!(output, ", ");
    let _ = vec_display::<XmlEvent>(output, depth1, &element.content);
    let _ = write!(output, ", ");
    let _ = vec_display::<XmlEvent>(output, depth1, &element.after_element);
    let _ = write!(output, ",");
    let _ = write!(output, "{}vec!(", nl_indent(depth2));
    Ok(())
}

pub fn display_element_end(element: &SchemaElement, output: &mut dyn Write,
    depth: usize) -> fmt::Result {
    let depth0 = TREE_DEPTH + 3 * depth;
    let depth1 = depth0 + 1;
    let depth2 = depth0 + 2;

    // FIXME: check for errors
    let _ = write!(output, "{})", nl_indent(depth2));
    let _ = write!(output, "{})", nl_indent(depth1));
    let _ = write!(output, "{}),", nl_indent(depth0));
        // FIXME: return error
    Ok(())
}

pub fn display_owned_name(output: &mut dyn Write, depth: usize, owned_name: &OwnedName) -> fmt::Result {
    // FIXME: check for errors
    let _ = write!(output, "{}OwnedName{{local_name: \"{}\".to_string(),", nl_indent(depth), owned_name.local_name);
    let _ = write!(output, "{}namespace: {:?}, prefix: {:?}}},", nl_indent(depth + 1), owned_name.namespace, owned_name.prefix);
    Ok(())
}
