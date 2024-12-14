#[cfg(test)]
mod tests {
    use pulumi_wasm_provider_common::OneOf2;
    use pulumi_wasm_rust::Output;
    use pulumi_wasm_typesystem::typesystem_server::TypesystemServerArgs;
    use pulumi_wasm_typesystem::{EnumCase1, EnumCase2};
    use std::panic::catch_unwind;

    #[test]
    fn test_compilation() {
        let _ = catch_unwind(compilation_test);
    }

    #[test]
    fn test_deserialization() {
        let case1 = EnumCase1::builder()
            .field_1("value1".to_string())
            .build_struct();
        let case2 = EnumCase2::builder()
            .field_2("value2".to_string())
            .build_struct();

        let case1_json = serde_json::to_string(&case1).unwrap();
        let case2_json = serde_json::to_string(&case2).unwrap();
        assert_eq!(case1_json, r#"{"field1":"value1"}"#);
        assert_eq!(case2_json, r#"{"field2":"value2"}"#);

        let deserialized_case1: EnumCase1 = serde_json::from_str(&case1_json).unwrap();
        assert_eq!(deserialized_case1, case1);

        let deserialized_case2: EnumCase2 = serde_json::from_str(&case2_json).unwrap();
        assert_eq!(deserialized_case2, case2);
    }

    fn compilation_test() {
        // String
        let output = Output::new(&"Hello, World!".to_string());

        let _ = TypesystemServerArgs::builder().required_string_input("&str");
        let _ = TypesystemServerArgs::builder().required_string_input("String".to_string());
        let _ = TypesystemServerArgs::builder().required_string_input(output);

        let _ = TypesystemServerArgs::builder().optional_string_input("&str");
        let _ = TypesystemServerArgs::builder().optional_string_input("String".to_string());
        let _ = TypesystemServerArgs::builder().optional_string_input(output);

        // Vec<String>
        let _ = TypesystemServerArgs::builder().required_string_array(vec!["&str"]);
        let _ = TypesystemServerArgs::builder().required_string_array(vec!["String".to_string()]);
        let _ = TypesystemServerArgs::builder().required_string_array(output.map(|s| vec![s]));
        // let _ = TypesystemServerArgs::builder().required_string_array(vec![string_output]);

        let _ = TypesystemServerArgs::builder().optional_string_array(vec!["&str"]);
        let _ = TypesystemServerArgs::builder().optional_string_array(vec!["String".to_string()]);
        let _ = TypesystemServerArgs::builder().optional_string_array(output.map(|s| vec![s]));
        // let _ = TypesystemServerArgs::builder().optional_string_array(vec![string_output]);

        // Vec<String> with array
        let _ = TypesystemServerArgs::builder().required_string_array(["&str"]);
        let _ = TypesystemServerArgs::builder().required_string_array(["String".to_string()]);
        let _ = TypesystemServerArgs::builder().required_string_array(output.map(|s| vec![s]));
        // let _ = TypesystemServerArgs::builder().required_string_array([string_output]);

        let _ = TypesystemServerArgs::builder().optional_string_array(["&str"]);
        let _ = TypesystemServerArgs::builder().optional_string_array(["String".to_string()]);
        let _ = TypesystemServerArgs::builder().optional_string_array(output.map(|s| vec![s]));
        // let _ = TypesystemServerArgs::builder().optional_string_array([string_output]);

        // Union
        let case1 = EnumCase1::builder()
            .field_1("value1".to_string())
            .build_struct();
        let case2 = EnumCase2::builder()
            .field_2("value2".to_string())
            .build_struct();
        let enum_case1_output = Output::new(&case1);
        let enum_case2_output = Output::new(&case2);
        let _ = TypesystemServerArgs::builder().required_union(OneOf2::left(case1));
        let _ = TypesystemServerArgs::builder().required_union(OneOf2::right(case2));
        let _ = TypesystemServerArgs::builder().required_union(enum_case1_output.map(OneOf2::left));
        let _ =
            TypesystemServerArgs::builder().required_union(enum_case2_output.map(OneOf2::right));

        let case1 = EnumCase1::builder()
            .field_1("value1".to_string())
            .build_struct();
        let case2 = EnumCase2::builder()
            .field_2("value2".to_string())
            .build_struct();
        let _ = TypesystemServerArgs::builder().optional_union(OneOf2::left(case1));
        let _ = TypesystemServerArgs::builder().optional_union(OneOf2::right(case2));
        let _ = TypesystemServerArgs::builder().optional_union(enum_case1_output.map(OneOf2::left));
        let _ =
            TypesystemServerArgs::builder().optional_union(enum_case2_output.map(OneOf2::right));

        // // Other types
        // let _ = TypesystemServerArgs::builder()
        //     .required_string_input(42);
        // let _ = TypesystemServerArgs::builder()
        //     .required_string_input(true);
    }
}
