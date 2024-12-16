#[cfg(test)]
mod tests {
    use pulumi_wasm_provider_common::OneOf2;
    use pulumi_wasm_rust::Output;
    use pulumi_wasm_typesystem::typesystem_server::TypesystemServerArgs;
    use pulumi_wasm_typesystem::{
        MyEnum, UnionCase1, UnionCase2, UnionCaseWithConst1, UnionCaseWithConst2,
    };
    use std::panic::catch_unwind;

    #[test]
    fn test_compilation() {
        let _ = catch_unwind(compilation_test);
    }

    #[test]
    fn test_case_deserialization() {
        let case1 = UnionCase1::builder()
            .field_1("value1".to_string())
            .build_struct();
        let case2 = UnionCase2::builder()
            .field_2("value2".to_string())
            .build_struct();

        let case1_json = serde_json::to_string(&case1).unwrap();
        let case2_json = serde_json::to_string(&case2).unwrap();
        assert_eq!(case1_json, r#"{"field1":"value1"}"#);
        assert_eq!(case2_json, r#"{"field2":"value2"}"#);

        let deserialized_case1: UnionCase1 = serde_json::from_str(&case1_json).unwrap();
        let deserialized_case2: UnionCase2 = serde_json::from_str(&case2_json).unwrap();
        assert_eq!(deserialized_case1, case1);
        assert_eq!(deserialized_case2, case2);
    }

    #[test]
    fn test_enum_deserialization() {
        let enum1 = MyEnum::Value1;
        let enum2 = MyEnum::Value2;

        let enum1_json = serde_json::to_string(&enum1).unwrap();
        let enum2_json = serde_json::to_string(&enum2).unwrap();
        assert_eq!(enum1_json, r#""VALUE1""#);
        assert_eq!(enum2_json, r#""Value2""#);

        let deserialized_enum1: MyEnum = serde_json::from_str(&enum1_json).unwrap();
        let deserialized_enum2: MyEnum = serde_json::from_str(&enum2_json).unwrap();
        assert_eq!(deserialized_enum1, enum1);
        assert_eq!(deserialized_enum2, enum2);
    }

    #[test]
    fn test_case_deserialization_with_oneof2() {
        let oneof1: OneOf2<UnionCaseWithConst1, UnionCaseWithConst2> = OneOf2::Left(
            UnionCaseWithConst1::builder()
                .field_1("value1".to_string())
                .build_struct(),
        );
        let oneof2: OneOf2<UnionCaseWithConst1, UnionCaseWithConst2> = OneOf2::Right(
            UnionCaseWithConst2::builder()
                .field_2("value2".to_string())
                .build_struct(),
        );

        let json1 = serde_json::to_string(&oneof1).unwrap();
        let json2 = serde_json::to_string(&oneof2).unwrap();
        assert_eq!(json1, r#"{"field":"1","field1":"value1"}"#);
        assert_eq!(json2, r#"{"field":"2","field2":"value2"}"#);

        let deserialized1: OneOf2<UnionCaseWithConst1, UnionCaseWithConst2> =
            serde_json::from_str(&json1).unwrap();
        let deserialized2: OneOf2<UnionCaseWithConst1, UnionCaseWithConst2> =
            serde_json::from_str(&json2).unwrap();
        assert_eq!(deserialized1, oneof1);
        assert_eq!(deserialized2, oneof2);
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
        let case1 = UnionCase1::builder()
            .field_1("value1".to_string())
            .build_struct();
        let case2 = UnionCase2::builder()
            .field_2("value2".to_string())
            .build_struct();
        let enum_case1_output = Output::new(&case1);
        let enum_case2_output = Output::new(&case2);
        let _ = TypesystemServerArgs::builder().required_union(OneOf2::left(case1));
        let _ = TypesystemServerArgs::builder().required_union(OneOf2::right(case2));
        let _ = TypesystemServerArgs::builder().required_union(enum_case1_output.map(OneOf2::left));
        let _ =
            TypesystemServerArgs::builder().required_union(enum_case2_output.map(OneOf2::right));

        let case1 = UnionCase1::builder()
            .field_1("value1".to_string())
            .build_struct();
        let case2 = UnionCase2::builder()
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
