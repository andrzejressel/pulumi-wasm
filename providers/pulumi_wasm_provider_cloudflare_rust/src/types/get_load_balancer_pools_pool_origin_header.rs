#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct GetLoadBalancerPoolsPoolOriginHeader {
    /// HTTP Header name.
    #[serde(rename = "header")]
    pub r#header: Box<String>,
    /// Values for the HTTP headers.
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
