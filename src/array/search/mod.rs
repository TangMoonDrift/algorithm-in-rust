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
