fn maxDepth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = maxDepth(node.left);
            let right_depth = maxDepth(node.right);
            cmp::max(left_depth, right_depth) + 1
        }
        None => 0,
    }
}

fn main() {
    // Considering that binary tree is allready created
    let depth = maxDepth(Some(Box::new(root)));
    println!("Maximum depth of the tree: {}", depth); 
}