#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetNetworkAttachedDataNetworkNetworkAddressPortTranslationPortRange {
    /// The maximum port number.
    #[builder(into)]
    #[serde(rename = "maximum")]
    pub r#maximum: Box<i32>,
    /// The minimum port number.
    #[builder(into)]
    #[serde(rename = "minimum")]
    pub r#minimum: Box<i32>,
}
