#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AlertPolicyAlertStrategyNotificationRateLimit {
    /// Not more than one notification per period.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example "60.5s".
    #[builder(into, default)]
    #[serde(rename = "period")]
    pub r#period: Box<Option<String>>,
}
