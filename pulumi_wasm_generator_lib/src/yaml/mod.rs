pub(crate) mod model;
pub(crate) mod yaml_model;

#[cfg(test)]
pub(crate) mod tests {
    use crate::yaml::yaml_model::YamlFile;
    use std::collections::BTreeMap;
    pub(crate) mod example_access_organization {

        use super::*;

        pub(crate) const YAML: &str = r#"
resources:
  example:
    type: cloudflare:AccessOrganization
    properties:
      accountId: f037e56e89293a057740de681ac9abbe
      name: example.cloudflareaccess.com
      authDomain: example.cloudflareaccess.com
      isUiReadOnly: false
      userSeatExpirationInactiveTime: 720h
      autoRedirectToIdentity: false
      loginDesigns:
        - backgroundColor: '#ffffff'
          textColor: '#000000'
          logoPath: https://example.com/logo.png
          headerText: My header text
          footerText: My footer text
    "#;

        pub(crate) fn get_yaml_file() -> YamlFile {
            use crate::yaml::yaml_model::{YamlExpression, YamlFile, YamlResource};

            YamlFile {
                resources: {
                    let mut resources = BTreeMap::new();
                    resources.insert(
                        "example".to_string(),
                        YamlResource {
                            type_: "cloudflare:AccessOrganization".to_string(),
                            name: None,
                            properties: {
                                let mut properties = BTreeMap::new();
                                properties.insert(
                                    "accountId".to_string(),
                                    YamlExpression::String(
                                        "f037e56e89293a057740de681ac9abbe".to_string(),
                                    ),
                                );
                                properties.insert(
                                    "name".to_string(),
                                    YamlExpression::String(
                                        "example.cloudflareaccess.com".to_string(),
                                    ),
                                );
                                properties.insert(
                                    "authDomain".to_string(),
                                    YamlExpression::String(
                                        "example.cloudflareaccess.com".to_string(),
                                    ),
                                );
                                properties.insert(
                                    "isUiReadOnly".to_string(),
                                    YamlExpression::Boolean(false),
                                );
                                properties.insert(
                                    "userSeatExpirationInactiveTime".to_string(),
                                    YamlExpression::String("720h".to_string()),
                                );
                                properties.insert(
                                    "autoRedirectToIdentity".to_string(),
                                    YamlExpression::Boolean(false),
                                );
                                properties.insert(
                                    "loginDesigns".to_string(),
                                    YamlExpression::Array(vec![YamlExpression::Object({
                                        let mut design = BTreeMap::new();
                                        design.insert(
                                            "backgroundColor".to_string(),
                                            YamlExpression::String("#ffffff".to_string()),
                                        );
                                        design.insert(
                                            "textColor".to_string(),
                                            YamlExpression::String("#000000".to_string()),
                                        );
                                        design.insert(
                                            "logoPath".to_string(),
                                            YamlExpression::String(
                                                "https://example.com/logo.png".to_string(),
                                            ),
                                        );
                                        design.insert(
                                            "headerText".to_string(),
                                            YamlExpression::String("My header text".to_string()),
                                        );
                                        design.insert(
                                            "footerText".to_string(),
                                            YamlExpression::String("My footer text".to_string()),
                                        );
                                        design
                                    })]),
                                );
                                properties
                            },
                        },
                    );
                    resources
                },
            }
        }
    }

    pub(crate) mod complex_yaml {

        use super::*;

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
            use crate::yaml::yaml_model::{YamlExpression, YamlFile, YamlResource};
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
                                    YamlExpression::String(
                                        "0da42c8d2132a9ddaf714f9e7c920711".to_string(),
                                    ),
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
                                                                "./schemas/petstore.json"
                                                                    .to_string(),
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
    }

    pub(crate) mod access_rule {
        use crate::model::ElementId;
        use crate::yaml::model::{Example, Expression};
        use crate::yaml::tests::reformat_code;
        use std::collections::BTreeMap;

        pub const YAML: &str = r#"
resources:
  # Challenge requests coming from known Tor exit nodes.
  torExitNodes:
    type: cloudflare:AccessRule
    name: tor_exit_nodes
    properties:
      zoneId: 0da42c8d2132a9ddaf714f9e7c920711
      notes: Requests coming from known Tor exit nodes
      mode: challenge
      configuration:
        target: country
        value: T1
"#;

