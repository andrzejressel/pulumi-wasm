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

pub(crate) fn to_lines(s: Option<String>) -> Vec<String> {
    s.unwrap_or("".to_string())
        .lines()
        .flat_map(|line| match line {
            "{{% examples %}}" | "{{% /examples %}}" | "{{% example %}}" | "{{% /example %}}" => {
                vec![]
            }
            "```typescript" => vec!["### Typescript", line],
            "```python" => vec!["### Python", line],
            "```go" => vec!["### Go", line],
            "```java" => vec!["### Java", line],
            "```yaml" => vec!["### YAML", line],
            "```csharp" => vec!["### C#", line],
            l => vec![l],
        })
        .map(|s| s.to_string())
        .collect()
}
