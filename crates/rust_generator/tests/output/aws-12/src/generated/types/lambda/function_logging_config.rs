#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FunctionLoggingConfig {
    /// for JSON structured logs, choose the detail level of the logs your application sends to CloudWatch when using supported logging libraries.
    #[builder(into, default)]
    #[serde(rename = "applicationLogLevel")]
    pub r#application_log_level: Box<Option<String>>,
    /// select between `Text` and structured `JSON` format for your function's logs.
    #[builder(into)]
    #[serde(rename = "logFormat")]
    pub r#log_format: Box<String>,
    /// the CloudWatch log group your function sends logs to.
    #[builder(into, default)]
    #[serde(rename = "logGroup")]
    pub r#log_group: Box<Option<String>>,
    /// for JSON structured logs, choose the detail level of the Lambda platform event logs sent to CloudWatch, such as `ERROR`, `DEBUG`, or `INFO`.
    #[builder(into, default)]
    #[serde(rename = "systemLogLevel")]
    pub r#system_log_level: Box<Option<String>>,
}
