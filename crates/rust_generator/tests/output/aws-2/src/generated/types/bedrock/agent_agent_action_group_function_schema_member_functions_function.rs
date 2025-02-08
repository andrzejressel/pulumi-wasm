#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AgentAgentActionGroupFunctionSchemaMemberFunctionsFunction {
    /// Description of the function and its purpose.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Name for the function.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Parameters that the agent elicits from the user to fulfill the function. See `parameters` Block for details.
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<Vec<super::super::types::bedrock::AgentAgentActionGroupFunctionSchemaMemberFunctionsFunctionParameter>>>,
}
