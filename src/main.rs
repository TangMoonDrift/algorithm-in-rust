mod array;

use array::search::lis;
use array::sort::Sort;

fn main() {
    let mut nums = [0, 5, 7, 1, 9, 4, 6, 2, 8, 3];
    let len = nums.len();
    let res = lis(&nums);
    let mut sortable = Sort::from(&mut nums);
    sortable.select_sort();
    sortable.bubble_sort();
    sortable.insert_sort();
    sortable.merge_sort();
    sortable.merge_sort_recursion(0, len - 1);
    println!("{}", res);
    println!("{:?}", nums);
}
