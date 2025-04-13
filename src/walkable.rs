use crate::xml_document::{Element, ElementInfo, XmlDocument};
use crate::xml_document_factory::DocumentInfo;
use std::convert::Infallible;
use std::ops::{ControlFlow, FromResidual, Try};
use xml::namespace::Namespace;
use xml::attribute::OwnedAttribute;
use xml::common::XmlVersion;
use xml::name::OwnedName;
use xml::reader::XmlEvent;
use std::collections::BTreeMap;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

mod tests {
    use crate::xml_document::{Element, ElementInfo, XmlDocument};
    use crate::xml_document_factory::DocumentInfo;
    use std::collections::BTreeMap;
    //use std::ops::{FromResidual};
    //use crate::xml_document::{Element, ElementInfo, XmlDocument};
    use std::convert::Infallible;
    use std::ops::{ControlFlow, FromResidual, Try};
    use xml::attribute::OwnedAttribute;
    use xml::common::XmlVersion;
    use xml::namespace::Namespace;
    use xml::name::OwnedName;
    use xml::reader::XmlEvent;
    use super::*;


    const INDENT: &str = "    ";

    // ----------------- Result Enums ----------------

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

    // ----------------- Data Types ----------------

    #[derive(Debug)]
    pub struct WalkdataA {
        pub data: String,
    }

    #[derive(Debug)]
    pub struct ElementdataA {
        pub depth: usize,
    }

    impl ElementdataA {
    // FIXME: remove tests::ElementResultA
        pub fn start(&self, element: &Element) -> 
// tests::ElementResultA<Self, Box<dyn Error + Send + Sync>> {
// FIXME: need tests::?
            tests::ElementResultA<ElementdataA, Error> {
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
            let result = format!("{}{}", INDENT.repeat(ed.depth), e.name.local_name);
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
}

// ----------------- Traits ----------------

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

/*
//    #![feature(trait_alias)]
/*
 * XML tree walker
 */

use crate::xml_document::{Element, XmlDocument};

/*
 * Trait for walking an XML document
 * 'a       Lifetime for trait
 * 'b       
 * DATA1    Trait for data passed to walk() and returned as data from
 *          element_start.
 * RET1     Trait for Try value from element_start()
 * DATA2    Trait for data passed from end() 
 * RET2     Trait for Try value from end() and walk_n()
 */
// -----------------------------------------
//use std::error::Error;
////use std::fmt;
use std::convert::Infallible;
use std::ops::{ControlFlow, Try};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

mod tests {
    use std::collections::BTreeMap;
    use std::ops::{FromResidual};
//    use std::marker::PhantomData;
    use crate::xml_document::{Element, ElementInfo, XmlDocument};
    use crate::xml_document_factory::DocumentInfo;
    use xml::attribute::OwnedAttribute;
    use xml::common::XmlVersion;
    use xml::namespace::Namespace;
    use xml::name::OwnedName;
    use xml::reader::XmlEvent;
    use super::*;

    const INDENT: &str = "    ";

    #[allow(dead_code)]
    fn initf() -> XmlDocument {
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

    #[test]
    fn test1() {
//    f: &'a mut fmt::Formatter<'a>,
println!("calling test1");
        let xmldoc = initf();
        frun(&xmldoc);
    }

    // Prints an indented list of elments in the XML document
    #[allow(dead_code)]
    pub fn frun(xmldoc: &XmlDocument) {
        let elemdata_a: ElementdataA = ElementdataA{
            depth:  0,
//            f:      0,
        };
        let a = A {
            xml_document: xmldoc,
        };
        
        let res_a = a.walk(&elemdata_a);
        println!("Result:\n{:?}", res_a);

/*
        assert_eq!(res_a.data, "n1\n".to_owned() +
            INDENT + "n2\n" +
            INDENT + "n3\n" +
            INDENT + INDENT + "n4");
*/
    }

    struct A<'a> {
        xml_document:   &'a XmlDocument,
    }

    impl<'a> A<'_> {
    }

    impl<'a> Walkable for A<'_> {
        fn xml_document(&self) -> &XmlDocument {
            self.xml_document
        }
    }

    #[derive(Debug)]
    pub struct WalkdataA {
        data:   String,
    }

    #[derive(Debug)]
    pub enum WalkResultA<T, E> {
        Err(E),
        Ok(T),
    }

    impl<T, E> WalkResultA<T, E> {
        #[allow(dead_code)]
        fn is_ok(&self) -> bool {
            matches!(self, WalkResultA::Ok(_))
        }

