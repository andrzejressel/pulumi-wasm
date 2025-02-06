#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsIntentConfirmationSettingFailureNextStepIntentSlot {
    /// Which attempt to configure. Valid values are `Initial`, `Retry1`, `Retry2`, `Retry3`, `Retry4`, `Retry5`.
    #[builder(into)]
    #[serde(rename = "mapBlockKey")]
    pub r#map_block_key: Box<String>,
    /// When the shape value is `List`, `values` contains a list of slot values. When the value is `Scalar`, `value` contains a single value.
    #[builder(into, default)]
    #[serde(rename = "shape")]
    pub r#shape: Box<Option<String>>,
    /// Configuration block for the current value of the slot. See `value`.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<super::super::types::lex::V2ModelsIntentConfirmationSettingFailureNextStepIntentSlotValue>>,
}
