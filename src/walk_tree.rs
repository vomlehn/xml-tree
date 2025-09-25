/* 
 * FIXME: This should probably go away
 */

/*
/*
 * Trait for building recursive walk types
 */

//use std::borrow::Borrow;
//use std::cell::RefCell;
use std::ops::{FromResidual, Try};

// FIXME: delete xml_tree::Element
use crate::xml_tree::XmlTree;
use crate::parse_tree::Element;

/**
 * Data for the Element being worked on by walk_down().
 */
pub trait ElemData<AC, ED>
{
    fn next_level(&self, acc: &AC, element: &dyn Element) -> ED;
}

/**
 * Data returned by Accumulator functions.
 */
pub trait WalkData {}

/*
/**
 * Data stored at the root level of the Walkable and a reference to which is
 * returned by the Walkable base_level_cell() function.
 */
pub trait BaseLevel {}
*/

/**
 * Data stored for the peers of the Element a given invocation of walk_down()
 */
pub trait Accumulator<'a, BL, ED, WD, WR> {
    fn new(bl: &mut BL, e: &dyn Element, ed: &ED) -> Self
    where
        Self: Sized;
    fn add(&mut self, wd: &WD, ed: &ED) -> WR;
    fn summary(&self, bl: &mut BL) -> WR;
}

/**
 * This is the core code for walking an XmlTree. Use it to implement
 * a struct. The structs that need to be implemented are:
 * Accumulator
 * BaseLevel
 * ElemData
 * WalkData
 * WalkResult
 */
pub fn walk<'a, AC, BL, ED, WD, WR>(bl: &mut BL, xml_doc: &XmlTree, ed: &ED) -> WR
where
    AC: Accumulator<'a, BL, ED, WD, WR>,
    ED: ElemData<AC, ED>,
    WR: Try<Output = WD>,
    WR: FromResidual,
{
    // Seems inelegant
    walk_down::<AC, BL, ED, WD, WR>(bl, &vec!(xml_doc.root.clone()), ed)
}

// FIXME: this assumes the root has only one element
fn walk_down<'a, AC, BL, ED, WD, WR>(bl: &mut BL, elements: &Vec<Box<dyn Element>>, ed: &ED) -> WR
where
    AC: Accumulator<'a, BL, ED, WD, WR>,
    ED: ElemData<AC, ED>,
    WR: Try<Output = WD>,
    WR: FromResidual,
{
    let mut acc = AC::new(bl, &*(elements[0]), ed);

    // Process subelements and collect WalkData results in a vector
//    let next_ed = ed.next_level(&acc, &(*elements)[0]);
    let next_ed = ed.next_level(&acc, &*(elements)[0]);

    let mut wd_vec = Vec::<WD>::new();

    for elem in (*elements)[0].subelements() {
        let wd = walk_down::<AC, BL, ED, WD, WR>(bl, elem.subelements(), &next_ed)?;
        wd_vec.push(wd);
    }

    // Accumulate results
    for wd in &wd_vec {
        acc.add(wd, &next_ed)?;
    }
    acc.summary(bl)
}
/*
}
*/

#[cfg(test)]
mod test_tests {
    use std::cell::RefCell;
    use std::fmt;

    use crate::xml_document::{create_test_doc, ParseElement, Element, XmlTree};
    use crate::parse_tree::{DocumentInfo};
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

    impl ElemData<TestElemData> for TestElemData {
        fn next_level(&self, &acc: Accumulator, _element: &Box<dyn Element>) -> TestElemData {
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
        fn new(bl: &mut TestBaseLevel, e: &'a Box<dyn Element>, ed: &TestElemData) -> Self {
            TestAccumulator {
                result: nl_indent(ed.depth) +  e.name.local_name.as_str() + "\n",
            }
        }

        fn add(&mut self, wd: &TestWalkData) -> TestWalkResult {
            self.result += &wd.data;
            Ok(TestWalkData::new(self.result.clone()))
        }

        fn summary(&self, bl: &mut TestBaseLevel) -> TestWalkResult {
            Ok(TestWalkData::new(self.result.clone()))
        }
    }

    pub struct TestWalkable<'a> {
        xml_doc:    &'a XmlTree,
        base:       RefCell<TestBaseLevel>,
    }

    impl<'a> TestWalkable<'a> {
        pub fn new(xml_doc: &'a XmlTree, base: TestBaseLevel) -> TestWalkable<'a> {
            TestWalkable{
                xml_doc:    xml_doc,
                base:       RefCell::new(base),
            }
        }
    }

    impl<'a> Walkable<'_, TestAccumulator, TestBaseLevel, TestElemData, TestWalkData, TestWalkResult> for TestWalkable<'_> {
        fn xml_document(&self) -> &XmlTree {
            self.xml_doc
        }
        /*
        fn base_level_cell(&self) -> &RefCell<TestBaseLevel> {
            &self.base
        }
        */
    }

/*
    pub fn nl_indent(n: usize) -> String {
        INDENT.repeat(n)
    }
*/

    #[test]
    fn test_result_return() {
        let xml_document = create_test_doc();

        println!("Try with a TestWalkResult");
        println!("-------------------------");
        let bl1 = TestBaseLevel::new();
        let ed1 = TestElemData::new(0);
        let w1 = TestWalkable::new(&xml_document);
        println!("Display w1:");
        let res = w1.walk(&bl1, &xml_document, &ed1);
        match res {
            Err(e) => println!("Error: {:?}", e),
            Ok(twd) => {
                println!("Actual: {:?}", twd.data);
                let expected = "n1\n".to_owned() +
                    &nl_indent(1) + "n2\n" +
                    &nl_indent(1) + "n3\n" +
                    &nl_indent(2) + "n4\n";
                println!("Expected: {:?}", expected);

                assert_eq!(twd.data, expected);
            },
        };
    }

}
*/
