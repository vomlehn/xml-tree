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
use xml::reader::XmlEvent;

// FIXME: split into walk and parse sets of errors
//use crate::xml_document_error::XmlDocumentError;
//use crate::parse_tree::ParseTree;
use crate::parse_item::LineNumber;
//use crate::parse_doc::ParseDoc;
use crate::parse_doc::Accumulator;
// FIXME: not sure where these should really reside
use crate::walk_print::{nl_indent, vec_display, XmlDisplay};

/*
 * trait making DirectElement and IndirectElement work well together
 * name:            Function that returns the name of the element
 * get:             Search for an element by name. FIXME: This is probably for
 *                  future expansion.
 * name:            Returns the name for the element. FIXME: This really only
 *                  makes sense for DirectElements and should probably be removed
 * subelements:     Returns a reference to a vector of Elements. These are
 *                  sub-elements for DirectElements and a linear set of elements
 *                  at the same depth as the parent element for IndirectElements.
 * subelements_mut: Like subelements but returns a mutable value
 */
pub trait Element: DynClone {
    fn display(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result;
    fn debug(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result;
    fn get(&self, name: &str) -> Option<&dyn Element>;
    fn name(&self) -> &str;
    fn lineno(&self) -> LineNumber;
    fn subelements(&self) -> &Vec<Box<dyn Element>>;
    fn subelements_mut(&mut self) -> &mut Vec<Box<dyn Element>>;

/*
    /*
     * Set up to process an element
     */
    fn start(&self) -> Accumulator;
*/

/*
    /**
     * Return the final result from processing an Element
     */
    fn end(&self, accumulator: Accumulator) -> Result<Self::Value, XmlDocumentError>;
*/
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

#[derive(Clone)]
pub struct DirectElement {
    pub element_info: ElementInfo,
    pub before_element: Vec<XmlEvent>,
    pub content: Vec<XmlEvent>,
    pub after_element: Vec<XmlEvent>,
    pub subelements: Vec<Box<dyn Element>>,
}

impl DirectElement {
    pub fn new(element_info: ElementInfo,
        before_element: Vec::<XmlEvent>,
        content: Vec::<XmlEvent>,
        after_element: Vec::<XmlEvent>,
        subelements: Vec<Box<dyn Element>>) -> DirectElement {
        DirectElement {
            element_info,
            subelements,
            before_element,
            content,
            after_element,
        }
    }
}

impl Default for DirectElement {
    fn default() -> DirectElement {
        DirectElement {
            element_info: ElementInfo {
                owned_name: OwnedName {
                    local_name: "".to_string(),
                    namespace:  None,
                    prefix:     None
                },
                lineno:     0,
            },
            subelements: vec!(),
            before_element: vec!(),
            content: vec!(),
            after_element: vec!(),
        }
    }
}

impl fmt::Display for DirectElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f, 0)
    }
}

impl fmt::Debug for DirectElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.debug(f, 0)
    }
}

impl Element for DirectElement {
    fn display(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {

        write!(f, "{}vec!(Box::new(DirectElement::new(", nl_indent(depth))
            .expect("Unable to write Box::new");

        let owned_name = OwnedName {
            local_name: self.name().to_string(),
            namespace:  None,
            prefix:     None,
        };
        owned_name_display(f, depth + 1, &owned_name)?;

        let element_info = ElementInfo {
            lineno:     0,
            owned_name: owned_name,
        };
        element_info_display(f, depth + 1, &element_info)?;
        write!(f, "{}", nl_indent(depth + 1))?;
        vec_display::<XmlEvent>(f, depth, &self.before_element)?;
        write!(f, ", ")?;
        vec_display::<XmlEvent>(f, depth, &self.content)?;
        write!(f, ", ")?;
        vec_display::<XmlEvent>(f, depth, &self.after_element)?;
        write!(f, ",")?;
        write!(f, "{}vec!(", nl_indent(depth + 1))
    }

    fn debug(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        self.display(f, depth)
    }

    /**
     * Find a subelement (one level deeper) with the given name
     */
    fn get(&self, name: &str) -> Option<&dyn Element> {
println!("get: looking for {} in {}", name, self.name());
println!("...");
for x in self.subelements() {
    println!(" {}", x);
}
        self.subelements()
            .iter()
            .find(|&x| {
                println!("get: is {} == {}", x.name(), name);
                x.name() == name
            })
            .map(|v| &**v)
    }

    /*
     * Return the element name
     */
    // FIXME: maybe remove this from Element
    fn name(&self) -> &str {
        &self.element_info.owned_name.local_name
    }

    fn lineno(&self) -> LineNumber {
        self.element_info.lineno
    }

    /**
     * Return a vector of all subelements.
     */
    fn subelements<'b>(&'b self) -> &'b Vec<Box<dyn Element + 'static>> {
        &self.subelements
    }

    /**
     * Return a mutable vector of all subelements.
     */
    fn subelements_mut<'b>(&'b mut self) -> &'b mut Vec<Box<dyn Element + 'static>> {
        &mut self.subelements
    }
}

impl XmlDisplay for DirectElement {
    fn print(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {

        write!(f, "{}Box::new(DirectElement::new(", nl_indent(depth))
            .expect("Unable to write Box::new");

        let element_info = ElementInfo {
            lineno: 0,
            owned_name: OwnedName {
                        local_name: self.name().to_string(),
                        namespace:  None,
                        prefix:     None,
            },
        };

        owned_name_display(f, depth + 1, &element_info.owned_name)?;
        element_info_display(f, depth + 1, &element_info)?;
        write!(f, "{}vec!(), vec!(), vec!(),", nl_indent(depth + 1))?;

        write!(f, "{}vec!(", nl_indent(depth + 1))
    }
}

fn owned_name_display(f: &mut fmt::Formatter<'_>, depth: usize, owned_name: &OwnedName) -> fmt::Result {
    write!(f, "{}OwnedName{{local_name: \"{}\".to_string(),", nl_indent(depth), owned_name.local_name)?;
    write!(f, "{}namespace: {:?}, prefix: {:?}}},", nl_indent(depth + 1), owned_name.namespace, owned_name.prefix)
}

fn element_info_display(f: &mut fmt::Formatter<'_>, depth: usize, element_info: &ElementInfo) -> fmt::Result {
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
