#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ParameterGroupParameter {
    /// The apply method of the Neptune parameter. Valid values are `immediate` and `pending-reboot`. Defaults to `pending-reboot`.
    #[builder(into, default)]
    #[serde(rename = "applyMethod")]
    pub r#apply_method: Box<Option<String>>,
    /// The name of the Neptune parameter.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The value of the Neptune parameter.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}