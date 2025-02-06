#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectorKafkaCluster {
    /// The Apache Kafka cluster to which the connector is connected. See `apache_kafka_cluster` Block for details.
    #[builder(into)]
    #[serde(rename = "apacheKafkaCluster")]
    pub r#apache_kafka_cluster: Box<super::super::types::mskconnect::ConnectorKafkaClusterApacheKafkaCluster>,
}
