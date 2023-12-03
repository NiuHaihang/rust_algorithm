pub mod palindrome {
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut v: Vec<i32> = Vec::new();
        loop {
            let a = x % 10;
            v.push(a);
            x = x / 10;
            if x == 0 {
                break;
            }
        }
        let mut l = 0;
        let mut r = v.len() - 1;
        while l < r {
            if v[l] != v[r] {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }
}
