#[derive(serde::Serialize)]
pub struct GetLoadBalancerPoolsFilter {
    /// A regular expression matching the name of the Load Balancer pool to lookup.
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
