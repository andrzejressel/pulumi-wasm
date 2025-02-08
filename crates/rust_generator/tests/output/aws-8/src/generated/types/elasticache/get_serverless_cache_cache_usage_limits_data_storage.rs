#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetServerlessCacheCacheUsageLimitsDataStorage {
    /// The maximum number of ECPUs the cache can consume per second.
    #[builder(into)]
    #[serde(rename = "maximum")]
    pub r#maximum: Box<i32>,
    /// The minimum number of ECPUs the cache can consume per second.
    #[builder(into)]
    #[serde(rename = "minimum")]
    pub r#minimum: Box<i32>,
    /// The unit that the storage is measured in.
    #[builder(into)]
    #[serde(rename = "unit")]
    pub r#unit: Box<String>,
}
