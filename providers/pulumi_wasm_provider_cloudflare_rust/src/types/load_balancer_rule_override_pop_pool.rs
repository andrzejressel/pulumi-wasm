#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct LoadBalancerRuleOverridePopPool {
    /// A list of pool IDs in failover priority to use for traffic reaching the given PoP.
    #[builder(into)]
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Box<Vec<String>>,
    /// A 3-letter code for the Point-of-Presence. Allowed values can be found in the list of datacenters on the [status page](https://www.cloudflarestatus.com/). Multiple entries should not be specified with the same PoP.
    #[builder(into)]
    #[serde(rename = "pop")]
    pub r#pop: Box<String>,
}
