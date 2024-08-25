#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct LoadBalancerMonitorHeader {
    /// The header name.
    #[serde(rename = "header")]
    pub r#header: Box<String>,
    /// A list of values for the header.
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
