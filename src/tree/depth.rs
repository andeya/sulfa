//! Depth of Binary Tree

use std::borrow::Borrow;
use std::cell::RefCell;
use std::cmp::max;

use crate::tree::OptBinaryNode;

pub fn max_depth_dfs<T>(root: &OptBinaryNode<T>) -> i32 {
    return match root {
        None => 0,
        Some(node) => {
            let d1 = max_depth_dfs(RefCell::borrow(node).left.borrow());
            let d2 = max_depth_dfs(RefCell::borrow(node).right.borrow());
            max(d1, d2) + 1
        }
    };
}

// min_depth_bfs calculate the number of nodes on the shortest path from the root node to the nearest leaf node.
pub fn min_depth_bfs<T>(root: &OptBinaryNode<T>) -> i32 {
    let mut depth: i32 = 0;
    if root.is_none() { return depth; }
    let mut queue: Vec<OptBinaryNode<T>> = Vec::new();
    queue.push(root.clone());
    while !queue.is_empty() {
        depth += 1;
        for _ in 0..queue.len() {
            let node = queue.remove(0);
            let node = RefCell::borrow(node.as_ref().unwrap());
            if node.left.is_none() && node.right.is_none() {
                return depth;
            }
            if node.left.is_some() {
                queue.push(node.left.clone());
            }
            if node.right.is_some() {
                queue.push(node.right.clone());
            }
        }
    }
    return depth;
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::tree::{BinaryNode, OptBinaryNode};

    use super::{max_depth_dfs, min_depth_bfs};

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
    fn max_depth() {
        let root = tree();
        let res = max_depth_dfs(&root);
        assert_eq!(3, res);
    }

    #[test]
    fn min_depth() {
        let root = tree();
        let res = min_depth_bfs(&root);
        assert_eq!(2, res);
    }
}
