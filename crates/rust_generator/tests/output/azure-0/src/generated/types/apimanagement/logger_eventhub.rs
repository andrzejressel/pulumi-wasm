#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct LoggerEventhub {
    /// The connection string of an EventHub Namespace.
    /// 
    /// > **Note:** At least one of `connection_string` or `endpoint_uri` must be specified
    #[builder(into, default)]
    #[serde(rename = "connectionString")]
    pub r#connection_string: Box<Option<String>>,
    /// The endpoint address of an EventHub Namespace. Required when `client_id` is set.
    #[builder(into, default)]
    #[serde(rename = "endpointUri")]
    pub r#endpoint_uri: Box<Option<String>>,
    /// The name of an EventHub.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The Client Id of the User Assigned Identity with the "Azure Event Hubs Data Sender" role to the target EventHub Namespace. Required when `endpoint_uri` is set. If not specified the System Assigned Identity will be used.
    #[builder(into, default)]
    #[serde(rename = "userAssignedIdentityClientId")]
    pub r#user_assigned_identity_client_id: Box<Option<String>>,
}
