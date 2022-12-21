use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Node<'a, T> {
    pub data: T,
    next: Option<Rc<RefCell<&'a mut Node<'a, T>>>>,
    prev: Option<Rc<RefCell<&'a mut Node<'a, T>>>>,
}

pub struct LinkedList<'a, T> {
    head: Option<Rc<RefCell<&'a mut Node<'a, T>>>>,
    tail: Option<Rc<RefCell<&'a mut Node<'a, T>>>>,
    size: usize,
}

impl<'a, T> Node<'a, T> {
    fn new(data: T) -> Self {
        Node {
            data,
            next: None,
            prev: None,
        }
    }
}

impl<'a, T> LinkedList<'a, T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn push_front(&mut self, node: &'a mut Node<'a, T>) {
        match self.size {
            0 => {
                let pointer = Rc::new(RefCell::new(node));

                self.head = Some(Rc::clone(&pointer));
                self.tail = Some(Rc::clone(&pointer));
                self.size = 1;
            }
            _ => {
                let head = self.head.as_ref().unwrap();

                node.next = Some(Rc::clone(head));
                node.prev = None;

                let pointer = Rc::new(RefCell::new(node));
                unsafe {
                    (*head.as_ptr()).prev = Some(Rc::clone(&pointer));
                }

                self.head = Some(Rc::clone(&pointer));
                self.size += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use super::*;

    #[test]
    fn test_linked_list_push_front() {
        let mut node = Node::new(15);
        let mut node1 = Node::new(16);

        let mut linked_list: LinkedList<i32> = LinkedList::new();

        linked_list.push_front(&mut node);
        linked_list.push_front(&mut node1);

        let node = linked_list.head.unwrap();

        println!("{:#?}", node);
        // node.as_ptr().data

        println!("works");
    }
}
