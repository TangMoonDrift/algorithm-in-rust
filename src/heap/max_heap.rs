pub struct MaxHeap {
    heap: Vec<isize>,
}

impl MaxHeap {
    pub fn new() -> Self {
        MaxHeap { heap: Vec::new() }
    }

    pub fn insert(&mut self, value: isize) {
        self.heap.push(value);
        self.bubble_up(self.heap.len() - 1);
    }

    pub fn pop_max(&mut self) -> Option<isize> {
        if self.heap.is_empty() {
            return None;
        }
        if self.heap.len() == 1 {
            return Some(self.heap.pop()?);
        }

        let max = self.heap.swap_remove(0);
        self.sink_down(0);
        Some(max)
    }

    fn bubble_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent_index = (index - 1) / 2;
            if self.heap[parent_index] >= self.heap[index] {
                break;
            }
            self.heap.swap(index, parent_index);
            index = parent_index;
        }
    }

    fn sink_down(&mut self, mut index: usize) {
        let len = self.heap.len();
        loop {
            let left_child_index = 2 * index + 1;
            let right_child_index = 2 * index + 2;
            let mut largest = index;

            if left_child_index < len && self.heap[left_child_index] > self.heap[largest] {
                largest = left_child_index;
            }
            if right_child_index < len && self.heap[right_child_index] > self.heap[largest] {
                largest = right_child_index;
            }
            if largest == index {
                break;
            }
            self.heap.swap(index, largest);
            index = largest;
        }
    }
}
