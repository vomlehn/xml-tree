/*
 * Define the data structures used to describe the XML used for parsing.
 */
// FIXME: make sure errors returned are appropriate

use std::any::Any;
use std::fmt;

use crate::xml_document_error::XmlDocumentError;

pub type DefIdx = usize;

/*
 * Top-level definition of the schema
 * name:        Name of the structure when printed
 * element:     Root element
 */
pub struct XmlSchema<'a> {
    pub name:       String,
    pub element:    Box<&'a dyn SchemaElement<'a>>,
}

/*
 * Information for each XML Element
 * name:                        Element name, which might not be unique
 * key:                         Key for this ElementDescription, which must be
 *                              unique
 * allowable_subelement_keys:   Keys indicating the subelements of this
 *                              DirectElement.
 * allowable_subelement_vec:   Array with the indices into schema_elements
 *                              for each item in allowable_element_keys
 */
pub struct DirectElement<'a> {
    pub name:           String,
    pub attributes:     Vec<SchemaAttribute>,
    pub subelements:    Vec<Box<dyn SchemaElement<'a>>>,
}

#[derive(Clone)]
pub struct IndirectElement<'a> {
    direct_element: &'a DirectElement<'a>,
}

#[derive(Clone, Debug)]
pub struct SchemaAttribute {
}

impl XmlSchema<'_> {
    pub fn new<'a>(name: &'a str, element: Box<&'a dyn SchemaElement<'a>>) -> XmlSchema<'a>  {
        let xml_schema = XmlSchema {
            name:       name.to_string(),
            element:    element,
        };

        xml_schema
    }

    pub fn validate(&self) -> Result<(), XmlDocumentError> {
        println!("Not validating yet");
        Ok(())
    }
}

impl<'a> DirectElement<'a> {
    pub fn new(name: &str, subelements: Vec<Box<dyn SchemaElement<'a>>>) ->
        DirectElement<'a> {
        DirectElement {
            name:           name.to_string(),
            attributes:     vec!(),
            subelements:    subelements,
        }
    }
}
        
impl fmt::Display for XmlSchema<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
write!(f, "{}\n", "Display for XmlSchema")?;
        let depth = 1;
        write!(f, "Schema: {}\n", self.name);
        self.element.display_element(f, depth)
    }
}

impl SchemaElement<'_> for DirectElement<'_> {
    fn name(&self) -> String {
        self.name.clone()
    }

    // Find an element whose name matches the given one
    fn get<'b>(&self, name: &str) -> Option<&Box<dyn SchemaElement>> {
        match self
            .subelements
            .iter()
            .find(move |&schema_element| schema_element.name() == name) {
            None => None,
            Some(schema_elem) => Some(schema_elem),
        }
    }

    fn subelements(&self) -> &Vec<Box<&dyn SchemaElement>> {
        &self.subelements
    }
}

impl fmt::Display for DirectElement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let depth = 0;
        self.display_element(f, depth)
    }
}

/*
 * IndirectElement
 */
impl<'a> IndirectElement<'a> {
    pub fn new<'b>(direct_element: &'b DirectElement<'b>) -> IndirectElement<'b> {
        IndirectElement {
            direct_element: direct_element,
        }
    }
}

impl SchemaElement<'_> for IndirectElement<'_> {
    fn name(&self) -> String {
        self.direct_element.name.clone()
    }

    fn get<'b>(&self, name: &str) -> Option<&Box<dyn SchemaElement>> {
        self.direct_element.get(name)
    }

    fn subelements(&self) -> &Vec<Box<&dyn SchemaElement>> {
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
 * DirectElement
 */
/*
struct DirectElementCollection {
    direct_elements: Vec<Box<DirectElement>>, // Owned heap-allocated trait objects
}

// Owned Iterator
struct DirectElementIterator {
    direct_elements: Vec<Box<DirectElement>>, // A Vec of trait objects
    index: usize,
}

impl Iterator for DirectElement {
    type Item = DirectElement;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len() {
            self.index += 1;
            Some(self.remove(0)) // Remove and return the first element
        } else {
            None
        }
    }
}

// Owned IntoIterator
impl IntoIterator for DirectElementCollection {
    type Item = Box<DirectElement>; // Owns the items when iterating
    type IntoIter = std::vec::IntoIter<Box<DirectElement>>;

    fn into_iter(self) -> Self::IntoIter {
        self.direct_elements.into_iter()
    }
}

// Borrowed IntoIterator
impl<'a> IntoIterator for &'a DirectElementIterator {
    type Item = &'a DirectElement;
    type IntoIter = std::iter::Map<std::slice::Iter<'a, Box<DirectElement>>, fn(&Box<DirectElement>) -> &DirectElement>;

    fn into_iter(self) -> Self::IntoIter {
        self.direct_elements.iter().map(|b| &**b) // Convert `&Box<DirectElement>` to `&DirectElement`
    }
}

// Mutable
impl<'a> IntoIterator for &'a mut DirectElementCollection {
    type Item = &'a mut DirectElement;
    type IntoIter = std::iter::Map<
        std::slice::IterMut<'a, Box<DirectElement>>,
        fn(&mut Box<DirectElement>) -> &mut DirectElement,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.direct_elements.iter_mut().map(|direct_element| direct_element.as_mut())
    }
}
*/

/*
 * SchemaElement
 */

/*
 * trait making DirectElement and IndirectElement work well together
 * name:    Function that returns the name of the element
 * get:     Search for an element by name
 */
pub trait SchemaElement<'a> {
    fn get<'b>(&self, name: &str) -> Option<&Box<dyn SchemaElement>>;
    fn name(&self) -> String;
    fn subelements(&self) -> &Vec<Box<&dyn SchemaElement>>;

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

    fn as_any(&self) -> &dyn Any {
        &self
    }
}

