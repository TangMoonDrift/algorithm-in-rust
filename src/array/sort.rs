use rand::prelude::*;

/// Sort结构体提供了多种排序算法的实现
///
/// # 泛型约束
/// - `T: Ord + Copy`：元素类型必须实现`Ord` trait（可比较）和`Copy` trait（可复制）
pub struct Sort<'a, T>
where
    T: Ord + Copy,
{
    pub array: &'a mut [T],
}

impl<'a, T> Sort<'a, T>
where
    T: Ord + Copy,
{
    /// 从可变切片创建Sort实例
    ///
    /// # 参数
    /// - `array`：要排序的可变切片
    ///
    /// # 返回值
    /// 返回包含该切片的Sort实例
    pub fn from(array: &'a mut [T]) -> Self {
        Self { array }
    }

    /// 检查数组是否已经按升序排序
    ///
    /// # 返回值
    /// 如果数组已排序返回true，否则返回false
    pub fn is_sorted(&self) -> bool {
        let len = self.array.len();
        if len <= 1 {
            return true;
        }

        for i in 0..(self.array.len() - 1) {
            if self.array[i] > self.array[i + 1] {
                return false;
            }
        }
        true
    }

    /// 返回排序后的数组切片
    ///
    /// # 返回值
    /// 返回包含排序后数据的切片引用
    pub fn as_slice(&self) -> &[T] {
        self.array
    }

    /// 默认排序方法（使用快速排序）
    ///
    /// 这是一个便捷方法，内部使用`random_quick_sort`
    ///
    /// # 返回值
    /// 返回self，支持方法链调用
    pub fn sort(&mut self) -> &mut Self {
        self.random_quick_sort()
    }

    /// 交换数组中两个位置的元素
    ///
    /// # 参数
    /// - `a`：第一个元素的索引
    /// - `b`：第二个元素的索引
    ///
    /// # 注意
    /// 如果索引超出数组范围，该方法会直接返回，不执行任何操作
    fn swap(&mut self, a: usize, b: usize) {
        if a >= self.array.len() || b >= self.array.len() {
            return;
        }
        (self.array[a], self.array[b]) = (self.array[b], self.array[a]);
    }

    /// 选择排序算法
    ///
    /// 选择排序的时间复杂度为O(n^2)，空间复杂度为O(1)
    ///
    /// # 返回值
    /// 返回self，支持方法链调用
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

    /// 冒泡排序算法
    ///
    /// 冒泡排序的时间复杂度为O(n^2)，空间复杂度为O(1)
    ///
    /// # 返回值
    /// 返回self，支持方法链调用
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

    /// 插入排序算法
    ///
    /// 插入排序的时间复杂度为O(n^2)，空间复杂度为O(1)
    ///
    /// # 返回值
    /// 返回self，支持方法链调用
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

    /// 归并排序算法（迭代实现）
    ///
    /// 归并排序的时间复杂度为O(n log n)，空间复杂度为O(n)
    ///
    /// # 返回值
    /// 返回self，支持方法链调用
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

    /// 归并排序算法（递归实现，需要指定范围）
    ///
    /// 归并排序的时间复杂度为O(n log n)，空间复杂度为O(n)
    ///
    /// # 参数
    /// - `l`：排序范围的左边界（包含）
    /// - `r`：排序范围的右边界（包含）
    ///
    /// # 返回值
    /// 返回self，支持方法链调用
    pub fn merge_sort_recursion(&mut self, l: usize, r: usize) -> &mut Self {
        let len = self.array.len();
        if len <= 1 {
            return self;
        }
        let r = r.min(len - 1);
        if l >= r {
            return self;
        }
        let m = l + (r - l) / 2;
        self.merge_sort_recursion(l, m);
        self.merge_sort_recursion(m + 1, r);
        Self::merge(self.array, l, m, r);
        self
    }

    /// 归并排序算法（递归实现，默认排序整个数组）
    ///
    /// 归并排序的时间复杂度为O(n log n)，空间复杂度为O(n)
    ///
    /// # 返回值
    /// 返回self，支持方法链调用
    pub fn merge_sort_recursive(&mut self) -> &mut Self {
        let len = self.array.len();
        if len <= 1 {
            return self;
        }
        self.merge_sort_recursion(0, len - 1)
    }

    /// 归并排序的辅助方法，用于合并两个有序子数组
    ///
    /// # 参数
    /// - `array`：要合并的数组
    /// - `l`：左子数组的左边界
    /// - `m`：左子数组的右边界，也是右子数组的左边界减一
    /// - `r`：右子数组的右边界
    fn merge(array: &mut [T], l: usize, m: usize, r: usize) {
        let mut help: Vec<T> = Vec::with_capacity(r - l + 1);

        let mut a = l;
        let mut b = m + 1;
        while a <= m && b <= r {
            if array[a] > array[b] {
                help.push(array[b]);
                b += 1;
            } else {
                help.push(array[a]);
                a += 1;
            }
        }

        while a <= m {
            help.push(array[a]);
            a += 1;
        }

        while b <= r {
            help.push(array[b]);
            b += 1;
        }

        for (i, &val) in help.iter().enumerate() {
            array[l + i] = val;
        }
    }

    /// 堆排序算法
    ///
    /// 堆排序的时间复杂度为O(n log n)，空间复杂度为O(1)
    ///
    /// # 返回值
    /// 返回self，支持方法链调用
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

    /// 堆排序的辅助方法，用于将元素插入堆中并维护堆结构
    ///
    /// # 参数
    /// - `array`：要操作的数组
    /// - `index`：要插入的元素索引
    fn heap_insert(array: &mut [T], index: usize) {
        let mut i = index as isize;
        let mut f_i = if i - 1 >= 0 { (i - 1) / 2 } else { 0 };
        while array[i as usize] > array[f_i as usize] {
            (array[i as usize], array[f_i as usize]) = (array[f_i as usize], array[i as usize]);
            i = f_i;
            f_i = if i - 1 >= 0 { (i - 1) / 2 } else { 0 };
        }
    }

    /// 堆排序的辅助方法，用于维护堆结构
    ///
    /// # 参数
    /// - `array`：要操作的数组
    /// - `index`：要维护的堆顶索引
    /// - `size`：堆的大小
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
            if best == i {
                break;
            }
            (array[i], array[best]) = (array[best], array[i]);
            i = best;
            l = i * 2 + 1;
        }
    }
}

