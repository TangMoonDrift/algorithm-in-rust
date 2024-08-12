//! 单调栈专题

/**
 * 739. 每日温度
 * https://leetcode.cn/problems/daily-temperatures/
 */
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let n = temperatures.len();
    let mut stack = vec![0; n];
    let mut ans = vec![0; n];
    let mut top_index = 0;

    for i in 0..n {
        while top_index > 0 && temperatures[stack[top_index - 1]] < temperatures[i] {
            top_index -= 1;
            let curr: usize = stack[top_index] as usize;
            ans[curr] = (i - curr) as i32;
        }
        stack[top_index] = i;
        top_index += 1;
    }

    ans
}

/**
 * 907. 子数组的最小值之和
 * https://leetcode.cn/problems/sum-of-subarray-minimums/
 */
pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
    const MOD: usize = 1000000007;
    let n = arr.len();
    let mut stack = vec![0; n];
    let mut ans: i64 = 0;
    let mut top_index = 0;

    for i in 0..n {
        while top_index > 0 && arr[stack[top_index - 1]] >= arr[i] {
            top_index -= 1;
            let cur_index = stack[top_index] as usize;
            let pre = if top_index == 0 {
                cur_index + 1
            } else {
                cur_index - stack[top_index - 1]
            };
            let post = i - cur_index;
            ans += (pre * post * arr[cur_index] as usize) as i64;
            ans %= MOD as i64;
        }
        stack[top_index] = i;
        top_index += 1;
    }

    while top_index > 0 {
        top_index -= 1;
        let cur_index = stack[top_index] as usize;
        let pre = if top_index == 0 {
            cur_index + 1
        } else {
            cur_index - stack[top_index - 1]
        };
        let post = n - cur_index;
        ans += (pre * post * arr[cur_index] as usize) as i64;
        ans %= MOD as i64;
    }

    ans as i32
}
