#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AgentAgentAliasRoutingConfiguration {
    /// Version of the agent with which the alias is associated.
    #[builder(into)]
    #[serde(rename = "agentVersion")]
    pub r#agent_version: Box<String>,
    /// ARN of the Provisioned Throughput assigned to the agent alias.
    #[builder(into)]
    #[serde(rename = "provisionedThroughput")]
    pub r#provisioned_throughput: Box<String>,
}
