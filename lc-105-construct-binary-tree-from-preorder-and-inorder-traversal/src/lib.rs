use leetcode_prelude::{btree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn build_tree_helper(preorder: &[i32], inorder: &[i32]) -> Node {
        if preorder.is_empty() || inorder.is_empty() {
            None
        } else {
            let val = *preorder.first().unwrap();
            // SAFETY: the caller guarantee val is in inorder
            let pos = inorder.iter().position(|v| *v == val).unwrap();
            Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: Self::build_tree_helper(&preorder[1..pos + 1], &inorder[..pos]),
                right: Self::build_tree_helper(&preorder[1 + pos..], &inorder[pos + 1..]),
            })))
        }
    }

    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Node {
        Self::build_tree_helper(&preorder, &inorder)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
            btree![3, 9, 20, null, null, 15, 7]
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::build_tree(vec![-1], vec![-1]),
            btree![-1]
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::build_tree(
                vec![1, 2, 4, 8, 5, 3, 6, 9, 7],
                vec![8, 4, 2, 5, 1, 6, 9, 3, 7]
            ),
            btree![1, 2, 3, 4, 5, 6, 7, 8, null, null, null, null, 9]
        );
    }
}