impl<'a, T> Sort<'a, T>
where
    T: Ord + Copy,
{
    /// 随机快速排序算法
    ///
    /// 快速排序的平均时间复杂度为O(n log n)，最坏时间复杂度为O(n^2)，空间复杂度为O(log n)
    /// 随机选择pivot可以避免最坏情况
    ///
    /// # 返回值
    /// 返回self，支持方法链调用
    pub fn random_quick_sort(&mut self) -> &mut Self {
        let len = self.array.len();
        if len <= 1 {
            return self;
        }
        Self::random_quick_sort_helper(self.array);
        self
    }

    /// 快速排序的辅助方法，用于递归排序
    ///
    /// # 参数
    /// - `array`：要排序的数组
    fn random_quick_sort_helper(array: &mut [T]) {
        let len = array.len();
        if len <= 1 {
            return;
        }

        let mut rng = rand::rng();
        let pivot_idx = rng.random_range(0..len);
        array.swap(0, pivot_idx);

        let (low, high) = Self::partition(array);

        // 仅对小于pivot的区域递归排序
        if low > 0 {
            Self::random_quick_sort_helper(&mut array[..low]);
        }

        // 仅对大于pivot的区域递归排序
        if high < len - 1 {
            Self::random_quick_sort_helper(&mut array[high + 1..]);
        }
    }

    /// 快速排序的辅助方法，使用荷兰国旗法分割数组
    ///
    /// 荷兰国旗法将数组分为三部分：小于pivot、等于pivot、大于pivot
    ///
    /// # 参数
    /// - `array`：要分割的数组
    ///
    /// # 返回值
    /// 返回等于pivot区域的左右边界 (low, high)
    fn partition(array: &mut [T]) -> (usize, usize) {
        let pivot = array[0];
        let mut low = 0;
        let mut current = 0;
        let mut high = array.len() - 1;

        while current <= high {
            if array[current] < pivot {
                array.swap(current, low);
                low += 1;
                current += 1;
            } else if array[current] > pivot {
                array.swap(current, high);
                high -= 1;
            } else {
                current += 1;
            }
        }

        (low, high)
    }
}
