#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsSlotTypeValueSelectionSettingAdvancedRecognitionSetting {
    /// Enables using the slot values as a custom vocabulary for recognizing user utterances.
    /// Valid value is `UseSlotValuesAsCustomVocabulary`.
    #[builder(into, default)]
    #[serde(rename = "audioRecognitionStrategy")]
    pub r#audio_recognition_strategy: Box<Option<String>>,
}
