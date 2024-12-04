use crate::code_generation::yaml::model::{Expression, FnInvoke, Variable};
use crate::code_generation::yaml::model::{Example, Resource};
use crate::code_generation::yaml::tests::reformat_code;
use crate::code_generation::yaml::yaml_model::{YamlFnInvoke, YamlExpression, YamlVariable};
use crate::code_generation::YamlFile;
use crate::model::ElementId;
use std::collections::BTreeMap;
use crate::code_generation::yaml::model::Variable::{FnInvokeVariable as OtherFnInvoke, FnInvokeVariable};

//language=YAML
pub const YAML: &str = r#"
    variables:
      example:
        fn::invoke:
          Function: cloudflare:getGatewayCategories
          Arguments:
            accountId: f037e56e89293a057740de681ac9abbe
"#;

pub fn get_yaml_file() -> YamlFile {
    use crate::code_generation::yaml::yaml_model::{YamlFile, YamlResource};

    YamlFile {
        resources: BTreeMap::new(),
        variables: {
            let mut variables = BTreeMap::new();
            variables.insert(
                "example".to_string(),
                YamlVariable {
                    fn_invoke: YamlFnInvoke {
                        function: "cloudflare:getGatewayCategories".to_string(),
                        arguments: {
                            let mut arguments = BTreeMap::new();
                            arguments.insert("accountId".to_string(), YamlExpression::String("f037e56e89293a057740de681ac9abbe".to_string()));
                            arguments
                        }
                    }
                }
            );
            variables
        },
    }
}

pub fn get_model() -> Example {
    Example {
        resources: BTreeMap::new(),
        variables: {
            let mut map = BTreeMap::new();
            map.insert(
                "example".to_string(),
                FnInvokeVariable(FnInvoke {
                        function: ElementId::new("cloudflare:index/getGatewayCategories:getGatewayCategories").unwrap(),
                        arguments: {
                            let mut arguments = BTreeMap::new();
                            arguments.insert("accountId".to_string(), Expression::String("f037e56e89293a057740de681ac9abbe".to_string()));
                            arguments
                        }
                    }
                )
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
            let example = keyless_certificate::create(
                "example",
                KeylessCertificateArgs::builder()
                    .port(24008)
                    .build_struct(),
            );
        }
    "#,
    )
}
