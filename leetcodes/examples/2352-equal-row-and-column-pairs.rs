#![allow(unused)]
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

use handy_leetcode::{paste, test_eq, tests};
use leetcode_prelude::TreeNode;

struct Solution {}

impl Solution {
    // Brute force O(n^3)
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut cnt = 0;
        for i in 0..n {
            for j in 0..n {
                // Every cross point.
                let mut flag = true;
                for k in 0..n {
                    if grid[i][k] != grid[k][j] {
                        flag = false;
                        break;
                    }
                }
                cnt += flag as i32;
            }
        }
        cnt
    }
    // Transpose o(n^2 + n^3) with good locality in n^3 part
    pub fn equal_pairs_transpose(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut transposed = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                transposed[j][i] = grid[i][j];
            }
        }
        let mut cnt = 0;
        for i in 0..n {
            for j in 0..n {
                if grid[i] == transposed[j] {
                    cnt += 1;
                }
            }
        }
        cnt
    }

    // Hashmap approach
    pub fn equal_pairs_hashmap(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut map: HashMap<u64, (i32, i32)> = HashMap::new();
        let mut col = vec![0; n];
        for i in 0..n {
            let mut hasher = DefaultHasher::new();
            grid[i].hash(&mut hasher);
            let row_hash = hasher.finish();
            for j in 0..n {
                col[j] = grid[j][i];
            }
            let mut hasher = DefaultHasher::new();
            col.hash(&mut hasher);
            let col_hash = hasher.finish();
            if let Some((r, _)) = map.get_mut(&row_hash) {
                *r += 1;
            } else {
                map.insert(row_hash, (1, 0));
            }

            if let Some((_, c)) = map.get_mut(&col_hash) {
                *c += 1;
            } else {
                map.insert(col_hash, (0, 1));
            }
        }
        map.values().map(|(x, y)| x * y).sum()
    }
}

tests! {
    test_eq!(1,
        Solution::equal_pairs(vec![vec![3,2,1],vec![1,7,6],vec![2,7,7]]),
        1
    );
    test_eq!(2,
        Solution::equal_pairs(vec![vec![3,1,2,2],vec![1,4,4,5],vec![2,4,2,2],vec![2,4,2,2]]),
        3
    );
    test_eq!(3,
        Solution::equal_pairs_transpose(vec![vec![3,2,1],vec![1,7,6],vec![2,7,7]]),
        1
    );
    test_eq!(4,
        Solution::equal_pairs_transpose(vec![vec![3,1,2,2],vec![1,4,4,5],vec![2,4,2,2],vec![2,4,2,2]]),
        3
    );
    test_eq!(5,
        Solution::equal_pairs_hashmap(vec![vec![3,2,1],vec![1,7,6],vec![2,7,7]]),
        1
    );
    test_eq!(6,
        Solution::equal_pairs_hashmap(vec![vec![3,1,2,2],vec![1,4,4,5],vec![2,4,2,2],vec![2,4,2,2]]),
        3
    );
    test_eq!(7,
        Solution::equal_pairs_hashmap(vec![vec![13,13],vec![13,13]]),
        4
    );
}

fn main() {}
