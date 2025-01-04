#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetApplicationGatewaySkus {
    /// The Capacity of the SKU in use for this Application Gateway.
    #[builder(into)]
    #[serde(rename = "capacity")]
    pub r#capacity: Box<i32>,
    /// The name of this Application Gateway.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The Tier of the SKU in use for this Application Gateway.
    #[builder(into)]
    #[serde(rename = "tier")]
    pub r#tier: Box<String>,
}
