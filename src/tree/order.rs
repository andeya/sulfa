use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

use super::node::{BinaryNode, OptBinaryNode};

#[derive(Clone, Copy)]
pub enum Order {
    InOrder,
    PreOrder,
    PostOrder,
}

pub fn traversal<T: Clone>(root: &OptBinaryNode<T>, order: Order) -> Vec<T> {
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

#[test]
fn test_inorder_traversal() {
    assert_eq!(2 + 2, 4);
    let mut tn1 = BinaryNode::new(1);
    let mut tn3 = BinaryNode::new(3);
    let tn2 = BinaryNode::new(2);
    tn3.right = Some(Rc::new(RefCell::new(tn2)));
    tn1.left = Some(Rc::new(RefCell::new(tn3)));
    let root = Some(Rc::new(RefCell::new(tn1)));
    let res = traversal(&root, Order::InOrder);
    println!("=================={:?}", res);
}



