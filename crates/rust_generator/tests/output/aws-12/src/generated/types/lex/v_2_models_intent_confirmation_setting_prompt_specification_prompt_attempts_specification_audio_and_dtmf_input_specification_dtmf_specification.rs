#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsIntentConfirmationSettingPromptSpecificationPromptAttemptsSpecificationAudioAndDtmfInputSpecificationDtmfSpecification {
    /// DTMF character that clears the accumulated DTMF digits and immediately ends the input.
    #[builder(into)]
    #[serde(rename = "deletionCharacter")]
    pub r#deletion_character: Box<String>,
    /// DTMF character that immediately ends input. If the user does not press this character, the input ends after the end timeout.
    #[builder(into)]
    #[serde(rename = "endCharacter")]
    pub r#end_character: Box<String>,
    /// How long the bot should wait after the last DTMF character input before assuming that the input has concluded.
    #[builder(into)]
    #[serde(rename = "endTimeoutMs")]
    pub r#end_timeout_ms: Box<i32>,
    /// Maximum number of DTMF digits allowed in an utterance.
    #[builder(into)]
    #[serde(rename = "maxLength")]
    pub r#max_length: Box<i32>,
}
