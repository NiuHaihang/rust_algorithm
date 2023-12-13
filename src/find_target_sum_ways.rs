pub mod target_sum {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        // 正数之和为p，负号数字之和为n p-n = target  p+n = sum(nums)=target+2n
        let mut sum = nums.iter().sum::<i32>();
        // sum = target +2n  n = (sum-target)/2 = newTarget
        // 转换成从n个数字中取数字和为newTarget的取法
        if (sum - target) < 0 || (sum - target) % 2 == 1 {
            return 0;
        }
        let c: usize = ((sum - target) / 2).try_into().unwrap();
        let n = nums.len();

        let mut dp = vec![vec![0; c + 1]; n + 1];

        dp[0][0] = 1;

        for i in 1..=n {
            for j in 0..=c {
                let num = nums[i - 1] as usize;
                dp[i][j] = dp[i - 1][j];
                if j >= num {
                    dp[i][j] += dp[i - 1][j - num];
                }
            }
        }
        dp[n][c]
    }
}
