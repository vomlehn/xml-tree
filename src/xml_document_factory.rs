// FIXME: probably consolidate this with xml_document.rs
/*
 * Take an Element tree and generate an XmlFactorTree, which is used
 * to parse XML input
 */
// FIXME: delete all uses of expect(), everywhere

//use std::borrow::Borrow;
//use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::Read;
use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::reader::XmlEvent;
use xml::namespace::Namespace;

use crate::parser::Parser;
//use crate::walk_and_print::PrintBaseLevel;
pub use crate::xml_document::{DirectElement, DocumentInfo, Element, ElementInfo, XmlDocument};
pub use crate::xml_document_error::{warning, XmlDocumentError};
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

        let document_info = xml_factory.parse_start_document()?;
        let elements = xml_factory.parse_elements("XML root element")?;

        // Let's look for the end of the document
        loop {
            // FIXME: this isn't really right
            let next_item = xml_factory.parser.next();
            match next_item.unwrap().event {
                XmlEvent::EndDocument => {
                    break;
                }

                _ => panic!("FIXME: didn't find EndDocument when expected"),
            }
        }

        // FIXME: add errors to XmlDocumentError
        match elements.len() {
            0 => panic!("No XML elements in document"),
            1 => {},
            _ => panic!("Exactly one XML element allowed in document"),
        }

        Ok(XmlDocument {
            document_info:  document_info,
            root:           elements,
        })
    }

    /*
     * Search for, and parse, the StartDocument event.
     *
     * Returns:
     * Ok(DocumentInfo)
     * Err(XmlDocumentError)
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
/*
                    let document_info = self.parse_start_document()?;
*/
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
     * Collect mutiple StartElements at a given level.  This means parsing
     * a bunch of pieces (Comment, Whitespace, Characters...) until we find
     * a StartElement, then handling that Element.
     *
     * Returns:
     * Ok(Vec<Box<dyn Element>>>)   An XmlDocument
     * Err(XmlDocumentError)
     */
    fn parse_elements(&mut self, parent_name: &str) -> Result<Vec<Box<dyn Element>>, XmlDocumentError> {
        let mut elements = Vec::<Box<dyn Element>>::new();

        loop {
            let before_pieces = self.parse_pieces()?;
            let xml_element = self.parser.next()?;
            let lineno = xml_element.lineno;

            match &xml_element.event {
                XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                } => {
                    let start_name = name;
                    println!("parse_element: got {:?} for start element", xml_element.event);
                    // FIXME: is this the right now
                    let (before_pieces, end_name, subelements) = self.parse_element(&start_name.local_name)?;

                    // FIXME: use namespace and prefix
                    if end_name.local_name != start_name.local_name {
                        // FIXME: use XmlDocumentError
                        panic!("Closing element name {:?} != opening element name {:?}",
                            end_name, start_name);
                    }

                    let after_pieces = self.parse_pieces()?;

                    let element_info =
                        ElementInfo::new(lineno, attributes.clone(), namespace.clone());

                    // FIXME: look for content and after_element?
                    let element = DirectElement::new(start_name.clone(), element_info,
                        before_pieces, vec!(), after_pieces, subelements);
                    elements.push(Box::new(element));
                },

                XmlEvent::ProcessingInstruction {
    //                processing_instruction,
                    name,
                    data
                } => {
                    self.parser.skip();
                    panic!("FIXME: stop skipping processing_instruction");
                },

                // Anything else means that we don't have any more elements at this
                // level.
                _ => {
                    self.parser.skip();
                    println!("parse_element: got unexpected {:?}", xml_element.event);
                    todo!();
    //                Ok(None)
    //                return XmlDocumentError::UnexpectedXml(xml_element.event.clone())
                },
            }
        }

        Ok(elements)
    }

    /*
     * Parse an EndElement corresponding to an already seen StartElement. There may be
     * a number of things to parse before the EndElement is seen, including
     * sub-StartElements.
     *
     * self:        XmlDocumentFactor
     * parent_name: Name of the element enclosing this element
     *
     * Returns:
     * Ok<(Vec<XmlEvent>, OwnedName)>   Name from the EndElement, any an preceeding
     *                                  pieces
     * Err<XmlDocumentError>            An error was found
     */
    fn parse_element(&mut self, parent_name: &str) -> Result<(Vec<XmlEvent>, OwnedName, Vec<Box<dyn Element>>), XmlDocumentError> {
        let mut subelements = Vec::new();

        loop {
            let before_pieces = self.parse_pieces()?;
            let xml_element = self.parser.lookahead()?;

            match &xml_element.event {
                // FIXME: supply names
                XmlEvent::StartDocument {
                    version,
                    encoding,
                    standalone,
                } => {
//                    Vec<Box<dyn Element>>
//                let start_name = name;
//                // FIXME: anything better for the parse_elements() argument?
                    let subelements = self.parse_elements("XML document root");
                },

                XmlEvent::EndDocument => {
                    self.parser.skip();
//                // FIXME: anything better for the parse_elements() argument?
                    let owned_name = OwnedName {local_name: "FIXME".to_string(),
                        namespace: None, prefix: None};
                    return Ok((before_pieces, owned_name, subelements));
                },

                // FIXME: fix the arguments
                XmlEvent::StartElement {name, ..} => {
                    let start_name = name.clone();
                    let subelements = self.parse_elements(&start_name.local_name)?;

                    // Collect the various incidental pieces
                    let after_pieces = self.parse_pieces()?;
                    // FIXME: think about this
                    if after_pieces.len() != 0 {
                        panic!("pieces after parsing element");
                    }

                    // Get the next XmlEvent, which *should* be an EndElement
                    let direct_element = self.parser.next()?;
                    match direct_element.event {
                        XmlEvent::EndElement { name } => {},
                        _ => {
                            // FIXME: get this right
                            panic!("Missing XmlEvent::EndElement");
                        },
                    }
                    
                    // Verify that the name given in the start element and the name from
                    // the end element match.
                    // FIXME: doesn't seem quite right
                    let end_name = match xml_element.event.clone() {
                        XmlEvent::EndElement {name} => name,
                        _ => panic!("FIXME: figure this out"),
                    };
                },

                XmlEvent::EndElement { name } => {
                    // Leave the EndElement in lookahead so that the StarElement code can
                    // parse it
                },

                XmlEvent::ProcessingInstruction {
//                processing_instruction,
                    name,
                    data
                } => {
                    self.parser.skip();
                    panic!("FIXME: stop skipping processing_instruction");
                },

                // Anything else means that we don't have any more elements at this
                // level.
                _ => {
                    self.parser.skip();
                    println!("parse_element: got unexpected {:?}", xml_element.event);
                    todo!();
//                Ok(None)
//                return XmlDocumentError::UnexpectedXml(xml_element.event.clone())
                },
            }
        }
    }

    /*
     * Accumulate a list of the following:
     *
     * o    XmlEvent::Comment
     * o    XmlEvent::Whitespace
     * o    XmlEvent::Characters
     * o    XmlEvent::CData
     *
     * The parser will return the first XmlElement that is not one of these.
     *
     * Returns:
     * Ok(Vec<XmlEvent>)
     * Err(XmlDocumentError>
     */
    fn parse_pieces(&mut self) -> Result<Vec<XmlEvent>, XmlDocumentError> {
        let mut pieces = Vec::<XmlEvent>::new();

        loop {
            let xml_element = self.parser.lookahead()?;
            let lineno = xml_element.lineno;

            match xml_element.event {
                XmlEvent::Comment(cmnt) => pieces.push(XmlEvent::Comment(cmnt.clone())),
                XmlEvent::Whitespace(ws) =>
                    pieces.push(XmlEvent::Whitespace(ws.clone())),
                XmlEvent::Characters(characters) =>
                    pieces.push(XmlEvent::Characters(characters.clone())),
                XmlEvent::CData(cdata) => pieces.push(XmlEvent::CData(cdata.clone())),

                _ => break,
            }

            self.parser.skip();
        }

        Ok(pieces)
    }

