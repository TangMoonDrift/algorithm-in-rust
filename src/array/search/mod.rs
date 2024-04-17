use std::usize;

/**
 * 最长增长子序列
 */
pub fn lis<T: PartialOrd>(nums: &[T]) -> usize {
    let len: usize = nums.len();
    let mut f = vec![1; len];
    let mut ans = 1;

    for i in 2..len {
        for j in 1..i {
            if nums[i] > nums[j] {
                f[i] = f[i].max(f[j] + 1);
            }
            ans = ans.max(f[i]);
        }
    }

    return ans;
}

/**
 * 寻找数组中大于等于目标值的最左位置，没有返回-1
 */
pub fn find_left<T: PartialOrd>(nums: &[T], target: T) -> i32 {
    let len: usize = nums.len();
    if len == 0 {
        return -1;
    };
    let mut l: usize = 0;
    let mut r: usize = len - 1;
    let mut ans: i32 = -1;
    while l <= r {
        let m = l + (r - l) / 2;
        if nums[m] >= target {
            r = m - 1;
            ans = m as i32;
        } else {
            l = m + 1;
        }
    }
    ans
}

/**
 * https://leetcode.cn/problems/find-peak-element/description/
 * 峰值元素是指其值严格大于左右相邻值的元素。
 * 给你一个整数数组 nums，找到峰值元素并返回其索引。数组可能包含多个峰值，在这种情况下，返回 任何一个峰值 所在位置即可。
 * 你可以假设 nums[-1] = nums[n] = -∞ 。
 * 你必须实现时间复杂度为 O(log n) 的算法来解决此问题。
 */
pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 1 {
        return 0;
    }
    if nums[0] > nums[1] {
        return 0;
    }
    if nums[n - 1] > nums[n - 2] {
        return (n - 1) as i32;
    }
    let mut l: usize = 1;
    let mut r: usize = n - 2;
    let mut ans: i32 = -1;
    while l <= r {
        let m: usize = l + (r - l) / 2;
        if nums[m - 1] > nums[m] {
            r = m - 1;
        } else if nums[m + 1] > nums[m] {
            l = m + 1;
        } else {
            ans = m as i32;
            break;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_lis() {
        assert_eq!(lis(&[1, 2, 3, 1, 2, 3, 4, 5, 4, 3]), 5);
        assert_eq!(lis(&[1, 3, 2, 1, 3, 4, 1, 5, 7, 9]), 6);
    }
}
