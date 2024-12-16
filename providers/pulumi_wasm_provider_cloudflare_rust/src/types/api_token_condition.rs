#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ApiTokenCondition {
    /// Request IP related conditions.
    #[builder(into, default)]
    #[serde(rename = "requestIp")]
    pub r#request_ip: Box<Option<crate::types::ApiTokenConditionRequestIp>>,
}
