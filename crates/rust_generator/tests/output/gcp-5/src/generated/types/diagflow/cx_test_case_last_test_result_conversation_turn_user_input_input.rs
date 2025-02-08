#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CxTestCaseLastTestResultConversationTurnUserInputInput {
    /// The DTMF event to be handled.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "dtmf")]
    pub r#dtmf: Box<Option<super::super::types::diagflow::CxTestCaseLastTestResultConversationTurnUserInputInputDtmf>>,
    /// The event to be triggered.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "event")]
    pub r#event: Box<Option<super::super::types::diagflow::CxTestCaseLastTestResultConversationTurnUserInputInputEvent>>,
    /// The language of the input. See [Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language) for a list of the currently supported language codes.
    /// Note that queries in the same session do not necessarily need to specify the same language.
    #[builder(into, default)]
    #[serde(rename = "languageCode")]
    pub r#language_code: Box<Option<String>>,
    /// The natural language text to be processed.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "text")]
    pub r#text: Box<Option<super::super::types::diagflow::CxTestCaseLastTestResultConversationTurnUserInputInputText>>,
}
