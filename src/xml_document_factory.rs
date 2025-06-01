/*
 * Take an Element tree and generate an XmlFactorTree, which is used
 * to parse XML input
 */
// FIXME: delete all uses of expect(), everywhere

//use std::borrow::Borrow;
//use std::cell::RefCell;
use std::io::Read;
use xml::name::OwnedName;
use xml::reader::XmlEvent;

use crate::parser::Parser;
//use crate::walk_and_print::PrintBaseLevel;
pub use crate::xml_document::{DirectElement, DocumentInfo, Element, ElementInfo, XmlDocument};
pub use crate::xml_document_error::XmlDocumentError;
//use crate::xml_schema::{Element, XmlSchema};
use crate::xml_schema::XmlSchema;

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

        let xml_factory = XmlDocumentFactory::<T> {
            parser: parser,
            xml_schema: xml_schema,
        };

        xml_factory.parse_end_document()
    }

    /*
     * Parse the StartDocument event.
     */
    fn parse_start_document(&mut self) -> Result<DocumentInfo, XmlDocumentError> {
        let mut comments_before = Vec::<XmlEvent>::new();

        let document_info = loop {
            let xml_element = self.parser.next()?;

            match &xml_element.event {
                XmlEvent::StartDocument {
                    version,
                    encoding,
                    standalone,
                } => {
                    let document_info =
                        DocumentInfo::new(version.clone(), encoding.clone(), standalone.clone());
                    break document_info;
                }
                XmlEvent::EndDocument => {
                    return Err(XmlDocumentError::NoEndDocument());
                }
                XmlEvent::Comment(cmnt) => {
                    comments_before.push(XmlEvent::Comment(cmnt.clone()));
                    continue;
                }
                XmlEvent::Whitespace(ws) => {
                    comments_before.push(XmlEvent::Whitespace(ws.clone()));
                    continue;
                }
                XmlEvent::Characters(characters) => {
                    comments_before.push(XmlEvent::Comment(characters.clone()));
                    continue;
                }
                XmlEvent::CData(cdata) => {
                    comments_before.push(XmlEvent::Comment(cdata.clone()));
                    continue;
                }
                _ => return Err(XmlDocumentError::UnexpectedXml(xml_element.event.clone())),
            };
        };

        return Ok(document_info);
    }

    /*
     * Parse until we find an EndDocument, filling in the
     */
    fn parse_end_document(mut self) -> Result<XmlDocument, XmlDocumentError> {
        let mut pieces = Vec::<XmlEvent>::new();
        let document_info = self.parse_start_document()?;
        let start_name = OwnedName {
            local_name: "".to_string(),
            prefix: None,
            namespace: None,
        };

        let start_element = loop {
            // Read the next element
            let xml_element = self.parser.next()?;
            let lineno = xml_element.lineno;

            match &xml_element.event {
                XmlEvent::StartDocument { .. } => {
                    return Err(XmlDocumentError::StartAfterStart(lineno));
                },

                XmlEvent::EndDocument => {
                    return Err(XmlDocumentError::Unknown(0));
                },

                XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                } => {
                    let start_name = name.clone();
                    let element_info =
                        ElementInfo::new(lineno, attributes.clone(), namespace.clone());

                    // Parse all of the items contained within this element
//                    let root_element = self.xml_schema.inner.xml_document.root;
                    let element = self.parse_element::<R>(
                        &self.xml_schema.inner.xml_document.root,
                        start_name.clone(),
                        element_info,
                        pieces,
                    )?;
// FIXME: in theory, parse_element does this, but I think it doesn't
//                    element.before_element = pieces;

                    // Get out of here so we can move on to the next element.
                    break Box::new(element);
                },

                XmlEvent::EndElement { name } => {
                    // This EndElement was not proceeded by a StartElement,
                    // oops!
                    let end_name = name.clone();
                    return Err(XmlDocumentError::MisplacedElementEnd(
                        lineno,
                        start_name.local_name,
                        end_name.local_name,
                    ));
                }

                XmlEvent::Comment(cmnt) => {
                    pieces.push(XmlEvent::Comment(cmnt.clone()));
                    continue;
                }

                XmlEvent::Whitespace(ws) => {
                    pieces.push(XmlEvent::Comment(ws.clone()));
                    continue;
                }

                XmlEvent::Characters(characters) => {
                    pieces.push(XmlEvent::Comment(characters.clone()));
                    continue;
                }

                XmlEvent::CData(cdata) => {
                    pieces.push(XmlEvent::Comment(cdata.clone()));
                    continue;
                }
                /*
                                XmlEvent::ProcessingInstruction(processing_instruction, name, data) => {
                println!("Skipping processing_instruction");
                                    continue;
                                },
                */

                _ => return Err(XmlDocumentError::UnexpectedXml(xml_element.event.clone())),
            };
        };

        let xml_document = XmlDocument {
            document_info:  document_info,
            root:           start_element,
        };

        Ok(xml_document)
    }

    /*
     * Parse the current element and subelements. The <StartElement> has
     * already been read, read up to, and including, the <EndElement>
     * element_in:   Definition for this element
//     * depth:                   Number of levels of element nesting
     * name_in:                 Name of the element
     * element_info_in:         Other information about the element
     *
     * This only produces DirectElements
     */
    fn parse_element<T: Read>(
        &mut self,
        // FIXME: remove if uneeded
        _element: &Box<dyn Element>,
//        depth: usize,
        name_in: OwnedName,
        element_info_in: ElementInfo,
        // FIXME: remove if uneeded
        _pieces: Vec::<XmlEvent>,
    ) -> Result<DirectElement, XmlDocumentError> {
        
        // First, we set up the element
        let mut pieces = Vec::new();
        let mut element = DirectElement::new(name_in.clone(), element_info_in.clone());
        element.before_element = Vec::new();

        loop {
            let xml_element = {
                let x = self.parser.next()?;
                x.clone()
            };
            let lineno = xml_element.lineno;

            match &xml_element.event {
                XmlEvent::StartDocument { .. } => {
                    return Err(XmlDocumentError::StartAfterStart(lineno));
                }
                XmlEvent::EndDocument => {
                    return Err(XmlDocumentError::Unknown(0));
                }
                XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                } => {
                    // See if we support this element under the current element
                    let start_name = name.clone();
                    let attributes2 = attributes.clone();
                    let namespace2 = namespace.clone();

                    let next_element =
                        match element.get(start_name.local_name.as_str()) {
                            None => {
                                return Err(XmlDocumentError::UnknownElement(
                                    lineno,
                                    start_name.local_name.to_string(),
                                ))
                            }
                            Some(elem) => elem,
                        };

                    let element_info =
                        ElementInfo::new(lineno, attributes2.clone(), namespace2.clone());
                    let subelement = self.parse_element::<R>(
                        &next_element,
//                        depth,
                        start_name.clone(),
                        element_info.clone(),
                        pieces,
                    )?;
//                    element.before_element = pieces;
//let x: u8 = subelement;
                    element.subelements.push(Box::new(subelement));
                    pieces = Vec::<XmlEvent>::new();
                }
                XmlEvent::EndElement { name } => {
                    if name.local_name != *element.name() {
                        return Err(XmlDocumentError::MisplacedElementEnd(
                            lineno,
                            element.name().to_string(),
                            name.local_name.to_string(),
                        ));
                    }

                    element.content = pieces;
                    // FIXME: is this right or should it be an error?
                    return Ok(*Box::new(element));
                }
                XmlEvent::Comment(cmnt) => {
                    pieces.push(XmlEvent::Comment(cmnt.clone()));
                }
                XmlEvent::Whitespace(ws) => {
                    pieces.push(XmlEvent::Whitespace(ws.clone()));
                }
                XmlEvent::Characters(characters) => {
                    pieces.push(XmlEvent::Characters(characters.clone()));
                }
                XmlEvent::CData(cdata) => {
                    pieces.push(XmlEvent::CData(cdata.clone()));
                }
                /*
                                XmlEvent::ProcessingInstruction(processing_instruction, name, data) => {
                println!("Skipping processing_instruction");
                                },
                */
                _ => {
                    return Err(XmlDocumentError::UnexpectedXml(xml_element.event.clone()));
                }
            };
        }
    }
}

/*
impl<R: Read> fmt::Display for XmlDocumentFactory<'_, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.xml_schema)
    }
}
*/
