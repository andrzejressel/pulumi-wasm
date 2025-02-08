#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClassificationJobS3JobDefinitionBucketCriteriaIncludes {
    /// An array of conditions, one for each condition that determines which objects to include or exclude from the job. (documented below)
    #[builder(into, default)]
    #[serde(rename = "ands")]
    pub r#ands: Box<Option<Vec<super::super::types::macie2::ClassificationJobS3JobDefinitionBucketCriteriaIncludesAnd>>>,
}
