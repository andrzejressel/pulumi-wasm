#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DataSourceCustomDocumentEnrichmentConfigurationPostExtractionHookConfiguration {
    /// A block that specifies the condition used for when a Lambda function should be invoked. For example, you can specify a condition that if there are empty date-time values, then Amazon Kendra should invoke a function that inserts the current date-time. See invocation_condition.
    #[builder(into, default)]
    #[serde(rename = "invocationCondition")]
    pub r#invocation_condition: Box<Option<super::super::types::kendra::DataSourceCustomDocumentEnrichmentConfigurationPostExtractionHookConfigurationInvocationCondition>>,
    /// The Amazon Resource Name (ARN) of a Lambda Function that can manipulate your document metadata fields or attributes and content.
    #[builder(into)]
    #[serde(rename = "lambdaArn")]
    pub r#lambda_arn: Box<String>,
    /// Stores the original, raw documents or the structured, parsed documents before and after altering them. For more information, see [Data contracts for Lambda functions](https://docs.aws.amazon.com/kendra/latest/dg/custom-document-enrichment.html#cde-data-contracts-lambda).
    #[builder(into)]
    #[serde(rename = "s3Bucket")]
    pub r#s_3_bucket: Box<String>,
}
