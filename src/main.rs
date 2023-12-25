mod binary_tree;

use binary_tree::TreeNode;

fn main() {
    let mut root = TreeNode::new(3);
    let values = vec![1, 4, 2, 5];

    for &value in &values {
        root.insert(value);
    }

    println!("Pre-order traversal:");
    let mut pre_order_result = Vec::new();
    root.pre_order_traversal_with_collector(&mut pre_order_result);
    println!("{:?}", pre_order_result);

    println!("In-order traversal:");
    let mut in_order_result = Vec::new();
    root.in_order_traversal_with_collector(&mut in_order_result);
    println!("{:?}", in_order_result);

    println!("Post-order traversal:");
    let mut post_order_result = Vec::new();
    root.post_order_traversal_with_collector(&mut post_order_result);
    println!("{:?}", post_order_result);
}
