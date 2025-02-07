#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TableReplica {
    /// ARN of the table
    #[builder(into, default)]
    #[serde(rename = "arn")]
    pub r#arn: Box<Option<String>>,
    /// ARN of the CMK that should be used for the AWS KMS encryption. This argument should only be used if the key is different from the default KMS-managed DynamoDB key, `alias/aws/dynamodb`. **Note:** This attribute will _not_ be populated with the ARN of _default_ keys.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyArn")]
    pub r#kms_key_arn: Box<Option<String>>,
    /// Whether to enable Point In Time Recovery for the replica. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "pointInTimeRecovery")]
    pub r#point_in_time_recovery: Box<Option<bool>>,
    /// Whether to propagate the global table's tags to a replica. Default is `false`. Changes to tags only move in one direction: from global (source) to replica. In other words, tag drift on a replica will not trigger an update. Tag or replica changes on the global table, whether from drift or configuration changes, are propagated to replicas. Changing from `true` to `false` on a subsequent `apply` means replica tags are left as they were, unmanaged, not deleted.
    #[builder(into, default)]
    #[serde(rename = "propagateTags")]
    pub r#propagate_tags: Box<Option<bool>>,
    /// Region name of the replica.
    #[builder(into)]
    #[serde(rename = "regionName")]
    pub r#region_name: Box<String>,
    /// ARN of the Table Stream. Only available when `stream_enabled = true`
    #[builder(into, default)]
    #[serde(rename = "streamArn")]
    pub r#stream_arn: Box<Option<String>>,
    /// Timestamp, in ISO 8601 format, for this stream. Note that this timestamp is not a unique identifier for the stream on its own. However, the combination of AWS customer ID, table name and this field is guaranteed to be unique. It can be used for creating CloudWatch Alarms. Only available when `stream_enabled = true`.
    #[builder(into, default)]
    #[serde(rename = "streamLabel")]
    pub r#stream_label: Box<Option<String>>,
}
