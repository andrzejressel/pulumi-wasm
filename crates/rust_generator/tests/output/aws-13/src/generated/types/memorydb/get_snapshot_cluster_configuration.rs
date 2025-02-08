#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetSnapshotClusterConfiguration {
    /// Description for the cluster.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// The engine that will run on cluster nodes.
    #[builder(into)]
    #[serde(rename = "engine")]
    pub r#engine: Box<String>,
    /// Version number of the engine used by the cluster.
    #[builder(into)]
    #[serde(rename = "engineVersion")]
    pub r#engine_version: Box<String>,
    /// The weekly time range during which maintenance on the cluster is performed.
    #[builder(into)]
    #[serde(rename = "maintenanceWindow")]
    pub r#maintenance_window: Box<String>,
    /// Name of the snapshot.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Compute and memory capacity of the nodes in the cluster.
    #[builder(into)]
    #[serde(rename = "nodeType")]
    pub r#node_type: Box<String>,
    /// Number of shards in the cluster.
    #[builder(into)]
    #[serde(rename = "numShards")]
    pub r#num_shards: Box<i32>,
    /// Name of the parameter group associated with the cluster.
    #[builder(into)]
    #[serde(rename = "parameterGroupName")]
    pub r#parameter_group_name: Box<String>,
    /// Port number on which the cluster accepts connections.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
    /// Number of days for which MemoryDB retains automatic snapshots before deleting them.
    #[builder(into)]
    #[serde(rename = "snapshotRetentionLimit")]
    pub r#snapshot_retention_limit: Box<i32>,
    /// The daily time range (in UTC) during which MemoryDB begins taking a daily snapshot of the shard.
    #[builder(into)]
    #[serde(rename = "snapshotWindow")]
    pub r#snapshot_window: Box<String>,
    /// Name of the subnet group used by the cluster.
    #[builder(into)]
    #[serde(rename = "subnetGroupName")]
    pub r#subnet_group_name: Box<String>,
    /// ARN of the SNS topic to which cluster notifications are sent.
    #[builder(into)]
    #[serde(rename = "topicArn")]
    pub r#topic_arn: Box<String>,
    /// The VPC in which the cluster exists.
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: Box<String>,
}
