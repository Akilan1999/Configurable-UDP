/// source: https://github.com/gabrielrufino/structo/blob/main/src/queue/mod.rs

pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, item: T) {
        self.elements.push(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.elements.remove(0))
        }
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn peek(&self) -> Option<&T> {
        self.elements.first()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}