//! Traverse the binary tree

use std::borrow::Borrow;
use std::cell::RefCell;
use std::cmp::max;

use super::node::OptBinaryNode;

#[derive(Clone, Copy)]
pub enum Order {
    InOrder,
    PreOrder,
    PostOrder,
}

pub fn bfs<T: Clone>(root: &OptBinaryNode<T>, order: Order) -> Vec<T> {
    let mut stack: Vec<OptBinaryNode<T>> = Vec::new();
    let mut result: Vec<T> = Vec::new();
    if root.is_some() { stack.push(root.clone()) }
    while !stack.is_empty() {
        let node = stack.pop().unwrap();
        if node.is_some() {
            let ref_node = RefCell::borrow(node.as_ref().unwrap());
            match order {
                Order::InOrder => {
                    if ref_node.right.is_some() {
                        stack.push(ref_node.right.clone())
                    }
                    stack.push(node.clone());
                    stack.push(None);
                    if ref_node.left.is_some() {
                        stack.push(ref_node.left.clone())
                    }
                }
                Order::PreOrder => {
                    if ref_node.right.is_some() {
                        stack.push(ref_node.right.clone())
                    }
                    if ref_node.left.is_some() {
                        stack.push(ref_node.left.clone())
                    }
                    stack.push(node.clone());
                    stack.push(None);
                }
                Order::PostOrder => {
                    stack.push(node.clone());
                    stack.push(None);
                    if ref_node.right.is_some() {
                        stack.push(ref_node.right.clone())
                    }
                    if ref_node.left.is_some() {
                        stack.push(ref_node.left.clone())
                    }
                }
            }
        } else {
            let node = stack.pop().unwrap();
            result.push(RefCell::borrow(node.unwrap().borrow()).val.clone())
        }
    }
    result
}

pub fn level_bfs<T: Clone>(root: &OptBinaryNode<T>) -> Vec<Vec<T>> {
    let mut queue: Vec<OptBinaryNode<T>> = Vec::new();
    let mut result: Vec<Vec<T>> = Vec::new();
    if root.is_some() { queue.push(root.clone()) }
    while !queue.is_empty() {
        let mut level: Vec<T> = Vec::new();
        for _ in 0..queue.len() {
            let node = queue.remove(0);
            let node = RefCell::borrow(node.as_ref().unwrap());
            level.push(node.val.clone());
            if node.left.is_some() {
                queue.push(node.left.clone())
            }
            if node.right.is_some() {
                queue.push(node.right.clone())
            }
        }
        result.push(level);
    }
    result
}

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
    if root.is_none() { return depth }
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
    use crate::tree::order::{bfs, level_bfs, max_depth_dfs, min_depth_bfs, Order};

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
    fn inorder_bfs() {
        assert_eq!(2 + 2, 4);
        let root = tree();
        let res = bfs(&root, Order::InOrder);
        println!("inorder_bfs:=================={:?}", res);
    }

    #[test]
    fn levelorder_bfs() {
        assert_eq!(2 + 2, 4);
        let root = tree();
        let res = level_bfs(&root);
        println!("levelorder_bfs:=================={:?}", res);
    }

    #[test]
    fn max_depth() {
        let root = tree();
        let res = max_depth_dfs(&root);
        println!("max_depth:=================={:?}", res);
    }

    #[test]
    fn min_depth() {
        let root = tree();
        let res = min_depth_bfs(&root);
        println!("max_depth:=================={:?}", res);
    }
}
