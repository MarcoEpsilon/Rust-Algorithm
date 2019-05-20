use super::create::RBTree;
use super::create::RBNode;
use super::create::ParentNode;
use super::create::ChildNodePointer;
use std::fmt::Debug;
use std::rc::{Rc,Weak};
use std::cell::{RefCell};
enum ChildPos {
    Left,
    Right,
    Invalid,
}
impl<K,V> RBNode<K,V>
    where
        K: Debug + Clone + PartialOrd,
        V: Debug + Clone + PartialOrd,
{
    fn modify_parent(child:Option<ChildNodePointer<K,V>>,parent:&ChildNodePointer<K,V>) {
        if let Some(ref child) = child {
            let mut child_node = child.borrow_mut();
            child_node.parent = Rc::downgrade(&parent);
        }
    }
    fn modify_child(parent:&ParentNode<K,V>,child:&ChildNodePointer<K,V>,pos:ChildPos) {
        let parent = Weak::upgrade(&parent);
        if let Some(ref parent) = parent {
            let mut parent = parent.borrow_mut();
            match pos {
                ChildPos::Left => {
                    parent.left = Some(Rc::clone(&child));
                },
                ChildPos::Right => {
                    parent.right = Some(Rc::clone(&child));
                },
                _ => unreachable!()             
            }
        }
    }
    fn child_pos(child:&ChildNodePointer<K,V>,parent:&ParentNode<K,V>) -> ChildPos {
        if let Some(parent) = Weak::upgrade(&parent) {
            let parent_node = parent.borrow();
            if let Some(ref left) = parent_node.left {
                if Rc::ptr_eq(&child,&left) {
                    return ChildPos::Left;
                }
            }
            if let Some(ref right) = parent_node.right {
                if Rc::ptr_eq(&child,&right) {
                    return ChildPos::Right;
                }
            }
        }
        return ChildPos::Invalid;            
    }
    fn rotate_to_left(this:&ChildNodePointer<K,V>) {
        let mut this_node = this.borrow_mut();
        if let Some(ref right) = this_node.right.clone() {
            let mut right_node = right.borrow_mut();
            right_node.parent = Weak::clone(&this_node.parent);
            Self::modify_child(&this_node.parent,&right,Self::child_pos(&this,&this_node.parent));
            this_node.right = right_node.left.clone();
            Self::modify_parent(right_node.left.clone(),&this);
            right_node.left = Some(Rc::clone(&this));
            this_node.parent = Rc::downgrade(&right);
        } else {
            panic!("can't fit rotate_to_left apply scene")
        }
    }
    fn rotate_to_right(this:&ChildNodePointer<K,V>) {
        let mut this_node = this.borrow_mut();
        if let Some(ref left) = this_node.left.clone() {
            let mut left_node = left.borrow_mut();
            left_node.parent = Weak::clone(&this_node.parent);
            Self::modify_child(&this_node.parent,&left,Self::child_pos(&this,&this_node.parent));
            this_node.left = left_node.right.clone();
            Self::modify_parent(left_node.right.clone(),&this);
            left_node.right = Some(Rc::clone(&this));
            this_node.parent = Rc::downgrade(&left);
        } else {
            panic!("can't rotate_to_right!");
        }
    }
}

impl<K,V> RBTree<K,V>
    where
        K: Debug + Clone + PartialOrd,
        V: Debug + Clone + PartialOrd,
{

}