#[cfg(test)]
mod tests {
    use pulumi_wasm_provider_common::OneOf2;
    use pulumi_wasm_rust::Output;
    use pulumi_wasm_typesystem::typesystem_server::typesystemServerArgs;
    use pulumi_wasm_typesystem::{EnumCase1, EnumCase2};
    use std::panic::catch_unwind;

    #[test]
    fn test_compilation() {
        let _ = catch_unwind(|| compilation_test());
    }

    fn compilation_test() {
        let string_output = Output::new(&"Hello, World!".to_string());

        // String
        let _ = typesystemServerArgs::builder().required_string_input("&str");
        let _ = typesystemServerArgs::builder().required_string_input("String".to_string());
        let _ = typesystemServerArgs::builder().required_string_input(string_output);

        let _ = typesystemServerArgs::builder().optional_string_input("&str");
        let _ = typesystemServerArgs::builder().optional_string_input("String".to_string());
        let _ = typesystemServerArgs::builder().optional_string_input(string_output);

        // Vec<String>
        let _ = typesystemServerArgs::builder().required_string_array(vec!["&str"]);
        let _ = typesystemServerArgs::builder().required_string_array(vec!["String".to_string()]);
        let _ = typesystemServerArgs::builder().required_string_array(string_output.map(|s| vec![s]));
        // let _ = typesystemServerArgs::builder().required_string_array(vec![string_output]);

        let _ = typesystemServerArgs::builder().optional_string_array(vec!["&str"]);
        let _ = typesystemServerArgs::builder().optional_string_array(vec!["String".to_string()]);
        let _ = typesystemServerArgs::builder().optional_string_array(string_output.map(|s| vec![s]));
        // let _ = typesystemServerArgs::builder().optional_string_array(vec![string_output]);

        // Vec<String> with array
        let _ = typesystemServerArgs::builder().required_string_array(["&str"]);
        let _ = typesystemServerArgs::builder().required_string_array(["String".to_string()]);
        let _ = typesystemServerArgs::builder().required_string_array(string_output.map(|s| vec![s]));
        // let _ = typesystemServerArgs::builder().required_string_array([string_output]);

        let _ = typesystemServerArgs::builder().optional_string_array(["&str"]);
        let _ = typesystemServerArgs::builder().optional_string_array(["String".to_string()]);
        let _ = typesystemServerArgs::builder().optional_string_array(string_output.map(|s| vec![s]));
        // let _ = typesystemServerArgs::builder().optional_string_array([string_output]);

        // Union
        let _ = typesystemServerArgs::builder().required_union(OneOf2::left(EnumCase1 {}));
        let _ = typesystemServerArgs::builder().required_union(OneOf2::right(EnumCase2 {}));
        let _ = typesystemServerArgs::builder().optional_union(OneOf2::left(EnumCase1 {}));
        let _ = typesystemServerArgs::builder().optional_union(OneOf2::right(EnumCase2 {}));

        // // Other types
        // let _ = typesystemServerArgs::builder()
        //     .required_string_input(42);
        // let _ = typesystemServerArgs::builder()
        //     .required_string_input(true);
    }
}
