struct Solution;
include!("solutions_1.rs");

fn main() {
    println!("{:?}", Solution::two_sum(vec![2, 7, 11, 15], 9));
}