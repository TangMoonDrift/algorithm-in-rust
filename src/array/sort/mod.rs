use super::merge;

pub struct Sort<'a, T>
where
    T: PartialOrd + Copy,
{
    array: &'a mut [T],
}

impl<'a, T> Sort<'a, T>
where
    T: PartialOrd + Copy,
{
    pub fn from(array: &'a mut [T]) -> Self {
        Self { array }
    }

    fn swap(&mut self, a: usize, b: usize) {
        if a > self.array.len() || b > self.array.len() {
            return;
        }
        (self.array[a], self.array[b]) = (self.array[b], self.array[a]);
    }

    pub fn select_sort(&mut self) {
        let len = self.array.len();
        if len <= 1 {
            return;
        }
        for i in 0..(len - 1) {
            let mut min = i;
            for j in (i + 1)..len {
                if self.array[j] < self.array[min] {
                    min = j;
                }
            }
            self.swap(min, i);
        }
    }

    pub fn bubble_sort(&mut self) {
        let len = self.array.len();
        if len <= 1 {
            return;
        }
        for i in 0..(len - 1) {
            for j in 0..(len - 1 - i) {
                if self.array[j] > self.array[j + 1] {
                    self.swap(j, j + 1);
                }
            }
        }
    }

    pub fn insert_sort(&mut self) {
        let len = self.array.len();
        if len <= 1 {
            return;
        }
        for i in 1..len {
            let mut j = i;
            while j > 0 && self.array[j - 1] > self.array[j] {
                self.swap(j - 1, j);
                j -= 1;
            }
        }
    }

    // 归并排序
    pub fn merge_sort(&mut self) {
        let len = self.array.len();
        if len <= 1 {
            return;
        }
        let mut pace: usize = 1;
        while pace < len {
            let mut l: usize = 0;
            while l < len {
                let m = l + pace - 1;
                if (m + 1) >= len {
                    break;
                }
                let r = (l + (pace << 1) - 1).min(len - 1);
                merge(self.array, l, m, r);
                l = r + 1;
            }
            pace <<= 1;
        }
    }

    pub fn merge_sort_recursion(&mut self, l: usize, r: usize) {
        let len = self.array.len();
        if len <= 1 {
            return;
        }
        if l == r {
            return;
        }
        let m = l + (r - l) / 2;
        self.merge_sort_recursion(l, m);
        self.merge_sort_recursion(m + 1, r);
        merge(self.array, l, m, r);
    }
}
