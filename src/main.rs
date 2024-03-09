mod array;
mod bit;
mod hash;
mod link;
mod topics;

use array::search::lis;
use array::sort::Sort;

fn main() {
    bit::print_binary(4);
    let mut arr = [5, 4, 3, 2, 1];
    let mut sort = Sort::from(&mut arr);
    sort.heap_sort();
    println!("{:?}", arr);
}
