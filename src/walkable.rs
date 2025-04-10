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
use std::error::Error;
use std::fmt;
use std::ops::Try;

mod tests {
    use std::collections::BTreeMap;
    use std::marker::PhantomData;
    use crate::xml_document::{Element, ElementInfo, XmlDocument};
    use crate::xml_document_factory::DocumentInfo;
    use xml::attribute::OwnedAttribute;
    use xml::common::XmlVersion;
    use xml::namespace::Namespace;
    use xml::name::OwnedName;
    use xml::reader::XmlEvent;
    use super::*;

    fn initf() -> XmlDocument {
        let ns: Namespace = Namespace(BTreeMap::<String, String>::new());

        let ei: ElementInfo = ElementInfo {
            lineno:     1,
            attributes: Vec::<OwnedAttribute>::new(),
            namespace:  ns,
        };

        let e4: Element = Element {
            name:           OwnedName { local_name: "n4".to_string(), namespace: None,
                                prefix: None},
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
    pub fn frun(xmldoc: &XmlDocument) {
        let elemdata_a: ElementdataA = ElementdataA{
            depth:  0,
//            f:      0,
        };
        let a = A {
            xml_document: xmldoc,
//            marker1: PhantomData,
//            marker2: PhantomData,
        };
        let res_a = a.walk(&elemdata_a);
        println!("res_a: {:?}", res_a);
    }

    // AC   Accumulator trait
    // WD   Working data trait
    struct A<'a> {
        xml_document:   &'a XmlDocument,
//        marker1:    PhantomData<AC>,
//        marker2:    PhantomData<WD>,
    }

    impl<'a> A<'_> {
    }

    impl<'a> Walkable for A<'_> {
        fn xml_document(&self) -> &XmlDocument {
            self.xml_document
        }
    }

/*
    impl<'a, AC: Accumulator, WD: WalkData> Walkable<AC, WD> for A<'a, AC, WD> {
//        type AC = dyn Accumulator<WD = WalkdataA>;
//        type AS = Result<Self::AC, Box<dyn Error>>;
        type ED = ElementdataA;
        type ES = Result<Self::ED, Box<dyn Error>>;
//        type WD = u8;
        type WS = Result<WD, Box<dyn Error>>;

        fn xml_document(&self) -> &XmlDocument {
            self.xml_document
        }
    }
*/

/*
    impl WalkdataA {
    }

    impl WalkData for WalkdataA {
    }
*/

/*
    impl ElementData for ElementdataA {
        type WD = u8;
        type WS = Result<Self::WD, Box<dyn Error>>;

        fn summary(&self) -> Self::WS {
            Ok(37)
        }
    }
*/

//     struct B<'a> {
//         xml_document:   &'a XmlDocument,
//     }
// 
//     impl<'a> Walkable for B<'a> {
//         type ED = Bd;
//         type ES = Result<Self::ED, Box<dyn Error>>;
// //        type WD = ();
//         type WS = Result<Self::WD, Box<dyn Error>>;
// 
//         fn xml_document(&self) -> &XmlDocument {
//             self.xml_document
//         }
//     }
// 
//     struct Bd {
//     }
// 
//     impl ElementData for Bd {
//         type WD = ();
//         type WS = fmt::Result;
// 
//         fn summary(&self) -> Self::WS {
//             Ok(())
//         }
//     }
}

struct WalkdataA {
}

type WalkstatusA<T, E> = Result<T, Box<E>>;

/*
 * This has to be a trait so the functions can be defined by users
 */
pub struct ElementdataA {
    depth:  usize,
//    f:      &'a mut fmt::Formatter<'a>,
//    type WD;
//    type WS: Try;

/*
    fn end(&mut self, element: &Element,
        subelements: Vec<Box<dyn W>>) ->
        Result<Box<dyn W>, Box<dyn Error>>;
*/
}

impl<'a> ElementdataA {
    fn start(&self, element: &Element) -> ElementstatusA<ElementdataA, dyn Error> {
        println!("{}{}", "    ".repeat(self.depth), element.name);
        Ok(ElementdataA {
            depth:  self.depth + 1,
//            f:      element.f,
        })
    }
    fn summary(&self) -> WalkstatusA<(), dyn Error> {
        Ok(())
    }
}

type ElementstatusA<T, E> = Result<T, Box<E>>;

pub struct AccumulatorA {
}

impl AccumulatorA {
    fn new() -> Self {
        AccumulatorA {}
    }
    fn add(&mut self, ws: WalkstatusA<(), dyn Error>) {
    }
    fn summary(&self) -> WalkstatusA <(), dyn Error>{
        Ok(())
    }
}

pub trait Walkable
    {
    fn xml_document(&self) -> &XmlDocument;
        
    fn walk(&self, d: &ElementdataA) -> WalkstatusA<(), dyn Error> {
        let document = self.xml_document();
        let e = &document.root;
println!("walk(): start at {}", e.name);
        self.walk_i(&e, &d)
    }

    fn walk_i(&self, e: &Element, d: &ElementdataA) -> WalkstatusA<(), dyn Error> {
        let subd = d.start(e)?;
        let d = AccumulatorA::new();

        for sub_e in &e.subelements {
            self.walk_i(&sub_e, &subd)?;
        }

        let result = d.summary();
        result
    }
}

pub trait WalkData {}
/*

pub trait Walkable {
    fn xml_document(&self) -> &XmlDocument;

    fn walk<'a, ED: ElementData>(&'a mut self, element_data: &'a mut ED) ->
        Result<dyn WalkData + 'a, Box<dyn Error + 'a>>
    {
        let root = &self.xml_document().root;
        self.walk_i(root, element_data)
    }

    fn walk_i<'a, ED: ElementData>(&self, element: &'a Element,
        element_data: &'a mut ED) ->
        Result<dyn WalkData + 'a, Box<dyn Error + 'a>>
    {
        let mut subelements = Vec::new();
        let mut d = Vec::new();

        for subelement in &element.subelements {
            let mut element_subdata = element_data.clone();
            // Pass the same mutable reference to avoid overlapping borrows
            let subdata = self.walk_i(subelement, &mut element_subdata)?;
            let s = subdata;
            subelements.push(Box::new(s));
            d.push(element_data.clone());
        }

        let e = element.clone();
        element_data.end(&e, subelements)
    }
}
*/

/*
pub struct PrintWalk<'a> {
    pub document: &'a XmlDocument,
//    pub f: &'a mut fmt::Formatter<'a>,
}

impl<'a> Walkable for PrintWalk<'a> {
    fn xml_document(&self) -> &XmlDocument {
        self.document
    }
}

pub struct PrintWalkData<'a, 'b> {
    depth: usize,
//    f: &'a mut fmt::Formatter<'b>,
}

impl<'a, 'b> WalkData for () {}

impl<'a, 'b> ElementData for PrintWalkData<'a, 'b> {
    type StartStatus = Result<Self, Box<dyn Error>>;
    type EndStatus = Result<(), Box<dyn Error>>;

    fn element_start(&mut self, element: &Element) -> Self::StartStatus {
        writeln!(self.f, "{}<{}>", "  ".repeat(self.depth), element.name)
            .map_err(|e| Box::new(e) as Box<dyn Error>);

        Ok(PrintWalkData {
            depth: self.depth + 1,
//            f: self.f,
        })
    }

    fn end(&mut self, element: &Element, _: Vec<()>) -> Self::EndStatus {
        writeln!(self.f, "{}<\{}>", "  ".repeat(self.depth), element.name)
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}
*/
