mod array;
mod bit;
mod hash;
mod heap;
mod link;
mod topics;

use array::search::lis;
use array::sort::Sort;
use heap::max_cover;

fn main() {
    bit::print_binary(4);
    let mut arr = [5, 4, 3, 2, 1];
    let mut sort = Sort::from(&mut arr);
    sort.heap_sort();
    println!("{:?}", arr);

    let lines = vec![[1, 3], [1, 5], [2, 6], [3, 5], [5, 7], [4, 8]];
    let count = max_cover(&lines);
    println!("{}", count);
}
