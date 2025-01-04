#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAppServiceConnectionString {
    /// The name of the App Service.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The type of the Connection String.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    /// The value for the Connection String.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
