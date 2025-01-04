#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ProvisionedProductProvisioningParameter {
    /// Parameter key.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// Whether to ignore `value` and keep the previous parameter value. Ignored when initially provisioning a product.
    #[builder(into, default)]
    #[serde(rename = "usePreviousValue")]
    pub r#use_previous_value: Box<Option<bool>>,
    /// Parameter value.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
