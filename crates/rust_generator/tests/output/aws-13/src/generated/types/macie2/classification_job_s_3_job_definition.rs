#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClassificationJobS3JobDefinition {
    /// The property- and tag-based conditions that determine which S3 buckets to include or exclude from the analysis. Conflicts with `bucket_definitions`. (documented below)
    #[builder(into, default)]
    #[serde(rename = "bucketCriteria")]
    pub r#bucket_criteria: Box<Option<super::super::types::macie2::ClassificationJobS3JobDefinitionBucketCriteria>>,
    /// An array of objects, one for each AWS account that owns buckets to analyze. Each object specifies the account ID for an account and one or more buckets to analyze for the account. Conflicts with `bucket_criteria`. (documented below)
    #[builder(into, default)]
    #[serde(rename = "bucketDefinitions")]
    pub r#bucket_definitions: Box<Option<Vec<super::super::types::macie2::ClassificationJobS3JobDefinitionBucketDefinition>>>,
    /// The property- and tag-based conditions that determine which objects to include or exclude from the analysis. (documented below)
    #[builder(into, default)]
    #[serde(rename = "scoping")]
    pub r#scoping: Box<Option<super::super::types::macie2::ClassificationJobS3JobDefinitionScoping>>,
}
