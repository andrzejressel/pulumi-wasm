#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ExpressRouteCircuitSku {
    /// The billing mode for bandwidth. Possible values are `MeteredData` or `UnlimitedData`.
    /// 
    /// > **NOTE:** You can migrate from `MeteredData` to `UnlimitedData`, but not the other way around.
    #[builder(into)]
    #[serde(rename = "family")]
    pub r#family: Box<String>,
    /// The service tier. Possible values are `Basic`, `Local`, `Standard` or `Premium`.
    #[builder(into)]
    #[serde(rename = "tier")]
    pub r#tier: Box<String>,
}
