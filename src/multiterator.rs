/*
/*
 * Handle a stack of iterators
 */

struct Multiterator<I: Iterator, V: Value> {
    pub fn new() {
        stack = Self::new();
    }

    pub fn push(iterator) {
        stack.push(iterator);
    }

    pub fn next() -> Option<V>{
        loop {  
            top_of_stack = stack.len() - 1;

            if top_of_stack == 0) {
                return None;
            }

            match stack[top_of_stack].next() {
                None => stack.pop(),
                Some(next) => return next,
            }
        }
    }
}

// Owned Iterator
struct SchemaElementIterator<I: Iterator> {
    iterators: Vec<Box<dyn I>>, // A Vec of trait objects
}

impl Iterator for SchemaElementIterator<V> {
    type Item = Box<dyn SchemaElement>;

    pub fn push(iterator: I) {
        stack.push(iterator);
    }

    pub fn next() -> Option<V>{
        loop {  
            top_of_stack = stack.len() - 1;

            if top_of_stack == 0) {
                return None;
            }

            match stack[top_of_stack].next() {
                None => {
                    self.schema_elements.remove(0);
                },
                Some(next) => return Some(next),
            }
        }
    }

/*
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.schema_elements.len() {
            self.index += 1;
            Some(self.schema_elements.remove(0)) // Remove and return the first element
        } else {
            None
        }
    }
*/
}

/*
// Borrowed Iterator (Not really)
impl Iterator for SchemaElementIterator {
    type Item = Box<dyn SchemaElement>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.schema_elements.len() {
            self.index += 1;
            Some(self.schema_elements.remove(0)) // Remove and return the first element
        } else {
            None
        }
    }
}
*/
*/
