#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationBucketingConfig {
    /// Set of buckets. Ranges must be non-overlapping.
    /// Bucket is represented as a range, along with replacement values.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "buckets")]
    pub r#buckets: Box<Option<Vec<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationBucketingConfigBucket>>>,
}
