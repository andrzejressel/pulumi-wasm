#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServerlessCacheCacheUsageLimitsDataStorage {
    /// The upper limit for data storage the cache is set to use. Must be between 1 and 5,000.
    #[builder(into, default)]
    #[serde(rename = "maximum")]
    pub r#maximum: Box<Option<i32>>,
    /// The lower limit for data storage the cache is set to use. Must be between 1 and 5,000.
    #[builder(into, default)]
    #[serde(rename = "minimum")]
    pub r#minimum: Box<Option<i32>>,
    /// The unit that the storage is measured in, in GB.
    #[builder(into)]
    #[serde(rename = "unit")]
    pub r#unit: Box<String>,
}
