#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsSlotTypeValueSelectionSetting {
    /// Provides settings that enable advanced recognition settings for slot values.
    /// You can use this to enable using slot values as a custom vocabulary for recognizing user utterances.
    /// See `advanced_recognition_setting` argument reference below.
    #[builder(into, default)]
    #[serde(rename = "advancedRecognitionSettings")]
    pub r#advanced_recognition_settings: Box<Option<Vec<super::super::types::lex::V2ModelsSlotTypeValueSelectionSettingAdvancedRecognitionSetting>>>,
    /// Used to validate the value of the slot.
    /// See `regex_filter` argument reference below.
    #[builder(into, default)]
    #[serde(rename = "regexFilters")]
    pub r#regex_filters: Box<Option<Vec<super::super::types::lex::V2ModelsSlotTypeValueSelectionSettingRegexFilter>>>,
    /// Determines the slot resolution strategy that Amazon Lex uses to return slot type values.
    /// Valid values are `OriginalValue`, `TopResolution`, and `Concatenation`.
    #[builder(into)]
    #[serde(rename = "resolutionStrategy")]
    pub r#resolution_strategy: Box<String>,
}
