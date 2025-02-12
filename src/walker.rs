/*
 * XML tree walker
 */

use lazy_static::lazy_static;
use thiserror;
use std::error;
use std::io::{BufReader, Read};
use std::io::Cursor;
use std::marker::PhantomData;
use std::process;
use std::rc::Rc;
use std::sync::Arc;

use crate::xml_document::{Element, XmlDocument};
use crate::xml_schema::{DirectElement, XmlSchema};

#[derive(Debug, thiserror::Error)]
    pub enum WalkerError {
    // Need full path to faulty element
    #[error("Unknown element \"{0}\"")]
    UnknownElement(String),
}

trait WalkData<I, O> {
    fn element_start(&self, element: &Element) ->
        Result<Self, WalkerError>
        where
            Self: Sized;
    fn element_end(&self, element: &Element, subelements: Vec<O>) ->
        Result<O, WalkerError>;
}

struct Walker<'a, I: WalkData<I, O>, O> {
    xml_document:   &'a XmlDocument,
    marker1:        PhantomData<I>,
    marker2:        PhantomData<O>,
}

impl<'a, I: WalkData<I, O>, O> Walker<'a, I, O> {
    fn new(xml_document: &'a XmlDocument) -> Self {
        Walker::<I, O> {
            xml_document:   xml_document,
            marker1:        PhantomData,
            marker2:        PhantomData,
        }
    }
        
    fn walk(&self, element_data: &I) -> Result<O, WalkerError> {
        self.walk_n(&self.xml_document.root, element_data)
    }

    fn walk_n<'b>(&self, element: &Element, element_data: &I) ->
        Result<O, WalkerError> {
        let subelement_data = element_data.element_start(element)?;
        let mut subelements = Vec::<O>::new();

        for subelement in &element.subelements {
            let subdata = self.walk_n(&subelement, &subelement_data)?;
            subelements.push(subdata);
        } 

        element_data.element_end(element, subelements)
    }
}

// FIXME: move to its own file
// --------------------------------
struct PrintWalkData<I, O> {
    depth:      usize,
    marker1:    PhantomData<I>,
    marker2:    PhantomData<O>,
}

impl<I, O> PrintWalkData<I, O> {
    pub fn new(depth: usize) -> Self {
        Self {
            depth:      depth,
            marker1:    PhantomData,
            marker2:    PhantomData,
        }
    }
}

impl<I, O: Clone> WalkData<I, O> for PrintWalkData<I, O> {
    fn element_start(&self, element: &Element) ->
        Result<PrintWalkData<I, O>, WalkerError> {
        println!("{}", element.name);
        let next_data = PrintWalkData::<I, O>::new(self.depth + 1);
        Ok(next_data)
    }

    fn element_end(&self, _element: &Element, subelements: Vec<O>) ->
        Result<O, WalkerError> {
        Ok(subelements[0].clone())
    }
}

/*
lazy_static!{
    static ref PRINT_DESC_TREE: XmlSchema<'static> =
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

    static ref input: &'static str = r#"<?xml version="1.0"?>
        <XTCE xmlns="http://www.omg.org/spec/XTCE/">
            <SpaceSystem xmlns="http://www.omg.org/space/xtce" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://www.omg.org/space/xtce ../SpaceSystemV1.0.xsd" name="TrivialSat">
            <a1 />
            <a2 attr1="xyz" attr2="abc">
            </a2>
            </SpaceSystem>
        </XTCE>"#;

    static ref cursor: Cursor<&'static str> = Cursor::new(*input);
    static ref buf_reader: BufReader<R: Read> = BufReader::<R>::new(cursor);

    static ref print_xml_document: XmlDocument =
        match XmlDocument::new_from_reader(buf_reader,
            &PRINT_DESC_TREE) {
            Err(e) => {
                println!("Failed: {}", e);
                process::exit(1);
            },
            Ok(xml_document) => xml_document,
        }            ;

    static ref print_walker:
        Walker<'static, PrintWalkData<(), ()>, ()> =
        Walker::<PrintWalkData<()>, ()>::new(&print_xml_document);

}
*/

