#[derive(serde::Serialize)]
pub struct LoadBalancerPopPool {
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Box<Vec<String>>,
    #[serde(rename = "pop")]
    pub r#pop: Box<String>,
}
