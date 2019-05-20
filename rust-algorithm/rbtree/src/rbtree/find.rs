use std::rc::{Rc,Weak};
use std::cell::RefCell;
use super::create::RBNode;
use super::create::RBTree;
use super::create::ChildNodePointer;
use std::fmt::Debug;

impl<K,V> RBNode<K,V>
    where
        K: Debug + Clone + PartialOrd,
        V: Debug + Clone,
{
    fn search(this:&ChildNodePointer<K,V>,key:K) -> Option<(K,V)> {
        let this_node = this.borrow();
        if key == this_node.key {
            return Some((this_node.key.clone(),this_node.value.clone()));
        } else if key < this_node.key {
            if let Some(ref left) = this_node.left {
                return Self::search(&left,key);
            }
            return None;
        } else {
            if let Some(ref right) = this_node.right {
                return Self::search(&right,key);
            }
            return None;
        }
    }
    fn maximum(this:&ChildNodePointer<K,V>) -> Option<(K,V)> {
        let this_node = this.borrow();
        match this_node.right {
            Some(ref right) => Self::maximum(&right),
            None => {
                return Some((this_node.key.clone(),this_node.value.clone()));
            }
        }
    }

    fn minimum(this:&ChildNodePointer<K,V>) -> Option<(K,V)> {
        let this_node = this.borrow();
        if let Some(ref left) = this_node.left {
            return Self::minimum(&left);
        }
        return Some((this_node.key.clone(),this_node.value.clone()));
    }
    fn predecessor(this:&ChildNodePointer<K,V>) -> Option<(K,V)> {
        let this_node = this.borrow();
        match this_node.left {
            Some(ref left) => Self::maximum(&left),
            None => {
                let mut current = Rc::clone(&this);
                while let Some(ref parent) = current.clone().borrow().parent.upgrade() {
                    let parent_node = parent.borrow();
                    if parent_node.right.is_some() {
                        let parent_right = parent_node.right.clone().unwrap();
                        if Rc::ptr_eq(&current,&parent_right) {
                            return Some((parent_node.key.clone(),parent_node.value.clone()));
                        }
                    }
                    current = Rc::clone(&parent);
                }
                return None;
            }
        }
    }
    fn successor(this:&ChildNodePointer<K,V>) -> Option<(K,V)> {
        let this_node = this.borrow();
        match this_node.right {
            Some(ref right) => Self::minimum(&right),
            None => {
                let mut current = Rc::clone(&this);
                while let Some(ref parent) = current.clone().borrow().parent.upgrade() {
                    let parent_node = parent.borrow();
                    if parent_node.left.is_some() {
                        let parent_left = parent_node.left.clone().unwrap();
                        if Rc::ptr_eq(&current, &parent_left) {
                            return Some((parent_node.key.clone(),parent_node.value.clone()));
                        }
                    }
                    current = Rc::clone(&parent);
                }
                return None;
            }
        }
    }
}
impl<K,V> RBTree<K,V>
    where
        K: Debug + Clone + PartialOrd,
        V: Debug + Clone,
{
    pub fn search(&self,key:K) -> Option<(K,V)> {
        match self.root {
            Some(ref root) => RBNode::search(&root,key),
            None => None,
        }
    }
    pub fn maximum(&self) -> Option<(K,V)> {
        match self.root {
            Some(ref root) => RBNode::maximum(&root),
            None => None,
        }
    }
    pub fn predecessor(&self) -> Option<(K,V)> {
        match self.root {
            Some(ref root) => RBNode::predecessor(&root),
            None => None,
        }
    }
    pub fn successor(&self) -> Option<(K,V)> {
        match self.root {
            Some(ref root) => RBNode::successor(&root),
            None => None,
        }
    }
}