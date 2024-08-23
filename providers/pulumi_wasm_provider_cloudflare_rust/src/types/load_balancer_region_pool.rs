#[derive(serde::Serialize)]
pub struct LoadBalancerRegionPool {
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Box<Vec<String>>,
    #[serde(rename = "region")]
    pub r#region: Box<String>,
}
