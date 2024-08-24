#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverridePopPool {
    /// A list of pool IDs in failover priority to use for traffic reaching the given PoP.
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Box<Vec<String>>,
    /// A 3-letter code for the Point-of-Presence. Allowed values can be found in the list of datacenters on the [status page](https://www.cloudflarestatus.com/). Multiple entries should not be specified with the same PoP.
    #[serde(rename = "pop")]
    pub r#pop: Box<String>,
}
