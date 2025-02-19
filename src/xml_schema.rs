/*
 * Define the data structures used to describe the XML used for parsing.
 */
// FIXME: make sure errors returned are appropriate

//use std::any::Any;
use std::fmt;
// FIXME: implement some more iterators
//use std::iter;
use std::marker::Sync;
use std::ops::Deref;
use std::sync::{Arc, Mutex, MutexGuard};
use std::vec;

use crate::xml_document_error::XmlDocumentError;

pub type SchemaElementType<'a> = Arc<dyn SchemaElement<'a> + Sync>;
pub type SubelementsType<'a> = Vec<SchemaElementType<'a>>;

/*
 * Top-level definition of the schema
 * name:        Name of the structure when printed
 * element:     Root element
 */
#[derive(Clone)]
pub struct XmlSchemaInner<'a> {
    pub name:       &'a str,
    pub element:    Arc<Mutex<SchemaElementType<'a>>>,
}

pub struct XmlSchema<'a> {
    inner:      XmlSchemaInner<'a>,
}

/*
 * Information for each XML Element
 * name:        Element name, which might not be unique
 * attributes:  Attributes for the element
 * subelements: All the elements under this element
 */
struct DirectElementInner<'a> {
    name:           &'a str,
    _attributes:     Vec<SchemaAttribute>,
    subelements:    Arc<Mutex<SubelementsType<'a>>>,
}

pub struct DirectElement<'a> {
    inner:     DirectElementInner<'a>,
}

#[derive(Clone)]
pub struct IndirectElement<'a> {
    direct_element: &'a DirectElement<'a>,
}

#[derive(Clone, Debug)]
pub struct SchemaAttribute {
}

impl<'a> XmlSchemaInner<'a> {
    pub fn new(name: &'a str, element: SchemaElementType<'a>) -> Self {
        Self {
            name:       name,
            element:    Arc::new(Mutex::new(element)),
        }
    }
}

/*
 * XmlSchema
 */
impl<'a> XmlSchema<'a> {
    pub fn new(name: &'a str, element: SchemaElementType<'a>) -> XmlSchema<'a> {
        XmlSchema {
            inner: XmlSchemaInner::new(name, element),
        }
    }

    pub fn name(&self) -> &'a str {
        &self.inner.name
    }

    pub fn element(&self) -> MutexGuard<'_, SchemaElementType<'a>> {
        self.inner.element.lock().unwrap()
    }

    pub fn validate(&self) -> Result<(), XmlDocumentError> {
        println!("Not validating yet");
        Ok(())
    }
}

impl fmt::Display for XmlSchema<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let depth = 0;
        write!(f, "Display for XmlSchema unimplemented, depth {}", depth)?;
        Ok(())
    }
}

/*
 * DirectElementInner
 */
impl<'a> DirectElementInner<'a> {
    pub fn new(name: &'a str, subelements: SubelementsType<'a>) ->
        DirectElementInner<'a> {
        DirectElementInner {
            name:           name,
            _attributes:     vec!(),
            subelements:    Arc::new(Mutex::new(subelements)),
        }
    }
}

/*
 * DirectElement
 */
impl<'a> DirectElement<'a> {
    pub fn new(name: &'a str, subelements: SubelementsType<'a>) ->
        DirectElement<'a> {
        DirectElement {
            inner: DirectElementInner::new(name, subelements),
        }
    }

    pub fn name(&self) -> &'a str {
        &self.inner.name
    }

    pub fn subelements(&self) -> MutexGuard<'_, SubelementsType<'a>> {
        self.inner.subelements.lock().unwrap()
    }
}

impl<'a> SchemaElement<'a> for DirectElement<'a> {
    fn name(&self) -> &'a str {
        self.name()
    }

    // Find an element whose name matches the given one
    fn get(&self, name: &str) -> Option<SchemaElementType<'a>> {
        let subelements = self.subelements();
        subelements.iter()
            .find(move |element| {element.name() == name})
            .cloned()
    }

    fn subelements(&self) -> MutexGuard<'_, SubelementsType<'a>> {
        self.inner.subelements.lock().unwrap()
    }
}

