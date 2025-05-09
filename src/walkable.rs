use std::boxed::Box;
use std::convert::Infallible;
use std::error::Error;
use std::ops::{ControlFlow, FromResidual, Try};
use std::marker::{Send, Sync};
use std::result::Result;

use crate::xml_document::{Element, XmlDocument};

// From Claude Code
pub type WalkError = Box<dyn Error + Send + Sync + 'static>;
//pub type WalkableResult<E, WD> = Result<WD, E>;

//pub trait WalkableResult: Try + FromResidual<Result<Infallible, Box<dyn Error + Send + Sync>>> {}

// ----------------- Traits ----------------
// Information that supplements the Element to produce a piece of the overall
// result.
pub trait ElemData {
    type Output;
    
    fn next_level(&self, element: &Element) -> Result<Self::Output, WalkError>;
}

pub trait WalkData {}

pub trait Accumulator<'a, ED, WD, R> 
where
    ED: ElemData,
    WD: WalkData,
    // This is what Claude Code gave us
//    R: Try + FromResidual<Result<Infallible, WalkError>>,
    // Then, it suggested this:
    R:  Try<Output = WD> + FromResidual<Result<Infallible, WalkError>>,
{
    fn new(e: &Element, ed: &ED) -> Self
    where
        Self: Sized;
    fn add(&mut self, wd: &WD) -> Result<(), WalkError>;
    fn summary(&self) -> R;
}

pub trait Walkable<'a, AC, ED, WD, R>
where
    AC: Accumulator<'a, ED, WD, R>,
    ED: ElemData<Output = ED>,  // This restriction ensures ED::Output is the same as ED
    WD: WalkData,
//    R:  WalkableResult<Output = WD>,
//    R:  WalkableResult,
//    R:  Try + FromResidual<Result<Infallible, Box<dyn Error + Send + Sync>>>,
    // This is what Claude Code gave us
//    R: Try + FromResidual<Result<Infallible, WalkError>>,
    // Then, it suggested this:
//    R:  Try + FromResidual<Result<T, WalkError>>,
//    R:  Try,
//    R:  for<T> FromResidual<T>,
    R:  Try<Output = WD> + FromResidual<Result<Infallible, WalkError>>,
{
    fn xml_document(&self) -> &XmlDocument;
    
    // Start the walk at the root of the document
    fn walk(&self, d: ED) -> R
    where
        Self: Sized,
    {
        let xml_doc = self.xml_document();
        let root = &xml_doc.root;
        self.walk_i(root, &d)
    }
    
    fn walk_i<'e>(&self, element: &'e Element, ed: &ED) -> R
    where
        Self:   Sized,
