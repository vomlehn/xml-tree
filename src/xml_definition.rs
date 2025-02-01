/*
 * Define the data structures used to describe the XML used for parsing.
 */
// FIXME: make sure errors returned are appropriate

use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::prelude::Dfs;

use crate::xml_document_error::XmlDocumentError;

/*
 * Top-level definition of the schema
 * root:    Pointer to the root ElementDefinition
 * key:     Name of the root ElementDefinition
 */
pub struct XmlDefinition {
    pub root_index:                 Option<NodeIndex>,
    pub key:                        String,
    pub graph:                      DiGraph<ElementDefinition, String>,
    pub element_definitions_map:    HashMap<String, NodeIndex>,
    pub element_definitions:        Vec::<ElementDefinition>,
}

#[derive(Clone)]
pub struct ElementDefinition {
    pub name:                       String,
    pub key:                        String,
    pub allowable_subelement_keys:  Vec<String>,
    pub allowable_subelements_map:  HashMap<String, NodeIndex>,
}

impl<'a> XmlDefinition {
    pub fn patch(&mut self) -> Result<(), XmlDocumentError> {
        let nodes_patch = Self::make_nodes_patch(&self.element_definitions)?;
        let edges_patch = Self::make_edges_patch(&mut self.element_definitions_map,
             &mut self.graph)?;
        Self::apply_nodes_patch(&mut self.graph, &mut self.element_definitions_map,
            nodes_patch)?;
        Self::apply_edges_patch(&mut self.graph, edges_patch);
        let root_index = {
            let index = self
                .element_definitions_map
                .get(&self.key)
                .copied();

            index.ok_or_else(|| XmlDocumentError::RootKeyNotFound(Cow::Owned(self.key.clone())))?
        };

        self.root_index = Some(root_index);
        Ok(())
    }

    fn make_nodes_patch(element_definitions: &Vec<ElementDefinition>) ->
        Result<Vec::<(String, ElementDefinition)>, XmlDocumentError> {
        let mut patch = Vec::<(String, ElementDefinition)>::new();

        for element_def in element_definitions {
            let element_key = element_def.key.clone();
            patch.push((element_key.to_string(), element_def.clone()));
        }

        Ok(patch)
    }

    fn apply_nodes_patch<'b>(graph: &mut DiGraph<ElementDefinition, String>,
        element_definitions_map: &mut HashMap<String, NodeIndex>,
        patch: Vec::<(String, ElementDefinition)>) ->
        Result<(), XmlDocumentError<'b>> {
        for (element_key, element_def) in patch {
            let node_index = graph.add_node(element_def.clone());
            let element_key2 = element_key.clone();

            match element_definitions_map
                .insert(element_key.clone(), node_index.clone()) {
                None => {},
                Some(_) => return Err(XmlDocumentError::DuplicateKey(Cow::Owned(element_key2.to_string()))),
            }
        }

        Ok(())
    }

    fn make_edges_patch<'b>(element_definitions_map: &mut HashMap<String, NodeIndex>,
        graph: &mut DiGraph<ElementDefinition, String>) ->
        Result<Vec::<(NodeIndex, NodeIndex)>, XmlDocumentError<'b>> {
        let mut patch = Vec::<(NodeIndex, NodeIndex)>::new();

        for (_key, &to_patch_index) in element_definitions_map
            .iter()
            .map(|(key, node_index)| (key, node_index)) {
            let element_def = &graph[to_patch_index];
            let element_def_key = element_def.key.clone();
            for key in &element_def.allowable_subelement_keys {
                let key2 = key.clone();
                let patch_with_index = match element_definitions_map
                    .get(key) {
                    Some(&idx) => idx,
                    None => return Err(XmlDocumentError::AllowableKeyNotAnElement(Cow::Owned(key2), Cow::Owned(element_def_key))),
                };
                patch.push((to_patch_index.clone(), patch_with_index.clone()));
            }
        }

        Ok(patch)
    }

    fn apply_edges_patch(graph: &mut DiGraph<ElementDefinition, String>,
        patch: Vec::<(NodeIndex, NodeIndex)>) {

        for (to_patch_index, patch_with_index) in patch {
            graph.add_edge(to_patch_index, patch_with_index, "".to_string());
        }
    }


    pub fn display_element_def(&self, f: &mut fmt::Formatter<'_>, depth: usize,
        element_definition: &ElementDefinition) ->
        fmt::Result {
        const INDENT_STR: &str = "   ";
        let indent_string = INDENT_STR.to_string().repeat(depth);

        write!(f, "{}{} [{}]", indent_string, element_definition.name,
            element_definition.key)?;

        let allowable_subelements = &element_definition.allowable_subelements_map;

        if allowable_subelements.len() == 0 {
            write!(f, " []\n")?;
        } else {
            write!(f, " [\n")?;

/*
            for element_def in allowable_subelements.values() {
// FIXME: handle errors
                self.display_element_def(f, depth + 1, element_def)?;
            }

*/
            write!(f, "{}]\n", indent_string)?;
        }

        Ok(())
    }

    pub fn validate(&self) -> Result<(), XmlDocumentError> {
        println!("Not validating yet");
        Ok(())
    }

    pub fn display(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let depth = 0;
        
        let root_index = match &self.root_index {
            None => return Err(fmt::Error),
            Some(idx) => idx,
        };

        let mut dfs = Dfs::new(&self.graph, *root_index);

        while let Some(node_index) = dfs.next(&self.graph) {
            self.display_element_def(f, depth, &self.graph[node_index])?;
        }

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

impl<'a> ElementDefinition {
    pub fn new(name: &str, key: &str, allowable_keys: Vec<String>) ->
        ElementDefinition {
        ElementDefinition {
            name:                       name.to_string(),
            key:                        key.to_string(),
            allowable_subelement_keys:  allowable_keys,
            allowable_subelements_map:  HashMap::<String, NodeIndex>::new(),
        }
    }

    pub fn display(&self, f: &mut fmt::Formatter<'_>, depth: usize) ->
        fmt::Result{
        const INDENT_SLOT: &str = "   ";
        let indent_str = INDENT_SLOT.repeat(depth);
        write!(f, "{}{} [{}]\n", indent_str, self.name, self.key)?;
        Ok(())
    }
}

impl<'a> fmt::Display for ElementDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.display(f, 0)?;
        Ok(())
    }
}
