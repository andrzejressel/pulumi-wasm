#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct NetworkWatcherFlowLogRetentionPolicy {
    /// The number of days to retain flow log records.
    #[builder(into)]
    #[serde(rename = "days")]
    pub r#days: Box<i32>,
    /// Boolean flag to enable/disable retention.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
}
