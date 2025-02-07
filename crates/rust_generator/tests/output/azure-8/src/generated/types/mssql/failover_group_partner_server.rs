#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FailoverGroupPartnerServer {
    /// The ID of a partner SQL server to include in the failover group.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The location of the partner server.
    #[builder(into, default)]
    #[serde(rename = "location")]
    pub r#location: Box<Option<String>>,
    /// The replication role of the partner server. Possible values include `Primary` or `Secondary`.
    #[builder(into, default)]
    #[serde(rename = "role")]
    pub r#role: Box<Option<String>>,
}
