use crate::xml_document::{Element, XmlDocument};

//use std::convert::Infallible;
//use std::ops::{/*ControlFlow, */FromResidual, Try};

pub type WalkError = Box<dyn std::error::Error + Send + Sync + 'static>;

// ----------------- Traits ----------------

//pub trait WalkableResult: FromResidual + Try<Output = dyn WalkData> {}
//pub trait WalkableResult<E>: FromResidual<Result<Infallible, E>> +
//    Try<Output = WalkData, Residual = E> {}
pub type WalkableResult<E> = Result<WalkDataType, E>;

// Information that supplements the Element to produce a piece of the overall
// result.
pub trait ElemData
where
{
    fn next_level(&self, element: &Element) -> Result<ElemDataType, WalkError>;
}

pub struct ElemDataType {
    _x:  u8,
}

impl ElemData for ElemDataType {
    fn next_level(&self, _element: &Element) -> Result<ElemDataType, WalkError> {
        Ok(ElemDataType {
            _x:  0,
        })
    }
}

pub struct AccumulatorType {
}

impl Accumulator<'_> for AccumulatorType {
    fn new(_e: &Element, _ed: &ElemDataType) -> Self
    where
        Self: Sized
    {
        AccumulatorType {}
    }

    fn add(&mut self, _wd: &WalkDataType) -> Result<(), WalkError> {
        Ok(())
    }

    fn summary(&self) -> WalkableResult<WalkError> {
        Ok(WalkDataType {
        })
    }
}

pub struct WalkDataType {
}

impl WalkData for WalkDataType {
}

// It seems as though ED and WalkDataType should be traits
pub trait Walkable<'a>
where
{
    fn xml_document(&self) -> &XmlDocument;

    // Start the walk at the root of the document
    fn walk(&self, d: ElemDataType) -> WalkableResult<WalkError>
    where
        Self:   Sized,
    {
        let xml_doc = self.xml_document();
        let root = &xml_doc.root;
        self.walk_i(root, &d)
    }

    fn walk_i<'e>(&self, element: &'e Element, ed: &ElemDataType) -> WalkableResult<WalkError>
    where
        Self:   Sized,
    {
        let mut acc = AccumulatorType::new(element, ed);
        let next_ed = ed.next_level(element)?;

        let mut wd_vec = Vec::<WalkDataType>::new();

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

pub trait WalkData {}

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

pub trait Accumulator<'a>
where
{
    fn new(e: &Element, ed: &ElemDataType) -> Self
    where
        Self: Sized;
    fn add(&mut self, wd: &WalkDataType) -> Result<(), WalkError>;
    fn summary(&self) -> WalkableResult<WalkError>;
}

/*
#[cfg(test)]
mod tests {
    use thiserror::Error;

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

    #[derive(Debug, Error)]
    pub enum TestError {
        #[error("Error1")]
        Error1(),

        #[error("Error2")]
        Error2(),

        #[error("Error3")]
        Error3(),
    }

    #[test]
    fn test_walk_tree_names() {
        let doc = create_test_doc(); // build a sample XmlDocument
        let walker = TestWalkable { xml_document: &doc };
        println!("Processing:");
        let ed = &TestElemData::new(0);
        let result = walker.walk(&ed);

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

    struct TestWalkable<'a> {
        xml_document: &'a XmlDocument,
    }

    impl Walkable<'_, TestElemData, TestWalkData, TestAccumulator, Result<TestWalkData, TestError>> for TestWalkable<'_> {
        fn xml_document(&self) -> &XmlDocument {
            self.xml_document
        }
    }

    // ----------------- Data Types ----------------

    type TestWalkableType<'a> = Box<&'a dyn Walkable<'a, TestElemData, TestWalkData, TestAccumulator, TestResultType>>;
    type TestResultType = Result<TestWalkData, WalkError>;

    #[derive(Debug)]
    pub struct TestWalkData {
        pub data: String,
    }

    impl WalkData for TestWalkData {}

    #[derive(Debug)]
    pub struct TestElemData {
        pub depth: usize,
    }

    impl<TestElemData, TestElemResult>
        ElemData<'_, TestElemData, TestElemResult>
        for TestElemData {
        fn start<'a>(&'a mut self, w: TestWalkableType, element: Element) ->
            WalkResult<TestElemData, WalkError> {
            println!("{}{}", INDENT.repeat(self.depth), element.name.local_name);
            let ed = TestElemData {
                depth: self.depth + 1,
            };

            ElemResult::Ok(ed)
        }
    }

    #[derive(Debug)]
    pub struct TestAccumulator {
        result: String,
    }

    impl
//<TestElemData, TestWalkData, TestAccumulator, TestResult>
         Accumulator<'_, TestElemData, TestWalkData, TestAccumulator, TestResultType> for TestAccumulator {
        fn new(e: &Element, ed: &TestElemData) -> Self {
            let result = format!("{}{}", INDENT.repeat(ed.depth), e.name.local_name);
            TestAccumulator { result }
        }

        fn add(&mut self, wd: &TestWalkData) -> WalkResult<TestWalkData, WalkError> {
            self.result += &format!("\n{}", wd.data);
            WalkResult::Ok(TestWalkData {
                data: self.result.clone(),
            })
        }

        fn summary(&self) -> WalkResult<TestWalkData, WalkError> {
            WalkResult::Ok(TestWalkData {
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
*/

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
