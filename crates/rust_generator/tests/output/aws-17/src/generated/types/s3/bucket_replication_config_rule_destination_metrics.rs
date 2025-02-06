#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketReplicationConfigRuleDestinationMetrics {
    /// Configuration block that specifies the time threshold for emitting the `s3:Replication:OperationMissedThreshold` event. See below.
    #[builder(into, default)]
    #[serde(rename = "eventThreshold")]
    pub r#event_threshold: Box<Option<super::super::types::s3::BucketReplicationConfigRuleDestinationMetricsEventThreshold>>,
    /// Status of the Destination Metrics. Either `"Enabled"` or `"Disabled"`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
}
