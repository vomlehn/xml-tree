use std::sync::Arc;
use thiserror::Error;
use xml::reader::XmlEvent;

use crate::parser::LineNumber;

#[derive(Debug, Error)]
pub enum XmlDocumentError {
    #[error("Can't insert element \"{0}\", is it a duplication?")]
    CantInsertElement(String),

    #[error("Element name \"{0}\" is duplicated in ElementDefs")]
    DuplicateElementDefsName(String),

    #[error("Duplicate allowable element {0} for Element {1}")]
    DuplicateAllowableElement(String, String),

    #[error("Duplicate key {0}")]
    DuplicateKey(String),

    // FIXME: RefCell?
    #[error("XML parser error: {0}")]
    Error(Arc<dyn std::error::Error>),

    #[error("Line {0}: Internal error: {1}")]
    InternalError(LineNumber, String),

    #[error("line {0}: Misplaced element end: {1}, found {2}")]
    MisplacedElementEnd(LineNumber, String, String),

    // FIXME: need to fix this
    #[error("No end element in input")]
    NoEndDocument(),

    #[error("No document found in input")]
    NoDocumentFound(),

    #[error("No element \"{0}\" as referenced in element description for \"{1}\"")]
    NoSuchElement(String, String),

    // FIXME: need to fix this
    #[error("No XML elements in input")]
    NoXTCE(),

    #[error("Allowable key \"{0}\" for element definition \"{1}\" not found in elements")]
    AllowableKeyNotAnElement(String, String),

    #[error("Must have exactly one root element")]
    OnlyOneRootElementAllowed(),

    #[error("ElementRef not resolved for \"{0}\"")]
    UnresolvedRef(String),

    #[error("line {0}: StartDocument after StartDocument")]
    StartAfterStart(LineNumber),

    #[error("ElementDef name \"{0}\" not in ElementDescs")]
    ElementDefNotInElementDescs(String),

    #[error("Root key \"{0}\" not found")]
    RootKeyNotFound(String),

    #[error("Root is unexpectedly None")]
    RootIsNone(),

    #[error("Unexpected XML error: {0:?}")]
    UnexpectedXml(XmlEvent),

    // FIXME: this is temporary and should eventually be deleted
    #[error("Line {0}: Unknown XTCE parsing error")]
    Unknown(LineNumber),

    #[error("line {0}: Unknown or misplaced element: <{1}>")]
    UnknownElement(LineNumber, String),

    // FIXME: get line number from the XmlEvent
    #[error("Line {0}: XML error: {1}")]
    XmlError(LineNumber, xml::reader::Error),

    #[error("No elements defined")]
    XmlNoElementDefined(),
}
