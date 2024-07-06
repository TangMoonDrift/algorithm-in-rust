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
}
