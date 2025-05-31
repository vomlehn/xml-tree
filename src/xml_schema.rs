/*
 * Define the data structures used to describe the XML used for parsing.
 */
// FIXME: make sure errors returned are appropriate

//use std::any::Any;
use std::fmt;
// FIXME: implement some more iterators
//use std::iter;
//use std::marker::Sync;
//use std::ops::Deref;
//use std::sync::{Arc, Mutex, MutexGuard};
//use std::vec;

//use crate::xml_document::Element;
use crate::xml_document::XmlDocument;
//use crate::xml_document_error::XmlDocumentError;

pub struct XmlSchema<'a> {
    pub inner: XmlSchemaInner<'a>,
}

// FIXME: remove unsafe
unsafe impl<'a> Sync for XmlSchema<'a> {
}

impl<'a> XmlSchema<'a> {
    pub fn new(schema_name: &'a str, const_name: &'a str, xml_document: XmlDocument<'a>) -> XmlSchema<'a> {
        XmlSchema {
            inner:  XmlSchemaInner {
                schema_name:    schema_name,
                const_name:     const_name,
                xml_document:   xml_document,
            }
        }
    }

    pub fn display(&self) {
        println!("XmlSchema::display");
        println!("{}", self.inner);
    }
}

pub struct XmlSchemaPrint {
}

impl<'a> fmt::Display for XmlSchema<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
/*
        write!(f, "direct element {}\n", self.name())?;
        write!(f, "subelements:\n")?;
        for element in &*self.subelements() {
            write!(f, "{:?}\n", element)?;
        }
        Ok(())
*/
    }
}

/*
 * Top-level definition of the schema
 * schema_name:     Name of the schema
 * const_name:      Const name. This is how the code refers to the schema
 * xml_document:    XML document
 */
//#[derive(Clone)]
pub struct XmlSchemaInner<'a> {
    pub schema_name:    &'a str,
    pub const_name:     &'a str,
    pub xml_document:   XmlDocument<'a>,
}

impl<'a> XmlSchemaInner<'a> {
    pub fn display(&self) {
        println!("XmlSchemaInner::display");
    }
}

impl fmt::Display for XmlSchemaInner<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let front = front_matter(self.const_name, self.schema_name);
        write!(f, "{}", front);
        write!(f, "...{}", self.xml_document)
    }
}

impl fmt::Debug for XmlSchemaInner<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "inner \"{}\" (\"{}\")\n", self.const_name, self.schema_name)?;
        write!(f, "xml_document {:?}\n", self.xml_document)
    }
}

/*
/*
 * Information for each XML Element
 * name:        Element name, which might not be unique
 * attributes:  Attributes for the element
 * subelements: All the elements under this element
 */
struct DirectElementInner<'a> {
    name: &'a str,
    _attributes: Vec<SchemaAttribute>,
    subelements: Vec<Box<dyn Element<'a>>>,
}

#[derive(Clone, Debug)]
pub struct SchemaAttribute {}

impl<'a> XmlSchemaInner<'a> {
    pub fn new(name: &'a str, element: Box<dyn Element<'a>>) -> Self {
        Self {
            name:       name,
            element:    element,
        }
    }
}

/*
 * XmlSchema
 */
impl<'a> XmlSchema<'a> {
    pub fn new(name: &'a str, element: Box<dyn Element<'a>>) -> XmlSchema<'a> {
        XmlSchema {
            inner: XmlSchemaInner::new(name, element),
        }
    }

    pub fn name(&self) -> &'a str {
        &self.inner.name
    }

    pub fn element<'b>(&'b self) -> &'b Box<dyn Element<'a>> {
        &self.inner.element
    }

    pub fn validate(&self) -> Result<(), XmlDocumentError> {
        println!("Not validating yet");
        Ok(())
    }
}

/*
 * DirectElementInner
 */
impl<'a> DirectElementInner<'a> {
    pub fn new(name: &'a str, subelements: Vec<Box<dyn Element<'a>>>) -> DirectElementInner<'a> {
        DirectElementInner {
            name: name,
            _attributes: vec![],
            subelements: subelements,
        }
    }
}

/*
 * DirectElement
 */
pub struct DirectElement<'a> {
    inner: DirectElementInner<'a>,
}

