pub mod coin {
    use std::cmp::min;
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        // dp[i]=min(dp[i-coins[j]])+1
        let n = coins.len();
        let mut dp = vec![i32::MAX - 1; amount as usize + 1];
        dp[0] = 0;
        for i in 1..=amount as usize {
            for j in 0..n {
                let num = coins[j] as usize;
                if i >= num {
                    dp[i] = min(dp[i], dp[i - num] + 1);
                }
            }
        }
        if dp[amount as usize] > amount {
            return -1;
        }
        dp[amount as usize]
    }
}
