#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetWorkspacePrivateEndpointConnectionConnection {
    /// Actions required for a private endpoint connection.
    #[builder(into)]
    #[serde(rename = "actionRequired")]
    pub r#action_required: Box<String>,
    /// The description for the current state of a private endpoint connection.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// The name of the Databricks Workspace.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The status of a private endpoint connection. Possible values are `Pending`, `Approved`, `Rejected` or `Disconnected`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
    /// The Databricks Workspace resource ID for the private link endpoint.
    #[builder(into)]
    #[serde(rename = "workspacePrivateEndpointId")]
    pub r#workspace_private_endpoint_id: Box<String>,
}
