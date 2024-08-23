#[derive(serde::Serialize)]
pub struct GetLoadBalancerPoolsPoolOriginHeader {
    #[serde(rename = "header")]
    pub r#header: Box<String>,
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
