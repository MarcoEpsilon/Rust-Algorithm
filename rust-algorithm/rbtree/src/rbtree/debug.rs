use super::create::{
    RBTree,
    RBNode,
    ChildNodePointer,
    NodeColor,
};
use std::fmt::Debug;

impl<K,V> RBNode<K,V>
    where
        K: Debug + PartialOrd + Clone,
        V: Debug + PartialOrd + Clone,
{
    fn inorder_debug(this:&ChildNodePointer<K,V>) {
        let this_node = this.borrow();
        if let Some(ref left) = this_node.left {
            Self::inorder_debug(&left);
        }
        let color;
        match this_node.color {
            NodeColor::Red => {
                color = String::from("Red");
            },
            _ => {
                color = String::from("Black");
            }
        }
        println!("{{key: {:?}   value: {:?} color: {}}}",this_node.key.clone(),this_node.value.clone(),color);
        if let Some(ref right) = this_node.right {
            Self::inorder_debug(&right);
        }
    }
}


impl<K,V> RBTree<K,V>
    where
        K: Debug + PartialOrd + Clone,
        V: Debug + PartialOrd + Clone,
{
    pub fn inorder_debug(&self) {
        if let Some(ref root) = self.root {
            RBNode::inorder_debug(&root);
        }
    }
}