mod source_code_types_code;

use crate::model::Package;
use crate::output::types::source_code_types_code::generate_single_type_source_file;
use crate::output::TreeNode;
use convert_case::{Case, Casing};
use std::fs::{File, FileTimes};
use std::io::Write;
use std::time::SystemTime;

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
                let times = FileTimes::new()
                    .set_modified(SystemTime::UNIX_EPOCH);
                file.set_times(times).unwrap();
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

    generate_module_imports_1(&tree, std::path::Path::new("types"))
}

fn generate_module_imports_1(tree_node: &TreeNode, current_path: &std::path::Path) -> String {
    match tree_node {
        TreeNode::Namespace(ns, functions) => {
            let mut s = String::new();

            for (name, node) in ns {
                s.push_str(&format!("pub mod {}{{\n", name));
                s.push_str(&generate_module_imports_1(node, &current_path.join(name)));
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
