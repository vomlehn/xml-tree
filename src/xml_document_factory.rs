/*
 * Take an Element tree and generate an XmlFactorTree, which is used
 * to parse XML input
 */
// FIXME: delete all uses of expect(), everywhere

//use std::borrow::Borrow;
//use std::cell::RefCell;
//use std::collections::BTreeMap;
use std::io::Read;
use xml::attribute::OwnedAttribute;
//use xml::common::XmlVersion;
use xml::name::OwnedName;
use xml::reader::XmlEvent;
use xml::namespace::Namespace;

use crate::parser::{LineNumber, Parser, XmlDirectElement/*, XmlElement*/};
//use crate::walk_and_print::PrintBaseLevel;
pub use crate::xml_document::{DirectElement, DocumentInfo, Element, ElementInfo, XmlDocument};
pub use crate::xml_document_error::XmlDocumentError;
//use crate::xml_schema::{Element, XmlSchema};
use crate::xml_schema::XmlSchema;

const READING_XML: bool = false;

/*
 * Structure used to hold parsing information
 * parser:          Used to extract XmlElement objects from the input stream
 * xml_schema:  Definition of what the input is expected to look like
 */
pub struct XmlDocumentFactory<'a, R: Read + 'a> {
    parser: Parser<R>,
    pub xml_schema: &'a XmlSchema<'a>,
}

#[derive(Clone, Debug, PartialEq)]
enum ParseState {
    Init,
    Top,
    InElement(OwnedName, LineNumber),
    End,
}

struct ElementArgs {
    name:       String,
    attributes: Vec<OwnedAttribute>,
    namespace:  String,
}

impl<'a, R: Read + 'a> XmlDocumentFactory<'_, R> {
    pub fn new_from_reader<T: Read + 'a>(
        reader: T,
        xml_schema: &'a XmlSchema<'a>,
    ) -> Result<XmlDocument, XmlDocumentError> {
        let parser = Parser::<T>::new(reader);

        let mut xml_factory = XmlDocumentFactory::<T> {
            parser: parser,
            xml_schema: xml_schema,
        };

/*
        let document_info = xml_factory.parse_start_document()?;
        let before_pieces = self.parse_pieces()?;
*/
        let xml_document = xml_factory.parse_document::<T>();
        xml_document
    }

    fn parse_document<T: Read + 'a>(&mut self) -> Result<XmlDocument, XmlDocumentError> {
        let document_info = self.parse_start_document()?;

        let xml_element = self.parser.lookahead()?;
        let event = xml_element.event.clone();
        let top_element = if let XmlEvent::StartElement{name, attributes, namespace} = xml_element.event {
            self.parser.skip();
            let element_info = ElementInfo::new(xml_element.lineno, attributes, namespace);
            self.parse_element(name, element_info)?
        } else {
            panic!("FIXME: Expected element, got {:?}", xml_element.event);
        };
            
        self.end_element(&event)?;
        Ok(XmlDocument::new(document_info, vec!(top_element)))
    }

    /*
     * Parse a StartDocument. Nothing can preceed this
     */
    fn parse_start_document(&mut self) -> Result<DocumentInfo, XmlDocumentError> {
        let xml_element = self.parser.next()?;

        if let XmlEvent::StartDocument{version, encoding, standalone} = xml_element.event {
            Ok(DocumentInfo::new(version, encoding, standalone))
        } else {
            panic!("FIXME: document doesn't start with StartDocument")
        }
    }

    /*
     * Parse an element. We have already seen the XmlStartElement as a lookahead.
     */
    fn parse_element(&mut self, name: OwnedName, element_info: ElementInfo) ->
        Result<Box<dyn Element>, XmlDocumentError> {
        self.parser.skip();
        let mut element = DirectElement::new(name, element_info, vec!(), vec!(), vec!(), vec!());
        let mut subelements = Vec::new();

        loop {
            let xml_element = self.parser.lookahead()?;
            match xml_element.event {
                XmlEvent::StartElement{name, attributes, namespace} => {
                    let element_info = ElementInfo::new(xml_element.lineno, attributes, namespace);
                    let subelement = self.parse_element(name, element_info)?;
                    subelements.push(subelement);
                },

                XmlEvent::EndElement{name} => {
                    if element.name() != name.local_name {
                        panic!("FIXME: name of element {} at {} does not match name of closing element {} at {}", element.name(), element.lineno(), name, xml_element.lineno);
                    }
                    break;
                },

                _ => panic!("FIXME: got {:?} instead of closing element {} at {}", xml_element.event, element.name(), element.lineno()),
            }
        }

        element.subelements_mut().append(&mut subelements);
        Ok(Box::new(element))
    }

    /*
     * We expect EndDocument, parsed as a lookahead
     */
    fn end_element(&mut self, event: &XmlEvent) -> Result<(), XmlDocumentError> {
        if let XmlEvent::EndDocument = event {
            self.parser.skip();
            return Ok(())
        }
        panic!("FIXME: Expected end of document but found {:?}", event)
    }
}
