/*
 * Take an XMLDefinition tree and generate an XmlDocument
 */

use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
use xml::attribute::OwnedAttribute;
use xml::common::XmlVersion;
use xml::name::OwnedName;
use xml::namespace::Namespace;

use crate::xml_document_error::XmlDocumentError;
use crate::xml_document_factory::XmlDocumentFactory;
use crate::parser::LineNumber;
use crate::xml_definition::XmlDefinition;

#[derive(Clone, Debug)]
pub struct ElementInfo {
    pub lineno:                 LineNumber,
    pub attributes:             Vec<OwnedAttribute>,
    pub namespace:              Namespace,
}

impl ElementInfo {
    pub fn new(lineno: LineNumber, attributes: Vec<OwnedAttribute>, namespace: Namespace) -> ElementInfo {
        ElementInfo {
            lineno:     lineno,
            attributes: attributes,
            namespace:  namespace,
        }
    }
}

/*
 * Define the structure used to construct the tree for the parsed document.
 */
#[derive(Clone, Debug)]
pub struct Element<'a> {
    pub name:               OwnedName,
    depth:                  usize,
    element_info:           ElementInfo,
    pub subelements:        Vec<&'a str>,
    before_comments:        Vec<String>,
    after_comments:         Vec<String>,
}

impl<'a> Element<'a> {
    pub fn new(name: OwnedName, depth: usize, element_info: ElementInfo) -> Element<'a> {
        Element {
            name:               name,
            depth:              depth,
            element_info:       element_info,
            subelements:        Vec::<&'a str>::new(),
            before_comments:    Vec::<String>::new(),
            after_comments:     Vec::<String>::new(),
        }
    }

    pub fn get_attribute(&self, name: &str) -> Option<&String> {
        for attribute in &self.element_info.attributes {
            if attribute.name.local_name == name {
                return Some(&attribute.value);
            }
        }

        return None;
    }
}

impl<'a> fmt::Display for Element<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const INDENT_STR: &str = "   ";
        let indent_string = INDENT_STR.to_string().repeat(self.depth);

        write!(f, "{}<{}", indent_string, self.name.local_name)?;
        for attribute in self.element_info.attributes.clone() {
            write!(f, " {}={}", attribute.name.local_name, attribute.value)?;
        }

        if self.subelements.len() == 0 {
            write!(f, " /> (line {})\n", self.element_info.lineno)?;
        } else {
            write!(f, "> (line {})\n", self.element_info.lineno)?;

            for element in &self.subelements {
                element.fmt(f)?;
            }

            write!(f, "{}</{}>\n", indent_string, self.name.local_name)?;
        }


        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct DocumentInfo {
    version:    XmlVersion,
    encoding:   String,
    standalone: Option<bool>,
}

impl DocumentInfo {
    pub fn new(version: XmlVersion, encoding: String, standalone: Option<bool>) ->
        DocumentInfo {
        DocumentInfo {
            version:    version,
            encoding:   encoding,
            standalone: standalone,
        }
    }
}

/*
 * Parsed XML document
 *
 * document_info    Information about the document
 * elements         The oarsed document
 */
#[derive(Debug)]
pub struct XmlDocument<'a> {
    document_info:      DocumentInfo,
    pub root:           Option<&'a Element<'a>>,
    pub elements:       Vec<Element<'a>>,
}

impl<'a> XmlDocument<'a> {
    pub fn new(path: String, xml_definition: &'a XmlDefinition<'a>) ->
        Result<XmlDocument<'a>, XmlDocumentError> {
        let file = match File::open(path) {
            Err(e) => return Err(XmlDocumentError::Error(Box::new(e))),
            Ok(f) => f,
        };
        let reader = BufReader::new(file);
        XmlDocument::new_from_reader(reader, xml_definition)
    }
}

impl XmlDocument<'_> {
    pub fn new_from_reader<'a, R: Read + 'a>(buf_reader: BufReader<R>,
        xml_definition: &'a XmlDefinition<'a>) ->
        Result<XmlDocument<'a>, XmlDocumentError> {
        let factory = XmlDocumentFactory::new_from_reader(buf_reader,
            xml_definition)?;

        let document_info = factory.parse_start_document()?;
        let elements = factory.parse_end_document()?;
        let xml_document = XmlDocument {
            document_info:  document_info,
            root:           None,
            elements:       Vec::<Element>::new(),
        };

        factory.set_root(&mut xml_document)?;

        Ok(xml_document)
    }
}
        
impl fmt::Display for XmlDocument<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
println!("document:");
        write!(f, "<?xml {} {} {:?}>\n",
            self.document_info.version, self.document_info.encoding, self.document_info.standalone)?;
        write!(f, "{:?}", self.root)       
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;
    use crate::xml_document_factory::Element;
    use crate::xml_definition::ElementDefinition;

    static TEST_XML_DESC_TREE: XmlDefinition = XmlDefinition {
        root_name:  "a1",
        element_definitions:  & [
            ElementDefinition {
                name:   "a1",
                allowable_subelements: & ["a2"],
            },
            ElementDefinition {
                name:   "a2",
                allowable_subelements: & ["a1"],
            },
        ]
    };

    #[test]
    fn test1() {
        let input = r#" <?xml version=\"1.0\"?>
            <XTCE xmlns=\"http://www.omg.org/spec/XTCE/\">
                <SpaceSystem xmlns=\"http://www.omg.org/space/xtce\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"http://www.omg.org/space/xtce ../SpaceSystemV1.0.xsd\" name=\"TrivialSat\">
                </SpaceSystem>
            </XTCE>"#;

        let cursor = Cursor::new(input);
        let buf_reader = BufReader::new(cursor);
        match XmlDocument::new_from_reader(buf_reader, &TEST_XML_DESC_TREE) {
            Err(e) => println!("Failed: {}", e),
            Ok(xml_document) => println!("Result: {}", xml_document),
        }
    }
}
