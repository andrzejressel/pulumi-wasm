#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceQueryInsightsConfig {
    /// Number of query execution plans captured by Insights per minute for all queries combined. The default value is 5. Any integer between 0 and 20 is considered valid.
    #[builder(into, default)]
    #[serde(rename = "queryPlansPerMinute")]
    pub r#query_plans_per_minute: Box<Option<i32>>,
    /// Query string length. The default value is 1024. Any integer between 256 and 4500 is considered valid.
    #[builder(into, default)]
    #[serde(rename = "queryStringLength")]
    pub r#query_string_length: Box<Option<i32>>,
    /// Record application tags for an instance. This flag is turned "on" by default.
    #[builder(into, default)]
    #[serde(rename = "recordApplicationTags")]
    pub r#record_application_tags: Box<Option<bool>>,
    /// Record client address for an instance. Client address is PII information. This flag is turned "on" by default.
    #[builder(into, default)]
    #[serde(rename = "recordClientAddress")]
    pub r#record_client_address: Box<Option<bool>>,
}
