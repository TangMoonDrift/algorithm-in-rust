use super::Graph;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
pub struct Path {
    node: usize,
    weight: usize,
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.weight.cmp(&other.weight) {
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less,
            Ordering::Equal => Ordering::Equal,
        }
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Into<Path> for (usize, usize) {
    fn into(self) -> Path {
        Path {
            node: self.0,
            weight: self.1,
        }
    }
}

pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    let mut heap: BinaryHeap<Path> = BinaryHeap::new();
    let edges = times
        .iter()
        .map(|edge| edge.iter().map(|&e| e as usize).collect())
        .collect();
    let n = n as usize;
    let k = k as usize;
    let mut visited = vec![false; n + 1];
    let mut distance = vec![usize::MAX; n + 1];
    distance[k] = 0;
    let graph = Graph::build(n, &edges, true);

    heap.push((k, 0).into());
    while !heap.is_empty() {
        match heap.pop() {
            Some(path) => {
                let Path { node, .. } = path;
                if visited[node] {
                    continue;
                }
                visited[node] = true;
                let neighbors = graph.collect_neighbors(node);
                for (next, weight) in neighbors {
                    if distance[node] + weight <= distance[next] && !visited[next] {
                        distance[next] = distance[node] + weight;
                        heap.push((next, distance[node] + weight).into());
                    }
                }
            }
            None => {
                break;
            }
        }
    }

    let mut ans = i32::MIN;
    for i in 1..=n {
        if distance[i] == usize::MAX {
            return -1;
        }
        ans = ans.max(distance[i] as i32);
    }
    ans
}
