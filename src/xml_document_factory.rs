/*
 * Take an ElementDefinition tree and generate an XmlFactorTree, which is used to parse
 * XML input
 */

use std::collections::HashMap;
use std::fmt;
use std::io::{Read};
use xml::name::OwnedName;
use xml::namespace::{Namespace};
use xml::reader::XmlEvent;

use crate::parser::Parser;
use crate::xml_definition::{XmlDefinition, ElementDefinition};
pub use crate::xml_document::{DocumentInfo, Element, ElementInfo, XmlDocument};
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

/*
 * Structure used to hold parsing information
 * parser:          Used to extract XmlElement objects from the input stream
 * xml_definition:  Definition of what the input is expected to look like
 * factory_defs     Hash table that makes it faster to get from an element
 *                  name to the corresponding entry in xml_definition.
 */
pub struct XmlDocumentFactory<'a, R: Read> {
    parser:         Parser<R>,
    xml_definition: &'a XmlDefinition<'a>,
    factory_defs:   HashMap<&'a str, XmlDocumentFactoryDesc<'a>>
}

impl<'a, R: Read + 'a> XmlDocumentFactory<'a, R> {
    pub fn new_from_reader<T: Read>(reader: T,
        xml_definition: &'a XmlDefinition<'a>) ->
        Result<XmlDocumentFactory<'a, T>, XmlDocumentError> {
        if xml_definition.element_definitions.is_empty() {
            return Err(XmlDocumentError::XmlNoElementDefined());
        }
        
        let parser = Parser::<T>::new(reader);

        let mut xml_factory = XmlDocumentFactory::<T> {
            parser:         parser,
            xml_definition: xml_definition,
            factory_defs:   HashMap::<&'a str, XmlDocumentFactoryDesc<'a>>::new(),
        };
//        xml_factory.populate::<T>();

        Ok(xml_factory)
    }

/*
    // Populate the HashMap with Elements
    fn populate<'b, U>(&mut self) -> Result<(), XmlDocumentError> {

        for element_definition in self.xml_definition.element_definitions {
            let xml_factory_desc = XmlDocumentFactoryDesc::new(element_definition);
            if self.factory_defs.insert(element_definition.name, xml_factory_desc).is_none() {
                return Err(XmlDocumentError::CantInsertElement(element_definition.name.to_string()))
            }
        }

        Ok(())
    }
*/

    /*
     * Parse the StartDocument event.
     */
    pub fn parse_start_document(&mut self) -> Result<DocumentInfo, XmlDocumentError> {
        let document_info = DocumentInfo::new(xml::common::XmlVersion::Version10, "tbd".to_string(), None);
        let mut comments_before = Vec::<String>::new();

        let document_info = loop {
            let xml_element = self.parser.next()?;

            match xml_element.event {
                XmlEvent::StartDocument{version, encoding, standalone} => {
                    let document_info = DocumentInfo::new(version,
                        encoding, standalone);
                    break document_info;
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
                _ => return Err(XmlDocumentError::UnexpectedXml(xml_element.event)),
            }; 
        };

        return Ok(document_info)
    }

    /*
     * Parse until we find an EndDocument, filling in the 
     */
    pub fn parse_end_document(&mut self) -> Result<Vec<Element>, XmlDocumentError> {
        let mut elements = Vec::<Element>::new();
        let mut start_name = "".to_string();

        loop {
            let xml_element = self.parser.next()?;
            let lineno = xml_element.lineno;

            match xml_element.event {
                XmlEvent::StartDocument{..} => {
                    return Err(XmlDocumentError::StartAfterStart(lineno));
                },
                XmlEvent::EndDocument => {
                    return Err(XmlDocumentError::Unknown(0));
                },
                XmlEvent::StartElement{name, attributes,
                    namespace} => {
                    let element_info = ElementInfo::new(lineno,
                        attributes, namespace);
                    let subelement = self.parse_subelement::<R>(0,
                        name, element_info)?;
                    elements.push(subelement);
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
                _ => return Err(XmlDocumentError::UnexpectedXml(xml_element.event))
            };
        }

        return Ok(elements)
    }

    // Parse a subelement, which may itself have subelements
    pub fn parse_subelement<T: Read>(&self, depth: usize,
        name: OwnedName, element_info: ElementInfo) ->
        Result<Element, XmlDocumentError> {
        let element_info = ElementInfo::new(0, Vec::<_>::new(), Namespace::empty() );
        let name = OwnedName{ namespace: None, local_name: "xyz".to_string(), prefix: None};
        let element = Element::new(name, 1, element_info);
        return Ok(element);
/*
        let start_name = name.local_name.clone();

        // Make sure this element is allowed where it is
        let pos = match self.xml_definition.element_definitions.iter().
            position(|x| x.name == start_name) {
            None => return Err(XmlDocumentError::UnknownElement(element_info.
                lineno, start_name)),
            Some(p) => p,
        };

        let new_desc = &self.xml_definition.element_definitions[pos];
        let mut subelements = Vec::<Element>::new();

        loop {
            let xml_element = self.parser.next()?;
            let lineno = xml_element.lineno;

            match xml_element.event {
                XmlEvent::StartDocument{..} => {
                    return Err(XmlDocumentError::StartAfterStart(lineno));
                },
                XmlEvent::EndDocument => {
                    return Err(XmlDocumentError::Unknown(0));
                },
                XmlEvent::StartElement{name, attributes, namespace} => {
                    let element_info = ElementInfo::new(lineno,
                        attributes, namespace);
                    let subelement = self.parse_subelement::<T>(depth + 1,
                        name, element_info)?;
                    subelements.push(subelement);
                }
                XmlEvent::EndElement{name} => {
                    if name.local_name != new_desc.name {
                        return Err(XmlDocumentError::MisplacedElementEnd(lineno,
                            name.local_name));
                    }

                    let element = Element::new(name, depth, element_info);
                    element.subelements.push(&name.local_name);
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
                    return Err(XmlDocumentError::UnexpectedXml(xml_element.event));
                }
            };
        }
*/
    }

/*
    /*
     * Look up the root name and get a reference to it in the list of Elements
     * self     Reference to the XmlDocumentFactory
     * xml_document:    The struct in which parsing results are placed. This
     *                  must have a Vec::<Element> in which the list of parsed
     *                  Elements have been placed.
     */
    pub fn set_root<'b>(&'b self, xml_document: &'b mut XmlDocument<'b>) ->
        Result<&'b Element, XmlDocumentError> {
        let start_name = self.xml_definition.root_name;

        let element_pos = match xml_document.elements.iter().
            position(|element| element.name.local_name.as_str() == start_name) {
            None => return Err(XmlDocumentError::UnknownElement(0, start_name.to_string())),
            Some(e) => e,
        };

        let root_ref = &xml_document.elements[element_pos];
        xml_document.root = Some(root_ref);
        Ok(xml_document.root.clone().unwrap())
    }
*/
}

impl<R: Read> fmt::Display for XmlDocumentFactory<'_, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.xml_definition.root_name)?;
        for factory_desc in self.factory_defs.values() {
            write!(f, "{}", factory_desc.element_definition.name)?;
        }
        Ok(())
    }
}
