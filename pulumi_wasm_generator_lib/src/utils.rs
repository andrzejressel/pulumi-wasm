use crate::code_generation::generate_code_from_string;
use regex::Regex;

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
        s => s,
    }
}

pub(crate) fn to_lines(s: Option<String>, package: &crate::model::Package) -> Vec<String> {
    let binding = s.clone().unwrap_or("".to_string());
    let lines = binding.lines();

    let mut in_yaml = false;
    let mut in_language = false;

    let mut yaml_lines = Vec::<String>::new();
    let mut new_lines = Vec::<String>::new();

    for line in lines {
        if in_yaml && line.trim() == "```" {
            let yaml_str = yaml_lines.join("\n");
            let example = generate_code_from_string(yaml_str, package);

            match example {
                Ok(rust_example) => {
                    new_lines.push("```rust".to_string());
                    new_lines.extend(
                        rust_example
                            .lines()
                            .map(|f| f.to_string())
                            .collect::<Vec<_>>(),
                    );
                    new_lines.push("```".to_string());
                }
                Err(err) => {
                    eprintln!("ERROR: {}", err);
                    err.chain()
                        .skip(1)
                        .for_each(|cause| eprintln!("because: {}", cause));
                    new_lines.push("```yaml".to_string());
                    new_lines.extend(yaml_lines.clone());
                    new_lines.push("```".to_string());
                }
            }
        } else if in_yaml {
            yaml_lines.push(line.to_string());
            continue;
        }

        let l = match line.trim() {
            "{{% examples %}}" | "{{% /examples %}}" | "{{% example %}}" | "{{% /example %}}" => {
                vec![]
            }
            "```yaml" => {
                in_yaml = true;
                vec![]
            }
            "```typescript" | "```python" | "```go" | "```java" | "```csharp" => {
                in_language = true;
                vec![]
            }
            "```" if in_yaml || in_language => {
                in_yaml = false;
                in_language = false;
                yaml_lines.clear();
                vec![]
            }
            _ if in_language || in_yaml => {
                vec![]
            }
            _ => vec![line.to_string()],
        };

        new_lines.extend(l);
    }

    new_lines
}

fn generate_example(p0: String) -> String {
    "RUST EXAMPLE".to_string()
}
