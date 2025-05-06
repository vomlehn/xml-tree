/*
/*
 * Recursive print
 */

use std::fmt;

use crate::{Accumulator, ElemData, WalkData, Walkable};
use crate::xml_document::{Element, XmlDocument};

const INDENT: &str = "    ";

pub struct WalkAndPrint<'a> {
    xml_document:   &'a XmlDocument,
    f:              &'a mut fmt::Formatter<'a>,
}

impl<'a> WalkAndPrint<'a> {
    pub fn new<'b>(xml_document: &'a XmlDocument, f: &'b mut fmt::Formatter<'b>) -> WalkAndPrint<'b> {
        WalkAndPrint {
            xml_document:   xml_document,
        }
    }
}

impl<'a> Walkable<'a, PrintElemData, (), PrintAccumulator, fmt::Result> for WalkAndPrint<'a> {
    fn xml_document(&self) -> &XmlDocument {
        self.xml_document
    }
}

// ----------------- Data Types ----------------

pub struct PrintWalkData {}

//impl () for PrintWalkData {}

impl WalkData for PrintWalkData {}

pub struct PrintElemData {
    pub depth:  usize,
}

impl PrintElemData {
    pub fn new(depth: usize) -> PrintElemData {
        PrintElemData {
            depth:  depth,
        }
    }

    fn indent(&self) -> String {
        INDENT.repeat(self.depth)
    }
}

impl ElemData<'_, PrintElemData, PrintWalkData, PrintAccumulator, fmt::Result> for PrintElemData {
    fn start(&mut self, w: new(&WalkAndPrint), element: &Element) -> fmt::Result {
        write!(w.f, "{}<{}>", self.indent(), element.name);
        let ed = PrintElemData::new(self.depth + 1);
        fmt::Result::Ok(ed)
    }
}

#[derive(Debug)]
pub struct PrintAccumulator {
    result: String,
}

impl<'a> Accumulator<'a, PrintElemData, PrintWalkData, PrintAccumulator, fmt::Result> for PrintAccumulator {
    fn new(e: &Element, ed: &PrintElemData) -> Self {
        let result = format!("{}<{}>", ed.indent(), e.name.local_name);
        PrintAccumulator { result }
    }

    fn add(&mut self, wd: PrintWalkData) -> fmt::Result {
        self.result += &format!("\n{}", wd.data);
        fmt::Result::Ok(())
    }

    fn summary(&self) -> fmt::Result {
        fmt::Result::Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;
    use xml::attribute::OwnedAttribute;
    use xml::common::XmlVersion;
    use xml::name::OwnedName;
    use xml::namespace::Namespace;
    use xml::reader::XmlEvent;

    use crate::{Accumulator, Walkable};
    use crate::xml_document::{Element, XmlDocument};
    use crate::xml_document_factory::{DocumentInfo, ElementInfo};
    use super::{PrintElemData, WalkAndPrint};

    #[test]
    fn test_walk_tree_print() {
        println!("\nStart test_walk_tree_print");
        let doc = create_test_doc(); // build a sample XmlDocument
        let walker = WalkAndPrint::new(&doc);
        let result = walker.walk(&PrintElemData { depth: 0 });

        let res = match result {
            fmt::Result::Ok(data) => {
                let res = format!("{}", data.data);
                println!("Output:\n{}", res);
                res
            }
            io::Result::Err(e) => {
                let res = format!("{}", e);
                eprintln!("Error: {}", res);
                res
            }
        };
        assert_eq!(res, concat!("<n1>\n",
            "    <n2>\n",
            "    <n3>\n",
            "        <n4>"));
    }

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
*/
