#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AgentAgentActionGroupFunctionSchema {
    /// Contains a list of functions.
    /// Each function describes and action in the action group.
    /// See `member_functions` Block for details.
    #[builder(into, default)]
    #[serde(rename = "memberFunctions")]
    pub r#member_functions: Box<Option<super::super::types::bedrock::AgentAgentActionGroupFunctionSchemaMemberFunctions>>,
}
