use crate::description::Description;
use crate::model::ElementId;
use anyhow::Context;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::sync::LazyLock;

pub(crate) fn replace_multiple_dashes(s: &str) -> String {
    let re = Regex::new("-+").unwrap();
    let result = re.replace_all(s, "-");
    result.to_string()
}

pub(crate) fn escape_rust_name(name: &str) -> &str {
    match name {
        // Escape Rust keywords.
        // Source: https://doc.rust-lang.org/reference/keywords.html
        "as" => "as_",
        "break" => "break_",
        "const" => "const_",
        "continue" => "continue_",
        "crate" => "crate_",
        "else" => "else_",
        "enum" => "enum_",
        "extern" => "extern_",
        "false" => "false_",
        "fn" => "fn_",
        "for" => "for_",
        "if" => "if_",
        "impl" => "impl_",
        "in" => "in_",
        "let" => "let_",
        "loop" => "loop_",
        "match" => "match_",
        "mod" => "mod_",
        "move" => "move_",
        "mut" => "mut_",
        "pub" => "pub_",
        "ref" => "ref_",
        "return" => "return_",
        "self" => "self_",
        "static" => "static_",
        "struct" => "struct_",
        "super" => "super_",
        "trait" => "trait_",
        "true" => "true_",
        "type" => "type_",
        "unsafe" => "unsafe_",
        "use" => "use_",
        "where" => "where_",
        "while" => "while_",
        "async" => "async_",
        "await" => "await_",
        "dyn" => "dyn_",
        "abstract" => "abstract_",
        "become" => "become_",
        "box" => "box_",
        "do" => "do_",
        "final" => "final_",
        "macro" => "macro_",
        "override" => "override_",
        "priv" => "priv_",
        "typeof" => "typeof_",
        "unsized" => "unsized_",
        "virtual" => "virtual_",
        "yield" => "yield_",
        "try" => "try_",
        s => s,
    }
}

pub(crate) fn escape_wit_identifier(s: &str) -> &str {
    match s {
        "flags" => "%flags",
        "list" => "%list",
        "result" => "%result",
        "record" => "%record",
        "type" => "%type",
        "resource" => "%resource",
        s => s,
    }
}

pub(crate) fn sanitize_identifier(input: &str) -> String {
    // Filter characters that are valid for an identifier in Rust
    input
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_') // Keep letters, digits, and underscores
        .collect()
}

pub(crate) fn to_lines(
    s: Option<String>,
    package: &crate::model::Package,
    element_id: Option<ElementId>,
) -> Vec<String> {
    let binding = s
        .clone()
        .map(|s| fix_pulumi_docker_docs(s, element_id))
        .unwrap_or("".to_string());
    let lines = binding.lines();

    let mut description = Description::create(package);

    for line in lines {
        description.transition(line)
    }

    description.get()
}

static DOCKER_SERVICE_REPLACEMENTS: LazyLock<HashMap<ElementId, Vec<(&str, &str)>>> =
    LazyLock::new(|| {
        HashMap::from([
            (
                ElementId::new("docker:index/service:Service").unwrap(),
                vec![(
                    include_str!("dockerfixes/service/1_original.md"),
                    include_str!("dockerfixes/service/1_fixed.md"),
                )],
            ),
            (
                ElementId::new("docker:index/getPlugin:getPlugin").unwrap(),
                vec![(
                    include_str!("dockerfixes/getPlugin/1_original.md"),
                    include_str!("dockerfixes/getPlugin/1_fixed.md"),
                )],
            ),
            (
                ElementId::new("docker:index/network:Network").unwrap(),
                vec![(
                    include_str!("dockerfixes/network/1_original.md"),
                    include_str!("dockerfixes/network/1_fixed.md"),
                )],
            ),
            (
                ElementId::new("docker:index/secret:Secret").unwrap(),
                vec![(
                    include_str!("dockerfixes/secret/1_original.md"),
                    include_str!("dockerfixes/secret/1_fixed.md"),
                )],
            ),
            (
                ElementId::new("docker:index/serviceConfig:ServiceConfig").unwrap(),
                vec![(
                    include_str!("dockerfixes/serviceConfig/1_original.md"),
                    include_str!("dockerfixes/serviceConfig/1_fixed.md"),
                )],
            ),
            (
                ElementId::new("docker:index/container:Container").unwrap(),
                vec![(
                    include_str!("dockerfixes/container/1_original.md"),
                    include_str!("dockerfixes/container/1_fixed.md"),
                )],
            ),
        ])
    });

fn fix_pulumi_docker_docs(s: String, element_id: Option<ElementId>) -> String {
    if let Some(id) = element_id {
        let replacement = &DOCKER_SERVICE_REPLACEMENTS;
        if let Some(replacements) = replacement.get(&id) {
            for (origin, fixed) in replacements {
                if s.contains(origin) {
                    return fixed.to_string();
                }
            }
            fs::write("error.md", s).unwrap();
            panic!("ElementId {:?} does not have valid replacement. Original markdown was saved to error.md", id);
        }
    }

    s
}

pub(crate) fn reformat_code(code: &str) -> anyhow::Result<String> {
    let syntax_tree = syn::parse_file(code)
        .with_context(|| code.to_string())
        .with_context(|| "Failed to parse code".to_string())?;
    Ok(prettyplease::unparse(&syntax_tree))
}

pub(crate) fn get_main_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
