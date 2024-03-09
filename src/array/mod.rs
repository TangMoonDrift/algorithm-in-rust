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

fn heap_insert<T: PartialOrd + Copy>(array: &mut [T], index: usize) {
    let mut i = index;
    while array[i] > array[(i - 1) / 2] {
        (array[i], array[(i - 1) / 2]) = (array[(i - 1) / 2], array[i]);
        i = (i - 1) / 2;
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
