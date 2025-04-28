// FIXME: remove tests:: everywhere
use crate::xml_document::{Element, ElementInfo, XmlDocument};
use crate::xml_document_factory::DocumentInfo;

use std::collections::BTreeMap;
use std::convert::Infallible;
use std::ops::{ControlFlow, FromResidual, Try};

use xml::attribute::OwnedAttribute;
use xml::common::XmlVersion;
use xml::name::OwnedName;
use xml::namespace::Namespace;
use xml::reader::XmlEvent;

type Error = Box::<dyn std::error::Error + Send + Sync + 'static>;

// ----------------- Traits ----------------
trait ElementDataTrait {
    fn start(&self, element: &Element) -> 
        ElementResult<Box<dyn ElementDataTrait>, Error>;
}

// It seems as though ED and WD should be traits
pub trait Walkable<ED, WD>
    where
        ED: ElementDataTrait,
        WD: WalkData
    {
    fn xml_document(&self) -> &XmlDocument;
    fn walk<'a>(&self, d: &ED) ->
        WalkResult<&WD, Error>;
    fn walk_i<'a>(&self, e: &Element, ed: Box<dyn ElementDataTrait>) ->
        WalkResult<&WD, Error>;
}

pub trait WalkData {}

pub trait XmlWalker {
    fn start(&mut self, element: &Element, depth: usize);
    fn end(&mut self, _element: &Element, _depth: usize) {}
}

// ----------------- Result Enums ----------------

#[derive(Debug)]
pub enum ElementResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Try for ElementResult<T, E> {
    type Output = T;
    type Residual = Result<Infallible, E>;

    fn from_output(output: T) -> Self {
        ElementResult::Ok(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            ElementResult::Ok(v) => ControlFlow::Continue(v),
            ElementResult::Err(e) => ControlFlow::Break(Err(e)),
        }
    }
}

impl<T, E> FromResidual<Result<Infallible, E>> for ElementResult<T, E> {
    fn from_residual(residual: Result<Infallible, E>) -> Self {
        match residual {
            Err(e) => ElementResult::Err(e),
            Ok(_) => unreachable!(),
        }
    }
}

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

