pub mod good {
    pub fn good_indices(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let mut left = vec![1; n];
        let mut right = vec![1; n];

        // println!("left is:{:?},right is:{:?}",left,right);
      
        for i in 1..n {  
            if nums[i] <= nums[i-1] {  
                left[i] = left[i-1] + 1;  
            }  
            if nums[n-i-1] <= nums[n-i] {  
                right[n-i-1] = right[n-i] + 1;  
            }  
        }  
      
        let mut ans:Vec<i32> = Vec::new();
        let l:usize = k.try_into().unwrap(); 
        // let l = k as usize;
        for i in l..(n-l) {  
            if left[(i-1) as usize] >= k && right[(i+1) as usize] >= k {  
               ans.push(i.try_into().unwrap());  
               // ans.push(i as i32);
            }  
        }  
        ans  
    }

    pub fn good_indices_bing(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let mut left = vec![1; n];
        let mut right = vec![1; n];
        let mut ans = Vec::new();
    
        for i in 1..n {
            if nums[i] <= nums[i - 1] {
                left[i] = left[i - 1] + 1;
            }
            if nums[n - i - 1] <= nums[n - i] {
                right[n - i - 1] = right[n - i] + 1;
            }
        }
    
        for i in (k as usize)..(n - k as usize) {
            if left[i - 1] >= k && right[i + 1] >= k {
                ans.push(i as i32);
            }
        }
    
        ans
    }
    
}
