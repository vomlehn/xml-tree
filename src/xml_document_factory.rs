/*
 * Take an ElementDefinition tree and generate an XmlFactorTree, which is used to parse
 * XML input
 */

use std::collections::HashMap;
use std::fmt;
use std::io::{Read};
use std::rc::Rc;
use xml::name::OwnedName;
use xml::reader::XmlEvent;

use crate::parser::Parser;
use crate::xml_definition::{XmlDefinition, ElementDefinition};
pub use crate::xml_document::{DocumentInfo, Element, ElementInfo, XmlDocument};
pub use crate::xml_document_error::XmlDocumentError;

#[derive(Debug)]
struct XmlDocumentFactoryDef<'a> {
    element_definition:   &'a ElementDefinition<'a>,
}

impl<'a> XmlDocumentFactoryDef<'a> {
    fn new(element_definition: &'a ElementDefinition) -> XmlDocumentFactoryDef<'a> {
        XmlDocumentFactoryDef {
            element_definition:   element_definition,
        }
    }
}

impl fmt::Display for XmlDocumentFactoryDef<'_> {
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
    parser:             Parser<R>,
    pub xml_definition: &'a XmlDefinition<'a>,
    factory_defs:       HashMap<&'a str, XmlDocumentFactoryDef<'a>>
}

impl<'a, R: Read + 'a> XmlDocumentFactory<'a, R> {
    pub fn new_from_reader<T: Read + 'a>(reader: T,
        xml_definition: &'a XmlDefinition<'a>) ->
        Result<XmlDocument, XmlDocumentError> {
        if xml_definition.element_definitions.is_empty() {
            return Err(XmlDocumentError::XmlNoElementDefined());
        }
        
        let parser = Parser::<T>::new(reader);

        let mut xml_factory = XmlDocumentFactory::<T> {
            parser:         parser,
            xml_definition: xml_definition,
            factory_defs:   HashMap::<&'a str, XmlDocumentFactoryDef<'a>>::new(),
        };
        let xml_document = xml_factory.parse_end_document(xml_definition.element_definitions)?;
        Ok(xml_document)
    }

    // Populate the HashMap with Elements
    fn populate<'b>(element_definitions: &'b [ElementDefinition<'b>]) ->
        Result<HashMap<&'b str, XmlDocumentFactoryDef<'b>>, XmlDocumentError> {
        let mut xml_factory_defs = HashMap::<&str, XmlDocumentFactoryDef>::new();

        for element_definition in element_definitions {
            let xml_factory_def = XmlDocumentFactoryDef::new(element_definition);
            if xml_factory_defs.insert(element_definition.name, xml_factory_def).is_some() {
                return Err(XmlDocumentError::CantInsertElement(element_definition.name.to_string()))
            }
        }

        Ok(xml_factory_defs)
    }

    /*
     * Parse the StartDocument event.
     */
    pub fn parse_start_document<'b>(&mut self) ->
        Result<DocumentInfo, XmlDocumentError> {
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
    pub fn parse_end_document(&'a mut self,
        element_definitions: &[ElementDefinition]) ->
        Result<XmlDocument, XmlDocumentError> {
        let xml_factory_defs = Self::populate(element_definitions)?;

        let document_info = self.parse_start_document()?;

        let mut elements = Vec::<Element>::new();

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
                    let element_info = ElementInfo::new(0, attributes, namespace);
                    let n = name.clone();
                    let depth = 0;
                    let subelement = self.parse_subelement::<R>(depth,
                        &xml_factory_defs, name, element_info)?;
println!("depth {}: push name {} on root", depth, subelement.name.local_name);
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

        if elements.len() != 1 {
            return Err(XmlDocumentError::OnlyOneRootElementAllowed());
        }

        let root = match elements.first() {
            None => return Err(XmlDocumentError::OnlyOneRootElementAllowed()),
            Some(r) => r,
        };
//Rc::new()?
         let xml_document = XmlDocument {
            document_info:  document_info,
            root:           root.clone(),
         };
         Ok(xml_document)
    }

    /*
     * Parse a subelement, which may itself have subelements
     * depth:   Number of levels of element nesting
     * name:    Name of the subelement
     */
    pub fn parse_subelement<T: Read + 'a>(&mut self, depth: usize,
        xml_factory_defs: &HashMap<&str, XmlDocumentFactoryDef>, name: OwnedName,
        element_info: ElementInfo) ->
        Result<Element, XmlDocumentError> {
        let mut element = Element::new(name.clone(), 1, element_info.clone());
        let start_name = name.local_name.clone();

        // Make sure this element is allowed where it is
        let pos = match self
            .xml_definition.element_definitions.iter()
            .position(|x| x.name == start_name) {
            None => return Err(XmlDocumentError::UnknownElement(element_info.
                lineno, start_name)),
            Some(p) => p,
        };

        let new_desc = &self.xml_definition.element_definitions[pos];

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
                    let element_info = ElementInfo::new(0, attributes, namespace);
                    let n = name.clone();
                    let subelement = self.parse_subelement::<T>(depth + 1,
                        xml_factory_defs, name, element_info)?;
println!("depth {}: push name {} on {}", depth + 1, subelement.name.local_name, element.name.local_name);
                    element.subelements.push(subelement);
                },
                XmlEvent::EndElement{name} => {
                    let local_name = name.local_name.to_string();

                    if name.local_name.clone() != new_desc.name {
                        return Err(XmlDocumentError::MisplacedElementEnd(lineno,
                            local_name.clone()));
                    }

                    return Ok(element);
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
    }

    /*
     * Look up the root name and get a reference to it in the list of Elements
     * self     Reference to the XmlDocumentFactory
     * xml_document:    The struct in which parsing results are placed. This
     *                  must have a Vec::<Element> in which the list of parsed
     *                  Elements have been placed.
     */
/*
    pub fn get_root<'b>(mut self) ->
        Result<Element, XmlDocumentError> {
        let start_name = self.xml_definition.root_name;

        let element_pos = match self.elements.iter().
            position(|element| element.name.local_name.as_str() == start_name) {
            None => return Err(XmlDocumentError::UnknownElement(0, start_name.to_string())),
            Some(e) => e,
        };

        let root_ref = &elements[element_pos];
        Ok(root_ref)
    }
*/
/*
     pub fn get_root<'b>(elements: &'b [Element], root_name: &str) ->
        Result<&'b Element, XmlDocumentError> {
        // Find the root element based on the XML definition's root name

        elements
            .iter()
            .find(|element| element.name.local_name == root_name)
            .ok_or_else(|| XmlDocumentError::UnknownElement(0, root_name.to_string()))
    }
*/
/*
    pub fn find_element<'a>(&self, name: &str) -> Option<&Element<'a> {
        let element = match (self.elements)
            .iter()
            .find(|element| element.name.local_name == root_name) {
            None => return Err(XmlDocumentError::UnknownElement(0, root_name.to_string())),
            Some(r) => Rc::new(r),
        };
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