impl fmt::Display for DirectElement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let depth = 0;
        self.display_element(f, depth)
    }
}

/* FIXME: impelement this
struct DirectElementCollection<'a> {
    subelements: Vec<dyn Iterator<Item = &'a Vec<Arc<Mutex<dyn SchemaElement<'a> + Sync>>>>>,
}

// Owned Iterator
struct DirectElementIterator<'a> {
    subelements: Vec<dyn Iterator<Item = &'a Vec<Arc<Mutex<dyn SchemaElement<'a> + Sync>>>>>,
}

impl<'a> Iterator for DirectElementIterator<'a> {
    type Item = Arc<Mutex<dyn SchemaElement<'a> + Sync>>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(tos) = self.subelements.last_mut() {
            while let Some(schema_element) = tos.next() {
                println!("Returning next schema element");
                return Some(schema_element);
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
pub trait SchemaElement<'a>: Sync + Send: Deref + DerefMut {
    fn name(&self) -> String;
}
*/

// Collection of DirectElements
pub struct DirectElementCollection<'a> {
    subelements: Vec<Arc<Mutex<dyn SchemaElement<'a> + Sync>>>,
}

// Owned Iterator
pub struct DirectElementIterator<'a> {
    subelements: vec::IntoIter<Arc<Mutex<dyn SchemaElement<'a> + Sync>>>,
}

// Implement `Iterator` for `DirectElementIterator`
impl<'a> Iterator for DirectElementIterator<'a> {
    type Item = Arc<Mutex<dyn SchemaElement<'a> + Sync>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.subelements.next()
    }
}

// **Owned IntoIterator**
impl<'a> IntoIterator for DirectElementCollection<'a> {
    type Item = Arc<Mutex<dyn SchemaElement<'a> + Sync>>;
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
    type Item = &'a dyn SchemaElement<'a>;
    type IntoIter = iter::Map<
        std::slice::Iter<'a, Arc<Mutex<dyn SchemaElement<'a> + Sync>>>,

        fn(&Arc::<Mutex<dyn SchemaElement<'a> + Sync>>) -> &'a dyn SchemaElement<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.subelements.iter()
    }
}

// **Mutable Borrowed IntoIterator**
impl<'a> IntoIterator for &'a mut DirectElementCollection<'a> {
    type Item = &'a mut dyn SchemaElement<'a>;
    type IntoIter = iter::Map<
        std::slice::IterMut<'a, Arc<Mutex<dyn SchemaElement<'a> + Sync>>>,
        fn(&mut Arc<Mutex<dyn SchemaElement<'a> + Sync>>) -> &'a mut dyn SchemaElement<'a>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.subelements.iter_mut().map(|arc| Arc::get_mut(arc).unwrap())
    }
}
*/


/*
 * IndirectElement
 */
impl IndirectElement<'_> {
    pub fn new<'bbb, 'aaa: 'bbb>(direct_element: &'aaa DirectElement<'aaa>) ->
        Arc<dyn SchemaElement<'aaa> + 'bbb> {
        Arc::new(IndirectElement {
            direct_element: direct_element,
        })
    }
}

impl<'aaa> SchemaElement<'aaa> for IndirectElement<'aaa> {
    fn name(&self) -> &'aaa str {
        self.direct_element.name()
    }

    fn get(&self, name: &str) -> Option<SchemaElementType<'aaa>> {
        self.direct_element.get(name)
    }

    fn subelements<'b>(&self) -> MutexGuard<'_, SubelementsType<'aaa>> {
        self.direct_element.subelements()
    }
}

impl fmt::Display for IndirectElement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let depth = 0;
        self.direct_element.display_element(f, depth)
    }
}

/*
 * SchemaElement
 */

