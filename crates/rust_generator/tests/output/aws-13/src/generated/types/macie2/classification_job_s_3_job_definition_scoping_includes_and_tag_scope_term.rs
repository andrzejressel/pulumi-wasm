#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClassificationJobS3JobDefinitionScopingIncludesAndTagScopeTerm {
    /// The operator to use in the condition.
    #[builder(into, default)]
    #[serde(rename = "comparator")]
    pub r#comparator: Box<Option<String>>,
    /// The tag key to use in the condition. The only valid value is `TAG`.
    #[builder(into, default)]
    #[serde(rename = "key")]
    pub r#key: Box<Option<String>>,
    /// The tag keys or tag key and value pairs to use in the condition.
    #[builder(into, default)]
    #[serde(rename = "tagValues")]
    pub r#tag_values: Box<Option<Vec<super::super::types::macie2::ClassificationJobS3JobDefinitionScopingIncludesAndTagScopeTermTagValue>>>,
    /// The type of object to apply the condition to. The only valid value is `S3_OBJECT`.
    #[builder(into, default)]
    #[serde(rename = "target")]
    pub r#target: Box<Option<String>>,
}
