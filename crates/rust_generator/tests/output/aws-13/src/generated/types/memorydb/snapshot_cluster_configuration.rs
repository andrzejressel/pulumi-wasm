#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SnapshotClusterConfiguration {
    /// Description for the cluster.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The engine that will run on cluster nodes.
    #[builder(into, default)]
    #[serde(rename = "engine")]
    pub r#engine: Box<Option<String>>,
    /// Version number of the engine used by the cluster.
    #[builder(into, default)]
    #[serde(rename = "engineVersion")]
    pub r#engine_version: Box<Option<String>>,
    /// The weekly time range during which maintenance on the cluster is performed.
    #[builder(into, default)]
    #[serde(rename = "maintenanceWindow")]
    pub r#maintenance_window: Box<Option<String>>,
    /// Name of the snapshot. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Compute and memory capacity of the nodes in the cluster.
    #[builder(into, default)]
    #[serde(rename = "nodeType")]
    pub r#node_type: Box<Option<String>>,
    /// Number of shards in the cluster.
    #[builder(into, default)]
    #[serde(rename = "numShards")]
    pub r#num_shards: Box<Option<i32>>,
    /// Name of the parameter group associated with the cluster.
    #[builder(into, default)]
    #[serde(rename = "parameterGroupName")]
    pub r#parameter_group_name: Box<Option<String>>,
    /// Port number on which the cluster accepts connections.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    /// Number of days for which MemoryDB retains automatic snapshots before deleting them.
    #[builder(into, default)]
    #[serde(rename = "snapshotRetentionLimit")]
    pub r#snapshot_retention_limit: Box<Option<i32>>,
    /// The daily time range (in UTC) during which MemoryDB begins taking a daily snapshot of the shard.
    #[builder(into, default)]
    #[serde(rename = "snapshotWindow")]
    pub r#snapshot_window: Box<Option<String>>,
    /// Name of the subnet group used by the cluster.
    #[builder(into, default)]
    #[serde(rename = "subnetGroupName")]
    pub r#subnet_group_name: Box<Option<String>>,
    /// ARN of the SNS topic to which cluster notifications are sent.
    #[builder(into, default)]
    #[serde(rename = "topicArn")]
    pub r#topic_arn: Box<Option<String>>,
    /// The VPC in which the cluster exists.
    #[builder(into, default)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: Box<Option<String>>,
}
