//! Binary Search Tree

use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

use crate::tree::{BinaryNode, OptBinaryNode};

// catalan calculate how many binary search trees can be formed by node_num nodes
pub fn catalan(node_num: u64) -> u64 {
    let mut c: f64 = 1.0;
    for i in 0..node_num {
        c = c * 2.0 * (2.0 * i as f64 + 1.0) / (i as f64 + 2.0);
    }
    c as u64
}

// generate_all generate all binary search trees that can be combined by asc_values.
pub fn generate_all<T: Clone>(asc_values: &Vec<T>) -> Vec<OptBinaryNode<T>> {
    generate_all_help(asc_values, 1, asc_values.len())
}

fn generate_all_help<T: Clone>(asc_values: &Vec<T>, start: usize, end: usize) -> Vec<OptBinaryNode<T>> {
    let mut trees = Vec::new();
    if start > end {
        trees.push(None);
        return trees;
    }

    for k in start..=end {
        let left_trees = generate_all_help(asc_values, start, k - 1);
        let right_trees = generate_all_help(asc_values, k + 1, end);
        for i in left_trees.iter() {
            for j in right_trees.iter() {
                let mut node = BinaryNode::new(asc_values.get(k - 1).unwrap().clone());
                node.left = i.clone();
                node.right = j.clone();
                trees.push(Some(Rc::new(RefCell::new(node))));
            }
        }
    }

    trees
}

// is_valid determine whether it is a binary search tree.
pub fn is_valid<T: Clone + PartialOrd>(root: &OptBinaryNode<T>) -> bool {
    is_valid_bst(root, &mut None)
}

fn is_valid_bst<T: Clone + PartialOrd>(root: &OptBinaryNode<T>, last_val: &mut Option<T>) -> bool {
    return match root {
        None => true,
        Some(node) => {
            let node = RefCell::borrow(node);
            if !is_valid_bst((node.left).borrow(), last_val) {
                return false
            }
            if last_val.is_some() && node.val <= *last_val.as_ref().unwrap() {
                return false
            }
            last_val.replace(node.val.clone());
            if !is_valid_bst((node.right).borrow(), last_val) {
                return false
            }
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::bst::generate_all;

    use super::catalan;

    #[test]
    fn test_catalan() {
        assert_eq!(1, catalan(1));
        assert_eq!(2, catalan(2));
        assert_eq!(5, catalan(3));
        assert_eq!(6564120420, catalan(20))
    }

    #[test]
    fn test_generate_all() {
        let asc_values = (1..=5).collect::<Vec<_>>();
        println!("{:?}", generate_all(&asc_values));
    }
}
