/*
 * Take an SchemaElement tree and generate an XmlFactorTree, which is used
 * to parse XML input
 */
// FIXME: delete all uses of expect(), everywhere

use std::fmt;
use std::io::{Read};
use std::sync::{Arc, Mutex};
use xml::name::OwnedName;
use xml::reader::XmlEvent;


use crate::parser::Parser;
use crate::xml_schema::{XmlSchema, SchemaElement};
pub use crate::xml_document::{DocumentInfo, Element, ElementInfo, XmlDocument};
pub use crate::xml_document_error::XmlDocumentError;

/*
 * Structure used to hold parsing information
 * parser:          Used to extract XmlElement objects from the input stream
 * xml_schema:  Definition of what the input is expected to look like
 */
pub struct XmlDocumentFactory<'a, R: Read + 'a> {
    parser:             Parser<R>,
    pub xml_schema:     XmlSchema<'a>,
}

impl<'a, R: Read + 'a> XmlDocumentFactory<'_, R> {
    pub fn new_from_reader<T: Read + 'a>(reader: T,
        xml_schema: XmlSchema<'a>) ->
        Result<XmlDocument, XmlDocumentError> {
        
        let parser = Parser::<T>::new(reader);

        let xml_factory = XmlDocumentFactory::<T> {
            parser:         parser,
            xml_schema:     xml_schema,
        };

        xml_factory.parse_end_document()
    }

    /*
     * Parse the StartDocument event.
     */
    fn parse_start_document(&mut self) ->
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
    fn parse_end_document(mut self) ->
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
                    let element_info = ElementInfo::new(lineno, attributes.clone(),
                        namespace.clone());
                    
                    let mut element = self.parse_element::<R>(&self.xml_schema.element, depth,
                        start_name.clone(), element_info)?;
                    element.before_element = pieces;
                    break element;
                },
                XmlEvent::EndElement{name} => {
                    let end_name = name.clone();
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

    /*
     * Parse the current element and subelements. The <StartElement> has
     * already been read, read up to, and including, the <EndElement>
     * schema_element_in:   Definition for this element
     * depth:                   Number of levels of element nesting
     * name_in:                 Name of the element
     * element_info_in:         Other information about the element
     */
    fn parse_element<T: Read>(&mut self,
        schema_element: &Arc<Mutex<dyn SchemaElement + Sync + 'static>>,
            depth: usize, name_in: OwnedName, element_info_in: ElementInfo) ->
        Result<Element, XmlDocumentError> {
        // First, we set up the element
        let mut pieces = Vec::<XmlEvent>::new();

        println!("<{}> ({:?}: {})", name_in.local_name, schema_element.name(), element_info_in.lineno);
        let mut element = Element::new(name_in.clone(), depth, element_info_in.clone());

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

                    let next_schema_element =
                        match schema_element.get(start_name.local_name.as_str()) {
                            None => return Err(XmlDocumentError::UnknownElement(lineno,
                                start_name.local_name.to_string())),
                            Some(elem) => elem,
                    };
                    
                    let element_info = ElementInfo::new(lineno,
                        attributes2.clone(), namespace2.clone());
                    let subelement = self.parse_element::<R>(
                        next_schema_element, depth,
                        start_name.clone(), element_info.clone())?;
                    element.before_element = pieces;
                    element.subelements.push(subelement);
                    pieces = Vec::<XmlEvent>::new();
                },
                XmlEvent::EndElement{name} => {
        println!("</{}> ({:?}: {}->{})", name.local_name, schema_element.name(), element_info_in.lineno, xml_element.lineno);
                    if name.local_name != schema_element.name() {
                        return Err(XmlDocumentError::MisplacedElementEnd(lineno,
                            schema_element.name().clone(), name.local_name.to_string()));
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
        write!(f, "{}", self.xml_schema)
    }
}
