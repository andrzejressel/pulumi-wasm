#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BucketIntelligentTieringConfigurationTiering {
    /// S3 Intelligent-Tiering access tier. Valid values: `ARCHIVE_ACCESS`, `DEEP_ARCHIVE_ACCESS`.
    #[builder(into)]
    #[serde(rename = "accessTier")]
    pub r#access_tier: Box<String>,
    /// Number of consecutive days of no access after which an object will be eligible to be transitioned to the corresponding tier.
    #[builder(into)]
    #[serde(rename = "days")]
    pub r#days: Box<i32>,
}
