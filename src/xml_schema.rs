/*
 * Define the data structures used to describe the XML used for parsing.
 */
// FIXME: make sure errors returned are appropriate

use std::fmt;

use crate::xml_document_error::XmlDocumentError;

pub type DefIdx = usize;

/*
 * Top-level definition of the schema
 * root_index:              Indicates which DirectElement is the root
 * key:                     Name of the root DirectElement
 * schema_elements_map: HashMap with they DirectElement key as the
 *                          key and the value an index into schema_elements
 * schema_elements:     Array of DirectElement
 */
#[derive(Debug)]
pub struct XmlSchema {
    pub name:       String,
    pub element:    DirectElement,
}

/*
 * trait making DirectElement and IndirectElement work well together
 * name:    Function that returns the name of the element
 * get:     Search for an element by name
 */
pub trait SchemaElement {
    fn name(&self) -> String;
    fn get(&self, name: &str) -> Option<&DirectElement>;
    fn display(&self, f: &mut fmt::Formatter, depth: DefIdx) -> fmt::Result;
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
#[derive(Clone, Debug)]
pub struct DirectElement {
    pub name:           String,
    pub attributes:     Vec<SchemaAttribute>,
    pub subelements:    Vec<DirectElement>,
}

#[derive(Clone, Debug)]
pub struct IndirectElement<'a> {
    direct_element: &'a DirectElement,
}

#[derive(Clone, Debug)]
pub struct SchemaAttribute {
}

impl XmlSchema {
    pub fn new(name: &str, element: DirectElement) -> XmlSchema {
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

    pub fn display_element(&self, f: &mut fmt::Formatter, depth: DefIdx,
        element: &DirectElement) ->
        fmt::Result {
        const INDENT_STR: &str = "   ";
        let indent_string = INDENT_STR.to_string().repeat(depth);

        write!(f, "{}{}", indent_string, element.name)?;
        let subelements = &element.subelements;

        if subelements.len() == 0 {
            write!(f, " []\n")?;
        } else {
            write!(f, " [\n")?;

            for elem in subelements {
                self.display_element(f, depth + 1, elem)?;
            }

            write!(f, "{}]\n", indent_string)?;
        }

        Ok(())
    }


    pub fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let depth = 0;
        self.display_element(f, depth, &self.element)?;

        Ok(())
    }
}
        
impl fmt::Display for XmlSchema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
write!(f, "{}\n", "Display for XmlSchema")?;
        self.display(f)?;
        Ok(())
    }
}

impl DirectElement {
    pub fn new(name: &str, subelements: Vec<DirectElement>) ->
        DirectElement {
        DirectElement {
            name:           name.to_string(),
            attributes:     vec!(),
            subelements:    subelements,
        }
    }

    // Find an DirectElement whose name matches the given one
    pub fn get(&self, name: &str) -> Option<&DirectElement> {
        match self
            .subelements
            .iter()
            .find(move |&schema_element| schema_element.name == name) {
            None => None,
            Some(schema_elem) => Some(&schema_elem),
        }
    }

    pub fn display_root(&self, f: &mut fmt::Formatter, depth: DefIdx) ->
        fmt::Result{
        const INDENT_SLOT: &str = "   ";
        let indent_str = INDENT_SLOT.repeat(depth);
        write!(f, "{}{}\n", indent_str, self.name)?;
        Ok(())
    }
}

impl SchemaElement for DirectElement {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn get(&self, name: &str) -> Option<&DirectElement> {
        println!("get({})", name);
        None
    }

    fn display(&self, f: &mut fmt::Formatter, _depth: DefIdx) -> fmt::Result {
        self.display_root(f, 0)
    }
}

impl fmt::Display for DirectElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.display_root(f, 0)?;
        Ok(())
    }
}

impl IndirectElement<'_> {
    pub fn new(direct_element: &DirectElement) -> IndirectElement {
        IndirectElement {
            direct_element: direct_element,
        }
    }


    fn display_root(&self, _f: &mut fmt::Formatter, _depth: DefIdx) -> fmt::Result {
        Ok(())
    }
}

impl SchemaElement for IndirectElement<'_> {
    fn name(&self) -> String {
        self.direct_element.name.clone()
    }

    fn get(&self, name: &str) -> Option<&DirectElement> {
        println!("get({})", name);
        None
    }

    fn display(&self, f: &mut fmt::Formatter, _depth: DefIdx) -> fmt::Result {
        self.display_root(f, 0)
    }
}
struct SchemaElementCollection {
    schema_elements: Vec<Box<dyn SchemaElement>>, // Owned heap-allocated trait objects
}

// Owned Iterator
struct SchemaElementIterator {
    schema_elements: Vec<Box<dyn SchemaElement>>, // A Vec of trait objects
    index: usize,
}

// Owned IntoIterator
impl IntoIterator for SchemaElementCollection {
    type Item = Box<dyn SchemaElement>; // Owns the items when iterating
    type IntoIter = std::vec::IntoIter<Box<dyn SchemaElement>>;

    fn into_iter(self) -> Self::IntoIter {
        self.schema_elements.into_iter()
    }
}

// Borrowed IntoIterator
impl<'a> IntoIterator for &'a SchemaElementIterator {
    type Item = &'a dyn SchemaElement;
    type IntoIter = std::iter::Map<std::slice::Iter<'a, Box<dyn SchemaElement>>, fn(&Box<dyn SchemaElement>) -> &dyn SchemaElement>;

    fn into_iter(self) -> Self::IntoIter {
        self.schema_elements.iter().map(|b| &**b) // Convert `&Box<dyn SchemaElement>` to `&dyn SchemaElement`
    }
}

// Mutable
impl<'a> IntoIterator for &'a mut SchemaElementCollection {
    type Item = &'a mut dyn SchemaElement;
    type IntoIter = std::iter::Map<
        std::slice::IterMut<'a, Box<dyn SchemaElement>>,
        fn(&mut Box<dyn SchemaElement>) -> &mut dyn SchemaElement,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.schema_elements.iter_mut().map(|schema_element| schema_element.as_mut())
    }
}

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
    animals: Vec<Box<dyn Animal>>, // Owned heap-allocated trait objects
}

// Owned Iterator
struct AnimalIterator {
    animals: Vec<Box<dyn Animal>>, // A Vec of trait objects
    index: usize,
}

impl Iterator for AnimalIterator {
    type Item = Box<dyn Animal>;

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
    type Item = Box<dyn Animal>;

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
    type Item = Box<dyn Animal>; // Owns the items when iterating
    type IntoIter = std::vec::IntoIter<Box<dyn Animal>>;

    fn into_iter(self) -> Self::IntoIter {
        self.animals.into_iter()
    }
}

// Borrowed IntoIterator
impl<'a> IntoIterator for &'a AnimalCollection {
    type Item = &'a dyn Animal;
    type IntoIter = std::iter::Map<std::slice::Iter<'a, Box<dyn Animal>>, fn(&Box<dyn Animal>) -> &dyn Animal>;

    fn into_iter(self) -> Self::IntoIter {
        self.animals.iter().map(|b| &**b) // Convert `&Box<dyn Animal>` to `&dyn Animal`
    }
}

// Mutable
impl<'a> IntoIterator for &'a mut AnimalCollection {
    type Item = &'a mut dyn Animal;
    type IntoIter = std::iter::Map<
        std::slice::IterMut<'a, Box<dyn Animal>>,
        fn(&mut Box<dyn Animal>) -> &mut dyn Animal,
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
