use regex::Regex;
use crate::generate_example;

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

pub(crate) fn to_lines(s: Option<String>) -> Vec<String> {
    let binding = s.clone().unwrap_or("".to_string());
    let lines = binding.lines();

    let mut in_protobuf = false;
    let mut in_language = false;

    let mut new_lines = Vec::<String>::new();

    for line in lines {
        if in_language && !line.starts_with("```") {
            continue
        }
        if in_language {
            in_language = false;
        }

        if (in_protobuf) {
            let example = generate_example(line.to_string()).unwrap();
            new_lines.push("```".to_string());
            new_lines.extend(example.lines().map(|f| f.to_string()).collect::<Vec<_>>());
            new_lines.push("```".to_string());
            in_protobuf = false;
        }

        let l = match line {
            "{{% examples %}}" | "{{% /examples %}}" | "{{% example %}}" | "{{% /example %}}" => {
                vec![]
            }
            "```typescript" |"```python" | "```go" | "```java" | "```yaml" | "```csharp" => {
                in_language = true;
                vec![]
            },
            "```pcl_protobuf" => {
                in_protobuf = true;
                vec![]
            }
            l => vec![l.to_string()],
        };

        new_lines.extend(l);

    }

    new_lines

    // s.unwrap_or("".to_string())
    //     .lines()
    //     .flat_map(|line| match line {
    //         "{{% examples %}}" | "{{% /examples %}}" | "{{% example %}}" | "{{% /example %}}" => {
    //             vec![]
    //         }
    //         "```typescript" => vec!["### Typescript", line],
    //         "```python" => vec!["### Python", line],
    //         "```go" => vec!["### Go", line],
    //         "```java" => vec!["### Java", line],
    //         "```yaml" => vec!["### YAML", line],
    //         "```csharp" => vec!["### C#", line],
    //         l => vec![l],
    //     })
    //     .map(|s| s.to_string())
    //     .collect()
}
