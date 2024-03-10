use core::num;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/**
 * 求一组线段最大重合区间数
 */
pub fn max_cover(lines: &Vec<[i32; 2]>) -> usize {
    let n = lines.len();
    let mut data = lines.clone();
    let mut ans = 0;
    data.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        let start = data[i][0];
        let end = data[i][1];
        while !heap.is_empty() && heap.peek().unwrap() >= &Reverse(start) {
            heap.pop().unwrap();
        }
        heap.push(Reverse(end));
        ans = ans.max(heap.len());
    }
    ans
}

/**
 * https://leetcode.cn/problems/minimum-operations-to-halve-array-sum/description/
 * 给你一个正整数数组nums
 * 每一次操作中，你可以从 nums 中选择 任意 一个数并将它减小到“恰好”一半（注意，在后续操作中你可以对减半过的数继续执行操作）
 * 请你返回将 nums 数组和“至少”减少一半的“最少”操作数
 */
#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct F64(f64);

impl Eq for F64 {}

impl Ord for F64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn halve_array(nums: Vec<i32>) -> i32 {
    let mut heap = BinaryHeap::new();
    let (mut sum, mut cur, mut ans) = (0.0, 0.0, 0);
    nums.into_iter().for_each(|x| {
        sum += x as f64;
        heap.push(F64(x as f64));
    });
    while cur < sum / 2.0 {
        let x = heap.pop().unwrap().0 / 2.0;
        cur += x;
        ans += 1;
        heap.push(F64(x));
    }
    ans
}
