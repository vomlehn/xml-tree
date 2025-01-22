/*
 * Take an ElementDefinition tree and generate an XmlFactorTree, which is used to parse
 * XML input
 */

use std::collections::HashMap;
use std::fmt;
use std::io::{BufReader, Read};
/*
use xml::attribute::OwnedAttribute;
use xml::common::XmlVersion;
*/
use xml::name::OwnedName;
/*
use xml::namespace::Namespace;
*/
use xml::reader::XmlEvent;

use crate::parser::Parser;
use crate::xml_definition::{XmlDefinition, ElementDefinition};
pub use crate::xml_document::{DocumentInfo, Element, ElementInfo};
pub use crate::xml_document_error::XmlDocumentError;

#[derive(Debug)]
struct XmlDocumentFactoryDesc<'a> {
    element_definition:   &'a ElementDefinition<'a>,
}

impl<'a> XmlDocumentFactoryDesc<'a> {
    fn new(element_definition: &'a ElementDefinition) -> XmlDocumentFactoryDesc<'a> {
        XmlDocumentFactoryDesc {
            element_definition:   element_definition,
        }
    }
}

impl fmt::Display for XmlDocumentFactoryDesc<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.element_definition.name)
    }
}

pub struct XmlDocumentFactory<'a, R: Read> {
    parser:         Parser<R>,
    xml_definition:  &'a XmlDefinition<'a>,
    tree:           HashMap<&'a str, XmlDocumentFactoryDesc<'a>>
}

