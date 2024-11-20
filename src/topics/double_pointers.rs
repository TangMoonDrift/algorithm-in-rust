//! 双指针专题

/**
 * 922. 按奇偶排序数组 II
 * https://leetcode.cn/problems/sort-array-by-parity-ii/description/
 */
pub fn sort_array_by_parity_ii(mut nums: Vec<i32>) -> Vec<i32> {
    let mut even = 0;
    let mut odd = 1;
    let n = nums.len();

    while odd < n && even < n {
        if nums[n - 1] & 1 != 0 {
            nums.swap(odd, n - 1);
            odd += 2;
        } else {
            nums.swap(even, n - 1);
            even += 2;
        }
    }

    nums
}

/**
 * 287. 寻找重复数
 * https://leetcode.cn/problems/find-the-duplicate-number/description/
 */
pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return -1;
    }

    let mut slow = nums[0];
    let mut fast = nums[nums[0] as usize];

    while slow != fast {
        slow = nums[slow as usize];
        fast = nums[nums[fast as usize] as usize];
    }

    fast = 0;
    while slow != fast {
        slow = nums[slow as usize];
        fast = nums[fast as usize];
    }

    fast
}

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
    fn test_sort_array_by_parity_ii() {
        assert_eq!(sort_array_by_parity_ii(vec![4, 2, 5, 7]), vec![4, 5, 2, 7]);
        assert_eq!(sort_array_by_parity_ii(vec![2, 3]), vec![2, 3]);
    }

    #[test]
    fn test_find_duplicate() {
        assert_eq!(find_duplicate(vec![1, 3, 4, 2, 2]), 2);
        assert_eq!(find_duplicate(vec![3, 1, 3, 4, 2]), 3);
    }

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
