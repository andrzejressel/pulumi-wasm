#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CxFlowAdvancedSettingsLoggingSettings {
    /// Enables consent-based end-user input redaction, if true, a pre-defined session parameter **$session.params.conversation-redaction** will be used to determine if the utterance should be redacted.
    #[builder(into, default)]
    #[serde(rename = "enableConsentBasedRedaction")]
    pub r#enable_consent_based_redaction: Box<Option<bool>>,
    /// Enables DF Interaction logging.
    #[builder(into, default)]
    #[serde(rename = "enableInteractionLogging")]
    pub r#enable_interaction_logging: Box<Option<bool>>,
    /// Enables Google Cloud Logging.
    #[builder(into, default)]
    #[serde(rename = "enableStackdriverLogging")]
    pub r#enable_stackdriver_logging: Box<Option<bool>>,
}
