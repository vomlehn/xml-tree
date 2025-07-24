/*
 * Take an Element tree and generate an XmlFactorTree, which is used
 * to parse XML input
 */
// FIXME: delete all uses of expect(), everywhere

//use std::borrow::Borrow;
//use std::cell::RefCell;
//use std::collections::BTreeMap;
use std::io::Read;
//use xml::attribute::OwnedAttribute;
//use xml::common::XmlVersion;
use xml::name::OwnedName;
use xml::reader::XmlEvent;
//use xml::namespace::Namespace;

use crate::parser::{/*LineNumber, */Parser/*, XmlDirectElement*//*, XmlElement*/};
//use crate::walk_and_print::PrintBaseLevel;
pub use crate::xml_document::{DirectElement, DocumentInfo, Element, ElementInfo, XmlDocument};
pub use crate::xml_document_error::XmlDocumentError;
//use crate::xml_schema::{Element, XmlSchema};
use crate::xml_schema::XmlSchema;

// const READING_XML: bool = false;

/*
 * Structure used to hold parsing information
 * parser:          Used to extract XmlElement objects from the input stream
 * xml_schema:  Definition of what the input is expected to look like
 */
pub struct XmlDocumentFactory<'a, R: Read + 'a> {
    parser: Parser<R>,
    pub xml_schema: &'a XmlSchema<'a>,
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

        let xml_document = xml_factory.parse_document::<T>();
        xml_document
    }

    fn parse_document<T: Read + 'a>(&mut self) -> Result<XmlDocument, XmlDocumentError> {
        let document_info = self.parse_start_document()?;

        // Read the next XML event, which is expected to be the start of an element. We use a
        // lookahead so that we can be specific about an error if one occurred
        let xml_element = self.parser.lookahead()?;

        let top_element = match xml_element.event {
            XmlEvent::StartElement{name, attributes, namespace} => {
                let element_info = ElementInfo::new(xml_element.lineno, attributes, namespace);
                self.parse_element(name, element_info, 0)?
            },

            _ => panic!("FIXME: Expected element, got {:?}", xml_element.event),
        };
//println!("parse_document: top_element.subelements().len() {}", top_element.subelements().len());

        self.parse_end_document()?;

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
    fn parse_element(&mut self, name: OwnedName, element_info: ElementInfo, depth: usize) ->
        Result<Box<dyn Element>, XmlDocumentError> {
//println!("{}<{}>: start-element", "   ".repeat(depth), name.local_name);
        self.parser.skip();
        let mut element = DirectElement::new(name, element_info, vec!(), vec!(), vec!(), vec!());
        let mut open_subelement: Option<Box<dyn Element>> = None;
//        let mut n: Option<String> = None;

        // Now parse all subelements of this element until we get to the EndElement for this
        // element.
        loop {
//println!("{}1-subelements.len() for <{}>: {}", "   ".repeat(depth), element.name(), element.subelements().len());
            let xml_element = self.parser.lookahead()?;

/*
let event = xml_element.event.clone();
match event {
    XmlEvent::StartElement{name, ..} => println!("--> {}: start", name.local_name),
    XmlEvent::EndElement{name} => println!("--> {}: end", name.local_name),
    _ => {},
}
*/

            match xml_element.event {
                XmlEvent::StartElement{name, attributes, namespace} => {
//println!("{}start-subelement <{}> for <{}>", "   ".repeat(depth), name.local_name, element.name());

                    if open_subelement.is_some() {
                        panic!("FIXME: element <{}> definition should be closed before defining <{}>",
                            element.name(), open_subelement.unwrap().name());
                    }

                    let element_info = ElementInfo::new(xml_element.lineno, attributes, namespace);
//                    n = Some(name.local_name.clone());
                    let subelement = self.parse_element(name, element_info, depth + 1)?;
                    open_subelement = Some(subelement);
                },

                XmlEvent::EndElement{name} => {

                    // If we are not in an element, this end element is for the element we are
                    // entered this function to parse

                    match open_subelement {
                        None => {
//println!("{}<{:?}>: pop", "   ".repeat(depth), element.name());
                                break;
                        },
                        Some(subelement) => {
                            self.parser.skip();
                            if name.local_name != subelement.name() {
                                panic!("FIXME: name of element <{}> at {} does not match name of closing element <{}> at {}", name, xml_element.lineno, subelement.name(), subelement.lineno());
                            }

                            element.subelements_mut().push(subelement);
/*
print!("{}end-subelement {} for {}: [", "   ".repeat(depth), name.local_name, element.name());
for e in element.subelements() {
    print!(" {}", e.name());
}
println!("]");
*/
                            open_subelement = None;
//n = None;
                        },
                    }
//println!("{}2-subelements.len() for <{}>: {}", "   ".repeat(depth), element.name(), element.subelements().len());
                },

                XmlEvent::EndDocument => {
                    if open_subelement.is_some() {
                        panic!("FIXME: element <{}> at {} is not closed", element.name(), element.lineno());
                    }
                }

                XmlEvent::Whitespace(_) |
                    XmlEvent::Characters(_) => {
                    self.parser.skip();
                },

                _ => panic!("FIXME: got {:?} instead of closing element <{}> at {}", xml_element.event, element.name(), element.lineno()),
            }
        }

/*
println!("{}3-subelements.len() for <{}>: {}", "   ".repeat(depth), element.name(), element.subelements().len());
print!("{}<{}>: end-element", "   ".repeat(depth), element.name());
for e in element.subelements() {
    print!(" {}", e.name());
}
println!();
*/
//println!("{}</{:?}>: start_element", "   ".repeat(depth), element.name());
        Ok(Box::new(element))
    }

    /*
     * We expect EndDocument, parsed as a lookahead
     */
    fn parse_end_document(&mut self) -> Result<(), XmlDocumentError> {
//println!("EndDocument");
        self.parser.skip();

//println!();
        loop {
            let xml_element = self.parser.next()?;
//println!("end_element: {:?}", xml_element.event);
            match xml_element.event {
                XmlEvent::Whitespace(_) |
                    XmlEvent::Characters(_) => {},

                XmlEvent::EndDocument => break,

                _ => panic!("FIXME: Expected end of document but found {:?}", xml_element.event)
            }
        }

        Ok(())
    }
}
