use std::fmt;
use std::boxed::Box;
use std::convert::Infallible;
use std::error::Error;
use std::ops::{FromResidual, Try};
use std::ops::{ControlFlow};
use std::marker::{Send, Sync};
use std::result::Result;

use crate::xml_document::{Element, XmlDocument};

// From Claude Code
pub type WalkError = Box<dyn Error + Send + Sync + 'static>;

// ----------------- Traits ----------------
// Information that supplements the Element to produce a piece of the overall
// result.
pub trait ElemData<ER>
{
    fn next_level(&self, element: &Element) -> ER;
}

pub trait WalkData {}

pub trait BaseLevel {}

pub trait Accumulator<'a, BL, ED, ER, WD, WR>
where
    BL: BaseLevel,
    ED: ElemData<ER>,
    ER: Try,
    WD: WalkData,
    WR: Try,
{
    fn new(_bl: &BL, e: &'a Element, ed: &ED) -> Self
    where
        Self: Sized;
    fn add(&mut self, _bl: &BL, wd: &WD) -> Result<(), WalkError>;
    fn summary(&self, _bl: &BL) -> WR;
}

pub trait Walkable<'a, BL, AC, ED, ER, WD, WR>
where
    BL: BaseLevel + 'a,
    AC: Accumulator<'a, BL, ED, ER, WD, WR>,
    ED: ElemData<ER>,
    ER: Try<Output = ED>,
    WD: WalkData,
    WR: Try<Output = WD>,
    WR: FromResidual<<ER as Try>::Residual>,
    WR: FromResidual<Result<Infallible, WalkError>>,
{
    fn xml_document(&self) -> &XmlDocument;
    fn base_level(&'a self) -> &'a BL;
    
    // Start the walk at the root of the document
    fn walk<'b: 'a>(&'b self, d: &ED) -> WR
    where   
        Self: Sized,
    {       
        let xml_doc = self.xml_document();
        let root = &xml_doc.root;
        self.walk_down(root, d)
    }

    fn walk_down<'b: 'a, 'c>(&'b self, element: &'a Element, ed: &ED) -> WR
    where
        Self: Sized,
    {
        let mut acc = AC::new(self.base_level(), element, ed);

        // Process subelements and collect WalkData results
        let mut wd_vec = Vec::<WD>::new();

        for elem in &element.subelements {
            let next_ed = ed.next_level(elem)?;
            let wd = self.walk_down(elem, &next_ed)?;
            wd_vec.push(wd);
        }

        // Accumulate results
        for wd in &wd_vec {
            acc.add(self.base_level(), wd)?;
        }

        acc.summary(self.base_level())
    }

}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use xml::attribute::OwnedAttribute;
    use xml::common::XmlVersion;
    use std::fmt;
    use xml::name::OwnedName;
    use xml::namespace::Namespace;
    use xml::reader::XmlEvent;

    use crate::xml_document::{Element, XmlDocument};
    use crate::xml_document_factory::{DocumentInfo, ElementInfo};
    use crate::walkable::{/*PrintResult, */BaseLevel};
    use super::{/*PrintElemData*//*, WalkAndPrint, */Walkable};
    use super::Accumulator;
    use super::WalkError;
    use super::{ElemData, WalkData};

    const INDENT: &str = "    ";

    pub struct TestAccumulator {
        result: String,
    }

    impl<'a> Accumulator<'a, TestBaseLevel, TestElemData, Result<TestElemData, WalkError>, TestWalkData, Result<TestWalkData, WalkError>>
    for TestAccumulator {
        fn new(_bl: &TestBaseLevel, e: &'a Element, ed: &TestElemData) -> Self {
            let result = format!("{}{}", INDENT.repeat(ed.depth), e.name.local_name);
            TestAccumulator { result }
        }

        fn add(&mut self, _bl: &TestBaseLevel, wd: &TestWalkData) -> 
            Result<(), WalkError> {
            self.result += &format!("\n{}", wd.data);
            Result::Ok(())
        }

        fn summary(&self, _bl: &TestBaseLevel) -> Result<TestWalkData, WalkError> {
            Result::Ok(TestWalkData {
                data: self.result.clone(),
            })
        }
    }

    #[derive(Debug)]
    pub struct TestWalkData {
        pub data: String,
    }

    impl WalkData for TestWalkData {}

    #[derive(Debug)]
    pub struct TestElemData {
        pub depth: usize,
    }

    impl TestElemData {
        fn new(depth: usize) -> TestElemData {
            TestElemData {
                depth:  depth,
            }
        }
    }

    impl ElemData<Result<TestElemData, WalkError>> for TestElemData {
        fn next_level(&self, element: &Element) -> Result<TestElemData, WalkError> {
            println!("{}{}", INDENT.repeat(self.depth), element.name.local_name);
            Result::Ok(TestElemData::new(self.depth + 1))
        }
    }

    struct TestWalk<'a> {
        xml_doc: &'a XmlDocument,
    }

    impl<'a> TestWalk<'a> {
        fn new(doc: &'a XmlDocument) -> Self {
            TestWalk {
                xml_doc: doc,
            }
        }
    }

    struct TestBaseLevel {
    }

    impl TestBaseLevel {
        pub fn new() -> Self {
            TestBaseLevel {}
        }
    }

    impl BaseLevel for TestBaseLevel {
    }

    impl<'a> fmt::Display for TestWalk<'a> {
        fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
/*
// FIXME: can I use () instread of base?
            let base = TestBaseLevel::new();
            let walker = TestWalkable::new(&self.xml_doc, base);
            let ped = PrintElemData::new(0);
            
//    FIXME: implement this
            // Get the PrintResult from walk
            let result: PrintResult = Walkable::<TestBaseLevel, TestAccumulator, PrintElemData, Result<PrintElemData, WalkError>, PrintWalkData, fmt::Result>::walk(&walker, &mut ped);
            
            // Convert PrintResult to fmt::Result
            result.into();
*/
            todo!()
        }
    }

    struct TestWalkable<'a> {
        xml_document:   &'a XmlDocument,
        base:           TestBaseLevel,
    }

    impl<'e, 'a> TestWalkable<'a> {
        fn new(xml_document: &'a XmlDocument, base: TestBaseLevel) -> Self {
            TestWalkable {
                xml_document:   xml_document,
                base:           base,
            }
        }
    }


    impl<'a> Walkable<'a, TestBaseLevel, TestAccumulator, TestElemData, Result<TestElemData, WalkError>, TestWalkData, Result<TestWalkData, WalkError>> 
    for TestWalkable<'a>
    {
        fn xml_document(&self) -> &XmlDocument {
            self.xml_document
        }
        fn base_level(&'a self) -> &'a TestBaseLevel {
            &self.base
        }
    }

    #[test]
    fn test_walk_tree_print() {
        println!("\nStart test_walk_tree_print");
        let test_doc = create_test_doc();
        let test_walk = TestWalk::new(&test_doc);
        
        // This will print using Display
        println!("XML Tree: {}", test_walk);
    }

    fn create_test_doc() -> XmlDocument {
        let ns: Namespace = Namespace(BTreeMap::<String, String>::new());

        let ei: ElementInfo = ElementInfo {
            lineno: 1,
            attributes: Vec::<OwnedAttribute>::new(),
            namespace: ns,
        };

        let e1 = branch("n1", &ei, vec![
            leaf("n2", &ei),
            branch("n3", &ei, vec![
                leaf("n4", &ei)])
        ]);

        let di = DocumentInfo {
            version: XmlVersion::Version10,
            encoding: "xxx".to_string(),
            standalone: None,
        };

        let d: XmlDocument = XmlDocument {
            root: e1,
            document_info: di,
        };

        d
    }

    fn leaf(name: &str, ei: &ElementInfo) -> Element {
        node(name, ei, Vec::<Element>::new())
    }

    fn branch(name: &str, ei: &ElementInfo, subelements: Vec<Element>) -> Element {
        node(name, ei, subelements)
    }

    fn node(name: &str, ei: &ElementInfo, subelements: Vec<Element>) -> Element {
        Element {
            name: OwnedName {
                local_name: name.to_string(),
                namespace: None,
                prefix: None,
            },
            element_info: ei.clone(),
            subelements,
            before_element: Vec::<XmlEvent>::new(),
            content: Vec::<XmlEvent>::new(),
            after_element: Vec::<XmlEvent>::new(),
        }
    }
}

