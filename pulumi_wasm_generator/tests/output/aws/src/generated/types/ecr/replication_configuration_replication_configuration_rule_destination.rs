#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ReplicationConfigurationReplicationConfigurationRuleDestination {
    /// A Region to replicate to.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Box<String>,
    /// The account ID of the destination registry to replicate to.
    #[builder(into)]
    #[serde(rename = "registryId")]
    pub r#registry_id: Box<String>,
}
