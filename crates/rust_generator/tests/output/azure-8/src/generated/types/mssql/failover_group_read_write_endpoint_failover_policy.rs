#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FailoverGroupReadWriteEndpointFailoverPolicy {
    /// The grace period in minutes, before failover with data loss is attempted for the read-write endpoint. Required when `mode` is `Automatic`.
    #[builder(into, default)]
    #[serde(rename = "graceMinutes")]
    pub r#grace_minutes: Box<Option<i32>>,
    /// The failover policy of the read-write endpoint for the failover group. Possible values are `Automatic` or `Manual`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
}
