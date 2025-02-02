/*
 * Define the data structures used to describe the XML used for parsing.
 */
// FIXME: make sure errors returned are appropriate

use std::collections::HashMap;
use std::fmt;

use crate::xml_document_error::XmlDocumentError;

pub type DefIdx = usize;

/*
 * Top-level definition of the schema
 * root_index:              Indicates which ElementDefinition is the root
 * key:                     Name of the root ElementDefinition
 * element_definitions_map: HashMap with they ElementDefinition key as the
 *                          key and the value an index into element_definitions
 * element_definitions:     Array of ElementDefinition
 */
pub struct XmlDefinition {
    pub root_index:                 Option<DefIdx>,
    pub key:                        String,
    pub element_definitions_map:    HashMap<String, DefIdx>,
    pub element_definitions:        Vec::<ElementDefinition>,
}

/*
 * Information for each XML Element
 * name:                        Element name, which might not be unique
 * key:                         Key for this ElementDescription, which must be
 *                              unique
 * allowable_subelement_keys:   Keys indicating the subelements of this
 *                              ElementDefinition.
 * allowable_subelement_vec:   Array with the indices into element_definitions
 *                              for each item in allowable_element_keys
 */
#[derive(Clone)]
pub struct ElementDefinition {
    pub name:                       String,
    pub key:                        String,
    pub allowable_subelement_keys:  Vec<String>,
    pub allowable_subelement_vec:   Vec<DefIdx>,
}

impl XmlDefinition {
    pub fn new(key: String, element_definitions: Vec<ElementDefinition>) ->
        XmlDefinition {
        let mut xml_definition = XmlDefinition {
            root_index:                 None,
            key:                        key,
            element_definitions_map:    HashMap::<String, DefIdx>::new(),
            element_definitions:        element_definitions,
        };

        let x = xml_definition.patch();
        println!("new(): {:?}", x);
        xml_definition
    }

    pub fn patch(&mut self) -> Result<(), XmlDocumentError> {
        let patches = Self::make_patches(&self.element_definitions,
            &self.element_definitions_map)?;

    for (key, index) in &patches.0 {
        println!("Element Key: {}, Allowable Subelements: {:?}", key, index);
    }

        Self::apply_patches(&mut self.element_definitions,
            &mut self.element_definitions_map, patches)?;

        Ok(())
    }

    fn make_patches(element_definitions: &Vec<ElementDefinition>,
        element_definitions_map: &HashMap<String, DefIdx>) ->
        Result<(Vec<(String, DefIdx)>, Vec::<(DefIdx, DefIdx)>),
            XmlDocumentError> {
        let element_patch = Self::make_element_patch(element_definitions)?;
        let subelement_patch = Self::make_subelement_patch(element_definitions,
            element_definitions_map)?;
        Ok((element_patch, subelement_patch))
    }

    fn apply_patches(element_definitions: &mut Vec<ElementDefinition>,
        element_definitions_map: &mut HashMap<String, DefIdx>,
        (element_patch, subelement_patch): (Vec<(String, DefIdx)>,Vec<(DefIdx, DefIdx)>)) ->
        Result<(), XmlDocumentError> {
        Self::apply_element_patch(element_definitions, element_definitions_map,
            element_patch)?;
        Self::apply_subelement_patch(element_definitions, subelement_patch)?;
        Ok(())
    }

    /*
     * Create a patch relating keys to ElementDefinition indices
     */
    fn make_element_patch(element_definitions: &Vec<ElementDefinition>) 
        -> Result<Vec<(String, usize)>, XmlDocumentError> {
        Ok(element_definitions
            .iter()
            .enumerate() // (index, &ElementDefinition)
            .map(|(index, element)| (element.key.clone(), index)) // Clone key to make it owned
            .collect() // Collect into Vec<(usize, String)>, which owns its data
        )
    }