        pub fn get_yaml_file() -> super::YamlFile {
            use crate::yaml::yaml_model::{YamlExpression, YamlFile, YamlResource};

            YamlFile {
                resources: {
                    let mut resources = BTreeMap::new();
                    resources.insert(
                        "torExitNodes".to_string(),
                        YamlResource {
                            type_: "cloudflare:AccessRule".to_string(),
                            name: Some("tor_exit_nodes".to_string()),
                            properties: {
                                let mut properties = BTreeMap::new();
                                properties.insert(
                                    "zoneId".to_string(),
                                    YamlExpression::String(
                                        "0da42c8d2132a9ddaf714f9e7c920711".to_string(),
                                    ),
                                );
                                properties.insert(
                                    "notes".to_string(),
                                    YamlExpression::String(
                                        "Requests coming from known Tor exit nodes".to_string(),
                                    ),
                                );
                                properties.insert(
                                    "mode".to_string(),
                                    YamlExpression::String("challenge".to_string()),
                                );
                                properties.insert(
                                    "configuration".to_string(),
                                    YamlExpression::Object({
                                        let mut configuration = BTreeMap::new();
                                        configuration.insert(
                                            "target".to_string(),
                                            YamlExpression::String("country".to_string()),
                                        );
                                        configuration.insert(
                                            "value".to_string(),
                                            YamlExpression::String("T1".to_string()),
                                        );
                                        configuration
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

        pub fn get_model() -> Example {
            Example {
                resources: {
                    let mut map = BTreeMap::new();
                    map.insert(
                        "torExitNodes".to_string(),
                        crate::yaml::model::Resource {
                            type_: ElementId::new("cloudflare:index/accessRule:AccessRule")
                                .unwrap(),
                            name: Some("tor_exit_nodes".to_string()),
                            properties: {
                                let mut props = BTreeMap::new();
                                props.insert(
                                    "zoneId".to_string(),
                                    Expression::String(
                                        "0da42c8d2132a9ddaf714f9e7c920711".to_string(),
                                    ),
                                );
                                props.insert(
                                    "notes".to_string(),
                                    Expression::String(
                                        "Requests coming from known Tor exit nodes".to_string(),
                                    ),
                                );
                                props.insert(
                                    "mode".to_string(),
                                    Expression::String("challenge".to_string()),
                                );
                                props.insert(
                                    "configuration"
                                        .to_string(),
                                    Expression::Object(
                                        ElementId::new("cloudflare:index/AccessRuleConfiguration:AccessRuleConfiguration").unwrap(), // You might need to adjust this ElementId
                                        {
                                            let mut config = BTreeMap::new();
                                            config.insert(
                                                "target".to_string(),
                                                Expression::String("country".to_string()),
                                            );
                                            config.insert(
                                                "value".to_string(),
                                                Expression::String("T1".to_string()),
                                            );
                                            config
                                        },
                                    ),
                                );
                                props
                            },
                        },
                    );
                    map
                },
            }
        }

        pub fn get_rust_code() -> String {
            reformat_code(
                r#"
                use pulumi_wasm_rust::Output;
                use pulumi_wasm_rust::{add_export, pulumi_main};
                #[pulumi_main]
                fn test_main() -> Result<(), Error> {
                    let torExitNodes = access_rule::create(
                        "torExitNodes",
                        AccessRuleArgs::builder()
                            .configuration(
                                AccessRuleConfiguration::builder()
                                    .target("country")
                                    .value("T1")
                                    .build_struct(),
                            )
                            .mode("challenge")
                            .notes("Requests coming from known Tor exit nodes")
                            .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
                            .build_struct(),
                    );
                }
            "#,
            )
        }
    }

    pub(crate) mod example_array {
        use super::*;
        use crate::model::ElementId;
        use crate::yaml::model::{Example, Expression, Resource};

        pub const YAML: &str = r#"
resources:
  myCert:
    type: cloudflare:AccessMutualTlsCertificate
    name: my_cert
    properties:
      zoneId: 0da42c8d2132a9ddaf714f9e7c920711
      name: My Root Cert
      certificate: ${caPem}
      associatedHostnames:
        - staging.example.com
"#;

        pub fn get_yaml_file() -> super::YamlFile {
            use crate::yaml::yaml_model::{YamlExpression, YamlFile, YamlResource};

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
                                    "zoneId".to_string(),
                                    YamlExpression::String(
                                        "0da42c8d2132a9ddaf714f9e7c920711".to_string(),
                                    ),
                                );
                                properties.insert(
                                    "name".to_string(),
                                    YamlExpression::String("My Root Cert".to_string()),
                                );
                                properties.insert(
                                    "certificate".to_string(),
                                    YamlExpression::String("${caPem}".to_string()),
                                );
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
            }
        }

        pub fn get_model() -> Example {
            Example {
                resources: {
                    let mut map = BTreeMap::new();
                    map.insert(
                        "myCert".to_string(),
                        Resource {
                            type_: ElementId::new("cloudflare:index/accessMutualTlsCertificate:AccessMutualTlsCertificate").unwrap(),
                            // type_: "cloudflare:AccessMutualTlsCertificate".to_string(),
                            name: Some("my_cert".to_string()),
                            properties: {
                                let mut props = BTreeMap::new();
                                props.insert(
                                    "zoneId".to_string(),
                                    Expression::String(
                                        "0da42c8d2132a9ddaf714f9e7c920711".to_string(),
                                    ),
                                );
                                props.insert(
                                    "name".to_string(),
                                    Expression::String("My Root Cert".to_string()),
                                );
                                props.insert(
                                    "certificate".to_string(),
                                    Expression::String("${caPem}".to_string()),
                                );
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
            }
        }

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
                        .certificate("${caPem}")
                        .name("My Root Cert")
                        .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
                        .build_struct(),
                );
            }
            "#,
            )
        }
    }

    pub(crate) mod example_variable {
        use super::*;

        pub const YAML: &str = r#"
resources:
  myCert:
    type: cloudflare:AccessMutualTlsCertificate
    name: my_cert
    properties:
      zoneId: 0da42c8d2132a9ddaf714f9e7c920711
      name: My Root Cert
      certificate: ${caPem}
      associatedHostnames:
        - staging.example.com
        "#;

        pub fn get_yaml_file() -> super::YamlFile {
            use crate::yaml::yaml_model::{YamlExpression, YamlFile, YamlResource};

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
                                    "zoneId".to_string(),
                                    YamlExpression::String(
                                        "0da42c8d2132a9ddaf714f9e7c920711".to_string(),
                                    ),
                                );
                                properties.insert(
                                    "name".to_string(),
                                    YamlExpression::String("My Root Cert".to_string()),
                                );
                                properties.insert(
                                    "certificate".to_string(),
                                    YamlExpression::String("${caPem}".to_string()),
                                );
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
            }
        }
    }

    fn reformat_code(code: &str) -> String {
        let syntax_tree = syn::parse_file(code).unwrap();
        prettyplease::unparse(&syntax_tree)
    }
}
