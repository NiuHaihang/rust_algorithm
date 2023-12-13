mod coin_change;
mod find_target_sum_ways;
mod good_indices;
mod is_palindrome;
mod length_of_lis;
mod longest_consecutive;
mod palindrome;
mod two_sum;

use crate::coin_change::coin::coin_change;
use crate::find_target_sum_ways::target_sum::find_target_sum_ways;
use crate::good_indices::good::good_indices;
use crate::is_palindrome::palindrome::is_palindrome;
use crate::length_of_lis::lis::length_of_lis;
use crate::longest_consecutive::longest::longest_consecutive;
use crate::palindrome::palindrome::make_smallest_palindrome;
use crate::two_sum::sum::two_sum;

fn main() {
    let nums = vec![2];
    let target = 3;
    let res = coin_change(nums, target);
    println!("res is:{:?}", res);
}
