/**
 * Basic information about the document
 */
use xml::common::XmlVersion;

#[derive(Clone, Debug)]
pub struct DocumentInfo {
    pub version: XmlVersion,
    pub encoding: String,
    pub standalone: Option<bool>,
}

impl DocumentInfo {
    pub fn new(version: XmlVersion, encoding: String, standalone: Option<bool>) -> DocumentInfo {
        DocumentInfo {
            version,
            encoding,
            standalone,
        }
    }
}
