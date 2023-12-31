pub mod greedy;
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
