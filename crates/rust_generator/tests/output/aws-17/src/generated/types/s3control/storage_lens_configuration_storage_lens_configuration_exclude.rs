#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StorageLensConfigurationStorageLensConfigurationExclude {
    /// List of S3 bucket ARNs.
    #[builder(into, default)]
    #[serde(rename = "buckets")]
    pub r#buckets: Box<Option<Vec<String>>>,
    /// List of AWS Regions.
    #[builder(into, default)]
    #[serde(rename = "regions")]
    pub r#regions: Box<Option<Vec<String>>>,
}
