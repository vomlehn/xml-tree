/*
 * A layer built on top of Xml::EventReader to provide look-ahead and line
 * numbers.
 */

//use std::borrow::Borrow;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::io::Read;
//use xml::attribute::OwnedAttribute;
//use xml::common::XmlVersion;
//use xml::name::OwnedName;
//use xml::namespace::Namespace;
use xml::reader::{EventReader, XmlEvent};

use crate::xml_document_error::{XmlDocumentError};

pub type LineNumber = usize;

#[derive(Clone, Debug)]
pub struct XmlElement {
    pub lineno:         LineNumber,
    pub event:          XmlEvent,
}

impl XmlElement {
    fn new(lineno: LineNumber, event: XmlEvent) -> XmlElement {
        XmlElement {
            lineno:         lineno,
            event:          event,
        }
    }
}

impl fmt::Display for XmlElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.event)
    }
}

pub struct Parser<R: Read> {
    lineno_ref:     Rc<RefCell<LineNumber>>,
    pending:        Option<Result<XmlElement, XmlDocumentError>>,
    event_reader:   EventReader<LinenoReader<R>>,
}

impl<R: Read> Parser<R> {
    pub fn new(reader: R) -> Self {
        let line_reader = LinenoReader::new(reader);
        let lineno_ref = line_reader.lineno_ref();
        let event_reader = EventReader::new(line_reader);
        
        Parser {
            lineno_ref:     lineno_ref,
            pending:        None,
            event_reader:   event_reader,
        }
    }

    /*
     * Read the next XmlElement. Each read returns a new value.
     */
    pub fn next(&mut self) -> Result<XmlElement, XmlDocumentError> {
        self.skip();
        self.lookahead()
    }

    /*
     * Discard the current XmlElement, forcing a fetch of the next item
     * if current() is used.
     */
    pub fn skip(&mut self) {
        self.pending = None;
    }

    /*
     * Read the next XmlElement from the input stream, disc without removing
     * it from the stream.
     */
pub fn lookahead(&mut self) -> Result<XmlElement, XmlDocumentError> {
    if self.pending.is_none() {
        let lineno = *self.lineno_ref.borrow();
        let evt = self.event_reader.next();

        // Process the event and store the result directly in `self.pending`
        self.pending = Some(match evt {
            Err(e) => Err(XmlDocumentError::XmlError(lineno, e)), // Create the error directly
            Ok(xml_event) => Ok(XmlElement::new(lineno, xml_event)), // Create the XmlElement directly
        });
    }

    // Consume the stored result and return it
    match self.pending.take() {
        None => Err(XmlDocumentError::InternalError(
            *self.lineno_ref.borrow(),
            "self.pending is None when it must be Some".to_string(),
        )),
        Some(result) => result, // Return the stored result (Ok or Err) without cloning
    }
}

/*
    pub fn lookahead(&mut self) -> Result<&XmlElement, XmlDocumentError> {
        if self.pending.is_none() {
            let lineno = *self.lineno_ref.borrow();
            let evt = self.event_reader.next();

            let xml_event = match evt {
                Err(e) => {
                    let err = Err(XmlDocumentError::XmlError(lineno, e));
                    self.pending = Some(err);
                },
                Ok(xml_event) => {
                    let ok = Ok(XmlElement::new(lineno, xml_event));
                    self.pending = Some(ok);
                },
            };
        }

//        if let Some(value) = &self.pending {
        match &self.pending {
            None => return Err(XmlDocumentError::InternalError(*self.lineno_ref.borrow(),
                "self.pending is None when is must be Some".to_string())),
            Some(value) => {
                match value {
                    Err(e) => {
                        return Err(e.clone());
                    },
                    Ok(xml_element) => {
                        return Ok(xml_element);
                    }
                }
            }
        };
    }
*/
/*
    pub fn lookahead(&mut self) -> Result<&XmlElement, XmlDocumentError> {
        let lineno = *self.lineno_ref.borrow();

        if self.pending.is_none() {
            let evt = self.event_reader.next();

            let xml_event = match evt {
                Err(e) => return Err(XmlDocumentError::XmlError(lineno, e)),
                Ok(xml_event) => xml_event,
            };

            let xml_element = XmlElement::new(lineno, xml_event);
            self.pending = Some(Ok(&xml_element));
        }

        // Safely return a reference to the cached value
//        self.pending.as_ref().ok_or(XmlDocumentError::Unknown(lineno))
        self.pending.unwrap()
    }
*/
}

impl<R: Read> fmt::Debug for Parser<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parser: lineno: {}", *self.lineno_ref.borrow())
    }
}

pub struct LinenoReader<R: Read> {
    inner:      R,
    lineno:     Rc<RefCell<LineNumber>>,
}

impl<R: Read> LinenoReader<R> {
    pub fn new(inner: R) -> Self {
        LinenoReader {
            inner:      inner,
            lineno:     Rc::new(RefCell::new(1)),
        }
    }

    pub fn lineno_ref(&self) -> Rc<RefCell<LineNumber>> {
        self.lineno.clone()
    }
}

impl<R: Read> Read for LinenoReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bytes_read = self.inner.read(buf)?;
        let mut lineno = self.lineno.borrow_mut();
        *lineno += buf[..bytes_read].iter().filter(|&&c| c == b'\n').count();
        Ok(bytes_read)
    }
}

/*
/*
 * xml::XmlEvent isn't clonable, so this maps to local events
 */
fn xml_event_map(xml_event: XmlEvent) -> XmlEvt {
    match xml_event {
		XmlEvent::StartElement{name, attributes, namespace} => XmlEvt::StartElement(name, attributes, namespace),
		XmlEvent::EndElement{name} => XmlEvt::EndElement(name),
//		XmlEvent::EmptyElement(name, attributes) => XmlEvt::EmptyElement(name, attributes),
		XmlEvent::Characters(chars) => XmlEvt::Characters(chars),
		XmlEvent::CData(cdata) => XmlEvt::CData(cdata),
		XmlEvent::Comment(cmnt) => XmlEvt::Comment(cmnt),
		XmlEvent::ProcessingInstruction{name, data} => XmlEvt::ProcessingInstruction(name, data),
//		XmlEvent::DocType(doctype) => XmlEvt::DocType(doctype),
		XmlEvent::StartDocument{version, encoding, standalone} => XmlEvt::StartDocument(version, encoding, standalone),
		XmlEvent::EndDocument => XmlEvt::EndDocument(),
		XmlEvent::Whitespace(ws) => XmlEvt::Whitespace(ws),
    }
}

enum XmlEvt {
    StartElement(OwnedName, Vec<OwnedAttribute>, Namespace),
    EndElement(OwnedName),
    EmptyElement(String, Vec<(String, Option<String>)>),
    Characters(String),
    CData(String),
    Comment(String),
    ProcessingInstruction(String, Option<String>),
    DocType(String),
    StartDocument(XmlVersion, String, Option<bool>),
    EndDocument(),
    Whitespace(String),
}
*/
