#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConformancePackInputParameter {
    /// The input key.
    #[builder(into)]
    #[serde(rename = "parameterName")]
    pub r#parameter_name: Box<String>,
    /// The input value.
    #[builder(into)]
    #[serde(rename = "parameterValue")]
    pub r#parameter_value: Box<String>,
}
