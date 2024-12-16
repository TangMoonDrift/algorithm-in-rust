//! 单调队列专题

/**
 * 239. 滑动窗口最大值
 * https://leetcode.cn/problems/sliding-window-maximum/description/
 */
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    if k == 1 {
        return nums;
    }

    let n = nums.len();
    let mut deque = [0; 100_001];
    let (mut head, mut tail) = (0, 0);
    let mut ans = vec![];

    for i in 0..(k as usize - 1) {
        while head < tail && nums[deque[tail - 1]] <= nums[i] {
            tail -= 1;
        }
        deque[tail] = i;
        tail += 1;
    }

    let mut l = 0;
    let mut r = (k as usize - 1);
    while r < n {
        while head < tail && nums[deque[tail - 1]] <= nums[r] {
            tail -= 1;
        }
        deque[tail] = r;
        tail += 1;
        ans.push(nums[deque[head]] as i32);

        if deque[head] == l {
            head += 1;
        }

        r += 1;
        l += 1;
    }

    ans
}
