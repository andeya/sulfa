//! Balanced Binary Tree
//! Definition:
//!  The absolute value of the height difference between the left and right subtrees of each node of a binary tree does not exceed 1,
//!  and the left and right subtrees are both a balanced binary tree.


use std::borrow::Borrow;
use std::cell::RefCell;
use std::cmp::max;

use crate::tree::OptBinaryNode;

pub fn is_balanced<T>(root: &OptBinaryNode<T>) -> bool {
    height_for_balanced(&root) >= 0
}

fn height_for_balanced<T>(root: &OptBinaryNode<T>) -> i32 {
    return match root {
        None => 0,
        Some(node) => {
            let node = RefCell::borrow(node);
            let left = node.left.borrow();
            let right = node.right.borrow();
            let h1 = height_for_balanced(left);
            let h2 = height_for_balanced(right);
            if h1 == -1 || h2 == -1 || (h1 - h2).abs() > 1 {
                -1
            } else {
                max(h1, h2) + 1
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::tree::{BinaryNode, OptBinaryNode};
    use crate::tree::avl::is_balanced;

    fn tree() -> OptBinaryNode<i32> {
        let mut tn1 = BinaryNode::new(1);
        let mut tn3 = BinaryNode::new(3);
        let tn4 = BinaryNode::new(4);
        let tn2 = BinaryNode::new(2);
        tn3.right = Some(Rc::new(RefCell::new(tn2)));
        tn1.left = Some(Rc::new(RefCell::new(tn3)));
        tn1.right = Some(Rc::new(RefCell::new(tn4)));
        let root = Some(Rc::new(RefCell::new(tn1)));
        root
    }

    #[test]
    fn test_is_balanced() {
        let root = tree();
        let res = is_balanced(&root);
        assert_eq!(true, res);
    }
}
