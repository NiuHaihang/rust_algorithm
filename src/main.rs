mod good_indices;
mod is_palindrome;
mod longest_consecutive;
mod two_sum;

use crate::good_indices::good::good_indices;
use crate::is_palindrome::palindrome::is_palindrome;
use crate::longest_consecutive::longest::longest_consecutive;
use crate::two_sum::sum::two_sum;

fn main() {
    let nums = vec![100, 4, 200, 1, 3, 2];
    let res = longest_consecutive(nums);
    println!("res is:{:?}", res);
}
