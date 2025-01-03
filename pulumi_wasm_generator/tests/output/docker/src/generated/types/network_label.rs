#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NetworkLabel {
    /// Name of the label
    #[builder(into)]
    #[serde(rename = "label")]
    pub r#label: Box<String>,
    /// Value of the label
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
