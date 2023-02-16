use std::{fmt::{Display, Formatter, Result}, cmp::Ordering};

struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    size: i32,
}

impl Node {
    fn new(val: i32) -> Node {
        Node {
            val: val,
            left: None,
            right: None,
            size: 1,
        }
    }

    //Iterative insert using cmp ordering to determine where the value should go in the tree (left or right)
    fn insert(&mut self, val: i32) {
        let mut curr = self;
        loop {
            curr = match val.cmp(&curr.val) {
                Ordering::Less => {curr.size += 1; curr.left.get_or_insert(Box::new(Node::new(val)))},
                Ordering::Greater => {curr.size += 1; curr.right.get_or_insert(Box::new(Node::new(val)))},
                Ordering::Equal => return,
            };
        }
        
    }

    fn search(&self, val: i32) -> bool {
        let mut curr = Some(self);
        while let Some(node) = curr {
            if val == node.val {
                return true;
            } else if val < node.val {
                curr = node.left.as_ref().map(|n| &**n);
            } else {
                curr = node.right.as_ref().map(|n| &**n);
            }
        }
        false
    }

    fn size(&self) -> i32 {
        let mut count = 1;
        if let Some(left) = &self.left {
            count += left.size();
        }
        if let Some(right) = &self.right {
            count += right.size();
        }
        count
    }

    fn height(&self) -> i32 {
        let mut left_height = 0;
        let mut right_height = 0;
        if let Some(left) = &self.left {
            left_height = left.height() + 1;
        }
        if let Some(right) = &self.right {
            right_height = right.height() + 1;
        }
        left_height.max(right_height)
    }

    fn leaves(&self) -> i32 {
        if self.left.is_none() && self.right.is_none() {
            1
        } else {
            let mut count = 0;
            if let Some(left) = &self.left {
                count += left.leaves();
            }
            if let Some(right) = &self.right {
                count += right.leaves();
            }
            count
        }
    }

    fn to_string_helper(&self, output: &mut String) {
        if let Some(left) = &self.left {
            left.to_string_helper(output);
            output.push_str(", ");
        }
        output.push_str(&self.val.to_string());
        if let Some(right) = &self.right {
            output.push_str(", ");
            right.to_string_helper(output);
        }
    }
}

pub struct BinaryTree {
    root: Option<Box<Node>>,
}

impl BinaryTree {
    pub fn new() -> BinaryTree {
        BinaryTree { root: None }
    }

    pub fn insert(&mut self, val: i32) {
        match &mut self.root {
            Some(root) => root.insert(val),
            None => self.root = Some(Box::new(Node::new(val))),
        }
    }

    pub fn search(&self, val: i32) -> bool {
        match &self.root {
            Some(root) => root.search(val),
            None => false,
        }
    }

    pub fn size(&self) -> i32 {
        match &self.root {
            Some(root) => root.size(),
            None => 0,
        }
    }

    pub fn height(&self) -> i32 {
        match &self.root {
            Some(root) => root.height(),
            None => 0,
        }
    }

    pub fn leaves(&self) -> i32 {
        match &self.root {
            Some(root) => root.leaves(),
            None => 0,
        }
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();
        output.push('[');
        if let Some(root) = &self.root {
            root.to_string_helper(&mut output);
        }
        output.push(']');
        output
    }
}



fn main() {
    let mut tree = BinaryTree::new();
    tree.insert(3);
    tree.insert(2);
    tree.insert(1);
    println!("tree: {}", tree.to_string());
    println!("size: {}", tree.size());
    println!("height: {}", tree.height());
}