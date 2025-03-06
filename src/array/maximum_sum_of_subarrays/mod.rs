//! 子数组最大累加和专题

/**
 * 53. 最大子数组和
 * https://leetcode.cn/problems/maximum-subarray/description/
 */
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut pre = nums[0];
    let mut ans = nums[0];
    for i in 1..n {
        pre = nums[i].max(nums[i] + pre);
        ans = ans.max(pre);
    }

    ans
}

/**
 * 198. 打家劫舍
 * https://leetcode.cn/problems/house-robber/description/
 */
pub fn rob(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    match n {
        1 => nums[0],
        2 => nums[0].max(nums[1]),
        _ => {
            let mut pre_pre = nums[0];
            let mut pre = nums[0].max(nums[1]);

            for i in 2..n {
                let curr = pre.max(pre_pre.max(0) + nums[i]);
                pre_pre = pre;
                pre = curr;
            }
            pre
        }
    }
}

/**
 * 918. 环形子数组的最大和
 * https://leetcode.cn/problems/maximum-sum-circular-subarray/description/
 */
pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let (mut all, mut min_sum, mut max_sum) = (nums[0], nums[0], nums[0]);
    let (mut min_pre, mut max_pre) = (nums[0], nums[0]);

    for i in 1..n {
        all += nums[i];

        // 更新最小子数组和
        min_pre = nums[i].min(nums[i] + min_pre);
        min_sum = min_sum.min(min_pre);

        // 更新最大子数组和
        max_pre = nums[i].max(nums[i] + max_pre);
        max_sum = max_sum.max(max_pre);
    }

    // 如果总和等于最小子数组和，说明所有元素都是负数，直接返回最大子数组和
    if all == min_sum {
        max_sum
    } else {
        // 返回最大子数组和或总和减去最小子数组和的较大值
        max_sum.max(all - min_sum)
    }
}

/**
 * 213. 打家劫舍 II
 * https://leetcode.cn/problems/house-robber-ii/description/
 */
pub fn rob_ii(nums: Vec<i32>) -> i32 {
    let n = nums.len();

    // Helper function to calculate the maximum amount that can be robbed
    // within a non-circular range [start, end].
    fn rob_range(nums: &[i32], start: usize, end: usize) -> i32 {
        let mut pre_pre = 0;
        let mut pre = 0;

        for i in start..=end {
            let curr = pre.max(pre_pre.max(0) + nums[i]);
            pre_pre = pre;
            pre = curr;
        }
        pre
    }

    // Handle edge cases for small arrays
    match n {
        0 => 0,       // If the array is empty, return 0
        1 => nums[0], // If there's only one house, rob it
        _ => {
            // For circular array, we have two cases:
            // 1. Do not rob the first house.
            // 2. Do not rob the last house.
            rob_range(&nums, 1, n - 1).max(nums[0] + rob_range(&nums, 2, n - 2))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sub_array() {
        assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_sub_array(vec![1]), 1);
        assert_eq!(max_sub_array(vec![5, -3, 5]), 10);
    }

    #[test]
    fn test_rob() {
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(rob(vec![2, 7, 9, 3, 1]), 12);
        assert_eq!(rob(vec![1, 2, 3, 1, 1, 3]), 6);
    }

    #[test]
    fn test_max_subarray_sum_circular() {
        assert_eq!(
            max_subarray_sum_circular(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(max_subarray_sum_circular(vec![1]), 1);
        assert_eq!(max_subarray_sum_circular(vec![5, -3, 5]), 10);
    }

    #[test]
    fn test_rob_ii() {
        assert_eq!(rob_ii(vec![2, 3, 2]), 3);
        assert_eq!(rob_ii(vec![1, 2, 3, 1]), 4);
        assert_eq!(rob_ii(vec![1, 2, 3]), 3);
    }
}
