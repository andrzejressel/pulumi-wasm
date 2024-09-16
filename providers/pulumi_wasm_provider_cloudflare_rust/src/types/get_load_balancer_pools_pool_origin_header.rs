#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct GetLoadBalancerPoolsPoolOriginHeader {
    /// HTTP Header name.
    #[builder(into)]
    #[serde(rename = "header")]
    pub r#header: Box<String>,
    /// Values for the HTTP headers.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
