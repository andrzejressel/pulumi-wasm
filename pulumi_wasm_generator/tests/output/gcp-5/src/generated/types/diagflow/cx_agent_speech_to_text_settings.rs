#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CxAgentSpeechToTextSettings {
    /// Whether to use speech adaptation for speech recognition.
    #[builder(into, default)]
    #[serde(rename = "enableSpeechAdaptation")]
    pub r#enable_speech_adaptation: Box<Option<bool>>,
}
