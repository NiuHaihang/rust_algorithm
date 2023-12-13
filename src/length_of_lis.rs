pub mod lis {
    use std::cmp::max;
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return 1;
        }
        let mut dp = vec![1; n];

        let mut i = 0;
        let mut j = 0;
        for i in 0..n {
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = max(dp[i], dp[j] + 1);
                }
            }
        }
        let mut res = 0;
        for &num in dp.iter() {
            res = max(num, res);
        }
        res
    }
}
