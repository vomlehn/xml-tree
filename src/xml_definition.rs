use std::fmt;

use crate::xml_document_error::XmlDocumentError;

/*
 * Define the data structures used to describe the XML used for parsing.
 */
pub struct XmlDefinition<'a> {
    pub root_name:  &'a str,
    pub element_definitions:  &'a [ElementDefinition<'a>],
}

impl<'a> XmlDefinition<'a> {
    fn validate(_xml_definition: &'a XmlDefinition) -> Result<(), XmlDocumentError> {
        todo!();
        // o    Make sure no duplications in element_definitions
        // o    Ensure no duplicates in any element_definitions
        // o    Ensure the root is in element_definitions
        // o    Ensure at least one element
//        Err(XmlDocumentError::Unknown(0))
    }
}
pub struct ElementDefinition<'a> {
    pub name:                   &'a str,
    pub allowable_subelements:  &'a [&'a str],
}

impl<'a> ElementDefinition<'a> {
    fn fmt_no_circular(&self, f: &mut fmt::Formatter<'_>, active: &mut Vec<&String>) -> fmt::Result {
        let mut sep_subelem = "";

        write!(f, "{}:\n", self.name)?;
        write!(f, "   [")?;

        for element_name in self.allowable_subelements.iter() {
            for name in &*active {
                if *name == element_name {
                    eprintln!("Circular dependency starting at {}", name);
                    std::process::exit(1);
                }
            }

            write!(f, "{}{}", sep_subelem, element_name)?;
            sep_subelem = ", ";
        }

        write!(f, "]\n")?;
       
        for element_name in self.allowable_subelements.iter() {
            write!(f, "{:?}", element_name)?;
        }

        Ok(())
    }
}

impl<'a> fmt::Display for ElementDefinition<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut active = Vec::<&String>::new();
        self.fmt_no_circular(f, &mut active)
    }
}

impl<'a> fmt::Debug for ElementDefinition<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?}", self.name, self.allowable_subelements)
    }
}
