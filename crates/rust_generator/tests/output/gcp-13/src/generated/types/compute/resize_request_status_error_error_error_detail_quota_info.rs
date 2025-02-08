#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ResizeRequestStatusErrorErrorErrorDetailQuotaInfo {
    /// (Output)
    /// The map holding related quota dimensions
    #[builder(into, default)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Box<Option<std::collections::HashMap<String, String>>>,
    /// (Output)
    /// Future quota limit being rolled out. The limit's unit depends on the quota type or metric.
    #[builder(into, default)]
    #[serde(rename = "futureLimit")]
    pub r#future_limit: Box<Option<i32>>,
    /// (Output)
    /// Current effective quota limit. The limit's unit depends on the quota type or metric.
    #[builder(into, default)]
    #[serde(rename = "limit")]
    pub r#limit: Box<Option<i32>>,
    /// (Output)
    /// The name of the quota limit.
    #[builder(into, default)]
    #[serde(rename = "limitName")]
    pub r#limit_name: Box<Option<String>>,
    /// (Output)
    /// The Compute Engine quota metric name.
    #[builder(into, default)]
    #[serde(rename = "metricName")]
    pub r#metric_name: Box<Option<String>>,
    /// (Output)
    /// Rollout status of the future quota limit.
    #[builder(into, default)]
    #[serde(rename = "rolloutStatus")]
    pub r#rollout_status: Box<Option<String>>,
}
