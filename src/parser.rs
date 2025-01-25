/*
 * A layer built on top of Xml::EventReader to provide look-ahead and line
 * numbers.
 */

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::io::Read;
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
    cur:            Option<Result<XmlElement, XmlDocumentError>>,
    event_reader:   EventReader<LinenoReader<R>>,
}

impl<R: Read> Parser<R> {
    pub fn new(reader: R) -> Self {
        let line_reader = LinenoReader::new(reader);
        let lineno_ref = line_reader.lineno_ref();
        let event_reader = EventReader::new(line_reader);
        
        Parser {
            lineno_ref:     lineno_ref,
            cur:            None,
            event_reader:   event_reader,
        }
    }

    /*
     * Read the current XmlElement. Multiple reads will return the same
     * value until skip() is called.
     */
    pub fn next(&mut self) -> Result<XmlElement, XmlDocumentError> {
/*
        if self.cur.is_none() {
            self.cur = Some(self.lookahead());
        }
println!("next: {:?}", self.cur);
// FIXME: this is really ugly and can't be right
//        self.cur.as_ref().unwrap()
        match &self.cur {
            None => Err(XmlDocumentError::Unknown(0)),
            Some(c) => match c {
                Err(e) => Err(*e),
                Ok(c) => Ok(c.clone()),
            }
        }
*/
        let la = self.lookahead();
        la
    }

    /*
     * Discard the current XmlElement, forcing a fetch of the next item
     * if current() is used.
     */
    pub fn skip(&mut self) {
        self.cur = None;
    }

    /*
     * Read the next XmlElement from the input stream, disc
     */
// FIXME: need better name
// FIXME: this is wrong
    pub fn lookahead(&mut self) -> Result<XmlElement, XmlDocumentError> {
        let lineno = *self.lineno_ref.borrow();

        match self.event_reader.next() {
            Err(e) => return Err(XmlDocumentError::XmlError(lineno, e)),
            Ok(elem) => return Ok(XmlElement::new(lineno, elem)),
        };
    }
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
