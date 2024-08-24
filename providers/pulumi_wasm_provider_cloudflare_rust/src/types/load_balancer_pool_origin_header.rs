#[derive(serde::Serialize)]
pub struct LoadBalancerPoolOriginHeader {
    /// HTTP request headers.
    #[serde(rename = "header")]
    pub r#header: Box<String>,
    /// Values for the HTTP headers.
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
