/*
 * Take an XML Definition tree and generate an XmlDocument
 */

use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
use std::sync::Arc;
use xml::attribute::OwnedAttribute;
use xml::common::XmlVersion;
use xml::name::OwnedName;
use xml::namespace::Namespace;
use xml::reader::XmlEvent;

//use crate::walker::Walker;
//use crate::walker_print::{PrintWalker, PrintWalkerData, PrintWalkerResult};
use crate::xml_document_error::XmlDocumentError;
use crate::xml_document_factory::XmlDocumentFactory;
use crate::xml_schema::XmlSchema;
use crate::parser::LineNumber;

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
    pub after_element:      Vec<XmlEvent>,
}

impl Element {
    pub fn new(name: OwnedName, depth: usize, element_info: ElementInfo) ->
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

    fn indent(depth: usize) -> String {
        "   ".repeat(depth)
    }

    pub fn start_string(&self, depth: usize) -> String {
        format!("{}<{}", Self::indent(depth), self.name.local_name)
    }

    pub fn attributes_string(&self) -> String {
        let mut result = "".to_string();

        for attribute in &self.element_info.attributes {
            result = result + format!(" {}=\"{}\"", attribute.name,
                attribute.value).as_str();
        }

        result
    }

    fn is_one_line(&self) -> bool {
        self.subelements.len() == 0 && self.content.len() == 0
    }

    pub fn end_first_line_string(&self) -> String {
        if self.is_one_line() {
            format!(" /> (line {})", self.element_info.lineno)
        } else {
            format!("> (line {})", self.element_info.lineno)
        }
    }

    pub fn end_n_line_string(&self, depth: usize) -> Option<String> {
        if !self.is_one_line() {
            Some(format!("{}</{}> (line {})", Self::indent(depth),
                self.name.local_name, self.element_info.lineno))
        } else {
            None
        }
    }
    
