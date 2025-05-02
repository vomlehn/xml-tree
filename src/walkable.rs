use crate::xml_document::{Element, XmlDocument};

use std::convert::Infallible;
use std::ops::{ControlFlow, FromResidual, Try};

pub type WalkError = Box<dyn std::error::Error + Send + Sync + 'static>;

// ----------------- Traits ----------------
// Information that supplements the Element to produce a piece of the overall
// result.
pub trait ElemData<ED>
where
    ED: ElemData<ED>,
{
    fn start(&self, element: &Element) -> WalkResult<ED, WalkError>;
}

// It seems as though ED and WD should be traits
pub trait Walkable<ED, WD, AC>
where
    ED: ElemData<ED>,
    WD: WalkData,
    AC: Accumulator<ED, WD>,
{
    fn xml_document(&self) -> &XmlDocument;

    // Start the walk at the root of the document
    fn walk<'a>(&self, d: &ED) -> WalkResult<WD, WalkError> {
        let xml_doc = self.xml_document();
        let root = &xml_doc.root;
        self.walk_i(root, d)
    }

    fn walk_i<'a>(&self, element: &Element, ed: &ED) -> WalkResult<WD, WalkError> {
        let mut acc = AC::new(element, &ed);
        let next_ed = ed.start(element)?;

        for elem in &element.subelements {
            let wd = self.walk_i(&elem, &next_ed)?;
            acc.add(&wd);
        }

        let wr = acc.summary();
        wr
    }
}

pub trait WalkData {}

// ----------------- Result Enums ----------------

#[derive(Debug)]
pub enum WalkResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Try for WalkResult<T, E> {
    type Output = T;
    type Residual = Result<Infallible, E>;

    fn from_output(output: T) -> Self {
        WalkResult::Ok(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            WalkResult::Ok(v) => ControlFlow::Continue(v),
            WalkResult::Err(e) => ControlFlow::Break(Err(e)),
        }
    }
}

impl<T, E> FromResidual<Result<Infallible, E>> for WalkResult<T, E> {
    fn from_residual(residual: Result<Infallible, E>) -> Self {
        match residual {
            Err(e) => WalkResult::Err(e),
            Ok(_) => unreachable!(),
        }
    }
}

pub trait Accumulator<ED, WD>
where
    ED: ElemData<ED>,
    WD: WalkData,
{
    fn new(e: &Element, ed: &ED) -> Self;
    fn add(&mut self, wd: &WD) -> WalkResult<WD, WalkError>;
    fn summary(&self) -> WalkResult<WD, WalkError>;
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

    use super::{Accumulator, ElemData, WalkData, WalkError, WalkResult, Walkable};

    const INDENT: &str = "    ";

    #[test]
    fn test_walk_tree_names() {
        let doc = create_test_doc(); // build a sample XmlDocument
        let walker = WalkableTest { xml_document: &doc };
        println!("Processing:");
        let result = walker.walk(&ElemDataTest { depth: 0 });

        let res = match result {
            WalkResult::Ok(data) => {
                let res = format!("{}", data.data);
                println!("Output:\n{}", res);
                res
            }
            WalkResult::Err(e) => {
                let res = format!("{}", e);
                eprintln!("Error: {}", res);
                res
            }
        };
        assert_eq!(res, concat!("n1\n", "    n2\n", "    n3\n", "        n4"));
    }

    struct WalkableTest<'a> {
        xml_document: &'a XmlDocument,
    }

    impl Walkable<ElemDataTest, WalkDataTest, AccumulatorTest> for WalkableTest<'_> {
        fn xml_document(&self) -> &XmlDocument {
            self.xml_document
        }
    }

    // ----------------- Data Types ----------------

    #[derive(Debug)]
    pub struct WalkDataTest {
        pub data: String,
    }

    impl WalkData for WalkDataTest {}

    #[derive(Debug)]
    pub struct ElemDataTest {
        pub depth: usize,
    }

    impl ElemData<ElemDataTest> for ElemDataTest {
        fn start(&self, element: &Element) -> WalkResult<ElemDataTest, WalkError> {
            println!("{}{}", INDENT.repeat(self.depth), element.name.local_name);
            let ed = ElemDataTest {
                depth: self.depth + 1,
            };
            WalkResult::Ok(ed)
        }
    }

    #[derive(Debug)]
    pub struct AccumulatorTest {
        result: String,
    }

    impl Accumulator<ElemDataTest, WalkDataTest> for AccumulatorTest {
        fn new(e: &Element, ed: &ElemDataTest) -> Self {
            let result = format!("{}{}", INDENT.repeat(ed.depth), e.name.local_name);
            AccumulatorTest { result }
        }

        fn add(&mut self, wd: &WalkDataTest) -> WalkResult<WalkDataTest, WalkError> {
            self.result += &format!("\n{}", wd.data);
            WalkResult::Ok(WalkDataTest {
                data: self.result.clone(),
            })
        }

        fn summary(&self) -> WalkResult<WalkDataTest, WalkError> {
            WalkResult::Ok(WalkDataTest {
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
            depth: 0,
            element_info: ei.clone(),
            subelements: subelements,
            before_element: Vec::<XmlEvent>::new(),
            content: Vec::<XmlEvent>::new(),
            after_element: Vec::<XmlEvent>::new(),
        }
    }
}
