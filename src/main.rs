mod good_indices;
mod is_palindrome;
mod two_sum;

use crate::good_indices::good::good_indices;
use crate::is_palindrome::palindrome::is_palindrome;
use crate::two_sum::sum::two_sum;

fn main() {
    let nums = vec![2, 1, 1, 1, 3, 4, 1];
    let k = 2;
    let res = good_indices(nums, k);
    println!("res is:{:?}", res);
}
