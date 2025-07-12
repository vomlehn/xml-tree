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
use xml::common::XmlVersion;
use xml::name::OwnedName;
use xml::reader::XmlEvent;
use xml::namespace::Namespace;

use crate::parser::{LineNumber, Parser};
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

        let mut pieces = Vec::new();
        let mut elements: Vec<Box<dyn Element>> = Vec::new();
        let mut document_info_opt = None;
        let mut parent_name = vec!("XML document root");
        let mut states = vec!(ParseState::Init);
//        let mut elem_args = None;
//        let mut result = None;

        let top_element = loop {
            let xml_element = self.parser.next()?;
            let lineno = xml_element.lineno;

            match xml_element.event {
                XmlEvent::StartDocument {
                    version,
                    encoding,
                    standalone,
                } => {
                    Self::swap_state(&mut states, vec!(ParseState::Init), ParseState::Top)?;
                    let document_info = DocumentInfo::new(version, encoding, standalone);
                    document_info_opt = Some(document_info);
                }

                XmlEvent::EndDocument => {
                    let state = match states.pop() {
                        None => panic!("FIXME: internal error: no state on stack"),
                        Some(state) => if states.len() != 0 {
                            panic!("FIXME: internal error: premature EndDocument");
                        } else {
                            state
                        },
                    };

                    let top_element = match state {
                        ParseState::Top => panic!("FIXME: no elements in file"),
                        ParseState::InElement(_, _) => {
                            if elements.len() != 0 {
                                panic!("FIXME: {} unclosed elements", elements.len())
                            }

                            elements.pop().unwrap()
                        },

                        _ => panic!("FIXME: internal error: unexpected state {:?}", state),
                    };

                    break top_element;
                },

                XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace
                } => {
                    Self::push_state(&mut states, vec!(ParseState::Top, ParseState::InElement(name.clone(), lineno)),
                        ParseState::InElement(name.clone(), lineno))?;
                    let element_info = ElementInfo::new(lineno, attributes, namespace);
                    let element = DirectElement::new(name, element_info, vec!(), vec!(), vec!(), vec!());
                    elements.push(Box::new(element));
                },

                XmlEvent::EndElement {
                    name
                } => {
                    if states.len() == 0 {
                        panic!("FIXME: end element without start element");
                    }
                    Self::pop_state(&mut states, vec!(ParseState::InElement(name.clone(), lineno)))?;

                    let element = elements.pop().unwrap();
                    let last = states.last_mut().unwrap();

                    match last {
                        ParseState::InElement(last_name, last_lineno) => {
                            if element.name() != name.local_name {
                                panic!("FIXME: <{}> on line {} does not match <{}> on line {}", element.name(), element.lineno(), last_name, last_lineno);
                            }
                        },
                        _ => panic!("FIXME: unexpected state: {:?}", last),
                    }
                },

                XmlEvent::Comment(_) |
                XmlEvent::Whitespace(_) |
                XmlEvent::Characters(_) |
                XmlEvent::CData(_) => pieces.push(xml_element),

                // FIXME: check this
                _ => return Err(XmlDocumentError::UnexpectedXml(xml_element.event.clone())),
            }
        };

        let document_info = document_info_opt.unwrap();
        Ok(XmlDocument::new(document_info, vec!(top_element)))
    }

    /*
     * Verifies that the state on the top of the stack is as expected
     */
    fn check_state(states: &Vec<ParseState>, expected: Vec<ParseState>) -> Result<(), XmlDocumentError> {
        if expected.contains(&states.last().unwrap()) {
            panic!("FIXME: need proper error");
        }
        Ok(())
    }

    /*
     * Verifies that the top of the state matches the expected current state and, if so, pops it
     * and pushes the new state
     */
    fn swap_state<'b>(states: &mut Vec<ParseState>, expected: Vec<ParseState>, new: ParseState) -> Result<(), XmlDocumentError> {
        Self::check_state(states, expected)?;
        states.pop();
        states.push(new);
        Ok(())
    }

    fn push_state<'b>(states: &mut Vec<ParseState>, expected: Vec<ParseState>, new: ParseState) -> Result<(), XmlDocumentError> {
        Self::check_state(states, expected)?;
        states.push(new);
        Ok(())
    }

    fn pop_state(states: &mut Vec<ParseState>, expected: Vec<ParseState>) -> Result<(), XmlDocumentError> {
        Self::check_state(states, expected)?;
        states.pop();
        Ok(())
    }

/*
    fn start_document(version: XmlVersion, encoding: String, standalone: Option<bool>) -> Result<Option<DocumentInfo>, XmlDocumentError> {
        let document_info = DocumentInfo::new(version, encoding, standalone);
        Ok(Some(document_info))
    }

    fn end_document(cur_state: ParseState, xml_element: XmlEvent) -> Result<(), XmlDocumentError> {
        Ok(ParseState::End)
    }

    fn start_element(cur_state: ParseState, xml_element: XmlEvent) -> Result<Option<ElementArgs>, XmlDocumentError> {
        if let XmlEvent::StartElement(name, attributes, namespace) = xml_element {
            Some(ElementArgs::new(name, attributes, namespace))
        } else {
            panic!("FIXME: Internal error: expected but failed to find StartElement");
        }
    }

    fn end_element(cur_state: ParseState, xml_element: XmlEvent) -> Result<(), XmlDocumentError> {
        Ok(())
    }
*/

