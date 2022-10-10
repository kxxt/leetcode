use leetcode_prelude::{btree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    fn build_tree_helper<'a>(inorder: &[i32], postorder: &'a [i32]) -> (Node, &'a [i32]) {
        if inorder.is_empty() || postorder.is_empty() {
            return (None, postorder);
        }
        let val = *postorder.last().unwrap();
        // Safety: the data guarantees val in inorder
        // O(N) find root
        let pos = inorder.iter().position(|v| *v == val).unwrap();

        let (inorder_left, inorder_right) = inorder.split_at(pos);
        let inorder_right = &inorder_right[1..]; // ignore the root.
        let (right, new_post_order) =
            Self::build_tree_helper(inorder_right, &postorder[..postorder.len() - 1]);
        let (left, ret_post_order) = Self::build_tree_helper(inorder_left, new_post_order);
        let node = Some(Rc::new(RefCell::new(TreeNode { val, left, right })));
        (node, ret_post_order)
    }

    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree_helper(&inorder, &postorder).0
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]),
            btree![3, 9, 20, null, null, 15, 7]
        );
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::build_tree(vec![-1], vec![-1]), btree![-1]);
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::build_tree(
                vec![8, 4, 2, 5, 1, 6, 9, 3, 7],
                vec![8, 4, 5, 2, 9, 6, 7, 3, 1]
            ),
            btree![1, 2, 3, 4, 5, 6, 7, 8, null, null, null, null, 9]
        );
    }
}
