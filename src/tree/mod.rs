pub use node::*;

pub mod order;
pub mod symmetric;

mod node {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug, PartialEq, Eq)]
    pub struct BinaryNode<T> {
        pub val: T,
        pub left: OptBinaryNode<T>,
        pub right: OptBinaryNode<T>,
    }

    pub type OptBinaryNode<T> = Option<Rc<RefCell<BinaryNode<T>>>>;

    impl<T> BinaryNode<T> {
        #[inline]
        pub fn new(val: T) -> Self {
            BinaryNode {
                val,
                left: None,
                right: None,
            }
        }
    }
}
