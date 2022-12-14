use std::collections::VecDeque;
pub struct FixedQueue {
    pub length: usize,
    queue: VecDeque<String>,
}

impl FixedQueue {
    pub fn new(length: usize) -> Self {
        FixedQueue { length, queue: VecDeque::new() }
    }

    pub fn enqueue(&mut self, item: String) {
        if self.length == self.queue.len() {
            self.queue.pop_front();
        }

        self.queue.push_back(item);
    }

    pub fn dequeue(&mut self) -> Option<String> {
        self.queue.pop_front()
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }
}
