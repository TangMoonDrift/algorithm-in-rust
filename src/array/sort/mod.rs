use rand::prelude::*;

pub struct Sort<'a, T>
where
    T: Ord + PartialOrd + Copy,
{
    array: &'a mut [T],
}

impl<'a, T> Sort<'a, T>
where
    T: Ord + PartialOrd + Copy,
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
                Self::merge(self.array, l, m, r);
                l = r + 1;
            }
            pace <<= 1;
        }
        self
    }

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
        Self::merge(self.array, l, m, r);
        self
    }

    fn merge(array: &mut [T], l: usize, m: usize, r: usize) {
        let len = array.len();
        let first: T = array[0];
        let mut help: Vec<T> = vec![first; len];

        let mut a = l;
        let mut b = m + 1;
        let mut i = l;
        while a <= m && b <= r {
            if array[a] > array[b] {
                help[i] = array[b];
                b += 1;
            } else {
                help[i] = array[a];
                a += 1;
            }
            i += 1;
        }

        while a <= m {
            help[i] = array[a];
            i += 1;
            a += 1;
        }

        while b <= r {
            help[i] = array[b];
            i += 1;
            b += 1;
        }

        for i in l..=r {
            array[i] = help[i];
        }
    }

    pub fn heap_sort(&mut self) -> &mut Self {
        let len = self.array.len();
        if len <= 1 {
            return self;
        }
        for i in 0..len {
            Self::heap_insert(&mut self.array, i);
        }
        let mut size = len;
        while size > 1 {
            size -= 1;
            self.swap(0, size);
            Self::heapify(&mut self.array, 0, size)
        }
        self
    }

    fn heap_insert(array: &mut [T], index: usize) {
        let mut i = index as isize;
        let mut f_i = if i - 1 >= 0 { (i - 1) / 2 } else { 0 };
        while array[i as usize] > array[f_i as usize] {
            (array[i as usize], array[f_i as usize]) = (array[f_i as usize], array[i as usize]);
            i = f_i;
            f_i = if i - 1 >= 0 { (i - 1) / 2 } else { 0 };
        }
    }

    fn heapify(array: &mut [T], index: usize, size: usize) {
        let mut i = index;
        let mut l = i * 2 + 1;
        while l < size {
            let mut best = if l + 1 < size && array[l + 1] > array[l] {
                l + 1
            } else {
                l
            };
            best = if array[i] > array[best] { i } else { best };
            if best == index {
                break;
            }
            (array[i], array[best]) = (array[best], array[i]);
            i = best;
            l = i * 2 + 1;
        }
    }
}

pub fn random_quick_sort<T: Ord + Copy>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }

    let mut rng = rand::rng();
    let pivot_idx = rng.random_range(0..array.len());
    array.swap(0, pivot_idx);

    let pivot_pos = partition(array);
    let (left, right) = array.split_at_mut(pivot_pos);
    random_quick_sort(left);
    random_quick_sort(&mut right[1..]);
}

fn partition<T: Ord + Copy>(array: &mut [T]) -> usize {
    let pivot = array[0];
    let mut left = 1;
    let mut right = array.len() - 1;

    while left <= right {
        while left <= right && array[left] <= pivot {
            left += 1;
        }
        while left <= right && array[right] >= pivot {
            right -= 1;
        }
        if left < right {
            array.swap(left, right);
        }
    }

    array.swap(0, right);
    right
}
