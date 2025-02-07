#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectorLogDeliveryWorkerLogDeliveryCloudwatchLogs {
    /// Whether log delivery to Amazon CloudWatch Logs is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// The name of the CloudWatch log group that is the destination for log delivery.
    #[builder(into, default)]
    #[serde(rename = "logGroup")]
    pub r#log_group: Box<Option<String>>,
}