/*
pub fn f<'a, R: Read, I: WalkData<I, O>, O>() {
    let input: &'static str = r#"<?xml version="1.0"?>
        <XTCE xmlns="http://www.omg.org/spec/XTCE/">
            <SpaceSystem xmlns="http://www.omg.org/space/xtce" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://www.omg.org/space/xtce ../SpaceSystemV1.0.xsd" name="TrivialSat">
            <a1 />
            <a2 attr1="xyz" attr2="abc">
            </a2>
            </SpaceSystem>
        </XTCE>"#;

    let PRINT_DESC_TREE: Arc<XmlSchema> = Arc::new(XmlSchema::new("MySchema",
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
    ));

    let cursor = Cursor::new(input);
    let buf_reader = BufReader::new(cursor);
    g::<I, O>(buf_reader, PRINT_DESC_TREE.clone());
}

pub fn g<'a, I: WalkData<I, O>, O>(buf_reader: BufReader<Cursor<&'a str>>,
    print_xml_schema: Arc<XmlSchema<'a>>) {

    let print_xml_document = XmlDocument::new_from_reader(buf_reader,
        &print_xml_schema);

    match print_xml_document {
        Err(e) => {
            println!("Failed: {}", e);
            process::exit(1);
        },
        Ok(xml_document) => {
            let print_xml_document = xml_document;
            let pwd = PrintWalkData::<(), ()>::new(0);
            let _w = Walker::<I, O>::new(&print_xml_document);
            process::exit(1);
        }
    };
}
*/

/*
use std::sync::Arc;
use std::io::{Read, BufReader, Cursor};
use std::process;

pub struct XmlSchema<'a> {
    name: &'a str,
    element: Arc<DirectElement<'a>>,
}

pub struct DirectElement<'a> {
    name: &'a str,
    children: Vec<Arc<DirectElement<'a>>>,
}

impl<'a> XmlSchema<'a> {
    fn new(name: &'a str, element: Arc<DirectElement<'a>>) -> XmlSchema<'a> {
        XmlSchema { name, element }
    }
}

impl<'a> DirectElement<'a> {
    fn new(name: &'a str, children: Vec<Arc<DirectElement<'a>>>) -> DirectElement<'a> {
        DirectElement { name, children }
    }
}
*/

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;

    use std::io::Cursor;

    use super::*;

    use crate::xml_schema::{DirectElement, SchemaElement};
    // Your `f` function, which prepares and passes data to `g`
    pub fn f<'a, R: Read, I: WalkData<I, O>, O>() {
        let input: &'static str = r#"<?xml version="1.0"?>
            <XTCE xmlns="http://www.omg.org/spec/XTCE/">
                <SpaceSystem xmlns="http://www.omg.org/space/xtce" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://www.omg.org/space/xtce ../SpaceSystemV1.0.xsd" name="TrivialSat">
                <a1 />
                <a2 attr1="xyz" attr2="abc">
                </a2>
                </SpaceSystem>
            </XTCE>"#;

        lazy_static!{
            // Wrap PRINT_DESC_TREE in Arc to extend its lifetime and share ownership
            static ref PRINT_DESC_TREE: Arc<XmlSchema<'static>> =
                Arc::new(XmlSchema::new("MySchema",
                Arc::new(DirectElement::new("XTCE", vec![
                    Arc::new(DirectElement::new("SpaceSystem", vec![
                        Arc::new(DirectElement::new("a1", vec![
                            Arc::new(DirectElement::new("a2", vec![])),
                        ])),
                        Arc::new(DirectElement::new("a2", vec![
                            Arc::new(DirectElement::new("a1", vec![])),
                        ])),
                    ])),
                ])),
            ));
        }

        let cursor = Cursor::new(input);
        let buf_reader = BufReader::new(cursor);

        // Pass the Arc-wrapped XmlSchema to function `g` directly (no need to reference it)
        g::<I, O>(buf_reader, PRINT_DESC_TREE.as_ref());
    }

    // The `g` function that receives PRINT_DESC_TREE
    pub fn g<'a, I: WalkData<I, O>, O>(buf_reader: BufReader<Cursor<&'static str>>, print_xml_schema: &'a XmlSchema<'a>) {
        // Directly use the Arc without borrowing
    /* FIXME: remove
        pub fn new_from_reader<'a, R: Read + 'a> (
            buf_reader: BufReader<R>,
            xml_schema: &'a XmlSchema<'a>) ->
            Result<XmlDocument, XmlDocumentError> {
    */
        let print_xml_document = XmlDocument::new_from_reader(buf_reader, print_xml_schema);

        match print_xml_document {
            Err(e) => {
                println!("Failed: {}", e);
                process::exit(1);
            },
            Ok(xml_document) => {
                let print_xml_document = xml_document;
                let pwd = PrintWalkData::<(), ()>::new(0);
                let _w = Walker::<I, O>::new(&print_xml_document);
                process::exit(1);
            }
        };
    }
}
