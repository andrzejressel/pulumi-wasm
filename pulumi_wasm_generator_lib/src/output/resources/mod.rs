mod source_code_resource_code;

use crate::model::{ElementId, Package};
use crate::output::resources::source_code_resource_code::{
    generate_docs, generate_single_resource_source_code,
};
use crate::output::TreeNode;
use crate::utils::reformat_code;
use anyhow::Context;
use convert_case::{Case, Casing};
use std::fs::File;
use std::io::Write;

pub(crate) fn generate_single_file(package: &Package, element_id: &ElementId) -> String {
    reformat_code(&generate_single_resource_source_code(package, element_id))
        .context("Failed to reformat resource source code")
        .unwrap()
}

pub(crate) fn generate_single_file_docs(package: &Package, element_id: &ElementId) -> Vec<String> {
    generate_docs(package, element_id)
}

pub(crate) fn generate_resources_code(
    package: &crate::model::Package,
    result_path: &std::path::Path,
) {
    if package.resources.is_empty() {
        return;
    }
    let mut tree = TreeNode::new();

    for element_id in package.resources.keys() {
        tree.insert(element_id.clone());
    }

    println!("Tree {:?}", tree);

    generate_files(package, &tree, result_path);
}

fn generate_files(
    package: &crate::model::Package,
    tree_node: &TreeNode,
    current_path: &std::path::Path,
) {
    match tree_node {
        TreeNode::Namespace(ns, resources) => {
            std::fs::create_dir_all(current_path).unwrap();
            for (name, node) in ns {
                generate_files(package, node, &current_path.join(name));
            }

            for resource in resources {
                let file_name = format!(
                    "{}.rs",
                    resource.get_rust_struct_name().to_case(Case::Snake)
                );
                let content = generate_single_resource_source_code(package, resource);
                let mut file = File::create(current_path.join(file_name)).unwrap();
                file.write_all(content.as_bytes()).unwrap();
            }
        }
    }
}

pub(crate) fn generate_module_imports(package: &Package) -> String {
    if package.resources.is_empty() {
        return "".to_string();
    }
    let mut tree = TreeNode::new();

    for element_id in package.resources.keys() {
        tree.insert(element_id.clone());
    }

    generate_module_imports_1(package, &tree, std::path::Path::new("resources"))
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
                    "pub mod {}{{\n",
                    function.get_rust_struct_name().to_case(Case::Snake)
                ));
                s.push_str(&format!(
                    "include!(\"{}/{}.rs\");\n",
                    current_path.to_str().unwrap().replace("\\", "/"),
                    function.get_rust_struct_name().to_case(Case::Snake)
                ));
                s.push_str("}\n");
            }

            s
        }
    }
}
