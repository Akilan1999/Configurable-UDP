/// source: https://medium.com/@olcay.d.cabbas/implementing-a-simple-and-efficient-queue-in-rust-using-e33dc9cb9123

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}
pub struct Queue<T> {
    head: Option<Box<Node<T>>>,
    tail: *mut Node<T>,  // Raw pointer for efficient tail updates
}

impl<T> Queue<T> {
    /// Creates an empty queue
    pub fn new() -> Self {
        Queue {
            head: None,
            tail: std::ptr::null_mut(),
        }
    }

    /// Adds an element to the back of the queue
    pub fn enqueue(&mut self, data: T) {
        let mut new_node = Box::new(Node {
            data,
            next: None,
        });

        let raw_node: *mut _ = &mut *new_node;

        if self.tail.is_null() {
            self.head = Some(new_node);
        } else {
            unsafe {
                (*self.tail).next = Some(new_node);
            }
        }

        self.tail = raw_node;
    }

    /// Removes and returns the front element
    pub fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;

            if self.head.is_none() {
                self.tail = std::ptr::null_mut();
            }

            node.data
        })
    }

    /// Peeks at the front element without removing it
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    /// Checks if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}