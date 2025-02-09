#[cfg(test)]
mod tests {
    use pulumi_gestalt_providers_typesystem::deep::nested::module::some_resource::SomeResourceArgs;
    use pulumi_gestalt_providers_typesystem::types::{
        IntegerEnum, MyEnum, NumberEnum, UnionCase1, UnionCase2, UnionCaseWithConst1,
        UnionCaseWithConst2,
    };
    use pulumi_gestalt_providers_typesystem::typesystem_server::TypesystemServerArgs;
    use pulumi_gestalt_rust::{OneOf2, Output};

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
    fn test_string_enum_deserialization() {
        let enum1 = MyEnum::Value1;
        let enum2 = MyEnum::Value2;
        let enum3 = MyEnum::special_characters;

        let enum1_json = serde_json::to_string(&enum1).unwrap();
        let enum2_json = serde_json::to_string(&enum2).unwrap();
        let enum3_json = serde_json::to_string(&enum3).unwrap();
        assert_eq!(enum1_json, r#""VALUE1""#);
        assert_eq!(enum2_json, r#""Value2""#);
        assert_eq!(enum3_json, r#""Plants'R'Us""#);

        let deserialized_enum1: MyEnum = serde_json::from_str(&enum1_json).unwrap();
        let deserialized_enum2: MyEnum = serde_json::from_str(&enum2_json).unwrap();
        let deserialized_enum3: MyEnum = serde_json::from_str(&enum3_json).unwrap();
        assert_eq!(deserialized_enum1, enum1);
        assert_eq!(deserialized_enum2, enum2);
        assert_eq!(deserialized_enum3, enum3);
    }

    #[test]
    fn test_integer_enum_deserialization() {
        let enum1 = IntegerEnum::Value1;
        let enum2 = IntegerEnum::Value2;

        let enum1_json = serde_json::to_string(&enum1).unwrap();
        let enum2_json = serde_json::to_string(&enum2).unwrap();
        assert_eq!(enum1_json, "1");
        assert_eq!(enum2_json, "2");

        let deserialized_enum1: IntegerEnum = serde_json::from_str(&enum1_json).unwrap();
        let deserialized_enum2: IntegerEnum = serde_json::from_str(&enum2_json).unwrap();
        assert_eq!(deserialized_enum1, enum1);
        assert_eq!(deserialized_enum2, enum2);
    }

    #[test]
    fn test_number_enum_deserialization() {
        let enum1 = NumberEnum::Value1;
        let enum2 = NumberEnum::Value2;

        let enum1_json = serde_json::to_string(&enum1).unwrap();
        let enum2_json = serde_json::to_string(&enum2).unwrap();
        assert_eq!(enum1_json, "1.0");
        assert_eq!(enum2_json, "2.0");

        let deserialized_enum1: NumberEnum = serde_json::from_str(&enum1_json).unwrap();
        let deserialized_enum2: NumberEnum = serde_json::from_str(&enum2_json).unwrap();
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

    #[allow(dead_code)]
    fn compilation_test() {
        let context = get_context();
        // String
        let output = Output::new(context, &"Hello, World!".to_string());

        let _ = TypesystemServerArgs::builder().required_string_input("&str");
        let _ = TypesystemServerArgs::builder().required_string_input("String".to_string());
        let _ = TypesystemServerArgs::builder().required_string_input(output.clone());

        let _ = TypesystemServerArgs::builder().optional_string_input("&str");
        let _ = TypesystemServerArgs::builder().optional_string_input("String".to_string());
        let _ = TypesystemServerArgs::builder().optional_string_input(output.clone());

        // Vec<String>
        let _ = TypesystemServerArgs::builder().required_string_array(vec!["&str"]);
        let _ = TypesystemServerArgs::builder().required_string_array(vec!["String".to_string()]);
        let _ =
            TypesystemServerArgs::builder().required_string_array(output.clone().map(|s| vec![s]));
        // let _ = TypesystemServerArgs::builder().required_string_array(vec![string_output]);

        let _ = TypesystemServerArgs::builder().optional_string_array(vec!["&str"]);
        let _ = TypesystemServerArgs::builder().optional_string_array(vec!["String".to_string()]);
        let _ =
            TypesystemServerArgs::builder().optional_string_array(output.clone().map(|s| vec![s]));
        // let _ = TypesystemServerArgs::builder().optional_string_array(vec![string_output]);

        // Vec<String> with array
        let _ = TypesystemServerArgs::builder().required_string_array(["&str"]);
        let _ = TypesystemServerArgs::builder().required_string_array(["String".to_string()]);
        let _ =
            TypesystemServerArgs::builder().required_string_array(output.clone().map(|s| vec![s]));
        // let _ = TypesystemServerArgs::builder().required_string_array([string_output]);

        let _ = TypesystemServerArgs::builder().optional_string_array(["&str"]);
        let _ = TypesystemServerArgs::builder().optional_string_array(["String".to_string()]);
        let _ =
            TypesystemServerArgs::builder().optional_string_array(output.clone().map(|s| vec![s]));
        // let _ = TypesystemServerArgs::builder().optional_string_array([string_output]);

        // Union
        let case1 = UnionCase1::builder()
            .field_1("value1".to_string())
            .build_struct();
        let case2 = UnionCase2::builder()
            .field_2("value2".to_string())
            .build_struct();
        let enum_case1_output = Output::new(context, &case1);
        let enum_case2_output = Output::new(context, &case2);
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

    #[allow(dead_code)]
    fn resource_compilation_test() {
        let context = get_context();

        pulumi_gestalt_providers_typesystem::deep::nested::module::some_resource::create(
            context,
            "test",
            SomeResourceArgs::builder().build_struct(),
        );
    }

    #[allow(dead_code)]
    fn function_compilation_test() {
        let context = get_context();
        pulumi_gestalt_providers_typesystem::functions::deep::nested::module::some_function::invoke(
            context,
        );
    }

    #[allow(dead_code)]
    fn types_compilation_test() {
        let _ =
            pulumi_gestalt_providers_typesystem::types::deep::nested::module::SomeType::builder()
                .build_struct();
    }

    fn get_context() -> &'static pulumi_gestalt_rust::PulumiContext {
        todo!()
    }
}
