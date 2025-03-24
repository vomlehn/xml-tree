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

        let n: OwnedName = OwnedName {
            local_name: "n1".to_string(),
            namespace:  None,
            prefix:     None
        };

        let e: Element = Element {
            name:           n,
            depth:          0,
            element_info:   ei,
            subelements:    Vec::<Element>::new(),
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
            root:   e,
            document_info:  di,
        };

        d
    }

    #[test]
    fn test1() {
//    f: &'a mut fmt::Formatter<'a>,
println!("calling test1");
        let d = initf();
        frun(&d);
    }

    pub fn frun(x: &XmlDocument) {
        let a = A { xml_document:x };
        let aa = a.walk();
        println!("aa: {:?}", aa);

        let b = B { xml_document:x };
        let bb = b.walk();
        println!("bb: {:?}", bb);
    }

    struct A<'a> {
        xml_document:   &'a XmlDocument,
    }

    impl<'a> Walkable for A<'a> {
        type ED = A<'a>;
        type ES = Result<Self::ED, Box<dyn Error>>;
        type WD = u8;
        type WS = Result<Self::WD, Box<dyn Error>>;

        fn xml_document(&self) -> &XmlDocument {
            self.xml_document
        }

        fn walk(&self) -> Self::WS {
            Ok(0)
        }

        fn walk_i(&self) -> Self::WS {
            Ok(1)
        }
    }

    impl ElementData for A<'_> {
        type WD = u8;
        type WS = Result<Self::WD, Box<dyn Error>>;

        fn xml_document(&self) -> &XmlDocument { self.xml_document }
        fn end(&mut self) -> Self::WS {
            Ok(37)
        }
    }

    struct B<'a> {
        xml_document:   &'a XmlDocument,
    }

    impl<'a> Walkable for B<'a> {
        type ED = B<'a>;
        type ES = Result<Self::ED, Box<dyn Error>>;
        type WD = ();
        type WS = Result<Self::WD, Box<dyn Error>>;

        fn xml_document(&self) -> &XmlDocument {
            self.xml_document
        }

        fn walk(&self) -> Self::WS {
            Ok(())
        }

        fn walk_i(&self) -> Self::WS {
            Ok(())
        }

    }

    impl ElementData for B<'_> {
        type WD = ();
        type WS = fmt::Result;

        fn xml_document(&self) -> &XmlDocument { self.xml_document }
        fn end(&mut self) -> Self::WS {
            Ok(())
        }
    }
}

/*
 * This has to be a trait so the functions can be defined by users
 */
pub trait ElementData:  {
    type WD;
    type WS: Try;

    fn xml_document(&self) -> &XmlDocument;
    fn end(&mut self) -> Self::WS;
/*
    fn element_start(&mut self, element: &Element) ->
        Result<Box<dyn E>, Box<dyn Error>>;
    fn end(&mut self, element: &Element,
        subelements: Vec<Box<dyn W>>) ->
        Result<Box<dyn W>, Box<dyn Error>>;
*/
}

pub trait Walkable {
    type ED;
    type ES;
    type WD;
    type WS: Try;

    fn xml_document(&self) -> &XmlDocument;

    fn walk(&self) -> Self::WS;

    fn walk_i(&self) -> Self::WS;
}

/*
pub trait WalkData {}

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
    pub f: &'a mut fmt::Formatter<'a>,
}

impl<'a> Walkable for PrintWalk<'a> {
    fn xml_document(&self) -> &XmlDocument {
        self.document
    }
}

pub struct PrintWalkData<'a, 'b> {
    depth: usize,
    f: &'a mut fmt::Formatter<'b>,
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
            f: self.f,
        })
    }

    fn end(&mut self, element: &Element, _: Vec<()>) -> Self::EndStatus {
        writeln!(self.f, "{}<\{}>", "  ".repeat(self.depth), element.name)
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}
*/
