#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsIntentSlotPriority {
    /// Priority that Amazon Lex should apply to the slot.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<i32>,
    /// Unique identifier of the slot.
    #[builder(into)]
    #[serde(rename = "slotId")]
    pub r#slot_id: Box<String>,
}
