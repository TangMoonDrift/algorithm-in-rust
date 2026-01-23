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
    let mut v = [2, 1, 4, 3, 6, 5, 8, 7, 10, 9];
    // 测试merge_sort（原有功能）
    let mut sort = array::sort::Sort::from(&mut v);
    sort.merge_sort();
    println!("Merge sort result: {:?}", sort.array);

    // 测试改进后的random_quick_sort
    let mut v2 = [5, 3, 8, 5, 2, 5, 1, 5, 9, 5];
    let mut sort2 = array::sort::Sort::from(&mut v2);
    sort2.random_quick_sort();
    println!("Random quick sort (Dutch flag) result: {:?}", sort2.array);
    println!("Is sorted: {}", sort2.is_sorted());

    // 测试全部元素相同的数组
    let mut v3 = [7, 7, 7, 7, 7, 7];
    let mut sort3 = array::sort::Sort::from(&mut v3);
    sort3.random_quick_sort();
    println!(
        "Random quick sort with all same elements: {:?}",
        sort3.array
    );
    println!("Is sorted: {}", sort3.is_sorted());
}
