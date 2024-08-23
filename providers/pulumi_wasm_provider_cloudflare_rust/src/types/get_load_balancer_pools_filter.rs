#[derive(serde::Serialize)]
pub struct GetLoadBalancerPoolsFilter {
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
