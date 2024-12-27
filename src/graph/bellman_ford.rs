/**
 * 787. K 站中转内最便宜的航班
 * https://leetcode.cn/problems/cheapest-flights-within-k-stops/description/
 */
pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    let n = n as usize;
    let start = src as usize;
    let target = dst as usize;
    let k = k as usize;

    let mut current_turn = vec![i32::MAX; n];
    current_turn[start] = 0;

    for _ in 0..=k {
        let mut next_turn = current_turn.clone();
        for flight in &flights {
            let from = flight[0] as usize;
            let to = flight[1] as usize;
            let price = flight[2];
            if current_turn[from] != i32::MAX {
                next_turn[to] = next_turn[to].min(current_turn[from] + price);
            }
        }
        current_turn = next_turn;
    }

    if current_turn[target] == i32::MAX {
        -1
    } else {
        current_turn[target]
    }
}
