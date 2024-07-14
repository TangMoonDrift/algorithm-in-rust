//! 滑动窗口专题

/**
 * https://leetcode.cn/problems/minimum-size-subarray-sum/description/
 */
pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ans = i32::MAX;

    let (mut l, mut r, mut sum) = (0, 0, 0);

    while r < n {
        sum += nums[r];
        while sum - nums[l] >= target {
            sum -= nums[l];
            l += 1;
        }
        if sum >= target {
            ans = ans.min((r - l + 1) as i32);
        }
        r += 1;
    }

    if ans == i32::MAX {
        0
    } else {
        ans
    }
}

#[test]
fn test_min_sub_array_len() {
    assert_eq!(min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    assert_eq!(min_sub_array_len(4, vec![1, 4, 4]), 1);
    assert_eq!(min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
}
