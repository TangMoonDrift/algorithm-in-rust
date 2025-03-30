mod array;
mod bit;
mod collections;
mod graph;
mod hash;
mod heap;
mod linked_list;
mod number_theoretic;
mod queue;
mod stack;
mod topics;
mod tree;

fn main() {
    let mut array = [2, 1, 4, 3, 6, 5, 8, 7, 10, 9];
    array::sort::random_quick_sort(&mut array);
    println!("{:?}", array);
}
