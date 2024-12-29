#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CanaryTimeline {
    /// Date and time the canary was created.
    #[builder(into, default)]
    #[serde(rename = "created")]
    pub r#created: Box<Option<String>>,
    /// Date and time the canary was most recently modified.
    #[builder(into, default)]
    #[serde(rename = "lastModified")]
    pub r#last_modified: Box<Option<String>>,
    /// Date and time that the canary's most recent run started.
    #[builder(into, default)]
    #[serde(rename = "lastStarted")]
    pub r#last_started: Box<Option<String>>,
    /// Date and time that the canary's most recent run ended.
    #[builder(into, default)]
    #[serde(rename = "lastStopped")]
    pub r#last_stopped: Box<Option<String>>,
}
