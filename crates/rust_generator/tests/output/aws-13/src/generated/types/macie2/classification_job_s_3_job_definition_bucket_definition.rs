#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClassificationJobS3JobDefinitionBucketDefinition {
    /// The unique identifier for the AWS account that owns the buckets.
    #[builder(into)]
    #[serde(rename = "accountId")]
    pub r#account_id: Box<String>,
    /// An array that lists the names of the buckets.
    #[builder(into)]
    #[serde(rename = "buckets")]
    pub r#buckets: Box<Vec<String>>,
}
