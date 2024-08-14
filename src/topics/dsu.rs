//! 并查集专题

/**
 * 765. 情侣牵手
 * https://leetcode.cn/problems/couples-holding-hands/
 */
pub fn min_swaps_couples(row: Vec<i32>) -> i32 {
    let n = row.len();
    let mut sets = n / 2;
    let mut father = vec![0; sets];
    build(&mut father, sets);

    fn build(father: &mut Vec<usize>, sets: usize) {
        for i in 0..sets {
            father[i] = i;
        }
    }

    fn union(father: &mut Vec<usize>, x: usize, y: usize) {
        let fx = find(father, x);
        let fy = find(father, y);
        father[fx] = fy;
    }

    fn find(father: &mut Vec<usize>, i: usize) -> usize {
        if father[i] != i {
            father[i] = find(father, father[i]);
        }
        father[i]
    }

    for i in (0..n).step_by(2) {
        if find(&mut father, (row[i] / 2) as usize) != find(&mut father, (row[i + 1] / 2) as usize)
        {
            union(
                &mut father,
                (row[i] / 2) as usize,
                (row[i + 1] / 2) as usize,
            );
            sets -= 1;
        }
    }

    (n / 2 - sets) as i32
}
