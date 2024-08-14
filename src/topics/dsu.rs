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

/**
 * 839. 相似字符串组
 * https://leetcode.cn/problems/similar-string-groups/
 */
pub fn num_similar_groups(strs: Vec<String>) -> i32 {
    let n = strs.len();
    let m = strs[0].len();
    let mut sets = n;
    let mut father = vec![0; sets];
    build(&mut father, n);

    fn build(father: &mut Vec<usize>, sets: usize) {
        for i in 0..sets {
            father[i] = i;
        }
    }

    fn find(father: &mut Vec<usize>, i: usize) -> usize {
        if father[i] != i {
            father[i] = find(father, father[i]);
        }
        father[i]
    }

    fn union(father: &mut Vec<usize>, x: usize, y: usize) {
        let fx = find(father, x);
        let fy = find(father, y);
        father[fx] = fy;
    }

    fn ok(a: &str, b: &str, k: usize) -> bool {
        let mut diff = 0;
        for (char_in_a, char_in_b) in a.bytes().zip(b.bytes()) {
            if char_in_a != char_in_b {
                diff += 1;
                if diff > 2 {
                    return false;
                }
            }
        }
        true
    }

    for i in 0..n {
        for j in (i + 1)..n {
            if find(&mut father, i) != find(&mut father, j) {
                if ok(&strs[i], &strs[j], m) {
                    union(&mut father, i, j);
                    sets -= 1;
                }
            }
        }
    }

    sets as i32
}

/**
 * 200. 岛屿数量
 * https://leetcode.cn/problems/number-of-islands/
 */
pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut father = vec![0; 100001];
    let mut sets = build(&grid, &mut father, n, m);

    fn build(grid: &Vec<Vec<char>>, father: &mut Vec<usize>, n: usize, m: usize) -> usize {
        let mut sets = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == '1' {
                    let index = serialize(m, i, j);
                    father[index] = index;
                    sets += 1;
                }
            }
        }

        sets
    }

    fn serialize(cols: usize, i: usize, j: usize) -> usize {
        i * cols + j
    }

    fn find(father: &mut Vec<usize>, i: usize) -> usize {
        if father[i] != i {
            father[i] = find(father, father[i]);
        }
        father[i]
    }

    fn union(father: &mut Vec<usize>, x: usize, y: usize) {
        let fx = find(father, x);
        let fy = find(father, y);
        father[fx] = fy;
    }

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == '1' {
                let index_1 = serialize(m, i, j);
                let index_2 = serialize(m, i - 1, j);
                let index_3 = serialize(m, i, j - 1);
                if i > 0 && grid[i - 1][j] == '1' {
                    if find(&mut father, index_1) != find(&mut father, index_2) {
                        union(&mut father, index_1, index_2);
                        sets -= 1;
                    }
                }

                if j > 0 && grid[i][j - 1] == '1' {
                    if find(&mut father, index_1) != find(&mut father, index_3) {
                        union(&mut father, index_1, index_3);
                        sets -= 1;
                    }
                }
            }
        }
    }

    sets as i32
}
