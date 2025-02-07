#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BrokerLogs {
    /// Enables audit logging. Auditing is only possible for `engine_type` of `ActiveMQ`. User management action made using JMX or the ActiveMQ Web Console is logged. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "audit")]
    pub r#audit: Box<Option<bool>>,
    /// Enables general logging via CloudWatch. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "general")]
    pub r#general: Box<Option<bool>>,
}
