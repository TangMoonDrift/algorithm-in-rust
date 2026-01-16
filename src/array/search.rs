/**
 * 最长增长子序列
 * 算法核心解析
 * 1. 动态维护 tails 数组
 * - tails 数组存储当前最长递增子序列的末尾元素索引，通过二分查找确定新元素的插入位置。
 * - 若新元素大于所有末尾元素，则扩展序列；否则替换第一个不小于它的元素，保证后续潜力。
 * 2. 路径回溯 parents 数组
 * - 每个元素通过 parents 记录其前驱元素位置，用于最终重构完整序列。
 * - 从 tails 末尾元素开始，逆序回溯前驱节点，最后反转得到正确顺序。
 * 3. 时间复杂度优化
 * - 二分查找将时间复杂度从 O(n²) 优化至 O(n log n)，适合处理大规模数据。
 */
pub fn lis<T: Ord + Clone>(nums: &[T]) -> Vec<T> {
    if nums.is_empty() {
        return Vec::new();
    }

    let mut tails = Vec::new(); // 存储递增子序列的末尾元素索引
    let mut parents = vec![None; nums.len()]; // 记录每个元素的前驱索引

    // 构建 tails 和 parents 数组
    for (i, x) in nums.iter().enumerate() {
        // 二分查找插入位置
        let mut left = 0;
        let mut right = tails.len();
        while left < right {
            let mid = (left + right) / 2;
            if nums[tails[mid]] < *x {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        // 更新 tails 和 parents
        if left < tails.len() {
            tails[left] = i;
        } else {
            tails.push(i);
        }
        parents[i] = if left > 0 {
            Some(tails[left - 1])
        } else {
            None
        };
    }

    // 回溯重构最长递增子序列
    let mut current = *tails.last().unwrap();
    let mut lis_indices = vec![current];
    while let Some(parent) = parents[current] {
        lis_indices.push(parent);
        current = parent;
    }

    // 反转并映射到原数组元素
    lis_indices.reverse();
    lis_indices.into_iter().map(|i| nums[i].clone()).collect()
}

pub fn lis_optimized(arr: &[i32]) -> Vec<i32> {
    if arr.is_empty() {
        return Vec::new();
    }

    let n = arr.len();
    let mut dp = vec![0; n]; // dp[i] 表示长度为 i+1 的递增子序列的最小结尾值
    let mut prev = vec![0; n]; // prev[i] 表示 arr[i] 在最长递增子序列中的前一个元素的索引
    let mut length = 0; // 当前最长递增子序列的长度

    for &num in arr {
        let pos = dp.binary_search(&num).unwrap_or_else(|e| e); // 找到 num 应该插入的位置

        if pos == length {
            length += 1; // 如果 num 大于 dp 中的所有值，扩展长度
        }

        dp[pos] = num; // 更新 dp[pos] 为 num
        if pos > 0 {
            prev[pos] = dp[pos - 1]; // 更新前驱
        }
    }

    // 通过 dp 和 prev 数组回溯找到最长递增子序列
    let mut lis = Vec::new();
    let mut current = dp[length - 1];
    for i in (0..length).rev() {
        lis.push(current);
        current = prev[i];
    }

    lis.reverse();
    lis
}

/**
 * 寻找数组中大于等于目标值的最左位置，没有返回-1
 */
pub fn find_left<T: PartialOrd>(nums: &[T], target: T) -> i32 {
    let len = nums.len();
    if len == 0 {
        return -1;
    };
    let mut l = 0;
    let mut r = len - 1;
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
    let mut l = 1;
    let mut r = n - 2;
    let mut ans: i32 = -1;
    while l <= r {
        let m = l + (r - l) / 2;
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
        assert_eq!(lis(&[1, 2, 3, 1, 2, 3, 4, 5, 4, 3]), vec![1, 2, 3, 4, 5]);
        assert_eq!(lis(&[1, 3, 2, 1, 3, 4, 1, 5, 7, 9]), vec![1, 2, 3, 4, 5, 7]);
    }
}
