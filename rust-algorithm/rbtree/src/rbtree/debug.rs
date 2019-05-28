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
    fn is_red_node(this:&ChildNodePointer<K,V>) -> bool {
        let this_node = this.borrow();
        return this_node.is_red();
    }
    fn is_black_node(this:&ChildNodePointer<K,V>) -> bool {
        return !Self::is_red_node(&this);
    }
    //fixme: first check whether is bst
    fn is_rbtree(root:&ChildNodePointer<K,V>) -> bool {
        let root_node = root.borrow();
        let mut height:usize = 0;
        if Self::is_black_node(&root) {
            height = 1;
        }
        match root_node.left {
            Some(ref left) => {
                match root_node.right {
                    Some(ref right) => {
                        let left_height = height + Self::black_height(&left);
                        let right_height = height + Self::black_height(&right);
                        if right_height != left_height {
                            return false;
                        }
                        return Self::is_rbtree(&left) && Self::is_rbtree(&right);
                    },
                    None => Self::black_height(&left) == 0,
                }
            },
            None => {
                match root_node.right {
                    Some(ref right) => Self::black_height(&right) == 0,
                    None => true,
                }
            },
        }
    }

    fn black_height(root:&ChildNodePointer<K,V>) -> usize {
        let root_node = root.borrow();
        let mut height:usize = 0;
        if Self::is_black_node(&root) {
            height = 1;
        }
        match root_node.left {
            Some(ref left) => {
                return height + Self::black_height(&left);                
            },
            None => height,
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
    pub fn is_rbtree(&self) -> bool {
        match self.root {
            Some(ref root) => RBNode::is_rbtree(&root),
            None => true,
        }
    }
}