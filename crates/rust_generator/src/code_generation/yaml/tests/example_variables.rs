use crate::code_generation::yaml::model::Example;
use crate::code_generation::yaml::model::Variable::FnInvokeVariable;
use crate::code_generation::yaml::model::{Expression, FnInvoke};
use crate::code_generation::yaml::yaml_model::{YamlExpression, YamlFnInvoke, YamlVariable};
use crate::code_generation::YamlFile;
use crate::model::ElementId;
use crate::utils::reformat_code;
use std::collections::BTreeMap;

//language=YAML
pub const YAML: &str = r#"
    variables:
      example:
        fn::invoke:
          Function: yamltests:getGatewayCategories
          Arguments:
            accountId: f037e56e89293a057740de681ac9abbe
"#;

pub fn get_yaml_file() -> YamlFile {
    use crate::code_generation::yaml::yaml_model::YamlFile;

    YamlFile {
        resources: BTreeMap::new(),
        variables: {
            let mut variables = BTreeMap::new();
            variables.insert(
                "example".to_string(),
                YamlVariable {
                    fn_invoke: YamlFnInvoke {
                        function: "yamltests:getGatewayCategories".to_string(),
                        arguments: {
                            let mut arguments = BTreeMap::new();
                            arguments.insert(
                                "accountId".to_string(),
                                YamlExpression::String(
                                    "f037e56e89293a057740de681ac9abbe".to_string(),
                                ),
                            );
                            arguments
                        },
                    },
                },
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
                    function: ElementId::new(
                        "yamltests:index/getGatewayCategories:getGatewayCategories",
                    )
                    .unwrap(),
                    arguments: {
                        let mut arguments = BTreeMap::new();
                        arguments.insert(
                            "accountId".to_string(),
                            Expression::String("f037e56e89293a057740de681ac9abbe".to_string()),
                        );
                        arguments
                    },
                }),
            );
            map
        },
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
            let example = get_gateway_categories::invoke(
                GetGatewayCategoriesArgs::builder()
                    .account_id("f037e56e89293a057740de681ac9abbe")
                    .build_struct(),
            );
        }
    "#,
    )
    .unwrap()
}
