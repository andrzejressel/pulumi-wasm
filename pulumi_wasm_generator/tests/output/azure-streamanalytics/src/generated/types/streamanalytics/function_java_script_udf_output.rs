#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FunctionJavaScriptUdfOutput {
    /// The Data Type output from this JavaScript Function. Possible values include `array`, `any`, `bigint`, `datetime`, `float`, `nvarchar(max)` and `record`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
