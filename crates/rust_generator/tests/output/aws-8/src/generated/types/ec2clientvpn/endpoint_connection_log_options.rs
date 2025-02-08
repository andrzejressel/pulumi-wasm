#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EndpointConnectionLogOptions {
    /// The name of the CloudWatch Logs log group.
    #[builder(into, default)]
    #[serde(rename = "cloudwatchLogGroup")]
    pub r#cloudwatch_log_group: Box<Option<String>>,
    /// The name of the CloudWatch Logs log stream to which the connection data is published.
    #[builder(into, default)]
    #[serde(rename = "cloudwatchLogStream")]
    pub r#cloudwatch_log_stream: Box<Option<String>>,
    /// Indicates whether connection logging is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
}
