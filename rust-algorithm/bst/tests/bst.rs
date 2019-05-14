use bst::bst::BinarySearchTree;
#[cfg(test)]
#[test]
fn bst_delete_test() {
    let mut tree = BinarySearchTree::new(7,"Rust");
    tree.insert(1,"is");
    tree.insert(5,"Good");
    tree.insert(6,"Programing");
    tree.insert(8,"Language");
    tree.insert(10,"?");
    tree.insert(9,"!");
    assert_eq!(tree.inorder_collections().unwrap(),vec![1,5,6,7,8,9,10]);
    //delete root with childs
    let _ = tree.delete(7);
    assert_eq!(tree.inorder_collections().unwrap(),vec![1,5,6,8,9,10]);
    //delete with left child
    let _ = tree.delete(10);
    assert_eq!(tree.inorder_collections().unwrap(),vec![1,5,6,8,9]);
    //delete with right child
    let _ = tree.delete(1);
    assert_eq!(tree.inorder_collections().unwrap(),vec![5,6,8,9]);
}