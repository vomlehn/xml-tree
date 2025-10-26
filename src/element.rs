/*
 * Information about an XML element, plus its subelements
 */

use dyn_clone::DynClone;
use std::fmt;
use std::io::Write;
use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::namespace::Namespace;
use xml::reader::XmlEvent;

// FIXME: split into walk and parse sets of errors
//use crate::xml_document_error::XmlDocumentError;
use crate::misc::{nl_indent, display_vec, owned_attribute_to_string,
    xml_event_to_string};
use crate::ParseLoc;

// FIXME: need to move to BaseElement or something
use crate::parse_schema::SchemaElement;

/*
 * Both indent() and nl_indent() produce strings used for indentation whose lengths
 * are a multiple of their arguments. Rust outputs have a header, which is followed
 * by the XML schema tree.
 * TREE_DEPTH       The minimum indentation of the schema tree
 * ELEMENT_INDENTS  The indentation from on ElementSchema to the next nested
 *                  ElementSchema.
 */
const TREE_DEPTH: usize = 2;
const ELEMENT_INDENTS: usize = 2;

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
    let depth1 = depth + 1;

    let _ = write!(output, "{}ElementInfo::new(", nl_indent(depth));
    display_owned_name(output, depth1, &element_info.owned_name)?;
    let _ = writeln!(output, "{}ParseLoc::new(\"{}\", {}), ", nl_indent(depth1),
        element_info.parse_loc.path, element_info.parse_loc.lineno);
    let _ = display_vec::<OwnedAttribute, _>(output, depth, &element_info.attributes,
        owned_attribute_to_string);

    let _ = write!(output, ",");
    let _ = write!(output, "{}Namespace(BTreeMap::<String, String>::new()),",
        nl_indent(depth1));
    let _ = write!(output, "{}),", nl_indent(depth));
    Ok(())
}

dyn_clone::clone_trait_object!(Element);

/*
 * Print the first part of the SchemaElement
 * self:    self
 * output:  Where to write the text
 * depth:   Number of nested SchemaElement
 */
pub fn display_element_start(element: &SchemaElement, output: &mut dyn Write, depth: usize, name: String) -> fmt::Result {
    let depth0 = TREE_DEPTH + ELEMENT_INDENTS * depth;
    let depth1 = depth0 + 1;

    // FIXME: return error code
    let _ = write!(output, "{}vec!(Box::new({}::new(",
        nl_indent(depth0), name);

    // FIXME: check for errors
    let _ = display_element_info(output, depth1, &element.element_info);
    let _ = write!(output, "{}", nl_indent(depth1));

    let _ = display_vec::<XmlEvent, _>(output, depth1, &element.before_element,
        xml_event_to_string);
    let _ = write!(output, ", ");

    let _ = display_vec::<XmlEvent, _>(output, depth1, &element.content,
        xml_event_to_string);
    let _ = write!(output, ", ");

    let _ = display_vec::<XmlEvent, _>(output, depth1, &element.after_element,
        xml_event_to_string);
    let _ = write!(output, ",");

    // This defines the start of the SchemaElement subelements
    let _ = write!(output, " vec!(");
    Ok(())
}

// FIXME: remove _element
pub fn display_element_end(element: &SchemaElement, output: &mut dyn Write,
    depth: usize) -> fmt::Result {
    let depth0 = TREE_DEPTH + ELEMENT_INDENTS * depth;
    let depth1 = depth0 + 1;
    let depth2 = depth0 + 2;

    // FIXME: check for errors
    // Close off the list of subelements
    if !element.has_subelements {
        let _ = write!(output, ") /* Close subelement list 0 */");
    } else {
        let _ = write!(output, "{}) /* Close subelement list 1 */", nl_indent(depth1));
    }

    let _ = write!(output, "{}))), /* Close vec!(Box::new(SchemaElement::new( */",
        nl_indent(depth0));

    Ok(())
}

pub fn display_owned_name(output: &mut dyn Write, depth: usize, owned_name: &OwnedName) -> fmt::Result {
    let depth1 = depth + 1;

    // FIXME: check for errors
    let _ = write!(output, "{}OwnedName{{local_name: \"{}\".to_string(),",
        nl_indent(depth), owned_name.local_name);
    let _ = write!(output, "{}namespace: {:?},", nl_indent(depth1),
        owned_name.namespace);
    let _ = write!(output, "{}prefix: {:?}}},", nl_indent(depth1), owned_name.prefix);
    Ok(())
}
