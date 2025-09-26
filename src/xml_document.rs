// FIXME: this has test code that needs to be put in a proper place
/*
 * Take an XML Definition tree and an input source, then use them to
 * generate an XmlTree
 */

use std::fmt;
use xml::reader::XmlEvent;

use crate::element::Element;
use crate::misc::nl_indent;
use crate::misc::XmlDisplay;
/*

#[cfg(test)]
mod tests {
/*
        use lazy_static::lazy_static;

        use std::io::Cursor;

        use super::*;

        use crate::xml_schema::{TreeElement, Element};

        lazy_static!{
            static ref TEST_XML_DESC_TREE: XmlSchema<'static> =
                XmlSchema::new("MySchema",
                    Arc::new(TreeElement::new("XTCE", vec!(
                    Arc::new(TreeElement::new("SpaceSystem", vec!(
                        Arc::new(TreeElement::new("a1", vec!(
                            Arc::new(TreeElement::new("a2", vec!())),
                        ))),
                        Arc::new(TreeElement::new("a2", vec!(
                            Arc::new(TreeElement::new("a1", vec!()))
                        ))),
                    ))),
                ))),
            );
        }

        lazy_static!{
            static ref TEST_MATH: XmlSchema<'static> =
                XmlSchema::new("MathSchema",
                    Arc::new(TreeElement::new("Math", vec!(
                    Arc::new(TreeElement::new("operand", vec!(
                        Arc::new(TreeElement::new("int", vec!())),
                    ))),
                    Arc::new(TreeElement::new("operator", vec!())),
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

            match XmlTree::new_from_reader(buf_reader, &TEST_XML_DESC_TREE) {
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

            match XmlTree::new_from_reader(buf_reader, &TEST_MATH) {
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

            match XmlTree::new_from_path("schema/SpaceSystem-patched.xsd",
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

            let xml_document = match XmlTree::new_from_reader(buf_reader,
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

#[cfg(test)]
/*
    use xml::attribute::OwnedAttribute;
    use xml::common::XmlVersion;
    use xml::name::OwnedName;
    use xml::namespace::Namespace;
    use xml::reader::XmlEvent;
*/

/**
 * Manually create an XmlTree.
 */
 // FIXME: This should be moved to a common area
pub fn create_test_doc() -> XmlTree {
    let ns: Namespace = Namespace(BTreeMap::<String, String>::new());

    let ei: ElementInfo = ElementInfo {
        lineno: 1,
        attributes: Vec::<OwnedAttribute>::new(),
        namespace: ns,
    };

    XmlTree {
        root:           branch("n1", &ei, vec![
                            leaf("n2", &ei),
                            branch("n3", &ei, vec![
                                leaf("n4", &ei)])
                        ]),
        document_info:  DocumentInfo {
                            version: XmlVersion::Version10,
                            encoding: "encoding".to_string(),
                            standalone: None,
                        },
    }
}

#[cfg(test)]
fn leaf(name: &str, ei: &ElementInfo) -> Box<dyn Element> {
    Box::new(node(name, ei, Vec::<Element>::new()))
}

#[cfg(test)]
fn branch(name: &str, ei: &ElementInfo, subelements: Vec<dyn Element>) -> Box<dyn Element> {
    Box::new(node(name, ei, subelements))
}

#[cfg(test)]
fn node(name: &str, ei: &ElementInfo, subelements: Vec<dyn Element>) -> Box<dyn Element> {
    Box::new(TreeElement {
        name: OwnedName {
            local_name: name.to_string(),
            namespace: None,
            prefix: None,
        },
        element_info: (*ei).clone(),
        subelements,
        before_element: Vec::<XmlEvent>::new(),
        content: Vec::<XmlEvent>::new(),
        after_element: Vec::<XmlEvent>::new(),
    })
}

pub trait ElemData<ED>
{
    fn next_level(&self, element: &dyn Element) -> ED;
}

/*
/**
 * Data returned by Accumulator functions.
 */
pub trait WalkData {}

/**
 * Data stored at the root level of the Walkable and a reference to which is
 * returned by the Walkable base_level_cell() function.
 */
pub trait BaseLevel {}

/**
 * Data stored for the peers of the Element a given invocation of walk_down()
 */
pub trait Accumulator<'a, BL, ED, WD, WR> {
    fn new(bl: &mut BL, e: &dyn Element, ed: &ED) -> ED
    where
        Self: Sized;
    fn add(&mut self, wd: &WD) -> WR;
    fn summary(&self, bl: &mut BL) -> WR;
}
*/
*/

impl XmlDisplay for XmlEvent {
    fn print(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        write!(f, "{}{:?}", nl_indent(depth), self)
    }
}

impl XmlDisplay for Box<dyn Element> {
    fn print(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        write!(f, "{}{:?}", nl_indent(depth), self)
    }
}
