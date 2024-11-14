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
 * https://leetcode.cn/problems/gas-station/
 */
pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let len = gas.len();
    let (mut l, mut sum, mut count) = (0, 0, 0);

    while l < len {
        while sum >= 0 {
            if count == len {
                return l as i32;
            }
            let r = (l + count) % len;
            count += 1;
            sum += gas[r] - cost[r];
        }
        count -= 1;
        sum -= gas[l] - cost[l];
        l += 1;
    }

    -1
}

/**
 * https://leetcode.cn/problems/subarrays-with-k-different-integers/description/
 */
pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
    fn nums_of_most_kinds(nums: &Vec<i32>, k: i32) -> i32 {
        let mut cnts = vec![0; 20001];
        let mut ans = 0;
        let mut collect = 0;
        let (mut l, mut r): (usize, usize) = (0, 0);

        while r < nums.len() {
            cnts[nums[r] as usize] += 1;
            if cnts[nums[r] as usize] == 1 {
                collect += 1;
            }

            while collect > k {
                cnts[nums[l] as usize] -= 1;
                if cnts[nums[l] as usize] == 0 {
                    collect -= 1;
                }
                l += 1;
            }

            ans += r - l + 1;
            r += 1;
        }

        ans as i32
    }

    nums_of_most_kinds(&nums, k) - nums_of_most_kinds(&nums, k - 1)
}

/**
 * https://leetcode.cn/problems/longest-substring-with-at-least-k-repeating-characters/description/
 */
pub fn longest_substring(s: String, k: i32) -> i32 {
    let nums: Vec<u8> = s.into_bytes();
    let n = nums.len();
    let mut cnts = vec![0; 256];
    let mut ans = 0;

    for required in 1..=26 {
        cnts.fill(0);
        let (mut l, mut r, mut collect, mut satisfy): (usize, usize, usize, usize) = (0, 0, 0, 0);
        while r < n {
            cnts[nums[r] as usize] += 1;
            if cnts[nums[r] as usize] == 1 {
                collect += 1;
            }

            if cnts[nums[r] as usize] == k {
                satisfy += 1;
            }

            while collect > required {
                if cnts[nums[l] as usize] == 1 {
                    collect -= 1;
                }

                if cnts[nums[l] as usize] == k {
                    satisfy -= 1;
                }

                cnts[nums[l] as usize] -= 1;
                l += 1;
            }

            if satisfy == required {
                ans = ans.max(r - l + 1);
            }
            r += 1;
        }
    }

    ans as i32
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

    #[test]
    fn test_can_complete_circuit() {
        assert_eq!(
            can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
        assert_eq!(can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]), -1);
    }
}
