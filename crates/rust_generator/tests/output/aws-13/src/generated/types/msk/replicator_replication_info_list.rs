#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ReplicatorReplicationInfoList {
    /// Configuration relating to consumer group replication.
    #[builder(into)]
    #[serde(rename = "consumerGroupReplications")]
    pub r#consumer_group_replications: Box<Vec<super::super::types::msk::ReplicatorReplicationInfoListConsumerGroupReplication>>,
    #[builder(into, default)]
    #[serde(rename = "sourceKafkaClusterAlias")]
    pub r#source_kafka_cluster_alias: Box<Option<String>>,
    /// The ARN of the source Kafka cluster.
    #[builder(into)]
    #[serde(rename = "sourceKafkaClusterArn")]
    pub r#source_kafka_cluster_arn: Box<String>,
    /// The type of compression to use writing records to target Kafka cluster.
    #[builder(into)]
    #[serde(rename = "targetCompressionType")]
    pub r#target_compression_type: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "targetKafkaClusterAlias")]
    pub r#target_kafka_cluster_alias: Box<Option<String>>,
    /// The ARN of the target Kafka cluster.
    #[builder(into)]
    #[serde(rename = "targetKafkaClusterArn")]
    pub r#target_kafka_cluster_arn: Box<String>,
    /// Configuration relating to topic replication.
    #[builder(into)]
    #[serde(rename = "topicReplications")]
    pub r#topic_replications: Box<Vec<super::super::types::msk::ReplicatorReplicationInfoListTopicReplication>>,
}
