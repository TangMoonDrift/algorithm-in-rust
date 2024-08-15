pub struct MinHeap {
    heap: Vec<isize>,
}

impl MinHeap {
    pub fn new() -> Self {
        MinHeap { heap: Vec::new() }
    }

    pub fn insert(&mut self, value: isize) {
        self.heap.push(value);
        self.bubble_up(self.heap.len() - 1);
    }

    pub fn pop_min(&mut self) -> Option<isize> {
        if self.heap.is_empty() {
            return None;
        }
        if self.heap.len() == 1 {
            return Some(self.heap.pop()?);
        }

        let min = self.heap.swap_remove(0);
        self.sink_down(0);
        Some(min)
    }

    fn bubble_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent_index = (index - 1) / 2;
            if self.heap[parent_index] <= self.heap[index] {
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
            let mut smallest = index;

            if left_child_index < len && self.heap[left_child_index] < self.heap[smallest] {
                smallest = left_child_index;
            }
            if right_child_index < len && self.heap[right_child_index] < self.heap[smallest] {
                smallest = right_child_index;
            }
            if smallest == index {
                break;
            }
            self.heap.swap(index, smallest);
            index = smallest;
        }
    }
}
