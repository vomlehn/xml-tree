/*
 * Take an ElementDefinition tree and generate an XmlFactorTree, which is used to parse
 * XML input
 */

use std::collections::HashMap;
use std::fmt;
use std::io::{Read};
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

/*
impl<'a> XmlDocumentFactoryDef<'a> {
    fn new(element_definition: &'a ElementDefinition) -> XmlDocumentFactoryDef<'a> {
        XmlDocumentFactoryDef {
            element_definition:   element_definition,
        }
    }
}
*/

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
        
        let parser = Parser::<T>::new(reader);

        let mut xml_factory = XmlDocumentFactory::<T> {
            parser:         parser,
            xml_definition: xml_definition,
            factory_defs:   HashMap::<&'a str, XmlDocumentFactoryDef<'a>>::new(),
        };
        let xml_document = xml_factory.parse_end_document(xml_definition)?;
        Ok(xml_document)
    }

    /*
     * Parse the StartDocument event.
     */
    fn parse_start_document<'b>(&mut self) ->
        Result<DocumentInfo, XmlDocumentError> {
        let mut comments_before = Vec::<XmlEvent>::new();

        let document_info = loop {
            let xml_element = self.parser.next()?;

            match &xml_element.event {
                XmlEvent::StartDocument{version, encoding, standalone} => {
                    let document_info = DocumentInfo::new(version.clone(),
                        encoding.clone(), standalone.clone());
                    break document_info;
                },
                XmlEvent::EndDocument => {
                    return Err(XmlDocumentError::NoEndDocument());
                },
                XmlEvent::Comment(cmnt) => {
                    comments_before.push(XmlEvent::Comment(cmnt.clone()));
                    continue;
                },
                XmlEvent::Whitespace(ws) => {
                    comments_before.push(XmlEvent::Whitespace(ws.clone()));
                    continue;
                },
                XmlEvent::Characters(characters) => {
                    comments_before.push(XmlEvent::Comment(characters.clone()));
                    continue;
                },
                XmlEvent::CData(cdata) => {
                    comments_before.push(XmlEvent::Comment(cdata.clone()));
                    continue;
                },
                _ => return Err(XmlDocumentError::UnexpectedXml(xml_element.event.clone())),
            }; 
        };

        return Ok(document_info)
    }

    /*
     * Parse until we find an EndDocument, filling in the 
     */
    fn parse_end_document(&'a mut self, xml_definition: &XmlDefinition) ->
        Result<XmlDocument, XmlDocumentError> {
        let mut pieces = Vec::<XmlEvent>::new();
        let document_info = self.parse_start_document()?;
        let start_name =
            OwnedName { local_name: "".to_string(), prefix: None, namespace: None };

        let root_element = loop {
            let xml_element = self.parser.next()?;
            let lineno = xml_element.lineno;

            match &xml_element.event {
                XmlEvent::StartDocument{..} => {
                    return Err(XmlDocumentError::StartAfterStart(lineno));
                },
                XmlEvent::EndDocument => {
                    return Err(XmlDocumentError::Unknown(0));
                },
                XmlEvent::StartElement{name, attributes, namespace} => {
                    let start_name = name.clone();
                    let depth = 0;
                    println!("depth {}: end StartElement1: start_name {}", depth, start_name.local_name);
                    let element_info = ElementInfo::new(lineno, attributes.clone(),
                        namespace.clone());
                    let mut element = self.process_element::<R>(xml_definition.root,
                        depth, start_name.clone(), element_info)?;
                    element.before_element = pieces;
                    break element;
                },
                XmlEvent::EndElement{name} => {
                    let end_name = name.clone();
println!("depth {}: EndElement: name {}", 0, end_name.local_name);
                    return Err(XmlDocumentError::MisplacedElementEnd(lineno,
                        start_name.local_name, end_name.local_name));
                },
                XmlEvent::Comment(cmnt) => {
                    pieces.push(XmlEvent::Comment(cmnt.clone()));
                    continue;
                },
                XmlEvent::Whitespace(ws) => {
                    pieces.push(XmlEvent::Comment(ws.clone()));
                    continue;
                },
                XmlEvent::Characters(characters) => {
                    pieces.push(XmlEvent::Comment(characters.clone()));
                    continue;
                },
                XmlEvent::CData(cdata) => {
                    pieces.push(XmlEvent::Comment(cdata.clone()));
                    continue;
                },
/*
                XmlEvent::ProcessingInstruction(processing_instruction, name, data) => {
println!("Skipping processing_instruction");
                    continue;
                },
*/
                _ => {
                    return Err(XmlDocumentError::UnexpectedXml(xml_element.event.clone()))
                },
            };
        };

         let xml_document = XmlDocument {
            document_info:  document_info,
            root:           root_element,
         };
         Ok(xml_document)
    }

    // Find an ElementDefinition whose name matches the given one
    fn find_subelement<'b> (&self,
        allowable_subelements: &'b[&'b ElementDefinition<'b>], name: &str) ->
        Option<&'b ElementDefinition<'b>> {
        let elem = allowable_subelements
            .iter()
            .find(move |&element_def| element_def.name == name);
        elem.copied()
    }

    /*
     * Parse the current element and subelements. The <StartElement> has
     * already been read, read up to, and including, the <EndElement>
     * element_definition_in:   Definition for this element
     * depth:                   Number of levels of element nesting
     * name_in:                 Name of the element
     * element_info_in:         Other information about the element
     */
    fn process_element<T: Read + 'a>(&mut self,
        element_definition_in: &ElementDefinition, depth: usize,
        name_in: OwnedName, element_info_in: ElementInfo) ->
        Result<Element, XmlDocumentError> {
        // First, we set up the element
        let mut pieces = Vec::<XmlEvent>::new();

        let mut element = Element::new(name_in.clone(), depth, element_info_in);

        // Parse any subelements
        let allowable_subelements = element_definition_in.allowable_subelements;

        loop {
            let xml_element = self.parser.next()?;
            let lineno = xml_element.lineno;

            match &xml_element.event {
                XmlEvent::StartDocument{..} => {
                    return Err(XmlDocumentError::StartAfterStart(lineno));
                },
                XmlEvent::EndDocument => {
                    return Err(XmlDocumentError::Unknown(0));
                },
                XmlEvent::StartElement{name, attributes, namespace} => {
                    // See if we support this element under the current element
                    let start_name = name.clone();
                    let attributes2 = attributes.clone();
                    let namespace2 = namespace.clone();

                    let element_definition =
                        match self.find_subelement(&allowable_subelements,
                            &start_name.local_name) {
                            None => return Err(XmlDocumentError::UnknownElement(lineno,
                                start_name.to_string())),
                            Some(el) => el,
                    };
                    
                    let element_info = ElementInfo::new(lineno,
                        attributes2.clone(), namespace2.clone());

                    let subelement = self.process_element::<R>(element_definition,
                        depth, start_name.clone(), element_info.clone())?;
                    element.before_element = pieces;
                    element.subelements.push(subelement);
                    pieces = Vec::<XmlEvent>::new();
                },
                XmlEvent::EndElement{name} => {
                    if name.local_name != element.name.local_name {
                        return Err(XmlDocumentError::MisplacedElementEnd(lineno,
                            element.name.local_name, name.local_name.to_string()));
                    }

                    element.content = pieces;
                    return Ok(element);
                },
                XmlEvent::Comment(cmnt) => {
                    pieces.push(XmlEvent::Comment(cmnt.clone()));
                },
                XmlEvent::Whitespace(ws) => {
                    pieces.push(XmlEvent::Whitespace(ws.clone()));
                },
                XmlEvent::Characters(characters) => {
                    pieces.push(XmlEvent::Characters(characters.clone()));
                },
                XmlEvent::CData(cdata) => {
                    pieces.push(XmlEvent::CData(cdata.clone()));
                },
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

impl<R: Read> fmt::Display for XmlDocumentFactory<'_, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.xml_definition.root.name)?;
        for factory_desc in self.factory_defs.values() {
            write!(f, "{}", factory_desc.element_definition.name)?;
        }
        Ok(())
    }
}
