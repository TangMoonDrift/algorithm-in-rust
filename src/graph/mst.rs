//! 最小生成树专题
use std::io::{self, BufRead};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Edge {
    source: usize,
    destination: usize,
    weight: i32,
}

impl Edge {
    fn new(source: usize, destination: usize, weight: i32) -> Self {
        Edge {
            source,
            destination,
            weight,
        }
    }
}

fn read_input() -> io::Result<(usize, Vec<Edge>)> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let first_line = lines.next().unwrap()?;
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    let mut edges = Vec::with_capacity(m);
    for _ in 0..m {
        let line = lines.next().unwrap()?;
        let mut parts = line.split_whitespace();
        let source: usize = parts.next().unwrap().parse().unwrap();
        let destination: usize = parts.next().unwrap().parse().unwrap();
        let weight: i32 = parts.next().unwrap().parse().unwrap();
        edges.push(Edge::new(source, destination, weight));
    }

    Ok((n, edges))
}
