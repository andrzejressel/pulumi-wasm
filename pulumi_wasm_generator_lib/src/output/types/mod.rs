mod source_code_types_code;

use crate::model::{ElementId, Package};
use crate::output::types::source_code_types_code::{
    generate_docs, generate_single_type_source_file,
};
use crate::output::TreeNode;
use crate::utils::reformat_code;
use anyhow::Context;
use convert_case::{Case, Casing};
use std::fs::File;
use std::io::Write;

pub(crate) fn generate_single_file(package: &Package, element_id: &ElementId) -> String {
    reformat_code(&generate_single_type_source_file(package, element_id))
        .context("Failed to generate type source code")
        .unwrap()
}

pub(crate) fn generate_single_file_docs(package: &Package, element_id: &ElementId) -> Vec<String> {
    generate_docs(package, element_id)
}

pub(crate) fn generate_types_code(package: &Package, result_path: &std::path::Path) {
    if package.types.is_empty() {
        return;
    }
    let mut tree = TreeNode::new();

    for element_id in package.types.keys() {
        tree.insert(element_id.clone());
    }

    generate_files(package, &tree, result_path);
}

fn generate_files(package: &Package, tree_node: &TreeNode, current_path: &std::path::Path) {
    match tree_node {
        TreeNode::Namespace(ns, types) => {
            std::fs::create_dir_all(current_path).unwrap();
            for (name, node) in ns {
                generate_files(package, node, &current_path.join(name));
            }

            for type_ in types {
                let file_name = format!("{}.rs", type_.get_rust_struct_name().to_case(Case::Snake));
                let content = generate_single_type_source_file(package, type_);
                let mut file = File::create(current_path.join(file_name)).unwrap();
                file.write_all(content.as_bytes()).unwrap();
            }
        }
    }
}

pub(crate) fn generate_module_imports(package: &Package) -> String {
    if package.types.is_empty() {
        return "".to_string();
    }
    let mut tree = TreeNode::new();

    for element_id in package.types.keys() {
        tree.insert(element_id.clone());
    }

    generate_module_imports_1(package, &tree, std::path::Path::new("types"))
}

fn generate_module_imports_1(
    package: &Package,
    tree_node: &TreeNode,
    current_path: &std::path::Path,
) -> String {
    match tree_node {
        TreeNode::Namespace(ns, functions) => {
            let mut s = String::new();

            for (name, node) in ns {
                s.push_str(&format!("pub mod {}{{\n", name));
                s.push_str(&generate_module_imports_1(
                    package,
                    node,
                    &current_path.join(name),
                ));
                s.push_str("}\n");
            }

            for function in functions {
                s.push_str(&format!(
                    "include!(\"{}/{}.rs\");\n",
                    current_path.to_str().unwrap().replace("\\", "/"),
                    function.get_rust_struct_name().to_case(Case::Snake)
                ));
            }

            s
        }
    }
}