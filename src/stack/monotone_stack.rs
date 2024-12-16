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
    const MOD: usize = 10_0000_0007;
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

/**
 * 84. 柱状图中最大的矩形
 * https://leetcode.cn/problems/largest-rectangle-in-histogram/
 */
pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let n = heights.len();
    let mut stack: Vec<usize> = Vec::new();
    let mut ans: isize = 0;

    for i in 0..n {
        while !stack.is_empty() && heights[*stack.last().unwrap()] >= heights[i] {
            let cur_index = stack.pop().unwrap();
            let prev = if stack.is_empty() {
                i
            } else {
                i - stack.last().unwrap() - 1
            };
            ans = ans.max(prev as isize * heights[cur_index] as isize);
        }
        stack.push(i);
    }

    while !stack.is_empty() {
        let cur_index = stack.pop().unwrap();
        let prev = if stack.is_empty() {
            n
        } else {
            n - stack.last().unwrap() - 1
        };
        ans = ans.max(prev as isize * heights[cur_index] as isize);
    }

    ans as i32
}

/**
 * 962. 最大宽度坡
 * https://leetcode.cn/problems/maximum-width-ramp/
 */
pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
    let mut top_index = 1;
    let mut stack = [0; 500001];
    let n = nums.len();
    let mut ans = 0;

    for i in 1..n {
        if nums[stack[top_index - 1]] > nums[i] {
            stack[top_index] = i;
            top_index += 1;
        }
    }

    for j in (0..=(n - 1)).rev() {
        while top_index > 0 && nums[stack[top_index - 1]] <= nums[j] {
            top_index -= 1;
            ans = ans.max(j - stack[top_index]);
        }
    }

    ans as i32
}

/**
 * 316. 去除重复字母
 * https://leetcode.cn/problems/remove-duplicate-letters/
 */
pub fn remove_duplicate_letters(s: String) -> String {
    let chars_vec: Vec<char> = s.chars().collect();
    let mut top_index = 0;
    let mut stack: [char; 10001] = ['a'; 10001];
    let mut frequency = [0; 26];
    let mut enter = [false; 26];
    let start = 'a' as usize;
    let mut ans = String::new();

    for &c in &chars_vec {
        frequency[c as usize - start] += 1;
    }

    for &c in &chars_vec {
        let curr = c as usize;
        if !enter[curr - start] {
            while top_index > 0
                && stack[top_index - 1] as usize > curr
                && frequency[stack[top_index - 1] as usize - start] > 0
            {
                enter[stack[top_index - 1] as usize - start] = false;
                top_index -= 1;
            }
            stack[top_index] = c;
            top_index += 1;
            enter[curr - start] = true;
        }
        frequency[curr - start] -= 1;
    }

    for i in 0..top_index {
        ans += &stack[i].to_string();
    }

    ans
}

/**
 * 2289. 使数组按非递减顺序排列
 * https://leetcode.cn/problems/steps-to-make-array-non-decreasing/description/
 */
pub fn total_steps(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ans = 0;
    let mut cur_turns = 0;
    let mut top_index = 0;
    let mut stack = [[0, 0]; 100001];

    for i in (0..n).rev() {
        cur_turns = 0;
        while top_index > 0 && stack[top_index - 1][0] < nums[i] {
            top_index -= 1;
            cur_turns = (cur_turns + 1).max(stack[top_index][1]);
        }
        ans = ans.max(cur_turns);
        stack[top_index][0] = nums[i];
        stack[top_index][1] = cur_turns;
        top_index += 1;
    }

    ans
}
