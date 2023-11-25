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
    // 获取可排序的可变数组引用
    pub fn from(array: &'a mut [T]) -> Self {
        Self { array }
    }

    fn swap(&mut self, a: usize, b: usize) {
        if a > self.array.len() || b > self.array.len() {
            return;
        }
        (self.array[a], self.array[b]) = (self.array[b], self.array[a]);
    }

    // 选择排序
    pub fn select_sort(&mut self) -> &mut Self {
        let len = self.array.len();
        if len <= 1 {
            return self;
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
        self
    }

    // 冒泡排序
    pub fn bubble_sort(&mut self) -> &mut Self {
        let len = self.array.len();
        if len <= 1 {
            return self;
        }
        for i in 0..(len - 1) {
            for j in 0..(len - 1 - i) {
                if self.array[j] > self.array[j + 1] {
                    self.swap(j, j + 1);
                }
            }
        }
        self
    }

    // 插入排序
    pub fn insert_sort(&mut self) -> &mut Self {
        let len = self.array.len();
        if len <= 1 {
            return self;
        }
        for i in 1..len {
            let mut j = i;
            while j > 0 && self.array[j - 1] > self.array[j] {
                self.swap(j - 1, j);
                j -= 1;
            }
        }
        self
    }

    // 归并排序
    pub fn merge_sort(&mut self) -> &mut Self {
        let len = self.array.len();
        if len <= 1 {
            return self;
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
        self
    }

    // 归并排序递归方式
    pub fn merge_sort_recursion(&mut self, l: usize, r: usize) -> &mut Self {
        let len = self.array.len();
        if len <= 1 {
            return self;
        }
        if l == r {
            return self;
        }
        let m = l + (r - l) / 2;
        self.merge_sort_recursion(l, m);
        self.merge_sort_recursion(m + 1, r);
        merge(self.array, l, m, r);
        self
    }
}
