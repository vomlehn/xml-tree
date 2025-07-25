/**
 * Parse XML text input and produce an XML tree
 */

use xml::name::OwnedName;

use crate::parser::LineNumber;
pub use crate::xml_document::{DirectElement, Element, ElementInfo};
use crate::xml_document_factory::{ElementData};

/**
 * Construct a tree
 */
pub struct XmlTreeElement {
    element:            Box<dyn Element>,
    open_subelement:    Option<Box<dyn Element>>,
}

impl ElementData<Box<dyn Element>> for XmlTreeElement {
    fn start(name: OwnedName, element_info: ElementInfo) -> XmlTreeElement {
        let element = Box::new(DirectElement::new(name, element_info, vec!(), vec!(), vec!(), vec!()));
        XmlTreeElement {
            element:            element,
            open_subelement:    None,
        }
    }

    fn end(&self) -> Box<dyn Element> {
        self.element.clone()
    }

    fn in_element(&self) -> bool {
        self.open_subelement.is_some()
    }

    fn start_subelement(&mut self, subelement: Box<dyn Element>) {
        self.open_subelement = Some(subelement);
    }

    fn end_subelement(&mut self) {
        let open_subelement = self.open_subelement().unwrap();
        self.element.subelements_mut().push(open_subelement);
        self.open_subelement = None;
    }

    fn name(&self) -> &str {
        self.element.name()
    }

    fn lineno(&self) -> LineNumber {
        self.element.lineno()
    }

    fn open_subelement(&self) -> Option<Box<dyn Element>> {
        self.open_subelement.clone()
    }
}

struct XmlTreeElementResult {
}

//impl XmlDocumentResult for 