impl<'a> DirectElement<'a> {
    pub fn new(name: &'a str, subelements: Vec<Box<dyn Element<'a>>>) -> DirectElement<'a> {
        DirectElement {
            inner: DirectElementInner::new(name, subelements),
        }
    }

    pub fn name(&self) -> &'a str {
        &self.inner.name
    }

    pub fn subelements(&self) -> &Vec<Box<dyn Element<'a>>> {
        &self.inner.subelements
    }
}

impl<'a> Element<'a> for DirectElement<'a> {
    fn debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }

    fn name<'b>(&self) -> &'b str {
        self.name()
    }

    // Find an element whose name matches the given one
    fn get<'b>(&self, name: &str) -> Option<Box<dyn Element<'b>>> {
/*
let xx: u8 = self.subelements(); // &Vec<Box<dyn Element>>

        letsubelements = self.subelements();
        let subelements = subelements.unwrap();
let x: u8 = subelements;
let x: u8 = subelements.iter();
let x: u8 = subelements.iter() .find(move |element| element.name() == name);
let x: u8 = subelements.iter() .find(move |element| {let y: u8 = element.name(); let z: u8 = name; element.name() == name});
*/

        for e in self.subelements() {
            println!("e {} ", e);
        }
        None
/*
        println!("{}", self.subelements());
            .iter()
            .find(move |element| element.name() == name)
*/
//            .cloned()
    }

    fn subelements<'b>(&'b self) -> &'b Vec<Box<dyn Element<'b>>> {
        &self.inner.subelements
    }
}

/*
impl<'a> fmt::Display for DirectElement<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "inner {}", self.name())
    }
}
*/
//    subelements: Arc<Mutex<Vec<Box<dyn Element<'a>>>>>,

impl<'a> fmt::Debug for DirectElement<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "direct element {}\n", self.name())?;
        write!(f, "subelements:\n")?;
        for element in &*self.subelements() {
            write!(f, "{}\n", element)?;
        }
        Ok(())
    }
}

/*
impl fmt::Display for DirectElement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let depth = 0;
        self.display_element(f, depth)
    }
}
*/

/* FIXME: impelement this
struct DirectElementCollection<'a> {
    subelements: Vec<dyn Iterator<Item = &'a Vec<Arc<Mutex<dyn Element<'a>>>>>>,
}

// Owned Iterator
struct DirectElementIterator<'a> {
    subelements: Vec<dyn Iterator<Item = &'a Vec<Arc<Mutex<dyn Element<'a>>>>>>,
}

impl<'a> Iterator for DirectElementIterator<'a> {
    type Item = Arc<Mutex<dyn Element<'a>>>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(tos) = self.subelements.last_mut() {
            while let Some(element) = tos.next() {
                println!("Returning next schema element");
                return Some(element);
            }
            self.subelements.pop();
        }

        println!("No more elements to iterate");
        None
    }
}

// Owned IntoIterator
impl<'a> IntoIterator for DirectElementCollection<'a> {
    type Item = Arc<Mutex<DirectElement<'a>>>; // Owns the items when iterating
    type IntoIter = std::vec::IntoIter<Arc<Mutex<DirectElement<'a>>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.subelements.into_iter()
    }
}

// Borrowed IntoIterator
impl<'aaa> IntoIterator for &'aaa DirectElementIterator<'_> {
    type Item = &'aaa DirectElement<'aaa>;
    type IntoIter = std::iter::Map<std::slice::Iter<'aaa, Arc<Mutex<DirectElement<'aaa>>>, fn(&Arc<DirectElement>) -> &'aaa DirectElement<'aaa>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.subelements.iter().map(|b| &**b) // Convert `&Arc<dyn DirectElement>` to `&DirectElement`
    }
}

// Mutable
impl<'aaa> IntoIterator for &'aaa mut DirectElementCollection<'_> {
    type Item = &'aaa mut DirectElement<'aaa>;
    type IntoIter = std::iter::Map<
        std::slice::IterMut<'aaa, Arc<Mutex<DirectElement<'aaa>>>>,
        fn(&mut Arc<Mutex<DirectElement>) -> &'aaa mut DirectElement<'aaa>>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.subelements.iter_mut().map(|subelements| subelements.as_mut())
    }
}
*/

