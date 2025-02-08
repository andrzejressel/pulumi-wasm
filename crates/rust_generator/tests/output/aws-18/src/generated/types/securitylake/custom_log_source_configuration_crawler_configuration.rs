#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CustomLogSourceConfigurationCrawlerConfiguration {
    /// The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role to be used by the AWS Glue crawler.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
}
