use std::fmt;

use std::cell::RefCell;
use std::rc::Rc;
use std::io::{BufReader, Read};

use xml::reader::{EventReader, XmlEvent};

use crate::xml_tree_error::{XmlTreeError};

pub type LineNumber = usize;

#[derive(Debug)]
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

pub struct Parser<R: Read> {
    lineno_ref:     Rc<RefCell<LineNumber>>,
    event_reader:   EventReader<LinenoReader<BufReader<R>>>,
}

impl<R: Read> fmt::Debug for Parser<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parser: lineno: {}", *self.lineno_ref.borrow())
    }
}

impl<R: Read> Parser<R> {
    pub fn new<T: Read>(buf_reader: BufReader<T>) -> Parser<T> {
        let line_reader = LinenoReader::new(buf_reader);
        let lineno_ref = line_reader.lineno_ref();
        let event_reader = EventReader::new(line_reader);
        Parser {
            lineno_ref:     lineno_ref,
            event_reader:   event_reader,
        }
    }

    /*
     * Read the next XmlElement from the input stream, disc
     */
    pub fn next(&mut self) -> Result<XmlElement, XmlTreeError> {
        let xml_event = self.event_reader.next();
        let lineno = *self.lineno_ref.borrow();
match &xml_event {
    Ok(XmlEvent::StartElement{name, ..}) => println!("start element: {}", name.local_name),
    Ok(XmlEvent::EndElement{name}) => println!("end element: {}", name.local_name),
    _ => {},
}

        let result = match xml_event {
            Err(e) => Err(XmlTreeError::XmlError(lineno, Box::new(e))),
            Ok(elem) => Ok(XmlElement::new(lineno, elem)),
        };

        result
    }
}

/*
impl<R: Read> IntoIterator for Parser<R> {
    type Item = XmlElement;
    type IntoIter = std::iter::Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self
    }
}
*/

// This is how we get line numbers for the XML file
// FIXME: handle nested files
pub struct LinenoReader<R: Read> {
    inner: BufReader<R>,
    line: Rc<RefCell<usize>>,
}

impl<R: Read> LinenoReader<R> {
    pub fn new(inner: R) -> Self {
        LinenoReader {
            inner: BufReader::new(inner),
            line: Rc::new(RefCell::new(1)),
        }
    }

    pub fn lineno_ref(&self) -> Rc<RefCell<usize>> {
        Rc::clone(&self.line)
    }
}

impl<R: Read> Read for LinenoReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bytes_read = self.inner.read(buf)?;
        let mut line = self.line.borrow_mut();
        *line += buf[..bytes_read].iter().filter(|&&c| c == b'\n').count();
        Ok(bytes_read)
    }
}
