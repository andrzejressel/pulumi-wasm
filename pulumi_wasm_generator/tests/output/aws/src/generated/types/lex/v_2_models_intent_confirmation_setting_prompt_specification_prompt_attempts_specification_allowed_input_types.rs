#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsIntentConfirmationSettingPromptSpecificationPromptAttemptsSpecificationAllowedInputTypes {
    /// Whether audio input is allowed.
    #[builder(into)]
    #[serde(rename = "allowAudioInput")]
    pub r#allow_audio_input: Box<bool>,
    /// Whether DTMF input is allowed.
    #[builder(into)]
    #[serde(rename = "allowDtmfInput")]
    pub r#allow_dtmf_input: Box<bool>,
}