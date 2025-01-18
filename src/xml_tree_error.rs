use thiserror::Error;
use xml::reader::XmlEvent;

use crate::parser::{LineNumber};

#[derive(Error, Debug)]
pub enum XmlTreeError {
    #[error("line {0}: Misplaced element end: {1}")]
    MisplacedElementEnd(LineNumber, String), 

    // FIXME: need to fix this
    #[error("No XTCE elements in input")]
    NoXTCE(),

    #[error("Line {0}: Only one root element is allowed\n")]
    OnlyOneRootElement(LineNumber),

    #[error("line {0}: StartDocument after StartDocument")]
    StartAfterStart(LineNumber), 

    #[error("Unexpected XML error: {0:?}")]
    UnexpectedXml(XmlEvent),

    // FIXME: this is temporary and should eventually be deleted
    #[error("Line {0}: Unknown XTCE parsing error")]
    Unknown(LineNumber),

    #[error("line {0}: Unknown or misplaced element: <{1}>")]
    UnknownElement(LineNumber, String),

    // FIXME: get line number from the XmlEvent
    #[error("Line {0}: XML error: {1}")]
    XmlError(LineNumber, Box<dyn std::error::Error>),
}
