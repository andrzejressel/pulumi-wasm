mod source_code_types_code;
mod source_code_types_mod;

use crate::output::rust::types::source_code_types_code::generate_single_type_source_file;
use crate::output::rust::types::source_code_types_mod::generate_mod;
use crate::output::rust::TreeNode;
use convert_case::{Case, Casing};
use std::fs::File;
use std::io::Write;

pub(crate) fn generate_types_code(package: &crate::model::Package, result_path: &std::path::Path) {
    if package.types.is_empty() {
        return;
    }
    let mut tree = TreeNode::new();

    for element_id in package.types.keys() {
        tree.insert(element_id.clone());
    }

    let root = result_path.join("src").join("types");

    generate_files(package, &tree, &root);
}

fn generate_files(
    package: &crate::model::Package,
    tree_node: &TreeNode,
    current_path: &std::path::Path,
) {
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
            let content = generate_single_type_source_file(package, leaf);
            let dir = current_path.parent().unwrap();
            std::fs::create_dir_all(dir).unwrap();
            let mut file = File::create(current_path.parent().unwrap().join(file_name)).unwrap();
            file.write_all(content.as_bytes()).unwrap();
        }
    }
}
