use std::cmp::Reverse;
use std::collections::BinaryHeap;

/**
 * 求一组线段最大重合区间数
 */
pub fn max_cover(lines: &Vec<[i32; 2]>) -> usize {
    let n = lines.len();
    let mut data = lines.clone();
    let mut ans = 0;
    data.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        let start = data[i][0];
        let end = data[i][1];
        while !heap.is_empty() && heap.peek().unwrap() >= &Reverse(start) {
            heap.pop().unwrap();
        }
        heap.push(Reverse(end));
        ans = ans.max(heap.len());
    }
    ans
}
