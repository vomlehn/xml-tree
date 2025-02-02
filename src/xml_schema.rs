/*
 * Define the data structures used to describe the XML used for parsing.
 */
// FIXME: make sure errors returned are appropriate

use std::fmt;

use crate::xml_document_error::XmlDocumentError;

pub type DefIdx = usize;

/*
 * Top-level definition of the schema
 * root_index:              Indicates which SchemaElement is the root
 * key:                     Name of the root SchemaElement
 * schema_elements_map: HashMap with they SchemaElement key as the
 *                          key and the value an index into schema_elements
 * schema_elements:     Array of SchemaElement
 */
#[derive(Debug)]
pub struct XmlSchema {
    pub name:       String,
    pub element:    SchemaElement,
}

/*
 * Information for each XML Element
 * name:                        Element name, which might not be unique
 * key:                         Key for this ElementDescription, which must be
 *                              unique
 * allowable_subelement_keys:   Keys indicating the subelements of this
 *                              SchemaElement.
 * allowable_subelement_vec:   Array with the indices into schema_elements
 *                              for each item in allowable_element_keys
 */
#[derive(Clone, Debug)]
pub struct SchemaElement {
    pub name:           String,
    pub subelements:    Vec<SchemaElement>,
}

impl XmlSchema {
    pub fn new(name: &str, element: SchemaElement) -> XmlSchema {
        let xml_schema = XmlSchema {
            name:       name.to_string(),
            element:    element,
        };

        println!("new(): {:?}", xml_schema);
        xml_schema
    }

    pub fn get(&self, name: &str) -> Option<&SchemaElement> {
        for element in &self.element.subelements {
            if element.name == name {
                return Some(element);
            }
        }

        None
    }

    pub fn validate(&self) -> Result<(), XmlDocumentError> {
        println!("Not validating yet");
        Ok(())
    }

    pub fn display_element(&self, f: &mut fmt::Formatter, depth: DefIdx,
        element: &SchemaElement) ->
        fmt::Result {
        const INDENT_STR: &str = "   ";
        let indent_string = INDENT_STR.to_string().repeat(depth);

        write!(f, "{}{}", indent_string, element.name);
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

impl SchemaElement {
    pub fn new(name: &str, subelements: Vec<SchemaElement>) ->
        SchemaElement {
        SchemaElement {
            name:           name.to_string(),
            subelements:    subelements,
        }
    }

    pub fn display(&self, f: &mut fmt::Formatter, depth: DefIdx) ->
        fmt::Result{
        const INDENT_SLOT: &str = "   ";
        let indent_str = INDENT_SLOT.repeat(depth);
        write!(f, "{}{}\n", indent_str, self.name)?;
        Ok(())
    }
}

impl fmt::Display for SchemaElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.display(f, 0)?;
        Ok(())
    }
}
