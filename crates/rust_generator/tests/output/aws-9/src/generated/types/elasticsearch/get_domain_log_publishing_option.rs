#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDomainLogPublishingOption {
    /// The CloudWatch Log Group where the logs are published.
    #[builder(into)]
    #[serde(rename = "cloudwatchLogGroupArn")]
    pub r#cloudwatch_log_group_arn: Box<String>,
    /// Whether node to node encryption is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// The type of Elasticsearch log being published.
    #[builder(into)]
    #[serde(rename = "logType")]
    pub r#log_type: Box<String>,
}
