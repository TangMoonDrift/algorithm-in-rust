//! 二分查找专题

/**
 * 875. 爱吃香蕉的珂珂
 * https://leetcode.cn/problems/koko-eating-bananas/
 */
pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    if piles.is_empty() {
        panic!("The piles vector cannot be empty.");
    }

    let mut l = 1;
    let mut r = *piles.iter().max().expect("Vector is not empty");

    // 更具描述性的函数名
    let is_feasible = |speed: i32| -> bool {
        let mut hours_needed = 0;
        for pile in &piles {
            hours_needed += ((*pile + speed - 1) / speed);
            if hours_needed > h {
                return false;
            }
        }
        true
    };

    while l <= r {
        let mid = l + (r - l) / 2; // 更改变量名以避免混淆
        if is_feasible(mid) {
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }

    l // 最终答案为 l，因为当 l > r 时跳出循环
}
/**719. 找出第 k 小的数对距离**
 * https://leetcode.cn/problems/find-k-th-smallest-pair-distance/
 */
pub fn smallest_distance_pair(nums: Vec<i32>, k: i32) -> i32 {
    let mut left = 0;
    let mut mid = 0;
    let mut right = *nums.iter().max().unwrap() - *nums.iter().min().unwrap();
    let mut answer = 0;
    let mut nums = nums;
    nums.sort();
    let n = nums.len();

    let is_valid = |target: i32| -> bool {
        let mut count = 0;
        let mut start = 0;
        let mut end = 0;
        while start < n {
            while end + 1 < n && nums[end + 1] - nums[start] <= target {
                end += 1;
            }
            count += end - start;
            start += 1;
        }
        count >= k as usize
    };

    while left <= right {
        mid = left + (right - left) / 2;
        if is_valid(mid) {
            answer = mid;
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    answer
}
