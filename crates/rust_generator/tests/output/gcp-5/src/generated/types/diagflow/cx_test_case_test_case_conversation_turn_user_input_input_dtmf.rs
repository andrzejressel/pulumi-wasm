#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CxTestCaseTestCaseConversationTurnUserInputInputDtmf {
    /// The dtmf digits.
    #[builder(into, default)]
    #[serde(rename = "digits")]
    pub r#digits: Box<Option<String>>,
    /// The finish digit (if any).
    #[builder(into, default)]
    #[serde(rename = "finishDigit")]
    pub r#finish_digit: Box<Option<String>>,
}
