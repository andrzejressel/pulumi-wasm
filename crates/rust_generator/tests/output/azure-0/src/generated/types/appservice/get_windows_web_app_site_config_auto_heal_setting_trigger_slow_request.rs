#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetWindowsWebAppSiteConfigAutoHealSettingTriggerSlowRequest {
    /// The number of occurrences of the defined `status_code` in the specified `interval` on which to trigger this rule.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Box<i32>,
    /// The time interval in the form `hh:mm:ss`.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Box<String>,
    /// (Required) The threshold of time passed to qualify as a Slow Request in `hh:mm:ss`.
    #[builder(into)]
    #[serde(rename = "timeTaken")]
    pub r#time_taken: Box<String>,
}
