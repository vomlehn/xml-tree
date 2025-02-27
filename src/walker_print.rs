/*
 * Recursive print
 */

use std::error::Error;
use std::fmt;
//use std::marker::PhantomData;

use crate::xml_document::{Element, XmlDocument};
use crate::walkable::{Walkable, WalkableData};

type DATA<'a, 'b, RET1, RET2> = PrintWalkData<'a, 'b, RET1, RET2>;
type RET1<'a, 'b, RET1, RET2> = Result<DATA<'a, 'b, RET1, RET2>, dyn Error>;
type RET2 = std::fmt::Result;

pub struct PrintWalk<'a, 'b: 'a, DATA: WalkableData<'a, 'b, RET1, RET2>, RET1: std::ops::Try<Output = DATA>,
    RET2: std::ops::FromResidual<<RET1 as std::ops::Try>::Residual> + std::ops::Try<Output = RET2>> {
    xml_document:   &'a XmlDocument,
}

impl <'a, 'b: 'a, DATA: WalkableData<'a, 'b, RET1, RET2>,
    RET1: std::ops::Try<Output = DATA>,
    RET2: std::ops::FromResidual<<RET1 as std::ops::Try>::Residual> + std::ops::Try<Output = RET2>> 
    PrintWalk<'a, 'b, DATA, RET1, RET2> {
    pub fn new(xml_document: &'a XmlDocument) ->
    PrintWalk<'a, 'b, DATA, RET1, RET2> {
        PrintWalk::<DATA, RET1, RET2> {
            xml_document:   xml_document,
        }
    }
}  

impl<'a, 'b: 'a, DATA: WalkableData<'a, 'b, RET1, RET2>,
    RET1: std::ops::Try<Output = DATA>,
    RET2: std::ops::FromResidual<<RET1 as std::ops::Try>::Residual> + std::ops::Try<Output = RET2>> 

    Walkable<'a, 'b, DATA, RET1, RET2> for


    PrintWalk<'a, 'b, DATA, RET1, RET2> {
    fn xml_document(&self) -> &'a XmlDocument {
        self.xml_document
    }
}

pub struct PrintWalkData<'a, 'b, RET1, RET2> {
    f:          &'a mut fmt::Formatter<'a>,
    depth:      usize,
//    marker1:    PhantomData<O>,
}

impl<'a, 'b, RET1, RET2> PrintWalkData<'a, 'b, RET1, RET2> {
    pub fn new(f: &'a mut fmt::Formatter<'a>, depth: usize) ->
    PrintWalkData<'a, 'b, RET1, RET2> {
        PrintWalkData::<RET1, RET2> {
            f:          f,
            depth:      depth,
//            marker1:    PhantomData,
        }
    }
}

impl<'a, 'b, RET1, RET2> WalkableData<'a, 'b, RET1, RET2> for
PrintWalkData<'a, 'b, RET1, RET2> {
    fn element_start<'c>(&'c mut self, element: &Element) -> RET1
    where
        'a: 'c {
        Ok(PrintWalkData::new(self.f, self.depth + 1))
    }

    fn element_end(&mut self, element: &Element, subelements: Vec<RET2>) ->
        RET2 {
        Ok(())
    }
}











/*
/*
 * Tools for walking the XML document
 */
pub trait PrintWalkable<'a, 'b, I: PrintWalkableData<'a, 'b, O>, O: PrintWalkableResult>:
    Walkable<'a, 'b, I, O>
    {}


pub struct PrintWalk<'a, 'b, I: PrintWalkData<'a, 'b, O>, O: PrintWalkResult> {
    xml_document:   &'a XmlDocument,
    marker1:        PhantomData<I>,
    marker2:        PhantomData<O>,
    marker3:        PhantomData<&'b ()>,
}

