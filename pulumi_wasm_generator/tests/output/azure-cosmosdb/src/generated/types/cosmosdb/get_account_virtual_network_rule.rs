#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAccountVirtualNetworkRule {
    /// The ID of the virtual network subnet.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
}