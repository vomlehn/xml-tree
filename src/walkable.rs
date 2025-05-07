use crate::xml_document::{Element, XmlDocument};
pub type WalkError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type WalkableResult<E, WD> = Result<WD, E>;

// ----------------- Traits ----------------
// Information that supplements the Element to produce a piece of the overall
// result.
pub trait ElemData {
    type Output;
    
    fn next_level(&self, element: &Element) -> Result<Self::Output, WalkError>;
}

pub trait WalkData {}

pub trait Accumulator<'a, ED, WD> 
where
    ED: ElemData,
    WD: WalkData,
{
    fn new(e: &Element, ed: &ED) -> Self
    where
        Self: Sized;
    fn add(&mut self, wd: &WD) -> Result<impl WalkData, WalkError>;
    fn summary(&self) -> WalkableResult<WalkError, WD>;
}

pub trait Walkable<'a, AC, ED, WD>
where
    AC: Accumulator<'a, ED, WD>,
    ED: ElemData<Output = ED>,  // This restriction ensures ED::Output is the same as ED
    WD: WalkData,
{
    fn xml_document(&self) -> &XmlDocument;
    
    // Start the walk at the root of the document
    fn walk(&self, d: ED) -> WalkableResult<WalkError, WD>
    where
        Self: Sized,
    {
        let xml_doc = self.xml_document();
        let root = &xml_doc.root;
        self.walk_i(root, &d)
    }
    
    fn walk_i<'e>(&self, element: &'e Element, ed: &ED) -> WalkableResult<WalkError, WD>
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
        
        let wr = acc.summary()?;
        Ok(wr)
    }
}

#[cfg(test)]
mod tests {
/*
//    use thiserror::Error;

    use std::collections::BTreeMap;
    use xml::attribute::OwnedAttribute;
    use xml::common::XmlVersion;
    use xml::name::OwnedName;
    use xml::namespace::Namespace;
    use xml::reader::XmlEvent;

    use crate::xml_document::{Element, ElementInfo, XmlDocument};
    use crate::xml_document_factory::DocumentInfo;
    use super::{Accumulator, ElemData, WalkData, WalkError, Walkable};
    use super::{WalkableResult};

    pub struct AccumulatorType {
        z:  u8,
    }

    impl Accumulator<'_> for AccumulatorType {
        fn new(_e: &Element, _ed: &ElemDataType) -> Self
        where
            Self: Sized
        {
            AccumulatorType {
                z:  0
            }
        }

        fn add(&mut self, _wd: &WalkDataType) -> 
            WalkResult<TestWalkData, WalkError> {
            self.z += 1;
            Ok(())
        }

        fn summary(&self) -> WalkableResult<WalkError> {
            Ok(WalkDataType {
            })
        }
    }


    pub struct ElemDataType {
        x:  u8,
    }

    impl ElemData for ElemDataType {
        fn next_level(&self, _element: &Element) -> Result<ElemDataType, WalkError> {
    println!("next_level:");
            Ok(ElemDataType {
                x:  0,
            })
        }
    }
    
    #[derive(Debug)]
    pub struct WalkDataType {
    }

    impl WalkData for WalkDataType {
    }

    #[test]
    fn build_to_traits() {
        struct WalkableType<'a> {
            xml_doc:    &'a XmlDocument,
            count:      u8,
        }

        impl<'a> Walkable<'a> for WalkableType<'_> {
            fn xml_document(&self) -> &XmlDocument {
                self.xml_doc
            }
        }

        let xml_doc = create_test_doc();

        let w = WalkableType {
            xml_doc:    &xml_doc,
            count:      0,
        };

        let e = ElemDataType {
            x:  100,
        };

        let result = match w.walk(e) {
            Err(e) => panic!("walk() error: {:?}", e),
            Ok(result) => result,
        };
        println!("walk() result: {:?}", result);
        println!("count {}", w.count);
    }

*/   
    
//    use thiserror::Error;

    use std::collections::BTreeMap;
    use xml::attribute::OwnedAttribute;
    use xml::common::XmlVersion;
    use xml::name::OwnedName;
    use xml::namespace::Namespace;
    use xml::reader::XmlEvent;

    use crate::xml_document::{Element, ElementInfo, XmlDocument};
    use crate::xml_document_factory::DocumentInfo;

    use super::{Accumulator, ElemData, WalkData, WalkError, Walkable};
    use super::{WalkableResult};

    const INDENT: &str = "    ";

/*
    #[derive(Debug, Error)]
    pub enum TestError {
        #[error("Error1")]
        Error1(),

        #[error("Error2")]
        Error2(),

        #[error("Error3")]
        Error3(),
    }
*/

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

    impl Walkable<'_, TestAccumulator, TestElemData, TestWalkData> for TestWalkable<'_> {
        fn xml_document(&self) -> &XmlDocument {
            self.xml_document
        }
    }

    // ----------------- Data Types ----------------

//    type TestWalkableType<'a> = Box<&'a dyn Walkable<'a, TestAccumulator, TestElemData, TestWalkData>>;
//    type TestResultType = Result<TestWalkData, WalkError>;

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

    #[derive(Debug)]
    pub struct TestAccumulator {
        result: String,
    }

    impl
         Accumulator<'_, TestElemData, TestWalkData> for TestAccumulator {
        fn new(e: &Element, ed: &TestElemData) -> Self {
            let result = format!("{}{}", INDENT.repeat(ed.depth), e.name.local_name);
            TestAccumulator { result }
        }

        fn add(&mut self, wd: &TestWalkData) -> 
            Result<TestWalkData, WalkError> {
            self.result += &format!("\n{}", wd.data);
            Result::Ok(TestWalkData {
                data: self.result.clone(),
            })
        }

        fn summary(&self) -> WalkableResult<WalkError, TestWalkData> {
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

/*
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Try for MyResult<T, E> {
    type Output = T;
    type Residual = Result<Infallible, E>;

    fn from_output(output: T) -> Self {
        MyResult::Ok(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, T> {
        match self {
            MyResult::Ok(v) => ControlFlow::Continue(v),
            MyResult::Err(e) => ControlFlow::Break(Result::Err(e)),
        }
    }
}

impl<T, E, F: From<E>> FromResidual<Result<Infallible, E>> for MyResult<T, F> {
    fn from_residual(residual: Result<Infallible, E>) -> Self {
        match residual {
            Err(e) => MyResult::Err(From::from(e)),
            _ => unreachable!(),
        }
    }
}
*/

// ----------------- Result Enums ----------------

/*
trait WalkResult: Try + FromResidual<Result<Infallible, E>> {
}
*/

/*
#[derive(Debug)]
pub enum WalkResult<T, E>
where
    T:  WalkData
{
    Ok(T),
    Err(E),
}

impl<T, E> Try for WalkResult<T, E>
where
    T:  WalkData
{
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

impl<T, E> FromResidual<Result<Infallible, E>> for WalkResult<T, E>
where
    T:  WalkData
{
    fn from_residual(residual: Result<Infallible, E>) -> Self {
        match residual {
            Err(e) => WalkResult::Err(e),
            Ok(_) => unreachable!(),
        }
    }
}
*/