//        R:      Try,
    {
        let next_ed = match ed.next_level(element) {
            Err(e) => return FromResidual::from_residual(Err(e)),
            Ok(next_ed) => next_ed,
        };

        let mut acc = AC::new(element, ed);
        let mut wd_vec = Vec::<WD>::new();
        
        for elem in &element.subelements {
            let wd = match Try::branch(self.walk_i(elem, &next_ed)) {
                ControlFlow::Continue(wd) => wd,
                ControlFlow::Break(e) => return FromResidual::from_residual(e),
            };

            wd_vec.push(wd);
        }
        
        for wd in &wd_vec {
            if let Err(e) = acc.add(&wd) {
                return FromResidual::from_residual(Err(e));
            }
        }
        
        acc.summary()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::convert::Infallible;
    use std::error::Error;
//    use std::fmt;
    use std::ops::{FromResidual, Try};
    use xml::attribute::OwnedAttribute;
    use xml::common::XmlVersion;
    use xml::name::OwnedName;
    use xml::namespace::Namespace;
    use xml::reader::XmlEvent;

    use crate::xml_document::{Element, ElementInfo, XmlDocument};
    use crate::xml_document_factory::DocumentInfo;

    use super::{Accumulator, ElemData, WalkData, WalkError, Walkable};
//    use super::WalkableResult;

    const INDENT: &str = "    ";

    #[test]
    fn test_walk_tree_names()
    {
        // Call a non-generic test function directly
        test_walk_tree_names_concrete();
/*
        test_walk_tree_names1::<TestAccumulator, TestElemData, TestWalkData,
            Result<TestWalkData, WalkError>>();
*/
    }

    fn test_walk_tree_names_concrete() {
        // Use concrete types directly without generics
        let doc = create_test_doc();
        let walker = TestWalkable { xml_document: &doc };
        println!("Processing:");
        let ed = TestElemData::new(0);
        
//        let result: Result<TestWalkData, WalkError> = walker.walk(ed);
         let result: Result<TestWalkData, WalkError> = 
            <TestWalkable<'_> as Walkable<'_, TestAccumulator, TestElemData, TestWalkData, Result<TestWalkData, WalkError>>>::walk(&walker, ed);

        let res = match result {
            Result::Ok(data) => {
                let res = format!("{}", data.data);
                println!("Output:\n{}", res);
                res
            }
            Result::Err(e) => {
                let res = format!("{}", e);
                eprintln!("Error: {}", res);
                res
            }
        };
        assert_eq!(res, concat!("n1\n", "    n2\n", "    n3\n", "        n4"));
    }

/*
    fn test_walk_tree_names1<'a, AC, ED, WD, R>()
    where
        AC: Accumulator<'a, ED, WD, R>,
        ED: ElemData<Output = ED>,
        WD: WalkData,
        R:  Try<Output = WD> + FromResidual<Result<Infallible, WalkError>>,
    {
        let doc = create_test_doc(); // build a sample XmlDocument
        let walker = TestWalkable { xml_document: &doc };
        println!("Processing:");
        let ed = TestElemData::new(0);
        let result = <TestWalkable<'_> as Walkable<'_, AC, ED, WD, Result<WD, WalkError>>>::walk(&walker, ed);

        let res = match result {
            Result::Ok(data) => {
                let res = format!("{:?}", data.data);
                println!("Output:\n{}", res);
                res
            }
            Result::Err(e) => {
                let res = format!("{}", e);
                eprintln!("Error: {}", res);
                res
            }
        };
        assert_eq!(res, concat!("n1\n", "    n2\n", "    n3\n", "        n4"));
    }
*/

    struct TestWalkable<'a> {
        xml_document: &'a XmlDocument,
    }

//    impl<R> Walkable<'_, TestAccumulator, TestElemData, TestWalkData, Result<TestWalkData, WalkError>>
    impl<'a, AC, ED, WD, R> Walkable<'a, AC, ED, WD, R>
    for TestWalkable<'a>
    where
        AC: Accumulator<'a, ED, WD, R>,
        ED: ElemData<Output = ED>,
        WD: WalkData,
        R:  Try<Output = WD> + FromResidual<Result<Infallible, WalkError>>,
    {
        fn xml_document(&self) -> &XmlDocument {
            self.xml_document
        }
    }

    // ----------------- Data Types ----------------
//    R:  FromResidual<Result<Infallible, Box<dyn Error + Send + Sync>>>,
    struct TestWalkableResult {
    }
    impl FromResidual<Result<Infallible, Box<dyn Error + Send + Sync>>> for
        TestWalkableResult {
        fn from_residual(_: Result<Infallible, Box<(dyn Error + Send + Sync + 'static)>>) ->
            Self
        {
            todo!()
        }

    }

    #[derive(Debug)]
    pub struct TestAccumulator {
        result: String,
    }

//    impl<'a, ED, WD, R> Accumulator<'a, ED, WD, R>
    impl Accumulator<'_, TestElemData, TestWalkData, Result<TestWalkData, WalkError>>
    for TestAccumulator
//    where
//        ED: ElemData,
//        WD: WalkData,
//        R:  Try,
    {
        fn new(e: &Element, ed: &TestElemData) -> Self {
            let result = format!("{}{}", INDENT.repeat(ed.depth), e.name.local_name);
            TestAccumulator { result }
        }

        fn add(&mut self, wd: &TestWalkData) -> 
            Result<(), WalkError> {
            self.result += &format!("\n{}", wd.data);
            Result::Ok(())
        }

        fn summary(&self) -> Result<TestWalkData, WalkError> {
            Result::Ok(TestWalkData {
                data: self.result.clone(),
            })
        }
    }

//    struct TestWalkableResult {
//    }

//    impl WalkableResult for TestWalkableResult {
//    }

//    impl WalkableResult for TestWalkableResult {
//    }
//    type TestWalkableResult = Result<TestWalkData, Box<dyn Error>>;

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

    impl ElemData for TestElemData {
        type Output = TestElemData;

        fn next_level<'a>(&'a self, element: &Element) ->
            Result<Self::Output, WalkError> {
            println!("{}{}", INDENT.repeat(self.depth), element.name.local_name);
            let ed = TestElemData {
                depth: self.depth + 1,
            };

            Result::Ok(ed)
        }
    }

    // ----------------- Data Types ----------------

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

/*
//------------------------------------------------------------------------
// FIXME: move to walk_and_print.rs
//------------------------------------------------------------------------
/*
 * Recursive print
 */

use std::fmt;
/*

use crate::xml_document::{Element, XmlDocument};

use super::{Accumulator, ElemData, WalkData, WalkError, Walkable};
//use super::WalkableResult;
*/

const INDENT: &str = "    ";

struct WalkAndPrint<'a> {
    xml_document: &'a XmlDocument,
    f: &'a mut fmt::Formatter<'a>,
}

