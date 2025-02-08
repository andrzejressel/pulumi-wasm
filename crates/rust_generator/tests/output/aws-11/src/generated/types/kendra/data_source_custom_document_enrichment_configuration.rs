#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSourceCustomDocumentEnrichmentConfiguration {
    /// Configuration information to alter document attributes or metadata fields and content when ingesting documents into Amazon Kendra. Minimum number of `0` items. Maximum number of `100` items. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "inlineConfigurations")]
    pub r#inline_configurations: Box<Option<Vec<super::super::types::kendra::DataSourceCustomDocumentEnrichmentConfigurationInlineConfiguration>>>,
    /// A block that specifies the configuration information for invoking a Lambda function in AWS Lambda on the structured documents with their metadata and text extracted. You can use a Lambda function to apply advanced logic for creating, modifying, or deleting document metadata and content. For more information, see [Advanced data manipulation](https://docs.aws.amazon.com/kendra/latest/dg/custom-document-enrichment.html#advanced-data-manipulation). Detailed below.
    #[builder(into, default)]
    #[serde(rename = "postExtractionHookConfiguration")]
    pub r#post_extraction_hook_configuration: Box<Option<super::super::types::kendra::DataSourceCustomDocumentEnrichmentConfigurationPostExtractionHookConfiguration>>,
    /// Configuration information for invoking a Lambda function in AWS Lambda on the original or raw documents before extracting their metadata and text. You can use a Lambda function to apply advanced logic for creating, modifying, or deleting document metadata and content. For more information, see [Advanced data manipulation](https://docs.aws.amazon.com/kendra/latest/dg/custom-document-enrichment.html#advanced-data-manipulation). Detailed below.
    #[builder(into, default)]
    #[serde(rename = "preExtractionHookConfiguration")]
    pub r#pre_extraction_hook_configuration: Box<Option<super::super::types::kendra::DataSourceCustomDocumentEnrichmentConfigurationPreExtractionHookConfiguration>>,
    /// The Amazon Resource Name (ARN) of a role with permission to run `pre_extraction_hook_configuration` and `post_extraction_hook_configuration` for altering document metadata and content during the document ingestion process. For more information, see [IAM roles for Amazon Kendra](https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html).
    #[builder(into, default)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<Option<String>>,
}
