pub struct Heap {
    pub data: Vec<i32>,
}

impl Heap {
    pub fn new() -> Self {
        Heap { data: vec![] }
    }

    pub fn insert(&mut self, element: i32) {
        self.data.push(element);
        let mut last_element_idx = self.data.len() - 1;

        while last_element_idx != 0 {
            let parent_idx = self.parent(last_element_idx);

            if self.data[last_element_idx] > self.data[parent_idx] {
                self.data.swap(last_element_idx, parent_idx)
            }

            last_element_idx = parent_idx;
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        match self.data.len() {
            n if n == 0 => None,
            n if n == 1 => self.data.pop(),
            _ => {
                let last_element_idx = self.data.len() - 1;
                self.data.swap(0, last_element_idx);
                let value = self.data.pop().unwrap();

                self.heapify(0);

                Some(value)
            }
        }
    }

    pub fn into_sorted_vec(&mut self) -> Vec<i32> {
        let mut vec = vec![];

        while self.data.len() > 0 {
            vec.push(self.pop().unwrap());
        }

        vec.reverse();

        vec
    }

    fn parent(&self, idx: usize) -> usize {
        idx / 2
    }

    fn left(&self, idx: usize) -> usize {
        2 * idx + 1
    }

    fn right(&self, idx: usize) -> usize {
        2 * idx + 2
    }

    fn heapify(&mut self, idx: usize) {
        let mut current = idx;

        loop {
            let left = self.left(current);
            let right = self.right(current);
            let size = self.data.len() - 1;

            if left > size || right > size {
                break;
            }

            let max = self.get_max_idx(left, right, current);

            if max != current {
                self.data.swap(current, max);

                current = max;
            } else {
                break;
            }
        }
    }

    fn get_max_idx(&self, left_idx: usize, right_idx: usize, idx: usize) -> usize {
        let max = self.data[left_idx]
            .max(self.data[right_idx])
            .max(self.data[idx]);

        match max {
            i if i == self.data[idx] => idx,
            i if i == self.data[right_idx] => right_idx,
            _ => left_idx,
        }
    }
}

impl From<Vec<i32>> for Heap {
    fn from(data: Vec<i32>) -> Self {
        let mut heap = Heap { data };

        for idx in (0..=heap.data.len() - 1).rev() {
            heap.heapify(idx)
        }

        heap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_element_in_an_non_empty_heap() {
        let mut heap = Heap {
            data: vec![50, 35, 20, 15, 10, 8, 6],
        };

        heap.insert(60);
        assert_eq!(heap.data, vec![60, 50, 20, 35, 10, 8, 6, 15]);
    }

    #[test]
    fn test_delete_element_from_complete_heap() {
        let mut heap = Heap {
            data: vec![50, 35, 20, 15, 10, 8, 6],
        };

        assert_eq!(heap.pop(), Some(50));
        assert_eq!(heap.data, vec![35, 15, 20, 6, 10, 8]);
    }

    #[test]
    fn test_from_vec_produces_max_heap() {
        let vec = vec![6, 50, 35, 20, 15, 10, 8];
        let heap = Heap::from(vec);

        assert_eq!(heap.data, vec![50, 20, 35, 6, 15, 10, 8]);
    }

    #[test]
    fn test_into_sorted_vec() {
        let mut heap = Heap {
            data: vec![50, 20, 35, 6, 15, 10, 8],
        };

        assert_eq!(heap.into_sorted_vec(), vec![6, 8, 10, 15, 20, 35, 50]);
    }
}
