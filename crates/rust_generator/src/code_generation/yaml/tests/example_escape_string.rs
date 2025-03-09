use crate::code_generation::YamlFile;
use crate::code_generation::yaml::model::Expression;
use crate::code_generation::yaml::model::{Example, Resource};
use crate::model::ElementId;
use crate::utils::reformat_code;
use std::collections::BTreeMap;

//language=YAML
pub(crate) const YAML: &str = r#"
    resources:
        example:
            type: yamltests:AccessOrganization
            properties:
                name: my "name"
"#;

pub(crate) fn get_yaml_file() -> YamlFile {
    use crate::code_generation::yaml::yaml_model::{YamlExpression, YamlFile, YamlResource};

    YamlFile {
        resources: {
            let mut resources = BTreeMap::new();
            resources.insert(
                "example".to_string(),
                YamlResource {
                    type_: "yamltests:AccessOrganization".to_string(),
                    name: None,
                    properties: {
                        let mut properties = BTreeMap::new();
                        properties.insert(
                            "name".to_string(),
                            YamlExpression::String("my \"name\"".to_string()),
                        );
                        properties
                    },
                },
            );
            resources
        },
        variables: BTreeMap::new(),
    }
}

pub fn get_model() -> Example {
    Example {
        resources: {
            let mut map = BTreeMap::new();
            map.insert(
                "example".to_string(),
                Resource {
                    type_: ElementId::new("yamltests:index/accessOrganization:AccessOrganization")
                        .unwrap(),
                    // type_: "yamltests:AccessMutualTlsCertificate".to_string(),
                    name: None,
                    properties: {
                        let mut props = BTreeMap::new();
                        props.insert(
                            "name".to_string(),
                            Expression::String("my \"name\"".to_string()),
                        );
                        props
                    },
                },
            );
            map
        },
        variables: BTreeMap::new(),
    }
}

// language=Rust
pub fn get_rust_code() -> String {
    reformat_code(
        r#"
    use pulumi_gestalt_rust::Output;
    use pulumi_gestalt_rust::{add_export, pulumi_main};
    #[pulumi_main]
    fn test_main() -> Result<(), Error> {
        let example = access_organization::create(
            "example",
            AccessOrganizationArgs::builder()
                .name("my \"name\"")
                .build_struct(),
        );
    }
    "#,
    )
    .unwrap()
}
