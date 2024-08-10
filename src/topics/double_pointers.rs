//! 双指针专题

/**
 * 475. 供暖器
 * https://leetcode.cn/problems/heaters/
 */
pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
    let n = houses.len();
    let mut houses = houses.clone();
    houses.sort();
    let mut heaters = heaters.clone();
    heaters.sort();

    let mut l = 0;
    let mut r = 0;
    let mut ans = 0;

    fn absolute_value(x: i32) -> i32 {
        if x < 0 {
            -x
        } else {
            x
        }
    }

    fn is_best(houses: &Vec<i32>, heaters: &Vec<i32>, i: usize, j: usize) -> bool {
        j == heaters.len() - 1
            || absolute_value(houses[i] - heaters[j]) < absolute_value(houses[i] - heaters[j + 1])
    }

    while l < n {
        while !is_best(&houses, &heaters, l, r) {
            r += 1;
        }
        let dis = absolute_value(houses[l] - heaters[r]);
        ans = ans.max(dis);
        l += 1;
    }

    ans
}

/**
 * 41. 缺失的第一个正数
 * https://leetcode.cn/problems/first-missing-positive/
 */
pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let l = nums.len() as i32;
    for i in 0..nums.len() {
        // n = nums[i] 搬到 nums[n-1]
        let mut n = nums[i];
        while n > 0 && n <= l && nums[(n - 1) as usize] != n {
            std::mem::swap(&mut nums[(n - 1) as usize], &mut n);
        }
    }

    for i in 0..l {
        if nums[i as usize] != i + 1 {
            return i + 1;
        }
    }

    l + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_radius() {
        assert_eq!(find_radius(vec![1, 2, 3], vec![2]), 1);
        assert_eq!(find_radius(vec![1, 2, 3, 4], vec![1, 4]), 1);
        assert_eq!(find_radius(vec![1, 5], vec![2]), 3);
    }

    #[test]
    fn test_first_missing_positive() {
        assert_eq!(first_missing_positive(vec![1, 2, 0]), 3);
        assert_eq!(first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    }
}
