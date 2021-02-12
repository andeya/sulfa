//! Symmetric Binary Tree

use crate::tree::OptBinaryNode;

pub fn is_symmetric<T: PartialEq>(root: OptBinaryNode<T>) -> bool {
    match root {
        None => true,
        Some(node) => {
            let node = node.borrow();
            compare(node.left.clone(), node.right.clone())
        }
    }
}

fn compare<T: PartialEq>(a: OptBinaryNode<T>, b: OptBinaryNode<T>) -> bool {
    if a.is_none() {
        return b.is_none()
    }
    if b.is_none() {
        return false
    }
    let a = a.unwrap();
    let b = b.unwrap();
    let a2 = a.borrow();
    let b2 = b.borrow();
    if a2.val != b2.val {
        return false
    } else {
        compare(a2.left.clone(), b2.right.clone()) &&
            compare(a2.right.clone(), b2.left.clone())
    }
}