/*
use std::sync::{Arc, Mutex};

// Trait Definition
pub trait Element<'a>: Deref + DerefMut {
    fn name(&self) -> String;
}
*/

// Collection of DirectElements
pub struct DirectElementCollection<'a> {
    subelements: Vec<Box<dyn Element<'a>>>,
}

// Owned Iterator
pub struct DirectElementIterator<'a> {
    subelements: vec::IntoIter<Box<dyn Element<'a>>>,
}

// Implement `Iterator` for `DirectElementIterator`
impl<'a> Iterator for DirectElementIterator<'a> {
    type Item = Box<dyn Element<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.subelements.next()
    }
}

// **Owned IntoIterator**
impl<'a> IntoIterator for DirectElementCollection<'a> {
    type Item = Box<dyn Element<'a>>;
    type IntoIter = DirectElementIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DirectElementIterator {
            subelements: self.subelements.into_iter(),
        }
    }
}

/* FIXME: implement this
// **Borrowed IntoIterator**
impl<'a> IntoIterator for &'a DirectElementCollection<'a> {
    type Item = &'a dyn Element<'a>;
    type IntoIter = iter::Map<
        std::slice::Iter<'a, Arc<Mutex<dyn Element<'a>>>>,

        fn(&Arc::<Mutex<dyn Element<'a>>>) -> &'a dyn Element<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.subelements.iter()
    }
}

// **Mutable Borrowed IntoIterator**
impl<'a> IntoIterator for &'a mut DirectElementCollection<'a> {
    type Item = &'a mut dyn Element<'a>;
    type IntoIter = iter::Map<
        std::slice::IterMut<'a, Arc<Mutex<dyn Element<'a>>>>,
        fn(&mut Arc<Mutex<dyn Element<'a>>>) -> &'a mut dyn Element<'a>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.subelements.iter_mut().map(|arc| Arc::get_mut(arc).unwrap())
    }
}
*/

/*
 * IndirectElement
 */
#[derive(Clone)]
pub struct IndirectElement<'a> {
    direct_element: &'a DirectElement<'a>,
}

impl IndirectElement<'_> {
    pub fn new<'bbb, 'aaa: 'bbb>(
        direct_element: &'aaa DirectElement<'aaa>,
    ) -> Box<dyn Element<'aaa> + 'bbb> {
        Box::new(IndirectElement {
            direct_element: direct_element,
        })
    }
}

impl<'aaa> Element<'aaa> for IndirectElement<'aaa> {
    fn debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }

    fn name<'b>(&self) -> &'b str {
        self.direct_element.name()
    }

    fn get<'b>(&self, name: &str) -> Option<Box<dyn Element<'b>>> {
        self.direct_element.get(name)
    }

    fn subelements<'b>(&'b self) -> &'b Vec<Box<dyn Element<'b>>> {
        &self.direct_element.subelements()
    }
}

impl fmt::Debug for IndirectElement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "indirect {:?}", self.direct_element)
    }
}

/*
/*
 * Element
 */

/*
 * trait making DirectElement and IndirectElement work well together
 * name:    Function that returns the name of the element
 * get:     Search for an element by name
 */
pub trait Element<'aaa> {
    fn debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    fn get(&self, name: &str) -> Option<Box<dyn Element<'aaa>>>;
    fn name(&self) -> &'aaa str;
    fn subelements<'b>(&'b self) -> &'b Vec<Box<dyn Element<'aaa>>>;

    fn display_element(&self, f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
        const INDENT_STR: &str = "   ";
        let indent_string = INDENT_STR.to_string().repeat(depth);

        write!(f, "{}\"{}\"", indent_string, self.name())?;
        let subelements = self.subelements();
        println!("subelements.len {}", subelements.len());

        if subelements.len() == 0 {
            write!(f, " []\n")?;
        } else {
            write!(f, " [\n")?;

            for elem in subelements.deref() {
                elem.display_element(f, depth + 1)?;
            }

            write!(f, "{}]\n", indent_string)?;
        }

        Ok(())
    }
}

