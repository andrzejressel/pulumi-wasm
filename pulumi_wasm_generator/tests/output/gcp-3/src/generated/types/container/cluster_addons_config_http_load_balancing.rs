#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterAddonsConfigHttpLoadBalancing {
    /// Whether the cluster disables default in-node sNAT rules. In-node sNAT rules will be disabled when defaultSnatStatus is disabled.When disabled is set to false, default IP masquerade rules will be applied to the nodes to prevent sNAT on cluster internal traffic
    /// 
    /// <a name="nested_cluster_telemetry"></a>The `cluster_telemetry` block supports
    #[builder(into)]
    #[serde(rename = "disabled")]
    pub r#disabled: Box<bool>,
}
