use leetcode_prelude::{btree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    fn is_leaf(node: &Node) -> bool {
        node.as_ref().map_or(false, |n| {
            n.borrow().left.is_none() && n.borrow().right.is_none()
        })
    }

    fn helper(root: &Option<Rc<RefCell<TreeNode>>>, target_sum: i32, parent_is_leaf: bool) -> bool {
        match root {
            Some(node) => {
                let borrowed = node.borrow();
                let next_sum = target_sum - borrowed.val;
                let is_leaf = Self::is_leaf(&Some(node.clone()));
                Self::helper(&borrowed.left, next_sum, is_leaf)
                    || Self::helper(&borrowed.right, next_sum, is_leaf)
            }
            None => target_sum == 0 && parent_is_leaf,
        }
    }

    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        Self::helper(&root, target_sum, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::has_path_sum(
            btree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, null, 1],
            22
        ));
    }

    #[test]
    fn case2() {
        assert!(!Solution::has_path_sum(btree![1, 2, 3], 5));
    }

    #[test]
    fn case3() {
        assert!(!Solution::has_path_sum(btree![], 0));
    }

    #[test]
    fn case4() {
        assert!(!Solution::has_path_sum(btree![1, 2], 1));
    }

    #[test]
    fn case5() {
        assert!(Solution::has_path_sum(btree![1], 1));
    }
}
