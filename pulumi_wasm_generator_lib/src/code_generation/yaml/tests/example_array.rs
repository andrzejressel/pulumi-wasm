use crate::code_generation::yaml::model::{Example, Expression, Resource};
use crate::code_generation::YamlFile;
use crate::model::ElementId;
use crate::utils::reformat_code;
use std::collections::BTreeMap;

//language=YAML
pub const YAML: &str = r#"
    resources:
        myCert:
            type: cloudflare:AccessMutualTlsCertificate
            name: my_cert
            properties:
                associatedHostnames:
                - staging.example.com
"#;

pub fn get_yaml_file() -> YamlFile {
    use crate::code_generation::yaml::yaml_model::{YamlExpression, YamlFile, YamlResource};

    YamlFile {
        resources: {
            let mut resources = BTreeMap::new();
            resources.insert(
                "myCert".to_string(),
                YamlResource {
                    type_: "cloudflare:AccessMutualTlsCertificate".to_string(),
                    name: Some("my_cert".to_string()),
                    properties: {
                        let mut properties = BTreeMap::new();
                        properties.insert(
                            "associatedHostnames".to_string(),
                            YamlExpression::Array(vec![YamlExpression::String(
                                "staging.example.com".to_string(),
                            )]),
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
                "myCert".to_string(),
                Resource {
                    type_: ElementId::new(
                        "cloudflare:index/accessMutualTlsCertificate:AccessMutualTlsCertificate",
                    )
                    .unwrap(),
                    // type_: "cloudflare:AccessMutualTlsCertificate".to_string(),
                    name: Some("my_cert".to_string()),
                    properties: {
                        let mut props = BTreeMap::new();
                        props.insert(
                            "associatedHostnames".to_string(),
                            Expression::Array(vec![Expression::String(
                                "staging.example.com".to_string(),
                            )]),
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
    use pulumi_wasm_rust::Output;
    use pulumi_wasm_rust::{add_export, pulumi_main};
    #[pulumi_main]
    fn test_main() -> Result<(), Error> {
        let myCert = access_mutual_tls_certificate::create(
            "myCert",
            AccessMutualTlsCertificateArgs::builder()
                .associated_hostnames(vec!["staging.example.com",])
                .build_struct(),
        );
    }
    "#,
    )
    .unwrap()
}
