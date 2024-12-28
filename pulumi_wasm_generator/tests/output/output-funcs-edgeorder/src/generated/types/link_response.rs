#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LinkResponse {
    /// Type of link
    #[builder(into)]
    #[serde(rename = "linkType")]
    pub r#link_type: Box<String>,
    /// Url of the link
    #[builder(into)]
    #[serde(rename = "linkUrl")]
    pub r#link_url: Box<String>,
}
