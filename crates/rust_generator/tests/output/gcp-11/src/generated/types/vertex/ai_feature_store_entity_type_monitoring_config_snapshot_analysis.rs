#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AiFeatureStoreEntityTypeMonitoringConfigSnapshotAnalysis {
    /// The monitoring schedule for snapshot analysis. For EntityType-level config: unset / disabled = true indicates disabled by default for Features under it; otherwise by default enable snapshot analysis monitoring with monitoringInterval for Features under it.
    #[builder(into, default)]
    #[serde(rename = "disabled")]
    pub r#disabled: Box<Option<bool>>,
    /// Configuration of the snapshot analysis based monitoring pipeline running interval. The value is rolled up to full day.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    /// 
    /// > **Warning:** `monitoring_interval` is deprecated and will be removed in a future release.
    #[builder(into, default)]
    #[serde(rename = "monitoringInterval")]
    pub r#monitoring_interval: Box<Option<String>>,
    /// Configuration of the snapshot analysis based monitoring pipeline running interval. The value indicates number of days. The default value is 1.
    /// If both FeaturestoreMonitoringConfig.SnapshotAnalysis.monitoring_interval_days and [FeaturestoreMonitoringConfig.SnapshotAnalysis.monitoring_interval][] are set when creating/updating EntityTypes/Features, FeaturestoreMonitoringConfig.SnapshotAnalysis.monitoring_interval_days will be used.
    #[builder(into, default)]
    #[serde(rename = "monitoringIntervalDays")]
    pub r#monitoring_interval_days: Box<Option<i32>>,
    /// Customized export features time window for snapshot analysis. Unit is one day. The default value is 21 days. Minimum value is 1 day. Maximum value is 4000 days.
    #[builder(into, default)]
    #[serde(rename = "stalenessDays")]
    pub r#staleness_days: Box<Option<i32>>,
}
