use std::io::{BufReader, Read};

pub struct ParseSimple {
}

impl ParseSimple {
    pub fn new(document_info: DocumentInfo, root: Box<dyn Element>) -> Self {
        ParseEcho {
        }
    }

    pub fn parse_path<'b>(
        path: &'b str,
        element_level_info: &<ParseSimple as ParseDoc>::LI,
    ) -> Result<(DocumentInfo, <<<ParseSimple as ParseDoc>::LI as LevelInfo>::AccumulatorType as Accumulator>::Value), XmlDocumentError>
    {
        Self::parse_path_base(path, element_level_info)
    }

    pub fn parse<R>(
        buf_reader: BufReader<R>,
        element_level_info: &<ParseSimple as ParseDoc>::LI,
    ) -> Result<(DocumentInfo, <<<ParseSimple as ParseDoc>::LI as LevelInfo>::AccumulatorType as Accumulator>::Value), XmlDocumentError>
    where
        R: Read,
    {
        Self::parse_base(buf_reader, element_level_info)
    }
}

/// LevelInfo that doesn't track depth or any other information
#[derive(Debug, Clone)]
pub struct SimpleLevelInfo;

impl SimpleLevelInfo {
    pub fn new() -> Self {
        SimpleLevelInfo
    }
}

impl LevelInfo for SimpleLevelInfo {
    type AccumulatorType = SimpleAccumulator;

    fn next_level(&self) -> Self {
        SimpleLevelInfo  // Always the same
    }
    
    fn create_accumulator(&self, element: &Box<&dyn Element>, element_info: ElementInfo) -> 
        Result<SimpleAccumulator, XmlDocumentError> 
    {
        Ok(SimpleAccumulator::new(element_info))
    }
}

/// Simple accumulator that just validates structure
pub struct SimpleAccumulator {
    element_name: String,
    element_lineno: LineNumber,
    has_subelement: bool,
}

impl SimpleAccumulator {
    pub fn new(element_info: ElementInfo) -> Self {
        SimpleAccumulator {
            element_name: element_info.owned_name.local_name.clone(),
            element_lineno: element_info.lineno,
            has_subelement: false,
        }
    }
}

impl Accumulator for SimpleAccumulator {
    type Value = ();

    fn start_subelement(&mut self, element_info: &ElementInfo) {
        self.has_subelement = true;
    }
    
    fn add_subelement(&mut self, _subelement: ()) {
        // Just validate that we're in the right state
    }
    
    fn end_subelement(&mut self) {
        self.has_subelement = false;
    }
    
    fn has_open_subelement(&self) -> bool {
        self.has_subelement
    }
    
    fn current_subelement_name(&self) -> &str {
        &self.element_name  // Simple implementation
    }
    
    fn finish(self) -> () {
        ()
    }
    
    fn element_name(&self) -> &str {
        &self.element_name
    }
    
    fn element_lineno(&self) -> LineNumber {
        self.element_lineno
    }
}
