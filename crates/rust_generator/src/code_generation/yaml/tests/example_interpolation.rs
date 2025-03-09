use crate::code_generation::YamlFile;
use crate::code_generation::yaml::model::{Example, Expression, Resource};
use crate::model::ElementId;
use crate::utils::reformat_code;
use std::collections::BTreeMap;

//language=YAML
pub const YAML: &str = r#"
    resources:
        myCert:
            type: yamltests:AccessMutualTlsCertificate
            name: my_cert
            properties:
                certificate: ${caPem}
"#;

pub fn get_yaml_file() -> YamlFile {
    use crate::code_generation::yaml::yaml_model::{YamlExpression, YamlFile, YamlResource};

    YamlFile {
        resources: {
            let mut resources = BTreeMap::new();
            resources.insert(
                "myCert".to_string(),
                YamlResource {
                    type_: "yamltests:AccessMutualTlsCertificate".to_string(),
                    name: Some("my_cert".to_string()),
                    properties: {
                        let mut properties = BTreeMap::new();
                        properties.insert(
                            "certificate".to_string(),
                            YamlExpression::String("${caPem}".to_string()),
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
                        "yamltests:index/accessMutualTlsCertificate:AccessMutualTlsCertificate",
                    )
                    .unwrap(),
                    name: Some("my_cert".to_string()),
                    properties: {
                        let mut props = BTreeMap::new();
                        props.insert(
                            "certificate".to_string(),
                            Expression::String("${caPem}".to_string()),
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
                let myCert = access_mutual_tls_certificate::create(
                    "myCert",
                    AccessMutualTlsCertificateArgs::builder()
                        .certificate("${caPem}")
                        .build_struct(),
                );
            }
    "#,
    )
    .unwrap()
}
