//! 博弈专题

// 两人轮流从n颗石子中拿出1-m颗，游戏以玩家无法拿出最后一颗石子结束，求 winner
fn winner_of_stone_game(n: i32, m: i32) -> i32 {
    if n < m {
        return 0;
    }
    if n % (m + 1) == 0 {
        return 1;
    }
    0
}
