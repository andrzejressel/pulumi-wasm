#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct LinuxWebAppSlotSiteConfigAutoHealSettingTriggerSlowRequestWithPath {
    /// The number of Slow Requests in the time `interval` to trigger this rule.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Box<i32>,
    /// The time interval in the form `hh:mm:ss`.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Box<String>,
    /// The path for which this slow request rule applies.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// The threshold of time passed to qualify as a Slow Request in `hh:mm:ss`.
    #[builder(into)]
    #[serde(rename = "timeTaken")]
    pub r#time_taken: Box<String>,
}
