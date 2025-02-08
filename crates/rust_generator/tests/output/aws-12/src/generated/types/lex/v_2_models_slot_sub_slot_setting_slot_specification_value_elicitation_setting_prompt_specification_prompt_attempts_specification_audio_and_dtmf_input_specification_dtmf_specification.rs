#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingPromptSpecificationPromptAttemptsSpecificationAudioAndDtmfInputSpecificationDtmfSpecification {
    #[builder(into)]
    #[serde(rename = "deletionCharacter")]
    pub r#deletion_character: Box<String>,
    #[builder(into)]
    #[serde(rename = "endCharacter")]
    pub r#end_character: Box<String>,
    #[builder(into)]
    #[serde(rename = "endTimeoutMs")]
    pub r#end_timeout_ms: Box<i32>,
    #[builder(into)]
    #[serde(rename = "maxLength")]
    pub r#max_length: Box<i32>,
}
