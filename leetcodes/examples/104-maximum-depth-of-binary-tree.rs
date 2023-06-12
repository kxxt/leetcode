use handy_leetcode::{paste, test_eq, tests};
use leetcode_prelude::{btree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

type Node = Option<Rc<RefCell<TreeNode>>>;

struct Solution;

impl Solution {
    fn max_depth_helper(node: &Node, cur: i32) -> i32 {
        node.as_ref().map_or(cur, |node| {
            Self::max_depth_helper(&node.borrow().left, cur + 1)
                .max(Self::max_depth_helper(&node.borrow().right, cur + 1))
        })
    }

    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::max_depth_helper(&root, 0)
    }
}

tests! {
    test_eq!(
        1,
        Solution::max_depth(btree![3, 9, 20, null, null, 15, 7]),
        3
    );
    test_eq!(2, Solution::max_depth(btree![1, null, 2]), 2);
}
