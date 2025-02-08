#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2ModelsSlotValueElicitationSettingPromptSpecificationPromptAttemptsSpecificationAllowedInputTypes {
    #[builder(into)]
    #[serde(rename = "allowAudioInput")]
    pub r#allow_audio_input: Box<bool>,
    #[builder(into)]
    #[serde(rename = "allowDtmfInput")]
    pub r#allow_dtmf_input: Box<bool>,
}
