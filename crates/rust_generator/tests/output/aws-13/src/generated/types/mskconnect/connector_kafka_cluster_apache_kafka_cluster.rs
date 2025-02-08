#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectorKafkaClusterApacheKafkaCluster {
    /// The bootstrap servers of the cluster.
    #[builder(into)]
    #[serde(rename = "bootstrapServers")]
    pub r#bootstrap_servers: Box<String>,
    /// Details of an Amazon VPC which has network connectivity to the Apache Kafka cluster. See `vpc` Block for details.
    #[builder(into)]
    #[serde(rename = "vpc")]
    pub r#vpc: Box<super::super::types::mskconnect::ConnectorKafkaClusterApacheKafkaClusterVpc>,
}