impl<'a, R: Read + 'a> XmlDocumentFactory<'a, R> {
    pub fn new_from_reader(reader: BufReader<R>,
        xml_definition: &'a XmlDefinition<'a>) ->
        Result<XmlDocumentFactory<'a, R>, XmlDocumentError> {
        if xml_definition.element_definitions.is_empty() {
            return Err(XmlDocumentError::XmlNoElementDefined());
        }
        
        let parser = Parser::<R>::new(reader);

        let mut xml_factory = XmlDocumentFactory::<R> {
            parser:         parser,
            xml_definition:  xml_definition,
            tree:           HashMap::<&'a str, XmlDocumentFactoryDesc<'a>>::new(),
        };
        xml_factory.populate();

        Ok(xml_factory)
    }

    // Populate the HashMap with Elements
    fn populate<'b>(&mut self) -> Result<(), XmlDocumentError> {

        for element_definition in self.xml_definition.element_definitions {
            let xml_factory_desc = XmlDocumentFactoryDesc::new(element_definition);
            if self.tree.insert(element_definition.name, xml_factory_desc).is_none() {
                return Err(XmlDocumentError::CantInsertElement(element_definition.name.to_string()))
            }
        }

        Ok(())
    }

    /*
     * Parse the StartDocument event.
     */
    pub fn parse_start_document(&mut self) -> Result<DocumentInfo, XmlDocumentError> {
        let mut comments_before = Vec::<String>::new();

        let document_info = loop {
            let xml_element = self.parser.next();

            match xml_element {
                Err(e) => return Err(XmlDocumentError::XmlError(0, Box::new(e))),
                Ok(evt) => {
                    match evt.event {
                        XmlEvent::StartDocument{version, encoding, standalone} => {
                            break DocumentInfo::new(version, encoding, standalone);
                        },
                        XmlEvent::EndDocument => {
                            return Err(XmlDocumentError::NoEndDocument());
                        },
                        XmlEvent::Comment(cmnt) => {
                            comments_before.push(cmnt);
                            continue;
                        },
                        XmlEvent::Whitespace(_ws) => {
                            continue;
                        },
                        _ => return Err(XmlDocumentError::UnexpectedXml(evt.event))
                    }
                }
            };
        };

        Ok(document_info)
    }

    /*
     * Parse until we find an EndDocument, filling in the 
     */
    pub fn parse_end_document(&mut self) -> Result<Vec<Element>, XmlDocumentError> {
        let tree = Vec::<Element>::new();
        let mut start_name = "".to_string();

        loop {
            let xml_element = self.parser.next();

            match xml_element {
                Err(e) => {
                    return Err(XmlDocumentError::XmlError(0, Box::new(e))); // FIXME: line number
                },
                Ok(evt) => {
                    let lineno = evt.lineno;

                    match evt.event {
                        XmlEvent::StartDocument{..} => {
                            return Err(XmlDocumentError::StartAfterStart(lineno));
                        },
                        XmlEvent::EndDocument => {
                            return Err(XmlDocumentError::Unknown(0));
                        },
                        XmlEvent::StartElement{name, attributes, namespace} => {
                            let element_info = ElementInfo::new(lineno,
                                attributes, namespace);
                            let subelement = self.parse_subelement(0,
                                name, element_info)?;
                            tree.push(subelement);
                            break;
                        }
                        XmlEvent::EndElement{name} => {
                            return Err(XmlDocumentError::MisplacedElementEnd(lineno,
                                name.local_name));
                        },
                        XmlEvent::Comment(_cmnt) => {
//                            comments_before.push(cmnt);
                            continue;
                        },
                        XmlEvent::Whitespace(_ws) => {
                            continue;
                        },
                        XmlEvent::Characters(_characters) => {
                            continue;
                        },
                        XmlEvent::CData(_cdata) => {
                            continue;
                        },
/*
                        XmlEvent::ProcessingInstruction(processing_instruction) => {
println!("Skipping processing_instruction");
                            continue;
                        },
*/
                        _ => return Err(XmlDocumentError::UnexpectedXml(evt.event))
                    }
                }
            }
        }

        return Ok(tree)
    }

    // Parse a subelement, which may itself have su elements
    pub fn parse_subelement<S: Read>(&self, depth: usize,
        name: OwnedName, element_info: ElementInfo) ->
        Result<Element, XmlDocumentError> {
        let start_name = name.local_name.clone();

        // Make sure this element is allowed where it is
        let pos = match self.xml_definition.element_definitions.iter().position(|x| x.name == start_name) {
            None => return Err(XmlDocumentError::UnknownElement(element_info.lineno,
                start_name)),
            Some(p) => p,
        };

        let new_desc = &self.xml_definition.element_definitions[pos];
        let mut subelements = Vec::<Element>::new();

        loop {
            let xml_element = self.parser.next();

            match xml_element {
                Err(e) => {
                    return Err(XmlDocumentError::XmlError(0, Box::new(e))); // FIXME: line number
                },
                Ok(evt) => {
                    let lineno = evt.lineno;

                    match evt.event {
                        XmlEvent::StartDocument{..} => {
                            return Err(XmlDocumentError::StartAfterStart(lineno));
                        },
                        XmlEvent::EndDocument => {
                            return Err(XmlDocumentError::Unknown(0));
                        },
                        XmlEvent::StartElement{name, attributes, namespace} => {
                            let element_info = ElementInfo::new(lineno,
                                attributes, namespace);
                            let subelement = self.parse_subelement(depth + 1,
                                name, element_info)?;
                            subelement.insert(name.local_name, subelement);
                        }
                        XmlEvent::EndElement{name} => {
                            if name.local_name != new_desc.name {
                                return Err(XmlDocumentError::MisplacedElementEnd(lineno,
                                    name.local_name));
                            }

                            let mut element = Element::new(name, depth, element_info);
                            element.subelements = subelements;
                            return Ok(element)
                        },
                        XmlEvent::Comment(_cmnt) => {
//                            comments_before.push(cmnt);
                            continue;
                        },
                        XmlEvent::Whitespace(_ws) => {
                            continue;
                        },
                        XmlEvent::Characters(_characters) => {
                            continue;
                        },
                        XmlEvent::CData(_cdata) => {
                            continue;
                        },
/*
                        XmlEvent::ProcessingInstruction(processing_instruction) => {
println!("Skipping processing_instruction");
                            continue;
                        },
*/
                        _ => {
                            return Err(XmlDocumentError::UnexpectedXml(evt.event));
                        }
                    }
                }
            };
        }
    }

    pub fn get_root(&self) -> &str {
        self.xml_definition.root_name
    }
}

impl<R: Read> fmt::Display for XmlDocumentFactory<'_, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.xml_definition.root)?;
        for desc in self.tree.values() {
            write!(f, "{}", desc.element_definition.name)?;
        }
        Ok(())
    }
}
