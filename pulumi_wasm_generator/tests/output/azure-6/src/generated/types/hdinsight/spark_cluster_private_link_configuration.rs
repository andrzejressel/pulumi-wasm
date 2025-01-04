#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SparkClusterPrivateLinkConfiguration {
    /// The ID of the private link service group.
    #[builder(into)]
    #[serde(rename = "groupId")]
    pub r#group_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "ipConfiguration")]
    pub r#ip_configuration: Box<super::super::types::hdinsight::SparkClusterPrivateLinkConfigurationIpConfiguration>,
    /// The name of the private link configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
