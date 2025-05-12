use std::boxed::Box;
use std::convert::Infallible;
use std::error::Error;
use std::ops::{FromResidual, Try};
use std::marker::{Send, Sync};
use std::result::Result;

use crate::xml_document::{Element, XmlDocument};

// From Claude Code
pub type WalkError = Box<dyn Error + Send + Sync + 'static>;

// ----------------- Traits ----------------
// Information that supplements the Element to produce a piece of the overall
// result.
pub trait ElemData<ER>
where
    ER: Try,
{
    fn next_level(&mut self, element: &Element) -> ER;
}

pub trait WalkData {}

// FIXME: get ride of numeric lifetimes, like 'e
//pub trait Accumulator<'e, 'a, ED, ER, WD, WR> 
pub trait Accumulator<'e, 'a, ED, ER, WD, WR> 
where
    ED: ElemData<ER>,
    ER: Try,
    WD: WalkData,
    WR: Try,
{
    fn new<'b>(e: &'e Element, ed: &'b mut ED) -> Self
    where
        Self: Sized;
    fn add(&mut self, wd: &WD) -> Result<(), WalkError>;
    fn summary(&self) -> WR;
}

pub trait Walkable<'e, 'a, AC, ED, ER, WD, WR>
where
    AC: Accumulator<'e, 'a, ED, ER, WD, WR>,
    ED: ElemData<ER>,
    ER: Try<Output = ED>,
    WD: WalkData,
    WR: Try<Output = WD>,
    WR: FromResidual<<ER as Try>::Residual>,
    WR: FromResidual<Result<Infallible, WalkError>>,
{
    fn xml_document(&self) -> &XmlDocument;
    
    // Start the walk at the root of the document
    fn walk<'g: 'e>(&'g self, d: &mut ED) -> WR
    where
        Self: Sized,
    {
        let xml_doc = self.xml_document();
        let root = &xml_doc.root;
        self.walk_i(root, d)
    }
    
    fn walk_i<'f: 'e>(&self, element: &'f Element, ed: &mut ED) -> WR
    where
        Self: Sized,
    {
        let mut next_ed: ED = match Try::branch(ed.next_level(element)) {
            std::ops::ControlFlow::Continue(val) => val,
            std::ops::ControlFlow::Break(residual) => return FromResidual::from_residual(residual),
        };

        let mut acc = AC::new(element, ed);
        let mut wd_vec = Vec::<WD>::new();
        
        for elem in &element.subelements {
            let wd: WD = match Try::branch(self.walk_i(elem, &mut next_ed)) {
                std::ops::ControlFlow::Continue(val) => val,
                std::ops::ControlFlow::Break(residual) => return FromResidual::from_residual(residual),
            };

            wd_vec.push(wd);
        }
        
        for wd in &wd_vec {
            if let Err(e) = acc.add(&wd) {
                // Create a properly typed error result using the standard error conversion method
                let result: Result<Infallible, WalkError> = Err(e);
                return FromResidual::from_residual(result);
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
    use std::ops::{FromResidual, Try};
    use xml::attribute::OwnedAttribute;
    use xml::common::XmlVersion;
    use xml::name::OwnedName;
    use xml::namespace::Namespace;
    use xml::reader::XmlEvent;

    use crate::xml_document::{Element, ElementInfo, XmlDocument};
    use crate::xml_document_factory::DocumentInfo;

    use super::{Accumulator, ElemData, WalkData, WalkError, Walkable};

    const INDENT: &str = "    ";

    #[test]
    fn test_walk_tree_names()
    {
        // Call a non-generic test function directly
        test_walk_tree_names_concrete();
    }

    fn test_walk_tree_names_concrete<'e>() {
        // Use concrete types directly without generics
        let doc = create_test_doc();

        // Create a closure that has access to doc
        let test_fn = || {
            let walker = TestWalkable { xml_document: &doc };
            println!("Processing:");
            let mut ed = TestElemData::new(0);
            
            // Use the walker
/*
            let result: Result<TestWalkData, WalkError> = walker.walk(&mut ed);
*/
            let result: Result<TestWalkData, WalkError> = <TestWalkable<'_> as Walkable<'_, '_, TestAccumulator, TestElemData, Result<TestElemData, Box<(dyn std::error::Error + Send + Sync + 'static)>>, TestWalkData, Result<TestWalkData, Box<dyn std::error::Error + Send + Sync>>>>::walk::<'_>(&walker, &mut ed);
            // Process result...
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

            res
        };
/*
        let walker = TestWalkable { xml_document: &doc };
        println!("Processing:");
        let mut ed = TestElemData::new(0);
        
        let result: Result<TestWalkData, WalkError> = 
            <TestWalkable<'_> as Walkable<'e, '_, TestAccumulator, TestElemData, Result<TestElemData, WalkError>, TestWalkData, Result<TestWalkData, WalkError>>>::walk(&walker, &mut ed);
drop(&walker);

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
*/

        let res = test_fn();

        assert_eq!(res, concat!("n1\n", "    n2\n", "    n3\n", "        n4"));
    }

    struct TestWalkable<'a> {
        xml_document: &'a XmlDocument,
    }

    impl<'e, 'a, AC, ED, ER, WD, WR> Walkable<'e, 'a, AC, ED, ER, WD, WR>
    for TestWalkable<'a>
    where
        AC: Accumulator<'e, 'a, ED, ER, WD, WR>,
        ED: ElemData<ER>,
        ER: Try<Output = ED>,
        WD: WalkData,
        WR: Try<Output = WD>,
        WR: FromResidual<<ER as Try>::Residual>,
        WR: FromResidual<Result<Infallible, WalkError>>,
    {
        fn xml_document(&self) -> &XmlDocument {
            self.xml_document
        }
    }

    // ----------------- Data Types ----------------
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

    impl<'e> Accumulator<'e, '_, TestElemData, Result<TestElemData, WalkError>, TestWalkData, Result<TestWalkData, WalkError>>
    for TestAccumulator
    {
        fn new(e: &Element, ed: &mut TestElemData) -> Self {
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
        fn next_level(&mut self, element: &Element) -> Result<TestElemData, WalkError> {
            println!("{}{}", INDENT.repeat(self.depth), element.name.local_name);
            let ed = TestElemData::new(self.depth + 1);
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
use std::convert::From;

const INDENT: &str = "    ";

// Custom adapter type for use with fmt::Display
#[derive(Debug)]
pub struct CustomWalkResult<T>(Result<T, WalkError>);

impl<T> Try for CustomWalkResult<T> {
    type Output = T;
    type Residual = Result<Infallible, WalkError>;

    fn from_output(output: T) -> Self {
        CustomWalkResult(Ok(output))
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self.0 {
            Ok(v) => std::ops::ControlFlow::Continue(v),
            Err(e) => std::ops::ControlFlow::Break(Err(e)),
        }
    }
}

impl<T> FromResidual<Result<Infallible, WalkError>> for CustomWalkResult<T> {
    fn from_residual(residual: Result<Infallible, WalkError>) -> Self {
        match residual {
            Ok(_) => unreachable!(),
            Err(e) => CustomWalkResult(Err(e)),
        }
    }
}

// Now implement From for fmt::Result to handle the ? in fmt::Display
impl From<CustomWalkResult<PrintWalkData>> for fmt::Result {
    fn from(res: CustomWalkResult<PrintWalkData>) -> Self {
        match res.0 {
            Ok(_) => Ok(()),
            Err(_) => Err(fmt::Error),
        }
    }
}

struct WalkAndPrint<'a> {
    xml_document: &'a XmlDocument,
    f: &'a mut fmt::Formatter<'a>,
}

impl<'a> WalkAndPrint<'a> {
    pub fn new(xml_document: &'a XmlDocument, f: &'a mut fmt::Formatter<'a>) -> WalkAndPrint<'a> {
        WalkAndPrint {
            xml_document: xml_document,
            f: f,
        }
    }
}

impl<'e, 'a, AC, ED, ER, WD, WR> Walkable<'e, 'a, AC, ED, ER, WD, WR>
for WalkAndPrint<'a>
where
    AC: Accumulator<'e, 'a, ED, ER, WD, WR>,
    ED: ElemData<ER>,
    ER: Try<Output = ED>,
    WD: WalkData,
    WR: Try<Output = WD>,
    WR: FromResidual<<ER as Try>::Residual>,
    WR: FromResidual<Result<Infallible, WalkError>>,
{
    fn xml_document(&self) -> &XmlDocument {
        self.xml_document
    }
}

pub struct PrintAccumulator<'a> {
    f: &'a mut fmt::Formatter<'a>,
    depth: usize,
}

impl<'e, 'a, 'g: 'a, 'f: 'a> Accumulator<'e, 'a, PrintElemData<'a>, Result<PrintElemData<'a>, WalkError>, PrintWalkData, CustomWalkResult<PrintWalkData>>
for PrintAccumulator<'g>
{
    fn new<'b>(e: &'e Element, ed: &'f mut PrintElemData<'g>) -> PrintAccumulator<'a> {
        // Write the element name at the correct indentation level
        let _ = write!(ed.f, "{}{}\n", INDENT.repeat(ed.depth), e.name.local_name);
        
        PrintAccumulator { 
            f: ed.f,
            depth: ed.depth,
        }
    }

    fn add(&mut self, _wd: &PrintWalkData) -> Result<(), WalkError> {
        Ok(())
    }

    fn summary(&self) -> CustomWalkResult<PrintWalkData> {
        CustomWalkResult(Ok(PrintWalkData {}))
    }
}

// ----------------- Data Types ----------------

#[derive(Debug)]
pub struct PrintWalkData {
}

impl WalkData for PrintWalkData {}

pub struct PrintElemData<'a> {
    pub depth: usize,
    pub f: &'a mut fmt::Formatter<'a>,
}

impl<'a> PrintElemData<'a> {
    fn new(depth: usize, f: &'a mut fmt::Formatter<'a>) -> PrintElemData<'a> {
        PrintElemData {
            depth,
            f,
        }
    }
}

impl<'a, 'e: 'a, 'f> ElemData<Result<PrintElemData<'a>, WalkError>> for PrintElemData<'f> {
    fn next_level(&'f mut self, _element: &Element) -> Result<PrintElemData<'a>, WalkError> {
        let next_ed = PrintElemData::new(self.depth + 1, self.f);
        Result::Ok(next_ed)
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
//    use std::convert::Infallible;

    use crate::xml_document::{Element, XmlDocument};
    use crate::xml_document_factory::{DocumentInfo, ElementInfo};
    use super::{PrintElemData, WalkAndPrint, Walkable, PrintAccumulator, PrintWalkData/*, PrintResult*/, WalkError};
    use super::CustomWalkResult;

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

    impl<'e, 'a: 'e, 'b: 'e, 'c: 'b> fmt::Display for TestWalk<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let walker = WalkAndPrint::new(&self.xml_doc, f);
            let mut ped = PrintElemData::new(0, f);
            
            // Use the CustomWalkResult directly
            let result: CustomWalkResult<PrintWalkData> = <WalkAndPrint<'_> as Walkable<'e, '_, 
                PrintAccumulator<'_>, 
                PrintElemData<'_>, 
                Result<PrintElemData<'_>, WalkError>, 
                PrintWalkData, 
                CustomWalkResult<PrintWalkData>>>::walk(&walker, &mut ped);
                
            // Convert CustomWalkResult to fmt::Result
            result.into()
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
*/
