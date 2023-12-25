// binary tree e meios de travesias
use std::cmp::Ordering;

struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: i32) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(left) = &mut self.left {
                    left.insert(value);
                } else {
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Greater => {
                if let Some(right) = &mut self.right {
                    right.insert(value);
                } else {
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Equal => { /* Handle duplicate values as needed */ }
        }
    }

    fn pre_order_traversal(&self) {
        println!("{}", self.value);
        if let Some(left) = &self.left {
            left.pre_order_traversal();
        }
        if let Some(right) = &self.right {
            right.pre_order_traversal();
        }
    }

    fn in_order_traversal(&self) {
        if let Some(left) = &self.left {
            left.in_order_traversal();
        }
        println!("{}", self.value);
        if let Some(right) = &self.right {
            right.in_order_traversal();
        }
    }

    fn post_order_traversal(&self) {
        if let Some(left) = &self.left {
            left.post_order_traversal();
        }
        if let Some(right) = &self.right {
            right.post_order_traversal();
        }
        println!("{}", self.value);
    }
}

fn main() {
    let mut root = TreeNode::new(3);
    let values = vec![1, 4, 2, 5];

    for &value in &values {
        root.insert(value);
    }

    println!("Pre-order traversal:");
    root.pre_order_traversal();

    println!("In-order traversal:");
    root.in_order_traversal();

    println!("Post-order traversal:");
    root.post_order_traversal();
}
