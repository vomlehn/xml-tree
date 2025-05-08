use std::boxed::Box;
use std::convert::Infallible;
use std::error::Error;
use std::ops::{FromResidual, Try};
use std::marker::{Send, Sync};
use std::result::Result;

use crate::xml_document::{Element, XmlDocument};

pub type WalkError = Box<dyn Error + Send + Sync + 'static>;
//pub type WalkableResult<E, WD> = Result<WD, E>;

pub trait WalkableResult: Try + FromResidual<Result<Infallible, Box<dyn Error + Send + Sync>>> {}

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
    R:  WalkableResult<Output = WD>,
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
    R:  WalkableResult<Output = WD>,
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
        Self: Sized,
    {
        let next_ed = ed.next_level(element)?;
        let mut acc = AC::new(element, ed);
        let mut wd_vec = Vec::<WD>::new();
        
        for elem in &element.subelements {
            let wd = self.walk_i(elem, &next_ed)?;
            wd_vec.push(wd);
        }
        
        for wd in &wd_vec {
            acc.add(&wd)?;
        }
        
        acc.summary()
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

    use crate::xml_document::{Element, ElementInfo, XmlDocument};
    use crate::xml_document_factory::DocumentInfo;

    use super::{Accumulator, ElemData, WalkData, WalkError, Walkable};
    use super::WalkableResult;

    const INDENT: &str = "    ";

    #[test]
    fn test_walk_tree_names() {
        let doc = create_test_doc(); // build a sample XmlDocument
        let walker = TestWalkable { xml_document: &doc };
        println!("Processing:");
        let ed = TestElemData::new(0);
        let result = walker.walk(ed);

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

    struct TestWalkable<'a> {
        xml_document: &'a XmlDocument,
    }

    impl Walkable<'_, TestAccumulator, TestElemData, TestWalkData, TestWalkableResult> for TestWalkable<'_> {
        fn xml_document(&self) -> &XmlDocument {
            self.xml_document
        }
    }

    // ----------------- Data Types ----------------

    type TestWalkableResult = Result<TestWalkData, WalkError>;

    impl WalkableResult for TestWalkableResult {
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

    impl ElemData for TestElemData {
        type Output = TestElemData;

        fn next_level<'a>(&'a self, element: &Element) ->
//            TestWalkableResult {
            Result<Self::Output, WalkError> {
//            Result<TestElemData, WalkError> {
            println!("{}{}", INDENT.repeat(self.depth), element.name.local_name);
            let ed = TestElemData {
                depth: self.depth + 1,
            };

            Result::Ok(ed)
        }
    }

    #[derive(Debug)]
    pub struct TestAccumulator {
        result: String,
    }

    impl Accumulator<'_, TestElemData, TestWalkData, TestWalkableResult> for TestAccumulator {
        fn new(e: &Element, ed: &TestElemData) -> Self {
            let result = format!("{}{}", INDENT.repeat(ed.depth), e.name.local_name);
            TestAccumulator { result }
        }

        fn add(&mut self, wd: &TestWalkData) -> 
            Result<(), WalkError> {
            self.result += &format!("\n{}", wd.data);
            Result::Ok(())
        }

        fn summary(&self) -> TestWalkableResult {
            Result::Ok(TestWalkData {
                data: self.result.clone(),
            })
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
