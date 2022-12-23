pub struct Stack<T> {
    tail: usize,
    data: Vec<Option<T>>,
}

impl<T> Stack<T> {
    pub fn new(size: usize) -> Self {
        Stack {
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
        let prev = match self.tail {
            0 => 0,
            _ => {
                self.tail -= 1;
                self.tail
            }
        };

        self.data[prev].take()
    }

    fn push_element(&mut self, element: T) {
        if self.is_full() {
            self.data.push(Some(element)); // grow the vec by pushing an Some(element
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
    fn test_stack_pushing_works_as_expected() {
        let mut stack = Stack::new(10);

        stack.push(10);
        stack.push(20);
        stack.push(30);

        assert_eq!(stack.data, vec![Some(10), Some(20), Some(30)]);
    }

    #[test]
    fn test_stack_poping_one_element() {
        let mut stack = Stack {
            data: vec![Some(1)],
            tail: 1,
        };

        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.tail, 0);
    }
}
