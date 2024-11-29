#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct LoadBalancerCountryPool {
    /// A country code which can be determined with the Load Balancing Regions API described [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api/). Multiple entries should not be specified with the same country.
    #[builder(into)]
    #[serde(rename = "country")]
    pub r#country: Box<String>,
    /// A list of pool IDs in failover priority to use in the given country.
    #[builder(into)]
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Box<Vec<String>>,
}
