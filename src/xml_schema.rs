/*
 * Define the data structures used to describe the XML used for parsing.
 */
// FIXME: make sure errors returned are appropriate

//use std::any::Any;
use std::fmt;
use std::iter;
use std::marker::Sync;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use std::vec;

use crate::xml_document_error::XmlDocumentError;

pub type DefIdx = usize;

struct S<T> {
    m: Arc<Mutex<T>>,
    // other fields...
}

/* FIXME: remove this
impl<T> Deref for S<T> {
    type Target = Arc<Mutex<T>>;

    fn deref(&self) -> &Self::Target {
        &self.m
    }
}

impl<T> DerefMut for S<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.m
    }
}
*/

/*
 * Top-level definition of the schema
 * name:        Name of the structure when printed
 * element:     Root element
 */
struct XmlSchema_<'a> {
    pub name:       String,
    pub element:    &'a (dyn SchemaElement<'a> + Sync + 'static),
}

pub struct XmlSchema<'a> {
    schema: Arc<Mutex<XmlSchema_<'a>>>,
}

impl<'a> Deref for XmlSchema<'a> {
    type Target = XmlSchema_<'a>;

    fn deref(&self) -> &Self::Target {
        self.schema.lock().unwrap().deref()
    }
}

impl<'a, 'b> DerefMut for XmlSchema<'b> {
    fn deref_mut(&mut self) -> &mut XmlSchema_<'b> {
        &mut self.schema.lock().unwrap()
    }
}

/*
 * Information for each XML Element
 * name:        Element name, which might not be unique
 * attributes:  Attributes for the element
 * subelements: All the elements under this element
 */
pub struct DirectElement<'a> {
    pub name:           String,
    pub attributes:     Vec<SchemaAttribute>,
    pub subelements:    Vec<&'a (dyn SchemaElement<'a> + Sync)>,
}

#[derive(Clone)]
pub struct IndirectElement<'a> {
    direct_element: &'a DirectElement<'a>,
}

#[derive(Clone, Debug)]
pub struct SchemaAttribute {
}

impl XmlSchema<'_> {
    pub fn new(name: &str, element: &dyn for<'aaa> SchemaElement<'aaa>) ->
        Arc<Mutex<Self>> {
        let xml_schema = XmlSchema {
            schema: Arc::new(Mutex::new(XmlSchema_ {
                name:       name.to_string(),
                element:    element,
            }))
        };

        xml_schema
    }

    pub fn validate(&self) -> Result<(), XmlDocumentError> {
        println!("Not validating yet");
        Ok(())
    }
}

/*
 * DirectElement
 */
impl<'a> DirectElement<'a> {
    pub fn new<'aaa>(name: &str,
        subelements: Vec<&'a (dyn for<'bbb> SchemaElement<'bbb> + Sync)> ) ->
        impl SchemaElement<'aaa> {
        Arc::new(DirectElement {
            name:           name.to_string(),
            attributes:     vec!(),
            subelements:    subelements,
        })
    }
}

impl<'aaa> SchemaElement<'_> for DirectElement<'_> {
    fn name(&self) -> String {
        self.name.clone()
    }

    // Find an element whose name matches the given one
    fn get(&self, name: &str) -> Option<&(dyn SchemaElement + Sync)> {
let x: i8 = self.subelements.iter();
        match self
            .subelements
            .iter()
            .find(move |schema_element| {
let i: i8 = schema_element;
                schema_element.lock().unwrap().name() == name
                }) {
            None => None,
            Some(schema_elem) => Some(schema_elem),
        }
    }

    fn subelements(&self) -> &Vec<Arc<Mutex<dyn SchemaElement + Sync>>> {
        &self.subelements
    }
}

impl<'aaa> SchemaElement<'_> for Arc<DirectElement<'aaa>> {
}

impl fmt::Display for DirectElement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let depth = 0;
        self.display_element(f, depth)
    }
}

/*
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

// **Borrowed IntoIterator**
impl<'a> IntoIterator for &'a DirectElementCollection<'a> {
    type Item = &'a dyn SchemaElement<'a>;
    type IntoIter = iter::Map<
        std::slice::Iter<'a, Arc<Mutex<dyn SchemaElement<'a> + Sync>>>,

        fn(&Arc::<Mutex<dyn SchemaElement<'a> + Sync>>) -> &'a dyn SchemaElement<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.subelements.iter().map(|subelement| subelement.as_ref())
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


/*
 * IndirectElement
 */
impl<'aaa> IndirectElement<'aaa> {
    pub fn new<'bbb>(direct_element: &'bbb DirectElement) ->
        Arc<dyn SchemaElement<'aaa> + 'bbb> {
        Arc::new(IndirectElement {
            direct_element: direct_element,
        })
    }
}

impl<'aaa> SchemaElement<'_> for IndirectElement<'_> {
    fn name(&self) -> String {
        self.direct_element.name()
    }

    fn get(&self, name: &str) -> Option<&(dyn SchemaElement + Sync)> {
        self.direct_element.get(name)
    }

    fn subelements(&self) -> &Vec<Arc::<Mutex<dyn SchemaElement + Sync>>> {
        &self.direct_element.subelements()
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
    fn get(&self, name: &str) -> Option<&(dyn SchemaElement + Sync)>;
    fn name(&self) -> String;
    fn subelements(&self) -> &Vec<Arc<Mutex<dyn SchemaElement + Sync>>>;

    fn display_element(&self, f: &mut fmt::Formatter, depth: DefIdx) -> fmt::Result {
        const INDENT_STR: &str = "   ";
        let indent_string = INDENT_STR.to_string().repeat(depth);

        write!(f, "{}{}", indent_string, self.name())?;
        let subelements = self.subelements();

        if subelements.len() == 0 {
            write!(f, " []\n")?;
        } else {
            write!(f, " [\n")?;

            for elem in subelements {
                elem.display_element(f, depth + 1)?;
            }

            write!(f, "{}]\n", indent_string)?;
        }

        Ok(())
    }
}

struct SchemaElementCollection<'aaa> {
    schema_elements: Vec<Arc::<Mutex<dyn Iterator<Item = Arc<&'aaa dyn SchemaElement<'aaa>>>>>>, // Owned heap-allocated trait objects
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
//    type IntoIter = Vec<Arc<Mutex<Self::IntoIter<Arc<&dyn SchemaElement>>>>>;
    type IntoIter = Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.schema_elements.into_iter()
    }
}

/* come back and fix this
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