impl<'a, 'b, I: PrintWalkData<'a, 'b, O>, O: PrintWalkResult>
PrintWalk<'a, 'b, I, O> {
    pub fn new(xml_document: &'a XmlDocument) -> Self {
        Self {
            xml_document:   xml_document,
            marker1:        PhantomData,
            marker2:        PhantomData,
            marker3:        PhantomData,
        }
    }
}

impl<'a, I: PrintWalkData<'a, 'a, O>, O: PrintWalkResult> Walkable<'a, 'a, I, O> for PrintWalk<'a, 'a, I, O> {
    fn xml_document(&self) -> &'a XmlDocument {
        self.xml_document
    }
}

/*
 * Tools for handling one level of printing
 */
pub trait PrintWalkableData<'a, 'b, O: PrintWalkableResult>:
    WalkableData<'a, 'b, O>
    {}

pub struct PrintWalkData<'a, O: PrintWalkableResult> {
    f:          &'a mut fmt::Formatter<'a>,
    depth:      usize,
    marker1:    PhantomData<O>,
}

impl<'a, 'b, O: PrintWalkableResult> PrintWalkData<'_, O> {
    pub fn new(f: &'a mut fmt::Formatter<'a>, depth: usize) -> PrintWalkData<'a, O> {
        PrintWalkData {
            f:          f,
            depth:      depth,
            marker1:    PhantomData,
        }
    }
}

impl<'a, O: PrintWalkableResult> WalkableData<'a, 'a, O> for
PrintWalkData<'a, O> {
    fn element_start<'c: 'a>(&'c mut self, element: &Element) ->
        Result<Self, Box<dyn Error>>
    where
        Self: Sized {
        element.display_start(self.f, self.depth)?;

        let next_data = PrintWalkData::<O>::new(self.f, self.depth + 1);
        Ok(next_data)
    }

    fn element_end(&mut self, element: &Element, _subelements: Vec<O>) ->
        Result<O, Box<dyn Error>> {
        element.display_end(self.f, self.depth)?;
        let o = PrintWalkResult {};

// FIXME: make this return OK(())
        Ok(O)
    }
}

pub trait PrintWalkableResult: WalkableResult
    {}

pub struct PrintWalkResult {
}

impl PrintWalkResult {
}

impl WalkableResult for PrintWalkResult {
}

impl PrintWalkableResult for PrintWalkResult {
}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;

    use std::error::Error;
    use std::fmt;
    use std::io::{BufReader, Read};
    use std::io::Cursor;
    use std::sync::Arc;

    use crate::walker_print::{PrintWalk, PrintWalkData};
    use crate::walker::Walk;
    use crate::xml_document::XmlDocument;
    use crate::xml_schema::{DirectElement, XmlSchema};

    #[test]
    pub fn test_print_walker() {
        let res = f::<()>();
        println!("test done: {:?}", res);
    }

    pub fn f<'a, PrintWalkResult: Default>() ->
        Result<PrintWalkResult, Box<dyn Error>> {
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

        g::<Cursor<&str>, PrintWalkResult>(buf_reader, PRINT_DESC_TREE.as_ref())
    }

    // The `g` function that receives PRINT_DESC_TREE
    pub fn g<'a, R: Read + 'a, PrintWalkResult>(
        buf_reader: BufReader<R>,
        print_xml_schema: &'a XmlSchema<'a>) ->
        Result<PrintWalkResult, Box<dyn Error>> {

        let mut outstr = String::new();
        let mut formatter = fmt::Formatter::new(&mut outstr);
        let mut f = &mut formatter;

        let print_xml_document = match XmlDocument::new_from_reader(buf_reader,
            print_xml_schema) {
            Err(e) => return Err(Box::new(e)),
            Ok(print_xml_document) => print_xml_document,
        };
        
        let mut pwd = PrintWalkData::<PrintWalkResult>::new(&mut f, 0);
        let mut w = PrintWalk::<PrintWalkData<PrintWalkResult>,
            PrintWalkResult>::new(&print_xml_document);
        w.walk(&mut pwd)?;
        println!("{}", outstr);
        Ok(())
    }
}
*/
