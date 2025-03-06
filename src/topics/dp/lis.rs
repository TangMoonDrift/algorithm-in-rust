//! 最长递增子序列专题

/**
 * 300. 最长递增子序列
 * https://leetcode.cn/problems/longest-increasing-subsequence/description/
 */
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ends = vec![0; n];
    let mut len: usize = 0;

    let search = |len: usize, target: i32, ends: &mut Vec<i32>| -> i32 {
        let (mut l, mut r) = (0, len);
        let mut index_of = -1;
        while l < r {
            let mid = l + (r - l) / 2;
            if target > ends[mid] {
                l = mid + 1;
            } else {
                index_of = mid as i32;
                r = mid;
            }
        }
        index_of
    };

    for &num in &nums {
        let index_of = search(len, num, &mut ends);
        if index_of == -1 {
            ends[len] = num;
            len += 1;
        } else {
            ends[index_of as usize] = num;
        }
    }

    len as i32
}

pub fn length_of_lis_ii(nums: Vec<i32>) -> i32 {
    let mut ends = vec![];

    for &num in &nums {
        if let Err(index) = ends.binary_search(&num) {
            if index == ends.len() {
                ends.push(num); // 如果 num 大于 ends 中的所有值，追加到 ends
            } else {
                ends[index] = num; // 否则，替换 ends[index] 为 num
            }
        }
    }

    ends.len() as i32
}

pub fn length_of_lis_iii(nums: Vec<i32>) -> i32 {
    let mut ends = vec![];

    for &num in &nums {
        match ends.binary_search(&num) {
            Ok(_) => (),
            Err(index) => {
                if index == ends.len() {
                    ends.push(num);
                } else if let Some(old) = ends.get_mut(index) {
                    *old = num;
                }
            }
        }
    }

    ends.len() as i32
}

/**
 * 354. 俄罗斯套娃信封问题
 * https://leetcode.cn/problems/russian-doll-envelopes/description/
 */
pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
    envelopes.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));
    let mut ends = vec![];

    for envelope in &envelopes {
        let height = envelope[1];
        // if height > *ends.last().unwrap_or(&0) {
        //     ends.push(height);
        // } else if let Err(index) = ends.binary_search(&height) {
        //     ends[index] = height;
        // }

        // match ends.binary_search(&height) {
        //     Ok(_) => {}
        //     Err(index) => {
        //         if index == ends.len() {
        //             ends.push(height);
        //         } else if let Some(old) = ends.get_mut(index) {
        //             *old = height;
        //         }
        //     }
        // }

        if let Err(index) = ends.binary_search(&height) {
            if index == ends.len() {
                ends.push(height);
            } else {
                ends[index] = height;
            }
        }
    }

    ends.len() as i32
}

/**
 * 2111. 使数组 K 递增的最少操作次数
 * https://leetcode.cn/problems/minimum-operations-to-make-the-array-k-increasing/description/
 */
pub fn k_increasing(arr: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let n = arr.len();
    let mut ans = 0;
    let mut ends = vec![];

    for i in 0..k {
        ends.clear();
        let nums: Vec<i32> = (i..n).step_by(k).map(|index| arr[index]).collect();
        ends.push(nums[0]);
        for &num in nums.iter().skip(1) {
            if num >= *ends.last().unwrap() {
                ends.push(num);
            } else {
                let (mut l, mut r) = (0, ends.len());
                while l < r {
                    let m = l + (r - l) / 2;
                    if ends[m] <= num {
                        l = m + 1;
                    } else {
                        r = m;
                    }
                }
                ends[l] = num;
            }
        }
        ans += (nums.len() - ends.len()) as i32;
    }

    ans
}

pub fn k_increasing_ii(arr: Vec<i32>, k: i32) -> i32 {
    const N: usize = 100_001;
    let mut nums = vec![0; N];
    let mut ends = vec![0; N];
    let k = k as usize;

    let n = arr.len();
    let mut ans = 0;

    for i in 0..k {
        let mut size = 0;
        for j in (i..n).step_by(k) {
            nums[size] = arr[j];
            size += 1;
        }
        ans += size - length_of_no_decreasing(&nums, size, &mut ends);
    }

    fn length_of_no_decreasing(nums: &Vec<i32>, size: usize, ends: &mut Vec<i32>) -> usize {
        let mut len = 0;
        for i in 0..size {
            let find = binary_search(len, nums[i], ends);
            if find == -1 {
                ends[len] = nums[i];
                len += 1;
            } else {
                ends[find as usize] = nums[i];
            }
        }
        len
    }

    fn binary_search(len: usize, target: i32, ends: &mut Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = len;
        let mut ans = -1;
        while l < r {
            let m = (l + r) / 2;
            if target < ends[m] {
                ans = m as i32;
                r = m;
            } else {
                l = m + 1;
            }
        }
        ans
    }

    ans as i32
}

/**
 * 646. 最长数对链
 * https://leetcode.cn/problems/maximum-length-of-pair-chain/description/
 */
pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
    pairs.sort_unstable_by(|a, b| a[1].cmp(&b[1]));
    let (mut pre, mut ans) = (pairs[0][1], 1);

    pairs.iter().skip(1).for_each(|pair| {
        if pair[0] > pre {
            pre = pair[1];
            ans += 1;
        }
    });

    ans
}

pub fn find_longest_chain_ii(mut pairs: Vec<Vec<i32>>) -> i32 {
    let mut ends = vec![];
    pairs.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
    ends.push(pairs[0][1]);

    for pair in pairs.iter().skip(1) {
        let (start, end) = (pair[0], pair[1]);
        if start > *ends.last().unwrap() {
            ends.push(end);
        } else if let Err(index) = ends.binary_search(&start) {
            ends[index] = end.min(ends[index]);
        }
    }

    ends.len() as i32
}
