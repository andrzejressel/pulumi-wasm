use super::*;
use crate::code_generation::YamlFile;

//language=YAML
pub const YAML: &str = r#"
    resources:
        petstoreSchema:
            type: cloudflare:ApiShieldSchema
            name: petstore_schema
            properties:
                zoneId: 0da42c8d2132a9ddaf714f9e7c920711
                name: myschema
                kind: openapi_v3
                validationEnabled: true
                source:
                    fn::invoke:
                      Function: std:file
                      Arguments:
                        input: ./schemas/petstore.json
                      Return: result
"#;

pub fn get_yaml_file() -> YamlFile {
    use crate::code_generation::yaml::yaml_model::{YamlExpression, YamlFile, YamlResource};
    use std::collections::BTreeMap;

    YamlFile {
        resources: {
            let mut resources = BTreeMap::new();
            resources.insert(
                "petstoreSchema".to_string(),
                YamlResource {
                    type_: "cloudflare:ApiShieldSchema".to_string(),
                    name: Some("petstore_schema".to_string()),
                    properties: {
                        let mut properties = BTreeMap::new();
                        properties.insert(
                            "zoneId".to_string(),
                            YamlExpression::String("0da42c8d2132a9ddaf714f9e7c920711".to_string()),
                        );
                        properties.insert(
                            "name".to_string(),
                            YamlExpression::String("myschema".to_string()),
                        );
                        properties.insert(
                            "kind".to_string(),
                            YamlExpression::String("openapi_v3".to_string()),
                        );
                        properties.insert(
                            "validationEnabled".to_string(),
                            YamlExpression::Boolean(true),
                        );
                        properties.insert(
                            "source".to_string(),
                            YamlExpression::Object({
                                let mut source = BTreeMap::new();
                                source.insert(
                                    "fn::invoke".to_string(),
                                    YamlExpression::Object({
                                        let mut fn_invoke = BTreeMap::new();
                                        fn_invoke.insert(
                                            "Function".to_string(),
                                            YamlExpression::String("std:file".to_string()),
                                        );
                                        fn_invoke.insert(
                                            "Arguments".to_string(),
                                            YamlExpression::Object({
                                                let mut arguments = BTreeMap::new();
                                                arguments.insert(
                                                    "input".to_string(),
                                                    YamlExpression::String(
                                                        "./schemas/petstore.json".to_string(),
                                                    ),
                                                );
                                                arguments
                                            }),
                                        );
                                        fn_invoke.insert(
                                            "Return".to_string(),
                                            YamlExpression::String("result".to_string()),
                                        );
                                        fn_invoke
                                    }),
                                );
                                source
                            }),
                        );
                        properties
                    },
                },
            );
            resources
        },
    }
}
