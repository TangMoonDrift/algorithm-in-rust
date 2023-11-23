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
