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
use crate::xml_document_error::XmlDocumentError;
use crate::xml_schema::{DirectElement, XmlSchema};

#[derive(Debug, thiserror::Error)]
    pub enum WalkerError {
    // Need full path to faulty element
    #[error("Unknown element \"{0}\"")]
    UnknownElement(String),

    #[error("XMLDocumentError: \"{0}\"")]
    XmlTreeError(XmlDocumentError),
}

trait WalkerData<O> {
    fn element_start(&self, element: &Element) ->
        Result<Self, WalkerError>
        where
            Self: Sized;
    fn element_end(&self, element: &Element, subelements: Vec<O>) ->
        Result<O, WalkerError>;
}

trait WalkerResult {
}

struct Walker<'a, I: WalkerData<O>, O> {
    xml_document:   &'a XmlDocument,
    marker1:        PhantomData<I>,
    marker2:        PhantomData<O>,
}

impl<'a, I: WalkerData<O>, O> Walker<'a, I, O> {
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
#[derive(Clone, Debug)]
struct PrintWalkerData<O> {
    depth:      usize,
    marker1:    PhantomData<O>,
}

impl<O> PrintWalkerData<O> {
    pub fn new(depth: usize) -> Self {
        Self {
            depth:      depth,
            marker1:    PhantomData,
        }
    }
}

impl<O: Clone + Default> WalkerData<O> for PrintWalkerData<O> {
    fn element_start(&self, element: &Element) ->
        Result<PrintWalkerData<O>, WalkerError> {
        println!("{}{}{}", element.start_string(self.depth),
            element.attributes_string(), element.end_first_line_string());
        let next_data = PrintWalkerData::<O>::new(self.depth + 1);
        Ok(next_data)
    }

    fn element_end(&self, element: &Element, subelements: Vec<O>) ->
        Result<O, WalkerError> {

        if let Some(string) = element.end_n_line_string(self.depth) {
            println!("{}", string);
        }

        Ok(O::default())
    }
}

#[derive(Clone)]
struct PrintWalkerResult {
    dummy:  i8,
}

impl PrintWalkerResult {
}

impl Default for PrintWalkerResult {
    fn default() -> Self {
        Self {
            dummy: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;

    use std::io::Cursor;

    use super::*;

    use crate::xml_schema::{DirectElement, SchemaElement};

    // Your `f` function, which prepares and passes data to `g`
    #[test]
    pub fn test_print_walker() {
        let res = f::<()>();
        println!("test done: {:?}", res);
    }

    pub fn f<'a, PrintWalkerResult: Clone + Default>() ->
        Result<PrintWalkerResult, WalkerError> {
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

        g::<Cursor<&str>, PrintWalkerResult>(buf_reader, PRINT_DESC_TREE.as_ref())
    }

    // The `g` function that receives PRINT_DESC_TREE
    pub fn g<'a, R: Read + 'a, PrintWalkerResult: Clone + Default>(
        buf_reader: BufReader<R>,
        print_xml_schema: &'a XmlSchema<'a>) ->
        Result<PrintWalkerResult, WalkerError> {

        let print_xml_document = match XmlDocument::new_from_reader(buf_reader,
            print_xml_schema) {
            Err(e) => return Err(WalkerError::XmlTreeError(e)),
            Ok(print_xml_document) => print_xml_document,
        };
        

        let pwd = PrintWalkerData::<PrintWalkerResult>::new(0);
        let w = Walker::<PrintWalkerData<PrintWalkerResult>,
            PrintWalkerResult>::new(&print_xml_document);
        w.walk(&pwd)
    }
}
