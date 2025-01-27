use std::fmt;

use crate::xml_document_error::XmlDocumentError;

/*
 * Define the data structures used to describe the XML used for parsing.
 */
pub struct XmlDefinition<'a> {
    pub root:  &'a [&'a ElementDefinition<'a>],
}

impl<'a> XmlDefinition<'a> {
// FIXME: use this
    fn _validate(_xml_definition: &'a XmlDefinition) -> Result<(), XmlDocumentError> {
        todo!();
        // o    Make sure no duplications in element_definitions
        // o    Ensure no duplicates in any element_definitions
        // o    Ensure the root is in element_definitions
        // o    Ensure at least one element
//        Err(XmlDocumentError::Unknown(0))
    }

    pub fn display_element_def(&self, f: &mut fmt::Formatter<'_>, depth: usize,
        element_definition: &[&ElementDefinition]) ->
    fmt::Result {
        const INDENT_STR: &str = "   ";
        let indent_string = INDENT_STR.to_string().repeat(depth);

        if element_definition.len() == 0 {
            write!(f, "[]\n");
        } else {
            write!(f, "{}[\n", indent_string)?;

            for element_def in element_definition.iter() {
                write!(f, "{}{}\n", indent_string, element_def.name)?;
                self.display_element_def(f, depth + 1,
                    element_def.allowable_subelements)?;
            }

            write!(f, "{}]\n", indent_string)?;
        }

        Ok(())
    }

    pub fn display(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
println!("display XmlDefinition");

        let depth = 0;
        self.display_element_def(f, depth, &self.root)?;

        Ok(())
    }
}
        
impl fmt::Display for XmlDefinition<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f)
    }
}

pub struct ElementDefinition<'a> {
    pub name:                   &'a str,
    pub allowable_subelements:  &'a [&'a ElementDefinition<'a>],
}

impl<'a> ElementDefinition<'a> {
    pub fn display(&self, f: &mut fmt::Formatter<'_>, depth: usize) ->
        fmt::Result{
        const INDENT_SLOT: &str = "   ";
        let indent_str = INDENT_SLOT.repeat(depth);
        write!(f, "{}{}\n", indent_str, self.name)?;
        let sub_indent = INDENT_SLOT.repeat(depth + 1);
        write!(f, "{}[\n", sub_indent)?;
        for element_def in self.allowable_subelements.iter() {
            element_def.display(f, depth)?;
        }
        write!(f, "{}]", sub_indent)?;
        Ok(())
    }
}

impl<'a> fmt::Display for ElementDefinition<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f, 0)?;
        Ok(())
    }
}

impl<'a> fmt::Debug for ElementDefinition<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f, 0)?;
        Ok(())
    }
}
