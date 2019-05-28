use std::rc::{Rc,Weak};
use std::cell::RefCell;
use std::fmt::Debug;
use super::{
    RBNode,
    RBTree,
    NodeColor,
    ChildNodePointer,
    ChildNode,
};

impl<K,V> RBNode<K,V>
    where
        K: PartialOrd + Clone + Debug,
        V: PartialOrd + Clone + Debug,
{
        
}

impl<K,V> RBTree<K,V>
    where
        K: PartialOrd + Clone + Debug,
        V: PartialOrd + Clone + Debug,
{

}