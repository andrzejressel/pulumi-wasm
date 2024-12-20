use crate::model::ElementId;
use std::collections::BTreeMap;
use std::fs::File;
use std::ops::{Deref, DerefMut};
use crate::output::rust::source_code_types_code::generate_single_source_file;
use std::io::{BufReader, Write};
use std::path::Path;

pub(crate) mod cargo;
pub(crate) mod source_code_function_code;
pub(crate) mod source_code_function_mod;
pub(crate) mod source_code_librs;
pub(crate) mod source_code_resource_code;
pub(crate) mod source_code_resource_mod;
pub(crate) mod source_code_types_code;
pub(crate) mod source_code_types_mod;

#[derive(Debug)]
enum TreeNode {
    Namespace(BTreeMap<String, TreeNode>),
    Leaf(ElementId),
}

impl TreeNode {
    // Create an empty tree
    pub(crate) fn new() -> Self {
        TreeNode::Namespace(BTreeMap::new())
    }

    fn insert(&mut self, element: ElementId) {
        self.insert_priv(element, 0);
    }

    fn insert_priv(&mut self, element: ElementId, index: usize) {
        match self {
            TreeNode::Namespace(children) => {
                if element.namespace.len() == index {
                    if children.contains_key(&element.name) {
                        panic!("Duplicate element: {:?}", element); // Replace with error handling code
                    }
                    children.insert(element.name.clone(), TreeNode::Leaf(element.clone()));
                } else {
                    let next_node = children
                        .entry(element.namespace[index].clone())
                        .or_insert_with(TreeNode::new);
                    next_node.insert_priv(element, index + 1);
                }
            }
            TreeNode::Leaf(_) => panic!("Cannot insert into a leaf node!"),
        }
    }
}

pub(crate) fn generate_types_code(package: &crate::model::Package, result_path: &std::path::Path) {
    let mut tree = TreeNode::new();

    for (element_id, _) in &package.types {
        println!("{:?}", element_id);
        tree.insert(element_id.clone());
    }

    let root = result_path.join("src").join("types");
    println!("Tree {:?}", tree);

    generate_files(package, tree, &root);

    // let mut lib_file =
    //     File::create(result_path.join("src").join("types").join(path)).unwrap();

}

pub(crate) fn generate_files(package: &crate::model::Package, tree_node: TreeNode, current_path: &std::path::Path) {
    match tree_node {
        TreeNode::Namespace(ns) => {
            for (name, node) in ns {
                let new_path = current_path.join(name);
                std::fs::create_dir_all(&new_path).unwrap();
                generate_files(package, node, &new_path);
            }
        }
        TreeNode::Leaf(ref leaf) => {
            let (file_name, content) = generate_single_source_file(package, leaf);
            let mut file = File::create(current_path.join(file_name)).unwrap();
            file.write_all(content.as_bytes()).unwrap();
        }
    }
}