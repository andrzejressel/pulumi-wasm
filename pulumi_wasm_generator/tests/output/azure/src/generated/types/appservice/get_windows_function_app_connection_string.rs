#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetWindowsFunctionAppConnectionString {
    /// The name of this Windows Function App.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The type of Managed Service Identity that is configured on this Windows Function App.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    /// The connection string value.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}