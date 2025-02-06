#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AgentAgentActionGroupFunctionSchemaMemberFunctions {
    /// Functions that each define an action in the action group. See `functions` Block for details.
    #[builder(into, default)]
    #[serde(rename = "functions")]
    pub r#functions: Box<Option<Vec<super::super::types::bedrock::AgentAgentActionGroupFunctionSchemaMemberFunctionsFunction>>>,
}
