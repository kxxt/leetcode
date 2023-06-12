use std::cell::RefCell;
use std::rc::Rc;

use leetcode_prelude::TreeNode;

type Node = Option<Rc<RefCell<TreeNode>>>;

pub fn is_symmetric_helper(l: &Node, r: &Node) -> bool {
    match (l, r) {
        (None, None) => true,
        (None, Some(_)) | (Some(_), None) => false,
        (Some(l), Some(r)) => {
            l.borrow().val == r.borrow().val
                && is_symmetric_helper(&l.borrow().left, &r.borrow().right)
                && is_symmetric_helper(&l.borrow().right, &r.borrow().left)
        }
    }
}

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    root.as_ref().map_or(true, |node| {
        is_symmetric_helper(&node.borrow().left, &node.borrow().right)
    })
}

#[cfg(test)]
mod test {
    use leetcode_prelude::btree;

    use super::*;

    #[test]
    fn case1() {
        assert!(is_symmetric(btree![1, 2, 2, 3, 4, 4, 3]));
    }

    #[test]
    fn case2() {
        assert!(!is_symmetric(btree![1, 2, 2, null, 3, null, 3]));
    }

    #[test]
    fn case3() {
        assert!(!is_symmetric(btree![1, 2, 3]));
    }
}

fn main() {}
