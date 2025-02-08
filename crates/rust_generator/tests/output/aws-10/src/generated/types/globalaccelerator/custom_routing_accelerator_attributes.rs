#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CustomRoutingAcceleratorAttributes {
    /// Indicates whether flow logs are enabled. Defaults to `false`. Valid values: `true`, `false`.
    #[builder(into, default)]
    #[serde(rename = "flowLogsEnabled")]
    pub r#flow_logs_enabled: Box<Option<bool>>,
    /// The name of the Amazon S3 bucket for the flow logs. Required if `flow_logs_enabled` is `true`.
    #[builder(into, default)]
    #[serde(rename = "flowLogsS3Bucket")]
    pub r#flow_logs_s_3_bucket: Box<Option<String>>,
    /// The prefix for the location in the Amazon S3 bucket for the flow logs. Required if `flow_logs_enabled` is `true`.
    #[builder(into, default)]
    #[serde(rename = "flowLogsS3Prefix")]
    pub r#flow_logs_s_3_prefix: Box<Option<String>>,
}
