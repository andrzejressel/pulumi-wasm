use crate::model::ElementId;
use std::collections::{BTreeMap, HashMap};
use std::ops::{Deref, DerefMut};
use std::sync::Mutex;

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

    // Insert an ElementId into the tree
    // pub(crate) fn insert(&self, element: ElementId) {
    //     match self {
    //         TreeNode::Namespace(children) => {
    //             let mut current_node = self;
    //             let mut mutexes = Vec::new();
    //             for segment in element.namespace.iter() {
    //                 let lock = children.lock().unwrap();
    //                 current_node = lock
    //                     .deref()
    //                     .entry(segment.clone())
    //                     .or_insert_with(TreeNode::new);
    //                 mutexes.push(lock);
    //             }
    //
    //             let t = Vec::new();
    //
    //             // Add the ElementId as a leaf at the final namespace level
    //             if let TreeNode::Namespace(children) = current_node {
    //                 children.lock().unwrap().deref_mut().insert(element.name.clone(), TreeNode::Leaf(element));
    //             }
    //         }
    //         TreeNode::Leaf(_) => panic!("Cannot insert into a leaf node!"),
    //     }
    // }

    pub(crate) fn insert(&mut self, element: ElementId) {
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

pub(crate) fn generate_types_code(package: &crate::model::Package) {
    let mut tree = TreeNode::new();

    for (element_id, _) in &package.types {
        tree.insert(element_id.clone());
    }

    println!("Tree {:?}", tree)
}
