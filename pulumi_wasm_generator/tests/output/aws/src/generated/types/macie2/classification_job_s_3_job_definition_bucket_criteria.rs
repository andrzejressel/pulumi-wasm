#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClassificationJobS3JobDefinitionBucketCriteria {
    /// The property- or tag-based conditions that determine which S3 buckets to exclude from the analysis. (documented below)
    #[builder(into, default)]
    #[serde(rename = "excludes")]
    pub r#excludes: Box<Option<super::super::types::macie2::ClassificationJobS3JobDefinitionBucketCriteriaExcludes>>,
    /// The property- or tag-based conditions that determine which S3 buckets to include in the analysis. (documented below)
    #[builder(into, default)]
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<super::super::types::macie2::ClassificationJobS3JobDefinitionBucketCriteriaIncludes>>,
}