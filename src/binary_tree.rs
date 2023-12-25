use std::cmp::Ordering;

#[derive(Debug)]
pub struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(value: i32) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, value: i32) {
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

    pub fn pre_order_traversal_with_collector(&self, result: &mut Vec<i32>) {
        result.push(self.value);
        if let Some(left) = &self.left {
            left.pre_order_traversal_with_collector(result);
        }
        if let Some(right) = &self.right {
            right.pre_order_traversal_with_collector(result);
        }
    }

    pub fn in_order_traversal_with_collector(&self, result: &mut Vec<i32>) {
        if let Some(left) = &self.left {
            left.in_order_traversal_with_collector(result);
        }
        result.push(self.value);
        if let Some(right) = &self.right {
            right.in_order_traversal_with_collector(result);
        }
    }

    pub fn post_order_traversal_with_collector(&self, result: &mut Vec<i32>) {
        if let Some(left) = &self.left {
            left.post_order_traversal_with_collector(result);
        }
        if let Some(right) = &self.right {
            right.post_order_traversal_with_collector(result);
        }
        result.push(self.value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_tree_traversals() {
        let mut root = TreeNode::new(3);
        let values = vec![1, 4, 2, 5];

        for &value in &values {
            root.insert(value);
        }

        let mut pre_order_result = Vec::new();
        root.pre_order_traversal_with_collector(&mut pre_order_result);
        assert_eq!(pre_order_result, vec![3, 1, 2, 4, 5]);

        let mut in_order_result = Vec::new();
        root.in_order_traversal_with_collector(&mut in_order_result);
        assert_eq!(in_order_result, vec![1, 2, 3, 4, 5]);

        let mut post_order_result = Vec::new();
        root.post_order_traversal_with_collector(&mut post_order_result);
        assert_eq!(post_order_result, vec![2, 1, 5, 4, 3]);
    }
}
