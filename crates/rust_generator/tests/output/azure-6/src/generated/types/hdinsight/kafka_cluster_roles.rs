#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KafkaClusterRoles {
    /// A `head_node` block as defined above.
    #[builder(into)]
    #[serde(rename = "headNode")]
    pub r#head_node: Box<super::super::types::hdinsight::KafkaClusterRolesHeadNode>,
    /// A `kafka_management_node` block as defined below.
    /// 
    /// > **Note:** This property has been deprecated and will be removed in version 4.0.
    #[builder(into, default)]
    #[serde(rename = "kafkaManagementNode")]
    pub r#kafka_management_node: Box<Option<super::super::types::hdinsight::KafkaClusterRolesKafkaManagementNode>>,
    /// A `worker_node` block as defined below.
    #[builder(into)]
    #[serde(rename = "workerNode")]
    pub r#worker_node: Box<super::super::types::hdinsight::KafkaClusterRolesWorkerNode>,
    /// A `zookeeper_node` block as defined below.
    #[builder(into)]
    #[serde(rename = "zookeeperNode")]
    pub r#zookeeper_node: Box<super::super::types::hdinsight::KafkaClusterRolesZookeeperNode>,
}
