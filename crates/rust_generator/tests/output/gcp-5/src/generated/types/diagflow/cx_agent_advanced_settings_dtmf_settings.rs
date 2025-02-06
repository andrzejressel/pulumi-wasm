#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CxAgentAdvancedSettingsDtmfSettings {
    /// If true, incoming audio is processed for DTMF (dual tone multi frequency) events. For example, if the caller presses a button on their telephone keypad and DTMF processing is enabled, Dialogflow will detect the event (e.g. a "3" was pressed) in the incoming audio and pass the event to the bot to drive business logic (e.g. when 3 is pressed, return the account balance).
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// The digit that terminates a DTMF digit sequence.
    #[builder(into, default)]
    #[serde(rename = "finishDigit")]
    pub r#finish_digit: Box<Option<String>>,
    /// Max length of DTMF digits.
    #[builder(into, default)]
    #[serde(rename = "maxDigits")]
    pub r#max_digits: Box<Option<i32>>,
}
