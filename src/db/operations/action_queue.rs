use std::collections::VecDeque;
use super::user_action::UserAction;

pub struct ActionQueue {
    queue: VecDeque<UserAction>,
}

impl ActionQueue {
    pub fn new() -> Self {
        ActionQueue {
            queue: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, action: UserAction) {
        self.queue.push_back(action);
    }

    pub fn dequeue(&mut self) -> Option<UserAction> {
        self.queue.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }
}