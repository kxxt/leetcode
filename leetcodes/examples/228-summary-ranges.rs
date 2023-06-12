#![allow(unused)]
use handy_leetcode::{paste, test_eq, tests};
use leetcode_prelude::TreeNode;

struct Solution {}

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut res = vec![];
        if nums.len() == 0 {
            return res;
        }
        let len = nums.len();
        let mut begin = nums[0];
        for (idx, &val) in nums.iter().enumerate() {
            if idx == len - 1 {
                // Special case
                if val == begin {
                    res.push(val.to_string());
                } else {
                    res.push(format!("{begin}->{val}"));
                }
                break;
            }
            if val + 1 == nums[idx + 1] {
                // continuous
                continue;
            } else {
                // discrete
                if val == begin {
                    res.push(val.to_string());
                } else {
                    res.push(format!("{begin}->{val}"));
                }
                begin = nums[idx + 1];
            }
        }
        res
    }
}

tests! {
    test_eq!(1,
        Solution::summary_ranges(vec![0,1,2,4,5,7]),
        ["0->2", "4->5", "7"].into_iter().map(|x| x.to_string()).collect::<Vec<_>>()
    );
    test_eq!(1,
        Solution::summary_ranges(vec![0,2,3,4,6,8,9]),
        ["0","2->4","6","8->9"].into_iter().map(|x| x.to_string()).collect::<Vec<_>>()
    );
}

fn main() {}