    pub fn display_start(&self, f: &mut fmt::Formatter<'_>, depth: usize) ->
        fmt::Result {
        write!(f, "{}", self.start_string(depth))?;
        write!(f, "{}", self.attributes_string())?;
        write!(f, "{}", self.end_first_line_string())
    }

    pub fn display_end(&self, f: &mut fmt::Formatter<'_>, depth: usize) ->
        fmt::Result {
        if let Some(string) = self.end_n_line_string(depth) {
            write!(f, "{}", string)?;
        }
        Ok(())
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_start(f, 0)?;
        self.display_end(f, 0)?;

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
    pub fn new<'a>(path: &str, xml_schema: &'a XmlSchema<'a>) ->
        Result<XmlDocument, XmlDocumentError> {
        let file = match File::open(path) {
            Err(e) => return Err(XmlDocumentError::Error(Arc::new(e))),
            Ok(f) => f,
        };
        let reader = BufReader::new(file);
        XmlDocument::new_from_reader(reader, xml_schema)
    }
}

impl XmlDocument {
    pub fn new_from_reader<'a, R: Read + 'a> (
        buf_reader: BufReader<R>,
        xml_schema: &'a XmlSchema<'a>) ->
        Result<XmlDocument, XmlDocumentError> {

        // Create the factory using the reader and XML definition
        let xml_document = XmlDocumentFactory::<R>::new_from_reader(buf_reader,
            xml_schema)?;
        Ok(xml_document)
    }

    fn _display_piece(&self, f: &mut fmt::Formatter<'_>, pieces: &Vec<XmlEvent>) -> fmt::Result {
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
        const INDENT_STR: &str = "   ";
        let indent_string = INDENT_STR.to_string().repeat(depth);

//        self.display_piece(f, &element.before_element)?;

        write!(f, "{}<{}", indent_string, element)?;

        if element.subelements.len() != 0 || element.content.len() != 0 {
            for element in &element.subelements {
                self.display_element(f, depth + 1, element)?;
            }

            write!(f, "{}</{}>\n", indent_string, element.name.local_name)?;
        }

        Ok(())
    }

    pub fn display(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<?xml {} {} {:?}>\n",
            self.document_info.version, self.document_info.encoding,
            self.document_info.standalone)?;

        let depth = 0;
        self.display_element(f, depth, &self.root)?;

        Ok(())
    }
}
        
/*
impl<'a> fmt::Display for XmlDocument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pwd = PrintWalkerData::<PrintWalkerResult>::new(f, 0);
        let w = PrintWalker::<PrintWalkerData<PrintWalkerResult>,
            PrintWalkerResult>::new(self);
        match w.walk(&mut pwd) {
            Err(_) => Err(fmt::Error),
            Ok(_) => Ok(()),
        }
        Ok(())
    }
}
*/

#[cfg(test)]
mod tests {
/*
    use lazy_static::lazy_static;

    use std::io::Cursor;

    use super::*;

    use crate::xml_schema::{DirectElement, SchemaElement};

    lazy_static!{
        static ref TEST_XML_DESC_TREE: XmlSchema<'static> =
            XmlSchema::new("MySchema",
                Arc::new(DirectElement::new("XTCE", vec!(
                Arc::new(DirectElement::new("SpaceSystem", vec!(
                    Arc::new(DirectElement::new("a1", vec!(
                        Arc::new(DirectElement::new("a2", vec!())),
                    ))),
                    Arc::new(DirectElement::new("a2", vec!(
                        Arc::new(DirectElement::new("a1", vec!()))
                    ))),
                ))),
            ))),
        );
    }

    lazy_static!{
        static ref TEST_MATH: XmlSchema<'static> =
            XmlSchema::new("MathSchema",
                Arc::new(DirectElement::new("Math", vec!(
                Arc::new(DirectElement::new("operand", vec!(
                    Arc::new(DirectElement::new("int", vec!())),
                ))),
                Arc::new(DirectElement::new("operator", vec!())),
            ))),
        );
    }

    #[test] #[ignore]
    fn test1() {
        println!("Test: test1");
        (*TEST_XML_DESC_TREE).validate().unwrap();
        println!("-----------------------------");
        println!("Schema:");
        println!("{}", *TEST_XML_DESC_TREE);

        println!("-----------------------------");
        println!("Input:");
        let input = r#"<?xml version="1.0"?>
            <XTCE xmlns="http://www.omg.org/spec/XTCE/">
                <SpaceSystem xmlns="http://www.omg.org/space/xtce" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://www.omg.org/space/xtce ../SpaceSystemV1.0.xsd" name="TrivialSat">
                <a1 />
                <a2 attr1="xyz" attr2="abc">
                </a2>
                </SpaceSystem>
            </XTCE>"#;
        println!("{}", input);
        println!("-----------------------------");
        println!("Parsing:");
        let cursor = Cursor::new(input);
        let buf_reader = BufReader::new(cursor);

        match XmlDocument::new_from_reader(buf_reader, &TEST_XML_DESC_TREE) {
            Err(e) => println!("Failed: {}", e),
            Ok(xml_document) => {
                println!("-----------------------------");
                println!("Result:");
                println!("{}", xml_document);
            },
        }
    }

    #[test]
    fn test2() {
        println!("Test: test2");
        (*TEST_MATH).validate().unwrap();
        println!("-----------------------------");
        println!("Schema:");
        println!("{}", *TEST_MATH);

        println!("-----------------------------");
        println!("Input:");
        let input = r#"<?xml version="1.0"?>
            <Math xmlns="http://www.omg.org/spec/XTCE/">
                <operand>
                    <int>
                        27
                    </int>
                </operand>
                <operator>
                        +
                </operator>
                <operand>
                    <int>
                        12
                    </int>
                </operand>
            </Math>"#;
        println!("{}", input);
        println!("-----------------------------");
        println!("Parsing:");
        let cursor = Cursor::new(input);
        let buf_reader = BufReader::new(cursor);

        match XmlDocument::new_from_reader(buf_reader, &TEST_MATH) {
            Err(e) => println!("Failed: {}", e),
            Ok(xml_document) => {
                println!("-----------------------------");
                println!("Result:");
                println!("{}", xml_document);
            },
        }
    }

    #[test] #[ignore]
    fn test3() {
        use crate::xsd_schema::XSD_SCHEMA;

        println!("Test: test3");
        println!("XML Definition: {}", *XSD_SCHEMA);
        println!();

        match XmlDocument::new("schema/SpaceSystem-patched.xsd",
            &XSD_SCHEMA) {
            Err(e) => println!("Failed: {}", e),
            Ok(xml_document) => println!("XML Document: {}", xml_document),
        }
    }
*/

/*
    #[test]
    fn test4() {
        println!("Test: test4");
        (*TEST_XML_DESC_TREE).validate().unwrap();
        println!("-----------------------------");
        println!("Schema:");
        println!("{}", *TEST_XML_DESC_TREE);

        println!("-----------------------------");
        println!("Input:");
        let input = r#"<?xml version="1.0"?>
            <XTCE xmlns="http://www.omg.org/spec/XTCE/">
                <SpaceSystem xmlns="http://www.omg.org/space/xtce" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://www.omg.org/space/xtce ../SpaceSystemV1.0.xsd" name="TrivialSat">
                <a1 />
                <a2 attr1="xyz" attr2="abc">
                </a2>
                </SpaceSystem>
            </XTCE>"#;
        println!("{}", input);
        println!("-----------------------------");
        println!("Parsing:");
        let cursor = Cursor::new(input);
        let buf_reader = BufReader::new(cursor);

        let xml_document = match XmlDocument::new_from_reader(buf_reader,
            &TEST_XML_DESC_TREE) {
            Err(e) => {
                println!("Failed: {}", e);
                return Err(e);
            },
            Ok(xml_document) => xml_document,
        };

        println!("-----------------------------");
//        println!("Result:");
//        println!("{}", xml_document);
        let print_item = PrintItem::new();
        let print = Print::new(print_item);
        print.walk(&xml_document);
    }
*/
}
