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
 * 42. 接雨水
 * https://leetcode.cn/problems/trapping-rain-water/description/
 */
pub fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();
    if n <= 2 {
        return 0;
    }

    let mut l_max = height[0];
    let mut r_max = height[n - 1];
    let mut l = 1;
    let mut r = n - 2;
    let mut ans = 0;

    while l <= r {
        if l_max <= r_max {
            ans += 0.max(l_max - height[l]);
            l_max = l_max.max(height[l]);
            l += 1;
        } else {
            ans += 0.max(r_max - height[r]);
            r_max = r_max.max(height[r]);
            r -= 1;
        }
    }

    ans
}

/**
 * 881. 救生艇
 * https://leetcode.cn/problems/boats-to-save-people/description/
 */
pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
    people.sort_unstable();

    let (mut l, mut r, mut ans) = (0, people.len() - 1, 0);

    while l < r {
        if people[l] + people[r] > limit {
            r -= 1;
        } else {
            l += 1;
            r -= 1;
        }
        ans += 1;
    }

    ans + if l == r { 1 } else { 0 }
}

/**
 * 11. 盛最多水的容器
 * https://leetcode.cn/problems/container-with-most-water/description/
 */
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = height.len() - 1;
    let mut ans = 0;

    while l < r {
        ans = ans.max(height[l].min(height[r]) * (r - l) as i32);
        if height[l] < height[r] {
            l += 1;
        } else {
            r -= 1;
        }
    }

    ans
}

/**
 * 475. 供暖器
 * https://leetcode.cn/problems/heaters/
 */
pub fn find_radius(mut houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
    let n = houses.len();
    let m = heaters.len();
    houses.sort();
    heaters.sort();

    let mut l = 0;
    let mut r = 0;
    let mut ans = 0;

    let is_best = |i: usize, j: usize| -> bool {
        j == m - 1 || (houses[i] - heaters[j]).abs() < (houses[i] - heaters[j + 1]).abs()
    };

    while l < n {
        while !is_best(l, r) {
            r += 1;
        }
        let dis = (houses[l] - heaters[r]).abs();
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
    fn test_max_area() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(max_area(vec![1, 1]), 1);
    }

    #[test]
    fn test_first_missing_positive() {
        assert_eq!(first_missing_positive(vec![1, 2, 0]), 3);
        assert_eq!(first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    }
}