/* Check all Display impls to ensure status is passed back properly */
// FIXME: why do I need two dyn Elements? Maybe eliminate everything
// with Sync or everything without Sync.
impl fmt::Display for dyn Element<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "schema element {}\n", self.name())?;
        write!(f, "...{} subelements\n", self.subelements().len());
        for element in &*self.subelements() {
            write!(f, "{}", element)?;
        }
        Ok(())
    }
}
/*
impl fmt::Display for dyn Element<'_> + 'static {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "schema element {}\n", self.name())?;
        write!(f, "...{} subelements\n", self.subelements().len());
        for element in &*self.subelements() {
            write!(f, "{}", element)?;
        }
        Ok(())
    }
}
*/

impl fmt::Debug for dyn Element<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.debug(f)
    }
}
*/

/* FIXME: implement this
struct ElementCollection<'aaa> {
//    elements: Vec<dyn Iterator<Item = &'aaa dyn Element<'aaa>>>,
//    elements: Vec<impl Iterator<Item = Box<dyn Element<'aaa>>>>,
    elements: Vec<Box<dyn Iterator<Item = &'aaa dyn Element<'aaa>> + 'aaa>>,
//    elements = Vec<Box<dyn Iterator<Item = Box<dyn Element>>>>,
}

// Owned Iterator
struct ElementIterator<'aaa> {
    elements: Vec<Arc<Mutex<dyn Iterator<Item = Arc<&'aaa dyn Element<'aaa>>>>>>,
}

impl<'aaa> Iterator for ElementIterator<'aaa> {
    type Item = Arc<Mutex<&'aaa dyn Element<'aaa>>>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(tos) = self.elements.last() {
            match tos.next() {
                Some(el) => {
                    println!("Returning next schema element");
                    return Some(el);
                }
                None => {
                    println!("Removing exhausted iterator");
                    self.elements.pop();
                }
            }
        }
        println!("No more elements to iterate");
        None
    }
}

// Owned IntoIterator
impl<'a> IntoIterator for ElementCollection<'a> {
    type Item = Arc<Mutex<&'a dyn Element<'a>>>;
// FIXME: remove this
//    type IntoIter = Vec<Arc<Mutex<Self::IntoIter<Arc<&dyn Element>>>>>;
//    type IntoIter = &'a dyn Iterator<Item = Self::Item>;
    type IntoIter = ElementIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

// Borrowed IntoIterator
impl<'aaa> IntoIterator for &'aaa ElementIterator<'aaa> {
    type Item = &'aaa dyn Element<'aaa>;
    type IntoIter = std::iter::Map<
        std::slice::Iter<'aaa, Arc<Mutex<(dyn Element<'aaa>)>>>,
        fn(&Arc::<dyn Element<'aaa>>) -> &'aaa (dyn Element<'aaa> + 'aaa)

    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter().map(|b| b.as_ref())
    }
}

// Mutable
impl<'aaa> IntoIterator for &'aaa mut ElementCollection<'_> {
    type Item = &'aaa mut (dyn Element<'aaa>);
    type IntoIter = std::iter::Map<
        std::slice::IterMut<'aaa, Arc<Mutex<dyn Element<'aaa>>>>,
        fn(&mut Arc<dyn Element>) -> &'aaa mut (dyn Element<'aaa>),
    >;

    fn into_iter(self) -> Self::IntoIter {
        self
           .elements
            .iter_mut()
            .rev()
            .collect::<Vec<_>>()
            .iter_mut()
            .map(|elements| elements)
            .as_mut()
    }
}
*/


/**
 * Data for the Element being worked on by walk_down().
 */
pub struct ElemData {
    depth:  usize,
}

impl ElemData {
    fn new(depth: usize) -> ElemData {
        ElemData {
            depth:  depth,
        }
    }
}

/**
 * Data stored at the root level of the Walkable and a reference to which is
 * returned by the Walkable base_level_cell() function.
 */
pub struct BaseLevel<'a> {
    name:   &'a str,
}

/**
 */
pub struct WalkSchema<'a> {
    f:      &'a mut fmt::Formatter<'a>,
    name:   &'a str,
    schema: &'a XmlSchema<'a>,
}

