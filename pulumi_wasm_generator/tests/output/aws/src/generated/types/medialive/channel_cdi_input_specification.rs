#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelCdiInputSpecification {
    /// Maximum CDI input resolution.
    #[builder(into)]
    #[serde(rename = "resolution")]
    pub r#resolution: Box<String>,
}