pub mod maximum_sum_of_subarrays;
pub mod search;
pub mod sort;

/**
 * 归并函数
 */
fn merge<T: PartialOrd + Copy>(array: &mut [T], l: usize, m: usize, r: usize) {
    let len = array.len();
    let first: T = array[0];
    let mut help: Vec<T> = vec![first; len];

    let mut a = l;
    let mut b = m + 1;
    let mut i = l;
    while a <= m && b <= r {
        if array[a] > array[b] {
            help[i] = array[b];
            b += 1;
        } else {
            help[i] = array[a];
            a += 1;
        }
        i += 1;
    }

    while a <= m {
        help[i] = array[a];
        i += 1;
        a += 1;
    }

    while b <= r {
        help[i] = array[b];
        i += 1;
        b += 1;
    }

    for i in l..=r {
        array[i] = help[i];
    }
}

fn heap_insert<T: PartialOrd + Copy>(array: &mut [T], index: usize) {
    let mut i = index as isize;
    let mut f_i = if i - 1 >= 0 { (i - 1) / 2 } else { 0 };
    while array[i as usize] > array[f_i as usize] {
        (array[i as usize], array[f_i as usize]) = (array[f_i as usize], array[i as usize]);
        i = f_i;
        f_i = if i - 1 >= 0 { (i - 1) / 2 } else { 0 };
    }
}

fn heapify<T: PartialOrd + Copy>(array: &mut [T], index: usize, size: usize) {
    let mut i = index;
    let mut l = i * 2 + 1;
    while l < size {
        let mut best = if l + 1 < size && array[l + 1] > array[l] {
            l + 1
        } else {
            l
        };
        best = if array[i] > array[best] { i } else { best };
        if best == index {
            break;
        }
        (array[i], array[best]) = (array[best], array[i]);
        i = best;
        l = i * 2 + 1;
    }
}

/**
 * 1371. 每个元音包含偶数次的最长子字符串
 * https://leetcode.cn/problems/find-the-longest-substring-containing-vowels-in-even-counts/description/
 */
pub fn find_the_longest_substring(s: String) -> i32 {
    let mut map = vec![-2; 32];
    map[0] = -1;
    let mut ans = 0;
    let mut status = 0;

    for (i, c) in s.chars().enumerate() {
        let m = f(c);
        if m != -1 {
            status ^= 1 << m;
        }
        if map[status] != -2 {
            ans = ans.max(i as i32 - map[status]);
        } else {
            map[status] = i as i32;
        }
    }

    fn f(c: char) -> i32 {
        match c {
            'a' => 0,
            'e' => 1,
            'i' => 2,
            'o' => 3,
            'u' => 4,
            _ => -1,
        }
    }

    ans
}
