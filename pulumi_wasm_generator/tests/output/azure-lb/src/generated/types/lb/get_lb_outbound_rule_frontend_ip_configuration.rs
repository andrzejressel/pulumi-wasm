#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetLbOutboundRuleFrontendIpConfiguration {
    /// The ID of the Frontend IP Configuration.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The name of this Load Balancer Outbound Rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}