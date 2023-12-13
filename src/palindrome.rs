pub mod palindrome {
    pub fn make_smallest_palindrome(s: String) -> String {
        let mut ans = String::new();
        for (x, y) in s.chars().zip(s.chars().rev()) {
            ans.push(x.min(y)) //较小值放入ans
        }
        ans
    }
    pub fn make_smallest_palindrome1(s: String) -> String {
        s.chars().zip(s.chars().rev()).fold(String::new(),|x,y|x+&y.0.min(y.1).to_string())
    }
}
