use std::rc::{Rc,Weak};
use std::cell::RefCell;
use std::fmt::Debug;
type RawChildNode<K:PartialOrd + Debug + Clone,V:Debug + Clone> = BinarySearchTreeNode<K,V>;
type ChildNodePointer<K:PartialOrd + Debug + Clone,V:Debug + Clone> = Rc<RefCell<BinarySearchTreeNode<K,V>>>;
type ChildNode<K:PartialOrd + Debug + Clone,V:Debug + Clone> = Option<Rc<RefCell<BinarySearchTreeNode<K,V>>>>;
type ParentNode<K:PartialOrd + Debug + Clone,V:Debug + Clone> = Weak<RefCell<BinarySearchTreeNode<K,V>>>;
#[derive(Debug)]
pub struct BinarySearchTreeNode<K:PartialOrd + Debug + Clone,V:Debug + Clone> {
    left:ChildNode<K,V>,
    right:ChildNode<K,V>,
    parent:ParentNode<K,V>,
    key:K,
    value:V,
}
enum DeleteMethod {
    Precessor,
    Successor,
}
enum DeleteState {
    MayOnlyLeft,
    MayOnlyRight,
    OnlyLeft,
    OnlyRight,
}
impl<K:PartialOrd + Debug + Clone,V:Debug + Clone> BinarySearchTreeNode<K,V> {
    fn new(key:K,value:V) -> Self {
        return BinarySearchTreeNode {
            left:None,
            right:None,
            parent:Weak::new(),
            key:key,
            value:value,
        };
    }
    fn wrap(node:RawChildNode<K,V>) -> ChildNode<K,V> {
        return Some(Rc::new(RefCell::new(node)));
    }
    fn insert(this:&ChildNodePointer<K,V>,mut node:RawChildNode<K,V>) {
        let mut this_node = this.borrow_mut();
        if node.key <= this_node.key {
            match this_node.left {
                Some(ref left) => Self::insert(&left,node),
                None => {
                    node.parent = Rc::downgrade(&this);
                    this_node.left = Self::wrap(node);
                }
            }
        } else {
            match this_node.right {
                Some(ref right) => Self::insert(&right,node),
                None => {
                    node.parent = Rc::downgrade(&this);
                    this_node.right = Self::wrap(node);
                } 
            }
        }
    }
    fn maximum(this:&ChildNodePointer<K,V>) -> (K,V) {
        let this = this.borrow();
        match this.right {
            Some(ref right) => Self::maximum(&right),
            None => {
                return (this.key.clone(),this.value.clone());
            }
        }
    }
    fn minimum(this:&ChildNodePointer<K,V>) -> (K,V) {
        let this = this.borrow();
        match this.left {
            Some(ref left) => Self::minimum(&left),
            None => {
                return (this.key.clone(),this.value.clone());
            }
        }
    }
    fn maximum_node(this:&ChildNodePointer<K,V>) -> ChildNodePointer<K,V> {
        let node = this.borrow();
        match node.right {
            Some(ref right) => Self::maximum_node(&right),
            None => {
                return Rc::clone(&this);
            }
        }
    }
    fn minimum_node(this:&ChildNodePointer<K,V>) -> ChildNodePointer<K,V> {
        let node = this.borrow();
        match node.left {
            Some(ref left) => Self::minimum_node(left),
            None => {
                return Rc::clone(&this);
            }
        }
    }
    fn has_parent(this:&ChildNodePointer<K,V>) -> bool {
        return this.borrow().parent.upgrade().is_some();
    }
    fn same_node(this:&ChildNodePointer<K,V>,other:&ChildNodePointer<K,V>) -> bool {
        return Rc::ptr_eq(this, other);
    }
    fn is_parent(child:&ChildNodePointer<K,V>,parent:&ParentNode<K,V>) -> bool {
        if !Self::has_parent(&child) {
            return false;
        }
        let parent = parent.upgrade();
        match parent {
            Some(ref parent) => {
                let child_parent = child.borrow().parent.upgrade().unwrap();
                return Self::same_node(&child_parent,&parent);
            },
            None => false,
        }
    }
    fn is_left(left:&ChildNodePointer<K,V>,parent:&ParentNode<K,V>) -> bool {
        if let false = Self::is_parent(&left,&parent) {
            return false;
        }
        let parent = parent.upgrade().unwrap();
        let parent = parent.borrow();
        match parent.left {
            Some(ref parent_left) => Self::same_node(&left,&parent_left),
            None => {
                return false;
            },
        }
    }
    fn is_right(right:&ChildNodePointer<K,V>,parent:&ParentNode<K,V>) -> bool {
        if !Self::is_parent(&right,&parent) {
            return false;
        }
        let parent = parent.upgrade().unwrap();
        let parent = parent.borrow();
        match parent.right {
            Some(ref parent_right) => Self::same_node(&right,&parent_right),
            None => {
                return false;                        
            },
        }
    }
    fn has_left(this:&ChildNodePointer<K,V>) -> bool {
        match this.borrow().left {
            Some(_) => true,
            None => false,
        }
    }
    fn has_right(this:&ChildNodePointer<K,V>) -> bool {
        match this.borrow().right {
            Some(_) => true,
            None => false,
        }
    }
    fn has_childs(this:&ChildNodePointer<K,V>) -> bool {
        return Self::has_left(&this) && Self::has_right(&this);
    }
    fn precessor(this:&ChildNodePointer<K,V>) -> Option<ChildNodePointer<K,V>> {
        match this.borrow().left {
            Some(ref left) => Some(Self::maximum_node(&left)),
            None => {
                let mut current = Rc::clone(&this);
                while let true = Self::has_parent(&current) {
                    let parent = Weak::clone(&current.borrow().parent);
                    if Self::is_right(&current, &parent) {
                        return Some(parent.upgrade().unwrap());     
                    }
                    if Self::has_parent(&current) {
                        current = parent.upgrade().unwrap();    
                    } else {
                        return None;
                    }
                }
                return None;
            },
        }
    }
    fn successor(this:&ChildNodePointer<K,V>) -> Option<ChildNodePointer<K,V>> {
        match this.borrow().right {
            Some(ref right) => Some(Self::minimum_node(&right)),
            None => {
                let mut current = Rc::clone(&this);
                while let true = Self::has_parent(&current) {
                    let parent = Weak::clone(&current.borrow().parent);
                    if Self::is_left(&current, &parent) {
                        return Some(parent.upgrade().unwrap());    
                    }
                    if Self::has_parent(&current) {
                        current = parent.upgrade().unwrap();
                    } else {
                        return None;
                    }
                }
                return None;
            },
        }
    }
    fn search_node(this:&ChildNodePointer<K,V>,key:K) -> Option<ChildNodePointer<K,V>> {
        let node = this.borrow();
        if node.key == key {
            return Some(Rc::clone(&this)); 
        } else if node.key < key {
            if let Some(ref left) = node.left {
                return Self::search_node(&left, key);
            }
            return None;
        } else {
            if let Some(ref right) = node.right {
                return Self::search_node(&right,key);
            }
            return None;
        }
    }
    fn delete_with_child(this:&ChildNodePointer<K,V>,state:DeleteState) {
        let mut this_node = this.borrow_mut();
        match state {
            DeleteState::MayOnlyLeft | DeleteState::OnlyLeft => {
                if let Some(ref left) = this_node.left {
                    let mut this_left = left.borrow_mut();
                    this_left.parent = Weak::clone(&this_node.parent);
                    if let Some(ref parent) = this_node.parent.upgrade() {
                        let mut parent_node = parent.borrow_mut();
                        if parent_node.left.is_some() {
                            if Self::same_node(&this,&parent_node.left.clone().unwrap()) {
                                parent_node.left = Some(Rc::clone(&left));
                            } else {
                                parent_node.right = Some(Rc::clone(&left));
                            }
                        } else {
                            parent_node.right = Some(Rc::clone(&left));
                        }
                    }
                } else {
                    this_node.parent.upgrade().unwrap();
                    match this_node.parent.upgrade() {
                        Some(ref parent) => {
                          let mut parent_node = parent.borrow_mut();
                          if parent_node.left.is_some() {
                              if Self::same_node(&this,&parent_node.left.clone().unwrap()) {
                                  parent_node.left = None;
                              } else {
                                  parent_node.right = None;
                              }
                          } else {
                              parent_node.right = None;
                          }
                        },
                        _ => (),
                    }
                }
            },
            DeleteState::MayOnlyRight | DeleteState::OnlyRight => {
                if let Some(ref right) = this_node.right {
                    let mut this_right = right.borrow_mut();
                    this_right.parent = Weak::clone(&this_node.parent);
                    match this_node.parent.upgrade() {
                        Some(ref parent) => {
                            let mut parent_node = parent.borrow_mut();
                            if parent_node.left.is_some() {
                                if Self::same_node(&this,&parent_node.left.clone().unwrap()) {
                                    parent_node.left = Some(Rc::clone(&right));
                                } else {
                                    parent_node.right = Some(Rc::clone(&right));
                                }
                            } else {
                                parent_node.right = Some(Rc::clone(&right));
                            }
                        },
                        _ => (),
                    }
                } else {
                    if let Some(ref parent) = this_node.parent.upgrade() {
                        let mut parent_node = parent.borrow_mut();
                        if parent_node.left.is_some() {
                            if Self::same_node(&this,&parent_node.left.clone().unwrap()) {
                                parent_node.left = None;
                            } else {
                                parent_node.right = None;
                            }
                        } else {
                            parent_node.right = None;
                        }
                    }
                }
            }
        }
        this_node.left = None;
        this_node.right = None;
        this_node.parent = Weak::new();
    }
    fn delete_node(this:&ChildNodePointer<K,V>,key:K,method:DeleteMethod) -> Option<()> {
        let this_key;
        {
            this_key = this.borrow().key.clone();
        }
        if this_key == key {
            if Self::has_childs(&this) {
                match method {
                    DeleteMethod::Precessor => {
                        let precessor = Self::precessor(&this).unwrap();
                        {
                            let precessor = precessor.borrow();
                            let mut this_node = this.borrow_mut();
                            this_node.key = precessor.key.clone();
                            this_node.value = precessor.value.clone();
                        }
                        Self::delete_with_child(&precessor,DeleteState::MayOnlyLeft);
                    },
                    _ => unreachable!(),
                }
                return Some(());
            } else if Self::has_left(&this) {
                Self::delete_with_child(&this, DeleteState::OnlyLeft);
                return Some(());
            } else {
                Self::delete_with_child(&this, DeleteState::MayOnlyRight);
                return Some(());
            }
        } else if key < this_key {
            let this_left;
            {
                this_left = this.borrow().left.clone();
            }
            match this_left {
                Some(ref left) => Self::delete_node(&left, key, method),
                None => None,
            }
        } else {
            let this_right;
            {
                this_right = this.borrow().right.clone();
            }
            match this_right {
                Some(ref right) => Self::delete_node(&right,key,method),
                None => None,
            }
        }
    }
    fn inorder_collections(this:&ChildNodePointer<K,V>,vec:&mut Vec<K>) {
        if let Some(ref left) = this.borrow().left {
            Self::inorder_collections(&left, vec);
        } 
        vec.push(this.borrow().key.clone());
        if let Some(ref right) = this.borrow().right {
            Self::inorder_collections(&right, vec);
        }
    }
}
#[derive(Debug)]
pub struct BinarySearchTree<K:PartialOrd + Debug + Clone,V:Debug + Clone> {
    root:ChildNode<K,V>,
}
impl<K:PartialOrd + Debug + Clone,V:Debug + Clone> BinarySearchTree<K,V> {
    pub fn new(key:K,value:V) -> Self {
        return BinarySearchTree {
            root:Some(Rc::new(RefCell::new(BinarySearchTreeNode::new(key,value)))),
        }
    }
    pub fn insert(&mut self,key:K,value:V) {
        match self.root {
            Some(ref root) => RawChildNode::insert(&root,RawChildNode::new(key,value)),
            None => {
                self.root = Some(Rc::new(RefCell::new(BinarySearchTreeNode::new(key,value))));
            }
        }
    }
    pub fn maximum(&self) -> Option<(K,V)> {
        if let Some(ref root) = self.root {
            return Some(RawChildNode::maximum(&root));
        }
        return None;
    }
    pub fn minimum(&self) -> Option<(K,V)> {
        if let Some(ref root) = self.root {
            return Some(RawChildNode::minimum(&root));
        }
        return None;
    }
    pub fn precessor(&self) -> Option<ChildNodePointer<K,V>> {
        match self.root {
            Some(ref root) => RawChildNode::precessor(&root),
            None => {
                return None;
            }
        }
    }
    pub fn successor(&self) -> Option<ChildNodePointer<K,V>> {
        match self.root {
            Some(ref root) => RawChildNode::successor(&root),
            None => {
                return None;
            }
        }
    }
    pub fn delete(&mut self,key:K) -> Option<()> {
        match self.root {
            Some(ref root) => {
                return RawChildNode::delete_node(&root,key,DeleteMethod::Precessor);
            },
            None => None,
        }
    }
    pub fn inorder_collections(&self) -> Option<Vec<K>> {
        match self.root {
            Some(ref root) => {
                let mut vec = vec![];
                RawChildNode::inorder_collections(&root,&mut vec);
                return Some(vec);
            },
            None => None,
        }
    }
}