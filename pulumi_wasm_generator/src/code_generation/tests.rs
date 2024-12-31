#[cfg(test)]
mod tests {
    use crate::code_generation::yaml::model::yaml_to_model;
    use crate::code_generation::yaml::tests::*;
    use crate::code_generation::YamlFile;
    use crate::{extract_schema_from_file, schema};

    use crate::code_generation::rust_generation::generate_code;
    use crate::code_generation::yaml::tests::{
        example_array, example_empty_properties, example_escape_string, example_numbers,
    };

    macro_rules! yaml_deserialization_test {
        ($test_name:ident, $test_module:ident) => {
            #[test]
            fn $test_name() {
                let yaml_file = YamlFile::from_yaml($test_module::YAML).unwrap();
                let expected_yaml_file = $test_module::get_yaml_file();
                assert_eq!(yaml_file, expected_yaml_file);
            }
        };
    }

    yaml_deserialization_test!(generate_yaml_complex_yaml, complex_yaml);

    macro_rules! full_pipeline_test {
        ($test_name:ident, $package_name:expr, $test_module:ident) => {
            #[test]
            fn $test_name() -> anyhow::Result<()> {
                let yaml_file = YamlFile::from_yaml($test_module::YAML).unwrap();
                let expected_yaml_file = $test_module::get_yaml_file();
                assert_eq!(yaml_file, expected_yaml_file);

                let schema_package: schema::Package = extract_schema_from_file(
                    concat!("test_cases/", $package_name, ".json").as_ref(),
                )
                .unwrap();

                let package = schema::to_model(&schema_package).unwrap();
                let yaml_file = $test_module::get_yaml_file();
                let result = yaml_to_model(yaml_file, $package_name.to_string(), &package)?;
                assert_eq!(result, $test_module::get_model());

                let model = $test_module::get_model();
                let code = generate_code(model)?;
                assert_eq!(code, $test_module::get_rust_code());
                Ok(())
            }
        };
    }

    full_pipeline_test!(full_pipeline_example_array, "yamltests", example_array);
    full_pipeline_test!(
        full_pipeline_example_empty_properties,
        "yamltests",
        example_empty_properties
    );
    full_pipeline_test!(
        full_pipeline_example_escape_string,
        "yamltests",
        example_escape_string
    );
    full_pipeline_test!(full_pipeline_example_numbers, "yamltests", example_numbers);
    full_pipeline_test!(
        full_pipeline_example_interpolation,
        "yamltests",
        example_interpolation
    );
    full_pipeline_test!(generate_yaml_variables, "yamltests", example_variables);
}
