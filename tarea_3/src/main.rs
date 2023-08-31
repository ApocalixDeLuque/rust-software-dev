use std::io;

struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode { value, left: None, right: None }
    }

    fn insert(&mut self, value: i32) {
        if value < self.value {
            if let Some(left) = &mut self.left {
                left.insert(value);
            } else {
                self.left = Some(Box::new(TreeNode::new(value)));
            }
        } else {
            if let Some(right) = &mut self.right {
                right.insert(value);
            } else {
                self.right = Some(Box::new(TreeNode::new(value)));
            }
        }
    }

    fn gather_leaves(&self) -> Vec<i32> {
        let mut leaves = Vec::new();
        if self.left.is_none() && self.right.is_none() {
            leaves.push(self.value);
        }
        if let Some(left) = &self.left {
            leaves.extend(left.gather_leaves());
        }
        if let Some(right) = &self.right {
            leaves.extend(right.gather_leaves());
        }
        leaves
    }
}

fn main() {
    let mut input_str = String::new();
    println!("Ingrese los valores del árbol separados por espacios:");
    io::stdin().read_line(&mut input_str).unwrap();
    let input: Vec<i32> = input_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
        
    let mut root = TreeNode::new(input[0]);

    for &value in &input[1..] {
        root.insert(value);
    }

    let mut leaves = root.gather_leaves();
    leaves.sort();

    println!("{}", leaves.len());
    println!("{}", leaves.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}
