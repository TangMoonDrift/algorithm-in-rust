//! 滑动窗口专题

use std::collections::HashMap;

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

/**
 * https://leetcode.cn/problems/longest-substring-without-repeating-characters/description/
*/
pub fn length_of_longest_substring(s: String) -> i32 {
    let n = s.len();
    if n <= 1 {
        return n as i32;
    }

    let mut ans = i32::MIN;

    let mut map = HashMap::<char, usize>::new();

    let (mut l, mut r) = (0, 0);

    while r < n {
        if let Some(v) = map.get(&s.chars().nth(r).unwrap()) {
            l = (*v + 1).max(l);
        }
        ans = ans.max((r - l + 1) as i32);
        map.insert(s.chars().nth(r).unwrap(), r);
        r += 1;
    }

    ans
}

/**
 * https://leetcode.cn/problems/minimum-window-substring/
*/
// pub fn min_window(s: String, t: String) -> String {}

/**
 * https://leetcode.cn/problems/gas-station/
 */
pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let len = gas.len();
    let mut rest = vec![];
    let (mut l, mut r, mut sum, mut count) = (0, 0, 0, 0);

    for i in 0..len {
        rest.push(gas[i] - cost[i]);
    }

    while l < len {
        while sum >= 0 {
            if count == len {
                return l as i32;
            }
            r = (l + count) % len;
            count += 1;
            sum += rest[r];
        }
        count -= 1;
        sum -= rest[l];
        l += 1;
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_sub_array_len() {
        assert_eq!(min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(min_sub_array_len(4, vec![1, 4, 4]), 1);
        assert_eq!(min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    }

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    }
}