struct SchemaElementCollection<'a> {
    schema_elements: Vec<Box<&'a dyn SchemaElement<'a>>>, // Owned heap-allocated trait objects
}

// Owned Iterator
struct SchemaElementIterator<'a> {
    schema_elements: Vec<&'a mut dyn Iterator<Item = &'a dyn SchemaElement<'a>>>,
}

impl<'a> Iterator for SchemaElementIterator<'a> {
    type Item = &'a dyn SchemaElement<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let len = self.schema_elements.len();

            if len == 0 {
println!("Got nuttin");
                return None;
            } else {
print!("using next() on element {}: ", len - 1);
                match self.schema_elements[len - 1].next() {
                    None => {
println!("nuttin");
                        self.schema_elements.pop();
                        if self.schema_elements.len() == 0 {
                            return None;
                        }
                    },
                    Some(schema_element) => {
println!("sumptin");
                        return Some(schema_element);
                    },
                };
            }
        }
    }
}

/*
// Owned IntoIterator
impl IntoIterator for SchemaElementCollection {
    type Item = Box<&dyn SchemaElement>; // Owns the items when iterating
    type IntoIter = std::vec::IntoIter<Box<&dyn SchemaElement>>;

    fn into_iter(self) -> Self::IntoIter {
        self.schema_elements.into_iter()
    }
}

// Borrowed IntoIterator
impl<'a> IntoIterator for &'a SchemaElementIterator {
    type Item = &'a dyn SchemaElement;
    type IntoIter = std::iter::Map<std::slice::Iter<'a, SchemaElement>, fn(&SchemaElement) -> &dyn SchemaElement>;

    fn into_iter(self) -> Self::IntoIter {
        self.schema_elements.iter().map(|b| &**b) // Convert `&SchemaElement` to `&dyn SchemaElement`
    }
}

// Mutable
impl<'a> IntoIterator for &'a mut SchemaElementCollection {
    type Item = &'a mut dyn SchemaElement;
    type IntoIter = std::iter::Map<
        std::slice::IterMut<'a, SchemaElement>,
        fn(&mut SchemaElement) -> &mut dyn SchemaElement,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.schema_elements.iter_mut().map(|schema_element| schema_element.as_mut())
    }
}
*/

/*
 * SchemaAttribute
 */
impl SchemaAttribute {
    pub fn new() -> SchemaAttribute {
        SchemaAttribute {
        }
    }
}

impl fmt::Display for SchemaAttribute {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

/*
// This allows both:
// Borrowed iteration (for animal in &collection { ... })
// Owned iteration (for animal in collection { ... })

trait Animal {
    fn name(&self) -> &str;
}

struct AnimalCollection {
    animals: Vec<Box<Box<dyndyn Animal>>, // Owned heap-allocated trait objects
}

// Owned Iterator
struct AnimalIterator {
    animals: Vec<Box<Box<dyndyn Animal>>, // A Vec of trait objects
    index: usize,
}

impl Iterator for AnimalIterator {
    type Item = Box<Box<dyndyn Animal>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.animals.len() {
            self.index += 1;
            Some(self.animals.remove(0)) // Remove and return the first element
        } else {
            None
        }
    }
}

// Borrowed Iterator
impl Iterator for AnimalIterator {
    type Item = Box<Box<dyndyn Animal>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.animals.len() {
            self.index += 1;
            Some(self.animals.remove(0)) // Remove and return the first element
        } else {
            None
        }
    }
}


// Owned IntoIterator
impl IntoIterator for AnimalCollection {
    type Item = Box<Box<dyndyn Animal>; // Owns the items when iterating
    type IntoIter = std::vec::IntoIter<Box<Box<dyndyn Animal>>;

    fn into_iter(self) -> Self::IntoIter {
        self.animals.into_iter()
    }
}

// Borrowed IntoIterator
impl<'a> IntoIterator for &'a AnimalCollection {
    type Item = &'a dyn Animal;
    type IntoIter = std::iter::Map<std::slice::Iter<'a, Box<Box<dyndyn Animal>>, fn(&Box<Box<dyndyn Animal>) -> &dyn Animal>;

    fn into_iter(self) -> Self::IntoIter {
        self.animals.iter().map(|b| &**b) // Convert `&Box<Box<dyndyn Animal>` to `&dyn Animal`
    }
}

// Mutable
impl<'a> IntoIterator for &'a mut AnimalCollection {
    type Item = &'a mut dyn Animal;
    type IntoIter = std::iter::Map<
        std::slice::IterMut<'a, Box<Box<dyndyn Animal>>,
        fn(&mut Box<Box<dyndyn Animal>) -> &mut dyn Animal,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.animals.iter_mut().map(|animal| animal.as_mut())
    }
}


fn main() {
    let dog = Box::new(Dog { name: "Rex".into() });
    let cat = Box::new(Cat { name: "Whiskers".into() });

    let animal_collection = AnimalCollection {
        animals: vec![dog, cat], // Owned trait objects
    };

    println!("Borrowed iteration:");
    for animal in &animal_collection {
        println!("Animal: {}", animal.name());
    }

    println!("\nOwned iteration:");
    for animal in animal_collection {
        println!("Animal: {}", animal.name());
    }
}
*/
