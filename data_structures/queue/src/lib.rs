pub struct Queue<T> {
    head: usize,
    tail: usize,
    data: Vec<Option<T>>,
}

impl<T> Queue<T> {
    pub fn new(size: usize) -> Self {
        Queue {
            head: 0,
            tail: 0,
            data: Vec::with_capacity(size),
        }
    }

    pub fn push(&mut self, element: T) {
        self.push_element(element);

        if self.tail + 1 < self.data.capacity() {
            self.tail += 1;
        } else {
            self.tail = 0;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let value = self.data[self.head].take();

        if self.head + 1 < self.data.capacity() {
            self.head += 1;
        } else {
            self.head = 0;
        }

        value
    }

    fn push_element(&mut self, element: T) {
        if self.is_full() {
            self.data.push(Some(element));
        } else {
            self.data[self.tail] = Some(element);
        }
    }

    fn is_full(&self) -> bool {
        self.tail == self.data.len() && self.tail < self.data.capacity()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_queue_enque_works_as_expected() {
        let mut queue = Queue::new(10);

        queue.push(10);
        queue.push(20);

        assert_eq!(queue.tail, 2);
        assert_eq!(queue.head, 0);
    }

    #[test]
    pub fn test_queue_enque_boundaries_works_fine() {
        let mut queue = Queue::new(2);

        queue.push(10);
        queue.push(20);

        queue.push(20);

        assert_eq!(queue.head, 0);
    }

    #[test]
    pub fn test_queue_deque_works_as_expected() {
        let mut queue = Queue::new(10);

        queue.push(10);
        queue.push(20);

        assert_eq!(queue.pop(), Some(10));
        assert_eq!(queue.pop(), Some(20));
    }

    #[test]
    pub fn test_queue_boundary_works_fine() {
        let mut queue = Queue::new(1);

        queue.push(10);
        queue.push(20);

        assert_eq!(queue.pop(), Some(20));
    }
}
