use std::rc::{Rc,Weak};
use std::cell::RefCell;
use std::fmt::Debug;
pub (super) type ChildNodePointer<K:Debug + Clone + PartialOrd,V:Debug + Clone> = Rc<RefCell<RBNode<K,V>>>;
pub (super) type ChildNode<K:Debug + Clone + PartialOrd,V:Debug + Clone> = Option<Rc<RefCell<RBNode<K,V>>>>;
pub (super) type ParentNode<K:Debug + Clone + PartialOrd,V:Debug + Clone> = Weak<RefCell<RBNode<K,V>>>;
pub (super) enum NodeColor {
    Red,
    Black,
}
pub (super) struct RBNode<K:Debug + Clone + PartialOrd,V:Debug + Clone> {
    pub (super) left:ChildNode<K,V>,
    pub (super) right:ChildNode<K,V>,
    pub (super) parent:ParentNode<K,V>,
    pub (super) color:NodeColor,
    pub (super) key:K,
    pub (super) value:V,
}
pub struct RBTree<K:Debug + Clone + PartialOrd,V:Debug + Clone> {
    pub (super) root:ChildNode<K,V>,
}

impl<K,V> RBNode<K,V>
    where 
        K: Debug + Clone + PartialOrd,
        V: Debug + Clone,
{
    pub (super) fn new(key:K,value:V,color:NodeColor) -> RBNode<K,V> {
        return RBNode {
            left:None,
            right:None,
            parent:Weak::new(),
            color:color,
            key:key,
            value:value,
        };
    }
}

impl<K,V> RBTree<K,V>
    where
        K: Debug + Clone + PartialOrd,
        V: Debug + Clone,
{
    pub fn new(key:K,value:V) -> RBTree<K,V> {
        return RBTree {
            root:Some(Rc::new(RefCell::new(RBNode::new(key,value,NodeColor::Black)))),
        }
    } 
}