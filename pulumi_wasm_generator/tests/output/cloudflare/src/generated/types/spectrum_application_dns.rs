#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SpectrumApplicationDns {
    /// The name of the DNS record associated with the application.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The type of DNS record associated with the application.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
