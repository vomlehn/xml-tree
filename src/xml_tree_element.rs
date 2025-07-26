/**
 * Parse XML text input and produce an XML tree
 */

use xml::name::OwnedName;

use crate::parser::LineNumber;
// FIXME: rename XmlDocument to XmlTreeDocument
pub use crate::xml_document::{DirectElement, Element, ElementInfo, XmlDocument};
use crate::xml_document_factory::{DocumentData, DocumentInfo, ElementData};

/**
 * Information for one element in an XML tree
 */
pub struct XmlTreeElement {
    element:            Box<dyn Element>,
    open_subelement:    Option<Box<dyn Element>>,
}

impl ElementData for XmlTreeElement
{
    type ElementResult = Box<dyn Element>;

    fn start(name: OwnedName, element_info: ElementInfo) -> Self {
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

pub struct XmlTreeDocument {
    document_info:  DocumentInfo,
}

impl DocumentData for XmlTreeDocument {
    type DocumentResult = XmlDocument;

    fn start(document_info: DocumentInfo) -> Self {
        XmlTreeDocument {
            document_info:  document_info,
        }
    }

    fn end(&self, top_element: Vec<Box<dyn Element>>) -> Self::DocumentResult {
        XmlDocument::new(self.document_info.clone(), top_element)
    }
}
