#[derive(serde::Serialize)]
pub struct LoadBalancerPoolOrigin {
    #[serde(rename = "address")]
    pub r#address: Box<String>,
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<crate::types::LoadBalancerPoolOriginHeader>>>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "weight")]
    pub r#weight: Box<Option<f64>>,
}
