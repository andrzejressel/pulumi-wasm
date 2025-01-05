#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EnvironmentLastUpdatedError {
    #[builder(into, default)]
    #[serde(rename = "errorCode")]
    pub r#error_code: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "errorMessage")]
    pub r#error_message: Box<Option<String>>,
}
