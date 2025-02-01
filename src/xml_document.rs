/*
 * Take an XML Definition tree and generate an XmlDocument
 */

use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
use xml::attribute::OwnedAttribute;
use xml::common::XmlVersion;
use xml::name::OwnedName;
use xml::namespace::Namespace;
use xml::reader::XmlEvent;

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
    pub depth:              usize,
    pub element_info:       ElementInfo,
    pub subelements:        Vec<Element>,
    pub before_element:     Vec<XmlEvent>,
    pub content:            Vec<XmlEvent>,
    pub after_element:          Vec<XmlEvent>,
}

impl Element {
    pub fn new<'b>(name: OwnedName, depth: usize, element_info: ElementInfo) ->
        Element {
        Element {
            name:               name,
            depth:              depth,
            element_info:       element_info,
            subelements:        Vec::<Element>::new(),
            before_element:     Vec::<XmlEvent>::new(),
            content:            Vec::<XmlEvent>::new(),
            after_element:      Vec::<XmlEvent>::new(),
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
        write!(f, ">\n")?;


        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct DocumentInfo {
    pub version:    XmlVersion,
    pub encoding:   String,
    pub standalone: Option<bool>,
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
    pub root:           Element,
}

impl XmlDocument {
    pub fn new<'a>(path: &str, xml_definition: &'a XmlDefinition) ->
        Result<XmlDocument, XmlDocumentError<'a>> {
        let file = match File::open(path) {
            Err(e) => return Err(XmlDocumentError::Error(Box::new(e))),
            Ok(f) => f,
        };
        let reader = BufReader::new(file);
        XmlDocument::new_from_reader(reader, xml_definition)
    }
}

impl XmlDocument {
    pub fn new_from_reader<'a, R: Read + 'a> (
        buf_reader: BufReader<R>,
        xml_definition: &'a XmlDefinition) ->
        Result<XmlDocument, XmlDocumentError<'a>> {

        // Create the factory using the reader and XML definition
        let xml_document = XmlDocumentFactory::<R>::new_from_reader(buf_reader,
            xml_definition)?;
        Ok(xml_document)
    }

    fn display_piece(&self, f: &mut fmt::Formatter<'_>, pieces: &Vec<XmlEvent>) -> fmt::Result {
        let result = for piece in pieces {
            match piece {
                XmlEvent::Comment(cmnt) => write!(f, "<!-- {} -->", cmnt)?,
                XmlEvent::Whitespace(ws) => write!(f, "{}", ws)?,
                XmlEvent::Characters(characters) => write!(f, "{}", characters)?,
                XmlEvent::CData(cdata) => write!(f, "{}", cdata)?,
                _ => return Err(fmt::Error),
            }
        };

        Ok(result)
    }

    pub fn display_element(&self, f: &mut fmt::Formatter<'_>, depth: usize,
        element: &Element) ->
    fmt::Result {
println!("depth {}", depth);
        const INDENT_STR: &str = "   ";
        let indent_string = INDENT_STR.to_string().repeat(depth);

        self.display_piece(f, &element.before_element)?;

        write!(f, "{}<{}", indent_string, element.name.local_name)?;

        for attribute in &element.element_info.attributes {
            write!(f, " {}=\"{}\"", attribute.name, attribute.value)?;
        }

        if element.subelements.len() == 0 && element.content.len() == 0 {
            write!(f, " /> (line {})\n", element.element_info.lineno)?;
        } else {
            write!(f, "> (line {})\n", element.element_info.lineno)?;
            self.display_piece(f, &element.content)?;

            for element in &element.subelements {
                self.display_element(f, depth + 1, element)?;
            }

            write!(f, "{}</{}>\n", indent_string, element.name.local_name)?;
        }

        self.display_piece(f, &element.after_element)?;

        Ok(())
    }

    pub fn display(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
println!("display");
        write!(f, "<?xml {} {} {:?}>\n",
            self.document_info.version, self.document_info.encoding,
            self.document_info.standalone)?;

        let depth = 0;
        self.display_element(f, depth, &self.root)?;

        Ok(())
    }
}
        
impl<'a> fmt::Display for XmlDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
println!("XmlDocument::fmt()");
        self.display(f)
    }
}

#[cfg(test)]
mod tests {
//    use std::io::Cursor;

    use lazy_static::lazy_static;
    use petgraph::graph::NodeIndex;
    use std::collections::HashMap;

    use super::*;

    use crate::xml_definition::ElementDefinition;
    use crate::xsd_schema::XSD_SCHEMA;

    lazy_static!{
        static ref TEST_XML_DESC_TREE: XmlDefinition =
            XmlDefinition::new("XTCE".to_string(), vec![
                ElementDefinition {
                    name:                       "XTCE".to_string(),
                    key:                        "XTCE".to_string(),
                    allowable_subelements_map:  HashMap::<String, NodeIndex>::new(),
                    allowable_subelement_keys:  vec!("SPACE_SYSTEM".to_string()),
                },
                ElementDefinition {
                    name:                       "SpaceSystem".to_string(),
                    key:                        "SpaceSystem".to_string(),
                    allowable_subelements_map:  HashMap::<String, NodeIndex>::new(),
                    allowable_subelement_keys:  vec!("A1".to_string()),
                },
                ElementDefinition{
                    name:                       "a1".to_string(),
                    key:                        "a1".to_string(),
                    allowable_subelements_map:  HashMap::<String, NodeIndex>::new(),
                    allowable_subelement_keys:  vec!("A2".to_string()),
                },
                ElementDefinition{
                    name:                       "a2".to_string(),
                    key:                        "a2".to_string(),
                    allowable_subelements_map:  HashMap::<String, NodeIndex>::new(),
                    allowable_subelement_keys:  vec!("A1".to_string()),
                }
            ]
        );
    }

    #[test]
    fn test1() {
        println!("Test: test1");
        TEST_XML_DESC_TREE.validate().unwrap();
        // FIXME: Display not defined?
        // println!("XML Definition: {}", TEST_XML_DESC_TREE);
/*
        println!("Tree done");

        let input = r#"<?xml version="1.0"?>
            <XTCE xmlns="http://www.omg.org/spec/XTCE/">
                <SpaceSystem xmlns="http://www.omg.org/space/xtce" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://www.omg.org/space/xtce ../SpaceSystemV1.0.xsd" name="TrivialSat">
                <a1 />
                <a2 attr1="xyz" attr2="abc">
                </a2>
                </SpaceSystem>
            </XTCE>"#;
        println!();
        println!("Input: {}", input);

        println!();
        let cursor = Cursor::new(input);
        let buf_reader = BufReader::new(cursor);

        match XmlDocument::new_from_reader(buf_reader, &TEST_XML_DESC_TREE) {
            Err(e) => println!("Failed: {}", e),
            Ok(xml_document) => println!("XML Document: {}", xml_document),
        }
*/
    }

/*
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
        let buf_reader = BufReader::new(cursor);
        let line_reader = crate::parser::LinenoReader::new(buf_reader);
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
*/

/*
    #[test]
    fn test3() {
        println!("Test: test3");
        XSD_SCHEMA.validate();
        println!("XML Definition: {}", XSD_SCHEMA);
        println!();

        match XmlDocument::new("schema/SpaceSystem-patched.xsd",
            &TEST_XML_DESC_TREE) {
            Err(e) => println!("Failed: {}", e),
            Ok(xml_document) => println!("XML Document: {}", xml_document),
        }
    }
*/
}
