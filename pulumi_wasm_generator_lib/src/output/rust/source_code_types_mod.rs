use crate::output::rust::TreeNode;
use convert_case::{Case, Casing};
use rinja::Template;
use std::collections::BTreeMap;

#[derive(Template)]
#[template(path = "types_mod.rs.jinja")]
struct TemplateModel {
    files: Vec<String>,
    directories: Vec<String>,
}

pub(crate) fn generate_mod(tree_nodes: &BTreeMap<String, TreeNode>) -> String {
    let mut files = Vec::new();
    let mut directories = Vec::new();

    for (name, tree_node) in tree_nodes {
        match tree_node {
            TreeNode::Namespace(_) => directories.push(name.clone()),
            TreeNode::Leaf(l) => {
                files.push(l.get_rust_struct_name().to_case(Case::Snake));
            }
        }
    }

    TemplateModel { files, directories }
        .render()
        .unwrap()
        .trim()
        .to_string()
        + "\n"
}
