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

// FIXME: need these be pub?
pub trait ElemResult: Try + FromResidual {}
pub trait WalkResult<T>: Try + FromResidual<<T as Try>::Residual> {}

pub trait Accumulator<'a, BL, ED, ER, WD, WR>
where
    BL: BaseLevel,
    ED: ElemData<ER>,
    ER: ElemResult,
    WD: WalkData,
    WR: WalkResult,
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
    ER: ElemResult<Output = ED>,
    WD: WalkData,
    WR: WalkResult<Output = WD>,
//    WR: Try<Output = WD>,
//    WR: FromResidual<<ER as Try>::Residual>,
//    WR: FromResidual<Result<Infallible, WalkError>>,
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
    use super::{ElemResult, WalkResult};

    const INDENT: &str = "    ";

    pub struct TestAccumulator {
        result: String,
    }

    impl<'a> Accumulator<'a, TestBaseLevel, TestElemData, Result<TestElemData, WalkError>, TestWalkData, Result<TestWalkData, WalkError>>
    for TestAccumulator {
        fn new(_bl: &TestBaseLevel, e: &'a Element, ed: &TestElemData) -> Self {
            
            let result = format!("{}{}", INDENT.repeat(ed.depth), e.name.local_name);
            TestAccumulator { result: result }
        }

        fn add(&mut self, _bl: &TestBaseLevel, wd: &TestWalkData) -> 
            Result<(), WalkError> {
            self.result += &format!("\n{}", wd.data);
// FIXME: don't really love the cloning
            Result::Ok(())
        }

        fn summary(&self, _bl: &TestBaseLevel) -> Result<TestWalkData, WalkError> {
            Result::Ok(TestWalkData {
// FIXME: don't really love the cloning
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
        depth:  usize,
    }

    impl TestElemData {
        fn new(depth: usize) -> TestElemData {
            TestElemData {
                depth:  depth,
            }
        }
    }

    impl ElemData<Result<TestElemData, WalkError>> for TestElemData {
        fn next_level(&self, _element: &Element) -> Result<TestElemData, WalkError> {
            Result::Ok(TestElemData::new(self.depth + 1))
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

    type TestElemResult = Result<TestElemData, WalkError>;
    impl ElemResult for TestElemResult {}


    type TestWalkResult = Result<TestWalkData, WalkError>;
    impl WalkResult for TestWalkResult {}

    impl<'a> Walkable<'a, TestBaseLevel, TestAccumulator, TestElemData, 
        TestElemResult, TestWalkData, TestWalkResult> 
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
    fn test_walk_tree_names() {
        println!("\nStart test_walk_tree_names");
        let test_doc = create_test_doc();
        let test_base_level = TestBaseLevel::new();
        let test_walk = TestWalkable::new(&test_doc, test_base_level);
        
        let test_elem_data = TestElemData::new(0);
        let expected = "n1\nn2\nn3\nn4";

        let result = test_walk.walk(&test_elem_data);
        let result = match result {
            Err(e) => panic!("Failed: err({:?})", e),
            Ok(result) => result,
        };
        println!("Expected {}", expected);
        println!("Actual {}", result.data);
        assert_eq!(expected, result.data);
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

const INDENT: &str = "    ";

// Revised implementation to fix lifetime issues

// WalkData implementation for printing
type PrintWalkData = ();
impl WalkData for PrintWalkData {}

// ElemData implementation with formatter
pub struct PrintElemData {
    pub depth: usize,
}

impl PrintElemData {
    fn new(depth: usize) -> Self {
        PrintElemData {
            depth: depth,
        }
    }
}

impl ElemData<Result<PrintElemData, WalkError>> for PrintElemData {
    fn next_level(&self, _element: &Element) -> Result<PrintElemData, WalkError> {
        Ok(PrintElemData::new(self.depth + 1))
    }
}

// PrintBaseLevel implementation
struct PrintBaseLevel<'a> {
    f: &'a mut fmt::Formatter<'a>,
}

impl<'a> PrintBaseLevel<'a> {
    pub fn new(f: &'a mut fmt::Formatter<'a>) -> Self {
        PrintBaseLevel {
            f: f,
        }
    }
}

impl<'a> BaseLevel for PrintBaseLevel<'a> {
}

// Accumulator for printing
pub struct PrintAccumulator {
}

impl<'a> Accumulator<'a, PrintBaseLevel<'a>, PrintElemData, Result<PrintElemData, WalkError>, PrintWalkData, fmt::Result>
    for PrintAccumulator {
    fn new(_bl: &PrintBaseLevel, _e: &'a Element, _ed: &PrintElemData) -> Self {
        PrintAccumulator { 
        }
    }

    fn add(&mut self, _bl: &PrintBaseLevel, _wd: &PrintWalkData) ->
        Result<(), WalkError> {
        Ok(())
    }

    fn summary(&self, _bl: &PrintBaseLevel) -> fmt::Result {
        Ok(())
    }
}

// Wrapper type for fmt::Result
pub struct PrintResult(pub fmt::Result);

// Implement FromResidual for wrapper type
impl FromResidual<Result<Infallible, WalkError>> for PrintResult {
    fn from_residual(_residual: Result<Infallible, WalkError>) -> Self {
        // Any error becomes fmt::Error
        PrintResult(Err(fmt::Error))
    }
}

// Implement conversion from PrintResult to fmt::Result
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

// Define result types
type PrintElemResult = Result<PrintElemData, WalkError>;
impl ElemResult for PrintElemResult {}

type PrintWalkResult = fmt::Result;
impl WalkResult for PrintWalkResult {}

// The formatter-based walker - UPDATED VERSION THAT FIXES LIFETIME ISSUES
struct WalkAndPrint<'a, 'fmt> {
    xml_document: &'a XmlDocument,
    base_level: PrintBaseLevel<'fmt>,
}

impl<'a, 'fmt> WalkAndPrint<'a, 'fmt> {
    pub fn new<'b>(xml_document: &'a XmlDocument, f: &'fmt mut fmt::Formatter<'fmt>) -> Self {
        let base_level = PrintBaseLevel::new(f);
        WalkAndPrint {
            xml_document,
            base_level,
        }
    }
}

// Implement Walkable for WalkAndPrint with correct lifetime parameters
impl<'a, 'fmt> Walkable<'fmt, PrintBaseLevel<'fmt>, PrintAccumulator, PrintElemData, 
    PrintElemResult, PrintWalkData, PrintWalkResult> 
for WalkAndPrint<'a, 'fmt>
where
    'a: 'fmt, // This ensures xml_document outlives the formatter reference
{
    fn xml_document(&self) -> &XmlDocument {
        self.xml_document
    }
    
    fn base_level(&'fmt self) -> &'fmt PrintBaseLevel<'fmt> {
        &self.base_level
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

    use super::{PrintElemData, WalkAndPrint};
    use super::Walkable;

    struct TestWalkObj<'a> {
        xml_doc: &'a XmlDocument,
    }

    impl<'a> TestWalkObj<'a> {
        fn new(doc: &'a XmlDocument) -> TestWalkObj<'a> {
            TestWalkObj {
                xml_doc: doc,
            }
        }
    }

    // FIXED implementation - removed explicit lifetime parameters to match trait declaration
    impl<'a> fmt::Display for TestWalkObj<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
        {
            // Create WalkAndPrint directly with the formatter
            let wap = WalkAndPrint::new(self.xml_doc, f);
            
            let ped = PrintElemData::new(0);
            wap.walk(&ped)
        }
    }

    #[test]
    fn test_walk_tree_print() {
        println!("\nStart test_walk_tree_print");
        let test_doc = create_test_doc(); // build a sample XmlDocument
        println!("doc: {}", test_doc);
        let test_walk = TestWalkObj::new(&test_doc);
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
            subelements,
            before_element: Vec::<XmlEvent>::new(),
            content: Vec::<XmlEvent>::new(),
            after_element: Vec::<XmlEvent>::new(),
        }
    }
}
