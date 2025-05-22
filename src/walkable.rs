//use std::collections::BTreeMap;

use crate::xml_document::{Element, XmlDocument};

use std::ops::{FromResidual, Try};

//use std::fmt::Display;

pub trait WalkResult: Try {}

/*
#[derive(Debug)]
enum WalkError {
    One,
}
impl std::error::Error for WalkError {}
impl fmt::Display for WalkError {
    fn fmt<'b>(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{:?}", self)
    }
}
*/

/*
pub struct Element {
    name:           OwnedName,
    _element_info:   ElementInfo,
    subelements:    Vec<Element>,
    _before_element: Vec::<XmlEvent>,
    _content:        Vec::<XmlEvent>,
    _after_element:  Vec::<XmlEvent>,
}

#[derive(Clone)]
pub struct ElementInfo {
    _lineno:     usize,
    _attributes: Vec::<OwnedAttribute>,
    _namespace:  Namespace,
//    namespace:  Namespace<BTreeMap<String, String>>,
}

pub struct XmlDocument {
    root:           Element,
    document_info:  DocumentInfo,
}
impl XmlDocument {
    pub fn new(element: Element, document_info: DocumentInfo) -> XmlDocument {
        XmlDocument {
            root:           element,
            document_info:  document_info,
        }
    }
}

pub struct DocumentInfo{
    _version:    XmlVersion,
    _encoding:   String,
    _standalone: Option<()>,
}
*/

//=========================================================

/**
 * Data for the Element being worked on by walk_down().
 */
pub trait ElemData<BL, ED>
{
    fn next_level(&self, element: &Element) -> ED;
}

/**
 * Data returned by Accumulator functions.
 */
pub trait WalkData {}

/**
 * Data stored at the root level of the Walkable and a reference to which is
 * returned by the Walkable base_level() function.
 */
pub trait BaseLevel {}

/**
 * Data stored for the peers of the Element a given invocation of walk_down()
 */
pub trait Accumulator<'a, BL, ED, WD, WR> {
    fn new(bl: &BL, e: &'a Element, ed: &ED) -> Self
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
    BL: 'a,
    ED: ElemData<BL, ED>,
    WR: Try<Output = WD>,
    WR: FromResidual,
{
    fn xml_document(&self) -> &XmlDocument;
    fn base_level(&'a self) -> &'a BL;
    
    // Start the walk at the root of the document
    fn walk<'b: 'a>(&'b self, ed: &ED) -> WR
    where   
        Self: Sized,
    {       
        let xml_doc = self.xml_document();
        let root = &xml_doc.root;
        self.walk_down(root, ed)
    }

    fn walk_down<'b>(&'b self, element: &'a Element, ed: &ED) -> WR
    where
        'b: 'a,
    {
        let bl = self.base_level();
        let mut acc = AC::new(&bl, element, ed);

        // Process subelements and collect WalkData results
        let mut wd_vec = Vec::<WD>::new();

        for elem in &element.subelements {
            let next_ed = ed.next_level(elem);
            let wd = self.walk_down(elem, &next_ed)?;
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
    use xml::attribute::OwnedAttribute;
    use xml::common::XmlVersion;
    use xml::name::OwnedName;
    use xml::namespace::Namespace;
    use xml::reader::XmlEvent;

    use std::collections::BTreeMap;
    use std::fmt;

    use crate::xml_document::{Element, XmlDocument};
    use crate::xml_document_factory::{DocumentInfo, ElementInfo};
    use crate::walkable::{Accumulator, BaseLevel, ElemData, WalkData, Walkable};

    const INDENT: &str = "    ";

    #[derive(Debug)]
    pub enum TestWalkError {
        FAILED,
    }
    impl std::error::Error for TestWalkError {}
    impl fmt::Display for TestWalkError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    type TestWalkResult = Result<TestWalkData, TestWalkError>;

    pub struct TestBaseLevel {}
    impl TestBaseLevel {
        pub fn new() -> TestBaseLevel {
            TestBaseLevel {}
        }
    }
    impl BaseLevel for TestBaseLevel {}

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
        fn next_level(&self, _element: &Element) -> TestElemData {
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
        fn new(_bl: &TestBaseLevel, e: &'a Element, ed: &TestElemData) -> Self {
            TestAccumulator {
                result: indent(ed.depth) +  e.name.local_name.as_str() + "\n",
            }
        }

        fn add(&mut self, wd: &TestWalkData) -> TestWalkResult {
            self.result += wd.data.as_str();
            Ok(TestWalkData::new(self.result.clone()))
        }

        fn summary(&self) -> TestWalkResult {
            Ok(TestWalkData::new(self.result.clone()))
        }
    }

    pub struct TestWalkable<'a> {
        xml_doc:    &'a XmlDocument,
        base:       TestBaseLevel,
    }

    impl<'a> TestWalkable<'a> {
        pub fn new(xml_doc: &'a XmlDocument, base: TestBaseLevel) -> TestWalkable<'a> {
            TestWalkable{
                xml_doc:    xml_doc,
                base:       base,
            }
        }
    }

    impl<'a> Walkable<'_, TestAccumulator, TestBaseLevel, TestElemData, TestWalkData, TestWalkResult> for TestWalkable<'_> {
        fn xml_document(&self) -> &XmlDocument {
            self.xml_doc
        }
        fn base_level(&self) -> &TestBaseLevel {
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
        let w1 = TestWalkable::new(&xml_document, bl1);
        println!("Display w1:");
        let res = w1.walk(&ed1);
        match res {
            Err(e) => println!("Error: {:?}", e),
            Ok(twd) => {
                println!("Ok: {:?}", twd.data);

                assert_eq!(twd.data, "n1\n".to_owned() +
                    &indent(1) + "n2\n" +
                    &indent(1) + "n3\n" +
                    &indent(2) + "n4\n");
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
