use crate::output::rust::TreeNode;
use convert_case::{Case, Casing};
use rinja::Template;
use std::collections::BTreeMap;

// static TEMPLATE: &str = include_str!("types_mod.rs.handlebars");

// #[derive(Template)] // this will generate the code...
// #[template(path = "hello.html")] // using the template in this path, relative
// // to the `templates` dir in the crate root
// struct HelloTemplate<'a> { // the name of the struct can be anything
//     name: &'a str, // the field name should match the variable name
//     // in your template
// }
//
// #[derive(Serialize)]
// struct RefType {
//     file_name: String,
// }
//
// #[derive(Serialize)]
// struct AliasType {
//     name: String,
//     type_: String,
// }
//
// #[derive(Serialize)]
// struct Package {
//     types: Vec<RefType>,
// }
//
// fn convert_model(package: &crate::model::Package) -> Package {
//     let mut real_types = Vec::new();
//
//     package
//         .types
//         .iter()
//         .for_each(|(element_id, resource)| match resource {
//             GlobalType::Object(_, _) => {
//                 let ref_type = RefType {
//                     file_name: element_id.get_rust_struct_name().to_case(Case::Snake),
//                 };
//                 real_types.push(ref_type);
//             }
//             GlobalType::StringEnum(_, _)
//             | GlobalType::IntegerEnum(_, _)
//             | GlobalType::NumberEnum(_, _) => {
//                 let ref_type = RefType {
//                     file_name: element_id.get_rust_struct_name().to_case(Case::Snake),
//                 };
//                 real_types.push(ref_type);
//             }
//         });
//
//     Package { types: real_types }
// }
//
// pub(crate) fn generate_source_code(package: &crate::model::Package) -> String {
//     let package = convert_model(package);
//
//     let content = {
//         let handlebars = Handlebars::new();
//         handlebars
//             .render_template(TEMPLATE, &json!({"package": &package}))
//             .unwrap()
//     };
//
//     content
// }

#[derive(Template)]
#[template(path = "types_mod.rs.jinja", whitespace = "suppress")]
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
