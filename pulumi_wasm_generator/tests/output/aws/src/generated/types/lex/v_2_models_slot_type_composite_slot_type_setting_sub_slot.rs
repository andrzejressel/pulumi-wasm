#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsSlotTypeCompositeSlotTypeSettingSubSlot {
    /// Name of a constituent sub slot inside a composite slot.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Unique identifier assigned to a slot type.
    /// This refers to either a built-in slot type or the unique `slot_type_id` of a custom slot type.
    #[builder(into)]
    #[serde(rename = "slotTypeId")]
    pub r#slot_type_id: Box<String>,
}