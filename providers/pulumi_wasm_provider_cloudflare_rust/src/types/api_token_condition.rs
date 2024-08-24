#[derive(serde::Serialize)]
pub struct ApiTokenCondition {
    /// Request IP related conditions.
    #[serde(rename = "requestIp")]
    pub r#request_ip: Box<Option<crate::types::ApiTokenConditionRequestIp>>,
}