/*
impl<'a> WalkSchema<'a> {
    fn new(f: &mut fmt::Formatter, name: &'a str, schema: &XmlSchema) -> WalkSchema<'a> {
        WalkSchema {
            f:      f,
            name:   name,
            schema: schema.
        }
    }

//    fn base_level_cell(&'a self) -> &'a RefCell<BL>;

    fn walk<'b>(&'b self) -> fmt::Result {
    {
        write!(self.f, front_matter())?;
        self.walk_down(&schema.inner.element)
    }
/*
 * This is the parsed scheme for XSD files.
 */
use lazy_static::lazy_static;
use std::sync::Arc;

use crate::xml_schema::{DirectElement, XmlSchema};

lazy_static! {
    pub static ref XSD_SCHEMA: XmlSchema<'static> = XmlSchema::new(
        "XsdSchema",
        Arc::new(DirectElement::new(
            "schema",
            vec!(
                Arc::new(DirectElement::new("import", vec!())),
                Arc::new(DirectElement::new(
                    "annotation",
                    vec!(Arc::new(DirectElement::new("documentation", vec!())),)
                )),
                Arc::new(DirectElement::new(
                    "element",
                    vec!(
                        Arc::new(DirectElement::new(
                            "annotation",
                            vec!(Arc::new(DirectElement::new("documentation", vec!())),)
                        )),
                        Arc::new(DirectElement::new(
                            "key",
                            vec!(
                                Arc::new(DirectElement::new(
                                    "annotation",
                                    vec!(Arc::new(DirectElement::new("documentation", vec!())),)
                                )),
                                Arc::new(DirectElement::new("selector", vec!())),
                                Arc::new(DirectElement::new("field", vec!())),
                            )
                        )),
                    )
                )),
                Arc::new(DirectElement::new(
                    "complexType",
                    vec!(
                        Arc::new(DirectElement::new(
                            "annotation",
                            vec!(
                                Arc::new(DirectElement::new("documentation", vec!())),
                                Arc::new(DirectElement::new("appinfo", vec!())),
                            )
                        )),
                        Arc::new(DirectElement::new(
                            "attribute",
                            vec!(
                                Arc::new(DirectElement::new(
                                    "annotation",
                                    vec!(
                                        Arc::new(DirectElement::new("documentation", vec!())),
                                        Arc::new(DirectElement::new("appinfo", vec!())),
                                    )
                                )),
                                Arc::new(DirectElement::new(
                                    "simpleType",
                                    vec!(Arc::new(DirectElement::new(
                                        "restriction",
                                        vec!(Arc::new(DirectElement::new(
                                            "enumeration",
                                            vec!(Arc::new(DirectElement::new(
                                                "annotation",
                                                vec!(Arc::new(DirectElement::new(
                                                    "documentation",
                                                    vec!()
                                                )),)
                                            )),)
                                        )),)
                                    )),)
                                )),
                            )
                        )),
                        Arc::new(DirectElement::new(
                            "choice",
                            vec!(
                                Arc::new(DirectElement::new(
                                    "annotation",
                                    vec!(Arc::new(DirectElement::new("documentation", vec!())),)
                                )),
                                Arc::new(DirectElement::new(
                                    "element",
                                    vec!(
                                        Arc::new(DirectElement::new(
                                            "annotation",
                                            vec!(Arc::new(DirectElement::new(
                                                "documentation",
                                                vec!()
                                            )),)
                                        )),
                                        Arc::new(DirectElement::new(
                                            "key",
                                            vec!(
                                                Arc::new(DirectElement::new(
                                                    "annotation",
                                                    vec!(Arc::new(DirectElement::new(
                                                        "documentation",
                                                        vec!()
                                                    )),)
                                                )),
                                                Arc::new(DirectElement::new("selector", vec!())),
                                                Arc::new(DirectElement::new("field", vec!())),
                                            )
                                        )),
                                    )
                                )),
                            )
                        )),
                        Arc::new(DirectElement::new(
                            "sequence",
                            vec!(
                                Arc::new(DirectElement::new(
                                    "annotation",
                                    vec!(
                                        Arc::new(DirectElement::new("documentation", vec!())),
                                        Arc::new(DirectElement::new("appinfo", vec!())),
                                    )
                                )),
                                Arc::new(DirectElement::new(
                                    "element",
                                    vec!(Arc::new(DirectElement::new(
                                        "annotation",
                                        vec!(
                                            Arc::new(DirectElement::new("documentation", vec!())),
                                            Arc::new(DirectElement::new("appinfo", vec!())),
                                        )
                                    )),)
                                )),
                                Arc::new(DirectElement::new(
                                    "choice",
                                    vec!(
                                        Arc::new(DirectElement::new(
                                            "annotation",
                                            vec!(Arc::new(DirectElement::new(
                                                "documentation",
                                                vec!()
                                            )),)
                                        )),
                                        Arc::new(DirectElement::new(
                                            "element",
                                            vec!(Arc::new(DirectElement::new(
                                                "annotation",
                                                vec!(
                                                    Arc::new(DirectElement::new(
                                                        "documentation",
                                                        vec!()
                                                    )),
                                                    Arc::new(DirectElement::new("appinfo", vec!())),
                                                )
                                            )),)
                                        )),
                                    )
                                )),
                            )
                        )),
                        Arc::new(DirectElement::new(
                            "simpleContent",
                            vec!(Arc::new(DirectElement::new(
                                "extension",
                                vec!(Arc::new(DirectElement::new("attribute", vec!())),)
                            )),)
                        )),
                        Arc::new(DirectElement::new(
                            "sequence",
                            vec!(
                                Arc::new(DirectElement::new(
                                    "element",
                                    vec!(Arc::new(DirectElement::new(
                                        "annotation",
                                        vec!(
                                            Arc::new(DirectElement::new("documentation", vec!())),
                                            Arc::new(DirectElement::new("appinfo", vec!())),
                                        )
                                    )),)
                                )),
                                Arc::new(DirectElement::new(
                                    "choice",
                                    vec!(
                                        Arc::new(DirectElement::new(
                                            "annotation",
                                            vec!(Arc::new(DirectElement::new(
                                                "documentation",
                                                vec!()
                                            )),)
                                        )),
                                        Arc::new(DirectElement::new(
                                            "element",
                                            vec!(Arc::new(DirectElement::new(
                                                "annotation",
                                                vec!(
                                                    Arc::new(DirectElement::new(
                                                        "documentation",
                                                        vec!()
                                                    )),
                                                    Arc::new(DirectElement::new("appinfo", vec!())),
                                                )
                                            )),)
                                        )),
                                    )
                                )),
                            )
                        )),
                        Arc::new(DirectElement::new(
                            "complexContent",
                            vec!(Arc::new(DirectElement::new(
                                "extension",
                                vec!(
                                    Arc::new(DirectElement::new(
                                        "attribute",
                                        vec!(Arc::new(DirectElement::new(
                                            "annotation",
                                            vec!(
                                                Arc::new(DirectElement::new(
                                                    "documentation",
                                                    vec!()
                                                )),
                                                Arc::new(DirectElement::new("appinfo", vec!())),
                                            )
                                        )),)
                                    )),
                                    Arc::new(DirectElement::new(
                                        "choice",
                                        vec!(
                                            Arc::new(DirectElement::new(
                                                "annotation",
                                                vec!(Arc::new(DirectElement::new(
                                                    "documentation",
                                                    vec!()
                                                )),)
                                            )),
                                            Arc::new(DirectElement::new(
                                                "element",
                                                vec!(Arc::new(DirectElement::new(
                                                    "annotation",
                                                    vec!(Arc::new(DirectElement::new(
                                                        "documentation",
                                                        vec!()
                                                    )),)
                                                )),)
                                            )),
                                        )
                                    )),
                                    Arc::new(DirectElement::new(
                                        "sequence",
                                        vec!(
                                            Arc::new(DirectElement::new(
                                                "annotation",
                                                vec!(
                                                    Arc::new(DirectElement::new(
                                                        "documentation",
                                                        vec!()
                                                    )),
                                                    Box::new(DirectElement::new("appinfo", vec!())),
                                                )
                                            )),
                                            Box::new(DirectElement::new(
                                                "choice",
                                                vec!(
                                                    Box::new(DirectElement::new(
                                                        "annotation",
                                                        vec!(Box::new(DirectElement::new(
                                                            "documentation",
                                                            vec!()
                                                        )),)
                                                    )),
                                                    Box::new(DirectElement::new(
                                                        "choice",
                                                        vec!(
                                                            Box::new(DirectElement::new(
                                                                "annotation",
                                                                vec!(Box::new(DirectElement::new(
                                                                    "documentation",
                                                                    vec!()
                                                                )),)
                                                            )),
                                                            Box::new(DirectElement::new(
                                                                "element",
                                                                vec!(Box::new(DirectElement::new(
                                                                    "annotation",
                                                                    vec!(Box::new(
                                                                        DirectElement::new(
                                                                            "documentation",
                                                                            vec!()
                                                                        )
                                                                    ),)
                                                                )),)
                                                            )),
                                                        )
                                                    )),
                                                    Box::new(DirectElement::new(
                                                        "element",
                                                        vec!(Box::new(DirectElement::new(
                                                            "annotation",
                                                            vec!(Box::new(DirectElement::new(
                                                                "documentation",
                                                                vec!()
                                                            )),)
                                                        )),)
                                                    )),
                                                )
                                            )),
                                            Box::new(DirectElement::new(
                                                "element",
                                                vec!(
                                                    Box::new(DirectElement::new(
                                                        "annotation",
                                                        vec!(
                                                            Box::new(DirectElement::new(
                                                                "documentation",
                                                                vec!()
                                                            )),
                                                            Box::new(DirectElement::new(
                                                                "appinfo",
                                                                vec!()
                                                            )),
                                                        )
                                                    )),
                                                    Box::new(DirectElement::new(
                                                        "complexType",
                                                        vec!(Box::new(DirectElement::new(
                                                            "complexContent",
                                                            vec!()
                                                        )),)
                                                    )),
                                                )
                                            )),
                                        )
                                    )),
                                )
                            )),)
                        )),
                    )
                )),
                Box::new(DirectElement::new(
                    "simpleType",
                    vec!(
                        Box::new(DirectElement::new(
                            "annotation",
                            vec!(Box::new(DirectElement::new("documentation", vec!())),)
                        )),
                        Box::new(DirectElement::new(
                            "enumeration",
                            vec!(Box::new(DirectElement::new(
                                "annotation",
                                vec!(Box::new(DirectElement::new("documentation", vec!())),)
                            )),)
                        )),
                        Box::new(DirectElement::new(
                            "restriction",
                            vec!(
                                Box::new(DirectElement::new("maxInclusive", vec!())),
                                Box::new(DirectElement::new("minInclusive", vec!())),
                                Box::new(DirectElement::new("pattern", vec!())),
                                Box::new(DirectElement::new(
                                    "enumeration",
                                    vec!(Box::new(DirectElement::new(
                                        "annotation",
                                        vec!(
                                            Box::new(DirectElement::new("documentation", vec!())),
                                            Box::new(DirectElement::new("appinfo", vec!())),
                                        )
                                    )),)
                                )),
                            )
                        )),
                        Box::new(DirectElement::new("union", vec!())),
                    )
                )),
            )
        )),
    );
}



    fn walk_down<'b>(&'b self, element: &'a Element, ed: &ED) -> fmt::Result {
    where
        'b: 'a,
    {
        let bl_ref = self.base_level_cell();
        let mut acc = AC::new(bl_ref, element, ed);

        // Process subelements and collect WalkData results
        let mut wd_vec = Vec::<WD>::new();
        for elem in &element.subelements {
            let next_ed = ed.next_level(elem);
            let wd = self.walk_down(elem, &next_ed)?;
            wd_vec.push(wd);
        }

        // Accumulate results
        for wd in &wd_vec {
            acc.add(wd)?;
        }
        acc.summary()
    }
}
*/
*/

fn front_matter(const_name: &str, schema_name: &str) -> String {
    let front_matter: Vec::<String> = vec!(
        "// FIXME: insert banner".to_string(),
        "use lazy_static::lazy_static;".to_string(), 
        "use std::sync::Arc;".to_string(), 
        "".to_string(), 
        "use crate::xml_schema::{{DirectElement, XmlSchema}};".to_string(), 
        "".to_string(), 
        "lazy_static! {{".to_string(), 
        format!("    pub static ref {const_name}: XmlSchema<'static> = XmlSchema::new("), 
        format!("        \"{schema_name}\","), 
        "}}".to_string()
    );
    front_matter.join("\n")
}
