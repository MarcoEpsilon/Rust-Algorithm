use rbtree::rbtree::RBTree;
use rbtree::rbtree::debug;
fn main() {
    let mut root = RBTree::new(6,"hello");
    root.insert(3,"rust");
    root.insert(8,"lala");
    root.insert(5,"ooo");
    root.insert(1,"aaa");
    root.insert(9,"hahaha");
    root.insert(15,"bbbb");
    root.insert(16,"ddddd");
    root.insert(19,"1111");
    root.inorder_debug();
}
