use super::create::RBTree;
use super::create::RBNode;
use super::create::ParentNode;
use super::create::ChildNode;
use super::create::ChildNodePointer;
use super::create::NodeColor;
use std::fmt::Debug;
use std::rc::{Rc,Weak};
use std::cell::{RefCell};
use std::ptr;
use std::mem::size_of;
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
    pub (super) fn is_red(&self) -> bool {
        if let NodeColor::Red = self.color {
            return true;
        }
        return false;
    }
    fn is_black(&self) -> bool {
        return !self.is_red();
    }
    fn has_parent(&self) -> bool {
        return self.parent.upgrade().is_some();
    }
    fn is_left_node_unchecked(this:&ChildNodePointer<K,V>) -> bool {
        let this_node = this.borrow();
        //you need check
        let parent = this_node.parent.upgrade().unwrap();
        if let Some(ref left) = parent.borrow().left {
            if Rc::ptr_eq(&left,&this) {
                return true;
            }
        }
        return false;
    }
    fn is_right_node_unchecked(this:&ChildNodePointer<K,V>) -> bool {
        return !Self::is_left_node_unchecked(&this);
    }
    fn has_black_parent(this:&ChildNodePointer<K,V>) -> bool {
        let this_node = this.borrow();
        if this_node.has_parent() {
            let parent_node = this_node.parent.upgrade().unwrap();
            let parent_node = parent_node.borrow();
            return parent_node.is_black();
        }
        return false;
    }
    fn has_red_parent(this:&ChildNodePointer<K,V>) -> bool {
        let this_node = this.borrow();
        if this_node.has_parent() {
            let parent_node = this_node.parent.upgrade().unwrap();
            let parent_node = parent_node.borrow();
            return parent_node.is_red();
        }
        return false;
    }
    fn get_uncle_unchecked(this:&ChildNodePointer<K,V>) -> ChildNodePointer<K,V> {
        let this_node = this.borrow();
        let parent = this_node.parent.upgrade().unwrap();
        let parent_node = parent.borrow();
        let grand = parent_node.parent.upgrade().unwrap();
        let grand_node = grand.borrow();
        if Self::is_left_node_unchecked(&parent) {
            return grand_node.right.clone().unwrap();
        } else {
            return grand_node.left.clone().unwrap();
        }
    }
    fn has_red_uncle(this:&ChildNodePointer<K,V>) -> bool {
        let this_node = this.borrow();
        if this_node.has_parent() {
            let parent = this_node.parent.upgrade().unwrap();
            let parent_node = parent.borrow();
            if parent_node.has_parent() {
                let grand = parent_node.parent.upgrade().unwrap();
                let grand_node = grand.borrow();
                if Self::is_left_node_unchecked(&parent) {
                    match grand_node.right {
                        Some(ref right) => {
                            return right.borrow().is_red();
                        },
                        None => {
                            return false;
                        }
                    }
                } else {
                    match grand_node.left {
                        Some(ref left) => {
                            return left.borrow().is_red();
                        },
                        None => {
                            return false;
                        }
                    }
                }      
            }
        }
        return false;
    }
    fn flip_parent_and_uncle_color_unchecked(this:&ChildNodePointer<K,V>) {
        let this_node = this.borrow() ;
        let uncle_node = Self::get_uncle_unchecked(&this);
        let mut uncle_node = uncle_node.borrow_mut();
        let parent_node = this_node.parent.upgrade().unwrap();
        let mut parent_node = parent_node.borrow_mut();
        let grand_node = parent_node.parent.upgrade().unwrap();
        let mut grand_node = grand_node.borrow_mut();
        uncle_node.color = NodeColor::Black;
        parent_node.color = NodeColor::Black;
        grand_node.color = NodeColor::Red;
    }
    fn get_grand_unchecked(this:&ChildNodePointer<K,V>) -> ChildNodePointer<K,V> {
        let this_node = this.borrow();
        let parent_node = this_node.parent.upgrade().unwrap();
        let parent_node = parent_node.borrow();
        let grand = Rc::clone(&parent_node.parent.upgrade().unwrap());
        return grand;
    }
    fn flip_parent_and_grand_color(this:&ChildNodePointer<K,V>) {
        let this_node = this.borrow();
        let parent_node = this_node.parent.upgrade().unwrap();
        let mut parent_node = parent_node.borrow_mut();
        let grand_node = parent_node.parent.upgrade().unwrap();
        let mut grand_node = grand_node.borrow_mut();
        parent_node.color = NodeColor::Black;
        grand_node.color = NodeColor::Red;
    }
    fn is_left_parent_unchecked(this:&ChildNodePointer<K,V>) -> bool {
        let this_node = this.borrow();
        let parent = this_node.parent.upgrade().unwrap();
        return Self::is_left_node_unchecked(&parent);
    }
    fn insert_fixup(this:&ChildNodePointer<K,V>,root_hook:*mut Option<ChildNode<K,V>>) {
        {
            let this_node = this.borrow();
            let parent = this_node.parent.upgrade();
            if let None = parent {
                unsafe {
                    let node = Some(Some(Rc::clone(&this)));
                    ptr::write(root_hook,node);
                }
            }
        }
        if Self::has_red_parent(&this) {
           if Self::has_red_uncle(&this) {
               Self::flip_parent_and_uncle_color_unchecked(&this);
               Self::insert_fixup(&Self::get_grand_unchecked(&this),root_hook);
           } else if Self::is_right_node_unchecked(&this) {
               if Self::is_left_parent_unchecked(&this) {
                   let parent;
                   {
                       let this_node = this.borrow();
                       parent = this_node.parent.upgrade().unwrap();
                   }
                   Self::rotate_to_left(&parent);
                   Self::insert_fixup(&parent,root_hook);
               } else {
                    Self::flip_parent_and_grand_color(&this);
                    let grand;
                    let parent;
                    {
                        let this_node = this.borrow();
                        parent = this_node.parent.upgrade().unwrap();
                        let parent_node = parent.borrow();
                        grand = parent_node.parent.upgrade().unwrap();
                    }
                   Self::rotate_to_left(&grand);
                   Self::insert_fixup(&parent,root_hook);
               }
           } else {
               if Self::is_left_parent_unchecked(&this) {
                    Self::flip_parent_and_grand_color(&this);
                    let grand;
                    let parent;
                    {
                        let this_node = this.borrow();
                        parent = this_node.parent.upgrade().unwrap();
                        let parent_node = parent.borrow();
                        grand = parent_node.parent.upgrade().unwrap();
                    }
                    Self::rotate_to_right(&grand);
                    Self::insert_fixup(&parent,root_hook);
               } else {
                   let parent;
                   {
                       let this_node = this.borrow();
                       parent = this_node.parent.upgrade().unwrap();
                   }
                   Self::rotate_to_right(&parent);
                   Self::insert_fixup(&parent,root_hook);
               }
           }
        }
    }
    fn insert_help(this:&ChildNodePointer<K,V>,key:K,value:V) -> ChildNodePointer<K,V> {
        let mut this_node = this.borrow_mut();
        if key <= this_node.key {
            match this_node.left {
                Some(ref left) => Self::insert_help(&left,key,value),
                None => {
                    let node:RBNode<K,V> = RBNode {
                        left: None,
                        right: None,
                        parent: Rc::downgrade(&this),
                        key: key,
                        value: value,
                        color: NodeColor::Red,
                    };
                    this_node.left = Some(Rc::new(RefCell::new(node)));
                    return this_node.left.clone().unwrap();
                }
            }
        } else {
            match this_node.right {
                Some(ref right) => Self::insert_help(&right,key,value),
                None => {
                    let node:RBNode<K,V> = RBNode {
                        left: None,
                        right: None,
                        parent: Rc::downgrade(&this),
                        key: key,
                        value: value,
                        color: NodeColor::Red,
                    };
                    this_node.right = Some(Rc::new(RefCell::new(node)));
                    return this_node.right.clone().unwrap();
                },
            }
        }
    }
    fn insert(this:&ChildNodePointer<K,V>,key:K,value:V) -> Option<ChildNode<K,V>> {
        let node = Self::insert_help(&this,key,value);
        let root_hook:*mut Option<ChildNode<K,V>>;
        unsafe {
            let mut t:Option<ChildNode<K,V>> = None;
            root_hook = &mut t as *mut Option<ChildNode<K,V>>;
        }
        Self::insert_fixup(&node,root_hook);
        unsafe {
            if !root_hook.is_null() {
                return root_hook.read();
            } else {
                unreachable!();
            }
        }
    }
}

impl<K,V> RBTree<K,V>
    where
        K: Debug + Clone + PartialOrd,
        V: Debug + Clone + PartialOrd,
{
    pub fn insert(&mut self,key:K,value:V) {
        let mut new_root:Option<ChildNode<K,V>> = None;
        match self.root {
            Some(ref root) => {
                new_root = RBNode::insert(&root,key,value);
            },
            None => {
                self.root = Some(Rc::new(RefCell::new(RBNode::new(key,value,NodeColor::Black))));
            }
        }
        if let Some(ref root) = new_root {
            self.root = root.clone();
        }
        if let Some(ref root) = self.root {
            let mut root_node = root.borrow_mut();
            root_node.color = NodeColor::Black;
        }
    }
}