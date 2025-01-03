#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSQuotaInfosQuotaInfo {
    /// (Output) The container type of the QuotaInfo.
    #[builder(into)]
    #[serde(rename = "containerType")]
    pub r#container_type: Box<String>,
    /// The map of dimensions for this dimensions info. The key of a map entry is "region", "zone" or the name of a service specific dimension, and the value of a map entry is the value of the dimension. If a dimension does not appear in the map of dimensions, the dimensions info applies to all the dimension values except for those that have another DimenisonInfo instance configured for the specific value. Example: {"provider" : "Foo Inc"} where "provider" is a service specific dimension of a quota.
    #[builder(into)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Box<Vec<String>>,
    /// (Output) The collection of dimensions info ordered by their dimensions from more specific ones to less specific ones.
    #[builder(into)]
    #[serde(rename = "dimensionsInfos")]
    pub r#dimensions_infos: Box<Vec<super::super::types::cloudquota::GetSQuotaInfosQuotaInfoDimensionsInfo>>,
    /// (Output) Whether the quota is a concurrent quota. Concurrent quotas are enforced on the total number of concurrent operations in flight at any given time.
    #[builder(into)]
    #[serde(rename = "isConcurrent")]
    pub r#is_concurrent: Box<bool>,
    /// (Output) Whether the quota value is fixed or adjustable.
    #[builder(into)]
    #[serde(rename = "isFixed")]
    pub r#is_fixed: Box<bool>,
    /// (Output) Whether this is a precise quota. A precise quota is tracked with absolute precision. In contrast, an imprecise quota is not tracked with precision.
    #[builder(into)]
    #[serde(rename = "isPrecise")]
    pub r#is_precise: Box<bool>,
    /// (Output) The metric of the quota. It specifies the resources consumption the quota is defined for, for example: `compute.googleapis.com/cpus`.
    #[builder(into)]
    #[serde(rename = "metric")]
    pub r#metric: Box<String>,
    /// (Output) The display name of the quota metric.
    #[builder(into)]
    #[serde(rename = "metricDisplayName")]
    pub r#metric_display_name: Box<String>,
    /// (Output) The unit in which the metric value is reported, e.g., `MByte`.
    #[builder(into)]
    #[serde(rename = "metricUnit")]
    pub r#metric_unit: Box<String>,
    /// (Output) Resource name of this QuotaInfo, for example: `projects/123/locations/global/services/compute.googleapis.com/quotaInfos/CpusPerProjectPerRegion`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// (Output) The display name of the quota.
    #[builder(into)]
    #[serde(rename = "quotaDisplayName")]
    pub r#quota_display_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "quotaId")]
    pub r#quota_id: Box<String>,
    /// (Output) Whether it is eligible to request a higher quota value for this quota.
    #[builder(into)]
    #[serde(rename = "quotaIncreaseEligibilities")]
    pub r#quota_increase_eligibilities: Box<Vec<super::super::types::cloudquota::GetSQuotaInfosQuotaInfoQuotaIncreaseEligibility>>,
    /// (Output) The reset time interval for the quota. Refresh interval applies to rate quota only. Example: "minute" for per minute, "day" for per day, or "10 seconds" for every 10 seconds.
    #[builder(into)]
    #[serde(rename = "refreshInterval")]
    pub r#refresh_interval: Box<String>,
    /// The name of the service in which the quotas are defined.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Box<String>,
    /// (Output) URI to the page where users can request more quota for the cloud service, for example: `https://console.cloud.google.com/iam-admin/quotas`.
    #[builder(into)]
    #[serde(rename = "serviceRequestQuotaUri")]
    pub r#service_request_quota_uri: Box<String>,
}
