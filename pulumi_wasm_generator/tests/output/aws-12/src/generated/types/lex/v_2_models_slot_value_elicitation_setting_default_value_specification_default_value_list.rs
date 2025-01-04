#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsSlotValueElicitationSettingDefaultValueSpecificationDefaultValueList {
    /// Default value to use when a user doesn't provide a value for a slot.
    #[builder(into)]
    #[serde(rename = "defaultValue")]
    pub r#default_value: Box<String>,
}
