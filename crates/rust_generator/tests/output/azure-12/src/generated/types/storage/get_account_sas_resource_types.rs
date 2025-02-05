#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAccountSasResourceTypes {
    /// Should permission be granted to the container?
    #[builder(into)]
    #[serde(rename = "container")]
    pub r#container: Box<bool>,
    /// Should permission be granted only to a specific object?
    #[builder(into)]
    #[serde(rename = "object")]
    pub r#object: Box<bool>,
    /// Should permission be granted to the entire service?
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Box<bool>,
}
