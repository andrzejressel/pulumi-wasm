#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct V2ModelsIntentConfirmationSettingFailureNextStepIntent {
    /// Name of the intent.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Configuration block for all of the slot value overrides for the intent. The name of the slot maps to the value of the slot. Slots that are not included in the map aren't overridden. See `slot`.
    #[builder(into, default)]
    #[serde(rename = "slots")]
    pub r#slots: Box<Option<Vec<super::super::types::lex::V2ModelsIntentConfirmationSettingFailureNextStepIntentSlot>>>,
}