impl<'a> WalkAndPrint<'a> {
    pub fn new(xml_document: &'a XmlDocument, f: &'a mut fmt::Formatter<'a>) -> WalkAndPrint<'a> {
        WalkAndPrint {
            xml_document:   xml_document,
            f:              f,
        }
    }
}

impl Walkable<'_, PrintAccumulator, PrintElemData, PrintWalkData, fmt::Result>
for WalkAndPrint<'_> {
    fn xml_document(&self) -> &XmlDocument {
        self.xml_document
    }
}

#[derive(Debug)]
pub struct PrintAccumulator {
    result: String,
}

impl Accumulator<'_, PrintElemData, PrintWalkData, fmt::Result> for PrintAccumulator {
    fn new(e: &Element, ed: &PrintElemData) -> Self {
        let result = format!("{}{}", INDENT.repeat(ed.depth), e.name.local_name);
        PrintAccumulator { result }
    }

    fn add(&mut self, wd: &PrintWalkData) -> 
        Result<(), WalkError> {
        self.result += &format!("\n{}", wd.data);
        Result::Ok(())
    }

    fn summary(&self) -> fmt::Result {
        Result::Ok(())
    }
}

// ----------------- Data Types ----------------

//type PrintWalkableResult = Result<PrintWalkData, WalkError>;

//impl WalkableResult for PrintWalkableResult {
//}

#[derive(Debug)]
pub struct PrintWalkData {
    pub data: String,
}

impl WalkData for PrintWalkData {}

#[derive(Debug)]
pub struct PrintElemData {
    pub depth: usize,
}

impl PrintElemData {
    fn new(depth: usize) -> PrintElemData {
        PrintElemData {
            depth:  depth,
        }
    }
}

impl ElemData for PrintElemData {
    type Output = PrintElemData;

    fn next_level<'a>(&'a self, element: &Element) ->
        Result<Self::Output, WalkError> {
        println!("{}{}", INDENT.repeat(self.depth), element.name.local_name);
        let ed = PrintElemData {
            depth: self.depth + 1,
        };

        Result::Ok(ed)
    }
}

#[cfg(test)]
// FIXME: restore name to tests
mod tests2 {
    use std::collections::BTreeMap;
    use xml::attribute::OwnedAttribute;
    use xml::common::XmlVersion;
    use std::fmt;
    use xml::name::OwnedName;
    use xml::namespace::Namespace;
    use xml::reader::XmlEvent;

    use crate::Walkable;
    use crate::xml_document::{Element, XmlDocument};
    use crate::xml_document_factory::{DocumentInfo, ElementInfo};
    use super::{PrintElemData, WalkAndPrint};


    struct TestWalk<'a> {
        xml_doc:    &'a XmlDocument,
    }

    impl<'a> TestWalk<'a> {
        fn new(doc: &'_ XmlDocument) -> TestWalk {
            TestWalk {
                xml_doc:    doc,
            }
        }
    }

    impl<'a> fmt::Display for TestWalk<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let walker = WalkAndPrint::new(&self.xml_doc, f);
            let ped = PrintElemData {
                depth:  0,
            };
            walker.walk(ped)
        }
    }

    #[test]
    fn test_walk_tree_print() {
        println!("\nStart test_walk_tree_print");
        let test_doc = create_test_doc(); // build a sample XmlDocument
        println!("doc: {}", test_doc);

/*
        let res = match result {
            fmt::Result::Ok(data) => {
                let res = format!("{}", data.data);
                println!("Output:\n{}", res);
                res
            }
            io::Result::Err(e) => {
                let res = format!("{}", e);
                eprintln!("Error: {}", res);
                res
            }
        };
        assert_eq!(res, concat!("<n1>\n",
            "    <n2>\n",
            "    <n3>\n",
            "        <n4>"));
*/
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
*/
