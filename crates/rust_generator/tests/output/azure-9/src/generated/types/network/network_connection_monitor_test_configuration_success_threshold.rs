#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NetworkConnectionMonitorTestConfigurationSuccessThreshold {
    /// The maximum percentage of failed checks permitted for a test to be successful.
    #[builder(into, default)]
    #[serde(rename = "checksFailedPercent")]
    pub r#checks_failed_percent: Box<Option<i32>>,
    /// The maximum round-trip time in milliseconds permitted for a test to be successful.
    #[builder(into, default)]
    #[serde(rename = "roundTripTimeMs")]
    pub r#round_trip_time_ms: Box<Option<f64>>,
}