/*
 * trait making DirectElement and IndirectElement work well together
 * name:    Function that returns the name of the element
 * get:     Search for an element by name
 */
pub trait SchemaElement<'aaa>: Sync + Send {
    fn get(&self, name: &str) -> Option<SchemaElementType<'aaa>>;
    fn name(&self) -> &'aaa str;
    fn subelements<'b>(&self) -> MutexGuard<'_, SubelementsType<'aaa>>;

    fn display_element(&self, f: &mut fmt::Formatter, depth: usize) ->
        fmt::Result {
        const INDENT_STR: &str = "   ";
        let indent_string = INDENT_STR.to_string().repeat(depth);

        write!(f, "{}{}", indent_string, self.name())?;
        let subelements = self.subelements();

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

/* FIXME: implement this
struct SchemaElementCollection<'aaa> {
//    schema_elements: Vec<dyn Iterator<Item = &'aaa dyn SchemaElement<'aaa>>>,
//    schema_elements: Vec<impl Iterator<Item = Box<dyn SchemaElement<'aaa>>>>,
    schema_elements: Vec<Box<dyn Iterator<Item = &'aaa dyn SchemaElement<'aaa>> + 'aaa>>,
//    schema_elements = Vec<Box<dyn Iterator<Item = Box<dyn SchemaElement>>>>,
}

// Owned Iterator
struct SchemaElementIterator<'aaa> {
    schema_elements: Vec<Arc<Mutex<dyn Iterator<Item = Arc<&'aaa dyn SchemaElement<'aaa>>>>>>,
}

impl<'aaa> Iterator for SchemaElementIterator<'aaa> {
    type Item = Arc<Mutex<&'aaa dyn SchemaElement<'aaa>>>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(tos) = self.schema_elements.last() {
            match tos.next() {
                Some(el) => {
                    println!("Returning next schema element");
                    return Some(el.lock().unwrap());
                }
                None => {
                    println!("Removing exhausted iterator");
                    self.schema_elements.pop();
                }
            }
        }
        println!("No more elements to iterate");
        None
    }
}

// Owned IntoIterator
impl<'a> IntoIterator for SchemaElementCollection<'a> {
    type Item = Arc<Mutex<&'a dyn SchemaElement<'a>>>;
// FIXME: remove this
//    type IntoIter = Vec<Arc<Mutex<Self::IntoIter<Arc<&dyn SchemaElement>>>>>;
//    type IntoIter = &'a dyn Iterator<Item = Self::Item>;
    type IntoIter = SchemaElementIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.schema_elements.into_iter()
    }
}

// Borrowed IntoIterator
impl<'aaa> IntoIterator for &'aaa SchemaElementIterator<'aaa> {
    type Item = &'aaa dyn SchemaElement<'aaa>;
    type IntoIter = std::iter::Map<
        std::slice::Iter<'aaa, Arc<Mutex<(dyn SchemaElement<'aaa>)>>>,
        fn(&Arc::<dyn SchemaElement<'aaa>>) -> &'aaa (dyn SchemaElement<'aaa> + 'aaa)

    fn into_iter(self) -> Self::IntoIter {
        self.schema_elements.iter().map(|b| b.as_ref())
    }
}

// Mutable
impl<'aaa> IntoIterator for &'aaa mut SchemaElementCollection<'_> {
    type Item = &'aaa mut (dyn SchemaElement<'aaa> + Sync);
    type IntoIter = std::iter::Map<
        std::slice::IterMut<'aaa, Arc<Mutex<dyn SchemaElement<'aaa>>>>,
        fn(&mut Arc<dyn SchemaElement>) -> &'aaa mut (dyn SchemaElement<'aaa> + Sync),
    >;

    fn into_iter(self) -> Self::IntoIter {
        self
           .schema_elements
            .iter_mut()
            .rev()
            .collect::<Vec<_>>()
            .iter_mut()
            .map(|elements| elements)
            .as_mut()
    }
}
*/
