use std::collections::VecDeque;
pub struct FixedQueue {
    pub length: usize,
    queue: VecDeque<String>,
}

impl FixedQueue {
    pub fn new(length: usize) -> Self {
        FixedQueue {
            length,
            queue: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, item: &str) {
        if self.length == self.queue.len() {
            self.queue.pop_front();
        }

        self.queue.push_back(item.to_string());
    }

    pub fn dequeue(&mut self) -> Option<String> {
        self.queue.pop_front()
    }
}
