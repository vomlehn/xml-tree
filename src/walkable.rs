// FIXME: remove tests:: everywhere
use crate::xml_document::{Element, ElementInfo, XmlDocument};
use crate::xml_document_factory::DocumentInfo;

use std::collections::BTreeMap;

use xml::attribute::OwnedAttribute;
use xml::common::XmlVersion;
use xml::name::OwnedName;
use xml::namespace::Namespace;
use xml::reader::XmlEvent;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;


// ----------------- Traits ----------------
trait ElementData {
    fn start(&self, element: &Element) -> 
// FIXME: remove tests::
        tests::ElementResultA<tests::ElementdataA, Error>;
}

pub trait Walkable {
    fn xml_document(&self) -> &XmlDocument;

// FIXME: remove tests::WalkResultA
    fn walk<'a>(&self, d: &tests::ElementdataA) ->
// FIXME: remove tests::
    tests::WalkResultA<tests::WalkdataA, Error> {
// tests::WalkResultA<WalkdataA, Box<dyn Error + Send + Sync>> {
        let root = &self.xml_document().root;
        self.walk_i(root, d)
    }

    fn walk_i<'a>(
        &self,
        e: &Element,
// FIXME: remove tests::
        ed: &tests::ElementdataA,
// FIXME: remove tests::
    ) -> tests::WalkResultA<tests::WalkdataA, Error> {
        let subd = ed.start(e)?;
// FIXME: remove tests::
        let mut d = tests::AccumulatorA::new(e, ed);

        for sub_e in &e.subelements {
            let wd = self.walk_i(sub_e, &subd)?;
            d.add(wd)?;
        }

        d.summary()
    }
}

pub trait WalkData {}

pub trait XmlWalker {
    fn start(&mut self, element: &Element, depth: usize);
    fn end(&mut self, _element: &Element, _depth: usize) {}
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

//#[cfg(test)]
mod tests {
//    use crate::xml_document::{Element, XmlDocument};
    use std::convert::Infallible;
    use std::ops::{ControlFlow, FromResidual, Try};
    use super::*;

    const INDENT: &str = "    ";

    #[test]
    fn test_walk_tree() {
        let doc = create_test_doc(); // build a sample XmlDocument
    // FIXME: Remove tests::
        let walker = tests::A { xml_document: &doc };

    // FIXME: Remove tests::
        let result = walker.walk(&tests::ElementdataA { depth: 0 });

        match result {
    // FIXME: Remove tests::
            tests::WalkResultA::Ok(data) => println!("Output:\n{}", data.data),
    // FIXME: Remove tests::
            tests::WalkResultA::Err(e) => eprintln!("Error: {}", e),
        }
    }

    // ----------------- Data Types ----------------

    #[derive(Debug)]
    pub struct WalkdataA {
        pub data: String,
    }

    #[derive(Debug)]
    pub struct ElementdataA {
        pub depth: usize,
    }

    impl ElementData for tests::ElementdataA {
        fn start(&self, element: &Element) -> 
            ElementResultA<ElementdataA, Error> {
            println!("{}{}", INDENT.repeat(self.depth), element.name.local_name);
            ElementResultA::Ok(Self {
                depth: self.depth + 1,
            })
        }
    }

    #[derive(Debug)]
    pub struct AccumulatorA {
        result: String,
    }

    impl AccumulatorA {
        pub fn new(e: &Element, ed: &ElementdataA) -> Self {
            let result = format!("{}{}", INDENT.repeat(ed.depth),
                e.name.local_name);
            AccumulatorA { result }
        }

        pub fn add(&mut self, ws: WalkdataA) ->
// WalkResultA<WalkdataA, Box<dyn Error + Send + Sync>> {
            WalkResultA<WalkdataA, Error> {
            self.result += &format!("\n{}", ws.data);
            WalkResultA::Ok(WalkdataA {
                data: self.result.clone(),
            })
        }

        pub fn summary(&self) -> WalkResultA<WalkdataA, Error> {
// WalkResultA<WalkdataA, Box<dyn Error + Send + Sync>> {
            WalkResultA::Ok(WalkdataA {
                data: self.result.clone(),
            })
        }
    }

    pub struct A<'a> {
        pub xml_document: &'a XmlDocument,
    }

    impl<'a> Walkable for A<'a> {
        fn xml_document(&self) -> &XmlDocument {
            self.xml_document
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
    pub enum WalkResultA<T, E> {
        Ok(T),
        Err(E),
    }

    impl<T, E> Try for WalkResultA<T, E> {
        type Output = T;
        type Residual = Result<Infallible, E>;

        fn from_output(output: T) -> Self {
            WalkResultA::Ok(output)
        }

        fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
            match self {
                WalkResultA::Ok(v) => ControlFlow::Continue(v),
                WalkResultA::Err(e) => ControlFlow::Break(Err(e)),
            }
        }
    }

    impl<T, E> FromResidual<Result<Infallible, E>> for WalkResultA<T, E> {
        fn from_residual(residual: Result<Infallible, E>) -> Self {
            match residual {
                Err(e) => WalkResultA::Err(e),
                Ok(_) => unreachable!(),
            }
        }
    }
}
