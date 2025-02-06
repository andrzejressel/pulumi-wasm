#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AzureNodePoolMaxPodsConstraint {
    /// The maximum number of pods to schedule on a single node.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "maxPodsPerNode")]
    pub r#max_pods_per_node: Box<i32>,
}
