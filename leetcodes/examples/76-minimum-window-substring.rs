#![allow(unused)]

use handy_leetcode::{paste, test_eq, tests};
use std::{collections::HashMap, ops::Range};

struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let bytes = s.as_bytes();
        let mut map: HashMap<u8, usize> = HashMap::new();
        let mut result: Option<Range<usize>> = None;
        let cnts = {
            let mut cnts: HashMap<u8, usize> = HashMap::new();
            for &ch in t.as_bytes() {
                *cnts.entry(ch).or_default() += 1;
            }
            cnts
        };
        let desired = cnts.len();
        let mut current = 0usize;
        let mut left = 0;
        for right in 0..s.len() {
            let ch = bytes[right];
            *map.entry(ch).or_default() += 1;
            if cnts
                .get(&ch)
                .map_or(false, |&v| *map.get(&ch).unwrap() == v)
            {
                // Strict == needed because we only want to inc current once per alphabet
                current += 1;
            }
            while left <= right && current == desired {
                let new_result = left..(right + 1);
                result = Some(result.map_or(new_result.clone(), |old| {
                    if old.len() > new_result.len() {
                        new_result
                    } else {
                        old
                    }
                }));
                let ch = bytes[left];
                if cnts
                    .get(&ch)
                    .map_or(false, |&v| *map.get(&ch).unwrap() == v)
                {
                    current -= 1;
                }
                *map.get_mut(&ch).unwrap() -= 1;
                left += 1;
            }
        }
        result.map_or(String::new(), |range| s[range].to_string())
    }
}

tests! {
    test_eq!(0, Solution::min_window("ADOBECODEBANC".to_string(),"ABC".to_string()), "BANC");
    test_eq!(1, Solution::min_window("a".to_string(),"aa".to_string()), "");
    test_eq!(2, Solution::min_window("a".to_string(),"a".to_string()), "a");
}


fn main() {

}