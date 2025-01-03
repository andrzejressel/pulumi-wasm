#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IntentFollowupIntentInfo {
    /// The unique identifier of the followup intent.
    /// Format: projects/<Project ID>/agent/intents/<Intent ID>.
    #[builder(into, default)]
    #[serde(rename = "followupIntentName")]
    pub r#followup_intent_name: Box<Option<String>>,
    /// The unique identifier of the parent intent in the chain of followup intents.
    /// Format: projects/<Project ID>/agent/intents/<Intent ID>.
    #[builder(into, default)]
    #[serde(rename = "parentFollowupIntentName")]
    pub r#parent_followup_intent_name: Box<Option<String>>,
}
