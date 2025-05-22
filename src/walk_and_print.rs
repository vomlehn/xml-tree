use crate::walkable::{Accumulator, BaseLevel, ElemData, Walkable};
use crate::xml_document::{Element, XmlDocument};
use std::fmt;

const INDENT: &str = "    ";

fn indent(n: usize) -> String {
    INDENT.repeat(n)
}

/**
 * Basic structure for recursive printing
 */
pub struct WalkAndPrint<'a, 'fmt> {
    xml_doc:    &'a XmlDocument,
    base:       PrintBaseLevel<'a, 'fmt>,
}

impl<'a, 'fmt> WalkAndPrint<'a, 'fmt> {
    pub fn new(xml_doc: &'a XmlDocument, base: PrintBaseLevel<'a, 'fmt>) -> WalkAndPrint<'a, 'fmt> {
        WalkAndPrint{
            xml_doc:    xml_doc,
            base:       base,}
    }
}

impl<'a, 'fmt> Walkable<'a, PrintAccumulator, PrintBaseLevel<'a, 'fmt>, PrintElemData, PrintWalkData, PrintWalkResult> for WalkAndPrint<'_, 'fmt> {
    fn xml_document(&self) -> &XmlDocument {
        self.xml_doc
    }
    fn base_level(&'a self) -> &'a PrintBaseLevel<'a, 'fmt> {
        &self.base
    }
}

/**
 * Since we're printing, our return type is the same as the type
 * returned from std::fmt::Display::fmt(). This means we have to
 * returns one of the fmt::Error types if we encounter another
 * error, or simply panic!.
 */
pub type PrintWalkResult = fmt::Result;

/**
 * We don't return any data, but do print the element name each
 * time we enter WalkAndPrint::walk_down().
 */
pub struct PrintAccumulator {
}

impl<'a, 'fmt> Accumulator<'a, PrintBaseLevel<'a, 'fmt>, PrintElemData, PrintWalkData, PrintWalkResult>
for PrintAccumulator {
    fn new(_bl: &PrintBaseLevel, e: &'a Element, ed: &PrintElemData) -> Self {
        println!("{}{}", indent(ed.depth), e.name);
        PrintAccumulator {
        }
    }

    fn add(&mut self, _wd: &PrintWalkData) -> PrintWalkResult {
        Ok(())
    }

    fn summary(&self) -> PrintWalkResult {
        Ok(())
    }
}

/**
 * The BaseLevel data consists of just an fmt::Formatter passed to
 * fmt::Display::fmt().
 */
pub struct PrintBaseLevel<'a, 'fmt> {
    f: &'a mut fmt::Formatter<'fmt>,
}

impl<'a, 'fmt> PrintBaseLevel<'a, 'fmt> {
    pub fn new(f: &'a mut fmt::Formatter<'fmt>) -> Self {
        PrintBaseLevel {
            f:  f,
        }
    }
}

impl<'a, 'fmt> BaseLevel for PrintBaseLevel<'a, 'fmt> {}

/**
 * Keep track of the depth so we can do proper indentation
 */
pub struct PrintElemData {
    depth:  usize,
}

impl PrintElemData {
    pub fn new(depth: usize) -> PrintElemData {
        PrintElemData {
            depth:  depth,
        }
    }
}

impl ElemData<PrintBaseLevel<'_, '_>, PrintElemData> for PrintElemData {
    fn next_level(&self, _element: &Element) -> PrintElemData {
        PrintElemData::new(self.depth + 1)
    }
}

/**
 * All we do is print, so there is no data to return. This is
 * consistent with the OK enum from fmt::Error
 */
type PrintWalkData = ();

#[cfg(test)]
mod print_tests {
    use xml::attribute::OwnedAttribute;
    use xml::common::XmlVersion;
    use xml::name::OwnedName;
    use xml::namespace::Namespace;
    use xml::reader::XmlEvent;

    use std::collections::BTreeMap;
    use std::fmt;

    use crate::walkable::{Accumulator, BaseLevel, ElemData, Walkable};
    use crate::xml_document::{Element, XmlDocument};
    use crate::xml_document_factory::{DocumentInfo, ElementInfo};

    pub type PrintWalkResult = fmt::Result;

    pub struct PrintBaseLevel<'a, 'fmt> {
        f: &'a mut fmt::Formatter<'fmt>,
    }

