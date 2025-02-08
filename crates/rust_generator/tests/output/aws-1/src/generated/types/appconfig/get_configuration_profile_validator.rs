#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetConfigurationProfileValidator {
    /// Either the JSON Schema content or the ARN of an AWS Lambda function.
    #[builder(into)]
    #[serde(rename = "content")]
    pub r#content: Box<String>,
    /// Type of validator. Valid values: JSON_SCHEMA and LAMBDA.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
