#[derive(serde::Serialize)]
pub struct LoadBalancerPoolOriginSteering {
    #[serde(rename = "policy")]
    pub r#policy: Box<Option<String>>,
}
