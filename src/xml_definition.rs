/*
 * Define the data structures used to describe the XML used for parsing.
 */

use std::fmt;

use crate::xml_document_error::XmlDocumentError;

/*
 * Top-level definition of the schema
 * root:    Pointer to the root ElementDefinition
 * key:     Name of the root ElementDefinition
 */
pub struct XmlDefinition<'a> {
    pub root:                   Option<&'a ElementDefinition<'a>>,
    pub key:                    &'a str,
    pub element_definitions:    &'a [ElementDefinition<'a>],
}

impl<'a> XmlDefinition<'a> {
// FIXME: use this
    pub fn validate(&self) -> Result<(), XmlDocumentError<'a>> {
        // o    Make sure no duplications in element_definitions
        // o    Ensure no duplicates in any element_definitions
        // o    Ensure the root is in element_definitions
        // o    Ensure at least one element
        // There are faster ways to do these things
        let def_len = self.element_definitions.len();

        for (outer, outer_def) in
            self.element_definitions[..def_len - 1].iter().enumerate()  {
            let outer_key = outer_def.key;

            for (inner, inner_def) in
                self.element_definitions[outer + 1..].iter().enumerate() {
                let inner_key = inner_def.name;
                if inner_key == outer_key {
                    return Err(XmlDocumentError::DuplicateKey(inner_key,
                        outer, outer + inner));
                }
            } 

            let allowable_len = outer_def.allowable_subelements.len();

            if allowable_len > 0 {
                for (i, i_allowable) in
                    outer_def.allowable_subelement_names[..allowable_len - 1].iter().enumerate() {
                    for (j, j_allowable) in
                        outer_def.allowable_subelement_names[i + 1..].iter().enumerate() {
                        if i_allowable == j_allowable {
                            return Err(XmlDocumentError::DuplicateAllowableElement(i_allowable, i, i + j));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn display_element_def(&self, f: &mut fmt::Formatter<'_>, depth: usize,
        element_definition: &ElementDefinition) ->
    fmt::Result {
// FIXME: use a better way to detect the end. I need some way to uniquely
// identify the ElementDefinitions
if depth > 8 {
    return Ok(());
}
        const INDENT_STR: &str = "   ";
        let indent_string = INDENT_STR.to_string().repeat(depth);

        write!(f, "{}{}", indent_string, element_definition.name)?;

        let allowable_subelements = &element_definition.allowable_subelements;

        if allowable_subelements.len() == 0 {
            write!(f, " []\n")?;
        } else {
            write!(f, " [\n")?;

            for element_def in allowable_subelements.iter() {
                self.display_element_def(f, depth + 1, element_def)?;
            }

            write!(f, "{}]\n", indent_string)?;
        }

        Ok(())
    }

    pub fn display(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let depth = 0;
        self.display_element_def(f, depth, self.root.unwrap())?;
        Ok(())
    }
}
        
impl fmt::Display for XmlDefinition<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
write!(f, "{}\n", "Display for XmlDefinition")?;
        self.display(f)?;
        Ok(())
    }
}

pub struct ElementDefinition<'a> {
    pub name:                       &'a str,
    pub key:                        &'a str,
    pub allowable_subelements:      Vec<&'a ElementDefinition<'a>>,
    pub allowable_subelement_names: &'a [&'a str],
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
