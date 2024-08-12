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
