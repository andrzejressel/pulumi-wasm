#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LoadBalancerRandomSteering {
    /// The default weight for pools in the load balancer that are not specified in the `pool_weights` map.
    #[builder(into, default)]
    #[serde(rename = "defaultWeight")]
    pub r#default_weight: Box<Option<f64>>,
    /// A mapping of pool IDs to custom weights. The weight is relative to other pools in the load balancer.
    #[builder(into, default)]
    #[serde(rename = "poolWeights")]
    pub r#pool_weights: Box<Option<std::collections::HashMap<String, f64>>>,
}
