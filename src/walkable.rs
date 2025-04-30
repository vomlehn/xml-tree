use crate::xml_document::{Element, XmlDocument};

use std::convert::Infallible;
use std::ops::{ControlFlow, FromResidual, Try};

type WalkError = Box::<dyn std::error::Error + Send + Sync + 'static>;

// ----------------- Traits ----------------
// Information that supplements the Element to produce a piece of the overall
// result.
pub trait ElementSup<ES>
    where 
        ES: ElementSup<ES>
    {
    fn start(&self, element: &Element) -> 
        ElementResult<ES, WalkError>;
}

// It seems as though ES and WD should be traits
pub trait Walkable<ES, WD, AC>
    where
        ES: ElementSup<ES>,
        WD: WalkData,
        AC: Accumulator<ES, WD>,
    {
    fn xml_document(&self) -> &XmlDocument;

    // Start the walk at the root of the document
    fn walk<'a>(&self, d: &ES) -> WalkResult<WD, WalkError> {
        let xml_doc = self.xml_document();
        let root = &xml_doc.root;
        self.walk_i(root, d)
    }

    fn walk_i<'a>(&self, element: &Element, es: &ES) ->
        WalkResult<WD, WalkError> {
        let next_es = match es.start(element) {
            // FIXME: return WalkError here and below
            ElementResult::Err(e) => panic!("es.start {:?}", e),
            ElementResult::Ok(next_es) => next_es,
        };

        let mut acc = AC::new(element, &next_es);

        for elem in &element.subelements {
            let wd = match self.walk_i(&elem, &next_es) {
                WalkResult::Err(e) => panic!("self.walk_i {:?}", e),
                WalkResult::Ok(wd) => wd,
            };

            match acc.add(&wd) {
                WalkResult::Err(e) => panic!("acc.add {:?}", e),
                WalkResult::Ok(wr) => wr,
            };
        }

        let wr = acc.summary();
        wr
    }
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

pub trait Accumulator<ES, WD>
where
    ES: ElementSup<ES>,
    WD: WalkData,
 {
        fn new(e: &Element, es: &ES) -> Self;
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

    use super::{Accumulator, ElementSup, WalkError, Walkable, WalkData, ElementResult, WalkResult};

    const INDENT: &str = "    ";

    #[test]
    fn test_walk_tree_names() {
        let doc = create_test_doc(); // build a sample XmlDocument
        let walker = WalkableA { xml_document: &doc };
        let result = walker.walk(&ElementdataA { depth: 0 });

        match result {
            WalkResult::Ok(data) => println!("Output:\n{}", data.data),
            WalkResult::Err(e) => eprintln!("Error: {}", e),
        }
    }

    struct WalkableA<'a> {
        xml_document:   &'a XmlDocument,
    }

    impl Walkable<ElementdataA, WalkdataA, AccumulatorA> for WalkableA<'_>
    {
        fn xml_document(&self) -> &XmlDocument {
            self.xml_document
        }

        fn walk<'a>(&self, d: &ElementdataA) ->
            WalkResult<WalkdataA, WalkError> {
            let root = &<WalkableA<'_> as Walkable<ElementdataA, WalkdataA, AccumulatorA>>::xml_document(self).root;
            self.walk_i(root, &d)
        }

        fn walk_i<'a>(&self, e: &Element, ed: &ElementdataA) ->
            WalkResult<WalkdataA, WalkError> {
            let subd = ed.start(e)?;
            let mut d = AccumulatorA::new(e, &ed);

            for sub_e in &e.subelements {
                let wd = self.walk_i(sub_e, &subd)?;
                d.add(&wd)?;
            }

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

    impl ElementSup<ElementdataA> for ElementdataA {
        fn start(&self, element: &Element) -> 
            ElementResult<ElementdataA, WalkError> {
            println!("{}{}", INDENT.repeat(self.depth), element.name.local_name);
            let ed = ElementdataA {
                depth: self.depth + 1,
            };
            ElementResult::Ok(ed)
        }
    }

    #[derive(Debug)]
    pub struct AccumulatorA {
        result: String,
    }

    impl AccumulatorA {
        fn clone(&self) -> AccumulatorA {
            AccumulatorA {
                result: self.result.clone(),
            }
        }
    }

    impl Accumulator<ElementdataA, WalkdataA> for AccumulatorA
     {
        fn new(e: &Element, ed: &ElementdataA) -> Self {
            let result = format!("{}{}", INDENT.repeat(ed.depth),
                e.name.local_name);
            AccumulatorA { result }
        }

        fn add(&mut self, ws: &WalkdataA) -> WalkResult<WalkdataA, WalkError> {
            self.result += &format!("\n{}", ws.data);
            WalkResult::Ok(WalkdataA {
                data: self.result.clone(),
            })
        }

        fn summary(&self) -> WalkResult<WalkdataA, WalkError> {
            WalkResult::Ok(WalkdataA {
                data: self.result.clone(),
            })
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

        let e1 = branch("n1", vec!(
            leaf("n2", &ei),
            branch("n3", vec!(
                leaf("n4", &ei),
            ), &ei),
        ), &ei);
                
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

    fn leaf(name: &str, ei: &ElementInfo) -> Element {
        Element {
            name:           OwnedName {
                local_name: name.to_string(),
                namespace: None,
                prefix: None,
            },
            depth:          0,
            element_info:   ei.clone(),
            subelements:    Vec::<Element>::new(),
            before_element: Vec::<XmlEvent>::new(),
            content:        Vec::<XmlEvent>::new(),
            after_element:  Vec::<XmlEvent>::new(),
        }
    }

    fn branch(name: &str, subelements: Vec<Element>, ei: &ElementInfo) -> Element {
        Element {
            name:           OwnedName {
                local_name: name.to_string(),
                namespace: None,
                prefix: None,
            },
            depth:          0,
            element_info:   ei.clone(),
            subelements:    subelements,
            before_element: Vec::<XmlEvent>::new(),
            content:        Vec::<XmlEvent>::new(),
            after_element:  Vec::<XmlEvent>::new(),
        }
    }
}
