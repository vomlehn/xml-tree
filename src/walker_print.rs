/*
 * Recursive print
 */

//use std::cell::RefCell;
use std::error::Error;
use std::fmt;
use std::marker::PhantomData;

use crate::xml_document::{Element, XmlDocument};
use crate::walker::{Walker, WalkerData};

pub struct PrintWalker<'a, 'b, I: WalkerData<'a, 'b, O>, O> {
    xml_document:   &'a XmlDocument,
    marker1:        PhantomData<I>,
    marker2:        PhantomData<O>,
    marker3:        PhantomData<&'b ()>,
}

impl<'a, 'b, I: WalkerData<'a, 'b, O>, O> PrintWalker<'a, 'b, I, O> {
    pub fn new(xml_document: &'a XmlDocument) -> Self {
        Self {
            xml_document:   xml_document,
            marker1:        PhantomData,
            marker2:        PhantomData,
            marker3:        PhantomData,
        }
    }
}

impl<'a, I: WalkerData<'a, 'a, O>, O> Walker<'a, 'a, I, O> for PrintWalker<'a, 'a, I, O> {
    fn xml_document(&self) -> &'a XmlDocument {
        self.xml_document
    }
}

pub struct PrintWalkerData<'a, O> {
    f:          &'a mut fmt::Formatter<'a>,
    depth:      usize,
    marker1:    PhantomData<O>,
}

impl<'a, O> PrintWalkerData<'a, O> {
    pub fn new(f: &'a mut fmt::Formatter<'a>, depth: usize) -> Self {
        PrintWalkerData {
            f:          f,
            depth:      depth,
            marker1:    PhantomData,
        }
    }
}

impl<'a, O: Default> WalkerData<'a, 'a, O> for PrintWalkerData<'a, O> {
    fn element_start<'c: 'a>(&'c mut self, element: &Element) ->
        Result<Self, Box<dyn Error>>
    where
        Self: Sized {
        element.display_start(self.f, self.depth)?;

        let next_data = PrintWalkerData::<O>::new(self.f, self.depth + 1);
        Ok(next_data)
    }

    fn element_end(&mut self, element: &Element, _subelements: Vec<O>) ->
        Result<O, Box<dyn Error>> {
        element.display_end(self.f, self.depth)?;

        Ok(O::default())
    }
}

pub struct PrintWalkerResult {
}

impl PrintWalkerResult {
}

impl Default for PrintWalkerResult {
    fn default() -> Self {
        Self {
        }
    }
}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;

    use std::error::Error;
    use std::fmt;
    use std::io::{BufReader, Read};
    use std::io::Cursor;
    use std::sync::Arc;

    use crate::walker_print::{PrintWalker, PrintWalkerData};
    use crate::walker::{Walker, WalkerError};
    use crate::xml_document::XmlDocument;
    use crate::xml_schema::{DirectElement, XmlSchema};

    #[test]
    pub fn test_print_walker() {
        let res = f::<()>();
        println!("test done: {:?}", res);
    }

    pub fn f<'a, PrintWalkerResult: Default>() ->
        Result<PrintWalkerResult, Box<dyn Error>> {
        let input: &'static str = r#"<?xml version="1.0"?>
            <XTCE xmlns="http://www.omg.org/spec/XTCE/">
                <SpaceSystem xmlns="http://www.omg.org/space/xtce" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://www.omg.org/space/xtce ../SpaceSystemV1.0.xsd" name="TrivialSat">
                <a1 />
                <a2 attr1="xyz" attr2="abc">
                    <a1 />
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
    pub fn g<'a, R: Read + 'a, PrintWalkerResult: Default>(
        buf_reader: BufReader<R>,
        print_xml_schema: &'a XmlSchema<'a>) ->
        Result<PrintWalkerResult, Box<dyn Error>> {

        let mut outstr = String::new();
        let mut formatter = fmt::Formatter::new(&mut outstr);
        let mut f = &mut formatter;
/*
        // Writing formatted output to a custom writer (String in this case)
        write!(f, "Hello, {}", "world")?;

        // Output will be "Hello, world"
        println!("Formatted output: {}", outstr);
*/

        let print_xml_document = match XmlDocument::new_from_reader(buf_reader,
            print_xml_schema) {
            Err(e) => return Err(Box::new(WalkerError::XmlTreeError(e))),
            Ok(print_xml_document) => print_xml_document,
        };
        
        let mut pwd = PrintWalkerData::<PrintWalkerResult>::new(&mut f, 0);
        let mut w = PrintWalker::<PrintWalkerData<PrintWalkerResult>,
            PrintWalkerResult>::new(&print_xml_document);
        w.walk(&mut pwd)
    }
}
