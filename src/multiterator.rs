/*
 * Handle a stack of iterators
 */

struct Multiterator<I: Iterator, V: Value> {
    stack: Vec<I>;

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