/*
        // Let's look for the end of the document
        loop {
            // FIXME: this isn't really right
            let next_item = xml_factory.parser.next();
println!("Looking for EndDocument");
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
*/

/*
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
    fn parse_elements(&mut self, parent_name: &str, before_pieces: Vec<XmlEvent>) -> Result<Vec<Box<dyn Element>>, XmlDocumentError> {
        let mut elements = Vec::<Box<dyn Element>>::new();

        loop {
            let xml_element = self.parser.next()?;
println!("parse_elements: looking for StartElement, have {:?}", xml_element.event);
            let lineno = xml_element.lineno;

            match &xml_element.event {
                XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                } => {
                    element = self.start_element(name, attributes, namespace, before_pieces)?;
                    elements.push(element);
                },

                XmlEvent::EndElement => {
                    panic!("FIXME: unexpected EndElement");
                },

                XmlEvent::EndDocument {} => { break; },

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
                    before_pieces.push(xml_element.event);
                    self.parser.skip();
//                    println!("parse_element: got unexpected {:?}", xml_element.event);
//                    todo!();
    //                Ok(None)
    //                return XmlDocumentError::UnexpectedXml(xml_element.event.clone())
                },
            }
        }

        Ok(elements)
    }

    /* We have parsed a StartElement and need to parse everything inside of it up to the
     * corresponding EndElement.
     * name:        The OwnedName from the StartElement
     * attributes:  The OwnedAttribute from the StartElement
     * namespace:   The Namespace from the StartElement
     *
     * Returns:
     * Ok(Box<dyn Element>)
     * Err(XmlDocumentError)
     */
    fn start_element(&self, parent_name: OwnedName, parent_attributes: OwnedAttribute, parent_namespace: Namespace, parent_before_pieces: Vec<XmlEvent>) -> Result<Box<dyn Element>, XmlDocumentError> {
        let start_name = parent_name;

        let element = loop {
            // FIXME: need a better name
            let after_pieces = self.parse_pieces()?;
            let xml_element = self.parser.lookahead()?;
println!("parse_start_element: {:?}", xml_element.event);

            match &xml_element.event {
                XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                } => {
                    // FIXME: not sure whether the pieces should be before or after
                    let elements = self.parse_elements(parent_name, parent_before_pieces)?;
                },

                XmlEvent::EndElement { name } => {
                    let end_name = name;

                    // FIXME: use namespace and prefix
                    if end_name.local_name != parent_name.local_name {
                        // FIXME: use XmlDocumentError
                        panic!("Closing element name {:?} != opening element name {:?}",
                            end_name, parent_name);
                    }

                    // FIXME: should this be here?
                    let after_pieces = self.parse_pieces()?;

                    let element_info =
                        ElementInfo::new(lineno, parent_attributes.clone(), parent_namespace.clone());

                    // FIXME: look for content and after_element?
                    let element = DirectElement::new(parent_name.clone(), element_info,
                        parent_before_pieces, vec!(), after_pieces, subelements);
                    break element;
                },

                // FIXME: handle processing instructions
 
                // Anything else means that we don't have any more elements at this
                // level.
                _ => {
                    println!("parse_element: got unexpected {:?}", xml_element.event);
                    todo!();
                },
            }

            Ok(element)
        };
    }

/*
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
            let mut before_pieces = self.parse_pieces()?;
            let xml_element = self.parser.lookahead()?;
println!("parse_element: {:?}", xml_element.event);

            match &xml_element.event {
                // FIXME: fix the arguments
                XmlEvent::StartElement {name, ..} => {
                    let start_name = name.clone();
                    let subelements = self.parse_elements(&start_name.local_name)?;

/*
                    // Collect the various incidental pieces
                    let after_pieces = self.parse_pieces()?;
                    // FIXME: think about this
                    if after_pieces.len() != 0 {
                        panic!("pieces after parsing element");
                    }

                    // Get the next XmlEvent, which *should* be an EndElement
                    let direct_element = self.parser.next()?;
                    let ev = direct_element.event.clone();
                    match direct_element.event {
                        XmlEvent::EndElement { name } => {},
                        _ => {
                            // FIXME: get this right
                            panic!("Missing XmlEvent::EndElement, got {:?}", ev);
                        },
                    }
                    
                    // Verify that the name given in the start element and the name from
                    // the end element match.
                    // FIXME: doesn't seem quite right
                    let end_name = match xml_element.event.clone() {
                        XmlEvent::EndElement {name} => name,
                        _ => panic!("FIXME: figure this out"),
                    };
*/
                },

                XmlEvent::EndElement { name } => {
                    self.parser.skip();
//                // FIXME: anything better for the parse_elements() argument?
                    let owned_name = OwnedName {local_name: "FIXME".to_string(),
                        namespace: None, prefix: None};
                    return Ok((before_pieces, owned_name, subelements));
                },

                XmlEvent::EndDocument => {
                    panic!("FIXME: unexpected document");
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
                    before_pieces.push(xml_element.event);
                    self.parser.skip();
//                    println!("parse_element: got unexpected {:?}", xml_element.event);
//                    todo!();
//                Ok(None)
//                return XmlDocumentError::UnexpectedXml(xml_element.event.clone())
                },
            }
        }
    }
*/

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
println!("parse_pieces: {:?}", xml_element.event);
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
*/

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
