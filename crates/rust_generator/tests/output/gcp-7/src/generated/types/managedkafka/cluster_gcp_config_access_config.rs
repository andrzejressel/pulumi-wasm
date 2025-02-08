#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterGcpConfigAccessConfig {
    /// Virtual Private Cloud (VPC) subnets where IP addresses for the Kafka cluster are allocated. To make the cluster available in a VPC, you must specify at least one subnet per network. You must specify between 1 and 10 subnets. Additional subnets may be specified with additional `network_configs` blocks.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "networkConfigs")]
    pub r#network_configs: Box<Vec<super::super::types::managedkafka::ClusterGcpConfigAccessConfigNetworkConfig>>,
}
