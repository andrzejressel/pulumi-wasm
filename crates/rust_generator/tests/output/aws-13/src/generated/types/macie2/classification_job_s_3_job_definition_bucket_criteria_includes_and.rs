#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClassificationJobS3JobDefinitionBucketCriteriaIncludesAnd {
    /// A property-based condition that defines a property, operator, and one or more values for including or excluding an S3 buckets from the job. (documented below)
    #[builder(into, default)]
    #[serde(rename = "simpleCriterion")]
    pub r#simple_criterion: Box<Option<super::super::types::macie2::ClassificationJobS3JobDefinitionBucketCriteriaIncludesAndSimpleCriterion>>,
    /// A tag-based condition that defines the operator and tag keys or tag key and value pairs for including or excluding an S3 buckets from the job. (documented below)
    #[builder(into, default)]
    #[serde(rename = "tagCriterion")]
    pub r#tag_criterion: Box<Option<super::super::types::macie2::ClassificationJobS3JobDefinitionBucketCriteriaIncludesAndTagCriterion>>,
}
