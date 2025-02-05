#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HBaseClusterComputeIsolation {
    /// This field indicates whether enable compute isolation or not. Possible values are `true` or `false`.
    #[builder(into, default)]
    #[serde(rename = "computeIsolationEnabled")]
    pub r#compute_isolation_enabled: Box<Option<bool>>,
    /// The name of the host SKU.
    #[builder(into, default)]
    #[serde(rename = "hostSku")]
    pub r#host_sku: Box<Option<String>>,
}
