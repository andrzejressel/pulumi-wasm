use crate::model::{ElementId, Package};
use crate::output::rust::functions::source_code_function_code::generate_single_function_source_code;
use crate::output::rust::functions::source_code_function_mod::generate_mod;
use crate::output::rust::TreeNode;
use convert_case::{Case, Casing};
use std::fs::File;
use std::io::Write;

mod source_code_function_code;
mod source_code_function_mod;

pub(crate) fn generate_function_code(package: &Package, result_path: &std::path::Path) {
    if package.functions.is_empty() {
        return;
    }
    let mut tree = TreeNode::new();

    for element_id in package.functions.keys() {
        tree.insert(element_id.clone());
    }

    let root = result_path.join("src").join("function");

    generate_files(package, &tree, &root);
}

fn generate_files(package: &Package, tree_node: &TreeNode, current_path: &std::path::Path) {
    match tree_node {
        TreeNode::Namespace(ns) => {
            for (name, node) in ns {
                generate_files(package, node, &current_path.join(name));
            }
            let content = generate_mod(ns);
            let mut file = File::create(current_path.join("mod.rs")).unwrap();
            file.write_all(content.as_bytes()).unwrap();
        }
        TreeNode::Leaf(ref leaf) => {
            let file_name = format!("{}.rs", leaf.get_rust_struct_name().to_case(Case::Snake));
            let content = generate_single_function_source_code(package, leaf);
            let dir = current_path.parent().unwrap();
            std::fs::create_dir_all(dir).unwrap();
            let mut file = File::create(current_path.parent().unwrap().join(file_name)).unwrap();
            file.write_all(content.as_bytes()).unwrap();
        }
    }
}
