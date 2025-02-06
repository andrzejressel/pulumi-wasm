#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApiTokenCondition {
    /// Request IP related conditions.
    #[builder(into, default)]
    #[serde(rename = "requestIp")]
    pub r#request_ip: Box<Option<super::types::ApiTokenConditionRequestIp>>,
}
