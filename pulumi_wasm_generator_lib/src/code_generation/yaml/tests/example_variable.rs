use crate::code_generation::yaml::model::{Example, Expression, Resource};
use crate::code_generation::yaml::tests::reformat_code;
use crate::code_generation::YamlFile;
use crate::model::ElementId;
use std::collections::BTreeMap;

//language=YAML
pub const YAML: &str = r#"
    resources:
        myCert:
            type: cloudflare:AccessMutualTlsCertificate
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
                    type_: "cloudflare:AccessMutualTlsCertificate".to_string(),
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
                        .certificate("${caPem}")
                        .build_struct(),
                );
            }
    "#,
    )
}
