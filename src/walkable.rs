/*
 * Trait for building recursive walk types
 */

//use std::borrow::Borrow;
//use std::cell::RefCell;
use std::ops::{FromResidual, Try};

use crate::xml_document::{Element, XmlDocument};

/**
 * Data for the Element being worked on by walk_down().
 */
pub trait ElemData<BL, ED>
{
    fn next_level(&self, element: &Box<dyn Element>) -> ED;
}

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
    fn new(bl: &mut BL, e: &Box<dyn Element>, ed: &ED) -> Self
    where
        Self: Sized;
    fn add(&mut self, wd: &WD) -> WR;
    fn summary(&self) -> WR;
}

/**
 * This is the core code for walking an XmlDocument. Use it to implement
 * a struct. The structs that need to be implemented are:
 * Accumulator
 * BaseLevel
 * ElemData
 * WalkData
 * WalkResult
 */
pub trait Walkable<'a, AC, BL, ED, WD, WR>
where
    AC: Accumulator<'a, BL, ED, WD, WR>,
    BL: 'a + BaseLevel,
    ED: ElemData<BL, ED>,
    WR: Try<Output = WD>,
    WR: FromResidual,
{
    fn xml_document(&self) -> &XmlDocument;

    fn walk<'b>(&'b self, bl: &mut BL, ed: &ED) -> WR
    where
        'b: 'a,
        Self: Sized,
    {
        let xml_doc = self.xml_document();
        self.walk_down(bl, &xml_doc.root, ed)
    }

    fn walk_down<'b>(&'b self, bl: &mut BL, element: &Box<dyn Element>, ed: &ED) -> WR
    where
        'b: 'a,
    {
        let mut acc = AC::new(bl, element, ed);

        // Process subelements and collect WalkData results in a vector
        let next_ed = ed.next_level(element);
        let mut wd_vec = Vec::<WD>::new();

        for elem in element.subelements() {
            let wd = self.walk_down(bl, elem, &next_ed)?;
            wd_vec.push(wd);
        }

        // Accumulate results
        for wd in &wd_vec {
            acc.add(wd)?;
        }
        acc.summary()
    }
}

#[cfg(test)]
mod test_tests {
    use std::cell::RefCell;
    use std::collections::BTreeMap;
    use std::fmt;

    use xml::attribute::OwnedAttribute;
    use xml::common::XmlVersion;
    use xml::name::OwnedName;
    use xml::namespace::Namespace;
    use xml::reader::XmlEvent;

    use crate::xml_document::{Element, XmlDocument};
    use crate::xml_document_factory::{DocumentInfo, ElementInfo};
    use crate::walkable::{Accumulator, BaseLevel, ElemData, WalkData, Walkable};

    const INDENT: &str = "    ";

    #[derive(Debug)]
    pub enum TestWalkError {
    }

    impl std::error::Error for TestWalkError {}

    impl fmt::Display for TestWalkError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    type TestWalkResult = Result<TestWalkData, TestWalkError>;

    /**
     * We don't have any data at the base level as it's all returned
     * directly
     */
    pub struct TestBaseLevel {}

    impl TestBaseLevel {
        pub fn new() -> TestBaseLevel {
            TestBaseLevel {}
        }
    }

    impl BaseLevel for TestBaseLevel {}

    /**
     * Keep track of the depth of nexting
     */
    pub struct TestElemData {
        depth:  usize,
    }

    impl TestElemData {
        pub fn new(depth: usize) -> TestElemData {
            TestElemData {
                depth:  depth,
            }
        }
    }

    impl ElemData<TestBaseLevel, TestElemData> for TestElemData {
        fn next_level(&self, _element: &Box<dyn Element>) -> TestElemData {
            TestElemData::new(self.depth + 1)
        }
    }

    #[derive(Debug)]
    pub struct TestWalkData {
        pub data: String,
    }
    impl TestWalkData {
        fn new(data: String) -> TestWalkData {
            TestWalkData {
                data:   data,
            }
        }
    }
    impl WalkData for TestWalkData {}

    pub struct TestAccumulator {
        result: String,
    }

    impl<'a> Accumulator<'a, TestBaseLevel, TestElemData, TestWalkData, TestWalkResult>
    for TestAccumulator
    {
        fn new(_bl: &RefCell<TestBaseLevel>, e: &'a Box<dyn Element>, ed: &TestElemData) -> Self {
            TestAccumulator {
                result: indent(ed.depth) +  e.name.local_name.as_str() + "\n",
            }
        }

        fn add(&mut self, wd: &TestWalkData) -> TestWalkResult {
            self.result += &wd.data;
            Ok(TestWalkData::new(self.result.clone()))
        }

        fn summary(&self) -> TestWalkResult {
            Ok(TestWalkData::new(self.result.clone()))
        }
    }

    pub struct TestWalkable<'a> {
        xml_doc:    &'a XmlDocument,
        base:       RefCell<TestBaseLevel>,
    }

    impl<'a> TestWalkable<'a> {
        pub fn new(xml_doc: &'a XmlDocument, base: TestBaseLevel) -> TestWalkable<'a> {
            TestWalkable{
                xml_doc:    xml_doc,
                base:       RefCell::new(base),
            }
        }
    }

    impl<'a> Walkable<'_, TestAccumulator, TestBaseLevel, TestElemData, TestWalkData, TestWalkResult> for TestWalkable<'_> {
        fn xml_document(&self) -> &XmlDocument {
            self.xml_doc
        }
        fn base_level_cell(&self) -> &RefCell<TestBaseLevel> {
            &self.base
        }
    }

    pub fn indent(n: usize) -> String {
        INDENT.repeat(n)
    }

    #[test]
    fn test_result_return() {
        let xml_document = create_test_doc();

        println!("Try with a TestWalkResult");
        println!("-------------------------");
        let bl1 = TestBaseLevel::new();
        let ed1 = TestElemData::new(0);
        let w1 = TestWalkable::new(&xml_document);
        println!("Display w1:");
        let res = w1.walk(&bl1, &ed1);
        match res {
            Err(e) => println!("Error: {:?}", e),
            Ok(twd) => {
                println!("Actual: {:?}", twd.data);
                let expected = "n1\n".to_owned() +
                    &indent(1) + "n2\n" +
                    &indent(1) + "n3\n" +
                    &indent(2) + "n4\n";
                println!("Expected: {:?}", expected);

                assert_eq!(twd.data, expected);
            },
        };
    }

    /**
     * Manually create an XmlDocument.
     */
     // FIXME: This should be moved to a common area
    fn create_test_doc() -> XmlDocument {
        let ns: Namespace = Namespace(BTreeMap::<String, String>::new());

        let ei: ElementInfo = ElementInfo {
            lineno: 1,
            attributes: Vec::<OwnedAttribute>::new(),
            namespace: ns,
        };

        XmlDocument {
            root:           branch("n1", &ei, vec![
                                leaf("n2", &ei),
                                branch("n3", &ei, vec![
                                    leaf("n4", &ei)])
                            ]),
            document_info:  DocumentInfo {
                                version: XmlVersion::Version10,
                                encoding: "xxx".to_string(),
                                standalone: None,
                            },
        }
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
            element_info: (*ei).clone(),
            subelements,
            before_element: Vec::<XmlEvent>::new(),
            content: Vec::<XmlEvent>::new(),
            after_element: Vec::<XmlEvent>::new(),
        }
    }
}
