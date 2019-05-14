use bst::bst::BinarySearchTree;
fn main() {
    let mut tree = BinarySearchTree::new(7,"root");
    tree.insert(1,"macro");
    tree.insert(5,"kkekiid");
    tree.insert(8,"eedd");
    tree.insert(6,"33333");
    tree.insert(10,"1111");
    let result = tree.inorder_collections().unwrap();
    for i in result.iter() {
        println!("{:?}",i);
    }
    println!("{:?}",tree.precessor());
    //println!("{:?}",tree);
    /*println!("{:?}",tree.maximum());
    println!("{:?}",tree.precessor());
    println!("{:?}",tree.successor());*/
    println!("{:?}",tree.delete(7));
    println!("{:?}",tree.delete(5));
    println!("{:?}",tree.delete(8));
    println!("{:?}",tree.precessor());
    println!("{:?}",tree.successor());
    println!("{:?}",tree.maximum());
    println!("{:?}",tree.minimum());
    //println!("{:?}",tree);
}
