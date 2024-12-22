use crate::model::ElementId;
use std::collections::BTreeMap;

pub(crate) mod cargo;
pub(crate) mod functions;
pub(crate) mod resources;
pub(crate) mod source_code_function_code;
pub(crate) mod source_code_function_mod;
pub(crate) mod source_code_librs;
pub(crate) mod types;

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
