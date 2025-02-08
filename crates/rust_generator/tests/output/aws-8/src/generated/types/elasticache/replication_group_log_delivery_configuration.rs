#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ReplicationGroupLogDeliveryConfiguration {
    /// Name of either the CloudWatch Logs LogGroup or Kinesis Data Firehose resource.
    #[builder(into)]
    #[serde(rename = "destination")]
    pub r#destination: Box<String>,
    /// For CloudWatch Logs use `cloudwatch-logs` or for Kinesis Data Firehose use `kinesis-firehose`.
    #[builder(into)]
    #[serde(rename = "destinationType")]
    pub r#destination_type: Box<String>,
    /// Valid values are `json` or `text`
    #[builder(into)]
    #[serde(rename = "logFormat")]
    pub r#log_format: Box<String>,
    /// Valid values are  `slow-log` or `engine-log`. Max 1 of each.
    #[builder(into)]
    #[serde(rename = "logType")]
    pub r#log_type: Box<String>,
}
