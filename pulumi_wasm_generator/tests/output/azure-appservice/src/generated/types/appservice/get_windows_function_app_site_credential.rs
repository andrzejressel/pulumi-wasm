#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetWindowsFunctionAppSiteCredential {
    /// The name of this Windows Function App.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The Site Credentials Password used for publishing.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
}