#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsSlotValueElicitationSettingSlotResolutionSetting {
    /// Specifies whether assisted slot resolution is turned on for the slot or not.
    /// Valid values are `EnhancedFallback` or `Default`.
    /// If the value is `EnhancedFallback`, assisted slot resolution is activated when Amazon Lex defaults to the `AMAZON.FallbackIntent`.
    /// If the value is `Default`, assisted slot resolution is turned off.
    #[builder(into)]
    #[serde(rename = "slotResolutionStrategy")]
    pub r#slot_resolution_strategy: Box<String>,
}
