#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FunctionEnvironment {
    /// Map of environment variables that are accessible from the function code during execution. If provided at least one key must be present.
    #[builder(into, default)]
    #[serde(rename = "variables")]
    pub r#variables: Box<Option<std::collections::HashMap<String, String>>>,
}
