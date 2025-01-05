#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CapacitySku {
    /// The name of the SKU to use for the Fabric Capacity. Possible values are `F2`, `F4`, `F8`, `F16`, `F32`, `F64`, `F128`, `F256`, `F512`, `F1024`, `F2048`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The tier of the SKU to use for the Fabric Capacity. The only possible value is `Fabric`.
    #[builder(into)]
    #[serde(rename = "tier")]
    pub r#tier: Box<String>,
}
