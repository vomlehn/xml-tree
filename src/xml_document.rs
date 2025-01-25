/*
 * Take an XMLDefinition tree and generate an XmlDocument
 */

use std::fmt;
use std::fs::File;
use std::rc::Rc;
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
    pub fn new(lineno: LineNumber, attributes: Vec<OwnedAttribute>,
    namespace: Namespace) -> ElementInfo {
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
pub struct Element {
    pub name:               OwnedName,
    depth:                  usize,
    element_info:           ElementInfo,
    pub subelements:        Vec<String>,
    before_comments:        Vec<String>,
    after_comments:         Vec<String>,
}

impl Element {
    pub fn new<'b>(name: OwnedName, depth: usize, element_info: ElementInfo) ->
        Element {
        Element {
            name:               name,
            depth:              depth,
            element_info:       element_info,
            subelements:        Vec::<String>::new(),
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

impl fmt::Display for Element {
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
            let subelement_indent = INDENT_STR.to_string().repeat(self.depth + 1);

            for element in &*self.subelements {
                write!(f, "{}", subelement_indent);
                element.fmt(f)?;
                write!(f, "\n");
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
pub struct XmlDocument {
    pub document_info:  DocumentInfo,
    pub root_name:      String,
    pub elements:       Rc<Vec<Element>>,
}

impl XmlDocument {
    pub fn new(path: String, xml_definition: &XmlDefinition) ->
        Result<XmlDocument, XmlDocumentError> {
        let file = match File::open(path) {
            Err(e) => return Err(XmlDocumentError::Error(Box::new(e))),
            Ok(f) => f,
        };
        let reader = BufReader::new(file);
        XmlDocument::new_from_reader(reader, xml_definition)
    }
}

impl XmlDocument {
    pub fn new_from_reader<R: Read>(
        buf_reader: BufReader<R>,
        xml_definition: &XmlDefinition) ->
        Result<XmlDocument, XmlDocumentError> {

        // Create the factory using the reader and XML definition
        let xml_document = XmlDocumentFactory::<R>::new_from_reader(buf_reader,
            xml_definition)?;
        Ok(xml_document)
    }
}
        
impl fmt::Display for XmlDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<?xml {} {} {:?}>\n",
            self.document_info.version, self.document_info.encoding,
            self.document_info.standalone)?;

        write!(f, "Root: {}\n", self.root_name)?;
        write!(f, "Elements:\n")?;

        for element in self.elements.iter() {
            write!(f, "{}\n", element)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use std::io::BufRead;

    use super::*;
    use crate::xml_definition::ElementDefinition;

    static TEST_XML_DESC_TREE: XmlDefinition = XmlDefinition {
        root_name:  "XTCE",
        element_definitions:  & [
            ElementDefinition {
                name:                   "XTCE",
                allowable_subelements:  &["SpaceSystem"],
            },
            ElementDefinition {
                name:                   "SpaceSystem",
                allowable_subelements:  &["a1"],
            },
            ElementDefinition {
                name:                   "a1",
                allowable_subelements:  &["a2"],
            },
            ElementDefinition {
                name:                   "a2",
                allowable_subelements:  &["a1"],
            },
        ]
    };

    #[test]
    fn test1() {
        println!("Test: test1");
        println!("XML Definition: {}", TEST_XML_DESC_TREE);

        let input = r#"<?xml version="1.0"?>
            <XTCE xmlns="http://www.omg.org/spec/XTCE/">
                <SpaceSystem xmlns="http://www.omg.org/space/xtce" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://www.omg.org/space/xtce ../SpaceSystemV1.0.xsd" name="TrivialSat">
                <a1 />
                <a2>
                </a2>
                </SpaceSystem>
            </XTCE>"#;
        println!();
        println!("Input: {}", input);

        println!();
        let cursor = Cursor::new(input);
        let mut buf_reader = BufReader::new(cursor);

        match XmlDocument::new_from_reader(buf_reader, &TEST_XML_DESC_TREE) {
            Err(e) => println!("Failed: {}", e),
            Ok(xml_document) => println!("XML Document: {}", xml_document),
        }
    }

    #[test]
    fn test2() {
        println!("Test: test2");
        let input = r#"<?xml version="1.0"?>
            <XTCE xmlns="http://www.omg.org/spec/XTCE/">
                <SpaceSystem xmlns="http://www.omg.org/space/xtce" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://www.omg.org/space/xtce ../SpaceSystemV1.0.xsd" name="TrivialSat">
                <a1 />
                <a2>
                </a2>
                </SpaceSystem>
            </XTCE>"#;
        println!();
        println!("Input: {}", input);

        println!();
        let cursor = Cursor::new(input.as_bytes());
        let mut buf_reader = BufReader::new(cursor);
        let line_reader = crate::parser::LinenoReader::new(buf_reader);
        let lineno_ref = line_reader.lineno_ref();
        let mut event_reader = xml::EventReader::new(line_reader);
        
        loop {
            let event = event_reader.next();

            match event {
                Err(e) => {
                        println!("Err: {:?}", e);      
                        break;
                },
                Ok(o) => match o {
                    xml::reader::XmlEvent::EndDocument{..} => {
                        println!("EOD");
                        break;
                    }
                    _ => println!("Ok: {:?}", o),
                }
            }

            println!("done");
        }
    }
}
