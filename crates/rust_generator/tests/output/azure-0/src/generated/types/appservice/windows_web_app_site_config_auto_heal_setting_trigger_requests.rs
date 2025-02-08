#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WindowsWebAppSiteConfigAutoHealSettingTriggerRequests {
    /// The number of requests in the specified `interval` to trigger this rule.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Box<i32>,
    /// The interval in `hh:mm:ss`.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Box<String>,
}
