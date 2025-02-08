#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
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
