#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SettingsApplicationSettingsAccessDeniedPageSettings {
    /// The URI to be redirected to when access is denied.
    #[builder(into, default)]
    #[serde(rename = "accessDeniedPageUri")]
    pub r#access_denied_page_uri: Box<Option<String>>,
    /// Whether to generate a troubleshooting URL on access denied events to this application.
    #[builder(into, default)]
    #[serde(rename = "generateTroubleshootingUri")]
    pub r#generate_troubleshooting_uri: Box<Option<bool>>,
    /// Whether to generate remediation token on access denied events to this application.
    #[builder(into, default)]
    #[serde(rename = "remediationTokenGenerationEnabled")]
    pub r#remediation_token_generation_enabled: Box<Option<bool>>,
}
