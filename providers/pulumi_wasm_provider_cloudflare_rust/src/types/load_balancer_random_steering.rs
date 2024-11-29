#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct LoadBalancerRandomSteering {
    /// The default weight for pools in the load balancer that are not specified in the `pool_weights` map.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "defaultWeight")]
    pub r#default_weight: Box<Option<f64>>,
    /// A mapping of pool IDs to custom weights. The weight is relative to other pools in the load balancer.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "poolWeights")]
    pub r#pool_weights: Box<Option<std::collections::HashMap<String, f64>>>,
}
