//! 前缀树
use std::collections::HashSet;

pub struct Trie {
    pass: usize,
    end: usize,
    nexts: Vec<Option<Box<Trie>>>,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            pass: 0,
            end: 0,
            nexts: vec![None; 26],
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = self;
        node.pass += 1;
        for &c in word.as_bytes() {
            let i = (c - b'a') as usize;
            if node.nexts[i].is_none() {
                node.nexts[i] = Some(Box::new(Trie::new()));
                node.nexts[i].as_mut().unwrap().pass += 1;
                node = node.nexts[i].as_mut().unwrap();
            }
        }
        node.end += 1;
    }

    pub fn count_words_equal_to(&self, word: &str) -> usize {
        let mut node = self;
        for &c in word.as_bytes() {
            let i = (c - b'a') as usize;
            if node.nexts[i].is_none() {
                return 0;
            }
            node = node.nexts[i].as_ref().unwrap();
        }
        node.end as usize
    }

    pub fn search(&self, word: &str) -> bool {
        let mut node = self;
        for &c in word.as_bytes() {
            let i = (c - b'a') as usize;
            if node.nexts[i].is_none() {
                return false;
            }
            node = node.nexts[i].as_ref().unwrap();
        }
        node.end > 0
    }

    pub fn exist_words_starting_with(&self, prefix: &str) -> bool {
        let mut node = self;
        for &c in prefix.as_bytes() {
            let i = (c - b'a') as usize;
            if node.nexts[i].is_none() {
                return false;
            }
            node = node.nexts[i].as_ref().unwrap();
        }
        true
    }

    pub fn count_words_starting_with(&self, prefix: &str) -> usize {
        let mut node = self;
        for &c in prefix.as_bytes() {
            let i = (c - b'a') as usize;
            if node.nexts[i].is_none() {
                return 0;
            }
            node = node.nexts[i].as_ref().unwrap();
        }
        node.pass as usize
    }

    pub fn erase(&mut self, word: &str) {
        let count = self.count_words_equal_to(&word);
        if count == 0 {
            return;
        }

        let mut node = self;
        node.pass -= 1;
        for &c in word.as_bytes() {
            let i = (c - b'a') as usize;
            if node.nexts[i].as_ref().unwrap().pass - 1 == 0 {
                let next_node = node.nexts[i].as_mut().unwrap();
                node.nexts[i] = None;
                free(next_node);
                return;
            } else {
                node.nexts[i].as_mut().unwrap().pass -= 1;
            }
            node = node.nexts[i].as_mut().unwrap();
        }
    }

    pub fn reset(&mut self, word: &str) {
        let count = self.count_words_equal_to(&word);
        if count == 0 {
            return;
        }

        let mut node = self;
        node.pass -= count;
        for &c in word.as_bytes() {
            let i = (c - b'a') as usize;
            if node.nexts[i].as_ref().unwrap().pass - count == 0 {
                let next_node = node.nexts[i].as_mut().unwrap();
                node.nexts[i] = None;
                free(next_node);
                return;
            } else {
                node.nexts[i].as_mut().unwrap().pass -= count;
            }
            node = node.nexts[i].as_mut().unwrap();
        }
    }
}

/**
* https://leetcode.cn/problems/maximum-xor-of-two-numbers-in-an-array/description/
* 给你一个整数数组 nums ，返回 nums[i] XOR nums[j] 的最大运算结果，其中 0 ≤ i ≤ j < n 。
* 示例 1：

* 输入：nums = [3,10,5,25,2,8]
* 输出：28
* 解释：最大运算结果是 5 XOR 25 = 28.
* 示例 2：

* 输入：nums = [14,70,53,83,49,91,36,80,92,51,66,70]
* 输出：127

* 提示：
* 1 <= nums.length <= 2 * 105
* 0 <= nums[i] <= 231 - 1
*/
pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
    let mx = nums.iter().max().unwrap();
    let high_bit = 31 - mx.leading_zeros() as i32;

    let mut ans = 0;
    let mut mask = 0;
    let mut seen = HashSet::new();

    for i in (0..=high_bit).rev() {
        seen.clear();
        mask |= 1 << i;
        let new_ans = ans | (1 << i);
        for &x in &nums {
            let x = x & mask;
            if seen.contains(&(new_ans ^ x)) {
                ans = new_ans;
                break;
            }
            seen.insert(x);
        }
    }
    ans
}

/**
 * https://leetcode.cn/problems/subarray-sum-equals-k/
 */
pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    // 使用HashMap来存储从数组开始到当前位置的元素和及其出现的次数
    let mut map: HashMap<i32, i32> = HashMap::new();
    // 初始化答案为0
    let mut ans = 0;
    // 初始化sum为0的出现次数为1，为了处理从数组开始处的子数组
    map.insert(0, 1);
    // 初始化当前的元素和
    let mut sum: i32 = 0;

    // 遍历数组中的每个元素
    for num in &nums {
        // 更新当前的元素和，注意检查整数溢出
        sum = sum.checked_add(*num).unwrap_or(0);
        // 如果存在和为sum - k的前缀，更新答案
        if let Some(val) = map.get(&(sum - k)) {
            ans += *val;
        }
        // 更新HashMap中当前元素和的出现次数
        *map.entry(sum).or_insert(0) += 1;
    }

    // 返回答案
    ans
}

/**
 * https://leetcode.cn/problems/longest-well-performing-interval/
 */
pub fn longest_wpi(hours: Vec<i32>) -> i32 {
    if hours.is_empty() || hours.iter().all(|&x| x <= 8) {
        return 0;
    }

    let mut map = HashMap::new();
    let mut sum = 0;
    let mut ans = 0;

    for (index, x) in hours.iter().enumerate() {
        sum += if *x > 8 { 1 } else { -1 };

        if sum > 0 {
            ans = index + 1;
        } else if let Some(val) = map.get(&(sum - 1)) {
            ans = ans.max(index - val);
        }

        if !map.contains_key(&sum) {
            map.insert(sum, index);
        }
    }
    ans as i32
}

/**
 * https://leetcode.cn/problems/corporate-flight-bookings/
 */
pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
    let flights_cnt = n as usize;
    let mut ans = vec![0; flights_cnt + 2];

    for booking in bookings {
        ans[booking[0] as usize] += booking[2];
        ans[booking[1] as usize + 1] -= booking[2];
    }

    for i in 1..(flights_cnt + 2) {
        ans[i] += ans[i - 1];
    }

    ans[1..=flights_cnt].to_vec()
}

/**
 * https://leetcode.cn/problems/range-sum-query-2d-immutable/
 */
struct NumMatrix {
    sum: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut sum = vec![vec![0; n + 1]; m + 1];

        for (i, row) in matrix.iter().enumerate() {
            for (j, x) in row.iter().enumerate() {
                sum[i + 1][j + 1] = sum[i + 1][j] + sum[i][j + 1] - sum[i][j] + x;
            }
        }

        Self { sum }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let r1 = row1 as usize;
        let c1 = col1 as usize;
        let r2 = row2 as usize + 1;
        let c2 = col2 as usize + 1;
        self.sum[r2][c2] - self.sum[r2][c1] - self.sum[r1][c2] + self.sum[r1][c1]
    }
}

pub fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len() as i32;
    let m = grid[0].len() as i32;
    let mut tries = grid.clone();
    let tries = bulid(n as usize, m as usize, tries);

    if sum(&tries, 0, 0, n - 1, m - 1) == 0 {
        return 0;
    }

    let mut ans: i32 = 1;

    for a in 0..n {
        for b in 0..m {
            // (a,b)所有左上角点
            //     (c,d)更大边长的右下角点，k是当前尝试的边长
            let (mut c, mut d, mut k) = (a + ans, b + ans, ans + 1);
            while c < n && d < m {
                let outer_area = sum(&tries, a, b, c, d);
                let inner_area = sum(&tries, a + 1, b + 1, c - 1, d - 1);
                let perimeter = (k - 1) << 2;
                if outer_area - inner_area == perimeter {
                    ans = k;
                }
                c += 1;
                d += 1;
                k += 1;
            }
        }
    }

    fn bulid(n: usize, m: usize, mut tries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for i in 0..n {
            for j in 0..m {
                tries[i][j] += get(i as i32 - 1, j as i32, &tries)
                    + get(i as i32, j as i32 - 1, &tries)
                    - get(i as i32 - 1, j as i32 - 1, &tries);
            }
        }
        tries
    }

    fn get(i: i32, j: i32, grid: &Vec<Vec<i32>>) -> i32 {
        if i < 0 || j < 0 {
            0
        } else {
            grid[i as usize][j as usize]
        }
    }

    fn sum(g: &Vec<Vec<i32>>, a: i32, b: i32, c: i32, d: i32) -> i32 {
        if a > c {
            return 0;
        } else {
            return g[c as usize][d as usize] - get(c, b - 1, g) - get(a - 1, d, g)
                + get(a - 1, b - 1, g);
        }
    }

    ans * ans
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_maximum_xor() {
        assert_eq!(find_maximum_xor(vec![3, 10, 5, 25, 2, 8]), 28);
        assert_eq!(
            find_maximum_xor(vec![14, 70, 53, 83, 49, 91, 36, 80, 92, 51, 66, 70]),
            127
        );
    }

    #[test]
    fn test_subarray_sum() {
        assert_eq!(subarray_sum(vec![1, 1, 1], 2), 2);
        assert_eq!(subarray_sum(vec![1, 2, 3], 3), 2);
    }
}
