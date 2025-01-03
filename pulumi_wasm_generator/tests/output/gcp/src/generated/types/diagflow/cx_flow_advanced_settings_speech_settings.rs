#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CxFlowAdvancedSettingsSpeechSettings {
    /// Sensitivity of the speech model that detects the end of speech. Scale from 0 to 100.
    #[builder(into, default)]
    #[serde(rename = "endpointerSensitivity")]
    pub r#endpointer_sensitivity: Box<Option<i32>>,
    /// Mapping from language to Speech-to-Text model. The mapped Speech-to-Text model will be selected for requests from its corresponding language. For more information, see [Speech models](https://cloud.google.com/dialogflow/cx/docs/concept/speech-models).
    /// An object containing a list of **"key": value** pairs. Example: **{ "name": "wrench", "mass": "1.3kg", "count": "3" }**.
    #[builder(into, default)]
    #[serde(rename = "models")]
    pub r#models: Box<Option<std::collections::HashMap<String, String>>>,
    /// Timeout before detecting no speech.
    /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
    #[builder(into, default)]
    #[serde(rename = "noSpeechTimeout")]
    pub r#no_speech_timeout: Box<Option<String>>,
    /// Use timeout based endpointing, interpreting endpointer sensitivity as seconds of timeout value.
    #[builder(into, default)]
    #[serde(rename = "useTimeoutBasedEndpointing")]
    pub r#use_timeout_based_endpointing: Box<Option<bool>>,
}
