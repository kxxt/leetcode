#![allow(unused)]
use handy_leetcode::{paste, test_eq, tests};
use leetcode_prelude::TreeNode;

struct Solution {}

impl Solution {
    pub fn func(a: i32, b: i32) -> i32 {
        return a + b;
    }
}

tests! {
    test_eq!(1,
        Solution::func(1, 2),
        3
    );
}

fn main() {}