/*
 * Recursive print
 */

//use std::fmt;
//use std::convert::Infallible;

//use crate::xml_document::{Element, XmlDocument};
//use super::{Accumulator, ElemData, WalkData, WalkError, Walkable};

const INDENT: &str = "    ";

// The formatter-based walker
struct WalkAndPrint<'a> {
    xml_document:   &'a XmlDocument,
    base:           PrintBaseLevel<'a>,
}

impl<'a> WalkAndPrint<'a> {
    pub fn new(xml_document: &'a XmlDocument, base: PrintBaseLevel<'a>) -> Self {
        WalkAndPrint {
            xml_document:   xml_document,
            base:           base,
        }
    }
}

// WalkData implementation for printing
pub type PrintWalkData = ();

impl WalkData for PrintWalkData {}

// ElemData implementation with formatter
pub struct PrintElemData {
    pub depth:  usize,
}

impl PrintElemData {
    fn new(depth: usize) -> Self {
        PrintElemData {
            depth:  depth,

        }
    }
}

impl ElemData<Result<PrintElemData, WalkError>> for PrintElemData {
    fn next_level(&self, _element: &Element) -> Result<PrintElemData, WalkError> {
        Ok(PrintElemData::new(self.depth + 1))
    }
}

// Accumulator for printing
pub struct PrintAccumulator {
    depth: usize,
}