/*
    /*
     * Parse the current element and subelements. The <StartElement> has
     * already been read, read up to, and including, the <EndElement>
     * element_in:   Definition for this element
     * name_in:                 Name of the element
     * element_info_in:         Other information about the element
     *
     * Returns:
     * Ok(DirectElement)
     * Err(XmlDocumentError)
     */
    fn parse_start_element_remainder<T: Read>(
        &mut self,
        name_in:            OwnedName,
        element_info_in:    ElementInfo,
    ) -> Result<DirectElement, XmlDocumentError> {
        let mut before_element = Vec::new();
        let mut content = Vec::new();
        let mut after_element = Vec::new();
        let mut subelements = Vec::new();

        let mut pieces = Vec::new();

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
                    let element_info =
                        ElementInfo::new(lineno, attributes2.clone(), namespace2.clone());
println!("StartElement has name {}", start_name.local_name);
                    let next_element = self.start_element(&mut subelements, &start_name, lineno)?;

                    let subelement = self.parse_start_element_remainder::<R>(
                        start_name.clone(),
                        element_info.clone(),
                    )?;

                      
                    subelements_mut().push(Box::new(subelement));
                    pieces = Vec::<XmlEvent>::new();
                }
                XmlEvent::EndElement { name } => {
                    if start_name.local_name != *element.name() {
                        return Err(XmlDocumentError::MisplacedElementEnd(
                            lineno,
                            start_name.local_name.to_string(),
                            end_name.local_name.to_string(),
                        ));
                    }

                    content = pieces;
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

        DirectElement::new(name_in.clone(), element_info_in.clone(), before_element, content, after_element, subelements)
    }

    fn start_element(
        &mut self,
        subelements:    &mut Vec<Box<dyn Element>>,
        start_name:     &OwnedName,
        lineno:         usize,
    ) -> Result<&Box<dyn Element>, XmlDocumentError> {
        let subelement = parent_element.get(start_name.local_name.as_str());

        let next_element = if READING_XML {
            match subelement {
                None => {
                    return Err(XmlDocumentError::UnknownElement(
                        lineno,
                        start_name.local_name.to_string(),
                        parent_element.name().to_string()
                    ))
                },
                Some(elem) => elem,
            }
        } else {
            // Reading XSD. It's an error if the element is found, otherwise
            // we create a new element.
            match subelement {
                None => {
                    warning(&XmlDocumentError::UnknownElement(
                        lineno,
                        start_name.local_name.to_string(),
                        parent_element.name().to_string()
                    ));
                    let e = DirectElement::new(
                            OwnedName {local_name: "FIXME".to_string(),
                                namespace: None, prefix: None},
                            ElementInfo::new(lineno,
                                Vec::<OwnedAttribute>::new(),
                                Namespace (BTreeMap::<String, String>::new()),
                            ),

                            vec!(), vec!(), vec!(), vec!()
                    );
                    let elem = Box::new(e) as Box<dyn Element>;
                    parent_element.subelements_mut().push(elem);
                    &elem
                },
                Some(elem) => elem,
            }
        };
        Ok(next_element)
    }
*/
}

/*
impl<R: Read> fmt::Display for XmlDocumentFactory<'_, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.xml_schema)
    }
}
*/