    impl<'a, 'fmt> PrintBaseLevel<'a, 'fmt> {
        pub fn new(f: &'a mut fmt::Formatter<'fmt>) -> Self {
            PrintBaseLevel {
                f:  f,
            }
        }
    }

    impl<'a, 'fmt> BaseLevel for PrintBaseLevel<'a, 'fmt> {}

    pub struct PrintElemData {
        depth:  usize,
    }

    impl PrintElemData {
        pub fn new(depth: usize) -> PrintElemData {
            PrintElemData {
                depth:  depth,
            }
        }
    }

    impl ElemData<PrintBaseLevel<'_, '_>, PrintElemData> for PrintElemData {
        fn next_level(&self, element: &Element) -> PrintElemData {
            PrintElemData::new(self.depth + 1)
        }
    }

    type PrintWalkData = ();

    pub struct PrintAccumulator {}

    impl<'a, 'fmt> Accumulator<'a, PrintBaseLevel<'a, 'fmt>, PrintElemData, PrintWalkData, PrintWalkResult>
    for PrintAccumulator {
        fn new(bl: &PrintBaseLevel, e: &'a Element, ed: &PrintElemData) -> Self {
            println!("{}{}", super::indent(ed.depth), e.name);
            PrintAccumulator {
            }
        }
        fn add(&mut self, wd: &PrintWalkData) -> PrintWalkResult {
            Ok(())
        }
        fn summary(&self) -> PrintWalkResult {
            Ok(())
        }
    }

    pub struct PrintWalkable<'a, 'fmt> {
        xml_doc:    &'a XmlDocument,
        base:       PrintBaseLevel<'a, 'fmt>,
    }

    impl<'a, 'fmt> PrintWalkable<'a, 'fmt> {
        pub fn new(xml_doc: &'a XmlDocument, base: PrintBaseLevel<'a, 'fmt>) -> PrintWalkable<'a, 'fmt> {
            PrintWalkable{
                xml_doc:    xml_doc,
                base:       base,
            }
        }
    }

    impl<'a, 'fmt> Walkable<'a, PrintAccumulator, PrintBaseLevel<'a, 'fmt>, PrintElemData, PrintWalkData, PrintWalkResult> for PrintWalkable<'_, 'fmt> {
        fn xml_document(&self) -> &XmlDocument {
            self.xml_doc
        }
        fn base_level(&'a self) -> &'a PrintBaseLevel<'a, 'fmt> {
            &self.base
        }
    }

    struct PrintObj<'a> {
        xml_doc:    &'a XmlDocument,
    }

    impl<'a> PrintObj<'a> {
        pub fn new(xml_doc: &'a XmlDocument) -> PrintObj<'a> {
            PrintObj {
                xml_doc:    xml_doc,
            }
        }
    }

    impl<'a> fmt::Display for PrintObj<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let bl2 = PrintBaseLevel::new(f);
            let ed2 = PrintElemData::new(0);
            let w2 = PrintWalkable::new(&self.xml_doc, bl2);
            w2.walk(&ed2)
        }
    }


    #[test]
    fn test_fmt_result() {
        println!();
        println!("Try with a fmt::Result");
        println!("----------------------");
        let xml_document = create_test_doc();
        let po = PrintObj::new(&xml_document);
        println!("Display PrintObj:");
        println!("{}", po);
    }

    /**
     * Manually create an XmlDocument.
     */
     // FIXME: This should be moved to a common area
    fn create_test_doc() -> XmlDocument {
        let ns: Namespace = Namespace(BTreeMap::<String, String>::new());

        let ei: ElementInfo = ElementInfo {
            lineno: 1,
            attributes: Vec::<OwnedAttribute>::new(),
            namespace: ns,
        };

        XmlDocument {
            root:           branch("n1", &ei, vec![
                                leaf("n2", &ei),
                                branch("n3", &ei, vec![
                                    leaf("n4", &ei)])
                            ]),
            document_info:  DocumentInfo {
                                version: XmlVersion::Version10,
                                encoding: "xxx".to_string(),
                                standalone: None,
                            },
        }
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
            element_info: (*ei).clone(),
            subelements,
            before_element: Vec::<XmlEvent>::new(),
            content: Vec::<XmlEvent>::new(),
            after_element: Vec::<XmlEvent>::new(),
        }
    }
}
