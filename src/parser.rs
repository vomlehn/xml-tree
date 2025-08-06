/*
 * A layer built on top of Xml::EventReader to provide look-ahead and line
 * numbers.
 */

//use std::borrow::Borrow;
use std::cell::RefCell;
use std::fmt;
use std::io::Read;
use std::rc::Rc;
//use xml::attribute::OwnedAttribute;
//use xml::common::XmlVersion;
//use xml::name::OwnedName;
//use xml::namespace::Namespace;
use xml::reader::{EventReader, XmlEvent};

use crate::xml_document_error::XmlDocumentError;

pub type LineNumber = usize;

pub trait XmlElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl fmt::Display for dyn XmlElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

/**
 * An XML element
 * lineno:  Line number of the start of this element
 * event:   XmlEvent returned by the XML low level parser
 */
#[derive(Clone, Debug)]
pub struct XmlDirectElement {
    pub lineno: LineNumber,
    pub event: XmlEvent,
}

impl XmlDirectElement {
    fn new(lineno: LineNumber, event: XmlEvent) -> XmlDirectElement {
        XmlDirectElement {
            lineno,
            event,
        }
    }
}

impl XmlElement for XmlDirectElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // FIXME: can I do better than the debug format?
        write!(f, "{:?}", self)
    }
}

/*
/**
 * List of XmlElements that can be shared to reduce the XML tree size
 * subelements: This is the list
 */
pub struct XmlIndirectElement {
    pub subelements:    Vec::<Box<dyn XmlElement>>,
}

impl<'a> XmlIndirectElement {
    /*
    fn new() -> XmlIndirectElement {
        XmlIndirectElement {
            subelements:    Vec::new(),
        }
    }
    */
}

impl XmlElement for XmlIndirectElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sep = "";
        write!(f, "[")?;

        for element in &self.subelements {
            element.fmt(f)?;
            write!(f, "{}", sep)?;
            sep = ", ";
        }

        write!(f, "]\n")?;
        Ok(())
    }
}
*/

/**
 * XML Parser
 * lineno_ref:      Reference counted reference to current line number
 *                  FIXME: check that this is appropriate
 * pending:         If None, we don't have a lookahead token. Otherwise,
 *                  this is the lookahead token wrapped in Some()
 * event_reader:    Object for reading the next XmlEvent
 */
pub struct Parser<R: Read> {
    lineno_ref: Rc<RefCell<LineNumber>>,
    pending: Option<Result<XmlDirectElement, XmlDocumentError>>,
    event_reader: EventReader<LinenoReader<R>>,
}

impl<R: Read> Parser<R> {
    pub fn new(reader: R) -> Self {
        let line_reader = LinenoReader::new(reader);
        let lineno_ref = line_reader.lineno_ref();
        let event_reader = EventReader::new(line_reader);

        Parser {
            lineno_ref,
            pending: None,
            event_reader,
        }
    }

    /**
     * Read the next XmlElement. Each read returns a new value. This
     * XmlElement is always an XmlDirectElement
     *
     * self:    &mut Parser
     *
     * Returns:
     * Ok(XmlDirectElement)
     * Err(XmlDocumentError)
     */
    pub fn next(&mut self) -> Result<XmlDirectElement, XmlDocumentError> {
        let result = self.lookahead()?;
/*
        if let Err(e) = result {
            return Err(e);
        }
*/

        self.skip();
        Ok(result)
    }

    /*
     * Discard the current XmlElement, forcing a fetch of the next item
     * if current() is used. This XmlElement is always an XmlDirectElement
     *
     * self:    &mut Parser
     */
    pub fn skip(&mut self) {
        self.pending = None;
    }

    /*
     * Read the next XmlElement from the input stream, without removing
     * it from the stream. This XmlElement is always an XmlDirectElement
     *
     * self:    &mut Parser
     *
     * Returns:
     * Ok(XmlDirectElement)
     * Err(XmlDocumentError)
     */
    pub fn lookahead(&mut self) -> Result<XmlDirectElement, XmlDocumentError> {
        // If we don't have any lookahead token, read another token to be
        // the lookahead token.
        if self.pending.is_none() {
            let lineno = *self.lineno_ref.borrow();
            let evt = self.event_reader.next();

            // We tried to read another lookahead token, but we might have
            // gotten an error. Check for this.
            match evt {
                Err(e) => {
                    // Indicate we have something, but that the something
                    // we have is an error
                    let error = XmlDocumentError::XmlError(lineno, e);
                    let err = Err(error.clone());
                    let pending_err = Some(Err(error));
                    self.pending = pending_err;
                    return err;
                },
                Ok(xml_event) => {
                    let element = XmlDirectElement::new(lineno, xml_event);
                    let ok = Ok(element.clone());
                    let pending_ok = Some(Ok(element));
                    self.pending = pending_ok;
                    return ok;
                }
            };

        }

        // We do have a pending token. If it's an error, return that. If
        // it's a token, return that, but in either case, don't remove it.
        match self.pending.take() {
            None => Err(XmlDocumentError::InternalError(
                *self.lineno_ref.borrow(),
                "self.pending is None when it must be Some".to_string(),
            )),
            Some(element) => element,
        }
    }
}

impl<R: Read> fmt::Debug for Parser<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parser: lineno: {}", *self.lineno_ref.borrow())
    }
}

/**
 * Object for reading an std::io::Read implementation, as annotated with
 * a line number.
 */
pub struct LinenoReader<R: Read> {
    inner: R,
    lineno: Rc<RefCell<LineNumber>>,
}

impl<R: Read> LinenoReader<R> {
    pub fn new(inner: R) -> Self {
        LinenoReader {
            inner,
            lineno: Rc::new(RefCell::new(1)),
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
 * FIXME: remove this, I think
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
