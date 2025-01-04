#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetJobDefinitionNodePropertyNodeRangePropertyContainerNetworkConfiguration {
    /// Indicates whether the job has a public IP address.
    #[builder(into)]
    #[serde(rename = "assignPublicIp")]
    pub r#assign_public_ip: Box<bool>,
}
