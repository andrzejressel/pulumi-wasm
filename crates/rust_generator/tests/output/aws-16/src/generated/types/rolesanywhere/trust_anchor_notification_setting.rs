#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TrustAnchorNotificationSetting {
    #[builder(into, default)]
    #[serde(rename = "channel")]
    pub r#channel: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "configuredBy")]
    pub r#configured_by: Box<Option<String>>,
    /// Whether or not the Trust Anchor should be enabled.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "event")]
    pub r#event: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "threshold")]
    pub r#threshold: Box<Option<i32>>,
}