// FIXME:
// Replace the direct implementation of FromResidual with this wrapper approach:

// 1. Create a wrapper type for fmt::Result
pub struct PrintResult(pub fmt::Result);

// 2. Implement FromResidual for your wrapper type
impl FromResidual<Result<Infallible, WalkError>> for PrintResult {
    fn from_residual(_residual: Result<Infallible, WalkError>) -> Self {
        // Any error becomes fmt::Error
        PrintResult(Err(fmt::Error))
    }
}

// 3. Implement conversion from PrintResult to fmt::Result
impl From<PrintResult> for fmt::Result {
    fn from(result: PrintResult) -> fmt::Result {
        result.0
    }
}

impl Try for PrintResult {
    type Output = ();
    type Residual = Box<dyn Error>;

    fn from_output(_: <Self as Try>::Output) -> Self {
        todo!()
    }
    fn branch(self) -> ControlFlow<<Self as Try>::Residual, <Self as Try>::Output> {
        todo!()
    }
}

impl FromResidual<Box<(dyn std::error::Error + 'static)>> for PrintResult {
    fn from_residual(_: Box<(dyn std::error::Error + 'static)>) -> Self {
        todo!()
    }
}

struct PrintBaseLevel<'a> {
    f:  &'a mut fmt::Formatter<'a>,
}

impl<'a> PrintBaseLevel<'a> {
    pub fn new(f: &'a mut fmt::Formatter<'a>) -> Self {
        PrintBaseLevel {
            f:  f,
        }
    }
}

impl<'a> BaseLevel for PrintBaseLevel<'a> {
}

// 4. Update your Accumulator trait implementation
impl<'a> Accumulator<'a, PrintBaseLevel<'_>, PrintElemData, Result<PrintElemData, WalkError>, PrintWalkData, PrintResult>
    for PrintAccumulator {
    fn new(_bl: &PrintBaseLevel, _e: &'a Element, ed: &PrintElemData) -> Self {
        PrintAccumulator { 
            depth:  0,
        }
    }

    fn add(&mut self, _bl: &PrintBaseLevel, _wd: &PrintWalkData) -> Result<(), WalkError> {
        Ok(())
    }

    fn summary(&self, _bl: &PrintBaseLevel) -> PrintResult {
        // Return your wrapped result
        PrintResult(Ok(()))
    }
}

#[cfg(test)]
mod tests2 {
    use std::collections::BTreeMap;
    use xml::attribute::OwnedAttribute;
    use xml::common::XmlVersion;
    use std::fmt;
    use xml::name::OwnedName;
    use xml::namespace::Namespace;
    use xml::reader::XmlEvent;

    use crate::xml_document::{Element, XmlDocument};
    use crate::xml_document_factory::{DocumentInfo, ElementInfo};

    struct TestWalk<'a> {
        xml_doc: &'a XmlDocument,
    }

    impl<'a> TestWalk<'a> {
        fn new(doc: &'a XmlDocument) -> TestWalk<'a> {
            TestWalk {
                xml_doc: doc,
            }
        }
    }

    impl<'a> fmt::Display for TestWalk<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "TestWalk::Display not done");
            todo!();
        }
    }

    #[test]
    fn test_walk_tree_print() {
        println!("\nStart test_walk_tree_print");
        let test_doc = create_test_doc(); // build a sample XmlDocument
        println!("doc: {}", test_doc);
        let test_walk = TestWalk::new(&test_doc);
        println!("walk: {}", test_walk);
    }

    fn create_test_doc() -> XmlDocument {
        let ns: Namespace = Namespace(BTreeMap::<String, String>::new());

        let ei: ElementInfo = ElementInfo {
            lineno: 1,
            attributes: Vec::<OwnedAttribute>::new(),
            namespace: ns,
        };

        let e1 = branch("n1", &ei, vec![
            leaf("n2", &ei),
            branch("n3", &ei, vec![
                leaf("n4", &ei)])
        ]);

        let di = DocumentInfo {
            version: XmlVersion::Version10,
            encoding: "xxx".to_string(),
            standalone: None,
        };

        let d: XmlDocument = XmlDocument {
            root: e1,
            document_info: di,
        };

        d
    }

    fn leaf(name: &str, ei: &ElementInfo) -> Element {
        node(name, ei, Vec::<Element>::new())
    }

    fn branch(name: &str, ei: &ElementInfo, subelements: Vec<Element>) -> Element {
        node(name, ei, subelements)
    }

    fn node(name: &str, ei: &ElementInfo, subelements: Vec<Element>) -> Element {
        Element {
            name: OwnedName {
                local_name: name.to_string(),
                namespace: None,
                prefix: None,
            },
            element_info: ei.clone(),
            subelements: subelements,
            before_element: Vec::<XmlEvent>::new(),
            content: Vec::<XmlEvent>::new(),
            after_element: Vec::<XmlEvent>::new(),
        }
    }
}
