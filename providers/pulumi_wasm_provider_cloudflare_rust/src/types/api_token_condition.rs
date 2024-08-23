#[derive(serde::Serialize)]
pub struct ApiTokenCondition {
    #[serde(rename = "requestIp")]
    pub r#request_ip: Box<Option<crate::types::ApiTokenConditionRequestIp>>,
}
