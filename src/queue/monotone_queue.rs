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
    let mut r = k as usize - 1;
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

/**
 * 862. 和至少为 K 的最短子数组
 * https://leetcode.cn/problems/shortest-subarray-with-sum-at-least-k/description/
 */
pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    if len == 0 {
        return -1;
    }

    let mut deque = vec![0; len + 1];
    let mut sum: Vec<i64> = vec![0; len + 1];
    let (mut h, mut t, mut ans) = (0, 0, i32::MAX);

    for i in 0..len {
        sum[i + 1] = nums[i] as i64 + sum[i];
    }

    for i in 0..=len {
        while h < t && sum[i] - sum[deque[h]] >= k as i64 {
            ans = ans.min(i as i32 - deque[h] as i32);
            h += 1;
        }

        while h < t && sum[deque[t - 1]] >= sum[i] {
            t -= 1;
        }

        deque[t] = i;
        t += 1;
    }

    if ans == i32::MAX {
        -1
    } else {
        ans
    }
}

/**
 * 1499. 满足不等式的最大值
 * https://leetcode.cn/problems/max-value-of-equation/description/
 */
pub fn find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32 {
    let n = points.len();
    let mut deque = vec![(0, 0); n + 1];
    let (mut head, mut tail, mut ans) = (0, 0, i32::MIN);

    for i in 0..n {
        let x = points[i][0];
        let y = points[i][1];

        while head < tail && deque[head].0 + k < x {
            head += 1;
        }

        if head < tail {
            ans = ans.max(y + deque[head].1 + (x - deque[head].0).abs());
        }

        while head < tail && deque[tail - 1].1 - deque[tail - 1].0 <= y - x {
            tail -= 1;
        }
        deque[tail].0 = x;
        deque[tail].1 = y;
        tail += 1;
    }

    ans
}

/**
 * 2071. 你可以安排的最多任务数目
 * https://leetcode.cn/problems/maximum-number-of-tasks-you-can-assign/description/
 */
pub fn max_task_assign(
    mut tasks: Vec<i32>,
    mut workers: Vec<i32>,
    pills: i32,
    strength: i32,
) -> i32 {
    let mut deque = [0; 50001];
    tasks.sort_unstable();
    workers.sort_unstable();

    let mut f =
        |t_l: usize, t_r: usize, w_l: usize, w_r: usize, pills: i32, strength: i32| -> bool {
            let (mut head, mut tail, mut count) = (0, 0, 0);
            let mut i = w_l;
            let mut j = t_l;
            while i <= w_r {
                while j <= t_r && tasks[j] <= workers[i] {
                    deque[tail] = j;
                    tail += 1;
                    j += 1;
                }
                if head < tail && tasks[deque[head]] <= workers[i] {
                    head += 1;
                } else {
                    while j <= t_r && tasks[j] <= workers[i] + strength {
                        deque[tail] = j;
                        tail += 1;
                        j += 1;
                    }

                    if head < tail {
                        count += 1;
                        tail -= 1;
                    } else {
                        return false;
                    }
                }
                i += 1;
            }

            return count <= pills;
        };

    let t_size = tasks.len();
    let w_size = workers.len();

    let mut ans = 0;
    let (mut l, mut r) = (0, t_size.min(w_size));
    while l <= r {
        let m = l + ((r - l) >> 1);
        if f(0, m - 1, w_size - m, w_size - 1, pills, strength) {
            ans = m;
            l = m + 1;
        } else {
            r = m - 1;
        }
    }

    ans as i32
}