        #[allow(dead_code)]
        fn is_err(&self) -> bool {
            matches!(self, WalkResultA::Err(_))
        }
    }

    impl<T, E> FromResidual<Result<Infallible, E>> for WalkResultA<T, E> {
        fn from_residual(residual: Result<Infallible, E>) -> Self {
            match residual {
                Result::Err(err) => WalkResultA::Err(err),
                Result::Ok(_) => unreachable!(),
            }
        }
    }

    impl<T, E> Try for WalkResultA<T, E> {
        type Output = T;
        type Residual = Result<Infallible, E>;

        fn from_output(output: Self::Output) -> Self {
            WalkResultA::Ok(output)
        }

        fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
            match self {
               WalkResultA::Ok(val) => ControlFlow::Continue(val),
               WalkResultA::Err(err) => ControlFlow::Break(Err(err)),
            }
        }
    }

    /*
     * This has to be a trait so the functions can be defined by users
     */
    pub struct ElementdataA {
        depth:  usize,
    //    f:      &'a mut fmt::Formatter<'a>,

    }

    impl<'a> ElementdataA {
        pub fn start(&self, element: &Element) ->
            ElementResultA<ElementdataA, Error> {
            println!("{}{}", INDENT.repeat(self.depth), element.name);
            ElementResultA::<_, _>::Ok(ElementdataA {
                depth:  self.depth + 1,
    //            f:      element.f,
            })
        }
    }

    #[derive(Debug)]
    pub enum ElementResultA<T, E> {
        Err(E),
        Ok(T),
    }

    impl<'a, T, E> ElementResultA<T, E> {
        #[allow(dead_code)]
        fn is_ok(&self) -> bool {
            matches!(self, ElementResultA::Ok(_))
        }
        
        #[allow(dead_code)]
        fn is_err(&self) -> bool {
            matches!(self, ElementResultA::Err(_))
        }
    }

    impl<'a, T, E> FromResidual<Result<Infallible, E>> for
        ElementResultA<T, E> {
        fn from_residual(residual: Result<Infallible, E>) -> Self {
            match residual {
                Result::Err(e) => ElementResultA::Err(e),
                Result::Ok(_) => unreachable!(),
            }
        }
    }

    impl<'a, T, E> Try for ElementResultA<T, E> {
        type Output = T;
        type Residual = Result<Infallible, E>;

        fn from_output(output: Self::Output) -> Self {
            ElementResultA::Ok(output)
        }

        fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
            match self {
               ElementResultA::Ok(val) => ControlFlow::Continue(val),
               ElementResultA::Err(err) => ControlFlow::Break(Err(err)),
            }
        }
    }

    #[derive(Debug)]
    pub struct AccumulatorA {
        result: String,
    }

    impl<'a> AccumulatorA {
        pub fn new(e: &Element, ed: &ElementdataA) -> Self {
            let result = INDENT.repeat(ed.depth) + &e.name.local_name.clone();
            AccumulatorA {
                result: result,
            }
        }

        // FIXME: what should this return?
        pub fn add(&mut self, ws: WalkdataA) -> WalkResultA<WalkdataA, Error> {
            self.result += &("\n".to_owned() + &ws.data);
            WalkResultA::<WalkdataA, Error>::Ok(WalkdataA {
                // FIXME: is there a performance impact here?
                data: self.result.clone()
            })
        }

        pub fn summary(&self) -> WalkResultA<WalkdataA, Error>{
            WalkResultA::<_, _>::Ok(WalkdataA {
                data:   self.result.clone(),
            })
        }
    }
}

pub trait Walkable
    {
    fn xml_document(&self) -> &XmlDocument;
        
    fn walk<'a>(&self, d: &tests::ElementdataA) ->
        tests::WalkResultA<tests::WalkdataA, Error> {
        let document = self.xml_document();
        let e = &document.root;
        self.walk_i(&e, &d)
    }

    fn walk_i<'a>(&self, e: &Element, ed: &tests::ElementdataA) ->
        tests::WalkResultA<tests::WalkdataA, Error> {
        let subd = ed.start(e)?;
        let mut d = tests::AccumulatorA::new(e, ed);

        for sub_e in &e.subelements {
            let wd = self.walk_i(&sub_e, &subd)?;
            d.add(wd)?;
        }

        let result = d.summary();
        result
    }
}

pub trait WalkData {}
*/
