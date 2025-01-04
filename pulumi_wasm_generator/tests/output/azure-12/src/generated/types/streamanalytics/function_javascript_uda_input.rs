#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FunctionJavascriptUdaInput {
    /// Is this input parameter a configuration parameter? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "configurationParameter")]
    pub r#configuration_parameter: Box<Option<bool>>,
    /// The input data type of this JavaScript Function. Possible values include `any`, `array`, `bigint`, `datetime`, `float`, `nvarchar(max)` and `record`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
