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

/**
 * 410. 分割数组的最大值
 * https://leetcode.cn/problems/split-array-largest-sum/
 */
pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
    let check = |aim: i32| -> bool {
        let mut parts = 1;
        let mut sum = 0;
        for &num in &nums {
            if sum + num <= aim {
                sum += num;
            } else {
                // 新划分一段
                if parts == k {
                    return false;
                }
                parts += 1;
                sum = num;
            }
        }
        true
    };

    let mut right = nums.iter().sum::<i32>();
    let mut left = (*nums.iter().max().unwrap() - 1).max((right - 1) / k);
    while left + 1 < right {
        let mid = left + (right - left) / 2;
        if check(mid) {
            right = mid;
        } else {
            left = mid;
        }
    }
    right
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

/**
 * 2141. 同时运行 N 台电脑的最长时间
 * https://leetcode.cn/problems/maximum-running-time-of-n-computers/
 */
pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
    let mut batteries = batteries;
    batteries.sort_unstable();
    let mut sum = batteries.iter().fold(0, |acc, &x| acc + x as i64);
    let mut n = n as i64;
    let mut ans = 0;
    for &x in batteries.iter().rev() {
        if sum / n >= x as i64 {
            ans = sum / n;
            break;
        }
        sum -= x as i64;
        n -= 1;
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_eating_speed() {
        assert_eq!(min_eating_speed(vec![3, 6, 7, 11], 8), 4);
        assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
        assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
    }

    #[test]
    fn test_split_array() {
        assert_eq!(split_array(vec![7, 2, 5, 10, 8], 2), 15);
        assert_eq!(split_array(vec![1, 2, 3, 4, 5], 2), 9);
    }

    #[test]
    fn test_smallest_distance_pair() {
        assert_eq!(smallest_distance_pair(vec![1, 3, 1], 1), 0);
        assert_eq!(smallest_distance_pair(vec![1, 1, 1], 2), 0);
        assert_eq!(smallest_distance_pair(vec![1, 6, 1], 3), 5);
    }
}