    fn apply_element_patch(element_definitions: &Vec<ElementDefinition>,
        element_definitions_map: &mut HashMap<String, DefIdx>,
        patch: Vec::<(String, DefIdx)>) ->
        Result<(), XmlDocumentError> {
println!("apply_nodes_patch: patch has {} elements", patch.len());
        for (element_key, index) in patch {
println!("add_node: {}", element_key);
            let element_key2 = element_key.clone();

println!("apply_element_patch: add ({}, {})", element_key, index);
            match element_definitions_map
                .insert(element_key.clone(), index) {
                None => {println!("node {} inserted: None", element_key2)},
                Some(idx) => {
                    println!("node {} not inserted: Some {}", element_key2, element_definitions[idx].key);
                    return Err(XmlDocumentError::DuplicateKey(element_key2.to_string()))
                    },
            }
        }
println!("apply_nodes_patch: element_definitions.len() {}", element_definitions_map.len());

        Ok(())
    }

    fn make_subelement_patch(element_definitions: &Vec<ElementDefinition>,
        element_definitions_map: &HashMap<String, DefIdx>)  ->
        Result<Vec::<(DefIdx, DefIdx)>, XmlDocumentError> {
        let mut patch = Vec::<(DefIdx, DefIdx)>::new();
println!("make_edge_patch: element_definitions.len() {}", element_definitions_map.len());

        for (to_patch_index, &ref _element_def) in element_definitions.into_iter().enumerate() {
            let element_def_key = element_definitions[to_patch_index].key.to_string();
println!("key: {}", element_def_key);

// FIXME: avoid this clone
            for (_j, key) in element_definitions[to_patch_index].allowable_subelement_keys.clone().into_iter().enumerate() {
                let key2 = key.clone();
println!("    {}", key2);

                let patch_with_index = match element_definitions_map
                    .get(&key) {
                    Some(&idx) => idx,
                    None => return Err(XmlDocumentError::AllowableKeyNotAnElement(key2, element_def_key)),
                };
                patch.push((to_patch_index.clone(), patch_with_index.clone()));
            }
        }

println!("make_edges_patch: patch has {} elements", patch.len());
        Ok(patch)
    }

    fn apply_subelement_patch(element_definitions: &mut Vec<ElementDefinition>,
        patch: Vec::<(DefIdx, DefIdx)>) -> Result<(), XmlDocumentError> {
        for (to_patch_index, patch_with_index) in patch {
            element_definitions[to_patch_index].allowable_subelement_vec.push(patch_with_index);
        }

        Ok(())
    }

    pub fn validate(&self) -> Result<(), XmlDocumentError> {
        println!("Not validating yet");
        Ok(())
    }

    pub fn display_element_def(&self, f: &mut fmt::Formatter, depth: DefIdx,
        element_def: &ElementDefinition) ->
        fmt::Result {
        const INDENT_STR: &str = "   ";
        let indent_string = INDENT_STR.to_string().repeat(depth);

        write!(f, "{}{} [{}]", indent_string, element_def.name,
            element_def.key)?;

        let allowable_subelements = &element_def.allowable_subelement_vec;

        if allowable_subelements.len() == 0 {
            write!(f, " []\n")?;
        } else {
            write!(f, " [\n")?;

            for i in &element_def.allowable_subelement_vec {
                self.display_element_def(f, depth + 1, &self.element_definitions[*i])?;
            }

            write!(f, "{}]\n", indent_string)?;
        }

        Ok(())
    }


    pub fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let depth = 0;
        
        let root_index = match self.root_index {
            None => return Err(fmt::Error),
            Some(idx) => idx,
        };

        self.display_element_def(f, depth, &self.element_definitions[root_index])?;

        Ok(())
    }
}
        
impl fmt::Display for XmlDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
write!(f, "{}\n", "Display for XmlDefinition")?;
        self.display(f)?;
        Ok(())
    }
}

impl ElementDefinition {
    pub fn new(name: &str, key: &str, allowable_keys: Vec<String>) ->
        ElementDefinition {
        ElementDefinition {
            name:                       name.to_string(),
            key:                        key.to_string(),
            allowable_subelement_keys:  allowable_keys,
            allowable_subelement_vec:   Vec::<DefIdx>::new(),
        }
    }

    pub fn display(&self, f: &mut fmt::Formatter, depth: DefIdx) ->
        fmt::Result{
        const INDENT_SLOT: &str = "   ";
        let indent_str = INDENT_SLOT.repeat(depth);
        write!(f, "{}{} [{}]\n", indent_str, self.name, self.key)?;
        Ok(())
    }
}

impl fmt::Display for ElementDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.display(f, 0)?;
        Ok(())
    }
}
