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
                        ParseState::Top => {
                            println!("{} elements on stack", elements.len());
                                panic!("FIXME: no elements in file");
                        },
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
                        ParseState::Top => {
                            println!("Found Top, end of all elements");
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
     * FIXME: figure out InElement
     */
    fn contains_state(states: &Vec<ParseState>, expected: &Vec<ParseState>) -> bool {
        for state in states {
            match state {
                ParseState::Init | ParseState::Top | ParseState::End => {
                    for exp in expected {
                        match exp {
                            ParseState::Init | ParseState::Top | ParseState::End => if state == exp { return true }
                            ParseState::InElement(_, _) => {}
                        }
                    }
                },

                ParseState::InElement(owned_name, _) => {
                    for exp in expected {
                        match exp {
                            ParseState::Init | ParseState::Top | ParseState::End => {},
                            ParseState::InElement(name, _) => {
                                if owned_name == name { return true; }
                            },
                        }
                    }
                }
            }
        }

        return false
    }

    /*
     * Verifies that the state on the top of the stack is as expected
     */
    fn check_state(states: &Vec<ParseState>, expected: Vec<ParseState>) -> Result<(), XmlDocumentError> {
        if !Self::contains_state(states, &expected) {
            panic!("FIXME: need proper error, expected {:?}, states.last() {:?}", expected, states.last().unwrap());
        }
        Ok(())
    }

    /*
     * Verifies that the top of the state matches the expected current state and, if so, pops it
     * and pushes the new state
     */
    fn swap_state<'b>(states: &mut Vec<ParseState>, expected: Vec<ParseState>, new: ParseState) -> Result<(), XmlDocumentError> {
//println!("swap_state: {:?}->{:?}", states.last().unwrap(), new);
        Self::check_state(states, expected)?;
        states.pop();
        states.push(new);
        Ok(())
    }

    fn push_state<'b>(states: &mut Vec<ParseState>, expected: Vec<ParseState>, new: ParseState) -> Result<(), XmlDocumentError> {
//println!("push_state: {:?}", new);
        Self::check_state(states, expected)?;
        states.push(new);
        Ok(())
    }

    fn pop_state(states: &mut Vec<ParseState>, expected: Vec<ParseState>) -> Result<(), XmlDocumentError> {
//println!("pop_state: {:?}", states.last().unwrap());
        Self::check_state(states, expected)?;
        states.pop();
        Ok(())
    }
}