trait Accumulator<ED, WD>
where
    ED: ElementDataTrait,
    WD: WalkData,
 {
        fn new(e: &Element, ed: &ED) -> Self;
        fn add(&mut self, ws: &WD) -> WalkResult<WD, Error>;
        fn summary(&self) -> WalkResult<WD, Error>;
}
/*

// FIXME: uncover this
//#[cfg(test)]
mod tests {
    use std::convert::Infallible;
    use std::ops::{ControlFlow, FromResidual, Try};
    use super::*;

    const INDENT: &str = "    ";

    #[test]
    fn test_walk_tree() {
        let doc = create_test_doc(); // build a sample XmlDocument
        let walker = A { xml_document: &doc };
        let result = walker.walk(&ElementdataA { depth: 0 });

        match result {
            WalkresultA::Ok(data) => println!("Output:\n{}", data.data),
            WalkresultA::Err(e) => eprintln!("Error: {}", e),
        }
    }

    struct WalkableA<'a> {
        xml_document:   &'a XmlDocument,
    }

    impl<ED, WD> Walkable<ED, WD> for WalkableA<'_>
        where
            // Must be traits
            ED: ElementData,
            WD: WalkData,
    {
        fn xml_document(&self) -> &XmlDocument {
            self.xml_document
        }

        fn walk<'a>(&self, d: &ED) ->
            WalkResult<&WD, Error> {
            let root = &<WalkableA<'_> as Walkable<ED, WD>>::xml_document(self).root;
            self.walk_i(root, Box::newi(d))
        }

        fn walk_i<'a>(&self, e: &Element, ed: Box<&ED>) ->
            WalkResult<&WD, Error> {
            let subd = ed.start(e)?;
            let mut d = AccumulatorA::new(e, ed);

let x: i8 = subd;
            for sub_e in &e.subelements {
                let wd = self.walk_i(sub_e, Box::new(subd))?;
                d.add(wd)?;
            }

// FIXME: clone() to right solution?
            d.clone().summary()
        }
    }

    // ----------------- Data Types ----------------

    #[derive(Debug)]
    pub struct WalkdataA {
        pub data: String,
    }

    impl WalkData for WalkdataA {
    }

    #[derive(Debug)]
    pub struct ElementdataA {
        pub depth: usize,
    }

    impl ElementData for ElementdataA {
        fn start(&self, element: &Element) -> 
            ElementResult<Box<dyn ElementData>, Error> {
            println!("{}{}", INDENT.repeat(self.depth), element.name.local_name);
            let ed = ElementdataA {
                depth: self.depth + 1,
            };
            ElementResultA::Ok(ed)
        }
    }

    #[derive(Debug)]
    pub struct AccumulatorA {
        result: String,
    }

    impl AccumulatorA {
        fn clone(&self) -> AccumulatorA {
            AccumulatorA {
                result: self.result,
            }
        }
    }

    impl<ED, WD> Accumulator<ED, WD> for AccumulatorA
    where
        ED: ElementData,
        WD: WalkData,
     {
        fn new(e: &Element, ed: &ED) -> Self {
            let result = format!("{}{}", INDENT.repeat(ed.depth),
                e.name.local_name);
            AccumulatorA { result }
        }

        fn add(&mut self, ws: &WD) -> WalkResult<WD, Error> {
            self.result += &format!("\n{}", ws.data);
            WalkResult::Ok(WalkdataA {
                data: self.result.clone(),
            })
        }

        fn summary(&self) -> WalkResult<WD, Error> {
            WalkResult::Ok(WalkdataA {
                data: self.result.clone(),
            })
        }
    }

    // ----------------- Result Enums ----------------

    #[derive(Debug)]
    pub enum ElementResultA<T, E> {
        Ok(T),
        Err(E),
    }

    impl<T, E> Try for ElementResultA<T, E> {
        type Output = T;
        type Residual = Result<Infallible, E>;

        fn from_output(output: T) -> Self {
            ElementResultA::Ok(output)
        }

        fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
            match self {
                ElementResultA::Ok(v) => ControlFlow::Continue(v),
                ElementResultA::Err(e) => ControlFlow::Break(Err(e)),
            }
        }
    }

    impl<T, E> FromResidual<Result<Infallible, E>> for ElementResultA<T, E> {
        fn from_residual(residual: Result<Infallible, E>) -> Self {
            match residual {
                Err(e) => ElementResultA::Err(e),
                Ok(_) => unreachable!(),
            }
        }
    }

    #[derive(Debug)]
    pub enum WalkresultA<T, E> {
        Ok(T),
        Err(E),
    }

    impl<T, E> Try for WalkresultA<T, E> {
        type Output = T;
        type Residual = Result<Infallible, E>;

        fn from_output(output: T) -> Self {
            WalkresultA::Ok(output)
        }

        fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
            match self {
                WalkresultA::Ok(v) => ControlFlow::Continue(v),
                WalkresultA::Err(e) => ControlFlow::Break(Err(e)),
            }
        }
    }

    impl<T, E> FromResidual<Result<Infallible, E>> for WalkresultA<T, E> {
        fn from_residual(residual: Result<Infallible, E>) -> Self {
            match residual {
                Err(e) => WalkresultA::Err(e),
                Ok(_) => unreachable!(),
            }
        }
    }

    // ----------------- Data Types ----------------

    fn create_test_doc() -> XmlDocument {
    println!("In create_test_doc");
        let ns: Namespace = Namespace(BTreeMap::<String, String>::new());

        let ei: ElementInfo = ElementInfo {
            lineno:     1,
            attributes: Vec::<OwnedAttribute>::new(),
            namespace:  ns,
        };

        let e4: Element = Element {
            name:           OwnedName {
                local_name: "n4".to_string(),
                namespace: None,
                prefix: None,
            },
            depth:          0,
            element_info:   ei.clone(),
            subelements:    Vec::<Element>::new(),
            before_element: Vec::<XmlEvent>::new(),
            content:        Vec::<XmlEvent>::new(),
            after_element:  Vec::<XmlEvent>::new(),
        };

        let e3: Element = Element {
            name:           OwnedName { local_name: "n3".to_string(), namespace: None,
                                prefix: None},
            depth:          0,
            element_info:   ei.clone(),
            subelements:    vec!(e4),
            before_element: Vec::<XmlEvent>::new(),
            content:        Vec::<XmlEvent>::new(),
            after_element:  Vec::<XmlEvent>::new(),
        };

        let e2: Element = Element {
            name:           OwnedName { local_name: "n2".to_string(), namespace: None,
                                prefix: None},
            depth:          0,
            element_info:   ei.clone(),
            subelements:    Vec::<Element>::new(),
            before_element: Vec::<XmlEvent>::new(),
            content:        Vec::<XmlEvent>::new(),
            after_element:  Vec::<XmlEvent>::new(),
        };

        let e1: Element = Element {
            name:           OwnedName { local_name: "n1".to_string(), namespace: None,
                                prefix: None},
            depth:          0,
            element_info:   ei.clone(),
            subelements:    vec!(e2, e3),
            before_element: Vec::<XmlEvent>::new(),
            content:        Vec::<XmlEvent>::new(),
            after_element:  Vec::<XmlEvent>::new(),
        };

        let di = DocumentInfo {
            version:    XmlVersion::Version10,
            encoding:   "xxx".to_string(),
            standalone: None,
        };

        let d: XmlDocument = XmlDocument {
            root:   e1,
            document_info:  di,
        };

        d
    }
}
*/
